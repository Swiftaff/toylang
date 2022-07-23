use crate::ast::elements::Element;

pub fn get_current_parent_element_from_parents(ast: &mut super::Ast) -> Element {
    let parent_ref = ast.get_current_parent_ref_from_parents();
    ast.elements[parent_ref].clone()
}

pub fn get_current_parent_ref_from_parents(ast: &mut super::Ast) -> usize {
    let last = ast.parents.len() - 1;
    ast.parents[last]
}

pub fn get_current_parent_element_from_element_children_search(
    ast: &mut super::Ast,
    child_ref: usize,
) -> Option<Element> {
    if let Some(index) = ast.get_current_parent_ref_from_element_children_search(child_ref) {
        return Some(ast.elements[index].clone());
    }
    None
}

pub fn get_current_parent_ref_from_element_children_search(
    ast: &mut super::Ast,
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

pub fn get_indent(ast: &super::Ast) -> String {
    " ".repeat(4 * (ast.parents.len() - 1))
}

pub fn indent(ast: &mut super::Ast) {
    ast.parents.push(ast.elements.len() - 1);
}

pub fn indent_this(ast: &mut super::Ast, index: usize) {
    ast.parents.push(index);
}

pub fn outdent(ast: &mut super::Ast) {
    ast.parents = if ast.parents.len() < 2 {
        vec![0]
    } else {
        vec_remove_tail(&ast.parents)
    };
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
