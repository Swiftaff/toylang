// TODO make most function arguments refs
mod ast;
mod file;
use ast::{Ast, ElementInfo};
use file::File;
use std::error::Error;

type Tokens = Vec<String>;
type ErrorStack = Vec<String>;
type FullOrValidationError = bool;

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
    //variable_assignment: &'static str,
    no_valid_comment_single_line: &'static str,
    no_valid_int: &'static str,
    no_valid_float: &'static str,
    no_valid_string: &'static str,
    no_valid_assignment: &'static str,
    no_valid_integer_arithmetic: &'static str,
    no_valid_expression: &'static str,
    //constants_are_immutable: &'static str,
}

const ERRORS: Errors = Errors {
    //variable_assignment: "Invalid variable assignment. Must contain Int or Float, e.g. x = Int 2",
    no_valid_comment_single_line: "No valid single line comment found",
    no_valid_int: "No valid integer found",
    no_valid_float: "No valid float found",
    no_valid_string: "No valid string found",
    no_valid_assignment: "No valid assignment found",
    no_valid_integer_arithmetic: "No valid integer arithmetic found",
    no_valid_expression: "No valid expression was found",
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

        match self.main_loop_over_lines_of_tokens() {
            Ok(_) => {
                ////dbg!(&self.ast);
                if self.error_stack.len() > 0 {
                    println!("----------\r\n\r\nTOYLANG COMPILE ERROR:");
                    for error in self.error_stack.clone() {
                        println!("{}", error);
                    }
                    println!("----------\r\n");
                } else {
                    self.ast.set_output();
                    println!(
                        "Toylang compiled successfully:\r\n----------\r\n{}\r\n----------\r\n",
                        self.ast.output
                    );
                }
            }
            Err(_) => {
                println!("----------\r\n\r\nTOYLANG COMPILE ERROR:");
                for error in self.error_stack.clone() {
                    println!("{}", error);
                }
                println!("----------\r\n");
            }
        };
        Ok(())
    }

    fn main_loop_over_lines_of_tokens(self: &mut Self) -> Result<(), FullOrValidationError> {
        //self.set_ast_output_for_main_fn_start();
        for line in 0..self.lines_of_tokens.len() {
            if self.lines_of_tokens[line].len() > 0 {
                println!("line: {}", line);
                self.current_line = line;
                self.current_line_token = 0;
                self.parse_current_line()?;
                println!("end of line: {}\r\n", line);
            }
        }
        Ok(())
    }

    fn parse_current_line(self: &mut Self) -> Result<(), FullOrValidationError> {
        let tokens = self.lines_of_tokens[self.current_line].clone();
        if tokens.len() > 0 {
            while self.current_line_token < tokens.len() {
                self.parse_current_token(&tokens)?;
                self.current_line_token = self.current_line_token + 1;
            }
        }
        Ok(())
    }

    fn parse_current_token(self: &mut Self, tokens: &Tokens) -> Result<(), FullOrValidationError> {
        let current_token = tokens[self.current_line_token].clone();
        let first_token_vec: &Vec<char> = &tokens[self.current_line_token].chars().collect();
        let first_char = first_token_vec[0];
        //dbg!(&current_token, first_char);
        match first_char {
            '=' => {
                dbg!("assign");
                Ok(())
            }
            '"' => {
                dbg!("string");
                Ok(())
            }
            '\\' => {
                dbg!("func_def");
                Ok(())
            }
            first_char if is_integer(&first_char.to_string()) => {
                if is_integer(&current_token) {
                    dbg!("integer");
                    Ok(())
                } else if is_float(&current_token) {
                    dbg!("float");
                    Ok(())
                } else {
                    dbg!("UNKNOWN");
                    Ok(())
                }
            }
            first_char if "abcdefghijklmnopqrstuvwxyz".contains(&first_char.to_string()) => {
                dbg!("constant");
                Ok(())
            }
            _ => match self.ast.get_inbuilt_function_index_by_name(&current_token) {
                Some(i) => {
                    println!("func_call: {}", i);
                    Ok(())
                }
                _ => {
                    dbg!("UNKNOWN");
                    Ok(())
                }
            },
        }
    }

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
            if c == '\r' || c == '\n' || eof {
                self.lines_of_chars
                    .push(char_vec[index_from..index_to + (if eof { 1 } else { 0 })].to_vec());
                index_from = index_to + incr;
            }
            index_to = index_to + incr;
        }
    }

    fn set_lines_of_tokens(self: &mut Self) {
        for line in 0..self.lines_of_chars.len() {
            let mut index_from = 0;
            let mut index_to = 0;

            let char_vec_initial: Vec<char> = self.lines_of_chars[line].clone();
            let char_as_string = char_vec_initial.iter().cloned().collect::<String>();
            let removed_leading_whitespace = strip_leading_whitespace(char_as_string);
            let char_vec: Vec<char> = removed_leading_whitespace.chars().collect();

            let mut inside_quotes = false;
            let mut line_of_tokens: Tokens = vec![];
            while index_to < char_vec.len() {
                let c = char_vec[index_to];
                let eof = index_to == char_vec.len() - 1;
                inside_quotes = if c == '"' {
                    !inside_quotes
                } else {
                    inside_quotes
                };
                let is_comment = char_vec.len() > 1 && char_vec[0] == '/' && char_vec[1] == '/';

                if (c.is_whitespace() && index_to != 0 && !inside_quotes && !is_comment) || eof {
                    let token_chars = char_vec[index_from..index_to + (if eof { 1 } else { 0 })]
                        .iter()
                        .cloned()
                        .collect::<String>();
                    line_of_tokens.push(token_chars);
                    index_from = index_to + 1;
                }
                index_to = index_to + 1;
            }

            self.lines_of_tokens.push(line_of_tokens);
        }
    }

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
            self.get_error_ok(0, 1, ERRORS.no_valid_comment_single_line, true)
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
            return self.get_error_ok(0, 1, ERRORS.no_valid_int, true);
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
            return self.get_error_ok(0, 1, ERRORS.no_valid_int, true);
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
            self.get_error_ok(0, 1, ERRORS.no_valid_float, true)
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
            self.get_error_ok(0, 1, ERRORS.no_valid_string, true)
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
                //create a new constant
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
}

