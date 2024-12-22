use super::item_resource::ItemResource;
use syn::parse::{Parse, ParseStream};

pub(crate) struct ItemState {
    resources: Vec<ItemResource>,
}

impl ItemState {
    pub fn item_resources(self) -> impl Iterator<Item = ItemResource> {
        self.resources.into_iter()
    }
}

impl Parse for ItemState {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut resources = Vec::new();

        while !input.is_empty() {
            let resource = input.parse::<ItemResource>()?;
            resources.push(resource);
        }

        Ok(Self { resources })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::Span;
    use quote::quote;
    use syn::{parse2, Ident};

    #[test]
    fn state_module_saves_correct_name() {
        let resource_name = Ident::new("DummyResourceA", Span::call_site());
        let field_name_1 = Ident::new("field_1", Span::call_site());
        let field_name_2 = Ident::new("field_2", Span::call_site());

        let stream = quote! {
                #[resource(name = hello)]
                #resource_name {#field_name_1: 10, #field_name_2: "bar"};
        };

        let state_module = parse2::<ItemState>(stream).unwrap();

        assert_eq!(state_module.resources.len(), 1);
    }
}
