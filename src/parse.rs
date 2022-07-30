use crate::ast::parents;
use crate::elements;
use crate::elements::{Element, ElementInfo};
use crate::errors;
use crate::errors::ERRORS;
use crate::Compiler;
use crate::Tokens;

pub fn types(compiler: &mut Compiler, index_of_type: usize) -> Result<(), ()> {
    // TODO error checking
    elements::append::types(compiler, index_of_type)
}

pub fn current_line(compiler: &mut Compiler) -> Result<(), ()> {
    let tokens = compiler.lines_of_tokens[compiler.current_line].clone();
    if tokens.len() > 0 {
        while compiler.current_line_token < tokens.len() {
            current_token(compiler, &tokens)?;
            compiler.current_line_token = compiler.current_line_token + 1;
        }
    }
    Ok(())
}

pub fn current_token(compiler: &mut Compiler, tokens: &Tokens) -> Result<(), ()> {
    let current_token = &tokens[compiler.current_line_token];
    let current_token_vec: &Vec<char> = &tokens[compiler.current_line_token].chars().collect();
    if current_token_vec.len() == 0 {
        return Ok(());
    }

    match elements::get_inbuilt_function_index_by_name(&mut compiler.ast, &current_token) {
        Some(index_of_function) => {
            //dbg!(&current_token);
            let func = &compiler.ast.elements[index_of_function];
            match &func.0 {
                ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => {
                    inbuilt_function_call(compiler, &current_token, index_of_function)
                }
                ElementInfo::FunctionDef(_, _, _, _) => {
                    function_call(compiler, &current_token, index_of_function)
                }
                ElementInfo::Arg(_, _, returntype) => {
                    if returntype.contains("&dyn Fn") {
                        function_call(compiler, &current_token, index_of_function)
                    } else {
                        token_by_first_chars(compiler, &current_token, &current_token_vec)
                    }
                }
                _ => token_by_first_chars(compiler, &current_token, &current_token_vec),
            }
        }
        _ => match elements::get_inbuilt_type_index_by_name(&mut compiler.ast, &current_token) {
            Some(index_of_type) => types(compiler, index_of_type),
            _ => token_by_first_chars(compiler, &current_token, &current_token_vec),
        },
    }
}

pub fn token_by_first_chars(
    compiler: &mut Compiler,
    current_token: &String,
    current_token_vec: &Vec<char>,
) -> Result<(), ()> {
    let first_char = current_token_vec[0];
    let second_char = if current_token_vec.len() > 1 {
        Some(current_token_vec[1])
    } else {
        None
    };
    match first_char {
        '\\' => function_definition_start(compiler),
        ':' => function_definition_end(compiler),
        '(' => functiontypesig_or_functionreference_start(compiler),
        ')' => functiontypesig_or_functionreference_end(compiler),
        '/' => comment_single_line(compiler, current_token_vec),
        '=' => {
            if current_token_vec.len() > 1 {
                return errors::append_error(compiler, 0, 1, ERRORS.assign);
            }
            assignment(compiler)
        }
        '"' => string(compiler, &current_token),
        //positive numbers
        first_char if is_integer(&first_char.to_string()) => {
            if is_float(&current_token) {
                float(compiler, &current_token)
            } else {
                int(compiler, &current_token)
            }
        }
        //negative numbers
        '-' => match second_char {
            Some(_digit) => {
                if is_float(&current_token) {
                    float(compiler, &current_token)
                } else {
                    int(compiler, &current_token)
                }
            }
            None => {
                return errors::append_error(compiler, 0, 1, ERRORS.int_negative);
            }
        },
        first_char if "abcdefghijklmnopqrstuvwxyz".contains(&first_char.to_string()) => {
            //dbg!("constant or constantRef", first_char);
            constant(compiler, &current_token)
        }
        _ => Err(()),
    }
}

pub fn comment_single_line(
    compiler: &mut Compiler,
    current_token_vec: &Vec<char>,
) -> Result<(), ()> {
    if current_token_vec.len() < 2 || current_token_vec[1] != '/' {
        return errors::append_error(compiler, 0, 1, ERRORS.comment_single_line);
    }
    let val = concatenate_vec_strings(&compiler.lines_of_tokens[compiler.current_line]);
    elements::append::comment_single_line(compiler, val)
}

