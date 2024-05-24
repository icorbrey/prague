use self::prelude::*;
use crate::api::prelude::*;

pub mod request;
pub mod response;

pub mod prelude {
    pub use super::{request::FetchRequest, response::FetchResponse, FetchApi};
}

pub struct FetchApi;

impl Api for FetchApi {
    const KEY: ApiKey = ApiKey::Fetch;

    type Request = FetchRequest;
    type Response = FetchResponse;
}
