use helpers::get_item_attribute;
use proc_macro2::Span;
use quote::quote;
use syn::{spanned::Spanned, Ident, ItemMod};

use crate::{
    provider_attribute::ProviderAttribute, provider_definition::ProviderDef,
    resource_definition::ResourceDef, resource_implementation::ResourceImpl,
};

pub(crate) struct Definition {
    ident: Ident,
    resource_defs: Vec<ResourceDef>,
    resource_impls: Vec<ResourceImpl>,
    provider_def: ProviderDef,
}

impl Definition {
    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        let mod_name = self.ident;

        let resource_def = self
            .resource_defs
            .iter()
            .map(|r| r.expand_resource_struct());
        let resource_impl = self.resource_impls.iter().map(|r_impl| r_impl.expand());
        let resource_trait = ResourceDef::expand_resource_trait();
        let provider_struct = self.provider_def.expand_provider_struct();
        let provider_trait = self.provider_def.expand_provider_trait();
        let provider_trait_impl = self.provider_def.expand_provider_trait_impl();

        quote! {
            pub mod #mod_name {
                pub mod prelude {
                    #provider_struct
                    #provider_trait
                    #provider_trait_impl

                    #resource_trait
                    #(#resource_def)*
                    #(#resource_impl)*
                }
            }
        }
    }
}

impl TryFrom<ItemMod> for Definition {
    type Error = syn::Error;

    fn try_from(value: ItemMod) -> Result<Self, Self::Error> {
        let item_span = value.span();
        let ident = value.ident;

        let items = value
            .content
            .ok_or_else(|| {
                let msg = "Invalid provider definition, expected mod to be inlined.";
                syn::Error::new(item_span, msg)
            })?
            .1;

        let mut resource_defs: Vec<ResourceDef> = Vec::new();
        let mut resource_impls: Vec<ResourceImpl> = Vec::new();
        let mut provider_def: Option<ProviderDef> = None;

        for item in items {
            let provider_attribute: Option<ProviderAttribute> = get_item_attribute(&item)?;

            match provider_attribute {
                Some(ProviderAttribute::ResourceDefinition) => {
                    let resrouce_def = ResourceDef::try_from(item)?;
                    resource_defs.push(resrouce_def);
                }
                Some(ProviderAttribute::ResourceImplementation) => {
                    let resource_impl = ResourceImpl::try_from(item)?;
                    resource_impls.push(resource_impl);
                }
                Some(ProviderAttribute::ProviderDefintion) => {
                    provider_def = Some(ProviderDef::try_from(item)?)
                }
                None => {}
            }
        }

        let Some(provider_def) = provider_def else {
            return Err(syn::Error::new(
                Span::call_site(),
                "Provider definition not specified",
            ));
        };

        Ok(Self {
            ident,
            resource_defs,
            resource_impls,
            provider_def,
        })
    }
}
