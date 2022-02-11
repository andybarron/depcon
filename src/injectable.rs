use crate::*;
use std::{rc::Rc, sync::Arc};

/// Trait for injecting providers into a container. Use [`macro@Injectable`] instead of
/// implementing manually!
pub trait Injectable: Sized + 'static {
    /// Build an instance of `Self`, using [`Container::resolve`] to
    /// resolve dependencies.
    ///
    /// # Errors
    /// Implementations should forward errors from [`Container::resolve`]
    /// if a dependency can't be resolved.
    fn inject(container: &mut Container) -> Result<Self, Error>;
}

impl<T> Injectable for Rc<T>
where
    T: Injectable,
{
    fn inject(container: &mut Container) -> Result<Self, Error> {
        T::inject(container).map(Self::new)
    }
}

impl<T> Injectable for Arc<T>
where
    T: Injectable,
{
    fn inject(container: &mut Container) -> Result<Self, Error> {
        T::inject(container).map(Self::new)
    }
}

// TODO: Someday, with specialization :(
// impl<T> Injectable for T
// where
//     T: Default + 'static,
// {
//     #[tracing::instrument(skip(_container))]
//     fn inject(_container: &mut Container) -> Result<Self, Error> {
//         Ok(Self::default())
//     }
// }
