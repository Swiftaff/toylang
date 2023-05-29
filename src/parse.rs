/*! Main Parser functions
 */
use crate::ast::output;
use crate::ast::parents;
use crate::elements;
use crate::elements::{Element, ElementInfo};
use crate::errors::ERRORS;
use crate::errors::{self, append_error};
use crate::Compiler;
use crate::Tokens;

/*
fn testy() {
    let list: Vec<i64> = vec![1];
    fn mapfn(i: &i64) -> i64 {
        i * 100
    }
    let mapped: Vec<i64> = list.iter().map(mapfn).collect();
}
*/

/// While loop over the tokens in the current line
pub fn current_line(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("parse::current_line {:?}", ""));
    let tokens = compiler.lines_of_tokens[compiler.current_line].clone();
    if tokens.len() > 0 {
        while compiler.current_line_token < tokens.len() {
            current_token(compiler, &tokens)?;
            compiler.current_line_token = compiler.current_line_token + 1;
        }
    }
    Ok(())
}

/// Parse token, it's either an inbuiltFn, Arg, Type or something else
pub fn current_token(compiler: &mut Compiler, tokens: &Tokens) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("parse::current_token {:?}", &tokens));
    let current_token = &tokens[compiler.current_line_token];
    let current_token_vec: &Vec<char> = &tokens[compiler.current_line_token].0.chars().collect();
    if current_token_vec.len() == 0 {
        return Ok(());
    }

    match elements::get_inbuilt_function_index_by_name(&mut compiler.ast, &current_token.0) {
        Some(index_of_function) => {
            //dbg!(&current_token);
            let func = &compiler.ast.elements[index_of_function];
            match &func.0 {
                ElementInfo::InbuiltFunctionDef(_, _, _, _, _, _) => {
                    inbuilt_function_call(compiler, &current_token.0, index_of_function)
                }
                ElementInfo::FunctionDef(_, _, _, _) => {
                    function_call(compiler, &current_token.0, index_of_function)
                }
                ElementInfo::Arg(_, _, _, returntype) => {
                    if returntype.contains("&dyn Fn") {
                        function_call(compiler, &current_token.0, index_of_function)
                    } else {
                        token_by_first_chars(compiler, &current_token.0, &current_token_vec)
                    }
                }
                _ => token_by_first_chars(compiler, &current_token.0, &current_token_vec),
            }
        }
        _ => match elements::get_inbuilt_type_index_by_name(&mut compiler.ast, &current_token.0) {
            Some(index_of_type) => elements::append::types(compiler, index_of_type),
            _ => token_by_first_chars(compiler, &current_token.0, &current_token_vec),
        },
    }
}

/// Parses something based on it's first character, if it's not been added as an inbuiltFn yet
///
/// The idea being to reduce this over time to make as many tokens as inbuiltFns, except new constant refs or values like ints, strings etc
pub fn token_by_first_chars(
    compiler: &mut Compiler,
    current_token: &String,
    current_token_vec: &Vec<char>,
) -> Result<(), ()> {
    compiler.ast.log(format!(
        "parse::token_by_first_chars {:?} {:?}",
        &current_token, current_token_vec
    ));
    let first_char = current_token_vec[0];
    let second_char = if current_token_vec.len() > 1 {
        Some(current_token_vec[1])
    } else {
        None
    };
    match first_char {
        '{' => match second_char {
            Some(_second) => {
                return errors::append_error(compiler, 0, 1, ERRORS.a_struct);
            }
            None => struct_start(compiler),
        },
        '}' => struct_end(compiler),
        '[' => match second_char {
            Some(second) => {
                if second == ']' {
                    list_empty(compiler)
                } else {
                    return errors::append_error(compiler, 0, 1, ERRORS.list);
                }
            }
            None => list_start(compiler),
        },
        ']' => list_end(compiler),
        '\\' => function_definition_start(compiler),
        '(' => functiontypesig_or_functionreference_start(compiler),
        ')' => functiontypesig_or_functionreference_end(compiler),
        '/' => comment_single_line(compiler, current_token_vec),
        '#' => rustcode(compiler, current_token_vec),
        '@' => println(compiler),
        '?' => if_expression(compiler),
        '=' => match second_char {
            Some(second) => {
                if second == '>' {
                    return function_definition_end(compiler);
                } else {
                    return errors::append_error(compiler, 0, 1, ERRORS.assign);
                }
            }
            _ => assignment(compiler),
        },
        '.' => match second_char {
            Some(second) => {
                if second == '.' {
                    return loop_for_range_start(compiler);
                } else {
                    return errors::append_error(compiler, 0, 1, ERRORS.loop_for);
                }
            }
            _ => loop_end(compiler),
        },
        '"' => string(compiler, &current_token),
        //positive numbers
        first_char if is_integer(&first_char.to_string()) => {
            if is_float(&current_token) {
                float(compiler, &current_token)
            } else {
                int(compiler, &current_token)
            }
        }
        //negative numbers
        '-' => match second_char {
            Some(_digit) => {
                if is_float(&current_token) {
                    float(compiler, &current_token)
                } else {
                    int(compiler, &current_token)
                }
            }
            None => {
                return errors::append_error(compiler, 0, 1, ERRORS.int_negative);
            }
        },
        /*
        first_char if "IFS".contains(&first_char.to_string()) => {
            //dbg!("Int Float or String", first_char);
            match current_token {
                "Int" =>
            }
        }*/
        first_char if "abcdefghijklmnopqrstuvwxyz_".contains(&first_char.to_string()) => {
            //dbg!("constant or constantRef", first_char);

            //check if contains a dot, so could be a struct edit, e.g. structname.key
            let token_split: Vec<&str> = current_token.split(".").into_iter().collect();
            if token_split.len() > 1 {
                struct_edit(compiler, &current_token)
            } else {
                constant(compiler, &current_token)
            }
        }
        _ => return errors::append_error(compiler, 0, 1, "parser - unknown error"),
    }
}

