use super::utils::*;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Ident, Item, ItemFn, ItemMod, Stmt};

pub fn cmod(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemMod);
    let name = input.ident.clone();
    let name_str = name.to_string();
    let lua_table_name = Ident::new(format!("lua_table_{}", name).as_str(), Span::call_site());
    let lua_module_name = Ident::rename_module(name.clone());
    let mut ifn: ItemFn = parse_quote!(
        pub fn #lua_table_name(lua:&mlua::Lua) -> mlua::Result<mlua::Table>{
            let m = lua.create_table()?;
        }
    );
    let ifn_module: ItemFn = parse_quote!(
        #[mlua::lua_module(name = #name_str)]
        pub fn #lua_module_name(lua:&mlua::Lua) -> mlua::Result<mlua::Table>{
            #lua_table_name(lua)
        }
    );

    let item = input.content.clone();
    let mut item_record: Vec<Stmt> = Vec::new();
    if let Some((_b, it)) = item {
        it.into_iter().for_each(|i| match i {
            Item::Fn(ifn) => {
                if ifn.attrs.iter().any(|a| a.meta.path().segments.last().unwrap().ident == "function") {
                    let name = ifn.sig.ident.clone();
                    let after_name = Ident::rename(name.clone());
                    let name_str = name.to_string();
                    let semi = if ifn.sig.asyncness.is_some() {
                        parse_quote!(
                            m.set(#name_str,lua.create_async_function(#after_name)?)?;
                        )
                    } else {
                        parse_quote!(
                            m.set(#name_str,lua.create_function(#after_name)?)?;
                        )
                    };
                    item_record.push(semi);
                }
            }
            Item::Mod(imd) => {
                if imd.attrs.iter().any(|a| a.meta.path().segments.last().unwrap().ident == "module") {
                    let name = imd.ident.clone();
                    let after_name = Ident::rename_module(imd.ident.clone());
                    let semi = parse_quote!(
                        #name::#after_name(lua,&m)?;
                    );
                    item_record.push(semi);
                }
            }
            Item::Struct(ist) => {
                if ist.attrs.iter().any(|a| a.meta.path().segments.last().unwrap().ident == "class") {
                    let name = ist.ident.clone();
                    let name_str = name.to_string();
                    let semi = parse_quote!(
                        m.set(#name_str,lua.create_proxy::<#name>()?)?;
                    );
                    item_record.push(semi);
                }
            }
            _ => (),
        });
    }
    item_record.push(parse_quote!(
        return Ok(m);
    ));
    ifn.block.stmts.append(&mut item_record);

    if let Some((_b, ref mut ct)) = input.content {
        ct.push(Item::Fn(ifn));
        ct.push(Item::Fn(ifn_module));
    }

    TokenStream::from(quote!(
        #input
    ))
}
