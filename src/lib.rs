#![warn(clippy::all, clippy::nursery)]
#![doc = include_str!("../README.md")]
mod container;
mod default_provider_hook;
mod error;
mod injectable;
mod macros;
mod provider;
mod resolution;
mod resolver;
mod sync_container;
mod type_info;

// Used by register_default! macro
pub use inventory;

pub mod prelude {
    pub use crate::Container;
    pub use crate::Injectable;
    pub use crate::Resolver;
    pub use crate::SyncContainer;

    #[cfg(feature = "codegen")]
    pub use depcon_codegen::*;
}

pub use crate::container::Container;
pub use crate::default_provider_hook::DefaultProviderHook;
pub use crate::error::Error;
pub use crate::injectable::Injectable;
pub use crate::provider::Provider;
pub use crate::resolution::Resolution;
pub use crate::resolver::Resolver;
pub use crate::sync_container::SyncContainer;
pub use crate::type_info::TypeInfo;

#[cfg(feature = "codegen")]
pub use depcon_codegen::*;
