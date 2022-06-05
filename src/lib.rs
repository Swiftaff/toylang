use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub filecontents: String,
    pub remaining: String,
    pub output: String,
    pub outputcursor: usize,
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
        Ok(Config {
            filename,
            filecontents,
            remaining,
            output,
            outputcursor,
        })
    }

    pub fn run(self: &mut Self) -> Result<(), Box<dyn Error>> {
        self.filecontents = fs::read_to_string(&self.filename)?;
        println!(
            "1. Reading contents of filename: {:?}\n----------\n{}",
            &self.filename, &self.filecontents
        );
        self.tokenizer()?;
        Ok(())
    }

    fn tokenizer(self: &mut Self) -> Result<(), &str> {
        //ref: https://doc.rust-lang.org/reference/tokens.html
        //let mut current = 0;
        //let mut tokens: Vec<char> = vec![];
        let mut input: String = self.filecontents.clone();
        self.remaining = self.filecontents.clone();
        let mut variable: String = "".to_string();
        while input.len() > 0 {
            self.check_program_syntax()?;
            println!("{:?} {:?}\n", self.output, self.outputcursor);
            //input = check_for strings (because they might have spaces)
            (variable, input) = check_variable_assignment(input)?;
            //let char = input.chars().nth(current).unwrap();
            //println!("{:?}: {:?}\n", current, char);
            //current += 1;
            //tokens.push(char);
        }
        //println!("compiled successfully. Tokens = {:?}\n", tokens);
        Ok(())
        /*
        //looking at this: https://github.com/jamiebuilds/the-super-tiny-compiler/blob/master/the-super-tiny-compiler.js
            while (current < input.length) {
              if (char === '(') {
                tokens.push({
                  type: 'paren',
                  value: '(',
                });
                current++;
                continue;
              }
              if (char === ')') {
                tokens.push({
                  type: 'paren',
                  value: ')',
                });
                current++;
                continue;
              }
              let WHITESPACE = /\s/;
              if (WHITESPACE.test(char)) {
                current++;
                continue;
              }
            let NUMBERS = /[0-9]/;
              if (NUMBERS.test(char)) {
                let value = '';
                while (NUMBERS.test(char)) {
                  value += char;
                  char = input[++current];
                }
                tokens.push({ type: 'number', value });
                continue;
              }
              if (char === '"') {
                let value = '';
                char = input[++current];
                while (char !== '"') {
                  value += char;
                  char = input[++current];
                }
                char = input[++current];
                tokens.push({ type: 'string', value });
                continue;
              }
              let LETTERS = /[a-z]/i;
              if (LETTERS.test(char)) {
                let value = '';
                while (LETTERS.test(char)) {
                  value += char;
                  char = input[++current];
                }
                tokens.push({ type: 'name', value });
                continue;
              }
              throw new TypeError('I dont know what this character is: ' + char);
            }
            */
    }

    fn check_program_syntax<'a>(self: &mut Self) -> Result<(), &'a str> {
        if self.remaining.len() < 8 {
            return Err(ERRORS.invalid_program_syntax);
        } else {
            let starts_with_run = &self.remaining[..5] == "RUN\r\n";
            if !starts_with_run {
                return Err(ERRORS.invalid_program_syntax);
            }
            self.remaining = self.remaining[5..].to_string();
            println!("input = {:?}\n", &self.remaining);

            let ends_with_end = &self.remaining[&self.remaining.len() - 3..] == "END";
            if !ends_with_end {
                return Err(ERRORS.invalid_program_syntax);
            }
            self.remaining = self.remaining[..self.remaining.len() - 3].to_string();
            println!("input = {:?}\n", &self.remaining);
            self.output = "fn main() {\r\n}".to_string();
            self.outputcursor = 13; // anything new will be inserted before end bracket
        }
        Ok(())
    }
}

struct Errors<'a> {
    invalid_program_syntax: &'a str,
    variable_assignment: &'a str,
    no_valid_identifier_found: &'a str,
}

const ERRORS: Errors = Errors {
    invalid_program_syntax: "Invalid program syntax. Must start with RUN, followed by linebreak, optional commands and linebreak, and end with END",
    variable_assignment: "Invalid variable assignment. Must contain Int or Float, e.g. x = Int 2",
    no_valid_identifier_found:"No valid identifier found"
};

fn check_program_syntax<'a>(input_string: String) -> Result<String, &'a str> {
    let mut input = input_string.clone();
    if input.len() < 8 {
        return Err(ERRORS.invalid_program_syntax);
    } else {
        let starts_with_run = &input[..5] == "RUN\r\n";
        if !starts_with_run {
            return Err(ERRORS.invalid_program_syntax);
        }
        input = input[5..].to_string();
        println!("input = {:?}\n", &input);

        let ends_with_end = &input[input.len() - 3..] == "END";
        if !ends_with_end {
            return Err(ERRORS.invalid_program_syntax);
        }
        input = input[..input.len() - 3].to_string();
        println!("input = {:?}\n", &input);
    }
    Ok(input)
}

