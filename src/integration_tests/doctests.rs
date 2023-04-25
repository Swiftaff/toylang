/*!
 * Examples using the Toylang CLI
 */

//use crate::integration_tests::example_tests;
//extern crate lazy_static;

//use syn::LitStr;
//extern crate toylang_macros; //::{call_to_generate_doctest, call_to_generate_doctest4, call_to_generate_doctest5, generate_doctest};
use toylang_macros::{call_to_generate_doctest5, call_to_generate_doctest6, generate_doctest};

/*
macro_rules! generate_multiple_doctests {
    ($($test:expr),*) => {
        $(
            let fn_name = LitStr::new($test[0], proc_macro2::Span::call_site());
            let toy = LitStr::new($test[1], proc_macro2::Span::call_site());
            let rust = LitStr::new($test[2], proc_macro2::Span::call_site());
            generate_doctest!(fn_name, toy, rust);
        )*
    };
}
*/

call_to_generate_doctest6!(0);

#[allow(dead_code)]
//#[cfg(any(test, feature = "dox2"))]
#[cfg(test)]
mod tests {
    use crate::Compiler;
    use toylang_macros::{call_to_generate_doctest5, call_to_generate_doctest6, generate_doctest};

    call_to_generate_doctest6!(0);

    /// helper function for tests
    fn test_pass_single_scenario(test: Vec<&str>) {
        let input = &test[0];
        let output = &test[1];
        let mut c: Compiler = Default::default();
        c.file.filecontents = input.to_string();
        match c.run_main_tasks() {
            Ok(_) => {
                assert_eq!(&c.ast.output, output);
            }
            Err(_e) => assert!(false, "error should not exist"),
        }
    }

    macro_rules! doc_and_int_test {
        ( $doctest_name:ident, $test_name:ident, $x:expr, $y:expr ) => {
            #[doc = concat!("Toylang: ",stringify!($x))]
            #[doc = "```toylang"]
            #[doc = $x]
            #[doc = "```"]
            #[doc = "generates rust code:"]
            #[doc = stringify!($y)]
            #[doc = "```rust"]
            #[doc = $y]
            #[doc = "```"]
            //#[cfg_attr(not(feature = "dox2"), test)]
            //#[test]
            fn $doctest_name() {
                //println!("{}", stringify!($doctest_name));
                //test_pass_single_scenario(vec![$x, $y]);
            }

            #[test]
            fn $test_name() {
                test_pass_single_scenario(vec![$x, $y]);
            }
        };
    }

    #[test]
    fn test_name() {
        test_pass_single_scenario(vec!["", "fn main() {\r\n}\r\n"]);
    }

    //doc_and_int_test!(doctest1, test_pass_empty_file, "", "fn main() {\r\n}\r\n");

    // Comment
    doc_and_int_test!(doctest10, test_pass_comment_singleline, "//comment", "fn main() {\r\n    //comment\r\n}\r\n");
    doc_and_int_test!(doctest11, test_pass_comment_singleline_with_space, "    //    comment    ", "fn main() {\r\n    //    comment\r\n}\r\n");
    doc_and_int_test!(doctest12, test_pass_comment_singleline_fn_no_longer_breaks, "//= a \\ i64 => 123", "fn main() {\r\n    //= a \\ i64 => 123\r\n}\r\n");

