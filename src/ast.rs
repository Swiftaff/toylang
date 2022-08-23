pub mod elements;
pub mod output;
pub mod parents;

use crate::ast::elements::{ElIndex, ElementInfo, Elements};
use std::fmt;

#[derive(Clone)]
pub struct Ast {
    //first element is always root. Real elements start at index 1
    pub elements: Elements,
    pub output: String,
    //note: parents are only used for building, ignored output.
    //becuse of that, split outputting to be less confusing?
    pub parents: Vec<ElIndex>,
}

impl Ast {
    pub fn new() -> Ast {
        let types = get_initial_types();
        let arithmetic = get_initial_arithmetic_operators();
        let list_map = get_list_map();
        let root = vec![(ElementInfo::Root, vec![])];
        let elements: Elements = vec![]
            .iter()
            .chain(&root)
            .chain(&arithmetic)
            .chain(&list_map)
            .chain(&types)
            .cloned()
            .collect();
        Ast {
            elements,
            output: "".to_string(),
            parents: vec![0], // get current indent from length of parents
        }
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

fn get_initial_types() -> Elements {
    let type_primitives = vec!["i64", "f64", "String"];
    let type_closure = |prim: &str| (ElementInfo::Type(prim.to_string()), vec![]);
    type_primitives.into_iter().map(type_closure).collect()
}

fn get_initial_arithmetic_operators() -> Elements {
    let arithmetic_primitives = vec!["+", "-", "*", "/", "%"];
    let arithmetic_closure = |prim: &str| {
        (
            ElementInfo::InbuiltFunctionDef(
                prim.to_string(),
                vec!["arg~1".to_string(), "arg~2".to_string()],
                vec!["i64|f64".to_string(), "i64|f64".to_string()],
                "i64|f64".to_string(),
                format!("arg~1 {} arg~2", prim).to_string(),
            ),
            vec![],
        )
    };
    arithmetic_primitives
        .into_iter()
        .map(arithmetic_closure)
        .collect()
}

fn get_list_map() -> Vec<elements::Element> {
    vec![(
        ElementInfo::InbuiltFunctionDef(
            "List.map".to_string(),
            vec!["arg~1".to_string(), "arg~2".to_string()],
            vec![
                "Vec<i64>|Vec<f64>|Vec<String>".to_string(),
                "Vec<i64>|Vec<f64>|Vec<String>".to_string(),
            ],
            "Vec<i64>|Vec<f64>|Vec<String>".to_string(),
            "arg~1.iter().map(arg~2).collect()".to_string(),
        ),
        vec![],
    )]
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
            "Custom Debug of Ast [\r\nElements:\r\n{}Parents: {}\r\nOutput: \r\n{:?}\r\n]",
            el_debug, parents_debug, self.output
        )
    }
}
