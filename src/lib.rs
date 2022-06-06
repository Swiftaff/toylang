use std::error::Error;
use std::fs;

#[derive(Clone, Debug)]
pub struct Config {
    pub filename: String,
    pub filecontents: String,
    pub remaining: String,
    pub output: String,
    pub outputcursor: usize,
    pub pass: usize,
    pub indent: usize,
    pub constants: Vec<String>,
    pub error_stack: Vec<&'static str>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("missing filename argument");
        }
        let filename = args[1].clone();
        let filecontents = "".to_string();
        let remaining = "".to_string();
        let output = "".to_string();
        let outputcursor = 0;
        let pass = 0;
        let indent = 0;
        let constants = vec![];
        let error_stack = vec![];
        Ok(Config {
            filename,
            filecontents,
            remaining,
            output,
            outputcursor,
            pass,
            indent,
            constants,
            error_stack,
        })
    }

    pub fn run(self: &mut Self) -> Result<(), Box<dyn Error>> {
        self.filecontents = fs::read_to_string(&self.filename)?;
        println!(
            "\r\nINPUT contents of filename: {:?}\n----------\n{}",
            &self.filename, &self.filecontents
        );
        self.tokenizer()?;
        if self.error_stack.len() == 0 {
            fs::write("output.rs", &self.output)?;
        }
        Ok(())
    }

    fn tokenizer(self: &mut Self) -> Result<(), &str> {
        //ref: https://doc.rust-lang.org/reference/tokens.html
        self.remaining = self.filecontents.clone();
        match self.main_loop() {
            Ok(()) => {
                println!(
                    "----------\r\n\r\nToylang compiled successfully:\r\n----------\r\n{}\r\n----------\r\n",
                    self.output
                );
            }
            Err(e) => {
                println!(
                    "----------\r\n\r\nTOYLANG COMPILE ERROR:\r\n----------\r\n{:?}\r\n----------\r\n",
                    e
                );
            }
        };
        Ok(())
    }

    fn main_loop(self: &mut Self) -> Result<(), Vec<&str>> {
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

    fn check_one_or_more_succeeds<'a>(self: &mut Self) -> Result<(), Vec<&'a str>> {
        println!("e0:{:?}... r::{:?}", self.remaining, self.error_stack);
        if self.check_one_succeeds("check_variable_assignment") {
            println!("e1:{:?}", self.error_stack);
            return Ok(());
        }
        if self.check_one_succeeds("check_comment_single_line") {
            println!("e2:{:?}", self.error_stack);
            return Ok(());
        }

        let e = ERRORS.no_valid_expression;
        self.error_stack.push(e);
        println!("e3:{:?}", self.error_stack);
        println!("{:?}", self);
        Err(self.error_stack.clone())
    }

    fn check_one_succeeds<'a>(self: &mut Self, function_name: &str) -> bool {
        let mut succeeded = false;
        let mut clone = self.clone();
        let result = match function_name {
            "check_variable_assignment" => clone.check_variable_assignment(),
            "check_comment_single_line" => clone.check_comment_single_line(),
            _ => {
                println!("check_one_succeeds: provided an unknown function_name");
                return false;
            }
        };
        println!("check result {:?} {:?}", function_name, result);
        match result {
            Ok(validation_error) => {
                self.clone_mut_ref(clone);
                match validation_error {
                    Some(e) => {
                        println!("one_succeeds e{:?}", e);
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
        self.output = to_clone.output;
        self.outputcursor = to_clone.outputcursor;
        self.pass = to_clone.pass;
        self.indent = to_clone.indent;
        self.constants = to_clone.constants;
        self.error_stack = to_clone.error_stack;
    }

    fn check_program_syntax<'a>(self: &mut Self) -> Result<(), Vec<&'a str>> {
        if self.pass == 0 {
            if self.remaining.len() < 8 {
                self.error_stack.push(ERRORS.invalid_program_syntax);
                return Err(self.error_stack.clone());
                //return Err();
            } else {
                let starts_with_run = &self.remaining[..5] == "RUN\r\n";
                if !starts_with_run {
                    self.error_stack.push(ERRORS.invalid_program_syntax);
                    return Err(self.error_stack.clone());
                    //return Err(ERRORS.invalid_program_syntax);
                }
                self.remaining = self.remaining[5..].to_string();
                //println!("input = {:?}\n", &self.remaining);

                let ends_with_end = &self.remaining[&self.remaining.len() - 3..] == "END";
                if !ends_with_end {
                    self.error_stack.push(ERRORS.invalid_program_syntax);
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

    fn check_variable_assignment<'a>(self: &mut Self) -> Result<Option<&'a str>, &'a str> {
        if self.remaining.len() < 3 {
            println!("error here?");
            return Err(ERRORS.variable_assignment);
        } else {
            // TODO - return more errors throughout, fix tests and add new function to optionally 'try' various options and ignore errors instead
            let mut remainder = strip_leading_whitespace(self.remaining.clone());
            println!("check_var {:?}", self);
            remainder = get_str(remainder.clone(), "=")?;

            remainder = strip_leading_whitespace(remainder);
            let (identifier, mut remainder) = get_identifier(remainder)?;
            let mut validation_error = None;
            if self.constants.iter().any(|c| c == &identifier) {
                //self.error_stack.push(ERRORS.constants_are_immutable);
                println!("r: {:?}\r\ne:{:?}", self.remaining, self.error_stack);
                validation_error = Some(ERRORS.constants_are_immutable);
            }

            remainder = strip_leading_whitespace(remainder);
            let (text, remain) = get_until_whitespace_or_eof(remainder);

            let insert = &format!(
                "{}let {} = {};\r\n",
                " ".repeat(self.indent * 4),
                &identifier,
                &text
            );
            self.constants.push(identifier);
            self.output.insert_str(self.outputcursor, &insert);
            self.outputcursor = self.outputcursor + insert.len();
            self.remaining = strip_leading_whitespace(remain);
            Ok(validation_error)
        }
    }

    fn check_comment_single_line<'a>(self: &mut Self) -> Result<Option<&'a str>, &'a str> {
        if self.remaining.len() < 3 {
            return Err(ERRORS.no_valid_comment_single_line);
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

struct Errors<'a> {
    invalid_program_syntax: &'a str,
    variable_assignment: &'a str,
    no_valid_identifier_found: &'a str,
    no_valid_comment_single_line: &'a str,
    no_valid_expression: &'a str,
    constants_are_immutable: &'a str,
}

const ERRORS: Errors = Errors {
    invalid_program_syntax: "Invalid program syntax. Must start with RUN, followed by linebreak, optional commands and linebreak, and end with END",
    variable_assignment: "Invalid variable assignment. Must contain Int or Float, e.g. x = Int 2",
    no_valid_identifier_found:"No valid identifier found",
    no_valid_comment_single_line: "No valid single line comment found",
    no_valid_expression: "No valid expression was found",
    constants_are_immutable: "Constants are immutable. You may be trying to assign a value to a constant that has already been defined. Try renaming this as a new constant."
};

fn get_identifier<'a>(input: String) -> Result<(String, String), &'a str> {
    let (identifier, remainder) = get_until_whitespace_or_eof(input.clone());
    let char_vec: Vec<char> = identifier.chars().collect();
    if identifier == "".to_string() {
        println!("empty string?");
        Err(ERRORS.no_valid_identifier_found)
    } else {
        for i in 0..identifier.len() {
            let c = char_vec[i];
            if i == 0 {
                if !c.is_alphabetic() && !(c == '_') {
                    // must start with a letter or underscore
                    println!("letter or underscore?");
                    return Err(ERRORS.no_valid_identifier_found);
                }
            } else {
                if !c.is_alphanumeric() && !(c == '_') {
                    {
                        // all other chars must be letter or number or underscore
                        println!("alphanumeric?");
                        return Err(ERRORS.no_valid_identifier_found);
                    }
                }
            }
        }
        Ok((identifier, remainder))
    }
}

fn get_comment<'a>(input: String) -> Result<(String, String), &'a str> {
    let temp_input = strip_leading_whitespace(input.clone());
    let (comment, remainder) = get_until_eol_or_eof(temp_input);
    //let char_vec: Vec<char> = comment.chars().collect();
    if comment.len() < 3 || &comment[..2] != "//" {
        Err(ERRORS.no_valid_comment_single_line)
    } else {
        Ok((comment, remainder))
    }
}

fn get_str<'a>(input: String, matchstr: &str) -> Result<String, &'a str> {
    let (identifier, remainder) = get_until_whitespace_or_eof(input.clone());
    if identifier == "".to_string() || &identifier != matchstr {
        println!("get_str");
        return Err(ERRORS.no_valid_identifier_found);
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
            output: "".to_string(),
            outputcursor: 0,
            pass: 0,
            indent: 1,
            constants: vec![],
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

    #[test]
    fn test_tokenizer_assignment_immutable_err() {
        let mut config = mock_config("RUN\r\n= x 2\r\n= x 3\r\nEND");
        match config.tokenizer() {
            Ok(_) => assert!(false, "error should not exist"),
            Err(e) => assert_eq!(e, ERRORS.no_valid_expression), // constants_are_immutable
        }
    }

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
        let err: Result<Option<&str>, &str> = Err(ERRORS.variable_assignment);
        let err2: Result<Option<&str>, &str> = Err(ERRORS.no_valid_identifier_found);
        assert_eq!(mock_config("").check_variable_assignment(), err);
        assert_eq!(mock_config("2 = x").check_variable_assignment(), err2);
        assert_eq!(mock_config("let x = 2").check_variable_assignment(), err2);
        //assert_eq!(check_variable_assignment("x = 2".to_string()), err);
        //assert_eq!(check_variable_assignment("x = Abc 2".to_string()), err);
        //assert_eq!(check_variable_assignment("x = Boats 2".to_string()), err);
        //assert_eq!(check_variable_assignment("x = Monkey 2".to_string()), err);

        //OK
        assert_eq!(
            mock_config("= x Int 2").check_variable_assignment(),
            Ok(None)
        );
        assert_eq!(mock_config(" = x 2").check_variable_assignment(), Ok(None));
        assert_eq!(
            mock_config("= x Float 2.2").check_variable_assignment(),
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
        let err = Err(ERRORS.no_valid_identifier_found);
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
        let err = Err(ERRORS.no_valid_identifier_found);
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
