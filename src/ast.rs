pub mod elements;
pub mod output;
pub mod parents;
use crate::ast::elements::{ElIndex, Element, ElementInfo, Elements};
use crate::ast::parents::vec_remove_head;
use crate::formatting::get_formatted_argname_argtype_pairs;
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

fn debug_flat_usize_array(arr: &Vec<usize>) -> String {
    let mut arr_debug = "[ ".to_string();
    for string in arr {
        arr_debug = format!("{}{}, ", arr_debug, string);
    }
    arr_debug = format!("{}]", arr_debug);
    arr_debug
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
        let element = self.elements[element_index].clone();
        //dbg!(&element.0);
        //dbg!(&element, self.parents); //            self.get_current_parent_ref_from_parents(),            self.get_current_parent_element()   );
        let skip = "".to_string();

        //skip children for certain parents who already parsed them
        if skip_in_case_handled_by_parent {
            match self.get_current_parent_element_from_element_children_search(element_index) {
                Some((ElementInfo::Assignment, _)) => return skip,
                Some((ElementInfo::FunctionCall(_, _), _)) => return skip,
                _ => (),
            }
        }

        match element.0 {
            ElementInfo::Root => "".to_string(),
            ElementInfo::CommentSingleLine(comment_string) => format!("{}", comment_string),
            ElementInfo::Int(val) => format!("{}", val),
            ElementInfo::Float(val) => format!("{}", val),
            ElementInfo::String(val) => format!("{}.to_string()", val),
            ElementInfo::Arg(name, _scope, _returntype) => format!("{}", name).to_string(),
            ElementInfo::Constant(name, _returntype) => format!("{}", name).to_string(),
            ElementInfo::ConstantRef(name, _typename, _reference) => {
                format!("{}", name)
            }
            ElementInfo::Assignment => {
                let mut returntype = "Undefined".to_string();
                let children = element.1;
                if children.len() < 1 {
                    format!("// let ?: ? = ? OUTPUT ERROR: Can't get constant for this assignment from : {:?}", children)
                } else {
                    let constant_index = children[0];
                    let constant_output = self.get_output_for_element_index(constant_index, false);
                    let constant = &self.elements[constant_index];
                    match &constant.0 {
                        ElementInfo::Constant(_, r) => {
                            returntype = r.clone();
                        }
                        _ => (),
                    }
                    format!("let {}: {} = ", constant_output, returntype)
                }
            }
            ElementInfo::InbuiltFunctionDef(name, _argnames, _argtypes, _returntype, _format) => {
                format!("fn {}() ->{{ /* stuff */ }}", name)
            }
            ElementInfo::InbuiltFunctionCall(name, _fndef_index, _returntype) => {
                //dbg!("InbuiltFunctionCall");
                if let Some(def) = self.get_inbuilt_function_by_name(&name) {
                    match def {
                        ElementInfo::InbuiltFunctionDef(_, argnames, _, _, format) => {
                            let children = element.1;
                            //dbg!(&argnames, &children);
                            let mut output = format;
                            //dbg!(&output);
                            for i in 0..argnames.len() {
                                let arg_var_num = format!("arg~{}", i + 1);
                                let arg_value_el_ref = children[i];
                                let arg_output =
                                    self.get_output_for_element_index(arg_value_el_ref, true);
                                output = output.replace(&arg_var_num, &arg_output);
                                //dbg!("---",&arg_var_num,arg_value_el_ref,&arg_output,&output);
                            }
                            if children.len() > 0 && children.len() == (argnames.len() + 1) {
                                let last_child = self.get_last_element();
                                match &last_child.0 {
                                    ElementInfo::Seol => {
                                        output = format!("{};\r\n", output);
                                        ()
                                    }
                                    _ => (),
                                }
                            }
                            return output;
                        }
                        _ => return "".to_string(),
                    }
                }
                "".to_string()
            }
            ElementInfo::FunctionDefWIP => "".to_string(),
            ElementInfo::FunctionDef(name, argnames, argtypes, returntype) => {
                let args = get_formatted_argname_argtype_pairs(&argnames, &argtypes);
                format!("fn {}({}) -> {} {{\r\n", name, args, returntype)
            }
            ElementInfo::FunctionCall(name, _) => {
                let arguments = element.1;
                let mut args = "".to_string();
                for i in 0..arguments.len() {
                    let arg_el_ref = arguments[i];
                    //let arg_el = self.elements[arg_el_ref];
                    let arg = self.get_output_for_element_index(arg_el_ref, false);
                    let mut borrow = "".to_string();
                    //dbg!("here", &name, &returntype, &arg_el);
                    if let Some(fndef_ref) = self.get_function_index_by_name(&name) {
                        let fndef = &self.elements[fndef_ref];
                        match &fndef.0 {
                            ElementInfo::FunctionDef(_, _, argtypes, _) => {
                                if argtypes.len() == arguments.len() {
                                    if argtypes[i].contains("&dyn Fn") {
                                        borrow = "&".to_string();
                                    }
                                }
                            }
                            _ => (),
                        }
                    }

                    let comma = if i == arguments.len() - 1 {
                        "".to_string()
                    } else {
                        ", ".to_string()
                    };
                    args = format!("{}{}{}{}", args, borrow, arg, comma);
                }
                format!("{}({})", name, args)
            }
            ElementInfo::Parens => {
                let children = &element.1;
                let mut output = "".to_string();
                for i in 0..children.len() {
                    let child_ref = children[i];
                    let child = self.get_output_for_element_index(child_ref, false);
                    output = format!("{}{}", output, child);
                }
                format!("({})", output)
            }
            ElementInfo::Eol => format!("\r\n"),
            ElementInfo::Seol => format!(";\r\n"),
            ElementInfo::Indent => self.get_indent(),
            ElementInfo::Type(name) => format!("{}", name),
            ElementInfo::Unused => "".to_string(),
        }
    }

    pub fn set_output(self: &mut Self) {
        //dbg!(&self);
        for _i in 0..10 {
            self.replace_any_unknown_types();
        }
        self.set_output_append("fn main() {\r\n");
        self.parents = vec![0];
        // the values of indent and outdent don't matter when outputting - only using parents.len()
        // values do matter when building the ast
        self.indent();

        let mut stack: Vec<usize> = self.elements[0].1.clone();
        while stack.len() > 0 {
            let current_item = stack[0];
            // remove current item from stack
            stack = vec_remove_head(&stack);
            // if it is an outdent marker, outdent level!
            if current_item == 0 {
                self.outdent();
                // push current end tag to output
                let end_tag = stack[0];

                self.set_output_for_element_close(end_tag);
                // removed the outdent marker earlier, now remove the end tag indicator
                stack = vec_remove_head(&stack);
                // if the end_tag was the end of a func_def we don't want to display the trailing semicolon
                // since it needs to be treated as the return statement, so remove it if there is one
            } else {
                // push current to output
                self.set_output_for_element_open(current_item);
                // if current item has children...
                let mut current_item_children = self.elements[current_item].1.clone();

                // don't render children of certain elements - they are rendered separately
                let el = &self.elements[current_item];
                match el.0 {
                    ElementInfo::InbuiltFunctionCall(_, _, _) => current_item_children = vec![],
                    _ => (),
                }

                if current_item < self.elements.len() && current_item_children.len() > 0 {
                    // prepend with current item end tag indicator - so we know to close it at after the outdent
                    stack.splice(0..0, vec![current_item]);
                    // prepend with 0 (marker for outdent)
                    stack.splice(0..0, vec![0]);
                    // prepend with children
                    stack.splice(0..0, self.elements[current_item].1.clone());
                    // and increase indent
                    self.indent();
                }
            }
        }
        self.outdent();
        self.set_output_append("}\r\n");
        //println!("AST_OUTPUT\r\n{:?}\r\n{:?}", self.elements, self.output);
    }

    fn set_output_for_element_open(self: &mut Self, el_index: usize) {
        if el_index < self.elements.len() {
            let element = self.elements[el_index].clone();
            let element_string = self.get_output_for_element_index(el_index, true);
            match element.0 {
                ElementInfo::Eol => self.set_output_append_no_indent(&element_string),
                ElementInfo::Seol => self.set_output_append_no_indent(&element_string),
                _ => self.set_output_append(&element_string),
            }
        }
    }

    fn set_output_for_element_close(self: &mut Self, el_index: usize) {
        if el_index < self.elements.len() {
            let element = &self.elements[el_index];
            let element_string = match element.0 {
                ElementInfo::FunctionDef(_, _, _, _) => {
                    format!("\r\n{}}}\r\n", self.get_indent())
                }
                _ => "".to_string(),
            };
            self.set_output_append(&element_string);
        }
    }

    fn set_output_append(self: &mut Self, append_string: &str) {
        self.output = format!("{}{}", self.output, append_string);
    }

    fn set_output_append_no_indent(self: &mut Self, append_string: &str) {
        self.output = format!("{}{}", self.output, append_string);
    }
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

