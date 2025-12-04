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
    _500Miles                                                              = 330,
    /// 1
    _1                                                                     = 829,
    /// 1-0
    _1_0                                                                   = 2408,
    /// 1-001
    _1_001                                                                 = 1152,
    /// 1-1-001
    _1_1_001                                                               = 2735,
    /// 1-8
    _1_8                                                                   = 281,
    /// 10
    _10                                                                    = 1806,
    /// 100
    _100                                                                   = 1471,
    /// 10000BASE-T1
    _10000baseT1                                                           = 819,
    /// 1000BASE-T
    _1000baseT                                                             = 899,
    /// 1000BASE-T1
    _1000baseT1                                                            = 1559,
    /// 100BASE-T1
    _100baseT1                                                             = 609,
    /// 100BASE-TX
    _100baseTx                                                             = 1288,
    /// 10BASE-T1S
    _10baseT1s                                                             = 637,
    /// 12
    _12                                                                    = 2051,
    /// 120
    _120                                                                   = 2296,
    /// 15
    _15                                                                    = 1352,
    /// 150
    _150                                                                   = 236,
    /// 16
    _16                                                                    = 2074,
    /// 16-KHZ
    _16Khz                                                                 = 2228,
    /// 176-4-KHZ
    _176_4Khz                                                              = 271,
    /// 192-KHZ
    _192Khz                                                                = 1585,
    /// 2
    _2                                                                     = 2618,
    /// 20
    _20                                                                    = 408,
    /// 200
    _200                                                                   = 353,
    /// 24
    _24                                                                    = 1765,
    /// 24-25
    _24_25                                                                 = 2571,
    /// 24-KHZ
    _24Khz                                                                 = 2575,
    /// 240
    _240                                                                   = 2712,
    /// 25
    _25                                                                    = 1256,
    /// 25-24
    _25_24                                                                 = 2418,
    /// 2500BASE-T1
    _2500baseT1                                                            = 2436,
    /// 30
    _30                                                                    = 1393,
    /// 300
    _300                                                                   = 303,
    /// 32-KHZ
    _32Khz                                                                 = 891,
    /// 4-1-1
    _4_1_1                                                                 = 2400,
    /// 4-2-0
    _4_2_0                                                                 = 267,
    /// 4-2-2
    _4_2_2                                                                 = 1768,
    /// 4-2-2-4
    _4_2_2_4                                                               = 1908,
    /// 4-4-4
    _4_4_4                                                                 = 2221,
    /// 4-4-4-4
    _4_4_4_4                                                               = 250,
    /// 44-1-KHZ
    _44_1Khz                                                               = 226,
    /// 48
    _48                                                                    = 1680,
    /// 48-KHZ
    _48Khz                                                                 = 32,
    /// 5
    _5                                                                     = 2569,
    /// 50
    _50                                                                    = 631,
    /// 5000BASE-T1
    _5000baseT1                                                            = 1926,
    /// 60
    _60                                                                    = 1892,
    /// 72
    _72                                                                    = 2691,
    /// 8
    _8                                                                     = 837,
    /// 8-KHZ
    _8Khz                                                                  = 730,
    /// 85
    _85                                                                    = 2696,
    /// 88-2-KHZ
    _88_2Khz                                                               = 73,
    /// 96-KHZ
    _96Khz                                                                 = 807,
    /// AA
    Aa                                                                     = 2448,
    /// AB
    Ab                                                                     = 2800,
    /// ABSTRACT
    Abstract                                                               = 806,
    /// ABSTRACT-ACCESS-POINT
    AbstractAccessPoint                                                    = 2172,
    /// ABSTRACT-CAN-CLUSTER
    AbstractCanCluster                                                     = 664,
    /// ABSTRACT-CAN-COMMUNICATION-CONNECTOR
    AbstractCanCommunicationConnector                                      = 739,
    /// ABSTRACT-CAN-COMMUNICATION-CONTROLLER
    AbstractCanCommunicationController                                     = 2119,
    /// ABSTRACT-CAN-PHYSICAL-CHANNEL
    AbstractCanPhysicalChannel                                             = 2070,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 857,
    /// ABSTRACT-CRYPTO-KEY-SLOT-INTERFACE
    AbstractCryptoKeySlotInterface                                         = 2732,
    /// ABSTRACT-CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    AbstractCryptoKeySlotToPortPrototypeMapping                            = 2198,
    /// ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS
    AbstractDoIpLogicAddressProps                                          = 777,
    /// ABSTRACT-DO-IP-PORT-MAPPING
    AbstractDoIpPortMapping                                                = 221,
    /// ABSTRACT-ETHERNET-FRAME
    AbstractEthernetFrame                                                  = 2398,
    /// ABSTRACT-EVENT
    AbstractEvent                                                          = 2733,
    /// ABSTRACT-EXECUTION-CONTEXT
    AbstractExecutionContext                                               = 2143,
    /// ABSTRACT-FUNCTIONAL-CLUSTER-DESIGN
    AbstractFunctionalClusterDesign                                        = 1614,
    /// ABSTRACT-IAM-REMOTE-SUBJECT
    AbstractIamRemoteSubject                                               = 1262,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE
    AbstractImplementationDataType                                         = 2352,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT
    AbstractImplementationDataTypeElement                                  = 2047,
    /// ABSTRACT-PROVIDED-PORT-PROTOTYPE
    AbstractProvidedPortPrototype                                          = 1113,
    /// ABSTRACT-RAW-DATA-STREAM-INTERFACE
    AbstractRawDataStreamInterface                                         = 2805,
    /// ABSTRACT-REQUIRED-PORT-PROTOTYPE
    AbstractRequiredPortPrototype                                          = 2087,
    /// ABSTRACT-SECURITY-EVENT-FILTER
    AbstractSecurityEventFilter                                            = 2681,
    /// ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER
    AbstractSecurityIdsmInstanceFilter                                     = 994,
    /// ABSTRACT-SERVICE-INSTANCE
    AbstractServiceInstance                                                = 99,
    /// ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING
    AbstractSignalBasedToISignalTriggeringMapping                          = 2090,
    /// ABSTRACT-SUSPEND-TO-RAM-INTERFACE
    AbstractSuspendToRamInterface                                          = 304,
    /// ABSTRACT-SUSPEND-TO-RAM-MAPPING
    AbstractSuspendToRamMapping                                            = 249,
    /// ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE
    AbstractSynchronizedTimeBaseInterface                                  = 2555,
    /// ACCEPT-ALL
    AcceptAll                                                              = 1488,
    /// ACCEPT-CONFIGURED
    AcceptConfigured                                                       = 492,
    /// ACCES-PERRMISSION-SERVICE-CLASS
    AccesPerrmissionServiceClass                                           = 2257,
    /// ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS
    AccessPermissionInstanceOverridesClass                                 = 2014,
    /// ACCESS-PERMISSION-SERVICE-CLASS
    AccessPermissionServiceClass                                           = 922,
    /// ACCESS-PERMISSION-SERVICE-INSTANCE
    AccessPermissionServiceInstance                                        = 1359,
    /// ACK-WITH-RT
    AckWithRt                                                              = 1411,
    /// ACK-WITHOUT-RT
    AckWithoutRt                                                           = 367,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 629,
    /// ACL-OPERATION
    AclOperation                                                           = 2595,
    /// ACL-PERMISSION
    AclPermission                                                          = 2888,
    /// ACL-ROLE
    AclRole                                                                = 1349,
    /// ACTION
    Action                                                                 = 1434,
    /// ACTION-ITEM
    ActionItem                                                             = 2189,
    /// ACTION-LIST
    ActionList                                                             = 1748,
    /// ACTIVATE
    Activate                                                               = 2100,
    /// ACTIVATION-AND-TRIGGER-UNICAST
    ActivationAndTriggerUnicast                                            = 1232,
    /// ACTIVATION-MULTICAST
    ActivationMulticast                                                    = 1293,
    /// ACTIVATION-UNICAST
    ActivationUnicast                                                      = 1462,
    /// ACTIVE
    Active                                                                 = 643,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 1229,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 1098,
    /// ADAPTIVE-EVENT-RECEIVED
    AdaptiveEventReceived                                                  = 678,
    /// ADAPTIVE-EVENT-SENT
    AdaptiveEventSent                                                      = 2522,
    /// ADAPTIVE-FIELD-GETTER-CALLED
    AdaptiveFieldGetterCalled                                              = 295,
    /// ADAPTIVE-FIELD-GETTER-COMPLETED
    AdaptiveFieldGetterCompleted                                           = 1826,
    /// ADAPTIVE-FIELD-NOTIFICATION-RECEIVED
    AdaptiveFieldNotificationReceived                                      = 1083,
    /// ADAPTIVE-FIELD-NOTIFICATION-SENT
    AdaptiveFieldNotificationSent                                          = 1612,
    /// ADAPTIVE-FIELD-SETTER-CALLED
    AdaptiveFieldSetterCalled                                              = 1738,
    /// ADAPTIVE-FIELD-SETTER-COMPLETED
    AdaptiveFieldSetterCompleted                                           = 803,
    /// ADAPTIVE-FIREWALL-MODULE-INSTANTIATION
    AdaptiveFirewallModuleInstantiation                                    = 1507,
    /// ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING
    AdaptiveFirewallToPortPrototypeMapping                                 = 2471,
    /// ADAPTIVE-METHOD-CALL-RECEIVED
    AdaptiveMethodCallReceived                                             = 913,
    /// ADAPTIVE-METHOD-CALLED
    AdaptiveMethodCalled                                                   = 2217,
    /// ADAPTIVE-METHOD-RESPONSE-RECEIVED
    AdaptiveMethodResponseReceived                                         = 1021,
    /// ADAPTIVE-METHOD-RESPONSE-SENT
    AdaptiveMethodResponseSent                                             = 391,
    /// ADAPTIVE-MODULE-INSTANTIATION
    AdaptiveModuleInstantiation                                            = 2873,
    /// ADAPTIVE-PLATFORM-SERVICE-INSTANCE
    AdaptivePlatformServiceInstance                                        = 432,
    /// ADAPTIVE-SERVICE-FIND-COMPLETED
    AdaptiveServiceFindCompleted                                           = 1791,
    /// ADAPTIVE-SERVICE-FIND-STARTED
    AdaptiveServiceFindStarted                                             = 2422,
    /// ADAPTIVE-SERVICE-OFFER-COMPLETED
    AdaptiveServiceOfferCompleted                                          = 1704,
    /// ADAPTIVE-SERVICE-OFFER-STARTED
    AdaptiveServiceOfferStarted                                            = 1175,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED
    AdaptiveServiceStopSubscriptionCompleted                               = 763,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED
    AdaptiveServiceStopSubscriptionStarted                                 = 1999,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED
    AdaptiveServiceSubscriptionAcknowledgeCompleted                        = 1751,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED
    AdaptiveServiceSubscriptionAcknowledgeStarted                          = 684,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED
    AdaptiveServiceSubscriptionCompleted                                   = 102,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED
    AdaptiveServiceSubscriptionStarted                                     = 225,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 2707,
    /// ADDR-METHOD-SHORT-NAME
    AddrMethodShortName                                                    = 2425,
    /// ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT
    AddrMethodShortNameAndAlignment                                        = 2298,
    /// AES-3-32-BIT
    Aes3_32Bit                                                             = 2206,
    /// AF
    Af                                                                     = 657,
    /// AFTER-SALES
    AfterSales                                                             = 177,
    /// AFTERMAKET
    Aftermaket                                                             = 1465,
    /// AFTERMARKET
    Aftermarket                                                            = 2801,
    /// AGE
    Age                                                                    = 1733,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 764,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 273,
    /// AGREED
    Agreed                                                                 = 1665,
    /// AH
    Ah                                                                     = 1086,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 862,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 1012,
    /// ALL
    All                                                                    = 2167,
    /// ALL-16-BIT
    All16Bit                                                               = 1676,
    /// ALL-INDICES-DIFFERENT-ARRAY-SIZE
    AllIndicesDifferentArraySize                                           = 59,
    /// ALL-INDICES-SAME-ARRAY-SIZE
    AllIndicesSameArraySize                                                = 2880,
    /// ALL-PARTIAL-NETWORKS-ACTIVE
    AllPartialNetworksActive                                               = 229,
    /// ALL-SUPPORTED-DTCS
    AllSupportedDtcs                                                       = 224,
    /// ALLOCATOR
    Allocator                                                              = 2338,
    /// ALLOW
    Allow                                                                  = 2209,
    /// ALTERNATING-8-BIT
    Alternating8Bit                                                        = 1793,
    /// ALWAYS
    Always                                                                 = 494,
    /// AM
    Am                                                                     = 1755,
    /// AMBER-WARNING
    AmberWarning                                                           = 1717,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 2044,
    /// AND
    And                                                                    = 2077,
    /// ANY
    Any                                                                    = 1581,
    /// ANY-PARTIAL-NETWORK-ACTIVE
    AnyPartialNetworkActive                                                = 1510,
    /// ANY-SEND-OPERATION
    AnySendOperation                                                       = 1271,
    /// ANY-STANDARDIZED
    AnyStandardized                                                        = 319,
    /// AP
    Ap                                                                     = 2568,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 390,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 844,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 1724,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 2279,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 1399,
    /// API
    Api                                                                    = 2736,
    /// API-BASED
    ApiBased                                                               = 2745,
    /// API-USE
    ApiUse                                                                 = 2289,
    /// APMC-ABSTRACT-DEFINITION
    ApmcAbstractDefinition                                                 = 1977,
    /// APMC-ABSTRACT-FOREIGN-REFERENCE-DEF
    ApmcAbstractForeignReferenceDef                                        = 2795,
    /// APMC-ABSTRACT-INSTANCE-REFERENCE-DEF
    ApmcAbstractInstanceReferenceDef                                       = 2808,
    /// APMC-ABSTRACT-INSTANCE-REFERENCE-VALUE
    ApmcAbstractInstanceReferenceValue                                     = 809,
    /// APMC-ABSTRACT-REFERENCE-VALUE
    ApmcAbstractReferenceValue                                             = 1592,
    /// APMC-ABSTRACT-RESTRICTED-STRING-PARAM-DEF
    ApmcAbstractRestrictedStringParamDef                                   = 234,
    /// APMC-ABSTRACT-RESTRICTED-TEXTUAL-PARAM-VALUE
    ApmcAbstractRestrictedTextualParamValue                                = 213,
    /// APMC-BOOLEAN-PARAM-DEF
    ApmcBooleanParamDef                                                    = 802,
    /// APMC-CHOICE-CONTAINER-DEF
    ApmcChoiceContainerDef                                                 = 2650,
    /// APMC-CHOICE-CONTAINER-REFERENCE-DEF
    ApmcChoiceContainerReferenceDef                                        = 1616,
    /// APMC-CONFIGURATION-ELEMENT-DEF
    ApmcConfigurationElementDef                                            = 915,
    /// APMC-CONTAINER-DEF
    ApmcContainerDef                                                       = 1179,
    /// APMC-CONTAINER-ELEMENT-DEF
    ApmcContainerElementDef                                                = 2783,
    /// APMC-CONTAINER-ELEMENT-VALUE
    ApmcContainerElementValue                                              = 666,
    /// APMC-CONTAINER-REFERENCE-DEF
    ApmcContainerReferenceDef                                              = 816,
    /// APMC-CONTAINER-REFERENCE-VALUE
    ApmcContainerReferenceValue                                            = 1381,
    /// APMC-CONTAINER-VALUE
    ApmcContainerValue                                                     = 1243,
    /// APMC-DEFINITION-COLLECTION
    ApmcDefinitionCollection                                               = 2314,
    /// APMC-ENUMERATION-LITERAL-DEF
    ApmcEnumerationLiteralDef                                              = 283,
    /// APMC-ENUMERATION-PARAM-DEF
    ApmcEnumerationParamDef                                                = 2016,
    /// APMC-FLOAT-PARAM-DEF
    ApmcFloatParamDef                                                      = 43,
    /// APMC-FOREIGN-REFERENCE-DEF
    ApmcForeignReferenceDef                                                = 423,
    /// APMC-FOREIGN-REFERENCE-VALUE
    ApmcForeignReferenceValue                                              = 1939,
    /// APMC-FUNCTIONAL-CLUSTER-DEF
    ApmcFunctionalClusterDef                                               = 2627,
    /// APMC-FUNCTIONAL-CLUSTER-VALUE
    ApmcFunctionalClusterValue                                             = 1451,
    /// APMC-INSTANCE-REFERENCE-DEF
    ApmcInstanceReferenceDef                                               = 2850,
    /// APMC-INSTANCE-REFERENCE-VALUE
    ApmcInstanceReferenceValue                                             = 2318,
    /// APMC-INTEGER-PARAM-DEF
    ApmcIntegerParamDef                                                    = 2488,
    /// APMC-IP-V4-ADDRESS-PARAM-DEF
    ApmcIpV4AddressParamDef                                                = 995,
    /// APMC-IP-V4-ADDRESS-PARAM-VALUE
    ApmcIpV4AddressParamValue                                              = 111,
    /// APMC-IP-V6-ADDRESS-PARAM-DEF
    ApmcIpV6AddressParamDef                                                = 2133,
    /// APMC-IP-V6-ADDRESS-PARAM-VALUE
    ApmcIpV6AddressParamValue                                              = 1369,
    /// APMC-MAC-ADDRESS-PARAM-DEF
    ApmcMacAddressParamDef                                                 = 2458,
    /// APMC-MAC-ADDRESS-PARAM-VALUE
    ApmcMacAddressParamValue                                               = 1736,
    /// APMC-NUMERICAL-PARAM-VALUE
    ApmcNumericalParamValue                                                = 1602,
    /// APMC-PARAM-CONF-CONTAINER-DEF
    ApmcParamConfContainerDef                                              = 1364,
    /// APMC-PARAMETER-DEF
    ApmcParameterDef                                                       = 2902,
    /// APMC-PARAMETER-VALUE
    ApmcParameterValue                                                     = 2138,
    /// APMC-REFERENCE-DEF
    ApmcReferenceDef                                                       = 1178,
    /// APMC-REFERENCE-VALUE
    ApmcReferenceValue                                                     = 1070,
    /// APMC-REVISION-LABEL-PARAM-DEF
    ApmcRevisionLabelParamDef                                              = 1719,
    /// APMC-REVISION-LABEL-PARAM-VALUE
    ApmcRevisionLabelParamValue                                            = 731,
    /// APMC-STRING-PARAM-DEF
    ApmcStringParamDef                                                     = 355,
    /// APMC-STRONG-REVISION-LABEL-PARAM-DEF
    ApmcStrongRevisionLabelParamDef                                        = 813,
    /// APMC-STRONG-REVISION-LABEL-PARAM-VALUE
    ApmcStrongRevisionLabelParamValue                                      = 1267,
    /// APMC-TEXTUAL-PARAM-VALUE
    ApmcTextualParamValue                                                  = 1737,
    /// APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-DEF
    ApmcUpstreamDocForeignReferenceDef                                     = 2756,
    /// APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-VALUE
    ApmcUpstreamDocForeignReferenceValue                                   = 2078,
    /// APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-DEF
    ApmcUpstreamDocInstanceReferenceDef                                    = 1611,
    /// APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-VALUE
    ApmcUpstreamDocInstanceReferenceValue                                  = 1010,
    /// APMC-URI-FOREIGN-REFERENCE-DEF
    ApmcUriForeignReferenceDef                                             = 756,
    /// APMC-URI-FOREIGN-REFERENCE-VALUE
    ApmcUriForeignReferenceValue                                           = 1771,
    /// APMC-URI-INSTANCE-REFERENCE-DEF
    ApmcUriInstanceReferenceDef                                            = 1476,
    /// APMC-URI-INSTANCE-REFERENCE-VALUE
    ApmcUriInstanceReferenceValue                                          = 178,
    /// APMC-URI-PARAM-DEF
    ApmcUriParamDef                                                        = 559,
    /// APMC-URI-PARAM-VALUE
    ApmcUriParamValue                                                      = 2130,
    /// APMC-VALUE-COLLECTION
    ApmcValueCollection                                                    = 1078,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 146,
    /// APPLICABILITY-INFO-SET
    ApplicabilityInfoSet                                                   = 1033,
    /// APPLICATION
    Application                                                            = 2139,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 299,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 2833,
    /// APPLICATION-ARRAY-ELEMENT
    ApplicationArrayElement                                                = 2452,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 2874,
    /// APPLICATION-ASSOC-MAP-ELEMENT
    ApplicationAssocMapElement                                             = 897,
    /// APPLICATION-COMPOSITE-DATA-TYPE
    ApplicationCompositeDataType                                           = 392,
    /// APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE
    ApplicationCompositeElementDataPrototype                               = 272,
    /// APPLICATION-DATA-TYPE
    ApplicationDataType                                                    = 241,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 1865,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 1862,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 2659,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 1572,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 1962,
    /// APPLICATION-ONLY
    ApplicationOnly                                                        = 78,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 2796,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 1803,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 1587,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 498,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 828,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 2246,
    /// APPLIED-STANDARD
    AppliedStandard                                                        = 2127,
    /// AR
    Ar                                                                     = 2794,
    /// AR--CLIENT--SERVER
    ArClientServer                                                         = 38,
    /// AR-ELEMENT
    ArElement                                                              = 371,
    /// AR-PACKAGE
    ArPackage                                                              = 1342,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 2591,
    /// ARBITRATION
    Arbitration                                                            = 2472,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 2798,
    /// ARRAY
    Array                                                                  = 1050,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 1774,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 2883,
    /// ARTIFACT-LOCATOR
    ArtifactLocator                                                        = 540,
    /// AS
    As                                                                     = 1813,
    /// AS-IS
    AsIs                                                                   = 1486,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 592,
    /// ASYMMETRIC-FROM-BYTE-ARRAY
    AsymmetricFromByteArray                                                = 1525,
    /// ASYMMETRIC-TO-BYTE-ARRAY
    AsymmetricToByteArray                                                  = 2395,
    /// ASYNCHRONOUS
    Asynchronous                                                           = 2414,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 126,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 84,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 493,
    /// ATOMIC-SW-COMPONENT-TYPE
    AtomicSwComponentType                                                  = 255,
    /// ATP-BLUEPRINT
    AtpBlueprint                                                           = 1155,
    /// ATP-BLUEPRINTABLE
    AtpBlueprintable                                                       = 238,
    /// ATP-CLASSIFIER
    AtpClassifier                                                          = 1876,
    /// ATP-DEFINITION
    AtpDefinition                                                          = 1691,
    /// ATP-FEATURE
    AtpFeature                                                             = 1668,
    /// ATP-PROTOTYPE
    AtpPrototype                                                           = 2041,
    /// ATP-STRUCTURE-ELEMENT
    AtpStructureElement                                                    = 2558,
    /// ATP-TYPE
    AtpType                                                                = 1833,
    /// ATTRIBUTE-TAILORING
    AttributeTailoring                                                     = 2776,
    /// AUDIO-SAMPLE
    AudioSample                                                            = 2649,
    /// AUTHENTICATE
    Authenticate                                                           = 2866,
    /// AUTO
    Auto                                                                   = 1145,
    /// AUTO-IP
    AutoIp                                                                 = 1827,
    /// AUTO-IP--DOIP
    AutoIpDoip                                                             = 1550,
    /// AUTO-IPDHCPV-4
    AutoIpdhcpv4                                                           = 1135,
    /// AUTOMATIC
    Automatic                                                              = 1767,
    /// AUTONOMOUS
    Autonomous                                                             = 2526,
    /// AUTOSAR-DATA-PROTOTYPE
    AutosarDataPrototype                                                   = 1983,
    /// AUTOSAR-DATA-TYPE
    AutosarDataType                                                        = 1188,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 1898,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 2000,
    /// AVB--IEEE-802--1-AS
    AvbIeee802_1As                                                         = 845,
    /// AY
    Ay                                                                     = 1685,
    /// AZ
    Az                                                                     = 1814,
    /// BA
    Ba                                                                     = 2199,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 2208,
    /// BAM
    Bam                                                                    = 1044,
    /// BAMCMDT
    Bamcmdt                                                                = 2470,
    /// BASE-T
    BaseT                                                                  = 1530,
    /// BASE-TYPE
    BaseType                                                               = 1031,
    /// BASIC-SOFTWARE-MODE-MANAGER
    BasicSoftwareModeManager                                               = 2584,
    /// BAYER-BGGR
    BayerBggr                                                              = 686,
    /// BAYER-GBRG
    BayerGbrg                                                              = 953,
    /// BAYER-GRBG
    BayerGrbg                                                              = 2128,
    /// BAYER-RGGB
    BayerRggb                                                              = 1110,
    /// BE
    Be                                                                     = 1963,
    /// BEST-EFFORT
    BestEffort                                                             = 701,
    /// BG
    Bg                                                                     = 2251,
    /// BH
    Bh                                                                     = 2841,
    /// BI
    Bi                                                                     = 2242,
    /// BIDIRECTIONAL
    Bidirectional                                                          = 2236,
    /// BINARY-MANIFEST-ADDRESSABLE-OBJECT
    BinaryManifestAddressableObject                                        = 26,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 1291,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 1966,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 1713,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 588,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 1761,
    /// BINARY-MANIFEST-RESOURCE
    BinaryManifestResource                                                 = 1120,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 2664,
    /// BLINK-MODE
    BlinkMode                                                              = 2248,
    /// BLINK-OR-CONTINUOUS-ON-MODE
    BlinkOrContinuousOnMode                                                = 2173,
    /// BLOCK
    Block                                                                  = 1095,
    /// BLOCK-SOURCE
    BlockSource                                                            = 569,
    /// BLOCK-STATE
    BlockState                                                             = 2468,
    /// BLUEPRINT-DERIVATION-TIME
    BlueprintDerivationTime                                                = 1644,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 501,
    /// BMP
    Bmp                                                                    = 1034,
    /// BN
    Bn                                                                     = 2048,
    /// BO
    Bo                                                                     = 757,
    /// BOLD
    Bold                                                                   = 2334,
    /// BOLDITALIC
    Bolditalic                                                             = 1759,
    /// BONJOUR
    Bonjour                                                                = 2310,
    /// BOOLEAN
    Boolean                                                                = 1895,
    /// BOTTOM
    Bottom                                                                 = 2233,
    /// BR
    Br                                                                     = 1635,
    /// BREAK
    Break                                                                  = 1573,
    /// BRIEF
    Brief                                                                  = 529,
    /// BRIEF-BYPASSING-FILTERS
    BriefBypassingFilters                                                  = 992,
    /// BROAD-R-REACH
    BroadRReach                                                            = 919,
    /// BSW
    Bsw                                                                    = 2610,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 676,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 832,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 1136,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 1426,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 135,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 799,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 2429,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 2766,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 321,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 1922,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 2900,
    /// BSW-EVENT
    BswEvent                                                               = 98,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 2293,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 1020,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 1591,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 1907,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 2196,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 1642,
    /// BSW-INTERRUPT-EVENT
    BswInterruptEvent                                                      = 2290,
    /// BSW-M-ENTRY-CALL-RETURNED
    BswMEntryCallReturned                                                  = 63,
    /// BSW-M-ENTRY-CALLED
    BswMEntryCalled                                                        = 2295,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 2721,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 1169,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 699,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 923,
    /// BSW-MODULE-CALL-POINT
    BswModuleCallPoint                                                     = 989,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 1493,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 2495,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 7,
    /// BSW-MODULE-ENTITY
    BswModuleEntity                                                        = 1198,
    /// BSW-MODULE-ENTITY-ACTIVATED
    BswModuleEntityActivated                                               = 2453,
    /// BSW-MODULE-ENTITY-STARTED
    BswModuleEntityStarted                                                 = 2320,
    /// BSW-MODULE-ENTITY-TERMINATED
    BswModuleEntityTerminated                                              = 2430,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 218,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 943,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 2748,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 2322,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 698,
    /// BSW-SCHEDULE-EVENT
    BswScheduleEvent                                                       = 1580,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 1989,
    /// BSW-SERVICE-DEPENDENCY-IDENT
    BswServiceDependencyIdent                                              = 1341,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 20,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 1168,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 2592,
    /// BT-REC-601
    BtRec601                                                               = 1111,
    /// BT-REC-709
    BtRec709                                                               = 1979,
    /// BUILD
    Build                                                                  = 1245,
    /// BUILD-ACTION
    BuildAction                                                            = 2413,
    /// BUILD-ACTION-ENTITY
    BuildActionEntity                                                      = 1992,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 2613,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 1240,
    /// BUILD-TYPE-DEBUG
    BuildTypeDebug                                                         = 2369,
    /// BUILD-TYPE-RELEASE
    BuildTypeRelease                                                       = 563,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 762,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 511,
    /// BUS-MIRROR-CHANNEL-MAPPING
    BusMirrorChannelMapping                                                = 2746,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 957,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 1720,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 2409,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 1313,
    /// BY-RECEPTION-TIMESTAMP
    ByReceptionTimestamp                                                   = 1645,
    /// BY-SOURCE-TIMESTAMP
    BySourceTimestamp                                                      = 491,
    /// C
    C                                                                      = 437,
    /// CA
    Ca                                                                     = 1637,
    /// CALCULATED
    Calculated                                                             = 1662,
    /// CALIBRATION-OFFLINE
    CalibrationOffline                                                     = 11,
    /// CALIBRATION-ONLINE
    CalibrationOnline                                                      = 2619,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 2806,
    /// CALIBRATION-VARIABLES
    CalibrationVariables                                                   = 237,
    /// CALLBACK
    Callback                                                               = 659,
    /// CALLOUT
    Callout                                                                = 906,
    /// CALPRM
    Calprm                                                                 = 2171,
    /// CAN
    Can                                                                    = 1920,
    /// CAN-20
    Can20                                                                  = 2252,
    /// CAN-BE-REMOVED
    CanBeRemoved                                                           = 2815,
    /// CAN-BE-TERMINATED
    CanBeTerminated                                                        = 1953,
    /// CAN-BE-TERMINATED-AND-RESTARTED
    CanBeTerminatedAndRestarted                                            = 2853,
    /// CAN-BRIEF
    CanBrief                                                               = 1875,
    /// CAN-CLUSTER
    CanCluster                                                             = 61,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 1264,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 2761,
    /// CAN-FD
    CanFd                                                                  = 17,
    /// CAN-FRAME
    CanFrame                                                               = 869,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 2702,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 1947,
    /// CAN-NM-NODE
    CanNmNode                                                              = 1899,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 1688,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 2531,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 1723,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 1363,
    /// CAN-TP-NODE
    CanTpNode                                                              = 69,
    /// CAN-XL-PROPS
    CanXlProps                                                             = 2142,
    /// CANCEL
    Cancel                                                                 = 1318,
    /// CANCEL-CAMPAIGN
    CancelCampaign                                                         = 1043,
    /// CANNOT-BE-REMOVED
    CannotBeRemoved                                                        = 846,
    /// CAPTION
    Caption                                                                = 106,
    /// CAPTURE-ASYNCHRONOUS-TO-REPORTING
    CaptureAsynchronousToReporting                                         = 2521,
    /// CAPTURE-ASYNCHRONOUSLY-TO-REPORTING
    CaptureAsynchronouslyToReporting                                       = 758,
    /// CAPTURE-SYNCHRONOUS-TO-REPORTING
    CaptureSynchronousToReporting                                          = 1925,
    /// CAPTURE-SYNCHRONOUSLY-TO-REPORTING
    CaptureSynchronouslyToReporting                                        = 827,
    /// CAT-1
    Cat1                                                                   = 1692,
    /// CAT-2
    Cat2                                                                   = 2484,
    /// CAUTION
    Caution                                                                = 1056,
    /// CENTER
    Center                                                                 = 2911,
    /// CHANNEL-A
    ChannelA                                                               = 1314,
    /// CHANNEL-B
    ChannelB                                                               = 752,
    /// CHAPTER
    Chapter                                                                = 96,
    /// CHARGE-MANAGER-NEEDS
    ChargeManagerNeeds                                                     = 2177,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 1344,
    /// CIRCLE
    Circle                                                                 = 879,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 516,
    /// CLASSIC
    Classic                                                                = 1163,
    /// CLEAR
    Clear                                                                  = 1431,
    /// CLEAR-ALL-DTCS
    ClearAllDtcs                                                           = 338,
    /// CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    ClearDynamicallyDefineDataIdentifier                                   = 6,
    /// CLIENT-AUTHENTICATE
    ClientAuthenticate                                                     = 1022,
    /// CLIENT-DECRYPT
    ClientDecrypt                                                          = 2502,
    /// CLIENT-ENCRYPT
    ClientEncrypt                                                          = 115,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 70,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 2832,
    /// CLIENT-MAC-GENERATE
    ClientMacGenerate                                                      = 2151,
    /// CLIENT-MAC-VERIFY
    ClientMacVerify                                                        = 1129,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 840,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 2710,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 196,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 1538,
    /// CLIENT-VERIFY
    ClientVerify                                                           = 1630,
    /// CLOSED
    Closed                                                                 = 1859,
    /// CM-MODULE-INSTANTIATION
    CmModuleInstantiation                                                  = 655,
    /// CMDT
    Cmdt                                                                   = 2155,
    /// CO
    Co                                                                     = 1490,
    /// CODE
    Code                                                                   = 2114,
    /// CODE-GENERATION-TIME
    CodeGenerationTime                                                     = 2405,
    /// CODEGENERATION
    Codegeneration                                                         = 2668,
    /// COLDSTART
    Coldstart                                                              = 1804,
    /// COLLECTABLE-ELEMENT
    CollectableElement                                                     = 467,
    /// COLLECTION
    Collection                                                             = 2021,
    /// COLOR-AWARE
    ColorAware                                                             = 765,
    /// COLOR-BLIND
    ColorBlind                                                             = 134,
    /// COM-AXIS
    ComAxis                                                                = 455,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 2094,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 2830,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 2195,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 614,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 1345,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 941,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 2361,
    /// COM-GRANT
    ComGrant                                                               = 2120,
    /// COM-GRANT-DESIGN
    ComGrantDesign                                                         = 1389,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 1406,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 1087,
    /// COM-MANAGER
    ComManager                                                             = 2434,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 2692,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 1222,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 2737,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 1526,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 336,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 2912,
    /// COM-TRIGGER-GRANT
    ComTriggerGrant                                                        = 743,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 2889,
    /// COMM-CONNECTOR-PORT
    CommConnectorPort                                                      = 833,
    /// COMMAND-LINE-LONG-FORM
    CommandLineLongForm                                                    = 105,
    /// COMMAND-LINE-SHORT-FORM
    CommandLineShortForm                                                   = 2447,
    /// COMMAND-LINE-SIMPLE-FORM
    CommandLineSimpleForm                                                  = 13,
    /// COMMON
    Common                                                                 = 2518,
    /// COMMUNICATION-CLUSTER
    CommunicationCluster                                                   = 1799,
    /// COMMUNICATION-CONNECTOR
    CommunicationConnector                                                 = 2781,
    /// COMMUNICATION-CONTROLLER
    CommunicationController                                                = 1014,
    /// COMMUNICATION-INTER-ECU
    CommunicationInterEcu                                                  = 201,
    /// COMMUNICATION-INTRA-PARTITION
    CommunicationIntraPartition                                            = 2717,
    /// COMPILE
    Compile                                                                = 830,
    /// COMPILER
    Compiler                                                               = 2457,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 789,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 1749,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 1972,
    /// COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING
    CompositionPortToExecutablePortMapping                                 = 2032,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 1560,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 696,
    /// COMPU-METHOD
    CompuMethod                                                            = 623,
    /// COM_AXIS
    Comaxis                                                                = 1322,
    /// CONCRETE
    Concrete                                                               = 2837,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 2278,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 2494,
    /// CONDITIONAL
    Conditional                                                            = 1681,
    /// CONFIDENTIALITY-OFFSET--0
    ConfidentialityOffset0                                                 = 965,
    /// CONFIDENTIALITY-OFFSET--30
    ConfidentialityOffset30                                                = 2688,
    /// CONFIDENTIALITY-OFFSET--50
    ConfidentialityOffset50                                                = 525,
    /// CONFIG-DATA
    ConfigData                                                             = 2020,
    /// CONFIGURED
    Configured                                                             = 1873,
    /// CONFIRMED
    Confirmed                                                              = 1453,
    /// CONFIRMED-DTC-BIT
    ConfirmedDtcBit                                                        = 1561,
    /// CONNECT
    Connect                                                                = 2250,
    /// CONSISTENCY-MECHANISM-REQUIRED
    ConsistencyMechanismRequired                                           = 240,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 634,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 2312,
    /// CONSOLE
    Console                                                                = 2219,
    /// CONST
    Const                                                                  = 961,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 116,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 2512,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 2499,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 2525,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 605,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 2141,
    /// CONSUMER
    Consumer                                                               = 412,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 2680,
    /// CONTINUE-AT-IT-POSITION
    ContinueAtItPosition                                                   = 1249,
    /// CONTINUOUS-ON-MODE
    ContinuousOnMode                                                       = 1856,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 382,
    /// COUPLING-ELEMENT-ABSTRACT-DETAILS
    CouplingElementAbstractDetails                                         = 2003,
    /// COUPLING-ELEMENT-SWITCH-DETAILS
    CouplingElementSwitchDetails                                           = 2269,
    /// COUPLING-PORT
    CouplingPort                                                           = 847,
    /// COUPLING-PORT-ABSTRACT-SHAPER
    CouplingPortAbstractShaper                                             = 2268,
    /// COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER
    CouplingPortAsynchronousTrafficShaper                                  = 801,
    /// COUPLING-PORT-CREDIT-BASED-SHAPER
    CouplingPortCreditBasedShaper                                          = 725,
    /// COUPLING-PORT-ENHANCED-TRAFFIC-SHAPER
    CouplingPortEnhancedTrafficShaper                                      = 572,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 305,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 1407,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 141,
    /// COUPLING-PORT-STRUCTURAL-ELEMENT
    CouplingPortStructuralElement                                          = 1041,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 2317,
    /// CP
    Cp                                                                     = 1927,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 1797,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 1147,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 1715,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 1601,
    /// CP-SOFTWARE-CLUSTER-RESOURCE
    CpSoftwareClusterResource                                              = 1578,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 1551,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 1073,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 2554,
    /// CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterToApplicationPartitionMapping                         = 1775,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 2642,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 1834,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 1067,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 2530,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 1373,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 450,
    /// CPP
    Cpp                                                                    = 1154,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 1760,
    /// CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET
    CppImplementationDataTypeContextTarget                                 = 2156,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 1549,
    /// CRC-IGNORED
    CrcIgnored                                                             = 2868,
    /// CRC-NOT-SUPPORTED
    CrcNotSupported                                                        = 2035,
    /// CRC-NOT-VALIDATED
    CrcNotValidated                                                        = 2357,
    /// CRC-OPTIONAL
    CrcOptional                                                            = 2008,
    /// CRC-SUPPORTED
    CrcSupported                                                           = 101,
    /// CRC-VALIDATED
    CrcValidated                                                           = 2694,
    /// CREATED-BY-INTEGRATOR
    CreatedByIntegrator                                                    = 1483,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 2280,
    /// CRYPTO-CERTIFICATE-GROUP
    CryptoCertificateGroup                                                 = 987,
    /// CRYPTO-CERTIFICATE-GROUP-INTERFACE
    CryptoCertificateGroupInterface                                        = 2353,
    /// CRYPTO-CERTIFICATE-GROUP-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateGroupToPortPrototypeMapping                           = 1148,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 1698,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 2442,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 79,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 405,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 618,
    /// CRYPTO-INTERFACE
    CryptoInterface                                                        = 538,
    /// CRYPTO-JOB
    CryptoJob                                                              = 1678,
    /// CRYPTO-KEY-MANAGEMENT
    CryptoKeyManagement                                                    = 284,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 2624,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 947,
    /// CRYPTO-KEY-SLOT-CLIENT-INTERFACE
    CryptoKeySlotClientInterface                                           = 790,
    /// CRYPTO-KEY-SLOT-DESIGN
    CryptoKeySlotDesign                                                    = 839,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 746,
    /// CRYPTO-KEY-SLOT-TO-CLIENT-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToClientPortPrototypeMapping                              = 363,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 1672,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN
    CryptoKeySlotUsageDesign                                               = 1646,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN-MAPPING
    CryptoKeySlotUsageDesignMapping                                        = 143,
    /// CRYPTO-KEY-SLOT-USER-DESIGN
    CryptoKeySlotUserDesign                                                = 2639,
    /// CRYPTO-KEY-SLOT-USER-DESIGN-MAPPING
    CryptoKeySlotUserDesignMapping                                         = 1063,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 1285,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 2176,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 1946,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 1481,
    /// CRYPTO-NEEDS
    CryptoNeeds                                                            = 92,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 1358,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 1338,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 1385,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 42,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 1475,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 564,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 990,
    /// CRYPTO-SERVICE-MANAGER
    CryptoServiceManager                                                   = 2045,
    /// CRYPTO-SERVICE-MAPPING
    CryptoServiceMapping                                                   = 2615,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 1693,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 379,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 153,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 334,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 1784,
    /// CS
    Cs                                                                     = 2282,
    /// CSERS
    Csers                                                                  = 2891,
    /// CURVE-AXIS
    CurveAxis                                                              = 2626,
    /// CURVE_AXIS
    Curveaxis                                                              = 535,
    /// CUSTOM
    Custom                                                                 = 1828,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 937,
    /// CVC
    Cvc                                                                    = 2633,
    /// CY
    Cy                                                                     = 610,
    /// CYCLE-REPETITION-1
    CycleRepetition1                                                       = 2870,
    /// CYCLE-REPETITION-10
    CycleRepetition10                                                      = 1841,
    /// CYCLE-REPETITION-16
    CycleRepetition16                                                      = 1130,
    /// CYCLE-REPETITION-2
    CycleRepetition2                                                       = 2580,
    /// CYCLE-REPETITION-20
    CycleRepetition20                                                      = 370,
    /// CYCLE-REPETITION-32
    CycleRepetition32                                                      = 2292,
    /// CYCLE-REPETITION-4
    CycleRepetition4                                                       = 288,
    /// CYCLE-REPETITION-40
    CycleRepetition40                                                      = 387,
    /// CYCLE-REPETITION-5
    CycleRepetition5                                                       = 411,
    /// CYCLE-REPETITION-50
    CycleRepetition50                                                      = 120,
    /// CYCLE-REPETITION-64
    CycleRepetition64                                                      = 917,
    /// CYCLE-REPETITION-8
    CycleRepetition8                                                       = 1284,
    /// CYCLIC
    Cyclic                                                                 = 2153,
    /// CYCLIC-AND-ON-CHANGE
    CyclicAndOnChange                                                      = 1625,
    /// CYCLIC-HANDLING-COM-DATA-TO-OS-TASK-PROXY-MAPPING
    CyclicHandlingComDataToOsTaskProxyMapping                              = 420,
    /// DA
    Da                                                                     = 1620,
    /// DATA-CONSTR
    DataConstr                                                             = 553,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 2216,
    /// DATA-FORMAT-ELEMENT-REFERENCE
    DataFormatElementReference                                             = 776,
    /// DATA-FORMAT-ELEMENT-SCOPE
    DataFormatElementScope                                                 = 2609,
    /// DATA-INTERFACE
    DataInterface                                                          = 2109,
    /// DATA-PROTOTYPE
    DataPrototype                                                          = 2709,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 1236,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPS-IDENT
    DataPrototypeTransformationPropsIdent                                  = 1446,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 212,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 2410,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 2507,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 117,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 831,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 1062,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 1390,
    /// DCM-I-PDU
    DcmIPdu                                                                = 2169,
    /// DDS-ABSTRACT-SERVICE-INSTANCE-ELEMENT-CP
    DdsAbstractServiceInstanceElementCp                                    = 576,
    /// DDS-CP-CONFIG
    DdsCpConfig                                                            = 808,
    /// DDS-CP-CONSUMED-SERVICE-INSTANCE
    DdsCpConsumedServiceInstance                                           = 2682,
    /// DDS-CP-DOMAIN
    DdsCpDomain                                                            = 820,
    /// DDS-CP-PARTITION
    DdsCpPartition                                                         = 1316,
    /// DDS-CP-PROVIDED-SERVICE-INSTANCE
    DdsCpProvidedServiceInstance                                           = 1412,
    /// DDS-CP-QOS-PROFILE
    DdsCpQosProfile                                                        = 1866,
    /// DDS-CP-SERVICE-INSTANCE
    DdsCpServiceInstance                                                   = 2015,
    /// DDS-CP-TOPIC
    DdsCpTopic                                                             = 2570,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 557,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 1988,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 1009,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 848,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 270,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 2356,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 2695,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 1068,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 1935,
    /// DDS-SERVICE
    DdsService                                                             = 2274,
    /// DDS-SERVICE-INSTANCE-EVENT-CP
    DdsServiceInstanceEventCp                                              = 2541,
    /// DDS-SERVICE-INSTANCE-FIELD-CP
    DdsServiceInstanceFieldCp                                              = 795,
    /// DDS-SERVICE-INSTANCE-OPERATION-CP
    DdsServiceInstanceOperationCp                                          = 473,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 129,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 2552,
    /// DDS-SIGNAL
    DdsSignal                                                              = 1124,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 2533,
    /// DE
    De                                                                     = 2496,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 2492,
    /// DEBOUNCE-DATA
    DebounceData                                                           = 416,
    /// DEBUG
    Debug                                                                  = 356,
    /// DECREASING
    Decreasing                                                             = 2789,
    /// DEDICATED
    Dedicated                                                              = 2096,
    /// DEF-ITEM
    DefItem                                                                = 1090,
    /// DEFAULT
    Default                                                                = 1967,
    /// DEFAULT-ERROR-TRACER
    DefaultErrorTracer                                                     = 2259,
    /// DEFAULT-IF-REVISION-UPDATE
    DefaultIfRevisionUpdate                                                = 2224,
    /// DEFAULT-IF-UNDEFINED
    DefaultIfUndefined                                                     = 1872,
    /// DEFAULT-MODE
    DefaultMode                                                            = 586,
    /// DEFAULT-TRACE-STATE-DISABLED
    DefaultTraceStateDisabled                                              = 248,
    /// DEFAULT-TRACE-STATE-ENABLED
    DefaultTraceStateEnabled                                               = 188,
    /// DEFAULT-TRIGGER
    DefaultTrigger                                                         = 2193,
    /// DEFERRED
    Deferred                                                               = 2073,
    /// DEFICIT-ROUND-ROBIN
    DeficitRoundRobin                                                      = 2720,
    /// DEFINE-BY-IDENTIFIER
    DefineByIdentifier                                                     = 2234,
    /// DEFINE-BY-MEMORY-ADDRESS
    DefineByMemoryAddress                                                  = 2842,
    /// DEFLATE
    Deflate                                                                = 2586,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 2545,
    /// DELETE
    Delete                                                                 = 724,
    /// DEPENDANT
    Dependant                                                              = 1900,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 459,
    /// DERIVED-FROM
    DerivedFrom                                                            = 705,
    /// DERIVED-FROM-DESIGN
    DerivedFromDesign                                                      = 1541,
    /// DESCENDANT
    Descendant                                                             = 246,
    /// DESELECTED
    Deselected                                                             = 1211,
    /// DETAILED
    Detailed                                                               = 929,
    /// DETAILED-BYPASSING-FILTERS
    DetailedBypassingFilters                                               = 2309,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 1339,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 1773,
    /// DETERMINISTIC-SYNC-INSTANTIATION
    DeterministicSyncInstantiation                                         = 772,
    /// DETERMINISTIC-SYNC-MASTER
    DeterministicSyncMaster                                                = 900,
    /// DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING
    DeterministicSyncMasterToTimeBaseConsumerMapping                       = 1912,
    /// DEVELOPMENT
    Development                                                            = 2500,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 1366,
    /// DEVELOPMENT-ERROR-TRACER
    DevelopmentErrorTracer                                                 = 863,
    /// DHCPV-4
    Dhcpv4                                                                 = 149,
    /// DHCPV-6
    Dhcpv6                                                                 = 2898,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 2174,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 519,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 822,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 1536,
    /// DIAG-REQUEST
    DiagRequest                                                            = 1292,
    /// DIAG-RESPONSE
    DiagResponse                                                           = 2273,
    /// DIAGNOSTIC-ABSTRACT-ALIAS-EVENT
    DiagnosticAbstractAliasEvent                                           = 1180,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER
    DiagnosticAbstractDataIdentifier                                       = 2122,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE
    DiagnosticAbstractDataIdentifierInterface                              = 261,
    /// DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE
    DiagnosticAbstractRoutineInterface                                     = 1776,
    /// DIAGNOSTIC-ABSTRACT-SOVD-CONTENT
    DiagnosticAbstractSovdContent                                          = 1005,
    /// DIAGNOSTIC-ABSTRACT-SOVD-CONTENT-INTERFACE
    DiagnosticAbstractSovdContentInterface                                 = 1943,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 675,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 1798,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 369,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE
    DiagnosticAuthTransmitCertificate                                      = 258,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION
    DiagnosticAuthTransmitCertificateEvaluation                            = 2037,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING
    DiagnosticAuthTransmitCertificateMapping                               = 853,
    /// DIAGNOSTIC-AUTHENTICATION
    DiagnosticAuthentication                                               = 2716,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 741,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 2149,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 2731,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 1658,
    /// DIAGNOSTIC-CAPABILITY-ELEMENT
    DiagnosticCapabilityElement                                            = 2461,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 415,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 2240,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 1533,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 500,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 2890,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 771,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 1076,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 81,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 991,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 1177,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 1552,
    /// DIAGNOSTIC-COMMON-ELEMENT
    DiagnosticCommonElement                                                = 926,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER
    DiagnosticCommunicationManager                                         = 2308,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 787,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 866,
    /// DIAGNOSTIC-CONDITION
    DiagnosticCondition                                                    = 812,
    /// DIAGNOSTIC-CONDITION-GROUP
    DiagnosticConditionGroup                                               = 2426,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 1185,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 713,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 1757,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 2635,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 2350,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 1583,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 264,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 1367,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 1885,
    /// DIAGNOSTIC-DATA-BY-IDENTIFIER
    DiagnosticDataByIdentifier                                             = 183,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 2582,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 90,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 2340,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 738,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 1880,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 80,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 1186,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 2265,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 2065,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 1884,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 2589,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 976,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 170,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-PORT-MAPPING
    DiagnosticDoIpActivationLinePortMapping                                = 1118,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE
    DiagnosticDoIpEntityIdentificationInterface                            = 1173,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-PORT-MAPPING
    DiagnosticDoIpEntityIdentificationPortMapping                          = 147,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 1987,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-PORT-MAPPING
    DiagnosticDoIpGroupIdentificationPortMapping                           = 1276,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 1231,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-PORT-MAPPING
    DiagnosticDoIpPowerModePortMapping                                     = 245,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 2594,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-PORT-MAPPING
    DiagnosticDoIpTriggerVehicleAnnouncementPortMapping                    = 1817,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 2884,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 733,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 2226,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 394,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 160,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 1498,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 2201,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 2165,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 1075,
    /// DIAGNOSTIC-EDR-DATA-PROVIDER-MAPPING
    DiagnosticEdrDataProviderMapping                                       = 2123,
    /// DIAGNOSTIC-EDR-SENDER-PORT-MAPPING
    DiagnosticEdrSenderPortMapping                                         = 951,
    /// DIAGNOSTIC-EDR-SERVER-PORT-MAPPING
    DiagnosticEdrServerPortMapping                                         = 1840,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 1543,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 1377,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 2299,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 156,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 549,
    /// DIAGNOSTIC-ENV-MODE-ELEMENT
    DiagnosticEnvModeElement                                               = 2504,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 1449,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 769,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 998,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 2793,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 1447,
    /// DIAGNOSTIC-EVENT-MANAGER
    DiagnosticEventManager                                                 = 2360,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 1801,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 1015,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 1626,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 2158,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 176,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 329,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 190,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 2301,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 2108,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 2371,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 364,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-CLIENT-PORT-MAPPING
    DiagnosticExtendedDataRecordClientPortMapping                          = 232,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-ELEMENT
    DiagnosticExtendedDataRecordElement                                    = 463,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-INTERFACE
    DiagnosticExtendedDataRecordInterface                                  = 207,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-NEEDS
    DiagnosticExtendedDataRecordNeeds                                      = 1663,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 1569,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 1376,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 166,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 2715,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 1906,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 2162,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 1174,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 959,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 2214,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 652,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 1194,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 920,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 1940,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 2719,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 1119,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 1297,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 2267,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 1633,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 2159,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 323,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 341,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 2403,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 515,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 2573,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 2734,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 206,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 2767,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 2264,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 199,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 2703,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 1116,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 2107,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 2465,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 1712,
    /// DIAGNOSTIC-LOG-AND-TRACE
    DiagnosticLogAndTrace                                                  = 2287,
    /// DIAGNOSTIC-MAPPING
    DiagnosticMapping                                                      = 1448,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 970,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 1131,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 1675,
    /// DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS
    DiagnosticMemoryAddressableRangeAccess                                 = 1016,
    /// DIAGNOSTIC-MEMORY-BY-ADDRESS
    DiagnosticMemoryByAddress                                              = 1636,
    /// DIAGNOSTIC-MEMORY-DESTINATION
    DiagnosticMemoryDestination                                            = 2210,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 2203,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 2212,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 940,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 2178,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 1754,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 2443,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 2625,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE
    DiagnosticMultipleConditionInterface                                   = 1405,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING
    DiagnosticMultipleConditionPortMapping                                 = 1019,
    /// DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE
    DiagnosticMultipleEventInterface                                       = 133,
    /// DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING
    DiagnosticMultipleEventPortMapping                                     = 597,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE
    DiagnosticMultipleMonitorInterface                                     = 2515,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING
    DiagnosticMultipleMonitorPortMapping                                   = 1524,
    /// DIAGNOSTIC-MULTIPLE-RESOURCE-INTERFACE
    DiagnosticMultipleResourceInterface                                    = 817,
    /// DIAGNOSTIC-MULTIPLE-RESOURCE-PORT-MAPPING
    DiagnosticMultipleResourcePortMapping                                  = 2256,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 2775,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 484,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 1714,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 2407,
    /// DIAGNOSTIC-PARAMETER-ELEMENT
    DiagnosticParameterElement                                             = 841,
    /// DIAGNOSTIC-PARAMETER-IDENT
    DiagnosticParameterIdent                                               = 613,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 883,
    /// DIAGNOSTIC-PORT-INTERFACE
    DiagnosticPortInterface                                                = 2170,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 193,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 216,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 2485,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 2535,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 118,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 1252,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 41,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 892,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 889,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 2316,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 2697,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 2597,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 1460,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 1223,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 2102,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 2646,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 796,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 1811,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 1007,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 1080,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 269,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 861,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 279,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 2147,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 2042,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 1980,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE
    DiagnosticRequestFileTransferInterface                                 = 2581,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 1729,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 1444,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 1579,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 362,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 1534,
    /// DIAGNOSTIC-REQUEST-ROUTINE-RESULTS
    DiagnosticRequestRoutineResults                                        = 1361,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 2397,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 2255,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 1401,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 2598,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 1461,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 2068,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 1383,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 2446,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 275,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 550,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 2846,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 997,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 514,
    /// DIAGNOSTIC-ROUTINE-SUBFUNCTION
    DiagnosticRoutineSubfunction                                           = 2404,
    /// DIAGNOSTIC-SECURE-CODING-MAPPING
    DiagnosticSecureCodingMapping                                          = 1442,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 328,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 152,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 1904,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 2699,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 967,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 1332,
    /// DIAGNOSTIC-SERVICE-CLASS
    DiagnosticServiceClass                                                 = 151,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 438,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 150,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 2191,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 340,
    /// DIAGNOSTIC-SERVICE-INSTANCE
    DiagnosticServiceInstance                                              = 660,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 2333,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 103,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 1948,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 479,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 171,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 574,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 2105,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 2764,
    /// DIAGNOSTIC-SOVD-ARRAY-CONTENT-ELEMENT
    DiagnosticSovdArrayContentElement                                      = 2449,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE
    DiagnosticSovdAuthorizationInterface                                   = 2183,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING
    DiagnosticSovdAuthorizationPortMapping                                 = 2019,
    /// DIAGNOSTIC-SOVD-BULK-DATA
    DiagnosticSovdBulkData                                                 = 114,
    /// DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE
    DiagnosticSovdBulkDataInterface                                        = 403,
    /// DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING
    DiagnosticSovdBulkDataPortMapping                                      = 1193,
    /// DIAGNOSTIC-SOVD-COMPOSITE-CONTENT-ELEMENT
    DiagnosticSovdCompositeContentElement                                  = 1469,
    /// DIAGNOSTIC-SOVD-CONFIG-CONTENT-MAPPING
    DiagnosticSovdConfigContentMapping                                     = 1072,
    /// DIAGNOSTIC-SOVD-CONFIGURATION
    DiagnosticSovdConfiguration                                            = 2166,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA
    DiagnosticSovdConfigurationBulkData                                    = 633,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING
    DiagnosticSovdConfigurationDataIdentifierMapping                       = 1721,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE
    DiagnosticSovdConfigurationInterface                                   = 1747,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER
    DiagnosticSovdConfigurationParameter                                   = 1890,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING
    DiagnosticSovdConfigurationPortMapping                                 = 276,
    /// DIAGNOSTIC-SOVD-CONTENT
    DiagnosticSovdContent                                                  = 1917,
    /// DIAGNOSTIC-SOVD-CONTENT-ELEMENT
    DiagnosticSovdContentElement                                           = 265,
    /// DIAGNOSTIC-SOVD-CONTENT-ELEMENT-INTERFACE
    DiagnosticSovdContentElementInterface                                  = 1181,
    /// DIAGNOSTIC-SOVD-CONTENT-GROUP
    DiagnosticSovdContentGroup                                             = 2475,
    /// DIAGNOSTIC-SOVD-CONTENT-INTERFACE
    DiagnosticSovdContentInterface                                         = 804,
    /// DIAGNOSTIC-SOVD-CONTENT-PORT-MAPPING
    DiagnosticSovdContentPortMapping                                       = 58,
    /// DIAGNOSTIC-SOVD-DATA
    DiagnosticSovdData                                                     = 714,
    /// DIAGNOSTIC-SOVD-DATA-CATEGORY
    DiagnosticSovdDataCategory                                             = 217,
    /// DIAGNOSTIC-SOVD-FAULT-MEMORY-ACCESS
    DiagnosticSovdFaultMemoryAccess                                        = 783,
    /// DIAGNOSTIC-SOVD-GROUP
    DiagnosticSovdGroup                                                    = 131,
    /// DIAGNOSTIC-SOVD-LOCK
    DiagnosticSovdLock                                                     = 1350,
    /// DIAGNOSTIC-SOVD-LOG
    DiagnosticSovdLog                                                      = 1171,
    /// DIAGNOSTIC-SOVD-METHOD
    DiagnosticSovdMethod                                                   = 683,
    /// DIAGNOSTIC-SOVD-METHOD-PRIMITIVE
    DiagnosticSovdMethodPrimitive                                          = 2679,
    /// DIAGNOSTIC-SOVD-OPERATION
    DiagnosticSovdOperation                                                = 2534,
    /// DIAGNOSTIC-SOVD-OPERATION-INTERFACE
    DiagnosticSovdOperationInterface                                       = 2887,
    /// DIAGNOSTIC-SOVD-OPERATION-PORT-MAPPING
    DiagnosticSovdOperationPortMapping                                     = 2822,
    /// DIAGNOSTIC-SOVD-PORT-INTERFACE
    DiagnosticSovdPortInterface                                            = 1807,
    /// DIAGNOSTIC-SOVD-PRIMITIVE-CONTENT-ELEMENT
    DiagnosticSovdPrimitiveContentElement                                  = 1228,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE
    DiagnosticSovdProximityChallengeInterface                              = 2427,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING
    DiagnosticSovdProximityChallengePortMapping                            = 1039,
    /// DIAGNOSTIC-SOVD-RECORD-CONTENT-ELEMENT
    DiagnosticSovdRecordContentElement                                     = 759,
    /// DIAGNOSTIC-SOVD-SERVICE-INSTANCE
    DiagnosticSovdServiceInstance                                          = 37,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE
    DiagnosticSovdServiceValidationInterface                               = 1001,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING
    DiagnosticSovdServiceValidationPortMapping                             = 1514,
    /// DIAGNOSTIC-SOVD-UPDATE
    DiagnosticSovdUpdate                                                   = 2510,
    /// DIAGNOSTIC-SOVD-UPDATE-INTERFACE
    DiagnosticSovdUpdateInterface                                          = 870,
    /// DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING
    DiagnosticSovdUpdatePortMapping                                        = 1049,
    /// DIAGNOSTIC-START-ROUTINE
    DiagnosticStartRoutine                                                 = 638,
    /// DIAGNOSTIC-STOP-ROUTINE
    DiagnosticStopRoutine                                                  = 326,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 773,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 1142,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 1575,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 1931,
    /// DIAGNOSTIC-SW-MAPPING
    DiagnosticSwMapping                                                    = 2416,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 2254,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 2825,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 2904,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 352,
    /// DIAGNOSTIC-TRANSMIT-CERTIFICATE-INTERFACE
    DiagnosticTransmitCertificateInterface                                 = 1143,
    /// DIAGNOSTIC-TROUBLE-CODE
    DiagnosticTroubleCode                                                  = 74,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 317,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 436,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 1474,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 474,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 2686,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 2345,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 1153,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 881,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 1107,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 2481,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 2249,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 228,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 401,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 9,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 2085,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 1013,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 1604,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 2245,
    /// DISABLE
    Disable                                                                = 457,
    /// DLNA
    Dlna                                                                   = 2306,
    /// DLT-APPLICATION
    DltApplication                                                         = 404,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 509,
    /// DLT-ARGUMENT
    DltArgument                                                            = 1356,
    /// DLT-ARGUMENT-PROPS
    DltArgumentProps                                                       = 1220,
    /// DLT-ARGUMENT-PROPS-SET
    DltArgumentPropsSet                                                    = 2652,
    /// DLT-CONTEXT
    DltContext                                                             = 306,
    /// DLT-ECU
    DltEcu                                                                 = 1564,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 2665,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 682,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 2844,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 2375,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 2419,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 1945,
    /// DLT-MESSAGE
    DltMessage                                                             = 277,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 2771,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 1125,
    /// DO-IP
    DoIp                                                                   = 702,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 419,
    /// DO-IP-FUNCTIONAL-CLUSTER-DESIGN
    DoIpFunctionalClusterDesign                                            = 2202,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 775,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 1253,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 497,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 1299,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 1955,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 2505,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 1909,
    /// DO-IP-LOGICAL-ADDRESS
    DoIpLogicalAddress                                                     = 108,
    /// DO-IP-NETWORK-CONFIGURATION-DESIGN
    DoIpNetworkConfigurationDesign                                         = 1336,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 975,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 2286,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 1371,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 2608,
    /// DO-IP-SERVICE-NEEDS
    DoIpServiceNeeds                                                       = 1582,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 1632,
    /// DO-NOT-INCLUDE
    DoNotInclude                                                           = 1994,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 1499,
    /// DOCUMENTATION
    Documentation                                                          = 1117,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 2034,
    /// DOES-NOT-REPORT-EXECUTION-STATE
    DoesNotReportExecutionState                                            = 503,
    /// DOES-NOT-SUPPORT-BUFFER-LOCKING
    DoesNotSupportBufferLocking                                            = 1829,
    /// DOES-NOT-USE-LOGGING
    DoesNotUseLogging                                                      = 1852,
    /// DOMAIN-PARTICIPANT-USER-DATA-QOS
    DomainParticipantUserDataQos                                           = 2343,
    /// DONT-INVALIDATE
    DontInvalidate                                                         = 2757,
    /// DROP
    Drop                                                                   = 2437,
    /// DROP-FRAME
    DropFrame                                                              = 1957,
    /// DROP-UNTAGGED
    DropUntagged                                                           = 1351,
    /// DSA
    Dsa                                                                    = 2307,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 2376,
    /// DYNAMIC
    Dynamic                                                                = 1301,
    /// DYNAMIC-PART-TRIGGER
    DynamicPartTrigger                                                     = 647,
    /// DZ
    Dz                                                                     = 86,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 1450,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 395,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 1984,
    /// ECC
    Ecc                                                                    = 2009,
    /// ECU
    Ecu                                                                    = 466,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 235,
    /// ECU-INSTANCE
    EcuInstance                                                            = 963,
    /// ECU-MANAGER
    EcuManager                                                             = 2838,
    /// ECU-MAPPING
    EcuMapping                                                             = 524,
    /// ECU-PARTITION
    EcuPartition                                                           = 75,
    /// ECU-PARTITION-TO-CORE-MAPPING
    EcuPartitionToCoreMapping                                              = 1065,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 703,
    /// ECU-TIMING
    EcuTiming                                                              = 64,
    /// ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF
    EcucAbstractExternalReferenceDef                                       = 417,
    /// ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF
    EcucAbstractInternalReferenceDef                                       = 2774,
    /// ECUC-ABSTRACT-REFERENCE-DEF
    EcucAbstractReferenceDef                                               = 204,
    /// ECUC-ABSTRACT-STRING-PARAM-DEF
    EcucAbstractStringParamDef                                             = 1096,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 180,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 2132,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 625,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 1378,
    /// ECUC-COMMON-ATTRIBUTES
    EcucCommonAttributes                                                   = 257,
    /// ECUC-CONTAINER-DEF
    EcucContainerDef                                                       = 626,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 621,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 1974,
    /// ECUC-DEFINITION-ELEMENT
    EcucDefinitionElement                                                  = 2160,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 1998,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 909,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 2540,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 2420,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 1529,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 2058,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 1505,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 704,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 1458,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 1853,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 1923,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 2423,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 2785,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 2335,
    /// ECUC-PARAMETER-DEF
    EcucParameterDef                                                       = 1879,
    /// ECUC-QUERY
    EcucQuery                                                              = 2514,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 2705,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 2666,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 2836,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 1570,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 2125,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 2319,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 2225,
    /// EDGE-NODE
    EdgeNode                                                               = 1183,
    /// EID-USE-API
    EidUseApi                                                              = 2834,
    /// EID-USE-CONFIG-VALUE
    EidUseConfigValue                                                      = 1421,
    /// EID-USE-MAC
    EidUseMac                                                              = 896,
    /// EL
    El                                                                     = 600,
    /// EMISSION-RELATED-DTC
    EmissionRelatedDtc                                                     = 2835,
    /// EN
    En                                                                     = 1137,
    /// ENABLE
    Enable                                                                 = 1330,
    /// ENABLED
    Enabled                                                                = 40,
    /// ENCRYPT-AND-SIGN
    EncryptAndSign                                                         = 1764,
    /// ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION
    EncryptAndSignWithOriginAuthentication                                 = 718,
    /// ENCRYPTION
    Encryption                                                             = 1159,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 159,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 399,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 2053,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 885,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 468,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 1077,
    /// ENHANCED
    Enhanced                                                               = 490,
    /// ENHANCED-TRAFFIC-SHAPER
    EnhancedTrafficShaper                                                  = 2440,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 2370,
    /// EO
    Eo                                                                     = 1279,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 635,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 1207,
    /// EOC-EXECUTABLE-ENTITY-REF-ABSTRACT
    EocExecutableEntityRefAbstract                                         = 2131,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 12,
    /// EPS
    Eps                                                                    = 2851,
    /// EQUAL
    Equal                                                                  = 88,
    /// ERROR
    Error                                                                  = 1258,
    /// ERROR-CORRECTION
    ErrorCorrection                                                        = 1413,
    /// ERROR-DETECTION
    ErrorDetection                                                         = 2116,
    /// ERROR-TRACER
    ErrorTracer                                                            = 981,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 2637,
    /// ES
    Es                                                                     = 2374,
    /// ESP
    Esp                                                                    = 312,
    /// ET
    Et                                                                     = 2431,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 1428,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 1423,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 1750,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 526,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 1479,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 930,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 2799,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 174,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 1466,
    /// ETHERNET-MAC-RAW-DATA-STREAM-MAPPING
    EthernetMacRawDataStreamMapping                                        = 1586,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 168,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 223,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 1969,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 335,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 624,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 1308,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 2607,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 2549,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 2230,
    /// ETP
    Etp                                                                    = 721,
    /// EU
    Eu                                                                     = 2285,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 1502,
    /// EVAP
    Evap                                                                   = 734,
    /// EVAPPURGEFLOW
    Evappurgeflow                                                          = 1657,
    /// EVENT-ACCEPTANCE-DISABLED
    EventAcceptanceDisabled                                                = 2532,
    /// EVENT-ACCEPTANCE-ENABLED
    EventAcceptanceEnabled                                                 = 2711,
    /// EVENT-COMBINATION-ON-RETRIEVAL
    EventCombinationOnRetrieval                                            = 1432,
    /// EVENT-COMBINATION-ON-STORAGE
    EventCombinationOnStorage                                              = 688,
    /// EVENT-HANDLER
    EventHandler                                                           = 1554,
    /// EVENT-MAPPING
    EventMapping                                                           = 1060,
    /// EVENT-STORAGE-DISABLED
    EventStorageDisabled                                                   = 1978,
    /// EVENT-STORAGE-ENABLED
    EventStorageEnabled                                                    = 1132,
    /// EVENT-TRIGGERING-CONSTRAINT
    EventTriggeringConstraint                                              = 163,
    /// EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE
    EventWindowCurrentAndFollowingCycle                                    = 880,
    /// EVENT-WINDOW-CURRENT-CYCLE
    EventWindowCurrentCycle                                                = 782,
    /// EVENT-WINDOW-INFINITE
    EventWindowInfinite                                                    = 988,
    /// EXACT-OR-ANY-MINOR-VERSION
    ExactOrAnyMinorVersion                                                 = 22,
    /// EXAMPLE
    Example                                                                = 1742,
    /// EXCLUDE-FROM-FLASH
    ExcludeFromFlash                                                       = 2075,
    /// EXCLUSIVE
    Exclusive                                                              = 1860,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 1556,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 875,
    /// EXECUTABLE
    Executable                                                             = 1869,
    /// EXECUTABLE-ENTITY
    ExecutableEntity                                                       = 854,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 2455,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 632,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 2445,
    /// EXECUTE
    Execute                                                                = 2529,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 2207,
    /// EXECUTION-TIME
    ExecutionTime                                                          = 2813,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 884,
    /// EXERCISE
    Exercise                                                               = 2238,
    /// EXPLICIT
    Explicit                                                               = 139,
    /// EXPRESS
    Express                                                                = 2444,
    /// EXTEND
    Extend                                                                 = 1051,
    /// EXTENDED
    Extended                                                               = 902,
    /// EXTERNAL-REPLACEMENT
    ExternalReplacement                                                    = 46,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 2080,
    /// EXTERNAL-TRIGGERING-POINT-IDENT
    ExternalTriggeringPointIdent                                           = 309,
    /// FA
    Fa                                                                     = 458,
    /// FAILURE-AND-SUCCESS
    FailureAndSuccess                                                      = 627,
    /// FAILURE-ONLY
    FailureOnly                                                            = 706,
    /// FALSE
    False                                                                  = 1088,
    /// FAST-FLASHING-MODE
    FastFlashingMode                                                       = 1064,
    /// FATAL
    Fatal                                                                  = 1055,
    /// FAULT
    Fault                                                                  = 640,
    /// FDBAM
    Fdbam                                                                  = 2076,
    /// FDBAMCMDT
    Fdbamcmdt                                                              = 242,
    /// FDC-THRESHOLD
    FdcThreshold                                                           = 2028,
    /// FDCMDT
    Fdcmdt                                                                 = 1623,
    /// FI
    Fi                                                                     = 1435,
    /// FIBEX-ELEMENT
    FibexElement                                                           = 2487,
    /// FIELD
    Field                                                                  = 2491,
    /// FIELD-MAPPING
    FieldMapping                                                           = 253,
    /// FILE
    File                                                                   = 2574,
    /// FILTERED
    Filtered                                                               = 1520,
    /// FINISH
    Finish                                                                 = 2380,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 442,
    /// FIRE-AND-FORGET-METHOD-MAPPING
    FireAndForgetMethodMapping                                             = 1888,
    /// FIREWALL-RULE
    FirewallRule                                                           = 1224,
    /// FIREWALL-STATE-SWITCH-INTERFACE
    FirewallStateSwitchInterface                                           = 536,
    /// FIRST-CONTAINED-TRIGGER
    FirstContainedTrigger                                                  = 1437,
    /// FIRST-TO-SECOND
    FirstToSecond                                                          = 1165,
    /// FIT-TO-PAGE
    FitToPage                                                              = 2567,
    /// FIT-TO-TEXT
    FitToText                                                              = 1851,
    /// FIX-AXIS
    FixAxis                                                                = 274,
    /// FIXED
    Fixed                                                                  = 19,
    /// FIXED-SIZE
    FixedSize                                                              = 661,
    /// FIX_AXIS
    Fixaxis                                                                = 968,
    /// FJ
    Fj                                                                     = 2760,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 1886,
    /// FLAT-MAP
    FlatMap                                                                = 2227,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 2154,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 1404,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 2727,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 1903,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 1190,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 1463,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 2135,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 674,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 347,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 1104,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 2523,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 278,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 1035,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 2557,
    /// FLOAT
    Float                                                                  = 2560,
    /// FLOAT-32
    Float32                                                                = 641,
    /// FLOAT-32-BIT
    Float32Bit                                                             = 2323,
    /// FLOAT-64
    Float64                                                                = 1219,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 215,
    /// FM-FEATURE
    FmFeature                                                              = 2022,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 1522,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 2564,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 2818,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 54,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 1202,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 429,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 938,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 1430,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 2150,
    /// FO
    Fo                                                                     = 2493,
    /// FOR-ALL
    ForAll                                                                 = 1850,
    /// FORGET
    Forget                                                                 = 1789,
    /// FORWARD-AS-IS
    ForwardAsIs                                                            = 2829,
    /// FPP
    Fpp                                                                    = 690,
    /// FR
    Fr                                                                     = 532,
    /// FRAME
    Frame                                                                  = 1290,
    /// FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION
    FrameEthernetQueuedForTransmission                                     = 2896,
    /// FRAME-ETHERNET-RECEIVED-BY-IF
    FrameEthernetReceivedByIf                                              = 2605,
    /// FRAME-ETHERNET-RECEIVED-ON-BUS
    FrameEthernetReceivedOnBus                                             = 1694,
    /// FRAME-ETHERNET-SENT-ON-BUS
    FrameEthernetSentOnBus                                                 = 1226,
    /// FRAME-PORT
    FramePort                                                              = 71,
    /// FRAME-QUEUED-FOR-TRANSMISSION
    FrameQueuedForTransmission                                             = 377,
    /// FRAME-RECEIVED-BY-IF
    FrameReceivedByIf                                                      = 921,
    /// FRAME-TRANSMITTED-ON-BUS
    FrameTransmittedOnBus                                                  = 2315,
    /// FRAME-TRIGGERING
    FrameTriggering                                                        = 1669,
    /// FULL
    Full                                                                   = 2630,
    /// FULL-COM
    FullCom                                                                = 1555,
    /// FULL-DUPLEX-MODE
    FullDuplexMode                                                         = 1796,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 2895,
    /// FUNCTION-GROUP-PORT-MAPPING
    FunctionGroupPortMapping                                               = 2394,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 1400,
    /// FUNCTION-GROUP-STATE-TO-NM-HANDLE
    FunctionGroupStateToNmHandle                                           = 154,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 2537,
    /// FUNCTION-INHIBITION-MANAGER
    FunctionInhibitionManager                                              = 1150,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 1259,
    /// FUNCTIONAL
    Functional                                                             = 1374,
    /// FUNCTIONAL-ADDRESS
    FunctionalAddress                                                      = 2547,
    /// FUNCTIONAL-CAN-FD
    FunctionalCanFd                                                        = 646,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-DIAGNOSTIC-EVENT-MAPPING
    FunctionalClusterInteractsWithDiagnosticEventMapping                   = 476,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING
    FunctionalClusterInteractsWithFunctionalClusterMapping                 = 2590,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING
    FunctionalClusterInteractsWithPersistencyDeploymentMapping             = 1993,
    /// FUNCTIONAL-CLUSTER-TO-DLT-LOG-SINK-MAPPING
    FunctionalClusterToDltLogSinkMapping                                   = 2110,
    /// FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING
    FunctionalClusterToSecurityEventDefinitionMapping                      = 1511,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 1942,
    /// FY
    Fy                                                                     = 1772,
    /// GA
    Ga                                                                     = 949,
    /// GATEWAY
    Gateway                                                                = 1002,
    /// GD
    Gd                                                                     = 1257,
    /// GENERAL-PARAMETER
    GeneralParameter                                                       = 1959,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 203,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 1402,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 658,
    /// GENERAL-PURPOSE-TIMER-SERVICE-NEEDS
    GeneralPurposeTimerServiceNeeds                                        = 785,
    /// GENERIC-DIAGNOSTIC-TRANSPORT-INSTANTIATION
    GenericDiagnosticTransportInstantiation                                = 144,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 1274,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 1362,
    /// GENERIC-TP-CONNECTION
    GenericTpConnection                                                    = 1915,
    /// GET
    Get                                                                    = 179,
    /// GETTER
    Getter                                                                 = 656,
    /// GETTER-SETTER
    GetterSetter                                                           = 2006,
    /// GIF
    Gif                                                                    = 460,
    /// GL
    Gl                                                                     = 2675,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 2326,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 1416,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 2200,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 710,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 933,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 935,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 636,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 1250,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 148,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 2378,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 10,
    /// GLOBAL-TIME-MASTER
    GlobalTimeMaster                                                       = 1842,
    /// GLOBAL-TIME-SLAVE
    GlobalTimeSlave                                                        = 1085,
    /// GN
    Gn                                                                     = 2089,
    /// GRANT
    Grant                                                                  = 1495,
    /// GRANT-DESIGN
    GrantDesign                                                            = 119,
    /// GRAYSCALE
    Grayscale                                                              = 219,
    /// GROSS
    Gross                                                                  = 2390,
    /// GU
    Gu                                                                     = 1375,
    /// GZIP
    Gzip                                                                   = 545,
    /// HA
    Ha                                                                     = 2728,
    /// HALF-DUPLEX-MODE
    HalfDuplexMode                                                         = 1235,
    /// HARDWARE-TEST-MANAGER
    HardwareTestManager                                                    = 2055,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 2524,
    /// HEAD
    Head                                                                   = 700,
    /// HEALTH-CHANNEL
    HealthChannel                                                          = 1726,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 2885,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 2863,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 1928,
    /// HEAP-USAGE
    HeapUsage                                                              = 1317,
    /// HI
    Hi                                                                     = 447,
    /// HIERARCHICAL-EOC
    HierarchicalEoc                                                        = 958,
    /// HIGH
    High                                                                   = 695,
    /// HINT
    Hint                                                                   = 2483,
    /// HOOK
    Hook                                                                   = 2602,
    /// HOST-PORT
    HostPort                                                               = 1477,
    /// HR
    Hr                                                                     = 357,
    /// HU
    Hu                                                                     = 722,
    /// HUB
    Hub                                                                    = 2566,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 2339,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 1331,
    /// HW-CATEGORY
    HwCategory                                                             = 211,
    /// HW-DESCRIPTION-ENTITY
    HwDescriptionEntity                                                    = 723,
    /// HW-ELEMENT
    HwElement                                                              = 1843,
    /// HW-PIN
    HwPin                                                                  = 974,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 2276,
    /// HW-TYPE
    HwType                                                                 = 1815,
    /// HY
    Hy                                                                     = 1871,
    /// I-4-G
    I4G                                                                    = 2393,
    /// I-PDU
    IPdu                                                                   = 999,
    /// I-PDU-PORT
    IPduPort                                                               = 2879,
    /// I-PDU-RECEIVED-BY-COM
    IPduReceivedByCom                                                      = 410,
    /// I-PDU-SENT-TO-IF
    IPduSentToIf                                                           = 2782,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 127,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 1690,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 1810,
    /// I-SIGNAL
    ISignal                                                                = 2849,
    /// I-SIGNAL-AVAILABLE-FOR-RTE
    ISignalAvailableForRte                                                 = 1029,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 1985,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 510,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 2146,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 1868,
    /// I-SIGNAL-PORT-TO-DIAGNOSTIC-EVENT-MAPPING
    ISignalPortToDiagnosticEventMapping                                    = 1008,
    /// I-SIGNAL-SENT-TO-COM
    ISignalSentToCom                                                       = 2112,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 142,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 380,
    /// IA
    Ia                                                                     = 663,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 1370,
    /// ICMP
    Icmp                                                                   = 1419,
    /// ICV-IGNORED
    IcvIgnored                                                             = 2389,
    /// ICV-NOT-SUPPORTED
    IcvNotSupported                                                        = 409,
    /// ICV-NOT-VERIFIED
    IcvNotVerified                                                         = 2137,
    /// ICV-OPTIONAL
    IcvOptional                                                            = 2603,
    /// ICV-SUPPORTED
    IcvSupported                                                           = 1965,
    /// ICV-VERIFIED
    IcvVerified                                                            = 619,
    /// IDENT-CAPTION
    IdentCaption                                                           = 2578,
    /// IDENTIFIABLE
    Identifiable                                                           = 2013,
    /// IDS-COMMON-ELEMENT
    IdsCommonElement                                                       = 2611,
    /// IDS-DESIGN
    IdsDesign                                                              = 639,
    /// IDS-MAPPING
    IdsMapping                                                             = 1197,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 761,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 332,
    /// IDS-PLATFORM-INSTANTIATION
    IdsPlatformInstantiation                                               = 1438,
    /// IDSM-ABSTRACT-PORT-INTERFACE
    IdsmAbstractPortInterface                                              = 1870,
    /// IDSM-CONTEXT-PROVIDER-INTERFACE
    IdsmContextProviderInterface                                           = 2685,
    /// IDSM-CONTEXT-PROVIDER-MAPPING
    IdsmContextProviderMapping                                             = 966,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 858,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 1845,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 2861,
    /// IDSM-QUALIFIED-EVENT-RECEIVER-INTERFACE
    IdsmQualifiedEventReceiverInterface                                    = 1705,
    /// IDSM-QUALIFIED-EVENT-RECEIVER-MAPPING
    IdsmQualifiedEventReceiverMapping                                      = 446,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 62,
    /// IDSM-REPORTING-MODE-PROVIDER-INTERFACE
    IdsmReportingModeProviderInterface                                     = 1304,
    /// IDSM-REPORTING-MODE-PROVIDER-MAPPING
    IdsmReportingModeProviderMapping                                       = 452,
    /// IDSM-TIMESTAMP-PROVIDER-INTERFACE
    IdsmTimestampProviderInterface                                         = 1661,
    /// IDSM-TIMESTAMP-PROVIDER-MAPPING
    IdsmTimestampProviderMapping                                           = 1501,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 2211,
    /// IE
    Ie                                                                     = 1324,
    /// IEC-61937
    Iec61937                                                               = 2559,
    /// IEEE-1722-ACF-BUS-PART-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722AcfBusPartRawDataStreamConsumerMapping                         = 689,
    /// IEEE-1722-ACF-BUS-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722AcfBusRawDataStreamConsumerMapping                             = 1960,
    /// IEEE-1722-RAW-DATA-STREAM-CONSUMER-INTERFACE
    Ieee1722RawDataStreamConsumerInterface                                 = 575,
    /// IEEE-1722-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722RawDataStreamConsumerMapping                                   = 1527,
    /// IEEE-1722-RAW-DATA-STREAM-MAPPING
    Ieee1722RawDataStreamMapping                                           = 2263,
    /// IEEE-1722-RAW-DATA-STREAM-PRODUCER-INTERFACE
    Ieee1722RawDataStreamProducerInterface                                 = 1949,
    /// IEEE-1722-RAW-DATA-STREAM-PRODUCER-MAPPING
    Ieee1722RawDataStreamProducerMapping                                   = 791,
    /// IEEE-1722-TP-AAF-CONNECTION
    Ieee1722TpAafConnection                                                = 1933,
    /// IEEE-1722-TP-ACF-BUS
    Ieee1722TpAcfBus                                                       = 797,
    /// IEEE-1722-TP-ACF-BUS-PART
    Ieee1722TpAcfBusPart                                                   = 918,
    /// IEEE-1722-TP-ACF-CAN
    Ieee1722TpAcfCan                                                       = 2653,
    /// IEEE-1722-TP-ACF-CAN-PART
    Ieee1722TpAcfCanPart                                                   = 692,
    /// IEEE-1722-TP-ACF-CONNECTION
    Ieee1722TpAcfConnection                                                = 888,
    /// IEEE-1722-TP-ACF-LIN
    Ieee1722TpAcfLin                                                       = 669,
    /// IEEE-1722-TP-ACF-LIN-PART
    Ieee1722TpAcfLinPart                                                   = 1306,
    /// IEEE-1722-TP-AV-CONNECTION
    Ieee1722TpAvConnection                                                 = 2411,
    /// IEEE-1722-TP-CONFIG
    Ieee1722TpConfig                                                       = 1566,
    /// IEEE-1722-TP-CONNECTION
    Ieee1722TpConnection                                                   = 1277,
    /// IEEE-1722-TP-CRF-CONNECTION
    Ieee1722TpCrfConnection                                                = 2050,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 651,
    /// IEEE-1722-TP-IIDC-CONNECTION
    Ieee1722TpIidcConnection                                               = 523,
    /// IEEE-1722-TP-RVF-CONNECTION
    Ieee1722TpRvfConnection                                                = 2197,
    /// IEEE802-11P
    Ieee802_11p                                                            = 2544,
    /// IEEE802-1AS
    Ieee802_1as                                                            = 1425,
    /// IEEE802-1AS-AUTOSAR
    Ieee802_1asAutosar                                                     = 2542,
    /// IGNITION
    Ignition                                                               = 2140,
    /// IGNORE
    Ignore                                                                 = 1360,
    /// IK
    Ik                                                                     = 2011,
    /// IMMEDIATE
    Immediate                                                              = 1792,
    /// IMPLEMENTATION
    Implementation                                                         = 1716,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 677,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 2025,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 1574,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 751,
    /// IMPLEMENTATION-PROPS
    ImplementationProps                                                    = 1115,
    /// IMPOSITION-TIME
    ImpositionTime                                                         = 742,
    /// IMPOSITION-TIME-DEFINITION-GROUP
    ImpositionTimeDefinitionGroup                                          = 1777,
    /// IN
    In                                                                     = 2864,
    /// INCLUDE-BUT-DO-NOT-START
    IncludeButDoNotStart                                                   = 2062,
    /// INCREASING
    Increasing                                                             = 2401,
    /// INDEPENDENT-VLAN-LEARNING
    IndependentVlanLearning                                                = 2330,
    /// INDICATE
    Indicate                                                               = 1127,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 383,
    /// INDIVIDUAL
    Individual                                                             = 95,
    /// INFINITE
    Infinite                                                               = 720,
    /// INFINITE-TIME-TO-RESPONSE
    InfiniteTimeToResponse                                                 = 1861,
    /// INFO
    Info                                                                   = 433,
    /// INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE
    InheritedFromArrayElementTypeSize                                      = 1735,
    /// INIT-EVENT
    InitEvent                                                              = 547,
    /// INLINE
    Inline                                                                 = 2424,
    /// INLINE-CONDITIONAL
    InlineConditional                                                      = 2056,
    /// INOUT
    Inout                                                                  = 1528,
    /// INSTALL
    Install                                                                = 716,
    /// INSTANCE-ID
    InstanceId                                                             = 1598,
    /// INSTRUCTION
    Instruction                                                            = 1708,
    /// INT-16-BIT
    Int16Bit                                                               = 1844,
    /// INT-24-BIT
    Int24Bit                                                               = 488,
    /// INT-32-BIT
    Int32Bit                                                               = 2054,
    /// INTEGER-32
    Integer32                                                              = 587,
    /// INTEGER-64
    Integer64                                                              = 1613,
    /// INTER-LET-ONLY
    InterLetOnly                                                           = 1617,
    /// INTER-PARTITION-INTRA-ECU
    InterPartitionIntraEcu                                                 = 1162,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 1357,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 2179,
    /// INTERGRITY-AND-CONFIDENTIALITY
    IntergrityAndConfidentiality                                           = 1472,
    /// INTERGRITY-WITHOUT-CONFIDENTIALITY
    IntergrityWithoutConfidentiality                                       = 2583,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 100,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 396,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 2516,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 1916,
    /// INTERRUPT
    Interrupt                                                              = 927,
    /// INTERRUPT-CAT-1
    InterruptCat1                                                          = 2046,
    /// INTERRUPT-CAT-2
    InterruptCat2                                                          = 852,
    /// INTRA-LET-EOC
    IntraLetEoc                                                            = 1763,
    /// INTRUSION-DETECTION-SECURITY-MANAGEMENT
    IntrusionDetectionSecurityManagement                                   = 2303,
    /// INVALID
    Invalid                                                                = 533,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 2489,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 2262,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 1679,
    /// IP-SEC-RULE
    IpSecRule                                                              = 439,
    /// IPSEC
    Ipsec                                                                  = 521,
    /// IS
    Is                                                                     = 2064,
    /// IS-EQUAL
    IsEqual                                                                = 1283,
    /// IS-EXPIRED
    IsExpired                                                              = 781,
    /// IS-FAILED
    IsFailed                                                               = 1740,
    /// IS-GREATER-OR-EQUAL
    IsGreaterOrEqual                                                       = 1275,
    /// IS-GREATER-THAN
    IsGreaterThan                                                          = 548,
    /// IS-GREATER-THAN-OR-EQUAL
    IsGreaterThanOrEqual                                                   = 44,
    /// IS-LESS-OR-EQUAL
    IsLessOrEqual                                                          = 230,
    /// IS-LESS-THAN
    IsLessThan                                                             = 2773,
    /// IS-LESS-THAN-OR-EQUAL
    IsLessThanOrEqual                                                      = 2620,
    /// IS-NOT-EQUAL
    IsNotEqual                                                             = 1589,
    /// IS-NOT-RELEVANT
    IsNotRelevant                                                          = 2684,
    /// IS-OK
    IsOk                                                                   = 2859,
    /// IS-RELEVANT
    IsRelevant                                                             = 1624,
    /// IS-STOPPED
    IsStopped                                                              = 14,
    /// ISO
    Iso                                                                    = 2300,
    /// ISO-11992--4
    Iso11992_4                                                             = 916,
    /// ISO-14229--1
    Iso14229_1                                                             = 687,
    /// ISO-15031--6
    Iso15031_6                                                             = 413,
    /// ISO-6
    Iso6                                                                   = 784,
    /// ISO-9141
    Iso9141                                                                = 2027,
    /// IT
    It                                                                     = 1311,
    /// ITALIC
    Italic                                                                 = 1897,
    /// ITU-BT-2020
    ItuBt2020                                                              = 1794,
    /// IW
    Iw                                                                     = 1521,
    /// J-1587
    J1587                                                                  = 339,
    /// J-1850
    J1850                                                                  = 0,
    /// J-1922
    J1922                                                                  = 1500,
    /// J-1939
    J1939                                                                  = 2043,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 200,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 851,
    /// J-1939-DCM
    J1939Dcm                                                               = 2222,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 952,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 726,
    /// J-1939-NM--AAC
    J1939NmAac                                                             = 1212,
    /// J-1939-NM--CCA
    J1939NmCca                                                             = 555,
    /// J-1939-NM--NCA
    J1939NmNca                                                             = 1830,
    /// J-1939-NM--SCA
    J1939NmSca                                                             = 2677,
    /// J-1939-NM--SVCA
    J1939NmSvca                                                            = 2024,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 818,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 2641,
    /// J-1939-NODE
    J1939Node                                                              = 2869,
    /// J-1939-PROTECTED-I-PDU
    J1939ProtectedIPdu                                                     = 1546,
    /// J-1939-REQUEST-MANAGER
    J1939RequestManager                                                    = 1857,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 2814,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 1621,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 849,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 793,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 2623,
    /// J-19391
    J19391                                                                 = 2168,
    /// J-193910
    J193910                                                                = 2780,
    /// J-193911
    J193911                                                                = 2415,
    /// J-19392
    J19392                                                                 = 1643,
    /// J-19393
    J19393                                                                 = 1321,
    /// J-19394
    J19394                                                                 = 83,
    /// J-19395
    J19395                                                                 = 424,
    /// J-19396
    J19396                                                                 = 1700,
    /// J-19397
    J19397                                                                 = 2428,
    /// J-19398
    J19398                                                                 = 2163,
    /// J-19399
    J19399                                                                 = 198,
    /// JA
    Ja                                                                     = 2030,
    /// JAVA
    Java                                                                   = 1664,
    /// JI
    Ji                                                                     = 596,
    /// JPG
    Jpg                                                                    = 1595,
    /// JUSTIFY
    Justify                                                                = 1334,
    /// JW
    Jw                                                                     = 1340,
    /// KA
    Ka                                                                     = 2113,
    /// KEEP
    Keep                                                                   = 960,
    /// KEEP-ALL
    KeepAll                                                                = 1766,
    /// KEEP-EXISTING
    KeepExisting                                                           = 1161,
    /// KEEP-LAST
    KeepLast                                                               = 91,
    /// KEY-DERIVATION
    KeyDerivation                                                          = 1786,
    /// KEY-SERVER
    KeyServer                                                              = 1082,
    /// KEY-STORAGE
    KeyStorage                                                             = 2763,
    /// KEYWORD
    Keyword                                                                = 580,
    /// KEYWORD-SET
    KeywordSet                                                             = 1255,
    /// KK
    Kk                                                                     = 2579,
    /// KL
    Kl                                                                     = 1518,
    /// KM
    Km                                                                     = 1622,
    /// KN
    Kn                                                                     = 1744,
    /// KO
    Ko                                                                     = 2740,
    /// KS
    Ks                                                                     = 76,
    /// KU
    Ku                                                                     = 175,
    /// KY
    Ky                                                                     = 1176,
    /// LA
    La                                                                     = 2291,
    /// LAND
    Land                                                                   = 2722,
    /// LAST-FAILED
    LastFailed                                                             = 465,
    /// LAST-IS-BEST
    LastIsBest                                                             = 1508,
    /// LAST-MODE
    LastMode                                                               = 112,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 475,
    /// LEAF-OF-TARGET-CONTAINER
    LeafOfTargetContainer                                                  = 1391,
    /// LEFT
    Left                                                                   = 296,
    /// LEGACY
    Legacy                                                                 = 681,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 1531,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 1247,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 697,
    /// LIGHT
    Light                                                                  = 2033,
    /// LIMIT
    Limit                                                                  = 1270,
    /// LIMIT-TO-PAGE
    LimitToPage                                                            = 1323,
    /// LIMIT-TO-TEXT
    LimitToText                                                            = 322,
    /// LIN-CLUSTER
    LinCluster                                                             = 1968,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 2474,
    /// LIN-COMMUNICATION-CONTROLLER
    LinCommunicationController                                             = 2462,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 578,
    /// LIN-FRAME
    LinFrame                                                               = 378,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 36,
    /// LIN-MASTER
    LinMaster                                                              = 811,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 662,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 2848,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 1905,
    /// LIN-SLAVE
    LinSlave                                                               = 2478,
    /// LIN-SLAVE-CONFIG-IDENT
    LinSlaveConfigIdent                                                    = 1392,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 2856,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 1673,
    /// LIN-TP-NODE
    LinTpNode                                                              = 2788,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 834,
    /// LINK
    Link                                                                   = 2126,
    /// LINK-LOCAL
    LinkLocal                                                              = 2332,
    /// LINK-LOCAL--DOIP
    LinkLocalDoip                                                          = 749,
    /// LINK-TIME
    LinkTime                                                               = 1808,
    /// LINKER
    Linker                                                                 = 2713,
    /// LISTEN
    Listen                                                                 = 570,
    /// LN
    Ln                                                                     = 2180,
    /// LO
    Lo                                                                     = 1251,
    /// LOCAL
    Local                                                                  = 648,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 1473,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 2244,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 1517,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 2270,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 871,
    /// LOGICAL-AND
    LogicalAnd                                                             = 2867,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 1986,
    /// LOGICAL-OR
    LogicalOr                                                              = 2363,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 1889,
    /// LONG-HEADER
    LongHeader                                                             = 1647,
    /// LOW
    Low                                                                    = 65,
    /// LOWER-12-BIT
    Lower12Bit                                                             = 2081,
    /// LOWER-8-BIT
    Lower8Bit                                                              = 1674,
    /// LT
    Lt                                                                     = 2364,
    /// LT-AFFECTS-PB
    LtAffectsPb                                                            = 1200,
    /// LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING
    LtMessageCollectionToPortPrototypeMapping                              = 301,
    /// LTS-13
    Lts13                                                                  = 1024,
    /// LV
    Lv                                                                     = 668,
    /// MAC-ADDRESS-VLAN-MEMBERSHIP
    MacAddressVlanMembership                                               = 1329,
    /// MAC-LAYER-RAW-DATA-STREAM-INTERFACE
    MacLayerRawDataStreamInterface                                         = 2311,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 1563,
    /// MAC-SEC-GLOBAL-KAY-PROPS
    MacSecGlobalKayProps                                                   = 1337,
    /// MAC-SEC-KAY-PARTICIPANT
    MacSecKayParticipant                                                   = 67,
    /// MAC-SEC-PARTICIPANT-SET
    MacSecParticipantSet                                                   = 2097,
    /// MACHINE
    Machine                                                                = 1205,
    /// MACHINE-CYCLE
    MachineCycle                                                           = 907,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 1666,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 1557,
    /// MACHINE-TIMING
    MachineTiming                                                          = 1149,
    /// MACRO
    Macro                                                                  = 2099,
    /// MALFUNCTION
    Malfunction                                                            = 72,
    /// MANUAL-BY-PARTICIPANT
    ManualByParticipant                                                    = 2769,
    /// MANUAL-BY-TOPIC
    ManualByTopic                                                          = 487,
    /// MANUFACTURING
    Manufacturing                                                          = 138,
    /// MAPPING-SCOPE-CORE
    MappingScopeCore                                                       = 1242,
    /// MAPPING-SCOPE-ECU
    MappingScopeEcu                                                        = 2341,
    /// MAPPING-SCOPE-PARTITION
    MappingScopePartition                                                  = 1287,
    /// MASEKD-NEW-EQUALS-MASKED-OLD
    MasekdNewEqualsMaskedOld                                               = 289,
    /// MASEKD-NEW-EQUALS-X
    MasekdNewEqualsX                                                       = 2819,
    /// MASKED-NEW-DIFFERS-MASKED-OLD
    MaskedNewDiffersMaskedOld                                              = 2754,
    /// MASKED-NEW-DIFFERS-X
    MaskedNewDiffersX                                                      = 711,
    /// MASKED-NEW-EQUALS-MASKED-OLD
    MaskedNewEqualsMaskedOld                                               = 308,
    /// MASKED-NEW-EQUALS-X
    MaskedNewEqualsX                                                       = 794,
    /// MASTER
    Master                                                                 = 1074,
    /// MASTER-ECU
    MasterEcu                                                              = 2886,
    /// MAX
    Max                                                                    = 2184,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 950,
    /// MC-FUNCTION
    McFunction                                                             = 2729,
    /// MC-GROUP
    McGroup                                                                = 393,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 1782,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 2877,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 1084,
    /// MEASUREMENT-POINT
    MeasurementPoint                                                       = 2827,
    /// MEDIUM
    Medium                                                                 = 955,
    /// MEMORY-SECTION
    MemorySection                                                          = 1128,
    /// MEMORY-USAGE
    MemoryUsage                                                            = 2585,
    /// METHOD-MAPPING
    MethodMapping                                                          = 2129,
    /// MG
    Mg                                                                     = 1911,
    /// MI
    Mi                                                                     = 606,
    /// MIDDLE
    Middle                                                                 = 1539,
    /// MIN
    Min                                                                    = 1335,
    /// MINIMUM-MINOR-VERSION
    MinimumMinorVersion                                                    = 389,
    /// MIXED
    Mixed                                                                  = 2741,
    /// MIXED-29-BIT
    Mixed29Bit                                                             = 1196,
    /// MK
    Mk                                                                     = 836,
    /// ML
    Ml                                                                     = 1424,
    /// MN
    Mn                                                                     = 1677,
    /// MO
    Mo                                                                     = 2018,
    /// MODE-ACCESS-POINT-IDENT
    ModeAccessPointIdent                                                   = 1537,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 2871,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 2305,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 2622,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 2091,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 732,
    /// MODE-DECLARATION-REQUESTED
    ModeDeclarationRequested                                               = 1184,
    /// MODE-DECLARATION-SWITCH-COMPLETED
    ModeDeclarationSwitchCompleted                                         = 331,
    /// MODE-DECLARATION-SWITCH-INITIATED
    ModeDeclarationSwitchInitiated                                         = 2388,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 1509,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 1837,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 208,
    /// MODE-SWITCH-SENDER-COM-SPEC-PROPS
    ModeSwitchSenderComSpecProps                                           = 2517,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 194,
    /// MODE-TRANSITION
    ModeTransition                                                         = 1971,
    /// MODELED
    Modeled                                                                = 2565,
    /// MONITOR-MODE
    MonitorMode                                                            = 2229,
    /// MONO
    Mono                                                                   = 1802,
    /// MONOCHROME
    Monochrome                                                             = 2882,
    /// MONOTONOUS
    Monotonous                                                             = 1027,
    /// MOST-SIGNIFICANT-BYTE-FIRST
    MostSignificantByteFirst                                               = 1265,
    /// MOST-SIGNIFICANT-BYTE-LAST
    MostSignificantByteLast                                                = 256,
    /// MR
    Mr                                                                     = 2674,
    /// MS
    Ms                                                                     = 2765,
    /// MT
    Mt                                                                     = 2910,
    /// MULTICORE-REENTRANT
    MulticoreReentrant                                                     = 2266,
    /// MULTILANGUAGE-REFERRABLE
    MultilanguageReferrable                                                = 2739,
    /// MULTIPLE
    Multiple                                                               = 2092,
    /// MULTIPLE-OCCURRENCES
    MultipleOccurrences                                                    = 1571,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 1605,
    /// MY
    My                                                                     = 1071,
    /// N-PDU
    NPdu                                                                   = 2243,
    /// NA
    Na                                                                     = 856,
    /// NAND
    Nand                                                                   = 2412,
    /// NE
    Ne                                                                     = 1506,
    /// NET
    Net                                                                    = 2038,
    /// NETWORK
    Network                                                                = 1997,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 1140,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 855,
    /// NETWORK-HANDLE-PORT-MAPPING
    NetworkHandlePortMapping                                               = 971,
    /// NETWORK-MANAGEMENT-PORT-INTERFACE
    NetworkManagementPortInterface                                         = 373,
    /// NETWORK-REPRESENTATION-FROM-COM-SPEC
    NetworkRepresentationFromComSpec                                       = 942,
    /// NEVER
    Never                                                                  = 2673,
    /// NEW-IS-DIFFERENT
    NewIsDifferent                                                         = 1101,
    /// NEW-IS-EQUAL
    NewIsEqual                                                             = 628,
    /// NEW-IS-GREATER
    NewIsGreater                                                           = 2663,
    /// NEW-IS-GREATER-OR-EQUAL
    NewIsGreaterOrEqual                                                    = 2284,
    /// NEW-IS-LESS
    NewIsLess                                                              = 1682,
    /// NEW-IS-LESS-OR-EQUAL
    NewIsLessOrEqual                                                       = 2271,
    /// NEW-IS-OUTSIDE
    NewIsOutside                                                           = 2628,
    /// NEW-IS-WITHIN
    NewIsWithin                                                            = 2148,
    /// NEWLINE
    Newline                                                                = 2862,
    /// NEWLINE-IF-NECESSARY
    NewlineIfNecessary                                                     = 2508,
    /// NFOLD
    Nfold                                                                  = 1838,
    /// NL
    Nl                                                                     = 2687,
    /// NM-CLUSTER
    NmCluster                                                              = 2809,
    /// NM-CONFIG
    NmConfig                                                               = 2261,
    /// NM-ECU
    NmEcu                                                                  = 247,
    /// NM-HANDLE-ACTIVE-TO-FUNCTION-GROUP-STATE
    NmHandleActiveToFunctionGroupState                                     = 2772,
    /// NM-HANDLE-INACTIVE-TO-FUNCTION-GROUP-STATE
    NmHandleInactiveToFunctionGroupState                                   = 898,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 1836,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 1881,
    /// NM-INTERACTS-WITH-SM-MAPPING
    NmInteractsWithSmMapping                                               = 2331,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 1822,
    /// NM-NODE
    NmNode                                                                 = 2503,
    /// NM-PDU
    NmPdu                                                                  = 2893,
    /// NO
    No                                                                     = 620,
    /// NO-ACK
    NoAck                                                                  = 1312,
    /// NO-AFFECT
    NoAffect                                                               = 1995,
    /// NO-BOOT
    NoBoot                                                                 = 1272,
    /// NO-BREAK
    NoBreak                                                                = 2101,
    /// NO-CHECKPOINT-SUPERVISION
    NoCheckpointSupervision                                                = 333,
    /// NO-COM
    NoCom                                                                  = 1878,
    /// NO-CONSISTENCY-MECHANISM
    NoConsistencyMechanism                                                 = 508,
    /// NO-DEFAULT
    NoDefault                                                              = 1812,
    /// NO-FLOAT
    NoFloat                                                                = 2807,
    /// NO-HEADER
    NoHeader                                                               = 2377,
    /// NO-KEEP
    NoKeep                                                                 = 1918,
    /// NO-MONOTONY
    NoMonotony                                                             = 1846,
    /// NO-NEWLINE
    NoNewline                                                              = 15,
    /// NO-OBD-SUPPORT
    NoObdSupport                                                           = 2205,
    /// NO-PGWIDE
    NoPgwide                                                               = 786,
    /// NO-PROTECTION
    NoProtection                                                           = 482,
    /// NO-RETURN-VALUE-PROVIDED
    NoReturnValueProvided                                                  = 327,
    /// NO-SHOW-ALIAS-NAME
    NoShowAliasName                                                        = 2700,
    /// NO-SHOW-CATEGORY
    NoShowCategory                                                         = 2469,
    /// NO-SHOW-CONTENT
    NoShowContent                                                          = 1093,
    /// NO-SHOW-LONG-NAME
    NoShowLongName                                                         = 745,
    /// NO-SHOW-NUMBER
    NoShowNumber                                                           = 1670,
    /// NO-SHOW-PAGE
    NoShowPage                                                             = 2704,
    /// NO-SHOW-SEE
    NoShowSee                                                              = 1745,
    /// NO-SHOW-SHORT-NAME
    NoShowShortName                                                        = 1609,
    /// NO-SHOW-TYPE
    NoShowType                                                             = 2738,
    /// NO-SLOPPY
    NoSloppy                                                               = 1824,
    /// NO-STATUS-BYTE-CHANGE
    NoStatusByteChange                                                     = 1278,
    /// NO-STORE-EVENT
    NoStoreEvent                                                           = 1282,
    /// NO-SUPERVISION
    NoSupervision                                                          = 1268,
    /// NO-SUPPORT
    NoSupport                                                              = 2103,
    /// NO-TRANSFORMER-ERROR-HANDLING
    NoTransformerErrorHandling                                             = 132,
    /// NO-TRANSFORMER-STATUS-FORWARDING
    NoTransformerStatusForwarding                                          = 944,
    /// NO-TRUSTED-PLATFORM-SUPPORT
    NoTrustedPlatformSupport                                               = 262,
    /// NODE
    Node                                                                   = 1638,
    /// NOHREF
    Nohref                                                                 = 1618,
    /// NON-EMMISSION-RELATED-DTC
    NonEmmissionRelatedDtc                                                 = 873,
    /// NON-OS-MODULE-INSTANTIATION
    NonOsModuleInstantiation                                               = 887,
    /// NON-REENTRANT
    NonReentrant                                                           = 805,
    /// NON-VOLATILE
    NonVolatile                                                            = 441,
    /// NON-VOLATILE-RAM-MANAGER
    NonVolatileRamManager                                                  = 821,
    /// NONE
    None                                                                   = 2669,
    /// NORMALFIXED
    Normalfixed                                                            = 864,
    /// NOT
    Not                                                                    = 1042,
    /// NOT-ACCESSIBLE
    NotAccessible                                                          = 2215,
    /// NOT-AVAILABLE
    NotAvailable                                                           = 428,
    /// NOT-DEFINED
    NotDefined                                                             = 1964,
    /// NOT-EQUAL
    NotEqual                                                               = 164,
    /// NOT-SENT
    NotSent                                                                = 1182,
    /// NOT-TESTED
    NotTested                                                              = 1627,
    /// NOT-VALID
    NotValid                                                               = 68,
    /// NOTHING
    Nothing                                                                = 2467,
    /// NOTIFICATION
    Notification                                                           = 1639,
    /// NTP--RFC-958
    NtpRfc958                                                              = 1260,
    /// NUMBER
    Number                                                                 = 642,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 2342,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 616,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 1660,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 2157,
    /// NV-RAM-MANAGER
    NvRamManager                                                           = 2347,
    /// OBD
    Obd                                                                    = 2061,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 2060,
    /// OBD-DCY
    ObdDcy                                                                 = 2288,
    /// OBD-DRIVING-CYCLE
    ObdDrivingCycle                                                        = 2708,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 2187,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 2587,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 2438,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 2237,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 530,
    /// OBJECT
    Object                                                                 = 1894,
    /// OBSERVER
    Observer                                                               = 2095,
    /// OBSERVER-BASED
    ObserverBased                                                          = 145,
    /// OC
    Oc                                                                     = 1649,
    /// OCCURENCE
    Occurence                                                              = 1655,
    /// OEM-BOOT
    OemBoot                                                                = 1929,
    /// OEM-BOOT-RESP-APP
    OemBootRespApp                                                         = 2063,
    /// OFF
    Off                                                                    = 314,
    /// OFFSET
    Offset                                                                 = 1045,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 1379,
    /// OM
    Om                                                                     = 1343,
    /// ON-CHANGE
    OnChange                                                               = 1061,
    /// ON-CHANGE-OF-DATA-IDENTIFIER
    OnChangeOfDataIdentifier                                               = 386,
    /// ON-COMPARISON-OF-VALUES
    OnComparisonOfValues                                                   = 842,
    /// ON-DTC-STATUS-CHANGE
    OnDtcStatusChange                                                      = 1320,
    /// ON-ENTRY
    OnEntry                                                                = 882,
    /// ON-EXIT
    OnExit                                                                 = 1656,
    /// ON-TRANSITION
    OnTransition                                                           = 1234,
    /// ONE-EVERY-N
    OneEveryN                                                              = 1201,
    /// ONLY-THIS-CYCLE-AND-READINESS
    OnlyThisCycleAndReadiness                                              = 48,
    /// OPAQUE
    Opaque                                                                 = 1702,
    /// OPEN
    Open                                                                   = 617,
    /// OPERATING-SYSTEM
    OperatingSystem                                                        = 478,
    /// OPERATION-CALL-RECEIVED
    OperationCallReceived                                                  = 583,
    /// OPERATION-CALL-RESPONSE-RECEIVED
    OperationCallResponseReceived                                          = 978,
    /// OPERATION-CALL-RESPONSE-SENT
    OperationCallResponseSent                                              = 60,
    /// OPERATION-CALLED
    OperationCalled                                                        = 130,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 55,
    /// OPTIONS
    Options                                                                = 2213,
    /// OR
    Or                                                                     = 1542,
    /// ORDINARY-EOC
    OrdinaryEoc                                                            = 2753,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 747,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 2698,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 375,
    /// OTHER
    Other                                                                  = 47,
    /// OUT
    Out                                                                    = 1151,
    /// OVERRIDE
    Override                                                               = 2678,
    /// OVERWRITE
    Overwrite                                                              = 2231,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 924,
    /// PA
    Pa                                                                     = 310,
    /// PACKAGEABLE-ELEMENT
    PackageableElement                                                     = 291,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 522,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 544,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 585,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 2272,
    /// PARTIAL-NETWORK
    PartialNetwork                                                         = 973,
    /// PARTITION
    Partition                                                              = 45,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 1054,
    /// PASSIVE
    Passive                                                                = 707,
    /// PASSTHROUGH
    Passthrough                                                            = 1254,
    /// PAYLOAD-AS-ARRAY
    PayloadAsArray                                                         = 1779,
    /// PAYLOAD-AS-POINTER-TO-ARRAY
    PayloadAsPointerToArray                                                = 1227,
    /// PC-AFFECTS-LT
    PcAffectsLt                                                            = 2744,
    /// PC-AFFECTS-LT-AND-PB
    PcAffectsLtAndPb                                                       = 2382,
    /// PC-AFFECTS-PB
    PcAffectsPb                                                            = 954,
    /// PCM
    Pcm                                                                    = 1687,
    /// PDF
    Pdf                                                                    = 1558,
    /// PDU
    Pdu                                                                    = 2084,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 121,
    /// PDU-R
    PduR                                                                   = 2029,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 754,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 2770,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 2839,
    /// PEER
    Peer                                                                   = 1436,
    /// PENDING
    Pending                                                                = 227,
    /// PER-EXECUTABLE
    PerExecutable                                                          = 1122,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 2304,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 77,
    /// PERIODIC-RATE-FAST
    PeriodicRateFast                                                       = 2631,
    /// PERIODIC-RATE-MEDIUM
    PeriodicRateMedium                                                     = 33,
    /// PERIODIC-RATE-SLOW
    PeriodicRateSlow                                                       = 173,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 768,
    /// PERSISTENCY-DEPLOYMENT
    PersistencyDeployment                                                  = 2031,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT
    PersistencyDeploymentElement                                           = 311,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 984,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 2577,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 1468,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 1006,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 97,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 1588,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 2441,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 1503,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 2671,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 778,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 1695,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 2845,
    /// PERSISTENCY-INTERFACE
    PersistencyInterface                                                   = 505,
    /// PERSISTENCY-INTERFACE-ELEMENT
    PersistencyInterfaceElement                                            = 51,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 2672,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 1615,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 489,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 1126,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 1739,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING
    PersistencyPortPrototypeToDeploymentMapping                            = 2792,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 1105,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 2820,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 2451,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 876,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE
    PersistencyRedundancyHandlingScopeDatabase                             = 1901,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT
    PersistencyRedundancyHandlingScopeElement                              = 2556,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE
    PersistencyRedundancyHandlingScopeFile                                 = 162,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY
    PersistencyRedundancyHandlingScopeKey                                  = 381,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE
    PersistencyRedundancyHandlingScopeStorage                              = 2621,
    /// PERSISTENT
    Persistent                                                             = 316,
    /// PGWIDE
    Pgwide                                                                 = 1307,
    /// PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE
    PhmAbstractRecoveryNotificationInterface                               = 384,
    /// PHM-ACTION
    PhmAction                                                              = 1281,
    /// PHM-ACTION-ITEM
    PhmActionItem                                                          = 2007,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 2247,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 1883,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 932,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 1018,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 2536,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 2026,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 579,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 2456,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 1484,
    /// PHM-RULE
    PhmRule                                                                = 901,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 483,
    /// PHM-SUPERVISION
    PhmSupervision                                                         = 1973,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 346,
    /// PHYSICAL
    Physical                                                               = 1217,
    /// PHYSICAL-ADDRESS
    PhysicalAddress                                                        = 1497,
    /// PHYSICAL-CAN-FD
    PhysicalCanFd                                                          = 2239,
    /// PHYSICAL-CHANNEL
    PhysicalChannel                                                        = 908,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 1294,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 499,
    /// PL
    Pl                                                                     = 2629,
    /// PLAIN
    Plain                                                                  = 181,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 2082,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 780,
    /// PLATFORM-HEALTH-MANAGEMENT-INTERFACE
    PlatformHealthManagementInterface                                      = 1848,
    /// PLATFORM-MODULE-ENDPOINT-CONFIGURATION
    PlatformModuleEndpointConfiguration                                    = 8,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 507,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 302,
    /// PNC-MAPPING-IDENT
    PncMappingIdent                                                        = 2175,
    /// PNG
    Png                                                                    = 2351,
    /// POLY
    Poly                                                                   = 2324,
    /// PORT
    Port                                                                   = 1707,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 451,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 1930,
    /// PORT-GROUP
    PortGroup                                                              = 774,
    /// PORT-INTERFACE
    PortInterface                                                          = 649,
    /// PORT-INTERFACE-DEFINITION
    PortInterfaceDefinition                                                = 2185,
    /// PORT-INTERFACE-MAPPING
    PortInterfaceMapping                                                   = 825,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 2617,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 2784,
    /// PORT-PROTOTYPE
    PortPrototype                                                          = 2802,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 2543,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 1489,
    /// POST
    Post                                                                   = 214,
    /// POST-BUILD
    PostBuild                                                              = 874,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 1667,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 1752,
    /// POWER
    Power                                                                  = 471,
    /// POWER-WINDOW-TIME
    PowerWindowTime                                                        = 470,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 2693,
    /// PRE--R-4--2
    PreR4_2                                                                = 1996,
    /// PRE-COMPILE
    PreCompile                                                             = 527,
    /// PRE-COMPILE-TIME
    PreCompileTime                                                         = 2235,
    /// PRECONFIGURED-CONFIGURATION
    PreconfiguredConfiguration                                             = 1300,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 1722,
    /// PREEMPTABLE
    Preemptable                                                            = 2223,
    /// PRESENTATION-CONTINUOUS
    PresentationContinuous                                                 = 980,
    /// PRESENTATION-DISCRETE
    PresentationDiscrete                                                   = 2079,
    /// PRESHARED-KEY-IDENTITY
    PresharedKeyIdentity                                                   = 1417,
    /// PRIMARY-ECU
    PrimaryEcu                                                             = 1523,
    /// PRIMITIVE
    Primitive                                                              = 2396,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 94,
    /// PRIO-OCC
    PrioOcc                                                                = 2348,
    /// PRIVATE-KEY
    PrivateKey                                                             = 186,
    /// PROCESS
    Process                                                                = 1651,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 2421,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 1172,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 1785,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 886,
    /// PROCESS-IS-NOT-SELF-TERMINATING
    ProcessIsNotSelfTerminating                                            = 2878,
    /// PROCESS-IS-SELF-TERMINATING
    ProcessIsSelfTerminating                                               = 2104,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 1017,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 1718,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 1134,
    /// PROCESSING-STYLE-ASYNCHRONOUS
    ProcessingStyleAsynchronous                                            = 1800,
    /// PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR
    ProcessingStyleAsynchronousWithError                                   = 1094,
    /// PROCESSING-STYLE-SYNCHRONOUS
    ProcessingStyleSynchronous                                             = 542,
    /// PROCESSOR
    Processor                                                              = 434,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 2479,
    /// PRODUCER
    Producer                                                               = 2275,
    /// PROPRIETARY-1
    Proprietary1                                                           = 2506,
    /// PROPRIETARY-2
    Proprietary2                                                           = 1328,
    /// PROTECT-LAMP
    ProtectLamp                                                            = 244,
    /// PROTECTED
    Protected                                                              = 1048,
    /// PROVIDED-AP-SERVICE-INSTANCE
    ProvidedApServiceInstance                                              = 220,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 1728,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 601,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 2858,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 172,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 2553,
    /// PS
    Ps                                                                     = 30,
    /// PSK
    Psk                                                                    = 1368,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 2905,
    /// PT
    Pt                                                                     = 1167,
    /// PTP--IEEE-1588--2002
    PtpIeee1588_2002                                                       = 1874,
    /// PTP--IEEE-1588--2008
    PtpIeee1588_2008                                                       = 2433,
    /// PUBLIC-KEY
    PublicKey                                                              = 414,
    /// PUBLISHED-INFORMATION
    PublishedInformation                                                   = 1305,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 595,
    /// PUT
    Put                                                                    = 292,
    /// QU
    Qu                                                                     = 1835,
    /// QUEUED
    Queued                                                                 = 1491,
    /// QUEUED-RECEIVER-COM-SPEC-PROPS
    QueuedReceiverComSpecProps                                             = 1141,
    /// R-4--2
    R4_2                                                                   = 671,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 85,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 109,
    /// RAW
    Raw                                                                    = 2528,
    /// RAW-DATA
    RawData                                                                = 1781,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 376,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 1303,
    /// RAW-DATA-STREAM-GRANT
    RawDataStreamGrant                                                     = 770,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 1823,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 2486,
    /// RAW-DATA-STREAM-MAPPING
    RawDataStreamMapping                                                   = 1092,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 320,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 788,
    /// REACTION
    Reaction                                                               = 1936,
    /// READ
    Read                                                                   = 155,
    /// READ-ONLY
    ReadOnly                                                               = 534,
    /// READ-WRITE
    ReadWrite                                                              = 2463,
    /// REBOOT
    Reboot                                                                 = 297,
    /// RECOMMENDED-CONFIGURATION
    RecommendedConfiguration                                               = 2402,
    /// RECORD-VALUE-FIELD
    RecordValueField                                                       = 1170,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 1932,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 556,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 444,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 1941,
    /// RECT
    Rect                                                                   = 2689,
    /// RED-STOP-LAMP
    RedStopLamp                                                            = 2466,
    /// REDUNDANT
    Redundant                                                              = 667,
    /// REDUNDANT-PER-ELEMENT
    RedundantPerElement                                                    = 1248,
    /// REDUNDANT-PER-KEY
    RedundantPerKey                                                        = 2059,
    /// REF-ALL
    RefAll                                                                 = 24,
    /// REF-NON-STANDARD
    RefNonStandard                                                         = 315,
    /// REF-NONE
    RefNone                                                                = 2067,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 342,
    /// REFERRABLE
    Referrable                                                             = 427,
    /// REGULAR
    Regular                                                                = 1310,
    /// REJECT
    Reject                                                                 = 1606,
    /// RELIABLE
    Reliable                                                               = 113,
    /// REMOVE
    Remove                                                                 = 979,
    /// REPETITIVE-EOC
    RepetitiveEoc                                                          = 2181,
    /// REPLACE
    Replace                                                                = 2588,
    /// REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE
    ReplaceByTimeoutSubstitutionValue                                      = 2379,
    /// REPORT
    Report                                                                 = 29,
    /// REPORT-AFTER-INIT
    ReportAfterInit                                                        = 1805,
    /// REPORT-BEFORE-INIT
    ReportBeforeInit                                                       = 2509,
    /// REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE
    ReportDtcRecordInformationOnDtcStatusChange                            = 2723,
    /// REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE
    ReportMostRecentDtcOnStatusChange                                      = 286,
    /// REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST
    ReportingInChronlogicalOrderOldestFirst                                = 1924,
    /// REPORTING-IN-CHRONOLOGICAL-ORDER-OLDEST-FIRST
    ReportingInChronologicalOrderOldestFirst                               = 1244,
    /// REPORTS-EXECUTION-STATE
    ReportsExecutionState                                                  = 2779,
    /// REQUEST
    Request                                                                = 939,
    /// REQUEST-CALLBACK-TYPE-MANUFACTURER
    RequestCallbackTypeManufacturer                                        = 1333,
    /// REQUEST-CALLBACK-TYPE-SUPPLIER
    RequestCallbackTypeSupplier                                            = 1238,
    /// REQUEST-NO-RETURN
    RequestNoReturn                                                        = 561,
    /// REQUIRED-AP-SERVICE-INSTANCE
    RequiredApServiceInstance                                              = 717,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 2192,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 670,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 1192,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 2520,
    /// REQUIRES-CALLBACK-EXECUTION
    RequiresCallbackExecution                                              = 2828,
    /// RES-AXIS
    ResAxis                                                                = 1731,
    /// RESET-ECU
    ResetEcu                                                               = 313,
    /// RESET-MACHINE
    ResetMachine                                                           = 285,
    /// RESET-MCU
    ResetMcu                                                               = 1280,
    /// RESET-VM
    ResetVm                                                                = 124,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 282,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 571,
    /// RESPOND-AFTER-RESET
    RespondAfterReset                                                      = 1961,
    /// RESPOND-BEFORE-RESET
    RespondBeforeReset                                                     = 622,
    /// RESPONSE
    Response                                                               = 868,
    /// RESPONSE-SYNCHRONIZATION
    ResponseSynchronization                                                = 590,
    /// REST-ABSTRACT-ENDPOINT
    RestAbstractEndpoint                                                   = 263,
    /// REST-ABSTRACT-NUMERICAL-PROPERTY-DEF
    RestAbstractNumericalPropertyDef                                       = 2241,
    /// REST-ABSTRACT-PROPERTY-DEF
    RestAbstractPropertyDef                                                = 815,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 1057,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 2742,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 1157,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 2367,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 1456,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 1787,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 1596,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 1706,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 2762,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 2297,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 591,
    /// REST-PRIMITIVE-PROPERTY-DEF
    RestPrimitivePropertyDef                                               = 502,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 551,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 2002,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 421,
    /// RESTART
    Restart                                                                = 1600,
    /// RESTART-APPLICATION
    RestartApplication                                                     = 430,
    /// RES_AXIS
    Resaxis                                                                = 2657,
    /// RETURN-ON-EVENT-CLEARED
    ReturnOnEventCleared                                                   = 2497,
    /// RETURN-ON-EVENT-STOPPED
    ReturnOnEventStopped                                                   = 2327,
    /// RETURN-VALUE-PROVIDED
    ReturnValueProvided                                                    = 1315,
    /// RIGHT
    Right                                                                  = 2651,
    /// RM
    Rm                                                                     = 1684,
    /// RN
    Rn                                                                     = 2643,
    /// RO
    Ro                                                                     = 2482,
    /// ROLL-BACK
    RollBack                                                               = 2450,
    /// ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE
    RootSwClusterDesignComponentPrototype                                  = 558,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 1418,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 2823,
    /// ROTATE-180
    Rotate180                                                              = 1011,
    /// ROTATE-180-LIMIT-TO-TEXT
    Rotate180LimitToText                                                   = 1567,
    /// ROTATE-90-CCW
    Rotate90Ccw                                                            = 1741,
    /// ROTATE-90-CCW-FIT-TO-TEXT
    Rotate90CcwFitToText                                                   = 1089,
    /// ROTATE-90-CCW-LIMIT-TO-TEXT
    Rotate90CcwLimitToText                                                 = 205,
    /// ROTATE-90-CW
    Rotate90Cw                                                             = 2551,
    /// ROTATE-90-CW-FIT-TO-TEXT
    Rotate90CwFitToText                                                    = 2667,
    /// ROTATE-90-CW-LIMIT-TO-TEXT
    Rotate90CwLimitToText                                                  = 835,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 1671,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 280,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 293,
    /// ROUTER
    Router                                                                 = 1496,
    /// ROUTER-ADVERTISEMENT
    RouterAdvertisement                                                    = 1,
    /// RPT-COMPONENT
    RptComponent                                                           = 1780,
    /// RPT-CONTAINER
    RptContainer                                                           = 599,
    /// RPT-ENABLER-RAM
    RptEnablerRam                                                          = 740,
    /// RPT-ENABLER-RAM-AND-ROM
    RptEnablerRamAndRom                                                    = 1296,
    /// RPT-ENABLER-ROM
    RptEnablerRom                                                          = 936,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 709,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 518,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 1158,
    /// RPT-LEVEL-1
    RptLevel1                                                              = 1730,
    /// RPT-LEVEL-2
    RptLevel2                                                              = 464,
    /// RPT-LEVEL-3
    RptLevel3                                                              = 1991,
    /// RPT-PROFILE
    RptProfile                                                             = 57,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 1459,
    /// RSA
    Rsa                                                                    = 665,
    /// RTE-EVENT
    RteEvent                                                               = 2576,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 2840,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 2001,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 1053,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 673,
    /// RTPGE
    Rtpge                                                                  = 1839,
    /// RU
    Ru                                                                     = 650,
    /// RULE
    Rule                                                                   = 1482,
    /// RUN-CONTINUOUS
    RunContinuous                                                          = 1535,
    /// RUN-ONCE
    RunOnce                                                                = 2768,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 2612,
    /// RUNNABLE-ENTITY-ACTIVATED
    RunnableEntityActivated                                                = 3,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 982,
    /// RUNNABLE-ENTITY-STARTED
    RunnableEntityStarted                                                  = 1914,
    /// RUNNABLE-ENTITY-TERMINATED
    RunnableEntityTerminated                                               = 480,
    /// RUNNABLE-ENTITY-VARIABLE-ACCESS
    RunnableEntityVariableAccess                                           = 1286,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 2561,
    /// RW
    Rw                                                                     = 893,
    /// RX-TRIGGER
    RxTrigger                                                              = 1944,
    /// SA
    Sa                                                                     = 243,
    /// SAE-J-1939--73
    SaeJ1939_73                                                            = 2903,
    /// SAE-J-2012--DA
    SaeJ2012Da                                                             = 2690,
    /// SAFETY
    Safety                                                                 = 877,
    /// SATURATE
    Saturate                                                               = 1079,
    /// SCHEDULE-VARIANT-1
    ScheduleVariant1                                                       = 1603,
    /// SCHEDULE-VARIANT-2
    ScheduleVariant2                                                       = 1213,
    /// SCHEDULE-VARIANT-3
    ScheduleVariant3                                                       = 612,
    /// SCHEDULE-VARIANT-4
    ScheduleVariant4                                                       = 1951,
    /// SCHEDULE-VARIANT-5
    ScheduleVariant5                                                       = 1289,
    /// SCHEDULE-VARIANT-6
    ScheduleVariant6                                                       = 779,
    /// SCHEDULE-VARIANT-7
    ScheduleVariant7                                                       = 860,
    /// SCHEDULED
    Scheduled                                                              = 1548,
    /// SD
    Sd                                                                     = 594,
    /// SDG-ABSTRACT-FOREIGN-REFERENCE
    SdgAbstractForeignReference                                            = 1654,
    /// SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE
    SdgAbstractPrimitiveAttribute                                          = 2787,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 810,
    /// SDG-ATTRIBUTE
    SdgAttribute                                                           = 1562,
    /// SDG-CAPTION
    SdgCaption                                                             = 137,
    /// SDG-CLASS
    SdgClass                                                               = 914,
    /// SDG-DEF
    SdgDef                                                                 = 766,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 2538,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 1394,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 1648,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 1233,
    /// SDG-REFERENCE
    SdgReference                                                           = 2040,
    /// SDG-TAILORING
    SdgTailoring                                                           = 1309,
    /// SEARCH-FOR-ALL
    SearchForAll                                                           = 1770,
    /// SEARCH-FOR-ALL-INSTANCES
    SearchForAllInstances                                                  = 890,
    /// SEARCH-FOR-ANY
    SearchForAny                                                           = 1492,
    /// SEARCH-FOR-ID
    SearchForId                                                            = 905,
    /// SEARCH-FOR-SPECIFIC-INSTANCE
    SearchForSpecificInstance                                              = 1422,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 1069,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 1302,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 520,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 50,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 136,
    /// SECOND-TO-FIRST
    SecondToFirst                                                          = 2057,
    /// SECONDARY-ECU
    SecondaryEcu                                                           = 2599,
    /// SECRET-SEED
    SecretSeed                                                             = 300,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 2088,
    /// SECURE-COM-PROPS
    SecureComProps                                                         = 843,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 1353,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 608,
    /// SECURE-COMMUNICATION-DEPLOYMENT
    SecureCommunicationDeployment                                          = 1913,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 1040,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 2381,
    /// SECURE-ON-BOARD-COMMUNICATION
    SecureOnBoardCommunication                                             = 2321,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 1003,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 2865,
    /// SECURED-PDU-HEADER-08-BIT
    SecuredPduHeader08Bit                                                  = 400,
    /// SECURED-PDU-HEADER-16-BIT
    SecuredPduHeader16Bit                                                  = 1849,
    /// SECURED-PDU-HEADER-32-BIT
    SecuredPduHeader32Bit                                                  = 2811,
    /// SECURITY
    Security                                                               = 1565,
    /// SECURITY-EVENT-AGGREGATION-FILTER
    SecurityEventAggregationFilter                                         = 513,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITION
    SecurityEventContextDataDefinition                                     = 448,
    /// SECURITY-EVENT-CONTEXT-DATA-ELEMENT
    SecurityEventContextDataElement                                        = 440,
    /// SECURITY-EVENT-CONTEXT-MAPPING
    SecurityEventContextMapping                                            = 2655,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 2281,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 25,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 2661,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 552,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 268,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 2648,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 767,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 2010,
    /// SECURITY-EVENT-ONE-EVERY-N-FILTER
    SecurityEventOneEveryNFilter                                           = 1346,
    /// SECURITY-EVENT-REPORT-INSTANCE-DEFINITION
    SecurityEventReportInstanceDefinition                                  = 1778,
    /// SECURITY-EVENT-REPORT-INSTANCE-VALUE
    SecurityEventReportInstanceValue                                       = 1109,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 1478,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 477,
    /// SECURITY-EVENT-STATE-FILTER
    SecurityEventStateFilter                                               = 2724,
    /// SECURITY-EVENT-THRESHOLD-FILTER
    SecurityEventThresholdFilter                                           = 1203,
    /// SELECTED
    Selected                                                               = 1847,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 1619,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 2662,
    /// SENT-TAGGED
    SentTagged                                                             = 266,
    /// SENT-UNTAGGED
    SentUntagged                                                           = 2134,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 593,
    /// SERIALIZER
    Serializer                                                             = 729,
    /// SERVER-AUTHENTICATE
    ServerAuthenticate                                                     = 2777,
    /// SERVER-CALL-POINT
    ServerCallPoint                                                        = 565,
    /// SERVER-COM-SPEC-PROPS
    ServerComSpecProps                                                     = 630,
    /// SERVER-DECRYPT
    ServerDecrypt                                                          = 402,
    /// SERVER-ENCRYPT
    ServerEncrypt                                                          = 1809,
    /// SERVER-MAC-GENERATE
    ServerMacGenerate                                                      = 187,
    /// SERVER-MAC-VERIFY
    ServerMacVerify                                                        = 1689,
    /// SERVER-VERIFY
    ServerVerify                                                           = 1547,
    /// SERVICE-DISCOVERY
    ServiceDiscovery                                                       = 1144,
    /// SERVICE-EVENT-DEPLOYMENT
    ServiceEventDeployment                                                 = 2385,
    /// SERVICE-FIELD-DEPLOYMENT
    ServiceFieldDeployment                                                 = 604,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 1607,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 577,
    /// SERVICE-INSTANCE-TO-MACHINE-MAPPING
    ServiceInstanceToMachineMapping                                        = 2550,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 1273,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 2406,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 506,
    /// SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToSwClusterDesignPortPrototypeMapping                   = 2164,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 653,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 1372,
    /// SERVICE-INTERFACE-DEPLOYMENT
    ServiceInterfaceDeployment                                             = 2072,
    /// SERVICE-INTERFACE-DEPLOYMENT-ELEMENT
    ServiceInterfaceDeploymentElement                                      = 1487,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING
    ServiceInterfaceElementMapping                                         = 251,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 2387,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 2847,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 2644,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 1396,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 360,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 2778,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 2750,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 2791,
    /// SERVICE-METHOD-DEPLOYMENT
    ServiceMethodDeployment                                                = 2901,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 2614,
    /// SERVICE-ONLY
    ServiceOnly                                                            = 107,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 1355,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 1970,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 531,
    /// SESSION-HANDLING-ACTIVE
    SessionHandlingActive                                                  = 2071,
    /// SESSION-HANDLING-INACTIVE
    SessionHandlingInactive                                                = 2670,
    /// SETTER
    Setter                                                                 = 2294,
    /// SG
    Sg                                                                     = 964,
    /// SH
    Sh                                                                     = 1584,
    /// SHARED
    Shared                                                                 = 1640,
    /// SHARED-VLAN-LEARNING
    SharedVlanLearning                                                     = 541,
    /// SHORT-HEADER
    ShortHeader                                                            = 2454,
    /// SHOW-ALIAS-NAME
    ShowAliasName                                                          = 985,
    /// SHOW-CATEGORY
    ShowCategory                                                           = 158,
    /// SHOW-CONTENT
    ShowContent                                                            = 1758,
    /// SHOW-LONG-NAME
    ShowLongName                                                           = 589,
    /// SHOW-NUMBER
    ShowNumber                                                             = 435,
    /// SHOW-PAGE
    ShowPage                                                               = 2136,
    /// SHOW-SEE
    ShowSee                                                                = 1819,
    /// SHOW-SHORT-NAME
    ShowShortName                                                          = 259,
    /// SHOW-TYPE
    ShowType                                                               = 2313,
    /// SI
    Si                                                                     = 1697,
    /// SIDES
    Sides                                                                  = 1540,
    /// SIGN
    Sign                                                                   = 814,
    /// SIGN-WITH-ORIGIN-AUTHENTICATION
    SignWithOriginAuthentication                                           = 2638,
    /// SIGNAL-BASED
    SignalBased                                                            = 2725,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 161,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 1384,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 34,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 1457,
    /// SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFireAndForgetMethodToISignalTriggeringMapping               = 418,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 2546,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 2366,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 445,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 1659,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 2743,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 1470,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 735,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 104,
    /// SIGNATURE
    Signature                                                              = 1160,
    /// SILENT
    Silent                                                                 = 443,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 1025,
    /// SINGLE
    Single                                                                 = 398,
    /// SINGLE-CORE-REENTRANT
    SingleCoreReentrant                                                    = 365,
    /// SINGLE-LANGUAGE-REFERRABLE
    SingleLanguageReferrable                                               = 2816,
    /// SINGLE-OCCURRENCE
    SingleOccurrence                                                       = 1441,
    /// SK
    Sk                                                                     = 1937,
    /// SL
    Sl                                                                     = 2117,
    /// SLAVE
    Slave                                                                  = 2876,
    /// SLAVE-ACTIVE
    SlaveActive                                                            = 1703,
    /// SLAVE-PASSIVE
    SlavePassive                                                           = 2417,
    /// SLOPPY
    Sloppy                                                                 = 517,
    /// SLOW-FLASHING-MODE
    SlowFlashingMode                                                       = 2365,
    /// SLP
    Slp                                                                    = 1710,
    /// SM
    Sm                                                                     = 2464,
    /// SM-INTERACTS-WITH-NM-MAPPING
    SmInteractsWithNmMapping                                               = 2855,
    /// SMPTE-338
    Smpte338                                                               = 644,
    /// SN
    Sn                                                                     = 2346,
    /// SO
    So                                                                     = 2432,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 2232,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 1206,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 1387,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 2857,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 318,
    /// SOFTWARE-ACTIVATION-DEPENDENCY
    SoftwareActivationDependency                                           = 2093,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 454,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 157,
    /// SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS
    SoftwareClusterDiagnosticDeploymentProps                               = 878,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 2755,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 123,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 2786,
    /// SOMEIP
    Someip                                                                 = 2636,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 1576,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 996,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 1058,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 573,
    /// SOMEIP-FIELD
    SomeipField                                                            = 1746,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 1711,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 859,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 197,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 461,
    /// SOMEIP-REMOTE-MULTICAST-CONFIG
    SomeipRemoteMulticastConfig                                            = 1443,
    /// SOMEIP-REMOTE-UNICAST-CONFIG
    SomeipRemoteUnicastConfig                                              = 2881,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 388,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 750,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 49,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 1199,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 1059,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 1863,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 165,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 1237,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 1832,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 2894,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 2359,
    /// SOVD-GATEWAY-INSTANTIATION
    SovdGatewayInstantiation                                               = 372,
    /// SOVD-MODULE-INSTANTIATION
    SovdModuleInstantiation                                                = 1641,
    /// SOVD-SERVER-INSTANTIATION
    SovdServerInstantiation                                                = 1882,
    /// SPEC-ELEMENT-REFERENCE
    SpecElementReference                                                   = 2914,
    /// SPEC-ELEMENT-SCOPE
    SpecElementScope                                                       = 354,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 1921,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 2860,
    /// SQ
    Sq                                                                     = 708,
    /// SR
    Sr                                                                     = 1230,
    /// SRGB
    Srgb                                                                   = 2260,
    /// SS
    Ss                                                                     = 1218,
    /// SSDP
    Ssdp                                                                   = 252,
    /// ST
    St                                                                     = 611,
    /// STACK-USAGE
    StackUsage                                                             = 1696,
    /// STANDARD
    Standard                                                               = 2190,
    /// STANDARD-PORT
    StandardPort                                                           = 21,
    /// START
    Start                                                                  = 1725,
    /// START-FROM-BEGINNING
    StartFromBeginning                                                     = 1347,
    /// START-ON-BOOT
    StartOnBoot                                                            = 1910,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 1397,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 680,
    /// STATE-CLIENT-INTERFACE
    StateClientInterface                                                   = 1455,
    /// STATE-DEPENDENT-FIREWALL
    StateDependentFirewall                                                 = 486,
    /// STATE-MANAGEMEN-PHM-ERROR-INTERFACE
    StateManagemenPhmErrorInterface                                        = 755,
    /// STATE-MANAGEMENT-ACTION-ITEM
    StateManagementActionItem                                              = 2821,
    /// STATE-MANAGEMENT-ACTION-LIST
    StateManagementActionList                                              = 2052,
    /// STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE
    StateManagementDiagTriggerInterface                                    = 1982,
    /// STATE-MANAGEMENT-EM-ERROR-INTERFACE
    StateManagementEmErrorInterface                                        = 2477,
    /// STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-ACTION-ITEM
    StateManagementEnterSuspendToRamActionItem                             = 679,
    /// STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-OS-ACTION-ITEM
    StateManagementEnterSuspendToRamOsActionItem                           = 2098,
    /// STATE-MANAGEMENT-ERROR-INTERFACE
    StateManagementErrorInterface                                          = 1629,
    /// STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE
    StateManagementFunctionGroupSwitchNotificationInterface                = 1753,
    /// STATE-MANAGEMENT-LEAVE-SUSPEND-TO-RAM-ACTION-ITEM
    StateManagementLeaveSuspendToRamActionItem                             = 986,
    /// STATE-MANAGEMENT-MODULE-INSTANTIATION
    StateManagementModuleInstantiation                                     = 694,
    /// STATE-MANAGEMENT-NM-ACTION-ITEM
    StateManagementNmActionItem                                            = 2909,
    /// STATE-MANAGEMENT-NOTIFICATION-INTERFACE
    StateManagementNotificationInterface                                   = 838,
    /// STATE-MANAGEMENT-PHM-ERROR-INTERFACE
    StateManagementPhmErrorInterface                                       = 760,
    /// STATE-MANAGEMENT-PORT-INTERFACE
    StateManagementPortInterface                                           = 2907,
    /// STATE-MANAGEMENT-REQUEST-ERROR
    StateManagementRequestError                                            = 53,
    /// STATE-MANAGEMENT-REQUEST-INTERFACE
    StateManagementRequestInterface                                        = 2804,
    /// STATE-MANAGEMENT-REQUEST-TRIGGER
    StateManagementRequestTrigger                                          = 539,
    /// STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM
    StateManagementSetFunctionGroupStateActionItem                         = 2519,
    /// STATE-MANAGEMENT-SLEEP-ACTION-ITEM
    StateManagementSleepActionItem                                         = 1439,
    /// STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM
    StateManagementStateMachineActionItem                                  = 1545,
    /// STATE-MANAGEMENT-STATE-NOTIFICATION
    StateManagementStateNotification                                       = 737,
    /// STATE-MANAGEMENT-STATE-REQUEST
    StateManagementStateRequest                                            = 2115,
    /// STATE-MANAGEMENT-SUSPEND-TO-RAM-ACTION-ITEM
    StateManagementSuspendToRamActionItem                                  = 792,
    /// STATE-MANAGEMENT-SYNC-ACTION-ITEM
    StateManagementSyncActionItem                                          = 560,
    /// STATE-MANAGEMENT-TRIGGER-INTERFACE
    StateManagementTriggerInterface                                        = 1121,
    /// STATIC-OR-DYNAMIC-PART-TRIGGER
    StaticOrDynamicPartTrigger                                             = 1221,
    /// STATIC-PART-TRIGGER
    StaticPartTrigger                                                      = 406,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 1166,
    /// STATUS-BIT-AGING-AND-DISPLACEMENT
    StatusBitAgingAndDisplacement                                          = 928,
    /// STATUS-BIT-NORMAL
    StatusBitNormal                                                        = 1319,
    /// STD
    Std                                                                    = 2892,
    /// STD-AXIS
    StdAxis                                                                = 2435,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 2647,
    /// STD_AXIS
    Stdaxis                                                                = 2817,
    /// STEADY
    Steady                                                                 = 1956,
    /// STIMULUS-SYNCHRONIZATION
    StimulusSynchronization                                                = 2204,
    /// STOP
    Stop                                                                   = 1504,
    /// STOP-TRIGGER
    StopTrigger                                                            = 2473,
    /// STORE-EVENT
    StoreEvent                                                             = 167,
    /// STORE-PERSISTENTLY
    StorePersistently                                                      = 169,
    /// STRICT-MODE
    StrictMode                                                             = 2069,
    /// STRICT-MONOTONOUS
    StrictMonotonous                                                       = 1429,
    /// STRICT-PRIORITY
    StrictPriority                                                         = 1052,
    /// STRICTLY-DECREASING
    StrictlyDecreasing                                                     = 554,
    /// STRICTLY-INCREASING
    StrictlyIncreasing                                                     = 566,
    /// STRING
    String                                                                 = 1762,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 1954,
    /// SU
    Su                                                                     = 753,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 1631,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 1216,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 904,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 2124,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 1156,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 2606,
    /// SUPPLIER
    Supplier                                                               = 27,
    /// SUPPORTS-BUFFER-LOCKING
    SupportsBufferLocking                                                  = 2706,
    /// SUSPEND-TO-RAM-AWARE
    SuspendToRamAware                                                      = 2145,
    /// SUSPEND-TO-RAM-HUB-INTERFACE
    SuspendToRamHubInterface                                               = 1100,
    /// SUSPEND-TO-RAM-HUB-MAPPING
    SuspendToRamHubMapping                                                 = 584,
    /// SUSPEND-TO-RAM-MODULE-INSTANTIATION
    SuspendToRamModuleInstantiation                                        = 2325,
    /// SUSPEND-TO-RAM-NOT-SUPPORTED
    SuspendToRamNotSupported                                               = 1263,
    /// SUSPEND-TO-RAM-SATELLITE-INTERFACE
    SuspendToRamSatelliteInterface                                         = 1594,
    /// SUSPEND-TO-RAM-SATELLITE-MAPPING
    SuspendToRamSatelliteMapping                                           = 1386,
    /// SUSPEND-TO-RAM-TOLERANT
    SuspendToRamTolerant                                                   = 2831,
    /// SV
    Sv                                                                     = 1325,
    /// SVG
    Svg                                                                    = 2086,
    /// SW
    Sw                                                                     = 1452,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 2161,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 2480,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 2634,
    /// SW-CALIBRATION-METHOD
    SwCalibrationMethod                                                    = 1480,
    /// SW-CALPRM-PROTOTYPE
    SwCalprmPrototype                                                      = 456,
    /// SW-CLASS-ATTR-IMPL
    SwClassAttrImpl                                                        = 2386,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 1046,
    /// SW-CLASS-PROTOTYPE
    SwClassPrototype                                                       = 82,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 2220,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 4,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 1854,
    /// SW-COMPONENT-TYPE
    SwComponentType                                                        = 2004,
    /// SW-CONNECTOR
    SwConnector                                                            = 1295,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 2218,
    /// SW-INSTANCE
    SwInstance                                                             = 1512,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 366,
    /// SW-MC-FRAME
    SwMcFrame                                                              = 1210,
    /// SW-MC-INTERFACE
    SwMcInterface                                                          = 2511,
    /// SW-MC-INTERFACE-SOURCE
    SwMcInterfaceSource                                                    = 28,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 87,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 2854,
    /// SW-SERVICE-PROTOTYPE
    SwServicePrototype                                                     = 2459,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 1103,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 1699,
    /// SW-VARIABLE-PROTOTYPE
    SwVariablePrototype                                                    = 2572,
    /// SWC
    Swc                                                                    = 1952,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 2640,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 693,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 1032,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 202,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 945,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 191,
    /// SWC-TIMING
    SwcTiming                                                              = 1410,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 1146,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 925,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 2372,
    /// SWITCH
    Switch                                                                 = 1919,
    /// SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY
    SwitchAsynchronousTrafficShaperGroupEntry                              = 2355,
    /// SWITCH-ATS-INSTANCE-ENTRY
    SwitchAtsInstanceEntry                                                 = 727,
    /// SWITCH-FLOW-METERING-ENTRY
    SwitchFlowMeteringEntry                                                = 1958,
    /// SWITCH-STREAM-FILTER-ACTION-DEST-PORT-MODIFICATION
    SwitchStreamFilterActionDestPortModification                           = 602,
    /// SWITCH-STREAM-FILTER-ENTRY
    SwitchStreamFilterEntry                                                = 1887,
    /// SWITCH-STREAM-FILTER-RULE
    SwitchStreamFilterRule                                                 = 1485,
    /// SWITCH-STREAM-GATE-ENTRY
    SwitchStreamGateEntry                                                  = 2277,
    /// SWITCH-STREAM-IDENTIFICATION
    SwitchStreamIdentification                                             = 425,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 1038,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 2121,
    /// SYMMETRIC
    Symmetric                                                              = 1026,
    /// SYMMETRIC-KEY
    SymmetricKey                                                           = 956,
    /// SYNC-BASE-TIME-MANAGER
    SyncBaseTimeManager                                                    = 1727,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 189,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 2439,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 1783,
    /// SYNCHRONIZED
    Synchronized                                                           = 2336,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 349,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 1266,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 56,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 2658,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 2383,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 1790,
    /// SYNCHRONOUS
    Synchronous                                                            = 2726,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 1189,
    /// SYSTEM
    System                                                                 = 946,
    /// SYSTEM-COM-SPEC-DEFINITION-SET
    SystemComSpecDefinitionSet                                             = 1123,
    /// SYSTEM-DESIGN-TIME
    SystemDesignTime                                                       = 358,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 1593,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 1877,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 1568,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 581,
    /// SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalGroupToCommunicationResourceMapping                        = 1821,
    /// SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalToCommunicationResourceMapping                             = 1816,
    /// SYSTEM-SUPPLIER-BOOT
    SystemSupplierBoot                                                     = 1467,
    /// SYSTEM-SUPPLIER-BOOT-RESP-APP
    SystemSupplierBootRespApp                                              = 1590,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 736,
    /// TA
    Ta                                                                     = 2049,
    /// TARGET-CONTAINER
    TargetContainer                                                        = 2111,
    /// TASK
    Task                                                                   = 691,
    /// TC
    Tc                                                                     = 254,
    /// TCP
    Tcp                                                                    = 1599,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 1650,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 2616,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 2039,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 1950,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 31,
    /// TD-EVENT-BSW
    TdEventBsw                                                             = 2023,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 1066,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 2872,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 2066,
    /// TD-EVENT-COM
    TdEventCom                                                             = 2812,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 1597,
    /// TD-EVENT-CYCLE-START
    TdEventCycleStart                                                      = 1380,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 867,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 2562,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 93,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 66,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 2392,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 128,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 2083,
    /// TD-EVENT-SERVICE-INSTANCE
    TdEventServiceInstance                                                 = 350,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 2600,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 1756,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 2344,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 2012,
    /// TD-EVENT-SLLET
    TdEventSllet                                                           = 1215,
    /// TD-EVENT-SLLET-PORT
    TdEventSlletPort                                                       = 2194,
    /// TD-EVENT-SWC
    TdEventSwc                                                             = 1028,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 1976,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 210,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 2362,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 2005,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 1030,
    /// TD-EVENT-VFB
    TdEventVfb                                                             = 2714,
    /// TD-EVENT-VFB-PORT
    TdEventVfbPort                                                         = 1454,
    /// TD-EVENT-VFB-PORT-GROUP
    TdEventVfbPortGroup                                                    = 1239,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 685,
    /// TDLET-ZONE-CLOCK
    TdletZoneClock                                                         = 1532,
    /// TE
    Te                                                                     = 2654,
    /// TERMINATE
    Terminate                                                              = 1893,
    /// TEST-FAILED
    TestFailed                                                             = 1327,
    /// TEST-FAILED-BIT
    TestFailedBit                                                          = 1577,
    /// TEST-FAILED-THIS-OPERATION-CYCLE
    TestFailedThisOperationCycle                                           = 2656,
    /// TEST-PASSED
    TestPassed                                                             = 1164,
    /// TESTED
    Tested                                                                 = 1445,
    /// TESTED-AND-FAILED
    TestedAndFailed                                                        = 1938,
    /// TG
    Tg                                                                     = 359,
    /// TH
    Th                                                                     = 948,
    /// TI
    Ti                                                                     = 1225,
    /// TIFF
    Tiff                                                                   = 1653,
    /// TIME
    Time                                                                   = 1047,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 1427,
    /// TIME-BASE-RESOURCE
    TimeBaseResource                                                       = 1004,
    /// TIME-MASTER
    TimeMaster                                                             = 1433,
    /// TIME-SLAVE
    TimeSlave                                                              = 2513,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 1097,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 2106,
    /// TIME-SYNC-SERVER-CONFIGURATION
    TimeSyncServerConfiguration                                            = 1365,
    /// TIME-SYNCHRONIZATION-INTERFACE
    TimeSynchronizationInterface                                           = 1896,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 719,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 1652,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 2258,
    /// TIMEOUT
    Timeout                                                                = 2843,
    /// TIMING-CLOCK
    TimingClock                                                            = 2490,
    /// TIMING-CLOCK-SYNC-ACCURACY
    TimingClockSyncAccuracy                                                = 2797,
    /// TIMING-CONDITION
    TimingCondition                                                        = 607,
    /// TIMING-CONSTRAINT
    TimingConstraint                                                       = 2384,
    /// TIMING-DESCRIPTION
    TimingDescription                                                      = 2302,
    /// TIMING-DESCRIPTION-EVENT
    TimingDescriptionEvent                                                 = 1820,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 1516,
    /// TIMING-EVENT
    TimingEvent                                                            = 1114,
    /// TIMING-EXTENSION
    TimingExtension                                                        = 934,
    /// TIMING-EXTENSION-RESOURCE
    TimingExtensionResource                                                = 568,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 1036,
    /// TIP
    Tip                                                                    = 324,
    /// TK
    Tk                                                                     = 1515,
    /// TL
    Tl                                                                     = 1440,
    /// TLS-12
    Tls12                                                                  = 1246,
    /// TLS-13
    Tls13                                                                  = 1102,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 469,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 1195,
    /// TLS-CRYPTO-CIPHER-SUITE-PROPS
    TlsCryptoCipherSuiteProps                                              = 2349,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 1975,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 528,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 1326,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 1544,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 1209,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 337,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 233,
    /// TN
    Tn                                                                     = 1701,
    /// TO
    To                                                                     = 1831,
    /// TOP
    Top                                                                    = 348,
    /// TOPBOT
    Topbot                                                                 = 865,
    /// TOPIC
    Topic                                                                  = 361,
    /// TOPIC-1
    Topic1                                                                 = 1138,
    /// TOPIC-PREFIX
    TopicPrefix                                                            = 504,
    /// TP-ADDRESS
    TpAddress                                                              = 294,
    /// TP-CONFIG
    TpConfig                                                               = 969,
    /// TP-CONNECTION-IDENT
    TpConnectionIdent                                                      = 672,
    /// TR
    Tr                                                                     = 2803,
    /// TRACE
    Trace                                                                  = 1420,
    /// TRACE-REFERRABLE
    TraceReferrable                                                        = 2852,
    /// TRACE-SWITCH-ARTI
    TraceSwitchArti                                                        = 1788,
    /// TRACE-SWITCH-ARTI-AND-LOG
    TraceSwitchArtiAndLog                                                  = 1795,
    /// TRACE-SWITCH-CONFIG
    TraceSwitchConfig                                                      = 2144,
    /// TRACE-SWITCH-LOG
    TraceSwitchLog                                                         = 343,
    /// TRACE-SWITCH-NONE
    TraceSwitchNone                                                        = 977,
    /// TRACEABLE
    Traceable                                                              = 298,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 5,
    /// TRACEABLE-TEXT
    TraceableText                                                          = 912,
    /// TRACED-FAILURE
    TracedFailure                                                          = 368,
    /// TRANSFER
    Transfer                                                               = 993,
    /// TRANSFORMATION-I-SIGNAL-PROPS-IDENT
    TransformationISignalPropsIdent                                        = 983,
    /// TRANSFORMATION-PROPS
    TransformationProps                                                    = 2118,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 260,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 2810,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 1241,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 1818,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 2676,
    /// TRANSFORMER-ERROR-HANDLING
    TransformerErrorHandling                                               = 2373,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 1388,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 512,
    /// TRANSFORMING-I-SIGNAL
    TransformingISignal                                                    = 2358,
    /// TRANSIENT
    Transient                                                              = 2527,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 1891,
    /// TRANSIENT-LOCAL
    TransientLocal                                                         = 2632,
    /// TRANSLATION-START
    TranslationStart                                                       = 1494,
    /// TRANSPORT
    Transport                                                              = 1298,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 972,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 1709,
    /// TRAP
    Trap                                                                   = 826,
    /// TRIGGER
    Trigger                                                                = 18,
    /// TRIGGER-ACTIVATED
    TriggerActivated                                                       = 1934,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 2660,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 2460,
    /// TRIGGER-RELEASED
    TriggerReleased                                                        = 1610,
    /// TRIGGER-UNICAST
    TriggerUnicast                                                         = 2790,
    /// TRIGGERED
    Triggered                                                              = 2036,
    /// TRIGGERED-ON-CHANGE
    TriggeredOnChange                                                      = 110,
    /// TRIGGERED-ON-CHANGE-WITHOUT-REPETITION
    TriggeredOnChangeWithoutRepetition                                     = 1261,
    /// TRIGGERED-ON-EVALUATION
    TriggeredOnEvaluation                                                  = 1348,
    /// TRIGGERED-WITHOUT-REPETITION
    TriggeredWithoutRepetition                                             = 2826,
    /// TRUE
    True                                                                   = 231,
    /// TS
    Ts                                                                     = 35,
    /// TT
    Tt                                                                     = 850,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 1108,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 431,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 1112,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 2182,
    /// TUNNEL
    Tunnel                                                                 = 2751,
    /// TW
    Tw                                                                     = 824,
    /// TX-REF-TRIGGER
    TxRefTrigger                                                           = 185,
    /// TX-REF-TRIGGER-GAP
    TxRefTriggerGap                                                        = 1204,
    /// TX-TRIGGER-MERGED
    TxTriggerMerged                                                        = 728,
    /// TX-TRIGGER-SINGLE
    TxTriggerSingle                                                        = 449,
    /// UCM
    Ucm                                                                    = 184,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 1139,
    /// UCM-MASTER
    UcmMaster                                                              = 1403,
    /// UCM-MASTER-MODULE-INSTANTIATION
    UcmMasterModuleInstantiation                                           = 89,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 2548,
    /// UCM-RETRY-STRATEGY
    UcmRetryStrategy                                                       = 52,
    /// UCM-STEP
    UcmStep                                                                = 1214,
    /// UCM-SUBORDINATE-MODULE-INSTANTIATION
    UcmSubordinateModuleInstantiation                                      = 2328,
    /// UCM-TO-TIME-BASE-RESOURCE-MAPPING
    UcmToTimeBaseResourceMapping                                           = 2601,
    /// UDP
    Udp                                                                    = 397,
    /// UDP-CHECKSUM-DISABLED
    UdpChecksumDisabled                                                    = 1382,
    /// UDP-CHECKSUM-ENABLED
    UdpChecksumEnabled                                                     = 1825,
    /// UDP-NM
    UdpNm                                                                  = 1902,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 2283,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 1409,
    /// UDS
    Uds                                                                    = 485,
    /// UK
    Uk                                                                     = 1864,
    /// UNDECIDED
    Undecided                                                              = 1408,
    /// UNDEFINED
    Undefined                                                              = 1081,
    /// UNIT
    Unit                                                                   = 1683,
    /// UNIT-GROUP
    UnitGroup                                                              = 1099,
    /// UNNUMBER
    Unnumber                                                               = 546,
    /// UNSPECIFIED
    Unspecified                                                            = 351,
    /// UP-LINK-PORT
    UpLinkPort                                                             = 1519,
    /// UPDATE
    Update                                                                 = 325,
    /// UPDATE-CONFIGURATION
    UpdateConfiguration                                                    = 1858,
    /// UPLOADABLE-DEPLOYMENT-ELEMENT
    UploadableDeploymentElement                                            = 122,
    /// UPLOADABLE-DESIGN-ELEMENT
    UploadableDesignElement                                                = 1414,
    /// UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT
    UploadableExclusivePackageElement                                      = 744,
    /// UPLOADABLE-PACKAGE-ELEMENT
    UploadablePackageElement                                               = 125,
    /// UR
    Ur                                                                     = 290,
    /// URI-DESCRIPTION
    UriDescription                                                         = 462,
    /// USE-ARGUMENT-TYPE
    UseArgumentType                                                        = 615,
    /// USE-ARRAY-BASE-TYPE
    UseArrayBaseType                                                       = 1106,
    /// USE-FIRST-CONTEXT-DATA
    UseFirstContextData                                                    = 1208,
    /// USE-LAST-CONTEXT-DATA
    UseLastContextData                                                     = 374,
    /// USE-VOID
    UseVoid                                                                = 582,
    /// USER
    User                                                                   = 598,
    /// USER-DEFINED
    UserDefined                                                            = 1354,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 307,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 2186,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 1191,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 345,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 800,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 654,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 1634,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 715,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 1398,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 910,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 1734,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 2539,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 239,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 894,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 748,
    /// USES-LOGGING
    UsesLogging                                                            = 2752,
    /// UZ
    Uz                                                                     = 1769,
    /// V-2-X-ACTIVE-SUPPORTED
    V2XActiveSupported                                                     = 895,
    /// V-2-X-DATA-MANAGER-NEEDS
    V2XDataManagerNeeds                                                    = 1395,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 2152,
    /// V-2-X-FACILITIES
    V2XFacilities                                                          = 453,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 1037,
    /// V-2-X-MANAGEMENT
    V2XManagement                                                          = 2498,
    /// V-2-X-NOT-SUPPORTED
    V2XNotSupported                                                        = 2596,
    /// VALID
    Valid                                                                  = 2399,
    /// VAR
    Var                                                                    = 1732,
    /// VAR-FAST
    VarFast                                                                = 872,
    /// VAR-NO-INIT
    VarNoInit                                                              = 1415,
    /// VAR-POWER-ON-INIT
    VarPowerOnInit                                                         = 543,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 2875,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 344,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 1981,
    /// VARIABLE-DATA-PROTOTYPE-RECEIVED
    VariableDataPrototypeReceived                                          = 903,
    /// VARIABLE-DATA-PROTOTYPE-SENT
    VariableDataPrototypeSent                                              = 2188,
    /// VARIABLE-SIZE
    VariableSize                                                           = 603,
    /// VARIANT-LINK-TIME
    VariantLinkTime                                                        = 287,
    /// VARIANT-POST-BUILD
    VariantPostBuild                                                       = 2759,
    /// VARIANT-POST-BUILD-LOADABLE
    VariantPostBuildLoadable                                               = 2913,
    /// VARIANT-POST-BUILD-SELECTABLE
    VariantPostBuildSelectable                                             = 182,
    /// VARIANT-PRE-COMPILE
    VariantPreCompile                                                      = 962,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 407,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 2604,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 2354,
    /// VENDOR
    Vendor                                                                 = 222,
    /// VENDOR-SPECIFIC
    VendorSpecific                                                         = 823,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 562,
    /// VERBOSE
    Verbose                                                                = 1867,
    /// VERIFICATION
    Verification                                                           = 2563,
    /// VERIFY
    Verify                                                                 = 2368,
    /// VERSION-1
    Version1                                                               = 1686,
    /// VERTEX-OF-TARGET-CONTAINER
    VertexOfTargetContainer                                                = 2701,
    /// VFB-TIMING
    VfbTiming                                                              = 1513,
    /// VI
    Vi                                                                     = 39,
    /// VIDEO-FRAME
    VideoFrame                                                             = 1553,
    /// VIDEO-LINE
    VideoLine                                                              = 385,
    /// VIEW-MAP
    ViewMap                                                                = 2,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 2758,
    /// VLAN-CONFIG
    VlanConfig                                                             = 209,
    /// VO
    Vo                                                                     = 1187,
    /// VOLATILE
    Volatile                                                               = 1133,
    /// WAIT-FOR-VEHICLE-SAFE-STATE
    WaitForVehicleSafeState                                                = 2824,
    /// WAIT-POINT
    WaitPoint                                                              = 2593,
    /// WAIT-TIME-DATE
    WaitTimeDate                                                           = 1000,
    /// WARMUP
    Warmup                                                                 = 140,
    /// WARN
    Warn                                                                   = 195,
    /// WARNING
    Warning                                                                = 537,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 798,
    /// WATCH-DOG-MANAGER
    WatchDogManager                                                        = 1608,
    /// WATCH-TRIGGER
    WatchTrigger                                                           = 23,
    /// WATCH-TRIGGER-GAP
    WatchTriggerGap                                                        = 2897,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 2730,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 2908,
    /// WEIGHTED-ROUND-ROBIN
    WeightedRoundRobin                                                     = 422,
    /// WILL-CALL
    WillCall                                                               = 495,
    /// WILL-RECEIVE
    WillReceive                                                            = 2899,
    /// WILL-SEND
    WillSend                                                               = 911,
    /// WO
    Wo                                                                     = 1464,
    /// WONT-CALL
    WontCall                                                               = 2017,
    /// WONT-RECEIVE
    WontReceive                                                            = 567,
    /// WONT-SEND
    WontSend                                                               = 931,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 1855,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 1990,
    /// WRITE
    Write                                                                  = 2906,
    /// WRITE-ONLY
    WriteOnly                                                              = 2718,
    /// WRITE-ONLY-ONCE
    WriteOnlyOnce                                                          = 2329,
    /// WRONG-TRIGGER
    WrongTrigger                                                           = 2747,
    /// X-509
    X509                                                                   = 1269,
    /// X-MII
    XMii                                                                   = 2391,
    /// X-MMI
    XMmi                                                                   = 2253,
    /// XCP
    Xcp                                                                    = 1023,
    /// XCP-PDU
    XcpPdu                                                                 = 481,
    /// XDOC
    Xdoc                                                                   = 1628,
    /// XFILE
    Xfile                                                                  = 645,
    /// XG-MII
    XgMii                                                                  = 472,
    /// XH
    Xh                                                                     = 2683,
    /// XOR
    Xor                                                                    = 1743,
    /// XREF-TARGET
    XrefTarget                                                             = 2476,
    /// XXG-MII
    XxgMii                                                                 = 2501,
    /// XYZ
    Xyz                                                                    = 2645,
    /// YCBCR
    Ycbcr                                                                  = 16,
    /// YCGCO
    Ycgco                                                                  = 426,
    /// YCM
    Ycm                                                                    = 496,
    /// YO
    Yo                                                                     = 2749,
    /// ZH
    Zh                                                                     = 712,
    /// ZU
    Zu                                                                     = 1091,
    /// default
    default                                                                = 192,
    /// preserve
    preserve                                                               = 2337,
}

