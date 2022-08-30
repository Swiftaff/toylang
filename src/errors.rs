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
    pub constantref_cant_be_child_of_parenthesis: &'static str,
    pub assignment_cant_be_child_of_constant: &'static str,
    pub assignment_cant_be_child_of_inbuiltfncall: &'static str,
    pub assignment_cant_be_child_of_fncal: &'static str,
    pub assignment_cant_be_child_of_assignment: &'static str,
    pub assignment_cant_be_child_of_parenthesis: &'static str,
    pub assignment_cant_be_child_of_list: &'static str,
    pub inbuiltfncall_cant_be_child_of_parenthesis: &'static str,
    pub fncall_wrong_number_of_args: &'static str,
    pub fncall_cant_be_child_of_parenthesis: &'static str,
    pub parenthesis_cant_be_child_of_root: &'static str,
    pub parenthesis_cant_be_child_of_constant: &'static str,
    pub parenthesis_cant_be_child_of_assignment: &'static str,
    pub println_cant_be_child_of_element: &'static str,
    pub fndefwip_can_only_be_child_of_constant: &'static str,
    pub string: &'static str,
    pub assign: &'static str,
    pub list: &'static str,
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
    pub constant_undefined: &'static str,
    pub loop_for: &'static str,
    pub loopfor_cant_be_child: &'static str,
    pub loopfor_end_but_no_start: &'static str,
    pub loopfor_malformed: &'static str,
    pub list_cant_be_child: &'static str,
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
    constantref_cant_be_child_of_parenthesis:"Invalid Constant Reference - only Types and Function names should be found inside parenthesis",
    constant_undefined:"Invalid Constant Definition - this constant has not previously been defined, so cannot be used anywhere except in a new definition, e.g. = a 123",
    assignment_cant_be_child_of_constant:"Invalid Constant Definition - \"=\" can't be the value of this constant",
    assignment_cant_be_child_of_inbuiltfncall:"Invalid Inbuilt Function Call - \"=\" found instead of value",
    assignment_cant_be_child_of_fncal:"Invalid Function Call - \"=\" found instead of value",
    assignment_cant_be_child_of_assignment: "Invalid Assignment - \"=\" found instead of constant or function definition",
    assignment_cant_be_child_of_parenthesis:"Invalid parenthesis - \"=\" found inside parenthesis. Can only include a type in a function definition, or a function name as a reference",
    assignment_cant_be_child_of_list:"Invalid list - you can't assign a value inside a list",
    inbuiltfncall_cant_be_child_of_parenthesis:"Invalid parenthesis - inbuilt function call found inside parenthesis. Can only include a type in a function definition, or a function name as a reference",
    fncall_wrong_number_of_args:"Invalid function call - wrong number of arguments found",
    fncall_cant_be_child_of_parenthesis:"Invalid parenthesis - function call found inside parenthesis. Can only include a type in a function definition, or a function name as a reference",
    parenthesis_cant_be_child_of_root:"Invalid parenthesis - parenthesis found at start of line. Can only use in a function definition, or a function name as a reference",
    parenthesis_cant_be_child_of_constant:"Invalid parenthesis - parenthesis found as value of constant. Can only use in a function definition, or a function name as a reference",
    parenthesis_cant_be_child_of_assignment:"Invalid parenthesis - parenthesis found as value of assignment. Can only use in a function definition, or a function name as a reference",
    println_cant_be_child_of_element:"Invalid PrintLn - can't be used as child of this element",
    fndefwip_can_only_be_child_of_constant:"Invalid Function Definition - \"\\\" found, which defines start of a function. Can only be used after a constant, i.e. = fn_name \\ i64 i64 arg1 : + arg1 123",
    loopfor_cant_be_child:"Invalid For Loop - can't be placed here",
    loopfor_end_but_no_start:"Invalid End For Loop found - can't find start of for loop",
    loopfor_malformed:"Invalid For Loop - is missing key parts like variable name, start or end of range",
    list_cant_be_child:"Invalid List - can't be placed here",
    string: "Invalid string found: Must be enclosed in quote marks \"\"",
    assign: "Invalid assignment: There are characters directly after '='. It must be followed by a space",
    list: "Invalid list: List must be defined by elements(s) surrounded by [ ] with spaces between. An empty list must contain the type in the list like [ i64 ] . Types can't be values in a list, so they will be removed and the last found type will be assigned to the list returntype.",
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
    loop_for:"Found character after \".\" For loops start with \"..\"",
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
    //dbg!("error_if_parent_is_invalid", &el, &parent);
    match el.0 {
        ElementInfo::Root => (),
        ElementInfo::List(_) => error_if_parent_is_invalid_for_list(compiler, &parent)?,
        ElementInfo::CommentSingleLine(_) => {
            error_if_parent_is_invalid_for_commentsingleline(compiler, &parent)?
        }
        ElementInfo::Int(_) => error_if_parent_is_invalid_for_int(compiler, &parent)?,
        ElementInfo::Float(_) => error_if_parent_is_invalid_for_float(compiler, &parent)?,
        ElementInfo::String(_) => error_if_parent_is_invalid_for_string(compiler, &parent)?,
        ElementInfo::Arg(_, _, _) => error_if_parent_is_invalid_for_arg(compiler, &parent)?,
        ElementInfo::ConstantRef(_, _, _) => {
            error_if_parent_is_invalid_for_constantref(compiler, &parent)?
        }
        ElementInfo::Constant(_, _) => error_if_parent_is_invalid_for_constant(compiler, &parent)?,
        ElementInfo::Assignment => error_if_parent_is_invalid_for_assignment(compiler, &parent)?,
        ElementInfo::InbuiltFunctionCall(_, _, _) => {
            error_if_parent_is_invalid_for_inbuiltfncall(compiler, &parent)?
        }
        ElementInfo::FunctionCall(_, _) => {
            error_if_parent_is_invalid_for_fncall(compiler, &parent)?
        }
        ElementInfo::Parens => error_if_parent_is_invalid_for_parenthesis(compiler, &parent)?,
        ElementInfo::LoopForRangeWIP => error_if_parent_is_invalid_for_loopfor(compiler, &parent)?,
        ElementInfo::LoopForRange(_, _, _) => {
            error_if_parent_is_invalid_for_loopfor(compiler, &parent)?
        }
        // TODO remaining variants
        ElementInfo::Type(_) => (),
        ElementInfo::Eol => (),
        ElementInfo::Seol => (),
        ElementInfo::Indent => (),
        ElementInfo::Unused => (),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => (),
        ElementInfo::FunctionDefWIP => error_if_parent_is_invalid_for_fndefwip(compiler, &parent)?,
        ElementInfo::FunctionDef(_, _, _, _) => (),
        ElementInfo::Println => error_if_parent_is_invalid_for_println(compiler, &parent)?,
        ElementInfo::If(_) => error_if_parent_is_invalid_for_if_expression(compiler, &parent)?,
    }
    Ok(())
}

