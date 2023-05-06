/*! Functions to append each Element to the AST
 */

use crate::ast::elements;
use crate::ast::elements::{Element, ElementInfo};
use crate::ast::parents;
use crate::ast::parents::outdent;
use crate::errors;
use crate::Ast;
use crate::Compiler;

use super::ArgModifier;

/// Append Element
pub fn append(ast: &mut Ast, element: Element) -> usize {
    // add element to list, and add to list of children of current parent where 0 = root
    ast.elements.push(element.clone());
    let new_items_index = ast.elements.len() - 1;
    let current_parent_ref = parents::get_current_parent_ref_from_parents(ast);
    ast.elements[current_parent_ref].1.push(new_items_index);
    //println!("AST append: {:?}", element);
    new_items_index
}

/*
/// Append element to end of AST (for reference purposes) but don't add as child to any element
pub fn _append_as_ref(ast: &mut Ast, element: Element) -> usize {
    ast.elements.push(element);
    let new_items_index = ast.elements.len() - 1;
    new_items_index
}
*/

/// Append to the end of the AST, but add as nth child of an existing element.
/// If n is greater then the number of children, just add to the end.
pub fn append_as_nth_child_of_elindex(
    ast: &mut Ast,
    element: Element,
    parent_index: usize,
    position: usize,
) -> usize {
    ast.elements.push(element);
    let new_items_index = ast.elements.len() - 1;

    let parent = ast.elements[parent_index].clone();
    let parent_children = parent.1;
    let mut new_children = vec![];

    if position > parent_children.len() {
        new_children = parent_children;
        new_children.push(new_items_index);
    } else {
        for i in 0..parent_children.len() as usize {
            new_children.push(parent_children[i]);
            if i == position {
                new_children.push(new_items_index);
            }
        }
    }
    // replace previous parent
    ast.elements[parent_index].1 = new_children;

    new_items_index
}

/// Append the ref as a type as a child of current parent, except if parent is a list you can only have child elements as types
/// to help define the list type in an empty list, e.g. List \[ f64 \]
/// so just apply the type to the list now, and DON'T add the type into the AST
pub fn types(compiler: &mut Compiler, index_of_type: usize) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("append::types {:?}", index_of_type));
    indent_if_first_in_line(compiler);
    let el = compiler.ast.elements[index_of_type].clone();
    let parent = parents::get_current_parent_element_from_parents(&compiler.ast);
    match parent.0 {
        ElementInfo::List(_) => {
            let list_ref = parents::get_current_parent_ref_from_parents(&compiler.ast);
            let list_type = elements::get_elementinfo_type(&compiler.ast, &el.0);
            let vec_type = format!("Vec<{}>", list_type);
            compiler.ast.elements[list_ref].0 = ElementInfo::List(vec_type);
        }
        _ => {
            append(&mut compiler.ast, el);
        }
    }
    Ok(())
}

/// Append an indent if this is the first element in the line
/// or if first part of the expression in a single line function (after the colon)
/// e.g. the "+ 123 arg1"  in
/// ```text
///  "= a \\ i64 i64 arg1 : + 123 arg1"
/// ```
pub fn indent_if_first_in_line(compiler: &mut Compiler) {
    compiler
        .ast
        .log(format!("append::indent_if_first_in_line {:?}", ""));

    if compiler.current_line_token == 0 {
        append(&mut compiler.ast, (ElementInfo::Indent, vec![]));
    }
}

/// Append Comment single line
pub fn comment_single_line(compiler: &mut Compiler, val: String) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("append::comment_single_line {:?}", val));
    append(&mut compiler.ast, (ElementInfo::Indent, vec![]));
    append(
        &mut compiler.ast,
        (ElementInfo::CommentSingleLine(val), vec![]),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    append(&mut compiler.ast, (ElementInfo::Eol, vec![]));
    Ok(())
}

