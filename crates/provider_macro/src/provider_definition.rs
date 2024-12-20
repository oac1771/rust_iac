use helpers::get_item_attribute;
use syn::{spanned::Spanned, ItemMod};

use crate::{provider_attribute::ProviderAttribute, resource_definition::ResourceDef};

// this should spit out :
//  mod::provider {
//      use resourceA...
//
//      struct Provider;
//      impl Provider {}
// }

pub(crate) struct ProviderDefinition;

impl TryFrom<ItemMod> for ProviderDefinition {
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

        let _resource_defs: Vec<ResourceDef> = Vec::new();

        for item in items {
            let _provider_attribute: Option<ProviderAttribute> = get_item_attribute(&item)?;
        }

        Ok(Self)
    }
}
