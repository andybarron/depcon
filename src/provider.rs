use std::sync::Arc;

/// Trait for types providing a service. Don't implement manually!
///
/// Use [`crate::provide`] instead, or [`impl_provider!`] if code generation is disabled.
pub trait Provider<Service: ?Sized>: 'static {
    /// Transform the provider into an instance of the service.
    fn provide(self: Arc<Self>) -> Arc<Service>;
}
