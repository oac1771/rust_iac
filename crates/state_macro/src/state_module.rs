use helpers::ItemAttrs;
// use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, FieldValue, Ident, Token,
};

pub(crate) struct StateModule {
    resources: Vec<ItemResource>,
}

pub(crate) struct ItemResource {
    pub ident: Ident,
    pub attrs: Vec<Attribute>,
    pub fields: Punctuated<FieldValue, Token![,]>,
}

impl StateModule {
    pub fn resources_iter(self) -> impl Iterator<Item = ItemResource> {
        self.resources.into_iter()
    }
}

impl Parse for StateModule {
    fn parse(input: ParseStream) -> syn::Result<Self> {

        let mut resources = Vec::new();

        while !input.is_empty() {
            let resource = input.parse::<ItemResource>()?;
            resources.push(resource);
        }

        Ok(Self {
            resources,
        })
    }
}

impl Parse for ItemResource {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let ident: Ident = input.parse()?;

        let content;
        braced!(content in input);

        let mut fields: Punctuated<FieldValue, Token![,]> = Punctuated::new();

        while !content.is_empty() {
            fields = Punctuated::<FieldValue, Token![,]>::parse_terminated(&content)?;
        }

        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
        }

        Ok(Self {
            ident,
            attrs,
            fields,
        })
    }
}

impl ItemAttrs for ItemResource {
    fn item_attrs(&self) -> Option<&Vec<Attribute>> {
        Some(&self.attrs)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::Span;
    use quote::quote;
    use syn::{parse2, Expr, ExprLit, Lit, LitInt, Member};

    #[test]
    fn resource_parses_correctly() {
        let resource_name = Ident::new("DummyResourceA", Span::call_site());
        let field_name_1 = Ident::new("field_1", Span::call_site());

        let stream = quote! {
            #[foo(name = hello)]
            #resource_name {#field_name_1: 10};
        };

        let resource = parse2::<ItemResource>(stream).unwrap();

        let attrs = resource.attrs;
        let mut fields = resource.fields.iter();
        let res_field_1 = fields.next().unwrap();

        let Member::Named(ref field_1_ident) = res_field_1.member else {
            panic!("Wrong member enum variant retunrned")
        };
        let Expr::Lit(ExprLit {
            attrs: _,
            lit: Lit::Int(ref lit_int),
        }) = res_field_1.expr
        else {
            panic!("Wrong literal found")
        };

        let expected_val = LitInt::new("10", Span::call_site());

        assert_eq!(resource.ident, resource_name);
        assert_eq!(field_name_1, *field_1_ident);
        assert_eq!(expected_val.base10_digits(), lit_int.base10_digits());
        assert!(!attrs.is_empty());
    }

    #[test]
    fn state_module_saves_correct_name() {
        let mod_name = Ident::new("foo", Span::call_site());
        let resource_name = Ident::new("DummyResourceA", Span::call_site());
        let field_name_1 = Ident::new("field_1", Span::call_site());
        let field_name_2 = Ident::new("field_2", Span::call_site());

        let stream = quote! {
            mod #mod_name {
                #[resource(name = hello)]
                #resource_name {#field_name_1: 10, #field_name_2: "bar"};
            }
        };

        let state_module = parse2::<StateModule>(stream).unwrap();

        assert_eq!(state_module.mod_name, mod_name);
        assert_eq!(state_module.resources.len(), 1);
    }
}
