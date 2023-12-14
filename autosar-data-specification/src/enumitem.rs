use crate::hashfunc;

#[derive(Debug)]
/// The error type `ParseEnumItemError` is returned when `from_str()` / `parse()` fails for `EnumItem`
pub struct ParseEnumItemError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
#[non_exhaustive]
/// Enum of all possible enum values in Autosar
pub enum EnumItem {
    /// -500-MILES
    _500Miles                                                              = 1146,
    /// 1
    _1                                                                     = 67,
    /// 1-0
    _1_0                                                                   = 2004,
    /// 1-001
    _1_001                                                                 = 1715,
    /// 1-1-001
    _1_1_001                                                               = 1461,
    /// 1-8
    _1_8                                                                   = 1651,
    /// 10
    _10                                                                    = 1450,
    /// 100
    _100                                                                   = 191,
    /// 1000BASE-T
    _1000baseT                                                             = 2200,
    /// 1000BASE-T1
    _1000baseT1                                                            = 1745,
    /// 100BASE-T1
    _100baseT1                                                             = 370,
    /// 100BASE-TX
    _100baseTx                                                             = 898,
    /// 10BASE-T1S
    _10baseT1s                                                             = 998,
    /// 12
    _12                                                                    = 1652,
    /// 120
    _120                                                                   = 1056,
    /// 15
    _15                                                                    = 1557,
    /// 150
    _150                                                                   = 2247,
    /// 16
    _16                                                                    = 1094,
    /// 16-KHZ
    _16Khz                                                                 = 1724,
    /// 176-4-KHZ
    _176_4Khz                                                              = 1395,
    /// 192-KHZ
    _192Khz                                                                = 1195,
    /// 2
    _2                                                                     = 1572,
    /// 20
    _20                                                                    = 2188,
    /// 200
    _200                                                                   = 1806,
    /// 24
    _24                                                                    = 341,
    /// 24-25
    _24_25                                                                 = 2086,
    /// 24-KHZ
    _24Khz                                                                 = 1167,
    /// 240
    _240                                                                   = 2641,
    /// 25
    _25                                                                    = 1718,
    /// 25-24
    _25_24                                                                 = 2355,
    /// 30
    _30                                                                    = 1561,
    /// 300
    _300                                                                   = 1609,
    /// 32-KHZ
    _32Khz                                                                 = 1407,
    /// 4-1-1
    _4_1_1                                                                 = 299,
    /// 4-2-0
    _4_2_0                                                                 = 2403,
    /// 4-2-2
    _4_2_2                                                                 = 235,
    /// 4-2-2-4
    _4_2_2_4                                                               = 499,
    /// 4-4-4
    _4_4_4                                                                 = 236,
    /// 4-4-4-4
    _4_4_4_4                                                               = 785,
    /// 44-1-KHZ
    _44_1Khz                                                               = 1101,
    /// 48
    _48                                                                    = 1734,
    /// 48-KHZ
    _48Khz                                                                 = 2531,
    /// 5
    _5                                                                     = 1636,
    /// 50
    _50                                                                    = 317,
    /// 60
    _60                                                                    = 237,
    /// 72
    _72                                                                    = 1003,
    /// 8
    _8                                                                     = 474,
    /// 8-KHZ
    _8Khz                                                                  = 1445,
    /// 85
    _85                                                                    = 971,
    /// 88-2-KHZ
    _88_2Khz                                                               = 1617,
    /// 96-KHZ
    _96Khz                                                                 = 989,
    /// AA
    Aa                                                                     = 553,
    /// AB
    Ab                                                                     = 2045,
    /// ABSTRACT
    Abstract                                                               = 1480,
    /// ABSTRACT-ACCESS-POINT
    AbstractAccessPoint                                                    = 780,
    /// ABSTRACT-CAN-CLUSTER
    AbstractCanCluster                                                     = 486,
    /// ABSTRACT-CAN-COMMUNICATION-CONNECTOR
    AbstractCanCommunicationConnector                                      = 2637,
    /// ABSTRACT-CAN-COMMUNICATION-CONTROLLER
    AbstractCanCommunicationController                                     = 10,
    /// ABSTRACT-CAN-PHYSICAL-CHANNEL
    AbstractCanPhysicalChannel                                             = 1225,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 789,
    /// ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS
    AbstractDoIpLogicAddressProps                                          = 1695,
    /// ABSTRACT-ETHERNET-FRAME
    AbstractEthernetFrame                                                  = 2292,
    /// ABSTRACT-EVENT
    AbstractEvent                                                          = 1159,
    /// ABSTRACT-EXECUTION-CONTEXT
    AbstractExecutionContext                                               = 2669,
    /// ABSTRACT-IAM-REMOTE-SUBJECT
    AbstractIamRemoteSubject                                               = 1684,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE
    AbstractImplementationDataType                                         = 1116,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT
    AbstractImplementationDataTypeElement                                  = 1121,
    /// ABSTRACT-PROVIDED-PORT-PROTOTYPE
    AbstractProvidedPortPrototype                                          = 234,
    /// ABSTRACT-RAW-DATA-STREAM-INTERFACE
    AbstractRawDataStreamInterface                                         = 419,
    /// ABSTRACT-REQUIRED-PORT-PROTOTYPE
    AbstractRequiredPortPrototype                                          = 829,
    /// ABSTRACT-SECURITY-EVENT-FILTER
    AbstractSecurityEventFilter                                            = 184,
    /// ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER
    AbstractSecurityIdsmInstanceFilter                                     = 1780,
    /// ABSTRACT-SERVICE-INSTANCE
    AbstractServiceInstance                                                = 1397,
    /// ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING
    AbstractSignalBasedToISignalTriggeringMapping                          = 1144,
    /// ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE
    AbstractSynchronizedTimeBaseInterface                                  = 2619,
    /// ACCEPT-ALL
    AcceptAll                                                              = 418,
    /// ACCEPT-CONFIGURED
    AcceptConfigured                                                       = 385,
    /// ACCES-PERRMISSION-SERVICE-CLASS
    AccesPerrmissionServiceClass                                           = 1804,
    /// ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS
    AccessPermissionInstanceOverridesClass                                 = 841,
    /// ACCESS-PERMISSION-SERVICE-CLASS
    AccessPermissionServiceClass                                           = 1799,
    /// ACCESS-PERMISSION-SERVICE-INSTANCE
    AccessPermissionServiceInstance                                        = 245,
    /// ACK-WITH-RT
    AckWithRt                                                              = 2075,
    /// ACK-WITHOUT-RT
    AckWithoutRt                                                           = 2311,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 228,
    /// ACL-OPERATION
    AclOperation                                                           = 186,
    /// ACL-PERMISSION
    AclPermission                                                          = 414,
    /// ACL-ROLE
    AclRole                                                                = 50,
    /// ACTION
    Action                                                                 = 1222,
    /// ACTION-ITEM
    ActionItem                                                             = 2479,
    /// ACTION-LIST
    ActionList                                                             = 1035,
    /// ACTIVATE
    Activate                                                               = 422,
    /// ACTIVATION-AND-TRIGGER-UNICAST
    ActivationAndTriggerUnicast                                            = 864,
    /// ACTIVATION-MULTICAST
    ActivationMulticast                                                    = 2574,
    /// ACTIVATION-UNICAST
    ActivationUnicast                                                      = 1405,
    /// ACTIVE
    Active                                                                 = 1145,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 1649,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 762,
    /// ADAPTIVE-EVENT-RECEIVED
    AdaptiveEventReceived                                                  = 2461,
    /// ADAPTIVE-EVENT-SENT
    AdaptiveEventSent                                                      = 632,
    /// ADAPTIVE-FIELD-GETTER-CALLED
    AdaptiveFieldGetterCalled                                              = 1541,
    /// ADAPTIVE-FIELD-GETTER-COMPLETED
    AdaptiveFieldGetterCompleted                                           = 22,
    /// ADAPTIVE-FIELD-NOTIFICATION-RECEIVED
    AdaptiveFieldNotificationReceived                                      = 2607,
    /// ADAPTIVE-FIELD-NOTIFICATION-SENT
    AdaptiveFieldNotificationSent                                          = 990,
    /// ADAPTIVE-FIELD-SETTER-CALLED
    AdaptiveFieldSetterCalled                                              = 2341,
    /// ADAPTIVE-FIELD-SETTER-COMPLETED
    AdaptiveFieldSetterCompleted                                           = 373,
    /// ADAPTIVE-FIREWALL-MODULE-INSTANTIATION
    AdaptiveFirewallModuleInstantiation                                    = 159,
    /// ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING
    AdaptiveFirewallToPortPrototypeMapping                                 = 1770,
    /// ADAPTIVE-METHOD-CALL-RECEIVED
    AdaptiveMethodCallReceived                                             = 1518,
    /// ADAPTIVE-METHOD-CALLED
    AdaptiveMethodCalled                                                   = 948,
    /// ADAPTIVE-METHOD-RESPONSE-RECEIVED
    AdaptiveMethodResponseReceived                                         = 1390,
    /// ADAPTIVE-METHOD-RESPONSE-SENT
    AdaptiveMethodResponseSent                                             = 1292,
    /// ADAPTIVE-MODULE-INSTANTIATION
    AdaptiveModuleInstantiation                                            = 337,
    /// ADAPTIVE-PLATFORM-SERVICE-INSTANCE
    AdaptivePlatformServiceInstance                                        = 1505,
    /// ADAPTIVE-SERVICE-FIND-COMPLETED
    AdaptiveServiceFindCompleted                                           = 310,
    /// ADAPTIVE-SERVICE-FIND-STARTED
    AdaptiveServiceFindStarted                                             = 1766,
    /// ADAPTIVE-SERVICE-OFFER-COMPLETED
    AdaptiveServiceOfferCompleted                                          = 1826,
    /// ADAPTIVE-SERVICE-OFFER-STARTED
    AdaptiveServiceOfferStarted                                            = 2602,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED
    AdaptiveServiceStopSubscriptionCompleted                               = 2510,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED
    AdaptiveServiceStopSubscriptionStarted                                 = 188,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED
    AdaptiveServiceSubscriptionAcknowledgeCompleted                        = 2683,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED
    AdaptiveServiceSubscriptionAcknowledgeStarted                          = 1304,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED
    AdaptiveServiceSubscriptionCompleted                                   = 1309,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED
    AdaptiveServiceSubscriptionStarted                                     = 2209,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 1314,
    /// ADDR-METHOD-SHORT-NAME
    AddrMethodShortName                                                    = 1177,
    /// ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT
    AddrMethodShortNameAndAlignment                                        = 2280,
    /// AES-3-32-BIT
    Aes3_32Bit                                                             = 2174,
    /// AF
    Af                                                                     = 2423,
    /// AFTER-SALES
    AfterSales                                                             = 1895,
    /// AFTERMAKET
    Aftermaket                                                             = 1076,
    /// AFTERMARKET
    Aftermarket                                                            = 1027,
    /// AGE
    Age                                                                    = 1823,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 940,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 2326,
    /// AGREED
    Agreed                                                                 = 2565,
    /// AH
    Ah                                                                     = 2483,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 2368,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 1478,
    /// ALL
    All                                                                    = 1248,
    /// ALL-16-BIT
    All16Bit                                                               = 1321,
    /// ALL-INDICES-DIFFERENT-ARRAY-SIZE
    AllIndicesDifferentArraySize                                           = 361,
    /// ALL-INDICES-SAME-ARRAY-SIZE
    AllIndicesSameArraySize                                                = 1441,
    /// ALL-PARTIAL-NETWORKS-ACTIVE
    AllPartialNetworksActive                                               = 2384,
    /// ALL-SUPPORTED-DTCS
    AllSupportedDtcs                                                       = 614,
    /// ALLOCATOR
    Allocator                                                              = 2161,
    /// ALLOW
    Allow                                                                  = 1587,
    /// ALTERNATING-8-BIT
    Alternating8Bit                                                        = 929,
    /// ALWAYS
    Always                                                                 = 1756,
    /// AM
    Am                                                                     = 1489,
    /// AMBER-WARNING
    AmberWarning                                                           = 413,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 2294,
    /// AND
    And                                                                    = 2589,
    /// ANY
    Any                                                                    = 883,
    /// ANY-PARTIAL-NETWORK-ACTIVE
    AnyPartialNetworkActive                                                = 39,
    /// ANY-SEND-OPERATION
    AnySendOperation                                                       = 164,
    /// ANY-STANDARDIZED
    AnyStandardized                                                        = 1935,
    /// AP
    Ap                                                                     = 392,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 1862,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 501,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 699,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 2044,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 2120,
    /// API
    Api                                                                    = 1124,
    /// API-BASED
    ApiBased                                                               = 423,
    /// API-USE
    ApiUse                                                                 = 1949,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 1044,
    /// APPLICABILITY-INFO-SET
    ApplicabilityInfoSet                                                   = 2137,
    /// APPLICATION
    Application                                                            = 1268,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 206,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 1965,
    /// APPLICATION-ARRAY-ELEMENT
    ApplicationArrayElement                                                = 1589,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 2160,
    /// APPLICATION-ASSOC-MAP-ELEMENT
    ApplicationAssocMapElement                                             = 968,
    /// APPLICATION-COMPOSITE-DATA-TYPE
    ApplicationCompositeDataType                                           = 675,
    /// APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE
    ApplicationCompositeElementDataPrototype                               = 1597,
    /// APPLICATION-DATA-TYPE
    ApplicationDataType                                                    = 1784,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 331,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 308,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 165,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 2372,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 2513,
    /// APPLICATION-ONLY
    ApplicationOnly                                                        = 1114,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 1825,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 1214,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 2524,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 398,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 738,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 2561,
    /// APPLIED-STANDARD
    AppliedStandard                                                        = 353,
    /// AR
    Ar                                                                     = 220,
    /// AR--CLIENT--SERVER
    ArClientServer                                                         = 1995,
    /// AR-ELEMENT
    ArElement                                                              = 1185,
    /// AR-PACKAGE
    ArPackage                                                              = 2154,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 2317,
    /// ARBITRATION
    Arbitration                                                            = 1471,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 1132,
    /// ARRAY
    Array                                                                  = 1709,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 941,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 70,
    /// ARTIFACT-LOCATOR
    ArtifactLocator                                                        = 857,
    /// AS
    As                                                                     = 535,
    /// AS-IS
    AsIs                                                                   = 1403,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 2415,
    /// ASYMMETRIC-FROM-BYTE-ARRAY
    AsymmetricFromByteArray                                                = 100,
    /// ASYMMETRIC-TO-BYTE-ARRAY
    AsymmetricToByteArray                                                  = 182,
    /// ASYNCHRONOUS
    Asynchronous                                                           = 146,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 110,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 2533,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 258,
    /// ATOMIC-SW-COMPONENT-TYPE
    AtomicSwComponentType                                                  = 2052,
    /// ATP-BLUEPRINT
    AtpBlueprint                                                           = 1001,
    /// ATP-BLUEPRINTABLE
    AtpBlueprintable                                                       = 2691,
    /// ATP-CLASSIFIER
    AtpClassifier                                                          = 999,
    /// ATP-DEFINITION
    AtpDefinition                                                          = 897,
    /// ATP-FEATURE
    AtpFeature                                                             = 104,
    /// ATP-PROTOTYPE
    AtpPrototype                                                           = 1420,
    /// ATP-STRUCTURE-ELEMENT
    AtpStructureElement                                                    = 650,
    /// ATP-TYPE
    AtpType                                                                = 1186,
    /// ATTRIBUTE-TAILORING
    AttributeTailoring                                                     = 1896,
    /// AUDIO-SAMPLE
    AudioSample                                                            = 1008,
    /// AUTHENTICATE
    Authenticate                                                           = 1479,
    /// AUTO
    Auto                                                                   = 377,
    /// AUTO-IP
    AutoIp                                                                 = 2437,
    /// AUTO-IP--DOIP
    AutoIpDoip                                                             = 134,
    /// AUTO-IPDHCPV-4
    AutoIpdhcpv4                                                           = 2349,
    /// AUTOMATIC
    Automatic                                                              = 1565,
    /// AUTONOMOUS
    Autonomous                                                             = 1978,
    /// AUTOSAR-DATA-PROTOTYPE
    AutosarDataPrototype                                                   = 464,
    /// AUTOSAR-DATA-TYPE
    AutosarDataType                                                        = 1714,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 1464,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 1613,
    /// AVB--IEEE-802--1-AS
    AvbIeee802_1As                                                         = 1723,
    /// AY
    Ay                                                                     = 1066,
    /// AZ
    Az                                                                     = 2632,
    /// BA
    Ba                                                                     = 1525,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 2140,
    /// BASE-T
    BaseT                                                                  = 2158,
    /// BASE-TYPE
    BaseType                                                               = 676,
    /// BASIC-SOFTWARE-MODE-MANAGER
    BasicSoftwareModeManager                                               = 997,
    /// BAYER-BGGR
    BayerBggr                                                              = 509,
    /// BAYER-GBRG
    BayerGbrg                                                              = 380,
    /// BAYER-GRBG
    BayerGrbg                                                              = 560,
    /// BAYER-RGGB
    BayerRggb                                                              = 605,
    /// BE
    Be                                                                     = 224,
    /// BEST-EFFORT
    BestEffort                                                             = 582,
    /// BG
    Bg                                                                     = 2339,
    /// BH
    Bh                                                                     = 810,
    /// BI
    Bi                                                                     = 2241,
    /// BIDIRECTIONAL
    Bidirectional                                                          = 376,
    /// BINARY-MANIFEST-ADDRESSABLE-OBJECT
    BinaryManifestAddressableObject                                        = 2287,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 659,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 1278,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 1633,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 960,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 906,
    /// BINARY-MANIFEST-RESOURCE
    BinaryManifestResource                                                 = 551,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 32,
    /// BLINK-MODE
    BlinkMode                                                              = 1086,
    /// BLINK-OR-CONTINUOUS-ON-MODE
    BlinkOrContinuousOnMode                                                = 318,
    /// BLOCK
    Block                                                                  = 137,
    /// BLOCK-SOURCE
    BlockSource                                                            = 2070,
    /// BLOCK-STATE
    BlockState                                                             = 469,
    /// BLUEPRINT-DERIVATION-TIME
    BlueprintDerivationTime                                                = 1152,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 2268,
    /// BMP
    Bmp                                                                    = 2239,
    /// BN
    Bn                                                                     = 1675,
    /// BO
    Bo                                                                     = 111,
    /// BOLD
    Bold                                                                   = 431,
    /// BOLDITALIC
    Bolditalic                                                             = 304,
    /// BONJOUR
    Bonjour                                                                = 1637,
    /// BOTTOM
    Bottom                                                                 = 1685,
    /// BR
    Br                                                                     = 1972,
    /// BREAK
    Break                                                                  = 180,
    /// BRIEF
    Brief                                                                  = 1317,
    /// BRIEF-BYPASSING-FILTERS
    BriefBypassingFilters                                                  = 1200,
    /// BROAD-R-REACH
    BroadRReach                                                            = 1795,
    /// BSW
    Bsw                                                                    = 1779,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 1339,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 2376,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 2505,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 2337,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 1902,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 783,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 2474,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 176,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 291,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 2277,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 2639,
    /// BSW-EVENT
    BswEvent                                                               = 2313,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 2312,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 1522,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 201,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 903,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 2580,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 2258,
    /// BSW-INTERRUPT-EVENT
    BswInterruptEvent                                                      = 488,
    /// BSW-M-ENTRY-CALL-RETURNED
    BswMEntryCallReturned                                                  = 1306,
    /// BSW-M-ENTRY-CALLED
    BswMEntryCalled                                                        = 820,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 1742,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 1746,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 1351,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 2547,
    /// BSW-MODULE-CALL-POINT
    BswModuleCallPoint                                                     = 1374,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 932,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 812,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 1903,
    /// BSW-MODULE-ENTITY
    BswModuleEntity                                                        = 19,
    /// BSW-MODULE-ENTITY-ACTIVATED
    BswModuleEntityActivated                                               = 1787,
    /// BSW-MODULE-ENTITY-STARTED
    BswModuleEntityStarted                                                 = 1648,
    /// BSW-MODULE-ENTITY-TERMINATED
    BswModuleEntityTerminated                                              = 2615,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 905,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 437,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 868,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 2057,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 1281,
    /// BSW-SCHEDULE-EVENT
    BswScheduleEvent                                                       = 2660,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 1174,
    /// BSW-SERVICE-DEPENDENCY-IDENT
    BswServiceDependencyIdent                                              = 2187,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 2370,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 152,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 1719,
    /// BT-REC-601
    BtRec601                                                               = 1357,
    /// BT-REC-709
    BtRec709                                                               = 287,
    /// BUILD
    Build                                                                  = 2347,
    /// BUILD-ACTION
    BuildAction                                                            = 462,
    /// BUILD-ACTION-ENTITY
    BuildActionEntity                                                      = 2069,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 1640,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 74,
    /// BUILD-TYPE-DEBUG
    BuildTypeDebug                                                         = 2171,
    /// BUILD-TYPE-RELEASE
    BuildTypeRelease                                                       = 2021,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 1594,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 1622,
    /// BUS-MIRROR-CHANNEL-MAPPING
    BusMirrorChannelMapping                                                = 346,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 1022,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 2228,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 2102,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 2556,
    /// BY-RECEPTION-TIMESTAMP
    ByReceptionTimestamp                                                   = 449,
    /// BY-SOURCE-TIMESTAMP
    BySourceTimestamp                                                      = 1588,
    /// C
    C                                                                      = 1491,
    /// CA
    Ca                                                                     = 485,
    /// CALCULATED
    Calculated                                                             = 1290,
    /// CALIBRATION-OFFLINE
    CalibrationOffline                                                     = 2482,
    /// CALIBRATION-ONLINE
    CalibrationOnline                                                      = 1275,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 2180,
    /// CALIBRATION-VARIABLES
    CalibrationVariables                                                   = 1327,
    /// CALLBACK
    Callback                                                               = 1757,
    /// CALLOUT
    Callout                                                                = 1549,
    /// CALPRM
    Calprm                                                                 = 1232,
    /// CAN
    Can                                                                    = 15,
    /// CAN-20
    Can20                                                                  = 264,
    /// CAN-BE-REMOVED
    CanBeRemoved                                                           = 1511,
    /// CAN-BE-TERMINATED
    CanBeTerminated                                                        = 576,
    /// CAN-BE-TERMINATED-AND-RESTARTED
    CanBeTerminatedAndRestarted                                            = 944,
    /// CAN-BRIEF
    CanBrief                                                               = 1915,
    /// CAN-CLUSTER
    CanCluster                                                             = 37,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 2076,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 208,
    /// CAN-FD
    CanFd                                                                  = 2590,
    /// CAN-FRAME
    CanFrame                                                               = 953,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 1702,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 2592,
    /// CAN-NM-NODE
    CanNmNode                                                              = 2466,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 92,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 2621,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 85,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 537,
    /// CAN-TP-NODE
    CanTpNode                                                              = 656,
    /// CAN-XL-PROPS
    CanXlProps                                                             = 823,
    /// CANCEL
    Cancel                                                                 = 729,
    /// CANCEL-CAMPAIGN
    CancelCampaign                                                         = 157,
    /// CANNOT-BE-REMOVED
    CannotBeRemoved                                                        = 390,
    /// CAPTION
    Caption                                                                = 447,
    /// CAPTURE-ASYNCHRONOUS-TO-REPORTING
    CaptureAsynchronousToReporting                                         = 1484,
    /// CAPTURE-ASYNCHRONOUSLY-TO-REPORTING
    CaptureAsynchronouslyToReporting                                       = 1534,
    /// CAPTURE-SYNCHRONOUS-TO-REPORTING
    CaptureSynchronousToReporting                                          = 2418,
    /// CAPTURE-SYNCHRONOUSLY-TO-REPORTING
    CaptureSynchronouslyToReporting                                        = 192,
    /// CAT-1
    Cat1                                                                   = 1324,
    /// CAT-2
    Cat2                                                                   = 1462,
    /// CAUTION
    Caution                                                                = 316,
    /// CENTER
    Center                                                                 = 2191,
    /// CHANNEL-A
    ChannelA                                                               = 1659,
    /// CHANNEL-B
    ChannelB                                                               = 1631,
    /// CHAPTER
    Chapter                                                                = 154,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 1754,
    /// CIRCLE
    Circle                                                                 = 2593,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 503,
    /// CLASSIC
    Classic                                                                = 2066,
    /// CLEAR
    Clear                                                                  = 1169,
    /// CLEAR-ALL-DTCS
    ClearAllDtcs                                                           = 545,
    /// CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    ClearDynamicallyDefineDataIdentifier                                   = 2386,
    /// CLIENT-AUTHENTICATE
    ClientAuthenticate                                                     = 187,
    /// CLIENT-DECRYPT
    ClientDecrypt                                                          = 1095,
    /// CLIENT-ENCRYPT
    ClientEncrypt                                                          = 760,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 510,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 707,
    /// CLIENT-MAC-GENERATE
    ClientMacGenerate                                                      = 1976,
    /// CLIENT-MAC-VERIFY
    ClientMacVerify                                                        = 2568,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 2363,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 1729,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 1481,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 2309,
    /// CLIENT-VERIFY
    ClientVerify                                                           = 1119,
    /// CLOSED
    Closed                                                                 = 2540,
    /// CO
    Co                                                                     = 145,
    /// CODE
    Code                                                                   = 1730,
    /// CODE-GENERATION-TIME
    CodeGenerationTime                                                     = 1387,
    /// CODEGENERATION
    Codegeneration                                                         = 472,
    /// COLDSTART
    Coldstart                                                              = 2486,
    /// COLLECTABLE-ELEMENT
    CollectableElement                                                     = 1130,
    /// COLLECTION
    Collection                                                             = 540,
    /// COLOR-AWARE
    ColorAware                                                             = 2131,
    /// COLOR-BLIND
    ColorBlind                                                             = 1367,
    /// COM-AXIS
    ComAxis                                                                = 2627,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 1596,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 1284,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 1725,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 1632,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 20,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 1510,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 364,
    /// COM-GRANT
    ComGrant                                                               = 2329,
    /// COM-GRANT-DESIGN
    ComGrantDesign                                                         = 665,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 434,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 2306,
    /// COM-MANAGER
    ComManager                                                             = 1953,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 1077,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 397,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 2308,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 2164,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 2601,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 567,
    /// COM-TRIGGER-GRANT
    ComTriggerGrant                                                        = 2385,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 1810,
    /// COMM-CONNECTOR-PORT
    CommConnectorPort                                                      = 106,
    /// COMMAND-LINE-LONG-FORM
    CommandLineLongForm                                                    = 1865,
    /// COMMAND-LINE-SHORT-FORM
    CommandLineShortForm                                                   = 1482,
    /// COMMAND-LINE-SIMPLE-FORM
    CommandLineSimpleForm                                                  = 207,
    /// COMMON
    Common                                                                 = 1681,
    /// COMMUNICATION-CLUSTER
    CommunicationCluster                                                   = 1544,
    /// COMMUNICATION-CONNECTOR
    CommunicationConnector                                                 = 1567,
    /// COMMUNICATION-CONTROLLER
    CommunicationController                                                = 2139,
    /// COMMUNICATION-INTER-ECU
    CommunicationInterEcu                                                  = 2103,
    /// COMMUNICATION-INTRA-PARTITION
    CommunicationIntraPartition                                            = 2321,
    /// COMPILE
    Compile                                                                = 725,
    /// COMPILER
    Compiler                                                               = 2501,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 466,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 2113,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 995,
    /// COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING
    CompositionPortToExecutablePortMapping                                 = 2493,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 1184,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 2663,
    /// COMPU-METHOD
    CompuMethod                                                            = 2197,
    /// COM_AXIS
    Comaxis                                                                = 1079,
    /// CONCRETE
    Concrete                                                               = 218,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 289,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 2454,
    /// CONDITIONAL
    Conditional                                                            = 2250,
    /// CONFIDENTIALITY-OFFSET--0
    ConfidentialityOffset0                                                 = 872,
    /// CONFIDENTIALITY-OFFSET--30
    ConfidentialityOffset30                                                = 985,
    /// CONFIDENTIALITY-OFFSET--50
    ConfidentialityOffset50                                                = 2611,
    /// CONFIG-DATA
    ConfigData                                                             = 2390,
    /// CONFIGURED
    Configured                                                             = 1703,
    /// CONFIRMED
    Confirmed                                                              = 1093,
    /// CONFIRMED-DTC-BIT
    ConfirmedDtcBit                                                        = 585,
    /// CONNECT
    Connect                                                                = 211,
    /// CONSISTENCY-MECHANISM-REQUIRED
    ConsistencyMechanismRequired                                           = 2399,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 634,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 401,
    /// CONSOLE
    Console                                                                = 946,
    /// CONST
    Const                                                                  = 1380,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 72,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 2672,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 1858,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 737,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 1203,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 1051,
    /// CONSUMER
    Consumer                                                               = 1868,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 1492,
    /// CONTINUE-AT-IT-POSITION
    ContinueAtItPosition                                                   = 2176,
    /// CONTINUOUS-ON-MODE
    ContinuousOnMode                                                       = 786,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 640,
    /// COUPLING-ELEMENT-ABSTRACT-DETAILS
    CouplingElementAbstractDetails                                         = 254,
    /// COUPLING-ELEMENT-SWITCH-DETAILS
    CouplingElementSwitchDetails                                           = 14,
    /// COUPLING-PORT
    CouplingPort                                                           = 1245,
    /// COUPLING-PORT-ABSTRACT-SHAPER
    CouplingPortAbstractShaper                                             = 2566,
    /// COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER
    CouplingPortAsynchronousTrafficShaper                                  = 2455,
    /// COUPLING-PORT-CREDIT-BASED-SHAPER
    CouplingPortCreditBasedShaper                                          = 670,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 1516,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 2235,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 2010,
    /// COUPLING-PORT-STRUCTURAL-ELEMENT
    CouplingPortStructuralElement                                          = 2307,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 2244,
    /// CP
    Cp                                                                     = 8,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 46,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 806,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 290,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 379,
    /// CP-SOFTWARE-CLUSTER-RESOURCE
    CpSoftwareClusterResource                                              = 2039,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 533,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 2281,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 1400,
    /// CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterToApplicationPartitionMapping                         = 1298,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 98,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 736,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 2251,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 827,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 2340,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 1032,
    /// CPP
    Cpp                                                                    = 63,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 2009,
    /// CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET
    CppImplementationDataTypeContextTarget                                 = 892,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 716,
    /// CRC-IGNORED
    CrcIgnored                                                             = 1166,
    /// CRC-NOT-SUPPORTED
    CrcNotSupported                                                        = 2670,
    /// CRC-NOT-VALIDATED
    CrcNotValidated                                                        = 1857,
    /// CRC-OPTIONAL
    CrcOptional                                                            = 558,
    /// CRC-SUPPORTED
    CrcSupported                                                           = 928,
    /// CRC-VALIDATED
    CrcValidated                                                           = 368,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 1793,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 1564,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 2264,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 1354,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 1337,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 630,
    /// CRYPTO-INTERFACE
    CryptoInterface                                                        = 954,
    /// CRYPTO-JOB
    CryptoJob                                                              = 150,
    /// CRYPTO-KEY-MANAGEMENT
    CryptoKeyManagement                                                    = 1202,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 442,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 429,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 162,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 1190,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 574,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 700,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 1316,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 1140,
    /// CRYPTO-NEEDS
    CryptoNeeds                                                            = 763,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 1634,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 1180,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 1678,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 1472,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 139,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 1569,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 1524,
    /// CRYPTO-SERVICE-MANAGER
    CryptoServiceManager                                                   = 566,
    /// CRYPTO-SERVICE-MAPPING
    CryptoServiceMapping                                                   = 471,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 2046,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 168,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 2155,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 655,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 1607,
    /// CS
    Cs                                                                     = 538,
    /// CSERS
    Csers                                                                  = 1837,
    /// CURVE-AXIS
    CurveAxis                                                              = 1820,
    /// CURVE_AXIS
    Curveaxis                                                              = 610,
    /// CUSTOM
    Custom                                                                 = 774,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 1653,
    /// CVC
    Cvc                                                                    = 1603,
    /// CY
    Cy                                                                     = 1009,
    /// CYCLE-REPETITION-1
    CycleRepetition1                                                       = 1107,
    /// CYCLE-REPETITION-10
    CycleRepetition10                                                      = 116,
    /// CYCLE-REPETITION-16
    CycleRepetition16                                                      = 1105,
    /// CYCLE-REPETITION-2
    CycleRepetition2                                                       = 2543,
    /// CYCLE-REPETITION-20
    CycleRepetition20                                                      = 1084,
    /// CYCLE-REPETITION-32
    CycleRepetition32                                                      = 1748,
    /// CYCLE-REPETITION-4
    CycleRepetition4                                                       = 35,
    /// CYCLE-REPETITION-40
    CycleRepetition40                                                      = 2406,
    /// CYCLE-REPETITION-5
    CycleRepetition5                                                       = 1474,
    /// CYCLE-REPETITION-50
    CycleRepetition50                                                      = 151,
    /// CYCLE-REPETITION-64
    CycleRepetition64                                                      = 1749,
    /// CYCLE-REPETITION-8
    CycleRepetition8                                                       = 492,
    /// CYCLIC
    Cyclic                                                                 = 2094,
    /// CYCLIC-AND-ON-CHANGE
    CyclicAndOnChange                                                      = 2254,
    /// DA
    Da                                                                     = 1951,
    /// DATA-CONSTR
    DataConstr                                                             = 2100,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 54,
    /// DATA-FORMAT-ELEMENT-REFERENCE
    DataFormatElementReference                                             = 1759,
    /// DATA-FORMAT-ELEMENT-SCOPE
    DataFormatElementScope                                                 = 2571,
    /// DATA-INTERFACE
    DataInterface                                                          = 1371,
    /// DATA-PROTOTYPE
    DataPrototype                                                          = 2393,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 2434,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 1688,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 615,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 654,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 2112,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 1162,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 1861,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 922,
    /// DCM-I-PDU
    DcmIPdu                                                                = 512,
    /// DDS-CP-CONFIG
    DdsCpConfig                                                            = 1932,
    /// DDS-CP-CONSUMED-SERVICE-INSTANCE
    DdsCpConsumedServiceInstance                                           = 1736,
    /// DDS-CP-DOMAIN
    DdsCpDomain                                                            = 818,
    /// DDS-CP-PARTITION
    DdsCpPartition                                                         = 350,
    /// DDS-CP-PROVIDED-SERVICE-INSTANCE
    DdsCpProvidedServiceInstance                                           = 1750,
    /// DDS-CP-QOS-PROFILE
    DdsCpQosProfile                                                        = 2552,
    /// DDS-CP-SERVICE-INSTANCE
    DdsCpServiceInstance                                                   = 1129,
    /// DDS-CP-TOPIC
    DdsCpTopic                                                             = 2327,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 2331,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 1878,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 2412,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 1125,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 1964,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 1262,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 1969,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 2077,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 1808,
    /// DDS-SERVICE
    DdsService                                                             = 1000,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 2465,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 2460,
    /// DDS-SIGNAL
    DdsSignal                                                              = 1164,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 1975,
    /// DE
    De                                                                     = 33,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 853,
    /// DEBOUNCE-DATA
    DebounceData                                                           = 330,
    /// DEBUG
    Debug                                                                  = 2040,
    /// DECREASING
    Decreasing                                                             = 894,
    /// DEDICATED
    Dedicated                                                              = 2654,
    /// DEF-ITEM
    DefItem                                                                = 740,
    /// DEFAULT
    Default                                                                = 305,
    /// DEFAULT-ERROR-TRACER
    DefaultErrorTracer                                                     = 619,
    /// DEFAULT-IF-REVISION-UPDATE
    DefaultIfRevisionUpdate                                                = 66,
    /// DEFAULT-IF-UNDEFINED
    DefaultIfUndefined                                                     = 1465,
    /// DEFAULT-MODE
    DefaultMode                                                            = 2603,
    /// DEFAULT-TRACE-STATE-DISABLED
    DefaultTraceStateDisabled                                              = 406,
    /// DEFAULT-TRACE-STATE-ENABLED
    DefaultTraceStateEnabled                                               = 1669,
    /// DEFAULT-TRIGGER
    DefaultTrigger                                                         = 61,
    /// DEFERRED
    Deferred                                                               = 723,
    /// DEFICIT-ROUND-ROBIN
    DeficitRoundRobin                                                      = 658,
    /// DEFINE-BY-IDENTIFIER
    DefineByIdentifier                                                     = 1439,
    /// DEFINE-BY-MEMORY-ADDRESS
    DefineByMemoryAddress                                                  = 1797,
    /// DEFLATE
    Deflate                                                                = 1378,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 307,
    /// DELETE
    Delete                                                                 = 2420,
    /// DEPENDANT
    Dependant                                                              = 2214,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 1470,
    /// DERIVED-FROM
    DerivedFrom                                                            = 1305,
    /// DESCENDANT
    Descendant                                                             = 2550,
    /// DESELECTED
    Deselected                                                             = 2204,
    /// DETAILED
    Detailed                                                               = 2056,
    /// DETAILED-BYPASSING-FILTERS
    DetailedBypassingFilters                                               = 202,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 1348,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 1402,
    /// DETERMINISTIC-SYNC-INSTANTIATION
    DeterministicSyncInstantiation                                         = 1755,
    /// DETERMINISTIC-SYNC-MASTER
    DeterministicSyncMaster                                                = 2324,
    /// DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING
    DeterministicSyncMasterToTimeBaseConsumerMapping                       = 616,
    /// DEVELOPMENT
    Development                                                            = 275,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 1382,
    /// DEVELOPMENT-ERROR-TRACER
    DevelopmentErrorTracer                                                 = 1844,
    /// DHCPV-4
    Dhcpv4                                                                 = 1235,
    /// DHCPV-6
    Dhcpv6                                                                 = 1901,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 885,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 649,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 2374,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 1655,
    /// DIAG-REQUEST
    DiagRequest                                                            = 352,
    /// DIAG-RESPONSE
    DiagResponse                                                           = 1809,
    /// DIAGNOSTIC-ABSTRACT-ALIAS-EVENT
    DiagnosticAbstractAliasEvent                                           = 389,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER
    DiagnosticAbstractDataIdentifier                                       = 2290,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE
    DiagnosticAbstractDataIdentifierInterface                              = 233,
    /// DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE
    DiagnosticAbstractRoutineInterface                                     = 1937,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 267,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 698,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 1336,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE
    DiagnosticAuthTransmitCertificate                                      = 2170,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION
    DiagnosticAuthTransmitCertificateEvaluation                            = 2256,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING
    DiagnosticAuthTransmitCertificateMapping                               = 870,
    /// DIAGNOSTIC-AUTHENTICATION
    DiagnosticAuthentication                                               = 1350,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 1954,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 56,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 2661,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 109,
    /// DIAGNOSTIC-CAPABILITY-ELEMENT
    DiagnosticCapabilityElement                                            = 2130,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 1295,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 878,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 1226,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 575,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 1982,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 2097,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 2013,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 1088,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 1006,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 411,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 1888,
    /// DIAGNOSTIC-COMMON-ELEMENT
    DiagnosticCommonElement                                                = 874,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER
    DiagnosticCommunicationManager                                         = 2221,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 1624,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 1886,
    /// DIAGNOSTIC-CONDITION
    DiagnosticCondition                                                    = 2662,
    /// DIAGNOSTIC-CONDITION-GROUP
    DiagnosticConditionGroup                                               = 1605,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 877,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 1974,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 444,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 516,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 1638,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 1331,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 2314,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 327,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 2642,
    /// DIAGNOSTIC-DATA-BY-IDENTIFIER
    DiagnosticDataByIdentifier                                             = 355,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 2059,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 1320,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 1379,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 160,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 2151,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 2631,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 1469,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 1532,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 1843,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 1142,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 2373,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 1869,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 335,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE
    DiagnosticDoIpEntityIdentificationInterface                            = 965,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 770,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 362,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 1575,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 1676,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 1173,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 1352,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 2194,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 967,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 1583,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 483,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 1822,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 2302,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 2449,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 691,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 2053,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 2111,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 1929,
    /// DIAGNOSTIC-ENV-MODE-ELEMENT
    DiagnosticEnvModeElement                                               = 2394,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 2338,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 882,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 1871,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 1082,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 441,
    /// DIAGNOSTIC-EVENT-MANAGER
    DiagnosticEventManager                                                 = 1334,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 213,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 2405,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 1938,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 2262,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 1747,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 450,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 443,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 991,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 543,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 2667,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 859,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 1677,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 458,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 257,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 303,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 2397,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 118,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 2452,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 776,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 2680,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 824,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 822,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 1358,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 536,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 1360,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 1422,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 556,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 794,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 1517,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 1280,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 2179,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 1014,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 1760,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 1388,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 366,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 42,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 1341,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 2523,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 1701,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 2475,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 1647,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 2091,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 1654,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 452,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 1612,
    /// DIAGNOSTIC-LOG-AND-TRACE
    DiagnosticLogAndTrace                                                  = 959,
    /// DIAGNOSTIC-MAPPING
    DiagnosticMapping                                                      = 7,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 2008,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 2490,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 31,
    /// DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS
    DiagnosticMemoryAddressableRangeAccess                                 = 1265,
    /// DIAGNOSTIC-MEMORY-BY-ADDRESS
    DiagnosticMemoryByAddress                                              = 2489,
    /// DIAGNOSTIC-MEMORY-DESTINATION
    DiagnosticMemoryDestination                                            = 1866,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 2421,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 1672,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 255,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 742,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 2079,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 1716,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 455,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE
    DiagnosticMultipleConditionInterface                                   = 183,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING
    DiagnosticMultipleConditionPortMapping                                 = 916,
    /// DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE
    DiagnosticMultipleEventInterface                                       = 386,
    /// DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING
    DiagnosticMultipleEventPortMapping                                     = 1563,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE
    DiagnosticMultipleMonitorInterface                                     = 221,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING
    DiagnosticMultipleMonitorPortMapping                                   = 1666,
    /// DIAGNOSTIC-MULTIPLE-RESOURCE-INTERFACE
    DiagnosticMultipleResourceInterface                                    = 1546,
    /// DIAGNOSTIC-MULTIPLE-RESOURCE-PORT-MAPPING
    DiagnosticMultipleResourcePortMapping                                  = 652,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 1573,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 2261,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 225,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 1322,
    /// DIAGNOSTIC-PARAMETER-ELEMENT
    DiagnosticParameterElement                                             = 1887,
    /// DIAGNOSTIC-PARAMETER-IDENT
    DiagnosticParameterIdent                                               = 2470,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 1012,
    /// DIAGNOSTIC-PORT-INTERFACE
    DiagnosticPortInterface                                                = 2577,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 2225,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 532,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 0,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 1223,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 879,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 2205,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 835,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 788,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 901,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 2491,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 590,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 2063,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 644,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 470,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 1098,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 75,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 130,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 889,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 2020,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 555,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 281,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 856,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 155,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 1515,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 2396,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 1928,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE
    DiagnosticRequestFileTransferInterface                                 = 851,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 1176,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 1099,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 943,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 938,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 1085,
    /// DIAGNOSTIC-REQUEST-ROUTINE-RESULTS
    DiagnosticRequestRoutineResults                                        = 2334,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 685,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 552,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 1841,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 866,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 1326,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 1251,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 2090,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 468,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 1396,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 1078,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 1639,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 2121,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 2232,
    /// DIAGNOSTIC-ROUTINE-SUBFUNCTION
    DiagnosticRoutineSubfunction                                           = 463,
    /// DIAGNOSTIC-SECURE-CODING-MAPPING
    DiagnosticSecureCodingMapping                                          = 648,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 1349,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 294,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 2417,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 528,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 2231,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 1230,
    /// DIAGNOSTIC-SERVICE-CLASS
    DiagnosticServiceClass                                                 = 1992,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 2233,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 203,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 18,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 727,
    /// DIAGNOSTIC-SERVICE-INSTANCE
    DiagnosticServiceInstance                                              = 2104,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 2033,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 678,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 626,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 2608,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 1463,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 1721,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 807,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 625,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE
    DiagnosticSovdAuthorizationInterface                                   = 1437,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING
    DiagnosticSovdAuthorizationPortMapping                                 = 222,
    /// DIAGNOSTIC-SOVD-BULK-DATA
    DiagnosticSovdBulkData                                                 = 2201,
    /// DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE
    DiagnosticSovdBulkDataInterface                                        = 1790,
    /// DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING
    DiagnosticSovdBulkDataPortMapping                                      = 2152,
    /// DIAGNOSTIC-SOVD-CONFIGURATION
    DiagnosticSovdConfiguration                                            = 1629,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA
    DiagnosticSovdConfigurationBulkData                                    = 2473,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING
    DiagnosticSovdConfigurationDataIdentifierMapping                       = 1782,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE
    DiagnosticSovdConfigurationInterface                                   = 748,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER
    DiagnosticSovdConfigurationParameter                                   = 34,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING
    DiagnosticSovdConfigurationPortMapping                                 = 1355,
    /// DIAGNOSTIC-SOVD-LOCK
    DiagnosticSovdLock                                                     = 2462,
    /// DIAGNOSTIC-SOVD-LOG
    DiagnosticSovdLog                                                      = 1802,
    /// DIAGNOSTIC-SOVD-METHOD
    DiagnosticSovdMethod                                                   = 259,
    /// DIAGNOSTIC-SOVD-METHOD-PRIMITIVE
    DiagnosticSovdMethodPrimitive                                          = 170,
    /// DIAGNOSTIC-SOVD-PORT-INTERFACE
    DiagnosticSovdPortInterface                                            = 295,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE
    DiagnosticSovdProximityChallengeInterface                              = 842,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING
    DiagnosticSovdProximityChallengePortMapping                            = 1342,
    /// DIAGNOSTIC-SOVD-SERVICE-INSTANCE
    DiagnosticSovdServiceInstance                                          = 2658,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE
    DiagnosticSovdServiceValidationInterface                               = 2675,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING
    DiagnosticSovdServiceValidationPortMapping                             = 753,
    /// DIAGNOSTIC-SOVD-UPDATE
    DiagnosticSovdUpdate                                                   = 1283,
    /// DIAGNOSTIC-SOVD-UPDATE-INTERFACE
    DiagnosticSovdUpdateInterface                                          = 2346,
    /// DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING
    DiagnosticSovdUpdatePortMapping                                        = 1426,
    /// DIAGNOSTIC-START-ROUTINE
    DiagnosticStartRoutine                                                 = 587,
    /// DIAGNOSTIC-STOP-ROUTINE
    DiagnosticStopRoutine                                                  = 336,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 1693,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 1508,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 2028,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 2038,
    /// DIAGNOSTIC-SW-MAPPING
    DiagnosticSwMapping                                                    = 1789,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 219,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 348,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 741,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 319,
    /// DIAGNOSTIC-TROUBLE-CODE
    DiagnosticTroubleCode                                                  = 581,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 561,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 271,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 2436,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 438,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 2126,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 930,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 1052,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 357,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 1069,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 1247,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 1171,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 1800,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 286,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 596,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 381,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 2353,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 1744,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 2457,
    /// DISABLE
    Disable                                                                = 721,
    /// DLNA
    Dlna                                                                   = 408,
    /// DLT-APPLICATION
    DltApplication                                                         = 2439,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 1791,
    /// DLT-ARGUMENT
    DltArgument                                                            = 586,
    /// DLT-CONTEXT
    DltContext                                                             = 79,
    /// DLT-ECU
    DltEcu                                                                 = 2364,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 2135,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 731,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 1023,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 880,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 2633,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 1979,
    /// DLT-MESSAGE
    DltMessage                                                             = 1111,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 2215,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 672,
    /// DO-IP
    DoIp                                                                   = 1711,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 1667,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 1385,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 2031,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 1160,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 1545,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 2673,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 1434,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 563,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 2050,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 2360,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 1855,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 757,
    /// DO-IP-SERVICE-NEEDS
    DoIpServiceNeeds                                                       = 1029,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 1198,
    /// DO-NOT-INCLUDE
    DoNotInclude                                                           = 1547,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 1894,
    /// DOCUMENTATION
    Documentation                                                          = 2006,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 2526,
    /// DOES-NOT-REPORT-EXECUTION-STATE
    DoesNotReportExecutionState                                            = 1244,
    /// DOES-NOT-SUPPORT-BUFFER-LOCKING
    DoesNotSupportBufferLocking                                            = 430,
    /// DOES-NOT-USE-LOGGING
    DoesNotUseLogging                                                      = 2509,
    /// DOMAIN-PARTICIPANT-USER-DATA-QOS
    DomainParticipantUserDataQos                                           = 1840,
    /// DONT-INVALIDATE
    DontInvalidate                                                         = 743,
    /// DROP
    Drop                                                                   = 38,
    /// DROP-FRAME
    DropFrame                                                              = 952,
    /// DROP-UNTAGGED
    DropUntagged                                                           = 2358,
    /// DSA
    Dsa                                                                    = 795,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 41,
    /// DYNAMIC
    Dynamic                                                                = 550,
    /// DYNAMIC-PART-TRIGGER
    DynamicPartTrigger                                                     = 1661,
    /// DZ
    Dz                                                                     = 1345,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 2395,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 2282,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 1149,
    /// ECC
    Ecc                                                                    = 487,
    /// ECU
    Ecu                                                                    = 2649,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 1838,
    /// ECU-INSTANCE
    EcuInstance                                                            = 96,
    /// ECU-MANAGER
    EcuManager                                                             = 2575,
    /// ECU-MAPPING
    EcuMapping                                                             = 140,
    /// ECU-PARTITION
    EcuPartition                                                           = 1466,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 1063,
    /// ECU-TIMING
    EcuTiming                                                              = 1704,
    /// ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF
    EcucAbstractExternalReferenceDef                                       = 1692,
    /// ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF
    EcucAbstractInternalReferenceDef                                       = 504,
    /// ECUC-ABSTRACT-REFERENCE-DEF
    EcucAbstractReferenceDef                                               = 1239,
    /// ECUC-ABSTRACT-STRING-PARAM-DEF
    EcucAbstractStringParamDef                                             = 2276,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 1246,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 369,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 114,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 2156,
    /// ECUC-COMMON-ATTRIBUTES
    EcucCommonAttributes                                                   = 2014,
    /// ECUC-CONTAINER-DEF
    EcucContainerDef                                                       = 270,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 592,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 693,
    /// ECUC-DEFINITION-ELEMENT
    EcucDefinitionElement                                                  = 2095,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 2496,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 793,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 1411,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 896,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 1389,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 2438,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 457,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 178,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 2671,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 890,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 1519,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 1503,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 1537,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 2591,
    /// ECUC-PARAMETER-DEF
    EcucParameterDef                                                       = 1536,
    /// ECUC-QUERY
    EcucQuery                                                              = 217,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 2456,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 527,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 1024,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 902,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 2404,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 1175,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 1037,
    /// EDGE-NODE
    EdgeNode                                                               = 1364,
    /// EID-USE-API
    EidUseApi                                                              = 2019,
    /// EID-USE-CONFIG-VALUE
    EidUseConfigValue                                                      = 1030,
    /// EID-USE-MAC
    EidUseMac                                                              = 1488,
    /// EL
    El                                                                     = 1580,
    /// EMISSION-RELATED-DTC
    EmissionRelatedDtc                                                     = 2144,
    /// EN
    En                                                                     = 416,
    /// ENABLE
    Enable                                                                 = 1054,
    /// ENABLED
    Enabled                                                                = 1307,
    /// ENCRYPT-AND-SIGN
    EncryptAndSign                                                         = 809,
    /// ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION
    EncryptAndSignWithOriginAuthentication                                 = 2109,
    /// ENCRYPTION
    Encryption                                                             = 1682,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 803,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 907,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 980,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 2049,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 2252,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 1110,
    /// ENHANCED
    Enhanced                                                               = 978,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 1707,
    /// EO
    Eo                                                                     = 1880,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 1879,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 2433,
    /// EOC-EXECUTABLE-ENTITY-REF-ABSTRACT
    EocExecutableEntityRefAbstract                                         = 1829,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 2356,
    /// EPS
    Eps                                                                    = 391,
    /// EQUAL
    Equal                                                                  = 48,
    /// ERROR
    Error                                                                  = 1867,
    /// ERROR-CORRECTION
    ErrorCorrection                                                        = 173,
    /// ERROR-DETECTION
    ErrorDetection                                                         = 1414,
    /// ERROR-TRACER
    ErrorTracer                                                            = 2481,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 325,
    /// ES
    Es                                                                     = 2098,
    /// ESP
    Esp                                                                    = 1618,
    /// ET
    Et                                                                     = 858,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 2365,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 1452,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 2447,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 2687,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 2692,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 1460,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 935,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 599,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 2548,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 2643,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 910,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 2234,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 1042,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 262,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 1071,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 1980,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 1347,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 40,
    /// EU
    Eu                                                                     = 2279,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 2128,
    /// EVAP
    Evap                                                                   = 326,
    /// EVAPPURGEFLOW
    Evappurgeflow                                                          = 739,
    /// EVENT-ACCEPTANCE-DISABLED
    EventAcceptanceDisabled                                                = 1115,
    /// EVENT-ACCEPTANCE-ENABLED
    EventAcceptanceEnabled                                                 = 1595,
    /// EVENT-COMBINATION-ON-RETRIEVAL
    EventCombinationOnRetrieval                                            = 1335,
    /// EVENT-COMBINATION-ON-STORAGE
    EventCombinationOnStorage                                              = 1906,
    /// EVENT-HANDLER
    EventHandler                                                           = 2093,
    /// EVENT-MAPPING
    EventMapping                                                           = 633,
    /// EVENT-STORAGE-DISABLED
    EventStorageDisabled                                                   = 2000,
    /// EVENT-STORAGE-ENABLED
    EventStorageEnabled                                                    = 891,
    /// EVENT-TRIGGERING-CONSTRAINT
    EventTriggeringConstraint                                              = 244,
    /// EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE
    EventWindowCurrentAndFollowingCycle                                    = 804,
    /// EVENT-WINDOW-CURRENT-CYCLE
    EventWindowCurrentCycle                                                = 2357,
    /// EVENT-WINDOW-INFINITE
    EventWindowInfinite                                                    = 91,
    /// EXACT-OR-ANY-MINOR-VERSION
    ExactOrAnyMinorVersion                                                 = 1045,
    /// EXAMPLE
    Example                                                                = 2229,
    /// EXCLUDE-FROM-FLASH
    ExcludeFromFlash                                                       = 115,
    /// EXCLUSIVE
    Exclusive                                                              = 1438,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 937,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 199,
    /// EXECUTABLE
    Executable                                                             = 667,
    /// EXECUTABLE-ENTITY
    ExecutableEntity                                                       = 814,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 248,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 1577,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 1912,
    /// EXECUTE
    Execute                                                                = 1108,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 2316,
    /// EXECUTION-TIME
    ExecutionTime                                                          = 1877,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 1026,
    /// EXERCISE
    Exercise                                                               = 332,
    /// EXPLICIT
    Explicit                                                               = 1075,
    /// EXTEND
    Extend                                                                 = 2237,
    /// EXTENDED
    Extended                                                               = 544,
    /// EXTERNAL-REPLACEMENT
    ExternalReplacement                                                    = 1743,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 1406,
    /// EXTERNAL-TRIGGERING-POINT-IDENT
    ExternalTriggeringPointIdent                                           = 1628,
    /// FA
    Fa                                                                     = 399,
    /// FAILURE-AND-SUCCESS
    FailureAndSuccess                                                      = 2048,
    /// FAILURE-ONLY
    FailureOnly                                                            = 2382,
    /// FALSE
    False                                                                  = 2604,
    /// FAST-FLASHING-MODE
    FastFlashingMode                                                       = 495,
    /// FATAL
    Fatal                                                                  = 2653,
    /// FAULT
    Fault                                                                  = 2253,
    /// FDC-THRESHOLD
    FdcThreshold                                                           = 900,
    /// FI
    Fi                                                                     = 1113,
    /// FIBEX-ELEMENT
    FibexElement                                                           = 133,
    /// FIELD
    Field                                                                  = 591,
    /// FIELD-MAPPING
    FieldMapping                                                           = 664,
    /// FILE
    File                                                                   = 1206,
    /// FILTERED
    Filtered                                                               = 1548,
    /// FINISH
    Finish                                                                 = 453,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 2522,
    /// FIRE-AND-FORGET-METHOD-MAPPING
    FireAndForgetMethodMapping                                             = 1020,
    /// FIREWALL-RULE
    FirewallRule                                                           = 1424,
    /// FIREWALL-STATE-SWITCH-INTERFACE
    FirewallStateSwitchInterface                                           = 1900,
    /// FIRST-CONTAINED-TRIGGER
    FirstContainedTrigger                                                  = 927,
    /// FIRST-TO-SECOND
    FirstToSecond                                                          = 747,
    /// FIT-TO-PAGE
    FitToPage                                                              = 1924,
    /// FIT-TO-TEXT
    FitToText                                                              = 2581,
    /// FIX-AXIS
    FixAxis                                                                = 2599,
    /// FIXED
    Fixed                                                                  = 573,
    /// FIXED-SIZE
    FixedSize                                                              = 1025,
    /// FIX_AXIS
    Fixaxis                                                                = 692,
    /// FJ
    Fj                                                                     = 2551,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 796,
    /// FLAT-MAP
    FlatMap                                                                = 2136,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 2304,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 253,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 293,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 103,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 2016,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 2092,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 525,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 2018,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 1428,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 1562,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 1971,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 2557,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 1955,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 1031,
    /// FLOAT
    Float                                                                  = 1220,
    /// FLOAT-32-BIT
    Float32Bit                                                             = 2459,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 2350,
    /// FM-FEATURE
    FmFeature                                                              = 662,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 277,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 2640,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 1523,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 608,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 1571,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 830,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 2586,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 992,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 323,
    /// FO
    Fo                                                                     = 1892,
    /// FOR-ALL
    ForAll                                                                 = 1252,
    /// FORGET
    Forget                                                                 = 557,
    /// FORWARD-AS-IS
    ForwardAsIs                                                            = 681,
    /// FR
    Fr                                                                     = 839,
    /// FRAME
    Frame                                                                  = 1645,
    /// FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION
    FrameEthernetQueuedForTransmission                                     = 2622,
    /// FRAME-ETHERNET-RECEIVED-BY-IF
    FrameEthernetReceivedByIf                                              = 734,
    /// FRAME-ETHERNET-RECEIVED-ON-BUS
    FrameEthernetReceivedOnBus                                             = 606,
    /// FRAME-ETHERNET-SENT-ON-BUS
    FrameEthernetSentOnBus                                                 = 261,
    /// FRAME-PORT
    FramePort                                                              = 1812,
    /// FRAME-QUEUED-FOR-TRANSMISSION
    FrameQueuedForTransmission                                             = 1958,
    /// FRAME-RECEIVED-BY-IF
    FrameReceivedByIf                                                      = 2499,
    /// FRAME-TRANSMITTED-ON-BUS
    FrameTransmittedOnBus                                                  = 1443,
    /// FRAME-TRIGGERING
    FrameTriggering                                                        = 426,
    /// FULL
    Full                                                                   = 956,
    /// FULL-COM
    FullCom                                                                = 1521,
    /// FULL-DUPLEX-MODE
    FullDuplexMode                                                         = 296,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 913,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 2525,
    /// FUNCTION-GROUP-STATE-TO-NM-HANDLE
    FunctionGroupStateToNmHandle                                           = 2299,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 2278,
    /// FUNCTION-INHIBITION-MANAGER
    FunctionInhibitionManager                                              = 2392,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 2362,
    /// FUNCTIONAL
    Functional                                                             = 141,
    /// FUNCTIONAL-ADDRESS
    FunctionalAddress                                                      = 1570,
    /// FUNCTIONAL-CAN-FD
    FunctionalCanFd                                                        = 1300,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING
    FunctionalClusterInteractsWithFunctionalClusterMapping                 = 2644,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING
    FunctionalClusterInteractsWithPersistencyDeploymentMapping             = 2497,
    /// FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING
    FunctionalClusterToSecurityEventDefinitionMapping                      = 76,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 912,
    /// FY
    Fy                                                                     = 1885,
    /// GA
    Ga                                                                     = 1882,
    /// GATEWAY
    Gateway                                                                = 1962,
    /// GD
    Gd                                                                     = 961,
    /// GENERAL-PARAMETER
    GeneralParameter                                                       = 710,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 301,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 126,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 686,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 433,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 138,
    /// GET
    Get                                                                    = 2558,
    /// GETTER
    Getter                                                                 = 2183,
    /// GETTER-SETTER
    GetterSetter                                                           = 1731,
    /// GIF
    Gif                                                                    = 888,
    /// GL
    Gl                                                                     = 71,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 2124,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 251,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 143,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 147,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 2223,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 865,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 2359,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 24,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 1834,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 2283,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 674,
    /// GLOBAL-TIME-MASTER
    GlobalTimeMaster                                                       = 322,
    /// GLOBAL-TIME-SLAVE
    GlobalTimeSlave                                                        = 1598,
    /// GN
    Gn                                                                     = 272,
    /// GRANT
    Grant                                                                  = 668,
    /// GRANT-DESIGN
    GrantDesign                                                            = 407,
    /// GRAYSCALE
    Grayscale                                                              = 2319,
    /// GROSS
    Gross                                                                  = 2560,
    /// GU
    Gu                                                                     = 2226,
    /// GZIP
    Gzip                                                                   = 1660,
    /// HA
    Ha                                                                     = 886,
    /// HALF-DUPLEX-MODE
    HalfDuplexMode                                                         = 1728,
    /// HARDWARE-TEST-MANAGER
    HardwareTestManager                                                    = 1267,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 2062,
    /// HEAD
    Head                                                                   = 1172,
    /// HEALTH-CHANNEL
    HealthChannel                                                          = 241,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 2685,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 328,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 105,
    /// HEAP-USAGE
    HeapUsage                                                              = 2153,
    /// HI
    Hi                                                                     = 1833,
    /// HIERARCHICAL-EOC
    HierarchicalEoc                                                        = 718,
    /// HIGH
    High                                                                   = 1070,
    /// HINT
    Hint                                                                   = 521,
    /// HOOK
    Hook                                                                   = 1487,
    /// HOST-PORT
    HostPort                                                               = 848,
    /// HR
    Hr                                                                     = 2303,
    /// HU
    Hu                                                                     = 1237,
    /// HUB
    Hub                                                                    = 1554,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 1805,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 1368,
    /// HW-CATEGORY
    HwCategory                                                             = 1816,
    /// HW-DESCRIPTION-ENTITY
    HwDescriptionEntity                                                    = 2240,
    /// HW-ELEMENT
    HwElement                                                              = 1919,
    /// HW-PIN
    HwPin                                                                  = 2168,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 62,
    /// HW-TYPE
    HwType                                                                 = 2485,
    /// HY
    Hy                                                                     = 1526,
    /// I-4-G
    I4G                                                                    = 1361,
    /// I-PDU
    IPdu                                                                   = 1151,
    /// I-PDU-PORT
    IPduPort                                                               = 1401,
    /// I-PDU-RECEIVED-BY-COM
    IPduReceivedByCom                                                      = 833,
    /// I-PDU-SENT-TO-IF
    IPduSentToIf                                                           = 1249,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 57,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 1973,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 1260,
    /// I-SIGNAL
    ISignal                                                                = 1188,
    /// I-SIGNAL-AVAILABLE-FOR-RTE
    ISignalAvailableForRte                                                 = 2236,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 2246,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 359,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 2682,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 772,
    /// I-SIGNAL-SENT-TO-COM
    ISignalSentToCom                                                       = 1259,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 949,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 520,
    /// IA
    Ia                                                                     = 826,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 2678,
    /// ICMP
    Icmp                                                                   = 2323,
    /// ICV-IGNORED
    IcvIgnored                                                             = 754,
    /// ICV-NOT-SUPPORTED
    IcvNotSupported                                                        = 378,
    /// ICV-NOT-VERIFIED
    IcvNotVerified                                                         = 169,
    /// ICV-OPTIONAL
    IcvOptional                                                            = 197,
    /// ICV-SUPPORTED
    IcvSupported                                                           = 1771,
    /// ICV-VERIFIED
    IcvVerified                                                            = 1139,
    /// IDENT-CAPTION
    IdentCaption                                                           = 1614,
    /// IDENTIFIABLE
    Identifiable                                                           = 410,
    /// IDS-COMMON-ELEMENT
    IdsCommonElement                                                       = 514,
    /// IDS-DESIGN
    IdsDesign                                                              = 1415,
    /// IDS-MAPPING
    IdsMapping                                                             = 862,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 1635,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 1436,
    /// IDS-PLATFORM-INSTANTIATION
    IdsPlatformInstantiation                                               = 1905,
    /// IDSM-ABSTRACT-PORT-INTERFACE
    IdsmAbstractPortInterface                                              = 1847,
    /// IDSM-CONTEXT-PROVIDER-INTERFACE
    IdsmContextProviderInterface                                           = 1860,
    /// IDSM-CONTEXT-PROVIDER-MAPPING
    IdsmContextProviderMapping                                             = 2025,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 1893,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 1191,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 181,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 1576,
    /// IDSM-TIMESTAMP-PROVIDER-INTERFACE
    IdsmTimestampProviderInterface                                         = 2664,
    /// IDSM-TIMESTAMP-PROVIDER-MAPPING
    IdsmTimestampProviderMapping                                           = 1908,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 2078,
    /// IE
    Ie                                                                     = 1179,
    /// IEC-61937
    Iec61937                                                               = 2344,
    /// IEEE-1722-TP-AAF-CONNECTION
    Ieee1722TpAafConnection                                                = 2583,
    /// IEEE-1722-TP-ACF-BUS
    Ieee1722TpAcfBus                                                       = 1375,
    /// IEEE-1722-TP-ACF-BUS-PART
    Ieee1722TpAcfBusPart                                                   = 1957,
    /// IEEE-1722-TP-ACF-CAN
    Ieee1722TpAcfCan                                                       = 216,
    /// IEEE-1722-TP-ACF-CAN-PART
    Ieee1722TpAcfCanPart                                                   = 712,
    /// IEEE-1722-TP-ACF-CONNECTION
    Ieee1722TpAcfConnection                                                = 919,
    /// IEEE-1722-TP-ACF-LIN
    Ieee1722TpAcfLin                                                       = 925,
    /// IEEE-1722-TP-ACF-LIN-PART
    Ieee1722TpAcfLinPart                                                   = 2500,
    /// IEEE-1722-TP-AV-CONNECTION
    Ieee1722TpAvConnection                                                 = 481,
    /// IEEE-1722-TP-CONFIG
    Ieee1722TpConfig                                                       = 1950,
    /// IEEE-1722-TP-CONNECTION
    Ieee1722TpConnection                                                   = 988,
    /// IEEE-1722-TP-CRF-CONNECTION
    Ieee1722TpCrfConnection                                                = 909,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 446,
    /// IEEE-1722-TP-IIDC-CONNECTION
    Ieee1722TpIidcConnection                                               = 375,
    /// IEEE-1722-TP-RVF-CONNECTION
    Ieee1722TpRvfConnection                                                = 604,
    /// IEEE802-11P
    Ieee802_11p                                                            = 2,
    /// IEEE802-1AS
    Ieee802_1as                                                            = 428,
    /// IEEE802-1AS-AUTOSAR
    Ieee802_1asAutosar                                                     = 2293,
    /// IGNITION
    Ignition                                                               = 661,
    /// IGNORE
    Ignore                                                                 = 1293,
    /// IK
    Ik                                                                     = 1706,
    /// IMMEDIATE
    Immediate                                                              = 697,
    /// IMPLEMENTATION
    Implementation                                                         = 246,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 371,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 1674,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 80,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 1207,
    /// IMPLEMENTATION-PROPS
    ImplementationProps                                                    = 88,
    /// IMPOSITION-TIME
    ImpositionTime                                                         = 549,
    /// IMPOSITION-TIME-DEFINITION-GROUP
    ImpositionTimeDefinitionGroup                                          = 2519,
    /// IN
    In                                                                     = 1874,
    /// INCLUDE-BUT-DO-NOT-START
    IncludeButDoNotStart                                                   = 473,
    /// INCREASING
    Increasing                                                             = 2676,
    /// INDICATE
    Indicate                                                               = 1103,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 395,
    /// INDIVIDUAL
    Individual                                                             = 1092,
    /// INFINITE
    Infinite                                                               = 1936,
    /// INFINITE-TIME-TO-RESPONSE
    InfiniteTimeToResponse                                                 = 993,
    /// INFO
    Info                                                                   = 1409,
    /// INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE
    InheritedFromArrayElementTypeSize                                      = 1801,
    /// INIT-EVENT
    InitEvent                                                              = 23,
    /// INLINE
    Inline                                                                 = 2511,
    /// INLINE-CONDITIONAL
    InlineConditional                                                      = 2222,
    /// INOUT
    Inout                                                                  = 68,
    /// INSTALL
    Install                                                                = 821,
    /// INSTANCE-ID
    InstanceId                                                             = 1242,
    /// INSTRUCTION
    Instruction                                                            = 53,
    /// INT-16-BIT
    Int16Bit                                                               = 843,
    /// INT-24-BIT
    Int24Bit                                                               = 603,
    /// INT-32-BIT
    Int32Bit                                                               = 1963,
    /// INTER-LET-ONLY
    InterLetOnly                                                           = 1623,
    /// INTER-PARTITION-INTRA-ECU
    InterPartitionIntraEcu                                                 = 2582,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 1959,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 621,
    /// INTERGRITY-AND-CONFIDENTIALITY
    IntergrityAndConfidentiality                                           = 2027,
    /// INTERGRITY-WITHOUT-CONFIDENTIALITY
    IntergrityWithoutConfidentiality                                       = 2127,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 2464,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 204,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 1050,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 620,
    /// INTERRUPT
    Interrupt                                                              = 669,
    /// INTERRUPT-CAT-1
    InterruptCat1                                                          = 749,
    /// INTERRUPT-CAT-2
    InterruptCat2                                                          = 1616,
    /// INTRA-LET-EOC
    IntraLetEoc                                                            = 875,
    /// INTRUSION-DETECTION-SECURITY-MANAGEMENT
    IntrusionDetectionSecurityManagement                                   = 1475,
    /// INVALID
    Invalid                                                                = 1123,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 1792,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 2213,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 746,
    /// IP-SEC-RULE
    IpSecRule                                                              = 1599,
    /// IPSEC
    Ipsec                                                                  = 1846,
    /// IS
    Is                                                                     = 828,
    /// IS-EQUAL
    IsEqual                                                                = 16,
    /// IS-EXPIRED
    IsExpired                                                              = 2067,
    /// IS-FAILED
    IsFailed                                                               = 1269,
    /// IS-GREATER-OR-EQUAL
    IsGreaterOrEqual                                                       = 2238,
    /// IS-GREATER-THAN
    IsGreaterThan                                                          = 1917,
    /// IS-GREATER-THAN-OR-EQUAL
    IsGreaterThanOrEqual                                                   = 2230,
    /// IS-LESS-OR-EQUAL
    IsLessOrEqual                                                          = 1412,
    /// IS-LESS-THAN
    IsLessThan                                                             = 2032,
    /// IS-LESS-THAN-OR-EQUAL
    IsLessThanOrEqual                                                      = 951,
    /// IS-NOT-EQUAL
    IsNotEqual                                                             = 1530,
    /// IS-NOT-RELEVANT
    IsNotRelevant                                                          = 801,
    /// IS-OK
    IsOk                                                                   = 931,
    /// IS-RELEVANT
    IsRelevant                                                             = 730,
    /// IS-STOPPED
    IsStopped                                                              = 415,
    /// ISO
    Iso                                                                    = 1986,
    /// ISO-11992--4
    Iso11992_4                                                             = 1767,
    /// ISO-14229--1
    Iso14229_1                                                             = 2665,
    /// ISO-15031--6
    Iso15031_6                                                             = 1786,
    /// ISO-6
    Iso6                                                                   = 673,
    /// IT
    It                                                                     = 2446,
    /// ITALIC
    Italic                                                                 = 461,
    /// ITU-BT-2020
    ItuBt2020                                                              = 1531,
    /// IW
    Iw                                                                     = 1600,
    /// J-1939
    J1939                                                                  = 2149,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 136,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 653,
    /// J-1939-DCM
    J1939Dcm                                                               = 637,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 1909,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 454,
    /// J-1939-NM--AAC
    J1939NmAac                                                             = 714,
    /// J-1939-NM--CCA
    J1939NmCca                                                             = 719,
    /// J-1939-NM--NCA
    J1939NmNca                                                             = 230,
    /// J-1939-NM--SCA
    J1939NmSca                                                             = 156,
    /// J-1939-NM--SVCA
    J1939NmSvca                                                            = 1196,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 1500,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 1417,
    /// J-1939-REQUEST-MANAGER
    J1939RequestManager                                                    = 2022,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 1646,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 2431,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 2555,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 2625,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 1611,
    /// JA
    Ja                                                                     = 2219,
    /// JAVA
    Java                                                                   = 2645,
    /// JI
    Ji                                                                     = 209,
    /// JPG
    Jpg                                                                    = 479,
    /// JUSTIFY
    Justify                                                                = 564,
    /// JW
    Jw                                                                     = 2535,
    /// KA
    Ka                                                                     = 750,
    /// KEEP
    Keep                                                                   = 131,
    /// KEEP-ALL
    KeepAll                                                                = 726,
    /// KEEP-EXISTING
    KeepExisting                                                           = 1156,
    /// KEEP-LAST
    KeepLast                                                               = 717,
    /// KEY-DERIVATION
    KeyDerivation                                                          = 647,
    /// KEY-SERVER
    KeyServer                                                              = 1586,
    /// KEY-STORAGE
    KeyStorage                                                             = 210,
    /// KEYWORD
    Keyword                                                                = 282,
    /// KEYWORD-SET
    KeywordSet                                                             = 339,
    /// KK
    Kk                                                                     = 195,
    /// KL
    Kl                                                                     = 1726,
    /// KM
    Km                                                                     = 382,
    /// KN
    Kn                                                                     = 1926,
    /// KO
    Ko                                                                     = 565,
    /// KS
    Ks                                                                     = 920,
    /// KU
    Ku                                                                     = 1096,
    /// KY
    Ky                                                                     = 1457,
    /// LA
    La                                                                     = 2342,
    /// LAND
    Land                                                                   = 2196,
    /// LAST-FAILED
    LastFailed                                                             = 977,
    /// LAST-IS-BEST
    LastIsBest                                                             = 1542,
    /// LAST-MODE
    LastMode                                                               = 950,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 623,
    /// LEAF-OF-TARGET-CONTAINER
    LeafOfTargetContainer                                                  = 2065,
    /// LEFT
    Left                                                                   = 694,
    /// LEGACY
    Legacy                                                                 = 1803,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 2478,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 2211,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 214,
    /// LIMIT-TO-PAGE
    LimitToPage                                                            = 1015,
    /// LIMIT-TO-TEXT
    LimitToText                                                            = 1353,
    /// LIN-CLUSTER
    LinCluster                                                             = 432,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 1772,
    /// LIN-COMMUNICATION-CONTROLLER
    LinCommunicationController                                             = 1943,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 1643,
    /// LIN-FRAME
    LinFrame                                                               = 1271,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 800,
    /// LIN-MASTER
    LinMaster                                                              = 9,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 1658,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 1231,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 409,
    /// LIN-SLAVE
    LinSlave                                                               = 526,
    /// LIN-SLAVE-CONFIG-IDENT
    LinSlaveConfigIdent                                                    = 171,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 1453,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 189,
    /// LIN-TP-NODE
    LinTpNode                                                              = 2193,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 2015,
    /// LINK
    Link                                                                   = 904,
    /// LINK-LOCAL
    LinkLocal                                                              = 2138,
    /// LINK-LOCAL--DOIP
    LinkLocalDoip                                                          = 1241,
    /// LINK-TIME
    LinkTime                                                               = 1540,
    /// LINKER
    Linker                                                                 = 1499,
    /// LISTEN
    Listen                                                                 = 1141,
    /// LN
    Ln                                                                     = 1219,
    /// LO
    Lo                                                                     = 2400,
    /// LOCAL
    Local                                                                  = 2002,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 1456,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 1021,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 1432,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 2143,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 17,
    /// LOGICAL-AND
    LogicalAnd                                                             = 1584,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 238,
    /// LOGICAL-OR
    LogicalOr                                                              = 1055,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 1183,
    /// LONG-HEADER
    LongHeader                                                             = 2166,
    /// LOW
    Low                                                                    = 2189,
    /// LOWER-12-BIT
    Lower12Bit                                                             = 1602,
    /// LOWER-8-BIT
    Lower8Bit                                                              = 628,
    /// LT
    Lt                                                                     = 1423,
    /// LT-AFFECTS-PB
    LtAffectsPb                                                            = 1210,
    /// LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING
    LtMessageCollectionToPortPrototypeMapping                              = 83,
    /// LTS-13
    Lts13                                                                  = 1157,
    /// LV
    Lv                                                                     = 1671,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 914,
    /// MAC-SEC-GLOBAL-KAY-PROPS
    MacSecGlobalKayProps                                                   = 163,
    /// MAC-SEC-KAY-PARTICIPANT
    MacSecKayParticipant                                                   = 1187,
    /// MAC-SEC-PARTICIPANT-SET
    MacSecParticipantSet                                                   = 2617,
    /// MACHINE
    Machine                                                                = 2035,
    /// MACHINE-CYCLE
    MachineCycle                                                           = 2267,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 2448,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 1920,
    /// MACHINE-TIMING
    MachineTiming                                                          = 2414,
    /// MACRO
    Macro                                                                  = 1303,
    /// MALFUNCTION
    Malfunction                                                            = 2429,
    /// MANUAL-BY-PARTICIPANT
    ManualByParticipant                                                    = 1904,
    /// MANUAL-BY-TOPIC
    ManualByTopic                                                          = 1914,
    /// MANUFACTURING
    Manufacturing                                                          = 265,
    /// MAPPING-SCOPE-CORE
    MappingScopeCore                                                       = 1870,
    /// MAPPING-SCOPE-ECU
    MappingScopeEcu                                                        = 1394,
    /// MAPPING-SCOPE-PARTITION
    MappingScopePartition                                                  = 1486,
    /// MASEKD-NEW-EQUALS-MASKED-OLD
    MasekdNewEqualsMaskedOld                                               = 2440,
    /// MASEKD-NEW-EQUALS-X
    MasekdNewEqualsX                                                       = 1762,
    /// MASKED-NEW-DIFFERS-MASKED-OLD
    MaskedNewDiffersMaskedOld                                              = 779,
    /// MASKED-NEW-DIFFERS-X
    MaskedNewDiffersX                                                      = 1234,
    /// MASKED-NEW-EQUALS-MASKED-OLD
    MaskedNewEqualsMaskedOld                                               = 506,
    /// MASKED-NEW-EQUALS-X
    MaskedNewEqualsX                                                       = 1827,
    /// MASTER
    Master                                                                 = 758,
    /// MASTER-ECU
    MasterEcu                                                              = 2636,
    /// MAX
    Max                                                                    = 1574,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 583,
    /// MC-FUNCTION
    McFunction                                                             = 2688,
    /// MC-GROUP
    McGroup                                                                = 1679,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 1722,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 1930,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 2082,
    /// MEASUREMENT-POINT
    MeasurementPoint                                                       = 2041,
    /// MEDIUM
    Medium                                                                 = 1136,
    /// MEMORY-SECTION
    MemorySection                                                          = 981,
    /// MEMORY-USAGE
    MemoryUsage                                                            = 706,
    /// METHOD-MAPPING
    MethodMapping                                                          = 1907,
    /// MG
    Mg                                                                     = 755,
    /// MI
    Mi                                                                     = 400,
    /// MIDDLE
    Middle                                                                 = 570,
    /// MIN
    Min                                                                    = 976,
    /// MINIMUM-MINOR-VERSION
    MinimumMinorVersion                                                    = 2141,
    /// MIXED
    Mixed                                                                  = 1034,
    /// MIXED-29-BIT
    Mixed29Bit                                                             = 2425,
    /// MK
    Mk                                                                     = 577,
    /// ML
    Ml                                                                     = 571,
    /// MN
    Mn                                                                     = 918,
    /// MO
    Mo                                                                     = 2206,
    /// MODE-ACCESS-POINT-IDENT
    ModeAccessPointIdent                                                   = 1708,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 541,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 1370,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 1325,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 861,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 1013,
    /// MODE-DECLARATION-REQUESTED
    ModeDeclarationRequested                                               = 607,
    /// MODE-DECLARATION-SWITCH-COMPLETED
    ModeDeclarationSwitchCompleted                                         = 645,
    /// MODE-DECLARATION-SWITCH-INITIATED
    ModeDeclarationSwitchInitiated                                         = 1080,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 1582,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 158,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 2173,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 1897,
    /// MODE-TRANSITION
    ModeTransition                                                         = 1473,
    /// MODELED
    Modeled                                                                = 2517,
    /// MONITOR-MODE
    MonitorMode                                                            = 2388,
    /// MONO
    Mono                                                                   = 1699,
    /// MONOCHROME
    Monochrome                                                             = 2336,
    /// MONOTONOUS
    Monotonous                                                             = 2435,
    /// MOST-SIGNIFICANT-BYTE-FIRST
    MostSignificantByteFirst                                               = 1981,
    /// MOST-SIGNIFICANT-BYTE-LAST
    MostSignificantByteLast                                                = 1483,
    /// MR
    Mr                                                                     = 1100,
    /// MS
    Ms                                                                     = 2529,
    /// MT
    Mt                                                                     = 1431,
    /// MULTICORE-REENTRANT
    MulticoreReentrant                                                     = 129,
    /// MULTILANGUAGE-REFERRABLE
    MultilanguageReferrable                                                = 278,
    /// MULTIPLE
    Multiple                                                               = 1627,
    /// MULTIPLE-OCCURRENCES
    MultipleOccurrences                                                    = 2411,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 1991,
    /// MY
    My                                                                     = 372,
    /// N-PDU
    NPdu                                                                   = 345,
    /// NA
    Na                                                                     = 1057,
    /// NAND
    Nand                                                                   = 135,
    /// NE
    Ne                                                                     = 1416,
    /// NET
    Net                                                                    = 642,
    /// NETWORK
    Network                                                                = 1990,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 2514,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 1446,
    /// NETWORK-HANDLE-PORT-MAPPING
    NetworkHandlePortMapping                                               = 2441,
    /// NETWORK-MANAGEMENT-PORT-INTERFACE
    NetworkManagementPortInterface                                         = 1229,
    /// NETWORK-REPRESENTATION-FROM-COM-SPEC
    NetworkRepresentationFromComSpec                                       = 402,
    /// NEVER
    Never                                                                  = 2269,
    /// NEW-IS-DIFFERENT
    NewIsDifferent                                                         = 309,
    /// NEW-IS-EQUAL
    NewIsEqual                                                             = 1,
    /// NEW-IS-GREATER
    NewIsGreater                                                           = 1040,
    /// NEW-IS-GREATER-OR-EQUAL
    NewIsGreaterOrEqual                                                    = 1137,
    /// NEW-IS-LESS
    NewIsLess                                                              = 2620,
    /// NEW-IS-LESS-OR-EQUAL
    NewIsLessOrEqual                                                       = 2542,
    /// NEW-IS-OUTSIDE
    NewIsOutside                                                           = 511,
    /// NEW-IS-WITHIN
    NewIsWithin                                                            = 2177,
    /// NEWLINE
    Newline                                                                = 1131,
    /// NEWLINE-IF-NECESSARY
    NewlineIfNecessary                                                     = 161,
    /// NFOLD
    Nfold                                                                  = 962,
    /// NL
    Nl                                                                     = 846,
    /// NM-CLUSTER
    NmCluster                                                              = 334,
    /// NM-CONFIG
    NmConfig                                                               = 1807,
    /// NM-ECU
    NmEcu                                                                  = 782,
    /// NM-HANDLE-ACTIVE-TO-FUNCTION-GROUP-STATE
    NmHandleActiveToFunctionGroupState                                     = 852,
    /// NM-HANDLE-INACTIVE-TO-FUNCTION-GROUP-STATE
    NmHandleInactiveToFunctionGroupState                                   = 1058,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 167,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 1126,
    /// NM-INTERACTS-WITH-SM-MAPPING
    NmInteractsWithSmMapping                                               = 2612,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 2263,
    /// NM-NODE
    NmNode                                                                 = 2600,
    /// NM-PDU
    NmPdu                                                                  = 2216,
    /// NO
    No                                                                     = 646,
    /// NO-ACK
    NoAck                                                                  = 1365,
    /// NO-AFFECT
    NoAffect                                                               = 2549,
    /// NO-BOOT
    NoBoot                                                                 = 2609,
    /// NO-BREAK
    NoBreak                                                                = 2054,
    /// NO-CHECKPOINT-SUPERVISION
    NoCheckpointSupervision                                                = 2105,
    /// NO-COM
    NoCom                                                                  = 1966,
    /// NO-CONSISTENCY-MECHANISM
    NoConsistencyMechanism                                                 = 2125,
    /// NO-DEFAULT
    NoDefault                                                              = 36,
    /// NO-FLOAT
    NoFloat                                                                = 1839,
    /// NO-HEADER
    NoHeader                                                               = 1253,
    /// NO-KEEP
    NoKeep                                                                 = 618,
    /// NO-MONOTONY
    NoMonotony                                                             = 2328,
    /// NO-NEWLINE
    NoNewline                                                              = 2469,
    /// NO-OBD-SUPPORT
    NoObdSupport                                                           = 2147,
    /// NO-PGWIDE
    NoPgwide                                                               = 1302,
    /// NO-PROTECTION
    NoProtection                                                           = 1163,
    /// NO-RETURN-VALUE-PROVIDED
    NoReturnValueProvided                                                  = 1899,
    /// NO-SHOW-ALIAS-NAME
    NoShowAliasName                                                        = 2255,
    /// NO-SHOW-CATEGORY
    NoShowCategory                                                         = 584,
    /// NO-SHOW-CONTENT
    NoShowContent                                                          = 2273,
    /// NO-SHOW-LONG-NAME
    NoShowLongName                                                         = 617,
    /// NO-SHOW-NUMBER
    NoShowNumber                                                           = 1254,
    /// NO-SHOW-PAGE
    NoShowPage                                                             = 2419,
    /// NO-SHOW-SEE
    NoShowSee                                                              = 1998,
    /// NO-SHOW-SHORT-NAME
    NoShowShortName                                                        = 1993,
    /// NO-SHOW-TYPE
    NoShowType                                                             = 811,
    /// NO-SLOPPY
    NoSloppy                                                               = 1890,
    /// NO-STATUS-BYTE-CHANGE
    NoStatusByteChange                                                     = 2042,
    /// NO-STORE-EVENT
    NoStoreEvent                                                           = 945,
    /// NO-SUPERVISION
    NoSupervision                                                          = 2515,
    /// NO-SUPPORT
    NoSupport                                                              = 2117,
    /// NO-TRANSFORMER-ERROR-HANDLING
    NoTransformerErrorHandling                                             = 394,
    /// NO-TRANSFORMER-STATUS-FORWARDING
    NoTransformerStatusForwarding                                          = 1642,
    /// NO-TRUSTED-PLATFORM-SUPPORT
    NoTrustedPlatformSupport                                               = 703,
    /// NODE
    Node                                                                   = 2554,
    /// NOHREF
    Nohref                                                                 = 1447,
    /// NON-EMMISSION-RELATED-DTC
    NonEmmissionRelatedDtc                                                 = 2142,
    /// NON-OS-MODULE-INSTANTIATION
    NonOsModuleInstantiation                                               = 2072,
    /// NON-REENTRANT
    NonReentrant                                                           = 200,
    /// NON-VOLATILE
    NonVolatile                                                            = 2274,
    /// NON-VOLATILE-RAM-MANAGER
    NonVolatileRamManager                                                  = 1538,
    /// NONE
    None                                                                   = 1233,
    /// NORMALFIXED
    Normalfixed                                                            = 1663,
    /// NOT
    Not                                                                    = 1109,
    /// NOT-ACCESSIBLE
    NotAccessible                                                          = 198,
    /// NOT-AVAILABLE
    NotAvailable                                                           = 314,
    /// NOT-DEFINED
    NotDefined                                                             = 43,
    /// NOT-EQUAL
    NotEqual                                                               = 518,
    /// NOT-SENT
    NotSent                                                                = 2651,
    /// NOT-TESTED
    NotTested                                                              = 347,
    /// NOT-VALID
    NotValid                                                               = 838,
    /// NOTHING
    Nothing                                                                = 1988,
    /// NOTIFICATION
    Notification                                                           = 1815,
    /// NTP--RFC-958
    NtpRfc958                                                              = 1133,
    /// NUMBER
    Number                                                                 = 515,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 1836,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 2203,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 2657,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 2116,
    /// NV-RAM-MANAGER
    NvRamManager                                                           = 1948,
    /// OBD
    Obd                                                                    = 1328,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 435,
    /// OBD-DCY
    ObdDcy                                                                 = 1017,
    /// OBD-DRIVING-CYCLE
    ObdDrivingCycle                                                        = 460,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 702,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 2332,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 598,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 1690,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 622,
    /// OBSERVER
    Observer                                                               = 2298,
    /// OBSERVER-BASED
    ObserverBased                                                          = 2389,
    /// OC
    Oc                                                                     = 1967,
    /// OCCURENCE
    Occurence                                                              = 2318,
    /// OEM-BOOT
    OemBoot                                                                = 2043,
    /// OEM-BOOT-RESP-APP
    OemBootRespApp                                                         = 1911,
    /// OFF
    Off                                                                    = 2305,
    /// OFFSET
    Offset                                                                 = 1194,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 1002,
    /// OM
    Om                                                                     = 2614,
    /// ON-CHANGE-OF-DATA-IDENTIFIER
    OnChangeOfDataIdentifier                                               = 863,
    /// ON-COMPARISON-OF-VALUES
    OnComparisonOfValues                                                   = 1668,
    /// ON-DTC-STATUS-CHANGE
    OnDtcStatusChange                                                      = 850,
    /// ON-ENTRY
    OnEntry                                                                = 529,
    /// ON-EXIT
    OnExit                                                                 = 5,
    /// ON-TRANSITION
    OnTransition                                                           = 144,
    /// ONE-EVERY-N
    OneEveryN                                                              = 2518,
    /// ONLY-THIS-CYCLE-AND-READINESS
    OnlyThisCycleAndReadiness                                              = 921,
    /// OPAQUE
    Opaque                                                                 = 1318,
    /// OPEN
    Open                                                                   = 677,
    /// OPERATING-SYSTEM
    OperatingSystem                                                        = 365,
    /// OPERATION-CALL-RECEIVED
    OperationCallReceived                                                  = 1419,
    /// OPERATION-CALL-RESPONSE-RECEIVED
    OperationCallResponseReceived                                          = 1490,
    /// OPERATION-CALL-RESPONSE-SENT
    OperationCallResponseSent                                              = 1662,
    /// OPERATION-CALLED
    OperationCalled                                                        = 2271,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 802,
    /// OPTIONS
    Options                                                                = 2507,
    /// OR
    Or                                                                     = 1047,
    /// ORDINARY-EOC
    OrdinaryEoc                                                            = 2381,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 12,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 733,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 2413,
    /// OTHER
    Other                                                                  = 1764,
    /// OUT
    Out                                                                    = 1061,
    /// OVERRIDE
    Override                                                               = 459,
    /// OVERWRITE
    Overwrite                                                              = 2605,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 3,
    /// PA
    Pa                                                                     = 1213,
    /// PACKAGEABLE-ELEMENT
    PackageableElement                                                     = 924,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 2330,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 764,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 1158,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 983,
    /// PARTIAL-NETWORK
    PartialNetwork                                                         = 28,
    /// PARTITION
    Partition                                                              = 2296,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 2260,
    /// PASSIVE
    Passive                                                                = 1170,
    /// PASSTHROUGH
    Passthrough                                                            = 148,
    /// PAYLOAD-AS-ARRAY
    PayloadAsArray                                                         = 1788,
    /// PAYLOAD-AS-POINTER-TO-ARRAY
    PayloadAsPointerToArray                                                = 1227,
    /// PC-AFFECTS-LT
    PcAffectsLt                                                            = 2528,
    /// PC-AFFECTS-LT-AND-PB
    PcAffectsLtAndPb                                                       = 759,
    /// PC-AFFECTS-PB
    PcAffectsPb                                                            = 284,
    /// PCM
    Pcm                                                                    = 2064,
    /// PDF
    Pdf                                                                    = 1848,
    /// PDU
    Pdu                                                                    = 1769,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 612,
    /// PDU-R
    PduR                                                                   = 1984,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 1898,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 1224,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 1261,
    /// PEER
    Peer                                                                   = 60,
    /// PENDING
    Pending                                                                = 724,
    /// PER-EXECUTABLE
    PerExecutable                                                          = 1625,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 2534,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 1819,
    /// PERIODIC-RATE-FAST
    PeriodicRateFast                                                       = 787,
    /// PERIODIC-RATE-MEDIUM
    PeriodicRateMedium                                                     = 1559,
    /// PERIODIC-RATE-SLOW
    PeriodicRateSlow                                                       = 1615,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 1579,
    /// PERSISTENCY-DEPLOYMENT
    PersistencyDeployment                                                  = 1856,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT
    PersistencyDeploymentElement                                           = 1072,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 2184,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 1043,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 2169,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 2472,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 2249,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 1273,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 2453,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 482,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 2159,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 2570,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 1046,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 1506,
    /// PERSISTENCY-INTERFACE
    PersistencyInterface                                                   = 2430,
    /// PERSISTENCY-INTERFACE-ELEMENT
    PersistencyInterfaceElement                                            = 412,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 11,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 2538,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 1294,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 2623,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 715,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING
    PersistencyPortPrototypeToDeploymentMapping                            = 2487,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 215,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 1947,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 2074,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 185,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE
    PersistencyRedundancyHandlingScopeDatabase                             = 1147,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT
    PersistencyRedundancyHandlingScopeElement                              = 177,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE
    PersistencyRedundancyHandlingScopeFile                                 = 1758,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY
    PersistencyRedundancyHandlingScopeKey                                  = 387,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE
    PersistencyRedundancyHandlingScopeStorage                              = 1859,
    /// PERSISTENT
    Persistent                                                             = 2587,
    /// PGWIDE
    Pgwide                                                                 = 2544,
    /// PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE
    PhmAbstractRecoveryNotificationInterface                               = 1875,
    /// PHM-ACTION
    PhmAction                                                              = 73,
    /// PHM-ACTION-ITEM
    PhmActionItem                                                          = 505,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 2172,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 2545,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 232,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 508,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 344,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 939,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 936,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 81,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 2272,
    /// PHM-RULE
    PhmRule                                                                = 539,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 2520,
    /// PHM-SUPERVISION
    PhmSupervision                                                         = 1994,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 2243,
    /// PHYSICAL
    Physical                                                               = 1794,
    /// PHYSICAL-ADDRESS
    PhysicalAddress                                                        = 1845,
    /// PHYSICAL-CAN-FD
    PhysicalCanFd                                                          = 1454,
    /// PHYSICAL-CHANNEL
    PhysicalChannel                                                        = 2559,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 388,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 123,
    /// PL
    Pl                                                                     = 2096,
    /// PLAIN
    Plain                                                                  = 1813,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 1392,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 696,
    /// PLATFORM-HEALTH-MANAGEMENT-INTERFACE
    PlatformHealthManagementInterface                                      = 1299,
    /// PLATFORM-MODULE-ENDPOINT-CONFIGURATION
    PlatformModuleEndpointConfiguration                                    = 1785,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 775,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 1413,
    /// PNC-MAPPING-IDENT
    PncMappingIdent                                                        = 1433,
    /// PNG
    Png                                                                    = 1983,
    /// POLY
    Poly                                                                   = 2467,
    /// PORT
    Port                                                                   = 1509,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 2588,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 297,
    /// PORT-GROUP
    PortGroup                                                              = 2689,
    /// PORT-INTERFACE
    PortInterface                                                          = 684,
    /// PORT-INTERFACE-DEFINITION
    PortInterfaceDefinition                                                = 2088,
    /// PORT-INTERFACE-MAPPING
    PortInterfaceMapping                                                   = 2463,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 1694,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 1913,
    /// PORT-PROTOTYPE
    PortPrototype                                                          = 427,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 2539,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 2123,
    /// POST
    Post                                                                   = 530,
    /// POST-BUILD
    PostBuild                                                              = 44,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 283,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 2085,
    /// POWER
    Power                                                                  = 987,
    /// POWER-WINDOW-TIME
    PowerWindowTime                                                        = 424,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 2285,
    /// PRE--R-4--2
    PreR4_2                                                                = 1824,
    /// PRE-COMPILE
    PreCompile                                                             = 2476,
    /// PRE-COMPILE-TIME
    PreCompileTime                                                         = 2178,
    /// PRECONFIGURED-CONFIGURATION
    PreconfiguredConfiguration                                             = 30,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 112,
    /// PRESENTATION-CONTINUOUS
    PresentationContinuous                                                 = 2427,
    /// PRESENTATION-DISCRETE
    PresentationDiscrete                                                   = 2210,
    /// PRESHARED-KEY-IDENTITY
    PresharedKeyIdentity                                                   = 1311,
    /// PRIMARY-ECU
    PrimaryEcu                                                             = 2668,
    /// PRIMITIVE
    Primitive                                                              = 2677,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 1689,
    /// PRIO-OCC
    PrioOcc                                                                = 1384,
    /// PRIVATE-KEY
    PrivateKey                                                             = 1289,
    /// PROCESS
    Process                                                                = 2108,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 2630,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 2322,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 1733,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 1485,
    /// PROCESS-IS-NOT-SELF-TERMINATING
    ProcessIsNotSelfTerminating                                            = 1359,
    /// PROCESS-IS-SELF-TERMINATING
    ProcessIsSelfTerminating                                               = 2480,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 2506,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 1228,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 548,
    /// PROCESSING-STYLE-ASYNCHRONOUS
    ProcessingStyleAsynchronous                                            = 2199,
    /// PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR
    ProcessingStyleAsynchronousWithError                                   = 1608,
    /// PROCESSING-STYLE-SYNCHRONOUS
    ProcessingStyleSynchronous                                             = 300,
    /// PROCESSOR
    Processor                                                              = 1285,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 420,
    /// PRODUCER
    Producer                                                               = 1717,
    /// PROTECT-LAMP
    ProtectLamp                                                            = 421,
    /// PROTECTED
    Protected                                                              = 29,
    /// PROVIDED-AP-SERVICE-INSTANCE
    ProvidedApServiceInstance                                              = 1060,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 1683,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 2562,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 1399,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 1761,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 101,
    /// PS
    Ps                                                                     = 1408,
    /// PSK
    Psk                                                                    = 494,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 773,
    /// PT
    Pt                                                                     = 119,
    /// PTP--IEEE-1588--2002
    PtpIeee1588_2002                                                       = 1989,
    /// PTP--IEEE-1588--2008
    PtpIeee1588_2008                                                       = 1451,
    /// PUBLIC-KEY
    PublicKey                                                              = 2165,
    /// PUBLISHED-INFORMATION
    PublishedInformation                                                   = 1687,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 2585,
    /// PUT
    Put                                                                    = 819,
    /// QU
    Qu                                                                     = 873,
    /// QUEUED
    Queued                                                                 = 2512,
    /// R-4--2
    R4_2                                                                   = 2207,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 107,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 1209,
    /// RAW
    Raw                                                                    = 1458,
    /// RAW-DATA
    RawData                                                                = 832,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 2133,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 663,
    /// RAW-DATA-STREAM-GRANT
    RawDataStreamGrant                                                     = 923,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 791,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 2333,
    /// RAW-DATA-STREAM-MAPPING
    RawDataStreamMapping                                                   = 1264,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 340,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 2647,
    /// REACTION
    Reaction                                                               = 2674,
    /// READ
    Read                                                                   = 1550,
    /// READ-ONLY
    ReadOnly                                                               = 484,
    /// READ-WRITE
    ReadWrite                                                              = 2546,
    /// REBOOT
    Reboot                                                                 = 926,
    /// RECOMMENDED-CONFIGURATION
    RecommendedConfiguration                                               = 1315,
    /// RECORD-VALUE-FIELD
    RecordValueField                                                       = 2208,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 1090,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 358,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 1996,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 1376,
    /// RECT
    Rect                                                                   = 78,
    /// RED-STOP-LAMP
    RedStopLamp                                                            = 766,
    /// REDUNDANT
    Redundant                                                              = 383,
    /// REDUNDANT-PER-ELEMENT
    RedundantPerElement                                                    = 1243,
    /// REDUNDANT-PER-KEY
    RedundantPerKey                                                        = 1854,
    /// REF-ALL
    RefAll                                                                 = 478,
    /// REF-NON-STANDARD
    RefNonStandard                                                         = 1276,
    /// REF-NONE
    RefNone                                                                = 1626,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 1641,
    /// REFERRABLE
    Referrable                                                             = 768,
    /// REGULAR
    Regular                                                                = 732,
    /// REJECT
    Reject                                                                 = 1199,
    /// RELIABLE
    Reliable                                                               = 1323,
    /// REMOVE
    Remove                                                                 = 108,
    /// REPETITIVE-EOC
    RepetitiveEoc                                                          = 500,
    /// REPLACE
    Replace                                                                = 59,
    /// REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE
    ReplaceByTimeoutSubstitutionValue                                      = 1104,
    /// REPORT
    Report                                                                 = 1630,
    /// REPORT-AFTER-INIT
    ReportAfterInit                                                        = 1238,
    /// REPORT-BEFORE-INIT
    ReportBeforeInit                                                       = 831,
    /// REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE
    ReportDtcRecordInformationOnDtcStatusChange                            = 2646,
    /// REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE
    ReportMostRecentDtcOnStatusChange                                      = 507,
    /// REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST
    ReportingInChronlogicalOrderOldestFirst                                = 166,
    /// REPORTS-EXECUTION-STATE
    ReportsExecutionState                                                  = 2530,
    /// REQUEST
    Request                                                                = 2606,
    /// REQUEST-CALLBACK-TYPE-MANUFACTURER
    RequestCallbackTypeManufacturer                                        = 2023,
    /// REQUEST-CALLBACK-TYPE-SUPPLIER
    RequestCallbackTypeSupplier                                            = 1910,
    /// REQUEST-NO-RETURN
    RequestNoReturn                                                        = 2335,
    /// REQUIRED-AP-SERVICE-INSTANCE
    RequiredApServiceInstance                                              = 1074,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 1467,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 1089,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 1923,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 1153,
    /// REQUIRES-CALLBACK-EXECUTION
    RequiresCallbackExecution                                              = 876,
    /// RES-AXIS
    ResAxis                                                                = 1710,
    /// RESET-ECU
    ResetEcu                                                               = 1449,
    /// RESET-MACHINE
    ResetMachine                                                           = 2284,
    /// RESET-MCU
    ResetMcu                                                               = 1925,
    /// RESET-VM
    ResetVm                                                                = 302,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 489,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 1921,
    /// RESPOND-AFTER-RESET
    RespondAfterReset                                                      = 1740,
    /// RESPOND-BEFORE-RESET
    RespondBeforeReset                                                     = 122,
    /// RESPONSE
    Response                                                               = 2618,
    /// RESPONSE-SYNCHRONIZATION
    ResponseSynchronization                                                = 324,
    /// REST-ABSTRACT-ENDPOINT
    RestAbstractEndpoint                                                   = 751,
    /// REST-ABSTRACT-NUMERICAL-PROPERTY-DEF
    RestAbstractNumericalPropertyDef                                       = 1440,
    /// REST-ABSTRACT-PROPERTY-DEF
    RestAbstractPropertyDef                                                = 1864,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 523,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 1049,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 97,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 2110,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 179,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 996,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 256,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 2624,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 2684,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 1277,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 1143,
    /// REST-PRIMITIVE-PROPERTY-DEF
    RestPrimitivePropertyDef                                               = 554,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 1204,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 465,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 2428,
    /// RESTART
    Restart                                                                = 524,
    /// RESTART-APPLICATION
    RestartApplication                                                     = 2471,
    /// RES_AXIS
    Resaxis                                                                = 1496,
    /// RETURN-ON-EVENT-CLEARED
    ReturnOnEventCleared                                                   = 534,
    /// RETURN-ON-EVENT-STOPPED
    ReturnOnEventStopped                                                   = 1444,
    /// RETURN-VALUE-PROVIDED
    ReturnValueProvided                                                    = 2297,
    /// RIGHT
    Right                                                                  = 1514,
    /// RM
    Rm                                                                     = 798,
    /// RN
    Rn                                                                     = 777,
    /// RO
    Ro                                                                     = 2492,
    /// ROLL-BACK
    RollBack                                                               = 2118,
    /// ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE
    RootSwClusterDesignComponentPrototype                                  = 2175,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 1495,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 393,
    /// ROTATE-180
    Rotate180                                                              = 813,
    /// ROTATE-180-LIMIT-TO-TEXT
    Rotate180LimitToText                                                   = 588,
    /// ROTATE-90-CCW
    Rotate90Ccw                                                            = 2190,
    /// ROTATE-90-CCW-FIT-TO-TEXT
    Rotate90CcwFitToText                                                   = 2266,
    /// ROTATE-90-CCW-LIMIT-TO-TEXT
    Rotate90CcwLimitToText                                                 = 2432,
    /// ROTATE-90-CW
    Rotate90Cw                                                             = 231,
    /// ROTATE-90-CW-FIT-TO-TEXT
    Rotate90CwFitToText                                                    = 808,
    /// ROTATE-90-CW-LIMIT-TO-TEXT
    Rotate90CwLimitToText                                                  = 682,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 2351,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 1073,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 274,
    /// ROUTER
    Router                                                                 = 708,
    /// ROUTER-ADVERTISEMENT
    RouterAdvertisement                                                    = 1018,
    /// RPT-COMPONENT
    RptComponent                                                           = 1010,
    /// RPT-CONTAINER
    RptContainer                                                           = 1753,
    /// RPT-ENABLER-RAM
    RptEnablerRam                                                          = 1087,
    /// RPT-ENABLER-RAM-AND-ROM
    RptEnablerRamAndRom                                                    = 1768,
    /// RPT-ENABLER-ROM
    RptEnablerRom                                                          = 1528,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 476,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 333,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 1236,
    /// RPT-LEVEL-1
    RptLevel1                                                              = 2444,
    /// RPT-LEVEL-2
    RptLevel2                                                              = 1011,
    /// RPT-LEVEL-3
    RptLevel3                                                              = 1513,
    /// RPT-PROFILE
    RptProfile                                                             = 125,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 2106,
    /// RSA
    Rsa                                                                    = 396,
    /// RTE-EVENT
    RteEvent                                                               = 1065,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 2001,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 2024,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 1738,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 593,
    /// RTPGE
    Rtpge                                                                  = 547,
    /// RU
    Ru                                                                     = 205,
    /// RULE
    Rule                                                                   = 2596,
    /// RUN-CONTINUOUS
    RunContinuous                                                          = 2573,
    /// RUN-ONCE
    RunOnce                                                                = 679,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 609,
    /// RUNNABLE-ENTITY-ACTIVATED
    RunnableEntityActivated                                                = 1039,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 1849,
    /// RUNNABLE-ENTITY-STARTED
    RunnableEntityStarted                                                  = 2114,
    /// RUNNABLE-ENTITY-TERMINATED
    RunnableEntityTerminated                                               = 1593,
    /// RUNNABLE-ENTITY-VARIABLE-ACCESS
    RunnableEntityVariableAccess                                           = 2122,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 2034,
    /// RW
    Rw                                                                     = 243,
    /// RX-TRIGGER
    RxTrigger                                                              = 836,
    /// SA
    Sa                                                                     = 2422,
    /// SAE-J-1939--73
    SaeJ1939_73                                                            = 1468,
    /// SAE-J-2012--DA
    SaeJ2012Da                                                             = 2150,
    /// SAFETY
    Safety                                                                 = 2132,
    /// SATURATE
    Saturate                                                               = 2578,
    /// SCHEDULE-VARIANT-1
    ScheduleVariant1                                                       = 1148,
    /// SCHEDULE-VARIANT-2
    ScheduleVariant2                                                       = 2595,
    /// SCHEDULE-VARIANT-3
    ScheduleVariant3                                                       = 2572,
    /// SCHEDULE-VARIANT-4
    ScheduleVariant4                                                       = 149,
    /// SCHEDULE-VARIANT-5
    ScheduleVariant5                                                       = 778,
    /// SCHEDULE-VARIANT-6
    ScheduleVariant6                                                       = 2218,
    /// SCHEDULE-VARIANT-7
    ScheduleVariant7                                                       = 964,
    /// SCHEDULED
    Scheduled                                                              = 1312,
    /// SD
    Sd                                                                     = 1775,
    /// SDG-ABSTRACT-FOREIGN-REFERENCE
    SdgAbstractForeignReference                                            = 572,
    /// SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE
    SdgAbstractPrimitiveAttribute                                          = 1067,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 2569,
    /// SDG-ATTRIBUTE
    SdgAttribute                                                           = 338,
    /// SDG-CAPTION
    SdgCaption                                                             = 1366,
    /// SDG-CLASS
    SdgClass                                                               = 52,
    /// SDG-DEF
    SdgDef                                                                 = 799,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 594,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 1830,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 2532,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 1019,
    /// SDG-REFERENCE
    SdgReference                                                           = 1664,
    /// SDG-TAILORING
    SdgTailoring                                                           = 2451,
    /// SEARCH-FOR-ALL
    SearchForAll                                                           = 13,
    /// SEARCH-FOR-ALL-INSTANCES
    SearchForAllInstances                                                  = 1005,
    /// SEARCH-FOR-ANY
    SearchForAny                                                           = 1476,
    /// SEARCH-FOR-ID
    SearchForId                                                            = 2348,
    /// SEARCH-FOR-SPECIFIC-INSTANCE
    SearchForSpecificInstance                                              = 869,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 298,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 65,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 2656,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 279,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 497,
    /// SECOND-TO-FIRST
    SecondToFirst                                                          = 638,
    /// SECONDARY-ECU
    SecondaryEcu                                                           = 315,
    /// SECRET-SEED
    SecretSeed                                                             = 2145,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 1033,
    /// SECURE-COM-PROPS
    SecureComProps                                                         = 142,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 752,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 2083,
    /// SECURE-COMMUNICATION-DEPLOYMENT
    SecureCommunicationDeployment                                          = 1363,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 2567,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 223,
    /// SECURE-ON-BOARD-COMMUNICATION
    SecureOnBoardCommunication                                             = 837,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 55,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 1274,
    /// SECURED-PDU-HEADER-08-BIT
    SecuredPduHeader08Bit                                                  = 2450,
    /// SECURED-PDU-HEADER-16-BIT
    SecuredPduHeader16Bit                                                  = 2167,
    /// SECURED-PDU-HEADER-32-BIT
    SecuredPduHeader32Bit                                                  = 174,
    /// SECURITY
    Security                                                               = 1410,
    /// SECURITY-EVENT-AGGREGATION-FILTER
    SecurityEventAggregationFilter                                         = 1083,
    /// SECURITY-EVENT-CONTEXT-MAPPING
    SecurityEventContextMapping                                            = 1884,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 1697,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 2352,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 513,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 1272,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 321,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 26,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 1621,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 153,
    /// SECURITY-EVENT-ONE-EVERY-N-FILTER
    SecurityEventOneEveryNFilter                                           = 1178,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 367,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 363,
    /// SECURITY-EVENT-STATE-FILTER
    SecurityEventStateFilter                                               = 1735,
    /// SECURITY-EVENT-THRESHOLD-FILTER
    SecurityEventThresholdFilter                                           = 966,
    /// SELECTED
    Selected                                                               = 1362,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 2503,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 1053,
    /// SENT-TAGGED
    SentTagged                                                             = 2616,
    /// SENT-UNTAGGED
    SentUntagged                                                           = 1330,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 1670,
    /// SERIALIZER
    Serializer                                                             = 124,
    /// SERVER-AUTHENTICATE
    ServerAuthenticate                                                     = 1346,
    /// SERVER-CALL-POINT
    ServerCallPoint                                                        = 1192,
    /// SERVER-DECRYPT
    ServerDecrypt                                                          = 1430,
    /// SERVER-ENCRYPT
    ServerEncrypt                                                          = 1798,
    /// SERVER-MAC-GENERATE
    ServerMacGenerate                                                      = 589,
    /// SERVER-MAC-VERIFY
    ServerMacVerify                                                        = 1831,
    /// SERVER-VERIFY
    ServerVerify                                                           = 2527,
    /// SERVICE-DISCOVERY
    ServiceDiscovery                                                       = 2579,
    /// SERVICE-EVENT-DEPLOYMENT
    ServiceEventDeployment                                                 = 1372,
    /// SERVICE-FIELD-DEPLOYMENT
    ServiceFieldDeployment                                                 = 2391,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 2055,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 1221,
    /// SERVICE-INSTANCE-TO-MACHINE-MAPPING
    ServiceInstanceToMachineMapping                                        = 816,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 2409,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 1552,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 695,
    /// SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToSwClusterDesignPortPrototypeMapping                   = 2217,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 1155,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 247,
    /// SERVICE-INTERFACE-DEPLOYMENT
    ServiceInterfaceDeployment                                             = 2291,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING
    ServiceInterfaceElementMapping                                         = 480,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 1448,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 984,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 99,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 1644,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 1606,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 77,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 2289,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 2163,
    /// SERVICE-METHOD-DEPLOYMENT
    ServiceMethodDeployment                                                = 1883,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 1590,
    /// SERVICE-ONLY
    ServiceOnly                                                            = 51,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 117,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 280,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 709,
    /// SESSION-HANDLING-ACTIVE
    SessionHandlingActive                                                  = 2536,
    /// SESSION-HANDLING-INACTIVE
    SessionHandlingInactive                                                = 2343,
    /// SETTER
    Setter                                                                 = 722,
    /// SG
    Sg                                                                     = 285,
    /// SH
    Sh                                                                     = 1881,
    /// SHARED
    Shared                                                                 = 613,
    /// SHORT-HEADER
    ShortHeader                                                            = 817,
    /// SHOW-ALIAS-NAME
    ShowAliasName                                                          = 27,
    /// SHOW-CATEGORY
    ShowCategory                                                           = 1970,
    /// SHOW-CONTENT
    ShowContent                                                            = 1818,
    /// SHOW-LONG-NAME
    ShowLongName                                                           = 1737,
    /// SHOW-NUMBER
    ShowNumber                                                             = 855,
    /// SHOW-PAGE
    ShowPage                                                               = 2650,
    /// SHOW-SEE
    ShowSee                                                                = 2576,
    /// SHOW-SHORT-NAME
    ShowShortName                                                          = 1205,
    /// SHOW-TYPE
    ShowType                                                               = 2652,
    /// SI
    Si                                                                     = 451,
    /// SIDES
    Sides                                                                  = 2224,
    /// SIGN
    Sign                                                                   = 963,
    /// SIGN-WITH-ORIGIN-AUTHENTICATION
    SignWithOriginAuthentication                                           = 1189,
    /// SIGNAL-BASED
    SignalBased                                                            = 805,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 1138,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 1343,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 102,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 1217,
    /// SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFireAndForgetMethodToISignalTriggeringMapping               = 1263,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 1038,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 2666,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 911,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 761,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 2300,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 979,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 1952,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 1568,
    /// SIGNATURE
    Signature                                                              = 546,
    /// SILENT
    Silent                                                                 = 1106,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 1543,
    /// SINGLE
    Single                                                                 = 1956,
    /// SINGLE-CORE-REENTRANT
    SingleCoreReentrant                                                    = 1556,
    /// SINGLE-LANGUAGE-REFERRABLE
    SingleLanguageReferrable                                               = 212,
    /// SINGLE-OCCURRENCE
    SingleOccurrence                                                       = 2442,
    /// SK
    Sk                                                                     = 955,
    /// SL
    Sl                                                                     = 2071,
    /// SLAVE
    Slave                                                                  = 611,
    /// SLOPPY
    Sloppy                                                                 = 47,
    /// SLOW-FLASHING-MODE
    SlowFlashingMode                                                       = 1853,
    /// SLP
    Slp                                                                    = 2597,
    /// SM
    Sm                                                                     = 1165,
    /// SM-INTERACTS-WITH-NM-MAPPING
    SmInteractsWithNmMapping                                               = 2162,
    /// SMPTE-338
    Smpte338                                                               = 1197,
    /// SN
    Sn                                                                     = 2416,
    /// SO
    So                                                                     = 1004,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 1939,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 2315,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 1941,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 1968,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 1741,
    /// SOFTWARE-ACTIVATION-DEPENDENCY
    SoftwareActivationDependency                                           = 1686,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 252,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 711,
    /// SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS
    SoftwareClusterDiagnosticDeploymentProps                               = 887,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 311,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 250,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 689,
    /// SOMEIP
    Someip                                                                 = 1329,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 1777,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 2265,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 194,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 2648,
    /// SOMEIP-FIELD
    SomeipField                                                            = 242,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 128,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 881,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 643,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 1062,
    /// SOMEIP-REMOTE-MULTICAST-CONFIG
    SomeipRemoteMulticastConfig                                            = 1713,
    /// SOMEIP-REMOTE-UNICAST-CONFIG
    SomeipRemoteUnicastConfig                                              = 1250,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 2426,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 1427,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 49,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 958,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 1520,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 456,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 2061,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 2037,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 86,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 273,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 735,
    /// SOVD-GATEWAY-INSTANTIATION
    SovdGatewayInstantiation                                               = 1650,
    /// SOVD-MODULE-INSTANTIATION
    SovdModuleInstantiation                                                = 624,
    /// SOVD-SERVER-INSTANTIATION
    SovdServerInstantiation                                                = 745,
    /// SPEC-ELEMENT-REFERENCE
    SpecElementReference                                                   = 1442,
    /// SPEC-ELEMENT-SCOPE
    SpecElementScope                                                       = 1591,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 2245,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 641,
    /// SQ
    Sq                                                                     = 1161,
    /// SR
    Sr                                                                     = 2629,
    /// SRGB
    Srgb                                                                   = 2073,
    /// SS
    Ss                                                                     = 1381,
    /// SSDP
    Ssdp                                                                   = 1319,
    /// ST
    St                                                                     = 94,
    /// STACK-USAGE
    StackUsage                                                             = 2477,
    /// STANDARD
    Standard                                                               = 1493,
    /// STANDARD-PORT
    StandardPort                                                           = 1258,
    /// START
    Start                                                                  = 1028,
    /// START-FROM-BEGINNING
    StartFromBeginning                                                     = 436,
    /// START-ON-BOOT
    StartOnBoot                                                            = 1501,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 1997,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 1041,
    /// STATE-DEPENDENT-FIREWALL
    StateDependentFirewall                                                 = 908,
    /// STATE-MANAGEMEN-PHM-ERROR-INTERFACE
    StateManagemenPhmErrorInterface                                        = 82,
    /// STATE-MANAGEMENT-ACTION-ITEM
    StateManagementActionItem                                              = 1211,
    /// STATE-MANAGEMENT-ACTION-LIST
    StateManagementActionList                                              = 986,
    /// STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE
    StateManagementDiagTriggerInterface                                    = 2681,
    /// STATE-MANAGEMENT-EM-ERROR-INTERFACE
    StateManagementEmErrorInterface                                        = 2690,
    /// STATE-MANAGEMENT-ERROR-INTERFACE
    StateManagementErrorInterface                                          = 2051,
    /// STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE
    StateManagementFunctionGroupSwitchNotificationInterface                = 2361,
    /// STATE-MANAGEMENT-MODULE-INSTANTIATION
    StateManagementModuleInstantiation                                     = 84,
    /// STATE-MANAGEMENT-NM-ACTION-ITEM
    StateManagementNmActionItem                                            = 288,
    /// STATE-MANAGEMENT-NOTIFICATION-INTERFACE
    StateManagementNotificationInterface                                   = 1287,
    /// STATE-MANAGEMENT-PORT-INTERFACE
    StateManagementPortInterface                                           = 2553,
    /// STATE-MANAGEMENT-REQUEST-ERROR
    StateManagementRequestError                                            = 522,
    /// STATE-MANAGEMENT-REQUEST-INTERFACE
    StateManagementRequestInterface                                        = 58,
    /// STATE-MANAGEMENT-REQUEST-TRIGGER
    StateManagementRequestTrigger                                          = 972,
    /// STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM
    StateManagementSetFunctionGroupStateActionItem                         = 2182,
    /// STATE-MANAGEMENT-SLEEP-ACTION-ITEM
    StateManagementSleepActionItem                                         = 1931,
    /// STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM
    StateManagementStateMachineActionItem                                  = 263,
    /// STATE-MANAGEMENT-STATE-NOTIFICATION
    StateManagementStateNotification                                       = 1418,
    /// STATE-MANAGEMENT-STATE-REQUEST
    StateManagementStateRequest                                            = 1477,
    /// STATE-MANAGEMENT-SYNC-ACTION-ITEM
    StateManagementSyncActionItem                                          = 349,
    /// STATE-MANAGEMENT-TRIGGER-INTERFACE
    StateManagementTriggerInterface                                        = 2084,
    /// STATIC-OR-DYNAMIC-PART-TRIGGER
    StaticOrDynamicPartTrigger                                             = 994,
    /// STATIC-PART-TRIGGER
    StaticPartTrigger                                                      = 1529,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 2030,
    /// STATUS-BIT-AGING-AND-DISPLACEMENT
    StatusBitAgingAndDisplacement                                          = 175,
    /// STATUS-BIT-NORMAL
    StatusBitNormal                                                        = 1282,
    /// STD
    Std                                                                    = 445,
    /// STD-AXIS
    StdAxis                                                                = 1122,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 1922,
    /// STD_AXIS
    Stdaxis                                                                = 688,
    /// STEADY
    Steady                                                                 = 2367,
    /// STIMULUS-SYNCHRONIZATION
    StimulusSynchronization                                                = 2029,
    /// STOP
    Stop                                                                   = 2087,
    /// STOP-TRIGGER
    StopTrigger                                                            = 2686,
    /// STORE-EVENT
    StoreEvent                                                             = 2242,
    /// STORE-PERSISTENTLY
    StorePersistently                                                      = 1016,
    /// STRICT-MODE
    StrictMode                                                             = 64,
    /// STRICT-MONOTONOUS
    StrictMonotonous                                                       = 1940,
    /// STRICT-PRIORITY
    StrictPriority                                                         = 1308,
    /// STRICTLY-DECREASING
    StrictlyDecreasing                                                     = 405,
    /// STRICTLY-INCREASING
    StrictlyIncreasing                                                     = 687,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 1811,
    /// SU
    Su                                                                     = 1553,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 404,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 683,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 2058,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 639,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 2047,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 1059,
    /// SUPPLIER
    Supplier                                                               = 1592,
    /// SUPPORTS-BUFFER-LOCKING
    SupportsBufferLocking                                                  = 2257,
    /// SV
    Sv                                                                     = 1558,
    /// SVG
    Svg                                                                    = 784,
    /// SW
    Sw                                                                     = 260,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 1255,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 226,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 268,
    /// SW-CALIBRATION-METHOD
    SwCalibrationMethod                                                    = 957,
    /// SW-CALPRM-PROTOTYPE
    SwCalprmPrototype                                                      = 915,
    /// SW-CLASS-ATTR-IMPL
    SwClassAttrImpl                                                        = 1620,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 651,
    /// SW-CLASS-PROTOTYPE
    SwClassPrototype                                                       = 2610,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 2488,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 1987,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 1796,
    /// SW-COMPONENT-TYPE
    SwComponentType                                                        = 2248,
    /// SW-CONNECTOR
    SwConnector                                                            = 1585,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 1961,
    /// SW-INSTANCE
    SwInstance                                                             = 680,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 559,
    /// SW-MC-FRAME
    SwMcFrame                                                              = 1344,
    /// SW-MC-INTERFACE
    SwMcInterface                                                          = 1705,
    /// SW-MC-INTERFACE-SOURCE
    SwMcInterfaceSource                                                    = 2202,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 1933,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 690,
    /// SW-SERVICE-PROTOTYPE
    SwServicePrototype                                                     = 196,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 2402,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 1455,
    /// SW-VARIABLE-PROTOTYPE
    SwVariablePrototype                                                    = 1256,
    /// SWC
    Swc                                                                    = 884,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 1081,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 1064,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 1551,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 403,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 531,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 790,
    /// SWC-TIMING
    SwcTiming                                                              = 519,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 1091,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 2301,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 769,
    /// SWITCH
    Switch                                                                 = 1134,
    /// SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY
    SwitchAsynchronousTrafficShaperGroupEntry                              = 1504,
    /// SWITCH-FLOW-METERING-ENTRY
    SwitchFlowMeteringEntry                                                = 1498,
    /// SWITCH-STREAM-FILTER-ACTION-DEST-PORT-MODIFICATION
    SwitchStreamFilterActionDestPortModification                           = 1068,
    /// SWITCH-STREAM-FILTER-ENTRY
    SwitchStreamFilterEntry                                                = 498,
    /// SWITCH-STREAM-FILTER-RULE
    SwitchStreamFilterRule                                                 = 1036,
    /// SWITCH-STREAM-GATE-ENTRY
    SwitchStreamGateEntry                                                  = 579,
    /// SWITCH-STREAM-IDENTIFICATION
    SwitchStreamIdentification                                             = 1999,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 1873,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 2598,
    /// SYMMETRIC
    Symmetric                                                              = 1673,
    /// SYMMETRIC-KEY
    SymmetricKey                                                           = 2286,
    /// SYNC-BASE-TIME-MANAGER
    SyncBaseTimeManager                                                    = 2634,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 2377,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 1751,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 860,
    /// SYNCHRONIZED
    Synchronized                                                           = 2017,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 844,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 2107,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 2383,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 2080,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 1266,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 2007,
    /// SYNCHRONOUS
    Synchronous                                                            = 671,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 2148,
    /// SYSTEM
    System                                                                 = 2659,
    /// SYSTEM-DESIGN-TIME
    SystemDesignTime                                                       = 2369,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 2468,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 2081,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 2484,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 2378,
    /// SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalGroupToCommunicationResourceMapping                        = 1712,
    /// SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalToCommunicationResourceMapping                             = 2005,
    /// SYSTEM-SUPPLIER-BOOT
    SystemSupplierBoot                                                     = 942,
    /// SYSTEM-SUPPLIER-BOOT-RESP-APP
    SystemSupplierBootRespApp                                              = 2387,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 2655,
    /// TA
    Ta                                                                     = 87,
    /// TARGET-CONTAINER
    TargetContainer                                                        = 1927,
    /// TASK
    Task                                                                   = 1135,
    /// TC
    Tc                                                                     = 1512,
    /// TCP
    Tcp                                                                    = 2508,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 2320,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 1286,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 1102,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 1398,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 2458,
    /// TD-EVENT-BSW
    TdEventBsw                                                             = 867,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 893,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 1168,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 1291,
    /// TD-EVENT-COM
    TdEventCom                                                             = 704,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 1296,
    /// TD-EVENT-CYCLE-START
    TdEventCycleStart                                                      = 351,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 1776,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 847,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 2495,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 1193,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 1835,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 1960,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 1934,
    /// TD-EVENT-SERVICE-INSTANCE
    TdEventServiceInstance                                                 = 636,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 1566,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 580,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 1459,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 933,
    /// TD-EVENT-SLLET
    TdEventSllet                                                           = 1182,
    /// TD-EVENT-SLLET-PORT
    TdEventSlletPort                                                       = 2371,
    /// TD-EVENT-SWC
    TdEventSwc                                                             = 127,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 1301,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 849,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 69,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 1425,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 2003,
    /// TD-EVENT-VFB
    TdEventVfb                                                             = 25,
    /// TD-EVENT-VFB-PORT
    TdEventVfbPort                                                         = 720,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 1340,
    /// TDLET-ZONE-CLOCK
    TdletZoneClock                                                         = 227,
    /// TE
    Te                                                                     = 1696,
    /// TERMINATE
    Terminate                                                              = 2679,
    /// TEST-FAILED
    TestFailed                                                             = 578,
    /// TEST-FAILED-BIT
    TestFailedBit                                                          = 1373,
    /// TEST-FAILED-THIS-OPERATION-CYCLE
    TestFailedThisOperationCycle                                           = 2115,
    /// TEST-PASSED
    TestPassed                                                             = 1773,
    /// TESTED
    Tested                                                                 = 934,
    /// TESTED-AND-FAILED
    TestedAndFailed                                                        = 2325,
    /// TG
    Tg                                                                     = 1891,
    /// TH
    Th                                                                     = 635,
    /// TI
    Ti                                                                     = 969,
    /// TIFF
    Tiff                                                                   = 2398,
    /// TIME
    Time                                                                   = 854,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 374,
    /// TIME-BASE-RESOURCE
    TimeBaseResource                                                       = 425,
    /// TIME-MASTER
    TimeMaster                                                             = 1916,
    /// TIME-SLAVE
    TimeSlave                                                              = 360,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 834,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 2185,
    /// TIME-SYNC-SERVER-CONFIGURATION
    TimeSyncServerConfiguration                                            = 568,
    /// TIME-SYNCHRONIZATION-INTERFACE
    TimeSynchronizationInterface                                           = 475,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 600,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 477,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 2220,
    /// TIMEOUT
    Timeout                                                                = 1007,
    /// TIMING-CLOCK
    TimingClock                                                            = 312,
    /// TIMING-CLOCK-SYNC-ACCURACY
    TimingClockSyncAccuracy                                                = 2068,
    /// TIMING-CONDITION
    TimingCondition                                                        = 982,
    /// TIMING-CONSTRAINT
    TimingConstraint                                                       = 1429,
    /// TIMING-DESCRIPTION
    TimingDescription                                                      = 1212,
    /// TIMING-DESCRIPTION-EVENT
    TimingDescriptionEvent                                                 = 2541,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 1404,
    /// TIMING-EVENT
    TimingEvent                                                            = 269,
    /// TIMING-EXTENSION
    TimingExtension                                                        = 1215,
    /// TIMING-EXTENSION-RESOURCE
    TimingExtensionResource                                                = 1535,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 266,
    /// TIP
    Tip                                                                    = 448,
    /// TK
    Tk                                                                     = 292,
    /// TL
    Tl                                                                     = 973,
    /// TLS-12
    Tls12                                                                  = 970,
    /// TLS-13
    Tls13                                                                  = 240,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 1657,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 1338,
    /// TLS-CRYPTO-CIPHER-SUITE-PROPS
    TlsCryptoCipherSuiteProps                                              = 1257,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 1850,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 1502,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 597,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 713,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 1128,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 93,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 89,
    /// TN
    Tn                                                                     = 1377,
    /// TO
    To                                                                     = 2594,
    /// TOP
    Top                                                                    = 1539,
    /// TOPBOT
    Topbot                                                                 = 4,
    /// TOPIC
    Topic                                                                  = 1578,
    /// TOPIC-1
    Topic1                                                                 = 1665,
    /// TOPIC-PREFIX
    TopicPrefix                                                            = 1270,
    /// TP-ADDRESS
    TpAddress                                                              = 1842,
    /// TP-CONFIG
    TpConfig                                                               = 1720,
    /// TP-CONNECTION-IDENT
    TpConnectionIdent                                                      = 2288,
    /// TR
    Tr                                                                     = 1507,
    /// TRACE
    Trace                                                                  = 666,
    /// TRACE-REFERRABLE
    TraceReferrable                                                        = 728,
    /// TRACE-SWITCH-ARTI
    TraceSwitchArti                                                        = 343,
    /// TRACE-SWITCH-ARTI-AND-LOG
    TraceSwitchArtiAndLog                                                  = 2134,
    /// TRACE-SWITCH-LOG
    TraceSwitchLog                                                         = 276,
    /// TRACE-SWITCH-NONE
    TraceSwitchNone                                                        = 1048,
    /// TRACEABLE
    Traceable                                                              = 1680,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 2380,
    /// TRACEABLE-TEXT
    TraceableText                                                          = 1391,
    /// TRACED-FAILURE
    TracedFailure                                                          = 1560,
    /// TRANSFER
    Transfer                                                               = 320,
    /// TRANSFORMATION-PROPS
    TransformationProps                                                    = 2445,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 1601,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 660,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 562,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 113,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 2212,
    /// TRANSFORMER-ERROR-HANDLING
    TransformerErrorHandling                                               = 120,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 2401,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 1656,
    /// TRANSFORMING-I-SIGNAL
    TransformingISignal                                                    = 2635,
    /// TRANSIENT
    Transient                                                              = 975,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 2498,
    /// TRANSIENT-LOCAL
    TransientLocal                                                         = 2119,
    /// TRANSLATION-START
    TranslationStart                                                       = 917,
    /// TRANSPORT
    Transport                                                              = 1313,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 602,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 627,
    /// TRAP
    Trap                                                                   = 815,
    /// TRIGGER
    Trigger                                                                = 797,
    /// TRIGGER-ACTIVATED
    TriggerActivated                                                       = 569,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 1821,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 1946,
    /// TRIGGER-RELEASED
    TriggerReleased                                                        = 974,
    /// TRIGGER-UNICAST
    TriggerUnicast                                                         = 657,
    /// TRIGGERED
    Triggered                                                              = 384,
    /// TRIGGERED-ON-CHANGE
    TriggeredOnChange                                                      = 2192,
    /// TRIGGERED-ON-CHANGE-WITHOUT-REPETITION
    TriggeredOnChangeWithoutRepetition                                     = 491,
    /// TRIGGERED-ON-EVALUATION
    TriggeredOnEvaluation                                                  = 1240,
    /// TRIGGERED-WITHOUT-REPETITION
    TriggeredWithoutRepetition                                             = 1421,
    /// TRUE
    True                                                                   = 1763,
    /// TS
    Ts                                                                     = 249,
    /// TT
    Tt                                                                     = 1700,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 439,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 1817,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 493,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 2089,
    /// TUNNEL
    Tunnel                                                                 = 1604,
    /// TW
    Tw                                                                     = 1852,
    /// TX-REF-TRIGGER
    TxRefTrigger                                                           = 1288,
    /// TX-REF-TRIGGER-GAP
    TxRefTriggerGap                                                        = 2366,
    /// TX-TRIGGER-MERGED
    TxTriggerMerged                                                        = 2157,
    /// TX-TRIGGER-SINGLE
    TxTriggerSingle                                                        = 1208,
    /// UCM
    Ucm                                                                    = 1872,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 2504,
    /// UCM-MASTER
    UcmMaster                                                              = 45,
    /// UCM-MASTER-MODULE-INSTANTIATION
    UcmMasterModuleInstantiation                                           = 2521,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 1610,
    /// UCM-RETRY-STRATEGY
    UcmRetryStrategy                                                       = 1216,
    /// UCM-STEP
    UcmStep                                                                = 229,
    /// UCM-SUBORDINATE-MODULE-INSTANTIATION
    UcmSubordinateModuleInstantiation                                      = 2181,
    /// UCM-TO-TIME-BASE-RESOURCE-MAPPING
    UcmToTimeBaseResourceMapping                                           = 1097,
    /// UDP
    Udp                                                                    = 1739,
    /// UDP-CHECKSUM-DISABLED
    UdpChecksumDisabled                                                    = 2626,
    /// UDP-CHECKSUM-ENABLED
    UdpChecksumEnabled                                                     = 21,
    /// UDP-NM
    UdpNm                                                                  = 1876,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 121,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 1279,
    /// UDS
    Uds                                                                    = 1386,
    /// UK
    Uk                                                                     = 2410,
    /// UNDECIDED
    Undecided                                                              = 1985,
    /// UNDEFINED
    Undefined                                                              = 767,
    /// UNIT
    Unit                                                                   = 1393,
    /// UNIT-GROUP
    UnitGroup                                                              = 1120,
    /// UNNUMBER
    Unnumber                                                               = 629,
    /// UNSPECIFIED
    Unspecified                                                            = 1863,
    /// UP-LINK-PORT
    UpLinkPort                                                             = 1118,
    /// UPDATE
    Update                                                                 = 1752,
    /// UPLOADABLE-DEPLOYMENT-ELEMENT
    UploadableDeploymentElement                                            = 1778,
    /// UPLOADABLE-DESIGN-ELEMENT
    UploadableDesignElement                                                = 1333,
    /// UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT
    UploadableExclusivePackageElement                                      = 2424,
    /// UPLOADABLE-PACKAGE-ELEMENT
    UploadablePackageElement                                               = 792,
    /// UR
    Ur                                                                     = 1533,
    /// USE-ARGUMENT-TYPE
    UseArgumentType                                                        = 2012,
    /// USE-ARRAY-BASE-TYPE
    UseArrayBaseType                                                       = 1918,
    /// USE-FIRST-CONTEXT-DATA
    UseFirstContextData                                                    = 1828,
    /// USE-LAST-CONTEXT-DATA
    UseLastContextData                                                     = 1356,
    /// USE-VOID
    UseVoid                                                                = 1332,
    /// USER
    User                                                                   = 1181,
    /// USER-DEFINED
    UserDefined                                                            = 467,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 2186,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 95,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 701,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 329,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 1851,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 1112,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 2227,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 1619,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 1297,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 6,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 2516,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 313,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 306,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 239,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 781,
    /// USES-LOGGING
    UsesLogging                                                            = 1369,
    /// UZ
    Uz                                                                     = 2584,
    /// V-2-X-ACTIVE-SUPPORTED
    V2XActiveSupported                                                     = 2146,
    /// V-2-X-DATA-MANAGER-NEEDS
    V2XDataManagerNeeds                                                    = 2099,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 2443,
    /// V-2-X-FACILITIES
    V2XFacilities                                                          = 2345,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 1383,
    /// V-2-X-MANAGEMENT
    V2XManagement                                                          = 2502,
    /// V-2-X-NOT-SUPPORTED
    V2XNotSupported                                                        = 771,
    /// VALID
    Valid                                                                  = 1154,
    /// VAR
    Var                                                                    = 765,
    /// VAR-FAST
    VarFast                                                                = 1944,
    /// VAR-NO-INIT
    VarNoInit                                                              = 1774,
    /// VAR-POWER-ON-INIT
    VarPowerOnInit                                                         = 1727,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 631,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 90,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 1581,
    /// VARIABLE-DATA-PROTOTYPE-RECEIVED
    VariableDataPrototypeReceived                                          = 172,
    /// VARIABLE-DATA-PROTOTYPE-SENT
    VariableDataPrototypeSent                                              = 2628,
    /// VARIABLE-SIZE
    VariableSize                                                           = 440,
    /// VARIANT-LINK-TIME
    VariantLinkTime                                                        = 601,
    /// VARIANT-POST-BUILD
    VariantPostBuild                                                       = 490,
    /// VARIANT-POST-BUILD-LOADABLE
    VariantPostBuildLoadable                                               = 1555,
    /// VARIANT-POST-BUILD-SELECTABLE
    VariantPostBuildSelectable                                             = 502,
    /// VARIANT-PRE-COMPILE
    VariantPreCompile                                                      = 193,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 2129,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 2537,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 354,
    /// VENDOR
    Vendor                                                                 = 1117,
    /// VENDOR-SPECIFIC
    VendorSpecific                                                         = 825,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 2375,
    /// VERBOSE
    Verbose                                                                = 1942,
    /// VERIFICATION
    Verification                                                           = 496,
    /// VERIFY
    Verify                                                                 = 2295,
    /// VERSION-1
    Version1                                                               = 1494,
    /// VERTEX-OF-TARGET-CONTAINER
    VertexOfTargetContainer                                                = 2310,
    /// VFB-TIMING
    VfbTiming                                                              = 1781,
    /// VI
    Vi                                                                     = 705,
    /// VIDEO-FRAME
    VideoFrame                                                             = 895,
    /// VIDEO-LINE
    VideoLine                                                              = 2195,
    /// VIEW-MAP
    ViewMap                                                                = 1977,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 2613,
    /// VLAN-CONFIG
    VlanConfig                                                             = 1832,
    /// VO
    Vo                                                                     = 1218,
    /// VOLATILE
    Volatile                                                               = 2259,
    /// WAIT-FOR-VEHICLE-SAFE-STATE
    WaitForVehicleSafeState                                                = 1435,
    /// WAIT-POINT
    WaitPoint                                                              = 132,
    /// WAIT-TIME-DATE
    WaitTimeDate                                                           = 899,
    /// WARMUP
    Warmup                                                                 = 2494,
    /// WARN
    Warn                                                                   = 2563,
    /// WARNING
    Warning                                                                = 871,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 947,
    /// WATCH-DOG-MANAGER
    WatchDogManager                                                        = 2638,
    /// WATCH-TRIGGER
    WatchTrigger                                                           = 1527,
    /// WATCH-TRIGGER-GAP
    WatchTriggerGap                                                        = 2060,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 1691,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 744,
    /// WEIGHTED-ROUND-ROBIN
    WeightedRoundRobin                                                     = 840,
    /// WILL-CALL
    WillCall                                                               = 2036,
    /// WILL-RECEIVE
    WillReceive                                                            = 2011,
    /// WILL-SEND
    WillSend                                                               = 1201,
    /// WO
    Wo                                                                     = 2564,
    /// WONT-CALL
    WontCall                                                               = 2026,
    /// WONT-RECEIVE
    WontReceive                                                            = 1127,
    /// WONT-SEND
    WontSend                                                               = 2379,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 1732,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 342,
    /// WRITE
    Write                                                                  = 595,
    /// WRITE-ONLY
    WriteOnly                                                              = 190,
    /// WRONG-TRIGGER
    WrongTrigger                                                           = 517,
    /// X-509
    X509                                                                   = 845,
    /// X-MII
    XMii                                                                   = 1889,
    /// X-MMI
    XMmi                                                                   = 1150,
    /// XCP
    Xcp                                                                    = 417,
    /// XCP-PDU
    XcpPdu                                                                 = 1310,
    /// XDOC
    Xdoc                                                                   = 1765,
    /// XFILE
    Xfile                                                                  = 1783,
    /// XG-MII
    XgMii                                                                  = 1814,
    /// XH
    Xh                                                                     = 2198,
    /// XOR
    Xor                                                                    = 356,
    /// XREF-TARGET
    XrefTarget                                                             = 2101,
    /// XXG-MII
    XxgMii                                                                 = 2408,
    /// XYZ
    Xyz                                                                    = 1945,
    /// YCBCR
    Ycbcr                                                                  = 1698,
    /// YCGCO
    Ycgco                                                                  = 2354,
    /// YCM
    Ycm                                                                    = 756,
    /// YO
    Yo                                                                     = 1497,
    /// ZH
    Zh                                                                     = 2275,
    /// ZU
    Zu                                                                     = 542,
    /// default
    default                                                                = 2407,
    /// preserve
    preserve                                                               = 2270,
}

