use syn::{
    braced,
    parse::{Parse, ParseStream},
    Attribute, Expr, Ident, Token,
};

pub(crate) struct StateModule {
    mod_name: Ident,
    resources: Vec<Resource>,
}

struct Resource {
    name: Ident,
    attrs: Vec<Attribute>,
    fields: Vec<(Ident, Expr)>,
}

impl Parse for StateModule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![mod]>()?;

        let mod_name: Ident = input.parse()?;

        let content;
        braced!(content in input);

        let mut resources = Vec::new();

        while !content.is_empty() {
            let resource = content.parse::<Resource>()?;
            resources.push(resource);
        }

        Ok(Self {
            mod_name,
            resources,
        })
    }
}

impl Parse for Resource {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
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

        Ok(Self {
            name,
            attrs,
            fields,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::Span;
    use quote::quote;
    use syn::parse2;

    #[test]
    fn resource_parses_correctly() {
        let resource_name = Ident::new("DummyResourceA", Span::call_site());
        let field_name_1 = Ident::new("field_1", Span::call_site());
        let field_name_2 = Ident::new("field_2", Span::call_site());

        let stream = quote! {
            #[foo(name = hello)]
            #resource_name {#field_name_1: 10, #field_name_2: "bar"};
        };

        let resource = parse2::<Resource>(stream).unwrap();

        let attrs = resource.attrs;
        let mut fields = resource.fields.iter();
        let (res_field_1, exp_1) = fields.next().unwrap();
        let (res_field_2, exp_2) = fields.next().unwrap();

        assert_eq!(resource.name, resource_name);
        assert_eq!(*res_field_1, field_name_1);
        assert_eq!(*res_field_2, field_name_2);
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
                #[foo(name = hello)]
                #resource_name {#field_name_1: 10, #field_name_2: "bar"};
            }
        };

        let state_module = parse2::<StateModule>(stream).unwrap();

        assert_eq!(state_module.mod_name, mod_name);
        assert_eq!(state_module.resources.len(), 1);
    }
}
