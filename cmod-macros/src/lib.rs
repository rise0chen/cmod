extern crate proc_macro;

mod backend;
use proc_macro::TokenStream;

// A proc macro used to implement modules.
#[proc_macro_attribute]
pub fn cmod(attr: TokenStream, input: TokenStream) -> TokenStream {
    backend::cmod(attr, input)
}

#[proc_macro_attribute]
pub fn function(attr:TokenStream, input:TokenStream) -> TokenStream {
    backend::cmod_function(attr, input)
}

#[proc_macro_attribute]
pub fn class(attr:TokenStream, input:TokenStream) -> TokenStream {
    backend::cmod_class(attr, input)
}

#[proc_macro_attribute]
pub fn methods(attr:TokenStream, input:TokenStream) -> TokenStream {
    backend::cmod_methods(attr, input)
}

#[proc_macro_attribute]
pub fn module(attr:TokenStream, input:TokenStream) -> TokenStream {
    backend::cmod_module(attr, input)
}

#[proc_macro_attribute]
pub fn tags(_attr:TokenStream, input:TokenStream) -> TokenStream {
    input
}