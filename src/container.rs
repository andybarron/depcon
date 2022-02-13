use crate::*;
use std::{any::Any, collections::HashMap, sync::Arc};

/// Dependency injection container where the magic happens.
///
/// This struct holds information about what provider types provide what
/// services and how to initialize them.
#[derive(Default, Debug)]
pub struct Container {
    provider_factories: HashMap<TypeInfo, ProviderFactory>,
    providers: HashMap<TypeInfo, DynamicBox>, // provider type -> Box<Arc<Provider>>
    services: HashMap<TypeInfo, DynamicBox>,  // service type -> Box<Arc<Service>>
    provide_map: HashMap<TypeInfo, (TypeInfo, ServiceConverter)>, // service -> provider
    init_stack: Vec<Resolution>,
}

type DynamicBox = Box<dyn Any>;

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
struct ProviderFactory(
    #[derivative(Debug = "ignore")] Arc<dyn Fn(&mut Container) -> Result<DynamicBox, Error>>,
);

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
struct ServiceConverter(
    #[derivative(Debug = "ignore")] Arc<dyn Fn(&DynamicBox) -> Result<DynamicBox, Error>>,
);

impl Container {
    /// Create a container with all providers pre-registered from
    /// [`auto_provide`] and [`auto_register!`].
    ///
    /// # Errors
    /// This function fails if multiple providers are auto-registered
    /// for a single service type.
    pub fn auto() -> Result<Self, Error> {
        let mut container = Self::empty();
        for hook in inventory::iter::<DefaultProviderHook>() {
            hook.call(&mut container)?;
        }
        Ok(container)
    }
    /// Create an empty container. Useful for testing and manual registration.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }
    fn init_provider(&mut self, res: Resolution) -> Result<&DynamicBox, Error> {
        let cycle = self.init_stack.contains(&res);
        self.init_stack.push(res);

        // do main work inside closure to avoid early returns leaving init stack
        // in a weird state
        let init = &mut || -> Result<DynamicBox, Error> {
            if cycle {
                return Err(Error::DependencyCycle {
                    service: res.service,
                    stack: self.init_stack.clone(),
                });
            }
            let factory = self
                .provider_factories
                .get(&res.provider)
                .ok_or_else(|| {
                    let provider = res.provider;
                    let service = res.service;
                    Error::Internal {
                        message: format!("No factory for provider {provider} (service: {service})"),
                    }
                })?
                .clone();
            let provider = (factory.0)(self)?;
            Ok(provider)
        };
        let result = init();
        self.init_stack.pop();
        let provider = result?;

        // slightly hacky workaround to insert a value into a HashMap
        // and also return a reference to that same value:
        // delete the value from the map, then use the entry API's
        // `or_insert` method, since we know the entry is now empty
        self.providers.remove(&res.provider);
        let entry = self.providers.entry(res.provider);
        Ok(entry.or_insert(provider))
    }
    fn init_service(&mut self, service_type: TypeInfo) -> Result<&DynamicBox, Error> {
        let (provider_type, converter) =
            self.provide_map
                .get(&service_type)
                .cloned()
                .ok_or(Error::NoProvider {
                    service: service_type,
                })?;

        let resolution = Resolution {
            service: service_type,
            provider: provider_type,
        };

        let provider = match self.providers.get(&provider_type) {
            Some(ptr) => ptr,
            None => self.init_provider(resolution)?,
        };

        let service = (converter.0)(provider)?;

        // slightly hacky workaround to insert a value into a HashMap
        // and also return a reference to that same value:
        // delete the value from the map, then use the entry API's
        // `or_insert` method, since we know the entry is now empty
        self.services.remove(&service_type);
        let entry = self.services.entry(service_type);
        Ok(entry.or_insert(service))
    }
}