pub fn error_if_parent_is_invalid_for_list(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Root => Ok(()),
        ElementInfo::List(_) => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::Assignment => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::If(_) => Ok(()),
        ElementInfo::Parens => append_error(compiler, 0, 1, ERRORS.list_cant_be_child),
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
        ElementInfo::Println => append_error(compiler, 0, 1, ERRORS.impossible_error),
    }
}

pub fn error_if_parent_is_invalid_for_commentsingleline(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Root => Ok(()),
        ElementInfo::List(_) => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        ElementInfo::If(_) => Ok(()),
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
        ElementInfo::Println => append_error(compiler, 0, 1, ERRORS.impossible_error),
    }
}

pub fn error_if_parent_is_invalid_for_int(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Root => Ok(()),
        ElementInfo::List(_) => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        ElementInfo::Println => Ok(()),
        ElementInfo::If(_) => Ok(()),
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
        ElementInfo::List(_) => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        ElementInfo::Println => Ok(()),
        ElementInfo::If(_) => Ok(()),
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
        ElementInfo::List(_) => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        ElementInfo::Println => Ok(()),
        ElementInfo::If(_) => Ok(()),
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

pub fn error_if_parent_is_invalid_for_arg(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::List(_) => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::If(_) => Ok(()),
        // explicitly listing other types rather than using _ to not overlook new types in future.
        ElementInfo::Root => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Constant(_, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::InbuiltFunctionCall(_, _, _) => {
            append_error(compiler, 0, 1, ERRORS.impossible_error)
        }
        ElementInfo::FunctionCall(_, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => {
            append_error(compiler, 0, 1, ERRORS.impossible_error)
        }
        ElementInfo::Assignment => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::LoopForRangeWIP => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::LoopForRange(_, _, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::Parens => append_error(compiler, 0, 1, ERRORS.impossible_error),
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
        ElementInfo::Println => append_error(compiler, 0, 1, ERRORS.impossible_error),
    }
}

pub fn error_if_parent_is_invalid_for_constantref(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::List(_) => Ok(()),
        ElementInfo::Root => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::Println => Ok(()),
        ElementInfo::If(_) => Ok(()),
        ElementInfo::Assignment => append_error(compiler, 0, 1, ERRORS.constants_are_immutable),
        ElementInfo::Parens => append_error(
            compiler,
            0,
            1,
            ERRORS.constantref_cant_be_child_of_parenthesis,
        ),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
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

pub fn error_if_parent_is_invalid_for_constant(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Assignment => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        _ => append_error(compiler, 0, 1, ERRORS.constant_undefined),
    }
}

pub fn error_if_parent_is_invalid_for_assignment(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Root => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        ElementInfo::If(_) => Ok(()),
        ElementInfo::List(_) => {
            append_error(compiler, 0, 1, ERRORS.assignment_cant_be_child_of_list)
        }
        ElementInfo::Constant(_, _) => {
            append_error(compiler, 0, 1, ERRORS.assignment_cant_be_child_of_constant)
        }
        ElementInfo::InbuiltFunctionCall(_, _, _) => append_error(
            compiler,
            0,
            1,
            ERRORS.assignment_cant_be_child_of_inbuiltfncall,
        ),
        ElementInfo::FunctionCall(_, _) => {
            append_error(compiler, 0, 1, ERRORS.assignment_cant_be_child_of_fncal)
        }
        ElementInfo::Assignment => append_error(
            compiler,
            0,
            1,
            ERRORS.assignment_cant_be_child_of_assignment,
        ),
        ElementInfo::Parens => append_error(
            compiler,
            0,
            1,
            ERRORS.assignment_cant_be_child_of_parenthesis,
        ),
        // explicitly listing other types rather than using _ to not overlook new types in future.
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => {
            append_error(compiler, 0, 1, ERRORS.impossible_error)
        }
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
        ElementInfo::Println => append_error(compiler, 0, 1, ERRORS.impossible_error),
    }
}

pub fn error_if_parent_is_invalid_for_inbuiltfncall(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Root => Ok(()),
        ElementInfo::List(_) => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::Assignment => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        ElementInfo::Println => Ok(()),
        ElementInfo::If(_) => Ok(()),
        ElementInfo::Parens => append_error(
            compiler,
            0,
            1,
            ERRORS.inbuiltfncall_cant_be_child_of_parenthesis,
        ),
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

pub fn error_if_parent_is_invalid_for_fncall(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Root => Ok(()),
        ElementInfo::List(_) => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::Assignment => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        ElementInfo::If(_) => Ok(()),
        ElementInfo::Parens => {
            append_error(compiler, 0, 1, ERRORS.fncall_cant_be_child_of_parenthesis)
        }
        // TODO need to allow parens for functionref
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
        ElementInfo::Println => Ok(()),
    }
}

pub fn error_if_parent_is_invalid_for_parenthesis(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::If(_) => Ok(()),
        ElementInfo::List(_) => append_error(compiler, 0, 1, ERRORS.list_cant_be_child),
        ElementInfo::LoopForRange(_, _, _) => {
            append_error(compiler, 0, 1, ERRORS.loopfor_cant_be_child)
        }
        ElementInfo::Parens => {
            append_error(compiler, 0, 1, ERRORS.fncall_cant_be_child_of_parenthesis)
        }
        ElementInfo::Root => append_error(compiler, 0, 1, ERRORS.parenthesis_cant_be_child_of_root),
        ElementInfo::Constant(_, _) => {
            append_error(compiler, 0, 1, ERRORS.parenthesis_cant_be_child_of_constant)
        }
        ElementInfo::Assignment => append_error(
            compiler,
            0,
            1,
            ERRORS.parenthesis_cant_be_child_of_assignment,
        ),
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
        ElementInfo::Println => append_error(compiler, 0, 1, ERRORS.impossible_error),
    }
}

pub fn error_if_parent_is_invalid_for_loopfor(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::List(_) => append_error(compiler, 0, 1, ERRORS.list_cant_be_child),
        ElementInfo::FunctionCall(_, _) => {
            append_error(compiler, 0, 1, ERRORS.loopfor_cant_be_child)
        }
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        ElementInfo::Root => Ok(()),
        ElementInfo::Println => Ok(()),
        ElementInfo::If(_) => Ok(()),
        ElementInfo::Parens => append_error(compiler, 0, 1, ERRORS.loopfor_cant_be_child),
        ElementInfo::Constant(_, _) => append_error(compiler, 0, 1, ERRORS.loopfor_cant_be_child),
        ElementInfo::Assignment => append_error(compiler, 0, 1, ERRORS.loopfor_cant_be_child),
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

pub fn error_if_parent_is_invalid_for_fndefwip(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::Println => Ok(()),
        ElementInfo::If(_) => Ok(()),
        ElementInfo::List(_) => append_error(
            compiler,
            0,
            1,
            ERRORS.fndefwip_can_only_be_child_of_constant,
        ),
        ElementInfo::LoopForRange(_, _, _) => append_error(
            compiler,
            0,
            1,
            ERRORS.fndefwip_can_only_be_child_of_constant,
        ),
        ElementInfo::Root => append_error(
            compiler,
            0,
            1,
            ERRORS.fndefwip_can_only_be_child_of_constant,
        ),
        ElementInfo::InbuiltFunctionCall(_, _, _) => append_error(
            compiler,
            0,
            1,
            ERRORS.fndefwip_can_only_be_child_of_constant,
        ),
        ElementInfo::FunctionCall(_, _) => append_error(
            compiler,
            0,
            1,
            ERRORS.fndefwip_can_only_be_child_of_constant,
        ),
        ElementInfo::Parens => append_error(
            compiler,
            0,
            1,
            ERRORS.fndefwip_can_only_be_child_of_constant,
        ),
        ElementInfo::Assignment => append_error(
            compiler,
            0,
            1,
            ERRORS.fndefwip_can_only_be_child_of_constant,
        ),
        // explicitly listing other types rather than using _ to not overlook new types in future.
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => {
            append_error(compiler, 0, 1, ERRORS.impossible_error)
        }
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

pub fn error_if_parent_is_invalid_for_println(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        ElementInfo::Root => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::If(_) => Ok(()),
        ElementInfo::List(_) => {
            append_error(compiler, 0, 1, ERRORS.println_cant_be_child_of_element)
        }
        ElementInfo::FunctionCall(_, _) => {
            append_error(compiler, 0, 1, ERRORS.println_cant_be_child_of_element)
        }
        ElementInfo::Parens => {
            append_error(compiler, 0, 1, ERRORS.println_cant_be_child_of_element)
        }
        ElementInfo::Assignment => {
            append_error(compiler, 0, 1, ERRORS.println_cant_be_child_of_element)
        }
        ElementInfo::Println => {
            append_error(compiler, 0, 1, ERRORS.println_cant_be_child_of_element)
        }
        // explicitly listing other types rather than using _ to not overlook new types in future.
        ElementInfo::Constant(_, _) => append_error(compiler, 0, 1, ERRORS.impossible_error),
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => {
            append_error(compiler, 0, 1, ERRORS.impossible_error)
        }
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

pub fn error_if_parent_is_invalid_for_if_expression(
    compiler: &mut Compiler,
    parent: &Element,
) -> Result<(), ()> {
    match parent.0 {
        ElementInfo::FunctionDef(_, _, _, _) => Ok(()),
        ElementInfo::FunctionDefWIP => Ok(()),
        ElementInfo::LoopForRangeWIP => Ok(()),
        ElementInfo::LoopForRange(_, _, _) => Ok(()),
        ElementInfo::Root => Ok(()),
        ElementInfo::InbuiltFunctionCall(_, _, _) => Ok(()),
        ElementInfo::If(_) => Ok(()),
        ElementInfo::List(_) => Ok(()),
        ElementInfo::FunctionCall(_, _) => Ok(()),
        ElementInfo::Assignment => Ok(()),
        ElementInfo::Constant(_, _) => Ok(()),
        ElementInfo::Parens => {
            append_error(compiler, 0, 1, ERRORS.println_cant_be_child_of_element)
        }
        ElementInfo::Println => {
            append_error(compiler, 0, 1, ERRORS.println_cant_be_child_of_element)
        }
        // explicitly listing other types rather than using _ to not overlook new types in future.
        ElementInfo::InbuiltFunctionDef(_, _, _, _, _) => {
            append_error(compiler, 0, 1, ERRORS.impossible_error)
        }
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
pub fn test_case_errors() -> Vec<Vec<String>> {
    vec![
        //
        //internalFunctionCalls
        //[ERRORS.int.to_string(),"+ 1 2.1".to_string()],
        //[ERRORS.int.to_string(),"- 1.1 2".to_string()],
        //
        //functionDefinitions
        //[ERRORS.funcdef_args.to_string(), "= a \\ =>".to_string()],
        //[ERRORS.funcdef_argtypes_first.to_string(),"= a \\ i64 monkey i64  =>".to_string()],
        //
    ]
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_error_scenario(tests: Vec<Vec<&str>>) {
        for test in tests {
            let input = &test[1];
            let error = &test[0];
            let mut c = crate::mock_compiler();
            c.file.filecontents = input.to_string();
            match c.run_main_tasks() {
                Ok(_) => {
                    if error == &"" && c.error_stack.len() == 0 {
                        assert_eq!(true, true)
                    } else {
                        assert!(c.error_stack[0].contains(error))
                    }
                }
                Err(_e) => assert!(false, "error should not exist"),
            }
        }
    }

    #[test]
    fn test_error_empty_file() {
        //empty file
        let tests = vec![vec!["", ""]];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_comment_singleline() {
        //comment single line
        let tests = vec![
            vec![ERRORS.comment_single_line, "/1/comment"],
            vec![ERRORS.comment_cant_be_child_of_assignment, "= //test"],
            vec![ERRORS.comment_cant_be_child_of_constant, "= c //test"],
            vec![ERRORS.comment_cant_be_child_of_inbuiltfncall, "+ //test"],
            vec![
                ERRORS.comment_cant_be_child_of_fncall,
                "= myfun \\ i64 i64 arg1 => + arg1 123\r\nmyfun //test",
            ],
            vec![
                ERRORS.comment_cant_be_child_of_parenthesis,
                "= myfun \\ ( i64 i64 // test ) i64 arg1 => arg1 123",
            ],
            vec![
                ERRORS.comment_cant_be_child_of_parenthesis,
                "= myfun \\ ( i64 i64 ) i64 arg1 => arg1 123\r\nmyfun ( //test ) 123",
            ],
        ];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_int() {
        let tests = vec![
            vec![ERRORS.int_cant_be_child_of_assignment, "= 123"],
            vec![
                ERRORS.int_cant_be_child_of_parenthesis,
                "= myfun \\ ( i64 123 ) i64 arg1 => arg1 123",
            ],
            vec![
                ERRORS.int_cant_be_child_of_parenthesis,
                "= myfun \\ ( i64 i64 ) i64 arg1 => arg1 123\r\nmyfun ( 123 ) 123",
            ],
            //int
            vec![ERRORS.int, "1a"],
            vec![ERRORS.int_out_of_bounds, "9223372036854775808"],
            //
            //int negative
            vec![ERRORS.int, "-1a"],
            vec![ERRORS.int_out_of_bounds, "-9223372036854775809"],
        ];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_float() {
        let tests = vec![
            vec![ERRORS.float_cant_be_child_of_assignment, "= 123.456"],
            vec![
                ERRORS.float_cant_be_child_of_parenthesis,
                "= myfun \\ ( i64 123.456 ) i64 arg1 => arg1 123",
            ],
            vec![
                ERRORS.float_cant_be_child_of_parenthesis,
                "= myfun \\ ( i64 i64 ) i64 arg1 => arg1 123\r\nmyfun ( 123.456 ) 123",
            ],
            //float (errors say int)
            vec![ERRORS.int, "1.1.1"],
            vec![ERRORS.int, "1.7976931348623157E+309"],
            //
            //float negative (errors say int)
            vec![ERRORS.int, "-1.1.1"],
            vec![ERRORS.int, "-1.7976931348623157E+309"],
        ];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_string() {
        let tests = vec![
            vec![ERRORS.string_cant_be_child_of_assignment, "= \"string\""],
            vec![
                ERRORS.string_cant_be_child_of_parenthesis,
                "= myfun \\ ( i64 \"string\" ) i64 arg1 => arg1 123",
            ],
            vec![
                ERRORS.string_cant_be_child_of_parenthesis,
                "= myfun \\ ( i64 i64 ) i64 arg1 => arg1 123\r\nmyfun ( \"string\" ) 123",
            ],
            //string
            vec![ERRORS.string, "\""],
            vec![ERRORS.string, "\"test\"\"test"],
        ];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_list() {
        let tests = vec![
            //lists - empty must contain type definition
            vec![ERRORS.list, "[ ]"],
            vec![ERRORS.list, "[]"],
        ];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_constantref() {
        let tests = vec![
            vec![
                ERRORS.constantref_cant_be_child_of_parenthesis,
                "= a 123\r\n= myfun \\ ( i64 a ) i64 arg1 => arg1 123",
            ],
            vec![
                ERRORS.constantref_cant_be_child_of_parenthesis,
                "= a 123\r\n= myfun \\ ( i64 i64 ) i64 arg1 => arg1 123\r\nmyfun ( a ) 123",
            ],
        ];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_constant() {
        let tests = vec![
            vec![ERRORS.constant_undefined, "a"],
            vec![ERRORS.constants_are_immutable, "= a 123\r\n= a 234"],
        ];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_assignment() {
        let tests = vec![
            vec![ERRORS.assignment_cant_be_child_of_constant, "= a ="],
            vec![ERRORS.assignment_cant_be_child_of_inbuiltfncall, "+ 123 ="],
            vec![
                ERRORS.assignment_cant_be_child_of_fncal,
                "= myfun \\ i64 i64 arg1 => + arg1 123\r\nmyfun = 123",
            ],
            vec![ERRORS.assignment_cant_be_child_of_assignment, "= ="],
            vec![
                ERRORS.assignment_cant_be_child_of_parenthesis,
                "= a 123\r\n= myfun \\ ( i64 = ) i64 arg1 => arg1 123",
            ],
            vec![
                ERRORS.assignment_cant_be_child_of_parenthesis,
                "= a 123\r\n= myfun \\ ( i64 i64 ) i64 arg1 => arg1 123\r\nmyfun ( = ) 123",
            ],
        ];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_inbuiltfncall() {
        let tests = vec![
            vec![
                ERRORS.inbuiltfncall_cant_be_child_of_parenthesis,
                "= a 123\r\n= myfun \\ ( i64 + ) i64 arg1 => arg1 123",
            ],
            vec![
                ERRORS.inbuiltfncall_cant_be_child_of_parenthesis,
                "= a 123\r\n= myfun \\ ( i64 i64 ) i64 arg1 => arg1 123\r\nmyfun ( + ) 123",
            ],
        ];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_fncall() {
        let tests = vec![
            // fncall_cant_be_child_of_parenthesis
            // but fails with other error funcdef_argtypes_first
            vec![
            ERRORS.funcdef_argtypes_first,
            "= myfun1 \\ i64 i64 arg1 => + arg1 123\r\n= myfun2 \\ ( i64 myfun1 ) i64 arg2 => arg2 123",
        ],
            //but not here
            //[
            //    ERRORS.fncall_cant_be_child_of_parenthesis,
            //    "= myfun1 \\ i64 i64 arg1 => + arg1 123\r\n= myfun2 \\ ( i64 i64 ) i64 arg2 => arg2 123\r\nmyfun2 ( myfun1 ) 123",
            //],
        ];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_parenthesis() {
        let tests = vec![
            vec![ERRORS.parenthesis_cant_be_child_of_root, "( i64 )"],
            vec![ERRORS.parenthesis_cant_be_child_of_constant, "= x ( i64 )"],
            vec![
                ERRORS.parenthesis_cant_be_child_of_assignment,
                "= ( i64 ) 123",
            ],
        ];
        test_error_scenario(tests);
    }

    #[test]
    fn test_error_fndefwip() {
        let tests = vec![
            vec![
                ERRORS.fndefwip_can_only_be_child_of_constant,
                "\\ i64 => 123",
            ],
            vec![ERRORS.fndefwip_can_only_be_child_of_constant, "+ 123 \\"],
            vec![
                ERRORS.fndefwip_can_only_be_child_of_constant,
                "= myfun \\ i64 i64 arg1 => + arg1 123\r\nmyfun \\",
            ],
            // fndefwip_can_only_be_child_of_constant
            // but fails with other error parens_of_assign
            vec![
                ERRORS.parenthesis_cant_be_child_of_assignment,
                "= ( \\ ) 123",
            ],
            vec![
                ERRORS.fndefwip_can_only_be_child_of_constant,
                "= a 123\r\n= myfun \\ ( \\ i64 ) i64 arg1 => arg1 123\r\nmyfun ( a ) 123",
            ],
            vec![ERRORS.fndefwip_can_only_be_child_of_constant, "= \\ 123"],
        ];
        test_error_scenario(tests);
    }
}
