/*! Elements are the individual nodes in the AST.
 *
 * Each is made of an ElementInfo like Int(Value), and a Vec of indexes, which are references to child Elements elsewhere in the AST.
 */

pub mod append;
use crate::ast::elements;
use crate::ast::parents;
use crate::formatting;
use crate::Ast;
use crate::Compiler;
use std::fmt;

#[derive(Clone)]

pub enum ElementInfo {
    List(ReturnType),                          //children = list items of same type
    CommentSingleLine(Value),                  //no children
    Int(Value),                                //no children
    Float(Value),                              //no children
    String(Value),                             //no children
    Bool(Value),                               //no children
    Arg(Name, Scope, ArgModifier, ReturnType), //no children
    Type(Name),                                //no children
    Eol,                                       //no children
    Seol,                                      //no children
    Indent,                                    //no children
    Unused,                                    //no children
    ConstantRef(Name, ReturnType, RefName),    //no children
    Struct(Name, ArgNames, Vec<ReturnType>), //children = each key value (Assignment > Constant > Value)
    StructEdit(Name, ArgNames),              //1 child, value
    Constant(Name, ReturnType),              //1 child, value
    Assignment,                              //1 child, constant
    InbuiltFunctionDef(Name, ArgNames, ArgTypes, ArgModifiers, ReturnType, Format), //children = lines of function contents
    InbuiltFunctionCall(Name, ElIndex, ReturnType), //fndef argnames.len() children
    FunctionDefWIP,                                 //children = lines of function contents
    FunctionDef(Name, ArgNames, ArgTypes, ReturnType), //children = lines of function contents
    FunctionCall(Name, ReturnType),                 //fndef argnames.len() children
    Parens,          //either 1 child, for function_ref, or 1+ for function type sig
    LoopForRangeWIP, //children = lines of loop contents
    LoopForRange(Name, From, To), //children = lines of loop contents
    Println,         //1 child, value
    If(ReturnType),  //3 children, boolean_expression true_return_expression false_return_expression

    Root, //children = lines of function contents
}

/// Fake function #1, only useful for copy/pasting all the elementinfo types!
fn _cut_and_paste_element_infos(el: ElementInfo) -> bool {
    let replaceme = true;
    match el {
        ElementInfo::Root => replaceme,
        // explicitly listing other types rather than using _ to not overlook new types in future.
        ElementInfo::List(_) => replaceme,
        ElementInfo::CommentSingleLine(_) => replaceme,
        ElementInfo::Int(_) => replaceme,
        ElementInfo::Float(_) => replaceme,
        ElementInfo::String(_) => replaceme,
        ElementInfo::Bool(_) => replaceme,
        ElementInfo::Arg(_, _, _, _) => replaceme,
        ElementInfo::Struct(_, _, _) => replaceme,
        ElementInfo::StructEdit(_, _) => replaceme,
        ElementInfo::Constant(_, _) => replaceme,
        ElementInfo::ConstantRef(_, _, _) => replaceme,
        ElementInfo::Assignment => replaceme,
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _, _) => replaceme,
        ElementInfo::InbuiltFunctionCall(_, _, _) => replaceme,
        ElementInfo::FunctionDefWIP => replaceme,
        ElementInfo::FunctionDef(_, _, _, _) => replaceme,
        ElementInfo::FunctionCall(_, _) => replaceme,
        ElementInfo::Parens => replaceme,
        ElementInfo::Type(_) => replaceme,

        ElementInfo::Eol => replaceme,
        ElementInfo::Seol => replaceme,
        ElementInfo::Indent => replaceme,
        ElementInfo::Unused => replaceme,
        ElementInfo::Println => replaceme,
        ElementInfo::If(_) => replaceme,
        ElementInfo::LoopForRangeWIP => replaceme,
        ElementInfo::LoopForRange(_, _, _) => replaceme,
    }
}