/// Parses a Comment single line
pub fn comment_single_line(
    compiler: &mut Compiler,
    current_token_vec: &Vec<char>,
) -> Result<(), ()> {
    compiler.ast.log(format!(
        "parse::comment_single_line {:?}",
        current_token_vec
    ));
    if current_token_vec.len() < 2 || current_token_vec[1] != '/' {
        return errors::append_error(compiler, 0, 1, ERRORS.comment_single_line);
    }
    let val = concatenate_vec_strings(&compiler.lines_of_tokens[compiler.current_line]);
    elements::append::comment_single_line(compiler, val)
}

/// Parses raw rust code to be inserted in place, either in main fn, or before it
pub fn rustcode(compiler: &mut Compiler, current_token_vec: &Vec<char>) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("parse::rustcode {:?}", current_token_vec));

    if current_token_vec.len() < 3 || current_token_vec[1] != '#' {
        return errors::append_error(compiler, 0, 1, ERRORS.rustcode);
    }
    let is_premain = current_token_vec[2] == '#';
    let mut val = concatenate_vec_strings(&compiler.lines_of_tokens[compiler.current_line]);
    let mut chars = val.chars();
    chars.next();
    chars.next();
    if is_premain {
        chars.next();
    }
    val = chars.as_str().to_string();
    elements::append::rustcode_main(compiler, val, is_premain)
}

/// Parses a Println
pub fn println(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("parse::println {:?}", ""));
    elements::append::println(compiler)
}

/// Parses a If
pub fn if_expression(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("parse::if_expression {:?}", ""));
    elements::append::if_expression(compiler)
}

/// Parses a String
pub fn string(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("parse::string {:?}", current_token));
    if is_string(&current_token) {
        elements::append::string(compiler, current_token)
    } else {
        //dbg!(&self.lines_of_tokens);
        errors::append_error(compiler, 0, 1, ERRORS.string)
    }
}

/// Parses an Int
pub fn int(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    //dbg!("parse_int - positive only for now");
    compiler.ast.log(format!("parse::int {:?}", current_token));
    let all_chars_are_numeric = current_token.chars().into_iter().all(|c| c.is_numeric());
    let chars: Vec<char> = current_token.chars().collect();
    let first_char_is_negative_sign = chars[0] == '-';
    let is_negative_all_other_chars_are_not_numeric = first_char_is_negative_sign
        && chars.len() > 1
        && !chars[1..chars.len()].into_iter().all(|c| c.is_numeric());

    if (!first_char_is_negative_sign && !all_chars_are_numeric)
        || is_negative_all_other_chars_are_not_numeric
    {
        errors::append_error(compiler, 0, current_token.len(), ERRORS.int)?;
    }
    match current_token.parse::<i64>() {
        Ok(_) => (),
        Err(_) => errors::append_error(compiler, 0, current_token.len(), ERRORS.int_out_of_bounds)?,
    }
    elements::append::int(compiler, current_token)
    //errors::error_if_parent_is_invalid(compiler)
}

