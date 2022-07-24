use crate::ast::parents;
use crate::elements;
use crate::elements::{Element, ElementInfo};
use crate::errors;
use crate::errors::ERRORS;
use crate::Compiler;
use crate::Tokens;

pub fn types(compiler: &mut Compiler, index_of_type: usize) -> Result<(), ()> {
    // TODO error checking
    elements::append::types(compiler, index_of_type)
}

pub fn parse_current_line(compiler: &mut Compiler) -> Result<(), ()> {
    let tokens = compiler.lines_of_tokens[compiler.current_line].clone();
    if tokens.len() > 0 {
        while compiler.current_line_token < tokens.len() {
            parse_current_token(compiler, &tokens)?;
            compiler.current_line_token = compiler.current_line_token + 1;
        }
    }
    Ok(())
}

pub fn parse_current_token(compiler: &mut Compiler, tokens: &Tokens) -> Result<(), ()> {
    let current_token = &tokens[compiler.current_line_token];
    let current_token_vec: &Vec<char> = &tokens[compiler.current_line_token].chars().collect();
    if current_token_vec.len() == 0 {
        return Ok(());
    }

    match elements::get_inbuilt_function_index_by_name(&mut compiler.ast, &current_token) {
        Some(index_of_function) => {
            //dbg!(&current_token);
            let func = &compiler.ast.elements[index_of_function];
            match &func.0 {
                ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => {
                    parse_inbuilt_function_call(compiler, &current_token, index_of_function)
                }
                ElementInfo::FunctionDef(_, _, _, _) => {
                    parse_function_call(compiler, &current_token, index_of_function)
                }
                ElementInfo::Arg(_, _, returntype) => {
                    if returntype.contains("&dyn Fn") {
                        parse_function_call(compiler, &current_token, index_of_function)
                    } else {
                        parse_token_by_first_chars(compiler, &current_token, &current_token_vec)
                    }
                }
                _ => parse_token_by_first_chars(compiler, &current_token, &current_token_vec),
            }
        }
        _ => match elements::get_inbuilt_type_index_by_name(&mut compiler.ast, &current_token) {
            Some(index_of_type) => types(compiler, index_of_type),
            _ => parse_token_by_first_chars(compiler, &current_token, &current_token_vec),
        },
    }
}

pub fn parse_token_by_first_chars(
    compiler: &mut Compiler,
    current_token: &String,
    current_token_vec: &Vec<char>,
) -> Result<(), ()> {
    let first_char = current_token_vec[0];
    let second_char = if current_token_vec.len() > 1 {
        Some(current_token_vec[1])
    } else {
        None
    };
    match first_char {
        '\\' => parse_function_definition_start(compiler),
        ':' => parse_function_definition_end(compiler),
        '(' => parse_functiontypesig_or_functionreference_start(compiler),
        ')' => parse_functiontypesig_or_functionreference_end(compiler),
        '/' => parse_comment_single_line(compiler, current_token_vec),
        '=' => {
            if current_token_vec.len() > 1 {
                return errors::append_error(compiler, 0, 1, ERRORS.assign);
            }
            parse_assignment(compiler)
        }
        '"' => parse_string(compiler, &current_token),
        //positive numbers
        first_char if is_integer(&first_char.to_string()) => {
            if is_float(&current_token) {
                parse_float(compiler, &current_token)
            } else {
                parse_int(compiler, &current_token)
            }
        }
        //negative numbers
        '-' => match second_char {
            Some(_digit) => {
                if is_float(&current_token) {
                    parse_float(compiler, &current_token)
                } else {
                    parse_int(compiler, &current_token)
                }
            }
            None => {
                return errors::append_error(compiler, 0, 1, ERRORS.int_negative);
            }
        },
        first_char if "abcdefghijklmnopqrstuvwxyz".contains(&first_char.to_string()) => {
            //dbg!("constant or constantRef", first_char);
            parse_constant(compiler, &current_token)
        }
        _ => Err(()),
    }
}

pub fn parse_comment_single_line(
    compiler: &mut Compiler,
    current_token_vec: &Vec<char>,
) -> Result<(), ()> {
    if current_token_vec.len() < 2 || current_token_vec[1] != '/' {
        return errors::append_error(compiler, 0, 1, ERRORS.comment_single_line);
    }
    let val = concatenate_vec_strings(&compiler.lines_of_tokens[compiler.current_line]);
    elements::append::comment_single_line(&mut compiler.ast, val)
}

pub fn parse_string(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    if is_string(&current_token) {
        elements::append::string(compiler, current_token)
    } else {
        //dbg!(&self.lines_of_tokens);
        errors::append_error(compiler, 0, 1, ERRORS.string)
    }
}

pub fn parse_int(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    //dbg!("parse_int - positive only for now");
    let all_chars_are_numeric = current_token.chars().into_iter().all(|c| c.is_numeric());
    let chars: Vec<char> = current_token.chars().collect();
    let first_char_is_negative_sign = chars[0] == '-';
    let is_negative_all_other_chars_are_not_numeric = first_char_is_negative_sign
        && chars.len() > 1
        && !chars[1..chars.len()].into_iter().all(|c| c.is_numeric());

    if (!first_char_is_negative_sign && !all_chars_are_numeric)
        || is_negative_all_other_chars_are_not_numeric
    {
        errors::append_error(compiler, 0, 1, ERRORS.int)?;
    }
    match current_token.parse::<i64>() {
        Ok(_) => (),
        Err(_) => errors::append_error(compiler, 0, 1, ERRORS.int_out_of_bounds)?,
    }
    elements::append::int(compiler, current_token)
}

