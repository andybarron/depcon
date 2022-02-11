#![warn(
    missing_docs,
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    clippy::cargo
)]
#![allow(clippy::wildcard_imports)]
#![doc = include_str!("../README.md")]
mod container;
mod default_provider_hook;
mod error;
mod injectable;
mod macros;
mod provider;
mod resolution;
mod type_info;

// Used by auto_register! macro
pub use inventory;

/// Prelude module for guilt-free glob imports
pub mod prelude {
    pub use crate::Container;
    pub use crate::Injectable;

    #[cfg(feature = "codegen")]
    pub use depcon_codegen::*;
}

pub use crate::container::Container;
pub use crate::default_provider_hook::DefaultProviderHook;
pub use crate::error::Error;
pub use crate::injectable::Injectable;
pub use crate::provider::Provider;
pub use crate::resolution::Resolution;
pub use crate::type_info::TypeInfo;

#[cfg(feature = "codegen")]
pub use depcon_codegen::*;
