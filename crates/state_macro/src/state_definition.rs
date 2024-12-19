use quote::quote;
use syn::{spanned::Spanned, ItemMod};

use crate::{
    resource::ResourceDef, state_attribute::StateAttribute,
    state_helpers::take_first_item_state_attr,
};

pub(crate) struct StateDefintion {
    resources: Vec<ResourceDef>,
}

impl StateDefintion {
    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        let resources_impls = self.resources.into_iter().map(|r| r.expand());

        quote! {
            #(#resources_impls)*
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

        let mut resources: Vec<ResourceDef> = Vec::new();

        for item in items {
            let state_attribute: Option<StateAttribute> = take_first_item_state_attr(&item)?;

            match state_attribute {
                Some(StateAttribute::Resource) => {
                    let resource = ResourceDef::try_from(item)?;
                    resources.push(resource);
                }
                None => {}
            }
        }

        Ok(Self { resources })
    }
}
