//! Includes types and functionality for determining the status of various Kafka operations.

/// Numeric error codes that indicate what problem occurred on the server.
///
/// See: <https://kafka.apache.org/protocol.html#protocol_error_codes>
pub enum ErrorCode {
    /// The server experienced an unexpected error when processing the request.
    UnknownServerError,

    /// The requested offset is not within the range of offsets maintained by the server.
    OffsetOutOfRange,

    /// This message has failed its CRC checksum, exceeds the valid size, has a null key for a
    /// compacted topic, or is otherwise corrupt.
    CorruptMessage,

    /// This server does not host this topic-partition.
    UnknownTopicOrPartition,

    /// The requested fetch size is invalid.
    InvalidFetchSize,

    /// There is no leader for this topic-partition as we are in the middle of a leadership election.
    LeaderNotAvailable,

    /// For requests intended only for the leader, this error indicates that the broker is not the
    /// current leader. For requests intended for any replica, this error indicates that the broker
    /// is not a replica of the topic partition.
    NotLeaderOrFollower,

    /// The request timed out.
    RequestTimedOut,

    /// The broker is not available.
    BrokerNotAvailable,

    /// The replica is not available for the requested topic-partition. Produce/Fetch requests and
    /// other requests intended only for the leader or follower return
    /// [`ErrorCode::NotLeaderOrFollower`] if the broker is not a replica of the topic-partition.
    ReplicaNotAvailable,

    /// The request included a message larger than the max message size the server will accept.
    MessageTooLarge,

    /// The controller moved to another broker.
    StaleControllerEpoch,

    /// The metadata field of the offset request was too large.
    OffsetMetadataTooLarge,

    /// The server disconnected before a response was received.
    NetworkException,

    /// The coordinator is loading and hence can't process requests.
    CoordinatorLoadInProgress,

    /// The coordinator is not available.
    CoordinatorNotAvailable,

    /// This is not the correct coordinator.
    NotCoordinator,

    /// The request attempted to perform an operation on an invalid topic.
    InvalidTopicException,

    /// The request included message batch larger than the configured segment size on the server.
    RecordListTooLarge,

    /// Messages are rejected since there are fewer in-sync replicas than required.
    NotEnoughReplicas,

    /// Messages are written to the log, but to fewer in-sync replicas than required.
    NotEnoughReplicasAfterAppend,

    /// Produce request specified an invalid value for required acks.
    InvalidRequiredAcks,

    /// Specified group generation id is not valid.
    IllegalGeneration,

    /// The group member's supported protocols are incompatible with those of existing members or
    /// first group member tried to join with empty protocol type or empty protocol list.
    InconsistentGroupProtocol,

    /// The configured group ID is invalid.
    InvalidGroupId,

    /// The coordinator is not aware of this member.
    UnknownMemberId,

    /// The session timeout is not within the range allowed by the broker (as configured by
    /// `group.min.session.timeout.ms` and `group.max.session.timeout.ms`).
    ///
    /// TODO: `group.min.session.timeout.ms`, `group.max.session.timeout.ms`
    InvalidSessionTimeout,

    /// The group is rebalancing, so a rejoin is needed.
    RebalanceInProgress,

    /// The committing offset data size is not valid.
    InvalidCommitOffsetSize,

    /// Topic authorization failed.
    TopicAuthorizationFailed,

    /// Group authorization failed.
    GroupAuthorizationFailed,

    /// Cluster authorization failed.
    ClusterAuthorizationFailed,

    /// The timestamp of the message is out of acceptable range.
    InvalidTimestamp,

    /// The broker does not support the requested SASL mechanism.
    UnsupportedSaslMechanism,

    /// Request is not valid given the current SASL state.
    IllegalSaslState,

    /// The version of API is not supported.
    UnsupportedVersion,

    /// Topic with this name already exists.
    TopicAlreadyExists,

    /// Number of partitions is below 1.
    InvalidPartitions,

    /// Replication factor is below 1 or larger than the number of available brokers.
    InvalidReplicationFactor,

    /// Replica assignment is invalid.
    InvalidReplicaAssignment,

    /// Configuration is invalid.
    InvalidConfig,

    /// This is not the correct controller for this cluster.
    NotController,

    /// This most likely occurs because of a request being malformed by the client library or the
    /// message was sent to an incompatible broker. See the broker logs for more details.
    InvalidRequest,

    /// The message format version on the broker does not support the request.
    UnsupportedForMessageFormat,

