use crate::ast::elements::{Element, ElementInfo};
use crate::ast::parents::indent;
use crate::Ast;
use crate::Compiler;

pub fn append(ast: &mut Ast, element: Element) -> usize {
    // add element to list, and add to list of children of current parent where 0 = root
    ast.elements.push(element);
    let new_items_index = ast.elements.len() - 1;
    let current_parent_ref = ast.get_current_parent_ref_from_parents();
    ast.elements[current_parent_ref].1.push(new_items_index);
    new_items_index
}

pub fn append_as_ref(ast: &mut Ast, element: Element) -> usize {
    // add element to list only, don't add as child
    ast.elements.push(element);
    let new_items_index = ast.elements.len() - 1;
    new_items_index
}

pub fn types(compiler: &mut Compiler, index_of_type: usize) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    compiler
        .ast
        .append(compiler.ast.elements[index_of_type].clone());
    Ok(())
}

pub fn indent_if_first_in_line(compiler: &mut Compiler) {
    //or if first part of the expression in a single line function (after the colon)
    //e.g. the "+ 123 arg1"  in "= a \\ i64 i64 arg1 : + 123 arg1"
    if compiler.current_line_token == 0 {
        compiler.ast.append((ElementInfo::Indent, vec![]));
    }
}

pub fn comment_single_line(ast: &mut Ast, val: String) -> Result<(), ()> {
    ast.append((ElementInfo::Indent, vec![]));
    ast.append((ElementInfo::CommentSingleLine(val), vec![]));
    ast.append((ElementInfo::Eol, vec![]));
    Ok(())
}

pub fn string(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    append(
        &mut compiler.ast,
        (ElementInfo::String(current_token.clone()), vec![]),
    );
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
        let current_parent_ref = compiler.ast.get_current_parent_ref_from_parents();
        let current_parent = compiler.ast.elements[current_parent_ref].clone();
        //dbg!("---", &compiler.ast);
        match current_parent.0.clone() {
            ElementInfo::Constant(_, _) => {
                compiler.outdent_constant(current_parent);
            }
            ElementInfo::Assignment => {
                compiler.outdent_assignment(current_parent);
            }
            ElementInfo::InbuiltFunctionCall(_, fndefref, _) => {
                compiler.outdent_inbuiltfncall_from_inbuiltfndef(current_parent, fndefref);
            }
            ElementInfo::FunctionDef(_, _argnames, _, _) => {
                compiler.outdent_within_fndef_from_return_expression();
            }
            ElementInfo::FunctionCall(name, _) => {
                compiler.outdent_fncall_from_fndef_or_arg(current_parent, name);
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
                        let parent_of_first_el_option = compiler
                            .ast
                            .get_current_parent_element_from_element_children_search(
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
            compiler.ast.append((ElementInfo::Seol, vec![]));
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
    outdent_if_last_expected_child(compiler);
    seol_if_last_in_line(compiler)
}

pub fn float(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    append(
        &mut compiler.ast,
        (ElementInfo::Float(current_token.clone()), vec![]),
    );
    outdent_if_last_expected_child(compiler);
    seol_if_last_in_line(compiler)
}

pub fn assignment(compiler: &mut Compiler) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    append(&mut compiler.ast, ((ElementInfo::Assignment), vec![]));
    outdent_if_last_expected_child(compiler);
    indent(&mut compiler.ast);
    Ok(())
}

pub fn inbuilt_function_call(
    compiler: &mut Compiler,
    current_token: &String,
    index_of_function: usize,
) -> Result<(), ()> {
    indent_if_first_in_line(compiler);
    let el = &compiler.ast.elements[index_of_function];
    let returntype = compiler.ast.get_elementinfo_type(&el.0);
    append(
        &mut compiler.ast,
        (
            ElementInfo::InbuiltFunctionCall(current_token.clone(), index_of_function, returntype),
            vec![],
        ),
    );
    outdent_if_last_expected_child(compiler);
    compiler.ast.indent();
    Ok(())
}
