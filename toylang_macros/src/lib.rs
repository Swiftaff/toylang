/*!
The purpose of these are experiments to create 2 proc_macros
one to generate standard tests - comparing the input to expected output
two to generate document tests - to check that the output rust code is actually valid
by having it in the doctests this is checked automatically at compile time!

I am hoping to have a central variable containing all the tests
which both sets of proc_macros could refer to, rather than manually duplicate them.
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Error, Parse, ParseStream};
use syn::parse_macro_input;
use toylang_common;

/// DocTest's first three strings are expected for parsing
/// the last two are generated, one for tests, the other for doctests
struct DocTest {
    fn_name: syn::LitStr,
    toylang_input: syn::LitStr,
    expected_rust_output: syn::LitStr,
    //
    fn_name_for_test: syn::Ident,
    fn_name_for_doctest: syn::Ident,
}

impl Parse for DocTest {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let fn_name: syn::LitStr = input.parse()?;
        let fn_name_for_test = syn::Ident::new(&fn_name.value(), fn_name.span());
        let fn_name_for_doctest =
            syn::Ident::new(&format!("doctest_{}", fn_name.value()), fn_name.span());
        input.parse::<syn::Token![,]>()?;
        let toylang_input: syn::LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let expected_rust_output: syn::LitStr = input.parse()?;

        Ok(DocTest {
            fn_name,
            toylang_input,
            expected_rust_output,
            fn_name_for_test,
            fn_name_for_doctest,
        })
    }
}

struct Example {
    toylang_input: syn::LitStr,
    expected_rust_output: syn::LitStr,
}

impl Parse for Example {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let toylang_input: syn::LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let expected_rust_output: syn::LitStr = input.parse()?;
        Ok(Example {
            toylang_input,
            expected_rust_output,
        })
    }
}

struct Example4 {
    index: syn::LitInt,
}

impl Parse for Example4 {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let index: syn::LitInt = input.parse()?;
        Ok(Example4 { index })
    }
}

lazy_static::lazy_static! {
    static ref ALL_TESTS: toylang_common::ExampleTests = toylang_common::ExampleTests::new();
}

#[proc_macro]
pub fn generate_doctest(input: TokenStream) -> TokenStream {
    let DocTest {
        fn_name: _,
        toylang_input,
        expected_rust_output,
        fn_name_for_test: _,
        fn_name_for_doctest,
    } = parse_macro_input!(input as DocTest);
    let output = quote! {
        //#[doc = concat!("Toylang: ",stringify!(#toylang_input))]
        #[doc = "```toylang"]
        #[doc = #toylang_input]
        #[doc = "```"]
        #[doc = "generates rust code:"]
        //#[doc = stringify!(#expected_rust_output)]
        #[doc = "```rust"]
        #[doc = #expected_rust_output]
        #[doc = "```"]
        fn #fn_name_for_doctest() {
            println!("testy");
        }
    };
    output.into()
}

#[proc_macro]
pub fn generate_test(input: TokenStream) -> TokenStream {
    let DocTest {
        fn_name: _,
        toylang_input,
        expected_rust_output,
        fn_name_for_test,
        fn_name_for_doctest: _,
    } = parse_macro_input!(input as DocTest);
    //note: "test_pass_single_scenario" is defined in the toylang integration tests, not needed here
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
        fn #fn_name_for_test() {
            test_pass_single_scenario(vec![#toylang_input, #expected_rust_output]);
        }
    };
    output.into()
}

#[proc_macro]
pub fn example_proc_macro(input: TokenStream) -> TokenStream {
    let Example {
        toylang_input,
        expected_rust_output,
    } = parse_macro_input!(input as Example);
    let output = quote! {
        pub fn concatenate_toy_and_rust() -> String {
            format!("{}{}", #toylang_input, #expected_rust_output)
        }
    };
    output.into()
}

#[proc_macro]
pub fn call_to_generate_doctest(input: TokenStream) -> TokenStream {
    let output = quote! {
        example_proc_macro!("left", "right");
    };
    output.into()
}

#[proc_macro]
pub fn call_to_generate_doctest2(input: TokenStream) -> TokenStream {
    let x = "left2";
    let y = "right2";
    let output = quote! {
        example_proc_macro!(#x, #y);
    };
    output.into()
}

#[proc_macro]
pub fn call_to_generate_doctest3(input: TokenStream) -> TokenStream {
    let v = vec!["left3", "right3"];
    let x = v[0];
    let y = v[1];
    let output = quote! {
        example_proc_macro!(#x, #y);
    };
    output.into()
}

#[proc_macro]
pub fn call_to_generate_doctest4(input: TokenStream) -> TokenStream {
    let Example4 { index } = parse_macro_input!(input as Example4);
    let i = index.to_string().parse::<usize>().unwrap();
    let v = vec![
        vec!["test1", "a", "b"],
        vec!["test2", "a", "b"],
        vec!["test3", "a", "b"],
        vec!["test4", "a", "b"],
    ];
    let x = v[i][0];
    let y = v[i][1];
    let output = quote! {
        example_proc_macro!(#x, #y);
    };
    output.into()
}

#[proc_macro]
pub fn call_to_generate_doctest5(input: TokenStream) -> TokenStream {
    let Example4 { index } = parse_macro_input!(input as Example4);
    let i = index.to_string().parse::<usize>().unwrap();

    //assumes that you have defined ALL_TESTS as Vec<(String, String, String)> in library
    /*
    let output = quote! {
        generate_doctest!(#(LitStr::new(ALL_TESTS.tests[#i].0, proc_macro2::Span::call_site())), #(LitStr::new(ALL_TESTS.tests[#i].1, proc_macro2::Span::call_site())), #(LitStr::new(ALL_TESTS.tests[#i].2, proc_macro2::Span::call_site())));
    };
    */
    let output = quote! {};
    dbg!(&output);
    output.into()
}

/// This one may be working!
#[proc_macro]
pub fn call_to_generate_doctest6(input: TokenStream) -> TokenStream {
    let Example4 { index } = parse_macro_input!(input as Example4);
    let i = index.to_string().parse::<usize>().unwrap();
    let fn_name = &ALL_TESTS.tests[i].0;
    let toy = &ALL_TESTS.tests[i].1;
    let rust = &ALL_TESTS.tests[i].2;
    let output = quote! {
        generate_doctest!(#fn_name,#toy,#rust);
    };
    output.into()
}
