use helpers::get_item_attribute;
use quote::quote;

use crate::{resource::Resource, state_attribute::StateAttribute, state_module::StateModule};

pub(crate) struct StateDefintion {
    resources: Vec<Resource>,
}

impl StateDefintion {
    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        let resources_impls = self.resources.into_iter().map(|r| r.expand());

        quote! {
            mod state {
                #(#resources_impls)*
            }
        }
    }
}

impl TryFrom<StateModule> for StateDefintion {
    type Error = syn::Error;

    fn try_from(value: StateModule) -> Result<Self, Self::Error> {
        let items = value.resources_iter();

        let mut resources: Vec<Resource> = Vec::new();

        for item in items {
            let state_attribute: Option<StateAttribute> = get_item_attribute(&item)?;

            match state_attribute {
                Some(StateAttribute::Resource(resource_field)) => {
                    let resource = Resource::try_from(item, resource_field)?;
                    resources.push(resource);
                }
                None => {}
            }
        }

        Ok(Self { resources })
    }
}
