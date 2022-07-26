use crate::ast::elements;
use crate::ast::elements::{Element, ElementInfo};
use crate::ast::parents;
use crate::Compiler;

#[derive(Clone, Debug)]
pub struct Errors {
    pub comment_single_line: &'static str,
    pub comment_cant_be_child_of_assignment: &'static str,
    pub comment_cant_be_child_of_constant: &'static str,
    pub comment_cant_be_child_of_inbuiltfncall: &'static str,
    pub comment_cant_be_child_of_fncall: &'static str,
    pub comment_cant_be_child_of_parenthesis: &'static str,
    pub int_cant_be_child_of_assignment: &'static str,
    pub int_cant_be_child_of_parenthesis: &'static str,
    pub float_cant_be_child_of_assignment: &'static str,
    pub float_cant_be_child_of_parenthesis: &'static str,
    pub string_cant_be_child_of_assignment: &'static str,
    pub string_cant_be_child_of_parenthesis: &'static str,
    pub string: &'static str,
    pub assign: &'static str,
    pub int: &'static str,
    pub int_out_of_bounds: &'static str,
    pub int_negative: &'static str,
    pub float: &'static str,
    //pub typeerror: &'static str,
    pub funcdef_args: &'static str,
    pub funcdef_argtypes_first: &'static str,
    //pub no_valid_assignment: &'static str,
    //pub no_valid_integer_arithmetic: &'static str,
    //pub no_valid_expression: &'static str,
    pub constants_are_immutable: &'static str,
    pub impossible_error: &'static str,
}

pub const ERRORS: Errors = Errors {
    comment_single_line: "Invalid single line comment: Must begin with two forward slashes '//'",
    comment_cant_be_child_of_assignment: "Invalid Assignment - comment found instead of constant or function definition",
    comment_cant_be_child_of_constant: "Invalid Constant Definition - comment found instead of value",
    comment_cant_be_child_of_inbuiltfncall:"Invalid Inbuilt Function Call - comment found instead of value",
    comment_cant_be_child_of_fncall:"Invalid Function Call - comment found instead of value",
    comment_cant_be_child_of_parenthesis:"Invalid Parenthesis - comment found inside parenthesis",
    int_cant_be_child_of_assignment:"Invalid Assignment - Int found  instead of constant or function definition",
    int_cant_be_child_of_parenthesis:"Invalid parenthesis - Int found inside parenthesis. Can only include a type in a function definition, or a function name as a reference",
    float_cant_be_child_of_assignment:"Invalid Assignment - Float found  instead of constant or function definition",
    float_cant_be_child_of_parenthesis:"Invalid parenthesis - Float found inside parenthesis. Can only include a type in a function definition, or a function name as a reference",
    string_cant_be_child_of_assignment:"Invalid Assignment - Float found  instead of constant or function definition",
    string_cant_be_child_of_parenthesis:"Invalid parenthesis - Float found inside parenthesis. Can only include a type in a function definition, or a function name as a reference",
    string: "Invalid string found: Must be enclosed in quote marks \"\"",
    assign: "Invalid assignment: There are characters directly after '='. It must be followed by a space",
    int: "Invalid int: there are characters after the first digit. Must only contain digits",
    int_out_of_bounds: "Invalid int: is out of bounds. Must be within the value of -9223372036854775808 to 9223372036854775807",
    int_negative:"Invalid negative int or float: Must follow a negative sign '-' with a digit",
    float: "Invalid float",
    //typeerror: "Invalid type",
    funcdef_args: "Invalid Functional Definition - wrong number of argument types: should be 1 type for each arg, plus a return type.",
    funcdef_argtypes_first:"Invalid Functional Definition - argument types should come before argument names.",
    //no_valid_assignment: "No valid assignment found",
    //no_valid_integer_arithmetic: "No valid integer arithmetic found",
    //no_valid_expression: "No valid expression was found",
    constants_are_immutable: "Constants are immutable. You may be trying to assign a value to a constant that has already been defined. Try renaming this as a new constant.",
    impossible_error: "Oh no, this error should be impossible... 'Well here's another nice mess you've gotten me into.'",
    };

