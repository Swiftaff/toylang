use std::fmt;

#[derive(Clone)]
pub struct Ast {
    //first element is always root. Real elements start at index 1
    pub elements: Vec<Element>,
    pub output: String,
    //note: parents are only used for building, ignored output.
    //becuse of that, split outputting to be less confusing?
    pub parents: Vec<ElIndex>,
}

impl fmt::Debug for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut el_debug = "".to_string();
        let els = self.elements.clone();
        let parents_debug = debug_flat_usize_array(&self.parents);
        for el in 0..els.len() {
            let children_debug = debug_flat_usize_array(&els[el].1);

            let elinfo_debug = match els[el].0.clone() {
                ElementInfo::Root => format!("Root: {}", children_debug),
                ElementInfo::CommentSingleLine(comment) => {
                    format!("Comment: {} {}", comment, children_debug)
                }
                ElementInfo::Int(int) => format!("Int: {} {}", int, children_debug),
                ElementInfo::Float(float) => format!("Float: {} {}", float, children_debug),
                ElementInfo::String(string) => format!("String: {} {}", string, children_debug),
                ElementInfo::Constant(name, returntype) => {
                    format!("Constant: {} ({}) {}", name, returntype, children_debug)
                }
                ElementInfo::ConstantRef(name, returntype, refname) => {
                    format!(
                        "ConstantRef: {} ({}) for \"{}\" {}",
                        name, returntype, refname, children_debug
                    )
                }
                ElementInfo::Assignment(returntype) => {
                    format!("Assignment: ({}) {}", returntype, children_debug)
                }
                ElementInfo::InbuiltFunctionDef(
                    name,
                    _argnames,
                    _argtypes,
                    returntype,
                    _format,
                ) => {
                    format!(
                        "InbuiltFunctionDef: \"{}\" ({}) {}",
                        name, returntype, children_debug
                    )
                }
                ElementInfo::InbuiltFunctionCall(name, _, returntype) => {
                    format!(
                        "InbuiltFunctionCall: {} ({}) {}",
                        name, returntype, children_debug
                    )
                }
                ElementInfo::FunctionDef(name, argnames, argtypes, returntype) => {
                    let args = get_formatted_argname_argtype_pairs(argnames, argtypes);
                    format!(
                        "FunctionDef: {} ({}) -> ({}) {}",
                        name, args, returntype, children_debug
                    )
                }
                ElementInfo::FunctionCall(name) => {
                    format!("FunctionCall: {} {}", name, children_debug)
                }
                ElementInfo::Eol => "Eol".to_string(),
                ElementInfo::Seol => "Seol".to_string(),
                ElementInfo::Indent => "Indent".to_string(),
                ElementInfo::Type(name) => {
                    format!("Type: {} {}", name, children_debug)
                }
                ElementInfo::Unused => "Unused".to_string(),
            };
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

#[derive(Clone, Debug)]
pub enum ElementInfo {
    Root,
    CommentSingleLine(Value),               //no children
    Int(Value),                             //no children
    Float(Value),                           //no children
    String(Value),                          //no children
    Constant(Name, ReturnType),             //1 child = value
    ConstantRef(Name, ReturnType, RefName), //no children
    Assignment(ReturnType),                 //2 children. constant, value
    InbuiltFunctionDef(Name, ArgNames, ArgTypes, ReturnType, Format), //no children
    InbuiltFunctionCall(Name, ElIndex, ReturnType), //fndef argnames.len() children
    FunctionDef(Name, ArgNames, ArgTypes, ReturnType), //no children
    FunctionCall(Name),                     //fndef argnames.len() children
    Type(Name),                             // no children
    Eol,
    Seol,
    Indent,
    Unused,
}
type Value = String;
type ElIndex = usize;
type ReturnType = String;
type Name = String;
type RefName = String;
type ArgNames = Vec<String>;
type ArgTypes = Vec<String>;
type Format = String;
// no need to track parents in Element
// should only ever be one per Element so can search for it each time
// to save double handling parent/child refs in two places
pub type Element = (ElementInfo, ElementChildren);
pub type ElementChildren = Vec<ElIndex>;

impl Ast {
    pub fn new() -> Ast {
        let type_primitives = vec!["i64", "f64", "string"];
        let type_closure = |prim: &str| (ElementInfo::Type(prim.to_string()), vec![]);
        let types: Vec<Element> = type_primitives
            .clone()
            .into_iter()
            .map(type_closure)
            .collect();
        let arithmetic_primitives = vec!["+", "-", "*", "/", "%"];
        let arithmetic_closure = |prim: &str| {
            (
                ElementInfo::InbuiltFunctionDef(
                    prim.to_string(),
                    vec!["arg1".to_string(), "arg2".to_string()],
                    vec!["i64|f64".to_string(), "i64|f64".to_string()],
                    "i64|f64".to_string(),
                    format!("arg1 {} arg2", prim).to_string(),
                ),
                vec![],
            )
        };
        let arithmetic_operators: Vec<Element> = arithmetic_primitives
            .clone()
            .into_iter()
            .map(arithmetic_closure)
            .collect();
        let root = vec![(ElementInfo::Root, vec![])];
        let elements: Vec<Element> = vec![]
            .iter()
            .chain(&root)
            .chain(&arithmetic_operators)
            .chain(&types)
            //.chain(&arithmetic_operators_f64)
            .map(|x| x.clone())
            .collect();
        Ast {
            elements,
            output: "".to_string(),
            parents: vec![0], // get current indent from length of parents
        }
    }

    pub fn append(self: &mut Self, element: Element) -> usize {
        // add element to list, and add to list of children of current parent where 0 = root
        self.elements.push(element);
        let new_items_index = self.elements.len() - 1;
        let current_parent_ref = self.get_current_parent_ref_from_parents();
        self.elements[current_parent_ref].1.push(new_items_index);
        new_items_index
    }

    pub fn append_as_ref(self: &mut Self, element: Element) -> usize {
        // add element to list only, don't add as child
        self.elements.push(element);
        let new_items_index = self.elements.len() - 1;
        new_items_index
    }

    fn get_depths_vec(self: &mut Self) -> Vec<Vec<usize>> {
        let mut tracked_parents: Vec<usize> = vec![0];
        let mut children: Vec<usize> = self.elements[0].1.clone();
        let mut depths: Vec<Vec<usize>> = vec![children.clone()];
        //println!("testy");
        loop {
            //println!("{:?}", tracked_parents.clone());
            let mut next_level = vec![];
            let current_level = depths[depths.len() - 1].clone();
            for el_ref in current_level {
                let el = self.elements[el_ref].clone();
                children = el.1;
                next_level = vec![]
                    .iter()
                    .chain(&next_level.clone())
                    .chain(&children)
                    .map(|x| x.clone())
                    .collect();
                tracked_parents.push(el_ref);
            }
            if next_level.len() > 0 {
                depths.push(next_level.clone());
            } else {
                break;
            }
        }
        depths
    }

    fn get_depths_flattened(self: &mut Self, depths: &Vec<Vec<usize>>) -> Vec<usize> {
        // flattens depths from bottom (deepest) to top
        // this is so that it can be used to traverse elements in the correct order
        // to allow correcting the types from the deepest elements first
        // since higher levels may rely on type of deeper elements.
        // e.g. a higher level "+" fn with type "i64|f64" will need to be disambiguated
        // to either i64 or f64 based on the type of it's 2 child args
        // so the two child args are fixed first (if unknown)
        //then "+" fn can be determined safely
        let mut output = vec![];
        for i in (0..depths.len()).rev() {
            let level = &depths[i];
            output = vec![]
                .iter()
                .chain(&output.clone())
                .chain(level)
                .map(|x| x.clone())
                .collect();
        }
        output
    }

    fn fix_any_unknown_types(self: &mut Self) {
        let depths = self.get_depths_vec();
        let depths_flattened = self.get_depths_flattened(&depths);

        for el_index in depths_flattened {
            let el = self.elements[el_index].clone();
            let el_info = el.clone().0;

            let el_type = self.get_elementinfo_type(el_info.clone());
            if el_type == "Undefined".to_string() {
                match el_info {
                    ElementInfo::Assignment(_) => {
                        let infered_type = self.get_infered_type_of_assignment_element(el);
                        self.elements[el_index].0 = ElementInfo::Assignment(infered_type);
                    }
                    ElementInfo::Constant(name, _) => {
                        let el_option =
                            self.get_current_parent_element_from_element_children_search(el_index);
                        match el_option {
                            Some(el) => {
                                let infered_type = self.get_infered_type_of_assignment_element(el);
                                self.elements[el_index].0 =
                                    ElementInfo::Constant(name, infered_type)
                            }
                            _ => (),
                        }
                    }
                    ElementInfo::InbuiltFunctionCall(name, fndef_index, _returntype) => {
                        let infered_type =
                            self.get_infered_type_of_functioncall_element(el, fndef_index);
                        //dbg!("3", infered_type.clone());
                        self.elements[el_index].0 =
                            ElementInfo::InbuiltFunctionCall(name, fndef_index, infered_type);
                    }
                    _el => {
                        //dbg!(el);
                        ()
                    }
                }
            }
        }
        //dbg!(self.elements.clone());
    }

    fn get_infered_type_of_assignment_element(self: &mut Self, el: Element) -> String {
        let el_children = el.1;
        let mut infered_type = "Undefined".to_string();
        if el_children.len() > 1 {
            let second_child_ref = el_children[1];
            let second_child = self.elements[second_child_ref].clone();
            infered_type = self.get_elementinfo_type(second_child.0);
        }
        infered_type
    }

    fn get_infered_type_of_functioncall_element(
        self: &mut Self,
        func_call_el: Element,
        funcdef_el_index: usize,
    ) -> String {
        //dbg!("1");
        let el_children = func_call_el.1;
        let el = self.elements[funcdef_el_index].clone();
        let elinfo = el.0;
        let mut infered_type = "Undefined".to_string();
        match elinfo {
            ElementInfo::InbuiltFunctionDef(_, _argnames, argtypes, returntype, _) => {
                //TODO could check all args match here for parser error
                //dbg!("2", returntype.clone());
                if returntype.contains("|") {
                    //dbg!("2.5", el_children.clone());
                    if el_children.len() > 0 && argtypes.len() <= el_children.len() {
                        for _argtype in argtypes {
                            let first_child_ref = el_children[0];
                            let first_child = self.elements[first_child_ref].clone();
                            infered_type = self.get_elementinfo_type(first_child.0);
                            //dbg!("2.6", infered_type.clone());
                        }
                    }
                } else {
                    infered_type = returntype;
                }
            }
            _ => (),
        }

        infered_type
    }

    pub fn set_output(self: &mut Self) {
        //dbg!(&self);
        self.fix_any_unknown_types();
        self.set_output_append("fn main() {\r\n");
        self.parents = vec![0];
        // the values of indent and outdent don't matter when outputting - only using parents.len()
        // values do matter when building the ast
        self.indent();

        let mut stack: Vec<usize> = self.elements[0].1.clone();
        while stack.len() > 0 {
            let current_item = stack[0];
            // remove current item from stack
            stack = vec_remove_head(stack);
            // if it is an outdent marker, outdent level!
            if current_item == 0 {
                self.outdent();
                // push current end tag to output
                let end_tag = stack[0];

                self.set_close_output_for_element(end_tag);
                // removed the outdent marker earlier, now remove the end tag indicator
                stack = vec_remove_head(stack);
                // if the end_tag was the end of a func_def we don't want to display the trailing semicolon
                // since it needs to be treated as the return statement, so remove it if there is one
            } else {
                // push current to output
                self.set_open_output_for_element(current_item);
                // if current item has children...
                let mut current_item_children = self.elements[current_item].1.clone();

                // don't render children of certain elements - they are rendered separately
                let el = self.elements[current_item].clone();
                match el.0 {
                    ElementInfo::InbuiltFunctionCall(_, _, _) => current_item_children = vec![],
                    _ => (),
                }

                //let does_indent = self.elements[current_item].2;
                if current_item < self.elements.len() && current_item_children.len() > 0 {
                    //if does_indent {
                    // prepend with current item end tag indicator - so we know to close it at after the outdent
                    stack.splice(0..0, vec![current_item]);
                    // prepend with 0 (marker for outdent)
                    stack.splice(0..0, vec![0]);
                    //}
                    // prepend with children
                    stack.splice(0..0, self.elements[current_item].1.clone());
                    // and increase indent
                    //if does_indent {
                    self.indent();
                    //}
                }
            }
        }
        self.outdent();
        self.set_output_append("}\r\n");
        //println!("AST_OUTPUT\r\n{:?}\r\n{:?}", self.elements, self.output);
    }

    fn set_open_output_for_element(self: &mut Self, el_index: usize) {
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

    pub fn get_current_parent_ref_from_parents(self: &mut Self) -> usize {
        let last = self.parents.len() - 1;
        self.parents[last]
    }

    pub fn get_current_parent_element_from_element_children_search(
        self: &mut Self,
        child_ref: usize,
    ) -> Option<Element> {
        let index_option = self.get_current_parent_ref_from_element_children_search(child_ref);
        match index_option {
            Some(index) => Some(self.elements[index].clone()),
            _ => None,
        }
    }

    pub fn get_current_parent_ref_from_element_children_search(
        self: &mut Self,
        child_ref: usize,
    ) -> Option<usize> {
        let index_option = self
            .elements
            .iter()
            .position(|(_, children)| children.contains(&child_ref));
        match index_option {
            Some(index) => Some(index),
            _ => None,
        }
    }

    fn get_output_for_element_index(
        self: &mut Self,
        element_index: usize,
        skip_in_case_handled_by_parent: bool,
    ) -> String {
        let element = self.elements[element_index].clone();
        //dbg!(element.0.clone());
        //dbg!(element.clone(), self.parents.clone()); //            self.get_current_parent_ref_from_parents(),            self.get_current_parent_element()   );
        let skip = "".to_string();

        //skip children for certain parents who already parsed them
        if skip_in_case_handled_by_parent {
            match self.get_current_parent_element_from_element_children_search(element_index) {
                Some((ElementInfo::Assignment(_), _)) => return skip,
                _ => (),
            }
        }

        match element.0.clone() {
            ElementInfo::Root => "".to_string(),
            ElementInfo::CommentSingleLine(comment_string) => format!("{}", comment_string),
            ElementInfo::Int(val) => format!("{}", val),
            ElementInfo::Float(val) => format!("{}", val),
            ElementInfo::String(val) => format!("{}.to_string()", val),
            ElementInfo::Constant(name, _returntype) => format!("{}", name).to_string(),
            ElementInfo::ConstantRef(name, _typename, _reference) => {
                format!("{}", name)
            }
            ElementInfo::Assignment(typename) => {
                let children = element.1.clone();
                if children.len() < 2 {
                    format!("// let ?: {} = ? OUTPUT ERROR: Can't get constant or value for this assignment from : {:?}", typename, children)
                } else {
                    let constant_index = children[0];
                    let constant_output = self.get_output_for_element_index(constant_index, false);
                    let mut output = format!("let {}: {} = ", constant_output, typename);
                    if children.len() > 1 {
                        for child_index in 1..children.len() {
                            let child_ref = children[child_index];
                            let child_output = self.get_output_for_element_index(child_ref, false);
                            output = format!("{}{}", output, child_output);
                        }
                    }
                    output
                }
            }

            /*
            ElementInfo::Constant(name, returntype) => {
                format!("let {}: {} = ", name, returntype).to_string()
            }
            ElementInfo::ConstantRef(name, typename, reference) => {
                format!("let {}: {} = {}", name, typename, reference)
            }
            */
            ElementInfo::InbuiltFunctionDef(name, _argnames, _argtypes, _returntype, _format) => {
                format!("fn {}() ->{{ /* stuff */ }}", name)
            }
            ElementInfo::InbuiltFunctionCall(name, _fndef_index, _returntype) => {
                //dbg!("InbuiltFunctionCall");
                let def_option = self.get_inbuilt_function_by_name(&name);
                match def_option {
                    Some(def) => match def {
                        ElementInfo::InbuiltFunctionDef(_, argnames, _, _, format) => {
                            let children = element.1.clone();
                            //dbg!(&argnames, &children);
                            //if children.len() == argnames.len() {
                            let mut output = format;
                            //dbg!(&output);
                            for i in 0..argnames.len() {
                                let arg_var_num = format!("arg{}", i + 1);
                                let arg_value_el_ref = children[i];
                                let arg_output =
                                    self.get_output_for_element_index(arg_value_el_ref, true);
                                //dbg!(&arg_var_num, arg_value_el_ref, arg_value_el, &arg_output);
                                output = output.replace(&arg_var_num, &arg_output);
                            }
                            if children.len() > 0 && children.len() == (argnames.len() + 1) {
                                let last_child =
                                    self.elements[children[children.len() - 1]].clone();
                                match last_child.0 {
                                    ElementInfo::Seol => {
                                        output = format!("{};\r\n", output);
                                        ()
                                    }
                                    _ => (),
                                }
                            }

                            return output;
                            //}
                            //return "".to_string();
                        }
                        _ => return "".to_string(),
                    },
                    None => return "".to_string(),
                }
            }
            ElementInfo::FunctionDef(name, argnames, argtypes, returntype) => {
                let args = get_formatted_argname_argtype_pairs(argnames, argtypes);
                format!("fn {}({}) -> {} {{\r\n", name, args, returntype)
            }
            ElementInfo::FunctionCall(name) => {
                format!("{}()", name)
            }
            ElementInfo::Eol => format!("\r\n"),
            ElementInfo::Seol => format!(";\r\n"),
            ElementInfo::Indent => self.get_indent(),
            ElementInfo::Type(name) => format!("{}", name),
            ElementInfo::Unused => "".to_string(),
        }
    }

    pub fn get_elementinfo_type(self: &Self, elementinfo: ElementInfo) -> String {
        match elementinfo {
            ElementInfo::Int(_) => "i64".to_string(),
            ElementInfo::Float(_) => "f64".to_string(),
            ElementInfo::String(_) => "String".to_string(),
            ElementInfo::Assignment(returntype) => returntype,
            ElementInfo::Constant(_, returntype) => returntype,
            ElementInfo::ConstantRef(_, returntype, _) => returntype,
            ElementInfo::InbuiltFunctionCall(_, _fndef_index, returntype) => returntype,
            _ => "Undefined".to_string(),
        }
    }

    /*
    fn get_single_line_expression_from_children(self: &mut Self, children: Vec<usize>) -> String {
        let mut expression = "".to_string();
        for i in 0..children.len() {
            let child_ref = children[i];
            let child_element = self.elements[child_ref].clone();
            let child_output = self.get_output_for_element(child_element);
            expression = format!("{}{}", expression, child_output);
        }
        expression
    }
    */

    fn set_close_output_for_element(self: &mut Self, el_index: usize) {
        //let next_tag_is_semicolon = false;
        if el_index < self.elements.len() {
            let element = self.elements[el_index].clone();
            let element_string = match element.0 {
                ElementInfo::FunctionDef(_, _, _, _) => {
                    //if self.elements.len() > el_index + 1 {
                    //    let next_element = self.elements[el_index + 1].clone();
                    //    dbg!(next_element);
                    //    //next_tag_is_semicolon = true;
                    //}

                    format!("\r\n{}}}", self.get_indent())
                }
                _ => "".to_string(),
            };
            self.set_output_append(&element_string);
        }
        //next_tag_is_semicolon
    }

    fn set_output_append(self: &mut Self, append_string: &str) {
        //let indent = if does_indent {
        //    self.get_indent()
        //} else {
        //    "".to_string()
        //};
        self.output = format!("{}{}", self.output, append_string);
    }

    fn set_output_append_no_indent(self: &mut Self, append_string: &str) {
        self.output = format!("{}{}", self.output, append_string);
    }

    fn get_indent(self: &Self) -> String {
        //format!(
        //    "{:?}{}",
        //    self.parents,
        //    " ".repeat(4 * (self.parents.len() - 1))
        //)
        " ".repeat(4 * (self.parents.len() - 1))
    }

    pub fn indent(self: &mut Self) {
        self.parents.push(self.elements.len() - 1);
    }

    pub fn indent_this(self: &mut Self, index: usize) {
        self.parents.push(index);
    }

    pub fn outdent(self: &mut Self) {
        self.parents = if self.parents.len() < 2 {
            vec![0]
        } else {
            vec_remove_tail(self.parents.clone())
        };
    }

    pub fn get_exists_element_by_name(self: &Self, name: &String) -> bool {
        let constant_option = self.get_constant_index_by_name(name);
        let inbuiltfn_option = self.get_inbuilt_function_index_by_name(name);
        let fn_option = self.get_function_index_by_name(name);
        let type_option = self.get_inbuilt_type_index_by_name(name);
        match (constant_option, inbuiltfn_option, fn_option, type_option) {
            (Some(_), Some(_), Some(_), Some(_)) => true,
            _ => false,
        }
    }

    pub fn get_inbuilt_type_index_by_name(self: &Self, name: &String) -> Option<usize> {
        self.elements.iter().position(|(elinfo, _)| match elinfo {
            ElementInfo::Type(n) => n == name,
            _ => false,
        })
    }

    pub fn get_constant_index_by_name(self: &Self, name: &String) -> Option<usize> {
        self.elements.iter().position(|(elinfo, _)| match elinfo {
            ElementInfo::Constant(n, _t) => n == name,
            ElementInfo::ConstantRef(n, _t, _refname) => n == name,
            _ => false,
        })
    }

    pub fn get_constant_by_name(self: &Self, name: &String) -> Option<ElementInfo> {
        let option_index = self.get_constant_index_by_name(name);
        match option_index {
            Some(index) => Some(self.elements[index].0.clone()),
            None => None,
        }
    }

    pub fn get_function_index_by_name(self: &Self, name: &String) -> Option<usize> {
        self.elements.iter().position(|(elinfo, _)| match &elinfo {
            ElementInfo::FunctionDef(n, _, _, _) => n == name,
            _ => false,
        })
    }

    pub fn get_inbuilt_function_index_by_name(self: &Self, name: &String) -> Option<usize> {
        self.elements.iter().position(|(elinfo, _)| match &elinfo {
            ElementInfo::InbuiltFunctionDef(n, _, _, _, _) => n == name,
            _ => false,
        })
    }

    pub fn get_inbuilt_function_by_name(self: &Self, name: &String) -> Option<ElementInfo> {
        let option_index = self.get_inbuilt_function_index_by_name(name);
        match option_index {
            Some(index) => Some(self.elements[index].0.clone()),
            None => None,
        }
    }

    pub fn get_inbuilt_function_index_by_name_and_returntype(
        self: &Self,
        name: &String,
        returntype: &String,
    ) -> Option<usize> {
        //dbg!(returntype);
        self.elements.iter().position(|(elinfo, _)| match &elinfo {
            ElementInfo::InbuiltFunctionDef(n, _, _, r, _) => {
                //dbg!("here", n, r, name, returntype);
                n == name && (r.contains(returntype) || returntype.contains(r))
            }
            _ => false,
        })
    }

    pub fn get_inbuilt_function_by_name_and_returntype(
        self: &Self,
        name: &String,
        returntype: &String,
    ) -> Option<ElementInfo> {
        //dbg!(returntype);
        let option_index = self.get_inbuilt_function_index_by_name_and_returntype(name, returntype);
        match option_index {
            Some(index) => Some(self.elements[index].0.clone()),
            None => None,
        }
    }
}

fn vec_remove_head(stack: Vec<usize>) -> Vec<usize> {
    if stack.len() == 1 {
        vec![]
    } else {
        stack[1..].to_vec()
    }
}

pub fn vec_remove_tail(stack: Vec<usize>) -> Vec<usize> {
    if stack.len() == 1 {
        vec![]
    } else {
        stack[..stack.len() - 1].to_vec()
    }
}

fn get_formatted_argname_argtype_pairs(argnames: Vec<String>, argtypes: Vec<String>) -> String {
    let mut args = "".to_string();
    for a in 0..argnames.len() {
        args = format!("{}{}: {}", args, argnames[a], argtypes[a]);
    }
    args
}

#[cfg(test)]
mod tests {

    // cargo watch -x "test test_get_depths_vec"
    // cargo watch -x "test test_get_depths_vec -- --show-output"
    // cargo test test_get_depths_vec -- --show-output

    use super::*;

    #[test]
    fn test_get_depths_vec() {
        //get_depths_vec

        //1 el
        let mut ast1 = Ast::new();
        let mut n = ast1.elements.len();
        let el1: Element = (ElementInfo::Int("1".to_string()), vec![]);
        ast1.append(el1.clone());
        assert_eq!(ast1.get_depths_vec(), vec![[n]]);

        //3 el under root
        let mut ast2 = Ast::new();
        n = ast2.elements.len();
        let el21: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el22: Element = (ElementInfo::Int("1".to_string()), vec![]);
        let el23: Element = (ElementInfo::Int("1".to_string()), vec![]);
        ast2.append(el21.clone());
        ast2.append(el22.clone());
        ast2.append(el23.clone());
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
        ast3.append(el31.clone());
        ast3.indent();
        ast3.append(el32.clone());
        ast3.append(el33.clone());
        assert_eq!(ast3.get_depths_vec(), vec![vec![n], vec![n + 1, n + 2]]);

        //typical nested tree         this flat ast
        //0 (root)                    |_(0,[1,2,3,8]) root
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
        ast4.append(el41.clone());
        ast4.append(el42.clone());
        ast4.append(el43.clone());
        ast4.indent();
        ast4.append(el44.clone());
        ast4.append(el45.clone());
        ast4.indent();
        ast4.append(el46.clone());
        ast4.append(el47.clone());
        ast4.indent();
        ast4.append(el48.clone());
        ast4.append(el49.clone());
        ast4.outdent();
        ast4.outdent();
        ast4.outdent();
        ast4.append(el410.clone());
        assert_eq!(
            ast4.get_depths_vec(),
            vec![
                vec![n, n + 1, n + 2, n + 9],
                vec![n + 3, n + 4],
                vec![n + 5, n + 6],
                vec![n + 7, n + 8]
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
