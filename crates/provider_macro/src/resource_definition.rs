use quote::{format_ident, quote, ToTokens};
use syn::{
    spanned::Spanned, token::Pub, Field, Fields, Item, ItemStruct, Pat, PatIdent, PatType, Token,
    Visibility,
};

const OUTPUT_IDENTIFIER: &str = "__output_";

pub(crate) struct ResourceDef {
    item_struct: ItemStruct,
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
                let ident = format_ident!(
                    "{}{}",
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

        Ok(Self { item_struct })
    }

    pub(crate) fn expand_resource_struct(self) -> proc_macro2::TokenStream {
        let item_struct_name = self.item_struct.ident.to_token_stream();
        let item_struct = self.item_struct.to_token_stream();

        let (output_field, non_output_field): (Vec<Field>, Vec<Field>) =
            self.item_struct.fields.into_iter().partition(|f| {
                if let Some(ident) = &f.ident {
                    ident.to_string().starts_with(OUTPUT_IDENTIFIER)
                } else {
                    false
                }
            });

        let new_fn = Self::expand_new_method(&output_field, &non_output_field);
        let getter_fns = Self::expand_getters(&output_field);
        let setter_fns = Self::expand_setters(&output_field);

        quote! {
            #[allow(dead_code)]
            #item_struct

            impl #item_struct_name {
                #new_fn
                #getter_fns
                #setter_fns
            }
        }
    }

    fn expand_new_method(
        output_field: &Vec<Field>,
        non_output_field: &Vec<Field>,
    ) -> proc_macro2::TokenStream {
        let output_field_name = output_field.iter().filter_map(|f| f.ident.clone());
        let non_output_field_name = non_output_field.iter().filter_map(|f| f.ident.clone());

        let non_output_pat = non_output_field.iter().filter_map(|f| {
            if let Some(ident) = f.ident.clone() {
                let pat_ident = PatIdent {
                    attrs: Vec::new(),
                    by_ref: None,
                    mutability: None,
                    ident,
                    subpat: None,
                };

                Some(PatType {
                    attrs: Vec::new(),
                    pat: Box::new(Pat::Ident(pat_ident)),
                    colon_token: Token![:](f.ty.span()),
                    ty: Box::new(f.ty.clone()),
                })
            } else {
                None
            }
        });

        quote! {
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

    fn expand_getters(output_field: &Vec<Field>) -> proc_macro2::TokenStream {
        let output_field_name = output_field.iter().filter_map(|f| f.ident.clone());
        let output_field_type = output_field.iter().map(|f| f.ty.clone());

        let getter_name = output_field_name
            .clone()
            .filter_map(|i| {
                let ident = i.to_string();
                ident
                    .split(OUTPUT_IDENTIFIER)
                    .filter(|s| !s.is_empty())
                    .next()
                    .map(|s| s.to_string())
            })
            .map(|i| format_ident!("get_{}", i));

        quote! {
            #(
                pub fn #getter_name(&self) -> #output_field_type {
                    self.#output_field_name
                }
            )*
        }
    }

    fn expand_setters(output_field: &Vec<Field>) -> proc_macro2::TokenStream {
        let output_field_name = output_field.iter().filter_map(|f| f.ident.clone());
        let output_field_type = output_field.iter().map(|f| f.ty.clone());

        let setter_name = output_field_name
            .clone()
            .filter_map(|i| {
                let ident = i.to_string();
                ident
                    .split(OUTPUT_IDENTIFIER)
                    .filter(|s| !s.is_empty())
                    .next()
                    .map(|s| s.to_string())
            })
            .map(|i| format_ident!("set_{}", i));

        quote! {
            #(
                pub fn #setter_name(&mut self, val: #output_field_type) {
                    self.#output_field_name = val
                }
            )*
        }
    }

    pub(crate) fn expand_resource_trait() -> proc_macro2::TokenStream {
        let resource_trait_name = helpers::resource_trait_name();
        quote! {
            pub trait #resource_trait_name {
                type Payload;

                fn payload(&self) -> Self::Payload;

                fn set_outputs(&mut self) {}
            }
        }
    }
}