pub fn append_error(
    compiler: &mut Compiler,
    mut arrow_indent: usize,
    arrow_len: usize,
    error: &str,
) -> Result<(), ()> {
    if arrow_indent == 0 && compiler.current_line_token != 0 {
        let line_of_tokens = compiler.lines_of_tokens[compiler.current_line].clone();
        arrow_indent = line_of_tokens[0..compiler.current_line_token]
            .iter()
            .cloned()
            .collect::<String>()
            .len()
            + compiler.current_line_token;
    }

    let e = format!(
        "----------\r\n./src/{}:{}:0\r\n{}\r\n{}{} {}",
        compiler.file.filename,
        compiler.current_line + 1,
        compiler.lines_of_chars[compiler.current_line]
            .iter()
            .collect::<String>(),
        " ".repeat(arrow_indent),
        "^".repeat(arrow_len),
        error,
    );
    compiler.error_stack.push(e);
    Err(())
}

pub fn error_if_parent_is_invalid(compiler: &mut Compiler) -> Result<(), ()> {
    let el = elements::get_last_element(&compiler.ast);
    let parent = parents::get_current_parent_element_from_parents(&compiler.ast);
    dbg!("error_if_parent_is_invalid", &el, &parent);
    match el.0 {
        ElementInfo::Root => (),
        ElementInfo::CommentSingleLine(_) => {
            error_if_parent_is_invalid_for_commentsingleline(compiler, &parent)?
        }
        ElementInfo::Int(_) => error_if_parent_is_invalid_for_int(compiler, &parent)?,
        ElementInfo::Float(_) => error_if_parent_is_invalid_for_float(compiler, &parent)?,
        ElementInfo::String(_) => error_if_parent_is_invalid_for_string(compiler, &parent)?,
        // TODO remaining variants
        _ => (),
    }
    Ok(())
}

