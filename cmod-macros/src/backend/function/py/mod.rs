use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input,Ident,ItemFn};
use super::super::utils::py::*;

pub fn cmod_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let py_input = input.clone();
    let function = Function::parse_fn(py_input);
    let name = function.name;
    let inp = function.input;
    let after_name = Ident::rename(name.clone());
    if function.asy{
        TokenStream::from(quote!(
            #input

            #[pyo3::pyfunction]
            #[pyo3(name = stringify!(#name))]
            fn #after_name(py: pyo3::Python, #inp) -> _{
                cmod::ffi::py::block_on(py, async move{
                    #input.await.map_err(cmod::ffi::py::map_err)
                })
            }
        ))
    }else{
        TokenStream::from(quote!(
            #input

            #[pyo3::pyfunction]
            #[pyo3(name = stringify!(#name))]
            fn #after_name(py: pyo3::Python, #inp) -> _{
                #input.map_err(cmod::ffi::py::map_err)
            }
        ))
    }
}
