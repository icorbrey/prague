pub mod request;
pub mod response;

use self::prelude::*;
use crate::api::prelude::*;

pub mod prelude {
    pub use super::{request::ListOffsetsRequest, response::ListOffsetsResponse, ListOffsetsApi};
}

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct ListOffsetsApi;

impl Api for ListOffsetsApi {
    const KEY: ApiKey = ApiKey::ListOffsets;

    type Request = ListOffsetsRequest;
    type Response = ListOffsetsResponse;
}