/// Fake function #2, only useful for copy/pasting all the option_element types!
fn _cut_and_paste_elements(el_option: Option<Element>) -> bool {
    let replaceme = true;
    match el_option {
        Some((ElementInfo::Root, _)) => replaceme,
        // explicitly listing other types rather than using _ to not overlook new types in future.
        Some((ElementInfo::List(_), _)) => replaceme,
        Some((ElementInfo::CommentSingleLine(_), _)) => replaceme,
        Some((ElementInfo::Int(_), _)) => replaceme,
        Some((ElementInfo::Float(_), _)) => replaceme,
        Some((ElementInfo::String(_), _)) => replaceme,
        Some((ElementInfo::Bool(_), _)) => replaceme,
        Some((ElementInfo::Arg(_, _, _, _), _)) => replaceme,
        Some((ElementInfo::Struct(_, _, _), _)) => replaceme,
        Some((ElementInfo::StructEdit(_, _), _)) => replaceme,
        Some((ElementInfo::Constant(_, _), _)) => replaceme,
        Some((ElementInfo::ConstantRef(_, _, _), _)) => replaceme,
        Some((ElementInfo::Assignment, _)) => replaceme,
        Some((ElementInfo::InbuiltFunctionDef(_, _, _, _, _, _), _)) => replaceme,
        Some((ElementInfo::InbuiltFunctionCall(_, _, _), _)) => replaceme,
        Some((ElementInfo::FunctionDefWIP, _)) => replaceme,
        Some((ElementInfo::FunctionDef(_, _, _, _), _)) => replaceme,
        Some((ElementInfo::FunctionCall(_, _), _)) => replaceme,
        Some((ElementInfo::Parens, _)) => replaceme,
        Some((ElementInfo::Type(_), _)) => replaceme,
        Some((ElementInfo::Eol, _)) => replaceme,
        Some((ElementInfo::Seol, _)) => replaceme,
        Some((ElementInfo::Indent, _)) => replaceme,
        Some((ElementInfo::Unused, _)) => replaceme,
        Some((ElementInfo::Println, _)) => replaceme,
        Some((ElementInfo::If(_), _)) => replaceme,
        Some((ElementInfo::LoopForRangeWIP, _)) => replaceme,
        Some((ElementInfo::LoopForRange(_, _, _), _)) => replaceme,
        None => replaceme,
    }
}

pub type ElIndex = usize;
pub type Elements = Vec<Element>;
pub type Element = (ElementInfo, ElementChildren);
pub type ElementChildren = Vec<ElIndex>;

type Value = String;
type From = usize;
type To = usize;
type ReturnType = String;
type Name = String;
type RefName = String;
type ArgNames = Vec<String>;
type ArgTypes = Vec<String>;
//type ArgModifier = String;
type ArgModifiers = Vec<ArgModifier>;
type Format = String;
type Scope = ElIndex;

#[derive(Clone, Debug)]
pub enum ArgModifier {
    Arg(String),
    FnArg(Vec<String>),
    None,
}

/// Finds the original element referred to, e.g. when using a variable name
pub fn get_element_by_name(ast: &Ast, name: &String) -> Option<Element> {
    if let Some(index) = get_constant_index_by_name(ast, name) {
        return Some(ast.elements[index].clone());
    }
    if let Some(index) = get_inbuilt_function_index_by_name(ast, name) {
        return Some(ast.elements[index].clone());
    }
    if let Some(index) = get_function_index_by_name(ast, name) {
        return Some(ast.elements[index].clone());
    }
    if let Some(index) = get_inbuilt_type_index_by_name(ast, name) {
        return Some(ast.elements[index].clone());
    }
    if let Some(index) = get_arg_index_by_name(ast, name) {
        return Some(ast.elements[index].clone());
    }
    if let Some(index) = get_struct_index_by_name(ast, name) {
        return Some(ast.elements[index].clone());
    }
    None
}

/// Get the index of the Struct based on its name
pub fn get_struct_index_by_name(ast: &Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match elinfo {
        ElementInfo::Struct(n, _, _) => n == &append::upper_first_char(name),
        _ => false,
    })
}

/// Get the index of the Arg based on its name
pub fn get_arg_index_by_name(ast: &Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match elinfo {
        ElementInfo::Arg(n, _, _, _) => n == name,
        _ => false,
    })
}

/// Get the index of the Type based on its name
pub fn get_inbuilt_type_index_by_name(ast: &Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match elinfo {
        ElementInfo::Type(n) => n == name,
        _ => false,
    })
}

/// Get the index of the Constant based on its name
pub fn get_constant_index_by_name(ast: &Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match elinfo {
        ElementInfo::Constant(n, _t) => n == name,
        ElementInfo::ConstantRef(n, _t, _refname) => n == name,
        _ => false,
    })
}

/// Get the Constant based on its name
pub fn get_constant_by_name(ast: &Ast, name: &String) -> Option<ElementInfo> {
    if let Some(index) = get_constant_index_by_name(ast, name) {
        return Some(ast.elements[index].0.clone());
    }
    None
}

/// Get the index of the Function based on its name
pub fn get_function_index_by_name(ast: &Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match &elinfo {
        ElementInfo::FunctionDef(n, _, _, _) => n == name,
        ElementInfo::Arg(n, _, _, r) => n == name && r.contains("&dyn Fn"),
        _ => false,
    })
}
/// Get the index of the InbuiltFn based on its name
pub fn get_inbuilt_function_index_by_name(ast: &Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match &elinfo {
        ElementInfo::InbuiltFunctionDef(n, _, _, _, _, _) => n == name,
        ElementInfo::Arg(n, _, _, r) => n == name && r.contains("&dyn Fn"),
        _ => false,
    })
}

/*
pub fn _get_inbuilt_function_index_by_name_and_returntype(
    ast: &Ast,
    name: &String,
    returntype: &String,
) -> Option<usize> {
    //dbg!(returntype);
    ast.elements.iter().position(|(elinfo, _)| match &elinfo {
        ElementInfo::InbuiltFunctionDef(n, _, _, _, r, _) => {
            //dbg!("here", n, r, name, returntype);
            n == name && (r.contains(returntype) || returntype.contains(r))
        }
        ElementInfo::Arg(n, _, _, r) => {
            n == name && r.contains("&dyn Fn") && r.contains(returntype)
        }
        _ => false,
    })
}
*/

