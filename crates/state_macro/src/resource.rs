use crate::state_attribute::ResourceField;
use quote::quote;
use syn::{spanned::Spanned, Ident, Item, ItemStruct};

pub(crate) struct ResourceDef {
    item_struct: ItemStruct,
    _name_val: Ident,
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

        let _name_val = resource_field.name_val;

        Ok(Self {
            item_struct,
            _name_val,
        })
    }

    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        let struct_token = self.item_struct.struct_token;
        let struct_ident = self.item_struct.ident;

        quote! {
            pub #struct_token #struct_ident;
        }
    }
}
