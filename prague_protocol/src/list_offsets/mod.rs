pub mod request;
pub mod response;

use self::prelude::*;
use crate::api::prelude::*;

pub mod prelude {
    pub use super::{request::ListOffsetsRequest, response::ListOffsetsResponse, ListOffsetsApi};
}

pub struct ListOffsetsApi;

impl Api for ListOffsetsApi {
    const KEY: ApiKey = ApiKey::ListOffsets;

    type Request = ListOffsetsRequest;
    type Response = ListOffsetsResponse;
}