    /// Request parameters do not satisfy the configured policy.
    PolicyViolation,

    /// The broker received an out of order sequence number.
    OutOfOrderSequenceNumber,

    /// The broker received a duplicate sequence number.
    DuplicateSequenceNumber,

    /// Producer attempted to produce with an old epoch.
    InvalidProducerEpoch,

    /// The producer attempted a transactional operation in an invalid state.
    InvalidTransactionState,

    /// The producer attempted to use a producer ID which is not currently assigned to its
    /// transactional ID.
    InvalidProducerIdMapping,

    /// The transaction timeout is larger than the maximum value allowed by the broker (as
    /// configured by `transaction.max.timeout.ms`).
    ///
    /// TODO: `transaction.max.timeout.ms`
    InvalidTransactionTimeout,

    /// The producer attempted to update a transaction while another concurrent operation on the
    /// same transaction was ongoing.
    ConcurrentTransactions,

    /// Indicates that the transaction coordinator sending a `WriteTxnMarker` is no longer the
    /// current coordinator for a given producer.
    ///
    /// TODO: `WriteTxnMarker`
    TransactionCoordinatorFenced,

    /// Transactional ID authorization failed.
    TransactionalIdAuthorizationFailed,

    /// Security features are disabled.
    SecurityDisabled,

    /// The broker did not attempt to execute this operation. This may happen for batched RPCs
    /// where some operations in the batch failed, causing the broker to respond without trying
    /// the rest.
    OperationNotAttempted,

    /// Disk error when trying to access log file on the disk.
    KafkaStorageError,

    /// The user-specified log directory is not found in the broker config.
    LogDirectoryNotFound,

    /// SASL Authentication failed.
    SaslAuthenticationFailed,

    /// This exception is raised by the broker if it could not locate the producer metadata
    /// associated with the producer ID in question. This could happen if, for instance, the
    /// producer's records were deleted because their retention time had elapsed. Once the last
    /// records of the producer ID are removed, the producer's metadata is removed from the broker,
    /// and future appends by the producer will return this exception.
    UnknownProducerId,

    /// A partition reassignment is in progress.
    ReassignmentInProgress,

    /// Delegation Token feature is not enabled.
    DelegationTokenAuthDisabled,

    /// Delegation Token is not found on server.
    DelegationTokenNotFound,

    /// Specified Principal is not valid Owner/Renewer.
    DelegationTokenOwnerMismatch,

    /// Delegation Token requests are not allowed on PLAINTEXT/1-way SSL channels and on delegation
    /// token authenticated channels.
    DelegationTokenRequestNotAllowed,

    /// Delegation Token authorization failed.
    DelegationTokenAuthorizationFailed,

    /// Delegation Token is expired.
    DelegationTokenExpired,

    /// Supplied principal type is not supported.
    InvalidPrincipalType,

    /// The group is not empty.
    NonEmptyGroup,

    /// The group ID does not exist.
    GroupIdNotFound,

    /// The fetch session ID was not found.
    FetchSessionIdNotFound,

    /// The fetch session epoch is invalid.
    InvalidFetchSessionEpoch,

    /// There is no listener on the leader broker that matches the listener on which metadata
    /// request was processed.
    ListenerNotFound,

    /// Topic deletion is disabled.
    TopicDeletionDisabled,

    /// The leader epoch in the request is older than the epoch on the broker.
    FencedLeaderEpoch,

    /// The leader epoch in the request is newer than the epoch on the broker.
    UnknownLeaderEpoch,

    /// The requesting client does not support the compression type of given partition.
    UnsupportedCompressionType,

    /// Broker epoch has changed.
    StaleBrokerEpoch,

    /// The leader high watermark has not caught up from a recent leader election so the offsets
    /// cannot be guaranteed to be monotonically increasing.
    OffsetNotAvailable,

    /// The group member needs to have a valid member ID before actually entering a consumer group.
    MemberIdRequired,

    /// The preferred leader was not available.
    PreferredLeaderNotAvailable,

    /// The consumer group has reached its max size.
    GroupMaxSizeReached,

    /// The broker rejected this static consumer since another consumer with the same
    /// `group.instance.id` has registered with a different `member.id`.
    ///
    /// TODO: `group.instance.id`, `member.id`
    FencedInstanceId,

    /// Eligible topic partition leaders are not available.
    EligibleLeadersNotAvailable,

    /// Leader election not needed for topic partition.
    ElectionNotNeeded,

