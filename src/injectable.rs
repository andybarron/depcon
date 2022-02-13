use crate::*;

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

// TODO: Someday, with specialization :(
// impl<T> Injectable for T
// where
//     T: Default + 'static,
// {
//     default fn inject(_container: &mut Container) -> Result<Self, Error> {
//         Ok(Self::default())
//     }
// }
