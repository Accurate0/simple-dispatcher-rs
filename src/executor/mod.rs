use std::sync::Arc;
mod executor_trait;

pub(crate) type ProtectorFunc<TContext, TRequest> = dyn Fn(&str, &TContext, &TRequest) -> bool;
pub(crate) type RouteProtector<TContext, TRequest> = Arc<ProtectorFunc<TContext, TRequest>>;
pub(crate) type DynamicExecutor<TContext, TRequest, TResponse> =
    Arc<dyn Executor<TContext, TRequest, TResponse> + Send + Sync>;
pub use executor_trait::Executor;

pub type ExecutorError = Box<dyn std::error::Error + Send + Sync>;
pub type ExecutorResult<T> = Result<T, ExecutorError>;
