use crate::{state_attribute::ResourceField, state_module::ItemResource};
use quote::quote;
use syn::Ident;

pub(crate) struct Resource {
    item_resource: ItemResource,
    name_val: Ident,
}

impl Resource {
    pub(crate) fn try_from(
        item_resource: ItemResource,
        resource_field: ResourceField,
    ) -> syn::Result<Self> {
        let name_val = resource_field.name_val;

        Ok(Self {
            item_resource,
            name_val,
        })
    }

    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        let name = self.name_val;
        let item_resource = self.item_resource;

        quote! {
            let #name =
        }
    }
}