    // Boolean
    doc_and_int_test!(doctest20, test_pass_boolean_true, "true", "fn main() {\r\n    true;\r\n}\r\n");
    doc_and_int_test!(doctest21, test_pass_boolean_false, "false", "fn main() {\r\n    false;\r\n}\r\n");
    doc_and_int_test!(doctest22, test_pass_boolean_eq_equality_true, "== 1 1", "fn main() {\r\n    1 == 1;\r\n}\r\n");
    doc_and_int_test!(doctest23, test_pass_boolean_eq_equality_false, "== 1 2", "fn main() {\r\n    1 == 2;\r\n}\r\n");
    doc_and_int_test!(doctest24, test_pass_boolean_neq_non_equality_true, "!= 1 2", "fn main() {\r\n    1 != 2;\r\n}\r\n");
    doc_and_int_test!(doctest25, test_pass_boolean_neq_non_equality_false, "!= 1 1", "fn main() {\r\n    1 != 1;\r\n}\r\n");
    doc_and_int_test!(doctest26, test_pass_boolean_gt_greater_than_true, "> 2 1", "fn main() {\r\n    2 > 1;\r\n}\r\n");
    doc_and_int_test!(doctest27, test_pass_boolean_gt_greater_than_false, "> 1 2", "fn main() {\r\n    1 > 2;\r\n}\r\n");
    doc_and_int_test!(doctest28, test_pass_boolean_lt_less_than_true, "< 1 2", "fn main() {\r\n    1 < 2;\r\n}\r\n");
    doc_and_int_test!(doctest29, test_pass_boolean_lt_less_than_false, "< 2 1", "fn main() {\r\n    2 < 1;\r\n}\r\n");
    doc_and_int_test!(doctest30, test_pass_boolean_gte_greater_than_equal_true, ">= 3 2", "fn main() {\r\n    3 >= 2;\r\n}\r\n");
    doc_and_int_test!(doctest31, test_pass_boolean_gte_greater_than_equal_true2, ">= 2 2", "fn main() {\r\n    2 >= 2;\r\n}\r\n");
    doc_and_int_test!(doctest32, test_pass_boolean_gte_greater_than_equal_false, ">= 1 2", "fn main() {\r\n    1 >= 2;\r\n}\r\n");
    doc_and_int_test!(doctest33, test_pass_boolean_lte_less_than_equal_true, "<= 2 3", "fn main() {\r\n    2 <= 3;\r\n}\r\n");
    doc_and_int_test!(doctest34, test_pass_boolean_lte_less_than_equal_true2, "<= 2 2", "fn main() {\r\n    2 <= 2;\r\n}\r\n");
    doc_and_int_test!(doctest35, test_pass_boolean_lte_less_than_equal_false, "<= 3 2", "fn main() {\r\n    3 <= 2;\r\n}\r\n");

    // String
    doc_and_int_test!(doctest40, test_pass_string, "\"string\"", "fn main() {\r\n    \"string\".to_string();\r\n}\r\n");
    doc_and_int_test!(doctest41, test_pass_string_escaped_quote, "\"\"", "fn main() {\r\n    \"\".to_string();\r\n}\r\n");

    // Int
    doc_and_int_test!(doctest50, test_pass_int, "1", "fn main() {\r\n    1 as i64;\r\n}\r\n");
    doc_and_int_test!(doctest51, test_pass_int_longer, "123", "fn main() {\r\n    123 as i64;\r\n}\r\n");
    doc_and_int_test!(doctest52, test_pass_int_space_before, "    123    ", "fn main() {\r\n    123 as i64;\r\n}\r\n");
    doc_and_int_test!(doctest53, test_pass_int_max, "9223372036854775807", "fn main() {\r\n    9223372036854775807 as i64;\r\n}\r\n");

    doc_and_int_test!(doctest54, test_pass_int_neg, "-1", "fn main() {\r\n    -1 as i64;\r\n}\r\n");
    doc_and_int_test!(doctest55, test_pass_int_longer_neg, "-123", "fn main() {\r\n    -123 as i64;\r\n}\r\n");
    doc_and_int_test!(doctest56, test_pass_int_space_before_neg, "    -123    ", "fn main() {\r\n    -123 as i64;\r\n}\r\n");
    doc_and_int_test!(doctest57, test_pass_int_max_neg, "-9223372036854775808", "fn main() {\r\n    -9223372036854775808 as i64;\r\n}\r\n");

