use syn::{
    parse::{Parse, ParseStream},
    token::Paren,
    PatType, Token,
};

pub(crate) enum Attribute {
    ResourceDefinition { outputs: Vec<PatType> },
    ResourceImplementation,
    ProviderDefintion,
    ProviderImplementation,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![#]>()?;
        let content;
        syn::bracketed!(content in input);

        if content.peek(keyword::resource_definition) {
            content.parse::<keyword::resource_definition>()?;

            let mut outputs: Vec<PatType> = Vec::new();

            if content.peek(Paren) {
                let resource_def_content;
                syn::parenthesized!(resource_def_content in content);
                resource_def_content.parse::<keyword::outputs>()?;
                resource_def_content.parse::<Token![=]>()?;

                let outputs_content;
                syn::braced!(outputs_content in resource_def_content);

                while !outputs_content.is_empty() {
                    outputs.push(outputs_content.parse::<PatType>()?);

                    if outputs_content.peek(Token![,]) {
                        outputs_content.parse::<Token![,]>()?;
                    }
                }
            }

            Ok(Self::ResourceDefinition { outputs })
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
    syn::custom_keyword!(outputs);
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
    fn test_resource_provider_attribute_parses_resource_def_with_single_output_correctly() {
        let input = quote! {
            #[resource_definition(outputs = {foo: String})]
        };

        let result: Attribute = parse2(input).unwrap();

        if let Attribute::ResourceDefinition { outputs } = result {
            assert_eq!(outputs.len(), 1);
        } else {
            panic!("parsed to incorrect attribute");
        }
    }

    #[test]
    fn test_resource_provider_attribute_parses_resource_def_with_double_output_correctly() {
        let input = quote! {
            #[resource_definition(outputs = {foo: String, bar: i32})]
        };

        let result: Attribute = parse2(input).unwrap();

        if let Attribute::ResourceDefinition { outputs } = result {
            assert_eq!(outputs.len(), 2);
        } else {
            panic!("parsed to incorrect attribute");
        }
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
