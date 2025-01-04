use helpers::ItemAttrs;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Expr, FieldValue, Ident, Token,
};

#[derive(Clone)]
pub(crate) struct ItemResource {
    pub(crate) ident: Ident,
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) fields: Punctuated<FieldValue, Token![,]>,
}

impl ItemResource {
    pub(crate) fn get_dependencies(&self) -> Vec<Ident> {
        let dependencies = self
            .fields
            .iter()
            .filter_map(|f| {
                if let Expr::MethodCall(exprt_method_call) = &f.expr {
                    let field_value = exprt_method_call.to_token_stream().to_string();

                    let mut dependency = field_value.split('.').map(|f| {
                        let f = f.trim();
                        Ident::new(f, Span::call_site())
                    });
                    dependency.next()
                } else {
                    None
                }
            })
            .collect::<Vec<Ident>>();

        dependencies
    }
}

impl Parse for ItemResource {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let ident: Ident = input.parse()?;

        let content;
        braced!(content in input);

        let fields = Punctuated::<FieldValue, Token![,]>::parse_terminated(&content)?;

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
    use syn::{parse2, Expr, ExprLit, Ident, Lit, LitInt, Member};

    #[test]
    fn resource_parses_correctly() {
        let resource_name = Ident::new("DummyResourceA", Span::call_site());
        let field_name = Ident::new("field_1", Span::call_site());
        let val_name = LitInt::new("10", Span::call_site());

        let stream = quote! {
            #[foo(bar = zap)]
            #resource_name {#field_name: #val_name};
        };

        let resource = parse2::<ItemResource>(stream).unwrap();

        let attrs = resource.attrs;
        let mut fields = resource.fields.iter();
        let res_field_1 = fields.next().unwrap();

        let Member::Named(ref field_1_ident) = res_field_1.member else {
            panic!("Wrong member enum variant retunrned")
        };

        let Expr::Lit(ExprLit {
            lit: Lit::Int(ref lit_int),
            ..
        }) = res_field_1.expr
        else {
            panic!("Wrong literal found")
        };

        assert_eq!(resource.ident, resource_name);
        assert_eq!(field_name, *field_1_ident);
        assert_eq!(val_name.base10_digits(), lit_int.base10_digits());
        assert!(!attrs.is_empty());
    }

    #[test]
    fn resource_gives_correct_dependencies() {
        let val = Ident::new("bar", Span::call_site());

        let stream = quote! {
            #[foo(bar = zap)]
            Foo {field_1: #val.field_1()};
        };

        let resource = parse2::<ItemResource>(stream).unwrap();

        let dependencies = resource.get_dependencies();
        assert_eq!(val, dependencies[0]);
    }

    #[test]
    fn resource_gives_correct_empty_dependencies() {
        let stream = quote! {
            #[foo(bar = zap)]
            Foo {field_1: 10};
        };

        let resource = parse2::<ItemResource>(stream).unwrap();

        let dependencies = resource.get_dependencies();
        assert!(dependencies.is_empty());
    }

    #[test]
    fn resource_gives_correct_multiple_dependencies() {
        let val_1 = Ident::new("bar_1", Span::call_site());
        let val_2 = Ident::new("bar_2", Span::call_site());

        let stream = quote! {
            #[foo(bar = zap)]
            Foo {
                field_1: #val_1.field_1(),
                field_2: #val_2.field_1(),
                field_3: 10,
            };
        };

        let resource = parse2::<ItemResource>(stream).unwrap();

        let dependencies = resource.get_dependencies();
        assert_eq!(dependencies.len(), 2);
    }
}
