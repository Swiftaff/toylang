/*!
 * Examples for use with the Toylang CLI
 *
 * These tests are used twice
 * - First as standard integration tests - comparing the input to expected output
 * - Second as document tests - which checks at build time that the output code examples are actually valid rust code
 *
 * ### Hello, world!
 *   Toylang code:
 *   ```toylang
 *   @ "Hello, world!"
 *   ```
 *   Compiles to Rust code:
 *   ```rust
 *   fn main() {
 *       println!("{}", "Hello, world!".to_string());
 *   }
 *   ```
 */

use toylang_macros::{
    call_to_generate_all_doctests, call_to_generate_single_doctest, generate_single_doctest,
};

call_to_generate_all_doctests!();

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use crate::Compiler;
    use toylang_macros::{
        call_to_generate_all_tests_or_only_named, call_to_generate_single_test,
        generate_single_test,
    };

    /// helper function for tests
    fn test_pass_single_scenario(test: Vec<&str>) {
        let input = &test[0];
        let output = &test[1];
        let mut c: Compiler = Default::default();
        c.file.filecontents = input.to_string();
        match c.run_main_tasks(false) {
            Ok(_) => {
                assert_eq!(&c.ast.output, output);
            }
            Err(_e) => assert!(false, "error should not exist"),
        }
    }

    // Update the test name, and restart rust-analyzer to run only one test
    call_to_generate_all_tests_or_only_named!("");
}
