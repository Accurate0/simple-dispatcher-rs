use std::sync::Arc;
mod executor;

pub(crate) type DynamicExecutor<TContext, TRequest, TResponse> =
    Arc<dyn Executor<TContext, TRequest, TResponse> + Send + Sync>;
pub use executor::Executor;

pub type ExecutorError = Box<dyn std::error::Error + Send + Sync>;
pub type ExecutorResult<T> = Result<T, ExecutorError>;