pub fn parse_float(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    if current_token.len() > 0 && is_float(current_token) {
        elements::append::float(compiler, current_token)
    } else {
        return errors::append_error(compiler, 0, 1, ERRORS.float);
    }
}

pub fn parse_constant(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    //dbg!(current_token);
    let el_option = elements::get_element_by_name(&compiler.ast, current_token);
    match el_option {
        Some(_) => {
            if elements::is_existing_constant(compiler) {
                return errors::append_error(compiler, 0, 1, ERRORS.constants_are_immutable);
            }
            match el_option {
                Some((ElementInfo::Constant(_, returntype), _)) => {
                    return elements::append::constant_ref(compiler, current_token, &returntype);
                }
                Some((ElementInfo::Arg(_, _, returntype), _)) => {
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
                // explicitly listing other types rather than using _ to not overlook new types in future
                Some((ElementInfo::Root, _)) => (),
                Some((ElementInfo::CommentSingleLine(_), _)) => (),
                Some((ElementInfo::Int(_), _)) => (),
                Some((ElementInfo::Float(_), _)) => (),
                Some((ElementInfo::String(_), _)) => (),
                Some((ElementInfo::ConstantRef(_, _, _), _)) => (),
                Some((ElementInfo::Assignment, _)) => (),
                Some((ElementInfo::InbuiltFunctionDef(_, _, _, _, _), _)) => (),
                Some((ElementInfo::InbuiltFunctionCall(_, _, _), _)) => (),
                Some((ElementInfo::FunctionDefWIP, _)) => (),
                Some((ElementInfo::FunctionCall(_, _), _)) => (),
                Some((ElementInfo::Parens, _)) => (),
                Some((ElementInfo::Type(_), _)) => (),
                Some((ElementInfo::Eol, _)) => (),
                Some((ElementInfo::Seol, _)) => (),
                Some((ElementInfo::Indent, _)) => (),
                Some((ElementInfo::Unused, _)) => (),
                None => (),
            }
        }
        None => (),
    }
    return elements::append::new_constant_or_arg(compiler, current_token);
}

pub fn parse_assignment(compiler: &mut Compiler) -> Result<(), ()> {
    // TODO error checking
    elements::append::assignment(compiler)
}

pub fn parse_inbuilt_function_call(
    compiler: &mut Compiler,
    current_token: &String,
    index_of_function: usize,
) -> Result<(), ()> {
    //TODO error checking
    elements::append::inbuilt_function_call(compiler, current_token, index_of_function)
}

pub fn parse_function_call(
    compiler: &mut Compiler,
    current_token: &String,
    index_of_function: usize,
) -> Result<(), ()> {
    elements::append::function_call1(compiler, current_token, index_of_function)
}

pub fn parse_function_definition_start(compiler: &mut Compiler) -> Result<(), ()> {
    elements::append::function_definition_start(compiler)
}

pub fn parse_function_definition_end(compiler: &mut Compiler) -> Result<(), ()> {
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
    if let Some(func_def_ref) = parents::get_current_parent_ref_from_element_children_search(
        &compiler.ast,
        compiler.ast.elements.len() - 1,
    ) {
        //get child refs
        let func_def: Element = compiler.ast.elements[func_def_ref].clone();
        let children = func_def.1.clone();
        //dbg!(&children);
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
            _ => return errors::append_error(compiler, 0, 1, ERRORS.funcdef_argtypes_first),
        }

        match func_def.0 {
            ElementInfo::FunctionDefWIP => {
                //Constant is parent of functionDefWIP
                if let Some(constant_ref) =
                    parents::get_current_parent_ref_from_element_children_search(
                        &compiler.ast,
                        func_def_ref,
                    )
                {
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
                                compiler.ast.elements[assignment_ref] =
                                    (ElementInfo::Unused, vec![]);

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
    }
    elements::append::outdent_if_last_expected_child(compiler);
    Ok(())
}

//TODO remember to error / or at least check if reusing arg names in nested functions

pub fn parse_functiontypesig_or_functionreference_start(compiler: &mut Compiler) -> Result<(), ()> {
    elements::append::functiontypesig_or_functionreference_start(compiler)
}

pub fn parse_functiontypesig_or_functionreference_end(compiler: &mut Compiler) -> Result<(), ()> {
    elements::append::functiontypesig_or_functionreference_end(compiler)
}

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

pub fn is_string(text: &String) -> bool {
    let mut is_valid = true;
    let char_vec: Vec<char> = text.chars().collect();
    if text.len() < 2 || char_vec[0] != '"' || char_vec[text.len() - 1] != '"' {
        is_valid = false;
    }
    is_valid
}

pub fn get_args_from_dyn_fn(string: &String) -> usize {
    string.matches(",").count() + (!string.contains("()") as usize)
    //0 args, e.g. "&dyn Fn() -> i64"         = 0 commas + 0 does match ()
    //1 args, e.g. "&dyn Fn(i64) -> i64"      = 0 commas + 1 does not match ()
    //2 args, e.g. "&dyn Fn(i64, i64) -> i64" = 1 comma  + 1 does not match ()
}

pub fn concatenate_vec_strings(tokens: &Tokens) -> String {
    let mut output = "".to_string();
    for i in 0..tokens.len() {
        output = format!("{}{}", output, tokens[i]);
    }
    output
}

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
