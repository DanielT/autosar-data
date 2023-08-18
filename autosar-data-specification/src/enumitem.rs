use crate::hashfunc;

#[cfg(feature = "pylib")]
use pyo3::prelude::*;

#[derive(Debug)]
/// The error type ParseEnumItemError is returned when from_str() / parse() fails for EnumItem
pub struct ParseEnumItemError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "pylib", pyclass)]
#[repr(u16)]
/// Enum of all possible enum values in Autosar
pub enum EnumItem {
    /// -500-MILES
    _500Miles                                                              = 532,
    /// 1000BASE-T
    _1000baseT                                                             = 1452,
    /// 1000BASE-T1
    _1000baseT1                                                            = 2464,
    /// 100BASE-T1
    _100baseT1                                                             = 976,
    /// 100BASE-TX
    _100baseTx                                                             = 1328,
    /// 10BASE-T1S
    _10baseT1s                                                             = 1416,
    /// AA
    Aa                                                                     = 2444,
    /// AB
    Ab                                                                     = 611,
    /// ABSTRACT
    Abstract                                                               = 1392,
    /// ABSTRACT-ACCESS-POINT
    AbstractAccessPoint                                                    = 1424,
    /// ABSTRACT-CAN-CLUSTER
    AbstractCanCluster                                                     = 1565,
    /// ABSTRACT-CAN-COMMUNICATION-CONNECTOR
    AbstractCanCommunicationConnector                                      = 2425,
    /// ABSTRACT-CAN-COMMUNICATION-CONTROLLER
    AbstractCanCommunicationController                                     = 1681,
    /// ABSTRACT-CAN-PHYSICAL-CHANNEL
    AbstractCanPhysicalChannel                                             = 1776,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 443,
    /// ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS
    AbstractDoIpLogicAddressProps                                          = 396,
    /// ABSTRACT-ETHERNET-FRAME
    AbstractEthernetFrame                                                  = 1470,
    /// ABSTRACT-EVENT
    AbstractEvent                                                          = 541,
    /// ABSTRACT-EXECUTION-CONTEXT
    AbstractExecutionContext                                               = 2281,
    /// ABSTRACT-IAM-REMOTE-SUBJECT
    AbstractIamRemoteSubject                                               = 911,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE
    AbstractImplementationDataType                                         = 1333,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT
    AbstractImplementationDataTypeElement                                  = 53,
    /// ABSTRACT-PROVIDED-PORT-PROTOTYPE
    AbstractProvidedPortPrototype                                          = 1842,
    /// ABSTRACT-RAW-DATA-STREAM-INTERFACE
    AbstractRawDataStreamInterface                                         = 1931,
    /// ABSTRACT-REQUIRED-PORT-PROTOTYPE
    AbstractRequiredPortPrototype                                          = 1676,
    /// ABSTRACT-SECURITY-EVENT-FILTER
    AbstractSecurityEventFilter                                            = 764,
    /// ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER
    AbstractSecurityIdsmInstanceFilter                                     = 760,
    /// ABSTRACT-SERVICE-INSTANCE
    AbstractServiceInstance                                                = 544,
    /// ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING
    AbstractSignalBasedToISignalTriggeringMapping                          = 675,
    /// ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE
    AbstractSynchronizedTimeBaseInterface                                  = 1916,
    /// ACCEPT-ALL
    AcceptAll                                                              = 1377,
    /// ACCEPT-CONFIGURED
    AcceptConfigured                                                       = 283,
    /// ACCES-PERRMISSION-SERVICE-CLASS
    AccesPerrmissionServiceClass                                           = 1386,
    /// ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS
    AccessPermissionInstanceOverridesClass                                 = 1689,
    /// ACCESS-PERMISSION-SERVICE-CLASS
    AccessPermissionServiceClass                                           = 525,
    /// ACCESS-PERMISSION-SERVICE-INSTANCE
    AccessPermissionServiceInstance                                        = 2007,
    /// ACK-WITH-RT
    AckWithRt                                                              = 635,
    /// ACK-WITHOUT-RT
    AckWithoutRt                                                           = 1380,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 974,
    /// ACL-OPERATION
    AclOperation                                                           = 493,
    /// ACL-PERMISSION
    AclPermission                                                          = 672,
    /// ACL-ROLE
    AclRole                                                                = 769,
    /// ACTION
    Action                                                                 = 1794,
    /// ACTION-ITEM
    ActionItem                                                             = 2173,
    /// ACTION-LIST
    ActionList                                                             = 1847,
    /// ACTIVATE
    Activate                                                               = 1145,
    /// ACTIVATION-AND-TRIGGER-UNICAST
    ActivationAndTriggerUnicast                                            = 2394,
    /// ACTIVATION-MULTICAST
    ActivationMulticast                                                    = 726,
    /// ACTIVATION-UNICAST
    ActivationUnicast                                                      = 2323,
    /// ACTIVE
    Active                                                                 = 1437,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 1329,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 142,
    /// ADAPTIVE-EVENT-RECEIVED
    AdaptiveEventReceived                                                  = 2045,
    /// ADAPTIVE-EVENT-SENT
    AdaptiveEventSent                                                      = 1826,
    /// ADAPTIVE-FIELD-GETTER-CALLED
    AdaptiveFieldGetterCalled                                              = 2270,
    /// ADAPTIVE-FIELD-GETTER-COMPLETED
    AdaptiveFieldGetterCompleted                                           = 812,
    /// ADAPTIVE-FIELD-NOTIFICATION-RECEIVED
    AdaptiveFieldNotificationReceived                                      = 2478,
    /// ADAPTIVE-FIELD-NOTIFICATION-SENT
    AdaptiveFieldNotificationSent                                          = 1908,
    /// ADAPTIVE-FIELD-SETTER-CALLED
    AdaptiveFieldSetterCalled                                              = 1334,
    /// ADAPTIVE-FIELD-SETTER-COMPLETED
    AdaptiveFieldSetterCompleted                                           = 1067,
    /// ADAPTIVE-FIREWALL-MODULE-INSTANTIATION
    AdaptiveFirewallModuleInstantiation                                    = 339,
    /// ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING
    AdaptiveFirewallToPortPrototypeMapping                                 = 2319,
    /// ADAPTIVE-METHOD-CALL-RECEIVED
    AdaptiveMethodCallReceived                                             = 110,
    /// ADAPTIVE-METHOD-CALLED
    AdaptiveMethodCalled                                                   = 551,
    /// ADAPTIVE-METHOD-RESPONSE-RECEIVED
    AdaptiveMethodResponseReceived                                         = 2137,
    /// ADAPTIVE-METHOD-RESPONSE-SENT
    AdaptiveMethodResponseSent                                             = 2121,
    /// ADAPTIVE-MODULE-INSTANTIATION
    AdaptiveModuleInstantiation                                            = 1226,
    /// ADAPTIVE-PLATFORM-SERVICE-INSTANCE
    AdaptivePlatformServiceInstance                                        = 84,
    /// ADAPTIVE-SERVICE-FIND-COMPLETED
    AdaptiveServiceFindCompleted                                           = 2001,
    /// ADAPTIVE-SERVICE-FIND-STARTED
    AdaptiveServiceFindStarted                                             = 1030,
    /// ADAPTIVE-SERVICE-OFFER-COMPLETED
    AdaptiveServiceOfferCompleted                                          = 2222,
    /// ADAPTIVE-SERVICE-OFFER-STARTED
    AdaptiveServiceOfferStarted                                            = 0,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED
    AdaptiveServiceStopSubscriptionCompleted                               = 1568,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED
    AdaptiveServiceStopSubscriptionStarted                                 = 2157,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED
    AdaptiveServiceSubscriptionAcknowledgeCompleted                        = 2219,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED
    AdaptiveServiceSubscriptionAcknowledgeStarted                          = 1668,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED
    AdaptiveServiceSubscriptionCompleted                                   = 577,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED
    AdaptiveServiceSubscriptionStarted                                     = 1382,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 1008,
    /// ADDR-METHOD-SHORT-NAME
    AddrMethodShortName                                                    = 1444,
    /// ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT
    AddrMethodShortNameAndAlignment                                        = 1583,
    /// AF
    Af                                                                     = 1664,
    /// AFTER-SALES
    AfterSales                                                             = 651,
    /// AFTERMAKET
    Aftermaket                                                             = 1312,
    /// AFTERMARKET
    Aftermarket                                                            = 18,
    /// AGE
    Age                                                                    = 1113,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 1764,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 22,
    /// AGREED
    Agreed                                                                 = 840,
    /// AH
    Ah                                                                     = 780,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 1152,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 1058,
    /// ALL
    All                                                                    = 2073,
    /// ALL-16-BIT
    All16Bit                                                               = 1233,
    /// ALL-INDICES-DIFFERENT-ARRAY-SIZE
    AllIndicesDifferentArraySize                                           = 2424,
    /// ALL-INDICES-SAME-ARRAY-SIZE
    AllIndicesSameArraySize                                                = 2108,
    /// ALL-PARTIAL-NETWORKS-ACTIVE
    AllPartialNetworksActive                                               = 148,
    /// ALL-SUPPORTED-DTCS
    AllSupportedDtcs                                                       = 830,
    /// ALLOCATOR
    Allocator                                                              = 1508,
    /// ALLOW
    Allow                                                                  = 1120,
    /// ALTERNATING-8-BIT
    Alternating8Bit                                                        = 2291,
    /// ALWAYS
    Always                                                                 = 368,
    /// AM
    Am                                                                     = 847,
    /// AMBER-WARNING
    AmberWarning                                                           = 1129,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 1079,
    /// AND
    And                                                                    = 2094,
    /// ANY
    Any                                                                    = 1141,
    /// ANY-PARTIAL-NETWORK-ACTIVE
    AnyPartialNetworkActive                                                = 2056,
    /// ANY-SEND-OPERATION
    AnySendOperation                                                       = 1398,
    /// ANY-STANDARDIZED
    AnyStandardized                                                        = 268,
    /// AP
    Ap                                                                     = 2331,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 303,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 219,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 1535,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 581,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 2335,
    /// API
    Api                                                                    = 76,
    /// API-BASED
    ApiBased                                                               = 2052,
    /// API-USE
    ApiUse                                                                 = 2285,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 1102,
    /// APPLICATION
    Application                                                            = 2329,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 1453,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 1709,
    /// APPLICATION-ARRAY-ELEMENT
    ApplicationArrayElement                                                = 1035,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 880,
    /// APPLICATION-ASSOC-MAP-ELEMENT
    ApplicationAssocMapElement                                             = 1624,
    /// APPLICATION-COMPOSITE-DATA-TYPE
    ApplicationCompositeDataType                                           = 549,
    /// APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE
    ApplicationCompositeElementDataPrototype                               = 1697,
    /// APPLICATION-DATA-TYPE
    ApplicationDataType                                                    = 1314,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 466,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 567,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 1376,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 1763,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 1605,
    /// APPLICATION-ONLY
    ApplicationOnly                                                        = 1223,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 1732,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 342,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 552,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 1647,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 1112,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 1536,
    /// AR
    Ar                                                                     = 321,
    /// AR--CLIENT--SERVER
    ArClientServer                                                         = 2368,
    /// AR-ELEMENT
    ArElement                                                              = 302,
    /// AR-PACKAGE
    ArPackage                                                              = 2151,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 233,
    /// ARBITRATION
    Arbitration                                                            = 2310,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 408,
    /// ARRAY
    Array                                                                  = 1625,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 1183,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 750,
    /// ARTIFACT-LOCATOR
    ArtifactLocator                                                        = 1887,
    /// AS
    As                                                                     = 739,
    /// AS-IS
    AsIs                                                                   = 2102,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 1975,
    /// ASYMMETRIC-FROM-BYTE-ARRAY
    AsymmetricFromByteArray                                                = 1296,
    /// ASYMMETRIC-TO-BYTE-ARRAY
    AsymmetricToByteArray                                                  = 315,
    /// ASYNCHRONOUS
    Asynchronous                                                           = 489,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 647,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 1136,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 2066,
    /// ATOMIC-SW-COMPONENT-TYPE
    AtomicSwComponentType                                                  = 185,
    /// ATP-BLUEPRINT
    AtpBlueprint                                                           = 1189,
    /// ATP-BLUEPRINTABLE
    AtpBlueprintable                                                       = 123,
    /// ATP-CLASSIFIER
    AtpClassifier                                                          = 1222,
    /// ATP-DEFINITION
    AtpDefinition                                                          = 2459,
    /// ATP-FEATURE
    AtpFeature                                                             = 2030,
    /// ATP-PROTOTYPE
    AtpPrototype                                                           = 2376,
    /// ATP-STRUCTURE-ELEMENT
    AtpStructureElement                                                    = 227,
    /// ATP-TYPE
    AtpType                                                                = 1566,
    /// ATTRIBUTE-TAILORING
    AttributeTailoring                                                     = 97,
    /// AUTHENTICATE
    Authenticate                                                           = 64,
    /// AUTO
    Auto                                                                   = 353,
    /// AUTO-IP
    AutoIp                                                                 = 2095,
    /// AUTO-IP--DOIP
    AutoIpDoip                                                             = 511,
    /// AUTO-IPDHCPV-4
    AutoIpdhcpv4                                                           = 569,
    /// AUTONOMOUS
    Autonomous                                                             = 1973,
    /// AUTOSAR-DATA-PROTOTYPE
    AutosarDataPrototype                                                   = 1795,
    /// AUTOSAR-DATA-TYPE
    AutosarDataType                                                        = 1670,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 979,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 51,
    /// AVB--IEEE-802--1-AS
    AvbIeee8021As                                                          = 356,
    /// AY
    Ay                                                                     = 589,
    /// AZ
    Az                                                                     = 1628,
    /// BA
    Ba                                                                     = 1155,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 1788,
    /// BASE-T
    BaseT                                                                  = 510,
    /// BASE-TYPE
    BaseType                                                               = 160,
    /// BASIC-SOFTWARE-MODE-MANAGER
    BasicSoftwareModeManager                                               = 861,
    /// BE
    Be                                                                     = 400,
    /// BG
    Bg                                                                     = 2435,
    /// BH
    Bh                                                                     = 68,
    /// BI
    Bi                                                                     = 338,
    /// BIDIRECTIONAL
    Bidirectional                                                          = 2253,
    /// BINARY-MANIFEST-ADDRESSABLE-OBJECT
    BinaryManifestAddressableObject                                        = 149,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 2313,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 409,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 1729,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 300,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 1399,
    /// BINARY-MANIFEST-RESOURCE
    BinaryManifestResource                                                 = 2292,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 826,
    /// BLINK-MODE
    BlinkMode                                                              = 2296,
    /// BLINK-OR-CONTINUOUS-ON-MODE
    BlinkOrContinuousOnMode                                                = 1017,
    /// BLOCK
    Block                                                                  = 1748,
    /// BLOCK-SOURCE
    BlockSource                                                            = 982,
    /// BLOCK-STATE
    BlockState                                                             = 959,
    /// BLUEPRINT-DERIVATION-TIME
    BlueprintDerivationTime                                                = 1506,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 309,
    /// BMP
    Bmp                                                                    = 1199,
    /// BN
    Bn                                                                     = 2477,
    /// BO
    Bo                                                                     = 1666,
    /// BOLD
    Bold                                                                   = 1324,
    /// BOLDITALIC
    Bolditalic                                                             = 652,
    /// BONJOUR
    Bonjour                                                                = 1651,
    /// BOTTOM
    Bottom                                                                 = 1117,
    /// BR
    Br                                                                     = 985,
    /// BREAK
    Break                                                                  = 1557,
    /// BRIEF
    Brief                                                                  = 1268,
    /// BRIEF-BYPASSING-FILTERS
    BriefBypassingFilters                                                  = 1235,
    /// BROAD-R-REACH
    BroadRReach                                                            = 1014,
    /// BSW
    Bsw                                                                    = 2427,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 1680,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 1533,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 1645,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 849,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 886,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 1052,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 2401,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 171,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 1875,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 757,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 448,
    /// BSW-EVENT
    BswEvent                                                               = 625,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 1543,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 206,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 1864,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 2032,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 1489,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 1346,
    /// BSW-M-ENTRY-CALL-RETURNED
    BswMEntryCallReturned                                                  = 524,
    /// BSW-M-ENTRY-CALLED
    BswMEntryCalled                                                        = 2288,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 2212,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 1273,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 1068,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 2063,
    /// BSW-MODULE-CALL-POINT
    BswModuleCallPoint                                                     = 241,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 1789,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 348,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 365,
    /// BSW-MODULE-ENTITY
    BswModuleEntity                                                        = 1428,
    /// BSW-MODULE-ENTITY-ACTIVATED
    BswModuleEntityActivated                                               = 98,
    /// BSW-MODULE-ENTITY-STARTED
    BswModuleEntityStarted                                                 = 728,
    /// BSW-MODULE-ENTITY-TERMINATED
    BswModuleEntityTerminated                                              = 1888,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 1956,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 152,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 420,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 1705,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 2437,
    /// BSW-SCHEDULE-EVENT
    BswScheduleEvent                                                       = 727,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 210,
    /// BSW-SERVICE-DEPENDENCY-IDENT
    BswServiceDependencyIdent                                              = 696,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 576,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 1752,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 2480,
    /// BUILD
    Build                                                                  = 1343,
    /// BUILD-ACTION
    BuildAction                                                            = 1217,
    /// BUILD-ACTION-ENTITY
    BuildActionEntity                                                      = 536,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 2474,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 327,
    /// BUILD-TYPE-DEBUG
    BuildTypeDebug                                                         = 1275,
    /// BUILD-TYPE-RELEASE
    BuildTypeRelease                                                       = 1412,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 2209,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 708,
    /// BUS-MIRROR-CHANNEL-MAPPING
    BusMirrorChannelMapping                                                = 479,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 12,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 1727,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 1056,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 2238,
    /// C
    C                                                                      = 1308,
    /// CA
    Ca                                                                     = 1678,
    /// CALCULATED
    Calculated                                                             = 121,
    /// CALIBRATION-OFFLINE
    CalibrationOffline                                                     = 2355,
    /// CALIBRATION-ONLINE
    CalibrationOnline                                                      = 1601,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 542,
    /// CALIBRATION-VARIABLES
    CalibrationVariables                                                   = 312,
    /// CALLBACK
    Callback                                                               = 34,
    /// CALLOUT
    Callout                                                                = 685,
    /// CALPRM
    Calprm                                                                 = 2146,
    /// CAN-20
    Can20                                                                  = 1997,
    /// CAN-BE-REMOVED
    CanBeRemoved                                                           = 126,
    /// CAN-BE-TERMINATED
    CanBeTerminated                                                        = 1327,
    /// CAN-BE-TERMINATED-AND-RESTARTED
    CanBeTerminatedAndRestarted                                            = 1358,
    /// CAN-CLUSTER
    CanCluster                                                             = 791,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 1711,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 194,
    /// CAN-FD
    CanFd                                                                  = 371,
    /// CAN-FRAME
    CanFrame                                                               = 237,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 440,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 428,
    /// CAN-NM-NODE
    CanNmNode                                                              = 1517,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 446,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 668,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 2363,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 377,
    /// CAN-TP-NODE
    CanTpNode                                                              = 91,
    /// CAN-XL-PROPS
    CanXlProps                                                             = 205,
    /// CANCEL
    Cancel                                                                 = 27,
    /// CANNOT-BE-REMOVED
    CannotBeRemoved                                                        = 2191,
    /// CAPTION
    Caption                                                                = 2213,
    /// CAPTURE-ASYNCHRONOUS-TO-REPORTING
    CaptureAsynchronousToReporting                                         = 1266,
    /// CAPTURE-ASYNCHRONOUSLY-TO-REPORTING
    CaptureAsynchronouslyToReporting                                       = 814,
    /// CAPTURE-SYNCHRONOUS-TO-REPORTING
    CaptureSynchronousToReporting                                          = 231,
    /// CAPTURE-SYNCHRONOUSLY-TO-REPORTING
    CaptureSynchronouslyToReporting                                        = 457,
    /// CAT-1
    Cat1                                                                   = 1755,
    /// CAT-2
    Cat2                                                                   = 1571,
    /// CAUTION
    Caution                                                                = 167,
    /// CENTER
    Center                                                                 = 323,
    /// CHANNEL-A
    ChannelA                                                               = 572,
    /// CHANNEL-B
    ChannelB                                                               = 1003,
    /// CHAPTER
    Chapter                                                                = 1163,
    /// CHECK-AT-NEXT-HALT
    CheckAtNextHalt                                                        = 1455,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 1440,
    /// CIRCLE
    Circle                                                                 = 2083,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 386,
    /// CLASSIC
    Classic                                                                = 1982,
    /// CLEAR
    Clear                                                                  = 1153,
    /// CLEAR-ALL-DTCS
    ClearAllDtcs                                                           = 329,
    /// CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    ClearDynamicallyDefineDataIdentifier                                   = 28,
    /// CLIENT-AUTHENTICATE
    ClientAuthenticate                                                     = 1890,
    /// CLIENT-DECRYPT
    ClientDecrypt                                                          = 1623,
    /// CLIENT-ENCRYPT
    ClientEncrypt                                                          = 2362,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 1924,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 2326,
    /// CLIENT-MAC-GENERATE
    ClientMacGenerate                                                      = 2061,
    /// CLIENT-MAC-VERIFY
    ClientMacVerify                                                        = 660,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 2197,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 2340,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 414,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 2140,
    /// CLIENT-VERIFY
    ClientVerify                                                           = 2026,
    /// CLOSED
    Closed                                                                 = 1162,
    /// CO
    Co                                                                     = 404,
    /// CODE
    Code                                                                   = 79,
    /// CODE-GENERATION-TIME
    CodeGenerationTime                                                     = 746,
    /// CODEGENERATION
    Codegeneration                                                         = 2490,
    /// COLDSTART
    Coldstart                                                              = 2372,
    /// COLLECTABLE-ELEMENT
    CollectableElement                                                     = 1291,
    /// COLLECTION
    Collection                                                             = 1015,
    /// COM-AXIS
    ComAxis                                                                = 1434,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 2463,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 1984,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 1337,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 1089,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 2090,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 437,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 1283,
    /// COM-GRANT
    ComGrant                                                               = 1500,
    /// COM-GRANT-DESIGN
    ComGrantDesign                                                         = 1134,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 2431,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 41,
    /// COM-MANAGER
    ComManager                                                             = 627,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 1669,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 1590,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 2220,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 2350,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 381,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 2432,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 1374,
    /// COMM-CONNECTOR-PORT
    CommConnectorPort                                                      = 2047,
    /// COMMAND-LINE-LONG-FORM
    CommandLineLongForm                                                    = 1617,
    /// COMMAND-LINE-SHORT-FORM
    CommandLineShortForm                                                   = 1824,
    /// COMMAND-LINE-SIMPLE-FORM
    CommandLineSimpleForm                                                  = 1284,
    /// COMMON
    Common                                                                 = 1367,
    /// COMMUNICATION-CLUSTER
    CommunicationCluster                                                   = 603,
    /// COMMUNICATION-CONNECTOR
    CommunicationConnector                                                 = 1522,
    /// COMMUNICATION-CONTROLLER
    CommunicationController                                                = 256,
    /// COMMUNICATION-INTER-ECU
    CommunicationInterEcu                                                  = 1467,
    /// COMMUNICATION-INTRA-PARTITION
    CommunicationIntraPartition                                            = 930,
    /// COMPILE
    Compile                                                                = 2381,
    /// COMPILER
    Compiler                                                               = 832,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 1967,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 463,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 821,
    /// COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING
    CompositionPortToExecutablePortMapping                                 = 995,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 512,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 1635,
    /// COMPU-METHOD
    CompuMethod                                                            = 31,
    /// COM_AXIS
    Comaxis                                                                = 322,
    /// CONCRETE
    Concrete                                                               = 1640,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 201,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 301,
    /// CONDITIONAL
    Conditional                                                            = 2223,
    /// CONFIDENTIALITY-OFFSET--0
    ConfidentialityOffset0                                                 = 2258,
    /// CONFIDENTIALITY-OFFSET--30
    ConfidentialityOffset30                                                = 1800,
    /// CONFIDENTIALITY-OFFSET--50
    ConfidentialityOffset50                                                = 1825,
    /// CONFIG-DATA
    ConfigData                                                             = 2383,
    /// CONFIGURED
    Configured                                                             = 1636,
    /// CONFIRMED
    Confirmed                                                              = 796,
    /// CONFIRMED-DTC-BIT
    ConfirmedDtcBit                                                        = 174,
    /// CONNECT
    Connect                                                                = 1665,
    /// CONSISTENCY-MECHANISM-REQUIRED
    ConsistencyMechanismRequired                                           = 857,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 2418,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 2227,
    /// CONSOLE
    Console                                                                = 2000,
    /// CONST
    Const                                                                  = 40,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 883,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 1462,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 2484,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 122,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 1837,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 1662,
    /// CONSUMER
    Consumer                                                               = 593,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 410,
    /// CONTINUE-AT-IT-POSITION
    ContinueAtItPosition                                                   = 99,
    /// CONTINUOUS-ON-MODE
    ContinuousOnMode                                                       = 914,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 346,
    /// COUPLING-PORT
    CouplingPort                                                           = 878,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 1856,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 902,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 413,
    /// COUPLING-PORT-STRUCTURAL-ELEMENT
    CouplingPortStructuralElement                                          = 1730,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 1944,
    /// CP
    Cp                                                                     = 213,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 1757,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 1161,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 1909,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 1853,
    /// CP-SOFTWARE-CLUSTER-RESOURCE
    CpSoftwareClusterResource                                              = 380,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 319,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 2099,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 670,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 1384,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 2020,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 2143,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 2150,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 906,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 1866,
    /// CPP
    Cpp                                                                    = 1240,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 2286,
    /// CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET
    CppImplementationDataTypeContextTarget                                 = 272,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 1761,
    /// CRC-IGNORED
    CrcIgnored                                                             = 1219,
    /// CRC-NOT-SUPPORTED
    CrcNotSupported                                                        = 1404,
    /// CRC-NOT-VALIDATED
    CrcNotValidated                                                        = 1562,
    /// CRC-OPTIONAL
    CrcOptional                                                            = 1215,
    /// CRC-SUPPORTED
    CrcSupported                                                           = 1458,
    /// CRC-VALIDATED
    CrcValidated                                                           = 1641,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 802,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 2155,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 1933,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 1106,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 234,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 2440,
    /// CRYPTO-INTERFACE
    CryptoInterface                                                        = 2301,
    /// CRYPTO-JOB
    CryptoJob                                                              = 1379,
    /// CRYPTO-KEY-MANAGEMENT
    CryptoKeyManagement                                                    = 46,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 2165,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 579,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 2259,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 154,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 823,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 2233,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 1739,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 424,
    /// CRYPTO-NEEDS
    CryptoNeeds                                                            = 1265,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 1351,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 372,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 1157,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 941,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 1773,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 1576,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 395,
    /// CRYPTO-SERVICE-MANAGER
    CryptoServiceManager                                                   = 1797,
    /// CRYPTO-SERVICE-MAPPING
    CryptoServiceMapping                                                   = 2294,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 387,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 1919,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 141,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 1448,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 845,
    /// CS
    Cs                                                                     = 749,
    /// CSERS
    Csers                                                                  = 1873,
    /// CURVE-AXIS
    CurveAxis                                                              = 281,
    /// CURVE_AXIS
    Curveaxis                                                              = 2104,
    /// CUSTOM
    Custom                                                                 = 1098,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 1756,
    /// CVC
    Cvc                                                                    = 586,
    /// CY
    Cy                                                                     = 940,
    /// CYCLE-REPETITION-1
    CycleRepetition1                                                       = 1204,
    /// CYCLE-REPETITION-10
    CycleRepetition10                                                      = 1851,
    /// CYCLE-REPETITION-16
    CycleRepetition16                                                      = 1598,
    /// CYCLE-REPETITION-2
    CycleRepetition2                                                       = 2256,
    /// CYCLE-REPETITION-20
    CycleRepetition20                                                      = 1227,
    /// CYCLE-REPETITION-32
    CycleRepetition32                                                      = 2458,
    /// CYCLE-REPETITION-4
    CycleRepetition4                                                       = 1138,
    /// CYCLE-REPETITION-40
    CycleRepetition40                                                      = 391,
    /// CYCLE-REPETITION-5
    CycleRepetition5                                                       = 11,
    /// CYCLE-REPETITION-50
    CycleRepetition50                                                      = 1549,
    /// CYCLE-REPETITION-64
    CycleRepetition64                                                      = 1834,
    /// CYCLE-REPETITION-8
    CycleRepetition8                                                       = 599,
    /// CYCLIC
    Cyclic                                                                 = 756,
    /// CYCLIC-AND-ON-CHANGE
    CyclicAndOnChange                                                      = 515,
    /// DA
    Da                                                                     = 1128,
    /// DATA-CONSTR
    DataConstr                                                             = 1658,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 743,
    /// DATA-FORMAT-ELEMENT-REFERENCE
    DataFormatElementReference                                             = 543,
    /// DATA-FORMAT-ELEMENT-SCOPE
    DataFormatElementScope                                                 = 355,
    /// DATA-INTERFACE
    DataInterface                                                          = 705,
    /// DATA-PROTOTYPE
    DataPrototype                                                          = 2033,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 700,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 2352,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 2406,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 2139,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 430,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 306,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 1259,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 1782,
    /// DCM-I-PDU
    DcmIPdu                                                                = 1754,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 1360,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 101,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 1400,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 2184,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 2303,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 677,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 758,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 738,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 1091,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 903,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 240,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 1736,
    /// DE
    De                                                                     = 244,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 332,
    /// DEBOUNCE-DATA
    DebounceData                                                           = 405,
    /// DEBUG
    Debug                                                                  = 1603,
    /// DECREASING
    Decreasing                                                             = 1581,
    /// DEDICATED
    Dedicated                                                              = 1540,
    /// DEF-ITEM
    DefItem                                                                = 615,
    /// DEFAULT
    Default                                                                = 1073,
    /// DEFAULT-ERROR-TRACER
    DefaultErrorTracer                                                     = 1844,
    /// DEFAULT-IF-REVISION-UPDATE
    DefaultIfRevisionUpdate                                                = 129,
    /// DEFAULT-IF-UNDEFINED
    DefaultIfUndefined                                                     = 1070,
    /// DEFAULT-MODE
    DefaultMode                                                            = 226,
    /// DEFAULT-TRACE-STATE-DISABLED
    DefaultTraceStateDisabled                                              = 742,
    /// DEFAULT-TRACE-STATE-ENABLED
    DefaultTraceStateEnabled                                               = 724,
    /// DEFAULT-TRIGGER
    DefaultTrigger                                                         = 1302,
    /// DEFERRED
    Deferred                                                               = 1884,
    /// DEFICIT-ROUND-ROBIN
    DeficitRoundRobin                                                      = 502,
    /// DEFINE-BY-IDENTIFIER
    DefineByIdentifier                                                     = 389,
    /// DEFINE-BY-MEMORY-ADDRESS
    DefineByMemoryAddress                                                  = 8,
    /// DEFLATE
    Deflate                                                                = 71,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 1999,
    /// DELETE
    Delete                                                                 = 1894,
    /// DEPENDANT
    Dependant                                                              = 1637,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 292,
    /// DERIVED-FROM
    DerivedFrom                                                            = 2481,
    /// DESCENDANT
    Descendant                                                             = 228,
    /// DESELECTED
    Deselected                                                             = 2183,
    /// DETAILED
    Detailed                                                               = 1515,
    /// DETAILED-BYPASSING-FILTERS
    DetailedBypassingFilters                                               = 703,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 773,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 1477,
    /// DETERMINISTIC-SYNC-INSTANTIATION
    DeterministicSyncInstantiation                                         = 1387,
    /// DETERMINISTIC-SYNC-MASTER
    DeterministicSyncMaster                                                = 570,
    /// DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING
    DeterministicSyncMasterToTimeBaseConsumerMapping                       = 736,
    /// DEVELOPMENT
    Development                                                            = 2343,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 1496,
    /// DEVELOPMENT-ERROR-TRACER
    DevelopmentErrorTracer                                                 = 1940,
    /// DHCPV-4
    Dhcpv4                                                                 = 962,
    /// DHCPV-6
    Dhcpv6                                                                 = 491,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 1427,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 1836,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 1482,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 1784,
    /// DIAG-REQUEST
    DiagRequest                                                            = 2377,
    /// DIAG-RESPONSE
    DiagResponse                                                           = 363,
    /// DIAGNOSTIC-ABSTRACT-ALIAS-EVENT
    DiagnosticAbstractAliasEvent                                           = 1735,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER
    DiagnosticAbstractDataIdentifier                                       = 1221,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE
    DiagnosticAbstractDataIdentifierInterface                              = 1619,
    /// DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE
    DiagnosticAbstractRoutineInterface                                     = 550,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 340,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 754,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 1104,
    /// DIAGNOSTIC-AUTHENTICATION
    DiagnosticAuthentication                                               = 1615,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 2172,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 1742,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 753,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 1037,
    /// DIAGNOSTIC-CAPABILITY-ELEMENT
    DiagnosticCapabilityElement                                            = 2353,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 212,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 1076,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 2138,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 2290,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 604,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 488,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 484,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 1272,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 2252,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 1803,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 452,
    /// DIAGNOSTIC-COMMON-ELEMENT
    DiagnosticCommonElement                                                = 1541,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER
    DiagnosticCommunicationManager                                         = 1524,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 923,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 1335,
    /// DIAGNOSTIC-CONDITION
    DiagnosticCondition                                                    = 1986,
    /// DIAGNOSTIC-CONDITION-GROUP
    DiagnosticConditionGroup                                               = 817,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 977,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 834,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 871,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 1518,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 2154,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 359,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 499,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 2341,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 509,
    /// DIAGNOSTIC-DATA-BY-IDENTIFIER
    DiagnosticDataByIdentifier                                             = 330,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 1974,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 44,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 107,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 1425,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 805,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 872,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 1131,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 1072,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 1339,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 877,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 619,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 275,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 513,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE
    DiagnosticDoIpEntityIdentificationInterface                            = 1213,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 1092,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 61,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 2190,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 2130,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 1,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 2175,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 689,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 211,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 1804,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 1053,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 1959,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 441,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 221,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 2366,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 1352,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 67,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 1004,
    /// DIAGNOSTIC-ENV-MODE-ELEMENT
    DiagnosticEnvModeElement                                               = 2200,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 1749,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 867,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 1431,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 2198,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 2148,
    /// DIAGNOSTIC-EVENT-MANAGER
    DiagnosticEventManager                                                 = 90,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 566,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 1319,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 1921,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 1930,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 263,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 1772,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 486,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 520,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 519,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 2101,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 1103,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 2014,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 318,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 1293,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 1082,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 588,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 1214,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 2016,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 1880,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 2064,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 1406,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 2171,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 1142,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 1069,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 592,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 1597,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 2076,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 1513,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 224,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 701,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 1980,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 285,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 777,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 2373,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 10,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 2207,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 1979,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 189,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 2423,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 1150,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 1315,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 1870,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 1459,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 633,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 1530,
    /// DIAGNOSTIC-LOG-AND-TRACE
    DiagnosticLogAndTrace                                                  = 1353,
    /// DIAGNOSTIC-MAPPING
    DiagnosticMapping                                                      = 1714,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 2297,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 1684,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 1488,
    /// DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS
    DiagnosticMemoryAddressableRangeAccess                                 = 1451,
    /// DIAGNOSTIC-MEMORY-BY-ADDRESS
    DiagnosticMemoryByAddress                                              = 255,
    /// DIAGNOSTIC-MEMORY-DESTINATION
    DiagnosticMemoryDestination                                            = 1001,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 897,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 1892,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 354,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 1050,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 468,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 1595,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 1962,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 374,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 496,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 1889,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 921,
    /// DIAGNOSTIC-PARAMETER-ELEMENT
    DiagnosticParameterElement                                             = 490,
    /// DIAGNOSTIC-PARAMETER-IDENT
    DiagnosticParameterIdent                                               = 87,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 890,
    /// DIAGNOSTIC-PORT-INTERFACE
    DiagnosticPortInterface                                                = 2162,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 419,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 860,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 2327,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 2120,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 1638,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 1906,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 1963,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 150,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 559,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 1029,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 1632,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 547,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 1084,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 69,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 106,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 215,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 2412,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 2116,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 1572,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 125,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 1547,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 168,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 417,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 1653,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 590,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 1010,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE
    DiagnosticRequestFileTransferInterface                                 = 776,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 2399,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 209,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 2307,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 770,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 385,
    /// DIAGNOSTIC-REQUEST-ROUTINE-RESULTS
    DiagnosticRequestRoutineResults                                        = 704,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 531,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 460,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 1816,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 896,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 637,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 2457,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 204,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 1907,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 655,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 2107,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 1264,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 1443,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 1981,
    /// DIAGNOSTIC-ROUTINE-SUBFUNCTION
    DiagnosticRoutineSubfunction                                           = 506,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 1710,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 1643,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 2382,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 2268,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 912,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 164,
    /// DIAGNOSTIC-SERVICE-CLASS
    DiagnosticServiceClass                                                 = 1147,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 846,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 1083,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 1471,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 1257,
    /// DIAGNOSTIC-SERVICE-INSTANCE
    DiagnosticServiceInstance                                              = 2088,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 1250,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 2380,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 516,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 1078,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 2468,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 1280,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 1361,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 1045,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE
    DiagnosticSovdAuthorizationInterface                                   = 957,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING
    DiagnosticSovdAuthorizationPortMapping                                 = 1990,
    /// DIAGNOSTIC-SOVD-LOCK
    DiagnosticSovdLock                                                     = 55,
    /// DIAGNOSTIC-SOVD-PORT-INTERFACE
    DiagnosticSovdPortInterface                                            = 1123,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE
    DiagnosticSovdProximityChallengeInterface                              = 1985,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING
    DiagnosticSovdProximityChallengePortMapping                            = 2158,
    /// DIAGNOSTIC-START-ROUTINE
    DiagnosticStartRoutine                                                 = 924,
    /// DIAGNOSTIC-STOP-ROUTINE
    DiagnosticStopRoutine                                                  = 1209,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 1332,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 1282,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 184,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 717,
    /// DIAGNOSTIC-SW-MAPPING
    DiagnosticSwMapping                                                    = 955,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 1551,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 2316,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 1303,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 470,
    /// DIAGNOSTIC-TROUBLE-CODE
    DiagnosticTroubleCode                                                  = 1791,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 1413,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 1953,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 38,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 1201,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 458,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 336,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 78,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 2398,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 1186,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 1295,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 2124,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 402,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 2419,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 331,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 344,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 247,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 3,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 2274,
    /// DISABLE
    Disable                                                                = 1132,
    /// DLNA
    Dlna                                                                   = 2322,
    /// DLT-APPLICATION
    DltApplication                                                         = 1622,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 1882,
    /// DLT-ARGUMENT
    DltArgument                                                            = 522,
    /// DLT-CONTEXT
    DltContext                                                             = 412,
    /// DLT-ECU
    DltEcu                                                                 = 351,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 1230,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 1491,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 1466,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 885,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 1237,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 1504,
    /// DLT-MESSAGE
    DltMessage                                                             = 597,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 2181,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 1064,
    /// DO-IP
    DoIp                                                                   = 2395,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 1483,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 1770,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 1943,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 2428,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 2229,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 2374,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 937,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 2357,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 1316,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 925,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 882,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 2089,
    /// DO-IP-SERVICE-NEEDS
    DoIpServiceNeeds                                                       = 230,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 981,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 2434,
    /// DOCUMENTATION
    Documentation                                                          = 104,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 772,
    /// DOES-NOT-REPORT-EXECUTION-STATE
    DoesNotReportExecutionState                                            = 214,
    /// DOES-NOT-SUPPORT-BUFFER-LOCKING
    DoesNotSupportBufferLocking                                            = 429,
    /// DOES-NOT-USE-LOGGING
    DoesNotUseLogging                                                      = 895,
    /// DOMAIN-PARTICIPANT-USER-DATA-QOS
    DomainParticipantUserDataQos                                           = 638,
    /// DONT-INVALIDATE
    DontInvalidate                                                         = 1009,
    /// DROP
    Drop                                                                   = 2309,
    /// DROP-FRAME
    DropFrame                                                              = 1612,
    /// DROP-UNTAGGED
    DropUntagged                                                           = 2456,
    /// DSA
    Dsa                                                                    = 2178,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 1502,
    /// DYNAMIC-PART-TRIGGER
    DynamicPartTrigger                                                     = 43,
    /// DZ
    Dz                                                                     = 697,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 328,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 782,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 1197,
    /// ECC
    Ecc                                                                    = 438,
    /// ECU
    Ecu                                                                    = 751,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 1671,
    /// ECU-INSTANCE
    EcuInstance                                                            = 1712,
    /// ECU-MANAGER
    EcuManager                                                             = 766,
    /// ECU-MAPPING
    EcuMapping                                                             = 1766,
    /// ECU-PARTITION
    EcuPartition                                                           = 82,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 1902,
    /// ECU-TIMING
    EcuTiming                                                              = 734,
    /// ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF
    EcucAbstractExternalReferenceDef                                       = 667,
    /// ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF
    EcucAbstractInternalReferenceDef                                       = 1905,
    /// ECUC-ABSTRACT-REFERENCE-DEF
    EcucAbstractReferenceDef                                               = 529,
    /// ECUC-ABSTRACT-STRING-PARAM-DEF
    EcucAbstractStringParamDef                                             = 828,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 795,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 2269,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 892,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 2057,
    /// ECUC-COMMON-ATTRIBUTES
    EcucCommonAttributes                                                   = 293,
    /// ECUC-CONTAINER-DEF
    EcucContainerDef                                                       = 1435,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 1262,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 789,
    /// ECUC-DEFINITION-ELEMENT
    EcucDefinitionElement                                                  = 1994,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 775,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 1047,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 1941,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 2029,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 2031,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 1818,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 1976,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 357,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 680,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 2414,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 1510,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 1140,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 873,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 607,
    /// ECUC-PARAMETER-DEF
    EcucParameterDef                                                       = 1101,
    /// ECUC-QUERY
    EcucQuery                                                              = 1252,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 598,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 2433,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 1450,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 1323,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 876,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 2134,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 1033,
    /// EDGE-NODE
    EdgeNode                                                               = 45,
    /// EID-USE-API
    EidUseApi                                                              = 2004,
    /// EID-USE-CONFIG-VALUE
    EidUseConfigValue                                                      = 320,
    /// EID-USE-MAC
    EidUseMac                                                              = 582,
    /// EL
    El                                                                     = 2247,
    /// EMISSION-RELATED-DTC
    EmissionRelatedDtc                                                     = 1149,
    /// EN
    En                                                                     = 1811,
    /// ENABLE
    Enable                                                                 = 961,
    /// ENABLED
    Enabled                                                                = 290,
    /// ENCRYPT-AND-SIGN
    EncryptAndSign                                                         = 1819,
    /// ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION
    EncryptAndSignWithOriginAuthentication                                 = 111,
    /// ENCRYPTION
    Encryption                                                             = 1849,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 337,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 1588,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 859,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 716,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 1269,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 2333,
    /// ENHANCED
    Enhanced                                                               = 1942,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 367,
    /// EO
    Eo                                                                     = 2312,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 899,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 1389,
    /// EOC-EXECUTABLE-ENTITY-REF-ABSTRACT
    EocExecutableEntityRefAbstract                                         = 1928,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 1586,
    /// EPS
    Eps                                                                    = 2306,
    /// EQUAL
    Equal                                                                  = 1725,
    /// ERROR
    Error                                                                  = 1607,
    /// ERROR-CORRECTION
    ErrorCorrection                                                        = 1760,
    /// ERROR-DETECTION
    ErrorDetection                                                         = 1479,
    /// ERROR-TRACER
    ErrorTracer                                                            = 996,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 1855,
    /// ES
    Es                                                                     = 874,
    /// ESP
    Esp                                                                    = 1679,
    /// ET
    Et                                                                     = 1672,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 630,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 1475,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 81,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 822,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 1701,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 2079,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 741,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 251,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 521,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 1371,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 561,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 900,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 686,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 1938,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 1521,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 2453,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 1110,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 2486,
    /// EU
    Eu                                                                     = 526,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 1449,
    /// EVAP
    Evap                                                                   = 539,
    /// EVENT-ACCEPTANCE-DISABLED
    EventAcceptanceDisabled                                                = 793,
    /// EVENT-ACCEPTANCE-ENABLED
    EventAcceptanceEnabled                                                 = 482,
    /// EVENT-COMBINATION-ON-RETRIEVAL
    EventCombinationOnRetrieval                                            = 1706,
    /// EVENT-COMBINATION-ON-STORAGE
    EventCombinationOnStorage                                              = 1815,
    /// EVENT-HANDLER
    EventHandler                                                           = 2131,
    /// EVENT-MAPPING
    EventMapping                                                           = 57,
    /// EVENT-STORAGE-DISABLED
    EventStorageDisabled                                                   = 601,
    /// EVENT-STORAGE-ENABLED
    EventStorageEnabled                                                    = 1527,
    /// EVENT-TRIGGERING-CONSTRAINT
    EventTriggeringConstraint                                              = 2186,
    /// EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE
    EventWindowCurrentAndFollowingCycle                                    = 2254,
    /// EVENT-WINDOW-CURRENT-CYCLE
    EventWindowCurrentCycle                                                = 1497,
    /// EVENT-WINDOW-INFINITE
    EventWindowInfinite                                                    = 674,
    /// EXACT-OR-ANY-MINOR-VERSION
    ExactOrAnyMinorVersion                                                 = 2263,
    /// EXAMPLE
    Example                                                                = 1473,
    /// EXCLUDE-FROM-FLASH
    ExcludeFromFlash                                                       = 1486,
    /// EXCLUSIVE
    Exclusive                                                              = 1947,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 1993,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 1229,
    /// EXECUTABLE
    Executable                                                             = 1660,
    /// EXECUTABLE-ENTITY
    ExecutableEntity                                                       = 1704,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 1657,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 1321,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 47,
    /// EXECUTE
    Execute                                                                = 1439,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 844,
    /// EXECUTION-TIME
    ExecutionTime                                                          = 2035,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 2470,
    /// EXERCISE
    Exercise                                                               = 809,
    /// EXPLICIT
    Explicit                                                               = 1874,
    /// EXTENDED
    Extended                                                               = 1468,
    /// EXTERNAL-REPLACEMENT
    ExternalReplacement                                                    = 2040,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 1573,
    /// EXTERNAL-TRIGGERING-POINT-IDENT
    ExternalTriggeringPointIdent                                           = 1116,
    /// FA
    Fa                                                                     = 1146,
    /// FAILURE-AND-SUCCESS
    FailureAndSuccess                                                      = 558,
    /// FAILURE-ONLY
    FailureOnly                                                            = 2075,
    /// FALSE
    False                                                                  = 325,
    /// FAST-FLASHING-MODE
    FastFlashingMode                                                       = 1991,
    /// FATAL
    Fatal                                                                  = 528,
    /// FAULT
    Fault                                                                  = 1674,
    /// FDC-THRESHOLD
    FdcThreshold                                                           = 1445,
    /// FI
    Fi                                                                     = 2397,
    /// FIBEX-ELEMENT
    FibexElement                                                           = 968,
    /// FIELD
    Field                                                                  = 388,
    /// FIELD-MAPPING
    FieldMapping                                                           = 1248,
    /// FILE
    File                                                                   = 2479,
    /// FILTERED
    Filtered                                                               = 787,
    /// FINISH
    Finish                                                                 = 273,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 943,
    /// FIRE-AND-FORGET-METHOD-MAPPING
    FireAndForgetMethodMapping                                             = 2119,
    /// FIREWALL-RULE
    FirewallRule                                                           = 661,
    /// FIREWALL-STATE-SWITCH-INTERFACE
    FirewallStateSwitchInterface                                           = 1939,
    /// FIRST-CONTAINED-TRIGGER
    FirstContainedTrigger                                                  = 291,
    /// FIRST-TO-SECOND
    FirstToSecond                                                          = 1165,
    /// FIT-TO-PAGE
    FitToPage                                                              = 1025,
    /// FIT-TO-TEXT
    FitToText                                                              = 1778,
    /// FIX-AXIS
    FixAxis                                                                = 568,
    /// FIXED
    Fixed                                                                  = 2400,
    /// FIXED-SIZE
    FixedSize                                                              = 1507,
    /// FIX_AXIS
    Fixaxis                                                                = 431,
    /// FJ
    Fj                                                                     = 1495,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 948,
    /// FLAT-MAP
    FlatMap                                                                = 1972,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 1465,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 151,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 1538,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 2106,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 1667,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 798,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 744,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 1867,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 36,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 2087,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 2467,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 2012,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 1621,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 483,
    /// FLOAT
    Float                                                                  = 2360,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 1390,
    /// FM-FEATURE
    FmFeature                                                              = 1325,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 1011,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 1151,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 1297,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 1143,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 580,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 2276,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 1023,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 392,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 1965,
    /// FO
    Fo                                                                     = 1802,
    /// FOR-ALL
    ForAll                                                                 = 316,
    /// FORGET
    Forget                                                                 = 2248,
    /// FORWARD-AS-IS
    ForwardAsIs                                                            = 2367,
    /// FR
    Fr                                                                     = 929,
    /// FRAME
    Frame                                                                  = 933,
    /// FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION
    FrameEthernetQueuedForTransmission                                     = 1746,
    /// FRAME-ETHERNET-RECEIVED-BY-IF
    FrameEthernetReceivedByIf                                              = 1195,
    /// FRAME-ETHERNET-RECEIVED-ON-BUS
    FrameEthernetReceivedOnBus                                             = 347,
    /// FRAME-ETHERNET-SENT-ON-BUS
    FrameEthernetSentOnBus                                                 = 958,
    /// FRAME-PORT
    FramePort                                                              = 60,
    /// FRAME-QUEUED-FOR-TRANSMISSION
    FrameQueuedForTransmission                                             = 2149,
    /// FRAME-RECEIVED-BY-IF
    FrameReceivedByIf                                                      = 1611,
    /// FRAME-TRANSMITTED-ON-BUS
    FrameTransmittedOnBus                                                  = 2226,
    /// FRAME-TRIGGERING
    FrameTriggering                                                        = 1363,
    /// FULL
    Full                                                                   = 2053,
    /// FULL-DUPLEX-MODE
    FullDuplexMode                                                         = 998,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 358,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 1901,
    /// FUNCTION-GROUP-STATE-TO-NM-HANDLE
    FunctionGroupStateToNmHandle                                           = 1957,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 276,
    /// FUNCTION-INHIBITION-MANAGER
    FunctionInhibitionManager                                              = 193,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 1065,
    /// FUNCTIONAL
    Functional                                                             = 866,
    /// FUNCTIONAL-ADDRESS
    FunctionalAddress                                                      = 1798,
    /// FUNCTIONAL-CAN-FD
    FunctionalCanFd                                                        = 426,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING
    FunctionalClusterInteractsWithFunctionalClusterMapping                 = 2023,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING
    FunctionalClusterInteractsWithPersistencyDeploymentMapping             = 1859,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 1708,
    /// FY
    Fy                                                                     = 1992,
    /// GA
    Ga                                                                     = 2091,
    /// GATEWAY
    Gateway                                                                = 1559,
    /// GD
    Gd                                                                     = 433,
    /// GENERAL-PARAMETER
    GeneralParameter                                                       = 2235,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 445,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 1503,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 1225,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 2378,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 2218,
    /// GET
    Get                                                                    = 494,
    /// GETTER
    Getter                                                                 = 992,
    /// GETTER-SETTER
    GetterSetter                                                           = 1751,
    /// GIF
    Gif                                                                    = 966,
    /// GL
    Gl                                                                     = 2311,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 250,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 33,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 1027,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 2239,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 1042,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 249,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 1835,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 1181,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 1108,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 75,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 143,
    /// GLOBAL-TIME-MASTER
    GlobalTimeMaster                                                       = 5,
    /// GLOBAL-TIME-SLAVE
    GlobalTimeSlave                                                        = 1373,
    /// GN
    Gn                                                                     = 1914,
    /// GRANT
    Grant                                                                  = 2142,
    /// GRANT-DESIGN
    GrantDesign                                                            = 500,
    /// GROSS
    Gross                                                                  = 1932,
    /// GU
    Gu                                                                     = 225,
    /// GZIP
    Gzip                                                                   = 829,
    /// HA
    Ha                                                                     = 1164,
    /// HALF-DUPLEX-MODE
    HalfDuplexMode                                                         = 835,
    /// HARDWARE-TEST-MANAGER
    HardwareTestManager                                                    = 595,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 1388,
    /// HEAD
    Head                                                                   = 2289,
    /// HEALTH-CHANNEL
    HealthChannel                                                          = 518,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 335,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 1821,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 702,
    /// HEAP-USAGE
    HeapUsage                                                              = 2192,
    /// HI
    Hi                                                                     = 884,
    /// HIERARCHICAL-EOC
    HierarchicalEoc                                                        = 1616,
    /// HIGH
    High                                                                   = 1781,
    /// HINT
    Hint                                                                   = 369,
    /// HOOK
    Hook                                                                   = 606,
    /// HOST-PORT
    HostPort                                                               = 2168,
    /// HR
    Hr                                                                     = 852,
    /// HU
    Hu                                                                     = 1457,
    /// HUB
    Hub                                                                    = 2262,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 314,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 1587,
    /// HW-CATEGORY
    HwCategory                                                             = 2141,
    /// HW-DESCRIPTION-ENTITY
    HwDescriptionEntity                                                    = 919,
    /// HW-ELEMENT
    HwElement                                                              = 261,
    /// HW-PIN
    HwPin                                                                  = 1036,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 229,
    /// HW-TYPE
    HwType                                                                 = 2396,
    /// HY
    Hy                                                                     = 1861,
    /// I-4-G
    I4G                                                                    = 2265,
    /// I-PDU
    IPdu                                                                   = 262,
    /// I-PDU-PORT
    IPduPort                                                               = 1176,
    /// I-PDU-RECEIVED-BY-COM
    IPduReceivedByCom                                                      = 1238,
    /// I-PDU-SENT-TO-IF
    IPduSentToIf                                                           = 1158,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 1019,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 280,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 2241,
    /// I-SIGNAL
    ISignal                                                                = 1923,
    /// I-SIGNAL-AVAILABLE-FOR-RTE
    ISignalAvailableForRte                                                 = 538,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 1596,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 105,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 311,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 1520,
    /// I-SIGNAL-SENT-TO-COM
    ISignalSentToCom                                                       = 1707,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 1418,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 1575,
    /// IA
    Ia                                                                     = 855,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 462,
    /// ICMP
    Icmp                                                                   = 1857,
    /// ICV-IGNORED
    IcvIgnored                                                             = 537,
    /// ICV-NOT-SUPPORTED
    IcvNotSupported                                                        = 609,
    /// ICV-NOT-VERIFIED
    IcvNotVerified                                                         = 1305,
    /// ICV-OPTIONAL
    IcvOptional                                                            = 2404,
    /// ICV-SUPPORTED
    IcvSupported                                                           = 1062,
    /// ICV-VERIFIED
    IcvVerified                                                            = 1952,
    /// IDENT-CAPTION
    IdentCaption                                                           = 1088,
    /// IDENTIFIABLE
    Identifiable                                                           = 1022,
    /// IDS-COMMON-ELEMENT
    IdsCommonElement                                                       = 1409,
    /// IDS-DESIGN
    IdsDesign                                                              = 2298,
    /// IDS-MAPPING
    IdsMapping                                                             = 785,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 333,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 2389,
    /// IDS-PLATFORM-INSTANTIATION
    IdsPlatformInstantiation                                               = 811,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 1525,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 15,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 1813,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 313,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 2206,
    /// IE
    Ie                                                                     = 1357,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 2448,
    /// IEEE802-11P
    Ieee80211p                                                             = 1492,
    /// IEEE802-1AS
    Ieee8021as                                                             = 1460,
    /// IEEE802-1AS-AUTOSAR
    Ieee8021asAutosar                                                      = 1456,
    /// IGNITION
    Ignition                                                               = 245,
    /// IGNORE
    Ignore                                                                 = 1955,
    /// IK
    Ik                                                                     = 1243,
    /// IMMEDIATE
    Immediate                                                              = 2393,
    /// IMMEDIATELY
    Immediately                                                            = 841,
    /// IMPLEMENTATION
    Implementation                                                         = 341,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 1481,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 684,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 1499,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 731,
    /// IMPLEMENTATION-PROPS
    ImplementationProps                                                    = 1978,
    /// IN
    In                                                                     = 2133,
    /// INCREASING
    Increasing                                                             = 1579,
    /// INDICATE
    Indicate                                                               = 1626,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 946,
    /// INDIVIDUAL
    Individual                                                             = 600,
    /// INFINITE
    Infinite                                                               = 1744,
    /// INFINITE-TIME-TO-RESPONSE
    InfiniteTimeToResponse                                                 = 1582,
    /// INFO
    Info                                                                   = 548,
    /// INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE
    InheritedFromArrayElementTypeSize                                      = 2408,
    /// INIT-EVENT
    InitEvent                                                              = 978,
    /// INLINE
    Inline                                                                 = 202,
    /// INLINE-CONDITIONAL
    InlineConditional                                                      = 1420,
    /// INOUT
    Inout                                                                  = 2187,
    /// INSTALL
    Install                                                                = 858,
    /// INSTANCE-ID
    InstanceId                                                             = 6,
    /// INSTRUCTION
    Instruction                                                            = 1519,
    /// INTER-PARTITION-INTRA-ECU
    InterPartitionIntraEcu                                                 = 663,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 432,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 1441,
    /// INTERGRITY-AND-CONFIDENTIALITY
    IntergrityAndConfidentiality                                           = 1493,
    /// INTERGRITY-WITHOUT-CONFIDENTIALITY
    IntergrityWithoutConfidentiality                                       = 487,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 1585,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 730,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 191,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 1278,
    /// INTERRUPT
    Interrupt                                                              = 1553,
    /// INTERRUPT-CAT-1
    InterruptCat1                                                          = 1970,
    /// INTERRUPT-CAT-2
    InterruptCat2                                                          = 546,
    /// INTRUSION-DETECTION-SECURITY-MANAGEMENT
    IntrusionDetectionSecurityManagement                                   = 1917,
    /// INVALID
    Invalid                                                                = 2037,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 384,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 1968,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 1485,
    /// IP-SEC-RULE
    IpSecRule                                                              = 1391,
    /// IPSEC
    Ipsec                                                                  = 197,
    /// IS
    Is                                                                     = 618,
    /// IS-EQUAL
    IsEqual                                                                = 1987,
    /// IS-EXPIRED
    IsExpired                                                              = 435,
    /// IS-FAILED
    IsFailed                                                               = 14,
    /// IS-GREATER-OR-EQUAL
    IsGreaterOrEqual                                                       = 1239,
    /// IS-GREATER-THAN
    IsGreaterThan                                                          = 1344,
    /// IS-GREATER-THAN-OR-EQUAL
    IsGreaterThanOrEqual                                                   = 1330,
    /// IS-LESS-OR-EQUAL
    IsLessOrEqual                                                          = 120,
    /// IS-LESS-THAN
    IsLessThan                                                             = 964,
    /// IS-LESS-THAN-OR-EQUAL
    IsLessThanOrEqual                                                      = 1063,
    /// IS-NOT-EQUAL
    IsNotEqual                                                             = 1097,
    /// IS-NOT-RELEVANT
    IsNotRelevant                                                          = 2441,
    /// IS-OK
    IsOk                                                                   = 362,
    /// IS-RELEVANT
    IsRelevant                                                             = 1203,
    /// IS-STOPPED
    IsStopped                                                              = 2487,
    /// ISO
    Iso                                                                    = 2430,
    /// ISO-11992--4
    Iso119924                                                              = 165,
    /// ISO-14229--1
    Iso142291                                                              = 1281,
    /// ISO-15031--6
    Iso150316                                                              = 1822,
    /// ISO-6
    Iso6                                                                   = 2234,
    /// IT
    It                                                                     = 1005,
    /// ITALIC
    Italic                                                                 = 1051,
    /// IW
    Iw                                                                     = 2159,
    /// J-1939
    J1939                                                                  = 56,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 1927,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 183,
    /// J-1939-DCM
    J1939Dcm                                                               = 422,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 1100,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 1034,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 1012,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 1951,
    /// J-1939-REQUEST-MANAGER
    J1939RequestManager                                                    = 2300,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 2115,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 175,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 1661,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 23,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 657,
    /// JA
    Ja                                                                     = 1105,
    /// JAVA
    Java                                                                   = 666,
    /// JI
    Ji                                                                     = 476,
    /// JPG
    Jpg                                                                    = 1673,
    /// JUSTIFY
    Justify                                                                = 117,
    /// JW
    Jw                                                                     = 951,
    /// KA
    Ka                                                                     = 2351,
    /// KEEP
    Keep                                                                   = 2442,
    /// KEEP-EXISTING
    KeepExisting                                                           = 719,
    /// KEY-DERIVATION
    KeyDerivation                                                          = 1107,
    /// KEY-SERVER
    KeyServer                                                              = 2203,
    /// KEY-STORAGE
    KeyStorage                                                             = 2055,
    /// KEYWORD
    Keyword                                                                = 1542,
    /// KEYWORD-SET
    KeywordSet                                                             = 938,
    /// KK
    Kk                                                                     = 2185,
    /// KL
    Kl                                                                     = 2320,
    /// KM
    Km                                                                     = 768,
    /// KN
    Kn                                                                     = 1775,
    /// KO
    Ko                                                                     = 304,
    /// KS
    Ks                                                                     = 640,
    /// KU
    Ku                                                                     = 180,
    /// KY
    Ky                                                                     = 888,
    /// LA
    La                                                                     = 1180,
    /// LAND
    Land                                                                   = 1734,
    /// LAST-FAILED
    LastFailed                                                             = 1550,
    /// LAST-IS-BEST
    LastIsBest                                                             = 1000,
    /// LAST-MODE
    LastMode                                                               = 2275,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 203,
    /// LEAF-OF-TARGET-CONTAINER
    LeafOfTargetContainer                                                  = 132,
    /// LEFT
    Left                                                                   = 628,
    /// LEGACY
    Legacy                                                                 = 2304,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 1627,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 926,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 1021,
    /// LIMIT-TO-PAGE
    LimitToPage                                                            = 127,
    /// LIMIT-TO-TEXT
    LimitToText                                                            = 266,
    /// LIN-CLUSTER
    LinCluster                                                             = 295,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 806,
    /// LIN-COMMUNICATION-CONTROLLER
    LinCommunicationController                                             = 763,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 1946,
    /// LIN-FRAME
    LinFrame                                                               = 30,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 970,
    /// LIN-MASTER
    LinMaster                                                              = 1895,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 459,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 1055,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 1364,
    /// LIN-SLAVE
    LinSlave                                                               = 862,
    /// LIN-SLAVE-CONFIG-IDENT
    LinSlaveConfigIdent                                                    = 904,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 116,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 1285,
    /// LIN-TP-NODE
    LinTpNode                                                              = 2065,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 807,
    /// LINK
    Link                                                                   = 1687,
    /// LINK-LOCAL
    LinkLocal                                                              = 1372,
    /// LINK-LOCAL--DOIP
    LinkLocalDoip                                                          = 169,
    /// LINK-TIME
    LinkTime                                                               = 2043,
    /// LINKER
    Linker                                                                 = 2123,
    /// LISTEN
    Listen                                                                 = 870,
    /// LN
    Ln                                                                     = 270,
    /// LO
    Lo                                                                     = 2008,
    /// LOCAL
    Local                                                                  = 130,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 85,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 2387,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 973,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 1858,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 732,
    /// LOGICAL-AND
    LogicalAnd                                                             = 2283,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 2096,
    /// LOGICAL-OR
    LogicalOr                                                              = 2264,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 999,
    /// LONG-HEADER
    LongHeader                                                             = 478,
    /// LOW
    Low                                                                    = 997,
    /// LOWER-12-BIT
    Lower12Bit                                                             = 1172,
    /// LOWER-8-BIT
    Lower8Bit                                                              = 1785,
    /// LT
    Lt                                                                     = 1046,
    /// LT-AFFECTS-PB
    LtAffectsPb                                                            = 2261,
    /// LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING
    LtMessageCollectionToPortPrototypeMapping                              = 673,
    /// LTS-13
    Lts13                                                                  = 2006,
    /// LV
    Lv                                                                     = 1721,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 73,
    /// MAC-SEC-GLOBAL-KAY-PROPS
    MacSecGlobalKayProps                                                   = 2230,
    /// MAC-SEC-KAY-PARTICIPANT
    MacSecKayParticipant                                                   = 621,
    /// MAC-SEC-PARTICIPANT-SET
    MacSecParticipantSet                                                   = 2359,
    /// MACHINE
    Machine                                                                = 63,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 1567,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 498,
    /// MACHINE-TIMING
    MachineTiming                                                          = 2038,
    /// MACRO
    Macro                                                                  = 1577,
    /// MAINTENANCE-ONLY
    MaintenanceOnly                                                        = 1988,
    /// MALFUNCTION
    Malfunction                                                            = 960,
    /// MANUFACTURING
    Manufacturing                                                          = 187,
    /// MAPPING-SCOPE-CORE
    MappingScopeCore                                                       = 2021,
    /// MAPPING-SCOPE-ECU
    MappingScopeEcu                                                        = 260,
    /// MAPPING-SCOPE-PARTITION
    MappingScopePartition                                                  = 557,
    /// MASEKD-NEW-EQUALS-MASKED-OLD
    MasekdNewEqualsMaskedOld                                               = 1173,
    /// MASEKD-NEW-EQUALS-X
    MasekdNewEqualsX                                                       = 2413,
    /// MASKED-NEW-DIFFERS-MASKED-OLD
    MaskedNewDiffersMaskedOld                                              = 2201,
    /// MASKED-NEW-DIFFERS-X
    MaskedNewDiffersX                                                      = 2356,
    /// MASKED-NEW-EQUALS-MASKED-OLD
    MaskedNewEqualsMaskedOld                                               = 2330,
    /// MASKED-NEW-EQUALS-X
    MaskedNewEqualsX                                                       = 1677,
    /// MASTER
    Master                                                                 = 467,
    /// MASTER-ECU
    MasterEcu                                                              = 2129,
    /// MAX
    Max                                                                    = 1750,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 136,
    /// MC-FUNCTION
    McFunction                                                             = 699,
    /// MC-GROUP
    McGroup                                                                = 646,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 350,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 2044,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 711,
    /// MEASUREMENT-POINT
    MeasurementPoint                                                       = 616,
    /// MEDIUM
    Medium                                                                 = 2027,
    /// MEMORY-SECTION
    MemorySection                                                          = 818,
    /// MEMORY-USAGE
    MemoryUsage                                                            = 1160,
    /// METHOD-MAPPING
    MethodMapping                                                          = 1313,
    /// MG
    Mg                                                                     = 2132,
    /// MI
    Mi                                                                     = 1762,
    /// MIDDLE
    Middle                                                                 = 296,
    /// MIN
    Min                                                                    = 179,
    /// MINIMUM-MINOR-VERSION
    MinimumMinorVersion                                                    = 1216,
    /// MIXED
    Mixed                                                                  = 2249,
    /// MIXED-29-BIT
    Mixed29Bit                                                             = 161,
    /// MK
    Mk                                                                     = 2386,
    /// ML
    Ml                                                                     = 2093,
    /// MN
    Mn                                                                     = 1700,
    /// MO
    Mo                                                                     = 2325,
    /// MODE-ACCESS-POINT-IDENT
    ModeAccessPointIdent                                                   = 762,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 1039,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 236,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 907,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 1048,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 969,
    /// MODE-DECLARATION-REQUESTED
    ModeDeclarationRequested                                               = 1258,
    /// MODE-DECLARATION-SWITCH-COMPLETED
    ModeDeclarationSwitchCompleted                                         = 1891,
    /// MODE-DECLARATION-SWITCH-INITIATED
    ModeDeclarationSwitchInitiated                                         = 1193,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 1948,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 287,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 1139,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 2403,
    /// MODE-TRANSITION
    ModeTransition                                                         = 58,
    /// MODELED
    Modeled                                                                = 1648,
    /// MONITOR-MODE
    MonitorMode                                                            = 2345,
    /// MONO
    Mono                                                                   = 397,
    /// MONOTONOUS
    Monotonous                                                             = 1753,
    /// MOST-SIGNIFICANT-BYTE-FIRST
    MostSignificantByteFirst                                               = 1995,
    /// MOST-SIGNIFICANT-BYTE-LAST
    MostSignificantByteLast                                                = 1614,
    /// MR
    Mr                                                                     = 2034,
    /// MS
    Ms                                                                     = 690,
    /// MT
    Mt                                                                     = 779,
    /// MULTICORE-REENTRANT
    MulticoreReentrant                                                     = 1876,
    /// MULTILANGUAGE-REFERRABLE
    MultilanguageReferrable                                                = 935,
    /// MULTIPLE
    Multiple                                                               = 1937,
    /// MULTIPLE-OCCURRENCES
    MultipleOccurrences                                                    = 1783,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 2405,
    /// MY
    My                                                                     = 1294,
    /// N-PDU
    NPdu                                                                   = 196,
    /// NA
    Na                                                                     = 2080,
    /// NAND
    Nand                                                                   = 1774,
    /// NE
    Ne                                                                     = 166,
    /// NET
    Net                                                                    = 808,
    /// NETWORK
    Network                                                                = 1879,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 833,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 1289,
    /// NETWORK-REPRESENTATION-FROM-COM-SPEC
    NetworkRepresentationFromComSpec                                       = 2422,
    /// NEVER
    Never                                                                  = 971,
    /// NEW-IS-DIFFERENT
    NewIsDifferent                                                         = 918,
    /// NEW-IS-EQUAL
    NewIsEqual                                                             = 1040,
    /// NEW-IS-GREATER
    NewIsGreater                                                           = 1096,
    /// NEW-IS-GREATER-OR-EQUAL
    NewIsGreaterOrEqual                                                    = 2144,
    /// NEW-IS-LESS
    NewIsLess                                                              = 1654,
    /// NEW-IS-LESS-OR-EQUAL
    NewIsLessOrEqual                                                       = 1713,
    /// NEW-IS-OUTSIDE
    NewIsOutside                                                           = 112,
    /// NEW-IS-WITHIN
    NewIsWithin                                                            = 118,
    /// NEWLINE
    Newline                                                                = 1831,
    /// NEWLINE-IF-NECESSARY
    NewlineIfNecessary                                                     = 1077,
    /// NFOLD
    Nfold                                                                  = 95,
    /// NL
    Nl                                                                     = 1066,
    /// NM-CLUSTER
    NmCluster                                                              = 565,
    /// NM-CONFIG
    NmConfig                                                               = 398,
    /// NM-ECU
    NmEcu                                                                  = 1526,
    /// NM-HANDLE-ACTIVE-TO-FUNCTION-GROUP-STATE
    NmHandleActiveToFunctionGroupState                                     = 879,
    /// NM-HANDLE-INACTIVE-TO-FUNCTION-GROUP-STATE
    NmHandleInactiveToFunctionGroupState                                   = 658,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 2009,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 1702,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 612,
    /// NM-NODE
    NmNode                                                                 = 360,
    /// NM-PDU
    NmPdu                                                                  = 216,
    /// NO
    No                                                                     = 1306,
    /// NO-ACK
    NoAck                                                                  = 503,
    /// NO-AFFECT
    NoAffect                                                               = 1964,
    /// NO-BOOT
    NoBoot                                                                 = 2392,
    /// NO-BREAK
    NoBreak                                                                = 269,
    /// NO-CHECKPOINT-SUPERVISION
    NoCheckpointSupervision                                                = 2232,
    /// NO-CONSISTENCY-MECHANISM
    NoConsistencyMechanism                                                 = 1829,
    /// NO-DEFAULT
    NoDefault                                                              = 2473,
    /// NO-FLOAT
    NoFloat                                                                = 1560,
    /// NO-HEADER
    NoHeader                                                               = 2299,
    /// NO-KEEP
    NoKeep                                                                 = 1869,
    /// NO-MONOTONY
    NoMonotony                                                             = 310,
    /// NO-NEWLINE
    NoNewline                                                              = 1061,
    /// NO-OBD-SUPPORT
    NoObdSupport                                                           = 2018,
    /// NO-PGWIDE
    NoPgwide                                                               = 1913,
    /// NO-PROTECTION
    NoProtection                                                           = 1002,
    /// NO-RETURN-VALUE-PROVIDED
    NoReturnValueProvided                                                  = 2127,
    /// NO-SEVERITY
    NoSeverity                                                             = 2426,
    /// NO-SHOW-ALIAS-NAME
    NoShowAliasName                                                        = 170,
    /// NO-SHOW-CATEGORY
    NoShowCategory                                                         = 220,
    /// NO-SHOW-CONTENT
    NoShowContent                                                          = 252,
    /// NO-SHOW-LONG-NAME
    NoShowLongName                                                         = 1793,
    /// NO-SHOW-NUMBER
    NoShowNumber                                                           = 952,
    /// NO-SHOW-PAGE
    NoShowPage                                                             = 2358,
    /// NO-SHOW-SEE
    NoShowSee                                                              = 1832,
    /// NO-SHOW-SHORT-NAME
    NoShowShortName                                                        = 535,
    /// NO-SHOW-TYPE
    NoShowType                                                             = 2369,
    /// NO-SLOPPY
    NoSloppy                                                               = 145,
    /// NO-STATUS-BYTE-CHANGE
    NoStatusByteChange                                                     = 1695,
    /// NO-STORE-EVENT
    NoStoreEvent                                                           = 2349,
    /// NO-SUPERVISION
    NoSupervision                                                          = 1383,
    /// NO-SUPPORT
    NoSupport                                                              = 555,
    /// NO-TRANSFORMER-ERROR-HANDLING
    NoTransformerErrorHandling                                             = 1246,
    /// NO-TRANSFORMER-STATUS-FORWARDING
    NoTransformerStatusForwarding                                          = 891,
    /// NO-TRUSTED-PLATFORM-SUPPORT
    NoTrustedPlatformSupport                                               = 1478,
    /// NODE
    Node                                                                   = 2039,
    /// NOHREF
    Nohref                                                                 = 2199,
    /// NON-EMMISSION-RELATED-DTC
    NonEmmissionRelatedDtc                                                 = 2388,
    /// NON-OS-MODULE-INSTANTIATION
    NonOsModuleInstantiation                                               = 1591,
    /// NON-REENTRANT
    NonReentrant                                                           = 2455,
    /// NON-VOLATILE
    NonVolatile                                                            = 128,
    /// NON-VOLATILE-RAM-MANAGER
    NonVolatileRamManager                                                  = 2317,
    /// NONE
    None                                                                   = 1840,
    /// NORMALFIXED
    Normalfixed                                                            = 1509,
    /// NOT
    Not                                                                    = 692,
    /// NOT-ACCESSIBLE
    NotAccessible                                                          = 455,
    /// NOT-AVAILABLE
    NotAvailable                                                           = 800,
    /// NOT-DEFINED
    NotDefined                                                             = 1694,
    /// NOT-EQUAL
    NotEqual                                                               = 648,
    /// NOT-SENT
    NotSent                                                                = 2174,
    /// NOT-TESTED
    NotTested                                                              = 2110,
    /// NOT-VALID
    NotValid                                                               = 1263,
    /// NOTHING
    Nothing                                                                = 671,
    /// NOTIFICATION
    Notification                                                           = 913,
    /// NTP--RFC-958
    NtpRfc958                                                              = 972,
    /// NUMBER
    Number                                                                 = 1630,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 1799,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 2084,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 1020,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 1548,
    /// NV-RAM-MANAGER
    NvRamManager                                                           = 1693,
    /// OBD
    Obd                                                                    = 83,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 258,
    /// OBD-DCY
    ObdDcy                                                                 = 991,
    /// OBD-DRIVING-CYCLE
    ObdDrivingCycle                                                        = 1407,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 390,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 1659,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 444,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 2010,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 1396,
    /// OBSERVER
    Observer                                                               = 1604,
    /// OBSERVER-BASED
    ObserverBased                                                          = 1537,
    /// OC
    Oc                                                                     = 1817,
    /// OCCURENCE
    Occurence                                                              = 694,
    /// OEM-BOOT
    OemBoot                                                                = 1340,
    /// OEM-BOOT-RESP-APP
    OemBootRespApp                                                         = 434,
    /// OFF
    Off                                                                    = 1865,
    /// OFFSET
    Offset                                                                 = 1099,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 1934,
    /// OM
    Om                                                                     = 819,
    /// ON-CHANGE-OF-DATA-IDENTIFIER
    OnChangeOfDataIdentifier                                               = 1211,
    /// ON-COMPARISON-OF-VALUES
    OnComparisonOfValues                                                   = 279,
    /// ON-DTC-STATUS-CHANGE
    OnDtcStatusChange                                                      = 527,
    /// ON-ENTRY
    OnEntry                                                                = 617,
    /// ON-EXIT
    OnExit                                                                 = 1839,
    /// ON-TRANSITION
    OnTransition                                                           = 853,
    /// ONE-EVERY-N
    OneEveryN                                                              = 265,
    /// ONLY-THIS-CYCLE-AND-READINESS
    OnlyThisCycleAndReadiness                                              = 1347,
    /// OPAQUE
    Opaque                                                                 = 159,
    /// OPEN
    Open                                                                   = 1288,
    /// OPERATING-SYSTEM
    OperatingSystem                                                        = 2454,
    /// OPERATION-CALL-RECEIVED
    OperationCallReceived                                                  = 786,
    /// OPERATION-CALL-RESPONSE-RECEIVED
    OperationCallResponseReceived                                          = 2348,
    /// OPERATION-CALL-RESPONSE-SENT
    OperationCallResponseSent                                              = 2152,
    /// OPERATION-CALLED
    OperationCalled                                                        = 1057,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 1726,
    /// OPTIONS
    Options                                                                = 1405,
    /// OR
    Or                                                                     = 1639,
    /// ORDINARY-EOC
    OrdinaryEoc                                                            = 1093,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 1634,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 1127,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 48,
    /// OTHER
    Other                                                                  = 574,
    /// OUT
    Out                                                                    = 242,
    /// OVERRIDE
    Override                                                               = 881,
    /// OVERWRITE
    Overwrite                                                              = 2118,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 602,
    /// PA
    Pa                                                                     = 1192,
    /// PACKAGEABLE-ELEMENT
    PackageableElement                                                     = 1253,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 451,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 761,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 1301,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 2005,
    /// PARTIAL-NETWORK
    PartialNetwork                                                         = 307,
    /// PARTITION
    Partition                                                              = 1304,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 2385,
    /// PASSIVE
    Passive                                                                = 1911,
    /// PASSTHROUGH
    Passthrough                                                            = 1461,
    /// PAYLOAD-AS-ARRAY
    PayloadAsArray                                                         = 343,
    /// PAYLOAD-AS-POINTER-TO-ARRAY
    PayloadAsPointerToArray                                                = 1292,
    /// PC-AFFECTS-LT
    PcAffectsLt                                                            = 2266,
    /// PC-AFFECTS-LT-AND-PB
    PcAffectsLtAndPb                                                       = 2439,
    /// PC-AFFECTS-PB
    PcAffectsPb                                                            = 771,
    /// PDF
    Pdf                                                                    = 2451,
    /// PDU
    Pdu                                                                    = 1554,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 989,
    /// PDU-R
    PduR                                                                   = 2208,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 2161,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 2338,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 983,
    /// PEER
    Peer                                                                   = 289,
    /// PENDING
    Pending                                                                = 364,
    /// PER-EXECUTABLE
    PerExecutable                                                          = 1159,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 1877,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 16,
    /// PERIODIC-RATE-FAST
    PeriodicRateFast                                                       = 222,
    /// PERIODIC-RATE-MEDIUM
    PeriodicRateMedium                                                     = 72,
    /// PERIODIC-RATE-SLOW
    PeriodicRateSlow                                                       = 2048,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 1170,
    /// PERSISTENCY-DEPLOYMENT
    PersistencyDeployment                                                  = 207,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT
    PersistencyDeploymentElement                                           = 584,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 827,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 2231,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 665,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 1501,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 514,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 1820,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 2081,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 19,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 2170,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 2489,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 755,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 2224,
    /// PERSISTENCY-INTERFACE
    PersistencyInterface                                                   = 1544,
    /// PERSISTENCY-INTERFACE-ELEMENT
    PersistencyInterfaceElement                                            = 66,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 259,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 721,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 571,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 636,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 2371,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING
    PersistencyPortPrototypeToDeploymentMapping                            = 1032,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 1602,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 620,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 664,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 2166,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE
    PersistencyRedundancyHandlingScopeDatabase                             = 944,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT
    PersistencyRedundancyHandlingScopeElement                              = 1954,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE
    PersistencyRedundancyHandlingScopeFile                                 = 176,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY
    PersistencyRedundancyHandlingScopeKey                                  = 1208,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE
    PersistencyRedundancyHandlingScopeStorage                              = 1228,
    /// PGWIDE
    Pgwide                                                                 = 1276,
    /// PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE
    PhmAbstractRecoveryNotificationInterface                               = 2237,
    /// PHM-ACTION
    PhmAction                                                              = 139,
    /// PHM-ACTION-ITEM
    PhmActionItem                                                          = 1307,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 936,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 803,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 1137,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 554,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 1745,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 2243,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 1144,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 1171,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 2491,
    /// PHM-RULE
    PhmRule                                                                = 783,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 1719,
    /// PHM-SUPERVISION
    PhmSupervision                                                         = 1863,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 1369,
    /// PHYSICAL
    Physical                                                               = 1194,
    /// PHYSICAL-ADDRESS
    PhysicalAddress                                                        = 2022,
    /// PHYSICAL-CAN-FD
    PhysicalCanFd                                                          = 1656,
    /// PHYSICAL-CHANNEL
    PhysicalChannel                                                        = 2015,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 1663,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 1580,
    /// PL
    Pl                                                                     = 294,
    /// PLAIN
    Plain                                                                  = 681,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 9,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 158,
    /// PLATFORM-HEALTH-MANAGEMENT-INTERFACE
    PlatformHealthManagementInterface                                      = 442,
    /// PLATFORM-MODULE-ENDPOINT-CONFIGURATION
    PlatformModuleEndpointConfiguration                                    = 1442,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 1809,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 133,
    /// PNC-MAPPING-IDENT
    PncMappingIdent                                                        = 238,
    /// PNG
    Png                                                                    = 752,
    /// POLY
    Poly                                                                   = 1255,
    /// PORT
    Port                                                                   = 1740,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 21,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 246,
    /// PORT-GROUP
    PortGroup                                                              = 1578,
    /// PORT-INTERFACE
    PortInterface                                                          = 2114,
    /// PORT-INTERFACE-DEFINITION
    PortInterfaceDefinition                                                = 1166,
    /// PORT-INTERFACE-MAPPING
    PortInterfaceMapping                                                   = 1989,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 774,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 1115,
    /// PORT-PROTOTYPE
    PortPrototype                                                          = 297,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 1787,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 1469,
    /// POST
    Post                                                                   = 1422,
    /// POST-BUILD
    PostBuild                                                              = 264,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 986,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 70,
    /// POWER
    Power                                                                  = 1385,
    /// POWER-WINDOW-TIME
    PowerWindowTime                                                        = 2465,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 2492,
    /// PRE--R-4--2
    PreR42                                                                 = 2202,
    /// PRE-COMPILE
    PreCompile                                                             = 1224,
    /// PRE-COMPILE-TIME
    PreCompileTime                                                         = 2354,
    /// PRECONFIGURED-CONFIGURATION
    PreconfiguredConfiguration                                             = 1691,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 274,
    /// PRESENTATION-CONTINUOUS
    PresentationContinuous                                                 = 2050,
    /// PRESENTATION-DISCRETE
    PresentationDiscrete                                                   = 2059,
    /// PRESHARED-KEY-IDENTITY
    PresharedKeyIdentity                                                   = 1958,
    /// PRIMARY-ECU
    PrimaryEcu                                                             = 2211,
    /// PRIMITIVE
    Primitive                                                              = 453,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 1480,
    /// PRIO-OCC
    PrioOcc                                                                = 2113,
    /// PRIVATE-KEY
    PrivateKey                                                             = 669,
    /// PROCESS
    Process                                                                = 1366,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 378,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 439,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 623,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 1484,
    /// PROCESS-IS-NOT-SELF-TERMINATING
    ProcessIsNotSelfTerminating                                            = 2332,
    /// PROCESS-IS-SELF-TERMINATING
    ProcessIsSelfTerminating                                               = 2314,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 624,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 2305,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 326,
    /// PROCESSING-STYLE-ASYNCHRONOUS
    ProcessingStyleAsynchronous                                            = 653,
    /// PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR
    ProcessingStyleAsynchronousWithError                                   = 707,
    /// PROCESSING-STYLE-SYNCHRONOUS
    ProcessingStyleSynchronous                                             = 1698,
    /// PROCESSOR
    Processor                                                              = 2246,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 842,
    /// PRODUCER
    Producer                                                               = 2180,
    /// PROTECT-LAMP
    ProtectLamp                                                            = 594,
    /// PROTECTED
    Protected                                                              = 1318,
    /// PROVIDED-AP-SERVICE-INSTANCE
    ProvidedApServiceInstance                                              = 1249,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 1119,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 733,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 1124,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 820,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 100,
    /// PS
    Ps                                                                     = 1833,
    /// PSK
    Psk                                                                    = 1715,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 553,
    /// PT
    Pt                                                                     = 286,
    /// PTP--IEEE-1588--2002
    PtpIeee15882002                                                        = 1777,
    /// PTP--IEEE-1588--2008
    PtpIeee15882008                                                        = 682,
    /// PUBLIC-KEY
    PublicKey                                                              = 1026,
    /// PUBLISHED-INFORMATION
    PublishedInformation                                                   = 2105,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 916,
    /// PUT
    Put                                                                    = 2337,
    /// QU
    Qu                                                                     = 20,
    /// QUEUED
    Queued                                                                 = 2245,
    /// R-4--2
    R42                                                                    = 1915,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 2475,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 1178,
    /// RAW
    Raw                                                                    = 485,
    /// RAW-DATA
    RawData                                                                = 1114,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 634,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 737,
    /// RAW-DATA-STREAM-GRANT
    RawDataStreamGrant                                                     = 2244,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 816,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 447,
    /// RAW-DATA-STREAM-MAPPING
    RawDataStreamMapping                                                   = 42,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 173,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 1349,
    /// REACTION
    Reaction                                                               = 2017,
    /// READ
    Read                                                                   = 792,
    /// READ-ONLY
    ReadOnly                                                               = 140,
    /// READ-WRITE
    ReadWrite                                                              = 2272,
    /// REBOOT
    Reboot                                                                 = 2334,
    /// RECOMMENDED-CONFIGURATION
    RecommendedConfiguration                                               = 1896,
    /// RECORD-VALUE-FIELD
    RecordValueField                                                       = 407,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 253,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 2194,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 573,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 1969,
    /// RECT
    Rect                                                                   = 712,
    /// RED-STOP-LAMP
    RedStopLamp                                                            = 2051,
    /// REDUNDANT
    Redundant                                                              = 2476,
    /// REDUNDANT-PER-ELEMENT
    RedundantPerElement                                                    = 688,
    /// REDUNDANT-PER-KEY
    RedundantPerKey                                                        = 1872,
    /// REF-ALL
    RefAll                                                                 = 1903,
    /// REF-NON-STANDARD
    RefNonStandard                                                         = 2321,
    /// REF-NONE
    RefNone                                                                = 2282,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 401,
    /// REFERRABLE
    Referrable                                                             = 1006,
    /// REGULAR
    Regular                                                                = 2221,
    /// REJECT
    Reject                                                                 = 1109,
    /// REMOVE
    Remove                                                                 = 1852,
    /// REPETITIVE-EOC
    RepetitiveEoc                                                          = 1378,
    /// REPLACE
    Replace                                                                = 65,
    /// REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE
    ReplaceByTimeoutSubstitutionValue                                      = 747,
    /// REPORT
    Report                                                                 = 984,
    /// REPORT-AFTER-INIT
    ReportAfterInit                                                        = 1810,
    /// REPORT-BEFORE-INIT
    ReportBeforeInit                                                       = 1169,
    /// REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE
    ReportDtcRecordInformationOnDtcStatusChange                            = 2347,
    /// REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE
    ReportMostRecentDtcOnStatusChange                                      = 254,
    /// REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST
    ReportingInChronlogicalOrderOldestFirst                                = 2164,
    /// REPORTS-EXECUTION-STATE
    ReportsExecutionState                                                  = 2147,
    /// REQUEST
    Request                                                                = 366,
    /// REQUEST-CALLBACK-TYPE-MANUFACTURER
    RequestCallbackTypeManufacturer                                        = 2445,
    /// REQUEST-CALLBACK-TYPE-SUPPLIER
    RequestCallbackTypeSupplier                                            = 994,
    /// REQUEST-NO-RETURN
    RequestNoReturn                                                        = 610,
    /// REQUIRED-AP-SERVICE-INSTANCE
    RequiredApServiceInstance                                              = 352,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 2111,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 146,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 920,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 1125,
    /// REQUIRES-CALLBACK-EXECUTION
    RequiresCallbackExecution                                              = 1935,
    /// RES-AXIS
    ResAxis                                                                = 144,
    /// RESET-ECU
    ResetEcu                                                               = 1365,
    /// RESET-MACHINE
    ResetMachine                                                           = 186,
    /// RESET-MCU
    ResetMcu                                                               = 641,
    /// RESET-VM
    ResetVm                                                                = 1629,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 394,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 248,
    /// RESPOND-AFTER-RESET
    RespondAfterReset                                                      = 1182,
    /// RESPOND-BEFORE-RESET
    RespondBeforeReset                                                     = 1177,
    /// RESPONSE
    Response                                                               = 801,
    /// RESPONSE-SYNCHRONIZATION
    ResponseSynchronization                                                = 2429,
    /// REST-ABSTRACT-ENDPOINT
    RestAbstractEndpoint                                                   = 691,
    /// REST-ABSTRACT-NUMERICAL-PROPERTY-DEF
    RestAbstractNumericalPropertyDef                                       = 2204,
    /// REST-ABSTRACT-PROPERTY-DEF
    RestAbstractPropertyDef                                                = 2240,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 1277,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 1317,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 1111,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 2024,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 2411,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 2169,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 2025,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 2071,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 456,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 1960,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 2062,
    /// REST-PRIMITIVE-PROPERTY-DEF
    RestPrimitivePropertyDef                                               = 534,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 908,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 1830,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 945,
    /// RESTART
    Restart                                                                = 2364,
    /// RESTART-APPLICATION
    RestartApplication                                                     = 639,
    /// RES_AXIS
    Resaxis                                                                = 308,
    /// RETURN-ON-EVENT-CLEARED
    ReturnOnEventCleared                                                   = 662,
    /// RETURN-ON-EVENT-STOPPED
    ReturnOnEventStopped                                                   = 2293,
    /// RETURN-VALUE-PROVIDED
    ReturnValueProvided                                                    = 1401,
    /// RIGHT
    Right                                                                  = 2122,
    /// RM
    Rm                                                                     = 2136,
    /// RN
    Rn                                                                     = 1531,
    /// RO
    Ro                                                                     = 4,
    /// ROLL-BACK
    RollBack                                                               = 2416,
    /// ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE
    RootSwClusterDesignComponentPrototype                                  = 382,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 1133,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 1417,
    /// ROTATE-180
    Rotate180                                                              = 1589,
    /// ROTATE-180-LIMIT-TO-TEXT
    Rotate180LimitToText                                                   = 1043,
    /// ROTATE-90-CCW
    Rotate90Ccw                                                            = 799,
    /// ROTATE-90-CCW-FIT-TO-TEXT
    Rotate90CcwFitToText                                                   = 1121,
    /// ROTATE-90-CCW-LIMIT-TO-TEXT
    Rotate90CcwLimitToText                                                 = 2215,
    /// ROTATE-90-CW
    Rotate90Cw                                                             = 2472,
    /// ROTATE-90-CW-FIT-TO-TEXT
    Rotate90CwFitToText                                                    = 345,
    /// ROTATE-90-CW-LIMIT-TO-TEXT
    Rotate90CwLimitToText                                                  = 1436,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 556,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 1523,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 2193,
    /// ROUTER
    Router                                                                 = 317,
    /// ROUTER-ADVERTISEMENT
    RouterAdvertisement                                                    = 373,
    /// RPT-COMPONENT
    RptComponent                                                           = 1642,
    /// RPT-CONTAINER
    RptContainer                                                           = 2054,
    /// RPT-ENABLER-RAM
    RptEnablerRam                                                          = 501,
    /// RPT-ENABLER-RAM-AND-ROM
    RptEnablerRamAndRom                                                    = 1606,
    /// RPT-ENABLER-ROM
    RptEnablerRom                                                          = 1411,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 1806,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 1210,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 713,
    /// RPT-LEVEL-1
    RptLevel1                                                              = 1086,
    /// RPT-LEVEL-2
    RptLevel2                                                              = 2342,
    /// RPT-LEVEL-3
    RptLevel3                                                              = 181,
    /// RPT-PROFILE
    RptProfile                                                             = 1472,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 1311,
    /// RSA
    Rsa                                                                    = 864,
    /// RTE-EVENT
    RteEvent                                                               = 2485,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 517,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 1370,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 622,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 1474,
    /// RTPGE
    Rtpge                                                                  = 1720,
    /// RU
    Ru                                                                     = 2195,
    /// RULE
    Rule                                                                   = 765,
    /// RUN-CONTINUOUS
    RunContinuous                                                          = 2049,
    /// RUN-ONCE
    RunOnce                                                                = 423,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 473,
    /// RUNNABLE-ENTITY-ACTIVATED
    RunnableEntityActivated                                                = 1563,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 1016,
    /// RUNNABLE-ENTITY-STARTED
    RunnableEntityStarted                                                  = 461,
    /// RUNNABLE-ENTITY-TERMINATED
    RunnableEntityTerminated                                               = 836,
    /// RUNNABLE-ENTITY-VARIABLE-ACCESS
    RunnableEntityVariableAccess                                           = 788,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 956,
    /// RW
    Rw                                                                     = 838,
    /// RX-TRIGGER
    RxTrigger                                                              = 2328,
    /// SA
    Sa                                                                     = 124,
    /// SAE-J-1939--73
    SaeJ193973                                                             = 1900,
    /// SAE-J-2012--DA
    SaeJ2012Da                                                             = 2257,
    /// SAFETY
    Safety                                                                 = 178,
    /// SATURATE
    Saturate                                                               = 1242,
    /// SCHEDULE-VARIANT-1
    ScheduleVariant1                                                       = 2462,
    /// SCHEDULE-VARIANT-2
    ScheduleVariant2                                                       = 282,
    /// SCHEDULE-VARIANT-3
    ScheduleVariant3                                                       = 2098,
    /// SCHEDULE-VARIANT-4
    ScheduleVariant4                                                       = 2409,
    /// SCHEDULE-VARIANT-5
    ScheduleVariant5                                                       = 917,
    /// SCHEDULE-VARIANT-6
    ScheduleVariant6                                                       = 2013,
    /// SCHEDULE-VARIANT-7
    ScheduleVariant7                                                       = 988,
    /// SCHEDULED
    Scheduled                                                              = 2260,
    /// SD
    Sd                                                                     = 2046,
    /// SDG-ABSTRACT-FOREIGN-REFERENCE
    SdgAbstractForeignReference                                            = 1322,
    /// SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE
    SdgAbstractPrimitiveAttribute                                          = 643,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 2449,
    /// SDG-ATTRIBUTE
    SdgAttribute                                                           = 508,
    /// SDG-CAPTION
    SdgCaption                                                             = 1310,
    /// SDG-CLASS
    SdgClass                                                               = 1463,
    /// SDG-DEF
    SdgDef                                                                 = 113,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 2214,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 74,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 288,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 2295,
    /// SDG-REFERENCE
    SdgReference                                                           = 1345,
    /// SDG-TAILORING
    SdgTailoring                                                           = 1807,
    /// SEARCH-FOR-ALL
    SearchForAll                                                           = 418,
    /// SEARCH-FOR-ALL-INSTANCES
    SearchForAllInstances                                                  = 1690,
    /// SEARCH-FOR-ANY
    SearchForAny                                                           = 2100,
    /// SEARCH-FOR-ID
    SearchForId                                                            = 965,
    /// SEARCH-FOR-SPECIFIC-INSTANCE
    SearchForSpecificInstance                                              = 1187,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 794,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 1236,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 2324,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 868,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 199,
    /// SECOND-TO-FIRST
    SecondToFirst                                                          = 138,
    /// SECONDARY-ECU
    SecondaryEcu                                                           = 1682,
    /// SECRET-SEED
    SecretSeed                                                             = 2344,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 1790,
    /// SECURE-COM-PROPS
    SecureComProps                                                         = 334,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 605,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 1862,
    /// SECURE-COMMUNICATION-DEPLOYMENT
    SecureCommunicationDeployment                                          = 2436,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 257,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 1190,
    /// SECURE-ON-BOARD-COMMUNICATION
    SecureOnBoardCommunication                                             = 481,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 1234,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 2361,
    /// SECURED-PDU-HEADER-08-BIT
    SecuredPduHeader08Bit                                                  = 1381,
    /// SECURED-PDU-HEADER-16-BIT
    SecuredPduHeader16Bit                                                  = 155,
    /// SECURED-PDU-HEADER-32-BIT
    SecuredPduHeader32Bit                                                  = 851,
    /// SECURITY
    Security                                                               = 1241,
    /// SECURITY-EVENT-AGGREGATION-FILTER
    SecurityEventAggregationFilter                                         = 1191,
    /// SECURITY-EVENT-CONTEXT-MAPPING
    SecurityEventContextMapping                                            = 2271,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 530,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 1279,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 1326,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 232,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 108,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 239,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 825,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 1792,
    /// SECURITY-EVENT-ONE-EVERY-N-FILTER
    SecurityEventOneEveryNFilter                                           = 2082,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 1546,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 759,
    /// SECURITY-EVENT-STATE-FILTER
    SecurityEventStateFilter                                               = 299,
    /// SECURITY-EVENT-THRESHOLD-FILTER
    SecurityEventThresholdFilter                                           = 131,
    /// SELECTED
    Selected                                                               = 115,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 939,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 2092,
    /// SENT-TAGGED
    SentTagged                                                             = 1081,
    /// SENT-UNTAGGED
    SentUntagged                                                           = 1796,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 1801,
    /// SERIALIZER
    Serializer                                                             = 1733,
    /// SERVER-AUTHENTICATE
    ServerAuthenticate                                                     = 698,
    /// SERVER-CALL-POINT
    ServerCallPoint                                                        = 2072,
    /// SERVER-DECRYPT
    ServerDecrypt                                                          = 2410,
    /// SERVER-ENCRYPT
    ServerEncrypt                                                          = 1085,
    /// SERVER-MAC-GENERATE
    ServerMacGenerate                                                      = 403,
    /// SERVER-MAC-VERIFY
    ServerMacVerify                                                        = 271,
    /// SERVER-VERIFY
    ServerVerify                                                           = 1196,
    /// SERVICE-DISCOVERY
    ServiceDiscovery                                                       = 1529,
    /// SERVICE-EVENT-DEPLOYMENT
    ServiceEventDeployment                                                 = 781,
    /// SERVICE-FIELD-DEPLOYMENT
    ServiceFieldDeployment                                                 = 198,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 1808,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 1438,
    /// SERVICE-INSTANCE-TO-MACHINE-MAPPING
    ServiceInstanceToMachineMapping                                        = 1375,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 2097,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 1256,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 2058,
    /// SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToSwClusterDesignPortPrototypeMapping                   = 1487,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 2346,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 656,
    /// SERVICE-INTERFACE-DEPLOYMENT
    ServiceInterfaceDeployment                                             = 718,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING
    ServiceInterfaceElementMapping                                         = 767,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 934,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 1355,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 1686,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 1202,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 2278,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 471,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 109,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 2488,
    /// SERVICE-METHOD-DEPLOYMENT
    ServiceMethodDeployment                                                = 2443,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 740,
    /// SERVICE-ONLY
    ServiceOnly                                                            = 2228,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 1446,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 2255,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 1868,
    /// SESSION-HANDLING-ACTIVE
    SessionHandlingActive                                                  = 1584,
    /// SESSION-HANDLING-INACTIVE
    SessionHandlingInactive                                                = 889,
    /// SETTER
    Setter                                                                 = 729,
    /// SG
    Sg                                                                     = 1561,
    /// SH
    Sh                                                                     = 591,
    /// SHORT-HEADER
    ShortHeader                                                            = 797,
    /// SHOW-ALIAS-NAME
    ShowAliasName                                                          = 1038,
    /// SHOW-CATEGORY
    ShowCategory                                                           = 1287,
    /// SHOW-CONTENT
    ShowContent                                                            = 119,
    /// SHOW-LONG-NAME
    ShowLongName                                                           = 1348,
    /// SHOW-NUMBER
    ShowNumber                                                             = 1812,
    /// SHOW-PAGE
    ShowPage                                                               = 1059,
    /// SHOW-SEE
    ShowSee                                                                = 931,
    /// SHOW-SHORT-NAME
    ShowShortName                                                          = 837,
    /// SHOW-TYPE
    ShowType                                                               = 1977,
    /// SI
    Si                                                                     = 1644,
    /// SIDES
    Sides                                                                  = 2493,
    /// SIGN
    Sign                                                                   = 1244,
    /// SIGN-WITH-ORIGIN-AUTHENTICATION
    SignWithOriginAuthentication                                           = 2068,
    /// SIGNAL-BASED
    SignalBased                                                            = 505,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 1395,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 1860,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 2375,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 2126,
    /// SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFireAndForgetMethodToISignalTriggeringMapping               = 678,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 1718,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 839,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 564,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 693,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 905,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 645,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 2482,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 497,
    /// SIGNATURE
    Signature                                                              = 1198,
    /// SILENT
    Silent                                                                 = 162,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 1514,
    /// SINGLE
    Single                                                                 = 1251,
    /// SINGLE-CORE-REENTRANT
    SingleCoreReentrant                                                    = 1126,
    /// SINGLE-LANGUAGE-REFERRABLE
    SingleLanguageReferrable                                               = 614,
    /// SINGLE-OCCURRENCE
    SingleOccurrence                                                       = 631,
    /// SK
    Sk                                                                     = 1184,
    /// SL
    Sl                                                                     = 1336,
    /// SLAVE
    Slave                                                                  = 163,
    /// SLOPPY
    Sloppy                                                                 = 477,
    /// SLOW-FLASHING-MODE
    SlowFlashingMode                                                       = 436,
    /// SLP
    Slp                                                                    = 523,
    /// SM
    Sm                                                                     = 1041,
    /// SN
    Sn                                                                     = 1675,
    /// SO
    So                                                                     = 192,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 411,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 1354,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 562,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 2370,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 2438,
    /// SOFTWARE-ACTIVATION-DEPENDENCY
    SoftwareActivationDependency                                           = 1767,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 1814,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 2217,
    /// SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS
    SoftwareClusterDiagnosticDeploymentProps                               = 52,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 1696,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 545,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 1983,
    /// SOMEIP
    Someip                                                                 = 1655,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 725,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 80,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 1731,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 1893,
    /// SOMEIP-FIELD
    SomeipField                                                            = 709,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 2242,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 1054,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 784,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 1609,
    /// SOMEIP-REMOTE-MULTICAST-CONFIG
    SomeipRemoteMulticastConfig                                            = 416,
    /// SOMEIP-REMOTE-UNICAST-CONFIG
    SomeipRemoteUnicastConfig                                              = 1309,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 427,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 1558,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 13,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 114,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 1922,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 2315,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 949,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 2421,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 642,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 92,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 86,
    /// SOVD-GATEWAY-INSTANTIATION
    SovdGatewayInstantiation                                               = 504,
    /// SOVD-MODULE-INSTANTIATION
    SovdModuleInstantiation                                                = 1771,
    /// SOVD-SERVER-INSTANTIATION
    SovdServerInstantiation                                                = 1156,
    /// SPEC-ELEMENT-REFERENCE
    SpecElementReference                                                   = 2003,
    /// SPEC-ELEMENT-SCOPE
    SpecElementScope                                                       = 608,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 1570,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 953,
    /// SQ
    Sq                                                                     = 1154,
    /// SR
    Sr                                                                     = 1805,
    /// SS
    Ss                                                                     = 596,
    /// SSDP
    Ssdp                                                                   = 1703,
    /// ST
    St                                                                     = 1620,
    /// STACK-USAGE
    StackUsage                                                             = 683,
    /// STANDARD
    Standard                                                               = 533,
    /// STANDARD-PORT
    StandardPort                                                           = 1765,
    /// START
    Start                                                                  = 1232,
    /// START-FROM-BEGINNING
    StartFromBeginning                                                     = 1574,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 745,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 2390,
    /// STATE-DEPENDENT-FIREWALL
    StateDependentFirewall                                                 = 2163,
    /// STATE-MANAGEMEN-PHM-ERROR-INTERFACE
    StateManagemenPhmErrorInterface                                        = 449,
    /// STATE-MANAGEMENT-ACTION-ITEM
    StateManagementActionItem                                              = 361,
    /// STATE-MANAGEMENT-ACTION-LIST
    StateManagementActionList                                              = 1207,
    /// STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE
    StateManagementDiagTriggerInterface                                    = 632,
    /// STATE-MANAGEMENT-EM-ERROR-INTERFACE
    StateManagementEmErrorInterface                                        = 1408,
    /// STATE-MANAGEMENT-ERROR-INTERFACE
    StateManagementErrorInterface                                          = 1245,
    /// STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE
    StateManagementFunctionGroupSwitchNotificationInterface                = 200,
    /// STATE-MANAGEMENT-MODULE-INSTANTIATION
    StateManagementModuleInstantiation                                     = 50,
    /// STATE-MANAGEMENT-NOTIFICATION-INTERFACE
    StateManagementNotificationInterface                                   = 954,
    /// STATE-MANAGEMENT-PORT-INTERFACE
    StateManagementPortInterface                                           = 243,
    /// STATE-MANAGEMENT-REQUEST-ERROR
    StateManagementRequestError                                            = 1267,
    /// STATE-MANAGEMENT-REQUEST-INTERFACE
    StateManagementRequestInterface                                        = 1920,
    /// STATE-MANAGEMENT-REQUEST-TRIGGER
    StateManagementRequestTrigger                                          = 2078,
    /// STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM
    StateManagementSetFunctionGroupStateActionItem                         = 1018,
    /// STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM
    StateManagementStateMachineActionItem                                  = 1031,
    /// STATE-MANAGEMENT-STATE-NOTIFICATION
    StateManagementStateNotification                                       = 26,
    /// STATE-MANAGEMENT-STATE-REQUEST
    StateManagementStateRequest                                            = 2302,
    /// STATE-MANAGEMENT-SYNC-ACTION-ITEM
    StateManagementSyncActionItem                                          = 1419,
    /// STATE-MANAGEMENT-TRIGGER-INTERFACE
    StateManagementTriggerInterface                                        = 2145,
    /// STATIC-OR-DYNAMIC-PART-TRIGGER
    StaticOrDynamicPartTrigger                                             = 993,
    /// STATIC-PART-TRIGGER
    StaticPartTrigger                                                      = 1848,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 831,
    /// STATUS-BIT-AGING-AND-DISPLACEMENT
    StatusBitAgingAndDisplacement                                          = 93,
    /// STATUS-BIT-NORMAL
    StatusBitNormal                                                        = 2225,
    /// STD
    Std                                                                    = 1556,
    /// STD-AXIS
    StdAxis                                                                = 1594,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 613,
    /// STD_AXIS
    Stdaxis                                                                = 947,
    /// STEADY
    Steady                                                                 = 59,
    /// STIMULUS-SYNCHRONIZATION
    StimulusSynchronization                                                = 854,
    /// STOP
    Stop                                                                   = 2402,
    /// STOP-TRIGGER
    StopTrigger                                                            = 2284,
    /// STORE-EVENT
    StoreEvent                                                             = 2415,
    /// STORE-PERSISTENTLY
    StorePersistently                                                      = 1350,
    /// STRICT-MODE
    StrictMode                                                             = 2391,
    /// STRICT-MONOTONOUS
    StrictMonotonous                                                       = 2452,
    /// STRICT-PRIORITY
    StrictPriority                                                         = 2077,
    /// STRICTLY-DECREASING
    StrictlyDecreasing                                                     = 1971,
    /// STRICTLY-INCREASING
    StrictlyIncreasing                                                     = 848,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 2069,
    /// SU
    Su                                                                     = 39,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 2287,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 2067,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 1341,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 1828,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 1759,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 2176,
    /// SUPPLIER
    Supplier                                                               = 1512,
    /// SUPPORTS-BUFFER-LOCKING
    SupportsBufferLocking                                                  = 1094,
    /// SV
    Sv                                                                     = 1592,
    /// SVG
    Svg                                                                    = 54,
    /// SW
    Sw                                                                     = 1087,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 2112,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 2167,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 863,
    /// SW-CALIBRATION-METHOD
    SwCalibrationMethod                                                    = 1122,
    /// SW-CALPRM-PROTOTYPE
    SwCalprmPrototype                                                      = 1270,
    /// SW-CLASS-ATTR-IMPL
    SwClassAttrImpl                                                        = 2135,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 1650,
    /// SW-CLASS-PROTOTYPE
    SwClassPrototype                                                       = 135,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 2365,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 1516,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 2450,
    /// SW-COMPONENT-TYPE
    SwComponentType                                                        = 2085,
    /// SW-CONNECTOR
    SwConnector                                                            = 1998,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 1564,
    /// SW-INSTANCE
    SwInstance                                                             = 2182,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 2279,
    /// SW-MC-FRAME
    SwMcFrame                                                              = 2280,
    /// SW-MC-INTERFACE
    SwMcInterface                                                          = 1505,
    /// SW-MC-INTERFACE-SOURCE
    SwMcInterfaceSource                                                    = 1926,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 1846,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 425,
    /// SW-SERVICE-PROTOTYPE
    SwServicePrototype                                                     = 1769,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 1779,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 2446,
    /// SW-VARIABLE-PROTOTYPE
    SwVariablePrototype                                                    = 103,
    /// SWC
    Swc                                                                    = 1717,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 157,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 560,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 223,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 2210,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 2336,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 815,
    /// SWC-TIMING
    SwcTiming                                                              = 1899,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 1049,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 1555,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 1185,
    /// SWITCH
    Switch                                                                 = 1843,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 2125,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 2461,
    /// SYMMETRIC
    Symmetric                                                              = 474,
    /// SYMMETRIC-KEY
    SymmetricKey                                                           = 2339,
    /// SYNC-BASE-TIME-MANAGER
    SyncBaseTimeManager                                                    = 208,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 1904,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 735,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 1758,
    /// SYNCHRONIZED
    Synchronized                                                           = 901,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 2447,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 950,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 2460,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 465,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 2420,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 1368,
    /// SYNCHRONOUS
    Synchronous                                                            = 1871,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 1298,
    /// SYSTEM
    System                                                                 = 1028,
    /// SYSTEM-DESIGN-TIME
    SystemDesignTime                                                       = 1338,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 2471,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 1205,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 1007,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 2189,
    /// SYSTEM-SUPPLIER-BOOT
    SystemSupplierBoot                                                     = 1881,
    /// SYSTEM-SUPPLIER-BOOT-RESP-APP
    SystemSupplierBootRespApp                                              = 2179,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 922,
    /// TA
    Ta                                                                     = 887,
    /// TARGET-CONTAINER
    TargetContainer                                                        = 1247,
    /// TASK
    Task                                                                   = 77,
    /// TC
    Tc                                                                     = 450,
    /// TCP
    Tcp                                                                    = 507,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 1545,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 2011,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 2156,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 804,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 748,
    /// TD-EVENT-BSW
    TdEventBsw                                                             = 1118,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 2216,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 563,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 1692,
    /// TD-EVENT-COM
    TdEventCom                                                             = 1898,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 89,
    /// TD-EVENT-CYCLE-START
    TdEventCycleStart                                                      = 17,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 1945,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 987,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 1359,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 278,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 1569,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 1074,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 1885,
    /// TD-EVENT-SERVICE-INSTANCE
    TdEventServiceInstance                                                 = 715,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 1044,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 2086,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 1403,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 172,
    /// TD-EVENT-SLLET
    TdEventSllet                                                           = 679,
    /// TD-EVENT-SLLET-PORT
    TdEventSlletPort                                                       = 915,
    /// TD-EVENT-SWC
    TdEventSwc                                                             = 1342,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 284,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 1356,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 1299,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 1910,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 2128,
    /// TD-EVENT-VFB
    TdEventVfb                                                             = 1490,
    /// TD-EVENT-VFB-PORT
    TdEventVfbPort                                                         = 1780,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 195,
    /// TDLET-ZONE-CLOCK
    TdletZoneClock                                                         = 928,
    /// TE
    Te                                                                     = 1688,
    /// TERMINATE
    Terminate                                                              = 1539,
    /// TEST-FAILED
    TestFailed                                                             = 495,
    /// TEST-FAILED-BIT
    TestFailedBit                                                          = 1936,
    /// TEST-FAILED-THIS-OPERATION-CYCLE
    TestFailedThisOperationCycle                                           = 1950,
    /// TEST-PASSED
    TestPassed                                                             = 2070,
    /// TESTED
    Tested                                                                 = 1722,
    /// TESTED-AND-FAILED
    TestedAndFailed                                                        = 963,
    /// TG
    Tg                                                                     = 2103,
    /// TH
    Th                                                                     = 406,
    /// TI
    Ti                                                                     = 583,
    /// TIFF
    Tiff                                                                   = 1534,
    /// TIME
    Time                                                                   = 383,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 393,
    /// TIME-BASE-RESOURCE
    TimeBaseResource                                                       = 277,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 2117,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 153,
    /// TIME-SYNC-SERVER-CONFIGURATION
    TimeSyncServerConfiguration                                            = 1414,
    /// TIME-SYNCHRONIZATION-INTERFACE
    TimeSynchronizationInterface                                           = 24,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 1421,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 94,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 1845,
    /// TIMEOUT
    Timeout                                                                = 1532,
    /// TIMING-CLOCK
    TimingClock                                                            = 379,
    /// TIMING-CLOCK-SYNC-ACCURACY
    TimingClockSyncAccuracy                                                = 967,
    /// TIMING-CONDITION
    TimingCondition                                                        = 1925,
    /// TIMING-CONSTRAINT
    TimingConstraint                                                       = 2384,
    /// TIMING-DESCRIPTION
    TimingDescription                                                      = 1090,
    /// TIMING-DESCRIPTION-EVENT
    TimingDescriptionEvent                                                 = 1168,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 156,
    /// TIMING-EVENT
    TimingEvent                                                            = 475,
    /// TIMING-EXTENSION
    TimingExtension                                                        = 2236,
    /// TIMING-EXTENSION-RESOURCE
    TimingExtensionResource                                                = 1095,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 1649,
    /// TIP
    Tip                                                                    = 1608,
    /// TK
    Tk                                                                     = 1300,
    /// TL
    Tl                                                                     = 2251,
    /// TLS-12
    Tls12                                                                  = 1685,
    /// TLS-13
    Tls13                                                                  = 676,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 644,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 1415,
    /// TLS-CRYPTO-CIPHER-SUITE-PROPS
    TlsCryptoCipherSuiteProps                                              = 1747,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 650,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 649,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 1432,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 480,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 865,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 2028,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 575,
    /// TN
    Tn                                                                     = 714,
    /// TO
    To                                                                     = 1179,
    /// TOP
    Top                                                                    = 2466,
    /// TOPBOT
    Topbot                                                                 = 1600,
    /// TOPIC
    Topic                                                                  = 324,
    /// TOPIC-1
    Topic1                                                                 = 850,
    /// TOPIC-PREFIX
    TopicPrefix                                                            = 376,
    /// TP-ADDRESS
    TpAddress                                                              = 1886,
    /// TP-CONFIG
    TpConfig                                                               = 720,
    /// TP-CONNECTION-IDENT
    TpConnectionIdent                                                      = 1286,
    /// TR
    Tr                                                                     = 695,
    /// TRACE
    Trace                                                                  = 2469,
    /// TRACE-REFERRABLE
    TraceReferrable                                                        = 1631,
    /// TRACEABLE
    Traceable                                                              = 856,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 1212,
    /// TRACEABLE-TEXT
    TraceableText                                                          = 893,
    /// TRACED-FAILURE
    TracedFailure                                                          = 2177,
    /// TRANSFER
    Transfer                                                               = 626,
    /// TRANSFORMATION-PROPS
    TransformationProps                                                    = 2002,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 2060,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 1394,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 1402,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 469,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 1175,
    /// TRANSFORMER-ERROR-HANDLING
    TransformerErrorHandling                                               = 96,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 810,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 305,
    /// TRANSFORMING-I-SIGNAL
    TransformingISignal                                                    = 706,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 1423,
    /// TRANSLATION-START
    TranslationStart                                                       = 540,
    /// TRANSPORT
    Transport                                                              = 217,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 454,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 1878,
    /// TRAP
    Trap                                                                   = 585,
    /// TRIGGER
    Trigger                                                                = 2,
    /// TRIGGER-ACTIVATED
    TriggerActivated                                                       = 2019,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 2277,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 1464,
    /// TRIGGER-RELEASED
    TriggerReleased                                                        = 927,
    /// TRIGGER-UNICAST
    TriggerUnicast                                                         = 421,
    /// TRIGGERED
    Triggered                                                              = 687,
    /// TRIGGERED-ON-CHANGE
    TriggeredOnChange                                                      = 1786,
    /// TRIGGERED-ON-CHANGE-WITHOUT-REPETITION
    TriggeredOnChangeWithoutRepetition                                     = 1362,
    /// TRIGGERED-ON-EVALUATION
    TriggeredOnEvaluation                                                  = 415,
    /// TRIGGERED-WITHOUT-REPETITION
    TriggeredWithoutRepetition                                             = 894,
    /// TRUE
    True                                                                   = 1430,
    /// TS
    Ts                                                                     = 1716,
    /// TT
    Tt                                                                     = 267,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 1528,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 235,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 1476,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 813,
    /// TUNNEL
    Tunnel                                                                 = 1618,
    /// TW
    Tw                                                                     = 790,
    /// TX-REF-TRIGGER
    TxRefTrigger                                                           = 1254,
    /// TX-REF-TRIGGER-GAP
    TxRefTriggerGap                                                        = 147,
    /// TX-TRIGGER-MERGED
    TxTriggerMerged                                                        = 990,
    /// TX-TRIGGER-SINGLE
    TxTriggerSingle                                                        = 1646,
    /// UCM
    Ucm                                                                    = 909,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 1206,
    /// UCM-MASTER
    UcmMaster                                                              = 898,
    /// UCM-MASTER-MODULE-INSTANTIATION
    UcmMasterModuleInstantiation                                           = 1737,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 654,
    /// UCM-RETRY-STRATEGY
    UcmRetryStrategy                                                       = 1290,
    /// UCM-STEP
    UcmStep                                                                = 1599,
    /// UCM-SUBORDINATE-MODULE-INSTANTIATION
    UcmSubordinateModuleInstantiation                                      = 1723,
    /// UCM-TO-TIME-BASE-RESOURCE-MAPPING
    UcmToTimeBaseResourceMapping                                           = 2407,
    /// UDP
    Udp                                                                    = 1426,
    /// UDP-CHECKSUM-DISABLED
    UdpChecksumDisabled                                                    = 1683,
    /// UDP-CHECKSUM-ENABLED
    UdpChecksumEnabled                                                     = 2160,
    /// UDP-NM
    UdpNm                                                                  = 177,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 190,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 218,
    /// UDS
    Uds                                                                    = 2379,
    /// UK
    Uk                                                                     = 1060,
    /// UNDECIDED
    Undecided                                                              = 2318,
    /// UNDEFINED
    Undefined                                                              = 1494,
    /// UNIT
    Unit                                                                   = 2417,
    /// UNIT-GROUP
    UnitGroup                                                              = 2267,
    /// UNNUMBER
    Unnumber                                                               = 1613,
    /// UNSPECIFIED
    Unspecified                                                            = 188,
    /// UP-LINK-PORT
    UpLinkPort                                                             = 2188,
    /// UPDATE
    Update                                                                 = 1854,
    /// UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT
    UploadableExclusivePackageElement                                      = 710,
    /// UPLOADABLE-PACKAGE-ELEMENT
    UploadablePackageElement                                               = 1741,
    /// UR
    Ur                                                                     = 1961,
    /// USE-ARGUMENT-TYPE
    UseArgumentType                                                        = 464,
    /// USE-ARRAY-BASE-TYPE
    UseArrayBaseType                                                       = 1167,
    /// USE-FIRST-CONTEXT-DATA
    UseFirstContextData                                                    = 1433,
    /// USE-LAST-CONTEXT-DATA
    UseLastContextData                                                     = 1827,
    /// USE-VOID
    UseVoid                                                                = 472,
    /// USER-DEFINED
    UserDefined                                                            = 1738,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 1552,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 932,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 1918,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 29,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 1188,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 62,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 1728,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 1841,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 910,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 1883,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 1768,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 1080,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 1218,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 1823,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 32,
    /// USES-LOGGING
    UsesLogging                                                            = 2483,
    /// UZ
    Uz                                                                     = 1929,
    /// V-2-X-ACTIVE-SUPPORTED
    V2XActiveSupported                                                     = 35,
    /// V-2-X-DATA-MANAGER-NEEDS
    V2XDataManagerNeeds                                                    = 25,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 349,
    /// V-2-X-FACILITIES
    V2XFacilities                                                          = 1261,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 659,
    /// V-2-X-MANAGEMENT
    V2XManagement                                                          = 2041,
    /// V-2-X-NOT-SUPPORTED
    V2XNotSupported                                                        = 942,
    /// VALID
    Valid                                                                  = 587,
    /// VAR
    Var                                                                    = 1850,
    /// VAR-FAST
    VarFast                                                                = 37,
    /// VAR-NO-INIT
    VarNoInit                                                              = 2153,
    /// VAR-POWER-ON-INIT
    VarPowerOnInit                                                         = 298,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 1148,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 1331,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 2109,
    /// VARIABLE-DATA-PROTOTYPE-RECEIVED
    VariableDataPrototypeReceived                                          = 629,
    /// VARIABLE-DATA-PROTOTYPE-SENT
    VariableDataPrototypeSent                                              = 975,
    /// VARIABLE-SIZE
    VariableSize                                                           = 1231,
    /// VARIANT-LINK-TIME
    VariantLinkTime                                                        = 980,
    /// VARIANT-POST-BUILD
    VariantPostBuild                                                       = 778,
    /// VARIANT-POST-BUILD-LOADABLE
    VariantPostBuildLoadable                                               = 1966,
    /// VARIANT-POST-BUILD-SELECTABLE
    VariantPostBuildSelectable                                             = 88,
    /// VARIANT-PRE-COMPILE
    VariantPreCompile                                                      = 1838,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 843,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 1511,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 1320,
    /// VENDOR-SPECIFIC
    VendorSpecific                                                         = 1593,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 1271,
    /// VERBOSE
    Verbose                                                                = 1724,
    /// VERIFICATION
    Verification                                                           = 1633,
    /// VERIFY
    Verify                                                                 = 1897,
    /// VERTEX-OF-TARGET-CONTAINER
    VertexOfTargetContainer                                                = 102,
    /// VFB-TIMING
    VfbTiming                                                              = 1949,
    /// VI
    Vi                                                                     = 370,
    /// VIEW-MAP
    ViewMap                                                                = 2205,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 1454,
    /// VLAN-CONFIG
    VlanConfig                                                             = 578,
    /// VO
    Vo                                                                     = 1447,
    /// VOLATILE
    Volatile                                                               = 1075,
    /// WAIT-POINT
    WaitPoint                                                              = 869,
    /// WAIT-TIME-DATE
    WaitTimeDate                                                           = 492,
    /// WARMUP
    Warmup                                                                 = 1174,
    /// WARN
    Warn                                                                   = 1220,
    /// WARNING
    Warning                                                                = 1743,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 1397,
    /// WATCH-DOG-MANAGER
    WatchDogManager                                                        = 137,
    /// WATCH-TRIGGER
    WatchTrigger                                                           = 1912,
    /// WATCH-TRIGGER-GAP
    WatchTriggerGap                                                        = 1130,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 1024,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 1013,
    /// WEIGHTED-ROUND-ROBIN
    WeightedRoundRobin                                                     = 1652,
    /// WILL-CALL
    WillCall                                                               = 1260,
    /// WILL-RECEIVE
    WillReceive                                                            = 1996,
    /// WILL-SEND
    WillSend                                                               = 399,
    /// WO
    Wo                                                                     = 134,
    /// WONT-CALL
    WontCall                                                               = 7,
    /// WONT-RECEIVE
    WontReceive                                                            = 2074,
    /// WONT-SEND
    WontSend                                                               = 2273,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 2196,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 1393,
    /// WRITE
    Write                                                                  = 1274,
    /// WRITE-ONLY
    WriteOnly                                                              = 375,
    /// WRONG-TRIGGER
    WrongTrigger                                                           = 1498,
    /// X-509
    X509                                                                   = 2036,
    /// X-MII
    XMii                                                                   = 1071,
    /// X-MMI
    XMmi                                                                   = 2308,
    /// XCP
    Xcp                                                                    = 722,
    /// XCP-PDU
    XcpPdu                                                                 = 1135,
    /// XDOC
    Xdoc                                                                   = 1699,
    /// XFILE
    Xfile                                                                  = 1410,
    /// XG-MII
    XgMii                                                                  = 182,
    /// XH
    Xh                                                                     = 2042,
    /// XOR
    Xor                                                                    = 723,
    /// XREF-TARGET
    XrefTarget                                                             = 1610,
    /// XXG-MII
    XxgMii                                                                 = 2250,
    /// YO
    Yo                                                                     = 49,
    /// ZH
    Zh                                                                     = 875,
    /// ZU
    Zu                                                                     = 1200,
    /// default
    default                                                                = 824,
    /// preserve
    preserve                                                               = 1429,
}