/// Get the inbuiltFn by its name
pub fn get_inbuilt_function_by_name(ast: &Ast, name: &String) -> Option<ElementInfo> {
    if let Some(index) = get_inbuilt_function_index_by_name(ast, name) {
        return Some(ast.elements[index].0.clone());
    }
    None
}

/*
pub fn _get_inbuilt_function_by_name_and_returntype(
    ast: &Ast,
    name: &String,
    returntype: &String,
) -> Option<ElementInfo> {
    //dbg!(returntype);
    if let Some(index) = _get_inbuilt_function_index_by_name_and_returntype(ast, name, returntype) {
        return Some(ast.elements[index].0.clone());
    }
    None
}
*/

/// Get last element in ast
pub fn get_last_element(ast: &Ast) -> Element {
    ast.elements.last().unwrap().clone()
}

/// Get ElementInfo but with inferred type added
pub fn get_updated_elementinfo_with_infered_type(ast: &mut Ast, el_index: usize) -> ElementInfo {
    let el = ast.elements[el_index].clone();
    let el_type = get_elementinfo_type(ast, &el.0);
    if el_type.contains("Undefined") || el_type.contains("|") {
        let infered_type = get_infered_type_of_any_element(ast, el_index);
        match el.0 {
            ElementInfo::Arg(name, scope, argmodifier, _) => {
                return ElementInfo::Arg(name, scope, argmodifier, infered_type);
            }
            ElementInfo::Constant(name, _) => {
                return ElementInfo::Constant(name, infered_type);
            }
            ElementInfo::ConstantRef(name, _, refname) => {
                return ElementInfo::ConstantRef(name, infered_type, refname);
            }
            ElementInfo::Assignment => {
                return ElementInfo::Assignment;
            }
            ElementInfo::InbuiltFunctionCall(name, fndef_index, _) => {
                return ElementInfo::InbuiltFunctionCall(name, fndef_index, infered_type);
            }
            ElementInfo::FunctionCall(name, _) => {
                return ElementInfo::FunctionCall(name, infered_type);
            }
            ElementInfo::List(returntype) => {
                if el.1.len() == 0 {
                    return ElementInfo::List(returntype);
                } else {
                    let first_child_type = get_infered_type_of_any_element(&ast, el.1[0]);
                    return ElementInfo::List(format!("Vec<{}>", first_child_type));
                }
            }
            ElementInfo::If(_returntype) => {
                let mut second_child_type =
                    "error in get_updated_elementinfo_with_infered_type".to_string();
                if el.1.len() > 1 {
                    second_child_type = get_infered_type_of_any_element(&ast, el.1[1]);
                } else {
                    //dbg!(&el.1);
                }
                return ElementInfo::If(second_child_type);
            }
            // explicitly listing other types rather than using _ to not overlook new types in future.
            // These either have no type or are predefined and can't be infered
            ElementInfo::Root => (),
            ElementInfo::CommentSingleLine(_) => (),
            ElementInfo::Int(_) => (),
            ElementInfo::Float(_) => (),
            ElementInfo::String(_) => (),
            ElementInfo::Bool(_) => (),
            ElementInfo::Struct(_, _, _) => (),
            ElementInfo::StructEdit(_, _) => (),
            ElementInfo::InbuiltFunctionDef(_, _, _, _, _, _) => (),
            ElementInfo::FunctionDefWIP => (),
            ElementInfo::FunctionDef(_, _, _, _) => (),
            ElementInfo::Parens => (),
            ElementInfo::Type(_) => (),
            ElementInfo::Eol => (),
            ElementInfo::Seol => (),
            ElementInfo::Indent => (),
            ElementInfo::Unused => (),
            ElementInfo::LoopForRangeWIP => (),
            ElementInfo::LoopForRange(_, _, _) => (),
            ElementInfo::Println => (),
        };
    }
    el.0
}

