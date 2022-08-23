pub mod append;
use crate::ast::elements;
use crate::ast::parents;
use crate::formatting;
use crate::Ast;
use crate::Compiler;
use std::fmt;

#[derive(Clone)]

pub enum ElementInfo {
    List(ReturnType),                       //children = list items of same type
    CommentSingleLine(Value),               //no children
    Int(Value),                             //no children
    Float(Value),                           //no children
    String(Value),                          //no children
    Arg(Name, Scope, ReturnType),           //no children
    Type(Name),                             //no children
    Eol,                                    //no children
    Seol,                                   //no children
    Indent,                                 //no children
    Unused,                                 //no children
    ConstantRef(Name, ReturnType, RefName), //no children
    Constant(Name, ReturnType),             //1 child, value
    Assignment,                             //1 child, constant
    InbuiltFunctionDef(Name, ArgNames, ArgTypes, ReturnType, Format), //children = lines of function contents
    InbuiltFunctionCall(Name, ElIndex, ReturnType), //fndef argnames.len() children
    FunctionDefWIP,                                 //children = lines of function contents
    FunctionDef(Name, ArgNames, ArgTypes, ReturnType), //children = lines of function contents
    FunctionCall(Name, ReturnType),                 //fndef argnames.len() children
    Parens,          //either 1 child, for function_ref, or 1+ for function type sig
    LoopForRangeWIP, //children = lines of loop contents
    LoopForRange(Name, From, To), //children = lines of loop contents
    Println,         //1 child, value
    Root,            //children = lines of function contents
}

// this is fake function #1, only useful for copy/pasting all the elementinfo types!
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
        ElementInfo::Arg(_, _, _) => replaceme,
        ElementInfo::Constant(_, _) => replaceme,
        ElementInfo::ConstantRef(_, _, _) => replaceme,
        ElementInfo::Assignment => replaceme,
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => replaceme,
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
        ElementInfo::LoopForRangeWIP => replaceme,
        ElementInfo::LoopForRange(_, _, _) => replaceme,
    }
}

// this is a fake function #2, only useful for copy/pasting all the option_element types!
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
        Some((ElementInfo::Arg(_, _, _), _)) => replaceme,
        Some((ElementInfo::Constant(_, _), _)) => replaceme,
        Some((ElementInfo::ConstantRef(_, _, _), _)) => replaceme,
        Some((ElementInfo::Assignment, _)) => replaceme,
        Some((ElementInfo::InbuiltFunctionDef(_, _, _, _, _), _)) => replaceme,
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
        Some((ElementInfo::LoopForRangeWIP, _)) => replaceme,
        Some((ElementInfo::LoopForRange(_, _, _), _)) => replaceme,
        None => replaceme,
    }
}

type Value = String;
pub type ElIndex = usize;
type From = usize;
type To = usize;
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
    None
}

pub fn get_arg_index_by_name(ast: &Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match elinfo {
        ElementInfo::Arg(n, _, _) => n == name,
        _ => false,
    })
}

pub fn get_inbuilt_type_index_by_name(ast: &Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match elinfo {
        ElementInfo::Type(n) => n == name,
        _ => false,
    })
}

pub fn get_constant_index_by_name(ast: &Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match elinfo {
        ElementInfo::Constant(n, _t) => n == name,
        ElementInfo::ConstantRef(n, _t, _refname) => n == name,
        _ => false,
    })
}

pub fn get_constant_by_name(ast: &Ast, name: &String) -> Option<ElementInfo> {
    if let Some(index) = get_constant_index_by_name(ast, name) {
        return Some(ast.elements[index].0.clone());
    }
    None
}

pub fn get_function_index_by_name(ast: &Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match &elinfo {
        ElementInfo::FunctionDef(n, _, _, _) => n == name,
        ElementInfo::Arg(n, _, r) => n == name && r.contains("&dyn Fn"),
        _ => false,
    })
}

pub fn get_inbuilt_function_index_by_name(ast: &Ast, name: &String) -> Option<usize> {
    ast.elements.iter().position(|(elinfo, _)| match &elinfo {
        ElementInfo::InbuiltFunctionDef(n, _, _, _, _) => n == name,
        ElementInfo::Arg(n, _, r) => n == name && r.contains("&dyn Fn"),
        _ => false,
    })
}

