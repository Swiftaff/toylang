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