/// Get the inferred type of an element where possible or ignore if not
pub fn get_infered_type_of_any_element(ast: &Ast, el_index: usize) -> String {
    let el = ast.elements[el_index].clone();
    let el_info = &el.0;
    match el_info {
        ElementInfo::Arg(_, _, _, _) => {
            return get_infered_type_of_arg_element(ast, el_info, el_index);
        }
        ElementInfo::Constant(_, _) => {
            return get_infered_type_of_constant_element(ast, &el);
        }
        ElementInfo::ConstantRef(_, _, refname) => {
            return get_infered_type_of_constantref_element(ast, &refname);
        }
        ElementInfo::InbuiltFunctionCall(_, fndef_index, _) => {
            return get_infered_type_of_inbuiltfunctioncall_element(ast, &el, *fndef_index);
        }
        ElementInfo::FunctionCall(name, _) => {
            return get_infered_type_of_functioncall_element(ast, &name);
        }
        ElementInfo::List(returntype) => {
            if el.1.len() > 0 {
                let first_child_ref = el.1[0];
                let first_child_type = get_infered_type_of_any_element(ast, first_child_ref);
                return format!("Vec<{}>", first_child_type);
            } else {
                return returntype.clone();
            }
        }
        ElementInfo::If(_) => {
            return get_infered_type_of_if_element(ast, el.1);
        }
        ElementInfo::Struct(name, _, _) => return name.clone(),
        // explicitly listing other types rather than using _ to not overlook new types in future
        ElementInfo::StructEdit(_, _) => (),
        ElementInfo::Root => (),
        ElementInfo::CommentSingleLine(_) => (),
        ElementInfo::Int(_) => (),
        ElementInfo::Float(_) => (),
        ElementInfo::String(_) => (),
        ElementInfo::Bool(_) => (),
        ElementInfo::Assignment => (),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _, _) => (),
        ElementInfo::FunctionDefWIP => (),
        ElementInfo::FunctionDef(_, _, _, _) => (),
        ElementInfo::Parens => (),
        ElementInfo::Type(_) => (),
        ElementInfo::Eol => (),
        ElementInfo::Seol => (),
        ElementInfo::Indent => (),
        ElementInfo::Unused => (),
        ElementInfo::LoopForRangeWIP => (),
        ElementInfo::LoopForRange(_, _, _) => (),
        ElementInfo::Println => (),
    }
    get_elementinfo_type(ast, el_info)
}

