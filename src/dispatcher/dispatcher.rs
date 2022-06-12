use crate::{DispatcherResult, Executor};
use std::{collections::HashMap, option::Option, sync::Arc};

pub struct Dispatcher<TCtx, TRequest, TResponse> {
    context: TCtx,
    fallback: Arc<dyn Executor<TCtx, TRequest, TResponse> + Send + Sync>,
    routes: HashMap<String, Arc<dyn Executor<TCtx, TRequest, TResponse> + Send + Sync>>,
}

impl<TCtx, TRequest, TResponse> Dispatcher<TCtx, TRequest, TResponse> {
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

    async fn execute_fallback(&self, request: &TRequest) -> DispatcherResult<TResponse> {
        self.fallback.execute(&request, &self.context).await
    }

    pub async fn dispatch<TFunc>(
        &self,
        request: &TRequest,
        get_path: TFunc,
    ) -> DispatcherResult<TResponse>
    where
        TFunc: Fn() -> Option<String>,
    {
        if let Some(resource_path) = get_path() {
            if let Some(route) = self.routes.get(&resource_path) {
                route.execute(&request, &self.context).await
            } else {
                self.execute_fallback(&request).await
            }
        } else {
            self.execute_fallback(&request).await
        }
    }
}