pub fn string(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    if is_string(&current_token) {
        elements::append::string(compiler, current_token)
    } else {
        //dbg!(&self.lines_of_tokens);
        errors::append_error(compiler, 0, 1, ERRORS.string)
    }
}

pub fn int(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    //dbg!("parse_int - positive only for now");
    let all_chars_are_numeric = current_token.chars().into_iter().all(|c| c.is_numeric());
    let chars: Vec<char> = current_token.chars().collect();
    let first_char_is_negative_sign = chars[0] == '-';
    let is_negative_all_other_chars_are_not_numeric = first_char_is_negative_sign
        && chars.len() > 1
        && !chars[1..chars.len()].into_iter().all(|c| c.is_numeric());

    if (!first_char_is_negative_sign && !all_chars_are_numeric)
        || is_negative_all_other_chars_are_not_numeric
    {
        errors::append_error(compiler, 0, 1, ERRORS.int)?;
    }
    match current_token.parse::<i64>() {
        Ok(_) => (),
        Err(_) => errors::append_error(compiler, 0, 1, ERRORS.int_out_of_bounds)?,
    }
    elements::append::int(compiler, current_token)
    //errors::error_if_parent_is_invalid(compiler)
}

pub fn float(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    if current_token.len() > 0 && is_float(current_token) {
        elements::append::float(compiler, current_token)
    } else {
        return errors::append_error(compiler, 0, 1, ERRORS.float);
    }
}

pub fn constant(compiler: &mut Compiler, current_token: &String) -> Result<(), ()> {
    //dbg!(current_token);
    let el_option = elements::get_element_by_name(&compiler.ast, current_token);
    match el_option {
        Some(_) => {
            if elements::is_existing_constant(compiler) {
                return errors::append_error(compiler, 0, 1, ERRORS.constants_are_immutable);
            }
            match el_option {
                Some((ElementInfo::Constant(_, returntype), _)) => {
                    return elements::append::constant_ref(compiler, current_token, &returntype);
                }
                Some((ElementInfo::Arg(_, _, returntype), _)) => {
                    //dbg!("Arg", &returntype);
                    if returntype.contains("&dyn Fn") {
                        let args = get_args_from_dyn_fn(&returntype);
                        return elements::append::function_call(
                            compiler,
                            current_token,
                            args,
                            &returntype,
                            false,
                        );
                    } else {
                        return elements::append::constant_ref(
                            compiler,
                            current_token,
                            &returntype,
                        );
                    }
                }
                Some((ElementInfo::FunctionDef(_, argnames, _, returntype), _)) => {
                    return elements::append::function_ref_or_call(
                        compiler,
                        current_token,
                        argnames.len(),
                        &returntype,
                    );
                }
                // explicitly listing other types rather than using _ to not overlook new types in future
                Some((ElementInfo::Root, _)) => (),
                Some((ElementInfo::CommentSingleLine(_), _)) => (),
                Some((ElementInfo::Int(_), _)) => (),
                Some((ElementInfo::Float(_), _)) => (),
                Some((ElementInfo::String(_), _)) => (),
                Some((ElementInfo::ConstantRef(_, _, _), _)) => (),
                Some((ElementInfo::Assignment, _)) => (),
                Some((ElementInfo::InbuiltFunctionDef(_, _, _, _, _), _)) => (),
                Some((ElementInfo::InbuiltFunctionCall(_, _, _), _)) => (),
                Some((ElementInfo::FunctionDefWIP, _)) => (),
                Some((ElementInfo::FunctionCall(_, _), _)) => (),
                Some((ElementInfo::Parens, _)) => (),
                Some((ElementInfo::Type(_), _)) => (),
                Some((ElementInfo::Eol, _)) => (),
                Some((ElementInfo::Seol, _)) => (),
                Some((ElementInfo::Indent, _)) => (),
                Some((ElementInfo::Unused, _)) => (),
                None => (),
            }
        }
        None => (),
    }
    return elements::append::new_constant_or_arg(compiler, current_token);
}

pub fn assignment(compiler: &mut Compiler) -> Result<(), ()> {
    // TODO error checking
    elements::append::assignment(compiler)
}