/// Parses a Float
pub fn float(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("parse::float {:?}", current_token));
    if current_token.len() > 0 && is_float(current_token) {
        elements::append::float(compiler, current_token)
    } else {
        return errors::append_error(compiler, 0, 1, ERRORS.float);
    }
}

/// Parses a Constant - as a Constant, ConstantRef, Arg, FunctionDef, If - or an error
pub fn constant(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("parse::constant {:?}", current_token));
    let el_option = elements::get_element_by_name(&compiler.ast, current_token);
    match el_option {
        Some(_) => {
            match el_option {
                //you may find the original constant...
                Some((ElementInfo::Constant(_, returntype), _)) => {
                    return elements::append::constant_ref(compiler, current_token, &returntype);
                }
                //...or a later reference to it
                Some((ElementInfo::ConstantRef(_, returntype, _), _)) => {
                    return elements::append::constant_ref(compiler, current_token, &returntype);
                }
                Some((ElementInfo::Arg(_, _, _, returntype), _)) => {
                    //dbg!("Arg", &returntype);
                    if returntype.contains("&dyn Fn") {
                        let args = get_args_from_dyn_fn(&returntype);
                        return elements::append::function_call(
                            compiler,
                            current_token,
                            args,
                            &returntype,
                            false,
                        );
                    } else {
                        return elements::append::constant_ref(
                            compiler,
                            current_token,
                            &returntype,
                        );
                    }
                }
                Some((ElementInfo::FunctionDef(_, argnames, _, returntype), _)) => {
                    return elements::append::function_ref_or_call(
                        compiler,
                        current_token,
                        argnames.len(),
                        &returntype,
                    );
                }
                Some((ElementInfo::If(_returntype), _)) => {
                    return elements::append::if_expression(compiler);
                }
                // explicitly listing other types rather than using _ to not overlook new types in future
                Some((ElementInfo::Root, _)) => (),
                Some((ElementInfo::Struct(_, _, _), _)) => (),
                Some((ElementInfo::StructEdit(_, _), _)) => (),
                Some((ElementInfo::List(_), _)) => (),
                Some((ElementInfo::CommentSingleLine(_), _)) => (),
                Some((ElementInfo::Int(_), _)) => (),
                Some((ElementInfo::Float(_), _)) => (),
                Some((ElementInfo::String(_), _)) => (),
                Some((ElementInfo::Bool(_), _)) => (),
                Some((ElementInfo::Assignment, _)) => (),
                Some((ElementInfo::InbuiltFunctionDef(_, _, _, _, _, _), _)) => (),
                Some((ElementInfo::InbuiltFunctionCall(_, _, _), _)) => (),
                Some((ElementInfo::FunctionDefWIP, _)) => (),
                Some((ElementInfo::FunctionCall(_, _, _), _)) => (),
                Some((ElementInfo::Parens, _)) => (),
                Some((ElementInfo::Type(_), _)) => (),
                Some((ElementInfo::Eol, _)) => (),
                Some((ElementInfo::Seol, _)) => (),
                Some((ElementInfo::Indent, _)) => (),
                Some((ElementInfo::Unused, _)) => (),
                Some((ElementInfo::LoopForRangeWIP, _)) => (),
                Some((ElementInfo::LoopForRange(_, _, _), _)) => (),
                Some((ElementInfo::Println, _)) => (),
                Some((ElementInfo::Rust(_, _), _)) => (),
                None => (),
            }
        }
        None => (),
    }
    return elements::append::new_constant_or_arg(compiler, current_token);
}

/// Parses as assignment
pub fn assignment(compiler: &mut Compiler) -> Result<(), ()> {
    // TODO error checking
    compiler.ast.log(format!("parse::assignment {:?}", ""));
    elements::append::assignment(compiler)
}

/// Parses an inbuiltFnCall
pub fn inbuilt_function_call(
    compiler: &mut Compiler,
    current_token: &String,
    index_of_function: usize,
) -> Result<(), ()> {
    //TODO error checking
    compiler.ast.log(format!(
        "parse::inbuilt_function_call {:?} {:?}",
        current_token, index_of_function
    ));
    elements::append::inbuilt_function_call(compiler, current_token, index_of_function)
}

