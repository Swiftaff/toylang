/*!
 * Examples using the Toylang CLI
*/

#[cfg(any(test, feature = "dox2"))]
pub mod integration_tests {
    use crate::Compiler;

    macro_rules! doc_and_int_test {
        ( $test_name:ident, $x:expr, $y:expr ) => {
            #[doc = concat!("Toylang: ",stringify!($x))]
            #[doc = "```toylang"]
            #[doc = $x]
            #[doc = "```"]
            #[doc = "generates rust code:"]
            #[doc = stringify!($y)]
            #[doc = "```rust"]
            #[doc = $y]
            #[doc = "```"]
            #[cfg_attr(not(feature = "dox2"), test)]
            fn $test_name() {
                test_pass_single_scenario(vec![$x, $y]);
            }
        };
    }

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

    doc_and_int_test!(test_pass_empty_file, "", "fn main() {\r\n}\r\n");

    // Comment
    doc_and_int_test!(test_pass_comment_singleline, "//comment", "fn main() {\r\n    //comment\r\n}\r\n");
    doc_and_int_test!(test_pass_comment_singleline_with_space, "    //    comment    ", "fn main() {\r\n    //    comment\r\n}\r\n");
    doc_and_int_test!(test_pass_comment_singleline_fn_no_longer_breaks, "//= a \\ i64 => 123", "fn main() {\r\n    //= a \\ i64 => 123\r\n}\r\n");