    /// No partition reassignment is in progress.
    NoReassignmentInProgress,

    /// Deleting offsets of a topic is forbidden while the consumer group is actively subscribed to
    /// it.
    GroupSubscribedToTopic,

    /// This record has failed the validation on broker and hence will be rejected.
    InvalidRecord,

    /// There are unstable offsets that need to be cleared.
    UnstableOffsetCommit,

    /// The throttling quota has been exceeded.
    ThrottlingQuotaExceeded,

    /// There is a newer producer with the same transactional ID which fences the current one.
    ProducerFenced,

    /// A request illegally referred to a resource that does not exist.
    ResourceNotFound,

    /// A request illegally referred to the same resource twice.
    DuplicateResource,

    /// Requested credential would not meet criteria for acceptability.
    UnacceptableCredential,

    /// Indicates that the either the sender or recipient of a voter-only request is not one of the
    /// expected voters.
    InconsistentVoterSet,

    /// The given update version was invalid.
    InvalidUpdateVersion,

    /// Unable to update finalized features due to an unexpected server error.
    FeatureUpdateFailed,

    /// Request principal deserialization failed during forwarding. This indicates an internal
    /// error on the broker cluster security setup.
    PrincipalDeserializationFailure,

    /// Requested snapshot was not found.
    SnapshotNotFound,

    /// Requested position is not greater than or equal to zero, and less than the size of the
    /// snapshot.
    PositionOutOfRange,

    /// This server does not host this topic ID.
    UnknownTopicId,

    /// This broker ID is already in use.
    DuplicateBrokerRegistration,

    /// The given broker ID was not registered.
    BrokerIdNotRegistered,

    /// The log's topic ID did not match the topic ID in the request.
    InconsistentTopicId,

    /// The cluster ID in the request does not match that found on the server.
    InconsistentClusterId,

    /// The transactional ID could not be found.
    TransactionalIdNotFound,

    /// The fetch session encountered inconsistent topic ID usage.
    FetchSessionTopicIdError,

    /// The new ISR contains at least one ineligible replica.
    InelligibleReplica,

    /// The `AlterPartition` request successfully updated the partition state but the leader has
    /// changed.
    ///
    /// TODO: `AlterPartition`
    NewLeaderElected,

    /// The requested offset is moved to tiered storage.
    OffsetMovedToTieredStorage,

    /// The member epoch is fenced by the group coordinator. The member must abandon all its
    /// partitions and rejoin.
    FencedMemberEpoch,

    /// The instance ID is still used by another member in the consumer group. That member must
    /// leave first.
    UnreleasedInstanceId,

    /// The assignor or its version range is not supported by the consumer group.
    UnsupportedAssignor,

    /// The member epoch is stale. The member must retry after receiving its updated member epoch
    /// via the ConsumerGroupHeartbeat API.
    ///
    /// TODO: `ConsumerGroupHeartbeat`
    StaleMemberEpoch,

    /// The request was sent to an endpoint of the wrong type.
    MismatchedEndpointType,

    /// This endpoint type is not supported yet.
    UnsupportedEndpointType,

    /// This controller ID is not known.
    UnknownControllerId,

    /// Client sent a push telemetry request with an invalid or outdated subscription ID.
    UnknownSubscriptionId,

    /// Client sent a push telemetry request larger than the maximum size the broker will accept.
    TelemetryTooLarge,

    /// The controller has considered the broker registration to be invalid.
    InvalidRegistration,
}