impl EnumItem {
    #[rustfmt::skip]
    const STRING_TABLE: [&'static str; 2915] = ["J-1850", "ROUTER-ADVERTISEMENT", "VIEW-MAP", "RUNNABLE-ENTITY-ACTIVATED", "SW-COMPONENT-MAPPING-CONSTRAINTS", "TRACEABLE-TABLE", "CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "BSW-MODULE-DESCRIPTION", "PLATFORM-MODULE-ENDPOINT-CONFIGURATION", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "GLOBAL-TIME-GATEWAY", "CALIBRATION-OFFLINE", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "COMMAND-LINE-SIMPLE-FORM", "IS-STOPPED", "NO-NEWLINE", "YCBCR", "CAN-FD", "TRIGGER", "FIXED", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "STANDARD-PORT", "EXACT-OR-ANY-MINOR-VERSION", "WATCH-TRIGGER", "REF-ALL", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "BINARY-MANIFEST-ADDRESSABLE-OBJECT", "SUPPLIER", "SW-MC-INTERFACE-SOURCE", "REPORT", "PS", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "48-KHZ", "PERIODIC-RATE-MEDIUM", "SIGNAL-BASED-FIELD-DEPLOYMENT", "TS", "LIN-FRAME-TRIGGERING", "DIAGNOSTIC-SOVD-SERVICE-INSTANCE", "AR--CLIENT--SERVER", "VI", "ENABLED", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "APMC-FLOAT-PARAM-DEF", "IS-GREATER-THAN-OR-EQUAL", "PARTITION", "EXTERNAL-REPLACEMENT", "OTHER", "ONLY-THIS-CYCLE-AND-READINESS", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "SEC-OC-JOB-REQUIREMENT", "PERSISTENCY-INTERFACE-ELEMENT", "UCM-RETRY-STRATEGY", "STATE-MANAGEMENT-REQUEST-ERROR", "FM-FEATURE-MAP-ELEMENT", "OPERATION-INVOKED-EVENT", "SYNCHRONIZED-TIME-BASE-CONSUMER", "RPT-PROFILE", "DIAGNOSTIC-SOVD-CONTENT-PORT-MAPPING", "ALL-INDICES-DIFFERENT-ARRAY-SIZE", "OPERATION-CALL-RESPONSE-SENT", "CAN-CLUSTER", "IDSM-RATE-LIMITATION", "BSW-M-ENTRY-CALL-RETURNED", "ECU-TIMING", "LOW", "TD-EVENT-I-PDU", "MAC-SEC-KAY-PARTICIPANT", "NOT-VALID", "CAN-TP-NODE", "CLIENT-ID-DEFINITION", "FRAME-PORT", "MALFUNCTION", "88-2-KHZ", "DIAGNOSTIC-TROUBLE-CODE", "ECU-PARTITION", "KS", "PERIODIC-EVENT-TRIGGERING", "APPLICATION-ONLY", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "SW-CLASS-PROTOTYPE", "J-19394", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "R-PORT-PROTOTYPE", "DZ", "SW-RECORD-LAYOUT", "EQUAL", "UCM-MASTER-MODULE-INSTANTIATION", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "KEEP-LAST", "CRYPTO-NEEDS", "TD-EVENT-FRAME-ETHERNET", "PRIMITIVE-ATTRIBUTE-TAILORING", "INDIVIDUAL", "CHAPTER", "PERSISTENCY-FILE", "BSW-EVENT", "ABSTRACT-SERVICE-INSTANCE", "INTERNAL-BEHAVIOR", "CRC-SUPPORTED", "ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED", "DIAGNOSTIC-SERVICE-TABLE", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "COMMAND-LINE-LONG-FORM", "CAPTION", "SERVICE-ONLY", "DO-IP-LOGICAL-ADDRESS", "RAPID-PROTOTYPING-SCENARIO", "TRIGGERED-ON-CHANGE", "APMC-IP-V4-ADDRESS-PARAM-VALUE", "LAST-MODE", "RELIABLE", "DIAGNOSTIC-SOVD-BULK-DATA", "CLIENT-ENCRYPT", "CONSTANT-SPECIFICATION", "DATA-TRANSFORMATION", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "GRANT-DESIGN", "CYCLE-REPETITION-50", "PDU-ACTIVATION-ROUTING-GROUP", "UPLOADABLE-DEPLOYMENT-ELEMENT", "SOFTWARE-PACKAGE", "RESET-VM", "UPLOADABLE-PACKAGE-ELEMENT", "ASYNCHRONOUS-SERVER-CALL-POINT", "I-PDU-TRIGGERING", "TD-EVENT-MODE-DECLARATION", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "OPERATION-CALLED", "DIAGNOSTIC-SOVD-GROUP", "NO-TRANSFORMER-ERROR-HANDLING", "DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE", "COLOR-BLIND", "BSW-CALLED-ENTITY", "SEC-OC-SECURE-COM-PROPS", "SDG-CAPTION", "MANUFACTURING", "EXPLICIT", "WARMUP", "COUPLING-PORT-SHAPER", "I-SIGNAL-TO-I-PDU-MAPPING", "CRYPTO-KEY-SLOT-USAGE-DESIGN-MAPPING", "GENERIC-DIAGNOSTIC-TRANSPORT-INSTANTIATION", "OBSERVER-BASED", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-PORT-MAPPING", "GLOBAL-TIME-FR-MASTER", "DHCPV-4", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "DIAGNOSTIC-SERVICE-CLASS", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "CRYPTO-SERVICE-QUEUE", "FUNCTION-GROUP-STATE-TO-NM-HANDLE", "READ", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "SOFTWARE-CLUSTER-DESIGN", "SHOW-CATEGORY", "END-2-END-EVENT-PROTECTION-PROPS", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "SIGNAL-BASED-EVENT-DEPLOYMENT", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE", "EVENT-TRIGGERING-CONSTRAINT", "NOT-EQUAL", "SOMEIP-SERVICE-INTERFACE", "DIAGNOSTIC-FIM-ALIAS-EVENT", "STORE-EVENT", "ETHERNET-NETWORK-CONFIGURATION", "STORE-PERSISTENTLY", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "DIAGNOSTIC-SESSION", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "PERIODIC-RATE-SLOW", "ETHERNET-FRAME", "KU", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "AFTER-SALES", "APMC-URI-INSTANCE-REFERENCE-VALUE", "GET", "ECUC-ADD-INFO-PARAM-DEF", "PLAIN", "VARIANT-POST-BUILD-SELECTABLE", "DIAGNOSTIC-DATA-BY-IDENTIFIER", "UCM", "TX-REF-TRIGGER", "PRIVATE-KEY", "SERVER-MAC-GENERATE", "DEFAULT-TRACE-STATE-ENABLED", "SYNC-TIME-BASE-MGR-USER-NEEDS", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "SWC-SERVICE-DEPENDENCY", "default", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "MODE-SWITCHED-ACK-EVENT", "WARN", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "SOMEIP-METHOD-DEPLOYMENT", "J-19399", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "J-1939-CLUSTER", "COMMUNICATION-INTER-ECU", "SWC-MODE-MANAGER-ERROR-EVENT", "GENERAL-PURPOSE-CONNECTION", "ECUC-ABSTRACT-REFERENCE-DEF", "ROTATE-90-CCW-LIMIT-TO-TEXT", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "DIAGNOSTIC-EXTENDED-DATA-RECORD-INTERFACE", "MODE-SWITCH-POINT", "VLAN-CONFIG", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "HW-CATEGORY", "DATA-RECEIVE-ERROR-EVENT", "APMC-ABSTRACT-RESTRICTED-TEXTUAL-PARAM-VALUE", "POST", "FM-ATTRIBUTE-DEF", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "DIAGNOSTIC-SOVD-DATA-CATEGORY", "BSW-MODULE-ENTRY", "GRAYSCALE", "PROVIDED-AP-SERVICE-INSTANCE", "ABSTRACT-DO-IP-PORT-MAPPING", "VENDOR", "ETHERNET-PHYSICAL-CHANNEL", "ALL-SUPPORTED-DTCS", "ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED", "44-1-KHZ", "PENDING", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "ALL-PARTIAL-NETWORKS-ACTIVE", "IS-LESS-OR-EQUAL", "TRUE", "DIAGNOSTIC-EXTENDED-DATA-RECORD-CLIENT-PORT-MAPPING", "TLV-DATA-ID-DEFINITION-SET", "APMC-ABSTRACT-RESTRICTED-STRING-PARAM-DEF", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "150", "CALIBRATION-VARIABLES", "ATP-BLUEPRINTABLE", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "CONSISTENCY-MECHANISM-REQUIRED", "APPLICATION-DATA-TYPE", "FDBAMCMDT", "SA", "PROTECT-LAMP", "DIAGNOSTIC-DO-IP-POWER-MODE-PORT-MAPPING", "DESCENDANT", "NM-ECU", "DEFAULT-TRACE-STATE-DISABLED", "ABSTRACT-SUSPEND-TO-RAM-MAPPING", "4-4-4-4", "SERVICE-INTERFACE-ELEMENT-MAPPING", "SSDP", "FIELD-MAPPING", "TC", "ATOMIC-SW-COMPONENT-TYPE", "MOST-SIGNIFICANT-BYTE-LAST", "ECUC-COMMON-ATTRIBUTES", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE", "SHOW-SHORT-NAME", "TRANSFORMATION-PROPS-SET", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE", "NO-TRUSTED-PLATFORM-SUPPORT", "REST-ABSTRACT-ENDPOINT", "DIAGNOSTIC-CONTROL-NEEDS", "DIAGNOSTIC-SOVD-CONTENT-ELEMENT", "SENT-TAGGED", "4-2-0", "SECURITY-EVENT-CONTEXT-PROPS", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "DDS-PROVIDED-SERVICE-INSTANCE", "176-4-KHZ", "APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE", "AGGREGATION-TAILORING", "FIX-AXIS", "DIAGNOSTIC-ROUTINE-CONTROL", "DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING", "DLT-MESSAGE", "FLEXRAY-TP-CONNECTION-CONTROL", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "1-8", "RESOURCE-CONSUMPTION", "APMC-ENUMERATION-LITERAL-DEF", "CRYPTO-KEY-MANAGEMENT", "RESET-MACHINE", "REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE", "VARIANT-LINK-TIME", "CYCLE-REPETITION-4", "MASEKD-NEW-EQUALS-MASKED-OLD", "UR", "PACKAGEABLE-ELEMENT", "PUT", "ROUGH-ESTIMATE-STACK-USAGE", "TP-ADDRESS", "ADAPTIVE-FIELD-GETTER-CALLED", "LEFT", "REBOOT", "TRACEABLE", "APPLICATION-ACTION-ITEM", "SECRET-SEED", "LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING", "PLATFORM-PHM-ACTION-ITEM", "300", "ABSTRACT-SUSPEND-TO-RAM-INTERFACE", "COUPLING-PORT-FIFO", "DLT-CONTEXT", "USER-DEFINED-CLUSTER", "MASKED-NEW-EQUALS-MASKED-OLD", "EXTERNAL-TRIGGERING-POINT-IDENT", "PA", "PERSISTENCY-DEPLOYMENT-ELEMENT", "ESP", "RESET-ECU", "OFF", "REF-NON-STANDARD", "PERSISTENT", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "ANY-STANDARDIZED", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "BSW-DIRECT-CALL-POINT", "LIMIT-TO-TEXT", "DIAGNOSTIC-INFO-TYPE", "TIP", "UPDATE", "DIAGNOSTIC-STOP-ROUTINE", "NO-RETURN-VALUE-PROVIDED", "DIAGNOSTIC-SECURITY-ACCESS", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "-500-MILES", "MODE-DECLARATION-SWITCH-COMPLETED", "IDS-MGR-NEEDS", "NO-CHECKPOINT-SUPERVISION", "CRYPTO-SIGNATURE-SCHEME", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "COM-OFFER-SERVICE-GRANT-DESIGN", "TLS-SECURE-COM-PROPS", "CLEAR-ALL-DTCS", "J-1587", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "REFERENCE-TAILORING", "TRACE-SWITCH-LOG", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "USER-DEFINED-ETHERNET-FRAME", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "FLEXRAY-NM-NODE", "TOP", "SYNCHRONIZED-MASTER-TIME-BASE", "TD-EVENT-SERVICE-INSTANCE", "UNSPECIFIED", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "200", "SPEC-ELEMENT-SCOPE", "APMC-STRING-PARAM-DEF", "DEBUG", "HR", "SYSTEM-DESIGN-TIME", "TG", "SERVICE-INTERFACE-MAPPING-SET", "TOPIC", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "CRYPTO-KEY-SLOT-TO-CLIENT-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "SINGLE-CORE-REENTRANT", "SW-MC-BASE-TYPE", "ACK-WITHOUT-RT", "TRACED-FAILURE", "DIAGNOSTIC-AUTH-ROLE", "CYCLE-REPETITION-20", "AR-ELEMENT", "SOVD-GATEWAY-INSTANTIATION", "NETWORK-MANAGEMENT-PORT-INTERFACE", "USE-LAST-CONTEXT-DATA", "OS-TASK-PROXY", "RAW-DATA-STREAM-CLIENT-INTERFACE", "FRAME-QUEUED-FOR-TRANSMISSION", "LIN-FRAME", "CRYPTO-SERVICE-PRIMITIVE", "I-SIGNAL-TRIGGERING", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY", "COUPLING-ELEMENT", "INDICATOR-STATUS-NEEDS", "PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE", "VIDEO-LINE", "ON-CHANGE-OF-DATA-IDENTIFIER", "CYCLE-REPETITION-40", "SOMEIP-REQUIRED-EVENT-GROUP", "MINIMUM-MINOR-VERSION", "AP-APPLICATION-ENDPOINT", "ADAPTIVE-METHOD-RESPONSE-SENT", "APPLICATION-COMPOSITE-DATA-TYPE", "MC-GROUP", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "E-2-E-PROFILE-CONFIGURATION", "INTERNAL-TRIGGER-OCCURRED-EVENT", "UDP", "SINGLE", "END-2-END-METHOD-PROTECTION-PROPS", "SECURED-PDU-HEADER-08-BIT", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "SERVER-DECRYPT", "DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE", "DLT-APPLICATION", "CRYPTO-DRIVER", "STATIC-PART-TRIGGER", "VARIATION-POINT-PROXY", "20", "ICV-NOT-SUPPORTED", "I-PDU-RECEIVED-BY-COM", "CYCLE-REPETITION-5", "CONSUMER", "ISO-15031--6", "PUBLIC-KEY", "DIAGNOSTIC-CLEAR-CONDITION", "DEBOUNCE-DATA", "ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF", "SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "DO-IP-ACTIVATION-LINE-NEEDS", "CYCLIC-HANDLING-COM-DATA-TO-OS-TASK-PROXY-MAPPING", "REST-STRING-PROPERTY-DEF", "WEIGHTED-ROUND-ROBIN", "APMC-FOREIGN-REFERENCE-DEF", "J-19395", "SWITCH-STREAM-IDENTIFICATION", "YCGCO", "REFERRABLE", "NOT-AVAILABLE", "FM-FEATURE-RELATION", "RESTART-APPLICATION", "TTCAN-COMMUNICATION-CONNECTOR", "ADAPTIVE-PLATFORM-SERVICE-INSTANCE", "INFO", "PROCESSOR", "SHOW-NUMBER", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "C", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "IP-SEC-RULE", "SECURITY-EVENT-CONTEXT-DATA-ELEMENT", "NON-VOLATILE", "FIRE-AND-FORGET-MAPPING", "SILENT", "RECOVERY-VIA-APPLICATION-ACTION", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "IDSM-QUALIFIED-EVENT-RECEIVER-MAPPING", "HI", "SECURITY-EVENT-CONTEXT-DATA-DEFINITION", "TX-TRIGGER-SINGLE", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "PORT-BLUEPRINT", "IDSM-REPORTING-MODE-PROVIDER-MAPPING", "V-2-X-FACILITIES", "SOFTWARE-CLUSTER", "COM-AXIS", "SW-CALPRM-PROTOTYPE", "DISABLE", "FA", "DEPENDENCY-ON-ARTIFACT", "GIF", "SOMEIP-PROVIDED-EVENT-GROUP", "URI-DESCRIPTION", "DIAGNOSTIC-EXTENDED-DATA-RECORD-ELEMENT", "RPT-LEVEL-2", "LAST-FAILED", "ECU", "COLLECTABLE-ELEMENT", "END-TO-END-PROTECTION-SET", "TLS-CONNECTION-GROUP", "POWER-WINDOW-TIME", "POWER", "XG-MII", "DDS-SERVICE-INSTANCE-OPERATION-CP", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "LATENCY-TIMING-CONSTRAINT", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-DIAGNOSTIC-EVENT-MAPPING", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "OPERATING-SYSTEM", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "RUNNABLE-ENTITY-TERMINATED", "XCP-PDU", "NO-PROTECTION", "PHM-SUPERVISED-ENTITY-INTERFACE", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "UDS", "STATE-DEPENDENT-FIREWALL", "MANUAL-BY-TOPIC", "INT-24-BIT", "PERSISTENCY-KEY-VALUE-PAIR", "ENHANCED", "BY-SOURCE-TIMESTAMP", "ACCEPT-CONFIGURED", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "ALWAYS", "WILL-CALL", "YCM", "DO-IP-INSTANTIATION", "APPLICATION-RECORD-DATA-TYPE", "PHYSICAL-DIMENSION-MAPPING-SET", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "BLUEPRINT-MAPPING-SET", "REST-PRIMITIVE-PROPERTY-DEF", "DOES-NOT-REPORT-EXECUTION-STATE", "TOPIC-PREFIX", "PERSISTENCY-INTERFACE", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "NO-CONSISTENCY-MECHANISM", "DLT-APPLICATION-TO-PROCESS-MAPPING", "I-SIGNAL-I-PDU", "BURST-PATTERN-EVENT-TRIGGERING", "TRANSFORMER-STATUS-FORWARDING", "SECURITY-EVENT-AGGREGATION-FILTER", "DIAGNOSTIC-ROUTINE-NEEDS", "DIAGNOSTIC-IO-CONTROL-CLASS", "CLASS-CONTENT-CONDITIONAL", "SLOPPY", "RPT-EXECUTABLE-ENTITY-EVENT", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "SEC-OC-JOB-MAPPING", "IPSEC", "PARAMETER-ACCESS", "IEEE-1722-TP-IIDC-CONNECTION", "ECU-MAPPING", "CONFIDENTIALITY-OFFSET--50", "ETH-TP-CONFIG", "PRE-COMPILE", "TLS-DEPLOYMENT", "BRIEF", "OBD-RATIO-SERVICE-NEEDS", "SERVICE-TIMING", "FR", "INVALID", "READ-ONLY", "CURVE_AXIS", "FIREWALL-STATE-SWITCH-INTERFACE", "WARNING", "CRYPTO-INTERFACE", "STATE-MANAGEMENT-REQUEST-TRIGGER", "ARTIFACT-LOCATOR", "SHARED-VLAN-LEARNING", "PROCESSING-STYLE-SYNCHRONOUS", "VAR-POWER-ON-INIT", "PARAMETER-DATA-PROTOTYPE", "GZIP", "UNNUMBER", "INIT-EVENT", "IS-GREATER-THAN", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "REST-RESOURCE-DEF", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "DATA-CONSTR", "STRICTLY-DECREASING", "J-1939-NM--CCA", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "DDS-DOMAIN-RANGE", "ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE", "APMC-URI-PARAM-DEF", "STATE-MANAGEMENT-SYNC-ACTION-ITEM", "REQUEST-NO-RETURN", "VENDOR-SPECIFIC-SERVICE-NEEDS", "BUILD-TYPE-RELEASE", "CRYPTO-SERVICE-JOB-NEEDS", "SERVER-CALL-POINT", "STRICTLY-INCREASING", "WONT-RECEIVE", "TIMING-EXTENSION-RESOURCE", "BLOCK-SOURCE", "LISTEN", "RESOURCE-GROUP", "COUPLING-PORT-ENHANCED-TRAFFIC-SHAPER", "SOMEIP-EVENT-GROUP", "DIAGNOSTIC-SESSION-CONTROL", "IEEE-1722-RAW-DATA-STREAM-CONSUMER-INTERFACE", "DDS-ABSTRACT-SERVICE-INSTANCE-ELEMENT-CP", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "LIN-EVENT-TRIGGERED-FRAME", "PHM-HEALTH-CHANNEL-STATUS", "KEYWORD", "SYSTEM-SIGNAL-GROUP", "USE-VOID", "OPERATION-CALL-RECEIVED", "SUSPEND-TO-RAM-HUB-MAPPING", "PARAMETER-INTERFACE", "DEFAULT-MODE", "INTEGER-32", "BINARY-MANIFEST-PROVIDE-RESOURCE", "SHOW-LONG-NAME", "RESPONSE-SYNCHRONIZATION", "REST-OBJECT-REF", "ASSEMBLY-SW-CONNECTOR", "SERIALIZATION-TECHNOLOGY", "SD", "PURE-LOCAL-TIME-BASE", "JI", "DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING", "USER", "RPT-CONTAINER", "EL", "PROVIDED-SERVICE-INSTANCE", "SWITCH-STREAM-FILTER-ACTION-DEST-PORT-MODIFICATION", "VARIABLE-SIZE", "SERVICE-FIELD-DEPLOYMENT", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "MI", "TIMING-CONDITION", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "100BASE-T1", "CY", "ST", "SCHEDULE-VARIANT-3", "DIAGNOSTIC-PARAMETER-IDENT", "COM-FIELD-GRANT", "USE-ARGUMENT-TYPE", "NV-BLOCK-NEEDS", "OPEN", "CRYPTO-ELLIPTIC-CURVE-PROPS", "ICV-VERIFIED", "NO", "ECUC-CONTAINER-VALUE", "RESPOND-BEFORE-RESET", "COMPU-METHOD", "ETHERNET-RAW-DATA-STREAM-GRANT", "ECUC-CHOICE-CONTAINER-DEF", "ECUC-CONTAINER-DEF", "FAILURE-AND-SUCCESS", "NEW-IS-EQUAL", "ACL-OBJECT-SET", "SERVER-COM-SPEC-PROPS", "50", "EXECUTABLE-GROUP", "DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA", "CONSISTENCY-NEEDS", "EOC-EVENT-REF", "GLOBAL-TIME-ETH-MASTER", "10BASE-T1S", "DIAGNOSTIC-START-ROUTINE", "IDS-DESIGN", "FAULT", "FLOAT-32", "NUMBER", "ACTIVE", "SMPTE-338", "XFILE", "FUNCTIONAL-CAN-FD", "DYNAMIC-PART-TRIGGER", "LOCAL", "PORT-INTERFACE", "RU", "IEEE-1722-TP-ETHERNET-FRAME", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "SERVICE-INTERFACE", "USER-DEFINED-FIELD-DEPLOYMENT", "CM-MODULE-INSTANTIATION", "GETTER", "AF", "GENERAL-PURPOSE-PDU", "CALLBACK", "DIAGNOSTIC-SERVICE-INSTANCE", "FIXED-SIZE", "LIN-NM-CLUSTER", "IA", "ABSTRACT-CAN-CLUSTER", "RSA", "APMC-CONTAINER-ELEMENT-VALUE", "REDUNDANT", "LV", "IEEE-1722-TP-ACF-LIN", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "R-4--2", "TP-CONNECTION-IDENT", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "FLEXRAY-NM-CLUSTER", "DIAGNOSTIC-ACCESS-PERMISSION", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "IMPLEMENTATION-DATA-TYPE", "ADAPTIVE-EVENT-RECEIVED", "STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-ACTION-ITEM", "STARTUP-CONFIG-SET", "LEGACY", "DLT-LOG-CHANNEL-DESIGN", "DIAGNOSTIC-SOVD-METHOD", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED", "TD-EVENT-VFB-REFERENCE", "BAYER-BGGR", "ISO-14229--1", "EVENT-COMBINATION-ON-STORAGE", "IEEE-1722-ACF-BUS-PART-RAW-DATA-STREAM-CONSUMER-MAPPING", "FPP", "TASK", "IEEE-1722-TP-ACF-CAN-PART", "SWC-IMPLEMENTATION", "STATE-MANAGEMENT-MODULE-INSTANTIATION", "HIGH", "COMPOSITION-SW-COMPONENT-TYPE", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "BSW-SCHEDULABLE-ENTITY", "BSW-MODE-SWITCH-EVENT", "HEAD", "BEST-EFFORT", "DO-IP", "ECU-STATE-MGR-USER-NEEDS", "ECUC-INSTANCE-REFERENCE-DEF", "DERIVED-FROM", "FAILURE-ONLY", "PASSIVE", "SQ", "RPT-EXECUTABLE-ENTITY", "GLOBAL-TIME-CAN-MASTER", "MASKED-NEW-DIFFERS-X", "ZH", "DIAGNOSTIC-CONNECTED-INDICATOR", "DIAGNOSTIC-SOVD-DATA", "USER-DEFINED-GLOBAL-TIME-SLAVE", "INSTALL", "REQUIRED-AP-SERVICE-INSTANCE", "ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "INFINITE", "ETP", "HU", "HW-DESCRIPTION-ENTITY", "DELETE", "COUPLING-PORT-CREDIT-BASED-SHAPER", "J-1939-DCM-I-PDU", "SWITCH-ATS-INSTANCE-ENTRY", "TX-TRIGGER-MERGED", "SERIALIZER", "8-KHZ", "APMC-REVISION-LABEL-PARAM-VALUE", "MODE-DECLARATION-MAPPING-SET", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "EVAP", "SIGNAL-SERVICE-TRANSLATION-PROPS", "SYSTEM-TIMING", "STATE-MANAGEMENT-STATE-NOTIFICATION", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "ABSTRACT-CAN-COMMUNICATION-CONNECTOR", "RPT-ENABLER-RAM", "DIAGNOSTIC-AUTHENTICATION-CLASS", "IMPOSITION-TIME", "COM-TRIGGER-GRANT", "UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT", "NO-SHOW-LONG-NAME", "CRYPTO-KEY-SLOT-INTERFACE", "OS-MODULE-INSTANTIATION", "USER-DEFINED-TRANSFORMATION-PROPS", "LINK-LOCAL--DOIP", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "CHANNEL-B", "SU", "PDU-TO-FRAME-MAPPING", "STATE-MANAGEMEN-PHM-ERROR-INTERFACE", "APMC-URI-FOREIGN-REFERENCE-DEF", "BO", "CAPTURE-ASYNCHRONOUSLY-TO-REPORTING", "DIAGNOSTIC-SOVD-RECORD-CONTENT-ELEMENT", "STATE-MANAGEMENT-PHM-ERROR-INTERFACE", "IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "BULK-NV-DATA-DESCRIPTOR", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED", "AGE-CONSTRAINT", "COLOR-AWARE", "SDG-DEF", "SECURITY-EVENT-FILTER-CHAIN", "PERSISTENCY-DATA-ELEMENT", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "RAW-DATA-STREAM-GRANT", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "DETERMINISTIC-SYNC-INSTANTIATION", "DIAGNOSTIC-STORAGE-CONDITION", "PORT-GROUP", "DO-IP-GID-NEEDS", "DATA-FORMAT-ELEMENT-REFERENCE", "ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "SCHEDULE-VARIANT-6", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "IS-EXPIRED", "EVENT-WINDOW-CURRENT-CYCLE", "DIAGNOSTIC-SOVD-FAULT-MEMORY-ACCESS", "ISO-6", "GENERAL-PURPOSE-TIMER-SERVICE-NEEDS", "NO-PGWIDE", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "RAW-DATA-STREAM-SERVER-INTERFACE", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "CRYPTO-KEY-SLOT-CLIENT-INTERFACE", "IEEE-1722-RAW-DATA-STREAM-PRODUCER-MAPPING", "STATE-MANAGEMENT-SUSPEND-TO-RAM-ACTION-ITEM", "J-1939-TP-CONFIG", "MASKED-NEW-EQUALS-X", "DDS-SERVICE-INSTANCE-FIELD-CP", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "IEEE-1722-TP-ACF-BUS", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "BSW-COMPOSITION-TIMING", "USER-DEFINED-EVENT-DEPLOYMENT", "COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER", "APMC-BOOLEAN-PARAM-DEF", "ADAPTIVE-FIELD-SETTER-COMPLETED", "DIAGNOSTIC-SOVD-CONTENT-INTERFACE", "NON-REENTRANT", "ABSTRACT", "96-KHZ", "DDS-CP-CONFIG", "APMC-ABSTRACT-INSTANCE-REFERENCE-VALUE", "SDG-AGGREGATION-WITH-VARIATION", "LIN-MASTER", "DIAGNOSTIC-CONDITION", "APMC-STRONG-REVISION-LABEL-PARAM-DEF", "SIGN", "REST-ABSTRACT-PROPERTY-DEF", "APMC-CONTAINER-REFERENCE-DEF", "DIAGNOSTIC-MULTIPLE-RESOURCE-INTERFACE", "J-1939-NM-CLUSTER", "10000BASE-T1", "DDS-CP-DOMAIN", "NON-VOLATILE-RAM-MANAGER", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "VENDOR-SPECIFIC", "TW", "PORT-INTERFACE-MAPPING", "TRAP", "CAPTURE-SYNCHRONOUSLY-TO-REPORTING", "APPLICATION-RECORD-ELEMENT", "1", "COMPILE", "DATA-TRANSFORMATION-SET", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "COMM-CONNECTOR-PORT", "LIN-UNCONDITIONAL-FRAME", "ROTATE-90-CW-LIMIT-TO-TEXT", "MK", "8", "STATE-MANAGEMENT-NOTIFICATION-INTERFACE", "CRYPTO-KEY-SLOT-DESIGN", "CLIENT-SERVER-INTERFACE", "DIAGNOSTIC-PARAMETER-ELEMENT", "ON-COMPARISON-OF-VALUES", "SECURE-COM-PROPS", "AP-APPLICATION-ERROR", "AVB--IEEE-802--1-AS", "CANNOT-BE-REMOVED", "COUPLING-PORT", "DDS-METHOD-DEPLOYMENT", "J-1939-SHARED-ADDRESS-CLUSTER", "TT", "J-1939-CONTROLLER-APPLICATION", "INTERRUPT-CAT-2", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING", "EXECUTABLE-ENTITY", "NETWORK-ENDPOINT", "NA", "ABSTRACT-CLASS-TAILORING", "IDSM-INSTANCE", "SOMEIP-METHOD", "SCHEDULE-VARIANT-7", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "ALIAS-NAME-SET", "DEVELOPMENT-ERROR-TRACER", "NORMALFIXED", "TOPBOT", "DIAGNOSTIC-COMPONENT-NEEDS", "TD-EVENT-FR-CLUSTER-CYCLE-START", "RESPONSE", "CAN-FRAME", "DIAGNOSTIC-SOVD-UPDATE-INTERFACE", "LOGIC-ADDRESS", "VAR-FAST", "NON-EMMISSION-RELATED-DTC", "POST-BUILD", "EXCLUSIVE-AREA-NESTING-ORDER", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "SAFETY", "SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS", "CIRCLE", "EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "ON-ENTRY", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "EXECUTION-TIME-CONSTRAINT", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "PROCESS-EXECUTION-ERROR", "NON-OS-MODULE-INSTANTIATION", "IEEE-1722-TP-ACF-CONNECTION", "DIAGNOSTIC-READ-DTC-INFORMATION", "SEARCH-FOR-ALL-INSTANCES", "32-KHZ", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "RW", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "V-2-X-ACTIVE-SUPPORTED", "EID-USE-MAC", "APPLICATION-ASSOC-MAP-ELEMENT", "NM-HANDLE-INACTIVE-TO-FUNCTION-GROUP-STATE", "1000BASE-T", "DETERMINISTIC-SYNC-MASTER", "PHM-RULE", "EXTENDED", "VARIABLE-DATA-PROTOTYPE-RECEIVED", "SUPERVISION-CHECKPOINT", "SEARCH-FOR-ID", "CALLOUT", "MACHINE-CYCLE", "PHYSICAL-CHANNEL", "ECUC-DESTINATION-URI-DEF-SET", "USER-DEFINED-METHOD-DEPLOYMENT", "WILL-SEND", "TRACEABLE-TEXT", "ADAPTIVE-METHOD-CALL-RECEIVED", "SDG-CLASS", "APMC-CONFIGURATION-ELEMENT-DEF", "ISO-11992--4", "CYCLE-REPETITION-64", "IEEE-1722-TP-ACF-BUS-PART", "BROAD-R-REACH", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "FRAME-RECEIVED-BY-IF", "ACCESS-PERMISSION-SERVICE-CLASS", "BSW-MODE-SWITCHED-ACK-EVENT", "P-PORT-PROTOTYPE", "SWC-TO-ECU-MAPPING", "DIAGNOSTIC-COMMON-ELEMENT", "INTERRUPT", "STATUS-BIT-AGING-AND-DISPLACEMENT", "DETAILED", "ETHERNET-COMMUNICATION-CONNECTOR", "WONT-SEND", "PHM-CHECKPOINT", "GLOBAL-TIME-CAN-SLAVE", "TIMING-EXTENSION", "GLOBAL-TIME-DOMAIN", "RPT-ENABLER-ROM", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "FM-FEATURE-RESTRICTION", "REQUEST", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "COM-FIND-SERVICE-GRANT", "NETWORK-REPRESENTATION-FROM-COM-SPEC", "BSW-MODULE-TIMING", "NO-TRANSFORMER-STATUS-FORWARDING", "SWC-MODE-SWITCH-EVENT", "SYSTEM", "CRYPTO-KEY-SLOT", "TH", "GA", "MC-DATA-INSTANCE", "DIAGNOSTIC-EDR-SENDER-PORT-MAPPING", "J-1939-DCM-DM-19-SUPPORT", "BAYER-GBRG", "PC-AFFECTS-PB", "MEDIUM", "SYMMETRIC-KEY", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "HIERARCHICAL-EOC", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "KEEP", "CONST", "VARIANT-PRE-COMPILE", "ECU-INSTANCE", "SG", "CONFIDENTIALITY-OFFSET--0", "IDSM-CONTEXT-PROVIDER-MAPPING", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "FIX_AXIS", "TP-CONFIG", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "NETWORK-HANDLE-PORT-MAPPING", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "PARTIAL-NETWORK", "HW-PIN", "DO-IP-POWER-MODE-STATUS-NEEDS", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "TRACE-SWITCH-NONE", "OPERATION-CALL-RESPONSE-RECEIVED", "REMOVE", "PRESENTATION-CONTINUOUS", "ERROR-TRACER", "RUNNABLE-ENTITY-GROUP", "TRANSFORMATION-I-SIGNAL-PROPS-IDENT", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "SHOW-ALIAS-NAME", "STATE-MANAGEMENT-LEAVE-SUSPEND-TO-RAM-ACTION-ITEM", "CRYPTO-CERTIFICATE-GROUP", "EVENT-WINDOW-INFINITE", "BSW-MODULE-CALL-POINT", "CRYPTO-SERVICE-KEY", "DIAGNOSTIC-COM-CONTROL", "BRIEF-BYPASSING-FILTERS", "TRANSFER", "ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER", "APMC-IP-V4-ADDRESS-PARAM-DEF", "SOMEIP-EVENT", "DIAGNOSTIC-ROUTINE-INTERFACE", "DIAGNOSTIC-EVENT", "I-PDU", "WAIT-TIME-DATE", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE", "GATEWAY", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "TIME-BASE-RESOURCE", "DIAGNOSTIC-ABSTRACT-SOVD-CONTENT", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "DIAGNOSTIC-REQUEST-DOWNLOAD", "I-SIGNAL-PORT-TO-DIAGNOSTIC-EVENT-MAPPING", "DDS-FIELD-DEPLOYMENT", "APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-VALUE", "ROTATE-180", "ALIVE-SUPERVISION", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "COMMUNICATION-CONTROLLER", "DIAGNOSTIC-EVENT-NEEDS", "DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS", "PROCESS-PHM-ACTION-ITEM", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING", "BSW-IMPLEMENTATION", "ADAPTIVE-METHOD-RESPONSE-RECEIVED", "CLIENT-AUTHENTICATE", "XCP", "LTS-13", "SIMULATED-EXECUTION-TIME", "SYMMETRIC", "MONOTONOUS", "TD-EVENT-SWC", "I-SIGNAL-AVAILABLE-FOR-RTE", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "BASE-TYPE", "SWC-INTERNAL-BEHAVIOR", "APPLICABILITY-INFO-SET", "BMP", "FLEXRAY-TP-NODE", "TIMING-MODE-INSTANCE", "V-2-X-M-USER-NEEDS", "SYMBOL-PROPS", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "COUPLING-PORT-STRUCTURAL-ELEMENT", "NOT", "CANCEL-CAMPAIGN", "BAM", "OFFSET", "SW-CLASS-INSTANCE", "TIME", "PROTECTED", "DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING", "ARRAY", "EXTEND", "STRICT-PRIORITY", "RTE-EVENT-IN-SYSTEM-SEPARATION", "PASS-THROUGH-SW-CONNECTOR", "FATAL", "CAUTION", "REST-ARRAY-PROPERTY-DEF", "SOMEIP-EVENT-DEPLOYMENT", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "EVENT-MAPPING", "ON-CHANGE", "DATA-TYPE-MAPPING-SET", "CRYPTO-KEY-SLOT-USER-DESIGN-MAPPING", "FAST-FLASHING-MODE", "ECU-PARTITION-TO-CORE-MAPPING", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "DDS-SECURE-COM-PROPS", "SEC-OC-CRYPTO-SERVICE-MAPPING", "APMC-REFERENCE-VALUE", "MY", "DIAGNOSTIC-SOVD-CONFIG-CONTENT-MAPPING", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "MASTER", "DIAGNOSTIC-ECU-RESET-INTERFACE", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "APMC-VALUE-COLLECTION", "SATURATE", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "UNDEFINED", "KEY-SERVER", "ADAPTIVE-FIELD-NOTIFICATION-RECEIVED", "MEASURED-STACK-USAGE", "GLOBAL-TIME-SLAVE", "AH", "COM-MANAGEMENT-MAPPING", "FALSE", "ROTATE-90-CCW-FIT-TO-TEXT", "DEF-ITEM", "ZU", "RAW-DATA-STREAM-MAPPING", "NO-SHOW-CONTENT", "PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR", "BLOCK", "ECUC-ABSTRACT-STRING-PARAM-DEF", "TIME-SYNC-MODULE-INSTANTIATION", "ADAPTIVE-AUTOSAR-APPLICATION", "UNIT-GROUP", "SUSPEND-TO-RAM-HUB-INTERFACE", "NEW-IS-DIFFERENT", "TLS-13", "SW-SYSTEMCONST", "FLEXRAY-PHYSICAL-CHANNEL", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "USE-ARRAY-BASE-TYPE", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "TTCAN-CLUSTER", "SECURITY-EVENT-REPORT-INSTANCE-VALUE", "BAYER-RGGB", "BT-REC-601", "TTCAN-COMMUNICATION-CONTROLLER", "ABSTRACT-PROVIDED-PORT-PROTOTYPE", "TIMING-EVENT", "IMPLEMENTATION-PROPS", "DIAGNOSTIC-J-1939-NODE", "DOCUMENTATION", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-PORT-MAPPING", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "BINARY-MANIFEST-RESOURCE", "STATE-MANAGEMENT-TRIGGER-INTERFACE", "PER-EXECUTABLE", "SYSTEM-COM-SPEC-DEFINITION-SET", "DDS-SIGNAL", "DLT-USER-NEEDS", "PERSISTENCY-KEY-VALUE-STORAGE", "INDICATE", "MEMORY-SECTION", "CLIENT-MAC-VERIFY", "CYCLE-REPETITION-16", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "EVENT-STORAGE-ENABLED", "VOLATILE", "PROCESS-TO-MACHINE-MAPPING-SET", "AUTO-IPDHCPV-4", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "EN", "TOPIC-1", "UCM-DESCRIPTION", "NETWORK-CONFIGURATION", "QUEUED-RECEIVER-COM-SPEC-PROPS", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "DIAGNOSTIC-TRANSMIT-CERTIFICATE-INTERFACE", "SERVICE-DISCOVERY", "AUTO", "SWC-TO-APPLICATION-PARTITION-MAPPING", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "CRYPTO-CERTIFICATE-GROUP-TO-PORT-PROTOTYPE-MAPPING", "MACHINE-TIMING", "FUNCTION-INHIBITION-MANAGER", "OUT", "1-001", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "CPP", "ATP-BLUEPRINT", "SUPERVISION-MODE", "REST-ELEMENT-DEF", "RPT-EXECUTION-CONTEXT", "ENCRYPTION", "SIGNATURE", "KEEP-EXISTING", "INTER-PARTITION-INTRA-ECU", "CLASSIC", "TEST-PASSED", "FIRST-TO-SECOND", "STATIC-SOCKET-CONNECTION", "PT", "BSW-TIMING-EVENT", "BSW-MODE-MANAGER-ERROR-EVENT", "RECORD-VALUE-FIELD", "DIAGNOSTIC-SOVD-LOG", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE", "DIAGNOSTIC-FIM-EVENT-GROUP", "ADAPTIVE-SERVICE-OFFER-STARTED", "KY", "DIAGNOSTIC-COM-CONTROL-CLASS", "APMC-REFERENCE-DEF", "APMC-CONTAINER-DEF", "DIAGNOSTIC-ABSTRACT-ALIAS-EVENT", "DIAGNOSTIC-SOVD-CONTENT-ELEMENT-INTERFACE", "NOT-SENT", "EDGE-NODE", "MODE-DECLARATION-REQUESTED", "DIAGNOSTIC-CONDITION-INTERFACE", "DIAGNOSTIC-DATA-PORT-MAPPING", "VO", "AUTOSAR-DATA-TYPE", "SYNCHRONOUS-SERVER-CALL-POINT", "FLEXRAY-COMMUNICATION-CONTROLLER", "USER-DEFINED-COMMUNICATION-CONTROLLER", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "TLS-CRYPTO-CIPHER-SUITE", "MIXED-29-BIT", "IDS-MAPPING", "BSW-MODULE-ENTITY", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "LT-AFFECTS-PB", "ONE-EVERY-N", "FM-FEATURE-MODEL", "SECURITY-EVENT-THRESHOLD-FILTER", "TX-REF-TRIGGER-GAP", "MACHINE", "SO-CON-I-PDU-IDENTIFIER", "EOC-EXECUTABLE-ENTITY-REF", "USE-FIRST-CONTEXT-DATA", "TLS-JOB-REQUIREMENT", "SW-MC-FRAME", "DESELECTED", "J-1939-NM--AAC", "SCHEDULE-VARIANT-2", "UCM-STEP", "TD-EVENT-SLLET", "SUPERVISED-ENTITY-NEEDS", "PHYSICAL", "SS", "FLOAT-64", "DLT-ARGUMENT-PROPS", "STATIC-OR-DYNAMIC-PART-TRIGGER", "COM-METHOD-GRANT-DESIGN", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "FIREWALL-RULE", "TI", "FRAME-ETHERNET-SENT-ON-BUS", "PAYLOAD-AS-POINTER-TO-ARRAY", "DIAGNOSTIC-SOVD-PRIMITIVE-CONTENT-ELEMENT", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "SR", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "ACTIVATION-AND-TRIGGER-UNICAST", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "ON-TRANSITION", "HALF-DUPLEX-MODE", "DATA-PROTOTYPE-GROUP", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "REQUEST-CALLBACK-TYPE-SUPPLIER", "TD-EVENT-VFB-PORT-GROUP", "BUILD-ACTION-MANIFEST", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "MAPPING-SCOPE-CORE", "APMC-CONTAINER-VALUE", "REPORTING-IN-CHRONOLOGICAL-ORDER-OLDEST-FIRST", "BUILD", "TLS-12", "LIFE-CYCLE-STATE", "REDUNDANT-PER-ELEMENT", "CONTINUE-AT-IT-POSITION", "GLOBAL-TIME-ETH-SLAVE", "LO", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "PASSTHROUGH", "KEYWORD-SET", "25", "GD", "ERROR", "FUNCTION-INHIBITION-NEEDS", "NTP--RFC-958", "TRIGGERED-ON-CHANGE-WITHOUT-REPETITION", "ABSTRACT-IAM-REMOTE-SUBJECT", "SUSPEND-TO-RAM-NOT-SUPPORTED", "CAN-COMMUNICATION-CONNECTOR", "MOST-SIGNIFICANT-BYTE-FIRST", "SYNCHRONIZED-SLAVE-TIME-BASE", "APMC-STRONG-REVISION-LABEL-PARAM-VALUE", "NO-SUPERVISION", "X-509", "LIMIT", "ANY-SEND-OPERATION", "NO-BOOT", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "GENERIC-ETHERNET-FRAME", "IS-GREATER-OR-EQUAL", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-PORT-MAPPING", "IEEE-1722-TP-CONNECTION", "NO-STATUS-BYTE-CHANGE", "EO", "RESET-MCU", "PHM-ACTION", "NO-STORE-EVENT", "IS-EQUAL", "CYCLE-REPETITION-8", "CRYPTO-MODULE-INSTANTIATION", "RUNNABLE-ENTITY-VARIABLE-ACCESS", "MAPPING-SCOPE-PARTITION", "100BASE-TX", "SCHEDULE-VARIANT-5", "FRAME", "BINARY-MANIFEST-ITEM", "DIAG-REQUEST", "ACTIVATION-MULTICAST", "PHYSICAL-DIMENSION", "SW-CONNECTOR", "RPT-ENABLER-RAM-AND-ROM", "DIAGNOSTIC-INDICATOR", "TRANSPORT", "DO-IP-INTERFACE", "PRECONFIGURED-CONFIGURATION", "DYNAMIC", "SEC-OC-DEPLOYMENT", "RAW-DATA-STREAM-DEPLOYMENT", "IDSM-REPORTING-MODE-PROVIDER-INTERFACE", "PUBLISHED-INFORMATION", "IEEE-1722-TP-ACF-LIN-PART", "PGWIDE", "ETHERNET-RAW-DATA-STREAM-MAPPING", "SDG-TAILORING", "REGULAR", "IT", "NO-ACK", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "CHANNEL-A", "RETURN-VALUE-PROVIDED", "DDS-CP-PARTITION", "HEAP-USAGE", "CANCEL", "STATUS-BIT-NORMAL", "ON-DTC-STATUS-CHANGE", "J-19393", "COM_AXIS", "LIMIT-TO-PAGE", "IE", "SV", "TLS-IAM-REMOTE-SUBJECT", "TEST-FAILED", "PROPRIETARY-2", "MAC-ADDRESS-VLAN-MEMBERSHIP", "ENABLE", "HW-ATTRIBUTE-LITERAL-DEF", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "REQUEST-CALLBACK-TYPE-MANUFACTURER", "JUSTIFY", "MIN", "DO-IP-NETWORK-CONFIGURATION-DESIGN", "MAC-SEC-GLOBAL-KAY-PROPS", "CRYPTO-PROVIDER", "DETERMINISTIC-CLIENT", "JW", "BSW-SERVICE-DEPENDENCY-IDENT", "AR-PACKAGE", "OM", "CHECKPOINT-TRANSITION", "COM-FIELD-GRANT-DESIGN", "SECURITY-EVENT-ONE-EVERY-N-FILTER", "START-FROM-BEGINNING", "TRIGGERED-ON-EVALUATION", "ACL-ROLE", "DIAGNOSTIC-SOVD-LOCK", "DROP-UNTAGGED", "15", "SECURE-COM-PROPS-SET", "USER-DEFINED", "SERVICE-PROXY-SW-COMPONENT-TYPE", "DLT-ARGUMENT", "INTERFACE-MAPPING", "CRYPTO-PRIMITIVE", "ACCESS-PERMISSION-SERVICE-INSTANCE", "IGNORE", "DIAGNOSTIC-REQUEST-ROUTINE-RESULTS", "GENERIC-MODULE-INSTANTIATION", "CAN-TP-CONFIG", "APMC-PARAM-CONF-CONTAINER-DEF", "TIME-SYNC-SERVER-CONFIGURATION", "DEVELOPMENT-ERROR", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "PSK", "APMC-IP-V6-ADDRESS-PARAM-VALUE", "IAM-MODULE-INSTANTIATION", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "FUNCTIONAL", "GU", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "ECUC-CHOICE-REFERENCE-DEF", "OFFSET-TIMING-CONSTRAINT", "TD-EVENT-CYCLE-START", "APMC-CONTAINER-REFERENCE-VALUE", "UDP-CHECKSUM-DISABLED", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "CRYPTO-PROVIDER-INTERFACE", "SUSPEND-TO-RAM-SATELLITE-MAPPING", "SOCKET-ADDRESS", "TRANSFORMER-HARD-ERROR-EVENT", "COM-GRANT-DESIGN", "DATA-WRITE-COMPLETED-EVENT", "LEAF-OF-TARGET-CONTAINER", "LIN-SLAVE-CONFIG-IDENT", "30", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "V-2-X-DATA-MANAGER-NEEDS", "SERVICE-INTERFACE-MAPPING", "STARTUP-CONFIG", "USER-DEFINED-I-PDU", "AP-SOMEIP-TRANSFORMATION-PROPS", "FUNCTION-GROUP-SET", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "GENERAL-PURPOSE-I-PDU", "UCM-MASTER", "FLEXRAY-AR-TP-NODE", "DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "COUPLING-PORT-SCHEDULER", "UNDECIDED", "UDP-NM-NODE", "SWC-TIMING", "ACK-WITH-RT", "DDS-CP-PROVIDED-SERVICE-INSTANCE", "ERROR-CORRECTION", "UPLOADABLE-DESIGN-ELEMENT", "VAR-NO-INIT", "GLOBAL-SUPERVISION-ENTITY", "PRESHARED-KEY-IDENTITY", "ROOT-SW-COMPONENT-PROTOTYPE", "ICMP", "TRACE", "EID-USE-CONFIG-VALUE", "SEARCH-FOR-SPECIFIC-INSTANCE", "ETH-TCP-IP-ICMP-PROPS", "ML", "IEEE802-1AS", "BSW-BACKGROUND-EVENT", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "ETH-IP-PROPS", "STRICT-MONOTONOUS", "FM-FEATURE-SELECTION", "CLEAR", "EVENT-COMBINATION-ON-RETRIEVAL", "TIME-MASTER", "ACTION", "FI", "PEER", "FIRST-CONTAINED-TRIGGER", "IDS-PLATFORM-INSTANTIATION", "STATE-MANAGEMENT-SLEEP-ACTION-ITEM", "TL", "SINGLE-OCCURRENCE", "DIAGNOSTIC-SECURE-CODING-MAPPING", "SOMEIP-REMOTE-MULTICAST-CONFIG", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "TESTED", "DATA-PROTOTYPE-TRANSFORMATION-PROPS-IDENT", "DIAGNOSTIC-EVENT-INTERFACE", "DIAGNOSTIC-MAPPING", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "APMC-FUNCTIONAL-CLUSTER-VALUE", "SW", "CONFIRMED", "TD-EVENT-VFB-PORT", "STATE-CLIENT-INTERFACE", "REST-ENDPOINT-GET", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "ECUC-INTEGER-PARAM-DEF", "RPT-SERVICE-POINT", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "DIAGNOSTIC-RESPONSE-ON-EVENT", "ACTIVATION-UNICAST", "FLEXRAY-FRAME", "WO", "AFTERMAKET", "ETHERNET-FRAME-TRIGGERING", "SYSTEM-SUPPLIER-BOOT", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "DIAGNOSTIC-SOVD-COMPOSITE-CONTENT-ELEMENT", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "100", "INTERGRITY-AND-CONFIDENTIALITY", "LOCAL-SUPERVISION", "DIAGNOSTIC-TROUBLE-CODE-OBD", "CRYPTO-SERVICE-CERTIFICATE", "APMC-URI-INSTANCE-REFERENCE-DEF", "HOST-PORT", "SECURITY-EVENT-REPORT-INTERFACE", "ETHERNET-CLUSTER", "SW-CALIBRATION-METHOD", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "RULE", "CREATED-BY-INTEGRATOR", "PHM-RECOVERY-ACTION-INTERFACE", "SWITCH-STREAM-FILTER-RULE", "AS-IS", "SERVICE-INTERFACE-DEPLOYMENT-ELEMENT", "ACCEPT-ALL", "POSSIBLE-ERROR-REACTION", "CO", "QUEUED", "SEARCH-FOR-ANY", "BSW-MODULE-CLIENT-SERVER-ENTRY", "TRANSLATION-START", "GRANT", "ROUTER", "PHYSICAL-ADDRESS", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "DOCUMENT-ELEMENT-SCOPE", "J-1922", "IDSM-TIMESTAMP-PROVIDER-MAPPING", "EVALUATED-VARIANT-SET", "PERSISTENCY-FILE-PROXY", "STOP", "ECUC-FUNCTION-NAME-DEF", "NE", "ADAPTIVE-FIREWALL-MODULE-INSTANTIATION", "LAST-IS-BEST", "MODE-INTERFACE-MAPPING", "ANY-PARTIAL-NETWORK-ACTIVE", "FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING", "SW-INSTANCE", "VFB-TIMING", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING", "TK", "TIMING-DESCRIPTION-EVENT-CHAIN", "LOG-AND-TRACE-INTERFACE", "KL", "UP-LINK-PORT", "FILTERED", "IW", "FM-FEATURE-MAP", "PRIMARY-ECU", "DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING", "ASYMMETRIC-FROM-BYTE-ARRAY", "COM-OFFER-SERVICE-GRANT", "IEEE-1722-RAW-DATA-STREAM-CONSUMER-MAPPING", "INOUT", "ECUC-FLOAT-PARAM-DEF", "BASE-T", "LIFE-CYCLE-INFO-SET", "TDLET-ZONE-CLOCK", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "RUN-CONTINUOUS", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "MODE-ACCESS-POINT-IDENT", "CLIENT-SERVER-OPERATION", "MIDDLE", "SIDES", "DERIVED-FROM-DESIGN", "OR", "DIAGNOSTIC-ENABLE-CONDITION", "TLS-JOB-MAPPING", "STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM", "J-1939-PROTECTED-I-PDU", "SERVER-VERIFY", "SCHEDULED", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "AUTO-IP--DOIP", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "VIDEO-FRAME", "EVENT-HANDLER", "FULL-COM", "EXCLUSIVE-AREA", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "PDF", "1000BASE-T1", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "CONFIRMED-DTC-BIT", "SDG-ATTRIBUTE", "MAC-MULTICAST-GROUP", "DLT-ECU", "SECURITY", "IEEE-1722-TP-CONFIG", "ROTATE-180-LIMIT-TO-TEXT", "SYSTEM-SIGNAL", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "MULTIPLE-OCCURRENCES", "APPLICATION-INTERFACE", "BREAK", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "TEST-FAILED-BIT", "CP-SOFTWARE-CLUSTER-RESOURCE", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "BSW-SCHEDULE-EVENT", "ANY", "DO-IP-SERVICE-NEEDS", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "SH", "192-KHZ", "ETHERNET-MAC-RAW-DATA-STREAM-MAPPING", "APPLICATION-PRIMITIVE-DATA-TYPE", "PERSISTENCY-FILE-ARRAY", "IS-NOT-EQUAL", "SYSTEM-SUPPLIER-BOOT-RESP-APP", "BSW-INTERNAL-BEHAVIOR", "APMC-ABSTRACT-REFERENCE-VALUE", "SYSTEM-MAPPING", "SUSPEND-TO-RAM-SATELLITE-INTERFACE", "JPG", "REST-ENDPOINT-PUT", "TD-EVENT-COMPLEX", "INSTANCE-ID", "TCP", "RESTART", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "APMC-NUMERICAL-PARAM-VALUE", "SCHEDULE-VARIANT-1", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "MULTIPLEXED-I-PDU", "REJECT", "SERVICE-INSTANCE-COLLECTION-SET", "WATCH-DOG-MANAGER", "NO-SHOW-SHORT-NAME", "TRIGGER-RELEASED", "APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-DEF", "ADAPTIVE-FIELD-NOTIFICATION-SENT", "INTEGER-64", "ABSTRACT-FUNCTIONAL-CLUSTER-DESIGN", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "APMC-CHOICE-CONTAINER-REFERENCE-DEF", "INTER-LET-ONLY", "NOHREF", "SENDER-RECEIVER-INTERFACE", "DA", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "KM", "FDCMDT", "IS-RELEVANT", "CYCLIC-AND-ON-CHANGE", "DIAGNOSTIC-EVENT-PORT-MAPPING", "NOT-TESTED", "XDOC", "STATE-MANAGEMENT-ERROR-INTERFACE", "CLIENT-VERIFY", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "DO-IP-TP-CONFIG", "DIAGNOSTIC-INDICATOR-NEEDS", "USER-DEFINED-GLOBAL-TIME-MASTER", "BR", "DIAGNOSTIC-MEMORY-BY-ADDRESS", "CA", "NODE", "NOTIFICATION", "SHARED", "SOVD-MODULE-INSTANTIATION", "BSW-INTERRUPT-ENTITY", "J-19392", "BLUEPRINT-DERIVATION-TIME", "BY-RECEPTION-TIMESTAMP", "CRYPTO-KEY-SLOT-USAGE-DESIGN", "LONG-HEADER", "SDG-PRIMITIVE-ATTRIBUTE", "OC", "TCP-OPTION-FILTER-LIST", "PROCESS", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "TIFF", "SDG-ABSTRACT-FOREIGN-REFERENCE", "OCCURENCE", "ON-EXIT", "EVAPPURGEFLOW", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "NV-BLOCK-SW-COMPONENT-TYPE", "IDSM-TIMESTAMP-PROVIDER-INTERFACE", "CALCULATED", "DIAGNOSTIC-EXTENDED-DATA-RECORD-NEEDS", "JAVA", "AGREED", "MACHINE-DESIGN", "POST-BUILD-VARIANT-CRITERION", "ATP-FEATURE", "FRAME-TRIGGERING", "NO-SHOW-NUMBER", "ROUGH-ESTIMATE-HEAP-USAGE", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "LIN-TP-CONFIG", "LOWER-8-BIT", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "ALL-16-BIT", "MN", "CRYPTO-JOB", "IP-SEC-IAM-REMOTE-SUBJECT", "48", "CONDITIONAL", "NEW-IS-LESS", "UNIT", "RM", "AY", "VERSION-1", "PCM", "CAN-PHYSICAL-CHANNEL", "SERVER-MAC-VERIFY", "I-PV-6-EXT-HEADER-FILTER-LIST", "ATP-DEFINITION", "CAT-1", "CRYPTO-SERVICE-NEEDS", "FRAME-ETHERNET-RECEIVED-ON-BUS", "PERSISTENCY-FILE-STORAGE", "STACK-USAGE", "SI", "CRYPTO-CERTIFICATE-INTERFACE", "SW-SYSTEMCONSTANT-VALUE-SET", "J-19396", "TN", "OPAQUE", "SLAVE-ACTIVE", "ADAPTIVE-SERVICE-OFFER-COMPLETED", "IDSM-QUALIFIED-EVENT-RECEIVER-INTERFACE", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "PORT", "INSTRUCTION", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "SLP", "SOMEIP-FIELD-DEPLOYMENT", "DIAGNOSTIC-J-1939-SW-MAPPING", "BINARY-MANIFEST-META-DATA-FIELD", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "IMPLEMENTATION", "AMBER-WARNING", "PROCESS-TO-MACHINE-MAPPING", "APMC-REVISION-LABEL-PARAM-DEF", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING", "PREDEFINED-VARIANT", "CAN-TP-CHANNEL", "AP-APPLICATION-ERROR-DOMAIN", "START", "HEALTH-CHANNEL", "SYNC-BASE-TIME-MANAGER", "PROVIDED-DDS-SERVICE-INSTANCE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "RPT-LEVEL-1", "RES-AXIS", "VAR", "AGE", "USER-DEFINED-PDU", "INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE", "APMC-MAC-ADDRESS-PARAM-VALUE", "APMC-TEXTUAL-PARAM-VALUE", "ADAPTIVE-FIELD-SETTER-CALLED", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "IS-FAILED", "ROTATE-90-CCW", "EXAMPLE", "XOR", "KN", "NO-SHOW-SEE", "SOMEIP-FIELD", "DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE", "ACTION-LIST", "COMPOSITE-INTERFACE", "ETH-TCP-IP-PROPS", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE", "DIAGNOSTIC-MEMORY-IDENTIFIER", "AM", "TD-EVENT-SERVICE-INSTANCE-EVENT", "DIAGNOSTIC-CONNECTION", "SHOW-CONTENT", "BOLDITALIC", "CPP-IMPLEMENTATION-DATA-TYPE", "BINARY-MANIFEST-REQUIRE-RESOURCE", "STRING", "INTRA-LET-EOC", "ENCRYPT-AND-SIGN", "24", "KEEP-ALL", "AUTOMATIC", "4-2-2", "UZ", "SEARCH-FOR-ALL", "APMC-URI-FOREIGN-REFERENCE-VALUE", "FY", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "ARTIFACT-CHECKSUM", "CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING", "DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE", "IMPOSITION-TIME-DEFINITION-GROUP", "SECURITY-EVENT-REPORT-INSTANCE-DEFINITION", "PAYLOAD-AS-ARRAY", "RPT-COMPONENT", "RAW-DATA", "MEASURED-EXECUTION-TIME", "SYNCHRONIZATION-TIMING-CONSTRAINT", "CRYPTO-TRUST-MASTER-INTERFACE", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "KEY-DERIVATION", "REST-ENDPOINT-POST", "TRACE-SWITCH-ARTI", "FORGET", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "ADAPTIVE-SERVICE-FIND-COMPLETED", "IMMEDIATE", "ALTERNATING-8-BIT", "ITU-BT-2020", "TRACE-SWITCH-ARTI-AND-LOG", "FULL-DUPLEX-MODE", "CP-SOFTWARE-CLUSTER", "DIAGNOSTIC-AGING", "COMMUNICATION-CLUSTER", "PROCESSING-STYLE-ASYNCHRONOUS", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "MONO", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "COLDSTART", "REPORT-AFTER-INIT", "10", "DIAGNOSTIC-SOVD-PORT-INTERFACE", "LINK-TIME", "SERVER-ENCRYPT", "I-PV-6-EXT-HEADER-FILTER-SET", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "NO-DEFAULT", "AS", "AZ", "HW-TYPE", "SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-PORT-MAPPING", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "SHOW-SEE", "TIMING-DESCRIPTION-EVENT", "SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING", "NM-NETWORK-HANDLE", "RAW-DATA-STREAM-GRANT-DESIGN", "NO-SLOPPY", "UDP-CHECKSUM-ENABLED", "ADAPTIVE-FIELD-GETTER-COMPLETED", "AUTO-IP", "CUSTOM", "DOES-NOT-SUPPORT-BUFFER-LOCKING", "J-1939-NM--NCA", "TO", "SOMEIP-TP-CHANNEL", "ATP-TYPE", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "QU", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "MODE-SWITCH-INTERFACE", "NFOLD", "RTPGE", "DIAGNOSTIC-EDR-SERVER-PORT-MAPPING", "CYCLE-REPETITION-10", "GLOBAL-TIME-MASTER", "HW-ELEMENT", "INT-16-BIT", "IDSM-MODULE-INSTANTIATION", "NO-MONOTONY", "SELECTED", "PLATFORM-HEALTH-MANAGEMENT-INTERFACE", "SECURED-PDU-HEADER-16-BIT", "FOR-ALL", "FIT-TO-TEXT", "DOES-NOT-USE-LOGGING", "ECUC-LINKER-SYMBOL-DEF", "SW-COMPONENT-PROTOTYPE", "WORST-CASE-HEAP-USAGE", "CONTINUOUS-ON-MODE", "J-1939-REQUEST-MANAGER", "UPDATE-CONFIGURATION", "CLOSED", "EXCLUSIVE", "INFINITE-TIME-TO-RESPONSE", "APPLICATION-ENDPOINT", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "UK", "APPLICATION-DEFERRED-DATA-TYPE", "DDS-CP-QOS-PROFILE", "VERBOSE", "I-SIGNAL-PORT", "EXECUTABLE", "IDSM-ABSTRACT-PORT-INTERFACE", "HY", "DEFAULT-IF-UNDEFINED", "CONFIGURED", "PTP--IEEE-1588--2002", "CAN-BRIEF", "ATP-CLASSIFIER", "SYSTEM-MEMORY-USAGE", "NO-COM", "ECUC-PARAMETER-DEF", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "NM-INSTANTIATION", "SOVD-SERVER-INSTANTIATION", "PHM-ARBITRATION", "DIAGNOSTIC-DE-AUTHENTICATION", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "FLAT-INSTANCE-DESCRIPTOR", "SWITCH-STREAM-FILTER-ENTRY", "FIRE-AND-FORGET-METHOD-MAPPING", "LOGICAL-SUPERVISION", "DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER", "TRANSIENT-FAULT", "60", "TERMINATE", "OBJECT", "BOOLEAN", "TIME-SYNCHRONIZATION-INTERFACE", "ITALIC", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "CAN-NM-NODE", "DEPENDANT", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE", "UDP-NM", "FLEXRAY-COMMUNICATION-CONNECTOR", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "LIN-SCHEDULE-TABLE", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "4-2-2-4", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "START-ON-BOOT", "MG", "DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING", "SECURE-COMMUNICATION-DEPLOYMENT", "RUNNABLE-ENTITY-STARTED", "GENERIC-TP-CONNECTION", "INTERPOLATION-ROUTINE-MAPPING-SET", "DIAGNOSTIC-SOVD-CONTENT", "NO-KEEP", "SWITCH", "CAN", "SPECIFICATION-DOCUMENT-SCOPE", "BSW-DISTINGUISHED-PARTITION", "ECUC-MODULE-CONFIGURATION-VALUES", "REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST", "CAPTURE-SYNCHRONOUS-TO-REPORTING", "5000BASE-T1", "CP", "HEALTH-CHANNEL-SUPERVISION", "OEM-BOOT", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "RECOVERY-NOTIFICATION", "IEEE-1722-TP-AAF-CONNECTION", "TRIGGER-ACTIVATED", "DDS-SECURE-GOVERNANCE", "REACTION", "SK", "TESTED-AND-FAILED", "APMC-FOREIGN-REFERENCE-VALUE", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "FURTHER-ACTION-BYTE-NEEDS", "DIAGNOSTIC-ABSTRACT-SOVD-CONTENT-INTERFACE", "RX-TRIGGER", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "CAN-NM-CLUSTER", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "IEEE-1722-RAW-DATA-STREAM-PRODUCER-INTERFACE", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "SCHEDULE-VARIANT-4", "SWC", "CAN-BE-TERMINATED", "STRUCTURED-REQ", "DO-IP-LOGIC-ADDRESS", "STEADY", "DROP-FRAME", "SWITCH-FLOW-METERING-ENTRY", "GENERAL-PARAMETER", "IEEE-1722-ACF-BUS-RAW-DATA-STREAM-CONSUMER-MAPPING", "RESPOND-AFTER-RESET", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "BE", "NOT-DEFINED", "ICV-SUPPORTED", "BINARY-MANIFEST-ITEM-DEFINITION", "DEFAULT", "LIN-CLUSTER", "ETHERNET-PRIORITY-REGENERATION", "SERVICE-SW-COMPONENT-TYPE", "MODE-TRANSITION", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "PHM-SUPERVISION", "ECUC-DEFINITION-COLLECTION", "TLS-CRYPTO-SERVICE-MAPPING", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "APMC-ABSTRACT-DEFINITION", "EVENT-STORAGE-DISABLED", "BT-REC-709", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "VARIABLE-DATA-PROTOTYPE", "STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE", "AUTOSAR-DATA-PROTOTYPE", "E-2-E-PROFILE-CONFIGURATION-SET", "I-SIGNAL-GROUP", "LOGICAL-EXPRESSION", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "DDS-EVENT-DEPLOYMENT", "BSW-SCHEDULER-NAME-PREFIX", "WORST-CASE-STACK-USAGE", "RPT-LEVEL-3", "BUILD-ACTION-ENTITY", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING", "DO-NOT-INCLUDE", "NO-AFFECT", "PRE--R-4--2", "NETWORK", "ECUC-DESTINATION-URI-DEF", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED", "AUTOSAR-VARIABLE-INSTANCE", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "REST-SERVICE-INTERFACE", "COUPLING-ELEMENT-ABSTRACT-DETAILS", "SW-COMPONENT-TYPE", "TD-EVENT-TT-CAN-CYCLE-START", "GETTER-SETTER", "PHM-ACTION-ITEM", "CRC-OPTIONAL", "ECC", "SECURITY-EVENT-MAPPING", "IK", "TD-EVENT-SERVICE-INSTANCE-METHOD", "IDENTIFIABLE", "ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS", "DDS-CP-SERVICE-INSTANCE", "APMC-ENUMERATION-PARAM-DEF", "WONT-CALL", "MO", "DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING", "CONFIG-DATA", "COLLECTION", "FM-FEATURE", "TD-EVENT-BSW", "J-1939-NM--SVCA", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "ISO-9141", "FDC-THRESHOLD", "PDU-R", "JA", "PERSISTENCY-DEPLOYMENT", "COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING", "LIGHT", "DOCUMENTATION-CONTEXT", "CRC-NOT-SUPPORTED", "TRIGGERED", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION", "NET", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "SDG-REFERENCE", "ATP-PROTOTYPE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "J-1939", "ANALYZED-EXECUTION-TIME", "CRYPTO-SERVICE-MANAGER", "INTERRUPT-CAT-1", "ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT", "BN", "TA", "IEEE-1722-TP-CRF-CONNECTION", "12", "STATE-MANAGEMENT-ACTION-LIST", "END-TO-END-PROTECTION", "INT-32-BIT", "HARDWARE-TEST-MANAGER", "INLINE-CONDITIONAL", "SECOND-TO-FIRST", "ECUC-FOREIGN-REFERENCE-DEF", "REDUNDANT-PER-KEY", "OBD-CONTROL-SERVICE-NEEDS", "OBD", "INCLUDE-BUT-DO-NOT-START", "OEM-BOOT-RESP-APP", "IS", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "TD-EVENT-BSW-MODULE", "REF-NONE", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "STRICT-MODE", "ABSTRACT-CAN-PHYSICAL-CHANNEL", "SESSION-HANDLING-ACTIVE", "SERVICE-INTERFACE-DEPLOYMENT", "DEFERRED", "16", "EXCLUDE-FROM-FLASH", "FDBAM", "AND", "APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-VALUE", "PRESENTATION-DISCRETE", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "LOWER-12-BIT", "PLATFORM-ACTION-ITEM", "TD-EVENT-OPERATION", "PDU", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "SVG", "ABSTRACT-REQUIRED-PORT-PROTOTYPE", "SECTION-NAME-PREFIX", "GN", "ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING", "MODE-DECLARATION-MAPPING", "MULTIPLE", "SOFTWARE-ACTIVATION-DEPENDENCY", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "OBSERVER", "DEDICATED", "MAC-SEC-PARTICIPANT-SET", "STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-OS-ACTION-ITEM", "MACRO", "ACTIVATE", "NO-BREAK", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "NO-SUPPORT", "PROCESS-IS-SELF-TERMINATING", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "DIAGNOSTIC-J-1939-SPN", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "DATA-INTERFACE", "FUNCTIONAL-CLUSTER-TO-DLT-LOG-SINK-MAPPING", "TARGET-CONTAINER", "I-SIGNAL-SENT-TO-COM", "KA", "CODE", "STATE-MANAGEMENT-STATE-REQUEST", "ERROR-DETECTION", "SL", "TRANSFORMATION-PROPS", "ABSTRACT-CAN-COMMUNICATION-CONTROLLER", "COM-GRANT", "SYMBOLIC-NAME-PROPS", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER", "DIAGNOSTIC-EDR-DATA-PROVIDER-MAPPING", "SUPERVISION-ENTITY", "ECUC-URI-REFERENCE-DEF", "LINK", "APPLIED-STANDARD", "BAYER-GRBG", "METHOD-MAPPING", "APMC-URI-PARAM-VALUE", "EOC-EXECUTABLE-ENTITY-REF-ABSTRACT", "ECUC-BOOLEAN-PARAM-DEF", "APMC-IP-V6-ADDRESS-PARAM-DEF", "SENT-UNTAGGED", "FLEXRAY-FRAME-TRIGGERING", "SHOW-PAGE", "ICV-NOT-VERIFIED", "APMC-PARAMETER-VALUE", "APPLICATION", "IGNITION", "CONSUMED-SERVICE-INSTANCE", "CAN-XL-PROPS", "ABSTRACT-EXECUTION-CONTEXT", "TRACE-SWITCH-CONFIG", "SUSPEND-TO-RAM-AWARE", "I-SIGNAL-I-PDU-GROUP", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "NEW-IS-WITHIN", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "FM-FEATURE-SELECTION-SET", "CLIENT-MAC-GENERATE", "V-2-X-FAC-USER-NEEDS", "CYCLIC", "FLEXRAY-AR-TP-CONFIG", "CMDT", "CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET", "NV-DATA-INTERFACE", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "ECUC-DEFINITION-ELEMENT", "SW-ADDR-METHOD", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "J-19398", "SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-ECU-RESET-CLASS", "DIAGNOSTIC-SOVD-CONFIGURATION", "ALL", "J-19391", "DCM-I-PDU", "DIAGNOSTIC-PORT-INTERFACE", "CALPRM", "ABSTRACT-ACCESS-POINT", "BLINK-OR-CONTINUOUS-ON-MODE", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "PNC-MAPPING-IDENT", "CRYPTO-NEED", "CHARGE-MANAGER-NEEDS", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "INTERFACE-MAPPING-SET", "LN", "REPETITIVE-EOC", "TTCAN-PHYSICAL-CHANNEL", "DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE", "MAX", "PORT-INTERFACE-DEFINITION", "USER-DEFINED-COMMUNICATION-CONNECTOR", "OBD-INFO-SERVICE-NEEDS", "VARIABLE-DATA-PROTOTYPE-SENT", "ACTION-ITEM", "STANDARD", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "REQUIRED-DDS-SERVICE-INSTANCE", "DEFAULT-TRIGGER", "TD-EVENT-SLLET-PORT", "COM-EVENT-GRANT-DESIGN", "BSW-INTERNAL-TRIGGERING-POINT", "IEEE-1722-TP-RVF-CONNECTION", "ABSTRACT-CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "BA", "GLOBAL-SUPERVISION-NEEDS", "DIAGNOSTIC-ECU-RESET", "DO-IP-FUNCTIONAL-CLUSTER-DESIGN", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "STIMULUS-SYNCHRONIZATION", "NO-OBD-SUPPORT", "AES-3-32-BIT", "EXECUTION-ORDER-CONSTRAINT", "BACKGROUND-EVENT", "ALLOW", "DIAGNOSTIC-MEMORY-DESTINATION", "IDSM-TRAFFIC-LIMITATION", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "OPTIONS", "DIAGNOSTIC-FREEZE-FRAME", "NOT-ACCESSIBLE", "DATA-EXCHANGE-POINT", "ADAPTIVE-METHOD-CALLED", "SW-GENERIC-AXIS-PARAM-TYPE", "CONSOLE", "SW-CODE-SYNTAX", "4-4-4", "J-1939-DCM", "PREEMPTABLE", "DEFAULT-IF-REVISION-UPDATE", "ECUC-VALUE-COLLECTION", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "FLAT-MAP", "16-KHZ", "MONITOR-MODE", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "OVERWRITE", "SO-AD-ROUTING-GROUP", "BOTTOM", "DEFINE-BY-IDENTIFIER", "PRE-COMPILE-TIME", "BIDIRECTIONAL", "OBD-RATIO-DENOMINATOR-NEEDS", "EXERCISE", "PHYSICAL-CAN-FD", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "REST-ABSTRACT-NUMERICAL-PROPERTY-DEF", "BI", "N-PDU", "LOG-AND-TRACE-INSTANTIATION", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "APPLICATION-SW-COMPONENT-TYPE", "PHM-ACTION-LIST", "BLINK-MODE", "DIAGNOSTIC-VALUE-NEEDS", "CONNECT", "BG", "CAN-20", "X-MMI", "DIAGNOSTIC-TEST-RESULT", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "DIAGNOSTIC-MULTIPLE-RESOURCE-PORT-MAPPING", "ACCES-PERRMISSION-SERVICE-CLASS", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "DEFAULT-ERROR-TRACER", "SRGB", "NM-CONFIG", "IP-SEC-CONFIG-PROPS", "IEEE-1722-RAW-DATA-STREAM-MAPPING", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "DIAGNOSTIC-DATA-TRANSFER", "MULTICORE-REENTRANT", "DIAGNOSTIC-INDICATOR-INTERFACE", "COUPLING-PORT-ABSTRACT-SHAPER", "COUPLING-ELEMENT-SWITCH-DETAILS", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "NEW-IS-LESS-OR-EQUAL", "PARAMETER-SW-COMPONENT-TYPE", "DIAG-RESPONSE", "DDS-SERVICE", "PRODUCER", "HW-PIN-GROUP", "SWITCH-STREAM-GATE-ENTRY", "CONCRETE-CLASS-TAILORING", "AP-APPLICATION-ERROR-SET", "CRYPTO-CERTIFICATE", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "CS", "UDP-NM-CLUSTER", "NEW-IS-GREATER-OR-EQUAL", "EU", "DO-IP-ROUTING-ACTIVATION", "DIAGNOSTIC-LOG-AND-TRACE", "OBD-DCY", "API-USE", "BSW-INTERRUPT-EVENT", "LA", "CYCLE-REPETITION-32", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "SETTER", "BSW-M-ENTRY-CALLED", "120", "REST-NUMBER-PROPERTY-DEF", "ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "ISO", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "TIMING-DESCRIPTION", "INTRUSION-DETECTION-SECURITY-MANAGEMENT", "PER-INSTANCE-MEMORY", "MODE-DECLARATION-GROUP", "DLNA", "DSA", "DIAGNOSTIC-COMMUNICATION-MANAGER", "DETAILED-BYPASSING-FILTERS", "BONJOUR", "MAC-LAYER-RAW-DATA-STREAM-INTERFACE", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "SHOW-TYPE", "APMC-DEFINITION-COLLECTION", "FRAME-TRANSMITTED-ON-BUS", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "APMC-INSTANCE-REFERENCE-VALUE", "ECUC-VALIDATION-CONDITION", "BSW-MODULE-ENTITY-STARTED", "SECURE-ON-BOARD-COMMUNICATION", "BSW-OS-TASK-EXECUTION-EVENT", "FLOAT-32-BIT", "POLY", "SUSPEND-TO-RAM-MODULE-INSTANTIATION", "GLOBAL-SUPERVISION", "RETURN-ON-EVENT-STOPPED", "UCM-SUBORDINATE-MODULE-INSTANTIATION", "WRITE-ONLY-ONCE", "INDEPENDENT-VLAN-LEARNING", "NM-INTERACTS-WITH-SM-MAPPING", "LINK-LOCAL", "DIAGNOSTIC-SERVICE-SW-MAPPING", "BOLD", "ECUC-PARAM-CONF-CONTAINER-DEF", "SYNCHRONIZED", "preserve", "ALLOCATOR", "HW-ATTRIBUTE-DEF", "DIAGNOSTIC-DATA-IDENTIFIER", "MAPPING-SCOPE-ECU", "NV-BLOCK-DESCRIPTOR", "DOMAIN-PARTICIPANT-USER-DATA-QOS", "TD-EVENT-SERVICE-INSTANCE-FIELD", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "SN", "NV-RAM-MANAGER", "PRIO-OCC", "TLS-CRYPTO-CIPHER-SUITE-PROPS", "DIAGNOSTIC-CONTROL-DTC-SETTING", "PNG", "ABSTRACT-IMPLEMENTATION-DATA-TYPE", "CRYPTO-CERTIFICATE-GROUP-INTERFACE", "VEHICLE-ROLLOUT-STEP", "SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY", "DDS-REQUIRED-SERVICE-INSTANCE", "CRC-NOT-VALIDATED", "TRANSFORMING-I-SIGNAL", "SOMEIP-TRANSFORMATION-PROPS", "DIAGNOSTIC-EVENT-MANAGER", "COM-FIND-SERVICE-GRANT-DESIGN", "TD-EVENT-TRIGGER", "LOGICAL-OR", "LT", "SLOW-FLASHING-MODE", "SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "REST-ENDPOINT-DELETE", "VERIFY", "BUILD-TYPE-DEBUG", "ENUMERATION-MAPPING-TABLE", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "SWC-TO-IMPL-MAPPING", "TRANSFORMER-ERROR-HANDLING", "ES", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "NO-HEADER", "GLOBAL-TIME-FR-SLAVE", "REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE", "FINISH", "SECURE-COMMUNICATION-PROPS-SET", "PC-AFFECTS-LT-AND-PB", "SYNCHRONIZED-TIME-BASE-PROVIDER", "TIMING-CONSTRAINT", "SERVICE-EVENT-DEPLOYMENT", "SW-CLASS-ATTR-IMPL", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "MODE-DECLARATION-SWITCH-INITIATED", "ICV-IGNORED", "GROSS", "X-MII", "TD-EVENT-I-SIGNAL", "I-4-G", "FUNCTION-GROUP-PORT-MAPPING", "ASYMMETRIC-TO-BYTE-ARRAY", "PRIMITIVE", "DIAGNOSTIC-REQUEST-UPLOAD", "ABSTRACT-ETHERNET-FRAME", "VALID", "4-1-1", "INCREASING", "RECOMMENDED-CONFIGURATION", "DIAGNOSTIC-IO-CONTROL", "DIAGNOSTIC-ROUTINE-SUBFUNCTION", "CODE-GENERATION-TIME", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "1-0", "BUS-MIRROR-CHANNEL-MAPPING-IP", "DATA-RECEIVED-EVENT", "IEEE-1722-TP-AV-CONNECTION", "NAND", "BUILD-ACTION", "ASYNCHRONOUS", "J-193911", "DIAGNOSTIC-SW-MAPPING", "SLAVE-PASSIVE", "25-24", "DLT-LOG-SINK", "ECUC-ENUMERATION-PARAM-DEF", "PROCESS-DESIGN", "ADAPTIVE-SERVICE-FIND-STARTED", "ECUC-MODULE-DEF", "INLINE", "ADDR-METHOD-SHORT-NAME", "DIAGNOSTIC-CONDITION-GROUP", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE", "J-19397", "BSW-DATA-RECEIVED-EVENT", "BSW-MODULE-ENTITY-TERMINATED", "ET", "SO", "PTP--IEEE-1588--2008", "COM-MANAGER", "STD-AXIS", "2500BASE-T1", "DROP", "OBD-PID-SERVICE-NEEDS", "SYNCHRONIZATION-POINT-CONSTRAINT", "ENHANCED-TRAFFIC-SHAPER", "PERSISTENCY-FILE-ELEMENT", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "DIAGNOSTIC-MONITOR-INTERFACE", "EXPRESS", "EXECUTABLE-TIMING", "DIAGNOSTIC-ROUTINE", "COMMAND-LINE-SHORT-FORM", "AA", "DIAGNOSTIC-SOVD-ARRAY-CONTENT-ELEMENT", "ROLL-BACK", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "APPLICATION-ARRAY-ELEMENT", "BSW-MODULE-ENTITY-ACTIVATED", "SHORT-HEADER", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "PHM-LOGICAL-EXPRESSION", "COMPILER", "APMC-MAC-ADDRESS-PARAM-DEF", "SW-SERVICE-PROTOTYPE", "TRIGGER-INTERFACE-MAPPING", "DIAGNOSTIC-CAPABILITY-ELEMENT", "LIN-COMMUNICATION-CONTROLLER", "READ-WRITE", "SM", "DIAGNOSTIC-J-1939-SPN-MAPPING", "RED-STOP-LAMP", "NOTHING", "BLOCK-STATE", "NO-SHOW-CATEGORY", "BAMCMDT", "ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING", "ARBITRATION", "STOP-TRIGGER", "LIN-COMMUNICATION-CONNECTOR", "DIAGNOSTIC-SOVD-CONTENT-GROUP", "XREF-TARGET", "STATE-MANAGEMENT-EM-ERROR-INTERFACE", "LIN-SLAVE", "PROCESSOR-CORE", "SW-AXIS-TYPE", "DIAGNOSTIC-UPLOAD-INTERFACE", "RO", "HINT", "CAT-2", "DIAGNOSTIC-PROTOCOL", "RAW-DATA-STREAM-INTERFACE", "FIBEX-ELEMENT", "APMC-INTEGER-PARAM-DEF", "IP-IAM-REMOTE-SUBJECT", "TIMING-CLOCK", "FIELD", "DEADLINE-SUPERVISION", "FO", "CONCRETE-PATTERN-EVENT-TRIGGERING", "BSW-MODULE-DEPENDENCY", "DE", "RETURN-ON-EVENT-CLEARED", "V-2-X-MANAGEMENT", "CONSTRAINT-TAILORING", "DEVELOPMENT", "XXG-MII", "CLIENT-DECRYPT", "NM-NODE", "DIAGNOSTIC-ENV-MODE-ELEMENT", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "PROPRIETARY-1", "DATA-SEND-COMPLETED-EVENT", "NEWLINE-IF-NECESSARY", "REPORT-BEFORE-INIT", "DIAGNOSTIC-SOVD-UPDATE", "SW-MC-INTERFACE", "CONSTANT-SPECIFICATION-MAPPING-SET", "TIME-SLAVE", "ECUC-QUERY", "DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE", "INTERNAL-TRIGGERING-POINT", "MODE-SWITCH-SENDER-COM-SPEC-PROPS", "COMMON", "STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "CAPTURE-ASYNCHRONOUS-TO-REPORTING", "ADAPTIVE-EVENT-SENT", "FLEXRAY-TP-CONFIG", "HARDWARE-TEST-NEEDS", "CONSUMED-EVENT-GROUP", "AUTONOMOUS", "TRANSIENT", "RAW", "EXECUTE", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "CAN-TP-ADDRESS", "EVENT-ACCEPTANCE-DISABLED", "DDS-TOPIC-ACCESS-RULE", "DIAGNOSTIC-SOVD-OPERATION", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "PHM-HEALTH-CHANNEL-INTERFACE", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "SDG-FOREIGN-REFERENCE", "USER-DEFINED-PHYSICAL-CHANNEL", "ECUC-ENUMERATION-LITERAL-DEF", "DDS-SERVICE-INSTANCE-EVENT-CP", "IEEE802-1AS-AUTOSAR", "PORT-PROTOTYPE-BLUEPRINT", "IEEE802-11P", "DELEGATION-SW-CONNECTOR", "SIGNAL-BASED-METHOD-DEPLOYMENT", "FUNCTIONAL-ADDRESS", "UCM-MODULE-INSTANTIATION", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "SERVICE-INSTANCE-TO-MACHINE-MAPPING", "ROTATE-90-CW", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT", "FLEXRAY-TP-PDU-POOL", "ATP-STRUCTURE-ELEMENT", "IEC-61937", "FLOAT", "RUNTIME-ERROR", "TD-EVENT-FRAME", "VERIFICATION", "FM-FEATURE-MAP-ASSERTION", "MODELED", "HUB", "FIT-TO-PAGE", "AP", "5", "DDS-CP-TOPIC", "24-25", "SW-VARIABLE-PROTOTYPE", "DIAGNOSTIC-IO-CONTROL-NEEDS", "FILE", "24-KHZ", "RTE-EVENT", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "IDENT-CAPTION", "KK", "CYCLE-REPETITION-2", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE", "DIAGNOSTIC-DATA-ELEMENT", "INTERGRITY-WITHOUT-CONFIDENTIALITY", "BASIC-SOFTWARE-MODE-MANAGER", "MEMORY-USAGE", "DEFLATE", "OBD-MONITOR-SERVICE-NEEDS", "REPLACE", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING", "ARBITRARY-EVENT-TRIGGERING", "BSW-VARIABLE-ACCESS", "WAIT-POINT", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "ACL-OPERATION", "V-2-X-NOT-SUPPORTED", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "SECONDARY-ECU", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "UCM-TO-TIME-BASE-RESOURCE-MAPPING", "HOOK", "ICV-OPTIONAL", "VEHICLE-PACKAGE", "FRAME-ETHERNET-RECEIVED-BY-IF", "SUPERVISION-MODE-CONDITION", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "DATA-FORMAT-ELEMENT-SCOPE", "BSW", "IDS-COMMON-ELEMENT", "RUNNABLE-ENTITY", "BUILD-ACTION-ENVIRONMENT", "SERVICE-NEEDS", "CRYPTO-SERVICE-MAPPING", "TCP-OPTION-FILTER-SET", "PORT-INTERFACE-MAPPING-SET", "2", "CALIBRATION-ONLINE", "IS-LESS-THAN-OR-EQUAL", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE", "MODE-DECLARATION-GROUP-PROTOTYPE", "J-1939-TP-NODE", "CRYPTO-KEY-MANAGEMENT-NEEDS", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "CURVE-AXIS", "APMC-FUNCTIONAL-CLUSTER-DEF", "NEW-IS-OUTSIDE", "PL", "FULL", "PERIODIC-RATE-FAST", "TRANSIENT-LOCAL", "CVC", "SW-BASE-TYPE", "DIAGNOSTIC-CONTRIBUTION-SET", "SOMEIP", "ERROR-TRACER-NEEDS", "SIGN-WITH-ORIGIN-AUTHENTICATION", "CRYPTO-KEY-SLOT-USER-DESIGN", "SWC-BSW-MAPPING", "J-1939-NM-NODE", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "RN", "SERVICE-INTERFACE-FIELD-MAPPING", "XYZ", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "SECURITY-EVENT-DEFINITION", "AUDIO-SAMPLE", "APMC-CHOICE-CONTAINER-DEF", "RIGHT", "DLT-ARGUMENT-PROPS-SET", "IEEE-1722-TP-ACF-CAN", "TE", "SECURITY-EVENT-CONTEXT-MAPPING", "TEST-FAILED-THIS-OPERATION-CYCLE", "RES_AXIS", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "APPLICATION-ERROR", "TRIGGER-INTERFACE", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "NEW-IS-GREATER", "BINARY-MANIFEST-RESOURCE-DEFINITION", "DLT-LOG-CHANNEL", "ECUC-REFERENCE-DEF", "ROTATE-90-CW-FIT-TO-TEXT", "CODEGENERATION", "NONE", "SESSION-HANDLING-INACTIVE", "PERSISTENCY-FILE-PROXY-INTERFACE", "PERSISTENCY-KEY-VALUE-DATABASE", "NEVER", "MR", "GL", "TRANSFORMATION-TECHNOLOGY", "J-1939-NM--SCA", "OVERRIDE", "DIAGNOSTIC-SOVD-METHOD-PRIMITIVE", "CONTAINER-I-PDU", "ABSTRACT-SECURITY-EVENT-FILTER", "DDS-CP-CONSUMED-SERVICE-INSTANCE", "XH", "IS-NOT-RELEVANT", "IDSM-CONTEXT-PROVIDER-INTERFACE", "DIAGNOSTIC-TROUBLE-CODE-UDS", "NL", "CONFIDENTIALITY-OFFSET--30", "RECT", "SAE-J-2012--DA", "72", "COM-METHOD-GRANT", "PR-PORT-PROTOTYPE", "CRC-VALIDATED", "DDS-RPC-SERVICE-DEPLOYMENT", "85", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "OS-TASK-EXECUTION-EVENT", "DIAGNOSTIC-SECURITY-LEVEL", "NO-SHOW-ALIAS-NAME", "VERTEX-OF-TARGET-CONTAINER", "CAN-FRAME-TRIGGERING", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "NO-SHOW-PAGE", "ECUC-QUERY-EXPRESSION", "SUPPORTS-BUFFER-LOCKING", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "OBD-DRIVING-CYCLE", "DATA-PROTOTYPE", "CLIENT-SERVER-INTERFACE-MAPPING", "EVENT-ACCEPTANCE-ENABLED", "240", "LINKER", "TD-EVENT-VFB", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "DIAGNOSTIC-AUTHENTICATION", "COMMUNICATION-INTRA-PARTITION", "WRITE-ONLY", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "DEFICIT-ROUND-ROBIN", "BSW-MGR-NEEDS", "LAND", "REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE", "SECURITY-EVENT-STATE-FILTER", "SIGNAL-BASED", "SYNCHRONOUS", "FLEXRAY-CLUSTER", "HA", "MC-FUNCTION", "WATCHDOG-ACTION-ITEM", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "ABSTRACT-CRYPTO-KEY-SLOT-INTERFACE", "ABSTRACT-EVENT", "DIAGNOSTIC-IUMPR", "1-1-001", "API", "COM-MGR-USER-NEEDS", "NO-SHOW-TYPE", "MULTILANGUAGE-REFERRABLE", "KO", "MIXED", "REST-BOOLEAN-PROPERTY-DEF", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "PC-AFFECTS-LT", "API-BASED", "BUS-MIRROR-CHANNEL-MAPPING", "WRONG-TRIGGER", "BSW-OPERATION-INVOKED-EVENT", "YO", "SERVICE-INTERFACE-PEDIGREE", "TUNNEL", "USES-LOGGING", "ORDINARY-EOC", "MASKED-NEW-DIFFERS-MASKED-OLD", "SOFTWARE-CLUSTER-REQUIREMENT", "APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-DEF", "DONT-INVALIDATE", "VIEW-MAP-SET", "VARIANT-POST-BUILD", "FJ", "CAN-COMMUNICATION-CONTROLLER", "REST-INTEGER-PROPERTY-DEF", "KEY-STORAGE", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "MS", "BSW-DEBUG-INFO", "DIAGNOSTIC-IUMPR-GROUP", "RUN-ONCE", "MANUAL-BY-PARTICIPANT", "PDU-TRIGGERING", "DLT-MESSAGE-COLLECTION-SET", "NM-HANDLE-ACTIVE-TO-FUNCTION-GROUP-STATE", "IS-LESS-THAN", "ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF", "DIAGNOSTIC-OPERATION-CYCLE", "ATTRIBUTE-TAILORING", "SERVER-AUTHENTICATE", "SERVICE-INTERFACE-METHOD-MAPPING", "REPORTS-EXECUTION-STATE", "J-193910", "COMMUNICATION-CONNECTOR", "I-PDU-SENT-TO-IF", "APMC-CONTAINER-ELEMENT-DEF", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "ECUC-MULTILINE-STRING-PARAM-DEF", "SOFTWARE-PACKAGE-STEP", "SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE", "LIN-TP-NODE", "DECREASING", "TRIGGER-UNICAST", "SERVICE-INTERFACE-TRIGGER-MAPPING", "PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING", "DIAGNOSTIC-EVENT-INFO-NEEDS", "AR", "APMC-ABSTRACT-FOREIGN-REFERENCE-DEF", "APPLICATION-PARTITION", "TIMING-CLOCK-SYNC-ACCURACY", "ARGUMENT-DATA-PROTOTYPE", "ETHERNET-COMMUNICATION-CONTROLLER", "AB", "AFTERMARKET", "PORT-PROTOTYPE", "TR", "STATE-MANAGEMENT-REQUEST-INTERFACE", "ABSTRACT-RAW-DATA-STREAM-INTERFACE", "CALIBRATION-PARAMETER-VALUE-SET", "NO-FLOAT", "APMC-ABSTRACT-INSTANCE-REFERENCE-DEF", "NM-CLUSTER", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "SECURED-PDU-HEADER-32-BIT", "TD-EVENT-COM", "EXECUTION-TIME", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "CAN-BE-REMOVED", "SINGLE-LANGUAGE-REFERRABLE", "STD_AXIS", "FM-FEATURE-MAP-CONDITION", "MASEKD-NEW-EQUALS-X", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "STATE-MANAGEMENT-ACTION-ITEM", "DIAGNOSTIC-SOVD-OPERATION-PORT-MAPPING", "ROOT-SW-COMPOSITION-PROTOTYPE", "WAIT-FOR-VEHICLE-SAFE-STATE", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "TRIGGERED-WITHOUT-REPETITION", "MEASUREMENT-POINT", "REQUIRES-CALLBACK-EXECUTION", "FORWARD-AS-IS", "COM-EVENT-GRANT", "SUSPEND-TO-RAM-TOLERANT", "CLIENT-ID-DEFINITION-SET", "APPLICATION-ARRAY-DATA-TYPE", "EID-USE-API", "EMISSION-RELATED-DTC", "ECUC-STRING-PARAM-DEF", "CONCRETE", "ECU-MANAGER", "PDUR-I-PDU-GROUP", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "BH", "DEFINE-BY-MEMORY-ADDRESS", "TIMEOUT", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "PERSISTENCY-FILE-STORAGE-INTERFACE", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "SERVICE-INTERFACE-EVENT-MAPPING", "LIN-PHYSICAL-CHANNEL", "I-SIGNAL", "APMC-INSTANCE-REFERENCE-DEF", "EPS", "TRACE-REFERRABLE", "CAN-BE-TERMINATED-AND-RESTARTED", "SW-SERVICE-ARG", "SM-INTERACTS-WITH-NM-MAPPING", "LIN-SPORADIC-FRAME", "SOCKET-CONNECTION-BUNDLE", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "IS-OK", "SPORADIC-EVENT-TRIGGERING", "IDSM-PROPERTIES", "NEWLINE", "HEALTH-CHANNEL-EXTERNAL-STATUS", "IN", "SECURED-I-PDU", "AUTHENTICATE", "LOGICAL-AND", "CRC-IGNORED", "J-1939-NODE", "CYCLE-REPETITION-1", "MODE-DECLARATION", "TD-EVENT-BSW-MODE-DECLARATION", "ADAPTIVE-MODULE-INSTANTIATION", "APPLICATION-ASSOC-MAP-DATA-TYPE", "VARIABLE-ACCESS", "SLAVE", "MEASURED-HEAP-USAGE", "PROCESS-IS-NOT-SELF-TERMINATING", "I-PDU-PORT", "ALL-INDICES-SAME-ARRAY-SIZE", "SOMEIP-REMOTE-UNICAST-CONFIG", "MONOCHROME", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "HEALTH-CHANNEL-EXTERNAL-MODE", "MASTER-ECU", "DIAGNOSTIC-SOVD-OPERATION-INTERFACE", "ACL-PERMISSION", "COM-TRIGGER-GRANT-DESIGN", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "CSERS", "STD", "NM-PDU", "SOMEIP-TP-CONFIG", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION", "WATCH-TRIGGER-GAP", "DHCPV-6", "WILL-RECEIVE", "BSW-ENTRY-RELATIONSHIP-SET", "SERVICE-METHOD-DEPLOYMENT", "APMC-PARAMETER-DEF", "SAE-J-1939--73", "DIAGNOSTIC-TRANSFER-EXIT", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "WRITE", "STATE-MANAGEMENT-PORT-INTERFACE", "WATCHDOG-PHM-ACTION-ITEM", "STATE-MANAGEMENT-NM-ACTION-ITEM", "MT", "CENTER", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "VARIANT-POST-BUILD-LOADABLE", "SPEC-ELEMENT-REFERENCE"];

    /// derive an enum entry from an input string using a perfect hash function
    ///
    /// # Errors
    ///
    /// [`ParseEnumItemError`]: The input string did not match the name of any enum item
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseEnumItemError> {
        #[rustfmt::skip]
        static DISPLACEMENTS: [(u16, u16); 583] = [(0, 0), (0, 20), (0, 525), (0, 122), (0, 4), (0, 315), (0, 10), (0, 129), (0, 23), (0, 113), (0, 713), (0, 13), (0, 4), (0, 103), (0, 85), (0, 47), (0, 69), (0, 130), (0, 2), (0, 0), (0, 463), (0, 1832), (0, 6), (0, 1254), (0, 12), (0, 767), (0, 32), (0, 118), (0, 41), (0, 1241), (0, 96), (0, 1074), (0, 789), (0, 148), (0, 12), (0, 213), (0, 205), (0, 114), (0, 1), (0, 47), (0, 5), (0, 109), (0, 28), (0, 16), (0, 52), (0, 2), (0, 326), (0, 361), (0, 236), (0, 4), (0, 50), (0, 102), (0, 586), (0, 2), (0, 276), (0, 532), (0, 49), (0, 3), (0, 262), (0, 194), (0, 522), (0, 475), (0, 121), (0, 1), (0, 21), (0, 5), (0, 33), (0, 181), (0, 49), (0, 25), (0, 14), (0, 44), (0, 7), (0, 1097), (0, 591), (0, 344), (0, 332), (0, 36), (0, 7), (0, 226), (0, 0), (0, 526), (0, 0), (0, 1351), (0, 873), (0, 233), (0, 51), (0, 0), (0, 120), (0, 1008), (0, 4), (1, 1742), (0, 151), (0, 19), (0, 277), (0, 3), (0, 0), (0, 32), (0, 482), (0, 171), (0, 0), (0, 1), (0, 78), (0, 9), (0, 62), (0, 61), (0, 200), (0, 4), (0, 104), (0, 30), (0, 8), (0, 35), (0, 119), (0, 0), (0, 307), (0, 370), (0, 1147), (0, 394), (0, 0), (0, 33), (0, 44), (0, 12), (0, 3), (0, 322), (0, 1), (0, 26), (0, 6), (0, 288), (0, 274), (0, 517), (0, 161), (0, 57), (0, 11), (0, 22), (0, 592), (0, 0), (1, 238), (0, 180), (0, 202), (0, 1), (0, 2407), (0, 0), (0, 18), (0, 29), (0, 31), (0, 31), (0, 307), (0, 364), (0, 105), (0, 21), (0, 143), (0, 1287), (0, 302), (0, 13), (0, 257), (0, 10), (0, 4), (0, 0), (0, 1752), (0, 8), (0, 0), (0, 96), (0, 2563), (0, 2674), (0, 176), (0, 386), (0, 282), (0, 414), (0, 29), (0, 217), (0, 7), (0, 2447), (0, 18), (0, 30), (0, 278), (0, 1609), (0, 1584), (0, 0), (0, 99), (0, 35), (0, 134), (0, 64), (0, 27), (0, 101), (0, 9), (0, 596), (0, 731), (0, 409), (0, 0), (0, 2), (0, 1908), (0, 3), (0, 755), (0, 178), (0, 1), (0, 137), (0, 911), (0, 222), (0, 1299), (0, 1272), (0, 18), (0, 531), (0, 0), (0, 35), (0, 2105), (0, 24), (0, 2024), (0, 2), (0, 0), (0, 17), (0, 0), (0, 76), (0, 27), (0, 184), (0, 6), (0, 138), (0, 13), (0, 1249), (0, 4), (0, 2), (0, 1), (0, 5), (0, 247), (0, 14), (0, 24), (0, 508), (0, 694), (0, 1461), (0, 4), (0, 229), (0, 3), (0, 20), (0, 22), (0, 603), (0, 32), (0, 4), (0, 48), (0, 955), (0, 4), (0, 50), (0, 0), (0, 303), (0, 9), (2, 427), (0, 19), (0, 4), (0, 18), (0, 2), (0, 1), (0, 7), (0, 10), (0, 0), (0, 2), (0, 45), (1, 11), (0, 2), (0, 2), (0, 1), (0, 363), (0, 2), (0, 10), (0, 156), (0, 58), (0, 1), (0, 986), (0, 9), (0, 56), (0, 0), (0, 5), (0, 59), (0, 1), (0, 518), (0, 274), (0, 12), (0, 63), (0, 79), (0, 43), (0, 1608), (0, 8), (0, 327), (0, 708), (0, 9), (0, 13), (0, 0), (0, 78), (0, 20), (2, 2894), (0, 743), (0, 523), (0, 18), (0, 4), (0, 77), (0, 893), (0, 95), (0, 22), (0, 68), (0, 263), (0, 621), (0, 855), (0, 63), (0, 1), (0, 53), (0, 59), (0, 1), (0, 42), (0, 173), (0, 699), (0, 5), (0, 804), (0, 5), (0, 589), (0, 806), (0, 223), (0, 1204), (0, 1037), (0, 7), (0, 72), (0, 3), (0, 119), (0, 21), (0, 3), (0, 39), (0, 40), (0, 45), (0, 17), (0, 22), (0, 0), (0, 17), (0, 228), (0, 2), (0, 717), (0, 115), (0, 2877), (0, 973), (0, 9), (0, 9), (0, 64), (0, 59), (0, 115), (0, 50), (0, 691), (0, 2210), (0, 0), (0, 1338), (0, 0), (0, 1683), (0, 565), (0, 15), (0, 6), (0, 17), (0, 871), (0, 35), (0, 173), (0, 4), (0, 175), (0, 182), (0, 138), (0, 403), (0, 0), (0, 29), (0, 2811), (0, 477), (0, 1), (0, 1633), (0, 57), (2, 937), (0, 9), (0, 15), (0, 2), (0, 45), (0, 46), (0, 35), (0, 0), (0, 2455), (0, 1625), (1, 50), (0, 7), (0, 7), (0, 151), (0, 0), (0, 1), (0, 175), (0, 14), (0, 1573), (0, 170), (0, 56), (0, 1848), (0, 162), (0, 2), (1, 1646), (0, 2), (0, 4), (1, 2695), (0, 2011), (0, 0), (0, 44), (0, 476), (0, 616), (0, 470), (0, 2409), (0, 583), (0, 359), (0, 78), (0, 2050), (0, 2029), (0, 282), (0, 1595), (0, 299), (0, 8), (0, 127), (0, 643), (0, 19), (0, 2), (1, 1086), (1, 451), (0, 10), (1, 2274), (0, 347), (0, 1), (1, 1102), (0, 233), (0, 2166), (0, 6), (1, 1125), (0, 203), (0, 2595), (0, 2), (0, 100), (0, 688), (0, 30), (0, 423), (1, 592), (0, 67), (0, 976), (0, 1), (2, 2630), (2, 1772), (0, 1312), (0, 267), (0, 726), (0, 1156), (0, 898), (3, 1197), (0, 890), (0, 1), (1, 2466), (0, 139), (0, 68), (0, 2148), (0, 105), (0, 4), (0, 2712), (0, 112), (0, 605), (5, 375), (0, 2393), (6, 421), (0, 1), (0, 0), (0, 656), (0, 0), (0, 1390), (0, 270), (0, 857), (0, 0), (0, 0), (0, 348), (0, 116), (0, 21), (1, 1058), (15, 783), (3, 928), (0, 0), (0, 102), (0, 655), (4, 660), (0, 1973), (0, 49), (0, 30), (0, 11), (0, 2624), (0, 108), (0, 281), (2, 921), (2, 2900), (0, 13), (0, 1062), (0, 47), (0, 760), (0, 28), (1, 2130), (0, 2392), (2, 2806), (0, 2350), (0, 2), (6, 975), (0, 16), (0, 22), (0, 2061), (0, 275), (0, 24), (0, 4), (0, 0), (0, 2490), (0, 4), (0, 1729), (0, 10), (0, 4), (0, 6), (0, 0), (1, 1310), (0, 572), (0, 964), (6, 1352), (0, 38), (0, 1397), (0, 138), (1, 834), (1, 1942), (0, 71), (0, 131), (0, 32), (0, 522), (0, 100), (1, 280), (0, 59), (2, 355), (0, 121), (0, 468), (0, 681), (0, 141), (0, 3), (0, 5), (0, 77), (0, 372), (0, 24), (0, 882), (0, 12), (0, 5), (0, 63), (0, 684), (0, 574), (0, 732), (0, 213), (0, 104), (7, 1769), (0, 243), (0, 926), (0, 26), (0, 2364), (0, 165), (0, 11), (0, 118), (1, 1462), (2, 1061), (0, 3), (8, 332), (12, 2444), (0, 867), (0, 46), (0, 1), (1, 434), (0, 153), (0, 27), (2, 1822), (0, 145), (0, 1000), (0, 795), (0, 1829), (0, 2083), (0, 7), (0, 303), (0, 140), (0, 204), (0, 0), (0, 1078), (0, 19), (0, 48), (0, 125), (0, 21), (0, 1564), (1, 440), (0, 40)];

        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 583) as usize];
        let item_idx = u32::from(d2)
            .wrapping_add(f1.wrapping_mul(u32::from(d1)))
            .wrapping_add(f2) as usize
            % 2915;
        if EnumItem::STRING_TABLE[item_idx].as_bytes() != input {
            return Err(ParseEnumItemError);
        }
        Ok(unsafe { core::mem::transmute::<u16, Self>(item_idx as u16) })
    }

    /// get the str corresponding to an item
    ///
    /// The returned &str has static lifetime, becasue it is a reference to an entry in a list of constants
    #[must_use]
    pub fn to_str(&self) -> &'static str {
        EnumItem::STRING_TABLE[*self as usize]
    }
}

impl core::str::FromStr for EnumItem {
    type Err = ParseEnumItemError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(input.as_bytes())
    }
}

impl core::fmt::Debug for EnumItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(EnumItem::STRING_TABLE[*self as usize])
    }
}

impl core::fmt::Display for EnumItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(EnumItem::STRING_TABLE[*self as usize])
    }
}