    // Float
    doc_and_int_test!(doctest60, test_pass_float, "1.1", "fn main() {\r\n    1.1;\r\n}\r\n");
    doc_and_int_test!(doctest61, test_pass_float_longer, "123.123", "fn main() {\r\n    123.123;\r\n}\r\n");
    doc_and_int_test!(doctest62, test_pass_float_space_before, "    123.123    ", "fn main() {\r\n    123.123;\r\n}\r\n");
    doc_and_int_test!(doctest63, test_pass_float_max1, "1234567890.123456789", "fn main() {\r\n    1234567890.123456789;\r\n}\r\n");
    doc_and_int_test!(doctest64, test_pass_float_max2, "1.7976931348623157E+308", "fn main() {\r\n    1.7976931348623157E+308;\r\n}\r\n");

    doc_and_int_test!(doctest65, test_pass_float_neg, "-1.1", "fn main() {\r\n    -1.1;\r\n}\r\n");
    doc_and_int_test!(doctest66, test_pass_float_longer_neg, "-123.123", "fn main() {\r\n    -123.123;\r\n}\r\n");
    doc_and_int_test!(doctest67, test_pass_float_space_before_neg, "    -123.123    ", "fn main() {\r\n    -123.123;\r\n}\r\n");
    doc_and_int_test!(doctest68, test_pass_float_max1_neg, "-1234567890.123456789", "fn main() {\r\n    -1234567890.123456789;\r\n}\r\n");
    doc_and_int_test!(doctest69, test_pass_float_max2_neg, "-1.7976931348623157E+308", "fn main() {\r\n    -1.7976931348623157E+308;\r\n}\r\n");

    // List empty
    doc_and_int_test!(doctest70, test_pass_list_empty_string, "[ String ]", "fn main() {\r\n    Vec::<String>::new();\r\n}\r\n");
    doc_and_int_test!(doctest71, test_pass_list_empty_int, "[ i64 ]", "fn main() {\r\n    Vec::<i64>::new();\r\n}\r\n");
    doc_and_int_test!(doctest72, test_pass_list_empty_float, "[ f64 ]", "fn main() {\r\n    Vec::<f64>::new();\r\n}\r\n");

    // List not empty
    doc_and_int_test!(doctest80, test_pass_list_int, "[ 1 ]", "fn main() {\r\n    vec![ 1 ];\r\n}\r\n");
    doc_and_int_test!(doctest81, test_pass_list_int2, "[ 1 2 3 4 5 ]", "fn main() {\r\n    vec![ 1, 2, 3, 4, 5 ];\r\n}\r\n");
    doc_and_int_test!(doctest82, test_pass_list_float, "[ 1.1 2.2 3.3 4.4 5.5 ]", "fn main() {\r\n    vec![ 1.1, 2.2, 3.3, 4.4, 5.5 ];\r\n}\r\n");
    doc_and_int_test!(
        doctest83,
        test_pass_list_string,
        "[ \"1.1\" \"2.2\" \"3.3\" \"4.4\" \"5.5\" ]",
        "fn main() {\r\n    vec![ \"1.1\".to_string(), \"2.2\".to_string(), \"3.3\".to_string(), \"4.4\".to_string(), \"5.5\".to_string() ];\r\n}\r\n"
    );
    doc_and_int_test!(doctest84, test_pass_list_int_assign, "= x [ 1 2 3 4 5 ]", "fn main() {\r\n    let x: Vec<i64> = vec![ 1, 2, 3, 4, 5 ];\r\n}\r\n");
    doc_and_int_test!(
        doctest85,
        test_pass_list_float_assign,
        "= x [ 1.1 2.2 3.3 4.4 5.5 ]",
        "fn main() {\r\n    let x: Vec<f64> = vec![ 1.1, 2.2, 3.3, 4.4, 5.5 ];\r\n}\r\n"
    );
    doc_and_int_test!(
        doctest86,
        test_pass_list_string_assign,
        "= x [ \"1.1\" \"2.2\" \"3.3\" \"4.4\" \"5.5\" ]",
        "fn main() {\r\n    let x: Vec<String> = vec![ \"1.1\".to_string(), \"2.2\".to_string(), \"3.3\".to_string(), \"4.4\".to_string(), \"5.5\".to_string() ];\r\n}\r\n"
    );