/// Parses a FnCall
pub fn function_call(
    compiler: &mut Compiler,
    current_token: &String,
    index_of_function: usize,
) -> Result<(), ()> {
    compiler.ast.log(format!(
        "parse::function_call {:?} {:?}",
        current_token, index_of_function
    ));
    elements::append::function_call1(compiler, current_token, index_of_function)
}

/// Parses an empty List
pub fn list_empty(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("parse::list_empty {:?}", ""));
    list_start(compiler)?;
    list_end(compiler)
}

/// Parses start of a Struct
pub fn struct_start(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("parse::struct_start {:?}", ""));
    elements::append::struct_start(compiler)
}

/// Parses start of a Struct Edit
pub fn struct_edit(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    compiler.ast.log(format!("parse::struct_edit {:?}", ""));
    elements::append::struct_edit(compiler, current_token)
}

/// Parses end of a Struct
pub fn struct_end(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("parse::struct_end {:?}", ""));

    let a_struct = parents::get_current_parent_element_from_parents(&compiler.ast);
    if let (ElementInfo::Struct(_, _, _), children) = a_struct {
        if children.len() == 0 {
            return append_error(compiler, 0, 1, ERRORS.a_struct);
        }
        if let Some(existing_struct_ref) =
            output::get_existing_identical_struct_el_ref(&mut compiler.ast.clone(), children)
        {
            let this_struct_el_ref = parents::get_current_parent_ref_from_parents(&compiler.ast);
            if existing_struct_ref != this_struct_el_ref {
                // don't redefine a new struct if it's the same as an existing struct, reuse it instead, i.e.
                // remove this struct and it's children
                // and replace the Constant's reference to it, to use the existing struct ref instead
                let mut struct_and_children = compiler.ast.elements[this_struct_el_ref].1.clone();
                struct_and_children.push(this_struct_el_ref);
                dbg!(&struct_and_children);
                for i in 0..struct_and_children.len() as usize {
                    compiler.ast.elements[struct_and_children[i]] = (ElementInfo::Unused, vec![]);
                }
                if let Some(structs_parent_ref) =
                    parents::get_current_parent_ref_from_element_children_search(
                        &compiler.ast,
                        this_struct_el_ref,
                    )
                {
                    if let ElementInfo::Constant(_, _) = compiler.ast.elements[structs_parent_ref].0
                    {
                        compiler.ast.elements[structs_parent_ref].1 = vec![existing_struct_ref];
                    }
                }

                dbg!("got copy", struct_and_children);
            }
        }
    }
    parents::outdent::outdent(compiler);
    parents::outdent::outdent(compiler);
    parents::outdent::outdent(compiler);
    elements::append::seol_if_last_in_line(compiler)
}

/// Parses start of a List
pub fn list_start(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("parse::list_start {:?}", ""));
    elements::append::list_start(compiler)
}

/// Parses end of a List
pub fn list_end(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("parse::list_end {:?}", ""));
    let list_parent_ref = parents::get_current_parent_ref_from_parents(&compiler.ast);
    let list_parent = parents::get_current_parent_element_from_parents(&compiler.ast);
    match list_parent {
        (ElementInfo::List(returntype), children) => {
            if returntype == "Undefined".to_string() {
                if children.len() == 0 {
                    return append_error(compiler, 0, 1, ERRORS.list);
                } else {
                    // may as well get type now if child is a list - removes an error if it is a nested list as an arg for a func def
                    if let ElementInfo::List(list_type) =
                        compiler.ast.elements[children[0]].0.clone()
                    {
                        compiler.ast.elements[list_parent_ref].0 =
                            ElementInfo::List(format!("Vec<{}>", list_type));
                    }
                }
            }
        }
        _ => (),
    }
    compiler.ast.log(format!(
        "parse::list_end - outdent_if_last_expected_child {:?}",
        ""
    ));
    parents::outdent::outdent(compiler);
    elements::append::outdent_if_last_expected_child(compiler);
    elements::append::seol_if_last_in_line(compiler)
}

/// Parses start of a FnDef
pub fn function_definition_start(compiler: &mut Compiler) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("parse::function_definition_start {:?}", ""));
    elements::append::function_definition_start(compiler)
}

/// Parses start of a Loop for range
pub fn loop_for_range_start(compiler: &mut Compiler) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("parse::loop_for_range_start {:?}", ""));
    elements::append::loop_for_range_start(compiler)
}

