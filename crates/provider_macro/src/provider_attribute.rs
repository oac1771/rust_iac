use syn::parse::{Parse, ParseStream};

pub(crate) enum ProviderAttribute {
    ResourceDefinition,
    ProviderDefintion,
}

impl Parse for ProviderAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);

        if content.peek(keyword::resource_definition) {
            content.parse::<keyword::resource_definition>()?;

            Ok(Self::ResourceDefinition)
        } else if content.peek(keyword::provider_definition) {
            content.parse::<keyword::provider_definition>()?;
            Ok(Self::ProviderDefintion)
        } else {
            Err(content.error("Expected keyword not found"))
        }
    }
}

mod keyword {
    syn::custom_keyword!(resource_definition);
    syn::custom_keyword!(provider_definition);
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
