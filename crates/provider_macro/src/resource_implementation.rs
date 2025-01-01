use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Item, ItemImpl};

pub(crate) struct ResourceImpl {
    item_impl: ItemImpl,
}

impl ResourceImpl {
    pub(crate) fn try_from(item: Item) -> syn::Result<Self> {
        let mut item_impl = if let Item::Impl(item) = item {
            item
        } else {
            return Err(syn::Error::new(
                item.span(),
                "Invalid resource implemenation, expected impl item",
            ));
        };

        item_impl.attrs = vec![];

        Ok(Self { item_impl })
    }

    pub(crate) fn expand(&self) -> proc_macro2::TokenStream {
        let item_impl = self.item_impl.to_token_stream();

        quote! {
            #item_impl
        }
    }
}
