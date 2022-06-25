#[derive(Clone, Debug)]
pub struct Ast {
    //first element is always root. Real elements start at index 1
    pub elements: Vec<Element>,
    pub output: String,
    parents: Vec<ElIndex>,
}
#[derive(Clone, Debug)]
pub enum ElementInfo {
    Root,
    CommentSingleLine(Value),
    Int(Value),
    Float(Value),
    String(Value),
    Constant(Name, ReturnType),
    ConstantRef(Name, ReturnType, RefName),
    FunctionCall(Name, ReturnType),
    Arithmetic(Name, ReturnType, Value, Value),
    Eol,
    Seol,
    Indent,
}
type Value = String;
type ElIndex = usize;
type ReturnType = String;
type Name = String;
type RefName = String;
// no need to track parents in Element
// should only ever be one per Element so can search for it each time
// to save double handling parent/child refs in two places
pub type Element = (ElementInfo, ElementChildren);
pub type ElementChildren = Vec<ElIndex>;

impl Ast {
    pub fn new() -> Ast {
        Ast {
            elements: vec![(ElementInfo::Root, vec![])],
            output: "".to_string(),
            parents: vec![0], // get current indent from length of parents
        }
    }

    pub fn append(self: &mut Self, element: Element) -> usize {
        // add element to list, and add to list of children of current parent where 0 = root
        self.elements.push(element);
        let new_items_index = self.elements.len() - 1;
        let current_parent = self.parents[self.parents.len() - 1];
        self.elements[current_parent].1.push(new_items_index);
        new_items_index
    }

    pub fn append_as_ref(self: &mut Self, element: Element) -> usize {
        // add element to list only, don't add as child
        self.elements.push(element);
        let new_items_index = self.elements.len() - 1;
        new_items_index
    }

    pub fn set_output(self: &mut Self) {
        self.set_output_append("fn main() {\r\n");
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
            } else {
                // push current to output
                self.set_open_output_for_element(current_item);
                // if current item has children...
                let current_item_children = self.elements[current_item].1.clone();
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
            let element_string = self.get_output_for_element(element.clone());
            match element.0 {
                ElementInfo::Eol => self.set_output_append_no_indent(&element_string),
                ElementInfo::Seol => self.set_output_append_no_indent(&element_string),
                _ => self.set_output_append(&element_string),
            }
        }
    }

    fn get_output_for_element(self: &mut Self, element: Element) -> String {
        match element.0.clone() {
            ElementInfo::Root => "".to_string(),
            ElementInfo::CommentSingleLine(comment_string) => format!("{}", comment_string),
            ElementInfo::Int(val) => format!("{}", val),
            ElementInfo::Float(val) => format!("{}", val),
            ElementInfo::String(val) => format!("{}.to_string()", val),
            ElementInfo::Constant(name, returntype) => {
                //let children = element.1.clone();
                //let expression = self.get_single_line_expression_from_children(children);
                //dbg!(&expression);
                // shouldn't need this, if children expressions output themselves correctly
                /*
                if returntype == "String".to_string() {
                    format!(
                        "let {}: {} = ({}).to_string()",
                        name, returntype, expression
                    )
                    .to_string()
                } else {
                */
                format!("let {}: {} = ", name, returntype).to_string()
                //}
            }
            ElementInfo::ConstantRef(name, typename, reference) => {
                format!("let {}: {} = {}", name, typename, reference)
            }
            ElementInfo::FunctionCall(name, _returntype) => {
                format!("{}()", name)
            }
            ElementInfo::Arithmetic(name, _typename, val1, val2) => {
                format!("{} {} {}", val1, name, val2)
            }
            ElementInfo::Eol => format!("\r\n"),
            ElementInfo::Seol => format!(";\r\n"),
            ElementInfo::Indent => self.get_indent(),
        }
    }

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

    fn set_close_output_for_element(self: &mut Self, el_index: usize) {
        if el_index < self.elements.len() {
            let element = self.elements[el_index].clone();
            let element_string = match element.0 {
                _ => "",
            };
            self.set_output_append(element_string);
        }
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
        " ".repeat(4 * (self.parents.len() - 1))
    }

    pub fn indent(self: &mut Self) {
        self.parents.push(self.elements.len() - 1);
    }

    pub fn outdent(self: &mut Self) {
        self.parents = if self.parents.len() < 2 {
            vec![0]
        } else {
            vec_remove_tail(self.parents.clone())
        };
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
}

fn vec_remove_head(stack: Vec<usize>) -> Vec<usize> {
    if stack.len() == 1 {
        vec![]
    } else {
        stack[1..].to_vec()
    }
}

fn vec_remove_tail(stack: Vec<usize>) -> Vec<usize> {
    if stack.len() == 1 {
        vec![]
    } else {
        stack[..stack.len() - 1].to_vec()
    }
}
