use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_quote,ItemMod, parse_macro_input};

pub fn cmod(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemMod);
    if let Some((_,ref mut items)) = input.content{
        items.insert(0,parse_quote!(
            use wasm_bindgen::prelude::*;
        ));
    }
    TokenStream::from(quote!(
        #input
    ))
}
