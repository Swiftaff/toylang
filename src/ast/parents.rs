pub mod indent;
pub mod outdent;
use crate::ast::elements::Element;
use crate::Ast;

pub fn get_current_parent_element_from_parents(ast: &Ast) -> Element {
    let parent_ref = get_current_parent_ref_from_parents(ast);
    ast.elements[parent_ref].clone()
}

pub fn get_current_parent_ref_from_parents(ast: &Ast) -> usize {
    let last = ast.parents.len() - 1;
    ast.parents[last]
}

pub fn get_current_parent_element_from_element_children_search(
    ast: &Ast,
    child_ref: usize,
) -> Option<Element> {
    if let Some(index) = get_current_parent_ref_from_element_children_search(ast, child_ref) {
        return Some(ast.elements[index].clone());
    }
    None
}

pub fn get_current_parent_ref_from_element_children_search(
    ast: &Ast,
    child_ref: usize,
) -> Option<usize> {
    if let Some(index) = ast
        .elements
        .iter()
        .position(|(_, children)| children.contains(&child_ref))
    {
        return Some(index);
    }
    None
}

pub fn get_indent(ast: &Ast) -> String {
    " ".repeat(4 * (ast.parents.len() - 1))
}

pub fn vec_remove_head(stack: &Vec<usize>) -> Vec<usize> {
    if stack.len() == 1 {
        vec![]
    } else {
        stack[1..].to_vec()
    }
}

pub fn vec_remove_tail(stack: &Vec<usize>) -> Vec<usize> {
    if stack.len() == 1 {
        vec![]
    } else {
        stack[..stack.len() - 1].to_vec()
    }
}