#[cfg(test)]
mod tests {

    // cargo watch -x "test test_get_depths_vec"
    // cargo watch -x "test test_get_depths_vec -- --show-output"
    // cargo test test_get_depths_vec -- --show-output

    use super::*;

    #[test]
    fn test_get_depths_vec() {
        //1 el
        let mut ast1 = Ast::new();
        let mut n = ast1.elements.len();
        let el1: Element = (ElementInfo::Int("1".to_string()), vec![]);
        ast1.append(el1);
        assert_eq!(ast1.get_depths_vec(), vec![[n]]);

        //3 el under root
        let mut ast2 = Ast::new();
        n = ast2.elements.len();
        let el21: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el22: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el23: Element = (ElementInfo::Int("1".to_string()), vec![]);
        ast2.append(el21);
        ast2.append(el22);
        ast2.append(el23);
        assert_eq!(ast2.get_depths_vec(), vec![[n, n + 1, n + 2]]);

        //1 el under with 2 children, under root
        let mut ast3 = Ast::new();
        n = ast3.elements.len();
        let el31: Element = (
            ElementInfo::InbuiltFunctionCall("+".to_string(), 1, "i64|f64".to_string()),
            vec![],
        );
        let el32: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el33: Element = (ElementInfo::Int("1".to_string()), vec![]);
        ast3.append(el31);
        ast3.indent();
        ast3.append(el32);
        ast3.append(el33);
        assert_eq!(ast3.get_depths_vec(), vec![vec![n], vec![n + 2, n + 1]]);

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
        let mut ast4 = Ast::new();
        n = ast4.elements.len();
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
        ast4.append(el41);
        ast4.append(el42);
        ast4.append(el43);
        ast4.indent();
        ast4.append(el44);
        ast4.append(el45);
        ast4.indent();
        ast4.append(el46);
        ast4.append(el47);
        ast4.indent();
        ast4.append(el48);
        ast4.append(el49);
        ast4.outdent();
        ast4.outdent();
        ast4.outdent();
        ast4.append(el410);
        assert_eq!(
            ast4.get_depths_vec(),
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
        let mut ast = Ast::new();
        let mut input = vec![vec![0]];
        assert_eq!(ast.get_depths_flattened(&input), vec![0]);

        input = vec![vec![1, 2, 3]];
        assert_eq!(ast.get_depths_flattened(&input), vec![1, 2, 3]);

        input = vec![vec![1], vec![2, 3]];
        assert_eq!(ast.get_depths_flattened(&input), vec![2, 3, 1]);

        input = vec![vec![1, 2, 3, 10], vec![4, 5], vec![6, 7], vec![8, 9]];
        assert_eq!(
            ast.get_depths_flattened(&input),
            vec![8, 9, 6, 7, 4, 5, 1, 2, 3, 10]
        );
    }
}
