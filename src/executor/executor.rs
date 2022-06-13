use async_trait::async_trait;

use super::ExecutorResult;

#[async_trait]
pub trait Executor<TCtx, TRequest, TResponse> {
    async fn execute(&self, context: &TCtx, request: &TRequest) -> ExecutorResult<TResponse>;
}
