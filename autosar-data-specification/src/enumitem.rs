use crate::hashfunc;

#[derive(Debug)]
/// The error type ParseEnumItemError is returned when from_str() / parse() fails for EnumItem
pub struct ParseEnumItemError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
/// Enum of all possible enum values in Autosar
pub enum EnumItem {
    /// -500-MILES
    _500Miles                                                              = 1687,
    /// 1000BASE-T
    _1000baseT                                                             = 1763,
    /// 1000BASE-T1
    _1000baseT1                                                            = 364,
    /// 100BASE-T1
    _100baseT1                                                             = 1468,
    /// 100BASE-TX
    _100baseTx                                                             = 2268,
    /// 10BASE-T1S
    _10baseT1s                                                             = 315,
    /// AA
    Aa                                                                     = 304,
    /// AB
    Ab                                                                     = 1097,
    /// ABSTRACT
    Abstract                                                               = 1209,
    /// ABSTRACT-ACCESS-POINT
    AbstractAccessPoint                                                    = 2152,
    /// ABSTRACT-CAN-CLUSTER
    AbstractCanCluster                                                     = 1316,
    /// ABSTRACT-CAN-COMMUNICATION-CONNECTOR
    AbstractCanCommunicationConnector                                      = 2253,
    /// ABSTRACT-CAN-COMMUNICATION-CONTROLLER
    AbstractCanCommunicationController                                     = 2032,
    /// ABSTRACT-CAN-PHYSICAL-CHANNEL
    AbstractCanPhysicalChannel                                             = 928,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 298,
    /// ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS
    AbstractDoIpLogicAddressProps                                          = 1682,
    /// ABSTRACT-ETHERNET-FRAME
    AbstractEthernetFrame                                                  = 802,
    /// ABSTRACT-EVENT
    AbstractEvent                                                          = 144,
    /// ABSTRACT-EXECUTION-CONTEXT
    AbstractExecutionContext                                               = 2045,
    /// ABSTRACT-IAM-REMOTE-SUBJECT
    AbstractIamRemoteSubject                                               = 1026,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE
    AbstractImplementationDataType                                         = 1803,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT
    AbstractImplementationDataTypeElement                                  = 703,
    /// ABSTRACT-PROVIDED-PORT-PROTOTYPE
    AbstractProvidedPortPrototype                                          = 1959,
    /// ABSTRACT-RAW-DATA-STREAM-INTERFACE
    AbstractRawDataStreamInterface                                         = 1477,
    /// ABSTRACT-REQUIRED-PORT-PROTOTYPE
    AbstractRequiredPortPrototype                                          = 1606,
    /// ABSTRACT-SECURITY-EVENT-FILTER
    AbstractSecurityEventFilter                                            = 473,
    /// ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER
    AbstractSecurityIdsmInstanceFilter                                     = 1310,
    /// ABSTRACT-SERVICE-INSTANCE
    AbstractServiceInstance                                                = 1777,
    /// ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING
    AbstractSignalBasedToISignalTriggeringMapping                          = 2208,
    /// ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE
    AbstractSynchronizedTimeBaseInterface                                  = 1761,
    /// ACCEPT-ALL
    AcceptAll                                                              = 598,
    /// ACCEPT-CONFIGURED
    AcceptConfigured                                                       = 475,
    /// ACCES-PERRMISSION-SERVICE-CLASS
    AccesPerrmissionServiceClass                                           = 222,
    /// ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS
    AccessPermissionInstanceOverridesClass                                 = 1526,
    /// ACCESS-PERMISSION-SERVICE-CLASS
    AccessPermissionServiceClass                                           = 81,
    /// ACCESS-PERMISSION-SERVICE-INSTANCE
    AccessPermissionServiceInstance                                        = 875,
    /// ACK-WITH-RT
    AckWithRt                                                              = 1955,
    /// ACK-WITHOUT-RT
    AckWithoutRt                                                           = 1125,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 1432,
    /// ACL-OPERATION
    AclOperation                                                           = 156,
    /// ACL-PERMISSION
    AclPermission                                                          = 2252,
    /// ACL-ROLE
    AclRole                                                                = 471,
    /// ACTION
    Action                                                                 = 111,
    /// ACTION-ITEM
    ActionItem                                                             = 1042,
    /// ACTION-LIST
    ActionList                                                             = 125,
    /// ACTIVATE
    Activate                                                               = 887,
    /// ACTIVATION-AND-TRIGGER-UNICAST
    ActivationAndTriggerUnicast                                            = 1921,
    /// ACTIVATION-MULTICAST
    ActivationMulticast                                                    = 943,
    /// ACTIVATION-UNICAST
    ActivationUnicast                                                      = 2225,
    /// ACTIVE
    Active                                                                 = 1515,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 881,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 2241,
    /// ADAPTIVE-EVENT-RECEIVED
    AdaptiveEventReceived                                                  = 2238,
    /// ADAPTIVE-EVENT-SENT
    AdaptiveEventSent                                                      = 171,
    /// ADAPTIVE-FIELD-GETTER-CALLED
    AdaptiveFieldGetterCalled                                              = 1724,
    /// ADAPTIVE-FIELD-GETTER-COMPLETED
    AdaptiveFieldGetterCompleted                                           = 1605,
    /// ADAPTIVE-FIELD-NOTIFICATION-RECEIVED
    AdaptiveFieldNotificationReceived                                      = 715,
    /// ADAPTIVE-FIELD-NOTIFICATION-SENT
    AdaptiveFieldNotificationSent                                          = 459,
    /// ADAPTIVE-FIELD-SETTER-CALLED
    AdaptiveFieldSetterCalled                                              = 1361,
    /// ADAPTIVE-FIELD-SETTER-COMPLETED
    AdaptiveFieldSetterCompleted                                           = 85,
    /// ADAPTIVE-METHOD-CALL-RECEIVED
    AdaptiveMethodCallReceived                                             = 2166,
    /// ADAPTIVE-METHOD-CALLED
    AdaptiveMethodCalled                                                   = 675,
    /// ADAPTIVE-METHOD-RESPONSE-RECEIVED
    AdaptiveMethodResponseReceived                                         = 585,
    /// ADAPTIVE-METHOD-RESPONSE-SENT
    AdaptiveMethodResponseSent                                             = 1408,
    /// ADAPTIVE-MODULE-INSTANTIATION
    AdaptiveModuleInstantiation                                            = 1862,
    /// ADAPTIVE-PLATFORM-SERVICE-INSTANCE
    AdaptivePlatformServiceInstance                                        = 2024,
    /// ADAPTIVE-SERVICE-FIND-COMPLETED
    AdaptiveServiceFindCompleted                                           = 1843,
    /// ADAPTIVE-SERVICE-FIND-STARTED
    AdaptiveServiceFindStarted                                             = 1035,
    /// ADAPTIVE-SERVICE-OFFER-COMPLETED
    AdaptiveServiceOfferCompleted                                          = 1934,
    /// ADAPTIVE-SERVICE-OFFER-STARTED
    AdaptiveServiceOfferStarted                                            = 1866,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED
    AdaptiveServiceStopSubscriptionCompleted                               = 1715,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED
    AdaptiveServiceStopSubscriptionStarted                                 = 1503,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED
    AdaptiveServiceSubscriptionAcknowledgeCompleted                        = 433,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED
    AdaptiveServiceSubscriptionAcknowledgeStarted                          = 29,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED
    AdaptiveServiceSubscriptionCompleted                                   = 218,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED
    AdaptiveServiceSubscriptionStarted                                     = 1008,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 1807,
    /// ADDR-METHOD-SHORT-NAME
    AddrMethodShortName                                                    = 931,
    /// ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT
    AddrMethodShortNameAndAlignment                                        = 2311,
    /// AF
    Af                                                                     = 469,
    /// AFTER-SALES
    AfterSales                                                             = 821,
    /// AFTERMAKET
    Aftermaket                                                             = 539,
    /// AFTERMARKET
    Aftermarket                                                            = 361,
    /// AGE
    Age                                                                    = 655,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 787,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 1639,
    /// AGREED
    Agreed                                                                 = 206,
    /// AH
    Ah                                                                     = 673,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 389,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 299,
    /// ALL
    All                                                                    = 1967,
    /// ALL-16-BIT
    All16Bit                                                               = 1292,
    /// ALL-INDICES-DIFFERENT-ARRAY-SIZE
    AllIndicesDifferentArraySize                                           = 1373,
    /// ALL-INDICES-SAME-ARRAY-SIZE
    AllIndicesSameArraySize                                                = 112,
    /// ALL-SUPPORTED-DTCS
    AllSupportedDtcs                                                       = 1928,
    /// ALLOCATOR
    Allocator                                                              = 1173,
    /// ALTERNATING-8-BIT
    Alternating8Bit                                                        = 906,
    /// ALWAYS
    Always                                                                 = 702,
    /// AM
    Am                                                                     = 1424,
    /// AMBER-WARNING
    AmberWarning                                                           = 1939,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 1583,
    /// AND
    And                                                                    = 1708,
    /// ANY
    Any                                                                    = 1369,
    /// ANY-SEND-OPERATION
    AnySendOperation                                                       = 400,
    /// ANY-STANDARDIZED
    AnyStandardized                                                        = 412,
    /// AP
    Ap                                                                     = 1859,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 1404,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 1785,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 488,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 1565,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 66,
    /// API
    Api                                                                    = 283,
    /// API-BASED
    ApiBased                                                               = 520,
    /// API-USE
    ApiUse                                                                 = 333,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 1909,
    /// APPLICATION
    Application                                                            = 457,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 736,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 2008,
    /// APPLICATION-ARRAY-ELEMENT
    ApplicationArrayElement                                                = 700,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 1630,
    /// APPLICATION-ASSOC-MAP-ELEMENT
    ApplicationAssocMapElement                                             = 369,
    /// APPLICATION-COMPOSITE-DATA-TYPE
    ApplicationCompositeDataType                                           = 1707,
    /// APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE
    ApplicationCompositeElementDataPrototype                               = 1274,
    /// APPLICATION-DATA-TYPE
    ApplicationDataType                                                    = 1020,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 1900,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 501,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 954,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 191,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 1255,
    /// APPLICATION-ONLY
    ApplicationOnly                                                        = 1904,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 823,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 1122,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 1193,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 1871,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 407,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 1425,
    /// AR
    Ar                                                                     = 1770,
    /// AR--CLIENT--SERVER
    ArClientServer                                                         = 615,
    /// AR-ELEMENT
    ArElement                                                              = 1146,
    /// AR-PACKAGE
    ArPackage                                                              = 740,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 1340,
    /// ARBITRATION
    Arbitration                                                            = 1760,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 1870,
    /// ARRAY
    Array                                                                  = 350,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 2321,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 288,
    /// AS
    As                                                                     = 945,
    /// AS-IS
    AsIs                                                                   = 1062,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 2172,
    /// ASYMMETRIC-FROM-BYTE-ARRAY
    AsymmetricFromByteArray                                                = 1490,
    /// ASYMMETRIC-TO-BYTE-ARRAY
    AsymmetricToByteArray                                                  = 2294,
    /// ASYNCHRONOUS
    Asynchronous                                                           = 390,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 1802,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 2094,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 1632,
    /// ATOMIC-SW-COMPONENT-TYPE
    AtomicSwComponentType                                                  = 616,
    /// ATP-BLUEPRINT
    AtpBlueprint                                                           = 595,
    /// ATP-BLUEPRINTABLE
    AtpBlueprintable                                                       = 88,
    /// ATP-CLASSIFIER
    AtpClassifier                                                          = 1407,
    /// ATP-DEFINITION
    AtpDefinition                                                          = 957,
    /// ATP-FEATURE
    AtpFeature                                                             = 781,
    /// ATP-PROTOTYPE
    AtpPrototype                                                           = 1651,
    /// ATP-STRUCTURE-ELEMENT
    AtpStructureElement                                                    = 2111,
    /// ATP-TYPE
    AtpType                                                                = 676,
    /// ATTRIBUTE-TAILORING
    AttributeTailoring                                                     = 685,
    /// AUTHENTICATE
    Authenticate                                                           = 772,
    /// AUTO
    Auto                                                                   = 935,
    /// AUTO-IP
    AutoIp                                                                 = 1854,
    /// AUTO-IP--DOIP
    AutoIpDoip                                                             = 2189,
    /// AUTO-IPDHCPV-4
    AutoIpdhcpv4                                                           = 2063,
    /// AUTONOMOUS
    Autonomous                                                             = 1392,
    /// AUTOSAR-DATA-PROTOTYPE
    AutosarDataPrototype                                                   = 1539,
    /// AUTOSAR-DATA-TYPE
    AutosarDataType                                                        = 2081,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 548,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 212,
    /// AVB--IEEE-802--1-AS
    AvbIeee8021As                                                          = 442,
    /// AY
    Ay                                                                     = 2251,
    /// AZ
    Az                                                                     = 312,
    /// BA
    Ba                                                                     = 338,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 1415,
    /// BASE-T
    BaseT                                                                  = 800,
    /// BASE-TYPE
    BaseType                                                               = 1697,
    /// BASIC-SOFTWARE-MODE-MANAGER
    BasicSoftwareModeManager                                               = 784,
    /// BE
    Be                                                                     = 1489,
    /// BG
    Bg                                                                     = 119,
    /// BH
    Bh                                                                     = 1798,
    /// BI
    Bi                                                                     = 2206,
    /// BIDIRECTIONAL
    Bidirectional                                                          = 2255,
    /// BINARY-MANIFEST-ADDRESSABLE-OBJECT
    BinaryManifestAddressableObject                                        = 686,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 2143,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 1318,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 492,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 2370,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 574,
    /// BINARY-MANIFEST-RESOURCE
    BinaryManifestResource                                                 = 74,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 192,
    /// BLINK-MODE
    BlinkMode                                                              = 2284,
    /// BLINK-OR-CONTINUOUS-ON-MODE
    BlinkOrContinuousOnMode                                                = 1236,
    /// BLOCK-SOURCE
    BlockSource                                                            = 495,
    /// BLOCK-STATE
    BlockState                                                             = 1553,
    /// BLUEPRINT-DERIVATION-TIME
    BlueprintDerivationTime                                                = 1470,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 1204,
    /// BMP
    Bmp                                                                    = 90,
    /// BN
    Bn                                                                     = 1643,
    /// BO
    Bo                                                                     = 872,
    /// BOLD
    Bold                                                                   = 2247,
    /// BOLDITALIC
    Bolditalic                                                             = 995,
    /// BONJOUR
    Bonjour                                                                = 1363,
    /// BOTTOM
    Bottom                                                                 = 9,
    /// BR
    Br                                                                     = 2201,
    /// BREAK
    Break                                                                  = 2213,
    /// BRIEF
    Brief                                                                  = 1972,
    /// BRIEF-BYPASSING-FILTERS
    BriefBypassingFilters                                                  = 1756,
    /// BROAD-R-REACH
    BroadRReach                                                            = 1004,
    /// BSW
    Bsw                                                                    = 472,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 1607,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 2171,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 738,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 733,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 1430,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 1326,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 279,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 2026,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 460,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 1138,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 1916,
    /// BSW-EVENT
    BswEvent                                                               = 1388,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 176,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 698,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 909,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 1213,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 2320,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 645,
    /// BSW-M-ENTRY-CALL-RETURNED
    BswMEntryCallReturned                                                  = 484,
    /// BSW-M-ENTRY-CALLED
    BswMEntryCalled                                                        = 52,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 1642,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 832,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 1855,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 849,
    /// BSW-MODULE-CALL-POINT
    BswModuleCallPoint                                                     = 2361,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 2250,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 1666,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 783,
    /// BSW-MODULE-ENTITY
    BswModuleEntity                                                        = 419,
    /// BSW-MODULE-ENTITY-ACTIVATED
    BswModuleEntityActivated                                               = 38,
    /// BSW-MODULE-ENTITY-STARTED
    BswModuleEntityStarted                                                 = 519,
    /// BSW-MODULE-ENTITY-TERMINATED
    BswModuleEntityTerminated                                              = 1560,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 380,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 629,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 1220,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 1389,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 619,
    /// BSW-SCHEDULE-EVENT
    BswScheduleEvent                                                       = 2325,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 1737,
    /// BSW-SERVICE-DEPENDENCY-IDENT
    BswServiceDependencyIdent                                              = 143,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 2060,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 1387,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 1733,
    /// BUILD
    Build                                                                  = 1104,
    /// BUILD-ACTION
    BuildAction                                                            = 1525,
    /// BUILD-ACTION-ENTITY
    BuildActionEntity                                                      = 759,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 287,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 541,
    /// BUILD-TYPE-DEBUG
    BuildTypeDebug                                                         = 155,
    /// BUILD-TYPE-RELEASE
    BuildTypeRelease                                                       = 795,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 2188,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 1402,
    /// BUS-MIRROR-CHANNEL-MAPPING
    BusMirrorChannelMapping                                                = 2035,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 249,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 2126,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 1917,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 1013,
    /// C
    C                                                                      = 430,
    /// CA
    Ca                                                                     = 1675,
    /// CALCULATED
    Calculated                                                             = 2266,
    /// CALIBRATION-OFFLINE
    CalibrationOffline                                                     = 2385,
    /// CALIBRATION-ONLINE
    CalibrationOnline                                                      = 831,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 970,
    /// CALIBRATION-VARIABLES
    CalibrationVariables                                                   = 1914,
    /// CALLBACK
    Callback                                                               = 1865,
    /// CALLOUT
    Callout                                                                = 1622,
    /// CALPRM
    Calprm                                                                 = 718,
    /// CAN-20
    Can20                                                                  = 104,
    /// CAN-BE-TERMINATED
    CanBeTerminated                                                        = 780,
    /// CAN-BE-TERMINATED-AND-RESTARTED
    CanBeTerminatedAndRestarted                                            = 1270,
    /// CAN-CLUSTER
    CanCluster                                                             = 1678,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 1890,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 1513,
    /// CAN-FD
    CanFd                                                                  = 734,
    /// CAN-FRAME
    CanFrame                                                               = 1886,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 1963,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 511,
    /// CAN-NM-NODE
    CanNmNode                                                              = 1185,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 1520,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 310,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 455,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 546,
    /// CAN-TP-NODE
    CanTpNode                                                              = 2257,
    /// CANCEL
    Cancel                                                                 = 2375,
    /// CAPTION
    Caption                                                                = 596,
    /// CAPTURE-ASYNCHRONOUS-TO-REPORTING
    CaptureAsynchronousToReporting                                         = 2,
    /// CAPTURE-ASYNCHRONOUSLY-TO-REPORTING
    CaptureAsynchronouslyToReporting                                       = 1043,
    /// CAPTURE-SYNCHRONOUS-TO-REPORTING
    CaptureSynchronousToReporting                                          = 1975,
    /// CAPTURE-SYNCHRONOUSLY-TO-REPORTING
    CaptureSynchronouslyToReporting                                        = 1839,
    /// CAT-1
    Cat1                                                                   = 1679,
    /// CAT-2
    Cat2                                                                   = 1098,
    /// CAUTION
    Caution                                                                = 406,
    /// CENTER
    Center                                                                 = 1399,
    /// CHANNEL-A
    ChannelA                                                               = 1453,
    /// CHANNEL-B
    ChannelB                                                               = 1118,
    /// CHAPTER
    Chapter                                                                = 53,
    /// CHECK-AT-NEXT-HALT
    CheckAtNextHalt                                                        = 829,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 1774,
    /// CIRCLE
    Circle                                                                 = 648,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 1025,
    /// CLASSIC
    Classic                                                                = 2249,
    /// CLEAR
    Clear                                                                  = 1555,
    /// CLEAR-ALL-DTCS
    ClearAllDtcs                                                           = 713,
    /// CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    ClearDynamicallyDefineDataIdentifier                                   = 187,
    /// CLIENT-AUTHENTICATE
    ClientAuthenticate                                                     = 1054,
    /// CLIENT-DECRYPT
    ClientDecrypt                                                          = 1189,
    /// CLIENT-ENCRYPT
    ClientEncrypt                                                          = 73,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 624,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 514,
    /// CLIENT-MAC-GENERATE
    ClientMacGenerate                                                      = 1232,
    /// CLIENT-MAC-VERIFY
    ClientMacVerify                                                        = 627,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 850,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 1046,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 1418,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 898,
    /// CLIENT-VERIFY
    ClientVerify                                                           = 1495,
    /// CLOSED
    Closed                                                                 = 896,
    /// CO
    Co                                                                     = 1103,
    /// CODE
    Code                                                                   = 1868,
    /// CODE-GENERATION-TIME
    CodeGenerationTime                                                     = 1362,
    /// CODEGENERATION
    Codegeneration                                                         = 752,
    /// COLDSTART
    Coldstart                                                              = 2343,
    /// COLLECTABLE-ELEMENT
    CollectableElement                                                     = 314,
    /// COLLECTION
    Collection                                                             = 1931,
    /// COM-AXIS
    ComAxis                                                                = 1147,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 1330,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 1958,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 371,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 2099,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 2279,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 1124,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 425,
    /// COM-GRANT
    ComGrant                                                               = 953,
    /// COM-GRANT-DESIGN
    ComGrantDesign                                                         = 1920,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 1156,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 1074,
    /// COM-MANAGER
    ComManager                                                             = 1541,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 503,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 958,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 368,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 913,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 496,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 1835,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 639,
    /// COMM-CONNECTOR-PORT
    CommConnectorPort                                                      = 1944,
    /// COMMAND-LINE-LONG-FORM
    CommandLineLongForm                                                    = 2372,
    /// COMMAND-LINE-SHORT-FORM
    CommandLineShortForm                                                   = 941,
    /// COMMAND-LINE-SIMPLE-FORM
    CommandLineSimpleForm                                                  = 2280,
    /// COMMON
    Common                                                                 = 401,
    /// COMMUNICATION-CLUSTER
    CommunicationCluster                                                   = 861,
    /// COMMUNICATION-CONNECTOR
    CommunicationConnector                                                 = 1646,
    /// COMMUNICATION-CONTROLLER
    CommunicationController                                                = 1623,
    /// COMMUNICATION-INTER-ECU
    CommunicationInterEcu                                                  = 132,
    /// COMMUNICATION-INTRA-PARTITION
    CommunicationIntraPartition                                            = 1376,
    /// COMPILE
    Compile                                                                = 1283,
    /// COMPILER
    Compiler                                                               = 1650,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 609,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 2059,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 1225,
    /// COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING
    CompositionPortToExecutablePortMapping                                 = 1143,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 424,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 1653,
    /// COMPU-METHOD
    CompuMethod                                                            = 963,
    /// COM_AXIS
    Comaxis                                                                = 1896,
    /// CONCRETE
    Concrete                                                               = 221,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 710,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 1753,
    /// CONDITIONAL
    Conditional                                                            = 1993,
    /// CONFIG-DATA
    ConfigData                                                             = 2129,
    /// CONFIGURED
    Configured                                                             = 791,
    /// CONFIRMED
    Confirmed                                                              = 211,
    /// CONFIRMED-DTC-BIT
    ConfirmedDtcBit                                                        = 2154,
    /// CONNECT
    Connect                                                                = 1188,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 1667,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 2184,
    /// CONSOLE
    Console                                                                = 2379,
    /// CONST
    Const                                                                  = 922,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 130,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 1461,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 1740,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 1092,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 1997,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 2391,
    /// CONSUMER
    Consumer                                                               = 815,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 1203,
    /// CONTINUE-AT-IT-POSITION
    ContinueAtItPosition                                                   = 205,
    /// CONTINUOUS-ON-MODE
    ContinuousOnMode                                                       = 1056,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 719,
    /// COUPLING-PORT
    CouplingPort                                                           = 2366,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 2177,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 907,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 2110,
    /// COUPLING-PORT-STRUCTURAL-ELEMENT
    CouplingPortStructuralElement                                          = 1580,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 2016,
    /// CP
    Cp                                                                     = 959,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 577,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 2240,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 1451,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 623,
    /// CP-SOFTWARE-CLUSTER-RESOURCE
    CpSoftwareClusterResource                                              = 198,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 1385,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 1864,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 2374,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 1508,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 381,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 1366,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 2291,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 166,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 989,
    /// CPP
    Cpp                                                                    = 1333,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 2119,
    /// CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET
    CppImplementationDataTypeContextTarget                                 = 536,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 2381,
    /// CRC-IGNORED
    CrcIgnored                                                             = 1152,
    /// CRC-NOT-SUPPORTED
    CrcNotSupported                                                        = 223,
    /// CRC-NOT-VALIDATED
    CrcNotValidated                                                        = 1884,
    /// CRC-OPTIONAL
    CrcOptional                                                            = 2102,
    /// CRC-SUPPORTED
    CrcSupported                                                           = 694,
    /// CRC-VALIDATED
    CrcValidated                                                           = 923,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 1727,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 882,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 666,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 1656,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 2354,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 722,
    /// CRYPTO-INTERFACE
    CryptoInterface                                                        = 1123,
    /// CRYPTO-JOB
    CryptoJob                                                              = 2065,
    /// CRYPTO-KEY-MANAGEMENT
    CryptoKeyManagement                                                    = 262,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 583,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 427,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 1609,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 1849,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 1166,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 2118,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 1027,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 1636,
    /// CRYPTO-NEEDS
    CryptoNeeds                                                            = 1259,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 2077,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 516,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 1635,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 1260,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 751,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 1160,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 656,
    /// CRYPTO-SERVICE-MANAGER
    CryptoServiceManager                                                   = 529,
    /// CRYPTO-SERVICE-MAPPING
    CryptoServiceMapping                                                   = 582,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 31,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 1246,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 1814,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 871,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 2260,
    /// CS
    Cs                                                                     = 1832,
    /// CURVE-AXIS
    CurveAxis                                                              = 1445,
    /// CURVE_AXIS
    Curveaxis                                                              = 35,
    /// CUSTOM
    Custom                                                                 = 481,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 1747,
    /// CVC
    Cvc                                                                    = 1109,
    /// CY
    Cy                                                                     = 717,
    /// CYCLE-REPETITION-1
    CycleRepetition1                                                       = 1030,
    /// CYCLE-REPETITION-10
    CycleRepetition10                                                      = 2331,
    /// CYCLE-REPETITION-16
    CycleRepetition16                                                      = 640,
    /// CYCLE-REPETITION-2
    CycleRepetition2                                                       = 1906,
    /// CYCLE-REPETITION-20
    CycleRepetition20                                                      = 978,
    /// CYCLE-REPETITION-32
    CycleRepetition32                                                      = 745,
    /// CYCLE-REPETITION-4
    CycleRepetition4                                                       = 1625,
    /// CYCLE-REPETITION-40
    CycleRepetition40                                                      = 1083,
    /// CYCLE-REPETITION-5
    CycleRepetition5                                                       = 263,
    /// CYCLE-REPETITION-50
    CycleRepetition50                                                      = 1720,
    /// CYCLE-REPETITION-64
    CycleRepetition64                                                      = 1195,
    /// CYCLE-REPETITION-8
    CycleRepetition8                                                       = 170,
    /// CYCLIC
    Cyclic                                                                 = 257,
    /// CYCLIC-AND-ON-CHANGE
    CyclicAndOnChange                                                      = 60,
    /// DA
    Da                                                                     = 576,
    /// DATA-CONSTR
    DataConstr                                                             = 289,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 572,
    /// DATA-FORMAT-ELEMENT-REFERENCE
    DataFormatElementReference                                             = 1729,
    /// DATA-FORMAT-ELEMENT-SCOPE
    DataFormatElementScope                                                 = 1504,
    /// DATA-INTERFACE
    DataInterface                                                          = 2141,
    /// DATA-PROTOTYPE
    DataPrototype                                                          = 78,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 1556,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 824,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 65,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 634,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 653,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 386,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 929,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 2388,
    /// DCM-I-PDU
    DcmIPdu                                                                = 1287,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 551,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 358,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 610,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 1170,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 878,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 2285,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 1115,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 2062,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 828,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 604,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 534,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 2084,
    /// DE
    De                                                                     = 1126,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 56,
    /// DEBOUNCE-DATA
    DebounceData                                                           = 1365,
    /// DEBUG
    Debug                                                                  = 1401,
    /// DECREASING
    Decreasing                                                             = 2161,
    /// DEDICATED
    Dedicated                                                              = 1099,
    /// DEF-ITEM
    DefItem                                                                = 1439,
    /// DEFAULT
    Default                                                                = 1751,
    /// DEFAULT-ERROR-TRACER
    DefaultErrorTracer                                                     = 1478,
    /// DEFAULT-IF-REVISION-UPDATE
    DefaultIfRevisionUpdate                                                = 163,
    /// DEFAULT-IF-UNDEFINED
    DefaultIfUndefined                                                     = 789,
    /// DEFAULT-MODE
    DefaultMode                                                            = 949,
    /// DEFAULT-TRACE-STATE-DISABLED
    DefaultTraceStateDisabled                                              = 2036,
    /// DEFAULT-TRACE-STATE-ENABLED
    DefaultTraceStateEnabled                                               = 605,
    /// DEFAULT-TRIGGER
    DefaultTrigger                                                         = 2353,
    /// DEFERRED
    Deferred                                                               = 543,
    /// DEFICIT-ROUND-ROBIN
    DeficitRoundRobin                                                      = 1127,
    /// DEFINE-BY-IDENTIFIER
    DefineByIdentifier                                                     = 1086,
    /// DEFINE-BY-MEMORY-ADDRESS
    DefineByMemoryAddress                                                  = 1961,
    /// DEFLATE
    Deflate                                                                = 1403,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 402,
    /// DELETE
    Delete                                                                 = 1181,
    /// DEPENDANT
    Dependant                                                              = 714,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 388,
    /// DERIVED-FROM
    DerivedFrom                                                            = 1573,
    /// DESCENDANT
    Descendant                                                             = 1360,
    /// DESELECTED
    Deselected                                                             = 1352,
    /// DETAILED
    Detailed                                                               = 2038,
    /// DETAILED-BYPASSING-FILTERS
    DetailedBypassingFilters                                               = 761,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 523,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 2248,
    /// DEVELOPMENT
    Development                                                            = 1952,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 1036,
    /// DEVELOPMENT-ERROR-TRACER
    DevelopmentErrorTracer                                                 = 1529,
    /// DHCPV-4
    Dhcpv4                                                                 = 154,
    /// DHCPV-6
    Dhcpv6                                                                 = 2394,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 1514,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 444,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 766,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 1237,
    /// DIAG-REQUEST
    DiagRequest                                                            = 2286,
    /// DIAG-RESPONSE
    DiagResponse                                                           = 1905,
    /// DIAGNOSTIC-ABSTRACT-ALIAS-EVENT
    DiagnosticAbstractAliasEvent                                           = 1659,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER
    DiagnosticAbstractDataIdentifier                                       = 697,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE
    DiagnosticAbstractDataIdentifierInterface                              = 2303,
    /// DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE
    DiagnosticAbstractRoutineInterface                                     = 1544,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 1586,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 13,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 606,
    /// DIAGNOSTIC-AUTHENTICATION
    DiagnosticAuthentication                                               = 1941,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 1263,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 1411,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 228,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 632,
    /// DIAGNOSTIC-CAPABILITY-ELEMENT
    DiagnosticCapabilityElement                                            = 2145,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 2194,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 2018,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 2293,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 1230,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 465,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 2015,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 1738,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 1214,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 950,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 1306,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 570,
    /// DIAGNOSTIC-COMMON-ELEMENT
    DiagnosticCommonElement                                                = 44,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER
    DiagnosticCommunicationManager                                         = 2262,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 2282,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 504,
    /// DIAGNOSTIC-CONDITION
    DiagnosticCondition                                                    = 247,
    /// DIAGNOSTIC-CONDITION-GROUP
    DiagnosticConditionGroup                                               = 306,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 97,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 2383,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 1055,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 581,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 399,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 1523,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 2115,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 1620,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 151,
    /// DIAGNOSTIC-DATA-BY-IDENTIFIER
    DiagnosticDataByIdentifier                                             = 1464,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 1766,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 2289,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 817,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 502,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 770,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 917,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 1604,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 1021,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 239,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 1192,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 243,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 947,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 115,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 1723,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 1319,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 571,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 1280,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 2086,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 1771,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 1450,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 944,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 1709,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 1395,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 2348,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 1320,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 699,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 1867,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 1498,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 2270,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 1860,
    /// DIAGNOSTIC-ENV-MODE-ELEMENT
    DiagnosticEnvModeElement                                               = 2085,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 708,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 866,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 2078,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 2382,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 1594,
    /// DIAGNOSTIC-EVENT-MANAGER
    DiagnosticEventManager                                                 = 774,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 47,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 37,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 2207,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 1059,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 2049,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 2239,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 2029,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 2392,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 584,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 1669,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 447,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 684,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 1140,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 2261,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 2185,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 1427,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 1786,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 340,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 1163,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 2039,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 1818,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 2195,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 1806,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 1603,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 468,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 39,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 357,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 1231,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 1992,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 24,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 858,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 1337,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 1677,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 1234,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 2052,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 867,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 654,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 40,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 1749,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 2046,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 2269,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 461,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 859,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 1253,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 1031,
    /// DIAGNOSTIC-LOG-AND-TRACE
    DiagnosticLogAndTrace                                                  = 1842,
    /// DIAGNOSTIC-MAPPING
    DiagnosticMapping                                                      = 174,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 936,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 55,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 1351,
    /// DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS
    DiagnosticMemoryAddressableRangeAccess                                 = 2242,
    /// DIAGNOSTIC-MEMORY-BY-ADDRESS
    DiagnosticMemoryByAddress                                              = 345,
    /// DIAGNOSTIC-MEMORY-DESTINATION
    DiagnosticMemoryDestination                                            = 68,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 1858,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 836,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 2090,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 1239,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 2355,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 2256,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 1746,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 731,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 2360,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 564,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 2100,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 182,
    /// DIAGNOSTIC-PORT-INTERFACE
    DiagnosticPortInterface                                                = 778,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 2287,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 2319,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 1206,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 2067,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 720,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 897,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 998,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 261,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 2275,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 775,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 1969,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 1444,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 235,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 1321,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 1112,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 966,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 2041,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 126,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 1322,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 1371,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 776,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 2104,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 1190,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 1516,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 691,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 2127,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 1113,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 726,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 1902,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 1979,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 256,
    /// DIAGNOSTIC-REQUEST-ROUTINE-RESULTS
    DiagnosticRequestRoutineResults                                        = 2101,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 1977,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 1587,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 336,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 1198,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 470,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 1485,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 747,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 377,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 2006,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 1759,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 2182,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 127,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 2265,
    /// DIAGNOSTIC-ROUTINE-SUBFUNCTION
    DiagnosticRoutineSubfunction                                           = 709,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 384,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 1954,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 506,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 1658,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 28,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 2070,
    /// DIAGNOSTIC-SERVICE-CLASS
    DiagnosticServiceClass                                                 = 1506,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 429,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 230,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 2158,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 2397,
    /// DIAGNOSTIC-SERVICE-INSTANCE
    DiagnosticServiceInstance                                              = 659,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 879,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 1978,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 1242,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 592,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 863,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 1518,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 1647,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 273,
    /// DIAGNOSTIC-START-ROUTINE
    DiagnosticStartRoutine                                                 = 1965,
    /// DIAGNOSTIC-STOP-ROUTINE
    DiagnosticStopRoutine                                                  = 1781,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 2133,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 1874,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 1852,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 450,
    /// DIAGNOSTIC-SW-MAPPING
    DiagnosticSwMapping                                                    = 1925,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 18,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 1218,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 431,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 729,
    /// DIAGNOSTIC-TROUBLE-CODE
    DiagnosticTroubleCode                                                  = 2276,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 1783,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 757,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 1562,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 1995,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 1400,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 1121,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 1177,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 45,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 363,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 1949,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 1652,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 1869,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 1629,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 1454,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 788,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 638,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 635,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 695,
    /// DISABLE
    Disable                                                                = 650,
    /// DLNA
    Dlna                                                                   = 229,
    /// DLT-APPLICATION
    DltApplication                                                         = 367,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 814,
    /// DLT-ARGUMENT
    DltArgument                                                            = 1015,
    /// DLT-CONTEXT
    DltContext                                                             = 199,
    /// DLT-ECU
    DltEcu                                                                 = 1384,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 324,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 1732,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 1457,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 1815,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 1383,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 677,
    /// DLT-MESSAGE
    DltMessage                                                             = 271,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 2156,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 91,
    /// DO-IP
    DoIp                                                                   = 841,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 5,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 98,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 679,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 1094,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 883,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 1382,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 1599,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 2088,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 1339,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 1676,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 2121,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 352,
    /// DO-IP-SERVICE-NEEDS
    DoIpServiceNeeds                                                       = 579,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 1817,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 1532,
    /// DOCUMENTATION
    Documentation                                                          = 1809,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 550,
    /// DOES-NOT-REPORT-EXECUTION-STATE
    DoesNotReportExecutionState                                            = 767,
    /// DOES-NOT-SUPPORT-BUFFER-LOCKING
    DoesNotSupportBufferLocking                                            = 813,
    /// DOES-NOT-USE-LOGGING
    DoesNotUseLogging                                                      = 2186,
    /// DOMAIN-PARTICIPANT-USER-DATA-QOS
    DomainParticipantUserDataQos                                           = 2044,
    /// DONT-INVALIDATE
    DontInvalidate                                                         = 2230,
    /// DROP
    Drop                                                                   = 1566,
    /// DROP-FRAME
    DropFrame                                                              = 1752,
    /// DROP-UNTAGGED
    DropUntagged                                                           = 477,
    /// DSA
    Dsa                                                                    = 421,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 2259,
    /// DYNAMIC-PART-TRIGGER
    DynamicPartTrigger                                                     = 1226,
    /// DZ
    Dz                                                                     = 852,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 320,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 1546,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 490,
    /// ECC
    Ecc                                                                    = 1017,
    /// ECU
    Ecu                                                                    = 1910,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 2153,
    /// ECU-INSTANCE
    EcuInstance                                                            = 164,
    /// ECU-MANAGER
    EcuManager                                                             = 1821,
    /// ECU-MAPPING
    EcuMapping                                                             = 1419,
    /// ECU-PARTITION
    EcuPartition                                                           = 1441,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 2244,
    /// ECU-TIMING
    EcuTiming                                                              = 670,
    /// ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF
    EcucAbstractExternalReferenceDef                                       = 1093,
    /// ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF
    EcucAbstractInternalReferenceDef                                       = 1834,
    /// ECUC-ABSTRACT-REFERENCE-DEF
    EcucAbstractReferenceDef                                               = 1974,
    /// ECUC-ABSTRACT-STRING-PARAM-DEF
    EcucAbstractStringParamDef                                             = 1791,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 552,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 238,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 2130,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 1324,
    /// ECUC-COMMON-ATTRIBUTES
    EcucCommonAttributes                                                   = 593,
    /// ECUC-CONTAINER-DEF
    EcucContainerDef                                                       = 525,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 1776,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 1782,
    /// ECUC-DEFINITION-ELEMENT
    EcucDefinitionElement                                                  = 1221,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 930,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 491,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 782,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 1356,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 765,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 1134,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 1039,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 1002,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 1238,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 1479,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 137,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 688,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 1161,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 1792,
    /// ECUC-PARAMETER-DEF
    EcucParameterDef                                                       = 443,
    /// ECUC-QUERY
    EcucQuery                                                              = 1131,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 2037,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 1602,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 1919,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 1681,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 1626,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 1665,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 1596,
    /// EDGE-NODE
    EdgeNode                                                               = 674,
    /// EL
    El                                                                     = 1158,
    /// EMISSION-RELATED-DTC
    EmissionRelatedDtc                                                     = 158,
    /// EN
    En                                                                     = 633,
    /// ENABLE
    Enable                                                                 = 1096,
    /// ENABLED
    Enabled                                                                = 1980,
    /// ENCRYPT-AND-SIGN
    EncryptAndSign                                                         = 1700,
    /// ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION
    EncryptAndSignWithOriginAuthentication                                 = 1222,
    /// ENCRYPTION
    Encryption                                                             = 753,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 101,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 1621,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 1168,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 1224,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 373,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 1895,
    /// ENHANCED
    Enhanced                                                               = 480,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 589,
    /// EO
    Eo                                                                     = 255,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 1467,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 392,
    /// EOC-EXECUTABLE-ENTITY-REF-ABSTRACT
    EocExecutableEntityRefAbstract                                         = 594,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 2163,
    /// EPS
    Eps                                                                    = 2211,
    /// EQUAL
    Equal                                                                  = 939,
    /// ERROR
    Error                                                                  = 2028,
    /// ERROR-CORRECTION
    ErrorCorrection                                                        = 2069,
    /// ERROR-DETECTION
    ErrorDetection                                                         = 1540,
    /// ERROR-TRACER
    ErrorTracer                                                            = 59,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 286,
    /// ES
    Es                                                                     = 968,
    /// ESP
    Esp                                                                    = 880,
    /// ET
    Et                                                                     = 1285,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 735,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 1881,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 904,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 80,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 1590,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 1024,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 106,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 482,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 1982,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 1718,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 771,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 704,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 1370,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 2278,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 1133,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 557,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 848,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 567,
    /// EU
    Eu                                                                     = 2048,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 1699,
    /// EVAP
    Evap                                                                   = 260,
    /// EVENT-ACCEPTANCE-DISABLED
    EventAcceptanceDisabled                                                = 2058,
    /// EVENT-ACCEPTANCE-ENABLED
    EventAcceptanceEnabled                                                 = 2107,
    /// EVENT-COMBINATION-ON-RETRIEVAL
    EventCombinationOnRetrieval                                            = 2131,
    /// EVENT-COMBINATION-ON-STORAGE
    EventCombinationOnStorage                                              = 1413,
    /// EVENT-HANDLER
    EventHandler                                                           = 2139,
    /// EVENT-MAPPING
    EventMapping                                                           = 2243,
    /// EVENT-STORAGE-DISABLED
    EventStorageDisabled                                                   = 1913,
    /// EVENT-STORAGE-ENABLED
    EventStorageEnabled                                                    = 2068,
    /// EVENT-TRIGGERING-CONSTRAINT
    EventTriggeringConstraint                                              = 730,
    /// EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE
    EventWindowCurrentAndFollowingCycle                                    = 1592,
    /// EVENT-WINDOW-CURRENT-CYCLE
    EventWindowCurrentCycle                                                = 1571,
    /// EVENT-WINDOW-INFINITE
    EventWindowInfinite                                                    = 2009,
    /// EXACT-OR-ANY-MINOR-VERSION
    ExactOrAnyMinorVersion                                                 = 1397,
    /// EXAMPLE
    Example                                                                = 1227,
    /// EXCLUDE-FROM-FLASH
    ExcludeFromFlash                                                       = 1079,
    /// EXCLUSIVE
    Exclusive                                                              = 1182,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 1069,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 1688,
    /// EXECUTABLE
    Executable                                                             = 1486,
    /// EXECUTABLE-ENTITY
    ExecutableEntity                                                       = 1661,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 1038,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 1205,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 1998,
    /// EXECUTE
    Execute                                                                = 278,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 1272,
    /// EXECUTION-TIME
    ExecutionTime                                                          = 1217,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 1922,
    /// EXERCISE
    Exercise                                                               = 530,
    /// EXPLICIT
    Explicit                                                               = 2168,
    /// EXTENDED
    Extended                                                               = 2390,
    /// EXTERNAL-REPLACEMENT
    ExternalReplacement                                                    = 1548,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 1476,
    /// EXTERNAL-TRIGGERING-POINT-IDENT
    ExternalTriggeringPointIdent                                           = 1469,
    /// FA
    Fa                                                                     = 665,
    /// FAILURE-AND-SUCCESS
    FailureAndSuccess                                                      = 1089,
    /// FAILURE-ONLY
    FailureOnly                                                            = 993,
    /// FALSE
    False                                                                  = 786,
    /// FAST-FLASHING-MODE
    FastFlashingMode                                                       = 1067,
    /// FATAL
    Fatal                                                                  = 61,
    /// FAULT
    Fault                                                                  = 927,
    /// FDC-THRESHOLD
    FdcThreshold                                                           = 498,
    /// FI
    Fi                                                                     = 2233,
    /// FIBEX-ELEMENT
    FibexElement                                                           = 237,
    /// FIELD
    Field                                                                  = 1172,
    /// FIELD-MAPPING
    FieldMapping                                                           = 2097,
    /// FILE
    File                                                                   = 1772,
    /// FILTERED
    Filtered                                                               = 2222,
    /// FINISH
    Finish                                                                 = 1463,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 1491,
    /// FIRST-CONTAINED-TRIGGER
    FirstContainedTrigger                                                  = 1637,
    /// FIRST-TO-SECOND
    FirstToSecond                                                          = 537,
    /// FIT-TO-PAGE
    FitToPage                                                              = 321,
    /// FIT-TO-TEXT
    FitToText                                                              = 322,
    /// FIX-AXIS
    FixAxis                                                                = 565,
    /// FIXED
    Fixed                                                                  = 696,
    /// FIXED-SIZE
    FixedSize                                                              = 1443,
    /// FIX_AXIS
    Fixaxis                                                                = 1778,
    /// FJ
    Fj                                                                     = 1736,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 1465,
    /// FLAT-MAP
    FlatMap                                                                = 862,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 1277,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 1617,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 1289,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 280,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 428,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 544,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 1422,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 1507,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 1951,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 1398,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 556,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 1844,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 890,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 712,
    /// FLOAT
    Float                                                                  = 1903,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 1496,
    /// FM-FEATURE
    FmFeature                                                              = 435,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 2224,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 1315,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 834,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 1301,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 1175,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 1757,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 303,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 1252,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 2226,
    /// FO
    Fo                                                                     = 895,
    /// FOR-ALL
    ForAll                                                                 = 1704,
    /// FORGET
    Forget                                                                 = 21,
    /// FORWARD-AS-IS
    ForwardAsIs                                                            = 173,
    /// FR
    Fr                                                                     = 1991,
    /// FRAME
    Frame                                                                  = 983,
    /// FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION
    FrameEthernetQueuedForTransmission                                     = 1298,
    /// FRAME-ETHERNET-RECEIVED-BY-IF
    FrameEthernetReceivedByIf                                              = 2120,
    /// FRAME-ETHERNET-RECEIVED-ON-BUS
    FrameEthernetReceivedOnBus                                             = 1088,
    /// FRAME-ETHERNET-SENT-ON-BUS
    FrameEthernetSentOnBus                                                 = 500,
    /// FRAME-PORT
    FramePort                                                              = 2033,
    /// FRAME-QUEUED-FOR-TRANSMISSION
    FrameQueuedForTransmission                                             = 1701,
    /// FRAME-RECEIVED-BY-IF
    FrameReceivedByIf                                                      = 196,
    /// FRAME-TRANSMITTED-ON-BUS
    FrameTransmittedOnBus                                                  = 2322,
    /// FRAME-TRIGGERING
    FrameTriggering                                                        = 888,
    /// FULL
    Full                                                                   = 1305,
    /// FULL-DUPLEX-MODE
    FullDuplexMode                                                         = 2080,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 16,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 669,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 2091,
    /// FUNCTION-INHIBITION-MANAGER
    FunctionInhibitionManager                                              = 940,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 1996,
    /// FUNCTIONAL
    Functional                                                             = 1378,
    /// FUNCTIONAL-ADDRESS
    FunctionalAddress                                                      = 919,
    /// FUNCTIONAL-CAN-FD
    FunctionalCanFd                                                        = 1892,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING
    FunctionalClusterInteractsWithFunctionalClusterMapping                 = 2057,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 990,
    /// FY
    Fy                                                                     = 1812,
    /// GA
    Ga                                                                     = 527,
    /// GATEWAY
    Gateway                                                                = 725,
    /// GD
    Gd                                                                     = 1554,
    /// GENERAL-PARAMETER
    GeneralParameter                                                       = 1741,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 2076,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 1394,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 2183,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 1828,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 2025,
    /// GET
    Get                                                                    = 798,
    /// GETTER
    Getter                                                                 = 1080,
    /// GETTER-SETTER
    GetterSetter                                                           = 644,
    /// GIF
    Gif                                                                    = 2210,
    /// GL
    Gl                                                                     = 394,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 1364,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 874,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 2272,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 590,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 2150,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 1937,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 614,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 1303,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 1001,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 588,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 1789,
    /// GLOBAL-TIME-MASTER
    GlobalTimeMaster                                                       = 1346,
    /// GLOBAL-TIME-SLAVE
    GlobalTimeSlave                                                        = 522,
    /// GN
    Gn                                                                     = 839,
    /// GRANT
    Grant                                                                  = 36,
    /// GRANT-DESIGN
    GrantDesign                                                            = 991,
    /// GROSS
    Gross                                                                  = 1293,
    /// GU
    Gu                                                                     = 17,
    /// GZIP
    Gzip                                                                   = 1162,
    /// HA
    Ha                                                                     = 1674,
    /// HALF-DUPLEX-MODE
    HalfDuplexMode                                                         = 1334,
    /// HARDWARE-TEST-MANAGER
    HardwareTestManager                                                    = 2047,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 2315,
    /// HEAD
    Head                                                                   = 1971,
    /// HEALTH-CHANNEL
    HealthChannel                                                          = 869,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 754,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 453,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 1569,
    /// HEAP-USAGE
    HeapUsage                                                              = 2122,
    /// HI
    Hi                                                                     = 796,
    /// HIERARCHICAL-EOC
    HierarchicalEoc                                                        = 141,
    /// HIGH
    High                                                                   = 810,
    /// HINT
    Hint                                                                   = 600,
    /// HOOK
    Hook                                                                   = 914,
    /// HOST-PORT
    HostPort                                                               = 573,
    /// HR
    Hr                                                                     = 1114,
    /// HU
    Hu                                                                     = 805,
    /// HUB
    Hub                                                                    = 1155,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 901,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 2167,
    /// HW-CATEGORY
    HwCategory                                                             = 2117,
    /// HW-DESCRIPTION-ENTITY
    HwDescriptionEntity                                                    = 129,
    /// HW-ELEMENT
    HwElement                                                              = 979,
    /// HW-PIN
    HwPin                                                                  = 1447,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 1381,
    /// HW-TYPE
    HwType                                                                 = 2200,
    /// HY
    Hy                                                                     = 348,
    /// I-4-G
    I4G                                                                    = 517,
    /// I-PDU
    IPdu                                                                   = 760,
    /// I-PDU-PORT
    IPduPort                                                               = 1517,
    /// I-PDU-RECEIVED-BY-COM
    IPduReceivedByCom                                                      = 301,
    /// I-PDU-SENT-TO-IF
    IPduSentToIf                                                           = 2389,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 202,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 1210,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 1108,
    /// I-SIGNAL
    ISignal                                                                = 478,
    /// I-SIGNAL-AVAILABLE-FOR-RTE
    ISignalAvailableForRte                                                 = 667,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 1145,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 276,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 8,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 305,
    /// I-SIGNAL-SENT-TO-COM
    ISignalSentToCom                                                       = 2306,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 965,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 1882,
    /// IA
    Ia                                                                     = 846,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 545,
    /// ICMP
    Icmp                                                                   = 840,
    /// IDENT-CAPTION
    IdentCaption                                                           = 1317,
    /// IDENTIFIABLE
    Identifiable                                                           = 1435,
    /// IDS-COMMON-ELEMENT
    IdsCommonElement                                                       = 2292,
    /// IDS-DESIGN
    IdsDesign                                                              = 1542,
    /// IDS-MAPPING
    IdsMapping                                                             = 494,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 877,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 270,
    /// IDS-PLATFORM-INSTANTIATION
    IdsPlatformInstantiation                                               = 2369,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 2179,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 1359,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 1837,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 236,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 62,
    /// IE
    Ie                                                                     = 2007,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 743,
    /// IEEE802-11P
    Ieee80211p                                                             = 1800,
    /// IEEE802-1AS
    Ieee8021as                                                             = 1712,
    /// IEEE802-1AS-AUTOSAR
    Ieee8021asAutosar                                                      = 2042,
    /// IGNITION
    Ignition                                                               = 803,
    /// IGNORE
    Ignore                                                                 = 649,
    /// IK
    Ik                                                                     = 1448,
    /// IMMEDIATE
    Immediate                                                              = 180,
    /// IMMEDIATELY
    Immediately                                                            = 1690,
    /// IMPLEMENTATION
    Implementation                                                         = 2082,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 976,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 48,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 1509,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 245,
    /// IMPLEMENTATION-PROPS
    ImplementationProps                                                    = 1345,
    /// IN
    In                                                                     = 2373,
    /// INCREASING
    Increasing                                                             = 1215,
    /// INDICATE
    Indicate                                                               = 1294,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 1095,
    /// INDIVIDUAL
    Individual                                                             = 1082,
    /// INFINITE
    Infinite                                                               = 1308,
    /// INFINITE-TIME-TO-RESPONSE
    InfiniteTimeToResponse                                                 = 2324,
    /// INFO
    Info                                                                   = 197,
    /// INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE
    InheritedFromArrayElementTypeSize                                      = 483,
    /// INIT-EVENT
    InitEvent                                                              = 2312,
    /// INLINE
    Inline                                                                 = 1286,
    /// INLINE-CONDITIONAL
    InlineConditional                                                      = 1799,
    /// INOUT
    Inout                                                                  = 1229,
    /// INSTALL
    Install                                                                = 1041,
    /// INSTANCE-ID
    InstanceId                                                             = 1110,
    /// INSTRUCTION
    Instruction                                                            = 2334,
    /// INTER-PARTITION-INTRA-ECU
    InterPartitionIntraEcu                                                 = 1223,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 920,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 2164,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 1645,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 625,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 937,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 2219,
    /// INTERRUPT
    Interrupt                                                              = 974,
    /// INTERRUPT-CAT-1
    InterruptCat1                                                          = 1813,
    /// INTERRUPT-CAT-2
    InterruptCat2                                                          = 2196,
    /// INTRUSION-DETECTION-SECURITY-MANAGEMENT
    IntrusionDetectionSecurityManagement                                   = 1267,
    /// INVALID
    Invalid                                                                = 1501,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 2004,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 1249,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 1662,
    /// IP-SEC-RULE
    IpSecRule                                                              = 1576,
    /// IPSEC
    Ipsec                                                                  = 641,
    /// IS
    Is                                                                     = 985,
    /// IS-EQUAL
    IsEqual                                                                = 1262,
    /// IS-EXPIRED
    IsExpired                                                              = 1960,
    /// IS-FAILED
    IsFailed                                                               = 1216,
    /// IS-GREATER-OR-EQUAL
    IsGreaterOrEqual                                                       = 1423,
    /// IS-GREATER-THAN
    IsGreaterThan                                                          = 1164,
    /// IS-GREATER-THAN-OR-EQUAL
    IsGreaterThanOrEqual                                                   = 2378,
    /// IS-LESS-OR-EQUAL
    IsLessOrEqual                                                          = 1296,
    /// IS-LESS-THAN
    IsLessThan                                                             = 1084,
    /// IS-LESS-THAN-OR-EQUAL
    IsLessThanOrEqual                                                      = 1459,
    /// IS-NOT-EQUAL
    IsNotEqual                                                             = 359,
    /// IS-NOT-RELEVANT
    IsNotRelevant                                                          = 1497,
    /// IS-OK
    IsOk                                                                   = 1649,
    /// IS-RELEVANT
    IsRelevant                                                             = 87,
    /// IS-STOPPED
    IsStopped                                                              = 1957,
    /// ISO
    Iso                                                                    = 1663,
    /// ISO-11992--4
    Iso119924                                                              = 1107,
    /// ISO-14229--1
    Iso142291                                                              = 1179,
    /// ISO-15031--6
    Iso150316                                                              = 426,
    /// ISO-6
    Iso6                                                                   = 924,
    /// IT
    It                                                                     = 1276,
    /// ITALIC
    Italic                                                                 = 1767,
    /// IW
    Iw                                                                     = 2136,
    /// J-1939
    J1939                                                                  = 32,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 2263,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 540,
    /// J-1939-DCM
    J1939Dcm                                                               = 618,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 2083,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 295,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 1261,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 2116,
    /// J-1939-REQUEST-MANAGER
    J1939RequestManager                                                    = 2146,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 1696,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 2220,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 204,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 2079,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 493,
    /// JA
    Ja                                                                     = 1492,
    /// JAVA
    Java                                                                   = 1375,
    /// JI
    Ji                                                                     = 448,
    /// JPG
    Jpg                                                                    = 1433,
    /// JUSTIFY
    Justify                                                                = 355,
    /// JW
    Jw                                                                     = 1927,
    /// KA
    Ka                                                                     = 356,
    /// KEEP
    Keep                                                                   = 2362,
    /// KEEP-EXISTING
    KeepExisting                                                           = 89,
    /// KEY-DERIVATION
    KeyDerivation                                                          = 436,
    /// KEY-STORAGE
    KeyStorage                                                             = 387,
    /// KEYWORD
    Keyword                                                                = 750,
    /// KEYWORD-SET
    KeywordSet                                                             = 622,
    /// KK
    Kk                                                                     = 220,
    /// KL
    Kl                                                                     = 281,
    /// KM
    Km                                                                     = 2051,
    /// KN
    Kn                                                                     = 2010,
    /// KO
    Ko                                                                     = 1550,
    /// KS
    Ks                                                                     = 1930,
    /// KU
    Ku                                                                     = 149,
    /// KY
    Ky                                                                     = 762,
    /// LA
    La                                                                     = 1695,
    /// LAND
    Land                                                                   = 414,
    /// LAST-FAILED
    LastFailed                                                             = 2013,
    /// LAST-IS-BEST
    LastIsBest                                                             = 99,
    /// LAST-MODE
    LastMode                                                               = 2190,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 1350,
    /// LEAF-OF-TARGET-CONTAINER
    LeafOfTargetContainer                                                  = 1406,
    /// LEFT
    Left                                                                   = 291,
    /// LEGACY
    Legacy                                                                 = 2113,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 2053,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 636,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 658,
    /// LIMIT-TO-PAGE
    LimitToPage                                                            = 1446,
    /// LIMIT-TO-TEXT
    LimitToText                                                            = 194,
    /// LIN-CLUSTER
    LinCluster                                                             = 297,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 1386,
    /// LIN-COMMUNICATION-CONTROLLER
    LinCommunicationController                                             = 732,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 1135,
    /// LIN-FRAME
    LinFrame                                                               = 84,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 1872,
    /// LIN-MASTER
    LinMaster                                                              = 1531,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 2176,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 404,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 1793,
    /// LIN-SLAVE
    LinSlave                                                               = 1989,
    /// LIN-SLAVE-CONFIG-IDENT
    LinSlaveConfigIdent                                                    = 1391,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 1331,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 1487,
    /// LIN-TP-NODE
    LinTpNode                                                              = 1377,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 434,
    /// LINK
    Link                                                                   = 418,
    /// LINK-LOCAL
    LinkLocal                                                              = 1538,
    /// LINK-LOCAL--DOIP
    LinkLocalDoip                                                          = 1032,
    /// LINK-TIME
    LinkTime                                                               = 2367,
    /// LINKER
    Linker                                                                 = 1888,
    /// LISTEN
    Listen                                                                 = 2202,
    /// LN
    Ln                                                                     = 1619,
    /// LO
    Lo                                                                     = 1264,
    /// LOCAL
    Local                                                                  = 2335,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 1119,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 918,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 2309,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 1111,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 1136,
    /// LOGICAL-AND
    LogicalAnd                                                             = 1545,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 329,
    /// LOGICAL-OR
    LogicalOr                                                              = 1911,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 1734,
    /// LONG-HEADER
    LongHeader                                                             = 797,
    /// LOW
    Low                                                                    = 997,
    /// LOWER-12-BIT
    Lower12Bit                                                             = 706,
    /// LOWER-8-BIT
    Lower8Bit                                                              = 385,
    /// LT
    Lt                                                                     = 134,
    /// LT-AFFECTS-PB
    LtAffectsPb                                                            = 721,
    /// LTS-13
    Lts13                                                                  = 994,
    /// LV
    Lv                                                                     = 1940,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 2356,
    /// MACHINE
    Machine                                                                = 285,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 1947,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 1692,
    /// MACHINE-TIMING
    MachineTiming                                                          = 1047,
    /// MACRO
    Macro                                                                  = 1641,
    /// MAINTENANCE-ONLY
    MaintenanceOnly                                                        = 515,
    /// MALFUNCTION
    Malfunction                                                            = 376,
    /// MANUFACTURING
    Manufacturing                                                          = 46,
    /// MAPPING-SCOPE-CORE
    MappingScopeCore                                                       = 693,
    /// MAPPING-SCOPE-ECU
    MappingScopeEcu                                                        = 1703,
    /// MAPPING-SCOPE-PARTITION
    MappingScopePartition                                                  = 462,
    /// MASEKD-NEW-EQUALS-MASKED-OLD
    MasekdNewEqualsMaskedOld                                               = 808,
    /// MASEKD-NEW-EQUALS-X
    MasekdNewEqualsX                                                       = 524,
    /// MASKED-NEW-DIFFERS-MASKED-OLD
    MaskedNewDiffersMaskedOld                                              = 1926,
    /// MASKED-NEW-DIFFERS-X
    MaskedNewDiffersX                                                      = 1211,
    /// MASKED-NEW-EQUALS-MASKED-OLD
    MaskedNewEqualsMaskedOld                                               = 1258,
    /// MASKED-NEW-EQUALS-X
    MaskedNewEqualsX                                                       = 842,
    /// MASTER
    Master                                                                 = 2142,
    /// MASTER-ECU
    MasterEcu                                                              = 398,
    /// MAX
    Max                                                                    = 2380,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 2277,
    /// MC-FUNCTION
    McFunction                                                             = 1344,
    /// MC-GROUP
    McGroup                                                                = 2040,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 2001,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 207,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 2134,
    /// MEASUREMENT-POINT
    MeasurementPoint                                                       = 837,
    /// MEDIUM
    Medium                                                                 = 2396,
    /// MEMORY-SECTION
    MemorySection                                                          = 1483,
    /// METHOD-MAPPING
    MethodMapping                                                          = 652,
    /// MG
    Mg                                                                     = 1254,
    /// MI
    Mi                                                                     = 785,
    /// MIDDLE
    Middle                                                                 = 1758,
    /// MIN
    Min                                                                    = 1796,
    /// MINIMUM-MINOR-VERSION
    MinimumMinorVersion                                                    = 15,
    /// MIXED
    Mixed                                                                  = 1889,
    /// MIXED-29-BIT
    Mixed29Bit                                                             = 1908,
    /// MK
    Mk                                                                     = 1091,
    /// ML
    Ml                                                                     = 1825,
    /// MN
    Mn                                                                     = 201,
    /// MO
    Mo                                                                     = 417,
    /// MODE-ACCESS-POINT-IDENT
    ModeAccessPointIdent                                                   = 1085,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 1672,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 325,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 227,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 344,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 1743,
    /// MODE-DECLARATION-REQUESTED
    ModeDeclarationRequested                                               = 1710,
    /// MODE-DECLARATION-SWITCH-COMPLETED
    ModeDeclarationSwitchCompleted                                         = 240,
    /// MODE-DECLARATION-SWITCH-INITIATED
    ModeDeclarationSwitchInitiated                                         = 0,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 2155,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 854,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 135,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 1611,
    /// MODE-TRANSITION
    ModeTransition                                                         = 1106,
    /// MODELED
    Modeled                                                                = 2340,
    /// MONITOR-MODE
    MonitorMode                                                            = 1581,
    /// MONO
    Mono                                                                   = 2368,
    /// MONOTONOUS
    Monotonous                                                             = 107,
    /// MOST-SIGNIFICANT-BYTE-FIRST
    MostSignificantByteFirst                                               = 1353,
    /// MOST-SIGNIFICANT-BYTE-LAST
    MostSignificantByteLast                                                = 75,
    /// MR
    Mr                                                                     = 2203,
    /// MS
    Ms                                                                     = 41,
    /// MT
    Mt                                                                     = 1295,
    /// MULTICORE-REENTRANT
    MulticoreReentrant                                                     = 2301,
    /// MULTILANGUAGE-REFERRABLE
    MultilanguageReferrable                                                = 1838,
    /// MULTIPLE
    Multiple                                                               = 342,
    /// MULTIPLE-OCCURRENCES
    MultipleOccurrences                                                    = 1745,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 1938,
    /// MY
    My                                                                     = 2304,
    /// N-PDU
    NPdu                                                                   = 1893,
    /// NA
    Na                                                                     = 1071,
    /// NAND
    Nand                                                                   = 1981,
    /// NE
    Ne                                                                     = 316,
    /// NET
    Net                                                                    = 2371,
    /// NETWORK
    Network                                                                = 847,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 2296,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 1178,
    /// NETWORK-REPRESENTATION-FROM-COM-SPEC
    NetworkRepresentationFromComSpec                                       = 2288,
    /// NEVER
    Never                                                                  = 933,
    /// NEW-IS-DIFFERENT
    NewIsDifferent                                                         = 756,
    /// NEW-IS-EQUAL
    NewIsEqual                                                             = 133,
    /// NEW-IS-GREATER
    NewIsGreater                                                           = 1347,
    /// NEW-IS-GREATER-OR-EQUAL
    NewIsGreaterOrEqual                                                    = 2030,
    /// NEW-IS-LESS
    NewIsLess                                                              = 264,
    /// NEW-IS-LESS-OR-EQUAL
    NewIsLessOrEqual                                                       = 1003,
    /// NEW-IS-OUTSIDE
    NewIsOutside                                                           = 946,
    /// NEW-IS-WITHIN
    NewIsWithin                                                            = 216,
    /// NEWLINE
    Newline                                                                = 2003,
    /// NEWLINE-IF-NECESSARY
    NewlineIfNecessary                                                     = 637,
    /// NFOLD
    Nfold                                                                  = 2132,
    /// NL
    Nl                                                                     = 528,
    /// NM-CLUSTER
    NmCluster                                                              = 95,
    /// NM-CONFIG
    NmConfig                                                               = 1744,
    /// NM-ECU
    NmEcu                                                                  = 818,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 1694,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 416,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 1452,
    /// NM-NODE
    NmNode                                                                 = 1300,
    /// NM-PDU
    NmPdu                                                                  = 1414,
    /// NO
    No                                                                     = 2365,
    /// NO-ACK
    NoAck                                                                  = 308,
    /// NO-AFFECT
    NoAffect                                                               = 967,
    /// NO-BOOT
    NoBoot                                                                 = 1693,
    /// NO-BREAK
    NoBreak                                                                = 1314,
    /// NO-DEFAULT
    NoDefault                                                              = 986,
    /// NO-FLOAT
    NoFloat                                                                = 2017,
    /// NO-HEADER
    NoHeader                                                               = 2246,
    /// NO-KEEP
    NoKeep                                                                 = 999,
    /// NO-MONOTONY
    NoMonotony                                                             = 2054,
    /// NO-NEWLINE
    NoNewline                                                              = 1050,
    /// NO-OBD-SUPPORT
    NoObdSupport                                                           = 339,
    /// NO-PGWIDE
    NoPgwide                                                               = 1628,
    /// NO-PROTECTION
    NoProtection                                                           = 1005,
    /// NO-RETURN-VALUE-PROVIDED
    NoReturnValueProvided                                                  = 962,
    /// NO-SEVERITY
    NoSeverity                                                             = 1455,
    /// NO-SHOW-ALIAS-NAME
    NoShowAliasName                                                        = 2297,
    /// NO-SHOW-CATEGORY
    NoShowCategory                                                         = 1307,
    /// NO-SHOW-CONTENT
    NoShowContent                                                          = 2095,
    /// NO-SHOW-LONG-NAME
    NoShowLongName                                                         = 812,
    /// NO-SHOW-NUMBER
    NoShowNumber                                                           = 1101,
    /// NO-SHOW-PAGE
    NoShowPage                                                             = 1271,
    /// NO-SHOW-SEE
    NoShowSee                                                              = 1689,
    /// NO-SHOW-SHORT-NAME
    NoShowShortName                                                        = 2124,
    /// NO-SHOW-TYPE
    NoShowType                                                             = 33,
    /// NO-SLOPPY
    NoSloppy                                                               = 1534,
    /// NO-STATUS-BYTE-CHANGE
    NoStatusByteChange                                                     = 2055,
    /// NO-STORE-EVENT
    NoStoreEvent                                                           = 1784,
    /// NO-SUPERVISION
    NoSupervision                                                          = 323,
    /// NO-SUPPORT
    NoSupport                                                              = 1728,
    /// NO-TRANSFORMER-ERROR-HANDLING
    NoTransformerErrorHandling                                             = 1608,
    /// NO-TRANSFORMER-STATUS-FORWARDING
    NoTransformerStatusForwarding                                          = 346,
    /// NO-TRUSTED-PLATFORM-SUPPORT
    NoTrustedPlatformSupport                                               = 902,
    /// NODE
    Node                                                                   = 1499,
    /// NOHREF
    Nohref                                                                 = 1820,
    /// NON-EMMISSION-RELATED-DTC
    NonEmmissionRelatedDtc                                                 = 172,
    /// NON-OS-MODULE-INSTANTIATION
    NonOsModuleInstantiation                                               = 889,
    /// NON-REENTRANT
    NonReentrant                                                           = 70,
    /// NON-VOLATILE
    NonVolatile                                                            = 1654,
    /// NON-VOLATILE-RAM-MANAGER
    NonVolatileRamManager                                                  = 1887,
    /// NONE
    None                                                                   = 948,
    /// NORMALFIXED
    Normalfixed                                                            = 626,
    /// NOT
    Not                                                                    = 2347,
    /// NOT-ACCESSIBLE
    NotAccessible                                                          = 441,
    /// NOT-AVAILABLE
    NotAvailable                                                           = 2192,
    /// NOT-DEFINED
    NotDefined                                                             = 742,
    /// NOT-EQUAL
    NotEqual                                                               = 971,
    /// NOT-SENT
    NotSent                                                                = 1819,
    /// NOT-TESTED
    NotTested                                                              = 2114,
    /// NOT-VALID
    NotValid                                                               = 1597,
    /// NOTHING
    Nothing                                                                = 716,
    /// NOTIFICATION
    Notification                                                           = 578,
    /// NTP--RFC-958
    NtpRfc958                                                              = 1019,
    /// NUMBER
    Number                                                                 = 2215,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 2022,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 2043,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 2337,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 2314,
    /// NV-RAM-MANAGER
    NvRamManager                                                           = 2264,
    /// OBD
    Obd                                                                    = 1187,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 961,
    /// OBD-DCY
    ObdDcy                                                                 = 1014,
    /// OBD-DRIVING-CYCLE
    ObdDrivingCycle                                                        = 2386,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 169,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 1291,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 2339,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 2169,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 1327,
    /// OBSERVER
    Observer                                                               = 1034,
    /// OBSERVER-BASED
    ObserverBased                                                          = 1615,
    /// OC
    Oc                                                                     = 678,
    /// OCCURENCE
    Occurence                                                              = 973,
    /// OEM-BOOT
    OemBoot                                                                = 912,
    /// OEM-BOOT-RESP-APP
    OemBootRespApp                                                         = 1524,
    /// OFF
    Off                                                                    = 2148,
    /// OFFSET
    Offset                                                                 = 409,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 2050,
    /// OM
    Om                                                                     = 1512,
    /// ON-CHANGE-OF-DATA-IDENTIFIER
    OnChangeOfDataIdentifier                                               = 23,
    /// ON-COMPARISON-OF-VALUES
    OnComparisonOfValues                                                   = 246,
    /// ON-DTC-STATUS-CHANGE
    OnDtcStatusChange                                                      = 2281,
    /// ON-ENTRY
    OnEntry                                                                = 2395,
    /// ON-EXIT
    OnExit                                                                 = 2151,
    /// ON-TRANSITION
    OnTransition                                                           = 327,
    /// ONE-EVERY-N
    OneEveryN                                                              = 664,
    /// ONLY-THIS-CYCLE-AND-READINESS
    OnlyThisCycleAndReadiness                                              = 2135,
    /// OPAQUE
    Opaque                                                                 = 1357,
    /// OPEN
    Open                                                                   = 1572,
    /// OPERATING-SYSTEM
    OperatingSystem                                                        = 835,
    /// OPERATION-CALL-RECEIVED
    OperationCallReceived                                                  = 1048,
    /// OPERATION-CALL-RESPONSE-RECEIVED
    OperationCallResponseReceived                                          = 1117,
    /// OPERATION-CALL-RESPONSE-SENT
    OperationCallResponseSent                                              = 671,
    /// OPERATION-CALLED
    OperationCalled                                                        = 597,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 1149,
    /// OPTIONS
    Options                                                                = 833,
    /// OR
    Or                                                                     = 1616,
    /// ORDINARY-EOC
    OrdinaryEoc                                                            = 1987,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 964,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 254,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 507,
    /// OTHER
    Other                                                                  = 876,
    /// OUT
    Out                                                                    = 1482,
    /// OVERRIDE
    Override                                                               = 234,
    /// OVERWRITE
    Overwrite                                                              = 445,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 1212,
    /// PA
    Pa                                                                     = 1390,
    /// PACKAGEABLE-ELEMENT
    PackageableElement                                                     = 318,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 2283,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 1474,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 2329,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 309,
    /// PARTIAL-NETWORK
    PartialNetwork                                                         = 2087,
    /// PARTITION
    Partition                                                              = 165,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 49,
    /// PASSIVE
    Passive                                                                = 2002,
    /// PASSTHROUGH
    Passthrough                                                            = 689,
    /// PAYLOAD-AS-ARRAY
    PayloadAsArray                                                         = 403,
    /// PAYLOAD-AS-POINTER-TO-ARRAY
    PayloadAsPointerToArray                                                = 1410,
    /// PC-AFFECTS-LT
    PcAffectsLt                                                            = 147,
    /// PC-AFFECTS-LT-AND-PB
    PcAffectsLtAndPb                                                       = 1863,
    /// PC-AFFECTS-PB
    PcAffectsPb                                                            = 1750,
    /// PDF
    Pdf                                                                    = 749,
    /// PDU
    Pdu                                                                    = 1183,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 1907,
    /// PDU-R
    PduR                                                                   = 809,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 1988,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 1861,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 512,
    /// PENDING
    Pending                                                                = 1853,
    /// PER-EXECUTABLE
    PerExecutable                                                          = 2352,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 602,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 1999,
    /// PERIODIC-RATE-FAST
    PeriodicRateFast                                                       = 1049,
    /// PERIODIC-RATE-MEDIUM
    PeriodicRateMedium                                                     = 2187,
    /// PERIODIC-RATE-SLOW
    PeriodicRateSlow                                                       = 975,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 1247,
    /// PERSISTENCY-DEPLOYMENT
    PersistencyDeployment                                                  = 1251,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT
    PersistencyDeploymentElement                                           = 1824,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 533,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 1923,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 768,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 167,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 231,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 1058,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 138,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 2137,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 1012,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 331,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 2092,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 2333,
    /// PERSISTENCY-INTERFACE
    PersistencyInterface                                                   = 1725,
    /// PERSISTENCY-INTERFACE-ELEMENT
    PersistencyInterfaceElement                                            = 886,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 71,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 1891,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 1348,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 1073,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 1685,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING
    PersistencyPortPrototypeToDeploymentMapping                            = 2236,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 2318,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 1634,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 157,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 1033,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE
    PersistencyRedundancyHandlingScopeDatabase                             = 7,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT
    PersistencyRedundancyHandlingScopeElement                              = 2349,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE
    PersistencyRedundancyHandlingScopeFile                                 = 150,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY
    PersistencyRedundancyHandlingScopeKey                                  = 2338,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE
    PersistencyRedundancyHandlingScopeStorage                              = 1191,
    /// PGWIDE
    Pgwide                                                                 = 290,
    /// PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE
    PhmAbstractRecoveryNotificationInterface                               = 1570,
    /// PHM-ACTION
    PhmAction                                                              = 250,
    /// PHM-ACTION-ITEM
    PhmActionItem                                                          = 1876,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 1282,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 1915,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 1563,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 232,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 727,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 1201,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 467,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 1157,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 1380,
    /// PHM-RULE
    PhmRule                                                                = 1045,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 302,
    /// PHM-SUPERVISION
    PhmSupervision                                                         = 2108,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 799,
    /// PHYSICAL
    Physical                                                               = 64,
    /// PHYSICAL-ADDRESS
    PhysicalAddress                                                        = 1547,
    /// PHYSICAL-CAN-FD
    PhysicalCanFd                                                          = 1063,
    /// PHYSICAL-CHANNEL
    PhysicalChannel                                                        = 938,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 42,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 1962,
    /// PL
    Pl                                                                     = 25,
    /// PLAIN
    Plain                                                                  = 2019,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 145,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 1472,
    /// PLATFORM-HEALTH-MANAGEMENT-INTERFACE
    PlatformHealthManagementInterface                                      = 1856,
    /// PLATFORM-MODULE-ENDPOINT-CONFIGURATION
    PlatformModuleEndpointConfiguration                                    = 1638,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 825,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 2056,
    /// PNC-MAPPING-IDENT
    PncMappingIdent                                                        = 1171,
    /// PNG
    Png                                                                    = 1561,
    /// POLY
    Poly                                                                   = 27,
    /// PORT
    Port                                                                   = 319,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 54,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 1686,
    /// PORT-GROUP
    PortGroup                                                              = 341,
    /// PORT-INTERFACE
    PortInterface                                                          = 1116,
    /// PORT-INTERFACE-DEFINITION
    PortInterfaceDefinition                                                = 1589,
    /// PORT-INTERFACE-MAPPING
    PortInterfaceMapping                                                   = 334,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 1764,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 2328,
    /// PORT-PROTOTYPE
    PortPrototype                                                          = 988,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 454,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 568,
    /// POST
    Post                                                                   = 942,
    /// POST-BUILD
    PostBuild                                                              = 1973,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 354,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 1159,
    /// POWER
    Power                                                                  = 375,
    /// POWER-WINDOW-TIME
    PowerWindowTime                                                        = 1711,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 124,
    /// PRE--R-4--2
    PreR42                                                                 = 19,
    /// PRE-COMPILE
    PreCompile                                                             = 739,
    /// PRE-COMPILE-TIME
    PreCompileTime                                                         = 2149,
    /// PRECONFIGURED-CONFIGURATION
    PreconfiguredConfiguration                                             = 446,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 1090,
    /// PRESENTATION-CONTINUOUS
    PresentationContinuous                                                 = 1601,
    /// PRESENTATION-DISCRETE
    PresentationDiscrete                                                   = 1273,
    /// PRESHARED-KEY-IDENTITY
    PresharedKeyIdentity                                                   = 1588,
    /// PRIMARY-ECU
    PrimaryEcu                                                             = 1633,
    /// PRIMITIVE
    Primitive                                                              = 561,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 2128,
    /// PRIO-OCC
    PrioOcc                                                                = 1612,
    /// PRIVATE-KEY
    PrivateKey                                                             = 1462,
    /// PROCESS
    Process                                                                = 758,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 1420,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 159,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 856,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 1875,
    /// PROCESS-IS-NOT-SELF-TERMINATING
    ProcessIsNotSelfTerminating                                            = 1880,
    /// PROCESS-IS-SELF-TERMINATING
    ProcessIsSelfTerminating                                               = 148,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 591,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 668,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 921,
    /// PROCESSING-STYLE-ASYNCHRONOUS
    ProcessingStyleAsynchronous                                            = 1705,
    /// PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR
    ProcessingStyleAsynchronousWithError                                   = 43,
    /// PROCESSING-STYLE-SYNCHRONOUS
    ProcessingStyleSynchronous                                             = 2363,
    /// PROCESSOR
    Processor                                                              = 908,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 1011,
    /// PRODUCER
    Producer                                                               = 219,
    /// PROTECT-LAMP
    ProtectLamp                                                            = 275,
    /// PROTECTED
    Protected                                                              = 2377,
    /// PROVIDED-AP-SERVICE-INSTANCE
    ProvidedApServiceInstance                                              = 1527,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 1466,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 439,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 2103,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 2218,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 982,
    /// PS
    Ps                                                                     = 1494,
    /// PSK
    Psk                                                                    = 1885,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 1795,
    /// PT
    Pt                                                                     = 1754,
    /// PTP--IEEE-1588--2002
    PtpIeee15882002                                                        = 826,
    /// PTP--IEEE-1588--2008
    PtpIeee15882008                                                        = 1808,
    /// PUBLIC-KEY
    PublicKey                                                              = 396,
    /// PUBLISHED-INFORMATION
    PublishedInformation                                                   = 792,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 916,
    /// PUT
    Put                                                                    = 365,
    /// QU
    Qu                                                                     = 2012,
    /// QUEUED
    Queued                                                                 = 1614,
    /// R-4--2
    R42                                                                    = 575,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 1894,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 213,
    /// RAW
    Raw                                                                    = 1240,
    /// RAW-DATA
    RawData                                                                = 1883,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 2072,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 1481,
    /// RAW-DATA-STREAM-GRANT
    RawDataStreamGrant                                                     = 2138,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 77,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 1077,
    /// RAW-DATA-STREAM-MAPPING
    RawDataStreamMapping                                                   = 690,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 103,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 79,
    /// REACTION
    Reaction                                                               = 1953,
    /// READ-ONLY
    ReadOnly                                                               = 116,
    /// READ-WRITE
    ReadWrite                                                              = 1202,
    /// REBOOT
    Reboot                                                                 = 1769,
    /// RECOMMENDED-CONFIGURATION
    RecommendedConfiguration                                               = 642,
    /// RECORD-VALUE-FIELD
    RecordValueField                                                       = 1984,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 1235,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 1564,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 1007,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 139,
    /// RECT
    Rect                                                                   = 179,
    /// RED-STOP-LAMP
    RedStopLamp                                                            = 2298,
    /// REDUNDANT
    Redundant                                                              = 683,
    /// REDUNDANT-PER-ELEMENT
    RedundantPerElement                                                    = 268,
    /// REDUNDANT-PER-KEY
    RedundantPerKey                                                        = 1801,
    /// REF-ALL
    RefAll                                                                 = 2034,
    /// REF-NON-STANDARD
    RefNonStandard                                                         = 1243,
    /// REF-NONE
    RefNone                                                                = 1775,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 464,
    /// REFERRABLE
    Referrable                                                             = 411,
    /// REGULAR
    Regular                                                                = 1655,
    /// REJECT
    Reject                                                                 = 1830,
    /// REMOVE
    Remove                                                                 = 2232,
    /// REPETITIVE-EOC
    RepetitiveEoc                                                          = 1640,
    /// REPLACE
    Replace                                                                = 566,
    /// REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE
    ReplaceByTimeoutSubstitutionValue                                      = 2299,
    /// REPORT
    Report                                                                 = 1879,
    /// REPORT-AFTER-INIT
    ReportAfterInit                                                        = 1248,
    /// REPORT-BEFORE-INIT
    ReportBeforeInit                                                       = 242,
    /// REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE
    ReportDtcRecordInformationOnDtcStatusChange                            = 870,
    /// REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE
    ReportMostRecentDtcOnStatusChange                                      = 1584,
    /// REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST
    ReportingInChronlogicalOrderOldestFirst                                = 1290,
    /// REPORTS-EXECUTION-STATE
    ReportsExecutionState                                                  = 2209,
    /// REQUEST
    Request                                                                = 1765,
    /// REQUEST-CALLBACK-TYPE-MANUFACTURER
    RequestCallbackTypeManufacturer                                        = 827,
    /// REQUEST-CALLBACK-TYPE-SUPPLIER
    RequestCallbackTypeSupplier                                            = 1023,
    /// REQUEST-NO-RETURN
    RequestNoReturn                                                        = 1811,
    /// REQUIRED-AP-SERVICE-INSTANCE
    RequiredApServiceInstance                                              = 563,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 168,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 405,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 317,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 681,
    /// REQUIRES-CALLBACK-EXECUTION
    RequiresCallbackExecution                                              = 1684,
    /// RES-AXIS
    ResAxis                                                                = 510,
    /// RESET-ECU
    ResetEcu                                                               = 1151,
    /// RESET-MACHINE
    ResetMachine                                                           = 69,
    /// RESET-MCU
    ResetMcu                                                               = 282,
    /// RESET-VM
    ResetVm                                                                = 960,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 2159,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 1968,
    /// RESPOND-AFTER-RESET
    RespondAfterReset                                                      = 474,
    /// RESPOND-BEFORE-RESET
    RespondBeforeReset                                                     = 252,
    /// RESPONSE
    Response                                                               = 1493,
    /// RESPONSE-SYNCHRONIZATION
    ResponseSynchronization                                                = 559,
    /// REST-ABSTRACT-ENDPOINT
    RestAbstractEndpoint                                                   = 1536,
    /// REST-ABSTRACT-NUMERICAL-PROPERTY-DEF
    RestAbstractNumericalPropertyDef                                       = 2064,
    /// REST-ABSTRACT-PROPERTY-DEF
    RestAbstractPropertyDef                                                = 1278,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 2316,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 1716,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 2332,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 1367,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 1358,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 328,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 932,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 1739,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 1100,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 905,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 2308,
    /// REST-PRIMITIVE-PROPERTY-DEF
    RestPrimitivePropertyDef                                               = 2217,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 1312,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 209,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 1841,
    /// RESTART
    Restart                                                                = 2223,
    /// RESTART-APPLICATION
    RestartApplication                                                     = 777,
    /// RES_AXIS
    Resaxis                                                                = 2175,
    /// RETURN-ON-EVENT-CLEARED
    ReturnOnEventCleared                                                   = 1044,
    /// RETURN-ON-EVENT-STOPPED
    ReturnOnEventStopped                                                   = 1683,
    /// RETURN-VALUE-PROVIDED
    ReturnValueProvided                                                    = 900,
    /// RIGHT
    Right                                                                  = 1706,
    /// RM
    Rm                                                                     = 1053,
    /// RN
    Rn                                                                     = 395,
    /// RO
    Ro                                                                     = 2393,
    /// ROLL-BACK
    RollBack                                                               = 560,
    /// ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE
    RootSwClusterDesignComponentPrototype                                  = 2342,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 1579,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 662,
    /// ROTATE-180
    Rotate180                                                              = 189,
    /// ROTATE-180-LIMIT-TO-TEXT
    Rotate180LimitToText                                                   = 1877,
    /// ROTATE-90-CCW
    Rotate90Ccw                                                            = 2384,
    /// ROTATE-90-CCW-FIT-TO-TEXT
    Rotate90CcwFitToText                                                   = 1022,
    /// ROTATE-90-CCW-LIMIT-TO-TEXT
    Rotate90CcwLimitToText                                                 = 391,
    /// ROTATE-90-CW
    Rotate90Cw                                                             = 136,
    /// ROTATE-90-CW-FIT-TO-TEXT
    Rotate90CwFitToText                                                    = 2326,
    /// ROTATE-90-CW-LIMIT-TO-TEXT
    Rotate90CwLimitToText                                                  = 1582,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 1600,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 801,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 2157,
    /// ROUTER
    Router                                                                 = 1266,
    /// ROUTER-ADVERTISEMENT
    RouterAdvertisement                                                    = 497,
    /// RPT-COMPONENT
    RptComponent                                                           = 466,
    /// RPT-CONTAINER
    RptContainer                                                           = 612,
    /// RPT-ENABLER-RAM
    RptEnablerRam                                                          = 1794,
    /// RPT-ENABLER-RAM-AND-ROM
    RptEnablerRamAndRom                                                    = 183,
    /// RPT-ENABLER-ROM
    RptEnablerRom                                                          = 1169,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 1942,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 2216,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 1680,
    /// RPT-LEVEL-1
    RptLevel1                                                              = 811,
    /// RPT-LEVEL-2
    RptLevel2                                                              = 1918,
    /// RPT-LEVEL-3
    RptLevel3                                                              = 2199,
    /// RPT-PROFILE
    RptProfile                                                             = 1219,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 437,
    /// RSA
    Rsa                                                                    = 131,
    /// RTE-EVENT
    RteEvent                                                               = 1105,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 1780,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 1950,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 2351,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 1816,
    /// RTPGE
    Rtpge                                                                  = 2387,
    /// RU
    Ru                                                                     = 1281,
    /// RULE
    Rule                                                                   = 2234,
    /// RUN-CONTINUOUS
    RunContinuous                                                          = 181,
    /// RUN-ONCE
    RunOnce                                                                = 555,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 1598,
    /// RUNNABLE-ENTITY-ACTIVATED
    RunnableEntityActivated                                                = 1313,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 1078,
    /// RUNNABLE-ENTITY-STARTED
    RunnableEntityStarted                                                  = 855,
    /// RUNNABLE-ENTITY-TERMINATED
    RunnableEntityTerminated                                               = 2376,
    /// RUNNABLE-ENTITY-VARIABLE-ACCESS
    RunnableEntityVariableAccess                                           = 2023,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 1983,
    /// RW
    Rw                                                                     = 2305,
    /// RX-TRIGGER
    RxTrigger                                                              = 1713,
    /// SA
    Sa                                                                     = 1985,
    /// SAE-J-1939--73
    SaeJ193973                                                             = 1936,
    /// SAE-J-2012--DA
    SaeJ2012Da                                                             = 2021,
    /// SAFETY
    Safety                                                                 = 2075,
    /// SATURATE
    Saturate                                                               = 410,
    /// SCHEDULE-VARIANT-1
    ScheduleVariant1                                                       = 438,
    /// SCHEDULE-VARIANT-2
    ScheduleVariant2                                                       = 724,
    /// SCHEDULE-VARIANT-3
    ScheduleVariant3                                                       = 1671,
    /// SCHEDULE-VARIANT-4
    ScheduleVariant4                                                       = 1899,
    /// SCHEDULE-VARIANT-5
    ScheduleVariant5                                                       = 1691,
    /// SCHEDULE-VARIANT-6
    ScheduleVariant6                                                       = 1976,
    /// SCHEDULE-VARIANT-7
    ScheduleVariant7                                                       = 1299,
    /// SCHEDULED
    Scheduled                                                              = 1627,
    /// SD
    Sd                                                                     = 873,
    /// SDG-ABSTRACT-FOREIGN-REFERENCE
    SdgAbstractForeignReference                                            = 120,
    /// SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE
    SdgAbstractPrimitiveAttribute                                          = 1567,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 476,
    /// SDG-ATTRIBUTE
    SdgAttribute                                                           = 2181,
    /// SDG-CAPTION
    SdgCaption                                                             = 1393,
    /// SDG-CLASS
    SdgClass                                                               = 2290,
    /// SDG-DEF
    SdgDef                                                                 = 2005,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 1174,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 413,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 1165,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 952,
    /// SDG-REFERENCE
    SdgReference                                                           = 2317,
    /// SDG-TAILORING
    SdgTailoring                                                           = 1833,
    /// SEARCH-FOR-ALL
    SearchForAll                                                           = 272,
    /// SEARCH-FOR-ALL-INSTANCES
    SearchForAllInstances                                                  = 2228,
    /// SEARCH-FOR-ANY
    SearchForAny                                                           = 1585,
    /// SEARCH-FOR-ID
    SearchForId                                                            = 1284,
    /// SEARCH-FOR-SPECIFIC-INSTANCE
    SearchForSpecificInstance                                              = 1180,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 1368,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 984,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 265,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 2027,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 1557,
    /// SECOND-TO-FIRST
    SecondToFirst                                                          = 420,
    /// SECONDARY-ECU
    SecondaryEcu                                                           = 382,
    /// SECRET-SEED
    SecretSeed                                                             = 1591,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 160,
    /// SECURE-COM-PROPS
    SecureComProps                                                         = 1256,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 1873,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 1405,
    /// SECURE-COMMUNICATION-DEPLOYMENT
    SecureCommunicationDeployment                                          = 586,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 1990,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 2014,
    /// SECURE-ON-BOARD-COMMUNICATION
    SecureOnBoardCommunication                                             = 1823,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 804,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 1072,
    /// SECURED-PDU-HEADER-08-BIT
    SecuredPduHeader08Bit                                                  = 631,
    /// SECURED-PDU-HEADER-16-BIT
    SecuredPduHeader16Bit                                                  = 737,
    /// SECURED-PDU-HEADER-32-BIT
    SecuredPduHeader32Bit                                                  = 1568,
    /// SECURITY
    Security                                                               = 2031,
    /// SECURITY-EVENT-AGGREGATION-FILTER
    SecurityEventAggregationFilter                                         = 208,
    /// SECURITY-EVENT-CONTEXT-MAPPING
    SecurityEventContextMapping                                            = 1964,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 2358,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 1374,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 214,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 1840,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 499,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 620,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 1068,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 1502,
    /// SECURITY-EVENT-ONE-EVERY-N-FILTER
    SecurityEventOneEveryNFilter                                           = 2020,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 1878,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 20,
    /// SECURITY-EVENT-STATE-FILTER
    SecurityEventStateFilter                                               = 1228,
    /// SECURITY-EVENT-THRESHOLD-FILTER
    SecurityEventThresholdFilter                                           = 337,
    /// SELECTED
    Selected                                                               = 744,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 1773,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 1543,
    /// SENT-TAGGED
    SentTagged                                                             = 2147,
    /// SENT-UNTAGGED
    SentUntagged                                                           = 50,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 1787,
    /// SERIALIZER
    Serializer                                                             = 108,
    /// SERVER-AUTHENTICATE
    ServerAuthenticate                                                     = 981,
    /// SERVER-CALL-POINT
    ServerCallPoint                                                        = 485,
    /// SERVER-DECRYPT
    ServerDecrypt                                                          = 58,
    /// SERVER-ENCRYPT
    ServerEncrypt                                                          = 300,
    /// SERVER-MAC-GENERATE
    ServerMacGenerate                                                      = 313,
    /// SERVER-MAC-VERIFY
    ServerMacVerify                                                        = 370,
    /// SERVER-VERIFY
    ServerVerify                                                           = 1417,
    /// SERVICE-DISCOVERY
    ServiceDiscovery                                                       = 587,
    /// SERVICE-EVENT-DEPLOYMENT
    ServiceEventDeployment                                                 = 1559,
    /// SERVICE-FIELD-DEPLOYMENT
    ServiceFieldDeployment                                                 = 1500,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 248,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 1061,
    /// SERVICE-INSTANCE-TO-MACHINE-MAPPING
    ServiceInstanceToMachineMapping                                        = 1028,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 1349,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 1657,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 1137,
    /// SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToSwClusterDesignPortPrototypeMapping                   = 152,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 1421,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 562,
    /// SERVICE-INTERFACE-DEPLOYMENT
    ServiceInterfaceDeployment                                             = 1797,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING
    ServiceInterfaceElementMapping                                         = 440,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 630,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 1471,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 580,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 1537,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 1788,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 934,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 1755,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 663,
    /// SERVICE-METHOD-DEPLOYMENT
    ServiceMethodDeployment                                                = 1325,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 1129,
    /// SERVICE-ONLY
    ServiceOnly                                                            = 651,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 1929,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 2165,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 1522,
    /// SESSION-HANDLING-ACTIVE
    SessionHandlingActive                                                  = 643,
    /// SESSION-HANDLING-INACTIVE
    SessionHandlingInactive                                                = 891,
    /// SETTER
    Setter                                                                 = 1128,
    /// SG
    Sg                                                                     = 1717,
    /// SH
    Sh                                                                     = 2237,
    /// SHORT-HEADER
    ShortHeader                                                            = 293,
    /// SHOW-ALIAS-NAME
    ShowAliasName                                                          = 682,
    /// SHOW-CATEGORY
    ShowCategory                                                           = 1948,
    /// SHOW-CONTENT
    ShowContent                                                            = 1051,
    /// SHOW-LONG-NAME
    ShowLongName                                                           = 210,
    /// SHOW-NUMBER
    ShowNumber                                                             = 1139,
    /// SHOW-PAGE
    ShowPage                                                               = 608,
    /// SHOW-SEE
    ShowSee                                                                = 1233,
    /// SHOW-SHORT-NAME
    ShowShortName                                                          = 351,
    /// SHOW-TYPE
    ShowType                                                               = 1528,
    /// SI
    Si                                                                     = 1970,
    /// SIDES
    Sides                                                                  = 343,
    /// SIGN
    Sign                                                                   = 1558,
    /// SIGN-WITH-ORIGIN-AUTHENTICATION
    SignWithOriginAuthentication                                           = 705,
    /// SIGNAL-BASED
    SignalBased                                                            = 2310,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 819,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 1488,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 1521,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 2235,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 1848,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 728,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 1551,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 1196,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 1742,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 1449,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 701,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 1549,
    /// SIGNATURE
    Signature                                                              = 117,
    /// SILENT
    Silent                                                                 = 449,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 1897,
    /// SINGLE
    Single                                                                 = 2073,
    /// SINGLE-CORE-REENTRANT
    SingleCoreReentrant                                                    = 1323,
    /// SINGLE-LANGUAGE-REFERRABLE
    SingleLanguageReferrable                                               = 794,
    /// SINGLE-OCCURRENCE
    SingleOccurrence                                                       = 2160,
    /// SK
    Sk                                                                     = 1850,
    /// SL
    Sl                                                                     = 790,
    /// SLAVE
    Slave                                                                  = 748,
    /// SLOPPY
    Sloppy                                                                 = 1804,
    /// SLOW-FLASHING-MODE
    SlowFlashingMode                                                       = 200,
    /// SLP
    Slp                                                                    = 1000,
    /// SM
    Sm                                                                     = 1060,
    /// SN
    Sn                                                                     = 1644,
    /// SO
    So                                                                     = 1341,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 628,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 347,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 1167,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 915,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 857,
    /// SOFTWARE-ACTIVATION-DEPENDENCY
    SoftwareActivationDependency                                           = 2227,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 2231,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 463,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 277,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 1648,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 806,
    /// SOMEIP
    Someip                                                                 = 1302,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 1847,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 1943,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 1484,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 2000,
    /// SOMEIP-FIELD
    SomeipField                                                            = 2105,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 2205,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 311,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 955,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 518,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 1779,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 992,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 259,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 408,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 1510,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 2112,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 94,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 1265,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 251,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 1805,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 1730,
    /// SPEC-ELEMENT-REFERENCE
    SpecElementReference                                                   = 34,
    /// SPEC-ELEMENT-SCOPE
    SpecElementScope                                                       = 1029,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 1618,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 1946,
    /// SQ
    Sq                                                                     = 1336,
    /// SR
    Sr                                                                     = 1624,
    /// SS
    Ss                                                                     = 1933,
    /// SSDP
    Ssdp                                                                   = 1829,
    /// ST
    St                                                                     = 2214,
    /// STACK-USAGE
    StackUsage                                                             = 195,
    /// STANDARD
    Standard                                                               = 1081,
    /// STANDARD-PORT
    StandardPort                                                           = 1409,
    /// START
    Start                                                                  = 680,
    /// START-FROM-BEGINNING
    StartFromBeginning                                                     = 1533,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 1431,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 1342,
    /// STATIC-OR-DYNAMIC-PART-TRIGGER
    StaticOrDynamicPartTrigger                                             = 1822,
    /// STATIC-PART-TRIGGER
    StaticPartTrigger                                                      = 423,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 6,
    /// STATUS-BIT-AGING-AND-DISPLACEMENT
    StatusBitAgingAndDisplacement                                          = 162,
    /// STATUS-BIT-NORMAL
    StatusBitNormal                                                        = 109,
    /// STD
    Std                                                                    = 1269,
    /// STD-AXIS
    StdAxis                                                                = 1343,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 215,
    /// STD_AXIS
    Stdaxis                                                                = 1018,
    /// STEADY
    Steady                                                                 = 554,
    /// STIMULUS-SYNCHRONIZATION
    StimulusSynchronization                                                = 96,
    /// STOP
    Stop                                                                   = 687,
    /// STOP-TRIGGER
    StopTrigger                                                            = 1197,
    /// STORE-EVENT
    StoreEvent                                                             = 646,
    /// STORE-PERSISTENTLY
    StorePersistently                                                      = 860,
    /// STRICT-MODE
    StrictMode                                                             = 647,
    /// STRICT-MONOTONOUS
    StrictMonotonous                                                       = 374,
    /// STRICT-PRIORITY
    StrictPriority                                                         = 2258,
    /// STRICTLY-DECREASING
    StrictlyDecreasing                                                     = 793,
    /// STRICTLY-INCREASING
    StrictlyIncreasing                                                     = 2346,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 379,
    /// SU
    Su                                                                     = 83,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 1719,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 2295,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 2267,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 1726,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 3,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 2140,
    /// SUPPLIER
    Supplier                                                               = 1328,
    /// SUPPORTS-BUFFER-LOCKING
    SupportsBufferLocking                                                  = 513,
    /// SV
    Sv                                                                     = 845,
    /// SVG
    Svg                                                                    = 807,
    /// SW
    Sw                                                                     = 1857,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 1426,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 617,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 1966,
    /// SW-CALIBRATION-METHOD
    SwCalibrationMethod                                                    = 92,
    /// SW-CALPRM-PROTOTYPE
    SwCalprmPrototype                                                      = 1372,
    /// SW-CLASS-ATTR-IMPL
    SwClassAttrImpl                                                        = 851,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 177,
    /// SW-CLASS-PROTOTYPE
    SwClassPrototype                                                       = 1511,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 1268,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 509,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 1912,
    /// SW-COMPONENT-TYPE
    SwComponentType                                                        = 1610,
    /// SW-CONNECTOR
    SwConnector                                                            = 122,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 72,
    /// SW-INSTANCE
    SwInstance                                                             = 57,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 884,
    /// SW-MC-FRAME
    SwMcFrame                                                              = 225,
    /// SW-MC-INTERFACE
    SwMcInterface                                                          = 660,
    /// SW-MC-INTERFACE-SOURCE
    SwMcInterfaceSource                                                    = 910,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 1279,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 613,
    /// SW-SERVICE-PROTOTYPE
    SwServicePrototype                                                     = 603,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 1827,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 657,
    /// SW-VARIABLE-PROTOTYPE
    SwVariablePrototype                                                    = 830,
    /// SWC
    Swc                                                                    = 526,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 458,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 531,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 100,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 1924,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 1,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 1075,
    /// SWC-TIMING
    SwcTiming                                                              = 521,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 2302,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 224,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 26,
    /// SWITCH
    Switch                                                                 = 307,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 1986,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 969,
    /// SYMMETRIC
    Symmetric                                                              = 292,
    /// SYMMETRIC-KEY
    SymmetricKey                                                           = 1436,
    /// SYNC-BASE-TIME-MANAGER
    SyncBaseTimeManager                                                    = 397,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 267,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 203,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 1456,
    /// SYNCHRONIZED
    Synchronized                                                           = 161,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 1519,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 2093,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 1288,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 2174,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 1429,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 244,
    /// SYNCHRONOUS
    Synchronous                                                            = 2254,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 1010,
    /// SYSTEM
    System                                                                 = 1731,
    /// SYSTEM-DESIGN-TIME
    SystemDesignTime                                                       = 383,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 2198,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 1574,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 542,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 956,
    /// SYSTEM-SUPPLIER-BOOT
    SystemSupplierBoot                                                     = 1846,
    /// SYSTEM-SUPPLIER-BOOT-RESP-APP
    SystemSupplierBootRespApp                                              = 1458,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 1208,
    /// TA
    Ta                                                                     = 1354,
    /// TARGET-CONTAINER
    TargetContainer                                                        = 2197,
    /// TASK
    Task                                                                   = 1595,
    /// TC
    Tc                                                                     = 451,
    /// TCP
    Tcp                                                                    = 105,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 1040,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 142,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 2274,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 2173,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 1945,
    /// TD-EVENT-BSW
    TdEventBsw                                                             = 102,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 2350,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 1154,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 274,
    /// TD-EVENT-COM
    TdEventCom                                                             = 86,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 672,
    /// TD-EVENT-CYCLE-START
    TdEventCycleStart                                                      = 2170,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 535,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 178,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 925,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 899,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 1037,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 601,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 1932,
    /// TD-EVENT-SERVICE-INSTANCE
    TdEventServiceInstance                                                 = 140,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 692,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 10,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 611,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 2096,
    /// TD-EVENT-SWC
    TdEventSwc                                                             = 987,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 1087,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 1735,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 1768,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 330,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 2364,
    /// TD-EVENT-VFB
    TdEventVfb                                                             = 114,
    /// TD-EVENT-VFB-PORT
    TdEventVfbPort                                                         = 1836,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 1250,
    /// TE
    Te                                                                     = 723,
    /// TERMINATE
    Terminate                                                              = 2204,
    /// TEST-FAILED
    TestFailed                                                             = 110,
    /// TEST-FAILED-BIT
    TestFailedBit                                                          = 1593,
    /// TEST-FAILED-THIS-OPERATION-CYCLE
    TestFailedThisOperationCycle                                           = 63,
    /// TESTED
    Tested                                                                 = 1309,
    /// TESTED-AND-FAILED
    TestedAndFailed                                                        = 1194,
    /// TG
    Tg                                                                     = 487,
    /// TH
    Th                                                                     = 1416,
    /// TI
    Ti                                                                     = 486,
    /// TIFF
    Tiff                                                                   = 549,
    /// TIME
    Time                                                                   = 67,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 1994,
    /// TIME-BASE-RESOURCE
    TimeBaseResource                                                       = 2191,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 1460,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 972,
    /// TIME-SYNC-SERVER-CONFIGURATION
    TimeSyncServerConfiguration                                            = 607,
    /// TIME-SYNCHRONIZATION-INTERFACE
    TimeSynchronizationInterface                                           = 1831,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 1505,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 190,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 1956,
    /// TIMING-CONDITION
    TimingCondition                                                        = 294,
    /// TIMING-CONSTRAINT
    TimingConstraint                                                       = 893,
    /// TIMING-DESCRIPTION
    TimingDescription                                                      = 1102,
    /// TIMING-DESCRIPTION-EVENT
    TimingDescriptionEvent                                                 = 233,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 1329,
    /// TIMING-EVENT
    TimingEvent                                                            = 284,
    /// TIMING-EXTENSION
    TimingExtension                                                        = 1748,
    /// TIMING-EXTENSION-RESOURCE
    TimingExtensionResource                                                = 1120,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 2066,
    /// TIP
    Tip                                                                    = 326,
    /// TK
    Tk                                                                     = 763,
    /// TL
    Tl                                                                     = 2341,
    /// TLS-12
    Tls12                                                                  = 1901,
    /// TLS-13
    Tls13                                                                  = 1613,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 479,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 349,
    /// TLS-CRYPTO-CIPHER-SUITE-PROPS
    TlsCryptoCipherSuiteProps                                              = 911,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 258,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 489,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 2178,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 2300,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 1668,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 2245,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 838,
    /// TN
    Tn                                                                     = 1257,
    /// TO
    To                                                                     = 769,
    /// TOP
    Top                                                                    = 621,
    /// TOPBOT
    Topbot                                                                 = 553,
    /// TOPIC
    Topic                                                                  = 1810,
    /// TOPIC-1
    Topic1                                                                 = 755,
    /// TOPIC-PREFIX
    TopicPrefix                                                            = 253,
    /// TP-ADDRESS
    TpAddress                                                              = 1898,
    /// TP-CONFIG
    TpConfig                                                               = 885,
    /// TP-CONNECTION-IDENT
    TpConnectionIdent                                                      = 2359,
    /// TR
    Tr                                                                     = 1338,
    /// TRACE
    Trace                                                                  = 296,
    /// TRACE-REFERRABLE
    TraceReferrable                                                        = 1552,
    /// TRACEABLE
    Traceable                                                              = 2212,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 456,
    /// TRACEABLE-TEXT
    TraceableText                                                          = 1016,
    /// TRACED-FAILURE
    TracedFailure                                                          = 335,
    /// TRANSFER
    Transfer                                                               = 1428,
    /// TRANSFORMATION-PROPS
    TransformationProps                                                    = 1335,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 1052,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 1412,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 1935,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 2109,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 4,
    /// TRANSFORMER-ERROR-HANDLING
    TransformerErrorHandling                                               = 422,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 903,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 844,
    /// TRANSFORMING-I-SIGNAL
    TransformingISignal                                                    = 1176,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 1396,
    /// TRANSLATION-START
    TranslationStart                                                       = 599,
    /// TRANSPORT
    Transport                                                              = 980,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 538,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 188,
    /// TRAP
    Trap                                                                   = 1714,
    /// TRIGGER
    Trigger                                                                = 996,
    /// TRIGGER-ACTIVATED
    TriggerActivated                                                       = 1670,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 2221,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 1275,
    /// TRIGGER-RELEASED
    TriggerReleased                                                        = 505,
    /// TRIGGER-UNICAST
    TriggerUnicast                                                         = 822,
    /// TRIGGERED
    Triggered                                                              = 558,
    /// TRIGGERED-ON-CHANGE
    TriggeredOnChange                                                      = 2106,
    /// TRIGGERED-ON-CHANGE-WITHOUT-REPETITION
    TriggeredOnChangeWithoutRepetition                                     = 1148,
    /// TRIGGERED-ON-EVALUATION
    TriggeredOnEvaluation                                                  = 2345,
    /// TRIGGERED-WITHOUT-REPETITION
    TriggeredWithoutRepetition                                             = 153,
    /// TRUE
    True                                                                   = 2011,
    /// TS
    Ts                                                                     = 266,
    /// TT
    Tt                                                                     = 1631,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 269,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 547,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 217,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 175,
    /// TUNNEL
    Tunnel                                                                 = 184,
    /// TW
    Tw                                                                     = 865,
    /// TX-REF-TRIGGER
    TxRefTrigger                                                           = 1438,
    /// TX-REF-TRIGGER-GAP
    TxRefTriggerGap                                                        = 1076,
    /// TX-TRIGGER-MERGED
    TxTriggerMerged                                                        = 2125,
    /// TX-TRIGGER-SINGLE
    TxTriggerSingle                                                        = 1070,
    /// UCM
    Ucm                                                                    = 93,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 1721,
    /// UCM-MASTER
    UcmMaster                                                              = 1245,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 2271,
    /// UCM-STEP
    UcmStep                                                                = 746,
    /// UDP
    Udp                                                                    = 977,
    /// UDP-CHECKSUM-DISABLED
    UdpChecksumDisabled                                                    = 764,
    /// UDP-CHECKSUM-ENABLED
    UdpChecksumEnabled                                                     = 1437,
    /// UDP-NM
    UdpNm                                                                  = 2313,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 82,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 532,
    /// UDS
    Uds                                                                    = 1673,
    /// UK
    Uk                                                                     = 1578,
    /// UNDECIDED
    Undecided                                                              = 2193,
    /// UNDEFINED
    Undefined                                                              = 353,
    /// UNIT
    Unit                                                                   = 1480,
    /// UNIT-GROUP
    UnitGroup                                                              = 892,
    /// UNNUMBER
    Unnumber                                                               = 661,
    /// UNSPECIFIED
    Unspecified                                                            = 1064,
    /// UP-LINK-PORT
    UpLinkPort                                                             = 569,
    /// UPDATE
    Update                                                                 = 76,
    /// UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT
    UploadableExclusivePackageElement                                      = 1440,
    /// UPLOADABLE-PACKAGE-ELEMENT
    UploadablePackageElement                                               = 1722,
    /// UR
    Ur                                                                     = 1141,
    /// USE-ARGUMENT-TYPE
    UseArgumentType                                                        = 452,
    /// USE-ARRAY-BASE-TYPE
    UseArrayBaseType                                                       = 820,
    /// USE-FIRST-CONTEXT-DATA
    UseFirstContextData                                                    = 1475,
    /// USE-LAST-CONTEXT-DATA
    UseLastContextData                                                     = 1142,
    /// USE-VOID
    UseVoid                                                                = 366,
    /// USER-DEFINED
    UserDefined                                                            = 1184,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 121,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 1535,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 1845,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 508,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 1304,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 432,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 2071,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 894,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 1434,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 926,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 22,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 1244,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 2357,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 951,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 1379,
    /// USES-LOGGING
    UsesLogging                                                            = 123,
    /// UZ
    Uz                                                                     = 711,
    /// V-2-X-ACTIVE-SUPPORTED
    V2XActiveSupported                                                     = 2327,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 1698,
    /// V-2-X-FACILITIES
    V2XFacilities                                                          = 2344,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 51,
    /// V-2-X-MANAGEMENT
    V2XManagement                                                          = 226,
    /// V-2-X-NOT-SUPPORTED
    V2XNotSupported                                                        = 1826,
    /// VALID
    Valid                                                                  = 362,
    /// VAR
    Var                                                                    = 1851,
    /// VAR-FAST
    VarFast                                                                = 1355,
    /// VAR-NO-INIT
    VarNoInit                                                              = 1130,
    /// VAR-POWER-ON-INIT
    VarPowerOnInit                                                         = 741,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 864,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 1009,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 1207,
    /// VARIABLE-DATA-PROTOTYPE-RECEIVED
    VariableDataPrototypeReceived                                          = 1132,
    /// VARIABLE-DATA-PROTOTYPE-SENT
    VariableDataPrototypeSent                                              = 2074,
    /// VARIABLE-SIZE
    VariableSize                                                           = 1530,
    /// VARIANT-LINK-TIME
    VariantLinkTime                                                        = 1311,
    /// VARIANT-POST-BUILD
    VariantPostBuild                                                       = 1006,
    /// VARIANT-POST-BUILD-LOADABLE
    VariantPostBuildLoadable                                               = 118,
    /// VARIANT-POST-BUILD-SELECTABLE
    VariantPostBuildSelectable                                             = 372,
    /// VARIANT-PRE-COMPILE
    VariantPreCompile                                                      = 360,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 12,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 11,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 868,
    /// VENDOR-SPECIFIC
    VendorSpecific                                                         = 2229,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 2180,
    /// VERBOSE
    Verbose                                                                = 186,
    /// VERIFICATION
    Verification                                                           = 779,
    /// VERIFY
    Verify                                                                 = 1057,
    /// VERTEX-OF-TARGET-CONTAINER
    VertexOfTargetContainer                                                = 1473,
    /// VFB-TIMING
    VfbTiming                                                              = 2123,
    /// VI
    Vi                                                                     = 2323,
    /// VIEW-MAP
    ViewMap                                                                = 113,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 816,
    /// VLAN-CONFIG
    VlanConfig                                                             = 1575,
    /// VO
    Vo                                                                     = 14,
    /// VOLATILE
    Volatile                                                               = 1150,
    /// WAIT-POINT
    WaitPoint                                                              = 1702,
    /// WAIT-TIME-DATE
    WaitTimeDate                                                           = 2330,
    /// WARMUP
    Warmup                                                                 = 1144,
    /// WARN
    Warn                                                                   = 1153,
    /// WARNING
    Warning                                                                = 1790,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 241,
    /// WATCH-DOG-MANAGER
    WatchDogManager                                                        = 378,
    /// WATCH-TRIGGER
    WatchTrigger                                                           = 193,
    /// WATCH-TRIGGER-GAP
    WatchTriggerGap                                                        = 2273,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 1660,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 1762,
    /// WEIGHTED-ROUND-ROBIN
    WeightedRoundRobin                                                     = 1066,
    /// WILL-CALL
    WillCall                                                               = 332,
    /// WILL-RECEIVE
    WillReceive                                                            = 1199,
    /// WILL-SEND
    WillSend                                                               = 2144,
    /// WO
    Wo                                                                     = 2061,
    /// WONT-CALL
    WontCall                                                               = 1442,
    /// WONT-RECEIVE
    WontReceive                                                            = 128,
    /// WONT-SEND
    WontSend                                                               = 393,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 1065,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 2098,
    /// WRITE-ONLY
    WriteOnly                                                              = 773,
    /// WRONG-TRIGGER
    WrongTrigger                                                           = 2089,
    /// X-509
    X509                                                                   = 2336,
    /// X-MII
    XMii                                                                   = 185,
    /// X-MMI
    XMmi                                                                   = 2307,
    /// XCP
    Xcp                                                                    = 1297,
    /// XCP-PDU
    XcpPdu                                                                 = 415,
    /// XDOC
    Xdoc                                                                   = 1577,
    /// XFILE
    Xfile                                                                  = 146,
    /// XG-MII
    XgMii                                                                  = 1664,
    /// XH
    Xh                                                                     = 1200,
    /// XOR
    Xor                                                                    = 1332,
    /// XREF-TARGET
    XrefTarget                                                             = 853,
    /// XXG-MII
    XxgMii                                                                 = 843,
    /// YO
    Yo                                                                     = 1241,
    /// ZH
    Zh                                                                     = 30,
    /// ZU
    Zu                                                                     = 1186,
    /// default
    default                                                                = 707,
    /// preserve
    preserve                                                               = 2162,
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

