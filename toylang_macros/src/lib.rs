use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Error, Parse, ParseStream};
use syn::{parse_macro_input, token::Struct, DeriveInput, LitStr};

//use syn::{parse_macro_input, Ident, LitStr};
//use proc_macro::{Span, TokenStream};

struct DocTest {
    fn_name: syn::Ident,
    arg1: syn::LitStr,
    arg2: syn::LitStr,
}

impl Parse for DocTest {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let fn_name = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let arg1 = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let arg2 = input.parse()?;
        Ok(DocTest {
            fn_name,
            arg1,
            arg2,
        })
    }
}

#[proc_macro]
pub fn example_proc_macro(input: TokenStream) -> TokenStream {
    let DocTest {
        fn_name,
        arg1,
        arg2,
    } = parse_macro_input!(input as DocTest);
    //dbg!(input_str);
    let output = quote! {
        pub fn #fn_name() -> String {
            format!("{}{}", #arg1, #arg2)
        }
    };
    output.into()
}

/*
#[proc_macro]
pub fn example_proc_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens as a string literal
    let input_str = parse_macro_input!(input as LitStr);
    dbg!(&input_str);

    // Extract the function name and the two string literals
    //let val = input_str.value();
    //dbg!(&val);
    //let input_parts: Vec<_> = val.split(',').collect();
    //dbg!(&input_parts);
    //let fn_name = input_parts[0].trim();
    //dbg!(&fn_name);
    //let name: Ident = Ident::new(&val, Span::call_site().into());

    //let greeting = input_parts[1].trim();
    //let subject = input_parts[2].trim();

    // Generate the function code
    let output = quote! {
        //pub fn #name() -> String {
        //    "testing".to_string()
            //println!("{}{}!", #greeting, #subject);
        //}
    };
    dbg!(&output);

    output.into()
}
*/
