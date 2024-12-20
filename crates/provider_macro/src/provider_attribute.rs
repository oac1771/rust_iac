use syn::parse::{Parse, ParseStream};

pub(crate) enum ProviderAttribute {
    ResourceDefinition,
}

impl Parse for ProviderAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);

        let lookahead = content.lookahead1();

        if lookahead.peek(keyword::resource_definition) {
            content.parse::<keyword::resource_definition>()?;

            Ok(Self::ResourceDefinition)
        } else {
            Err(lookahead.error())
        }
    }
}

mod keyword {
    syn::custom_keyword!(resource_definition);
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;
    use syn::parse2;

    #[test]
    fn test_resource_provider_attribute_parses_correctly() {
        let input = quote! {
            #[resource_definition]
        };

        let _result: ProviderAttribute = parse2(input).unwrap();
    }
}
