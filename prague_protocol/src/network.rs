/// Marks this type as a message that can be sent over the network.
pub trait Message {
    /// Returns the size of this message.
    fn size(&self) -> i32;
}

/// The root of all Kafka network messages.
///
/// See: <https://kafka.apache.org/protocol.html#protocol_common>
pub struct NetworkMessage<M: Message> {
    /// The size of the subsequent network message in bytes. The client can read network messages
    /// by first reading this field as an integer N, then reading and parsing the subsequent N
    /// bytes of the message.
    pub size: i32,

    /// The contents of this network message.
    pub message: M,
}

impl<M: Message> From<M> for NetworkMessage<M> {
    fn from(value: M) -> Self {
        Self {
            size: value.size(),
            message: value,
        }
    }
}
