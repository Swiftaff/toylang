// TODO make most function arguments refs
mod ast;
mod file;
mod errors;
use ast::{Ast, Element, ElementInfo};
use file::File;
use errors::ERRORS;
use std::error::Error;

type Tokens = Vec<String>;
type ErrorStack = Vec<String>;

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
        //dbg!(&self.file);
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
                    eprintln!("{:?}", &self.ast);
                    eprintln!("----------\r\n\r\nTOYLANG COMPILE ERROR:");
                    for error in &self.error_stack {
                        println!("{}", error);
                    }
                    eprintln!("----------\r\n");
                } else {
                    self.ast.set_output();
                    println!(
                        "\r\nToylang compiled successfully:\r\n----------\r\n{:?}\r\n----------\r\n",
                        self.ast
                    );
                }
            }
            Err(_) => {
                eprintln!("{:?}", &self.ast);
                eprintln!("----------\r\n\r\nTOYLANG COMPILE ERROR:");
                for error in &self.error_stack {
                    println!("{}", error);
                }
                eprintln!("----------\r\n");
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
        let current_token = &tokens[self.current_line_token];
        let current_token_vec: &Vec<char> = &tokens[self.current_line_token].chars().collect();
        if current_token_vec.len() == 0 {
            return Ok(());
        }

        match self.ast.get_inbuilt_function_index_by_name(&current_token) {
            Some(index_of_function) => {
                //dbg!(&current_token);
                let func = &self.ast.elements[index_of_function];
                match &func.0 {
                    ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => {
                        self.parse_inbuilt_function_call(&current_token, index_of_function)
                    }
                    ElementInfo::FunctionDef(_, _, _, _) => {
                        self.parse_function_call(&current_token, index_of_function)
                    }
                    ElementInfo::Arg(_, _, returntype) => {
                        if returntype.contains("&dyn Fn") {
                            self.parse_function_call(&current_token, index_of_function)
                        } else {
                            self.parse_token_by_first_chars(&current_token, &current_token_vec)
                        }
                    }
                    _ => self.parse_token_by_first_chars(&current_token, &current_token_vec),
                }
            }
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
            '(' => self.parse_functiontypesig_or_functionreference_start(),
            ')' => self.parse_functiontypesig_or_functionreference_end(),
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
                Ok(())
            }
        }
    }

    fn parse_string(self: &mut Self, current_token: &String) -> Result<(), ()> {
        if is_string(&current_token) {
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
            Some(_) => {
                //check if constant already exists
                let parent = self.ast.get_current_parent_element_from_parents();
                match parent.0 {
                    ElementInfo::Assignment => {
                        let parent_assignment_has_no_children = parent.1.len() == 0;
                        if parent_assignment_has_no_children {
                            // then this constant is the first child of the assignment
                            // so it is the name of the constant (and not the value if it were the second child),
                            // and since constants are immutable it can't have the same name as a pre-existing constant
                            // so it is invalid!
                            return self.get_error2(0, 1, ERRORS.constants_are_immutable);
                        }
                    }
                    _ => (),
                }
                match el_option {
                    Some((ElementInfo::Constant(_, returntype), _)) => {
                        self.create_constant_ref(current_token, &returntype);
                        return Ok(());
                    }
                    Some((ElementInfo::Arg(_, _, returntype), _)) => {
                        //dbg!("Arg", &returntype);
                        if returntype.contains("&dyn Fn") {
                            let args = self.get_args_from_dyn_fn(&returntype);
                            self.ast.append((
                                ElementInfo::FunctionCall(current_token.clone(), returntype),
                                vec![],
                            ));
                            if args > 0 {
                                self.ast.indent();
                            }
                        } else {
                            self.create_constant_ref(current_token, &returntype);
                        }
                        return Ok(());
                    }
                    // explicitly listing other types rather than using _ to not overlook new types in future
                    Some((ElementInfo::Root, _)) => (),
                    Some((ElementInfo::CommentSingleLine(_), _)) => (),
                    Some((ElementInfo::Int(_), _)) => (),
                    Some((ElementInfo::Float(_), _)) => (),
                    Some((ElementInfo::String(_), _)) => (),
                    Some((ElementInfo::ConstantRef(_, _, _), _)) => (),
                    Some((ElementInfo::Assignment, _)) => (),
                    Some((ElementInfo::InbuiltFunctionDef(_, _, _, _, _), _)) => (),
                    Some((ElementInfo::InbuiltFunctionCall(_, _, _), _)) => (),
                    Some((ElementInfo::FunctionDefWIP, _)) => (),
                    Some((ElementInfo::FunctionDef(_, argnames, _, returntype), _)) => {
                        self.create_function_call(current_token, argnames.len(), &returntype);
                        return Ok(());
                    }
                    Some((ElementInfo::FunctionCall(_, _), _)) => (),
                    Some((ElementInfo::Parens, _)) => (),
                    Some((ElementInfo::Type(_), _)) => (),
                    Some((ElementInfo::Eol, _)) => (),
                    Some((ElementInfo::Seol, _)) => (),
                    Some((ElementInfo::Indent, _)) => (),
                    Some((ElementInfo::Unused, _)) => (),
                    None => (),
                }
            }
            None => (),
        }

        self.create_new_constant_or_arg(current_token);
        Ok(())
    }

    fn get_args_from_dyn_fn(self: &Self, string: &String) -> usize {
        string.matches(",").count() + (!string.contains("()") as usize)
        //0 args, e.g. "&dyn Fn() -> i64"         = 0 commas + 0 does match ()
        //1 args, e.g. "&dyn Fn(i64) -> i64"      = 0 commas + 1 does not match ()
        //2 args, e.g. "&dyn Fn(i64, i64) -> i64" = 1 comma  + 1 does not match ()
    }

    fn create_constant_ref(self: &mut Self, current_token: &String, returntype: &String) {
        self.indent_if_first_in_line();
        self.ast.append((
            ElementInfo::ConstantRef(current_token.clone(), returntype.clone(), current_token.clone()),
            vec![],
        ));
        self.outdent_if_last_expected_child();
        self.seol_if_last_in_line();
    }

    fn create_new_constant_or_arg(self: &mut Self, current_token: &String) {
        let typename = "Undefined".to_string();
        self.indent_if_first_in_line();
        //TODO change this to inbuiltfunction?

        let parent_ref = self.ast.get_current_parent_ref_from_parents();
        let parent = self.ast.elements[parent_ref].clone();
        match parent.0 {
            ElementInfo::FunctionDefWIP => {
                self.ast.append((
                    ElementInfo::Arg(current_token.clone(), parent_ref, "Undefined".to_string()),
                    vec![],
                ));
            }
            _ => {
                self.ast.append((
                    ElementInfo::Constant(current_token.clone(), typename),
                    vec![],
                ));
                //self.outdent_if_last_expected_child();
                self.ast.indent();
            }
        }

        //dbg!("constant 1", &self.ast.parents);
        self.outdent_if_last_expected_child();
        //dbg!("constant 2", &self.ast.parents);
        self.seol_if_last_in_line();
    }

    fn create_function_call(
        self: &mut Self,
        current_token: &String,
        args: usize,
        returntype: &String,
    ) {
        //dbg!("FunctionCall", &current_token);
        self.indent_if_first_in_line();

        let parent = self.ast.get_current_parent_element_from_parents();
        //dbg!("penguin",&parent);
        match parent.0 {
            ElementInfo::Parens => {
                // if parent is parens, then this is just a function reference
                // don't treat it like a functionCall,
                // just change the parent to be a ConstantRef

                let parent_ref = self.ast.get_current_parent_ref_from_parents();
                let new_constant_ref: Element = (
                    ElementInfo::ConstantRef(
                        current_token.clone(),
                        returntype.clone(),
                        current_token.clone(),
                    ),
                    [].to_vec(),
                );
                self.ast.elements[parent_ref] = new_constant_ref;
                //self.ast.outdent();
            }
            _ => {
                //else it is a function call...

                self.ast.append((
                    ElementInfo::FunctionCall(current_token.clone(), returntype.clone()),
                    vec![],
                ));
                if args > 0 {
                    self.ast.indent();
                }
            }
        }
        self.seol_if_last_in_line();
    }

    fn parse_assignment(self: &mut Self, _current_token: &String) -> Result<(), ()> {
        self.indent_if_first_in_line();
        self.ast.append((ElementInfo::Assignment, vec![]));
        self.outdent_if_last_expected_child();
        self.ast.indent();
        Ok(())
    }

    fn parse_inbuilt_function_call(
        self: &mut Self,
        current_token: &String,
        index_of_function: usize,
    ) -> Result<(), ()> {
        self.indent_if_first_in_line();
        let el = &self.ast.elements[index_of_function];
        let returntype = self.ast.get_elementinfo_type(&el.0);
        self.ast.append((
            ElementInfo::InbuiltFunctionCall(current_token.clone(), index_of_function, returntype),
            vec![],
        ));
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
        let el = &self.ast.elements[index_of_function];
        let returntype = self.ast.get_elementinfo_type(&el.0);
        self.ast.append((
            ElementInfo::FunctionCall(current_token.clone(), returntype),
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
        //dbg!(&self, func_def_ref_option);
        match func_def_ref_option {
            Some(func_def_ref) => {
                //get child refs
                let func_def = &self.ast.elements[func_def_ref];
                let children = func_def.1.clone();
                //dbg!(&children);
                //error if count is NOT odd (argtypes + returntype + argnames)
                if children.len() % 2 == 0 || children.len() == 0 {
                    return self.get_error2(0, 1, ERRORS.funcdef_args);
                }

                //TODO deal with brackets later (i.e. for type signature containing argument(s) which are fns)

                //error if arg types are NOT first
                let first_child_ref = children[0];

                let first_child = &self.ast.elements[first_child_ref];
                match first_child.0 {
                    ElementInfo::Type(_) => (),
                    ElementInfo::Parens => (),
                    _ => return self.get_error2(0, 1, ERRORS.funcdef_argtypes_first),
                }

                match func_def.0 {
                    ElementInfo::FunctionDefWIP => {
                        //Constant is parent of functionDefWIP
                        let constant_ref_option = self
                            .ast
                            .get_current_parent_ref_from_element_children_search(func_def_ref);

                        match constant_ref_option {
                            Some(constant_ref) => {
                                let constant = self.ast.elements[constant_ref].clone();

                                //assignment is parent of constant
                                let assignment_ref_option = self
                                    .ast
                                    .get_current_parent_ref_from_element_children_search(
                                        constant_ref,
                                    );

                                match assignment_ref_option {
                                    Some(assignment_ref) => {
                                        //let constant_ref = assignment_element.1[0];

                                        match constant.0 {
                                            ElementInfo::Constant(name, _) => {
                                                //assign name to parent funcdef (from constant
                                                let num_args = children.len() / 2;
                                                let argtype_refs = &children[..num_args];
                                                let mut argtypes: Vec<String> = vec![];
                                                for a in argtype_refs {
                                                    match &self.ast.elements[a.clone()] {
                                                        (ElementInfo::Type(typename), _) => {
                                                            argtypes.push(typename.clone())
                                                        }
                                                        (ElementInfo::Parens, paren_children) => {
                                                            if paren_children.len() > 0 {
                                                                let paren_returntype_ref =
                                                                    paren_children
                                                                        [paren_children.len() - 1];
                                                                let paren_returntype_el = &self
                                                                    .ast
                                                                    .elements[paren_returntype_ref]
                                                                    ;
                                                                let paren_returntype =
                                                                    self.ast.get_elementinfo_type(
                                                                        &paren_returntype_el.0,
                                                                    );
                                                                let paren_main_types =
                                                                    &paren_children[0
                                                                        ..paren_children.len() - 1];
                                                                let mut main_types = "".to_string();
                                                                for i in 0..paren_main_types.len() {
                                                                    let main_type_ref =
                                                                        paren_main_types[i];
                                                                    let main_type_el = &self
                                                                        .ast
                                                                        .elements[main_type_ref]
                                                                        ;
                                                                    //dbg!(&main_type_el);
                                                                    let main_type = self
                                                                        .ast
                                                                        .get_elementinfo_type(
                                                                            &main_type_el.0,
                                                                        );
                                                                    let comma = if i + 1
                                                                        == paren_main_types.len()
                                                                    {
                                                                        "".to_string()
                                                                    } else {
                                                                        ", ".to_string()
                                                                    };
                                                                    main_types = format!(
                                                                        "{}{}{}",
                                                                        main_types,
                                                                        comma,
                                                                        main_type
                                                                    );
                                                                }
                                                                let fn_type_signature = format!(
                                                                    "&dyn Fn({}) -> {}",
                                                                    main_types, paren_returntype
                                                                );
                                                                argtypes.push(fn_type_signature)
                                                            }
                                                        }
                                                        _ => (),
                                                    }
                                                }

                                                let returntype_ref = &children[num_args];
                                                let returntype: String = match &self.ast.elements
                                                    [returntype_ref.clone()]
                                                
                                                {
                                                    (ElementInfo::Type(typename), _) => typename.clone(),
                                                    _ => "Undefined".to_string(),
                                                };

                                                //get argnames from Arg tokens
                                                //but also update Arg tokens returntypes at same time
                                                //TODO make up mind about just using the Arg tokens as the definition of argnames/argtypes
                                                let argname_refs = &children[num_args + 1..];
                                                let mut argnames: Vec<String> = vec![];
                                                for i in 0..argname_refs.len() {
                                                    let a = argname_refs[i];
                                                    match &self.ast.elements[a] {
                                                        (
                                                            ElementInfo::Arg(argname, scope, _),
                                                            _,
                                                        ) => {
                                                            argnames.push(argname.clone());
                                                            let returntype = argtypes[i].clone();
                                                            let updated_arg_token =
                                                                ElementInfo::Arg(
                                                                    argname.clone(),
                                                                    scope.clone(),
                                                                    returntype,
                                                                );
                                                            self.ast.elements[a].0 =
                                                                updated_arg_token;
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

                                                //let assignment_el =
                                                //    self.ast.elements[assignment_ref].clone();
                                                //dbg!(assignment_el);
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
                                                self.ast.elements[func_def_ref] =
                                                    (new_funcdef, vec![]);

                                                //let constant_el =
                                                //    self.ast.elements[constant_ref].clone();
                                                //dbg!(constant_el);
                                                // replace constant with Unused
                                                self.ast.elements[constant_ref] =
                                                    (ElementInfo::Unused, vec![]);

                                                // replace funcdef children with Unused
                                                //for child_ref in children {
                                                //    self.ast.elements[child_ref] =
                                                //        (ElementInfo::Unused, vec![]);
                                                //}

                                                //re-add the new funcdef as latest parent, so we can continue parsing with it's child statements
                                                //dbg!(&self);

                                                self.ast.outdent();
                                                self.ast.outdent();
                                                self.ast.outdent();
                                                self.ast.indent_this(func_def_ref);
                                                //dbg!(&self.ast.parents);
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
            }
            _ => (),
        }
        self.outdent_if_last_expected_child();
        Ok(())
    }

    //TODO remember to error / or at least check if reusing arg names in nested functions

    fn parse_functiontypesig_or_functionreference_start(self: &mut Self) -> Result<(), ()> {
        self.indent_if_first_in_line();
        self.ast.append((ElementInfo::Parens, vec![]));
        self.ast.indent();
        Ok(())
    }

    fn parse_functiontypesig_or_functionreference_end(self: &mut Self) -> Result<(), ()> {
        self.ast.outdent();
        self.outdent_if_last_expected_child();
        Ok(())
    }

    fn indent_if_first_in_line(self: &mut Self) {
        //or if first part of the expression in a single line function (after the colon)
        //e.g. the "+ 123 arg1"  in "= a \\ i64 i64 arg1 : + 123 arg1"
        if self.current_line_token == 0 {
            self.ast.append((ElementInfo::Indent, vec![]));
        }
    }

    fn seol_if_last_in_line(self: &mut Self) {
        let is_last_token_in_this_line =
            self.current_line_token == self.lines_of_tokens[self.current_line].len() - 1;
        let mut is_end_of_return_statement_of_a_func_def: bool = false;

        if is_last_token_in_this_line {
            for el_index in (0..self.ast.elements.len()).rev() {
                let el = &self.ast.elements[el_index];
                match el.0 {
                    ElementInfo::Indent => {
                        // get start of current line

                        if el_index != self.ast.elements.len() - 1 {
                            let first_element_after_indent_ref = el_index + 1;
                            let parent_of_first_el_option = self
                                .ast
                                .get_current_parent_element_from_element_children_search(
                                    first_element_after_indent_ref,
                                );
                            match parent_of_first_el_option {
                                Some((ElementInfo::FunctionDef(_, _, _, _), _)) => {
                                    // confirm this line is a statement from a func def

                                    let first_element_after_indent_el =
                                        &self.ast.elements[first_element_after_indent_ref];
                                    match first_element_after_indent_el.0 {
                                        // confirm this statement is a return statement
                                        // i.e. must be one of these types
                                        ElementInfo::Int(_) => {
                                            is_end_of_return_statement_of_a_func_def = true;
                                        }
                                        ElementInfo::Float(_) => {
                                            is_end_of_return_statement_of_a_func_def = true;
                                        }
                                        ElementInfo::String(_) => {
                                            is_end_of_return_statement_of_a_func_def = true;
                                        }
                                        ElementInfo::Constant(_, _) => {
                                            is_end_of_return_statement_of_a_func_def = true;
                                        }
                                        ElementInfo::ConstantRef(_, _, _) => {
                                            is_end_of_return_statement_of_a_func_def = true;
                                        }
                                        ElementInfo::InbuiltFunctionCall(_, _, _) => {
                                            is_end_of_return_statement_of_a_func_def = true;
                                        }
                                        ElementInfo::FunctionCall(_, _) => {
                                            is_end_of_return_statement_of_a_func_def = true;
                                        }
                                        ElementInfo::Parens => {
                                            is_end_of_return_statement_of_a_func_def = true;
                                        }
                                        // explicitly listing other types rather than using _ to not overlook new types in future
                                        ElementInfo::Root => (),
                                        ElementInfo::CommentSingleLine(_) => (),
                                        ElementInfo::Arg(_, _, _) => (),
                                        ElementInfo::Assignment => (),
                                        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => (),
                                        ElementInfo::FunctionDefWIP => (),
                                        ElementInfo::FunctionDef(_, _, _, _) => (),
                                        ElementInfo::Type(_) => (),
                                        ElementInfo::Eol => (),
                                        ElementInfo::Seol => (),
                                        ElementInfo::Indent => (),
                                        ElementInfo::Unused => (),
                                    }
                                }
                                _ => (),
                            }
                            break;
                        }
                    }
                    _ => (),
                }
            }

            // if is the last return expression of a func_def
            // then don't add the semicolon, just the EOL
            if !is_end_of_return_statement_of_a_func_def {
                //self.ast.append((ElementInfo::Eol, vec![]));
                self.ast.append((ElementInfo::Seol, vec![]));
            }
        }
    }

    fn outdent_if_last_expected_child(self: &mut Self) {
        let mut prev_parents_len = 999999999;
        loop {
            //dbg!("loop", &self.ast.parents);
            if self.ast.parents.len() < 2 || self.ast.parents.len() == prev_parents_len {
                break;
            }
            prev_parents_len = self.ast.parents.len();
            let current_parent_ref = self.ast.get_current_parent_ref_from_parents();
            let current_parent = &self.ast.elements[current_parent_ref];
            //dbg!("---", &self.ast);
            match current_parent.0.clone() {
                ElementInfo::Constant(_, _) => {
                    //dbg!("Constant");
                    if current_parent.1.len() > 0 {
                        //dbg!("Constant outdent", &self.ast.parents,);
                        self.ast.outdent();
                    }
                }
                ElementInfo::Assignment => {
                    //dbg!("Assignment");
                    if current_parent.1.len() > 0 {
                        //dbg!("Assignment outdent", &self.ast.parents);
                        self.ast.outdent();
                    }
                }
                ElementInfo::InbuiltFunctionCall(_, fndefref, _) => {
                    //dbg!("InbuiltFunctionCall", &name);
                    let fndef = self.ast.elements[fndefref].clone();
                    match fndef.0 {
                        ElementInfo::InbuiltFunctionDef(_, argnames, _, _, _) => {
                            // current assumption is inbuiltfunctionCalls expect a fixed number
                            // of children to match args.
                            if current_parent.1.len() == argnames.len() {
                                //dbg!("InbuiltFunctionCall outdent", &self.ast.parents,);
                                self.ast.outdent();
                            }
                        }
                        _ => (),
                    }
                }
                ElementInfo::FunctionDef(_, _argnames, _, _) => {
                    //dbg!("FunctionDef");
                    // outdent if a return expression
                    // i.e. if previous element is an indent
                    let previous_element = self.ast.elements[self.ast.elements.len() - 2].clone(); //should be safe to subtract 2 since there should always be a root

                    // then the following are the first element in the row
                    // and they are all return expressions

                    match (previous_element.0, self.ast.get_last_element().0) {
                        (ElementInfo::Indent, ElementInfo::Int(_)) => {
                            //dbg!("FunctionDef outdent Int", &self.ast.parents,);
                            self.ast.outdent();
                            self.ast.outdent();
                        }
                        (ElementInfo::Indent, ElementInfo::Float(_)) => {
                            //dbg!("FunctionDef outdent Float", &self.ast.parents,);
                            self.ast.outdent();
                            self.ast.outdent();
                        }
                        (ElementInfo::Indent, ElementInfo::String(_)) => {
                            //dbg!("FunctionDef outdent String", &self.ast.parents,);
                            self.ast.outdent();
                            self.ast.outdent();
                        }
                        (ElementInfo::Indent, ElementInfo::Constant(_, _)) => {
                            //dbg!("FunctionDef outdent Constant", &self.ast.parents,);
                            self.ast.outdent();
                            self.ast.outdent();
                        }
                        (ElementInfo::Indent, ElementInfo::ConstantRef(_, _, _)) => {
                            //dbg!("FunctionDef outdent ConstantRef", &self.ast.parents,);
                            self.ast.outdent();
                            self.ast.outdent();
                        }
                        (ElementInfo::Indent, ElementInfo::InbuiltFunctionCall(_, fndefref, _)) => {
                            //dbg!("InbuiltFunctionCall");
                            let fndef = &self.ast.elements[fndefref];
                            match &fndef.0 {
                                ElementInfo::InbuiltFunctionDef(_, argnames, _, _, _) => {
                                    // current assumption is inbuiltFunctionCalls expect a fixed number
                                    // of children to match args
                                    if fndef.1.len() == argnames.len() {
                                        //dbg!(
                                        //    "FunctionDef outdent InbuiltFunctionCall enough args",
                                        //    &self.ast.parents,
                                        //);
                                        self.ast.outdent();
                                    }
                                    self.ast.outdent();
                                    self.ast.outdent();
                                }
                                _ => (),
                            }
                        }
                        (ElementInfo::Indent, ElementInfo::FunctionCall(name, _)) => {
                            let fn_index = self.ast.get_function_index_by_name(&name);
                            match fn_index {
                                Some(index) => {
                                    let fndef = &self.ast.elements[index];
                                    match &fndef.0 {
                                        ElementInfo::FunctionDef(_, argnames, _, _) => {
                                            // current assumption is functionCalls expect a fixed number
                                            // of children to match args
                                            if fndef.1.len() == argnames.len() {
                                                //dbg!(
                                                //    "FunctionDef outdent FunctionCall enough args",
                                                //    &self.ast.parents,
                                                //);
                                                self.ast.outdent();
                                            }
                                            self.ast.outdent();
                                            self.ast.outdent();
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
                ElementInfo::FunctionCall(name, _) => {
                    //dbg!("FunctionCall", &name);
                    let fn_index = self.ast.get_function_index_by_name(&name);
                    match fn_index {
                        Some(index) => {
                            let fndef = &self.ast.elements[index];
                            match &fndef.0 {
                                ElementInfo::FunctionDef(_, argnames, _, _) => {
                                    if current_parent.1.len() > 0
                                        && current_parent.1.len() == argnames.len()
                                    {
                                        self.ast.outdent();
                                        self.ast.outdent();
                                    }
                                }
                                ElementInfo::Arg(_, _, returntype) => {
                                    let args = self.get_args_from_dyn_fn(&returntype);
                                    if current_parent.1.len() > 0 && current_parent.1.len() == args
                                    {
                                        self.ast.outdent();
                                        self.ast.outdent();
                                    }
                                }
                                _ => (),
                            }
                        }
                        _ => (), // something went wrong
                    }
                }
                // explicitly listing other types rather than using _ to not overlook new types in future
                ElementInfo::Root => (),
                ElementInfo::CommentSingleLine(_) => (),
                ElementInfo::Int(_) => (),
                ElementInfo::Float(_) => (),
                ElementInfo::String(_) => (),
                ElementInfo::Arg(_, _, _) => (),
                ElementInfo::ConstantRef(_, _, _) => (),
                ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => (),
                ElementInfo::FunctionDefWIP => (),
                ElementInfo::Parens => (),
                ElementInfo::Type(_) => (),
                ElementInfo::Eol => (),
                ElementInfo::Seol => (),
                ElementInfo::Indent => (),
                ElementInfo::Unused => (),
            }
        }
    }

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

            // split line at colon for single line functions (after args, before body of function)
            // except if part of a comment in which case ignore
            let this_line_so_far = char_vec[index_from..index_to].to_vec();
            let is_a_comment_line = this_line_so_far.len() > 1
                && this_line_so_far[0] == '/'
                && this_line_so_far[1] == '/';
            let is_colon_for_singlelinefunction = c == ':' && !is_a_comment_line;

            if c == '\r' || c == '\n' || eof || is_colon_for_singlelinefunction {
                self.lines_of_chars.push(
                    char_vec[index_from
                        ..index_to
                            + (if eof || is_colon_for_singlelinefunction {
                                1
                            } else {
                                0
                            })]
                        .to_vec(),
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

            let char_vec_initial: &Vec<char> = &self.lines_of_chars[line];
            let char_as_string = char_vec_initial.iter().collect::<String>();
            let removed_leading_whitespace = strip_leading_whitespace(&char_as_string);
            let removed_trailing_whitespace = strip_trailing_whitespace(&removed_leading_whitespace);
            let char_vec: Vec<char> = removed_trailing_whitespace.chars().collect();

            let mut inside_quotes = false;
            let mut line_of_tokens: Tokens = vec![];
            while index_to < char_vec.len() {
                let c = char_vec[index_to];
                let eof = index_to == char_vec.len() - 1;
                if c == '"' {
                    inside_quotes = !inside_quotes;
                    count_quotes = count_quotes + 1;
                };
                let is_comment = char_vec.len() > 1 && char_vec[0] == '/' && char_vec[1] == '/';
                if (c.is_whitespace() && index_to != 0 && !inside_quotes && !is_comment)
                    || eof
                    || count_quotes == 2
                {
                    let token_chars = char_vec
                        [index_from..index_to + (if eof || count_quotes == 2 { 1 } else { 0 })]
                        .iter()
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
        //dbg!(&self.lines_of_tokens);
    }

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
                .collect::<String>(),
            " ".repeat(arrow_indent),
            "^".repeat(arrow_len),
            error,
        );
        self.error_stack.push(e);
        Err(())
    }
}

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

fn strip_leading_whitespace(input: &String) -> String {
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

fn strip_trailing_whitespace(input: &String) -> String {
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
    fn test_run() {
        let test_case_passes = [
            //empty file
            ["", "fn main() {\r\n}\r\n"],
            //comment single line
            ["//comment", "fn main() {\r\n    //comment\r\n}\r\n"],
            [
                "    //    comment    ",
                "fn main() {\r\n    //    comment\r\n}\r\n",
            ],
            //single line function no longer breaks comments
            [
                "//= a \\ i64 : 123",
                "fn main() {\r\n    //= a \\ i64 : 123\r\n}\r\n",
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
            //basic arithmetic, assignment, type inference
            [
                "= a + 1 2",
                "fn main() {\r\n    let a: i64 = 1 + 2;\r\n}\r\n",
            ],
            [
                "= a + 1.1 2.2",
                "fn main() {\r\n    let a: f64 = 1.1 + 2.2;\r\n}\r\n",
            ],
            [
                "= a - 1 2",
                "fn main() {\r\n    let a: i64 = 1 - 2;\r\n}\r\n",
            ],
            [
                "= a - 1.1 2.2",
                "fn main() {\r\n    let a: f64 = 1.1 - 2.2;\r\n}\r\n",
            ],
            [
                "= a * 1 2",
                "fn main() {\r\n    let a: i64 = 1 * 2;\r\n}\r\n",
            ],
            [
                "= a * 1.1 2.2",
                "fn main() {\r\n    let a: f64 = 1.1 * 2.2;\r\n}\r\n",
            ],
            [
                "= a / 1 2",
                "fn main() {\r\n    let a: i64 = 1 / 2;\r\n}\r\n",
            ],
            [
                "= a / 1.1 2.2",
                "fn main() {\r\n    let a: f64 = 1.1 / 2.2;\r\n}\r\n",
            ],
            [
                "= a % 1 2",
                "fn main() {\r\n    let a: i64 = 1 % 2;\r\n}\r\n",
            ],
            [
                "= a % 1.1 2.2",
                "fn main() {\r\n    let a: f64 = 1.1 % 2.2;\r\n}\r\n",
            ],
            //constant
            ["a", "fn main() {\r\n    a;\r\n}\r\n"],
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
            //assignment internalFunctionCalls with references
            [
                "= a + 1 2\r\n= b - 3 a",
                "fn main() {\r\n    let a: i64 = 1 + 2;\r\n    let b: i64 = 3 - a;\r\n}\r\n",
            ],
            //nested internalFunctionCalls
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

            //function definitions
            //function definitions - single line
            [
                "= a \\ i64 : 123",
                "fn main() {\r\n    fn a() -> i64 {\r\n        123\r\n    }\r\n}\r\n",
            ],
            [
                "= a \\ i64 i64 arg1 : + 123 arg1",
                "fn main() {\r\n    fn a(arg1: i64) -> i64 {\r\n        123 + arg1\r\n    }\r\n}\r\n",
            ],
            //function definitions - multiline
            [
                "= a \\ i64 i64 i64 arg1 arg2 :\r\n+ arg1 arg2",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        arg1 + arg2\r\n    }\r\n}\r\n",
            ],
            [
                "= a \\ i64 i64 i64 i64 arg1 arg2 arg3 :\r\n= x + arg1 arg2\r\n+ x arg3",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64, arg3: i64) -> i64 {\r\n        let x: i64 = arg1 + arg2;\r\n        x + arg3\r\n    }\r\n}\r\n",
            ],
            //function definitions - multiline, nested function calls
            [
                "= a \\ i64 i64 i64 i64 arg1 arg2 arg3 :\r\n + arg1 + arg2 arg3",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64, arg3: i64) -> i64 {\r\n        arg1 + arg2 + arg3\r\n    }\r\n}\r\n",
            ],
            //function definitions - multiline, constant assignment, nested function calls
            [
                "= a \\ i64 i64 i64 arg1 arg2 :\r\n= arg3 + arg2 123\r\n+ arg3 arg1",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        let arg3: i64 = arg2 + 123;\r\n        arg3 + arg1\r\n    }\r\n}\r\n",
            ],
            //function definitions - multiline, several semicolon statements, with final return statement
            [
                "= a \\ i64 i64 i64 arg1 arg2 :\r\n= b + arg1 123\r\n= c - b arg2\r\n= z * c 10\r\nz",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        let b: i64 = arg1 + 123;\r\n        let c: i64 = b - arg2;\r\n        let z: i64 = c * 10;\r\n        z\r\n    }\r\n}\r\n",
            ],
            //function definitions - pass functions as arguments
            //arg1 is a function that takes i64 returns i64, arg2 is an i64
            //the function body calls arg1 with arg2 as its argument, returning which returns i64
            [
                "= a \\ ( i64 i64 ) i64 i64 arg1 arg2 :\r\n arg1 arg2\r\n= b \\ i64 i64 arg3 : + 123 arg3\r\n= c a ( b ) 456",
                "fn main() {\r\n    fn a(arg1: &dyn Fn(i64) -> i64, arg2: i64) -> i64 {\r\n        arg1(arg2)\r\n    }\r\n    fn b(arg3: i64) -> i64 {\r\n        123 + arg3\r\n    }\r\n    let c: i64 = a(&b, 456);\r\n}\r\n",
            ],
            //type inference
            //type inference - assignment to constantrefs
            [
                "= a 123\r\n= aa a\r\n= aaa aa\r\n= aaaa aaa",
                "fn main() {\r\n    let a: i64 = 123;\r\n    let aa: i64 = a;\r\n    let aaa: i64 = aa;\r\n    let aaaa: i64 = aaa;\r\n}\r\n",
            ],
            //type inference - assignment to function call
            [
                "= a + 1 2",
                "fn main() {\r\n    let a: i64 = 1 + 2;\r\n}\r\n",
            ],
            //type inference - assignment to constantrefs of function call
            [
                "= a + 1 2\r\n= aa a\r\n= aaa aa\r\n= aaaa aaa",
                "fn main() {\r\n    let a: i64 = 1 + 2;\r\n    let aa: i64 = a;\r\n    let aaa: i64 = aa;\r\n    let aaaa: i64 = aaa;\r\n}\r\n",
            ],
            //function calls - zero arguments
            [
                "//define function\r\n= a \\ i64 :\r\n123\r\n\r\n//call function\r\na",
                "fn main() {\r\n    //define function\r\n    fn a() -> i64 {\r\n        123\r\n    }\r\n    //call function\r\n    a();\r\n}\r\n",
            ],
            //function calls - one argument
            [
                "//define function\r\n= a \\ i64 i64 arg1 :\r\narg1\r\n\r\n//call function\r\na 123",
                "fn main() {\r\n    //define function\r\n    fn a(arg1: i64) -> i64 {\r\n        arg1\r\n    }\r\n    //call function\r\n    a(123);\r\n}\r\n",
            ],
            //function calls - two arguments, where one is an evaluated internal function call
            [
                "//define function\r\n= a \\ i64 i64 i64 arg1 arg2 :\r\n+ arg1 arg2\r\n\r\n//call function\r\na + 123 456 789",
                "fn main() {\r\n    //define function\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        arg1 + arg2\r\n    }\r\n    //call function\r\n    a(123 + 456, 789);\r\n}\r\n",
            ] 
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
            //["= a \\ :", ERRORS.funcdef_args],
            //["= a \\ i64 monkey i64  :", ERRORS.funcdef_argtypes_first],
            //constants are immutable
            ["= a 123\r\n= a 234", ERRORS.constants_are_immutable],
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

    // cargo watch -x "test"
    // cargo watch -x "test test_run"
    // cargo watch -x "test test_run -- --show-output"
    // cargo watch -x "test test_is_float -- --show-output"

    // cd target/debug
    // cargo build
    // toylang ../../src/test.toy
}
