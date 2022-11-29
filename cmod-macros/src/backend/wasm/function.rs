use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input,ItemFn};

pub fn cmod_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let name = input.sig.ident.clone();

    TokenStream::from(quote!(
        #[wasm_bindgen(js_name = #name)]
        #input
    ))
}
