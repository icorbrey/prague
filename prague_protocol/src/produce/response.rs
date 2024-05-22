use crate::{error_code::ErrorCode, primitives::tagged_fields::TaggedFields};

pub struct ProduceResponse {
    /// Responses from each topic that was produced to.
    pub responses: Vec<TopicResponse>,

    /// If the associated produce request violated a quota, the duration for which the request was
    /// throttled. Otherwise, None.
    pub throttle_time: Option<i32>,

    /// This produce response's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct TopicResponse {
    /// This topic's name.
    pub name: String,

    /// Responses from each partition that was produced to in this topic.
    pub partition_responses: Vec<PartitionResponse>,
}

pub struct PartitionResponse {
    /// This partition's index.
    pub index: i32,

    /// If the request was unsuccessful, the error code. Otherwise, None.
    pub error_code: Option<ErrorCode>,

    /// This partition's base offset.
    pub base_offset: i64,

    /// If LogAppendTime is used for the topic, the broker local time when the messages are
    /// appended. Otherwise, None.
    pub log_append_time: Option<i64>,

    /// The log start offset.
    pub log_start_offset: i64,

    /// The list of records that caused batches to be dropped.
    pub record_batch_errors: Vec<RecordBatchError>,

    /// If the request was unsuccessful, a summary of the root cause of the error.
    pub error_message: Option<String>,
}

pub struct RecordBatchError {
    /// The index of the record that caused this batch to be dropped.
    pub index: i32,

    /// An explanation of the error.
    pub message: String,
}
