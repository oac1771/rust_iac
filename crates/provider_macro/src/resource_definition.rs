use quote::{format_ident, quote, ToTokens};
use syn::{
    spanned::Spanned, token::Pub, Field, Fields, Item, ItemStruct, Pat, PatIdent, PatType, Token,
    Visibility,
};

const OUTPUT_IDENTIFIER: &str = "__output";

pub(crate) struct ResourceDef {
    item_struct: ItemStruct,
    outputs: Vec<PatType>,
}

impl ResourceDef {
    pub(crate) fn try_from(item: Item, mut outputs: Vec<PatType>) -> syn::Result<Self> {
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
            outputs.iter_mut().for_each(|p| {
                println!(">> {:?}", p.pat.to_token_stream());

                let ident = format_ident!(
                    "{}_{}",
                    OUTPUT_IDENTIFIER,
                    p.pat.as_ref().to_token_stream().to_string()
                );

                let field = Field {
                    attrs: Vec::new(),
                    vis: Visibility::Inherited,
                    mutability: syn::FieldMutability::None,
                    ident: Some(ident),
                    colon_token: Some(p.colon_token),
                    ty: *p.ty.clone(),
                };

                named_fields.named.push_value(field);
            });
        }

        item_struct.attrs = vec![];
        item_struct.vis = Visibility::Public(Pub(span));
        item_struct
            .fields
            .iter_mut()
            .for_each(|f| f.vis = Visibility::Public(Pub(span)));

        Ok(Self {
            item_struct,
            outputs,
        })
    }

    pub(crate) fn expand_resource_struct(self) -> proc_macro2::TokenStream {
        let item_struct = self.item_struct.to_token_stream();
        let item_struct_name = self.item_struct.ident.to_token_stream();

        let (output_field, non_output_field): (Vec<Field>, Vec<Field>) =
            self.item_struct.fields.into_iter().partition(|f| {
                if let Some(ident) = &f.ident {
                    ident.to_string().starts_with(OUTPUT_IDENTIFIER)
                } else {
                    false
                }
            });

        let output_field_name = output_field.iter().map(|f| f.ident.to_token_stream());
        let non_output_field_name = non_output_field.iter().map(|f| f.ident.to_token_stream());
        let non_output_pat = non_output_field.iter().map(|f| {
            let ident = f.ident.clone().unwrap();
            let pat_ident = PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident,
                subpat: None,
            };

            PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Ident(pat_ident)),
                colon_token: Token![:](f.ty.span()),
                ty: Box::new(f.ty.clone()),
            }
        });

        quote! {
            #[allow(dead_code)]
            #item_struct

            impl #item_struct_name {
                pub fn new(
                    #(#non_output_pat)*
                ) -> Self {
                    Self {
                        #(#non_output_field_name,)*
                        #(#output_field_name: Default::default(),)*
                    }
                }
            }
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
