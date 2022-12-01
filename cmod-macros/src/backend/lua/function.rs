use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input,ItemFn,Ident};
use super::utils::*;

pub fn cmod_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let py_input = input.clone();
    let function = Function::parse_fn(py_input);
    let Function {
        name,
        asy,
        input: inp,
        args,
        ret,
    } = function;
    let after_name = Ident::rename(name.clone());
    if asy {
        TokenStream::from(quote!(
            #input

            fn #after_name<'lua>(lua: &'lua mlua::Lua, #inp)#ret{
                #name(#args).await.map_err(cmod::ffi::lua::map_err).map(|x|x.into())
            }
        ))
    } else {
        TokenStream::from(quote!(
            #input

            fn #after_name<'lua>(lua: &'lua mlua::Lua, #inp)#ret{
                #name(#args).map_err(cmod::ffi::lua::map_err).map(|x|x.into())
            }
        ))
    }
}