pub fn inbuilt_function_call(
    compiler: &mut Compiler,
    current_token: &String,
    index_of_function: usize,
) -> Result<(), ()> {
    //TODO error checking
    elements::append::inbuilt_function_call(compiler, current_token, index_of_function)
}

pub fn function_call(
    compiler: &mut Compiler,
    current_token: &String,
    index_of_function: usize,
) -> Result<(), ()> {
    elements::append::function_call1(compiler, current_token, index_of_function)
}

pub fn function_definition_start(compiler: &mut Compiler) -> Result<(), ()> {
    elements::append::function_definition_start(compiler)
}

pub fn function_definition_end(compiler: &mut Compiler) -> Result<(), ()> {
    /*
    At the point you parse a function definition end,
    and because we don't look ahead when parsing,
    the Ast thinks this is what has been parsed
    ...
    10: Assignment: (Undefined) [ 11, 12, ]
    11: Constant: a (Undefined) [ ]
    12: FunctionDef: ("", [], [], Undefined) [ 13, 14, 15, ]
    13: Type: i64 [ ]
    14: Type: i64 [ ]
    15: Constant: arg1 (Undefined) [ ]

    We need to change this to, e.g. this for a single line function...
    10: Unused
    11: Unused
    12: FunctionDef(name, argtypes, argnames, returntype): [ ] (<-ready to accept 16 return statement)
    13: Unused
    14: Unused
    15: Unused
    ... ready to insert next element 16 which is the return statement
    */

    //get parent funcdef
    if let Some(func_def_ref) = parents::get_current_parent_ref_from_element_children_search(
        &compiler.ast,
        compiler.ast.elements.len() - 1,
    ) {
        //get child refs
        let func_def: Element = compiler.ast.elements[func_def_ref].clone();
        let children = func_def.1.clone();
        //dbg!(&children);
        //error if count is NOT odd (argtypes + returntype + argnames)
        if children.len() % 2 == 0 || children.len() == 0 {
            return errors::append_error(compiler, 0, 1, ERRORS.funcdef_args);
        }

        //TODO deal with brackets later (i.e. for type signature containing argument(s) which are fns)

        //error if arg types are NOT first
        let first_child_ref = children[0];

        let first_child: Element = compiler.ast.elements[first_child_ref].clone();
        match first_child.0 {
            ElementInfo::Type(_) => (),
            ElementInfo::Parens => (),
            _ => return errors::append_error(compiler, 0, 1, ERRORS.funcdef_argtypes_first),
        }

        match func_def.0 {
            ElementInfo::FunctionDefWIP => {
                //Constant is parent of functionDefWIP
                if let Some(constant_ref) =
                    parents::get_current_parent_ref_from_element_children_search(
                        &compiler.ast,
                        func_def_ref,
                    )
                {
                    let constant = compiler.ast.elements[constant_ref].clone();

                    //assignment is parent of constant
                    if let Some(assignment_ref) =
                        parents::get_current_parent_ref_from_element_children_search(
                            &compiler.ast,
                            constant_ref,
                        )
                    {
                        match constant.0 {
                            ElementInfo::Constant(name, _) => {
                                elements::replace_funcdefwip_with_funcdef(
                                    compiler,
                                    &children,
                                    &name,
                                    func_def_ref,
                                );

                                // replace assignment with unused
                                compiler.ast.elements[assignment_ref] =
                                    (ElementInfo::Unused, vec![]);

                                // replace constant with Unused
                                compiler.ast.elements[constant_ref] = (ElementInfo::Unused, vec![]);

                                // replace parents child reference to the assignment, with the func_def_ref
                                if let Some(index) =
                                    parents::get_current_parent_ref_from_element_children_search(
                                        &compiler.ast,
                                        assignment_ref,
                                    )
                                {
                                    elements::replace_element_child(
                                        &mut compiler.ast,
                                        index,
                                        assignment_ref,
                                        func_def_ref,
                                    );
                                }

                                //re-add the new funcdef as latest parent, so we can continue parsing with it's child statements
                                parents::outdent::outdent(compiler);
                                parents::outdent::outdent(compiler);
                                parents::outdent::outdent(compiler);
                                parents::indent::indent_this(&mut compiler.ast, func_def_ref);
                                //dbg!(&self.ast.parents);
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => (),
        }
    }
    elements::append::outdent_if_last_expected_child(compiler);
    Ok(())
}

//TODO remember to error / or at least check if reusing arg names in nested functions

pub fn functiontypesig_or_functionreference_start(compiler: &mut Compiler) -> Result<(), ()> {
    elements::append::functiontypesig_or_functionreference_start(compiler)
}

pub fn functiontypesig_or_functionreference_end(compiler: &mut Compiler) -> Result<(), ()> {
    elements::append::functiontypesig_or_functionreference_end(compiler)
}

pub fn is_integer(text: &String) -> bool {
    let mut is_valid = true;
    let all_chars_are_numeric = text.chars().into_iter().all(|c| c.is_numeric());
    let text_chars: Vec<char> = text.chars().collect();
    let first_char_is_negative_sign = text_chars[0] == '-';

    let is_negative_all_other_chars_are_numeric = first_char_is_negative_sign
        && text_chars.len() > 1
        && text_chars[1..text_chars.len()]
            .into_iter()
            .all(|c| c.is_numeric());

    if !all_chars_are_numeric && !is_negative_all_other_chars_are_numeric {
        is_valid = false;
    }

    match text.parse::<i64>() {
        Ok(_) => (),
        Err(_) => is_valid = false,
    }
    is_valid
}

pub fn is_float(text: &String) -> bool {
    let mut is_valid = true;
    let mut index_decimal_point = 0;
    let mut index_e = 0;
    let mut index_plus = 0;
    let char_vec: Vec<char> = text.chars().collect();
    for i in 0..text.len() {
        let c = char_vec[i];
        if c == '.' && index_decimal_point == 0 {
            index_decimal_point = i;
        } else if c == 'E' && index_e == 0 {
            index_e = i;
        } else if c == '+' && index_plus == 0 {
            index_plus = i;
        } else if !c.is_numeric() && !(i == 0 && c == '-') {
            is_valid = false;
        }
    }
    let has_one_decimal_point = index_decimal_point != 0;
    let no_power_of_10 = index_e == 0 && index_plus == 0;
    let has_one_power_of_10 = index_e != 0
        && index_plus > 0
        && index_plus == index_e + 1
        && (char_vec.len() > 1 && index_plus < char_vec.len() - 1)
        && index_e > 0;
    //println!("{} {:?}", text, text.parse::<f64>());
    match text.parse::<f64>() {
        Ok(val) => {
            if val == f64::INFINITY || val == f64::NEG_INFINITY {
                is_valid = false;
            }
        }
        Err(_) => is_valid = false,
    }
    is_valid && has_one_decimal_point && (no_power_of_10 || has_one_power_of_10)
}

pub fn is_string(text: &String) -> bool {
    let mut is_valid = true;
    let char_vec: Vec<char> = text.chars().collect();
    if text.len() < 2 || char_vec[0] != '"' || char_vec[text.len() - 1] != '"' {
        is_valid = false;
    }
    is_valid
}

pub fn get_args_from_dyn_fn(string: &String) -> usize {
    string.matches(",").count() + (!string.contains("()") as usize)
    //0 args, e.g. "&dyn Fn() -> i64"         = 0 commas + 0 does match ()
    //1 args, e.g. "&dyn Fn(i64) -> i64"      = 0 commas + 1 does not match ()
    //2 args, e.g. "&dyn Fn(i64, i64) -> i64" = 1 comma  + 1 does not match ()
}

pub fn concatenate_vec_strings(tokens: &Tokens) -> String {
    let mut output = "".to_string();
    for i in 0..tokens.len() {
        output = format!("{}{}", output, tokens[i]);
    }
    output
}

pub fn strip_leading_whitespace(input: &String) -> String {
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

pub fn strip_trailing_whitespace(input: &String) -> String {
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

#[allow(dead_code)] 
pub const TEST_CASE_PASSES: [[&str; 2]; 61] = [
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
    ["= a 123\r\na", "fn main() {\r\n    let a: i64 = 123;\r\n    a;\r\n}\r\n"],
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