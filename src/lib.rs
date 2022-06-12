mod dispatcher;
mod executor;

pub use dispatcher::Dispatcher;
pub use executor::Executor;

type DispatcherResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
