use crate::{error_code::ErrorCode, records::Records, types::prelude::*};

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct FetchResponse {
    /// If the associated fetch request violated a quota, the duration for which the request was
    /// throttled. Otherwise, None.
    pub throttle_time: Option<Duration>,

    /// This fetch response's overall status code.
    pub error_code: Option<ErrorCode>,

    /// If this request was part of a fetch session, the session's ID. Otherwise, None.
    pub session_id: Option<i32>,

    /// The list of responses from each topic.
    pub topic_responses: Vec<Topic>,

    /// This fetch response's tagged fields.
    pub tagged_fields: TaggedFields,
}

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct Topic {
    /// This topic's unique ID.
    pub id: Uuid,

    /// The responses from this topic's partitions.
    pub partitions: Vec<Partition>,

    /// This topic's tagged fields.
    pub tagged_fields: TaggedFields,
}

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct Partition {
    /// This partition's index.
    pub index: PartitionIndex,

    /// This partition's error code if something went wrong, otherwise None.
    pub error_code: Option<ErrorCode>,

    /// The current high watermark.
    pub high_watermark: Offset,

    /// The last stable offset (or LSO) of this partition. This is the last offset such that the
    /// state of all transactional records prior to this offset have been decided (i.e. ABORTED or
    /// COMMITTED).
    pub last_stable_offset: Offset,

    /// The current log start offset.
    pub log_start_offset: Offset,

    /// The list of aborted transactions for this partition.
    pub aborted_transactions: Vec<AbortedTransaction>,

    /// The preferred read replica for the consumer to use on its next fetch request.
    pub preferred_read_replica: BrokerId,

    /// The fetched record data.
    pub records: Records,

    /// This partition response's tagged fields.
    pub tagged_fields: TaggedFields,
}

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct AbortedTransaction {
    /// The producer ID associated with this aborted transaction.
    pub producer_id: ProducerId,

    /// The first offset in this aborted transaction.
    pub first_offset: Offset,

    /// This aborted transaction's tagged fields.
    pub tagged_fields: TaggedFields,
}
