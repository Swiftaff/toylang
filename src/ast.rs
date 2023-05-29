/*! AST containing adjacency list of Elements, output string, parents array
 */
pub mod elements;
pub mod output;
pub mod parents;

use crate::ast::elements::{ArgModifier, DebugElements, ElIndex, Element, ElementInfo, Elements};
use crate::Token;
use std::fmt;

type Logs = Vec<(String, Token)>;

/// AST containing Elements as an adjacency list of tree nodes.
/// Each node is represented by a tuple containing its identifier and a list of the indexes of its children.
/// The Tree can be walked by starting at the Root nodes children
/// ```text
/// // typical nested tree                          this flat ast
/// // 0 (root)                                     |_(0: Root, [1,2,3,10])
/// // note internal functions are inserted here first
/// // so indexes of actual elements will increase by # of functions rather than just 1 below
/// // |_1 int                                      |_(1: Int,  [])
/// // |_2 int                                      |_(2: Int,  [])
/// // |_3                                          |_(3,       [4,5])
/// // | |_4 int                                    |_(4: Int,  [])
/// // | |_5                                        |_(5,       [6,7])
/// // |   |_6 int                                  |_(6: Int,  [])
/// // |   |_7                                      |_(7,       [8,9])
/// // |     |_8 int                                |_(8: Int,  [])
/// // |     |_9 int                                |_(9: Int,  [])
/// // |_10 Int                                     |_(10: Int, [])
/// ```
#[derive(Clone)]
pub struct Ast {
    //first element is always root. Real elements start at index 1
    pub elements: Elements,
    pub output: String,
    pub output_stack: Vec<ElIndex>,
    pub premain_output: String,
    //note: parents are only used for building, ignored output.
    //becuse of that, split outputting to be less confusing?
    pub parents: Vec<ElIndex>,
    pub debug: bool,
    pub logs: Logs,
    pub debug_compiler_history: Vec<String>,
}

impl Default for Ast {
    fn default() -> Self {
        Ast {
            elements: init(),
            output: "".to_string(),
            output_stack: vec![],
            premain_output: "".to_string(),
            parents: vec![0], // get current indent from length of parents
            debug: false,
            logs: vec![],
            debug_compiler_history: vec![],
        }
    }
}

impl Ast {
    pub fn new(debug: bool) -> Ast {
        Ast {
            debug,
            ..Ast::default()
        }
    }
    /// Called by all functions - inserts a single log line, and a copy of the AST state
    pub fn log(self: &mut Self, string: String) {
        if self.debug {
            self.logs.push((string, ("".to_string(), 0, 0, 0)));
            let debug_els = format!(
                "{:?}\r\n\r\nParents:\r\n{:?}\r\n\r\nOutput_stack:\r\n{:?}\r\n\r\nPre-main Output:\r\n{:?}r\n\r\nOutput:\r\n{:?}",
                DebugElements(&self.elements),
                &self.parents,
                &self.output_stack,
                &self.premain_output,
                &self.output,
            );
            self.debug_compiler_history.push(debug_els);
        }
    }
}

/// Initialise a list of the internal functions - which are inserted "hidden" at the start of the AST after Root, but not as children of Root
/// They just sit there for later use, and before any real elements are added
fn init() -> Vec<Element> {
    vec![]
        .iter()
        .chain(&init_root())
        .chain(&init_initial_types())
        .chain(&init_initial_arithmetic_operators())
        .chain(&init_list_functions())
        .chain(&init_booleans())
        .chain(&init_boolean_fns())
        .cloned()
        .collect()
}

/// Initialise Root
fn init_root() -> Elements {
    vec![(ElementInfo::Root, vec![])]
}

/// Initialise Types
fn init_initial_types() -> Elements {
    let type_primitives = vec!["i64", "f64", "String", "bool"];
    let type_closure = |prim: &str| (ElementInfo::Type(prim.to_string()), vec![]);
    type_primitives.into_iter().map(type_closure).collect()
}

/// Initialise Booleans
fn init_booleans() -> Elements {
    let bool_fns = vec!["true", "false"];
    let bool_closure = |bool_name: &str| {
        (
            ElementInfo::InbuiltFunctionDef(
                bool_name.to_string(),
                vec![],
                vec![],
                vec![],
                "bool".to_string(),
                bool_name.to_string(),
            ),
            vec![],
        )
    };
    bool_fns.into_iter().map(bool_closure).collect()
}

