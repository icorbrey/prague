use crate::{error_code::ErrorCode, primitives::prelude::*};

pub struct ListOffsetsResponse {
    /// If the associated fetch request violated a quota, the duration for which the request was
    /// throttled. Otherwise, None.
    pub throttle_time: Option<Duration>,

    /// The list of responses from each topic.
    pub topics: Vec<Topic>,

    /// This response's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct Topic {
    /// This topic's unique ID.
    pub name: String,

    /// The responses from this topic's partitions.
    pub partitions: Vec<Partition>,

    /// This topic's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct Partition {
    /// This partition's index.
    pub index: PartitionIndex,

    /// This partition's error code if something went wrong, otherwise None.
    pub error_code: Option<ErrorCode>,

    /// The timestamp associated with the returned offset.
    pub timestamp: Timestamp,

    /// This partition's current offset.
    pub offset: Offset,

    /// The leader's epoch.
    pub leader_epoch: Epoch,
}
