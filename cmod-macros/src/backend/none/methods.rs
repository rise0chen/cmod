use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ItemImpl};

pub fn cmod_methods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemImpl);
    input.attrs.clear();
    input.items.iter_mut().for_each(|ii| match ii {
        ImplItem::Method(md) => {
            md.attrs.clear();
        }
        _ => (),
    });
    TokenStream::from(quote!(
        #input
    ))
}
