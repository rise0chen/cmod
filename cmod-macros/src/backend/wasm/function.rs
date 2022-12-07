use super::utils::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn cmod_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let wasm_input = input.clone();
    let function = Function::parse_fn(wasm_input);
    TokenStream::from(quote!(
        #[wasm_bindgen]
        #function
    ))
}
