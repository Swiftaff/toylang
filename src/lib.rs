// TODO make most function arguments refs
mod ast;
mod file;
use ast::{Ast, ElementInfo};
use file::File;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct Config {
    pub file: File,
    pub lines_of_chars: Vec<Vec<char>>,
    pub lines_of_tokens: Vec<Vec<String>>,
    pub output: String,
    pub current_line: usize,
    pub error_stack: Vec<String>,
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
        let error_stack = vec![];
        let ast = Ast::new();
        Ok(Config {
            file,
            lines_of_chars,
            lines_of_tokens,
            output,
            current_line,
            error_stack,
            ast,
        })
    }

    pub fn run(self: &mut Self, args: &[String]) -> Result<(), Box<dyn Error>> {
        self.file.get(args)?;
        dbg!(self.file.clone());
        match self.run_main_tasks() {
            Ok(_) => (),
            Err(_e) => (),
        }
        self.file
            .writefile_or_error(&self.ast.output, self.error_stack.len() > 0)
    }

    pub fn run_main_tasks(self: &mut Self) -> Result<(), String> {
        self.set_lines_of_chars();
        self.set_lines_of_tokens();
        self.run_main_loop()
    }

    fn run_main_loop(self: &mut Self) -> Result<(), String> {
        // ref: https://doc.rust-lang.org/reference/tokens.html

        match self.main_loop_over_lines_of_tokens() {
            Ok(()) => {
                ////dbg!(&self.ast);
                self.ast.set_output();
                println!(
                    "Toylang compiled successfully:\r\n----------\r\n{}\r\n----------\r\n",
                    self.ast.output
                );
                ////dbg!(&self.ast.elements[0].1);

                let len = self.ast.elements[0].1.len();
                for i in 0..len {
                    let _refer = self.ast.elements[0].1[i];
                    ////dbg!(i, refer, &self.ast.elements[refer]);
                }
            }
            Err(e) => {
                println!("----------\r\n\r\nTOYLANG COMPILE ERROR:");
                for error in e {
                    println!("{}", error);
                }
                println!("----------\r\n");
            }
        };
        Ok(())
    }

    fn main_loop_over_lines_of_tokens(self: &mut Self) -> Result<(), Vec<String>> {
        //self.set_ast_output_for_main_fn_start();
        for line in 0..self.lines_of_tokens.len() {
            if self.lines_of_tokens[line].len() > 0 {
                self.current_line = line;
                let tokens = self.lines_of_tokens[self.current_line].clone();
                self.check_one_or_more_succeeds(tokens)?;
            }
        }
        //self.set_ast_output_for_main_fn_end();
        Ok(())
    }

    fn check_one_or_more_succeeds(self: &mut Self, tokens: Vec<String>) -> Result<(), Vec<String>> {
        match self.check_one_succeeds("ast_set_comment_single_line", &tokens, None, true) {
            Ok(_) => return Ok(()),
            _ => (),
        }
        match self.check_one_succeeds("ast_set_int", &tokens, None, true) {
            Ok(_) => {
                self.ast.append((ElementInfo::Seol, vec![]));
                return Ok(());
            }
            _ => (),
        }
        match self.check_one_succeeds("ast_set_float", &tokens, None, true) {
            Ok(_) => {
                self.ast.append((ElementInfo::Seol, vec![]));
                return Ok(());
            }
            _ => (),
        }
        match self.check_one_succeeds("ast_set_string", &tokens, None, true) {
            Ok(_) => {
                self.ast.append((ElementInfo::Seol, vec![]));
                return Ok(());
            }
            _ => (),
        }
        match self.check_one_succeeds("ast_set_constant", &tokens, None, true) {
            Ok(_) => {
                self.ast.append((ElementInfo::Seol, vec![]));
                return Ok(());
            }
            _ => (),
        }
        match self.check_one_succeeds("ast_set_inbuilt_function", &tokens, None, true) {
            Ok(_) => return Ok(()),
            _ => (),
        }

        //if self.check_one_succeeds("get_expression_result_for_variable_assignment") {
        //    return Ok(());
        //}
        //if self.check_one_succeeds("get_expression_result_for_expression") {
        //    return Ok(());
        //}
        let e = self.get_error(0, 1, ERRORS.no_valid_expression);
        self.error_stack.push(e.to_string());
        Err(self.error_stack.clone())
    }

    fn check_one_or_more_succeeds_for_returntypes(
        self: &mut Self,
        tokens: Vec<String>,
        returntype: String,
    ) -> Result<Vec<String>, Vec<String>> {
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
        let e = self.get_error(0, 1, ERRORS.no_valid_expression);
        self.error_stack.push(e.to_string());
        Err(self.error_stack.clone())
    }

    fn check_one_succeeds(
        self: &mut Self,
        function_name: &str,
        tokens: &Vec<String>,
        returntype: Option<String>,
        singleline: bool,
    ) -> Result<Vec<String>, String> {
        let mut _succeeded = false;
        let mut clone = self.clone();
        let result = match function_name {
            /*
            "get_expression_result_for_variable_assignment" => {
                clone.get_expression_result_for_variable_assignment()
            }
            "get_expression_result_for_expression" => clone.get_expression_result_for_expression(tokens),
            */
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
        match result {
            Ok(vec_string) => {
                self.set_all_from_clone(clone);
                Ok(vec_string)
            }
            Err(e) => Err(e), // println!("error {:?}", e), // self.error_stack.push(e), // just testing - move to a temporary error_stack
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
            let mut line_of_tokens: Vec<String> = vec![];
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
        tokens: &Vec<String>,
    ) -> Result<Vec<String>, String> {
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
            return Err(ERRORS.no_valid_comment_single_line.to_string());
        }
    }

    fn ast_set_int(
        self: &mut Self,
        tokens: &Vec<String>,
        singleline: bool,
    ) -> Result<Vec<String>, String> {
        //dbg!("set_int");
        if tokens.len() > 0 && is_integer(&tokens[0].to_string()) {
            let val = tokens[0].clone();
            if singleline {
                self.ast.append((ElementInfo::Indent, vec![]));
            }
            let _x = self.ast.append((ElementInfo::Int(val), vec![]));
            //dbg!(self.ast.elements[x].clone());
            if singleline {
                self.ast.append((ElementInfo::Seol, vec![]));
            }
            Ok(tokens_remove_head(tokens.clone()))
            //let validation_error = None;
            //Ok(validation_error)
        } else {
            return Err(ERRORS.no_valid_int.to_string());
        }
    }

    fn ast_set_float(
        self: &mut Self,
        tokens: &Vec<String>,
        singleline: bool,
    ) -> Result<Vec<String>, String> {
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
            return Err(ERRORS.no_valid_float.to_string());
        }
    }

    fn ast_set_string(
        self: &mut Self,
        tokens: &Vec<String>,
        singleline: bool,
    ) -> Result<Vec<String>, String> {
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
            return Err(ERRORS.no_valid_string.to_string());
        }
    }

    fn ast_set_constant(self: &mut Self, tokens: &Vec<String>) -> Result<Vec<String>, String> {
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

                                                        /*
                                                        //fix constant first
                                                        if typename.contains("|") {
                                                            let previously_saved_constant =
                                                                self.ast.elements[ref_of_constant].clone();
                                                            //dbg!("a", &previously_saved_constant);
                                                            match previously_saved_constant {
                                                                (
                                                                    ElementInfo::Constant(constant_name, _),
                                                                    constant_children,
                                                                ) => {
                                                                    let new_constant_with_corrected_type = (
                                                                        ElementInfo::Constant(
                                                                            constant_name,
                                                                            arg_types[0].clone(),
                                                                        ),
                                                                        constant_children,
                                                                    );
                                                                    self.ast.elements[ref_of_value] =
                                                                        new_constant_with_corrected_type;
                                                                    //dbg!("b", new_constant_with_corrected_type);
                                                                    ()
                                                                }
                                                                _ => {
                                                                    //dbg!("c");
                                                                    ()
                                                                }
                                                            }
                                                        }
                                                        */

                                                        //self.ast.append_as_ref((ElementInfo::Indent, vec![]));
                                                    }
                                                    Err(_e) => {
                                                        return Err(ERRORS
                                                            .no_valid_assignment
                                                            .to_string())
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
                            Err(ERRORS.no_valid_assignment.to_string())
                        }
                    }
                }
            }
        } else {
            return Err(ERRORS.no_valid_assignment.to_string());
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
        tokens: &Vec<String>,
        required_return_type_option: Option<String>,
    ) -> Result<Vec<String>, String> {
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
                        return Err(ERRORS.no_valid_integer_arithmetic.to_string());
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
                        return Err(ERRORS.no_valid_integer_arithmetic.to_string());
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
            return Err(ERRORS.no_valid_integer_arithmetic.to_string());
        } else {
            return Err(ERRORS.no_valid_integer_arithmetic.to_string());
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

    /*
    fn set_output_for_return_expression(self: &mut Self, tokens: &Vec<String>) {
        // if we found an expression while inside a function, then it must be the returning expression
        // so we should close this function brace and move scope back up a level
        //dbg!("close }", &self.current_scope);
        //if self.indent > 0 {
        self.indent = self.indent - 1;
        //}
        let trailing_brace = format!("{}}}\r\n", " ".repeat(self.indent * 4));
        let option_parent_scope = self
            .get_option_function_definition(self.current_scope.clone(), self.current_scope.clone());
        match option_parent_scope.clone() {
            Some(parent_scope) => self.current_scope = parent_scope.scope,
            None => (), // couldn't find function called self.current_scope - so um, leave as is, or maybe default to main??
        }
        //dbg!("close }", &self.current_scope, option_parent_scope);
        //self.current_scope = self.parent_scope.clone(); // TODO hm will only work for 1 level
        let insert = format!(
            "{}{}\r\n{}",
            " ".repeat((self.indent + 1) * 4),
            concatenate_vec_strings(tokens),
            trailing_brace
        );
        self.output.insert_str(self.outputcursor, &insert);
        self.outputcursor = self.outputcursor + insert.len();
    }

    fn set_output_for_plain_expression(self: &mut Self, tokens: &Vec<String>) {
        //}, expression: &String) {
        ////dbg!("plain_expression", &expression, &tokens);
        let insert = format!(
            "{}{};\r\n",
            " ".repeat(self.indent * 4),
            //expression
            concatenate_vec_strings(&tokens)
        );
        self.output.insert_str(self.outputcursor, &insert);
        self.outputcursor = self.outputcursor + insert.len();
    }

    fn set_output_for_variable_assignment(
        self: &mut Self,
        identifier: &String,
        single_line_expression: &String,
        final_type: &String,
    ) {
        let type_colon = if final_type.len() == 0 { "" } else { ": " };
        let insert = format!(
            "{}let {}{}{} = {};\r\n",
            " ".repeat(self.indent * 4),
            identifier,
            type_colon,
            final_type,
            single_line_expression
        );
        self.output.insert_str(self.outputcursor, &insert);
        self.outputcursor = self.outputcursor + insert.len();
    }

    fn set_output_for_function_definition_singleline(
        self: &mut Self,
        identifier: &String,
        single_line_function_expression: &String,
    ) {
        let insert = format!(
            "{}fn {}{}",
            " ".repeat(self.indent * 4),
            identifier,
            single_line_function_expression
        );
        self.output.insert_str(self.outputcursor, &insert);
        self.outputcursor = self.outputcursor + insert.len();
    }

    fn set_output_for_function_definition_singleline_or_firstline_of_multi(
        self: &mut Self,
        identifier: &String,
        single_line_function_expression: &String,
    ) {
        let insert = format!(
            "{}fn {}{}",
            " ".repeat(self.indent * 4),
            identifier,
            single_line_function_expression
        );
        self.output.insert_str(self.outputcursor, &insert);
        self.outputcursor = self.outputcursor + insert.len();
    }

    fn set_functions_for_func_args(
        self: &mut Self,
        identifier: &String,
        args: &Vec<String>,
        type_signature: &Vec<String>,
        func_body: &Vec<String>,
    ) -> Result<(Expression, ExpressionType), String> {
        for a in 0..args.len() {
            let new_arg = FunctionDefinition {
                name: args[a].to_string(),
                format: args[a].clone(),
                types: vec![],
                validations: vec![],
                return_type: type_signature[a].clone(),
                scope: identifier.clone(),
            };
            self.functions.push(new_arg);
        }

        // switch scope
        let temp_scope = self.current_scope.clone();
        self.current_scope = identifier.clone();
        let body2 = func_body.clone();
        let expression_result = self.get_expression_result(&identifier, body2);
        self.current_scope = temp_scope;
        expression_result
    }

    fn set_functions_for_constant(
        self: &mut Self,
        identifier: String,
        value: String,
        final_type: String,
    ) {
        let new_constant_function = FunctionDefinition {
            name: identifier.clone(),
            format: value,
            types: vec![],
            validations: vec!["is_constant".to_string()],
            return_type: final_type,
            scope: self.current_scope.clone(),
        };
        self.functions.push(new_constant_function);
        if &identifier == &"k2".to_string() {
            //dbg!(&self.functions, &self.current_scope);
        }
    }

    fn get_expression_result(
        self: &mut Self,
        identifier: &String,
        tokens: Vec<String>,
    ) -> Result<(Expression, ExpressionType), String> {
        //dbg!(&tokens);
        if tokens.len() == 1 {
            if is_integer(&tokens[0]) {
                return Ok((tokens[0].clone(), "i64".to_string()));
            }
            if is_float(&tokens[0]) {
                return Ok((tokens[0].clone(), "f64".to_string()));
            }
            let possible_string: Vec<char> = tokens[0].chars().collect();
            if possible_string[0] == '\"' && possible_string[possible_string.len() - 1] == '\"' {
                return Ok((
                    format!("{}{}", tokens[0], ".to_string()",).to_string(),
                    "String".to_string(),
                ));
            }
        }
        if tokens.len() > 0 {
            ////dbg!(
            //    &tokens,
            //    self.get_exists_function(&tokens[0], self.current_scope.clone())
            //);
            if is_function_definition(&tokens) {
                return self.get_expression_result_for_funcdef(&tokens, &identifier);
            } else if self.get_exists_function(&tokens[0], self.current_scope.clone()) {
                return self.get_expression_result_for_function_call(&identifier, &tokens);
            }
        }
        // or error if none of above
        let arrow_indent = 3 + identifier.len();
        let len = get_len_tokens_string(&tokens);
        let arrow_len = if arrow_indent > len {
            len
        } else {
            len - arrow_indent
        };
        Err(self.get_error(
            0,
            arrow_len,
            "is not a valid expression: must be either an: integer, e.g. 12345, float, e.g. 123.45, existing constant, e.g. x, string, e.g. \"string\", function call, e.g. + 1 2, function definition, e.g. \\ arg1 => arg1",
        ))
    }

    fn get_expression_result_for_expression(self: &mut Self) -> Result<Option<String>, String> {
        ////dbg!("get_expression_result_for_expression");
        let tokens = self.lines_of_tokens[self.current_line].clone();
        let identifier = self.current_scope.clone();
        let mut validation_error = None;
        let expression_result = &self.get_expression_result(&identifier, tokens.clone());
        match expression_result {
            Ok((expression, exp_type)) => {
                //dbg!(&expression, &exp_type, &self.current_scope);
                if self.current_scope != "main".to_string() {
                    self.set_output_for_return_expression(&tokens);
                } else {
                    ////dbg!(expression);
                    self.set_output_for_plain_expression(&tokens); //, expression);
                }
            }
            Err(e) => {
                validation_error = Some(e.clone());
            }
        }
        Ok(validation_error)
    }

    fn get_expression_result_for_function_call(
        self: &mut Self,
        identifier: &String,
        tokens: &Vec<String>,
    ) -> Result<(Expression, ExpressionType), String> {
        let fn_option: Option<FunctionDefinition> =
            self.get_option_function_definition(tokens[0].clone(), self.current_scope.clone());
        let tokens_string_length = get_len_tokens_string(&tokens);
        ////dbg!(&fn_option);
        match fn_option {
            Some(def) => {
                let allow_for_fn_name = 1;
                let count_arguments = tokens.len() - allow_for_fn_name;

                let expression_indents = 3 + def.name.len();
                let validate_arg_types_must_match = def
                    .validations
                    .contains(&"arg_types_must_match".to_string());

                let mut final_return_type = &def.return_type;

                // check number of arguments
                if def.types.len() != count_arguments {
                    return Err(self.get_error(
                        expression_indents,
                        tokens_string_length,
                        &format!(
                            "wrong number of function arguments. Expected: {}. Found {}.",
                            def.types.len(),
                            count_arguments
                        ),
                    ));
                }

                // check types of values
                let mut value_types: Vec<String> = vec![];
                for i in allow_for_fn_name..tokens.len() {
                    value_types.push(self.get_type(&tokens[i], self.current_scope.clone()));
                }
                for i in allow_for_fn_name..count_arguments {
                    if !def.types[i].contains(&value_types[i]) {
                        return Err(self.get_error(
                                expression_indents,
                                tokens_string_length,
                                &format!(
                                    "function arguments are not the correct types. Expected: {:?}. Found {:?}",
                                    def.types,
                                    value_types
                                ),
                            ));
                    }
                }

                // validation: check all types match
                if count_arguments > 0 && validate_arg_types_must_match {
                    // need to check if at least one value otherwise can't determine 'first' below
                    let first = &value_types[0];
                    if final_return_type == "" {
                        final_return_type = first;
                    };
                    if value_types
                        .clone()
                        .into_iter()
                        .any(|c| &c != first && !&c.contains(first) && !&first.contains(&c))
                    {
                        return Err(self.get_error(
                                    expression_indents,
                                tokens_string_length - expression_indents - 1,
                                &format!(
                                    "This function's arguments must all be of the same type. Some values have types that appear to differ: {:?}",
                                    value_types
                                ),
                            ));
                    }
                }
                let output = match def.types.len() {
                    2 => {
                        let out1 = def.format.replace("#1", &tokens[allow_for_fn_name]);
                        let out2 = out1.replace("#2", &tokens[allow_for_fn_name + 1]);
                        out2
                    }
                    1 => {
                        let out1 = def.format.replace("#1", &tokens[allow_for_fn_name]);
                        out1
                    }
                    _ => def.format,
                };
                ////dbg!(&output);

                return Ok((output, final_return_type.clone()));
            }
            _ => {
                return Err(self.get_error(
                    3 + identifier.len(),
                    tokens_string_length,
                    &format!("is not a valid call to function '{}'", tokens[0]),
                ));
            }
        }
    }

    fn get_expression_result_for_variable_assignment(
        self: &mut Self,
    ) -> Result<Option<String>, String> {
        let tokens = self.lines_of_tokens[self.current_line].clone();
        if tokens.len() < 2 || tokens[0] != "=" {
            return Err(ERRORS.variable_assignment.to_string());
        } else {
            let identifier = tokens[1].clone();

            let mut validation_error = None;
            if self.get_exists_function(&identifier, self.current_scope.clone()) {
                let e = self.get_error(2, identifier.len(), ERRORS.constants_are_immutable);
                validation_error = Some(e.clone());
            }

            let expression_result =
                &self.get_expression_result(&identifier, tokens[2..tokens.len()].to_vec());

            match expression_result {
                Ok((exp, exp_type)) => {
                    let possible_referenced_constant_result =
                        self.get_expression_result_for_referenced_constant(&tokens, exp, exp_type);
                    match possible_referenced_constant_result {
                        Ok((expression, expression_type)) => {
                            if *expression_type == "Function".to_string() {
                                //if *expression_type == "FunctionDefSingle".to_string() {
                                self.set_output_for_function_definition_singleline(
                                    &identifier,
                                    &expression,
                                );
                            } else if *expression_type == "FunctionDefFirstOfMulti".to_string() {
                                self.set_output_for_function_definition_singleline_or_firstline_of_multi(
                                    &identifier,
                                    &expression,
                                );
                                //self.current_scope = identifier.clone();
                                self.indent = self.indent + 1;
                            } else {
                                self.set_output_for_variable_assignment(
                                    &identifier,
                                    &expression,
                                    &expression_type,
                                );
                            }
                            //dbg!("here?", &self.current_scope, &identifier);
                            self.set_functions_for_constant(
                                identifier,
                                expression,
                                expression_type,
                            );
                        }
                        Err(e) => validation_error = Some(e.clone()),
                    }
                }
                Err(e) => validation_error = Some(e.clone()),
            }

            Ok(validation_error)
        }
    }

    fn get_expression_result_for_referenced_constant(
        self: &mut Self,
        tokens: &Vec<String>,
        expression: &Expression,
        expression_type: &ExpressionType,
    ) -> Result<(Expression, ExpressionType), String> {
        let mut final_type = expression_type.clone();
        let mut final_expression = expression.clone();
        let referenced_function_name = tokens[2].clone();
        let function_exists =
            self.get_exists_function(&referenced_function_name, self.current_scope.clone());
        if function_exists {
            // disambiguate i64|f64 for the actual type based on the type of first arg
            let def_option = self.get_option_function_definition(
                referenced_function_name.to_string(),
                self.current_scope.clone(),
            );
            match def_option {
                Some(def) => {
                    if def
                        .validations
                        .contains(&"arg_types_must_match".to_string())
                        && final_type.contains("|")
                    {
                        let first_arg = tokens[3].clone();
                        let first_arg_type = self.get_type(&first_arg, self.current_scope.clone());
                        final_type = first_arg_type;
                    }
                    if def.validations.contains(&"is_constant".to_string()) {
                        final_expression = def.name;
                    }
                }
                None => (),
            }
        }
        Ok((final_expression, final_type))
    }

    fn get_expression_result_for_funcdef(
        self: &mut Self,
        tokens: &Vec<String>,
        identifier: &String,
    ) -> Result<(Expression, ExpressionType), String> {
        let slash_option = get_function_def_slash(tokens);
        let arrow_option = get_function_def_arrow(tokens);
        let arrow_indent = 3 + identifier.len();
        let mut arrow_len = concatenate_vec_strings(&tokens).len() + &tokens.len();
        if arrow_len > 0 {
            arrow_len = arrow_len - 1;
        }
        let not_valid = "is not a valid function definition.";
        let example_syntax = "Example syntax:\r\n'= func_name : i64 i64 \\ arg1 => + arg1 123'\r\n               ^         ^       ^_after arrow return expression\r\n                \\         \\_after slash argument names\r\n                 \\_after colon argument types, last one is return type";
        match (slash_option, arrow_option) {
            (Some(slash), Some(arrow)) => {
                if arrow < slash || slash == 0 || arrow == 0 {
                    return Err(self.get_error(
                        arrow_indent,
                        arrow_len,
                        &format!(
                            "{} Missing slash or arrow.\r\n{}",
                            not_valid, example_syntax
                        ),
                    ));
                } else {
                    let type_signature = get_function_def_type_signature(&tokens, slash);
                    let args = get_function_def_args(&tokens, slash, arrow);

                    // check if all args have types, plus a return type
                    if type_signature.len() != args.len() + 1 {
                        return Err(self.get_error(
                                    arrow_indent,
                                    arrow_len,
                                    &format!("{} The count of argument types and arguments don't match. {:?} {:?}\r\n{}", not_valid, type_signature, args,example_syntax),
                                ));
                    }

                    let body = get_function_def_body(&tokens, arrow);
                    if body.len() == 0 {
                        // i.e. there is nothing after the "=>" then assume it is a multiline function
                        return self.get_expression_result_for_funcdef_of_multiline_function(
                            &body,
                            &args,
                            &type_signature,
                            &identifier,
                        );
                    } else {
                        // get return expression/value for single line function

                        // define the arguments so get_expression_result doesn't return "Undefined" for their types
                        let expression_result = self.set_functions_for_func_args(
                            &identifier,
                            &args,
                            &type_signature,
                            &body,
                        );

                        match expression_result {
                            Ok((expression, expression_type)) => {
                                return self
                                    .get_expression_result_for_funcdef_of_singleline_function(
                                        &body,
                                        &args,
                                        &type_signature,
                                        &identifier,
                                        &expression,
                                        &expression_type,
                                        arrow_indent,
                                        arrow_len,
                                        not_valid,
                                        example_syntax,
                                    )
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
            }
            _ => {
                return Err(self.get_error(
                    arrow_indent,
                    arrow_len,
                    &format!("{} {}", not_valid, example_syntax),
                ))
            }
        }
    }

    fn get_expression_result_for_funcdef_of_singleline_function(
        self: &mut Self,
        body_of_expression: &Vec<String>,
        args: &Vec<String>,
        type_signature: &Vec<String>,
        identifier: &String,
        expression: &String,
        expression_type: &ExpressionType,
        arrow_indent: usize,
        arrow_len: usize,
        not_valid: &str,
        example_syntax: &str,
    ) -> Result<(Expression, ExpressionType), String> {
        // validate that expression type matches provided type
        let return_type_signature = &type_signature[type_signature.len() - 1];
        let first_arg_of_return_expression = if expression_type.contains("|") {
            let test = self.get_type(&body_of_expression.clone()[1], identifier.clone());
            test
        } else {
            expression_type.clone()
        };
        let expression_type_without_pipe = if expression_type.contains("|") {
            first_arg_of_return_expression
        } else {
            expression_type.clone()
        };
        if !return_type_signature.contains(&expression_type_without_pipe.clone()) {
            return Err(self.get_error(
            arrow_indent,
            arrow_len,
            &format!("{}\r\n{:?} - the type of this function's return expression\r\n{:?} - does not match it's signature's return type\r\n{}", not_valid, &expression_type_without_pipe, &type_signature[type_signature.len()-1], example_syntax),
        ));
        }

        let args_with_types = get_function_args_with_types(args.clone(), type_signature.clone());
        let expression_indent = " ".repeat((self.indent + 1) * 4);
        let trailing_brace_indent = " ".repeat(self.indent * 4);

        let output = format!(
            "({}) -> {} {{\r\n{}{}\r\n{}}}\r\n",
            args_with_types,
            type_signature[type_signature.len() - 1],
            expression_indent,
            expression,
            trailing_brace_indent
        );
        return Ok((output, "Function".to_string()));
        //return Ok((output, "FunctionDefSingle".to_string()));
    }

    fn get_expression_result_for_funcdef_of_multiline_function(
        self: &mut Self,
        body_of_expression: &Vec<String>,
        args: &Vec<String>,
        type_signature: &Vec<String>,
        identifier: &String,
    ) -> Result<(Expression, ExpressionType), String> {
        self.current_scope = identifier.clone();

        self.current_scope = identifier.clone();
        //let temp_scope = self.current_scope.clone();
        ////dbg!("testy", &identifier, &self.current_scope);
        let args_with_types = get_function_args_with_types(args.clone(), type_signature.clone());

        // define the arguments so get_expression_result doesn't return "Undefined" for their types
        let _expression_result = self.set_functions_for_func_args(
            &identifier,
            &args,
            &type_signature,
            &body_of_expression,
        );
        //self.current_scope = temp_scope;

        let output = format!(
            "({}) -> {} {{\r\n", // no end function brace
            args_with_types,
            &type_signature[type_signature.len() - 1]
        );
        return Ok((output, "FunctionDefFirstOfMulti".to_string()));
    }

    fn get_exists_function(self: &Self, function_name: &str, scope_name: String) -> bool {
        self.functions.iter().any(|def| {
            get_names_and_scope_match(
                def.name.clone(),
                function_name.to_string(),
                def.scope.clone(),
                scope_name.clone(),
            )
        })
    }

    fn get_option_function_definition(
        self: &Self,
        function_name: String,
        scope_name: String,
    ) -> Option<FunctionDefinition> {
        let funcs: Vec<&FunctionDefinition> = self
            .functions
            .iter()
            .filter(|def| {
                get_names_and_scope_match(
                    def.name.clone(),
                    function_name.to_string(),
                    def.scope.clone(),
                    scope_name.clone(),
                )
            })
            .collect::<Vec<_>>();
        if funcs.len() == 1 {
            return Some(funcs[0].clone());
        } else {
            return None;
        }
    }

    fn get_type(self: &Self, text: &String, scope_name: String) -> String {
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
        let s = scope_name.clone();
        if self.get_exists_function(text, s.clone()) {
            let def_option = self.get_option_function_definition(text.to_string(), s.clone());
            match def_option {
                Some(def) => {
                    return_type = def.return_type.clone();
                }
                _ => (),
            }
        }
        return return_type;
    }

    */

    fn get_error(self: &Self, arrow_indent: usize, arrow_len: usize, error: &str) -> String {
        format!(
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
        )
    }
}

