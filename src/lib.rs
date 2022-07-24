// TODO make most function arguments refs
mod ast;
mod formatting;
mod file;
mod errors;
mod parse;
use ast::Ast;
use ast::output;
use ast::elements;
use file::File;
use std::error::Error;

pub type Tokens = Vec<String>;
type ErrorStack = Vec<String>;

#[derive(Clone, Debug)]
pub struct Compiler {
    pub file: File,
    pub lines_of_chars: Vec<Vec<char>>,
    pub lines_of_tokens: Vec<Tokens>,
    pub output: String,
    pub current_line: usize,
    pub current_line_token: usize,
    pub error_stack: ErrorStack,
    pub ast: Ast,
}

impl Compiler {
    pub fn new(args: &[String]) -> Result<Compiler, String> {
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
        Ok(Compiler {
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
                    output::set_output(self);
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
                parse::parse_current_line(self)?;
            }
        }
        Ok(())
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
            let removed_leading_whitespace = parse::strip_leading_whitespace(&char_as_string);
            let removed_trailing_whitespace = parse::strip_trailing_whitespace(&removed_leading_whitespace);
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

}

#[cfg(test)]
mod tests {

    use super::*;
    use ast::elements::{Element,ElementInfo};
    use ast::parents;
    use errors::ERRORS;

    fn mock_compiler() -> Compiler {
        Compiler {
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
            assert!(parse::is_integer(input));
        }
        let test_case_fails = ["1a", "9223372036854775808", "-1a", "-9223372036854775809"];
        for test in test_case_fails {
            let input = &test.to_string();
            assert!(!parse::is_integer(input));
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
            assert!(parse::is_float(input));
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
            assert!(!parse::is_float(input));
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
        elements::append::append(&mut ast,el1);
        elements::append::append(&mut ast,el2);
        elements::append::append(&mut ast,el3);
        parents::indent::indent(&mut ast);
        elements::append::append(&mut ast,el4);
        elements::append::append(&mut ast,el5);
        parents::indent::indent(&mut ast);
        elements::append::append(&mut ast,el6);
        elements::append::append(&mut ast,el7);
        parents::indent::indent(&mut ast);
        parents::indent::indent(&mut ast);
        elements::append::append(&mut ast,el8);
        assert!(true);
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
            let mut c = mock_compiler();
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
            let mut c = mock_compiler();
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

    // cargo watch -x "test"
    // cargo watch -x "test test_run"
    // cargo watch -x "test test_run -- --show-output"
    // cargo watch -x "test test_is_float -- --show-output"

    // cd target/debug
    // cargo build
    // toylang ../../src/test.toy
}
