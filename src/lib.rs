mod dispatcher;
mod executor;

pub use dispatcher::RouteDispatcher;
pub use executor::Executor;
pub use executor::ExecutorError;
pub use executor::ExecutorResult;

#[cfg(test)]
mod tests {
    use crate::{executor::ExecutorResult, Executor, RouteDispatcher};
    use async_trait::async_trait;

    struct Context;
    struct Response {
        code: i64,
    }
    struct Fallback;
    struct Root;

    #[derive(Hash, PartialEq, Eq)]
    struct TestRequest;
    pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    fn success() -> Result<Response, Error> {
        Ok(Response { code: 2 })
    }

    fn error() -> Result<Response, Error> {
        Err(Box::new(std::io::Error::from_raw_os_error(2)))
    }

    #[async_trait]
    impl Executor<Context, TestRequest, Response> for Fallback {
        async fn execute(&self, _ctx: &Context, _request: &TestRequest) -> ExecutorResult<Response> {
            error()
        }
    }

    #[async_trait]
    impl Executor<Context, TestRequest, Response> for Root {
        async fn execute(&self, _ctx: &Context, _request: &TestRequest) -> ExecutorResult<Response> {
            success()
        }
    }

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn route_dispatcher() {
        let ctx = Context {};
        let dispatcher = RouteDispatcher::new(ctx, Fallback).add_route("/", Root);

        let response = aw!(dispatcher.dispatch(&TestRequest, || Some("/".to_string())));
        assert_eq!(response.unwrap().code, 2);
    }

    #[test]
    fn route_dispatcher_fallback() {
        let ctx = Context {};
        let dispatcher = RouteDispatcher::new(ctx, Fallback).add_route("/", Root);
        let failed = aw!(dispatcher.dispatch(&TestRequest, || Some("/2222".to_string())));
        assert!(failed.is_err());
    }
}
