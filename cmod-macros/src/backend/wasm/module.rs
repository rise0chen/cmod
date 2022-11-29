use proc_macro::TokenStream;

pub fn cmod_module(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
