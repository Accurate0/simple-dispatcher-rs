mod request;

pub use self::request::BaseRequest;
pub type Request = Box<dyn BaseRequest + Send + Sync>;
