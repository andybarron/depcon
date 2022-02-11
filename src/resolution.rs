use crate::*;

/// Debug type representing a resolved service.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Resolution {
    /// The service that was resolved
    pub service: TypeInfo,
    /// The service's underlying provider type
    pub provider: TypeInfo,
}
