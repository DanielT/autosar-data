use crate::hashfunc;

#[derive(Debug)]
/// The error type `ParseEnumItemError` is returned when `from_str()` / `parse()` fails for `EnumItem`
pub struct ParseEnumItemError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
#[non_exhaustive]
#[rustfmt::skip]
/// Enum of all possible enum values in Autosar
pub enum EnumItem {
    /// -500-MILES
    _500Miles                                                              = 1715,
    /// 1
    _1                                                                     = 523,
    /// 1-0
    _1_0                                                                   = 2322,
    /// 1-001
    _1_001                                                                 = 15,
    /// 1-1-001
    _1_1_001                                                               = 2181,
    /// 1-8
    _1_8                                                                   = 2148,
    /// 10
    _10                                                                    = 2491,
    /// 100
    _100                                                                   = 2046,
    /// 10000BASE-T1
    _10000baseT1                                                           = 2771,
    /// 1000BASE-T
    _1000baseT                                                             = 2275,
    /// 1000BASE-T1
    _1000baseT1                                                            = 839,
    /// 100BASE-T1
    _100baseT1                                                             = 518,
    /// 100BASE-TX
    _100baseTx                                                             = 240,
    /// 10BASE-T1S
    _10baseT1s                                                             = 201,
    /// 12
    _12                                                                    = 1256,
    /// 120
    _120                                                                   = 2230,
    /// 15
    _15                                                                    = 239,
    /// 150
    _150                                                                   = 340,
    /// 16
    _16                                                                    = 2778,
    /// 16-KHZ
    _16Khz                                                                 = 2101,
    /// 176-4-KHZ
    _176_4Khz                                                              = 2519,
    /// 192-KHZ
    _192Khz                                                                = 701,
    /// 2
    _2                                                                     = 1443,
    /// 20
    _20                                                                    = 299,
    /// 200
    _200                                                                   = 205,
    /// 24
    _24                                                                    = 446,
    /// 24-25
    _24_25                                                                 = 530,
    /// 24-KHZ
    _24Khz                                                                 = 2766,
    /// 240
    _240                                                                   = 1694,
    /// 25
    _25                                                                    = 1157,
    /// 25-24
    _25_24                                                                 = 699,
    /// 2500BASE-T1
    _2500baseT1                                                            = 910,
    /// 30
    _30                                                                    = 2343,
    /// 300
    _300                                                                   = 2522,
    /// 32-KHZ
    _32Khz                                                                 = 2432,
    /// 4-1-1
    _4_1_1                                                                 = 785,
    /// 4-2-0
    _4_2_0                                                                 = 144,
    /// 4-2-2
    _4_2_2                                                                 = 716,
    /// 4-2-2-4
    _4_2_2_4                                                               = 1013,
    /// 4-4-4
    _4_4_4                                                                 = 1769,
    /// 4-4-4-4
    _4_4_4_4                                                               = 1913,
    /// 44-1-KHZ
    _44_1Khz                                                               = 914,
    /// 48
    _48                                                                    = 432,
    /// 48-KHZ
    _48Khz                                                                 = 807,
    /// 5
    _5                                                                     = 2777,
    /// 50
    _50                                                                    = 2424,
    /// 5000BASE-T1
    _5000baseT1                                                            = 1404,
    /// 60
    _60                                                                    = 153,
    /// 72
    _72                                                                    = 1041,
    /// 8
    _8                                                                     = 2425,
    /// 8-KHZ
    _8Khz                                                                  = 2783,
    /// 85
    _85                                                                    = 2113,
    /// 88-2-KHZ
    _88_2Khz                                                               = 1920,
    /// 96-KHZ
    _96Khz                                                                 = 781,
    /// AA
    Aa                                                                     = 2451,
    /// AB
    Ab                                                                     = 2797,
    /// ABSTRACT
    Abstract                                                               = 2290,
    /// ABSTRACT-ACCESS-POINT
    AbstractAccessPoint                                                    = 248,
    /// ABSTRACT-CAN-CLUSTER
    AbstractCanCluster                                                     = 1932,
    /// ABSTRACT-CAN-COMMUNICATION-CONNECTOR
    AbstractCanCommunicationConnector                                      = 1181,
    /// ABSTRACT-CAN-COMMUNICATION-CONTROLLER
    AbstractCanCommunicationController                                     = 1936,
    /// ABSTRACT-CAN-PHYSICAL-CHANNEL
    AbstractCanPhysicalChannel                                             = 271,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 1618,
    /// ABSTRACT-CRYPTO-KEY-SLOT-INTERFACE
    AbstractCryptoKeySlotInterface                                         = 540,
    /// ABSTRACT-CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    AbstractCryptoKeySlotToPortPrototypeMapping                            = 320,
    /// ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS
    AbstractDoIpLogicAddressProps                                          = 1102,
    /// ABSTRACT-DO-IP-PORT-MAPPING
    AbstractDoIpPortMapping                                                = 568,
    /// ABSTRACT-ETHERNET-FRAME
    AbstractEthernetFrame                                                  = 138,
    /// ABSTRACT-EVENT
    AbstractEvent                                                          = 691,
    /// ABSTRACT-EXECUTION-CONTEXT
    AbstractExecutionContext                                               = 1029,
    /// ABSTRACT-FUNCTIONAL-CLUSTER-DESIGN
    AbstractFunctionalClusterDesign                                        = 207,
    /// ABSTRACT-IAM-REMOTE-SUBJECT
    AbstractIamRemoteSubject                                               = 1472,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE
    AbstractImplementationDataType                                         = 1508,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT
    AbstractImplementationDataTypeElement                                  = 706,
    /// ABSTRACT-PROVIDED-PORT-PROTOTYPE
    AbstractProvidedPortPrototype                                          = 1747,
    /// ABSTRACT-RAW-DATA-STREAM-INTERFACE
    AbstractRawDataStreamInterface                                         = 2029,
    /// ABSTRACT-REQUIRED-PORT-PROTOTYPE
    AbstractRequiredPortPrototype                                          = 1916,
    /// ABSTRACT-SECURITY-EVENT-FILTER
    AbstractSecurityEventFilter                                            = 1528,
    /// ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER
    AbstractSecurityIdsmInstanceFilter                                     = 2711,
    /// ABSTRACT-SERVICE-INSTANCE
    AbstractServiceInstance                                                = 1246,
    /// ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING
    AbstractSignalBasedToISignalTriggeringMapping                          = 1098,
    /// ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE
    AbstractSynchronizedTimeBaseInterface                                  = 1099,
    /// ACCEPT-ALL
    AcceptAll                                                              = 990,
    /// ACCEPT-CONFIGURED
    AcceptConfigured                                                       = 524,
    /// ACCES-PERRMISSION-SERVICE-CLASS
    AccesPerrmissionServiceClass                                           = 538,
    /// ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS
    AccessPermissionInstanceOverridesClass                                 = 1620,
    /// ACCESS-PERMISSION-SERVICE-CLASS
    AccessPermissionServiceClass                                           = 56,
    /// ACCESS-PERMISSION-SERVICE-INSTANCE
    AccessPermissionServiceInstance                                        = 819,
    /// ACK-WITH-RT
    AckWithRt                                                              = 1520,
    /// ACK-WITHOUT-RT
    AckWithoutRt                                                           = 681,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 337,
    /// ACL-OPERATION
    AclOperation                                                           = 2224,
    /// ACL-PERMISSION
    AclPermission                                                          = 693,
    /// ACL-ROLE
    AclRole                                                                = 1340,
    /// ACTION
    Action                                                                 = 1084,
    /// ACTION-ITEM
    ActionItem                                                             = 2414,
    /// ACTION-LIST
    ActionList                                                             = 1255,
    /// ACTIVATE
    Activate                                                               = 451,
    /// ACTIVATION-AND-TRIGGER-UNICAST
    ActivationAndTriggerUnicast                                            = 1011,
    /// ACTIVATION-MULTICAST
    ActivationMulticast                                                    = 1502,
    /// ACTIVATION-UNICAST
    ActivationUnicast                                                      = 2420,
    /// ACTIVE
    Active                                                                 = 2434,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 1871,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 2515,
    /// ADAPTIVE-EVENT-RECEIVED
    AdaptiveEventReceived                                                  = 89,
    /// ADAPTIVE-EVENT-SENT
    AdaptiveEventSent                                                      = 2372,
    /// ADAPTIVE-FIELD-GETTER-CALLED
    AdaptiveFieldGetterCalled                                              = 894,
    /// ADAPTIVE-FIELD-GETTER-COMPLETED
    AdaptiveFieldGetterCompleted                                           = 119,
    /// ADAPTIVE-FIELD-NOTIFICATION-RECEIVED
    AdaptiveFieldNotificationReceived                                      = 1922,
    /// ADAPTIVE-FIELD-NOTIFICATION-SENT
    AdaptiveFieldNotificationSent                                          = 2500,
    /// ADAPTIVE-FIELD-SETTER-CALLED
    AdaptiveFieldSetterCalled                                              = 1406,
    /// ADAPTIVE-FIELD-SETTER-COMPLETED
    AdaptiveFieldSetterCompleted                                           = 442,
    /// ADAPTIVE-FIREWALL-MODULE-INSTANTIATION
    AdaptiveFirewallModuleInstantiation                                    = 1864,
    /// ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING
    AdaptiveFirewallToPortPrototypeMapping                                 = 557,
    /// ADAPTIVE-METHOD-CALL-RECEIVED
    AdaptiveMethodCallReceived                                             = 175,
    /// ADAPTIVE-METHOD-CALLED
    AdaptiveMethodCalled                                                   = 544,
    /// ADAPTIVE-METHOD-RESPONSE-RECEIVED
    AdaptiveMethodResponseReceived                                         = 2648,
    /// ADAPTIVE-METHOD-RESPONSE-SENT
    AdaptiveMethodResponseSent                                             = 554,
    /// ADAPTIVE-MODULE-INSTANTIATION
    AdaptiveModuleInstantiation                                            = 1952,
    /// ADAPTIVE-PLATFORM-SERVICE-INSTANCE
    AdaptivePlatformServiceInstance                                        = 847,
    /// ADAPTIVE-SERVICE-FIND-COMPLETED
    AdaptiveServiceFindCompleted                                           = 504,
    /// ADAPTIVE-SERVICE-FIND-STARTED
    AdaptiveServiceFindStarted                                             = 832,
    /// ADAPTIVE-SERVICE-OFFER-COMPLETED
    AdaptiveServiceOfferCompleted                                          = 150,
    /// ADAPTIVE-SERVICE-OFFER-STARTED
    AdaptiveServiceOfferStarted                                            = 425,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED
    AdaptiveServiceStopSubscriptionCompleted                               = 1661,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED
    AdaptiveServiceStopSubscriptionStarted                                 = 1817,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED
    AdaptiveServiceSubscriptionAcknowledgeCompleted                        = 1865,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED
    AdaptiveServiceSubscriptionAcknowledgeStarted                          = 535,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED
    AdaptiveServiceSubscriptionCompleted                                   = 1123,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED
    AdaptiveServiceSubscriptionStarted                                     = 1773,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 1958,
    /// ADDR-METHOD-SHORT-NAME
    AddrMethodShortName                                                    = 436,
    /// ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT
    AddrMethodShortNameAndAlignment                                        = 1391,
    /// AES-3-32-BIT
    Aes3_32Bit                                                             = 1887,
    /// AF
    Af                                                                     = 265,
    /// AFTER-SALES
    AfterSales                                                             = 553,
    /// AFTERMAKET
    Aftermaket                                                             = 2242,
    /// AFTERMARKET
    Aftermarket                                                            = 417,
    /// AGE
    Age                                                                    = 2169,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 1034,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 334,
    /// AGREED
    Agreed                                                                 = 113,
    /// AH
    Ah                                                                     = 1162,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 1919,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 179,
    /// ALL
    All                                                                    = 2334,
    /// ALL-16-BIT
    All16Bit                                                               = 2719,
    /// ALL-INDICES-DIFFERENT-ARRAY-SIZE
    AllIndicesDifferentArraySize                                           = 1803,
    /// ALL-INDICES-SAME-ARRAY-SIZE
    AllIndicesSameArraySize                                                = 1122,
    /// ALL-PARTIAL-NETWORKS-ACTIVE
    AllPartialNetworksActive                                               = 620,
    /// ALL-SUPPORTED-DTCS
    AllSupportedDtcs                                                       = 2399,
    /// ALLOCATOR
    Allocator                                                              = 1923,
    /// ALLOW
    Allow                                                                  = 2002,
    /// ALTERNATING-8-BIT
    Alternating8Bit                                                        = 1429,
    /// ALWAYS
    Always                                                                 = 1049,
    /// AM
    Am                                                                     = 1701,
    /// AMBER-WARNING
    AmberWarning                                                           = 2019,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 1296,
    /// AND
    And                                                                    = 2218,
    /// ANY
    Any                                                                    = 2376,
    /// ANY-PARTIAL-NETWORK-ACTIVE
    AnyPartialNetworkActive                                                = 2044,
    /// ANY-SEND-OPERATION
    AnySendOperation                                                       = 2358,
    /// ANY-STANDARDIZED
    AnyStandardized                                                        = 1061,
    /// AP
    Ap                                                                     = 67,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 1394,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 1837,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 1634,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 1028,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 2150,
    /// API
    Api                                                                    = 2276,
    /// API-BASED
    ApiBased                                                               = 1170,
    /// API-USE
    ApiUse                                                                 = 2558,
    /// APMC-ABSTRACT-DEFINITION
    ApmcAbstractDefinition                                                 = 130,
    /// APMC-ABSTRACT-FOREIGN-REFERENCE-DEF
    ApmcAbstractForeignReferenceDef                                        = 1128,
    /// APMC-ABSTRACT-INSTANCE-REFERENCE-DEF
    ApmcAbstractInstanceReferenceDef                                       = 2311,
    /// APMC-ABSTRACT-INSTANCE-REFERENCE-VALUE
    ApmcAbstractInstanceReferenceValue                                     = 2234,
    /// APMC-ABSTRACT-REFERENCE-VALUE
    ApmcAbstractReferenceValue                                             = 2493,
    /// APMC-BOOLEAN-PARAM-DEF
    ApmcBooleanParamDef                                                    = 2246,
    /// APMC-CHOICE-CONTAINER-DEF
    ApmcChoiceContainerDef                                                 = 178,
    /// APMC-CHOICE-CONTAINER-REFERENCE-DEF
    ApmcChoiceContainerReferenceDef                                        = 1889,
    /// APMC-CONFIGURATION-ELEMENT-DEF
    ApmcConfigurationElementDef                                            = 1330,
    /// APMC-CONTAINER-DEF
    ApmcContainerDef                                                       = 2154,
    /// APMC-CONTAINER-ELEMENT-DEF
    ApmcContainerElementDef                                                = 1454,
    /// APMC-CONTAINER-ELEMENT-VALUE
    ApmcContainerElementValue                                              = 2721,
    /// APMC-CONTAINER-REFERENCE-DEF
    ApmcContainerReferenceDef                                              = 1987,
    /// APMC-CONTAINER-REFERENCE-VALUE
    ApmcContainerReferenceValue                                            = 776,
    /// APMC-CONTAINER-VALUE
    ApmcContainerValue                                                     = 473,
    /// APMC-DEFINITION-COLLECTION
    ApmcDefinitionCollection                                               = 88,
    /// APMC-ENUMERATION-LITERAL-DEF
    ApmcEnumerationLiteralDef                                              = 2727,
    /// APMC-ENUMERATION-PARAM-DEF
    ApmcEnumerationParamDef                                                = 1468,
    /// APMC-FLOAT-PARAM-DEF
    ApmcFloatParamDef                                                      = 702,
    /// APMC-FOREIGN-REFERENCE-DEF
    ApmcForeignReferenceDef                                                = 2147,
    /// APMC-FOREIGN-REFERENCE-VALUE
    ApmcForeignReferenceValue                                              = 43,
    /// APMC-FUNCTIONAL-CLUSTER-DEF
    ApmcFunctionalClusterDef                                               = 645,
    /// APMC-FUNCTIONAL-CLUSTER-VALUE
    ApmcFunctionalClusterValue                                             = 235,
    /// APMC-INSTANCE-REFERENCE-DEF
    ApmcInstanceReferenceDef                                               = 753,
    /// APMC-INSTANCE-REFERENCE-VALUE
    ApmcInstanceReferenceValue                                             = 1301,
    /// APMC-INTEGER-PARAM-DEF
    ApmcIntegerParamDef                                                    = 1532,
    /// APMC-NUMERICAL-PARAM-VALUE
    ApmcNumericalParamValue                                                = 490,
    /// APMC-PARAM-CONF-CONTAINER-DEF
    ApmcParamConfContainerDef                                              = 2666,
    /// APMC-PARAMETER-DEF
    ApmcParameterDef                                                       = 878,
    /// APMC-PARAMETER-VALUE
    ApmcParameterValue                                                     = 1935,
    /// APMC-REFERENCE-DEF
    ApmcReferenceDef                                                       = 1779,
    /// APMC-REFERENCE-VALUE
    ApmcReferenceValue                                                     = 390,
    /// APMC-STRING-PARAM-DEF
    ApmcStringParamDef                                                     = 1904,
    /// APMC-TEXTUAL-PARAM-VALUE
    ApmcTextualParamValue                                                  = 976,
    /// APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-DEF
    ApmcUpstreamDocForeignReferenceDef                                     = 2722,
    /// APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-VALUE
    ApmcUpstreamDocForeignReferenceValue                                   = 2523,
    /// APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-DEF
    ApmcUpstreamDocInstanceReferenceDef                                    = 1893,
    /// APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-VALUE
    ApmcUpstreamDocInstanceReferenceValue                                  = 2064,
    /// APMC-URI-FOREIGN-REFERENCE-DEF
    ApmcUriForeignReferenceDef                                             = 1050,
    /// APMC-URI-FOREIGN-REFERENCE-VALUE
    ApmcUriForeignReferenceValue                                           = 2307,
    /// APMC-URI-INSTANCE-REFERENCE-DEF
    ApmcUriInstanceReferenceDef                                            = 2301,
    /// APMC-URI-INSTANCE-REFERENCE-VALUE
    ApmcUriInstanceReferenceValue                                          = 7,
    /// APMC-VALUE-COLLECTION
    ApmcValueCollection                                                    = 166,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 444,
    /// APPLICABILITY-INFO-SET
    ApplicabilityInfoSet                                                   = 1766,
    /// APPLICATION
    Application                                                            = 2339,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 185,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 1500,
    /// APPLICATION-ARRAY-ELEMENT
    ApplicationArrayElement                                                = 279,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 1928,
    /// APPLICATION-ASSOC-MAP-ELEMENT
    ApplicationAssocMapElement                                             = 1764,
    /// APPLICATION-COMPOSITE-DATA-TYPE
    ApplicationCompositeDataType                                           = 2377,
    /// APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE
    ApplicationCompositeElementDataPrototype                               = 104,
    /// APPLICATION-DATA-TYPE
    ApplicationDataType                                                    = 2191,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 616,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 780,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 126,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 1496,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 1270,
    /// APPLICATION-ONLY
    ApplicationOnly                                                        = 2325,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 1207,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 612,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 1586,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 213,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 2640,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 1412,
    /// APPLIED-STANDARD
    AppliedStandard                                                        = 1707,
    /// AR
    Ar                                                                     = 2508,
    /// AR--CLIENT--SERVER
    ArClientServer                                                         = 1901,
    /// AR-ELEMENT
    ArElement                                                              = 296,
    /// AR-PACKAGE
    ArPackage                                                              = 2692,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 191,
    /// ARBITRATION
    Arbitration                                                            = 731,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 46,
    /// ARRAY
    Array                                                                  = 1258,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 1135,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 1705,
    /// ARTIFACT-LOCATOR
    ArtifactLocator                                                        = 415,
    /// AS
    As                                                                     = 627,
    /// AS-IS
    AsIs                                                                   = 1095,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 2262,
    /// ASYMMETRIC-FROM-BYTE-ARRAY
    AsymmetricFromByteArray                                                = 2244,
    /// ASYMMETRIC-TO-BYTE-ARRAY
    AsymmetricToByteArray                                                  = 1266,
    /// ASYNCHRONOUS
    Asynchronous                                                           = 607,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 2188,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 835,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 2162,
    /// ATOMIC-SW-COMPONENT-TYPE
    AtomicSwComponentType                                                  = 2197,
    /// ATP-BLUEPRINT
    AtpBlueprint                                                           = 258,
    /// ATP-BLUEPRINTABLE
    AtpBlueprintable                                                       = 2324,
    /// ATP-CLASSIFIER
    AtpClassifier                                                          = 1042,
    /// ATP-DEFINITION
    AtpDefinition                                                          = 406,
    /// ATP-FEATURE
    AtpFeature                                                             = 1761,
    /// ATP-PROTOTYPE
    AtpPrototype                                                           = 2687,
    /// ATP-STRUCTURE-ELEMENT
    AtpStructureElement                                                    = 1652,
    /// ATP-TYPE
    AtpType                                                                = 683,
    /// ATTRIBUTE-TAILORING
    AttributeTailoring                                                     = 637,
    /// AUDIO-SAMPLE
    AudioSample                                                            = 1331,
    /// AUTHENTICATE
    Authenticate                                                           = 2283,
    /// AUTO
    Auto                                                                   = 366,
    /// AUTO-IP
    AutoIp                                                                 = 1912,
    /// AUTO-IP--DOIP
    AutoIpDoip                                                             = 1615,
    /// AUTO-IPDHCPV-4
    AutoIpdhcpv4                                                           = 102,
    /// AUTOMATIC
    Automatic                                                              = 1663,
    /// AUTONOMOUS
    Autonomous                                                             = 718,
    /// AUTOSAR-DATA-PROTOTYPE
    AutosarDataPrototype                                                   = 1907,
    /// AUTOSAR-DATA-TYPE
    AutosarDataType                                                        = 2757,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 680,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 2259,
    /// AVB--IEEE-802--1-AS
    AvbIeee802_1As                                                         = 5,
    /// AY
    Ay                                                                     = 2041,
    /// AZ
    Az                                                                     = 2231,
    /// BA
    Ba                                                                     = 837,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 2734,
    /// BAM
    Bam                                                                    = 1156,
    /// BAMCMDT
    Bamcmdt                                                                = 1562,
    /// BASE-T
    BaseT                                                                  = 1744,
    /// BASE-TYPE
    BaseType                                                               = 1368,
    /// BASIC-SOFTWARE-MODE-MANAGER
    BasicSoftwareModeManager                                               = 1504,
    /// BAYER-BGGR
    BayerBggr                                                              = 1540,
    /// BAYER-GBRG
    BayerGbrg                                                              = 1414,
    /// BAYER-GRBG
    BayerGrbg                                                              = 13,
    /// BAYER-RGGB
    BayerRggb                                                              = 829,
    /// BE
    Be                                                                     = 1007,
    /// BEST-EFFORT
    BestEffort                                                             = 131,
    /// BG
    Bg                                                                     = 799,
    /// BH
    Bh                                                                     = 2634,
    /// BI
    Bi                                                                     = 2710,
    /// BIDIRECTIONAL
    Bidirectional                                                          = 1063,
    /// BINARY-MANIFEST-ADDRESSABLE-OBJECT
    BinaryManifestAddressableObject                                        = 1456,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 866,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 269,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 52,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 686,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 97,
    /// BINARY-MANIFEST-RESOURCE
    BinaryManifestResource                                                 = 2749,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 1146,
    /// BLINK-MODE
    BlinkMode                                                              = 1134,
    /// BLINK-OR-CONTINUOUS-ON-MODE
    BlinkOrContinuousOnMode                                                = 1205,
    /// BLOCK
    Block                                                                  = 2631,
    /// BLOCK-SOURCE
    BlockSource                                                            = 380,
    /// BLOCK-STATE
    BlockState                                                             = 267,
    /// BLUEPRINT-DERIVATION-TIME
    BlueprintDerivationTime                                                = 2505,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 828,
    /// BMP
    Bmp                                                                    = 2086,
    /// BN
    Bn                                                                     = 2779,
    /// BO
    Bo                                                                     = 93,
    /// BOLD
    Bold                                                                   = 2373,
    /// BOLDITALIC
    Bolditalic                                                             = 700,
    /// BONJOUR
    Bonjour                                                                = 1175,
    /// BOTTOM
    Bottom                                                                 = 1491,
    /// BR
    Br                                                                     = 1352,
    /// BREAK
    Break                                                                  = 1879,
    /// BRIEF
    Brief                                                                  = 2670,
    /// BRIEF-BYPASSING-FILTERS
    BriefBypassingFilters                                                  = 2575,
    /// BROAD-R-REACH
    BroadRReach                                                            = 120,
    /// BSW
    Bsw                                                                    = 2608,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 2679,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 2118,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 433,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 1130,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 64,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 1833,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 2149,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 654,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 2281,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 1497,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 2293,
    /// BSW-EVENT
    BswEvent                                                               = 2003,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 2255,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 233,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 1863,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 1989,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 1622,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 1979,
    /// BSW-INTERRUPT-EVENT
    BswInterruptEvent                                                      = 1785,
    /// BSW-M-ENTRY-CALL-RETURNED
    BswMEntryCallReturned                                                  = 1828,
    /// BSW-M-ENTRY-CALLED
    BswMEntryCalled                                                        = 1393,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 2629,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 2638,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 741,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 1355,
    /// BSW-MODULE-CALL-POINT
    BswModuleCallPoint                                                     = 2742,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 1071,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 2391,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 362,
    /// BSW-MODULE-ENTITY
    BswModuleEntity                                                        = 111,
    /// BSW-MODULE-ENTITY-ACTIVATED
    BswModuleEntityActivated                                               = 264,
    /// BSW-MODULE-ENTITY-STARTED
    BswModuleEntityStarted                                                 = 1774,
    /// BSW-MODULE-ENTITY-TERMINATED
    BswModuleEntityTerminated                                              = 2653,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 765,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 2186,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 817,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 1601,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 2586,
    /// BSW-SCHEDULE-EVENT
    BswScheduleEvent                                                       = 549,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 658,
    /// BSW-SERVICE-DEPENDENCY-IDENT
    BswServiceDependencyIdent                                              = 377,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 158,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 2239,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 1142,
    /// BT-REC-601
    BtRec601                                                               = 1208,
    /// BT-REC-709
    BtRec709                                                               = 1149,
    /// BUILD
    Build                                                                  = 2217,
    /// BUILD-ACTION
    BuildAction                                                            = 1723,
    /// BUILD-ACTION-ENTITY
    BuildActionEntity                                                      = 1448,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 370,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 1279,
    /// BUILD-TYPE-DEBUG
    BuildTypeDebug                                                         = 2326,
    /// BUILD-TYPE-RELEASE
    BuildTypeRelease                                                       = 124,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 2057,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 2492,
    /// BUS-MIRROR-CHANNEL-MAPPING
    BusMirrorChannelMapping                                                = 1010,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 2624,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 2751,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 1629,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 516,
    /// BY-RECEPTION-TIMESTAMP
    ByReceptionTimestamp                                                   = 899,
    /// BY-SOURCE-TIMESTAMP
    BySourceTimestamp                                                      = 678,
    /// C
    C                                                                      = 1259,
    /// CA
    Ca                                                                     = 989,
    /// CALCULATED
    Calculated                                                             = 209,
    /// CALIBRATION-OFFLINE
    CalibrationOffline                                                     = 1983,
    /// CALIBRATION-ONLINE
    CalibrationOnline                                                      = 1677,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 152,
    /// CALIBRATION-VARIABLES
    CalibrationVariables                                                   = 2504,
    /// CALLBACK
    Callback                                                               = 1995,
    /// CALLOUT
    Callout                                                                = 1210,
    /// CALPRM
    Calprm                                                                 = 1786,
    /// CAN
    Can                                                                    = 1910,
    /// CAN-20
    Can20                                                                  = 1684,
    /// CAN-BE-REMOVED
    CanBeRemoved                                                           = 462,
    /// CAN-BE-TERMINATED
    CanBeTerminated                                                        = 585,
    /// CAN-BE-TERMINATED-AND-RESTARTED
    CanBeTerminatedAndRestarted                                            = 1832,
    /// CAN-BRIEF
    CanBrief                                                               = 1299,
    /// CAN-CLUSTER
    CanCluster                                                             = 1545,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 1742,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 222,
    /// CAN-FD
    CanFd                                                                  = 1249,
    /// CAN-FRAME
    CanFrame                                                               = 2034,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 1680,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 1100,
    /// CAN-NM-NODE
    CanNmNode                                                              = 2554,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 1015,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 1722,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 1683,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 1236,
    /// CAN-TP-NODE
    CanTpNode                                                              = 2400,
    /// CAN-XL-PROPS
    CanXlProps                                                             = 2514,
    /// CANCEL
    Cancel                                                                 = 981,
    /// CANCEL-CAMPAIGN
    CancelCampaign                                                         = 1437,
    /// CANNOT-BE-REMOVED
    CannotBeRemoved                                                        = 2598,
    /// CAPTION
    Caption                                                                = 2755,
    /// CAPTURE-ASYNCHRONOUS-TO-REPORTING
    CaptureAsynchronousToReporting                                         = 936,
    /// CAPTURE-ASYNCHRONOUSLY-TO-REPORTING
    CaptureAsynchronouslyToReporting                                       = 811,
    /// CAPTURE-SYNCHRONOUS-TO-REPORTING
    CaptureSynchronousToReporting                                          = 634,
    /// CAPTURE-SYNCHRONOUSLY-TO-REPORTING
    CaptureSynchronouslyToReporting                                        = 755,
    /// CAT-1
    Cat1                                                                   = 1189,
    /// CAT-2
    Cat2                                                                   = 1918,
    /// CAUTION
    Caution                                                                = 2793,
    /// CENTER
    Center                                                                 = 262,
    /// CHANNEL-A
    ChannelA                                                               = 1748,
    /// CHANNEL-B
    ChannelB                                                               = 1019,
    /// CHAPTER
    Chapter                                                                = 1551,
    /// CHARGE-MANAGER-NEEDS
    ChargeManagerNeeds                                                     = 283,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 1924,
    /// CIRCLE
    Circle                                                                 = 1856,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 1630,
    /// CLASSIC
    Classic                                                                = 596,
    /// CLEAR
    Clear                                                                  = 895,
    /// CLEAR-ALL-DTCS
    ClearAllDtcs                                                           = 2536,
    /// CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    ClearDynamicallyDefineDataIdentifier                                   = 2486,
    /// CLIENT-AUTHENTICATE
    ClientAuthenticate                                                     = 1903,
    /// CLIENT-DECRYPT
    ClientDecrypt                                                          = 2207,
    /// CLIENT-ENCRYPT
    ClientEncrypt                                                          = 1951,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 1611,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 1899,
    /// CLIENT-MAC-GENERATE
    ClientMacGenerate                                                      = 435,
    /// CLIENT-MAC-VERIFY
    ClientMacVerify                                                        = 2194,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 763,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 2266,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 1650,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 1732,
    /// CLIENT-VERIFY
    ClientVerify                                                           = 522,
    /// CLOSED
    Closed                                                                 = 1465,
    /// CM-MODULE-INSTANTIATION
    CmModuleInstantiation                                                  = 457,
    /// CMDT
    Cmdt                                                                   = 889,
    /// CO
    Co                                                                     = 827,
    /// CODE
    Code                                                                   = 1576,
    /// CODE-GENERATION-TIME
    CodeGenerationTime                                                     = 2370,
    /// CODEGENERATION
    Codegeneration                                                         = 2537,
    /// COLDSTART
    Coldstart                                                              = 1829,
    /// COLLECTABLE-ELEMENT
    CollectableElement                                                     = 218,
    /// COLLECTION
    Collection                                                             = 2318,
    /// COLOR-AWARE
    ColorAware                                                             = 1870,
    /// COLOR-BLIND
    ColorBlind                                                             = 107,
    /// COM-AXIS
    ComAxis                                                                = 1312,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 384,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 2465,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 1602,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 1051,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 1728,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 1758,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 17,
    /// COM-GRANT
    ComGrant                                                               = 2453,
    /// COM-GRANT-DESIGN
    ComGrantDesign                                                         = 2507,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 1950,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 882,
    /// COM-MANAGER
    ComManager                                                             = 859,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 815,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 168,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 1604,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 1525,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 1420,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 2077,
    /// COM-TRIGGER-GRANT
    ComTriggerGrant                                                        = 1379,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 1700,
    /// COMM-CONNECTOR-PORT
    CommConnectorPort                                                      = 2471,
    /// COMMAND-LINE-LONG-FORM
    CommandLineLongForm                                                    = 1846,
    /// COMMAND-LINE-SHORT-FORM
    CommandLineShortForm                                                   = 1302,
    /// COMMAND-LINE-SIMPLE-FORM
    CommandLineSimpleForm                                                  = 2096,
    /// COMMON
    Common                                                                 = 2497,
    /// COMMUNICATION-CLUSTER
    CommunicationCluster                                                   = 1384,
    /// COMMUNICATION-CONNECTOR
    CommunicationConnector                                                 = 1869,
    /// COMMUNICATION-CONTROLLER
    CommunicationController                                                = 2466,
    /// COMMUNICATION-INTER-ECU
    CommunicationInterEcu                                                  = 629,
    /// COMMUNICATION-INTRA-PARTITION
    CommunicationIntraPartition                                            = 767,
    /// COMPILE
    Compile                                                                = 682,
    /// COMPILER
    Compiler                                                               = 1685,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 1640,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 1669,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 809,
    /// COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING
    CompositionPortToExecutablePortMapping                                 = 53,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 2279,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 2517,
    /// COMPU-METHOD
    CompuMethod                                                            = 3,
    /// COM_AXIS
    Comaxis                                                                = 2611,
    /// CONCRETE
    Concrete                                                               = 977,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 2336,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 2091,
    /// CONDITIONAL
    Conditional                                                            = 1940,
    /// CONFIDENTIALITY-OFFSET--0
    ConfidentialityOffset0                                                 = 2576,
    /// CONFIDENTIALITY-OFFSET--30
    ConfidentialityOffset30                                                = 1772,
    /// CONFIDENTIALITY-OFFSET--50
    ConfidentialityOffset50                                                = 463,
    /// CONFIG-DATA
    ConfigData                                                             = 669,
    /// CONFIGURED
    Configured                                                             = 1233,
    /// CONFIRMED
    Confirmed                                                              = 1243,
    /// CONFIRMED-DTC-BIT
    ConfirmedDtcBit                                                        = 1425,
    /// CONNECT
    Connect                                                                = 1129,
    /// CONSISTENCY-MECHANISM-REQUIRED
    ConsistencyMechanismRequired                                           = 831,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 2316,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 1895,
    /// CONSOLE
    Console                                                                = 1274,
    /// CONST
    Const                                                                  = 202,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 670,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 1628,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 1094,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 2534,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 933,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 562,
    /// CONSUMER
    Consumer                                                               = 661,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 2632,
    /// CONTINUE-AT-IT-POSITION
    ContinueAtItPosition                                                   = 2633,
    /// CONTINUOUS-ON-MODE
    ContinuousOnMode                                                       = 1587,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 2192,
    /// COUPLING-ELEMENT-ABSTRACT-DETAILS
    CouplingElementAbstractDetails                                         = 2094,
    /// COUPLING-ELEMENT-SWITCH-DETAILS
    CouplingElementSwitchDetails                                           = 2107,
    /// COUPLING-PORT
    CouplingPort                                                           = 1982,
    /// COUPLING-PORT-ABSTRACT-SHAPER
    CouplingPortAbstractShaper                                             = 703,
    /// COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER
    CouplingPortAsynchronousTrafficShaper                                  = 559,
    /// COUPLING-PORT-CREDIT-BASED-SHAPER
    CouplingPortCreditBasedShaper                                          = 1138,
    /// COUPLING-PORT-ENHANCED-TRAFFIC-SHAPER
    CouplingPortEnhancedTrafficShaper                                      = 947,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 589,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 606,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 2639,
    /// COUPLING-PORT-STRUCTURAL-ELEMENT
    CouplingPortStructuralElement                                          = 0,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 937,
    /// CP
    Cp                                                                     = 1826,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 903,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 995,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 867,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 1985,
    /// CP-SOFTWARE-CLUSTER-RESOURCE
    CpSoftwareClusterResource                                              = 944,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 1195,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 1159,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 529,
    /// CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterToApplicationPartitionMapping                         = 1093,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 336,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 1914,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 1445,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 1058,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 2117,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 984,
    /// CPP
    Cpp                                                                    = 1561,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 1145,
    /// CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET
    CppImplementationDataTypeContextTarget                                 = 500,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 1808,
    /// CRC-IGNORED
    CrcIgnored                                                             = 26,
    /// CRC-NOT-SUPPORTED
    CrcNotSupported                                                        = 1027,
    /// CRC-NOT-VALIDATED
    CrcNotValidated                                                        = 2482,
    /// CRC-OPTIONAL
    CrcOptional                                                            = 1838,
    /// CRC-SUPPORTED
    CrcSupported                                                           = 368,
    /// CRC-VALIDATED
    CrcValidated                                                           = 2641,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 48,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 1830,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 2178,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 132,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 2428,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 2545,
    /// CRYPTO-INTERFACE
    CryptoInterface                                                        = 304,
    /// CRYPTO-JOB
    CryptoJob                                                              = 1484,
    /// CRYPTO-KEY-MANAGEMENT
    CryptoKeyManagement                                                    = 2023,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 2464,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 2601,
    /// CRYPTO-KEY-SLOT-CLIENT-INTERFACE
    CryptoKeySlotClientInterface                                           = 2662,
    /// CRYPTO-KEY-SLOT-DESIGN
    CryptoKeySlotDesign                                                    = 2573,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 1397,
    /// CRYPTO-KEY-SLOT-TO-CLIENT-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToClientPortPrototypeMapping                              = 909,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 2185,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN
    CryptoKeySlotUsageDesign                                               = 422,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN-MAPPING
    CryptoKeySlotUsageDesignMapping                                        = 766,
    /// CRYPTO-KEY-SLOT-USER-DESIGN
    CryptoKeySlotUserDesign                                                = 618,
    /// CRYPTO-KEY-SLOT-USER-DESIGN-MAPPING
    CryptoKeySlotUserDesignMapping                                         = 2459,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 742,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 1544,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 595,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 2098,
    /// CRYPTO-NEEDS
    CryptoNeeds                                                            = 2105,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 196,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 1850,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 676,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 698,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 512,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 2202,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 707,
    /// CRYPTO-SERVICE-MANAGER
    CryptoServiceManager                                                   = 1439,
    /// CRYPTO-SERVICE-MAPPING
    CryptoServiceMapping                                                   = 2112,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 1269,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 584,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 2635,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 45,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 648,
    /// CS
    Cs                                                                     = 2681,
    /// CSERS
    Csers                                                                  = 2216,
    /// CURVE-AXIS
    CurveAxis                                                              = 1180,
    /// CURVE_AXIS
    Curveaxis                                                              = 2612,
    /// CUSTOM
    Custom                                                                 = 556,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 1781,
    /// CVC
    Cvc                                                                    = 1385,
    /// CY
    Cy                                                                     = 1273,
    /// CYCLE-REPETITION-1
    CycleRepetition1                                                       = 2738,
    /// CYCLE-REPETITION-10
    CycleRepetition10                                                      = 310,
    /// CYCLE-REPETITION-16
    CycleRepetition16                                                      = 1892,
    /// CYCLE-REPETITION-2
    CycleRepetition2                                                       = 2100,
    /// CYCLE-REPETITION-20
    CycleRepetition20                                                      = 1930,
    /// CYCLE-REPETITION-32
    CycleRepetition32                                                      = 424,
    /// CYCLE-REPETITION-4
    CycleRepetition4                                                       = 256,
    /// CYCLE-REPETITION-40
    CycleRepetition40                                                      = 779,
    /// CYCLE-REPETITION-5
    CycleRepetition5                                                       = 363,
    /// CYCLE-REPETITION-50
    CycleRepetition50                                                      = 597,
    /// CYCLE-REPETITION-64
    CycleRepetition64                                                      = 733,
    /// CYCLE-REPETITION-8
    CycleRepetition8                                                       = 1444,
    /// CYCLIC
    Cyclic                                                                 = 1599,
    /// CYCLIC-AND-ON-CHANGE
    CyclicAndOnChange                                                      = 2038,
    /// DA
    Da                                                                     = 1527,
    /// DATA-CONSTR
    DataConstr                                                             = 1624,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 2538,
    /// DATA-FORMAT-ELEMENT-REFERENCE
    DataFormatElementReference                                             = 491,
    /// DATA-FORMAT-ELEMENT-SCOPE
    DataFormatElementScope                                                 = 460,
    /// DATA-INTERFACE
    DataInterface                                                          = 1577,
    /// DATA-PROTOTYPE
    DataPrototype                                                          = 2584,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 992,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPS-IDENT
    DataPrototypeTransformationPropsIdent                                  = 371,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 364,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 311,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 1106,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 1079,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 2129,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 1148,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 2599,
    /// DCM-I-PDU
    DcmIPdu                                                                = 579,
    /// DDS-CP-CONFIG
    DdsCpConfig                                                            = 1101,
    /// DDS-CP-CONSUMED-SERVICE-INSTANCE
    DdsCpConsumedServiceInstance                                           = 1087,
    /// DDS-CP-DOMAIN
    DdsCpDomain                                                            = 275,
    /// DDS-CP-PARTITION
    DdsCpPartition                                                         = 1030,
    /// DDS-CP-PROVIDED-SERVICE-INSTANCE
    DdsCpProvidedServiceInstance                                           = 1648,
    /// DDS-CP-QOS-PROFILE
    DdsCpQosProfile                                                        = 2013,
    /// DDS-CP-SERVICE-INSTANCE
    DdsCpServiceInstance                                                   = 1961,
    /// DDS-CP-TOPIC
    DdsCpTopic                                                             = 49,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 308,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 2263,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 536,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 902,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 1726,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 2394,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 1247,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 810,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 1754,
    /// DDS-SERVICE
    DdsService                                                             = 87,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 1733,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 253,
    /// DDS-SIGNAL
    DdsSignal                                                              = 2059,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 712,
    /// DE
    De                                                                     = 1953,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 2125,
    /// DEBOUNCE-DATA
    DebounceData                                                           = 2060,
    /// DEBUG
    Debug                                                                  = 494,
    /// DECREASING
    Decreasing                                                             = 1698,
    /// DEDICATED
    Dedicated                                                              = 134,
    /// DEF-ITEM
    DefItem                                                                = 1248,
    /// DEFAULT
    Default                                                                = 1371,
    /// DEFAULT-ERROR-TRACER
    DefaultErrorTracer                                                     = 2306,
    /// DEFAULT-IF-REVISION-UPDATE
    DefaultIfRevisionUpdate                                                = 674,
    /// DEFAULT-IF-UNDEFINED
    DefaultIfUndefined                                                     = 752,
    /// DEFAULT-MODE
    DefaultMode                                                            = 247,
    /// DEFAULT-TRACE-STATE-DISABLED
    DefaultTraceStateDisabled                                              = 395,
    /// DEFAULT-TRACE-STATE-ENABLED
    DefaultTraceStateEnabled                                               = 2212,
    /// DEFAULT-TRIGGER
    DefaultTrigger                                                         = 1201,
    /// DEFERRED
    Deferred                                                               = 495,
    /// DEFICIT-ROUND-ROBIN
    DeficitRoundRobin                                                      = 1607,
    /// DEFINE-BY-IDENTIFIER
    DefineByIdentifier                                                     = 2440,
    /// DEFINE-BY-MEMORY-ADDRESS
    DefineByMemoryAddress                                                  = 2126,
    /// DEFLATE
    Deflate                                                                = 1365,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 414,
    /// DELETE
    Delete                                                                 = 1790,
    /// DEPENDANT
    Dependant                                                              = 650,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 1480,
    /// DERIVED-FROM
    DerivedFrom                                                            = 1692,
    /// DESCENDANT
    Descendant                                                             = 1287,
    /// DESELECTED
    Deselected                                                             = 1854,
    /// DETAILED
    Detailed                                                               = 1321,
    /// DETAILED-BYPASSING-FILTERS
    DetailedBypassingFilters                                               = 2058,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 1708,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 2302,
    /// DETERMINISTIC-SYNC-INSTANTIATION
    DeterministicSyncInstantiation                                         = 2445,
    /// DETERMINISTIC-SYNC-MASTER
    DeterministicSyncMaster                                                = 2136,
    /// DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING
    DeterministicSyncMasterToTimeBaseConsumerMapping                       = 1486,
    /// DEVELOPMENT
    Development                                                            = 2099,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 479,
    /// DEVELOPMENT-ERROR-TRACER
    DevelopmentErrorTracer                                                 = 1281,
    /// DHCPV-4
    Dhcpv4                                                                 = 2329,
    /// DHCPV-6
    Dhcpv6                                                                 = 768,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 1221,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 1241,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 738,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 2085,
    /// DIAG-REQUEST
    DiagRequest                                                            = 2650,
    /// DIAG-RESPONSE
    DiagResponse                                                           = 869,
    /// DIAGNOSTIC-ABSTRACT-ALIAS-EVENT
    DiagnosticAbstractAliasEvent                                           = 778,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER
    DiagnosticAbstractDataIdentifier                                       = 2351,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE
    DiagnosticAbstractDataIdentifierInterface                              = 1503,
    /// DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE
    DiagnosticAbstractRoutineInterface                                     = 1222,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 959,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 1395,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 514,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE
    DiagnosticAuthTransmitCertificate                                      = 1845,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION
    DiagnosticAuthTransmitCertificateEvaluation                            = 2803,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING
    DiagnosticAuthTransmitCertificateMapping                               = 1727,
    /// DIAGNOSTIC-AUTHENTICATION
    DiagnosticAuthentication                                               = 2033,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 1113,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 2268,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 751,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 1574,
    /// DIAGNOSTIC-CAPABILITY-ELEMENT
    DiagnosticCapabilityElement                                            = 1152,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 2717,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 471,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 534,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 2535,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 1548,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 2241,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 1547,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 2156,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 466,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 892,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 456,
    /// DIAGNOSTIC-COMMON-ELEMENT
    DiagnosticCommonElement                                                = 2209,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER
    DiagnosticCommunicationManager                                         = 2746,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 2571,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 1909,
    /// DIAGNOSTIC-CONDITION
    DiagnosticCondition                                                    = 533,
    /// DIAGNOSTIC-CONDITION-GROUP
    DiagnosticConditionGroup                                               = 1188,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 2678,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 2796,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 2184,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 2433,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 656,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 1898,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 1738,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 978,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 1166,
    /// DIAGNOSTIC-DATA-BY-IDENTIFIER
    DiagnosticDataByIdentifier                                             = 1254,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 2561,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 1501,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 954,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 1005,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 843,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 408,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 1452,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 2446,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 1202,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 2348,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 2713,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 2043,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 230,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-PORT-MAPPING
    DiagnosticDoIpActivationLinePortMapping                                = 561,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE
    DiagnosticDoIpEntityIdentificationInterface                            = 30,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-PORT-MAPPING
    DiagnosticDoIpEntityIdentificationPortMapping                          = 2499,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 1020,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-PORT-MAPPING
    DiagnosticDoIpGroupIdentificationPortMapping                           = 2018,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 142,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-PORT-MAPPING
    DiagnosticDoIpPowerModePortMapping                                     = 771,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 2214,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-PORT-MAPPING
    DiagnosticDoIpTriggerVehicleAnnouncementPortMapping                    = 1392,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 685,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 590,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 263,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 2655,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 2406,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 189,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 133,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 2627,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 1862,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 231,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 2754,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 1558,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 724,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 2228,
    /// DIAGNOSTIC-ENV-MODE-ELEMENT
    DiagnosticEnvModeElement                                               = 615,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 1104,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 2155,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 2303,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 327,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 2798,
    /// DIAGNOSTIC-EVENT-MANAGER
    DiagnosticEventManager                                                 = 122,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 916,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 2789,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 289,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 2123,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 1356,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 244,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 1464,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 1398,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 103,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 1200,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 2365,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 1509,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 2331,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 1257,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 1092,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 2407,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 1874,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 292,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 198,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 2393,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 2128,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 388,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 1173,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 1238,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 812,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 2761,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 1943,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 631,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 1711,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 1032,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 74,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 2785,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 743,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 1103,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 740,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 2035,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 1614,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 1668,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 1288,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 2189,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 322,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 2179,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 2381,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 1658,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 1984,
    /// DIAGNOSTIC-LOG-AND-TRACE
    DiagnosticLogAndTrace                                                  = 1244,
    /// DIAGNOSTIC-MAPPING
    DiagnosticMapping                                                      = 1925,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 2295,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 2198,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 2363,
    /// DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS
    DiagnosticMemoryAddressableRangeAccess                                 = 2680,
    /// DIAGNOSTIC-MEMORY-BY-ADDRESS
    DiagnosticMemoryByAddress                                              = 1521,
    /// DIAGNOSTIC-MEMORY-DESTINATION
    DiagnosticMemoryDestination                                            = 290,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 192,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 2088,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 2747,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 1389,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 930,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 2341,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 454,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE
    DiagnosticMultipleConditionInterface                                   = 468,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING
    DiagnosticMultipleConditionPortMapping                                 = 1897,
    /// DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE
    DiagnosticMultipleEventInterface                                       = 2097,
    /// DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING
    DiagnosticMultipleEventPortMapping                                     = 136,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE
    DiagnosticMultipleMonitorInterface                                     = 2170,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING
    DiagnosticMultipleMonitorPortMapping                                   = 409,
    /// DIAGNOSTIC-MULTIPLE-RESOURCE-INTERFACE
    DiagnosticMultipleResourceInterface                                    = 1311,
    /// DIAGNOSTIC-MULTIPLE-RESOURCE-PORT-MAPPING
    DiagnosticMultipleResourcePortMapping                                  = 2541,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 2609,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 245,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 2134,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 982,
    /// DIAGNOSTIC-PARAMETER-ELEMENT
    DiagnosticParameterElement                                             = 2569,
    /// DIAGNOSTIC-PARAMETER-IDENT
    DiagnosticParameterIdent                                               = 2457,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 2647,
    /// DIAGNOSTIC-PORT-INTERFACE
    DiagnosticPortInterface                                                = 1678,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 69,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 2526,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 1481,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 1478,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 18,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 1408,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 789,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 804,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 2310,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 1182,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 999,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 625,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 4,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 525,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 1067,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 1402,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 2315,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 1796,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 1260,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 531,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 1383,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 1483,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 351,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 1619,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 594,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 1519,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE
    DiagnosticRequestFileTransferInterface                                 = 1303,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 2636,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 2802,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 988,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 164,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 2444,
    /// DIAGNOSTIC-REQUEST-ROUTINE-RESULTS
    DiagnosticRequestRoutineResults                                        = 2781,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 1285,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 12,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 2567,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 1651,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 1351,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 1807,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 2646,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 157,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 1373,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 1896,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 1811,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 2042,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 1900,
    /// DIAGNOSTIC-ROUTINE-SUBFUNCTION
    DiagnosticRoutineSubfunction                                           = 2413,
    /// DIAGNOSTIC-SECURE-CODING-MAPPING
    DiagnosticSecureCodingMapping                                          = 1573,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 2261,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 228,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 1691,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 2257,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 489,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 2382,
    /// DIAGNOSTIC-SERVICE-CLASS
    DiagnosticServiceClass                                                 = 1132,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 1418,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 27,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 2408,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 400,
    /// DIAGNOSTIC-SERVICE-INSTANCE
    DiagnosticServiceInstance                                              = 2807,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 1317,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 849,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 416,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 2173,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 2543,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 344,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 1605,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 1112,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE
    DiagnosticSovdAuthorizationInterface                                   = 586,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING
    DiagnosticSovdAuthorizationPortMapping                                 = 1831,
    /// DIAGNOSTIC-SOVD-BULK-DATA
    DiagnosticSovdBulkData                                                 = 193,
    /// DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE
    DiagnosticSovdBulkDataInterface                                        = 854,
    /// DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING
    DiagnosticSovdBulkDataPortMapping                                      = 2292,
    /// DIAGNOSTIC-SOVD-CONFIGURATION
    DiagnosticSovdConfiguration                                            = 748,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA
    DiagnosticSovdConfigurationBulkData                                    = 1442,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING
    DiagnosticSovdConfigurationDataIdentifierMapping                       = 109,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE
    DiagnosticSovdConfigurationInterface                                   = 1400,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER
    DiagnosticSovdConfigurationParameter                                   = 2115,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING
    DiagnosticSovdConfigurationPortMapping                                 = 2139,
    /// DIAGNOSTIC-SOVD-FAULT-MEMORY-ACCESS
    DiagnosticSovdFaultMemoryAccess                                        = 1890,
    /// DIAGNOSTIC-SOVD-LOCK
    DiagnosticSovdLock                                                     = 801,
    /// DIAGNOSTIC-SOVD-LOG
    DiagnosticSovdLog                                                      = 609,
    /// DIAGNOSTIC-SOVD-METHOD
    DiagnosticSovdMethod                                                   = 2006,
    /// DIAGNOSTIC-SOVD-METHOD-PRIMITIVE
    DiagnosticSovdMethodPrimitive                                          = 154,
    /// DIAGNOSTIC-SOVD-PORT-INTERFACE
    DiagnosticSovdPortInterface                                            = 2390,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE
    DiagnosticSovdProximityChallengeInterface                              = 704,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING
    DiagnosticSovdProximityChallengePortMapping                            = 475,
    /// DIAGNOSTIC-SOVD-SERVICE-INSTANCE
    DiagnosticSovdServiceInstance                                          = 2108,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE
    DiagnosticSovdServiceValidationInterface                               = 2313,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING
    DiagnosticSovdServiceValidationPortMapping                             = 2525,
    /// DIAGNOSTIC-SOVD-UPDATE
    DiagnosticSovdUpdate                                                   = 2366,
    /// DIAGNOSTIC-SOVD-UPDATE-INTERFACE
    DiagnosticSovdUpdateInterface                                          = 2317,
    /// DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING
    DiagnosticSovdUpdatePortMapping                                        = 324,
    /// DIAGNOSTIC-START-ROUTINE
    DiagnosticStartRoutine                                                 = 1654,
    /// DIAGNOSTIC-STOP-ROUTINE
    DiagnosticStopRoutine                                                  = 941,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 1539,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 2141,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 2027,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 1278,
    /// DIAGNOSTIC-SW-MAPPING
    DiagnosticSwMapping                                                    = 2285,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 1996,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 1751,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 2205,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 1582,
    /// DIAGNOSTIC-TRANSMIT-CERTIFICATE-INTERFACE
    DiagnosticTransmitCertificateInterface                                 = 2481,
    /// DIAGNOSTIC-TROUBLE-CODE
    DiagnosticTroubleCode                                                  = 1313,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 1090,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 268,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 2235,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 2238,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 171,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 2353,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 2613,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 483,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 1802,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 2380,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 382,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 2579,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 1199,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 745,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 880,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 858,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 593,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 2110,
    /// DISABLE
    Disable                                                                = 1177,
    /// DLNA
    Dlna                                                                   = 2360,
    /// DLT-APPLICATION
    DltApplication                                                         = 1818,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 2120,
    /// DLT-ARGUMENT
    DltArgument                                                            = 1006,
    /// DLT-CONTEXT
    DltContext                                                             = 853,
    /// DLT-ECU
    DltEcu                                                                 = 2371,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 2772,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 2072,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 2135,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 868,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 1646,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 746,
    /// DLT-MESSAGE
    DltMessage                                                             = 2443,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 848,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 277,
    /// DO-IP
    DoIp                                                                   = 1003,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 2102,
    /// DO-IP-FUNCTIONAL-CLUSTER-DESIGN
    DoIpFunctionalClusterDesign                                            = 2741,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 2765,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 1954,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 141,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 1488,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 1441,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 1957,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 571,
    /// DO-IP-LOGICAL-ADDRESS
    DoIpLogicalAddress                                                     = 1672,
    /// DO-IP-NETWORK-CONFIGURATION-DESIGN
    DoIpNetworkConfigurationDesign                                         = 1934,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 347,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 321,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 86,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 1978,
    /// DO-IP-SERVICE-NEEDS
    DoIpServiceNeeds                                                       = 70,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 2752,
    /// DO-NOT-INCLUDE
    DoNotInclude                                                           = 846,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 2794,
    /// DOCUMENTATION
    Documentation                                                          = 1131,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 2411,
    /// DOES-NOT-REPORT-EXECUTION-STATE
    DoesNotReportExecutionState                                            = 375,
    /// DOES-NOT-SUPPORT-BUFFER-LOCKING
    DoesNotSupportBufferLocking                                            = 1136,
    /// DOES-NOT-USE-LOGGING
    DoesNotUseLogging                                                      = 749,
    /// DOMAIN-PARTICIPANT-USER-DATA-QOS
    DomainParticipantUserDataQos                                           = 1291,
    /// DONT-INVALIDATE
    DontInvalidate                                                         = 555,
    /// DROP
    Drop                                                                   = 1372,
    /// DROP-FRAME
    DropFrame                                                              = 1505,
    /// DROP-UNTAGGED
    DropUntagged                                                           = 1154,
    /// DSA
    Dsa                                                                    = 333,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 288,
    /// DYNAMIC
    Dynamic                                                                = 1606,
    /// DYNAMIC-PART-TRIGGER
    DynamicPartTrigger                                                     = 966,
    /// DZ
    Dz                                                                     = 1213,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 884,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 940,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 358,
    /// ECC
    Ecc                                                                    = 2590,
    /// ECU
    Ecu                                                                    = 1688,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 1048,
    /// ECU-INSTANCE
    EcuInstance                                                            = 2284,
    /// ECU-MANAGER
    EcuManager                                                             = 1021,
    /// ECU-MAPPING
    EcuMapping                                                             = 639,
    /// ECU-PARTITION
    EcuPartition                                                           = 2119,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 2298,
    /// ECU-TIMING
    EcuTiming                                                              = 2589,
    /// ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF
    EcucAbstractExternalReferenceDef                                       = 1568,
    /// ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF
    EcucAbstractInternalReferenceDef                                       = 1530,
    /// ECUC-ABSTRACT-REFERENCE-DEF
    EcucAbstractReferenceDef                                               = 2730,
    /// ECUC-ABSTRACT-STRING-PARAM-DEF
    EcucAbstractStringParamDef                                             = 960,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 1143,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 2658,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 2396,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 519,
    /// ECUC-COMMON-ATTRIBUTES
    EcucCommonAttributes                                                   = 1263,
    /// ECUC-CONTAINER-DEF
    EcucContainerDef                                                       = 528,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 1413,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 2221,
    /// ECUC-DEFINITION-ELEMENT
    EcucDefinitionElement                                                  = 2215,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 1018,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 1565,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 821,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 883,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 1111,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 709,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 874,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 355,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 1348,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 1776,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 690,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 186,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 1538,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 2277,
    /// ECUC-PARAMETER-DEF
    EcucParameterDef                                                       = 2750,
    /// ECUC-QUERY
    EcucQuery                                                              = 1956,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 2087,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 156,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 2078,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 735,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 1589,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 76,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 800,
    /// EDGE-NODE
    EdgeNode                                                               = 114,
    /// EID-USE-API
    EidUseApi                                                              = 1307,
    /// EID-USE-CONFIG-VALUE
    EidUseConfigValue                                                      = 28,
    /// EID-USE-MAC
    EidUseMac                                                              = 2524,
    /// EL
    El                                                                     = 1571,
    /// EMISSION-RELATED-DTC
    EmissionRelatedDtc                                                     = 1419,
    /// EN
    En                                                                     = 1969,
    /// ENABLE
    Enable                                                                 = 2760,
    /// ENABLED
    Enabled                                                                = 1245,
    /// ENCRYPT-AND-SIGN
    EncryptAndSign                                                         = 1137,
    /// ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION
    EncryptAndSignWithOriginAuthentication                                 = 2805,
    /// ENCRYPTION
    Encryption                                                             = 118,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 1546,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 2166,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 1533,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 2144,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 1963,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 1693,
    /// ENHANCED
    Enhanced                                                               = 1812,
    /// ENHANCED-TRAFFIC-SHAPER
    EnhancedTrafficShaper                                                  = 487,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 649,
    /// EO
    Eo                                                                     = 2031,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 2668,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 2614,
    /// EOC-EXECUTABLE-ENTITY-REF-ABSTRACT
    EocExecutableEntityRefAbstract                                         = 1127,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 2689,
    /// EPS
    Eps                                                                    = 80,
    /// EQUAL
    Equal                                                                  = 1295,
    /// ERROR
    Error                                                                  = 1585,
    /// ERROR-CORRECTION
    ErrorCorrection                                                        = 506,
    /// ERROR-DETECTION
    ErrorDetection                                                         = 2,
    /// ERROR-TRACER
    ErrorTracer                                                            = 665,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 250,
    /// ES
    Es                                                                     = 626,
    /// ESP
    Esp                                                                    = 1179,
    /// ET
    Et                                                                     = 527,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 1300,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 979,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 1314,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 548,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 517,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 1004,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 1631,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 1242,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 922,
    /// ETHERNET-MAC-RAW-DATA-STREAM-MAPPING
    EthernetMacRawDataStreamMapping                                        = 159,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 2582,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 2050,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 1797,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 708,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 181,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 2012,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 602,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 1554,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 469,
    /// ETP
    Etp                                                                    = 1022,
    /// EU
    Eu                                                                     = 635,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 657,
    /// EVAP
    Evap                                                                   = 980,
    /// EVAPPURGEFLOW
    Evappurgeflow                                                          = 2473,
    /// EVENT-ACCEPTANCE-DISABLED
    EventAcceptanceDisabled                                                = 1417,
    /// EVENT-ACCEPTANCE-ENABLED
    EventAcceptanceEnabled                                                 = 411,
    /// EVENT-COMBINATION-ON-RETRIEVAL
    EventCombinationOnRetrieval                                            = 1121,
    /// EVENT-COMBINATION-ON-STORAGE
    EventCombinationOnStorage                                              = 1231,
    /// EVENT-HANDLER
    EventHandler                                                           = 1805,
    /// EVENT-MAPPING
    EventMapping                                                           = 1938,
    /// EVENT-STORAGE-DISABLED
    EventStorageDisabled                                                   = 234,
    /// EVENT-STORAGE-ENABLED
    EventStorageEnabled                                                    = 1809,
    /// EVENT-TRIGGERING-CONSTRAINT
    EventTriggeringConstraint                                              = 2356,
    /// EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE
    EventWindowCurrentAndFollowingCycle                                    = 187,
    /// EVENT-WINDOW-CURRENT-CYCLE
    EventWindowCurrentCycle                                                = 2062,
    /// EVENT-WINDOW-INFINITE
    EventWindowInfinite                                                    = 2232,
    /// EXACT-OR-ANY-MINOR-VERSION
    ExactOrAnyMinorVersion                                                 = 188,
    /// EXAMPLE
    Example                                                                = 2695,
    /// EXCLUDE-FROM-FLASH
    ExcludeFromFlash                                                       = 1689,
    /// EXCLUSIVE
    Exclusive                                                              = 2288,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 2304,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 2684,
    /// EXECUTABLE
    Executable                                                             = 1239,
    /// EXECUTABLE-ENTITY
    ExecutableEntity                                                       = 1458,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 926,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 2297,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 2733,
    /// EXECUTE
    Execute                                                                = 39,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 545,
    /// EXECUTION-TIME
    ExecutionTime                                                          = 905,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 664,
    /// EXERCISE
    Exercise                                                               = 2699,
    /// EXPLICIT
    Explicit                                                               = 71,
    /// EXPRESS
    Express                                                                = 211,
    /// EXTEND
    Extend                                                                 = 1428,
    /// EXTENDED
    Extended                                                               = 1057,
    /// EXTERNAL-REPLACEMENT
    ExternalReplacement                                                    = 934,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 2300,
    /// EXTERNAL-TRIGGERING-POINT-IDENT
    ExternalTriggeringPointIdent                                           = 1814,
    /// FA
    Fa                                                                     = 402,
    /// FAILURE-AND-SUCCESS
    FailureAndSuccess                                                      = 1994,
    /// FAILURE-ONLY
    FailureOnly                                                            = 2286,
    /// FALSE
    False                                                                  = 865,
    /// FAST-FLASHING-MODE
    FastFlashingMode                                                       = 1232,
    /// FATAL
    Fatal                                                                  = 983,
    /// FAULT
    Fault                                                                  = 1741,
    /// FDBAM
    Fdbam                                                                  = 1052,
    /// FDBAMCMDT
    Fdbamcmdt                                                              = 946,
    /// FDC-THRESHOLD
    FdcThreshold                                                           = 2321,
    /// FDCMDT
    Fdcmdt                                                                 = 1347,
    /// FI
    Fi                                                                     = 1119,
    /// FIBEX-ELEMENT
    FibexElement                                                           = 666,
    /// FIELD
    Field                                                                  = 1775,
    /// FIELD-MAPPING
    FieldMapping                                                           = 896,
    /// FILE
    File                                                                   = 2540,
    /// FILTERED
    Filtered                                                               = 242,
    /// FINISH
    Finish                                                                 = 2143,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 2084,
    /// FIRE-AND-FORGET-METHOD-MAPPING
    FireAndForgetMethodMapping                                             = 2439,
    /// FIREWALL-RULE
    FirewallRule                                                           = 2673,
    /// FIREWALL-STATE-SWITCH-INTERFACE
    FirewallStateSwitchInterface                                           = 476,
    /// FIRST-CONTAINED-TRIGGER
    FirstContainedTrigger                                                  = 1118,
    /// FIRST-TO-SECOND
    FirstToSecond                                                          = 1218,
    /// FIT-TO-PAGE
    FitToPage                                                              = 1593,
    /// FIT-TO-TEXT
    FitToText                                                              = 1386,
    /// FIX-AXIS
    FixAxis                                                                = 183,
    /// FIXED
    Fixed                                                                  = 1942,
    /// FIXED-SIZE
    FixedSize                                                              = 1151,
    /// FIX_AXIS
    Fixaxis                                                                = 1271,
    /// FJ
    Fj                                                                     = 2305,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 1471,
    /// FLAT-MAP
    FlatMap                                                                = 407,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 1843,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 438,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 2226,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 508,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 1905,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 1580,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 219,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 513,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 1596,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 2052,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 2715,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 503,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 2442,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 261,
    /// FLOAT
    Float                                                                  = 2349,
    /// FLOAT-32-BIT
    Float32Bit                                                             = 1584,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 2513,
    /// FM-FEATURE
    FmFeature                                                              = 932,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 737,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 574,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 1959,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 1613,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 91,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 572,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 1396,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 2203,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 8,
    /// FO
    Fo                                                                     = 127,
    /// FOR-ALL
    ForAll                                                                 = 439,
    /// FORGET
    Forget                                                                 = 2385,
    /// FORWARD-AS-IS
    ForwardAsIs                                                            = 1703,
    /// FPP
    Fpp                                                                    = 1364,
    /// FR
    Fr                                                                     = 931,
    /// FRAME
    Frame                                                                  = 2588,
    /// FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION
    FrameEthernetQueuedForTransmission                                     = 2758,
    /// FRAME-ETHERNET-RECEIVED-BY-IF
    FrameEthernetReceivedByIf                                              = 1045,
    /// FRAME-ETHERNET-RECEIVED-ON-BUS
    FrameEthernetReceivedOnBus                                             = 1627,
    /// FRAME-ETHERNET-SENT-ON-BUS
    FrameEthernetSentOnBus                                                 = 1047,
    /// FRAME-PORT
    FramePort                                                              = 904,
    /// FRAME-QUEUED-FOR-TRANSMISSION
    FrameQueuedForTransmission                                             = 610,
    /// FRAME-RECEIVED-BY-IF
    FrameReceivedByIf                                                      = 756,
    /// FRAME-TRANSMITTED-ON-BUS
    FrameTransmittedOnBus                                                  = 2587,
    /// FRAME-TRIGGERING
    FrameTriggering                                                        = 155,
    /// FULL
    Full                                                                   = 2140,
    /// FULL-COM
    FullCom                                                                = 281,
    /// FULL-DUPLEX-MODE
    FullDuplexMode                                                         = 1974,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 2392,
    /// FUNCTION-GROUP-PORT-MAPPING
    FunctionGroupPortMapping                                               = 1451,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 1126,
    /// FUNCTION-GROUP-STATE-TO-NM-HANDLE
    FunctionGroupStateToNmHandle                                           = 116,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 2024,
    /// FUNCTION-INHIBITION-MANAGER
    FunctionInhibitionManager                                              = 29,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 95,
    /// FUNCTIONAL
    Functional                                                             = 1091,
    /// FUNCTIONAL-ADDRESS
    FunctionalAddress                                                      = 1223,
    /// FUNCTIONAL-CAN-FD
    FunctionalCanFd                                                        = 844,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-DIAGNOSTIC-EVENT-MAPPING
    FunctionalClusterInteractsWithDiagnosticEventMapping                   = 1939,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING
    FunctionalClusterInteractsWithFunctionalClusterMapping                 = 1608,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING
    FunctionalClusterInteractsWithPersistencyDeploymentMapping             = 1320,
    /// FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING
    FunctionalClusterToSecurityEventDefinitionMapping                      = 1038,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 2271,
    /// FY
    Fy                                                                     = 2564,
    /// GA
    Ga                                                                     = 802,
    /// GATEWAY
    Gateway                                                                = 2430,
    /// GD
    Gd                                                                     = 1513,
    /// GENERAL-PARAMETER
    GeneralParameter                                                       = 58,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 1268,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 2617,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 2591,
    /// GENERAL-PURPOSE-TIMER-SERVICE-NEEDS
    GeneralPurposeTimerServiceNeeds                                        = 642,
    /// GENERIC-DIAGNOSTIC-TRANSPORT-INSTANTIATION
    GenericDiagnosticTransportInstantiation                                = 532,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 343,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 1555,
    /// GENERIC-TP-CONNECTION
    GenericTpConnection                                                    = 2799,
    /// GET
    Get                                                                    = 251,
    /// GETTER
    Getter                                                                 = 2553,
    /// GETTER-SETTER
    GetterSetter                                                           = 2199,
    /// GIF
    Gif                                                                    = 25,
    /// GL
    Gl                                                                     = 298,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 286,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 777,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 2158,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 2737,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 1463,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 1343,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 1293,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 619,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 551,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 1350,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 1882,
    /// GLOBAL-TIME-MASTER
    GlobalTimeMaster                                                       = 1931,
    /// GLOBAL-TIME-SLAVE
    GlobalTimeSlave                                                        = 323,
    /// GN
    Gn                                                                     = 2273,
    /// GRANT
    Grant                                                                  = 1659,
    /// GRANT-DESIGN
    GrantDesign                                                            = 2578,
    /// GRAYSCALE
    Grayscale                                                              = 381,
    /// GROSS
    Gross                                                                  = 1713,
    /// GU
    Gu                                                                     = 1998,
    /// GZIP
    Gzip                                                                   = 2171,
    /// HA
    Ha                                                                     = 1536,
    /// HALF-DUPLEX-MODE
    HalfDuplexMode                                                         = 2574,
    /// HARDWARE-TEST-MANAGER
    HardwareTestManager                                                    = 346,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 1662,
    /// HEAD
    Head                                                                   = 2039,
    /// HEALTH-CHANNEL
    HealthChannel                                                          = 2355,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 1024,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 1054,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 2055,
    /// HEAP-USAGE
    HeapUsage                                                              = 2496,
    /// HI
    Hi                                                                     = 137,
    /// HIERARCHICAL-EOC
    HierarchicalEoc                                                        = 2001,
    /// HIGH
    High                                                                   = 1997,
    /// HINT
    Hint                                                                   = 2619,
    /// HOOK
    Hook                                                                   = 929,
    /// HOST-PORT
    HostPort                                                               = 2030,
    /// HR
    Hr                                                                     = 2487,
    /// HU
    Hu                                                                     = 203,
    /// HUB
    Hub                                                                    = 898,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 1316,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 72,
    /// HW-CATEGORY
    HwCategory                                                             = 2151,
    /// HW-DESCRIPTION-ENTITY
    HwDescriptionEntity                                                    = 2193,
    /// HW-ELEMENT
    HwElement                                                              = 2724,
    /// HW-PIN
    HwPin                                                                  = 1077,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 1881,
    /// HW-TYPE
    HwType                                                                 = 1190,
    /// HY
    Hy                                                                     = 956,
    /// I-4-G
    I4G                                                                    = 1999,
    /// I-PDU
    IPdu                                                                   = 374,
    /// I-PDU-PORT
    IPduPort                                                               = 485,
    /// I-PDU-RECEIVED-BY-COM
    IPduReceivedByCom                                                      = 723,
    /// I-PDU-SENT-TO-IF
    IPduSentToIf                                                           = 1633,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 824,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 961,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 1706,
    /// I-SIGNAL
    ISignal                                                                = 923,
    /// I-SIGNAL-AVAILABLE-FOR-RTE
    ISignalAvailableForRte                                                 = 2163,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 1737,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 757,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 1908,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 300,
    /// I-SIGNAL-SENT-TO-COM
    ISignalSentToCom                                                       = 2175,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 486,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 1564,
    /// IA
    Ia                                                                     = 872,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 2559,
    /// ICMP
    Icmp                                                                   = 305,
    /// ICV-IGNORED
    IcvIgnored                                                             = 1153,
    /// ICV-NOT-SUPPORTED
    IcvNotSupported                                                        = 373,
    /// ICV-NOT-VERIFIED
    IcvNotVerified                                                         = 21,
    /// ICV-OPTIONAL
    IcvOptional                                                            = 2768,
    /// ICV-SUPPORTED
    IcvSupported                                                           = 806,
    /// ICV-VERIFIED
    IcvVerified                                                            = 1720,
    /// IDENT-CAPTION
    IdentCaption                                                           = 1114,
    /// IDENTIFIABLE
    Identifiable                                                           = 795,
    /// IDS-COMMON-ELEMENT
    IdsCommonElement                                                       = 2729,
    /// IDS-DESIGN
    IdsDesign                                                              = 106,
    /// IDS-MAPPING
    IdsMapping                                                             = 1187,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 57,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 834,
    /// IDS-PLATFORM-INSTANTIATION
    IdsPlatformInstantiation                                               = 2806,
    /// IDSM-ABSTRACT-PORT-INTERFACE
    IdsmAbstractPortInterface                                              = 1422,
    /// IDSM-CONTEXT-PROVIDER-INTERFACE
    IdsmContextProviderInterface                                           = 1799,
    /// IDSM-CONTEXT-PROVIDER-MAPPING
    IdsmContextProviderMapping                                             = 2676,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 165,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 2053,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 1225,
    /// IDSM-QUALIFIED-EVENT-RECEIVER-INTERFACE
    IdsmQualifiedEventReceiverInterface                                    = 1697,
    /// IDSM-QUALIFIED-EVENT-RECEIVER-MAPPING
    IdsmQualifiedEventReceiverMapping                                      = 762,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 2528,
    /// IDSM-REPORTING-MODE-PROVIDER-INTERFACE
    IdsmReportingModeProviderInterface                                     = 1917,
    /// IDSM-REPORTING-MODE-PROVIDER-MAPPING
    IdsmReportingModeProviderMapping                                       = 736,
    /// IDSM-TIMESTAMP-PROVIDER-INTERFACE
    IdsmTimestampProviderInterface                                         = 1717,
    /// IDSM-TIMESTAMP-PROVIDER-MAPPING
    IdsmTimestampProviderMapping                                           = 1595,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 968,
    /// IE
    Ie                                                                     = 2480,
    /// IEC-61937
    Iec61937                                                               = 2509,
    /// IEEE-1722-ACF-BUS-PART-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722AcfBusPartRawDataStreamConsumerMapping                         = 419,
    /// IEEE-1722-ACF-BUS-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722AcfBusRawDataStreamConsumerMapping                             = 1262,
    /// IEEE-1722-RAW-DATA-STREAM-CONSUMER-INTERFACE
    Ieee1722RawDataStreamConsumerInterface                                 = 974,
    /// IEEE-1722-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722RawDataStreamConsumerMapping                                   = 2274,
    /// IEEE-1722-RAW-DATA-STREAM-MAPPING
    Ieee1722RawDataStreamMapping                                           = 890,
    /// IEEE-1722-RAW-DATA-STREAM-PRODUCER-INTERFACE
    Ieee1722RawDataStreamProducerInterface                                 = 1319,
    /// IEEE-1722-RAW-DATA-STREAM-PRODUCER-MAPPING
    Ieee1722RawDataStreamProducerMapping                                   = 2474,
    /// IEEE-1722-TP-AAF-CONNECTION
    Ieee1722TpAafConnection                                                = 303,
    /// IEEE-1722-TP-ACF-BUS
    Ieee1722TpAcfBus                                                       = 2594,
    /// IEEE-1722-TP-ACF-BUS-PART
    Ieee1722TpAcfBusPart                                                   = 791,
    /// IEEE-1722-TP-ACF-CAN
    Ieee1722TpAcfCan                                                       = 101,
    /// IEEE-1722-TP-ACF-CAN-PART
    Ieee1722TpAcfCanPart                                                   = 2005,
    /// IEEE-1722-TP-ACF-CONNECTION
    Ieee1722TpAcfConnection                                                = 641,
    /// IEEE-1722-TP-ACF-LIN
    Ieee1722TpAcfLin                                                       = 2000,
    /// IEEE-1722-TP-ACF-LIN-PART
    Ieee1722TpAcfLinPart                                                   = 2427,
    /// IEEE-1722-TP-AV-CONNECTION
    Ieee1722TpAvConnection                                                 = 621,
    /// IEEE-1722-TP-CONFIG
    Ieee1722TpConfig                                                       = 1638,
    /// IEEE-1722-TP-CONNECTION
    Ieee1722TpConnection                                                   = 1591,
    /// IEEE-1722-TP-CRF-CONNECTION
    Ieee1722TpCrfConnection                                                = 63,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 1336,
    /// IEEE-1722-TP-IIDC-CONNECTION
    Ieee1722TpIidcConnection                                               = 1575,
    /// IEEE-1722-TP-RVF-CONNECTION
    Ieee1722TpRvfConnection                                                = 655,
    /// IEEE802-11P
    Ieee802_11p                                                            = 338,
    /// IEEE802-1AS
    Ieee802_1as                                                            = 167,
    /// IEEE802-1AS-AUTOSAR
    Ieee802_1asAutosar                                                     = 1001,
    /// IGNITION
    Ignition                                                               = 927,
    /// IGNORE
    Ignore                                                                 = 1409,
    /// IK
    Ik                                                                     = 1081,
    /// IMMEDIATE
    Immediate                                                              = 2010,
    /// IMPLEMENTATION
    Implementation                                                         = 1125,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 328,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 2551,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 1264,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 1346,
    /// IMPLEMENTATION-PROPS
    ImplementationProps                                                    = 543,
    /// IMPOSITION-TIME
    ImpositionTime                                                         = 2200,
    /// IMPOSITION-TIME-DEFINITION-GROUP
    ImpositionTimeDefinitionGroup                                          = 1511,
    /// IN
    In                                                                     = 1721,
    /// INCLUDE-BUT-DO-NOT-START
    IncludeButDoNotStart                                                   = 2490,
    /// INCREASING
    Increasing                                                             = 1139,
    /// INDEPENDENT-VLAN-LEARNING
    IndependentVlanLearning                                                = 2606,
    /// INDICATE
    Indicate                                                               = 331,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 1610,
    /// INDIVIDUAL
    Individual                                                             = 2312,
    /// INFINITE
    Infinite                                                               = 1367,
    /// INFINITE-TIME-TO-RESPONSE
    InfiniteTimeToResponse                                                 = 1756,
    /// INFO
    Info                                                                   = 481,
    /// INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE
    InheritedFromArrayElementTypeSize                                      = 448,
    /// INIT-EVENT
    InitEvent                                                              = 2478,
    /// INLINE
    Inline                                                                 = 2441,
    /// INLINE-CONDITIONAL
    InlineConditional                                                      = 719,
    /// INOUT
    Inout                                                                  = 994,
    /// INSTALL
    Install                                                                = 991,
    /// INSTANCE-ID
    InstanceId                                                             = 1390,
    /// INSTRUCTION
    Instruction                                                            = 1064,
    /// INT-16-BIT
    Int16Bit                                                               = 608,
    /// INT-24-BIT
    Int24Bit                                                               = 1866,
    /// INT-32-BIT
    Int32Bit                                                               = 499,
    /// INTER-LET-ONLY
    InterLetOnly                                                           = 1220,
    /// INTER-PARTITION-INTRA-ECU
    InterPartitionIntraEcu                                                 = 1911,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 2529,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 1140,
    /// INTERGRITY-AND-CONFIDENTIALITY
    IntergrityAndConfidentiality                                           = 825,
    /// INTERGRITY-WITHOUT-CONFIDENTIALITY
    IntergrityWithoutConfidentiality                                       = 730,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 2020,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 396,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 624,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 863,
    /// INTERRUPT
    Interrupt                                                              = 2153,
    /// INTERRUPT-CAT-1
    InterruptCat1                                                          = 11,
    /// INTERRUPT-CAT-2
    InterruptCat2                                                          = 958,
    /// INTRA-LET-EOC
    IntraLetEoc                                                            = 1012,
    /// INTRUSION-DETECTION-SECURITY-MANAGEMENT
    IntrusionDetectionSecurityManagement                                   = 1966,
    /// INVALID
    Invalid                                                                = 993,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 1823,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 2763,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 2603,
    /// IP-SEC-RULE
    IpSecRule                                                              = 2201,
    /// IPSEC
    Ipsec                                                                  = 1023,
    /// IS
    Is                                                                     = 272,
    /// IS-EQUAL
    IsEqual                                                                = 1040,
    /// IS-EXPIRED
    IsExpired                                                              = 1842,
    /// IS-FAILED
    IsFailed                                                               = 2291,
    /// IS-GREATER-OR-EQUAL
    IsGreaterOrEqual                                                       = 2704,
    /// IS-GREATER-THAN
    IsGreaterThan                                                          = 2320,
    /// IS-GREATER-THAN-OR-EQUAL
    IsGreaterThanOrEqual                                                   = 967,
    /// IS-LESS-OR-EQUAL
    IsLessOrEqual                                                          = 1431,
    /// IS-LESS-THAN
    IsLessThan                                                             = 2503,
    /// IS-LESS-THAN-OR-EQUAL
    IsLessThanOrEqual                                                      = 329,
    /// IS-NOT-EQUAL
    IsNotEqual                                                             = 1847,
    /// IS-NOT-RELEVANT
    IsNotRelevant                                                          = 566,
    /// IS-OK
    IsOk                                                                   = 1080,
    /// IS-RELEVANT
    IsRelevant                                                             = 1237,
    /// IS-STOPPED
    IsStopped                                                              = 1543,
    /// ISO
    Iso                                                                    = 2809,
    /// ISO-11992--4
    Iso11992_4                                                             = 833,
    /// ISO-14229--1
    Iso14229_1                                                             = 117,
    /// ISO-15031--6
    Iso15031_6                                                             = 1704,
    /// ISO-6
    Iso6                                                                   = 2716,
    /// IT
    It                                                                     = 341,
    /// ITALIC
    Italic                                                                 = 1286,
    /// ITU-BT-2020
    ItuBt2020                                                              = 393,
    /// IW
    Iw                                                                     = 881,
    /// J-1939
    J1939                                                                  = 2498,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 295,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 1851,
    /// J-1939-DCM
    J1939Dcm                                                               = 2712,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 2426,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 391,
    /// J-1939-NM--AAC
    J1939NmAac                                                             = 547,
    /// J-1939-NM--CCA
    J1939NmCca                                                             = 335,
    /// J-1939-NM--NCA
    J1939NmNca                                                             = 1752,
    /// J-1939-NM--SCA
    J1939NmSca                                                             = 2460,
    /// J-1939-NM--SVCA
    J1939NmSvca                                                            = 1280,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 2417,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 1875,
    /// J-1939-PROTECTED-I-PDU
    J1939ProtectedIPdu                                                     = 893,
    /// J-1939-REQUEST-MANAGER
    J1939RequestManager                                                    = 2079,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 1407,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 1096,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 1089,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 2270,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 841,
    /// JA
    Ja                                                                     = 237,
    /// JAVA
    Java                                                                   = 319,
    /// JI
    Ji                                                                     = 405,
    /// JPG
    Jpg                                                                    = 361,
    /// JUSTIFY
    Justify                                                                = 773,
    /// JW
    Jw                                                                     = 1623,
    /// KA
    Ka                                                                     = 1436,
    /// KEEP
    Keep                                                                   = 879,
    /// KEEP-ALL
    KeepAll                                                                = 581,
    /// KEEP-EXISTING
    KeepExisting                                                           = 814,
    /// KEEP-LAST
    KeepLast                                                               = 2164,
    /// KEY-DERIVATION
    KeyDerivation                                                          = 2220,
    /// KEY-SERVER
    KeyServer                                                              = 1949,
    /// KEY-STORAGE
    KeyStorage                                                             = 2161,
    /// KEYWORD
    Keyword                                                                = 2625,
    /// KEYWORD-SET
    KeywordSet                                                             = 194,
    /// KK
    Kk                                                                     = 2745,
    /// KL
    Kl                                                                     = 44,
    /// KM
    Km                                                                     = 173,
    /// KN
    Kn                                                                     = 259,
    /// KO
    Ko                                                                     = 515,
    /// KS
    Ks                                                                     = 1570,
    /// KU
    Ku                                                                     = 1827,
    /// KY
    Ky                                                                     = 169,
    /// LA
    La                                                                     = 1115,
    /// LAND
    Land                                                                   = 484,
    /// LAST-FAILED
    LastFailed                                                             = 1857,
    /// LAST-IS-BEST
    LastIsBest                                                             = 318,
    /// LAST-MODE
    LastMode                                                               = 2429,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 2550,
    /// LEAF-OF-TARGET-CONTAINER
    LeafOfTargetContainer                                                  = 1556,
    /// LEFT
    Left                                                                   = 2146,
    /// LEGACY
    Legacy                                                                 = 474,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 659,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 1746,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 651,
    /// LIGHT
    Light                                                                  = 1765,
    /// LIMIT-TO-PAGE
    LimitToPage                                                            = 238,
    /// LIMIT-TO-TEXT
    LimitToText                                                            = 2461,
    /// LIN-CLUSTER
    LinCluster                                                             = 1315,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 697,
    /// LIN-COMMUNICATION-CONTROLLER
    LinCommunicationController                                             = 2436,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 2327,
    /// LIN-FRAME
    LinFrame                                                               = 1401,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 2566,
    /// LIN-MASTER
    LinMaster                                                              = 1215,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 734,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 461,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 1535,
    /// LIN-SLAVE
    LinSlave                                                               = 788,
    /// LIN-SLAVE-CONFIG-IDENT
    LinSlaveConfigIdent                                                    = 1214,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 1141,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 2080,
    /// LIN-TP-NODE
    LinTpNode                                                              = 2004,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 2416,
    /// LINK
    Link                                                                   = 826,
    /// LINK-LOCAL
    LinkLocal                                                              = 105,
    /// LINK-LOCAL--DOIP
    LinkLocalDoip                                                          = 1025,
    /// LINK-TIME
    LinkTime                                                               = 1171,
    /// LINKER
    Linker                                                                 = 1403,
    /// LISTEN
    Listen                                                                 = 2669,
    /// LN
    Ln                                                                     = 803,
    /// LO
    Lo                                                                     = 901,
    /// LOCAL
    Local                                                                  = 1065,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 1860,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 845,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 1757,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 1739,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 2454,
    /// LOGICAL-AND
    LogicalAnd                                                             = 660,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 1043,
    /// LOGICAL-OR
    LogicalOr                                                              = 313,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 1224,
    /// LONG-HEADER
    LongHeader                                                             = 1357,
    /// LOW
    Low                                                                    = 1510,
    /// LOWER-12-BIT
    Lower12Bit                                                             = 632,
    /// LOWER-8-BIT
    Lower8Bit                                                              = 505,
    /// LT
    Lt                                                                     = 2718,
    /// LT-AFFECTS-PB
    LtAffectsPb                                                            = 1524,
    /// LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING
    LtMessageCollectionToPortPrototypeMapping                              = 1457,
    /// LTS-13
    Lts13                                                                  = 420,
    /// LV
    Lv                                                                     = 2422,
    /// MAC-ADDRESS-VLAN-MEMBERSHIP
    MacAddressVlanMembership                                               = 2485,
    /// MAC-LAYER-RAW-DATA-STREAM-INTERFACE
    MacLayerRawDataStreamInterface                                         = 908,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 1361,
    /// MAC-SEC-GLOBAL-KAY-PROPS
    MacSecGlobalKayProps                                                   = 2552,
    /// MAC-SEC-KAY-PARTICIPANT
    MacSecKayParticipant                                                   = 1334,
    /// MAC-SEC-PARTICIPANT-SET
    MacSecParticipantSet                                                   = 578,
    /// MACHINE
    Machine                                                                = 2660,
    /// MACHINE-CYCLE
    MachineCycle                                                           = 2437,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 1387,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 935,
    /// MACHINE-TIMING
    MachineTiming                                                          = 1886,
    /// MACRO
    Macro                                                                  = 2269,
    /// MALFUNCTION
    Malfunction                                                            = 1734,
    /// MANUAL-BY-PARTICIPANT
    ManualByParticipant                                                    = 1792,
    /// MANUAL-BY-TOPIC
    ManualByTopic                                                          = 1835,
    /// MANUFACTURING
    Manufacturing                                                          = 2463,
    /// MAPPING-SCOPE-CORE
    MappingScopeCore                                                       = 35,
    /// MAPPING-SCOPE-ECU
    MappingScopeEcu                                                        = 41,
    /// MAPPING-SCOPE-PARTITION
    MappingScopePartition                                                  = 227,
    /// MASEKD-NEW-EQUALS-MASKED-OLD
    MasekdNewEqualsMaskedOld                                               = 842,
    /// MASEKD-NEW-EQUALS-X
    MasekdNewEqualsX                                                       = 1729,
    /// MASKED-NEW-DIFFERS-MASKED-OLD
    MaskedNewDiffersMaskedOld                                              = 1083,
    /// MASKED-NEW-DIFFERS-X
    MaskedNewDiffersX                                                      = 679,
    /// MASKED-NEW-EQUALS-MASKED-OLD
    MaskedNewEqualsMaskedOld                                               = 2510,
    /// MASKED-NEW-EQUALS-X
    MaskedNewEqualsX                                                       = 2435,
    /// MASTER
    Master                                                                 = 611,
    /// MASTER-ECU
    MasterEcu                                                              = 2388,
    /// MAX
    Max                                                                    = 1184,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 2707,
    /// MC-FUNCTION
    McFunction                                                             = 2532,
    /// MC-GROUP
    McGroup                                                                = 786,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 1836,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 458,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 838,
    /// MEASUREMENT-POINT
    MeasurementPoint                                                       = 1569,
    /// MEDIUM
    Medium                                                                 = 732,
    /// MEMORY-SECTION
    MemorySection                                                          = 1086,
    /// MEMORY-USAGE
    MemoryUsage                                                            = 1198,
    /// METHOD-MAPPING
    MethodMapping                                                          = 2495,
    /// MG
    Mg                                                                     = 9,
    /// MI
    Mi                                                                     = 667,
    /// MIDDLE
    Middle                                                                 = 1926,
    /// MIN
    Min                                                                    = 1033,
    /// MINIMUM-MINOR-VERSION
    MinimumMinorVersion                                                    = 2703,
    /// MIXED
    Mixed                                                                  = 1822,
    /// MIXED-29-BIT
    Mixed29Bit                                                             = 689,
    /// MK
    Mk                                                                     = 2516,
    /// ML
    Ml                                                                     = 1325,
    /// MN
    Mn                                                                     = 1759,
    /// MO
    Mo                                                                     = 1656,
    /// MODE-ACCESS-POINT-IDENT
    ModeAccessPointIdent                                                   = 14,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 332,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 2174,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 2106,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 1941,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 1965,
    /// MODE-DECLARATION-REQUESTED
    ModeDeclarationRequested                                               = 2367,
    /// MODE-DECLARATION-SWITCH-COMPLETED
    ModeDeclarationSwitchCompleted                                         = 1345,
    /// MODE-DECLARATION-SWITCH-INITIATED
    ModeDeclarationSwitchInitiated                                         = 897,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 1590,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 1988,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 317,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 1088,
    /// MODE-TRANSITION
    ModeTransition                                                         = 2066,
    /// MODELED
    Modeled                                                                = 928,
    /// MONITOR-MODE
    MonitorMode                                                            = 1839,
    /// MONO
    Mono                                                                   = 1487,
    /// MONOCHROME
    Monochrome                                                             = 1423,
    /// MONOTONOUS
    Monotonous                                                             = 2070,
    /// MOST-SIGNIFICANT-BYTE-FIRST
    MostSignificantByteFirst                                               = 2253,
    /// MOST-SIGNIFICANT-BYTE-LAST
    MostSignificantByteLast                                                = 1516,
    /// MR
    Mr                                                                     = 2787,
    /// MS
    Ms                                                                     = 94,
    /// MT
    Mt                                                                     = 197,
    /// MULTICORE-REENTRANT
    MulticoreReentrant                                                     = 1560,
    /// MULTILANGUAGE-REFERRABLE
    MultilanguageReferrable                                                = 1885,
    /// MULTIPLE
    Multiple                                                               = 2402,
    /// MULTIPLE-OCCURRENCES
    MultipleOccurrences                                                    = 1859,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 1160,
    /// MY
    My                                                                     = 552,
    /// N-PDU
    NPdu                                                                   = 605,
    /// NA
    Na                                                                     = 2790,
    /// NAND
    Nand                                                                   = 2138,
    /// NE
    Ne                                                                     = 398,
    /// NET
    Net                                                                    = 421,
    /// NETWORK
    Network                                                                = 50,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 1309,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 925,
    /// NETWORK-HANDLE-PORT-MAPPING
    NetworkHandlePortMapping                                               = 161,
    /// NETWORK-MANAGEMENT-PORT-INTERFACE
    NetworkManagementPortInterface                                         = 1382,
    /// NETWORK-REPRESENTATION-FROM-COM-SPEC
    NetworkRepresentationFromComSpec                                       = 54,
    /// NEVER
    Never                                                                  = 176,
    /// NEW-IS-DIFFERENT
    NewIsDifferent                                                         = 1240,
    /// NEW-IS-EQUAL
    NewIsEqual                                                             = 1598,
    /// NEW-IS-GREATER
    NewIsGreater                                                           = 542,
    /// NEW-IS-GREATER-OR-EQUAL
    NewIsGreaterOrEqual                                                    = 2089,
    /// NEW-IS-LESS
    NewIsLess                                                              = 1512,
    /// NEW-IS-LESS-OR-EQUAL
    NewIsLessOrEqual                                                       = 163,
    /// NEW-IS-OUTSIDE
    NewIsOutside                                                           = 2682,
    /// NEW-IS-WITHIN
    NewIsWithin                                                            = 2152,
    /// NEWLINE
    Newline                                                                = 1076,
    /// NEWLINE-IF-NECESSARY
    NewlineIfNecessary                                                     = 2769,
    /// NFOLD
    Nfold                                                                  = 663,
    /// NL
    Nl                                                                     = 128,
    /// NM-CLUSTER
    NmCluster                                                              = 1655,
    /// NM-CONFIG
    NmConfig                                                               = 215,
    /// NM-ECU
    NmEcu                                                                  = 601,
    /// NM-HANDLE-ACTIVE-TO-FUNCTION-GROUP-STATE
    NmHandleActiveToFunctionGroupState                                     = 1600,
    /// NM-HANDLE-INACTIVE-TO-FUNCTION-GROUP-STATE
    NmHandleInactiveToFunctionGroupState                                   = 1108,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 668,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 467,
    /// NM-INTERACTS-WITH-SM-MAPPING
    NmInteractsWithSmMapping                                               = 1687,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 1637,
    /// NM-NODE
    NmNode                                                                 = 2706,
    /// NM-PDU
    NmPdu                                                                  = 2690,
    /// NO
    No                                                                     = 710,
    /// NO-ACK
    NoAck                                                                  = 1915,
    /// NO-AFFECT
    NoAffect                                                               = 1657,
    /// NO-BOOT
    NoBoot                                                                 = 2051,
    /// NO-BREAK
    NoBreak                                                                = 1515,
    /// NO-CHECKPOINT-SUPERVISION
    NoCheckpointSupervision                                                = 720,
    /// NO-COM
    NoCom                                                                  = 1970,
    /// NO-CONSISTENCY-MECHANISM
    NoConsistencyMechanism                                                 = 924,
    /// NO-DEFAULT
    NoDefault                                                              = 246,
    /// NO-FLOAT
    NoFloat                                                                = 598,
    /// NO-HEADER
    NoHeader                                                               = 1816,
    /// NO-KEEP
    NoKeep                                                                 = 1168,
    /// NO-MONOTONY
    NoMonotony                                                             = 99,
    /// NO-NEWLINE
    NoNewline                                                              = 1209,
    /// NO-OBD-SUPPORT
    NoObdSupport                                                           = 47,
    /// NO-PGWIDE
    NoPgwide                                                               = 2022,
    /// NO-PROTECTION
    NoProtection                                                           = 1150,
    /// NO-RETURN-VALUE-PROVIDED
    NoReturnValueProvided                                                  = 546,
    /// NO-SHOW-ALIAS-NAME
    NoShowAliasName                                                        = 312,
    /// NO-SHOW-CATEGORY
    NoShowCategory                                                         = 949,
    /// NO-SHOW-CONTENT
    NoShowContent                                                          = 1,
    /// NO-SHOW-LONG-NAME
    NoShowLongName                                                         = 1074,
    /// NO-SHOW-NUMBER
    NoShowNumber                                                           = 2542,
    /// NO-SHOW-PAGE
    NoShowPage                                                             = 2431,
    /// NO-SHOW-SEE
    NoShowSee                                                              = 1435,
    /// NO-SHOW-SHORT-NAME
    NoShowShortName                                                        = 1852,
    /// NO-SHOW-TYPE
    NoShowType                                                             = 1191,
    /// NO-SLOPPY
    NoSloppy                                                               = 2597,
    /// NO-STATUS-BYTE-CHANGE
    NoStatusByteChange                                                     = 997,
    /// NO-STORE-EVENT
    NoStoreEvent                                                           = 790,
    /// NO-SUPERVISION
    NoSupervision                                                          = 1169,
    /// NO-SUPPORT
    NoSupport                                                              = 630,
    /// NO-TRANSFORMER-ERROR-HANDLING
    NoTransformerErrorHandling                                             = 2361,
    /// NO-TRANSFORMER-STATUS-FORWARDING
    NoTransformerStatusForwarding                                          = 1695,
    /// NO-TRUSTED-PLATFORM-SUPPORT
    NoTrustedPlatformSupport                                               = 1660,
    /// NODE
    Node                                                                   = 1641,
    /// NOHREF
    Nohref                                                                 = 1339,
    /// NON-EMMISSION-RELATED-DTC
    NonEmmissionRelatedDtc                                                 = 870,
    /// NON-OS-MODULE-INSTANTIATION
    NonOsModuleInstantiation                                               = 42,
    /// NON-REENTRANT
    NonReentrant                                                           = 2651,
    /// NON-VOLATILE
    NonVolatile                                                            = 955,
    /// NON-VOLATILE-RAM-MANAGER
    NonVolatileRamManager                                                  = 805,
    /// NONE
    None                                                                   = 428,
    /// NORMALFIXED
    Normalfixed                                                            = 464,
    /// NOT
    Not                                                                    = 2568,
    /// NOT-ACCESSIBLE
    NotAccessible                                                          = 1059,
    /// NOT-AVAILABLE
    NotAvailable                                                           = 2330,
    /// NOT-DEFINED
    NotDefined                                                             = 81,
    /// NOT-EQUAL
    NotEqual                                                               = 1730,
    /// NOT-SENT
    NotSent                                                                = 2774,
    /// NOT-TESTED
    NotTested                                                              = 1522,
    /// NOT-VALID
    NotValid                                                               = 1523,
    /// NOTHING
    Nothing                                                                = 2049,
    /// NOTIFICATION
    Notification                                                           = 643,
    /// NTP--RFC-958
    NtpRfc958                                                              = 59,
    /// NUMBER
    Number                                                                 = 2544,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 2697,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 427,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 2685,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 1709,
    /// NV-RAM-MANAGER
    NvRamManager                                                           = 2254,
    /// OBD
    Obd                                                                    = 591,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 1326,
    /// OBD-DCY
    ObdDcy                                                                 = 199,
    /// OBD-DRIVING-CYCLE
    ObdDrivingCycle                                                        = 1921,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 2626,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 1499,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 886,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 1844,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 135,
    /// OBSERVER
    Observer                                                               = 2222,
    /// OBSERVER-BASED
    ObserverBased                                                          = 2642,
    /// OC
    Oc                                                                     = 920,
    /// OCCURENCE
    Occurence                                                              = 2308,
    /// OEM-BOOT
    OemBoot                                                                = 2725,
    /// OEM-BOOT-RESP-APP
    OemBootRespApp                                                         = 2294,
    /// OFF
    Off                                                                    = 2401,
    /// OFFSET
    Offset                                                                 = 2708,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 2328,
    /// OM
    Om                                                                     = 2759,
    /// ON-CHANGE
    OnChange                                                               = 2211,
    /// ON-CHANGE-OF-DATA-IDENTIFIER
    OnChangeOfDataIdentifier                                               = 2546,
    /// ON-COMPARISON-OF-VALUES
    OnComparisonOfValues                                                   = 2736,
    /// ON-DTC-STATUS-CHANGE
    OnDtcStatusChange                                                      = 180,
    /// ON-ENTRY
    OnEntry                                                                = 1161,
    /// ON-EXIT
    OnExit                                                                 = 2219,
    /// ON-TRANSITION
    OnTransition                                                           = 1876,
    /// ONE-EVERY-N
    OneEveryN                                                              = 906,
    /// ONLY-THIS-CYCLE-AND-READINESS
    OnlyThisCycleAndReadiness                                              = 1359,
    /// OPAQUE
    Opaque                                                                 = 750,
    /// OPEN
    Open                                                                   = 2605,
    /// OPERATING-SYSTEM
    OperatingSystem                                                        = 1217,
    /// OPERATION-CALL-RECEIVED
    OperationCallReceived                                                  = 293,
    /// OPERATION-CALL-RESPONSE-RECEIVED
    OperationCallResponseReceived                                          = 493,
    /// OPERATION-CALL-RESPONSE-SENT
    OperationCallResponseSent                                              = 2570,
    /// OPERATION-CALLED
    OperationCalled                                                        = 243,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 617,
    /// OPTIONS
    Options                                                                = 973,
    /// OR
    Or                                                                     = 1848,
    /// ORDINARY-EOC
    OrdinaryEoc                                                            = 2539,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 915,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 2615,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 2040,
    /// OTHER
    Other                                                                  = 2036,
    /// OUT
    Out                                                                    = 1872,
    /// OVERRIDE
    Override                                                               = 1635,
    /// OVERWRITE
    Overwrite                                                              = 2714,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 622,
    /// PA
    Pa                                                                     = 1178,
    /// PACKAGEABLE-ELEMENT
    PackageableElement                                                     = 147,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 1567,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 715,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 688,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 2700,
    /// PARTIAL-NETWORK
    PartialNetwork                                                         = 1537,
    /// PARTITION
    Partition                                                              = 1056,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 1378,
    /// PASSIVE
    Passive                                                                = 359,
    /// PASSTHROUGH
    Passthrough                                                            = 2145,
    /// PAYLOAD-AS-ARRAY
    PayloadAsArray                                                         = 1960,
    /// PAYLOAD-AS-POINTER-TO-ARRAY
    PayloadAsPointerToArray                                                = 1563,
    /// PC-AFFECTS-LT
    PcAffectsLt                                                            = 129,
    /// PC-AFFECTS-LT-AND-PB
    PcAffectsLtAndPb                                                       = 1265,
    /// PC-AFFECTS-PB
    PcAffectsPb                                                            = 221,
    /// PCM
    Pcm                                                                    = 2368,
    /// PDF
    Pdf                                                                    = 2282,
    /// PDU
    Pdu                                                                    = 2332,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 2686,
    /// PDU-R
    PduR                                                                   = 1777,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 1430,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 2237,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 1541,
    /// PEER
    Peer                                                                   = 823,
    /// PENDING
    Pending                                                                = 1211,
    /// PER-EXECUTABLE
    PerExecutable                                                          = 1354,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 2076,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 34,
    /// PERIODIC-RATE-FAST
    PeriodicRateFast                                                       = 1026,
    /// PERIODIC-RATE-MEDIUM
    PeriodicRateMedium                                                     = 2278,
    /// PERIODIC-RATE-SLOW
    PeriodicRateSlow                                                       = 349,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 871,
    /// PERSISTENCY-DEPLOYMENT
    PersistencyDeployment                                                  = 145,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT
    PersistencyDeploymentElement                                           = 2350,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 583,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 1696,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 647,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 783,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 521,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 808,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 729,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 1082,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 1044,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 728,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 10,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 564,
    /// PERSISTENCY-INTERFACE
    PersistencyInterface                                                   = 2182,
    /// PERSISTENCY-INTERFACE-ELEMENT
    PersistencyInterfaceElement                                            = 1894,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 1399,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 582,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 1440,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 273,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 1338,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING
    PersistencyPortPrototypeToDeploymentMapping                            = 1632,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 1679,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 501,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 1594,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 2068,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE
    PersistencyRedundancyHandlingScopeDatabase                             = 1304,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT
    PersistencyRedundancyHandlingScopeElement                              = 1085,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE
    PersistencyRedundancyHandlingScopeFile                                 = 1597,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY
    PersistencyRedundancyHandlingScopeKey                                  = 2014,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE
    PersistencyRedundancyHandlingScopeStorage                              = 2452,
    /// PERSISTENT
    Persistent                                                             = 2419,
    /// PGWIDE
    Pgwide                                                                 = 945,
    /// PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE
    PhmAbstractRecoveryNotificationInterface                               = 249,
    /// PHM-ACTION
    PhmAction                                                              = 1670,
    /// PHM-ACTION-ITEM
    PhmActionItem                                                          = 2196,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 2547,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 307,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 1284,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 2258,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 2319,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 713,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 792,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 714,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 1666,
    /// PHM-RULE
    PhmRule                                                                = 66,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 210,
    /// PHM-SUPERVISION
    PhmSupervision                                                         = 497,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 671,
    /// PHYSICAL
    Physical                                                               = 1975,
    /// PHYSICAL-ADDRESS
    PhysicalAddress                                                        = 1219,
    /// PHYSICAL-CAN-FD
    PhysicalCanFd                                                          = 599,
    /// PHYSICAL-CHANNEL
    PhysicalChannel                                                        = 2206,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 2280,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 1531,
    /// PL
    Pl                                                                     = 996,
    /// PLAIN
    Plain                                                                  = 16,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 541,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 1946,
    /// PLATFORM-HEALTH-MANAGEMENT-INTERFACE
    PlatformHealthManagementInterface                                      = 1450,
    /// PLATFORM-MODULE-ENDPOINT-CONFIGURATION
    PlatformModuleEndpointConfiguration                                    = 2739,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 121,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 2405,
    /// PNC-MAPPING-IDENT
    PncMappingIdent                                                        = 1649,
    /// PNG
    Png                                                                    = 1482,
    /// POLY
    Poly                                                                   = 348,
    /// PORT
    Port                                                                   = 1473,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 662,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 1740,
    /// PORT-GROUP
    PortGroup                                                              = 1476,
    /// PORT-INTERFACE
    PortInterface                                                          = 2344,
    /// PORT-INTERFACE-DEFINITION
    PortInterfaceDefinition                                                = 2780,
    /// PORT-INTERFACE-MAPPING
    PortInterfaceMapping                                                   = 850,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 301,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 1716,
    /// PORT-PROTOTYPE
    PortPrototype                                                          = 204,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 2240,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 2074,
    /// POST
    Post                                                                   = 1388,
    /// POST-BUILD
    PostBuild                                                              = 498,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 2176,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 1745,
    /// POWER
    Power                                                                  = 1469,
    /// POWER-WINDOW-TIME
    PowerWindowTime                                                        = 2691,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 2180,
    /// PRE--R-4--2
    PreR4_2                                                                = 6,
    /// PRE-COMPILE
    PreCompile                                                             = 2748,
    /// PRE-COMPILE-TIME
    PreCompileTime                                                         = 372,
    /// PRECONFIGURED-CONFIGURATION
    PreconfiguredConfiguration                                             = 38,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 787,
    /// PREEMPTABLE
    Preemptable                                                            = 1492,
    /// PRESENTATION-CONTINUOUS
    PresentationContinuous                                                 = 1795,
    /// PRESENTATION-DISCRETE
    PresentationDiscrete                                                   = 1945,
    /// PRESHARED-KEY-IDENTITY
    PresharedKeyIdentity                                                   = 813,
    /// PRIMARY-ECU
    PrimaryEcu                                                             = 2075,
    /// PRIMITIVE
    Primitive                                                              = 711,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 2210,
    /// PRIO-OCC
    PrioOcc                                                                = 2702,
    /// PRIVATE-KEY
    PrivateKey                                                             = 2671,
    /// PROCESS
    Process                                                                = 2131,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 948,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 907,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 1072,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 2247,
    /// PROCESS-IS-NOT-SELF-TERMINATING
    ProcessIsNotSelfTerminating                                            = 628,
    /// PROCESS-IS-SELF-TERMINATING
    ProcessIsSelfTerminating                                               = 1529,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 1120,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 1621,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 342,
    /// PROCESSING-STYLE-ASYNCHRONOUS
    ProcessingStyleAsynchronous                                            = 1972,
    /// PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR
    ProcessingStyleAsynchronousWithError                                   = 1749,
    /// PROCESSING-STYLE-SYNCHRONOUS
    ProcessingStyleSynchronous                                             = 1046,
    /// PROCESSOR
    Processor                                                              = 975,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 2011,
    /// PRODUCER
    Producer                                                               = 1884,
    /// PROTECT-LAMP
    ProtectLamp                                                            = 220,
    /// PROTECTED
    Protected                                                              = 1432,
    /// PROVIDED-AP-SERVICE-INSTANCE
    ProvidedApServiceInstance                                              = 2469,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 1644,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 725,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 79,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 2265,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 717,
    /// PS
    Ps                                                                     = 2654,
    /// PSK
    Psk                                                                    = 1825,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 2667,
    /// PT
    Pt                                                                     = 2520,
    /// PTP--IEEE-1588--2002
    PtpIeee1588_2002                                                       = 2032,
    /// PTP--IEEE-1588--2008
    PtpIeee1588_2008                                                       = 797,
    /// PUBLIC-KEY
    PublicKey                                                              = 1332,
    /// PUBLISHED-INFORMATION
    PublishedInformation                                                   = 294,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 280,
    /// PUT
    Put                                                                    = 2340,
    /// QU
    Qu                                                                     = 37,
    /// QUEUED
    Queued                                                                 = 675,
    /// R-4--2
    R4_2                                                                   = 1470,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 2256,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 1710,
    /// RAW
    Raw                                                                    = 1665,
    /// RAW-DATA
    RawData                                                                = 2618,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 1193,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 1821,
    /// RAW-DATA-STREAM-GRANT
    RawDataStreamGrant                                                     = 2740,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 357,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 1318,
    /// RAW-DATA-STREAM-MAPPING
    RawDataStreamMapping                                                   = 276,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 1667,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 241,
    /// REACTION
    Reaction                                                               = 1416,
    /// READ
    Read                                                                   = 195,
    /// READ-ONLY
    ReadOnly                                                               = 224,
    /// READ-WRITE
    ReadWrite                                                              = 413,
    /// REBOOT
    Reboot                                                                 = 68,
    /// RECOMMENDED-CONFIGURATION
    RecommendedConfiguration                                               = 1147,
    /// RECORD-VALUE-FIELD
    RecordValueField                                                       = 440,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 2267,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 2354,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 1105,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 852,
    /// RECT
    Rect                                                                   = 123,
    /// RED-STOP-LAMP
    RedStopLamp                                                            = 1075,
    /// REDUNDANT
    Redundant                                                              = 77,
    /// REDUNDANT-PER-ELEMENT
    RedundantPerElement                                                    = 1337,
    /// REDUNDANT-PER-KEY
    RedundantPerKey                                                        = 2593,
    /// REF-ALL
    RefAll                                                                 = 125,
    /// REF-NON-STANDARD
    RefNonStandard                                                         = 985,
    /// REF-NONE
    RefNone                                                                = 426,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 694,
    /// REFERRABLE
    Referrable                                                             = 2063,
    /// REGULAR
    Regular                                                                = 2122,
    /// REJECT
    Reject                                                                 = 2701,
    /// RELIABLE
    Reliable                                                               = 2472,
    /// REMOVE
    Remove                                                                 = 1981,
    /// REPETITIVE-EOC
    RepetitiveEoc                                                          = 567,
    /// REPLACE
    Replace                                                                = 770,
    /// REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE
    ReplaceByTimeoutSubstitutionValue                                      = 1252,
    /// REPORT
    Report                                                                 = 2616,
    /// REPORT-AFTER-INIT
    ReportAfterInit                                                        = 216,
    /// REPORT-BEFORE-INIT
    ReportBeforeInit                                                       = 387,
    /// REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE
    ReportDtcRecordInformationOnDtcStatusChange                            = 2726,
    /// REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE
    ReportMostRecentDtcOnStatusChange                                      = 177,
    /// REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST
    ReportingInChronlogicalOrderOldestFirst                                = 1328,
    /// REPORTS-EXECUTION-STATE
    ReportsExecutionState                                                  = 1810,
    /// REQUEST
    Request                                                                = 1176,
    /// REQUEST-CALLBACK-TYPE-MANUFACTURER
    RequestCallbackTypeManufacturer                                        = 2743,
    /// REQUEST-CALLBACK-TYPE-SUPPLIER
    RequestCallbackTypeSupplier                                            = 1849,
    /// REQUEST-NO-RETURN
    RequestNoReturn                                                        = 1861,
    /// REQUIRED-AP-SERVICE-INSTANCE
    RequiredApServiceInstance                                              = 1363,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 759,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 1206,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 73,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 950,
    /// REQUIRES-CALLBACK-EXECUTION
    RequiresCallbackExecution                                              = 1070,
    /// RES-AXIS
    ResAxis                                                                = 2168,
    /// RESET-ECU
    ResetEcu                                                               = 1228,
    /// RESET-MACHINE
    ResetMachine                                                           = 2252,
    /// RESET-MCU
    ResetMcu                                                               = 149,
    /// RESET-VM
    ResetVm                                                                = 1327,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 297,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 1625,
    /// RESPOND-AFTER-RESET
    RespondAfterReset                                                      = 314,
    /// RESPOND-BEFORE-RESET
    RespondBeforeReset                                                     = 1235,
    /// RESPONSE
    Response                                                               = 646,
    /// RESPONSE-SYNCHRONIZATION
    ResponseSynchronization                                                = 376,
    /// REST-ABSTRACT-ENDPOINT
    RestAbstractEndpoint                                                   = 1186,
    /// REST-ABSTRACT-NUMERICAL-PROPERTY-DEF
    RestAbstractNumericalPropertyDef                                       = 1552,
    /// REST-ABSTRACT-PROPERTY-DEF
    RestAbstractPropertyDef                                                = 397,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 478,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 31,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 2124,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 470,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 2195,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 1626,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 1163,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 2581,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 1329,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 1763,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 1310,
    /// REST-PRIMITIVE-PROPERTY-DEF
    RestPrimitivePropertyDef                                               = 2786,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 1466,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 2674,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 83,
    /// RESTART
    Restart                                                                = 1366,
    /// RESTART-APPLICATION
    RestartApplication                                                     = 563,
    /// RES_AXIS
    Resaxis                                                                = 378,
    /// RETURN-ON-EVENT-CLEARED
    ReturnOnEventCleared                                                   = 1475,
    /// RETURN-ON-EVENT-STOPPED
    ReturnOnEventStopped                                                   = 1642,
    /// RETURN-VALUE-PROVIDED
    ReturnValueProvided                                                    = 2688,
    /// RIGHT
    Right                                                                  = 1008,
    /// RM
    Rm                                                                     = 1791,
    /// RN
    Rn                                                                     = 2264,
    /// RO
    Ro                                                                     = 2369,
    /// ROLL-BACK
    RollBack                                                               = 840,
    /// ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE
    RootSwClusterDesignComponentPrototype                                  = 1341,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 1937,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 570,
    /// ROTATE-180
    Rotate180                                                              = 1294,
    /// ROTATE-180-LIMIT-TO-TEXT
    Rotate180LimitToText                                                   = 2387,
    /// ROTATE-90-CCW
    Rotate90Ccw                                                            = 2621,
    /// ROTATE-90-CCW-FIT-TO-TEXT
    Rotate90CcwFitToText                                                   = 1489,
    /// ROTATE-90-CCW-LIMIT-TO-TEXT
    Rotate90CcwLimitToText                                                 = 1794,
    /// ROTATE-90-CW
    Rotate90Cw                                                             = 2190,
    /// ROTATE-90-CW-FIT-TO-TEXT
    Rotate90CwFitToText                                                    = 577,
    /// ROTATE-90-CW-LIMIT-TO-TEXT
    Rotate90CwLimitToText                                                  = 1165,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 1375,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 1172,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 2753,
    /// ROUTER
    Router                                                                 = 1559,
    /// ROUTER-ADVERTISEMENT
    RouterAdvertisement                                                    = 939,
    /// RPT-COMPONENT
    RptComponent                                                           = 1447,
    /// RPT-CONTAINER
    RptContainer                                                           = 2784,
    /// RPT-ENABLER-RAM
    RptEnablerRam                                                          = 316,
    /// RPT-ENABLER-RAM-AND-ROM
    RptEnablerRamAndRom                                                    = 437,
    /// RPT-ENABLER-ROM
    RptEnablerRom                                                          = 51,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 1681,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 2213,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 291,
    /// RPT-LEVEL-1
    RptLevel1                                                              = 1834,
    /// RPT-LEVEL-2
    RptLevel2                                                              = 2470,
    /// RPT-LEVEL-3
    RptLevel3                                                              = 2657,
    /// RPT-PROFILE
    RptProfile                                                             = 229,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 1699,
    /// RSA
    Rsa                                                                    = 2081,
    /// RTE-EVENT
    RteEvent                                                               = 19,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 443,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 1204,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 2208,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 2248,
    /// RTPGE
    Rtpge                                                                  = 2409,
    /// RU
    Ru                                                                     = 962,
    /// RULE
    Rule                                                                   = 1507,
    /// RUN-CONTINUOUS
    RunContinuous                                                          = 2531,
    /// RUN-ONCE
    RunOnce                                                                = 2705,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 1782,
    /// RUNNABLE-ENTITY-ACTIVATED
    RunnableEntityActivated                                                = 1426,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 2475,
    /// RUNNABLE-ENTITY-STARTED
    RunnableEntityStarted                                                  = 1427,
    /// RUNNABLE-ENTITY-TERMINATED
    RunnableEntityTerminated                                               = 1880,
    /// RUNNABLE-ENTITY-VARIABLE-ACCESS
    RunnableEntityVariableAccess                                           = 1653,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 2728,
    /// RW
    Rw                                                                     = 1192,
    /// RX-TRIGGER
    RxTrigger                                                              = 761,
    /// SA
    Sa                                                                     = 623,
    /// SAE-J-1939--73
    SaeJ1939_73                                                            = 1009,
    /// SAE-J-2012--DA
    SaeJ2012Da                                                             = 2467,
    /// SAFETY
    Safety                                                                 = 986,
    /// SATURATE
    Saturate                                                               = 255,
    /// SCHEDULE-VARIANT-1
    ScheduleVariant1                                                       = 2359,
    /// SCHEDULE-VARIANT-2
    ScheduleVariant2                                                       = 2412,
    /// SCHEDULE-VARIANT-3
    ScheduleVariant3                                                       = 22,
    /// SCHEDULE-VARIANT-4
    ScheduleVariant4                                                       = 1760,
    /// SCHEDULE-VARIANT-5
    ScheduleVariant5                                                       = 2111,
    /// SCHEDULE-VARIANT-6
    ScheduleVariant6                                                       = 1986,
    /// SCHEDULE-VARIANT-7
    ScheduleVariant7                                                       = 1455,
    /// SCHEDULED
    Scheduled                                                              = 964,
    /// SD
    Sd                                                                     = 820,
    /// SDG-ABSTRACT-FOREIGN-REFERENCE
    SdgAbstractForeignReference                                            = 600,
    /// SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE
    SdgAbstractPrimitiveAttribute                                          = 1955,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 560,
    /// SDG-ATTRIBUTE
    SdgAttribute                                                           = 1770,
    /// SDG-CAPTION
    SdgCaption                                                             = 96,
    /// SDG-CLASS
    SdgClass                                                               = 1433,
    /// SDG-DEF
    SdgDef                                                                 = 1116,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 1421,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 913,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 1016,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 2383,
    /// SDG-REFERENCE
    SdgReference                                                           = 1493,
    /// SDG-TAILORING
    SdgTailoring                                                           = 1675,
    /// SEARCH-FOR-ALL
    SearchForAll                                                           = 2083,
    /// SEARCH-FOR-ALL-INSTANCES
    SearchForAllInstances                                                  = 2565,
    /// SEARCH-FOR-ANY
    SearchForAny                                                           = 1671,
    /// SEARCH-FOR-ID
    SearchForId                                                            = 65,
    /// SEARCH-FOR-SPECIFIC-INSTANCE
    SearchForSpecificInstance                                              = 951,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 2628,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 2398,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 1449,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 1841,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 1592,
    /// SECOND-TO-FIRST
    SecondToFirst                                                          = 1891,
    /// SECONDARY-ECU
    SecondaryEcu                                                           = 162,
    /// SECRET-SEED
    SecretSeed                                                             = 1039,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 1438,
    /// SECURE-COM-PROPS
    SecureComProps                                                         = 1362,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 1380,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 412,
    /// SECURE-COMMUNICATION-DEPLOYMENT
    SecureCommunicationDeployment                                          = 2225,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 1793,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 385,
    /// SECURE-ON-BOARD-COMMUNICATION
    SecureOnBoardCommunication                                             = 1344,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 1308,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 1053,
    /// SECURED-PDU-HEADER-08-BIT
    SecuredPduHeader08Bit                                                  = 459,
    /// SECURED-PDU-HEADER-16-BIT
    SecuredPduHeader16Bit                                                  = 2384,
    /// SECURED-PDU-HEADER-32-BIT
    SecuredPduHeader32Bit                                                  = 1750,
    /// SECURITY
    Security                                                               = 2483,
    /// SECURITY-EVENT-AGGREGATION-FILTER
    SecurityEventAggregationFilter                                         = 1461,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITION
    SecurityEventContextDataDefinition                                     = 236,
    /// SECURITY-EVENT-CONTEXT-DATA-ELEMENT
    SecurityEventContextDataElement                                        = 182,
    /// SECURITY-EVENT-CONTEXT-MAPPING
    SecurityEventContextMapping                                            = 2773,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 794,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 369,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 2776,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 2095,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 633,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 1588,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 963,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 2364,
    /// SECURITY-EVENT-ONE-EVERY-N-FILTER
    SecurityEventOneEveryNFilter                                           = 1664,
    /// SECURITY-EVENT-REPORT-INSTANCE-DEFINITION
    SecurityEventReportInstanceDefinition                                  = 2649,
    /// SECURITY-EVENT-REPORT-INSTANCE-VALUE
    SecurityEventReportInstanceValue                                       = 1902,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 1549,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 970,
    /// SECURITY-EVENT-STATE-FILTER
    SecurityEventStateFilter                                               = 754,
    /// SECURITY-EVENT-THRESHOLD-FILTER
    SecurityEventThresholdFilter                                           = 2604,
    /// SELECTED
    Selected                                                               = 864,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 1251,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 1767,
    /// SENT-TAGGED
    SentTagged                                                             = 769,
    /// SENT-UNTAGGED
    SentUntagged                                                           = 2183,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 1686,
    /// SERIALIZER
    Serializer                                                             = 1349,
    /// SERVER-AUTHENTICATE
    ServerAuthenticate                                                     = 326,
    /// SERVER-CALL-POINT
    ServerCallPoint                                                        = 40,
    /// SERVER-DECRYPT
    ServerDecrypt                                                          = 287,
    /// SERVER-ENCRYPT
    ServerEncrypt                                                          = 1927,
    /// SERVER-MAC-GENERATE
    ServerMacGenerate                                                      = 1415,
    /// SERVER-MAC-VERIFY
    ServerMacVerify                                                        = 1261,
    /// SERVER-VERIFY
    ServerVerify                                                           = 2103,
    /// SERVICE-DISCOVERY
    ServiceDiscovery                                                       = 282,
    /// SERVICE-EVENT-DEPLOYMENT
    ServiceEventDeployment                                                 = 1216,
    /// SERVICE-FIELD-DEPLOYMENT
    ServiceFieldDeployment                                                 = 969,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 1806,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 861,
    /// SERVICE-INSTANCE-TO-MACHINE-MAPPING
    ServiceInstanceToMachineMapping                                        = 2652,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 1784,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 151,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 2506,
    /// SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToSwClusterDesignPortPrototypeMapping                   = 1690,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 918,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 2338,
    /// SERVICE-INTERFACE-DEPLOYMENT
    ServiceInterfaceDeployment                                             = 1002,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING
    ServiceInterfaceElementMapping                                         = 1888,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 638,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 2644,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 1735,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 2403,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 1498,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 2423,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 285,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 2512,
    /// SERVICE-METHOD-DEPLOYMENT
    ServiceMethodDeployment                                                = 260,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 2488,
    /// SERVICE-ONLY
    ServiceOnly                                                            = 1789,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 394,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 423,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 2801,
    /// SESSION-HANDLING-ACTIVE
    SessionHandlingActive                                                  = 1297,
    /// SESSION-HANDLING-INACTIVE
    SessionHandlingInactive                                                = 449,
    /// SETTER
    Setter                                                                 = 112,
    /// SG
    Sg                                                                     = 1144,
    /// SH
    Sh                                                                     = 2017,
    /// SHARED
    Shared                                                                 = 1877,
    /// SHARED-VLAN-LEARNING
    SharedVlanLearning                                                     = 1933,
    /// SHORT-HEADER
    ShortHeader                                                            = 2347,
    /// SHOW-ALIAS-NAME
    ShowAliasName                                                          = 575,
    /// SHOW-CATEGORY
    ShowCategory                                                           = 1410,
    /// SHOW-CONTENT
    ShowContent                                                            = 2047,
    /// SHOW-LONG-NAME
    ShowLongName                                                           = 2521,
    /// SHOW-NUMBER
    ShowNumber                                                             = 1518,
    /// SHOW-PAGE
    ShowPage                                                               = 509,
    /// SHOW-SEE
    ShowSee                                                                = 1906,
    /// SHOW-SHORT-NAME
    ShowShortName                                                          = 502,
    /// SHOW-TYPE
    ShowType                                                               = 987,
    /// SI
    Si                                                                     = 2132,
    /// SIDES
    Sides                                                                  = 2560,
    /// SIGN
    Sign                                                                   = 888,
    /// SIGN-WITH-ORIGIN-AUTHENTICATION
    SignWithOriginAuthentication                                           = 84,
    /// SIGNAL-BASED
    SignalBased                                                            = 1212,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 1078,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 434,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 943,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 816,
    /// SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFireAndForgetMethodToISignalTriggeringMapping               = 2386,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 2342,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 1724,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 912,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 1253,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 644,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 2788,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 1068,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 403,
    /// SIGNATURE
    Signature                                                              = 957,
    /// SILENT
    Silent                                                                 = 2015,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 2260,
    /// SINGLE
    Single                                                                 = 2477,
    /// SINGLE-CORE-REENTRANT
    SingleCoreReentrant                                                    = 796,
    /// SINGLE-LANGUAGE-REFERRABLE
    SingleLanguageReferrable                                               = 2791,
    /// SINGLE-OCCURRENCE
    SingleOccurrence                                                       = 2804,
    /// SK
    Sk                                                                     = 2357,
    /// SL
    Sl                                                                     = 2449,
    /// SLAVE
    Slave                                                                  = 1353,
    /// SLAVE-ACTIVE
    SlaveActive                                                            = 2762,
    /// SLAVE-PASSIVE
    SlavePassive                                                           = 588,
    /// SLOPPY
    Sloppy                                                                 = 604,
    /// SLOW-FLASHING-MODE
    SlowFlashingMode                                                       = 1342,
    /// SLP
    Slp                                                                    = 2415,
    /// SM
    Sm                                                                     = 2577,
    /// SM-INTERACTS-WITH-NM-MAPPING
    SmInteractsWithNmMapping                                               = 1883,
    /// SMPTE-338
    Smpte338                                                               = 325,
    /// SN
    Sn                                                                     = 2245,
    /// SO
    So                                                                     = 2775,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 1674,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 687,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 1553,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 1110,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 482,
    /// SOFTWARE-ACTIVATION-DEPENDENCY
    SoftwareActivationDependency                                           = 353,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 2137,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 190,
    /// SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS
    SoftwareClusterDiagnosticDeploymentProps                               = 430,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 302,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 1446,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 2502,
    /// SOMEIP
    Someip                                                                 = 1479,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 1158,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 1376,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 2037,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 1276,
    /// SOMEIP-FIELD
    SomeipField                                                            = 1272,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 1993,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 1639,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 404,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 1976,
    /// SOMEIP-REMOTE-MULTICAST-CONFIG
    SomeipRemoteMulticastConfig                                            = 2289,
    /// SOMEIP-REMOTE-UNICAST-CONFIG
    SomeipRemoteUnicastConfig                                              = 911,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 818,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 455,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 520,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 2696,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 998,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 764,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 640,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 2661,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 510,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 1495,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 1369,
    /// SOVD-GATEWAY-INSTANTIATION
    SovdGatewayInstantiation                                               = 875,
    /// SOVD-MODULE-INSTANTIATION
    SovdModuleInstantiation                                                = 2448,
    /// SOVD-SERVER-INSTANTIATION
    SovdServerInstantiation                                                = 526,
    /// SPEC-ELEMENT-REFERENCE
    SpecElementReference                                                   = 1185,
    /// SPEC-ELEMENT-SCOPE
    SpecElementScope                                                       = 1603,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 1962,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 1335,
    /// SQ
    Sq                                                                     = 1873,
    /// SR
    Sr                                                                     = 1066,
    /// SRGB
    Srgb                                                                   = 1725,
    /// SS
    Ss                                                                     = 1506,
    /// SSDP
    Ssdp                                                                   = 2073,
    /// ST
    St                                                                     = 2533,
    /// STACK-USAGE
    StackUsage                                                             = 20,
    /// STANDARD
    Standard                                                               = 1282,
    /// STANDARD-PORT
    StandardPort                                                           = 2792,
    /// START
    Start                                                                  = 360,
    /// START-FROM-BEGINNING
    StartFromBeginning                                                     = 2637,
    /// START-ON-BOOT
    StartOnBoot                                                            = 2159,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 2527,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 2600,
    /// STATE-CLIENT-INTERFACE
    StateClientInterface                                                   = 2770,
    /// STATE-DEPENDENT-FIREWALL
    StateDependentFirewall                                                 = 1474,
    /// STATE-MANAGEMEN-PHM-ERROR-INTERFACE
    StateManagemenPhmErrorInterface                                        = 1971,
    /// STATE-MANAGEMENT-ACTION-ITEM
    StateManagementActionItem                                              = 2157,
    /// STATE-MANAGEMENT-ACTION-LIST
    StateManagementActionList                                              = 2767,
    /// STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE
    StateManagementDiagTriggerInterface                                    = 2007,
    /// STATE-MANAGEMENT-EM-ERROR-INTERFACE
    StateManagementEmErrorInterface                                        = 266,
    /// STATE-MANAGEMENT-ERROR-INTERFACE
    StateManagementErrorInterface                                          = 1778,
    /// STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE
    StateManagementFunctionGroupSwitchNotificationInterface                = 2016,
    /// STATE-MANAGEMENT-MODULE-INSTANTIATION
    StateManagementModuleInstantiation                                     = 1804,
    /// STATE-MANAGEMENT-NM-ACTION-ITEM
    StateManagementNmActionItem                                            = 278,
    /// STATE-MANAGEMENT-NOTIFICATION-INTERFACE
    StateManagementNotificationInterface                                   = 1292,
    /// STATE-MANAGEMENT-PHM-ERROR-INTERFACE
    StateManagementPhmErrorInterface                                       = 2659,
    /// STATE-MANAGEMENT-PORT-INTERFACE
    StateManagementPortInterface                                           = 744,
    /// STATE-MANAGEMENT-REQUEST-ERROR
    StateManagementRequestError                                            = 1467,
    /// STATE-MANAGEMENT-REQUEST-INTERFACE
    StateManagementRequestInterface                                        = 2530,
    /// STATE-MANAGEMENT-REQUEST-TRIGGER
    StateManagementRequestTrigger                                          = 1322,
    /// STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM
    StateManagementSetFunctionGroupStateActionItem                         = 2623,
    /// STATE-MANAGEMENT-SLEEP-ACTION-ITEM
    StateManagementSleepActionItem                                         = 1712,
    /// STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM
    StateManagementStateMachineActionItem                                  = 2335,
    /// STATE-MANAGEMENT-STATE-NOTIFICATION
    StateManagementStateNotification                                       = 472,
    /// STATE-MANAGEMENT-STATE-REQUEST
    StateManagementStateRequest                                            = 1762,
    /// STATE-MANAGEMENT-SYNC-ACTION-ITEM
    StateManagementSyncActionItem                                          = 2223,
    /// STATE-MANAGEMENT-TRIGGER-INTERFACE
    StateManagementTriggerInterface                                        = 1037,
    /// STATIC-OR-DYNAMIC-PART-TRIGGER
    StaticOrDynamicPartTrigger                                             = 352,
    /// STATIC-PART-TRIGGER
    StaticPartTrigger                                                      = 971,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 2572,
    /// STATUS-BIT-AGING-AND-DISPLACEMENT
    StatusBitAgingAndDisplacement                                          = 1381,
    /// STATUS-BIT-NORMAL
    StatusBitNormal                                                        = 2720,
    /// STD
    Std                                                                    = 2130,
    /// STD-AXIS
    StdAxis                                                                = 1055,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 1000,
    /// STD_AXIS
    Stdaxis                                                                = 225,
    /// STEADY
    Steady                                                                 = 1117,
    /// STIMULUS-SYNCHRONIZATION
    StimulusSynchronization                                                = 2675,
    /// STOP
    Stop                                                                   = 2563,
    /// STOP-TRIGGER
    StopTrigger                                                            = 2421,
    /// STORE-EVENT
    StoreEvent                                                             = 2345,
    /// STORE-PERSISTENTLY
    StorePersistently                                                      = 2455,
    /// STRICT-MODE
    StrictMode                                                             = 2501,
    /// STRICT-MONOTONOUS
    StrictMonotonous                                                       = 1174,
    /// STRICT-PRIORITY
    StrictPriority                                                         = 2795,
    /// STRICTLY-DECREASING
    StrictlyDecreasing                                                     = 539,
    /// STRICTLY-INCREASING
    StrictlyIncreasing                                                     = 2109,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 2458,
    /// SU
    Su                                                                     = 1617,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 2090,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 592,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 2172,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 1673,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 1853,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 1768,
    /// SUPPLIER
    Supplier                                                               = 1124,
    /// SUPPORTS-BUFFER-LOCKING
    SupportsBufferLocking                                                  = 441,
    /// SV
    Sv                                                                     = 1819,
    /// SVG
    Svg                                                                    = 1490,
    /// SW
    Sw                                                                     = 2025,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 170,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 1229,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 143,
    /// SW-CALIBRATION-METHOD
    SwCalibrationMethod                                                    = 98,
    /// SW-CALPRM-PROTOTYPE
    SwCalprmPrototype                                                      = 383,
    /// SW-CLASS-ATTR-IMPL
    SwClassAttrImpl                                                        = 758,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 2067,
    /// SW-CLASS-PROTOTYPE
    SwClassPrototype                                                       = 477,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 2287,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 860,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 257,
    /// SW-COMPONENT-TYPE
    SwComponentType                                                        = 1226,
    /// SW-CONNECTOR
    SwConnector                                                            = 580,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 636,
    /// SW-INSTANCE
    SwInstance                                                             = 172,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 2683,
    /// SW-MC-FRAME
    SwMcFrame                                                              = 61,
    /// SW-MC-INTERFACE
    SwMcInterface                                                          = 160,
    /// SW-MC-INTERFACE-SOURCE
    SwMcInterfaceSource                                                    = 1227,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 1097,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 2243,
    /// SW-SERVICE-PROTOTYPE
    SwServicePrototype                                                     = 2056,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 1277,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 2677,
    /// SW-VARIABLE-PROTOTYPE
    SwVariablePrototype                                                    = 85,
    /// SWC
    Swc                                                                    = 822,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 692,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 1517,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 1743,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 1477,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 445,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 212,
    /// SWC-TIMING
    SwcTiming                                                              = 2694,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 1305,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 972,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 1980,
    /// SWITCH
    Switch                                                                 = 367,
    /// SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY
    SwitchAsynchronousTrafficShaperGroupEntry                              = 140,
    /// SWITCH-FLOW-METERING-ENTRY
    SwitchFlowMeteringEntry                                                = 2054,
    /// SWITCH-STREAM-FILTER-ACTION-DEST-PORT-MODIFICATION
    SwitchStreamFilterActionDestPortModification                           = 2484,
    /// SWITCH-STREAM-FILTER-ENTRY
    SwitchStreamFilterEntry                                                = 1275,
    /// SWITCH-STREAM-FILTER-RULE
    SwitchStreamFilterRule                                                 = 354,
    /// SWITCH-STREAM-GATE-ENTRY
    SwitchStreamGateEntry                                                  = 90,
    /// SWITCH-STREAM-IDENTIFICATION
    SwitchStreamIdentification                                             = 782,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 1036,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 2620,
    /// SYMMETRIC
    Symmetric                                                              = 2557,
    /// SYMMETRIC-KEY
    SymmetricKey                                                           = 953,
    /// SYNC-BASE-TIME-MANAGER
    SyncBaseTimeManager                                                    = 2663,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 78,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 2236,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 917,
    /// SYNCHRONIZED
    Synchronized                                                           = 2092,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 1557,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 217,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 452,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 2645,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 2104,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 2622,
    /// SYNCHRONOUS
    Synchronous                                                            = 2511,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 1333,
    /// SYSTEM
    System                                                                 = 1434,
    /// SYSTEM-DESIGN-TIME
    SystemDesignTime                                                       = 309,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 226,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 1298,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 200,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 1203,
    /// SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalGroupToCommunicationResourceMapping                        = 2296,
    /// SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalToCommunicationResourceMapping                             = 877,
    /// SYSTEM-SUPPLIER-BOOT
    SystemSupplierBoot                                                     = 2071,
    /// SYSTEM-SUPPLIER-BOOT-RESP-APP
    SystemSupplierBootRespApp                                              = 726,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 1991,
    /// TA
    Ta                                                                     = 721,
    /// TARGET-CONTAINER
    TargetContainer                                                        = 2021,
    /// TASK
    Task                                                                   = 2764,
    /// TC
    Tc                                                                     = 1250,
    /// TCP
    Tcp                                                                    = 876,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 1073,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 284,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 1944,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 92,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 1815,
    /// TD-EVENT-BSW
    TdEventBsw                                                             = 274,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 100,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 2592,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 684,
    /// TD-EVENT-COM
    TdEventCom                                                             = 2397,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 1462,
    /// TD-EVENT-CYCLE-START
    TdEventCycleStart                                                      = 1947,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 873,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 2133,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 110,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 856,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 2093,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 1820,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 653,
    /// TD-EVENT-SERVICE-INSTANCE
    TdEventServiceInstance                                                 = 1290,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 2395,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 727,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 1069,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 1183,
    /// TD-EVENT-SLLET
    TdEventSllet                                                           = 2800,
    /// TD-EVENT-SLLET-PORT
    TdEventSlletPort                                                       = 793,
    /// TD-EVENT-SWC
    TdEventSwc                                                             = 613,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 1377,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 2204,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 32,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 2374,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 399,
    /// TD-EVENT-VFB
    TdEventVfb                                                             = 1572,
    /// TD-EVENT-VFB-PORT
    TdEventVfbPort                                                         = 1323,
    /// TD-EVENT-VFB-PORT-GROUP
    TdEventVfbPortGroup                                                    = 379,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 2389,
    /// TDLET-ZONE-CLOCK
    TdletZoneClock                                                         = 1867,
    /// TE
    Te                                                                     = 1460,
    /// TERMINATE
    Terminate                                                              = 36,
    /// TEST-FAILED
    TestFailed                                                             = 1453,
    /// TEST-FAILED-BIT
    TestFailedBit                                                          = 1035,
    /// TEST-FAILED-THIS-OPERATION-CYCLE
    TestFailedThisOperationCycle                                           = 2323,
    /// TEST-PASSED
    TestPassed                                                             = 1780,
    /// TESTED
    Tested                                                                 = 1714,
    /// TESTED-AND-FAILED
    TestedAndFailed                                                        = 2476,
    /// TG
    Tg                                                                     = 1676,
    /// TH
    Th                                                                     = 2548,
    /// TI
    Ti                                                                     = 2665,
    /// TIFF
    Tiff                                                                   = 938,
    /// TIME
    Time                                                                   = 2048,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 550,
    /// TIME-BASE-RESOURCE
    TimeBaseResource                                                       = 576,
    /// TIME-MASTER
    TimeMaster                                                             = 2602,
    /// TIME-SLAVE
    TimeSlave                                                              = 2556,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 2672,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 2580,
    /// TIME-SYNC-SERVER-CONFIGURATION
    TimeSyncServerConfiguration                                            = 2456,
    /// TIME-SYNCHRONIZATION-INTERFACE
    TimeSynchronizationInterface                                           = 1973,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 1550,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 1968,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 2028,
    /// TIMEOUT
    Timeout                                                                = 587,
    /// TIMING-CLOCK
    TimingClock                                                            = 1964,
    /// TIMING-CLOCK-SYNC-ACCURACY
    TimingClockSyncAccuracy                                                = 410,
    /// TIMING-CONDITION
    TimingCondition                                                        = 2362,
    /// TIMING-CONSTRAINT
    TimingConstraint                                                       = 55,
    /// TIMING-DESCRIPTION
    TimingDescription                                                      = 108,
    /// TIMING-DESCRIPTION-EVENT
    TimingDescriptionEvent                                                 = 270,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 2555,
    /// TIMING-EVENT
    TimingEvent                                                            = 1283,
    /// TIMING-EXTENSION
    TimingExtension                                                        = 1358,
    /// TIMING-EXTENSION-RESOURCE
    TimingExtensionResource                                                = 2610,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 339,
    /// TIP
    Tip                                                                    = 1753,
    /// TK
    Tk                                                                     = 60,
    /// TL
    Tl                                                                     = 174,
    /// TLS-12
    Tls12                                                                  = 1197,
    /// TLS-13
    Tls13                                                                  = 857,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 2479,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 2165,
    /// TLS-CRYPTO-CIPHER-SUITE-PROPS
    TlsCryptoCipherSuiteProps                                              = 1616,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 1167,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 739,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 2069,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 431,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 115,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 2468,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 2160,
    /// TN
    Tn                                                                     = 392,
    /// TO
    To                                                                     = 496,
    /// TOP
    Top                                                                    = 1977,
    /// TOPBOT
    Topbot                                                                 = 1107,
    /// TOPIC
    Topic                                                                  = 254,
    /// TOPIC-1
    Topic1                                                                 = 2607,
    /// TOPIC-PREFIX
    TopicPrefix                                                            = 507,
    /// TP-ADDRESS
    TpAddress                                                              = 573,
    /// TP-CONFIG
    TpConfig                                                               = 1494,
    /// TP-CONNECTION-IDENT
    TpConnectionIdent                                                      = 774,
    /// TR
    Tr                                                                     = 1583,
    /// TRACE
    Trace                                                                  = 1682,
    /// TRACE-REFERRABLE
    TraceReferrable                                                        = 350,
    /// TRACE-SWITCH-ARTI
    TraceSwitchArti                                                        = 2410,
    /// TRACE-SWITCH-ARTI-AND-LOG
    TraceSwitchArtiAndLog                                                  = 356,
    /// TRACE-SWITCH-LOG
    TraceSwitchLog                                                         = 2065,
    /// TRACE-SWITCH-NONE
    TraceSwitchNone                                                        = 1234,
    /// TRACEABLE
    Traceable                                                              = 2549,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 2045,
    /// TRACEABLE-TEXT
    TraceableText                                                          = 23,
    /// TRACED-FAILURE
    TracedFailure                                                          = 855,
    /// TRANSFER
    Transfer                                                               = 2404,
    /// TRANSFORMATION-I-SIGNAL-PROPS-IDENT
    TransformationISignalPropsIdent                                        = 2127,
    /// TRANSFORMATION-PROPS
    TransformationProps                                                    = 1645,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 1787,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 315,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 1868,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 2061,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 830,
    /// TRANSFORMER-ERROR-HANDLING
    TransformerErrorHandling                                               = 2121,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 146,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 1581,
    /// TRANSFORMING-I-SIGNAL
    TransformingISignal                                                    = 1514,
    /// TRANSIENT
    Transient                                                              = 695,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 1060,
    /// TRANSIENT-LOCAL
    TransientLocal                                                         = 1289,
    /// TRANSLATION-START
    TranslationStart                                                       = 345,
    /// TRANSPORT
    Transport                                                              = 2489,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 2732,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 1324,
    /// TRAP
    Trap                                                                   = 1824,
    /// TRIGGER
    Trigger                                                                = 1929,
    /// TRIGGER-ACTIVATED
    TriggerActivated                                                       = 1155,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 480,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 2596,
    /// TRIGGER-RELEASED
    TriggerReleased                                                        = 1643,
    /// TRIGGER-UNICAST
    TriggerUnicast                                                         = 1612,
    /// TRIGGERED
    Triggered                                                              = 2187,
    /// TRIGGERED-ON-CHANGE
    TriggeredOnChange                                                      = 2233,
    /// TRIGGERED-ON-CHANGE-WITHOUT-REPETITION
    TriggeredOnChangeWithoutRepetition                                     = 488,
    /// TRIGGERED-ON-EVALUATION
    TriggeredOnEvaluation                                                  = 1858,
    /// TRIGGERED-WITHOUT-REPETITION
    TriggeredWithoutRepetition                                             = 1405,
    /// TRUE
    True                                                                   = 511,
    /// TS
    Ts                                                                     = 1731,
    /// TT
    Tt                                                                     = 1801,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 2418,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 1424,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 722,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 1109,
    /// TUNNEL
    Tunnel                                                                 = 2744,
    /// TW
    Tw                                                                     = 2438,
    /// TX-REF-TRIGGER
    TxRefTrigger                                                           = 2249,
    /// TX-REF-TRIGGER-GAP
    TxRefTriggerGap                                                        = 942,
    /// TX-TRIGGER-MERGED
    TxTriggerMerged                                                        = 1374,
    /// TX-TRIGGER-SINGLE
    TxTriggerSingle                                                        = 492,
    /// UCM
    Ucm                                                                    = 2562,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 365,
    /// UCM-MASTER
    UcmMaster                                                              = 2167,
    /// UCM-MASTER-MODULE-INSTANTIATION
    UcmMasterModuleInstantiation                                           = 1360,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 537,
    /// UCM-RETRY-STRATEGY
    UcmRetryStrategy                                                       = 1578,
    /// UCM-STEP
    UcmStep                                                                = 1878,
    /// UCM-SUBORDINATE-MODULE-INSTANTIATION
    UcmSubordinateModuleInstantiation                                      = 2735,
    /// UCM-TO-TIME-BASE-RESOURCE-MAPPING
    UcmToTimeBaseResourceMapping                                           = 747,
    /// UDP
    Udp                                                                    = 1062,
    /// UDP-CHECKSUM-DISABLED
    UdpChecksumDisabled                                                    = 1755,
    /// UDP-CHECKSUM-ENABLED
    UdpChecksumEnabled                                                     = 1267,
    /// UDP-NM
    UdpNm                                                                  = 1230,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 1534,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 1702,
    /// UDS
    Uds                                                                    = 450,
    /// UK
    Uk                                                                     = 2664,
    /// UNDECIDED
    Undecided                                                              = 1992,
    /// UNDEFINED
    Undefined                                                              = 418,
    /// UNIT
    Unit                                                                   = 2227,
    /// UNIT-GROUP
    UnitGroup                                                              = 148,
    /// UNNUMBER
    Unnumber                                                               = 214,
    /// UNSPECIFIED
    Unspecified                                                            = 2251,
    /// UP-LINK-PORT
    UpLinkPort                                                             = 1031,
    /// UPDATE
    Update                                                                 = 2447,
    /// UPDATE-CONFIGURATION
    UpdateConfiguration                                                    = 652,
    /// UPLOADABLE-DEPLOYMENT-ELEMENT
    UploadableDeploymentElement                                            = 252,
    /// UPLOADABLE-DESIGN-ELEMENT
    UploadableDesignElement                                                = 2643,
    /// UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT
    UploadableExclusivePackageElement                                      = 603,
    /// UPLOADABLE-PACKAGE-ELEMENT
    UploadablePackageElement                                               = 2808,
    /// UR
    Ur                                                                     = 2314,
    /// USE-ARGUMENT-TYPE
    UseArgumentType                                                        = 887,
    /// USE-ARRAY-BASE-TYPE
    UseArrayBaseType                                                       = 2731,
    /// USE-FIRST-CONTEXT-DATA
    UseFirstContextData                                                    = 2008,
    /// USE-LAST-CONTEXT-DATA
    UseLastContextData                                                     = 2337,
    /// USE-VOID
    UseVoid                                                                = 1718,
    /// USER
    User                                                                   = 2272,
    /// USER-DEFINED
    UserDefined                                                            = 2756,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 2299,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 1370,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 565,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 24,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 2375,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 1736,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 2709,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 1948,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 2229,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 2518,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 760,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 1306,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 798,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 2346,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 885,
    /// USES-LOGGING
    UsesLogging                                                            = 2009,
    /// UZ
    Uz                                                                     = 1133,
    /// V-2-X-ACTIVE-SUPPORTED
    V2XActiveSupported                                                     = 2450,
    /// V-2-X-DATA-MANAGER-NEEDS
    V2XDataManagerNeeds                                                    = 2585,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 1164,
    /// V-2-X-FACILITIES
    V2XFacilities                                                          = 784,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 965,
    /// V-2-X-MANAGEMENT
    V2XManagement                                                          = 465,
    /// V-2-X-NOT-SUPPORTED
    V2XNotSupported                                                        = 1719,
    /// VALID
    Valid                                                                  = 2656,
    /// VAR
    Var                                                                    = 1459,
    /// VAR-FAST
    VarFast                                                                = 2494,
    /// VAR-NO-INIT
    VarNoInit                                                              = 1783,
    /// VAR-POWER-ON-INIT
    VarPowerOnInit                                                         = 1579,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 2723,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 705,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 921,
    /// VARIABLE-DATA-PROTOTYPE-RECEIVED
    VariableDataPrototypeReceived                                          = 447,
    /// VARIABLE-DATA-PROTOTYPE-SENT
    VariableDataPrototypeSent                                              = 386,
    /// VARIABLE-SIZE
    VariableSize                                                           = 139,
    /// VARIANT-LINK-TIME
    VariantLinkTime                                                        = 775,
    /// VARIANT-POST-BUILD
    VariantPostBuild                                                       = 2250,
    /// VARIANT-POST-BUILD-LOADABLE
    VariantPostBuildLoadable                                               = 569,
    /// VARIANT-POST-BUILD-SELECTABLE
    VariantPostBuildSelectable                                             = 1017,
    /// VARIANT-PRE-COMPILE
    VariantPreCompile                                                      = 389,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 672,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 208,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 1771,
    /// VENDOR
    Vendor                                                                 = 1967,
    /// VENDOR-SPECIFIC
    VendorSpecific                                                         = 2026,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 306,
    /// VERBOSE
    Verbose                                                                = 2352,
    /// VERIFICATION
    Verification                                                           = 2595,
    /// VERIFY
    Verify                                                                 = 1647,
    /// VERSION-1
    Version1                                                               = 1411,
    /// VERTEX-OF-TARGET-CONTAINER
    VertexOfTargetContainer                                                = 772,
    /// VFB-TIMING
    VfbTiming                                                              = 62,
    /// VI
    Vi                                                                     = 1194,
    /// VIDEO-FRAME
    VideoFrame                                                             = 1800,
    /// VIDEO-LINE
    VideoLine                                                              = 75,
    /// VIEW-MAP
    ViewMap                                                                = 2309,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 2782,
    /// VLAN-CONFIG
    VlanConfig                                                             = 223,
    /// VO
    Vo                                                                     = 2177,
    /// VOLATILE
    Volatile                                                               = 2379,
    /// WAIT-FOR-VEHICLE-SAFE-STATE
    WaitForVehicleSafeState                                                = 1542,
    /// WAIT-POINT
    WaitPoint                                                              = 614,
    /// WAIT-TIME-DATE
    WaitTimeDate                                                           = 862,
    /// WARMUP
    Warmup                                                                 = 1788,
    /// WARN
    Warn                                                                   = 952,
    /// WARNING
    Warning                                                                = 1609,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 206,
    /// WATCH-DOG-MANAGER
    WatchDogManager                                                        = 2333,
    /// WATCH-TRIGGER
    WatchTrigger                                                           = 851,
    /// WATCH-TRIGGER-GAP
    WatchTriggerGap                                                        = 677,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 1855,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 891,
    /// WEIGHTED-ROUND-ROBIN
    WeightedRoundRobin                                                     = 184,
    /// WILL-CALL
    WillCall                                                               = 2142,
    /// WILL-RECEIVE
    WillReceive                                                            = 1566,
    /// WILL-SEND
    WillSend                                                               = 1526,
    /// WO
    Wo                                                                     = 2583,
    /// WONT-CALL
    WontCall                                                               = 1485,
    /// WONT-RECEIVE
    WontReceive                                                            = 900,
    /// WONT-SEND
    WontSend                                                               = 82,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 453,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 33,
    /// WRITE
    Write                                                                  = 1990,
    /// WRITE-ONLY
    WriteOnly                                                              = 673,
    /// WRONG-TRIGGER
    WrongTrigger                                                           = 2630,
    /// X-509
    X509                                                                   = 2114,
    /// X-MII
    XMii                                                                   = 1840,
    /// X-MMI
    XMmi                                                                   = 429,
    /// XCP
    Xcp                                                                    = 232,
    /// XCP-PDU
    XcpPdu                                                                 = 2082,
    /// XDOC
    Xdoc                                                                   = 1813,
    /// XFILE
    Xfile                                                                  = 696,
    /// XG-MII
    XgMii                                                                  = 2116,
    /// XH
    Xh                                                                     = 2462,
    /// XOR
    Xor                                                                    = 2698,
    /// XREF-TARGET
    XrefTarget                                                             = 2378,
    /// XXG-MII
    XxgMii                                                                 = 330,
    /// XYZ
    Xyz                                                                    = 836,
    /// YCBCR
    Ycbcr                                                                  = 401,
    /// YCGCO
    Ycgco                                                                  = 1196,
    /// YCM
    Ycm                                                                    = 2693,
    /// YO
    Yo                                                                     = 1636,
    /// ZH
    Zh                                                                     = 919,
    /// ZU
    Zu                                                                     = 558,
    /// default
    default                                                                = 1014,
    /// preserve
    preserve                                                               = 1798,
}

