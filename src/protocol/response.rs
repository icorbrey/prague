use std::io::Read;

use super::try_take::{TryReadable, TryTake};

pub struct Response<const VERSION: usize> {
    pub correlation_id: i32,
}

impl TryReadable for Response<0> {
    fn try_read_from<R: Read + TryTake>(mut reader: R) -> Result<Self, String>
    where
        Self: Sized,
    {
        (|| {
            Ok(Self {
                correlation_id: reader.try_take()?,
            })
        })()
        .map_err(|e: String| format!("Unable to read response header: {0}", e))
    }
}
