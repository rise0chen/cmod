use proc_macro2::TokenStream;
use syn::{parse_macro_input,Expr,Ident,Item,Local,Pat,Stmt,Token};

pub trait utils<T>{
    fn rename(before: Ident) -> Ident;
}

impl utils for Ident{
    fn rename(before: Ident) -> Ident{
        return Ident::new(format!("py_{}",stringify!(name)).as_str(),Span::call_site());
    }

}

