use syn::{braced, parse::{Parse, ParseStream}, Ident, Token, Expr};

pub(crate) struct StateModule {
    mod_name: Ident
}

struct Resource {
    name: Ident,
    fields: Vec<(Ident, Expr)>,
}

impl Parse for StateModule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![mod]>()?;

        let mod_name: Ident = input.parse()?;

        let content;
        braced!(content in input);

        // let mut resources = Vec::new();

        // while !content.is_empty() {

        // }

        Ok(Self {mod_name})
    }
}

impl Parse for Resource {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let mut fields = Vec::new();

        let content;
        braced!(content in input);

        while !content.is_empty() {
            let field_name = content.parse::<Ident>()?;
            content.parse::<Token![:]>()?;

            let expr: syn::Expr = content.parse()?;
            fields.push((field_name, expr));

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
        }

        Ok(Self {name, fields})
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::Span;
    use quote::quote;
    use syn::parse2;

    #[test]
    fn foo() {
        let resource_name = Ident::new("foo", Span::call_site());
        let stream = quote! {
            DummyResourceA {id_a: 10, foo: "bar"};
        };

        let resource = parse2::<Resource>(stream).unwrap();

    }

    #[test]
    fn state_module_saves_correct_name() {
        let mod_name = Ident::new("foo", Span::call_site());
        let stream = quote! {
            mod #mod_name {}
        };

        let state_module = parse2::<StateModule>(stream).unwrap();

        assert_eq!(state_module.mod_name, mod_name);
    }
}