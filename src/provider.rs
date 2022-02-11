use std::sync::Arc;

pub trait Provider<Service: ?Sized>: 'static {
    fn provide(self: Arc<Self>) -> Arc<Service>;
}
