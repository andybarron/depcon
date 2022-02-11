mod attribute_auto_provide;
mod attribute_provide;
mod derive_injectable;
mod utils;

use attribute_auto_provide::attribute_auto_provide_impl;
use attribute_provide::attribute_provide_impl;
use derive_injectable::derive_injectable_impl;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Injectable)]
pub fn derive_injectable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    derive_injectable_impl(input).into()
}

// TODO: support on struct definitions to provide Arc<Self>
#[proc_macro_attribute]
pub fn provide(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item);
    attribute_provide_impl(attribute.into(), input)
        .output
        .into()
}

#[proc_macro_attribute]
pub fn auto_provide(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item);
    attribute_auto_provide_impl(attribute.into(), input).into()
}
