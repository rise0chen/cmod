use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input,Expr,Ident,ItemFn,Local,Pat,Stmt,Token, FnArg, ReturnType, token::Comma};
use super::super::utils::py::utils;
struct Function{
    name: Ident,
    asy: bool,
    input: Punctuated<FnArg,Comma>,
}

impl Function{
    fn parse(input:ItemFn) -> Self{
        Self{
            name: input.sig.ident,
            asy: input.sig.asyncness.is_some(),
            input: input.sig.inputs
        }
    }
}

#[proc_macro_attribute]
pub fn cmod_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let py_input = input.clone();
    let function = Function::parse(py_input);
    let name = function.name;
    let inp = function.input;
    let after_name = Ident::rename(name);
    if function.asy{
        TokenStream::from(quote!(
            #input

            #[pyo3::pyfunction]
            #[pyo3(stringify!(#name))]
            fn py_#name(py: pyo3::Python, #inp) -> _{
                cmod::ffi::py::block_on(py, async move{
                    #input.await.map_err(cmod::ffi::py::map_err)
                })
            }
        ))
    }else{
        TokenStream::from(quote!(
            #input

            #[pyo3::pyfunction]
            #[pyo3(stringify!(#name))]
            fn #after_name(py: pyo3::Python, #inp) -> _{
                #input.map_err(cmod::ffi::py::map_err)
            }
        ))
    }
}
