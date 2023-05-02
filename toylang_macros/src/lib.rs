/*!
The purpose of these are to create 2 proc_macros
one to generate standard tests - comparing the input to expected output
two to generate document tests - to check that the output rust code is actually valid
by having it in the doctests this is checked automatically at compile time!
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Error, Parse, ParseStream};
use syn::parse_macro_input;
use toylang_common;

lazy_static::lazy_static! {
    /// ALL_TESTS is a central variable stored in toylang_common containing all the tests
    /// which both sets of proc_macros refer to, rather than having to manually duplicate them in two places in the code
    /// of the module, and of the module tests.
    static ref ALL_TESTS: toylang_common::IntegrationTests = toylang_common::IntegrationTests::new();
}

/// DocTestOrTest's first three strings are expected for parsing - they are generated from the tests in ALL_TESTS
/// where each test is a tuple containing these three items as strings.
/// The last two are generated, one for tests, the other for doctests
struct DocTestOrTest {
    _fn_name: syn::LitStr,
    toylang_input: syn::LitStr,
    expected_rust_output: syn::LitStr,
    //
    fn_name_for_test: syn::Ident,
    fn_name_for_doctest: syn::Ident,
}

/// Parser for DocTestOrTest expects three strings separated by two commas
/// string, string, string
impl Parse for DocTestOrTest {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        // first string
        let fn_name: syn::LitStr = input.parse()?;

        // first comma
        input.parse::<syn::Token![,]>()?;

        // second string
        let toylang_input: syn::LitStr = input.parse()?;

        // second comma
        input.parse::<syn::Token![,]>()?;

        // third string
        let expected_rust_output: syn::LitStr = input.parse()?;

        // generate the test and doctest function names
        let fn_name_for_test = syn::Ident::new(&fn_name.value(), fn_name.span());
        let fn_name_for_doctest =
            syn::Ident::new(&format!("doc{}", fn_name.value()), fn_name.span());

        // return
        Ok(DocTestOrTest {
            _fn_name: fn_name,
            toylang_input,
            expected_rust_output,
            fn_name_for_test,
            fn_name_for_doctest,
        })
    }
}

/// TestIndex is the index of the test required out of ALL_TESTS
struct TestIndex {
    index: syn::LitInt,
}

/// Parser for TestIndex
impl Parse for TestIndex {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        // parse a single int
        let index: syn::LitInt = input.parse()?;

        // return
        Ok(TestIndex { index })
    }
}

/// TestName is the name of the test required out of ALL_TESTS
struct TestName {
    _name: syn::LitStr,
}

/// Parser for TestName
impl Parse for TestName {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        // parse a single string
        let name: syn::LitStr = input.parse()?;

        // return
        Ok(TestName { _name: name })
    }
}

/// ## DocTests
///
/// Takes three strings, generates a single doctest
#[proc_macro]
pub fn generate_single_doctest(input: TokenStream) -> TokenStream {
    let DocTestOrTest {
        _fn_name: _,
        toylang_input,
        expected_rust_output,
        fn_name_for_test: _,
        fn_name_for_doctest,
    } = parse_macro_input!(input as DocTestOrTest);
    let output = quote! {
        #[doc = concat!("Toylang: ",stringify!(#toylang_input))]
        #[doc = "```toylang"]
        #[doc = #toylang_input]
        #[doc = "```"]
        #[doc = "generates rust code:"]
        #[doc = stringify!(#expected_rust_output)]
        #[doc = "```rust"]
        #[doc = #expected_rust_output]
        #[doc = "```"]
        pub fn #fn_name_for_doctest() {
            println!("testy");
        }
    };
    output.into()
}

/// Takes an integer for the required test from proc macro below, generates three strings to call proc macro above
#[proc_macro]
pub fn call_to_generate_single_doctest(input: TokenStream) -> TokenStream {
    let TestIndex { index } = parse_macro_input!(input as TestIndex);
    let i = index.to_string().parse::<usize>().unwrap();
    let fn_name = &ALL_TESTS.tests[i].0;
    let toy = &ALL_TESTS.tests[i].1;
    let rust = &ALL_TESTS.tests[i].2;
    let output = quote! {
        generate_single_doctest!(#fn_name,#toy,#rust);
    };
    output.into()
}

/// No input needed, just call it once and it loops over ALL_TESTS, calling proc macro above
#[proc_macro]
pub fn call_to_generate_all_doctests(_input: TokenStream) -> TokenStream {
    let loopy = (0..ALL_TESTS.tests.len()).map(syn::Index::from);
    let output = quote! {
        #(
            call_to_generate_single_doctest!(#loopy);
        )*
    };
    output.into()
}

/// ## Tests
///
/// Takes three strings, generates a single test
#[proc_macro]
pub fn generate_single_test(input: TokenStream) -> TokenStream {
    let DocTestOrTest {
        _fn_name: _,
        toylang_input,
        expected_rust_output,
        fn_name_for_test,
        fn_name_for_doctest: _,
    } = parse_macro_input!(input as DocTestOrTest);
    //note: "test_pass_single_scenario" is defined in the toylang integration tests, not needed here
    let output = quote! {
        #[test]
        fn #fn_name_for_test() {
            test_pass_single_scenario(vec![#toylang_input, #expected_rust_output]);
        }
    };
    output.into()
}

/// Takes an integer for the required test from proc macro below, generates three strings to call proc macro above
#[proc_macro]
pub fn call_to_generate_single_test(input: TokenStream) -> TokenStream {
    let TestIndex { index } = parse_macro_input!(input as TestIndex);
    let i = index.to_string().parse::<usize>().unwrap();
    let fn_name = &ALL_TESTS.tests[i].0;
    let toy = &ALL_TESTS.tests[i].1;
    let rust = &ALL_TESTS.tests[i].2;
    let output = quote! {
        generate_single_test!(#fn_name,#toy,#rust);
    };
    output.into()
}

/// Use this once with empty string and it loops over ALL_TESTS, calling proc macro above.
/// Or provide a name to run only one test
#[proc_macro]
pub fn call_to_generate_all_tests_or_only_named(input: TokenStream) -> TokenStream {
    let TestName { _name: name } = parse_macro_input!(input as TestName);
    let only = ALL_TESTS
        .tests
        .clone()
        .into_iter()
        .position(|t| t.0 == name.value());
    match only {
        Some(index) => {
            dbg!("WARNING! only running one test");
            dbg!("WARNING! only running one test");
            dbg!("WARNING! only running one test");
            dbg!("WARNING! only running one test");
            dbg!("WARNING! only running one test");
            let test_index = syn::Index::from(index);
            let output = quote! {
                call_to_generate_single_test!(#test_index);
            };
            return output.into();
        }
        None => {
            let loopy = (0..ALL_TESTS.tests.len()).map(syn::Index::from);
            let output = quote! {
                #(
                    call_to_generate_single_test!(#loopy);
                )*
            };
            return output.into();
        }
    };
}
