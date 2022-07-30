use crate::ast::elements;
use crate::ast::elements::{Element, ElementInfo};
use crate::ast::parents;
use crate::ast::parents::outdent;
use crate::errors;
use crate::Ast;
use crate::Compiler;

pub fn append(ast: &mut Ast, element: Element) -> usize {
    // add element to list, and add to list of children of current parent where 0 = root
    ast.elements.push(element);
    let new_items_index = ast.elements.len() - 1;
    let current_parent_ref = parents::get_current_parent_ref_from_parents(ast);
    ast.elements[current_parent_ref].1.push(new_items_index);
    new_items_index
}

pub fn _append_as_ref(ast: &mut Ast, element: Element) -> usize {
    // add element to list only, don't add as child
    ast.elements.push(element);
    let new_items_index = ast.elements.len() - 1;
    new_items_index
}

pub fn types(compiler: &mut Compiler, index_of_type: usize) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    let el = compiler.ast.elements[index_of_type].clone();
    append(&mut compiler.ast, el);
    Ok(())
}

pub fn indent_if_first_in_line(compiler: &mut Compiler) {
    //or if first part of the expression in a single line function (after the colon)
    //e.g. the "+ 123 arg1"  in "= a \\ i64 i64 arg1 : + 123 arg1"
    if compiler.current_line_token == 0 {
        append(&mut compiler.ast, (ElementInfo::Indent, vec![]));
    }
}

