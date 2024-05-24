use crate::types::prelude::*;

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct MetadataRequest {
    pub topics: Vec<Topic>,
    pub allow_auto_topic_creation: bool,
    pub include_topic_authorized_operations: bool,
    pub tagged_fields: TaggedFields,
}

/// See: <https://kafka.apache.org/protocol.html#protocol_messages>
pub struct Topic {
    pub id: Uuid,
    pub name: String,
    pub tagged_fields: TaggedFields,
}
