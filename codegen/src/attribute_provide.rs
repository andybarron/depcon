use crate::utils::import_crate;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Generics, ItemImpl};

pub struct ProvideAttribute {
    pub provider_type: TokenStream,
    pub service_type: TokenStream,
    pub generics: Generics,
    pub output: TokenStream,
}

// TODO: Assert attribute stream is empty
pub fn transform(_attribute: &TokenStream, input: ItemImpl) -> ProvideAttribute {
    let struct_type = input.self_ty.to_token_stream();
    let trait_ = input
        .trait_
        .as_ref()
        .expect("attribute must be applied to a trait impl")
        .1
        .to_token_stream();
    let dyn_trait = quote! {dyn #trait_};
    let implemented = implement_provider(
        &struct_type,
        &quote! {#dyn_trait},
        &input.generics,
        &quote! {self},
    );
    let output = TokenStream::from_iter([input.into_token_stream(), implemented.output]);
    ProvideAttribute {
        output,
        ..implemented
    }
}

fn implement_provider(
    target_type: &TokenStream,
    service_type: &TokenStream,
    generics: &Generics,
    body: &TokenStream,
) -> ProvideAttribute {
    let crate_path = import_crate();
    let (impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

    let output = quote! {
        impl #impl_generics #crate_path::Provider<#service_type> for #target_type #where_clause {
            fn provide(self: std::sync::Arc<Self>) -> std::sync::Arc<#service_type> {
                #body
            }
        }
    };

    ProvideAttribute {
        provider_type: target_type.clone(),
        service_type: service_type.clone(),
        generics: generics.clone(),
        output,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;
    use syn::parse2;

    #[test]
    fn test_provide() {
        let input = quote! {
            impl some::Trait for some::Struct {}
        };
        let input: ItemImpl = parse2(input).unwrap();
        let attribute = TokenStream::new();
        let actual = transform(&attribute, input).output.to_string();
        let expected = quote! {
            impl some::Trait for some::Struct {}
            impl depcon::Provider<dyn some::Trait> for some::Struct {
                fn provide(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn some::Trait> {
                    self
                }
            }
        }
        .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_provide_generics() {
        let input = quote! {
            impl<A, C> Trait<A> for Struct<C>
            where
                A: Clone,
                C: Send + Sync + 'static,
            {}
        };
        let input: ItemImpl = parse2(input).unwrap();
        let attribute = TokenStream::new();
        let actual = transform(&attribute, input).output.to_string();
        let expected = quote! {
            impl<A, C> Trait<A> for Struct<C>
            where
                A: Clone,
                C: Send + Sync + 'static,
            {}
            impl<A, C> depcon::Provider<dyn Trait<A> > for Struct<C>
            where
                A: Clone ,
                C: Send + Sync + 'static,
            {
                fn provide(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn Trait<A> > {
                    self
                }
            }
        }
        .to_string();

        assert_eq!(actual, expected);
    }
}
