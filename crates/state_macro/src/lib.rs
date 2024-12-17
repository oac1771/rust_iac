use quote::quote;

#[proc_macro_attribute]
pub fn foo(_attrs: proc_macro::TokenStream, _input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let res = quote!{};

    res.into()
}