/// Append Println
pub fn println(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("append::println {:?}", ""));
    append(&mut compiler.ast, (ElementInfo::Indent, vec![]));
    append(&mut compiler.ast, (ElementInfo::Println, vec![]));
    errors::error_if_parent_is_invalid(compiler)?;
    parents::indent::indent(&mut compiler.ast);
    seol_if_last_in_line(compiler)
}

/// Append If
pub fn if_expression(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("append::if_expression {:?}", ""));
    let undefined = "Undefined".to_string();
    append(&mut compiler.ast, (ElementInfo::Indent, vec![]));
    append(&mut compiler.ast, (ElementInfo::If(undefined), vec![]));
    errors::error_if_parent_is_invalid(compiler)?;
    parents::indent::indent(&mut compiler.ast);
    seol_if_last_in_line(compiler)
}

/// Append String
pub fn string(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("append::string {:?}", current_token));
    indent_if_first_in_line(compiler);
    append(
        &mut compiler.ast,
        (ElementInfo::String(current_token.clone()), vec![]),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    outdent_if_last_expected_child(compiler);
    seol_if_last_in_line(compiler)
}

/// Append outdent, if it is the last expected child of current parent
///
/// Loop upwards through parents until reaching root or some scenario causes no further progress.
///
/// For all parents check if we should outdent, since you may need to outdent multiple times
/// depending on each parent
pub fn outdent_if_last_expected_child(compiler: &mut Compiler) {
    compiler
        .ast
        .log(format!("append::outdent_if_last_expected_child {:?}", ""));
    let mut prev_parents_len = 999999999;

    loop {
        let parent_is_root = compiler.ast.parents.len() < 2;
        let parent_is_same_as_last_time = compiler.ast.parents.len() == prev_parents_len;
        if parent_is_root || parent_is_same_as_last_time {
            break;
        }
        prev_parents_len = compiler.ast.parents.len();

        //decide if we should outdent based on current_parent
        let current_parent_ref = parents::get_current_parent_ref_from_parents(&mut compiler.ast);
        let current_parent = compiler.ast.elements[current_parent_ref].clone();
        match current_parent.0.clone() {
            ElementInfo::Println => {
                outdent::println(compiler, current_parent);
            }
            ElementInfo::Struct(_, _, _) => (), //the end_struct tag will outdent insstead of this start_struct tag
            ElementInfo::Constant(_, _) => {
                outdent::constant(compiler, current_parent);
            }
            ElementInfo::Assignment => {
                outdent::assignment(compiler, current_parent);
            }
            ElementInfo::InbuiltFunctionCall(_, fndefref, _) => {
                outdent::inbuiltfncall_from_inbuiltfndef(compiler, current_parent, fndefref);
            }
            ElementInfo::FunctionDef(_, _argnames, _, _) => {
                outdent::within_fndef_from_return_expression(compiler);
            }
            ElementInfo::FunctionCall(name, _) => {
                outdent::fncall(compiler, current_parent, name);
            }
            ElementInfo::If(_) => {
                outdent::if_expression(compiler, current_parent);
            }
            // explicitly listing other types rather than using _ to not overlook new types in future
            ElementInfo::Root => (),
            ElementInfo::List(_) => (),
            ElementInfo::CommentSingleLine(_) => (),
            ElementInfo::Int(_) => (),
            ElementInfo::Float(_) => (),
            ElementInfo::String(_) => (),
            ElementInfo::Bool(_) => (),
            ElementInfo::Arg(_, _, _, _) => (),
            ElementInfo::ConstantRef(_, _, _) => (),
            ElementInfo::InbuiltFunctionDef(_, _, _, _, _, _) => (),
            ElementInfo::FunctionDefWIP => (),
            ElementInfo::Parens => (),
            ElementInfo::Type(_) => (),
            ElementInfo::Eol => (),
            ElementInfo::Seol => (),
            ElementInfo::Indent => (),
            ElementInfo::Unused => (),
            ElementInfo::LoopForRangeWIP => (),
            ElementInfo::LoopForRange(_, _, _) => (),
        }
    }
}

