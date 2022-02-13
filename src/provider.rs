use std::sync::Arc;

/// Trait for types providing a service. Don't implement manually!
///
/// Use [`crate::provide`] instead, or [`provide_trait!`] if code generation is disabled.
pub trait Provider<Service: ?Sized>: 'static {
    /// Transform the provider into an instance of the service.
    fn provide(self: Arc<Self>) -> Arc<Service>;
}

impl<T> Provider<Self> for T
where
    T: 'static,
{
    fn provide(self: Arc<Self>) -> Arc<Self> {
        self
    }
}