pub fn _get_inbuilt_function_index_by_name_and_returntype(
    ast: &Ast,
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

pub fn get_inbuilt_function_by_name(ast: &Ast, name: &String) -> Option<ElementInfo> {
    if let Some(index) = get_inbuilt_function_index_by_name(ast, name) {
        return Some(ast.elements[index].0.clone());
    }
    None
}

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

pub fn get_last_element(ast: &Ast) -> Element {
    ast.elements.last().unwrap().clone()
}

pub fn get_updated_elementinfo_with_infered_type(ast: &mut Ast, el_index: usize) -> ElementInfo {
    let el = ast.elements[el_index].clone();
    let el_type = get_elementinfo_type(ast, &el.0);
    if el_type == "Undefined".to_string() {
        let infered_type = get_infered_type_of_any_element(ast, el_index);
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
            ElementInfo::List(returntype) => {
                if el.1.len() == 0 {
                    return ElementInfo::List(returntype);
                } else {
                    let first_child_type = get_infered_type_of_any_element(&ast, el.1[0]);
                    return ElementInfo::List(format!("Vec<{}>", first_child_type));
                }
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
            ElementInfo::LoopForRangeWIP => (),
            ElementInfo::LoopForRange(_, _, _) => (),
            ElementInfo::Println => (),
        }
        //dbg!(el_index, &ast.elements[el_index].0);
    }
    el.0
}

pub fn get_infered_type_of_any_element(ast: &Ast, el_index: usize) -> String {
    let el = ast.elements[el_index].clone();
    let el_info = &el.0;
    match el_info {
        ElementInfo::Arg(_, _, _) => {
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
        // explicitly listing other types rather than using _ to not overlook new types in future
        ElementInfo::Root => (),
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
        ElementInfo::LoopForRangeWIP => (),
        ElementInfo::LoopForRange(_, _, _) => (),
        ElementInfo::Println => (),
    }
    get_elementinfo_type(ast, el_info)
}

pub fn get_infered_type_of_arg_element(
    ast: &Ast,
    el_info: &ElementInfo,
    el_index: usize,
) -> String {
    let mut infered_type = "Undefined".to_string();
    match el_info {
        ElementInfo::Arg(name, _, _) => {
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

pub fn get_infered_type_of_constant_element(ast: &Ast, el: &Element) -> String {
    let mut infered_type = "Undefined".to_string();
    match el.0 {
        ElementInfo::Constant(_, _) => {
            if el.1.len() > 0 {
                let child_ref = el.1[0];
                infered_type = get_infered_type_of_any_element(ast, child_ref);
            }
        }
        _ => (),
    }
    infered_type
}

pub fn get_infered_type_of_constantref_element(ast: &Ast, refname: &String) -> String {
    let mut infered_type = "Undefined".to_string();
    if let Some(ElementInfo::Constant(_, returntype)) = get_constant_by_name(ast, &refname) {
        infered_type = returntype
    }
    infered_type
}

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
        ElementInfo::InbuiltFunctionDef(_, _argnames, argtypes, returntype, _) => {
            //TODO could check all args match here for parser error
            //dbg!("2", &returntype);
            if returntype.contains("|") {
                //dbg!("2.5", &el_children);
                if el_children.len() > 0 && argtypes.len() <= el_children.len() {
                    for _argtype in argtypes {
                        let first_child_ref = el_children[0];
                        let first_child = &ast.elements[first_child_ref];
                        infered_type = get_elementinfo_type(ast, &first_child.0);
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

pub fn get_infered_type_of_functioncall_element(ast: &Ast, name: &String) -> String {
    let undefined = "Undefined".to_string();
    if let Some(index) = get_function_index_by_name(ast, &name) {
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

pub fn get_elementinfo_type(ast: &Ast, elementinfo: &ElementInfo) -> String {
    let undefined = "Undefined".to_string();
    match elementinfo {
        ElementInfo::List(returntype) => returntype.clone(),
        ElementInfo::Int(_) => "i64".to_string(),
        ElementInfo::Float(_) => "f64".to_string(),
        ElementInfo::String(_) => "String".to_string(),
        ElementInfo::Assignment => undefined,
        ElementInfo::Constant(_, returntype) => returntype.clone(),
        ElementInfo::ConstantRef(_, returntype, _) => returntype.clone(),
        ElementInfo::InbuiltFunctionCall(_, _fndef_index, returntype) => returntype.clone(),
        ElementInfo::Arg(_, _, returntype) => returntype.clone(),
        ElementInfo::FunctionCall(name, _) => get_infered_type_of_functioncall_element(ast, &name),
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
        ElementInfo::LoopForRangeWIP => undefined,
        ElementInfo::LoopForRange(_, _, _) => undefined,
        ElementInfo::Println => undefined,
    }
}

pub fn is_existing_constant(compiler: &Compiler) -> bool {
    let parent = parents::get_current_parent_element_from_parents(&compiler.ast);
    let mut parent_assignment_has_no_children = false;
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
    parent_assignment_has_no_children
}

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

pub fn replace_funcdefwip_with_funcdef(
    compiler: &mut Compiler,
    children: &[usize],
    name: &String,
    func_def_ref: usize,
) {
    //assign name, argtypes, argnames, returntype to parent funcdef
    let argtypes = get_argtypes_from_argtokens(compiler, &children);
    let returntype = get_returntype_from_argtokens(compiler, &children);
    let argnames = get_argnames_from_argtokens(compiler, &children, &argtypes);
    let new_funcdef = ElementInfo::FunctionDef(name.clone(), argnames, argtypes, returntype);

    // replace original funcdefWIP with funcdef
    compiler.ast.elements[func_def_ref] = (new_funcdef, vec![]);
}

pub fn get_argtypes_from_argtokens(compiler: &Compiler, children: &[usize]) -> Vec<String> {
    let mut argtypes: Vec<String> = vec![];
    let num_args = children.len() / 2;
    let argtype_refs = &children[..num_args];
    for a in argtype_refs {
        match &compiler.ast.elements[a.clone()] {
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

pub fn get_returntype_from_argtokens(compiler: &Compiler, children: &[usize]) -> String {
    let num_args = children.len() / 2;
    let returntype_ref = &children[num_args];
    return match &compiler.ast.elements[returntype_ref.clone()] {
        (ElementInfo::Type(typename), _) => typename.clone(),
        _ => "Undefined".to_string(),
    };
}

pub fn get_argnames_from_argtokens(
    compiler: &mut Compiler,
    children: &[usize],
    argtypes: &Vec<String>,
) -> Vec<String> {
    //get argnames from Arg tokens
    //but also update Arg tokens returntypes at same time
    //TODO make up mind about just using the Arg tokens as the definition of argnames/argtypes
    let mut argnames: Vec<String> = vec![];
    let num_args = children.len() / 2;
    let argname_refs = &children[num_args + 1..];
    for i in 0..argname_refs.len() {
        let a = argname_refs[i];
        match &compiler.ast.elements[a] {
            (ElementInfo::Arg(argname, scope, _), _) => {
                argnames.push(argname.clone());
                let returntype = argtypes[i].clone();
                let updated_arg_token =
                    ElementInfo::Arg(argname.clone(), scope.clone(), returntype);
                compiler.ast.elements[a].0 = updated_arg_token;
            }
            _ => (),
        }
    }
    argnames
}

pub fn get_formatted_dyn_fn_type_sig(compiler: &Compiler, paren_children: &Vec<usize>) -> String {
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
                let args = formatting::get_formatted_argname_argtype_pairs(&argnames, &argtypes);
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
        };
        write!(f, "{}", el_debug)
    }
}

#[cfg(test)]
mod tests {

    // cargo watch -x "test test_get_depths_vec"
    // cargo watch -x "test test_get_depths_vec -- --show-output"
    // cargo test test_get_depths_vec -- --show-output

    use super::*;
    use crate::ast::output;
    use crate::Ast;
    use crate::Compiler;
    use crate::File;

    #[test]
    fn test_get_depths_vec() {
        //1 el
        let mut ast1 = Ast::new();
        let mut n = ast1.elements.len();
        let el1: Element = (ElementInfo::Int("1".to_string()), vec![]);
        append::append(&mut ast1, el1);
        assert_eq!(output::get_depths_vec(ast1), vec![[n]]);

        //3 el under root
        let mut ast2 = Ast::new();
        n = ast2.elements.len();
        let el21: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el22: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el23: Element = (ElementInfo::Int("1".to_string()), vec![]);
        append::append(&mut ast2, el21);
        append::append(&mut ast2, el22);
        append::append(&mut ast2, el23);
        assert_eq!(output::get_depths_vec(ast2), vec![[n, n + 1, n + 2]]);

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
            output::get_depths_vec(ast3),
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

        fn mock_compiler() -> Compiler {
            Compiler {
                file: File::new(),
                lines_of_chars: vec![],
                lines_of_tokens: vec![],
                output: "".to_string(),
                current_line: 0,
                current_line_token: 0,
                error_stack: vec![],
                ast: Ast::new(),
            }
        }
        let mut c = mock_compiler();
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
            output::get_depths_vec(c.ast),
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
