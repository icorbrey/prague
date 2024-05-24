use crate::{isolation_level::IsolationLevel, primitives::prelude::*};

pub struct ListOffsetsRequest {
    /// If this request is being made by a broker, the ID of the broker. Otherwise, None.
    pub replica_id: Option<BrokerId>,

    /// Controls the visibility of transactional records.
    pub isolation_level: IsolationLevel,

    /// The list of topics to list the offsets of.
    pub topics: Vec<Topic>,

    /// This request's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct Topic {
    /// This topic's name.
    pub name: String,

    /// The list of partitions to list the offsets of.
    pub partitions: Vec<Partition>,

    /// This topic's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct Partition {
    /// The index of this partition.
    pub index: PartitionIndex,

    /// The current leader's epoch.
    pub current_leader_epoch: Epoch,

    /// The current timestamp.
    pub timestamp: Timestamp,

    /// This partition's tagged fields.
    pub tagged_fields: TaggedFields,
}