/// Parses end of a Loop for range
pub fn loop_end(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("parse::loop_end {:?}", ""));
    /* At the point you parse a loop end,
       and because we don't look ahead when parsing,
       the Ast thinks this is what has been parsed

       - example for a LoopForRange
    21: LoopForRangeWIP [ 22, 24, 25, 26, ... ]
    22: Constant: b (i64) [ 23, ]
    23: Int: 5 [ ]
    24: Int: 100 [ ]
    25: Seol [ ]
    26: Indent [ ]
    ... remaining loop contents
    */
    // We need to change this to
    /*
    21: LoopForRange: b 5 100 [ ]
    22: Unused
    23: Unused
    24: Unused
    25: Unused
    26: Unused
    ...
    */

    //get parent LoopForRange
    let mut loopforrangewip_ref = 0;
    for n in (0..compiler.ast.elements.len()).rev() {
        let el = compiler.ast.elements[n].clone();
        match el.0 {
            ElementInfo::LoopForRangeWIP => {
                loopforrangewip_ref = n;
                break;
            }
            _ => (),
        }
    }
    if loopforrangewip_ref == 0 {
        return append_error(compiler, 0, 1, ERRORS.loopfor_end_but_no_start);
    }
    let loopforrangewip = compiler.ast.elements[loopforrangewip_ref].clone();
    //check it has two children, 1. let (with its child first int) 2. second int
    if loopforrangewip.1.len() < 3 {
        return append_error(compiler, 0, 1, ERRORS.loopfor_malformed);
    }
    let first_child = compiler.ast.elements[loopforrangewip.1[0]].clone();
    let second_child = compiler.ast.elements[loopforrangewip.1[1]].clone();
    match (first_child.0, second_child.0) {
        (ElementInfo::Constant(name, _), ElementInfo::Int(to)) => {
            //rename LoopForRange with name, from, to
            let const_children = first_child.1;
            if const_children.len() == 1 {
                let const_child = compiler.ast.elements[const_children[0]].clone();
                match const_child.0 {
                    ElementInfo::Int(from) => {
                        let new_loopforrange = ElementInfo::LoopForRange(
                            name,
                            from.parse::<usize>().unwrap(),
                            to.parse::<usize>().unwrap(),
                        );
                        let mut new_loopforrange_children = loopforrangewip.1;
                        new_loopforrange_children =
                            parents::vec_remove_head(&new_loopforrange_children);
                        new_loopforrange_children =
                            parents::vec_remove_head(&new_loopforrange_children);
                        new_loopforrange_children =
                            parents::vec_remove_head(&new_loopforrange_children);
                        compiler.ast.elements[loopforrangewip_ref] =
                            (new_loopforrange, new_loopforrange_children);
                        Ok(())
                    }
                    _ => append_error(compiler, 0, 1, ERRORS.loopfor_malformed),
                }
            } else {
                append_error(compiler, 0, 1, ERRORS.loopfor_malformed)
            }
        }
        _ => append_error(compiler, 0, 1, ERRORS.loopfor_malformed),
    }
}

