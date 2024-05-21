use crate::api_key::ApiKey;

pub struct Request<'a> {
    /// The API key of this request.
    pub api_key: ApiKey,

    /// The API version of this request.
    pub api_version: i16,

    /// The correlation ID of this request.
    pub correlation_id: i32,

    /// The client ID string.
    pub client_id: &'a str,
}

pub struct Response {
    /// The correlation ID of this response.
    pub correlation_id: i32,
}
