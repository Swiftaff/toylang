/*!
    ## Toylang
    A toy language which compiles to rust for fun and experimentation (Windows only for now)

    Private functions are provided for information, the main info is here:
    - [Github](https://github.com/Swiftaff/toylang) or the [compiler_runner] for general usage instructions
    - [debug_window_derive] for an extra debug windows app
    - [integration_tests][crate::integration_tests::main] for examples
*/

// TODO make most function arguments refs
mod ast;
pub mod compiler_runner;
pub mod debug_window_derive;
mod errors;
//mod example_tests;
mod file;
mod formatting;
mod integration_tests;
mod parse;
use ast::elements;
use ast::output;
use ast::Ast;
use file::File;
use std::error::Error;
use std::fmt;

pub type Tokens = Vec<String>;
type ErrorStack = Vec<String>;
type LinesOfChars = Vec<Vec<char>>;
type LinesOfTokens = Vec<Tokens>;
type Logs = Vec<String>;

fn rem_first_and_last(value: &str) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

pub struct DebugErrorStack<'a>(&'a ErrorStack);

impl<'a> fmt::Debug for DebugErrorStack<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = "".to_string();
        for el in 0..self.0.len() {
            let spaces = left_pad(self.0.len(), el);
            debug = format!(
                "{}\r\n  {}{}: {},",
                debug,
                spaces,
                el,
                rem_first_and_last(&self.0[el])
            );
        }
        write!(f, "Custom Debug of ErrorStack [{}\r\n]", debug)
    }
}

pub struct DebugLogs<'a>(&'a ErrorStack);

impl<'a> fmt::Debug for DebugLogs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = "".to_string();
        for el in 0..self.0.len() {
            let spaces = left_pad(self.0.len(), el);
            debug = format!("{}\r\n  {}{}: {},", debug, spaces, el, &self.0[el]);
        }
        write!(f, "Custom Debug of Logs\r\n{}", debug)
    }
}

fn left_pad(total: usize, index: usize) -> String {
    let digits = num_digits(total);
    let num = num_digits(index);
    " ".repeat(digits - num)
}

fn num_digits(num: usize) -> usize {
    num.to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect::<Vec<u32>>()
        .len()
}

pub struct DebugLinesOfChars<'a>(&'a LinesOfChars);

impl<'a> fmt::Debug for DebugLinesOfChars<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = "".to_string();

        for el in 0..self.0.len() {
            let spaces = left_pad(self.0.len(), el);
            let el_debug = format!("{:?}", self.0[el]);
            debug = format!("{}\r\n  {}{}: {},", debug, spaces, el, el_debug);
        }
        write!(f, "Custom Debug of LinesOfChars [{}\r\n]", debug)
    }
}

pub struct DebugLinesOfTokens<'a>(&'a LinesOfTokens);

impl<'a> fmt::Debug for DebugLinesOfTokens<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = "".to_string();
        for el in 0..self.0.len() {
            let spaces = left_pad(self.0.len(), el);
            let el_debug = format!("{:?}", self.0[el]);
            debug = format!("{}\r\n  {}{}: {},", debug, spaces, el, el_debug);
        }
        write!(f, "Custom Debug of LinesOfTokens [{}\r\n]", debug)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Compiler {
    pub file: File,
    pub debug: bool,
    pub debug_step: usize,
    pub debug_line: usize,
    pub debug_compiler_history: Vec<String>,
    pub filepath: String,
    pub outputdir: String,
    pub lines_of_chars: LinesOfChars,
    pub lines_of_tokens: Vec<Tokens>,
    pub output: String,
    pub current_line: usize,
    pub current_line_token: usize,
    pub error_stack: ErrorStack,
    pub ast: Ast,
    pub logs: Logs,
}