fn tokens_remove_head(tokens: Tokens) -> Tokens {
    if tokens.len() == 1 {
        vec![]
    } else {
        tokens[1..].to_vec()
    }
}

fn is_integer(text: &String) -> bool {
    let mut is_valid = true;
    if !text.chars().into_iter().all(|c| c.is_numeric()) {
        is_valid = false;
    }
    is_valid
}

fn is_float(text: &String) -> bool {
    let mut is_valid = true;
    let mut count_decimal_points = 0;
    let char_vec: Vec<char> = text.chars().collect();
    for i in 0..text.len() {
        let c = char_vec[i];
        if c == '.' {
            count_decimal_points = count_decimal_points + 1;
        } else {
            if !c.is_numeric() {
                is_valid = false;
            }
        }
    }
    is_valid && count_decimal_points == 1
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

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Element;

    fn mock_config(_contents: &str) -> Config {
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
        let test_cases = [
            ["//comment", "fn main() {\r\n    //comment\r\n}\r\n}"],
            /*
            ["123", "fn main() {\r\n    123\r\n}\r\n}"],
            ["= a 123", "fn main() {\r\n    let a: i64 = 123;\r\n}"],
            ["= b 123.45", "fn main() {\r\n    let b: f64 = 123.45;\r\n}"],
            [
                "= c \"a string\"",
                "fn main() {\r\n    let c: String = \"a string\".to_string();\r\n}",
            ],
            [
                "= e + 1 2\r\n",
                "fn main() {\r\n    let e: i64 = 1 + 2;\r\n}",
            ],
            */
        ];
        for test in test_cases {
            let input = test[0];
            let output = test[1]; //wrong, needs trailing semicolon, no extra brace
            let mut c = mock_config(input);
            match c.run_main_tasks() {
                Ok(_) => assert_eq!(c.ast.output, output),
                Err(_e) => {
                    //dbg!(c, e);
                    assert!(false, "error should not exist");
                }
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
}
