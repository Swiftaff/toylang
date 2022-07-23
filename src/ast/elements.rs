use crate::formatting::get_formatted_argname_argtype_pairs;
use std::fmt;

impl fmt::Debug for ElementInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let el_debug = match self {
            ElementInfo::Root => format!("Root"),
            ElementInfo::CommentSingleLine(comment) => {
                format!("Comment: {}", comment)
            }
            ElementInfo::Int(int) => format!("Int: {}", int),
            ElementInfo::Float(float) => format!("Float: {}", float),
            ElementInfo::String(string) => format!("String: {}", string),
            ElementInfo::Arg(name, scope, returntype) => {
                format!("Arg: {} scope:{} ({})", name, scope, returntype)
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
            ElementInfo::InbuiltFunctionDef(name, _argnames, _argtypes, returntype, _format) => {
                format!("InbuiltFunctionDef: \"{}\" ({})", name, returntype)
            }
            ElementInfo::InbuiltFunctionCall(name, _, returntype) => {
                format!("InbuiltFunctionCall: {} ({})", name, returntype)
            }
            ElementInfo::FunctionDefWIP => format!("FunctionDefWIP"),
            ElementInfo::FunctionDef(name, argnames, argtypes, returntype) => {
                let args = get_formatted_argname_argtype_pairs(&argnames, &argtypes);
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
        };
        write!(f, "{}", el_debug)
    }
}

#[derive(Clone)]

pub enum ElementInfo {
    Root,
    CommentSingleLine(Value),               //no children
    Int(Value),                             //no children
    Float(Value),                           //no children
    String(Value),                          //no children
    Arg(Name, Scope, ReturnType),           //no children
    Constant(Name, ReturnType),             //1 child, value
    ConstantRef(Name, ReturnType, RefName), //no children
    Assignment,                             //1 child, constant
    InbuiltFunctionDef(Name, ArgNames, ArgTypes, ReturnType, Format), //no children
    InbuiltFunctionCall(Name, ElIndex, ReturnType), //fndef argnames.len() children
    FunctionDefWIP,                         //no children
    FunctionDef(Name, ArgNames, ArgTypes, ReturnType), //no children
    FunctionCall(Name, ReturnType),         //fndef argnames.len() children
    Parens,     //either 1 child, for function_ref, or 1+ for function type sig
    Type(Name), // no children
    Eol,
    Seol,
    Indent,
    Unused,
}

type Value = String;
pub type ElIndex = usize;
type ReturnType = String;
type Name = String;
type RefName = String;
type ArgNames = Vec<String>;
type ArgTypes = Vec<String>;
type Format = String;
type Scope = ElIndex;
// no need to track parents in Element
// should only ever be one per Element so can search for it each time
// to save double handling parent/child refs in two places

/*
//this is not defined in the current crate because tuples are always foreign
impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let children_debug = debug_flat_usize_array(&self.1);
        let elinfo_debug = format!("{:?} {}", self.0, children_debug);
        let el_debug = format!("{}\r\n", elinfo_debug);

        write!(f, "{}", el_debug)
    }
}
*/

pub type Elements = Vec<Element>;
pub type Element = (ElementInfo, ElementChildren);
pub type ElementChildren = Vec<ElIndex>;

pub fn append(ast: &mut super::Ast, element: Element) -> usize {
    // add element to list, and add to list of children of current parent where 0 = root
    ast.elements.push(element);
    let new_items_index = ast.elements.len() - 1;
    let current_parent_ref = ast.get_current_parent_ref_from_parents();
    ast.elements[current_parent_ref].1.push(new_items_index);
    new_items_index
}

pub fn append_as_ref(ast: &mut super::Ast, element: Element) -> usize {
    // add element to list only, don't add as child
    ast.elements.push(element);
    let new_items_index = ast.elements.len() - 1;
    new_items_index
}

pub fn get_element_by_name(ast: &super::Ast, name: &String) -> Option<Element> {
    if let Some(index) = ast.get_constant_index_by_name(name) {
        return Some(ast.elements[index].clone());
    }
    if let Some(index) = ast.get_inbuilt_function_index_by_name(name) {
        return Some(ast.elements[index].clone());
    }
    if let Some(index) = ast.get_function_index_by_name(name) {
        return Some(ast.elements[index].clone());
    }
    if let Some(index) = ast.get_inbuilt_type_index_by_name(name) {
        return Some(ast.elements[index].clone());
    }
    if let Some(index) = ast.get_arg_index_by_name(name) {
        return Some(ast.elements[index].clone());
    }
    None
}

pub fn get_arg_index_by_name(ast: &super::Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match elinfo {
        ElementInfo::Arg(n, _, _) => n == name,
        _ => false,
    })
}

pub fn get_inbuilt_type_index_by_name(ast: &super::Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match elinfo {
        ElementInfo::Type(n) => n == name,
        _ => false,
    })
}