/// Get inferred type of Arg
pub fn get_infered_type_of_arg_element(
    ast: &Ast,
    el_info: &ElementInfo,
    el_index: usize,
) -> String {
    let mut infered_type = "Undefined".to_string();
    match el_info {
        ElementInfo::Arg(name, _, _, _) => {
            // get type of this arg, from the argtypes of parent funcdef
            if let Some(parent_funcdef) =
                parents::get_current_parent_element_from_element_children_search(ast, el_index)
            {
                match parent_funcdef.0 {
                    ElementInfo::FunctionDef(_, argnames, argtypes, _) => {
                        if let Some(index) = argnames.iter().position(|argname| argname == name) {
                            if argtypes.len() > index {
                                infered_type = argtypes[index].clone();
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }

    infered_type
}

/// Get inferred type of Constant
pub fn get_infered_type_of_constant_element(ast: &Ast, el: &Element) -> String {
    let mut infered_type = "Undefined".to_string();
    match el.0 {
        ElementInfo::Constant(_, _) => {
            if el.1.len() > 0 {
                let mut child_ref = el.1[0];
                //if indent, skip it and use the next element
                match &ast.elements[child_ref].0 {
                    ElementInfo::Indent => {
                        if el.1.len() > 1 {
                            child_ref = el.1[1];
                        }
                    }
                    _ => (),
                }
                infered_type = get_infered_type_of_any_element(ast, child_ref);
            }
        }
        _ => (),
    }
    infered_type
}

/// Get inferred type of Constantref
pub fn get_infered_type_of_constantref_element(ast: &Ast, refname: &String) -> String {
    let mut infered_type = "Undefined".to_string();
    if let Some(ElementInfo::Constant(_, returntype)) = get_constant_by_name(ast, &refname) {
        infered_type = returntype
    }
    infered_type
}

/// Get inferred type of InbuiltFnCall
pub fn get_infered_type_of_inbuiltfunctioncall_element(
    ast: &Ast,
    func_call_el: &Element,
    funcdef_el_index: usize,
) -> String {
    let mut infered_type = "Undefined".to_string();
    let el_children = func_call_el.1.clone();
    let el = &ast.elements[funcdef_el_index];
    let elinfo = &el.0;
    match elinfo {
        ElementInfo::InbuiltFunctionDef(_, _argnames, argtypes, _argmodifiers, returntype, _) => {
            //dbg!(funcdef_el_index, &returntype);
            if returntype.contains("|") {
                //dbg!("2.5", &el_children, &argtypes, &returntype);

                // get infered_type from first child (note: children may contain more items than args like SEOL, thus <=)
                if el_children.len() > 0 && argtypes.len() <= el_children.len() {
                    let first_child_ref = el_children[0];
                    let first_child = &ast.elements[first_child_ref];
                    let type_of_first_child = get_elementinfo_type(ast, &first_child.0);
                    //dbg!(&type_of_first_child, first_child_ref);
                    let argtypes_split = argtypes[0].split("|");
                    let argtypes_vec: Vec<&str> = argtypes_split.collect();
                    let argtype_index_option = argtypes_vec
                        .iter()
                        .position(|&x| x.to_string() == type_of_first_child);
                    match argtype_index_option {
                        Some(argtype_index) => {
                            let returntypes_split = returntype.split("|");
                            let returntypes_vec: Vec<&str> = returntypes_split.collect();
                            infered_type = returntypes_vec[argtype_index].to_string();
                        }
                        _ => {
                            infered_type = returntype.clone();
                        }
                    }
                }
            } else {
                infered_type = returntype.clone();
            }
        }
        _ => (),
    }
    infered_type
}

/// Get inferred type of FnCall
pub fn get_infered_type_of_functioncall_element(ast: &Ast, name: &String) -> String {
    let undefined = "Undefined".to_string();
    if let Some(index) = get_function_index_by_name(ast, &name) {
        let funcdef = &ast.elements[index];
        match &funcdef.0 {
            ElementInfo::FunctionDef(_, _, _, returntype) => return returntype.clone(),
            ElementInfo::Arg(_, _, _, returntype) => {
                if returntype.contains("&dyn Fn") {
                    return returntype.clone();
                } else {
                    return undefined;
                }
            }
            _ => return undefined,
        }
    }
    undefined
}

/// Get inferred type of If
pub fn get_infered_type_of_if_element(ast: &Ast, children: Vec<usize>) -> String {
    if children.len() > 1 {
        let second_child = &ast.elements[children[1]];
        get_elementinfo_type(ast, &second_child.0)
    } else {
        //dbg!("error in get_infered_type_of_if_element");
        "error".to_string()
    }
}

/// Get type of an ElementInfo
pub fn get_elementinfo_type(ast: &Ast, elementinfo: &ElementInfo) -> String {
    let undefined = "Undefined".to_string();
    let none = "None".to_string();
    match elementinfo {
        ElementInfo::List(returntype) => returntype.clone(),
        ElementInfo::Int(_) => "i64".to_string(),
        ElementInfo::Float(_) => "f64".to_string(),
        ElementInfo::String(_) => "String".to_string(),
        ElementInfo::Bool(_) => "bool".to_string(),
        ElementInfo::Assignment => none,
        ElementInfo::Struct(name, _, _) => name.clone(),
        ElementInfo::Constant(_, returntype) => returntype.clone(),
        ElementInfo::ConstantRef(_, returntype, _) => returntype.clone(),
        ElementInfo::InbuiltFunctionCall(_, _fndef_index, returntype) => returntype.clone(),
        ElementInfo::Arg(_, _, _, returntype) => returntype.clone(),
        ElementInfo::FunctionCall(name, _) => get_infered_type_of_functioncall_element(ast, &name),
        ElementInfo::Type(returntype) => returntype.clone(),
        ElementInfo::If(returntype) => returntype.clone(),
        // explicitly listing other types rather than using _ to not overlook new types in future
        ElementInfo::Root => none,
        ElementInfo::StructEdit(_, _) => none,
        ElementInfo::CommentSingleLine(_) => none,
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _, _) => undefined, // don't want to 'find' definitions
        ElementInfo::FunctionDefWIP => none,
        ElementInfo::FunctionDef(_, _, _, _) => undefined, // don't want to 'find' definitions
        ElementInfo::Parens => none,
        ElementInfo::Eol => none,
        ElementInfo::Seol => none,
        ElementInfo::Indent => none,
        ElementInfo::Unused => none,
        ElementInfo::LoopForRangeWIP => none,
        ElementInfo::LoopForRange(_, _, _) => none,
        ElementInfo::Println => none,
    }
}

/*
pub fn _is_existing_constant(compiler: &mut Compiler) -> bool {
    compiler.ast.log(format!("elements::is_existing_constant {:?}", ""));
    let parent = parents::get_current_parent_element_from_parents(&compiler.ast);
    let parent_assignment_has_no_children = false;
    if compiler.logs.len() == 150 {
        //dbg!(compiler.logs.len(), &parent, &compiler.ast.parents);
    }
*/
/*
//The below may be nonsense - it is not checking if there is a pre-existing constant of the same name!
Instead replacing it with a function which will check all parents for containing "Arg"s like "Arg: n scope:24 (i64) [ ]"

match parent.0 {
    ElementInfo::Assignment => {
        parent_assignment_has_no_children = parent.1.len() == 0;
        // then this constant is the first child of the assignment
        // so it is the name of the constant (and not the value if it were the second child),
        // and since constants are immutable it can't have the same name as a pre-existing constant
        // so it is invalid!
    }
    _ => (),
}
*/
/*
    parent_assignment_has_no_children
}
*/

/// Replace a child ref of an element with a ref to another child
pub fn replace_element_child(ast: &mut Ast, element_ref: usize, from: usize, to: usize) {
    let closure = |el_ref: usize| {
        if el_ref == from {
            to
        } else {
            el_ref
        }
    };
    let children: Vec<usize> = ast.elements[element_ref]
        .1
        .clone()
        .into_iter()
        .map(closure)
        .collect();
    ast.elements[element_ref].1 = children;
}

/// Replace the WIP FuncDef placeholder, with the final FuncDef
pub fn replace_funcdefwip_with_funcdef(
    compiler: &mut Compiler,
    children: &[usize],
    name: &String,
    func_def_ref: usize,
) {
    compiler.ast.log(format!(
        "elements::replace_funcdefwip_with_funcdef {:?} {:?} {:?}",
        children, name, func_def_ref
    ));
    //assign name, argtypes, argnames, returntype to parent funcdef
    let argtypes = get_argtypes_from_argtokens(compiler, &children);
    let returntype = get_returntype_from_argtokens(compiler, &children);
    let argnames = get_argnames_from_argtokens(compiler, &children, &argtypes);
    let new_funcdef = ElementInfo::FunctionDef(name.clone(), argnames, argtypes, returntype);

    // replace original funcdefWIP with funcdef
    compiler.ast.elements[func_def_ref] = (new_funcdef, vec![]);
}

/// Get a vec of types based on child refs, assuming they are Types or Parens (containing a Dyn Fn with types)
pub fn get_argtypes_from_argtokens(compiler: &mut Compiler, children: &[usize]) -> Vec<String> {
    compiler.ast.log(format!(
        "elements::get_argtypes_from_argtokens {:?}",
        children
    ));
    let mut argtypes: Vec<String> = vec![];
    let num_args = children.len() / 2;
    let argtype_refs = &children[..num_args];
    for a in argtype_refs {
        match &compiler.clone().ast.elements[a.clone()] {
            (ElementInfo::Type(typename), _) => argtypes.push(typename.clone()),
            (ElementInfo::Parens, paren_children) => {
                if paren_children.len() > 0 {
                    let fn_type_signature =
                        get_formatted_dyn_fn_type_sig(compiler, &paren_children);
                    argtypes.push(fn_type_signature)
                }
            }
            _ => (),
        }
    }
    argtypes
}

/// Get returntype from children assuming one is a Type
pub fn get_returntype_from_argtokens(compiler: &mut Compiler, children: &[usize]) -> String {
    compiler.ast.log(format!(
        "elements::get_returntype_from_argtokens {:?}",
        children
    ));
    let num_args = children.len() / 2;
    let returntype_ref = &children[num_args];
    return match &compiler.ast.elements[returntype_ref.clone()] {
        (ElementInfo::Type(typename), _) => typename.clone(),
        _ => "Undefined".to_string(),
    };
}

/// Get argnames from Arg tokens, but also update Arg tokens returntypes at same time
pub fn get_argnames_from_argtokens(
    compiler: &mut Compiler,
    children: &[usize],
    argtypes: &Vec<String>,
) -> Vec<String> {
    compiler.ast.log(format!(
        "elements::get_argnames_from_argtokens {:?} {:?}",
        children, argtypes
    ));

    //TODO make up mind about just using the Arg tokens as the definition of argnames/argtypes
    let mut argnames: Vec<String> = vec![];
    let num_args = children.len() / 2;
    let argname_refs = &children[num_args + 1..];
    for i in 0..argname_refs.len() {
        let a = argname_refs[i];
        match &compiler.ast.elements[a] {
            (ElementInfo::Arg(argname, scope, argmodifier, _), _) => {
                argnames.push(argname.clone());
                let returntype = argtypes[i].clone();
                let updated_arg_token = ElementInfo::Arg(
                    argname.clone(),
                    scope.clone(),
                    argmodifier.clone(),
                    returntype,
                );
                compiler.ast.elements[a].0 = updated_arg_token;
            }
            _ => (),
        }
    }
    argnames
}

/// Get Type sig of Dyn Fn
pub fn get_formatted_dyn_fn_type_sig(
    compiler: &mut Compiler,
    paren_children: &Vec<usize>,
) -> String {
    compiler.ast.log(format!(
        "elements::get_formatted_dyn_fn_type_sig {:?}",
        paren_children
    ));
    let paren_returntype_ref = *paren_children.last().unwrap();
    let paren_returntype_el = &compiler.ast.elements[paren_returntype_ref];
    let paren_returntype = elements::get_elementinfo_type(&compiler.ast, &paren_returntype_el.0);
    let paren_main_types = &paren_children[0..paren_children.len() - 1];
    let mut main_types = "".to_string();
    for i in 0..paren_main_types.len() {
        let main_type_ref = paren_main_types[i];
        let main_type_el = &compiler.ast.elements[main_type_ref];
        //dbg!(&main_type_el);
        let main_type = elements::get_elementinfo_type(&compiler.ast, &main_type_el.0);
        let comma = if i + 1 == paren_main_types.len() {
            "".to_string()
        } else {
            ", ".to_string()
        };
        main_types = format!("{}{}{}", main_types, comma, main_type);
    }
    format!("&dyn Fn({}) -> {}", main_types, paren_returntype)
}

impl fmt::Debug for ElementInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let el_debug = match self {
            ElementInfo::Root => format!("Root"),
            ElementInfo::List(returntype) => format!("List ({})", returntype),
            ElementInfo::CommentSingleLine(comment) => {
                format!("Comment: {}", comment)
            }
            ElementInfo::Int(int) => format!("Int: {}", int),
            ElementInfo::Float(float) => format!("Float: {}", float),
            ElementInfo::String(string) => format!("String: {}", string),
            ElementInfo::Bool(boolean) => format!("Bool: {}", boolean),
            ElementInfo::Arg(name, scope, argmodifier, returntype) => {
                format!(
                    "Arg: {} scope:{} argmodifier:({:?}) ({})",
                    name, scope, argmodifier, returntype
                )
            }
            ElementInfo::Struct(name, keys, keytypes) => {
                format!("Struct: {} keys: {:?} keytypes: {:?}", name, keys, keytypes)
            }
            ElementInfo::StructEdit(name, keys) => {
                format!("StructEdit: {} keys: {:?}", name, keys)
            }
            ElementInfo::Constant(name, returntype) => {
                format!("Constant: {} ({})", name, returntype)
            }
            ElementInfo::ConstantRef(name, returntype, refname) => {
                format!("ConstantRef: {} ({}) for \"{}\"", name, returntype, refname)
            }
            ElementInfo::Assignment => {
                format!("Assignment")
            }
            ElementInfo::InbuiltFunctionDef(
                name,
                _argnames,
                argtypes,
                argmodifiers,
                returntype,
                _format,
            ) => {
                format!(
                    "InbuiltFunctionDef: \"{}\" argtypes({:?}) argmodifiers({:?}) -> ({})",
                    name, argtypes, argmodifiers, returntype
                )
            }
            ElementInfo::InbuiltFunctionCall(name, _, returntype) => {
                format!("InbuiltFunctionCall: {} ({})", name, returntype)
            }
            ElementInfo::FunctionDefWIP => format!("FunctionDefWIP"),
            ElementInfo::FunctionDef(name, argnames, argtypes, returntype) => {
                let empty_arg_modifiers = argnames.iter().map(|_s| String::new()).collect();
                let args = formatting::get_formatted_argname_argtype_pairs(
                    &argnames,
                    &argtypes,
                    &empty_arg_modifiers,
                );
                format!("FunctionDef: {} ({}) -> ({})", name, args, returntype)
            }
            ElementInfo::FunctionCall(name, returntype) => {
                format!("FunctionCall: {} ({})", name, returntype)
            }
            ElementInfo::Parens => "Parens".to_string(),
            ElementInfo::Eol => "Eol".to_string(),
            ElementInfo::Seol => "Seol".to_string(),
            ElementInfo::Indent => "Indent".to_string(),
            ElementInfo::Type(name) => {
                format!("Type: {}", name)
            }
            ElementInfo::Unused => "Unused".to_string(),
            ElementInfo::LoopForRangeWIP => format!("LoopForRangeWIP"),
            ElementInfo::LoopForRange(name, from, to) => {
                format!("Loop For {} in {} to {}", name, from, to)
            }
            ElementInfo::Println => "Println".to_string(),
            ElementInfo::If(returntype) => format!("If ({})", returntype),
        };
        write!(f, "{}", el_debug)
    }
}

pub struct ElementsVec(pub Elements);

impl fmt::Debug for ElementsVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let root_index = 0;
        //let elements_vec = ElementsVec(self);
        write_subtree(&self, f, root_index, 0)?;
        Ok(())
    }
}

pub struct DebugElements<'a>(pub &'a Elements);

