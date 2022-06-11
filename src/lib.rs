use std::error::Error;
use std::fs;

type FunctionName = &'static str;
type FunctionFormat = String;
type FunctionType = &'static str;
type FunctionValidation = String;
type FunctionReturnType = &'static str;
type FunctionDefinition = (
    FunctionName,
    FunctionFormat,
    Vec<FunctionType>,
    Vec<FunctionValidation>,
    FunctionReturnType,
);
type Expression = &'static str;
type ExpressionType = &'static str;

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

#[derive(Clone, Debug)]
pub struct Config {
    pub filename: &'static str,
    pub filecontents: String,
    pub lines_of_chars: Vec<Vec<char>>,
    pub lines_of_tokens: Vec<Vec<&'static str>>,
    pub output: &'static str,
    pub outputcursor: usize,
    pub pass: usize,
    pub indent: usize,
    pub functions: Vec<FunctionDefinition>,
    pub error_stack: Vec<String>,
}

impl Config {
    pub fn new(args: &[&'static str]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("missing filename argument");
        }
        let filename = args[1].clone();
        let filecontents = "".to_string();

        let lines_of_chars = vec![];
        let lines_of_tokens = vec![];
        let output = "";
        let outputcursor = 0;
        let pass = 0;
        let indent = 0;
        let arithmetic_primitives = vec!["+", "-", "*", "/", "%"];
        let arithmetic_operators: Vec<FunctionDefinition> = arithmetic_primitives
            .into_iter()
            .map(|prim| {
                let text = format!("#1 {} #2", prim);
                (
                    prim,
                    text,
                    vec!["i64|f64", "i64|f64"],
                    vec!["arg_types_must_match".to_string()],
                    "",
                )
            })
            .collect();
        let function_def: FunctionDefinition = (
            "\\",
            "#0fn #1(#2) {\r\n#3\r\n#0}\r\n".to_string(),
            vec![],
            vec!["lambda".to_string()],
            "",
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
        //self.get_lines_of_tokens();
        //self.run_main_loop()?;
        if self.error_stack.len() == 0 {
            fs::write("../../src/bin/output.rs", &self.output)?;
        } else {
            println!("DIDN'T SAVE - error stack: {:?}", self.error_stack);
        }
        Ok(())
    }

    fn get_lines_of_chars(self: &mut Self) {
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
        //println!("@@ {:?}", self.lines_of_chars);
    }

    fn get_lines_of_tokens(self: &mut Self) {
        for line in 0..self.lines_of_chars.len() {
            let mut index_from = 0;
            let mut index_to = 0;
            let char_vec: Vec<char> = self.lines_of_chars[line].clone();
            //println!("line: {}", line);
            let mut inside_quotes = false;
            let mut line_of_tokens: Vec<&str> = vec![];
            while index_to < char_vec.len() {
                let c = char_vec[index_to];
                //println!(
                //    "line: {}, index_from: {}, index_to: {} - '{}'",
                //    line, index_from, index_to, c
                //);
                let eof = index_to == char_vec.len() - 1;
                inside_quotes = if c == '"' {
                    !inside_quotes
                } else {
                    inside_quotes
                };
                if (c.is_whitespace() && index_to != 0 && !inside_quotes) || eof {
                    let token_chars = char_vec[index_from..index_to + (if eof { 1 } else { 0 })]
                        .iter()
                        .cloned()
                        .collect::<String>();
                    //println!("token_chars:{:}", token_chars);
                    line_of_tokens.push(token_chars);
                    index_from = index_to + 1;
                }
                index_to = index_to + 1;
            }
            self.lines_of_tokens.push(line_of_tokens);
        }
        //println!("@@ {:?}", self.lines_of_tokens);
    }
}
/*






    fn run_main_loop(self: &mut Self) -> Result<(), &str> {
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
        //if self.check_one_succeeds("check_function_definition") {
        //    return Ok(());
        //}
        if self.check_one_succeeds("check_variable_assignment") {
            //println!("succeeded");
            return Ok(());
        }
        //if self.check_one_succeeds("check_comment_single_line") {
        //    return Ok(());
        //}
        let e = self.get_error(0, 1, ERRORS.no_valid_expression);
        self.error_stack.push(e);
        Err(self.error_stack.clone())
    }

    fn check_one_succeeds(self: &mut Self, function_name: &str) -> bool {
        let mut succeeded = false;
        let mut clone = self.clone();
        let result = match function_name {
            //"check_function_definition" => clone.check_function_definition(),
            "check_variable_assignment" => clone.check_variable_assignment(),
            //"check_comment_single_line" => clone.check_comment_single_line(),
            _ => {
                //println!("check_one_succeeds: provided an unknown function_name");
                return false;
            }
        };
        //println!("check result {:?} {:?}", function_name, result);
        match result {
            Ok(validation_error) => {
                self.clone_mut_ref(clone);
                match validation_error {
                    Some(e) => {
                        //println!("one_succeeds e{:?}", e);
                        self.error_stack.push(e);
                        succeeded = false;
                    }
                    None => succeeded = true,
                }
            }
            Err(_e) => println!("error"), //self.error_stack.push(e), // just testing - move to a temporary error_stack
        }
        succeeded
    }

    fn clone_mut_ref(self: &mut Self, to_clone: Config) -> () {
        // wokraround - can't just do 'self = clone.clone();' due to &mut derferencing ??
        self.filename = to_clone.filename;
        self.filecontents = to_clone.filecontents;
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
            || self.lines_of_tokens[0][0] != "RUN"
            || self.lines_of_tokens[self.lines_of_tokens.len() - 1][0] != "END"
        {
            self.error_stack
                .push(ERRORS.invalid_program_syntax.to_string());
            return Err(self.error_stack.clone());
            //return Err();
        } else {
            self.output = "fn main() {\r\n}";
            self.indent = 1;
            self.outputcursor = 13; // anything new will be inserted before end bracket
        }

        Ok(())
    }

    fn check_variable_assignment(self: &'static mut Self) -> Result<Option<String>, String> {
        let tokens = &self.lines_of_tokens[self.pass];
        if tokens[1] == "c" {
            //println!("tokens {:?}", &tokens);
        }
        if tokens.len() < 2 || tokens[0] != "=" {
            return Err(ERRORS.variable_assignment.to_string());
        } else {
            let identifier = tokens[1].clone();
            let mut validation_error = None;
            if self.exists_function(&identifier) {
                let e = self.get_error(2, identifier.len(), ERRORS.constants_are_immutable);
                validation_error = Some(e.to_string());
            }

            let expression_result =
                self.check_expression(&identifier, tokens[2..tokens.len()].to_vec());
            if tokens[1] == "c" {
                //println!(
                //    "expression_result {:?} {:?}",
                //    expression_result,
                //    tokens[2]
                //);
            }

            match expression_result {
                Ok((expression, exp_type)) => {
                    let type_colon = if exp_type.len() == 0 { "" } else { ": " };
                    let mut value = expression;
                    let mut final_type = exp_type.clone();
                    let mut validations: Vec<String> = vec![];

                    if tokens[1] == "c" {
                        //println!("!!!!!!!!!!!!!!!");
                    }
                    if self.exists_function(&tokens[2]) {
                        //println!("!!!!!!!!!!!!!!! {:?}", tokens[2]);
                        final_type = "";
                        validations.push("get_type_from_referred_function".to_string());
                        value = tokens[2];
                    }

                    let insert = &format!(
                        "{}let {}{}{} = {};\r\n",
                        " ".repeat(self.indent * 4),
                        &identifier,
                        type_colon,
                        &exp_type,
                        &value
                    );

                    let new_constant_function =
                        (identifier, value, vec![], validations, final_type);
                    if tokens[1] == "c" {
                        //println!("!!!!!!!!!!!!!!! {:?}", new_constant_function);
                    }
                    self.functions.push(new_constant_function);
                    self.output
                        .to_string()
                        .insert_str(self.outputcursor, &insert);
                    self.outputcursor = self.outputcursor + insert.len();
                }
                Err(e) => validation_error = Some(e.to_string()),
                _ => validation_error = Some("some other error".to_string()),
            }

            Ok(validation_error)
        }
    }

    fn exists_function(self: &Self, function_name: &str) -> bool {
        self.functions.iter().any(|c| c.0 == function_name)
    }

    fn get_function_definition(self: &Self, function_name: &str) -> Option<FunctionDefinition> {
        let funcs: Vec<&FunctionDefinition> = self
            .functions
            .iter()
            .filter(|c| c.0 == function_name)
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
            .filter(|c| c.0 == function_name)
            .collect::<Vec<_>>();
        if function_vec.len() == 0 {
            None
        } else {
            Some(function_vec[0].clone())
        }
    }

    fn get_tokens_string_len(self: &Self, tokens: &Vec<&str>) -> usize {
        let mut total = 0;
        for i in 0..tokens.len() {
            total += tokens[i].len();
        }
        let num_spaces_inbetween = total - 1;
        total + num_spaces_inbetween
    }

    fn check_expression(
        self: &'static Self,
        identifier: &str,
        tokens: Vec<&'static str>,
    ) -> Result<(Expression, ExpressionType), String> {
        if tokens[0] == "c" {
            dbg!("check_expression", identifier, &tokens);
        }
        if tokens.len() == 1 {
            if self.is_integer(&tokens[0]) {
                return Ok((tokens[0].clone(), "i64"));
            }
            if self.is_float(&tokens[0]) {
                return Ok((tokens[0].clone(), "f64"));
            }

            let possible_string: Vec<char> = tokens[0].chars().collect();
            if possible_string[0] == '\"' && possible_string[possible_string.len() - 1] == '\"' {
                return Ok((format!("{}{}", tokens[0], "").as_str(), "String"));
            }
        }

        if tokens.len() > 0 && self.is_function_call(&tokens[0]) {
            let fn_option: Option<FunctionDefinition> = self.get_function_definition(&tokens[0]);
            if (tokens[0] == "c") {
                dbg!("ARGH {:?}", &fn_option);
            };
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
                    if tokens[0] == "c" {
                        dbg!(function_name);
                    }
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
                    let mut value_types: Vec<&str> = vec![];
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
                        if *final_return_type == "" {
                            final_return_type = first;
                        };
                        //println!("###first {}", first);
                        if value_types.clone().into_iter().any(|c| &c != first) {
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
                            let out1 = function_format
                                .replace("#1", &tokens[allow_for_fn_name])
                                .as_str();
                            let out2 = out1.replace("#2", &tokens[allow_for_fn_name + 1]).as_str();
                            out2
                        }
                        1 => {
                            let out1 = function_format
                                .replace("#1", &tokens[allow_for_fn_name])
                                .as_str();
                            out1
                        }
                        _ => function_format,
                    };

                    let get_type_from_referred_function = function_validation
                        .contains(&"get_type_from_referred_function".to_string());
                    if get_type_from_referred_function {
                        // this is variable assignment is just a reference to another constant (i.e. a function) e.g. let x: ? = a;
                        // So to determine the return type ? of x, we must get it from a

                        let testy = &self.recurs_get_referred_function(function_name);
                    }

                    //println!("ARGH2 {:?}", output);
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

        // or error if none of above
        //let text: String = tokens.iter().collect();

        //        let text: &str = tokens.iter().map(|s| *s.to_string()).collect();
        Err(self.get_error(
            3 + identifier.len(),
            tokens.len(),
            "is not a valid expression: must be either an: integer, e.g. 12345, float, e.g. 123.45, existing constant, e.g. x, string, e.g. \"string\", function, e.g. + 1 2",
        ))
    }

    fn recurs_get_referred_function(self: &Self, function_name: &str) -> &str {
        let referred_function_option = self.get_function_definition(&function_name.clone());
        match referred_function_option {
            Some((ref_func_name, _, _, ref_validations, ref_func_return_type)) => {
                let get_type_from_referred_function =
                    ref_validations.contains(&"get_type_from_referred_function".to_string());
                if get_type_from_referred_function {
                    return self.recurs_get_referred_function(&ref_func_name);
                } else {
                    return ref_func_return_type;
                };
            }
            None => return "",
        };
    }

    fn get_type(self: &Self, text: &str) -> &str {
        if self.is_integer(text) {
            return "i64";
        }
        if self.is_float(text) {
            return "f64";
        }
        if self.is_string(text) {
            return "String";
        }
        if self.is_function_call(text) {
            return "Function";
        }
        if self.is_lambda(text) {
            return "FunctionDef";
        }
        "Undefined"
    }

    fn is_integer(self: &Self, text: &str) -> bool {
        //println!("is_integer? {}", text);
        let mut is_valid = true;
        if !text.chars().into_iter().all(|c| c.is_numeric()) {
            //println!("iter? {}", text);
            is_valid = false;
        }
        is_valid
    }

    fn is_float(self: &Self, text: &str) -> bool {
        //println!("is_float? {}", text);
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

    fn is_string(self: &Self, text: &str) -> bool {
        //println!("is_string? {}", text);
        let mut is_valid = true;
        let char_vec: Vec<char> = text.chars().collect();
        if text.len() < 2 || char_vec[0] != '"' || char_vec[text.len() - 1] != '"' {
            is_valid = false;
        }
        is_valid
    }

    fn is_lambda(self: &Self, text: &str) -> bool {
        let mut is_valid = true;
        let char_vec: Vec<char> = text.chars().collect();
        if char_vec[0] != '\\' {
            is_valid = false;
        }
        is_valid
    }

    fn is_function_call(self: &Self, text: &str) -> bool {
        self.exists_function(&text)
    }

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
}

fn get_identifier(input: &str) -> Result<(&str, &str), &str> {
    let (identifier, remainder) = get_until_whitespace_or_eof(input.clone());
    let char_vec: Vec<char> = identifier.chars().collect();
    if identifier == "" {
        //println!("empty string?");
        Err(ERRORS.no_valid_identifier_found)
    } else {
        for i in 0..identifier.len() {
            let c = char_vec[i];
            if i == 0 {
                if !c.is_alphabetic() && !(c == '_') && !(c == '\\' && identifier.len() == 1) {
                    // must start with a letter or underscore (or '\' if function definition)
                    //println!("letter or underscore?");
                    return Err(ERRORS.no_valid_identifier_found);
                }
            } else {
                if !c.is_alphanumeric() && !(c == '_') {
                    {
                        // all other chars must be letter or number or underscore
                        //println!("alphanumeric?");
                        return Err(ERRORS.no_valid_identifier_found);
                    }
                }
            }
        }
        Ok((identifier, remainder))
    }
}

fn get_str(input: &'static str, matchstr: &str) -> Result<&'static str, &'static str> {
    let (identifier, remainder) = &get_until_whitespace_or_eof(input);
    if *identifier == "" || identifier != &matchstr {
        //println!("get_str");
        return Err(ERRORS.no_valid_identifier_found);
    }
    Ok(&remainder)
}

fn get_until_whitespace_or_eof(input: &str) -> (&str, &str) {
    let mut output: String = "".to_string();
    let mut remainder = "";
    let char_vec: Vec<char> = input.chars().collect();
    for i in 0..input.len() {
        if i == input.len() {
            remainder = "";
        } else {
            if char_vec[i].is_whitespace() {
                remainder = &input[i..];
                break;
            } else {
                remainder = &input[i + 1..];
                output.push(char_vec[i]);
            }
        }
    }
    (output.as_str(), remainder)
}

fn strip_leading_whitespace(input: &str) -> &str {
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
        return "";
    }
    &input[first_non_whitespace_index..]
}

fn strip_trailing_whitespace(input: &str) -> &str {
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
        return "";
    }
    &input[..first_non_whitespace_index]
}
*/