/// Append an SEOL (Semicolon and End of line) if the current element is last in this line
pub fn seol_if_last_in_line(compiler: &mut Compiler) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("append::seol_if_last_in_line {:?}", ""));
    let is_last_token_in_this_line =
        compiler.current_line_token == compiler.lines_of_tokens[compiler.current_line].len() - 1;
    let mut append_seol: bool = true;
    if is_last_token_in_this_line {
        for el_index in (0..compiler.ast.elements.len()).rev() {
            let el = &compiler.ast.elements[el_index];
            match el.0 {
                ElementInfo::Indent => {
                    // get start of current line

                    if el_index != compiler.ast.elements.len() - 1 {
                        let first_element_after_indent_ref = el_index + 1;

                        //let first_element = &compiler.ast.elements[first_element_after_indent_ref];

                        let parent_of_first_el_option =
                            parents::get_current_parent_element_from_element_children_search(
                                &mut compiler.ast,
                                first_element_after_indent_ref,
                            );
                        let first_element_after_indent_el =
                            compiler.ast.elements[first_element_after_indent_ref].clone();

                        // Add "as i64" to any int as return expression, since Rust seems to type infer it as i32 otherwise
                        match first_element_after_indent_el.0.clone() {
                            ElementInfo::Int(x) => {
                                compiler.ast.elements[first_element_after_indent_ref] = (
                                    ElementInfo::Int(format!("{} as i64", x)),
                                    first_element_after_indent_el.1.clone(),
                                )
                            }
                            _ => (),
                        }

                        //don't add semicolon if it is the return expression
                        match parent_of_first_el_option {
                            Some((ElementInfo::FunctionDef(_, _, _, _), _)) => {
                                if is_return_expression(&first_element_after_indent_el.0) {
                                    append_seol = false;
                                }
                            }
                            _ => (),
                        }
                        break;
                    }
                }
                _ => (),
            }
        }

        // if is the last return expression of a func_def
        // then don't add the semicolon, just the EOL
        if append_seol {
            //self.ast.append((ElementInfo::Eol, vec![]));
            append(&mut compiler.ast, (ElementInfo::Seol, vec![]));
        }
    }
    Ok(())
}

/// Check if the ElementInfo is a return expression
pub fn is_return_expression(elinfo: &ElementInfo) -> bool {
    match elinfo {
        ElementInfo::List(_) => true,
        ElementInfo::Int(_) => true,
        ElementInfo::Float(_) => true,
        ElementInfo::String(_) => true,
        ElementInfo::Bool(_) => true,
        ElementInfo::Struct(_, _, _) => true,
        ElementInfo::Constant(_, _) => true,
        ElementInfo::ConstantRef(_, _, _) => true,
        ElementInfo::InbuiltFunctionCall(_, _, _) => true,
        ElementInfo::FunctionCall(_, _) => true,
        ElementInfo::If(_) => true,
        ElementInfo::Parens => true,
        // explicitly listing other types rather than using _ to not overlook new types in future
        ElementInfo::Root => false,
        ElementInfo::CommentSingleLine(_) => false,
        ElementInfo::Arg(_, _, _, _) => false,
        ElementInfo::Assignment => false,
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _, _) => false,
        ElementInfo::FunctionDefWIP => false,
        ElementInfo::FunctionDef(_, _, _, _) => false,
        ElementInfo::Type(_) => false,
        ElementInfo::Eol => false,
        ElementInfo::Seol => false,
        ElementInfo::Indent => false,
        ElementInfo::Unused => false,
        ElementInfo::LoopForRangeWIP => false,
        ElementInfo::LoopForRange(_, _, _) => false,
        ElementInfo::Println => false,
    }
}

