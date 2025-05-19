use std::{any::Any, sync::Arc};

use crate::ServiceContext;

pub trait Startable {
    fn start(&self, service: Arc<ServiceContext>) -> impl Future<Output = ()> + Send + Sync;
}

pub trait CreatableServiceModule: ServiceModule + Sized {
    fn new(context: &ServiceContext) -> Self;
}

pub trait ConfigurableServiceModule: ServiceModule + Sized {
    type Config;
    fn new(context: &ServiceContext, config: Self::Config) -> Self;
}

pub trait ServiceModule: Any + Send + Sync {
    fn run(self: Arc<Self>, service: Arc<ServiceContext>)
    -> Result<(), Box<dyn std::error::Error>>;
}

impl dyn ServiceModule {
    pub(crate) fn downcast<T: 'static>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref()
    }
}

impl<T: Startable + Send + Sync + 'static> ServiceModule for T {
    fn run(
        self: Arc<Self>,
        service: Arc<ServiceContext>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tokio::spawn(async move {
            self.start(service).await;
        });

        // TODO: maybe we can do something better?
        // i.e. split Startable in 2 parts (prepare (fallible) and start (infallible))
        Ok(())
    }
}