impl<'a> fmt::Debug for DebugElements<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut el_debug = "".to_string();
        let els = &self;
        for el in 0..els.0.len() {
            let children_debug = debug_flat_usize_array(&els.0[el].1);
            let elinfo_debug = format!("{:?} {}", els.0[el].0, children_debug);
            let el_index = if el > 9 {
                "".to_string()
            } else {
                " ".to_string()
            };
            el_debug = format!("{}{}{}: {}\r\n", el_debug, el_index, el, elinfo_debug);
        }
        write!(
            f,
            "Custom Debug of Elements [\r\nElements:\r\n{}\r\n]",
            el_debug
        )
    }
}

fn debug_flat_usize_array(arr: &Vec<usize>) -> String {
    let mut arr_debug = "[ ".to_string();
    for string in arr {
        arr_debug = format!("{}{}, ", arr_debug, string);
    }
    arr_debug = format!("{}]", arr_debug);
    arr_debug
}

fn write_subtree(
    elements: &ElementsVec,
    f: &mut fmt::Formatter<'_>,
    index: usize,
    indent_level: usize,
) -> fmt::Result {
    let (element_info, child_indices) = &elements.0[index];
    write_indent(f, indent_level)?;
    let el = match element_info {
        ElementInfo::Indent => "...".to_string(),
        _ => format!("{:?}", element_info),
    };
    write!(f, "{}", el)?;
    if child_indices.is_empty() {
        write!(f, "\n")?;
    } else {
        write!(f, " {{\n")?;
        for child_index in child_indices {
            write_subtree(elements, f, *child_index, indent_level + 1)?;
        }
        write_indent(f, indent_level)?;
        write!(f, "}}\n")?;
    }
    Ok(())
}

