mod attribute;
mod definition;
mod provider_definition;
mod provider_implementation;
mod resource_definition;
mod resource_implementation;

use crate::definition::Definition;
use syn::{parse2, ItemMod};

#[proc_macro_attribute]
pub fn provider(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_mod = match parse2::<ItemMod>(input.into()) {
        Ok(item_mod) => item_mod,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let def = match Definition::try_from(item_mod) {
        Ok(def) => def,
        Err(err) => return err.to_compile_error().into(),
    };

    def.expand().into()
}
