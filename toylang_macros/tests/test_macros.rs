#[cfg(test)]
mod tests {
    use toylang_macros::example_proc_macro;

    struct Testy {
        fn_name: String,
        comment1: String,
        comment2: String,
    }

    impl Testy {
        pub fn new(name: &str, comment1: &str, comment2: &str) -> Testy {
            Testy {
                fn_name: name.to_string(),
                comment1: comment1.to_string(),
                comment2: comment2.to_string(),
            }
        }
    }

    #[test]
    fn test_macro() {
        let v = Testy::new("name_of_function", "a", "b");
        example_proc_macro!(testy, "te", "st");

        //example_proc_macro!(v);

        assert_eq!(testy(), "test");
    }
}

/*
#[test]
fn test_macro() {
    //"test_pass_empty_file" "" "fn main() {\r\n}\r\n"
    //let s = Testy::new("testymctestface");
    //let v = vec!["a".to_string(), "b".to_string()];
    //assert_eq!(name_of_function(), "testing");
    assert_eq!(true, true);
}
*/