impl ErrorCode {
    /// Attempts to resolve an [`ErrorCode`] from the given integer.
    pub fn parse(value: i32) -> Result<Option<ErrorCode>, String> {
        match value {
            -1 => Ok(Some(Self::UnknownServerError)),
            0 => Ok(None),
            1 => Ok(Some(Self::OffsetOutOfRange)),
            2 => Ok(Some(Self::CorruptMessage)),
            3 => Ok(Some(Self::UnknownTopicOrPartition)),
            4 => Ok(Some(Self::InvalidFetchSize)),
            5 => Ok(Some(Self::LeaderNotAvailable)),
            6 => Ok(Some(Self::NotLeaderOrFollower)),
            7 => Ok(Some(Self::RequestTimedOut)),
            8 => Ok(Some(Self::BrokerNotAvailable)),
            9 => Ok(Some(Self::ReplicaNotAvailable)),
            10 => Ok(Some(Self::MessageTooLarge)),
            11 => Ok(Some(Self::StaleControllerEpoch)),
            12 => Ok(Some(Self::OffsetMetadataTooLarge)),
            13 => Ok(Some(Self::NetworkException)),
            14 => Ok(Some(Self::CoordinatorLoadInProgress)),
            15 => Ok(Some(Self::CoordinatorNotAvailable)),
            16 => Ok(Some(Self::NotCoordinator)),
            17 => Ok(Some(Self::InvalidTopicException)),
            18 => Ok(Some(Self::RecordListTooLarge)),
            19 => Ok(Some(Self::NotEnoughReplicas)),
            20 => Ok(Some(Self::NotEnoughReplicasAfterAppend)),
            21 => Ok(Some(Self::InvalidRequiredAcks)),
            23 => Ok(Some(Self::IllegalGeneration)),
            24 => Ok(Some(Self::InconsistentGroupProtocol)),
            25 => Ok(Some(Self::InvalidGroupId)),
            26 => Ok(Some(Self::InvalidSessionTimeout)),
            27 => Ok(Some(Self::RebalanceInProgress)),
            28 => Ok(Some(Self::InvalidCommitOffsetSize)),
            29 => Ok(Some(Self::TopicAuthorizationFailed)),
            30 => Ok(Some(Self::GroupAuthorizationFailed)),
            31 => Ok(Some(Self::ClusterAuthorizationFailed)),
            32 => Ok(Some(Self::InvalidTimestamp)),
            33 => Ok(Some(Self::UnsupportedSaslMechanism)),
            34 => Ok(Some(Self::IllegalSaslState)),
            35 => Ok(Some(Self::UnsupportedVersion)),
            36 => Ok(Some(Self::TopicAlreadyExists)),
            37 => Ok(Some(Self::InvalidPartitions)),
            38 => Ok(Some(Self::InvalidReplicationFactor)),
            39 => Ok(Some(Self::InvalidReplicaAssignment)),
            40 => Ok(Some(Self::InvalidConfig)),
            41 => Ok(Some(Self::NotController)),
            42 => Ok(Some(Self::InvalidRequest)),
            43 => Ok(Some(Self::UnsupportedForMessageFormat)),
            44 => Ok(Some(Self::PolicyViolation)),
            45 => Ok(Some(Self::OutOfOrderSequenceNumber)),
            46 => Ok(Some(Self::DuplicateSequenceNumber)),
            47 => Ok(Some(Self::InvalidProducerEpoch)),
            48 => Ok(Some(Self::InvalidTransactionState)),
            49 => Ok(Some(Self::InvalidProducerIdMapping)),
            50 => Ok(Some(Self::InvalidTransactionTimeout)),
            51 => Ok(Some(Self::ConcurrentTransactions)),
            52 => Ok(Some(Self::TransactionCoordinatorFenced)),
            53 => Ok(Some(Self::TransactionalIdAuthorizationFailed)),
            54 => Ok(Some(Self::SecurityDisabled)),
            55 => Ok(Some(Self::OperationNotAttempted)),
            56 => Ok(Some(Self::KafkaStorageError)),
            57 => Ok(Some(Self::LogDirectoryNotFound)),
            58 => Ok(Some(Self::SaslAuthenticationFailed)),
            59 => Ok(Some(Self::UnknownProducerId)),
            60 => Ok(Some(Self::ReassignmentInProgress)),
            61 => Ok(Some(Self::DelegationTokenAuthDisabled)),
            62 => Ok(Some(Self::DelegationTokenNotFound)),
            63 => Ok(Some(Self::DelegationTokenOwnerMismatch)),
            64 => Ok(Some(Self::DelegationTokenRequestNotAllowed)),
            65 => Ok(Some(Self::DelegationTokenAuthorizationFailed)),
            66 => Ok(Some(Self::DelegationTokenExpired)),
            67 => Ok(Some(Self::InvalidPrincipalType)),
            68 => Ok(Some(Self::NonEmptyGroup)),
            69 => Ok(Some(Self::GroupIdNotFound)),
            70 => Ok(Some(Self::FetchSessionIdNotFound)),
            71 => Ok(Some(Self::InvalidFetchSessionEpoch)),
            72 => Ok(Some(Self::ListenerNotFound)),
            73 => Ok(Some(Self::TopicDeletionDisabled)),
            74 => Ok(Some(Self::FencedLeaderEpoch)),
            75 => Ok(Some(Self::UnknownLeaderEpoch)),
            76 => Ok(Some(Self::UnsupportedCompressionType)),
            77 => Ok(Some(Self::StaleBrokerEpoch)),
            78 => Ok(Some(Self::OffsetNotAvailable)),
            79 => Ok(Some(Self::MemberIdRequired)),
            80 => Ok(Some(Self::PreferredLeaderNotAvailable)),
            81 => Ok(Some(Self::GroupMaxSizeReached)),
            82 => Ok(Some(Self::FencedInstanceId)),
            83 => Ok(Some(Self::EligibleLeadersNotAvailable)),
            84 => Ok(Some(Self::ElectionNotNeeded)),
            85 => Ok(Some(Self::NoReassignmentInProgress)),
            86 => Ok(Some(Self::GroupSubscribedToTopic)),
            87 => Ok(Some(Self::InvalidRecord)),
            88 => Ok(Some(Self::UnstableOffsetCommit)),
            89 => Ok(Some(Self::ThrottlingQuotaExceeded)),
            90 => Ok(Some(Self::ProducerFenced)),
            91 => Ok(Some(Self::ResourceNotFound)),
            92 => Ok(Some(Self::DuplicateResource)),
            93 => Ok(Some(Self::UnacceptableCredential)),
            94 => Ok(Some(Self::InconsistentVoterSet)),
            95 => Ok(Some(Self::InvalidUpdateVersion)),
            96 => Ok(Some(Self::FeatureUpdateFailed)),
            97 => Ok(Some(Self::PrincipalDeserializationFailure)),
            98 => Ok(Some(Self::SnapshotNotFound)),
            99 => Ok(Some(Self::PositionOutOfRange)),
            100 => Ok(Some(Self::UnknownTopicId)),
            101 => Ok(Some(Self::DuplicateBrokerRegistration)),
            102 => Ok(Some(Self::BrokerIdNotRegistered)),
            103 => Ok(Some(Self::InconsistentTopicId)),
            104 => Ok(Some(Self::InconsistentClusterId)),
            105 => Ok(Some(Self::TransactionalIdNotFound)),
            106 => Ok(Some(Self::FetchSessionTopicIdError)),
            107 => Ok(Some(Self::InelligibleReplica)),
            108 => Ok(Some(Self::NewLeaderElected)),
            109 => Ok(Some(Self::OffsetMovedToTieredStorage)),
            110 => Ok(Some(Self::FencedMemberEpoch)),
            111 => Ok(Some(Self::UnreleasedInstanceId)),
            112 => Ok(Some(Self::UnsupportedAssignor)),
            113 => Ok(Some(Self::StaleMemberEpoch)),
            114 => Ok(Some(Self::MismatchedEndpointType)),
            115 => Ok(Some(Self::UnsupportedEndpointType)),
            116 => Ok(Some(Self::UnknownControllerId)),
            117 => Ok(Some(Self::UnknownSubscriptionId)),
            118 => Ok(Some(Self::TelemetryTooLarge)),
            119 => Ok(Some(Self::InvalidRegistration)),
            x => Err(format!("Code was not a valid status: {0}", x)),
        }
    }

