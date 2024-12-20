use helpers::get_item_attribute;
use quote::quote;
use syn::{spanned::Spanned, ItemMod};

use crate::{resource::Resource, state_attribute::StateAttribute};

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

impl TryFrom<ItemMod> for StateDefintion {
    type Error = syn::Error;

    fn try_from(value: ItemMod) -> Result<Self, Self::Error> {
        let item_span = value.span();

        let items = value
            .content
            .ok_or_else(|| {
                let msg = "Invalid state definition, expected mod to be inlined.";
                syn::Error::new(item_span, msg)
            })?
            .1;

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
