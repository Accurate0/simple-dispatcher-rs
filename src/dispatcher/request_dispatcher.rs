use crate::{
    executor::{DynamicExecutor, ExecutorResult},
    request::BaseRequest,
    Executor, Request,
};
use std::{collections::HashMap, sync::Arc};

pub struct RequestDispatcher<TContext, TRequest, TResponse> {
    context: TContext,
    fallback: DynamicExecutor<TContext, TRequest, TResponse>,
    request_map: HashMap<Request, DynamicExecutor<TContext, TRequest, TResponse>>,
}

impl<TContext, TResponse> RequestDispatcher<TContext, Request, TResponse> {
    pub fn new<E: 'static>(context: TContext, fallback: E) -> Self
    where
        E: Executor<TContext, Request, TResponse> + Send + Sync,
    {
        Self {
            context,
            fallback: Arc::new(fallback),
            request_map: HashMap::new(),
        }
    }

    pub fn add_request<R: 'static, E: 'static>(mut self, request: R, executor: E) -> Self
    where
        R: BaseRequest + Send + Sync,
        E: Executor<TContext, Request, TResponse> + Send + Sync,
    {
        self.request_map.insert(Box::new(request), Arc::new(executor));
        self
    }

    pub async fn dispatch(&self, request: Request) -> ExecutorResult<TResponse> {
        if let Some(executor) = self.request_map.get(&request) {
            executor.execute(&self.context, &request).await
        } else {
            self.fallback.execute(&self.context, &request).await
        }
    }
}
