use crate::{items::item_resource::ItemResource, state_attribute::ResourceField};
use quote::quote;
use syn::Ident;

#[derive(Clone)]
pub(crate) struct Resource {
    pub(crate) item_resource: ItemResource,
    pub(crate) name_val: Ident,
}

impl Resource {
    pub(crate) fn from(item_resource: ItemResource, resource_field: ResourceField) -> Self {
        let name_val = resource_field.name_val;

        Self {
            item_resource,
            name_val,
        }
    }

    pub(crate) fn expand_instantiation(&self) -> proc_macro2::TokenStream {
        let name = &self.name_val;
        let struct_name = &self.item_resource.ident;
        let fields = self.item_resource.fields.iter().map(|f| f.expr.clone());

        quote! {
            let #name: #struct_name = #struct_name::new(
                #(#fields,)*
            );
        }
    }

    pub(crate) fn expand_name(&self) -> proc_macro2::TokenStream {
        let name = &self.name_val;

        quote! {
            #name
        }
    }

    pub(crate) fn get_dependencies(&self) -> Vec<Ident> {
        self.item_resource.get_dependencies()
    }

    pub(crate) fn name(&self) -> String {
        self.name_val.to_string()
    }
}
