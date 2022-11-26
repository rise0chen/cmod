use super::utils::Utils;
use proc_macro::TokenStream;
use proc_macro2::Span;
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
                if ifn
                    .attrs
                    .iter()
                    .any(|attr| attr.path.segments.last().unwrap().ident == Ident::new("function", Span::call_site()))
                {
                    ident_record.push(ifn.sig.ident.clone());
                }
            }
            _ => (),
        });
    }
    let mut add_func = ident_record
        .into_iter()
        .map(|ident| {
            let ident = Ident::rename(ident);
            let semi: Stmt = parse_quote!(
                m.add_function(pyo3::wrap_pyfunction!(#ident, m)?)?;
            );
            return semi;
        })
        .collect::<Vec<Stmt>>();
    add_func.push(parse_quote!(
        return father.add_submodule(m);
    ));
    if let Some(ref mut ct) = input.content {
        let mut ifn: ItemFn = parse_quote!(
            pub fn #after_name(py: pyo3::Python<'_>, father: &pyo3::types::PyModule) -> pyo3::PyResult<()>{
                let m = pyo3::types::PyModule::new(py, #name_str)?;
            }
        );
        ifn.block.stmts.append(&mut add_func);
        ct.1.push(Item::Fn(ifn));
    }
    TokenStream::from(quote!(
        #input
    ))
}