fn check_variable_assignment<'a>(input_string: String) -> Result<(String, String), &'a str> {
    let mut input = input_string.clone();
    if input.len() < 3 {
        return Err(ERRORS.variable_assignment);
    } else {
        // TODO - return more errors throughout, fix tests and add new function to optionally 'try' various options and ignore errors instead

        let temp_input = strip_leading_whitespace(input_string.clone());
        let identifier = get_identifier(temp_input)?;
        Ok(identifier)
        /*
                //println!("**{:?}", input2);
        let starts_with_x = &input[..4] == "x = ";
        if !starts_with_x {
            println!("no");
            return Ok(input_string);
        }
        println!("yes x = ");
        input = input[4..].to_string();
        println!("next {:?}", &input);
        if input.len() > 3 {
            let has_type_int = &input[..3] == "Int";
            if has_type_int {
                println!("yes Int");
                input = input[3..].to_string();
            } else {
                let has_type_float = input.len() > 5 && &input[..5] == "Float";
                if !has_type_float {
                    println!("no");
                    return Err(ERRORS.variable_assignment);
                }
                println!("yes Float");
                input = input[5..].to_string();
            }
        } else {
            return Err(ERRORS.variable_assignment);
        }*/
    }
}

fn get_identifier<'a>(input: String) -> Result<(String, String), &'a str> {
    let (identifier, remainder) = get_until_whitespace_or_eof(input.clone());
    let char_vec: Vec<char> = identifier.chars().collect();
    if identifier == "".to_string() {
        Err(ERRORS.no_valid_identifier_found)
    } else {
        for i in 0..identifier.len() {
            let c = char_vec[i];
            if i == 0 {
                if !c.is_alphabetic() && !(c == '_') {
                    // must start with a letter or underscore
                    return Err(ERRORS.no_valid_identifier_found);
                }
            } else {
                if !c.is_alphanumeric() && !(c == '_') {
                    {
                        // all other chars must be letter or number or underscore
                        return Err(ERRORS.no_valid_identifier_found);
                    }
                }
            }
        }
        Ok((identifier, remainder))
    }
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
    input[first_non_whitespace_index..].to_string()
}

// assign to variable
// Lang                 Rust
// x = Int 2;           let x: int64 = 2;
// x = Float 3.14;      let x: f64 = 3.14;

// add two integers, assign to variable
// Lang         Rust
// = x + 2 2;   let x = 2 + 2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let args = ["rustlang".to_string(), "filename_example".to_string()];
        let config_result = Config::new(&args);
        let filename = "filename_example".to_string();
        match config_result {
            Ok(config) => assert_eq!(config.filename, filename),
            Err(_) => assert!(false, "error should not exist"),
        }
    }

    #[test]
    fn test_tokenizer() {
        let args = ["rustlang".to_string(), "filename_example".to_string()];
        let config_result = Config::new(&args);
        match config_result {
            Ok(mut config) => {
                //all of above just mocks a new config for assert scenarios below
                config.filecontents = "RUN\r\ntesty\r\nEND".to_string();
                config.tokenizer();
                assert_eq!(config.output, "fn main() {\r\n}");
                assert_eq!(config.outputcursor, 13);
            }
            Err(_) => assert!(false, "error should not exist"),
        }
    }

    #[test]
    fn test_check_program_syntax() {
        let err = Err(ERRORS.invalid_program_syntax);
        assert_eq!(check_program_syntax("".to_string()), err);
        assert_eq!(check_program_syntax("commands".to_string()), err);
        assert_eq!(check_program_syntax("RUN".to_string()), err);
        assert_eq!(check_program_syntax("RUN\r\ncommands\r\n".to_string()), err);
        assert_eq!(check_program_syntax("END".to_string()), err);
        assert_eq!(check_program_syntax("commands\r\nEND".to_string()), err);
        assert_eq!(check_program_syntax("RUNEND".to_string()), err);
        assert_eq!(check_program_syntax("END\r\nRUN".to_string()), err);
        assert_eq!(check_program_syntax("RUN commands END".to_string()), err);
        assert_eq!(
            check_program_syntax("RUN\r\nEND".to_string()),
            Ok("".to_string())
        );
        assert_eq!(
            check_program_syntax("RUN\r\ncommands\r\nEND".to_string()),
            Ok("commands\r\n".to_string())
        );
        assert_eq!(
            check_program_syntax("RUN\r\ncommands\r\ncommands\r\ncommands\r\nEND".to_string()),
            Ok("commands\r\ncommands\r\ncommands\r\n".to_string())
        );
    }

    #[test]
    fn test_check_variable_assignment() {
        let err: Result<(String, String), &str> = Err(ERRORS.variable_assignment);
        let err2: Result<(String, String), &str> = Err(ERRORS.no_valid_identifier_found);
        assert_eq!(check_variable_assignment("".to_string()), err);
        assert_eq!(check_variable_assignment("2 = x".to_string()), err2);
        //assert_eq!(check_variable_assignment("let x = 2".to_string()), err);
        //assert_eq!(check_variable_assignment("x = 2".to_string()), err);
        //assert_eq!(check_variable_assignment("x = Abc 2".to_string()), err);
        //assert_eq!(check_variable_assignment("x = Boats 2".to_string()), err);
        //assert_eq!(check_variable_assignment("x = Monkey 2".to_string()), err);

        //OK
        assert_eq!(
            check_variable_assignment(" x = 2".to_string()),
            Ok(("x".to_string(), " = 2".to_string()))
        );
        /*
        assert_eq!(
            check_variable_assignment("x = Int 2".to_string()),
            Ok(" 2".to_string())
        );
        assert_eq!(
            check_variable_assignment("x = Float 2.2".to_string()),
            Ok(" 2.2".to_string())
        );
        */
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
}
