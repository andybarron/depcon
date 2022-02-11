use crate::*;

pub struct DefaultProviderHook(pub fn(&mut Container) -> Result<(), Error>);

impl DefaultProviderHook {
    pub(crate) fn call(&self, container: &mut Container) -> Result<(), Error> {
        (self.0)(container)
    }
}

inventory::collect!(DefaultProviderHook);
