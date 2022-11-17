extern crate proc_macro;

mod backend;

use proc_macro::TokenStream;

/// A proc macro used to implement modules.
#[proc_macro_attribute]
pub fn cmod(attr: TokenStream, input: TokenStream) -> TokenStream {
    backend::impl_(attr, input)
}
