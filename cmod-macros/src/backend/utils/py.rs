use proc_macro2::{TokenStream,Span};
use syn::{parse_macro_input,Expr,Ident,ItemFn,ImplItemMethod,Local,Pat,Stmt,Token,token::Comma,FnArg,punctuated::Punctuated};

pub trait Utils{
    fn rename(before: Ident) -> Ident;
    fn rename_module(before: Ident) -> Ident;
}

impl Utils for Ident{
    fn rename(before: Ident) -> Ident{
        return Ident::new(format!("py_{}",stringify!(name)).as_str(),Span::call_site());
    }

    fn rename_module(before: Ident) -> Ident{
        return Ident::new(format!("py_module_{}",stringify!(name)).as_str(),Span::call_site());
    }
}

pub struct Function{
    pub name: Ident,
    pub asy: bool,
    pub input: Punctuated<FnArg,Comma>,
}

impl Function{
    pub fn parse_fn(input:ItemFn) -> Self{
        Self{
            name: input.sig.ident,
            asy: input.sig.asyncness.is_some(),
            input: input.sig.inputs
        }
    }

    pub fn parse_impl_fn(input:ImplItemMethod) -> Self{
        Self{    
            name: input.sig.ident,
            asy: input.sig.asyncness.is_some(),
            input: input.sig.inputs
        }
    }
}