/// Initialise Boolean Functions
fn init_boolean_fns() -> Elements {
    let bool_fns = vec!["==", "!=", "<", ">", "<=", ">="];
    let bool_closure = |bool_fn_name: &str| {
        (
            ElementInfo::InbuiltFunctionDef(
                bool_fn_name.to_string(),
                vec!["arg~1".to_string(), "arg~2".to_string()],
                vec![
                    "i64|f64|String|bool".to_string(),
                    "i64|f64|String|bool".to_string(),
                ],
                vec![ArgModifier::None, ArgModifier::None],
                "bool".to_string(),
                format!("arg~1 {} arg~2", bool_fn_name).to_string(),
            ),
            vec![],
        )
    };
    bool_fns.into_iter().map(bool_closure).collect()
}

/// Initialise Arithmetic Functions
fn init_initial_arithmetic_operators() -> Elements {
    let arithmetic_fns = vec!["+", "-", "*", "/", "%"];
    let arithmetic_closure = |fn_name: &str| {
        (
            ElementInfo::InbuiltFunctionDef(
                fn_name.to_string(),
                vec!["arg~1".to_string(), "arg~2".to_string()],
                vec!["i64|f64".to_string(), "i64|f64".to_string()],
                vec![ArgModifier::None, ArgModifier::None],
                "i64|f64".to_string(),
                format!("arg~1 {} arg~2", fn_name).to_string(),
            ),
            vec![],
        )
    };
    arithmetic_fns.into_iter().map(arithmetic_closure).collect()
}

/// Initialise List Functions
fn init_list_functions() -> Vec<elements::Element> {
    let vecs: &str = "Vec<i64>|Vec<f64>|Vec<String>";
    let list_fns = vec![
        (
            "map",
            "arg~1.iter().map(arg~2).collect()",
            vec![ArgModifier::None, ArgModifier::FnArg(vec!["&".to_string()])],
            vecs,
            vecs,
        ),
        (
            "mapindex",
            "arg~1.iter().enumerate().map(|x| {\r\nlet index = x.0;\r\nlet val = x.1;\r\n|index, val| arg~2).collect()",
            vec![ArgModifier::None, ArgModifier::FnArg(vec!["".to_string(),"&".to_string()])],
            vecs,
            vecs,
        ),
        (
            "append",
            "arg~1.iter().cloned().chain(arg~2.iter().cloned()).collect()",
            vec![ArgModifier::None, ArgModifier::None],
            vecs,
            vecs,
        ),
        (
            "len",
            "arg~1.len() as i64",
            vec![ArgModifier::None],
            vecs,
            "i64|f64|String",
        ),
        (
            "reverse",
            "arg~1.into_iter().rev().collect()",
            vec![ArgModifier::None],
            vecs,
            vecs,
        ),
    ];
    let list_closure = |(fn_name, output, modifier, argtype, returntype): (
        &str,
        &str,
        Vec<ArgModifier>,
        &str,
        &str,
    )| {
        let num_args = output.matches("arg~").count();
        let mut arg_names: Vec<String> = vec![];
        let mut arg_types: Vec<String> = vec![];
        let mut arg_modifiers: Vec<ArgModifier> = vec![];
        for i in 0..num_args {
            arg_names.push(format!("arg~{}", i + 1));
            arg_modifiers.push(modifier[i].clone());
            arg_types.push(argtype.to_string());
        }
        (
            ElementInfo::InbuiltFunctionDef(
                format!("List::{}", fn_name),
                arg_names,
                arg_types,
                arg_modifiers,
                returntype.to_string(),
                output.to_string(),
            ),
            vec![],
        )
    };
    list_fns.into_iter().map(list_closure).collect()
}

impl fmt::Debug for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut el_debug = "".to_string();
        let els = &self.elements;
        let parents_debug = debug_flat_usize_array(&self.parents);
        for el in 0..els.len() {
            let children_debug = debug_flat_usize_array(&els[el].1);
            let elinfo_debug = format!("{:?} {}", els[el].0, children_debug);
            let el_index = if el > 9 {
                "".to_string()
            } else {
                " ".to_string()
            };
            el_debug = format!("{}{}{}: {}\r\n", el_debug, el_index, el, elinfo_debug);
        }
        write!(
            f,
            "Custom Debug of Ast [\r\nElements:\r\n{}Parents: {}\r\nOutput_stack: {:?}\r\nOutput: \r\n{:?}\r\n]",
            el_debug, parents_debug, self.output_stack, self.output
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
