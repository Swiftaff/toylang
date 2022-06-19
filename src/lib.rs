//TODO make most function arguments refs
use std::error::Error;
use std::fs;
use std::path::Path;

type FunctionName = String;
type FunctionFormat = String;
type FunctionType = String;
type FunctionValidation = String;
type FunctionReturnType = String;
type FunctionScope = String;

#[derive(Clone, Debug)]
pub struct FunctionDefinition {
    pub name: FunctionName,
    pub format: FunctionFormat,
    pub types: Vec<FunctionType>,
    pub validations: Vec<FunctionValidation>,
    pub return_type: FunctionReturnType,
    pub scope: FunctionScope,
}
type Expression = String;
type ExpressionType = String;

#[derive(Clone, Debug)]
pub struct Config {
    pub filepath: String,
    pub filename: String,
    pub filecontents: String,
    pub lines_of_chars: Vec<Vec<char>>,
    pub lines_of_tokens: Vec<Vec<String>>,
    pub output: String,
    pub outputcursor: usize,
    pub current_line: usize,
    pub current_scope: FunctionScope,
    pub indent: usize,
    pub functions: Vec<FunctionDefinition>,
    pub error_stack: Vec<String>,
}

struct Errors {
    invalid_program_syntax: &'static str,
    variable_assignment: &'static str,
    no_valid_comment_single_line: &'static str,
    no_valid_expression: &'static str,
    constants_are_immutable: &'static str,
}

