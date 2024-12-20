use helpers::get_item_attribute;
use quote::quote;
use syn::{spanned::Spanned, Ident, ItemMod};

use crate::{provider_attribute::ProviderAttribute, resource_definition::ResourceDef};

// this should spit out :
//  mod::provider {
//      use resourceA...
//
//      struct Provider;
//      impl Provider {}
// }

pub(crate) struct ProviderDefinition {
    ident: Ident,
    resource_defs: Vec<ResourceDef>,
}

impl ProviderDefinition {
    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        let resource_defs = self.resource_defs.into_iter().map(|r| r.expand());
        let mod_name = self.ident;

        quote! {
            pub mod #mod_name {
                pub mod prelude {
                    #(#resource_defs)*
                }
            }
        }
    }
}

impl TryFrom<ItemMod> for ProviderDefinition {
    type Error = syn::Error;

    fn try_from(value: ItemMod) -> Result<Self, Self::Error> {
        let item_span = value.span();
        let ident = value.ident;

        let items = value
            .content
            .ok_or_else(|| {
                let msg = "Invalid state definition, expected mod to be inlined.";
                syn::Error::new(item_span, msg)
            })?
            .1;

        let mut resource_defs: Vec<ResourceDef> = Vec::new();

        for item in items {
            let provider_attribute: Option<ProviderAttribute> = get_item_attribute(&item)?;

            match provider_attribute {
                Some(ProviderAttribute::ResourceDefinition) => {
                    let resrouce_def = ResourceDef::try_from(item)?;
                    resource_defs.push(resrouce_def);
                }
                None => {}
            }
        }

        Ok(Self {
            ident,
            resource_defs,
        })
    }
}
