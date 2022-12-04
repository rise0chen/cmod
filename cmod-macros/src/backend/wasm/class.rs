use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

pub fn cmod_class(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = input.ident.clone();

    TokenStream::from(quote!(
        #[wasm_bindgen::prelude::wasm_bindgen(js_classc = #name)]
        #input
    ))
}
