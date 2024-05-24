pub mod request;
pub mod response;

use self::prelude::*;
use crate::api::prelude::*;

pub mod prelude {
    pub use super::{request::MetadataRequest, response::MetadataResponse, MetadataApi};
}

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct MetadataApi;

impl Api for MetadataApi {
    const KEY: ApiKey = ApiKey::Metadata;

    type Request = MetadataRequest;
    type Response = MetadataResponse;
}
