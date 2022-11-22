use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input,parse_quote,Ident,ItemImpl, ImplItemMethod, ImplItem, ItemStruct, ItemFn};
use super::super::utils::py::*;

pub fn cmod_class(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    TokenStream::from(quote!(
        #[pyo3::pyclass]
        #input
    ))
}

pub fn cmod_methods(_attr: TokenStream, input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as ItemImpl);
    let mut py_input = input.clone();
    let name = *py_input.self_ty.clone();
    let mut token  = Vec::new();
    py_input.items.iter().for_each(|method|{
        match method{
            &ImplItem::Method(ref inner_method) => {
                let flag = inner_method_handle(inner_method);
                match flag{
                    Flag::Static => {
                        let ret = to_static(inner_method.clone());
                        token.push(ImplItem::Method(ret));
                    },
                    Flag::Class => {
                        let ret = to_class(inner_method.clone());
                        token.push(ImplItem::Method(ret));
                    },
                    Flag::Empty => ()
                }
            }
            _=>()
        }
    });
    py_input.items.append(&mut token);
    TokenStream::from(quote!(
        #input 
        
        #py_input
    ))
}

enum Flag{
    Empty,
    Static,
    Class,
}

fn inner_method_handle(inner_method:&ImplItemMethod) -> Flag{
    for p in inner_method.attrs.iter(){
        if p.path.is_ident("staticmethod"){
            return Flag::Static;
        }else if p.path.is_ident("classmethod"){
            return Flag::Class;
        }
    }
    return Flag::Empty;
}

pub fn to_static(inner_method:ImplItemMethod) -> ImplItemMethod {
    let py_input = inner_method.clone();
    let function = Function::parse_impl_fn(py_input);
    let name = function.name;
    let inp = function.input;
    let after_name = Ident::rename(name.clone());
    if function.asy{
        parse_quote!(
            #[pyo3::pyfunction]
            #[pyo3(name = stringify!(#name))]
            fn #after_name(py: pyo3::Python, #inp) -> _{
                cmod::ffi::py::block_on(py, async move{
                    #name().await.map_err(cmod::ffi::py::map_err)
                })
            }
        )
    }else{
        parse_quote!(
            #[pyo3::pyfunction]
            #[pyo3(name = stringify!(#name))]
            fn #after_name(py: pyo3::Python, #inp) -> _{
                #name().map_err(cmod::ffi::py::map_err)
            }
        )
    }
}

fn to_class(inner_method:ImplItemMethod) -> ImplItemMethod {
    let py_input = inner_method.clone();
    let function = Function::parse_impl_fn(py_input);
    let name = function.name;
    let inp = function.input;
    let after_name = Ident::rename(name.clone());
    if function.asy{
        parse_quote!(
            #[pyo3::pyfunction]
            #[pyo3(name = stringify!(#name))]
            fn #after_name<'py>(this: pyo3::Py<Self>, py: pyo3::Python, #inp) -> _{
                cmod::ffi::py::block_on(py, async move{
                    let this: Self = this.extract(py)?
                    this.#name().await.map_err(cmod::ffi::py::map_err)
                })
            }
        )
    }else{
        parse_quote!(
            #[pyo3::pyfunction]
            #[pyo3(name = stringify!(#name))]
            fn #after_name<'py>(this: pyo3::Py<Self>, py: pyo3::Python, #inp) -> _{
                let this: Self = this.extract(py)?;
                this.#name().map_err(cmod::ffi::py::map_err)
            }
        )
    }
}