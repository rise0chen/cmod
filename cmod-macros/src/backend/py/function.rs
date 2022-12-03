use super::utils::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

pub fn cmod_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let py_input = input.clone();
    let function = Function::parse_fn(py_input);
    let Function {
        name,
        asy,
        input: inp,
        args,
        ret,
        map_ret,
    } = function;
    let after_name = Ident::rename(name.clone());
    let name_str = name.to_string();
    if asy {
        TokenStream::from(quote!(
            #input

            #[pyo3::pyfunction]
            #[pyo3(name = #name_str)]
            fn #after_name(py: pyo3::Python, #inp)#ret{
                cmod::ffi::py::block_on(py, async move{
                    #name(#args).await.map_err(cmod::ffi::py::map_err)#map_ret
                })
            }
        ))
    } else {
        TokenStream::from(quote!(
            #input

            #[pyo3::pyfunction]
            #[pyo3(name = #name_str)]
            fn #after_name(py: pyo3::Python, #inp)#ret{
                #name(#args).map_err(cmod::ffi::py::map_err)#map_ret
            }
        ))
    }
}
