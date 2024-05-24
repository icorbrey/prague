use crate::{error_code::ErrorCode, primitives::prelude::*, records::RecordBatch};

pub struct FetchResponse {
    /// If the associated fetch request violated a quota, the duration for which the request was
    /// throttled. Otherwise, None.
    pub throttle_time: Option<i32>,

    /// This fetch response's overall status code.
    pub error_code: Option<ErrorCode>,

    /// If this request was part of a fetch session, the session's ID. Otherwise, None.
    pub session_id: Option<i32>,

    /// The list of responses from each topic.
    pub topic_responses: Vec<TopicResponse>,

    /// This fetch response's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct TopicResponse {
    /// This topic's unique ID.
    pub id: Uuid,

    /// The responses from this topic's partitions.
    pub partitions: Vec<PartitionResponse>,

    /// This topic response's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct PartitionResponse {
    /// This partition's index.
    pub index: i32,

    /// This partition's error code if something went wrong, otherwise None.
    pub error_code: Option<ErrorCode>,

    /// The current high watermark.
    pub high_watermark: i64,

    /// The last stable offset (or LSO) of this partition. This is the last offset such that the
    /// state of all transactional records prior to this offset have been decided (i.e. ABORTED or
    /// COMMITTED).
    pub last_stable_offset: i64,

    /// The current log start offset.
    pub log_start_offset: i64,

    /// The list of aborted transactions for this partition.
    pub aborted_transactions: Vec<AbortedTransaction>,

    /// The preferred read replica for the consumer to use on its next fetch request.
    pub preferred_read_replica: i32,

    /// The fetched record data.
    pub records: RecordBatch,

    /// This partition response's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct AbortedTransaction {
    /// The producer ID associated with this aborted transaction.
    pub producer_id: i64,

    /// The first offset in this aborted transaction.
    pub first_offset: i64,

    /// This aborted transaction's tagged fields.
    pub tagged_fields: TaggedFields,
}
