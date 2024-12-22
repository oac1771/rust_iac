use helpers::ItemAttrs;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Expr, FieldValue, Ident, Token,
};

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
                if let Expr::Field(expr_field) = &f.expr {
                    let field_val = expr_field.to_token_stream().to_string();
                    let mut dependency = field_val
                        .split('.')
                        .map(|f| Ident::new(f, Span::call_site()))
                        .take(1);
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

        let mut fields: Punctuated<FieldValue, Token![,]> = Punctuated::new();

        while !content.is_empty() {
            fields = Punctuated::<FieldValue, Token![,]>::parse_terminated(&content)?;
        }

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
    use syn::{parse2, Expr, ExprLit, Lit, LitInt, Member};

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
}
