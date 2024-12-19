use syn::parse::{Parse, ParseStream};

pub(crate) enum StateAttribute {
    Resource,
}

impl Parse for StateAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);

        let lookahead = content.lookahead1();

        if lookahead.peek(keyword::resource) {
            content.parse::<keyword::resource>()?;
            Ok(Self::Resource)
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
    use quote::quote;
    use syn::parse2;

    #[test]
    fn test_macro() {
        let input = quote! {
            #[resource]
        };

        let result: StateAttribute = parse2(input).unwrap();
        assert!(matches!(result, StateAttribute::Resource));
    }
}
