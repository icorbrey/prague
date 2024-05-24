use crate::{isolation_level::IsolationLevel, primitives::prelude::*};

pub struct FetchRequest {
    /// The maximum time in milliseconds to wait for the response.
    pub max_wait: i32,

    /// The minimum number of bytes to accumulate in the response.
    pub min_bytes: i32,

    /// The maximum number of bytes to fetch.
    ///
    /// See KIP-74 for cases where this limit may not be honored.
    pub max_bytes: i32,

    /// Controls the visibility of transactional records.
    pub isolation_level: IsolationLevel,

    /// The fetch session's ID.
    pub session_id: i32,

    /// The fetch session's epoch, which is used for ordering requests in a session.
    pub session_epoch: i32,

    /// The list of topics to fetch.
    pub topics: Vec<Topic>,

    /// The list of topics to forget.
    pub topics_to_forget: Vec<TopicToForget>,

    /// The rack ID of the consumer making this request.
    pub rack_id: String,

    /// This fetch request's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct Topic {
    /// This topic's unique ID.
    pub id: Uuid,

    /// The list of partitions to fetch.
    pub partitions: Vec<Partition>,

    /// This topic's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct Partition {
    /// This partition's index.
    pub index: i32,

    /// The epoch of the current leader for this partition.
    pub current_leader_epoch: i32,

    /// The message offset.
    pub fetch_offset: i64,

    /// If previously fetched, the epoch of the last fetched record, otherwise None.
    pub last_fetched_epoch: Option<i32>,

    /// The earliest available offset of the follower replica.
    pub log_start_offset: i64,

    /// The maximum number of bytes to fetch from this partition.
    ///
    /// See KIP-74 for cases where this limit may not be honored.
    pub max_bytes: i32,

    /// This partition's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct TopicToForget {
    /// This topic's unique ID.
    pub id: Uuid,

    /// The indices of the partitions to forget.
    pub partition_indices: Vec<i32>,

    /// This forgotten topic's tagged fields.
    pub tagged_fields: TaggedFields,
}
