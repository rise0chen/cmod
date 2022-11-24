use proc_macro2::Span;
use syn::{Ident,ItemFn,ImplItemMethod,token::Comma,FnArg,punctuated::Punctuated,ReturnType, parse_quote,Type,Expr};

pub trait Utils{
    fn rename(before: Ident) -> Ident;
    fn rename_module(before: Ident) -> Ident;
}

impl Utils for Ident{
    fn rename(before: Ident) -> Ident{
        return Ident::new(format!("py_{}",before.to_string()).as_str(),Span::call_site());
    }

    fn rename_module(before: Ident) -> Ident{
        return Ident::new(format!("py_module_{}",before.to_string()).as_str(),Span::call_site());
    }
}

pub struct Function{
    pub name: Ident,
    pub asy: bool,
    pub input: Punctuated<FnArg,Comma>,
    pub args: Punctuated<Expr,Comma>,
    pub ret: ReturnType,
}

impl Function{
    pub fn parse_fn(mut input:ItemFn) -> Self{
        if let ReturnType::Type(_r,ref mut ty) = input.sig.output{
            if let Type::Path(tp) = (**ty).clone(){
                let t = tp.path.segments.last().unwrap().arguments.clone();
                **ty = parse_quote!(
                    pyo3::PyResult<cmod::ffi::py::ToFfi#t>
                )
            }
        }
        let mut args = Punctuated::new();
        input.sig.inputs.iter_mut().for_each(|i|{
            if let FnArg::Typed(t) = i{
                let pt = *t.pat.clone();
                args.push(parse_quote!(#pt.into_inner()));

                let ty = *t.ty.clone();
                *(t.ty) = parse_quote!(cmod::ffi::py::FromFfi<#ty>);
            }
        });
        Self{
            name: input.sig.ident,
            asy: input.sig.asyncness.is_some(),
            input: input.sig.inputs,
            args,
            ret: input.sig.output,
        }
    }

    pub fn parse_impl_fn(mut input:ImplItemMethod) -> Self{
        if let ReturnType::Type(_r,ref mut ty) = input.sig.output{
            if let Type::Path(tp) = (**ty).clone(){
                let t = tp.path.segments.last().unwrap().arguments.clone();
                **ty = parse_quote!(
                    pyo3::PyResult<cmod::ffi::py::ToFfi#t>
                )
            }
        }
        let mut args = Punctuated::new();
        input.sig.inputs.iter_mut().for_each(|i|{
            if let FnArg::Typed(t) = i{
                let pt = *t.pat.clone();
                args.push(parse_quote!(#pt.into_inner()));

                let ty = *t.ty.clone();
                *(t.ty) = parse_quote!(cmod::ffi::py::FromFfi<#ty>);
            }
        });
        Self{    
            name: input.sig.ident,
            asy: input.sig.asyncness.is_some(),
            input: input.sig.inputs,
            args,
            ret:input.sig.output,
        }
    }
}

