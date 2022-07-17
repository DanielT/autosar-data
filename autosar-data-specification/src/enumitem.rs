use crate::hashfunc;

#[derive(Debug)]
/// The error type ParseEnumItemError is returned when from_str() / parse() fails for EnumItem
pub struct ParseEnumItemError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
/// Enum of all possible enum values in Autosar
pub enum EnumItem {
    /// MODE-DECLARATION-SWITCH-INITIATED
    ModeDeclarationSwitchInitiated                                         = 0,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 1,
    /// CAPTURE-ASYNCHRONOUS-TO-REPORTING
    CaptureAsynchronousToReporting                                         = 2,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 3,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 4,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 5,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 6,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE
    PersistencyRedundancyHandlingScopeDatabase                             = 7,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 8,
    /// BOTTOM
    Bottom                                                                 = 9,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 10,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 11,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 12,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 13,
    /// VO
    Vo                                                                     = 14,
    /// MINIMUM-MINOR-VERSION
    MinimumMinorVersion                                                    = 15,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 16,
    /// GU
    Gu                                                                     = 17,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 18,
    /// PRE--R-4--2
    PreR42                                                                 = 19,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 20,
    /// FORGET
    Forget                                                                 = 21,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 22,
    /// ON-CHANGE-OF-DATA-IDENTIFIER
    OnChangeOfDataIdentifier                                               = 23,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 24,
    /// PL
    Pl                                                                     = 25,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 26,
    /// POLY
    Poly                                                                   = 27,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 28,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED
    AdaptiveServiceSubscriptionAcknowledgeStarted                          = 29,
    /// ZH
    Zh                                                                     = 30,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 31,
    /// J-1939
    J1939                                                                  = 32,
    /// NO-SHOW-TYPE
    NoShowType                                                             = 33,
    /// SPEC-ELEMENT-REFERENCE
    SpecElementReference                                                   = 34,
    /// CURVE_AXIS
    Curveaxis                                                              = 35,
    /// GRANT
    Grant                                                                  = 36,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 37,
    /// BSW-MODULE-ENTITY-ACTIVATED
    BswModuleEntityActivated                                               = 38,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 39,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 40,
    /// MS
    Ms                                                                     = 41,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 42,
    /// PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR
    ProcessingStyleAsynchronousWithError                                   = 43,
    /// DIAGNOSTIC-COMMON-ELEMENT
    DiagnosticCommonElement                                                = 44,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 45,
    /// MANUFACTURING
    Manufacturing                                                          = 46,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 47,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 48,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 49,
    /// SENT-UNTAGGED
    SentUntagged                                                           = 50,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 51,
    /// BSW-M-ENTRY-CALLED
    BswMEntryCalled                                                        = 52,
    /// CHAPTER
    Chapter                                                                = 53,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 54,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 55,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 56,
    /// SW-INSTANCE
    SwInstance                                                             = 57,
    /// SERVER-DECRYPT
    ServerDecrypt                                                          = 58,
    /// ERROR-TRACER
    ErrorTracer                                                            = 59,
    /// CYCLIC-AND-ON-CHANGE
    CyclicAndOnChange                                                      = 60,
    /// FATAL
    Fatal                                                                  = 61,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 62,
    /// TEST-FAILED-THIS-OPERATION-CYCLE
    TestFailedThisOperationCycle                                           = 63,
    /// PHYSICAL
    Physical                                                               = 64,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 65,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 66,
    /// TIME
    Time                                                                   = 67,
    /// DIAGNOSTIC-MEMORY-DESTINATION
    DiagnosticMemoryDestination                                            = 68,
    /// RESET-MACHINE
    ResetMachine                                                           = 69,
    /// NON-REENTRANT
    NonReentrant                                                           = 70,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 71,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 72,
    /// CLIENT-ENCRYPT
    ClientEncrypt                                                          = 73,
    /// BINARY-MANIFEST-RESOURCE
    BinaryManifestResource                                                 = 74,
    /// MOST-SIGNIFICANT-BYTE-LAST
    MostSignificantByteLast                                                = 75,
    /// UPDATE
    Update                                                                 = 76,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 77,
    /// DATA-PROTOTYPE
    DataPrototype                                                          = 78,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 79,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 80,
    /// ACCESS-PERMISSION-SERVICE-CLASS
    AccessPermissionServiceClass                                           = 81,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 82,
    /// SU
    Su                                                                     = 83,
    /// LIN-FRAME
    LinFrame                                                               = 84,
    /// ADAPTIVE-FIELD-SETTER-COMPLETED
    AdaptiveFieldSetterCompleted                                           = 85,
    /// TD-EVENT-COM
    TdEventCom                                                             = 86,
    /// IS-RELEVANT
    IsRelevant                                                             = 87,
    /// ATP-BLUEPRINTABLE
    AtpBlueprintable                                                       = 88,
    /// KEEP-EXISTING
    KeepExisting                                                           = 89,
    /// BMP
    Bmp                                                                    = 90,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 91,
    /// SW-CALIBRATION-METHOD
    SwCalibrationMethod                                                    = 92,
    /// UCM
    Ucm                                                                    = 93,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 94,
    /// NM-CLUSTER
    NmCluster                                                              = 95,
    /// STIMULUS-SYNCHRONIZATION
    StimulusSynchronization                                                = 96,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 97,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 98,
    /// LAST-IS-BEST
    LastIsBest                                                             = 99,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 100,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 101,
    /// TD-EVENT-BSW
    TdEventBsw                                                             = 102,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 103,
    /// CAN-20
    Can20                                                                  = 104,
    /// TCP
    Tcp                                                                    = 105,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 106,
    /// MONOTONOUS
    Monotonous                                                             = 107,
    /// SERIALIZER
    Serializer                                                             = 108,
    /// STATUS-BIT-NORMAL
    StatusBitNormal                                                        = 109,
    /// TEST-FAILED
    TestFailed                                                             = 110,
    /// ACTION
    Action                                                                 = 111,
    /// ALL-INDICES-SAME-ARRAY-SIZE
    AllIndicesSameArraySize                                                = 112,
    /// VIEW-MAP
    ViewMap                                                                = 113,
    /// TD-EVENT-VFB
    TdEventVfb                                                             = 114,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 115,
    /// READ-ONLY
    ReadOnly                                                               = 116,
    /// SIGNATURE
    Signature                                                              = 117,
    /// VARIANT-POST-BUILD-LOADABLE
    VariantPostBuildLoadable                                               = 118,
    /// BG
    Bg                                                                     = 119,
    /// SDG-ABSTRACT-FOREIGN-REFERENCE
    SdgAbstractForeignReference                                            = 120,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 121,
    /// SW-CONNECTOR
    SwConnector                                                            = 122,
    /// USES-LOGGING
    UsesLogging                                                            = 123,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 124,
    /// ACTION-LIST
    ActionList                                                             = 125,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 126,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 127,
    /// WONT-RECEIVE
    WontReceive                                                            = 128,
    /// HW-DESCRIPTION-ENTITY
    HwDescriptionEntity                                                    = 129,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 130,
    /// RSA
    Rsa                                                                    = 131,
    /// COMMUNICATION-INTER-ECU
    CommunicationInterEcu                                                  = 132,
    /// NEW-IS-EQUAL
    NewIsEqual                                                             = 133,
    /// LT
    Lt                                                                     = 134,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 135,
    /// ROTATE-90-CW
    Rotate90Cw                                                             = 136,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 137,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 138,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 139,
    /// TD-EVENT-SERVICE-INSTANCE
    TdEventServiceInstance                                                 = 140,
    /// HIERARCHICAL-EOC
    HierarchicalEoc                                                        = 141,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 142,
    /// BSW-SERVICE-DEPENDENCY-IDENT
    BswServiceDependencyIdent                                              = 143,
    /// ABSTRACT-EVENT
    AbstractEvent                                                          = 144,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 145,
    /// XFILE
    Xfile                                                                  = 146,
    /// PC-AFFECTS-LT
    PcAffectsLt                                                            = 147,
    /// PROCESS-IS-SELF-TERMINATING
    ProcessIsSelfTerminating                                               = 148,
    /// KU
    Ku                                                                     = 149,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE
    PersistencyRedundancyHandlingScopeFile                                 = 150,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 151,
    /// SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToSwClusterDesignPortPrototypeMapping                   = 152,
    /// TRIGGERED-WITHOUT-REPETITION
    TriggeredWithoutRepetition                                             = 153,
    /// DHCPV-4
    Dhcpv4                                                                 = 154,
    /// BUILD-TYPE-DEBUG
    BuildTypeDebug                                                         = 155,
    /// ACL-OPERATION
    AclOperation                                                           = 156,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 157,
    /// EMISSION-RELATED-DTC
    EmissionRelatedDtc                                                     = 158,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 159,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 160,
    /// SYNCHRONIZED
    Synchronized                                                           = 161,
    /// STATUS-BIT-AGING-AND-DISPLACEMENT
    StatusBitAgingAndDisplacement                                          = 162,
    /// DEFAULT-IF-REVISION-UPDATE
    DefaultIfRevisionUpdate                                                = 163,
    /// ECU-INSTANCE
    EcuInstance                                                            = 164,
    /// PARTITION
    Partition                                                              = 165,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 166,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 167,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 168,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 169,
    /// CYCLE-REPETITION-8
    CycleRepetition8                                                       = 170,
    /// ADAPTIVE-EVENT-SENT
    AdaptiveEventSent                                                      = 171,
    /// NON-EMMISSION-RELATED-DTC
    NonEmmissionRelatedDtc                                                 = 172,
    /// FORWARD-AS-IS
    ForwardAsIs                                                            = 173,
    /// DIAGNOSTIC-MAPPING
    DiagnosticMapping                                                      = 174,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 175,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 176,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 177,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 178,
    /// RECT
    Rect                                                                   = 179,
    /// IMMEDIATE
    Immediate                                                              = 180,
    /// RUN-CONTINUOUS
    RunContinuous                                                          = 181,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 182,
    /// RPT-ENABLER-RAM-AND-ROM
    RptEnablerRamAndRom                                                    = 183,
    /// TUNNEL
    Tunnel                                                                 = 184,
    /// X-MII
    XMii                                                                   = 185,
    /// VERBOSE
    Verbose                                                                = 186,
    /// CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    ClearDynamicallyDefineDataIdentifier                                   = 187,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 188,
    /// ROTATE-180
    Rotate180                                                              = 189,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 190,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 191,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 192,
    /// WATCH-TRIGGER
    WatchTrigger                                                           = 193,
    /// LIMIT-TO-TEXT
    LimitToText                                                            = 194,
    /// STACK-USAGE
    StackUsage                                                             = 195,
    /// FRAME-RECEIVED-BY-IF
    FrameReceivedByIf                                                      = 196,
    /// INFO
    Info                                                                   = 197,
    /// CP-SOFTWARE-CLUSTER-RESOURCE
    CpSoftwareClusterResource                                              = 198,
    /// DLT-CONTEXT
    DltContext                                                             = 199,
    /// SLOW-FLASHING-MODE
    SlowFlashingMode                                                       = 200,
    /// MN
    Mn                                                                     = 201,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 202,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 203,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 204,
    /// CONTINUE-AT-IT-POSITION
    ContinueAtItPosition                                                   = 205,
    /// AGREED
    Agreed                                                                 = 206,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 207,
    /// SECURITY-EVENT-AGGREGATION-FILTER
    SecurityEventAggregationFilter                                         = 208,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 209,
    /// SHOW-LONG-NAME
    ShowLongName                                                           = 210,
    /// CONFIRMED
    Confirmed                                                              = 211,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 212,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 213,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 214,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 215,
    /// NEW-IS-WITHIN
    NewIsWithin                                                            = 216,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 217,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED
    AdaptiveServiceSubscriptionCompleted                                   = 218,
    /// PRODUCER
    Producer                                                               = 219,
    /// KK
    Kk                                                                     = 220,
    /// CONCRETE
    Concrete                                                               = 221,
    /// ACCES-PERRMISSION-SERVICE-CLASS
    AccesPerrmissionServiceClass                                           = 222,
    /// CRC-NOT-SUPPORTED
    CrcNotSupported                                                        = 223,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 224,
    /// SW-MC-FRAME
    SwMcFrame                                                              = 225,
    /// V-2-X-MANAGEMENT
    V2XManagement                                                          = 226,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 227,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 228,
    /// DLNA
    Dlna                                                                   = 229,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 230,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 231,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 232,
    /// TIMING-DESCRIPTION-EVENT
    TimingDescriptionEvent                                                 = 233,
    /// OVERRIDE
    Override                                                               = 234,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 235,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 236,
    /// FIBEX-ELEMENT
    FibexElement                                                           = 237,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 238,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 239,
    /// MODE-DECLARATION-SWITCH-COMPLETED
    ModeDeclarationSwitchCompleted                                         = 240,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 241,
    /// REPORT-BEFORE-INIT
    ReportBeforeInit                                                       = 242,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 243,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 244,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 245,
    /// ON-COMPARISON-OF-VALUES
    OnComparisonOfValues                                                   = 246,
    /// DIAGNOSTIC-CONDITION
    DiagnosticCondition                                                    = 247,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 248,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 249,
    /// PHM-ACTION
    PhmAction                                                              = 250,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 251,
    /// RESPOND-BEFORE-RESET
    RespondBeforeReset                                                     = 252,
    /// TOPIC-PREFIX
    TopicPrefix                                                            = 253,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 254,
    /// EO
    Eo                                                                     = 255,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 256,
    /// CYCLIC
    Cyclic                                                                 = 257,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 258,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 259,
    /// EVAP
    Evap                                                                   = 260,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 261,
    /// CRYPTO-KEY-MANAGEMENT
    CryptoKeyManagement                                                    = 262,
    /// CYCLE-REPETITION-5
    CycleRepetition5                                                       = 263,
    /// NEW-IS-LESS
    NewIsLess                                                              = 264,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 265,
    /// TS
    Ts                                                                     = 266,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 267,
    /// REDUNDANT-PER-ELEMENT
    RedundantPerElement                                                    = 268,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 269,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 270,
    /// DLT-MESSAGE
    DltMessage                                                             = 271,
    /// SEARCH-FOR-ALL
    SearchForAll                                                           = 272,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 273,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 274,
    /// PROTECT-LAMP
    ProtectLamp                                                            = 275,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 276,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 277,
    /// EXECUTE
    Execute                                                                = 278,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 279,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 280,
    /// KL
    Kl                                                                     = 281,
    /// RESET-MCU
    ResetMcu                                                               = 282,
    /// API
    Api                                                                    = 283,
    /// TIMING-EVENT
    TimingEvent                                                            = 284,
    /// MACHINE
    Machine                                                                = 285,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 286,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 287,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 288,
    /// DATA-CONSTR
    DataConstr                                                             = 289,
    /// PGWIDE
    Pgwide                                                                 = 290,
    /// LEFT
    Left                                                                   = 291,
    /// SYMMETRIC
    Symmetric                                                              = 292,
    /// SHORT-HEADER
    ShortHeader                                                            = 293,
    /// TIMING-CONDITION
    TimingCondition                                                        = 294,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 295,
    /// TRACE
    Trace                                                                  = 296,
    /// LIN-CLUSTER
    LinCluster                                                             = 297,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 298,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 299,
    /// SERVER-ENCRYPT
    ServerEncrypt                                                          = 300,
    /// I-PDU-RECEIVED-BY-COM
    IPduReceivedByCom                                                      = 301,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 302,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 303,
    /// AA
    Aa                                                                     = 304,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 305,
    /// DIAGNOSTIC-CONDITION-GROUP
    DiagnosticConditionGroup                                               = 306,
    /// SWITCH
    Switch                                                                 = 307,
    /// NO-ACK
    NoAck                                                                  = 308,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 309,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 310,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 311,
    /// AZ
    Az                                                                     = 312,
    /// SERVER-MAC-GENERATE
    ServerMacGenerate                                                      = 313,
    /// COLLECTABLE-ELEMENT
    CollectableElement                                                     = 314,
    /// 10BASE-T1S
    _10baseT1s                                                             = 315,
    /// NE
    Ne                                                                     = 316,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 317,
    /// PACKAGEABLE-ELEMENT
    PackageableElement                                                     = 318,
    /// PORT
    Port                                                                   = 319,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 320,
    /// FIT-TO-PAGE
    FitToPage                                                              = 321,
    /// FIT-TO-TEXT
    FitToText                                                              = 322,
    /// NO-SUPERVISION
    NoSupervision                                                          = 323,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 324,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 325,
    /// TIP
    Tip                                                                    = 326,
    /// ON-TRANSITION
    OnTransition                                                           = 327,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 328,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 329,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 330,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 331,
    /// WILL-CALL
    WillCall                                                               = 332,
    /// API-USE
    ApiUse                                                                 = 333,
    /// PORT-INTERFACE-MAPPING
    PortInterfaceMapping                                                   = 334,
    /// TRACED-FAILURE
    TracedFailure                                                          = 335,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 336,
    /// SECURITY-EVENT-THRESHOLD-FILTER
    SecurityEventThresholdFilter                                           = 337,
    /// BA
    Ba                                                                     = 338,
    /// NO-OBD-SUPPORT
    NoObdSupport                                                           = 339,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 340,
    /// PORT-GROUP
    PortGroup                                                              = 341,
    /// MULTIPLE
    Multiple                                                               = 342,
    /// SIDES
    Sides                                                                  = 343,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 344,
    /// DIAGNOSTIC-MEMORY-BY-ADDRESS
    DiagnosticMemoryByAddress                                              = 345,
    /// NO-TRANSFORMER-STATUS-FORWARDING
    NoTransformerStatusForwarding                                          = 346,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 347,
    /// HY
    Hy                                                                     = 348,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 349,
    /// ARRAY
    Array                                                                  = 350,
    /// SHOW-SHORT-NAME
    ShowShortName                                                          = 351,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 352,
    /// UNDEFINED
    Undefined                                                              = 353,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 354,
    /// JUSTIFY
    Justify                                                                = 355,
    /// KA
    Ka                                                                     = 356,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 357,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 358,
    /// IS-NOT-EQUAL
    IsNotEqual                                                             = 359,
    /// VARIANT-PRE-COMPILE
    VariantPreCompile                                                      = 360,
    /// AFTERMARKET
    Aftermarket                                                            = 361,
    /// VALID
    Valid                                                                  = 362,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 363,
    /// 1000BASE-T1
    _1000baseT1                                                            = 364,
    /// PUT
    Put                                                                    = 365,
    /// USE-VOID
    UseVoid                                                                = 366,
    /// DLT-APPLICATION
    DltApplication                                                         = 367,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 368,
    /// APPLICATION-ASSOC-MAP-ELEMENT
    ApplicationAssocMapElement                                             = 369,
    /// SERVER-MAC-VERIFY
    ServerMacVerify                                                        = 370,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 371,
    /// VARIANT-POST-BUILD-SELECTABLE
    VariantPostBuildSelectable                                             = 372,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 373,
    /// STRICT-MONOTONOUS
    StrictMonotonous                                                       = 374,
    /// POWER
    Power                                                                  = 375,
    /// MALFUNCTION
    Malfunction                                                            = 376,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 377,
    /// WATCH-DOG-MANAGER
    WatchDogManager                                                        = 378,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 379,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 380,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 381,
    /// SECONDARY-ECU
    SecondaryEcu                                                           = 382,
    /// SYSTEM-DESIGN-TIME
    SystemDesignTime                                                       = 383,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 384,
    /// LOWER-8-BIT
    Lower8Bit                                                              = 385,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 386,
    /// KEY-STORAGE
    KeyStorage                                                             = 387,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 388,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 389,
    /// ASYNCHRONOUS
    Asynchronous                                                           = 390,
    /// ROTATE-90-CCW-LIMIT-TO-TEXT
    Rotate90CcwLimitToText                                                 = 391,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 392,
    /// WONT-SEND
    WontSend                                                               = 393,
    /// GL
    Gl                                                                     = 394,
    /// RN
    Rn                                                                     = 395,
    /// PUBLIC-KEY
    PublicKey                                                              = 396,
    /// SYNC-BASE-TIME-MANAGER
    SyncBaseTimeManager                                                    = 397,
    /// MASTER-ECU
    MasterEcu                                                              = 398,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 399,
    /// ANY-SEND-OPERATION
    AnySendOperation                                                       = 400,
    /// COMMON
    Common                                                                 = 401,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 402,
    /// PAYLOAD-AS-ARRAY
    PayloadAsArray                                                         = 403,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 404,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 405,
    /// CAUTION
    Caution                                                                = 406,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 407,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 408,
    /// OFFSET
    Offset                                                                 = 409,
    /// SATURATE
    Saturate                                                               = 410,
    /// REFERRABLE
    Referrable                                                             = 411,
    /// ANY-STANDARDIZED
    AnyStandardized                                                        = 412,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 413,
    /// LAND
    Land                                                                   = 414,
    /// XCP-PDU
    XcpPdu                                                                 = 415,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 416,
    /// MO
    Mo                                                                     = 417,
    /// LINK
    Link                                                                   = 418,
    /// BSW-MODULE-ENTITY
    BswModuleEntity                                                        = 419,
    /// SECOND-TO-FIRST
    SecondToFirst                                                          = 420,
    /// DSA
    Dsa                                                                    = 421,
    /// TRANSFORMER-ERROR-HANDLING
    TransformerErrorHandling                                               = 422,
    /// STATIC-PART-TRIGGER
    StaticPartTrigger                                                      = 423,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 424,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 425,
    /// ISO-15031--6
    Iso150316                                                              = 426,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 427,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 428,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 429,
    /// C
    C                                                                      = 430,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 431,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 432,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED
    AdaptiveServiceSubscriptionAcknowledgeCompleted                        = 433,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 434,
    /// FM-FEATURE
    FmFeature                                                              = 435,
    /// KEY-DERIVATION
    KeyDerivation                                                          = 436,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 437,
    /// SCHEDULE-VARIANT-1
    ScheduleVariant1                                                       = 438,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 439,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING
    ServiceInterfaceElementMapping                                         = 440,
    /// NOT-ACCESSIBLE
    NotAccessible                                                          = 441,
    /// AVB--IEEE-802--1-AS
    AvbIeee8021As                                                          = 442,
    /// ECUC-PARAMETER-DEF
    EcucParameterDef                                                       = 443,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 444,
    /// OVERWRITE
    Overwrite                                                              = 445,
    /// PRECONFIGURED-CONFIGURATION
    PreconfiguredConfiguration                                             = 446,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 447,
    /// JI
    Ji                                                                     = 448,
    /// SILENT
    Silent                                                                 = 449,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 450,
    /// TC
    Tc                                                                     = 451,
    /// USE-ARGUMENT-TYPE
    UseArgumentType                                                        = 452,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 453,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 454,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 455,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 456,
    /// APPLICATION
    Application                                                            = 457,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 458,
    /// ADAPTIVE-FIELD-NOTIFICATION-SENT
    AdaptiveFieldNotificationSent                                          = 459,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 460,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 461,
    /// MAPPING-SCOPE-PARTITION
    MappingScopePartition                                                  = 462,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 463,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 464,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 465,
    /// RPT-COMPONENT
    RptComponent                                                           = 466,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 467,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 468,
    /// AF
    Af                                                                     = 469,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 470,
    /// ACL-ROLE
    AclRole                                                                = 471,
    /// BSW
    Bsw                                                                    = 472,
    /// ABSTRACT-SECURITY-EVENT-FILTER
    AbstractSecurityEventFilter                                            = 473,
    /// RESPOND-AFTER-RESET
    RespondAfterReset                                                      = 474,
    /// ACCEPT-CONFIGURED
    AcceptConfigured                                                       = 475,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 476,
    /// DROP-UNTAGGED
    DropUntagged                                                           = 477,
    /// I-SIGNAL
    ISignal                                                                = 478,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 479,
    /// ENHANCED
    Enhanced                                                               = 480,
    /// CUSTOM
    Custom                                                                 = 481,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 482,
    /// INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE
    InheritedFromArrayElementTypeSize                                      = 483,
    /// BSW-M-ENTRY-CALL-RETURNED
    BswMEntryCallReturned                                                  = 484,
    /// SERVER-CALL-POINT
    ServerCallPoint                                                        = 485,
    /// TI
    Ti                                                                     = 486,
    /// TG
    Tg                                                                     = 487,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 488,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 489,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 490,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 491,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 492,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 493,
    /// IDS-MAPPING
    IdsMapping                                                             = 494,
    /// BLOCK-SOURCE
    BlockSource                                                            = 495,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 496,
    /// ROUTER-ADVERTISEMENT
    RouterAdvertisement                                                    = 497,
    /// FDC-THRESHOLD
    FdcThreshold                                                           = 498,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 499,
    /// FRAME-ETHERNET-SENT-ON-BUS
    FrameEthernetSentOnBus                                                 = 500,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 501,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 502,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 503,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 504,
    /// TRIGGER-RELEASED
    TriggerReleased                                                        = 505,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 506,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 507,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 508,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 509,
    /// RES-AXIS
    ResAxis                                                                = 510,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 511,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 512,
    /// SUPPORTS-BUFFER-LOCKING
    SupportsBufferLocking                                                  = 513,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 514,
    /// MAINTENANCE-ONLY
    MaintenanceOnly                                                        = 515,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 516,
    /// I-4-G
    I4G                                                                    = 517,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 518,
    /// BSW-MODULE-ENTITY-STARTED
    BswModuleEntityStarted                                                 = 519,
    /// API-BASED
    ApiBased                                                               = 520,
    /// SWC-TIMING
    SwcTiming                                                              = 521,
    /// GLOBAL-TIME-SLAVE
    GlobalTimeSlave                                                        = 522,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 523,
    /// MASEKD-NEW-EQUALS-X
    MasekdNewEqualsX                                                       = 524,
    /// ECUC-CONTAINER-DEF
    EcucContainerDef                                                       = 525,
    /// SWC
    Swc                                                                    = 526,
    /// GA
    Ga                                                                     = 527,
    /// NL
    Nl                                                                     = 528,
    /// CRYPTO-SERVICE-MANAGER
    CryptoServiceManager                                                   = 529,
    /// EXERCISE
    Exercise                                                               = 530,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 531,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 532,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 533,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 534,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 535,
    /// CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET
    CppImplementationDataTypeContextTarget                                 = 536,
    /// FIRST-TO-SECOND
    FirstToSecond                                                          = 537,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 538,
    /// AFTERMAKET
    Aftermaket                                                             = 539,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 540,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 541,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 542,
    /// DEFERRED
    Deferred                                                               = 543,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 544,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 545,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 546,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 547,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 548,
    /// TIFF
    Tiff                                                                   = 549,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 550,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 551,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 552,
    /// TOPBOT
    Topbot                                                                 = 553,
    /// STEADY
    Steady                                                                 = 554,
    /// RUN-ONCE
    RunOnce                                                                = 555,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 556,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 557,
    /// TRIGGERED
    Triggered                                                              = 558,
    /// RESPONSE-SYNCHRONIZATION
    ResponseSynchronization                                                = 559,
    /// ROLL-BACK
    RollBack                                                               = 560,
    /// PRIMITIVE
    Primitive                                                              = 561,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 562,
    /// REQUIRED-AP-SERVICE-INSTANCE
    RequiredApServiceInstance                                              = 563,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 564,
    /// FIX-AXIS
    FixAxis                                                                = 565,
    /// REPLACE
    Replace                                                                = 566,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 567,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 568,
    /// UP-LINK-PORT
    UpLinkPort                                                             = 569,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 570,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 571,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 572,
    /// HOST-PORT
    HostPort                                                               = 573,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 574,
    /// R-4--2
    R42                                                                    = 575,
    /// DA
    Da                                                                     = 576,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 577,
    /// NOTIFICATION
    Notification                                                           = 578,
    /// DO-IP-SERVICE-NEEDS
    DoIpServiceNeeds                                                       = 579,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 580,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 581,
    /// CRYPTO-SERVICE-MAPPING
    CryptoServiceMapping                                                   = 582,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 583,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 584,
    /// ADAPTIVE-METHOD-RESPONSE-RECEIVED
    AdaptiveMethodResponseReceived                                         = 585,
    /// SECURE-COMMUNICATION-DEPLOYMENT
    SecureCommunicationDeployment                                          = 586,
    /// SERVICE-DISCOVERY
    ServiceDiscovery                                                       = 587,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 588,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 589,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 590,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 591,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 592,
    /// ECUC-COMMON-ATTRIBUTES
    EcucCommonAttributes                                                   = 593,
    /// EOC-EXECUTABLE-ENTITY-REF-ABSTRACT
    EocExecutableEntityRefAbstract                                         = 594,
    /// ATP-BLUEPRINT
    AtpBlueprint                                                           = 595,
    /// CAPTION
    Caption                                                                = 596,
    /// OPERATION-CALLED
    OperationCalled                                                        = 597,
    /// ACCEPT-ALL
    AcceptAll                                                              = 598,
    /// TRANSLATION-START
    TranslationStart                                                       = 599,
    /// HINT
    Hint                                                                   = 600,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 601,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 602,
    /// SW-SERVICE-PROTOTYPE
    SwServicePrototype                                                     = 603,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 604,
    /// DEFAULT-TRACE-STATE-ENABLED
    DefaultTraceStateEnabled                                               = 605,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 606,
    /// TIME-SYNC-SERVER-CONFIGURATION
    TimeSyncServerConfiguration                                            = 607,
    /// SHOW-PAGE
    ShowPage                                                               = 608,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 609,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 610,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 611,
    /// RPT-CONTAINER
    RptContainer                                                           = 612,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 613,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 614,
    /// AR--CLIENT--SERVER
    ArClientServer                                                         = 615,
    /// ATOMIC-SW-COMPONENT-TYPE
    AtomicSwComponentType                                                  = 616,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 617,
    /// J-1939-DCM
    J1939Dcm                                                               = 618,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 619,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 620,
    /// TOP
    Top                                                                    = 621,
    /// KEYWORD-SET
    KeywordSet                                                             = 622,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 623,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 624,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 625,
    /// NORMALFIXED
    Normalfixed                                                            = 626,
    /// CLIENT-MAC-VERIFY
    ClientMacVerify                                                        = 627,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 628,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 629,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 630,
    /// SECURED-PDU-HEADER-08-BIT
    SecuredPduHeader08Bit                                                  = 631,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 632,
    /// EN
    En                                                                     = 633,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 634,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 635,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 636,
    /// NEWLINE-IF-NECESSARY
    NewlineIfNecessary                                                     = 637,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 638,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 639,
    /// CYCLE-REPETITION-16
    CycleRepetition16                                                      = 640,
    /// IPSEC
    Ipsec                                                                  = 641,
    /// RECOMMENDED-CONFIGURATION
    RecommendedConfiguration                                               = 642,
    /// SESSION-HANDLING-ACTIVE
    SessionHandlingActive                                                  = 643,
    /// GETTER-SETTER
    GetterSetter                                                           = 644,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 645,
    /// STORE-EVENT
    StoreEvent                                                             = 646,
    /// STRICT-MODE
    StrictMode                                                             = 647,
    /// CIRCLE
    Circle                                                                 = 648,
    /// IGNORE
    Ignore                                                                 = 649,
    /// DISABLE
    Disable                                                                = 650,
    /// SERVICE-ONLY
    ServiceOnly                                                            = 651,
    /// METHOD-MAPPING
    MethodMapping                                                          = 652,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 653,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 654,
    /// AGE
    Age                                                                    = 655,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 656,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 657,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 658,
    /// DIAGNOSTIC-SERVICE-INSTANCE
    DiagnosticServiceInstance                                              = 659,
    /// SW-MC-INTERFACE
    SwMcInterface                                                          = 660,
    /// UNNUMBER
    Unnumber                                                               = 661,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 662,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 663,
    /// ONE-EVERY-N
    OneEveryN                                                              = 664,
    /// FA
    Fa                                                                     = 665,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 666,
    /// I-SIGNAL-AVAILABLE-FOR-RTE
    ISignalAvailableForRte                                                 = 667,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 668,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 669,
    /// ECU-TIMING
    EcuTiming                                                              = 670,
    /// OPERATION-CALL-RESPONSE-SENT
    OperationCallResponseSent                                              = 671,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 672,
    /// AH
    Ah                                                                     = 673,
    /// EDGE-NODE
    EdgeNode                                                               = 674,
    /// ADAPTIVE-METHOD-CALLED
    AdaptiveMethodCalled                                                   = 675,
    /// ATP-TYPE
    AtpType                                                                = 676,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 677,
    /// OC
    Oc                                                                     = 678,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 679,
    /// START
    Start                                                                  = 680,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 681,
    /// SHOW-ALIAS-NAME
    ShowAliasName                                                          = 682,
    /// REDUNDANT
    Redundant                                                              = 683,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 684,
    /// ATTRIBUTE-TAILORING
    AttributeTailoring                                                     = 685,
    /// BINARY-MANIFEST-ADDRESSABLE-OBJECT
    BinaryManifestAddressableObject                                        = 686,
    /// STOP
    Stop                                                                   = 687,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 688,
    /// PASSTHROUGH
    Passthrough                                                            = 689,
    /// RAW-DATA-STREAM-MAPPING
    RawDataStreamMapping                                                   = 690,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 691,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 692,
    /// MAPPING-SCOPE-CORE
    MappingScopeCore                                                       = 693,
    /// CRC-SUPPORTED
    CrcSupported                                                           = 694,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 695,
    /// FIXED
    Fixed                                                                  = 696,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER
    DiagnosticAbstractDataIdentifier                                       = 697,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 698,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 699,
    /// APPLICATION-ARRAY-ELEMENT
    ApplicationArrayElement                                                = 700,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 701,
    /// ALWAYS
    Always                                                                 = 702,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT
    AbstractImplementationDataTypeElement                                  = 703,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 704,
    /// SIGN-WITH-ORIGIN-AUTHENTICATION
    SignWithOriginAuthentication                                           = 705,
    /// LOWER-12-BIT
    Lower12Bit                                                             = 706,
    /// default
    default                                                                = 707,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 708,
    /// DIAGNOSTIC-ROUTINE-SUBFUNCTION
    DiagnosticRoutineSubfunction                                           = 709,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 710,
    /// UZ
    Uz                                                                     = 711,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 712,
    /// CLEAR-ALL-DTCS
    ClearAllDtcs                                                           = 713,
    /// DEPENDANT
    Dependant                                                              = 714,
    /// ADAPTIVE-FIELD-NOTIFICATION-RECEIVED
    AdaptiveFieldNotificationReceived                                      = 715,
    /// NOTHING
    Nothing                                                                = 716,
    /// CY
    Cy                                                                     = 717,
    /// CALPRM
    Calprm                                                                 = 718,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 719,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 720,
    /// LT-AFFECTS-PB
    LtAffectsPb                                                            = 721,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 722,
    /// TE
    Te                                                                     = 723,
    /// SCHEDULE-VARIANT-2
    ScheduleVariant2                                                       = 724,
    /// GATEWAY
    Gateway                                                                = 725,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 726,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 727,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 728,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 729,
    /// EVENT-TRIGGERING-CONSTRAINT
    EventTriggeringConstraint                                              = 730,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 731,
    /// LIN-COMMUNICATION-CONTROLLER
    LinCommunicationController                                             = 732,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 733,
    /// CAN-FD
    CanFd                                                                  = 734,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 735,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 736,
    /// SECURED-PDU-HEADER-16-BIT
    SecuredPduHeader16Bit                                                  = 737,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 738,
    /// PRE-COMPILE
    PreCompile                                                             = 739,
    /// AR-PACKAGE
    ArPackage                                                              = 740,
    /// VAR-POWER-ON-INIT
    VarPowerOnInit                                                         = 741,
    /// NOT-DEFINED
    NotDefined                                                             = 742,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 743,
    /// SELECTED
    Selected                                                               = 744,
    /// CYCLE-REPETITION-32
    CycleRepetition32                                                      = 745,
    /// UCM-STEP
    UcmStep                                                                = 746,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 747,
    /// SLAVE
    Slave                                                                  = 748,
    /// PDF
    Pdf                                                                    = 749,
    /// KEYWORD
    Keyword                                                                = 750,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 751,
    /// CODEGENERATION
    Codegeneration                                                         = 752,
    /// ENCRYPTION
    Encryption                                                             = 753,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 754,
    /// TOPIC-1
    Topic1                                                                 = 755,
    /// NEW-IS-DIFFERENT
    NewIsDifferent                                                         = 756,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 757,
    /// PROCESS
    Process                                                                = 758,
    /// BUILD-ACTION-ENTITY
    BuildActionEntity                                                      = 759,
    /// I-PDU
    IPdu                                                                   = 760,
    /// DETAILED-BYPASSING-FILTERS
    DetailedBypassingFilters                                               = 761,
    /// KY
    Ky                                                                     = 762,
    /// TK
    Tk                                                                     = 763,
    /// UDP-CHECKSUM-DISABLED
    UdpChecksumDisabled                                                    = 764,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 765,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 766,
    /// DOES-NOT-REPORT-EXECUTION-STATE
    DoesNotReportExecutionState                                            = 767,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 768,
    /// TO
    To                                                                     = 769,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 770,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 771,
    /// AUTHENTICATE
    Authenticate                                                           = 772,
    /// WRITE-ONLY
    WriteOnly                                                              = 773,
    /// DIAGNOSTIC-EVENT-MANAGER
    DiagnosticEventManager                                                 = 774,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 775,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 776,
    /// RESTART-APPLICATION
    RestartApplication                                                     = 777,
    /// DIAGNOSTIC-PORT-INTERFACE
    DiagnosticPortInterface                                                = 778,
    /// VERIFICATION
    Verification                                                           = 779,
    /// CAN-BE-TERMINATED
    CanBeTerminated                                                        = 780,
    /// ATP-FEATURE
    AtpFeature                                                             = 781,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 782,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 783,
    /// BASIC-SOFTWARE-MODE-MANAGER
    BasicSoftwareModeManager                                               = 784,
    /// MI
    Mi                                                                     = 785,
    /// FALSE
    False                                                                  = 786,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 787,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 788,
    /// DEFAULT-IF-UNDEFINED
    DefaultIfUndefined                                                     = 789,
    /// SL
    Sl                                                                     = 790,
    /// CONFIGURED
    Configured                                                             = 791,
    /// PUBLISHED-INFORMATION
    PublishedInformation                                                   = 792,
    /// STRICTLY-DECREASING
    StrictlyDecreasing                                                     = 793,
    /// SINGLE-LANGUAGE-REFERRABLE
    SingleLanguageReferrable                                               = 794,
    /// BUILD-TYPE-RELEASE
    BuildTypeRelease                                                       = 795,
    /// HI
    Hi                                                                     = 796,
    /// LONG-HEADER
    LongHeader                                                             = 797,
    /// GET
    Get                                                                    = 798,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 799,
    /// BASE-T
    BaseT                                                                  = 800,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 801,
    /// ABSTRACT-ETHERNET-FRAME
    AbstractEthernetFrame                                                  = 802,
    /// IGNITION
    Ignition                                                               = 803,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 804,
    /// HU
    Hu                                                                     = 805,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 806,
    /// SVG
    Svg                                                                    = 807,
    /// MASEKD-NEW-EQUALS-MASKED-OLD
    MasekdNewEqualsMaskedOld                                               = 808,
    /// PDU-R
    PduR                                                                   = 809,
    /// HIGH
    High                                                                   = 810,
    /// RPT-LEVEL-1
    RptLevel1                                                              = 811,
    /// NO-SHOW-LONG-NAME
    NoShowLongName                                                         = 812,
    /// DOES-NOT-SUPPORT-BUFFER-LOCKING
    DoesNotSupportBufferLocking                                            = 813,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 814,
    /// CONSUMER
    Consumer                                                               = 815,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 816,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 817,
    /// NM-ECU
    NmEcu                                                                  = 818,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 819,
    /// USE-ARRAY-BASE-TYPE
    UseArrayBaseType                                                       = 820,
    /// AFTER-SALES
    AfterSales                                                             = 821,
    /// TRIGGER-UNICAST
    TriggerUnicast                                                         = 822,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 823,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 824,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 825,
    /// PTP--IEEE-1588--2002
    PtpIeee15882002                                                        = 826,
    /// REQUEST-CALLBACK-TYPE-MANUFACTURER
    RequestCallbackTypeManufacturer                                        = 827,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 828,
    /// CHECK-AT-NEXT-HALT
    CheckAtNextHalt                                                        = 829,
    /// SW-VARIABLE-PROTOTYPE
    SwVariablePrototype                                                    = 830,
    /// CALIBRATION-ONLINE
    CalibrationOnline                                                      = 831,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 832,
    /// OPTIONS
    Options                                                                = 833,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 834,
    /// OPERATING-SYSTEM
    OperatingSystem                                                        = 835,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 836,
    /// MEASUREMENT-POINT
    MeasurementPoint                                                       = 837,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 838,
    /// GN
    Gn                                                                     = 839,
    /// ICMP
    Icmp                                                                   = 840,
    /// DO-IP
    DoIp                                                                   = 841,
    /// MASKED-NEW-EQUALS-X
    MaskedNewEqualsX                                                       = 842,
    /// XXG-MII
    XxgMii                                                                 = 843,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 844,
    /// SV
    Sv                                                                     = 845,
    /// IA
    Ia                                                                     = 846,
    /// NETWORK
    Network                                                                = 847,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 848,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 849,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 850,
    /// SW-CLASS-ATTR-IMPL
    SwClassAttrImpl                                                        = 851,
    /// DZ
    Dz                                                                     = 852,
    /// XREF-TARGET
    XrefTarget                                                             = 853,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 854,
    /// RUNNABLE-ENTITY-STARTED
    RunnableEntityStarted                                                  = 855,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 856,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 857,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 858,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 859,
    /// STORE-PERSISTENTLY
    StorePersistently                                                      = 860,
    /// COMMUNICATION-CLUSTER
    CommunicationCluster                                                   = 861,
    /// FLAT-MAP
    FlatMap                                                                = 862,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 863,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 864,
    /// TW
    Tw                                                                     = 865,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 866,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 867,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 868,
    /// HEALTH-CHANNEL
    HealthChannel                                                          = 869,
    /// REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE
    ReportDtcRecordInformationOnDtcStatusChange                            = 870,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 871,
    /// BO
    Bo                                                                     = 872,
    /// SD
    Sd                                                                     = 873,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 874,
    /// ACCESS-PERMISSION-SERVICE-INSTANCE
    AccessPermissionServiceInstance                                        = 875,
    /// OTHER
    Other                                                                  = 876,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 877,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 878,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 879,
    /// ESP
    Esp                                                                    = 880,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 881,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 882,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 883,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 884,
    /// TP-CONFIG
    TpConfig                                                               = 885,
    /// PERSISTENCY-INTERFACE-ELEMENT
    PersistencyInterfaceElement                                            = 886,
    /// ACTIVATE
    Activate                                                               = 887,
    /// FRAME-TRIGGERING
    FrameTriggering                                                        = 888,
    /// NON-OS-MODULE-INSTANTIATION
    NonOsModuleInstantiation                                               = 889,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 890,
    /// SESSION-HANDLING-INACTIVE
    SessionHandlingInactive                                                = 891,
    /// UNIT-GROUP
    UnitGroup                                                              = 892,
    /// TIMING-CONSTRAINT
    TimingConstraint                                                       = 893,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 894,
    /// FO
    Fo                                                                     = 895,
    /// CLOSED
    Closed                                                                 = 896,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 897,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 898,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 899,
    /// RETURN-VALUE-PROVIDED
    ReturnValueProvided                                                    = 900,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 901,
    /// NO-TRUSTED-PLATFORM-SUPPORT
    NoTrustedPlatformSupport                                               = 902,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 903,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 904,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 905,
    /// ALTERNATING-8-BIT
    Alternating8Bit                                                        = 906,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 907,
    /// PROCESSOR
    Processor                                                              = 908,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 909,
    /// SW-MC-INTERFACE-SOURCE
    SwMcInterfaceSource                                                    = 910,
    /// TLS-CRYPTO-CIPHER-SUITE-PROPS
    TlsCryptoCipherSuiteProps                                              = 911,
    /// OEM-BOOT
    OemBoot                                                                = 912,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 913,
    /// HOOK
    Hook                                                                   = 914,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 915,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 916,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 917,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 918,
    /// FUNCTIONAL-ADDRESS
    FunctionalAddress                                                      = 919,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 920,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 921,
    /// CONST
    Const                                                                  = 922,
    /// CRC-VALIDATED
    CrcValidated                                                           = 923,
    /// ISO-6
    Iso6                                                                   = 924,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 925,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 926,
    /// FAULT
    Fault                                                                  = 927,
    /// ABSTRACT-CAN-PHYSICAL-CHANNEL
    AbstractCanPhysicalChannel                                             = 928,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 929,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 930,
    /// ADDR-METHOD-SHORT-NAME
    AddrMethodShortName                                                    = 931,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 932,
    /// NEVER
    Never                                                                  = 933,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 934,
    /// AUTO
    Auto                                                                   = 935,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 936,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 937,
    /// PHYSICAL-CHANNEL
    PhysicalChannel                                                        = 938,
    /// EQUAL
    Equal                                                                  = 939,
    /// FUNCTION-INHIBITION-MANAGER
    FunctionInhibitionManager                                              = 940,
    /// COMMAND-LINE-SHORT-FORM
    CommandLineShortForm                                                   = 941,
    /// POST
    Post                                                                   = 942,
    /// ACTIVATION-MULTICAST
    ActivationMulticast                                                    = 943,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 944,
    /// AS
    As                                                                     = 945,
    /// NEW-IS-OUTSIDE
    NewIsOutside                                                           = 946,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 947,
    /// NONE
    None                                                                   = 948,
    /// DEFAULT-MODE
    DefaultMode                                                            = 949,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 950,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 951,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 952,
    /// COM-GRANT
    ComGrant                                                               = 953,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 954,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 955,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 956,
    /// ATP-DEFINITION
    AtpDefinition                                                          = 957,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 958,
    /// CP
    Cp                                                                     = 959,
    /// RESET-VM
    ResetVm                                                                = 960,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 961,
    /// NO-RETURN-VALUE-PROVIDED
    NoReturnValueProvided                                                  = 962,
    /// COMPU-METHOD
    CompuMethod                                                            = 963,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 964,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 965,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 966,
    /// NO-AFFECT
    NoAffect                                                               = 967,
    /// ES
    Es                                                                     = 968,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 969,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 970,
    /// NOT-EQUAL
    NotEqual                                                               = 971,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 972,
    /// OCCURENCE
    Occurence                                                              = 973,
    /// INTERRUPT
    Interrupt                                                              = 974,
    /// PERIODIC-RATE-SLOW
    PeriodicRateSlow                                                       = 975,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 976,
    /// UDP
    Udp                                                                    = 977,
    /// CYCLE-REPETITION-20
    CycleRepetition20                                                      = 978,
    /// HW-ELEMENT
    HwElement                                                              = 979,
    /// TRANSPORT
    Transport                                                              = 980,
    /// SERVER-AUTHENTICATE
    ServerAuthenticate                                                     = 981,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 982,
    /// FRAME
    Frame                                                                  = 983,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 984,
    /// IS
    Is                                                                     = 985,
    /// NO-DEFAULT
    NoDefault                                                              = 986,
    /// TD-EVENT-SWC
    TdEventSwc                                                             = 987,
    /// PORT-PROTOTYPE
    PortPrototype                                                          = 988,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 989,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 990,
    /// GRANT-DESIGN
    GrantDesign                                                            = 991,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 992,
    /// FAILURE-ONLY
    FailureOnly                                                            = 993,
    /// LTS-13
    Lts13                                                                  = 994,
    /// BOLDITALIC
    Bolditalic                                                             = 995,
    /// TRIGGER
    Trigger                                                                = 996,
    /// LOW
    Low                                                                    = 997,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 998,
    /// NO-KEEP
    NoKeep                                                                 = 999,
    /// SLP
    Slp                                                                    = 1000,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 1001,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 1002,
    /// NEW-IS-LESS-OR-EQUAL
    NewIsLessOrEqual                                                       = 1003,
    /// BROAD-R-REACH
    BroadRReach                                                            = 1004,
    /// NO-PROTECTION
    NoProtection                                                           = 1005,
    /// VARIANT-POST-BUILD
    VariantPostBuild                                                       = 1006,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 1007,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED
    AdaptiveServiceSubscriptionStarted                                     = 1008,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 1009,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 1010,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 1011,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 1012,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 1013,
    /// OBD-DCY
    ObdDcy                                                                 = 1014,
    /// DLT-ARGUMENT
    DltArgument                                                            = 1015,
    /// TRACEABLE-TEXT
    TraceableText                                                          = 1016,
    /// ECC
    Ecc                                                                    = 1017,
    /// STD_AXIS
    Stdaxis                                                                = 1018,
    /// NTP--RFC-958
    NtpRfc958                                                              = 1019,
    /// APPLICATION-DATA-TYPE
    ApplicationDataType                                                    = 1020,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 1021,
    /// ROTATE-90-CCW-FIT-TO-TEXT
    Rotate90CcwFitToText                                                   = 1022,
    /// REQUEST-CALLBACK-TYPE-SUPPLIER
    RequestCallbackTypeSupplier                                            = 1023,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 1024,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 1025,
    /// ABSTRACT-IAM-REMOTE-SUBJECT
    AbstractIamRemoteSubject                                               = 1026,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 1027,
    /// SERVICE-INSTANCE-TO-MACHINE-MAPPING
    ServiceInstanceToMachineMapping                                        = 1028,
    /// SPEC-ELEMENT-SCOPE
    SpecElementScope                                                       = 1029,
    /// CYCLE-REPETITION-1
    CycleRepetition1                                                       = 1030,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 1031,
    /// LINK-LOCAL--DOIP
    LinkLocalDoip                                                          = 1032,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 1033,
    /// OBSERVER
    Observer                                                               = 1034,
    /// ADAPTIVE-SERVICE-FIND-STARTED
    AdaptiveServiceFindStarted                                             = 1035,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 1036,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 1037,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 1038,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 1039,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 1040,
    /// INSTALL
    Install                                                                = 1041,
    /// ACTION-ITEM
    ActionItem                                                             = 1042,
    /// CAPTURE-ASYNCHRONOUSLY-TO-REPORTING
    CaptureAsynchronouslyToReporting                                       = 1043,
    /// RETURN-ON-EVENT-CLEARED
    ReturnOnEventCleared                                                   = 1044,
    /// PHM-RULE
    PhmRule                                                                = 1045,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 1046,
    /// MACHINE-TIMING
    MachineTiming                                                          = 1047,
    /// OPERATION-CALL-RECEIVED
    OperationCallReceived                                                  = 1048,
    /// PERIODIC-RATE-FAST
    PeriodicRateFast                                                       = 1049,
    /// NO-NEWLINE
    NoNewline                                                              = 1050,
    /// SHOW-CONTENT
    ShowContent                                                            = 1051,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 1052,
    /// RM
    Rm                                                                     = 1053,
    /// CLIENT-AUTHENTICATE
    ClientAuthenticate                                                     = 1054,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 1055,
    /// CONTINUOUS-ON-MODE
    ContinuousOnMode                                                       = 1056,
    /// VERIFY
    Verify                                                                 = 1057,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 1058,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 1059,
    /// SM
    Sm                                                                     = 1060,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 1061,
    /// AS-IS
    AsIs                                                                   = 1062,
    /// PHYSICAL-CAN-FD
    PhysicalCanFd                                                          = 1063,
    /// UNSPECIFIED
    Unspecified                                                            = 1064,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 1065,
    /// WEIGHTED-ROUND-ROBIN
    WeightedRoundRobin                                                     = 1066,
    /// FAST-FLASHING-MODE
    FastFlashingMode                                                       = 1067,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 1068,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 1069,
    /// TX-TRIGGER-SINGLE
    TxTriggerSingle                                                        = 1070,
    /// NA
    Na                                                                     = 1071,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 1072,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 1073,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 1074,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 1075,
    /// TX-REF-TRIGGER-GAP
    TxRefTriggerGap                                                        = 1076,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 1077,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 1078,
    /// EXCLUDE-FROM-FLASH
    ExcludeFromFlash                                                       = 1079,
    /// GETTER
    Getter                                                                 = 1080,
    /// STANDARD
    Standard                                                               = 1081,
    /// INDIVIDUAL
    Individual                                                             = 1082,
    /// CYCLE-REPETITION-40
    CycleRepetition40                                                      = 1083,
    /// IS-LESS-THAN
    IsLessThan                                                             = 1084,
    /// MODE-ACCESS-POINT-IDENT
    ModeAccessPointIdent                                                   = 1085,
    /// DEFINE-BY-IDENTIFIER
    DefineByIdentifier                                                     = 1086,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 1087,
    /// FRAME-ETHERNET-RECEIVED-ON-BUS
    FrameEthernetReceivedOnBus                                             = 1088,
    /// FAILURE-AND-SUCCESS
    FailureAndSuccess                                                      = 1089,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 1090,
    /// MK
    Mk                                                                     = 1091,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 1092,
    /// ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF
    EcucAbstractExternalReferenceDef                                       = 1093,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 1094,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 1095,
    /// ENABLE
    Enable                                                                 = 1096,
    /// AB
    Ab                                                                     = 1097,
    /// CAT-2
    Cat2                                                                   = 1098,
    /// DEDICATED
    Dedicated                                                              = 1099,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 1100,
    /// NO-SHOW-NUMBER
    NoShowNumber                                                           = 1101,
    /// TIMING-DESCRIPTION
    TimingDescription                                                      = 1102,
    /// CO
    Co                                                                     = 1103,
    /// BUILD
    Build                                                                  = 1104,
    /// RTE-EVENT
    RteEvent                                                               = 1105,
    /// MODE-TRANSITION
    ModeTransition                                                         = 1106,
    /// ISO-11992--4
    Iso119924                                                              = 1107,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 1108,
    /// CVC
    Cvc                                                                    = 1109,
    /// INSTANCE-ID
    InstanceId                                                             = 1110,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 1111,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 1112,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 1113,
    /// HR
    Hr                                                                     = 1114,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 1115,
    /// PORT-INTERFACE
    PortInterface                                                          = 1116,
    /// OPERATION-CALL-RESPONSE-RECEIVED
    OperationCallResponseReceived                                          = 1117,
    /// CHANNEL-B
    ChannelB                                                               = 1118,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 1119,
    /// TIMING-EXTENSION-RESOURCE
    TimingExtensionResource                                                = 1120,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 1121,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 1122,
    /// CRYPTO-INTERFACE
    CryptoInterface                                                        = 1123,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 1124,
    /// ACK-WITHOUT-RT
    AckWithoutRt                                                           = 1125,
    /// DE
    De                                                                     = 1126,
    /// DEFICIT-ROUND-ROBIN
    DeficitRoundRobin                                                      = 1127,
    /// SETTER
    Setter                                                                 = 1128,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 1129,
    /// VAR-NO-INIT
    VarNoInit                                                              = 1130,
    /// ECUC-QUERY
    EcucQuery                                                              = 1131,
    /// VARIABLE-DATA-PROTOTYPE-RECEIVED
    VariableDataPrototypeReceived                                          = 1132,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 1133,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 1134,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 1135,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 1136,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 1137,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 1138,
    /// SHOW-NUMBER
    ShowNumber                                                             = 1139,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 1140,
    /// UR
    Ur                                                                     = 1141,
    /// USE-LAST-CONTEXT-DATA
    UseLastContextData                                                     = 1142,
    /// COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING
    CompositionPortToExecutablePortMapping                                 = 1143,
    /// WARMUP
    Warmup                                                                 = 1144,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 1145,
    /// AR-ELEMENT
    ArElement                                                              = 1146,
    /// COM-AXIS
    ComAxis                                                                = 1147,
    /// TRIGGERED-ON-CHANGE-WITHOUT-REPETITION
    TriggeredOnChangeWithoutRepetition                                     = 1148,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 1149,
    /// VOLATILE
    Volatile                                                               = 1150,
    /// RESET-ECU
    ResetEcu                                                               = 1151,
    /// CRC-IGNORED
    CrcIgnored                                                             = 1152,
    /// WARN
    Warn                                                                   = 1153,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 1154,
    /// HUB
    Hub                                                                    = 1155,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 1156,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 1157,
    /// EL
    El                                                                     = 1158,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 1159,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 1160,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 1161,
    /// GZIP
    Gzip                                                                   = 1162,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 1163,
    /// IS-GREATER-THAN
    IsGreaterThan                                                          = 1164,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 1165,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 1166,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 1167,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 1168,
    /// RPT-ENABLER-ROM
    RptEnablerRom                                                          = 1169,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 1170,
    /// PNC-MAPPING-IDENT
    PncMappingIdent                                                        = 1171,
    /// FIELD
    Field                                                                  = 1172,
    /// ALLOCATOR
    Allocator                                                              = 1173,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 1174,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 1175,
    /// TRANSFORMING-I-SIGNAL
    TransformingISignal                                                    = 1176,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 1177,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 1178,
    /// ISO-14229--1
    Iso142291                                                              = 1179,
    /// SEARCH-FOR-SPECIFIC-INSTANCE
    SearchForSpecificInstance                                              = 1180,
    /// DELETE
    Delete                                                                 = 1181,
    /// EXCLUSIVE
    Exclusive                                                              = 1182,
    /// PDU
    Pdu                                                                    = 1183,
    /// USER-DEFINED
    UserDefined                                                            = 1184,
    /// CAN-NM-NODE
    CanNmNode                                                              = 1185,
    /// ZU
    Zu                                                                     = 1186,
    /// OBD
    Obd                                                                    = 1187,
    /// CONNECT
    Connect                                                                = 1188,
    /// CLIENT-DECRYPT
    ClientDecrypt                                                          = 1189,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 1190,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE
    PersistencyRedundancyHandlingScopeStorage                              = 1191,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 1192,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 1193,
    /// TESTED-AND-FAILED
    TestedAndFailed                                                        = 1194,
    /// CYCLE-REPETITION-64
    CycleRepetition64                                                      = 1195,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 1196,
    /// STOP-TRIGGER
    StopTrigger                                                            = 1197,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 1198,
    /// WILL-RECEIVE
    WillReceive                                                            = 1199,
    /// XH
    Xh                                                                     = 1200,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 1201,
    /// READ-WRITE
    ReadWrite                                                              = 1202,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 1203,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 1204,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 1205,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 1206,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 1207,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 1208,
    /// ABSTRACT
    Abstract                                                               = 1209,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 1210,
    /// MASKED-NEW-DIFFERS-X
    MaskedNewDiffersX                                                      = 1211,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 1212,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 1213,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 1214,
    /// INCREASING
    Increasing                                                             = 1215,
    /// IS-FAILED
    IsFailed                                                               = 1216,
    /// EXECUTION-TIME
    ExecutionTime                                                          = 1217,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 1218,
    /// RPT-PROFILE
    RptProfile                                                             = 1219,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 1220,
    /// ECUC-DEFINITION-ELEMENT
    EcucDefinitionElement                                                  = 1221,
    /// ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION
    EncryptAndSignWithOriginAuthentication                                 = 1222,
    /// INTER-PARTITION-INTRA-ECU
    InterPartitionIntraEcu                                                 = 1223,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 1224,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 1225,
    /// DYNAMIC-PART-TRIGGER
    DynamicPartTrigger                                                     = 1226,
    /// EXAMPLE
    Example                                                                = 1227,
    /// SECURITY-EVENT-STATE-FILTER
    SecurityEventStateFilter                                               = 1228,
    /// INOUT
    Inout                                                                  = 1229,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 1230,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 1231,
    /// CLIENT-MAC-GENERATE
    ClientMacGenerate                                                      = 1232,
    /// SHOW-SEE
    ShowSee                                                                = 1233,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 1234,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 1235,
    /// BLINK-OR-CONTINUOUS-ON-MODE
    BlinkOrContinuousOnMode                                                = 1236,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 1237,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 1238,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 1239,
    /// RAW
    Raw                                                                    = 1240,
    /// YO
    Yo                                                                     = 1241,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 1242,
    /// REF-NON-STANDARD
    RefNonStandard                                                         = 1243,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 1244,
    /// UCM-MASTER
    UcmMaster                                                              = 1245,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 1246,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 1247,
    /// REPORT-AFTER-INIT
    ReportAfterInit                                                        = 1248,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 1249,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 1250,
    /// PERSISTENCY-DEPLOYMENT
    PersistencyDeployment                                                  = 1251,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 1252,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 1253,
    /// MG
    Mg                                                                     = 1254,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 1255,
    /// SECURE-COM-PROPS
    SecureComProps                                                         = 1256,
    /// TN
    Tn                                                                     = 1257,
    /// MASKED-NEW-EQUALS-MASKED-OLD
    MaskedNewEqualsMaskedOld                                               = 1258,
    /// CRYPTO-NEEDS
    CryptoNeeds                                                            = 1259,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 1260,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 1261,
    /// IS-EQUAL
    IsEqual                                                                = 1262,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 1263,
    /// LO
    Lo                                                                     = 1264,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 1265,
    /// ROUTER
    Router                                                                 = 1266,
    /// INTRUSION-DETECTION-SECURITY-MANAGEMENT
    IntrusionDetectionSecurityManagement                                   = 1267,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 1268,
    /// STD
    Std                                                                    = 1269,
    /// CAN-BE-TERMINATED-AND-RESTARTED
    CanBeTerminatedAndRestarted                                            = 1270,
    /// NO-SHOW-PAGE
    NoShowPage                                                             = 1271,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 1272,
    /// PRESENTATION-DISCRETE
    PresentationDiscrete                                                   = 1273,
    /// APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE
    ApplicationCompositeElementDataPrototype                               = 1274,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 1275,
    /// IT
    It                                                                     = 1276,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 1277,
    /// REST-ABSTRACT-PROPERTY-DEF
    RestAbstractPropertyDef                                                = 1278,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 1279,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 1280,
    /// RU
    Ru                                                                     = 1281,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 1282,
    /// COMPILE
    Compile                                                                = 1283,
    /// SEARCH-FOR-ID
    SearchForId                                                            = 1284,
    /// ET
    Et                                                                     = 1285,
    /// INLINE
    Inline                                                                 = 1286,
    /// DCM-I-PDU
    DcmIPdu                                                                = 1287,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 1288,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 1289,
    /// REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST
    ReportingInChronlogicalOrderOldestFirst                                = 1290,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 1291,
    /// ALL-16-BIT
    All16Bit                                                               = 1292,
    /// GROSS
    Gross                                                                  = 1293,
    /// INDICATE
    Indicate                                                               = 1294,
    /// MT
    Mt                                                                     = 1295,
    /// IS-LESS-OR-EQUAL
    IsLessOrEqual                                                          = 1296,
    /// XCP
    Xcp                                                                    = 1297,
    /// FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION
    FrameEthernetQueuedForTransmission                                     = 1298,
    /// SCHEDULE-VARIANT-7
    ScheduleVariant7                                                       = 1299,
    /// NM-NODE
    NmNode                                                                 = 1300,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 1301,
    /// SOMEIP
    Someip                                                                 = 1302,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 1303,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 1304,
    /// FULL
    Full                                                                   = 1305,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 1306,
    /// NO-SHOW-CATEGORY
    NoShowCategory                                                         = 1307,
    /// INFINITE
    Infinite                                                               = 1308,
    /// TESTED
    Tested                                                                 = 1309,
    /// ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER
    AbstractSecurityIdsmInstanceFilter                                     = 1310,
    /// VARIANT-LINK-TIME
    VariantLinkTime                                                        = 1311,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 1312,
    /// RUNNABLE-ENTITY-ACTIVATED
    RunnableEntityActivated                                                = 1313,
    /// NO-BREAK
    NoBreak                                                                = 1314,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 1315,
    /// ABSTRACT-CAN-CLUSTER
    AbstractCanCluster                                                     = 1316,
    /// IDENT-CAPTION
    IdentCaption                                                           = 1317,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 1318,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 1319,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 1320,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 1321,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 1322,
    /// SINGLE-CORE-REENTRANT
    SingleCoreReentrant                                                    = 1323,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 1324,
    /// SERVICE-METHOD-DEPLOYMENT
    ServiceMethodDeployment                                                = 1325,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 1326,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 1327,
    /// SUPPLIER
    Supplier                                                               = 1328,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 1329,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 1330,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 1331,
    /// XOR
    Xor                                                                    = 1332,
    /// CPP
    Cpp                                                                    = 1333,
    /// HALF-DUPLEX-MODE
    HalfDuplexMode                                                         = 1334,
    /// TRANSFORMATION-PROPS
    TransformationProps                                                    = 1335,
    /// SQ
    Sq                                                                     = 1336,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 1337,
    /// TR
    Tr                                                                     = 1338,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 1339,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 1340,
    /// SO
    So                                                                     = 1341,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 1342,
    /// STD-AXIS
    StdAxis                                                                = 1343,
    /// MC-FUNCTION
    McFunction                                                             = 1344,
    /// IMPLEMENTATION-PROPS
    ImplementationProps                                                    = 1345,
    /// GLOBAL-TIME-MASTER
    GlobalTimeMaster                                                       = 1346,
    /// NEW-IS-GREATER
    NewIsGreater                                                           = 1347,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 1348,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 1349,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 1350,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 1351,
    /// DESELECTED
    Deselected                                                             = 1352,
    /// MOST-SIGNIFICANT-BYTE-FIRST
    MostSignificantByteFirst                                               = 1353,
    /// TA
    Ta                                                                     = 1354,
    /// VAR-FAST
    VarFast                                                                = 1355,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 1356,
    /// OPAQUE
    Opaque                                                                 = 1357,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 1358,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 1359,
    /// DESCENDANT
    Descendant                                                             = 1360,
    /// ADAPTIVE-FIELD-SETTER-CALLED
    AdaptiveFieldSetterCalled                                              = 1361,
    /// CODE-GENERATION-TIME
    CodeGenerationTime                                                     = 1362,
    /// BONJOUR
    Bonjour                                                                = 1363,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 1364,
    /// DEBOUNCE-DATA
    DebounceData                                                           = 1365,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 1366,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 1367,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 1368,
    /// ANY
    Any                                                                    = 1369,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 1370,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 1371,
    /// SW-CALPRM-PROTOTYPE
    SwCalprmPrototype                                                      = 1372,
    /// ALL-INDICES-DIFFERENT-ARRAY-SIZE
    AllIndicesDifferentArraySize                                           = 1373,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 1374,
    /// JAVA
    Java                                                                   = 1375,
    /// COMMUNICATION-INTRA-PARTITION
    CommunicationIntraPartition                                            = 1376,
    /// LIN-TP-NODE
    LinTpNode                                                              = 1377,
    /// FUNCTIONAL
    Functional                                                             = 1378,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 1379,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 1380,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 1381,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 1382,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 1383,
    /// DLT-ECU
    DltEcu                                                                 = 1384,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 1385,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 1386,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 1387,
    /// BSW-EVENT
    BswEvent                                                               = 1388,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 1389,
    /// PA
    Pa                                                                     = 1390,
    /// LIN-SLAVE-CONFIG-IDENT
    LinSlaveConfigIdent                                                    = 1391,
    /// AUTONOMOUS
    Autonomous                                                             = 1392,
    /// SDG-CAPTION
    SdgCaption                                                             = 1393,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 1394,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 1395,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 1396,
    /// EXACT-OR-ANY-MINOR-VERSION
    ExactOrAnyMinorVersion                                                 = 1397,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 1398,
    /// CENTER
    Center                                                                 = 1399,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 1400,
    /// DEBUG
    Debug                                                                  = 1401,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 1402,
    /// DEFLATE
    Deflate                                                                = 1403,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 1404,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 1405,
    /// LEAF-OF-TARGET-CONTAINER
    LeafOfTargetContainer                                                  = 1406,
    /// ATP-CLASSIFIER
    AtpClassifier                                                          = 1407,
    /// ADAPTIVE-METHOD-RESPONSE-SENT
    AdaptiveMethodResponseSent                                             = 1408,
    /// STANDARD-PORT
    StandardPort                                                           = 1409,
    /// PAYLOAD-AS-POINTER-TO-ARRAY
    PayloadAsPointerToArray                                                = 1410,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 1411,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 1412,
    /// EVENT-COMBINATION-ON-STORAGE
    EventCombinationOnStorage                                              = 1413,
    /// NM-PDU
    NmPdu                                                                  = 1414,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 1415,
    /// TH
    Th                                                                     = 1416,
    /// SERVER-VERIFY
    ServerVerify                                                           = 1417,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 1418,
    /// ECU-MAPPING
    EcuMapping                                                             = 1419,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 1420,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 1421,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 1422,
    /// IS-GREATER-OR-EQUAL
    IsGreaterOrEqual                                                       = 1423,
    /// AM
    Am                                                                     = 1424,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 1425,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 1426,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 1427,
    /// TRANSFER
    Transfer                                                               = 1428,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 1429,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 1430,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 1431,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 1432,
    /// JPG
    Jpg                                                                    = 1433,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 1434,
    /// IDENTIFIABLE
    Identifiable                                                           = 1435,
    /// SYMMETRIC-KEY
    SymmetricKey                                                           = 1436,
    /// UDP-CHECKSUM-ENABLED
    UdpChecksumEnabled                                                     = 1437,
    /// TX-REF-TRIGGER
    TxRefTrigger                                                           = 1438,
    /// DEF-ITEM
    DefItem                                                                = 1439,
    /// UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT
    UploadableExclusivePackageElement                                      = 1440,
    /// ECU-PARTITION
    EcuPartition                                                           = 1441,
    /// WONT-CALL
    WontCall                                                               = 1442,
    /// FIXED-SIZE
    FixedSize                                                              = 1443,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 1444,
    /// CURVE-AXIS
    CurveAxis                                                              = 1445,
    /// LIMIT-TO-PAGE
    LimitToPage                                                            = 1446,
    /// HW-PIN
    HwPin                                                                  = 1447,
    /// IK
    Ik                                                                     = 1448,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 1449,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 1450,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 1451,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 1452,
    /// CHANNEL-A
    ChannelA                                                               = 1453,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 1454,
    /// NO-SEVERITY
    NoSeverity                                                             = 1455,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 1456,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 1457,
    /// SYSTEM-SUPPLIER-BOOT-RESP-APP
    SystemSupplierBootRespApp                                              = 1458,
    /// IS-LESS-THAN-OR-EQUAL
    IsLessThanOrEqual                                                      = 1459,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 1460,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 1461,
    /// PRIVATE-KEY
    PrivateKey                                                             = 1462,
    /// FINISH
    Finish                                                                 = 1463,
    /// DIAGNOSTIC-DATA-BY-IDENTIFIER
    DiagnosticDataByIdentifier                                             = 1464,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 1465,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 1466,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 1467,
    /// 100BASE-T1
    _100baseT1                                                             = 1468,
    /// EXTERNAL-TRIGGERING-POINT-IDENT
    ExternalTriggeringPointIdent                                           = 1469,
    /// BLUEPRINT-DERIVATION-TIME
    BlueprintDerivationTime                                                = 1470,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 1471,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 1472,
    /// VERTEX-OF-TARGET-CONTAINER
    VertexOfTargetContainer                                                = 1473,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 1474,
    /// USE-FIRST-CONTEXT-DATA
    UseFirstContextData                                                    = 1475,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 1476,
    /// ABSTRACT-RAW-DATA-STREAM-INTERFACE
    AbstractRawDataStreamInterface                                         = 1477,
    /// DEFAULT-ERROR-TRACER
    DefaultErrorTracer                                                     = 1478,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 1479,
    /// UNIT
    Unit                                                                   = 1480,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 1481,
    /// OUT
    Out                                                                    = 1482,
    /// MEMORY-SECTION
    MemorySection                                                          = 1483,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 1484,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 1485,
    /// EXECUTABLE
    Executable                                                             = 1486,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 1487,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 1488,
    /// BE
    Be                                                                     = 1489,
    /// ASYMMETRIC-FROM-BYTE-ARRAY
    AsymmetricFromByteArray                                                = 1490,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 1491,
    /// JA
    Ja                                                                     = 1492,
    /// RESPONSE
    Response                                                               = 1493,
    /// PS
    Ps                                                                     = 1494,
    /// CLIENT-VERIFY
    ClientVerify                                                           = 1495,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 1496,
    /// IS-NOT-RELEVANT
    IsNotRelevant                                                          = 1497,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 1498,
    /// NODE
    Node                                                                   = 1499,
    /// SERVICE-FIELD-DEPLOYMENT
    ServiceFieldDeployment                                                 = 1500,
    /// INVALID
    Invalid                                                                = 1501,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 1502,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED
    AdaptiveServiceStopSubscriptionStarted                                 = 1503,
    /// DATA-FORMAT-ELEMENT-SCOPE
    DataFormatElementScope                                                 = 1504,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 1505,
    /// DIAGNOSTIC-SERVICE-CLASS
    DiagnosticServiceClass                                                 = 1506,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 1507,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 1508,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 1509,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 1510,
    /// SW-CLASS-PROTOTYPE
    SwClassPrototype                                                       = 1511,
    /// OM
    Om                                                                     = 1512,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 1513,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 1514,
    /// ACTIVE
    Active                                                                 = 1515,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 1516,
    /// I-PDU-PORT
    IPduPort                                                               = 1517,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 1518,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 1519,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 1520,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 1521,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 1522,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 1523,
    /// OEM-BOOT-RESP-APP
    OemBootRespApp                                                         = 1524,
    /// BUILD-ACTION
    BuildAction                                                            = 1525,
    /// ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS
    AccessPermissionInstanceOverridesClass                                 = 1526,
    /// PROVIDED-AP-SERVICE-INSTANCE
    ProvidedApServiceInstance                                              = 1527,
    /// SHOW-TYPE
    ShowType                                                               = 1528,
    /// DEVELOPMENT-ERROR-TRACER
    DevelopmentErrorTracer                                                 = 1529,
    /// VARIABLE-SIZE
    VariableSize                                                           = 1530,
    /// LIN-MASTER
    LinMaster                                                              = 1531,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 1532,
    /// START-FROM-BEGINNING
    StartFromBeginning                                                     = 1533,
    /// NO-SLOPPY
    NoSloppy                                                               = 1534,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 1535,
    /// REST-ABSTRACT-ENDPOINT
    RestAbstractEndpoint                                                   = 1536,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 1537,
    /// LINK-LOCAL
    LinkLocal                                                              = 1538,
    /// AUTOSAR-DATA-PROTOTYPE
    AutosarDataPrototype                                                   = 1539,
    /// ERROR-DETECTION
    ErrorDetection                                                         = 1540,
    /// COM-MANAGER
    ComManager                                                             = 1541,
    /// IDS-DESIGN
    IdsDesign                                                              = 1542,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 1543,
    /// DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE
    DiagnosticAbstractRoutineInterface                                     = 1544,
    /// LOGICAL-AND
    LogicalAnd                                                             = 1545,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 1546,
    /// PHYSICAL-ADDRESS
    PhysicalAddress                                                        = 1547,
    /// EXTERNAL-REPLACEMENT
    ExternalReplacement                                                    = 1548,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 1549,
    /// KO
    Ko                                                                     = 1550,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 1551,
    /// TRACE-REFERRABLE
    TraceReferrable                                                        = 1552,
    /// BLOCK-STATE
    BlockState                                                             = 1553,
    /// GD
    Gd                                                                     = 1554,
    /// CLEAR
    Clear                                                                  = 1555,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 1556,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 1557,
    /// SIGN
    Sign                                                                   = 1558,
    /// SERVICE-EVENT-DEPLOYMENT
    ServiceEventDeployment                                                 = 1559,
    /// BSW-MODULE-ENTITY-TERMINATED
    BswModuleEntityTerminated                                              = 1560,
    /// PNG
    Png                                                                    = 1561,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 1562,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 1563,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 1564,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 1565,
    /// DROP
    Drop                                                                   = 1566,
    /// SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE
    SdgAbstractPrimitiveAttribute                                          = 1567,
    /// SECURED-PDU-HEADER-32-BIT
    SecuredPduHeader32Bit                                                  = 1568,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 1569,
    /// PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE
    PhmAbstractRecoveryNotificationInterface                               = 1570,
    /// EVENT-WINDOW-CURRENT-CYCLE
    EventWindowCurrentCycle                                                = 1571,
    /// OPEN
    Open                                                                   = 1572,
    /// DERIVED-FROM
    DerivedFrom                                                            = 1573,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 1574,
    /// VLAN-CONFIG
    VlanConfig                                                             = 1575,
    /// IP-SEC-RULE
    IpSecRule                                                              = 1576,
    /// XDOC
    Xdoc                                                                   = 1577,
    /// UK
    Uk                                                                     = 1578,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 1579,
    /// COUPLING-PORT-STRUCTURAL-ELEMENT
    CouplingPortStructuralElement                                          = 1580,
    /// MONITOR-MODE
    MonitorMode                                                            = 1581,
    /// ROTATE-90-CW-LIMIT-TO-TEXT
    Rotate90CwLimitToText                                                  = 1582,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 1583,
    /// REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE
    ReportMostRecentDtcOnStatusChange                                      = 1584,
    /// SEARCH-FOR-ANY
    SearchForAny                                                           = 1585,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 1586,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 1587,
    /// PRESHARED-KEY-IDENTITY
    PresharedKeyIdentity                                                   = 1588,
    /// PORT-INTERFACE-DEFINITION
    PortInterfaceDefinition                                                = 1589,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 1590,
    /// SECRET-SEED
    SecretSeed                                                             = 1591,
    /// EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE
    EventWindowCurrentAndFollowingCycle                                    = 1592,
    /// TEST-FAILED-BIT
    TestFailedBit                                                          = 1593,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 1594,
    /// TASK
    Task                                                                   = 1595,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 1596,
    /// NOT-VALID
    NotValid                                                               = 1597,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 1598,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 1599,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 1600,
    /// PRESENTATION-CONTINUOUS
    PresentationContinuous                                                 = 1601,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 1602,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 1603,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 1604,
    /// ADAPTIVE-FIELD-GETTER-COMPLETED
    AdaptiveFieldGetterCompleted                                           = 1605,
    /// ABSTRACT-REQUIRED-PORT-PROTOTYPE
    AbstractRequiredPortPrototype                                          = 1606,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 1607,
    /// NO-TRANSFORMER-ERROR-HANDLING
    NoTransformerErrorHandling                                             = 1608,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 1609,
    /// SW-COMPONENT-TYPE
    SwComponentType                                                        = 1610,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 1611,
    /// PRIO-OCC
    PrioOcc                                                                = 1612,
    /// TLS-13
    Tls13                                                                  = 1613,
    /// QUEUED
    Queued                                                                 = 1614,
    /// OBSERVER-BASED
    ObserverBased                                                          = 1615,
    /// OR
    Or                                                                     = 1616,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 1617,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 1618,
    /// LN
    Ln                                                                     = 1619,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 1620,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 1621,
    /// CALLOUT
    Callout                                                                = 1622,
    /// COMMUNICATION-CONTROLLER
    CommunicationController                                                = 1623,
    /// SR
    Sr                                                                     = 1624,
    /// CYCLE-REPETITION-4
    CycleRepetition4                                                       = 1625,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 1626,
    /// SCHEDULED
    Scheduled                                                              = 1627,
    /// NO-PGWIDE
    NoPgwide                                                               = 1628,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 1629,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 1630,
    /// TT
    Tt                                                                     = 1631,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 1632,
    /// PRIMARY-ECU
    PrimaryEcu                                                             = 1633,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 1634,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 1635,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 1636,
    /// FIRST-CONTAINED-TRIGGER
    FirstContainedTrigger                                                  = 1637,
    /// PLATFORM-MODULE-ENDPOINT-CONFIGURATION
    PlatformModuleEndpointConfiguration                                    = 1638,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 1639,
    /// REPETITIVE-EOC
    RepetitiveEoc                                                          = 1640,
    /// MACRO
    Macro                                                                  = 1641,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 1642,
    /// BN
    Bn                                                                     = 1643,
    /// SN
    Sn                                                                     = 1644,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 1645,
    /// COMMUNICATION-CONNECTOR
    CommunicationConnector                                                 = 1646,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 1647,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 1648,
    /// IS-OK
    IsOk                                                                   = 1649,
    /// COMPILER
    Compiler                                                               = 1650,
    /// ATP-PROTOTYPE
    AtpPrototype                                                           = 1651,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 1652,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 1653,
    /// NON-VOLATILE
    NonVolatile                                                            = 1654,
    /// REGULAR
    Regular                                                                = 1655,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 1656,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 1657,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 1658,
    /// DIAGNOSTIC-ABSTRACT-ALIAS-EVENT
    DiagnosticAbstractAliasEvent                                           = 1659,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 1660,
    /// EXECUTABLE-ENTITY
    ExecutableEntity                                                       = 1661,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 1662,
    /// ISO
    Iso                                                                    = 1663,
    /// XG-MII
    XgMii                                                                  = 1664,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 1665,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 1666,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 1667,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 1668,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 1669,
    /// TRIGGER-ACTIVATED
    TriggerActivated                                                       = 1670,
    /// SCHEDULE-VARIANT-3
    ScheduleVariant3                                                       = 1671,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 1672,
    /// UDS
    Uds                                                                    = 1673,
    /// HA
    Ha                                                                     = 1674,
    /// CA
    Ca                                                                     = 1675,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 1676,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 1677,
    /// CAN-CLUSTER
    CanCluster                                                             = 1678,
    /// CAT-1
    Cat1                                                                   = 1679,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 1680,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 1681,
    /// ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS
    AbstractDoIpLogicAddressProps                                          = 1682,
    /// RETURN-ON-EVENT-STOPPED
    ReturnOnEventStopped                                                   = 1683,
    /// REQUIRES-CALLBACK-EXECUTION
    RequiresCallbackExecution                                              = 1684,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 1685,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 1686,
    /// -500-MILES
    _500Miles                                                              = 1687,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 1688,
    /// NO-SHOW-SEE
    NoShowSee                                                              = 1689,
    /// IMMEDIATELY
    Immediately                                                            = 1690,
    /// SCHEDULE-VARIANT-5
    ScheduleVariant5                                                       = 1691,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 1692,
    /// NO-BOOT
    NoBoot                                                                 = 1693,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 1694,
    /// LA
    La                                                                     = 1695,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 1696,
    /// BASE-TYPE
    BaseType                                                               = 1697,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 1698,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 1699,
    /// ENCRYPT-AND-SIGN
    EncryptAndSign                                                         = 1700,
    /// FRAME-QUEUED-FOR-TRANSMISSION
    FrameQueuedForTransmission                                             = 1701,
    /// WAIT-POINT
    WaitPoint                                                              = 1702,
    /// MAPPING-SCOPE-ECU
    MappingScopeEcu                                                        = 1703,
    /// FOR-ALL
    ForAll                                                                 = 1704,
    /// PROCESSING-STYLE-ASYNCHRONOUS
    ProcessingStyleAsynchronous                                            = 1705,
    /// RIGHT
    Right                                                                  = 1706,
    /// APPLICATION-COMPOSITE-DATA-TYPE
    ApplicationCompositeDataType                                           = 1707,
    /// AND
    And                                                                    = 1708,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 1709,
    /// MODE-DECLARATION-REQUESTED
    ModeDeclarationRequested                                               = 1710,
    /// POWER-WINDOW-TIME
    PowerWindowTime                                                        = 1711,
    /// IEEE802-1AS
    Ieee8021as                                                             = 1712,
    /// RX-TRIGGER
    RxTrigger                                                              = 1713,
    /// TRAP
    Trap                                                                   = 1714,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED
    AdaptiveServiceStopSubscriptionCompleted                               = 1715,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 1716,
    /// SG
    Sg                                                                     = 1717,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 1718,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 1719,
    /// CYCLE-REPETITION-50
    CycleRepetition50                                                      = 1720,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 1721,
    /// UPLOADABLE-PACKAGE-ELEMENT
    UploadablePackageElement                                               = 1722,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 1723,
    /// ADAPTIVE-FIELD-GETTER-CALLED
    AdaptiveFieldGetterCalled                                              = 1724,
    /// PERSISTENCY-INTERFACE
    PersistencyInterface                                                   = 1725,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 1726,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 1727,
    /// NO-SUPPORT
    NoSupport                                                              = 1728,
    /// DATA-FORMAT-ELEMENT-REFERENCE
    DataFormatElementReference                                             = 1729,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 1730,
    /// SYSTEM
    System                                                                 = 1731,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 1732,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 1733,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 1734,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 1735,
    /// FJ
    Fj                                                                     = 1736,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 1737,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 1738,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 1739,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 1740,
    /// GENERAL-PARAMETER
    GeneralParameter                                                       = 1741,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 1742,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 1743,
    /// NM-CONFIG
    NmConfig                                                               = 1744,
    /// MULTIPLE-OCCURRENCES
    MultipleOccurrences                                                    = 1745,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 1746,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 1747,
    /// TIMING-EXTENSION
    TimingExtension                                                        = 1748,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 1749,
    /// PC-AFFECTS-PB
    PcAffectsPb                                                            = 1750,
    /// DEFAULT
    Default                                                                = 1751,
    /// DROP-FRAME
    DropFrame                                                              = 1752,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 1753,
    /// PT
    Pt                                                                     = 1754,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 1755,
    /// BRIEF-BYPASSING-FILTERS
    BriefBypassingFilters                                                  = 1756,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 1757,
    /// MIDDLE
    Middle                                                                 = 1758,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 1759,
    /// ARBITRATION
    Arbitration                                                            = 1760,
    /// ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE
    AbstractSynchronizedTimeBaseInterface                                  = 1761,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 1762,
    /// 1000BASE-T
    _1000baseT                                                             = 1763,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 1764,
    /// REQUEST
    Request                                                                = 1765,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 1766,
    /// ITALIC
    Italic                                                                 = 1767,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 1768,
    /// REBOOT
    Reboot                                                                 = 1769,
    /// AR
    Ar                                                                     = 1770,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 1771,
    /// FILE
    File                                                                   = 1772,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 1773,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 1774,
    /// REF-NONE
    RefNone                                                                = 1775,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 1776,
    /// ABSTRACT-SERVICE-INSTANCE
    AbstractServiceInstance                                                = 1777,
    /// FIX_AXIS
    Fixaxis                                                                = 1778,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 1779,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 1780,
    /// DIAGNOSTIC-STOP-ROUTINE
    DiagnosticStopRoutine                                                  = 1781,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 1782,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 1783,
    /// NO-STORE-EVENT
    NoStoreEvent                                                           = 1784,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 1785,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 1786,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 1787,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 1788,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 1789,
    /// WARNING
    Warning                                                                = 1790,
    /// ECUC-ABSTRACT-STRING-PARAM-DEF
    EcucAbstractStringParamDef                                             = 1791,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 1792,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 1793,
    /// RPT-ENABLER-RAM
    RptEnablerRam                                                          = 1794,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 1795,
    /// MIN
    Min                                                                    = 1796,
    /// SERVICE-INTERFACE-DEPLOYMENT
    ServiceInterfaceDeployment                                             = 1797,
    /// BH
    Bh                                                                     = 1798,
    /// INLINE-CONDITIONAL
    InlineConditional                                                      = 1799,
    /// IEEE802-11P
    Ieee80211p                                                             = 1800,
    /// REDUNDANT-PER-KEY
    RedundantPerKey                                                        = 1801,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 1802,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE
    AbstractImplementationDataType                                         = 1803,
    /// SLOPPY
    Sloppy                                                                 = 1804,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 1805,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 1806,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 1807,
    /// PTP--IEEE-1588--2008
    PtpIeee15882008                                                        = 1808,
    /// DOCUMENTATION
    Documentation                                                          = 1809,
    /// TOPIC
    Topic                                                                  = 1810,
    /// REQUEST-NO-RETURN
    RequestNoReturn                                                        = 1811,
    /// FY
    Fy                                                                     = 1812,
    /// INTERRUPT-CAT-1
    InterruptCat1                                                          = 1813,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 1814,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 1815,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 1816,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 1817,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 1818,
    /// NOT-SENT
    NotSent                                                                = 1819,
    /// NOHREF
    Nohref                                                                 = 1820,
    /// ECU-MANAGER
    EcuManager                                                             = 1821,
    /// STATIC-OR-DYNAMIC-PART-TRIGGER
    StaticOrDynamicPartTrigger                                             = 1822,
    /// SECURE-ON-BOARD-COMMUNICATION
    SecureOnBoardCommunication                                             = 1823,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT
    PersistencyDeploymentElement                                           = 1824,
    /// ML
    Ml                                                                     = 1825,
    /// V-2-X-NOT-SUPPORTED
    V2XNotSupported                                                        = 1826,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 1827,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 1828,
    /// SSDP
    Ssdp                                                                   = 1829,
    /// REJECT
    Reject                                                                 = 1830,
    /// TIME-SYNCHRONIZATION-INTERFACE
    TimeSynchronizationInterface                                           = 1831,
    /// CS
    Cs                                                                     = 1832,
    /// SDG-TAILORING
    SdgTailoring                                                           = 1833,
    /// ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF
    EcucAbstractInternalReferenceDef                                       = 1834,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 1835,
    /// TD-EVENT-VFB-PORT
    TdEventVfbPort                                                         = 1836,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 1837,
    /// MULTILANGUAGE-REFERRABLE
    MultilanguageReferrable                                                = 1838,
    /// CAPTURE-SYNCHRONOUSLY-TO-REPORTING
    CaptureSynchronouslyToReporting                                        = 1839,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 1840,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 1841,
    /// DIAGNOSTIC-LOG-AND-TRACE
    DiagnosticLogAndTrace                                                  = 1842,
    /// ADAPTIVE-SERVICE-FIND-COMPLETED
    AdaptiveServiceFindCompleted                                           = 1843,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 1844,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 1845,
    /// SYSTEM-SUPPLIER-BOOT
    SystemSupplierBoot                                                     = 1846,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 1847,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 1848,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 1849,
    /// SK
    Sk                                                                     = 1850,
    /// VAR
    Var                                                                    = 1851,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 1852,
    /// PENDING
    Pending                                                                = 1853,
    /// AUTO-IP
    AutoIp                                                                 = 1854,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 1855,
    /// PLATFORM-HEALTH-MANAGEMENT-INTERFACE
    PlatformHealthManagementInterface                                      = 1856,
    /// SW
    Sw                                                                     = 1857,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 1858,
    /// AP
    Ap                                                                     = 1859,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 1860,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 1861,
    /// ADAPTIVE-MODULE-INSTANTIATION
    AdaptiveModuleInstantiation                                            = 1862,
    /// PC-AFFECTS-LT-AND-PB
    PcAffectsLtAndPb                                                       = 1863,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 1864,
    /// CALLBACK
    Callback                                                               = 1865,
    /// ADAPTIVE-SERVICE-OFFER-STARTED
    AdaptiveServiceOfferStarted                                            = 1866,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 1867,
    /// CODE
    Code                                                                   = 1868,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 1869,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 1870,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 1871,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 1872,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 1873,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 1874,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 1875,
    /// PHM-ACTION-ITEM
    PhmActionItem                                                          = 1876,
    /// ROTATE-180-LIMIT-TO-TEXT
    Rotate180LimitToText                                                   = 1877,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 1878,
    /// REPORT
    Report                                                                 = 1879,
    /// PROCESS-IS-NOT-SELF-TERMINATING
    ProcessIsNotSelfTerminating                                            = 1880,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 1881,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 1882,
    /// RAW-DATA
    RawData                                                                = 1883,
    /// CRC-NOT-VALIDATED
    CrcNotValidated                                                        = 1884,
    /// PSK
    Psk                                                                    = 1885,
    /// CAN-FRAME
    CanFrame                                                               = 1886,
    /// NON-VOLATILE-RAM-MANAGER
    NonVolatileRamManager                                                  = 1887,
    /// LINKER
    Linker                                                                 = 1888,
    /// MIXED
    Mixed                                                                  = 1889,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 1890,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 1891,
    /// FUNCTIONAL-CAN-FD
    FunctionalCanFd                                                        = 1892,
    /// N-PDU
    NPdu                                                                   = 1893,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 1894,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 1895,
    /// COM_AXIS
    Comaxis                                                                = 1896,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 1897,
    /// TP-ADDRESS
    TpAddress                                                              = 1898,
    /// SCHEDULE-VARIANT-4
    ScheduleVariant4                                                       = 1899,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 1900,
    /// TLS-12
    Tls12                                                                  = 1901,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 1902,
    /// FLOAT
    Float                                                                  = 1903,
    /// APPLICATION-ONLY
    ApplicationOnly                                                        = 1904,
    /// DIAG-RESPONSE
    DiagResponse                                                           = 1905,
    /// CYCLE-REPETITION-2
    CycleRepetition2                                                       = 1906,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 1907,
    /// MIXED-29-BIT
    Mixed29Bit                                                             = 1908,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 1909,
    /// ECU
    Ecu                                                                    = 1910,
    /// LOGICAL-OR
    LogicalOr                                                              = 1911,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 1912,
    /// EVENT-STORAGE-DISABLED
    EventStorageDisabled                                                   = 1913,
    /// CALIBRATION-VARIABLES
    CalibrationVariables                                                   = 1914,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 1915,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 1916,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 1917,
    /// RPT-LEVEL-2
    RptLevel2                                                              = 1918,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 1919,
    /// COM-GRANT-DESIGN
    ComGrantDesign                                                         = 1920,
    /// ACTIVATION-AND-TRIGGER-UNICAST
    ActivationAndTriggerUnicast                                            = 1921,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 1922,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 1923,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 1924,
    /// DIAGNOSTIC-SW-MAPPING
    DiagnosticSwMapping                                                    = 1925,
    /// MASKED-NEW-DIFFERS-MASKED-OLD
    MaskedNewDiffersMaskedOld                                              = 1926,
    /// JW
    Jw                                                                     = 1927,
    /// ALL-SUPPORTED-DTCS
    AllSupportedDtcs                                                       = 1928,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 1929,
    /// KS
    Ks                                                                     = 1930,
    /// COLLECTION
    Collection                                                             = 1931,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 1932,
    /// SS
    Ss                                                                     = 1933,
    /// ADAPTIVE-SERVICE-OFFER-COMPLETED
    AdaptiveServiceOfferCompleted                                          = 1934,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 1935,
    /// SAE-J-1939--73
    SaeJ193973                                                             = 1936,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 1937,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 1938,
    /// AMBER-WARNING
    AmberWarning                                                           = 1939,
    /// LV
    Lv                                                                     = 1940,
    /// DIAGNOSTIC-AUTHENTICATION
    DiagnosticAuthentication                                               = 1941,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 1942,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 1943,
    /// COMM-CONNECTOR-PORT
    CommConnectorPort                                                      = 1944,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 1945,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 1946,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 1947,
    /// SHOW-CATEGORY
    ShowCategory                                                           = 1948,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 1949,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 1950,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 1951,
    /// DEVELOPMENT
    Development                                                            = 1952,
    /// REACTION
    Reaction                                                               = 1953,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 1954,
    /// ACK-WITH-RT
    AckWithRt                                                              = 1955,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 1956,
    /// IS-STOPPED
    IsStopped                                                              = 1957,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 1958,
    /// ABSTRACT-PROVIDED-PORT-PROTOTYPE
    AbstractProvidedPortPrototype                                          = 1959,
    /// IS-EXPIRED
    IsExpired                                                              = 1960,
    /// DEFINE-BY-MEMORY-ADDRESS
    DefineByMemoryAddress                                                  = 1961,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 1962,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 1963,
    /// SECURITY-EVENT-CONTEXT-MAPPING
    SecurityEventContextMapping                                            = 1964,
    /// DIAGNOSTIC-START-ROUTINE
    DiagnosticStartRoutine                                                 = 1965,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 1966,
    /// ALL
    All                                                                    = 1967,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 1968,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 1969,
    /// SI
    Si                                                                     = 1970,
    /// HEAD
    Head                                                                   = 1971,
    /// BRIEF
    Brief                                                                  = 1972,
    /// POST-BUILD
    PostBuild                                                              = 1973,
    /// ECUC-ABSTRACT-REFERENCE-DEF
    EcucAbstractReferenceDef                                               = 1974,
    /// CAPTURE-SYNCHRONOUS-TO-REPORTING
    CaptureSynchronousToReporting                                          = 1975,
    /// SCHEDULE-VARIANT-6
    ScheduleVariant6                                                       = 1976,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 1977,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 1978,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 1979,
    /// ENABLED
    Enabled                                                                = 1980,
    /// NAND
    Nand                                                                   = 1981,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 1982,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 1983,
    /// RECORD-VALUE-FIELD
    RecordValueField                                                       = 1984,
    /// SA
    Sa                                                                     = 1985,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 1986,
    /// ORDINARY-EOC
    OrdinaryEoc                                                            = 1987,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 1988,
    /// LIN-SLAVE
    LinSlave                                                               = 1989,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 1990,
    /// FR
    Fr                                                                     = 1991,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 1992,
    /// CONDITIONAL
    Conditional                                                            = 1993,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 1994,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 1995,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 1996,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 1997,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 1998,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 1999,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 2000,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 2001,
    /// PASSIVE
    Passive                                                                = 2002,
    /// NEWLINE
    Newline                                                                = 2003,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 2004,
    /// SDG-DEF
    SdgDef                                                                 = 2005,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 2006,
    /// IE
    Ie                                                                     = 2007,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 2008,
    /// EVENT-WINDOW-INFINITE
    EventWindowInfinite                                                    = 2009,
    /// KN
    Kn                                                                     = 2010,
    /// TRUE
    True                                                                   = 2011,
    /// QU
    Qu                                                                     = 2012,
    /// LAST-FAILED
    LastFailed                                                             = 2013,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 2014,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 2015,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 2016,
    /// NO-FLOAT
    NoFloat                                                                = 2017,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 2018,
    /// PLAIN
    Plain                                                                  = 2019,
    /// SECURITY-EVENT-ONE-EVERY-N-FILTER
    SecurityEventOneEveryNFilter                                           = 2020,
    /// SAE-J-2012--DA
    SaeJ2012Da                                                             = 2021,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 2022,
    /// RUNNABLE-ENTITY-VARIABLE-ACCESS
    RunnableEntityVariableAccess                                           = 2023,
    /// ADAPTIVE-PLATFORM-SERVICE-INSTANCE
    AdaptivePlatformServiceInstance                                        = 2024,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 2025,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 2026,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 2027,
    /// ERROR
    Error                                                                  = 2028,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 2029,
    /// NEW-IS-GREATER-OR-EQUAL
    NewIsGreaterOrEqual                                                    = 2030,
    /// SECURITY
    Security                                                               = 2031,
    /// ABSTRACT-CAN-COMMUNICATION-CONTROLLER
    AbstractCanCommunicationController                                     = 2032,
    /// FRAME-PORT
    FramePort                                                              = 2033,
    /// REF-ALL
    RefAll                                                                 = 2034,
    /// BUS-MIRROR-CHANNEL-MAPPING
    BusMirrorChannelMapping                                                = 2035,
    /// DEFAULT-TRACE-STATE-DISABLED
    DefaultTraceStateDisabled                                              = 2036,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 2037,
    /// DETAILED
    Detailed                                                               = 2038,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 2039,
    /// MC-GROUP
    McGroup                                                                = 2040,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 2041,
    /// IEEE802-1AS-AUTOSAR
    Ieee8021asAutosar                                                      = 2042,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 2043,
    /// DOMAIN-PARTICIPANT-USER-DATA-QOS
    DomainParticipantUserDataQos                                           = 2044,
    /// ABSTRACT-EXECUTION-CONTEXT
    AbstractExecutionContext                                               = 2045,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 2046,
    /// HARDWARE-TEST-MANAGER
    HardwareTestManager                                                    = 2047,
    /// EU
    Eu                                                                     = 2048,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 2049,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 2050,
    /// KM
    Km                                                                     = 2051,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 2052,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 2053,
    /// NO-MONOTONY
    NoMonotony                                                             = 2054,
    /// NO-STATUS-BYTE-CHANGE
    NoStatusByteChange                                                     = 2055,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 2056,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING
    FunctionalClusterInteractsWithFunctionalClusterMapping                 = 2057,
    /// EVENT-ACCEPTANCE-DISABLED
    EventAcceptanceDisabled                                                = 2058,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 2059,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 2060,
    /// WO
    Wo                                                                     = 2061,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 2062,
    /// AUTO-IPDHCPV-4
    AutoIpdhcpv4                                                           = 2063,
    /// REST-ABSTRACT-NUMERICAL-PROPERTY-DEF
    RestAbstractNumericalPropertyDef                                       = 2064,
    /// CRYPTO-JOB
    CryptoJob                                                              = 2065,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 2066,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 2067,
    /// EVENT-STORAGE-ENABLED
    EventStorageEnabled                                                    = 2068,
    /// ERROR-CORRECTION
    ErrorCorrection                                                        = 2069,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 2070,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 2071,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 2072,
    /// SINGLE
    Single                                                                 = 2073,
    /// VARIABLE-DATA-PROTOTYPE-SENT
    VariableDataPrototypeSent                                              = 2074,
    /// SAFETY
    Safety                                                                 = 2075,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 2076,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 2077,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 2078,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 2079,
    /// FULL-DUPLEX-MODE
    FullDuplexMode                                                         = 2080,
    /// AUTOSAR-DATA-TYPE
    AutosarDataType                                                        = 2081,
    /// IMPLEMENTATION
    Implementation                                                         = 2082,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 2083,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 2084,
    /// DIAGNOSTIC-ENV-MODE-ELEMENT
    DiagnosticEnvModeElement                                               = 2085,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 2086,
    /// PARTIAL-NETWORK
    PartialNetwork                                                         = 2087,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 2088,
    /// WRONG-TRIGGER
    WrongTrigger                                                           = 2089,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 2090,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 2091,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 2092,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 2093,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 2094,
    /// NO-SHOW-CONTENT
    NoShowContent                                                          = 2095,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 2096,
    /// FIELD-MAPPING
    FieldMapping                                                           = 2097,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 2098,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 2099,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 2100,
    /// DIAGNOSTIC-REQUEST-ROUTINE-RESULTS
    DiagnosticRequestRoutineResults                                        = 2101,
    /// CRC-OPTIONAL
    CrcOptional                                                            = 2102,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 2103,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 2104,
    /// SOMEIP-FIELD
    SomeipField                                                            = 2105,
    /// TRIGGERED-ON-CHANGE
    TriggeredOnChange                                                      = 2106,
    /// EVENT-ACCEPTANCE-ENABLED
    EventAcceptanceEnabled                                                 = 2107,
    /// PHM-SUPERVISION
    PhmSupervision                                                         = 2108,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 2109,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 2110,
    /// ATP-STRUCTURE-ELEMENT
    AtpStructureElement                                                    = 2111,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 2112,
    /// LEGACY
    Legacy                                                                 = 2113,
    /// NOT-TESTED
    NotTested                                                              = 2114,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 2115,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 2116,
    /// HW-CATEGORY
    HwCategory                                                             = 2117,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 2118,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 2119,
    /// FRAME-ETHERNET-RECEIVED-BY-IF
    FrameEthernetReceivedByIf                                              = 2120,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 2121,
    /// HEAP-USAGE
    HeapUsage                                                              = 2122,
    /// VFB-TIMING
    VfbTiming                                                              = 2123,
    /// NO-SHOW-SHORT-NAME
    NoShowShortName                                                        = 2124,
    /// TX-TRIGGER-MERGED
    TxTriggerMerged                                                        = 2125,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 2126,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 2127,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 2128,
    /// CONFIG-DATA
    ConfigData                                                             = 2129,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 2130,
    /// EVENT-COMBINATION-ON-RETRIEVAL
    EventCombinationOnRetrieval                                            = 2131,
    /// NFOLD
    Nfold                                                                  = 2132,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 2133,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 2134,
    /// ONLY-THIS-CYCLE-AND-READINESS
    OnlyThisCycleAndReadiness                                              = 2135,
    /// IW
    Iw                                                                     = 2136,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 2137,
    /// RAW-DATA-STREAM-GRANT
    RawDataStreamGrant                                                     = 2138,
    /// EVENT-HANDLER
    EventHandler                                                           = 2139,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 2140,
    /// DATA-INTERFACE
    DataInterface                                                          = 2141,
    /// MASTER
    Master                                                                 = 2142,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 2143,
    /// WILL-SEND
    WillSend                                                               = 2144,
    /// DIAGNOSTIC-CAPABILITY-ELEMENT
    DiagnosticCapabilityElement                                            = 2145,
    /// J-1939-REQUEST-MANAGER
    J1939RequestManager                                                    = 2146,
    /// SENT-TAGGED
    SentTagged                                                             = 2147,
    /// OFF
    Off                                                                    = 2148,
    /// PRE-COMPILE-TIME
    PreCompileTime                                                         = 2149,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 2150,
    /// ON-EXIT
    OnExit                                                                 = 2151,
    /// ABSTRACT-ACCESS-POINT
    AbstractAccessPoint                                                    = 2152,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 2153,
    /// CONFIRMED-DTC-BIT
    ConfirmedDtcBit                                                        = 2154,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 2155,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 2156,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 2157,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 2158,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 2159,
    /// SINGLE-OCCURRENCE
    SingleOccurrence                                                       = 2160,
    /// DECREASING
    Decreasing                                                             = 2161,
    /// preserve
    preserve                                                               = 2162,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 2163,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 2164,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 2165,
    /// ADAPTIVE-METHOD-CALL-RECEIVED
    AdaptiveMethodCallReceived                                             = 2166,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 2167,
    /// EXPLICIT
    Explicit                                                               = 2168,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 2169,
    /// TD-EVENT-CYCLE-START
    TdEventCycleStart                                                      = 2170,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 2171,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 2172,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 2173,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 2174,
    /// RES_AXIS
    Resaxis                                                                = 2175,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 2176,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 2177,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 2178,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 2179,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 2180,
    /// SDG-ATTRIBUTE
    SdgAttribute                                                           = 2181,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 2182,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 2183,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 2184,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 2185,
    /// DOES-NOT-USE-LOGGING
    DoesNotUseLogging                                                      = 2186,
    /// PERIODIC-RATE-MEDIUM
    PeriodicRateMedium                                                     = 2187,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 2188,
    /// AUTO-IP--DOIP
    AutoIpDoip                                                             = 2189,
    /// LAST-MODE
    LastMode                                                               = 2190,
    /// TIME-BASE-RESOURCE
    TimeBaseResource                                                       = 2191,
    /// NOT-AVAILABLE
    NotAvailable                                                           = 2192,
    /// UNDECIDED
    Undecided                                                              = 2193,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 2194,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 2195,
    /// INTERRUPT-CAT-2
    InterruptCat2                                                          = 2196,
    /// TARGET-CONTAINER
    TargetContainer                                                        = 2197,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 2198,
    /// RPT-LEVEL-3
    RptLevel3                                                              = 2199,
    /// HW-TYPE
    HwType                                                                 = 2200,
    /// BR
    Br                                                                     = 2201,
    /// LISTEN
    Listen                                                                 = 2202,
    /// MR
    Mr                                                                     = 2203,
    /// TERMINATE
    Terminate                                                              = 2204,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 2205,
    /// BI
    Bi                                                                     = 2206,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 2207,
    /// ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING
    AbstractSignalBasedToISignalTriggeringMapping                          = 2208,
    /// REPORTS-EXECUTION-STATE
    ReportsExecutionState                                                  = 2209,
    /// GIF
    Gif                                                                    = 2210,
    /// EPS
    Eps                                                                    = 2211,
    /// TRACEABLE
    Traceable                                                              = 2212,
    /// BREAK
    Break                                                                  = 2213,
    /// ST
    St                                                                     = 2214,
    /// NUMBER
    Number                                                                 = 2215,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 2216,
    /// REST-PRIMITIVE-PROPERTY-DEF
    RestPrimitivePropertyDef                                               = 2217,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 2218,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 2219,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 2220,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 2221,
    /// FILTERED
    Filtered                                                               = 2222,
    /// RESTART
    Restart                                                                = 2223,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 2224,
    /// ACTIVATION-UNICAST
    ActivationUnicast                                                      = 2225,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 2226,
    /// SOFTWARE-ACTIVATION-DEPENDENCY
    SoftwareActivationDependency                                           = 2227,
    /// SEARCH-FOR-ALL-INSTANCES
    SearchForAllInstances                                                  = 2228,
    /// VENDOR-SPECIFIC
    VendorSpecific                                                         = 2229,
    /// DONT-INVALIDATE
    DontInvalidate                                                         = 2230,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 2231,
    /// REMOVE
    Remove                                                                 = 2232,
    /// FI
    Fi                                                                     = 2233,
    /// RULE
    Rule                                                                   = 2234,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 2235,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING
    PersistencyPortPrototypeToDeploymentMapping                            = 2236,
    /// SH
    Sh                                                                     = 2237,
    /// ADAPTIVE-EVENT-RECEIVED
    AdaptiveEventReceived                                                  = 2238,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 2239,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 2240,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 2241,
    /// DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS
    DiagnosticMemoryAddressableRangeAccess                                 = 2242,
    /// EVENT-MAPPING
    EventMapping                                                           = 2243,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 2244,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 2245,
    /// NO-HEADER
    NoHeader                                                               = 2246,
    /// BOLD
    Bold                                                                   = 2247,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 2248,
    /// CLASSIC
    Classic                                                                = 2249,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 2250,
    /// AY
    Ay                                                                     = 2251,
    /// ACL-PERMISSION
    AclPermission                                                          = 2252,
    /// ABSTRACT-CAN-COMMUNICATION-CONNECTOR
    AbstractCanCommunicationConnector                                      = 2253,
    /// SYNCHRONOUS
    Synchronous                                                            = 2254,
    /// BIDIRECTIONAL
    Bidirectional                                                          = 2255,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 2256,
    /// CAN-TP-NODE
    CanTpNode                                                              = 2257,
    /// STRICT-PRIORITY
    StrictPriority                                                         = 2258,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 2259,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 2260,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 2261,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER
    DiagnosticCommunicationManager                                         = 2262,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 2263,
    /// NV-RAM-MANAGER
    NvRamManager                                                           = 2264,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 2265,
    /// CALCULATED
    Calculated                                                             = 2266,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 2267,
    /// 100BASE-TX
    _100baseTx                                                             = 2268,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 2269,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 2270,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 2271,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 2272,
    /// WATCH-TRIGGER-GAP
    WatchTriggerGap                                                        = 2273,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 2274,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 2275,
    /// DIAGNOSTIC-TROUBLE-CODE
    DiagnosticTroubleCode                                                  = 2276,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 2277,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 2278,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 2279,
    /// COMMAND-LINE-SIMPLE-FORM
    CommandLineSimpleForm                                                  = 2280,
    /// ON-DTC-STATUS-CHANGE
    OnDtcStatusChange                                                      = 2281,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 2282,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 2283,
    /// BLINK-MODE
    BlinkMode                                                              = 2284,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 2285,
    /// DIAG-REQUEST
    DiagRequest                                                            = 2286,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 2287,
    /// NETWORK-REPRESENTATION-FROM-COM-SPEC
    NetworkRepresentationFromComSpec                                       = 2288,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 2289,
    /// SDG-CLASS
    SdgClass                                                               = 2290,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 2291,
    /// IDS-COMMON-ELEMENT
    IdsCommonElement                                                       = 2292,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 2293,
    /// ASYMMETRIC-TO-BYTE-ARRAY
    AsymmetricToByteArray                                                  = 2294,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 2295,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 2296,
    /// NO-SHOW-ALIAS-NAME
    NoShowAliasName                                                        = 2297,
    /// RED-STOP-LAMP
    RedStopLamp                                                            = 2298,
    /// REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE
    ReplaceByTimeoutSubstitutionValue                                      = 2299,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 2300,
    /// MULTICORE-REENTRANT
    MulticoreReentrant                                                     = 2301,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 2302,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE
    DiagnosticAbstractDataIdentifierInterface                              = 2303,
    /// MY
    My                                                                     = 2304,
    /// RW
    Rw                                                                     = 2305,
    /// I-SIGNAL-SENT-TO-COM
    ISignalSentToCom                                                       = 2306,
    /// X-MMI
    XMmi                                                                   = 2307,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 2308,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 2309,
    /// SIGNAL-BASED
    SignalBased                                                            = 2310,
    /// ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT
    AddrMethodShortNameAndAlignment                                        = 2311,
    /// INIT-EVENT
    InitEvent                                                              = 2312,
    /// UDP-NM
    UdpNm                                                                  = 2313,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 2314,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 2315,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 2316,
    /// SDG-REFERENCE
    SdgReference                                                           = 2317,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 2318,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 2319,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 2320,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 2321,
    /// FRAME-TRANSMITTED-ON-BUS
    FrameTransmittedOnBus                                                  = 2322,
    /// VI
    Vi                                                                     = 2323,
    /// INFINITE-TIME-TO-RESPONSE
    InfiniteTimeToResponse                                                 = 2324,
    /// BSW-SCHEDULE-EVENT
    BswScheduleEvent                                                       = 2325,
    /// ROTATE-90-CW-FIT-TO-TEXT
    Rotate90CwFitToText                                                    = 2326,
    /// V-2-X-ACTIVE-SUPPORTED
    V2XActiveSupported                                                     = 2327,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 2328,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 2329,
    /// WAIT-TIME-DATE
    WaitTimeDate                                                           = 2330,
    /// CYCLE-REPETITION-10
    CycleRepetition10                                                      = 2331,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 2332,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 2333,
    /// INSTRUCTION
    Instruction                                                            = 2334,
    /// LOCAL
    Local                                                                  = 2335,
    /// X-509
    X509                                                                   = 2336,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 2337,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY
    PersistencyRedundancyHandlingScopeKey                                  = 2338,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 2339,
    /// MODELED
    Modeled                                                                = 2340,
    /// TL
    Tl                                                                     = 2341,
    /// ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE
    RootSwClusterDesignComponentPrototype                                  = 2342,
    /// COLDSTART
    Coldstart                                                              = 2343,
    /// V-2-X-FACILITIES
    V2XFacilities                                                          = 2344,
    /// TRIGGERED-ON-EVALUATION
    TriggeredOnEvaluation                                                  = 2345,
    /// STRICTLY-INCREASING
    StrictlyIncreasing                                                     = 2346,
    /// NOT
    Not                                                                    = 2347,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 2348,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT
    PersistencyRedundancyHandlingScopeElement                              = 2349,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 2350,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 2351,
    /// PER-EXECUTABLE
    PerExecutable                                                          = 2352,
    /// DEFAULT-TRIGGER
    DefaultTrigger                                                         = 2353,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 2354,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 2355,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 2356,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 2357,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 2358,
    /// TP-CONNECTION-IDENT
    TpConnectionIdent                                                      = 2359,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 2360,
    /// BSW-MODULE-CALL-POINT
    BswModuleCallPoint                                                     = 2361,
    /// KEEP
    Keep                                                                   = 2362,
    /// PROCESSING-STYLE-SYNCHRONOUS
    ProcessingStyleSynchronous                                             = 2363,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 2364,
    /// NO
    No                                                                     = 2365,
    /// COUPLING-PORT
    CouplingPort                                                           = 2366,
    /// LINK-TIME
    LinkTime                                                               = 2367,
    /// MONO
    Mono                                                                   = 2368,
    /// IDS-PLATFORM-INSTANTIATION
    IdsPlatformInstantiation                                               = 2369,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 2370,
    /// NET
    Net                                                                    = 2371,
    /// COMMAND-LINE-LONG-FORM
    CommandLineLongForm                                                    = 2372,
    /// IN
    In                                                                     = 2373,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 2374,
    /// CANCEL
    Cancel                                                                 = 2375,
    /// RUNNABLE-ENTITY-TERMINATED
    RunnableEntityTerminated                                               = 2376,
    /// PROTECTED
    Protected                                                              = 2377,
    /// IS-GREATER-THAN-OR-EQUAL
    IsGreaterThanOrEqual                                                   = 2378,
    /// CONSOLE
    Console                                                                = 2379,
    /// MAX
    Max                                                                    = 2380,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 2381,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 2382,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 2383,
    /// ROTATE-90-CCW
    Rotate90Ccw                                                            = 2384,
    /// CALIBRATION-OFFLINE
    CalibrationOffline                                                     = 2385,
    /// OBD-DRIVING-CYCLE
    ObdDrivingCycle                                                        = 2386,
    /// RTPGE
    Rtpge                                                                  = 2387,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 2388,
    /// I-PDU-SENT-TO-IF
    IPduSentToIf                                                           = 2389,
    /// EXTENDED
    Extended                                                               = 2390,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 2391,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 2392,
    /// RO
    Ro                                                                     = 2393,
    /// DHCPV-6
    Dhcpv6                                                                 = 2394,
    /// ON-ENTRY
    OnEntry                                                                = 2395,
    /// MEDIUM
    Medium                                                                 = 2396,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 2397,
}

