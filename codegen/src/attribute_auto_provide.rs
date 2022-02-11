use crate::{attribute_provide, utils::import_crate};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemImpl;

pub fn transform(attribute: &TokenStream, input: ItemImpl) -> TokenStream {
    let base = attribute_provide::transform(attribute, input);
    assert!(
        base.generics.params.is_empty(),
        "Only concrete types can be registered. Use auto_register! instead"
    );

    let crate_path = import_crate();

    let provider_type = base.provider_type;
    let service_type = base.service_type;

    let register = quote! {
        #crate_path::auto_register!(#provider_type, #service_type);
    };

    TokenStream::from_iter([base.output, register])
}
