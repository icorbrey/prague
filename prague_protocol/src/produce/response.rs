//! Includes definitions for responses from [ProduceApi](super::ProduceApi).
//!
//! ## See also:
//!
//! - <https://kafka.apache.org/protocol.html#protocol_messages>
//! - <https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceResponse.json>

use crate::{error_code::ErrorCode, types::prelude::TaggedFields};

/// A response message associated with [ProduceRequest](super::request::ProduceRequest).
///
/// ## See also:
///
/// - <https://kafka.apache.org/protocol.html#protocol_messages>
/// - <https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceResponse.json>
pub struct ProduceResponse {
    /// Each produce response.
    ///
    /// Versions: 0+
    pub topics: Vec<TopicProduceResult>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation,
    /// or None if the request did not violate any quota.
    ///
    /// Versions: 1+
    pub throttle_time: Option<i32>,

    /// Endpoints for all current-leaders enumerated in [PartitionProduceResult], with errors
    /// [NotLeaderOrFollower](crate::error_code::ErrorCode::NotLeaderOrFollower).
    ///
    /// Versions: 10+
    pub endpoints: Vec<NodeResult>,

    /// The tagged fields.
    ///
    /// Versions: 9+
    pub tagged_fields: TaggedFields,
}

/// Describes the result of producing to a topic.
///
/// ## See also:
///
/// - <https://kafka.apache.org/protocol.html#protocol_messages>
/// - <https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceResponse.json>
pub struct TopicProduceResult {
    /// The topic name.
    ///
    /// Version: 0+
    pub name: String,

    /// Each partition that we produced to within the topic.
    ///
    /// Versions: 0+
    pub partitions: Vec<PartitionProduceResult>,

    /// The tagged fields.
    ///
    /// Versions: 9+
    pub tagged_fields: TaggedFields,
}

/// Describes the result of producing to a partition.
///
/// ## See also:
///
/// - <https://kafka.apache.org/protocol.html#protocol_messages>
/// - <https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceResponse.json>
pub struct PartitionProduceResult {
    /// The partition index.
    ///
    /// Versions: 0+
    pub index: i32,

    /// The error code, or None if there was no error.
    ///
    /// Versions: 0+
    pub error_code: Option<ErrorCode>,

    /// The base offset.
    ///
    /// Versions: 0+
    pub base_offset: i64,

    /// The timestamp returned by the broker after appending the messages. If CreateTime is used
    /// for the topic, the timestamp will be None. If LogAppendTime is used for the topic, the
    /// timestamp will be the broker local time when the messages are appended.
    ///
    /// Versions: 2+
    pub log_append_time: Option<i64>,

    /// The log start offset.
    ///
    /// Versions: 5+
    pub log_start_offset: Option<i64>,

    /// The batch indices of records that caused the batch to be dropped.
    ///
    /// Versions: 8+
    pub record_errors: Vec<BatchError>,

    /// The global error message summarizing the common root cause of the records that caused the
    /// batch to be dropped.
    ///
    /// Versions: 8+
    pub error_message: Option<String>,

    /// The current leader.
    ///
    /// Versions: 10+
    pub current_leader: Option<CurrentLeaderResult>,

    /// The tagged fields.
    ///
    /// Versions: 9+
    pub tagged_fields: TaggedFields,
}

/// Describes what batch was dropped and why.
///
/// ## See also:
///
/// - <https://kafka.apache.org/protocol.html#protocol_messages>
/// - <https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceResponse.json>
pub struct BatchError {
    /// The batch index of the record that caused the batch to be dropped.
    ///
    /// Versions: 8+
    pub index: i32,

    /// The error message of the record that caused the batch to be dropped.
    ///
    /// Versions: 8+
    pub message: Option<String>,

    /// The tagged fields.
    ///
    /// Versions: 9+
    pub tagged_fields: TaggedFields,
}

/// Describes the current leader.
///
/// ## See also:
///
/// - <https://kafka.apache.org/protocol.html#protocol_messages>
/// - <https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceResponse.json>
pub struct CurrentLeaderResult {
    /// The ID of the current leader, or None if the leader is unknown.
    ///
    /// Versions: 10+
    pub id: Option<i32>,

    /// The latest known leader epoch, or None if the epoch is unknown.
    pub epoch: Option<i32>,
}

/// Describes a broker that hosts partitions of topics.
///
/// ## See also:
///
/// - <https://kafka.apache.org/protocol.html#protocol_messages>
/// - <https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceResponse.json>
pub struct NodeResult {
    /// The ID of the associated node.
    ///
    /// Versions: 10+
    pub id: i32,

    /// The node's hostname.
    ///
    /// Versions: 10+
    pub hostname: String,

    /// The node's port.
    ///
    /// Versions: 10+
    pub port: i32,

    /// The node's rack, or None if it has not been assigned to a rack.
    ///
    /// Version: 10+
    pub rack: Option<String>,
}