impl EnumItem {
    #[rustfmt::skip]
    const STRING_TABLE: [&'static str; 2810] = ["COUPLING-PORT-STRUCTURAL-ELEMENT", "NO-SHOW-CONTENT", "ERROR-DETECTION", "COMPU-METHOD", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "AVB--IEEE-802--1-AS", "PRE--R-4--2", "APMC-URI-INSTANCE-REFERENCE-VALUE", "FM-FEATURE-SELECTION-SET", "MG", "PERSISTENCY-FILE-STORAGE", "INTERRUPT-CAT-1", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "BAYER-GRBG", "MODE-ACCESS-POINT-IDENT", "1-001", "PLAIN", "COM-FIND-SERVICE-GRANT-DESIGN", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "RTE-EVENT", "STACK-USAGE", "ICV-NOT-VERIFIED", "SCHEDULE-VARIANT-3", "TRACEABLE-TEXT", "USER-DEFINED-ETHERNET-FRAME", "GIF", "CRC-IGNORED", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "EID-USE-CONFIG-VALUE", "FUNCTION-INHIBITION-MANAGER", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE", "REST-BOOLEAN-PROPERTY-DEF", "TD-EVENT-TRIGGER", "WORST-CASE-STACK-USAGE", "PERIODIC-EVENT-TRIGGERING", "MAPPING-SCOPE-CORE", "TERMINATE", "QU", "PRECONFIGURED-CONFIGURATION", "EXECUTE", "SERVER-CALL-POINT", "MAPPING-SCOPE-ECU", "NON-OS-MODULE-INSTANTIATION", "APMC-FOREIGN-REFERENCE-VALUE", "KL", "CRYPTO-SIGNATURE-SCHEME", "ARGUMENT-DATA-PROTOTYPE", "NO-OBD-SUPPORT", "CRYPTO-CERTIFICATE", "DDS-CP-TOPIC", "NETWORK", "RPT-ENABLER-ROM", "BINARY-MANIFEST-META-DATA-FIELD", "COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING", "NETWORK-REPRESENTATION-FROM-COM-SPEC", "TIMING-CONSTRAINT", "ACCESS-PERMISSION-SERVICE-CLASS", "IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "GENERAL-PARAMETER", "NTP--RFC-958", "TK", "SW-MC-FRAME", "VFB-TIMING", "IEEE-1722-TP-CRF-CONNECTION", "BSW-CALLED-ENTITY", "SEARCH-FOR-ID", "PHM-RULE", "AP", "REBOOT", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "DO-IP-SERVICE-NEEDS", "EXPLICIT", "HW-ATTRIBUTE-LITERAL-DEF", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "DIAGNOSTIC-INFO-TYPE", "VIDEO-LINE", "ECUC-VALIDATION-CONDITION", "REDUNDANT", "SYNC-TIME-BASE-MGR-USER-NEEDS", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "EPS", "NOT-DEFINED", "WONT-SEND", "REST-STRING-PROPERTY-DEF", "SIGN-WITH-ORIGIN-AUTHENTICATION", "SW-VARIABLE-PROTOTYPE", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "DDS-SERVICE", "APMC-DEFINITION-COLLECTION", "ADAPTIVE-EVENT-RECEIVED", "SWITCH-STREAM-GATE-ENTRY", "FM-FEATURE-MODEL", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "BO", "MS", "FUNCTION-INHIBITION-NEEDS", "SDG-CAPTION", "BINARY-MANIFEST-REQUIRE-RESOURCE", "SW-CALIBRATION-METHOD", "NO-MONOTONY", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "IEEE-1722-TP-ACF-CAN", "AUTO-IPDHCPV-4", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE", "LINK-LOCAL", "IDS-DESIGN", "COLOR-BLIND", "TIMING-DESCRIPTION", "DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING", "TD-EVENT-FRAME-ETHERNET", "BSW-MODULE-ENTITY", "SETTER", "AGREED", "EDGE-NODE", "TLS-JOB-REQUIREMENT", "FUNCTION-GROUP-STATE-TO-NM-HANDLE", "ISO-14229--1", "ENCRYPTION", "ADAPTIVE-FIELD-GETTER-COMPLETED", "BROAD-R-REACH", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "DIAGNOSTIC-EVENT-MANAGER", "RECT", "BUILD-TYPE-RELEASE", "REF-ALL", "APPLICATION-ERROR", "FO", "NL", "PC-AFFECTS-LT", "APMC-ABSTRACT-DEFINITION", "BEST-EFFORT", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-ECU-RESET", "DEDICATED", "OBD-RATIO-SERVICE-NEEDS", "DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING", "HI", "ABSTRACT-ETHERNET-FRAME", "VARIABLE-SIZE", "SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY", "DO-IP-INSTANTIATION", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "SW-BASE-TYPE", "4-2-0", "PERSISTENCY-DEPLOYMENT", "TRANSFORMER-HARD-ERROR-EVENT", "PACKAGEABLE-ELEMENT", "UNIT-GROUP", "RESET-MCU", "ADAPTIVE-SERVICE-OFFER-COMPLETED", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "CALIBRATION-PARAMETER-VALUE-SET", "60", "DIAGNOSTIC-SOVD-METHOD-PRIMITIVE", "FRAME-TRIGGERING", "ECUC-REFERENCE-DEF", "DIAGNOSTIC-ROUTINE", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "ETHERNET-MAC-RAW-DATA-STREAM-MAPPING", "SW-MC-INTERFACE", "NETWORK-HANDLE-PORT-MAPPING", "SECONDARY-ECU", "NEW-IS-LESS-OR-EQUAL", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "IDSM-INSTANCE", "APMC-VALUE-COLLECTION", "IEEE802-1AS", "COM-METHOD-GRANT-DESIGN", "KY", "SW-ADDR-METHOD", "DIAGNOSTIC-TROUBLE-CODE-UDS", "SW-INSTANCE", "KM", "TL", "ADAPTIVE-METHOD-CALL-RECEIVED", "NEVER", "REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE", "APMC-CHOICE-CONTAINER-DEF", "ALIVE-SUPERVISION", "ON-DTC-STATUS-CHANGE", "ETHERNET-RAW-DATA-STREAM-GRANT", "SECURITY-EVENT-CONTEXT-DATA-ELEMENT", "FIX-AXIS", "WEIGHTED-ROUND-ROBIN", "APPLICATION-ACTION-ITEM", "ECUC-MODULE-DEF", "EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE", "EXACT-OR-ANY-MINOR-VERSION", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "SOFTWARE-CLUSTER-DESIGN", "ARBITRARY-EVENT-TRIGGERING", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "DIAGNOSTIC-SOVD-BULK-DATA", "KEYWORD-SET", "READ", "CRYPTO-PRIMITIVE", "MT", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "OBD-DCY", "SYSTEM-SIGNAL", "10BASE-T1S", "CONST", "HU", "PORT-PROTOTYPE", "200", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "ABSTRACT-FUNCTIONAL-CLUSTER-DESIGN", "VEHICLE-PACKAGE", "CALCULATED", "PHM-SUPERVISED-ENTITY-INTERFACE", "EXPRESS", "SWC-SERVICE-DEPENDENCY", "APPLICATION-RECORD-DATA-TYPE", "UNNUMBER", "NM-CONFIG", "REPORT-AFTER-INIT", "SYNCHRONIZED-SLAVE-TIME-BASE", "COLLECTABLE-ELEMENT", "FLEXRAY-FRAME-TRIGGERING", "PROTECT-LAMP", "PC-AFFECTS-PB", "CAN-COMMUNICATION-CONTROLLER", "VLAN-CONFIG", "READ-ONLY", "STD_AXIS", "SYSTEM-MAPPING", "MAPPING-SCOPE-PARTITION", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "RPT-PROFILE", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "DIAGNOSTIC-ENABLE-CONDITION", "XCP", "BSW-IMPLEMENTATION", "EVENT-STORAGE-DISABLED", "APMC-FUNCTIONAL-CLUSTER-VALUE", "SECURITY-EVENT-CONTEXT-DATA-DEFINITION", "JA", "LIMIT-TO-PAGE", "15", "100BASE-TX", "RAW-DATA-STREAM-SERVER-INTERFACE", "FILTERED", "OPERATION-CALLED", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "NO-DEFAULT", "DEFAULT-MODE", "ABSTRACT-ACCESS-POINT", "PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE", "ERROR-TRACER-NEEDS", "GET", "UPLOADABLE-DEPLOYMENT-ELEMENT", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "TOPIC", "SATURATE", "CYCLE-REPETITION-4", "SW-COMPONENT-PROTOTYPE", "ATP-BLUEPRINT", "KN", "SERVICE-METHOD-DEPLOYMENT", "FLEXRAY-TP-PDU-POOL", "CENTER", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "BSW-MODULE-ENTITY-ACTIVATED", "AF", "STATE-MANAGEMENT-EM-ERROR-INTERFACE", "BLOCK-STATE", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "BINARY-MANIFEST-ITEM-DEFINITION", "TIMING-DESCRIPTION-EVENT", "ABSTRACT-CAN-PHYSICAL-CHANNEL", "IS", "PERSISTENCY-KEY-VALUE-STORAGE", "TD-EVENT-BSW", "DDS-CP-DOMAIN", "RAW-DATA-STREAM-MAPPING", "DLT-USER-NEEDS", "STATE-MANAGEMENT-NM-ACTION-ITEM", "APPLICATION-ARRAY-ELEMENT", "PURE-LOCAL-TIME-BASE", "FULL-COM", "SERVICE-DISCOVERY", "CHARGE-MANAGER-NEEDS", "TCP-OPTION-FILTER-SET", "SERVICE-INTERFACE-PEDIGREE", "GLOBAL-SUPERVISION", "SERVER-DECRYPT", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "DIAGNOSTIC-EVENT-PORT-MAPPING", "DIAGNOSTIC-MEMORY-DESTINATION", "RPT-EXECUTION-CONTEXT", "DIAGNOSTIC-FIM-EVENT-GROUP", "OPERATION-CALL-RECEIVED", "PUBLISHED-INFORMATION", "J-1939-CLUSTER", "AR-ELEMENT", "RESOURCE-CONSUMPTION", "GL", "20", "I-SIGNAL-PORT", "PORT-INTERFACE-MAPPING-SET", "SOFTWARE-CLUSTER-REQUIREMENT", "IEEE-1722-TP-AAF-CONNECTION", "CRYPTO-INTERFACE", "ICMP", "VENDOR-SPECIFIC-SERVICE-NEEDS", "PHM-ARBITRATION", "DDS-DOMAIN-RANGE", "SYSTEM-DESIGN-TIME", "CYCLE-REPETITION-10", "DATA-RECEIVED-EVENT", "NO-SHOW-ALIAS-NAME", "LOGICAL-OR", "RESPOND-AFTER-RESET", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "RPT-ENABLER-RAM", "MODE-SWITCH-POINT", "LAST-IS-BEST", "JAVA", "ABSTRACT-CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "DO-IP-ROUTING-ACTIVATION", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "GLOBAL-TIME-SLAVE", "DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING", "SMPTE-338", "SERVER-AUTHENTICATE", "DIAGNOSTIC-EVENT-INFO-NEEDS", "IMPLEMENTATION-DATA-TYPE", "IS-LESS-THAN-OR-EQUAL", "XXG-MII", "INDICATE", "MODE-DECLARATION", "DSA", "AGGREGATION-TAILORING", "J-1939-NM--CCA", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "ACL-OBJECT-SET", "IEEE802-11P", "TIMING-MODE-INSTANCE", "150", "IT", "PROCESS-TO-MACHINE-MAPPING-SET", "GENERIC-ETHERNET-FRAME", "DIAGNOSTIC-SESSION-CONTROL", "TRANSLATION-START", "HARDWARE-TEST-MANAGER", "DO-IP-POWER-MODE-STATUS-NEEDS", "POLY", "PERIODIC-RATE-SLOW", "TRACE-REFERRABLE", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "STATIC-OR-DYNAMIC-PART-TRIGGER", "SOFTWARE-ACTIVATION-DEPENDENCY", "SWITCH-STREAM-FILTER-RULE", "ECUC-INSTANCE-REFERENCE-DEF", "TRACE-SWITCH-ARTI-AND-LOG", "RAW-DATA-STREAM-GRANT-DESIGN", "E-2-E-PROFILE-CONFIGURATION-SET", "PASSIVE", "START", "JPG", "BSW-MODULE-DESCRIPTION", "CYCLE-REPETITION-5", "DATA-RECEIVE-ERROR-EVENT", "UCM-DESCRIPTION", "AUTO", "SWITCH", "CRC-SUPPORTED", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "BUILD-ACTION-ENVIRONMENT", "DATA-PROTOTYPE-TRANSFORMATION-PROPS-IDENT", "PRE-COMPILE-TIME", "ICV-NOT-SUPPORTED", "I-PDU", "DOES-NOT-REPORT-EXECUTION-STATE", "RESPONSE-SYNCHRONIZATION", "BSW-SERVICE-DEPENDENCY-IDENT", "RES_AXIS", "TD-EVENT-VFB-PORT-GROUP", "BLOCK-SOURCE", "GRAYSCALE", "DIAGNOSTIC-VALUE-NEEDS", "SW-CALPRM-PROTOTYPE", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "SECURE-COMMUNICATION-PROPS-SET", "VARIABLE-DATA-PROTOTYPE-SENT", "REPORT-BEFORE-INIT", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "VARIANT-PRE-COMPILE", "APMC-REFERENCE-VALUE", "J-1939-DCM-I-PDU", "TN", "ITU-BT-2020", "SERVICE-PROXY-SW-COMPONENT-TYPE", "DEFAULT-TRACE-STATE-DISABLED", "INTERNAL-TRIGGER-OCCURRED-EVENT", "REST-ABSTRACT-PROPERTY-DEF", "NE", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING", "YCBCR", "FA", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "SOMEIP-METHOD-DEPLOYMENT", "JI", "ATP-DEFINITION", "FLAT-MAP", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING", "TIMING-CLOCK-SYNC-ACCURACY", "EVENT-ACCEPTANCE-ENABLED", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "READ-WRITE", "DELEGATION-SW-CONNECTOR", "ARTIFACT-LOCATOR", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "AFTERMARKET", "UNDEFINED", "IEEE-1722-ACF-BUS-PART-RAW-DATA-STREAM-CONSUMER-MAPPING", "LTS-13", "NET", "CRYPTO-KEY-SLOT-USAGE-DESIGN", "SERVICE-SW-COMPONENT-TYPE", "CYCLE-REPETITION-32", "ADAPTIVE-SERVICE-OFFER-STARTED", "REF-NONE", "NV-BLOCK-NEEDS", "NONE", "X-MMI", "SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS", "TLS-JOB-MAPPING", "48", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "CLIENT-MAC-GENERATE", "ADDR-METHOD-SHORT-NAME", "RPT-ENABLER-RAM-AND-ROM", "FLEXRAY-AR-TP-NODE", "FOR-ALL", "RECORD-VALUE-FIELD", "SUPPORTS-BUFFER-LOCKING", "ADAPTIVE-FIELD-SETTER-COMPLETED", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "SWC-MODE-SWITCH-EVENT", "24", "VARIABLE-DATA-PROTOTYPE-RECEIVED", "INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE", "SESSION-HANDLING-INACTIVE", "UDS", "ACTIVATE", "SYNCHRONIZED-TIME-BASE-CONSUMER", "WORST-CASE-HEAP-USAGE", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "CM-MODULE-INSTANTIATION", "MEASURED-HEAP-USAGE", "SECURED-PDU-HEADER-08-BIT", "DATA-FORMAT-ELEMENT-SCOPE", "LIN-PHYSICAL-CHANNEL", "CAN-BE-REMOVED", "CONFIDENTIALITY-OFFSET--50", "NORMALFIXED", "V-2-X-MANAGEMENT", "DIAGNOSTIC-COM-CONTROL", "NM-INSTANTIATION", "DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "REST-ENDPOINT-DELETE", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "STATE-MANAGEMENT-STATE-NOTIFICATION", "APMC-CONTAINER-VALUE", "LEGACY", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING", "FIREWALL-STATE-SWITCH-INTERFACE", "SW-CLASS-PROTOTYPE", "REST-ARRAY-PROPERTY-DEF", "DEVELOPMENT-ERROR", "TRIGGER-INTERFACE", "INFO", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "LAND", "I-PDU-PORT", "I-SIGNAL-TO-I-PDU-MAPPING", "ENHANCED-TRAFFIC-SHAPER", "TRIGGERED-ON-CHANGE-WITHOUT-REPETITION", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "APMC-NUMERICAL-PARAM-VALUE", "DATA-FORMAT-ELEMENT-REFERENCE", "TX-TRIGGER-SINGLE", "OPERATION-CALL-RESPONSE-RECEIVED", "DEBUG", "DEFERRED", "TO", "PHM-SUPERVISION", "POST-BUILD", "INT-32-BIT", "CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "SHOW-SHORT-NAME", "FLEXRAY-TP-CONNECTION-CONTROL", "ADAPTIVE-SERVICE-FIND-COMPLETED", "LOWER-8-BIT", "ERROR-CORRECTION", "TOPIC-PREFIX", "FLEXRAY-COMMUNICATION-CONNECTOR", "SHOW-PAGE", "SOMEIP-TP-CHANNEL", "TRUE", "CRYPTO-SERVICE-CERTIFICATE", "FLEXRAY-NM-CLUSTER", "DIAGNOSTIC-AUTH-ROLE", "KO", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "ETHERNET-CLUSTER", "100BASE-T1", "ECUC-CHOICE-REFERENCE-DEF", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "PERSISTENCY-FILE", "CLIENT-VERIFY", "1", "ACCEPT-CONFIGURED", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "SOVD-SERVER-INSTANTIATION", "ET", "ECUC-CONTAINER-DEF", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "24-25", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "GENERIC-DIAGNOSTIC-TRANSPORT-INSTANTIATION", "DIAGNOSTIC-CONDITION", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED", "DDS-FIELD-DEPLOYMENT", "UCM-MODULE-INSTANTIATION", "ACCES-PERRMISSION-SERVICE-CLASS", "STRICTLY-DECREASING", "ABSTRACT-CRYPTO-KEY-SLOT-INTERFACE", "PLATFORM-ACTION-ITEM", "NEW-IS-GREATER", "IMPLEMENTATION-PROPS", "ADAPTIVE-METHOD-CALLED", "EXECUTION-ORDER-CONSTRAINT", "NO-RETURN-VALUE-PROVIDED", "J-1939-NM--AAC", "ETH-TP-CONFIG", "BSW-SCHEDULE-EVENT", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "GLOBAL-TIME-FR-MASTER", "MY", "AFTER-SALES", "ADAPTIVE-METHOD-RESPONSE-SENT", "DONT-INVALIDATE", "CUSTOM", "ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING", "ZU", "COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER", "SDG-AGGREGATION-WITH-VARIATION", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-PORT-MAPPING", "CONSUMED-SERVICE-INSTANCE", "RESTART-APPLICATION", "PERSISTENCY-FILE-STORAGE-INTERFACE", "USER-DEFINED-COMMUNICATION-CONTROLLER", "IS-NOT-RELEVANT", "REPETITIVE-EOC", "ABSTRACT-DO-IP-PORT-MAPPING", "VARIANT-POST-BUILD-LOADABLE", "ROOT-SW-COMPOSITION-PROTOTYPE", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "FM-FEATURE-RELATION", "TP-ADDRESS", "FM-FEATURE-MAP-ASSERTION", "SHOW-ALIAS-NAME", "TIME-BASE-RESOURCE", "ROTATE-90-CW-FIT-TO-TEXT", "MAC-SEC-PARTICIPANT-SET", "DCM-I-PDU", "SW-CONNECTOR", "KEEP-ALL", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "CRYPTO-SERVICE-PRIMITIVE", "CAN-BE-TERMINATED", "DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE", "TIMEOUT", "SLAVE-PASSIVE", "COUPLING-PORT-FIFO", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "OBD", "SUPERVISED-ENTITY-NEEDS", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "CLASSIC", "CYCLE-REPETITION-50", "NO-FLOAT", "PHYSICAL-CAN-FD", "SDG-ABSTRACT-FOREIGN-REFERENCE", "NM-ECU", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT", "SLOPPY", "N-PDU", "COUPLING-PORT-SCHEDULER", "ASYNCHRONOUS", "INT-16-BIT", "DIAGNOSTIC-SOVD-LOG", "FRAME-QUEUED-FOR-TRANSMISSION", "MASTER", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "TD-EVENT-SWC", "WAIT-POINT", "DIAGNOSTIC-ENV-MODE-ELEMENT", "APPLICATION-DEFERRED-DATA-TYPE", "OPERATION-INVOKED-EVENT", "CRYPTO-KEY-SLOT-USER-DESIGN", "GLOBAL-TIME-ETH-SLAVE", "ALL-PARTIAL-NETWORKS-ACTIVE", "IEEE-1722-TP-AV-CONNECTION", "P-PORT-PROTOTYPE", "SA", "INTERNAL-TRIGGERING-POINT", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "ES", "AS", "PROCESS-IS-NOT-SELF-TERMINATING", "COMMUNICATION-INTER-ECU", "NO-SUPPORT", "DIAGNOSTIC-INDICATOR-INTERFACE", "LOWER-12-BIT", "SECURITY-EVENT-CONTEXT-PROPS", "CAPTURE-SYNCHRONOUS-TO-REPORTING", "EU", "SW-GENERIC-AXIS-PARAM-TYPE", "ATTRIBUTE-TAILORING", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "ECU-MAPPING", "SOMEIP-SERVICE-INTERFACE", "IEEE-1722-TP-ACF-CONNECTION", "GENERAL-PURPOSE-TIMER-SERVICE-NEEDS", "NOTIFICATION", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "APMC-FUNCTIONAL-CLUSTER-DEF", "RESPONSE", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "CRYPTO-TRUST-MASTER-INTERFACE", "ENUMERATION-MAPPING-TABLE", "DEPENDANT", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "UPDATE-CONFIGURATION", "TD-EVENT-OPERATION", "BSW-DEBUG-INFO", "IEEE-1722-TP-RVF-CONNECTION", "DIAGNOSTIC-CONTROL-DTC-SETTING", "EVALUATED-VARIANT-SET", "BSW-SCHEDULER-NAME-PREFIX", "LIFE-CYCLE-INFO-SET", "LOGICAL-AND", "CONSUMER", "PORT-BLUEPRINT", "NFOLD", "EXECUTION-TIME-CONSTRAINT", "ERROR-TRACER", "FIBEX-ELEMENT", "MI", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "CONFIG-DATA", "CONSTANT-SPECIFICATION", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "VARIATION-POINT-PROXY", "WRITE-ONLY", "DEFAULT-IF-REVISION-UPDATE", "QUEUED", "CRYPTO-PROVIDER-INTERFACE", "WATCH-TRIGGER-GAP", "BY-SOURCE-TIMESTAMP", "MASKED-NEW-DIFFERS-X", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "ACK-WITHOUT-RT", "COMPILE", "ATP-TYPE", "TD-EVENT-BSW-MODULE", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "BINARY-MANIFEST-PROVIDE-RESOURCE", "SO-CON-I-PDU-IDENTIFIER", "PARAMETER-INTERFACE", "MIXED-29-BIT", "ECUC-MODULE-CONFIGURATION-VALUES", "ABSTRACT-EVENT", "SWC-BSW-MAPPING", "ACL-PERMISSION", "REFERENCE-TAILORING", "TRANSIENT", "XFILE", "LIN-COMMUNICATION-CONNECTOR", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "25-24", "BOLDITALIC", "192-KHZ", "APMC-FLOAT-PARAM-DEF", "COUPLING-PORT-ABSTRACT-SHAPER", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT", "CRYPTO-SERVICE-KEY", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "ECUC-FOREIGN-REFERENCE-DEF", "NO", "PRIMITIVE", "DDS-TOPIC-ACCESS-RULE", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "PHM-LOGICAL-EXPRESSION", "PARAMETER-DATA-PROTOTYPE", "4-2-2", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "AUTONOMOUS", "INLINE-CONDITIONAL", "NO-CHECKPOINT-SUPERVISION", "TA", "TTCAN-COMMUNICATION-CONTROLLER", "I-PDU-RECEIVED-BY-COM", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "PROVIDED-SERVICE-INSTANCE", "SYSTEM-SUPPLIER-BOOT-RESP-APP", "TD-EVENT-SERVICE-INSTANCE-EVENT", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "PERSISTENCY-FILE-ELEMENT", "INTERGRITY-WITHOUT-CONFIDENTIALITY", "ARBITRATION", "MEDIUM", "CYCLE-REPETITION-64", "LIN-NM-CLUSTER", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "IDSM-REPORTING-MODE-PROVIDER-MAPPING", "FM-FEATURE-MAP", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "TLS-DEPLOYMENT", "DIAGNOSTIC-IO-CONTROL-NEEDS", "BSW-MODE-SWITCH-EVENT", "CRYPTO-MODULE-INSTANTIATION", "DIAGNOSTIC-IO-CONTROL", "STATE-MANAGEMENT-PORT-INTERFACE", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "UCM-TO-TIME-BASE-RESOURCE-MAPPING", "DIAGNOSTIC-SOVD-CONFIGURATION", "DOES-NOT-USE-LOGGING", "OPAQUE", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "DEFAULT-IF-UNDEFINED", "APMC-INSTANCE-REFERENCE-DEF", "SECURITY-EVENT-STATE-FILTER", "CAPTURE-SYNCHRONOUSLY-TO-REPORTING", "FRAME-RECEIVED-BY-IF", "I-SIGNAL-I-PDU", "SW-CLASS-ATTR-IMPL", "REQUIRED-DDS-SERVICE-INSTANCE", "USER-DEFINED-PDU", "RX-TRIGGER", "IDSM-QUALIFIED-EVENT-RECEIVER-MAPPING", "CLIENT-SERVER-INTERFACE", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "BSW-MODULE-ENTRY", "CRYPTO-KEY-SLOT-USAGE-DESIGN-MAPPING", "COMMUNICATION-INTRA-PARTITION", "DHCPV-6", "SENT-TAGGED", "REPLACE", "DIAGNOSTIC-DO-IP-POWER-MODE-PORT-MAPPING", "VERTEX-OF-TARGET-CONTAINER", "JUSTIFY", "TP-CONNECTION-IDENT", "VARIANT-LINK-TIME", "APMC-CONTAINER-REFERENCE-VALUE", "GLOBAL-SUPERVISION-ENTITY", "DIAGNOSTIC-ABSTRACT-ALIAS-EVENT", "CYCLE-REPETITION-40", "APPLICATION-ENDPOINT", "96-KHZ", "SWITCH-STREAM-IDENTIFICATION", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "V-2-X-FACILITIES", "4-1-1", "MC-GROUP", "PREDEFINED-VARIANT", "LIN-SLAVE", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "NO-STORE-EVENT", "IEEE-1722-TP-ACF-BUS-PART", "PHM-HEALTH-CHANNEL-STATUS", "TD-EVENT-SLLET-PORT", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "IDENTIFIABLE", "SINGLE-CORE-REENTRANT", "PTP--IEEE-1588--2008", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "BG", "ECUC-VALUE-COLLECTION", "DIAGNOSTIC-SOVD-LOCK", "GA", "LN", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "NON-VOLATILE-RAM-MANAGER", "ICV-SUPPORTED", "48-KHZ", "PERSISTENCY-FILE-ARRAY", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "DDS-SECURE-COM-PROPS", "CAPTURE-ASYNCHRONOUSLY-TO-REPORTING", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "PRESHARED-KEY-IDENTITY", "KEEP-EXISTING", "COM-METHOD-GRANT", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "BSW-OPERATION-INVOKED-EVENT", "SOMEIP-REQUIRED-EVENT-GROUP", "ACCESS-PERMISSION-SERVICE-INSTANCE", "SD", "ECUC-ENUMERATION-LITERAL-DEF", "SWC", "PEER", "I-PDU-TRIGGERING", "INTERGRITY-AND-CONFIDENTIALITY", "LINK", "CO", "BLUEPRINT-MAPPING-SET", "BAYER-RGGB", "TRANSFORMATION-TECHNOLOGY", "CONSISTENCY-MECHANISM-REQUIRED", "ADAPTIVE-SERVICE-FIND-STARTED", "ISO-11992--4", "IDS-MGR-NEEDS", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "XYZ", "BA", "MEASURED-STACK-USAGE", "1000BASE-T1", "ROLL-BACK", "J-1939-TP-NODE", "MASEKD-NEW-EQUALS-MASKED-OLD", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "FUNCTIONAL-CAN-FD", "LOG-AND-TRACE-INSTANTIATION", "DO-NOT-INCLUDE", "ADAPTIVE-PLATFORM-SERVICE-INSTANCE", "DLT-MESSAGE-COLLECTION-SET", "DIAGNOSTIC-SERVICE-TABLE", "PORT-INTERFACE-MAPPING", "WATCH-TRIGGER", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "DLT-CONTEXT", "DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE", "TRACED-FAILURE", "TD-EVENT-I-PDU", "TLS-13", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "COM-MANAGER", "SW-COMPONENT-MAPPING-CONSTRAINTS", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "WAIT-TIME-DATE", "INTERPOLATION-ROUTINE-MAPPING-SET", "SELECTED", "FALSE", "BINARY-MANIFEST-ITEM", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "DIAG-RESPONSE", "NON-EMMISSION-RELATED-DTC", "PERSISTENCY-DATA-ELEMENT", "IA", "TD-EVENT-FR-CLUSTER-CYCLE-START", "ECUC-FUNCTION-NAME-DEF", "SOVD-GATEWAY-INSTANTIATION", "TCP", "SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING", "APMC-PARAMETER-DEF", "KEEP", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "IW", "COM-MANAGEMENT-MAPPING", "ECUC-ENUMERATION-PARAM-DEF", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "USER-DEFINED-TRANSFORMATION-PROPS", "OBD-PID-SERVICE-NEEDS", "USE-ARGUMENT-TYPE", "SIGN", "CMDT", "IEEE-1722-RAW-DATA-STREAM-MAPPING", "WATCHDOG-PHM-ACTION-ITEM", "DIAGNOSTIC-COM-CONTROL-CLASS", "J-1939-PROTECTED-I-PDU", "ADAPTIVE-FIELD-GETTER-CALLED", "CLEAR", "FIELD-MAPPING", "MODE-DECLARATION-SWITCH-INITIATED", "HUB", "BY-RECEPTION-TIMESTAMP", "WONT-RECEIVE", "LO", "DDS-METHOD-DEPLOYMENT", "CP-SOFTWARE-CLUSTER", "FRAME-PORT", "EXECUTION-TIME", "ONE-EVERY-N", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "MAC-LAYER-RAW-DATA-STREAM-INTERFACE", "CRYPTO-KEY-SLOT-TO-CLIENT-PORT-PROTOTYPE-MAPPING", "2500BASE-T1", "SOMEIP-REMOTE-UNICAST-CONFIG", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "44-1-KHZ", "OS-MODULE-INSTANTIATION", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "SYNCHRONIZATION-TIMING-CONSTRAINT", "SERVICE-INTERFACE", "ZH", "OC", "VARIABLE-DATA-PROTOTYPE", "ETHERNET-FRAME-TRIGGERING", "I-SIGNAL", "NO-CONSISTENCY-MECHANISM", "NETWORK-ENDPOINT", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "IGNITION", "MODELED", "HOOK", "DIAGNOSTIC-MEMORY-IDENTIFIER", "FR", "FM-FEATURE", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "EXTERNAL-REPLACEMENT", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "CAPTURE-ASYNCHRONOUS-TO-REPORTING", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "TIFF", "ROUTER-ADVERTISEMENT", "E-2-E-PROFILE-CONFIGURATION", "DIAGNOSTIC-STOP-ROUTINE", "TX-REF-TRIGGER-GAP", "SIGNAL-BASED-FIELD-DEPLOYMENT", "CP-SOFTWARE-CLUSTER-RESOURCE", "PGWIDE", "FDBAMCMDT", "COUPLING-PORT-ENHANCED-TRAFFIC-SHAPER", "PROCESS-DESIGN", "NO-SHOW-CATEGORY", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "SEARCH-FOR-SPECIFIC-INSTANCE", "WARN", "SYMMETRIC-KEY", "DIAGNOSTIC-DATA-IDENTIFIER", "NON-VOLATILE", "HY", "SIGNATURE", "INTERRUPT-CAT-2", "DIAGNOSTIC-ACCESS-PERMISSION", "ECUC-ABSTRACT-STRING-PARAM-DEF", "I-PV-6-EXT-HEADER-FILTER-LIST", "RU", "SECURITY-EVENT-FILTER-CHAIN", "SCHEDULED", "V-2-X-M-USER-NEEDS", "DYNAMIC-PART-TRIGGER", "IS-GREATER-THAN-OR-EQUAL", "IDSM-TRAFFIC-LIMITATION", "SERVICE-FIELD-DEPLOYMENT", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "STATIC-PART-TRIGGER", "SWC-TO-ECU-MAPPING", "OPTIONS", "IEEE-1722-RAW-DATA-STREAM-CONSUMER-INTERFACE", "PROCESSOR", "APMC-TEXTUAL-PARAM-VALUE", "CONCRETE", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "ETH-TCP-IP-ICMP-PROPS", "EVAP", "CANCEL", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "FATAL", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "REF-NON-STANDARD", "SAFETY", "SHOW-TYPE", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "CA", "ACCEPT-ALL", "INSTALL", "DATA-PROTOTYPE-GROUP", "INVALID", "INOUT", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "PL", "NO-STATUS-BYTE-CHANGE", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "IEEE802-1AS-AUTOSAR", "SERVICE-INTERFACE-DEPLOYMENT", "DO-IP", "ETHERNET-COMMUNICATION-CONNECTOR", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "DLT-ARGUMENT", "BE", "RIGHT", "SAE-J-1939--73", "BUS-MIRROR-CHANNEL-MAPPING", "ACTIVATION-AND-TRIGGER-UNICAST", "INTRA-LET-EOC", "4-2-2-4", "default", "CAN-PHYSICAL-CHANNEL", "SDG-PRIMITIVE-ATTRIBUTE", "VARIANT-POST-BUILD-SELECTABLE", "ECUC-DESTINATION-URI-DEF", "CHANNEL-B", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "ECU-MANAGER", "ETP", "IPSEC", "HEALTH-CHANNEL-EXTERNAL-MODE", "LINK-LOCAL--DOIP", "PERIODIC-RATE-FAST", "CRC-NOT-SUPPORTED", "AP-APPLICATION-ERROR-SET", "ABSTRACT-EXECUTION-CONTEXT", "DDS-CP-PARTITION", "UP-LINK-PORT", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "MIN", "AGE-CONSTRAINT", "TEST-FAILED-BIT", "SYMBOL-PROPS", "STATE-MANAGEMENT-TRIGGER-INTERFACE", "FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING", "SECRET-SEED", "IS-EQUAL", "72", "ATP-CLASSIFIER", "LOGICAL-EXPRESSION", "PERSISTENCY-FILE-PROXY-INTERFACE", "FRAME-ETHERNET-RECEIVED-BY-IF", "PROCESSING-STYLE-SYNCHRONOUS", "FRAME-ETHERNET-SENT-ON-BUS", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "ALWAYS", "APMC-URI-FOREIGN-REFERENCE-DEF", "COM-FIELD-GRANT", "FDBAM", "SECURED-I-PDU", "HEALTH-CHANNEL-EXTERNAL-STATUS", "STD-AXIS", "PARTITION", "EXTENDED", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "NOT-ACCESSIBLE", "TRANSIENT-FAULT", "ANY-STANDARDIZED", "UDP", "BIDIRECTIONAL", "INSTRUCTION", "LOCAL", "SR", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "SIGNAL-SERVICE-TRANSLATION-PROPS", "TD-EVENT-SERVICE-INSTANCE-FIELD", "REQUIRES-CALLBACK-EXECUTION", "BSW-MODULE-CLIENT-SERVER-ENTRY", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "TCP-OPTION-FILTER-LIST", "NO-SHOW-LONG-NAME", "RED-STOP-LAMP", "NEWLINE", "HW-PIN", "SIGNAL-BASED-EVENT-DEPLOYMENT", "DATA-TRANSFORMATION", "IS-OK", "IK", "PERSISTENCY-FILE-PROXY", "MASKED-NEW-DIFFERS-MASKED-OLD", "ACTION", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT", "MEMORY-SECTION", "DDS-CP-CONSUMED-SERVICE-INSTANCE", "MODE-SWITCHED-ACK-EVENT", "J-1939-SHARED-ADDRESS-CLUSTER", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "FUNCTIONAL", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING", "CONSTRAINT-TAILORING", "AS-IS", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "SW-RECORD-LAYOUT", "ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING", "ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE", "CAN-NM-CLUSTER", "DDS-CP-CONFIG", "ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS", "DIAGNOSTIC-IO-CONTROL-CLASS", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "RECOVERY-VIA-APPLICATION-ACTION", "DATA-SEND-COMPLETED-EVENT", "TOPBOT", "NM-HANDLE-INACTIVE-TO-FUNCTION-GROUP-STATE", "TTCAN-PHYSICAL-CHANNEL", "SOCKET-CONNECTION-BUNDLE", "ECUC-FLOAT-PARAM-DEF", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "DIAGNOSTIC-AUTHENTICATION-CLASS", "IDENT-CAPTION", "LA", "SDG-DEF", "STEADY", "FIRST-CONTAINED-TRIGGER", "FI", "PROCESS-PHM-ACTION-ITEM", "EVENT-COMBINATION-ON-RETRIEVAL", "ALL-INDICES-SAME-ARRAY-SIZE", "ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED", "SUPPLIER", "IMPLEMENTATION", "FUNCTION-GROUP-SET", "EOC-EXECUTABLE-ENTITY-REF-ABSTRACT", "APMC-ABSTRACT-FOREIGN-REFERENCE-DEF", "CONNECT", "BSW-BACKGROUND-EVENT", "DOCUMENTATION", "DIAGNOSTIC-SERVICE-CLASS", "UZ", "BLINK-MODE", "ARTIFACT-CHECKSUM", "DOES-NOT-SUPPORT-BUFFER-LOCKING", "ENCRYPT-AND-SIGN", "COUPLING-PORT-CREDIT-BASED-SHAPER", "INCREASING", "INTERFACE-MAPPING-SET", "LIN-SPORADIC-FRAME", "BSW-VARIABLE-ACCESS", "ECUC-ADD-INFO-PARAM-DEF", "SG", "CPP-IMPLEMENTATION-DATA-TYPE", "BINARY-MANIFEST-RESOURCE-DEFINITION", "RECOMMENDED-CONFIGURATION", "DATA-TYPE-MAPPING-SET", "BT-REC-709", "NO-PROTECTION", "FIXED-SIZE", "DIAGNOSTIC-CAPABILITY-ELEMENT", "ICV-IGNORED", "DROP-UNTAGGED", "TRIGGER-ACTIVATED", "BAM", "25", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "MULTIPLEXED-I-PDU", "ON-ENTRY", "AH", "REST-ENDPOINT-PUT", "V-2-X-FAC-USER-NEEDS", "ROTATE-90-CW-LIMIT-TO-TEXT", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "TLS-CRYPTO-SERVICE-MAPPING", "NO-KEEP", "NO-SUPERVISION", "API-BASED", "LINK-TIME", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "STRICT-MONOTONOUS", "BONJOUR", "REQUEST", "DISABLE", "PA", "ESP", "CURVE-AXIS", "ABSTRACT-CAN-COMMUNICATION-CONNECTOR", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "TD-EVENT-SERVICE-INSTANCE-METHOD", "MAX", "SPEC-ELEMENT-REFERENCE", "REST-ABSTRACT-ENDPOINT", "IDS-MAPPING", "DIAGNOSTIC-CONDITION-GROUP", "CAT-1", "HW-TYPE", "NO-SHOW-TYPE", "RW", "RAW-DATA-STREAM-CLIENT-INTERFACE", "VI", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "YCGCO", "TLS-12", "MEMORY-USAGE", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "DEFAULT-TRIGGER", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "SYSTEM-SIGNAL-GROUP", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "BLINK-OR-CONTINUOUS-ON-MODE", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "APPLICATION-PARTITION", "BT-REC-601", "NO-NEWLINE", "CALLOUT", "PENDING", "SIGNAL-BASED", "DZ", "LIN-SLAVE-CONFIG-IDENT", "LIN-MASTER", "SERVICE-EVENT-DEPLOYMENT", "OPERATING-SYSTEM", "FIRST-TO-SECOND", "PHYSICAL-ADDRESS", "INTER-LET-ONLY", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE", "FUNCTIONAL-ADDRESS", "LOGICAL-SUPERVISION", "IDSM-PROPERTIES", "SW-COMPONENT-TYPE", "SW-MC-INTERFACE-SOURCE", "RESET-ECU", "SW-AXIS-TYPE", "UDP-NM", "EVENT-COMBINATION-ON-STORAGE", "FAST-FLASHING-MODE", "CONFIGURED", "TRACE-SWITCH-NONE", "RESPOND-BEFORE-RESET", "CAN-TP-CONFIG", "IS-RELEVANT", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "EXECUTABLE", "NEW-IS-DIFFERENT", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "ETHERNET-FRAME", "CONFIRMED", "DIAGNOSTIC-LOG-AND-TRACE", "ENABLED", "ABSTRACT-SERVICE-INSTANCE", "DDS-RPC-SERVICE-DEPLOYMENT", "DEF-ITEM", "CAN-FD", "TC", "SENDER-RECEIVER-INTERFACE", "REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "DIAGNOSTIC-DATA-BY-IDENTIFIER", "ACTION-LIST", "12", "DIAGNOSTIC-FIM-ALIAS-EVENT", "ARRAY", "C", "DIAGNOSTIC-REQUEST-DOWNLOAD", "SERVER-MAC-VERIFY", "IEEE-1722-ACF-BUS-RAW-DATA-STREAM-CONSUMER-MAPPING", "ECUC-COMMON-ATTRIBUTES", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "PC-AFFECTS-LT-AND-PB", "ASYMMETRIC-TO-BYTE-ARRAY", "UDP-CHECKSUM-ENABLED", "GENERAL-PURPOSE-CONNECTION", "CRYPTO-SERVICE-NEEDS", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "FIX_AXIS", "SOMEIP-FIELD", "CY", "CONSOLE", "SWITCH-STREAM-FILTER-ENTRY", "SOMEIP-EVENT-GROUP", "SW-SYSTEMCONST", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "BUILD-ACTION-MANIFEST", "J-1939-NM--SVCA", "DEVELOPMENT-ERROR-TRACER", "STANDARD", "TIMING-EVENT", "PHM-CHECKPOINT", "DIAGNOSTIC-REQUEST-UPLOAD", "ITALIC", "DESCENDANT", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "TRANSIENT-LOCAL", "TD-EVENT-SERVICE-INSTANCE", "DOMAIN-PARTICIPANT-USER-DATA-QOS", "STATE-MANAGEMENT-NOTIFICATION-INTERFACE", "GLOBAL-TIME-ETH-MASTER", "ROTATE-180", "EQUAL", "ANALYZED-EXECUTION-TIME", "SESSION-HANDLING-ACTIVE", "SYSTEM-MEMORY-USAGE", "CAN-BRIEF", "ETH-IP-PROPS", "APMC-INSTANCE-REFERENCE-VALUE", "COMMAND-LINE-SHORT-FORM", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE", "SWC-TO-APPLICATION-PARTITION-MAPPING", "USER-DEFINED-PHYSICAL-CHANNEL", "EID-USE-API", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "NETWORK-CONFIGURATION", "REST-OBJECT-REF", "DIAGNOSTIC-MULTIPLE-RESOURCE-INTERFACE", "COM-AXIS", "DIAGNOSTIC-TROUBLE-CODE", "ETH-TCP-IP-PROPS", "LIN-CLUSTER", "HW-ATTRIBUTE-DEF", "DIAGNOSTIC-SERVICE-SW-MAPPING", "RAW-DATA-STREAM-INTERFACE", "IEEE-1722-RAW-DATA-STREAM-PRODUCER-INTERFACE", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING", "DETAILED", "STATE-MANAGEMENT-REQUEST-TRIGGER", "TD-EVENT-VFB-PORT", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "ML", "OBD-CONTROL-SERVICE-NEEDS", "RESET-VM", "REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST", "REST-INTEGER-PROPERTY-DEF", "APMC-CONFIGURATION-ELEMENT-DEF", "AUDIO-SAMPLE", "PUBLIC-KEY", "SYNCHRONOUS-SERVER-CALL-POINT", "MAC-SEC-KAY-PARTICIPANT", "SPORADIC-EVENT-TRIGGERING", "IEEE-1722-TP-ETHERNET-FRAME", "REDUNDANT-PER-ELEMENT", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "NOHREF", "ACL-ROLE", "ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE", "SLOW-FLASHING-MODE", "GLOBAL-TIME-DOMAIN", "SECURE-ON-BOARD-COMMUNICATION", "MODE-DECLARATION-SWITCH-COMPLETED", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "FDCMDT", "ECUC-INTEGER-PARAM-DEF", "SERIALIZER", "GLOBAL-TIME-FR-SLAVE", "DIAGNOSTIC-RESPONSE-ON-EVENT", "BR", "SLAVE", "PER-EXECUTABLE", "BSW-MODE-SWITCHED-ACK-EVENT", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "LONG-HEADER", "TIMING-EXTENSION", "ONLY-THIS-CYCLE-AND-READINESS", "UCM-MASTER-MODULE-INSTANTIATION", "MAC-MULTICAST-GROUP", "SECURE-COM-PROPS", "REQUIRED-AP-SERVICE-INSTANCE", "FPP", "DEFLATE", "RESTART", "INFINITE", "BASE-TYPE", "SOMEIP-TRANSFORMATION-PROPS", "USER-DEFINED-COMMUNICATION-CONNECTOR", "DEFAULT", "DROP", "DIAGNOSTIC-ROUTINE-CONTROL", "TX-TRIGGER-MERGED", "ROUGH-ESTIMATE-HEAP-USAGE", "SOMEIP-EVENT", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "PASS-THROUGH-SW-CONNECTOR", "COM-TRIGGER-GRANT", "SECURE-COM-PROPS-SET", "STATUS-BIT-AGING-AND-DISPLACEMENT", "NETWORK-MANAGEMENT-PORT-INTERFACE", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "COMMUNICATION-CLUSTER", "CVC", "FIT-TO-TEXT", "MACHINE-DESIGN", "POST", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "INSTANCE-ID", "ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-PORT-MAPPING", "BSW-M-ENTRY-CALLED", "AP-APPLICATION-ENDPOINT", "DIAGNOSTIC-AGING", "FM-FEATURE-RESTRICTION", "CRYPTO-KEY-SLOT-INTERFACE", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "PERSISTENCY-KEY-VALUE-DATABASE", "DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE", "LIN-FRAME", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "LINKER", "5000BASE-T1", "TRIGGERED-WITHOUT-REPETITION", "ADAPTIVE-FIELD-SETTER-CALLED", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "IGNORE", "SHOW-CATEGORY", "VERSION-1", "APPLICATION-SW-COMPONENT-TYPE", "ECUC-CONTAINER-VALUE", "BAYER-GBRG", "SERVER-MAC-GENERATE", "REACTION", "EVENT-ACCEPTANCE-DISABLED", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "EMISSION-RELATED-DTC", "COM-OFFER-SERVICE-GRANT-DESIGN", "SDG-FOREIGN-REFERENCE", "IDSM-ABSTRACT-PORT-INTERFACE", "MONOCHROME", "TTCAN-COMMUNICATION-CONNECTOR", "CONFIRMED-DTC-BIT", "RUNNABLE-ENTITY-ACTIVATED", "RUNNABLE-ENTITY-STARTED", "EXTEND", "ALTERNATING-8-BIT", "PDU-TO-FRAME-MAPPING", "IS-LESS-OR-EQUAL", "PROTECTED", "SDG-CLASS", "SYSTEM", "NO-SHOW-SEE", "KA", "CANCEL-CAMPAIGN", "SECTION-NAME-PREFIX", "CRYPTO-SERVICE-MANAGER", "PERSISTENCY-KEY-VALUE-PAIR", "DO-IP-LOGIC-ADDRESS", "DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA", "2", "CYCLE-REPETITION-8", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "SOFTWARE-PACKAGE", "RPT-COMPONENT", "BUILD-ACTION-ENTITY", "SEC-OC-JOB-MAPPING", "PLATFORM-HEALTH-MANAGEMENT-INTERFACE", "FUNCTION-GROUP-PORT-MAPPING", "DIAGNOSTIC-DATA-PORT-MAPPING", "TEST-FAILED", "APMC-CONTAINER-ELEMENT-DEF", "SCHEDULE-VARIANT-7", "BINARY-MANIFEST-ADDRESSABLE-OBJECT", "LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING", "EXECUTABLE-ENTITY", "VAR", "TE", "SECURITY-EVENT-AGGREGATION-FILTER", "TD-EVENT-COMPLEX", "GLOBAL-TIME-CAN-SLAVE", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "CLOSED", "REST-RESOURCE-DEF", "STATE-MANAGEMENT-REQUEST-ERROR", "APMC-ENUMERATION-PARAM-DEF", "POWER", "R-4--2", "FLAT-INSTANCE-DESCRIPTOR", "ABSTRACT-IAM-REMOTE-SUBJECT", "PORT", "STATE-DEPENDENT-FIREWALL", "RETURN-ON-EVENT-CLEARED", "PORT-GROUP", "SWC-MODE-MANAGER-ERROR-EVENT", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "SOMEIP", "DEPENDENCY-ON-ARTIFACT", "DIAGNOSTIC-PROTOCOL", "PNG", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "CRYPTO-JOB", "WONT-CALL", "DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING", "MONO", "DO-IP-INTERFACE", "ROTATE-90-CCW-FIT-TO-TEXT", "SVG", "BOTTOM", "PREEMPTABLE", "SDG-REFERENCE", "TP-CONFIG", "SOMEIP-TP-CONFIG", "APPLICATION-INTERFACE", "BSW-DISTINGUISHED-PARTITION", "SERVICE-INTERFACE-MAPPING-SET", "OBD-MONITOR-SERVICE-NEEDS", "APPLICATION-ARRAY-DATA-TYPE", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "ACTIVATION-MULTICAST", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE", "BASIC-SOFTWARE-MODE-MANAGER", "DROP-FRAME", "SS", "RULE", "ABSTRACT-IMPLEMENTATION-DATA-TYPE", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "LOW", "IMPOSITION-TIME-DEFINITION-GROUP", "NEW-IS-LESS", "GD", "TRANSFORMING-I-SIGNAL", "NO-BREAK", "MOST-SIGNIFICANT-BYTE-LAST", "SWC-IMPLEMENTATION", "SHOW-NUMBER", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "ACK-WITH-RT", "DIAGNOSTIC-MEMORY-BY-ADDRESS", "NOT-TESTED", "NOT-VALID", "LT-AFFECTS-PB", "COM-OFFER-SERVICE-GRANT", "WILL-SEND", "DA", "ABSTRACT-SECURITY-EVENT-FILTER", "PROCESS-IS-SELF-TERMINATING", "ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF", "PHYSICAL-DIMENSION-MAPPING-SET", "APMC-INTEGER-PARAM-DEF", "END-TO-END-PROTECTION", "UDP-NM-CLUSTER", "LIN-SCHEDULE-TABLE", "HA", "PARTIAL-NETWORK", "ECUC-MULTILINE-STRING-PARAM-DEF", "DIAGNOSTIC-STORAGE-CONDITION", "BAYER-BGGR", "PDUR-I-PDU-GROUP", "WAIT-FOR-VEHICLE-SAFE-STATE", "IS-STOPPED", "CRYPTO-NEED", "CAN-CLUSTER", "END-2-END-EVENT-PROTECTION-PROPS", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "SECURITY-EVENT-REPORT-INTERFACE", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "CHAPTER", "REST-ABSTRACT-NUMERICAL-PROPERTY-DEF", "SOCKET-ADDRESS", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "GENERIC-MODULE-INSTANTIATION", "LEAF-OF-TARGET-CONTAINER", "SYNCHRONIZED-MASTER-TIME-BASE", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "ROUTER", "MULTICORE-REENTRANT", "CPP", "BAMCMDT", "PAYLOAD-AS-POINTER-TO-ARRAY", "I-SIGNAL-TRIGGERING", "ECUC-DESTINATION-URI-DEF-SET", "WILL-RECEIVE", "PARAMETER-ACCESS", "ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF", "MEASUREMENT-POINT", "KS", "EL", "TD-EVENT-VFB", "DIAGNOSTIC-SECURE-CODING-MAPPING", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "IEEE-1722-TP-IIDC-CONNECTION", "CODE", "DATA-INTERFACE", "UCM-RETRY-STRATEGY", "VAR-POWER-ON-INIT", "FLEXRAY-FRAME", "TRANSFORMER-STATUS-FORWARDING", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "TR", "FLOAT-32-BIT", "ERROR", "APPLICATION-PRIMITIVE-DATA-TYPE", "CONTINUOUS-ON-MODE", "SECURITY-EVENT-DEFINITION", "ECUC-URI-REFERENCE-DEF", "MODE-INTERFACE-MAPPING", "IEEE-1722-TP-CONNECTION", "SEC-OC-SECURE-COM-PROPS", "FIT-TO-PAGE", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "IDSM-TIMESTAMP-PROVIDER-MAPPING", "FLEXRAY-NM-NODE", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE", "NEW-IS-EQUAL", "CYCLIC", "NM-HANDLE-ACTIVE-TO-FUNCTION-GROUP-STATE", "BSW-OS-TASK-EXECUTION-EVENT", "COM-EVENT-GRANT-DESIGN", "SPEC-ELEMENT-SCOPE", "COM-MGR-USER-NEEDS", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "DYNAMIC", "DEFICIT-ROUND-ROBIN", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING", "WARNING", "INDICATOR-STATUS-NEEDS", "CLIENT-ID-DEFINITION", "TRIGGER-UNICAST", "FM-FEATURE-MAP-ELEMENT", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "AUTO-IP--DOIP", "TLS-CRYPTO-CIPHER-SUITE-PROPS", "SU", "ABSTRACT-CLASS-TAILORING", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS", "PROCESS-TO-MACHINE-MAPPING", "BSW-INTERNAL-TRIGGERING-POINT", "JW", "DATA-CONSTR", "RESOURCE-GROUP", "REST-ENDPOINT-POST", "FRAME-ETHERNET-RECEIVED-ON-BUS", "CONSTANT-SPECIFICATION-MAPPING-SET", "BUS-MIRROR-CHANNEL-MAPPING-IP", "CLASS-CONTENT-CONDITIONAL", "ETHERNET-COMMUNICATION-CONTROLLER", "PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING", "I-PDU-SENT-TO-IF", "AP-APPLICATION-ERROR-DOMAIN", "OVERRIDE", "YO", "NM-NETWORK-HANDLE", "IEEE-1722-TP-CONFIG", "SOMEIP-METHOD", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "NODE", "RETURN-ON-EVENT-STOPPED", "TRIGGER-RELEASED", "PROVIDED-DDS-SERVICE-INSTANCE", "TRANSFORMATION-PROPS", "DLT-LOG-SINK", "VERIFY", "DDS-CP-PROVIDED-SERVICE-INSTANCE", "PNC-MAPPING-IDENT", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "ATP-STRUCTURE-ELEMENT", "RUNNABLE-ENTITY-VARIABLE-ACCESS", "DIAGNOSTIC-START-ROUTINE", "NM-CLUSTER", "MO", "NO-AFFECT", "DIAGNOSTIC-J-1939-SPN-MAPPING", "GRANT", "NO-TRUSTED-PLATFORM-SUPPORT", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED", "HARDWARE-TEST-NEEDS", "AUTOMATIC", "SECURITY-EVENT-ONE-EVERY-N-FILTER", "RAW", "PHM-RECOVERY-ACTION-INTERFACE", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "DIAGNOSTIC-IUMPR-GROUP", "COMPOSITE-INTERFACE", "PHM-ACTION", "SEARCH-FOR-ANY", "DO-IP-LOGICAL-ADDRESS", "SUPERVISION-ENTITY", "SO-AD-ROUTING-GROUP", "SDG-TAILORING", "TG", "CALIBRATION-ONLINE", "DIAGNOSTIC-PORT-INTERFACE", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "CAN-FRAME-TRIGGERING", "RPT-EXECUTABLE-ENTITY", "TRACE", "CAN-TP-CHANNEL", "CAN-20", "COMPILER", "SERIALIZATION-TECHNOLOGY", "NM-INTERACTS-WITH-SM-MAPPING", "ECU", "EXCLUDE-FROM-FLASH", "SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "DERIVED-FROM", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "240", "NO-TRANSFORMER-STATUS-FORWARDING", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "IDSM-QUALIFIED-EVENT-RECEIVER-INTERFACE", "DECREASING", "RPT-SERVICE-POINT", "COM-TRIGGER-GRANT-DESIGN", "AM", "UDP-NM-NODE", "FORWARD-AS-IS", "ISO-15031--6", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "I-PV-6-EXT-HEADER-FILTER-SET", "APPLIED-STANDARD", "DETERMINISTIC-CLIENT", "NV-DATA-INTERFACE", "RAPID-PROTOTYPING-SCENARIO", "DIAGNOSTIC-INDICATOR-NEEDS", "STATE-MANAGEMENT-SLEEP-ACTION-ITEM", "GROSS", "TESTED", "-500-MILES", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "IDSM-TIMESTAMP-PROVIDER-INTERFACE", "USE-VOID", "V-2-X-NOT-SUPPORTED", "ICV-VERIFIED", "IN", "CAN-TP-ADDRESS", "BUILD-ACTION", "SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "SRGB", "DDS-PROVIDED-SERVICE-INSTANCE", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING", "COM-FIELD-GRANT-DESIGN", "MASEKD-NEW-EQUALS-X", "NOT-EQUAL", "TS", "CLIENT-SERVER-OPERATION", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "MALFUNCTION", "SERVICE-INTERFACE-FIELD-MAPPING", "USER-DEFINED-FIELD-DEPLOYMENT", "I-SIGNAL-GROUP", "DIAGNOSTIC-CONTROL-NEEDS", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "FAULT", "CAN-COMMUNICATION-CONNECTOR", "SWC-INTERNAL-BEHAVIOR", "BASE-T", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "LIFE-CYCLE-STATE", "ABSTRACT-PROVIDED-PORT-PROTOTYPE", "CHANNEL-A", "PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR", "SECURED-PDU-HEADER-32-BIT", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "J-1939-NM--NCA", "TIP", "DDS-SECURE-GOVERNANCE", "UDP-CHECKSUM-DISABLED", "INFINITE-TIME-TO-RESPONSE", "LOG-AND-TRACE-INTERFACE", "COM-FIND-SERVICE-GRANT", "MN", "SCHEDULE-VARIANT-4", "ATP-FEATURE", "STATE-MANAGEMENT-STATE-REQUEST", "REST-NUMBER-PROPERTY-DEF", "APPLICATION-ASSOC-MAP-ELEMENT", "LIGHT", "APPLICABILITY-INFO-SET", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "SUPERVISION-MODE-CONDITION", "4-4-4", "SDG-ATTRIBUTE", "VEHICLE-ROLLOUT-STEP", "CONFIDENTIALITY-OFFSET--30", "ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED", "BSW-MODULE-ENTITY-STARTED", "FIELD", "ECUC-LINKER-SYMBOL-DEF", "PDU-R", "STATE-MANAGEMENT-ERROR-INTERFACE", "APMC-REFERENCE-DEF", "TEST-PASSED", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "RUNNABLE-ENTITY", "VAR-NO-INIT", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "BSW-INTERRUPT-EVENT", "CALPRM", "TRANSFORMATION-PROPS-SET", "WARMUP", "SERVICE-ONLY", "DELETE", "RM", "MANUAL-BY-PARTICIPANT", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "ROTATE-90-CCW-LIMIT-TO-TEXT", "PRESENTATION-CONTINUOUS", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "ETHERNET-PRIORITY-REGENERATION", "preserve", "IDSM-CONTEXT-PROVIDER-INTERFACE", "VIDEO-FRAME", "TT", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "ALL-INDICES-DIFFERENT-ARRAY-SIZE", "STATE-MANAGEMENT-MODULE-INSTANTIATION", "EVENT-HANDLER", "SERVICE-INSTANCE-COLLECTION-SET", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "EVENT-STORAGE-ENABLED", "REPORTS-EXECUTION-STATE", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "ENHANCED", "XDOC", "EXTERNAL-TRIGGERING-POINT-IDENT", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "NO-HEADER", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED", "DLT-APPLICATION", "SV", "TD-EVENT-MODE-DECLARATION", "RAW-DATA-STREAM-DEPLOYMENT", "MIXED", "IP-IAM-REMOTE-SUBJECT", "TRAP", "PSK", "CP", "KU", "BSW-M-ENTRY-CALL-RETURNED", "COLDSTART", "CRYPTO-CERTIFICATE-INTERFACE", "DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING", "CAN-BE-TERMINATED-AND-RESTARTED", "BSW-COMPOSITION-TIMING", "RPT-LEVEL-1", "MANUAL-BY-TOPIC", "MEASURED-EXECUTION-TIME", "AP-APPLICATION-ERROR", "CRC-OPTIONAL", "MONITOR-MODE", "X-MII", "SEC-OC-JOB-REQUIREMENT", "IS-EXPIRED", "FLEXRAY-AR-TP-CONFIG", "OBD-RATIO-DENOMINATOR-NEEDS", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE", "COMMAND-LINE-LONG-FORM", "IS-NOT-EQUAL", "OR", "REQUEST-CALLBACK-TYPE-SUPPLIER", "CRYPTO-PROVIDER", "J-1939-CONTROLLER-APPLICATION", "NO-SHOW-SHORT-NAME", "SUPERVISION-MODE", "DESELECTED", "WATCHDOG-ACTION-ITEM", "CIRCLE", "LAST-FAILED", "TRIGGERED-ON-EVALUATION", "MULTIPLE-OCCURRENCES", "LOCAL-SUPERVISION", "REQUEST-NO-RETURN", "DIAGNOSTIC-ECU-RESET-INTERFACE", "BSW-INTERNAL-BEHAVIOR", "ADAPTIVE-FIREWALL-MODULE-INSTANTIATION", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED", "INT-24-BIT", "TDLET-ZONE-CLOCK", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "COMMUNICATION-CONNECTOR", "COLOR-AWARE", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "OUT", "SQ", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "J-1939-NM-NODE", "ON-TRANSITION", "SHARED", "UCM-STEP", "BREAK", "RUNNABLE-ENTITY-TERMINATED", "HW-PIN-GROUP", "GLOBAL-TIME-GATEWAY", "SM-INTERACTS-WITH-NM-MAPPING", "PRODUCER", "MULTILANGUAGE-REFERRABLE", "MACHINE-TIMING", "AES-3-32-BIT", "SERVICE-INTERFACE-ELEMENT-MAPPING", "APMC-CHOICE-CONTAINER-REFERENCE-DEF", "DIAGNOSTIC-SOVD-FAULT-MEMORY-ACCESS", "SECOND-TO-FIRST", "CYCLE-REPETITION-16", "APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-DEF", "PERSISTENCY-INTERFACE-ELEMENT", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "CLIENT-ID-DEFINITION-SET", "DIAGNOSTIC-ROUTINE-NEEDS", "AR--CLIENT--SERVER", "SECURITY-EVENT-REPORT-INSTANCE-VALUE", "CLIENT-AUTHENTICATE", "APMC-STRING-PARAM-DEF", "FLEXRAY-COMMUNICATION-CONTROLLER", "SHOW-SEE", "AUTOSAR-DATA-PROTOTYPE", "I-SIGNAL-I-PDU-GROUP", "DIAGNOSTIC-COMPONENT-NEEDS", "CAN", "INTER-PARTITION-INTRA-ECU", "AUTO-IP", "4-4-4-4", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "NO-ACK", "ABSTRACT-REQUIRED-PORT-PROTOTYPE", "IDSM-REPORTING-MODE-PROVIDER-INTERFACE", "CAT-2", "ALIAS-NAME-SET", "88-2-KHZ", "OBD-DRIVING-CYCLE", "ADAPTIVE-FIELD-NOTIFICATION-RECEIVED", "ALLOCATOR", "CHECKPOINT-TRANSITION", "DIAGNOSTIC-MAPPING", "MIDDLE", "SERVER-ENCRYPT", "APPLICATION-ASSOC-MAP-DATA-TYPE", "TRIGGER", "CYCLE-REPETITION-20", "GLOBAL-TIME-MASTER", "ABSTRACT-CAN-CLUSTER", "SHARED-VLAN-LEARNING", "DO-IP-NETWORK-CONFIGURATION-DESIGN", "APMC-PARAMETER-VALUE", "ABSTRACT-CAN-COMMUNICATION-CONTROLLER", "ROOT-SW-COMPONENT-PROTOTYPE", "EVENT-MAPPING", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-DIAGNOSTIC-EVENT-MAPPING", "CONDITIONAL", "MODE-DECLARATION-MAPPING", "FIXED", "DIAGNOSTIC-INDICATOR", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "PRESENTATION-DISCRETE", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "TD-EVENT-CYCLE-START", "USER-DEFINED-GLOBAL-TIME-SLAVE", "KEY-SERVER", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "CLIENT-ENCRYPT", "ADAPTIVE-MODULE-INSTANTIATION", "DE", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE", "ECUC-QUERY", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "FM-FEATURE-MAP-CONDITION", "PAYLOAD-AS-ARRAY", "DDS-CP-SERVICE-INSTANCE", "SPECIFICATION-DOCUMENT-SCOPE", "END-TO-END-PROTECTION-SET", "TIMING-CLOCK", "MODE-DECLARATION-MAPPING-SET", "INTRUSION-DETECTION-SECURITY-MANAGEMENT", "VENDOR", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "EN", "NO-COM", "STATE-MANAGEMEN-PHM-ERROR-INTERFACE", "PROCESSING-STYLE-ASYNCHRONOUS", "TIME-SYNCHRONIZATION-INTERFACE", "FULL-DUPLEX-MODE", "PHYSICAL", "SOMEIP-PROVIDED-EVENT-GROUP", "TOP", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "BSW-INTERRUPT-ENTITY", "SWC-TO-IMPL-MAPPING", "REMOVE", "COUPLING-PORT", "CALIBRATION-OFFLINE", "DIAGNOSTIC-J-1939-SW-MAPPING", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "SCHEDULE-VARIANT-6", "APMC-CONTAINER-REFERENCE-DEF", "MODE-SWITCH-INTERFACE", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "WRITE", "SYSTEM-TIMING", "UNDECIDED", "SOMEIP-FIELD-DEPLOYMENT", "FAILURE-AND-SUCCESS", "CALLBACK", "DIAGNOSTIC-TEST-RESULT", "HIGH", "GU", "I-4-G", "IEEE-1722-TP-ACF-LIN", "HIERARCHICAL-EOC", "ALLOW", "BSW-EVENT", "LIN-TP-NODE", "IEEE-1722-TP-ACF-CAN-PART", "DIAGNOSTIC-SOVD-METHOD", "STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE", "USE-FIRST-CONTEXT-DATA", "USES-LOGGING", "IMMEDIATE", "PROCESSOR-CORE", "ETHERNET-RAW-DATA-STREAM-MAPPING", "DDS-CP-QOS-PROFILE", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY", "SILENT", "STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE", "SH", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-PORT-MAPPING", "AMBER-WARNING", "INTERNAL-BEHAVIOR", "TARGET-CONTAINER", "NO-PGWIDE", "CRYPTO-KEY-MANAGEMENT", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "SW", "VENDOR-SPECIFIC", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "ABSTRACT-RAW-DATA-STREAM-INTERFACE", "HOST-PORT", "EO", "PTP--IEEE-1588--2002", "DIAGNOSTIC-AUTHENTICATION", "CAN-FRAME", "DIAGNOSTIC-IUMPR", "OTHER", "SOMEIP-EVENT-DEPLOYMENT", "CYCLIC-AND-ON-CHANGE", "HEAD", "OS-TASK-PROXY", "AY", "DIAGNOSTIC-ROUTINE-INTERFACE", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "ANY-PARTIAL-NETWORK-ACTIVE", "TRACEABLE-TABLE", "100", "SHOW-CONTENT", "TIME", "NOTHING", "ETHERNET-PHYSICAL-CHANNEL", "NO-BOOT", "FLEXRAY-PHYSICAL-CHANNEL", "IDSM-MODULE-INSTANTIATION", "SWITCH-FLOW-METERING-ENTRY", "HEALTH-CHANNEL-SUPERVISION", "SW-SERVICE-PROTOTYPE", "BULK-NV-DATA-DESCRIPTOR", "DETAILED-BYPASSING-FILTERS", "DDS-SIGNAL", "DEBOUNCE-DATA", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "EVENT-WINDOW-CURRENT-CYCLE", "REFERRABLE", "APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-VALUE", "TRACE-SWITCH-LOG", "MODE-TRANSITION", "SW-CLASS-INSTANCE", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "TLS-IAM-REMOTE-SUBJECT", "MONOTONOUS", "SYSTEM-SUPPLIER-BOOT", "DLT-LOG-CHANNEL-DESIGN", "SSDP", "POSSIBLE-ERROR-REACTION", "PRIMARY-ECU", "PER-INSTANCE-MEMORY", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "ECUC-STRING-PARAM-DEF", "J-1939-REQUEST-MANAGER", "LIN-TP-CONFIG", "RSA", "XCP-PDU", "SEARCH-FOR-ALL", "FIRE-AND-FORGET-MAPPING", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "BMP", "ECUC-QUERY-EXPRESSION", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "NEW-IS-GREATER-OR-EQUAL", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "CONCRETE-PATTERN-EVENT-TRIGGERING", "SYNCHRONIZED", "TD-EVENT-I-SIGNAL", "COUPLING-ELEMENT-ABSTRACT-DETAILS", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "COMMAND-LINE-SIMPLE-FORM", "DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "DEVELOPMENT", "CYCLE-REPETITION-2", "16-KHZ", "DO-IP-ACTIVATION-LINE-NEEDS", "SERVER-VERIFY", "SYNCHRONIZED-TIME-BASE-PROVIDER", "CRYPTO-NEEDS", "MODE-DECLARATION-GROUP-PROTOTYPE", "COUPLING-ELEMENT-SWITCH-DETAILS", "DIAGNOSTIC-SOVD-SERVICE-INSTANCE", "STRICTLY-INCREASING", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "SCHEDULE-VARIANT-5", "CRYPTO-SERVICE-MAPPING", "85", "X-509", "DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER", "XG-MII", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "ECU-PARTITION", "DLT-APPLICATION-TO-PROCESS-MAPPING", "TRANSFORMER-ERROR-HANDLING", "REGULAR", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "REST-ELEMENT-DEF", "DEADLINE-SUPERVISION", "DEFINE-BY-MEMORY-ADDRESS", "TRANSFORMATION-I-SIGNAL-PROPS-IDENT", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "DATA-TRANSFORMATION-SET", "STD", "PROCESS", "SI", "TD-EVENT-FRAME", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "DETERMINISTIC-SYNC-MASTER", "SOFTWARE-CLUSTER", "NAND", "DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING", "FULL", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "WILL-CALL", "FINISH", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "PASSTHROUGH", "LEFT", "APMC-FOREIGN-REFERENCE-DEF", "1-8", "BSW-DATA-RECEIVED-EVENT", "AP-SOMEIP-TRANSFORMATION-PROPS", "HW-CATEGORY", "NEW-IS-WITHIN", "INTERRUPT", "APMC-CONTAINER-DEF", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "STATE-MANAGEMENT-ACTION-ITEM", "GLOBAL-SUPERVISION-NEEDS", "START-ON-BOOT", "TLV-DATA-ID-DEFINITION-SET", "KEY-STORAGE", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "I-SIGNAL-AVAILABLE-FOR-RTE", "KEEP-LAST", "TLS-CRYPTO-CIPHER-SUITE", "END-2-END-METHOD-PROTECTION-PROPS", "UCM-MASTER", "RES-AXIS", "AGE", "DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE", "GZIP", "SUPERVISION-CHECKPOINT", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "MODE-DECLARATION-GROUP", "I-SIGNAL-SENT-TO-COM", "POST-BUILD-VARIANT-CRITERION", "VO", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "DIAGNOSTIC-J-1939-NODE", "PR-PORT-PROTOTYPE", "1-1-001", "PERSISTENCY-INTERFACE", "SENT-UNTAGGED", "DIAGNOSTIC-CONNECTION", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "BSW-MODULE-TIMING", "TRIGGERED", "ASYNCHRONOUS-SERVER-CALL-POINT", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "ROTATE-90-CW", "APPLICATION-DATA-TYPE", "COUPLING-ELEMENT", "HW-DESCRIPTION-ENTITY", "CLIENT-MAC-VERIFY", "REST-ENDPOINT-GET", "PHM-ACTION-ITEM", "ATOMIC-SW-COMPONENT-TYPE", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "GETTER-SETTER", "IMPOSITION-TIME", "IP-SEC-RULE", "CRYPTO-SERVICE-JOB-NEEDS", "FM-FEATURE-SELECTION", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "DIAGNOSTIC-TRANSFER-EXIT", "PHYSICAL-CHANNEL", "CLIENT-DECRYPT", "RTE-EVENT-IN-SYSTEM-SEPARATION", "DIAGNOSTIC-COMMON-ELEMENT", "PRIMITIVE-ATTRIBUTE-TAILORING", "ON-CHANGE", "DEFAULT-TRACE-STATE-ENABLED", "RPT-EXECUTABLE-ENTITY-EVENT", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "ECUC-DEFINITION-ELEMENT", "CSERS", "BUILD", "AND", "ON-EXIT", "KEY-DERIVATION", "ECUC-DEFINITION-COLLECTION", "OBSERVER", "STATE-MANAGEMENT-SYNC-ACTION-ITEM", "ACL-OPERATION", "SECURE-COMMUNICATION-DEPLOYMENT", "FLEXRAY-CLUSTER", "UNIT", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "USER-DEFINED-I-PDU", "120", "AZ", "EVENT-WINDOW-INFINITE", "TRIGGERED-ON-CHANGE", "APMC-ABSTRACT-INSTANCE-REFERENCE-VALUE", "DIAGNOSTIC-TROUBLE-CODE-OBD", "SYNCHRONIZATION-POINT-CONSTRAINT", "PDU-TRIGGERING", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "BSW-TIMING-EVENT", "PORT-PROTOTYPE-BLUEPRINT", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "AFTERMAKET", "SW-SERVICE-ARG", "ASYMMETRIC-FROM-BYTE-ARRAY", "SN", "APMC-BOOLEAN-PARAM-DEF", "PROCESS-EXECUTION-ERROR", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "TX-REF-TRIGGER", "VARIANT-POST-BUILD", "UNSPECIFIED", "RESET-MACHINE", "MOST-SIGNIFICANT-BYTE-FIRST", "NV-RAM-MANAGER", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "R-PORT-PROTOTYPE", "DIAGNOSTIC-SECURITY-LEVEL", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "AUTOSAR-VARIABLE-INSTANCE", "SIMULATED-EXECUTION-TIME", "DIAGNOSTIC-SECURITY-ACCESS", "ASSEMBLY-SW-CONNECTOR", "DDS-EVENT-DEPLOYMENT", "RN", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "CLIENT-SERVER-INTERFACE-MAPPING", "RECOVERY-NOTIFICATION", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "MACRO", "J-1939-TP-CONFIG", "FURTHER-ACTION-BYTE-NEEDS", "USER", "GN", "IEEE-1722-RAW-DATA-STREAM-CONSUMER-MAPPING", "1000BASE-T", "API", "ECUC-PARAM-CONF-CONTAINER-DEF", "PERIODIC-RATE-MEDIUM", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "PHYSICAL-DIMENSION", "BSW-DIRECT-CALL-POINT", "PDF", "AUTHENTICATE", "ECU-INSTANCE", "DIAGNOSTIC-SW-MAPPING", "FAILURE-ONLY", "SW-CODE-SYNTAX", "EXCLUSIVE", "SOMEIP-REMOTE-MULTICAST-CONFIG", "ABSTRACT", "IS-FAILED", "DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING", "BSW-ENTRY-RELATIONSHIP-SET", "OEM-BOOT-RESP-APP", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING", "EXECUTABLE-GROUP", "ECU-STATE-MGR-USER-NEEDS", "USER-DEFINED-CLUSTER", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "APMC-URI-INSTANCE-REFERENCE-DEF", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "DIAGNOSTIC-EVENT", "EXCLUSIVE-AREA", "FJ", "DEFAULT-ERROR-TRACER", "APMC-URI-FOREIGN-REFERENCE-VALUE", "OCCURENCE", "VIEW-MAP", "DIAGNOSTIC-READ-DTC-INFORMATION", "APMC-ABSTRACT-INSTANCE-REFERENCE-DEF", "INDIVIDUAL", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE", "UR", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "CONSISTENCY-NEEDS", "DIAGNOSTIC-SOVD-UPDATE-INTERFACE", "COLLECTION", "PHM-HEALTH-CHANNEL-INTERFACE", "IS-GREATER-THAN", "FDC-THRESHOLD", "1-0", "TEST-FAILED-THIS-OPERATION-CYCLE", "ATP-BLUEPRINTABLE", "APPLICATION-ONLY", "BUILD-TYPE-DEBUG", "LIN-EVENT-TRIGGERED-FRAME", "OFFSET-TIMING-CONSTRAINT", "DHCPV-4", "NOT-AVAILABLE", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "PDU", "WATCH-DOG-MANAGER", "ALL", "STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM", "CONCRETE-CLASS-TAILORING", "USE-LAST-CONTEXT-DATA", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "APPLICATION", "PUT", "DIAGNOSTIC-MONITOR-INTERFACE", "SIGNAL-BASED-METHOD-DEPLOYMENT", "30", "PORT-INTERFACE", "STORE-EVENT", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "SHORT-HEADER", "DIAGNOSTIC-DE-AUTHENTICATION", "FLOAT", "PERSISTENCY-DEPLOYMENT-ELEMENT", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER", "VERBOSE", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "HEALTH-CHANNEL", "EVENT-TRIGGERING-CONSTRAINT", "SK", "ANY-SEND-OPERATION", "SCHEDULE-VARIANT-1", "DLNA", "NO-TRANSFORMER-ERROR-HANDLING", "TIMING-CONDITION", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "SECURITY-EVENT-MAPPING", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "DIAGNOSTIC-SOVD-UPDATE", "MODE-DECLARATION-REQUESTED", "PCM", "RO", "CODE-GENERATION-TIME", "DLT-ECU", "ADAPTIVE-EVENT-SENT", "BOLD", "TD-EVENT-TT-CAN-CYCLE-START", "USER-DEFINED-EVENT-DEPLOYMENT", "ANY", "APPLICATION-COMPOSITE-DATA-TYPE", "XREF-TARGET", "VOLATILE", "DIAGNOSTIC-UPLOAD-INTERFACE", "DIAGNOSTIC-J-1939-SPN", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "SECURED-PDU-HEADER-16-BIT", "FORGET", "SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "ROTATE-180-LIMIT-TO-TEXT", "MASTER-ECU", "TD-EVENT-VFB-REFERENCE", "DIAGNOSTIC-SOVD-PORT-INTERFACE", "BSW-MODULE-DEPENDENCY", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "DIAGNOSTIC-FREEZE-FRAME", "DDS-REQUIRED-SERVICE-INSTANCE", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "ECUC-CHOICE-CONTAINER-DEF", "TD-EVENT-COM", "SEC-OC-DEPLOYMENT", "ALL-SUPPORTED-DTCS", "CAN-TP-NODE", "OFF", "MULTIPLE", "SERVICE-INTERFACE-MAPPING", "TRANSFER", "PLATFORM-PHM-ACTION-ITEM", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "RTPGE", "TRACE-SWITCH-ARTI", "DOCUMENTATION-CONTEXT", "SCHEDULE-VARIANT-2", "DIAGNOSTIC-ROUTINE-SUBFUNCTION", "ACTION-ITEM", "SLP", "LIN-UNCONDITIONAL-FRAME", "J-1939-NM-CLUSTER", "TTCAN-CLUSTER", "PERSISTENT", "ACTIVATION-UNICAST", "STOP-TRIGGER", "LV", "SERVICE-INTERFACE-METHOD-MAPPING", "50", "8", "J-1939-DCM-DM-19-SUPPORT", "IEEE-1722-TP-ACF-LIN-PART", "CRYPTO-DRIVER", "LAST-MODE", "GATEWAY", "NO-SHOW-PAGE", "32-KHZ", "DIAGNOSTIC-CONTRIBUTION-SET", "ACTIVE", "MASKED-NEW-EQUALS-X", "LIN-COMMUNICATION-CONTROLLER", "MACHINE-CYCLE", "TW", "FIRE-AND-FORGET-METHOD-MAPPING", "DEFINE-BY-IDENTIFIER", "INLINE", "FLEXRAY-TP-NODE", "DLT-MESSAGE", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "DETERMINISTIC-SYNC-INSTANTIATION", "DIAGNOSTIC-DATA-TRANSFER", "UPDATE", "SOVD-MODULE-INSTANTIATION", "SL", "V-2-X-ACTIVE-SUPPORTED", "AA", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE", "COM-GRANT", "LOGIC-ADDRESS", "STORE-PERSISTENTLY", "TIME-SYNC-SERVER-CONFIGURATION", "DIAGNOSTIC-PARAMETER-IDENT", "STRUCTURED-REQ", "CRYPTO-KEY-SLOT-USER-DESIGN-MAPPING", "J-1939-NM--SCA", "LIMIT-TO-TEXT", "XH", "MANUFACTURING", "CRYPTO-KEY-MANAGEMENT-NEEDS", "COM-EVENT-GRANT", "COMMUNICATION-CONTROLLER", "SAE-J-2012--DA", "TLS-SECURE-COM-PROPS", "PROVIDED-AP-SERVICE-INSTANCE", "RPT-LEVEL-2", "COMM-CONNECTOR-PORT", "RELIABLE", "EVAPPURGEFLOW", "IEEE-1722-RAW-DATA-STREAM-PRODUCER-MAPPING", "RUNNABLE-ENTITY-GROUP", "TESTED-AND-FAILED", "SINGLE", "INIT-EVENT", "TLS-CONNECTION-GROUP", "IE", "DIAGNOSTIC-TRANSMIT-CERTIFICATE-INTERFACE", "CRC-NOT-VALIDATED", "SECURITY", "SWITCH-STREAM-FILTER-ACTION-DEST-PORT-MODIFICATION", "MAC-ADDRESS-VLAN-MEMBERSHIP", "CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "HR", "SERVICE-NEEDS", "TRANSPORT", "INCLUDE-BUT-DO-NOT-START", "10", "BURST-PATTERN-EVENT-TRIGGERING", "APMC-ABSTRACT-REFERENCE-VALUE", "VAR-FAST", "METHOD-MAPPING", "HEAP-USAGE", "COMMON", "J-1939", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-PORT-MAPPING", "ADAPTIVE-FIELD-NOTIFICATION-SENT", "STRICT-MODE", "SOFTWARE-PACKAGE-STEP", "IS-LESS-THAN", "CALIBRATION-VARIABLES", "BLUEPRINT-DERIVATION-TIME", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "COM-GRANT-DESIGN", "AR", "IEC-61937", "MASKED-NEW-EQUALS-MASKED-OLD", "SYNCHRONOUS", "SERVICE-INTERFACE-TRIGGER-MAPPING", "FM-ATTRIBUTE-DEF", "CAN-XL-PROPS", "ADAPTIVE-AUTOSAR-APPLICATION", "MK", "COMPOSITION-SW-COMPONENT-TYPE", "USER-DEFINED-METHOD-DEPLOYMENT", "176-4-KHZ", "PT", "SHOW-LONG-NAME", "300", "APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-VALUE", "EID-USE-MAC", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "STARTUP-CONFIG", "IDSM-RATE-LIMITATION", "INTERFACE-MAPPING", "STATE-MANAGEMENT-REQUEST-INTERFACE", "RUN-CONTINUOUS", "MC-FUNCTION", "ST", "CONSUMED-EVENT-GROUP", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "CLEAR-ALL-DTCS", "CODEGENERATION", "DATA-EXCHANGE-POINT", "ORDINARY-EOC", "FILE", "DIAGNOSTIC-MULTIPLE-RESOURCE-PORT-MAPPING", "NO-SHOW-NUMBER", "DIAGNOSTIC-SESSION", "NUMBER", "CRYPTO-ELLIPTIC-CURVE-PROPS", "ON-CHANGE-OF-DATA-IDENTIFIER", "PHM-ACTION-LIST", "TH", "TRACEABLE", "LATENCY-TIMING-CONSTRAINT", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "MAC-SEC-GLOBAL-KAY-PROPS", "GETTER", "CAN-NM-NODE", "TIMING-DESCRIPTION-EVENT-CHAIN", "TIME-SLAVE", "SYMMETRIC", "API-USE", "IAM-MODULE-INSTANTIATION", "SIDES", "DIAGNOSTIC-DATA-ELEMENT", "UCM", "STOP", "FY", "SEARCH-FOR-ALL-INSTANCES", "LIN-FRAME-TRIGGERING", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "NOT", "DIAGNOSTIC-PARAMETER-ELEMENT", "OPERATION-CALL-RESPONSE-SENT", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "STATIC-SOCKET-CONNECTION", "CRYPTO-KEY-SLOT-DESIGN", "HALF-DUPLEX-MODE", "BRIEF-BYPASSING-FILTERS", "CONFIDENTIALITY-OFFSET--0", "SM", "GRANT-DESIGN", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "ETHERNET-NETWORK-CONFIGURATION", "WO", "DATA-PROTOTYPE", "V-2-X-DATA-MANAGER-NEEDS", "BSW-SCHEDULABLE-ENTITY", "FRAME-TRANSMITTED-ON-BUS", "FRAME", "ECU-TIMING", "ECC", "GENERAL-PURPOSE-PDU", "TD-EVENT-BSW-MODE-DECLARATION", "REDUNDANT-PER-KEY", "IEEE-1722-TP-ACF-BUS", "VERIFICATION", "TRIGGER-INTERFACE-MAPPING", "NO-SLOPPY", "CANNOT-BE-REMOVED", "DATA-WRITE-COMPLETED-EVENT", "STARTUP-CONFIG-SET", "CRYPTO-KEY-SLOT", "TIME-MASTER", "IP-SEC-IAM-REMOTE-SUBJECT", "SECURITY-EVENT-THRESHOLD-FILTER", "OPEN", "INDEPENDENT-VLAN-LEARNING", "TOPIC-1", "BSW", "DIAGNOSTIC-OPERATION-CYCLE", "TIMING-EXTENSION-RESOURCE", "COM_AXIS", "CURVE_AXIS", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "EOC-EXECUTABLE-ENTITY-REF", "OS-TASK-EXECUTION-EVENT", "REPORT", "GENERAL-PURPOSE-I-PDU", "RAW-DATA", "HINT", "SYMBOLIC-NAME-PROPS", "ROTATE-90-CCW", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "KEYWORD", "OBD-INFO-SERVICE-NEEDS", "DIAGNOSTIC-ECU-RESET-CLASS", "SEC-OC-CRYPTO-SERVICE-MAPPING", "BSW-MGR-NEEDS", "WRONG-TRIGGER", "BLOCK", "CONTAINER-I-PDU", "CONTINUE-AT-IT-POSITION", "BH", "CRYPTO-SERVICE-QUEUE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "START-FROM-BEGINNING", "BSW-MODE-MANAGER-ERROR-EVENT", "COUPLING-PORT-SHAPER", "APPLICATION-RECORD-ELEMENT", "CRC-VALIDATED", "OBSERVER-BASED", "UPLOADABLE-DESIGN-ELEMENT", "SERVICE-INTERFACE-EVENT-MAPPING", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "ADAPTIVE-METHOD-RESPONSE-RECEIVED", "SECURITY-EVENT-REPORT-INSTANCE-DEFINITION", "DIAG-REQUEST", "NON-REENTRANT", "SERVICE-INSTANCE-TO-MACHINE-MAPPING", "BSW-MODULE-ENTITY-TERMINATED", "PS", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "VALID", "RPT-LEVEL-3", "ECUC-BOOLEAN-PARAM-DEF", "STATE-MANAGEMENT-PHM-ERROR-INTERFACE", "MACHINE", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "CRYPTO-KEY-SLOT-CLIENT-INTERFACE", "SYNC-BASE-TIME-MANAGER", "UK", "TI", "APMC-PARAM-CONF-CONTAINER-DEF", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "EOC-EVENT-REF", "LISTEN", "BRIEF", "PRIVATE-KEY", "TIME-SYNC-MODULE-INSTANTIATION", "FIREWALL-RULE", "REST-SERVICE-INTERFACE", "STIMULUS-SYNCHRONIZATION", "IDSM-CONTEXT-PROVIDER-MAPPING", "SW-SYSTEMCONSTANT-VALUE-SET", "DIAGNOSTIC-CONDITION-INTERFACE", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS", "CS", "NEW-IS-OUTSIDE", "SW-MC-BASE-TYPE", "EXCLUSIVE-AREA-NESTING-ORDER", "NV-BLOCK-SW-COMPONENT-TYPE", "PDU-ACTIVATION-ROUTING-GROUP", "ATP-PROTOTYPE", "RETURN-VALUE-PROVIDED", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "NM-PDU", "POWER-WINDOW-TIME", "AR-PACKAGE", "YCM", "SWC-TIMING", "EXAMPLE", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "NV-BLOCK-DESCRIPTOR", "XOR", "EXERCISE", "PARAMETER-SW-COMPONENT-TYPE", "REJECT", "PRIO-OCC", "MINIMUM-MINOR-VERSION", "IS-GREATER-OR-EQUAL", "RUN-ONCE", "NM-NODE", "MC-DATA-INSTANCE", "OFFSET", "USER-DEFINED-GLOBAL-TIME-MASTER", "BI", "ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER", "J-1939-DCM", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "OVERWRITE", "FLEXRAY-TP-CONFIG", "ISO-6", "DIAGNOSTIC-CLEAR-CONDITION", "LT", "ALL-16-BIT", "STATUS-BIT-NORMAL", "APMC-CONTAINER-ELEMENT-VALUE", "APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-DEF", "VARIABLE-ACCESS", "HW-ELEMENT", "OEM-BOOT", "REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE", "APMC-ENUMERATION-LITERAL-DEF", "RUNTIME-ERROR", "IDS-COMMON-ELEMENT", "ECUC-ABSTRACT-REFERENCE-DEF", "USE-ARRAY-BASE-TYPE", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "EXECUTABLE-TIMING", "BACKGROUND-EVENT", "UCM-SUBORDINATE-MODULE-INSTANTIATION", "ON-COMPARISON-OF-VALUES", "GLOBAL-TIME-CAN-MASTER", "CYCLE-REPETITION-1", "PLATFORM-MODULE-ENDPOINT-CONFIGURATION", "RAW-DATA-STREAM-GRANT", "DO-IP-FUNCTIONAL-CLUSTER-DESIGN", "BSW-MODULE-CALL-POINT", "REQUEST-CALLBACK-TYPE-MANUFACTURER", "TUNNEL", "KK", "DIAGNOSTIC-COMMUNICATION-MANAGER", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "PRE-COMPILE", "BINARY-MANIFEST-RESOURCE", "ECUC-PARAMETER-DEF", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "DO-IP-TP-CONFIG", "ROUGH-ESTIMATE-STACK-USAGE", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "CAPTION", "USER-DEFINED", "AUTOSAR-DATA-TYPE", "FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION", "OM", "ENABLE", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "SLAVE-ACTIVE", "IP-SEC-CONFIG-PROPS", "TASK", "DO-IP-GID-NEEDS", "24-KHZ", "STATE-MANAGEMENT-ACTION-LIST", "ICV-OPTIONAL", "NEWLINE-IF-NECESSARY", "STATE-CLIENT-INTERFACE", "10000BASE-T1", "DLT-LOG-CHANNEL", "SECURITY-EVENT-CONTEXT-MAPPING", "NOT-SENT", "SO", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "5", "16", "BN", "PORT-INTERFACE-DEFINITION", "DIAGNOSTIC-REQUEST-ROUTINE-RESULTS", "VIEW-MAP-SET", "8-KHZ", "RPT-CONTAINER", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "REST-PRIMITIVE-PROPERTY-DEF", "MR", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "DIAGNOSTIC-EVENT-NEEDS", "NA", "SINGLE-LANGUAGE-REFERRABLE", "STANDARD-PORT", "CAUTION", "DOCUMENT-ELEMENT-SCOPE", "STRICT-PRIORITY", "DIAGNOSTIC-CONNECTED-INDICATOR", "AB", "DIAGNOSTIC-EVENT-INTERFACE", "GENERIC-TP-CONNECTION", "TD-EVENT-SLLET", "SERVICE-TIMING", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION", "SINGLE-OCCURRENCE", "ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION", "IDS-PLATFORM-INSTANTIATION", "DIAGNOSTIC-SERVICE-INSTANCE", "UPLOADABLE-PACKAGE-ELEMENT", "ISO"];

    /// derive an enum entry from an input string using a perfect hash function
    ///
    /// # Errors
    ///
    /// [`ParseEnumItemError`]: The input string did not match the name of any enum item
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseEnumItemError> {
        #[rustfmt::skip]
        static DISPLACEMENTS: [(u16, u16); 562] = [(0, 86), (0, 22), (0, 0), (0, 0), (0, 3), (0, 2), (0, 626), (0, 3), (0, 9), (0, 176), (0, 3), (0, 6), (0, 0), (0, 1), (0, 0), (0, 83), (0, 0), (0, 0), (0, 23), (0, 781), (0, 4), (0, 2), (0, 76), (0, 79), (0, 15), (0, 2), (0, 3), (0, 383), (0, 2), (0, 9), (0, 2), (0, 95), (0, 897), (0, 135), (0, 655), (0, 2), (0, 266), (0, 49), (0, 53), (0, 865), (0, 3), (0, 160), (0, 2), (0, 65), (0, 0), (0, 1), (0, 0), (0, 4), (0, 32), (0, 207), (0, 55), (0, 342), (0, 3), (0, 3), (0, 37), (0, 16), (0, 5), (0, 2), (0, 32), (0, 5), (0, 8), (0, 615), (0, 105), (0, 1), (0, 1), (0, 0), (0, 252), (0, 924), (0, 20), (0, 1), (0, 293), (0, 3), (0, 3), (0, 109), (0, 107), (0, 108), (0, 892), (0, 2), (0, 0), (0, 345), (0, 27), (0, 61), (0, 299), (0, 75), (0, 8), (0, 16), (0, 80), (0, 70), (0, 24), (0, 20), (0, 69), (0, 256), (0, 960), (0, 0), (0, 284), (0, 148), (0, 135), (0, 46), (0, 3), (0, 460), (0, 34), (0, 0), (0, 185), (0, 31), (0, 234), (0, 30), (0, 56), (0, 250), (0, 97), (0, 1320), (0, 2), (0, 1211), (0, 10), (0, 54), (0, 268), (0, 0), (0, 291), (0, 984), (0, 7), (0, 64), (0, 252), (0, 10), (0, 1214), (0, 15), (0, 14), (0, 591), (0, 5), (0, 4), (0, 15), (0, 621), (0, 526), (0, 331), (0, 82), (0, 360), (0, 65), (0, 0), (0, 0), (0, 1110), (0, 20), (0, 5), (0, 55), (0, 22), (0, 490), (0, 124), (0, 599), (0, 5), (0, 540), (0, 0), (0, 318), (0, 434), (0, 14), (0, 128), (0, 1508), (0, 121), (0, 0), (0, 8), (0, 4), (0, 53), (0, 13), (0, 0), (0, 647), (0, 234), (0, 4), (0, 55), (0, 28), (0, 116), (0, 12), (0, 478), (0, 0), (2, 720), (0, 1), (0, 0), (0, 29), (0, 3), (0, 11), (0, 77), (0, 81), (0, 1316), (0, 20), (0, 93), (0, 30), (0, 237), (0, 556), (0, 56), (0, 3), (0, 329), (1, 205), (0, 6), (0, 241), (0, 56), (0, 1356), (0, 1), (0, 280), (0, 186), (0, 43), (0, 141), (0, 516), (0, 1283), (0, 69), (0, 1725), (0, 3), (0, 8), (0, 84), (0, 43), (0, 11), (0, 64), (0, 2159), (0, 344), (0, 582), (0, 3), (0, 164), (0, 130), (0, 219), (0, 688), (0, 764), (0, 30), (0, 4), (0, 12), (0, 7), (0, 2), (0, 2), (0, 2406), (0, 3), (0, 1), (0, 4), (0, 167), (0, 350), (0, 37), (0, 501), (0, 232), (0, 86), (0, 12), (0, 19), (0, 593), (0, 2252), (0, 1361), (0, 527), (0, 340), (0, 485), (0, 7), (0, 72), (0, 163), (0, 101), (0, 1287), (0, 326), (0, 418), (0, 21), (0, 39), (0, 0), (0, 233), (0, 118), (0, 2), (0, 2775), (0, 170), (0, 0), (0, 938), (0, 313), (0, 180), (0, 18), (1, 464), (0, 570), (0, 295), (0, 60), (0, 0), (0, 0), (0, 1264), (0, 1265), (0, 339), (0, 1648), (0, 1284), (0, 523), (0, 34), (0, 148), (0, 0), (0, 25), (0, 278), (0, 1), (0, 74), (0, 1269), (0, 816), (0, 264), (0, 252), (0, 81), (1, 3), (0, 222), (0, 171), (0, 606), (0, 98), (0, 815), (0, 223), (0, 2), (0, 13), (0, 1417), (0, 373), (0, 0), (1, 297), (2, 542), (1, 5), (0, 380), (0, 241), (0, 6), (0, 1873), (1, 1573), (0, 17), (0, 762), (0, 12), (0, 5), (0, 39), (0, 92), (0, 2), (0, 30), (0, 86), (0, 1481), (1, 598), (0, 56), (0, 240), (0, 105), (0, 80), (0, 27), (0, 434), (0, 1197), (0, 44), (0, 111), (0, 129), (0, 258), (0, 1), (0, 2), (0, 2), (0, 541), (0, 2), (0, 1929), (0, 63), (0, 39), (0, 928), (0, 197), (0, 1161), (0, 10), (0, 0), (0, 2212), (0, 3), (0, 114), (0, 1637), (0, 27), (0, 533), (1, 979), (0, 170), (0, 84), (0, 1353), (0, 0), (0, 0), (0, 576), (0, 853), (0, 243), (0, 4), (0, 0), (0, 1), (0, 197), (0, 1), (1, 1086), (1, 1211), (0, 1991), (0, 2), (0, 4), (0, 191), (0, 1258), (0, 132), (0, 14), (0, 136), (0, 59), (0, 2516), (0, 10), (0, 22), (0, 144), (0, 0), (0, 2060), (0, 257), (0, 37), (0, 137), (0, 5), (0, 0), (1, 2132), (0, 343), (0, 6), (0, 790), (0, 0), (0, 1682), (0, 41), (0, 11), (0, 970), (0, 2451), (1, 1365), (0, 29), (0, 16), (0, 40), (0, 8), (1, 1430), (0, 421), (0, 18), (0, 2595), (0, 250), (0, 1118), (0, 18), (0, 289), (0, 26), (0, 1128), (0, 1406), (0, 2), (0, 38), (0, 26), (0, 53), (0, 1478), (0, 593), (1, 690), (0, 123), (1, 23), (0, 88), (0, 42), (0, 7), (2, 1149), (0, 261), (0, 56), (0, 478), (0, 168), (2, 1004), (0, 1879), (0, 2609), (0, 2), (0, 261), (0, 137), (0, 41), (0, 508), (0, 1048), (0, 4), (0, 615), (0, 2146), (0, 5), (0, 13), (0, 17), (1, 2211), (0, 13), (2, 2470), (0, 95), (0, 1961), (0, 655), (0, 95), (0, 0), (1, 8), (0, 827), (0, 1023), (0, 236), (0, 24), (0, 22), (0, 452), (0, 13), (2, 2410), (0, 854), (0, 2027), (0, 89), (0, 1847), (0, 1), (1, 1599), (0, 176), (0, 268), (1, 6), (0, 12), (0, 712), (1, 1195), (2, 1247), (2, 1672), (0, 2), (0, 0), (0, 268), (0, 5), (0, 609), (0, 120), (5, 1919), (0, 1142), (0, 68), (0, 1402), (0, 2311), (0, 101), (0, 1052), (0, 0), (0, 2472), (0, 529), (2, 1283), (0, 1070), (2, 1375), (1, 1153), (0, 152), (0, 80), (5, 450), (2, 2444), (0, 29), (0, 1), (0, 46), (0, 1733), (0, 64), (0, 39), (0, 1916), (0, 223), (0, 1483), (0, 11), (1, 0), (0, 0), (0, 318), (0, 266), (4, 1853), (0, 2590), (0, 180), (0, 2134), (0, 24), (0, 1302), (1, 1532), (0, 319), (0, 192), (2, 630), (0, 1), (1, 245), (0, 1272), (6, 1041), (0, 227), (0, 265), (0, 0), (0, 449), (0, 0), (0, 1), (0, 10), (0, 0), (0, 20), (0, 0), (0, 0), (0, 1961), (0, 16), (0, 1034), (1, 1514), (0, 33), (0, 232), (0, 8), (0, 9), (0, 4), (0, 247), (0, 2353), (3, 132), (1, 2483), (5, 781), (0, 520), (0, 154), (2, 206), (0, 121), (7, 1527), (0, 81), (1, 202), (0, 2424), (1, 902), (0, 23), (0, 920), (0, 3), (3, 2495), (2, 2038), (0, 3), (4, 910)];

        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 562) as usize];
        let item_idx = u32::from(d2)
            .wrapping_add(f1.wrapping_mul(u32::from(d1)))
            .wrapping_add(f2) as usize
            % 2810;
        if EnumItem::STRING_TABLE[item_idx].as_bytes() != input {
            return Err(ParseEnumItemError);
        }
        Ok(unsafe { std::mem::transmute::<u16, Self>(item_idx as u16) })
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