pub fn get_constant_index_by_name(ast: &super::Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match elinfo {
        ElementInfo::Constant(n, _t) => n == name,
        ElementInfo::ConstantRef(n, _t, _refname) => n == name,
        _ => false,
    })
}

pub fn get_constant_by_name(ast: &super::Ast, name: &String) -> Option<ElementInfo> {
    if let Some(index) = ast.get_constant_index_by_name(name) {
        return Some(ast.elements[index].0.clone());
    }
    None
}

pub fn get_function_index_by_name(ast: &super::Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match &elinfo {
        ElementInfo::FunctionDef(n, _, _, _) => n == name,
        ElementInfo::Arg(n, _, r) => n == name && r.contains("&dyn Fn"),
        _ => false,
    })
}

pub fn get_inbuilt_function_index_by_name(ast: &super::Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match &elinfo {
        ElementInfo::InbuiltFunctionDef(n, _, _, _, _) => n == name,
        ElementInfo::Arg(n, _, r) => n == name && r.contains("&dyn Fn"),
        _ => false,
    })
}

pub fn get_inbuilt_function_index_by_name_and_returntype(
    ast: &super::Ast,
    name: &String,
    returntype: &String,
) -> Option<usize> {
    //dbg!(returntype);
    ast.elements.iter().position(|(elinfo, _)| match &elinfo {
        ElementInfo::InbuiltFunctionDef(n, _, _, r, _) => {
            //dbg!("here", n, r, name, returntype);
            n == name && (r.contains(returntype) || returntype.contains(r))
        }
        ElementInfo::Arg(n, _, r) => n == name && r.contains("&dyn Fn") && r.contains(returntype),
        _ => false,
    })
}

pub fn get_inbuilt_function_by_name(ast: &super::Ast, name: &String) -> Option<ElementInfo> {
    if let Some(index) = ast.get_inbuilt_function_index_by_name(name) {
        return Some(ast.elements[index].0.clone());
    }
    None
}

pub fn get_inbuilt_function_by_name_and_returntype(
    ast: &super::Ast,
    name: &String,
    returntype: &String,
) -> Option<ElementInfo> {
    //dbg!(returntype);
    if let Some(index) = ast.get_inbuilt_function_index_by_name_and_returntype(name, returntype) {
        return Some(ast.elements[index].0.clone());
    }
    None
}

pub fn get_last_element(ast: &super::Ast) -> Element {
    ast.elements.last().unwrap().clone()
}

