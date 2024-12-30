use quote::{quote, ToTokens};
use syn::{spanned::Spanned, token::Pub, Item, ItemStruct, Visibility};

pub(crate) struct ProviderDef {
    item_struct: ItemStruct,
}

impl ProviderDef {
    pub(crate) fn try_from(item: Item) -> syn::Result<Self> {
        let span = item.span();
        let mut item_struct = if let Item::Struct(item) = item {
            item
        } else {
            return Err(syn::Error::new(
                item.span(),
                "Invalid provider definition, expected struct item",
            ));
        };

        item_struct.attrs = vec![];
        item_struct.vis = Visibility::Public(Pub(span));
        item_struct
            .fields
            .iter_mut()
            .for_each(|f| f.vis = Visibility::Public(Pub(span)));

        Ok(Self { item_struct })
    }

    pub(crate) fn expand_provider_trait(&self) -> proc_macro2::TokenStream {
        quote! {
            trait Provider {}
        }
    }

    pub(crate) fn expand_provider_trait_impl(&self) -> proc_macro2::TokenStream {
        let provider_ident = self.item_struct.ident.clone();

        quote! {
            impl Provider for #provider_ident {}
        }
    }

    pub(crate) fn expand_provider_struct(&self) -> proc_macro2::TokenStream {
        let item_struct = self.item_struct.to_token_stream();

        quote! {
            #[allow(dead_code)]
            #item_struct
        }
    }
}
