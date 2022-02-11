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

#[macro_export]
macro_rules! auto_register {
    ($provider: ty, $service: ty) => {
        $crate::inventory::submit! {
            $crate::DefaultProviderHook(|c| {
                c.register::<$provider, $service>()
            })
        }
    };
}
