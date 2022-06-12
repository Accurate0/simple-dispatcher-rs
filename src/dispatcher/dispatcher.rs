use crate::Executor;
use std::{collections::HashMap, ops::Try, option::Option, sync::Arc};

pub struct Dispatcher<TCtx, TRequest, TResponse>
where
    TResponse: Try + Try<Output = TResponse>,
{
    context: TCtx,
    fallback: Arc<dyn Executor<TCtx, TRequest, TResponse> + Send + Sync>,
    routes: HashMap<String, Arc<dyn Executor<TCtx, TRequest, TResponse> + Send + Sync>>,
}

impl<TCtx, TRequest, TResponse> Dispatcher<TCtx, TRequest, TResponse>
where
    TResponse: Try + Try<Output = TResponse>,
{
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

    async fn execute_fallback(&self, request: &TRequest) -> TResponse {
        self.fallback.execute(&request, &self.context).await
    }

    pub async fn dispatch<TFunc, TOut>(&self, request: &TRequest, get_path: TFunc) -> TResponse
    where
        TFunc: Fn() -> Option<String>,
    {
        if let Some(resource_path) = get_path() {
            if let Some(route) = self.routes.get(&resource_path) {
                route.execute(&request, &self.context).await?
            } else {
                self.execute_fallback(&request).await?
            }
        } else {
            self.execute_fallback(&request).await?
        }
    }
}
