use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input,Expr,Ident,Item,Local,Pat,Stmt,Token};

#[proc_macro_attribute]
pub fn impl_(_meta: TokenStream, input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as Item);
    TokenStream::from(quote!(
        #input
    ))
}