/// Parses end of a FnDef
pub fn function_definition_end(compiler: &mut Compiler) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("parse::function_definition_end {:?}", ""));
    /*
    At the point you parse a function definition end,
    and because we don't look ahead when parsing,
    the Ast thinks this is what has been parsed
    ...
    10: Assignment: (Undefined) [ 11, 12, ]
    11: Constant: a (Undefined) [ ]
    12: FunctionDef: ("", [], [], Undefined) [ 13, 14, 15, ]
    13: Type: i64 [ ]
    14: Type: i64 [ ]
    15: Constant: arg1 (Undefined) [ ]

    We need to change this to, e.g. this for a single line function...
    10: Unused
    11: Unused
    12: FunctionDef(name, argtypes, argnames, returntype): [ ] (<-ready to accept 16 return statement)
    13: Unused
    14: Unused
    15: Unused
    ... ready to insert next element 16 which is the return statement
    */

    //get parent funcdef
    let func_def_ref = parents::get_current_parent_ref_from_parents(&compiler.ast);
    //no need to get fancy - child elements should already be outdented so current parent should be func_def
    //parents::get_current_parent_ref_from_element_children_search(&compiler.ast,compiler.ast.elements.len() - 1,){

    //get child refs
    let func_def: Element = compiler.ast.elements[func_def_ref].clone();
    let children = func_def.1.clone();

    //error if count is NOT odd (argtypes + returntype + argnames)
    if children.len() % 2 == 0 || children.len() == 0 {
        return errors::append_error(compiler, 0, 1, ERRORS.funcdef_args);
    }

    //TODO deal with brackets later (i.e. for type signature containing argument(s) which are fns)

    //error if arg types are NOT first
    let first_child_ref = children[0];

    let first_child: Element = compiler.ast.elements[first_child_ref].clone();
    match first_child.0 {
        ElementInfo::Type(_) => (),
        ElementInfo::Parens => (),
        ElementInfo::List(_) => (),
        _ => return errors::append_error(compiler, 0, 1, ERRORS.funcdef_argtypes_first),
    }

    // now change any top level List items into Types
    for child_ref in children.clone() {
        if let ElementInfo::List(list_type) = compiler.ast.elements[child_ref].0.clone() {
            compiler.ast.elements[child_ref].0 = ElementInfo::Type(list_type);
        }
    }

    match func_def.0 {
        ElementInfo::FunctionDefWIP => {
            //Constant is parent of functionDefWIP
            if let Some(constant_ref) = parents::get_current_parent_ref_from_element_children_search(
                &compiler.ast,
                func_def_ref,
            ) {
                let constant = compiler.ast.elements[constant_ref].clone();

                //assignment is parent of constant
                if let Some(assignment_ref) =
                    parents::get_current_parent_ref_from_element_children_search(
                        &compiler.ast,
                        constant_ref,
                    )
                {
                    match constant.0 {
                        ElementInfo::Constant(name, _) => {
                            elements::replace_funcdefwip_with_funcdef(
                                compiler,
                                &children,
                                &name,
                                func_def_ref,
                            );

                            // replace assignment with unused
                            compiler.ast.elements[assignment_ref] = (ElementInfo::Unused, vec![]);

                            // replace constant with Unused
                            compiler.ast.elements[constant_ref] = (ElementInfo::Unused, vec![]);

                            // replace parents child reference to the assignment, with the func_def_ref
                            if let Some(index) =
                                parents::get_current_parent_ref_from_element_children_search(
                                    &compiler.ast,
                                    assignment_ref,
                                )
                            {
                                elements::replace_element_child(
                                    &mut compiler.ast,
                                    index,
                                    assignment_ref,
                                    func_def_ref,
                                );
                            }

                            //re-add the new funcdef as latest parent, so we can continue parsing with it's child statements
                            parents::outdent::outdent(compiler);
                            parents::outdent::outdent(compiler);
                            parents::outdent::outdent(compiler);
                            parents::indent::indent_this(&mut compiler.ast, func_def_ref);
                            //dbg!(&self.ast.parents);
                        }
                        _ => (),
                    }
                }
            }
        }
        _ => (),
    }

    elements::append::outdent_if_last_expected_child(compiler);
    Ok(())
}

//TODO remember to error / or at least check if reusing arg names in nested functions

/// Parses a Fn type signature or start of a fnRef
pub fn functiontypesig_or_functionreference_start(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!(
        "parse::functiontypesig_or_functionreference_start {:?}",
        ""
    ));
    elements::append::functiontypesig_or_functionreference_start(compiler)
}

/// Parses a Fn type signature, or end of a FnRef
pub fn functiontypesig_or_functionreference_end(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!(
        "parse::functiontypesig_or_functionreference_end {:?}",
        ""
    ));
    elements::append::functiontypesig_or_functionreference_end(compiler)
}

/// Checks if an Int
pub fn is_integer(text: &String) -> bool {
    let mut is_valid = true;
    let all_chars_are_numeric = text.chars().into_iter().all(|c| c.is_numeric());
    let text_chars: Vec<char> = text.chars().collect();
    let first_char_is_negative_sign = text_chars[0] == '-';

    let is_negative_all_other_chars_are_numeric = first_char_is_negative_sign
        && text_chars.len() > 1
        && text_chars[1..text_chars.len()]
            .into_iter()
            .all(|c| c.is_numeric());

    if !all_chars_are_numeric && !is_negative_all_other_chars_are_numeric {
        is_valid = false;
    }

    match text.parse::<i64>() {
        Ok(_) => (),
        Err(_) => is_valid = false,
    }
    is_valid
}

