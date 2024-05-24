pub mod field;
pub mod tagged_fields;
pub mod uuid;

pub mod prelude {
    pub use super::{
        field::Field, tagged_fields::TaggedFields, uuid::Uuid, BrokerId, Buffer, Duration, Epoch,
        Offset, PartitionIndex, ProducerId, Timestamp, VarInt, VarLong,
    };
}

pub type BrokerId = i32;
pub type Duration = i32;
pub type Epoch = i32;
pub type Offset = i64;
pub type PartitionIndex = i32;
pub type ProducerId = i64;
pub type Timestamp = i64;

pub struct VarInt;

pub struct VarLong;

pub struct String {
    pub length: VarInt,
    pub bytes: Vec<u8>,
}

pub struct Buffer {
    pub length: VarInt,
    pub bytes: Vec<u8>,
}