pub fn comment_single_line(compiler: &mut Compiler, val: String) -> Result<(), ()> {
    append(&mut compiler.ast, (ElementInfo::Indent, vec![]));
    append(
        &mut compiler.ast,
        (ElementInfo::CommentSingleLine(val), vec![]),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    append(&mut compiler.ast, (ElementInfo::Eol, vec![]));
    Ok(())
}

pub fn string(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    append(
        &mut compiler.ast,
        (ElementInfo::String(current_token.clone()), vec![]),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    outdent_if_last_expected_child(compiler);
    seol_if_last_in_line(compiler)
}

pub fn outdent_if_last_expected_child(compiler: &mut Compiler) {
    let mut prev_parents_len = 999999999;
    loop {
        //dbg!("loop", &self.ast.parents);
        if compiler.ast.parents.len() < 2 || compiler.ast.parents.len() == prev_parents_len {
            break;
        }
        prev_parents_len = compiler.ast.parents.len();
        let current_parent_ref = parents::get_current_parent_ref_from_parents(&mut compiler.ast);
        let current_parent = compiler.ast.elements[current_parent_ref].clone();
        //dbg!("---", &compiler.ast);
        match current_parent.0.clone() {
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
                outdent::fncall_from_fndef_or_arg(compiler, current_parent, name);
            }
            // explicitly listing other types rather than using _ to not overlook new types in future
            ElementInfo::Root => (),
            ElementInfo::CommentSingleLine(_) => (),
            ElementInfo::Int(_) => (),
            ElementInfo::Float(_) => (),
            ElementInfo::String(_) => (),
            ElementInfo::Arg(_, _, _) => (),
            ElementInfo::ConstantRef(_, _, _) => (),
            ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => (),
            ElementInfo::FunctionDefWIP => (),
            ElementInfo::Parens => (),
            ElementInfo::Type(_) => (),
            ElementInfo::Eol => (),
            ElementInfo::Seol => (),
            ElementInfo::Indent => (),
            ElementInfo::Unused => (),
        }
    }
}

pub fn seol_if_last_in_line(compiler: &mut Compiler) -> Result<(), ()> {
    let is_last_token_in_this_line =
        compiler.current_line_token == compiler.lines_of_tokens[compiler.current_line].len() - 1;
    let mut is_end_of_return_statement_of_a_func_def: bool = false;

    if is_last_token_in_this_line {
        for el_index in (0..compiler.ast.elements.len()).rev() {
            let el = &compiler.ast.elements[el_index];
            match el.0 {
                ElementInfo::Indent => {
                    // get start of current line

                    if el_index != compiler.ast.elements.len() - 1 {
                        let first_element_after_indent_ref = el_index + 1;
                        let parent_of_first_el_option =
                            parents::get_current_parent_element_from_element_children_search(
                                &mut compiler.ast,
                                first_element_after_indent_ref,
                            );
                        match parent_of_first_el_option {
                            Some((ElementInfo::FunctionDef(_, _, _, _), _)) => {
                                // confirm this line is a statement from a func def

                                let first_element_after_indent_el =
                                    &compiler.ast.elements[first_element_after_indent_ref];
                                match first_element_after_indent_el.0 {
                                    // confirm this statement is a return statement
                                    // i.e. must be one of these types
                                    ElementInfo::Int(_) => {
                                        is_end_of_return_statement_of_a_func_def = true;
                                    }
                                    ElementInfo::Float(_) => {
                                        is_end_of_return_statement_of_a_func_def = true;
                                    }
                                    ElementInfo::String(_) => {
                                        is_end_of_return_statement_of_a_func_def = true;
                                    }
                                    ElementInfo::Constant(_, _) => {
                                        is_end_of_return_statement_of_a_func_def = true;
                                    }
                                    ElementInfo::ConstantRef(_, _, _) => {
                                        is_end_of_return_statement_of_a_func_def = true;
                                    }
                                    ElementInfo::InbuiltFunctionCall(_, _, _) => {
                                        is_end_of_return_statement_of_a_func_def = true;
                                    }
                                    ElementInfo::FunctionCall(_, _) => {
                                        is_end_of_return_statement_of_a_func_def = true;
                                    }
                                    ElementInfo::Parens => {
                                        is_end_of_return_statement_of_a_func_def = true;
                                    }
                                    // explicitly listing other types rather than using _ to not overlook new types in future
                                    ElementInfo::Root => (),
                                    ElementInfo::CommentSingleLine(_) => (),
                                    ElementInfo::Arg(_, _, _) => (),
                                    ElementInfo::Assignment => (),
                                    ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => (),
                                    ElementInfo::FunctionDefWIP => (),
                                    ElementInfo::FunctionDef(_, _, _, _) => (),
                                    ElementInfo::Type(_) => (),
                                    ElementInfo::Eol => (),
                                    ElementInfo::Seol => (),
                                    ElementInfo::Indent => (),
                                    ElementInfo::Unused => (),
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
        if !is_end_of_return_statement_of_a_func_def {
            //self.ast.append((ElementInfo::Eol, vec![]));
            append(&mut compiler.ast, (ElementInfo::Seol, vec![]));
        }
    }
    Ok(())
}

pub fn int(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    append(
        &mut compiler.ast,
        (ElementInfo::Int(current_token.clone()), vec![]),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    outdent_if_last_expected_child(compiler);
    seol_if_last_in_line(compiler)
}

pub fn float(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    append(
        &mut compiler.ast,
        (ElementInfo::Float(current_token.clone()), vec![]),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    outdent_if_last_expected_child(compiler);
    seol_if_last_in_line(compiler)
}

pub fn assignment(compiler: &mut Compiler) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    append(&mut compiler.ast, ((ElementInfo::Assignment), vec![]));
    errors::error_if_parent_is_invalid(compiler)?;
    outdent_if_last_expected_child(compiler);
    parents::indent::indent(&mut compiler.ast);
    Ok(())
}

pub fn inbuilt_function_call(
    compiler: &mut Compiler,
    current_token: &String,
    index_of_function: usize,
) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    let el = &compiler.ast.elements[index_of_function];
    let returntype = elements::get_elementinfo_type(&compiler.ast, &el.0);
    append(
        &mut compiler.ast,
        (
            ElementInfo::InbuiltFunctionCall(current_token.clone(), index_of_function, returntype),
            vec![],
        ),
    );
    errors::error_if_parent_is_invalid(compiler)?;
    outdent_if_last_expected_child(compiler);
    parents::indent::indent(&mut compiler.ast);
    Ok(())
}

pub fn function_call1(
    compiler: &mut Compiler,
    current_token: &String,
    index_of_function: usize,
) -> Result<(), ()> {
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
    outdent_if_last_expected_child(compiler);
    parents::indent::indent(&mut compiler.ast);
    Ok(())
}

pub fn function_definition_start(compiler: &mut Compiler) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    append(&mut compiler.ast, (ElementInfo::FunctionDefWIP, vec![]));
    //self.outdent_if_last_expected_child();
    parents::indent::indent(&mut compiler.ast);
    Ok(())
}

pub fn functiontypesig_or_functionreference_start(compiler: &mut Compiler) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    append(&mut compiler.ast, (ElementInfo::Parens, vec![]));
    parents::indent::indent(&mut compiler.ast);
    Ok(())
}

pub fn functiontypesig_or_functionreference_end(compiler: &mut Compiler) -> Result<(), ()> {
    parents::outdent::outdent(compiler);
    outdent_if_last_expected_child(compiler);
    Ok(())
}

pub fn constant_ref(
    compiler: &mut Compiler,
    current_token: &String,
    returntype: &String,
) -> Result<(), ()> {
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

pub fn new_constant_or_arg(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
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
                    ElementInfo::Arg(current_token.clone(), parent_ref, "Undefined".to_string()),
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

    //dbg!("constant 1", &self.ast.parents);
    outdent_if_last_expected_child(compiler);
    //dbg!("constant 2", &self.ast.parents);
    seol_if_last_in_line(compiler)
}

pub fn function_ref_or_call(
    compiler: &mut Compiler,
    current_token: &String,
    args: usize,
    returntype: &String,
) -> Result<(), ()> {
    //dbg!("FunctionCall", &current_token);
    indent_if_first_in_line(compiler);

    let parent = parents::get_current_parent_element_from_parents(&mut compiler.ast);
    //dbg!("penguin",&parent);
    match parent.0 {
        ElementInfo::Parens => {
            // if parent is parens, then this is just a function reference
            // don't treat it like a functionCall,
            // just change the parent to be a ConstantRef

            let parent_ref = parents::get_current_parent_ref_from_parents(&compiler.ast);
            let new_constant_ref: Element = (
                ElementInfo::ConstantRef(
                    current_token.clone(),
                    returntype.clone(),
                    current_token.clone(),
                ),
                [].to_vec(),
            );
            compiler.ast.elements[parent_ref] = new_constant_ref;
            //compiler.ast.outdent();
            return seol_if_last_in_line(compiler);
        }
        _ => {
            //else it is a function call...
            return function_call(compiler, current_token, args, returntype, true);
        }
    }
}

pub fn function_call(
    compiler: &mut Compiler,
    current_token: &String,
    args: usize,
    returntype: &String,
    seol: bool,
) -> Result<(), ()> {
    append(
        &mut compiler.ast,
        (
            ElementInfo::FunctionCall(current_token.clone(), returntype.clone()),
            vec![],
        ),
    );
    if args > 0 {
        parents::indent::indent(&mut compiler.ast);
    }
    if seol {
        return seol_if_last_in_line(compiler);
    } else {
        Ok(())
    }
}
