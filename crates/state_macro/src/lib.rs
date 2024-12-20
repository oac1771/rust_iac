mod resource;
mod state_attribute;
mod state_definition;
mod state_module;

use state_definition::StateDefintion;
use state_module::StateModule;
use syn::parse2;

#[proc_macro_attribute]
pub fn state(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let state_mod = match parse2::<StateModule>(input.into()) {
        Ok(state_mod) => state_mod,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let def = match StateDefintion::try_from(state_mod) {
        Ok(def) => def,
        Err(err) => return err.to_compile_error().into(),
    };

    def.expand().into()
}
