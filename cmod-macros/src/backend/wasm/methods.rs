use super::utils::*;
use proc_macro::TokenStream;
use quote::quote;
use std::mem;
use syn::{parse_macro_input, parse_quote, Ident, ImplItem, ImplItemMethod, ItemImpl};

pub fn cmod_methods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemImpl);
    let mut wasm_input = input.clone();

    input.attrs.clear();
    input.items.iter_mut().for_each(|ii| match ii {
        ImplItem::Method(md) => {
            md.attrs.clear();
        }
        _ => (),
    });

    let mut item_record: Vec<ImplItem> = Vec::new();
    wasm_input.items.iter().for_each(|ii| match ii {
        ImplItem::Method(md) => match inner_method_handle(md) {
            Flag::Static => {
                item_record.push(ImplItem::from(method_static(md.clone())));
            }
            Flag::Class => {
                item_record.push(ImplItem::from(method_class(md.clone())));
            }
            _ => (),
        },
        _ => (),
    });
    {
        let item = &mut wasm_input.items;
        let _ = mem::replace(item, item_record);
    }
    let name = *input.self_ty.clone();
    TokenStream::from(quote!(
        #input

        #[wasm_bindgen(js_class = #name)]
        #wasm_input
    ))
}

enum Flag {
    Empty,
    Static,
    Class,
}

fn inner_method_handle(inner_method: &ImplItemMethod) -> Flag {
    for p in inner_method.attrs.iter() {
        if p.path.is_ident("staticmethod") {
            return Flag::Static;
        } else if p.path.is_ident("classmethod") {
            return Flag::Class;
        }
    }
    return Flag::Empty;
}

pub fn method_static(input: ImplItemMethod) -> ImplItemMethod {
    let wasm_input = input.clone();
    let function = Function::parse_impl_fn(wasm_input);
    let Function {
        name,
        asy,
        input: inp,
        args,
        ret,
        map_ret
    } = function;
    let after_name = Ident::rename(name.clone());
    if asy {
        if name == "new"{
            parse_quote!(
                #[wasm_bindgen(js_name = #name, constructor)]
                pub async fn #after_name(#inp)#ret{
                    Self::#name(#args).await.map_err(cmod::ffi::wasm::map_err)#map_ret
                }
            )
        }else{
            parse_quote!(
                #[wasm_bindgen(js_name = #name)]
                pub async fn #after_name(#inp)#ret{
                    Self::#name(#args).await.map_err(cmod::ffi::wasm::map_err)#map_ret
                }
            )
        }
    } else { 
        if name == "new"{
            parse_quote!(
                #[wasm_bindgen(js_name = #name, constructor)]
                pub fn #after_name(#inp)#ret{
                    Self::#name(#args).map_err(cmod::ffi::wasm::map_err)#map_ret
                }
            )
        }else{
            parse_quote!(
                #[wasm_bindgen(js_name = #name)]
                pub fn #after_name(#inp)#ret{
                    Self::#name(#args).map_err(cmod::ffi::wasm::map_err)#map_ret
                }
            )
        }
    }
}

pub fn method_class(input: ImplItemMethod) -> ImplItemMethod {
    let wasm_input = input.clone();
    let function = Function::parse_impl_fn(wasm_input);
    let Function {
        name,
        asy,
        input: inp,
        args,
        ret,
        map_ret
    } = function;
    let after_name = Ident::rename(name.clone());
    if asy {
        parse_quote!(
            #[wasm_bindgen(js_name = #name)]
            pub async fn #after_name(#inp)#ret{
                self.#name(#args).await.map_err(cmod::ffi::wasm::map_err)#map_ret
            }
        )
    } else {
        parse_quote!(
            #[wasm_bindgen(js_name = #name)]
            pub fn #after_name(#inp)#ret{
                self.#name(#args).map_err(cmod::ffi::wasm::map_err)#map_ret
            }
        )
    }
}
