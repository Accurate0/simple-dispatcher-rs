use async_trait::async_trait;

#[async_trait]
pub trait Executor<TCtx, TRequest, TResponse>
where
    TResponse: std::ops::Try,
{
    async fn execute(&self, request: &TRequest, context: &TCtx) -> TResponse;
}
