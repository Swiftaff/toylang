mod test_macros {
    use toylang_macros::generate_doctest;

    //generate_doctest!("testy", "te", "st");
}

#[cfg(test)]
mod tests {
    extern crate toylang;
    use toylang::tests::example_tests;
    //use crate::tests::example_tests::ExampleTests;

    use toylang_macros::{
        call_to_generate_doctest, call_to_generate_doctest2, call_to_generate_doctest3,
        call_to_generate_doctest4, example_proc_macro,
    };

    #[test]
    fn test_example_proc_macro() {
        example_proc_macro!("te", "st");
        assert_eq!(concatenate_toy_and_rust(), "test");
    }

    #[test]
    fn test_call_to_generate_doctest() {
        call_to_generate_doctest!();
        assert_eq!(concatenate_toy_and_rust(), "leftright");
    }

    #[test]
    fn test_call_to_generate_doctest2() {
        call_to_generate_doctest2!();
        assert_eq!(concatenate_toy_and_rust(), "left2right2");
    }

    #[test]
    fn test_call_to_generate_doctest3() {
        call_to_generate_doctest3!();
        assert_eq!(concatenate_toy_and_rust(), "left3right3");
    }

    #[test]
    fn test_call_to_generate_doctest4() {
        call_to_generate_doctest4!(1);
        assert_eq!(concatenate_toy_and_rust(), "left4bright4b");
    }

    /*
    #[test]
    fn test_call_to_generate_doctest5() {
        call_to_generate_doctest5!(1);
        assert_eq!(concatenate_toy_and_rust(), "cd");
    }
    */
}
