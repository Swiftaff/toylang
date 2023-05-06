/*! Minor module to indent the parser at a code block
 */
use crate::Ast;

/// Indent by adding current element to parent stack
pub fn indent(ast: &mut Ast) {
    ast.parents.push(ast.elements.len() - 1);
}

/// Indent by adding current index to parent stack
pub fn indent_this(ast: &mut Ast, index: usize) {
    ast.parents.push(index);
}
