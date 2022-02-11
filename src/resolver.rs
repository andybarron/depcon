use crate::*;
use std::sync::Arc;

pub trait Resolver {
    fn resolve<T>(&mut self) -> Result<Arc<T>, Error>
    where
        T: ?Sized + 'static;
}
