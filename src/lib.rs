use std::error::Error;
use std::fs;

type FunctionName = String;
type FunctionFormat = String;
type FunctionType = String;
type FunctionValidation = String;
type FunctionReturnType = String;
type FunctionDefinition = (
    FunctionName,
    FunctionFormat,
    Vec<FunctionType>,
    Vec<FunctionValidation>,
    FunctionReturnType,
);
type Expression = String;
type ExpressionType = String;

#[derive(Clone, Debug)]
pub struct Config {
    pub filename: String,
    pub filecontents: String,
    pub remaining: String,
    pub lines_of_chars: Vec<Vec<char>>,
    pub lines_of_tokens: Vec<Vec<String>>,
    pub output: String,
    pub outputcursor: usize,
    pub pass: usize,
    pub indent: usize,
    pub functions: Vec<FunctionDefinition>,
    pub error_stack: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err("missing filename argument".to_string());
        }
        let filename = args[1].clone();
        let filecontents = "".to_string();
        let remaining = "".to_string();
        let lines_of_chars = vec![];
        let lines_of_tokens = vec![];
        let output = "".to_string();
        let outputcursor = 0;
        let pass = 0;
        let indent = 0;
        let arithmetic_primitives = vec!["+", "-", "*", "/", "%"];
        let arithmetic_operators: Vec<FunctionDefinition> = arithmetic_primitives
            .into_iter()
            .map(|prim| {
                (
                    prim.to_string(),
                    format!("#1 {} #2", prim).to_string(),
                    vec!["i64|f64".to_string(), "i64|f64".to_string()],
                    vec!["arg_types_must_match".to_string()],
                    "i64|f64".to_string(),
                )
            })
            .collect();
        let function_def: FunctionDefinition = (
            "\\".to_string(),
            "#0fn #1(#2) {\r\n#3\r\n#0}\r\n".to_string(),
            vec![],
            vec!["lambda".to_string()],
            "".to_string(),
        );
        let functions: Vec<FunctionDefinition> = vec![]
            .iter()
            .chain(&arithmetic_operators)
            .chain(&vec![function_def])
            .map(|x| x.clone())
            .collect();
        let error_stack = vec![];
        Ok(Config {
            filename,
            filecontents,
            remaining,
            lines_of_chars,
            lines_of_tokens,
            output,
            outputcursor,
            pass,
            indent,

            functions,
            error_stack,
        })
    }

    pub fn run(self: &mut Self) -> Result<(), Box<dyn Error>> {
        self.filecontents = fs::read_to_string(&self.filename)?;
        println!("\r\nINPUT contents of filename: {:?}", &self.filename); //, &self.filecontents
        self.get_lines_of_chars();
        self.get_lines_of_tokens();
        self.run_main_loop()?;
        if self.error_stack.len() == 0 {
            fs::write("../../src/bin/output.rs", &self.output)?;
        } else {
            println!("DIDN'T SAVE - error stack: {:?}", self.error_stack);
        }
        Ok(())
    }

    fn get_lines_of_chars(self: &mut Self) {
        self.remaining = self.filecontents.clone();
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

    fn get_lines_of_tokens(self: &mut Self) {
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
        self.check_program_syntax()?;
        for line in 1..self.lines_of_tokens.len() - 1 {
            if self.lines_of_tokens[line].len() > 0 {
                self.pass = line;
                self.check_one_or_more_succeeds()?;
            }
        }
        Ok(())
    }

    fn check_one_or_more_succeeds(self: &mut Self) -> Result<(), Vec<String>> {
        if self.check_one_succeeds("check_variable_assignment") {
            return Ok(());
        }
        if self.check_one_succeeds("check_comment_single_line") {
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
            "check_variable_assignment" => clone.check_variable_assignment(),
            "check_comment_single_line" => clone.check_comment_single_line(),
            _ => {
                return false;
            }
        };
        match result {
            Ok(validation_error) => {
                self.clone_mut_ref(clone);
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

    fn clone_mut_ref(self: &mut Self, to_clone: Config) -> () {
        // wokraround - can't just do 'self = clone.clone();' due to &mut derferencing ??
        self.filename = to_clone.filename;
        self.filecontents = to_clone.filecontents;
        self.remaining = to_clone.remaining;
        self.lines_of_chars = to_clone.lines_of_chars;
        self.lines_of_tokens = to_clone.lines_of_tokens;
        self.output = to_clone.output;
        self.outputcursor = to_clone.outputcursor;
        self.pass = to_clone.pass;
        self.indent = to_clone.indent;

        self.functions = to_clone.functions;
        self.error_stack = to_clone.error_stack;
    }

    fn check_program_syntax(self: &mut Self) -> Result<(), Vec<String>> {
        if self.lines_of_tokens.len() < 2
            || self.lines_of_tokens[0][0] != "RUN".to_string()
            || self.lines_of_tokens[self.lines_of_tokens.len() - 1][0] != "END".to_string()
        {
            self.error_stack
                .push(ERRORS.invalid_program_syntax.to_string());
            return Err(self.error_stack.clone());
        } else {
            self.output = "fn main() {\r\n}".to_string();
            self.indent = 1;
            self.outputcursor = 13; // anything new will be inserted before end bracket
        }

        Ok(())
    }

    fn check_variable_assignment(self: &mut Self) -> Result<Option<String>, String> {
        let tokens = &self.lines_of_tokens[self.pass];

        if tokens.len() < 2 || tokens[0] != "=" {
            return Err(ERRORS.variable_assignment.to_string());
        } else {
            let identifier = tokens[1].clone();
            let mut validation_error = None;
            if self.exists_function(&identifier) {
                let e = self.get_error(2, identifier.len(), ERRORS.constants_are_immutable);
                validation_error = Some(e);
            }

            let expression_result =
                self.check_expression(&identifier, tokens[2..tokens.len()].to_vec());
            match expression_result {
                Ok((expression, exp_type)) => {
                    let type_colon = if exp_type.len() == 0 { "" } else { ": " };
                    let mut value = expression.clone();
                    let mut final_type = exp_type.clone();
                    let mut validations = vec![];

                    if self.exists_function(&tokens[2]) {
                        final_type = "".to_string();
                        validations.push("get_type_from_referred_function".to_string());
                        value = tokens[2].to_string();
                    }
                    let new_expresion = expression.clone();
                    dbg!(&identifier, &exp_type);
                    let mut insert = format!(
                        "{}let {}{}{} = {};\r\n",
                        " ".repeat(self.indent * 4),
                        &identifier,
                        type_colon,
                        &exp_type,
                        &new_expresion
                    );
                    
                    if exp_type=="Function".to_string() {
                        insert = format!(
                        "{}fn {}{}\r\n",
                        " ".repeat(self.indent * 4),
                        &identifier,
                        &new_expresion
                    );
                    }

                    let new_constant_function = (
                        identifier.to_string(),
                        value,
                        vec![],
                        validations,
                        final_type,
                    );

                    self.functions.push(new_constant_function);
                    self.output.insert_str(self.outputcursor, &insert);
                    self.outputcursor = self.outputcursor + insert.len();
                }
                Err(e) => validation_error = Some(e),
                //_ => validation_error = Some("some other error".to_string()),
            }

            Ok(validation_error)
        }
    }

    fn check_function_definition(self: &mut Self) -> Result<Option<String>, String> {
        if self.remaining.len() < 3 {
            return Err(ERRORS.function_definition.to_string());
        } else {
            let mut remainder = strip_leading_whitespace(self.remaining.clone());
            remainder = get_str(remainder.clone(), "=")?;
            remainder = strip_leading_whitespace(remainder);
            let (identifier, mut remainder) = get_identifier(remainder)?;

            let mut validation_error = None;
            if identifier == "\\" {
                // check for pre-existing same function_name
                remainder =
                    strip_leading_whitespace(remainder[(&identifier.len() + 0)..].to_string());
                let (text, remain) = get_until_eol_or_eof(remainder);
                let function_name = text.clone(); // TODO need to fix this, assumes no args currently just for testing
                let fun = self.get_function(&"\\");
                match fun {
                    Some((_, function_format, _, _, _)) => {
                        let insert = function_format
                            .replace("#0", &" ".repeat(self.indent * 4))
                            .replace("#1", &function_name)
                            .replace("#2", &"function_arg: &str")
                            .replace(
                                "#3",
                                &format!(
                                    "{}{}",
                                    " ".repeat((self.indent + 1) * 4),
                                    "let x = function_arg;"
                                ),
                            );

                        // create and push function
                        //self.functions.push(identifier);

                        self.output.insert_str(self.outputcursor, &insert);
                        self.outputcursor = self.outputcursor + insert.len();
                        self.remaining = strip_leading_whitespace(remain);
                    }
                    _ => {
                        validation_error = Some("NOPE - not a function definition".to_string());
                    }
                }
            } else {
                return Err(ERRORS.function_definition.to_string());
            }
            Ok(validation_error)
        }
    }

    fn exists_function(self: &Self, function_name: &str) -> bool {
        self.functions.iter().any(|c| c.0 == *function_name)
    }

    fn get_function_definition(self: &Self, function_name: String) -> Option<FunctionDefinition> {
        let funcs: Vec<&FunctionDefinition> = self
            .functions
            .iter()
            .filter(|c| c.0 == *function_name)
            .collect::<Vec<_>>();
        if funcs.len() == 1 {
            return Some(funcs[0].clone());
        } else {
            return None;
        }
    }

    fn get_function(self: &Self, function_name: &str) -> Option<FunctionDefinition> {
        let function_vec = self
            .functions
            .iter()
            .filter(|c| c.0 == *function_name)
            .collect::<Vec<_>>();
        if function_vec.len() == 0 {
            None
        } else {
            Some(function_vec[0].clone())
        }
    }

    fn get_tokens_string_len(self: &Self, tokens: &Vec<String>) -> usize {
        let mut total = 0;
        for i in 0..tokens.len() {
            total += tokens[i].len();
        }
        let num_spaces_inbetween = total - 1;
        total + num_spaces_inbetween
    }

    fn check_expression(
        self: &Self,
        identifier: &String,
        tokens: Vec<String>,
    ) -> Result<(Expression, ExpressionType), String> {
        if tokens.len() == 1 {
            if self.is_integer(&tokens[0]) {
                return Ok((tokens[0].clone(), "i64".to_string()));
            }
            if self.is_float(&tokens[0]) {
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
            if is_function_definition(&tokens) {
                let slash_option = get_function_def_slash(&tokens);
                let arrow_option = get_function_def_arrow(&tokens);
                match (slash_option, arrow_option) {
                    (Some(slash),Some(arrow)) =>{
                        if arrow < slash || slash == 0 || arrow == 0 {
                            return Err(self.get_error(
                                3 + identifier.len(),
                                10, //text.len(),
                                &format!("is not a valid function definition. Missing slash or arrow. Should be in the form\r\n= func_name : i64 i64 \\ arg1 => + arg1 123\r\n{:?}", &tokens[0]),
                            ))
                        } else {
                            let type_signature = get_function_def_type_signature(&tokens, slash);
                            let args = get_function_def_args(&tokens, slash, arrow);
                            
                            // check if all args have types, plus a return type
                            if type_signature.len() != args.len() + 1 {
                                return Err(self.get_error(
                                    3 + identifier.len(),
                                    10, //text.len(),
                                    &format!("is not a valid function definition. The count of argument types and arguments don't match. {:?} {:?} {:?}", type_signature, args, &tokens[0]),
                                ))
                            }

                            let body = get_function_def_body(&tokens, arrow);
                            if body.len() == 0 {
                                // TODO
                                // use this to check for multiline function
                                
                                // optional check if any multiline variable assignments
                                // (or just let main parser deal with those regardless of whitespace - in which case need to scope any variable checks to just within this function)
                                // e.g. change self.functions from Vec<FunctionName> Vec(FunctionName, ScopedParentFunctionName)
                                // and add a self.currentScopedParentFunctionName = default to "global" (assuming we will make users start code with = "main \ =>" like rust)
        
                                // for now just error
                                return Err(self.get_error(
                                    3 + identifier.len(),
                                    10, //text.len(),
                                    &format!("is not a valid function definition. There is no function body following the => {:?}", &tokens[0]),
                                ))
                            } else {
                                // get return expression/value
                                // recursive
                                let expression_result =
                                self.check_expression(&identifier, body.clone());

                                dbg!(&tokens, slash, arrow, &type_signature, &args, &body, &expression_result);
                                //dbg!(&self);
                                match expression_result {
                                    Ok((expression, expression_type)) =>{
                                        // validate that expression type matches provided type
                                        if !type_signature[type_signature.len()-1].contains(&expression_type) && !expression_type.contains(&type_signature[type_signature.len()-1]) {
                                            return Err(self.get_error(
                                                3 + identifier.len(),
                                                10, //text.len(),
                                                &format!("The type of this function's return expression '{:?}' does not match it's signature's return type: {:?}", &expression_type, &type_signature[type_signature.len()-1]),
                                            ));
                                        }

                                // example
                                // fn check_function_definition(self: &mut Self) -> Result<Option<String>, String> {
                                //      return_expression    
                                //}
                                
                                let mut args_with_types="".to_string();
                                for i in 0..args.len() {
                                    let comma_not_first = if i==0 {"".to_string()} else {", ".to_string()};
                                    args_with_types = format!("{}{}{}: {}", args_with_types, comma_not_first, args[i], type_signature[i]);
                                }
                                let expression_indent = " ".repeat((self.indent+1) * 4);
                                let trailing_brace_indent = " ".repeat(self.indent * 4);
                                let output = format!("({}) -> {} {{\r\n{}{}\r\n{}}}", args_with_types, type_signature[type_signature.len()-1], expression_indent,&expression, trailing_brace_indent);
                                return Ok((output, "Function".to_string()));
                                    },
                                    Err(e) => return Err(e)
                                }

                                
                            }                            
                        }
                    },
                    _ => return Err(self.get_error(
                        3 + identifier.len(),
                        10, //text.len(),
                        &format!("is not a valid function definition. Missing slash or arrow. Should be in the form\r\n= func_name : i64 i64 \\ arg1 => + arg1 123\r\n{:?}", &tokens[0]),
                    ))
                }
            } else if self.is_function_call(&tokens[0]) {
                let fn_option: Option<FunctionDefinition> =
                    self.get_function_definition(tokens[0].clone());

                match fn_option {
                    Some((
                        function_name,
                        function_format,
                        function_args,
                        function_validation,
                        function_return_type,
                    )) => {
                        let allow_for_fn_name = 1;
                        let count_arguments = tokens.len() - allow_for_fn_name;
                        let tokens_string_length = self.get_tokens_string_len(&tokens);
                        let expression_indents = 3 + function_name.len();
                        let validate_arg_types_must_match =
                            function_validation.contains(&"arg_types_must_match".to_string());

                        let mut final_return_type = &function_return_type;

                        // check number of arguments
                        if function_args.len() != count_arguments {
                            return Err(self.get_error(
                                expression_indents,
                                tokens_string_length,
                                &format!(
                                    "wrong number of function arguments. Expected: {}. Found {}.",
                                    function_args.len(),
                                    count_arguments
                                ),
                            ));
                        }

                        // check types of values
                        let mut value_types: Vec<String> = vec![];
                        for i in allow_for_fn_name..tokens.len() {
                            value_types.push(self.get_type(&tokens[i]));
                        }
                        for i in 0..count_arguments {
                            if !function_args[i].contains(&value_types[i]) {
                                return Err(self.get_error(
                                expression_indents,
                                tokens_string_length,
                                &format!(
                                    "function arguments are not the correct types. Expected: {:?}. Found {:?}",
                                    function_args,
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
                            if value_types.clone().into_iter().any(|c| &c != first && !&c.contains(first) && !&first.contains(&c)) {
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

                        let output = match function_args.len() {
                            2 => {
                                let out1 =
                                    function_format.replace("#1", &tokens[allow_for_fn_name]);
                                let out2 = out1.replace("#2", &tokens[allow_for_fn_name + 1]);
                                out2
                            }
                            1 => {
                                let out1 =
                                    function_format.replace("#1", &tokens[allow_for_fn_name]);
                                out1
                            }
                            _ => function_format,
                        };

                        let get_type_from_referred_function = function_validation
                            .contains(&"get_type_from_referred_function".to_string());
                        if get_type_from_referred_function {
                            // this is variable assignment is just a reference to another constant (i.e. a function) e.g. let x: ? = a;
                            // So to determine the return type ? of x, we must get it from a
                            let fn_name = function_name.clone();
                            let final_return_type_from_parent_refs =
                                &self.recurs_get_referred_function(fn_name);
                            return Ok((output, final_return_type_from_parent_refs.clone()));
                        }

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
        }

        // or error if none of above
        let text: String = tokens.iter().map(String::as_str).collect();
        Err(self.get_error(
            3 + identifier.len(),
            text.len(),
            "is not a valid expression: must be either an: integer, e.g. 12345, float, e.g. 123.45, existing constant, e.g. x, string, e.g. \"string\", function call, e.g. + 1 2, function definition, e.g. \\ arg1 => arg1",
        ))
    }

    fn recurs_get_referred_function(self: &Self, function_name: String) -> String {
        let referred_function_option = self.get_function_definition(function_name.clone());
        match referred_function_option {
            Some((_, ref_func_name, _, ref_validations, ref_func_return_type)) => {
                
                let get_type_from_referred_function =
                    ref_validations.contains(&"get_type_from_referred_function".to_string());
                    //dbg!(&function_name, &ref_func_return_type, get_type_from_referred_function);
                if get_type_from_referred_function {
                    return self.recurs_get_referred_function(ref_func_name);
                } else {
                    return ref_func_return_type;
                };
            }
            None => return "".to_string(),
        };
    }

    fn get_type(self: &Self, text: &String) -> String {
        if self.is_integer(text) {
            return "i64".to_string();
        }
        if self.is_float(text) {
            return "f64".to_string();
        }
        if self.is_string(text) {
            return "String".to_string();
        }
        if self.is_function_call(text) {
            let def_option = self.get_function_definition(text.to_string());
            match def_option {
                Some((_,_,_,function_validation,mut return_type)) => {
                    let get_type_from_referred_function = function_validation
                            .contains(&"get_type_from_referred_function".to_string());
                        
                    if get_type_from_referred_function && return_type == "".to_string() {
                        return_type = self.recurs_get_referred_function(text.to_string())
                    }
                        return return_type
                    
                    
                },
                _ =>{
                    return "Function".to_string();
                }
            }
            
        }
        if self.is_lambda(text) {
            return "FunctionDef".to_string();
        }
        "Undefined".to_string()
    }

    fn is_integer(self: &Self, text: &String) -> bool {
        let mut is_valid = true;
        if !text.chars().into_iter().all(|c| c.is_numeric()) {
            is_valid = false;
        }
        is_valid
    }

    fn is_float(self: &Self, text: &String) -> bool {
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

    fn is_string(self: &Self, text: &String) -> bool {
        let mut is_valid = true;
        let char_vec: Vec<char> = text.chars().collect();
        if text.len() < 2 || char_vec[0] != '"' || char_vec[text.len() - 1] != '"' {
            is_valid = false;
        }
        is_valid
    }

    fn is_lambda(self: &Self, text: &String) -> bool {
        let mut is_valid = true;
        let char_vec: Vec<char> = text.chars().collect();
        if char_vec[0] != '\\' {
            is_valid = false;
        }
        is_valid
    }

    fn is_function_call(self: &Self, text: &String) -> bool {
        self.exists_function(&text)
    }

    /*
    fn get_function_def(self: &Self, text: &String) -> () {
        let mut is_valid = true;
        let (slash, remain1) = get_until_whitespace_or_eol_or_eof(text.clone());
        let arrow_option = remain1.find("=>");
        let mut args: Vec<String> = vec![];
        match arrow_option {
            Some(arrow) => {
                let mut remain2 = strip_trailing_whitespace(strip_leading_whitespace(
                    remain1[0..arrow].to_string(),
                ));
                while remain2.len() > 0 {
                    let (arg, remainder) = get_until_whitespace_or_eol_or_eof(remain2.clone());
                    remain2 = remainder.clone();
                    args.push(arg);
                }
            }
            _ => (),
        };
        //let (slash, remain2) = get_until_whitespace_or_eol_or_eof(strip_leading_whitespace(remain1));
    }
    */

    fn get_error(self: &Self, arrow_indent: usize, arrow_len: usize, error: &str) -> String {
        format!(
            "----------\r\n{}\r\n{}{} {}",
            self.lines_of_chars[self.pass]
                .iter()
                .cloned()
                .collect::<String>(),
            " ".repeat(arrow_indent),
            "^".repeat(arrow_len),
            error,
        )
    }

    fn check_comment_single_line(self: &mut Self) -> Result<Option<String>, String> {
        let tokens = &self.lines_of_tokens[self.pass];
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
}

fn concatenate_vec_strings(tokens: &Vec<String>) -> String {
    let mut output = "".to_string();
    for i in 0..tokens.len() {
        output = format!("{}{}", output, tokens[i]);
    }
    output
}
struct Errors {
    invalid_program_syntax: &'static str,
    variable_assignment: &'static str,
    function_definition: &'static str,
    no_valid_identifier_found: &'static str,
    no_valid_comment_single_line: &'static str,
    no_valid_expression: &'static str,
    constants_are_immutable: &'static str,
}

const ERRORS: Errors = Errors {
    invalid_program_syntax: "Invalid program syntax. Must start with RUN, followed by linebreak, optional commands and linebreak, and end with END",
    variable_assignment: "Invalid variable assignment. Must contain Int or Float, e.g. x = Int 2",
    function_definition: "Invalid function definition. e.g. = \\ function_name arg1 arg2 => stuff",
    no_valid_identifier_found:"No valid identifier found",
    no_valid_comment_single_line: "No valid single line comment found",
    no_valid_expression: "No valid expression was found",
    constants_are_immutable: "Constants are immutable. You may be trying to assign a value to a constant that has already been defined. Try renaming this as a new constant."
};

fn get_identifier(input: String) -> Result<(String, String), String> {
    let (identifier, remainder) = get_until_whitespace_or_eof(input.clone());
    let char_vec: Vec<char> = identifier.chars().collect();
    if identifier == "".to_string() {
        Err(ERRORS.no_valid_identifier_found.to_string())
    } else {
        for i in 0..identifier.len() {
            let c = char_vec[i];
            if i == 0 {
                if !c.is_alphabetic() && !(c == '_') {
                    // must start with a letter or underscore

                    return Err(ERRORS.no_valid_identifier_found.to_string());
                }
            } else {
                if !c.is_alphanumeric() && !(c == '_') {
                    {
                        // all other chars must be letter or number or underscore

                        return Err(ERRORS.no_valid_identifier_found.to_string());
                    }
                }
            }
        }
        Ok((identifier, remainder))
    }
}

fn get_str(input: String, matchstr: &str) -> Result<String, String> {
    let (identifier, remainder) = get_until_whitespace_or_eof(input.clone());
    if identifier == "".to_string() || &identifier != matchstr {
        return Err(ERRORS.no_valid_identifier_found.to_string());
    }
    Ok(remainder)
}

fn get_until_whitespace_or_eof(input: String) -> (String, String) {
    let mut output = "".to_string();
    let mut remainder = "".to_string();
    let char_vec: Vec<char> = input.chars().collect();
    for i in 0..input.len() {
        if i == input.len() {
            remainder = "".to_string();
        } else {
            if char_vec[i].is_whitespace() {
                remainder = input[i..].to_string();
                break;
            } else {
                remainder = input[i + 1..].to_string();
                output.push(char_vec[i]);
            }
        }
    }
    (output, remainder)
}

fn get_until_eol_or_eof(input: String) -> (String, String) {
    let mut output = "".to_string();
    let mut remainder = "".to_string();
    let char_vec: Vec<char> = input.chars().collect();
    for i in 0..input.len() {
        if i == input.len() {
            remainder = "".to_string();
        } else {
            if char_vec[i] == '\r' {
                remainder = input[i..].to_string();
                break;
            } else {
                remainder = input[i + 1..].to_string();
                output.push(char_vec[i]);
            }
        }
    }
    (output, remainder)
}

/*
fn get_until_whitespace_or_eol_or_eof(input: String) -> (String, String) {
    let mut output = "".to_string();
    let mut remainder = "".to_string();
    let char_vec: Vec<char> = input.chars().collect();
    for i in 0..input.len() {
        if i == input.len() {
            remainder = "".to_string();
        } else {
            if char_vec[i] == '\r' || char_vec[i].is_whitespace() {
                remainder = input[i..].to_string();
                break;
            } else {
                remainder = input[i + 1..].to_string();
                output.push(char_vec[i]);
            }
        }
    }
    (output, remainder)
}
*/

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

/*
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
*/

fn is_function_definition(tokens: &Vec<String>) -> bool {
    tokens.len() > 1 && tokens[0] == ":".to_string()
    //&& tokens[1..].iter().any(|c| c == &"\\".to_string())
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
    return tokens[slash+2..arrow+1].to_vec();
}

fn get_function_def_body(tokens: &Vec<String>, arrow: usize) -> Vec<String> {
    return tokens[arrow+2..].to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_config(contents: &str) -> Config {
        Config {
            filename: "".to_string(),
            filecontents: contents.to_string(),
            remaining: contents.to_string(),
            lines_of_chars: vec![],
            lines_of_tokens: vec![],
            output: "".to_string(),
            outputcursor: 0,
            pass: 0,
            indent: 1,

            functions: vec![],
            error_stack: vec![],
        }
    }

    #[test]
    fn test_new() {
        let args = ["toylang".to_string(), "filename_example".to_string()];
        let config_result = Config::new(&args);
        let filename = "filename_example".to_string();
        match config_result {
            Ok(config) => assert_eq!(config.filename, filename),
            Err(_) => assert!(false, "error should not exist"),
        }
    }

    /*
    #[test]
    fn test_tokenizer_assignment() {
        let mut config = mock_config("RUN\r\n= x 2\r\nEND");
        match config.tokenizer() {
            Ok(_) => {
                assert_eq!(config.output, "fn main() {\r\n    let x = 2;\r\n}");
                assert_eq!(config.outputcursor, 29);
            }
            Err(_) => assert!(false, "error should not exist"),
        }
    }

    #[test]
    fn test_tokenizer_assignment_immutable_ok() {
        let mut config = mock_config("RUN\r\n= x 2\r\n= y 3\r\nEND");
        match config.tokenizer() {
            Ok(_) => {
                assert_eq!(
                    config.output,
                    "fn main() {\r\n    let x = 2;\r\n    let y = 3;\r\n}"
                );
                assert_eq!(config.outputcursor, 45);
            }
            Err(_) => assert!(false, "error should not exist"),
        }
    }
    */

    /*
    #[test]
    fn test_check_program_syntax() {
        let err = Err(ERRORS.invalid_program_syntax);
        assert_eq!(mock_config("").check_program_syntax(), err);
        assert_eq!(mock_config("commands").check_program_syntax(), err);
        assert_eq!(mock_config("RUN").check_program_syntax(), err);
        assert_eq!(
            mock_config("RUN\r\ncommands\r\n").check_program_syntax(),
            err
        );
        assert_eq!(mock_config("END").check_program_syntax(), err);
        assert_eq!(mock_config("commands\r\nEND").check_program_syntax(), err);
        assert_eq!(mock_config("RUNEND").check_program_syntax(), err);
        assert_eq!(mock_config("END\r\nRUN").check_program_syntax(), err);
        assert_eq!(mock_config("RUN commands END").check_program_syntax(), err);
        assert_eq!(
            mock_config("RUN\r\n//comment\r\nEND").check_program_syntax(),
            Ok(())
        );
        assert_eq!(
            mock_config("RUN\r\ncommands\r\nEND").check_program_syntax(),
            Ok(())
        );
        assert_eq!(
            mock_config("RUN\r\ncommands\r\ncommands\r\ncommands\r\nEND").check_program_syntax(),
            Ok(())
        );
    }
    */

    #[test]
    fn test_check_variable_assignment() {
        let err: Result<Option<String>, String> = Err(ERRORS.variable_assignment.to_string());
        let err2: Result<Option<String>, String> =
            Err(ERRORS.no_valid_identifier_found.to_string());
        assert_eq!(mock_config("").check_variable_assignment(), err);
        assert_eq!(mock_config("2 = x").check_variable_assignment(), err2);
        assert_eq!(mock_config("let x = 2").check_variable_assignment(), err2);
        //assert_eq!(check_variable_assignment("x = 2".to_string()), err);
        //assert_eq!(check_variable_assignment("x = Abc 2".to_string()), err);
        //assert_eq!(check_variable_assignment("x = Boats 2".to_string()), err);
        //assert_eq!(check_variable_assignment("x = Monkey 2".to_string()), err);

        //OK
        assert_eq!(mock_config("= x 2").check_variable_assignment(), Ok(None));
        assert_eq!(mock_config("= x 2.2").check_variable_assignment(), Ok(None));
        assert_eq!(
            mock_config("= x 1\r\n= y x").check_variable_assignment(),
            Ok(None)
        );
        assert_eq!(
            mock_config("= x \"string\"").check_variable_assignment(),
            Ok(None)
        );
    }

    #[test]
    fn test_get_until_whitespace_or_eof() {
        assert_eq!(
            get_until_whitespace_or_eof("".to_string()),
            ("".to_string(), "".to_string())
        );
        assert_eq!(
            get_until_whitespace_or_eof("abc".to_string()),
            ("abc".to_string(), "".to_string())
        );
        assert_eq!(
            get_until_whitespace_or_eof("abc def".to_string()),
            ("abc".to_string(), " def".to_string())
        );
        assert_eq!(
            get_until_whitespace_or_eof("abc\r\ndef".to_string()),
            ("abc".to_string(), "\r\ndef".to_string())
        );
        assert_eq!(
            get_until_whitespace_or_eof(" abc".to_string()),
            ("".to_string(), " abc".to_string())
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
    #[test]
    fn test_get_identifier() {
        let err = Err(ERRORS.no_valid_identifier_found.to_string());
        assert_eq!(get_identifier("".to_string()), err);
        assert_eq!(get_identifier("  abc".to_string()), err);
        assert_eq!(get_identifier("1abc = 123".to_string()), err);
        assert_eq!(get_identifier("-abc = 123".to_string()), err);
        assert_eq!(
            get_identifier("abc".to_string()),
            Ok(("abc".to_string(), "".to_string()))
        );
        assert_eq!(
            get_identifier("_abc".to_string()),
            Ok(("_abc".to_string(), "".to_string()))
        );
        assert_eq!(
            get_identifier("abc = 123".to_string()),
            Ok(("abc".to_string(), " = 123".to_string()))
        );
        assert_eq!(
            get_identifier("abc_123def = 123".to_string()),
            Ok(("abc_123def".to_string(), " = 123".to_string()))
        );
    }
    #[test]
    fn test_get_str() {
        let err = Err(ERRORS.no_valid_identifier_found.to_string());
        assert_eq!(get_str("".to_string(), ""), err);
        assert_eq!(get_str("  abc".to_string(), "abc"), err);
        assert_eq!(get_str("1abc = 123".to_string(), "abc"), err);
        assert_eq!(get_str("-abc = 123".to_string(), "abc"), err);
        assert_eq!(get_str("abc".to_string(), "abc"), Ok("".to_string()));
        assert_eq!(get_str("_abc".to_string(), "_abc"), Ok("".to_string()));
        assert_eq!(
            get_str("abc = 123".to_string(), "abc"),
            Ok(" = 123".to_string())
        );
        assert_eq!(
            get_str("abc_123def = 123".to_string(), "abc_123def"),
            Ok(" = 123".to_string())
        );
    }
}
