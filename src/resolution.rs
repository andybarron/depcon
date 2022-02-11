use crate::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Resolution {
    pub service: TypeInfo,
    pub provider: TypeInfo,
}