/// Append Int
pub fn int(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    compiler.ast.log(format!("append::int {:?}", current_token));
    indent_if_first_in_line(compiler);
    append(
        &mut compiler.ast,
        (ElementInfo::Int(current_token.clone()), vec![]),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    outdent_if_last_expected_child(compiler);
    seol_if_last_in_line(compiler)
}

///Append Float
pub fn float(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("append::float {:?}", current_token));
    indent_if_first_in_line(compiler);
    append(
        &mut compiler.ast,
        (ElementInfo::Float(current_token.clone()), vec![]),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    outdent_if_last_expected_child(compiler);
    seol_if_last_in_line(compiler)
}

/// Append Assignment
pub fn assignment(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("append::assignment {:?}", ""));
    indent_if_first_in_line(compiler);
    append(&mut compiler.ast, ((ElementInfo::Assignment), vec![]));
    errors::error_if_parent_is_invalid(compiler)?;
    outdent_if_last_expected_child(compiler);
    parents::indent::indent(&mut compiler.ast);
    seol_if_last_in_line(compiler)
}

///Append InbuiltFnCall
pub fn inbuilt_function_call(
    compiler: &mut Compiler,
    current_token: &String,
    index_of_function: usize,
) -> Result<(), ()> {
    compiler.ast.log(format!(
        "append::int {:?} {:?}",
        current_token, index_of_function
    ));
    indent_if_first_in_line(compiler);
    let el = &compiler.ast.elements[index_of_function];
    let returntype = elements::get_elementinfo_type(&compiler.ast, &el.0);
    match el.clone().0 {
        ElementInfo::InbuiltFunctionDef(_, argnames, _, _, _, _) => {
            append(
                &mut compiler.ast,
                (
                    ElementInfo::InbuiltFunctionCall(
                        current_token.clone(),
                        index_of_function,
                        returntype,
                    ),
                    vec![],
                ),
            );
            errors::error_if_parent_is_invalid(compiler)?;
            outdent_if_last_expected_child(compiler);
            if argnames.len() > 0 {
                parents::indent::indent(&mut compiler.ast);
            }
            seol_if_last_in_line(compiler)
        }
        //should not error here
        _ => Ok(()),
    }
}

/// Append FnCall
pub fn function_call1(
    compiler: &mut Compiler,
    current_token: &String,
    index_of_function: usize,
) -> Result<(), ()> {
    compiler.ast.log(format!(
        "append::function_call1 {:?} {:?}",
        current_token, index_of_function
    ));
    //TODO find difference with other append_function_call
    indent_if_first_in_line(compiler);
    let el = &compiler.ast.elements[index_of_function];
    let returntype = elements::get_elementinfo_type(&compiler.ast, &el.0);
    append(
        &mut compiler.ast,
        (
            ElementInfo::FunctionCall(current_token.clone(), returntype),
            vec![],
        ),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    outdent_if_last_expected_child(compiler);
    parents::indent::indent(&mut compiler.ast);
    seol_if_last_in_line(compiler)
}

/// Append start of a struct
pub fn struct_start(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("append::struct_start {:?}", ""));
    indent_if_first_in_line(compiler);
    let parent = parents::get_current_parent_element_from_parents(&compiler.ast);
    let mut struct_name = "Undefined".to_string();
    if let ElementInfo::Constant(name, _) = parent.0 {
        struct_name = name;
    };
    struct_name = struct_name.to_lowercase().replace("_", "");
    let mut temp_struct_name: Vec<char> = struct_name.chars().collect();
    temp_struct_name[0] = temp_struct_name[0].to_uppercase().nth(0).unwrap();
    struct_name = temp_struct_name.into_iter().collect::<String>();
    append(
        &mut compiler.ast,
        (ElementInfo::Struct(struct_name, vec![], vec![]), vec![]),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    parents::indent::indent(&mut compiler.ast);
    Ok(())
}

/// Append start of a list
pub fn list_start(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!("append::list_start {:?}", ""));
    indent_if_first_in_line(compiler);
    append(
        &mut compiler.ast,
        (ElementInfo::List("Undefined".to_string()), vec![]),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    parents::indent::indent(&mut compiler.ast);
    Ok(())
}

/// Append start of a FnDef
pub fn function_definition_start(compiler: &mut Compiler) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("append::function_definition_start {:?}", ""));
    indent_if_first_in_line(compiler);
    append(&mut compiler.ast, (ElementInfo::FunctionDefWIP, vec![]));
    errors::error_if_parent_is_invalid(compiler)?;
    parents::indent::indent(&mut compiler.ast);
    seol_if_last_in_line(compiler)
}

