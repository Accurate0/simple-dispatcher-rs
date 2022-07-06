use crate::{
    executor::{DynamicExecutor, ExecutorResult},
    Executor,
};
use std::{collections::HashMap, option::Option, sync::Arc};

// TODO: add concept of methods
pub struct RouteDispatcher<TContext, TRequest, TResponse> {
    context: TContext,
    fallback: DynamicExecutor<TContext, TRequest, TResponse>,
    routes: HashMap<String, DynamicExecutor<TContext, TRequest, TResponse>>,
}

impl<TContext, TRequest, TResponse> RouteDispatcher<TContext, TRequest, TResponse> {
    pub fn new<E: 'static>(context: TContext, fallback: E) -> Self
    where
        E: Executor<TContext, TRequest, TResponse> + Send + Sync,
    {
        Self {
            routes: HashMap::new(),
            context,
            fallback: Arc::new(fallback),
        }
    }

    pub fn add_route<E: 'static>(mut self, path: &str, executor: E) -> Self
    where
        E: Executor<TContext, TRequest, TResponse> + Send + Sync,
    {
        self.routes.insert(path.to_string(), Arc::new(executor));
        self
    }

    pub async fn dispatch<TFunc>(&self, request: &TRequest, get_path: TFunc) -> ExecutorResult<TResponse>
    where
        TFunc: Fn() -> Option<String>,
    {
        if let Some(resource_path) = get_path() {
            if let Some(route) = self.routes.get(&resource_path) {
                route.execute(&self.context, request).await
            } else {
                self.execute_fallback(request).await
            }
        } else {
            self.execute_fallback(request).await
        }
    }

    async fn execute_fallback(&self, request: &TRequest) -> ExecutorResult<TResponse> {
        self.fallback.execute(&self.context, request).await
    }
}
