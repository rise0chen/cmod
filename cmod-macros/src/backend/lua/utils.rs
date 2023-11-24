use proc_macro::TokenStream;
use proc_macro2::Span;
use std::collections::HashSet as Set;
use syn::{parse_quote, punctuated::Punctuated, token::Comma, Expr, FnArg, Ident, ImplItemFn, ItemFn, Meta, Pat, ReturnType, Type};
pub trait Utils {
    fn rename(before: Ident) -> Ident;
    fn rename_module(before: Ident) -> Ident;
}

impl Utils for Ident {
    fn rename(before: Ident) -> Ident {
        return Ident::new(format!("lua_{}", before).as_str(), Span::call_site());
    }

    fn rename_module(before: Ident) -> Ident {
        return Ident::new(format!("lua_module_{}", before).as_str(), Span::call_site());
    }
}

pub struct Function {
    pub name: Ident,
    pub asy: bool,
    pub input: Punctuated<FnArg, Comma>,
    pub args: Punctuated<Expr, Comma>,
    pub ret: ReturnType,
    pub map_ret: proc_macro2::TokenStream,
}

impl Function {
    pub fn parse_fn(mut input: ItemFn) -> Self {
        let mut map_ret = proc_macro2::TokenStream::default();
        let mut set: (bool, Set<Ident>) = (false, Set::new());
        input.attrs.iter().for_each(|attr| {
            if attr.meta.path().segments.last().unwrap().ident == "tags" {
                let token = attr.meta.require_list().unwrap().parse_args_with(Punctuated::parse_terminated).unwrap();
                let _ = Self::args_detect(token, &mut set);
            }
        });
        if let ReturnType::Type(_r, ref mut ty) = input.sig.output {
            if let Type::Path(tp) = (**ty).clone() {
                let t = tp.path.segments.last().unwrap().arguments.clone();
                if set.0 {
                    **ty = parse_quote!(
                        mlua::Result<cmod::ffi::lua::ToFfi #t>
                    );
                    map_ret = quote::quote!(
                        .map(cmod::ffi::lua::ToFfi::from)
                    );
                } else {
                    **ty = parse_quote!(
                        mlua::Result #t
                    );
                }
            }
        }
        let mut args = Punctuated::new();
        let mut input_pat: Punctuated<Pat, Comma> = Punctuated::new();
        let mut input_type: Punctuated<Type, Comma> = Punctuated::new();
        input.sig.inputs.iter_mut().for_each(|i| {
            if let FnArg::Typed(t) = i {
                input_pat.push(*t.pat.clone());
                let pt = if let Pat::Ident(pt) = &*t.pat { &pt.ident } else { return };
                if set.1.contains(pt) {
                    let ty = *t.ty.clone();
                    if let Type::Path(tp) = &ty {
                        let ps = tp.path.segments.last().unwrap();
                        if ps.ident == "Option" {
                            let ty = ps.arguments.clone();
                            input_type.push(parse_quote!(Option<cmod::ffi::lua::FromFfi #ty>));
                            args.push(parse_quote!(#pt.map(|x| x.into_inner())));
                        } else {
                            input_type.push(parse_quote!(cmod::ffi::lua::FromFfi<#ty>));
                            args.push(parse_quote!(#pt.into_inner()));
                        }
                    }
                } else {
                    input_type.push(*t.ty.clone());
                    args.push(parse_quote!(#pt));
                }
            }
        });
        let input_arg: Punctuated<FnArg, Comma> = parse_quote!((#input_pat):(#input_type));
        Self {
            name: input.sig.ident,
            asy: input.sig.asyncness.is_some(),
            input: input_arg,
            args,
            ret: input.sig.output,
            map_ret,
        }
    }

    pub fn parse_impl_fn(mut input: ImplItemFn) -> Self {
        let mut map_ret = proc_macro2::TokenStream::default();
        let mut set: (bool, Set<Ident>) = (false, Set::new());
        input.attrs.iter().for_each(|attr| {
            if attr.path().segments.last().unwrap().ident == "tags" {
                let token = attr.meta.require_list().unwrap().parse_args_with(Punctuated::parse_terminated).unwrap();
                let _ = Self::args_detect(token, &mut set);
            }
        });
        if let ReturnType::Type(_r, ref mut ty) = input.sig.output {
            if let Type::Path(tp) = (**ty).clone() {
                let t = tp.path.segments.last().unwrap().arguments.clone();
                if set.0 {
                    **ty = parse_quote!(
                        mlua::Result<cmod::ffi::lua::ToFfi #t>
                    );
                    map_ret = quote::quote!(
                        .map(cmod::ffi::lua::ToFfi::from)
                    );
                } else {
                    **ty = parse_quote!(
                        mlua::Result #t
                    );
                }
            }
        }
        let mut args = Punctuated::new();
        let mut input_pat: Punctuated<Pat, Comma> = Punctuated::new();
        let mut input_type: Punctuated<Type, Comma> = Punctuated::new();
        input.sig.inputs.iter_mut().for_each(|i| {
            if let FnArg::Typed(t) = i {
                input_pat.push(*t.pat.clone());
                let pt = if let Pat::Ident(pt) = &*t.pat { &pt.ident } else { return };
                if set.1.contains(pt) {
                    let ty = *t.ty.clone();
                    if let Type::Path(tp) = &ty {
                        let ps = tp.path.segments.last().unwrap();
                        if ps.ident == "Option" {
                            let ty = ps.arguments.clone();
                            input_type.push(parse_quote!(Option<cmod::ffi::lua::FromFfi #ty>));
                            args.push(parse_quote!(#pt.map(|x| x.into_inner())));
                        } else {
                            input_type.push(parse_quote!(cmod::ffi::lua::FromFfi<#ty>));
                            args.push(parse_quote!(#pt.into_inner()));
                        }
                    }
                } else {
                    input_type.push(*t.ty.clone());
                    args.push(parse_quote!(#pt));
                }
            }
        });
        let input_arg: Punctuated<FnArg, Comma> = parse_quote!((#input_pat):(#input_type));
        Self {
            name: input.sig.ident,
            asy: input.sig.asyncness.is_some(),
            input: input_arg,
            args,
            ret: input.sig.output,
            map_ret,
        }
    }

    fn args_detect(input: Punctuated<Meta, Comma>, outset: &mut (bool, Set<Ident>)) -> TokenStream {
        input.iter().for_each(|e| {
            if e.path().segments.last().unwrap().ident == "ret" {
                outset.0 = true;
            }
            if e.path().segments.last().unwrap().ident == "args" {
                let args: Punctuated<Ident, Comma> = e.require_list().unwrap().parse_args_with(Punctuated::parse_terminated).unwrap();
                args.into_iter().for_each(|p| {
                    outset.1.insert(p);
                });
            }
        });
        TokenStream::new()
    }
}
