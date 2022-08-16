use crate::ast::elements;
use crate::ast::elements::{Element, ElementInfo};
use crate::ast::parents;
use crate::parse;
use crate::Compiler;

pub fn outdent(compiler: &mut Compiler) {
    compiler.ast.parents = if compiler.ast.parents.len() < 2 {
        vec![0]
    } else {
        parents::vec_remove_tail(&compiler.ast.parents)
    };
}

pub fn within_fndef_from_return_expression(compiler: &mut Compiler) {
    //dbg!("FunctionDef");
    let previous_element = compiler.ast.elements[compiler.ast.elements.len() - 2].clone();
    // (should be safe to subtract 2 since there should always be a root)

    // outdent if a return expression i.e.
    // if previous element is an indent
    // then the last element on that row is the next element after the indent
    // so it can be checked for being a return expression
    match previous_element.0 {
        ElementInfo::Indent => {
            // outdent if it is a return expression
            // based on these valid examples of return expression
            match elements::get_last_element(&mut compiler.ast).0 {
                ElementInfo::Int(_) => {
                    //dbg!("FunctionDef outdent Int", &self.ast.parents,);
                    outdent(compiler);
                    outdent(compiler);
                }
                ElementInfo::Float(_) => {
                    //dbg!("FunctionDef outdent Float", &self.ast.parents,);
                    outdent(compiler);
                    outdent(compiler);
                }
                ElementInfo::String(_) => {
                    //dbg!("FunctionDef outdent String", &self.ast.parents,);
                    outdent(compiler);
                    outdent(compiler);
                }
                ElementInfo::Arg(_, _, _) => {
                    //TODO
                }
                ElementInfo::Constant(_, _) => {
                    //dbg!("FunctionDef outdent Constant", &self.ast.parents,);
                    outdent(compiler);
                    outdent(compiler);
                }
                ElementInfo::ConstantRef(_, _, _) => {
                    //dbg!("FunctionDef outdent ConstantRef", &self.ast.parents,);
                    outdent(compiler);
                    outdent(compiler);
                }
                ElementInfo::InbuiltFunctionCall(_, fndefref, _) => {
                    within_fndef_for_inbuiltfncall_from_inbuiltfndef(compiler, fndefref);
                }
                ElementInfo::FunctionCall(name, _) => {
                    within_fndef_for_fncall_from_fndef(compiler, &name);
                }
                ElementInfo::Parens => {
                    //TODO for a function ref?
                }
                // non-return expresions
                // explicitly listing other types rather than using _ to not overlook new types in future
                ElementInfo::Root => (),
                ElementInfo::CommentSingleLine(_) => (),
                ElementInfo::Assignment => (),
                ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => (),
                ElementInfo::FunctionDefWIP => (),
                ElementInfo::FunctionDef(_, _, _, _) => (),
                ElementInfo::Type(_) => (),
                ElementInfo::Eol => (),
                ElementInfo::Seol => (),
                ElementInfo::Indent => (),
                ElementInfo::Unused => (),
                ElementInfo::LoopForRangeWIP => (),
                ElementInfo::LoopForRange(_, _, _) => (),
                ElementInfo::Println => (),
            }
        }
        _ => (),
    }
}

pub fn within_fndef_for_inbuiltfncall_from_inbuiltfndef(compiler: &mut Compiler, fndefref: usize) {
    let fndef = &compiler.ast.elements[fndefref];
    match &fndef.0 {
        ElementInfo::InbuiltFunctionDef(_, argnames, _, _, _) => {
            // current assumption is inbuiltFunctionCalls expect a fixed number
            // of children to match args
            if fndef.1.len() == argnames.len() {
                outdent(compiler);
            }
            outdent(compiler);
            outdent(compiler);
        }
        _ => (),
    }
}

pub fn inbuiltfncall_from_inbuiltfndef(
    compiler: &mut Compiler,
    current_parent: Element,
    fndefref: usize,
) {
    //dbg!("InbuiltFunctionCall", &name);
    let fndef = compiler.ast.elements[fndefref].clone();
    match fndef.0 {
        ElementInfo::InbuiltFunctionDef(_, argnames, _, _, _) => {
            // current assumption is inbuiltfunctionCalls expect a fixed number
            // of children to match args.
            if current_parent.1.len() == argnames.len() {
                outdent(compiler);
            }
        }
        _ => (),
    }
}

pub fn within_fndef_for_fncall_from_fndef(compiler: &mut Compiler, name: &String) {
    if let Some(index) = elements::get_function_index_by_name(&mut compiler.ast, name) {
        let fndef = &compiler.ast.elements[index];
        match &fndef.0 {
            ElementInfo::FunctionDef(_, argnames, _, _) => {
                // current assumption is functionCalls expect a fixed number
                // of children to match args
                if fndef.1.len() == argnames.len() {
                    outdent(compiler);
                }
                outdent(compiler);
                outdent(compiler);
            }
            _ => (),
        }
    }
}

pub fn fncall(compiler: &mut Compiler, current_parent: Element, name: String) {
    match current_parent.0 {
        ElementInfo::Println => println(compiler, current_parent),
        _ => {
            if let Some(index) = elements::get_function_index_by_name(&mut compiler.ast, &name) {
                let fndef = &compiler.ast.elements[index];
                match &fndef.0 {
                    ElementInfo::FunctionDef(_, argnames, _, _) => {
                        let args = argnames.clone().len();
                        functiondef(compiler, current_parent.1.len(), args);
                    }
                    ElementInfo::Arg(_, _, returntype) => {
                        let r = returntype.clone();
                        arg(compiler, &r, current_parent.1.len());
                    }
                    _ => (),
                }
            };
        }
    }
}

pub fn println(compiler: &mut Compiler, current_parent: Element) {
    //dbg!("outdent.println");
    if current_parent.1.len() > 0 {
        outdent(compiler);
    }
}

pub fn constant(compiler: &mut Compiler, current_parent: Element) {
    //dbg!("Constant");
    if current_parent.1.len() > 0 {
        outdent(compiler);
    }
}

pub fn assignment(compiler: &mut Compiler, current_parent: Element) {
    //dbg!("Assignment");
    if current_parent.1.len() > 0 {
        outdent(compiler);
    }
}

pub fn arg(compiler: &mut Compiler, returntype: &String, num_children: usize) {
    let args = parse::get_args_from_dyn_fn(returntype);
    if num_children > 0 && num_children == args {
        outdent(compiler);
        outdent(compiler);
    }
}

pub fn functiondef(compiler: &mut Compiler, num_children: usize, args: usize) {
    if num_children > 0 && num_children == args {
        outdent(compiler);
        outdent(compiler);
    }
}
