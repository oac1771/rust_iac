use quote::ToTokens;
use syn::{parse::Parse, parse2, Attribute, Item};

pub trait ItemAttrs {
    fn item_attrs(&self) -> Option<&Vec<Attribute>>;
}

impl ItemAttrs for Item {
    fn item_attrs(&self) -> Option<&Vec<Attribute>> {
        match self {
            Self::Struct(item) => Some(&item.attrs),
            _ => None,
        }
    }
}

pub fn get_item_attribute<Attr>(item: &impl ItemAttrs) -> syn::Result<Option<Attr>>
where
    Attr: Parse,
{
    if let Some(attr) = item.item_attrs().and_then(|attrs| attrs.iter().next()) {
        Ok(Some(parse2(attr.into_token_stream())?))
    } else {
        Ok(None)
    }
}
