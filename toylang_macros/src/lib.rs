extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(item: TokenStream) -> TokenStream {
    println!("###{:?}", item);
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