    /// Returns whether the operation this error code is associated with is retriable.
    pub fn is_retriable(&self) -> bool {
        match self {
            Self::CorruptMessage
            | Self::UnknownTopicOrPartition
            | Self::LeaderNotAvailable
            | Self::NotLeaderOrFollower
            | Self::RequestTimedOut
            | Self::ReplicaNotAvailable
            | Self::NetworkException
            | Self::CoordinatorLoadInProgress
            | Self::CoordinatorNotAvailable
            | Self::NotCoordinator
            | Self::NotEnoughReplicas
            | Self::NotEnoughReplicasAfterAppend
            | Self::NotController
            | Self::ConcurrentTransactions
            | Self::KafkaStorageError
            | Self::FetchSessionIdNotFound
            | Self::InvalidFetchSessionEpoch
            | Self::ListenerNotFound
            | Self::FencedLeaderEpoch
            | Self::UnknownLeaderEpoch
            | Self::OffsetNotAvailable
            | Self::PreferredLeaderNotAvailable
            | Self::EligibleLeadersNotAvailable
            | Self::ElectionNotNeeded
            | Self::UnstableOffsetCommit
            | Self::ThrottlingQuotaExceeded
            | Self::UnknownTopicId
            | Self::InconsistentTopicId
            | Self::FetchSessionTopicIdError => true,
            _ => false,
        }
    }
}
