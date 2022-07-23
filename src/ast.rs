pub mod elements;
pub mod output;
pub mod parents;
use crate::ast::elements::{ElIndex, Element, ElementInfo, Elements};
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
        let root = vec![(ElementInfo::Root, vec![])];
        let elements: Elements = vec![]
            .iter()
            .chain(&root)
            .chain(&arithmetic)
            .chain(&types)
            .cloned()
            .collect();
        Ast {
            elements,
            output: "".to_string(),
            parents: vec![0], // get current indent from length of parents
        }
    }

    // ELEMENTS
    pub fn append(&mut self, element: Element) {
        elements::append(self, element);
    }

    pub fn append_as_ref(self: &mut Self, element: Element) -> usize {
        elements::append_as_ref(self, element)
    }

    pub fn get_element_by_name(self: &Self, name: &String) -> Option<Element> {
        elements::get_element_by_name(self, name)
    }

    pub fn get_arg_index_by_name(self: &Self, name: &String) -> Option<usize> {
        elements::get_arg_index_by_name(self, name)
    }

    pub fn get_inbuilt_type_index_by_name(self: &Self, name: &String) -> Option<usize> {
        elements::get_inbuilt_type_index_by_name(self, name)
    }

    pub fn get_constant_index_by_name(self: &Self, name: &String) -> Option<usize> {
        elements::get_constant_index_by_name(self, name)
    }

    pub fn get_constant_by_name(self: &Self, name: &String) -> Option<ElementInfo> {
        elements::get_constant_by_name(self, name)
    }

    pub fn get_function_index_by_name(self: &Self, name: &String) -> Option<usize> {
        elements::get_function_index_by_name(self, name)
    }

    pub fn get_inbuilt_function_index_by_name(self: &Self, name: &String) -> Option<usize> {
        elements::get_inbuilt_function_index_by_name(self, name)
    }

    pub fn get_inbuilt_function_index_by_name_and_returntype(
        self: &Self,
        name: &String,
        returntype: &String,
    ) -> Option<usize> {
        elements::get_inbuilt_function_index_by_name_and_returntype(self, name, returntype)
    }

    pub fn get_inbuilt_function_by_name(self: &Self, name: &String) -> Option<ElementInfo> {
        elements::get_inbuilt_function_by_name(self, name)
    }

    pub fn get_inbuilt_function_by_name_and_returntype(
        self: &Self,
        name: &String,
        returntype: &String,
    ) -> Option<ElementInfo> {
        elements::get_inbuilt_function_by_name_and_returntype(self, name, returntype)
    }

    pub fn get_last_element(self: &Self) -> Element {
        elements::get_last_element(self)
    }

    fn get_updated_elementinfo_with_infered_type(self: &mut Self, el_index: usize) -> ElementInfo {
        elements::get_updated_elementinfo_with_infered_type(self, el_index)
    }

    fn get_infered_type_of_any_element(self: &mut Self, el_index: usize) -> String {
        elements::get_infered_type_of_any_element(self, el_index)
    }

    fn get_infered_type_of_arg_element(
        self: &mut Self,
        el_info: &ElementInfo,
        el_index: usize,
    ) -> String {
        elements::get_infered_type_of_arg_element(self, el_info, el_index)
    }

    fn get_infered_type_of_constant_element(self: &mut Self, el: &Element) -> String {
        elements::get_infered_type_of_constant_element(self, el)
    }

    fn get_infered_type_of_constantref_element(self: &mut Self, refname: &String) -> String {
        elements::get_infered_type_of_constantref_element(self, refname)
    }

    fn get_infered_type_of_inbuiltfunctioncall_element(
        self: &mut Self,
        func_call_el: &Element,
        funcdef_el_index: usize,
    ) -> String {
        elements::get_infered_type_of_inbuiltfunctioncall_element(
            self,
            func_call_el,
            funcdef_el_index,
        )
    }

    fn get_infered_type_of_functioncall_element(self: &Self, name: &String) -> String {
        elements::get_infered_type_of_functioncall_element(self, name)
    }

    pub fn get_elementinfo_type(self: &Self, elementinfo: &ElementInfo) -> String {
        elements::get_elementinfo_type(self, elementinfo)
    }

    pub fn replace_element_child(self: &mut Self, element_ref: usize, from: usize, to: usize) {
        elements::replace_element_child(self, element_ref, from, to)
    }

    // PARENTS

    pub fn get_current_parent_element_from_parents(self: &mut Self) -> Element {
        parents::get_current_parent_element_from_parents(self)
    }

    pub fn get_current_parent_ref_from_parents(self: &mut Self) -> usize {
        parents::get_current_parent_ref_from_parents(self)
    }

    pub fn get_current_parent_element_from_element_children_search(
        self: &mut Self,
        child_ref: usize,
    ) -> Option<Element> {
        parents::get_current_parent_element_from_element_children_search(self, child_ref)
    }

    pub fn get_current_parent_ref_from_element_children_search(
        self: &mut Self,
        child_ref: usize,
    ) -> Option<usize> {
        parents::get_current_parent_ref_from_element_children_search(self, child_ref)
    }

    fn get_indent(self: &Self) -> String {
        parents::get_indent(self)
    }

    pub fn indent(self: &mut Self) {
        parents::indent(self)
    }

    pub fn indent_this(self: &mut Self, index: usize) {
        parents::indent_this(self, index)
    }

    pub fn outdent(self: &mut Self) {
        parents::outdent(self)
    }

    // OUTPUT

    fn replace_any_unknown_types(self: &mut Self) {
        output::replace_any_unknown_types(self)
    }

    fn get_depths_vec(self: &mut Self) -> Vec<Vec<usize>> {
        output::get_depths_vec(self)
    }

    fn get_depths_flattened(self: &mut Self, depths: &Vec<Vec<usize>>) -> Vec<usize> {
        output::get_depths_flattened(depths)
    }

    fn get_output_for_element_index(
        self: &mut Self,
        element_index: usize,
        skip_in_case_handled_by_parent: bool,
    ) -> String {
        output::get_output_for_element_index(self, element_index, skip_in_case_handled_by_parent)
    }

    pub fn set_output(self: &mut Self) {
        output::set_output(self)
    }

    fn set_output_for_element_open(self: &mut Self, el_index: usize) {
        output::set_output_for_element_open(self, el_index)
    }

    fn set_output_for_element_close(self: &mut Self, el_index: usize) {
        output::set_output_for_element_close(self, el_index)
    }

    fn set_output_append(self: &mut Self, append_string: &str) {
        output::set_output_append(self, append_string)
    }

    fn set_output_append_no_indent(self: &mut Self, append_string: &str) {
        output::set_output_append_no_indent(self, append_string)
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
    let type_primitives = vec!["i64", "f64", "string"];
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
