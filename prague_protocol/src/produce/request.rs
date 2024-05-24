//! Includes definitions for requests to [ProduceApi](super::ProduceApi).
//!
//! ## See also
//!
//! - <https://kafka.apache.org/protocol.html#protocol_messages>
//! - <https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceRequest.json>

use crate::{acks::AckRequirement, records::Records, types::prelude::*};

/// A request to produce data.
///
/// ## See also
///
/// - <https://kafka.apache.org/protocol.html#protocol_messages>
/// - <https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceRequest.json>
pub struct ProduceRequest {
    /// The transactional ID, or None if the producer is not transactional.
    ///
    /// This ID is used for authorization when attempting to write transactional data.
    ///
    /// Versions: 3+
    pub transactional_id: Option<String>,

    /// The number of acknowledgments the producer requires the leader to have received before
    /// considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the
    /// leader and -1 for the full ISR.
    ///
    /// Versions: 0+
    pub acks: AckRequirement,

    /// The timeout to await a response in milliseconds.
    ///
    /// Versions: 0+
    pub timeout: i32,

    /// Each topic to produce to.
    ///
    /// Versions: 0+
    pub topics: Vec<TopicProduceTarget>,

    /// The tagged fields.
    ///
    /// Versions: 9+
    pub tagged_fields: TaggedFields,
}

/// A topic to produce to.
///
/// ## See also
///
/// - <https://kafka.apache.org/protocol.html#protocol_messages>
/// - <https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceRequest.json>
pub struct TopicProduceTarget {
    /// The topic name.
    ///
    /// Versions: 0+
    pub name: String,

    /// Each partition to produce to.
    ///
    /// Versions: 0+
    pub partitions: Vec<PartitionProduceTarget>,

    /// The tagged fields.
    ///
    /// Versions: 9+
    pub tagged_fields: TaggedFields,
}

/// A partition to produce to.
///
/// ## See also
///
/// - <https://kafka.apache.org/protocol.html#protocol_messages>
/// - <https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceRequest.json>
pub struct PartitionProduceTarget {
    /// The partition index.
    ///
    /// Version: 0+
    pub index: i32,

    /// The record data to be produced.
    ///
    /// Versions: 0+
    pub records: Records,

    /// The tagged fields.
    ///
    /// Versions: 9+
    pub tagged_fields: TaggedFields,
}
