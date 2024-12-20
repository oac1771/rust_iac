use crate::state_attribute::ResourceField;
use quote::quote;
use syn::{spanned::Spanned, Ident, Item, ItemStruct};

pub(crate) struct Resource {
    item_struct: ItemStruct,
    name_val: Ident,
}

impl Resource {
    pub(crate) fn try_from(item: Item, resource_field: ResourceField) -> syn::Result<Self> {
        let item_struct = if let Item::Struct(item) = item {
            item
        } else {
            return Err(syn::Error::new(
                item.span(),
                "Invalid resource, expected struct item",
            ));
        };

        let name_val = resource_field.name_val;

        Ok(Self {
            item_struct,
            name_val,
        })
    }

    pub(crate) fn expand(self) -> proc_macro2::TokenStream {

        // let name = self.name_val;

        // quote! {
        //     let #name: Ident;
        // }

        quote! {}
    }
}
