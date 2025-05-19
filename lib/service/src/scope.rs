use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use crate::{ConfigurableServiceModule, CreatableServiceModule, ServiceContext, ServiceModule};

pub struct ServiceScope {
    service: Arc<ServiceContext>,
    parent: Option<Arc<ServiceScope>>,
    scoped_modules: HashMap<TypeId, Arc<dyn ServiceModule>>,
    scoped_variables: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

pub struct ServiceScopeBuilder {
    service: Arc<ServiceContext>,
    parent: Option<Arc<ServiceScope>>,
    modules: HashMap<TypeId, Arc<dyn ServiceModule>>,
    variables: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ServiceScopeBuilder {
    pub fn new(service: Arc<ServiceContext>, parent: Option<Arc<ServiceScope>>) -> Self {
        Self {
            service,
            parent,
            modules: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn with_variable<T: Any + Send + Sync>(mut self, value: T) -> Self {
        self.variables.insert(TypeId::of::<T>(), Box::new(value));
        self
    }

    pub fn with_module<T: CreatableServiceModule + 'static>(self) -> Self {
        let module = T::new(self.service.as_ref());
        self.insert_module(module)
    }

    pub fn configure_module<T: ConfigurableServiceModule + 'static>(
        self,
        config: T::Config,
    ) -> Self {
        let module = T::new(self.service.as_ref(), config);
        self.insert_module(module)
    }

    pub fn insert_module<T: ServiceModule + 'static>(mut self, module: T) -> Self {
        self.modules.insert(TypeId::of::<T>(), Arc::new(module));
        self
    }

    pub fn build(self) -> Arc<ServiceScope> {
        Arc::new(ServiceScope {
            service: self.service,
            parent: self.parent,
            scoped_modules: self.modules,
            scoped_variables: self.variables,
        })
    }
}

impl ServiceScope {
    pub fn service(&self) -> &Arc<ServiceContext> {
        &self.service
    }

    pub fn child_scope(self: &Arc<Self>) -> Arc<Self> {
        Arc::new(Self {
            service: Arc::clone(&self.service),
            parent: Some(Arc::clone(self)),
            scoped_modules: HashMap::new(),
            scoped_variables: HashMap::new(),
        })
    }

    pub fn resolve<T: ServiceModule>(&self) -> &T {
        self.scoped_modules
            .get(&TypeId::of::<T>())
            .map(|service| service.downcast().unwrap())
            .unwrap_or_else(|| {
                if let Some(parent) = self.parent.as_ref() {
                    parent.resolve()
                } else {
                    self.service.resolve()
                }
            })
    }

    pub fn fetch<T: Any>(&self) -> Option<&T> {
        self.scoped_variables
            .get(&TypeId::of::<T>())
            .map(|variable| variable.downcast_ref().unwrap())
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.fetch()))
    }
}
