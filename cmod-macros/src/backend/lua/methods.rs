use super::utils::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ImplItem, ImplItemMethod, ItemImpl, Stmt};

pub fn cmod_methods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemImpl);
    let lua_input = input.clone();

    input.attrs.clear();
    input.items.iter_mut().for_each(|ii| match ii {
        ImplItem::Method(md) => {
            md.attrs.clear();
        }
        _ => (),
    });

    let mut item_record: Vec<Stmt> = Vec::new();
    lua_input.items.iter().for_each(|ii| match ii {
        ImplItem::Method(md) => match inner_method_handle(md) {
            Flag::Static => {
                item_record.push(method_static(md.clone()));
            }
            Flag::Class => {
                item_record.push(method_class(md.clone()));
            }
            _ => (),
        },
        _ => (),
    });
    let mut ifn: ImplItemMethod = parse_quote!(
        fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {}
    );
    ifn.block.stmts = item_record;
    TokenStream::from(quote!(
        #input

        impl mlua::UserData for Human{
            #ifn
        }
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

pub fn method_static(input: ImplItemMethod) -> Stmt {
    let lua_input = input.clone();
    let function = Function::parse_impl_fn(lua_input);
    let Function {
        name,
        asy,
        input: inp,
        args,
        ret: _,
    } = function;
    let name_str = name.to_string();
    if asy {
        parse_quote!(
            methods.add_async_function(#name_str,|lua,#inp|Self::#name(#args).await.map_err(cmod::ffi::lua::map_err));
        )
    } else {
        parse_quote!(
            methods.add_function(#name_str,|lua,#inp|Self::#name(#args).map_err(cmod::ffi::lua::map_err));
        )
    }
}

pub fn method_class(input: ImplItemMethod) -> Stmt {
    let lua_input = input.clone();
    let function = Function::parse_impl_fn(lua_input);
    let Function {
        name,
        asy,
        input: inp,
        args,
        ret: _,
    } = function;
    let name_str = name.to_string();
    if asy {
        parse_quote!(
            methods.add_async_method(#name_str,|lua,this,#inp|this.#name(#args).await.map_err(cmod::ffi::lua::map_err));
        )
    } else {
        parse_quote!(
            methods.add_method(#name_str,|lua,this,#inp|this.#name(#args).map_err(cmod::ffi::lua::map_err));
        )
    }
}