impl EnumItem {
    const STRING_TABLE: [&'static str; 2494] = ["ADAPTIVE-SERVICE-OFFER-STARTED", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "TRIGGER", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "RO", "GLOBAL-TIME-MASTER", "INSTANCE-ID", "WONT-CALL", "DEFINE-BY-MEMORY-ADDRESS", "PLATFORM-ACTION-ITEM", "DIAGNOSTIC-IO-CONTROL-NEEDS", "CYCLE-REPETITION-5", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "IS-FAILED", "IDSM-MODULE-INSTANTIATION", "PERIODIC-EVENT-TRIGGERING", "TD-EVENT-CYCLE-START", "AFTERMARKET", "PERSISTENCY-FILE-PROXY", "QU", "PORT-BLUEPRINT", "AGGREGATION-TAILORING", "J-1939-TP-CONFIG", "TIME-SYNCHRONIZATION-INTERFACE", "V-2-X-DATA-MANAGER-NEEDS", "STATE-MANAGEMENT-STATE-NOTIFICATION", "CANCEL", "CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "USER-DEFINED-ETHERNET-FRAME", "LIN-FRAME", "COMPU-METHOD", "USER-DEFINED-TRANSFORMATION-PROPS", "GLOBAL-SUPERVISION-ENTITY", "CALLBACK", "V-2-X-ACTIVE-SUPPORTED", "FLEXRAY-NM-NODE", "VAR-FAST", "DIAGNOSTIC-TROUBLE-CODE-OBD", "SU", "CONST", "COM-MANAGEMENT-MAPPING", "RAW-DATA-STREAM-MAPPING", "DYNAMIC-PART-TRIGGER", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "EDGE-NODE", "CRYPTO-KEY-MANAGEMENT", "EXECUTABLE-TIMING", "OS-TASK-PROXY", "YO", "STATE-MANAGEMENT-MODULE-INSTANTIATION", "AUTOSAR-VARIABLE-INSTANCE", "SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS", "ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT", "SVG", "DIAGNOSTIC-SOVD-LOCK", "J-1939", "EVENT-MAPPING", "MODE-TRANSITION", "STEADY", "FRAME-PORT", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "USER-DEFINED-FIELD-DEPLOYMENT", "MACHINE", "AUTHENTICATE", "REPLACE", "PERSISTENCY-INTERFACE-ELEMENT", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "BH", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "DEFLATE", "PERIODIC-RATE-MEDIUM", "MAC-MULTICAST-GROUP", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "GLOBAL-TIME-FR-SLAVE", "API", "TASK", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "CODE", "SOMEIP-EVENT", "ETH-TCP-IP-PROPS", "ECU-PARTITION", "OBD", "ADAPTIVE-PLATFORM-SERVICE-INSTANCE", "LOCAL-SUPERVISION", "SOMEIP-TRANSFORMATION-PROPS", "DIAGNOSTIC-PARAMETER-IDENT", "VARIANT-POST-BUILD-SELECTABLE", "TD-EVENT-COMPLEX", "DIAGNOSTIC-EVENT-MANAGER", "CAN-TP-NODE", "SOMEIP-TP-CONFIG", "STATUS-BIT-AGING-AND-DISPLACEMENT", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "NFOLD", "TRANSFORMER-ERROR-HANDLING", "ATTRIBUTE-TAILORING", "BSW-MODULE-ENTITY-ACTIVATED", "CONTINUE-AT-IT-POSITION", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "DDS-EVENT-DEPLOYMENT", "VERTEX-OF-TARGET-CONTAINER", "SW-VARIABLE-PROTOTYPE", "DOCUMENTATION", "I-SIGNAL-I-PDU", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "DIAGNOSTIC-DATA-IDENTIFIER", "SECURITY-EVENT-CONTEXT-PROPS", "SERVICE-INTERFACE-PEDIGREE", "ADAPTIVE-METHOD-CALL-RECEIVED", "ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION", "NEW-IS-OUTSIDE", "SDG-DEF", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "SELECTED", "LIN-SPORADIC-FRAME", "JUSTIFY", "NEW-IS-WITHIN", "SHOW-CONTENT", "IS-LESS-OR-EQUAL", "CALCULATED", "CONSUMED-EVENT-GROUP", "ATP-BLUEPRINTABLE", "SA", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "CAN-BE-REMOVED", "LIMIT-TO-PAGE", "NON-VOLATILE", "DEFAULT-IF-REVISION-UPDATE", "LOCAL", "SECURITY-EVENT-THRESHOLD-FILTER", "LEAF-OF-TARGET-CONTAINER", "PLATFORM-PHM-ACTION-ITEM", "WO", "SW-CLASS-PROTOTYPE", "MC-DATA-INSTANCE", "WATCH-DOG-MANAGER", "SECOND-TO-FIRST", "PHM-ACTION", "READ-ONLY", "CRYPTO-SERVICE-QUEUE", "ADAPTIVE-AUTOSAR-APPLICATION", "GLOBAL-TIME-GATEWAY", "RES-AXIS", "NO-SLOPPY", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "TX-REF-TRIGGER-GAP", "ALL-PARTIAL-NETWORKS-ACTIVE", "BINARY-MANIFEST-ADDRESSABLE-OBJECT", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "FLEXRAY-AR-TP-NODE", "BSW-MODULE-TIMING", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "SECURED-PDU-HEADER-16-BIT", "TIMING-DESCRIPTION-EVENT-CHAIN", "SWC-BSW-MAPPING", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "OPAQUE", "BASE-TYPE", "MIXED-29-BIT", "SILENT", "SLAVE", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "ISO-11992--4", "NE", "CAUTION", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "LINK-LOCAL--DOIP", "NO-SHOW-ALIAS-NAME", "BSW-DEBUG-INFO", "TD-EVENT-SERVICE-INSTANCE-METHOD", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "CONFIRMED-DTC-BIT", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE", "UDP-NM", "SAFETY", "MIN", "KU", "RPT-LEVEL-3", "XG-MII", "J-1939-CONTROLLER-APPLICATION", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "ATOMIC-SW-COMPONENT-TYPE", "RESET-MACHINE", "MANUFACTURING", "UNSPECIFIED", "DIAGNOSTIC-IUMPR-GROUP", "UDP-NM-CLUSTER", "INTERNAL-TRIGGERING-POINT", "SO", "FUNCTION-INHIBITION-MANAGER", "CAN-COMMUNICATION-CONTROLLER", "TD-EVENT-VFB-REFERENCE", "N-PDU", "IPSEC", "SERVICE-FIELD-DEPLOYMENT", "SEC-OC-SECURE-COM-PROPS", "STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE", "CONCRETE-CLASS-TAILORING", "INLINE", "LATENCY-TIMING-CONSTRAINT", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "CAN-XL-PROPS", "BSW-IMPLEMENTATION", "PERSISTENCY-DEPLOYMENT", "SYNC-BASE-TIME-MANAGER", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "BSW-SCHEDULER-NAME-PREFIX", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "DIAGNOSTIC-CLEAR-CONDITION", "CP", "DOES-NOT-REPORT-EXECUTION-STATE", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "NM-PDU", "TRANSPORT", "UDP-NM-NODE", "AP-APPLICATION-ERROR", "NO-SHOW-CATEGORY", "DIAGNOSTIC-ENABLE-CONDITION", "PERIODIC-RATE-FAST", "SWC-INTERNAL-BEHAVIOR", "DIAGNOSTIC-INDICATOR-NEEDS", "GU", "DEFAULT-MODE", "ATP-STRUCTURE-ELEMENT", "DESCENDANT", "HW-PIN-GROUP", "DO-IP-SERVICE-NEEDS", "CAPTURE-SYNCHRONOUS-TO-REPORTING", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "ARBITRARY-EVENT-TRIGGERING", "CRYPTO-DRIVER", "TTCAN-COMMUNICATION-CONNECTOR", "MODE-DECLARATION-GROUP", "CAN-FRAME", "PNC-MAPPING-IDENT", "SECURITY-EVENT-DEFINITION", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "BSW-MODULE-CALL-POINT", "OUT", "STATE-MANAGEMENT-PORT-INTERFACE", "DE", "IGNITION", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "RESOURCE-GROUP", "GLOBAL-TIME-DOMAIN", "GLOBAL-SUPERVISION", "ETHERNET-FRAME", "NO-SHOW-CONTENT", "RECOVERY-NOTIFICATION", "REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE", "DIAGNOSTIC-MEMORY-BY-ADDRESS", "COMMUNICATION-CONTROLLER", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "OBD-CONTROL-SERVICE-NEEDS", "PERSISTENCY-KEY-VALUE-DATABASE", "MAPPING-SCOPE-ECU", "HW-ELEMENT", "I-PDU", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "POST-BUILD", "ONE-EVERY-N", "LIMIT-TO-TEXT", "TT", "ANY-STANDARDIZED", "NO-BREAK", "LN", "SERVER-MAC-VERIFY", "CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET", "FINISH", "PREDEFINED-VARIANT", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "TIME-BASE-RESOURCE", "TD-EVENT-I-PDU", "ON-COMPARISON-OF-VALUES", "I-PV-6-EXT-HEADER-FILTER-LIST", "CURVE-AXIS", "SCHEDULE-VARIANT-2", "ACCEPT-CONFIGURED", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "PT", "MODE-SWITCH-INTERFACE", "SDG-PRIMITIVE-ATTRIBUTE", "PEER", "ENABLED", "FIRST-CONTAINED-TRIGGER", "DEPENDENCY-ON-ARTIFACT", "ECUC-COMMON-ATTRIBUTES", "PL", "LIN-CLUSTER", "MIDDLE", "PORT-PROTOTYPE", "VAR-POWER-ON-INIT", "SECURITY-EVENT-STATE-FILTER", "BINARY-MANIFEST-PROVIDE-RESOURCE", "CONCRETE-PATTERN-EVENT-TRIGGERING", "AR-ELEMENT", "AP-APPLICATION-ENDPOINT", "KO", "TRANSFORMER-STATUS-FORWARDING", "DATA-TRANSFORMATION-SET", "PARTIAL-NETWORK", "RES_AXIS", "BLUEPRINT-MAPPING-SET", "NO-MONOTONY", "I-SIGNAL-I-PDU-GROUP", "CALIBRATION-VARIABLES", "IDSM-RATE-LIMITATION", "HW-ATTRIBUTE-DEF", "ASYMMETRIC-TO-BYTE-ARRAY", "FOR-ALL", "ROUTER", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "EID-USE-CONFIG-VALUE", "AR", "COM_AXIS", "CENTER", "TOPIC", "FALSE", "PROCESS-TO-MACHINE-MAPPING-SET", "BUILD-ACTION-MANIFEST", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "CLEAR-ALL-DTCS", "DIAGNOSTIC-DATA-BY-IDENTIFIER", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "DEADLINE-SUPERVISION", "IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "SECURE-COM-PROPS", "HEALTH-CHANNEL-EXTERNAL-MODE", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "END-2-END-EVENT-PROTECTION-PROPS", "BI", "ADAPTIVE-FIREWALL-MODULE-INSTANTIATION", "DIAGNOSTIC-ACCESS-PERMISSION", "IMPLEMENTATION", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "PAYLOAD-AS-ARRAY", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "ROTATE-90-CW-FIT-TO-TEXT", "COUPLING-ELEMENT", "FRAME-ETHERNET-RECEIVED-ON-BUS", "BSW-MODULE-DEPENDENCY", "V-2-X-FAC-USER-NEEDS", "MEASURED-EXECUTION-TIME", "DLT-ECU", "REQUIRED-AP-SERVICE-INSTANCE", "AUTO", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "DATA-FORMAT-ELEMENT-SCOPE", "AVB--IEEE-802--1-AS", "ECUC-INSTANCE-REFERENCE-DEF", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "NM-NODE", "STATE-MANAGEMENT-ACTION-ITEM", "IS-OK", "DIAG-RESPONSE", "PENDING", "BSW-MODULE-DESCRIPTION", "REQUEST", "ENUMERATION-MAPPING-TABLE", "ALWAYS", "HINT", "VI", "CAN-FD", "CRYPTO-PROVIDER", "ROUTER-ADVERTISEMENT", "DIAGNOSTIC-OPERATION-CYCLE", "WRITE-ONLY", "TOPIC-PREFIX", "CAN-TP-CONFIG", "PROCESS-DESIGN", "TIMING-CLOCK", "CP-SOFTWARE-CLUSTER-RESOURCE", "COM-OFFER-SERVICE-GRANT-DESIGN", "ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE", "TIME", "IP-IAM-REMOTE-SUBJECT", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "CLASS-CONTENT-CONDITIONAL", "CRYPTO-SERVICE-NEEDS", "FIELD", "DEFINE-BY-IDENTIFIER", "OBD-INFO-SERVICE-NEEDS", "CYCLE-REPETITION-40", "FM-FEATURE-SELECTION", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "RESOURCE-CONSUMPTION", "CRYPTO-SERVICE-KEY", "ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS", "MONO", "NM-CONFIG", "WILL-SEND", "BE", "REFERENCE-TAILORING", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "SERVER-MAC-GENERATE", "CO", "DEBOUNCE-DATA", "TH", "RECORD-VALUE-FIELD", "ARGUMENT-DATA-PROTOTYPE", "BINARY-MANIFEST-ITEM-DEFINITION", "CONTAINER-I-PDU", "SO-AD-ROUTING-GROUP", "DLT-CONTEXT", "COUPLING-PORT-SHAPER", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "TRIGGERED-ON-EVALUATION", "SOMEIP-REMOTE-MULTICAST-CONFIG", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "SEARCH-FOR-ALL", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "BSW-OPERATION-INVOKED-EVENT", "TRIGGER-UNICAST", "J-1939-DCM", "RUN-ONCE", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "SW-SERVICE-ARG", "FUNCTIONAL-CAN-FD", "SOMEIP-REQUIRED-EVENT-GROUP", "CAN-NM-CLUSTER", "DOES-NOT-SUPPORT-BUFFER-LOCKING", "DATA-TRANSFORMATION", "FIX_AXIS", "INTERFACE-MAPPING", "GD", "OEM-BOOT-RESP-APP", "IS-EXPIRED", "SLOW-FLASHING-MODE", "COM-FIND-SERVICE-GRANT", "ECC", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "CAN-FRAME-TRIGGERING", "DIAGNOSTIC-ECU-RESET-INTERFACE", "PLATFORM-HEALTH-MANAGEMENT-INTERFACE", "ABSTRACT-CLASS-TAILORING", "OBD-PID-SERVICE-NEEDS", "GENERAL-PURPOSE-CONNECTION", "CAN-PHYSICAL-CHANNEL", "RAW-DATA-STREAM-INTERFACE", "BSW-ENTRY-RELATIONSHIP-SET", "STATE-MANAGEMEN-PHM-ERROR-INTERFACE", "TC", "PARAMETER-ACCESS", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "PRIMITIVE", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "NOT-ACCESSIBLE", "REST-INTEGER-PROPERTY-DEF", "CAPTURE-SYNCHRONOUSLY-TO-REPORTING", "DIAGNOSTIC-TROUBLE-CODE-UDS", "LIN-NM-CLUSTER", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "RUNNABLE-ENTITY-STARTED", "IAM-MODULE-INSTANTIATION", "COMPOSITE-INTERFACE", "USE-ARGUMENT-TYPE", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "APPLICATION-DEFERRED-DATA-TYPE", "MASTER", "DIAGNOSTIC-MEMORY-IDENTIFIER", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "SERVICE-INTERFACE-METHOD-MAPPING", "USE-VOID", "RUNNABLE-ENTITY", "SYMMETRIC", "TIMING-EVENT", "JI", "SLOPPY", "LONG-HEADER", "BUS-MIRROR-CHANNEL-MAPPING", "TLS-JOB-MAPPING", "SECURE-ON-BOARD-COMMUNICATION", "EVENT-ACCEPTANCE-ENABLED", "FLEXRAY-TP-PDU-POOL", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "RAW", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "INTERGRITY-WITHOUT-CONFIDENTIALITY", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "ASYNCHRONOUS", "DIAGNOSTIC-PARAMETER-ELEMENT", "DHCPV-6", "WAIT-TIME-DATE", "ACL-OPERATION", "GET", "TEST-FAILED", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "DIAGNOSTIC-CONTROL-NEEDS", "GRANT-DESIGN", "RPT-ENABLER-RAM", "DEFICIT-ROUND-ROBIN", "NO-ACK", "SOVD-GATEWAY-INSTANTIATION", "SIGNAL-BASED", "DIAGNOSTIC-ROUTINE-SUBFUNCTION", "TCP", "SDG-ATTRIBUTE", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "BASE-T", "AUTO-IP--DOIP", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "PERSISTENCY-FILE", "CYCLIC-AND-ON-CHANGE", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "HEALTH-CHANNEL", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "ETHERNET-FRAME-TRIGGERING", "DLT-ARGUMENT", "SLP", "BSW-M-ENTRY-CALL-RETURNED", "ACCESS-PERMISSION-SERVICE-CLASS", "EU", "ON-DTC-STATUS-CHANGE", "FATAL", "ECUC-ABSTRACT-REFERENCE-DEF", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "DIAGNOSTIC-REQUEST-UPLOAD", "-500-MILES", "STANDARD", "REST-PRIMITIVE-PROPERTY-DEF", "NO-SHOW-SHORT-NAME", "BUILD-ACTION-ENTITY", "ICV-IGNORED", "I-SIGNAL-AVAILABLE-FOR-RTE", "EVAP", "TRANSLATION-START", "ABSTRACT-EVENT", "CALIBRATION-PARAMETER-VALUE-SET", "DATA-FORMAT-ELEMENT-REFERENCE", "ABSTRACT-SERVICE-INSTANCE", "SOFTWARE-PACKAGE", "INTERRUPT-CAT-2", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "INFO", "APPLICATION-COMPOSITE-DATA-TYPE", "DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE", "ADAPTIVE-METHOD-CALLED", "APPLICATION-PRIMITIVE-DATA-TYPE", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "NO-SUPPORT", "ROUGH-ESTIMATE-HEAP-USAGE", "MAPPING-SCOPE-PARTITION", "FAILURE-AND-SUCCESS", "DIAGNOSTIC-READ-DTC-INFORMATION", "SWC-IMPLEMENTATION", "ETHERNET-PHYSICAL-CHANNEL", "SOCKET-ADDRESS", "TD-EVENT-BSW-MODE-DECLARATION", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "NM-CLUSTER", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "APPLICATION-ENDPOINT", "FIX-AXIS", "AUTO-IPDHCPV-4", "DETERMINISTIC-SYNC-MASTER", "PERSISTENCY-KEY-VALUE-PAIR", "CHANNEL-A", "RECOVERY-VIA-APPLICATION-ACTION", "OTHER", "TLV-DATA-ID-DEFINITION-SET", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED", "VLAN-CONFIG", "CRYPTO-KEY-SLOT", "FM-FEATURE-MODEL", "AP-APPLICATION-ERROR-SET", "EID-USE-MAC", "TI", "PERSISTENCY-DEPLOYMENT-ELEMENT", "TRAP", "CVC", "VALID", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "AY", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "SH", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "CONSUMER", "PROTECT-LAMP", "HARDWARE-TEST-MANAGER", "SS", "DLT-MESSAGE", "ECUC-QUERY-EXPRESSION", "CYCLE-REPETITION-8", "INDIVIDUAL", "EVENT-STORAGE-DISABLED", "P-PORT-PROTOTYPE", "COMMUNICATION-CLUSTER", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "SECURE-COM-PROPS-SET", "HOOK", "ECUC-PARAM-CONF-CONTAINER-DEF", "SPEC-ELEMENT-SCOPE", "ICV-NOT-SUPPORTED", "REQUEST-NO-RETURN", "AB", "NM-NETWORK-HANDLE", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "SINGLE-LANGUAGE-REFERRABLE", "DEF-ITEM", "MEASUREMENT-POINT", "ON-ENTRY", "IS", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "MAC-SEC-KAY-PARTICIPANT", "RTE-EVENT-IN-SYSTEM-SEPARATION", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "PROCESS-PHM-ACTION-ITEM", "BSW-EVENT", "TRANSFER", "COM-MANAGER", "LEFT", "VARIABLE-DATA-PROTOTYPE-RECEIVED", "ETH-IP-PROPS", "SINGLE-OCCURRENCE", "STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE", "DIAGNOSTIC-J-1939-SPN-MAPPING", "RAW-DATA-STREAM-CLIENT-INTERFACE", "ACK-WITH-RT", "PERSISTENCY-KEY-VALUE-STORAGE", "DIAGNOSTIC-RESPONSE-ON-EVENT", "DOMAIN-PARTICIPANT-USER-DATA-QOS", "RESTART-APPLICATION", "KS", "RESET-MCU", "SOMEIP-TP-CHANNEL", "SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE", "TLS-CONNECTION-GROUP", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "MC-GROUP", "ASYNCHRONOUS-SERVER-CALL-POINT", "NOT-EQUAL", "TLS-DEPLOYMENT", "TLS-CRYPTO-SERVICE-MAPPING", "AFTER-SALES", "BOLDITALIC", "PROCESSING-STYLE-ASYNCHRONOUS", "UCM-MODULE-INSTANTIATION", "DIAGNOSTIC-ROUTINE-CONTROL", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "J-1939-TP-NODE", "NM-HANDLE-INACTIVE-TO-FUNCTION-GROUP-STATE", "V-2-X-M-USER-NEEDS", "CLIENT-MAC-VERIFY", "FIREWALL-RULE", "RETURN-ON-EVENT-CLEARED", "INTER-PARTITION-INTRA-ECU", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "JAVA", "ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF", "CAN-TP-ADDRESS", "PRIVATE-KEY", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "NOTHING", "ACL-PERMISSION", "LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING", "EVENT-WINDOW-INFINITE", "ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING", "TLS-13", "DDS-REQUIRED-SERVICE-INSTANCE", "SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "TD-EVENT-SLLET", "ECUC-INTEGER-PARAM-DEF", "PLAIN", "PTP--IEEE-1588--2008", "STACK-USAGE", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "CALLOUT", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "TRIGGERED", "REDUNDANT-PER-ELEMENT", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "MS", "REST-ABSTRACT-ENDPOINT", "NOT", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "OCCURENCE", "TR", "BSW-SERVICE-DEPENDENCY-IDENT", "DZ", "SERVER-AUTHENTICATE", "MC-FUNCTION", "DATA-PROTOTYPE-GROUP", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "HEALTH-CHANNEL-SUPERVISION", "DETAILED-BYPASSING-FILTERS", "DIAGNOSTIC-REQUEST-ROUTINE-RESULTS", "DATA-INTERFACE", "TRANSFORMING-I-SIGNAL", "PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR", "BURST-PATTERN-EVENT-TRIGGERING", "SOMEIP-FIELD", "UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT", "MEASURED-STACK-USAGE", "RECT", "RPT-EXECUTION-CONTEXT", "TN", "TD-EVENT-SERVICE-INSTANCE", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "SERVICE-INTERFACE-DEPLOYMENT", "KEEP-EXISTING", "TP-CONFIG", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "XCP", "XOR", "DEFAULT-TRACE-STATE-ENABLED", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "ACTIVATION-MULTICAST", "BSW-SCHEDULE-EVENT", "BSW-MODULE-ENTITY-STARTED", "SETTER", "INTERNAL-TRIGGER-OCCURRED-EVENT", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "LOGIC-ADDRESS", "PROVIDED-SERVICE-INSTANCE", "ECU-TIMING", "SYNCHRONIZATION-POINT-CONSTRAINT", "DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING", "RAW-DATA-STREAM-DEPLOYMENT", "DDS-SECURE-COM-PROPS", "AS", "SERVICE-NEEDS", "ETHERNET-COMMUNICATION-CONTROLLER", "DEFAULT-TRACE-STATE-DISABLED", "DATA-EXCHANGE-POINT", "FLEXRAY-FRAME-TRIGGERING", "STARTUP-CONFIG", "CODE-GENERATION-TIME", "REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "CS", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "ECU", "PNG", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "DIAGNOSTIC-AGING", "PERSISTENCY-FILE-STORAGE", "CYCLIC", "BSW-DISTINGUISHED-PARTITION", "DDS-RPC-SERVICE-DEPLOYMENT", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER", "PARAMETER-DATA-PROTOTYPE", "MODE-ACCESS-POINT-IDENT", "LIN-COMMUNICATION-CONTROLLER", "ABSTRACT-SECURITY-EVENT-FILTER", "RULE", "ECU-MANAGER", "SERVICE-INTERFACE-ELEMENT-MAPPING", "KM", "ACL-ROLE", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "PC-AFFECTS-PB", "DOCUMENTATION-CONTEXT", "DETERMINISTIC-CLIENT", "PORT-INTERFACE-MAPPING-SET", "ECUC-DESTINATION-URI-DEF", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE", "DIAGNOSTIC-IO-CONTROL", "VARIANT-POST-BUILD", "MT", "AH", "SERVICE-EVENT-DEPLOYMENT", "E-2-E-PROFILE-CONFIGURATION", "PHM-RULE", "SOMEIP-METHOD-DEPLOYMENT", "IDS-MAPPING", "OPERATION-CALL-RECEIVED", "FILTERED", "RUNNABLE-ENTITY-VARIABLE-ACCESS", "ECUC-DEFINITION-COLLECTION", "TW", "CAN-CLUSTER", "READ", "EVENT-ACCEPTANCE-DISABLED", "SEC-OC-CRYPTO-SERVICE-MAPPING", "ECUC-ADD-INFO-PARAM-DEF", "CONFIRMED", "SHORT-HEADER", "FLEXRAY-FRAME", "ROTATE-90-CCW", "NOT-AVAILABLE", "RESPONSE", "CRYPTO-CERTIFICATE", "PHM-ARBITRATION", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "LIN-COMMUNICATION-CONNECTOR", "LIN-UNCONDITIONAL-FRAME", "NET", "EXERCISE", "TRANSFORMER-HARD-ERROR-EVENT", "IDS-PLATFORM-INSTANTIATION", "ADAPTIVE-FIELD-GETTER-COMPLETED", "TTCAN-PHYSICAL-CHANNEL", "CAPTURE-ASYNCHRONOUSLY-TO-REPORTING", "SWC-SERVICE-DEPENDENCY", "RAW-DATA-STREAM-GRANT-DESIGN", "DIAGNOSTIC-CONDITION-GROUP", "MEMORY-SECTION", "OM", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "ETH-TP-CONFIG", "CRYPTO-MODULE-INSTANTIATION", "default", "SECURITY-EVENT-FILTER-CHAIN", "BINARY-MANIFEST-RESOURCE-DEFINITION", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "ECUC-ABSTRACT-STRING-PARAM-DEF", "GZIP", "ALL-SUPPORTED-DTCS", "STATIC-SOCKET-CONNECTION", "COMPILER", "NETWORK-CONFIGURATION", "DIAGNOSTIC-CONNECTED-INDICATOR", "HALF-DUPLEX-MODE", "RUNNABLE-ENTITY-TERMINATED", "SHOW-SHORT-NAME", "RW", "SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "AGREED", "IMMEDIATELY", "PROCESSOR-CORE", "VARIATION-POINT-PROXY", "EXECUTION-ORDER-CONSTRAINT", "CRYPTO-TRUST-MASTER-INTERFACE", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "AM", "STRICTLY-INCREASING", "BSW-BACKGROUND-EVENT", "TOPIC-1", "SECURED-PDU-HEADER-32-BIT", "HR", "ON-TRANSITION", "STIMULUS-SYNCHRONIZATION", "IA", "TRACEABLE", "CONSISTENCY-MECHANISM-REQUIRED", "INSTALL", "END-TO-END-PROTECTION", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "BASIC-SOFTWARE-MODE-MANAGER", "LIN-SLAVE", "SW-BASE-TYPE", "RSA", "TLS-JOB-REQUIREMENT", "FUNCTIONAL", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "SEC-OC-JOB-REQUIREMENT", "WAIT-POINT", "LISTEN", "DIAGNOSTIC-CONNECTION", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "ECUC-MULTILINE-STRING-PARAM-DEF", "ES", "ZH", "ECUC-URI-REFERENCE-DEF", "DIAGNOSTIC-DE-AUTHENTICATION", "COUPLING-PORT", "NM-HANDLE-ACTIVE-TO-FUNCTION-GROUP-STATE", "APPLICATION-ASSOC-MAP-DATA-TYPE", "OVERRIDE", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "CONSTANT-SPECIFICATION", "HI", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "BSW-CALLED-ENTITY", "TA", "KY", "SESSION-HANDLING-INACTIVE", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "NO-TRANSFORMER-STATUS-FORWARDING", "ECUC-CHOICE-CONTAINER-DEF", "TRACEABLE-TEXT", "TRIGGERED-WITHOUT-REPETITION", "DOES-NOT-USE-LOGGING", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "UCM-MASTER", "EOC-EVENT-REF", "ETHERNET-PRIORITY-REGENERATION", "SYNCHRONIZED", "COUPLING-PORT-SCHEDULER", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "LIN-SLAVE-CONFIG-IDENT", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "MODE-DECLARATION-GROUP-PROTOTYPE", "REST-RESOURCE-DEF", "UCM", "USER-DEFINED-I-PDU", "ABSTRACT-IAM-REMOTE-SUBJECT", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "NOTIFICATION", "CONTINUOUS-ON-MODE", "TD-EVENT-SLLET-PORT", "PURE-LOCAL-TIME-BASE", "SCHEDULE-VARIANT-5", "NEW-IS-DIFFERENT", "HW-DESCRIPTION-ENTITY", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "SYSTEM-TIMING", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "DIAGNOSTIC-START-ROUTINE", "DO-IP-ROUTING-ACTIVATION", "LIFE-CYCLE-STATE", "TRIGGER-RELEASED", "TDLET-ZONE-CLOCK", "FR", "COMMUNICATION-INTRA-PARTITION", "SHOW-SEE", "USER-DEFINED-COMMUNICATION-CONNECTOR", "FRAME", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "MULTILANGUAGE-REFERRABLE", "PHM-ACTION-LIST", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "KEYWORD-SET", "SENDER-RECEIVER-INTERFACE", "CY", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "V-2-X-NOT-SUPPORTED", "FIRE-AND-FORGET-MAPPING", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE", "REST-STRING-PROPERTY-DEF", "INDICATOR-STATUS-NEEDS", "STD_AXIS", "FLAT-INSTANCE-DESCRIPTOR", "SOMEIP-SERVICE-INTERFACE", "SYNCHRONIZED-SLAVE-TIME-BASE", "JW", "NO-SHOW-NUMBER", "SPORADIC-EVENT-TRIGGERING", "STATE-MANAGEMENT-NOTIFICATION-INTERFACE", "DIAGNOSTIC-SW-MAPPING", "RUNTIME-ERROR", "DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE", "FRAME-ETHERNET-SENT-ON-BUS", "BLOCK-STATE", "MALFUNCTION", "ENABLE", "DHCPV-4", "TESTED-AND-FAILED", "IS-LESS-THAN", "SEARCH-FOR-ID", "GIF", "TIMING-CLOCK-SYNC-ACCURACY", "FIBEX-ELEMENT", "MODE-DECLARATION-MAPPING-SET", "LIN-FRAME-TRIGGERING", "NEVER", "NTP--RFC-958", "LOG-AND-TRACE-INTERFACE", "ACL-OBJECT-SET", "VARIABLE-DATA-PROTOTYPE-SENT", "100BASE-T1", "DIAGNOSTIC-CONDITION-INTERFACE", "INIT-EVENT", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "VARIANT-LINK-TIME", "DO-IP-TP-CONFIG", "BLOCK-SOURCE", "PDUR-I-PDU-GROUP", "REPORT", "BR", "POST-BUILD-VARIANT-CRITERION", "TD-EVENT-FRAME", "SCHEDULE-VARIANT-7", "PDU-ACTIVATION-ROUTING-GROUP", "TX-TRIGGER-MERGED", "OBD-DCY", "GETTER", "STATIC-OR-DYNAMIC-PART-TRIGGER", "REQUEST-CALLBACK-TYPE-SUPPLIER", "COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING", "ERROR-TRACER", "LOW", "FULL-DUPLEX-MODE", "LOGICAL-SUPERVISION", "LAST-IS-BEST", "DIAGNOSTIC-MEMORY-DESTINATION", "NO-PROTECTION", "CHANNEL-B", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "IT", "REFERRABLE", "SYSTEM-SIGNAL", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "DONT-INVALIDATE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "FM-FEATURE-MAP", "J-1939-NM-CLUSTER", "WATCHDOG-PHM-ACTION-ITEM", "BROAD-R-REACH", "COLLECTION", "RUNNABLE-ENTITY-GROUP", "BLINK-OR-CONTINUOUS-ON-MODE", "STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM", "I-PDU-TRIGGERING", "NV-BLOCK-SW-COMPONENT-TYPE", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "IDENTIFIABLE", "FM-FEATURE-RESTRICTION", "WATCHDOG-ACTION-ITEM", "FIT-TO-PAGE", "PUBLIC-KEY", "GLOBAL-SUPERVISION-NEEDS", "SYSTEM", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "ADAPTIVE-SERVICE-FIND-STARTED", "STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM", "PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING", "ECUC-VALUE-COLLECTION", "J-1939-DCM-I-PDU", "APPLICATION-ARRAY-ELEMENT", "HW-PIN", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "SHOW-ALIAS-NAME", "MODE-DECLARATION", "NEW-IS-EQUAL", "SM", "GLOBAL-TIME-CAN-SLAVE", "ROTATE-180-LIMIT-TO-TEXT", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "LT", "ECUC-DESTINATION-URI-DEF-SET", "MODE-DECLARATION-MAPPING", "SWC-TO-APPLICATION-PARTITION-MAPPING", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "ITALIC", "BSW-COMPOSITION-TIMING", "DIAGNOSTIC-ECU-RESET", "SOMEIP-METHOD", "LIN-PHYSICAL-CHANNEL", "BUS-MIRROR-CHANNEL-MAPPING-IP", "OPERATION-CALLED", "ALIVE-SUPERVISION", "SHOW-PAGE", "UK", "NO-NEWLINE", "ICV-SUPPORTED", "IS-LESS-THAN-OR-EQUAL", "DLT-USER-NEEDS", "FUNCTION-INHIBITION-NEEDS", "NL", "ADAPTIVE-FIELD-SETTER-COMPLETED", "BSW-MODE-SWITCH-EVENT", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "DEFAULT-IF-UNDEFINED", "X-MII", "DIAGNOSTIC-DATA-TRANSFER", "DEFAULT", "TD-EVENT-MODE-DECLARATION", "VOLATILE", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "NEWLINE-IF-NECESSARY", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "ANALYZED-EXECUTION-TIME", "USER-DEFINED-PHYSICAL-CHANNEL", "SENT-TAGGED", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "SERVER-ENCRYPT", "RPT-LEVEL-1", "SW", "IDENT-CAPTION", "COM-FIELD-GRANT", "TIMING-DESCRIPTION", "DDS-SECURE-GOVERNANCE", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "ORDINARY-EOC", "SUPPORTS-BUFFER-LOCKING", "TIMING-EXTENSION-RESOURCE", "NEW-IS-GREATER", "IS-NOT-EQUAL", "CUSTOM", "OFFSET", "J-1939-DCM-DM-19-SUPPORT", "ECUC-PARAMETER-DEF", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "DIAGNOSTIC-AUTH-ROLE", "JA", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "KEY-DERIVATION", "GLOBAL-TIME-FR-MASTER", "REJECT", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "REST-ELEMENT-DEF", "APPLICATION-RECORD-ELEMENT", "AGE", "RAW-DATA", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "EXTERNAL-TRIGGERING-POINT-IDENT", "BOTTOM", "TD-EVENT-BSW", "PROVIDED-DDS-SERVICE-INSTANCE", "ALLOW", "ROTATE-90-CCW-FIT-TO-TEXT", "SW-CALIBRATION-METHOD", "DIAGNOSTIC-SOVD-PORT-INTERFACE", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "SINGLE-CORE-REENTRANT", "OS-TASK-EXECUTION-EVENT", "DA", "AMBER-WARNING", "WATCH-TRIGGER-GAP", "DIAGNOSTIC-DATA-PORT-MAPPING", "DISABLE", "ROOT-SW-COMPONENT-PROTOTYPE", "COM-GRANT-DESIGN", "XCP-PDU", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "PHM-CHECKPOINT", "CYCLE-REPETITION-4", "MODE-SWITCH-POINT", "ECUC-MODULE-DEF", "ANY", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "FM-FEATURE-MAP-ELEMENT", "PHM-HEALTH-CHANNEL-STATUS", "ACTIVATE", "FA", "DIAGNOSTIC-SERVICE-CLASS", "VARIABLE-ACCESS", "EMISSION-RELATED-DTC", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "FM-FEATURE-MAP-ASSERTION", "ALIAS-NAME-SET", "CLEAR", "SQ", "BA", "SOVD-SERVER-INSTANTIATION", "CRYPTO-PROVIDER-INTERFACE", "I-PDU-SENT-TO-IF", "PER-EXECUTABLE", "MEMORY-USAGE", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "CLOSED", "CHAPTER", "HA", "FIRST-TO-SECOND", "PORT-INTERFACE-DEFINITION", "USE-ARRAY-BASE-TYPE", "TIMING-DESCRIPTION-EVENT", "REPORT-BEFORE-INIT", "PERSISTENCY-DATA-ELEMENT", "PHM-LOGICAL-EXPRESSION", "LOWER-12-BIT", "MASEKD-NEW-EQUALS-MASKED-OLD", "WARMUP", "TRANSFORMATION-TECHNOLOGY", "I-PDU-PORT", "RESPOND-BEFORE-RESET", "RAPID-PROTOTYPING-SCENARIO", "TO", "LA", "GLOBAL-TIME-ETH-SLAVE", "RESPOND-AFTER-RESET", "ARTIFACT-CHECKSUM", "SK", "SWC-TO-IMPL-MAPPING", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "SEARCH-FOR-SPECIFIC-INSTANCE", "USER-DEFINED-EVENT-DEPLOYMENT", "ATP-BLUEPRINT", "SECURE-COMMUNICATION-PROPS-SET", "SECURITY-EVENT-AGGREGATION-FILTER", "PA", "MODE-DECLARATION-SWITCH-INITIATED", "PHYSICAL", "FRAME-ETHERNET-RECEIVED-BY-IF", "SERVER-VERIFY", "E-2-E-PROFILE-CONFIGURATION-SET", "SIGNATURE", "BMP", "ZU", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "SERVICE-INTERFACE-MAPPING", "IS-RELEVANT", "CYCLE-REPETITION-1", "SYSTEM-MEMORY-USAGE", "UCM-DESCRIPTION", "STATE-MANAGEMENT-ACTION-LIST", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY", "DIAGNOSTIC-STOP-ROUTINE", "RPT-EXECUTABLE-ENTITY-EVENT", "ON-CHANGE-OF-DATA-IDENTIFIER", "TRACEABLE-TABLE", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "CRC-OPTIONAL", "MINIMUM-MINOR-VERSION", "BUILD-ACTION", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "CRC-IGNORED", "WARN", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER", "ATP-CLASSIFIER", "APPLICATION-ONLY", "PRE-COMPILE", "GENERAL-PURPOSE-PDU", "ADAPTIVE-MODULE-INSTANTIATION", "CYCLE-REPETITION-20", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE", "EXCLUSIVE-AREA-NESTING-ORDER", "DLT-LOG-CHANNEL", "VARIABLE-SIZE", "START", "ALL-16-BIT", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "BRIEF-BYPASSING-FILTERS", "SEC-OC-DEPLOYMENT", "DLT-LOG-SINK", "I-PDU-RECEIVED-BY-COM", "IS-GREATER-OR-EQUAL", "CPP", "SECURITY", "SATURATE", "IK", "SIGN", "STATE-MANAGEMENT-ERROR-INTERFACE", "NO-TRANSFORMER-ERROR-HANDLING", "TARGET-CONTAINER", "FIELD-MAPPING", "PROVIDED-AP-SERVICE-INSTANCE", "DIAGNOSTIC-SERVICE-SW-MAPPING", "SINGLE", "ECUC-QUERY", "PACKAGEABLE-ELEMENT", "TX-REF-TRIGGER", "POLY", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING", "MODE-DECLARATION-REQUESTED", "DATA-TYPE-MAPPING-SET", "WILL-CALL", "V-2-X-FACILITIES", "ECUC-CONTAINER-VALUE", "NOT-VALID", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "CRYPTO-NEEDS", "CAPTURE-ASYNCHRONOUS-TO-REPORTING", "STATE-MANAGEMENT-REQUEST-ERROR", "BRIEF", "END-TO-END-PROTECTION-SET", "SW-CALPRM-PROTOTYPE", "VENDOR-SPECIFIC-SERVICE-NEEDS", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "BSW-MODE-MANAGER-ERROR-EVENT", "WRITE", "BUILD-TYPE-DEBUG", "PGWIDE", "REST-ARRAY-PROPERTY-DEF", "INTERPOLATION-ROUTINE-MAPPING-SET", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "DIAGNOSTIC-SESSION-CONTROL", "ISO-14229--1", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "COM-FIND-SERVICE-GRANT-DESIGN", "COMMAND-LINE-SIMPLE-FORM", "LIN-TP-CONFIG", "TP-CONNECTION-IDENT", "SHOW-CATEGORY", "OPEN", "NETWORK-ENDPOINT", "UCM-RETRY-STRATEGY", "COLLECTABLE-ELEMENT", "PAYLOAD-AS-POINTER-TO-ARRAY", "DIAGNOSTIC-FIM-ALIAS-EVENT", "MY", "DIAGNOSTIC-UPLOAD-INTERFACE", "ASYMMETRIC-FROM-BYTE-ARRAY", "FM-FEATURE-MAP-CONDITION", "SYNCHRONOUS-SERVER-CALL-POINT", "TD-EVENT-TRIGGER", "TK", "PARAMETER-INTERFACE", "DEFAULT-TRIGGER", "DIAGNOSTIC-TRANSFER-EXIT", "PARTITION", "ICV-NOT-VERIFIED", "NO", "PHM-ACTION-ITEM", "C", "SOMEIP-REMOTE-UNICAST-CONFIG", "SDG-CAPTION", "RPT-SERVICE-POINT", "AFTERMAKET", "METHOD-MAPPING", "APPLICATION-DATA-TYPE", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "DO-IP-POWER-MODE-STATUS-NEEDS", "REST-BOOLEAN-PROPERTY-DEF", "PROTECTED", "DIAGNOSTIC-EVENT-NEEDS", "VEHICLE-ROLLOUT-STEP", "EXECUTABLE-GROUP", "SDG-ABSTRACT-FOREIGN-REFERENCE", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "BOLD", "FM-FEATURE", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "CAN-BE-TERMINATED", "100BASE-TX", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "IS-GREATER-THAN-OR-EQUAL", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "DIAGNOSTIC-STORAGE-CONDITION", "ABSTRACT-IMPLEMENTATION-DATA-TYPE", "ADAPTIVE-FIELD-SETTER-CALLED", "DIAGNOSTIC-COMPONENT-NEEDS", "SL", "COM-EVENT-GRANT-DESIGN", "SYSTEM-DESIGN-TIME", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "OEM-BOOT", "SUPERVISION-CHECKPOINT", "TD-EVENT-SWC", "BUILD", "IS-GREATER-THAN", "SDG-REFERENCE", "BSW-INTERRUPT-ENTITY", "ONLY-THIS-CYCLE-AND-READINESS", "SHOW-LONG-NAME", "RAW-DATA-STREAM-SERVER-INTERFACE", "STORE-PERSISTENTLY", "CRYPTO-PRIMITIVE", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "DIAGNOSTIC-LOG-AND-TRACE", "SO-CON-I-PDU-IDENTIFIER", "SERVICE-INTERFACE-EVENT-MAPPING", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "IE", "CAN-BE-TERMINATED-AND-RESTARTED", "TD-EVENT-FRAME-ETHERNET", "DDS-DOMAIN-RANGE", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "TRIGGERED-ON-CHANGE-WITHOUT-REPETITION", "FRAME-TRIGGERING", "LIN-SCHEDULE-TABLE", "RESET-ECU", "PROCESS", "COMMON", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "ETHERNET-NETWORK-CONFIGURATION", "LINK-LOCAL", "GLOBAL-TIME-SLAVE", "COM-TRIGGER-GRANT-DESIGN", "SERVICE-INSTANCE-TO-MACHINE-MAPPING", "APPLICATION-ERROR", "ACCEPT-ALL", "REPETITIVE-EOC", "CRYPTO-JOB", "ACK-WITHOUT-RT", "SECURED-PDU-HEADER-08-BIT", "ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED", "NO-SUPERVISION", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "POWER", "ACCES-PERRMISSION-SERVICE-CLASS", "DETERMINISTIC-SYNC-INSTANTIATION", "HARDWARE-TEST-NEEDS", "EOC-EXECUTABLE-ENTITY-REF", "FM-ATTRIBUTE-DEF", "IP-SEC-RULE", "ABSTRACT", "WORST-CASE-STACK-USAGE", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "SIGNAL-BASED-EVENT-DEPLOYMENT", "OBD-RATIO-SERVICE-NEEDS", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "ANY-SEND-OPERATION", "BINARY-MANIFEST-REQUIRE-RESOURCE", "DDS-FIELD-DEPLOYMENT", "RETURN-VALUE-PROVIDED", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "TD-EVENT-SERVICE-INSTANCE-FIELD", "CRC-NOT-SUPPORTED", "OPTIONS", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "OBD-DRIVING-CYCLE", "STATE-MANAGEMENT-EM-ERROR-INTERFACE", "IDS-COMMON-ELEMENT", "XFILE", "RPT-ENABLER-ROM", "BUILD-TYPE-RELEASE", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "TIME-SYNC-SERVER-CONFIGURATION", "TLS-CRYPTO-CIPHER-SUITE", "10BASE-T1S", "ROOT-SW-COMPOSITION-PROTOTYPE", "I-SIGNAL-TO-I-PDU-MAPPING", "STATE-MANAGEMENT-SYNC-ACTION-ITEM", "INLINE-CONDITIONAL", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "POST", "TRANSIENT-FAULT", "ABSTRACT-ACCESS-POINT", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "UDP", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "BSW-MODULE-ENTITY", "preserve", "TRUE", "DIAGNOSTIC-EVENT", "TLS-IAM-REMOTE-SUBJECT", "USE-FIRST-CONTEXT-DATA", "COM-AXIS", "ECUC-CONTAINER-DEF", "ROTATE-90-CW-LIMIT-TO-TEXT", "ACTIVE", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "EXECUTE", "CHECKPOINT-TRANSITION", "INTERFACE-MAPPING-SET", "PLATFORM-MODULE-ENDPOINT-CONFIGURATION", "DIAGNOSTIC-ROUTINE-INTERFACE", "ADDR-METHOD-SHORT-NAME", "FDC-THRESHOLD", "SERVICE-PROXY-SW-COMPONENT-TYPE", "VO", "CRYPTO-SIGNATURE-SCHEME", "EVALUATED-VARIANT-SET", "ECUC-STRING-PARAM-DEF", "DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS", "1000BASE-T", "APPLICATION-ACTION-ITEM", "VIEW-MAP-SET", "CHECK-AT-NEXT-HALT", "IEEE802-1AS-AUTOSAR", "HU", "CRC-SUPPORTED", "DIAGNOSTIC-J-1939-SPN", "IEEE802-1AS", "PASSTHROUGH", "CONSTANT-SPECIFICATION-MAPPING-SET", "SDG-CLASS", "TRIGGER-INTERFACE-MAPPING", "FLEXRAY-AR-TP-CONFIG", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "COMMUNICATION-INTER-ECU", "EXTENDED", "POSSIBLE-ERROR-REACTION", "ABSTRACT-ETHERNET-FRAME", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "RPT-PROFILE", "EXAMPLE", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "ETH-TCP-IP-ICMP-PROPS", "TTCAN-COMMUNICATION-CONTROLLER", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "NO-TRUSTED-PLATFORM-SUPPORT", "ERROR-DETECTION", "PRIMITIVE-ATTRIBUTE-TAILORING", "IMPLEMENTATION-DATA-TYPE", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "DO-IP-ACTIVATION-LINE-NEEDS", "PROCESS-EXECUTION-ERROR", "IP-SEC-IAM-REMOTE-SUBJECT", "EXCLUDE-FROM-FLASH", "SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "BSW-INTERNAL-TRIGGERING-POINT", "TD-EVENT-VFB", "DLT-LOG-CHANNEL-DESIGN", "IEEE802-11P", "INTERGRITY-AND-CONFIDENTIALITY", "UNDEFINED", "FJ", "DEVELOPMENT-ERROR", "EVENT-WINDOW-CURRENT-CYCLE", "WRONG-TRIGGER", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "COM-GRANT", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "GENERAL-PURPOSE-I-PDU", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "SW-MC-INTERFACE", "BLUEPRINT-DERIVATION-TIME", "FIXED-SIZE", "ALLOCATOR", "NORMALFIXED", "ECUC-MODULE-CONFIGURATION-VALUES", "VEHICLE-PACKAGE", "SUPPLIER", "DIAGNOSTIC-INDICATOR-INTERFACE", "SIMULATED-EXECUTION-TIME", "DETAILED", "SW-COMPONENT-MAPPING-CONSTRAINTS", "CAN-NM-NODE", "DIAGNOSTIC-CONTRIBUTION-SET", "INSTRUCTION", "I-SIGNAL-PORT", "ETHERNET-RAW-DATA-STREAM-MAPPING", "COMMUNICATION-CONNECTOR", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "DIAGNOSTIC-COMMUNICATION-MANAGER", "IDSM-INSTANCE", "NM-ECU", "EVENT-STORAGE-ENABLED", "TTCAN-CLUSTER", "SERVICE-DISCOVERY", "DIAGNOSTIC-J-1939-SW-MAPPING", "RN", "TIMEOUT", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "TIFF", "AP-APPLICATION-ERROR-DOMAIN", "APPLICATION-SW-COMPONENT-TYPE", "OBSERVER-BASED", "FLEXRAY-CLUSTER", "TERMINATE", "DEDICATED", "DIAGNOSTIC-COMMON-ELEMENT", "KEYWORD", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "PERSISTENCY-INTERFACE", "TCP-OPTION-FILTER-LIST", "SECURITY-EVENT-REPORT-INTERFACE", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "NV-DATA-INTERFACE", "CYCLE-REPETITION-50", "LAST-FAILED", "DIAGNOSTIC-TEST-RESULT", "USER-DEFINED-CLUSTER", "INTERRUPT", "PDU", "SWC-TO-ECU-MAPPING", "STD", "BREAK", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "GATEWAY", "NO-FLOAT", "SG", "CRC-NOT-VALIDATED", "RUNNABLE-ENTITY-ACTIVATED", "SW-GENERIC-AXIS-PARAM-TYPE", "ABSTRACT-CAN-CLUSTER", "ATP-TYPE", "MACHINE-DESIGN", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED", "TD-EVENT-I-SIGNAL", "SPECIFICATION-DOCUMENT-SCOPE", "CAT-2", "DIAGNOSTIC-REQUEST-DOWNLOAD", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "START-FROM-BEGINNING", "I-SIGNAL-TRIGGERING", "CRYPTO-SERVICE-JOB-NEEDS", "MACRO", "PORT-GROUP", "INCREASING", "PHYSICAL-DIMENSION-MAPPING-SET", "DECREASING", "INFINITE-TIME-TO-RESPONSE", "ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT", "SESSION-HANDLING-ACTIVE", "INTERNAL-BEHAVIOR", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "HW-ATTRIBUTE-LITERAL-DEF", "END-2-END-METHOD-PROTECTION-PROPS", "ROTATE-180", "COM-METHOD-GRANT-DESIGN", "NON-OS-MODULE-INSTANTIATION", "SV", "VENDOR-SPECIFIC", "STD-AXIS", "DIAGNOSTIC-MONITOR-INTERFACE", "I-SIGNAL-GROUP", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "CYCLE-REPETITION-16", "UCM-STEP", "TOPBOT", "CALIBRATION-ONLINE", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "DEBUG", "OBSERVER", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "RPT-ENABLER-RAM-AND-ROM", "ERROR", "TIP", "SOMEIP-PROVIDED-EVENT-GROUP", "XREF-TARGET", "FRAME-RECEIVED-BY-IF", "DROP-FRAME", "UNNUMBER", "MOST-SIGNIFICANT-BYTE-LAST", "DIAGNOSTIC-AUTHENTICATION", "HIERARCHICAL-EOC", "COMMAND-LINE-LONG-FORM", "TUNNEL", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE", "ST", "FLEXRAY-TP-NODE", "DLT-APPLICATION", "CLIENT-DECRYPT", "APPLICATION-ASSOC-MAP-ELEMENT", "ARRAY", "INDICATE", "LIFE-CYCLE-INFO-SET", "AZ", "RESET-VM", "NUMBER", "TRACE-REFERRABLE", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "VERIFICATION", "OS-MODULE-INSTANTIATION", "COMPOSITION-SW-COMPONENT-TYPE", "CONFIGURED", "DEPENDANT", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "OR", "CONCRETE", "CRC-VALIDATED", "RPT-COMPONENT", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "SI", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "TX-TRIGGER-SINGLE", "APPLICATION-RECORD-DATA-TYPE", "MODELED", "TIMING-MODE-INSTANCE", "SW-CLASS-INSTANCE", "BONJOUR", "WEIGHTED-ROUND-ROBIN", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "NEW-IS-LESS", "SOMEIP", "PHYSICAL-CAN-FD", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "DATA-CONSTR", "OBD-MONITOR-SERVICE-NEEDS", "EXECUTABLE", "J-1939-SHARED-ADDRESS-CLUSTER", "CONSUMED-SERVICE-INSTANCE", "PHYSICAL-DIMENSION", "AF", "CONNECT", "BO", "FLEXRAY-COMMUNICATION-CONTROLLER", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED", "COM-METHOD-GRANT", "AUTOSAR-DATA-TYPE", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "ET", "JPG", "FAULT", "SN", "ABSTRACT-REQUIRED-PORT-PROTOTYPE", "MASKED-NEW-EQUALS-X", "CA", "ESP", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "ABSTRACT-CAN-COMMUNICATION-CONTROLLER", "SECONDARY-ECU", "UDP-CHECKSUM-DISABLED", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "TLS-12", "SERVICE-INTERFACE-FIELD-MAPPING", "LINK", "TE", "ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS", "SEARCH-FOR-ALL-INSTANCES", "PRECONFIGURED-CONFIGURATION", "TD-EVENT-BSW-MODULE", "NV-RAM-MANAGER", "NOT-DEFINED", "NO-STATUS-BYTE-CHANGE", "SOFTWARE-CLUSTER-REQUIREMENT", "APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE", "PROCESSING-STYLE-SYNCHRONOUS", "XDOC", "MN", "ETHERNET-CLUSTER", "NM-INSTANTIATION", "SSDP", "EXECUTABLE-ENTITY", "BSW-OS-TASK-EXECUTION-EVENT", "EVENT-COMBINATION-ON-RETRIEVAL", "I-SIGNAL-SENT-TO-COM", "FURTHER-ACTION-BYTE-NEEDS", "APPLICATION-ARRAY-DATA-TYPE", "DIAGNOSTIC-SECURITY-ACCESS", "CAN-COMMUNICATION-CONNECTOR", "ECU-INSTANCE", "NEW-IS-LESS-OR-EQUAL", "DIAGNOSTIC-MAPPING", "PSK", "TS", "SWC", "SIGNAL-BASED-METHOD-DEPLOYMENT", "PHM-SUPERVISED-ENTITY-INTERFACE", "RTPGE", "LV", "TESTED", "UCM-SUBORDINATE-MODULE-INSTANTIATION", "VERBOSE", "EQUAL", "OPERATION-INVOKED-EVENT", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "USER-DEFINED-GLOBAL-TIME-MASTER", "BINARY-MANIFEST-META-DATA-FIELD", "COUPLING-PORT-STRUCTURAL-ELEMENT", "SOMEIP-EVENT-DEPLOYMENT", "APPLICATION-PARTITION", "SERIALIZER", "LAND", "DIAGNOSTIC-ABSTRACT-ALIAS-EVENT", "DDS-TOPIC-ACCESS-RULE", "UCM-MASTER-MODULE-INSTANTIATION", "USER-DEFINED", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "PORT", "UPLOADABLE-PACKAGE-ELEMENT", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "WARNING", "INFINITE", "PHM-HEALTH-CHANNEL-INTERFACE", "FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION", "TLS-CRYPTO-CIPHER-SUITE-PROPS", "BLOCK", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "MAX", "GETTER-SETTER", "BSW-TIMING-EVENT", "MONOTONOUS", "DCM-I-PDU", "CAT-1", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "CP-SOFTWARE-CLUSTER", "SYNCHRONIZATION-TIMING-CONSTRAINT", "SUPERVISION-MODE", "ERROR-CORRECTION", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "MI", "APPLICATION-INTERFACE", "AGE-CONSTRAINT", "STANDARD-PORT", "ECU-MAPPING", "SOFTWARE-ACTIVATION-DEPENDENCY", "USER-DEFINED-PDU", "SW-SERVICE-PROTOTYPE", "DO-IP-GID-NEEDS", "SOVD-MODULE-INSTANTIATION", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "CRYPTO-SERVICE-CERTIFICATE", "NAND", "KN", "ABSTRACT-CAN-PHYSICAL-CHANNEL", "PTP--IEEE-1588--2002", "FIT-TO-TEXT", "SW-SYSTEMCONST", "TD-EVENT-VFB-PORT", "HIGH", "DATA-WRITE-COMPLETED-EVENT", "MULTIPLE-OCCURRENCES", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "LOWER-8-BIT", "TRIGGERED-ON-CHANGE", "PORT-PROTOTYPE-BLUEPRINT", "BACKGROUND-EVENT", "BSW-MODULE-CLIENT-SERVER-ENTRY", "SECTION-NAME-PREFIX", "DIAGNOSTIC-TROUBLE-CODE", "SECURITY-EVENT-MAPPING", "NO-SHOW-LONG-NAME", "ACTION", "AUTOSAR-DATA-PROTOTYPE", "SENT-UNTAGGED", "CRYPTO-SERVICE-MANAGER", "FUNCTIONAL-ADDRESS", "NV-BLOCK-DESCRIPTOR", "CONFIDENTIALITY-OFFSET--30", "SERIALIZATION-TECHNOLOGY", "FO", "DIAGNOSTIC-COM-CONTROL-CLASS", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "SR", "RPT-EXECUTABLE-ENTITY", "SDG-TAILORING", "SERVICE-INSTANCE-COLLECTION-SET", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "REPORT-AFTER-INIT", "EN", "SHOW-NUMBER", "IDSM-PROPERTIES", "SOFTWARE-CLUSTER", "EVENT-COMBINATION-ON-STORAGE", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "OC", "ECUC-FOREIGN-REFERENCE-DEF", "ENCRYPT-AND-SIGN", "PERSISTENCY-FILE-ARRAY", "HEALTH-CHANNEL-EXTERNAL-STATUS", "ISO-15031--6", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "COMMAND-LINE-SHORT-FORM", "CONFIDENTIALITY-OFFSET--50", "ADAPTIVE-EVENT-SENT", "USE-LAST-CONTEXT-DATA", "SUPERVISION-ENTITY", "NO-CONSISTENCY-MECHANISM", "REST-SERVICE-INTERFACE", "NEWLINE", "NO-SHOW-SEE", "PS", "CYCLE-REPETITION-64", "GLOBAL-TIME-ETH-MASTER", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "VARIANT-PRE-COMPILE", "ON-EXIT", "NONE", "USER-DEFINED-GLOBAL-TIME-SLAVE", "ABSTRACT-PROVIDED-PORT-PROTOTYPE", "SWITCH", "DEFAULT-ERROR-TRACER", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "SW-RECORD-LAYOUT", "ACTION-LIST", "STATIC-PART-TRIGGER", "ENCRYPTION", "VAR", "CYCLE-REPETITION-10", "REMOVE", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "UPDATE", "ERROR-TRACER-NEEDS", "COUPLING-PORT-FIFO", "ICMP", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "HY", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "PHM-SUPERVISION", "BSW-INTERNAL-BEHAVIOR", "OFF", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "FLEXRAY-NM-CLUSTER", "SERVICE-TIMING", "NO-KEEP", "DIAGNOSTIC-J-1939-NODE", "SYNCHRONOUS", "REDUNDANT-PER-KEY", "CSERS", "EXPLICIT", "BSW-DIRECT-CALL-POINT", "MULTICORE-REENTRANT", "PER-INSTANCE-MEMORY", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "NETWORK", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "SYSTEM-SUPPLIER-BOOT", "DLT-APPLICATION-TO-PROCESS-MAPPING", "USER-DEFINED-METHOD-DEPLOYMENT", "DEFERRED", "TD-EVENT-OPERATION", "TP-ADDRESS", "ARTIFACT-LOCATOR", "BSW-MODULE-ENTITY-TERMINATED", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "CLIENT-AUTHENTICATE", "MODE-DECLARATION-SWITCH-COMPLETED", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "SOMEIP-EVENT-GROUP", "DELETE", "LIN-MASTER", "RECOMMENDED-CONFIGURATION", "VERIFY", "TD-EVENT-COM", "SWC-TIMING", "SAE-J-1939--73", "FUNCTION-GROUP-SET", "ECU-STATE-MGR-USER-NEEDS", "REF-ALL", "SYNC-TIME-BASE-MGR-USER-NEEDS", "ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "DIAGNOSTIC-ROUTINE", "ADAPTIVE-FIELD-NOTIFICATION-SENT", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "TD-EVENT-TT-CAN-CYCLE-START", "PASSIVE", "WATCH-TRIGGER", "NO-PGWIDE", "GN", "R-4--2", "ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE", "INTRUSION-DETECTION-SECURITY-MANAGEMENT", "USER-DEFINED-COMMUNICATION-CONTROLLER", "CRYPTO-SERVICE-PRIMITIVE", "STATE-MANAGEMENT-REQUEST-INTERFACE", "DIAGNOSTIC-EVENT-PORT-MAPPING", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "I-SIGNAL", "CLIENT-ID-DEFINITION", "TIMING-CONDITION", "SW-MC-INTERFACE-SOURCE", "J-1939-CLUSTER", "EOC-EXECUTABLE-ENTITY-REF-ABSTRACT", "UZ", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "ABSTRACT-RAW-DATA-STREAM-INTERFACE", "GROSS", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "OFFSET-TIMING-CONSTRAINT", "REQUIRES-CALLBACK-EXECUTION", "TEST-FAILED-BIT", "MULTIPLE", "ETHERNET-RAW-DATA-STREAM-GRANT", "FIREWALL-STATE-SWITCH-INTERFACE", "DEVELOPMENT-ERROR-TRACER", "ECUC-ENUMERATION-LITERAL-DEF", "ENHANCED", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "TD-EVENT-FR-CLUSTER-CYCLE-START", "LIN-EVENT-TRIGGERED-FRAME", "EXCLUSIVE", "MODE-INTERFACE-MAPPING", "VFB-TIMING", "TEST-FAILED-THIS-OPERATION-CYCLE", "J-1939-NM-NODE", "ICV-VERIFIED", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT", "IGNORE", "BSW-MODULE-ENTRY", "FUNCTION-GROUP-STATE-TO-NM-HANDLE", "PRESHARED-KEY-IDENTITY", "DIAGNOSTIC-ECU-RESET-CLASS", "REST-NUMBER-PROPERTY-DEF", "UR", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "NO-AFFECT", "FM-FEATURE-SELECTION-SET", "VARIANT-POST-BUILD-LOADABLE", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "IP-SEC-CONFIG-PROPS", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "INTERRUPT-CAT-1", "STRICTLY-DECREASING", "FLAT-MAP", "AUTONOMOUS", "DIAGNOSTIC-DATA-ELEMENT", "ASSEMBLY-SW-CONNECTOR", "ECUC-FUNCTION-NAME-DEF", "SHOW-TYPE", "IMPLEMENTATION-PROPS", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "DIAGNOSTIC-INFO-TYPE", "DIAGNOSTIC-ROUTINE-NEEDS", "CLASSIC", "SOFTWARE-PACKAGE-STEP", "COM-EVENT-GRANT", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE", "DIAGNOSTIC-CONDITION", "IS-EQUAL", "MAINTENANCE-ONLY", "PORT-INTERFACE-MAPPING", "DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING", "FAST-FLASHING-MODE", "FY", "EXCLUSIVE-AREA", "ECUC-DEFINITION-ELEMENT", "MOST-SIGNIFICANT-BYTE-FIRST", "WILL-RECEIVE", "CAN-20", "SW-CONNECTOR", "DELEGATION-SW-CONNECTOR", "CONSOLE", "ADAPTIVE-SERVICE-FIND-COMPLETED", "TRANSFORMATION-PROPS", "SPEC-ELEMENT-REFERENCE", "EID-USE-API", "PARAMETER-SW-COMPONENT-TYPE", "LTS-13", "ACCESS-PERMISSION-SERVICE-INSTANCE", "LO", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "OBD-RATIO-DENOMINATOR-NEEDS", "TCP-OPTION-FILTER-SET", "FLEXRAY-TP-CONNECTION-CONTROL", "SCHEDULE-VARIANT-6", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "PHYSICAL-CHANNEL", "DIAGNOSTIC-FIM-EVENT-GROUP", "REACTION", "NO-OBD-SUPPORT", "TRIGGER-ACTIVATED", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "MAPPING-SCOPE-CORE", "PHYSICAL-ADDRESS", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING", "REST-ENDPOINT-DELETE", "REST-ENDPOINT-PUT", "CLIENT-VERIFY", "MEDIUM", "TLS-SECURE-COM-PROPS", "ECUC-ENUMERATION-PARAM-DEF", "ATP-FEATURE", "ECUC-FLOAT-PARAM-DEF", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "DATA-PROTOTYPE", "MR", "EXECUTION-TIME", "X-509", "INVALID", "MACHINE-TIMING", "NODE", "EXTERNAL-REPLACEMENT", "V-2-X-MANAGEMENT", "XH", "LINK-TIME", "MEASURED-HEAP-USAGE", "ADAPTIVE-EVENT-RECEIVED", "SD", "COMM-CONNECTOR-PORT", "PERIODIC-RATE-SLOW", "RUN-CONTINUOUS", "PRESENTATION-CONTINUOUS", "RED-STOP-LAMP", "API-BASED", "FULL", "RPT-CONTAINER", "KEY-STORAGE", "ANY-PARTIAL-NETWORK-ACTIVE", "ECUC-CHOICE-REFERENCE-DEF", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "PRESENTATION-DISCRETE", "TRANSFORMATION-PROPS-SET", "CLIENT-MAC-GENERATE", "REST-OBJECT-REF", "BSW-MODE-SWITCHED-ACK-EVENT", "DIAGNOSTIC-FREEZE-FRAME", "LIN-TP-NODE", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "SUPERVISED-ENTITY-NEEDS", "SIGN-WITH-ORIGIN-AUTHENTICATION", "STRUCTURED-REQ", "TEST-PASSED", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "SERVER-CALL-POINT", "ALL", "WONT-RECEIVE", "FAILURE-ONLY", "DIAGNOSTIC-INDICATOR", "STRICT-PRIORITY", "STATE-MANAGEMENT-REQUEST-TRIGGER", "ETHERNET-COMMUNICATION-CONNECTOR", "NA", "PERSISTENCY-FILE-ELEMENT", "SECURITY-EVENT-ONE-EVERY-N-FILTER", "CIRCLE", "NV-BLOCK-NEEDS", "SW-COMPONENT-TYPE", "TD-EVENT-SERVICE-INSTANCE-EVENT", "FLEXRAY-PHYSICAL-CHANNEL", "DIAGNOSTIC-SERVICE-INSTANCE", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "COM-FIELD-GRANT-DESIGN", "GA", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "ML", "AND", "AUTO-IP", "LOGICAL-EXPRESSION", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "SCHEDULE-VARIANT-3", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "SEARCH-FOR-ANY", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "AS-IS", "TG", "CURVE_AXIS", "PUBLISHED-INFORMATION", "FLEXRAY-COMMUNICATION-CONNECTOR", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "ALL-INDICES-SAME-ARRAY-SIZE", "VARIABLE-DATA-PROTOTYPE", "NOT-TESTED", "REQUIRED-DDS-SERVICE-INSTANCE", "SW-ADDR-METHOD", "PRIO-OCC", "PORT-INTERFACE", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "TIME-SYNC-MODULE-INSTANTIATION", "OVERWRITE", "FIRE-AND-FORGET-METHOD-MAPPING", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "ADAPTIVE-METHOD-RESPONSE-SENT", "RIGHT", "LINKER", "DIAGNOSTIC-VALUE-NEEDS", "SYMBOL-PROPS", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "NO-RETURN-VALUE-PROVIDED", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "MASTER-ECU", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "EVENT-HANDLER", "MG", "IN", "ECUC-VALIDATION-CONDITION", "SW-CLASS-ATTR-IMPL", "RM", "ADAPTIVE-METHOD-RESPONSE-RECEIVED", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "DATA-SEND-COMPLETED-EVENT", "CLIENT-SERVER-OPERATION", "HW-CATEGORY", "GRANT", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "NEW-IS-GREATER-OR-EQUAL", "STATE-MANAGEMENT-TRIGGER-INTERFACE", "CALPRM", "REPORTS-EXECUTION-STATE", "DIAGNOSTIC-EVENT-INTERFACE", "FRAME-QUEUED-FOR-TRANSMISSION", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "AR-PACKAGE", "OPERATION-CALL-RESPONSE-SENT", "VAR-NO-INIT", "DIAGNOSTIC-CONTROL-DTC-SETTING", "CRYPTO-CERTIFICATE-INTERFACE", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING", "IW", "UDP-CHECKSUM-ENABLED", "PDU-TO-FRAME-MAPPING", "DIAGNOSTIC-PORT-INTERFACE", "STATE-DEPENDENT-FIREWALL", "REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST", "CRYPTO-KEY-MANAGEMENT-NEEDS", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "SW-AXIS-TYPE", "HOST-PORT", "REST-ENDPOINT-POST", "PERSISTENCY-FILE-PROXY-INTERFACE", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "DIAGNOSTIC-AUTHENTICATION-CLASS", "ACTION-ITEM", "NOT-SENT", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "SUPERVISION-MODE-CONDITION", "TRACED-FAILURE", "DSA", "SYSTEM-SUPPLIER-BOOT-RESP-APP", "PRODUCER", "DLT-MESSAGE-COLLECTION-SET", "SW-INSTANCE", "DESELECTED", "DDS-METHOD-DEPLOYMENT", "KK", "EVENT-TRIGGERING-CONSTRAINT", "INOUT", "UP-LINK-PORT", "SYSTEM-SIGNAL-GROUP", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "CANNOT-BE-REMOVED", "HEAP-USAGE", "ROUGH-ESTIMATE-STACK-USAGE", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "RU", "WORST-CASE-HEAP-USAGE", "CLIENT-SERVER-INTERFACE", "DIAGNOSTIC-EVENT-INFO-NEEDS", "NOHREF", "DIAGNOSTIC-ENV-MODE-ELEMENT", "MASKED-NEW-DIFFERS-MASKED-OLD", "PRE--R-4--2", "KEY-SERVER", "REST-ABSTRACT-NUMERICAL-PROPERTY-DEF", "VIEW-MAP", "IDSM-TRAFFIC-LIMITATION", "DIAGNOSTIC-IUMPR", "PDU-R", "BULK-NV-DATA-DESCRIPTOR", "SWC-MODE-MANAGER-ERROR-EVENT", "PRIMARY-ECU", "BSW-MGR-NEEDS", "CAPTION", "SDG-FOREIGN-REFERENCE", "ROTATE-90-CCW-LIMIT-TO-TEXT", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "SOFTWARE-CLUSTER-DESIGN", "GENERIC-MODULE-INSTANTIATION", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED", "COM-MGR-USER-NEEDS", "REGULAR", "ADAPTIVE-SERVICE-OFFER-COMPLETED", "CONDITIONAL", "PERSISTENCY-FILE-STORAGE-INTERFACE", "STATUS-BIT-NORMAL", "FRAME-TRANSMITTED-ON-BUS", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "SERVICE-ONLY", "DO-IP-INTERFACE", "MAC-SEC-GLOBAL-KAY-PROPS", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "NO-CHECKPOINT-SUPERVISION", "CRYPTO-NEED", "ISO-6", "GENERAL-PARAMETER", "TIMING-EXTENSION", "PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "GLOBAL-TIME-CAN-MASTER", "REST-ABSTRACT-PROPERTY-DEF", "I-PV-6-EXT-HEADER-FILTER-SET", "SOMEIP-FIELD-DEPLOYMENT", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "RAW-DATA-STREAM-GRANT", "QUEUED", "PROCESSOR", "EL", "FORGET", "MIXED", "XXG-MII", "TL", "DIAGNOSTIC-COM-CONTROL", "BIDIRECTIONAL", "EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE", "SERVICE-SW-COMPONENT-TYPE", "CYCLE-REPETITION-2", "SAE-J-2012--DA", "CONFIDENTIALITY-OFFSET--0", "CRYPTO-KEY-SLOT-INTERFACE", "SCHEDULED", "LT-AFFECTS-PB", "HUB", "EXACT-OR-ANY-MINOR-VERSION", "LOGICAL-OR", "I-4-G", "PC-AFFECTS-LT", "UNIT-GROUP", "DIAGNOSTIC-SECURITY-LEVEL", "ECUC-BOOLEAN-PARAM-DEF", "ADAPTIVE-FIELD-GETTER-CALLED", "SECURITY-EVENT-CONTEXT-MAPPING", "READ-WRITE", "WONT-SEND", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "LAST-MODE", "FM-FEATURE-RELATION", "TRIGGER-INTERFACE", "SERVICE-INTERFACE-MAPPING-SET", "SW-MC-BASE-TYPE", "SW-MC-FRAME", "ABSTRACT-EXECUTION-CONTEXT", "REF-NONE", "LOGICAL-AND", "STOP-TRIGGER", "API-USE", "CPP-IMPLEMENTATION-DATA-TYPE", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "BSW-M-ENTRY-CALLED", "HEAD", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "ALTERNATING-8-BIT", "BINARY-MANIFEST-RESOURCE", "RETURN-ON-EVENT-STOPPED", "CRYPTO-SERVICE-MAPPING", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "BLINK-MODE", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "IDS-DESIGN", "NO-HEADER", "J-1939-REQUEST-MANAGER", "CRYPTO-INTERFACE", "STATE-MANAGEMENT-STATE-REQUEST", "DDS-PROVIDED-SERVICE-INSTANCE", "LEGACY", "PROCESS-TO-MACHINE-MAPPING", "EPS", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "X-MMI", "DROP", "ARBITRATION", "GL", "EO", "BINARY-MANIFEST-ITEM", "PROCESS-IS-SELF-TERMINATING", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "NON-VOLATILE-RAM-MANAGER", "UNDECIDED", "ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING", "KL", "REF-NON-STANDARD", "DLNA", "ACTIVATION-UNICAST", "SEC-OC-JOB-MAPPING", "MO", "CLIENT-ID-DEFINITION-SET", "DIAGNOSTIC-PROTOCOL", "RX-TRIGGER", "APPLICATION", "MASKED-NEW-EQUALS-MASKED-OLD", "AP", "PROCESS-IS-NOT-SELF-TERMINATING", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "REBOOT", "AP-SOMEIP-TRANSFORMATION-PROPS", "SWC-MODE-SWITCH-EVENT", "PUT", "PDU-TRIGGERING", "SYMMETRIC-KEY", "CLIENT-SERVER-INTERFACE-MAPPING", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "RPT-LEVEL-2", "DEVELOPMENT", "SECRET-SEED", "MONITOR-MODE", "SERVICE-INTERFACE", "REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE", "OPERATION-CALL-RESPONSE-RECEIVED", "NO-STORE-EVENT", "COM-OFFER-SERVICE-GRANT", "KA", "DATA-RECEIVE-ERROR-EVENT", "DIAGNOSTIC-CAPABILITY-ELEMENT", "PRE-COMPILE-TIME", "CALIBRATION-OFFLINE", "MASKED-NEW-DIFFERS-X", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "NO-SHOW-PAGE", "MAC-SEC-PARTICIPANT-SET", "FLOAT", "SECURED-I-PDU", "CLIENT-ENCRYPT", "CAN-TP-CHANNEL", "RESTART", "SW-CODE-SYNTAX", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "FORWARD-AS-IS", "AR--CLIENT--SERVER", "NO-SHOW-TYPE", "SOCKET-CONNECTION-BUNDLE", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "COLDSTART", "DIAGNOSTIC-IO-CONTROL-CLASS", "DO-IP-LOGIC-ADDRESS", "SIGNAL-BASED-FIELD-DEPLOYMENT", "ATP-PROTOTYPE", "DIAG-REQUEST", "GENERIC-ETHERNET-FRAME", "UDS", "DIAGNOSTIC-SERVICE-TABLE", "COMPILE", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "CONFIG-DATA", "TIMING-CONSTRAINT", "PASS-THROUGH-SW-CONNECTOR", "MK", "LOG-AND-TRACE-INSTANTIATION", "NON-EMMISSION-RELATED-DTC", "IDS-MGR-NEEDS", "STARTUP-CONFIG-SET", "STRICT-MODE", "NO-BOOT", "IMMEDIATE", "ACTIVATION-AND-TRIGGER-UNICAST", "DO-IP", "HW-TYPE", "FI", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "FIXED", "BSW-DATA-RECEIVED-EVENT", "STOP", "MODE-SWITCHED-ACK-EVENT", "ICV-OPTIONAL", "MULTIPLEXED-I-PDU", "DATA-RECEIVED-EVENT", "UCM-TO-TIME-BASE-RESOURCE-MAPPING", "INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE", "SCHEDULE-VARIANT-4", "SERVER-DECRYPT", "REST-ENDPOINT-GET", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "MASEKD-NEW-EQUALS-X", "ECUC-LINKER-SYMBOL-DEF", "STORE-EVENT", "ROLL-BACK", "UNIT", "CONSISTENCY-NEEDS", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "SYNCHRONIZED-TIME-BASE-PROVIDER", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "NETWORK-REPRESENTATION-FROM-COM-SPEC", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "ALL-INDICES-DIFFERENT-ARRAY-SIZE", "ABSTRACT-CAN-COMMUNICATION-CONNECTOR", "NO-SEVERITY", "BSW", "DO-IP-INSTANTIATION", "RESPONSE-SYNCHRONIZATION", "ISO", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "ECUC-REFERENCE-DEF", "DOCUMENT-ELEMENT-SCOPE", "BG", "SECURE-COMMUNICATION-DEPLOYMENT", "BSW-SCHEDULABLE-ENTITY", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "PC-AFFECTS-LT-AND-PB", "CRYPTO-ELLIPTIC-CURVE-PROPS", "IS-NOT-RELEVANT", "KEEP", "SERVICE-METHOD-DEPLOYMENT", "AA", "REQUEST-CALLBACK-TYPE-MANUFACTURER", "SW-SYSTEMCONSTANT-VALUE-SET", "SYNCHRONIZED-MASTER-TIME-BASE", "IEEE-1722-TP-ETHERNET-FRAME", "SDG-AGGREGATION-WITH-VARIATION", "SW-COMPONENT-PROTOTYPE", "PDF", "STRICT-MONOTONOUS", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "OPERATING-SYSTEM", "NON-REENTRANT", "DROP-UNTAGGED", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "CYCLE-REPETITION-32", "ATP-DEFINITION", "SYNCHRONIZED-TIME-BASE-CONSUMER", "SYMBOLIC-NAME-PROPS", "SCHEDULE-VARIANT-1", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "1000BASE-T1", "POWER-WINDOW-TIME", "TOP", "FLEXRAY-TP-CONFIG", "DIAGNOSTIC-SESSION", "TRACE", "EXECUTION-TIME-CONSTRAINT", "SYSTEM-MAPPING", "ROTATE-90-CW", "NO-DEFAULT", "BUILD-ACTION-ENVIRONMENT", "R-PORT-PROTOTYPE", "REDUNDANT", "BN", "ADAPTIVE-FIELD-NOTIFICATION-RECEIVED", "FILE", "BSW-VARIABLE-ACCESS", "DERIVED-FROM", "SIGNAL-SERVICE-TRANSLATION-PROPS", "USES-LOGGING", "CONSTRAINT-TAILORING", "RTE-EVENT", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "IS-STOPPED", "SERVICE-INTERFACE-TRIGGER-MAPPING", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "CODEGENERATION", "PHM-RECOVERY-ACTION-INTERFACE", "PR-PORT-PROTOTYPE", "SIDES"];

    /// derive an enum entry from an input string using a perfect hash function
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseEnumItemError> {
        static DISPLACEMENTS: [(u16, u16); 357] = [(0, 842), (0, 11), (0, 0), (0, 62), (0, 0), (0, 1529), (0, 24), (0, 193), (0, 2), (0, 527), (0, 75), (0, 2080), (0, 20), (0, 85), (0, 1985), (0, 1), (10, 733), (0, 129), (4, 391), (0, 1506), (0, 73), (0, 141), (15, 1423), (0, 213), (7, 2125), (0, 1154), (0, 34), (0, 83), (8, 1338), (8, 738), (19, 2004), (0, 250), (0, 171), (2, 1854), (0, 2), (1, 1461), (0, 8), (0, 2), (0, 53), (33, 381), (0, 287), (2, 1252), (0, 234), (0, 1734), (0, 127), (2, 2327), (0, 0), (3, 837), (0, 0), (0, 1323), (0, 656), (0, 1), (0, 43), (23, 2001), (2, 1843), (8, 2435), (2, 1654), (0, 1252), (0, 2062), (0, 40), (5, 597), (0, 1956), (18, 817), (0, 57), (0, 15), (0, 23), (0, 1), (0, 43), (5, 2184), (1, 578), (14, 1059), (0, 885), (17, 1464), (0, 87), (0, 3), (0, 40), (0, 21), (0, 28), (0, 8), (56, 1520), (0, 1693), (0, 97), (0, 69), (0, 116), (0, 102), (0, 212), (0, 3), (0, 152), (0, 1), (0, 108), (0, 1012), (0, 144), (0, 6), (1, 238), (0, 71), (0, 570), (0, 82), (0, 0), (0, 9), (0, 3), (0, 109), (0, 14), (0, 116), (0, 32), (0, 5), (6, 401), (0, 378), (8, 702), (0, 415), (0, 30), (0, 536), (0, 183), (0, 3), (0, 2), (0, 94), (2, 2158), (0, 30), (1, 119), (0, 343), (0, 630), (0, 11), (0, 8), (0, 148), (1, 785), (9, 2488), (67, 140), (0, 644), (0, 6), (0, 167), (2, 1688), (0, 2147), (14, 1412), (5, 726), (0, 80), (0, 0), (34, 1997), (0, 8), (0, 203), (0, 1019), (0, 312), (0, 1534), (0, 570), (0, 1005), (0, 8), (0, 1), (28, 395), (0, 1865), (2, 2383), (0, 29), (5, 457), (0, 77), (35, 2200), (0, 64), (0, 554), (3, 552), (1, 1719), (17, 1677), (0, 5), (12, 1371), (0, 33), (5, 781), (0, 1128), (0, 11), (0, 1222), (0, 250), (0, 54), (0, 2), (0, 0), (0, 2), (0, 18), (0, 0), (3, 464), (4, 562), (5, 1470), (0, 57), (0, 250), (0, 529), (0, 2), (40, 768), (0, 161), (0, 84), (16, 2445), (0, 146), (0, 5), (0, 13), (0, 287), (11, 1671), (0, 774), (0, 5), (0, 0), (2, 1676), (0, 28), (37, 1664), (0, 9), (1, 1654), (0, 2414), (10, 2488), (1, 494), (0, 194), (0, 40), (7, 496), (21, 2432), (1, 223), (9, 210), (43, 92), (0, 163), (0, 2447), (14, 1771), (1, 0), (1, 13), (90, 2125), (0, 2), (3, 1574), (0, 1299), (0, 0), (0, 4), (2, 992), (0, 1486), (0, 300), (75, 1839), (46, 1835), (4, 1004), (19, 821), (59, 719), (0, 2166), (0, 1046), (361, 1492), (24, 1801), (0, 22), (0, 29), (3, 2365), (1, 1), (2, 960), (0, 119), (0, 1743), (0, 212), (0, 11), (34, 1332), (198, 576), (0, 47), (0, 18), (0, 316), (8, 811), (0, 12), (9, 1236), (0, 1), (3, 1732), (0, 198), (110, 1261), (0, 303), (0, 1), (21, 2215), (3, 881), (4, 2042), (69, 981), (0, 24), (0, 1), (0, 1535), (4, 538), (166, 963), (0, 1), (0, 364), (30, 2038), (0, 0), (2, 1091), (8, 255), (0, 41), (0, 20), (0, 102), (0, 4), (10, 428), (278, 2036), (0, 2), (0, 1377), (0, 32), (0, 339), (49, 397), (0, 48), (2, 1974), (0, 3), (0, 22), (7, 899), (2, 1272), (67, 1687), (0, 62), (0, 490), (3, 1985), (2, 637), (13, 1009), (0, 107), (21, 1306), (1, 151), (12, 1179), (14, 2351), (1, 304), (6, 1548), (2, 1141), (9, 2316), (0, 914), (2, 1564), (83, 720), (0, 6), (1, 527), (0, 1161), (0, 177), (148, 1076), (1, 611), (1, 3), (16, 111), (1, 640), (358, 191), (0, 436), (0, 23), (5, 1154), (0, 1057), (0, 12), (0, 178), (0, 51), (7, 74), (0, 42), (0, 4), (6, 2022), (0, 71), (371, 1013), (0, 1), (19, 942), (3, 1631), (0, 670), (4, 1651), (17, 1453), (0, 226), (1, 1431), (0, 2), (0, 22), (0, 2302), (0, 0), (110, 1197), (1, 258), (0, 110), (0, 12), (0, 1), (0, 30), (0, 0), (1171, 1279), (345, 1578), (54, 115), (0, 246), (0, 403), (24, 1187), (0, 1951), (0, 53), (0, 2244), (142, 498), (0, 52), (945, 2148), (11, 2336), (9, 75)];
        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 357) as usize];
        let item_idx = (d2 as u32).wrapping_add(f1.wrapping_mul(d1 as u32)).wrapping_add(f2) as usize % 2494;
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

