use std::{
    any::{type_name, TypeId},
    fmt,
    hash::{Hash, Hasher},
};

/// Debug type for identifying services & providers.
///
/// Basically identical to [`std::any::TypeId`], but with the type's name
/// for debugging.
#[derive(derivative::Derivative, Clone, Copy, Eq)]
pub struct TypeInfo {
    /// ID of the type
    pub id: TypeId,
    /// Name of the type
    pub name: &'static str,
}

impl TypeInfo {
    /// Get type info of a type `T`
    #[must_use]
    pub fn of<T: ?Sized + 'static>() -> Self {
        Self {
            id: TypeId::of::<T>(),
            name: type_name::<T>(),
        }
    }
}

impl PartialEq for TypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for TypeInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Debug for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
}
