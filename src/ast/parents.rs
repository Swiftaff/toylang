/*! Parents is a Vec of Element Indexes, used as a stack to push/pop the current parent element,
 * so the lastmost parent is the Element you are inside whilst parsing.
 *
 * i.e. if El3 is first child of El2, and El2 is first child of El1, and you are currently parsing El3
 * then Parents will look like
 * ```text
 * [1, 2, 3] // where an empty parents Vec means you are in Root
 * ```
 * So when you finish parsing 3, you will pop 3 off, and be left inside 2
 * ```text
 * [1, 2]
 * ```
 * etc
 */
pub mod indent;
pub mod outdent;
use crate::ast::elements::Element;
use crate::Ast;

/// Get the current parent as an Element
pub fn get_current_parent_element_from_parents(ast: &Ast) -> Element {
    let parent_ref = get_current_parent_ref_from_parents(ast);
    ast.elements[parent_ref].clone()
}

/// Get the current parent as an ElIndex
pub fn get_current_parent_ref_from_parents(ast: &Ast) -> usize {
    let last = ast.parents.len() - 1;
    ast.parents[last]
}

/// Option - Gets the parent Element of a child based on child's ElIndex
pub fn get_current_parent_element_from_element_children_search(
    ast: &Ast,
    child_ref: usize,
) -> Option<Element> {
    if let Some(index) = get_current_parent_ref_from_element_children_search(ast, child_ref) {
        return Some(ast.elements[index].clone());
    }
    None
}

/// Option - Gets the parent ElIndex of a child based on child's ElIndex
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

/// Helper for formatting code - spaces * number of indents for the current parent depth
pub fn get_indent(ast: &Ast) -> String {
    " ".repeat(4 * (ast.parents.len() - 1))
}

/// Helper - to remove first item from a vec
pub fn vec_remove_head(stack: &Vec<usize>) -> Vec<usize> {
    if stack.len() == 1 {
        vec![]
    } else {
        stack[1..].to_vec()
    }
}

/// Helper - to remove last item from a vec
pub fn vec_remove_tail(stack: &Vec<usize>) -> Vec<usize> {
    if stack.len() == 1 {
        vec![]
    } else {
        stack[..stack.len() - 1].to_vec()
    }
}
