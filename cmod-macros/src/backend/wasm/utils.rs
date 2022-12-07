use proc_macro::TokenStream;
use proc_macro2::Span;
use std::collections::HashSet as Set;
use syn::{parse_macro_input, parse_quote, Expr, FnArg, Ident, ImplItemMethod, ItemFn, Pat, ReturnType, Type, Stmt};
pub trait Utils {
    fn rename(before: Ident) -> Ident;
    fn rename_module(before: Ident) -> Ident;
}

impl Utils for Ident {
    fn rename(before: Ident) -> Ident {
        return Ident::new(format!("wasm_{}", before.to_string()).as_str(), Span::call_site());
    }

    fn rename_module(before: Ident) -> Ident {
        return Ident::new(format!("wasm_module_{}", before.to_string()).as_str(), Span::call_site());
    }
}

pub struct Function {
}

impl Function {
    pub fn parse_fn(mut input: ItemFn) -> ItemFn {
        let mut map_ret = false;
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
                        Result<cmod::ffi::wasm::ToFfi#t>
                    );
                    map_ret = true;
                }
            }
        }
        let mut args:Vec<Stmt> = Vec::new();
        input.sig.inputs.iter_mut().for_each(|i| {
            if let FnArg::Typed(t) = i {
                let pt = *t.pat.clone();
                if set.1.contains(&pt) {
                    let ty = *t.ty.clone();
                    match &ty {
                        Type::Path(tp) => {
                            let ps = tp.path.segments.last().unwrap();
                            if ps.ident == "Option" {
                                let ty = ps.arguments.clone();
                                *(t.ty) = parse_quote!(Option<cmod::ffi::wasm::FromFfi#ty>);
                                args.push(parse_quote!(let #pt = #pt.map(|x| x.into_inner());));
                            } else {
                                *(t.ty) = parse_quote!(cmod::ffi::wasm::FromFfi<#ty>);
                                args.push(parse_quote!(let #pt = #pt.into_inner();));
                            }
                        }
                        _ => (),
                    }
                }
            }
        });
        args.append(&mut input.block.stmts.clone());
        if map_ret{
            args.iter_mut().for_each(|stmt|{
                match stmt{
                    Stmt::Semi(expr,_) => {
                        if let Expr::Return(ret) = expr{
                            if let Some(ref mut exp) = ret.expr{
                                let p = *exp.clone();
                                *exp = parse_quote!(#p.map(cmod::ffi::wasm::ToFfi::from));
                            }
                        }
                    }
                    _=>()
                }
            })
        }
        input.block.stmts = args;
        input
    }

    pub fn parse_impl_fn(mut input: ImplItemMethod) -> ImplItemMethod {
        let mut map_ret = false;
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
                        Result<cmod::ffi::wasm::ToFfi#t>
                    );
                    map_ret = true;
                }
            }
        }
        let mut args:Vec<Stmt> = Vec::new();
        input.sig.inputs.iter_mut().for_each(|i| {
            if let FnArg::Typed(t) = i {
                let pt = *t.pat.clone();
                if set.1.contains(&pt) {
                    let ty = *t.ty.clone();
                    match &ty {
                        Type::Path(tp) => {
                            let ps = tp.path.segments.last().unwrap();
                            if ps.ident == "Option" {
                                let ty = ps.arguments.clone();
                                *(t.ty) = parse_quote!(Option<cmod::ffi::wasm::FromFfi#ty>);
                                args.push(parse_quote!(let #pt = #pt.map(|x| x.into_inner());));
                            } else {
                                *(t.ty) = parse_quote!(cmod::ffi::wasm::FromFfi<#ty>);
                                args.push(parse_quote!(let #pt = #pt.into_inner();));
                            }
                        }
                        _ => (),
                    }
                }
            }
        });
        args.append(&mut input.block.stmts.clone());
        if map_ret{
            args.iter_mut().for_each(|stmt|{
                match stmt{
                    Stmt::Semi(expr,_) => {
                        if let Expr::Return(ret) = expr{
                            if let Some(ref mut exp) = ret.expr{
                                let p = *exp.clone();
                                *exp = parse_quote!(#p.map(cmod::ffi::wasm::ToFfi::from));
                            }
                        }
                    }
                    _=>()
                }
            })
        }
        input.block.stmts = args;
        input
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
