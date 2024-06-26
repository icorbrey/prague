#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("API key `{0}` is not valid")]
    InvalidApiKey(i16),
}

/// Numeric codes that the ApiKey in a request can take.
///
/// See: <https://kafka.apache.org/protocol.html#protocol_api_keys>
pub enum ApiKey {
    /// The API key for the [`ProduceApi`](crate::produce::ProduceApi).
    Produce,

    /// The API key for the [`FetchApi`](crate::fetch::FetchApi).
    Fetch,
    ListOffsets,
    Metadata,
    LeaderAndIsr,
    StopReplica,
    UpdateMetadata,
    ControlledShutdown,
    OffsetCommit,
    OffsetFetch,
    FindCoordinator,
    JoinGroup,
    Heartbeat,
    LeaveGroup,
    SyncGroup,
    DescribeGroups,
    ListGroups,
    SaslHandshake,
    ApiVersions,
    CreateTopics,
    DeleteTopics,
    DeleteRecords,
    InitProducerId,
    OffsetForLeaderEpoch,
    AddPartitionsToTransaction,
    AddOffsetsToTransaction,
    EndTransaction,
    WriteTransactionMarkers,
    TransactionOffsetCommit,
    DescribeAcls,
    CreateAcls,
    DeleteAcls,
    DescribeConfigs,
    AlterConfigs,
    AlterReplicaLogDirs,
    DescribeLogDirs,
    SaslAuthenticate,
    CreatePartitions,
    CreateDelegationToken,
    RenewDelegationToken,
    ExpireDelegationToken,
    DescribeDelegationToken,
    DeleteGroups,
    ElectLeaders,
    IncrementalAlterConfigs,
    AlterPartitionReassignments,
    ListPartitionReassignments,
    OffsetDelete,
    DescribeClientQuotas,
    AlterClientQuotas,
    DescribeUserScramCredentials,
    AlterUserScramCredentials,
    DescribeQuorum,
    AlterPartition,
    UpdateFeatures,
    Envelope,
    DescribeCluster,
    DescribeProducers,
    UnregisterBroker,
    DescribeTransactions,
    ListTransactions,
    AllocateProducerIds,
    ConsumerGroupHeartbeat,
    ConsumerGroupDescribe,
    GetTelemetrySubscriptions,
    PushTelemetry,
    ListClientMetricsResources,
}

impl TryFrom<i16> for ApiKey {
    type Error = Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Produce),
            1 => Ok(Self::Fetch),
            2 => Ok(Self::ListOffsets),
            3 => Ok(Self::Metadata),
            4 => Ok(Self::LeaderAndIsr),
            5 => Ok(Self::StopReplica),
            6 => Ok(Self::UpdateMetadata),
            7 => Ok(Self::ControlledShutdown),
            8 => Ok(Self::OffsetCommit),
            9 => Ok(Self::OffsetFetch),
            10 => Ok(Self::FindCoordinator),
            11 => Ok(Self::JoinGroup),
            12 => Ok(Self::Heartbeat),
            13 => Ok(Self::LeaveGroup),
            14 => Ok(Self::SyncGroup),
            15 => Ok(Self::DescribeGroups),
            16 => Ok(Self::ListGroups),
            17 => Ok(Self::SaslHandshake),
            18 => Ok(Self::ApiVersions),
            19 => Ok(Self::CreateTopics),
            20 => Ok(Self::DeleteTopics),
            21 => Ok(Self::DeleteRecords),
            22 => Ok(Self::InitProducerId),
            23 => Ok(Self::OffsetForLeaderEpoch),
            24 => Ok(Self::AddPartitionsToTransaction),
            25 => Ok(Self::AddOffsetsToTransaction),
            26 => Ok(Self::EndTransaction),
            27 => Ok(Self::WriteTransactionMarkers),
            28 => Ok(Self::TransactionOffsetCommit),
            29 => Ok(Self::DescribeAcls),
            30 => Ok(Self::CreateAcls),
            31 => Ok(Self::DeleteAcls),
            32 => Ok(Self::DescribeConfigs),
            33 => Ok(Self::AlterConfigs),
            34 => Ok(Self::AlterReplicaLogDirs),
            35 => Ok(Self::DescribeLogDirs),
            36 => Ok(Self::SaslAuthenticate),
            37 => Ok(Self::CreatePartitions),
            38 => Ok(Self::CreateDelegationToken),
            39 => Ok(Self::RenewDelegationToken),
            40 => Ok(Self::ExpireDelegationToken),
            41 => Ok(Self::DescribeDelegationToken),
            42 => Ok(Self::DeleteGroups),
            43 => Ok(Self::ElectLeaders),
            44 => Ok(Self::IncrementalAlterConfigs),
            45 => Ok(Self::AlterPartitionReassignments),
            46 => Ok(Self::ListPartitionReassignments),
            47 => Ok(Self::OffsetDelete),
            48 => Ok(Self::DescribeClientQuotas),
            49 => Ok(Self::AlterClientQuotas),
            50 => Ok(Self::DescribeUserScramCredentials),
            51 => Ok(Self::AlterUserScramCredentials),
            55 => Ok(Self::DescribeQuorum),
            56 => Ok(Self::AlterPartition),
            57 => Ok(Self::UpdateFeatures),
            58 => Ok(Self::Envelope),
            60 => Ok(Self::DescribeCluster),
            61 => Ok(Self::DescribeProducers),
            64 => Ok(Self::UnregisterBroker),
            65 => Ok(Self::DescribeTransactions),
            66 => Ok(Self::ListTransactions),
            67 => Ok(Self::AllocateProducerIds),
            68 => Ok(Self::ConsumerGroupHeartbeat),
            69 => Ok(Self::ConsumerGroupDescribe),
            71 => Ok(Self::GetTelemetrySubscriptions),
            72 => Ok(Self::PushTelemetry),
            74 => Ok(Self::ListClientMetricsResources),
            code => Err(Error::InvalidApiKey(code)),
        }
    }
}
