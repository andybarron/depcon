/// Implement `[Provider]<Service>` for a type. Prefer [`crate::provide`] if
/// codegen is enabled.
#[macro_export]
macro_rules! provide_trait {
    ($provider: ty, $tr: ty) => {
        impl $crate::Provider<$tr> for $provider {
            fn provide(self: std::sync::Arc<Self>) -> std::sync::Arc<$tr> {
                self
            }
        }
    };
}

/// Flag a type as the default provider for a service when
/// [`crate::Container::auto`] is used. Prefer [`crate::auto_provide`] if
/// codegen is enabled.
#[macro_export]
macro_rules! auto_register {
    ($provider: ty, $service: ty) => {
        $crate::inventory::submit! {
            $crate::DefaultProviderHook(|c| {
                c.register::<$provider, $service>()
            })
        }
    };
    ($provider: ty) => {
        $crate::auto_register!($provider, $provider)
    };
}
