#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Isolation level `{0}` was not valid")]
    InvalidIsolationLevel(i8),
}

/// Controls the visibility of transactional records.
pub enum IsolationLevel {
    /// Makes all records visible.
    ReadUncommitted,

    /// Makes only non-transactional and committed transactional records visible.
    ///
    /// Under `ReadCommitted`, all data is returned from offsets smaller than the current LSO
    /// (last stable offset) and the list of aborted transactions is included in the result,
    /// allowing consumers to discard aborted transactional records.
    ReadCommitted,
}

impl TryFrom<i8> for IsolationLevel {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ReadUncommitted),
            1 => Ok(Self::ReadCommitted),
            level => Err(Error::InvalidIsolationLevel(level)),
        }
    }
}
