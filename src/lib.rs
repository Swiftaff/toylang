// TODO make most function arguments refs
mod ast;
mod file;
use ast::{Ast, ElementInfo};
use file::File;
use std::error::Error;

type Tokens = Vec<String>;
type ErrorStack = Vec<String>;
//type FullOrValidationError = bool;

#[derive(Clone, Debug)]
pub struct Config {
    pub file: File,
    pub lines_of_chars: Vec<Vec<char>>,
    pub lines_of_tokens: Vec<Tokens>,
    pub output: String,
    pub current_line: usize,
    pub current_line_token: usize,
    pub error_stack: ErrorStack,
    pub ast: Ast,
}

struct Errors {
    comment_single_line: &'static str,
    string: &'static str,
    assign: &'static str,
    int: &'static str,
    int_out_of_bounds: &'static str,
    int_negative: &'static str,
    float: &'static str,
    //typeerror: &'static str,
    funcdef_args: &'static str,
    funcdef_argtypes_first: &'static str,
    //no_valid_assignment: &'static str,
    //no_valid_integer_arithmetic: &'static str,
    //no_valid_expression: &'static str,
    //constants_are_immutable: &'static str
}

const ERRORS: Errors = Errors {
    comment_single_line: "Invalid single line comment: Must begin with two forward slashes '//'",
    string: "Invalid string found: Must be enclosed in quote marks \"\"",
    assign: "Invalid assignment: There are characters directly after '='. It must be followed by a space",
    int: "Invalid int: there are characters after the first digit. Must only contain digits",
    int_out_of_bounds: "Invalid int: is out of bounds. Must be within the value of -9223372036854775808 to 9223372036854775807",
    int_negative:"Invalid negative int or float: Must follow a negative sign '-' with a digit",
    float: "Invalid float",
    //typeerror: "Invalid type",
    funcdef_args: "Invalid Functional Definition - wrong number of argument types: should be 1 type for each arg, plus a return type.",
    funcdef_argtypes_first:"Invalid Functional Definition - argument types should come before argument names.",
    //no_valid_assignment: "No valid assignment found",
    //no_valid_integer_arithmetic: "No valid integer arithmetic found",
    //no_valid_expression: "No valid expression was found",
    //constants_are_immutable: "Constants are immutable. You may be trying to assign a value to a constant that has already been defined. Try renaming this as a new constant."
};

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err("missing filepath argument".to_string());
        }
        let file = File::new();
        let lines_of_chars = vec![];
        let lines_of_tokens = vec![];
        let output = "".to_string();
        let current_line = 0;
        let current_line_token = 0;
        let error_stack = vec![];
        let ast = Ast::new();
        Ok(Config {
            file,
            lines_of_chars,
            lines_of_tokens,
            output,
            current_line,
            current_line_token,
            error_stack,
            ast,
        })
    }

    pub fn run(self: &mut Self, args: &[String]) -> Result<(), Box<dyn Error>> {
        self.file.get(args)?;
        //dbg!(self.file.clone());
        match self.run_main_tasks() {
            Ok(_) => (),
            Err(_e) => (),
        }
        self.file
            .writefile_or_error(&self.ast.output, self.error_stack.len() > 0)
    }

    pub fn run_main_tasks(self: &mut Self) -> Result<(), ()> {
        self.set_lines_of_chars();
        self.set_lines_of_tokens();
        self.run_main_loop()
    }

    fn run_main_loop(self: &mut Self) -> Result<(), ()> {
        // ref: https://doc.rust-lang.org/reference/tokens.html
        // ref: https://elm-lang.org/docs/syntax

        match self.main_loop_over_lines_of_tokens() {
            Ok(_) => {
                ////dbg!(&self.ast);
                if self.error_stack.len() > 0 {
                    println!("----------\r\n\r\nTOYLANG COMPILE ERROR:");
                    for error in self.error_stack.clone() {
                        println!("{}", error);
                    }
                    println!("----------\r\n");
                    dbg!(self.ast.clone());
                } else {
                    self.ast.set_output();
                    println!(
                        "\r\nToylang compiled successfully:\r\n----------\r\n{:?}\r\n----------\r\n",
                        self.ast
                    );
                }
            }
            Err(_) => {
                println!("----------\r\n\r\nTOYLANG COMPILE ERROR:");
                for error in self.error_stack.clone() {
                    println!("{}", error);
                }
                println!("----------\r\n");
                dbg!(self.ast.clone());
            }
        };
        Ok(())
    }

    fn main_loop_over_lines_of_tokens(self: &mut Self) -> Result<(), ()> {
        //self.set_ast_output_for_main_fn_start();
        for line in 0..self.lines_of_tokens.len() {
            if self.lines_of_tokens[line].len() > 0 {
                //println!("line: {}", line);
                self.current_line = line;
                self.current_line_token = 0;
                self.parse_current_line()?;
                //println!("end of line: {}\r\n", line);
            }
        }
        Ok(())
    }

    fn parse_current_line(self: &mut Self) -> Result<(), ()> {
        let tokens = self.lines_of_tokens[self.current_line].clone();
        if tokens.len() > 0 {
            while self.current_line_token < tokens.len() {
                self.parse_current_token(&tokens)?;
                self.current_line_token = self.current_line_token + 1;
            }
        }
        Ok(())
    }

    fn parse_current_token(self: &mut Self, tokens: &Tokens) -> Result<(), ()> {
        let current_token = tokens[self.current_line_token].clone();
        let current_token_vec: &Vec<char> = &tokens[self.current_line_token].chars().collect();
        if current_token_vec.len() == 0 {
            return Ok(());
        }

        match self.ast.get_inbuilt_function_index_by_name(&current_token) {
            Some(index_of_function) => self.parse_function_call(&current_token, index_of_function),
            _ => match self.ast.get_inbuilt_type_index_by_name(&current_token) {
                Some(index_of_type) => self.parse_type(index_of_type),
                _ => self.parse_token_by_first_chars(&current_token, &current_token_vec),
            },
        }
    }

    fn parse_token_by_first_chars(
        self: &mut Self,
        current_token: &String,
        current_token_vec: &Vec<char>,
    ) -> Result<(), ()> {
        let first_char = current_token_vec[0];
        let second_char = if current_token_vec.len() > 1 {
            Some(current_token_vec[1])
        } else {
            None
        };
        match first_char {
            '\\' => self.parse_function_definition_start(),
            ':' => self.parse_function_definition_end(),
            '/' => self.parse_comment_single_line(current_token_vec),
            '=' => {
                if current_token_vec.len() > 1 {
                    return self.get_error2(0, 1, ERRORS.assign);
                }
                self.parse_assignment(&current_token)
            }
            '"' => self.parse_string(&current_token),
            //positive numbers
            first_char if is_integer(&first_char.to_string()) => {
                if is_float(&current_token) {
                    self.parse_float(&current_token)
                } else {
                    self.parse_int(&current_token)
                }
            }
            //negative numbers
            '-' => match second_char {
                Some(_digit) => {
                    if is_float(&current_token) {
                        self.parse_float(&current_token)
                    } else {
                        self.parse_int(&current_token)
                    }
                }
                None => {
                    return self.get_error2(0, 1, ERRORS.int_negative);
                }
            },
            first_char if "abcdefghijklmnopqrstuvwxyz".contains(&first_char.to_string()) => {
                //dbg!("constant or constantRef", first_char);
                self.parse_constant(&current_token)
            }
            _ => Err(()),
        }
    }

    fn parse_comment_single_line(self: &mut Self, current_token_vec: &Vec<char>) -> Result<(), ()> {
        if current_token_vec.len() < 2 || current_token_vec[1] != '/' {
            return self.get_error2(0, 1, ERRORS.comment_single_line);
        }
        let val = concatenate_vec_strings(&self.lines_of_tokens[self.current_line]);
        self.ast.append((ElementInfo::Indent, vec![]));
        self.ast
            .append((ElementInfo::CommentSingleLine(val), vec![]));
        self.ast.append((ElementInfo::Eol, vec![]));
        Ok(())
    }

    fn parse_type(self: &mut Self, index_of_type: usize) -> Result<(), ()> {
        match self.ast.elements[index_of_type].clone() {
            sometype => {
                self.indent_if_first_in_line();
                self.ast.append(sometype);
                //self.outdent_if_last_expected_child();
                //self.seol_if_last_in_line();
                Ok(())
            } //_ => self.get_error2(0, 1, ERRORS.typeerror),
        }
    }

    fn parse_string(self: &mut Self, current_token: &String) -> Result<(), ()> {
        if is_string(&current_token.clone()) {
            self.indent_if_first_in_line();
            self.ast
                .append((ElementInfo::String(current_token.clone()), vec![]));
            self.outdent_if_last_expected_child();
            self.seol_if_last_in_line();
            Ok(())
        } else {
            //dbg!(&self.lines_of_tokens);
            self.get_error2(0, 1, ERRORS.string)
        }
    }

    fn parse_int(self: &mut Self, current_token: &String) -> Result<(), ()> {
        //dbg!("parse_int - positive only for now");
        let all_chars_are_numeric = current_token.chars().into_iter().all(|c| c.is_numeric());
        let chars: Vec<char> = current_token.chars().collect();
        let first_char_is_negative_sign = chars[0] == '-';
        let is_negative_all_other_chars_are_not_numeric = first_char_is_negative_sign
            && chars.len() > 1
            && !chars[1..chars.len()].into_iter().all(|c| c.is_numeric());

        if (!first_char_is_negative_sign && !all_chars_are_numeric)
            || is_negative_all_other_chars_are_not_numeric
        {
            self.get_error2(0, 1, ERRORS.int)?;
        }
        match current_token.parse::<i64>() {
            Ok(_) => (),
            Err(_) => self.get_error2(0, 1, ERRORS.int_out_of_bounds)?,
        }
        self.indent_if_first_in_line();
        self.ast
            .append((ElementInfo::Int(current_token.clone()), vec![]));
        //dbg!(
        //    self.current_line_token,
        //    self.lines_of_tokens[self.current_line].clone()
        //);
        self.outdent_if_last_expected_child();

        //allow seol before outdenting
        self.seol_if_last_in_line();

        Ok(())
    }

    fn parse_float(self: &mut Self, current_token: &String) -> Result<(), ()> {
        if current_token.len() > 0 && is_float(current_token) {
            self.indent_if_first_in_line();
            self.ast
                .append((ElementInfo::Float(current_token.clone()), vec![]));
            self.outdent_if_last_expected_child();
            self.seol_if_last_in_line();
            Ok(())
        } else {
            return self.get_error2(0, 1, ERRORS.float);
        }
    }

    fn parse_constant(self: &mut Self, current_token: &String) -> Result<(), ()> {
        //dbg!(current_token);
        let el_option = self.ast.get_existing_element_by_name(current_token);
        match el_option {
            Some((ElementInfo::Constant(_, returntype), _)) => {
                //equals a reference to existing constant
                //Some(_ref_of_constant) => {
                //dbg!("1.ref");
                //let typename = self.ast_get_type(&current_token);
                self.indent_if_first_in_line();
                self.ast.append((
                    ElementInfo::ConstantRef(
                        current_token.clone(),
                        returntype, //typename,
                        current_token.clone(),
                    ),
                    vec![],
                ));
                self.outdent_if_last_expected_child();
                self.seol_if_last_in_line();
                //}
                //create a new constant
                //_ => (),
            }
            Some((ElementInfo::Arg(_, _, returntype), _)) => {
                self.indent_if_first_in_line();
                self.ast.append((
                    ElementInfo::ConstantRef(
                        current_token.clone(),
                        returntype,
                        current_token.clone(),
                    ),
                    vec![],
                ));
                self.outdent_if_last_expected_child();
                self.seol_if_last_in_line();
            }
            _ => {
                //create a new constant
                //dbg!("2.const");
                let typename = "Undefined".to_string();
                self.indent_if_first_in_line();
                //TODO change this to inbuiltfunction?

                let parent_ref = self.ast.get_current_parent_ref_from_parents();
                let parent = self.ast.elements[parent_ref].clone();
                match parent.0 {
                    ElementInfo::FunctionDefWIP => {
                        self.ast.append((
                            ElementInfo::Arg(
                                current_token.clone(),
                                parent_ref,
                                "Undefined".to_string(),
                            ),
                            vec![],
                        ));
                    }
                    _ => {
                        self.ast.append((
                            ElementInfo::Constant(current_token.clone(), typename),
                            vec![],
                        ));
                    }
                }

                //dbg!("constant 1", self.ast.parents.clone());
                self.outdent_if_last_expected_child();
                //dbg!("constant 2", self.ast.parents.clone());
                self.seol_if_last_in_line();
            }
        }

        Ok(())
    }

    fn parse_assignment(self: &mut Self, _current_token: &String) -> Result<(), ()> {
        self.indent_if_first_in_line();
        let undefined_for_now = "Undefined".to_string();
        self.ast
            .append((ElementInfo::Assignment(undefined_for_now), vec![]));
        self.outdent_if_last_expected_child();
        self.ast.indent();
        Ok(())
    }

    fn parse_function_call(
        self: &mut Self,
        current_token: &String,
        index_of_function: usize,
    ) -> Result<(), ()> {
        self.indent_if_first_in_line();
        let undefined_for_now = "Undefined".to_string();
        self.ast.append((
            ElementInfo::InbuiltFunctionCall(
                current_token.clone(),
                index_of_function,
                undefined_for_now,
            ),
            vec![],
        ));
        self.outdent_if_last_expected_child();
        self.ast.indent();
        Ok(())
    }

    fn parse_function_definition_start(self: &mut Self) -> Result<(), ()> {
        self.indent_if_first_in_line();
        self.ast.append((ElementInfo::FunctionDefWIP, vec![]));
        //self.outdent_if_last_expected_child();
        self.ast.indent();
        Ok(())
    }

    fn parse_function_definition_end(self: &mut Self) -> Result<(), ()> {
        /*
        At the point you parse a function definition end,
        and because we don't look ahead when parsing,
        the Ast thinks this is what has been parsed
        ...
        10: Assignment: (Undefined) [ 11, 12, ]
        11: Constant: a (Undefined) [ ]
        12: FunctionDef: ("", [], [], Undefined) [ 13, 14, 15, ]
        13: Type: i64 [ ]
        14: Type: i64 [ ]
        15: Constant: arg1 (Undefined) [ ]

        We need to change this to, e.g. this for a single line function...
        10: Unused
        11: Unused
        12: FunctionDef(name, argtypes, argnames, returntype): [ ] (<-ready to accept 16 return statement)
        13: Unused
        14: Unused
        15: Unused
        ... ready to insert next element 16 which is the return statement
        */

        //get parent funcdef
        let func_def_ref_option = self
            .ast
            .get_current_parent_ref_from_element_children_search(self.ast.elements.len() - 1);
        //dbg!(self.clone(), func_def_ref_option);
        match func_def_ref_option {
            Some(func_def_ref) => {
                //get child refs
                let func_def = self.ast.elements[func_def_ref].clone();
                let children = func_def.1;
                //dbg!(children.clone());
                //error if count is NOT odd (argtypes + returntype + argnames)
                if children.len() % 2 == 0 || children.len() == 0 {
                    return self.get_error2(0, 1, ERRORS.funcdef_args);
                }

                //TODO deal with brackets later (i.e. for type signature containing argument(s) which are fns)

                //error if arg types are NOT first
                let first_child_ref = children[0];

                let first_child = self.ast.elements[first_child_ref].clone();
                match first_child.0 {
                    ElementInfo::Type(_) => (),
                    _ => return self.get_error2(0, 1, ERRORS.funcdef_argtypes_first),
                }

                match func_def.0 {
                    ElementInfo::FunctionDefWIP => {
                        //(Assignment is parent of functionDefWIP)
                        let assignment_ref_option = self
                            .ast
                            .get_current_parent_ref_from_element_children_search(func_def_ref);

                        match assignment_ref_option {
                            Some(assignment_ref) => {
                                let assignment_element = self.ast.elements[assignment_ref].clone();

                                //constant is first child of assignment
                                let constant_ref = assignment_element.1[0];
                                let constant = self.ast.elements[constant_ref].clone();
                                match constant.0 {
                                    ElementInfo::Constant(name, _) => {
                                        //assign name to parent funcdef (from constant
                                        let num_args = children.len() / 2;
                                        let argtype_refs = &children[..num_args];
                                        let mut argtypes: Vec<String> = vec![];
                                        for a in argtype_refs {
                                            match self.ast.elements[a.clone()].clone() {
                                                (ElementInfo::Type(typename), _) => {
                                                    argtypes.push(typename)
                                                }
                                                _ => (),
                                            }
                                        }

                                        let returntype_ref = &children[num_args];
                                        let returntype: String =
                                            match self.ast.elements[returntype_ref.clone()].clone()
                                            {
                                                (ElementInfo::Type(typename), _) => typename,
                                                _ => "Undefined".to_string(),
                                            };

                                        let argname_refs = &children[num_args + 1..];
                                        let mut argnames: Vec<String> = vec![];
                                        for a in argname_refs {
                                            match self.ast.elements[a.clone()].clone() {
                                                (ElementInfo::Arg(argname, _, _), _) => {
                                                    argnames.push(argname)
                                                }
                                                _ => (),
                                            }
                                        }

                                        //assign argtypes to parent funcdef
                                        //assign returntype to parent funcdef
                                        //assign argnames to parent funcdef
                                        let new_funcdef = ElementInfo::FunctionDef(
                                            name, argnames, argtypes, returntype,
                                        );

                                        // replace assignment with unused
                                        self.ast.elements[assignment_ref] =
                                            (ElementInfo::Unused, vec![]);

                                        // replace parents child reference to the assignment
                                        // with the func_def_ref
                                        let parent_of_assignment_ref_option = self
                                            .ast
                                            .get_current_parent_ref_from_element_children_search(
                                                assignment_ref,
                                            );
                                        match parent_of_assignment_ref_option {
                                            Some(index) => {
                                                self.ast.replace_element_child(
                                                    index,
                                                    assignment_ref,
                                                    func_def_ref,
                                                );
                                            }
                                            _ => (),
                                        }

                                        // replace original funcdefWIP with funcdef
                                        self.ast.elements[func_def_ref] = (new_funcdef, vec![]);

                                        // replace constant with Unused
                                        self.ast.elements[constant_ref] =
                                            (ElementInfo::Unused, vec![]);

                                        // replace funcdef children with Unused
                                        //for child_ref in children {
                                        //    self.ast.elements[child_ref] =
                                        //        (ElementInfo::Unused, vec![]);
                                        //}

                                        //re-add the new funcdef as latest parent, so we can continue parsing with it's child statements
                                        //dbg!(self.clone());
                                        self.ast.outdent();
                                        self.ast.outdent();
                                        self.ast.indent_this(func_def_ref);
                                        //self.ast.indent_this(assignment_ref);
                                        //dbg!(self.ast.parents.clone());
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        //unsure if needed
        self.outdent_if_last_expected_child();
        Ok(())
    }

    fn indent_if_first_in_line(self: &mut Self) {
        //or if first part of the expression in a single line function (after the colon)
        //e.g. the "+ 123 arg1"  in "= a \\ i64 i64 arg1 : + 123 arg1"
        if self.current_line_token == 0 {
            //for _n in 0..self.ast.parents.len() - 1 {
            //    dbg!("self.ast.parents", self.ast.parents.clone());
            self.ast.append((ElementInfo::Indent, vec![]));
            //}
        }
    }

    fn seol_if_last_in_line(self: &mut Self) {
        let is_last_token_in_this_line =
            self.current_line_token == self.lines_of_tokens[self.current_line].len() - 1;

        if is_last_token_in_this_line {
            // also if is the last return expression of a func_def
            // then don't add the semicolon, just the EOL
            let parent = self.ast.get_current_parent_ref_from_parents();
            let parent_children_el = self.ast.elements[parent].clone();
            let parent_children = parent_children_el.1;
            if parent_children.len() > 0 {
                let last_child_ref = parent_children[parent_children.len() - 1];
                let current_el = self.ast.elements[last_child_ref].clone();
                match current_el.0 {
                    ElementInfo::FunctionDef(_, _, _, _) => {
                        self.ast.append((ElementInfo::Eol, vec![]));
                        return;
                    }
                    _ => (),
                }
                //dbg!(last_child_ref);
                self.ast.append((ElementInfo::Seol, vec![]));
            }
        }
    }

    fn outdent_if_last_expected_child(self: &mut Self) {
        let mut prev_parents_len = 999999999;
        loop {
            //dbg!("loop", self.ast.parents.clone());
            if self.ast.parents.len() < 2 || self.ast.parents.len() == prev_parents_len {
                break;
            }
            prev_parents_len = self.ast.parents.len();
            let current_parent_ref = self.ast.get_current_parent_ref_from_parents();
            let current_parent = self.ast.elements[current_parent_ref].clone();
            dbg!("---", self.ast.clone());
            match current_parent.0 {
                ElementInfo::Root => (),
                ElementInfo::CommentSingleLine(_) => (),
                ElementInfo::Int(_) => (),
                ElementInfo::Float(_) => (),
                ElementInfo::String(_) => (),
                ElementInfo::Arg(_, _, _) => (),
                ElementInfo::Constant(_, _) => {
                    //dbg!("Constant");
                    if current_parent.1.len() > 0 {
                        dbg!("Constant outdent", self.ast.parents.clone(),);
                        self.ast.outdent();
                    }
                }
                ElementInfo::ConstantRef(_, _, _) => (),
                ElementInfo::Assignment(_) => {
                    //dbg!("Assignment");
                    if current_parent.1.len() > 1 {
                        dbg!("Assignment outdent", self.ast.parents.clone(),);
                        self.ast.outdent();
                    }
                }
                ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => (),
                ElementInfo::InbuiltFunctionCall(_, fndefref, _) => {
                    //dbg!("InbuiltFunctionCall");
                    let fndef = self.ast.elements[fndefref].clone();
                    match fndef.0 {
                        ElementInfo::InbuiltFunctionDef(_, argnames, _, _, _) => {
                            // current assumption is inbuiltfunctionCalls expect a fixed number
                            // of children to match args.
                            if current_parent.1.len() == argnames.len() {
                                dbg!("InbuiltFunctionCall outdent", self.ast.parents.clone(),);
                                self.ast.outdent();
                            }
                        }
                        _ => (),
                    }
                }
                ElementInfo::FunctionDefWIP => (),
                ElementInfo::FunctionDef(_, argnames, _, _) => {
                    //dbg!("FunctionDef");
                    // outdent if a return expression
                    // i.e. if previous element is an indent
                    let previous_element = self.ast.elements[self.ast.elements.len() - 2].clone(); //should be safe to subtract 2 since there should always be a root

                    // then the following are the first element in the row
                    // and they are all return expressions

                    match (previous_element.0, self.ast.get_last_element().0) {
                        (ElementInfo::Indent, ElementInfo::Int(_)) => {
                            dbg!("FunctionDef outdent Int", self.ast.parents.clone(),);
                            self.ast.outdent();
                        }
                        (ElementInfo::Indent, ElementInfo::Float(_)) => {
                            dbg!("FunctionDef outdent Float", self.ast.parents.clone(),);
                            self.ast.outdent();
                        }
                        (ElementInfo::Indent, ElementInfo::String(_)) => {
                            dbg!("FunctionDef outdent String", self.ast.parents.clone(),);
                            self.ast.outdent();
                        }
                        (ElementInfo::Indent, ElementInfo::Constant(_, _)) => {
                            dbg!("FunctionDef outdent Constant", self.ast.parents.clone(),);
                            self.ast.outdent();
                        }
                        (ElementInfo::Indent, ElementInfo::ConstantRef(_, _, _)) => {
                            dbg!("FunctionDef outdent ConstantRef", self.ast.parents.clone(),);
                            self.ast.outdent();
                        }
                        (ElementInfo::Indent, ElementInfo::InbuiltFunctionCall(_, fndefref, _)) => {
                            //dbg!("InbuiltFunctionCall");
                            let fndef = self.ast.elements[fndefref].clone();
                            match fndef.0 {
                                ElementInfo::InbuiltFunctionDef(_, argnames, _, _, _) => {
                                    // current assumption is inbuiltFunctionCalls expect a fixed number
                                    // of children to match args
                                    if fndef.1.len() == argnames.len() {
                                        dbg!(
                                            "FunctionDef outdent InbuiltFunctionCall enough args",
                                            self.ast.parents.clone(),
                                        );
                                        self.ast.outdent();
                                    }
                                }
                                _ => (),
                            }
                        }
                        (ElementInfo::Indent, ElementInfo::FunctionCall(name)) => {
                            //dbg!("FunctionCall");
                            let fn_index = self.ast.get_function_index_by_name(&name);
                            match fn_index {
                                Some(index) => {
                                    let fndef = self.ast.elements[index].clone();
                                    match fndef.0 {
                                        ElementInfo::FunctionDef(_, argnames, _, _) => {
                                            // current assumption is functionCalls expect a fixed number
                                            // of children to match args
                                            if fndef.1.len() == argnames.len() {
                                                dbg!(
                                                    "FunctionDef outdent FunctionCall enough args",
                                                    self.ast.parents.clone(),
                                                );
                                                self.ast.outdent();
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                _ => (), // something went wrong
                            }
                        }
                        _ => (),
                    }
                }
                ElementInfo::FunctionCall(name) => {
                    //dbg!("FunctionCall");
                    let fn_index = self.ast.get_function_index_by_name(&name);
                    match fn_index {
                        Some(index) => {
                            let fndef = self.ast.elements[index].clone();
                            match fndef.0 {
                                ElementInfo::FunctionDef(_, argnames, _, _) => {
                                    if fndef.1.len() == argnames.len() {
                                        dbg!("FunctionCall outdent enough args");
                                        self.ast.outdent();
                                    }
                                }
                                _ => (),
                            }
                        }
                        _ => (), // something went wrong
                    }
                }
                ElementInfo::Type(_) => (),
                ElementInfo::Eol => (),
                ElementInfo::Seol => (),
                ElementInfo::Indent => (),
                ElementInfo::Unused => (),
            }
        }
    }

    /*
    fn check_one_or_more_succeeds(
        self: &mut Self,
        tokens: Tokens,
    ) -> Result<Tokens, FullOrValidationError> {
        match self.check_one_succeeds("ast_set_comment_single_line", &tokens, None, true) {
            Ok(_) => return Ok(tokens),
            Err(false) => return Err(false),
            Err(true) => (),
        }
        match self.check_one_succeeds("ast_set_int", &tokens, None, true) {
            Ok(_) => {
                self.ast.append((ElementInfo::Seol, vec![]));
                return Ok(tokens);
            }
            Err(false) => return Err(false),
            Err(true) => (),
        }
        match self.check_one_succeeds("ast_set_float", &tokens, None, true) {
            Ok(_) => {
                self.ast.append((ElementInfo::Seol, vec![]));
                return Ok(tokens);
            }
            Err(false) => return Err(false),
            Err(true) => (),
        }
        match self.check_one_succeeds("ast_set_string", &tokens, None, true) {
            Ok(_) => {
                self.ast.append((ElementInfo::Seol, vec![]));
                return Ok(tokens);
            }
            Err(false) => return Err(false),
            Err(true) => (),
        }
        match self.check_one_succeeds("ast_set_constant", &tokens, None, true) {
            Ok(_) => {
                self.ast.append((ElementInfo::Seol, vec![]));
                return Ok(tokens);
            }
            Err(false) => return Err(false),
            Err(true) => (),
        }
        match self.check_one_succeeds("ast_set_inbuilt_function", &tokens, None, true) {
            Ok(_) => return Ok(tokens),
            Err(false) => return Err(false),
            Err(true) => (),
        }
        self.get_error_ok(0, 1, ERRORS.no_valid_expression, true)
    }

    fn check_one_or_more_succeeds_for_returntypes(
        self: &mut Self,
        tokens: Tokens,
        returntype: String,
    ) -> Result<Tokens, ()> {
        //dbg!(&returntype);
        if returntype.contains(&"i64".to_string()) {
            //dbg!("i64");
            match self.check_one_succeeds("ast_set_int", &tokens, None, false) {
                Ok(remaining_tokens) => {
                    //dbg!("ok i64");
                    return Ok(remaining_tokens);
                }
                _ => (),
            }
        }
        if returntype.contains(&"f64".to_string()) {
            match self.check_one_succeeds("ast_set_float", &tokens, None, false) {
                Ok(remaining_tokens) => return Ok(remaining_tokens),
                _ => (),
            }
        }
        if returntype.contains(&"String".to_string()) {
            match self.check_one_succeeds("ast_set_string", &tokens, None, false) {
                Ok(remaining_tokens) => return Ok(remaining_tokens),
                _ => (),
            }
        }
        match self.check_one_succeeds("ast_set_inbuilt_function", &tokens, Some(returntype), false)
        {
            Ok(remaining_tokens) => return Ok(remaining_tokens),
            _ => (),
        }
        self.get_error(0, 1, ERRORS.no_valid_expression)
    }

    fn check_one_succeeds(
        self: &mut Self,
        function_name: &str,
        tokens: &Tokens,
        returntype: Option<String>,
        singleline: bool,
    ) -> Result<Tokens, FullOrValidationError> {
        let mut _succeeded = false;
        let mut clone = self.clone();
        let result = match function_name {
            "ast_set_comment_single_line" => clone.ast_set_comment_single_line(tokens),
            "ast_set_int" => clone.ast_set_int(tokens, singleline),
            "ast_set_float" => clone.ast_set_float(tokens, singleline),
            "ast_set_string" => clone.ast_set_string(tokens, singleline),
            "ast_set_constant" => clone.ast_set_constant(tokens),
            "ast_set_inbuilt_function" => clone.ast_set_inbuilt_function(tokens, returntype),
            _ => {
                return Ok(tokens.clone());
            }
        };
        dbg!(&clone.error_stack);
        match result {
            Ok(vec_string) => {
                self.set_all_from_clone(clone);
                Ok(vec_string)
            }
            Err(false) => {
                self.set_all_from_clone(clone);
                Err(false)
            }
            Err(true) => {
                //self.set_all_from_clone(clone);
                Err(true)
            }
        }
    }
    */

    fn set_lines_of_chars(self: &mut Self) {
        let mut index_from = 0;
        let mut index_to = 0;
        let char_vec: Vec<char> = self.file.filecontents.chars().collect();
        while index_to < char_vec.len() {
            let c = char_vec[index_to];
            let incr =
                if index_to + 1 < char_vec.len() && c == '\r' && char_vec[index_to + 1] == '\n' {
                    2
                } else {
                    1
                };
            let eof = index_to == char_vec.len() - 1;
            let is_colon = c == ':'; // split line here for single line functions
            if c == '\r' || c == '\n' || eof || is_colon {
                self.lines_of_chars.push(
                    char_vec[index_from..index_to + (if eof || is_colon { 1 } else { 0 })].to_vec(),
                );
                index_from = index_to + incr;
            }
            index_to = index_to + incr;
        }
    }

    fn set_lines_of_tokens(self: &mut Self) {
        for line in 0..self.lines_of_chars.len() {
            let mut index_from = 0;
            let mut index_to = 0;
            let mut count_quotes = 0;

            let char_vec_initial: Vec<char> = self.lines_of_chars[line].clone();
            let char_as_string = char_vec_initial.iter().cloned().collect::<String>();
            let removed_leading_whitespace = strip_leading_whitespace(char_as_string);
            let removed_trailing_whitespace = strip_trailing_whitespace(removed_leading_whitespace);
            let char_vec: Vec<char> = removed_trailing_whitespace.chars().collect();

            let mut inside_quotes = false;
            //let mut is_colon: bool = false;
            let mut line_of_tokens: Tokens = vec![];
            while index_to < char_vec.len() {
                let c = char_vec[index_to];
                let eof = index_to == char_vec.len() - 1;
                if c == '"' {
                    inside_quotes = !inside_quotes;
                    count_quotes = count_quotes + 1;
                };
                let is_comment = char_vec.len() > 1 && char_vec[0] == '/' && char_vec[1] == '/';
                //is_colon = c == ':';
                //dbg!(c, is_colon);
                if (c.is_whitespace() && index_to != 0 && !inside_quotes && !is_comment)
                    || eof
                    || count_quotes == 2
                //|| is_colon
                {
                    let token_chars = char_vec
                        [index_from..index_to + (if eof || count_quotes == 2 { 1 } else { 0 })]
                        .iter()
                        .cloned()
                        .collect::<String>();
                    line_of_tokens.push(token_chars);
                    index_from = index_to + 1;
                    inside_quotes = false;
                    count_quotes = 0;
                }
                index_to = index_to + 1;
            }

            self.lines_of_tokens.push(line_of_tokens);
        }
        //dbg!(self.lines_of_tokens.clone());
    }

    /*
    fn set_all_from_clone(self: &mut Self, to_clone: Config) -> () {
        // wokraround - can't just do 'self = clone.clone();' due to &mut derferencing ??
        self.file = to_clone.file;
        self.lines_of_chars = to_clone.lines_of_chars;
        self.lines_of_tokens = to_clone.lines_of_tokens;
        self.output = to_clone.output;
        self.current_line = to_clone.current_line;
        self.error_stack = to_clone.error_stack;
        self.ast = to_clone.ast;
    }

    fn ast_set_comment_single_line(
        self: &mut Self,
        tokens: &Tokens,
    ) -> Result<Tokens, FullOrValidationError> {
        let first_token_chars = tokens[0].chars().collect::<Vec<char>>();
        if first_token_chars.len() > 1 && first_token_chars[0] == '/' && first_token_chars[1] == '/'
        {
            let val = concatenate_vec_strings(tokens);
            self.ast.append((ElementInfo::Indent, vec![]));
            self.ast
                .append((ElementInfo::CommentSingleLine(val), vec![]));
            self.ast.append((ElementInfo::Eol, vec![]));
            Ok(vec![])
            //let validation_error = None;
            //Ok(validation_error)
        } else {
            self.get_error_ok(0, 1, ERRORS.comment_single_line, true)
            //self.get_error(0, 1, ERRORS.no_valid_comment_single_line)
        }
    }

    fn ast_set_int(
        self: &mut Self,
        tokens: &Tokens,
        singleline: bool,
    ) -> Result<Tokens, FullOrValidationError> {
        //dbg!("set_int");
        if tokens.len() == 0 || tokens[0].len() == 0 {
            dbg!(is_integer(&tokens[0]));
            return self.get_error_ok(0, 1, ERRORS.int, true);
        }
        let first_token_vec: &Vec<char> = &tokens[0].chars().collect();
        let first_char = first_token_vec[0];
        dbg!(
            "1",
            &first_char,
            is_integer(&first_char.to_string()),
            is_integer(&tokens[0])
        );
        if is_integer(&first_char.to_string()) && !is_integer(&tokens[0]) {
            return self.get_error_ok(
                0,
                1,
                "not a valid int: starts with a digit, but contains non-digits",
                false,
            );
        }
        if !is_integer(&tokens[0]) {
            return self.get_error_ok(0, 1, ERRORS.int, true);
        }
        let val = tokens[0].clone();
        if singleline {
            self.ast.append((ElementInfo::Indent, vec![]));
        }
        let _x = self.ast.append((ElementInfo::Int(val), vec![]));
        //dbg!(self.ast.elements[x].clone());
        if singleline {
            self.ast.append((ElementInfo::Seol, vec![]));
        }
        return Ok(tokens_remove_head(tokens.clone()));
    }

    fn ast_set_float(
        self: &mut Self,
        tokens: &Tokens,
        singleline: bool,
    ) -> Result<Tokens, FullOrValidationError> {
        if tokens.len() > 0 && is_float(&tokens[0].to_string()) {
            let val = tokens[0].clone();
            if singleline {
                self.ast.append((ElementInfo::Indent, vec![]));
            }
            self.ast.append((ElementInfo::Float(val), vec![]));
            if singleline {
                self.ast.append((ElementInfo::Seol, vec![]));
            }
            Ok(tokens_remove_head(tokens.clone()))
            //let validation_error = None;
            //Ok(validation_error)
        } else {
            self.get_error_ok(0, 1, ERRORS.float, true)
            //self.get_error(0, 1, ERRORS.no_valid_float)
        }
    }

    fn ast_set_string(
        self: &mut Self,
        tokens: &Tokens,
        singleline: bool,
    ) -> Result<Tokens, FullOrValidationError> {
        if tokens.len() > 0 && is_string(&tokens[0].to_string()) {
            let val = tokens[0].clone();
            if singleline {
                self.ast.append((ElementInfo::Indent, vec![]));
            }
            self.ast.append((ElementInfo::String(val), vec![]));
            if singleline {
                self.ast.append((ElementInfo::Seol, vec![]));
            }
            Ok(tokens_remove_head(tokens.clone()))
            //let validation_error = None;
            //Ok(validation_error)
        } else {
            self.get_error_ok(0, 1, ERRORS.string, true)
            //self.get_error(0, 1, ERRORS.no_valid_string)
        }
    }

    fn ast_set_constant(self: &mut Self, tokens: &Tokens) -> Result<Tokens, FullOrValidationError> {
        if tokens.len() > 2 && tokens[0] == "=".to_string() {
            //dbg!("ast_set_constant");
            let name = tokens[1].clone();
            let typename = self.ast_get_type(&tokens[2].clone());
            let val = tokens[2].clone();

            //let validation_error = None;
            match self.ast.get_constant_index_by_name(&val) {
                //equals a reference to existing constant
                Some(_ref_of_constant) => {
                    //dbg!("1");
                    self.ast.append((ElementInfo::Indent, vec![]));
                    self.ast
                        .append((ElementInfo::ConstantRef(name, typename, val), vec![]));
                    let mut ret_tokens = tokens_remove_head(tokens.clone());
                    ret_tokens = tokens_remove_head(ret_tokens);
                    Ok(tokens_remove_head(ret_tokens))
                    //Ok(validation_error)
                }
                //create a new constant, with no value assigned yet
                _ => {
                    //dbg!("2");
                    self.ast.append((ElementInfo::Indent, vec![]));
                    let ref_of_value_option = self.ast_set_ref_by_type(&val);
                    match ref_of_value_option {
                        Some(ref_of_value) => {
                            //dbg!("3");

                            let ref_of_constant = self.ast.append((
                                ElementInfo::Constant(name, typename.clone()),
                                vec![ref_of_value],
                            ));
                            //self.ast.parents = vec_remove_tail(self.ast.parents.clone());
                            self.ast.append((ElementInfo::Seol, vec![]));

                            //remove 3 tokens
                            // "=", name of constant, first part of value
                            // e.g. remove "= const_name +" from "= const_name + 2 3"
                            // leaving just 2 3 as the args for the function (if it was a function)
                            // or nothing if "+" was instead just an int (then no more tokens left)
                            let mut remaining_tokens = tokens_remove_head(tokens.clone());
                            remaining_tokens = tokens_remove_head(remaining_tokens);
                            remaining_tokens = tokens_remove_head(remaining_tokens);

                            let element_from_ref = self.ast.elements[ref_of_value].clone();
                            //dbg!(element_from_ref.clone());
                            match element_from_ref.0 {
                                ElementInfo::InbuiltFunctionCall(name, _) => {
                                    let function_def_for_this_call_option =
                                        self.ast.get_inbuilt_function_by_name(&name);
                                    //dbg!(function_def_for_this_call_option.clone());
                                    //dbg!(&self.ast.parents);
                                    match function_def_for_this_call_option {
                                        Some(ElementInfo::InbuiltFunctionDef(
                                            _,
                                            _,
                                            arg_types,
                                            return_type,
                                            _,
                                        )) => {
                                            //penguin

                                            self.ast.parents.push(ref_of_value); //[self.parents.len() - 1];

                                            for _argtype in arg_types {
                                                //dbg!(argtype);
                                                //dbg!(&self.ast.parents);
                                                match self
                                                    .check_one_or_more_succeeds_for_returntypes(
                                                        remaining_tokens,
                                                        return_type.clone(),
                                                    ) {
                                                    Ok(returned_tokens) => {
                                                        //dbg!("return_type", return_type.clone());
                                                        remaining_tokens = returned_tokens;

                                                        // TODO also fix the type if it happens to be optional, like i64/f64

                                                        //ref_of_constant = constant
                                                        //ref_of_value = functionCall

                                                        //get type from first child of function
                                                        if return_type.contains("|") {
                                                            let el_of_fn = self.ast.elements
                                                                [ref_of_value]
                                                                .clone();
                                                            let ref_of_first_child = el_of_fn.1[0];
                                                            let el_of_first_child = self
                                                                .ast
                                                                .elements[ref_of_first_child]
                                                                .clone();
                                                            let first_child_type =
                                                                self.ast.get_elementinfo_type(
                                                                    el_of_first_child.0,
                                                                );
                                                            let previously_saved_function =
                                                                self.ast.elements[ref_of_value]
                                                                    .clone();
                                                            match previously_saved_function {
                                                                (
                                                                    ElementInfo::Constant(
                                                                        fn_name,
                                                                        _,
                                                                    ),
                                                                    fn_children,
                                                                ) => {
                                                                    let new_function_with_corrected_type = (
                                                                        ElementInfo::InbuiltFunctionCall(
                                                                            fn_name,
                                                                            first_child_type
                                                                                .clone(),
                                                                        ),
                                                                        fn_children,
                                                                    );
                                                                    self.ast.elements[ref_of_value] =
                                                                            new_function_with_corrected_type.clone();
                                                                    //dbg!("b", new_function_with_corrected_type);
                                                                    ()
                                                                }
                                                                _ => {
                                                                    //dbg!("c");
                                                                    ()
                                                                }
                                                            }

                                                            //then fix constant

                                                            let previously_saved_constant =
                                                                self.ast.elements[ref_of_constant]
                                                                    .clone();
                                                            match previously_saved_constant {
                                                                (
                                                                    ElementInfo::Constant(
                                                                        constant_name,
                                                                        _,
                                                                    ),
                                                                    constant_children,
                                                                ) => {
                                                                    let new_constant_with_corrected_type = (
                                                                        ElementInfo::Constant(
                                                                            constant_name,
                                                                            first_child_type
                                                                                .clone(),
                                                                        ),
                                                                        constant_children,
                                                                    );
                                                                    self.ast.elements[ref_of_constant] =
                                                                    new_constant_with_corrected_type.clone();
                                                                    //dbg!("b", new_constant_with_corrected_type);
                                                                    ()
                                                                }
                                                                _ => {
                                                                    //dbg!("c");
                                                                    ()
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Err(_e) => {
                                                        //return self.get_error(
                                                        //    0,
                                                        //    1,
                                                        //    ERRORS.no_valid_assignment,
                                                        //)
                                                        return self.get_error_ok(
                                                            0,
                                                            1,
                                                            ERRORS.no_valid_assignment,
                                                            true,
                                                        );
                                                    }
                                                }
                                            }
                                            //self.ast.parents =
                                            //    vec_remove_head(self.ast.parents.clone());
                                            //self.ast.append((ElementInfo::Seol, vec![]));
                                        }
                                        _ => (),
                                    }
                                    ////dbg!(&self.ast);
                                }
                                _ => (),
                            }

                            // also fix the type if it happens to be optional, like i64/f64

                            // then deal with recursive nested arguments
                            // while ret_tokens.len() > 0 {}

                            Ok(remaining_tokens)
                            //Ok(validation_error
                        }
                        None => {
                            //dbg!("4");
                            return self.get_error_ok(0, 1, ERRORS.no_valid_assignment, true);
                            //return self.get_error(0, 1, ERRORS.no_valid_assignment);
                        }
                    }
                }
            }
        } else {
            return self.get_error_ok(0, 1, ERRORS.no_valid_assignment, true);
            //return self.get_error(0, 1, ERRORS.no_valid_assignment);
        }
    }

    fn ast_set_ref_by_type(self: &mut Self, val: &String) -> Option<usize> {
        match self.ast_get_enumtype_of_elementinfo(&val) {
            None => None,
            Some(ElementInfo::InbuiltFunctionDef(
                name,
                _argnames,
                _argtypes,
                return_type,
                _format,
            )) => {
                //dbg!(return_type.clone());
                let elinfo = ElementInfo::InbuiltFunctionCall(name, return_type);
                let child_refs = vec![];
                Some(self.ast.append_as_ref((elinfo, child_refs)))
            }
            Some(elinfo) => Some(self.ast.append_as_ref((elinfo, vec![]))),
        }
    }

    fn ast_set_inbuilt_function(
        self: &mut Self,
        tokens: &Tokens,
        required_return_type_option: Option<String>,
    ) -> Result<Tokens, FullOrValidationError> {
        if tokens.len() > 0 {
            //dbg!(tokens);
            let inbuilt_function_option = match required_return_type_option {
                Some(required_return_type) => self
                    .ast
                    .get_inbuilt_function_by_name_and_returntype(&tokens[0], &required_return_type),
                None => self.ast.get_inbuilt_function_by_name(&tokens[0]),
            };
            match inbuilt_function_option {
                Some(ElementInfo::InbuiltFunctionDef(
                    _name,
                    argnames,
                    argtypes,
                    returntype,
                    format,
                )) => {
                    if argnames.len() != tokens.len() - 1 {
                        return self.get_error_ok(0, 1, ERRORS.no_valid_integer_arithmetic, true);
                        //return self.get_error(0, 1, ERRORS.no_valid_integer_arithmetic);
                    }

                    //dbg!("yes", &name, &argnames, &argtypes, &returntype);
                    let mut types_match = true;
                    for i in 0..argtypes.len() {
                        let argtype = argtypes[i].clone();
                        let tokentype = self.ast_get_type(&tokens[i + 1]);

                        if argtype.contains("|") {
                            if !argtype.contains(&tokentype) {
                                types_match = false;
                            }
                        } else if argtype != tokentype {
                            types_match = false;
                        }
                        /*dbg!(
                            &argtype,
                            &tokens[i + 1],
                            &tokentype,
                            argtype.contains("|"),
                            argtype.contains(&tokentype),
                            types_match
                        );*/
                    }
                    if !types_match {
                        return self.get_error_ok(0, 1, ERRORS.no_valid_integer_arithmetic, true);
                        //return self.get_error(0, 1, ERRORS.no_valid_integer_arithmetic);
                    }

                    let mut output = format;
                    for i in 0..argnames.len() {
                        let argname = argnames[i].clone();
                        output = output.replace(&argname, &tokens[i + 1]);
                    }

                    let mut final_returntype = returntype.clone();
                    if returntype.contains("|") {
                        final_returntype = argtypes[0].clone();
                    }

                    self.ast.append((ElementInfo::Indent, vec![]));
                    //dbg!("###########################");
                    self.ast.append((
                        ElementInfo::InbuiltFunctionCall(output, final_returntype),
                        vec![],
                    ));
                    //self.ast.parents = vec_remove_tail(self.ast.parents.clone());
                    //self.ast.append((ElementInfo::Seol, vec![]));

                    //let validation_error = None;
                    let mut ret_tokens = tokens_remove_head(tokens.clone());
                    ret_tokens = tokens_remove_head(ret_tokens);
                    return Ok(tokens_remove_head(ret_tokens));
                    //Ok(validation_error
                }
                _ => {}
            }
            return self.get_error_ok(0, 1, ERRORS.no_valid_integer_arithmetic, true);
            //return self.get_error(0, 1, ERRORS.no_valid_integer_arithmetic);
        } else {
            return self.get_error_ok(0, 1, ERRORS.no_valid_integer_arithmetic, true);
            //return self.get_error(0, 1, ERRORS.no_valid_integer_arithmetic);
        }
    }
    */

    fn ast_get_type(self: &Self, text: &String) -> String {
        //dbg!("ast_get_type");
        let mut return_type = "Undefined".to_string();
        if is_integer(text) {
            return_type = "i64".to_string();
        }
        if is_float(text) {
            return_type = "f64".to_string();
        }
        if is_string(text) {
            return_type = "String".to_string();
        }
        match self.ast.get_inbuilt_function_by_name(text) {
            Some(ElementInfo::InbuiltFunctionDef(_, _, _, returntype, _)) => {
                //dbg!("ast_get_type - 1", &returntype);
                return returntype;
            }
            _ => {
                //dbg!("ast_get_type - 2");
                ()
            }
        }
        match self.ast.get_constant_by_name(text) {
            Some(ElementInfo::Constant(_, typename)) => return typename,
            Some(ElementInfo::ConstantRef(_, typename, _)) => return typename,
            _ => (),
        }
        // allow for Function Return Type
        return_type
    }

    /*
    fn ast_get_enumtype_of_elementinfo(self: &Self, text: &String) -> Option<ElementInfo> {
        //dbg!("ast_get_enum_of_element");
        //note: these don't have real values - just indicates correct Enum to use
        let mut return_type = None;
        if is_integer(text) {
            return_type = Some(ElementInfo::Int(text.clone()));
        }
        if is_float(text) {
            return_type = Some(ElementInfo::Float(text.clone()));
        }
        if is_string(text) {
            return_type = Some(ElementInfo::String(text.clone()));
        }
        match self.ast.get_inbuilt_function_by_name(text) {
            Some(ElementInfo::InbuiltFunctionDef(
                name,
                argnames,
                argtypes,
                return_type,
                format,
            )) => {
                //dbg!("ast_get_type - 1", &return_type);
                return Some(ElementInfo::InbuiltFunctionDef(
                    name,
                    argnames,
                    argtypes,
                    return_type,
                    format,
                ));
            }
            _ => {
                //dbg!("ast_get_type - 2");
                ()
            }
        }
        match self.ast.get_constant_by_name(text) {
            Some(ElementInfo::Constant(name, typename)) => {
                return Some(ElementInfo::Constant(name, typename))
            }
            Some(ElementInfo::ConstantRef(name, typename, refname)) => {
                return Some(ElementInfo::ConstantRef(name, typename, refname))
            }
            _ => (),
        }
        // allow for Function Return Type
        return_type
    }

    fn get_error(
        self: &mut Self,
        arrow_indent: usize,
        arrow_len: usize,
        error: &str,
    ) -> Result<Tokens, ()> {
        let e = format!(
            "----------\r\n./src/{}:{}:0\r\n{}\r\n{}{} {}",
            self.file.filename,
            self.current_line + 1,
            self.lines_of_chars[self.current_line]
                .iter()
                .cloned()
                .collect::<String>(),
            " ".repeat(arrow_indent),
            "^".repeat(arrow_len),
            error,
        );
        self.error_stack.push(e);
        Err(())
    }

    fn get_error_ok(
        self: &mut Self,
        arrow_indent: usize,
        arrow_len: usize,
        error: &str,
        is_real_error: bool,
    ) -> Result<Tokens, FullOrValidationError> {
        let e = format!(
            "----------\r\n./src/{}:{}:0\r\n{}\r\n{}{} {}",
            self.file.filename,
            self.current_line + 1,
            self.lines_of_chars[self.current_line]
                .iter()
                .cloned()
                .collect::<String>(),
            " ".repeat(arrow_indent),
            "^".repeat(arrow_len),
            error,
        );
        self.error_stack.push(e);
        Err(is_real_error)
    }
    */
    fn get_error2(
        self: &mut Self,
        mut arrow_indent: usize,
        arrow_len: usize,
        error: &str,
    ) -> Result<(), ()> {
        if arrow_indent == 0 && self.current_line_token != 0 {
            let line_of_tokens = self.lines_of_tokens[self.current_line].clone();
            arrow_indent = line_of_tokens[0..self.current_line_token]
                .iter()
                .cloned()
                .collect::<String>()
                .len()
                + self.current_line_token;
        }

        let e = format!(
            "----------\r\n./src/{}:{}:0\r\n{}\r\n{}{} {}",
            self.file.filename,
            self.current_line + 1,
            self.lines_of_chars[self.current_line]
                .iter()
                .cloned()
                .collect::<String>(),
            " ".repeat(arrow_indent),
            "^".repeat(arrow_len),
            error,
        );
        self.error_stack.push(e);
        Err(())
    }
}

/*
fn tokens_remove_head(tokens: Tokens) -> Tokens {
    if tokens.len() == 1 {
        vec![]
    } else {
        tokens[1..].to_vec()
    }
}
*/

fn _is_type(_text: &String) -> bool {
    true
}

fn is_integer(text: &String) -> bool {
    let mut is_valid = true;
    let all_chars_are_numeric = text.chars().into_iter().all(|c| c.is_numeric());
    let text_chars: Vec<char> = text.chars().collect();
    let first_char_is_negative_sign = text_chars[0] == '-';

    let is_negative_all_other_chars_are_numeric = first_char_is_negative_sign
        && text_chars.len() > 1
        && text_chars[1..text_chars.len()]
            .into_iter()
            .all(|c| c.is_numeric());

    if !all_chars_are_numeric && !is_negative_all_other_chars_are_numeric {
        is_valid = false;
    }
    //println!(
    //    "{}",
    //    !all_chars_are_numeric || !is_negative_all_other_chars_are_numeric
    //);
    match text.parse::<i64>() {
        Ok(_) => (),
        Err(_) => is_valid = false,
    }
    is_valid
}

fn is_float(text: &String) -> bool {
    let mut is_valid = true;
    let mut index_decimal_point = 0;
    let mut index_e = 0;
    let mut index_plus = 0;
    let char_vec: Vec<char> = text.chars().collect();
    for i in 0..text.len() {
        let c = char_vec[i];
        if c == '.' && index_decimal_point == 0 {
            index_decimal_point = i;
        } else if c == 'E' && index_e == 0 {
            index_e = i;
        } else if c == '+' && index_plus == 0 {
            index_plus = i;
        } else if !c.is_numeric() && !(i == 0 && c == '-') {
            is_valid = false;
        }
    }
    let has_one_decimal_point = index_decimal_point != 0;
    let no_power_of_10 = index_e == 0 && index_plus == 0;
    let has_one_power_of_10 = index_e != 0
        && index_plus > 0
        && index_plus == index_e + 1
        && (char_vec.len() > 1 && index_plus < char_vec.len() - 1)
        && index_e > 0;
    //println!("{} {:?}", text, text.parse::<f64>());
    match text.parse::<f64>() {
        Ok(val) => {
            if val == f64::INFINITY || val == f64::NEG_INFINITY {
                is_valid = false;
            }
        }
        Err(_) => is_valid = false,
    }
    is_valid && has_one_decimal_point && (no_power_of_10 || has_one_power_of_10)
}

fn is_string(text: &String) -> bool {
    let mut is_valid = true;
    let char_vec: Vec<char> = text.chars().collect();
    if text.len() < 2 || char_vec[0] != '"' || char_vec[text.len() - 1] != '"' {
        is_valid = false;
    }
    is_valid
}

fn concatenate_vec_strings(tokens: &Tokens) -> String {
    let mut output = "".to_string();
    for i in 0..tokens.len() {
        output = format!("{}{}", output, tokens[i]);
    }
    output
}

fn strip_leading_whitespace(input: String) -> String {
    let char_vec: Vec<char> = input.chars().collect();
    let mut checking_for_whitespace = true;
    let mut first_non_whitespace_index = 0;
    for i in 0..input.len() {
        if checking_for_whitespace {
            if !char_vec[i].is_whitespace() {
                first_non_whitespace_index = i;
                checking_for_whitespace = false;
            }
        }
    }
    if first_non_whitespace_index == 0 && checking_for_whitespace {
        // if you get to end of string and it's all whitespace return empty string
        return "".to_string();
    }
    input[first_non_whitespace_index..].to_string()
}

fn strip_trailing_whitespace(input: String) -> String {
    let char_vec: Vec<char> = input.chars().collect();
    let mut checking_for_whitespace = true;
    let mut first_non_whitespace_index = input.len();
    for i in (0..input.len()).rev() {
        if checking_for_whitespace {
            if !char_vec[i].is_whitespace() {
                first_non_whitespace_index = i + 1;
                checking_for_whitespace = false;
            }
        }
    }
    if first_non_whitespace_index == 0 && checking_for_whitespace {
        //if you get to end of string and it's all whitespace return empty string
        return "".to_string();
    }
    input[..first_non_whitespace_index].to_string()
}

#[cfg(test)]
mod tests {

    // cargo watch -x "test test_run"
    // cargo watch -x "test test_run -- --show-output"
    // cargo watch -x "test test_is_float -- --show-output"

    use super::*;
    use ast::Element;

    fn mock_config() -> Config {
        Config {
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

    #[test]
    fn test_is_integer() {
        let test_case_passes = [
            "1",
            "123",
            "1234567890",
            "9223372036854775807",
            "-1",
            "-123",
            "-1234567890",
            "-9223372036854775808",
        ];
        for test in test_case_passes {
            let input = &test.to_string();
            assert!(is_integer(input));
        }
        let test_case_fails = ["1a", "9223372036854775808", "-1a", "-9223372036854775809"];
        for test in test_case_fails {
            let input = &test.to_string();
            assert!(!is_integer(input));
        }
    }

    #[test]
    fn test_is_float() {
        let test_case_passes = [
            "1.1",
            "123.123",
            "1234567890.123456789",
            "1.7976931348623157E+308",
            "-1.1",
            "-123.123",
            "-1234567890.123456789",
            "-1.7976931348623157E+308",
        ];
        for test in test_case_passes {
            let input = &test.to_string();
            assert!(is_float(input));
        }
        let test_case_fails = [
            "123",
            "-123",
            "1.1.1",
            "1.7976931348623157E+309",
            "-1.7976931348623157E+309",
            "1.797693134E+8623157E+309",
            "-1.79769313E+48623157E+309",
        ];
        for test in test_case_fails {
            let input = &test.to_string();
            assert!(!is_float(input));
        }
    }

    #[test]
    fn test_new() {
        let args = ["toylang".to_string(), "filepath_example".to_string()];
        let config_result = Config::new(&args);
        let filepath = "filepath_example".to_string();
        match config_result {
            Ok(config) => assert_eq!(config.file.filepath, filepath),
            Err(_) => assert!(false, "error should not exist"),
        }
    }

    #[test]
    fn test_run() {
        let test_case_passes = [
            /*
            //empty file
            ["", "fn main() {\r\n}\r\n"],
            //comment single line
            ["//comment", "fn main() {\r\n    //comment\r\n}\r\n"],
            [
                "    //    comment    ",
                "fn main() {\r\n    //    comment\r\n}\r\n",
            ],
            //string
            [
                "\"string\"",
                "fn main() {\r\n    \"string\".to_string();\r\n}\r\n",
            ],
            ["\"\"", "fn main() {\r\n    \"\".to_string();\r\n}\r\n"],
            //int
            ["1", "fn main() {\r\n    1;\r\n}\r\n"],
            ["123", "fn main() {\r\n    123;\r\n}\r\n"],
            ["    123    ", "fn main() {\r\n    123;\r\n}\r\n"],
            [
                "9223372036854775807",
                "fn main() {\r\n    9223372036854775807;\r\n}\r\n",
            ],
            //int negative
            ["-1", "fn main() {\r\n    -1;\r\n}\r\n"],
            ["-123", "fn main() {\r\n    -123;\r\n}\r\n"],
            ["    -123    ", "fn main() {\r\n    -123;\r\n}\r\n"],
            [
                "-9223372036854775808",
                "fn main() {\r\n    -9223372036854775808;\r\n}\r\n",
            ],
            //float
            ["1.1", "fn main() {\r\n    1.1;\r\n}\r\n"],
            ["123.123", "fn main() {\r\n    123.123;\r\n}\r\n"],
            ["    123.123    ", "fn main() {\r\n    123.123;\r\n}\r\n"],
            [
                "1234567890.123456789",
                "fn main() {\r\n    1234567890.123456789;\r\n}\r\n",
            ],
            [
                "1.7976931348623157E+308",
                "fn main() {\r\n    1.7976931348623157E+308;\r\n}\r\n",
            ],
            //float negative
            ["-1.1", "fn main() {\r\n    -1.1;\r\n}\r\n"],
            ["-123.123", "fn main() {\r\n    -123.123;\r\n}\r\n"],
            ["    -123.123    ", "fn main() {\r\n    -123.123;\r\n}\r\n"],
            [
                "-1234567890.123456789",
                "fn main() {\r\n    -1234567890.123456789;\r\n}\r\n",
            ],
            [
                "-1.7976931348623157E+308",
                "fn main() {\r\n    -1.7976931348623157E+308;\r\n}\r\n",
            ],
            //internalFunctionCalls
            ["+ 1 2", "fn main() {\r\n    1 + 2;\r\n}\r\n"],
            ["- 1.1 2.2", "fn main() {\r\n    1.1 - 2.2;\r\n}\r\n"],
            ["/ 9 3", "fn main() {\r\n    9 / 3;\r\n}\r\n"],
            //constant
            ["a", "fn main() {\r\n    a;\r\n}\r\n"],
            ["a\r\na", "fn main() {\r\n    a;\r\n    a;\r\n}\r\n"],
            //assignment
            [
                "= a \"string\"",
                "fn main() {\r\n    let a: String = \"string\".to_string();\r\n}\r\n",
            ],
            ["= a 1", "fn main() {\r\n    let a: i64 = 1;\r\n}\r\n"],
            ["= a 1.1", "fn main() {\r\n    let a: f64 = 1.1;\r\n}\r\n"],
            [
                "= a -1.7976931348623157E+308",
                "fn main() {\r\n    let a: f64 = -1.7976931348623157E+308;\r\n}\r\n",
            ],
            [
                "= a + 1 2",
                "fn main() {\r\n    let a: i64 = 1 + 2;\r\n}\r\n",
            ],
            //nested function calls
            [
                "= a - + 1 2 3",
                "fn main() {\r\n    let a: i64 = 1 + 2 - 3;\r\n}\r\n",
            ],
            [
                "= a / * - + 1 2 3 4 5",
                "fn main() {\r\n    let a: i64 = 1 + 2 - 3 * 4 / 5;\r\n}\r\n",
            ],
            [
                "= a + 1 * 3 2",
                "fn main() {\r\n    let a: i64 = 1 + 3 * 2;\r\n}\r\n",
            ],

            //TODO handle reserved names of i64 by adding to inbuiltfndefs
            
            [
                "= a \\ i64 : 123",
                "fn main() {\r\n    fn a() -> i64 {\r\n        123\r\n    }\r\n}\r\n",
            ],
            [
                "= a \\ i64 i64 arg1 : + 123 arg1",
                "fn main() {\r\n    fn a(arg1: i64) -> i64 {\r\n        123 + arg1\r\n    }\r\n}\r\n",
            ],
            [
                "= a \\ i64 i64 i64 arg1 arg2 :\r\n+ arg1 arg2",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        arg1 + arg2\r\n    }\r\n}\r\n",
            ],*/
            [
                "= a \\ i64 i64 i64 i64 arg1 arg2 arg3 :\r\n= x + arg1 arg2\r\n+ x arg3",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64, arg3: i64) -> i64 {\r\n        let x: i64 = arg1 + arg2;\r\n        x + arg3\r\n    }\r\n}\r\n",
            ],
            /*
            [
                // interesting bug
                "= a \\ i64 i64 i64 i64 arg1 arg2 arg3 :\r\n + arg1 + arg2 arg3",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64, arg3: i64) -> i64 {\r\n        arg1 + arg2 + arg3\r\n    }\r\n}\r\n",
            ],
            [
                "= a \\ i64 i64 i64 arg1 arg2 :\r\n= arg3 + arg2 123\r\n+ arg2 arg1",
                "fn main() {\r\n    fn a(arg1: i64) -> i64 {\r\n        123 + arg1\r\n    }\r\n}\r\n",
            ]*/
        ];

        for test in test_case_passes {
            let input = test[0];
            let output = test[1];
            let mut c = mock_config();
            c.file.filecontents = input.to_string();
            match c.run_main_tasks() {
                Ok(_) => {
                    //dbg!(&c.ast, input, output);
                    assert_eq!(c.ast.output, output);
                }
                Err(_e) => assert!(false, "error should not exist"),
            }
        }

        /*
        let test_case_errors = [
            //empty file
            ["", ""],
            //comment single line
            ["/1/comment", ERRORS.comment_single_line],
            //string
            ["\"", ERRORS.string],
            ["\"\"\"", ERRORS.string],
            ["\"\" \"", ERRORS.string],
            //int
            ["1a", ERRORS.int],
            ["9223372036854775808", ERRORS.int_out_of_bounds],
            //int negative
            ["-1a", ERRORS.int],
            ["-9223372036854775809", ERRORS.int_out_of_bounds],
            //float (errors say int)
            ["1.1.1", ERRORS.int],
            ["1.7976931348623157E+309", ERRORS.int],
            //float negative (errors say int)
            ["-1.1.1", ERRORS.int],
            ["-1.7976931348623157E+309", ERRORS.int],
            //internalFunctionCalls
            //["+ 1 2.1", ERRORS.int],
            //["- 1.1 2", ERRORS.int],
            //functionDefinitions
            ["= a \\ :", ERRORS.funcdef_args],
            ["= a \\ i64 monkey i64  :", ERRORS.funcdef_argtypes_first],
        ];

        for test in test_case_errors {
            let input = test[0];
            let error = test[1];
            let mut c = mock_config();
            c.file.filecontents = input.to_string();
            match c.run_main_tasks() {
                Ok(_) => {
                    if error == "" && c.error_stack.len() == 0 {
                        assert_eq!(true, true)
                    } else {
                        assert!(c.error_stack[0].contains(error))
                    }
                }
                Err(_e) => assert!(false, "error should not exist"),
            }
        }
        */
    }

    #[test]
    fn test_ast_walk() {
        /*

        WIP attempting to generate nested output without recursing, using a stack

        Example, nested AST:

        typical nested tree         this flat ast
        0 (root)                    |_(0,[1,2,3,8]) root
        |_1                         |_(1,[])
        |_2                         |_(2,[])
        |_3                         |_(3,[4,5])
        | |_4                       |_(4,[])
        | |_5                       |_(5,[6,7])
        |   |_6                     |_(6,[])
        |   |_7                     |_(7,[])
        |_8                         |_(8,[])

        */
        //let root: Element = (ElementInfo::CommentSingleLine("root".to_string()), vec![1, 2, 3, 8]);
        // we use the 0 index (for root) to mean outdent a level
        // so all real elements start at index 1!
        let el1: Element = (ElementInfo::CommentSingleLine("1".to_string()), vec![]);
        let el2: Element = (ElementInfo::CommentSingleLine("2".to_string()), vec![]);
        let el3: Element = (ElementInfo::CommentSingleLine("3".to_string()), vec![4, 5]);
        let el4: Element = (ElementInfo::CommentSingleLine("4".to_string()), vec![]);
        let el5: Element = (ElementInfo::CommentSingleLine("5".to_string()), vec![6, 7]);
        let el6: Element = (ElementInfo::CommentSingleLine("6".to_string()), vec![]);
        let el7: Element = (ElementInfo::CommentSingleLine("7".to_string()), vec![]);
        let el8: Element = (ElementInfo::CommentSingleLine("8".to_string()), vec![]);
        let mut ast: Ast = Ast::new();
        ast.append(el1);
        ast.append(el2);
        ast.append(el3);
        ast.indent();
        ast.append(el4);
        ast.append(el5);
        ast.indent();
        ast.append(el6);
        ast.append(el7);
        ast.outdent();
        ast.outdent();
        ast.append(el8);
        assert!(true);
    }
}
