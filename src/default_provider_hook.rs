use crate::*;

/// Static registration info used by [`Container::auto`]. Don't instantiate manually!
///
/// Use [`auto_provide`] instead, or [`auto_register!`] if code generation is disabled.
pub struct DefaultProviderHook(pub fn(&mut Container) -> Result<(), Error>);

impl DefaultProviderHook {
    pub(crate) fn call(&self, container: &mut Container) -> Result<(), Error> {
        (self.0)(container)
    }
}

inventory::collect!(DefaultProviderHook);
