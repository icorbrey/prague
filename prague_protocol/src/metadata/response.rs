use crate::{error_code::ErrorCode, types::prelude::*};

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct MetadataResponse {
    /// If the associated fetch request violated a quota, the duration for which the request was
    /// throttled. Otherwise, None.
    pub throttle_time: Option<Duration>,

    /// The list of brokers.
    pub brokers: Vec<BrokerMetadata>,
    pub cluster_id: Option<String>,
    pub controller_id: i32,
    pub topics: Vec<TopicMetadata>,
    pub tagged_fields: TaggedFields,
}

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct BrokerMetadata {
    /// This broker's ID.
    pub id: BrokerId,

    /// This broker's hostname.
    pub hostname: String,

    /// This broker's port.
    pub port: i32,

    /// This broker's rack, if it has been assigned to one. Otherwise, None.
    pub rack: Option<String>,

    /// This broker's tagged fields.
    pub tagged_fields: TaggedFields,
}

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct TopicMetadata {
    pub error_code: Option<ErrorCode>,
    pub name: Option<String>,
    pub id: Uuid,
    pub is_internal: bool,
    pub partitions: Vec<PartitionMetadata>,
    pub authorized_operations: i32,
    pub tagged_fields: TaggedFields,
}

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct PartitionMetadata {
    pub error_code: Option<ErrorCode>,
    pub index: PartitionIndex,
    pub leader_id: i32,
    pub leader_epoch: Epoch,
    pub replica_nodes: Vec<BrokerId>,
    pub isr_nodes: Vec<BrokerId>,
    pub offline_replicas: Vec<BrokerId>,
    pub tagged_fields: TaggedFields,
}
