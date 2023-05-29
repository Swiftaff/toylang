type TestName = String;
type ToylangCodeInput = String;
type ExpectedRustCodeOutput = String;

pub struct IntegrationTests {
    pub tests: Vec<(TestName, ToylangCodeInput, ExpectedRustCodeOutput)>,
}

impl IntegrationTests {
    pub fn new() -> IntegrationTests {
        let test_strs = vec![
            ("test_pass_empty_file", "", "fn main() {\r\n}\r\n"),
            //
            // Comments
            ("test_pass_comment_singleline", "//comment", "fn main() {\r\n    //comment\r\n}\r\n"),
            ("test_pass_comment_singleline_with_space", "    //    comment    ", "fn main() {\r\n    //    comment\r\n}\r\n"),
            ("test_pass_comment_singleline_fn_no_longer_breaks", "//= a \\ i64 => 123", "fn main() {\r\n    //= a \\ i64 => 123\r\n}\r\n"),
            //
            // Boolean
            ("test_pass_boolean_true", "true", "fn main() {\r\n    true;\r\n}\r\n"),
            ("test_pass_boolean_false", "false", "fn main() {\r\n    false;\r\n}\r\n"),
            ("test_pass_boolean_eq_equality_true", "== 1 1", "fn main() {\r\n    1 == 1;\r\n}\r\n"),
            ("test_pass_boolean_eq_equality_false", "== 1 2", "fn main() {\r\n    1 == 2;\r\n}\r\n"),
            ("test_pass_boolean_neq_non_equality_true", "!= 1 2", "fn main() {\r\n    1 != 2;\r\n}\r\n"),
            ("test_pass_boolean_neq_non_equality_false", "!= 1 1", "fn main() {\r\n    1 != 1;\r\n}\r\n"),
            ("test_pass_boolean_gt_greater_than_true", "> 2 1", "fn main() {\r\n    2 > 1;\r\n}\r\n"),
            ("test_pass_boolean_gt_greater_than_false", "> 1 2", "fn main() {\r\n    1 > 2;\r\n}\r\n"),
            ("test_pass_boolean_lt_less_than_true", "< 1 2", "fn main() {\r\n    1 < 2;\r\n}\r\n"),
            ("test_pass_boolean_lt_less_than_false", "< 2 1", "fn main() {\r\n    2 < 1;\r\n}\r\n"),
            ("test_pass_boolean_gte_greater_than_equal_true", ">= 3 2", "fn main() {\r\n    3 >= 2;\r\n}\r\n"),
            ("test_pass_boolean_gte_greater_than_equal_true2", ">= 2 2", "fn main() {\r\n    2 >= 2;\r\n}\r\n"),
            ("test_pass_boolean_gte_greater_than_equal_false", ">= 1 2", "fn main() {\r\n    1 >= 2;\r\n}\r\n"),
            ("test_pass_boolean_lte_less_than_equal_true", "<= 2 3", "fn main() {\r\n    2 <= 3;\r\n}\r\n"),
            ("test_pass_boolean_lte_less_than_equal_true2", "<= 2 2", "fn main() {\r\n    2 <= 2;\r\n}\r\n"),
            ("test_pass_boolean_lte_less_than_equal_false", "<= 3 2", "fn main() {\r\n    3 <= 2;\r\n}\r\n"),
            //
            // String
            ("test_pass_string", "\"string\"", "fn main() {\r\n    \"string\".to_string();\r\n}\r\n"),
            ("test_pass_string_escaped_quote", "\"\"", "fn main() {\r\n    \"\".to_string();\r\n}\r\n"),
            //
            // Int
            ("test_pass_int", "1", "fn main() {\r\n    1 as i64;\r\n}\r\n"),
            ("test_pass_int_longer", "123", "fn main() {\r\n    123 as i64;\r\n}\r\n"),
            ("test_pass_int_space_before", "    123    ", "fn main() {\r\n    123 as i64;\r\n}\r\n"),
            ("test_pass_int_max", "9223372036854775807", "fn main() {\r\n    9223372036854775807 as i64;\r\n}\r\n"),
            ("test_pass_int_neg", "-1", "fn main() {\r\n    -1 as i64;\r\n}\r\n"),
            ("test_pass_int_longer_neg", "-123", "fn main() {\r\n    -123 as i64;\r\n}\r\n"),
            ("test_pass_int_space_before_neg", "    -123    ", "fn main() {\r\n    -123 as i64;\r\n}\r\n"),
            ("test_pass_int_max_neg", "-9223372036854775808", "fn main() {\r\n    -9223372036854775808 as i64;\r\n}\r\n"),
            //
            // Float
            ("test_pass_float", "1.1", "fn main() {\r\n    1.1;\r\n}\r\n"),
            ("test_pass_float_longer", "123.123", "fn main() {\r\n    123.123;\r\n}\r\n"),
            ("test_pass_float_space_before", "    123.123    ", "fn main() {\r\n    123.123;\r\n}\r\n"),
            ("test_pass_float_max1", "1234567890.123456789", "fn main() {\r\n    1234567890.123456789;\r\n}\r\n"),
            ("test_pass_float_max2", "1.7976931348623157E+308", "fn main() {\r\n    1.7976931348623157E+308;\r\n}\r\n"),
            ("test_pass_float_neg", "-1.1", "fn main() {\r\n    -1.1;\r\n}\r\n"),
            ("test_pass_float_longer_neg", "-123.123", "fn main() {\r\n    -123.123;\r\n}\r\n"),
            ("test_pass_float_space_before_neg", "    -123.123    ", "fn main() {\r\n    -123.123;\r\n}\r\n"),
            ("test_pass_float_max1_neg", "-1234567890.123456789", "fn main() {\r\n    -1234567890.123456789;\r\n}\r\n"),
            ("test_pass_float_max2_neg", "-1.7976931348623157E+308", "fn main() {\r\n    -1.7976931348623157E+308;\r\n}\r\n"),
            //
            // List empty
            ("test_pass_list_empty_string", "[ String ]", "fn main() {\r\n    Vec::<String>::new();\r\n}\r\n"),
            ("test_pass_list_empty_int", "[ i64 ]", "fn main() {\r\n    Vec::<i64>::new();\r\n}\r\n"),
            ("test_pass_list_empty_float", "[ f64 ]", "fn main() {\r\n    Vec::<f64>::new();\r\n}\r\n"),
            //
            // List not empty
            ("test_pass_list_int", "[ 1 ]", "fn main() {\r\n    vec![ 1 ];\r\n}\r\n"),
            ("test_pass_list_int2", "[ 1 2 3 4 5 ]", "fn main() {\r\n    vec![ 1, 2, 3, 4, 5 ];\r\n}\r\n"),
            ("test_pass_list_float", "[ 1.1 2.2 3.3 4.4 5.5 ]", "fn main() {\r\n    vec![ 1.1, 2.2, 3.3, 4.4, 5.5 ];\r\n}\r\n"),
            (
                "test_pass_list_string",
                "[ \"1.1\" \"2.2\" \"3.3\" \"4.4\" \"5.5\" ]",
                "fn main() {\r\n    vec![ \"1.1\".to_string(), \"2.2\".to_string(), \"3.3\".to_string(), \"4.4\".to_string(), \"5.5\".to_string() ];\r\n}\r\n",
            ),
            ("test_pass_list_int_assign", "= x [ 1 2 3 4 5 ]", "fn main() {\r\n    let x: Vec<i64> = vec![ 1, 2, 3, 4, 5 ];\r\n}\r\n"),
            ("test_pass_list_float_assign", "= x [ 1.1 2.2 3.3 4.4 5.5 ]", "fn main() {\r\n    let x: Vec<f64> = vec![ 1.1, 2.2, 3.3, 4.4, 5.5 ];\r\n}\r\n"),
            (
                "test_pass_list_string_assign",
                "= x [ \"1.1\" \"2.2\" \"3.3\" \"4.4\" \"5.5\" ]",
                "fn main() {\r\n    let x: Vec<String> = vec![ \"1.1\".to_string(), \"2.2\".to_string(), \"3.3\".to_string(), \"4.4\".to_string(), \"5.5\".to_string() ];\r\n}\r\n",
            ),
            //
            // List append
            (
                "test_pass_list_append",
                "= list1 [ 1 ]\r\n= list2 [ 2 3 ]\r\n= appended List::append list1 list2",
                "fn main() {\r\n    let list1: Vec<i64> = vec![ 1 ];\r\n    let list2: Vec<i64> = vec![ 2, 3 ];\r\n    let appended: Vec<i64> = list1.clone().iter().cloned().chain(list2.clone().iter().cloned()).collect();\r\n}\r\n",
            ),
            //
            // List len
            ("test_pass_list_len", "= list [ 1 2 3 ]\r\n= len List::len list", "fn main() {\r\n    let list: Vec<i64> = vec![ 1, 2, 3 ];\r\n    let len: i64 = list.clone().len() as i64;\r\n}\r\n"),
            //
            // List map
            (
                "test_pass_list_map",
                "= list [ 1 ]\r\n= mapfn \\ i64 i64 i => * i 100\r\n= mapped List::map list ( mapfn )",
                "fn main() {\r\n    let list: Vec<i64> = vec![ 1 ];\r\n    fn mapfn(i: i64) -> i64 {\r\n        i.clone() * 100\r\n    }\r\n    fn mapfn_for_list_map(i: &i64) -> i64 {\r\n        i.clone() * 100\r\n    }\r\nlet mapped: Vec<i64> = list.clone().iter().map(mapfn_for_list_map.clone()).collect();\r\n}\r\n",
            ),
            //
            // List reverse
            (
                "test_pass_list_reverse",
                "= list [ 1 2 3 ]\r\n= reversed List::reverse list",
                "fn main() {\r\n    let list: Vec<i64> = vec![ 1, 2, 3 ];\r\n    let reversed: Vec<i64> = list.clone().into_iter().rev().collect();\r\n}\r\n",
            ),
            (
                "test_pass_list_reverse_float",
                "= list [ 1.1 2.1 3.1 ]\r\n= reversed List::reverse list",
                "fn main() {\r\n    let list: Vec<f64> = vec![ 1.1, 2.1, 3.1 ];\r\n    let reversed: Vec<f64> = list.clone().into_iter().rev().collect();\r\n}\r\n",
            ),
            //
            // Basic arithmetic function calls
            ("test_pass_internal_function_calls_plus", "+ 1 2", "fn main() {\r\n    1 + 2;\r\n}\r\n"),
            ("test_pass_internal_function_calls_negative", "- 1.1 2.2", "fn main() {\r\n    1.1 - 2.2;\r\n}\r\n"),
            ("test_pass_internal_function_calls_multiply", "* 3 4", "fn main() {\r\n    3 * 4;\r\n}\r\n"),
            ("test_pass_internal_function_calls_divide", "/ 9 3", "fn main() {\r\n    9 / 3;\r\n}\r\n"),
            //
            // Basic arithmetic assignment type inference
            ("test_pass_assign_type_inf_plus_int", "= a + 1 2", "fn main() {\r\n    let a: i64 = 1 + 2;\r\n}\r\n"),
            ("test_pass_assign_type_inf_plus_float", "= a + 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 + 2.2;\r\n}\r\n"),
            ("test_pass_assign_type_inf_plus_minus_int", "= a - 1 2", "fn main() {\r\n    let a: i64 = 1 - 2;\r\n}\r\n"),
            ("test_pass_assign_type_inf_minus_float", "= a - 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 - 2.2;\r\n}\r\n"),
            ("test_pass_assign_type_inf_multiply_int", "= a * 1 2", "fn main() {\r\n    let a: i64 = 1 * 2;\r\n}\r\n"),
            ("test_pass_assign_type_inf_multiply_float", "= a * 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 * 2.2;\r\n}\r\n"),
            ("test_pass_assign_type_inf_divide_int", "= a / 1 2", "fn main() {\r\n    let a: i64 = 1 / 2;\r\n}\r\n"),
            ("test_pass_assign_type_inf_divide_float", "= a / 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 / 2.2;\r\n}\r\n"),
            ("test_pass_assign_type_inf_modulo_int", "= a % 1 2", "fn main() {\r\n    let a: i64 = 1 % 2;\r\n}\r\n"),
            ("test_pass_assign_type_inf_modulo_float", "= a % 1.1 2.2", "fn main() {\r\n    let a: f64 = 1.1 % 2.2;\r\n}\r\n"),
            //
            // Constant
            ("test_pass_constant", "= a 123\r\na", "fn main() {\r\n    let a: i64 = 123;\r\n    a.clone();\r\n}\r\n"),
            //
            // Assignment
            ("test_pass_assignment_string", "= a \"string\"", "fn main() {\r\n    let a: String = \"string\".to_string();\r\n}\r\n"),
            ("test_pass_assignment_int", "= a 1", "fn main() {\r\n    let a: i64 = 1;\r\n}\r\n"),
            ("test_pass_assignment_float", "= a 1.1", "fn main() {\r\n    let a: f64 = 1.1;\r\n}\r\n"),
            ("test_pass_assignment_float_neg", "= a -1.7976931348623157E+308", "fn main() {\r\n    let a: f64 = -1.7976931348623157E+308;\r\n}\r\n"),
            ("test_pass_assignment_arithmetic", "= a + 1 2", "fn main() {\r\n    let a: i64 = 1 + 2;\r\n}\r\n"),
            ("test_pass_assignment_internal_function_calls_with_references", "= a + 1 2\r\n= b - 3 a", "fn main() {\r\n    let a: i64 = 1 + 2;\r\n    let b: i64 = 3 - a.clone();\r\n}\r\n"),
            //
            // Functions
            ("test_pass_nested_internal_function_call1", "= a - + 1 2 3", "fn main() {\r\n    let a: i64 = 1 + 2 - 3;\r\n}\r\n"),
            ("test_pass_nested_internal_function_call2", "= a / * - + 1 2 3 4 5", "fn main() {\r\n    let a: i64 = 1 + 2 - 3 * 4 / 5;\r\n}\r\n"),
            ("test_pass_nested_internal_function_call3", "= a + 1 * 3 2", "fn main() {\r\n    let a: i64 = 1 + 3 * 2;\r\n}\r\n"),
            ("test_pass_func_def_singleline1", "= a \\ i64 => 123", "fn main() {\r\n    fn a() -> i64 {\r\n        123 as i64\r\n    }\r\n}\r\n"),
            ("test_pass_func_def_singleline2", "= a \\ i64 i64 arg1 => + 123 arg1", "fn main() {\r\n    fn a(arg1: i64) -> i64 {\r\n        123 + arg1.clone()\r\n    }\r\n}\r\n"),
            ("test_pass_func_def_singleline_list", "= a \\ [ i64 ] => [ 1 2 3 ]", "fn main() {\r\n    fn a() -> Vec<i64> {\r\n        vec![ 1, 2, 3 ]\r\n    }\r\n}\r\n"),
            ("test_pass_func_def_singleline_nested_list", "= a \\ [ [ i64 ] ] => [ [ 1 2 3 ] [ 4 5 6 ] ]", "fn main() {\r\n    fn a() -> Vec<Vec<i64>> {\r\n        vec![ vec![ 1, 2, 3 ], vec![ 4, 5, 6 ] ]\r\n    }\r\n}\r\n"),
            ("test_pass_func_def_multiline1", "= a \\ i64 i64 i64 arg1 arg2 =>\r\n+ arg1 arg2", "fn main() {\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        arg1.clone() + arg2.clone()\r\n    }\r\n}\r\n"),
            (
                "test_pass_func_def_multiline2",
                "= a \\ i64 i64 i64 i64 arg1 arg2 arg3 =>\r\n= x + arg1 arg2\r\n+ x arg3",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64, arg3: i64) -> i64 {\r\n        let x: i64 = arg1.clone() + arg2.clone();\r\n        x.clone() + arg3.clone()\r\n    }\r\n}\r\n",
            ),
            (
                "test_pass_func_def_multiline_nested",
                "= a \\ i64 i64 i64 i64 arg1 arg2 arg3 =>\r\n + arg1 + arg2 arg3",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64, arg3: i64) -> i64 {\r\n        arg1.clone() + arg2.clone() + arg3.clone()\r\n    }\r\n}\r\n",
            ),
            (
                "test_pass_func_def_multiline_const_assign_nested",
                "= a \\ i64 i64 i64 arg1 arg2 =>\r\n= arg3 + arg2 123\r\n+ arg3 arg1",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        let arg3: i64 = arg2.clone() + 123;\r\n        arg3.clone() + arg1.clone()\r\n    }\r\n}\r\n",
            ),
            (
                "test_pass_func_def_multiline_several_semicolon_and_return",
                "= a \\ i64 i64 i64 arg1 arg2 =>\r\n= b + arg1 123\r\n= c - b arg2\r\n= z * c 10\r\nz",
                "fn main() {\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        let b: i64 = arg1.clone() + 123;\r\n        let c: i64 = b.clone() - arg2.clone();\r\n        let z: i64 = c.clone() * 10;\r\n        z.clone()\r\n    }\r\n}\r\n",
            ),
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
            (
                "test_pass_passing_func_as_args",
                "= a \\ ( i64 i64 ) i64 i64 arg1 arg2 =>\r\n arg1 arg2\r\n= b \\ i64 i64 arg3 => + 123 arg3\r\n= c a ( b ) 456",
                "fn main() {\r\n    fn a(arg1: &dyn Fn(i64) -> i64, arg2: i64) -> i64 {\r\n        arg1(arg2.clone())\r\n    }\r\n    fn b(arg3: i64) -> i64 {\r\n        123 + arg3.clone()\r\n    }\r\n    let c: i64 = a(&b.clone(), 456);\r\n}\r\n",
            ),
            // TODO func with no args and return value
            //
            // Struct
            (
                "test_pass_define_struct",
                "= newstruct { = firstname \"firstname\" = surname \"surname\" = age 21 }",
                "#[derive(Clone, Debug)]\r\npub struct Newstruct {\r\n    pub firstname: String,\r\n    pub surname: String,\r\n    pub age: i64,\r\n}\r\n\r\nimpl Newstruct {\r\n    pub fn new(\r\n        firstname: String,\r\n        surname: String,\r\n        age: i64,\r\n) -> Newstruct {\r\n        Newstruct {\r\n            firstname,\r\n            surname,\r\n            age,\r\n        }\r\n    }\r\n}\r\n\r\nfn main() {\r\n    let mut newstruct: Newstruct = Newstruct::new(\"firstname\".to_string(), \"surname\".to_string(), 21);\r\n}\r\n",
            ),
            (
                "test_pass_define_struct_debug_print",
                "= newstruct { = firstname \"firstname\" = surname \"surname\" = age 21 }\r\n@ newstruct",
                "#[derive(Clone, Debug)]\r\npub struct Newstruct {\r\n    pub firstname: String,\r\n    pub surname: String,\r\n    pub age: i64,\r\n}\r\n\r\nimpl Newstruct {\r\n    pub fn new(\r\n        firstname: String,\r\n        surname: String,\r\n        age: i64,\r\n) -> Newstruct {\r\n        Newstruct {\r\n            firstname,\r\n            surname,\r\n            age,\r\n        }\r\n    }\r\n}\r\n\r\nfn main() {\r\n    let mut newstruct: Newstruct = Newstruct::new(\"firstname\".to_string(), \"surname\".to_string(), 21);\r\n    println!(\"{:?}\", &newstruct.clone());\r\n}\r\n",
            ),
            (
                "test_pass_define_struct_with_short_notation_using_constantref",
                "= firstname \"firstname\"\r\n= surname \"surname\"\r\n= age 21\r\n= newstruct { firstname surname age }",
                "#[derive(Clone, Debug)]\r\npub struct Newstruct {\r\n    pub firstname: String,\r\n    pub surname: String,\r\n    pub age: i64,\r\n}\r\n\r\nimpl Newstruct {\r\n    pub fn new(\r\n        firstname: String,\r\n        surname: String,\r\n        age: i64,\r\n) -> Newstruct {\r\n        Newstruct {\r\n            firstname,\r\n            surname,\r\n            age,\r\n        }\r\n    }\r\n}\r\n\r\nfn main() {\r\n    let firstname: String = \"firstname\".to_string();\r\n    let surname: String = \"surname\".to_string();\r\n    let age: i64 = 21;\r\n    let mut newstruct: Newstruct = Newstruct::new(firstname.clone(), surname.clone(), age.clone());\r\n}\r\n",
            ),
            (
                "test_pass_define_two_structs_with_same_short_notation_fields_reuses_existing_struct",
                "= firstname \"firstname\"\r\n= surname \"surname\"\r\n= age 21\r\n= newstruct { firstname surname age }\r\n= newstruct2 { firstname surname age }",
                "#[derive(Clone, Debug)]\r\npub struct Newstruct {\r\n    pub firstname: String,\r\n    pub surname: String,\r\n    pub age: i64,\r\n}\r\n\r\nimpl Newstruct {\r\n    pub fn new(\r\n        firstname: String,\r\n        surname: String,\r\n        age: i64,\r\n) -> Newstruct {\r\n        Newstruct {\r\n            firstname,\r\n            surname,\r\n            age,\r\n        }\r\n    }\r\n}\r\n\r\nfn main() {\r\n    let firstname: String = \"firstname\".to_string();\r\n    let surname: String = \"surname\".to_string();\r\n    let age: i64 = 21;\r\n    let mut newstruct: Newstruct = Newstruct::new(firstname.clone(), surname.clone(), age.clone());\r\n    let mut newstruct2: Newstruct = Newstruct::new(firstname.clone(), surname.clone(), age.clone());\r\n}\r\n",
            ),
            // TODO - version with inline fields needs to work with scoping
            /*
            (
                "test_pass_define_two_structs_with_same_inline_fields_reuses_existing_struct",
                "= newstruct { = firstname \"firstname\" = surname \"surname\" = age 21 }\r\n= newstruct2 { = firstname \"firstname2\" = surname \"surname2\" = age 22 }",
                "#[derive(Clone, Debug)]\r\npub struct Newstruct {\r\n    pub firstname: String,\r\n    pub surname: String,\r\n    pub age: i64,\r\n}\r\n\r\nimpl Newstruct {\r\n    pub fn new(\r\n        firstname: String,\r\n        surname: String,\r\n        age: i64,\r\n) -> Newstruct {\r\n        Newstruct {\r\n            firstname,\r\n            surname,\r\n            age,\r\n        }\r\n    }\r\n}\r\n\r\nfn main() {\r\n    let mut newstruct: Newstruct = Newstruct::new(\"firstname\".to_string(), \"surname\".to_string(), 21);\r\n    let mut newstruct2: Newstruct = Newstruct::new(\"firstname2\".to_string(), \"surname2\".to_string(), 22);}\r\n",
            ),
            */
            // TODO - Also how to print a key - it is same as a struct edit - need to check for assignment before to differentiate
            // TODO - need to outdent at end of struct def only once for constantref, compared to 3 times for assign, constant, value - depending on what last struct item is defined
            // TODO -- not possible? create a new outdent_until fn (current parent ref) - so you no longer have to count
            (
                "test_pass_define_struct_edit_and_print",
                "= newstruct { = firstname \"firstname\" = surname \"surname\" = age 21 }\r\n= newstruct.age 99\r\n@ newstruct",
                "#[derive(Clone, Debug)]\r\npub struct Newstruct {\r\n    pub firstname: String,\r\n    pub surname: String,\r\n    pub age: i64,\r\n}\r\n\r\nimpl Newstruct {\r\n    pub fn new(\r\n        firstname: String,\r\n        surname: String,\r\n        age: i64,\r\n) -> Newstruct {\r\n        Newstruct {\r\n            firstname,\r\n            surname,\r\n            age,\r\n        }\r\n    }\r\n}\r\n\r\nfn main() {\r\n    let mut newstruct: Newstruct = Newstruct::new(\"firstname\".to_string(), \"surname\".to_string(), 21);\r\n    newstruct.age = 99;\r\n    println!(\"{:?}\", &newstruct.clone());\r\n}\r\n",
            ),
            //
            // Type inference
            (
                "test_pass_type_inference_assign_to_constref",
                "= a 123\r\n= aa a\r\n= aaa aa\r\n= aaaa aaa",
                "fn main() {\r\n    let a: i64 = 123;\r\n    let aa: i64 = a.clone();\r\n    let aaa: i64 = aa.clone();\r\n    let aaaa: i64 = aaa.clone();\r\n}\r\n",
            ),
            ("test_pass_type_inference_assign_to_funccall", "= a + 1 2", "fn main() {\r\n    let a: i64 = 1 + 2;\r\n}\r\n"),
            (
                "test_pass_type_inference_assign_to_constref_of_funccall",
                "= a + 1 2\r\n= aa a\r\n= aaa aa\r\n= aaaa aaa",
                "fn main() {\r\n    let a: i64 = 1 + 2;\r\n    let aa: i64 = a.clone();\r\n    let aaa: i64 = aa.clone();\r\n    let aaaa: i64 = aaa.clone();\r\n}\r\n",
            ),
            (
                "test_pass_fndef_return_statement",
                "= a \\ i64 => ? == 1 1 1 0\r\na",
                "fn main() {\r\n    fn a() -> i64 {\r\n        if 1 == 1 {\r\n            1\r\n        } else {\r\n            0\r\n        }\r\n    }\r\n    a();\r\n}\r\n",
            ),
            (
                "test_pass_funccall_zero_args",
                "//define function\r\n= a \\ i64 =>\r\n123\r\n\r\n//call function\r\na",
                "fn main() {\r\n    //define function\r\n    fn a() -> i64 {\r\n        123 as i64\r\n    }\r\n    //call function\r\n    a();\r\n}\r\n",
            ),
            // TODO function call void/null/() return
            (
                "test_pass_funccall_one_arg",
                "//define function\r\n= a \\ i64 i64 arg1 =>\r\narg1\r\n\r\n//call function\r\na 123",
                "fn main() {\r\n    //define function\r\n    fn a(arg1: i64) -> i64 {\r\n        arg1.clone()\r\n    }\r\n    //call function\r\n    a(123);\r\n}\r\n",
            ),
            (
                "test_pass_funccall_two_args_eval_internal_func_call",
                "//define function\r\n= a \\ i64 i64 i64 arg1 arg2 =>\r\n+ arg1 arg2\r\n\r\n//call function\r\na + 123 456 789",
                "fn main() {\r\n    //define function\r\n    fn a(arg1: i64, arg2: i64) -> i64 {\r\n        arg1.clone() + arg2.clone()\r\n    }\r\n    //call function\r\n    a(123 + 456, 789);\r\n}\r\n",
            ),
            //
            // Println
            ("test_pass_println_int", "@ 1", "fn main() {\r\n    println!(\"{}\", 1);\r\n}\r\n"),
            ("test_pass_println_float", "@ 1.23", "fn main() {\r\n    println!(\"{}\", 1.23);\r\n}\r\n"),
            ("test_pass_println_string", "@ \"Hello, world\"", "fn main() {\r\n    println!(\"{}\", \"Hello, world\".to_string());\r\n}\r\n"),
            ("test_pass_println_fn_call", "@ + 1 2", "fn main() {\r\n    println!(\"{}\", 1 + 2);\r\n}\r\n"),
            ("test_pass_println_constantref", "= a 1\r\n@ a", "fn main() {\r\n    let a: i64 = 1;\r\n    println!(\"{}\", a.clone());\r\n}\r\n"),
            ("test_pass_println_constantref_twice", "= a 1\r\n= b a\r\n@ b", "fn main() {\r\n    let a: i64 = 1;\r\n    let b: i64 = a.clone();\r\n    println!(\"{}\", b.clone());\r\n}\r\n"),
            ("test_pass_println_from_fn_def", "= a \\ i64 => 1\r\n@ a", "fn main() {\r\n    fn a() -> i64 {\r\n        1 as i64\r\n    }\r\n    println!(\"{}\", a());\r\n}\r\n"),
            //
            // If expressions / Ternary expressions
            ("test_pass_if", "? true 1 0", "fn main() {\r\n    if true {\r\n        1\r\n    } else {\r\n        0\r\n    };\r\n}\r\n"),
            (
                "test_pass_if_assignment",
                "= get_true \\ bool => true\r\n? get_true 1 0",
                "fn main() {\r\n    fn get_true() -> bool {\r\n        true\r\n    }\r\n    if get_true() {\r\n        1\r\n    } else {\r\n        0\r\n    };\r\n}\r\n",
            ),
            (
                "test_pass_if_fn_assignment",
                "= get_truer \\ i64 bool arg1 => > arg1 5\r\n? get_truer 10 1 0",
                "fn main() {\r\n    fn get_truer(arg1: i64) -> bool {\r\n        arg1.clone() > 5\r\n    }\r\n    if get_truer(10) {\r\n        1\r\n    } else {\r\n        0\r\n    };\r\n}\r\n",
            ),
            //
            // Rust code
            ("test_pass_rustcode_premain_only", "###use std::io::{stdin, stdout, Write};\r\n= x 1", "use std::io::{stdin, stdout, Write};\r\nfn main() {\r\n    let x: i64 = 1;\r\n}\r\n"),
            ("test_pass_rustcode_premain_and_main", "###use std::io::{stdin, stdout, Write};\r\n= x 1\r\n##stdout().flush().unwrap();", "use std::io::{stdin, stdout, Write};\r\nfn main() {\r\n    let x: i64 = 1;\r\n    stdout().flush().unwrap();\r\n}\r\n"),
            ("test_pass_rustcode_main_only", "= x 1\r\n##println!(\"{}\",x);", "fn main() {\r\n    let x: i64 = 1;\r\n    println!(\"{}\",x);\r\n}\r\n"),

            
            
            /*
            (TODO is valid output but has extra spaces - need to find way to remove Indents when If is used in an assignment)
            ("test_pass_if_assignment_with_if_expr", "= a ? true 1 0", "fn main() {\r\n    let a: i64 =             if true {\r\n                1\r\n            } else {\r\n                0\r\n            };\r\n}\r\n"),
            */
            //
            // Example Functions
            (
                "test_pass_example_fibonacci",
                "= fibonacci \\ i64 i64 n => ? < n 2 1 + fibonacci - n 1 fibonacci - n 2\r\n@ fibonacci 10",
                "fn main() {\r\n    fn fibonacci(n: i64) -> i64 {\r\n        if n.clone() < 2 {\r\n            1\r\n        } else {\r\n            fibonacci(n.clone() - 1) + fibonacci(n.clone() - 2)\r\n        }\r\n    }\r\n    println!(\"{}\", fibonacci(10));\r\n}\r\n",
            ),
            /* Loops - for loops - not required?
            ("for_loops",
                "= a \\ i64 i64 arg1 => + 123 arg1\r\n.. b 0 100\r\na b\r\n.",
                "fn main() {\r\n    fn a(arg1: i64) -> i64 {\r\n        123 + arg1\r\n    }\r\n    for b in 0..100 {\r\n        a(b);\r\n    }\r\n}\r\n")
            */
        ];
        let tests = test_strs.iter().map(|(a, b, c)| (a.to_string(), b.to_string(), c.to_string())).collect::<Vec<(TestName, ToylangCodeInput, ExpectedRustCodeOutput)>>();
        IntegrationTests { tests }
    }
}
