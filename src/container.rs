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
    fn init_provider(&mut self, res: Resolution) -> Result<(), Error> {
        let cycle = self.init_stack.contains(&res);
        self.init_stack.push(res);

        // do main work inside closure to avoid early returns leaving init stack
        // in a weird state
        let init = &mut || -> Result<(), Error> {
            if cycle {
                return Err(Error::DependencyCycle {
                    service: res.service,
                    stack: self.init_stack.clone(),
                });
            }
            let factory = self.provider_factories.get(&res.provider).unwrap().clone();
            let provider = (factory.0)(self)?;
            self.providers.insert(res.provider, provider);
            Ok(())
        };
        let result = init();
        self.init_stack.pop();
        result
    }
    fn init_service(&mut self, service_type: TypeInfo) -> Result<(), Error> {
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

        if !self.providers.contains_key(&provider_type) {
            self.init_provider(resolution)?;
        }

        let provider = self.providers.get(&provider_type).unwrap();
        let service = (converter.0)(provider)?;
        self.services.insert(service_type, service);
        Ok(())
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
        self.provider_factories.insert(
            provider_type,
            ProviderFactory(Arc::new(|c| {
                let instance = TProvider::inject(c)?;
                let arc: Arc<TProvider> = Arc::new(instance);
                Ok(Box::new(arc))
            })),
        );
        self.provide_map.insert(
            service_type,
            (
                provider_type,
                ServiceConverter(Arc::new(move |any| {
                    let provider = any
                        .downcast_ref::<Arc<TProvider>>()
                        .ok_or({
                            let box_type = TypeInfo::of::<DynamicBox>();
                            Error::Internal {
                                message: format!(
                                    "Failed to downcast {box_type} to Arc<{service_type}>"
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
        if !self.services.contains_key(&service_type) {
            self.init_service(service_type)?;
        }

        let service_ptr = self
            .services
            .get(&service_type)
            .ok_or_else(|| Error::Internal {
                message: format!("No service instance for {service_type}"),
            })?;
        let service_ptr = service_ptr.downcast_ref::<Arc<T>>().ok_or_else(|| {
            let box_type = TypeInfo::of::<DynamicBox>();
            Error::Internal {
                message: format!("Failed to downcast {box_type} to Arc<{service_type}>"),
            }
        })?;
        Ok(service_ptr.clone())
    }
}