impl EnumItem {
    const STRING_TABLE: [&'static str; 2693] = ["DIAGNOSTIC-PROTOCOL", "NEW-IS-EQUAL", "IEEE802-11P", "P-PORT-PROTOTYPE", "TOPBOT", "ON-EXIT", "USER-DEFINED-METHOD-DEPLOYMENT", "DIAGNOSTIC-MAPPING", "CP", "LIN-MASTER", "ABSTRACT-CAN-COMMUNICATION-CONTROLLER", "PERSISTENCY-KEY-VALUE-DATABASE", "OS-MODULE-INSTANTIATION", "SEARCH-FOR-ALL", "COUPLING-ELEMENT-SWITCH-DETAILS", "CAN", "IS-EQUAL", "LOGIC-ADDRESS", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "BSW-MODULE-ENTITY", "COM-FIELD-GRANT-DESIGN", "UDP-CHECKSUM-ENABLED", "ADAPTIVE-FIELD-GETTER-COMPLETED", "INIT-EVENT", "GLOBAL-TIME-ETH-SLAVE", "TD-EVENT-VFB", "SECURITY-EVENT-DEFINITION", "SHOW-ALIAS-NAME", "PARTIAL-NETWORK", "PROTECTED", "PRECONFIGURED-CONFIGURATION", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "BINARY-MANIFEST-RESOURCE-DEFINITION", "DE", "DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER", "CYCLE-REPETITION-4", "NO-DEFAULT", "CAN-CLUSTER", "DROP", "ANY-PARTIAL-NETWORK-ACTIVE", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "DIAGNOSTIC-IUMPR", "NOT-DEFINED", "POST-BUILD", "UCM-MASTER", "CP-SOFTWARE-CLUSTER", "SLOPPY", "EQUAL", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "ACL-ROLE", "SERVICE-ONLY", "SDG-CLASS", "INSTRUCTION", "DATA-EXCHANGE-POINT", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "I-PDU-TRIGGERING", "STATE-MANAGEMENT-REQUEST-INTERFACE", "REPLACE", "PEER", "DEFAULT-TRIGGER", "HW-PIN-GROUP", "CPP", "STRICT-MODE", "SEC-OC-DEPLOYMENT", "DEFAULT-IF-REVISION-UPDATE", "1", "INOUT", "TD-EVENT-TRIGGER", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "GL", "CONSTANT-SPECIFICATION", "PHM-ACTION", "BUILD-ACTION-MANIFEST", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING", "SERVICE-INTERFACE-METHOD-MAPPING", "RECT", "DLT-CONTEXT", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "PHM-LOGICAL-EXPRESSION", "STATE-MANAGEMEN-PHM-ERROR-INTERFACE", "LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING", "STATE-MANAGEMENT-MODULE-INSTANTIATION", "CAN-TP-CHANNEL", "SOMEIP-TP-CHANNEL", "TA", "IMPLEMENTATION-PROPS", "TLV-DATA-ID-DEFINITION-SET", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "EVENT-WINDOW-INFINITE", "CAN-PHYSICAL-CHANNEL", "TLS-SECURE-COM-PROPS", "ST", "USER-DEFINED-COMMUNICATION-CONNECTOR", "ECU-INSTANCE", "REST-ELEMENT-DEF", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "SERVICE-INTERFACE-FIELD-MAPPING", "ASYMMETRIC-FROM-BYTE-ARRAY", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "SIGNAL-BASED-FIELD-DEPLOYMENT", "FLEXRAY-COMMUNICATION-CONNECTOR", "ATP-FEATURE", "HEALTH-CHANNEL-SUPERVISION", "COMM-CONNECTOR-PORT", "R-PORT-PROTOTYPE", "REMOVE", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "ASYNCHRONOUS-SERVER-CALL-POINT", "BO", "PREDEFINED-VARIANT", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "ECUC-CHOICE-CONTAINER-DEF", "EXCLUDE-FROM-FLASH", "CYCLE-REPETITION-10", "SERVICE-PROXY-SW-COMPONENT-TYPE", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "PT", "TRANSFORMER-ERROR-HANDLING", "UDP-NM-CLUSTER", "RESPOND-BEFORE-RESET", "PHYSICAL-DIMENSION-MAPPING-SET", "SERIALIZER", "RPT-PROFILE", "GENERAL-PURPOSE-I-PDU", "TD-EVENT-SWC", "SOMEIP-FIELD-DEPLOYMENT", "MULTICORE-REENTRANT", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "KEEP", "WAIT-POINT", "FIBEX-ELEMENT", "AUTO-IP--DOIP", "NAND", "J-1939-CLUSTER", "BLOCK", "GENERIC-MODULE-INSTANTIATION", "CRYPTO-SERVICE-CERTIFICATE", "ECU-MAPPING", "FUNCTIONAL", "SECURE-COM-PROPS", "GLOBAL-SUPERVISION-NEEDS", "ON-TRANSITION", "CO", "ASYNCHRONOUS", "GLOBAL-TIME-CAN-MASTER", "PASSTHROUGH", "SCHEDULE-VARIANT-4", "CRYPTO-JOB", "CYCLE-REPETITION-50", "BSW-TIMING-EVENT", "SECURITY-EVENT-MAPPING", "CHAPTER", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "J-1939-NM--SCA", "CANCEL-CAMPAIGN", "MODE-SWITCH-INTERFACE", "ADAPTIVE-FIREWALL-MODULE-INSTANTIATION", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "NEWLINE-IF-NECESSARY", "CRYPTO-KEY-SLOT-INTERFACE", "MAC-SEC-GLOBAL-KAY-PROPS", "ANY-SEND-OPERATION", "APPLICATION-ERROR", "REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "CRYPTO-SERVICE-PRIMITIVE", "ICV-NOT-VERIFIED", "DIAGNOSTIC-SOVD-METHOD-PRIMITIVE", "LIN-SLAVE-CONFIG-IDENT", "VARIABLE-DATA-PROTOTYPE-RECEIVED", "ERROR-CORRECTION", "SECURED-PDU-HEADER-32-BIT", "STATUS-BIT-AGING-AND-DISPLACEMENT", "BSW-DEBUG-INFO", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT", "ECUC-INSTANCE-REFERENCE-DEF", "REST-ENDPOINT-GET", "BREAK", "IDSM-PROPERTIES", "ASYMMETRIC-TO-BYTE-ARRAY", "DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE", "ABSTRACT-SECURITY-EVENT-FILTER", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "ACL-OPERATION", "CLIENT-AUTHENTICATE", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED", "LIN-TP-CONFIG", "WRITE-ONLY", "100", "CAPTURE-SYNCHRONOUSLY-TO-REPORTING", "VARIANT-PRE-COMPILE", "SOMEIP-EVENT-DEPLOYMENT", "KK", "SW-SERVICE-PROTOTYPE", "ICV-OPTIONAL", "NOT-ACCESSIBLE", "EXCLUSIVE-AREA-NESTING-ORDER", "NON-REENTRANT", "BSW-INTERNAL-BEHAVIOR", "DETAILED-BYPASSING-FILTERS", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "INTERNAL-TRIGGER-OCCURRED-EVENT", "RU", "APPLICATION-ACTION-ITEM", "COMMAND-LINE-SIMPLE-FORM", "CAN-COMMUNICATION-CONTROLLER", "JI", "KEY-STORAGE", "CONNECT", "SINGLE-LANGUAGE-REFERRABLE", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "IEEE-1722-TP-ACF-CAN", "ECUC-QUERY", "CONCRETE", "DIAGNOSTIC-TEST-RESULT", "AR", "DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE", "DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING", "SECURE-COMMUNICATION-PROPS-SET", "BE", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "SW-AXIS-TYPE", "TDLET-ZONE-CLOCK", "ACL-OBJECT-SET", "UCM-STEP", "J-1939-NM--NCA", "ROTATE-90-CW", "PHM-CHECKPOINT", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE", "ABSTRACT-PROVIDED-PORT-PROTOTYPE", "4-2-2", "4-4-4", "60", "LOGICAL-EXPRESSION", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "TLS-13", "HEALTH-CHANNEL", "SOMEIP-FIELD", "RW", "EVENT-TRIGGERING-CONSTRAINT", "ACCESS-PERMISSION-SERVICE-INSTANCE", "IMPLEMENTATION", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "TS", "SOFTWARE-PACKAGE", "GLOBAL-SUPERVISION-ENTITY", "SOFTWARE-CLUSTER", "FLEXRAY-AR-TP-NODE", "COUPLING-ELEMENT-ABSTRACT-DETAILS", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "REST-ENDPOINT-PUT", "DIAGNOSTIC-FIM-ALIAS-EVENT", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "DIAGNOSTIC-SOVD-METHOD", "SW", "FRAME-ETHERNET-SENT-ON-BUS", "ETHERNET-RAW-DATA-STREAM-GRANT", "STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM", "CAN-20", "MANUFACTURING", "TIMING-MODE-INSTANCE", "DIAGNOSTIC-ACCESS-PERMISSION", "SW-BASE-TYPE", "TIMING-EVENT", "ECUC-CONTAINER-DEF", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "GN", "SOMEIP-TP-CONFIG", "ROUGH-ESTIMATE-STACK-USAGE", "DEVELOPMENT", "TRACE-SWITCH-LOG", "FM-FEATURE-MAP", "MULTILANGUAGE-REFERRABLE", "SEC-OC-JOB-REQUIREMENT", "SERVICE-SW-COMPONENT-TYPE", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "KEYWORD", "POST-BUILD-VARIANT-CRITERION", "PC-AFFECTS-PB", "SG", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "BT-REC-709", "STATE-MANAGEMENT-NM-ACTION-ITEM", "CONCRETE-CLASS-TAILORING", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "BSW-DIRECT-CALL-POINT", "TK", "FLEXRAY-CLUSTER", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "DIAGNOSTIC-SOVD-PORT-INTERFACE", "FULL-DUPLEX-MODE", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "SEC-OC-CRYPTO-SERVICE-MAPPING", "4-1-1", "PROCESSING-STYLE-SYNCHRONOUS", "GENERAL-PURPOSE-CONNECTION", "RESET-VM", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "BOLDITALIC", "DEFAULT", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "DELEGATION-SW-CONNECTOR", "APPLICATION-ENDPOINT", "NEW-IS-DIFFERENT", "ADAPTIVE-SERVICE-FIND-COMPLETED", "SOFTWARE-CLUSTER-REQUIREMENT", "TIMING-CLOCK", "USER-DEFINED-PHYSICAL-CHANNEL", "NOT-AVAILABLE", "SECONDARY-ECU", "CAUTION", "50", "BLINK-OR-CONTINUOUS-ON-MODE", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "TRANSFER", "SECURITY-EVENT-CONTEXT-PROPS", "GLOBAL-TIME-MASTER", "FM-FEATURE-SELECTION-SET", "RESPONSE-SYNCHRONIZATION", "ERROR-TRACER-NEEDS", "EVAP", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "HEALTH-CHANNEL-EXTERNAL-STATUS", "USER-DEFINED-ETHERNET-FRAME", "DEBOUNCE-DATA", "APPLICATION-DEFERRED-DATA-TYPE", "EXERCISE", "RPT-EXECUTABLE-ENTITY-EVENT", "NM-CLUSTER", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "DIAGNOSTIC-STOP-ROUTINE", "ADAPTIVE-MODULE-INSTANTIATION", "SDG-ATTRIBUTE", "KEYWORD-SET", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "24", "WORST-CASE-STACK-USAGE", "TRACE-SWITCH-ARTI", "PHM-HEALTH-CHANNEL-INTERFACE", "N-PDU", "BUS-MIRROR-CHANNEL-MAPPING", "NOT-TESTED", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "STATE-MANAGEMENT-SYNC-ACTION-ITEM", "DDS-CP-PARTITION", "TD-EVENT-CYCLE-START", "DIAG-REQUEST", "APPLIED-STANDARD", "VEHICLE-ROLLOUT-STEP", "DIAGNOSTIC-DATA-BY-IDENTIFIER", "XOR", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "I-SIGNAL-I-PDU", "TIME-SLAVE", "ALL-INDICES-DIFFERENT-ARRAY-SIZE", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "COM-FIND-SERVICE-GRANT-DESIGN", "OPERATING-SYSTEM", "DIAGNOSTIC-IO-CONTROL-NEEDS", "SECURITY-EVENT-REPORT-INTERFACE", "CRC-VALIDATED", "ECUC-BOOLEAN-PARAM-DEF", "100BASE-T1", "IMPLEMENTATION-DATA-TYPE", "MY", "ADAPTIVE-FIELD-SETTER-COMPLETED", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "IEEE-1722-TP-IIDC-CONNECTION", "BIDIRECTIONAL", "AUTO", "ICV-NOT-SUPPORTED", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "BAYER-GBRG", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "KM", "REDUNDANT", "TRIGGERED", "ACCEPT-CONFIGURED", "DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY", "PHYSICAL-DIMENSION", "DIAGNOSTIC-ABSTRACT-ALIAS-EVENT", "CANNOT-BE-REMOVED", "EPS", "AP", "ROOT-SW-COMPOSITION-PROTOTYPE", "NO-TRANSFORMER-ERROR-HANDLING", "INDICATOR-STATUS-NEEDS", "RSA", "COM-METHOD-GRANT-DESIGN", "APPLICATION-RECORD-DATA-TYPE", "FA", "MI", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "NETWORK-REPRESENTATION-FROM-COM-SPEC", "SWC-MODE-MANAGER-ERROR-EVENT", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "STRICTLY-DECREASING", "DEFAULT-TRACE-STATE-DISABLED", "GRANT-DESIGN", "DLNA", "LIN-SCHEDULE-TABLE", "IDENTIFIABLE", "DIAGNOSTIC-COM-CONTROL-CLASS", "PERSISTENCY-INTERFACE-ELEMENT", "AMBER-WARNING", "ACL-PERMISSION", "IS-STOPPED", "EN", "XCP", "ACCEPT-ALL", "ABSTRACT-RAW-DATA-STREAM-INTERFACE", "PROCESSOR-CORE", "PROTECT-LAMP", "ACTIVATE", "API-BASED", "POWER-WINDOW-TIME", "TIME-BASE-RESOURCE", "FRAME-TRIGGERING", "PORT-PROTOTYPE", "IEEE802-1AS", "CRYPTO-KEY-SLOT", "DOES-NOT-SUPPORT-BUFFER-LOCKING", "BOLD", "LIN-CLUSTER", "GENERIC-ETHERNET-FRAME", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "OBD-CONTROL-SERVICE-NEEDS", "START-FROM-BEGINNING", "BSW-MODULE-TIMING", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "TTCAN-CLUSTER", "VARIABLE-SIZE", "DIAGNOSTIC-EVENT-INTERFACE", "CRYPTO-KEY-MANAGEMENT-NEEDS", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "DIAGNOSTIC-CONNECTION", "STD", "IEEE-1722-TP-ETHERNET-FRAME", "CAPTION", "TIP", "BY-RECEPTION-TIMESTAMP", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "SI", "DIAGNOSTIC-J-1939-SPN-MAPPING", "FINISH", "J-1939-DCM-I-PDU", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "ECUC-FUNCTION-NAME-DEF", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "OVERRIDE", "OBD-DRIVING-CYCLE", "ITALIC", "BUILD-ACTION", "DIAGNOSTIC-ROUTINE-SUBFUNCTION", "AUTOSAR-DATA-PROTOTYPE", "REST-SERVICE-INTERFACE", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "USER-DEFINED", "DIAGNOSTIC-ROUTINE", "BLOCK-STATE", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "CRYPTO-SERVICE-MAPPING", "CODEGENERATION", "INCLUDE-BUT-DO-NOT-START", "8", "TIME-SYNCHRONIZATION-INTERFACE", "RPT-EXECUTABLE-ENTITY", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "REF-ALL", "JPG", "SERVICE-INTERFACE-ELEMENT-MAPPING", "IEEE-1722-TP-AV-CONNECTION", "PERSISTENCY-FILE-PROXY", "DIAGNOSTIC-ECU-RESET", "READ-ONLY", "CA", "ABSTRACT-CAN-CLUSTER", "ECC", "BSW-INTERRUPT-EVENT", "RESOURCE-CONSUMPTION", "VARIANT-POST-BUILD", "TRIGGERED-ON-CHANGE-WITHOUT-REPETITION", "CYCLE-REPETITION-8", "TTCAN-COMMUNICATION-CONTROLLER", "PSK", "FAST-FLASHING-MODE", "VERIFICATION", "SEC-OC-SECURE-COM-PROPS", "SWITCH-STREAM-FILTER-ENTRY", "4-2-2-4", "REPETITIVE-EOC", "AP-APPLICATION-ERROR", "VARIANT-POST-BUILD-SELECTABLE", "CLASS-CONTENT-CONDITIONAL", "ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF", "PHM-ACTION-ITEM", "MASKED-NEW-EQUALS-MASKED-OLD", "REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "BAYER-BGGR", "CLIENT-ID-DEFINITION", "NEW-IS-OUTSIDE", "DCM-I-PDU", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "IDS-COMMON-ELEMENT", "NUMBER", "DIAGNOSTIC-CONTRIBUTION-SET", "WRONG-TRIGGER", "NOT-EQUAL", "SWC-TIMING", "I-SIGNAL-TRIGGERING", "HINT", "STATE-MANAGEMENT-REQUEST-ERROR", "REST-ARRAY-PROPERTY-DEF", "RESTART", "FLEXRAY-FRAME-TRIGGERING", "LIN-SLAVE", "ECUC-REFERENCE-DEF", "DIAGNOSTIC-SECURITY-LEVEL", "ON-ENTRY", "POST", "SWC-MODE-SWITCH-EVENT", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "RETURN-ON-EVENT-CLEARED", "AS", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "CAN-TP-CONFIG", "CS", "PHM-RULE", "COLLECTION", "MODE-DECLARATION", "ZU", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "EXTENDED", "CLEAR-ALL-DTCS", "SIGNATURE", "RTPGE", "PROCESS-TO-MACHINE-MAPPING-SET", "IMPOSITION-TIME", "DYNAMIC", "BINARY-MANIFEST-RESOURCE", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "AA", "REST-PRIMITIVE-PROPERTY-DEF", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "DIAGNOSTIC-INDICATOR", "FORGET", "CRC-OPTIONAL", "SW-MC-BASE-TYPE", "BAYER-GRBG", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "JUSTIFY", "KO", "CRYPTO-SERVICE-MANAGER", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "TIME-SYNC-SERVER-CONFIGURATION", "TRIGGER-ACTIVATED", "MIDDLE", "ML", "SDG-ABSTRACT-FOREIGN-REFERENCE", "FIXED", "CRYPTO-MODULE-INSTANTIATION", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "CAN-BE-TERMINATED", "MK", "TEST-FAILED", "SWITCH-STREAM-GATE-ENTRY", "TD-EVENT-SERVICE-INSTANCE-EVENT", "DIAGNOSTIC-TROUBLE-CODE", "BEST-EFFORT", "MC-DATA-INSTANCE", "NO-SHOW-CATEGORY", "CONFIRMED-DTC-BIT", "DLT-ARGUMENT", "DIAGNOSTIC-START-ROUTINE", "ROTATE-180-LIMIT-TO-TEXT", "SERVER-MAC-GENERATE", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "FIELD", "ECUC-CONTAINER-VALUE", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "SDG-FOREIGN-REFERENCE", "WRITE", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "TLS-IAM-REMOTE-SUBJECT", "OBD-PID-SERVICE-NEEDS", "ETHERNET-FRAME", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "VARIANT-LINK-TIME", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "INT-24-BIT", "IEEE-1722-TP-RVF-CONNECTION", "BAYER-RGGB", "FRAME-ETHERNET-RECEIVED-ON-BUS", "MODE-DECLARATION-REQUESTED", "FM-FEATURE-MAP-ELEMENT", "RUNNABLE-ENTITY", "CURVE_AXIS", "SLAVE", "PDU-ACTIVATION-ROUTING-GROUP", "SHARED", "ALL-SUPPORTED-DTCS", "DATA-RECEIVED-EVENT", "DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING", "NO-SHOW-LONG-NAME", "NO-KEEP", "DEFAULT-ERROR-TRACER", "INTERPOLATION-ROUTINE-MAPPING-SET", "INTERFACE-MAPPING-SET", "OBD-RATIO-SERVICE-NEEDS", "LATENCY-TIMING-CONSTRAINT", "SOVD-MODULE-INSTANTIATION", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "LOWER-8-BIT", "UNNUMBER", "CRYPTO-ELLIPTIC-CURVE-PROPS", "VARIABLE-ACCESS", "ADAPTIVE-EVENT-SENT", "EVENT-MAPPING", "CONSISTENCY-NEEDS", "TH", "TD-EVENT-SERVICE-INSTANCE", "J-1939-DCM", "SECOND-TO-FIRST", "SUPERVISION-ENTITY", "COUPLING-ELEMENT", "SPORADIC-EVENT-TRIGGERING", "NET", "SOMEIP-METHOD-DEPLOYMENT", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "MODE-DECLARATION-SWITCH-COMPLETED", "NO", "KEY-DERIVATION", "DIAGNOSTIC-SECURE-CODING-MAPPING", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "ATP-STRUCTURE-ELEMENT", "SW-CLASS-INSTANCE", "DIAGNOSTIC-MULTIPLE-RESOURCE-PORT-MAPPING", "J-1939-CONTROLLER-APPLICATION", "DATA-SEND-COMPLETED-EVENT", "CRYPTO-SIGNATURE-SCHEME", "CAN-TP-NODE", "TRIGGER-UNICAST", "DEFICIT-ROUND-ROBIN", "BINARY-MANIFEST-ITEM", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "IGNITION", "FM-FEATURE", "RAW-DATA-STREAM-DEPLOYMENT", "FIELD-MAPPING", "COM-GRANT-DESIGN", "TRACE", "EXECUTABLE", "GRANT", "INTERRUPT", "COUPLING-PORT-CREDIT-BASED-SHAPER", "SYNCHRONOUS", "DLT-USER-NEEDS", "ISO-6", "GLOBAL-TIME-GATEWAY", "APPLICATION-COMPOSITE-DATA-TYPE", "BASE-TYPE", "OPEN", "DIAGNOSTIC-SERVICE-TABLE", "RUN-ONCE", "SW-INSTANCE", "FORWARD-AS-IS", "ROTATE-90-CW-LIMIT-TO-TEXT", "SUPERVISED-ENTITY-NEEDS", "PORT-INTERFACE", "DIAGNOSTIC-REQUEST-UPLOAD", "GENERAL-PURPOSE-PDU", "STRICTLY-INCREASING", "STD_AXIS", "SOFTWARE-PACKAGE-STEP", "SW-SERVICE-ARG", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "FIX_AXIS", "ECUC-DEFINITION-COLLECTION", "LEFT", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "IMMEDIATE", "DIAGNOSTIC-AGING", "AP-APPLICATION-ERROR-DOMAIN", "CRYPTO-NEED", "USER-DEFINED-COMMUNICATION-CONTROLLER", "OBD-INFO-SERVICE-NEEDS", "NO-TRUSTED-PLATFORM-SUPPORT", "TD-EVENT-COM", "VI", "MEMORY-USAGE", "CLIENT-ID-DEFINITION-SET", "ROUTER", "SERVICE-TIMING", "GENERAL-PARAMETER", "SOFTWARE-CLUSTER-DESIGN", "IEEE-1722-TP-ACF-CAN-PART", "TLS-JOB-MAPPING", "J-1939-NM--AAC", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "KEEP-LAST", "HIERARCHICAL-EOC", "J-1939-NM--CCA", "TD-EVENT-VFB-PORT", "DISABLE", "SETTER", "DEFERRED", "PENDING", "COMPILE", "KEEP-ALL", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING", "TRACE-REFERRABLE", "CANCEL", "IS-RELEVANT", "DLT-LOG-CHANNEL-DESIGN", "REGULAR", "OS-TASK-EXECUTION-EVENT", "FRAME-ETHERNET-RECEIVED-BY-IF", "SOMEIP-TRANSFORMATION-PROPS", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "CONSUMED-EVENT-GROUP", "APPLICATION-RECORD-ELEMENT", "EVAPPURGEFLOW", "DEF-ITEM", "DIAGNOSTIC-TRANSFER-EXIT", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "DONT-INVALIDATE", "WATCHDOG-PHM-ACTION-ITEM", "SOVD-SERVER-INSTANTIATION", "IP-SEC-IAM-REMOTE-SUBJECT", "FIRST-TO-SECOND", "DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE", "INTERRUPT-CAT-1", "KA", "REST-ABSTRACT-ENDPOINT", "SECURE-COM-PROPS-SET", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING", "ICV-IGNORED", "MG", "YCM", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "MASTER", "PC-AFFECTS-LT-AND-PB", "CLIENT-ENCRYPT", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "ADAPTIVE-AUTOSAR-APPLICATION", "CRYPTO-NEEDS", "PARAMETER-DATA-PROTOTYPE", "VAR", "RED-STOP-LAMP", "UNDEFINED", "REFERRABLE", "SWC-TO-IMPL-MAPPING", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "V-2-X-NOT-SUPPORTED", "I-SIGNAL-PORT", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "CUSTOM", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "RN", "SCHEDULE-VARIANT-5", "MASKED-NEW-DIFFERS-MASKED-OLD", "ABSTRACT-ACCESS-POINT", "USER-DEFINED-TRANSFORMATION-PROPS", "NM-ECU", "BSW-COMPOSITION-TIMING", "SVG", "4-4-4-4", "CONTINUOUS-ON-MODE", "PERIODIC-RATE-FAST", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "ABSTRACT-CLASS-TAILORING", "SWC-SERVICE-DEPENDENCY", "RAW-DATA-STREAM-GRANT-DESIGN", "UPLOADABLE-PACKAGE-ELEMENT", "ECUC-DESTINATION-URI-DEF-SET", "DIAGNOSTIC-INDICATOR-INTERFACE", "DSA", "FLAT-INSTANCE-DESCRIPTOR", "TRIGGER", "RM", "SDG-DEF", "LIN-FRAME-TRIGGERING", "IS-NOT-RELEVANT", "OPERATION-INVOKED-EVENT", "END-2-END-EVENT-PROTECTION-PROPS", "EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE", "SIGNAL-BASED", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "ROTATE-90-CW-FIT-TO-TEXT", "ENCRYPT-AND-SIGN", "BH", "NO-SHOW-TYPE", "BSW-MODULE-DEPENDENCY", "ROTATE-180", "EXECUTABLE-ENTITY", "TRAP", "SERVICE-INSTANCE-TO-MACHINE-MAPPING", "SHORT-HEADER", "DDS-CP-DOMAIN", "PUT", "BSW-M-ENTRY-CALLED", "INSTALL", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "CAN-XL-PROPS", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "VENDOR-SPECIFIC", "IA", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "IS", "ABSTRACT-REQUIRED-PORT-PROTOTYPE", "FM-FEATURE-RELATION", "REPORT-BEFORE-INIT", "RAW-DATA", "I-PDU-RECEIVED-BY-COM", "TIME-SYNC-MODULE-INSTANTIATION", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "RX-TRIGGER", "SECURE-ON-BOARD-COMMUNICATION", "NOT-VALID", "FR", "WEIGHTED-ROUND-ROBIN", "ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE", "INT-16-BIT", "SYNCHRONIZED-MASTER-TIME-BASE", "X-509", "NL", "TD-EVENT-FRAME", "HOST-PORT", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "ON-DTC-STATUS-CHANGE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE", "NM-HANDLE-ACTIVE-TO-FUNCTION-GROUP-STATE", "DEADLINE-SUPERVISION", "TIME", "SHOW-NUMBER", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "ARTIFACT-LOCATOR", "ET", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "SYNCHRONIZATION-TIMING-CONSTRAINT", "MODE-DECLARATION-MAPPING", "IDS-MAPPING", "ON-CHANGE-OF-DATA-IDENTIFIER", "ACTIVATION-AND-TRIGGER-UNICAST", "GLOBAL-TIME-DOMAIN", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "TD-EVENT-BSW", "BSW-OPERATION-INVOKED-EVENT", "SEARCH-FOR-SPECIFIC-INSTANCE", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING", "WARNING", "CONFIDENTIALITY-OFFSET--0", "QU", "DIAGNOSTIC-COMMON-ELEMENT", "INTRA-LET-EOC", "REQUIRES-CALLBACK-EXECUTION", "DIAGNOSTIC-CONDITION-INTERFACE", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "SOMEIP-METHOD", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "ANY", "SWC", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "HA", "SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS", "GIF", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "ECUC-LINKER-SYMBOL-DEF", "EVENT-STORAGE-ENABLED", "CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "DECREASING", "VIDEO-FRAME", "ECUC-ENUMERATION-PARAM-DEF", "ATP-DEFINITION", "100BASE-TX", "WAIT-TIME-DATE", "FDC-THRESHOLD", "DIAGNOSTIC-READ-DTC-INFORMATION", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "LINK", "BSW-MODULE-ENTRY", "BINARY-MANIFEST-REQUIRE-RESOURCE", "END-2-END-METHOD-PROTECTION-PROPS", "STATE-DEPENDENT-FIREWALL", "IEEE-1722-TP-CRF-CONNECTION", "ETHERNET-PHYSICAL-CHANNEL", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "FURTHER-ACTION-BYTE-NEEDS", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "MAC-MULTICAST-GROUP", "SW-CALPRM-PROTOTYPE", "DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING", "TRANSLATION-START", "MN", "IEEE-1722-TP-ACF-CONNECTION", "KS", "ONLY-THIS-CYCLE-AND-READINESS", "DATA-WRITE-COMPLETED-EVENT", "RAW-DATA-STREAM-GRANT", "PACKAGEABLE-ELEMENT", "IEEE-1722-TP-ACF-LIN", "REBOOT", "FIRST-CONTAINED-TRIGGER", "CRC-SUPPORTED", "ALTERNATING-8-BIT", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "IS-OK", "BSW-MODULE-CLIENT-SERVER-ENTRY", "TD-EVENT-SERVICE-INSTANCE-METHOD", "TESTED", "ETHERNET-COMMUNICATION-CONTROLLER", "PHM-HEALTH-CHANNEL-STATUS", "EXCLUSIVE-AREA", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "AGE-CONSTRAINT", "ARTIFACT-CHECKSUM", "SYSTEM-SUPPLIER-BOOT", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "CAN-BE-TERMINATED-AND-RESTARTED", "NO-STORE-EVENT", "CONSOLE", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "ADAPTIVE-METHOD-CALLED", "I-SIGNAL-TO-I-PDU-MAPPING", "LAST-MODE", "IS-LESS-THAN-OR-EQUAL", "DROP-FRAME", "CAN-FRAME", "CRYPTO-INTERFACE", "SK", "FULL", "SW-CALIBRATION-METHOD", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "DIAGNOSTIC-LOG-AND-TRACE", "BINARY-MANIFEST-PROVIDE-RESOURCE", "GD", "NFOLD", "SIGN", "SCHEDULE-VARIANT-7", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE", "SECURITY-EVENT-THRESHOLD-FILTER", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "APPLICATION-ASSOC-MAP-ELEMENT", "TI", "TLS-12", "85", "STATE-MANAGEMENT-REQUEST-TRIGGER", "TL", "TRIGGER-RELEASED", "TRANSIENT", "MIN", "LAST-FAILED", "ENHANCED", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "END-TO-END-PROTECTION", "MEMORY-SECTION", "TIMING-CONDITION", "PARAMETER-SW-COMPONENT-TYPE", "SERVICE-INTERFACE-EVENT-MAPPING", "CONFIDENTIALITY-OFFSET--30", "STATE-MANAGEMENT-ACTION-LIST", "POWER", "IEEE-1722-TP-CONNECTION", "96-KHZ", "ADAPTIVE-FIELD-NOTIFICATION-SENT", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "FM-FEATURE-SELECTION", "INFINITE-TIME-TO-RESPONSE", "STATIC-OR-DYNAMIC-PART-TRIGGER", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "REST-ENDPOINT-POST", "BASIC-SOFTWARE-MODE-MANAGER", "10BASE-T1S", "ATP-CLASSIFIER", "DDS-SERVICE", "ATP-BLUEPRINT", "OFFSET-TIMING-CONSTRAINT", "72", "SO", "SEARCH-FOR-ALL-INSTANCES", "DIAGNOSTIC-COM-CONTROL", "TIMEOUT", "AUDIO-SAMPLE", "CY", "RPT-COMPONENT", "RPT-LEVEL-2", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "MODE-DECLARATION-MAPPING-SET", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "LIMIT-TO-PAGE", "STORE-PERSISTENTLY", "OBD-DCY", "ROUTER-ADVERTISEMENT", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "FIRE-AND-FORGET-METHOD-MAPPING", "LOG-AND-TRACE-INSTANTIATION", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "ECUC-STRING-PARAM-DEF", "FIXED-SIZE", "EXECUTION-TIME-CONSTRAINT", "AFTERMARKET", "START", "DO-IP-SERVICE-NEEDS", "EID-USE-CONFIG-VALUE", "FLEXRAY-TP-PDU-POOL", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "SECTION-NAME-PREFIX", "MIXED", "ACTION-LIST", "SWITCH-STREAM-FILTER-RULE", "ECUC-VALUE-COLLECTION", "SIGNAL-BASED-METHOD-DEPLOYMENT", "RUNNABLE-ENTITY-ACTIVATED", "NEW-IS-GREATER", "STARTUP-CONFIG-SET", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "EXACT-OR-ANY-MINOR-VERSION", "PERSISTENCY-FILE-STORAGE", "OR", "TRACE-SWITCH-NONE", "REST-BOOLEAN-PROPERTY-DEF", "INTERNAL-TRIGGERING-POINT", "CONSUMED-SERVICE-INSTANCE", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "ENABLE", "LOGICAL-OR", "120", "NA", "NM-HANDLE-INACTIVE-TO-FUNCTION-GROUP-STATE", "SUPERVISION-MODE-CONDITION", "PROVIDED-AP-SERVICE-INSTANCE", "OUT", "SOMEIP-PROVIDED-EVENT-GROUP", "ECU-STATE-MGR-USER-NEEDS", "SWC-IMPLEMENTATION", "RTE-EVENT", "AY", "SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE", "SWITCH-STREAM-FILTER-ACTION-DEST-PORT-MODIFICATION", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "HIGH", "ETHERNET-RAW-DATA-STREAM-MAPPING", "PERSISTENCY-DEPLOYMENT-ELEMENT", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "REQUIRED-AP-SERVICE-INSTANCE", "EXPLICIT", "AFTERMAKET", "COM-METHOD-GRANT", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "COM_AXIS", "MODE-DECLARATION-SWITCH-INITIATED", "SWC-BSW-MAPPING", "DIAGNOSTIC-EVENT-INFO-NEEDS", "SECURITY-EVENT-AGGREGATION-FILTER", "CYCLE-REPETITION-20", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "BLINK-MODE", "RPT-ENABLER-RAM", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "RECOVERY-NOTIFICATION", "SWC-TO-APPLICATION-PARTITION-MAPPING", "INDIVIDUAL", "CONFIRMED", "16", "CLIENT-DECRYPT", "KU", "UCM-TO-TIME-BASE-RESOURCE-MAPPING", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "MR", "44-1-KHZ", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "INDICATE", "REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE", "CYCLE-REPETITION-16", "SILENT", "CYCLE-REPETITION-1", "EXECUTE", "NOT", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "DLT-MESSAGE", "USER-DEFINED-FIELD-DEPLOYMENT", "FI", "APPLICATION-ONLY", "EVENT-ACCEPTANCE-DISABLED", "ABSTRACT-IMPLEMENTATION-DATA-TYPE", "VENDOR", "UP-LINK-PORT", "CLIENT-VERIFY", "UNIT-GROUP", "ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT", "STD-AXIS", "INVALID", "API", "DDS-METHOD-DEPLOYMENT", "NM-INSTANTIATION", "WONT-RECEIVE", "TLS-JOB-REQUIREMENT", "DDS-CP-SERVICE-INSTANCE", "COLLECTABLE-ELEMENT", "NEWLINE", "ARGUMENT-DATA-PROTOTYPE", "NTP--RFC-958", "SWITCH", "TASK", "MEDIUM", "NEW-IS-GREATER-OR-EQUAL", "SIGNAL-BASED-EVENT-DEPLOYMENT", "ICV-VERIFIED", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "LISTEN", "DIAGNOSTIC-DE-AUTHENTICATION", "REST-OBJECT-REF", "ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING", "ACTIVE", "-500-MILES", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE", "SCHEDULE-VARIANT-1", "E-2-E-PROFILE-CONFIGURATION-SET", "X-MMI", "I-PDU", "BLUEPRINT-DERIVATION-TIME", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "VALID", "SERVICE-INTERFACE", "KEEP-EXISTING", "LTS-13", "PARAMETER-INTERFACE", "ABSTRACT-EVENT", "DO-IP-INSTANTIATION", "SQ", "DATA-TRANSFORMATION-SET", "NO-PROTECTION", "DDS-SIGNAL", "SM", "CRC-IGNORED", "24-KHZ", "TD-EVENT-BSW-MODE-DECLARATION", "CLEAR", "PASSIVE", "DIAGNOSTIC-VALUE-NEEDS", "HEAD", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "BSW-SCHEDULER-NAME-PREFIX", "ECUC-VALIDATION-CONDITION", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "ADDR-METHOD-SHORT-NAME", "SECURITY-EVENT-ONE-EVERY-N-FILTER", "IE", "CRYPTO-PROVIDER", "USER", "TD-EVENT-SLLET", "LOGICAL-SUPERVISION", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "AR-ELEMENT", "ATP-TYPE", "MAC-SEC-KAY-PARTICIPANT", "I-SIGNAL", "SIGN-WITH-ORIGIN-AUTHENTICATION", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "IDSM-MODULE-INSTANTIATION", "SERVER-CALL-POINT", "TD-EVENT-I-PDU", "OFFSET", "192-KHZ", "J-1939-NM--SVCA", "SMPTE-338", "DO-IP-TP-CONFIG", "REJECT", "BRIEF-BYPASSING-FILTERS", "WILL-SEND", "CRYPTO-KEY-MANAGEMENT", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "REST-RESOURCE-DEF", "SHOW-SHORT-NAME", "FILE", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "TX-TRIGGER-SINGLE", "RAPID-PROTOTYPING-SCENARIO", "LT-AFFECTS-PB", "STATE-MANAGEMENT-ACTION-ITEM", "TIMING-DESCRIPTION", "PA", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "TIMING-EXTENSION", "UCM-RETRY-STRATEGY", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "VO", "LN", "FLOAT", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "ACTION", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "PDU-TRIGGERING", "ABSTRACT-CAN-PHYSICAL-CHANNEL", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "PAYLOAD-AS-POINTER-TO-ARRAY", "PROCESS-TO-MACHINE-MAPPING", "NETWORK-MANAGEMENT-PORT-INTERFACE", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "LIN-PHYSICAL-CHANNEL", "CALPRM", "NONE", "MASKED-NEW-DIFFERS-X", "DHCPV-4", "RPT-EXECUTION-CONTEXT", "HU", "REPORT-AFTER-INIT", "ECUC-ABSTRACT-REFERENCE-DEF", "TRIGGERED-ON-EVALUATION", "LINK-LOCAL--DOIP", "INSTANCE-ID", "REDUNDANT-PER-ELEMENT", "DOES-NOT-REPORT-EXECUTION-STATE", "COUPLING-PORT", "ECUC-ADD-INFO-PARAM-DEF", "DIAGNOSTIC-UPLOAD-INTERFACE", "ALL", "I-PDU-SENT-TO-IF", "SOMEIP-REMOTE-UNICAST-CONFIG", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "FOR-ALL", "NO-HEADER", "NO-SHOW-NUMBER", "SW-ADDR-METHOD", "SW-VARIABLE-PROTOTYPE", "TLS-CRYPTO-CIPHER-SUITE-PROPS", "STANDARD-PORT", "I-SIGNAL-SENT-TO-COM", "I-PV-6-EXT-HEADER-FILTER-SET", "PDUR-I-PDU-GROUP", "DDS-REQUIRED-SERVICE-INSTANCE", "SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "RAW-DATA-STREAM-MAPPING", "DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS", "SYNCHRONIZED-TIME-BASE-PROVIDER", "HARDWARE-TEST-MANAGER", "APPLICATION", "IS-FAILED", "TOPIC-PREFIX", "LIN-FRAME", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "PERSISTENCY-FILE-ARRAY", "SECURED-I-PDU", "CALIBRATION-ONLINE", "REF-NON-STANDARD", "REST-NUMBER-PROPERTY-DEF", "BINARY-MANIFEST-ITEM-DEFINITION", "UDP-NM-NODE", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "BSW-SCHEDULABLE-ENTITY", "STATUS-BIT-NORMAL", "DIAGNOSTIC-SOVD-UPDATE", "COM-EVENT-GRANT", "PROCESSOR", "TCP-OPTION-FILTER-SET", "STATE-MANAGEMENT-NOTIFICATION-INTERFACE", "TX-REF-TRIGGER", "PRIVATE-KEY", "CALCULATED", "TD-EVENT-BSW-MODULE", "ADAPTIVE-METHOD-RESPONSE-SENT", "IGNORE", "PERSISTENCY-KEY-VALUE-PAIR", "DIAGNOSTIC-CLEAR-CONDITION", "TD-EVENT-COMPLEX", "USER-DEFINED-I-PDU", "CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING", "PLATFORM-HEALTH-MANAGEMENT-INTERFACE", "FUNCTIONAL-CAN-FD", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "NO-PGWIDE", "MACRO", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED", "DERIVED-FROM", "BSW-M-ENTRY-CALL-RETURNED", "ENABLED", "STRICT-PRIORITY", "ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED", "XCP-PDU", "PRESHARED-KEY-IDENTITY", "SCHEDULED", "TRANSPORT", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "RECOMMENDED-CONFIGURATION", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "BRIEF", "OPAQUE", "SSDP", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "ALL-16-BIT", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "RELIABLE", "CAT-1", "MODE-DECLARATION-GROUP-PROTOTYPE", "DIAGNOSTIC-RESPONSE-ON-EVENT", "CALIBRATION-VARIABLES", "OBD", "SOMEIP", "SENT-UNTAGGED", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "USE-VOID", "UPLOADABLE-DESIGN-ELEMENT", "DIAGNOSTIC-EVENT-MANAGER", "EVENT-COMBINATION-ON-RETRIEVAL", "DIAGNOSTIC-AUTH-ROLE", "CRYPTO-DRIVER", "TLS-CRYPTO-CIPHER-SUITE", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "TD-EVENT-VFB-REFERENCE", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "SW-MC-FRAME", "DZ", "SERVER-AUTHENTICATE", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "DETERMINISTIC-CLIENT", "DIAGNOSTIC-SECURITY-ACCESS", "DIAGNOSTIC-AUTHENTICATION", "BSW-MODE-SWITCH-EVENT", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "LIMIT-TO-TEXT", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING", "USE-LAST-CONTEXT-DATA", "BT-REC-601", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "PROCESS-IS-NOT-SELF-TERMINATING", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "I-4-G", "SELECTED", "SECURE-COMMUNICATION-DEPLOYMENT", "EDGE-NODE", "NO-ACK", "SDG-CAPTION", "COLOR-BLIND", "HW-ATTRIBUTE-LITERAL-DEF", "USES-LOGGING", "MODE-DECLARATION-GROUP", "DATA-INTERFACE", "SERVICE-EVENT-DEPLOYMENT", "TEST-FAILED-BIT", "BSW-MODULE-CALL-POINT", "IEEE-1722-TP-ACF-BUS", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "TN", "DEFLATE", "DIAGNOSTIC-DATA-IDENTIFIER", "CONST", "SS", "DEVELOPMENT-ERROR", "V-2-X-M-USER-NEEDS", "PRIO-OCC", "DO-IP-GID-NEEDS", "UDS", "CODE-GENERATION-TIME", "DIAGNOSTIC-IO-CONTROL-CLASS", "ECUC-FLOAT-PARAM-DEF", "ADAPTIVE-METHOD-RESPONSE-RECEIVED", "TRACEABLE-TEXT", "PLATFORM-ACTION-ITEM", "UNIT", "MAPPING-SCOPE-ECU", "176-4-KHZ", "DIAGNOSTIC-ROUTINE-CONTROL", "ABSTRACT-SERVICE-INSTANCE", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "I-PDU-PORT", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "AS-IS", "TIMING-DESCRIPTION-EVENT-CHAIN", "ACTIVATION-UNICAST", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "32-KHZ", "PS", "INFO", "SECURITY", "ECUC-ENUMERATION-LITERAL-DEF", "IS-LESS-OR-EQUAL", "PLATFORM-PHM-ACTION-ITEM", "ERROR-DETECTION", "IDS-DESIGN", "NE", "J-1939-NM-NODE", "STATE-MANAGEMENT-STATE-NOTIFICATION", "OPERATION-CALL-RECEIVED", "ATP-PROTOTYPE", "TRIGGERED-WITHOUT-REPETITION", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "LT", "FIREWALL-RULE", "TD-EVENT-TT-CAN-CYCLE-START", "DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "FLEXRAY-NM-NODE", "TIMING-CONSTRAINT", "SERVER-DECRYPT", "MT", "LOG-AND-TRACE-INTERFACE", "PNC-MAPPING-IDENT", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "WAIT-FOR-VEHICLE-SAFE-STATE", "IDS-MGR-NEEDS", "DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE", "EXCLUSIVE", "DEFINE-BY-IDENTIFIER", "REST-ABSTRACT-NUMERICAL-PROPERTY-DEF", "ALL-INDICES-SAME-ARRAY-SIZE", "SPEC-ELEMENT-REFERENCE", "FRAME-TRANSMITTED-ON-BUS", "RETURN-ON-EVENT-STOPPED", "8-KHZ", "NETWORK-ENDPOINT", "NOHREF", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "RESET-ECU", "10", "PTP--IEEE-1588--2008", "ETH-TCP-IP-ICMP-PROPS", "LIN-SPORADIC-FRAME", "PHYSICAL-CAN-FD", "SW-SYSTEMCONSTANT-VALUE-SET", "LOCAL-SUPERVISION", "KY", "RAW", "TD-EVENT-SERVICE-INSTANCE-FIELD", "ETHERNET-COMMUNICATION-CONNECTOR", "1-1-001", "CAT-2", "DIAGNOSTIC-SESSION", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "DEFAULT-IF-UNDEFINED", "ECU-PARTITION", "REQUIRED-DDS-SERVICE-INSTANCE", "SAE-J-1939--73", "DIAGNOSTIC-DATA-PORT-MAPPING", "DEPENDENCY-ON-ARTIFACT", "ARBITRATION", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "MODE-TRANSITION", "CYCLE-REPETITION-5", "INTRUSION-DETECTION-SECURITY-MANAGEMENT", "SEARCH-FOR-ANY", "STATE-MANAGEMENT-STATE-REQUEST", "ALIVE-SUPERVISION", "AUTHENTICATE", "ABSTRACT", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "COMMAND-LINE-SHORT-FORM", "MOST-SIGNIFICANT-BYTE-LAST", "CAPTURE-ASYNCHRONOUS-TO-REPORTING", "PROCESS-EXECUTION-ERROR", "MAPPING-SCOPE-PARTITION", "HOOK", "EID-USE-MAC", "AM", "OPERATION-CALL-RESPONSE-RECEIVED", "C", "CONTAINER-I-PDU", "STANDARD", "VERSION-1", "ROOT-SW-COMPONENT-PROTOTYPE", "RES_AXIS", "YO", "SWITCH-FLOW-METERING-ENTRY", "LINKER", "J-1939-NM-CLUSTER", "START-ON-BOOT", "TLS-DEPLOYMENT", "ECUC-MODULE-DEF", "SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY", "ADAPTIVE-PLATFORM-SERVICE-INSTANCE", "PERSISTENCY-FILE-STORAGE-INTERFACE", "TR", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "PORT", "COM-FIND-SERVICE-GRANT", "CAN-BE-REMOVED", "TC", "RPT-LEVEL-3", "RIGHT", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "COUPLING-PORT-FIFO", "DIAGNOSTIC-INDICATOR-NEEDS", "ADAPTIVE-METHOD-CALL-RECEIVED", "ECUC-MODULE-CONFIGURATION-VALUES", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "FULL-COM", "BSW-IMPLEMENTATION", "FM-FEATURE-MAP-CONDITION", "CRYPTO-SERVICE-KEY", "BA", "HY", "WATCH-TRIGGER", "RPT-ENABLER-ROM", "STATIC-PART-TRIGGER", "IS-NOT-EQUAL", "ITU-BT-2020", "DIAGNOSTIC-DATA-TRANSFER", "UR", "CAPTURE-ASYNCHRONOUSLY-TO-REPORTING", "TIMING-EXTENSION-RESOURCE", "ECUC-PARAMETER-DEF", "ECUC-MULTILINE-STRING-PARAM-DEF", "NON-VOLATILE-RAM-MANAGER", "TOP", "LINK-TIME", "ADAPTIVE-FIELD-GETTER-CALLED", "LAST-IS-BEST", "SIMULATED-EXECUTION-TIME", "COMMUNICATION-CLUSTER", "DO-IP-INTERFACE", "DIAGNOSTIC-MULTIPLE-RESOURCE-INTERFACE", "DO-NOT-INCLUDE", "FILTERED", "CALLOUT", "READ", "SWC-INTERNAL-BEHAVIOR", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "SU", "HUB", "VARIANT-POST-BUILD-LOADABLE", "SINGLE-CORE-REENTRANT", "15", "SV", "PERIODIC-RATE-MEDIUM", "TRACED-FAILURE", "30", "FLEXRAY-PHYSICAL-CHANNEL", "DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING", "CRYPTO-CERTIFICATE-INTERFACE", "AUTOMATIC", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "COMMUNICATION-CONNECTOR", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "CRYPTO-SERVICE-JOB-NEEDS", "FUNCTIONAL-ADDRESS", "FM-FEATURE-MODEL", "2", "DIAGNOSTIC-OPERATION-CYCLE", "MAX", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "IDSM-RATE-LIMITATION", "EXECUTABLE-GROUP", "TOPIC", "PERSISTENCY-DATA-ELEMENT", "EL", "VARIABLE-DATA-PROTOTYPE", "MODE-INTERFACE-MAPPING", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "LOGICAL-AND", "SW-CONNECTOR", "KEY-SERVER", "ALLOW", "BY-SOURCE-TIMESTAMP", "APPLICATION-ARRAY-ELEMENT", "SERVICE-NEEDS", "SPEC-ELEMENT-SCOPE", "SUPPLIER", "RUNNABLE-ENTITY-TERMINATED", "BULK-NV-DATA-DESCRIPTOR", "EVENT-ACCEPTANCE-ENABLED", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE", "GLOBAL-TIME-SLAVE", "IP-SEC-RULE", "IW", "TRANSFORMATION-PROPS-SET", "LOWER-12-BIT", "CVC", "TUNNEL", "DIAGNOSTIC-CONDITION-GROUP", "SERVICE-INTERFACE-MAPPING-SET", "CRYPTO-TRUST-MASTER-INTERFACE", "PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR", "300", "UCM-MODULE-INSTANTIATION", "J-1939-TP-NODE", "DIAGNOSTIC-J-1939-SW-MAPPING", "AUTOSAR-VARIABLE-INSTANCE", "IDENT-CAPTION", "PERIODIC-RATE-SLOW", "INTERRUPT-CAT-2", "88-2-KHZ", "ESP", "USER-DEFINED-GLOBAL-TIME-SLAVE", "SW-CLASS-ATTR-IMPL", "SECURITY-EVENT-FILTER-CHAIN", "BURST-PATTERN-EVENT-TRIGGERING", "INTER-LET-ONLY", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "PER-EXECUTABLE", "REF-NONE", "MULTIPLE", "EXTERNAL-TRIGGERING-POINT-IDENT", "DIAGNOSTIC-SOVD-CONFIGURATION", "REPORT", "CHANNEL-B", "COM-FIELD-GRANT", "BINARY-MANIFEST-META-DATA-FIELD", "CRYPTO-PRIMITIVE", "IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "5", "BONJOUR", "DIAGNOSTIC-CONTROL-DTC-SETTING", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "BUILD-ACTION-ENVIRONMENT", "REFERENCE-TAILORING", "NO-TRANSFORMER-STATUS-FORWARDING", "LIN-EVENT-TRIGGERED-FRAME", "SERVICE-INTERFACE-MAPPING", "FRAME", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "BSW-MODULE-ENTITY-STARTED", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "SOVD-GATEWAY-INSTANTIATION", "1-8", "12", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "DIAGNOSTIC-J-1939-SPN", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "TRANSFORMER-STATUS-FORWARDING", "TLS-CONNECTION-GROUP", "LIN-NM-CLUSTER", "CHANNEL-A", "GZIP", "DYNAMIC-PART-TRIGGER", "OPERATION-CALL-RESPONSE-SENT", "NORMALFIXED", "SDG-REFERENCE", "TOPIC-1", "DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING", "DO-IP-ACTIVATION-LINE-NEEDS", "ON-COMPARISON-OF-VALUES", "DEFAULT-TRACE-STATE-ENABLED", "SERIALIZATION-TECHNOLOGY", "LV", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "SYMMETRIC", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "BN", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "CRYPTO-PROVIDER-INTERFACE", "MC-GROUP", "TRACEABLE", "COMMON", "ENCRYPTION", "PROVIDED-DDS-SERVICE-INSTANCE", "ABSTRACT-IAM-REMOTE-SUBJECT", "BOTTOM", "SOFTWARE-ACTIVATION-DEPENDENCY", "PUBLISHED-INFORMATION", "DATA-RECEIVE-ERROR-EVENT", "PRIMITIVE-ATTRIBUTE-TAILORING", "OBD-RATIO-DENOMINATOR-NEEDS", "WATCHDOG-ACTION-ITEM", "ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF", "DIAGNOSTIC-STORAGE-CONDITION", "PORT-INTERFACE-MAPPING-SET", "ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS", "TE", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "YCBCR", "MONO", "TT", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "CAN-FRAME-TRIGGERING", "CONFIGURED", "ECU-TIMING", "SW-MC-INTERFACE", "IK", "ENUMERATION-MAPPING-TABLE", "MODE-ACCESS-POINT-IDENT", "ARRAY", "RES-AXIS", "DO-IP", "SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING", "SOMEIP-REMOTE-MULTICAST-CONFIG", "AUTOSAR-DATA-TYPE", "1-001", "DIAGNOSTIC-MONITOR-INTERFACE", "PRODUCER", "25", "BSW-VARIABLE-ACCESS", "TP-CONFIG", "DIAGNOSTIC-SESSION-CONTROL", "MEASURED-EXECUTION-TIME", "AVB--IEEE-802--1-AS", "16-KHZ", "COM-EVENT-GRANT-DESIGN", "KL", "VAR-POWER-ON-INIT", "HALF-DUPLEX-MODE", "CLIENT-SERVER-INTERFACE-MAPPING", "CODE", "GETTER-SETTER", "WORST-CASE-HEAP-USAGE", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "48", "SECURITY-EVENT-STATE-FILTER", "DDS-CP-CONSUMED-SERVICE-INSTANCE", "SHOW-LONG-NAME", "RTE-EVENT-IN-SYSTEM-SEPARATION", "UDP", "RESPOND-AFTER-RESET", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "BSW-MGR-NEEDS", "EXTERNAL-REPLACEMENT", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "1000BASE-T1", "BSW-MODE-MANAGER-ERROR-EVENT", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "CYCLE-REPETITION-32", "CYCLE-REPETITION-64", "DDS-CP-PROVIDED-SERVICE-INSTANCE", "SYNCHRONIZATION-POINT-CONSTRAINT", "UPDATE", "RPT-CONTAINER", "CHECKPOINT-TRANSITION", "DETERMINISTIC-SYNC-INSTANTIATION", "ALWAYS", "CALLBACK", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE", "DATA-FORMAT-ELEMENT-REFERENCE", "DIAGNOSTIC-IO-CONTROL", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "MASEKD-NEW-EQUALS-X", "TRUE", "OTHER", "XDOC", "ADAPTIVE-SERVICE-FIND-STARTED", "ISO-11992--4", "RPT-ENABLER-RAM-AND-ROM", "PDU", "ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING", "ICV-SUPPORTED", "LIN-COMMUNICATION-CONNECTOR", "TEST-PASSED", "VAR-NO-INIT", "SD", "TD-EVENT-FR-CLUSTER-CYCLE-START", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "UPLOADABLE-DEPLOYMENT-ELEMENT", "BSW", "ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER", "VFB-TIMING", "DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING", "XFILE", "APPLICATION-DATA-TYPE", "PLATFORM-MODULE-ENDPOINT-CONFIGURATION", "ISO-15031--6", "BSW-MODULE-ENTITY-ACTIVATED", "PAYLOAD-AS-ARRAY", "DIAGNOSTIC-SW-MAPPING", "DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE", "DLT-APPLICATION-TO-PROCESS-MAPPING", "IP-IAM-REMOTE-SUBJECT", "CRYPTO-CERTIFICATE", "PHYSICAL", "BROAD-R-REACH", "SW-COMPONENT-PROTOTYPE", "DEFINE-BY-MEMORY-ADDRESS", "SERVER-ENCRYPT", "ACCESS-PERMISSION-SERVICE-CLASS", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE", "DIAGNOSTIC-SOVD-LOG", "LEGACY", "ACCES-PERRMISSION-SERVICE-CLASS", "HW-ATTRIBUTE-DEF", "200", "NM-CONFIG", "DDS-SECURE-GOVERNANCE", "DIAG-RESPONSE", "COM-TRIGGER-GRANT-DESIGN", "STRUCTURED-REQ", "FRAME-PORT", "PLAIN", "XG-MII", "NOTIFICATION", "HW-CATEGORY", "TTCAN-COMMUNICATION-CONNECTOR", "SHOW-CONTENT", "PERIODIC-EVENT-TRIGGERING", "CURVE-AXIS", "TRIGGER-INTERFACE", "DIAGNOSTIC-ECU-RESET-CLASS", "AGE", "PRE--R-4--2", "APPLICATION-PARTITION", "ADAPTIVE-SERVICE-OFFER-COMPLETED", "MASKED-NEW-EQUALS-X", "USE-FIRST-CONTEXT-DATA", "EOC-EXECUTABLE-ENTITY-REF-ABSTRACT", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "SERVER-MAC-VERIFY", "VLAN-CONFIG", "HI", "GLOBAL-TIME-FR-MASTER", "TD-EVENT-I-SIGNAL", "NV-BLOCK-DESCRIPTOR", "CSERS", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "NO-FLOAT", "DOMAIN-PARTICIPANT-USER-DATA-QOS", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "TP-ADDRESS", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "DEVELOPMENT-ERROR-TRACER", "PHYSICAL-ADDRESS", "IPSEC", "IDSM-ABSTRACT-PORT-INTERFACE", "PDF", "RUNNABLE-ENTITY-GROUP", "TLS-CRYPTO-SERVICE-MAPPING", "USER-DEFINED-EVENT-DEPLOYMENT", "TW", "SLOW-FLASHING-MODE", "REDUNDANT-PER-KEY", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "PERSISTENCY-DEPLOYMENT", "CRC-NOT-VALIDATED", "CONSTRAINT-TAILORING", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE", "IDSM-CONTEXT-PROVIDER-INTERFACE", "DATA-TYPE-MAPPING-SET", "AP-APPLICATION-ENDPOINT", "UNSPECIFIED", "REST-ABSTRACT-PROPERTY-DEF", "COMMAND-LINE-LONG-FORM", "DIAGNOSTIC-MEMORY-DESTINATION", "ERROR", "CONSUMER", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "MAPPING-SCOPE-CORE", "DIAGNOSTIC-EVENT", "UCM", "SYMBOL-PROPS", "IN", "PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE", "UDP-NM", "EXECUTION-TIME", "DDS-EVENT-DEPLOYMENT", "EOC-EVENT-REF", "EO", "SH", "GA", "SERVICE-METHOD-DEPLOYMENT", "SECURITY-EVENT-CONTEXT-MAPPING", "FY", "DIAGNOSTIC-COMPONENT-NEEDS", "DIAGNOSTIC-PARAMETER-ELEMENT", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "X-MII", "NO-SLOPPY", "TG", "FO", "IDSM-INSTANCE", "DOCUMENT-ELEMENT-SCOPE", "AFTER-SALES", "ATTRIBUTE-TAILORING", "MODE-SWITCHED-ACK-EVENT", "PDU-TO-FRAME-MAPPING", "NO-RETURN-VALUE-PROVIDED", "FIREWALL-STATE-SWITCH-INTERFACE", "DHCPV-6", "BSW-CALLED-ENTITY", "BSW-MODULE-DESCRIPTION", "MANUAL-BY-PARTICIPANT", "IDS-PLATFORM-INSTANTIATION", "EVENT-COMBINATION-ON-STORAGE", "METHOD-MAPPING", "IDSM-TIMESTAMP-PROVIDER-MAPPING", "J-1939-DCM-DM-19-SUPPORT", "REQUEST-CALLBACK-TYPE-SUPPLIER", "OEM-BOOT-RESP-APP", "EXECUTABLE-TIMING", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "MANUAL-BY-TOPIC", "CAN-BRIEF", "TIME-MASTER", "IS-GREATER-THAN", "USE-ARRAY-BASE-TYPE", "HW-ELEMENT", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "RESOURCE-GROUP", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "FIT-TO-PAGE", "RESET-MCU", "KN", "TARGET-CONTAINER", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "MEASURED-HEAP-USAGE", "STATE-MANAGEMENT-SLEEP-ACTION-ITEM", "DDS-CP-CONFIG", "SW-RECORD-LAYOUT", "TD-EVENT-OPERATION", "ANY-STANDARDIZED", "INFINITE", "DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE", "DIAGNOSTIC-EVENT-PORT-MAPPING", "SO-AD-ROUTING-GROUP", "STRICT-MONOTONOUS", "SOCKET-ADDRESS", "VERBOSE", "LIN-COMMUNICATION-CONTROLLER", "VAR-FAST", "XYZ", "TRIGGER-INTERFACE-MAPPING", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "NV-RAM-MANAGER", "API-USE", "IEEE-1722-TP-CONFIG", "DA", "SIGNAL-SERVICE-TRANSLATION-PROPS", "COM-MANAGER", "DIAGNOSTIC-AUTHENTICATION-CLASS", "FLEXRAY-TP-NODE", "SINGLE", "IEEE-1722-TP-ACF-BUS-PART", "FRAME-QUEUED-FOR-TRANSMISSION", "INTERFACE-MAPPING", "TD-EVENT-MODE-DECLARATION", "SW-GENERIC-AXIS-PARAM-TYPE", "GATEWAY", "INT-32-BIT", "DDS-PROVIDED-SERVICE-INSTANCE", "APPLICATION-ARRAY-DATA-TYPE", "NO-COM", "OC", "SOCKET-CONNECTION-BUNDLE", "DDS-RPC-SERVICE-DEPLOYMENT", "SHOW-CATEGORY", "FLEXRAY-TP-CONFIG", "BR", "I-PV-6-EXT-HEADER-FILTER-LIST", "DIAGNOSTIC-CONNECTED-INDICATOR", "DDS-TOPIC-ACCESS-RULE", "CLIENT-MAC-GENERATE", "VIEW-MAP", "AUTONOMOUS", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "MOST-SIGNIFICANT-BYTE-FIRST", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "PNG", "PDU-R", "UNDECIDED", "ISO", "SW-COMPONENT-MAPPING-CONSTRAINTS", "NOTHING", "PTP--IEEE-1588--2002", "NETWORK", "MULTIPLEXED-I-PDU", "DIAGNOSTIC-SERVICE-CLASS", "NO-SHOW-SHORT-NAME", "PHM-SUPERVISION", "AR--CLIENT--SERVER", "RECOVERY-VIA-APPLICATION-ACTION", "STARTUP-CONFIG", "NO-SHOW-SEE", "SWITCH-STREAM-IDENTIFICATION", "EVENT-STORAGE-DISABLED", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "LOCAL", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "1-0", "SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING", "DOCUMENTATION", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "CPP-IMPLEMENTATION-DATA-TYPE", "COUPLING-PORT-SHAPER", "WILL-RECEIVE", "USE-ARGUMENT-TYPE", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "ECUC-COMMON-ATTRIBUTES", "LIN-UNCONDITIONAL-FRAME", "FLEXRAY-COMMUNICATION-CONTROLLER", "SYNCHRONIZED", "FLEXRAY-NM-CLUSTER", "EID-USE-API", "DIAGNOSTIC-REQUEST-DOWNLOAD", "BUILD-TYPE-RELEASE", "J-1939-REQUEST-MANAGER", "REQUEST-CALLBACK-TYPE-MANUFACTURER", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "IDSM-CONTEXT-PROVIDER-MAPPING", "WONT-CALL", "INTERGRITY-AND-CONFIDENTIALITY", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "STIMULUS-SYNCHRONIZATION", "STATIC-SOCKET-CONNECTION", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "IS-LESS-THAN", "DIAGNOSTIC-SERVICE-SW-MAPPING", "RUNTIME-ERROR", "MACHINE", "WILL-CALL", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "CP-SOFTWARE-CLUSTER-RESOURCE", "DEBUG", "MEASUREMENT-POINT", "NO-STATUS-BYTE-CHANGE", "OEM-BOOT", "AP-APPLICATION-ERROR-SET", "AB", "CRYPTO-SERVICE-NEEDS", "SUPERVISION-MODE", "FAILURE-AND-SUCCESS", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "DO-IP-POWER-MODE-STATUS-NEEDS", "STATE-MANAGEMENT-ERROR-INTERFACE", "ATOMIC-SW-COMPONENT-TYPE", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "NO-BREAK", "SERVICE-INSTANCE-COLLECTION-SET", "DETAILED", "BSW-OS-TASK-EXECUTION-EVENT", "SUPERVISION-CHECKPOINT", "DIAGNOSTIC-DATA-ELEMENT", "WATCH-TRIGGER-GAP", "SOMEIP-SERVICE-INTERFACE", "HARDWARE-TEST-NEEDS", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "PCM", "LEAF-OF-TARGET-CONTAINER", "CLASSIC", "IS-EXPIRED", "TIMING-CLOCK-SYNC-ACCURACY", "BUILD-ACTION-ENTITY", "BLOCK-SOURCE", "SL", "NON-OS-MODULE-INSTANTIATION", "SRGB", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "ACK-WITH-RT", "CAN-COMMUNICATION-CONNECTOR", "DDS-SECURE-COM-PROPS", "IDSM-TRAFFIC-LIMITATION", "DIAGNOSTIC-MEMORY-IDENTIFIER", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "SYSTEM-MEMORY-USAGE", "MEASURED-STACK-USAGE", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "STATE-MANAGEMENT-TRIGGER-INTERFACE", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "24-25", "STOP", "PORT-INTERFACE-DEFINITION", "TTCAN-PHYSICAL-CHANNEL", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "DIAGNOSTIC-J-1939-NODE", "FLEXRAY-FRAME", "EVENT-HANDLER", "CYCLIC", "ECUC-DEFINITION-ELEMENT", "PL", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "ES", "V-2-X-DATA-MANAGER-NEEDS", "DATA-CONSTR", "XREF-TARGET", "BUS-MIRROR-CHANNEL-MAPPING-IP", "COMMUNICATION-INTER-ECU", "DIAGNOSTIC-SERVICE-INSTANCE", "NO-CHECKPOINT-SUPERVISION", "RPT-SERVICE-POINT", "SYNCHRONIZED-SLAVE-TIME-BASE", "PROCESS", "ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION", "REST-ENDPOINT-DELETE", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "DATA-TRANSFORMATION", "COMPOSITE-INTERFACE", "RUNNABLE-ENTITY-STARTED", "TEST-FAILED-THIS-OPERATION-CYCLE", "NV-DATA-INTERFACE", "NO-SUPPORT", "ROLL-BACK", "TRANSIENT-LOCAL", "AP-SOMEIP-TRANSFORMATION-PROPS", "DIAGNOSTIC-ROUTINE-INTERFACE", "RUNNABLE-ENTITY-VARIABLE-ACCESS", "POSSIBLE-ERROR-REACTION", "GLOBAL-SUPERVISION", "NO-CONSISTENCY-MECHANISM", "DIAGNOSTIC-TROUBLE-CODE-UDS", "INTERGRITY-WITHOUT-CONFIDENTIALITY", "EVALUATED-VARIANT-SET", "VARIATION-POINT-PROXY", "DIAGNOSTIC-CAPABILITY-ELEMENT", "COLOR-AWARE", "SAFETY", "RAW-DATA-STREAM-CLIENT-INTERFACE", "TRACE-SWITCH-ARTI-AND-LOG", "DLT-LOG-CHANNEL", "FLAT-MAP", "APPLICABILITY-INFO-SET", "LINK-LOCAL", "COMMUNICATION-CONTROLLER", "BACKGROUND-EVENT", "MINIMUM-MINOR-VERSION", "NON-EMMISSION-RELATED-DTC", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "EMISSION-RELATED-DTC", "SECRET-SEED", "V-2-X-ACTIVE-SUPPORTED", "NO-OBD-SUPPORT", "SYNCHRONOUS-SERVER-CALL-POINT", "J-1939", "SAE-J-2012--DA", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING", "HEAP-USAGE", "AR-PACKAGE", "CRYPTO-SERVICE-QUEUE", "ECUC-CHOICE-REFERENCE-DEF", "TX-TRIGGER-MERGED", "BASE-T", "PERSISTENCY-FILE-PROXY-INTERFACE", "APPLICATION-ASSOC-MAP-DATA-TYPE", "ALLOCATOR", "SM-INTERACTS-WITH-NM-MAPPING", "SERVICE-INTERFACE-TRIGGER-MAPPING", "COM-OFFER-SERVICE-GRANT", "PUBLIC-KEY", "LONG-HEADER", "SECURED-PDU-HEADER-16-BIT", "HW-PIN", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE", "BUILD-TYPE-DEBUG", "PHM-ACTION-LIST", "MODE-SWITCH-POINT", "AES-3-32-BIT", "ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE", "CONTINUE-AT-IT-POSITION", "NEW-IS-WITHIN", "PRE-COMPILE-TIME", "DIAGNOSTIC-INFO-TYPE", "CALIBRATION-PARAMETER-VALUE-SET", "UCM-SUBORDINATE-MODULE-INSTANTIATION", "STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM", "GETTER", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "USER-DEFINED-CLUSTER", "BSW-SERVICE-DEPENDENCY-IDENT", "20", "LOW", "ROTATE-90-CCW", "CENTER", "TRIGGERED-ON-CHANGE", "LIN-TP-NODE", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "VIDEO-LINE", "LAND", "COMPU-METHOD", "XH", "PROCESSING-STYLE-ASYNCHRONOUS", "1000BASE-T", "DIAGNOSTIC-SOVD-BULK-DATA", "SW-MC-INTERFACE-SOURCE", "NV-BLOCK-NEEDS", "DESELECTED", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "MO", "R-4--2", "RECORD-VALUE-FIELD", "ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED", "PRESENTATION-DISCRETE", "LIFE-CYCLE-STATE", "TRANSFORMATION-TECHNOLOGY", "IP-SEC-CONFIG-PROPS", "DEPENDANT", "DLT-MESSAGE-COLLECTION-SET", "NM-PDU", "SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING", "SCHEDULE-VARIANT-6", "JA", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "DIAGNOSTIC-COMMUNICATION-MANAGER", "INLINE-CONDITIONAL", "GLOBAL-TIME-CAN-SLAVE", "SIDES", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "GU", "USER-DEFINED-GLOBAL-TIME-MASTER", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "EXAMPLE", "IS-GREATER-THAN-OR-EQUAL", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "DIAGNOSTIC-ROUTINE-NEEDS", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "ETHERNET-PRIORITY-REGENERATION", "COUPLING-PORT-SCHEDULER", "I-SIGNAL-AVAILABLE-FOR-RTE", "EXTEND", "IS-GREATER-OR-EQUAL", "BMP", "HW-DESCRIPTION-ENTITY", "BI", "STORE-EVENT", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "SPECIFICATION-DOCUMENT-SCOPE", "I-SIGNAL-GROUP", "150", "SW-COMPONENT-TYPE", "PERSISTENCY-FILE", "CONDITIONAL", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "END-TO-END-PROTECTION-SET", "FAULT", "CYCLIC-AND-ON-CHANGE", "NO-SHOW-ALIAS-NAME", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION", "SUPPORTS-BUFFER-LOCKING", "BSW-INTERRUPT-ENTITY", "VOLATILE", "PASS-THROUGH-SW-CONNECTOR", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "NM-NETWORK-HANDLE", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "SOMEIP-EVENT", "ROTATE-90-CCW-FIT-TO-TEXT", "MACHINE-CYCLE", "BLUEPRINT-MAPPING-SET", "NEVER", "preserve", "OPERATION-CALLED", "PHM-RECOVERY-ACTION-INTERFACE", "NO-SHOW-CONTENT", "NON-VOLATILE", "ZH", "ECUC-ABSTRACT-STRING-PARAM-DEF", "BSW-DISTINGUISHED-PARTITION", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "EU", "ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "E-2-E-PROFILE-CONFIGURATION", "GLOBAL-TIME-FR-SLAVE", "RESET-MACHINE", "PR-PORT-PROTOTYPE", "SYMMETRIC-KEY", "BINARY-MANIFEST-ADDRESSABLE-OBJECT", "TP-CONNECTION-IDENT", "SERVICE-INTERFACE-PEDIGREE", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER", "SERVICE-INTERFACE-DEPLOYMENT", "ABSTRACT-ETHERNET-FRAME", "IEEE802-1AS-AUTOSAR", "ANALYZED-EXECUTION-TIME", "VERIFY", "PARTITION", "RETURN-VALUE-PROVIDED", "OBSERVER", "FUNCTION-GROUP-STATE-TO-NM-HANDLE", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "SWC-TO-ECU-MAPPING", "DIAGNOSTIC-ECU-RESET-INTERFACE", "HR", "FLEXRAY-AR-TP-CONFIG", "OFF", "COM-MANAGEMENT-MAPPING", "COUPLING-PORT-STRUCTURAL-ELEMENT", "COM-MGR-USER-NEEDS", "CLIENT-SERVER-OPERATION", "VERTEX-OF-TARGET-CONTAINER", "ACK-WITHOUT-RT", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "BSW-EVENT", "DIAGNOSTIC-CONTROL-NEEDS", "SO-CON-I-PDU-IDENTIFIER", "EXECUTION-ORDER-CONSTRAINT", "ARBITRARY-EVENT-TRIGGERING", "OCCURENCE", "GRAYSCALE", "TCP-OPTION-FILTER-LIST", "COMMUNICATION-INTRA-PARTITION", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "ICMP", "DETERMINISTIC-SYNC-MASTER", "TESTED-AND-FAILED", "AGGREGATION-TAILORING", "DDS-CP-TOPIC", "NO-MONOTONY", "COM-GRANT", "PARAMETER-ACCESS", "DDS-DOMAIN-RANGE", "OBD-MONITOR-SERVICE-NEEDS", "RAW-DATA-STREAM-INTERFACE", "DIAGNOSTIC-REQUEST-ROUTINE-RESULTS", "REQUEST-NO-RETURN", "MONOCHROME", "BSW-BACKGROUND-EVENT", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "BG", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "ADAPTIVE-FIELD-SETTER-CALLED", "LA", "SESSION-HANDLING-INACTIVE", "IEC-61937", "V-2-X-FACILITIES", "DIAGNOSTIC-SOVD-UPDATE-INTERFACE", "BUILD", "SEARCH-FOR-ID", "AUTO-IPDHCPV-4", "FM-ATTRIBUTE-DEF", "ROUGH-ESTIMATE-HEAP-USAGE", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "YCGCO", "25-24", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "EVENT-WINDOW-CURRENT-CYCLE", "DROP-UNTAGGED", "GLOBAL-TIME-ETH-MASTER", "DO-IP-ROUTING-ACTIVATION", "STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE", "FUNCTION-INHIBITION-NEEDS", "CLIENT-SERVER-INTERFACE", "DLT-ECU", "ETH-IP-PROPS", "TX-REF-TRIGGER-GAP", "STEADY", "ALIAS-NAME-SET", "SYSTEM-DESIGN-TIME", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "TD-EVENT-SLLET-PORT", "APPLICATION-INTERFACE", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "VENDOR-SPECIFIC-SERVICE-NEEDS", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "SYNC-TIME-BASE-MGR-USER-NEEDS", "SYSTEM-SIGNAL-GROUP", "WONT-SEND", "TRACEABLE-TABLE", "ORDINARY-EOC", "FAILURE-ONLY", "SYNCHRONIZED-TIME-BASE-CONSUMER", "ALL-PARTIAL-NETWORKS-ACTIVE", "COM-TRIGGER-GRANT", "CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "SYSTEM-SUPPLIER-BOOT-RESP-APP", "MONITOR-MODE", "OBSERVER-BASED", "CONFIG-DATA", "SERVICE-FIELD-DEPLOYMENT", "FUNCTION-INHIBITION-MANAGER", "DATA-PROTOTYPE", "DIAGNOSTIC-ENV-MODE-ELEMENT", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "TIFF", "CONSISTENCY-MECHANISM-REQUIRED", "LO", "TRANSFORMER-HARD-ERROR-EVENT", "SW-SYSTEMCONST", "4-2-0", "ECUC-URI-REFERENCE-DEF", "DIAGNOSTIC-EVENT-NEEDS", "CYCLE-REPETITION-40", "default", "XXG-MII", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "UK", "MULTIPLE-OCCURRENCES", "DDS-FIELD-DEPLOYMENT", "OS-TASK-PROXY", "MACHINE-TIMING", "ASSEMBLY-SW-CONNECTOR", "SN", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "CAPTURE-SYNCHRONOUS-TO-REPORTING", "NO-SHOW-PAGE", "DELETE", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "SA", "AF", "UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT", "MIXED-29-BIT", "SOMEIP-REQUIRED-EVENT-GROUP", "PRESENTATION-CONTINUOUS", "REST-STRING-PROPERTY-DEF", "MALFUNCTION", "PERSISTENCY-INTERFACE", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "ROTATE-90-CCW-LIMIT-TO-TEXT", "EOC-EXECUTABLE-ENTITY-REF", "DATA-PROTOTYPE-GROUP", "MONOTONOUS", "DIAGNOSTIC-TROUBLE-CODE-OBD", "AUTO-IP", "ECUC-FOREIGN-REFERENCE-DEF", "DLT-APPLICATION", "MASEKD-NEW-EQUALS-MASKED-OLD", "NETWORK-HANDLE-PORT-MAPPING", "SINGLE-OCCURRENCE", "V-2-X-FAC-USER-NEEDS", "RPT-LEVEL-1", "TRANSFORMATION-PROPS", "IT", "ETH-TCP-IP-PROPS", "MACHINE-DESIGN", "DIAGNOSTIC-ENABLE-CONDITION", "SECURED-PDU-HEADER-08-BIT", "SDG-TAILORING", "DIAGNOSTIC-FIM-EVENT-GROUP", "PERSISTENCY-FILE-ELEMENT", "CONCRETE-PATTERN-EVENT-TRIGGERING", "COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER", "ECUC-QUERY-EXPRESSION", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "FLOAT-32-BIT", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "ADAPTIVE-EVENT-RECEIVED", "DIAGNOSTIC-SOVD-LOCK", "PORT-INTERFACE-MAPPING", "INTERNAL-BEHAVIOR", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "CAN-NM-NODE", "POLY", "SYSTEM-MAPPING", "NO-NEWLINE", "DIAGNOSTIC-PARAMETER-IDENT", "RESTART-APPLICATION", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA", "BSW-DATA-RECEIVED-EVENT", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "PRE-COMPILE", "STACK-USAGE", "LIFE-CYCLE-INFO-SET", "ACTION-ITEM", "PROCESS-IS-SELF-TERMINATING", "ERROR-TRACER", "CALIBRATION-OFFLINE", "AH", "SYSTEM-SIGNAL", "HW-TYPE", "COLDSTART", "PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING", "SW-CODE-SYNTAX", "DIAGNOSTIC-MEMORY-BY-ADDRESS", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "RO", "COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING", "WARMUP", "TD-EVENT-FRAME-ETHERNET", "ECUC-DESTINATION-URI-DEF", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING", "TRANSIENT-FAULT", "FRAME-RECEIVED-BY-IF", "IEEE-1722-TP-ACF-LIN-PART", "COMPILER", "V-2-X-MANAGEMENT", "SENDER-RECEIVER-INTERFACE", "UCM-DESCRIPTION", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "PROCESS-PHM-ACTION-ITEM", "OPTIONS", "TCP", "DOES-NOT-USE-LOGGING", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED", "INLINE", "QUEUED", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "NETWORK-CONFIGURATION", "NO-SUPERVISION", "USER-DEFINED-PDU", "MODELED", "ONE-EVERY-N", "IMPOSITION-TIME-DEFINITION-GROUP", "PHM-SUPERVISED-ENTITY-INTERFACE", "UCM-MASTER-MODULE-INSTANTIATION", "FIRE-AND-FORGET-MAPPING", "DIAGNOSTIC-IUMPR-GROUP", "APPLICATION-PRIMITIVE-DATA-TYPE", "FUNCTION-GROUP-SET", "DOCUMENTATION-CONTEXT", "SERVER-VERIFY", "PC-AFFECTS-LT", "MS", "REPORTS-EXECUTION-STATE", "48-KHZ", "SDG-PRIMITIVE-ATTRIBUTE", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "PER-INSTANCE-MEMORY", "JW", "SESSION-HANDLING-ACTIVE", "VEHICLE-PACKAGE", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "PORT-PROTOTYPE-BLUEPRINT", "CLOSED", "TIMING-DESCRIPTION-EVENT", "NEW-IS-LESS-OR-EQUAL", "CYCLE-REPETITION-2", "PGWIDE", "PHM-ARBITRATION", "READ-WRITE", "BSW-MODE-SWITCHED-ACK-EVENT", "ETHERNET-FRAME-TRIGGERING", "NO-AFFECT", "DESCENDANT", "FJ", "DDS-CP-QOS-PROFILE", "STATE-MANAGEMENT-PORT-INTERFACE", "NODE", "J-1939-SHARED-ADDRESS-CLUSTER", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "FLEXRAY-TP-CONNECTION-CONTROL", "GET", "PHYSICAL-CHANNEL", "GROSS", "APPLICATION-SW-COMPONENT-TYPE", "PROVIDED-SERVICE-INSTANCE", "WARN", "WO", "AGREED", "COUPLING-PORT-ABSTRACT-SHAPER", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "CLIENT-MAC-VERIFY", "SDG-AGGREGATION-WITH-VARIATION", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "DATA-FORMAT-ELEMENT-SCOPE", "SCHEDULE-VARIANT-3", "RUN-CONTINUOUS", "ACTIVATION-MULTICAST", "ECU-MANAGER", "SHOW-SEE", "DIAGNOSTIC-PORT-INTERFACE", "SATURATE", "SERVICE-DISCOVERY", "BSW-INTERNAL-TRIGGERING-POINT", "FIT-TO-TEXT", "INTER-PARTITION-INTRA-ECU", "IEEE-1722-TP-AAF-CONNECTION", "UZ", "PURE-LOCAL-TIME-BASE", "FM-FEATURE-RESTRICTION", "PERSISTENT", "PORT-BLUEPRINT", "AND", "CAN-FD", "ECUC-PARAM-CONF-CONTAINER-DEF", "CAN-NM-CLUSTER", "CIRCLE", "TO", "SCHEDULE-VARIANT-2", "RULE", "SLP", "SYMBOLIC-NAME-PROPS", "FIX-AXIS", "NM-NODE", "COM-OFFER-SERVICE-GRANT-DESIGN", "ADAPTIVE-SERVICE-OFFER-STARTED", "DEFAULT-MODE", "FALSE", "OVERWRITE", "REQUEST", "ADAPTIVE-FIELD-NOTIFICATION-RECEIVED", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "NO-BOOT", "SW-CLASS-PROTOTYPE", "CONFIDENTIALITY-OFFSET--50", "NM-INTERACTS-WITH-SM-MAPPING", "VIEW-MAP-SET", "OM", "BSW-MODULE-ENTITY-TERMINATED", "SENT-TAGGED", "MAC-SEC-PARTICIPANT-SET", "RESPONSE", "ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE", "NEW-IS-LESS", "CAN-TP-ADDRESS", "FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION", "PERSISTENCY-KEY-VALUE-STORAGE", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "J-1939-TP-CONFIG", "UDP-CHECKSUM-DISABLED", "COM-AXIS", "VARIABLE-DATA-PROTOTYPE-SENT", "SR", "PROCESS-DESIGN", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "AZ", "DLT-LOG-SINK", "SYNC-BASE-TIME-MANAGER", "TRANSFORMING-I-SIGNAL", "MASTER-ECU", "ABSTRACT-CAN-COMMUNICATION-CONNECTOR", "WATCH-DOG-MANAGER", "BSW-ENTRY-RELATIONSHIP-SET", "FM-FEATURE-MAP-ASSERTION", "240", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "ETHERNET-NETWORK-CONFIGURATION", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING", "JAVA", "REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE", "RAW-DATA-STREAM-SERVER-INTERFACE", "SOMEIP-EVENT-GROUP", "ECU", "SHOW-PAGE", "NOT-SENT", "SHOW-TYPE", "FATAL", "DEDICATED", "SYSTEM-TIMING", "SEC-OC-JOB-MAPPING", "NV-BLOCK-SW-COMPONENT-TYPE", "DIAGNOSTIC-SOVD-SERVICE-INSTANCE", "SYSTEM", "BSW-SCHEDULE-EVENT", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "DIAGNOSTIC-CONDITION", "COMPOSITION-SW-COMPONENT-TYPE", "IDSM-TIMESTAMP-PROVIDER-INTERFACE", "ISO-14229--1", "SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "PRIMARY-ECU", "ABSTRACT-EXECUTION-CONTEXT", "CRC-NOT-SUPPORTED", "ECUC-INTEGER-PARAM-DEF", "CONSTANT-SPECIFICATION-MAPPING-SET", "DO-IP-LOGIC-ADDRESS", "REACTION", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE", "INCREASING", "PRIMITIVE", "IAM-MODULE-INSTANTIATION", "TERMINATE", "DIAGNOSTIC-FREEZE-FRAME", "STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE", "I-SIGNAL-I-PDU-GROUP", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED", "REST-INTEGER-PROPERTY-DEF", "HEALTH-CHANNEL-EXTERNAL-MODE", "STOP-TRIGGER", "ETH-TP-CONFIG", "MC-FUNCTION", "PORT-GROUP", "STATE-MANAGEMENT-EM-ERROR-INTERFACE", "ATP-BLUEPRINTABLE", "ETHERNET-CLUSTER"];

    /// derive an enum entry from an input string using a perfect hash function
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseEnumItemError> {
        static DISPLACEMENTS: [(u16, u16); 539] = [(0, 81), (0, 0), (0, 35), (0, 135), (0, 10), (0, 0), (0, 1), (0, 0), (0, 62), (0, 137), (0, 40), (0, 75), (0, 343), (0, 4), (0, 2), (0, 10), (0, 1), (0, 76), (0, 6), (0, 29), (0, 350), (0, 10), (0, 140), (0, 4), (0, 1244), (0, 0), (0, 0), (0, 56), (0, 30), (0, 3), (0, 98), (0, 5), (0, 379), (0, 0), (0, 157), (0, 1596), (0, 625), (0, 8), (0, 21), (0, 0), (0, 84), (0, 857), (0, 667), (0, 169), (0, 0), (0, 0), (0, 5), (0, 0), (0, 0), (0, 110), (0, 4), (0, 0), (0, 0), (0, 1960), (0, 267), (0, 612), (0, 143), (0, 375), (0, 2367), (0, 16), (0, 76), (0, 417), (0, 67), (0, 237), (0, 42), (0, 2321), (0, 26), (0, 810), (0, 23), (1, 435), (0, 3), (0, 1353), (0, 166), (0, 5), (0, 548), (0, 221), (0, 73), (0, 317), (0, 1), (0, 11), (0, 0), (0, 222), (0, 17), (0, 422), (0, 24), (0, 781), (0, 1), (0, 245), (0, 1137), (0, 91), (0, 191), (0, 255), (0, 9), (0, 19), (0, 2), (0, 0), (0, 69), (0, 1), (0, 0), (0, 22), (0, 22), (0, 3), (0, 2), (0, 287), (0, 12), (0, 3), (0, 58), (0, 310), (0, 20), (0, 10), (0, 5), (0, 408), (0, 1803), (0, 157), (0, 1860), (0, 74), (0, 8), (0, 132), (0, 17), (0, 131), (0, 146), (0, 1088), (0, 19), (0, 21), (0, 0), (0, 74), (0, 1), (0, 106), (0, 160), (0, 627), (0, 1993), (0, 28), (0, 1), (0, 6), (0, 1), (0, 454), (0, 276), (0, 2402), (0, 162), (0, 688), (0, 591), (0, 13), (0, 4), (0, 59), (0, 10), (0, 27), (0, 170), (0, 1397), (0, 42), (0, 24), (0, 12), (0, 59), (0, 2), (0, 965), (0, 182), (0, 0), (0, 22), (0, 666), (0, 1), (0, 0), (0, 3), (0, 427), (0, 6), (0, 1458), (0, 297), (0, 18), (0, 8), (0, 83), (0, 47), (0, 756), (0, 5), (0, 1183), (0, 25), (0, 15), (0, 194), (0, 28), (0, 148), (0, 218), (0, 2), (0, 1561), (0, 0), (0, 140), (0, 205), (0, 0), (0, 454), (0, 996), (0, 1), (0, 1063), (0, 2), (0, 5), (0, 1), (0, 7), (0, 0), (0, 0), (0, 196), (0, 20), (0, 378), (0, 292), (0, 574), (0, 39), (0, 5), (0, 628), (0, 1), (0, 170), (0, 365), (0, 0), (0, 1192), (0, 1), (0, 193), (0, 3), (0, 215), (0, 1), (0, 84), (0, 3), (0, 1885), (0, 104), (0, 888), (0, 0), (0, 1414), (0, 368), (0, 551), (0, 161), (0, 156), (0, 107), (0, 162), (0, 18), (0, 47), (0, 1351), (0, 1231), (0, 2160), (0, 1), (0, 33), (0, 39), (0, 213), (0, 39), (0, 15), (0, 68), (0, 24), (0, 168), (0, 1), (0, 20), (0, 1), (0, 382), (0, 62), (0, 2), (0, 140), (0, 158), (0, 664), (0, 60), (0, 1592), (0, 1305), (0, 72), (0, 2), (0, 362), (0, 44), (0, 932), (0, 133), (0, 175), (0, 18), (0, 728), (0, 1), (1, 1936), (0, 9), (0, 208), (0, 269), (0, 42), (0, 102), (0, 334), (0, 456), (0, 83), (0, 947), (0, 19), (0, 340), (0, 1626), (0, 4), (0, 3), (0, 33), (1, 80), (0, 2543), (0, 29), (0, 1), (0, 5), (0, 3), (0, 3), (0, 1), (0, 354), (0, 27), (0, 1334), (0, 26), (0, 1792), (0, 71), (0, 2), (0, 0), (0, 362), (0, 2), (0, 0), (1, 776), (0, 37), (0, 1), (0, 706), (0, 1372), (0, 45), (0, 265), (0, 313), (0, 6), (0, 52), (0, 20), (0, 57), (0, 7), (0, 611), (1, 62), (0, 12), (0, 16), (0, 22), (0, 1928), (1, 88), (0, 32), (0, 0), (0, 1), (0, 136), (0, 254), (0, 0), (0, 519), (0, 23), (0, 109), (0, 78), (0, 14), (0, 1195), (0, 1587), (0, 54), (0, 778), (0, 0), (3, 2025), (0, 111), (0, 66), (0, 350), (0, 38), (0, 3), (0, 426), (0, 1709), (0, 555), (0, 374), (0, 1520), (0, 2366), (0, 1446), (0, 6), (0, 188), (0, 68), (0, 2234), (0, 161), (0, 38), (0, 144), (1, 2276), (0, 8), (0, 521), (0, 450), (0, 37), (0, 132), (0, 996), (0, 670), (0, 156), (0, 2), (0, 775), (0, 54), (0, 184), (0, 3), (0, 11), (0, 911), (1, 5), (0, 2), (0, 304), (0, 3), (0, 8), (0, 3), (0, 5), (0, 2352), (1, 1375), (0, 174), (0, 0), (0, 34), (1, 2245), (0, 173), (0, 7), (0, 18), (0, 325), (0, 1981), (0, 1842), (0, 4), (1, 2513), (0, 78), (0, 182), (2, 2112), (0, 45), (0, 43), (0, 2), (3, 2674), (1, 2505), (0, 2322), (0, 308), (0, 567), (3, 1150), (0, 4), (0, 2629), (1, 186), (0, 811), (0, 175), (0, 9), (0, 1581), (0, 225), (0, 590), (0, 58), (0, 1342), (0, 1270), (0, 318), (0, 51), (0, 331), (0, 40), (2, 1213), (0, 823), (0, 5), (0, 1022), (0, 174), (0, 408), (3, 1147), (0, 25), (1, 217), (0, 27), (0, 775), (0, 0), (0, 140), (5, 1198), (0, 403), (0, 2346), (0, 118), (2, 506), (0, 282), (0, 44), (0, 1156), (0, 154), (0, 5), (2, 2547), (3, 349), (0, 1643), (0, 1061), (0, 829), (0, 0), (0, 49), (0, 1599), (1, 1232), (0, 737), (2, 1271), (0, 2321), (1, 834), (1, 712), (0, 35), (1, 321), (0, 0), (2, 1942), (7, 1706), (0, 88), (0, 1808), (3, 247), (0, 1), (0, 301), (1, 2048), (0, 259), (0, 2530), (0, 7), (0, 771), (0, 548), (0, 259), (1, 2338), (0, 92), (0, 0), (0, 995), (0, 1696), (0, 764), (0, 226), (0, 537), (1, 489), (0, 12), (4, 2346), (0, 2), (0, 1), (3, 2051), (0, 98), (14, 1898), (2, 2236), (0, 8), (0, 238), (0, 1371), (0, 0), (0, 1286), (0, 1205), (0, 11), (0, 244), (1, 2650), (15, 1845), (0, 5), (0, 1607), (0, 79), (10, 1946), (0, 12), (0, 17), (0, 1765), (0, 1), (0, 214), (0, 16), (1, 1127), (0, 114), (0, 1321), (0, 125), (0, 414), (0, 67), (0, 1), (0, 259), (6, 1001), (11, 2511), (0, 0), (0, 1770), (0, 2431), (0, 50), (0, 18), (0, 0), (0, 24), (1, 7), (1, 1), (0, 4), (19, 2098), (0, 5), (0, 502), (1, 1606), (0, 32), (0, 1966), (5, 1359), (0, 115), (0, 253), (0, 85), (3, 540)];
        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 539) as usize];
        let item_idx = u32::from(d2).wrapping_add(f1.wrapping_mul(u32::from(d1))).wrapping_add(f2) as usize % 2693;
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
    #[must_use]
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

