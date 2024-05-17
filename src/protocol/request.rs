//! Includes types for request headers.
//!
//! See: <https://kafka.apache.org/protocol.html#protocol_messages>

use super::api::ApiKey;

/// Represents a request header.
pub struct RequestHeader {
    /// This request's API key
    pub api_key: ApiKey,

    /// This request's API version.
    pub api_version: i16,

    /// This request's correlation ID.
    pub correlation_id: i32,

    /// This request's client ID.
    pub client_id: Option<String>,
}