    // List map
    /*
        doc_and_int_test!(
            doctest87,
            test_pass_list_map,
            "= list [ 1 ]\r\n= mapfn \\ i64 i64 i => * i 100\r\n= mapped List.map list ( mapfn )",
            "fn main() {\r\n    let list: Vec<i64> = vec![ 1 ];\r\n    fn mapfn(i: i64) -> i64 {\r\n        i * 100\r\n    }\r\n    let mapped: Vec<i64> = list.iter().map(mapfn).collect();\r\n}\r\n"
        );
    */
    // List append
    doc_and_int_test!(
        doctest88,
        test_pass_list_append,
        "= list1 [ 1 ]\r\n= list2 [ 2 3 ]\r\n= appended List.append list1 list2",
        "fn main() {\r\n    let list1: Vec<i64> = vec![ 1 ];\r\n    let list2: Vec<i64> = vec![ 2, 3 ];\r\n    let appended: Vec<i64> = list1.iter().cloned().chain(list2.iter().cloned()).collect();\r\n}\r\n"
    );

    // List len
    doc_and_int_test!(
        doctest89,
        test_pass_list_len,
        "= list [ 1 2 3 ]\r\n= len List.len list",
        "fn main() {\r\n    let list: Vec<i64> = vec![ 1, 2, 3 ];\r\n    let len: i64 = list.len() as i64;\r\n}\r\n"
    );

    // Function calls
    doc_and_int_test!(doctest90, test_pass_internal_function_calls_plus, "+ 1 2", "fn main() {\r\n    1 + 2;\r\n}\r\n");
    doc_and_int_test!(doctest91, test_pass_internal_function_calls_negative, "- 1.1 2.2", "fn main() {\r\n    1.1 - 2.2;\r\n}\r\n");
    doc_and_int_test!(doctest92, test_pass_internal_function_calls_multiply, "* 3 4", "fn main() {\r\n    3 * 4;\r\n}\r\n");
    doc_and_int_test!(doctest93, test_pass_internal_function_calls_divide, "/ 9 3", "fn main() {\r\n    9 / 3;\r\n}\r\n");

    // Basic arithmetic assignment type inference
    doc_and_int_test!(doctest100, test_pass_assign_type_inf_plus_int, "= a + 1 2", "fn main() {\r\n    let a: i64 = 1 + 2;\r\n}\r\n");
    doc_and_int_test!(doctest101, test_pass_assign_type_inf_plus_float, "= a + 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 + 2.2;\r\n}\r\n");
    doc_and_int_test!(doctest102, test_pass_assign_type_inf_plus_minus_int, "= a - 1 2", "fn main() {\r\n    let a: i64 = 1 - 2;\r\n}\r\n");
    doc_and_int_test!(doctest103, test_pass_assign_type_inf_minus_float, "= a - 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 - 2.2;\r\n}\r\n");
    doc_and_int_test!(doctest104, test_pass_assign_type_inf_multiply_int, "= a * 1 2", "fn main() {\r\n    let a: i64 = 1 * 2;\r\n}\r\n");
    doc_and_int_test!(doctest105, test_pass_assign_type_inf_multiply_float, "= a * 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 * 2.2;\r\n}\r\n");
    doc_and_int_test!(doctest106, test_pass_assign_type_inf_divide_int, "= a / 1 2", "fn main() {\r\n    let a: i64 = 1 / 2;\r\n}\r\n");
    doc_and_int_test!(doctest107, test_pass_assign_type_inf_divide_float, "= a / 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 / 2.2;\r\n}\r\n");
    doc_and_int_test!(doctest108, test_pass_assign_type_inf_modulo_int, "= a % 1 2", "fn main() {\r\n    let a: i64 = 1 % 2;\r\n}\r\n");
    doc_and_int_test!(doctest109, test_pass_assign_type_inf_modulo_float, "= a % 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 % 2.2;\r\n}\r\n");

