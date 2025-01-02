use helpers::get_item_attribute;
use proc_macro2::Span;
use quote::quote;
use syn::{spanned::Spanned, Ident, ItemMod};

use crate::{
    attribute::Attribute, provider_definition::ProviderDef, provider_implementation::ProviderImpl,
    resource_definition::ResourceDef, resource_implementation::ResourceImpl,
};

pub(crate) struct Definition {
    ident: Ident,
    resource_defs: Vec<ResourceDef>,
    resource_impls: Vec<ResourceImpl>,
    provider_def: ProviderDef,
    provider_impl: ProviderImpl,
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

        let provider_def = self.provider_def.expand_provider_struct();
        let provider_impl = self.provider_impl.expand();
        let provider_trait = ProviderDef::expand_provider_trait();

        quote! {
            pub mod #mod_name {
                pub mod prelude {
                    #provider_def
                    #provider_trait
                    #provider_impl

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
        let mut provider_impl: Option<ProviderImpl> = None;

        for item in items {
            let provider_attribute: Option<Attribute> = get_item_attribute(&item)?;

            match provider_attribute {
                Some(Attribute::ResourceDefinition) => {
                    let resrouce_def = ResourceDef::try_from(item)?;
                    resource_defs.push(resrouce_def);
                }
                Some(Attribute::ResourceImplementation) => {
                    let resource_impl = ResourceImpl::try_from(item)?;
                    resource_impls.push(resource_impl);
                }
                Some(Attribute::ProviderDefintion) => {
                    provider_def = Some(ProviderDef::try_from(item)?)
                }
                Some(Attribute::ProviderImplementation) => {
                    provider_impl = Some(ProviderImpl::try_from(item)?)
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

        let Some(provider_impl) = provider_impl else {
            return Err(syn::Error::new(
                Span::call_site(),
                "Provider implementation not specified",
            ));
        };

        Ok(Self {
            ident,
            resource_defs,
            resource_impls,
            provider_def,
            provider_impl,
        })
    }
}
