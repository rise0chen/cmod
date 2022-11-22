use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input,parse_quote,Ident,Item,ItemFn,ItemMod, Stmt};

use crate::backend::utils::py::Utils;

pub fn cmod_module(_attr: TokenStream, input: TokenStream) -> TokenStream{
    let mut input = parse_macro_input!(input as ItemMod);
    let name = input.ident.clone();
    let after_name = Ident::rename_module(name.clone());
    let item = input.content.clone();
    let mut ident_record = Vec::new();
    if let Some((_b, it)) = item{
        it.into_iter().filter(|i|{
            match i{
                Item::Fn(ifn) =>{
                    ifn.attrs.iter().any(|attr|attr.path.is_ident("cmod_function"))
                },
                _=>false
            }
        }).for_each(|i|{
            match i{
                Item::Fn(ifn) =>{
                    ident_record.push(ifn.sig.ident.clone());
                }
                _=>()
            }
        });
    }
    let mut add_func = ident_record.into_iter().map(|ident|{
        let ident = Ident::rename(ident);
        let semi:Stmt = parse_quote!(
            m.add_function(pyo3::wrap_pyfunction!(#ident, m));
        );
        return semi;
    }).collect::<Vec<Stmt>>();
    add_func.push(parse_quote!(
        father.add_submodule(m);
    ));
    if let Some(ref mut ct) = input.content{
        let mut ifn:ItemFn = parse_quote!(
            pub fn #after_name(py: pyo3::Python<'_>, father: &pyo3::types::PyModule) -> _{
                let m = pyo3::types::PyModule::new(py, stringify!(#name))?;
            }
        );
        ifn.block.stmts.append(&mut add_func);
        ct.1.push(Item::Fn(ifn));
    }
    TokenStream::from(quote!(
        #input
    ))
}