/// Checks if a Float
pub fn is_float(text: &String) -> bool {
    let mut is_valid = true;
    let mut index_decimal_point = 0;
    let mut index_e = 0;
    let mut index_plus = 0;
    let char_vec: Vec<char> = text.chars().collect();
    for i in 0..text.len() {
        let c = char_vec[i];
        if c == '.' && index_decimal_point == 0 {
            index_decimal_point = i;
        } else if c == 'E' && index_e == 0 {
            index_e = i;
        } else if c == '+' && index_plus == 0 {
            index_plus = i;
        } else if !c.is_numeric() && !(i == 0 && c == '-') {
            is_valid = false;
        }
    }
    let has_one_decimal_point = index_decimal_point != 0;
    let no_power_of_10 = index_e == 0 && index_plus == 0;
    let has_one_power_of_10 = index_e != 0
        && index_plus > 0
        && index_plus == index_e + 1
        && (char_vec.len() > 1 && index_plus < char_vec.len() - 1)
        && index_e > 0;
    //println!("{} {:?}", text, text.parse::<f64>());
    match text.parse::<f64>() {
        Ok(val) => {
            if val == f64::INFINITY || val == f64::NEG_INFINITY {
                is_valid = false;
            }
        }
        Err(_) => is_valid = false,
    }
    is_valid && has_one_decimal_point && (no_power_of_10 || has_one_power_of_10)
}

/// Checks if a string
pub fn is_string(text: &String) -> bool {
    let mut is_valid = true;
    let char_vec: Vec<char> = text.chars().collect();
    if text.len() < 2 || char_vec[0] != '"' || char_vec[text.len() - 1] != '"' {
        is_valid = false;
    }
    is_valid
}

/// Gets args from a Dyn Fn, e.g.
///
/// 0 args, e.g. "&dyn Fn() -> i64"         = 0 commas + 0 does match ()
/// 1 args, e.g. "&dyn Fn(i64) -> i64"      = 0 commas + 1 does not match ()
/// 2 args, e.g. "&dyn Fn(i64, i64) -> i64" = 1 comma  + 1 does not match ()
pub fn get_args_from_dyn_fn(string: &String) -> usize {
    string.matches(",").count() + (!string.contains("()") as usize)
}

/// Concatenates a vec of tokens, used in a comment
pub fn concatenate_vec_strings(tokens: &Tokens) -> String {
    let mut output = "".to_string();
    for i in 0..tokens.len() {
        output = format!("{}{}", output, tokens[i].0);
    }
    output
}

/// Strips leading whitespace from a String
pub fn strip_leading_whitespace(input: &String) -> String {
    let char_vec: Vec<char> = input.chars().collect();
    let mut checking_for_whitespace = true;
    let mut first_non_whitespace_index = 0;
    for i in 0..input.len() {
        if checking_for_whitespace {
            if !char_vec[i].is_whitespace() {
                first_non_whitespace_index = i;
                checking_for_whitespace = false;
            }
        }
    }
    if first_non_whitespace_index == 0 && checking_for_whitespace {
        // if you get to end of string and it's all whitespace return empty string
        return "".to_string();
    }
    input[first_non_whitespace_index..].to_string()
}

