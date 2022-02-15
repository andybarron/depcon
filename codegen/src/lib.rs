#![warn(
    missing_docs,
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    clippy::cargo
)]
#![allow(clippy::wildcard_imports)]
#![doc = include_str!("../README.md")]
mod attribute_auto_provide;
mod attribute_provide;
mod derive_injectable;
mod utils;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Procedural macro for `#[derive(Injectable)]`
#[cfg(not(tarpaulin_include))]
#[proc_macro_derive(Injectable)]
pub fn derive_injectable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    derive_injectable::transform(input).into()
}

/// Procedural macro for `#[provide]`
#[cfg(not(tarpaulin_include))]
#[proc_macro_attribute]
pub fn provide(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item);
    attribute_provide::transform(&attribute.into(), input)
        .output
        .into()
}

/// Procedural macro for `#[auto_provide]`
#[cfg(not(tarpaulin_include))]
#[proc_macro_attribute]
pub fn auto_provide(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item);
    attribute_auto_provide::transform(&attribute.into(), input).into()
}
