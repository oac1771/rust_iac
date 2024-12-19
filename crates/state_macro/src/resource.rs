use crate::state_attribute::ResourceField;
use quote::quote;
use syn::{spanned::Spanned, Item, ItemStruct};

pub(crate) struct ResourceDef {
    item_struct: ItemStruct,
}

impl ResourceDef {
    pub(crate) fn try_from(item: Item, resource_field: ResourceField) -> syn::Result<Self> {
        let item_struct = if let Item::Struct(item) = item {
            item
        } else {
            return Err(syn::Error::new(
                item.span(),
                "Invalid resource, expected struct item",
            ));
        };

        Ok(Self { item_struct })
    }

    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        let struct_token = self.item_struct.struct_token;
        let ident = self.item_struct.ident;

        quote! {
            #struct_token #ident;
        }
    }
}
