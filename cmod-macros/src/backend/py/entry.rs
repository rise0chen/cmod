use super::utils::Utils;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::mem;
use syn::{parse_macro_input, parse_quote, Ident, Item, ItemFn, ItemMod, Stmt};

pub fn cmod(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemMod);
    let name = input.ident.clone();
    let name_str = name.to_string();
    let after_name = Ident::rename_module(name.clone());
    let mut ifn: ItemFn = parse_quote!(
        #[pyo3::pymodule]
        #[pyo3(name = #name_str)]
        fn #after_name(py: pyo3::Python<'_>, m: &pyo3::types::PyModule) -> pyo3::PyResult<()>{
            Ok(())
        }
    );

    let item = input.content.clone();
    let mut item_record: Vec<Stmt> = Vec::new();
    if let Some((_b, it)) = item {
        it.into_iter().for_each(|i| match i {
            Item::Fn(ifn) => {
                if ifn
                    .attrs
                    .iter()
                    .any(|a| a.path.segments.last().unwrap().ident == Ident::new("function", Span::call_site()))
                {
                    let name = Ident::rename(ifn.sig.ident.clone());
                    let semi = parse_quote!(
                        m.add_function(pyo3::wrap_pyfunction!(#name,m)?)?;
                    );
                    item_record.push(semi);
                }
            }
            Item::Mod(imd) => {
                if imd
                    .attrs
                    .iter()
                    .any(|a| a.path.segments.last().unwrap().ident == Ident::new("module", Span::call_site()))
                {
                    let name = imd.ident.clone();
                    let after_name = Ident::rename_module(imd.ident.clone());
                    let semi = parse_quote!(
                        #name::#after_name(py,m)?;
                    );
                    item_record.push(semi);
                }
            }
            Item::Struct(ist) => {
                if ist
                    .attrs
                    .iter()
                    .any(|a| a.path.segments.last().unwrap().ident == Ident::new("class", Span::call_site()))
                {
                    let name = ist.ident.clone();
                    let semi = parse_quote!(
                        m.add_class::<#name>()?;
                    );
                    item_record.push(semi);
                }
            }
            _ => (),
        });
    }
    item_record.push(parse_quote!(
        return Ok(());
    ));
    {
        let stmts = &mut ifn.block.stmts;
        let _ = mem::replace(stmts, item_record);
    }

    if let Some((_b, ref mut ct)) = input.content {
        ct.push(Item::Fn(ifn));
    }

    TokenStream::from(quote!(
        #input
    ))
}
