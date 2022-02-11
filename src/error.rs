use crate::*;

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum Error {
    #[error("Could not resolve {service} due to dependency cycle:\n{}", format_type_stack(.stack))]
    DependencyCycle {
        service: TypeInfo,
        stack: Vec<Resolution>,
    },
    #[error("No provider registered for service {service}")]
    NoProvider { service: TypeInfo },
    #[error(
        "Could not register {} for {} due to conflict with existing provider: {}",
        .rejected_provider,
        .service,
        .registered_provider,
    )]
    DuplicateRegistration {
        service: TypeInfo,
        registered_provider: TypeInfo,
        rejected_provider: TypeInfo,
    },
}

fn format_type_stack(stack: &[Resolution]) -> String {
    stack
        .iter()
        .map(|r| format!("{} (as {})", r.provider, r.service))
        .collect::<Vec<_>>()
        .join(" ->\n")
}