    // Constant
    doc_and_int_test!(doctest110, test_pass_constant, "= a 123\r\na", "fn main() {\r\n    let a: i64 = 123;\r\n    a;\r\n}\r\n");

    // Assignment
    doc_and_int_test!(doctest120, test_pass_assignment_string, "= a \"string\"", "fn main() {\r\n    let a: String = \"string\".to_string();\r\n}\r\n");
    doc_and_int_test!(doctest121, test_pass_assignment_int, "= a 1", "fn main() {\r\n    let a: i64 = 1;\r\n}\r\n");
    doc_and_int_test!(doctest122, test_pass_assignment_float, "= a 1.1", "fn main() {\r\n    let a: f64 = 1.1;\r\n}\r\n");
    doc_and_int_test!(doctest123, test_pass_assignment_float_neg, "= a -1.7976931348623157E+308", "fn main() {\r\n    let a: f64 = -1.7976931348623157E+308;\r\n}\r\n");
    doc_and_int_test!(doctest124, test_pass_assignment_arithmetic, "= a + 1 2", "fn main() {\r\n    let a: i64 = 1 + 2;\r\n}\r\n");
    doc_and_int_test!(
        doctest125,
        test_pass_assignment_internal_function_calls_with_references,
        "= a + 1 2\r\n= b - 3 a",
        "fn main() {\r\n    let a: i64 = 1 + 2;\r\n    let b: i64 = 3 - a;\r\n}\r\n"
    );

    // Functions
    doc_and_int_test!(doctest130, test_pass_nested_internal_function_call1, "= a - + 1 2 3", "fn main() {\r\n    let a: i64 = 1 + 2 - 3;\r\n}\r\n");
    doc_and_int_test!(doctest131, test_pass_nested_internal_function_call2, "= a / * - + 1 2 3 4 5", "fn main() {\r\n    let a: i64 = 1 + 2 - 3 * 4 / 5;\r\n}\r\n");
    doc_and_int_test!(doctest132, test_pass_nested_internal_function_call3, "= a + 1 * 3 2", "fn main() {\r\n    let a: i64 = 1 + 3 * 2;\r\n}\r\n");

    doc_and_int_test!(doctest133, test_pass_func_def_singleline1, "= a \\ i64 => 123", "fn main() {\r\n    fn a() -> i64 {\r\n        123 as i64\r\n    }\r\n}\r\n");
    doc_and_int_test!(
        doctest134,
        test_pass_func_def_singleline2,
        "= a \\ i64 i64 arg1 => + 123 arg1",
        "fn main() {\r\n    fn a(arg1: i64) -> i64 {\r\n        123 + arg1\r\n    }\r\n}\r\n"
    );

    doc_and_int_test!(
        doctest135,
        test_pass_func_def_multiline1,
        "= a \\ i64 i64 i64 arg1 arg2 =>\r\n+ arg1 arg2",
        "fn main() {\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        arg1 + arg2\r\n    }\r\n}\r\n"
    );
    doc_and_int_test!(
        doctest136,
        test_pass_func_def_multiline2,
        "= a \\ i64 i64 i64 i64 arg1 arg2 arg3 =>\r\n= x + arg1 arg2\r\n+ x arg3",
        "fn main() {\r\n    fn a(arg1: i64, arg2: i64, arg3: i64) -> i64 {\r\n        let x: i64 = arg1 + arg2;\r\n        x + arg3\r\n    }\r\n}\r\n"
    );

