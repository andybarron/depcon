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

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;
    use syn::parse2;

    #[test]
    fn test_auto_provide() {
        let input = quote! {
            impl some::Trait for some::Struct {}
        };
        let input: ItemImpl = parse2(input).unwrap();
        let attribute = TokenStream::new();
        let actual = transform(&attribute, input).to_string();
        let expected = quote! {
            impl some::Trait for some::Struct {}
            impl depcon::Provider<dyn some::Trait> for some::Struct {
                fn provide(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn some::Trait> {
                    self
                }
            }
            depcon::auto_register!(some::Struct, dyn some::Trait);
        }
        .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic(expected = "attribute does not accept parameters")]
    fn test_reject_params() {
        let input = quote! {
            impl Trait for Struct {}
        };
        let input: ItemImpl = parse2(input).unwrap();
        let attribute = quote! { bad };
        transform(&attribute, input);
    }
}
