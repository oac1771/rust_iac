use quote::{format_ident, quote, ToTokens};
use syn::{spanned::Spanned, token::Pub, Field, Fields, Item, ItemStruct, Visibility};

pub(crate) struct ResourceDef {
    item_struct: ItemStruct,
    output_fields: Vec<Field>,
}

impl ResourceDef {
    pub(crate) fn try_from(item: Item, output_fields: Vec<Field>) -> syn::Result<Self> {
        let span = item.span();
        let mut item_struct = if let Item::Struct(item) = item {
            item
        } else {
            return Err(syn::Error::new(
                item.span(),
                "Invalid resource, expected struct item",
            ));
        };

        if let Fields::Named(ref mut named_fields) = item_struct.fields {
            output_fields.iter().for_each(|f| named_fields.named.push_value(f.clone()));
        }

        item_struct.attrs = vec![];
        item_struct.vis = Visibility::Public(Pub(span));
        item_struct
            .fields
            .iter_mut()
            .for_each(|f| f.vis = Visibility::Public(Pub(span)));

        Ok(Self {
            item_struct,
            output_fields,
        })
    }

    pub(crate) fn expand_resource_struct(&self) -> proc_macro2::TokenStream {
        let item_struct = self.item_struct.to_token_stream();
        let item_struct_name = self.item_struct.ident.to_token_stream();

        // let output_fn_name = self
        //     .outputs
        //     .iter()
        //     .map(|f| format_ident!("get_{}", f.member));

        // let output_fn_type = self.outputs.iter().map(|f| f.expr.to_token_stream());

        quote! {
            #[allow(dead_code)]
            #item_struct

            // impl #item_struct_name {
            //     #(
            //         pub fn #output_fn_name(&self) -> #output_fn_type {}
            //     )*
            // }
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
