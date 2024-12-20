use quote::ToTokens;
use syn::{spanned::Spanned, Item, ItemStruct};

pub(crate) struct ResourceDef {
    item_struct: ItemStruct,
}

impl ResourceDef {
    pub(crate) fn try_from(item: Item) -> syn::Result<Self> {
        let mut item_struct = if let Item::Struct(item) = item {
            item
        } else {
            return Err(syn::Error::new(
                item.span(),
                "Invalid resource, expected struct item",
            ));
        };

        item_struct.attrs = vec![];

        Ok(Self { item_struct })
    }

    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        self.item_struct.to_token_stream()
    }
}