impl Container {
    /// Register type `TProvider` as the provider for type `TService`.
    ///
    /// # Errors
    /// This method fails if a provider is already registered for `TService`.
    pub fn register<TProvider, TService: ?Sized>(&mut self) -> Result<(), Error>
    where
        TProvider: Injectable + Provider<TService>,
        TService: 'static,
    {
        let service_type = TypeInfo::of::<TService>();
        let provider_type = TypeInfo::of::<TProvider>();
        if let Some((prev_provider_type, _)) = self.provide_map.get(&service_type) {
            return Err(Error::DuplicateRegistration {
                service: service_type,
                registered_provider: *prev_provider_type,
                rejected_provider: provider_type,
            });
        }
        self.register_overwrite::<TProvider, TService>();
        Ok(())
    }
    /// Same as [`Container::register`], but overwrites existing registrations.
    pub fn register_overwrite<TProvider, TService: ?Sized>(&mut self)
    where
        TProvider: Injectable + Provider<TService>,
        TService: 'static,
    {
        let service_type = TypeInfo::of::<TService>();
        let provider_type = TypeInfo::of::<TProvider>();

        // always allow resolving concrete provider types
        if service_type != provider_type && !self.provider_factories.contains_key(&provider_type) {
            self.register_overwrite::<TProvider, TProvider>();
        }

        self.provider_factories
            .entry(provider_type)
            .or_insert_with(|| {
                ProviderFactory(Arc::new(|c| {
                    let instance = TProvider::inject(c)?;
                    let arc: Arc<TProvider> = Arc::new(instance);
                    Ok(Box::new(arc))
                }))
            });

        self.provide_map.insert(
            service_type,
            (
                provider_type,
                // TODO: Fix funky coverage results for format! macro and downcast_ref method.
                //       https://github.com/xd009642/tarpaulin/issues/351
                ServiceConverter(Arc::new(move |any| {
                    let provider = any
                        .downcast_ref::<Arc<TProvider>>()
                        .ok_or({
                            let box_type = TypeInfo::of::<DynamicBox>();
                            Error::Internal {
                                message: format!(
                                    "Failed to downcast provider {box_type} to Arc<{service_type}>"
                                ),
                            }
                        })?
                        .clone();
                    let service: Arc<TService> = provider.provide();
                    Ok(Box::new(service))
                })),
            ),
        );
    }
    /// Resolve an instance of type `T`.
    ///
    /// # Errors
    /// This method fails if no provider has been registered for `T` or
    /// any of its transitive dependencies.
    pub fn resolve<T>(&mut self) -> Result<Arc<T>, Error>
    where
        T: ?Sized + 'static,
    {
        let service_type = TypeInfo::of::<T>();
        let service_ptr = match self.services.get(&service_type) {
            Some(ptr) => ptr,
            None => self.init_service(service_type)?,
        };

        let service_ptr = service_ptr.downcast_ref::<Arc<T>>().ok_or_else(|| {
            let box_type = TypeInfo::of::<DynamicBox>();
            Error::Internal {
                message: format!("Failed to downcast service {box_type} to Arc<{service_type}>"),
            }
        })?;
        Ok(service_ptr.clone())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use crate::*;
    // TODO: this is a hack. see import_crate in codegen/src/utils.rs
    use crate as depcon;
    use std::sync::Arc;

    #[test]
    fn test_register_duplicate() {
        #[derive(Injectable)]
        struct Service;

        let mut container = Container::empty();
        container.register::<Service, Service>().unwrap();
        let actual = container
            .register::<Service, Service>()
            .unwrap_err()
            .to_string();
        let expected = "Could not register \
            depcon::container::test::test_register_duplicate::Service for \
            depcon::container::test::test_register_duplicate::Service due to \
            conflict with existing provider: \
            depcon::container::test::test_register_duplicate::Service";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_service_converter_failure() {
        #[derive(Injectable, Debug)]
        struct Service;
        let type_info = TypeInfo::of::<Service>();

        let mut container = Container::empty();
        container.register::<Service, Service>().unwrap();
        container
            .providers
            .insert(type_info, Box::new(Arc::new(0_u8)));

        let actual = container.resolve::<Service>().unwrap_err().to_string();
        let expected = "Internal error: Failed to downcast provider alloc::boxed::Box<dyn core::any::Any> to Arc<depcon::container::test::test_service_converter_failure::Service>";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_service_downcast_failure() {
        #[derive(Injectable, Debug)]
        struct Service;
        let type_info = TypeInfo::of::<Service>();

        let mut container = Container::empty();
        container.register::<Service, Service>().unwrap();
        container
            .services
            .insert(type_info, Box::new(Arc::new(0_u8)));

        let actual = container.resolve::<Service>().unwrap_err().to_string();
        let expected = "Internal error: Failed to downcast service alloc::boxed::Box<dyn core::any::Any> to Arc<depcon::container::test::test_service_downcast_failure::Service>";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_register_service_resolve_provider() {
        trait Interface {}

        #[derive(Injectable, Debug, PartialEq)]
        struct Implementation;

        impl Interface for Implementation {}
        provide_trait!(Implementation, dyn Interface);

        let mut container = Container::empty();
        container
            .register::<Implementation, dyn Interface>()
            .unwrap();

        let actual = container.resolve::<Implementation>();
        let expected = Ok(Arc::new(Implementation));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_missing_provider_factory() {
        #[derive(Injectable, Debug)]
        struct Service;

        let mut container = Container::empty();
        container.register::<Service, Service>().unwrap();
        container.provider_factories.clear();

        let error = container.resolve::<Service>().unwrap_err();
        let actual = format!("{}", error);
        let expected = "Internal error: No factory for provider \
            depcon::container::test::test_missing_provider_factory::Service \
            (service: depcon::container::test::test_missing_provider_factory::Service)";

        assert_eq!(actual, expected);
    }
}
