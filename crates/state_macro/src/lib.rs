mod resource;
mod state_attribute;
mod state_definition;
mod state_module;

use state_definition::StateDefintion;
use syn::{parse2, ItemMod};

#[proc_macro_attribute]
pub fn state(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_mod = match parse2::<ItemMod>(input.into()) {
        Ok(item_mod) => item_mod,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let def = match StateDefintion::try_from(item_mod) {
        Ok(def) => def,
        Err(err) => return err.to_compile_error().into(),
    };

    def.expand().into()
}
