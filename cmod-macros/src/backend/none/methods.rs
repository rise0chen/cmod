use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ItemImpl};

pub fn cmod_methods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemImpl);
    input.attrs.clear();
    input.items.iter_mut().for_each(|ii| {
        if let ImplItem::Fn(md) = ii {
            md.attrs.clear();
        }
    });
    TokenStream::from(quote!(
        #input
    ))
}
