use self::prelude::*;
use crate::api::prelude::*;

pub mod request;
pub mod response;

pub mod prelude {
    pub use super::{request::ProduceRequest, response::ProduceResponse, ProduceApi};
}

pub struct ProduceApi;

impl Api for ProduceApi {
    const KEY: ApiKey = ApiKey::Produce;

    type Request = ProduceRequest;
    type Response = ProduceResponse;
}
