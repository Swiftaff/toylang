use std::error::Error;
use std::fs;

type FunctionName = String;
type FunctionFormat = String;
type FunctionType = String;
type FunctionValidation = String;
type FunctionDefinition = (
    FunctionName,
    FunctionFormat,
    Vec<FunctionType>,
    Vec<FunctionValidation>,
);

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
    pub constants: Vec<String>,
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
        let constants = vec![];
        let arithmetic_primitives = vec!["+", "-", "*", "/", "%"];
        let arithmetic_operators: Vec<FunctionDefinition> = arithmetic_primitives
            .into_iter()
            .map(|prim| {
                (
                    prim.to_string(),
                    format!("#1 {} #2", prim).to_string(),
                    vec!["Int|Float".to_string(), "Int|Float".to_string()],
                    vec!["arg_types_must_match".to_string()],
                )
            })
            .collect();
        let function_def: FunctionDefinition = (
            "\\".to_string(),
            "#0fn #1(#2) {\r\n#3\r\n#0}\r\n".to_string(),
            vec![],
            vec!["lambda".to_string()],
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
            constants,
            functions,
            error_stack,
        })
    }

    pub fn run(self: &mut Self) -> Result<(), Box<dyn Error>> {
        self.filecontents = fs::read_to_string(&self.filename)?;
        println!(
            "\r\nINPUT contents of filename: {:?}\n----------\n{}",
            &self.filename, &self.filecontents
        );
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
        println!("@@ {:?}", self.lines_of_chars);
    }

    fn get_lines_of_tokens(self: &mut Self) {
        for line in 0..self.lines_of_chars.len() {
            let mut index_from = 0;
            let mut index_to = 0;
            let char_vec: Vec<char> = self.lines_of_chars[line].clone();
            //println!("line: {}", line);
            //let mut inside_quotes = false;
            let mut line_of_tokens: Vec<String> = vec![];
            while index_to < char_vec.len() {
                println!(
                    "line: {}, index_from: {}, index_to: {}",
                    line, index_from, index_to
                );
                let c = char_vec[index_to];
                let eof = index_to == char_vec.len() - 1;
                if (c.is_whitespace() && index_to != 0) || eof {
                    let token_chars = char_vec[index_from..index_to + (if eof { 1 } else { 0 })]
                        .iter()
                        .cloned()
                        .collect::<String>();
                    println!("token_chars:{:?}", token_chars);
                    line_of_tokens.push(token_chars);
                    index_from = index_to + 1;
                }
                index_to = index_to + 1;
            }
            self.lines_of_tokens.push(line_of_tokens);
        }
        println!("@@ {:?}", self.lines_of_tokens);
    }

    fn run_main_loop(self: &mut Self) -> Result<(), String> {
        //ref: https://doc.rust-lang.org/reference/tokens.html

        match self.main_loop() {
            Ok(()) => {
                println!(
                    "----------\r\n\r\nToylang compiled successfully:\r\n----------\r\n{}\r\n----------\r\n",
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

    fn main_loop(self: &mut Self) -> Result<(), Vec<String>> {
        while self.remaining.len() > 0 {
            //println!("pass:{:?}", self.pass);
            self.check_program_syntax()?;
            //println!("{:?} {:?}\n", self.output, self.outputcursor);
            //input = check_for strings (because they might have spaces)
            self.check_one_or_more_succeeds()?;
            self.pass = self.pass + 1;
        }
        Ok(())
    }

    fn check_one_or_more_succeeds(self: &mut Self) -> Result<(), Vec<String>> {
        //println!("e0:{:?}... r::{:?}", self.remaining, self.error_stack);
        if self.check_one_succeeds("check_function_definition") {
            return Ok(());
        }
        if self.check_one_succeeds("check_variable_assignment") {
            return Ok(());
        }
        if self.check_one_succeeds("check_comment_single_line") {
            return Ok(());
        }
        let e = self.get_error(0, 1, ERRORS.no_valid_expression);
        self.error_stack.push(e.to_string());
        //println!("e3:{:?}", self.error_stack);
        //println!("{:?}", self);
        Err(self.error_stack.clone())
    }

    fn check_one_succeeds(self: &mut Self, function_name: &str) -> bool {
        let mut succeeded = false;
        let mut clone = self.clone();
        let result = match function_name {
            "check_function_definition" => clone.check_function_definition(),
            "check_variable_assignment" => clone.check_variable_assignment(),
            "check_comment_single_line" => clone.check_comment_single_line(),
            _ => {
                //println!("check_one_succeeds: provided an unknown function_name");
                return false;
            }
        };
        println!("check result {:?} {:?}", function_name, result);
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
            Err(e) => println!("error"), //self.error_stack.push(e), // just testing - move to a temporary error_stack
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
        self.constants = to_clone.constants;
        self.functions = to_clone.functions;
        self.error_stack = to_clone.error_stack;
    }

    fn check_program_syntax(self: &mut Self) -> Result<(), Vec<String>> {
        if self.pass == 0 {
            if self.remaining.len() < 8 {
                self.error_stack
                    .push(ERRORS.invalid_program_syntax.to_string());
                return Err(self.error_stack.clone());
                //return Err();
            } else {
                let starts_with_run = &self.remaining[..5] == "RUN\r\n";
                if !starts_with_run {
                    self.error_stack
                        .push(ERRORS.invalid_program_syntax.to_string());
                    return Err(self.error_stack.clone());
                    //return Err(ERRORS.invalid_program_syntax);
                }
                self.remaining = self.remaining[5..].to_string();
                //println!("input = {:?}\n", &self.remaining);

                let ends_with_end = &self.remaining[&self.remaining.len() - 3..] == "END";
                if !ends_with_end {
                    self.error_stack
                        .push(ERRORS.invalid_program_syntax.to_string());
                    return Err(self.error_stack.clone());
                    //return Err(ERRORS.invalid_program_syntax);
                }
                self.remaining = self.remaining[..self.remaining.len() - 3].to_string();
                //println!("input = {:?}\n", &self.remaining);
                self.output = "fn main() {\r\n}".to_string();
                self.indent = 1;
                self.outputcursor = 13; // anything new will be inserted before end bracket
            }
        }
        Ok(())
    }

    fn check_variable_assignment(self: &mut Self) -> Result<Option<String>, String> {
        if self.remaining.len() < 3 {
            //println!("error here?");
            return Err(ERRORS.variable_assignment.to_string());
        } else {
            // TODO - return more errors throughout, fix tests and add new function to optionally 'try' various options and ignore errors instead
            let mut remainder = strip_leading_whitespace(self.remaining.clone());
            //println!("check_var {:?}", self);
            remainder = get_str(remainder.clone(), "=")?;

            remainder = strip_leading_whitespace(remainder);
            let (identifier, mut remainder) = get_identifier(remainder)?;
            println!("##id{}", identifier);
            let mut validation_error = None;
            if self.exists_constant(&identifier) {
                let e = self.get_error(2, identifier.len(), ERRORS.constants_are_immutable);
                validation_error = Some(e);
            }

            remainder = strip_leading_whitespace(remainder[(&identifier.len() + 0)..].to_string());
            let (text, remain) = get_until_eol_or_eof(remainder);

            let expression_result = self.check_expression(&identifier);
            match expression_result {
                Ok(Some(expression)) => {
                    let insert = &format!(
                        "{}let {} = {};\r\n",
                        " ".repeat(self.indent * 4),
                        &identifier,
                        &expression
                    );
                    self.constants.push(identifier);
                    self.output.insert_str(self.outputcursor, &insert);
                    self.outputcursor = self.outputcursor + insert.len();
                    self.remaining = strip_leading_whitespace(remain);
                }
                Err(e) => validation_error = Some(e),
                _ => validation_error = Some("some other error".to_string()),
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
            println!("##id{}", identifier);
            let mut validation_error = None;
            if identifier == "\\" {
                //check for pre-existing same function_name

                remainder =
                    strip_leading_whitespace(remainder[(&identifier.len() + 0)..].to_string());
                let (text, remain) = get_until_eol_or_eof(remainder);
                println!("### {} {} '{}'", identifier, text, remain);
                let function_name = text.clone(); // TODO need to fix this, assumes no args currently just for testing
                let fun = self.get_function(&"\\");
                match fun {
                    Some((_, function_format, function_args, function_validation)) => {
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
                        println!("DONE? {:?}", self);
                    }
                    _ => {
                        println!("NOPE - not a function definition");
                        validation_error = Some("NOPE - not a function definition".to_string());
                    }
                }
            } else {
                return Err(ERRORS.function_definition.to_string());
            }
            Ok(validation_error)
        }
    }

    fn exists_constant(self: &Self, constant: &str) -> bool {
        self.constants.iter().any(|c| c == &constant)
    }

    fn exists_function(self: &Self, function_name: &str) -> bool {
        self.functions.iter().any(|c| c.0 == *function_name)
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

    fn check_expression(self: &mut Self, identifier: &str) -> Result<Option<String>, String> {
        let remainder =
            strip_leading_whitespace(self.remaining.clone()[(identifier.len() + 2)..].to_string());
        let (text, remain) = get_until_eol_or_eof(remainder);
        //println!("EXPRESSION: {} {:?}", identifier, text);
        if self.get_type(&text) == "Undefined".to_string() && !self.exists_constant(&text) {
            return Err(self.get_error(
                3 + identifier.len(),
                text.len(),
                "is not a valid expression: must be either an: integer, e.g. 12345, float, e.g. 123.45, existing constant, e.g. x, string, e.g. \"string\", function, e.g. + 1 2",
            ));
        }
        if self.is_lambda(&text) {
            println!("### testing function_def");
            //self.get_function_def(&text);
        } else if self.is_function_call(&text) {
            let (function_name, remainder) = get_until_whitespace_or_eol_or_eof(text.clone());
            let fun = self.get_function(&function_name);
            match fun {
                Some((_, function_format, function_args, function_validation)) => {
                    let mut values: Vec<String> = vec![];
                    let mut val: String; // = text.clone();
                    let (mut remain, _) =
                        get_until_eol_or_eof(strip_leading_whitespace(remainder.clone()));
                    while remain.len() > 0 {
                        (val, remain) =
                            get_until_whitespace_or_eol_or_eof(strip_leading_whitespace(remain));
                        if val != "" {
                            values.push(val.clone());
                        }
                    }

                    // check number of arguments supplied
                    if function_args.len() != values.len() {
                        return Err(self.get_error(
                            3 + identifier.len(),
                            text.len(),
                            &format!(
                                "wrong number of function arguments. Expected: {}. Found {}.",
                                function_args.len(),
                                values.len()
                            ),
                        ));
                    }

                    // check types of values
                    let mut value_types: Vec<String> = vec![];
                    for i in 0..values.len() {
                        value_types.push(self.get_type(&values[i]));
                    }
                    for i in 0..values.len() {
                        if !function_args[i].contains(&value_types[i]) {
                            return Err(self.get_error(
                                3 + identifier.len(),
                                text.len(),
                                &format!(
                                    "function arguments are not the correct types. Expected: {:?}. Found {:?}",
                                    function_args,
                                    value_types
                                ),
                            ));
                        }
                    }

                    // check all types match
                    if values.len() > 0 {
                        // need to check if at least one value otherwise can't determine 'first' below
                        if function_validation.contains(&"arg_types_must_match".to_string()) {
                            let first = &value_types[0];
                            //println!("###first {}", first);
                            if value_types.clone().into_iter().any(|c| {
                                //println!("###c {}", c);
                                return &c != first;
                            }) {
                                return Err(self.get_error(
                                3 + identifier.len(),
                                text.len(),
                                &format!(
                                    "This function's arguments must all be of the same type. Some values have types that appear to differ: {:?}",
                                    value_types
                                ),
                            ));
                            }
                        }
                    }

                    let output = match function_args.len() {
                        2 => {
                            let out1 = function_format.replace("#1", &values[0]);
                            let out2 = out1.replace("#2", &values[1]);
                            out2
                        }
                        1 => {
                            let out1 = function_format.replace("#1", &values[0]);
                            out1
                        }
                        _ => function_format,
                    };
                    return Ok(Some(output));
                }
                _ => {
                    return Err(self.get_error(
                    3 + identifier.len(),
                    text.len(),
                    "is not a valid expression: must be either an: integer, e.g. 12345, float, e.g. 123.45, existing constant, e.g. x, string, e.g. \"string\", function, e.g. + 1 2",
                ));
                }
            }
        }
        Ok(Some(text))
    }

    fn get_type(self: &Self, text: &String) -> String {
        if self.is_integer(text) {
            return "Int".to_string();
        }
        if self.is_float(text) {
            return "Float".to_string();
        }
        if self.is_string(text) {
            return "String".to_string();
        }
        if self.is_function_call(text) {
            return "Function".to_string();
        }
        if self.is_lambda(text) {
            return "FunctionDef".to_string();
        }
        "Undefined".to_string()
    }

    fn is_integer(self: &Self, text: &String) -> bool {
        //println!("is_integer? {}", text);
        let mut is_valid = true;
        if !text.chars().into_iter().all(|c| c.is_numeric()) {
            //println!("iter? {}", text);
            is_valid = false;
        }
        is_valid
    }

    fn is_float(self: &Self, text: &String) -> bool {
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

    fn is_string(self: &Self, text: &String) -> bool {
        //println!("is_string? {}", text);
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
        //println!("is_function_call? {}", text);
        let mut is_valid = true;
        let (function_name, _) = get_until_whitespace_or_eol_or_eof(text.clone());
        if text.len() == 0 || !self.exists_function(&function_name) {
            is_valid = false;
        }
        is_valid
    }

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
                    println!("#{}", remain2);
                    remain2 = remainder.clone();
                    args.push(arg);
                }
                println!("fn'{}' '{}' '{:?}'", slash, remain2, args);
            }
            _ => (),
        };
        //let (slash, remain2) = get_until_whitespace_or_eol_or_eof(strip_leading_whitespace(remain1));
    }

    fn get_error(self: &Self, arrow_indent: usize, arrow_len: usize, error: &str) -> String {
        format!(
            "----------\r\n{}\r\n{}{} {}",
            get_until_eol_or_eof(self.remaining.to_string()).0,
            " ".repeat(arrow_indent),
            "^".repeat(arrow_len),
            error,
        )
    }

    fn check_comment_single_line(self: &mut Self) -> Result<Option<String>, String> {
        if self.remaining.len() < 3 {
            return Err(ERRORS.no_valid_comment_single_line.to_string());
        } else {
            let temp_input = strip_leading_whitespace(self.remaining.clone());
            let (comment, remainder) = get_comment(temp_input)?;

            //println!("remainder {:?}", remainder);
            self.remaining = strip_leading_whitespace(remainder);
            //println!("remainder {:?}", self.remaining);

            let insert = &format!("{}{}\r\n", " ".repeat(self.indent * 4), &comment);
            self.output.insert_str(self.outputcursor, &insert);
            self.outputcursor = self.outputcursor + insert.len();
            let validation_error = None;
            Ok(validation_error)
        }
    }
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
        //println!("empty string?");
        Err(ERRORS.no_valid_identifier_found.to_string())
    } else {
        for i in 0..identifier.len() {
            let c = char_vec[i];
            if i == 0 {
                if !c.is_alphabetic() && !(c == '_') && !(c == '\\' && identifier.len() == 1) {
                    // must start with a letter or underscore (or '\' if function definition)
                    //println!("letter or underscore?");
                    return Err(ERRORS.no_valid_identifier_found.to_string());
                }
            } else {
                if !c.is_alphanumeric() && !(c == '_') {
                    {
                        // all other chars must be letter or number or underscore
                        //println!("alphanumeric?");
                        return Err(ERRORS.no_valid_identifier_found.to_string());
                    }
                }
            }
        }
        Ok((identifier, remainder))
    }
}

fn get_comment(input: String) -> Result<(String, String), String> {
    let temp_input = strip_leading_whitespace(input.clone());
    let (comment, remainder) = get_until_eol_or_eof(temp_input);
    //let char_vec: Vec<char> = comment.chars().collect();
    if comment.len() < 3 || &comment[..2] != "//" {
        Err(ERRORS.no_valid_comment_single_line.to_string())
    } else {
        Ok((comment, remainder))
    }
}

fn get_str(input: String, matchstr: &str) -> Result<String, String> {
    let (identifier, remainder) = get_until_whitespace_or_eof(input.clone());
    if identifier == "".to_string() || &identifier != matchstr {
        //println!("get_str");
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

// assign to variable
// Toy                  Rust
// x = Int 2;           let x: int64 = 2;
// x = Float 3.14;      let x: f64 = 3.14;

// add two integers, assign to variable
// Toy          Rust
// = x + 2 2;   let x = 2 + 2;

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
            constants: vec![],
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
