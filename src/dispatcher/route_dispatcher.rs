use crate::{
    executor::{DynamicExecutor, ExecutorResult, RouteProtector},
    Executor,
};
use std::{collections::HashMap, option::Option, sync::Arc};

// TODO: add concept of methods
pub struct RouteDispatcher<TContext, TRequest, TResponse> {
    context: TContext,
    fallback: DynamicExecutor<TContext, TRequest, TResponse>,
    protected_route_fallback: DynamicExecutor<TContext, TRequest, TResponse>,
    route_protector: Option<RouteProtector<TContext, TRequest>>,
    routes: HashMap<String, DynamicExecutor<TContext, TRequest, TResponse>>,
    protected_routes: HashMap<String, DynamicExecutor<TContext, TRequest, TResponse>>,
}

impl<TContext, TRequest, TResponse> RouteDispatcher<TContext, TRequest, TResponse> {
    pub fn new<E: 'static, E1: 'static>(context: TContext, fallback: E, protected_route_fallback: E1) -> Self
    where
        E: Executor<TContext, TRequest, TResponse> + Send + Sync,
        E1: Executor<TContext, TRequest, TResponse> + Send + Sync,
    {
        Self {
            routes: HashMap::new(),
            protected_routes: HashMap::new(),
            context,
            route_protector: None,
            fallback: Arc::new(fallback),
            protected_route_fallback: Arc::new(protected_route_fallback),
        }
    }

    pub fn add_route<E: 'static>(mut self, path: &str, executor: E) -> Self
    where
        E: Executor<TContext, TRequest, TResponse> + Send + Sync,
    {
        self.routes.insert(path.to_string(), Arc::new(executor));
        self
    }

    pub fn add_protected_route<E: 'static>(mut self, path: &str, executor: E) -> Self
    where
        E: Executor<TContext, TRequest, TResponse> + Send + Sync,
    {
        self.protected_routes.insert(path.to_string(), Arc::new(executor));
        self
    }

    pub fn add_router_protector<E: 'static>(mut self, protector: E) -> Self
    where
        E: (Fn(&str, &TContext, &TRequest) -> bool) + Send + Sized + Sync,
    {
        self.route_protector = Some(Arc::new(protector));
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
                if let Some(route) = self.protected_routes.get(&resource_path) {
                    if let Some(route_protector) = &self.route_protector {
                        if route_protector(&resource_path, &self.context, request) {
                            return route.execute(&self.context, request).await;
                        } else {
                            return self.protected_route_fallback.execute(&self.context, request).await;
                        }
                    }
                }

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
