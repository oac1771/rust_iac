use quote::ToTokens;
use syn::ItemStruct;

// this should spit out :
//  mod::provider {
//      use resourceA...
//
//      struct Provider;
//      impl Provider {}
// }

pub(crate) struct ProviderDefinition;

impl TryFrom<ItemStruct> for ProviderDefinition {
    type Error = syn::Error;

    fn try_from(value: ItemStruct) -> Result<Self, Self::Error> {
        value.fields.iter().for_each(|f| {
            println!(">>> {}", f.ty.to_token_stream());
        });
        Ok(Self)
    }
}
