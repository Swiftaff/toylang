use crate::Compiler;

#[derive(Clone, Debug)]
pub struct Errors {
    pub comment_single_line: &'static str,
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
}

pub const ERRORS: Errors = Errors {
    comment_single_line: "Invalid single line comment: Must begin with two forward slashes '//'",
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
    constants_are_immutable: "Constants are immutable. You may be trying to assign a value to a constant that has already been defined. Try renaming this as a new constant."
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
