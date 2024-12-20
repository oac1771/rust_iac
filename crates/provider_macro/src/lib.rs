mod provider_definition;

use crate::provider_definition::ProviderDefinition;
use syn::{parse2, ItemStruct};

#[proc_macro_attribute]
pub fn provider(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_struct = match parse2::<ItemStruct>(input.into()) {
        Ok(item_struct) => item_struct,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let _def = match ProviderDefinition::try_from(item_struct) {
        Ok(def) => def,
        Err(err) => return err.to_compile_error().into(),
    };

    quote::quote! {}.into()
}
