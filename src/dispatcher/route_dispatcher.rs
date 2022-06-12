use crate::{DispatcherResult, DynamicExecutor, Executor};
use std::{collections::HashMap, option::Option, sync::Arc};

pub struct RouteDispatcher<TCtx, TRequest, TResponse> {
    context: TCtx,
    fallback: DynamicExecutor<TCtx, TRequest, TResponse>,
    routes: HashMap<String, DynamicExecutor<TCtx, TRequest, TResponse>>,
}

impl<TCtx, TRequest, TResponse> RouteDispatcher<TCtx, TRequest, TResponse> {
    pub fn new<E: 'static>(context: TCtx, fallback: E) -> Self
    where
        E: Executor<TCtx, TRequest, TResponse> + Send + Sync,
    {
        Self {
            routes: HashMap::new(),
            context,
            fallback: Arc::new(fallback),
        }
    }

    pub fn add_route<E: 'static>(mut self, path: &str, executor: E) -> Self
    where
        E: Executor<TCtx, TRequest, TResponse> + Send + Sync,
    {
        self.routes.insert(path.to_string(), Arc::new(executor));
        self
    }

    pub async fn dispatch<TFunc>(&self, request: &TRequest, get_path: TFunc) -> DispatcherResult<TResponse>
    where
        TFunc: Fn() -> Option<String>,
    {
        if let Some(resource_path) = get_path() {
            if let Some(route) = self.routes.get(&resource_path) {
                route.execute(&self.context, &request).await
            } else {
                self.execute_fallback(&request).await
            }
        } else {
            self.execute_fallback(&request).await
        }
    }

    async fn execute_fallback(&self, request: &TRequest) -> DispatcherResult<TResponse> {
        self.fallback.execute(&self.context, &request).await
    }
}