/// Strips trailing whitespace from a String
pub fn strip_trailing_whitespace(input: &String) -> String {
    let char_vec: Vec<char> = input.chars().collect();
    let mut checking_for_whitespace = true;
    let mut first_non_whitespace_index = input.len();
    for i in (0..input.len()).rev() {
        if checking_for_whitespace {
            if !char_vec[i].is_whitespace() {
                first_non_whitespace_index = i + 1;
                checking_for_whitespace = false;
            }
        }
    }
    if first_non_whitespace_index == 0 && checking_for_whitespace {
        //if you get to end of string and it's all whitespace return empty string
        return "".to_string();
    }
    input[..first_non_whitespace_index].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_integer() {
        let test_case_passes = [
            "1",
            "123",
            "1234567890",
            "9223372036854775807",
            "-1",
            "-123",
            "-1234567890",
            "-9223372036854775808",
        ];
        for test in test_case_passes {
            let input = &test.to_string();
            assert!(is_integer(input));
        }
        let test_case_fails = ["1a", "9223372036854775808", "-1a", "-9223372036854775809"];
        for test in test_case_fails {
            let input = &test.to_string();
            assert!(!is_integer(input));
        }
    }

    #[test]
    fn test_is_float() {
        let test_case_passes = [
            "1.1",
            "123.123",
            "1234567890.123456789",
            "1.7976931348623157E+308",
            "-1.1",
            "-123.123",
            "-1234567890.123456789",
            "-1.7976931348623157E+308",
        ];
        for test in test_case_passes {
            let input = &test.to_string();
            assert!(is_float(input));
        }
        let test_case_fails = [
            "123",
            "-123",
            "1.1.1",
            "1.7976931348623157E+309",
            "-1.7976931348623157E+309",
            "1.797693134E+8623157E+309",
            "-1.79769313E+48623157E+309",
        ];
        for test in test_case_fails {
            let input = &test.to_string();
            assert!(!is_float(input));
        }
    }

    #[test]
    fn test_is_string() {
        let test_case_passes = [
            "\"1\"",
            "\"123\"",
            "\"1234567890\"",
            "\"9223372036854775807\"",
            "\"-1\"",
            "\"-123\"",
            "\"-1234567890\"",
            "\"-9223372036854775808\"",
        ];
        for test in test_case_passes {
            let input = &test.to_string();
            assert!(is_string(input));
        }
        let test_case_fails = ["\"1a", "9223372036854775808\"", "'-1a'", ""];
        for test in test_case_fails {
            let input = &test.to_string();
            assert!(!is_string(input));
        }
    }

    #[test]
    fn test_get_args_from_dyn_fn() {
        let test_cases = ["()", "(i64", "(i64, i64)"];
        let test_case_passes = [0, 1, 2];
        for t in 0..test_case_passes.len() as usize {
            let input = &test_cases[t].to_string();
            let output = test_case_passes[t] as usize;
            assert_eq!(get_args_from_dyn_fn(input), output);
        }
    }

    #[test]
    fn test_concatenate_vec_strings() {
        let test_cases = [
            vec![("1".to_string(), 0 as usize, 0 as usize, 0 as usize)],
            vec![("1 2 3".to_string(), 0 as usize, 0 as usize, 4 as usize)],
            vec![
                ("1".to_string(), 0 as usize, 0 as usize, 0 as usize),
                ("1".to_string(), 0 as usize, 0 as usize, 0 as usize),
            ],
            vec![
                ("1 2 3".to_string(), 0 as usize, 0 as usize, 4 as usize),
                ("1 2 3".to_string(), 0 as usize, 0 as usize, 4 as usize),
            ],
            vec![
                ("1 2 3".to_string(), 0 as usize, 0 as usize, 4 as usize),
                (" 1 2 3".to_string(), 0 as usize, 0 as usize, 5 as usize),
                (" 1 2 3".to_string(), 0 as usize, 0 as usize, 5 as usize),
                (" 1 2 3".to_string(), 0 as usize, 0 as usize, 5 as usize),
            ],
            vec![
                (
                    "               \r\n1".to_string(),
                    0 as usize,
                    0 as usize,
                    17 as usize,
                ),
                (
                    "               \r\n1".to_string(),
                    0 as usize,
                    0 as usize,
                    17 as usize,
                ),
            ],
            vec![
                (
                    "               \r\n1 2 3".to_string(),
                    0 as usize,
                    0 as usize,
                    21 as usize,
                ),
                (
                    "               \r\n1 2 3".to_string(),
                    0 as usize,
                    0 as usize,
                    21 as usize,
                ),
            ],
        ];
        let test_case_passes = [
            "1",
            "1 2 3",
            "11",
            "1 2 31 2 3",
            "1 2 3 1 2 3 1 2 3 1 2 3",
            "               \r\n1               \r\n1",
            "               \r\n1 2 3               \r\n1 2 3",
        ];
        for t in 0..test_case_passes.len() as usize {
            let input = &test_cases[t];
            let output = (&test_case_passes[t]).to_string();
            assert_eq!(concatenate_vec_strings(input), output);
        }
    }

    #[test]
    fn test_strip_leading_whitespace() {
        let test_case_passes = [
            ["1", "1"],
            ["1 2 3", "1 2 3"],
            [" 1", "1"],
            [" 1 2 3", "1 2 3"],
            ["               \r\n1", "1"],
            ["               \r\n1 2 3", "1 2 3"],
        ];
        for test in test_case_passes {
            let input = &test[0].to_string();
            let output = (&test[1]).to_string();
            assert_eq!(strip_leading_whitespace(input), output);
        }
    }

    #[test]
    fn test_strip_trailing_whitespace() {
        let test_case_passes = [
            ["1", "1"],
            ["1 2 3", "1 2 3"],
            ["1 ", "1"],
            ["1 2 3 ", "1 2 3"],
            ["1               \r\n", "1"],
            ["1 2 3               \r\n", "1 2 3"],
        ];
        for test in test_case_passes {
            let input = &test[0].to_string();
            let output = (&test[1]).to_string();
            assert_eq!(strip_trailing_whitespace(input), output);
        }
    }
}
