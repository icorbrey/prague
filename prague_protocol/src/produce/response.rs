use crate::{error_code::ErrorCode, primitives::prelude::*};

pub struct ProduceResponse {
    /// Responses from each topic that was produced to.
    pub topic_responses: Vec<TopicResponse>,

    /// If the associated produce request violated a quota, the duration for which the request was
    /// throttled. Otherwise, None.
    pub throttle_time: Option<Duration>,

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
    pub index: PartitionIndex,

    /// If the request was unsuccessful, the error code. Otherwise, None.
    pub error_code: Option<ErrorCode>,

    /// This partition's base offset.
    pub base_offset: Offset,

    /// If LogAppendTime is used for the topic, the broker local time when the messages are
    /// appended. Otherwise, None.
    pub log_append_time: Option<Timestamp>,

    /// The log start offset.
    pub log_start_offset: Offset,

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
