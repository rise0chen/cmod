use super::utils::Utils;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Ident, Item, ItemFn, ItemMod, Stmt};

pub fn cmod_module(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemMod);
    let name = input.ident.clone();
    let name_str = name.to_string();
    let after_name = Ident::rename_module(name.clone());
    let item = input.content.clone();
    let mut ident_record = Vec::new();
    if let Some((_b, it)) = item {
        it.into_iter().for_each(|i| match i {
            Item::Fn(ifn) => {
                if ifn.attrs.iter().any(|attr| attr.path.segments.last().unwrap().ident == "function") {
                    ident_record.push(ifn.sig.ident.clone());
                }
            }
            _ => (),
        });
    }
    let mut add_func = ident_record
        .into_iter()
        .map(|ident| {
            let name = Ident::rename(ident.clone());
            let ident_str = ident.clone().to_string();
            let semi: Stmt = parse_quote!(
                m.set(#ident_str,lua.create_function(#name)?)?;
            );
            return semi;
        })
        .collect::<Vec<Stmt>>();
    add_func.push(parse_quote!(
        return father.set(#name_str,m);
    ));
    if let Some(ref mut ct) = input.content {
        let mut ifn: ItemFn = parse_quote!(
            pub fn #after_name(lua:&'lua mlua::Lua, father: &mlua::Table) -> mlua::Result<()>{
                let m = lua.create_table()?;
            }
        );
        ifn.block.stmts.append(&mut add_func);
        ct.1.push(Item::Fn(ifn));
    }
    TokenStream::from(quote!(
        #input
    ))
}
