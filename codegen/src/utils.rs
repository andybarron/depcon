use proc_macro2::TokenStream;
use quote::quote;

pub fn import_crate() -> TokenStream {
    quote!(depcon)

    // TODO: Use proc-macro-crate to look up depcon crate and support renaming.
    //       Doc comments break due to: https://github.com/bkchr/proc-macro-crate/issues/14

    // let found_crate = crate_name("depcon").expect("depcon is present in `Cargo.toml`");
    // match found_crate {
    //     FoundCrate::Itself => quote!(crate),
    //     FoundCrate::Name(name) => {
    //         let ident = Ident::new(&name, Span::call_site());
    //         quote!( #ident )
    //     }
    // }
}