fn tokens_remove_head(tokens: Vec<String>) -> Vec<String> {
    if tokens.len() == 1 {
        vec![]
    } else {
        tokens[1..].to_vec()
    }
}

/*
fn get_names_and_scope_match(
    defname: String,
    name2: String,
    defscope: String,
    scope2: String,
) -> bool {
    defname == *name2 && (defscope == scope2 || defscope == "global".to_string())
}

fn get_len_tokens_string(tokens: &Vec<String>) -> usize {
    let mut total = 0;
    for i in 0..tokens.len() {
        total += tokens[i].len();
    }
    let num_spaces_inbetween = if total > 0 { total - 1 } else { 0 };
    total + num_spaces_inbetween
}
*/

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

/*
fn is_function_definition(tokens: &Vec<String>) -> bool {
    tokens.len() > 1 && tokens[0] == ":".to_string()
}
*/

fn concatenate_vec_strings(tokens: &Vec<String>) -> String {
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

/*
fn get_function_def_slash(tokens: &Vec<String>) -> Option<usize> {
    if tokens.len() > 1 {
        return tokens[1..].iter().position(|c| c == &"\\".to_string());
    } else {
        return None;
    }
}

fn get_function_def_arrow(tokens: &Vec<String>) -> Option<usize> {
    if tokens.len() > 1 {
        return tokens[1..].iter().position(|c| c == &"=>".to_string());
    } else {
        return None;
    }
}

fn get_function_def_type_signature(tokens: &Vec<String>, slash: usize) -> Vec<String> {
    return tokens[1..slash + 1].to_vec();
}

fn get_function_def_args(tokens: &Vec<String>, slash: usize, arrow: usize) -> Vec<String> {
    return tokens[slash + 2..arrow + 1].to_vec();
}

fn get_function_def_body(tokens: &Vec<String>, arrow: usize) -> Vec<String> {
    return tokens[arrow + 2..].to_vec();
}

fn get_function_args_with_types(args: Vec<String>, type_signature: Vec<String>) -> String {
    let mut args_with_types = "".to_string();
    for i in 0..args.len() {
        let comma_not_first = if i == 0 {
            "".to_string()
        } else {
            ", ".to_string()
        };
        args_with_types = format!(
            "{}{}{}: {}",
            args_with_types, comma_not_first, args[i], type_signature[i]
        );
    }
    args_with_types
}
*/
#[cfg(test)]
mod tests {
    use super::*;
    use ast::Element;

    fn mock_config(contents: &str) -> Config {
        Config {
            file: File::new(),
            lines_of_chars: vec![],
            lines_of_tokens: vec![],
            output: "".to_string(),
            current_line: 0,
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
