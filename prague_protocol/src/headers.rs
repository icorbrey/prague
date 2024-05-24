use crate::{api::prelude::*, primitives::tagged_fields::TaggedFields};

pub struct Request {
    /// The API key of this request.
    pub api_key: ApiKey,

    /// The API version of this request.
    pub api_version: i16,

    /// The correlation ID of this request.
    pub correlation_id: i32,

    /// The client ID string.
    pub client_id: Option<String>,

    /// This request's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct Response {
    /// The correlation ID of this response.
    pub correlation_id: i32,

    /// This response's tagged fields.
    pub tagged_fields: TaggedFields,
}
