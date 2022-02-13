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
    #[cfg(not(tarpaulin_include))]
    fn provide(self: Arc<Self>) -> Arc<Self> {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_all_types_provide_self() {
        struct Service;

        let arc = Arc::new(Service);
        let _arc2: Arc<Service> = arc.provide();
    }
}
