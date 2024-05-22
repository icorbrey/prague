pub mod field;
pub mod tagged_fields;

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