const ERRORS: Errors = Errors {
    invalid_program_syntax: "Invalid program syntax. Must start with RUN, followed by linebreak, optional commands and linebreak, and end with END",
    variable_assignment: "Invalid variable assignment. Must contain Int or Float, e.g. x = Int 2",
    no_valid_comment_single_line: "No valid single line comment found",
    no_valid_expression: "No valid expression was found",
    constants_are_immutable: "Constants are immutable. You may be trying to assign a value to a constant that has already been defined. Try renaming this as a new constant."
};

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err("missing filepath argument".to_string());
        }
        let filepath = args[1].clone();
        let filename = Path::new(&filepath.clone())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let filecontents = "".to_string();
        let lines_of_chars = vec![];
        let lines_of_tokens = vec![];
        let output = "".to_string();
        let outputcursor = 0;
        let current_line = 0;
        let indent = 0;
        let current_scope = "main".to_string();
        let arithmetic_primitives = vec!["+", "-", "*", "/", "%"];
        let arithmetic_operators: Vec<FunctionDefinition> = arithmetic_primitives
            .into_iter()
            .map(|prim| FunctionDefinition {
                name: prim.to_string(),
                format: format!("#1 {} #2", prim).to_string(),
                types: vec!["i64|f64".to_string(), "i64|f64".to_string()],
                validations: vec!["arg_types_must_match".to_string()],
                return_type: "i64|f64".to_string(),
                scope: "global".to_string(),
            })
            .collect();
        let function_def = FunctionDefinition {
            name: "\\".to_string(),
            format: "#0fn #1(#2) {\r\n#3\r\n#0}\r\n".to_string(),
            types: vec![],
            validations: vec![],
            return_type: "".to_string(),
            scope: "".to_string(),
        };
        let functions: Vec<FunctionDefinition> = vec![]
            .iter()
            .chain(&arithmetic_operators)
            .chain(&vec![function_def])
            .map(|x| x.clone())
            .collect();
        let error_stack = vec![];
        Ok(Config {
            filepath,
            filename,
            filecontents,
            lines_of_chars,
            lines_of_tokens,
            output,
            outputcursor,
            current_line,
            current_scope,
            indent,
            functions,
            error_stack,
        })
    }

    pub fn run(self: &mut Self) -> Result<(), Box<dyn Error>> {
        self.filecontents = fs::read_to_string(&self.filepath)?;
        println!("\r\nINPUT contents of filepath: {:?}", &self.filepath);
        self.set_lines_of_chars();
        self.set_lines_of_tokens();
        self.run_main_loop()?;
        self.writefile_or_error()
    }

    fn writefile_or_error(self: &Self) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        if self.error_stack.len() == 0 {
            fs::write("../../src/bin/output.rs", &self.output)?;
        } else {
            println!("DIDN'T SAVE");
        }
        Ok(())
    }

    fn run_main_loop(self: &mut Self) -> Result<(), String> {
        //ref: https://doc.rust-lang.org/reference/tokens.html

        match self.main_loop_over_lines_of_tokens() {
            Ok(()) => {
                println!(
                    "Toylang compiled successfully:\r\n----------\r\n{}\r\n----------\r\n",
                    self.output
                );
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
        self.set_output_for_main_fn();
        for line in 0..self.lines_of_tokens.len() {
            if self.lines_of_tokens[line].len() > 0 {
                self.current_line = line;
                self.check_one_or_more_succeeds()?;
            }
        }
        Ok(())
    }

    fn check_one_or_more_succeeds(self: &mut Self) -> Result<(), Vec<String>> {
        if self.check_one_succeeds("set_output_for_comment_single_line") {
            return Ok(());
        }
        if self.check_one_succeeds("get_expression_result_for_variable_assignment") {
            return Ok(());
        }
        if self.check_one_succeeds("get_expression_result_for_expression") {
            return Ok(());
        }

        let e = self.get_error(0, 1, ERRORS.no_valid_expression);
        self.error_stack.push(e.to_string());
        Err(self.error_stack.clone())
    }

    fn check_one_succeeds(self: &mut Self, function_name: &str) -> bool {
        let mut succeeded = false;
        let mut clone = self.clone();
        let result = match function_name {
            "get_expression_result_for_variable_assignment" => {
                clone.get_expression_result_for_variable_assignment()
            }
            "get_expression_result_for_expression" => clone.get_expression_result_for_expression(),
            "set_output_for_comment_single_line" => clone.set_output_for_comment_single_line(),
            _ => {
                return false;
            }
        };
        match result {
            Ok(validation_error) => {
                self.set_all_from_clone(clone);
                match validation_error {
                    Some(e) => {
                        self.error_stack.push(e);
                        succeeded = false;
                    }
                    None => succeeded = true,
                }
            }
            Err(_e) => (), //println!("error {:?}", e), //self.error_stack.push(e), // just testing - move to a temporary error_stack
        }
        succeeded
    }

    fn set_lines_of_chars(self: &mut Self) {
        let mut index_from = 0;
        let mut index_to = 0;
        let char_vec: Vec<char> = self.filecontents.chars().collect();
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
        self.filepath = to_clone.filepath;
        self.filename = to_clone.filename;
        self.filecontents = to_clone.filecontents;
        self.lines_of_chars = to_clone.lines_of_chars;
        self.lines_of_tokens = to_clone.lines_of_tokens;
        self.output = to_clone.output;
        self.outputcursor = to_clone.outputcursor;
        self.current_line = to_clone.current_line;
        self.current_scope = to_clone.current_scope;
        self.indent = to_clone.indent;
        self.functions = to_clone.functions;
        self.error_stack = to_clone.error_stack;
    }

    fn set_output_for_main_fn(self: &mut Self) {
        self.output = "fn main() {\r\n}".to_string();
        self.indent = 1;
        self.outputcursor = 13;
    }

    fn set_output_for_return_expression(self: &mut Self, tokens: &Vec<String>) {
        // if we found an expression while inside a function, then it must be the returning expression
        // so we should close this function brace and move scope back up a level
        self.indent = self.indent - 1;
        let trailing_brace = format!("{}}}\r\n", " ".repeat(self.indent * 4));
        let option_parent_scope = self
            .get_option_function_definition(self.current_scope.clone(), self.current_scope.clone());
        match option_parent_scope {
            Some(parent_scope) => self.current_scope = parent_scope.scope,
            None => (), // couldn't find function called self.current_scope - so um, leave as is, or maybe default to main??
        }
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
        let insert = format!(
            "{}{};\r\n",
            " ".repeat(self.indent * 4),
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

    fn set_output_for_comment_single_line(self: &mut Self) -> Result<Option<String>, String> {
        let tokens = &self.lines_of_tokens[self.current_line];
        let first_token_chars = tokens[0].chars().collect::<Vec<char>>();
        if first_token_chars.len() < 2 || first_token_chars[0] != '/' || first_token_chars[1] != '/'
        {
            return Err(ERRORS.no_valid_comment_single_line.to_string());
        } else {
            let comment = concatenate_vec_strings(tokens);
            let insert = &format!("{}{}\r\n", " ".repeat(self.indent * 4), &comment);
            self.output.insert_str(self.outputcursor, &insert);
            self.outputcursor = self.outputcursor + insert.len();
            let validation_error = None;
            Ok(validation_error)
        }
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
            name: identifier,
            format: value,
            types: vec![],
            validations: vec!["is_constant".to_string()],
            return_type: final_type,
            scope: self.current_scope.clone(),
        };
        self.functions.push(new_constant_function);
    }

    fn get_expression_result_for_expression(self: &mut Self) -> Result<Option<String>, String> {
        let tokens = self.lines_of_tokens[self.current_line].clone();
        let identifier = self.current_scope.clone();
        let mut validation_error = None;
        let expression_result = &self.get_expression_result(&identifier, tokens.clone());
        match expression_result {
            Ok((expression, exp_type)) => {
                if self.current_scope != "main".to_string() {
                    self.set_output_for_return_expression(&tokens);
                } else {
                    self.set_output_for_plain_expression(&tokens);
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

        match fn_option {
            Some(def) => {
                let allow_for_fn_name = 1;
                let count_arguments = tokens.len() - allow_for_fn_name;
                let tokens_string_length = get_len_tokens_string(&tokens);
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
                    value_types.push(self.get_type(&tokens[i], identifier.clone()));
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
                                tokens_string_length,
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

                return Ok((output, final_return_type.clone()));
            }
            _ => {
                return Err(self.get_error(
                    3 + identifier.len(),
                    10, //text.len(),
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
                                // single line, or first line of multiline
                                self.set_output_for_function_definition_singleline_or_firstline_of_multi(
                                    &identifier,
                                    &expression,
                                );
                            } else {
                                self.set_output_for_variable_assignment(
                                    &identifier,
                                    &expression,
                                    &expression_type,
                                );
                            }
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

    fn get_expression_result(
        self: &mut Self,
        identifier: &String,
        tokens: Vec<String>,
    ) -> Result<(Expression, ExpressionType), String> {
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
            //else is a function call - see below
        }

        let arrow_indent = 3 + identifier.len();
        let mut arrow_len = concatenate_vec_strings(&tokens).len() + &tokens.len();
        if arrow_len > 0 {
            arrow_len = arrow_len - 1;
        }
        let not_valid = "is not a valid function definition.";
        let example_syntax = "Example syntax:\r\n'= func_name : i64 i64 \\ arg1 => + arg1 123'\r\n               ^         ^       ^_after arrow return expression\r\n                \\         \\_after slash argument names\r\n                 \\_after colon argument types, last one is return type";

        if tokens.len() > 0 {
            if is_function_definition(&tokens) {
                return self.get_expression_result_for_funcdef(
                    &tokens,
                    arrow_indent,
                    arrow_len,
                    not_valid,
                    example_syntax,
                    &identifier,
                );
            } else if self.get_exists_function(&tokens[0], self.current_scope.clone()) {
                return self.get_expression_result_for_function_call(&identifier, &tokens);
            }
        }

        // or error if none of above
        let text: String = tokens.iter().map(String::as_str).collect();
        Err(self.get_error(
            3 + identifier.len(),
            text.len(),
            "is not a valid expression: must be either an: integer, e.g. 12345, float, e.g. 123.45, existing constant, e.g. x, string, e.g. \"string\", function call, e.g. + 1 2, function definition, e.g. \\ arg1 => arg1",
        ))
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
            //disambiguate i64|f64 for the actual type based on the type of first arg
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
        arrow_indent: usize,
        arrow_len: usize,
        not_valid: &str,
        example_syntax: &str,
        identifier: &String,
    ) -> Result<(Expression, ExpressionType), String> {
        let slash_option = get_function_def_slash(tokens);
        let arrow_option = get_function_def_arrow(tokens);
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
        //TODO check this - should it return the function return_type??
        return Ok((output, "Function".to_string()));
    }

    fn get_expression_result_for_funcdef_of_multiline_function(
        self: &mut Self,
        body_of_expression: &Vec<String>,
        args: &Vec<String>,
        type_signature: &Vec<String>,
        identifier: &String,
    ) -> Result<(Expression, ExpressionType), String> {
        self.current_scope = identifier.clone();

        // <- TODO fix below - indents the fn def line as well as contents
        self.indent = self.indent + 1;
        self.current_scope = identifier.clone();
        let args_with_types = get_function_args_with_types(args.clone(), type_signature.clone());

        // define the arguments so get_expression_result doesn't return "Undefined" for their types
        let _expression_result = self.set_functions_for_func_args(
            &identifier,
            &args,
            &type_signature,
            &body_of_expression,
        );

        let output = format!(
            "({}) -> {} {{\r\n", //no end function brace
            args_with_types,
            &type_signature[type_signature.len() - 1]
        );

        //TODO check this - should it return the function return_type??
        return Ok((output, "Function".to_string()));
    }

    /*
    FYI wanted a generic closure for both below - but they seem to have slightly different types for each
    &FunctionDefinition
    &&FunctionDefinition

    fn create_closure(
        self: &Self,
        function_name: String,
        scope_name: String,
    ) -> impl Fn(&FunctionDefinition) -> bool {
        move |def| {
            def.name == *function_name
                && (def.scope == scope_name || def.scope == "global".to_string())
        }
    }
    */

    //watch out - duplicated closures below
    fn get_exists_function(self: &Self, function_name: &str, scope_name: String) -> bool {
        //let closure = self.create_closure(function_name.to_string(), scope_name);
        self.functions.iter().any(|def| {
            def.name == *function_name
                && (def.scope == scope_name || def.scope == "global".to_string())
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
                def.name == *function_name
                    && (def.scope == scope_name || def.scope == "global".to_string())
            })
            .collect::<Vec<_>>();
        if funcs.len() == 1 {
            return Some(funcs[0].clone());
        } else {
            return None;
        }
    }

    fn get_type(self: &Self, text: &String, scope_name: String) -> String {
        if is_integer(text) {
            return "i64".to_string();
        }
        if is_float(text) {
            return "f64".to_string();
        }
        if is_string(text) {
            return "String".to_string();
        }
        if self.get_exists_function(text, scope_name.clone()) {
            let def_option = self.get_option_function_definition(text.to_string(), scope_name);
            match def_option {
                Some(def) => {
                    return def.return_type.clone();
                }
                _ => {
                    return "Undefined".to_string();
                }
            }
        } else {
            return "Undefined".to_string();
        }
    }

    fn get_error(self: &Self, arrow_indent: usize, arrow_len: usize, error: &str) -> String {
        format!(
            "----------\r\n{}:{}:0\r\n{}\r\n{}{} {}",
            self.filename, // TODO try to fix path so it becomes a link in VS Code
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

fn get_len_tokens_string(tokens: &Vec<String>) -> usize {
    let mut total = 0;
    for i in 0..tokens.len() {
        total += tokens[i].len();
    }
    let num_spaces_inbetween = total - 1;
    total + num_spaces_inbetween
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

fn is_function_definition(tokens: &Vec<String>) -> bool {
    tokens.len() > 1 && tokens[0] == ":".to_string()
}

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
        //if you get to end of string and it's all whitespace return empty string
        return "".to_string();
    }
    input[first_non_whitespace_index..].to_string()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_config(contents: &str) -> Config {
        Config {
            filepath: "".to_string(),
            filename: "".to_string(),
            filecontents: contents.to_string(),
            lines_of_chars: vec![],
            lines_of_tokens: vec![],
            output: "".to_string(),
            outputcursor: 0,
            current_line: 0,
            current_scope: "".to_string(),
            indent: 1,
            functions: vec![],
            error_stack: vec![],
        }
    }

    #[test]
    fn test_new() {
        let args = ["toylang".to_string(), "filepath_example".to_string()];
        let config_result = Config::new(&args);
        let filepath = "filepath_example".to_string();
        match config_result {
            Ok(config) => assert_eq!(config.filepath, filepath),
            Err(_) => assert!(false, "error should not exist"),
        }
    }
    #[test]
    fn test_get_expression_result_for_variable_assignment() {
        let err: Result<Option<String>, String> = Err(ERRORS.variable_assignment.to_string());
        //let err2: Result<Option<String>, String> =
        //    Err(ERRORS.no_valid_identifier_found.to_string());
        assert_eq!(
            mock_config("").get_expression_result_for_variable_assignment(),
            err
        );
        //assert_eq!(mock_config("2 = x").get_expression_result_for_variable_assignment(), err2);
        //assert_eq!(mock_config("let x = 2").get_expression_result_for_variable_assignment(), err2);
        //assert_eq!(get_expression_result_for_variable_assignment("x = 2".to_string()), err);
        //assert_eq!(get_expression_result_for_variable_assignment("x = Abc 2".to_string()), err);
        //assert_eq!(get_expression_result_for_variable_assignment("x = Boats 2".to_string()), err);
        //assert_eq!(get_expression_result_for_variable_assignment("x = Monkey 2".to_string()), err);

        //OK
        assert_eq!(
            mock_config("= x 2").get_expression_result_for_variable_assignment(),
            Ok(None)
        );
        assert_eq!(
            mock_config("= x 2.2").get_expression_result_for_variable_assignment(),
            Ok(None)
        );
        assert_eq!(
            mock_config("= x 1\r\n= y x").get_expression_result_for_variable_assignment(),
            Ok(None)
        );
        assert_eq!(
            mock_config("= x \"string\"").get_expression_result_for_variable_assignment(),
            Ok(None)
        );
    }

    #[test]
    fn test_strip_leading_whitespace() {
        assert_eq!(strip_leading_whitespace("".to_string()), "".to_string());
        assert_eq!(
            strip_leading_whitespace("abc".to_string()),
            "abc".to_string()
        );
        assert_eq!(
            strip_leading_whitespace("abc   \r\n".to_string()),
            "abc   \r\n".to_string()
        );
        assert_eq!(
            strip_leading_whitespace(" abc".to_string()),
            "abc".to_string()
        );
        assert_eq!(
            strip_leading_whitespace("    abc".to_string()),
            "abc".to_string()
        );
        assert_eq!(
            strip_leading_whitespace("\r\n    abc  \r\n".to_string()),
            "abc  \r\n".to_string()
        );
    }
}