pub fn get_updated_elementinfo_with_infered_type(
    ast: &mut super::Ast,
    el_index: usize,
) -> ElementInfo {
    let el = ast.elements[el_index].clone();
    let el_type = ast.get_elementinfo_type(&el.0);
    if el_type == "Undefined".to_string() {
        let infered_type = ast.get_infered_type_of_any_element(el_index);
        match el.0 {
            ElementInfo::Arg(name, scope, _) => {
                return ElementInfo::Arg(name, scope, infered_type);
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
            // explicitly listing other types rather than using _ to not overlook new types in future.
            // These either have no type or are predefined and can't be infered
            ElementInfo::Root => (),
            ElementInfo::CommentSingleLine(_) => (),
            ElementInfo::Int(_) => (),
            ElementInfo::Float(_) => (),
            ElementInfo::String(_) => (),
            ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => (),
            ElementInfo::FunctionDefWIP => (),
            ElementInfo::FunctionDef(_, _, _, _) => (),
            ElementInfo::Parens => (),
            ElementInfo::Type(_) => (),
            ElementInfo::Eol => (),
            ElementInfo::Seol => (),
            ElementInfo::Indent => (),
            ElementInfo::Unused => (),
        }
        //dbg!(el_index, &ast.elements[el_index].0);
    }
    el.0
}

pub fn get_infered_type_of_any_element(ast: &mut super::Ast, el_index: usize) -> String {
    let el = ast.elements[el_index].clone();
    let el_info = &el.0;
    match el_info {
        ElementInfo::Arg(_, _, _) => {
            return ast.get_infered_type_of_arg_element(el_info, el_index);
        }
        ElementInfo::Constant(_, _) => {
            return ast.get_infered_type_of_constant_element(&el);
        }
        ElementInfo::ConstantRef(_, _, refname) => {
            return ast.get_infered_type_of_constantref_element(&refname);
        }
        ElementInfo::InbuiltFunctionCall(_, fndef_index, _) => {
            return ast.get_infered_type_of_inbuiltfunctioncall_element(&el, *fndef_index);
        }
        ElementInfo::FunctionCall(name, _) => {
            return ast.get_infered_type_of_functioncall_element(&name);
        }
        // explicitly listing other types rather than using _ to not overlook new types in future
        ElementInfo::Root => {}
        ElementInfo::CommentSingleLine(_) => (),
        ElementInfo::Int(_) => (),
        ElementInfo::Float(_) => (),
        ElementInfo::String(_) => (),
        ElementInfo::Assignment => (),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => (),
        ElementInfo::FunctionDefWIP => (),
        ElementInfo::FunctionDef(_, _, _, _) => (),
        ElementInfo::Parens => (),
        ElementInfo::Type(_) => (),
        ElementInfo::Eol => (),
        ElementInfo::Seol => (),
        ElementInfo::Indent => (),
        ElementInfo::Unused => (),
    }
    ast.get_elementinfo_type(el_info)
}

pub fn get_infered_type_of_arg_element(
    ast: &mut super::Ast,
    el_info: &ElementInfo,
    el_index: usize,
) -> String {
    let mut infered_type = "Undefined".to_string();
    match el_info {
        ElementInfo::Arg(name, _, _) => {
            // get type of this arg, from the argtypes of parent funcdef
            if let Some(parent_funcdef) =
                ast.get_current_parent_element_from_element_children_search(el_index)
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

pub fn get_infered_type_of_constant_element(ast: &mut super::Ast, el: &Element) -> String {
    let mut infered_type = "Undefined".to_string();
    match el.0 {
        ElementInfo::Constant(_, _) => {
            if el.1.len() > 0 {
                let child_ref = el.1[0];
                infered_type = ast.get_infered_type_of_any_element(child_ref);
            }
        }
        _ => (),
    }
    infered_type
}

pub fn get_infered_type_of_constantref_element(ast: &mut super::Ast, refname: &String) -> String {
    let mut infered_type = "Undefined".to_string();
    if let Some(ElementInfo::Constant(_, returntype)) = ast.get_constant_by_name(&refname) {
        infered_type = returntype
    }
    infered_type
}

pub fn get_infered_type_of_inbuiltfunctioncall_element(
    ast: &mut super::Ast,
    func_call_el: &Element,
    funcdef_el_index: usize,
) -> String {
    let mut infered_type = "Undefined".to_string();
    let el_children = func_call_el.1.clone();
    let el = &ast.elements[funcdef_el_index];
    let elinfo = &el.0;
    match elinfo {
        ElementInfo::InbuiltFunctionDef(_, _argnames, argtypes, returntype, _) => {
            //TODO could check all args match here for parser error
            //dbg!("2", &returntype);
            if returntype.contains("|") {
                //dbg!("2.5", &el_children);
                if el_children.len() > 0 && argtypes.len() <= el_children.len() {
                    for _argtype in argtypes {
                        let first_child_ref = el_children[0];
                        let first_child = &ast.elements[first_child_ref];
                        infered_type = ast.get_elementinfo_type(&first_child.0);
                        //dbg!("2.6", &infered_type);
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

pub fn get_infered_type_of_functioncall_element(ast: &super::Ast, name: &String) -> String {
    let undefined = "Undefined".to_string();
    if let Some(index) = ast.get_function_index_by_name(&name) {
        let funcdef = &ast.elements[index];
        match &funcdef.0 {
            ElementInfo::FunctionDef(_, _, _, returntype) => return returntype.clone(),
            ElementInfo::Arg(_, _, returntype) => {
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

pub fn get_elementinfo_type(ast: &super::Ast, elementinfo: &ElementInfo) -> String {
    let undefined = "Undefined".to_string();
    match elementinfo {
        ElementInfo::Int(_) => "i64".to_string(),
        ElementInfo::Float(_) => "f64".to_string(),
        ElementInfo::String(_) => "String".to_string(),
        ElementInfo::Assignment => undefined,
        ElementInfo::Constant(_, returntype) => returntype.clone(),
        ElementInfo::ConstantRef(_, returntype, _) => returntype.clone(),
        ElementInfo::InbuiltFunctionCall(_, _fndef_index, returntype) => returntype.clone(),
        ElementInfo::Arg(_, _, returntype) => returntype.clone(),
        ElementInfo::FunctionCall(name, _) => ast.get_infered_type_of_functioncall_element(&name),
        ElementInfo::Type(returntype) => returntype.clone(),
        // explicitly listing other types rather than using _ to not overlook new types in future
        ElementInfo::Root => undefined,
        ElementInfo::CommentSingleLine(_) => undefined,
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => undefined, // don't want to 'find' definitions
        ElementInfo::FunctionDefWIP => undefined,
        ElementInfo::FunctionDef(_, _, _, _) => undefined, // don't want to 'find' definitions
        ElementInfo::Parens => undefined,
        ElementInfo::Eol => undefined,
        ElementInfo::Seol => undefined,
        ElementInfo::Indent => undefined,
        ElementInfo::Unused => undefined,
    }
}

pub fn replace_element_child(ast: &mut super::Ast, element_ref: usize, from: usize, to: usize) {
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
