use crate::{acks::AckRequirement, primitives::tagged_fields::TaggedFields, records::RecordBatch};

pub struct ProduceRequest {
    /// If the producer is transactional, Some(id), otherwise None.
    pub transactional_id: Option<String>,

    /// The number of acknowledgements the producer requires the leader to have received before
    /// considering a request complete.
    pub ack_requirement: AckRequirement,

    /// The timeout to await a response in milliseconds.
    pub timeout: i32,

    /// The list of topics to produce to.
    pub topics: Vec<Topic>,

    /// This request's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct Topic {
    /// This topic's name.
    pub name: String,

    /// The list of partitions to produce to.
    pub partitions: Vec<Partition>,

    /// This topic's tagged fields.
    pub tagged_fields: TaggedFields,
}

pub struct Partition {
    /// This partition's index.
    pub index: i32,

    /// The batch of records to produce to this partition.
    pub records: RecordBatch,

    /// This partition's tagged fields.
    pub tagged_fields: TaggedFields,
}
