use crate::Ast;

pub fn indent(ast: &mut Ast) {
    ast.parents.push(ast.elements.len() - 1);
}

pub fn indent_this(ast: &mut Ast, index: usize) {
    ast.parents.push(index);
}
