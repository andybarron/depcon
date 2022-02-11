use crate::*;
use std::{rc::Rc, sync::Arc};

pub trait Injectable: Sized + 'static {
    fn inject(container: &mut Container) -> Result<Self, Error>;
}

impl<T> Injectable for Rc<T>
where
    T: Injectable,
{
    #[tracing::instrument(skip(container))]
    fn inject(container: &mut Container) -> Result<Self, Error> {
        T::inject(container).map(Self::new)
    }
}

impl<T> Injectable for Arc<T>
where
    T: Injectable,
{
    #[tracing::instrument(skip(container))]
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
