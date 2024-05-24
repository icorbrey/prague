use crate::types::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Record batch compression code `{0}` is not valid")]
    InvalidRecordBatchCompression(i16),
}

///
/// See: <https://kafka.apache.org/documentation/#recordbatch>
pub struct Records {
    pub base_offset: Offset,
    pub batch_length: i32,
    pub partition_leader_epoch: Epoch,
    pub magic_number: i8,
    pub crc: CyclicRedundancyCheck,
    pub attributes: RecordBatchAttributes,
    pub last_offset_delta: Duration,
    pub base_timestamp: Timestamp,
    pub max_timestamp: Timestamp,
    pub producer_id: ProducerId,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub records: Vec<Record>,
}

impl Records {
    pub const MAGIC_NUMBER: i8 = 2;
}

/// Describes the attributes of a [record batch](RecordBatch).
///
/// Over the wire, this is transmitted as a bitfield, such that `____ ____ _edc baaa` gets
/// interpreted as:
///
/// - `aaa`: The [compression type](RecordBatchCompression) of this record batch.
/// - `b`: Whether this record batch is transactional.
/// - `c`: Whether this record batch is a control batch.
/// - `d`: Whether this record batch's `base_timestamp` is set as the delete horizon for
///   compaction.
pub struct RecordBatchAttributes {
    pub compression: RecordBatchCompression,
    pub timestamp_type: bool,

    /// If true, this [record batch](RecordBatch) is transactional.
    pub is_transactional: bool,

    /// If true, this [record batch](RecordBatch) is a control batch.
    pub is_control_batch: bool,

    /// If true, this [record batch](RecordBatch)'s `base_timestamp` is set as the delete horizon
    /// for compaction.
    pub has_delete_horizon_ms: bool,
}

impl RecordBatchAttributes {
    const COMPRESSION_MASK: i16 = 0b111;
    const TIMESTAMP_TYPE_MASK: i16 = 0b1 << 3;
    const IS_TRANSACTIONAL_MASK: i16 = 0b1 << 4;
    const IS_CONTROL_BATCH_MASK: i16 = 0b1 << 5;
    const HAS_DELETE_HORIZON_MS_MASK: i16 = 0b1 << 6;
}

impl TryFrom<i16> for RecordBatchAttributes {
    type Error = Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(Self {
            compression: (value & Self::COMPRESSION_MASK).try_into()?,
            timestamp_type: (value & Self::TIMESTAMP_TYPE_MASK) != 0,
            is_transactional: (value & Self::IS_TRANSACTIONAL_MASK) != 0,
            is_control_batch: (value & Self::IS_CONTROL_BATCH_MASK) != 0,
            has_delete_horizon_ms: (value & Self::HAS_DELETE_HORIZON_MS_MASK) != 0,
        })
    }
}

pub enum RecordBatchCompression {
    NoCompression,
    Gzip,
    Snappy,
    Lz4,
    Zstd,
}

impl TryFrom<i16> for RecordBatchCompression {
    type Error = Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoCompression),
            1 => Ok(Self::Gzip),
            2 => Ok(Self::Snappy),
            3 => Ok(Self::Lz4),
            4 => Ok(Self::Zstd),
            code => Err(Error::InvalidRecordBatchCompression(code)),
        }
    }
}

pub struct Record {
    pub length: VarInt,
    pub attributes: i8,
    pub timestamp_delta: VarLong,
    pub offset_delta: VarInt,
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub headers: Vec<Header>,
}

pub struct Header {
    pub key: String,
    pub value: Vec<u8>,
}
