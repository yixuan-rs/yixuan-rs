use std::{
    any::{TypeId, type_name},
    collections::HashMap,
    sync::Arc,
};

pub mod config;
pub mod network;

mod module;
mod scope;
pub use module::*;
pub use scope::*;

#[derive(Default)]
pub struct ServiceContext {
    modules: HashMap<TypeId, Arc<dyn ServiceModule>>,
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("failed to start a module: {0}")]
    ModuleStartup(Box<dyn std::error::Error>),
}

impl ServiceContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve<T: ServiceModule + 'static>(&self) -> &T {
        self.modules
            .get(&TypeId::of::<T>())
            .map(|service| service.downcast().unwrap())
            .unwrap_or_else(|| {
                panic!(
                    "ServiceContext::resolve - service of type {} not found",
                    type_name::<T>(),
                )
            })
    }

    pub fn start(self) -> Result<Arc<Self>, ServiceError> {
        let context = Arc::new(self);

        for (_, module) in context.modules.iter() {
            Arc::clone(module)
                .run(Arc::clone(&context))
                .map_err(ServiceError::ModuleStartup)?;
        }

        Ok(context)
    }

    pub fn new_scope(self: &Arc<Self>) -> ServiceScopeBuilder {
        ServiceScopeBuilder::new(Arc::clone(self), None)
    }

    pub fn with_module<T: CreatableServiceModule + 'static>(self) -> Self {
        let module = T::new(&self);
        self.insert_module(module)
    }

    pub fn configure_module<T: ConfigurableServiceModule + 'static>(
        self,
        config: T::Config,
    ) -> Self {
        let module = T::new(&self, config);
        self.insert_module(module)
    }

    pub fn insert_module<T: ServiceModule + 'static>(mut self, module: T) -> Self {
        self.modules.insert(TypeId::of::<T>(), Arc::new(module));
        self
    }
}
