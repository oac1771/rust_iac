use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token,
};

pub(crate) enum StateAttribute {
    Resource(ResourceField),
}

pub(crate) struct ResourceField {
    name_val: Ident,
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

impl Parse for ResourceField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let metaitem: Punctuated<Ident, Token![=]> =
            content.parse_terminated(Ident::parse, Token![=])?;

        let name_pos = metaitem
            .iter()
            .by_ref()
            .position(|i| i.to_string() == "name")
            .ok_or_else(|| {
                syn::Error::new(
                    input.span(),
                    "Expected `name` metaitem in resource attribute",
                )
            })?;

        let name_val = metaitem
            .into_iter()
            .nth(name_pos + 1)
            .ok_or_else(|| syn::Error::new(input.span(), "`name` metaitem did not have value"))?;

        Ok(Self { name_val })
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

        let name_val = resource_field.name_val;

        assert_eq!(name_val, name);
    }

    #[test]
    fn test_resource_state_attribute_returns_error_if_name_is_not_found() {
        let input = quote! {
            #[resource(foo = bar)]
        };

        let err = parse2::<StateAttribute>(input).err().unwrap();

        assert_eq!(
            err.to_string(),
            "Expected `name` metaitem in resource attribute"
        );
    }

    #[test]
    fn test_resource_state_attribute_returns_error_if_name_does_not_have_value() {
        let input = quote! {
            #[resource(name = )]
        };

        let err = parse2::<StateAttribute>(input).err().unwrap();

        assert_eq!(err.to_string(), "`name` metaitem did not have value");
    }
}
