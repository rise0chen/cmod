use super::utils::*;
use proc_macro::TokenStream;
use quote::quote;
use std::mem;
use syn::{parse_macro_input, parse_quote, Ident, ImplItem, ImplItemMethod, ItemImpl};

pub fn cmod_methods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemImpl);
    let mut py_input = input.clone();

    input.attrs.clear();
    input.items.iter_mut().for_each(|ii| match ii {
        ImplItem::Method(md) => {
            md.attrs.clear();
        }
        _ => (),
    });

    let mut item_record: Vec<ImplItem> = Vec::new();
    py_input.items.iter().for_each(|ii| match ii {
        ImplItem::Method(md) => match inner_method_handle(md) {
            Flag::Static => {
                item_record.push(ImplItem::from(method_static(md.clone())));
            }
            Flag::Class => {
                item_record.push(ImplItem::from(method_class(md.clone())));
            }
            _ => (),
        },
        _ => (),
    });
    {
        let item = &mut py_input.items;
        let _ = mem::replace(item, item_record);
    }
    TokenStream::from(quote!(
        #input

        #[pyo3::pymethods]
        #py_input
    ))
}

enum Flag {
    Empty,
    Static,
    Class,
}

fn inner_method_handle(inner_method: &ImplItemMethod) -> Flag {
    for p in inner_method.attrs.iter() {
        if p.path.is_ident("staticmethod") {
            return Flag::Static;
        } else if p.path.is_ident("classmethod") {
            return Flag::Class;
        }
    }
    return Flag::Empty;
}

pub fn method_static(input: ImplItemMethod) -> ImplItemMethod {
    let py_input = input.clone();
    let function = Function::parse_impl_fn(py_input);
    let Function {
        name,
        asy,
        input: inp,
        args,
        ret,
        map_ret
    } = function;
    let after_name = Ident::rename(name.clone());
    let name_str = name.to_string();
    if asy {
        parse_quote!(
            #[staticmethod]
            #[pyo3(name = #name_str)]
            fn #after_name(py: pyo3::Python, #inp)#ret{
                cmod::ffi::py::block_on(py, async move{
                    Self::#name(#args).await.map_err(cmod::ffi::py::map_err)#map_ret
                })
            }
        )
    } else {
        parse_quote!(
            #[staticmethod]
            #[pyo3(name = #name_str)]
            fn #after_name(py: pyo3::Python, #inp)#ret{
                Self::#name(#args).map_err(cmod::ffi::py::map_err)#map_ret
            }
        )
    }
}

pub fn method_class(input: ImplItemMethod) -> ImplItemMethod {
    let py_input = input.clone();
    let function = Function::parse_impl_fn(py_input);
    let Function {
        name,
        asy,
        input: mut inp,
        args,
        ret,
        map_ret
    } = function;
    inp = inp.into_iter().skip(1).collect();
    let after_name = Ident::rename(name.clone());
    let name_str = name.to_string();
    if asy {
        parse_quote!(
            #[pyo3(name = #name_str)]
            fn #after_name<'py>(this:pyo3::Py<Self>,py: pyo3::Python<'py>, #inp)#ret{
                let this:Self = this.extract(py)?;
                cmod::ffi::py::block_on(py, async move{
                    this.#name(#args).await.map_err(cmod::ffi::py::map_err)#map_ret
                })
            }
        )
    } else {
        parse_quote!(
            #[pyo3(name = #name_str)]
            fn #after_name<'py>(this: pyo3::Py<Self>,py: pyo3::Python<'py>, #inp)#ret{
                let this:Self = this.extract(py)?;
                this.#name(#args).map_err(cmod::ffi::py::map_err)#map_ret
            }
        )
    }
}
