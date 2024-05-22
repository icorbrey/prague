#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid acknowledgement requirement `{0}`")]
    InvalidAckRequirement(i16),
}

/// The number of acknowledgments the producer requires the leader to have received before
/// considering a request complete.
#[derive(Debug, PartialEq, Eq)]
pub enum AckRequirement {
    FullISR,
    NoAcknowledgements,
    LeaderOnly,
}

impl TryFrom<i16> for AckRequirement {
    type Error = Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Self::FullISR),
            0 => Ok(Self::NoAcknowledgements),
            1 => Ok(Self::LeaderOnly),
            x => Err(Error::InvalidAckRequirement(x)),
        }
    }
}
