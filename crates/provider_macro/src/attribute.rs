use syn::parse::{Parse, ParseStream};

pub(crate) enum Attribute {
    ResourceDefinition,
    ResourceImplementation,
    ProviderDefintion,
    ProviderImplementation,
}

impl Parse for Attribute {
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
        } else if content.peek(keyword::resource_implementation) {
            content.parse::<keyword::resource_implementation>()?;
            Ok(Self::ResourceImplementation)
        } else if content.peek(keyword::provider_implementation) {
            content.parse::<keyword::provider_implementation>()?;
            Ok(Self::ProviderImplementation)
        } else {
            Err(content.error("Expected keyword not found"))
        }
    }
}

mod keyword {
    syn::custom_keyword!(resource_definition);
    syn::custom_keyword!(resource_implementation);
    syn::custom_keyword!(provider_definition);
    syn::custom_keyword!(provider_implementation);
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;
    use syn::parse2;

    #[test]
    fn test_resource_provider_attribute_parses_resource_def_correctly() {
        let input = quote! {
            #[resource_definition]
        };

        let _result: Attribute = parse2(input).unwrap();
    }

    #[test]
    fn test_resource_provider_attribute_parses_provider_def_correctly() {
        let input = quote! {
            #[provider_definition]
        };

        let _result: Attribute = parse2(input).unwrap();
    }

    #[test]
    fn test_resource_provider_attribute_parses_resource_impl_correctly() {
        let input = quote! {
            #[resource_implementation]
        };

        let _result: Attribute = parse2(input).unwrap();
    }

    #[test]
    fn test_resource_provider_attribute_parses_provider_impl_correctly() {
        let input = quote! {
            #[provider_implementation]
        };

        let _result: Attribute = parse2(input).unwrap();
    }
}
