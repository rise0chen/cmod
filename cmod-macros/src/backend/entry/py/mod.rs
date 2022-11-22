use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input,ItemMod,Ident, parse_quote, ItemFn};
use super::super::utils::py::Utils;

pub fn cmod(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemMod);
    let name = input.ident.clone();
    let after_name = Ident::rename_module(name.clone());
    let ifn:ItemFn = parse_quote!(
        #[pyo3::pymodule]
        #[pyo3(name = stringify(#name))]
        fn #after_name(py: pyo3::Python<'_>, m: &pyo3::types::PyModule) -> _ {
            Ok(())
        }
    );

    
    
    TokenStream::from(quote!(
        None
    ))
}
