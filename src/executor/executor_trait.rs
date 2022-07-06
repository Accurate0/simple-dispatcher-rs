use async_trait::async_trait;

use super::ExecutorResult;

#[async_trait]
pub trait Executor<TContext, TRequest, TResponse> {
    async fn execute(&self, context: &TContext, request: &TRequest) -> ExecutorResult<TResponse>;
}
