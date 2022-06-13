use std::sync::Arc;
mod executor;

pub(crate) type DynamicExecutor<TCtx, TRequest, TResponse> = Arc<dyn Executor<TCtx, TRequest, TResponse> + Send + Sync>;
pub use executor::Executor;

pub type ExecutorError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type ExecutorResult<T> = Result<T, ExecutorError>;
