use crate::{api::prelude::*, types::tagged_fields::TaggedFields};

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
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

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct Response {
    /// The correlation ID of this response.
    pub correlation_id: i32,

    /// This response's tagged fields.
    pub tagged_fields: TaggedFields,
}
