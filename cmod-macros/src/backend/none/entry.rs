use proc_macro::TokenStream;

pub fn cmod(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
