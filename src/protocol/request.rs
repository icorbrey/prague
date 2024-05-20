//! Includes types for request headers.
//!
//! See: <https://kafka.apache.org/protocol.html#protocol_messages>

use std::io::Read;

use super::{
    api::ApiKey,
    try_take::{TryReadable, TryTake},
};

/// Represents a request header.
pub struct RequestHeader<const VERSION: usize> {
    /// This request's API key
    pub api_key: ApiKey,

    /// This request's API version.
    pub api_version: i16,

    /// This request's correlation ID.
    pub correlation_id: i32,

    /// This request's client ID.
    pub client_id: Option<String>,
}

impl TryReadable for RequestHeader<0> {
    fn try_read_from<R: Read + TryTake>(mut reader: R) -> Result<Self, String>
    where
        Self: Sized,
    {
        (|| {
            Ok(Self {
                api_key: reader.try_take()?,
                api_version: reader.try_take()?,
                correlation_id: reader.try_take()?,
                client_id: None,
            })
        })()
        .map_err(|e: String| format!("Unable to read request header: {0}", e))
    }
}

impl TryReadable for RequestHeader<1> {
    fn try_read_from<R: Read + TryTake>(mut reader: R) -> Result<Self, String>
    where
        Self: Sized,
    {
        (|| {
            Ok(Self {
                api_key: reader.try_take()?,
                api_version: reader.try_take()?,
                correlation_id: reader.try_take()?,
                client_id: reader.try_take()?,
            })
        })()
        .map_err(|e: String| format!("Unable to read request header: {0}", e))
    }
}