    doc_and_int_test!(
        doctest137,
        test_pass_func_def_multiline_nested,
        "= a \\ i64 i64 i64 i64 arg1 arg2 arg3 =>\r\n + arg1 + arg2 arg3",
        "fn main() {\r\n    fn a(arg1: i64, arg2: i64, arg3: i64) -> i64 {\r\n        arg1 + arg2 + arg3\r\n    }\r\n}\r\n"
    );

    doc_and_int_test!(
        doctest138,
        test_pass_func_def_multiline_const_assign_nested,
        "= a \\ i64 i64 i64 arg1 arg2 =>\r\n= arg3 + arg2 123\r\n+ arg3 arg1",
        "fn main() {\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        let arg3: i64 = arg2 + 123;\r\n        arg3 + arg1\r\n    }\r\n}\r\n"
    );
    doc_and_int_test!(
        doctest139,
        test_pass_func_def_multiline_several_semicolon_and_return,
        "= a \\ i64 i64 i64 arg1 arg2 =>\r\n= b + arg1 123\r\n= c - b arg2\r\n= z * c 10\r\nz",
        "fn main() {\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        let b: i64 = arg1 + 123;\r\n        let c: i64 = b - arg2;\r\n        let z: i64 = c * 10;\r\n        z\r\n    }\r\n}\r\n"
    );

    //arg1 is a function that takes i64 returns i64, arg2 is an i64
    //the function body calls arg1 with arg2 as its argument, returning which returns i64
    /*
    // working excerpt using 2 outdents  in outdent::functioncall_of_arg
    33: FunctionCall: arg1 (&dyn Fn(i64) -> i64) [ 34, ]
    34: ConstantRef: arg2 (i64) for "arg2" [ ]
    35: Indent [ ]
    36: Unused [ ]
    37: Unused [ ]
    38: FunctionDef: b (arg3: i64) -> (i64) [ 42, 43, ]
    39: Type: i64 [ ]
    40: Type: i64 [ ]
    41: Arg: arg3 scope:38 (i64) [ ]
    42: Indent [ ]
    43: InbuiltFunctionCall: + (i64) [ 44, 45, ]
    44: Int: 123 [ ]
    45: ConstantRef: arg3 (i64) for "arg3" [ ]
    46: Indent [ ]
    47: Assignment [ 48, ]
    48: Constant: c (i64) [ 49, ]
    49: FunctionCall: a (i64) [ 50, 51, ]
    50: ConstantRef: b (i64) for "b" [ ]
    51: Int: 456 [ ]
    52: Seol [ ]
    */
    doc_and_int_test!(
        doctest140,
        test_pass_passing_func_as_args,
        "= a \\ ( i64 i64 ) i64 i64 arg1 arg2 =>\r\n arg1 arg2\r\n= b \\ i64 i64 arg3 => + 123 arg3\r\n= c a ( b ) 456",
        "fn main() {\r\n    fn a(arg1: &dyn Fn(i64) -> i64, arg2: i64) -> i64 {\r\n        arg1(arg2)\r\n    }\r\n    fn b(arg3: i64) -> i64 {\r\n        123 + arg3\r\n    }\r\n    let c: i64 = a(&b, 456);\r\n}\r\n"
    );
    doc_and_int_test!(
        doctest141,
        test_pass_type_inference_assign_to_constref,
        "= a 123\r\n= aa a\r\n= aaa aa\r\n= aaaa aaa",
        "fn main() {\r\n    let a: i64 = 123;\r\n    let aa: i64 = a;\r\n    let aaa: i64 = aa;\r\n    let aaaa: i64 = aaa;\r\n}\r\n"
    );
    doc_and_int_test!(doctest142, test_pass_type_inference_assign_to_funccall, "= a + 1 2", "fn main() {\r\n    let a: i64 = 1 + 2;\r\n}\r\n");

    /*
    doc_and_int_test!(doctest10, test_pass_boolean_string, );
    */
}
