use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token,
};

pub(crate) enum StateAttribute {
    Resource(ResourceField),
}

pub(crate) struct ResourceField {
    fields: Punctuated<Ident, Token![=]>,
}

impl Parse for ResourceField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        // add name validation here

        Ok(Self {
            fields: content.parse_terminated(Ident::parse, Token![=])?,
        })
    }
}

impl Parse for StateAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);

        let lookahead = content.lookahead1();

        if lookahead.peek(keyword::resource) {
            content.parse::<keyword::resource>()?;

            let resource_field = content.parse::<ResourceField>()?;

            Ok(Self::Resource(resource_field))
        } else {
            Err(lookahead.error())
        }
    }
}

mod keyword {
    syn::custom_keyword!(resource);
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::Span;
    use quote::quote;
    use syn::parse2;

    #[test]
    fn test_resource_state_attribute_parses_correctly_with_items_inside_paranthesis() {
        let ident = Ident::new("name", Span::call_site());
        let name = Ident::new("foo", Span::call_site());

        let input = quote! {
            #[resource(#ident = #name)]
        };

        let result: StateAttribute = parse2(input).unwrap();

        let StateAttribute::Resource(resource_field) = result;

        let mut fields = resource_field.fields.iter();

        assert_eq!(fields.next(), Some(ident).as_ref());
        assert_eq!(fields.next(), Some(name).as_ref());
        assert_eq!(fields.next(), None);
    }

    #[test]
    fn test_resource_state_attribute_parses_correctly_without_items_inside_paranthesis() {
        let input = quote! {
            #[resource()]
        };

        let result: StateAttribute = parse2(input).unwrap();

        let StateAttribute::Resource(resource_field) = result;

        let mut fields = resource_field.fields.iter();

        assert_eq!(fields.next(), None);
    }
}
