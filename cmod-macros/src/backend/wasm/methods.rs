use super::utils::*;
use proc_macro::TokenStream;
use quote::quote;
use std::mem;
use syn::{parse_macro_input, parse_quote, ImplItem, ImplItemMethod, ItemImpl};

pub fn cmod_methods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);
    let mut wasm_input = input.clone();
    let mut item_record: Vec<ImplItem> = Vec::new();
    wasm_input.items.iter().for_each(|ii| match ii {
        ImplItem::Method(md) => {
            item_record.push(ImplItem::from(method(md.clone())));
        },
        _ => (),
    });
    {
        let item = &mut wasm_input.items;
        let _ = mem::replace(item, item_record);
    }
    let id = *input.self_ty.clone();
    TokenStream::from(quote!(
        #[wasm_bindgen(js_class = #id)]
        #wasm_input
    ))
}

pub fn method(input: ImplItemMethod) -> ImplItemMethod {
    let wasm_input = input.clone();
    let function = Function::parse_impl_fn(wasm_input);
    if input.sig.ident == "new"{
        parse_quote!(
            #[wasm_bindgen(constructor)]
            #function
        )
    }else{
        parse_quote!(
            #[wasm_bindgen]
            #function
        )
    }
}