impl Compiler {
    pub fn new(
        filepath: String,
        debug: bool,
        option_outputdir: Option<String>,
        nosave: bool,
    ) -> Result<Compiler, String> {
        println!("\r\nOUTPUT: {:?}", &option_outputdir);
        if debug {
            println!("DEBUG:  true");
            //let ui = debug_window::run();
            //println!("1");
            //ui.win_title("testy2");
        }
        let debug_step = 0;
        let debug_line = 0 as usize;
        let debug_compiler_history = vec![];
        //println!("2");
        let mut outputdir = "".to_string();
        if let Some(outputdir_found) = &option_outputdir {
            outputdir = outputdir_found.to_owned();
        }
        let file = File::new(nosave);
        let lines_of_chars = vec![];
        let lines_of_tokens = vec![];
        let output = "".to_string();
        let current_line = 0;
        let current_line_token = 0;
        let error_stack = vec![];
        let ast = Ast::new();
        let logs = vec![format!(
            "lib::new {:?} {:?} {:?}",
            filepath, debug, option_outputdir
        )];
        Ok(Compiler {
            file,
            debug,
            debug_step,
            debug_line,
            debug_compiler_history,
            filepath,
            outputdir,
            lines_of_chars,
            lines_of_tokens,
            output,
            current_line,
            current_line_token,
            error_stack,
            ast,
            logs,
        })
    }

    pub fn run(self: &mut Self) -> Result<(), Box<dyn Error>> {
        self.log(format!("lib::run {:?}", ""));
        self.file.get(&self.filepath)?;
        match self.run_main_tasks() {
            Ok(_) => (),
            Err(_e) => (),
        }
        self.file.writefile_or_error(
            &self.ast.output,
            &self.outputdir,
            self.error_stack.len() > 0,
        )
    }

    pub fn log(self: &mut Self, string: String) {
        self.logs.push(string);
        self.debug_compiler_history.push(format!("{:?}", self.ast));
    }

    pub fn debug_step(self: &mut Self, step: usize) -> usize {
        self.log(format!("lib::debug_step {:?}", step));
        let mut completed_step: usize = 0;
        self.debug_step = step;

        if self.debug_step == 1 as usize {
            dbg!("1");
            let _result = self.file.get(&self.filepath);
        }

        if self.debug_step == 2 as usize {
            println!("2");
            self.set_lines_of_chars();
        }

        if self.debug_step == 3 as usize {
            println!("3");
            self.set_lines_of_tokens();
        }

        if self.debug_step == 4 as usize {
            if self.debug_line < self.lines_of_tokens.len() {
                let _result = self.main_loop_over_lines_of_tokens();
                self.debug_line = self.debug_line + 1;
                println!("4");
            }
        }

        if self.debug_step == 5 as usize {
            self.ast.output = "".to_string();
            println!("5");
            output::set_output(self);
        }

        if self.file.filepath != "".to_string() {
            completed_step = 1;
        }

        if self.lines_of_chars.len() > 0 {
            completed_step = 2;
        }

        if self.lines_of_tokens.len() > 0 {
            completed_step = 3;
        }

        if self.debug_line > 0 && self.debug_line == self.lines_of_tokens.len() {
            completed_step = 4;
        }

        if self.ast.output.len() > 0 {
            completed_step = 5;
        }

        completed_step

        /*
        if self.debug_step == "writefile_or_error".to_string() {
            println!("2. writefile_or_error");
            match self.file.writefile_or_error(
                &self.ast.output,
                &self.outputdir,
                self.error_stack.len() > 0,
            ) {
                Ok(_) => (),
                Err(_e) => (),
            };
        }
        */
    }

    pub fn run_main_tasks(self: &mut Self) -> Result<(), ()> {
        self.log(format!("lib::run_main_tasks {:?}", ""));
        self.set_lines_of_chars();
        self.set_lines_of_tokens();
        self.run_main_loop()
    }

