use proc_macro::TokenStream;
use proc_macro2::Span;
use std::collections::HashSet as Set;
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, Expr, FnArg, Ident, ImplItemMethod, ItemFn, Pat, ReturnType, Type };
pub trait Utils {
    fn rename(before: Ident) -> Ident;
    fn rename_module(before: Ident) -> Ident;
}

impl Utils for Ident {
    fn rename(before: Ident) -> Ident {
        return Ident::new(format!("lua_{}", before.to_string()).as_str(), Span::call_site());
    }

    fn rename_module(before: Ident) -> Ident {
        return Ident::new(format!("lua_module_{}", before.to_string()).as_str(), Span::call_site());
    }
}

pub struct Function {
    pub name: Ident,
    pub asy: bool,
    pub input: Punctuated<FnArg, Comma>,
    pub args: Punctuated<Expr, Comma>,
    pub ret: ReturnType,
}

impl Function {
    pub fn parse_fn(mut input: ItemFn) -> Self {
        let mut set: (bool, Set<Pat>) = (false, Set::new());
        input.attrs.iter().for_each(|attr| {
            if attr.path.segments.last().unwrap().ident == "tags" {
                let token = TokenStream::from(attr.tokens.clone());
                let _ = Self::args_detect(token, &mut set);
            }
        });
        if let ReturnType::Type(_r, ref mut ty) = input.sig.output {
            if let Type::Path(tp) = (**ty).clone() {
                let t = tp.path.segments.last().unwrap().arguments.clone();
                if set.0 {
                    **ty = parse_quote!(
                        mlua::Result<cmod::ffi::py::ToFfi#t>
                    )
                } else {
                    **ty = parse_quote!(
                        mlua::Result#t
                    )
                }
            }
        }
        let mut args = Punctuated::new();
        input.sig.inputs.iter_mut().for_each(|i| {
            if let FnArg::Typed(t) = i {
                let mut pat:Punctuated<Pat,Comma> = Punctuated::new();
                match *t.pat{
                    Pat::Tuple(ref pt) =>{
                        pat = pt.elems.clone();
                    },
                    _ => ()
                }
                match *t.ty{
                    Type::Tuple(ref mut ty) => {
                        ty.elems.iter_mut().zip(pat.into_iter()).for_each(|(t,p)|{
                            if set.1.contains(&p){
                                let ty = t.clone();
                                match &ty {
                                    Type::Path(tp) => {
                                        let ps = tp.path.segments.last().unwrap();
                                        if ps.ident == "Option" {
                                            let ty = ps.arguments.clone();
                                            *t = parse_quote!(Option<cmod::ffi::py::FromFfi#ty>);
                                            args.push(parse_quote!(#p.map(|x| x.into_inner())));
                                        } else {
                                            *t = parse_quote!(cmod::ffi::py::FromFfi<#ty>);
                                            args.push(parse_quote!(#p.into_inner()));
                                        }
                                    }
                                    _ => (),
                                }
                            } else {
                                args.push(parse_quote!(#p));
                            }
                        })
                    },
                    _ => ()
                }
            }
        });
        Self {
            name: input.sig.ident,
            asy: input.sig.asyncness.is_some(),
            input: input.sig.inputs,
            args,
            ret: input.sig.output,
        }
    }

    pub fn parse_impl_fn(mut input: ImplItemMethod) -> Self {
        let mut set: (bool, Set<Pat>) = (false, Set::new());
        input.attrs.iter().for_each(|attr| {
            if attr.path.segments.last().unwrap().ident == "tags" {
                let token = TokenStream::from(attr.tokens.clone());
                let _ = Self::args_detect(token, &mut set);
            }
        });
        if let ReturnType::Type(_r, ref mut ty) = input.sig.output {
            if let Type::Path(tp) = (**ty).clone() {
                let t = tp.path.segments.last().unwrap().arguments.clone();
                if set.0 {
                    **ty = parse_quote!(
                        mlua::Result<cmod::ffi::py::ToFfi#t>
                    )
                } else {
                    **ty = parse_quote!(
                        mlua::Result#t
                    )
                }
            }
        }
        let mut args = Punctuated::new();
        input.sig.inputs.iter_mut().for_each(|i| {
            if let FnArg::Typed(t) = i {
                let mut pat:Punctuated<Pat,Comma> = Punctuated::new();
                match *t.pat{
                    Pat::Tuple(ref pt) =>{
                        pat = pt.elems.clone();
                    },
                    _ => ()
                }
                match *t.ty{
                    Type::Tuple(ref mut ty) => {
                        ty.elems.iter_mut().zip(pat.into_iter()).for_each(|(t,p)|{
                            if set.1.contains(&p){
                                let ty = t.clone();
                                match &ty {
                                    Type::Path(tp) => {
                                        let ps = tp.path.segments.last().unwrap();
                                        if ps.ident == "Option" {
                                            let ty = ps.arguments.clone();
                                            *t = parse_quote!(Option<cmod::ffi::py::FromFfi#ty>);
                                            args.push(parse_quote!(#p.map(|x| x.into_inner())));
                                        } else {
                                            *t = parse_quote!(cmod::ffi::py::FromFfi<#ty>);
                                            args.push(parse_quote!(#p.into_inner()));
                                        }
                                    }
                                    _ => (),
                                }
                            } else {
                                args.push(parse_quote!(#p));
                            }
                        })
                    },
                    _ => ()
                }
            }
        });
        Self {
            name: input.sig.ident,
            asy: input.sig.asyncness.is_some(),
            input: input.sig.inputs,
            args,
            ret: input.sig.output,
        }
    }

    fn args_detect(input: TokenStream, outset: &mut (bool, Set<Pat>)) -> TokenStream {
        let pat = parse_macro_input!(input as Pat);
        if let Pat::Tuple(et) = pat {
            et.elems.iter().for_each(|e| match e {
                Pat::Ident(pi) => {
                    if pi.ident == "ret" {
                        outset.0 = true;
                    }
                }
                Pat::TupleStruct(pts) => {
                    if pts.path.segments.last().unwrap().ident == "args" {
                        pts.pat.elems.iter().for_each(|p| {
                            outset.1.insert(p.clone());
                        });
                    }
                }
                _ => (),
            })
        }
        TokenStream::new()
    }
}