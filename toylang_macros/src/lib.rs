use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Error, Parse, ParseStream};
use syn::parse_macro_input;

struct DocTest {
    fn_name: syn::LitStr,
    toylang_input: syn::LitStr,
    expected_rust_output: syn::LitStr,
    new_fn_name: syn::Ident,
}

impl Parse for DocTest {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let fn_name: syn::LitStr = input.parse()?;
        let new_fn_name = syn::Ident::new(&fn_name.value(), fn_name.span());
        input.parse::<syn::Token![,]>()?;
        let toylang_input: syn::LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let expected_rust_output: syn::LitStr = input.parse()?;

        Ok(DocTest {
            fn_name,
            toylang_input,
            expected_rust_output,
            new_fn_name,
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

#[proc_macro]
pub fn generate_doctest(input: TokenStream) -> TokenStream {
    let DocTest {
        fn_name: _,
        toylang_input,
        expected_rust_output,
        new_fn_name,
    } = parse_macro_input!(input as DocTest);
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
        fn #new_fn_name() {
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
    let v = vec![vec!["left4", "right4"], vec!["left4b", "right4b"]];
    let x = v[i][0];
    let y = v[i][1];
    let output = quote! {
        example_proc_macro!(#x, #y);
    };
    output.into()
}
