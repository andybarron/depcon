use crate::*;

/// Error type for this crate.
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum Error {
    /// Could not resolve a service due to a dependency cycle.
    #[error("Could not resolve {service} due to dependency cycle:\n{}", format_type_stack(.stack))]
    DependencyCycle {
        /// The service that was resolved while already resolving itself
        service: TypeInfo,
        /// Full stack of dependency resolutions when the cycle was encountered
        stack: Vec<Resolution>,
    },
    /// Could not resolve a service because no provider was registered for it.
    #[error("No provider registered for service {service}")]
    NoProvider {
        /// The service that was missing a provider
        service: TypeInfo,
    },
    /// Could no register a provider for a service because the service already had
    /// a provider registered.
    #[error(
        "Could not register {} for {} due to conflict with existing provider: {}",
        .rejected_provider,
        .service,
        .registered_provider,
    )]
    DuplicateRegistration {
        /// The service that was double-registered
        service: TypeInfo,
        /// The provider that was previously registered
        registered_provider: TypeInfo,
        /// The provider that could not be registered
        rejected_provider: TypeInfo,
    },
    /// An internal invariant was violated.
    #[error("Internal error: {message}")]
    Internal {
        /// Description of the error
        message: String,
    },
}

fn format_type_stack(stack: &[Resolution]) -> String {
    stack
        .iter()
        .map(|r| format!("{} (as {})", r.provider, r.service))
        .collect::<Vec<_>>()
        .join(" ->\n")
}