/// Append start of a loop for range
pub fn loop_for_range_start(compiler: &mut Compiler) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("append::loop_for_range_start {:?}", ""));
    indent_if_first_in_line(compiler);
    append(&mut compiler.ast, (ElementInfo::LoopForRangeWIP, vec![]));
    errors::error_if_parent_is_invalid(compiler)?;
    parents::indent::indent(&mut compiler.ast);
    seol_if_last_in_line(compiler)
}

/// Append Fn type signature, or start of a FnRef
pub fn functiontypesig_or_functionreference_start(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!(
        "append::functiontypesig_or_functionreference_start {:?}",
        ""
    ));
    indent_if_first_in_line(compiler);
    append(&mut compiler.ast, (ElementInfo::Parens, vec![]));
    errors::error_if_parent_is_invalid(compiler)?;
    parents::indent::indent(&mut compiler.ast);
    seol_if_last_in_line(compiler)
}

/// Append Fn type signature, or end of a FnRef
pub fn functiontypesig_or_functionreference_end(compiler: &mut Compiler) -> Result<(), ()> {
    compiler.ast.log(format!(
        "append::functiontypesig_or_functionreference_end {:?}",
        ""
    ));
    parents::outdent::outdent(compiler);
    outdent_if_last_expected_child(compiler);
    seol_if_last_in_line(compiler)
}

