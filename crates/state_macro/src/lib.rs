mod items;
mod resource;
mod state_attribute;
mod state_definition;

use items::item_state::ItemState;
use state_definition::StateDefintion;
use syn::parse2;

#[proc_macro]
pub fn state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item_state = match parse2::<ItemState>(input.into()) {
        Ok(item_state) => item_state,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let def = match StateDefintion::try_from(item_state) {
        Ok(def) => def,
        Err(err) => return err.to_compile_error().into(),
    };

    def.expand().into()
}
