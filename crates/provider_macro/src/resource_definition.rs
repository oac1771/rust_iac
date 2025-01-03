use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Comma, Pub},
    FieldValue, Item, ItemStruct, Visibility,
};

pub(crate) struct ResourceDef {
    item_struct: ItemStruct,
    outputs: Punctuated<FieldValue, Comma>,
}

impl ResourceDef {
    pub(crate) fn try_from(
        item: Item,
        outputs: Option<Punctuated<FieldValue, Comma>>,
    ) -> syn::Result<Self> {
        let span = item.span();
        let mut item_struct = if let Item::Struct(item) = item {
            item
        } else {
            return Err(syn::Error::new(
                item.span(),
                "Invalid resource, expected struct item",
            ));
        };

        item_struct.attrs = vec![];
        item_struct.vis = Visibility::Public(Pub(span));
        item_struct
            .fields
            .iter_mut()
            .for_each(|f| f.vis = Visibility::Public(Pub(span)));

        let outputs = match outputs {
            Some(o) => o,
            None => Punctuated::new(),
        };

        Ok(Self {
            item_struct,
            outputs,
        })
    }

    pub(crate) fn expand_resource_struct(&self) -> proc_macro2::TokenStream {
        let item_struct = self.item_struct.to_token_stream();

        quote! {
            #[allow(dead_code)]
            #item_struct
        }
    }

    pub(crate) fn expand_resource_trait() -> proc_macro2::TokenStream {
        let resource_trait_name = helpers::resource_trait_name();
        quote! {
            pub trait #resource_trait_name {
                type Payload;

                fn payload(&self) -> Self::Payload;
            }
        }
    }
}
