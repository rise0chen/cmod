extern crate proc_macro;

mod backend;
use proc_macro::TokenStream;

// A proc macro used to implement modules.
#[proc_macro_attribute]
pub fn cmod(attr: TokenStream, input: TokenStream) -> TokenStream {
    backend::cmod(attr, input)
}

#[proc_macro_attribute]
pub fn cmod_function(attr:TokenStream, input:TokenStream) -> TokenStream {
    backend::cmod_function(attr, input)
}

#[proc_macro_attribute]
pub fn cmod_class(attr:TokenStream, input:TokenStream) -> TokenStream {
    backend::cmod_class(attr, input)
}

#[proc_macro_attribute]
pub fn cmod_methods(attr:TokenStream, input:TokenStream) -> TokenStream {
    backend::cmod_methods(attr, input)
}

#[proc_macro_attribute]
pub fn cmod_module(attr:TokenStream, input:TokenStream) -> TokenStream {
    backend::cmod_module(attr, input)
}