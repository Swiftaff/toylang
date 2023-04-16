use crate::ast::elements;
use crate::ast::elements::{Element, ElementInfo};
use crate::ast::parents;
use crate::parse;
use crate::Compiler;

pub fn outdent(compiler: &mut Compiler) {
    compiler.log(format!("outdent::outdent {:?}", ""));
    compiler.ast.parents = if compiler.ast.parents.len() < 2 {
        vec![0]
    } else {
        parents::vec_remove_tail(&compiler.ast.parents)
    };
}

pub fn within_fndef_from_return_expression(compiler: &mut Compiler) {
    compiler.log(format!(
        "outdent::within_fndef_from_return_expression {:?}",
        ""
    ));

    // really not sure when this version of previous_element would have ever worked
    // in conjunction with elements::get_last_element(&mut compiler.ast) too
    // it's not the last items in the ast it needs the second last and the last child
    // of the function instead based on the current parent pointing to the function
    // ...
    //let previous_element = compiler.ast.elements[compiler.ast.elements.len() - 2].clone();

    let this_fn_ref = compiler.ast.parents[compiler.ast.parents.len() - 1];
    let this_fn_children = compiler.ast.elements[this_fn_ref].1.clone();
    if this_fn_children.len() < 2 {
        dbg!("error in within_fndef_from_return_expression");
        return ();
    }
    let last_child_ref = this_fn_children[this_fn_children.len() - 1];
    // (should be safe to subtract 2 since there should always be a root)
    let second_last_child_ref = this_fn_children[this_fn_children.len() - 2];

    let last_child = compiler.ast.elements[last_child_ref].clone();
    let second_last_child = compiler.ast.elements[second_last_child_ref].clone();

    // outdent if a return expression i.e.
    // if previous element is an indent
    // then the last element on that row is the next element after the indent
    // so it can be checked for being a return expression
    match second_last_child.0 {
        ElementInfo::Indent => {
            // outdent if it is a return expression
            // based on these valid examples of return expression
            match last_child.0 {
                ElementInfo::List(_) => {
                    //dbg!("FunctionDef outdent List", &self.ast.parents,);
                    outdent(compiler);
                    outdent(compiler);
                }
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
                ElementInfo::Bool(_) => {
                    //dbg!("FunctionDef outdent Bool", &self.ast.parents,);
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
                ElementInfo::If(_) => {
                    //dbg!("If outdent ConstantRef", &self.ast.parents,);
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
    compiler.log(format!(
        "outdent::within_fndef_for_inbuiltfncall_from_inbuiltfndef {:?}",
        fndefref
    ));
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

//TODO tidy these all up - define names better and find best place to check all args match here for parser error
//if el_children.len() == 0 || argtypes.len() > el_children.len() {
//    append::append_error(compiler, 0, 1, ERRORS.fncall_wrong_number_of_args);
//} else {
//    for argtype in argtypes {
//        //
//    }
//}

pub fn inbuiltfncall_from_inbuiltfndef(
    compiler: &mut Compiler,
    current_parent: Element,
    fndefref: usize,
) {
    compiler.log(format!(
        "outdent::within_fndef_for_inbuiltfncall_from_inbuiltfndef {:?} {:?}",
        current_parent, fndefref
    ));
    //dbg!("InbuiltFunctionCall", &fndefref);
    let fndef = compiler.ast.elements[fndefref].clone();
    match fndef.0 {
        ElementInfo::InbuiltFunctionDef(_, argnames, _, _, _) => {
            // current assumption is inbuiltfunctionCalls expect a fixed number
            // of children to match args.
            //dbg!("InbuiltFunctionCall", &current_parent.1, &argnames);
            if current_parent.1.len() == argnames.len() {
                outdent(compiler);
            }
        }
        _ => (),
    }
}

pub fn within_fndef_for_fncall_from_fndef(compiler: &mut Compiler, name: &String) {
    compiler.log(format!(
        "outdent::within_fndef_for_fncall_from_fndef {:?}",
        name
    ));
    if let Some(index) = elements::get_function_index_by_name(&mut compiler.ast, name) {
        let fndef = &compiler.ast.elements[index];
        match &fndef.0 {
            ElementInfo::FunctionDef(_, argnames, _, _) => {
                // current assumption is functionCalls expect a fixed number
                // of children to match args
                //dbg!("here");
                if fndef.1.len() == argnames.len() {
                    outdent(compiler);
                }
                outdent(compiler);
                //outdent(compiler);
            }
            _ => (),
        }
    }
}

pub fn fncall(compiler: &mut Compiler, current_parent: Element, name: String) {
    compiler.log(format!("outdent::fncall {:?} {:?}", current_parent, name));
    match current_parent.0 {
        ElementInfo::Println => println(compiler, current_parent),
        _ => {
            if let Some(index) = elements::get_function_index_by_name(&mut compiler.ast, &name) {
                let fndef = &compiler.ast.elements[index];
                match &fndef.0 {
                    ElementInfo::FunctionDef(_, argnames, _, _) => {
                        let args = argnames.clone().len();
                        functioncall_of_functiondef(compiler, current_parent.1.len(), args);
                    }
                    ElementInfo::Arg(_, _, returntype) => {
                        let r = returntype.clone();
                        functioncall_of_arg(compiler, &r, current_parent.1.len());
                    }
                    _ => (),
                }
            };
        }
    }
}

//pub fn list(compiler: &mut Compiler, current_parent: Element) {
//    //dbg!("outdent.list");
//    if current_parent.1.len() > 0 {
//        outdent(compiler);
//    }
//}

pub fn println(compiler: &mut Compiler, current_parent: Element) {
    compiler.log(format!("outdent::println {:?}", current_parent));
    //dbg!("outdent.println");
    if current_parent.1.len() > 0 {
        outdent(compiler);
    }
}

pub fn constant(compiler: &mut Compiler, current_parent: Element) {
    compiler.log(format!("outdent::constant {:?}", current_parent));
    //dbg!("Constant");
    if current_parent.1.len() > 0 {
        outdent(compiler);
    }
}

pub fn assignment(compiler: &mut Compiler, current_parent: Element) {
    compiler.log(format!("outdent::assignment {:?}", current_parent));
    //dbg!("Assignment");
    if current_parent.1.len() > 0 {
        outdent(compiler);
    }
}

pub fn if_expression(compiler: &mut Compiler, current_parent: Element) {
    compiler.log(format!("outdent::if_expression {:?}", current_parent));
    //dbg!("If");
    if current_parent.1.len() > 2 {
        outdent(compiler);
    }
}

pub fn functioncall_of_arg(compiler: &mut Compiler, returntype: &String, num_children: usize) {
    compiler.log(format!(
        "outdent::functioncall_of_arg {:?} {:?}",
        returntype, num_children
    ));
    let args = parse::get_args_from_dyn_fn(returntype);
    if num_children > 0 && num_children == args {
        outdent(compiler);
        //TODO figure out how to move 2nd outdent to more appropriate spot for test 'parse::test_pass_passing_func_as_args'
        outdent(compiler);
    }
}

pub fn functioncall_of_functiondef(compiler: &mut Compiler, num_children: usize, args: usize) {
    compiler.log(format!(
        "outdent::functioncall_of_functiondef {:?} {:?}",
        num_children, args
    ));
    if num_children > 0 && num_children == args {
        outdent(compiler);
    }
}
