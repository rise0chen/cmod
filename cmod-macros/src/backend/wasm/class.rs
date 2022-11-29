use proc_macro::TokenStream;

pub fn cmod_class(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = input.ident.clone();

    TokenStream::from(quote!(
        #[wasm_bindgen(js_classc = #name)]
        #input
    ))
}
