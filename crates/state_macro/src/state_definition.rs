use helpers::get_item_attribute;
use quote::quote;

use crate::{item::ItemState, resource::Resource, state_attribute::StateAttribute};

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
}

impl TryFrom<ItemState> for StateDefintion {
    type Error = syn::Error;

    fn try_from(value: ItemState) -> Result<Self, Self::Error> {
        let item_resources = value.item_resources();

        let mut resources: Vec<Resource> = Vec::new();

        for item_resource in item_resources {
            let state_attribute: Option<StateAttribute> = get_item_attribute(&item_resource)?;

            match state_attribute {
                Some(StateAttribute::Resource(resource_field)) => {
                    let resource = Resource::from(item_resource, resource_field);
                    resources.push(resource);
                }
                None => {}
            }
        }

        Ok(Self { resources })
    }
}