pub fn error_if_parent_is_invalid_for_commentsingleline(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Root => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::Constant(_, _) => {
            append_error(compiler, 0, 1, ERRORS.comment_cant_be_child_of_constant)
        }
        ElementInfo::Assignment => {
            append_error(compiler, 0, 1, ERRORS.comment_cant_be_child_of_assignment)
        }
        ElementInfo::InbuiltFunctionCall(_, _, _) => append_error(
            compiler,
            0,
            1,
            ERRORS.comment_cant_be_child_of_inbuiltfncall,
        ),
        ElementInfo::FunctionCall(_, _) => {
            append_error(compiler, 0, 1, ERRORS.comment_cant_be_child_of_fncall)
        }
        ElementInfo::Parens => {
            append_error(compiler, 0, 1, ERRORS.comment_cant_be_child_of_parenthesis)
        }
        // explicitly listing other types rather than using _ to not overlook new types in future.
        ElementInfo::CommentSingleLine(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Int(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Float(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::String(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Arg(_, _, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Type(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Eol => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Seol => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Indent => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Unused => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::ConstantRef(_, _, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
    }
}

pub fn error_if_parent_is_invalid_for_int(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Root => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::Assignment => {
            append_error(compiler, 0, 1, ERRORS.int_cant_be_child_of_assignment)
        }
        ElementInfo::Parens => {
            append_error(compiler, 0, 1, ERRORS.int_cant_be_child_of_parenthesis)
        }
        // explicitly listing other types rather than using _ to not overlook new types in future.
        ElementInfo::CommentSingleLine(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Int(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Float(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::String(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Arg(_, _, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Type(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Eol => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Seol => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Indent => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Unused => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::ConstantRef(_, _, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
    }
}

pub fn error_if_parent_is_invalid_for_float(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Root => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::Assignment => {
            append_error(compiler, 0, 1, ERRORS.float_cant_be_child_of_assignment)
        }
        ElementInfo::Parens => {
            append_error(compiler, 0, 1, ERRORS.float_cant_be_child_of_parenthesis)
        }
        // explicitly listing other types rather than using _ to not overlook new types in future.
        ElementInfo::CommentSingleLine(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Int(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Float(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::String(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Arg(_, _, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Type(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Eol => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Seol => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Indent => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Unused => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::ConstantRef(_, _, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
    }
}

pub fn error_if_parent_is_invalid_for_string(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Root => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::Assignment => {
            append_error(compiler, 0, 1, ERRORS.string_cant_be_child_of_assignment)
        }
        ElementInfo::Parens => {
            append_error(compiler, 0, 1, ERRORS.string_cant_be_child_of_parenthesis)
        }
        // explicitly listing other types rather than using _ to not overlook new types in future.
        ElementInfo::CommentSingleLine(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Int(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Float(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::String(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Arg(_, _, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Type(_) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Eol => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Seol => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Indent => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Unused => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::ConstantRef(_, _, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
    }
}

#[allow(dead_code)]
pub const TEST_CASE_ERRORS: [[&str; 2]; 29] = [
    //empty file
    ["", ""],
    //
    //comment single line
    [ERRORS.comment_single_line, "/1/comment"],
    [ERRORS.comment_cant_be_child_of_assignment, "= //test"],
    [ERRORS.comment_cant_be_child_of_constant, "= c //test"],
    [ERRORS.comment_cant_be_child_of_inbuiltfncall, "+ //test"],
    [
        ERRORS.comment_cant_be_child_of_fncall,
        "= myfun \\ i64 i64 arg1 : + arg1 123\r\nmyfun //test",
    ],
    [
        ERRORS.comment_cant_be_child_of_parenthesis,
        "= myfun \\ ( i64 i64 // test ) i64 arg1 : arg1 123",
    ],
    [
        ERRORS.comment_cant_be_child_of_parenthesis,
        "= myfun \\ ( i64 i64 ) i64 arg1 : arg1 123\r\nmyfun ( //test ) 123",
    ],
    [ERRORS.int_cant_be_child_of_assignment, "= 123"],
    [
        ERRORS.int_cant_be_child_of_parenthesis,
        "= myfun \\ ( i64 123 ) i64 arg1 : arg1 123",
    ],
    [
        ERRORS.int_cant_be_child_of_parenthesis,
        "= myfun \\ ( i64 i64 ) i64 arg1 : arg1 123\r\nmyfun ( 123 ) 123",
    ],
    [ERRORS.float_cant_be_child_of_assignment, "= 123.456"],
    [
        ERRORS.float_cant_be_child_of_parenthesis,
        "= myfun \\ ( i64 123.456 ) i64 arg1 : arg1 123",
    ],
    [
        ERRORS.float_cant_be_child_of_parenthesis,
        "= myfun \\ ( i64 i64 ) i64 arg1 : arg1 123\r\nmyfun ( 123.456 ) 123",
    ],
    [ERRORS.string_cant_be_child_of_assignment, "= \"string\""],
    [
        ERRORS.string_cant_be_child_of_parenthesis,
        "= myfun \\ ( i64 \"string\" ) i64 arg1 : arg1 123",
    ],
    [
        ERRORS.string_cant_be_child_of_parenthesis,
        "= myfun \\ ( i64 i64 ) i64 arg1 : arg1 123\r\nmyfun ( \"string\" ) 123",
    ],
    //
    //string
    [ERRORS.string, "\""],
    [ERRORS.string, "\"\"\""],
    [ERRORS.string, "\"\" \""],
    //
    //int
    [ERRORS.int, "1a"],
    [ERRORS.int_out_of_bounds, "9223372036854775808"],
    //
    //int negative
    [ERRORS.int, "-1a"],
    [ERRORS.int_out_of_bounds, "-9223372036854775809"],
    //
    //float (errors say int)
    [ERRORS.int, "1.1.1"],
    [ERRORS.int, "1.7976931348623157E+309"],
    //
    //float negative (errors say int)
    [ERRORS.int, "-1.1.1"],
    [ERRORS.int, "-1.7976931348623157E+309"],
    //
    //internalFunctionCalls
    //[ERRORS.int,"+ 1 2.1"],
    //[ERRORS.int,"- 1.1 2"],
    //
    //functionDefinitions
    //[ERRORS.funcdef_args, "= a \\ :"],
    //[ERRORS.funcdef_argtypes_first,"= a \\ i64 monkey i64  :"],
    //
    //constants are immutable
    [ERRORS.constants_are_immutable, "= a 123\r\n= a 234"],
];