fn write_indent(f: &mut fmt::Formatter<'_>, indent_level: usize) -> fmt::Result {
    for _ in 0..indent_level {
        write!(f, "  ")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast::output;
    use crate::Ast;
    use crate::Compiler;
    //use crate::File;

    #[test]
    fn test_get_depths_vec() {
        //1 el
        let mut ast1 = Ast::new();
        let mut n = ast1.elements.len();
        let el1: Element = (ElementInfo::Int("1".to_string()), vec![]);
        append::append(&mut ast1, el1);
        assert_eq!(output::get_depths_vec(&mut ast1), vec![[n]]);

        //3 el under root
        let mut ast2 = Ast::new();
        n = ast2.elements.len();
        let el21: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el22: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el23: Element = (ElementInfo::Int("1".to_string()), vec![]);
        append::append(&mut ast2, el21);
        append::append(&mut ast2, el22);
        append::append(&mut ast2, el23);
        assert_eq!(output::get_depths_vec(&mut ast2), vec![[n, n + 1, n + 2]]);

        //1 el under with 2 children, under root
        let mut ast3 = Ast::new();
        n = ast3.elements.len();
        let el31: Element = (
            ElementInfo::InbuiltFunctionCall("+".to_string(), 1, "i64|f64".to_string()),
            vec![],
        );
        let el32: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el33: Element = (ElementInfo::Int("1".to_string()), vec![]);
        append::append(&mut ast3, el31);
        parents::indent::indent(&mut ast3);
        append::append(&mut ast3, el32);
        append::append(&mut ast3, el33);
        assert_eq!(
            output::get_depths_vec(&mut ast3),
            vec![vec![n], vec![n + 2, n + 1]]
        );

        //typical nested tree         this flat ast
        //0 (root)                    |_(0,[1,2,3,8]) root
        // note insert default functions first
        // so indexes will increase by # of functions
        //|_1 int                     |_(1,[])
        //|_2 int                     |_(2,[])
        //|_3 +                       |_(3,[4,5])
        //| |_4 int                   |_(4,[])
        //| |_5 +                     |_(5,[6,7])
        //|   |_6 int                 |_(6,[])
        //|   |_7 +                   |_(7,[8,9])
        //|     |_8 int               |_(8,[])
        //|     |_9 int               |_(9,[])
        //|_10 i64                    |_(10,[])

        /*
        [
            [1,2,3,10],
            [4,5],
            [6,7],
            [8,9]
        ]
        */

        //10 el in nested structure above

        let mut c: Compiler = Default::default();
        n = c.ast.elements.len();
        let el41: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el42: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el43: Element = (
            ElementInfo::InbuiltFunctionCall("+".to_string(), 1, "i64|f64".to_string()),
            vec![],
        );
        let el44: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el45: Element = (
            ElementInfo::InbuiltFunctionCall("+".to_string(), 1, "i64|f64".to_string()),
            vec![],
        );
        let el46: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el47: Element = (
            ElementInfo::InbuiltFunctionCall("+".to_string(), 1, "i64|f64".to_string()),
            vec![],
        );
        let el48: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el49: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el410: Element = (ElementInfo::Int("1".to_string()), vec![]);
        append::append(&mut c.ast, el41);
        append::append(&mut c.ast, el42);
        append::append(&mut c.ast, el43);
        parents::indent::indent(&mut c.ast);
        append::append(&mut c.ast, el44);
        append::append(&mut c.ast, el45);
        parents::indent::indent(&mut c.ast);
        append::append(&mut c.ast, el46);
        append::append(&mut c.ast, el47);
        parents::indent::indent(&mut c.ast);
        append::append(&mut c.ast, el48);
        append::append(&mut c.ast, el49);
        parents::outdent::outdent(&mut c);
        parents::outdent::outdent(&mut c);
        parents::outdent::outdent(&mut c);
        append::append(&mut c.ast, el410);

        assert_eq!(
            output::get_depths_vec(&mut c.ast),
            vec![
                vec![n, n + 1, n + 2, n + 9],
                vec![n + 4, n + 3],
                vec![n + 6, n + 5],
                vec![n + 8, n + 7]
            ]
        );
    }

    #[test]
    fn test_get_depths_flattened() {
        //let mut ast = Ast::new();
        let mut input = vec![vec![0]];
        assert_eq!(output::get_depths_flattened(&input), vec![0]);

        input = vec![vec![1, 2, 3]];
        assert_eq!(output::get_depths_flattened(&input), vec![1, 2, 3]);

        input = vec![vec![1], vec![2, 3]];
        assert_eq!(output::get_depths_flattened(&input), vec![2, 3, 1]);

        input = vec![vec![1, 2, 3, 10], vec![4, 5], vec![6, 7], vec![8, 9]];
        assert_eq!(
            output::get_depths_flattened(&input),
            vec![8, 9, 6, 7, 4, 5, 1, 2, 3, 10]
        );
    }
}
