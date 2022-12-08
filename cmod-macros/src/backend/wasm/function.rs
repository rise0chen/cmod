use super::utils::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

pub fn cmod_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let wasm_input = input.clone();
    let function = Function::parse_fn(wasm_input);
    let Function {
        name,
        asy,
        input: inp,
        args,
        ret,
        map_ret,
    } = function;
    let after_name = Ident::rename(name.clone());
    if asy {
        TokenStream::from(quote!(
            #input

            #[wasm_bindgen(js_name = #name)]
            pub async fn #after_name(#inp)#ret{
                #name(#args).await.map_err(cmod::ffi::wasm::map_err)#map_ret
            }
        ))
    } else {
        TokenStream::from(quote!(
            #input

            #[wasm_bindgen(js_name = #name)]
            pub fn #after_name(#inp)#ret{
                #name(#args).map_err(cmod::ffi::wasm::map_err)#map_ret
            }
        ))
    }
}