    fn run_main_loop(self: &mut Self) -> Result<(), ()> {
        self.log(format!("lib::run_main_loop {:?}", ""));
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
                    println!("\r\nToylang compiled successfully:\r\n----------\r\n");
                    if self.debug {
                        println!("{:?}\r\n----------\r\n", self.ast);
                    }
                }
            }
            Err(_e) => {
                eprintln!("{:?}", &self.ast);
                eprintln!("----------\r\n\r\nTOYLANG COMPILE ERROR:");
                //println!("{:?}", e);
                for error in &self.error_stack {
                    println!("{}", error);
                }
                eprintln!("----------\r\n");
            }
        };
        Ok(())
    }

    fn main_loop_over_lines_of_tokens(self: &mut Self) -> Result<(), ()> {
        self.log(format!("lib::main_loop_over_lines_of_tokens {:?}", ""));
        //self.set_ast_output_for_main_fn_start();
        if self.debug {
            let line = self.debug_line;
            self.parse_one_line(line)?;
        } else {
            for line in 0..self.lines_of_tokens.len() {
                self.parse_one_line(line)?;
            }
        }
        Ok(())
    }

    fn parse_one_line(self: &mut Self, line: usize) -> Result<(), ()> {
        self.log(format!("lib::parse_one_line {:?}", line));
        if line < self.lines_of_tokens.len() && self.lines_of_tokens[line].len() > 0 {
            self.current_line = line;
            self.current_line_token = 0;
            parse::current_line(self)?;
        }
        Ok(())
    }

    fn set_lines_of_chars(self: &mut Self) {
        self.log(format!("lib::set_lines_of_chars {:?}", ""));
        let mut index_from = 0;
        let mut index_to = 0;
        let char_vec: Vec<char> = self.file.filecontents.chars().collect();
        while index_to < char_vec.len() {
            let c = char_vec[index_to];
            let d = if index_to + 1 < char_vec.len() {
                char_vec[index_to + 1]
            } else {
                ' '
            };
            let incr = if index_to + 1 < char_vec.len()
                && ((c == '\r' && char_vec[index_to + 1] == '\n') || c == '=' && d == '>')
            {
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
            let is_colon_for_singlelinefunction = c == '=' && d == '>' && !is_a_comment_line;
            if c == '\r' || c == '\n' || eof || is_colon_for_singlelinefunction {
                self.lines_of_chars.push(
                    char_vec[index_from
                        ..index_to
                            + (if is_colon_for_singlelinefunction {
                                2
                            } else if eof {
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
        self.log(format!("lib::set_lines_of_tokens {:?}", ""));
        for line in 0..self.lines_of_chars.len() {
            let mut index_from = 0;
            let mut index_to = 0;
            let mut count_quotes = 0;

            let char_vec_initial: &Vec<char> = &self.lines_of_chars[line];
            let char_as_string = char_vec_initial.iter().collect::<String>();
            let removed_leading_whitespace = parse::strip_leading_whitespace(&char_as_string);
            let removed_trailing_whitespace =
                parse::strip_trailing_whitespace(&removed_leading_whitespace);
            let char_vec: Vec<char> = removed_trailing_whitespace.chars().collect();

            let mut inside_quotes = false;
            let mut line_of_tokens: Tokens = vec![];
            while index_to < char_vec.len() {
                let c = char_vec[index_to];
                let eof = index_to == char_vec.len() - 1;
                if c == '"' {
                    if inside_quotes {
                        inside_quotes = false;
                        count_quotes = count_quotes - 1;
                    } else {
                        inside_quotes = true;
                        count_quotes = count_quotes + 1;
                    }
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
    use ast::elements::{Element, ElementInfo};
    use ast::parents;

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
        elements::append::append(&mut ast, el1);
        elements::append::append(&mut ast, el2);
        elements::append::append(&mut ast, el3);
        parents::indent::indent(&mut ast);
        elements::append::append(&mut ast, el4);
        elements::append::append(&mut ast, el5);
        parents::indent::indent(&mut ast);
        elements::append::append(&mut ast, el6);
        elements::append::append(&mut ast, el7);
        parents::indent::indent(&mut ast);
        parents::indent::indent(&mut ast);
        elements::append::append(&mut ast, el8);
        assert!(true);
    }
}
