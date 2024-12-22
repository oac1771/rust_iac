use helpers::get_item_attribute;
use quote::quote;

use crate::{items::item_state::ItemState, resource::Resource, state_attribute::StateAttribute};

pub(crate) struct StateDefintion {
    resources: Vec<Resource>,
}

impl StateDefintion {
    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        let resources_impls = self.resources.into_iter().map(|r| r.expand());

        quote! {
            #(#resources_impls)*
        }
    }

    fn get_resources(item_state: ItemState) -> Result<Vec<Resource>, syn::Error> {
        let resources = item_state
            .item_resources()
            .filter_map(|item_resource| {
                match get_item_attribute::<StateAttribute>(&item_resource) {
                    Ok(Some(state_attribute)) => match state_attribute {
                        StateAttribute::Resource(resource_field) => {
                            Some(Ok(Resource::from(item_resource, resource_field)))
                        }
                    },
                    Ok(None) => None,
                    Err(err) => Some(Err(err)),
                }
            })
            .collect::<Result<Vec<Resource>, syn::Error>>()?;

        Ok(resources)
    }
}

impl TryFrom<ItemState> for StateDefintion {
    type Error = syn::Error;

    fn try_from(value: ItemState) -> Result<Self, Self::Error> {
        let resources = Self::get_resources(value)?;

        Ok(Self { resources })
    }
}