impl EnumItem {
    const STRING_TABLE: [&'static str; 2398] = ["MODE-DECLARATION-SWITCH-INITIATED", "SWC-MODE-SWITCH-EVENT", "CAPTURE-ASYNCHRONOUS-TO-REPORTING", "SUPERVISION-MODE", "TRANSFORMATION-TECHNOLOGY", "DO-IP-ACTIVATION-LINE-NEEDS", "STATIC-SOCKET-CONNECTION", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE", "I-SIGNAL-I-PDU-GROUP", "BOTTOM", "TD-EVENT-SERVICE-INSTANCE-EVENT", "VEHICLE-PACKAGE", "VARIATION-POINT-PROXY", "DIAGNOSTIC-AGING", "VO", "MINIMUM-MINOR-VERSION", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "GU", "DIAGNOSTIC-TEST-RESULT", "PRE--R-4--2", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "FORGET", "USER-DEFINED-PDU", "ON-CHANGE-OF-DATA-IDENTIFIER", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "PL", "SWC-TO-IMPL-MAPPING", "POLY", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED", "ZH", "CRYPTO-SERVICE-NEEDS", "J-1939", "NO-SHOW-TYPE", "SPEC-ELEMENT-REFERENCE", "CURVE_AXIS", "GRANT", "DIAGNOSTIC-EVENT-NEEDS", "BSW-MODULE-ENTITY-ACTIVATED", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "DIAGNOSTIC-IUMPR-GROUP", "MS", "PHYSICAL-DIMENSION", "PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR", "DIAGNOSTIC-COMMON-ELEMENT", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "MANUFACTURING", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "PASS-THROUGH-SW-CONNECTOR", "SENT-UNTAGGED", "V-2-X-M-USER-NEEDS", "BSW-M-ENTRY-CALLED", "CHAPTER", "PORT-BLUEPRINT", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "DEADLINE-SUPERVISION", "SW-INSTANCE", "SERVER-DECRYPT", "ERROR-TRACER", "CYCLIC-AND-ON-CHANGE", "FATAL", "IDSM-TRAFFIC-LIMITATION", "TEST-FAILED-THIS-OPERATION-CYCLE", "PHYSICAL", "DATA-RECEIVED-EVENT", "AP-SOMEIP-TRANSFORMATION-PROPS", "TIME", "DIAGNOSTIC-MEMORY-DESTINATION", "RESET-MACHINE", "NON-REENTRANT", "PERSISTENCY-KEY-VALUE-DATABASE", "SW-GENERIC-AXIS-PARAM-TYPE", "CLIENT-ENCRYPT", "BINARY-MANIFEST-RESOURCE", "MOST-SIGNIFICANT-BYTE-LAST", "UPDATE", "RAW-DATA-STREAM-GRANT-DESIGN", "DATA-PROTOTYPE", "RAW-DATA-STREAM-SERVER-INTERFACE", "ETH-TP-CONFIG", "ACCESS-PERMISSION-SERVICE-CLASS", "UDP-NM-CLUSTER", "SU", "LIN-FRAME", "ADAPTIVE-FIELD-SETTER-COMPLETED", "TD-EVENT-COM", "IS-RELEVANT", "ATP-BLUEPRINTABLE", "KEEP-EXISTING", "BMP", "DLT-USER-NEEDS", "SW-CALIBRATION-METHOD", "UCM", "SOMEIP-SERVICE-INTERFACE", "NM-CLUSTER", "STIMULUS-SYNCHRONIZATION", "DIAGNOSTIC-CONDITION-INTERFACE", "DO-IP-GID-NEEDS", "LAST-IS-BEST", "SWC-INTERNAL-BEHAVIOR", "END-2-END-EVENT-PROTECTION-PROPS", "TD-EVENT-BSW", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "CAN-20", "TCP", "ETHERNET-COMMUNICATION-CONTROLLER", "MONOTONOUS", "SERIALIZER", "STATUS-BIT-NORMAL", "TEST-FAILED", "ACTION", "ALL-INDICES-SAME-ARRAY-SIZE", "VIEW-MAP", "TD-EVENT-VFB", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "READ-ONLY", "SIGNATURE", "VARIANT-POST-BUILD-LOADABLE", "BG", "SDG-ABSTRACT-FOREIGN-REFERENCE", "USER-DEFINED-CLUSTER", "SW-CONNECTOR", "USES-LOGGING", "PR-PORT-PROTOTYPE", "ACTION-LIST", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "DIAGNOSTIC-ROUTINE-INTERFACE", "WONT-RECEIVE", "HW-DESCRIPTION-ENTITY", "CONSTANT-SPECIFICATION", "RSA", "COMMUNICATION-INTER-ECU", "NEW-IS-EQUAL", "LT", "MODE-SWITCH-POINT", "ROTATE-90-CW", "ECUC-MODULE-CONFIGURATION-VALUES", "PERSISTENCY-FILE-ELEMENT", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "TD-EVENT-SERVICE-INSTANCE", "HIERARCHICAL-EOC", "TCP-OPTION-FILTER-SET", "BSW-SERVICE-DEPENDENCY-IDENT", "ABSTRACT-EVENT", "PLATFORM-ACTION-ITEM", "XFILE", "PC-AFFECTS-LT", "PROCESS-IS-SELF-TERMINATING", "KU", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING", "TRIGGERED-WITHOUT-REPETITION", "DHCPV-4", "BUILD-TYPE-DEBUG", "ACL-OPERATION", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "EMISSION-RELATED-DTC", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "SECTION-NAME-PREFIX", "SYNCHRONIZED", "STATUS-BIT-AGING-AND-DISPLACEMENT", "DEFAULT-IF-REVISION-UPDATE", "ECU-INSTANCE", "PARTITION", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "REQUIRED-DDS-SERVICE-INSTANCE", "OBD-INFO-SERVICE-NEEDS", "CYCLE-REPETITION-8", "ADAPTIVE-EVENT-SENT", "NON-EMMISSION-RELATED-DTC", "FORWARD-AS-IS", "DIAGNOSTIC-MAPPING", "TTCAN-PHYSICAL-CHANNEL", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "SW-CLASS-INSTANCE", "TD-EVENT-FRAME", "RECT", "IMMEDIATE", "RUN-CONTINUOUS", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "RPT-ENABLER-RAM-AND-ROM", "TUNNEL", "X-MII", "VERBOSE", "CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "ROTATE-180", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "APPLICATION-INTERFACE", "BINARY-MANIFEST-RESOURCE-DEFINITION", "WATCH-TRIGGER", "LIMIT-TO-TEXT", "STACK-USAGE", "FRAME-RECEIVED-BY-IF", "INFO", "CP-SOFTWARE-CLUSTER-RESOURCE", "DLT-CONTEXT", "SLOW-FLASHING-MODE", "MN", "I-PDU-TRIGGERING", "SYNCHRONIZATION-POINT-CONSTRAINT", "J-1939-SHARED-ADDRESS-CLUSTER", "CONTINUE-AT-IT-POSITION", "AGREED", "MEASURED-HEAP-USAGE", "SECURITY-EVENT-AGGREGATION-FILTER", "REST-SERVICE-INTERFACE", "SHOW-LONG-NAME", "CONFIRMED", "AUTOSAR-VARIABLE-INSTANCE", "RAPID-PROTOTYPING-SCENARIO", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "NEW-IS-WITHIN", "TTCAN-COMMUNICATION-CONTROLLER", "ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED", "PRODUCER", "KK", "CONCRETE", "ACCES-PERRMISSION-SERVICE-CLASS", "CRC-NOT-SUPPORTED", "SWC-TO-ECU-MAPPING", "SW-MC-FRAME", "V-2-X-MANAGEMENT", "MODE-DECLARATION-GROUP-PROTOTYPE", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "DLNA", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "PERSISTENCY-FILE", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "TIMING-DESCRIPTION-EVENT", "OVERRIDE", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "IDSM-RATE-LIMITATION", "FIBEX-ELEMENT", "ECUC-BOOLEAN-PARAM-DEF", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "MODE-DECLARATION-SWITCH-COMPLETED", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "REPORT-BEFORE-INIT", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "ON-COMPARISON-OF-VALUES", "DIAGNOSTIC-CONDITION", "SERVICE-INSTANCE-COLLECTION-SET", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "PHM-ACTION", "SOMEIP-TP-CHANNEL", "RESPOND-BEFORE-RESET", "TOPIC-PREFIX", "OS-TASK-EXECUTION-EVENT", "EO", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "CYCLIC", "TLS-CRYPTO-SERVICE-MAPPING", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "EVAP", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "CRYPTO-KEY-MANAGEMENT", "CYCLE-REPETITION-5", "NEW-IS-LESS", "SEC-OC-JOB-MAPPING", "TS", "SYNC-TIME-BASE-MGR-USER-NEEDS", "REDUNDANT-PER-ELEMENT", "TTCAN-CLUSTER", "IDS-MGR-NEEDS", "DLT-MESSAGE", "SEARCH-FOR-ALL", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "TD-EVENT-BSW-MODULE", "PROTECT-LAMP", "I-SIGNAL-I-PDU", "SOFTWARE-CLUSTER-REQUIREMENT", "EXECUTE", "BSW-DATA-RECEIVED-EVENT", "FLEXRAY-COMMUNICATION-CONNECTOR", "KL", "RESET-MCU", "API", "TIMING-EVENT", "MACHINE", "ERROR-TRACER-NEEDS", "BUILD-ACTION-ENVIRONMENT", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "DATA-CONSTR", "PGWIDE", "LEFT", "SYMMETRIC", "SHORT-HEADER", "TIMING-CONDITION", "J-1939-DCM-I-PDU", "TRACE", "LIN-CLUSTER", "ABSTRACT-CLASS-TAILORING", "ALIVE-SUPERVISION", "SERVER-ENCRYPT", "I-PDU-RECEIVED-BY-COM", "PHM-SUPERVISED-ENTITY-INTERFACE", "FM-FEATURE-RESTRICTION", "AA", "I-SIGNAL-PORT", "DIAGNOSTIC-CONDITION-GROUP", "SWITCH", "NO-ACK", "PARAMETER-SW-COMPONENT-TYPE", "CAN-TP-ADDRESS", "SOMEIP-METHOD", "AZ", "SERVER-MAC-GENERATE", "COLLECTABLE-ELEMENT", "10BASE-T1S", "NE", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "PACKAGEABLE-ELEMENT", "PORT", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "FIT-TO-PAGE", "FIT-TO-TEXT", "NO-SUPERVISION", "DLT-LOG-CHANNEL", "MODE-DECLARATION-GROUP", "TIP", "ON-TRANSITION", "REST-ENDPOINT-POST", "LOGICAL-EXPRESSION", "TD-EVENT-TT-CAN-CYCLE-START", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "WILL-CALL", "API-USE", "PORT-INTERFACE-MAPPING", "TRACED-FAILURE", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "SECURITY-EVENT-THRESHOLD-FILTER", "BA", "NO-OBD-SUPPORT", "DIAGNOSTIC-FIM-EVENT-GROUP", "PORT-GROUP", "MULTIPLE", "SIDES", "MODE-DECLARATION-MAPPING", "DIAGNOSTIC-MEMORY-BY-ADDRESS", "NO-TRANSFORMER-STATUS-FORWARDING", "SO-CON-I-PDU-IDENTIFIER", "HY", "TLS-CRYPTO-CIPHER-SUITE", "ARRAY", "SHOW-SHORT-NAME", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "UNDEFINED", "POST-BUILD-VARIANT-CRITERION", "JUSTIFY", "KA", "DIAGNOSTIC-INDICATOR", "DDS-EVENT-DEPLOYMENT", "IS-NOT-EQUAL", "VARIANT-PRE-COMPILE", "AFTERMARKET", "VALID", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "1000BASE-T1", "PUT", "USE-VOID", "DLT-APPLICATION", "COM-MGR-USER-NEEDS", "APPLICATION-ASSOC-MAP-ELEMENT", "SERVER-MAC-VERIFY", "COM-EVENT-GRANT-DESIGN", "VARIANT-POST-BUILD-SELECTABLE", "END-TO-END-PROTECTION-SET", "STRICT-MONOTONOUS", "POWER", "MALFUNCTION", "DIAGNOSTIC-ROUTINE", "WATCH-DOG-MANAGER", "STRUCTURED-REQ", "BSW-MODULE-ENTRY", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "SECONDARY-ECU", "SYSTEM-DESIGN-TIME", "DIAGNOSTIC-SECURITY-ACCESS", "LOWER-8-BIT", "DATA-TRANSFORMATION-SET", "KEY-STORAGE", "DEPENDENCY-ON-ARTIFACT", "ALIAS-NAME-SET", "ASYNCHRONOUS", "ROTATE-90-CCW-LIMIT-TO-TEXT", "EOC-EXECUTABLE-ENTITY-REF", "WONT-SEND", "GL", "RN", "PUBLIC-KEY", "SYNC-BASE-TIME-MANAGER", "MASTER-ECU", "DIAGNOSTIC-CONTROL-DTC-SETTING", "ANY-SEND-OPERATION", "COMMON", "DELEGATION-SW-CONNECTOR", "PAYLOAD-AS-ARRAY", "LIN-PHYSICAL-CHANNEL", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "CAUTION", "APPLICATION-RECORD-ELEMENT", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "OFFSET", "SATURATE", "REFERRABLE", "ANY-STANDARDIZED", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "LAND", "XCP-PDU", "NM-INSTANTIATION", "MO", "LINK", "BSW-MODULE-ENTITY", "SECOND-TO-FIRST", "DSA", "TRANSFORMER-ERROR-HANDLING", "STATIC-PART-TRIGGER", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "COM-FIND-SERVICE-GRANT-DESIGN", "ISO-15031--6", "CRYPTO-KEY-SLOT", "FLEXRAY-COMMUNICATION-CONTROLLER", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "C", "DIAGNOSTIC-TRANSFER-EXIT", "USER-DEFINED-FIELD-DEPLOYMENT", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED", "LIN-UNCONDITIONAL-FRAME", "FM-FEATURE", "KEY-DERIVATION", "RPT-SERVICE-POINT", "SCHEDULE-VARIANT-1", "PROVIDED-SERVICE-INSTANCE", "SERVICE-INTERFACE-ELEMENT-MAPPING", "NOT-ACCESSIBLE", "AVB--IEEE-802--1-AS", "ECUC-PARAMETER-DEF", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "OVERWRITE", "PRECONFIGURED-CONFIGURATION", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "JI", "SILENT", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "TC", "USE-ARGUMENT-TYPE", "HEALTH-CHANNEL-EXTERNAL-STATUS", "PORT-PROTOTYPE-BLUEPRINT", "CAN-TP-CHANNEL", "TRACEABLE-TABLE", "APPLICATION", "SWC-BSW-MAPPING", "ADAPTIVE-FIELD-NOTIFICATION-SENT", "BSW-DIRECT-CALL-POINT", "DIAGNOSTIC-J-1939-NODE", "MAPPING-SCOPE-PARTITION", "SOFTWARE-CLUSTER-DESIGN", "REFERENCE-TAILORING", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "RPT-COMPONENT", "PHM-HEALTH-CHANNEL-STATUS", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "AF", "DIAGNOSTIC-RESPONSE-ON-EVENT", "ACL-ROLE", "BSW", "ABSTRACT-SECURITY-EVENT-FILTER", "RESPOND-AFTER-RESET", "ACCEPT-CONFIGURED", "SDG-AGGREGATION-WITH-VARIATION", "DROP-UNTAGGED", "I-SIGNAL", "TLS-CONNECTION-GROUP", "ENHANCED", "CUSTOM", "ETHERNET-FRAME", "INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE", "BSW-M-ENTRY-CALL-RETURNED", "SERVER-CALL-POINT", "TI", "TG", "AP-APPLICATION-ERROR-DOMAIN", "TLS-DEPLOYMENT", "E-2-E-PROFILE-CONFIGURATION-SET", "ECUC-DESTINATION-URI-DEF-SET", "BINARY-MANIFEST-META-DATA-FIELD", "J-1939-TP-NODE", "IDS-MAPPING", "BLOCK-SOURCE", "COM-OFFER-SERVICE-GRANT-DESIGN", "ROUTER-ADVERTISEMENT", "FDC-THRESHOLD", "SECURITY-EVENT-CONTEXT-PROPS", "FRAME-ETHERNET-SENT-ON-BUS", "APPLICATION-ENDPOINT", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "COM-METHOD-GRANT", "DIAGNOSTIC-COMPONENT-NEEDS", "TRIGGER-RELEASED", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "OS-TASK-PROXY", "USER-DEFINED-ETHERNET-FRAME", "SW-COMPONENT-MAPPING-CONSTRAINTS", "RES-AXIS", "CAN-NM-CLUSTER", "PDUR-I-PDU-GROUP", "SUPPORTS-BUFFER-LOCKING", "CLIENT-ID-DEFINITION-SET", "MAINTENANCE-ONLY", "CRYPTO-PROVIDER", "I-4-G", "SOMEIP-PROVIDED-EVENT-GROUP", "BSW-MODULE-ENTITY-STARTED", "API-BASED", "SWC-TIMING", "GLOBAL-TIME-SLAVE", "DETERMINISTIC-CLIENT", "MASEKD-NEW-EQUALS-X", "ECUC-CONTAINER-DEF", "SWC", "GA", "NL", "CRYPTO-SERVICE-MANAGER", "EXERCISE", "SWC-IMPLEMENTATION", "UDP-NM-NODE", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "TD-EVENT-FR-CLUSTER-CYCLE-START", "CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET", "FIRST-TO-SECOND", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "AFTERMAKET", "J-1939-CONTROLLER-APPLICATION", "BUILD-ACTION-MANIFEST", "SYSTEM-SIGNAL", "DEFERRED", "FLEXRAY-FRAME", "IAM-MODULE-INSTANTIATION", "CAN-TP-CONFIG", "TTCAN-COMMUNICATION-CONNECTOR", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "TIFF", "DOCUMENTATION-CONTEXT", "DDS-DOMAIN-RANGE", "ECUC-ADD-INFO-PARAM-DEF", "TOPBOT", "STEADY", "RUN-ONCE", "FLEXRAY-TP-CONFIG", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "TRIGGERED", "RESPONSE-SYNCHRONIZATION", "ROLL-BACK", "PRIMITIVE", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "REQUIRED-AP-SERVICE-INSTANCE", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "FIX-AXIS", "REPLACE", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "POSSIBLE-ERROR-REACTION", "UP-LINK-PORT", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "DATA-EXCHANGE-POINT", "HOST-PORT", "BINARY-MANIFEST-REQUIRE-RESOURCE", "R-4--2", "DA", "CP-SOFTWARE-CLUSTER", "NOTIFICATION", "DO-IP-SERVICE-NEEDS", "SERVICE-INTERFACE-FIELD-MAPPING", "DIAGNOSTIC-CONTRIBUTION-SET", "CRYPTO-SERVICE-MAPPING", "CRYPTO-KEY-MANAGEMENT-NEEDS", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "ADAPTIVE-METHOD-RESPONSE-RECEIVED", "SECURE-COMMUNICATION-DEPLOYMENT", "SERVICE-DISCOVERY", "GLOBAL-TIME-FR-SLAVE", "ENUMERATION-MAPPING-TABLE", "GLOBAL-TIME-CAN-MASTER", "PROCESS-PHM-ACTION-ITEM", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "ECUC-COMMON-ATTRIBUTES", "EOC-EXECUTABLE-ENTITY-REF-ABSTRACT", "ATP-BLUEPRINT", "CAPTION", "OPERATION-CALLED", "ACCEPT-ALL", "TRANSLATION-START", "HINT", "TD-EVENT-MODE-DECLARATION", "PER-INSTANCE-MEMORY", "SW-SERVICE-PROTOTYPE", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "DEFAULT-TRACE-STATE-ENABLED", "DIAGNOSTIC-AUTH-ROLE", "TIME-SYNC-SERVER-CONFIGURATION", "SHOW-PAGE", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "DDS-FIELD-DEPLOYMENT", "TD-EVENT-SERVICE-INSTANCE-FIELD", "RPT-CONTAINER", "SW-SERVICE-ARG", "GLOBAL-TIME-ETH-MASTER", "AR--CLIENT--SERVER", "ATOMIC-SW-COMPONENT-TYPE", "SW-AXIS-TYPE", "J-1939-DCM", "BSW-SCHEDULABLE-ENTITY", "SECURITY-EVENT-DEFINITION", "TOP", "KEYWORD-SET", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "CLIENT-ID-DEFINITION", "INTERNAL-TRIGGER-OCCURRED-EVENT", "NORMALFIXED", "CLIENT-MAC-VERIFY", "SO-AD-ROUTING-GROUP", "BSW-MODULE-TIMING", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "SECURED-PDU-HEADER-08-BIT", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "EN", "DATA-SEND-COMPLETED-EVENT", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "LIFE-CYCLE-STATE", "NEWLINE-IF-NECESSARY", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "COM-TRIGGER-GRANT-DESIGN", "CYCLE-REPETITION-16", "IPSEC", "RECOMMENDED-CONFIGURATION", "SESSION-HANDLING-ACTIVE", "GETTER-SETTER", "BSW-INTERRUPT-ENTITY", "STORE-EVENT", "STRICT-MODE", "CIRCLE", "IGNORE", "DISABLE", "SERVICE-ONLY", "METHOD-MAPPING", "DATA-TRANSFORMATION", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "AGE", "CRYPTO-SERVICE-KEY", "SW-SYSTEMCONSTANT-VALUE-SET", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "DIAGNOSTIC-SERVICE-INSTANCE", "SW-MC-INTERFACE", "UNNUMBER", "ROOT-SW-COMPOSITION-PROTOTYPE", "SERVICE-INTERFACE-TRIGGER-MAPPING", "ONE-EVERY-N", "FA", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "I-SIGNAL-AVAILABLE-FOR-RTE", "PROCESS-TO-MACHINE-MAPPING", "FUNCTION-GROUP-SET", "ECU-TIMING", "OPERATION-CALL-RESPONSE-SENT", "TD-EVENT-COMPLEX", "AH", "EDGE-NODE", "ADAPTIVE-METHOD-CALLED", "ATP-TYPE", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "OC", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "START", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "SHOW-ALIAS-NAME", "REDUNDANT", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "ATTRIBUTE-TAILORING", "BINARY-MANIFEST-ADDRESSABLE-OBJECT", "STOP", "ECUC-MODULE-DEF", "PASSTHROUGH", "RAW-DATA-STREAM-MAPPING", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "MAPPING-SCOPE-CORE", "CRC-SUPPORTED", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "FIXED", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER", "BSW-IMPLEMENTATION", "DIAGNOSTIC-ENABLE-CONDITION", "APPLICATION-ARRAY-ELEMENT", "SIGNAL-SERVICE-TRANSLATION-PROPS", "ALWAYS", "ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT", "ETHERNET-PRIORITY-REGENERATION", "SIGN-WITH-ORIGIN-AUTHENTICATION", "LOWER-12-BIT", "default", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "DIAGNOSTIC-ROUTINE-SUBFUNCTION", "CONCRETE-CLASS-TAILORING", "UZ", "FLEXRAY-TP-PDU-POOL", "CLEAR-ALL-DTCS", "DEPENDANT", "ADAPTIVE-FIELD-NOTIFICATION-RECEIVED", "NOTHING", "CY", "CALPRM", "COUPLING-ELEMENT", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "LT-AFFECTS-PB", "CRYPTO-ELLIPTIC-CURVE-PROPS", "TE", "SCHEDULE-VARIANT-2", "GATEWAY", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "PHM-HEALTH-CHANNEL-INTERFACE", "SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "EVENT-TRIGGERING-CONSTRAINT", "DIAGNOSTIC-OPERATION-CYCLE", "LIN-COMMUNICATION-CONTROLLER", "BSW-BACKGROUND-EVENT", "CAN-FD", "ETH-IP-PROPS", "APPLICATION-ACTION-ITEM", "SECURED-PDU-HEADER-16-BIT", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "PRE-COMPILE", "AR-PACKAGE", "VAR-POWER-ON-INIT", "NOT-DEFINED", "IEEE-1722-TP-ETHERNET-FRAME", "SELECTED", "CYCLE-REPETITION-32", "UCM-STEP", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "SLAVE", "PDF", "KEYWORD", "CRYPTO-SERVICE-CERTIFICATE", "CODEGENERATION", "ENCRYPTION", "HEALTH-CHANNEL-EXTERNAL-MODE", "TOPIC-1", "NEW-IS-DIFFERENT", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "PROCESS", "BUILD-ACTION-ENTITY", "I-PDU", "DETAILED-BYPASSING-FILTERS", "KY", "TK", "UDP-CHECKSUM-DISABLED", "ECUC-FLOAT-PARAM-DEF", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "DOES-NOT-REPORT-EXECUTION-STATE", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "TO", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "ETHERNET-PHYSICAL-CHANNEL", "AUTHENTICATE", "WRITE-ONLY", "DIAGNOSTIC-EVENT-MANAGER", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "RESTART-APPLICATION", "DIAGNOSTIC-PORT-INTERFACE", "VERIFICATION", "CAN-BE-TERMINATED", "ATP-FEATURE", "ECUC-ENUMERATION-LITERAL-DEF", "BSW-MODULE-DESCRIPTION", "BASIC-SOFTWARE-MODE-MANAGER", "MI", "FALSE", "AGE-CONSTRAINT", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "DEFAULT-IF-UNDEFINED", "SL", "CONFIGURED", "PUBLISHED-INFORMATION", "STRICTLY-DECREASING", "SINGLE-LANGUAGE-REFERRABLE", "BUILD-TYPE-RELEASE", "HI", "LONG-HEADER", "GET", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "BASE-T", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "ABSTRACT-ETHERNET-FRAME", "IGNITION", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "HU", "SOFTWARE-PACKAGE-STEP", "SVG", "MASEKD-NEW-EQUALS-MASKED-OLD", "PDU-R", "HIGH", "RPT-LEVEL-1", "NO-SHOW-LONG-NAME", "DOES-NOT-SUPPORT-BUFFER-LOCKING", "DLT-APPLICATION-TO-PROCESS-MAPPING", "CONSUMER", "VIEW-MAP-SET", "DIAGNOSTIC-DATA-IDENTIFIER", "NM-ECU", "SIGNAL-BASED-EVENT-DEPLOYMENT", "USE-ARRAY-BASE-TYPE", "AFTER-SALES", "TRIGGER-UNICAST", "APPLICATION-PARTITION", "DATA-RECEIVE-ERROR-EVENT", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "PTP--IEEE-1588--2002", "REQUEST-CALLBACK-TYPE-MANUFACTURER", "DDS-SECURE-GOVERNANCE", "CHECK-AT-NEXT-HALT", "SW-VARIABLE-PROTOTYPE", "CALIBRATION-ONLINE", "BSW-MODE-MANAGER-ERROR-EVENT", "OPTIONS", "FM-FEATURE-MAP-CONDITION", "OPERATING-SYSTEM", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "MEASUREMENT-POINT", "TLV-DATA-ID-DEFINITION-SET", "GN", "ICMP", "DO-IP", "MASKED-NEW-EQUALS-X", "XXG-MII", "TRANSFORMER-STATUS-FORWARDING", "SV", "IA", "NETWORK", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "BSW-MODE-SWITCHED-ACK-EVENT", "CLIENT-SERVER-INTERFACE", "SW-CLASS-ATTR-IMPL", "DZ", "XREF-TARGET", "MODE-SWITCH-INTERFACE", "RUNNABLE-ENTITY-STARTED", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "DIAGNOSTIC-INFO-TYPE", "DIAGNOSTIC-J-1939-SPN", "STORE-PERSISTENTLY", "COMMUNICATION-CLUSTER", "FLAT-MAP", "DIAGNOSTIC-SESSION", "VARIABLE-ACCESS", "TW", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "DIAGNOSTIC-IUMPR", "VEHICLE-ROLLOUT-STEP", "HEALTH-CHANNEL", "REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE", "CRYPTO-SIGNATURE-SCHEME", "BO", "SD", "GLOBAL-SUPERVISION-ENTITY", "ACCESS-PERMISSION-SERVICE-INSTANCE", "OTHER", "IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "DDS-PROVIDED-SERVICE-INSTANCE", "DIAGNOSTIC-SERVICE-SW-MAPPING", "ESP", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "CRYPTO-CERTIFICATE-INTERFACE", "DO-IP-INTERFACE", "SW-MC-BASE-TYPE", "TP-CONFIG", "PERSISTENCY-INTERFACE-ELEMENT", "ACTIVATE", "FRAME-TRIGGERING", "NON-OS-MODULE-INSTANTIATION", "FLEXRAY-TP-NODE", "SESSION-HANDLING-INACTIVE", "UNIT-GROUP", "TIMING-CONSTRAINT", "USER-DEFINED-GLOBAL-TIME-SLAVE", "FO", "CLOSED", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "CLIENT-SERVER-OPERATION", "TD-EVENT-I-PDU", "RETURN-VALUE-PROVIDED", "HW-ATTRIBUTE-DEF", "NO-TRUSTED-PLATFORM-SUPPORT", "TRANSFORMER-HARD-ERROR-EVENT", "ETH-TCP-IP-PROPS", "REST-NUMBER-PROPERTY-DEF", "ALTERNATING-8-BIT", "COUPLING-PORT-SCHEDULER", "PROCESSOR", "BSW-INTERNAL-BEHAVIOR", "SW-MC-INTERFACE-SOURCE", "TLS-CRYPTO-CIPHER-SUITE-PROPS", "OEM-BOOT", "COM-OFFER-SERVICE-GRANT", "HOOK", "SOCKET-CONNECTION-BUNDLE", "PURE-LOCAL-TIME-BASE", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "LOG-AND-TRACE-INSTANTIATION", "FUNCTIONAL-ADDRESS", "INTERFACE-MAPPING", "PROCESS-TO-MACHINE-MAPPING-SET", "CONST", "CRC-VALIDATED", "ISO-6", "TD-EVENT-FRAME-ETHERNET", "USER-DEFINED-METHOD-DEPLOYMENT", "FAULT", "ABSTRACT-CAN-PHYSICAL-CHANNEL", "DATA-TYPE-MAPPING-SET", "ECUC-DESTINATION-URI-DEF", "ADDR-METHOD-SHORT-NAME", "REST-ENDPOINT-PUT", "NEVER", "SERVICE-INTERFACE-METHOD-MAPPING", "AUTO", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "INTERNAL-TRIGGERING-POINT", "PHYSICAL-CHANNEL", "EQUAL", "FUNCTION-INHIBITION-MANAGER", "COMMAND-LINE-SHORT-FORM", "POST", "ACTIVATION-MULTICAST", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "AS", "NEW-IS-OUTSIDE", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "NONE", "DEFAULT-MODE", "DIAGNOSTIC-COM-CONTROL", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "COM-GRANT", "APPLICATION-ERROR", "SOMEIP-METHOD-DEPLOYMENT", "SYSTEM-SIGNAL-GROUP", "ATP-DEFINITION", "COM-METHOD-GRANT-DESIGN", "CP", "RESET-VM", "OBD-CONTROL-SERVICE-NEEDS", "NO-RETURN-VALUE-PROVIDED", "COMPU-METHOD", "OS-MODULE-INSTANTIATION", "I-SIGNAL-TO-I-PDU-MAPPING", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "NO-AFFECT", "ES", "SYMBOLIC-NAME-PROPS", "CALIBRATION-PARAMETER-VALUE-SET", "NOT-EQUAL", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "OCCURENCE", "INTERRUPT", "PERIODIC-RATE-SLOW", "IMPLEMENTATION-DATA-TYPE", "UDP", "CYCLE-REPETITION-20", "HW-ELEMENT", "TRANSPORT", "SERVER-AUTHENTICATE", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "FRAME", "SEC-OC-DEPLOYMENT", "IS", "NO-DEFAULT", "TD-EVENT-SWC", "PORT-PROTOTYPE", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "FURTHER-ACTION-BYTE-NEEDS", "GRANT-DESIGN", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "FAILURE-ONLY", "LTS-13", "BOLDITALIC", "TRIGGER", "LOW", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "NO-KEEP", "SLP", "GLOBAL-TIME-FR-MASTER", "ECUC-INSTANCE-REFERENCE-DEF", "NEW-IS-LESS-OR-EQUAL", "BROAD-R-REACH", "NO-PROTECTION", "VARIANT-POST-BUILD", "RECOVERY-VIA-APPLICATION-ACTION", "ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "SYNCHRONOUS-SERVER-CALL-POINT", "PROCESSOR-CORE", "PERSISTENCY-FILE-PROXY-INTERFACE", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "OBD-DCY", "DLT-ARGUMENT", "TRACEABLE-TEXT", "ECC", "STD_AXIS", "NTP--RFC-958", "APPLICATION-DATA-TYPE", "DIAGNOSTIC-DATA-TRANSFER", "ROTATE-90-CCW-FIT-TO-TEXT", "REQUEST-CALLBACK-TYPE-SUPPLIER", "ETHERNET-COMMUNICATION-CONNECTOR", "CLASS-CONTENT-CONDITIONAL", "ABSTRACT-IAM-REMOTE-SUBJECT", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "SERVICE-INSTANCE-TO-MACHINE-MAPPING", "SPEC-ELEMENT-SCOPE", "CYCLE-REPETITION-1", "DIAGNOSTIC-J-1939-SW-MAPPING", "LINK-LOCAL--DOIP", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "OBSERVER", "ADAPTIVE-SERVICE-FIND-STARTED", "DEVELOPMENT-ERROR", "TD-EVENT-I-SIGNAL", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "ECUC-FUNCTION-NAME-DEF", "TCP-OPTION-FILTER-LIST", "INSTALL", "ACTION-ITEM", "CAPTURE-ASYNCHRONOUSLY-TO-REPORTING", "RETURN-ON-EVENT-CLEARED", "PHM-RULE", "CLIENT-SERVER-INTERFACE-MAPPING", "MACHINE-TIMING", "OPERATION-CALL-RECEIVED", "PERIODIC-RATE-FAST", "NO-NEWLINE", "SHOW-CONTENT", "TRANSFORMATION-PROPS-SET", "RM", "CLIENT-AUTHENTICATE", "DIAGNOSTIC-CONNECTION", "CONTINUOUS-ON-MODE", "VERIFY", "PERSISTENCY-FILE-ARRAY", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "SM", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "AS-IS", "PHYSICAL-CAN-FD", "UNSPECIFIED", "WORST-CASE-HEAP-USAGE", "WEIGHTED-ROUND-ROBIN", "FAST-FLASHING-MODE", "SECURITY-EVENT-FILTER-CHAIN", "EXCLUSIVE-AREA", "TX-TRIGGER-SINGLE", "NA", "SECURED-I-PDU", "PERSISTENCY-KEY-VALUE-STORAGE", "COM-MANAGEMENT-MAPPING", "SWC-SERVICE-DEPENDENCY", "TX-REF-TRIGGER-GAP", "RAW-DATA-STREAM-INTERFACE", "RUNNABLE-ENTITY-GROUP", "EXCLUDE-FROM-FLASH", "GETTER", "STANDARD", "INDIVIDUAL", "CYCLE-REPETITION-40", "IS-LESS-THAN", "MODE-ACCESS-POINT-IDENT", "DEFINE-BY-IDENTIFIER", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "FRAME-ETHERNET-RECEIVED-ON-BUS", "FAILURE-AND-SUCCESS", "PREDEFINED-VARIANT", "MK", "CONSUMED-EVENT-GROUP", "ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF", "DO-IP-INSTANTIATION", "INDICATOR-STATUS-NEEDS", "ENABLE", "AB", "CAT-2", "DEDICATED", "REST-INTEGER-PROPERTY-DEF", "NO-SHOW-NUMBER", "TIMING-DESCRIPTION", "CO", "BUILD", "RTE-EVENT", "MODE-TRANSITION", "ISO-11992--4", "I-PV-6-EXT-HEADER-FILTER-SET", "CVC", "INSTANCE-ID", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "HR", "DDS-RPC-SERVICE-DEPLOYMENT", "PORT-INTERFACE", "OPERATION-CALL-RESPONSE-RECEIVED", "CHANNEL-B", "LOCAL-SUPERVISION", "TIMING-EXTENSION-RESOURCE", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "CRYPTO-INTERFACE", "COM-FIND-SERVICE-GRANT", "ACK-WITHOUT-RT", "DE", "DEFICIT-ROUND-ROBIN", "SETTER", "SERVICE-NEEDS", "VAR-NO-INIT", "ECUC-QUERY", "VARIABLE-DATA-PROTOTYPE-RECEIVED", "ETHERNET-RAW-DATA-STREAM-MAPPING", "ECUC-FOREIGN-REFERENCE-DEF", "LIN-EVENT-TRIGGERED-FRAME", "LOGIC-ADDRESS", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "BSW-DISTINGUISHED-PARTITION", "SHOW-NUMBER", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "UR", "USE-LAST-CONTEXT-DATA", "COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING", "WARMUP", "I-SIGNAL-GROUP", "AR-ELEMENT", "COM-AXIS", "TRIGGERED-ON-CHANGE-WITHOUT-REPETITION", "OPERATION-INVOKED-EVENT", "VOLATILE", "RESET-ECU", "CRC-IGNORED", "WARN", "TD-EVENT-BSW-MODE-DECLARATION", "HUB", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "PHM-LOGICAL-EXPRESSION", "EL", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "CRYPTO-SERVICE-JOB-NEEDS", "ECUC-MULTILINE-STRING-PARAM-DEF", "GZIP", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "IS-GREATER-THAN", "SDG-PRIMITIVE-ATTRIBUTE", "CRYPTO-MODULE-INSTANTIATION", "SOCKET-ADDRESS", "END-TO-END-PROTECTION", "RPT-ENABLER-ROM", "DDS-METHOD-DEPLOYMENT", "PNC-MAPPING-IDENT", "FIELD", "ALLOCATOR", "SDG-FOREIGN-REFERENCE", "FM-FEATURE-MODEL", "TRANSFORMING-I-SIGNAL", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "NETWORK-ENDPOINT", "ISO-14229--1", "SEARCH-FOR-SPECIFIC-INSTANCE", "DELETE", "EXCLUSIVE", "PDU", "USER-DEFINED", "CAN-NM-NODE", "ZU", "OBD", "CONNECT", "CLIENT-DECRYPT", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE", "DIAGNOSTIC-DE-AUTHENTICATION", "APPLICATION-PRIMITIVE-DATA-TYPE", "TESTED-AND-FAILED", "CYCLE-REPETITION-64", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "STOP-TRIGGER", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "WILL-RECEIVE", "XH", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "READ-WRITE", "CONTAINER-I-PDU", "BLUEPRINT-MAPPING-SET", "EXECUTABLE-GROUP", "DIAGNOSTIC-PROTOCOL", "VARIABLE-DATA-PROTOTYPE", "SYSTEM-TIMING", "ABSTRACT", "I-PV-6-EXT-HEADER-FILTER-LIST", "MASKED-NEW-DIFFERS-X", "P-PORT-PROTOTYPE", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "INCREASING", "IS-FAILED", "EXECUTION-TIME", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "RPT-PROFILE", "BSW-OPERATION-INVOKED-EVENT", "ECUC-DEFINITION-ELEMENT", "ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION", "INTER-PARTITION-INTRA-ECU", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "DYNAMIC-PART-TRIGGER", "EXAMPLE", "SECURITY-EVENT-STATE-FILTER", "INOUT", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "DIAGNOSTIC-INDICATOR-INTERFACE", "CLIENT-MAC-GENERATE", "SHOW-SEE", "DIAGNOSTIC-IO-CONTROL-CLASS", "RECOVERY-NOTIFICATION", "BLINK-OR-CONTINUOUS-ON-MODE", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "ECUC-INTEGER-PARAM-DEF", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "RAW", "YO", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "REF-NON-STANDARD", "USER-DEFINED-PHYSICAL-CHANNEL", "UCM-MASTER", "CRYPTO-SERVICE-PRIMITIVE", "PERSISTENCY-DATA-ELEMENT", "REPORT-AFTER-INIT", "IP-SEC-CONFIG-PROPS", "TD-EVENT-VFB-REFERENCE", "PERSISTENCY-DEPLOYMENT", "FM-FEATURE-SELECTION", "DIAGNOSTIC-J-1939-SPN-MAPPING", "MG", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "SECURE-COM-PROPS", "TN", "MASKED-NEW-EQUALS-MASKED-OLD", "CRYPTO-NEEDS", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "J-1939-NM-CLUSTER", "IS-EQUAL", "DIAGNOSTIC-AUTHENTICATION-CLASS", "LO", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "ROUTER", "INTRUSION-DETECTION-SECURITY-MANAGEMENT", "SW-CODE-SYNTAX", "STD", "CAN-BE-TERMINATED-AND-RESTARTED", "NO-SHOW-PAGE", "EXECUTION-ORDER-CONSTRAINT", "PRESENTATION-DISCRETE", "APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE", "TRIGGER-INTERFACE-MAPPING", "IT", "FLEXRAY-AR-TP-CONFIG", "REST-ABSTRACT-PROPERTY-DEF", "SW-RECORD-LAYOUT", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "RU", "PHM-ACTION-LIST", "COMPILE", "SEARCH-FOR-ID", "ET", "INLINE", "DCM-I-PDU", "SYNCHRONIZED-TIME-BASE-CONSUMER", "FLEXRAY-CLUSTER", "REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST", "OBD-MONITOR-SERVICE-NEEDS", "ALL-16-BIT", "GROSS", "INDICATE", "MT", "IS-LESS-OR-EQUAL", "XCP", "FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION", "SCHEDULE-VARIANT-7", "NM-NODE", "FM-FEATURE-MAP-ELEMENT", "SOMEIP", "GLOBAL-TIME-ETH-SLAVE", "USER-DEFINED-EVENT-DEPLOYMENT", "FULL", "DIAGNOSTIC-COM-CONTROL-CLASS", "NO-SHOW-CATEGORY", "INFINITE", "TESTED", "ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER", "VARIANT-LINK-TIME", "REST-RESOURCE-DEF", "RUNNABLE-ENTITY-ACTIVATED", "NO-BREAK", "FM-FEATURE-MAP-ASSERTION", "ABSTRACT-CAN-CLUSTER", "IDENT-CAPTION", "BINARY-MANIFEST-ITEM-DEFINITION", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "DIAGNOSTIC-ECU-RESET-INTERFACE", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "DIAGNOSTIC-REQUEST-DOWNLOAD", "SINGLE-CORE-REENTRANT", "ECUC-CHOICE-REFERENCE-DEF", "SERVICE-METHOD-DEPLOYMENT", "BSW-COMPOSITION-TIMING", "OBD-RATIO-SERVICE-NEEDS", "SUPPLIER", "TIMING-DESCRIPTION-EVENT-CHAIN", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "LIN-SPORADIC-FRAME", "XOR", "CPP", "HALF-DUPLEX-MODE", "TRANSFORMATION-PROPS", "SQ", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "TR", "DO-IP-POWER-MODE-STATUS-NEEDS", "ARBITRARY-EVENT-TRIGGERING", "SO", "STARTUP-CONFIG-SET", "STD-AXIS", "MC-FUNCTION", "IMPLEMENTATION-PROPS", "GLOBAL-TIME-MASTER", "NEW-IS-GREATER", "PERSISTENCY-KEY-VALUE-PAIR", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "LATENCY-TIMING-CONSTRAINT", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "DESELECTED", "MOST-SIGNIFICANT-BYTE-FIRST", "TA", "VAR-FAST", "ECUC-ENUMERATION-PARAM-DEF", "OPAQUE", "REST-ENDPOINT-GET", "IDSM-MODULE-INSTANTIATION", "DESCENDANT", "ADAPTIVE-FIELD-SETTER-CALLED", "CODE-GENERATION-TIME", "BONJOUR", "GLOBAL-SUPERVISION", "DEBOUNCE-DATA", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "REST-ENDPOINT-DELETE", "SEC-OC-CRYPTO-SERVICE-MAPPING", "ANY", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "SW-CALPRM-PROTOTYPE", "ALL-INDICES-DIFFERENT-ARRAY-SIZE", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "JAVA", "COMMUNICATION-INTRA-PARTITION", "LIN-TP-NODE", "FUNCTIONAL", "USER-DEFINED-TRANSFORMATION-PROPS", "PHM-RECOVERY-ACTION-INTERFACE", "HW-PIN-GROUP", "DO-IP-LOGIC-ADDRESS", "DLT-LOG-SINK", "DLT-ECU", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "LIN-COMMUNICATION-CONNECTOR", "BSW-TIMING-EVENT", "BSW-EVENT", "BSW-OS-TASK-EXECUTION-EVENT", "PA", "LIN-SLAVE-CONFIG-IDENT", "AUTONOMOUS", "SDG-CAPTION", "GENERAL-PURPOSE-I-PDU", "DIAGNOSTIC-ECU-RESET", "TRANSIENT-FAULT", "EXACT-OR-ANY-MINOR-VERSION", "FLEXRAY-PHYSICAL-CHANNEL", "CENTER", "DIAGNOSTIC-TROUBLE-CODE-UDS", "DEBUG", "BURST-PATTERN-EVENT-TRIGGERING", "DEFLATE", "AP-APPLICATION-ENDPOINT", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "LEAF-OF-TARGET-CONTAINER", "ATP-CLASSIFIER", "ADAPTIVE-METHOD-RESPONSE-SENT", "STANDARD-PORT", "PAYLOAD-AS-POINTER-TO-ARRAY", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "EVENT-COMBINATION-ON-STORAGE", "NM-PDU", "BACKGROUND-EVENT", "TH", "SERVER-VERIFY", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "ECU-MAPPING", "PROCESS-DESIGN", "SERVICE-INTERFACE", "FLEXRAY-FRAME-TRIGGERING", "IS-GREATER-OR-EQUAL", "AM", "APPLICATION-SW-COMPONENT-TYPE", "SW-ADDR-METHOD", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "TRANSFER", "SYNCHRONIZED-TIME-BASE-PROVIDER", "BSW-CALLED-ENTITY", "STARTUP-CONFIG", "ACL-OBJECT-SET", "JPG", "USER-DEFINED-I-PDU", "IDENTIFIABLE", "SYMMETRIC-KEY", "UDP-CHECKSUM-ENABLED", "TX-REF-TRIGGER", "DEF-ITEM", "UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT", "ECU-PARTITION", "WONT-CALL", "FIXED-SIZE", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "CURVE-AXIS", "LIMIT-TO-PAGE", "HW-PIN", "IK", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "NM-NETWORK-HANDLE", "CHANNEL-A", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "NO-SEVERITY", "SYNCHRONIZATION-TIMING-CONSTRAINT", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "SYSTEM-SUPPLIER-BOOT-RESP-APP", "IS-LESS-THAN-OR-EQUAL", "TIME-SYNC-MODULE-INSTANTIATION", "CONSTANT-SPECIFICATION-MAPPING-SET", "PRIVATE-KEY", "FINISH", "DIAGNOSTIC-DATA-BY-IDENTIFIER", "FLAT-INSTANCE-DESCRIPTOR", "PROVIDED-DDS-SERVICE-INSTANCE", "EOC-EVENT-REF", "100BASE-T1", "EXTERNAL-TRIGGERING-POINT-IDENT", "BLUEPRINT-DERIVATION-TIME", "SERVICE-INTERFACE-EVENT-MAPPING", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "VERTEX-OF-TARGET-CONTAINER", "PARAMETER-DATA-PROTOTYPE", "USE-FIRST-CONTEXT-DATA", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "ABSTRACT-RAW-DATA-STREAM-INTERFACE", "DEFAULT-ERROR-TRACER", "ECUC-LINKER-SYMBOL-DEF", "UNIT", "RAW-DATA-STREAM-DEPLOYMENT", "OUT", "MEMORY-SECTION", "SOMEIP-EVENT-DEPLOYMENT", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "EXECUTABLE", "LIN-TP-CONFIG", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "BE", "ASYMMETRIC-FROM-BYTE-ARRAY", "FIRE-AND-FORGET-MAPPING", "JA", "RESPONSE", "PS", "CLIENT-VERIFY", "FM-ATTRIBUTE-DEF", "IS-NOT-RELEVANT", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "NODE", "SERVICE-FIELD-DEPLOYMENT", "INVALID", "SECURITY-EVENT-MAPPING", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED", "DATA-FORMAT-ELEMENT-SCOPE", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "DIAGNOSTIC-SERVICE-CLASS", "FLEXRAY-NM-CLUSTER", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "SW-CLASS-PROTOTYPE", "OM", "CAN-COMMUNICATION-CONTROLLER", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "ACTIVE", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "I-PDU-PORT", "DIAGNOSTIC-SESSION-CONTROL", "SYNCHRONIZED-MASTER-TIME-BASE", "CAN-PHYSICAL-CHANNEL", "SIGNAL-BASED-FIELD-DEPLOYMENT", "SERVICE-TIMING", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "OEM-BOOT-RESP-APP", "BUILD-ACTION", "ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS", "PROVIDED-AP-SERVICE-INSTANCE", "SHOW-TYPE", "DEVELOPMENT-ERROR-TRACER", "VARIABLE-SIZE", "LIN-MASTER", "DOCUMENT-ELEMENT-SCOPE", "START-FROM-BEGINNING", "NO-SLOPPY", "USER-DEFINED-COMMUNICATION-CONNECTOR", "REST-ABSTRACT-ENDPOINT", "SERVICE-INTERFACE-MAPPING", "LINK-LOCAL", "AUTOSAR-DATA-PROTOTYPE", "ERROR-DETECTION", "COM-MANAGER", "IDS-DESIGN", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE", "LOGICAL-AND", "E-2-E-PROFILE-CONFIGURATION", "PHYSICAL-ADDRESS", "EXTERNAL-REPLACEMENT", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "KO", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "TRACE-REFERRABLE", "BLOCK-STATE", "GD", "CLEAR", "DATA-PROTOTYPE-GROUP", "SEC-OC-SECURE-COM-PROPS", "SIGN", "SERVICE-EVENT-DEPLOYMENT", "BSW-MODULE-ENTITY-TERMINATED", "PNG", "DIAGNOSTIC-TROUBLE-CODE-OBD", "PHM-CHECKPOINT", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "AP-APPLICATION-ERROR-SET", "DROP", "SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE", "SECURED-PDU-HEADER-32-BIT", "HEALTH-CHANNEL-SUPERVISION", "PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE", "EVENT-WINDOW-CURRENT-CYCLE", "OPEN", "DERIVED-FROM", "SYSTEM-MEMORY-USAGE", "VLAN-CONFIG", "IP-SEC-RULE", "XDOC", "UK", "ROOT-SW-COMPONENT-PROTOTYPE", "COUPLING-PORT-STRUCTURAL-ELEMENT", "MONITOR-MODE", "ROTATE-90-CW-LIMIT-TO-TEXT", "ANALYZED-EXECUTION-TIME", "REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE", "SEARCH-FOR-ANY", "DIAGNOSTIC-ACCESS-PERMISSION", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "PRESHARED-KEY-IDENTITY", "PORT-INTERFACE-DEFINITION", "ETHERNET-CLUSTER", "SECRET-SEED", "EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE", "TEST-FAILED-BIT", "DIAGNOSTIC-EVENT-INTERFACE", "TASK", "ECUC-VALUE-COLLECTION", "NOT-VALID", "RUNNABLE-ENTITY", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "ROUGH-ESTIMATE-HEAP-USAGE", "PRESENTATION-CONTINUOUS", "ECUC-REFERENCE-DEF", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "DIAGNOSTIC-DATA-PORT-MAPPING", "ADAPTIVE-FIELD-GETTER-COMPLETED", "ABSTRACT-REQUIRED-PORT-PROTOTYPE", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "NO-TRANSFORMER-ERROR-HANDLING", "CRYPTO-KEY-SLOT-INTERFACE", "SW-COMPONENT-TYPE", "MODE-SWITCHED-ACK-EVENT", "PRIO-OCC", "TLS-13", "QUEUED", "OBSERVER-BASED", "OR", "FLEXRAY-AR-TP-NODE", "SPECIFICATION-DOCUMENT-SCOPE", "LN", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "END-2-END-METHOD-PROTECTION-PROPS", "CALLOUT", "COMMUNICATION-CONTROLLER", "SR", "CYCLE-REPETITION-4", "ECUC-URI-REFERENCE-DEF", "SCHEDULED", "NO-PGWIDE", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "APPLICATION-ASSOC-MAP-DATA-TYPE", "TT", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "PRIMARY-ECU", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "CRYPTO-PROVIDER-INTERFACE", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "FIRST-CONTAINED-TRIGGER", "PLATFORM-MODULE-ENDPOINT-CONFIGURATION", "AGGREGATION-TAILORING", "REPETITIVE-EOC", "MACRO", "BSW-MGR-NEEDS", "BN", "SN", "INTERNAL-BEHAVIOR", "COMMUNICATION-CONNECTOR", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "SOFTWARE-PACKAGE", "IS-OK", "COMPILER", "ATP-PROTOTYPE", "DIAGNOSTIC-VALUE-NEEDS", "COMPOSITION-SW-COMPONENT-TYPE", "NON-VOLATILE", "REGULAR", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "DIAGNOSTIC-SECURITY-LEVEL", "DIAGNOSTIC-ABSTRACT-ALIAS-EVENT", "WATCHDOG-ACTION-ITEM", "EXECUTABLE-ENTITY", "IP-SEC-IAM-REMOTE-SUBJECT", "ISO", "XG-MII", "ECUC-VALIDATION-CONDITION", "BSW-MODULE-DEPENDENCY", "CONSISTENCY-NEEDS", "TLS-JOB-REQUIREMENT", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "TRIGGER-ACTIVATED", "SCHEDULE-VARIANT-3", "MODE-DECLARATION", "UDS", "HA", "CA", "DO-IP-ROUTING-ACTIVATION", "DIAGNOSTIC-IO-CONTROL", "CAN-CLUSTER", "CAT-1", "RPT-EXECUTION-CONTEXT", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS", "RETURN-ON-EVENT-STOPPED", "REQUIRES-CALLBACK-EXECUTION", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "-500-MILES", "EXCLUSIVE-AREA-NESTING-ORDER", "NO-SHOW-SEE", "IMMEDIATELY", "SCHEDULE-VARIANT-5", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "NO-BOOT", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "LA", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "BASE-TYPE", "V-2-X-FAC-USER-NEEDS", "EVALUATED-VARIANT-SET", "ENCRYPT-AND-SIGN", "FRAME-QUEUED-FOR-TRANSMISSION", "WAIT-POINT", "MAPPING-SCOPE-ECU", "FOR-ALL", "PROCESSING-STYLE-ASYNCHRONOUS", "RIGHT", "APPLICATION-COMPOSITE-DATA-TYPE", "AND", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "MODE-DECLARATION-REQUESTED", "POWER-WINDOW-TIME", "IEEE802-1AS", "RX-TRIGGER", "TRAP", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED", "REST-BOOLEAN-PROPERTY-DEF", "SG", "ETHERNET-NETWORK-CONFIGURATION", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "CYCLE-REPETITION-50", "UCM-DESCRIPTION", "UPLOADABLE-PACKAGE-ELEMENT", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "ADAPTIVE-FIELD-GETTER-CALLED", "PERSISTENCY-INTERFACE", "SUPERVISION-ENTITY", "CRYPTO-CERTIFICATE", "NO-SUPPORT", "DATA-FORMAT-ELEMENT-REFERENCE", "SOMEIP-TRANSFORMATION-PROPS", "SYSTEM", "DLT-LOG-CHANNEL-DESIGN", "BSW-VARIABLE-ACCESS", "LOGICAL-SUPERVISION", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "FJ", "BSW-SCHEDULER-NAME-PREFIX", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "CONSTRAINT-TAILORING", "GENERAL-PARAMETER", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "MODE-DECLARATION-MAPPING-SET", "NM-CONFIG", "MULTIPLE-OCCURRENCES", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "TIMING-EXTENSION", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "PC-AFFECTS-PB", "DEFAULT", "DROP-FRAME", "CONCRETE-PATTERN-EVENT-TRIGGERING", "PT", "SERVICE-INTERFACE-PEDIGREE", "BRIEF-BYPASSING-FILTERS", "FM-FEATURE-RELATION", "MIDDLE", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "ARBITRATION", "ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE", "WATCHDOG-PHM-ACTION-ITEM", "1000BASE-T", "PORT-INTERFACE-MAPPING-SET", "REQUEST", "DIAGNOSTIC-DATA-ELEMENT", "ITALIC", "TD-EVENT-TRIGGER", "REBOOT", "AR", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "FILE", "SENDER-RECEIVER-INTERFACE", "CHECKPOINT-TRANSITION", "REF-NONE", "ECUC-CONTAINER-VALUE", "ABSTRACT-SERVICE-INSTANCE", "FIX_AXIS", "SOMEIP-REQUIRED-EVENT-GROUP", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "DIAGNOSTIC-STOP-ROUTINE", "ECUC-DEFINITION-COLLECTION", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "NO-STORE-EVENT", "AP-APPLICATION-ERROR", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "SERIALIZATION-TECHNOLOGY", "SERVICE-INTERFACE-MAPPING-SET", "GLOBAL-TIME-GATEWAY", "WARNING", "ECUC-ABSTRACT-STRING-PARAM-DEF", "ECUC-PARAM-CONF-CONTAINER-DEF", "LIN-SCHEDULE-TABLE", "RPT-ENABLER-RAM", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "MIN", "SERVICE-INTERFACE-DEPLOYMENT", "BH", "INLINE-CONDITIONAL", "IEEE802-11P", "REDUNDANT-PER-KEY", "ASYNCHRONOUS-SERVER-CALL-POINT", "ABSTRACT-IMPLEMENTATION-DATA-TYPE", "SLOPPY", "SOMEIP-TP-CONFIG", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "PTP--IEEE-1588--2008", "DOCUMENTATION", "TOPIC", "REQUEST-NO-RETURN", "FY", "INTERRUPT-CAT-1", "CRYPTO-SERVICE-QUEUE", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "DO-IP-TP-CONFIG", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "NOT-SENT", "NOHREF", "ECU-MANAGER", "STATIC-OR-DYNAMIC-PART-TRIGGER", "SECURE-ON-BOARD-COMMUNICATION", "PERSISTENCY-DEPLOYMENT-ELEMENT", "ML", "V-2-X-NOT-SUPPORTED", "SW-SYSTEMCONST", "GENERIC-ETHERNET-FRAME", "SSDP", "REJECT", "TIME-SYNCHRONIZATION-INTERFACE", "CS", "SDG-TAILORING", "ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "TD-EVENT-VFB-PORT", "IDSM-PROPERTIES", "MULTILANGUAGE-REFERRABLE", "CAPTURE-SYNCHRONOUSLY-TO-REPORTING", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "REST-STRING-PROPERTY-DEF", "DIAGNOSTIC-LOG-AND-TRACE", "ADAPTIVE-SERVICE-FIND-COMPLETED", "FLEXRAY-TP-CONNECTION-CONTROL", "USER-DEFINED-COMMUNICATION-CONTROLLER", "SYSTEM-SUPPLIER-BOOT", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "SIGNAL-BASED-METHOD-DEPLOYMENT", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "SK", "VAR", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "PENDING", "AUTO-IP", "BSW-MODE-SWITCH-EVENT", "PLATFORM-HEALTH-MANAGEMENT-INTERFACE", "SW", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "AP", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "PDU-TRIGGERING", "ADAPTIVE-MODULE-INSTANTIATION", "PC-AFFECTS-LT-AND-PB", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "CALLBACK", "ADAPTIVE-SERVICE-OFFER-STARTED", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "CODE", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "ARGUMENT-DATA-PROTOTYPE", "APPLICATION-RECORD-DATA-TYPE", "LIN-FRAME-TRIGGERING", "SECURE-COM-PROPS-SET", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "PROCESS-EXECUTION-ERROR", "PHM-ACTION-ITEM", "ROTATE-180-LIMIT-TO-TEXT", "SECURITY-EVENT-REPORT-INTERFACE", "REPORT", "PROCESS-IS-NOT-SELF-TERMINATING", "ETH-TCP-IP-ICMP-PROPS", "I-SIGNAL-TRIGGERING", "RAW-DATA", "CRC-NOT-VALIDATED", "PSK", "CAN-FRAME", "NON-VOLATILE-RAM-MANAGER", "LINKER", "MIXED", "CAN-COMMUNICATION-CONNECTOR", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "FUNCTIONAL-CAN-FD", "N-PDU", "R-PORT-PROTOTYPE", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "COM_AXIS", "SIMULATED-EXECUTION-TIME", "TP-ADDRESS", "SCHEDULE-VARIANT-4", "APPLICATION-DEFERRED-DATA-TYPE", "TLS-12", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "FLOAT", "APPLICATION-ONLY", "DIAG-RESPONSE", "CYCLE-REPETITION-2", "PDU-ACTIVATION-ROUTING-GROUP", "MIXED-29-BIT", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "ECU", "LOGICAL-OR", "SW-COMPONENT-PROTOTYPE", "EVENT-STORAGE-DISABLED", "CALIBRATION-VARIABLES", "PHM-ARBITRATION", "BSW-ENTRY-RELATIONSHIP-SET", "BUS-MIRROR-CHANNEL-MAPPING-IP", "RPT-LEVEL-2", "ECUC-STRING-PARAM-DEF", "COM-GRANT-DESIGN", "ACTIVATION-AND-TRIGGER-UNICAST", "EXECUTION-TIME-CONSTRAINT", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "SWC-MODE-MANAGER-ERROR-EVENT", "DIAGNOSTIC-SW-MAPPING", "MASKED-NEW-DIFFERS-MASKED-OLD", "JW", "ALL-SUPPORTED-DTCS", "SERVICE-PROXY-SW-COMPONENT-TYPE", "KS", "COLLECTION", "TD-EVENT-OPERATION", "SS", "ADAPTIVE-SERVICE-OFFER-COMPLETED", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "SAE-J-1939--73", "GLOBAL-TIME-DOMAIN", "MULTIPLEXED-I-PDU", "AMBER-WARNING", "LV", "DIAGNOSTIC-AUTHENTICATION", "RPT-EXECUTABLE-ENTITY", "SOMEIP-EVENT", "COMM-CONNECTOR-PORT", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "SPORADIC-EVENT-TRIGGERING", "MACHINE-DESIGN", "SHOW-CATEGORY", "DIAGNOSTIC-UPLOAD-INTERFACE", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "FLEXRAY-NM-NODE", "DEVELOPMENT", "REACTION", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "ACK-WITH-RT", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "IS-STOPPED", "COM-EVENT-GRANT", "ABSTRACT-PROVIDED-PORT-PROTOTYPE", "IS-EXPIRED", "DEFINE-BY-MEMORY-ADDRESS", "PHYSICAL-DIMENSION-MAPPING-SET", "CAN-FRAME-TRIGGERING", "SECURITY-EVENT-CONTEXT-MAPPING", "DIAGNOSTIC-START-ROUTINE", "SW-BASE-TYPE", "ALL", "RESOURCE-GROUP", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "SI", "HEAD", "BRIEF", "POST-BUILD", "ECUC-ABSTRACT-REFERENCE-DEF", "CAPTURE-SYNCHRONOUS-TO-REPORTING", "SCHEDULE-VARIANT-6", "DIAGNOSTIC-REQUEST-UPLOAD", "DIAGNOSTIC-SERVICE-TABLE", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "ENABLED", "NAND", "ETHERNET-FRAME-TRIGGERING", "RUNTIME-ERROR", "RECORD-VALUE-FIELD", "SA", "SYMBOL-PROPS", "ORDINARY-EOC", "PDU-TO-FRAME-MAPPING", "LIN-SLAVE", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "FR", "DIAGNOSTIC-INDICATOR-NEEDS", "CONDITIONAL", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "FUNCTION-INHIBITION-NEEDS", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "EXECUTABLE-TIMING", "PERIODIC-EVENT-TRIGGERING", "SOMEIP-EVENT-GROUP", "MEASURED-EXECUTION-TIME", "PASSIVE", "NEWLINE", "IP-IAM-REMOTE-SUBJECT", "SDG-DEF", "DIAGNOSTIC-ROUTINE-CONTROL", "IE", "APPLICATION-ARRAY-DATA-TYPE", "EVENT-WINDOW-INFINITE", "KN", "TRUE", "QU", "LAST-FAILED", "SECURE-COMMUNICATION-PROPS-SET", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "NO-FLOAT", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "PLAIN", "SECURITY-EVENT-ONE-EVERY-N-FILTER", "SAE-J-2012--DA", "NV-BLOCK-DESCRIPTOR", "RUNNABLE-ENTITY-VARIABLE-ACCESS", "ADAPTIVE-PLATFORM-SERVICE-INSTANCE", "GENERIC-MODULE-INSTANTIATION", "BSW-DEBUG-INFO", "SEC-OC-JOB-REQUIREMENT", "ERROR", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "NEW-IS-GREATER-OR-EQUAL", "SECURITY", "ABSTRACT-CAN-COMMUNICATION-CONTROLLER", "FRAME-PORT", "REF-ALL", "BUS-MIRROR-CHANNEL-MAPPING", "DEFAULT-TRACE-STATE-DISABLED", "ECUC-QUERY-EXPRESSION", "DETAILED", "DIAGNOSTIC-FREEZE-FRAME", "MC-GROUP", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "IEEE802-1AS-AUTOSAR", "NV-BLOCK-NEEDS", "DOMAIN-PARTICIPANT-USER-DATA-QOS", "ABSTRACT-EXECUTION-CONTEXT", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "HARDWARE-TEST-MANAGER", "EU", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "OFFSET-TIMING-CONSTRAINT", "KM", "DIAGNOSTIC-IO-CONTROL-NEEDS", "LIFE-CYCLE-INFO-SET", "NO-MONOTONY", "NO-STATUS-BYTE-CHANGE", "PLATFORM-PHM-ACTION-ITEM", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING", "EVENT-ACCEPTANCE-DISABLED", "COMPOSITE-INTERFACE", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "WO", "DDS-SECURE-COM-PROPS", "AUTO-IPDHCPV-4", "REST-ABSTRACT-NUMERICAL-PROPERTY-DEF", "CRYPTO-JOB", "TIMING-MODE-INSTANCE", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "EVENT-STORAGE-ENABLED", "ERROR-CORRECTION", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "USER-DEFINED-GLOBAL-TIME-MASTER", "RAW-DATA-STREAM-CLIENT-INTERFACE", "SINGLE", "VARIABLE-DATA-PROTOTYPE-SENT", "SAFETY", "GENERAL-PURPOSE-CONNECTION", "CRYPTO-PRIMITIVE", "DIAGNOSTIC-EVENT", "J-1939-TP-CONFIG", "FULL-DUPLEX-MODE", "AUTOSAR-DATA-TYPE", "IMPLEMENTATION", "J-1939-DCM-DM-19-SUPPORT", "DDS-TOPIC-ACCESS-RULE", "DIAGNOSTIC-ENV-MODE-ELEMENT", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "PARTIAL-NETWORK", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "WRONG-TRIGGER", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "PERSISTENCY-FILE-STORAGE", "SYNCHRONIZED-SLAVE-TIME-BASE", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "NO-SHOW-CONTENT", "TD-EVENT-SERVICE-INSTANCE-METHOD", "FIELD-MAPPING", "WORST-CASE-STACK-USAGE", "COM-FIELD-GRANT", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "DIAGNOSTIC-REQUEST-ROUTINE-RESULTS", "CRC-OPTIONAL", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "SOMEIP-FIELD", "TRIGGERED-ON-CHANGE", "EVENT-ACCEPTANCE-ENABLED", "PHM-SUPERVISION", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "COUPLING-PORT-SHAPER", "ATP-STRUCTURE-ELEMENT", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "LEGACY", "NOT-TESTED", "DIAGNOSTIC-CONTROL-NEEDS", "J-1939-NM-NODE", "HW-CATEGORY", "CRYPTO-NEED", "CPP-IMPLEMENTATION-DATA-TYPE", "FRAME-ETHERNET-RECEIVED-BY-IF", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "HEAP-USAGE", "VFB-TIMING", "NO-SHOW-SHORT-NAME", "TX-TRIGGER-MERGED", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "PRIMITIVE-ATTRIBUTE-TAILORING", "CONFIG-DATA", "ECUC-CHOICE-CONTAINER-DEF", "EVENT-COMBINATION-ON-RETRIEVAL", "NFOLD", "DIAGNOSTIC-STORAGE-CONDITION", "MEASURED-STACK-USAGE", "ONLY-THIS-CYCLE-AND-READINESS", "IW", "PERSISTENCY-FILE-PROXY", "RAW-DATA-STREAM-GRANT", "EVENT-HANDLER", "SUPERVISION-MODE-CONDITION", "DATA-INTERFACE", "MASTER", "BINARY-MANIFEST-ITEM", "WILL-SEND", "DIAGNOSTIC-CAPABILITY-ELEMENT", "J-1939-REQUEST-MANAGER", "SENT-TAGGED", "OFF", "PRE-COMPILE-TIME", "GLOBAL-TIME-CAN-SLAVE", "ON-EXIT", "ABSTRACT-ACCESS-POINT", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "CONFIRMED-DTC-BIT", "MODE-INTERFACE-MAPPING", "DLT-MESSAGE-COLLECTION-SET", "ROUGH-ESTIMATE-STACK-USAGE", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "RESOURCE-CONSUMPTION", "SINGLE-OCCURRENCE", "DECREASING", "preserve", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "INTERFACE-MAPPING-SET", "SERVICE-SW-COMPONENT-TYPE", "ADAPTIVE-METHOD-CALL-RECEIVED", "HW-ATTRIBUTE-LITERAL-DEF", "EXPLICIT", "OBD-RATIO-DENOMINATOR-NEEDS", "TD-EVENT-CYCLE-START", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "ASSEMBLY-SW-CONNECTOR", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "RES_AXIS", "LIN-NM-CLUSTER", "COUPLING-PORT-FIFO", "TLS-IAM-REMOTE-SUBJECT", "IDSM-INSTANCE", "VENDOR-SPECIFIC-SERVICE-NEEDS", "SDG-ATTRIBUTE", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "GENERAL-PURPOSE-PDU", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "DOES-NOT-USE-LOGGING", "PERIODIC-RATE-MEDIUM", "BULK-NV-DATA-DESCRIPTOR", "AUTO-IP--DOIP", "LAST-MODE", "TIME-BASE-RESOURCE", "NOT-AVAILABLE", "UNDECIDED", "DIAGNOSTIC-CLEAR-CONDITION", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "INTERRUPT-CAT-2", "TARGET-CONTAINER", "SYSTEM-MAPPING", "RPT-LEVEL-3", "HW-TYPE", "BR", "LISTEN", "MR", "TERMINATE", "SOMEIP-FIELD-DEPLOYMENT", "BI", "DIAGNOSTIC-EVENT-PORT-MAPPING", "ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING", "REPORTS-EXECUTION-STATE", "GIF", "EPS", "TRACEABLE", "BREAK", "ST", "NUMBER", "RPT-EXECUTABLE-ENTITY-EVENT", "REST-PRIMITIVE-PROPERTY-DEF", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "INTERPOLATION-ROUTINE-MAPPING-SET", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "TRIGGER-INTERFACE", "FILTERED", "RESTART", "FM-FEATURE-MAP", "ACTIVATION-UNICAST", "FM-FEATURE-SELECTION-SET", "SOFTWARE-ACTIVATION-DEPENDENCY", "SEARCH-FOR-ALL-INSTANCES", "VENDOR-SPECIFIC", "DONT-INVALIDATE", "SOFTWARE-CLUSTER", "REMOVE", "FI", "RULE", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING", "SH", "ADAPTIVE-EVENT-RECEIVED", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "ADAPTIVE-AUTOSAR-APPLICATION", "DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS", "EVENT-MAPPING", "ECU-STATE-MGR-USER-NEEDS", "TLS-SECURE-COM-PROPS", "NO-HEADER", "BOLD", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "CLASSIC", "BSW-MODULE-CLIENT-SERVER-ENTRY", "AY", "ACL-PERMISSION", "ABSTRACT-CAN-COMMUNICATION-CONNECTOR", "SYNCHRONOUS", "BIDIRECTIONAL", "DIAGNOSTIC-MONITOR-INTERFACE", "CAN-TP-NODE", "STRICT-PRIORITY", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "CRYPTO-TRUST-MASTER-INTERFACE", "DIAGNOSTIC-FIM-ALIAS-EVENT", "DIAGNOSTIC-COMMUNICATION-MANAGER", "J-1939-CLUSTER", "NV-RAM-MANAGER", "DIAGNOSTIC-ROUTINE-NEEDS", "CALCULATED", "SUPERVISION-CHECKPOINT", "100BASE-TX", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "UCM-MODULE-INSTANTIATION", "GLOBAL-SUPERVISION-NEEDS", "WATCH-TRIGGER-GAP", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "DIAGNOSTIC-READ-DTC-INFORMATION", "DIAGNOSTIC-TROUBLE-CODE", "MC-DATA-INSTANCE", "ETHERNET-RAW-DATA-STREAM-GRANT", "COM-FIELD-GRANT-DESIGN", "COMMAND-LINE-SIMPLE-FORM", "ON-DTC-STATUS-CHANGE", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "PARAMETER-ACCESS", "BLINK-MODE", "DDS-REQUIRED-SERVICE-INSTANCE", "DIAG-REQUEST", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "NETWORK-REPRESENTATION-FROM-COM-SPEC", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "SDG-CLASS", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "IDS-COMMON-ELEMENT", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "ASYMMETRIC-TO-BYTE-ARRAY", "SUPERVISED-ENTITY-NEEDS", "NETWORK-CONFIGURATION", "NO-SHOW-ALIAS-NAME", "RED-STOP-LAMP", "REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE", "TLS-JOB-MAPPING", "MULTICORE-REENTRANT", "SWC-TO-APPLICATION-PARTITION-MAPPING", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE", "MY", "RW", "I-SIGNAL-SENT-TO-COM", "X-MMI", "REST-OBJECT-REF", "LOG-AND-TRACE-INTERFACE", "SIGNAL-BASED", "ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT", "INIT-EVENT", "UDP-NM", "NV-DATA-INTERFACE", "HARDWARE-TEST-NEEDS", "REST-ARRAY-PROPERTY-DEF", "SDG-REFERENCE", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "BSW-INTERNAL-TRIGGERING-POINT", "ARTIFACT-CHECKSUM", "FRAME-TRANSMITTED-ON-BUS", "VI", "INFINITE-TIME-TO-RESPONSE", "BSW-SCHEDULE-EVENT", "ROTATE-90-CW-FIT-TO-TEXT", "V-2-X-ACTIVE-SUPPORTED", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "PARAMETER-INTERFACE", "WAIT-TIME-DATE", "CYCLE-REPETITION-10", "REST-ELEMENT-DEF", "PERSISTENCY-FILE-STORAGE-INTERFACE", "INSTRUCTION", "LOCAL", "X-509", "NV-BLOCK-SW-COMPONENT-TYPE", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY", "OBD-PID-SERVICE-NEEDS", "MODELED", "TL", "ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE", "COLDSTART", "V-2-X-FACILITIES", "TRIGGERED-ON-EVALUATION", "STRICTLY-INCREASING", "NOT", "DIAGNOSTIC-ECU-RESET-CLASS", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "RTE-EVENT-IN-SYSTEM-SEPARATION", "PER-EXECUTABLE", "DEFAULT-TRIGGER", "CRYPTO-DRIVER", "DIAGNOSTIC-MEMORY-IDENTIFIER", "MAC-MULTICAST-GROUP", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "TP-CONNECTION-IDENT", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "BSW-MODULE-CALL-POINT", "KEEP", "PROCESSING-STYLE-SYNCHRONOUS", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "NO", "COUPLING-PORT", "LINK-TIME", "MONO", "IDS-PLATFORM-INSTANTIATION", "BINARY-MANIFEST-PROVIDE-RESOURCE", "NET", "COMMAND-LINE-LONG-FORM", "IN", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "CANCEL", "RUNNABLE-ENTITY-TERMINATED", "PROTECTED", "IS-GREATER-THAN-OR-EQUAL", "CONSOLE", "MAX", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "DIAGNOSTIC-EVENT-INFO-NEEDS", "DIAGNOSTIC-CONNECTED-INDICATOR", "ROTATE-90-CCW", "CALIBRATION-OFFLINE", "OBD-DRIVING-CYCLE", "RTPGE", "DATA-WRITE-COMPLETED-EVENT", "I-PDU-SENT-TO-IF", "EXTENDED", "CONSUMED-SERVICE-INSTANCE", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "RO", "DHCPV-6", "ON-ENTRY", "MEDIUM", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING"];

    /// derive an enum entry from an input string using a perfect hash function
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseEnumItemError> {
        static DISPLACEMENTS: [(u16, u16); 343] = [(0, 3), (0, 166), (0, 39), (0, 46), (0, 1476), (18, 1489), (0, 177), (0, 670), (17, 648), (0, 409), (0, 1), (0, 17), (2, 576), (0, 13), (3, 834), (0, 55), (0, 7), (0, 609), (0, 14), (1, 1077), (41, 1393), (1, 689), (17, 27), (0, 1), (10, 2392), (0, 0), (0, 57), (8, 529), (0, 1446), (1, 25), (0, 49), (0, 4), (16, 1622), (1, 2003), (4, 713), (1, 759), (0, 0), (0, 2), (0, 14), (0, 180), (0, 141), (0, 1817), (0, 231), (0, 69), (3, 904), (0, 18), (0, 181), (0, 0), (0, 305), (64, 1005), (0, 596), (0, 511), (0, 1618), (0, 1486), (0, 1), (0, 898), (1, 609), (9, 7), (0, 773), (0, 115), (0, 266), (0, 476), (0, 800), (64, 2361), (0, 22), (32, 1167), (0, 0), (0, 39), (0, 132), (0, 65), (0, 870), (53, 2012), (1, 980), (1, 1587), (101, 218), (0, 90), (45, 1995), (14, 1357), (0, 204), (0, 1754), (17, 1686), (0, 399), (0, 0), (0, 259), (0, 1508), (0, 625), (142, 2235), (62, 64), (0, 442), (0, 1865), (9, 1134), (0, 2), (0, 288), (0, 56), (1, 161), (2, 446), (0, 15), (0, 2), (2, 203), (0, 0), (1, 1846), (0, 1262), (0, 21), (7, 1285), (0, 418), (0, 266), (2, 1831), (0, 253), (25, 637), (11, 1818), (0, 231), (0, 8), (0, 1162), (1, 1257), (0, 180), (0, 196), (2, 2075), (1, 179), (0, 70), (0, 429), (4, 892), (20, 254), (3, 1573), (0, 455), (40, 175), (10, 2167), (9, 1171), (3, 1544), (7, 2274), (1, 901), (0, 641), (0, 71), (0, 167), (6, 1807), (0, 3), (25, 1488), (0, 46), (0, 279), (0, 747), (46, 19), (1, 1218), (0, 153), (0, 1), (3, 1154), (1, 0), (0, 1037), (0, 0), (31, 416), (0, 1156), (4, 1004), (0, 13), (0, 47), (0, 540), (0, 6), (8, 535), (0, 1367), (1, 606), (0, 797), (0, 587), (0, 2), (0, 28), (0, 14), (0, 5), (0, 802), (0, 1091), (0, 2), (0, 1027), (15, 1820), (0, 5), (0, 335), (2, 1804), (0, 68), (578, 1175), (0, 2), (0, 8), (0, 3), (0, 56), (53, 1067), (0, 16), (0, 0), (0, 143), (0, 74), (0, 1147), (0, 596), (0, 10), (62, 1982), (0, 0), (150, 572), (2, 166), (0, 409), (0, 149), (1, 997), (0, 30), (0, 0), (0, 2278), (2, 1867), (0, 39), (0, 1), (0, 34), (1, 1413), (0, 0), (0, 1119), (9, 403), (0, 375), (0, 1), (0, 146), (2, 2047), (0, 1743), (46, 2039), (0, 10), (38, 1468), (0, 139), (156, 963), (4, 1133), (36, 2337), (0, 1205), (376, 2141), (12, 930), (64, 2245), (0, 636), (0, 73), (460, 270), (0, 854), (4, 914), (0, 66), (1, 378), (0, 5), (0, 7), (0, 104), (0, 525), (4, 1730), (0, 1434), (0, 671), (0, 1), (54, 1088), (0, 3), (0, 239), (0, 11), (0, 378), (3, 344), (110, 876), (0, 13), (0, 9), (2, 1946), (226, 2034), (0, 1739), (0, 110), (37, 775), (2, 1842), (24, 357), (2, 1898), (2, 251), (9, 1539), (0, 6), (0, 8), (0, 380), (0, 54), (2, 1247), (0, 28), (0, 5), (2, 1478), (0, 19), (0, 1591), (0, 107), (0, 7), (0, 10), (0, 821), (233, 488), (0, 20), (0, 6), (272, 863), (0, 198), (2, 1735), (27, 2224), (1, 4), (0, 149), (26, 1882), (0, 36), (58, 1978), (0, 954), (0, 196), (25, 267), (0, 1054), (41, 791), (0, 6), (18, 1257), (4, 1869), (0, 443), (0, 138), (0, 7), (0, 62), (0, 78), (77, 1148), (4, 988), (12, 1228), (2, 133), (17, 779), (15, 213), (0, 0), (0, 9), (0, 13), (0, 6), (197, 68), (67, 1078), (49, 1038), (0, 77), (42, 1055), (13, 203), (34, 23), (0, 0), (0, 10), (0, 267), (72, 544), (0, 233), (0, 451), (0, 395), (52, 585), (36, 1857), (0, 180), (2, 1401), (0, 41), (0, 44), (0, 6), (306, 26), (106, 283), (0, 114), (0, 0), (2, 2313), (0, 16), (20, 142), (0, 21), (0, 1), (70, 191), (270, 1384), (73, 955), (0, 473), (86, 1455), (18, 2058), (0, 8), (0, 6), (0, 2), (129, 1463), (767, 231)];
        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 343) as usize];
        let item_idx = (d2 as u32).wrapping_add(f1.wrapping_mul(d1 as u32)).wrapping_add(f2) as usize % 2398;
        if EnumItem::STRING_TABLE[item_idx].as_bytes() != input {
            return Err(ParseEnumItemError);
        }
        Ok(unsafe {
            std::mem::transmute::<u16, Self>(item_idx as u16)
        })
    }

    /// get the str corresponding to an item
    ///
    /// The returned &str has static lifetime, becasue it is a reference to an entry in a list of constants
    pub fn to_str(&self) -> &'static str {
        EnumItem::STRING_TABLE[*self as usize]
    }
}

impl std::str::FromStr for EnumItem {
    type Err = ParseEnumItemError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(input.as_bytes())
    }
}

impl std::fmt::Debug for EnumItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(EnumItem::STRING_TABLE[*self as usize])
    }
}

impl std::fmt::Display for EnumItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(EnumItem::STRING_TABLE[*self as usize])
    }
}