/// Append ConstantRef
pub fn constant_ref(
    compiler: &mut Compiler,
    current_token: &String,
    returntype: &String,
) -> Result<(), ()> {
    compiler.ast.log(format!(
        "append::constant_ref {:?} {:?}",
        current_token, returntype
    ));
    indent_if_first_in_line(compiler);
    append(
        &mut compiler.ast,
        (
            ElementInfo::ConstantRef(
                current_token.clone(),
                returntype.clone(),
                current_token.clone(),
            ),
            vec![],
        ),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    outdent_if_last_expected_child(compiler);
    seol_if_last_in_line(compiler)
}

/// Append a new constant, or an Argument if within a WIP FnDef
pub fn new_constant_or_arg(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    compiler
        .ast
        .log(format!("append::new_constant_or_arg {:?}", current_token));
    let typename = "Undefined".to_string();
    indent_if_first_in_line(compiler);
    //TODO change this to inbuiltfunction?

    let parent_ref = parents::get_current_parent_ref_from_parents(&mut compiler.ast);
    let parent = compiler.ast.elements[parent_ref].clone();
    match parent.0 {
        ElementInfo::FunctionDefWIP => {
            append(
                &mut compiler.ast,
                (
                    ElementInfo::Arg(
                        current_token.clone(),
                        parent_ref,
                        ArgModifier::None,
                        "Undefined".to_string(),
                    ),
                    vec![],
                ),
            );
            errors::error_if_parent_is_invalid(compiler)?;
        }
        _ => {
            append(
                &mut compiler.ast,
                (
                    ElementInfo::Constant(current_token.clone(), typename),
                    vec![],
                ),
            );
            errors::error_if_parent_is_invalid(compiler)?;
            parents::indent::indent(&mut compiler.ast);
        }
    }

    outdent_if_last_expected_child(compiler);
    seol_if_last_in_line(compiler)
}

/// Append FnRef or FnCall
pub fn function_ref_or_call(
    compiler: &mut Compiler,
    current_token: &String,
    args: usize,
    returntype: &String,
) -> Result<(), ()> {
    compiler.ast.log(format!(
        "append::function_ref_or_call {:?} {:?} {:?}",
        current_token, args, returntype
    ));
    indent_if_first_in_line(compiler);

    let parent = parents::get_current_parent_element_from_parents(&mut compiler.ast);
    match parent.0 {
        ElementInfo::Parens => return handle_parens(compiler, current_token, returntype),
        _ => return function_call(compiler, current_token, args, returntype, true),
    }

    /// if parent is parens, then this is just a function reference, so swap out the parent parens to be a ConstantRef instead
    fn handle_parens(
        compiler: &mut Compiler,
        current_token: &String,
        returntype: &String,
    ) -> Result<(), ()> {
        let new_constant_ref: Element = (
            ElementInfo::ConstantRef(
                current_token.clone(),
                returntype.clone(),
                current_token.clone(),
            ),
            [].to_vec(),
        );
        let parens_ref = parents::get_current_parent_ref_from_parents(&compiler.ast);
        compiler.ast.elements[parens_ref] = new_constant_ref;

        check_parens_parent(compiler, current_token, parens_ref);
        return seol_if_last_in_line(compiler);
    }

    /// TODO look at again, may not be the best solution...
    /// if parens parent is an InbuiltFunctionCall it is possible it is List.map with an argmodifier on this functioncall, if so, apply argmodifier.
    /// start by getting the parens_parent
    fn check_parens_parent(compiler: &mut Compiler, current_token: &String, parens_ref: usize) {
        if let Some(parens_parent_ref) =
            parents::get_current_parent_ref_from_element_children_search(&compiler.ast, parens_ref)
        {
            check_for_inbuiltfncall(compiler, parens_ref, parens_parent_ref, current_token);
        }
    }

    /// if the parens_parent is an inbuiltfncall, check the argmodifier
    fn check_for_inbuiltfncall(
        compiler: &mut Compiler,
        parens_ref: usize,
        parens_parent_ref: usize,
        current_token: &String,
    ) {
        if let ElementInfo::InbuiltFunctionCall(name, _, _) =
            compiler.ast.elements[parens_parent_ref].0.clone()
        {
            get_parens_index(compiler, parens_ref, parens_parent_ref, name, current_token);
        }
    }

    /// Assuming that an InbuiltFunctionCall's children are only Args (as ConstantRefs)
    /// we can find out the index of the arg of this overall function
    fn get_parens_index(
        compiler: &mut Compiler,
        parens_ref: usize,
        parens_parent_ref: usize,
        name: String,
        current_token: &String,
    ) {
        let children = compiler.ast.elements[parens_parent_ref].1.clone();
        if let Some(index) = children.into_iter().position(|v| v == parens_ref) {
            get_fn_def(compiler, name, index, current_token, parens_ref);
        }
    }

    /// Then get the matching fn_def of the inbuiltfncall, to get all it's argmodifiers
    fn get_fn_def(
        compiler: &mut Compiler,
        name: String,
        index: usize,
        current_token: &String,
        parens_ref: usize,
    ) {
        if let Some(ElementInfo::InbuiltFunctionDef(_, _, _, argmodifiers, _, _)) =
            elements::get_inbuilt_function_by_name(&compiler.ast, &name)
        {
            get_fn_argmodifier(
                compiler,
                argmodifiers,
                index,
                name,
                current_token,
                parens_ref,
            );
        }
    }

    /// Get the single fn argmodifier for the fncall's Arg index
    fn get_fn_argmodifier(
        compiler: &mut Compiler,
        argmodifiers: Vec<ArgModifier>,
        index: usize,
        name: String,
        current_token: &String,
        parens_ref: usize,
    ) {
        if let ArgModifier::FnArg(fn_arg_modifier) = argmodifiers[index].clone() {
            create_name_for_duplicate_function(
                compiler,
                name,
                current_token,
                fn_arg_modifier,
                parens_ref,
            );
        }
    }

    /// Create a new name for the duplicate fn e.g. constant_name & "_for_" & name_of_this_function
    /// and if it doesn't already exist duplicate it
    fn create_name_for_duplicate_function(
        compiler: &mut Compiler,
        name: String,
        current_token: &String,
        fn_arg_modifier: Vec<String>,
        parens_ref: usize,
    ) {
        let name_snake_case = name.replace(".", "_").to_lowercase();
        let new_fn_name = format!("{}_for_{}", current_token, name_snake_case);
        if let None = elements::get_constant_index_by_name(&compiler.ast, &new_fn_name) {
            duplicate_fn(
                compiler,
                current_token,
                new_fn_name,
                fn_arg_modifier,
                parens_ref,
            );
        }
    }

    /// Duplicate the fn with updated arg types
    fn duplicate_fn(
        compiler: &mut Compiler,
        name: &String,
        new_fn_name: String,
        fn_arg_modifier: Vec<String>,
        parens_ref: usize,
    ) {
        if let Some(fn_index_being_referenced) =
            elements::get_function_index_by_name(&compiler.ast, &name)
        {
            let mut duplicate_fn = compiler.ast.elements[fn_index_being_referenced].clone();

            if let ElementInfo::FunctionDef(_, argnames, argtypes, returntype) = duplicate_fn.0 {
                // update the fn args with the fn arg modifiers
                let mut new_argtypes = argtypes.clone();
                for i in 0..argtypes.len() as usize {
                    new_argtypes[i] = format!("{}{}", fn_arg_modifier[i], new_argtypes[i]);
                }

                duplicate_fn.0 = ElementInfo::FunctionDef(
                    new_fn_name.clone(),
                    argnames,
                    new_argtypes,
                    returntype,
                );
                insert_duplicate_fn_into_ast(
                    compiler,
                    fn_index_being_referenced,
                    duplicate_fn,
                    parens_ref,
                    new_fn_name,
                );
            }
        }

        /// Inserts the duplicate fn just after the existing fn that it is based on, and under its same parent
        fn insert_duplicate_fn_into_ast(
            compiler: &mut Compiler,
            fn_index_being_referenced: usize,
            duplicate_fn: Element,
            parens_ref: usize,
            new_fn_name: String,
        ) {
            if let Some(parent_of_current_fn_ref) =
                parents::get_current_parent_ref_from_element_children_search(
                    &compiler.ast,
                    fn_index_being_referenced,
                )
            {
                let parent = compiler.ast.elements[parent_of_current_fn_ref].clone();
                let children = parent.1;
                let current_fn_position = children
                    .iter()
                    .position(|&r| r == fn_index_being_referenced)
                    .unwrap();
                elements::append::append_as_nth_child_of_elindex(
                    &mut compiler.ast,
                    duplicate_fn.clone(),
                    parent_of_current_fn_ref,
                    current_fn_position + 1,
                );

                switch_old_fn_ref_for_new(compiler, parens_ref, new_fn_name);
            }
        }

        /// And finally, switch out the reference from the original fn to the duplicate fn instead
        fn switch_old_fn_ref_for_new(
            compiler: &mut Compiler,
            parens_ref: usize,
            new_fn_name: String,
        ) {
            let constant_ref_for_current_fn = compiler.ast.elements[parens_ref].clone();
            if let ElementInfo::ConstantRef(_, returntype, _) = constant_ref_for_current_fn.0 {
                compiler.ast.elements[parens_ref] = (
                    ElementInfo::ConstantRef(new_fn_name.clone(), returntype, new_fn_name),
                    constant_ref_for_current_fn.1,
                )
            }
        }
    }
}

/// Append FnCall
pub fn function_call(
    compiler: &mut Compiler,
    current_token: &String,
    args: usize,
    returntype: &String,
    seol: bool,
) -> Result<(), ()> {
    compiler.ast.log(format!(
        "append::function_call {:?} {:?} {:?} {:?}",
        current_token, args, returntype, seol
    ));
    append(
        &mut compiler.ast,
        (
            ElementInfo::FunctionCall(current_token.clone(), returntype.clone()),
            vec![],
        ),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    if args > 0 {
        parents::indent::indent(&mut compiler.ast);
    }
    outdent_if_last_expected_child(compiler);
    if seol {
        return seol_if_last_in_line(compiler);
    } else {
        Ok(())
    }
}