    // Boolean
    doc_and_int_test!(test_pass_boolean_true, "true", "fn main() {\r\n    true;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_false, "false", "fn main() {\r\n    false;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_eq_equality_true, "== 1 1", "fn main() {\r\n    1 == 1;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_eq_equality_false, "== 1 2", "fn main() {\r\n    1 == 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_neq_non_equality_true, "!= 1 2", "fn main() {\r\n    1 != 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_neq_non_equality_false, "!= 1 1", "fn main() {\r\n    1 != 1;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_gt_greater_than_true, "> 2 1", "fn main() {\r\n    2 > 1;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_gt_greater_than_false, "> 1 2", "fn main() {\r\n    1 > 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_lt_less_than_true, "< 1 2", "fn main() {\r\n    1 < 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_lt_less_than_false, "< 2 1", "fn main() {\r\n    2 < 1;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_gte_greater_than_equal_true, ">= 3 2", "fn main() {\r\n    3 >= 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_gte_greater_than_equal_true2, ">= 2 2", "fn main() {\r\n    2 >= 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_gte_greater_than_equal_false, ">= 1 2", "fn main() {\r\n    1 >= 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_lte_less_than_equal_true, "<= 2 3", "fn main() {\r\n    2 >= 3;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_lte_less_than_equal_true2, "<= 2 2", "fn main() {\r\n    2 <= 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_boolean_lte_less_than_equal_false, "<= 3 2", "fn main() {\r\n    3 <= 2;\r\n}\r\n");

    // String
    doc_and_int_test!(test_pass_string, "\"string\"", "fn main() {\r\n    \"string\".to_string();\r\n}\r\n");
    doc_and_int_test!(test_pass_string_escaped_quote, "\"\"", "fn main() {\r\n    \"\".to_string();\r\n}\r\n");

    // Int
    doc_and_int_test!(test_pass_int, "1", "fn main() {\r\n    1;\r\n}\r\n");
    doc_and_int_test!(test_pass_int_longer, "123", "fn main() {\r\n    123;\r\n}\r\n");
    doc_and_int_test!(test_pass_int_space_before, "    123    ", "fn main() {\r\n    123;\r\n}\r\n");
    doc_and_int_test!(test_pass_int_max, "9223372036854775807", "fn main() {\r\n    9223372036854775807;\r\n}\r\n");

    doc_and_int_test!(test_pass_int_neg, "-1", "fn main() {\r\n    -1;\r\n}\r\n");
    doc_and_int_test!(test_pass_int_longer_neg, "-123", "fn main() {\r\n    -123;\r\n}\r\n");
    doc_and_int_test!(test_pass_int_space_before_neg, "    -123    ", "fn main() {\r\n    -123;\r\n}\r\n");
    doc_and_int_test!(test_pass_int_max_neg, "-9223372036854775808", "fn main() {\r\n    -9223372036854775808;\r\n}\r\n");

    // Float
    doc_and_int_test!(test_pass_float, "1.1", "fn main() {\r\n    1.1;\r\n}\r\n");
    doc_and_int_test!(test_pass_float_longer, "123.123", "fn main() {\r\n    123.123;\r\n}\r\n");
    doc_and_int_test!(test_pass_float_space_before, "    123.123    ", "fn main() {\r\n    123.123;\r\n}\r\n");
    doc_and_int_test!(test_pass_float_max1, "1234567890.123456789", "fn main() {\r\n    1234567890.123456789;\r\n}\r\n");
    doc_and_int_test!(test_pass_float_max2, "1.7976931348623157E+308", "fn main() {\r\n    1.7976931348623157E+308;\r\n}\r\n");

    doc_and_int_test!(test_pass_float_neg, "-1.1", "fn main() {\r\n    -1.1;\r\n}\r\n");
    doc_and_int_test!(test_pass_float_longer_neg, "-123.123", "fn main() {\r\n    -123.123;\r\n}\r\n");
    doc_and_int_test!(test_pass_float_space_before_neg, "    -123.123    ", "fn main() {\r\n    -123.123;\r\n}\r\n");
    doc_and_int_test!(test_pass_float_max1_neg, "-1234567890.123456789", "fn main() {\r\n    -1234567890.123456789;\r\n}\r\n");
    doc_and_int_test!(test_pass_float_max2_neg, "-1.7976931348623157E+308", "fn main() {\r\n    -1.7976931348623157E+308;\r\n}\r\n");

    // List empty
    doc_and_int_test!(test_pass_list_empty_string, "[ String ]", "fn main() {\r\n    Vec::<String>::new();\r\n}\r\n");
    doc_and_int_test!(test_pass_list_empty_int, "[ i64 ]", "fn main() {\r\n    Vec::<i64>::new();\r\n}\r\n");
    doc_and_int_test!(test_pass_list_empty_float, "[ f64 ]", "fn main() {\r\n    Vec::<f64>::new();\r\n}\r\n");

    // List not empty
    doc_and_int_test!(test_pass_list_int, "[ 1 ]", "fn main() {\r\n    vec![ 1 ];\r\n}\r\n");
    doc_and_int_test!(test_pass_list_int2, "[ 1 2 3 4 5 ]", "fn main() {\r\n    vec![ 1, 2, 3, 4, 5 ];\r\n}\r\n");
    doc_and_int_test!(test_pass_list_float, "[ 1.1 2.2 3.3 4.4 5.5 ]", "fn main() {\r\n    vec![ 1.1, 2.2, 3.3, 4.4, 5.5 ];\r\n}\r\n");
    doc_and_int_test!(
        test_pass_list_string,
        "[ \"1.1\" \"2.2\" \"3.3\" \"4.4\" \"5.5\" ]",
        "fn main() {\r\n    vec![ \"1.1\".to_string(), \"2.2\".to_string(), \"3.3\".to_string(), \"4.4\".to_string(), \"5.5\".to_string() ];\r\n}\r\n"
    );
    doc_and_int_test!(test_pass_list_int_assign, "= x [ 1 2 3 4 5 ]", "fn main() {\r\n    let x: Vec<i64> = vec![ 1, 2, 3, 4, 5 ];\r\n}\r\n");
    doc_and_int_test!(test_pass_list_float_assign, "= x [ 1.1 2.2 3.3 4.4 5.5 ]", "fn main() {\r\n    let x: Vec<f64> = vec![ 1.1, 2.2, 3.3, 4.4, 5.5 ];\r\n}\r\n");
    doc_and_int_test!(
        test_pass_list_string_assign,
        "= x [ \"1.1\" \"2.2\" \"3.3\" \"4.4\" \"5.5\" ]",
        "fn main() {\r\n    let x: Vec<String> = vec![ \"1.1\".to_string(), \"2.2\".to_string(), \"3.3\".to_string(), \"4.4\".to_string(), \"5.5\".to_string() ];\r\n}\r\n"
    );

    // List map

    doc_and_int_test!(
        test_pass_list_map,
        "= list [ 1 ]\r\n= mapfn \\ i64 i64 i => * i 100\r\n= mapped List.map list ( mapfn )",
        "fn main() {\r\n    let list: Vec<i64> = vec![ 1 ];\r\n    fn mapfn(i: i64) -> i64 {\r\n        i * 100\r\n    }\r\n    let mapped: Vec<i64> = list.iter().map(mapfn).collect();\r\n}\r\n"
    );

    // List append
    doc_and_int_test!(
        test_pass_list_append,
        "= list1 [ 1 ]\r\n= list2 [ 2 3 ]\r\n= appended List.append list1 list2",
        "fn main() {\r\n    let list1: Vec<i64> = vec![ 1 ];\r\n    let list2: Vec<i64> = vec![ 2, 3 ];\r\n    let appended: Vec<i64> = list1.iter().cloned().chain(list2.iter().cloned()).collect();\r\n}\r\n"
    );

    // List len
    doc_and_int_test!(
        test_pass_list_len,
        "= list [ 1 2 3 ]\r\n= len List.len list",
        "fn main() {\r\n    let list: Vec<i64> = vec![ 1, 2, 3 ];\r\n    let len: i64 = list.len() as i64;\r\n}\r\n"
    );

    // Function calls
    doc_and_int_test!(test_pass_internal_function_calls_plus, "+ 1 2", "fn main() {\r\n    1 + 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_internal_function_calls_negative, "- 1.1 2.2", "fn main() {\r\n    1.1 - 2.2;\r\n}\r\n");
    doc_and_int_test!(test_pass_internal_function_calls_multiply, "* 3 4", "fn main() {\r\n    3 * 4;\r\n}\r\n");
    doc_and_int_test!(test_pass_internal_function_calls_divide, "/ 9 3", "fn main() {\r\n    9 / 3;\r\n}\r\n");

    // Basic arithmetic assignment type inference
    doc_and_int_test!(test_pass_assign_type_inf_plus_int, "= a + 1 2", "fn main() {\r\n    let a: i64 = 1 + 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_assign_type_inf_plus_float, "= a + 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 + 2.2;\r\n}\r\n");
    doc_and_int_test!(test_pass_assign_type_inf_plus_minus_int, "= a - 1 2", "fn main() {\r\n    let a: i64 = 1 - 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_assign_type_inf_minus_float, "= a - 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 - 2.2;\r\n}\r\n");
    doc_and_int_test!(test_pass_assign_type_inf_multiply_int, "= a * 1 2", "fn main() {\r\n    let a: i64 = 1 * 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_assign_type_inf_multiply_float, "= a * 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 * 2.2;\r\n}\r\n");
    doc_and_int_test!(test_pass_assign_type_inf_divide_int, "= a / 1 2", "fn main() {\r\n    let a: i64 = 1 / 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_assign_type_inf_divide_float, "= a / 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 / 2.2;\r\n}\r\n");
    doc_and_int_test!(test_pass_assign_type_inf_modulo_int, "= a % 1 2", "fn main() {\r\n    let a: i64 = 1 % 2;\r\n}\r\n");
    doc_and_int_test!(test_pass_assign_type_inf_modulo_float, "= a % 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 % 2.2;\r\n}\r\n");

    // Constant
    doc_and_int_test!(test_pass_constant, "= a 123\r\na", "fn main() {\r\n    let a: i64 = 123;\r\n    a;\r\n}\r\n");

    // Assignment
    doc_and_int_test!(test_pass_assignment_string, "= a \"string\"", "fn main() {\r\n    let a: String = \"string\".to_string();\r\n}\r\n");
    doc_and_int_test!(test_pass_assignment_int, "= a 1", "fn main() {\r\n    let a: i64 = 1;\r\n}\r\n");
    doc_and_int_test!(test_pass_assignment_float, "= a 1.1", "fn main() {\r\n    let a: f64 = 1.1;\r\n}\r\n");
    doc_and_int_test!(test_pass_assignment_float_neg, "= a -1.7976931348623157E+308", "fn main() {\r\n    let a: f64 = -1.7976931348623157E+308;\r\n}\r\n");
    doc_and_int_test!(test_pass_assignment_arithmetic, "= a + 1 2", "fn main() {\r\n    let a: i64 = 1 + 2;\r\n}\r\n");

    /*
    doc_and_int_test!(test_pass_boolean_string, );
    */
}
