use crate::hashfunc;

#[derive(Debug)]
/// The error type `ParseElementNameError` is returned when `from_str()` / `parse()` fails for `ElementName`
pub struct ParseElementNameError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
#[non_exhaustive]
#[rustfmt::skip]
/// Enum of all element names in Autosar
pub enum ElementName {
    /// A-2-L-DISPLAY-TEXT
    A2LDisplayText                                                         = 4571,
    /// AAF-AES-3-DATA-TYPE
    AafAes3DataType                                                        = 3976,
    /// AAF-FORMAT
    AafFormat                                                              = 6203,
    /// AAF-NOMINAL-RATE
    AafNominalRate                                                         = 2810,
    /// ABBR-NAME
    AbbrName                                                               = 1138,
    /// ABS
    Abs                                                                    = 1508,
    /// ABSOLUTE
    Absolute                                                               = 4794,
    /// ABSOLUTE-TOLERANCE
    AbsoluteTolerance                                                      = 1266,
    /// ABSOLUTELY-SCHEDULED-TIMINGS
    AbsolutelyScheduledTimings                                             = 1027,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 2087,
    /// ACCEPT-ENCODING
    AcceptEncoding                                                         = 6385,
    /// ACCEPT-VARIABLE-DA
    AcceptVariableDa                                                       = 914,
    /// ACCEPT-VARIABLE-SA
    AcceptVariableSa                                                       = 4740,
    /// ACCEPTANCE-FIELD
    AcceptanceField                                                        = 3823,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF
    AcceptedCryptoCipherSuiteWithPskRef                                    = 1017,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS
    AcceptedCryptoCipherSuiteWithPskRefs                                   = 1588,
    /// ACCEPTED-REMOTE-CERTIFICATE-REF
    AcceptedRemoteCertificateRef                                           = 1406,
    /// ACCEPTED-REMOTE-CERTIFICATE-REFS
    AcceptedRemoteCertificateRefs                                          = 3728,
    /// ACCEPTED-STARTUP-RANGE
    AcceptedStartupRange                                                   = 4386,
    /// ACCEPTS-ENCODINGS
    AcceptsEncodings                                                       = 6064,
    /// ACCESS-CONTROL
    AccessControl                                                          = 854,
    /// ACCESS-COUNT
    AccessCount                                                            = 1321,
    /// ACCESS-COUNT-SET
    AccessCountSet                                                         = 2162,
    /// ACCESS-COUNT-SETS
    AccessCountSets                                                        = 5616,
    /// ACCESS-COUNTS
    AccessCounts                                                           = 1804,
    /// ACCESS-DATA-RECORD-SIZE
    AccessDataRecordSize                                                   = 1849,
    /// ACCESS-MODE
    AccessMode                                                             = 4416,
    /// ACCESS-PERMISSION-REF
    AccessPermissionRef                                                    = 3407,
    /// ACCESS-PERMISSION-VALIDITY
    AccessPermissionValidity                                               = 5421,
    /// ACCESS-POINT-REF
    AccessPointRef                                                         = 4783,
    /// ACCESSED-DATA-PROTOTYPE-IREF
    AccessedDataPrototypeIref                                              = 2619,
    /// ACCESSED-MODE-GROUPS
    AccessedModeGroups                                                     = 1414,
    /// ACCESSED-PARAMETER
    AccessedParameter                                                      = 678,
    /// ACCESSED-VARIABLE
    AccessedVariable                                                       = 4960,
    /// ACCESSED-VARIABLE-REF
    AccessedVariableRef                                                    = 2249,
    /// ACCURACY
    Accuracy                                                               = 3183,
    /// ACCURACY-EXT
    AccuracyExt                                                            = 578,
    /// ACCURACY-INT
    AccuracyInt                                                            = 2943,
    /// ACF-BUS-PART-REF
    AcfBusPartRef                                                          = 5310,
    /// ACF-BUS-PART-REFS
    AcfBusPartRefs                                                         = 1004,
    /// ACF-BUS-REF
    AcfBusRef                                                              = 221,
    /// ACF-BUS-REFS
    AcfBusRefs                                                             = 3676,
    /// ACF-MAX-TRANSIT-TIME
    AcfMaxTransitTime                                                      = 371,
    /// ACF-PARTS
    AcfParts                                                               = 5543,
    /// ACF-TRANSPORTED-BUSS
    AcfTransportedBuss                                                     = 3902,
    /// ACK-REQUEST
    AckRequest                                                             = 5159,
    /// ACK-TYPE
    AckType                                                                = 3134,
    /// ACL-CONTEXT
    AclContext                                                             = 327,
    /// ACL-CONTEXTS
    AclContexts                                                            = 538,
    /// ACL-OBJECT-CLASS
    AclObjectClass                                                         = 4887,
    /// ACL-OBJECT-CLASSS
    AclObjectClasss                                                        = 2927,
    /// ACL-OBJECT-REF
    AclObjectRef                                                           = 4824,
    /// ACL-OBJECT-REFS
    AclObjectRefs                                                          = 3095,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 1182,
    /// ACL-OPERATION
    AclOperation                                                           = 826,
    /// ACL-OPERATION-REF
    AclOperationRef                                                        = 6047,
    /// ACL-OPERATION-REFS
    AclOperationRefs                                                       = 6315,
    /// ACL-PERMISSION
    AclPermission                                                          = 4669,
    /// ACL-ROLE
    AclRole                                                                = 4764,
    /// ACL-ROLE-REF
    AclRoleRef                                                             = 2583,
    /// ACL-ROLE-REFS
    AclRoleRefs                                                            = 4664,
    /// ACL-SCOPE
    AclScope                                                               = 4471,
    /// ACTION
    Action                                                                 = 3212,
    /// ACTION-ITEM-LISTS
    ActionItemLists                                                        = 4504,
    /// ACTION-ITEM-REF
    ActionItemRef                                                          = 1287,
    /// ACTION-ITEMS
    ActionItems                                                            = 4921,
    /// ACTION-LIST
    ActionList                                                             = 807,
    /// ACTION-LIST-EXECUTION
    ActionListExecution                                                    = 5827,
    /// ACTION-LIST-ITEM-REF
    ActionListItemRef                                                      = 497,
    /// ACTION-LIST-ITEM-REFS
    ActionListItemRefs                                                     = 1220,
    /// ACTION-LIST-PROCESSING-FAILED-ERROR-REF
    ActionListProcessingFailedErrorRef                                     = 6376,
    /// ACTION-LISTS
    ActionLists                                                            = 2645,
    /// ACTION-POINT-OFFSET
    ActionPointOffset                                                      = 1029,
    /// ACTION-TYPE
    ActionType                                                             = 95,
    /// ACTIONS
    Actions                                                                = 3338,
    /// ACTIVATE-AT-START
    ActivateAtStart                                                        = 8,
    /// ACTIVATION
    Activation                                                             = 2176,
    /// ACTIVATION-ACTION
    ActivationAction                                                       = 162,
    /// ACTIVATION-POINTS
    ActivationPoints                                                       = 5693,
    /// ACTIVATION-REASON-REPRESENTATION-REF
    ActivationReasonRepresentationRef                                      = 400,
    /// ACTIVATION-REASONS
    ActivationReasons                                                      = 4472,
    /// ACTIVATION-SWITCH
    ActivationSwitch                                                       = 6566,
    /// ACTIVE-SUPERVISION-REF
    ActiveSupervisionRef                                                   = 6309,
    /// ACTIVE-SUPERVISION-REFS
    ActiveSupervisionRefs                                                  = 727,
    /// ACTUAL-EVENT-REF
    ActualEventRef                                                         = 4857,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 3029,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 4939,
    /// ADAPTIVE-FIREWALL-MODULE-INSTANTIATION
    AdaptiveFirewallModuleInstantiation                                    = 4743,
    /// ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING
    AdaptiveFirewallToPortPrototypeMapping                                 = 238,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 10,
    /// ADD-INFO-5
    AddInfo5                                                               = 2342,
    /// ADDITIONAL-INFORMATION
    AdditionalInformation                                                  = 1494,
    /// ADDITIONAL-NATIVE-TYPE-QUALIFIER
    AdditionalNativeTypeQualifier                                          = 4425,
    /// ADDRESS
    Address                                                                = 468,
    /// ADDRESS-CLAIM-ENABLED
    AddressClaimEnabled                                                    = 6508,
    /// ADDRESS-CONFIGURATION-CAPABILITY
    AddressConfigurationCapability                                         = 545,
    /// ADDRESS-RANGE-LOWER-BOUND
    AddressRangeLowerBound                                                 = 3735,
    /// ADDRESS-RANGE-UPPER-BOUND
    AddressRangeUpperBound                                                 = 5047,
    /// ADDRESS-SEMANTICS
    AddressSemantics                                                       = 4326,
    /// ADDRESSING-FORMAT
    AddressingFormat                                                       = 58,
    /// ADMIN-DATA
    AdminData                                                              = 5240,
    /// ADMIT-WITHOUT-MESSAGE-ID
    AdmitWithoutMessageId                                                  = 4150,
    /// AES-3-DATA-TYPE-H
    Aes3DataTypeH                                                          = 4049,
    /// AES-3-DATA-TYPE-L
    Aes3DataTypeL                                                          = 2988,
    /// AFFECTED-APPLICATION
    AffectedApplication                                                    = 530,
    /// AFFECTED-BSW-MODULE
    AffectedBswModule                                                      = 6052,
    /// AFFECTED-BY-CONTROL-DTC-SETTING
    AffectedByControlDtcSetting                                            = 4108,
    /// AFFECTED-FUNCTIONAL-CLUSTER
    AffectedFunctionalCluster                                              = 5430,
    /// AFFECTED-REF
    AffectedRef                                                            = 4147,
    /// AFFECTED-REFS
    AffectedRefs                                                           = 5823,
    /// AFFECTED-STATE-IREF
    AffectedStateIref                                                      = 1725,
    /// AFFECTION-KIND
    AffectionKind                                                          = 6575,
    /// AGE
    Age                                                                    = 560,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 3303,
    /// AGGREGATION
    Aggregation                                                            = 1427,
    /// AGGREGATION-CONDITION
    AggregationCondition                                                   = 4781,
    /// AGGREGATION-REF
    AggregationRef                                                         = 5978,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 1937,
    /// AGING-ALLOWED
    AgingAllowed                                                           = 2452,
    /// AGING-CYCLES
    AgingCycles                                                            = 4032,
    /// AGING-REF
    AgingRef                                                               = 571,
    /// AGING-REQUIRES-TESTED-CYCLE
    AgingRequiresTestedCycle                                               = 1320,
    /// AH-CIPHER-SUITE-NAME
    AhCipherSuiteName                                                      = 4277,
    /// AH-CIPHER-SUITE-NAMES
    AhCipherSuiteNames                                                     = 6424,
    /// ALGORITHM-DESCRIPTIONS
    AlgorithmDescriptions                                                  = 4059,
    /// ALGORITHM-FAMILY
    AlgorithmFamily                                                        = 941,
    /// ALGORITHM-MODE
    AlgorithmMode                                                          = 649,
    /// ALGORITHM-SECONDARY-FAMILY
    AlgorithmSecondaryFamily                                               = 6649,
    /// ALIAS-EVENT-REF
    AliasEventRef                                                          = 2787,
    /// ALIAS-NAME-ASSIGNMENT
    AliasNameAssignment                                                    = 3002,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 4435,
    /// ALIAS-NAMES
    AliasNames                                                             = 41,
    /// ALIGNMENT
    Alignment                                                              = 3216,
    /// ALIVE-CHECK-RESPONSE-TIMEOUT
    AliveCheckResponseTimeout                                              = 6342,
    /// ALIVE-REFERENCE-CYCLE
    AliveReferenceCycle                                                    = 4497,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 2303,
    /// ALIVE-SUPERVISIONS
    AliveSupervisions                                                      = 134,
    /// ALIVE-TIMEOUT
    AliveTimeout                                                           = 2183,
    /// ALL-CHANNELS-REF
    AllChannelsRef                                                         = 6152,
    /// ALL-CHANNELS-REFS
    AllChannelsRefs                                                        = 304,
    /// ALL-NM-MESSAGES-KEEP-AWAKE
    AllNmMessagesKeepAwake                                                 = 6293,
    /// ALL-PHYSICAL-CHANNELS
    AllPhysicalChannels                                                    = 3273,
    /// ALL-PHYSICAL-CHANNELS-REF
    AllPhysicalChannelsRef                                                 = 898,
    /// ALLOCATE-SHADOW-COPY
    AllocateShadowCopy                                                     = 5847,
    /// ALLOCATOR
    Allocator                                                              = 5953,
    /// ALLOCATOR-REF
    AllocatorRef                                                           = 1815,
    /// ALLOW-CONTENT-TYPE-CHANGE
    AllowContentTypeChange                                                 = 4621,
    /// ALLOW-DYNAMIC-L-SDU-LENGTH
    AllowDynamicLSduLength                                                 = 3259,
    /// ALLOW-HALT-DUE-TO-CLOCK
    AllowHaltDueToClock                                                    = 4334,
    /// ALLOW-MASTER-RATE-CORRECTION
    AllowMasterRateCorrection                                              = 4826,
    /// ALLOW-PASSIVE-TO-ACTIVE
    AllowPassiveToActive                                                   = 3461,
    /// ALLOW-PLAIN-TEXT-EXPORT
    AllowPlainTextExport                                                   = 1447,
    /// ALLOW-PLAIN-TEXT-IMPORT
    AllowPlainTextImport                                                   = 5605,
    /// ALLOW-PROVIDER-RATE-CORRECTION
    AllowProviderRateCorrection                                            = 1157,
    /// ALLOW-UNAUTHENTICATED-PARTICIPANTS
    AllowUnauthenticatedParticipants                                       = 3070,
    /// ALLOWED-I-PV-6-EXT-HEADER
    AllowedIPv6ExtHeader                                                   = 4465,
    /// ALLOWED-I-PV-6-EXT-HEADERS
    AllowedIPv6ExtHeaders                                                  = 6448,
    /// ALLOWED-I-PV-6-EXT-HEADERS-REF
    AllowedIPv6ExtHeadersRef                                               = 5796,
    /// ALLOWED-KEYSLOT-USAGE
    AllowedKeyslotUsage                                                    = 2200,
    /// ALLOWED-SERVICE-CONSUMERS
    AllowedServiceConsumers                                                = 1154,
    /// ALLOWED-SERVICE-PROVIDERS
    AllowedServiceProviders                                                = 3516,
    /// ALLOWED-TCP-OPTION
    AllowedTcpOption                                                       = 2585,
    /// ALLOWED-TCP-OPTIONS
    AllowedTcpOptions                                                      = 5833,
    /// ALLOWED-TCP-OPTIONS-REF
    AllowedTcpOptionsRef                                                   = 3199,
    /// ALTERNATIVE-NAME
    AlternativeName                                                        = 4481,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 5198,
    /// ANNOTATION
    Annotation                                                             = 3499,
    /// ANNOTATION-ORIGIN
    AnnotationOrigin                                                       = 3593,
    /// ANNOTATION-TEXT
    AnnotationText                                                         = 505,
    /// ANNOTATIONS
    Annotations                                                            = 5287,
    /// ANONYMOUS
    Anonymous                                                              = 2510,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 3806,
    /// AP-APPLICATION-ENDPOINTS
    ApApplicationEndpoints                                                 = 1762,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 693,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 1143,
    /// AP-APPLICATION-ERROR-REF
    ApApplicationErrorRef                                                  = 3136,
    /// AP-APPLICATION-ERROR-REFS
    ApApplicationErrorRefs                                                 = 2993,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 4588,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 1379,
    /// API-PRINCIPLE
    ApiPrinciple                                                           = 2997,
    /// API-SERVICE-PREFIX
    ApiServicePrefix                                                       = 5316,
    /// APMC-BOOLEAN-PARAM-DEF
    ApmcBooleanParamDef                                                    = 4251,
    /// APMC-CHOICE-CONTAINER-DEF
    ApmcChoiceContainerDef                                                 = 2171,
    /// APMC-CHOICE-CONTAINER-REFERENCE-DEF
    ApmcChoiceContainerReferenceDef                                        = 6304,
    /// APMC-CONTAINER-REFERENCE-DEF
    ApmcContainerReferenceDef                                              = 1394,
    /// APMC-CONTAINER-REFERENCE-VALUE
    ApmcContainerReferenceValue                                            = 6096,
    /// APMC-CONTAINER-VALUE
    ApmcContainerValue                                                     = 4979,
    /// APMC-DEFINITION-COLLECTION
    ApmcDefinitionCollection                                               = 2453,
    /// APMC-ENUMERATION-LITERAL-DEF
    ApmcEnumerationLiteralDef                                              = 6628,
    /// APMC-ENUMERATION-PARAM-DEF
    ApmcEnumerationParamDef                                                = 4593,
    /// APMC-FLOAT-PARAM-DEF
    ApmcFloatParamDef                                                      = 5133,
    /// APMC-FOREIGN-REFERENCE-DEF
    ApmcForeignReferenceDef                                                = 1879,
    /// APMC-FOREIGN-REFERENCE-VALUE
    ApmcForeignReferenceValue                                              = 3414,
    /// APMC-FUNCTIONAL-CLUSTER-DEF
    ApmcFunctionalClusterDef                                               = 3860,
    /// APMC-FUNCTIONAL-CLUSTER-VALUE
    ApmcFunctionalClusterValue                                             = 4026,
    /// APMC-INSTANCE-REFERENCE-DEF
    ApmcInstanceReferenceDef                                               = 953,
    /// APMC-INSTANCE-REFERENCE-VALUE
    ApmcInstanceReferenceValue                                             = 239,
    /// APMC-INTEGER-PARAM-DEF
    ApmcIntegerParamDef                                                    = 2363,
    /// APMC-IP-V4-ADDRESS-PARAM-DEF
    ApmcIpV4AddressParamDef                                                = 210,
    /// APMC-IP-V4-ADDRESS-PARAM-VALUE
    ApmcIpV4AddressParamValue                                              = 333,
    /// APMC-IP-V6-ADDRESS-PARAM-DEF
    ApmcIpV6AddressParamDef                                                = 2739,
    /// APMC-IP-V6-ADDRESS-PARAM-VALUE
    ApmcIpV6AddressParamValue                                              = 1623,
    /// APMC-MAC-ADDRESS-PARAM-DEF
    ApmcMacAddressParamDef                                                 = 3124,
    /// APMC-MAC-ADDRESS-PARAM-VALUE
    ApmcMacAddressParamValue                                               = 5162,
    /// APMC-NUMERICAL-PARAM-VALUE
    ApmcNumericalParamValue                                                = 5138,
    /// APMC-PARAM-CONF-CONTAINER-DEF
    ApmcParamConfContainerDef                                              = 3180,
    /// APMC-REVISION-LABEL-PARAM-DEF
    ApmcRevisionLabelParamDef                                              = 4773,
    /// APMC-REVISION-LABEL-PARAM-VALUE
    ApmcRevisionLabelParamValue                                            = 5882,
    /// APMC-STRING-PARAM-DEF
    ApmcStringParamDef                                                     = 5524,
    /// APMC-STRONG-REVISION-LABEL-PARAM-DEF
    ApmcStrongRevisionLabelParamDef                                        = 4825,
    /// APMC-STRONG-REVISION-LABEL-PARAM-VALUE
    ApmcStrongRevisionLabelParamValue                                      = 2151,
    /// APMC-TEXTUAL-PARAM-VALUE
    ApmcTextualParamValue                                                  = 3131,
    /// APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-DEF
    ApmcUpstreamDocForeignReferenceDef                                     = 4437,
    /// APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-VALUE
    ApmcUpstreamDocForeignReferenceValue                                   = 660,
    /// APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-DEF
    ApmcUpstreamDocInstanceReferenceDef                                    = 470,
    /// APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-VALUE
    ApmcUpstreamDocInstanceReferenceValue                                  = 2696,
    /// APMC-URI-FOREIGN-REFERENCE-DEF
    ApmcUriForeignReferenceDef                                             = 6543,
    /// APMC-URI-FOREIGN-REFERENCE-VALUE
    ApmcUriForeignReferenceValue                                           = 3682,
    /// APMC-URI-INSTANCE-REFERENCE-DEF
    ApmcUriInstanceReferenceDef                                            = 4679,
    /// APMC-URI-INSTANCE-REFERENCE-VALUE
    ApmcUriInstanceReferenceValue                                          = 611,
    /// APMC-URI-PARAM-DEF
    ApmcUriParamDef                                                        = 912,
    /// APMC-URI-PARAM-VALUE
    ApmcUriParamValue                                                      = 4967,
    /// APMC-VALUE-COLLECTION
    ApmcValueCollection                                                    = 5028,
    /// APP-ID
    AppId                                                                  = 351,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 3120,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS
    AppOsTaskProxyToEcuTaskProxyMappings                                   = 4407,
    /// APP-TASK-PROXY-REF
    AppTaskProxyRef                                                        = 4105,
    /// APPL-CONSTANT-REF
    ApplConstantRef                                                        = 663,
    /// APPL-INIT-VALUE
    ApplInitValue                                                          = 6095,
    /// APPL-WATCHDOG-LIMIT
    ApplWatchdogLimit                                                      = 4411,
    /// APPLICABILITY-INFO
    ApplicabilityInfo                                                      = 213,
    /// APPLICABILITY-INFO-SET
    ApplicabilityInfoSet                                                   = 871,
    /// APPLICABILITY-INFOS
    ApplicabilityInfos                                                     = 4635,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 3633,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 4278,
    /// APPLICATION-ARRAY-ELEMENT-REF
    ApplicationArrayElementRef                                             = 94,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 1970,
    /// APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION
    ApplicationAssocMapElementValueSpecification                           = 5072,
    /// APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION
    ApplicationAssocMapValueSpecification                                  = 428,
    /// APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF
    ApplicationCompositeDataTypeSubElementRef                              = 2977,
    /// APPLICATION-COMPOSITE-ELEMENT-IREF
    ApplicationCompositeElementIref                                        = 1156,
    /// APPLICATION-CONTEXT-REF
    ApplicationContextRef                                                  = 5161,
    /// APPLICATION-CONTEXT-REFS
    ApplicationContextRefs                                                 = 1368,
    /// APPLICATION-DATA-TYPE-REF
    ApplicationDataTypeRef                                                 = 2353,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 3632,
    /// APPLICATION-DESCRIPTION
    ApplicationDescription                                                 = 1278,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 5039,
    /// APPLICATION-ENDPOINT-REF
    ApplicationEndpointRef                                                 = 2049,
    /// APPLICATION-ENDPOINT-REF-CONDITIONAL
    ApplicationEndpointRefConditional                                      = 5071,
    /// APPLICATION-ENTRY
    ApplicationEntry                                                       = 5768,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 3969,
    /// APPLICATION-ID
    ApplicationId                                                          = 5345,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 2974,
    /// APPLICATION-MODE-IREF
    ApplicationModeIref                                                    = 4996,
    /// APPLICATION-MODE-MACHINE
    ApplicationModeMachine                                                 = 958,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 2351,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 650,
    /// APPLICATION-PARTITION-REF
    ApplicationPartitionRef                                                = 3966,
    /// APPLICATION-PARTITION-REFS
    ApplicationPartitionRefs                                               = 991,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 6126,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS
    ApplicationPartitionToEcuPartitionMappings                             = 1146,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 1884,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 2228,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 4952,
    /// APPLICATION-RECORD-ELEMENT-REF
    ApplicationRecordElementRef                                            = 305,
    /// APPLICATION-RULE-BASED-VALUE-SPECIFICATION
    ApplicationRuleBasedValueSpecification                                 = 6625,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 3416,
    /// APPLICATION-TYPE-REF
    ApplicationTypeRef                                                     = 1924,
    /// APPLICATION-TYPE-TREF
    ApplicationTypeTref                                                    = 5250,
    /// APPLICATION-VALUE-SPECIFICATION
    ApplicationValueSpecification                                          = 5558,
    /// APPLICATIONS
    Applications                                                           = 2620,
    /// APPLIED-STANDARD
    AppliedStandard                                                        = 2256,
    /// APPLIED-STANDARDS
    AppliedStandards                                                       = 3051,
    /// APPLIES-TO
    AppliesTo                                                              = 3887,
    /// APPLIES-TO-DEPENDENCIES
    AppliesToDependencies                                                  = 1457,
    /// APPROVAL-REQUIRED
    ApprovalRequired                                                       = 6624,
    /// APPROVAL-STATUS
    ApprovalStatus                                                         = 6627,
    /// AR-PACKAGE
    ArPackage                                                              = 5611,
    /// AR-PACKAGES
    ArPackages                                                             = 887,
    /// AR-PARAMETER
    ArParameter                                                            = 3025,
    /// AR-RELEASE-VERSION
    ArReleaseVersion                                                       = 1469,
    /// AR-TYPED-PER-INSTANCE-MEMORY-REF
    ArTypedPerInstanceMemoryRef                                            = 3754,
    /// AR-TYPED-PER-INSTANCE-MEMORYS
    ArTypedPerInstanceMemorys                                              = 4316,
    /// ARBITRARY-ADDRESS-CAPABLE
    ArbitraryAddressCapable                                                = 5288,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 5620,
    /// ARBITRATION
    Arbitration                                                            = 70,
    /// ARBITRATIONS
    Arbitrations                                                           = 6482,
    /// AREA
    Area                                                                   = 3795,
    /// ARGUMENT
    Argument                                                               = 295,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 2212,
    /// ARGUMENT-MAPPINGS
    ArgumentMappings                                                       = 2548,
    /// ARGUMENT-REF
    ArgumentRef                                                            = 5442,
    /// ARGUMENTS
    Arguments                                                              = 3788,
    /// ARGUMENTSS
    Argumentss                                                             = 5015,
    /// ARP-PROPS
    ArpProps                                                               = 4962,
    /// ARRAY-ELEMENT-MAPPINGS
    ArrayElementMappings                                                   = 377,
    /// ARRAY-ELEMENT-REF
    ArrayElementRef                                                        = 2878,
    /// ARRAY-IMPL-POLICY
    ArrayImplPolicy                                                        = 1626,
    /// ARRAY-SIZE
    ArraySize                                                              = 5504,
    /// ARRAY-SIZE-HANDLING
    ArraySizeHandling                                                      = 4445,
    /// ARRAY-SIZE-SEMANTICS
    ArraySizeSemantics                                                     = 2663,
    /// ARRAY-VALUE-SPECIFICATION
    ArrayValueSpecification                                                = 2247,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 2335,
    /// ARTIFACT-CHECKSUM-REF
    ArtifactChecksumRef                                                    = 6167,
    /// ARTIFACT-CHECKSUM-REFS
    ArtifactChecksumRefs                                                   = 3610,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 163,
    /// ARTIFACT-CHECKSUMS
    ArtifactChecksums                                                      = 1659,
    /// ARTIFACT-DESCRIPTOR
    ArtifactDescriptor                                                     = 615,
    /// ARTIFACT-DESCRIPTORS
    ArtifactDescriptors                                                    = 6026,
    /// ARTIFACT-LOCATOR
    ArtifactLocator                                                        = 359,
    /// ARTIFACT-LOCATORS
    ArtifactLocators                                                       = 3349,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 5966,
    /// ASSERTIONS
    Assertions                                                             = 3895,
    /// ASSIGN-FRAME-ID
    AssignFrameId                                                          = 276,
    /// ASSIGN-FRAME-ID-RANGE
    AssignFrameIdRange                                                     = 1987,
    /// ASSIGN-NAD
    AssignNad                                                              = 6208,
    /// ASSIGNED-CONTROLLER-REF
    AssignedControllerRef                                                  = 2925,
    /// ASSIGNED-DATA-TYPES
    AssignedDataTypes                                                      = 5347,
    /// ASSIGNED-DATAS
    AssignedDatas                                                          = 1585,
    /// ASSIGNED-ENTRY-REF
    AssignedEntryRef                                                       = 2100,
    /// ASSIGNED-ENTRY-ROLES
    AssignedEntryRoles                                                     = 1971,
    /// ASSIGNED-FRAME-TRIGGERING-REF
    AssignedFrameTriggeringRef                                             = 1775,
    /// ASSIGNED-LIN-SLAVE-CONFIG-REF
    AssignedLinSlaveConfigRef                                              = 1258,
    /// ASSIGNED-PORTS
    AssignedPorts                                                          = 1420,
    /// ASSIGNED-TRAFFIC-CLASS
    AssignedTrafficClass                                                   = 3729,
    /// ASSIGNED-TRAFFIC-CLASSS
    AssignedTrafficClasss                                                  = 1938,
    /// ASSIGNMENT-PRIORITY
    AssignmentPriority                                                     = 5812,
    /// ASSOCIATED-COM-I-PDU-GROUP-REF
    AssociatedComIPduGroupRef                                              = 5260,
    /// ASSOCIATED-COM-I-PDU-GROUP-REFS
    AssociatedComIPduGroupRefs                                             = 5163,
    /// ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    AssociatedConsumedProvidedServiceInstanceGroups                        = 5296,
    /// ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF
    AssociatedCrossSwClusterComRtePluginRef                                = 1462,
    /// ASSOCIATED-EVENT-IDENTIFICATION
    AssociatedEventIdentification                                          = 191,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REF
    AssociatedPdurIPduGroupRef                                             = 1832,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REFS
    AssociatedPdurIPduGroupRefs                                            = 1673,
    /// ASSOCIATED-RTE-PLUGIN-REF
    AssociatedRtePluginRef                                                 = 6205,
    /// ASSUMED-CURRENT-STATE-IREF
    AssumedCurrentStateIref                                                = 878,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 128,
    /// ASYNCHRONOUS-SERVER-CALL-POINT-REF
    AsynchronousServerCallPointRef                                         = 4684,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 3519,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS
    AsynchronousServerCallResultPoints                                     = 850,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 6080,
    /// ASYNCHRONOUS-TRAFFIC-SHAPER-REF
    AsynchronousTrafficShaperRef                                           = 3929,
    /// ATS-INSTANCE-ENTRY-REF
    AtsInstanceEntryRef                                                    = 1505,
    /// ATS-INSTANCE-ENTRYS
    AtsInstanceEntrys                                                      = 4657,
    /// ATTRIBUTE-DEFS
    AttributeDefs                                                          = 2706,
    /// ATTRIBUTE-NAME
    AttributeName                                                          = 4749,
    /// ATTRIBUTE-REF
    AttributeRef                                                           = 355,
    /// ATTRIBUTE-TAILORINGS
    AttributeTailorings                                                    = 2332,
    /// ATTRIBUTE-VALUES
    AttributeValues                                                        = 2991,
    /// ATTRIBUTES
    Attributes                                                             = 6433,
    /// AUDIENCE
    Audience                                                               = 3937,
    /// AUDIENCES
    Audiences                                                              = 2449,
    /// AUTH-ALGORITHM
    AuthAlgorithm                                                          = 1760,
    /// AUTH-DATA-FRESHNESS-LENGTH
    AuthDataFreshnessLength                                                = 3571,
    /// AUTH-DATA-FRESHNESS-START-POSITION
    AuthDataFreshnessStartPosition                                         = 5740,
    /// AUTH-INFO-TX-LENGTH
    AuthInfoTxLength                                                       = 3567,
    /// AUTH-ROLE-REF
    AuthRoleRef                                                            = 2119,
    /// AUTH-ROLE-REFS
    AuthRoleRefs                                                           = 4133,
    /// AUTHENTIC-CONNECTION-PROPSS
    AuthenticConnectionPropss                                              = 5637,
    /// AUTHENTICATION-BUILD-ATTEMPTS
    AuthenticationBuildAttempts                                            = 5300,
    /// AUTHENTICATION-CLASS-REF
    AuthenticationClassRef                                                 = 3340,
    /// AUTHENTICATION-ENABLED
    AuthenticationEnabled                                                  = 6483,
    /// AUTHENTICATION-KEY-REF
    AuthenticationKeyRef                                                   = 3275,
    /// AUTHENTICATION-PROPS-REF
    AuthenticationPropsRef                                                 = 4257,
    /// AUTHENTICATION-PROPSS
    AuthenticationPropss                                                   = 703,
    /// AUTHENTICATION-REF
    AuthenticationRef                                                      = 1737,
    /// AUTHENTICATION-RETRIES
    AuthenticationRetries                                                  = 4966,
    /// AUTHENTICATION-ROLE-REF
    AuthenticationRoleRef                                                  = 2133,
    /// AUTHENTICATION-ROLE-REFS
    AuthenticationRoleRefs                                                 = 3879,
    /// AUTHENTICATION-TIMEOUT
    AuthenticationTimeout                                                  = 3455,
    /// AUTHENTICATION-VERIFY-ATTEMPTS
    AuthenticationVerifyAttempts                                           = 5683,
    /// AUTO-AVAILABLE
    AutoAvailable                                                          = 6421,
    /// AUTO-COLLECT
    AutoCollect                                                            = 5685,
    /// AUTO-IP-PROPS
    AutoIpProps                                                            = 2219,
    /// AUTO-REQUIRE
    AutoRequire                                                            = 3905,
    /// AUTO-START
    AutoStart                                                              = 6128,
    /// AUTOMATIC-END
    AutomaticEnd                                                           = 5642,
    /// AUTOSAR
    Autosar                                                                = 4866,
    /// AUTOSAR-CONNECTOR
    AutosarConnector                                                       = 2248,
    /// AUTOSAR-DATA-TYPE-REF
    AutosarDataTypeRef                                                     = 4024,
    /// AUTOSAR-DATA-TYPE-REF-CONDITIONAL
    AutosarDataTypeRefConditional                                          = 5185,
    /// AUTOSAR-ENGINEERING-OBJECT
    AutosarEngineeringObject                                               = 2466,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 1263,
    /// AUTOSAR-PARAMETER-IREF
    AutosarParameterIref                                                   = 2523,
    /// AUTOSAR-VARIABLE
    AutosarVariable                                                        = 2217,
    /// AUTOSAR-VARIABLE-IN-IMPL-DATATYPE
    AutosarVariableInImplDatatype                                          = 495,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 5706,
    /// AUTOSAR-VARIABLE-IREF
    AutosarVariableIref                                                    = 5761,
    /// AUXILIARY-FIELD-DEFINITIONS
    AuxiliaryFieldDefinitions                                              = 4868,
    /// AUXILIARY-FIELDS
    AuxiliaryFields                                                        = 939,
    /// AVERAGE-MEMORY-CONSUMPTION
    AverageMemoryConsumption                                               = 2695,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 5802,
    /// BAM-TIMING
    BamTiming                                                              = 6239,
    /// BANDWIDTH-LIMITATION
    BandwidthLimitation                                                    = 399,
    /// BASE-COMPOSITION-REF
    BaseCompositionRef                                                     = 4930,
    /// BASE-CYCLE
    BaseCycle                                                              = 4560,
    /// BASE-FREQUENCY
    BaseFrequency                                                          = 4628,
    /// BASE-IS-THIS-PACKAGE
    BaseIsThisPackage                                                      = 2374,
    /// BASE-REF
    BaseRef                                                                = 4448,
    /// BASE-REFERENCE-REF
    BaseReferenceRef                                                       = 5382,
    /// BASE-TYPE-ENCODING
    BaseTypeEncoding                                                       = 1721,
    /// BASE-TYPE-REF
    BaseTypeRef                                                            = 3133,
    /// BASE-TYPE-SIZE
    BaseTypeSize                                                           = 3045,
    /// BASIC-CYCLE-LENGTH
    BasicCycleLength                                                       = 2129,
    /// BAUDRATE
    Baudrate                                                               = 5891,
    /// BEHAVIOR
    Behavior                                                               = 2555,
    /// BEHAVIOR-REF
    BehaviorRef                                                            = 2253,
    /// BEST-CASE-EXECUTION-TIME
    BestCaseExecutionTime                                                  = 5602,
    /// BGCOLOR
    Bgcolor                                                                = 6219,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 4180,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 4833,
    /// BINARY-MANIFEST-ITEM-NUMERICAL-VALUE
    BinaryManifestItemNumericalValue                                       = 5946,
    /// BINARY-MANIFEST-ITEM-POINTER-VALUE
    BinaryManifestItemPointerValue                                         = 5319,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 3235,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 2090,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 4916,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 5241,
    /// BIT
    Bit                                                                    = 1969,
    /// BIT-NUMBER
    BitNumber                                                              = 4972,
    /// BIT-OFFSET
    BitOffset                                                              = 3944,
    /// BIT-POSITION
    BitPosition                                                            = 658,
    /// BITFIELD-TEXT-TABLE-MASK-FIRST
    BitfieldTextTableMaskFirst                                             = 6119,
    /// BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR
    BitfieldTextTableMaskNvBlockDescriptor                                 = 2737,
    /// BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE
    BitfieldTextTableMaskPortPrototype                                     = 1912,
    /// BITFIELD-TEXT-TABLE-MASK-SECOND
    BitfieldTextTableMaskSecond                                            = 3646,
    /// BLACKLISTED-VERSIONS
    BlacklistedVersions                                                    = 4847,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREF
    BlockIfStateActiveApIref                                               = 4330,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREFS
    BlockIfStateActiveApIrefs                                              = 1693,
    /// BLOCK-IF-STATE-ACTIVE-CP-REF
    BlockIfStateActiveCpRef                                                = 3570,
    /// BLOCK-IF-STATE-ACTIVE-CP-REFS
    BlockIfStateActiveCpRefs                                               = 21,
    /// BLOCK-INCONSISTENT
    BlockInconsistent                                                      = 28,
    /// BLOCK-STATE
    BlockState                                                             = 3175,
    /// BLOCK-STATES
    BlockStates                                                            = 6267,
    /// BLOCKLISTED-VERSIONS
    BlocklistedVersions                                                    = 1354,
    /// BLUEPRINT-CONDITION
    BlueprintCondition                                                     = 237,
    /// BLUEPRINT-DERIVATION-GUIDE
    BlueprintDerivationGuide                                               = 2394,
    /// BLUEPRINT-MAPPING
    BlueprintMapping                                                       = 4958,
    /// BLUEPRINT-MAPPING-GUIDE
    BlueprintMappingGuide                                                  = 2390,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 5643,
    /// BLUEPRINT-MAPS
    BlueprintMaps                                                          = 3489,
    /// BLUEPRINT-POLICY-LIST
    BlueprintPolicyList                                                    = 2816,
    /// BLUEPRINT-POLICY-NOT-MODIFIABLE
    BlueprintPolicyNotModifiable                                           = 6485,
    /// BLUEPRINT-POLICY-SINGLE
    BlueprintPolicySingle                                                  = 387,
    /// BLUEPRINT-POLICYS
    BlueprintPolicys                                                       = 2493,
    /// BLUEPRINT-REF
    BlueprintRef                                                           = 1861,
    /// BOOLEAN-VALUE-VARIATION-POINT
    BooleanValueVariationPoint                                             = 3734,
    /// BOR-COUNTER-L-1-TO-L-2
    BorCounterL1ToL2                                                       = 5929,
    /// BOR-TIME-L-1
    BorTimeL1                                                              = 1574,
    /// BOR-TIME-L-2
    BorTimeL2                                                              = 6402,
    /// BOR-TIME-TX-ENSURED
    BorTimeTxEnsured                                                       = 2871,
    /// BR
    Br                                                                     = 846,
    /// BROADCAST
    Broadcast                                                              = 5490,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 6292,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 342,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 3836,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 2835,
    /// BSW-BEHAVIOR-REF
    BswBehaviorRef                                                         = 654,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 5193,
    /// BSW-CLIENT-POLICY
    BswClientPolicy                                                        = 2975,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 3774,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 1810,
    /// BSW-DATA-SEND-POLICY
    BswDataSendPolicy                                                      = 409,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 3278,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 5815,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 3590,
    /// BSW-ENTITY-REF
    BswEntityRef                                                           = 5164,
    /// BSW-ENTRY-KIND
    BswEntryKind                                                           = 229,
    /// BSW-ENTRY-RELATIONSHIP
    BswEntryRelationship                                                   = 2802,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 286,
    /// BSW-ENTRY-RELATIONSHIP-TYPE
    BswEntryRelationshipType                                               = 3341,
    /// BSW-ENTRY-RELATIONSHIPS
    BswEntryRelationships                                                  = 3452,
    /// BSW-EXCLUSIVE-AREA-POLICY
    BswExclusiveAreaPolicy                                                 = 1418,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 4426,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 3015,
    /// BSW-IMPLEMENTATION-REF
    BswImplementationRef                                                   = 795,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 4040,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 4346,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 5623,
    /// BSW-INTERNAL-TRIGGERING-POINT-POLICY
    BswInternalTriggeringPointPolicy                                       = 1625,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF
    BswInternalTriggeringPointRef                                          = 3757,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL
    BswInternalTriggeringPointRefConditional                               = 1493,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 5570,
    /// BSW-INTERRUPT-EVENT
    BswInterruptEvent                                                      = 2819,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 1881,
    /// BSW-MODE-GROUP-REF
    BswModeGroupRef                                                        = 3767,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 2903,
    /// BSW-MODE-RECEIVER-POLICY
    BswModeReceiverPolicy                                                  = 1369,
    /// BSW-MODE-SENDER-POLICY
    BswModeSenderPolicy                                                    = 5262,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 5143,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 2605,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 3512,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 6183,
    /// BSW-MODULE-DEPENDENCYS
    BswModuleDependencys                                                   = 115,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 218,
    /// BSW-MODULE-DESCRIPTION-REF
    BswModuleDescriptionRef                                                = 1546,
    /// BSW-MODULE-DESCRIPTION-REF-CONDITIONAL
    BswModuleDescriptionRefConditional                                     = 38,
    /// BSW-MODULE-DOCUMENTATIONS
    BswModuleDocumentations                                                = 963,
    /// BSW-MODULE-ENTITY-REF
    BswModuleEntityRef                                                     = 2762,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 4341,
    /// BSW-MODULE-ENTRY-REF
    BswModuleEntryRef                                                      = 4372,
    /// BSW-MODULE-ENTRY-REF-CONDITIONAL
    BswModuleEntryRefConditional                                           = 3355,
    /// BSW-MODULE-INSTANCE-REF
    BswModuleInstanceRef                                                   = 2829,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 6372,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 4708,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 3811,
    /// BSW-PARAMETER-POLICY
    BswParameterPolicy                                                     = 4624,
    /// BSW-PER-INSTANCE-MEMORY-POLICY
    BswPerInstanceMemoryPolicy                                             = 4072,
    /// BSW-PER-INSTANCE-MEMORY-POLICYS
    BswPerInstanceMemoryPolicys                                            = 1051,
    /// BSW-QUEUED-DATA-RECEPTION-POLICY
    BswQueuedDataReceptionPolicy                                           = 2035,
    /// BSW-RELEASED-TRIGGER-POLICY
    BswReleasedTriggerPolicy                                               = 3945,
    /// BSW-RESOLUTION
    BswResolution                                                          = 6341,
    /// BSW-RESOURCE-ESTIMATION
    BswResourceEstimation                                                  = 5255,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 2163,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 5387,
    /// BSW-SERVICE-DEPENDENCY
    BswServiceDependency                                                   = 4228,
    /// BSW-SERVICE-DEPENDENCY-REF
    BswServiceDependencyRef                                                = 5173,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 3681,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 1544,
    /// BSW-TRIGGER-DIRECT-IMPLEMENTATION
    BswTriggerDirectImplementation                                         = 5115,
    /// BSW-TRIGGER-REF
    BswTriggerRef                                                          = 1265,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 604,
    /// BUCKET-SIZE
    BucketSize                                                             = 1229,
    /// BUFFER-COMPUTATION
    BufferComputation                                                      = 535,
    /// BUFFER-OUTPUT
    BufferOutput                                                           = 2180,
    /// BUFFER-PROPERTIES
    BufferProperties                                                       = 3759,
    /// BUFFER-RATIO
    BufferRatio                                                            = 3508,
    /// BUILD-ACTION
    BuildAction                                                            = 5950,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 5425,
    /// BUILD-ACTION-ENVIRONMENTS
    BuildActionEnvironments                                                = 1642,
    /// BUILD-ACTION-IO-ELEMENT
    BuildActionIoElement                                                   = 2632,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 3928,
    /// BUILD-ACTION-MANIFEST-REF
    BuildActionManifestRef                                                 = 4185,
    /// BUILD-ACTION-MANIFEST-REF-CONDITIONAL
    BuildActionManifestRefConditional                                      = 4977,
    /// BUILD-ACTION-MANIFESTS
    BuildActionManifests                                                   = 6279,
    /// BUILD-ACTIONS
    BuildActions                                                           = 353,
    /// BUILD-TYPE
    BuildType                                                              = 438,
    /// BULK-DATA-CATEGORY
    BulkDataCategory                                                       = 4484,
    /// BULK-NV-BLOCK
    BulkNvBlock                                                            = 3544,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 5576,
    /// BULK-NV-DATA-DESCRIPTORS
    BulkNvDataDescriptors                                                  = 3892,
    /// BUNDLED-CONNECTIONS
    BundledConnections                                                     = 350,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 4084,
    /// BURST-SIZE
    BurstSize                                                              = 4208,
    /// BURST-SIZE-REQUEST
    BurstSizeRequest                                                       = 106,
    /// BURST-SIZE-RESPONSE
    BurstSizeResponse                                                      = 5076,
    /// BUS-DEPENDENT-NM-ECUS
    BusDependentNmEcus                                                     = 1751,
    /// BUS-ID
    BusId                                                                  = 5676,
    /// BUS-IDLE-TIMEOUT-PERIOD
    BusIdleTimeoutPeriod                                                   = 5545,
    /// BUS-MIRROR-CAN-ID-RANGE-MAPPING
    BusMirrorCanIdRangeMapping                                             = 6632,
    /// BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING
    BusMirrorCanIdToCanIdMapping                                           = 5783,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 5233,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 3217,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 1873,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 5124,
    /// BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING
    BusMirrorLinPidToCanIdMapping                                          = 285,
    /// BUS-MIRROR-NETWORK-ID
    BusMirrorNetworkId                                                     = 1515,
    /// BUS-OFF-RECOVERY
    BusOffRecovery                                                         = 2723,
    /// BUS-SPECIFIC-NM-ECU
    BusSpecificNmEcu                                                       = 4550,
    /// BY-PASS-POINT-IREF
    ByPassPointIref                                                        = 2634,
    /// BY-PASS-POINT-IREFS
    ByPassPointIrefs                                                       = 5203,
    /// BYPASS-ETHER-TYPE
    BypassEtherType                                                        = 4947,
    /// BYPASS-ETHER-TYPES
    BypassEtherTypes                                                       = 5973,
    /// BYPASS-VLAN
    BypassVlan                                                             = 756,
    /// BYPASS-VLANS
    BypassVlans                                                            = 5390,
    /// BYTE
    Byte                                                                   = 6249,
    /// BYTE-ORDER
    ByteOrder                                                              = 6151,
    /// BYTE-VALUE
    ByteValue                                                              = 2260,
    /// BYTE-VALUES
    ByteValues                                                             = 1776,
    /// C-CODE
    CCode                                                                  = 208,
    /// CAK-REF
    CakRef                                                                 = 4152,
    /// CALC-RAM-BLOCK-CRC
    CalcRamBlockCrc                                                        = 5636,
    /// CALCULATION-FORMULA
    CalculationFormula                                                     = 4070,
    /// CALIBRATION-PARAMETER-VALUE
    CalibrationParameterValue                                              = 4075,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 2656,
    /// CALIBRATION-PARAMETER-VALUE-SET-REF
    CalibrationParameterValueSetRef                                        = 3244,
    /// CALIBRATION-PARAMETER-VALUE-SET-REFS
    CalibrationParameterValueSetRefs                                       = 694,
    /// CALIBRATION-PARAMETER-VALUES
    CalibrationParameterValues                                             = 413,
    /// CALL-POINTS
    CallPoints                                                             = 55,
    /// CALL-SIGNAL-REF
    CallSignalRef                                                          = 5907,
    /// CALL-SIGNAL-TRIGGERING-REF
    CallSignalTriggeringRef                                                = 6029,
    /// CALL-TIMEOUT
    CallTimeout                                                            = 4006,
    /// CALL-TYPE
    CallType                                                               = 802,
    /// CALLBACK-HEADER-REF
    CallbackHeaderRef                                                      = 6531,
    /// CALLBACK-HEADER-REFS
    CallbackHeaderRefs                                                     = 1956,
    /// CALLED-ENTRY-REF
    CalledEntryRef                                                         = 174,
    /// CALLED-ENTRYS
    CalledEntrys                                                           = 3782,
    /// CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF
    CalledFromWithinExclusiveAreaRef                                       = 1363,
    /// CAN-ADDRESSING-MODE
    CanAddressingMode                                                      = 4762,
    /// CAN-BAUDRATE
    CanBaudrate                                                            = 277,
    /// CAN-BE-INVOKED-CONCURRENTLY
    CanBeInvokedConcurrently                                               = 1635,
    /// CAN-BIT-RATE-SWITCH
    CanBitRateSwitch                                                       = 1002,
    /// CAN-CLUSTER
    CanCluster                                                             = 4446,
    /// CAN-CLUSTER-CONDITIONAL
    CanClusterConditional                                                  = 962,
    /// CAN-CLUSTER-VARIANTS
    CanClusterVariants                                                     = 4365,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 1933,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 5770,
    /// CAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    CanCommunicationControllerConditional                                  = 5773,
    /// CAN-COMMUNICATION-CONTROLLER-VARIANTS
    CanCommunicationControllerVariants                                     = 268,
    /// CAN-CONFIG
    CanConfig                                                              = 3794,
    /// CAN-CONTROLLER-ATTRIBUTES
    CanControllerAttributes                                                = 680,
    /// CAN-CONTROLLER-CONFIGURATION
    CanControllerConfiguration                                             = 601,
    /// CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS
    CanControllerConfigurationRequirements                                 = 6359,
    /// CAN-CONTROLLER-FD-ATTRIBUTES
    CanControllerFdAttributes                                              = 5709,
    /// CAN-CONTROLLER-FD-REQUIREMENTS
    CanControllerFdRequirements                                            = 5066,
    /// CAN-CONTROLLER-XL-ATTRIBUTES
    CanControllerXlAttributes                                              = 5937,
    /// CAN-CONTROLLER-XL-REQUIREMENTS
    CanControllerXlRequirements                                            = 2623,
    /// CAN-ENTER-EXCLUSIVE-AREA-REF
    CanEnterExclusiveAreaRef                                               = 3166,
    /// CAN-ENTER-EXCLUSIVE-AREA-REFS
    CanEnterExclusiveAreaRefs                                              = 1444,
    /// CAN-ENTERS
    CanEnters                                                              = 3527,
    /// CAN-FD-BAUDRATE
    CanFdBaudrate                                                          = 4904,
    /// CAN-FD-CONFIG
    CanFdConfig                                                            = 4209,
    /// CAN-FD-FRAME-SUPPORT
    CanFdFrameSupport                                                      = 692,
    /// CAN-FRAME
    CanFrame                                                               = 1480,
    /// CAN-FRAME-RX-BEHAVIOR
    CanFrameRxBehavior                                                     = 6165,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 5614,
    /// CAN-FRAME-TX-BEHAVIOR
    CanFrameTxBehavior                                                     = 4223,
    /// CAN-GLOBAL-TIME-DOMAIN-PROPS
    CanGlobalTimeDomainProps                                               = 862,
    /// CAN-ID-RANGE-MAPPINGS
    CanIdRangeMappings                                                     = 4522,
    /// CAN-ID-TO-CAN-ID-MAPPINGS
    CanIdToCanIdMappings                                                   = 5509,
    /// CAN-IDENTIFIER
    CanIdentifier                                                          = 212,
    /// CAN-IDENTIFIER-MASK
    CanIdentifierMask                                                      = 5268,
    /// CAN-IDENTIFIER-RANGE
    CanIdentifierRange                                                     = 3804,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 4517,
    /// CAN-NM-CLUSTER-COUPLING
    CanNmClusterCoupling                                                   = 6560,
    /// CAN-NM-ECU
    CanNmEcu                                                               = 426,
    /// CAN-NM-NODE
    CanNmNode                                                              = 4088,
    /// CAN-NM-TX-CAN-ID
    CanNmTxCanId                                                           = 5505,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 6395,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 4237,
    /// CAN-TP-ADDRESS-REF
    CanTpAddressRef                                                        = 2204,
    /// CAN-TP-ADDRESS-REFS
    CanTpAddressRefs                                                       = 4565,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 5059,
    /// CAN-TP-CHANNEL-REF
    CanTpChannelRef                                                        = 4064,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 587,
    /// CAN-TP-CONNECTION
    CanTpConnection                                                        = 5171,
    /// CAN-TP-ECU
    CanTpEcu                                                               = 1891,
    /// CAN-TP-NODE
    CanTpNode                                                              = 1940,
    /// CAN-XL-BAUDRATE
    CanXlBaudrate                                                          = 5552,
    /// CAN-XL-CONFIG
    CanXlConfig                                                            = 5375,
    /// CAN-XL-CONFIG-REF
    CanXlConfigRef                                                         = 1117,
    /// CAN-XL-CONFIG-REQS
    CanXlConfigReqs                                                        = 5432,
    /// CAN-XL-FRAME-TRIGGERING-PROPS
    CanXlFrameTriggeringProps                                              = 5876,
    /// CAN-XL-NM-PROPS
    CanXlNmProps                                                           = 2062,
    /// CAN-XL-PROPS
    CanXlProps                                                             = 2611,
    /// CAN-XL-PROPS-REF
    CanXlPropsRef                                                          = 6377,
    /// CAN-XL-PROPS-REFS
    CanXlPropsRefs                                                         = 4908,
    /// CANCELLATION
    Cancellation                                                           = 1046,
    /// CAPABILITY
    Capability                                                             = 5473,
    /// CAPABILITY-RECORDS
    CapabilityRecords                                                      = 6332,
    /// CAPACITY
    Capacity                                                               = 3730,
    /// CAPTION
    Caption                                                                = 4347,
    /// CAS-RX-LOW-MAX
    CasRxLowMax                                                            = 5324,
    /// CAT-2-ISR
    Cat2Isr                                                                = 3408,
    /// CATEGORY
    Category                                                               = 1031,
    /// CDD-TYPE
    CddType                                                                = 82,
    /// CERT-COMMON-NAME
    CertCommonName                                                         = 5720,
    /// CERTIFICATE-EVALUATIONS
    CertificateEvaluations                                                 = 6500,
    /// CERTIFICATE-GROUP-REF
    CertificateGroupRef                                                    = 886,
    /// CERTIFICATE-REF
    CertificateRef                                                         = 2130,
    /// CERTIFICATE-TO-KEY-SLOT-MAPPINGS
    CertificateToKeySlotMappings                                           = 1984,
    /// CHANGE
    Change                                                                 = 3864,
    /// CHANNEL-ID
    ChannelId                                                              = 6054,
    /// CHANNEL-MODE
    ChannelMode                                                            = 5082,
    /// CHANNEL-NAME
    ChannelName                                                            = 4195,
    /// CHANNEL-REF
    ChannelRef                                                             = 2046,
    /// CHANNEL-SYNCHRONOUS-WAKEUP
    ChannelSynchronousWakeup                                               = 2274,
    /// CHANNELS
    Channels                                                               = 4112,
    /// CHANNELS-PER-FRAME
    ChannelsPerFrame                                                       = 2584,
    /// CHAPTER
    Chapter                                                                = 4963,
    /// CHAPTERS
    Chapters                                                               = 5002,
    /// CHARGE-MANAGER-NEEDS
    ChargeManagerNeeds                                                     = 4218,
    /// CHECK-PER-SOURCE-ID
    CheckPerSourceId                                                       = 5750,
    /// CHECK-STATIC-BLOCK-ID
    CheckStaticBlockId                                                     = 6250,
    /// CHECKPOINT-FINAL-REF
    CheckpointFinalRef                                                     = 5857,
    /// CHECKPOINT-FINAL-REFS
    CheckpointFinalRefs                                                    = 3756,
    /// CHECKPOINT-ID
    CheckpointId                                                           = 5516,
    /// CHECKPOINT-INITIAL-REF
    CheckpointInitialRef                                                   = 1176,
    /// CHECKPOINT-INITIAL-REFS
    CheckpointInitialRefs                                                  = 1485,
    /// CHECKPOINT-IREF
    CheckpointIref                                                         = 1400,
    /// CHECKPOINT-REF
    CheckpointRef                                                          = 1631,
    /// CHECKPOINT-REFS
    CheckpointRefs                                                         = 3958,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 4969,
    /// CHECKPOINT-TRANSITION-REF
    CheckpointTransitionRef                                                = 3561,
    /// CHECKPOINTS
    Checkpoints                                                            = 364,
    /// CHECKPOINTSS
    Checkpointss                                                           = 3454,
    /// CHECKSUM-VALUE
    ChecksumValue                                                          = 5325,
    /// CHECKSUM-VERIFICATION
    ChecksumVerification                                                   = 1556,
    /// CHOICES
    Choices                                                                = 827,
    /// CIPHER-SUITE
    CipherSuite                                                            = 1846,
    /// CIPHER-SUITE-CONFIGS
    CipherSuiteConfigs                                                     = 5741,
    /// CIPHER-SUITE-ID
    CipherSuiteId                                                          = 673,
    /// CIPHER-SUITE-NAME
    CipherSuiteName                                                        = 1683,
    /// CIPHER-SUITE-PRIORITY
    CipherSuitePriority                                                    = 6263,
    /// CIPHER-SUITE-SHORT-LABEL
    CipherSuiteShortLabel                                                  = 5074,
    /// CKN-REF
    CknRef                                                                 = 4815,
    /// CLAIMED-FUNCTION-GROUP-REF
    ClaimedFunctionGroupRef                                                = 3684,
    /// CLAIMED-FUNCTION-GROUP-REFS
    ClaimedFunctionGroupRefs                                               = 3186,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 2244,
    /// CLASS-CONTENTS
    ClassContents                                                          = 5472,
    /// CLASS-TAILORINGS
    ClassTailorings                                                        = 3717,
    /// CLASSIFICATION
    Classification                                                         = 450,
    /// CLASSIFICATIONS
    Classifications                                                        = 1437,
    /// CLEAR-CONDITION-GROUP-REF
    ClearConditionGroupRef                                                 = 5631,
    /// CLEAR-CONDITION-REF
    ClearConditionRef                                                      = 5881,
    /// CLEAR-CONDITION-REFS
    ClearConditionRefs                                                     = 411,
    /// CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF
    ClearDiagnosticInformationClassRef                                     = 3919,
    /// CLEAR-DTC-LIMITATION
    ClearDtcLimitation                                                     = 2098,
    /// CLEAR-EVENT-ALLOWED-BEHAVIOR
    ClearEventAllowedBehavior                                              = 3876,
    /// CLEAR-EVENT-BEHAVIOR
    ClearEventBehavior                                                     = 906,
    /// CLEAR-FROM-VALID-TO-INVALID
    ClearFromValidToInvalid                                                = 4870,
    /// CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF
    ClearResetEmissionRelatedDiagnosticInfoClassRef                        = 4456,
    /// CLIENT-CAPABILITY
    ClientCapability                                                       = 1563,
    /// CLIENT-COM-SPEC
    ClientComSpec                                                          = 3280,
    /// CLIENT-ID
    ClientId                                                               = 1116,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 4514,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 4901,
    /// CLIENT-ID-DEFINITION-SET-REF
    ClientIdDefinitionSetRef                                               = 4117,
    /// CLIENT-ID-DEFINITION-SET-REFS
    ClientIdDefinitionSetRefs                                              = 3714,
    /// CLIENT-ID-DEFINITIONS
    ClientIdDefinitions                                                    = 4922,
    /// CLIENT-ID-RANGE
    ClientIdRange                                                          = 2393,
    /// CLIENT-INTENT
    ClientIntent                                                           = 3559,
    /// CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST
    ClientIpAddrFromConnectionRequest                                      = 394,
    /// CLIENT-POLICYS
    ClientPolicys                                                          = 1441,
    /// CLIENT-PORT-FROM-CONNECTION-REQUEST
    ClientPortFromConnectionRequest                                        = 6169,
    /// CLIENT-PORT-REF
    ClientPortRef                                                          = 4444,
    /// CLIENT-SERVER-ANNOTATION
    ClientServerAnnotation                                                 = 3841,
    /// CLIENT-SERVER-ANNOTATIONS
    ClientServerAnnotations                                                = 5850,
    /// CLIENT-SERVER-APPLICATION-ERROR-MAPPING
    ClientServerApplicationErrorMapping                                    = 5147,
    /// CLIENT-SERVER-ARRAY-ELEMENT-MAPPING
    ClientServerArrayElementMapping                                        = 944,
    /// CLIENT-SERVER-ARRAY-TYPE-MAPPING
    ClientServerArrayTypeMapping                                           = 3859,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 6486,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 194,
    /// CLIENT-SERVER-INTERFACE-REF
    ClientServerInterfaceRef                                               = 2554,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 5443,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 2420,
    /// CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING
    ClientServerOperationBlueprintMapping                                  = 2911,
    /// CLIENT-SERVER-OPERATION-COM-PROPS
    ClientServerOperationComProps                                          = 1036,
    /// CLIENT-SERVER-OPERATION-IREF
    ClientServerOperationIref                                              = 6076,
    /// CLIENT-SERVER-OPERATION-MAPPING
    ClientServerOperationMapping                                           = 5906,
    /// CLIENT-SERVER-OPERATION-REF
    ClientServerOperationRef                                               = 3939,
    /// CLIENT-SERVER-PORTS
    ClientServerPorts                                                      = 4339,
    /// CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING
    ClientServerPrimitiveTypeMapping                                       = 3784,
    /// CLIENT-SERVER-RECORD-ELEMENT-MAPPING
    ClientServerRecordElementMapping                                       = 1025,
    /// CLIENT-SERVER-RECORD-TYPE-MAPPING
    ClientServerRecordTypeMapping                                          = 3323,
    /// CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING
    ClientServerToSignalGroupMapping                                       = 6297,
    /// CLIENT-SERVER-TO-SIGNAL-MAPPING
    ClientServerToSignalMapping                                            = 195,
    /// CLIENT-SERVICE-MAJOR-VERSION
    ClientServiceMajorVersion                                              = 2245,
    /// CLIENT-SERVICE-MINOR-VERSION
    ClientServiceMinorVersion                                              = 4689,
    /// CLOCK-REFERENCE-REF
    ClockReferenceRef                                                      = 1500,
    /// CLUSTER-DRIFT-DAMPING
    ClusterDriftDamping                                                    = 3344,
    /// CLUSTERED-COMPONENT-IREF
    ClusteredComponentIref                                                 = 358,
    /// CLUSTERED-COMPONENT-IREFS
    ClusteredComponentIrefs                                                = 748,
    /// CM-MODULE-INSTANTIATION
    CmModuleInstantiation                                                  = 2822,
    /// CODE
    Code                                                                   = 4421,
    /// CODE-DESCRIPTORS
    CodeDescriptors                                                        = 968,
    /// CODE-LABEL
    CodeLabel                                                              = 5126,
    /// COLD-START-ATTEMPTS
    ColdStartAttempts                                                      = 4569,
    /// COLLECTED-INSTANCE-IREF
    CollectedInstanceIref                                                  = 2549,
    /// COLLECTED-INSTANCE-IREFS
    CollectedInstanceIrefs                                                 = 5424,
    /// COLLECTION
    Collection                                                             = 2206,
    /// COLLECTION-PROPS
    CollectionProps                                                        = 5169,
    /// COLLECTION-REF
    CollectionRef                                                          = 2378,
    /// COLLECTION-SEMANTICS
    CollectionSemantics                                                    = 4805,
    /// COLLECTION-THRESHOLD
    CollectionThreshold                                                    = 5960,
    /// COLLECTION-TIMEOUT
    CollectionTimeout                                                      = 4882,
    /// COLLECTION-TRIGGER
    CollectionTrigger                                                      = 3579,
    /// COLLISION-RESOLVING-SCHEDULE-REF
    CollisionResolvingScheduleRef                                          = 5160,
    /// COLOR-MODE
    ColorMode                                                              = 294,
    /// COLSPEC
    Colspec                                                                = 5279,
    /// COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS
    ComBasedSignalGroupTransformations                                     = 6193,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 706,
    /// COM-CONFIGURATION-GW-TIME-BASE
    ComConfigurationGwTimeBase                                             = 1824,
    /// COM-CONFIGURATION-ID
    ComConfigurationId                                                     = 6106,
    /// COM-CONFIGURATION-RX-TIME-BASE
    ComConfigurationRxTimeBase                                             = 2573,
    /// COM-CONFIGURATION-TX-TIME-BASE
    ComConfigurationTxTimeBase                                             = 3665,
    /// COM-CONTROL-CLASS-REF
    ComControlClassRef                                                     = 1108,
    /// COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION
    ComEnableMdtForCyclicTransmission                                      = 457,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 4667,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 3488,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 4729,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 3150,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 1291,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 6391,
    /// COM-HANDLER-TASK-MAPPING-ENABLED
    ComHandlerTaskMappingEnabled                                           = 2852,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 5990,
    /// COM-MANAGEMENT-GROUP-REF
    ComManagementGroupRef                                                  = 975,
    /// COM-MANAGEMENT-GROUP-REFS
    ComManagementGroupRefs                                                 = 6345,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 4383,
    /// COM-MANAGEMENT-MAPPINGS
    ComManagementMappings                                                  = 2678,
    /// COM-MANAGEMENT-PORT-GROUP-IREF
    ComManagementPortGroupIref                                             = 480,
    /// COM-MANAGEMENT-PORT-GROUP-IREFS
    ComManagementPortGroupIrefs                                            = 3630,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 600,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 2141,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 3543,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 4457,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 4106,
    /// COM-PROCESSING-PERIOD
    ComProcessingPeriod                                                    = 5341,
    /// COM-PROPS
    ComProps                                                               = 469,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 1095,
    /// COM-TIMEOUT-ERROR-REF
    ComTimeoutErrorRef                                                     = 3239,
    /// COM-TRIGGER-GRANT
    ComTriggerGrant                                                        = 1826,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 1838,
    /// COMBINED-NO-DATA-INIT-COUNT
    CombinedNoDataInitCount                                                = 217,
    /// COMM-CONNECTORS
    CommConnectors                                                         = 4109,
    /// COMM-CONTROLLER-MAPPINGS
    CommControllerMappings                                                 = 493,
    /// COMM-CONTROLLER-REF
    CommControllerRef                                                      = 3515,
    /// COMM-CONTROLLERS
    CommControllers                                                        = 1069,
    /// COMMAND
    Command                                                                = 2440,
    /// COMMANDS
    Commands                                                               = 2418,
    /// COMMENT
    Comment                                                                = 3753,
    /// COMMITTED-BURST-SIZE
    CommittedBurstSize                                                     = 793,
    /// COMMITTED-INFORMATION-RATE
    CommittedInformationRate                                               = 5419,
    /// COMMON-PROPERTIES
    CommonProperties                                                       = 2933,
    /// COMMON-SIGNAL-PATH
    CommonSignalPath                                                       = 14,
    /// COMMUNICATION-BUFFER-LOCKING
    CommunicationBufferLocking                                             = 557,
    /// COMMUNICATION-CLUSTER-REF
    CommunicationClusterRef                                                = 612,
    /// COMMUNICATION-CLUSTER-REFS
    CommunicationClusterRefs                                               = 5273,
    /// COMMUNICATION-CONNECTOR-REF
    CommunicationConnectorRef                                              = 842,
    /// COMMUNICATION-CONNECTOR-REF-CONDITIONAL
    CommunicationConnectorRefConditional                                   = 1189,
    /// COMMUNICATION-CONNECTOR-REFS
    CommunicationConnectorRefs                                             = 3357,
    /// COMMUNICATION-CONNECTORS
    CommunicationConnectors                                                = 2290,
    /// COMMUNICATION-CONTROLLER-MAPPING
    CommunicationControllerMapping                                         = 995,
    /// COMMUNICATION-CONTROLLER-REF
    CommunicationControllerRef                                             = 4557,
    /// COMMUNICATION-CONTROLLERS
    CommunicationControllers                                               = 2566,
    /// COMMUNICATION-CYCLE
    CommunicationCycle                                                     = 4502,
    /// COMMUNICATION-DIRECTION
    CommunicationDirection                                                 = 1953,
    /// COMMUNICATION-MODE
    CommunicationMode                                                      = 4602,
    /// COMMUNICATION-RESOURCE-PROPS
    CommunicationResourceProps                                             = 1955,
    /// COMMUNICATION-RESOURCE-REF
    CommunicationResourceRef                                               = 4906,
    /// COMPARE-TYPE
    CompareType                                                            = 855,
    /// COMPARE-VALUE
    CompareValue                                                           = 3548,
    /// COMPILER
    Compiler                                                               = 4533,
    /// COMPILERS
    Compilers                                                              = 2314,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 5037,
    /// COMPLEX-TYPE-MAPPING
    ComplexTypeMapping                                                     = 5564,
    /// COMPONENT-CLUSTERING
    ComponentClustering                                                    = 1994,
    /// COMPONENT-IMPLEMENTATION-REF
    ComponentImplementationRef                                             = 1618,
    /// COMPONENT-IREF
    ComponentIref                                                          = 5330,
    /// COMPONENT-IREFS
    ComponentIrefs                                                         = 1814,
    /// COMPONENT-QUALIFIER
    ComponentQualifier                                                     = 2108,
    /// COMPONENT-REF
    ComponentRef                                                           = 2704,
    /// COMPONENT-SEPARATION
    ComponentSeparation                                                    = 3121,
    /// COMPONENTS
    Components                                                             = 3950,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 1091,
    /// COMPOSITE-NETWORK-REPRESENTATION
    CompositeNetworkRepresentation                                         = 5376,
    /// COMPOSITE-NETWORK-REPRESENTATIONS
    CompositeNetworkRepresentations                                        = 2060,
    /// COMPOSITE-RULE-BASED-VALUE-SPECIFICATION
    CompositeRuleBasedValueSpecification                                   = 6587,
    /// COMPOSITE-SERVICE-INTERFACE-REF
    CompositeServiceInterfaceRef                                           = 5860,
    /// COMPOSITE-TYPE-MAPPINGS
    CompositeTypeMappings                                                  = 1175,
    /// COMPOSITION-DATA-PROTOTYPE-REF
    CompositionDataPrototypeRef                                            = 3778,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 3381,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 5948,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 2546,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF
    CompositionSwComponentTypeRef                                          = 4048,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL
    CompositionSwComponentTypeRefConditional                               = 5731,
    /// COMPOUND-PRIMITIVE-ARGUMENTS
    CompoundPrimitiveArguments                                             = 979,
    /// COMPRESSED-SOFTWARE-PACKAGE-SIZE
    CompressedSoftwarePackageSize                                          = 5270,
    /// COMPU-CONST
    CompuConst                                                             = 1100,
    /// COMPU-DEFAULT-VALUE
    CompuDefaultValue                                                      = 4381,
    /// COMPU-DENOMINATOR
    CompuDenominator                                                       = 537,
    /// COMPU-INTERNAL-TO-PHYS
    CompuInternalToPhys                                                    = 653,
    /// COMPU-INVERSE-VALUE
    CompuInverseValue                                                      = 111,
    /// COMPU-METHOD
    CompuMethod                                                            = 5821,
    /// COMPU-METHOD-REF
    CompuMethodRef                                                         = 4811,
    /// COMPU-NUMERATOR
    CompuNumerator                                                         = 4167,
    /// COMPU-PHYS-TO-INTERNAL
    CompuPhysToInternal                                                    = 458,
    /// COMPU-RATIONAL-COEFFS
    CompuRationalCoeffs                                                    = 1125,
    /// COMPU-SCALE
    CompuScale                                                             = 4171,
    /// COMPU-SCALES
    CompuScales                                                            = 2940,
    /// COMPUTED
    Computed                                                               = 5176,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 3847,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 3777,
    /// COND
    Cond                                                                   = 4275,
    /// CONDITION
    Condition                                                              = 5413,
    /// CONDITION-ACCESS
    ConditionAccess                                                        = 2639,
    /// CONDITION-FORMULA
    ConditionFormula                                                       = 1941,
    /// CONDITIONAL-CHANGE-NAD
    ConditionalChangeNad                                                   = 3900,
    /// CONDITIONS
    Conditions                                                             = 341,
    /// CONFIDENCE-INTERVAL
    ConfidenceInterval                                                     = 361,
    /// CONFIDENCE-INTERVALS
    ConfidenceIntervals                                                    = 6204,
    /// CONFIDENTIALITY-OFFSET
    ConfidentialityOffset                                                  = 582,
    /// CONFIG-CLASS
    ConfigClass                                                            = 307,
    /// CONFIG-ELEMENT-DEF-GLOBAL-REF
    ConfigElementDefGlobalRef                                              = 945,
    /// CONFIG-ELEMENT-DEF-LOCAL-REF
    ConfigElementDefLocalRef                                               = 6557,
    /// CONFIG-VARIANT
    ConfigVariant                                                          = 5481,
    /// CONFIGURATION-CLASS-AFFECTION
    ConfigurationClassAffection                                            = 5467,
    /// CONFIGURATION-HANDLING
    ConfigurationHandling                                                  = 3688,
    /// CONFIGURED-NAD
    ConfiguredNad                                                          = 1788,
    /// CONFIRMATION-THRESHOLD
    ConfirmationThreshold                                                  = 4297,
    /// CONFLICTS
    Conflicts                                                              = 465,
    /// CONFLICTS-TO
    ConflictsTo                                                            = 1602,
    /// CONNECT
    Connect                                                                = 274,
    /// CONNECTED-INDICATORS
    ConnectedIndicators                                                    = 1357,
    /// CONNECTION-BUNDLES
    ConnectionBundles                                                      = 829,
    /// CONNECTION-IS-MANDATORY
    ConnectionIsMandatory                                                  = 6337,
    /// CONNECTION-NEGOTIATION-BEHAVIOR
    ConnectionNegotiationBehavior                                          = 4582,
    /// CONNECTION-PRIO-PDUS
    ConnectionPrioPdus                                                     = 1007,
    /// CONNECTION-TYPE
    ConnectionType                                                         = 4027,
    /// CONNECTIONS
    Connections                                                            = 3428,
    /// CONNECTIVITY-KEY-NAME
    ConnectivityKeyName                                                    = 18,
    /// CONNECTOR-REF
    ConnectorRef                                                           = 5042,
    /// CONNECTOR-REFS
    ConnectorRefs                                                          = 3405,
    /// CONNECTORS
    Connectors                                                             = 4910,
    /// CONSIDER-BUILD-NUMBER
    ConsiderBuildNumber                                                    = 4397,
    /// CONSIDER-PTO-STATUS
    ConsiderPtoStatus                                                      = 3526,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 4107,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 3286,
    /// CONSISTENCY-NEEDSS
    ConsistencyNeedss                                                      = 5070,
    /// CONSTANT-MAPPING-REF
    ConstantMappingRef                                                     = 1297,
    /// CONSTANT-MAPPING-REFS
    ConstantMappingRefs                                                    = 5818,
    /// CONSTANT-MEMORYS
    ConstantMemorys                                                        = 6446,
    /// CONSTANT-REF
    ConstantRef                                                            = 6101,
    /// CONSTANT-REFERENCE
    ConstantReference                                                      = 1524,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 3283,
    /// CONSTANT-SPECIFICATION-MAPPING
    ConstantSpecificationMapping                                           = 770,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 4142,
    /// CONSTANT-VALUE-MAPPING-REF
    ConstantValueMappingRef                                                = 3995,
    /// CONSTANT-VALUE-MAPPING-REFS
    ConstantValueMappingRefs                                               = 255,
    /// CONSTR-LEVEL
    ConstrLevel                                                            = 148,
    /// CONSTRAINT-REF
    ConstraintRef                                                          = 4008,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 4020,
    /// CONSTRAINT-TAILORINGS
    ConstraintTailorings                                                   = 1082,
    /// CONSUMED-DDS-OPERATIONS
    ConsumedDdsOperations                                                  = 5580,
    /// CONSUMED-DDS-SERVICE-EVENTS
    ConsumedDdsServiceEvents                                               = 5488,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 2366,
    /// CONSUMED-EVENT-GROUP-REF
    ConsumedEventGroupRef                                                  = 915,
    /// CONSUMED-EVENT-GROUP-REFS
    ConsumedEventGroupRefs                                                 = 1638,
    /// CONSUMED-EVENT-GROUPS
    ConsumedEventGroups                                                    = 3856,
    /// CONSUMED-EVENTS
    ConsumedEvents                                                         = 643,
    /// CONSUMED-FIELDS
    ConsumedFields                                                         = 6245,
    /// CONSUMED-OPERATIONS
    ConsumedOperations                                                     = 2089,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 287,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF
    ConsumedProvidedServiceInstanceGroupRef                                = 3394,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL
    ConsumedProvidedServiceInstanceGroupRefConditional                     = 2437,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 5057,
    /// CONSUMED-SERVICE-INSTANCE-REF
    ConsumedServiceInstanceRef                                             = 2215,
    /// CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL
    ConsumedServiceInstanceRefConditional                                  = 4905,
    /// CONSUMED-SERVICE-INSTANCES
    ConsumedServiceInstances                                               = 1545,
    /// CONTAINED-AR-ELEMENT-REF
    ContainedArElementRef                                                  = 6221,
    /// CONTAINED-AR-ELEMENT-REFS
    ContainedArElementRefs                                                 = 1481,
    /// CONTAINED-FIBEX-ELEMENT-REF
    ContainedFibexElementRef                                               = 2572,
    /// CONTAINED-FIBEX-ELEMENT-REFS
    ContainedFibexElementRefs                                              = 6399,
    /// CONTAINED-I-PDU-PROPS
    ContainedIPduProps                                                     = 3700,
    /// CONTAINED-I-PDU-TRIGGERING-PROPSS
    ContainedIPduTriggeringPropss                                          = 1858,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REF
    ContainedISignalIPduGroupRef                                           = 83,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS
    ContainedISignalIPduGroupRefs                                          = 1647,
    /// CONTAINED-PACKAGE-ELEMENT-REF
    ContainedPackageElementRef                                             = 6184,
    /// CONTAINED-PACKAGE-ELEMENT-REFS
    ContainedPackageElementRefs                                            = 345,
    /// CONTAINED-PDU-TRIGGERING-REF
    ContainedPduTriggeringRef                                              = 4479,
    /// CONTAINED-PDU-TRIGGERING-REFS
    ContainedPduTriggeringRefs                                             = 4549,
    /// CONTAINED-PROCESS-REF
    ContainedProcessRef                                                    = 2961,
    /// CONTAINED-PROCESS-REFS
    ContainedProcessRefs                                                   = 5056,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 2653,
    /// CONTAINER-I-PDU-HEADER-BYTE-ORDER
    ContainerIPduHeaderByteOrder                                           = 385,
    /// CONTAINER-TIMEOUT
    ContainerTimeout                                                       = 2815,
    /// CONTAINER-TRIGGER
    ContainerTrigger                                                       = 113,
    /// CONTAINERS
    Containers                                                             = 369,
    /// CONTENT-ELEMENT-REF
    ContentElementRef                                                      = 2354,
    /// CONTENT-OBJECT
    ContentObject                                                          = 2531,
    /// CONTENT-REF
    ContentRef                                                             = 1158,
    /// CONTENT-TYPE
    ContentType                                                            = 3434,
    /// CONTENT-URI
    ContentUri                                                             = 4931,
    /// CONTEXT-APPLICATION-RECORD-ELEMENT-REF
    ContextApplicationRecordElementRef                                     = 2413,
    /// CONTEXT-ATOMIC-COMPONENT-REF
    ContextAtomicComponentRef                                              = 5628,
    /// CONTEXT-BSW-IMPLEMENTATION-REF
    ContextBswImplementationRef                                            = 4087,
    /// CONTEXT-COMPONENT-PROTOTYPE-REF
    ContextComponentPrototypeRef                                           = 5380,
    /// CONTEXT-COMPONENT-REF
    ContextComponentRef                                                    = 5572,
    /// CONTEXT-COMPONENT-REFS
    ContextComponentRefs                                                   = 3034,
    /// CONTEXT-COMPOSITION-REF
    ContextCompositionRef                                                  = 3316,
    /// CONTEXT-DATA-ELEMENT-IDENTIFICATIONS
    ContextDataElementIdentifications                                      = 3979,
    /// CONTEXT-DATA-ELEMENTS
    ContextDataElements                                                    = 2512,
    /// CONTEXT-DATA-PROTOTYPE-IN-CS-REF
    ContextDataPrototypeInCsRef                                            = 1094,
    /// CONTEXT-DATA-PROTOTYPE-IN-SR-REF
    ContextDataPrototypeInSrRef                                            = 5445,
    /// CONTEXT-DATA-PROTOTYPE-REF
    ContextDataPrototypeRef                                                = 4337,
    /// CONTEXT-DATA-PROTOTYPE-REFS
    ContextDataPrototypeRefs                                               = 4755,
    /// CONTEXT-DATA-SOURCE
    ContextDataSource                                                      = 3248,
    /// CONTEXT-DATAS
    ContextDatas                                                           = 6035,
    /// CONTEXT-DESCRIPTION
    ContextDescription                                                     = 6523,
    /// CONTEXT-ELEMENT-REF
    ContextElementRef                                                      = 4419,
    /// CONTEXT-ELEMENT-REFS
    ContextElementRefs                                                     = 6088,
    /// CONTEXT-ID
    ContextId                                                              = 1122,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF
    ContextImplementationDataElementRef                                    = 2697,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS
    ContextImplementationDataElementRefs                                   = 4625,
    /// CONTEXT-LIMITATION-REF
    ContextLimitationRef                                                   = 723,
    /// CONTEXT-LIMITATION-REFS
    ContextLimitationRefs                                                  = 6637,
    /// CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ContextModeDeclarationGroupPrototypeRef                                = 2301,
    /// CONTEXT-MODE-DECLARATION-GROUP-REF
    ContextModeDeclarationGroupRef                                         = 6365,
    /// CONTEXT-OPERATION-REF
    ContextOperationRef                                                    = 3188,
    /// CONTEXT-P-PORT-PROTOTYPE-REF
    ContextPPortPrototypeRef                                               = 816,
    /// CONTEXT-P-PORT-REF
    ContextPPortRef                                                        = 1337,
    /// CONTEXT-PORT-PROTOTYPE-REF
    ContextPortPrototypeRef                                                = 1253,
    /// CONTEXT-PORT-REF
    ContextPortRef                                                         = 910,
    /// CONTEXT-R-PORT-PROTOTYPE-REF
    ContextRPortPrototypeRef                                               = 4791,
    /// CONTEXT-R-PORT-REF
    ContextRPortRef                                                        = 5790,
    /// CONTEXT-REF
    ContextRef                                                             = 138,
    /// CONTEXT-ROOT-COMPONENT-REF
    ContextRootComponentRef                                                = 381,
    /// CONTEXT-ROOT-COMPOSITION-REF
    ContextRootCompositionRef                                              = 2023,
    /// CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF
    ContextRootSwClusterDesignComponentPrototypeRef                        = 6023,
    /// CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF
    ContextRootSwComponentPrototypeRef                                     = 2676,
    /// CONTEXT-ROOT-SW-COMPOSITION-REF
    ContextRootSwCompositionRef                                            = 3035,
    /// CONTEXT-SW-COMPONENT-PROTOTYPE-REF
    ContextSwComponentPrototypeRef                                         = 2422,
    /// CONTEXT-SW-COMPONENT-REF
    ContextSwComponentRef                                                  = 1467,
    /// CONTEXT-SWC-PROTOTYPE-REF
    ContextSwcPrototypeRef                                                 = 4929,
    /// CONTEXT-SWC-PROTOTYPE-REFS
    ContextSwcPrototypeRefs                                                = 1825,
    /// CONTEXT-TYPE-LIST
    ContextTypeList                                                        = 2361,
    /// CONTEXT-TYPE-LISTS
    ContextTypeLists                                                       = 5901,
    /// CONTEXTS
    Contexts                                                               = 5036,
    /// CONTRACT-VERSION
    ContractVersion                                                        = 726,
    /// CONTROL-CONSUMED-EVENT-GROUP-REF
    ControlConsumedEventGroupRef                                           = 5191,
    /// CONTROL-CONSUMED-EVENT-GROUP-REFS
    ControlConsumedEventGroupRefs                                          = 5507,
    /// CONTROL-ENABLE-MASK-BITS
    ControlEnableMaskBits                                                  = 5459,
    /// CONTROL-OPTION-RECORD-PRESENT
    ControlOptionRecordPresent                                             = 4580,
    /// CONTROL-PNC-REF
    ControlPncRef                                                          = 1976,
    /// CONTROL-PNC-REFS
    ControlPncRefs                                                         = 5592,
    /// CONTROL-PROVIDED-EVENT-GROUP-REF
    ControlProvidedEventGroupRef                                           = 6125,
    /// CONTROL-PROVIDED-EVENT-GROUP-REFS
    ControlProvidedEventGroupRefs                                          = 5664,
    /// CONTROLLED-DATA-ELEMENT-REF
    ControlledDataElementRef                                               = 4305,
    /// CONTROLLED-DATA-ELEMENT-REFS
    ControlledDataElementRefs                                              = 3596,
    /// CONTROLLED-FID-REF
    ControlledFidRef                                                       = 270,
    /// CONTROLLED-HW-ELEMENT-REF
    ControlledHwElementRef                                                 = 4173,
    /// CONTROLLER-REF
    ControllerRef                                                          = 801,
    /// CORE-ID
    CoreId                                                                 = 919,
    /// CORES
    Cores                                                                  = 3503,
    /// COUNT-PROFILE
    CountProfile                                                           = 5462,
    /// COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE
    CounterBasedFdcThresholdStorageValue                                   = 2907,
    /// COUNTER-DECREMENT-STEP-SIZE
    CounterDecrementStepSize                                               = 857,
    /// COUNTER-FAILED-THRESHOLD
    CounterFailedThreshold                                                 = 1609,
    /// COUNTER-INCREMENT-STEP-SIZE
    CounterIncrementStepSize                                               = 4169,
    /// COUNTER-JUMP-DOWN
    CounterJumpDown                                                        = 5003,
    /// COUNTER-JUMP-DOWN-VALUE
    CounterJumpDownValue                                                   = 291,
    /// COUNTER-JUMP-UP
    CounterJumpUp                                                          = 5471,
    /// COUNTER-JUMP-UP-VALUE
    CounterJumpUpValue                                                     = 5717,
    /// COUNTER-OFFSET
    CounterOffset                                                          = 1869,
    /// COUNTER-PASSED-THRESHOLD
    CounterPassedThreshold                                                 = 3346,
    /// COUPLED-CLUSTER-REF
    CoupledClusterRef                                                      = 1141,
    /// COUPLED-CLUSTER-REFS
    CoupledClusterRefs                                                     = 4399,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 5153,
    /// COUPLING-ELEMENT-DETAILS
    CouplingElementDetails                                                 = 688,
    /// COUPLING-ELEMENT-SWITCH-DETAILS
    CouplingElementSwitchDetails                                           = 1610,
    /// COUPLING-FLAG
    CouplingFlag                                                           = 6271,
    /// COUPLING-PORT
    CouplingPort                                                           = 4652,
    /// COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER
    CouplingPortAsynchronousTrafficShaper                                  = 4482,
    /// COUPLING-PORT-CONNECTION
    CouplingPortConnection                                                 = 5201,
    /// COUPLING-PORT-CONNECTIONS
    CouplingPortConnections                                                = 6194,
    /// COUPLING-PORT-CREDIT-BASED-SHAPER
    CouplingPortCreditBasedShaper                                          = 1658,
    /// COUPLING-PORT-DETAILS
    CouplingPortDetails                                                    = 3201,
    /// COUPLING-PORT-ENHANCED-TRAFFIC-SHAPER
    CouplingPortEnhancedTrafficShaper                                      = 6607,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 2886,
    /// COUPLING-PORT-RATE-POLICY
    CouplingPortRatePolicy                                                 = 1692,
    /// COUPLING-PORT-REF
    CouplingPortRef                                                        = 5536,
    /// COUPLING-PORT-REF-CONDITIONAL
    CouplingPortRefConditional                                             = 5266,
    /// COUPLING-PORT-ROLE
    CouplingPortRole                                                       = 609,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 3547,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 927,
    /// COUPLING-PORT-SPEED
    CouplingPortSpeed                                                      = 36,
    /// COUPLING-PORT-STARTUP-ACTIVE-TIME
    CouplingPortStartupActiveTime                                          = 4030,
    /// COUPLING-PORT-STRUCTURAL-ELEMENTS
    CouplingPortStructuralElements                                         = 2916,
    /// COUPLING-PORT-SWITCHOFF-DELAY
    CouplingPortSwitchoffDelay                                             = 300,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 5551,
    /// COUPLING-PORTS
    CouplingPorts                                                          = 959,
    /// COUPLING-TYPE
    CouplingType                                                           = 6455,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 2694,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 3463,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 3738,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 5446,
    /// CP-SOFTWARE-CLUSTER-REF
    CpSoftwareClusterRef                                                   = 6565,
    /// CP-SOFTWARE-CLUSTER-REF-CONDITIONAL
    CpSoftwareClusterRefConditional                                        = 5560,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 1565,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-REF
    CpSoftwareClusterResourceRef                                           = 2603,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 3742,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 4200,
    /// CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterToApplicationPartitionMapping                         = 6060,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 2876,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 6389,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 4800,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 6228,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 2677,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 3523,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 6533,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 5687,
    /// CPP-TEMPLATE-ARGUMENT
    CppTemplateArgument                                                    = 2941,
    /// CPU-USAGE
    CpuUsage                                                               = 4654,
    /// CRC-CORRECTION-FIELD
    CrcCorrectionField                                                     = 4247,
    /// CRC-DOMAIN-NUMBER
    CrcDomainNumber                                                        = 4927,
    /// CRC-FLAGS
    CrcFlags                                                               = 5261,
    /// CRC-MESSAGE-LENGTH
    CrcMessageLength                                                       = 1962,
    /// CRC-OFFSET
    CrcOffset                                                              = 6311,
    /// CRC-PRECISE-ORIGIN-TIMESTAMP
    CrcPreciseOriginTimestamp                                              = 3922,
    /// CRC-SECURED
    CrcSecured                                                             = 1632,
    /// CRC-SEQUENCE-ID
    CrcSequenceId                                                          = 6109,
    /// CRC-SOURCE-PORT-IDENTITY
    CrcSourcePortIdentity                                                  = 5008,
    /// CRC-VALIDATED
    CrcValidated                                                           = 6617,
    /// CREATE-ECU-WAKEUP-SOURCE
    CreateEcuWakeupSource                                                  = 6538,
    /// CREATED-DATAS
    CreatedDatas                                                           = 1733,
    /// CRF-PULL
    CrfPull                                                                = 5197,
    /// CRF-TYPE
    CrfType                                                                = 5788,
    /// CRYPTO-ALG-ID
    CryptoAlgId                                                            = 2794,
    /// CRYPTO-ALGO-CONFIG
    CryptoAlgoConfig                                                       = 204,
    /// CRYPTO-ALGORITHM-DESCRIPTION
    CryptoAlgorithmDescription                                             = 3697,
    /// CRYPTO-ALGORITHM-ID
    CryptoAlgorithmId                                                      = 2689,
    /// CRYPTO-ALGORITHM-STRING
    CryptoAlgorithmString                                                  = 790,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 5360,
    /// CRYPTO-CERTIFICATE-GROUP
    CryptoCertificateGroup                                                 = 4385,
    /// CRYPTO-CERTIFICATE-GROUP-INTERFACE
    CryptoCertificateGroupInterface                                        = 869,
    /// CRYPTO-CERTIFICATE-GROUP-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateGroupToPortPrototypeMapping                           = 5617,
    /// CRYPTO-CERTIFICATE-GROUPS
    CryptoCertificateGroups                                                = 1305,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 3723,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 4356,
    /// CRYPTO-CERTIFICATE-REF
    CryptoCertificateRef                                                   = 1187,
    /// CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING
    CryptoCertificateToCryptoKeySlotMapping                                = 3267,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 2083,
    /// CRYPTO-CERTIFICATES
    CryptoCertificates                                                     = 5835,
    /// CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS
    CryptoDesignToCryptoDriverMappings                                     = 1351,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 5172,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING
    CryptoDriverToCryptoJobMapping                                         = 363,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS
    CryptoDriverToCryptoJobMappings                                        = 2885,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 4562,
    /// CRYPTO-JOB
    CryptoJob                                                              = 3306,
    /// CRYPTO-JOB-REF
    CryptoJobRef                                                           = 709,
    /// CRYPTO-JOBS
    CryptoJobs                                                             = 64,
    /// CRYPTO-KEY-DESCRIPTION
    CryptoKeyDescription                                                   = 4954,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 1697,
    /// CRYPTO-KEY-REF
    CryptoKeyRef                                                           = 4812,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 5550,
    /// CRYPTO-KEY-SLOT-CLIENT-INTERFACE
    CryptoKeySlotClientInterface                                           = 1294,
    /// CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE
    CryptoKeySlotContentAllowedUsage                                       = 3725,
    /// CRYPTO-KEY-SLOT-DESIGN
    CryptoKeySlotDesign                                                    = 3514,
    /// CRYPTO-KEY-SLOT-DESIGN-DESCRIPTION
    CryptoKeySlotDesignDescription                                         = 1140,
    /// CRYPTO-KEY-SLOT-DESIGN-REF
    CryptoKeySlotDesignRef                                                 = 4693,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 3507,
    /// CRYPTO-KEY-SLOT-REF
    CryptoKeySlotRef                                                       = 6041,
    /// CRYPTO-KEY-SLOT-REFS
    CryptoKeySlotRefs                                                      = 753,
    /// CRYPTO-KEY-SLOT-TO-CLIENT-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToClientPortPrototypeMapping                              = 4259,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 3643,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN
    CryptoKeySlotUsageDesign                                               = 4095,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN-MAPPING
    CryptoKeySlotUsageDesignMapping                                        = 1411,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN-REF
    CryptoKeySlotUsageDesignRef                                            = 2478,
    /// CRYPTO-KEY-SLOT-USER-DESIGN
    CryptoKeySlotUserDesign                                                = 1807,
    /// CRYPTO-KEY-SLOT-USER-DESIGN-MAPPING
    CryptoKeySlotUserDesignMapping                                         = 3082,
    /// CRYPTO-KEY-SLOT-USER-DESIGN-REF
    CryptoKeySlotUserDesignRef                                             = 2754,
    /// CRYPTO-KEY-SLOTS
    CryptoKeySlots                                                         = 4945,
    /// CRYPTO-KEY-TYPE
    CryptoKeyType                                                          = 1210,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 3521,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 4415,
    /// CRYPTO-NEED-REF
    CryptoNeedRef                                                          = 3691,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 6002,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 1,
    /// CRYPTO-OBJECT-TYPE
    CryptoObjectType                                                       = 1934,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 796,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 5367,
    /// CRYPTO-PROVIDER-DOCUMENTATION-REF
    CryptoProviderDocumentationRef                                         = 4089,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 4673,
    /// CRYPTO-PROVIDER-REF
    CryptoProviderRef                                                      = 3372,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 1908,
    /// CRYPTO-PROVIDERS
    CryptoProviders                                                        = 2637,
    /// CRYPTO-R-PORT-COM-SPEC
    CryptoRPortComSpec                                                     = 954,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 4101,
    /// CRYPTO-SERVICE-CERTIFICATE-REF
    CryptoServiceCertificateRef                                            = 3531,
    /// CRYPTO-SERVICE-CERTIFICATE-REFS
    CryptoServiceCertificateRefs                                           = 4491,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 2955,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 4877,
    /// CRYPTO-SERVICE-KEY-REF
    CryptoServiceKeyRef                                                    = 1742,
    /// CRYPTO-SERVICE-MAPPINGS
    CryptoServiceMappings                                                  = 273,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 6467,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 453,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 614,
    /// CRYPTO-SERVICE-QUEUE-REF
    CryptoServiceQueueRef                                                  = 5441,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 6073,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 6425,
    /// CS-ERROR-REACTION
    CsErrorReaction                                                        = 6153,
    /// CSE-CODE
    CseCode                                                                = 1240,
    /// CSE-CODE-FACTOR
    CseCodeFactor                                                          = 1177,
    /// CURRENT-DATA-TYPE-REF
    CurrentDataTypeRef                                                     = 6277,
    /// CURRENT-EXP
    CurrentExp                                                             = 5757,
    /// CURRENT-VALUE-REF
    CurrentValueRef                                                        = 875,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 390,
    /// CUSTOM-DOCUMENT-ELEMENT-REF
    CustomDocumentElementRef                                               = 5080,
    /// CUSTOM-DOCUMENTATION-REF
    CustomDocumentationRef                                                 = 6068,
    /// CUSTOM-SDG-DEF-REF
    CustomSdgDefRef                                                        = 1474,
    /// CUSTOM-SDG-DEF-REFS
    CustomSdgDefRefs                                                       = 5748,
    /// CUSTOM-SERVICE-CLASS-REF
    CustomServiceClassRef                                                  = 4893,
    /// CUSTOM-SERVICE-ID
    CustomServiceId                                                        = 5797,
    /// CUSTOM-SPECIFICATION-REF
    CustomSpecificationRef                                                 = 3827,
    /// CUSTOM-SPECIFICATION-REFS
    CustomSpecificationRefs                                                = 3542,
    /// CUSTOM-SUB-FUNCTION-NUMBER
    CustomSubFunctionNumber                                                = 51,
    /// CUSTOM-TRIGGER
    CustomTrigger                                                          = 1960,
    /// CYCLE
    Cycle                                                                  = 3048,
    /// CYCLE-AUTOSTART
    CycleAutostart                                                         = 262,
    /// CYCLE-COUNT-MAX
    CycleCountMax                                                          = 3511,
    /// CYCLE-COUNTER
    CycleCounter                                                           = 921,
    /// CYCLE-REPETITION
    CycleRepetition                                                        = 4974,
    /// CYCLE-STATUS-STORAGE
    CycleStatusStorage                                                     = 1601,
    /// CYCLE-TIME-MAIN-FUNCTION
    CycleTimeMainFunction                                                  = 1886,
    /// CYCLE-TIME-VALUE
    CycleTimeValue                                                         = 6336,
    /// CYCLIC-BACKUP-INTERVAL
    CyclicBackupInterval                                                   = 3731,
    /// CYCLIC-HANDLING-COM-DATA-TO-OS-TASK-PROXY-MAPPING
    CyclicHandlingComDataToOsTaskProxyMapping                              = 3623,
    /// CYCLIC-HANDLING-COM-DATA-TO-OS-TASK-PROXY-MAPPINGS
    CyclicHandlingComDataToOsTaskProxyMappings                             = 4166,
    /// CYCLIC-TIMING
    CyclicTiming                                                           = 4803,
    /// CYCLIC-WRITING-PERIOD
    CyclicWritingPeriod                                                    = 4333,
    /// DATA-COM-PROPS
    DataComProps                                                           = 4127,
    /// DATA-CONSISTENCY-POLICY
    DataConsistencyPolicy                                                  = 1032,
    /// DATA-CONSTR
    DataConstr                                                             = 4758,
    /// DATA-CONSTR-REF
    DataConstrRef                                                          = 3242,
    /// DATA-CONSTR-RULE
    DataConstrRule                                                         = 952,
    /// DATA-CONSTR-RULES
    DataConstrRules                                                        = 5855,
    /// DATA-DUMP-ENTRY
    DataDumpEntry                                                          = 4534,
    /// DATA-ELEMENT-IREF
    DataElementIref                                                        = 84,
    /// DATA-ELEMENT-IREFS
    DataElementIrefs                                                       = 2004,
    /// DATA-ELEMENT-REF
    DataElementRef                                                         = 5116,
    /// DATA-ELEMENT-REFS
    DataElementRefs                                                        = 3886,
    /// DATA-ELEMENTS
    DataElements                                                           = 5795,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 1115,
    /// DATA-FILTER
    DataFilter                                                             = 5342,
    /// DATA-FILTER-TYPE
    DataFilterType                                                         = 6572,
    /// DATA-FORMAT-TAILORING
    DataFormatTailoring                                                    = 6490,
    /// DATA-ID
    DataId                                                                 = 5819,
    /// DATA-ID-MODE
    DataIdMode                                                             = 6581,
    /// DATA-ID-NIBBLE-OFFSET
    DataIdNibbleOffset                                                     = 1853,
    /// DATA-IDENTIFIER-REF
    DataIdentifierRef                                                      = 1101,
    /// DATA-IDENTIFIER-REFS
    DataIdentifierRefs                                                     = 5152,
    /// DATA-IDS
    DataIds                                                                = 35,
    /// DATA-IREF
    DataIref                                                               = 4140,
    /// DATA-LENGTH
    DataLength                                                             = 3573,
    /// DATA-LENGTH-REQUEST
    DataLengthRequest                                                      = 2892,
    /// DATA-LENGTH-RESPONSE
    DataLengthResponse                                                     = 2950,
    /// DATA-LINK-LAYER-RULE
    DataLinkLayerRule                                                      = 4413,
    /// DATA-MAPPINGS
    DataMappings                                                           = 6647,
    /// DATA-OFFSET
    DataOffset                                                             = 4225,
    /// DATA-PDU-REF
    DataPduRef                                                             = 613,
    /// DATA-PROTECTION-KIND
    DataProtectionKind                                                     = 423,
    /// DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF
    DataProtototypeInPortInterfaceRef                                      = 1107,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 3020,
    /// DATA-PROTOTYPE-GROUP-IREF
    DataPrototypeGroupIref                                                 = 1275,
    /// DATA-PROTOTYPE-GROUP-IREFS
    DataPrototypeGroupIrefs                                                = 3084,
    /// DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF
    DataPrototypeInClientServerInterfaceIref                               = 3973,
    /// DATA-PROTOTYPE-IN-METHOD-ARGUMENT-INSTANCE-REF-IREF
    DataPrototypeInMethodArgumentInstanceRefIref                           = 1512,
    /// DATA-PROTOTYPE-IN-PORT-INTERFACE-REF
    DataPrototypeInPortInterfaceRef                                        = 5914,
    /// DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF
    DataPrototypeInSenderReceiverInterfaceIref                             = 3874,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF
    DataPrototypeInServiceInterfaceIref                                    = 3884,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF
    DataPrototypeInServiceInterfaceRef                                     = 2467,
    /// DATA-PROTOTYPE-IREF
    DataPrototypeIref                                                      = 2135,
    /// DATA-PROTOTYPE-MAPPING
    DataPrototypeMapping                                                   = 4995,
    /// DATA-PROTOTYPE-REF
    DataPrototypeRef                                                       = 47,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPS
    DataPrototypeTransformationProps                                       = 5355,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPSS
    DataPrototypeTransformationPropss                                      = 5251,
    /// DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF
    DataPrototypeWithApplicationDataTypeInSystemRef                        = 5061,
    /// DATA-PROTOTYPES
    DataPrototypes                                                         = 681,
    /// DATA-PROVIDER
    DataProvider                                                           = 5589,
    /// DATA-READ-ACCESSS
    DataReadAccesss                                                        = 1168,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 2095,
    /// DATA-RECEIVE-POINT-BY-ARGUMENTS
    DataReceivePointByArguments                                            = 3651,
    /// DATA-RECEIVE-POINT-BY-VALUES
    DataReceivePointByValues                                               = 3625,
    /// DATA-RECEIVE-POINTS
    DataReceivePoints                                                      = 3957,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 6517,
    /// DATA-REF
    DataRef                                                                = 3321,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 11,
    /// DATA-SEND-POINTS
    DataSendPoints                                                         = 1219,
    /// DATA-TRANSFER-CLASS-REF
    DataTransferClassRef                                                   = 735,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 1722,
    /// DATA-TRANSFORMATION-KIND
    DataTransformationKind                                                 = 4418,
    /// DATA-TRANSFORMATION-REF
    DataTransformationRef                                                  = 4685,
    /// DATA-TRANSFORMATION-REF-CONDITIONAL
    DataTransformationRefConditional                                       = 4296,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 867,
    /// DATA-TRANSFORMATIONS
    DataTransformations                                                    = 1463,
    /// DATA-TYPE-FOR-SERIALIZATION-REF
    DataTypeForSerializationRef                                            = 4269,
    /// DATA-TYPE-FOR-SERIALIZATION-REFS
    DataTypeForSerializationRefs                                           = 595,
    /// DATA-TYPE-MAP
    DataTypeMap                                                            = 4158,
    /// DATA-TYPE-MAPPING
    DataTypeMapping                                                        = 4081,
    /// DATA-TYPE-MAPPING-REF
    DataTypeMappingRef                                                     = 1074,
    /// DATA-TYPE-MAPPING-REFS
    DataTypeMappingRefs                                                    = 6024,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 6213,
    /// DATA-TYPE-MAPPING-SET-REF
    DataTypeMappingSetRef                                                  = 3907,
    /// DATA-TYPE-MAPPING-SET-REFS
    DataTypeMappingSetRefs                                                 = 6479,
    /// DATA-TYPE-MAPS
    DataTypeMaps                                                           = 5220,
    /// DATA-TYPE-POLICY
    DataTypePolicy                                                         = 3857,
    /// DATA-TYPE-REF
    DataTypeRef                                                            = 922,
    /// DATA-TYPE-REFS
    DataTypeRefs                                                           = 1812,
    /// DATA-UPDATE-PERIOD
    DataUpdatePeriod                                                       = 1339,
    /// DATA-VALUE-ERROR-REF
    DataValueErrorRef                                                      = 6103,
    /// DATA-WRITE-ACCESSS
    DataWriteAccesss                                                       = 5216,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 4543,
    /// DATE
    Date                                                                   = 2657,
    /// DCM-BUS-TYPE
    DcmBusType                                                             = 6375,
    /// DCM-I-PDU
    DcmIPdu                                                                = 1603,
    /// DDS-CP-CONFIG
    DdsCpConfig                                                            = 4230,
    /// DDS-CP-CONSUMED-SERVICE-INSTANCE
    DdsCpConsumedServiceInstance                                           = 2809,
    /// DDS-CP-DOMAIN
    DdsCpDomain                                                            = 1930,
    /// DDS-CP-I-SIGNAL-TO-DDS-TOPIC-MAPPING
    DdsCpISignalToDdsTopicMapping                                          = 5745,
    /// DDS-CP-PARTITION
    DdsCpPartition                                                         = 1741,
    /// DDS-CP-PROVIDED-SERVICE-INSTANCE
    DdsCpProvidedServiceInstance                                           = 5356,
    /// DDS-CP-QOS-PROFILE
    DdsCpQosProfile                                                        = 6461,
    /// DDS-CP-SERVICE-INSTANCE-EVENT
    DdsCpServiceInstanceEvent                                              = 110,
    /// DDS-CP-SERVICE-INSTANCE-OPERATION
    DdsCpServiceInstanceOperation                                          = 117,
    /// DDS-CP-TOPIC
    DdsCpTopic                                                             = 2823,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 4607,
    /// DDS-DOMAINS
    DdsDomains                                                             = 4760,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 4627,
    /// DDS-EVENT-QOS-PROFILE-REF
    DdsEventQosProfileRef                                                  = 758,
    /// DDS-EVENT-QOS-PROPS
    DdsEventQosProps                                                       = 3040,
    /// DDS-EVENT-REF
    DdsEventRef                                                            = 5736,
    /// DDS-EVENT-TOPIC-REF
    DdsEventTopicRef                                                       = 3351,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 1343,
    /// DDS-FIELD-QOS-PROPS
    DdsFieldQosProps                                                       = 2958,
    /// DDS-FIELD-REPLY-TOPIC-REF
    DdsFieldReplyTopicRef                                                  = 2642,
    /// DDS-FIELD-REQUEST-TOPIC-REF
    DdsFieldRequestTopicRef                                                = 3110,
    /// DDS-I-SIGNAL-TO-TOPIC-MAPPINGS
    DdsISignalToTopicMappings                                              = 1465,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 4919,
    /// DDS-METHOD-QOS-PROPS
    DdsMethodQosProps                                                      = 2138,
    /// DDS-METHOD-REPLY-TOPIC-REF
    DdsMethodReplyTopicRef                                                 = 2358,
    /// DDS-METHOD-REQUEST-TOPIC-REF
    DdsMethodRequestTopicRef                                               = 4865,
    /// DDS-OPERATION-REQUEST-TRIGGERING-REF
    DdsOperationRequestTriggeringRef                                       = 6645,
    /// DDS-OPERATION-RESPONSE-TRIGGERING-REF
    DdsOperationResponseTriggeringRef                                      = 6521,
    /// DDS-PARTITION-REF
    DdsPartitionRef                                                        = 1042,
    /// DDS-PARTITIONS
    DdsPartitions                                                          = 6346,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 4688,
    /// DDS-QOS-PROFILE-REF
    DdsQosProfileRef                                                       = 5867,
    /// DDS-QOS-PROFILES
    DdsQosProfiles                                                         = 2294,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 2052,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 5917,
    /// DDS-RPC-SERVICE-REF
    DdsRpcServiceRef                                                       = 3538,
    /// DDS-RPC-SERVICES
    DdsRpcServices                                                         = 3438,
    /// DDS-RULE
    DdsRule                                                                = 3089,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 1079,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 4451,
    /// DDS-SERVICE-INSTANCE-EVENT-CP
    DdsServiceInstanceEventCp                                              = 4547,
    /// DDS-SERVICE-INSTANCE-FIELD-CP
    DdsServiceInstanceFieldCp                                              = 5045,
    /// DDS-SERVICE-INSTANCE-OPERATION-CP
    DdsServiceInstanceOperationCp                                          = 895,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 1651,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 5601,
    /// DDS-SERVICE-QOS-PROFILE-REF
    DdsServiceQosProfileRef                                                = 1312,
    /// DDS-SERVICE-VERSION
    DdsServiceVersion                                                      = 3866,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 2029,
    /// DDS-TOPIC-REF
    DdsTopicRef                                                            = 5639,
    /// DDS-TOPICS
    DdsTopics                                                              = 3322,
    /// DDS-TRANSFORMATION-DESCRIPTION
    DdsTransformationDescription                                           = 2652,
    /// DDS-TRANSFORMATION-I-SIGNAL-PROPS
    DdsTransformationISignalProps                                          = 1088,
    /// DDS-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    DdsTransformationISignalPropsConditional                               = 6397,
    /// DDS-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    DdsTransformationISignalPropsVariants                                  = 2458,
    /// DEADLINE
    Deadline                                                               = 92,
    /// DEADLINE-MAX
    DeadlineMax                                                            = 3798,
    /// DEADLINE-MIN
    DeadlineMin                                                            = 934,
    /// DEADLINE-PERIOD
    DeadlinePeriod                                                         = 4057,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 4564,
    /// DEADLINE-SUPERVISIONS
    DeadlineSupervisions                                                   = 5290,
    /// DEBOUNCE-ALGORITHM
    DebounceAlgorithm                                                      = 4300,
    /// DEBOUNCE-ALGORITHM-PROPSS
    DebounceAlgorithmPropss                                                = 1178,
    /// DEBOUNCE-ALGORITHM-REF
    DebounceAlgorithmRef                                                   = 4462,
    /// DEBOUNCE-BEHAVIOR
    DebounceBehavior                                                       = 5977,
    /// DEBOUNCE-COUNTER-STORAGE
    DebounceCounterStorage                                                 = 3765,
    /// DEBOUNCE-TIME
    DebounceTime                                                           = 2533,
    /// DEBUG-INFOS
    DebugInfos                                                             = 3137,
    /// DECODING-CORRECTION
    DecodingCorrection                                                     = 1421,
    /// DECOMPOSITIONS
    Decompositions                                                         = 1166,
    /// DEF
    Def                                                                    = 3673,
    /// DEF-CALPRM-SET
    DefCalprmSet                                                           = 3437,
    /// DEF-ITEM
    DefItem                                                                = 3614,
    /// DEF-LIST
    DefList                                                                = 4890,
    /// DEFAULT-ACTION
    DefaultAction                                                          = 4640,
    /// DEFAULT-APPLICATION-TIMEOUT
    DefaultApplicationTimeout                                              = 1894,
    /// DEFAULT-ENDIANNESS
    DefaultEndianness                                                      = 516,
    /// DEFAULT-GATEWAY
    DefaultGateway                                                         = 3401,
    /// DEFAULT-LC-STATE-REF
    DefaultLcStateRef                                                      = 281,
    /// DEFAULT-LEASE-TIME
    DefaultLeaseTime                                                       = 3775,
    /// DEFAULT-LOG-THRESHOLD
    DefaultLogThreshold                                                    = 3931,
    /// DEFAULT-MODE-REF
    DefaultModeRef                                                         = 4719,
    /// DEFAULT-PATTERN
    DefaultPattern                                                         = 1672,
    /// DEFAULT-PERIOD-BEGIN
    DefaultPeriodBegin                                                     = 2847,
    /// DEFAULT-PERIOD-END
    DefaultPeriodEnd                                                       = 3814,
    /// DEFAULT-PRIORITY
    DefaultPriority                                                        = 3938,
    /// DEFAULT-REPORTING-MODE
    DefaultReportingMode                                                   = 383,
    /// DEFAULT-ROUTER
    DefaultRouter                                                          = 4268,
    /// DEFAULT-TRACE-STATE
    DefaultTraceState                                                      = 4431,
    /// DEFAULT-TRAFFIC-CLASS
    DefaultTrafficClass                                                    = 6638,
    /// DEFAULT-VALUE
    DefaultValue                                                           = 1936,
    /// DEFAULT-VALUE-ELEMENT
    DefaultValueElement                                                    = 5155,
    /// DEFAULT-VALUE-ELEMENTS
    DefaultValueElements                                                   = 1472,
    /// DEFAULT-VALUE-HANDLING
    DefaultValueHandling                                                   = 1383,
    /// DEFAULT-VLAN-REF
    DefaultVlanRef                                                         = 421,
    /// DEFERRING-FID-REF
    DeferringFidRef                                                        = 3652,
    /// DEFERRING-FID-REFS
    DeferringFidRefs                                                       = 1234,
    /// DEFINITION-REF
    DefinitionRef                                                          = 3896,
    /// DELAY
    Delay                                                                  = 3566,
    /// DELAY-COMPENSATION-A
    DelayCompensationA                                                     = 5336,
    /// DELAY-COMPENSATION-B
    DelayCompensationB                                                     = 2275,
    /// DELEGATED-PORT-ANNOTATION
    DelegatedPortAnnotation                                                = 2875,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 5607,
    /// DELETES
    Deletes                                                                = 3909,
    /// DELIVERY-ARTIFACTS
    DeliveryArtifacts                                                      = 2748,
    /// DELTA-PACKAGE-APPLICABLE-VERSION
    DeltaPackageApplicableVersion                                          = 6356,
    /// DENOMINATOR-CONDITION
    DenominatorCondition                                                   = 266,
    /// DENOMINATOR-GROUP
    DenominatorGroup                                                       = 4282,
    /// DEPENDENCIES
    Dependencies                                                           = 1883,
    /// DEPENDENCY
    Dependency                                                             = 5820,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 4601,
    /// DEPENDENT-RESOURCES
    DependentResources                                                     = 5956,
    /// DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF
    DependentSoftwareClusterDesignRef                                      = 6218,
    /// DEPENDENT-SOFTWARE-CLUSTER-REF
    DependentSoftwareClusterRef                                            = 3635,
    /// DEPENDS-ON
    DependsOn                                                              = 4734,
    /// DEPENDS-ON-REF
    DependsOnRef                                                           = 677,
    /// DEPENDS-ON-REFS
    DependsOnRefs                                                          = 5034,
    /// DEPENDS-ONS
    DependsOns                                                             = 6646,
    /// DEPLOYMENT-REF
    DeploymentRef                                                          = 3456,
    /// DEPLOYMENT-URIS
    DeploymentUris                                                         = 5800,
    /// DERIVATION
    Derivation                                                             = 2771,
    /// DERIVED-CERTIFICATE-ACCEPTED
    DerivedCertificateAccepted                                             = 2633,
    /// DERIVED-FROM-BLUEPRINT-REF
    DerivedFromBlueprintRef                                                = 2241,
    /// DERIVED-FROM-BLUEPRINT-REFS
    DerivedFromBlueprintRefs                                               = 1961,
    /// DERIVED-INTERFACE-REF
    DerivedInterfaceRef                                                    = 4956,
    /// DERIVED-OBJECT-REF
    DerivedObjectRef                                                       = 4713,
    /// DERIVED-PORT-INTERFACE-REF
    DerivedPortInterfaceRef                                                = 5535,
    /// DERIVED-PORT-PROTOTYPE-REF
    DerivedPortPrototypeRef                                                = 3493,
    /// DESC
    Desc                                                                   = 5372,
    /// DESCRIPTION
    Description                                                            = 322,
    /// DESIGN-REF
    DesignRef                                                              = 5830,
    /// DESIGN-REFS
    DesignRefs                                                             = 215,
    /// DEST-META-CLASS
    DestMetaClass                                                          = 3465,
    /// DEST-SDG-REF
    DestSdgRef                                                             = 4686,
    /// DESTINATION-BASE-ID
    DestinationBaseId                                                      = 3793,
    /// DESTINATION-CONTEXT
    DestinationContext                                                     = 4077,
    /// DESTINATION-IP-ADDRESS
    DestinationIpAddress                                                   = 5308,
    /// DESTINATION-IPV-4-ADDRESS
    DestinationIpv4Address                                                 = 3828,
    /// DESTINATION-IPV-6-ADDRESS
    DestinationIpv6Address                                                 = 2659,
    /// DESTINATION-MAC-ADDRESS
    DestinationMacAddress                                                  = 1504,
    /// DESTINATION-MAC-ADDRESS-MASK
    DestinationMacAddressMask                                              = 4864,
    /// DESTINATION-MAX-ADDRESS
    DestinationMaxAddress                                                  = 1479,
    /// DESTINATION-MIN-ADDRESS
    DestinationMinAddress                                                  = 4162,
    /// DESTINATION-NETWORK-MASK
    DestinationNetworkMask                                                 = 5394,
    /// DESTINATION-ORDER
    DestinationOrder                                                       = 2502,
    /// DESTINATION-ORDER-KIND
    DestinationOrderKind                                                   = 3423,
    /// DESTINATION-PHYSICAL-ADDRESS
    DestinationPhysicalAddress                                             = 3074,
    /// DESTINATION-PORTS
    DestinationPorts                                                       = 3168,
    /// DESTINATION-REF
    DestinationRef                                                         = 5313,
    /// DESTINATION-REFS
    DestinationRefs                                                        = 5388,
    /// DESTINATION-TYPE
    DestinationType                                                        = 1503,
    /// DESTINATION-URI-DEFS
    DestinationUriDefs                                                     = 1646,
    /// DESTINATION-URI-NESTING-CONTRACT
    DestinationUriNestingContract                                          = 5328,
    /// DESTINATION-URI-POLICY
    DestinationUriPolicy                                                   = 4896,
    /// DESTINATION-URI-REF
    DestinationUriRef                                                      = 335,
    /// DESTINATION-URI-REFS
    DestinationUriRefs                                                     = 4122,
    /// DETECT-NIT-ERROR
    DetectNitError                                                         = 6084,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 6086,
    /// DETERMINISTIC-CLIENT-REF
    DeterministicClientRef                                                 = 2728,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 5236,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDSS
    DeterministicClientResourceNeedss                                      = 4272,
    /// DETERMINISTIC-SYNC-M-OUT-OF-N
    DeterministicSyncMOutOfN                                               = 2863,
    /// DETERMINISTIC-SYNC-MASTER
    DeterministicSyncMaster                                                = 2195,
    /// DETERMINISTIC-SYNC-MASTER-REF
    DeterministicSyncMasterRef                                             = 2436,
    /// DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING
    DeterministicSyncMasterToTimeBaseConsumerMapping                       = 2255,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 288,
    /// DEVELOPMENT-VALUE
    DevelopmentValue                                                       = 4400,
    /// DHCP-ADDRESS-ASSIGNMENT
    DhcpAddressAssignment                                                  = 825,
    /// DHCP-PROPS
    DhcpProps                                                              = 3117,
    /// DHCP-SERVER-CONFIGURATION
    DhcpServerConfiguration                                                = 5077,
    /// DIAG-ARG-INTEGRITY
    DiagArgIntegrity                                                       = 116,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 4054,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 5958,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 1570,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 5253,
    /// DIAG-FAULT-DETECTION-COUNTER-PORT-REF
    DiagFaultDetectionCounterPortRef                                       = 2824,
    /// DIAG-PDU-TYPE
    DiagPduType                                                            = 348,
    /// DIAG-REQUIREMENT
    DiagRequirement                                                        = 5684,
    /// DIAG-ROUTINE-TYPE
    DiagRoutineType                                                        = 1616,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 5093,
    /// DIAGNOSTIC-ADDRESS
    DiagnosticAddress                                                      = 5555,
    /// DIAGNOSTIC-ADDRESSES
    DiagnosticAddresses                                                    = 1460,
    /// DIAGNOSTIC-ADDRESSS
    DiagnosticAddresss                                                     = 3602,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 1636,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 2386,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE
    DiagnosticAuthTransmitCertificate                                      = 5479,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION
    DiagnosticAuthTransmitCertificateEvaluation                            = 5292,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING
    DiagnosticAuthTransmitCertificateMapping                               = 1928,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 3447,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 2443,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 3111,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 956,
    /// DIAGNOSTIC-AUTHENTICATION-REF
    DiagnosticAuthenticationRef                                            = 5588,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 4495,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 873,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 2919,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 435,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 4154,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 2730,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 4801,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 896,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 2567,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 4647,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 943,
    /// DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL
    DiagnosticComControlSpecificChannel                                    = 5791,
    /// DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL
    DiagnosticComControlSubNodeChannel                                     = 2746,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF
    DiagnosticCommonElementRef                                             = 1131,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL
    DiagnosticCommonElementRefConditional                                  = 3271,
    /// DIAGNOSTIC-COMMON-PROPS-CONDITIONAL
    DiagnosticCommonPropsConditional                                       = 4116,
    /// DIAGNOSTIC-COMMON-PROPS-VARIANTS
    DiagnosticCommonPropsVariants                                          = 2641,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 5142,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 1086,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 2064,
    /// DIAGNOSTIC-CONDITION-REF
    DiagnosticConditionRef                                                 = 6123,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 6007,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 1757,
    /// DIAGNOSTIC-CONNECTION-REF
    DiagnosticConnectionRef                                                = 947,
    /// DIAGNOSTIC-CONNECTION-REF-CONDITIONAL
    DiagnosticConnectionRefConditional                                     = 5967,
    /// DIAGNOSTIC-CONNECTIONS
    DiagnosticConnections                                                  = 4862,
    /// DIAGNOSTIC-CONTRIBUTION-REF
    DiagnosticContributionRef                                              = 3525,
    /// DIAGNOSTIC-CONTRIBUTION-REFS
    DiagnosticContributionRefs                                             = 6216,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 4953,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 2477,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 1307,
    /// DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT
    DiagnosticControlEnableMaskBit                                         = 3440,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 5984,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 1276,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 5982,
    /// DIAGNOSTIC-DATA-CHANGE-TRIGGER
    DiagnosticDataChangeTrigger                                            = 3313,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 6004,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 6630,
    /// DIAGNOSTIC-DATA-ELEMENT-REF
    DiagnosticDataElementRef                                               = 3868,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 1314,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 3740,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 29,
    /// DIAGNOSTIC-DATA-IDENTIFIER-REF
    DiagnosticDataIdentifierRef                                            = 6042,
    /// DIAGNOSTIC-DATA-IDENTIFIER-REF-CONDITIONAL
    DiagnosticDataIdentifierRefConditional                                 = 556,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 3353,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF
    DiagnosticDataIdentifierSetRef                                         = 4742,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL
    DiagnosticDataIdentifierSetRefConditional                              = 2024,
    /// DIAGNOSTIC-DATA-IDENTIFIERS
    DiagnosticDataIdentifiers                                              = 479,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 4181,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 1754,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 3289,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 666,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 590,
    /// DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT
    DiagnosticDebounceBehaviorEnumValueVariationPoint                      = 2590,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 1385,
    /// DIAGNOSTIC-DEPLOYMENT-PROPS-REF
    DiagnosticDeploymentPropsRef                                           = 4301,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 4733,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-PORT-MAPPING
    DiagnosticDoIpActivationLinePortMapping                                = 3213,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE
    DiagnosticDoIpEntityIdentificationInterface                            = 2039,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-PORT-MAPPING
    DiagnosticDoIpEntityIdentificationPortMapping                          = 5776,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 443,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-PORT-MAPPING
    DiagnosticDoIpGroupIdentificationPortMapping                           = 248,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 1205,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-PORT-MAPPING
    DiagnosticDoIpPowerModePortMapping                                     = 5707,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 2729,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-PORT-MAPPING
    DiagnosticDoIpTriggerVehicleAnnouncementPortMapping                    = 3849,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 1523,
    /// DIAGNOSTIC-DTC-CHANGE-TRIGGER
    DiagnosticDtcChangeTrigger                                             = 782,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 5339,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 667,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 5878,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 3861,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 5326,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 2518,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 4045,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 1335,
    /// DIAGNOSTIC-EDR-DATA-PROVIDER-MAPPING
    DiagnosticEdrDataProviderMapping                                       = 804,
    /// DIAGNOSTIC-EDR-SENDER-PORT-MAPPING
    DiagnosticEdrSenderPortMapping                                         = 3123,
    /// DIAGNOSTIC-EDR-SERVER-PORT-MAPPING
    DiagnosticEdrServerPortMapping                                         = 989,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 5130,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 4165,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 160,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 3797,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF
    DiagnosticEnableConditionRef                                           = 554,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL
    DiagnosticEnableConditionRefConditional                                = 375,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 6417,
    /// DIAGNOSTIC-ENV-CONDITION-FORMULA
    DiagnosticEnvConditionFormula                                          = 4891,
    /// DIAGNOSTIC-ENV-DATA-CONDITION
    DiagnosticEnvDataCondition                                             = 3241,
    /// DIAGNOSTIC-ENV-DATA-ELEMENT-CONDITION
    DiagnosticEnvDataElementCondition                                      = 1992,
    /// DIAGNOSTIC-ENV-MODE-CONDITION
    DiagnosticEnvModeCondition                                             = 6494,
    /// DIAGNOSTIC-ENV-SOVD-DATA-CONDITION
    DiagnosticEnvSovdDataCondition                                         = 6015,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 2956,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 1024,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 2962,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 4042,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 2172,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 3721,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 640,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 6477,
    /// DIAGNOSTIC-EVENT-REF
    DiagnosticEventRef                                                     = 4714,
    /// DIAGNOSTIC-EVENT-REF-CONDITIONAL
    DiagnosticEventRefConditional                                          = 641,
    /// DIAGNOSTIC-EVENT-REFS
    DiagnosticEventRefs                                                    = 5769,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 3143,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 2621,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 3768,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 1212,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 5581,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 4292,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 1378,
    /// DIAGNOSTIC-EVENT-WINDOW
    DiagnosticEventWindow                                                  = 4478,
    /// DIAGNOSTIC-EVENTS
    DiagnosticEvents                                                       = 3506,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 6480,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-CLIENT-PORT-MAPPING
    DiagnosticExtendedDataRecordClientPortMapping                          = 594,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-ELEMENT
    DiagnosticExtendedDataRecordElement                                    = 4651,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-INTERFACE
    DiagnosticExtendedDataRecordInterface                                  = 4933,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-NEEDS
    DiagnosticExtendedDataRecordNeeds                                      = 4810,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF
    DiagnosticExtendedDataRecordRef                                        = 3749,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL
    DiagnosticExtendedDataRecordRefConditional                             = 4585,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION
    DiagnosticExternalAuthenticationIdentification                         = 140,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 104,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 5151,
    /// DIAGNOSTIC-EXTRACT-REF
    DiagnosticExtractRef                                                   = 4610,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 2895,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 4736,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 5523,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 6383,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 4345,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 136,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 1455,
    /// DIAGNOSTIC-FREEZE-FRAME-REF
    DiagnosticFreezeFrameRef                                               = 4761,
    /// DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL
    DiagnosticFreezeFrameRefConditional                                    = 3152,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 1118,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 5335,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 5100,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 2593,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 1972,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 1225,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 4819,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 6232,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 1582,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 4061,
    /// DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT
    DiagnosticIndicatorTypeEnumValueVariationPoint                         = 1963,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 3992,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 3396,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 1592,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 5561,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 3374,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 3334,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 378,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 6281,
    /// DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER
    DiagnosticIumprGroupIdentifier                                         = 6618,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 1848,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 180,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 6215,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 6313,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 1310,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 2043,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 1170,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 4899,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 1435,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 367,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 6264,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 6117,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 56,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 2284,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 261,
    /// DIAGNOSTIC-MEMORY-REF
    DiagnosticMemoryRef                                                    = 4992,
    /// DIAGNOSTIC-MESSAGE-TYPE
    DiagnosticMessageType                                                  = 42,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 2223,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 1452,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE
    DiagnosticMultipleConditionInterface                                   = 1982,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING
    DiagnosticMultipleConditionPortMapping                                 = 1464,
    /// DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE
    DiagnosticMultipleEventInterface                                       = 4186,
    /// DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING
    DiagnosticMultipleEventPortMapping                                     = 228,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE
    DiagnosticMultipleMonitorInterface                                     = 2459,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING
    DiagnosticMultipleMonitorPortMapping                                   = 1569,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 3552,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 1719,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 6018,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 5920,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF
    DiagnosticOperationCycleRef                                            = 3947,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL
    DiagnosticOperationCycleRefConditional                                 = 5333,
    /// DIAGNOSTIC-OPERATION-CYCLES
    DiagnosticOperationCycles                                              = 1780,
    /// DIAGNOSTIC-PARAMETER
    DiagnosticParameter                                                    = 540,
    /// DIAGNOSTIC-PARAMETER-ELEMENT
    DiagnosticParameterElement                                             = 2987,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 3182,
    /// DIAGNOSTIC-PARAMETER-REF
    DiagnosticParameterRef                                                 = 1015,
    /// DIAGNOSTIC-PERIODIC-RATE
    DiagnosticPeriodicRate                                                 = 5714,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 4103,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 3598,
    /// DIAGNOSTIC-PROPS
    DiagnosticProps                                                        = 813,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 2489,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 6472,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 1983,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 3046,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 1274,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 1322,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 573,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 3032,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 6593,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 3501,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 6504,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 2880,
    /// DIAGNOSTIC-RELEVANCE
    DiagnosticRelevance                                                    = 3971,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 1134,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 4680,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 1530,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 1778,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 3943,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 1513,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 3818,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 1201,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 206,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 3249,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 2999,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 5450,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE
    DiagnosticRequestFileTransferInterface                                 = 4915,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 1829,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 553,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 3288,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 6038,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 5894,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 196,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 3715,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 6040,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 3420,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 5947,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 4779,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 309,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 2123,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 158,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 5729,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 4450,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 4918,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 3129,
    /// DIAGNOSTIC-SECURE-CODING-MAPPING
    DiagnosticSecureCodingMapping                                          = 3558,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 5401,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 4793,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 3751,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 2826,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 2594,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 1039,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 5404,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 6526,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 6582,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 2398,
    /// DIAGNOSTIC-SERVICE-INSTANCE-REF
    DiagnosticServiceInstanceRef                                           = 2700,
    /// DIAGNOSTIC-SERVICE-INSTANCE-REF-CONDITIONAL
    DiagnosticServiceInstanceRefConditional                                = 2197,
    /// DIAGNOSTIC-SERVICE-INSTANCES
    DiagnosticServiceInstances                                             = 6079,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 5010,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 5395,
    /// DIAGNOSTIC-SERVICE-TABLE-REF
    DiagnosticServiceTableRef                                              = 5475,
    /// DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL
    DiagnosticServiceTableRefConditional                                   = 6173,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 5904,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 5322,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 153,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 3418,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 2406,
    /// DIAGNOSTIC-SESSION-REF
    DiagnosticSessionRef                                                   = 569,
    /// DIAGNOSTIC-SESSION-REFS
    DiagnosticSessionRefs                                                  = 3982,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 2280,
    /// DIAGNOSTIC-SOVD-ACCESS-ARGUMENT
    DiagnosticSovdAccessArgument                                           = 1867,
    /// DIAGNOSTIC-SOVD-ARRAY-CONTENT-ELEMENT
    DiagnosticSovdArrayContentElement                                      = 1365,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE
    DiagnosticSovdAuthorizationInterface                                   = 404,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING
    DiagnosticSovdAuthorizationPortMapping                                 = 1110,
    /// DIAGNOSTIC-SOVD-BULK-DATA
    DiagnosticSovdBulkData                                                 = 3710,
    /// DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE
    DiagnosticSovdBulkDataInterface                                        = 931,
    /// DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING
    DiagnosticSovdBulkDataPortMapping                                      = 1613,
    /// DIAGNOSTIC-SOVD-CONFIG-CONTENT-MAPPING
    DiagnosticSovdConfigContentMapping                                     = 3445,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA
    DiagnosticSovdConfigurationBulkData                                    = 4480,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING
    DiagnosticSovdConfigurationDataIdentifierMapping                       = 2352,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE
    DiagnosticSovdConfigurationInterface                                   = 5633,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER
    DiagnosticSovdConfigurationParameter                                   = 6453,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING
    DiagnosticSovdConfigurationPortMapping                                 = 6130,
    /// DIAGNOSTIC-SOVD-CONTENT
    DiagnosticSovdContent                                                  = 3889,
    /// DIAGNOSTIC-SOVD-CONTENT-ELEMENT-INTERFACE
    DiagnosticSovdContentElementInterface                                  = 5317,
    /// DIAGNOSTIC-SOVD-CONTENT-GROUP
    DiagnosticSovdContentGroup                                             = 5000,
    /// DIAGNOSTIC-SOVD-CONTENT-INTERFACE
    DiagnosticSovdContentInterface                                         = 4389,
    /// DIAGNOSTIC-SOVD-CONTENT-PORT-MAPPING
    DiagnosticSovdContentPortMapping                                       = 4183,
    /// DIAGNOSTIC-SOVD-DATA
    DiagnosticSovdData                                                     = 173,
    /// DIAGNOSTIC-SOVD-DATA-CATEGORY
    DiagnosticSovdDataCategory                                             = 406,
    /// DIAGNOSTIC-SOVD-FAULT-MEMORY-ACCESS
    DiagnosticSovdFaultMemoryAccess                                        = 909,
    /// DIAGNOSTIC-SOVD-GROUP
    DiagnosticSovdGroup                                                    = 2302,
    /// DIAGNOSTIC-SOVD-LOCK
    DiagnosticSovdLock                                                     = 1996,
    /// DIAGNOSTIC-SOVD-LOG
    DiagnosticSovdLog                                                      = 2661,
    /// DIAGNOSTIC-SOVD-METHOD
    DiagnosticSovdMethod                                                   = 5501,
    /// DIAGNOSTIC-SOVD-METHOD-PRIMITIVE
    DiagnosticSovdMethodPrimitive                                          = 6363,
    /// DIAGNOSTIC-SOVD-OPERATION
    DiagnosticSovdOperation                                                = 4475,
    /// DIAGNOSTIC-SOVD-OPERATION-INTERFACE
    DiagnosticSovdOperationInterface                                       = 4320,
    /// DIAGNOSTIC-SOVD-OPERATION-PORT-MAPPING
    DiagnosticSovdOperationPortMapping                                     = 1298,
    /// DIAGNOSTIC-SOVD-PRIMITIVE-CONTENT-ELEMENT
    DiagnosticSovdPrimitiveContentElement                                  = 6626,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE
    DiagnosticSovdProximityChallengeInterface                              = 3581,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING
    DiagnosticSovdProximityChallengePortMapping                            = 4496,
    /// DIAGNOSTIC-SOVD-RECORD-CONTENT-ELEMENT
    DiagnosticSovdRecordContentElement                                     = 3518,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE
    DiagnosticSovdServiceValidationInterface                               = 5456,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING
    DiagnosticSovdServiceValidationPortMapping                             = 5650,
    /// DIAGNOSTIC-SOVD-UPDATE
    DiagnosticSovdUpdate                                                   = 4428,
    /// DIAGNOSTIC-SOVD-UPDATE-INTERFACE
    DiagnosticSovdUpdateInterface                                          = 146,
    /// DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING
    DiagnosticSovdUpdatePortMapping                                        = 1726,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 1121,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 1921,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 133,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 4752,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF
    DiagnosticStorageConditionRef                                          = 969,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL
    DiagnosticStorageConditionRefConditional                               = 636,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 6010,
    /// DIAGNOSTIC-TEST-RESULT-REF
    DiagnosticTestResultRef                                                = 3569,
    /// DIAGNOSTIC-TEST-RESULT-REFS
    DiagnosticTestResultRefs                                               = 3272,
    /// DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT
    DiagnosticTestResultUpdateEnumValueVariationPoint                      = 1847,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 2221,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 6481,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 5870,
    /// DIAGNOSTIC-TRANSMIT-CERTIFICATE-INTERFACE
    DiagnosticTransmitCertificateInterface                                 = 652,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 4232,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 3792,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 913,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD-PROPS
    DiagnosticTroubleCodeObdProps                                          = 3996,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 3522,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS-REF
    DiagnosticTroubleCodePropsRef                                          = 4655,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS-REF-CONDITIONAL
    DiagnosticTroubleCodePropsRefConditional                               = 2312,
    /// DIAGNOSTIC-TROUBLE-CODE-REF
    DiagnosticTroubleCodeRef                                               = 3238,
    /// DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL
    DiagnosticTroubleCodeRefConditional                                    = 5208,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 249,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 6196,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 6181,
    /// DIAGNOSTIC-UDS-SEVERITY-ENUM-VALUE-VARIATION-POINT
    DiagnosticUdsSeverityEnumValueVariationPoint                           = 3495,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 4179,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 4716,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 6083,
    /// DIAGNOSTIC-VALUE-ACCESS
    DiagnosticValueAccess                                                  = 4236,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 3616,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 698,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 1763,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 6384,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 4671,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 5593,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 3384,
    /// DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT
    DiagnosticWwhObdDtcClassEnumValueVariationPoint                        = 3510,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 2702,
    /// DID-NUMBER
    DidNumber                                                              = 5492,
    /// DID-SIZE
    DidSize                                                                = 2326,
    /// DIFFERENTIATED-SERVICE-CODE-POINT
    DifferentiatedServiceCodePoint                                         = 5970,
    /// DIFFERENTIATED-SERVICE-FIELD
    DifferentiatedServiceField                                             = 6634,
    /// DIRECT-PDU-REF
    DirectPduRef                                                           = 455,
    /// DIRECT-SUCCESSOR-REF
    DirectSuccessorRef                                                     = 742,
    /// DIRECT-SUCCESSOR-REFS
    DirectSuccessorRefs                                                    = 985,
    /// DIRECT-TP-SDU-REF
    DirectTpSduRef                                                         = 763,
    /// DIRECTION
    Direction                                                              = 5743,
    /// DISABLE-END-TO-END-CHECK
    DisableEndToEndCheck                                                   = 3062,
    /// DISABLE-END-TO-END-STATE-MACHINE
    DisableEndToEndStateMachine                                            = 2913,
    /// DISABLED-IN-MODE-IREF
    DisabledInModeIref                                                     = 3839,
    /// DISABLED-IN-MODE-IREFS
    DisabledInModeIrefs                                                    = 5449,
    /// DISABLED-MODE-IREF
    DisabledModeIref                                                       = 1552,
    /// DISABLED-MODE-IREFS
    DisabledModeIrefs                                                      = 1273,
    /// DISCOVERY-PROTECTION-KIND
    DiscoveryProtectionKind                                                = 5053,
    /// DISCOVERY-TECHNOLOGY
    DiscoveryTechnology                                                    = 2451,
    /// DISCOVERY-TYPE
    DiscoveryType                                                          = 490,
    /// DISPLAY-FORMAT
    DisplayFormat                                                          = 6401,
    /// DISPLAY-IDENTIFIER
    DisplayIdentifier                                                      = 2011,
    /// DISPLAY-NAME
    DisplayName                                                            = 1868,
    /// DISPLAY-PRESENTATION
    DisplayPresentation                                                    = 1077,
    /// DISTINGUISHED-PARTITIONS
    DistinguishedPartitions                                                = 5713,
    /// DLT-APPLICATION
    DltApplication                                                         = 2643,
    /// DLT-APPLICATION-REF
    DltApplicationRef                                                      = 1358,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 2307,
    /// DLT-ARGUMENT
    DltArgument                                                            = 3709,
    /// DLT-ARGUMENT-ENTRYS
    DltArgumentEntrys                                                      = 2980,
    /// DLT-ARGUMENT-PROPS
    DltArgumentProps                                                       = 4263,
    /// DLT-ARGUMENT-PROPS-REF
    DltArgumentPropsRef                                                    = 2588,
    /// DLT-ARGUMENT-PROPS-REF-CONDITIONAL
    DltArgumentPropsRefConditional                                         = 4876,
    /// DLT-ARGUMENT-PROPS-SET
    DltArgumentPropsSet                                                    = 3178,
    /// DLT-ARGUMENT-PROPSS
    DltArgumentPropss                                                      = 5681,
    /// DLT-ARGUMENTS
    DltArguments                                                           = 3158,
    /// DLT-CONFIG
    DltConfig                                                              = 3955,
    /// DLT-CONTEXT
    DltContext                                                             = 2042,
    /// DLT-CONTEXT-REF
    DltContextRef                                                          = 2139,
    /// DLT-CONTEXT-REF-CONDITIONAL
    DltContextRefConditional                                               = 6642,
    /// DLT-ECU
    DltEcu                                                                 = 5627,
    /// DLT-ECU-ID
    DltEcuId                                                               = 91,
    /// DLT-ECU-REF
    DltEcuRef                                                              = 1215,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 2758,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 6382,
    /// DLT-LOG-CHANNEL-DESIGN-REF
    DltLogChannelDesignRef                                                 = 4438,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 6179,
    /// DLT-LOG-CHANNEL-REF
    DltLogChannelRef                                                       = 402,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 222,
    /// DLT-LOG-CHANNELS
    DltLogChannels                                                         = 234,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 6160,
    /// DLT-LOG-SINK-REF
    DltLogSinkRef                                                          = 5566,
    /// DLT-LOG-SINK-REFS
    DltLogSinkRefs                                                         = 697,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 1566,
    /// DLT-MESSAGE
    DltMessage                                                             = 1823,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 3065,
    /// DLT-MESSAGE-REF
    DltMessageRef                                                          = 5873,
    /// DLT-MESSAGE-REF-CONDITIONAL
    DltMessageRefConditional                                               = 3851,
    /// DLT-MESSAGE-REFS
    DltMessageRefs                                                         = 4492,
    /// DLT-MESSAGES
    DltMessages                                                            = 818,
    /// DLT-SESSION-ID
    DltSessionId                                                           = 6299,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 2760,
    /// DNS-SERVER-ADDRESS
    DnsServerAddress                                                       = 6412,
    /// DNS-SERVER-ADDRESSES
    DnsServerAddresses                                                     = 3297,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 6464,
    /// DO-IP-CONFIG
    DoIpConfig                                                             = 382,
    /// DO-IP-DESIGN-REF
    DoIpDesignRef                                                          = 4235,
    /// DO-IP-ENTITY
    DoIpEntity                                                             = 118,
    /// DO-IP-ENTITY-ROLE
    DoIpEntityRole                                                         = 2070,
    /// DO-IP-FUNCTIONAL-CLUSTER-DESIGN
    DoIpFunctionalClusterDesign                                            = 5277,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 4216,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 3770,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 2937,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 3920,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 3282,
    /// DO-IP-LOGIC-ADDRESS-PROPS
    DoIpLogicAddressProps                                                  = 6240,
    /// DO-IP-LOGIC-ADDRESSS
    DoIpLogicAddresss                                                      = 1943,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 314,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 3776,
    /// DO-IP-LOGICAL-ADDRESS
    DoIpLogicalAddress                                                     = 936,
    /// DO-IP-LOGICAL-ADDRESS-REF
    DoIpLogicalAddressRef                                                  = 5385,
    /// DO-IP-NETWORK-CONFIGURATION
    DoIpNetworkConfiguration                                               = 2945,
    /// DO-IP-NETWORK-CONFIGURATION-DESIGN
    DoIpNetworkConfigurationDesign                                         = 4859,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 731,
    /// DO-IP-PROTOCOL-VERSION
    DoIpProtocolVersion                                                    = 1550,
    /// DO-IP-REQUEST-CONFIGURATION
    DoIpRequestConfiguration                                               = 4848,
    /// DO-IP-REQUEST-CONFIGURATION-DESIGN
    DoIpRequestConfigurationDesign                                         = 2896,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 523,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 397,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 5403,
    /// DO-IP-ROUTING-ACTIVATIONS
    DoIpRoutingActivations                                                 = 3024,
    /// DO-IP-RULE
    DoIpRule                                                               = 4143,
    /// DO-IP-SOURCE-ADDRESS-REF
    DoIpSourceAddressRef                                                   = 2373,
    /// DO-IP-TARGET-ADDRESS-REF
    DoIpTargetAddressRef                                                   = 4051,
    /// DO-IP-TARGET-ADDRESS-REFS
    DoIpTargetAddressRefs                                                  = 1830,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REF
    DoIpTesterRoutingActivationRef                                         = 821,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REFS
    DoIpTesterRoutingActivationRefs                                        = 2902,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 6200,
    /// DO-IP-TP-CONNECTION
    DoIpTpConnection                                                       = 4829,
    /// DO-NOT-FRAGMENT
    DoNotFragment                                                          = 3159,
    /// DOC-REVISION
    DocRevision                                                            = 3007,
    /// DOC-REVISIONS
    DocRevisions                                                           = 3370,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 5397,
    /// DOCUMENT-ELEMENT-SCOPES
    DocumentElementScopes                                                  = 737,
    /// DOCUMENTATION
    Documentation                                                          = 6622,
    /// DOCUMENTATION-CONTENT
    DocumentationContent                                                   = 6588,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 2894,
    /// DOIP-CHANNEL-COLLECTION-REF
    DoipChannelCollectionRef                                               = 2038,
    /// DOIP-CONNECTION-REF
    DoipConnectionRef                                                      = 5084,
    /// DOIP-CONNECTION-REFS
    DoipConnectionRefs                                                     = 1267,
    /// DOIP-INTERFACES
    DoipInterfaces                                                         = 2630,
    /// DOMAIN
    Domain                                                                 = 5939,
    /// DOMAIN-ID
    DomainId                                                               = 5065,
    /// DOMAIN-IDS
    DomainIds                                                              = 608,
    /// DPD-ACTION
    DpdAction                                                              = 2111,
    /// DPD-DELAY
    DpdDelay                                                               = 1323,
    /// DPG-DOES-NOT-REQUIRE-COHERENCYS
    DpgDoesNotRequireCoherencys                                            = 3135,
    /// DPG-REQUIRES-COHERENCYS
    DpgRequiresCoherencys                                                  = 1843,
    /// DRIVER-NOTIFICATIONS
    DriverNotifications                                                    = 5559,
    /// DRIVER-PRIMITIVE-REF
    DriverPrimitiveRef                                                     = 3044,
    /// DROP-NOT-REQUESTED-NAD
    DropNotRequestedNad                                                    = 3336,
    /// DTC-FORMAT-TYPE
    DtcFormatType                                                          = 3752,
    /// DTC-KIND
    DtcKind                                                                = 6501,
    /// DTC-NUMBER
    DtcNumber                                                              = 3941,
    /// DTC-PROPS-REF
    DtcPropsRef                                                            = 2246,
    /// DTC-SETTING-CLASS-REF
    DtcSettingClassRef                                                     = 5087,
    /// DTC-SETTING-PARAMETER
    DtcSettingParameter                                                    = 5624,
    /// DTC-STATUS-AVAILABILITY-MASK
    DtcStatusAvailabilityMask                                              = 72,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 1089,
    /// DTC-STATUS-MASK
    DtcStatusMask                                                          = 2528,
    /// DTCS
    Dtcs                                                                   = 489,
    /// DURABILITY
    Durability                                                             = 446,
    /// DURABILITY-KIND
    DurabilityKind                                                         = 5044,
    /// DURABILITY-SERVICE
    DurabilityService                                                      = 3910,
    /// DURABILITY-SERVICE-CLEANUP-DELAY
    DurabilityServiceCleanupDelay                                          = 5836,
    /// DURABILITY-SERVICE-HISTORY-DEPTH
    DurabilityServiceHistoryDepth                                          = 1526,
    /// DURABILITY-SERVICE-HISTORY-KIND
    DurabilityServiceHistoryKind                                           = 1612,
    /// DURABILITY-SERVICE-MAX-INSTANCES
    DurabilityServiceMaxInstances                                          = 3092,
    /// DURABILITY-SERVICE-MAX-SAMPLES
    DurabilityServiceMaxSamples                                            = 4753,
    /// DURABILITY-SERVICE-MAX-SAMPLES-PER-INSTANCE
    DurabilityServiceMaxSamplesPerInstance                                 = 5041,
    /// DYNAMIC-ACTION-REF
    DynamicActionRef                                                       = 2327,
    /// DYNAMIC-ACTION-REFS
    DynamicActionRefs                                                      = 6529,
    /// DYNAMIC-ARRAY-SIZE-PROFILE
    DynamicArraySizeProfile                                                = 2054,
    /// DYNAMIC-BS
    DynamicBs                                                              = 5378,
    /// DYNAMIC-LENGTH
    DynamicLength                                                          = 2776,
    /// DYNAMIC-PART
    DynamicPart                                                            = 4420,
    /// DYNAMIC-PART-ALTERNATIVE
    DynamicPartAlternative                                                 = 293,
    /// DYNAMIC-PART-ALTERNATIVES
    DynamicPartAlternatives                                                = 5660,
    /// DYNAMIC-PARTS
    DynamicParts                                                           = 1410,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REF
    DynamicPncMappingPduGroupRef                                           = 4528,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS
    DynamicPncMappingPduGroupRefs                                          = 5398,
    /// DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED
    DynamicPncToChannelMappingEnabled                                      = 5964,
    /// DYNAMIC-RUNTIME-LENGTH-HANDLING
    DynamicRuntimeLengthHandling                                           = 2254,
    /// DYNAMIC-SLOT-IDLE-PHASE
    DynamicSlotIdlePhase                                                   = 4975,
    /// DYNAMICALLY-ASSIGNED
    DynamicallyAssigned                                                    = 3160,
    /// DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF
    DynamicallyDefineDataIdentifierClassRef                                = 6302,
    /// E
    E                                                                      = 1238,
    /// E-2-E-COUNTER-ERROR-REF
    E2ECounterErrorRef                                                     = 6456,
    /// E-2-E-CRC-ERROR-REF
    E2ECrcErrorRef                                                         = 5497,
    /// E-2-E-EVENT-PROTECTION-PROPSS
    E2EEventProtectionPropss                                               = 3954,
    /// E-2-E-METHOD-PROTECTION-PROPSS
    E2EMethodProtectionPropss                                              = 3562,
    /// E-2-E-NO-NEW-DATA-EXTENSION-FACTOR
    E2ENoNewDataExtensionFactor                                            = 2085,
    /// E-2-E-OVERSAMPLING-PERIOD
    E2EOversamplingPeriod                                                  = 184,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 4532,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS-REF
    E2EProfileCompatibilityPropsRef                                        = 52,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 5980,
    /// E-2-E-PROFILE-CONFIGURATION-REF
    E2EProfileConfigurationRef                                             = 3648,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 2992,
    /// E-2-E-PROFILE-CONFIGURATIONS
    E2EProfileConfigurations                                               = 3671,
    /// E-2-E-TIMEOUT-ERROR-REF
    E2ETimeoutErrorRef                                                     = 496,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 6579,
    /// ECU-COMM-PORT-INSTANCES
    EcuCommPortInstances                                                   = 1459,
    /// ECU-CONFIGURATION-REF
    EcuConfigurationRef                                                    = 2978,
    /// ECU-EXTRACT-REF
    EcuExtractRef                                                          = 183,
    /// ECU-EXTRACT-REFERENCE-IREF
    EcuExtractReferenceIref                                                = 6416,
    /// ECU-EXTRACT-VERSION
    EcuExtractVersion                                                      = 3815,
    /// ECU-ID
    EcuId                                                                  = 2508,
    /// ECU-INSTANCE
    EcuInstance                                                            = 3743,
    /// ECU-INSTANCE-PROPS
    EcuInstanceProps                                                       = 4987,
    /// ECU-INSTANCE-PROPSS
    EcuInstancePropss                                                      = 4174,
    /// ECU-INSTANCE-REF
    EcuInstanceRef                                                         = 3398,
    /// ECU-INSTANCE-REF-CONDITIONAL
    EcuInstanceRefConditional                                              = 4014,
    /// ECU-INSTANCE-REFS
    EcuInstanceRefs                                                        = 3683,
    /// ECU-INSTANCES
    EcuInstances                                                           = 2935,
    /// ECU-MAPPING
    EcuMapping                                                             = 5410,
    /// ECU-PARTITION
    EcuPartition                                                           = 3305,
    /// ECU-PARTITION-REF
    EcuPartitionRef                                                        = 2495,
    /// ECU-PARTITION-TO-CORE-MAPPING
    EcuPartitionToCoreMapping                                              = 2030,
    /// ECU-PARTITION-TO-CORE-MAPPINGS
    EcuPartitionToCoreMappings                                             = 3852,
    /// ECU-REF
    EcuRef                                                                 = 5893,
    /// ECU-RESET-CLASS-REF
    EcuResetClassRef                                                       = 4820,
    /// ECU-RESOURCE-ESTIMATION
    EcuResourceEstimation                                                  = 1250,
    /// ECU-RESOURCE-MAPPINGS
    EcuResourceMappings                                                    = 4342,
    /// ECU-SCOPE-REF
    EcuScopeRef                                                            = 6001,
    /// ECU-SCOPE-REFS
    EcuScopeRefs                                                           = 2714,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 1918,
    /// ECU-TASK-PROXY-REF
    EcuTaskProxyRef                                                        = 3702,
    /// ECU-TASK-PROXY-REFS
    EcuTaskProxyRefs                                                       = 2777,
    /// ECU-TIMING
    EcuTiming                                                              = 6223,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 2505,
    /// ECUC-ADD-INFO-PARAM-VALUE
    EcucAddInfoParamValue                                                  = 1330,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 4242,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 4146,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 2845,
    /// ECUC-COND
    EcucCond                                                               = 606,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 6049,
    /// ECUC-DEF-EDITION
    EcucDefEdition                                                         = 4629,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 2088,
    /// ECUC-DEFINITION-REF
    EcucDefinitionRef                                                      = 5239,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 1703,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 1796,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 5883,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 5771,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 50,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 6317,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 1044,
    /// ECUC-FUNCTION-NAME-DEF-CONDITIONAL
    EcucFunctionNameDefConditional                                         = 1731,
    /// ECUC-FUNCTION-NAME-DEF-VARIANTS
    EcucFunctionNameDefVariants                                            = 4100,
    /// ECUC-IMPLEMENTATION-CONFIGURATION-CLASS
    EcucImplementationConfigurationClass                                   = 5571,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 1066,
    /// ECUC-INSTANCE-REFERENCE-VALUE
    EcucInstanceReferenceValue                                             = 890,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 3834,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 6189,
    /// ECUC-LINKER-SYMBOL-DEF-CONDITIONAL
    EcucLinkerSymbolDefConditional                                         = 2778,
    /// ECUC-LINKER-SYMBOL-DEF-VARIANTS
    EcucLinkerSymbolDefVariants                                            = 6032,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 5012,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF
    EcucModuleConfigurationValuesRef                                       = 1808,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL
    EcucModuleConfigurationValuesRefConditional                            = 4161,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 2565,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 253,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL
    EcucMultilineStringParamDefConditional                                 = 6570,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS
    EcucMultilineStringParamDefVariants                                    = 1622,
    /// ECUC-MULTIPLICITY-CONFIGURATION-CLASS
    EcucMultiplicityConfigurationClass                                     = 3220,
    /// ECUC-NUMERICAL-PARAM-VALUE
    EcucNumericalParamValue                                                = 1104,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 2832,
    /// ECUC-QUERY
    EcucQuery                                                              = 1535,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 5281,
    /// ECUC-QUERY-REF
    EcucQueryRef                                                           = 145,
    /// ECUC-QUERY-STRING-REF
    EcucQueryStringRef                                                     = 6251,
    /// ECUC-QUERYS
    EcucQuerys                                                             = 2664,
    /// ECUC-REF
    EcucRef                                                                = 6536,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 4612,
    /// ECUC-REFERENCE-VALUE
    EcucReferenceValue                                                     = 6241,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 3279,
    /// ECUC-STRING-PARAM-DEF-CONDITIONAL
    EcucStringParamDefConditional                                          = 2838,
    /// ECUC-STRING-PARAM-DEF-VARIANTS
    EcucStringParamDefVariants                                             = 837,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 1244,
    /// ECUC-TEXTUAL-PARAM-VALUE
    EcucTextualParamValue                                                  = 5097,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 4572,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 5498,
    /// ECUC-VALIDATION-CONDS
    EcucValidationConds                                                    = 2338,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 5454,
    /// ECUC-VALUE-CONFIGURATION-CLASS
    EcucValueConfigurationClass                                            = 1977,
    /// ECUC-VALUES
    EcucValues                                                             = 2091,
    /// EGRESS-PORT-REF
    EgressPortRef                                                          = 5485,
    /// EGRESS-PORT-REFS
    EgressPortRefs                                                         = 4080,
    /// EID
    Eid                                                                    = 5348,
    /// EID-RETRIEVAL
    EidRetrieval                                                           = 3654,
    /// EID-USE-MAC
    EidUseMac                                                              = 2391,
    /// ELEMENT
    Element                                                                = 5936,
    /// ELEMENT-BYTE-VALUE
    ElementByteValue                                                       = 4338,
    /// ELEMENT-CREATION
    ElementCreation                                                        = 1408,
    /// ELEMENT-GROUPS
    ElementGroups                                                          = 823,
    /// ELEMENT-IN-IMPL-DATATYPE
    ElementInImplDatatype                                                  = 6427,
    /// ELEMENT-MAPPINGS
    ElementMappings                                                        = 3417,
    /// ELEMENT-POSITION
    ElementPosition                                                        = 5221,
    /// ELEMENT-PROPSS
    ElementPropss                                                          = 1748,
    /// ELEMENT-REF
    ElementRef                                                             = 3744,
    /// ELEMENT-REFS
    ElementRefs                                                            = 2289,
    /// ELEMENT-ROLE
    ElementRole                                                            = 3638,
    /// ELEMENTS
    Elements                                                               = 4750,
    /// ELLIPTIC-CURVE-REF
    EllipticCurveRef                                                       = 2732,
    /// ELLIPTIC-CURVE-REFS
    EllipticCurveRefs                                                      = 4940,
    /// EMPTY-SIGNAL
    EmptySignal                                                            = 4509,
    /// EMULATION-SUPPORTS
    EmulationSupports                                                      = 2074,
    /// ENABLE-ANYCAST
    EnableAnycast                                                          = 440,
    /// ENABLE-CONDITION-GROUP-REF
    EnableConditionGroupRef                                                = 2513,
    /// ENABLE-CONDITION-REF
    EnableConditionRef                                                     = 911,
    /// ENABLE-CONDITIONS
    EnableConditions                                                       = 5364,
    /// ENABLE-DEACTIVATION
    EnableDeactivation                                                     = 4082,
    /// ENABLE-DISCOVERY-PROTECTION
    EnableDiscoveryProtection                                              = 777,
    /// ENABLE-JOIN-ACCESS-CONTROL
    EnableJoinAccessControl                                                = 1713,
    /// ENABLE-LIVELINESS-PROTECTION
    EnableLivelinessProtection                                             = 3813,
    /// ENABLE-READ-ACCESS-CONTROL
    EnableReadAccessControl                                                = 1192,
    /// ENABLE-TAKE-ADDRESS
    EnableTakeAddress                                                      = 1532,
    /// ENABLE-UPDATE
    EnableUpdate                                                           = 1124,
    /// ENABLE-WRITE-ACCESS-CONTROL
    EnableWriteAccessControl                                               = 2415,
    /// ENABLES-LOG-TRACE
    EnablesLogTrace                                                        = 5086,
    /// ENCAPSULATED-ENTRY-REF
    EncapsulatedEntryRef                                                   = 838,
    /// ENCODING
    Encoding                                                               = 4410,
    /// ENCRYPTION-REF
    EncryptionRef                                                          = 4091,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 1509,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 2426,
    /// END-ADDRESS
    EndAddress                                                             = 4871,
    /// END-TO-END-CALL-RESPONSE-TIMEOUT
    EndToEndCallResponseTimeout                                            = 3163,
    /// END-TO-END-PROFILE
    EndToEndProfile                                                        = 4335,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 2402,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 3232,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDUS
    EndToEndProtectionISignalIPdus                                         = 3899,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 5690,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 2693,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPES
    EndToEndProtectionVariablePrototypes                                   = 346,
    /// END-TO-END-PROTECTIONS
    EndToEndProtections                                                    = 2609,
    /// END-TO-END-TRANSFORMATION-COM-SPEC-PROPS
    EndToEndTransformationComSpecProps                                     = 3557,
    /// END-TO-END-TRANSFORMATION-DESCRIPTION
    EndToEndTransformationDescription                                      = 6568,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS
    EndToEndTransformationISignalProps                                     = 201,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    EndToEndTransformationISignalPropsConditional                          = 4063,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    EndToEndTransformationISignalPropsVariants                             = 5663,
    /// ENDPOINT-CONFIGURATION-REF
    EndpointConfigurationRef                                               = 5351,
    /// ENDPOINTS
    Endpoints                                                              = 5928,
    /// ENGINEERING-OBJECT
    EngineeringObject                                                      = 3086,
    /// ENGINEERING-OBJECTS
    EngineeringObjects                                                     = 6563,
    /// ENHANCED-MODE-API
    EnhancedModeApi                                                        = 4138,
    /// ENTER-SUSPEND-TO-RAM-ERROR-REF
    EnterSuspendToRamErrorRef                                              = 3236,
    /// ENTER-TIMEOUT-VALUE
    EnterTimeoutValue                                                      = 978,
    /// ENTERED-MODE-REF
    EnteredModeRef                                                         = 3898,
    /// ENTITY-STATUS-MAX-BYTE-FIELD-USE
    EntityStatusMaxByteFieldUse                                            = 2726,
    /// ENTITYS
    Entitys                                                                = 1045,
    /// ENTRY
    Entry                                                                  = 1634,
    /// ENTRY-MODE-DECLARATION-REF
    EntryModeDeclarationRef                                                = 4827,
    /// ENTRY-REF
    EntryRef                                                               = 5521,
    /// ENTRY-TYPE
    EntryType                                                              = 3026,
    /// ENTRYS
    Entrys                                                                 = 5995,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 4622,
    /// ENUMERATOR-VALUE
    EnumeratorValue                                                        = 5466,
    /// ENUMS
    Enums                                                                  = 885,
    /// ENVIRONMENT-CAPTURE-TO-REPORTING
    EnvironmentCaptureToReporting                                          = 6640,
    /// ENVIRONMENT-DATA-CAPTURE
    EnvironmentDataCapture                                                 = 5811,
    /// ENVIRONMENT-VARIABLES
    EnvironmentVariables                                                   = 2581,
    /// ENVIRONMENTAL-CONDITION-REF
    EnvironmentalConditionRef                                              = 3959,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 828,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 5306,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 2131,
    /// ERROR-CODE
    ErrorCode                                                              = 6435,
    /// ERROR-CONTEXT-REF
    ErrorContextRef                                                        = 1966,
    /// ERROR-CONTEXT-REFS
    ErrorContextRefs                                                       = 4286,
    /// ERROR-DOMAIN-REF
    ErrorDomainRef                                                         = 2559,
    /// ERROR-HANDLING
    ErrorHandling                                                          = 4304,
    /// ERROR-MAPPINGS
    ErrorMappings                                                          = 6650,
    /// ERROR-REACTION-POLICY
    ErrorReactionPolicy                                                    = 2792,
    /// ERROR-SIGNALING-ENABLED
    ErrorSignalingEnabled                                                  = 4665,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 3059,
    /// ESP-CIPHER-SUITE-NAME
    EspCipherSuiteName                                                     = 2282,
    /// ESP-CIPHER-SUITE-NAMES
    EspCipherSuiteNames                                                    = 2456,
    /// ESTIMATED-DURATION-OF-CAMPAIGN
    EstimatedDurationOfCampaign                                            = 6358,
    /// ESTIMATED-DURATION-OF-OPERATION
    EstimatedDurationOfOperation                                           = 879,
    /// ESTIMATED-EXECUTION-TIME
    EstimatedExecutionTime                                                 = 2330,
    /// ETH-GLOBAL-TIME-DOMAIN-PROPS
    EthGlobalTimeDomainProps                                               = 1120,
    /// ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT
    EthGlobalTimeManagedCouplingPort                                       = 2596,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 5352,
    /// ETH-IP-PROPS-REF
    EthIpPropsRef                                                          = 5107,
    /// ETH-IP-PROPS-REFS
    EthIpPropsRefs                                                         = 5099,
    /// ETH-SWITCH-PORT-GROUP-DERIVATION
    EthSwitchPortGroupDerivation                                           = 627,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 5822,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 6014,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 3617,
    /// ETH-TP-CONNECTION
    EthTpConnection                                                        = 5651,
    /// ETHER-TYPE
    EtherType                                                              = 971,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 392,
    /// ETHERNET-CLUSTER-CONDITIONAL
    EthernetClusterConditional                                             = 198,
    /// ETHERNET-CLUSTER-REF
    EthernetClusterRef                                                     = 2915,
    /// ETHERNET-CLUSTER-VARIANTS
    EthernetClusterVariants                                                = 4949,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 4510,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 3298,
    /// ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL
    EthernetCommunicationControllerConditional                             = 2866,
    /// ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS
    EthernetCommunicationControllerVariants                                = 389,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 5307,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 4404,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 3733,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 5738,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 3162,
    /// ETHERNET-PRIORITY-REGENERATIONS
    EthernetPriorityRegenerations                                          = 1543,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 2136,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 1627,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 3397,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING-REF
    EthernetRawDataStreamMappingRef                                        = 5414,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 3281,
    /// ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS
    EthernetTrafficClassAssignments                                        = 1903,
    /// ETHERNET-VLAN-TRANSLATION-TABLE
    EthernetVlanTranslationTable                                           = 4454,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 4917,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 1030,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS
    EthernetWakeupSleepOnDatalineConfigs                                   = 5874,
    /// ETS-AVAILABLE-BANDWIDTH-IN-PERCENT
    EtsAvailableBandwidthInPercent                                         = 1198,
    /// ETS-AVAILABLE-BANDWIDTH-IN-WEIGHT-VALUE
    EtsAvailableBandwidthInWeightValue                                     = 6116,
    /// EVALUATED-ELEMENT-REF
    EvaluatedElementRef                                                    = 1290,
    /// EVALUATED-ELEMENT-REFS
    EvaluatedElementRefs                                                   = 1736,
    /// EVALUATED-VARIANT-REF
    EvaluatedVariantRef                                                    = 4687,
    /// EVALUATED-VARIANT-REFS
    EvaluatedVariantRefs                                                   = 1675,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 360,
    /// EVALUATION-ID
    EvaluationId                                                           = 2514,
    /// EVENT-CLEAR-ALLOWED
    EventClearAllowed                                                      = 3389,
    /// EVENT-COMBINATION-REPORTING-BEHAVIOR
    EventCombinationReportingBehavior                                      = 1950,
    /// EVENT-CONTROLLED-TIMING
    EventControlledTiming                                                  = 3807,
    /// EVENT-DEFAULT-VALUE
    EventDefaultValue                                                      = 6030,
    /// EVENT-DEPLOYMENTS
    EventDeployments                                                       = 5195,
    /// EVENT-DISPLACEMENT-STRATEGY
    EventDisplacementStrategy                                              = 1373,
    /// EVENT-ELEMENT-MAPPINGS
    EventElementMappings                                                   = 6621,
    /// EVENT-ELEMENTS
    EventElements                                                          = 1746,
    /// EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD
    EventFailureCycleCounterThreshold                                      = 5618,
    /// EVENT-GROUP-CONTROL-TYPE
    EventGroupControlType                                                  = 67,
    /// EVENT-GROUP-ID
    EventGroupId                                                           = 3073,
    /// EVENT-GROUP-IDENTIFIER
    EventGroupIdentifier                                                   = 5298,
    /// EVENT-GROUP-REF
    EventGroupRef                                                          = 791,
    /// EVENT-GROUP-REFS
    EventGroupRefs                                                         = 5004,
    /// EVENT-GROUPS
    EventGroups                                                            = 1264,
    /// EVENT-HANDLER
    EventHandler                                                           = 1249,
    /// EVENT-HANDLER-REF
    EventHandlerRef                                                        = 3716,
    /// EVENT-HANDLER-REFS
    EventHandlerRefs                                                       = 1473,
    /// EVENT-HANDLERS
    EventHandlers                                                          = 2058,
    /// EVENT-ID
    EventId                                                                = 4775,
    /// EVENT-IREF
    EventIref                                                              = 78,
    /// EVENT-KIND
    EventKind                                                              = 1621,
    /// EVENT-MAPPING
    EventMapping                                                           = 2586,
    /// EVENT-MAPPINGS
    EventMappings                                                          = 4774,
    /// EVENT-MULTICAST-ADDRESSS
    EventMulticastAddresss                                                 = 2834,
    /// EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS
    EventMulticastSubscriptionAddresss                                     = 3480,
    /// EVENT-MULTICAST-UDP-PORT
    EventMulticastUdpPort                                                  = 2194,
    /// EVENT-NEEDS-REF
    EventNeedsRef                                                          = 6222,
    /// EVENT-OBD-READINESS-GROUP
    EventObdReadinessGroup                                                 = 3915,
    /// EVENT-OCCURRENCE-KIND
    EventOccurrenceKind                                                    = 1925,
    /// EVENT-PROPSS
    EventPropss                                                            = 5547,
    /// EVENT-QOS-PROPSS
    EventQosPropss                                                         = 2286,
    /// EVENT-READINESS-GROUPS
    EventReadinessGroups                                                   = 6437,
    /// EVENT-RECEPTION-DEFAULT-VALUE
    EventReceptionDefaultValue                                             = 1442,
    /// EVENT-REF
    EventRef                                                               = 6585,
    /// EVENT-REFS
    EventRefs                                                              = 1342,
    /// EVENT-SOURCE-REF
    EventSourceRef                                                         = 5436,
    /// EVENT-SYMBOL-NAME
    EventSymbolName                                                        = 868,
    /// EVENT-TOPIC-ACCESS-RULE-REF
    EventTopicAccessRuleRef                                                = 5829,
    /// EVENT-TRIGGERING-REF
    EventTriggeringRef                                                     = 3403,
    /// EVENT-WINDOW-TIME
    EventWindowTime                                                        = 1292,
    /// EVENT-WINDOWS
    EventWindows                                                           = 5780,
    /// EVENTS
    Events                                                                 = 616,
    /// EXCESS-BURST-SIZE
    ExcessBurstSize                                                        = 2668,
    /// EXCESS-INFORMATION-RATE
    ExcessInformationRate                                                  = 3308,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 2990,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 5144,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REF
    ExclusiveAreaNestingOrderRef                                           = 5096,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REFS
    ExclusiveAreaNestingOrderRefs                                          = 1789,
    /// EXCLUSIVE-AREA-NESTING-ORDERS
    ExclusiveAreaNestingOrders                                             = 4253,
    /// EXCLUSIVE-AREA-POLICYS
    ExclusiveAreaPolicys                                                   = 5210,
    /// EXCLUSIVE-AREA-REF
    ExclusiveAreaRef                                                       = 884,
    /// EXCLUSIVE-AREA-REF-CONDITIONAL
    ExclusiveAreaRefConditional                                            = 4831,
    /// EXCLUSIVE-AREA-REFS
    ExclusiveAreaRefs                                                      = 3011,
    /// EXCLUSIVE-AREAS
    ExclusiveAreas                                                         = 3363,
    /// EXEC-IN-USER-MODE
    ExecInUserMode                                                         = 1304,
    /// EXECUTABLE
    Executable                                                             = 2278,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 481,
    /// EXECUTABLE-ENTITY-REF
    ExecutableEntityRef                                                    = 418,
    /// EXECUTABLE-ENTITY-REFS
    ExecutableEntityRefs                                                   = 444,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 4682,
    /// EXECUTABLE-LOGGING-IMPLEMENTATION-PROPS
    ExecutableLoggingImplementationProps                                   = 786,
    /// EXECUTABLE-PROVIDED-PORT-IREF
    ExecutableProvidedPortIref                                             = 5721,
    /// EXECUTABLE-REF
    ExecutableRef                                                          = 3451,
    /// EXECUTABLE-REFS
    ExecutableRefs                                                         = 3173,
    /// EXECUTABLE-REQUIRED-PORT-IREF
    ExecutableRequiredPortIref                                             = 3908,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 2553,
    /// EXECUTE-DESPITE-DATA-UNAVAILABILITY
    ExecuteDespiteDataUnavailability                                       = 1993,
    /// EXECUTION-CONTEXT
    ExecutionContext                                                       = 6328,
    /// EXECUTION-CONTEXT-REF
    ExecutionContextRef                                                    = 4311,
    /// EXECUTION-CONTEXT-REFS
    ExecutionContextRefs                                                   = 536,
    /// EXECUTION-CONTEXTS
    ExecutionContexts                                                      = 3986,
    /// EXECUTION-DEPENDENCY
    ExecutionDependency                                                    = 6595,
    /// EXECUTION-DEPENDENCYS
    ExecutionDependencys                                                   = 1615,
    /// EXECUTION-ERROR
    ExecutionError                                                         = 4241,
    /// EXECUTION-ERROR-REF
    ExecutionErrorRef                                                      = 4603,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 589,
    /// EXECUTION-ORDER-CONSTRAINT-TYPE
    ExecutionOrderConstraintType                                           = 1428,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 2266,
    /// EXECUTION-TIME-TYPE
    ExecutionTimeType                                                      = 1392,
    /// EXECUTION-TIMES
    ExecutionTimes                                                         = 3453,
    /// EXIT-MODE-DECLARATION-REF
    ExitModeDeclarationRef                                                 = 4390,
    /// EXIT-TIMEOUT-VALUE
    ExitTimeoutValue                                                       = 256,
    /// EXITED-MODE-REF
    ExitedModeRef                                                          = 4417,
    /// EXPECTED-ALIVE-CYCLE
    ExpectedAliveCycle                                                     = 3077,
    /// EXPECTED-ALIVE-INDICATIONS
    ExpectedAliveIndications                                               = 3008,
    /// EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE
    ExpectedAliveIndicationsInSupervisionCycle                             = 4554,
    /// EXPECTED-CALLBACKS
    ExpectedCallbacks                                                      = 1021,
    /// EXPECTED-ENTRYS
    ExpectedEntrys                                                         = 2211,
    /// EXPECTED-TX-TRIGGER
    ExpectedTxTrigger                                                      = 2380,
    /// EXPIRED-SUPERVISION-CYCLE-TOL
    ExpiredSupervisionCycleTol                                             = 2929,
    /// EXPIRED-SUPERVISION-CYCLES-TOLERANCE
    ExpiredSupervisionCyclesTolerance                                      = 2117,
    /// EXPIRED-SUPERVISION-TOLERANCE
    ExpiredSupervisionTolerance                                            = 6400,
    /// EXPLICIT-CONGESTION-NOTIFICATION
    ExplicitCongestionNotification                                         = 2237,
    /// EXPLICIT-INTER-RUNNABLE-VARIABLES
    ExplicitInterRunnableVariables                                         = 2600,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REF
    ExplicitRptProfileSelectionRef                                         = 3385,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REFS
    ExplicitRptProfileSelectionRefs                                        = 4818,
    /// EXPLICIT-WAKEUP-CHANNELS
    ExplicitWakeupChannels                                                 = 1700,
    /// EXPLICIT-WAKEUP-PNCS
    ExplicitWakeupPncs                                                     = 449,
    /// EXPORTABILITY
    Exportability                                                          = 6305,
    /// EXPRESSION
    Expression                                                             = 3601,
    /// EXPRESSION-REF
    ExpressionRef                                                          = 5520,
    /// EXT-HEADER-FILTER-LISTS
    ExtHeaderFilterLists                                                   = 3607,
    /// EXTENDED-ADDRESSING
    ExtendedAddressing                                                     = 830,
    /// EXTENDED-DATA-RECORD-ELEMENT-REF
    ExtendedDataRecordElementRef                                           = 3627,
    /// EXTENDED-DATA-RECORDS
    ExtendedDataRecords                                                    = 3421,
    /// EXTENDS-META-CLASS
    ExtendsMetaClass                                                       = 1236,
    /// EXTERN-OFFSET-CORRECTION
    ExternOffsetCorrection                                                 = 3227,
    /// EXTERN-RATE-CORRECTION
    ExternRateCorrection                                                   = 178,
    /// EXTERNAL-AUTHENTICATIONS
    ExternalAuthentications                                                = 3333,
    /// EXTERNAL-CLOCK-SYNCHRONISATION
    ExternalClockSynchronisation                                           = 4661,
    /// EXTERNAL-REPLACEMENT-REF
    ExternalReplacementRef                                                 = 764,
    /// EXTERNAL-SYNC
    ExternalSync                                                           = 6188,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 4357,
    /// EXTERNAL-TRIGGERING-POINT
    ExternalTriggeringPoint                                                = 100,
    /// EXTERNAL-TRIGGERING-POINTS
    ExternalTriggeringPoints                                               = 2691,
    /// FACTOR-SI-TO-UNIT
    FactorSiToUnit                                                         = 96,
    /// FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL
    FailedAliveSupervisionRefCycleTol                                      = 3075,
    /// FAILED-REFERENCE-CYCLES-TOLERANCE
    FailedReferenceCyclesTolerance                                         = 3644,
    /// FAILED-SUPERVISION-CYCLES-TOLERANCE
    FailedSupervisionCyclesTolerance                                       = 168,
    /// FAILURE-MONITORING-REF
    FailureMonitoringRef                                                   = 6171,
    /// FALL-BACK-INTERNAL
    FallBackInternal                                                       = 2539,
    /// FALSE-ACTION-LIST-REF
    FalseActionListRef                                                     = 1295,
    /// FDC-THRESHOLD-STORAGE-VALUE
    FdcThresholdStorageValue                                               = 4694,
    /// FEATURE-IREF
    FeatureIref                                                            = 2408,
    /// FEATURE-MODEL-REF
    FeatureModelRef                                                        = 4018,
    /// FEATURE-MODEL-REFS
    FeatureModelRefs                                                       = 5379,
    /// FEATURE-REF
    FeatureRef                                                             = 1707,
    /// FEATURE-REFS
    FeatureRefs                                                            = 3443,
    /// FIBEX-ELEMENT-REF
    FibexElementRef                                                        = 5119,
    /// FIBEX-ELEMENT-REF-CONDITIONAL
    FibexElementRefConditional                                             = 2377,
    /// FIBEX-ELEMENTS
    FibexElements                                                          = 5942,
    /// FIELD
    Field                                                                  = 6085,
    /// FIELD-DEPLOYMENTS
    FieldDeployments                                                       = 5005,
    /// FIELD-ELEMENTS
    FieldElements                                                          = 5179,
    /// FIELD-GET-SET-QOS-PROPSS
    FieldGetSetQosPropss                                                   = 4290,
    /// FIELD-IREF
    FieldIref                                                              = 1740,
    /// FIELD-MAPPING
    FieldMapping                                                           = 6211,
    /// FIELD-MAPPINGS
    FieldMappings                                                          = 6230,
    /// FIELD-NOTIFIER-QOS-PROPSS
    FieldNotifierQosPropss                                                 = 3663,
    /// FIELD-NOTIFIER-REF
    FieldNotifierRef                                                       = 3049,
    /// FIELD-REF
    FieldRef                                                               = 4993,
    /// FIELD-REFS
    FieldRefs                                                              = 2791,
    /// FIELD-REPLY-TOPIC-NAME
    FieldReplyTopicName                                                    = 4,
    /// FIELD-REQUEST-TOPIC-NAME
    FieldRequestTopicName                                                  = 2601,
    /// FIELD-SENDER-COM-SPEC
    FieldSenderComSpec                                                     = 6324,
    /// FIELD-TOPICS-ACCESS-RULE-REF
    FieldTopicsAccessRuleRef                                               = 6542,
    /// FIELDS
    Fields                                                                 = 3490,
    /// FIFO-DEPTH
    FifoDepth                                                              = 252,
    /// FIFO-RANGES
    FifoRanges                                                             = 534,
    /// FIGURE
    Figure                                                                 = 3704,
    /// FIGURE-CAPTION
    FigureCaption                                                          = 2423,
    /// FILE-ELEMENTS
    FileElements                                                           = 2545,
    /// FILE-INFO-COMMENT
    FileInfoComment                                                        = 2828,
    /// FILE-NAME
    FileName                                                               = 6544,
    /// FILE-PROXYS
    FileProxys                                                             = 3553,
    /// FILE-REF
    FileRef                                                                = 5343,
    /// FILE-REFS
    FileRefs                                                               = 5377,
    /// FILE-STORAGE-REF
    FileStorageRef                                                         = 632,
    /// FILE-TYPE
    FileType                                                               = 888,
    /// FILE-TYPE-PATTERN
    FileTypePattern                                                        = 2084,
    /// FILES
    Files                                                                  = 2931,
    /// FILTER
    Filter                                                                 = 5540,
    /// FILTER-ACTION-BLOCK-SOURCE
    FilterActionBlockSource                                                = 1164,
    /// FILTER-ACTION-DEST-PORT-MODIFICATION
    FilterActionDestPortModification                                       = 5136,
    /// FILTER-ACTION-DROP-FRAME
    FilterActionDropFrame                                                  = 923,
    /// FILTER-ACTION-VLAN-MODIFICATION
    FilterActionVlanModification                                           = 1448,
    /// FILTER-CHAINS
    FilterChains                                                           = 950,
    /// FILTER-PRIORITY
    FilterPriority                                                         = 1026,
    /// FILTERING-DEBOUNCING
    FilteringDebouncing                                                    = 864,
    /// FINAL-CHECKPOINT-REF
    FinalCheckpointRef                                                     = 2101,
    /// FINAL-CHECKPOINT-REFS
    FinalCheckpointRefs                                                    = 6156,
    /// FIRE-AND-FORGET
    FireAndForget                                                          = 1896,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 5541,
    /// FIRE-AND-FORGET-MAPPINGS
    FireAndForgetMappings                                                  = 4731,
    /// FIRE-AND-FORGET-METHOD-MAPPING
    FireAndForgetMethodMapping                                             = 810,
    /// FIRE-AND-FORGET-METHOD-MAPPINGS
    FireAndForgetMethodMappings                                            = 5803,
    /// FIREWALL-REF
    FirewallRef                                                            = 2144,
    /// FIREWALL-RULE
    FirewallRule                                                           = 81,
    /// FIREWALL-RULE-PROPS
    FirewallRuleProps                                                      = 4869,
    /// FIREWALL-RULE-PROPSS
    FirewallRulePropss                                                     = 3354,
    /// FIREWALL-RULE-REF
    FirewallRuleRef                                                        = 5751,
    /// FIREWALL-RULE-REFS
    FirewallRuleRefs                                                       = 1242,
    /// FIREWALL-STATE-IREF
    FirewallStateIref                                                      = 4668,
    /// FIREWALL-STATE-IREFS
    FirewallStateIrefs                                                     = 2156,
    /// FIREWALL-STATE-MACHINES
    FirewallStateMachines                                                  = 5993,
    /// FIREWALL-STATE-MODE-DECLARATION-REF
    FirewallStateModeDeclarationRef                                        = 2712,
    /// FIREWALL-STATE-MODE-DECLARATION-REFS
    FirewallStateModeDeclarationRefs                                       = 798,
    /// FIREWALL-STATE-SWITCH-INTERFACE
    FirewallStateSwitchInterface                                           = 2484,
    /// FIRST-APPLICATION-ERROR-REF
    FirstApplicationErrorRef                                               = 4038,
    /// FIRST-DATA-PROTOTYPE-REF
    FirstDataPrototypeRef                                                  = 2942,
    /// FIRST-ELEMENT-INSTANCE-IREF
    FirstElementInstanceIref                                               = 4069,
    /// FIRST-ELEMENT-INSTANCE-IREFS
    FirstElementInstanceIrefs                                              = 2192,
    /// FIRST-ELEMENT-REF
    FirstElementRef                                                        = 3339,
    /// FIRST-ELEMENT-REFS
    FirstElementRefs                                                       = 3491,
    /// FIRST-ELEMENTS
    FirstElements                                                          = 4503,
    /// FIRST-MODE-GROUP-REF
    FirstModeGroupRef                                                      = 5785,
    /// FIRST-MODE-REF
    FirstModeRef                                                           = 3726,
    /// FIRST-MODE-REFS
    FirstModeRefs                                                          = 87,
    /// FIRST-OPERATION-REF
    FirstOperationRef                                                      = 422,
    /// FIRST-PHYSICAL-DIMENSION-REF
    FirstPhysicalDimensionRef                                              = 6149,
    /// FIRST-PORT-REF
    FirstPortRef                                                           = 167,
    /// FIRST-TIMEOUT
    FirstTimeout                                                           = 3534,
    /// FIRST-TO-SECOND-DATA-TRANSFORMATION-REF
    FirstToSecondDataTransformationRef                                     = 2716,
    /// FIRST-TRIGGER-REF
    FirstTriggerRef                                                        = 150,
    /// FIRST-VALUE
    FirstValue                                                             = 4606,
    /// FIXED-LENGTH
    FixedLength                                                            = 1318,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 6017,
    /// FLAT-INSTANCE-REF
    FlatInstanceRef                                                        = 1773,
    /// FLAT-MAP
    FlatMap                                                                = 4823,
    /// FLAT-MAP-ENTRY-REF
    FlatMapEntryRef                                                        = 1624,
    /// FLAT-MAP-ENTRY-REFS
    FlatMapEntryRefs                                                       = 6107,
    /// FLAT-MAP-REF
    FlatMapRef                                                             = 318,
    /// FLAT-OBJECT-REF
    FlatObjectRef                                                          = 721,
    /// FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING
    FlexrayAbsolutelyScheduledTiming                                       = 2721,
    /// FLEXRAY-AR-TP-CHANNEL
    FlexrayArTpChannel                                                     = 1905,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 4489,
    /// FLEXRAY-AR-TP-CONNECTION
    FlexrayArTpConnection                                                  = 6535,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 6534,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 197,
    /// FLEXRAY-CLUSTER-CONDITIONAL
    FlexrayClusterConditional                                              = 1426,
    /// FLEXRAY-CLUSTER-VARIANTS
    FlexrayClusterVariants                                                 = 1720,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 6175,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 1554,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL
    FlexrayCommunicationControllerConditional                              = 935,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS
    FlexrayCommunicationControllerVariants                                 = 4747,
    /// FLEXRAY-FIFO-CONFIGURATION
    FlexrayFifoConfiguration                                               = 2535,
    /// FLEXRAY-FIFO-RANGE
    FlexrayFifoRange                                                       = 5737,
    /// FLEXRAY-FIFOS
    FlexrayFifos                                                           = 2745,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 3840,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 3871,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 4261,
    /// FLEXRAY-NM-CLUSTER-COUPLING
    FlexrayNmClusterCoupling                                               = 278,
    /// FLEXRAY-NM-ECU
    FlexrayNmEcu                                                           = 5562,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 2480,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 5205,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 2315,
    /// FLEXRAY-TP-CONNECTION
    FlexrayTpConnection                                                    = 2150,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 5862,
    /// FLEXRAY-TP-ECU
    FlexrayTpEcu                                                           = 1282,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 66,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 1875,
    /// FLOAT
    Float                                                                  = 5670,
    /// FLOAT-VALUE-VARIATION-POINT
    FloatValueVariationPoint                                               = 5909,
    /// FLOW-CONTROL-PDU-REF
    FlowControlPduRef                                                      = 3206,
    /// FLOW-CONTROL-PDU-REFS
    FlowControlPduRefs                                                     = 2686,
    /// FLOW-CONTROL-REF
    FlowControlRef                                                         = 1179,
    /// FLOW-LABEL
    FlowLabel                                                              = 1109,
    /// FLOW-METERING-REF
    FlowMeteringRef                                                        = 242,
    /// FLOW-METERINGS
    FlowMeterings                                                          = 4898,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 186,
    /// FM-ATTRIBUTE-VALUE
    FmAttributeValue                                                       = 2061,
    /// FM-COND
    FmCond                                                                 = 4855,
    /// FM-FEATURE
    FmFeature                                                              = 1213,
    /// FM-FEATURE-DECOMPOSITION
    FmFeatureDecomposition                                                 = 4843,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 5528,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 5656,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 3019,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 4013,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 4210,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 3885,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 2981,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 2783,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 3195,
    /// FM-SYSCOND
    FmSyscond                                                              = 337,
    /// FMI
    Fmi                                                                    = 1113,
    /// FOLLOW-UP-ACTION-REF
    FollowUpActionRef                                                      = 876,
    /// FOLLOW-UP-ACTION-REFS
    FollowUpActionRefs                                                     = 3064,
    /// FOLLOW-UP-OFFSET
    FollowUpOffset                                                         = 4486,
    /// FOLLOW-UP-TIMEOUT-VALUE
    FollowUpTimeoutValue                                                   = 5181,
    /// FORBIDDEN-SIGNAL-PATH
    ForbiddenSignalPath                                                    = 2756,
    /// FOREIGN-MODEL-REFERENCE
    ForeignModelReference                                                  = 4058,
    /// FORMAL-BLUEPRINT-CONDITION
    FormalBlueprintCondition                                               = 334,
    /// FORMAL-BLUEPRINT-GENERATOR
    FormalBlueprintGenerator                                               = 5742,
    /// FORMAT
    Format                                                                 = 483,
    /// FORMULA
    Formula                                                                = 6493,
    /// FORMULA-CAPTION
    FormulaCaption                                                         = 3904,
    /// FR-CLUSTER-REF
    FrClusterRef                                                           = 734,
    /// FR-GLOBAL-TIME-DOMAIN-PROPS
    FrGlobalTimeDomainProps                                                = 6008,
    /// FRAGMENT
    Fragment                                                               = 5705,
    /// FRAGMENTATION-PROPS
    FragmentationProps                                                     = 3482,
    /// FRAME-LENGTH
    FrameLength                                                            = 1790,
    /// FRAME-MAPPING
    FrameMapping                                                           = 143,
    /// FRAME-MAPPINGS
    FrameMappings                                                          = 4343,
    /// FRAME-PID
    FramePid                                                               = 1341,
    /// FRAME-PIDS
    FramePids                                                              = 526,
    /// FRAME-PORT
    FramePort                                                              = 1489,
    /// FRAME-PORT-REF
    FramePortRef                                                           = 6285,
    /// FRAME-PORT-REFS
    FramePortRefs                                                          = 2969,
    /// FRAME-PREEMPTION-SUPPORT
    FramePreemptionSupport                                                 = 4004,
    /// FRAME-REF
    FrameRef                                                               = 6306,
    /// FRAME-SYNC-ENABLED
    FrameSyncEnabled                                                       = 4464,
    /// FRAME-TRIGGERING-REF
    FrameTriggeringRef                                                     = 1576,
    /// FRAME-TRIGGERINGS
    FrameTriggerings                                                       = 6648,
    /// FREE-FORMAT
    FreeFormat                                                             = 1199,
    /// FREEZE-CURRENT-STATE
    FreezeCurrentState                                                     = 4507,
    /// FREEZE-CURRENT-STATE-SUPPORTED
    FreezeCurrentStateSupported                                            = 90,
    /// FREEZE-FRAME-CONTENT-REF
    FreezeFrameContentRef                                                  = 5842,
    /// FREEZE-FRAME-CONTENT-WWH-OBD-REF
    FreezeFrameContentWwhObdRef                                            = 792,
    /// FREEZE-FRAME-REF
    FreezeFrameRef                                                         = 2340,
    /// FREEZE-FRAMES
    FreezeFrames                                                           = 5569,
    /// FRESHNESS-COUNTER-SYNC-ATTEMPTS
    FreshnessCounterSyncAttempts                                           = 3639,
    /// FRESHNESS-PROPS-REF
    FreshnessPropsRef                                                      = 5013,
    /// FRESHNESS-PROPSS
    FreshnessPropss                                                        = 728,
    /// FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR
    FreshnessTimestampTimePeriodFactor                                     = 788,
    /// FRESHNESS-VALUE-ID
    FreshnessValueId                                                       = 1915,
    /// FRESHNESS-VALUE-LENGTH
    FreshnessValueLength                                                   = 3554,
    /// FRESHNESS-VALUE-TX-LENGTH
    FreshnessValueTxLength                                                 = 1184,
    /// FROM-REF
    FromRef                                                                = 3846,
    /// FT
    Ft                                                                     = 4240,
    /// FULL-DUPLEX-ENABLED
    FullDuplexEnabled                                                      = 6371,
    /// FULLY-QUALIFIED-DOMAIN-NAME
    FullyQualifiedDomainName                                               = 4595,
    /// FUNCTION
    Function                                                               = 3326,
    /// FUNCTION-CLUSTER-AFFILIATION
    FunctionClusterAffiliation                                             = 3083,
    /// FUNCTION-GROUP-ERROR-MAPPING
    FunctionGroupErrorMapping                                              = 4724,
    /// FUNCTION-GROUP-ERROR-MAPPINGS
    FunctionGroupErrorMappings                                             = 216,
    /// FUNCTION-GROUP-IREF
    FunctionGroupIref                                                      = 4120,
    /// FUNCTION-GROUP-IREFS
    FunctionGroupIrefs                                                     = 832,
    /// FUNCTION-GROUP-MODE-IREF
    FunctionGroupModeIref                                                  = 4137,
    /// FUNCTION-GROUP-MODE-IREFS
    FunctionGroupModeIrefs                                                 = 6444,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 2236,
    /// FUNCTION-GROUP-PHM-STATE-REFERENCE
    FunctionGroupPhmStateReference                                         = 6499,
    /// FUNCTION-GROUP-PORT-MAPPING
    FunctionGroupPortMapping                                               = 5511,
    /// FUNCTION-GROUP-REF
    FunctionGroupRef                                                       = 4016,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 1844,
    /// FUNCTION-GROUP-STATE-IREF
    FunctionGroupStateIref                                                 = 2025,
    /// FUNCTION-GROUP-STATE-IREFS
    FunctionGroupStateIrefs                                                = 4840,
    /// FUNCTION-GROUP-TRANSITION-REQUEST-FAILED-ERROR-REF
    FunctionGroupTransitionRequestFailedErrorRef                           = 617,
    /// FUNCTION-GROUP-TYPE-REF
    FunctionGroupTypeRef                                                   = 1011,
    /// FUNCTION-GROUPS
    FunctionGroups                                                         = 4212,
    /// FUNCTION-ID
    FunctionId                                                             = 2912,
    /// FUNCTION-IDENTIFIER-REF
    FunctionIdentifierRef                                                  = 4377,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 973,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 814,
    /// FUNCTION-INSTANCE
    FunctionInstance                                                       = 3674,
    /// FUNCTION-NAME
    FunctionName                                                           = 2867,
    /// FUNCTION-NAMES
    FunctionNames                                                          = 2608,
    /// FUNCTION-POINTER-SIGNATURE-REF
    FunctionPointerSignatureRef                                            = 2734,
    /// FUNCTION-PROTOTYPE-EMITTER
    FunctionPrototypeEmitter                                               = 6172,
    /// FUNCTIONAL-CLUSTER-DESIGNS
    FunctionalClusterDesigns                                               = 2910,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-DIAGNOSTIC-EVENT-MAPPING
    FunctionalClusterInteractsWithDiagnosticEventMapping                   = 1171,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING
    FunctionalClusterInteractsWithPersistencyDeploymentMapping             = 2768,
    /// FUNCTIONAL-CLUSTER-REF
    FunctionalClusterRef                                                   = 2547,
    /// FUNCTIONAL-CLUSTER-REFS
    FunctionalClusterRefs                                                  = 539,
    /// FUNCTIONAL-CLUSTER-TO-DLT-LOG-SINK-MAPPING
    FunctionalClusterToDltLogSinkMapping                                   = 3712,
    /// FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING
    FunctionalClusterToSecurityEventDefinitionMapping                      = 3655,
    /// FUNCTIONAL-REQUEST-REF
    FunctionalRequestRef                                                   = 4270,
    /// FUNCTIONAL-REQUEST-REFS
    FunctionalRequestRefs                                                  = 1728,
    /// FUNCTIONAL-UNIT
    FunctionalUnit                                                         = 3359,
    /// FUP-DATA-ID-LIST
    FupDataIdList                                                          = 244,
    /// FUP-DATA-ID-LISTS
    FupDataIdLists                                                         = 4159,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 6319,
    /// GATEWAY
    Gateway                                                                = 171,
    /// GATEWAYS
    Gateways                                                               = 3888,
    /// GENERAL-INACTIVITY-TIME
    GeneralInactivityTime                                                  = 670,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 4126,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 3441,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 3254,
    /// GENERAL-PURPOSE-TIMER-SERVICE-NEEDS
    GeneralPurposeTimerServiceNeeds                                        = 132,
    /// GENERATED-ARTIFACTS
    GeneratedArtifacts                                                     = 6459,
    /// GENERATOR
    Generator                                                              = 3498,
    /// GENERIC-DIAGNOSTIC-TRANSPORT-INSTANTIATION
    GenericDiagnosticTransportInstantiation                                = 4505,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 6547,
    /// GENERIC-MATH
    GenericMath                                                            = 5140,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 6028,
    /// GENERIC-TP
    GenericTp                                                              = 4821,
    /// GENERIC-TP-CONNECTION
    GenericTpConnection                                                    = 1008,
    /// GET
    Get                                                                    = 4943,
    /// GETS
    Gets                                                                   = 5918,
    /// GETTER-CALL-REF
    GetterCallRef                                                          = 370,
    /// GETTER-CALL-SIGNAL-REF
    GetterCallSignalRef                                                    = 5653,
    /// GETTER-OPERATION-REF
    GetterOperationRef                                                     = 1734,
    /// GETTER-REF
    GetterRef                                                              = 22,
    /// GETTER-RETURN-REF
    GetterReturnRef                                                        = 1093,
    /// GETTER-RETURN-SIGNAL-REF
    GetterReturnSignalRef                                                  = 3589,
    /// GID
    Gid                                                                    = 3037,
    /// GID-INVALIDITY-PATTERN
    GidInvalidityPattern                                                   = 124,
    /// GLOBAL-ELEMENT
    GlobalElement                                                          = 657,
    /// GLOBAL-ELEMENTS
    GlobalElements                                                         = 5692,
    /// GLOBAL-IN-PACKAGE-REF
    GlobalInPackageRef                                                     = 2381,
    /// GLOBAL-IN-PACKAGE-REFS
    GlobalInPackageRefs                                                    = 4329,
    /// GLOBAL-KAY-PROPS-REF
    GlobalKayPropsRef                                                      = 3023,
    /// GLOBAL-RESOURCE-ID
    GlobalResourceId                                                       = 2803,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 2412,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 5089,
    /// GLOBAL-SUPERVISION-ENTITYS
    GlobalSupervisionEntitys                                               = 3114,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 5411,
    /// GLOBAL-SUPERVISIONS
    GlobalSupervisions                                                     = 2048,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 6520,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 1528,
    /// GLOBAL-TIME-CORRECTION-PROPS
    GlobalTimeCorrectionProps                                              = 2717,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 4441,
    /// GLOBAL-TIME-DOMAIN-PROPERTYS
    GlobalTimeDomainPropertys                                              = 6469,
    /// GLOBAL-TIME-DOMAIN-PROPS
    GlobalTimeDomainProps                                                  = 2790,
    /// GLOBAL-TIME-DOMAIN-REF
    GlobalTimeDomainRef                                                    = 3572,
    /// GLOBAL-TIME-DOMAIN-REF-CONDITIONAL
    GlobalTimeDomainRefConditional                                         = 3050,
    /// GLOBAL-TIME-DOMAINS
    GlobalTimeDomains                                                      = 2857,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 7,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 3583,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 2844,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 1269,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 6164,
    /// GLOBAL-TIME-MASTERS
    GlobalTimeMasters                                                      = 593,
    /// GLOBAL-TIME-PDU-REF
    GlobalTimePduRef                                                       = 6209,
    /// GLOBAL-TIME-PDU-TRIGGERING-REF
    GlobalTimePduTriggeringRef                                             = 4092,
    /// GLOBAL-TIME-PORT-ROLE
    GlobalTimePortRole                                                     = 4274,
    /// GLOBAL-TIME-PROPS
    GlobalTimeProps                                                        = 1661,
    /// GLOBAL-TIME-SUB-DOMAINS
    GlobalTimeSubDomains                                                   = 3975,
    /// GLOBAL-TIME-TX-PERIOD
    GlobalTimeTxPeriod                                                     = 785,
    /// GOVERNANCE-REF
    GovernanceRef                                                          = 552,
    /// GRANT-REF
    GrantRef                                                               = 4378,
    /// GRANT-REFS
    GrantRefs                                                              = 1126,
    /// GRAPHIC
    Graphic                                                                = 5299,
    /// GROUP-ID
    GroupId                                                                = 6127,
    /// GROUP-IDENTIFIER
    GroupIdentifier                                                        = 4310,
    /// GROUP-NUMBER
    GroupNumber                                                            = 4424,
    /// GROUP-REF
    GroupRef                                                               = 5630,
    /// GROUP-REFS
    GroupRefs                                                              = 79,
    /// GROUPED-ALIAS-EVENT-REF
    GroupedAliasEventRef                                                   = 2283,
    /// GROUPED-ALIAS-EVENT-REFS
    GroupedAliasEventRefs                                                  = 6552,
    /// HANDLE-DATA-STATUS
    HandleDataStatus                                                       = 5584,
    /// HANDLE-INVALID
    HandleInvalid                                                          = 987,
    /// HANDLE-NEVER-RECEIVED
    HandleNeverReceived                                                    = 4709,
    /// HANDLE-OUT-OF-RANGE
    HandleOutOfRange                                                       = 5575,
    /// HANDLE-OUT-OF-RANGE-STATUS
    HandleOutOfRangeStatus                                                 = 6368,
    /// HANDLE-TERMINATION-AND-RESTART
    HandleTerminationAndRestart                                            = 1328,
    /// HANDLE-TIMEOUT-TYPE
    HandleTimeoutType                                                      = 2265,
    /// HARDWARE-CONFIGURATION
    HardwareConfiguration                                                  = 5026,
    /// HARDWARE-ELEMENT-REF
    HardwareElementRef                                                     = 373,
    /// HARDWARE-ELEMENT-REFS
    HardwareElementRefs                                                    = 2719,
    /// HARDWARE-PLATFORM
    HardwarePlatform                                                       = 2470,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 3780,
    /// HAS-DYNAMIC-LENGTH
    HasDynamicLength                                                       = 1837,
    /// HAS-GETTER
    HasGetter                                                              = 1531,
    /// HAS-INTERNAL-STATE
    HasInternalState                                                       = 2271,
    /// HAS-NOTIFIER
    HasNotifier                                                            = 1313,
    /// HAS-SETTER
    HasSetter                                                              = 2840,
    /// HEADER-FILE
    HeaderFile                                                             = 5597,
    /// HEADER-ID
    HeaderId                                                               = 3204,
    /// HEADER-ID-LONG-HEADER
    HeaderIdLongHeader                                                     = 1010,
    /// HEADER-ID-SHORT-HEADER
    HeaderIdShortHeader                                                    = 3122,
    /// HEADER-LENGTH
    HeaderLength                                                           = 5483,
    /// HEADER-TYPE
    HeaderType                                                             = 2861,
    /// HEALING-CYCLE-COUNTER-THRESHOLD
    HealingCycleCounterThreshold                                           = 3672,
    /// HEALING-CYCLE-REF
    HealingCycleRef                                                        = 1687,
    /// HEALTH-CHANNEL-ARGUMENT-REF
    HealthChannelArgumentRef                                               = 561,
    /// HEALTH-CHANNEL-ARGUMENT-REFS
    HealthChannelArgumentRefs                                              = 4575,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 6075,
    /// HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS
    HealthChannelExternalReportedStatus                                    = 2097,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 4842,
    /// HEALTH-CHANNEL-ID
    HealthChannelId                                                        = 4501,
    /// HEALTH-CHANNEL-IREF
    HealthChannelIref                                                      = 5792,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 5886,
    /// HEALTH-CHANNELS
    HealthChannels                                                         = 576,
    /// HEAP-USAGES
    HeapUsages                                                             = 787,
    /// HISTORY
    History                                                                = 4121,
    /// HISTORY-KIND
    HistoryKind                                                            = 2178,
    /// HISTORY-ORDER-DEPTH
    HistoryOrderDepth                                                      = 1137,
    /// HOLD-OVER-TIME
    HoldOverTime                                                           = 3769,
    /// HOP-COUNT
    HopCount                                                               = 2240,
    /// HOP-LIMIT
    HopLimit                                                               = 4658,
    /// HOST-ID
    HostId                                                                 = 1281,
    /// HOST-REF
    HostRef                                                                = 37,
    /// HOST-SYSTEM-REF
    HostSystemRef                                                          = 3656,
    /// HTTP-ACCEPT-ENCODING
    HttpAcceptEncoding                                                     = 3154,
    /// HTTP-TP
    HttpTp                                                                 = 4134,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 1852,
    /// HW-ATTRIBUTE-DEF-REF
    HwAttributeDefRef                                                      = 3096,
    /// HW-ATTRIBUTE-DEFS
    HwAttributeDefs                                                        = 2612,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 775,
    /// HW-ATTRIBUTE-LITERALS
    HwAttributeLiterals                                                    = 2926,
    /// HW-ATTRIBUTE-VALUE
    HwAttributeValue                                                       = 5435,
    /// HW-ATTRIBUTE-VALUES
    HwAttributeValues                                                      = 1174,
    /// HW-CATEGORY
    HwCategory                                                             = 4526,
    /// HW-CATEGORY-REF
    HwCategoryRef                                                          = 2785,
    /// HW-CATEGORY-REFS
    HwCategoryRefs                                                         = 1053,
    /// HW-COMMUNICATION-CONTROLLER-REF
    HwCommunicationControllerRef                                           = 2939,
    /// HW-COMMUNICATION-PORT-REF
    HwCommunicationPortRef                                                 = 1897,
    /// HW-ELEMENT
    HwElement                                                              = 3484,
    /// HW-ELEMENT-CONNECTIONS
    HwElementConnections                                                   = 5092,
    /// HW-ELEMENT-CONNECTOR
    HwElementConnector                                                     = 2492,
    /// HW-ELEMENT-REF
    HwElementRef                                                           = 6378,
    /// HW-ELEMENT-REF-CONDITIONAL
    HwElementRefConditional                                                = 6135,
    /// HW-ELEMENT-REFS
    HwElementRefs                                                          = 5957,
    /// HW-PIN
    HwPin                                                                  = 5767,
    /// HW-PIN-CONNECTIONS
    HwPinConnections                                                       = 6509,
    /// HW-PIN-CONNECTOR
    HwPinConnector                                                         = 1071,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 4723,
    /// HW-PIN-GROUP-CONNECTIONS
    HwPinGroupConnections                                                  = 6201,
    /// HW-PIN-GROUP-CONNECTOR
    HwPinGroupConnector                                                    = 2252,
    /// HW-PIN-GROUP-CONTENT
    HwPinGroupContent                                                      = 4379,
    /// HW-PIN-GROUP-REF
    HwPinGroupRef                                                          = 1484,
    /// HW-PIN-GROUP-REFS
    HwPinGroupRefs                                                         = 1914,
    /// HW-PIN-GROUPS
    HwPinGroups                                                            = 2526,
    /// HW-PIN-REF
    HwPinRef                                                               = 4078,
    /// HW-PIN-REFS
    HwPinRefs                                                              = 4003,
    /// HW-PORT-MAPPING
    HwPortMapping                                                          = 6100,
    /// HW-PORT-MAPPINGS
    HwPortMappings                                                         = 177,
    /// HW-TYPE
    HwType                                                                 = 1377,
    /// HW-TYPE-REF
    HwTypeRef                                                              = 6120,
    /// I-PDU-IDENTIFIER-TCP-REF
    IPduIdentifierTcpRef                                                   = 4777,
    /// I-PDU-IDENTIFIER-TCP-REFS
    IPduIdentifierTcpRefs                                                  = 4875,
    /// I-PDU-IDENTIFIER-UDP-REF
    IPduIdentifierUdpRef                                                   = 1668,
    /// I-PDU-IDENTIFIER-UDP-REFS
    IPduIdentifierUdpRefs                                                  = 6150,
    /// I-PDU-IDENTIFIERS
    IPduIdentifiers                                                        = 5314,
    /// I-PDU-MAPPING
    IPduMapping                                                            = 2142,
    /// I-PDU-MAPPINGS
    IPduMappings                                                           = 5629,
    /// I-PDU-PORT
    IPduPort                                                               = 5539,
    /// I-PDU-PORT-REF
    IPduPortRef                                                            = 4999,
    /// I-PDU-PORT-REFS
    IPduPortRefs                                                           = 5530,
    /// I-PDU-REF
    IPduRef                                                                = 773,
    /// I-PDU-SIGNAL-PROCESSING
    IPduSignalProcessing                                                   = 4458,
    /// I-PDU-TIMING
    IPduTiming                                                             = 2598,
    /// I-PDU-TIMING-SPECIFICATIONS
    IPduTimingSpecifications                                               = 5110,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 5338,
    /// I-PDU-TRIGGERING-REF
    IPduTriggeringRef                                                      = 2986,
    /// I-PDU-TRIGGERING-REF-CONDITIONAL
    IPduTriggeringRefConditional                                           = 2795,
    /// I-PDU-TRIGGERINGS
    IPduTriggerings                                                        = 172,
    /// I-PDUS
    IPdus                                                                  = 4151,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 903,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 2959,
    /// I-SIGNAL
    ISignal                                                                = 2872,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 2120,
    /// I-SIGNAL-GROUP-REF
    ISignalGroupRef                                                        = 272,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 3475,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 5064,
    /// I-SIGNAL-I-PDU-REF
    ISignalIPduRef                                                         = 1366,
    /// I-SIGNAL-I-PDU-REF-CONDITIONAL
    ISignalIPduRefConditional                                              = 755,
    /// I-SIGNAL-I-PDUS
    ISignalIPdus                                                           = 6611,
    /// I-SIGNAL-IN-I-PDU-REF
    ISignalInIPduRef                                                       = 3926,
    /// I-SIGNAL-MAPPING
    ISignalMapping                                                         = 5230,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 518,
    /// I-SIGNAL-PORT-REF
    ISignalPortRef                                                         = 5204,
    /// I-SIGNAL-PORT-REFS
    ISignalPortRefs                                                        = 4584,
    /// I-SIGNAL-PORT-TO-DIAGNOSTIC-EVENT-MAPPING
    ISignalPortToDiagnosticEventMapping                                    = 3205,
    /// I-SIGNAL-PROPS
    ISignalProps                                                           = 2205,
    /// I-SIGNAL-REF
    ISignalRef                                                             = 6396,
    /// I-SIGNAL-REFS
    ISignalRefs                                                            = 5055,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 5081,
    /// I-SIGNAL-TO-I-PDU-MAPPINGS
    ISignalToIPduMappings                                                  = 61,
    /// I-SIGNAL-TO-PDU-MAPPINGS
    ISignalToPduMappings                                                   = 3310,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 2509,
    /// I-SIGNAL-TRIGGERING-REF
    ISignalTriggeringRef                                                   = 3942,
    /// I-SIGNAL-TRIGGERING-REF-CONDITIONAL
    ISignalTriggeringRefConditional                                        = 4387,
    /// I-SIGNAL-TRIGGERINGS
    ISignalTriggerings                                                     = 618,
    /// I-SIGNAL-TYPE
    ISignalType                                                            = 1034,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 2869,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF
    IamRelevantTlsSecureComPropsRef                                        = 794,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS
    IamRelevantTlsSecureComPropsRefs                                       = 851,
    /// ICMP-RULE
    IcmpRule                                                               = 2658,
    /// ICMP-V-4-PROPS
    IcmpV4Props                                                            = 243,
    /// ICMP-V-6-PROPS
    IcmpV6Props                                                            = 5212,
    /// ICV-FRESHNESS-VALUE-ID
    IcvFreshnessValueId                                                    = 1771,
    /// ICV-SECURE-COM-PROPSS
    IcvSecureComPropss                                                     = 224,
    /// ICV-SECURED
    IcvSecured                                                             = 203,
    /// ICV-VERIFICATION
    IcvVerification                                                        = 751,
    /// ID
    Id                                                                     = 513,
    /// IDENT
    Ident                                                                  = 2556,
    /// IDENTICAL-MAPPING
    IdenticalMapping                                                       = 4639,
    /// IDENTIFIABLE-REF
    IdentifiableRef                                                        = 6157,
    /// IDENTIFIER
    Identifier                                                             = 5286,
    /// IDENTITIY-NUMBER
    IdentitiyNumber                                                        = 6445,
    /// IDENTITY
    Identity                                                               = 2145,
    /// IDENTITY-CERTIFICATE-AUTHORITY-REF
    IdentityCertificateAuthorityRef                                        = 6347,
    /// IDENTITY-REF
    IdentityRef                                                            = 4926,
    /// IDLE-SLOPE
    IdleSlope                                                              = 1657,
    /// IDS-COMMON-ELEMENT-REF
    IdsCommonElementRef                                                    = 1518,
    /// IDS-COMMON-ELEMENT-REF-CONDITIONAL
    IdsCommonElementRefConditional                                         = 3324,
    /// IDS-DESIGN
    IdsDesign                                                              = 3388,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 4455,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 1196,
    /// IDS-PLATFORM-INSTANTIATION-REF
    IdsPlatformInstantiationRef                                            = 284,
    /// IDS-TIME-BASE-REF
    IdsTimeBaseRef                                                         = 1819,
    /// IDS-TIME-BASE-REFS
    IdsTimeBaseRefs                                                        = 6272,
    /// IDSM-CONTEXT-PROVIDER-INTERFACE
    IdsmContextProviderInterface                                           = 3068,
    /// IDSM-CONTEXT-PROVIDER-MAPPING
    IdsmContextProviderMapping                                             = 6043,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 5905,
    /// IDSM-INSTANCE-ID
    IdsmInstanceId                                                         = 6055,
    /// IDSM-INSTANCE-REF
    IdsmInstanceRef                                                        = 892,
    /// IDSM-INSTANCE-REF-CONDITIONAL
    IdsmInstanceRefConditional                                             = 3800,
    /// IDSM-INSTANCES
    IdsmInstances                                                          = 3295,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 5016,
    /// IDSM-MODULE-INSTANTIATION-REF
    IdsmModuleInstantiationRef                                             = 2298,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 5188,
    /// IDSM-QUALIFIED-EVENT-RECEIVER-INTERFACE
    IdsmQualifiedEventReceiverInterface                                    = 6387,
    /// IDSM-QUALIFIED-EVENT-RECEIVER-MAPPING
    IdsmQualifiedEventReceiverMapping                                      = 467,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 2464,
    /// IDSM-RATE-LIMITATION-REF
    IdsmRateLimitationRef                                                  = 4029,
    /// IDSM-RATE-LIMITATION-REF-CONDITIONAL
    IdsmRateLimitationRefConditional                                       = 1511,
    /// IDSM-REPORTING-MODE-PROVIDER-INTERFACE
    IdsmReportingModeProviderInterface                                     = 6329,
    /// IDSM-REPORTING-MODE-PROVIDER-MAPPING
    IdsmReportingModeProviderMapping                                       = 5069,
    /// IDSM-TIMESTAMP-PROVIDER-INTERFACE
    IdsmTimestampProviderInterface                                         = 1300,
    /// IDSM-TIMESTAMP-PROVIDER-MAPPING
    IdsmTimestampProviderMapping                                           = 861,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 5832,
    /// IDSM-TRAFFIC-LIMITATION-REF
    IdsmTrafficLimitationRef                                               = 2752,
    /// IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL
    IdsmTrafficLimitationRefConditional                                    = 88,
    /// IE
    Ie                                                                     = 1866,
    /// IEE-E-1722-TP-CONNECTION-REF
    IeeE1722TpConnectionRef                                                = 6019,
    /// IEEE-1722-ACF-BUS-PART-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722AcfBusPartRawDataStreamConsumerMapping                         = 102,
    /// IEEE-1722-ACF-BUS-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722AcfBusRawDataStreamConsumerMapping                             = 6353,
    /// IEEE-1722-RAW-DATA-STREAM-CONSUMER-INTERFACE
    Ieee1722RawDataStreamConsumerInterface                                 = 2357,
    /// IEEE-1722-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722RawDataStreamConsumerMapping                                   = 6291,
    /// IEEE-1722-RAW-DATA-STREAM-PRODUCER-INTERFACE
    Ieee1722RawDataStreamProducerInterface                                 = 5238,
    /// IEEE-1722-RAW-DATA-STREAM-PRODUCER-MAPPING
    Ieee1722RawDataStreamProducerMapping                                   = 2527,
    /// IEEE-1722-STREAM-REF
    Ieee1722StreamRef                                                      = 2002,
    /// IEEE-1722-TP
    Ieee1722Tp                                                             = 3156,
    /// IEEE-1722-TP-AAF-CONNECTION
    Ieee1722TpAafConnection                                                = 170,
    /// IEEE-1722-TP-ACF-CAN
    Ieee1722TpAcfCan                                                       = 5223,
    /// IEEE-1722-TP-ACF-CAN-PART
    Ieee1722TpAcfCanPart                                                   = 344,
    /// IEEE-1722-TP-ACF-CONNECTION
    Ieee1722TpAcfConnection                                                = 1549,
    /// IEEE-1722-TP-ACF-LIN
    Ieee1722TpAcfLin                                                       = 3952,
    /// IEEE-1722-TP-ACF-LIN-PART
    Ieee1722TpAcfLinPart                                                   = 3677,
    /// IEEE-1722-TP-CONFIG
    Ieee1722TpConfig                                                       = 2917,
    /// IEEE-1722-TP-CONNECTION-REF-CONDITIONAL
    Ieee1722TpConnectionRefConditional                                     = 5263,
    /// IEEE-1722-TP-CRF-CONNECTION
    Ieee1722TpCrfConnection                                                = 1277,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 2607,
    /// IEEE-1722-TP-IIDC-CONNECTION
    Ieee1722TpIidcConnection                                               = 835,
    /// IEEE-1722-TP-RULE
    Ieee1722TpRule                                                         = 4521,
    /// IEEE-1722-TP-RVF-CONNECTION
    Ieee1722TpRvfConnection                                                = 6260,
    /// IGNORE-AFTER-TX
    IgnoreAfterTx                                                          = 6470,
    /// IGNORE-ORDER-ALLOWED
    IgnoreOrderAllowed                                                     = 6137,
    /// IIDC-CHANNEL
    IidcChannel                                                            = 62,
    /// IIDC-DATA-BLOCK-SIZE
    IidcDataBlockSize                                                      = 3624,
    /// IIDC-FRACTION-NUMBER
    IidcFractionNumber                                                     = 6541,
    /// IIDC-SOURCE-PACKET-HEADER
    IidcSourcePacketHeader                                                 = 1308,
    /// IIDC-STREAM-FORMAT
    IidcStreamFormat                                                       = 1809,
    /// IIDC-SY
    IidcSy                                                                 = 6495,
    /// IIDC-T-CODE
    IidcTCode                                                              = 5596,
    /// IIDC-TAG
    IidcTag                                                                = 5646,
    /// IKE-AUTHENTICATION-METHOD
    IkeAuthenticationMethod                                                = 1332,
    /// IKE-CIPHER-SUITE-NAME
    IkeCipherSuiteName                                                     = 3478,
    /// IKE-OVER-TIME
    IkeOverTime                                                            = 6192,
    /// IKE-RAND-TIME
    IkeRandTime                                                            = 5433,
    /// IKE-REAUTH-TIME
    IkeReauthTime                                                          = 5101,
    /// IKE-REKEY-TIME
    IkeRekeyTime                                                           = 4695,
    /// IMMEDIATE-NV-DATA-STORAGE
    ImmediateNvDataStorage                                                 = 205,
    /// IMMEDIATE-RESUME-TIME
    ImmediateResumeTime                                                    = 1498,
    /// IMPL-CONSTANT-REF
    ImplConstantRef                                                        = 3838,
    /// IMPL-INIT-VALUE
    ImplInitValue                                                          = 5274,
    /// IMPLEMENTATION-ARRAY-ELEMENT-REF
    ImplementationArrayElementRef                                          = 1872,
    /// IMPLEMENTATION-CONFIG-CLASSES
    ImplementationConfigClasses                                            = 3953,
    /// IMPLEMENTATION-CONFIG-VARIANT
    ImplementationConfigVariant                                            = 715,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 3356,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 2966,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 3231,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF
    ImplementationDataTypeElementInAutosarDataPrototypeRef                 = 482,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF
    ImplementationDataTypeElementInPortInterfaceRef                        = 25,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF
    ImplementationDataTypeElementInSystemRef                               = 2160,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    ImplementationDataTypeElementRef                                       = 6406,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 4807,
    /// IMPLEMENTATION-DATA-TYPE-REF
    ImplementationDataTypeRef                                              = 6410,
    /// IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF
    ImplementationDataTypeSubElementRef                                    = 607,
    /// IMPLEMENTATION-PROPSS
    ImplementationPropss                                                   = 3395,
    /// IMPLEMENTATION-RECORD-ELEMENT-REF
    ImplementationRecordElementRef                                         = 1000,
    /// IMPLEMENTATION-REF
    ImplementationRef                                                      = 1013,
    /// IMPLEMENTATION-REFS
    ImplementationRefs                                                     = 2189,
    /// IMPLEMENTED-ENTRY-REF
    ImplementedEntryRef                                                    = 6545,
    /// IMPLEMENTED-ENTRYS
    ImplementedEntrys                                                      = 3621,
    /// IMPLEMENTED-IN-REF
    ImplementedInRef                                                       = 2114,
    /// IMPLEMENTS-LEGACY-STRING-SERIALIZATION
    ImplementsLegacyStringSerialization                                    = 414,
    /// IMPLEMENTS-SOMEIP-STRING-HANDLING
    ImplementsSomeipStringHandling                                         = 3413,
    /// IMPLICIT-DATA-ACCESS-IREF
    ImplicitDataAccessIref                                                 = 3013,
    /// IMPLICIT-DATA-ACCESS-IREFS
    ImplicitDataAccessIrefs                                                = 4792,
    /// IMPLICIT-INTER-RUNNABLE-VARIABLES
    ImplicitInterRunnableVariables                                         = 1818,
    /// IMPLIED-OPERATION-REF
    ImpliedOperationRef                                                    = 5120,
    /// IMPLIED-OPERATION-REFS
    ImpliedOperationRefs                                                   = 2922,
    /// IMPORTANCE
    Importance                                                             = 2883,
    /// IMPOSITION-TIME
    ImpositionTime                                                         = 2369,
    /// IMPOSITION-TIME-DEFINITION-GROUP
    ImpositionTimeDefinitionGroup                                          = 4596,
    /// IMPOSITION-TIMES
    ImpositionTimes                                                        = 6155,
    /// IN-MEASUREMENT-SET
    InMeasurementSet                                                       = 4613,
    /// IN-PLACE
    InPlace                                                                = 6198,
    /// IN-SCOPE
    InScope                                                                = 3253,
    /// INCLUDE-REF
    IncludeRef                                                             = 4909,
    /// INCLUDE-REFS
    IncludeRefs                                                            = 3606,
    /// INCLUDED-CERTIFICATE-REF
    IncludedCertificateRef                                                 = 5006,
    /// INCLUDED-CERTIFICATE-REFS
    IncludedCertificateRefs                                                = 2515,
    /// INCLUDED-DATA-TYPE-SET
    IncludedDataTypeSet                                                    = 1560,
    /// INCLUDED-DATA-TYPE-SETS
    IncludedDataTypeSets                                                   = 2296,
    /// INCLUDED-LIBRARY-REF
    IncludedLibraryRef                                                     = 4350,
    /// INCLUDED-LIBRARY-REFS
    IncludedLibraryRefs                                                    = 3772,
    /// INCLUDED-MODE-DECLARATION-GROUP-SET
    IncludedModeDeclarationGroupSet                                        = 6457,
    /// INCLUDED-MODE-DECLARATION-GROUP-SETS
    IncludedModeDeclarationGroupSets                                       = 1149,
    /// INCLUDED-VARIANT-REF
    IncludedVariantRef                                                     = 5919,
    /// INCLUDED-VARIANT-REFS
    IncludedVariantRefs                                                    = 3983,
    /// INDENT-SAMPLE
    IndentSample                                                           = 4895,
    /// INDEX
    Index                                                                  = 4079,
    /// INDEX-DATA-TYPE-REF
    IndexDataTypeRef                                                       = 760,
    /// INDEXED-ARRAY-ELEMENT
    IndexedArrayElement                                                    = 5025,
    /// INDICATIONS
    Indications                                                            = 5108,
    /// INDICATOR-FAILURE-CYCLE-COUNTER-THRESHOLD
    IndicatorFailureCycleCounterThreshold                                  = 5912,
    /// INDICATOR-REF
    IndicatorRef                                                           = 6170,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 3255,
    /// INDIRECT-API
    IndirectApi                                                            = 1148,
    /// INDUSTRY-GROUP
    IndustryGroup                                                          = 4702,
    /// INFO-TYPE
    InfoType                                                               = 591,
    /// INFO-TYPE-REF
    InfoTypeRef                                                            = 1834,
    /// INFORMAL-FORMULA
    InformalFormula                                                        = 3750,
    /// INFRASTRUCTURE-SERVICES
    InfrastructureServices                                                 = 424,
    /// INGRESS-PORT-REF
    IngressPortRef                                                         = 2473,
    /// INGRESS-PORT-REFS
    IngressPortRefs                                                        = 1016,
    /// INGRESS-PRIORITY
    IngressPriority                                                        = 5301,
    /// INGRESS-VLAN-ID
    IngressVlanId                                                          = 656,
    /// INHIBIT-SOURCES
    InhibitSources                                                         = 5708,
    /// INHIBITING-FID-REF
    InhibitingFidRef                                                       = 4839,
    /// INHIBITING-SECONDARY-FID-REF
    InhibitingSecondaryFidRef                                              = 4490,
    /// INHIBITING-SECONDARY-FID-REFS
    InhibitingSecondaryFidRefs                                             = 4988,
    /// INHIBITION-MASK
    InhibitionMask                                                         = 5415,
    /// INHIBITION-SOURCE-REF
    InhibitionSourceRef                                                    = 5484,
    /// INIT-EVENT
    InitEvent                                                              = 707,
    /// INIT-RESOURCE
    InitResource                                                           = 6562,
    /// INIT-VALUE
    InitValue                                                              = 3066,
    /// INIT-VALUES
    InitValues                                                             = 3171,
    /// INITIAL-CHECKPOINT-REF
    InitialCheckpointRef                                                   = 5174,
    /// INITIAL-CHECKPOINT-REFS
    InitialCheckpointRefs                                                  = 478,
    /// INITIAL-DELAY-MAX-VALUE
    InitialDelayMaxValue                                                   = 2392,
    /// INITIAL-DELAY-MIN-VALUE
    InitialDelayMinValue                                                   = 5054,
    /// INITIAL-DYNAMIC-PART
    InitialDynamicPart                                                     = 2647,
    /// INITIAL-EVENT-STATUS
    InitialEventStatus                                                     = 5892,
    /// INITIAL-FIND-BEHAVIOR
    InitialFindBehavior                                                    = 1568,
    /// INITIAL-INACTIVITY-TIME
    InitialInactivityTime                                                  = 781,
    /// INITIAL-MODE-REF
    InitialModeRef                                                         = 1245,
    /// INITIAL-NAD
    InitialNad                                                             = 592,
    /// INITIAL-OFFER-BEHAVIOR
    InitialOfferBehavior                                                   = 2591,
    /// INITIAL-REF-OFFSET
    InitialRefOffset                                                       = 4050,
    /// INITIAL-REPETITIONS-BASE-DELAY
    InitialRepetitionsBaseDelay                                            = 1680,
    /// INITIAL-REPETITIONS-MAX
    InitialRepetitionsMax                                                  = 5574,
    /// INITIAL-STATUS
    InitialStatus                                                          = 54,
    /// INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    InitialVehicleAnnouncementTime                                         = 2033,
    /// INITIALIZATION-VECTOR-LENGTH
    InitializationVectorLength                                             = 555,
    /// INITIALIZED-PARAMETER-REF
    InitializedParameterRef                                                = 564,
    /// INNER-GROUP-IREF
    InnerGroupIref                                                         = 3233,
    /// INNER-GROUP-IREFS
    InnerGroupIrefs                                                        = 3211,
    /// INNER-PORT-IREF
    InnerPortIref                                                          = 3147,
    /// INPLACE
    Inplace                                                                = 674,
    /// INPUT
    Input                                                                  = 1183,
    /// INPUT-DATAS
    InputDatas                                                             = 3287,
    /// INPUT-VARIABLE-TYPE-REF
    InputVariableTypeRef                                                   = 1946,
    /// INSTALLATION-BEHAVIOR
    InstallationBehavior                                                   = 5926,
    /// INSTANCE-ID
    InstanceId                                                             = 6330,
    /// INSTANCE-ID-PROPS
    InstanceIdProps                                                        = 5841,
    /// INSTANCE-IDENTIFIER
    InstanceIdentifier                                                     = 5594,
    /// INSTANCE-IDS
    InstanceIds                                                            = 3197,
    /// INSTANCE-IN-MEMORY
    InstanceInMemory                                                       = 6146,
    /// INSTANCES
    Instances                                                              = 6409,
    /// INSTANTIATION-DATA-DEF-PROPS
    InstantiationDataDefProps                                              = 5514,
    /// INSTANTIATION-DATA-DEF-PROPSS
    InstantiationDataDefPropss                                             = 1702,
    /// INSTANTIATION-RTE-EVENT-PROPSS
    InstantiationRteEventPropss                                            = 662,
    /// INSTANTIATION-TIMING-EVENT-PROPS
    InstantiationTimingEventProps                                          = 3352,
    /// INTEGER-VALUE-VARIATION-POINT
    IntegerValueVariationPoint                                             = 316,
    /// INTENDED-FILENAME
    IntendedFilename                                                       = 57,
    /// INTENDED-PARTIAL-INITIALIZATION-COUNT
    IntendedPartialInitializationCount                                     = 4737,
    /// INTENDED-TARGET-MACHINE-REF
    IntendedTargetMachineRef                                               = 1902,
    /// INTENDED-TARGET-MACHINE-REFS
    IntendedTargetMachineRefs                                              = 1497,
    /// INTER-MESSAGE-TIME
    InterMessageTime                                                       = 5,
    /// INTERFACE-BLUEPRINT-MAPPING
    InterfaceBlueprintMapping                                              = 1958,
    /// INTERFACE-BLUEPRINT-MAPPINGS
    InterfaceBlueprintMappings                                             = 5137,
    /// INTERFACE-BLUEPRINT-REF
    InterfaceBlueprintRef                                                  = 1097,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 5675,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 4795,
    /// INTERFACE-MAPPINGS
    InterfaceMappings                                                      = 1482,
    /// INTERFACE-REF
    InterfaceRef                                                           = 5496,
    /// INTERFACE-VERSION
    InterfaceVersion                                                       = 3109,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 2272,
    /// INTERNAL-BEHAVIORS
    InternalBehaviors                                                      = 3513,
    /// INTERNAL-CONSTRS
    InternalConstrs                                                        = 2525,
    /// INTERNAL-PRIORITY-VALUE
    InternalPriorityValue                                                  = 2365,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 4546,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 981,
    /// INTERNAL-TRIGGERING-POINT-POLICYS
    InternalTriggeringPointPolicys                                         = 53,
    /// INTERNAL-TRIGGERING-POINTS
    InternalTriggeringPoints                                               = 994,
    /// INTERNET-HEADER-LENGTH
    InternetHeaderLength                                                   = 970,
    /// INTERPOLATION-ROUTINE
    InterpolationRoutine                                                   = 3072,
    /// INTERPOLATION-ROUTINE-MAPPING
    InterpolationRoutineMapping                                            = 3085,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 4889,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REF
    InterpolationRoutineMappingSetRef                                      = 1643,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REFS
    InterpolationRoutineMappingSetRefs                                     = 5215,
    /// INTERPOLATION-ROUTINE-MAPPINGS
    InterpolationRoutineMappings                                           = 86,
    /// INTERPOLATION-ROUTINE-REF
    InterpolationRoutineRef                                                = 1471,
    /// INTERPOLATION-ROUTINES
    InterpolationRoutines                                                  = 1527,
    /// INTERRUPT-CATEGORY
    InterruptCategory                                                      = 3056,
    /// INTERRUPT-SOURCE
    InterruptSource                                                        = 3563,
    /// INTERVAL-LENGTH
    IntervalLength                                                         = 1705,
    /// INTRODUCTION
    Introduction                                                           = 5987,
    /// INVALID-VALUE
    InvalidValue                                                           = 3539,
    /// INVALID-VALUE-BITFIELD-ERRORS-ENABLED
    InvalidValueBitfieldErrorsEnabled                                      = 247,
    /// INVALIDATION-POLICY
    InvalidationPolicy                                                     = 507,
    /// INVALIDATION-POLICYS
    InvalidationPolicys                                                    = 4463,
    /// INVERSE-PROTOCOL-VERSION
    InverseProtocolVersion                                                 = 6016,
    /// INVERT
    Invert                                                                 = 6574,
    /// INVERT-CONDITION
    InvertCondition                                                        = 6283,
    /// INVOCATION
    Invocation                                                             = 1822,
    /// IO-CONTROL-CLASS-REF
    IoControlClassRef                                                      = 5682,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATION
    IoHwAbstractionServerAnnotation                                        = 2948,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATIONS
    IoHwAbstractionServerAnnotations                                       = 2887,
    /// IP-ADDRESS
    IpAddress                                                              = 4748,
    /// IP-ADDRESS-KEEP-BEHAVIOR
    IpAddressKeepBehavior                                                  = 4394,
    /// IP-ADDRESS-PREFIX-LENGTH
    IpAddressPrefixLength                                                  = 6134,
    /// IP-IAM-AUTHENTIC-CONNECTION-PROPS
    IpIamAuthenticConnectionProps                                          = 6505,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 6290,
    /// IP-PROTOCOL
    IpProtocol                                                             = 598,
    /// IP-SEC-CONFIG
    IpSecConfig                                                            = 2560,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 661,
    /// IP-SEC-CONFIG-PROPS-REF
    IpSecConfigPropsRef                                                    = 2557,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 2173,
    /// IP-SEC-RULE
    IpSecRule                                                              = 2026,
    /// IP-SEC-RULES
    IpSecRules                                                             = 6608,
    /// IP-TP-RULE
    IpTpRule                                                               = 5922,
    /// IP-V-4-ADDRESS
    IpV4Address                                                            = 484,
    /// IP-V-6-ADDRESS
    IpV6Address                                                            = 5022,
    /// IP-V-6-PATH-MTU-ENABLED
    IpV6PathMtuEnabled                                                     = 6045,
    /// IP-V-6-PATH-MTU-TIMEOUT
    IpV6PathMtuTimeout                                                     = 5619,
    /// IPV-4-ADDRESS
    Ipv4Address                                                            = 6129,
    /// IPV-4-ADDRESS-MASK
    Ipv4AddressMask                                                        = 3047,
    /// IPV-4-ADDRESS-SOURCE
    Ipv4AddressSource                                                      = 2820,
    /// IPV-4-CONFIGURATION
    Ipv4Configuration                                                      = 4294,
    /// IPV-4-DHCP-SERVER-CONFIGURATION
    Ipv4DhcpServerConfiguration                                            = 4725,
    /// IPV-4-MULTICAST-IP-ADDRESS
    Ipv4MulticastIpAddress                                                 = 2947,
    /// IPV-4-PROPS
    Ipv4Props                                                              = 4644,
    /// IPV-4-RULE
    Ipv4Rule                                                               = 5674,
    /// IPV-6-ADDRESS
    Ipv6Address                                                            = 4592,
    /// IPV-6-ADDRESS-MASK
    Ipv6AddressMask                                                        = 3951,
    /// IPV-6-ADDRESS-SOURCE
    Ipv6AddressSource                                                      = 4355,
    /// IPV-6-CONFIGURATION
    Ipv6Configuration                                                      = 3057,
    /// IPV-6-DHCP-SERVER-CONFIGURATION
    Ipv6DhcpServerConfiguration                                            = 2534,
    /// IPV-6-MULTICAST-IP-ADDRESS
    Ipv6MulticastIpAddress                                                 = 2671,
    /// IPV-6-PROPS
    Ipv6Props                                                              = 2140,
    /// IPV-6-RULE
    Ipv6Rule                                                               = 5998,
    /// IS-ACTIVATION-LINE-DEPENDENT
    IsActivationLineDependent                                              = 2059,
    /// IS-AUTO-VALUE
    IsAutoValue                                                            = 6102,
    /// IS-DEFAULT
    IsDefault                                                              = 3936,
    /// IS-DELTA-PACKAGE
    IsDeltaPackage                                                         = 3668,
    /// IS-DYNAMIC-LENGTH-FIELD-SIZE
    IsDynamicLengthFieldSize                                               = 5489,
    /// IS-EVENT
    IsEvent                                                                = 4132,
    /// IS-EXTERNAL
    IsExternal                                                             = 3612,
    /// IS-FUNC-CLUSTER-ENDPOINT
    IsFuncClusterEndpoint                                                  = 1600,
    /// IS-FUNC-CLUSTER-ENDPOINT-REF
    IsFuncClusterEndpointRef                                               = 4409,
    /// IS-GLOBAL
    IsGlobal                                                               = 4735,
    /// IS-MANDATORY
    IsMandatory                                                            = 2675,
    /// IS-OBD-RELEVANT
    IsObdRelevant                                                          = 863,
    /// IS-OPTIONAL
    IsOptional                                                             = 3705,
    /// IS-PIPELINING-PERMITTED
    IsPipeliningPermitted                                                  = 6343,
    /// IS-PRIVATE
    IsPrivate                                                              = 1716,
    /// IS-REENTRANT
    IsReentrant                                                            = 550,
    /// IS-REQUIRED
    IsRequired                                                             = 4391,
    /// IS-SERVICE
    IsService                                                              = 1889,
    /// IS-STRUCT-WITH-OPTIONAL-ELEMENT
    IsStructWithOptionalElement                                            = 3659,
    /// IS-SYNCHRONOUS
    IsSynchronous                                                          = 568,
    /// IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER
    IsSystemWideGlobalTimeMaster                                           = 2214,
    /// IS-UNUSED
    IsUnused                                                               = 1664,
    /// IS-VARIADIC-TEMPLATE
    IsVariadicTemplate                                                     = 4900,
    /// ISSUED-BY
    IssuedBy                                                               = 2520,
    /// ISSUED-TRIGGERS
    IssuedTriggers                                                         = 5117,
    /// IT-REF
    ItRef                                                                  = 1684,
    /// ITEM
    Item                                                                   = 1309,
    /// ITEM-DEFINITIONS
    ItemDefinitions                                                        = 6620,
    /// ITEM-LABEL
    ItemLabel                                                              = 2455,
    /// ITEMS
    Items                                                                  = 3650,
    /// IUMPR-GROUP
    IumprGroup                                                             = 3390,
    /// IUMPR-GROUP-IDENTIFIERS
    IumprGroupIdentifiers                                                  = 1395,
    /// IUMPR-REF
    IumprRef                                                               = 5577,
    /// IUMPR-REFS
    IumprRefs                                                              = 5583,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 812,
    /// J-1939-CLUSTER-CONDITIONAL
    J1939ClusterConditional                                                = 3844,
    /// J-1939-CLUSTER-VARIANTS
    J1939ClusterVariants                                                   = 822,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 5634,
    /// J-1939-CONTROLLER-APPLICATION-REF
    J1939ControllerApplicationRef                                          = 4324,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING
    J1939ControllerApplicationToJ1939NmNodeMapping                         = 1641,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS
    J1939ControllerApplicationToJ1939NmNodeMappings                        = 439,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NODE-MAPPING
    J1939ControllerApplicationToJ1939NodeMapping                           = 1370,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NODE-MAPPINGS
    J1939ControllerApplicationToJ1939NodeMappings                          = 4028,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 6432,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 3368,
    /// J-1939-DTC-VALUE
    J1939DtcValue                                                          = 2360,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 2898,
    /// J-1939-NM-ECU
    J1939NmEcu                                                             = 2744,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 2890,
    /// J-1939-NM-NODE-REF
    J1939NmNodeRef                                                         = 1898,
    /// J-1939-NODE
    J1939Node                                                              = 4641,
    /// J-1939-NODE-REF
    J1939NodeRef                                                           = 3661,
    /// J-1939-NODES
    J1939Nodes                                                             = 356,
    /// J-1939-PROTECTED-I-PDU
    J1939ProtectedIPdu                                                     = 4980,
    /// J-1939-REQUESTABLE
    J1939Requestable                                                       = 1557,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 5123,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 6468,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 4012,
    /// J-1939-SHARED-ADDRESS-CLUSTERS
    J1939SharedAddressClusters                                             = 5723,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 684,
    /// J-1939-TP-CONNECTION
    J1939TpConnection                                                      = 6348,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 2310,
    /// J-1939-TP-PG
    J1939TpPg                                                              = 2928,
    /// JITTER
    Jitter                                                                 = 5622,
    /// JOB-PRIMITIVE-REF
    JobPrimitiveRef                                                        = 1645,
    /// JOB-REF
    JobRef                                                                 = 5254,
    /// JOB-REQUIREMENTS
    JobRequirements                                                        = 2944,
    /// JUMP-TO-BOOT-LOADER
    JumpToBootLoader                                                       = 546,
    /// K-INIT-RESOURCE
    KInitResource                                                          = 5831,
    /// K-RUN-RESOURCE
    KRunResource                                                           = 5858,
    /// KEEP-ALIVE-INTERVAL
    KeepAliveInterval                                                      = 3481,
    /// KEEP-ALIVE-PROBES-MAX
    KeepAliveProbesMax                                                     = 1483,
    /// KEEP-ALIVE-TIME
    KeepAliveTime                                                          = 3294,
    /// KEEP-ALIVES
    KeepAlives                                                             = 1231,
    /// KEY
    Key                                                                    = 6266,
    /// KEY-EXCHANGE-AUTHENTICATION-REF
    KeyExchangeAuthenticationRef                                           = 665,
    /// KEY-EXCHANGE-AUTHENTICATION-REFS
    KeyExchangeAuthenticationRefs                                          = 214,
    /// KEY-EXCHANGE-REF
    KeyExchangeRef                                                         = 5470,
    /// KEY-EXCHANGE-REFS
    KeyExchangeRefs                                                        = 5079,
    /// KEY-GENERATION
    KeyGeneration                                                          = 1764,
    /// KEY-ID
    KeyId                                                                  = 5686,
    /// KEY-IDENTITY-REF
    KeyIdentityRef                                                         = 4813,
    /// KEY-REF
    KeyRef                                                                 = 3436,
    /// KEY-SERVER-PRIORITY
    KeyServerPriority                                                      = 4043,
    /// KEY-SIZE
    KeySize                                                                = 99,
    /// KEY-SLOT-ALLOWED-MODIFICATION
    KeySlotAllowedModification                                             = 4597,
    /// KEY-SLOT-CONTENT-ALLOWED-USAGES
    KeySlotContentAllowedUsages                                            = 4289,
    /// KEY-SLOT-ID
    KeySlotId                                                              = 2106,
    /// KEY-SLOT-ONLY-ENABLED
    KeySlotOnlyEnabled                                                     = 1496,
    /// KEY-SLOT-REF
    KeySlotRef                                                             = 2102,
    /// KEY-SLOT-USAGE
    KeySlotUsage                                                           = 3347,
    /// KEY-SLOT-USAGES
    KeySlotUsages                                                          = 562,
    /// KEY-SLOT-USED-FOR-START-UP
    KeySlotUsedForStartUp                                                  = 3101,
    /// KEY-SLOT-USED-FOR-SYNC
    KeySlotUsedForSync                                                     = 2767,
    /// KEY-SLOTS
    KeySlots                                                               = 6512,
    /// KEY-STORAGE-TYPE
    KeyStorageType                                                         = 2825,
    /// KEY-VALUE-PAIRS
    KeyValuePairs                                                          = 5214,
    /// KEY-VALUE-STORAGE-REF
    KeyValueStorageRef                                                     = 3492,
    /// KEYWORD
    Keyword                                                                = 260,
    /// KEYWORD-SET
    KeywordSet                                                             = 126,
    /// KEYWORDS
    Keywords                                                               = 3247,
    /// KIND
    Kind                                                                   = 5224,
    /// L-1
    L1                                                                     = 2965,
    /// L-10
    L10                                                                    = 2996,
    /// L-2
    L2                                                                     = 6530,
    /// L-4
    L4                                                                     = 6601,
    /// L-5
    L5                                                                     = 1785,
    /// L-GRAPHIC
    LGraphic                                                               = 2842,
    /// LABEL
    Label                                                                  = 2210,
    /// LABEL-1
    Label1                                                                 = 4308,
    /// LABELED-ITEM
    LabeledItem                                                            = 2229,
    /// LABELED-LIST
    LabeledList                                                            = 2998,
    /// LANGUAGE
    Language                                                               = 2329,
    /// LAST-EGRESS-SCHEDULER-REF
    LastEgressSchedulerRef                                                 = 101,
    /// LATENCY-BUDGET
    LatencyBudget                                                          = 5826,
    /// LATENCY-BUDGET-DURATION
    LatencyBudgetDuration                                                  = 5103,
    /// LATENCY-CONSTRAINT-TYPE
    LatencyConstraintType                                                  = 2909,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 1139,
    /// LATEST-TX
    LatestTx                                                               = 6136,
    /// LC-OBJECT-REF
    LcObjectRef                                                            = 3444,
    /// LC-STATE-REF
    LcStateRef                                                             = 1805,
    /// LC-STATES
    LcStates                                                               = 2322,
    /// LDAP-URL
    LdapUrl                                                                = 1555,
    /// LEAF-ELEMENT-IREF
    LeafElementIref                                                        = 1228,
    /// LEAVE-SUSPEND-TO-RAM-ERROR-REF
    LeaveSuspendToRamErrorRef                                              = 3660,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS
    LegislatedFreezeFrameContentUdsObds                                    = 710,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS
    LegislatedFreezeFrameContentWwhObds                                    = 3262,
    /// LEGISLATED-SOVD-FREEZE-FRAME-CONTENT-REF
    LegislatedSovdFreezeFrameContentRef                                    = 2446,
    /// LENGTH
    Length                                                                 = 1424,
    /// LENGTH-CLIENT-ID
    LengthClientId                                                         = 2571,
    /// LENGTH-EXP
    LengthExp                                                              = 1579,
    /// LENGTH-SEQUENCE-COUNTER
    LengthSequenceCounter                                                  = 209,
    /// LENGTH-VERIFICATION
    LengthVerification                                                     = 3741,
    /// LET-DATA-EXCHANGE-PARADIGM
    LetDataExchangeParadigm                                                = 2530,
    /// LET-INTERVAL-REF
    LetIntervalRef                                                         = 957,
    /// LET-INTERVAL-REFS
    LetIntervalRefs                                                        = 2606,
    /// LICENSE-REF
    LicenseRef                                                             = 2476,
    /// LICENSE-REFS
    LicenseRefs                                                            = 3820,
    /// LIFE-CYCLE-INFO
    LifeCycleInfo                                                          = 4577,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 3977,
    /// LIFE-CYCLE-INFOS
    LifeCycleInfos                                                         = 1336,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 2614,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 2830,
    /// LIFESPAN
    Lifespan                                                               = 6564,
    /// LIFESPAN-DURATION
    LifespanDuration                                                       = 2432,
    /// LIMIT
    Limit                                                                  = 4637,
    /// LIMIT-KIND
    LimitKind                                                              = 4371,
    /// LIN-CHECKSUM
    LinChecksum                                                            = 5206,
    /// LIN-CLUSTER
    LinCluster                                                             = 5598,
    /// LIN-CLUSTER-CONDITIONAL
    LinClusterConditional                                                  = 1413,
    /// LIN-CLUSTER-VARIANTS
    LinClusterVariants                                                     = 6474,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 4976,
    /// LIN-CONFIGURABLE-FRAME
    LinConfigurableFrame                                                   = 2007,
    /// LIN-CONFIGURABLE-FRAMES
    LinConfigurableFrames                                                  = 4766,
    /// LIN-ERROR-RESPONSE
    LinErrorResponse                                                       = 2648,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 401,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 3343,
    /// LIN-IDENTIFIER
    LinIdentifier                                                          = 1084,
    /// LIN-MASTER
    LinMaster                                                              = 3018,
    /// LIN-MASTER-CONDITIONAL
    LinMasterConditional                                                   = 4897,
    /// LIN-MASTER-VARIANTS
    LinMasterVariants                                                      = 5587,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 761,
    /// LIN-ORDERED-CONFIGURABLE-FRAME
    LinOrderedConfigurableFrame                                            = 3667,
    /// LIN-ORDERED-CONFIGURABLE-FRAMES
    LinOrderedConfigurableFrames                                           = 5890,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 6322,
    /// LIN-PID-TO-CAN-ID-MAPPINGS
    LinPidToCanIdMappings                                                  = 3265,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 1929,
    /// LIN-SLAVE
    LinSlave                                                               = 5359,
    /// LIN-SLAVE-CONDITIONAL
    LinSlaveConditional                                                    = 5135,
    /// LIN-SLAVE-CONFIG
    LinSlaveConfig                                                         = 6233,
    /// LIN-SLAVE-ECU-REF
    LinSlaveEcuRef                                                         = 1710,
    /// LIN-SLAVE-VARIANTS
    LinSlaveVariants                                                       = 5417,
    /// LIN-SLAVES
    LinSlaves                                                              = 5965,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 2147,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 5851,
    /// LIN-TP-CONNECTION
    LinTpConnection                                                        = 5258,
    /// LIN-TP-N-SDU-REF
    LinTpNSduRef                                                           = 2532,
    /// LIN-TP-NODE
    LinTpNode                                                              = 6492,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 977,
    /// LIN-UNCONDITIONAL-FRAME-REF
    LinUnconditionalFrameRef                                               = 2181,
    /// LIN-UNCONDITIONAL-FRAME-REFS
    LinUnconditionalFrameRefs                                              = 1932,
    /// LINKER
    Linker                                                                 = 3469,
    /// LINKERS
    Linkers                                                                = 6062,
    /// LIST
    List                                                                   = 2347,
    /// LISTEN-NOISE
    ListenNoise                                                            = 2281,
    /// LISTEN-TIMEOUT
    ListenTimeout                                                          = 4066,
    /// LITERAL-PREFIX
    LiteralPrefix                                                          = 4861,
    /// LITERALS
    Literals                                                               = 3869,
    /// LIVELINESS
    Liveliness                                                             = 1222,
    /// LIVELINESS-LEASE-DURATION
    LivelinessLeaseDuration                                                = 2317,
    /// LIVELINESS-PROTECTION-KIND
    LivelinessProtectionKind                                               = 302,
    /// LIVENESS-KIND
    LivenessKind                                                           = 308,
    /// LOAD-BALANCING-PRIORITY
    LoadBalancingPriority                                                  = 2806,
    /// LOAD-BALANCING-WEIGHT
    LoadBalancingWeight                                                    = 1888,
    /// LOC-MEASUREMENT-SET
    LocMeasurementSet                                                      = 872,
    /// LOCAL-CERTIFICATE-REF
    LocalCertificateRef                                                    = 1735,
    /// LOCAL-CERTIFICATE-REFS
    LocalCertificateRefs                                                   = 3925,
    /// LOCAL-COM-ACCESS-CONTROL-ENABLED
    LocalComAccessControlEnabled                                           = 5231,
    /// LOCAL-COMM-CONNECTOR-REF
    LocalCommConnectorRef                                                  = 2077,
    /// LOCAL-DEBUG-DATAS
    LocalDebugDatas                                                        = 3263,
    /// LOCAL-ENDPOINT-CONFIG
    LocalEndpointConfig                                                    = 1620,
    /// LOCAL-ID
    LocalId                                                                = 5508,
    /// LOCAL-IP-SEC-RULE-REF
    LocalIpSecRuleRef                                                      = 6325,
    /// LOCAL-IP-SEC-RULE-REFS
    LocalIpSecRuleRefs                                                     = 4529,
    /// LOCAL-NETWORK-ENDPOINT-REF
    LocalNetworkEndpointRef                                                = 5275,
    /// LOCAL-PARAMETER-REF
    LocalParameterRef                                                      = 4062,
    /// LOCAL-PORT-RANGE-END
    LocalPortRangeEnd                                                      = 1130,
    /// LOCAL-PORT-RANGE-START
    LocalPortRangeStart                                                    = 4412,
    /// LOCAL-PORT-REF
    LocalPortRef                                                           = 4187,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 2920,
    /// LOCAL-SUPERVISION-ENTITY-REF
    LocalSupervisionEntityRef                                              = 4853,
    /// LOCAL-SUPERVISION-ENTITY-REFS
    LocalSupervisionEntityRefs                                             = 5199,
    /// LOCAL-SUPERVISION-REF
    LocalSupervisionRef                                                    = 637,
    /// LOCAL-SUPERVISION-REFS
    LocalSupervisionRefs                                                   = 5655,
    /// LOCAL-SUPERVISIONS
    LocalSupervisions                                                      = 3430,
    /// LOCAL-TCP-PORT-REF
    LocalTcpPortRef                                                        = 6331,
    /// LOCAL-UDP-PORT-REF
    LocalUdpPortRef                                                        = 739,
    /// LOCAL-UNICAST-ADDRESSES
    LocalUnicastAddresses                                                  = 3218,
    /// LOCAL-UNICAST-ADDRESSS
    LocalUnicastAddresss                                                   = 5344,
    /// LOCAL-VARIABLE-REF
    LocalVariableRef                                                       = 5951,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 1723,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 3555,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 5245,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF
    LogAndTraceMessageCollectionSetRef                                     = 1831,
    /// LOG-CHANNEL-ID
    LogChannelId                                                           = 1699,
    /// LOG-SINK-REF
    LogSinkRef                                                             = 6327,
    /// LOG-SINK-REFS
    LogSinkRefs                                                            = 2428,
    /// LOG-TRACE-DEFAULT-LOG-LEVEL
    LogTraceDefaultLogLevel                                                = 2722,
    /// LOG-TRACE-DEFAULT-LOG-THRESHOLD
    LogTraceDefaultLogThreshold                                            = 6301,
    /// LOG-TRACE-FILE-PATH
    LogTraceFilePath                                                       = 3967,
    /// LOG-TRACE-LOG-MODE
    LogTraceLogMode                                                        = 5358,
    /// LOG-TRACE-LOG-MODES
    LogTraceLogModes                                                       = 4728,
    /// LOG-TRACE-PORT-ID
    LogTracePortId                                                         = 2577,
    /// LOG-TRACE-PROCESS-DESC
    LogTraceProcessDesc                                                    = 4845,
    /// LOG-TRACE-PROCESS-ID
    LogTraceProcessId                                                      = 5427,
    /// LOGGING-BEHAVIOR
    LoggingBehavior                                                        = 39,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 3258,
    /// LOGIC-ADDRESSS
    LogicAddresss                                                          = 5864,
    /// LOGICAL-ADDRESS
    LogicalAddress                                                         = 803,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 246,
    /// LOGICAL-EXPRESSION-ARGUMENT-REF
    LogicalExpressionArgumentRef                                           = 5809,
    /// LOGICAL-EXPRESSION-ARGUMENT-REFS
    LogicalExpressionArgumentRefs                                          = 2817,
    /// LOGICAL-EXPRESSIONS
    LogicalExpressions                                                     = 1208,
    /// LOGICAL-OPERATOR
    LogicalOperator                                                        = 6507,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 1686,
    /// LOGICAL-SUPERVISIONS
    LogicalSupervisions                                                    = 3432,
    /// LONG-NAME
    LongName                                                               = 5658,
    /// LONG-NAME-1
    LongName1                                                              = 6144,
    /// LOWER-BOUND
    LowerBound                                                             = 1806,
    /// LOWER-BOUNDARY
    LowerBoundary                                                          = 2701,
    /// LOWER-CAN-ID
    LowerCanId                                                             = 1344,
    /// LOWER-LIMIT
    LowerLimit                                                             = 4265,
    /// LOWER-MULTIPLICITY
    LowerMultiplicity                                                      = 6133,
    /// LOWER-REF
    LowerRef                                                               = 1386,
    /// LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING
    LtMessageCollectionToPortPrototypeMapping                              = 745,
    /// LUMINOUS-INTENSITY-EXP
    LuminousIntensityExp                                                   = 3803,
    /// M
    M                                                                      = 4199,
    /// MAC-ADDRESS
    MacAddress                                                             = 1730,
    /// MAC-ADDRESS-MASK
    MacAddressMask                                                         = 4010,
    /// MAC-ADDRESS-STREAM-ID
    MacAddressStreamId                                                     = 4822,
    /// MAC-ADDRESS-VLAN-ASSIGNMENTS
    MacAddressVlanAssignments                                              = 317,
    /// MAC-ADDRESS-VLAN-MEMBERSHIP
    MacAddressVlanMembership                                               = 59,
    /// MAC-LAYER-TYPE
    MacLayerType                                                           = 926,
    /// MAC-MULTICAST-ADDRESS
    MacMulticastAddress                                                    = 2127,
    /// MAC-MULTICAST-ADDRESS-REF
    MacMulticastAddressRef                                                 = 33,
    /// MAC-MULTICAST-ADDRESS-REFS
    MacMulticastAddressRefs                                                = 454,
    /// MAC-MULTICAST-CONFIGURATION
    MacMulticastConfiguration                                              = 1005,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 3379,
    /// MAC-MULTICAST-GROUP-REF
    MacMulticastGroupRef                                                   = 5050,
    /// MAC-MULTICAST-GROUPS
    MacMulticastGroups                                                     = 6488,
    /// MAC-SEC-CIPHER-SUITE-CONFIG
    MacSecCipherSuiteConfig                                                = 1965,
    /// MAC-SEC-GLOBAL-KAY-PROPS
    MacSecGlobalKayProps                                                   = 2604,
    /// MAC-SEC-KAY-CONFIG
    MacSecKayConfig                                                        = 3170,
    /// MAC-SEC-KAY-PARTICIPANT
    MacSecKayParticipant                                                   = 966,
    /// MAC-SEC-PARTICIPANT-SET
    MacSecParticipantSet                                                   = 579,
    /// MAC-SEC-PROPS
    MacSecProps                                                            = 5400,
    /// MAC-SEC-PROPSS
    MacSecPropss                                                           = 1299,
    /// MAC-UNICAST-ADDRESS
    MacUnicastAddress                                                      = 4836,
    /// MACHINE
    Machine                                                                = 3185,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 4295,
    /// MACHINE-DESIGN-REF
    MachineDesignRef                                                       = 5638,
    /// MACHINE-ID
    MachineId                                                              = 1374,
    /// MACHINE-MODE-IREF
    MachineModeIref                                                        = 2568,
    /// MACHINE-MODE-IREFS
    MachineModeIrefs                                                       = 4254,
    /// MACHINE-MODE-MACHINES
    MachineModeMachines                                                    = 4715,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 5487,
    /// MACHINE-REF
    MachineRef                                                             = 1227,
    /// MACHINE-TIMING
    MachineTiming                                                          = 4583,
    /// MACRO-INITIAL-OFFSET-A
    MacroInitialOffsetA                                                    = 4535,
    /// MACRO-INITIAL-OFFSET-B
    MacroInitialOffsetB                                                    = 2417,
    /// MACRO-PER-CYCLE
    MacroPerCycle                                                          = 5595,
    /// MACROTICK-DURATION
    MacrotickDuration                                                      = 3604,
    /// MAIN-FUNCTION-PERIOD
    MainFunctionPeriod                                                     = 967,
    /// MAJOR-PROTOCOL-VERSION
    MajorProtocolVersion                                                   = 5202,
    /// MAJOR-VERSION
    MajorVersion                                                           = 1398,
    /// MANAGED-COUPLING-PORTS
    ManagedCouplingPorts                                                   = 4604,
    /// MANAGED-MODE-GROUPS
    ManagedModeGroups                                                      = 2068,
    /// MANAGED-PHYSICAL-CHANNEL-REF
    ManagedPhysicalChannelRef                                              = 2262,
    /// MANAGED-PHYSICAL-CHANNEL-REFS
    ManagedPhysicalChannelRefs                                             = 4156,
    /// MANDATORY
    Mandatory                                                              = 6063,
    /// MANUFACTURER-CODE
    ManufacturerCode                                                       = 3978,
    /// MANUFACTURER-VALIDATION-ORDER-REF
    ManufacturerValidationOrderRef                                         = 3022,
    /// MANUFACTURER-VALIDATION-ORDER-REFS
    ManufacturerValidationOrderRefs                                        = 2799,
    /// MAP
    Map                                                                    = 5478,
    /// MAP-ELEMENT-TUPLES
    MapElementTuples                                                       = 5046,
    /// MAPPED-AP-DATA-ELEMENT-IREF
    MappedApDataElementIref                                                = 6513,
    /// MAPPED-BSW-SERVICE-DEPENDENCY-REF
    MappedBswServiceDependencyRef                                          = 894,
    /// MAPPED-DATA-ELEMENT-IREF
    MappedDataElementIref                                                  = 5038,
    /// MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF
    MappedFlatSwcServiceDependencyRef                                      = 3611,
    /// MAPPED-FUNCTION-REF
    MappedFunctionRef                                                      = 1340,
    /// MAPPED-OPERATION-IREF
    MappedOperationIref                                                    = 5063,
    /// MAPPED-SECURITY-EVENTS
    MappedSecurityEvents                                                   = 691,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    MappedSwcServiceDependencyInExecutableIref                             = 2304,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    MappedSwcServiceDependencyInSystemIref                                 = 6362,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IREF
    MappedSwcServiceDependencyIref                                         = 1438,
    /// MAPPING-CONSTRAINTS
    MappingConstraints                                                     = 4190,
    /// MAPPING-DIRECTION
    MappingDirection                                                       = 3055,
    /// MAPPING-REF
    MappingRef                                                             = 4067,
    /// MAPPING-SCOPE
    MappingScope                                                           = 2076,
    /// MAPPINGS
    Mappings                                                               = 5175,
    /// MASK
    Mask                                                                   = 192,
    /// MASS-EXP
    MassExp                                                                = 2487,
    /// MASTER
    Master                                                                 = 5872,
    /// MASTER-EVENT-REF
    MasterEventRef                                                         = 5702,
    /// MASTER-REF
    MasterRef                                                              = 5846,
    /// MASTER-TO-SLAVE-EVENT-MAPPINGS
    MasterToSlaveEventMappings                                             = 240,
    /// MASTERED-TRIGGER-REF
    MasteredTriggerRef                                                     = 1727,
    /// MATCHING-CRITERION-REF
    MatchingCriterionRef                                                   = 4124,
    /// MATCHING-EGRESS-RULE-REF
    MatchingEgressRuleRef                                                  = 2638,
    /// MATCHING-EGRESS-RULE-REFS
    MatchingEgressRuleRefs                                                 = 2288,
    /// MATCHING-INGRESS-RULE-REF
    MatchingIngressRuleRef                                                 = 1923,
    /// MATCHING-INGRESS-RULE-REFS
    MatchingIngressRuleRefs                                                = 463,
    /// MATCHING-RULE-REF
    MatchingRuleRef                                                        = 645,
    /// MATCHING-RULE-REFS
    MatchingRuleRefs                                                       = 5227,
    /// MAX
    Max                                                                    = 6072,
    /// MAX-ACTION-ITEM-DURATION
    MaxActionItemDuration                                                  = 1142,
    /// MAX-ACTION-LIST-DURATION
    MaxActionListDuration                                                  = 638,
    /// MAX-ALIVE-CYCLE
    MaxAliveCycle                                                          = 5293,
    /// MAX-AR
    MaxAr                                                                  = 2336,
    /// MAX-AS
    MaxAs                                                                  = 3565,
    /// MAX-AVAILABLE-PERSISTENCY-STORAGE-SPACE
    MaxAvailablePersistencyStorageSpace                                    = 3773,
    /// MAX-BASE-TYPE-SIZE
    MaxBaseTypeSize                                                        = 3801,
    /// MAX-BLOCK-SIZE
    MaxBlockSize                                                           = 1743,
    /// MAX-BS
    MaxBs                                                                  = 103,
    /// MAX-BUFFER-REQUEST
    MaxBufferRequest                                                       = 928,
    /// MAX-BUFFER-SIZE
    MaxBufferSize                                                          = 2403,
    /// MAX-BYTES-IN-INTERVAL
    MaxBytesInInterval                                                     = 3214,
    /// MAX-COMM-MODE
    MaxCommMode                                                            = 6148,
    /// MAX-CONVERSATIONS
    MaxConversations                                                       = 808,
    /// MAX-CYCLE-REPETITIONS
    MaxCycleRepetitions                                                    = 1949,
    /// MAX-CYCLES
    MaxCycles                                                              = 4816,
    /// MAX-DATA-LENGTH
    MaxDataLength                                                          = 2780,
    /// MAX-DEADLINE
    MaxDeadline                                                            = 5420,
    /// MAX-DELTA-COUNTER
    MaxDeltaCounter                                                        = 4238,
    /// MAX-DELTA-COUNTER-INIT
    MaxDeltaCounterInit                                                    = 1402,
    /// MAX-DESTINATION-PORT-NUMBER
    MaxDestinationPortNumber                                               = 6118,
    /// MAX-DID-TO-READ
    MaxDidToRead                                                           = 2196,
    /// MAX-DIFF
    MaxDiff                                                                = 503,
    /// MAX-DURATION-EXCEEDED-ERROR-REF
    MaxDurationExceededErrorRef                                            = 3999,
    /// MAX-ERROR-STATE-INIT
    MaxErrorStateInit                                                      = 5150,
    /// MAX-ERROR-STATE-INVALID
    MaxErrorStateInvalid                                                   = 1998,
    /// MAX-ERROR-STATE-VALID
    MaxErrorStateValid                                                     = 4769,
    /// MAX-EVENTS-IN-INTERVAL
    MaxEventsInInterval                                                    = 907,
    /// MAX-EXP-BS
    MaxExpBs                                                               = 231,
    /// MAX-FC-WAIT
    MaxFcWait                                                              = 5677,
    /// MAX-FR-IF
    MaxFrIf                                                                = 2963,
    /// MAX-GRADIENT
    MaxGradient                                                            = 1892,
    /// MAX-HEADER-ID
    MaxHeaderId                                                            = 3345,
    /// MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    MaxInitialVehicleAnnouncementTime                                      = 1679,
    /// MAX-INSTANCES
    MaxInstances                                                           = 412,
    /// MAX-LENGTH
    MaxLength                                                              = 6532,
    /// MAX-MAJOR-VERSION
    MaxMajorVersion                                                        = 5085,
    /// MAX-MARGIN
    MaxMargin                                                              = 5405,
    /// MAX-MINOR-VERSION
    MaxMinorVersion                                                        = 5533,
    /// MAX-NO-NEW-OR-REPEATED-DATA
    MaxNoNewOrRepeatedData                                                 = 5779,
    /// MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS
    MaxNumChangeOfDataIdentfierEvents                                      = 3921,
    /// MAX-NUM-COMPARISION-OF-VALUE-EVENTS
    MaxNumComparisionOfValueEvents                                         = 2105,
    /// MAX-NUMBER-FREEZE-FRAME-RECORDS
    MaxNumberFreezeFrameRecords                                            = 3094,
    /// MAX-NUMBER-OF-ALLOWED-UPDATES
    MaxNumberOfAllowedUpdates                                              = 3824,
    /// MAX-NUMBER-OF-CONNECTIONS
    MaxNumberOfConnections                                                 = 4659,
    /// MAX-NUMBER-OF-ELEMENTS
    MaxNumberOfElements                                                    = 4615,
    /// MAX-NUMBER-OF-EVENT-ENTRIES
    MaxNumberOfEventEntries                                                = 631,
    /// MAX-NUMBER-OF-FILES
    MaxNumberOfFiles                                                       = 3307,
    /// MAX-NUMBER-OF-NPDU-PER-CYCLE
    MaxNumberOfNpduPerCycle                                                = 6441,
    /// MAX-NUMBER-OF-OCCURRENCES
    MaxNumberOfOccurrences                                                 = 3882,
    /// MAX-NUMBER-OF-PARALLEL-TRANSFERS
    MaxNumberOfParallelTransfers                                           = 4523,
    /// MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING
    MaxNumberOfRequestCorrectlyReceivedResponsePending                     = 1986,
    /// MAX-NUMBER-OF-RESP-PENDING-FRAMES
    MaxNumberOfRespPendingFrames                                           = 5635,
    /// MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS
    MaxNumberOfStoredDtcStatusChangedEvents                                = 4363,
    /// MAX-NUMBER-OF-TIME-QUANTA-PER-BIT
    MaxNumberOfTimeQuantaPerBit                                            = 1964,
    /// MAX-PERIODIC-DID-TO-READ
    MaxPeriodicDidToRead                                                   = 4854,
    /// MAX-PROGRESSION-MISMATCH-THRESHOLD
    MaxProgressionMismatchThreshold                                        = 3620,
    /// MAX-PWM-L
    MaxPwmL                                                                = 3103,
    /// MAX-PWM-O
    MaxPwmO                                                                = 4630,
    /// MAX-PWM-S
    MaxPwmS                                                                = 2751,
    /// MAX-REQUEST-BYTES
    MaxRequestBytes                                                        = 2731,
    /// MAX-RESIDENCE-TIME
    MaxResidenceTime                                                       = 338,
    /// MAX-RETRIES
    MaxRetries                                                             = 329,
    /// MAX-RPT-EVENT-ID
    MaxRptEventId                                                          = 2757,
    /// MAX-RUN-CYCLE-LOOP
    MaxRunCycleLoop                                                        = 332,
    /// MAX-SAMPLE-POINT
    MaxSamplePoint                                                         = 4000,
    /// MAX-SAMPLES
    MaxSamples                                                             = 3848,
    /// MAX-SAMPLES-PER-INSTANCE
    MaxSamplesPerInstance                                                  = 2461,
    /// MAX-SDU-SIZE
    MaxSduSize                                                             = 5457,
    /// MAX-SERVICE-POINT-ID
    MaxServicePointId                                                      = 5225,
    /// MAX-SIZE-TO-FILL
    MaxSizeToFill                                                          = 3221,
    /// MAX-SLOTS
    MaxSlots                                                               = 5482,
    /// MAX-SLOTS-PER-CYCLE
    MaxSlotsPerCycle                                                       = 2853,
    /// MAX-SOURCE-ELEMENT
    MaxSourceElement                                                       = 2488,
    /// MAX-SOURCE-PORT-NUMBER
    MaxSourcePortNumber                                                    = 3230,
    /// MAX-SUPPORTED-DID-LENGTH
    MaxSupportedDidLength                                                  = 4663,
    /// MAX-SYNC-JUMP-WIDTH
    MaxSyncJumpWidth                                                       = 6428,
    /// MAX-TESTER-CONNECTIONS
    MaxTesterConnections                                                   = 5465,
    /// MAX-TRANSIT-TIME
    MaxTransitTime                                                         = 1667,
    /// MAX-TRCV-DELAY-COMPENSATION-OFFSET
    MaxTrcvDelayCompensationOffset                                         = 646,
    /// MAX-VALUE
    MaxValue                                                               = 5667,
    /// MAX-WITHOUT-CLOCK-CORRECTION-FATAL
    MaxWithoutClockCorrectionFatal                                         = 3698,
    /// MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE
    MaxWithoutClockCorrectionPassive                                       = 4452,
    /// MAXIMUM
    Maximum                                                                = 1214,
    /// MAXIMUM-ALLOWED-SIZE
    MaximumAllowedSize                                                     = 1436,
    /// MAXIMUM-DISTANCES
    MaximumDistances                                                       = 4220,
    /// MAXIMUM-DURATION-OF-CAMPAIGN
    MaximumDurationOfCampaign                                              = 3139,
    /// MAXIMUM-DYNAMIC-PAYLOAD-LENGTH
    MaximumDynamicPayloadLength                                            = 46,
    /// MAXIMUM-EXECUTION-TIME
    MaximumExecutionTime                                                   = 3198,
    /// MAXIMUM-INTENDED-BINDING-TIME
    MaximumIntendedBindingTime                                             = 5453,
    /// MAXIMUM-INTER-ARRIVAL-TIME
    MaximumInterArrivalTime                                                = 2078,
    /// MAXIMUM-KEY-LENGTH
    MaximumKeyLength                                                       = 3021,
    /// MAXIMUM-LENGTH
    MaximumLength                                                          = 6321,
    /// MAXIMUM-MEMORY-CONSUMPTION
    MaximumMemoryConsumption                                               = 5114,
    /// MAXIMUM-MESSAGE-LENGTH
    MaximumMessageLength                                                   = 297,
    /// MAXIMUM-NUMBER-OF-RETRIES
    MaximumNumberOfRetries                                                 = 1012,
    /// MAXIMUM-RECEIVE-BUFFER-LENGTH
    MaximumReceiveBufferLength                                             = 6316,
    /// MAXIMUM-RESIDENCE-TIME
    MaximumResidenceTime                                                   = 1417,
    /// MAXIMUM-SEGMENT-LENGTH
    MaximumSegmentLength                                                   = 724,
    /// MAXIMUM-SEGMENT-LENGTH-REQUEST
    MaximumSegmentLengthRequest                                            = 5246,
    /// MAXIMUM-SEGMENT-LENGTH-RESPONSE
    MaximumSegmentLengthResponse                                           = 5849,
    /// MAXIMUM-SELECTED-BINDING-TIME
    MaximumSelectedBindingTime                                             = 2344,
    /// MAXIMUM-SUPPORTED-UCM-MASTER-VERSION
    MaximumSupportedUcmMasterVersion                                       = 4368,
    /// MAXIMUM-SUPPORTED-UCM-VERSION
    MaximumSupportedUcmVersion                                             = 2116,
    /// MAXIMUM-TRANSMISSION-UNIT
    MaximumTransmissionUnit                                                = 6591,
    /// MAXIMUM-TRANSMIT-BUFFER-LENGTH
    MaximumTransmitBufferLength                                            = 880,
    /// MAY-BE-UNCONNECTED
    MayBeUnconnected                                                       = 5868,
    /// MC-DATA-ACCESS-DETAILS
    McDataAccessDetails                                                    = 6056,
    /// MC-DATA-ASSIGNMENTS
    McDataAssignments                                                      = 2735,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 1706,
    /// MC-DATA-INSTANCE-REF
    McDataInstanceRef                                                      = 4249,
    /// MC-DATA-INSTANCE-REFS
    McDataInstanceRefs                                                     = 2020,
    /// MC-DATA-INSTANCE-VAR-REF
    McDataInstanceVarRef                                                   = 2774,
    /// MC-FUNCTION
    McFunction                                                             = 107,
    /// MC-FUNCTION-DATA-REF-SET-CONDITIONAL
    McFunctionDataRefSetConditional                                        = 3965,
    /// MC-FUNCTION-DATA-REF-SET-VARIANTS
    McFunctionDataRefSetVariants                                           = 752,
    /// MC-FUNCTION-REF
    McFunctionRef                                                          = 1678,
    /// MC-FUNCTION-REFS
    McFunctionRefs                                                         = 3425,
    /// MC-GROUP
    McGroup                                                                = 6594,
    /// MC-GROUP-DATA-REF-SET-CONDITIONAL
    McGroupDataRefSetConditional                                           = 6458,
    /// MC-GROUP-DATA-REF-SET-VARIANTS
    McGroupDataRefSetVariants                                              = 990,
    /// MC-PARAMETER-ELEMENT-GROUP
    McParameterElementGroup                                                = 5437,
    /// MC-PARAMETER-INSTANCES
    McParameterInstances                                                   = 1412,
    /// MC-SUPPORT
    McSupport                                                              = 4090,
    /// MC-SW-EMULATION-METHOD-SUPPORT
    McSwEmulationMethodSupport                                             = 5887,
    /// MC-VARIABLE-INSTANCES
    McVariableInstances                                                    = 5029,
    /// MCD-IDENTIFIER
    McdIdentifier                                                          = 3732,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REF
    MeasurableSystemConstantValuesRef                                      = 6207,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS
    MeasurableSystemConstantValuesRefs                                     = 2168,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 3189,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 2149,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 4429,
    /// MEM-ALIGNMENT
    MemAlignment                                                           = 4327,
    /// MEM-CLASS-SYMBOL
    MemClassSymbol                                                         = 2472,
    /// MEM-USAGE
    MemUsage                                                               = 4476,
    /// MEMORY-ALLOCATION-KEYWORD-POLICY
    MemoryAllocationKeywordPolicy                                          = 5207,
    /// MEMORY-CONSUMPTION
    MemoryConsumption                                                      = 445,
    /// MEMORY-DESTINATION-REF
    MemoryDestinationRef                                                   = 1768,
    /// MEMORY-DESTINATION-REFS
    MemoryDestinationRefs                                                  = 4319,
    /// MEMORY-ENTRY-STORAGE-TRIGGER
    MemoryEntryStorageTrigger                                              = 3916,
    /// MEMORY-HIGH-ADDRESS
    MemoryHighAddress                                                      = 3187,
    /// MEMORY-HIGH-ADDRESS-LABEL
    MemoryHighAddressLabel                                                 = 1587,
    /// MEMORY-ID
    MemoryId                                                               = 6124,
    /// MEMORY-LOW-ADDRESS
    MemoryLowAddress                                                       = 2625,
    /// MEMORY-LOW-ADDRESS-LABEL
    MemoryLowAddressLabel                                                  = 6,
    /// MEMORY-RANGE-REF
    MemoryRangeRef                                                         = 6514,
    /// MEMORY-RANGE-REFS
    MemoryRangeRefs                                                        = 997,
    /// MEMORY-SECTION
    MemorySection                                                          = 725,
    /// MEMORY-SECTION-LOCATION
    MemorySectionLocation                                                  = 1704,
    /// MEMORY-SECTION-LOCATIONS
    MemorySectionLocations                                                 = 1975,
    /// MEMORY-SECTIONS
    MemorySections                                                         = 2184,
    /// MEMORY-USAGE
    MemoryUsage                                                            = 6403,
    /// MEMORY-USAGES
    MemoryUsages                                                           = 403,
    /// MESSAGE-COMPLIANCE
    MessageCompliance                                                      = 5772,
    /// MESSAGE-ID
    MessageId                                                              = 4772,
    /// MESSAGE-LINE-NUMBER
    MessageLineNumber                                                      = 2,
    /// MESSAGE-LINK-LENGTH
    MessageLinkLength                                                      = 2463,
    /// MESSAGE-LINK-POSITION
    MessageLinkPosition                                                    = 448,
    /// MESSAGE-SOURCE-FILE
    MessageSourceFile                                                      = 6255,
    /// MESSAGE-TYPE
    MessageType                                                            = 5703,
    /// MESSAGE-TYPE-INFO
    MessageTypeInfo                                                        = 4153,
    /// META-DATA-FIELDS
    MetaDataFields                                                         = 5704,
    /// META-DATA-ITEM
    MetaDataItem                                                           = 3626,
    /// META-DATA-ITEM-SET
    MetaDataItemSet                                                        = 1711,
    /// META-DATA-ITEM-SETS
    MetaDataItemSets                                                       = 3833,
    /// META-DATA-ITEM-TYPE
    MetaDataItemType                                                       = 3419,
    /// META-DATA-ITEMS
    MetaDataItems                                                          = 1919,
    /// META-DATA-LENGTH
    MetaDataLength                                                         = 365,
    /// METADATA-PROTECTION-KIND
    MetadataProtectionKind                                                 = 5248,
    /// METHOD-ACTIVATION-ROUTING-GROUPS
    MethodActivationRoutingGroups                                          = 3727,
    /// METHOD-CALL-REF
    MethodCallRef                                                          = 965,
    /// METHOD-CALL-REFS
    MethodCallRefs                                                         = 747,
    /// METHOD-DEPLOYMENTS
    MethodDeployments                                                      = 3881,
    /// METHOD-ID
    MethodId                                                               = 3391,
    /// METHOD-IREF
    MethodIref                                                             = 1259,
    /// METHOD-MAPPING
    MethodMapping                                                          = 4806,
    /// METHOD-MAPPINGS
    MethodMappings                                                         = 1695,
    /// METHOD-QOS-PROPSS
    MethodQosPropss                                                        = 4129,
    /// METHOD-REF
    MethodRef                                                              = 258,
    /// METHOD-REFS
    MethodRefs                                                             = 4148,
    /// METHOD-REPLY-TOPIC-NAME
    MethodReplyTopicName                                                   = 3009,
    /// METHOD-REQUEST-PROPSS
    MethodRequestPropss                                                    = 3842,
    /// METHOD-REQUEST-TOPIC-NAME
    MethodRequestTopicName                                                 = 6312,
    /// METHOD-RESPONSE-PROPSS
    MethodResponsePropss                                                   = 2341,
    /// METHOD-RETURN-REF
    MethodReturnRef                                                        = 820,
    /// METHOD-RETURN-REFS
    MethodReturnRefs                                                       = 4303,
    /// METHOD-TOPICS-ACCESS-RULE-REF
    MethodTopicsAccessRuleRef                                              = 6217,
    /// METHODS
    Methods                                                                = 3594,
    /// MICRO-INITIAL-OFFSET-A
    MicroInitialOffsetA                                                    = 4468,
    /// MICRO-INITIAL-OFFSET-B
    MicroInitialOffsetB                                                    = 5402,
    /// MICRO-PER-CYCLE
    MicroPerCycle                                                          = 4226,
    /// MICROTICK-DURATION
    MicrotickDuration                                                      = 4983,
    /// MIN
    Min                                                                    = 4176,
    /// MIN-ALIVE-CYCLE
    MinAliveCycle                                                          = 5249,
    /// MIN-DATA-LENGTH
    MinDataLength                                                          = 429,
    /// MIN-DEADLINE
    MinDeadline                                                            = 223,
    /// MIN-DESTINATION-PORT-NUMBER
    MinDestinationPortNumber                                               = 514,
    /// MIN-HEADER-ID
    MinHeaderId                                                            = 3079,
    /// MIN-LENGTH
    MinLength                                                              = 574,
    /// MIN-MAJOR-VERSION
    MinMajorVersion                                                        = 2037,
    /// MIN-MARGIN
    MinMargin                                                              = 4539,
    /// MIN-MINOR-VERSION
    MinMinorVersion                                                        = 5021,
    /// MIN-NUMBER-OF-ELEMENTS
    MinNumberOfElements                                                    = 5945,
    /// MIN-NUMBER-OF-OCCURRENCES
    MinNumberOfOccurrences                                                 = 4203,
    /// MIN-NUMBER-OF-TIME-QUANTA-PER-BIT
    MinNumberOfTimeQuantaPerBit                                            = 6586,
    /// MIN-OK-STATE-INIT
    MinOkStateInit                                                         = 2261,
    /// MIN-OK-STATE-INVALID
    MinOkStateInvalid                                                      = 4098,
    /// MIN-OK-STATE-VALID
    MinOkStateValid                                                        = 6643,
    /// MIN-PWM-L
    MinPwmL                                                                = 3304,
    /// MIN-PWM-O
    MinPwmO                                                                = 1959,
    /// MIN-PWM-S
    MinPwmS                                                                = 4763,
    /// MIN-RPT-EVENT-ID
    MinRptEventId                                                          = 1103,
    /// MIN-SAMPLE-POINT
    MinSamplePoint                                                         = 6614,
    /// MIN-SERVICE-POINT-ID
    MinServicePointId                                                      = 6069,
    /// MIN-SOURCE-PORT-NUMBER
    MinSourcePortNumber                                                    = 2544,
    /// MIN-SYNC-JUMP-WIDTH
    MinSyncJumpWidth                                                       = 3030,
    /// MIN-TRCV-DELAY-COMPENSATION-OFFSET
    MinTrcvDelayCompensationOffset                                         = 2063,
    /// MIN-VALUE
    MinValue                                                               = 1155,
    /// MINIMUM
    Minimum                                                                = 2185,
    /// MINIMUM-DELAY
    MinimumDelay                                                           = 942,
    /// MINIMUM-DISTANCES
    MinimumDistances                                                       = 5170,
    /// MINIMUM-EXECUTION-TIME
    MinimumExecutionTime                                                   = 5632,
    /// MINIMUM-FIFO-LENGTH
    MinimumFifoLength                                                      = 2182,
    /// MINIMUM-INTENDED-BINDING-TIME
    MinimumIntendedBindingTime                                             = 202,
    /// MINIMUM-INTER-ARRIVAL-TIME
    MinimumInterArrivalTime                                                = 431,
    /// MINIMUM-INTERVAL-LENGTH
    MinimumIntervalLength                                                  = 4071,
    /// MINIMUM-MEMORY-CONSUMPTION
    MinimumMemoryConsumption                                               = 2359,
    /// MINIMUM-MULTICAST-SEPERATION-TIME
    MinimumMulticastSeperationTime                                         = 4144,
    /// MINIMUM-NUMBER-OF-REQUESTS
    MinimumNumberOfRequests                                                = 1331,
    /// MINIMUM-RX-CONTAINER-QUEUE-SIZE
    MinimumRxContainerQueueSize                                            = 461,
    /// MINIMUM-SELECTED-BINDING-TIME
    MinimumSelectedBindingTime                                             = 3113,
    /// MINIMUM-SEND-INTERVAL
    MinimumSendInterval                                                    = 6013,
    /// MINIMUM-SEPARATION-TIME
    MinimumSeparationTime                                                  = 6393,
    /// MINIMUM-START-INTERVAL
    MinimumStartInterval                                                   = 3605,
    /// MINIMUM-SUPPORTED-UCM-MASTER-VERSION
    MinimumSupportedUcmMasterVersion                                       = 4299,
    /// MINIMUM-SUPPORTED-UCM-VERSION
    MinimumSupportedUcmVersion                                             = 4136,
    /// MINIMUM-SUSTAINED-SIZE
    MinimumSustainedSize                                                   = 676,
    /// MINIMUM-TIMER-GRANULARITY
    MinimumTimerGranularity                                                = 2507,
    /// MINIMUM-TX-CONTAINER-QUEUE-SIZE
    MinimumTxContainerQueueSize                                            = 5146,
    /// MINISLOT-ACTION-POINT-OFFSET
    MinislotActionPointOffset                                              = 5722,
    /// MINISLOT-DURATION
    MinislotDuration                                                       = 1682,
    /// MINOR-PROTOCOL-VERSION
    MinorProtocolVersion                                                   = 230,
    /// MINOR-VERSION
    MinorVersion                                                           = 2167,
    /// MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID
    MirrorSourceLinToCanRangeBaseId                                        = 6227,
    /// MIRROR-STATUS-CAN-ID
    MirrorStatusCanId                                                      = 1338,
    /// MIRRORING-PROTOCOL
    MirroringProtocol                                                      = 3972,
    /// MIXED-BUS-TYPE-COLLECTION
    MixedBusTypeCollection                                                 = 1458,
    /// MKA-PARTICIPANT-REF
    MkaParticipantRef                                                      = 1739,
    /// MKA-PARTICIPANT-REFS
    MkaParticipantRefs                                                     = 5861,
    /// MKA-PARTICIPANTS
    MkaParticipants                                                        = 2541,
    /// ML-GRAPHIC
    MlGraphic                                                              = 32,
    /// MODE
    Mode                                                                   = 5645,
    /// MODE-ACCESS-POINT
    ModeAccessPoint                                                        = 263,
    /// MODE-ACCESS-POINTS
    ModeAccessPoints                                                       = 5361,
    /// MODE-CONDITION
    ModeCondition                                                          = 4751,
    /// MODE-CONDITION-REF
    ModeConditionRef                                                       = 5327,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 2540,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 889,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 5242,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-IREF
    ModeDeclarationGroupPrototypeIref                                      = 5554,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ModeDeclarationGroupPrototypeRef                                       = 6438,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL
    ModeDeclarationGroupPrototypeRefConditional                            = 5244,
    /// MODE-DECLARATION-GROUP-REF
    ModeDeclarationGroupRef                                                = 233,
    /// MODE-DECLARATION-GROUP-REFS
    ModeDeclarationGroupRefs                                               = 1235,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 391,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 5455,
    /// MODE-DECLARATION-MAPPING-SET-REF
    ModeDeclarationMappingSetRef                                           = 1922,
    /// MODE-DECLARATION-MAPPINGS
    ModeDeclarationMappings                                                = 2516,
    /// MODE-DECLARATION-REF
    ModeDeclarationRef                                                     = 3680,
    /// MODE-DECLARATION-REFS
    ModeDeclarationRefs                                                    = 1666,
    /// MODE-DECLARATIONS
    ModeDeclarations                                                       = 417,
    /// MODE-DEPENDENT-STARTUP-CONFIG
    ModeDependentStartupConfig                                             = 696,
    /// MODE-DEPENDENT-STARTUP-CONFIGS
    ModeDependentStartupConfigs                                            = 1003,
    /// MODE-DRIVEN-FALSE-CONDITIONS
    ModeDrivenFalseConditions                                              = 2759,
    /// MODE-DRIVEN-TRANSMISSION-MODE-CONDITION
    ModeDrivenTransmissionModeCondition                                    = 2013,
    /// MODE-DRIVEN-TRUE-CONDITIONS
    ModeDrivenTrueConditions                                               = 2552,
    /// MODE-ELEMENT-REF
    ModeElementRef                                                         = 2833,
    /// MODE-ELEMENTS
    ModeElements                                                           = 6270,
    /// MODE-GROUP
    ModeGroup                                                              = 3822,
    /// MODE-GROUP-IREF
    ModeGroupIref                                                          = 3987,
    /// MODE-GROUP-REF
    ModeGroupRef                                                           = 6413,
    /// MODE-IN-BSW-INSTANCE-REF
    ModeInBswInstanceRef                                                   = 2654,
    /// MODE-IN-SWC-INSTANCE-REF
    ModeInSwcInstanceRef                                                   = 5493,
    /// MODE-INSTANCE
    ModeInstance                                                           = 5542,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 1075,
    /// MODE-IREF
    ModeIref                                                               = 870,
    /// MODE-IREFS
    ModeIrefs                                                              = 2154,
    /// MODE-MANAGER-ERROR-BEHAVIOR
    ModeManagerErrorBehavior                                               = 6065,
    /// MODE-MAPPING
    ModeMapping                                                            = 4973,
    /// MODE-PORT-ANNOTATION
    ModePortAnnotation                                                     = 3331,
    /// MODE-PORT-ANNOTATIONS
    ModePortAnnotations                                                    = 6113,
    /// MODE-RECEIVER-POLICYS
    ModeReceiverPolicys                                                    = 2698,
    /// MODE-REF
    ModeRef                                                                = 2765,
    /// MODE-REPORTER-PORT-IREF
    ModeReporterPortIref                                                   = 2786,
    /// MODE-REQUEST-TYPE-MAP
    ModeRequestTypeMap                                                     = 279,
    /// MODE-REQUEST-TYPE-MAPS
    ModeRequestTypeMaps                                                    = 122,
    /// MODE-SENDER-POLICYS
    ModeSenderPolicys                                                      = 2877,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY
    ModeSwitchEventTriggeredActivity                                       = 5122,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS
    ModeSwitchEventTriggeredActivitys                                      = 2075,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 4701,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 6411,
    /// MODE-SWITCH-POINTS
    ModeSwitchPoints                                                       = 2874,
    /// MODE-SWITCH-RECEIVER-COM-SPEC
    ModeSwitchReceiverComSpec                                              = 6003,
    /// MODE-SWITCH-SENDER-COM-SPEC
    ModeSwitchSenderComSpec                                                = 3132,
    /// MODE-SWITCH-SENDER-COM-SPEC-PROPS
    ModeSwitchSenderComSpecProps                                           = 4239,
    /// MODE-SWITCH-SENDER-COM-SPEC-PROPSS
    ModeSwitchSenderComSpecPropss                                          = 1611,
    /// MODE-SWITCHED-ACK
    ModeSwitchedAck                                                        = 2202,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 2433,
    /// MODE-TRANSITION
    ModeTransition                                                         = 4336,
    /// MODE-TRANSITIONS
    ModeTransitions                                                        = 1717,
    /// MODE-USER-ERROR-BEHAVIOR
    ModeUserErrorBehavior                                                  = 1855,
    /// MODEL-OBJECT-REFERENCE
    ModelObjectReference                                                   = 1797,
    /// MODES
    Modes                                                                  = 961,
    /// MODIFICATION
    Modification                                                           = 1671,
    /// MODIFICATIONS
    Modifications                                                          = 5701,
    /// MODIFIED-DATAS
    ModifiedDatas                                                          = 986,
    /// MODULE-DESCRIPTION-REF
    ModuleDescriptionRef                                                   = 1541,
    /// MODULE-ID
    ModuleId                                                               = 4938,
    /// MODULE-INSTANTIATION-REF
    ModuleInstantiationRef                                                 = 3695,
    /// MODULE-INSTANTIATION-REFS
    ModuleInstantiationRefs                                                = 6140,
    /// MODULE-INSTANTIATIONS
    ModuleInstantiations                                                   = 3470,
    /// MODULE-REF
    ModuleRef                                                              = 5200,
    /// MODULE-REFS
    ModuleRefs                                                             = 1862,
    /// MOLAR-AMOUNT-EXP
    MolarAmountExp                                                         = 4841,
    /// MONITORED-IDENTIFIER-REF
    MonitoredIdentifierRef                                                 = 4609,
    /// MONOTONY
    Monotony                                                               = 1303,
    /// MORE-FRAGMENTS
    MoreFragments                                                          = 974,
    /// MSG-ID-MASK
    MsgIdMask                                                              = 5019,
    /// MSG-ID-MATCH
    MsgIdMatch                                                             = 165,
    /// MSR-QUERY-ARG
    MsrQueryArg                                                            = 1203,
    /// MSR-QUERY-CHAPTER
    MsrQueryChapter                                                        = 3161,
    /// MSR-QUERY-NAME
    MsrQueryName                                                           = 1346,
    /// MSR-QUERY-P-1
    MsrQueryP1                                                             = 672,
    /// MSR-QUERY-P-2
    MsrQueryP2                                                             = 4984,
    /// MSR-QUERY-PROPS
    MsrQueryProps                                                          = 848,
    /// MSR-QUERY-RESULT-CHAPTER
    MsrQueryResultChapter                                                  = 5329,
    /// MSR-QUERY-RESULT-P-1
    MsrQueryResultP1                                                       = 5033,
    /// MSR-QUERY-RESULT-P-2
    MsrQueryResultP2                                                       = 1239,
    /// MSR-QUERY-RESULT-TOPIC-1
    MsrQueryResultTopic1                                                   = 4384,
    /// MSR-QUERY-TOPIC-1
    MsrQueryTopic1                                                         = 6286,
    /// MULTICAST-CONNECTOR-REF
    MulticastConnectorRef                                                  = 6048,
    /// MULTICAST-CONNECTOR-REFS
    MulticastConnectorRefs                                                 = 4717,
    /// MULTICAST-CREDENTIALS
    MulticastCredentials                                                   = 4440,
    /// MULTICAST-REF
    MulticastRef                                                           = 1040,
    /// MULTICAST-SD-IP-ADDRESS-REF
    MulticastSdIpAddressRef                                                = 4177,
    /// MULTICAST-SECURE-COM-PROPS-REF
    MulticastSecureComPropsRef                                             = 6357,
    /// MULTICAST-SEGMENTATION
    MulticastSegmentation                                                  = 2001,
    /// MULTICAST-THRESHOLD
    MulticastThreshold                                                     = 4660,
    /// MULTICAST-UDP-PORT
    MulticastUdpPort                                                       = 4459,
    /// MULTIPLE-CONFIGURATION-CONTAINER
    MultipleConfigurationContainer                                         = 1409,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 4721,
    /// MULTIPLICITY-CONFIG-CLASSES
    MultiplicityConfigClasses                                              = 2022,
    /// MULTIPLICITY-RESTRICTION
    MultiplicityRestriction                                                = 5962,
    /// N
    N                                                                      = 144,
    /// N-DATA-SETS
    NDataSets                                                              = 551,
    /// N-PDU
    NPdu                                                                   = 5789,
    /// N-PDU-REF
    NPduRef                                                                = 1057,
    /// N-PDU-REFS
    NPduRefs                                                               = 5641,
    /// N-ROM-BLOCKS
    NRomBlocks                                                             = 1054,
    /// NAGLES-ALGORITHM
    NaglesAlgorithm                                                        = 3645,
    /// NAME
    Name                                                                   = 4570,
    /// NAME-TOKEN-VALUE-VARIATION-POINT
    NameTokenValueVariationPoint                                           = 2713,
    /// NAMED-CURVE-ID
    NamedCurveId                                                           = 4726,
    /// NAMESPACES
    Namespaces                                                             = 2858,
    /// NAS-TIMEOUT
    NasTimeout                                                             = 6300,
    /// NATIVE-DECLARATION
    NativeDeclaration                                                      = 5760,
    /// NDP-PROPS
    NdpProps                                                               = 4393,
    /// NEEDS-ORIGINAL-DATA
    NeedsOriginalData                                                      = 3796,
    /// NEIGHBOR-CACHE-SIZE
    NeighborCacheSize                                                      = 3144,
    /// NESTED-CONTEXT-DATAS
    NestedContextDatas                                                     = 6352,
    /// NESTED-ELEMENT-REF
    NestedElementRef                                                       = 5774,
    /// NESTED-ELEMENT-REFS
    NestedElementRefs                                                      = 4650,
    /// NESTED-ELEMENTS
    NestedElements                                                         = 1067,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 1430,
    /// NETWORK-CONFIGURATION-DESIGN-REF
    NetworkConfigurationDesignRef                                          = 1663,
    /// NETWORK-CONFIGURATION-REF
    NetworkConfigurationRef                                                = 1432,
    /// NETWORK-CONFIGURATION-REFS
    NetworkConfigurationRefs                                               = 2179,
    /// NETWORK-CONFIGURATIONS
    NetworkConfigurations                                                  = 3783,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 4578,
    /// NETWORK-ENDPOINT-ADDRESSES
    NetworkEndpointAddresses                                               = 6466,
    /// NETWORK-ENDPOINT-REF
    NetworkEndpointRef                                                     = 74,
    /// NETWORK-ENDPOINT-REF-CONDITIONAL
    NetworkEndpointRefConditional                                          = 199,
    /// NETWORK-ENDPOINT-REFS
    NetworkEndpointRefs                                                    = 492,
    /// NETWORK-ENDPOINTS
    NetworkEndpoints                                                       = 89,
    /// NETWORK-HANDLE-PORT-MAPPING
    NetworkHandlePortMapping                                               = 2705,
    /// NETWORK-HANDLE-REF
    NetworkHandleRef                                                       = 3041,
    /// NETWORK-HANDLES
    NetworkHandles                                                         = 3337,
    /// NETWORK-ID
    NetworkId                                                              = 6036,
    /// NETWORK-IDLE-TIME
    NetworkIdleTime                                                        = 232,
    /// NETWORK-INTERFACE-ID
    NetworkInterfaceId                                                     = 1887,
    /// NETWORK-INTERFACE-REF
    NetworkInterfaceRef                                                    = 2934,
    /// NETWORK-INTERFACE-REFS
    NetworkInterfaceRefs                                                   = 112,
    /// NETWORK-INTERFACES
    NetworkInterfaces                                                      = 840,
    /// NETWORK-LAYER-RULE
    NetworkLayerRule                                                       = 2045,
    /// NETWORK-MANAGEMENT-PORT-INTERFACE
    NetworkManagementPortInterface                                         = 2709,
    /// NETWORK-MANAGEMENT-VECTOR-LENGTH
    NetworkManagementVectorLength                                          = 4697,
    /// NETWORK-MASK
    NetworkMask                                                            = 5749,
    /// NETWORK-REPRESENTATION
    NetworkRepresentation                                                  = 3097,
    /// NETWORK-REPRESENTATION-PROPS
    NetworkRepresentationProps                                             = 5127,
    /// NETWORK-REPRESENTATIONS
    NetworkRepresentations                                                 = 5961,
    /// NETWORK-SEGMENT-ID
    NetworkSegmentId                                                       = 599,
    /// NETWORK-TIME-CONSUMER-REF
    NetworkTimeConsumerRef                                                 = 597,
    /// NETWORK-TIME-MASTER-REF
    NetworkTimeMasterRef                                                   = 2096,
    /// NETWORK-TIME-PROVIDER-REF
    NetworkTimeProviderRef                                                 = 3093,
    /// NETWORK-TIME-SLAVE-REF
    NetworkTimeSlaveRef                                                    = 6333,
    /// NEW-NAD
    NewNad                                                                 = 5719,
    /// NEXT-HEADER
    NextHeader                                                             = 2388,
    /// NEXT-HIGHER-CERTIFICATE-REF
    NextHigherCertificateRef                                               = 6237,
    /// NEXT-STATE-IREF
    NextStateIref                                                          = 5058,
    /// NM-ACTIVE-COORDINATOR
    NmActiveCoordinator                                                    = 6294,
    /// NM-BUS-LOAD-REDUCTION-ENABLED
    NmBusLoadReductionEnabled                                              = 4942,
    /// NM-BUS-SYNCHRONIZATION-ENABLED
    NmBusSynchronizationEnabled                                            = 999,
    /// NM-BUSLOAD-REDUCTION-ACTIVE
    NmBusloadReductionActive                                               = 4307,
    /// NM-BUSLOAD-REDUCTION-ENABLED
    NmBusloadReductionEnabled                                              = 5131,
    /// NM-CAR-WAKE-UP-BIT-POSITION
    NmCarWakeUpBitPosition                                                 = 5458,
    /// NM-CAR-WAKE-UP-FILTER-ENABLED
    NmCarWakeUpFilterEnabled                                               = 651,
    /// NM-CAR-WAKE-UP-FILTER-NODE-ID
    NmCarWakeUpFilterNodeId                                                = 2976,
    /// NM-CAR-WAKE-UP-RX-ENABLED
    NmCarWakeUpRxEnabled                                                   = 1102,
    /// NM-CBV-POSITION
    NmCbvPosition                                                          = 844,
    /// NM-CHANNEL-ACTIVE
    NmChannelActive                                                        = 2788,
    /// NM-CHANNEL-ID
    NmChannelId                                                            = 241,
    /// NM-CHANNEL-SLEEP-MASTER
    NmChannelSleepMaster                                                   = 3010,
    /// NM-CLUSTER-COUPLINGS
    NmClusterCouplings                                                     = 5157,
    /// NM-CLUSTERS
    NmClusters                                                             = 5158,
    /// NM-COM-CONTROL-ENABLED
    NmComControlEnabled                                                    = 4928,
    /// NM-CONFIG
    NmConfig                                                               = 5527,
    /// NM-CONTROL-BIT-VECTOR-ACTIVE
    NmControlBitVectorActive                                               = 5762,
    /// NM-CONTROL-BIT-VECTOR-ENABLED
    NmControlBitVectorEnabled                                              = 5180,
    /// NM-COORD-CLUSTER
    NmCoordCluster                                                         = 1288,
    /// NM-COORD-SYNC-SUPPORT
    NmCoordSyncSupport                                                     = 4493,
    /// NM-COORDINATOR
    NmCoordinator                                                          = 5971,
    /// NM-COORDINATOR-ROLE
    NmCoordinatorRole                                                      = 4189,
    /// NM-CYCLETIME-MAIN-FUNCTION
    NmCycletimeMainFunction                                                = 809,
    /// NM-DATA-CYCLE
    NmDataCycle                                                            = 1876,
    /// NM-DATA-DISABLED
    NmDataDisabled                                                         = 1431,
    /// NM-DATA-ENABLED
    NmDataEnabled                                                          = 4643,
    /// NM-DATA-INFORMATION
    NmDataInformation                                                      = 476,
    /// NM-DETECTION-LOCK
    NmDetectionLock                                                        = 2793,
    /// NM-ECU
    NmEcu                                                                  = 1564,
    /// NM-GLOBAL-COORDINATOR-TIME
    NmGlobalCoordinatorTime                                                = 5994,
    /// NM-HANDLE-REF
    NmHandleRef                                                            = 711,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 5787,
    /// NM-HW-VOTE-ENABLED
    NmHwVoteEnabled                                                        = 2655,
    /// NM-IF-ECU-REF
    NmIfEcuRef                                                             = 1973,
    /// NM-IF-ECUS
    NmIfEcus                                                               = 6475,
    /// NM-IMMEDIATE-NM-CYCLE-TIME
    NmImmediateNmCycleTime                                                 = 4037,
    /// NM-IMMEDIATE-NM-TRANSMISSIONS
    NmImmediateNmTransmissions                                             = 1009,
    /// NM-IMMEDIATE-RESTART-ENABLED
    NmImmediateRestartEnabled                                              = 5518,
    /// NM-INSTANCE-ID
    NmInstanceId                                                           = 722,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 3348,
    /// NM-INTERACTS-WITH-SM-MAPPING
    NmInteractsWithSmMapping                                               = 5840,
    /// NM-LIGHT-TIMEOUT
    NmLightTimeout                                                         = 1724,
    /// NM-MAIN-FUNCTION-ACROSS-FR-CYCLE
    NmMainFunctionAcrossFrCycle                                            = 3036,
    /// NM-MAIN-FUNCTION-PERIOD
    NmMainFunctionPeriod                                                   = 6436,
    /// NM-MESSAGE-TIMEOUT-TIME
    NmMessageTimeoutTime                                                   = 6262,
    /// NM-MSG-CYCLE-OFFSET
    NmMsgCycleOffset                                                       = 2019,
    /// NM-MSG-CYCLE-TIME
    NmMsgCycleTime                                                         = 2738,
    /// NM-MSG-REDUCED-TIME
    NmMsgReducedTime                                                       = 6471,
    /// NM-MULTIPLE-CHANNELS-ENABLED
    NmMultipleChannelsEnabled                                              = 4260,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 3724,
    /// NM-NETWORK-HANDLE-REF
    NmNetworkHandleRef                                                     = 4182,
    /// NM-NETWORK-TIMEOUT
    NmNetworkTimeout                                                       = 4619,
    /// NM-NID-POSITION
    NmNidPosition                                                          = 4704,
    /// NM-NODE-DETECTION-ENABLED
    NmNodeDetectionEnabled                                                 = 182,
    /// NM-NODE-ID
    NmNodeId                                                               = 3067,
    /// NM-NODE-ID-ENABLED
    NmNodeIdEnabled                                                        = 4834,
    /// NM-NODE-REF
    NmNodeRef                                                              = 2499,
    /// NM-NODE-REFS
    NmNodeRefs                                                             = 3821,
    /// NM-NODES
    NmNodes                                                                = 2952,
    /// NM-PASSIVE-MODE-ENABLED
    NmPassiveModeEnabled                                                   = 3172,
    /// NM-PDU
    NmPdu                                                                  = 2665,
    /// NM-PDU-REF
    NmPduRef                                                               = 4542,
    /// NM-PDU-REF-CONDITIONAL
    NmPduRefConditional                                                    = 1035,
    /// NM-PDU-RX-INDICATION-ENABLED
    NmPduRxIndicationEnabled                                               = 5599,
    /// NM-PDUS
    NmPdus                                                                 = 2538,
    /// NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS
    NmPnHandleMultipleNetworkRequests                                      = 1049,
    /// NM-PNC-PARTICIPATION
    NmPncParticipation                                                     = 3327,
    /// NM-RANGE-CONFIG
    NmRangeConfig                                                          = 3591,
    /// NM-READY-SLEEP-COUNT
    NmReadySleepCount                                                      = 5813,
    /// NM-READY-SLEEP-TIME
    NmReadySleepTime                                                       = 2782,
    /// NM-REMOTE-SLEEP-IND-ENABLED
    NmRemoteSleepIndEnabled                                                = 4396,
    /// NM-REMOTE-SLEEP-INDICATION-TIME
    NmRemoteSleepIndicationTime                                            = 441,
    /// NM-REPEAT-MESSAGE-BIT-ACTIVE
    NmRepeatMessageBitActive                                               = 6583,
    /// NM-REPEAT-MESSAGE-BIT-ENABLE
    NmRepeatMessageBitEnable                                               = 4285,
    /// NM-REPEAT-MESSAGE-TIME
    NmRepeatMessageTime                                                    = 2146,
    /// NM-REPEAT-MSG-IND-ENABLED
    NmRepeatMsgIndEnabled                                                  = 135,
    /// NM-REPEAT-MSG-INDICATION-ENABLED
    NmRepeatMsgIndicationEnabled                                           = 6044,
    /// NM-REPETITION-CYCLE
    NmRepetitionCycle                                                      = 5083,
    /// NM-SCHEDULE-VARIANT
    NmScheduleVariant                                                      = 5526,
    /// NM-SHUTDOWN-DELAY-TIMER
    NmShutdownDelayTimer                                                   = 4555,
    /// NM-STATE-CHANGE-IND-ENABLED
    NmStateChangeIndEnabled                                                = 437,
    /// NM-STATE-REQUEST
    NmStateRequest                                                         = 4110,
    /// NM-SYNCHRONIZATION-POINT-ENABLED
    NmSynchronizationPointEnabled                                          = 1092,
    /// NM-SYNCHRONIZING-NETWORK
    NmSynchronizingNetwork                                                 = 4011,
    /// NM-USER-DATA-ENABLED
    NmUserDataEnabled                                                      = 5668,
    /// NM-USER-DATA-LENGTH
    NmUserDataLength                                                       = 4925,
    /// NM-USER-DATA-OFFSET
    NmUserDataOffset                                                       = 5728,
    /// NM-VARIANT
    NmVariant                                                              = 6210,
    /// NM-VECTOR-EARLY-UPDATE
    NmVectorEarlyUpdate                                                    = 1064,
    /// NM-VOTE-INFORMATION
    NmVoteInformation                                                      = 3991,
    /// NM-VOTING-CYCLE
    NmVotingCycle                                                          = 3126,
    /// NM-WAIT-BUS-SLEEP-TIME
    NmWaitBusSleepTime                                                     = 1584,
    /// NO-CHECKPOINT-SUPERVISION
    NoCheckpointSupervision                                                = 319,
    /// NO-CHECKPOINT-SUPERVISIONS
    NoCheckpointSupervisions                                               = 1590,
    /// NO-SUPERVISION
    NoSupervision                                                          = 1519,
    /// NO-SUPERVISIONS
    NoSupervisions                                                         = 4587,
    /// NODE-IDENTIFIER
    NodeIdentifier                                                         = 750,
    /// NODE-NAME
    NodeName                                                               = 3699,
    /// NODE-PORTS
    NodePorts                                                              = 527,
    /// NODE-REF
    NodeRef                                                                = 2644,
    /// NOMINAL
    Nominal                                                                = 1882,
    /// NOMINAL-EXECUTION-TIME
    NominalExecutionTime                                                   = 4767,
    /// NON-OS-MODULE-INSTANTIATION-REF
    NonOsModuleInstantiationRef                                            = 1589,
    /// NON-VERBOSE-MODE
    NonVerboseMode                                                         = 4217,
    /// NONQUEUED-RECEIVER-COM-SPEC
    NonqueuedReceiverComSpec                                               = 5816,
    /// NONQUEUED-SENDER-COM-SPEC
    NonqueuedSenderComSpec                                                 = 6226,
    /// NOT-AVAILABLE-VALUE-SPECIFICATION
    NotAvailableValueSpecification                                         = 628,
    /// NOTATION
    Notation                                                               = 1880,
    /// NOTE
    Note                                                                   = 3564,
    /// NOTIFICATION-PORT-IREF
    NotificationPortIref                                                   = 5563,
    /// NOTIFICATION-STATE
    NotificationState                                                      = 3179,
    /// NOTIFICATION-TIME
    NotificationTime                                                       = 5711,
    /// NOTIFICATIONS
    Notifications                                                          = 3872,
    /// NOTIFIED-STATUSS
    NotifiedStatuss                                                        = 4678,
    /// NOTIFIER
    Notifier                                                               = 5480,
    /// NOTIFIER-DATA-ELEMENT-REF
    NotifierDataElementRef                                                 = 4019,
    /// NOTIFIER-REF
    NotifierRef                                                            = 929,
    /// NOTIFIER-SIGNAL-TRIGGERING-REF
    NotifierSignalTriggeringRef                                            = 2624,
    /// NRC-VALUE
    NrcValue                                                               = 4941,
    /// NTU
    Ntu                                                                    = 1522,
    /// NUM-FAILED-SECURITY-ACCESS
    NumFailedSecurityAccess                                                = 6619,
    /// NUMBER
    Number                                                                 = 301,
    /// NUMBER-OF-BITS
    NumberOfBits                                                           = 4349,
    /// NUMBER-OF-CONNECTED-CLIENTS
    NumberOfConnectedClients                                               = 3675,
    /// NUMBER-OF-ELEMENTS
    NumberOfElements                                                       = 4559,
    /// NUMBER-OF-INSTRUCTIONS
    NumberOfInstructions                                                   = 5075,
    /// NUMBER-OF-MINISLOTS
    NumberOfMinislots                                                      = 6256,
    /// NUMBER-OF-NOTIFIER-SETS
    NumberOfNotifierSets                                                   = 473,
    /// NUMBER-OF-PARALLEL-TCP-SESSIONS
    NumberOfParallelTcpSessions                                            = 6275,
    /// NUMBER-OF-REPETITIONS
    NumberOfRepetitions                                                    = 984,
    /// NUMBER-OF-RESTART-ATTEMPTS
    NumberOfRestartAttempts                                                = 5284,
    /// NUMBER-OF-STATIC-SLOTS
    NumberOfStaticSlots                                                    = 2250,
    /// NUMBER-OF-WORKERS
    NumberOfWorkers                                                        = 48,
    /// NUMERICAL-RULE-BASED-VALUE-SPECIFICATION
    NumericalRuleBasedValueSpecification                                   = 700,
    /// NUMERICAL-VALUE
    NumericalValue                                                         = 6089,
    /// NUMERICAL-VALUE-SPECIFICATION
    NumericalValueSpecification                                            = 5756,
    /// NUMERICAL-VALUE-VARIATION-POINT
    NumericalValueVariationPoint                                           = 4204,
    /// NV-BLOCK-DATA-MAPPING
    NvBlockDataMapping                                                     = 1153,
    /// NV-BLOCK-DATA-MAPPINGS
    NvBlockDataMappings                                                    = 5807,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 442,
    /// NV-BLOCK-DESCRIPTORS
    NvBlockDescriptors                                                     = 1204,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 6386,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 620,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 3104,
    /// NV-DATA-PORT-ANNOTATION
    NvDataPortAnnotation                                                   = 2542,
    /// NV-DATA-PORT-ANNOTATIONS
    NvDataPortAnnotations                                                  = 5869,
    /// NV-DATAS
    NvDatas                                                                = 3854,
    /// NV-PROVIDE-COM-SPEC
    NvProvideComSpec                                                       = 1257,
    /// NV-RAM-BLOCK-ELEMENT
    NvRamBlockElement                                                      = 2396,
    /// NV-REQUIRE-COM-SPEC
    NvRequireComSpec                                                       = 2425,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 5506,
    /// OBD-DTC-NUMBER
    ObdDtcNumber                                                           = 4273,
    /// OBD-DTC-VALUE
    ObdDtcValue                                                            = 5935,
    /// OBD-DTC-VALUE-3-BYTE
    ObdDtcValue3Byte                                                       = 1319,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 4944,
    /// OBD-MID
    ObdMid                                                                 = 908,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 3720,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 1301,
    /// OBD-PROPSS
    ObdPropss                                                              = 4128,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 5196,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 4423,
    /// OBD-RELEVANCE
    ObdRelevance                                                           = 4982,
    /// OBD-SUPPORT
    ObdSupport                                                             = 6097,
    /// OBJECT-DEFINITION-REF
    ObjectDefinitionRef                                                    = 1698,
    /// OBJECT-DEFINITION-REFS
    ObjectDefinitionRefs                                                   = 2521,
    /// OBJECT-DEFINTION-REF
    ObjectDefintionRef                                                     = 5407,
    /// OBJECT-DEFINTION-REFS
    ObjectDefintionRefs                                                    = 2798,
    /// OBJECT-IREF
    ObjectIref                                                             = 4125,
    /// OBJECT-REF
    ObjectRef                                                              = 6098,
    /// OBJECT-REFS
    ObjectRefs                                                             = 1580,
    /// OCCURRENCE-COUNTER-PROCESSING
    OccurrenceCounterProcessing                                            = 4884,
    /// OCCURRENCE-EXPRESSION
    OccurrenceExpression                                                   = 2899,
    /// OFFER-CYCLIC-DELAY
    OfferCyclicDelay                                                       = 1534,
    /// OFFSET
    Offset                                                                 = 4298,
    /// OFFSET-CORRECTION-ADAPTION-INTERVAL
    OffsetCorrectionAdaptionInterval                                       = 4846,
    /// OFFSET-CORRECTION-JUMP-THRESHOLD
    OffsetCorrectionJumpThreshold                                          = 4837,
    /// OFFSET-CORRECTION-OUT
    OffsetCorrectionOut                                                    = 4936,
    /// OFFSET-CORRECTION-START
    OffsetCorrectionStart                                                  = 4886,
    /// OFFSET-SI-TO-UNIT
    OffsetSiToUnit                                                         = 4937,
    /// OFFSET-TIME-DOMAIN-REF
    OffsetTimeDomainRef                                                    = 1052,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 5852,
    /// OFFSETS
    Offsets                                                                = 4318,
    /// OFNS-DATA-ID-LIST
    OfnsDataIdList                                                         = 5416,
    /// OFNS-DATA-ID-LISTS
    OfnsDataIdLists                                                        = 3584,
    /// OFS-DATA-ID-LIST
    OfsDataIdList                                                          = 6206,
    /// OFS-DATA-ID-LISTS
    OfsDataIdLists                                                         = 3664,
    /// OFS-SUB-TLV
    OfsSubTlv                                                              = 4306,
    /// ON-BOARD-MONITOR-ID
    OnBoardMonitorId                                                       = 1729,
    /// ON-CHANGE-DATA-PROTOTYPES
    OnChangeDataPrototypes                                                 = 4506,
    /// ON-FAIL-PERMISSIVE-MODE
    OnFailPermissiveMode                                                   = 5753,
    /// ON-FAIL-PERMISSIVE-MODE-TIMEOUT
    OnFailPermissiveModeTimeout                                            = 6374,
    /// ON-TRANSITION-VALUE
    OnTransitionValue                                                      = 3637,
    /// ONE-EVERY-N
    OneEveryN                                                              = 4997,
    /// OP
    Op                                                                     = 5264,
    /// OPERATION-ARGUMENT-INSTANCE-IREF
    OperationArgumentInstanceIref                                          = 6174,
    /// OPERATION-CYCLE
    OperationCycle                                                         = 5409,
    /// OPERATION-CYCLE-AUTOMATIC-END
    OperationCycleAutomaticEnd                                             = 6550,
    /// OPERATION-CYCLE-AUTOSTART
    OperationCycleAutostart                                                = 2564,
    /// OPERATION-CYCLE-REF
    OperationCycleRef                                                      = 1989,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 491,
    /// OPERATION-IREF
    OperationIref                                                          = 5549,
    /// OPERATION-IREFS
    OperationIrefs                                                         = 4739,
    /// OPERATION-MAPPINGS
    OperationMappings                                                      = 5996,
    /// OPERATION-MODE
    OperationMode                                                          = 1387,
    /// OPERATION-PROTOTYPE-IREF
    OperationPrototypeIref                                                 = 3184,
    /// OPERATION-REF
    OperationRef                                                           = 3169,
    /// OPERATION-REQUEST-TRIGGERING-REF
    OperationRequestTriggeringRef                                          = 3200,
    /// OPERATION-RESPONSE-TRIGGERING-REF
    OperationResponseTriggeringRef                                         = 31,
    /// OPERATIONS
    Operations                                                             = 3464,
    /// OPERATOR
    Operator                                                               = 4683,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF
    OptAppRecordElementInMethodIref                                        = 2692,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS
    OptAppRecordElementInMethodIrefs                                       = 1329,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    OptAppRecordElementInServiceInterfaceIref                              = 3927,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS
    OptAppRecordElementInServiceInterfaceIrefs                             = 5754,
    /// OPT-ARGUMENT-REF
    OptArgumentRef                                                         = 6426,
    /// OPT-ARGUMENT-REFS
    OptArgumentRefs                                                        = 5304,
    /// OPT-IMPL-RECORD-ELEMENT
    OptImplRecordElement                                                   = 2124,
    /// OPT-IMPL-RECORD-ELEMENTS
    OptImplRecordElements                                                  = 5156,
    /// OPTION
    Option                                                                 = 3580,
    /// OPTION-ARGUMENT
    OptionArgument                                                         = 3001,
    /// OPTION-KIND
    OptionKind                                                             = 5844,
    /// OPTION-NAME
    OptionName                                                             = 4608,
    /// OPTIONAL
    Optional                                                               = 3863,
    /// OPTIONAL-ELEMENTS
    OptionalElements                                                       = 5149,
    /// OPTIONS
    Options                                                                = 3853,
    /// ORDERED-ELEMENTS
    OrderedElements                                                        = 4219,
    /// ORDERED-MASTER
    OrderedMaster                                                          = 6584,
    /// ORDERED-MASTER-LIST
    OrderedMasterList                                                      = 1779,
    /// ORIGIN
    Origin                                                                 = 5764,
    /// OS-ARTI-ADAPTER-LAUNCH-BEHAVIOR
    OsArtiAdapterLaunchBehavior                                            = 1916,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 5331,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 2490,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 5091,
    /// OS-TASK-PROXY-REF
    OsTaskProxyRef                                                         = 6496,
    /// OUT-MEASUREMENT-SET
    OutMeasurementSet                                                      = 2370,
    /// OUT-MEASURMENT-SET
    OutMeasurmentSet                                                       = 2635,
    /// OUT-OF-RANGE-BITFIELD-ERRORS-ENABLED
    OutOfRangeBitfieldErrorsEnabled                                        = 5621,
    /// OUTER-PORT-REF
    OuterPortRef                                                           = 2616,
    /// OUTER-PORTS
    OuterPorts                                                             = 4540,
    /// OUTGOING-CALLBACKS
    OutgoingCallbacks                                                      = 4279,
    /// OVERRIDE-ID
    OverrideId                                                             = 6000,
    /// OVERRIDE-INITIAL-STATE-IREF
    OverrideInitialStateIref                                               = 800,
    /// OWNER-REF
    OwnerRef                                                               = 3003,
    /// OWNER-REFS
    OwnerRefs                                                              = 1014,
    /// OWNERSHIP
    Ownership                                                              = 5814,
    /// OWNERSHIP-KIND
    OwnershipKind                                                          = 1514,
    /// OWNERSHIP-STRENGTH
    OwnershipStrength                                                      = 5052,
    /// P
    P                                                                      = 4022,
    /// P-2-MAX
    P2Max                                                                  = 2687,
    /// P-2-SERVER-MAX
    P2ServerMax                                                            = 779,
    /// P-2-STAR-SERVER-MAX
    P2StarServerMax                                                        = 4178,
    /// P-2-TIMING
    P2Timing                                                               = 120,
    /// P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    PModeGroupInAtomicSwcInstanceRef                                       = 1871,
    /// P-PORT-IN-COMPOSITION-INSTANCE-REF
    PPortInCompositionInstanceRef                                          = 1920,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 3429,
    /// P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    PPortPrototypeInExecutableIref                                         = 6114,
    /// P-PORT-PROTOTYPE-IREF
    PPortPrototypeIref                                                     = 4194,
    /// P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF
    PTriggerInAtomicSwcTypeInstanceRef                                     = 899,
    /// PACKAGE-REF
    PackageRef                                                             = 1770,
    /// PACKAGER-ID
    PackagerId                                                             = 2346,
    /// PACKAGER-SIGNATURE-REF
    PackagerSignatureRef                                                   = 6257,
    /// PACKAGING-PIN-NAME
    PackagingPinName                                                       = 1660,
    /// PACKING-BYTE-ORDER
    PackingByteOrder                                                       = 6334,
    /// PADDING-ACTIVATION
    PaddingActivation                                                      = 1081,
    /// PADDING-VALUE
    PaddingValue                                                           = 4814,
    /// PARAMETER
    Parameter                                                              = 4168,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 68,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REF
    ParameterAccessedForDebugRef                                           = 3071,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REFS
    ParameterAccessedForDebugRefs                                          = 76,
    /// PARAMETER-ACCESSS
    ParameterAccesss                                                       = 4494,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 1581,
    /// PARAMETER-DATA-PROTOTYPE-IREF
    ParameterDataPrototypeIref                                             = 4880,
    /// PARAMETER-ELEMENT-ACCESS
    ParameterElementAccess                                                 = 6510,
    /// PARAMETER-ID
    ParameterId                                                            = 6143,
    /// PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT
    ParameterImplementationDataTypeElement                                 = 603,
    /// PARAMETER-INSTANCE
    ParameterInstance                                                      = 43,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 5134,
    /// PARAMETER-POLICYS
    ParameterPolicys                                                       = 3229,
    /// PARAMETER-PORT-ANNOTATION
    ParameterPortAnnotation                                                = 3332,
    /// PARAMETER-PORT-ANNOTATIONS
    ParameterPortAnnotations                                               = 5900,
    /// PARAMETER-PROVIDE-COM-SPEC
    ParameterProvideComSpec                                                = 4315,
    /// PARAMETER-REF
    ParameterRef                                                           = 432,
    /// PARAMETER-REQUIRE-COM-SPEC
    ParameterRequireComSpec                                                = 1422,
    /// PARAMETER-SIZE
    ParameterSize                                                          = 142,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 4114,
    /// PARAMETER-VALUES
    ParameterValues                                                        = 1577,
    /// PARAMETERS
    Parameters                                                             = 4488,
    /// PARENT-CATEGORY
    ParentCategory                                                         = 2287,
    /// PARENT-SHORT-LABEL
    ParentShortLabel                                                       = 1020,
    /// PARTIAL-NETWORK-REF
    PartialNetworkRef                                                      = 6236,
    /// PARTIAL-NETWORK-REFS
    PartialNetworkRefs                                                     = 964,
    /// PARTICIPATING-J-1939-CLUSTER-REF
    ParticipatingJ1939ClusterRef                                           = 6012,
    /// PARTICIPATING-J-1939-CLUSTER-REFS
    ParticipatingJ1939ClusterRefs                                          = 4964,
    /// PARTITION-NAME
    PartitionName                                                          = 572,
    /// PARTITION-REF
    PartitionRef                                                           = 226,
    /// PARTITIONS
    Partitions                                                             = 2576,
    /// PARTS
    Parts                                                                  = 4703,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 2578,
    /// PATH
    Path                                                                   = 5167,
    /// PATH-MTU-DISCOVERY-ENABLED
    PathMtuDiscoveryEnabled                                                = 3033,
    /// PATH-MTU-ENABLED
    PathMtuEnabled                                                         = 85,
    /// PATH-MTU-TIMEOUT
    PathMtuTimeout                                                         = 2615,
    /// PATTERN
    Pattern                                                                = 2115,
    /// PATTERN-JITTER
    PatternJitter                                                          = 5102,
    /// PATTERN-LENGTH
    PatternLength                                                          = 3870,
    /// PATTERN-PERIOD
    PatternPeriod                                                          = 2441,
    /// PAYLOAD-BYTE-PATTERN-RULE
    PayloadBytePatternRule                                                 = 3578,
    /// PAYLOAD-BYTE-PATTERN-RULE-PART
    PayloadBytePatternRulePart                                             = 1859,
    /// PAYLOAD-BYTE-PATTERN-RULE-PARTS
    PayloadBytePatternRuleParts                                            = 1380,
    /// PAYLOAD-BYTE-PATTERN-RULES
    PayloadBytePatternRules                                                = 4787,
    /// PAYLOAD-LENGTH
    PayloadLength                                                          = 1272,
    /// PAYLOAD-LENGTH-STATIC
    PayloadLengthStatic                                                    = 6034,
    /// PAYLOAD-PREAMBLE-INDICATOR
    PayloadPreambleIndicator                                               = 2086,
    /// PAYLOAD-REF
    PayloadRef                                                             = 3505,
    /// PAYLOAD-TYPE
    PayloadType                                                            = 6022,
    /// PAYLOADS
    Payloads                                                               = 6527,
    /// PCM-BIT-DEPTH
    PcmBitDepth                                                            = 5502,
    /// PDELAY-LATENCY-THRESHOLD
    PdelayLatencyThreshold                                                 = 265,
    /// PDELAY-REQUEST-PERIOD
    PdelayRequestPeriod                                                    = 992,
    /// PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT
    PdelayRespAndRespFollowUpTimeout                                       = 4636,
    /// PDELAY-RESPONSE-ENABLED
    PdelayResponseEnabled                                                  = 4690,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 1127,
    /// PDU-ACTIVATION-ROUTING-GROUPS
    PduActivationRoutingGroups                                             = 3330,
    /// PDU-COLLECTION-MAX-BUFFER-SIZE
    PduCollectionMaxBufferSize                                             = 639,
    /// PDU-COLLECTION-PDU-TIMEOUT
    PduCollectionPduTimeout                                                = 2159,
    /// PDU-COLLECTION-SEMANTICS
    PduCollectionSemantics                                                 = 3502,
    /// PDU-COLLECTION-TIMEOUT
    PduCollectionTimeout                                                   = 5229,
    /// PDU-COLLECTION-TRIGGER
    PduCollectionTrigger                                                   = 2311,
    /// PDU-COUNTER-SIZE
    PduCounterSize                                                         = 4401,
    /// PDU-COUNTER-START-POSITION
    PduCounterStartPosition                                                = 4015,
    /// PDU-COUNTER-THRESHOLD
    PduCounterThreshold                                                    = 6394,
    /// PDU-COUNTERS
    PduCounters                                                            = 6603,
    /// PDU-MAX-LENGTH
    PduMaxLength                                                           = 3191,
    /// PDU-POOLS
    PduPools                                                               = 3424,
    /// PDU-REF
    PduRef                                                                 = 1841,
    /// PDU-REPLICATION-VOTING
    PduReplicationVoting                                                   = 2264,
    /// PDU-REPLICATIONS
    PduReplications                                                        = 45,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 882,
    /// PDU-TO-FRAME-MAPPINGS
    PduToFrameMappings                                                     = 5895,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 2081,
    /// PDU-TRIGGERING-REF
    PduTriggeringRef                                                       = 5585,
    /// PDU-TRIGGERING-REF-CONDITIONAL
    PduTriggeringRefConditional                                            = 3790,
    /// PDU-TRIGGERING-REFS
    PduTriggeringRefs                                                      = 1283,
    /// PDU-TRIGGERINGS
    PduTriggerings                                                         = 2277,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 460,
    /// PDUR-TP-CHUNK-SIZE
    PdurTpChunkSize                                                        = 836,
    /// PDUS
    Pdus                                                                   = 4912,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 4221,
    /// PER-INSTANCE-MEMORY-REF
    PerInstanceMemoryRef                                                   = 2900,
    /// PER-INSTANCE-MEMORY-SIZE
    PerInstanceMemorySize                                                  = 1501,
    /// PER-INSTANCE-MEMORY-SIZES
    PerInstanceMemorySizes                                                 = 2231,
    /// PER-INSTANCE-MEMORYS
    PerInstanceMemorys                                                     = 6407,
    /// PER-INSTANCE-PARAMETER-REF
    PerInstanceParameterRef                                                = 6360,
    /// PER-INSTANCE-PARAMETERS
    PerInstanceParameters                                                  = 3981,
    /// PER-STATE-TIMEOUT
    PerStateTimeout                                                        = 4434,
    /// PER-STATE-TIMEOUTS
    PerStateTimeouts                                                       = 315,
    /// PERIOD
    Period                                                                 = 3817,
    /// PERIOD-BEGIN
    PeriodBegin                                                            = 3276,
    /// PERIOD-END
    PeriodEnd                                                              = 3422,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 4422,
    /// PERIODIC-RATE-CATEGORY
    PeriodicRateCategory                                                   = 1195,
    /// PERIODIC-RATES
    PeriodicRates                                                          = 6224,
    /// PERIODIC-RESPONSE-UUDT-REF
    PeriodicResponseUudtRef                                                = 772,
    /// PERIODIC-RESPONSE-UUDT-REFS
    PeriodicResponseUudtRefs                                               = 6145,
    /// PERMISSIBLE-SIGNAL-PATH
    PermissibleSignalPath                                                  = 4395,
    /// PERMISSION-CERTIFICATE-AUTHORITY-REF
    PermissionCertificateAuthorityRef                                      = 1461,
    /// PERMISSION-TO-CREATE-CHILD-PROCESS
    PermissionToCreateChildProcess                                         = 6390,
    /// PERMIT-MULTIPLE-REFERENCES-TO-EE
    PermitMultipleReferencesToEe                                           = 6091,
    /// PERSISTENCY-ACCESS
    PersistencyAccess                                                      = 528,
    /// PERSISTENCY-CENTRAL-STORAGE-URI
    PersistencyCentralStorageUri                                           = 5235,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 1325,
    /// PERSISTENCY-DATA-PROVIDED-COM-SPEC
    PersistencyDataProvidedComSpec                                         = 2457,
    /// PERSISTENCY-DATA-REQUIRED-COM-SPEC
    PersistencyDataRequiredComSpec                                         = 6061,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-REF
    PersistencyDeploymentElementRef                                        = 4656,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 4626,
    /// PERSISTENCY-DEPLOYMENT-REF
    PersistencyDeploymentRef                                               = 3442,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 2551,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 6220,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 4252,
    /// PERSISTENCY-DEPLOYMENT-URI
    PersistencyDeploymentUri                                               = 5222,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 5370,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 559,
    /// PERSISTENCY-FILE-ARRAY-REF
    PersistencyFileArrayRef                                                = 1538,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 2779,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 2279,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 2862,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 3367,
    /// PERSISTENCY-FILE-REQUIRED-COM-SPEC
    PersistencyFileRequiredComSpec                                         = 529,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 5218,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 9,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 5567,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 5051,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 188,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 2684,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 2050,
    /// PERSISTENCY-PORT-PROTOTYPE-IREF
    PersistencyPortPrototypeIref                                           = 4123,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 6168,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 4211,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 4796,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 2323,
    /// PERSISTENCY-PROVIDED-COM-SPEC
    PersistencyProvidedComSpec                                             = 2599,
    /// PERSISTENCY-REDUNDANCY-CRC
    PersistencyRedundancyCrc                                               = 6154,
    /// PERSISTENCY-REDUNDANCY-HASH
    PersistencyRedundancyHash                                              = 5494,
    /// PERSISTENCY-REDUNDANCY-M-OUT-OF-N
    PersistencyRedundancyMOutOfN                                           = 1799,
    /// PERSISTENT-STORAGE
    PersistentStorage                                                      = 500,
    /// PGN
    Pgn                                                                    = 4036,
    /// PHM-ACTION
    PhmAction                                                              = 4832,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 2121,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 4115,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 5940,
    /// PHM-CHECKPOINT-IREF
    PhmCheckpointIref                                                      = 1243,
    /// PHM-CONTRIBUTION-REF
    PhmContributionRef                                                     = 459,
    /// PHM-CONTRIBUTION-REFS
    PhmContributionRefs                                                    = 1694,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 6141,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 1648,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 398,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 5817,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 2469,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 151,
    /// PHM-RULE
    PhmRule                                                                = 452,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 1633,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 933,
    /// PHYS-CONSTRS
    PhysConstrs                                                            = 4157,
    /// PHYSICAL-CHANNEL-REF
    PhysicalChannelRef                                                     = 5007,
    /// PHYSICAL-CHANNEL-REF-CONDITIONAL
    PhysicalChannelRefConditional                                          = 5805,
    /// PHYSICAL-CHANNEL-REFS
    PhysicalChannelRefs                                                    = 5312,
    /// PHYSICAL-CHANNELS
    PhysicalChannels                                                       = 5109,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 623,
    /// PHYSICAL-DIMENSION-MAPPING
    PhysicalDimensionMapping                                               = 3587,
    /// PHYSICAL-DIMENSION-MAPPING-REF
    PhysicalDimensionMappingRef                                            = 5139,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 336,
    /// PHYSICAL-DIMENSION-MAPPINGS
    PhysicalDimensionMappings                                              = 5418,
    /// PHYSICAL-DIMENSION-REF
    PhysicalDimensionRef                                                   = 4965,
    /// PHYSICAL-LAYER-TYPE
    PhysicalLayerType                                                      = 6561,
    /// PHYSICAL-PROPS
    PhysicalProps                                                          = 2475,
    /// PHYSICAL-REQUEST-REF
    PhysicalRequestRef                                                     = 407,
    /// PID
    Pid                                                                    = 2650,
    /// PID-REF
    PidRef                                                                 = 1375,
    /// PID-REFS
    PidRefs                                                                = 1353,
    /// PID-SIZE
    PidSize                                                                = 2558,
    /// PIN-NUMBER
    PinNumber                                                              = 5679,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 3311,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 4007,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 1425,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 5477,
    /// PLATFORM-TIME-BASES
    PlatformTimeBases                                                      = 533,
    /// PLCA-LOCAL-NODE-COUNT
    PlcaLocalNodeCount                                                     = 3858,
    /// PLCA-LOCAL-NODE-ID
    PlcaLocalNodeId                                                        = 699,
    /// PLCA-MAX-BURST-COUNT
    PlcaMaxBurstCount                                                      = 2198,
    /// PLCA-MAX-BURST-TIMER
    PlcaMaxBurstTimer                                                      = 6578,
    /// PLCA-PROPS
    PlcaProps                                                              = 2355,
    /// PLCA-TRANSMIT-OPPORTUNITY-TIMER
    PlcaTransmitOpportunityTimer                                           = 5468,
    /// PN-RESET-TIME
    PnResetTime                                                            = 6525,
    /// PN-RESET-TIMER
    PnResetTimer                                                           = 1803,
    /// PNC-CLUSTER-VECTOR-LENGTH
    PncClusterVectorLength                                                 = 4858,
    /// PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    PncConsumedProvidedServiceInstanceGroups                               = 3105,
    /// PNC-FILTER-ARRAY-MASK
    PncFilterArrayMask                                                     = 5184,
    /// PNC-FILTER-ARRAY-MASKS
    PncFilterArrayMasks                                                    = 3649,
    /// PNC-FILTER-DATA-MASK
    PncFilterDataMask                                                      = 1595,
    /// PNC-GATEWAY-TYPE
    PncGatewayType                                                         = 4233,
    /// PNC-GROUP-REF
    PncGroupRef                                                            = 2430,
    /// PNC-GROUP-REFS
    PncGroupRefs                                                           = 2306,
    /// PNC-IDENTIFIER
    PncIdentifier                                                          = 1860,
    /// PNC-MAPPING
    PncMapping                                                             = 5974,
    /// PNC-MAPPING-IDENT-REF
    PncMappingIdentRef                                                     = 2500,
    /// PNC-MAPPING-IDENT-REF-CONDITIONAL
    PncMappingIdentRefConditional                                          = 4989,
    /// PNC-MAPPING-REF
    PncMappingRef                                                          = 1185,
    /// PNC-MAPPING-REFS
    PncMappingRefs                                                         = 5877,
    /// PNC-MAPPINGS
    PncMappings                                                            = 1870,
    /// PNC-NM-REQUEST
    PncNmRequest                                                           = 5626,
    /// PNC-PDUR-GROUP-REF
    PncPdurGroupRef                                                        = 5969,
    /// PNC-PDUR-GROUP-REFS
    PncPdurGroupRefs                                                       = 5726,
    /// PNC-PREPARE-SLEEP-TIMER
    PncPrepareSleepTimer                                                   = 2651,
    /// PNC-SYNCHRONOUS-WAKEUP
    PncSynchronousWakeup                                                   = 2766,
    /// PNC-VECTOR-LENGTH
    PncVectorLength                                                        = 1654,
    /// PNC-VECTOR-OFFSET
    PncVectorOffset                                                        = 5423,
    /// PNC-WAKEUP-CAN-ID
    PncWakeupCanId                                                         = 3504,
    /// PNC-WAKEUP-CAN-ID-EXTENDED
    PncWakeupCanIdExtended                                                 = 3533,
    /// PNC-WAKEUP-CAN-ID-MASK
    PncWakeupCanIdMask                                                     = 1712,
    /// PNC-WAKEUP-DATA-MASK
    PncWakeupDataMask                                                      = 6011,
    /// PNC-WAKEUP-DLC
    PncWakeupDlc                                                           = 2481,
    /// PNC-WAKEUP-ENABLE
    PncWakeupEnable                                                        = 5243,
    /// POLICY
    Policy                                                                 = 6443,
    /// POLICY-ACTION
    PolicyAction                                                           = 6253,
    /// PORT-ADDRESS
    PortAddress                                                            = 3964,
    /// PORT-API-OPTION
    PortApiOption                                                          = 5933,
    /// PORT-API-OPTIONS
    PortApiOptions                                                         = 5944,
    /// PORT-ARG-VALUES
    PortArgValues                                                          = 2681,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 5649,
    /// PORT-BLUEPRINT-INIT-VALUE
    PortBlueprintInitValue                                                 = 6308,
    /// PORT-BLUEPRINT-MAPPING
    PortBlueprintMapping                                                   = 2971,
    /// PORT-BLUEPRINT-MAPPINGS
    PortBlueprintMappings                                                  = 4645,
    /// PORT-BLUEPRINT-REF
    PortBlueprintRef                                                       = 26,
    /// PORT-CONFIGS
    PortConfigs                                                            = 451,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINT
    PortDefinedArgumentBlueprint                                           = 1533,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINTS
    PortDefinedArgumentBlueprints                                          = 4031,
    /// PORT-DEFINED-ARGUMENT-VALUE
    PortDefinedArgumentValue                                               = 354,
    /// PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS
    PortElementToComResourceMappings                                       = 2831,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 6558,
    /// PORT-GROUP
    PortGroup                                                              = 3005,
    /// PORT-GROUPS
    PortGroups                                                             = 6202,
    /// PORT-INSTANTIATION-BEHAVIOR
    PortInstantiationBehavior                                              = 819,
    /// PORT-INTERFACE-BLUEPRINT-MAPPING
    PortInterfaceBlueprintMapping                                          = 4443,
    /// PORT-INTERFACE-BLUEPRINT-REF
    PortInterfaceBlueprintRef                                              = 1732,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 4197,
    /// PORT-INTERFACE-MAPPINGS
    PortInterfaceMappings                                                  = 1954,
    /// PORT-INTERFACE-REF
    PortInterfaceRef                                                       = 5393,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 502,
    /// PORT-NUMBER
    PortNumber                                                             = 2649,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 558,
    /// PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE
    PortPrototypeBlueprintInitValue                                        = 5128,
    /// PORT-PROTOTYPE-BLUEPRINT-MAPPING
    PortPrototypeBlueprintMapping                                          = 2065,
    /// PORT-PROTOTYPE-BLUEPRINT-REF
    PortPrototypeBlueprintRef                                              = 1540,
    /// PORT-PROTOTYPE-IREF
    PortPrototypeIref                                                      = 2152,
    /// PORT-PROTOTYPE-PROPS
    PortPrototypeProps                                                     = 4001,
    /// PORT-PROTOTYPE-REF
    PortPrototypeRef                                                       = 6105,
    /// PORT-PROTOTYPE-REF-CONDITIONAL
    PortPrototypeRefConditional                                            = 2442,
    /// PORT-PROTOTYPE-SLUG-FRAGMENT
    PortPrototypeSlugFragment                                              = 3669,
    /// PORT-REF
    PortRef                                                                = 2328,
    /// PORT-SCHEDULER
    PortScheduler                                                          = 4131,
    /// PORTS
    Ports                                                                  = 5848,
    /// POSITION
    Position                                                               = 4518,
    /// POSITION-IN-TABLE
    PositionInTable                                                        = 2800,
    /// POSITIVE-INTEGER-VALUE-VARIATION-POINT
    PositiveIntegerValueVariationPoint                                     = 5428,
    /// POSSIBLE-AP-ERROR-REF
    PossibleApErrorRef                                                     = 2879,
    /// POSSIBLE-AP-ERROR-REFS
    PossibleApErrorRefs                                                    = 6057,
    /// POSSIBLE-AP-ERROR-SET-REF
    PossibleApErrorSetRef                                                  = 4768,
    /// POSSIBLE-AP-ERROR-SET-REFS
    PossibleApErrorSetRefs                                                 = 3789,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 2445,
    /// POSSIBLE-ERROR-REACTIONS
    PossibleErrorReactions                                                 = 6415,
    /// POSSIBLE-ERROR-REF
    PossibleErrorRef                                                       = 4786,
    /// POSSIBLE-ERROR-REFS
    PossibleErrorRefs                                                      = 3551,
    /// POSSIBLE-ERRORS
    PossibleErrors                                                         = 324,
    /// POST-BUILD-CHANGEABLE
    PostBuildChangeable                                                    = 5408,
    /// POST-BUILD-VALUE-ACCESS-REF
    PostBuildValueAccessRef                                                = 105,
    /// POST-BUILD-VARIANT-CONDITION
    PostBuildVariantCondition                                              = 6122,
    /// POST-BUILD-VARIANT-CONDITIONS
    PostBuildVariantConditions                                             = 3435,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 3830,
    /// POST-BUILD-VARIANT-CRITERION-VALUE
    PostBuildVariantCriterionValue                                         = 5712,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 5276,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF
    PostBuildVariantCriterionValueSetRef                                   = 2854,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS
    PostBuildVariantCriterionValueSetRefs                                  = 2012,
    /// POST-BUILD-VARIANT-CRITERION-VALUES
    PostBuildVariantCriterionValues                                        = 5154,
    /// POST-BUILD-VARIANT-MULTIPLICITY
    PostBuildVariantMultiplicity                                           = 4256,
    /// POST-BUILD-VARIANT-SUPPORT
    PostBuildVariantSupport                                                = 920,
    /// POST-BUILD-VARIANT-USED
    PostBuildVariantUsed                                                   = 2158,
    /// POST-BUILD-VARIANT-VALUE
    PostBuildVariantValue                                                  = 1529,
    /// POST-VERIFICATION-REBOOT
    PostVerificationReboot                                                 = 834,
    /// POSTS
    Posts                                                                  = 3222,
    /// POWER-DOWN-TIME
    PowerDownTime                                                          = 2733,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 4581,
    /// PRE-ACTIVATE-IREF
    PreActivateIref                                                        = 5346,
    /// PRE-ACTIVATE-IREFS
    PreActivateIrefs                                                       = 2968,
    /// PRE-ACTIVATE-REF
    PreActivateRef                                                         = 4524,
    /// PRE-ACTIVATE-REFS
    PreActivateRefs                                                        = 4744,
    /// PRE-ACTIVATION-REBOOT
    PreActivationReboot                                                    = 4911,
    /// PRE-MAPPING
    PreMapping                                                             = 164,
    /// PRE-SHARED-KEY-REF
    PreSharedKeyRef                                                        = 6092,
    /// PRECONFIGURED-CONFIGURATION-REF
    PreconfiguredConfigurationRef                                          = 3076,
    /// PRECONFIGURED-CONFIGURATION-REFS
    PreconfiguredConfigurationRefs                                         = 648,
    /// PREDECESSOR-ACTION-REF
    PredecessorActionRef                                                   = 4354,
    /// PREDECESSOR-ACTION-REFS
    PredecessorActionRefs                                                  = 5078,
    /// PREDECESSOR-FIFO-REF
    PredecessorFifoRef                                                     = 983,
    /// PREDECESSOR-REF
    PredecessorRef                                                         = 485,
    /// PREDECESSOR-REFS
    PredecessorRefs                                                        = 2699,
    /// PREDEFINED-TEXT
    PredefinedText                                                         = 1506,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 3317,
    /// PREEMPTABILITY
    Preemptability                                                         = 4563,
    /// PREFIX
    Prefix                                                                 = 5219,
    /// PREFIX-REF
    PrefixRef                                                              = 4366,
    /// PREPARE-ROLLBACK
    PrepareRollback                                                        = 6366,
    /// PREPARE-UPDATE
    PrepareUpdate                                                          = 2667,
    /// PRESTORAGE-FREEZE-FRAME
    PrestorageFreezeFrame                                                  = 2967,
    /// PRESTORED-FREEZEFRAME-STORED-IN-NVM
    PrestoredFreezeframeStoredInNvm                                        = 3903,
    /// PREVIOUS-CONTRACT-VERSION
    PreviousContractVersion                                                = 2165,
    /// PREVIOUS-DATA-TYPE-REF
    PreviousDataTypeRef                                                    = 5438,
    /// PREVIOUS-EXECUTABLE-VERSION
    PreviousExecutableVersion                                              = 366,
    /// PRIMARY-DOWNLOAD-URI
    PrimaryDownloadUri                                                     = 2938,
    /// PRIMITIVE
    Primitive                                                              = 2438,
    /// PRIMITIVE-ATTRIBUTE-CONDITION
    PrimitiveAttributeCondition                                            = 499,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 3369,
    /// PRIMITIVE-FAMILY
    PrimitiveFamily                                                        = 23,
    /// PRIMITIVE-TYPE-MAPPINGS
    PrimitiveTypeMappings                                                  = 3053,
    /// PRIORITY
    Priority                                                               = 3708,
    /// PRIORITY-ID
    PriorityId                                                             = 2224,
    /// PRIORITYS
    Prioritys                                                              = 6429,
    /// PRIVACY-LEVEL
    PrivacyLevel                                                           = 6058,
    /// PRM
    Prm                                                                    = 2860,
    /// PRM-CHAR
    PrmChar                                                                = 5968,
    /// PRM-UNIT
    PrmUnit                                                                = 1063,
    /// PRMS
    Prms                                                                   = 5067,
    /// PROCESS
    Process                                                                = 2299,
    /// PROCESS-ARGUMENT
    ProcessArgument                                                        = 3320,
    /// PROCESS-ARGUMENTS
    ProcessArguments                                                       = 4515,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 71,
    /// PROCESS-DESIGN-REF
    ProcessDesignRef                                                       = 767,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 2014,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 5334,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS
    ProcessDesignToMachineDesignMappings                                   = 3933,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 4881,
    /// PROCESS-EXECUTION-ERROR-REF
    ProcessExecutionErrorRef                                               = 866,
    /// PROCESS-MODE-IREF
    ProcessModeIref                                                        = 2682,
    /// PROCESS-MODE-MACHINE
    ProcessModeMachine                                                     = 4519,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 2680,
    /// PROCESS-REF
    ProcessRef                                                             = 2519,
    /// PROCESS-STATE-IREF
    ProcessStateIref                                                       = 5529,
    /// PROCESS-STATE-MACHINE
    ProcessStateMachine                                                    = 155,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 1302,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 6613,
    /// PROCESS-TO-MACHINE-MAPPINGS
    ProcessToMachineMappings                                               = 1850,
    /// PROCESSING-KIND
    ProcessingKind                                                         = 3497,
    /// PROCESSING-STYLE
    ProcessingStyle                                                        = 2870,
    /// PROCESSING-UNIT-REF
    ProcessingUnitRef                                                      = 3194,
    /// PROCESSOR
    Processor                                                              = 6229,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 5311,
    /// PROCESSOR-MODE
    ProcessorMode                                                          = 567,
    /// PROCESSOR-SPEED
    ProcessorSpeed                                                         = 4380,
    /// PROCESSORS
    Processors                                                             = 1439,
    /// PRODUCT-ID
    ProductId                                                              = 3540,
    /// PRODUCT-RELEASE
    ProductRelease                                                         = 4903,
    /// PROFILE-BEHAVIOR
    ProfileBehavior                                                        = 250,
    /// PROFILE-NAME
    ProfileName                                                            = 5434,
    /// PROG-CODE
    ProgCode                                                               = 1708,
    /// PROGRAMMING-LANGUAGE
    ProgrammingLanguage                                                    = 2320,
    /// PROP-SEG
    PropSeg                                                                = 1466,
    /// PROPABILITY
    Propability                                                            = 3462,
    /// PROPAGATION-DELAY
    PropagationDelay                                                       = 6050,
    /// PROPERTYS
    Propertys                                                              = 4574,
    /// PROPS
    Props                                                                  = 4757,
    /// PROTOCOL
    Protocol                                                               = 504,
    /// PROTOCOL-KIND
    ProtocolKind                                                           = 5121,
    /// PROTOCOL-NAME
    ProtocolName                                                           = 3409,
    /// PROTOCOL-VERSION
    ProtocolVersion                                                        = 5659,
    /// PROVIDE
    Provide                                                                = 3997,
    /// PROVIDE-RESOURCES
    ProvideResources                                                       = 2104,
    /// PROVIDED-CLIENT-SERVER-ENTRYS
    ProvidedClientServerEntrys                                             = 4961,
    /// PROVIDED-COM-SPECS
    ProvidedComSpecs                                                       = 1476,
    /// PROVIDED-DATA-REF
    ProvidedDataRef                                                        = 6369,
    /// PROVIDED-DATAS
    ProvidedDatas                                                          = 1232,
    /// PROVIDED-DDS-EVENT-QOS-PROPS
    ProvidedDdsEventQosProps                                               = 5698,
    /// PROVIDED-DDS-OPERATIONS
    ProvidedDdsOperations                                                  = 2235,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 3411,
    /// PROVIDED-DDS-SERVICE-INSTANCE-EVENTS
    ProvidedDdsServiceInstanceEvents                                       = 12,
    /// PROVIDED-ENTRYS
    ProvidedEntrys                                                         = 5647,
    /// PROVIDED-EVENT-GROUPS
    ProvidedEventGroups                                                    = 4414,
    /// PROVIDED-EVENTS
    ProvidedEvents                                                         = 2569,
    /// PROVIDED-FIELDS
    ProvidedFields                                                         = 1060,
    /// PROVIDED-INTERFACE-TREF
    ProvidedInterfaceTref                                                  = 5392,
    /// PROVIDED-MEMORY-REF
    ProvidedMemoryRef                                                      = 2435,
    /// PROVIDED-MODE-GROUP-REF
    ProvidedModeGroupRef                                                   = 4576,
    /// PROVIDED-MODE-GROUPS
    ProvidedModeGroups                                                     = 4594,
    /// PROVIDED-OPERATIONS
    ProvidedOperations                                                     = 3568,
    /// PROVIDED-OUTER-PORT-REF
    ProvidedOuterPortRef                                                   = 4207,
    /// PROVIDED-PORT-PROTOTYPE-IREF
    ProvidedPortPrototypeIref                                              = 5934,
    /// PROVIDED-REQUIRED-INTERFACE-TREF
    ProvidedRequiredInterfaceTref                                          = 3360,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 3781,
    /// PROVIDED-SERVICE-INSTANCE-ID
    ProvidedServiceInstanceId                                              = 1160,
    /// PROVIDED-SERVICE-INSTANCE-REF
    ProvidedServiceInstanceRef                                             = 1854,
    /// PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL
    ProvidedServiceInstanceRefConditional                                  = 5444,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 4817,
    /// PROVIDED-SERVICE-INSTANCES
    ProvidedServiceInstances                                               = 2308,
    /// PROVIDED-SERVICE-PORT-IREF
    ProvidedServicePortIref                                                = 386,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 3878,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 588,
    /// PROVIDER-IREF
    ProviderIref                                                           = 1845,
    /// PROVIDER-RATE-DEVIATION-MAX
    ProviderRateDeviationMax                                               = 2003,
    /// PROVIDER-REF
    ProviderRef                                                            = 1106,
    /// PROVIDER-SOFTWARE-CLUSTER-REF
    ProviderSoftwareClusterRef                                             = 3138,
    /// PROVIEDE-DATA-REF
    ProviedeDataRef                                                        = 3210,
    /// PROXIMITY-PROOF-REQUIRED
    ProximityProofRequired                                                 = 3371,
    /// PSK-IDENTITY
    PskIdentity                                                            = 6576,
    /// PSK-IDENTITY-HINT
    PskIdentityHint                                                        = 2072,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 3745,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS
    PskIdentityToKeySlotMappings                                           = 127,
    /// PUBLISHER
    Publisher                                                              = 2291,
    /// PULSE-TEST
    PulseTest                                                              = 6108,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 372,
    /// PURPOSE-OF-UPDATE-REF
    PurposeOfUpdateRef                                                     = 303,
    /// PUTS
    Puts                                                                   = 4676,
    /// PWM-L
    PwmL                                                                   = 4498,
    /// PWM-O
    PwmO                                                                   = 3292,
    /// PWM-S
    PwmS                                                                   = 1604,
    /// QOS-PROFILE
    QosProfile                                                             = 6180,
    /// QUEUE-LENGTH
    QueueLength                                                            = 2257,
    /// QUEUE-SIZE
    QueueSize                                                              = 1350,
    /// QUEUED-RECEIVER-COM-SPEC
    QueuedReceiverComSpec                                                  = 3151,
    /// QUEUED-RECEIVER-COM-SPEC-PROPS
    QueuedReceiverComSpecProps                                             = 4874,
    /// QUEUED-RECEIVER-COM-SPEC-PROPSS
    QueuedReceiverComSpecPropss                                            = 3819,
    /// QUEUED-SENDER-COM-SPEC
    QueuedSenderComSpec                                                    = 515,
    /// R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    RModeGroupInAtomicSwcInstanceRef                                       = 1495,
    /// R-PORT-IN-COMPOSITION-INSTANCE-REF
    RPortInCompositionInstanceRef                                          = 3812,
    /// R-PORT-IREF
    RPortIref                                                              = 4375,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 4923,
    /// R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    RPortPrototypeInExecutableIref                                         = 464,
    /// R-PORT-PROTOTYPE-IREF
    RPortPrototypeIref                                                     = 951,
    /// R-PORT-PROTOTYPE-PROPS
    RPortPrototypeProps                                                    = 3961,
    /// R-PORT-PROTOTYPE-REF
    RPortPrototypeRef                                                      = 2259,
    /// R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF
    RTriggerInAtomicSwcInstanceRef                                         = 509,
    /// RAM-BLOCK
    RamBlock                                                               = 6187,
    /// RAM-BLOCK-INIT-VALUE
    RamBlockInitValue                                                      = 577,
    /// RAM-BLOCK-STATUS-CONTROL
    RamBlockStatusControl                                                  = 3302,
    /// RAM-LOCATION-REF
    RamLocationRef                                                         = 759,
    /// RANGE-MAX
    RangeMax                                                               = 2225,
    /// RANGE-MIN
    RangeMin                                                               = 1181,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 6537,
    /// RATE-BASED-MONITORED-EVENT-REF
    RateBasedMonitoredEventRef                                             = 4258,
    /// RATE-CORRECTION-MEASUREMENT-DURATION
    RateCorrectionMeasurementDuration                                      = 3270,
    /// RATE-CORRECTION-OUT
    RateCorrectionOut                                                      = 2220,
    /// RATE-CORRECTION-THRESHOLD
    RateCorrectionThreshold                                                = 6559,
    /// RATE-CORRECTIONS-PER-MEASUREMENT-DURATION
    RateCorrectionsPerMeasurementDuration                                  = 1119,
    /// RATE-DEVIATION-MEASUREMENT-DURATION
    RateDeviationMeasurementDuration                                       = 5608,
    /// RATE-LIMITATION-FILTERS
    RateLimitationFilters                                                  = 5165,
    /// RATE-POLICYS
    RatePolicys                                                            = 532,
    /// RATIO-KIND
    RatioKind                                                              = 5568,
    /// RATIONALE
    Rationale                                                              = 4935,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 2888,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 841,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 1761,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 2270,
    /// RAW-DATA-STREAM-INTERFACE-REF
    RawDataStreamInterfaceRef                                              = 5118,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 2385,
    /// RAW-DATA-STREAM-REF
    RawDataStreamRef                                                       = 932,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 4192,
    /// REACTION-CODE
    ReactionCode                                                           = 6280,
    /// READ
    Read                                                                   = 924,
    /// READ-CLASS-REF
    ReadClassRef                                                           = 511,
    /// READ-DATA-CLASS-REF
    ReadDataClassRef                                                       = 3867,
    /// READ-DTC-INFORMATION-CLASS-REF
    ReadDtcInformationClassRef                                             = 642,
    /// READ-LOCAL-VARIABLES
    ReadLocalVariables                                                     = 6551,
    /// READ-NV-DATA
    ReadNvData                                                             = 4135,
    /// READ-SCALING-DATA-CLASS-REF
    ReadScalingDataClassRef                                                = 1050,
    /// READ-STATUS
    ReadStatus                                                             = 1995,
    /// READER-ENTITY-ID
    ReaderEntityId                                                         = 2292,
    /// READONLY
    Readonly                                                               = 4267,
    /// REASON
    Reason                                                                 = 296,
    /// RECEIVE-ACTIVITY
    ReceiveActivity                                                        = 2110,
    /// RECEIVE-WINDOW-MIN
    ReceiveWindowMin                                                       = 3392,
    /// RECEIVED-DATA-REF
    ReceivedDataRef                                                        = 3176,
    /// RECEIVER-ANNOTATION
    ReceiverAnnotation                                                     = 1114,
    /// RECEIVER-CAPABILITY
    ReceiverCapability                                                     = 2494,
    /// RECEIVER-INTENT
    ReceiverIntent                                                         = 331,
    /// RECEIVER-IREF
    ReceiverIref                                                           = 916,
    /// RECEIVER-IREFS
    ReceiverIrefs                                                          = 1286,
    /// RECEIVER-REF
    ReceiverRef                                                            = 845,
    /// RECEIVER-REFS
    ReceiverRefs                                                           = 4403,
    /// RECEPTION-DEFAULT-VALUES
    ReceptionDefaultValues                                                 = 3855,
    /// RECEPTION-POLICYS
    ReceptionPolicys                                                       = 3377,
    /// RECEPTION-PROPS
    ReceptionProps                                                         = 3690,
    /// RECOMMENDED-CONFIGURATION-REF
    RecommendedConfigurationRef                                            = 5546,
    /// RECOMMENDED-CONFIGURATION-REFS
    RecommendedConfigurationRefs                                           = 2258,
    /// RECORD-ELEMENT-MAPPINGS
    RecordElementMappings                                                  = 1571,
    /// RECORD-ELEMENT-REF
    RecordElementRef                                                       = 980,
    /// RECORD-ELEMENTS
    RecordElements                                                         = 4537,
    /// RECORD-NUMBER
    RecordNumber                                                           = 6031,
    /// RECORD-VALUE-SPECIFICATION
    RecordValueSpecification                                               = 379,
    /// RECOVERABLE-IN-SAME-OPERATION-CYCLE
    RecoverableInSameOperationCycle                                        = 3012,
    /// RECOVERY
    Recovery                                                               = 5302,
    /// RECOVERY-ACTION-IREF
    RecoveryActionIref                                                     = 1893,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 4382,
    /// RECOVERY-NOTIFICATION-REF
    RecoveryNotificationRef                                                = 2747,
    /// RECOVERY-NOTIFICATION-REFS
    RecoveryNotificationRefs                                               = 3350,
    /// RECOVERY-NOTIFICATION-RETRY
    RecoveryNotificationRetry                                              = 2764,
    /// RECOVERY-NOTIFICATION-TIMEOUT
    RecoveryNotificationTimeout                                            = 5880,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 2411,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 1162,
    /// RECOVERY-VIA-APPLICATION-ACTION-REF
    RecoveryViaApplicationActionRef                                        = 1087,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 4959,
    /// REDUNDANCY
    Redundancy                                                             = 3980,
    /// REDUNDANCY-HANDLINGS
    RedundancyHandlings                                                    = 5510,
    /// REENTRANCY-LEVEL
    ReentrancyLevel                                                        = 757,
    /// REF
    Ref                                                                    = 3764,
    /// REF-CALPRM-SET
    RefCalprmSet                                                           = 1180,
    /// REF-MEASUREMENT-SET
    RefMeasurementSet                                                      = 3810,
    /// REFERENCE-BASE
    ReferenceBase                                                          = 2497,
    /// REFERENCE-BASES
    ReferenceBases                                                         = 575,
    /// REFERENCE-CONDITION
    ReferenceCondition                                                     = 1794,
    /// REFERENCE-REF
    ReferenceRef                                                           = 6142,
    /// REFERENCE-TABLE-REF
    ReferenceTableRef                                                      = 1559,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 1197,
    /// REFERENCE-VALUE-REF
    ReferenceValueRef                                                      = 1019,
    /// REFERENCE-VALUE-SPECIFICATION
    ReferenceValueSpecification                                            = 1393,
    /// REFERENCE-VALUES
    ReferenceValues                                                        = 580,
    /// REFERENCED-BASELINE
    ReferencedBaseline                                                     = 1985,
    /// REFERENCED-TD-EVENT-SWC-REF
    ReferencedTdEventSwcRef                                                = 4086,
    /// REFERENCED-TD-EVENT-VFB-REF
    ReferencedTdEventVfbRef                                                = 3894,
    /// REFERENCES
    References                                                             = 5389,
    /// REFERRABLE-REF
    ReferrableRef                                                          = 6379,
    /// REFILL-AMOUNT
    RefillAmount                                                           = 5177,
    /// REFINED-EVENT-IREF
    RefinedEventIref                                                       = 494,
    /// REFINED-MODULE-DEF-REF
    RefinedModuleDefRef                                                    = 853,
    /// REG-DOES-NOT-REQUIRE-STABILITYS
    RegDoesNotRequireStabilitys                                            = 3450,
    /// REG-REQUIRES-STABILITYS
    RegRequiresStabilitys                                                  = 4076,
    /// REGENERATED-PRIORITY
    RegeneratedPriority                                                    = 181,
    /// REGULAR-EXPRESSION
    RegularExpression                                                      = 3234,
    /// REKEY-INTERVAL
    RekeyInterval                                                          = 4611,
    /// RELATED-TRACE-ITEM-REF
    RelatedTraceItemRef                                                    = 4553,
    /// RELATIONS
    Relations                                                              = 6005,
    /// RELATIVE
    Relative                                                               = 5718,
    /// RELATIVE-REPRESENTATION-TIME
    RelativeRepresentationTime                                             = 5440,
    /// RELATIVE-TOLERANCE
    RelativeTolerance                                                      = 6162,
    /// RELEASE-NOTES-REF
    ReleaseNotesRef                                                        = 4538,
    /// RELEASED-TRIGGER-POLICYS
    ReleasedTriggerPolicys                                                 = 4642,
    /// RELEASED-TRIGGER-REF
    ReleasedTriggerRef                                                     = 2113,
    /// RELEASED-TRIGGERS
    ReleasedTriggers                                                       = 2143,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF
    RelevantForDynamicPncMappingRef                                        = 3585,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS
    RelevantForDynamicPncMappingRefs                                       = 799,
    /// RELIABILITY
    Reliability                                                            = 3090,
    /// RELIABILITY-KIND
    ReliabilityKind                                                        = 1696,
    /// RELIABILITY-MAX-BLOCKING-TIME
    ReliabilityMaxBlockingTime                                             = 3622,
    /// REMAPPED-CAN-ID
    RemappedCanId                                                          = 4184,
    /// REMARK
    Remark                                                                 = 4863,
    /// REMOTE-ACCESS-CONTROL-ENABLED
    RemoteAccessControlEnabled                                             = 1159,
    /// REMOTE-ADDRESSS
    RemoteAddresss                                                         = 20,
    /// REMOTE-CERTIFICATE-REF
    RemoteCertificateRef                                                   = 5865,
    /// REMOTE-CERTIFICATE-REFS
    RemoteCertificateRefs                                                  = 4955,
    /// REMOTE-CLIENT-CONFIG
    RemoteClientConfig                                                     = 4406,
    /// REMOTE-CLIENT-QUEUE-LENGTH-CONTRIBUTION
    RemoteClientQueueLengthContribution                                    = 5111,
    /// REMOTE-CONFIGS
    RemoteConfigs                                                          = 2047,
    /// REMOTE-ENDPOINT-CONFIGURATION
    RemoteEndpointConfiguration                                            = 6641,
    /// REMOTE-ID
    RemoteId                                                               = 1112,
    /// REMOTE-IP-ADDRESS-REF
    RemoteIpAddressRef                                                     = 3291,
    /// REMOTE-IP-ADDRESS-REFS
    RemoteIpAddressRefs                                                    = 2429,
    /// REMOTE-MODE-SENDER-QUEUE-LENGTH-CONTRIBUTION
    RemoteModeSenderQueueLengthContribution                                = 1714,
    /// REMOTE-MULTICAST-CONFIG-REF
    RemoteMulticastConfigRef                                               = 6546,
    /// REMOTE-MULTICAST-CONFIG-REFS
    RemoteMulticastConfigRefs                                              = 1583,
    /// REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS
    RemoteMulticastSubscriptionAddresss                                    = 2474,
    /// REMOTE-NETWORK-ENDPOINT-REF
    RemoteNetworkEndpointRef                                               = 1649,
    /// REMOTE-PORT-RANGE-END
    RemotePortRangeEnd                                                     = 5600,
    /// REMOTE-PORT-RANGE-START
    RemotePortRangeStart                                                   = 3647,
    /// REMOTE-PORT-REF
    RemotePortRef                                                          = 5910,
    /// REMOTE-SENDER-QUEUE-LENGTH-CONTRIBUTION
    RemoteSenderQueueLengthContribution                                    = 5666,
    /// REMOTE-SERVER-CONFIG
    RemoteServerConfig                                                     = 4332,
    /// REMOTE-SUBJECT-REF
    RemoteSubjectRef                                                       = 2983,
    /// REMOTE-SUBJECT-REFS
    RemoteSubjectRefs                                                      = 3224,
    /// REMOTE-UNICAST-ADDRESSS
    RemoteUnicastAddresss                                                  = 754,
    /// REMOTE-UNICAST-CONFIG-REF
    RemoteUnicastConfigRef                                                 = 6178,
    /// REMOTE-UNICAST-CONFIG-REFS
    RemoteUnicastConfigRefs                                                = 2148,
    /// REMOTING-TECHNOLOGY
    RemotingTechnology                                                     = 3528,
    /// REPETITION-PERIOD
    RepetitionPeriod                                                       = 1628,
    /// REPLACE-WITH
    ReplaceWith                                                            = 6423,
    /// REPLAY-PROTECTION
    ReplayProtection                                                       = 5955,
    /// REPLAY-PROTECTION-WINDOW
    ReplayProtectionWindow                                                 = 902,
    /// REPLICA-PDUS-REF
    ReplicaPdusRef                                                         = 2053,
    /// REPLICA-PDUS-REFS
    ReplicaPdusRefs                                                        = 690,
    /// REPLY-TOPIC-NAME
    ReplyTopicName                                                         = 2506,
    /// REPORT-BEHAVIOR
    ReportBehavior                                                         = 4073,
    /// REPORTABLE-SECURITY-EVENT-REF
    ReportableSecurityEventRef                                             = 5017,
    /// REPORTABLE-SECURITY-EVENT-REFS
    ReportableSecurityEventRefs                                            = 5460,
    /// REPORTED-SECURITY-EVENT-IREF
    ReportedSecurityEventIref                                              = 17,
    /// REPORTING-BEHAVIOR
    ReportingBehavior                                                      = 2504,
    /// REPORTING-PORT-PROTOTYPE-IREF
    ReportingPortPrototypeIref                                             = 5661,
    /// REPOSITORY
    Repository                                                             = 1669,
    /// REPRESENTED-MODEL-ELEMENT-REF
    RepresentedModelElementRef                                             = 4227,
    /// REPRESENTED-PORT-GROUP-REF
    RepresentedPortGroupRef                                                = 3631,
    /// REPRESENTS-VIN
    RepresentsVin                                                          = 5777,
    /// REQUEST-2-SUPPORT
    Request2Support                                                        = 4244,
    /// REQUEST-ARGUMENT-REF
    RequestArgumentRef                                                     = 2407,
    /// REQUEST-CONFIGURATION-DESIGNS
    RequestConfigurationDesigns                                            = 2175,
    /// REQUEST-CONFIGURATIONS
    RequestConfigurations                                                  = 856,
    /// REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF
    RequestControlOfOnBoardDeviceClassRef                                  = 2199,
    /// REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF
    RequestCurrentPowertrainDiagnosticDataClassRef                         = 5513,
    /// REQUEST-DATA-SIZE
    RequestDataSize                                                        = 19,
    /// REQUEST-DOWNLOAD-CLASS-REF
    RequestDownloadClassRef                                                = 2841,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF
    RequestEmissionRelatedDtcClassPermanentStatusRef                       = 2953,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-REF
    RequestEmissionRelatedDtcClassRef                                      = 3471,
    /// REQUEST-FILE-TRANSFER-CLASS-REF
    RequestFileTransferClassRef                                            = 2410,
    /// REQUEST-GROUP-REF
    RequestGroupRef                                                        = 2856,
    /// REQUEST-METHOD
    RequestMethod                                                          = 4675,
    /// REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF
    RequestOnBoardMonitoringTestResultsClassRef                            = 488,
    /// REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF
    RequestPowertrainFreezeFrameDataRef                                    = 3202,
    /// REQUEST-RESPONSE-DELAY
    RequestResponseDelay                                                   = 2574,
    /// REQUEST-RESULT
    RequestResult                                                          = 2666,
    /// REQUEST-SEED-ID
    RequestSeedId                                                          = 374,
    /// REQUEST-TOPIC-NAME
    RequestTopicName                                                       = 5525,
    /// REQUEST-TYPE
    RequestType                                                            = 2979,
    /// REQUEST-UPLOAD-CLASS-REF
    RequestUploadClassRef                                                  = 1190,
    /// REQUEST-VEHICLE-INFORMATION-CLASS-REF
    RequestVehicleInformationClassRef                                      = 4590,
    /// REQUESTABLE
    Requestable                                                            = 1629,
    /// REQUESTED-MODE-IREF
    RequestedModeIref                                                      = 2662,
    /// REQUESTER-IREF
    RequesterIref                                                          = 5903,
    /// REQUESTER-REF
    RequesterRef                                                           = 289,
    /// REQUESTER-REFS
    RequesterRefs                                                          = 471,
    /// REQUESTER-SOFTWARE-CLUSTER-REF
    RequesterSoftwareClusterRef                                            = 3494,
    /// REQUESTOR-REF
    RequestorRef                                                           = 5001,
    /// REQUESTOR-REFS
    RequestorRefs                                                          = 2188,
    /// REQUESTS
    Requests                                                               = 4348,
    /// REQUIRE-RESOURCES
    RequireResources                                                       = 4754,
    /// REQUIRED-AR-ELEMENT-REF
    RequiredArElementRef                                                   = 4579,
    /// REQUIRED-AR-ELEMENT-REFS
    RequiredArElementRefs                                                  = 5269,
    /// REQUIRED-ARTIFACTS
    RequiredArtifacts                                                      = 6524,
    /// REQUIRED-CLIENT-SERVER-ENTRY-REF
    RequiredClientServerEntryRef                                           = 3912,
    /// REQUIRED-CLIENT-SERVER-ENTRYS
    RequiredClientServerEntrys                                             = 1327,
    /// REQUIRED-COM-SPECS
    RequiredComSpecs                                                       = 5897,
    /// REQUIRED-DATAS
    RequiredDatas                                                          = 5976,
    /// REQUIRED-DDS-EVENT-QOS-PROPS
    RequiredDdsEventQosProps                                               = 190,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 419,
    /// REQUIRED-DESIGN-ELEMENT-REF
    RequiredDesignElementRef                                               = 2032,
    /// REQUIRED-DESIGN-ELEMENT-REFS
    RequiredDesignElementRefs                                              = 3486,
    /// REQUIRED-ENTRYS
    RequiredEntrys                                                         = 508,
    /// REQUIRED-ENVIRONMENT-REF
    RequiredEnvironmentRef                                                 = 2267,
    /// REQUIRED-EVENT-GROUPS
    RequiredEventGroups                                                    = 1811,
    /// REQUIRED-FIBEX-ELEMENT-REF
    RequiredFibexElementRef                                                = 2763,
    /// REQUIRED-FIBEX-ELEMENT-REFS
    RequiredFibexElementRefs                                               = 4433,
    /// REQUIRED-GENERATOR-TOOLS
    RequiredGeneratorTools                                                 = 543,
    /// REQUIRED-INTERFACE-TREF
    RequiredInterfaceTref                                                  = 5579,
    /// REQUIRED-MINOR-VERSION
    RequiredMinorVersion                                                   = 2213,
    /// REQUIRED-MODE-GROUP-REF
    RequiredModeGroupRef                                                   = 893,
    /// REQUIRED-MODE-GROUPS
    RequiredModeGroups                                                     = 3588,
    /// REQUIRED-OUTER-PORT-REF
    RequiredOuterPortRef                                                   = 5190,
    /// REQUIRED-PACKAGE-ELEMENT-REF
    RequiredPackageElementRef                                              = 659,
    /// REQUIRED-PACKAGE-ELEMENT-REFS
    RequiredPackageElementRefs                                             = 5752,
    /// REQUIRED-PORT-PROTOTYPE-IREF
    RequiredPortPrototypeIref                                              = 4328,
    /// REQUIRED-RTE-VENDOR
    RequiredRteVendor                                                      = 1749,
    /// REQUIRED-SERVICE-INSTANCE-ID
    RequiredServiceInstanceId                                              = 4561,
    /// REQUIRED-SERVICE-INSTANCE-REF
    RequiredServiceInstanceRef                                             = 395,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 4986,
    /// REQUIRED-SERVICE-PORT-IREF
    RequiredServicePortIref                                                = 114,
    /// REQUIRED-SERVICE-VERSION
    RequiredServiceVersion                                                 = 1755,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 4599,
    /// REQUIRED-TRIGGER-IREF
    RequiredTriggerIref                                                    = 3968,
    /// REQUIRED-TRIGGERS
    RequiredTriggers                                                       = 2454,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 1573,
    /// REQUIREMENT-REF
    RequirementRef                                                         = 3679,
    /// REQUIREMENT-REFS
    RequirementRefs                                                        = 3027,
    /// REQUIRES-INDEX
    RequiresIndex                                                          = 6248,
    /// REQUIRES-SYMBOLIC-NAME-VALUE
    RequiresSymbolicNameValue                                              = 3549,
    /// RESET-CONFIRMED-BIT-ON-OVERFLOW
    ResetConfirmedBitOnOverflow                                            = 5512,
    /// RESET-PENDING-BIT-ON-OVERFLOW
    ResetPendingBitOnOverflow                                              = 5979,
    /// RESET-TO-DEFAULT
    ResetToDefault                                                         = 108,
    /// RESET-TO-DEFAULT-SUPPORTED
    ResetToDefaultSupported                                                = 4556,
    /// RESISTANT-TO-CHANGED-SW
    ResistantToChangedSw                                                   = 3641,
    /// RESOLUTION
    Resolution                                                             = 4291,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 1865,
    /// RESOURCE-DEFINITION-REF
    ResourceDefinitionRef                                                  = 988,
    /// RESOURCE-DEFINITIONS
    ResourceDefinitions                                                    = 2066,
    /// RESOURCE-ESTIMATIONS
    ResourceEstimations                                                    = 5282,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 3399,
    /// RESOURCE-GROUP-REF
    ResourceGroupRef                                                       = 708,
    /// RESOURCE-GROUP-REFS
    ResourceGroupRefs                                                      = 5451,
    /// RESOURCE-GROUPS
    ResourceGroups                                                         = 3771,
    /// RESOURCE-GUARD-VALUE
    ResourceGuardValue                                                     = 2041,
    /// RESOURCE-IDENTIFIER-TYPE
    ResourceIdentifierType                                                 = 245,
    /// RESOURCE-LIMITS
    ResourceLimits                                                         = 4885,
    /// RESOURCE-NEEDS-REF
    ResourceNeedsRef                                                       = 4913,
    /// RESOURCE-NEEDS-REFS
    ResourceNeedsRefs                                                      = 3653,
    /// RESOURCE-REF
    ResourceRef                                                            = 1800,
    /// RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS
    ResourceToApplicationPartitionMappings                                 = 5183,
    /// RESOURCES
    Resources                                                              = 3825,
    /// RESPOND-TO-RESET
    RespondToReset                                                         = 4436,
    /// RESPONSE-ARGUMENT-REF
    ResponseArgumentRef                                                    = 2801,
    /// RESPONSE-DATA-SIZE
    ResponseDataSize                                                       = 2610,
    /// RESPONSE-ERROR-POSITION
    ResponseErrorPosition                                                  = 6398,
    /// RESPONSE-ERROR-REF
    ResponseErrorRef                                                       = 6569,
    /// RESPONSE-GROUP-REF
    ResponseGroupRef                                                       = 3087,
    /// RESPONSE-ON-ALL-REQUEST-SIDS
    ResponseOnAllRequestSids                                               = 2498,
    /// RESPONSE-ON-EVENT-ACTION
    ResponseOnEventAction                                                  = 6071,
    /// RESPONSE-ON-EVENT-CLASS-REF
    ResponseOnEventClassRef                                                = 4718,
    /// RESPONSE-ON-EVENT-REF
    ResponseOnEventRef                                                     = 1948,
    /// RESPONSE-ON-EVENT-SCHEDULER-RATE
    ResponseOnEventSchedulerRate                                           = 3634,
    /// RESPONSE-ON-SECOND-DECLINED-REQUEST
    ResponseOnSecondDeclinedRequest                                        = 1777,
    /// RESPONSE-REF
    ResponseRef                                                            = 6487,
    /// RESPONSES
    Responses                                                              = 4271,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 2203,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 2827,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 5032,
    /// REST-ENDPOINT-ARGUMENT
    RestEndpointArgument                                                   = 3245,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 6422,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 733,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 2772,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 5778,
    /// REST-ENUMERATOR-DEF
    RestEnumeratorDef                                                      = 2067,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 1836,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 6177,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 6197,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 5094,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 2893,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 30,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 1558,
    /// REST-SYSTEM-TRIGGERED-EVENT
    RestSystemTriggeredEvent                                               = 264,
    /// RESTORE-AT-START
    RestoreAtStart                                                         = 2672,
    /// RESTRICT-UPDATE
    RestrictUpdate                                                         = 6009,
    /// RESTRICTION
    Restriction                                                            = 2405,
    /// RESTRICTIONS
    Restrictions                                                           = 1816,
    /// RESULTING-PROPERTIES
    ResultingProperties                                                    = 1537,
    /// RESULTING-RPT-SW-PROTOTYPING-ACCESS
    ResultingRptSwPrototypingAccess                                        = 5899,
    /// RESUME-POSITION
    ResumePosition                                                         = 2985,
    /// RETRY
    Retry                                                                  = 4025,
    /// RETRY-INTERVAL-TIME
    RetryIntervalTime                                                      = 157,
    /// RETURN-CODE
    ReturnCode                                                             = 624,
    /// RETURN-NO-NEW-DATA-ENABLED
    ReturnNoNewDataEnabled                                                 = 6121,
    /// RETURN-SIGNAL-REF
    ReturnSignalRef                                                        = 6380,
    /// RETURN-SIGNAL-TRIGGERING-REF
    ReturnSignalTriggeringRef                                              = 5972,
    /// RETURN-TYPE
    ReturnType                                                             = 1165,
    /// RETURN-VALUE-PROVISION
    ReturnValueProvision                                                   = 1150,
    /// REVERSED-TP-SDU-REF
    ReversedTpSduRef                                                       = 1132,
    /// REVISION-LABEL
    RevisionLabel                                                          = 6185,
    /// REVISION-LABEL-P-1
    RevisionLabelP1                                                        = 4991,
    /// REVISION-LABEL-P-2
    RevisionLabelP2                                                        = 3880,
    /// REVISION-LABELS
    RevisionLabels                                                         = 5930,
    /// RID-NUMBER
    RidNumber                                                              = 525,
    /// ROLE
    Role                                                                   = 4765,
    /// ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT
    RoleBasedBswModuleEntryAssignment                                      = 2434,
    /// ROLE-BASED-DATA-ASSIGNMENT
    RoleBasedDataAssignment                                                = 3935,
    /// ROLE-BASED-DATA-TYPE-ASSIGNMENT
    RoleBasedDataTypeAssignment                                            = 4789,
    /// ROLE-BASED-MC-DATA-ASSIGNMENT
    RoleBasedMcDataAssignment                                              = 2741,
    /// ROLE-BASED-PORT-ASSIGNMENT
    RoleBasedPortAssignment                                                = 1839,
    /// ROLE-BASED-RESOURCE-DEPENDENCY
    RoleBasedResourceDependency                                            = 4994,
    /// ROLLOUT-QUALIFICATIONS
    RolloutQualifications                                                  = 5610,
    /// ROM-BLOCK
    RomBlock                                                               = 2640,
    /// ROM-BLOCK-INIT-VALUE
    RomBlockInitValue                                                      = 765,
    /// ROM-LOCATION-REF
    RomLocationRef                                                         = 6310,
    /// ROOT-ARGUMENT-DATA-PROTOTYPE-REF
    RootArgumentDataPrototypeRef                                           = 6093,
    /// ROOT-AUTOSAR-DATA-PROTOTYPE-REF
    RootAutosarDataPrototypeRef                                            = 741,
    /// ROOT-COMPOSITION
    RootComposition                                                        = 4720,
    /// ROOT-CONTEXT-REF
    RootContextRef                                                         = 2448,
    /// ROOT-DATA-PROTOTYPE-IN-CS-REF
    RootDataPrototypeInCsRef                                               = 5452,
    /// ROOT-DATA-PROTOTYPE-IN-SR-REF
    RootDataPrototypeInSrRef                                               = 5386,
    /// ROOT-DATA-PROTOTYPE-REF
    RootDataPrototypeRef                                                   = 2837,
    /// ROOT-PARAMETER-DATA-PROTOTYPE-REF
    RootParameterDataPrototypeRef                                          = 5106,
    /// ROOT-REF
    RootRef                                                                = 2018,
    /// ROOT-SOFTWARE-COMPOSITIONS
    RootSoftwareCompositions                                               = 4130,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 4838,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 1670,
    /// ROOT-VARIABLE-DATA-PROTOTYPE-REF
    RootVariableDataPrototypeRef                                           = 2010,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 5431,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 4851,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 2006,
    /// ROUTINE-CONTROL-CLASS-REF
    RoutineControlClassRef                                                 = 4485,
    /// ROUTINE-INFO
    RoutineInfo                                                            = 3809,
    /// ROUTINE-REF
    RoutineRef                                                             = 6182,
    /// ROUTINE-SUBFUNCTION-REF
    RoutineSubfunctionRef                                                  = 5943,
    /// ROUTING-ACTIVATION-TYPE
    RoutingActivationType                                                  = 4770,
    /// ROUTING-GROUP-REF
    RoutingGroupRef                                                        = 0,
    /// ROUTING-GROUP-REFS
    RoutingGroupRefs                                                       = 541,
    /// ROW
    Row                                                                    = 1913,
    /// RP-IMPL-POLICY
    RpImplPolicy                                                           = 705,
    /// RPT-AR-HOOK-IREF
    RptArHookIref                                                          = 1391,
    /// RPT-COMPONENT
    RptComponent                                                           = 6598,
    /// RPT-COMPONENTS
    RptComponents                                                          = 1681,
    /// RPT-CONTAINER
    RptContainer                                                           = 5582,
    /// RPT-CONTAINERS
    RptContainers                                                          = 1470,
    /// RPT-ENABLER-IMPL-TYPE
    RptEnablerImplType                                                     = 946,
    /// RPT-EVENT-ID
    RptEventId                                                             = 3362,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 1578,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 3203,
    /// RPT-EXECUTABLE-ENTITY-EVENTS
    RptExecutableEntityEvents                                              = 5531,
    /// RPT-EXECUTABLE-ENTITY-PROPERTIES
    RptExecutableEntityProperties                                          = 1890,
    /// RPT-EXECUTABLE-ENTITYS
    RptExecutableEntitys                                                   = 5784,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 5678,
    /// RPT-EXECUTION-CONTROL
    RptExecutionControl                                                    = 2269,
    /// RPT-HOOK
    RptHook                                                                = 744,
    /// RPT-HOOK-ACCESS
    RptHookAccess                                                          = 5665,
    /// RPT-HOOKS
    RptHooks                                                               = 3431,
    /// RPT-IMPL-POLICY
    RptImplPolicy                                                          = 1405,
    /// RPT-PREPARATION-LEVEL
    RptPreparationLevel                                                    = 2017,
    /// RPT-PROFILE
    RptProfile                                                             = 1630,
    /// RPT-PROFILES
    RptProfiles                                                            = 5921,
    /// RPT-READ-ACCESS
    RptReadAccess                                                          = 5992,
    /// RPT-READS
    RptReads                                                               = 3923,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 5168,
    /// RPT-SERVICE-POINT-POST-REF
    RptServicePointPostRef                                                 = 4118,
    /// RPT-SERVICE-POINT-POST-REFS
    RptServicePointPostRefs                                                = 2864,
    /// RPT-SERVICE-POINT-PRE-REF
    RptServicePointPreRef                                                  = 2132,
    /// RPT-SERVICE-POINT-PRE-REFS
    RptServicePointPreRefs                                                 = 6528,
    /// RPT-SERVICE-POINTS
    RptServicePoints                                                       = 2170,
    /// RPT-SUPPORT-DATA
    RptSupportData                                                         = 5272,
    /// RPT-SW-PROTOTYPING-ACCESS
    RptSwPrototypingAccess                                                 = 3670,
    /// RPT-SYSTEM-REF
    RptSystemRef                                                           = 6234,
    /// RPT-WRITE-ACCESS
    RptWriteAccess                                                         = 1551,
    /// RPT-WRITES
    RptWrites                                                              = 6635,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 77,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 6059,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 2582,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 4033,
    /// RTE-EVENT-IREF
    RteEventIref                                                           = 2362,
    /// RTE-EVENT-IREFS
    RteEventIrefs                                                          = 5691,
    /// RTE-EVENT-SEPARATIONS
    RteEventSeparations                                                    = 522,
    /// RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS
    RteEventToOsTaskProxyMappings                                          = 3102,
    /// RTE-PLUGIN-PROPS
    RtePluginProps                                                         = 1247,
    /// RTE-RESOURCE-ESTIMATION
    RteResourceEstimation                                                  = 1041,
    /// RTP-TP
    RtpTp                                                                  = 4096,
    /// RTPS-PROTECTION-KIND
    RtpsProtectionKind                                                     = 6408,
    /// RULE
    Rule                                                                   = 6020,
    /// RULE-ARGUMENTS
    RuleArguments                                                          = 3476,
    /// RULE-BASED-AXIS-CONT
    RuleBasedAxisCont                                                      = 6289,
    /// RULE-BASED-VALUES
    RuleBasedValues                                                        = 1211,
    /// RULE-INIT-STATE
    RuleInitState                                                          = 6420,
    /// RULES
    Rules                                                                  = 189,
    /// RUN-MODE
    RunMode                                                                = 5747,
    /// RUN-RESOURCE
    RunResource                                                            = 3575,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 2295,
    /// RUNNABLE-ENTITY-ARGUMENT
    RunnableEntityArgument                                                 = 736,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 5699,
    /// RUNNABLE-ENTITY-GROUP-IREF
    RunnableEntityGroupIref                                                = 1939,
    /// RUNNABLE-ENTITY-GROUP-IREFS
    RunnableEntityGroupIrefs                                               = 3130,
    /// RUNNABLE-ENTITY-IREF
    RunnableEntityIref                                                     = 2339,
    /// RUNNABLE-ENTITY-IREFS
    RunnableEntityIrefs                                                    = 5801,
    /// RUNNABLE-MAPPINGS
    RunnableMappings                                                       = 2337,
    /// RUNNABLE-REF
    RunnableRef                                                            = 3342,
    /// RUNNABLES
    Runnables                                                              = 4711,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REF
    RunsInsideExclusiveAreaRef                                             = 4053,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REFS
    RunsInsideExclusiveAreaRefs                                            = 5782,
    /// RUNS-INSIDES
    RunsInsides                                                            = 4705,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 1782,
    /// RUNTIME-IP-ADDRESS-CONFIGURATION
    RuntimeIpAddressConfiguration                                          = 2908,
    /// RUNTIME-PORT-CONFIGURATION
    RuntimePortConfiguration                                               = 4102,
    /// RVF-ACTIVE-PIXELS
    RvfActivePixels                                                        = 313,
    /// RVF-COLOR-SPACE
    RvfColorSpace                                                          = 4558,
    /// RVF-EVENT-DEFAULT
    RvfEventDefault                                                        = 3457,
    /// RVF-FRAME-RATE
    RvfFrameRate                                                           = 2309,
    /// RVF-INTERLACED
    RvfInterlaced                                                          = 2157,
    /// RVF-PIXEL-DEPTH
    RvfPixelDepth                                                          = 4351,
    /// RVF-PIXEL-FORMAT
    RvfPixelFormat                                                         = 2711,
    /// RVF-TOTAL-LINES
    RvfTotalLines                                                          = 6615,
    /// RX-ACCEPT-CONTAINED-I-PDU
    RxAcceptContainedIPdu                                                  = 1324,
    /// RX-CYCLE-PROCESS-TIME
    RxCycleProcessTime                                                     = 6503,
    /// RX-IDENTIFIER-RANGE
    RxIdentifierRange                                                      = 24,
    /// RX-MASK
    RxMask                                                                 = 4477,
    /// RX-NM-PDU-REF
    RxNmPduRef                                                             = 6370,
    /// RX-NM-PDU-REFS
    RxNmPduRefs                                                            = 5765,
    /// RX-PDU-POOL-REF
    RxPduPoolRef                                                           = 5020,
    /// RX-PDU-TRIGGERING-REF
    RxPduTriggeringRef                                                     = 2522,
    /// RX-SECURITY-VERIFICATION
    RxSecurityVerification                                                 = 384,
    /// RX-TIMEOUT-TIME
    RxTimeoutTime                                                          = 3449,
    /// S-3-SERVER-TIMEOUT
    S3ServerTimeout                                                        = 1186,
    /// SA-OVER-TIME
    SaOverTime                                                             = 3459,
    /// SA-RAND-TIME
    SaRandTime                                                             = 6612,
    /// SA-REKEY-TIME
    SaRekeyTime                                                            = 2750,
    /// SAFE-TRANSLATION
    SafeTranslation                                                        = 5981,
    /// SAFETY-CONDITION
    SafetyCondition                                                        = 4799,
    /// SAFETY-CONDITIONS
    SafetyConditions                                                       = 1226,
    /// SAFETY-MARGIN
    SafetyMargin                                                           = 2761,
    /// SAFETY-POLICY
    SafetyPolicy                                                           = 3228,
    /// SAK-REF
    SakRef                                                                 = 2208,
    /// SAK-REKEY-TIME-SPAN
    SakRekeyTimeSpan                                                       = 3746,
    /// SAMPLE-CLOCK-PERIOD
    SampleClockPeriod                                                      = 3786,
    /// SAMPLES-PER-MICROTICK
    SamplesPerMicrotick                                                    = 1056,
    /// SAVE-CONFIGURATION
    SaveConfiguration                                                      = 2859,
    /// SAVE-CONFIGURATION-ENTRY
    SaveConfigurationEntry                                                 = 2936,
    /// SCALE-CONSTR
    ScaleConstr                                                            = 4707,
    /// SCALE-CONSTRS
    ScaleConstrs                                                           = 5735,
    /// SCALING-INFO-SIZE
    ScalingInfoSize                                                        = 2660,
    /// SCHEDULE-CHANGE-NEXT-TIME-BASE
    ScheduleChangeNextTimeBase                                             = 6516,
    /// SCHEDULE-TABLES
    ScheduleTables                                                         = 4573,
    /// SCHEDULER-MAX-NUMBER
    SchedulerMaxNumber                                                     = 2447,
    /// SCHEDULER-NAME-PREFIX-REF
    SchedulerNamePrefixRef                                                 = 949,
    /// SCHEDULER-NAME-PREFIXS
    SchedulerNamePrefixs                                                   = 4666,
    /// SCHEDULING-POLICY
    SchedulingPolicy                                                       = 858,
    /// SCHEDULING-PRIORITY
    SchedulingPriority                                                     = 1429,
    /// SCOPE
    Scope                                                                  = 3098,
    /// SCOPE-EVENT-REF
    ScopeEventRef                                                          = 4149,
    /// SCOPE-EVENT-REFS
    ScopeEventRefs                                                         = 4924,
    /// SCOPE-REF
    ScopeRef                                                               = 3439,
    /// SCOPE-REFS
    ScopeRefs                                                              = 774,
    /// SD
    Sd                                                                     = 5615,
    /// SD-CLIENT-CONFIG
    SdClientConfig                                                         = 2930,
    /// SD-CLIENT-CONFIG-REF
    SdClientConfigRef                                                      = 6191,
    /// SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SdClientEventGroupTimingConfigRef                                      = 865,
    /// SD-CLIENT-EVENT-TIMING-CONFIG
    SdClientEventTimingConfig                                              = 1525,
    /// SD-CLIENT-TIMER-CONFIGS
    SdClientTimerConfigs                                                   = 1758,
    /// SD-SERVER-CONFIG
    SdServerConfig                                                         = 2118,
    /// SD-SERVER-CONFIG-REF
    SdServerConfigRef                                                      = 3843,
    /// SD-SERVER-EG-TIMING-CONFIGS
    SdServerEgTimingConfigs                                                = 682,
    /// SD-SERVER-EVENT-CONFIG
    SdServerEventConfig                                                    = 3862,
    /// SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SdServerEventGroupTimingConfigRef                                      = 4280,
    /// SD-SERVER-TIMER-CONFIGS
    SdServerTimerConfigs                                                   = 3309,
    /// SDF
    Sdf                                                                    = 3560,
    /// SDG
    Sdg                                                                    = 2724,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 2536,
    /// SDG-CAPTION
    SdgCaption                                                             = 4287,
    /// SDG-CAPTION-REF
    SdgCaptionRef                                                          = 5023,
    /// SDG-CLASS
    SdgClass                                                               = 3299,
    /// SDG-CLASS-REF
    SdgClassRef                                                            = 5837,
    /// SDG-CLASSES
    SdgClasses                                                             = 3615,
    /// SDG-CONSTRAINT-REF
    SdgConstraintRef                                                       = 1068,
    /// SDG-CONSTRAINT-REFS
    SdgConstraintRefs                                                      = 976,
    /// SDG-DEF
    SdgDef                                                                 = 3378,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 4283,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 1791,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 2316,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 6021,
    /// SDG-REFERENCE
    SdgReference                                                           = 5125,
    /// SDG-TAILORING
    SdgTailoring                                                           = 1851,
    /// SDG-TAILORINGS
    SdgTailorings                                                          = 1999,
    /// SDGS
    Sdgs                                                                   = 4313,
    /// SDU-REF
    SduRef                                                                 = 2850,
    /// SDU-REFS
    SduRefs                                                                = 5794,
    /// SDU-TYPE
    SduType                                                                = 2561,
    /// SDX-REF
    SdxRef                                                                 = 4034,
    /// SDXF
    Sdxf                                                                   = 2812,
    /// SEARCH-BEHAVIOR
    SearchBehavior                                                         = 3157,
    /// SEARCH-INTENTION
    SearchIntention                                                        = 3193,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REF
    SecOcComPropsForMulticastRef                                           = 5657,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REFS
    SecOcComPropsForMulticastRefs                                          = 225,
    /// SEC-OC-CRYPTO-MAPPING-REF
    SecOcCryptoMappingRef                                                  = 4920,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 3829,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING-REF
    SecOcCryptoServiceMappingRef                                           = 5532,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING-REF-CONDITIONAL
    SecOcCryptoServiceMappingRefConditional                                = 187,
    /// SEC-OC-CRYPTO-SERVICE-MAPPINGS
    SecOcCryptoServiceMappings                                             = 852,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 1033,
    /// SEC-OC-IGNORE-VERIFICATION-RESULT
    SecOcIgnoreVerificationResult                                          = 1169,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 6623,
    /// SEC-OC-JOB-MAPPINGS
    SecOcJobMappings                                                       = 2345,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 292,
    /// SEC-OC-JOB-REQUIREMENT-REF
    SecOcJobRequirementRef                                                 = 6147,
    /// SEC-OC-JOB-SEMANTIC
    SecOcJobSemantic                                                       = 5211,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 4692,
    /// SEC-OC-SECURE-COM-PROPS-REF
    SecOcSecureComPropsRef                                                 = 80,
    /// SEC-OC-SECURE-COM-PROPS-REF-CONDITIONAL
    SecOcSecureComPropsRefConditional                                      = 267,
    /// SECOND-APPLICATION-ERROR-REF
    SecondApplicationErrorRef                                              = 669,
    /// SECOND-DATA-PROTOTYPE-REF
    SecondDataPrototypeRef                                                 = 5889,
    /// SECOND-ELEMENT-INSTANCE-IREF
    SecondElementInstanceIref                                              = 784,
    /// SECOND-ELEMENT-INSTANCE-IREFS
    SecondElementInstanceIrefs                                             = 815,
    /// SECOND-ELEMENT-REF
    SecondElementRef                                                       = 3737,
    /// SECOND-ELEMENT-REFS
    SecondElementRefs                                                      = 1023,
    /// SECOND-ELEMENTS
    SecondElements                                                         = 2465,
    /// SECOND-KEY-SLOT-ID
    SecondKeySlotId                                                        = 6405,
    /// SECOND-MODE-GROUP-REF
    SecondModeGroupRef                                                     = 3875,
    /// SECOND-MODE-REF
    SecondModeRef                                                          = 2051,
    /// SECOND-OPERATION-REF
    SecondOperationRef                                                     = 1397,
    /// SECOND-PHYSICAL-DIMENSION-REF
    SecondPhysicalDimensionRef                                             = 447,
    /// SECOND-PORT-REF
    SecondPortRef                                                          = 5217,
    /// SECOND-TO-FIRST-DATA-TRANSFORMATION-REF
    SecondToFirstDataTransformationRef                                     = 2409,
    /// SECOND-TRIGGER-REF
    SecondTriggerRef                                                       = 6556,
    /// SECOND-VALUE
    SecondValue                                                            = 6104,
    /// SECONDARY-DOWNLOAD-URIS
    SecondaryDownloadUris                                                  = 4957,
    /// SECONDARY-FRESHNESS-VALUE-ID
    SecondaryFreshnessValueId                                              = 5283,
    /// SECTION-INITIALIZATION-POLICY
    SectionInitializationPolicy                                            = 415,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 3335,
    /// SECTION-NAME-PREFIXS
    SectionNamePrefixs                                                     = 1491,
    /// SECTION-TYPE
    SectionType                                                            = 2161,
    /// SECURE-COM-CONFIG-REF
    SecureComConfigRef                                                     = 1792,
    /// SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING
    SecureComConfigToCryptoKeySlotMapping                                  = 925,
    /// SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS
    SecureComConfigToKeySlotMappings                                       = 1188,
    /// SECURE-COM-CONFIGS
    SecureComConfigs                                                       = 4164,
    /// SECURE-COM-PROPS-FOR-DDS-REF
    SecureComPropsForDdsRef                                                = 3760,
    /// SECURE-COM-PROPS-FOR-TCP-REF
    SecureComPropsForTcpRef                                                = 5469,
    /// SECURE-COM-PROPS-FOR-TCP-REFS
    SecureComPropsForTcpRefs                                               = 3826,
    /// SECURE-COM-PROPS-FOR-UDP-REF
    SecureComPropsForUdpRef                                                = 4620,
    /// SECURE-COM-PROPS-FOR-UDP-REFS
    SecureComPropsForUdpRefs                                               = 1640,
    /// SECURE-COM-PROPS-REF
    SecureComPropsRef                                                      = 1382,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 4646,
    /// SECURE-COM-PROPSS
    SecureComPropss                                                        = 1285,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 5591,
    /// SECURE-COMMUNICATION-DEPLOYMENTS
    SecureCommunicationDeployments                                         = 3685,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 4545,
    /// SECURE-COMMUNICATION-PROPS
    SecureCommunicationProps                                               = 1520,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 2073,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 6577,
    /// SECURE-ONBOARD-COM-ERROR-REF
    SecureOnboardComErrorRef                                               = 1252,
    /// SECURE-PROPS-FOR-EXTERNAL-COMM-REF
    SecurePropsForExternalCommRef                                          = 1945,
    /// SECURE-PROPS-FOR-TCP-REF
    SecurePropsForTcpRef                                                   = 3266,
    /// SECURE-PROPS-FOR-UDP-REF
    SecurePropsForUdpRef                                                   = 5439,
    /// SECURE-TRANSLATION
    SecureTranslation                                                      = 6296,
    /// SECURED-AREA-LENGTH
    SecuredAreaLength                                                      = 3146,
    /// SECURED-AREA-OFFSET
    SecuredAreaOffset                                                      = 3315,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 5474,
    /// SECURED-RX-VERIFICATION
    SecuredRxVerification                                                  = 4002,
    /// SECURITY-ACCESS-CLASS-REF
    SecurityAccessClassRef                                                 = 6303,
    /// SECURITY-ACCESS-LEVEL
    SecurityAccessLevel                                                    = 4662,
    /// SECURITY-DELAY-TIME
    SecurityDelayTime                                                      = 3404,
    /// SECURITY-DELAY-TIME-ON-BOOT
    SecurityDelayTimeOnBoot                                                = 2376,
    /// SECURITY-EVENT-CONTEXT-DATA
    SecurityEventContextData                                               = 1907,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITION
    SecurityEventContextDataDefinition                                     = 3692,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF
    SecurityEventContextDataDefinitionRef                                  = 6110,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF-CONDITIONAL
    SecurityEventContextDataDefinitionRefConditional                       = 73,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITIONS
    SecurityEventContextDataDefinitions                                    = 843,
    /// SECURITY-EVENT-CONTEXT-DATA-ELEMENT
    SecurityEventContextDataElement                                        = 2813,
    /// SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF
    SecurityEventContextDataElementRef                                     = 6555,
    /// SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF-CONDITIONAL
    SecurityEventContextDataElementRefConditional                          = 4888,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 4516,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 4104,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 5148,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 2166,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 3545,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 3893,
    /// SECURITY-EVENT-DEFINITION-REF
    SecurityEventDefinitionRef                                             = 5856,
    /// SECURITY-EVENT-DEFINITION-REF-CONDITIONAL
    SecurityEventDefinitionRefConditional                                  = 5192,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 6633,
    /// SECURITY-EVENT-FILTER-CHAIN-REF
    SecurityEventFilterChainRef                                            = 1539,
    /// SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL
    SecurityEventFilterChainRefConditional                                 = 3988,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 2562,
    /// SECURITY-EVENT-PROPS-REF
    SecurityEventPropsRef                                                  = 5285,
    /// SECURITY-EVENT-REF
    SecurityEventRef                                                       = 6414,
    /// SECURITY-EVENT-REFS
    SecurityEventRefs                                                      = 5476,
    /// SECURITY-EVENT-REPORT-INSTANCE-DEFINITION
    SecurityEventReportInstanceDefinition                                  = 2710,
    /// SECURITY-EVENT-REPORT-INSTANCE-DEFINITION-REF
    SecurityEventReportInstanceDefinitionRef                               = 5399,
    /// SECURITY-EVENT-REPORT-INSTANCE-VALUE
    SecurityEventReportInstanceValue                                       = 2444,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 3000,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 147,
    /// SECURITY-EVENTS
    SecurityEvents                                                         = 4325,
    /// SECURITY-LEVEL-REF
    SecurityLevelRef                                                       = 3897,
    /// SECURITY-LEVEL-REFS
    SecurityLevelRefs                                                      = 5265,
    /// SEED-SIZE
    SeedSize                                                               = 3128,
    /// SEGMENT-BYTE-ORDER
    SegmentByteOrder                                                       = 5695,
    /// SEGMENT-LENGTH
    SegmentLength                                                          = 5357,
    /// SEGMENT-POSITION
    SegmentPosition                                                        = 4605,
    /// SEGMENT-POSITIONS
    SegmentPositions                                                       = 6247,
    /// SEGMENT-RECEPTION-TIMEOUT-TIME
    SegmentReceptionTimeoutTime                                            = 2356,
    /// SEGMENT-RECEPTION-TIMEOUT-TIME-REQUEST
    SegmentReceptionTimeoutTimeRequest                                     = 1709,
    /// SEGMENT-RECEPTION-TIMEOUT-TIME-RESPONSE
    SegmentReceptionTimeoutTimeResponse                                    = 4706,
    /// SEGMENT-REF
    SegmentRef                                                             = 2349,
    /// SEGMENT-REFS
    SegmentRefs                                                            = 4746,
    /// SEGMENTATION-SUPPORTED
    SegmentationSupported                                                  = 6515,
    /// SELECT-BLOCK-FOR-FIRST-INIT-ALL
    SelectBlockForFirstInitAll                                             = 4950,
    /// SELECTIONS
    Selections                                                             = 312,
    /// SELECTOR-FIELD-BYTE-ORDER
    SelectorFieldByteOrder                                                 = 6631,
    /// SELECTOR-FIELD-CODE
    SelectorFieldCode                                                      = 5879,
    /// SELECTOR-FIELD-LENGTH
    SelectorFieldLength                                                    = 1840,
    /// SELECTOR-FIELD-START-POSITION
    SelectorFieldStartPosition                                             = 2263,
    /// SEND-ACTIVITY
    SendActivity                                                           = 4364,
    /// SEND-INDICATION
    SendIndication                                                         = 1216,
    /// SEND-POLICYS
    SendPolicys                                                            = 2399,
    /// SEND-RESP-PEND-ON-TRANS-TO-BOOT
    SendRespPendOnTransToBoot                                              = 2276,
    /// SENDER-ANNOTATION
    SenderAnnotation                                                       = 2479,
    /// SENDER-CAPABILITY
    SenderCapability                                                       = 3149,
    /// SENDER-INTENT
    SenderIntent                                                           = 900,
    /// SENDER-IREF
    SenderIref                                                             = 3058,
    /// SENDER-REC-ARRAY-ELEMENT-MAPPING
    SenderRecArrayElementMapping                                           = 2770,
    /// SENDER-REC-ARRAY-TYPE-MAPPING
    SenderRecArrayTypeMapping                                              = 2821,
    /// SENDER-REC-RECORD-ELEMENT-MAPPING
    SenderRecRecordElementMapping                                          = 4224,
    /// SENDER-REC-RECORD-TYPE-MAPPING
    SenderRecRecordTypeMapping                                             = 1326,
    /// SENDER-RECEIVER-ANNOTATIONS
    SenderReceiverAnnotations                                              = 5252,
    /// SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING
    SenderReceiverCompositeElementToSignalMapping                          = 6451,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 5586,
    /// SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING
    SenderReceiverToSignalGroupMapping                                     = 2781,
    /// SENDER-RECEIVER-TO-SIGNAL-MAPPING
    SenderReceiverToSignalMapping                                          = 4631,
    /// SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING
    SenderToSignalTextTableMapping                                         = 4867,
    /// SENDING-NODE-REF
    SendingNodeRef                                                         = 6070,
    /// SENDING-NODE-REFS
    SendingNodeRefs                                                        = 3618,
    /// SENSOR-ACTUATOR-REF
    SensorActuatorRef                                                      = 49,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 477,
    /// SENSOR-INSTANCE-ID
    SensorInstanceId                                                       = 2207,
    /// SEPARATE-SIGNAL-PATH
    SeparateSignalPath                                                     = 6225,
    /// SEPARATED-COMPONENT-IREF
    SeparatedComponentIref                                                 = 4009,
    /// SEPARATED-COMPONENT-IREFS
    SeparatedComponentIrefs                                                = 2421,
    /// SEPARATION-CYCLE-EXPONENT
    SeparationCycleExponent                                                = 4849,
    /// SEPARATION-TIME
    SeparationTime                                                         = 4985,
    /// SEPARATION-TIME-REQUEST
    SeparationTimeRequest                                                  = 1241,
    /// SEPARATION-TIME-RESPONSE
    SeparationTimeResponse                                                 = 130,
    /// SEQUENCE-COUNTER
    SequenceCounter                                                        = 2905,
    /// SEQUENCE-COUNTER-JUMP-WIDTH
    SequenceCounterJumpWidth                                               = 311,
    /// SEQUENCE-OFFSET
    SequenceOffset                                                         = 6497,
    /// SEQUENTIAL-INSTRUCTIONS-BEGIN
    SequentialInstructionsBegin                                            = 5609,
    /// SEQUENTIAL-INSTRUCTIONS-END
    SequentialInstructionsEnd                                              = 3911,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 805,
    /// SERIALIZATION-TECHNOLOGY-REF
    SerializationTechnologyRef                                             = 2914,
    /// SERIALIZER
    Serializer                                                             = 3052,
    /// SERIALIZER-REF
    SerializerRef                                                          = 4616,
    /// SERVER-ARGUMENT-IMPL-POLICY
    ServerArgumentImplPolicy                                               = 2242,
    /// SERVER-CALL-POINTS
    ServerCallPoints                                                       = 466,
    /// SERVER-COM-SPEC
    ServerComSpec                                                          = 3535,
    /// SERVER-COM-SPEC-PROPS
    ServerComSpecProps                                                     = 2015,
    /// SERVER-COM-SPEC-PROPSS
    ServerComSpecPropss                                                    = 3640,
    /// SERVER-NAME-IDENTIFICATION
    ServerNameIdentification                                               = 2016,
    /// SERVER-PORT-REF
    ServerPortRef                                                          = 4738,
    /// SERVER-SERVICE-MAJOR-VERSION
    ServerServiceMajorVersion                                              = 5606,
    /// SERVER-SERVICE-MINOR-VERSION
    ServerServiceMinorVersion                                              = 2056,
    /// SERVICE-BUSY
    ServiceBusy                                                            = 1572,
    /// SERVICE-CONTROL
    ServiceControl                                                         = 1674,
    /// SERVICE-DEPENDENCYS
    ServiceDependencys                                                     = 5913,
    /// SERVICE-DEPLOYMENT-REF
    ServiceDeploymentRef                                                   = 4284,
    /// SERVICE-DISCOVER-CONFIGS
    ServiceDiscoverConfigs                                                 = 1605,
    /// SERVICE-DISCOVERY-CONFIGS
    ServiceDiscoveryConfigs                                                = 6282,
    /// SERVICE-ELEMENT-MAPPING-REF
    ServiceElementMappingRef                                               = 4276,
    /// SERVICE-ELEMENT-MAPPING-REFS
    ServiceElementMappingRefs                                              = 6610,
    /// SERVICE-ELEMENT-SECURE-COM-CONFIG-REF
    ServiceElementSecureComConfigRef                                       = 2082,
    /// SERVICE-FIND-TIME-TO-LIVE
    ServiceFindTimeToLive                                                  = 4206,
    /// SERVICE-ID
    ServiceId                                                              = 290,
    /// SERVICE-IDENTIFIER
    ServiceIdentifier                                                      = 3460,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 5464,
    /// SERVICE-INSTANCE-ID
    ServiceInstanceId                                                      = 2674,
    /// SERVICE-INSTANCE-PORT-CONFIG
    ServiceInstancePortConfig                                              = 2839,
    /// SERVICE-INSTANCE-REF
    ServiceInstanceRef                                                     = 1073,
    /// SERVICE-INSTANCE-REFS
    ServiceInstanceRefs                                                    = 5182,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 998,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 1665,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF
    ServiceInstanceToPortPrototypeMappingRef                               = 1293,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 3209,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 5318,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS
    ServiceInstanceToSignalMappings                                        = 4172,
    /// SERVICE-INSTANCES
    ServiceInstances                                                       = 3240,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 2468,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 1753,
    /// SERVICE-INTERFACE-DEPLOYMENT-REF
    ServiceInterfaceDeploymentRef                                          = 5369,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REF
    ServiceInterfaceElementMappingRef                                      = 3031,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REFS
    ServiceInterfaceElementMappingRefs                                     = 4314,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 501,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 1774,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 3174,
    /// SERVICE-INTERFACE-ID
    ServiceInterfaceId                                                     = 1445,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 1606,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 4461,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 4376,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 5885,
    /// SERVICE-INTERFACE-REF
    ServiceInterfaceRef                                                    = 5362,
    /// SERVICE-INTERFACE-REFS
    ServiceInterfaceRefs                                                   = 3004,
    /// SERVICE-INTERFACE-SUB-ELEMENT
    ServiceInterfaceSubElement                                             = 1820,
    /// SERVICE-INTERFACE-TREF
    ServiceInterfaceTref                                                   = 4483,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 1981,
    /// SERVICE-INTERFACE-VERSION
    ServiceInterfaceVersion                                                = 6244,
    /// SERVICE-ITEMS
    ServiceItems                                                           = 2924,
    /// SERVICE-KIND
    ServiceKind                                                            = 6447,
    /// SERVICE-MINOR-VERSION
    ServiceMinorVersion                                                    = 4623,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 408,
    /// SERVICE-OFFER-TIME-TO-LIVE
    ServiceOfferTimeToLive                                                 = 811,
    /// SERVICE-POINT-SYMBOL-POST
    ServicePointSymbolPost                                                 = 2773,
    /// SERVICE-POINT-SYMBOL-PRE
    ServicePointSymbolPre                                                  = 5884,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 4369,
    /// SERVICE-REQUEST-CALLBACK-TYPE
    ServiceRequestCallbackType                                             = 5515,
    /// SERVICE-RESOURCE-REF
    ServiceResourceRef                                                     = 5194,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 2321,
    /// SERVICE-TABLES
    ServiceTables                                                          = 362,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 1475,
    /// SESSION-CONTROL-CLASS-REF
    SessionControlClassRef                                                 = 4170,
    /// SESSION-HANDLING
    SessionHandling                                                        = 6616,
    /// SESSION-HANDLING-SR
    SessionHandlingSr                                                      = 1947,
    /// SESSION-ID
    SessionId                                                              = 4097,
    /// SESSION-ID-SUPPORT
    SessionIdSupport                                                       = 2543,
    /// SET
    Set                                                                    = 547,
    /// SET-FUNCTION-GROUP-STATE-IREF
    SetFunctionGroupStateIref                                              = 5145,
    /// SETTER-CALL-REF
    SetterCallRef                                                          = 4530,
    /// SETTER-CALL-SIGNAL-REF
    SetterCallSignalRef                                                    = 2134,
    /// SETTER-OPERATION-REF
    SetterOperationRef                                                     = 2334,
    /// SETTER-REF
    SetterRef                                                              = 3386,
    /// SETTER-RETURN-REF
    SetterReturnRef                                                        = 3261,
    /// SETTER-RETURN-SIGNAL-REF
    SetterReturnSignalRef                                                  = 5271,
    /// SEVERITY
    Severity                                                               = 259,
    /// SHALL-NOT-RUN-ON-REF
    ShallNotRunOnRef                                                       = 1022,
    /// SHALL-NOT-RUN-ON-REFS
    ShallNotRunOnRefs                                                      = 4163,
    /// SHALL-RUN-ON-REF
    ShallRunOnRef                                                          = 3949,
    /// SHALL-RUN-ON-REFS
    ShallRunOnRefs                                                         = 1407,
    /// SHAPER
    Shaper                                                                 = 425,
    /// SHARED
    Shared                                                                 = 5553,
    /// SHARED-AXIS-TYPE-REF
    SharedAxisTypeRef                                                      = 5697,
    /// SHARED-PARAMETERS
    SharedParameters                                                       = 972,
    /// SHARED-TIMER
    SharedTimer                                                            = 1817,
    /// SHORT-LABEL
    ShortLabel                                                             = 2742,
    /// SHORT-LABEL-PATTERN
    ShortLabelPattern                                                      = 5859,
    /// SHORT-NAME
    ShortName                                                              = 1688,
    /// SHORT-NAME-FRAGMENT
    ShortNameFragment                                                      = 3243,
    /// SHORT-NAME-FRAGMENTS
    ShortNameFragments                                                     = 2784,
    /// SHORT-NAME-PATTERN
    ShortNamePattern                                                       = 4512,
    /// SHORT-TERM-ADJUSTMENT
    ShortTermAdjustment                                                    = 2626,
    /// SHORT-TERM-ADJUSTMENT-SUPPORTED
    ShortTermAdjustmentSupported                                           = 2401,
    /// SHUTDOWN
    Shutdown                                                               = 3970,
    /// SIGNAL-AGE
    SignalAge                                                              = 5371,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 1967,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 1006,
    /// SIGNAL-BASED-EVENT-REF
    SignalBasedEventRef                                                    = 2313,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 5866,
    /// SIGNAL-BASED-FIELD-REF
    SignalBasedFieldRef                                                    = 3747,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 6242,
    /// SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFireAndForgetMethodToISignalTriggeringMapping               = 6074,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 5898,
    /// SIGNAL-BASED-METHOD-REF
    SignalBasedMethodRef                                                   = 4990,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 1652,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 5090,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 4691,
    /// SIGNAL-FAN
    SignalFan                                                              = 1656,
    /// SIGNAL-GROUP-REF
    SignalGroupRef                                                         = 283,
    /// SIGNAL-I-PDU-COUNTER
    SignalIPduCounter                                                      = 3592,
    /// SIGNAL-I-PDU-REPLICATION
    SignalIPduReplication                                                  = 740,
    /// SIGNAL-MAPPINGS
    SignalMappings                                                         = 340,
    /// SIGNAL-PATH-CONSTRAINTS
    SignalPathConstraints                                                  = 1271,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 5537,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 839,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS
    SignalServiceTranslationEventPropss                                    = 1507,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 4681,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 4344,
    /// SIGNAL-SERVICE-TRANSLATION-PROPSS
    SignalServiceTranslationPropss                                         = 161,
    /// SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING
    SignalToReceiverTextTableMapping                                       = 200,
    /// SIGNALS
    Signals                                                                = 2720,
    /// SIGNATURE-SCHEME-ID
    SignatureSchemeId                                                      = 570,
    /// SIGNATURE-SCHEME-REF
    SignatureSchemeRef                                                     = 4367,
    /// SIGNATURE-SCHEME-REFS
    SignatureSchemeRefs                                                    = 2881,
    /// SIGNATURE-SUPPORT-AP
    SignatureSupportAp                                                     = 743,
    /// SIGNATURE-SUPPORT-CP
    SignatureSupportCp                                                     = 5925,
    /// SIGNIFICANCE
    Significance                                                           = 689,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 2629,
    /// SINGLE-ELEMENT-SERIALIZATION
    SingleElementSerialization                                             = 1910,
    /// SIZE
    Size                                                                   = 2333,
    /// SIZE-OF-ARRAY-LENGTH-FIELD
    SizeOfArrayLengthField                                                 = 5854,
    /// SIZE-OF-ARRAY-LENGTH-FIELDS
    SizeOfArrayLengthFields                                                = 207,
    /// SIZE-OF-STRING-LENGTH-FIELD
    SizeOfStringLengthField                                                = 1037,
    /// SIZE-OF-STRING-LENGTH-FIELDS
    SizeOfStringLengthFields                                               = 5226,
    /// SIZE-OF-STRUCT-LENGTH-FIELD
    SizeOfStructLengthField                                                = 695,
    /// SIZE-OF-STRUCT-LENGTH-FIELDS
    SizeOfStructLengthFields                                               = 1968,
    /// SIZE-OF-UNION-LENGTH-FIELD
    SizeOfUnionLengthField                                                 = 306,
    /// SIZE-OF-UNION-LENGTH-FIELDS
    SizeOfUnionLengthFields                                                = 4052,
    /// SIZE-OF-UNION-TYPE-SELECTOR-FIELD
    SizeOfUnionTypeSelectorField                                           = 3427,
    /// SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE
    SlaveActAsPassiveCommunicationSlave                                    = 3666,
    /// SLAVE-EVENT-REF
    SlaveEventRef                                                          = 5186,
    /// SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME
    SlaveQualifiedUnexpectedLinkDownTime                                   = 1218,
    /// SLAVE-REF
    SlaveRef                                                               = 6498,
    /// SLAVES
    Slaves                                                                 = 3609,
    /// SLEEP-MODE-EXECUTION-DELAY
    SleepModeExecutionDelay                                                = 1070,
    /// SLEEP-MODE-SUPPORTED
    SleepModeSupported                                                     = 1206,
    /// SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST
    SleepRepetitionDelayOfSleepRequest                                     = 5875,
    /// SLEEP-REPETITIONS-OF-SLEEP-REQUEST
    SleepRepetitionsOfSleepRequest                                         = 1783,
    /// SLEEP-TIME
    SleepTime                                                              = 4056,
    /// SLOT-CAPACITY
    SlotCapacity                                                           = 1586,
    /// SLOT-ID
    SlotId                                                                 = 4771,
    /// SLOT-TYPE
    SlotType                                                               = 2775,
    /// SM-INTERACTS-WITH-NM-MAPPING
    SmInteractsWithNmMapping                                               = 4698,
    /// SNAPSHOT-RECORD-CONTENTS
    SnapshotRecordContents                                                 = 778,
    /// SO-AD-CONFIG
    SoAdConfig                                                             = 6484,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 4797,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 6388,
    /// SO-CON-I-PDU-IDENTIFIER-REF
    SoConIPduIdentifierRef                                                 = 257,
    /// SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL
    SoConIPduIdentifierRefConditional                                      = 5043,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 5916,
    /// SOCKET-ADDRESS-REF
    SocketAddressRef                                                       = 3835,
    /// SOCKET-ADDRESS-REF-CONDITIONAL
    SocketAddressRefConditional                                            = 937,
    /// SOCKET-ADDRESSS
    SocketAddresss                                                         = 4193,
    /// SOCKET-CONNECTION
    SocketConnection                                                       = 4405,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 1650,
    /// SOCKET-CONNECTION-BUNDLE-REF
    SocketConnectionBundleRef                                              = 3226,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER
    SocketConnectionIpduIdentifier                                         = 1677,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 6287,
    /// SOCKET-CONNECTION-REF
    SocketConnectionRef                                                    = 1083,
    /// SOCKET-CONNECTION-REFS
    SocketConnectionRefs                                                   = 4948,
    /// SOCKET-OPTION
    SocketOption                                                           = 2368,
    /// SOCKET-OPTIONS
    SocketOptions                                                          = 4828,
    /// SOCKET-PROTOCOL
    SocketProtocol                                                         = 2884,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION
    SoftwareActivationDependencyCompareCondition                           = 1202,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA
    SoftwareActivationDependencyFormula                                    = 2008,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-REF
    SoftwareActivationDependencyRef                                        = 2364,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 3496,
    /// SOFTWARE-CLUSTER-COM-RESOURCE-REF
    SoftwareClusterComResourceRef                                          = 5350,
    /// SOFTWARE-CLUSTER-DEPENDENCY
    SoftwareClusterDependency                                              = 1167,
    /// SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION
    SoftwareClusterDependencyCompareCondition                              = 4047,
    /// SOFTWARE-CLUSTER-DEPENDENCY-FORMULA
    SoftwareClusterDependencyFormula                                       = 1701,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 2305,
    /// SOFTWARE-CLUSTER-DESIGN-DEPENDENCY
    SoftwareClusterDesignDependency                                        = 2201,
    /// SOFTWARE-CLUSTER-DESIGN-REF
    SoftwareClusterDesignRef                                               = 3500,
    /// SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS
    SoftwareClusterDiagnosticDeploymentProps                               = 5853,
    /// SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS
    SoftwareClusterDoipDiagnosticAddress                                   = 34,
    /// SOFTWARE-CLUSTER-ID
    SoftwareClusterId                                                      = 3208,
    /// SOFTWARE-CLUSTER-REF
    SoftwareClusterRef                                                     = 5141,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 5824,
    /// SOFTWARE-CLUSTER-SOVD-ADDRESS
    SoftwareClusterSovdAddress                                             = 6115,
    /// SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING
    SoftwareClusterToApplicationPartitionMapping                           = 3364,
    /// SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPINGS
    SoftwareClusterToApplicationPartitionMappings                          = 1135,
    /// SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS
    SoftwareClusterToResourceMappings                                      = 3595,
    /// SOFTWARE-CLUSTER-UDS-DIAGNOSTIC-ADDRESS
    SoftwareClusterUdsDiagnosticAddress                                    = 6261,
    /// SOFTWARE-COMPOSITION-TREF
    SoftwareCompositionTref                                                = 4005,
    /// SOFTWARE-CONTEXT
    SoftwareContext                                                        = 3177,
    /// SOFTWARE-MEMORY-SECTION-REF
    SoftwareMemorySectionRef                                               = 5997,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 2921,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 6449,
    /// SOFTWARE-PACKAGE-STEPS
    SoftwarePackageSteps                                                   = 1767,
    /// SOFTWARE-PACKAGE-STORING
    SoftwarePackageStoring                                                 = 4139,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 347,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 6274,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 5625,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 1289,
    /// SOMEIP-EVENT-PROPS
    SomeipEventProps                                                       = 3192,
    /// SOMEIP-FIELD
    SomeipField                                                            = 1798,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 1745,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 3260,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 3107,
    /// SOMEIP-METHOD-PROPS
    SomeipMethodProps                                                      = 1129,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 1900,
    /// SOMEIP-REMOTE-MULTICAST-CONFIG
    SomeipRemoteMulticastConfig                                            = 542,
    /// SOMEIP-REMOTE-UNICAST-CONFIG
    SomeipRemoteUnicastConfig                                              = 4696,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 5798,
    /// SOMEIP-RULE
    SomeipRule                                                             = 5383,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 6212,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdClientEventGroupTimingConfigRef                                = 4511,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdClientEventGroupTimingConfigRefConditional                     = 3458,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 3466,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdClientServiceInstanceConfigRef                                 = 5294,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdClientServiceInstanceConfigRefConditional                      = 2187,
    /// SOMEIP-SD-RULE
    SomeipSdRule                                                           = 1899,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 2126,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdServerEventGroupTimingConfigRef                                = 5517,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdServerEventGroupTimingConfigRefConditional                     = 3865,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 4188,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdServerServiceInstanceConfigRef                                 = 4732,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdServerServiceInstanceConfigRefConditional                      = 2984,
    /// SOMEIP-SERVICE-DISCOVERY
    SomeipServiceDiscovery                                                 = 1765,
    /// SOMEIP-SERVICE-DISCOVERY-PORT
    SomeipServiceDiscoveryPort                                             = 704,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 6112,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 456,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 4835,
    /// SOMEIP-SERVICE-VERSION
    SomeipServiceVersion                                                   = 6176,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 3608,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 2218,
    /// SOMEIP-TP-CONNECTION
    SomeipTpConnection                                                     = 3693,
    /// SOMEIP-TRANSFORMATION-DESCRIPTION
    SomeipTransformationDescription                                        = 3890,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS
    SomeipTransformationISignalProps                                       = 2109,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    SomeipTransformationISignalPropsConditional                            = 5932,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    SomeipTransformationISignalPropsVariants                               = 6506,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 5941,
    /// SOMEIP-TRANSFORMATION-PROPS-REF
    SomeipTransformationPropsRef                                           = 1451,
    /// SOUCE-CAN-ID-REF
    SouceCanIdRef                                                          = 1446,
    /// SOURCE-ADDRESS-CODE
    SourceAddressCode                                                      = 1575,
    /// SOURCE-ADDRESS-MASK
    SourceAddressMask                                                      = 4370,
    /// SOURCE-ADDRESS-RANGE-END
    SourceAddressRangeEnd                                                  = 4541,
    /// SOURCE-ADDRESS-RANGE-START
    SourceAddressRangeStart                                                = 1874,
    /// SOURCE-CAN-ID-CODE
    SourceCanIdCode                                                        = 716,
    /// SOURCE-CAN-ID-MASK
    SourceCanIdMask                                                        = 4934,
    /// SOURCE-CAN-ID-REF
    SourceCanIdRef                                                         = 1510,
    /// SOURCE-CHANNEL
    SourceChannel                                                          = 1296,
    /// SOURCE-EEC-REF
    SourceEecRef                                                           = 4041,
    /// SOURCE-EEC-REFS
    SourceEecRefs                                                          = 2646,
    /// SOURCE-ELEMENT-REF
    SourceElementRef                                                       = 2099,
    /// SOURCE-ELEMENT-REFS
    SourceElementRefs                                                      = 2471,
    /// SOURCE-EVENT-REF
    SourceEventRef                                                         = 4531,
    /// SOURCE-EVENT-REFS
    SourceEventRefs                                                        = 5040,
    /// SOURCE-FIELD-REF
    SourceFieldRef                                                         = 4998,
    /// SOURCE-FRAME-REF
    SourceFrameRef                                                         = 718,
    /// SOURCE-I-PDU-REF
    SourceIPduRef                                                          = 179,
    /// SOURCE-ID
    SourceId                                                               = 376,
    /// SOURCE-INSTANCE-IREF
    SourceInstanceIref                                                     = 4971,
    /// SOURCE-INSTANCE-IREFS
    SourceInstanceIrefs                                                    = 1207,
    /// SOURCE-IP-ADDRESS
    SourceIpAddress                                                        = 5983,
    /// SOURCE-IPV-4-ADDRESS
    SourceIpv4Address                                                      = 3406,
    /// SOURCE-IPV-6-ADDRESS
    SourceIpv6Address                                                      = 4293,
    /// SOURCE-LIN-PID-REF
    SourceLinPidRef                                                        = 2807,
    /// SOURCE-MAC-ADDRESS
    SourceMacAddress                                                       = 4591,
    /// SOURCE-MAC-ADDRESS-MASK
    SourceMacAddressMask                                                   = 3816,
    /// SOURCE-MAX-ADDRESS
    SourceMaxAddress                                                       = 1306,
    /// SOURCE-METHOD-REF
    SourceMethodRef                                                        = 3061,
    /// SOURCE-MIN-ADDRESS
    SourceMinAddress                                                       = 1401,
    /// SOURCE-NETWORK-MASK
    SourceNetworkMask                                                      = 2849,
    /// SOURCE-PORTS
    SourcePorts                                                            = 4467,
    /// SOURCE-POSSIBLE-ERROR-REF
    SourcePossibleErrorRef                                                 = 3628,
    /// SOURCE-REF
    SourceRef                                                              = 6265,
    /// SOURCE-SERVICE-INTERFACE-REF
    SourceServiceInterfaceRef                                              = 6025,
    /// SOURCE-SERVICE-INTERFACE-REFS
    SourceServiceInterfaceRefs                                             = 1404,
    /// SOURCE-SIGNAL-REF
    SourceSignalRef                                                        = 4074,
    /// SOURCE-TRIGGER-REF
    SourceTriggerRef                                                       = 5871,
    /// SOVD-CONTENT-REF
    SovdContentRef                                                         = 3974,
    /// SOVD-CONTENT-REFS
    SovdContentRefs                                                        = 4649,
    /// SOVD-DATA-CATEGORY-REF
    SovdDataCategoryRef                                                    = 3155,
    /// SOVD-GATEWAY-INSTANTIATION
    SovdGatewayInstantiation                                               = 6186,
    /// SOVD-GROUP-REF
    SovdGroupRef                                                           = 1477,
    /// SOVD-GROUP-REFS
    SovdGroupRefs                                                          = 219,
    /// SOVD-LOCK-REF
    SovdLockRef                                                            = 1988,
    /// SOVD-OPERATION-REF
    SovdOperationRef                                                       = 5715,
    /// SOVD-PROPERTIES
    SovdProperties                                                         = 3845,
    /// SOVD-SERVER-INSTANTIATION
    SovdServerInstantiation                                                = 1856,
    /// SOVD-SNAPSHOT-CONTENT-REF
    SovdSnapshotContentRef                                                 = 3380,
    /// SOVD-VALIDATION-ORDER-REF
    SovdValidationOrderRef                                                 = 5671,
    /// SOVD-VALIDATION-ORDER-REFS
    SovdValidationOrderRefs                                                = 2749,
    /// SPARSE-TIMESTAMP-ENABLED
    SparseTimestampEnabled                                                 = 4474,
    /// SPECIFIC-CHANNEL-REF
    SpecificChannelRef                                                     = 5303,
    /// SPECIFIC-CHANNELS
    SpecificChannels                                                       = 1246,
    /// SPECIFIC-PHYSICAL-CHANNEL-REF
    SpecificPhysicalChannelRef                                             = 4892,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 2107,
    /// SPECIFICATION-DOCUMENT-SCOPES
    SpecificationDocumentScopes                                            = 1747,
    /// SPECIFICATION-SCOPE
    SpecificationScope                                                     = 701,
    /// SPEED
    Speed                                                                  = 2486,
    /// SPEEDUP
    Speedup                                                                = 2239,
    /// SPN
    Spn                                                                    = 3832,
    /// SPN-REF
    SpnRef                                                                 = 3586,
    /// SPN-REFS
    SpnRefs                                                                = 4473,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 433,
    /// SRVT
    Srvt                                                                   = 4398,
    /// SSP-OFFSET
    SspOffset                                                              = 1389,
    /// SSRC
    Ssrc                                                                   = 2579,
    /// ST-MIN
    StMin                                                                  = 2685,
    /// STACK-USAGES
    StackUsages                                                            = 1251,
    /// STANDARD
    Standard                                                               = 4470,
    /// STANDARD-REVISION
    StandardRevision                                                       = 720,
    /// STANDARD-REVISIONS
    StandardRevisions                                                      = 5688,
    /// START
    Start                                                                  = 3883,
    /// START-ACTION-REF
    StartActionRef                                                         = 5927,
    /// START-ACTION-REFS
    StartActionRefs                                                        = 3014,
    /// START-ADDRESS
    StartAddress                                                           = 1828,
    /// START-AGENT-ERROR-REF
    StartAgentErrorRef                                                     = 1597,
    /// START-AGENT-REF
    StartAgentRef                                                          = 2372,
    /// START-INDEX
    StartIndex                                                             = 3877,
    /// START-IREF
    StartIref                                                              = 4780,
    /// START-ON-EVENT-REF
    StartOnEventRef                                                        = 1715,
    /// START-POSITION
    StartPosition                                                          = 3387,
    /// START-STATE-MACHINE-REF
    StartStateMachineRef                                                   = 1043,
    /// STARTS-ON-EVENT-REF
    StartsOnEventRef                                                       = 5320,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 3319,
    /// STARTUP-CONFIG-REF
    StartupConfigRef                                                       = 762,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 1598,
    /// STARTUP-CONFIGS
    StartupConfigs                                                         = 4362,
    /// STARTUP-OPTION
    StartupOption                                                          = 4453,
    /// STARTUP-OPTIONS
    StartupOptions                                                         = 2397,
    /// STATE
    State                                                                  = 2273,
    /// STATE-CLIENT-INTERFACE
    StateClientInterface                                                   = 3739,
    /// STATE-DEP-FIREWALL-REF
    StateDepFirewallRef                                                    = 1454,
    /// STATE-DEP-FIREWALL-REFS
    StateDepFirewallRefs                                                   = 388,
    /// STATE-DEPENDENT-FIREWALL
    StateDependentFirewall                                                 = 1655,
    /// STATE-DEPENDENT-FIREWALL-REF
    StateDependentFirewallRef                                              = 3312,
    /// STATE-DEPENDENT-STARTUP-CONFIG
    StateDependentStartupConfig                                            = 3318,
    /// STATE-DEPENDENT-STARTUP-CONFIGS
    StateDependentStartupConfigs                                           = 1416,
    /// STATE-MACHINE
    StateMachine                                                           = 3269,
    /// STATE-MANAGEMEN-PHM-ERROR-INTERFACE
    StateManagemenPhmErrorInterface                                        = 1038,
    /// STATE-MANAGEMENT-ACTION-LIST
    StateManagementActionList                                              = 2814,
    /// STATE-MANAGEMENT-BASED-ON-TCP-FLAGS
    StateManagementBasedOnTcpFlags                                         = 625,
    /// STATE-MANAGEMENT-COMPARE-FORMULA
    StateManagementCompareFormula                                          = 4759,
    /// STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE
    StateManagementDiagTriggerInterface                                    = 5952,
    /// STATE-MANAGEMENT-EM-ERROR-INTERFACE
    StateManagementEmErrorInterface                                        = 5908,
    /// STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-ACTION-ITEM
    StateManagementEnterSuspendToRamActionItem                             = 4432,
    /// STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-OS-ACTION-ITEM
    StateManagementEnterSuspendToRamOsActionItem                           = 1759,
    /// STATE-MANAGEMENT-ERROR-COMPARE-RULE
    StateManagementErrorCompareRule                                        = 4141,
    /// STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE
    StateManagementFunctionGroupSwitchNotificationInterface                = 2383,
    /// STATE-MANAGEMENT-LEAVE-SUSPEND-TO-RAM-ACTION-ITEM
    StateManagementLeaveSuspendToRamActionItem                             = 1453,
    /// STATE-MANAGEMENT-MODULE-INSTANTIATION
    StateManagementModuleInstantiation                                     = 5988,
    /// STATE-MANAGEMENT-NM-ACTION-ITEM
    StateManagementNmActionItem                                            = 5062,
    /// STATE-MANAGEMENT-PHM-ERROR-INTERFACE
    StateManagementPhmErrorInterface                                       = 1842,
    /// STATE-MANAGEMENT-REQUEST-ERROR
    StateManagementRequestError                                            = 462,
    /// STATE-MANAGEMENT-REQUEST-RULE
    StateManagementRequestRule                                             = 5989,
    /// STATE-MANAGEMENT-REQUEST-TRIGGER
    StateManagementRequestTrigger                                          = 6067,
    /// STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM
    StateManagementSetFunctionGroupStateActionItem                         = 5578,
    /// STATE-MANAGEMENT-SLEEP-ACTION-ITEM
    StateManagementSleepActionItem                                         = 3805,
    /// STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM
    StateManagementStateMachineActionItem                                  = 506,
    /// STATE-MANAGEMENT-STATE-NOTIFICATION
    StateManagementStateNotification                                       = 519,
    /// STATE-MANAGEMENT-SYNC-ACTION-ITEM
    StateManagementSyncActionItem                                          = 4788,
    /// STATE-MANAGEMENT-TRIGGER-COMPARE-RULE
    StateManagementTriggerCompareRule                                      = 1637,
    /// STATE-REF
    StateRef                                                               = 5228,
    /// STATE-REFERENCES
    StateReferences                                                        = 644,
    /// STATE-REQUEST-PORT-IREF
    StateRequestPortIref                                                   = 3358,
    /// STATE-REQUEST-REF
    StateRequestRef                                                        = 6350,
    /// STATIC-MEMORYS
    StaticMemorys                                                          = 3153,
    /// STATIC-PART
    StaticPart                                                             = 4674,
    /// STATIC-PARTS
    StaticParts                                                            = 299,
    /// STATIC-REMOTE-MULTICAST-ADDRESSES
    StaticRemoteMulticastAddresses                                         = 6606,
    /// STATIC-REMOTE-UNICAST-ADDRESSES
    StaticRemoteUnicastAddresses                                           = 3582,
    /// STATIC-SLOT-DURATION
    StaticSlotDuration                                                     = 3375,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 1254,
    /// STATIC-SOCKET-CONNECTION-REF
    StaticSocketConnectionRef                                              = 5305,
    /// STATIC-SOCKET-CONNECTIONS
    StaticSocketConnections                                                = 6295,
    /// STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR
    StatusBitHandlingTestFailedSinceLastClear                              = 6554,
    /// STATUS-BIT-STORAGE-TEST-FAILED
    StatusBitStorageTestFailed                                             = 3410,
    /// STATUS-ID
    StatusId                                                               = 3256,
    /// STATUS-IREF
    StatusIref                                                             = 6450,
    /// STATUS-REF
    StatusRef                                                              = 1450,
    /// STATUS-SUB-TLV
    StatusSubTlv                                                           = 3366,
    /// STATUSS
    Statuss                                                                = 6284,
    /// STD
    Std                                                                    = 3225,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 4302,
    /// STD-REF
    StdRef                                                                 = 2103,
    /// STD-REFS
    StdRefs                                                                = 6066,
    /// STEP-SIZE
    StepSize                                                               = 1997,
    /// STIM-ENABLER
    StimEnabler                                                            = 1490,
    /// STIMULUS-REF
    StimulusRef                                                            = 1772,
    /// STOP
    Stop                                                                   = 881,
    /// STOP-AGENT-ERROR-REF
    StopAgentErrorRef                                                      = 1786,
    /// STOP-AGENT-REF
    StopAgentRef                                                           = 4946,
    /// STOP-IREF
    StopIref                                                               = 6600,
    /// STOP-STATE-MACHINE-REF
    StopStateMachineRef                                                    = 4288,
    /// STORAGE-CONDITION-GROUP-REF
    StorageConditionGroupRef                                               = 6639,
    /// STORAGE-CONDITIONS
    StorageConditions                                                      = 6339,
    /// STORAGE-STATE-EVALUATION
    StorageStateEvaluation                                                 = 1957,
    /// STORE-AT-SHUTDOWN
    StoreAtShutdown                                                        = 2636,
    /// STORE-CYCLIC
    StoreCyclic                                                            = 1499,
    /// STORE-EMERGENCY
    StoreEmergency                                                         = 2529,
    /// STORE-EVENT-ENABLED
    StoreEventEnabled                                                      = 185,
    /// STORE-EVENT-SUPPORT
    StoreEventSupport                                                      = 4222,
    /// STORE-IMMEDIATE
    StoreImmediate                                                         = 1951,
    /// STORE-ON-CHANGE
    StoreOnChange                                                          = 6166,
    /// STORING
    Storing                                                                = 2949,
    /// STREAM-FILTER-PORT-RANGE
    StreamFilterPortRange                                                  = 1028,
    /// STREAM-FILTER-RULE
    StreamFilterRule                                                       = 4085,
    /// STREAM-FILTERS
    StreamFilters                                                          = 4175,
    /// STREAM-GATE-REF
    StreamGateRef                                                          = 4745,
    /// STREAM-GATES
    StreamGates                                                            = 4430,
    /// STREAM-ID
    StreamId                                                               = 1607,
    /// STREAM-IDENTIFICATION-HANDLE-REF
    StreamIdentificationHandleRef                                          = 548,
    /// STREAM-IDENTIFICATION-HANDLE-REFS
    StreamIdentificationHandleRefs                                         = 4500,
    /// STREAM-IDENTIFICATION-WILDCARD
    StreamIdentificationWildcard                                           = 5095,
    /// STREAM-IDENTIFIER
    StreamIdentifier                                                       = 3116,
    /// STREAMS-PER-FRAME
    StreamsPerFrame                                                        = 938,
    /// STRING-ENCODING
    StringEncoding                                                         = 1163,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 3658,
    /// SUB
    Sub                                                                    = 5315,
    /// SUB-ATTRIBUTE-TAILORINGS
    SubAttributeTailorings                                                 = 1260,
    /// SUB-CONTAINERS
    SubContainers                                                          = 5049,
    /// SUB-DOMAIN-REF
    SubDomainRef                                                           = 5954,
    /// SUB-DOMAIN-REFS
    SubDomainRefs                                                          = 2191,
    /// SUB-ELEMENT-MAPPING
    SubElementMapping                                                      = 2431,
    /// SUB-ELEMENT-MAPPINGS
    SubElementMappings                                                     = 1316,
    /// SUB-ELEMENTS
    SubElements                                                            = 1917,
    /// SUB-FUNCTION-REF
    SubFunctionRef                                                         = 4776,
    /// SUB-FUNCTION-REFS
    SubFunctionRefs                                                        = 5381,
    /// SUB-GROUP-REF
    SubGroupRef                                                            = 3524,
    /// SUB-GROUP-REFS
    SubGroupRefs                                                           = 1261,
    /// SUB-NODE-CHANNEL-REF
    SubNodeChannelRef                                                      = 5280,
    /// SUB-NODE-CHANNELS
    SubNodeChannels                                                        = 4215,
    /// SUB-NODE-NUMBER
    SubNodeNumber                                                          = 436,
    /// SUB-NODE-PHYSICAL-CHANNEL-REF
    SubNodePhysicalChannelRef                                              = 125,
    /// SUB-SDG-REF
    SubSdgRef                                                              = 3678,
    /// SUB-SOFTWARE-CLUSTER-REF
    SubSoftwareClusterRef                                                  = 235,
    /// SUB-SOFTWARE-CLUSTER-REFS
    SubSoftwareClusterRefs                                                 = 6338,
    /// SUB-TLV-CONFIG
    SubTlvConfig                                                           = 2524,
    /// SUB-TYPE
    SubType                                                                = 5257,
    /// SUBFUNCTION
    Subfunction                                                            = 420,
    /// SUBFUNCTIONS
    Subfunctions                                                           = 3487,
    /// SUBMESSAGE-TYPE
    SubmessageType                                                         = 1449,
    /// SUBNET-NUMBER
    SubnetNumber                                                           = 4589,
    /// SUBSCRIBE-EVENTGROUP-RETRY-DELAY
    SubscribeEventgroupRetryDelay                                          = 6039,
    /// SUBSCRIBE-EVENTGROUP-RETRY-MAX
    SubscribeEventgroupRetryMax                                            = 3181,
    /// SUBSTITUTED-FRAME-REF
    SubstitutedFrameRef                                                    = 1591,
    /// SUBSTITUTED-FRAME-REFS
    SubstitutedFrameRefs                                                   = 2932,
    /// SUBTITLE
    Subtitle                                                               = 6259,
    /// SUCCESSOR-REF
    SuccessorRef                                                           = 1311,
    /// SUCCESSOR-REFS
    SuccessorRefs                                                          = 634,
    /// SUP
    Sup                                                                    = 4598,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 3719,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF
    SupervisedEntityCheckpointNeedsRef                                     = 2450,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL
    SupervisedEntityCheckpointNeedsRefConditional                          = 6430,
    /// SUPERVISED-ENTITY-ID
    SupervisedEntityId                                                     = 3689,
    /// SUPERVISED-ENTITY-IREF
    SupervisedEntityIref                                                   = 5786,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 5716,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 4756,
    /// SUPERVISION-CONDITION
    SupervisionCondition                                                   = 1990,
    /// SUPERVISION-CYCLE
    SupervisionCycle                                                       = 5644,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 3696,
    /// SUPERVISION-ENTITYS
    SupervisionEntitys                                                     = 780,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 6364,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 368,
    /// SUPERVISION-MODE-CONDITIONS
    SupervisionModeConditions                                              = 3932,
    /// SUPERVISION-MODES
    SupervisionModes                                                       = 6454,
    /// SUPERVISION-REF
    SupervisionRef                                                         = 6138,
    /// SUPPLIER-ID
    SupplierId                                                             = 154,
    /// SUPPLIER-VALIDATION-ORDER-REF
    SupplierValidationOrderRef                                             = 6571,
    /// SUPPLIER-VALIDATION-ORDER-REFS
    SupplierValidationOrderRefs                                            = 2382,
    /// SUPPORT-BUFFER-LOCKING
    SupportBufferLocking                                                   = 2483,
    /// SUPPORT-DIRTY-FLAG
    SupportDirtyFlag                                                       = 1863,
    /// SUPPORT-INFO
    SupportInfo                                                            = 2740,
    /// SUPPORT-INFO-BIT
    SupportInfoBit                                                         = 5366,
    /// SUPPORT-INFO-BYTE
    SupportInfoByte                                                        = 1676,
    /// SUPPORTED-CIPHER-SUITES
    SupportedCipherSuites                                                  = 1059,
    /// SUPPORTED-CONFIG-VARIANT
    SupportedConfigVariant                                                 = 5321,
    /// SUPPORTED-CONFIG-VARIANTS
    SupportedConfigVariants                                                = 6580,
    /// SUPPORTED-FEATURES
    SupportedFeatures                                                      = 3550,
    /// SUPPORTED-TIMER-GRANULARITY
    SupportedTimerGranularity                                              = 1224,
    /// SUPPORTING-MATERIAL
    SupportingMaterial                                                     = 6243,
    /// SUPPORTS-ASYNCHRONOUS-MODE-SWITCH
    SupportsAsynchronousModeSwitch                                         = 1516,
    /// SUPPORTS-MULTIPLE-INSTANTIATION
    SupportsMultipleInstantiation                                          = 633,
    /// SUPPORTS-MULTIPLE-NOTIFIER-SETS
    SupportsMultipleNotifierSets                                           = 3657,
    /// SUSPEND-TO-RAM-AWARENESS
    SuspendToRamAwareness                                                  = 905,
    /// SUSPEND-TO-RAM-HUB-INTERFACE
    SuspendToRamHubInterface                                               = 1105,
    /// SUSPEND-TO-RAM-HUB-MAPPING
    SuspendToRamHubMapping                                                 = 3274,
    /// SUSPEND-TO-RAM-MODULE-INSTANTIATION
    SuspendToRamModuleInstantiation                                        = 5048,
    /// SUSPEND-TO-RAM-SATELLITE-INTERFACE
    SuspendToRamSatelliteInterface                                         = 1952,
    /// SUSPEND-TO-RAM-SATELLITE-MAPPING
    SuspendToRamSatelliteMapping                                           = 897,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 2395,
    /// SW-ADDR-METHOD-REF
    SwAddrMethodRef                                                        = 1468,
    /// SW-ADDRMETHOD-REF
    SwAddrmethodRef                                                        = 4160,
    /// SW-ALIGNMENT
    SwAlignment                                                            = 566,
    /// SW-ARRAYSIZE
    SwArraysize                                                            = 4202,
    /// SW-AXIS-CONT
    SwAxisCont                                                             = 4883,
    /// SW-AXIS-CONTS
    SwAxisConts                                                            = 6131,
    /// SW-AXIS-GENERIC
    SwAxisGeneric                                                          = 1381,
    /// SW-AXIS-GROUPED
    SwAxisGrouped                                                          = 5213,
    /// SW-AXIS-INDEX
    SwAxisIndex                                                            = 1744,
    /// SW-AXIS-INDIVIDUAL
    SwAxisIndividual                                                       = 4017,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 4023,
    /// SW-AXIS-TYPE-REF
    SwAxisTypeRef                                                          = 6651,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 686,
    /// SW-BIT-REPRESENTATION
    SwBitRepresentation                                                    = 6349,
    /// SW-CALIBRATION-ACCESS
    SwCalibrationAccess                                                    = 1802,
    /// SW-CALIBRATION-NOTES
    SwCalibrationNotes                                                     = 586,
    /// SW-CALIBRATION-NOTESS
    SwCalibrationNotess                                                    = 325,
    /// SW-CALPRM-AXIS
    SwCalprmAxis                                                           = 1561,
    /// SW-CALPRM-AXIS-SET
    SwCalprmAxisSet                                                        = 5363,
    /// SW-CALPRM-IMPL
    SwCalprmImpl                                                           = 6314,
    /// SW-CALPRM-IMPLS
    SwCalprmImpls                                                          = 4408,
    /// SW-CALPRM-PROTOTYPE-REF
    SwCalprmPrototypeRef                                                   = 1833,
    /// SW-CARB-DOC
    SwCarbDoc                                                              = 4360,
    /// SW-CARB-DOCS
    SwCarbDocs                                                             = 3376,
    /// SW-CLASS-ATTR-INSTANCE-IMPL
    SwClassAttrInstanceImpl                                                = 1173,
    /// SW-CLASS-IMPL
    SwClassImpl                                                            = 69,
    /// SW-CLASS-IMPLS
    SwClassImpls                                                           = 683,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 3472,
    /// SW-CLASS-PROTOTYPE-REF
    SwClassPrototypeRef                                                    = 1152,
    /// SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF
    SwClusterDesignProvidedPortIref                                        = 2137,
    /// SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF
    SwClusterDesignRequiredPortIref                                        = 4322,
    /// SW-CLUSTER-MAPPINGS
    SwClusterMappings                                                      = 6518,
    /// SW-CLUSTER-REF
    SwClusterRef                                                           = 1878,
    /// SW-CLUSTER-REFS
    SwClusterRefs                                                          = 434,
    /// SW-CLUSTERS
    SwClusters                                                             = 2923,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 5804,
    /// SW-COMP-TO-ECU-MAPPING-REF
    SwCompToEcuMappingRef                                                  = 2164,
    /// SW-COMP-TO-ECU-MAPPING-REFS
    SwCompToEcuMappingRefs                                                 = 6320,
    /// SW-COMPARISON-VARIABLES
    SwComparisonVariables                                                  = 1502,
    /// SW-COMPONENT-ASSIGNMENTS
    SwComponentAssignments                                                 = 2227,
    /// SW-COMPONENT-DOCUMENTATION
    SwComponentDocumentation                                               = 6462,
    /// SW-COMPONENT-DOCUMENTATIONS
    SwComponentDocumentations                                              = 3787,
    /// SW-COMPONENT-IREF
    SwComponentIref                                                        = 1334,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 3468,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 2021,
    /// SW-COMPONENT-PROTOTYPE-ASSIGNMENT
    SwComponentPrototypeAssignment                                         = 44,
    /// SW-COMPONENT-PROTOTYPE-IREF
    SwComponentPrototypeIref                                               = 2404,
    /// SW-COMPONENT-PROTOTYPE-REF
    SwComponentPrototypeRef                                                = 3779,
    /// SW-COMPOSITIONS
    SwCompositions                                                         = 4902,
    /// SW-DATA-DEF-PROPS
    SwDataDefProps                                                         = 5689,
    /// SW-DATA-DEF-PROPS-CONDITIONAL
    SwDataDefPropsConditional                                              = 349,
    /// SW-DATA-DEF-PROPS-VARIANTS
    SwDataDefPropsVariants                                                 = 2496,
    /// SW-DATA-DEPENDENCY
    SwDataDependency                                                       = 1979,
    /// SW-DATA-DEPENDENCY-ARGS
    SwDataDependencyArgs                                                   = 2482,
    /// SW-DATA-DEPENDENCY-FORMULA
    SwDataDependencyFormula                                                = 6288,
    /// SW-DIAGNOSTICS-NOTES
    SwDiagnosticsNotes                                                     = 2424,
    /// SW-DIAGNOSTICS-NOTESS
    SwDiagnosticsNotess                                                    = 2690,
    /// SW-FEATURE-DEF
    SwFeatureDef                                                           = 4439,
    /// SW-FEATURE-DEFS
    SwFeatureDefs                                                          = 4638,
    /// SW-FEATURE-DESC
    SwFeatureDesc                                                          = 5353,
    /// SW-FEATURE-DESCS
    SwFeatureDescs                                                         = 330,
    /// SW-FILL-CHARACTER
    SwFillCharacter                                                        = 5291,
    /// SW-GENERIC-AXIS-DESC
    SwGenericAxisDesc                                                      = 5237,
    /// SW-GENERIC-AXIS-PARAM
    SwGenericAxisParam                                                     = 2319,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 2128,
    /// SW-GENERIC-AXIS-PARAM-TYPE-REF
    SwGenericAxisParamTypeRef                                              = 1396,
    /// SW-GENERIC-AXIS-PARAM-TYPES
    SwGenericAxisParamTypes                                                = 5365,
    /// SW-GENERIC-AXIS-PARAMS
    SwGenericAxisParams                                                    = 2891,
    /// SW-HOST-VARIABLE
    SwHostVariable                                                         = 2918,
    /// SW-IMPL-MAPPINGS
    SwImplMappings                                                         = 3687,
    /// SW-IMPL-POLICY
    SwImplPolicy                                                           = 4508,
    /// SW-INTENDED-RESOLUTION
    SwIntendedResolution                                                   = 5323,
    /// SW-INTERPOLATION-METHOD
    SwInterpolationMethod                                                  = 2069,
    /// SW-IS-VIRTUAL
    SwIsVirtual                                                            = 1793,
    /// SW-MAINTENANCE-NOTES
    SwMaintenanceNotes                                                     = 2517,
    /// SW-MAINTENANCE-NOTESS
    SwMaintenanceNotess                                                    = 5340,
    /// SW-MAPPINGS
    SwMappings                                                             = 2000,
    /// SW-MAX-AXIS-POINTS
    SwMaxAxisPoints                                                        = 5104,
    /// SW-MAX-TEXT-SIZE
    SwMaxTextSize                                                          = 5654,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 2055,
    /// SW-MIN-AXIS-POINTS
    SwMinAxisPoints                                                        = 6077,
    /// SW-NUMBER-OF-AXIS-POINTS
    SwNumberOfAxisPoints                                                   = 3762,
    /// SW-POINTER-TARGET-PROPS
    SwPointerTargetProps                                                   = 1619,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 3016,
    /// SW-RECORD-LAYOUT-COMPONENT
    SwRecordLayoutComponent                                                = 2155,
    /// SW-RECORD-LAYOUT-GROUP
    SwRecordLayoutGroup                                                    = 738,
    /// SW-RECORD-LAYOUT-GROUP-AXIS
    SwRecordLayoutGroupAxis                                                = 2174,
    /// SW-RECORD-LAYOUT-GROUP-FROM
    SwRecordLayoutGroupFrom                                                = 5486,
    /// SW-RECORD-LAYOUT-GROUP-INDEX
    SwRecordLayoutGroupIndex                                               = 1944,
    /// SW-RECORD-LAYOUT-GROUP-STEP
    SwRecordLayoutGroupStep                                                = 2036,
    /// SW-RECORD-LAYOUT-GROUP-TO
    SwRecordLayoutGroupTo                                                  = 3577,
    /// SW-RECORD-LAYOUT-REF
    SwRecordLayoutRef                                                      = 3474,
    /// SW-RECORD-LAYOUT-V
    SwRecordLayoutV                                                        = 5931,
    /// SW-RECORD-LAYOUT-V-AXIS
    SwRecordLayoutVAxis                                                    = 141,
    /// SW-RECORD-LAYOUT-V-FIX-VALUE
    SwRecordLayoutVFixValue                                                = 1488,
    /// SW-RECORD-LAYOUT-V-INDEX
    SwRecordLayoutVIndex                                                   = 1521,
    /// SW-RECORD-LAYOUT-V-PROP
    SwRecordLayoutVProp                                                    = 1835,
    /// SW-REFRESH-TIMING
    SwRefreshTiming                                                        = 5491,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 427,
    /// SW-SERVICE-IMPL-POLICY
    SwServiceImplPolicy                                                    = 152,
    /// SW-SYSCOND
    SwSyscond                                                              = 3642,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 3301,
    /// SW-SYSTEMCONST-REF
    SwSystemconstRef                                                       = 6392,
    /// SW-SYSTEMCONST-VALUE
    SwSystemconstValue                                                     = 1399,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 2727,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REF
    SwSystemconstantValueSetRef                                            = 5234,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REFS
    SwSystemconstantValueSetRefs                                           = 2501,
    /// SW-SYSTEMCONSTANT-VALUES
    SwSystemconstantValues                                                 = 1279,
    /// SW-TEST-DESC
    SwTestDesc                                                             = 3799,
    /// SW-TEST-DESCS
    SwTestDescs                                                            = 40,
    /// SW-TEXT-PROPS
    SwTextProps                                                            = 5896,
    /// SW-VALUE-BLOCK-SIZE
    SwValueBlockSize                                                       = 4460,
    /// SW-VALUE-BLOCK-SIZE-MULTS
    SwValueBlockSizeMults                                                  = 2350,
    /// SW-VALUE-CONT
    SwValueCont                                                            = 1456,
    /// SW-VALUES-PHYS
    SwValuesPhys                                                           = 4566,
    /// SW-VARIABLE-IMPL
    SwVariableImpl                                                         = 5590,
    /// SW-VARIABLE-IMPLS
    SwVariableImpls                                                        = 4094,
    /// SW-VARIABLE-PROTOTYPE-REF
    SwVariablePrototypeRef                                                 = 3060,
    /// SW-VARIABLE-REFS
    SwVariableRefs                                                         = 1492,
    /// SW-VERSION
    SwVersion                                                              = 797,
    /// SWC-BEHAVIOR-REF
    SwcBehaviorRef                                                         = 6434,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 1857,
    /// SWC-BSW-MAPPING-REF
    SwcBswMappingRef                                                       = 5448,
    /// SWC-BSW-RUNNABLE-MAPPING
    SwcBswRunnableMapping                                                  = 1078,
    /// SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE
    SwcBswSynchronizedModeGroupPrototype                                   = 4055,
    /// SWC-BSW-SYNCHRONIZED-TRIGGER
    SwcBswSynchronizedTrigger                                              = 131,
    /// SWC-EXCLUSIVE-AREA-POLICY
    SwcExclusiveAreaPolicy                                                 = 60,
    /// SWC-FLAT-SERVICE-DEPENDENCY-REF
    SwcFlatServiceDependencyRef                                            = 3140,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 3736,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 1367,
    /// SWC-MAPPING-CONSTRAINT-REF
    SwcMappingConstraintRef                                                = 6465,
    /// SWC-MAPPING-CONSTRAINT-REFS
    SwcMappingConstraintRefs                                               = 3785,
    /// SWC-MODE-GROUP-IREF
    SwcModeGroupIref                                                       = 1662,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 1911,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 3914,
    /// SWC-MODE-SWITCH-EVENT-REF
    SwcModeSwitchEventRef                                                  = 4205,
    /// SWC-RUNNABLE-REF
    SwcRunnableRef                                                         = 487,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 5018,
    /// SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    SwcServiceDependencyInExecutableIref                                   = 6354,
    /// SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    SwcServiceDependencyInSystemIref                                       = 4981,
    /// SWC-SERVICE-DEPENDENCY-IREF
    SwcServiceDependencyIref                                               = 1085,
    /// SWC-TIMING
    SwcTiming                                                              = 602,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 2092,
    /// SWC-TO-APPLICATION-PARTITION-MAPPINGS
    SwcToApplicationPartitionMappings                                      = 6511,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 5368,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT
    SwcToEcuMappingConstraint                                              = 1315,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE
    SwcToEcuMappingConstraintType                                          = 6439,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 5031,
    /// SWC-TO-SWC-OPERATION-ARGUMENTS
    SwcToSwcOperationArguments                                             = 5810,
    /// SWC-TO-SWC-SIGNAL
    SwcToSwcSignal                                                         = 321,
    /// SWC-TRIGGER-IREF
    SwcTriggerIref                                                         = 4970,
    /// SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY
    SwitchAsynchronousTrafficShaperGroupEntry                              = 3448,
    /// SWITCH-ATS-INSTANCE-ENTRY
    SwitchAtsInstanceEntry                                                 = 4352,
    /// SWITCH-FLOW-METERING-ENTRY
    SwitchFlowMeteringEntry                                                = 2707,
    /// SWITCH-MAC-ADDRESS-LEARNING-MODE
    SwitchMacAddressLearningMode                                           = 4914,
    /// SWITCH-STREAM-FILTER-ENTRY
    SwitchStreamFilterEntry                                                = 3237,
    /// SWITCH-STREAM-GATE-ENTRY
    SwitchStreamGateEntry                                                  = 5495,
    /// SWITCH-STREAM-IDENTIFICATION
    SwitchStreamIdentification                                             = 996,
    /// SWITCH-STREAM-IDENTIFICATIONS
    SwitchStreamIdentifications                                            = 5845,
    /// SYMBOL
    Symbol                                                                 = 2703,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 1193,
    /// SYMBOL-WINDOW
    SymbolWindow                                                           = 6090,
    /// SYMBOL-WINDOW-ACTION-POINT-OFFSET
    SymbolWindowActionPointOffset                                          = 3127,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 621,
    /// SYMBOLIC-NAME-VALUE
    SymbolicNameValue                                                      = 3603,
    /// SYNC-CONFIRMATION-TIMEOUT
    SyncConfirmationTimeout                                                = 3196,
    /// SYNC-COUNTER-INIT
    SyncCounterInit                                                        = 3532,
    /// SYNC-DATA-ID-LIST
    SyncDataIdList                                                         = 6132,
    /// SYNC-DATA-ID-LISTS
    SyncDataIdLists                                                        = 4060,
    /// SYNC-FRAME-ID-COUNT-MAX
    SyncFrameIdCountMax                                                    = 1536,
    /// SYNC-INTERVAL
    SyncInterval                                                           = 5759,
    /// SYNC-JUMP-WIDTH
    SyncJumpWidth                                                          = 3701,
    /// SYNC-LOSS-THRESHOLD
    SyncLossThreshold                                                      = 3808,
    /// SYNC-LOSS-TIMEOUT
    SyncLossTimeout                                                        = 5923,
    /// SYNC-NODE-MAX
    SyncNodeMax                                                            = 1926,
    /// SYNC-PERIOD
    SyncPeriod                                                             = 5557,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 702,
    /// SYNCHRONIZATION-CONSTRAINT-TYPE
    SynchronizationConstraintType                                          = 1333,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 679,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 2570,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 5544,
    /// SYNCHRONIZED-MODE-GROUPS
    SynchronizedModeGroups                                                 = 6081,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 4231,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 1270,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 6418,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 2190,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 320,
    /// SYNCHRONIZED-TRIGGERS
    SynchronizedTriggers                                                   = 4323,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 766,
    /// SYSC-REF
    SyscRef                                                                = 6273,
    /// SYSC-STRING-REF
    SyscStringRef                                                          = 2972,
    /// SYSTEM
    System                                                                 = 6419,
    /// SYSTEM-COM-SPEC-DEFINITION-REF
    SystemComSpecDefinitionRef                                             = 3078,
    /// SYSTEM-COM-SPEC-DEFINITION-REFS
    SystemComSpecDefinitionRefs                                            = 5354,
    /// SYSTEM-COM-SPEC-DEFINITION-SET
    SystemComSpecDefinitionSet                                             = 1766,
    /// SYSTEM-DOCUMENTATIONS
    SystemDocumentations                                                   = 1384,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 27,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 298,
    /// SYSTEM-MEMORY-USAGES
    SystemMemoryUsages                                                     = 3901,
    /// SYSTEM-REF
    SystemRef                                                              = 2679,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 2769,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 6361,
    /// SYSTEM-SIGNAL-GROUP-REF
    SystemSignalGroupRef                                                   = 4392,
    /// SYSTEM-SIGNAL-GROUP-TO-COM-RESOURCE-MAPPINGS
    SystemSignalGroupToComResourceMappings                                 = 5337,
    /// SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalGroupToCommunicationResourceMapping                        = 5613,
    /// SYSTEM-SIGNAL-REF
    SystemSignalRef                                                        = 1136,
    /// SYSTEM-SIGNAL-REFS
    SystemSignalRefs                                                       = 5166,
    /// SYSTEM-SIGNAL-TO-COM-RESOURCE-MAPPINGS
    SystemSignalToComResourceMappings                                      = 5295,
    /// SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalToCommunicationResourceMapping                             = 1718,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 675,
    /// SYSTEM-TRIGGERED-EVENTS
    SystemTriggeredEvents                                                  = 4568,
    /// SYSTEM-VERSION
    SystemVersion                                                          = 2617,
    /// TA-TYPE
    TaType                                                                 = 714,
    /// TABLE
    Table                                                                  = 6644,
    /// TABLE-CAPTION
    TableCaption                                                           = 982,
    /// TABLE-ENTRYS
    TableEntrys                                                            = 2293,
    /// TAG-ID
    TagId                                                                  = 5986,
    /// TAG-WITH-OPTIONAL-VALUE
    TagWithOptionalValue                                                   = 2970,
    /// TAILORING-REF
    TailoringRef                                                           = 3039,
    /// TAILORING-REFS
    TailoringRefs                                                          = 137,
    /// TARGET-APPLICATION-RECORD-ELEMENT-REF
    TargetApplicationRecordElementRef                                      = 5573,
    /// TARGET-CATEGORY
    TargetCategory                                                         = 5548,
    /// TARGET-CHANNEL
    TargetChannel                                                          = 776,
    /// TARGET-COMPONENT-REF
    TargetComponentRef                                                     = 5775,
    /// TARGET-DATA-ELEMENT-REF
    TargetDataElementRef                                                   = 5499,
    /// TARGET-DATA-PROTOTYPE-GROUP-REF
    TargetDataPrototypeGroupRef                                            = 4044,
    /// TARGET-DATA-PROTOTYPE-IN-CS-REF
    TargetDataPrototypeInCsRef                                             = 4234,
    /// TARGET-DATA-PROTOTYPE-IN-SR-REF
    TargetDataPrototypeInSrRef                                             = 5834,
    /// TARGET-DATA-PROTOTYPE-REF
    TargetDataPrototypeRef                                                 = 6373,
    /// TARGET-DATA-PROTOYPE-REF
    TargetDataProtoypeRef                                                  = 2400,
    /// TARGET-EEC-REF
    TargetEecRef                                                           = 3713,
    /// TARGET-EEC-REFS
    TargetEecRefs                                                          = 874,
    /// TARGET-ELEMENT-REF
    TargetElementRef                                                       = 4442,
    /// TARGET-EVENT-REF
    TargetEventRef                                                         = 833,
    /// TARGET-EVENT-REFS
    TargetEventRefs                                                        = 2226,
    /// TARGET-FIELD-REF
    TargetFieldRef                                                         = 6268,
    /// TARGET-FRAME-REF
    TargetFrameRef                                                         = 521,
    /// TARGET-HEALTH-CHANNEL-REF
    TargetHealthChannelRef                                                 = 6539,
    /// TARGET-I-PDU
    TargetIPdu                                                             = 2367,
    /// TARGET-I-PDU-REF
    TargetIPduRef                                                          = 2587,
    /// TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TargetImplementationDataTypeElementRef                                 = 5384,
    /// TARGET-METHOD-REF
    TargetMethodRef                                                        = 3415,
    /// TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    TargetModeDeclarationGroupPrototypeRef                                 = 4672,
    /// TARGET-MODE-DECLARATION-REF
    TargetModeDeclarationRef                                               = 6590,
    /// TARGET-MODE-GROUP-REF
    TargetModeGroupRef                                                     = 3257,
    /// TARGET-MODE-REF
    TargetModeRef                                                          = 4229,
    /// TARGET-MODULE-ID
    TargetModuleId                                                         = 3766,
    /// TARGET-MODULE-REFS
    TargetModuleRefs                                                       = 6540,
    /// TARGET-OPERATION-REF
    TargetOperationRef                                                     = 4830,
    /// TARGET-P-PORT-PROTOTYPE-REF
    TargetPPortPrototypeRef                                                = 1927,
    /// TARGET-P-PORT-REF
    TargetPPortRef                                                         = 328,
    /// TARGET-PARAMETER-DATA-PROTOTYPE-REF
    TargetParameterDataPrototypeRef                                        = 3990,
    /// TARGET-PDU-TRIGGERINGS
    TargetPduTriggerings                                                   = 877,
    /// TARGET-PHM-CHECKPOINT-REF
    TargetPhmCheckpointRef                                                 = 5461,
    /// TARGET-PHM-SUPERVISED-ENTITY-IREF
    TargetPhmSupervisedEntityIref                                          = 783,
    /// TARGET-PORT-PROTOTYPE-REF
    TargetPortPrototypeRef                                                 = 1217,
    /// TARGET-POSSIBLE-ERROR-REF
    TargetPossibleErrorRef                                                 = 472,
    /// TARGET-PROVIDED-OPERATION-REF
    TargetProvidedOperationRef                                             = 3412,
    /// TARGET-R-PORT-PROTOTYPE-REF
    TargetRPortPrototypeRef                                                = 2673,
    /// TARGET-R-PORT-REF
    TargetRPortRef                                                         = 4093,
    /// TARGET-REF
    TargetRef                                                              = 3993,
    /// TARGET-REFS
    TargetRefs                                                             = 3118,
    /// TARGET-REQUIRED-OPERATION-REF
    TargetRequiredOperationRef                                             = 5209,
    /// TARGET-RTE-EVENT-REF
    TargetRteEventRef                                                      = 5519,
    /// TARGET-RUNNABLE-ENTITY-GROUP-REF
    TargetRunnableEntityGroupRef                                           = 326,
    /// TARGET-RUNNABLE-ENTITY-REF
    TargetRunnableEntityRef                                                = 1096,
    /// TARGET-SIGNAL-REF
    TargetSignalRef                                                        = 4804,
    /// TARGET-STATUS-REF
    TargetStatusRef                                                        = 3984,
    /// TARGET-SWC-SERVICE-DEPENDENCY-REF
    TargetSwcServiceDependencyRef                                          = 4312,
    /// TARGET-TRIGGER-REF
    TargetTriggerRef                                                       = 343,
    /// TARGET-TYPE
    TargetType                                                             = 5267,
    /// TARGET-VARIABLE-ACCESS-REF
    TargetVariableAccessRef                                                = 63,
    /// TARGET-VARIABLE-DATA-PROTOTYPE-REF
    TargetVariableDataPrototypeRef                                         = 4727,
    /// TASK
    Task                                                                   = 1599,
    /// TBODY
    Tbody                                                                  = 4487,
    /// TCP-ALIVE-CHECK-RESPONSE-TIMEOUT
    TcpAliveCheckResponseTimeout                                           = 6599,
    /// TCP-CONGESTION-AVOIDANCE-ENABLED
    TcpCongestionAvoidanceEnabled                                          = 5669,
    /// TCP-CONNECT-TIMEOUT
    TcpConnectTimeout                                                      = 2416,
    /// TCP-DELAYED-ACK-TIMEOUT
    TcpDelayedAckTimeout                                                   = 4653,
    /// TCP-FAST-RECOVERY-ENABLED
    TcpFastRecoveryEnabled                                                 = 883,
    /// TCP-FAST-RETRANSMIT-ENABLED
    TcpFastRetransmitEnabled                                               = 1237,
    /// TCP-FIN-WAIT-2-TIMEOUT
    TcpFinWait2Timeout                                                     = 3517,
    /// TCP-GENERAL-INACTIVITY-TIME
    TcpGeneralInactivityTime                                               = 6235,
    /// TCP-INITIAL-INACTIVITY-TIME
    TcpInitialInactivityTime                                               = 1877,
    /// TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP
    TcpIpArpNumGratuitousArpOnStartup                                      = 5391,
    /// TCP-IP-ARP-PACKET-QUEUE-ENABLED
    TcpIpArpPacketQueueEnabled                                             = 3426,
    /// TCP-IP-ARP-REQUEST-TIMEOUT
    TcpIpArpRequestTimeout                                                 = 4677,
    /// TCP-IP-ARP-TABLE-ENTRY-TIMEOUT
    TcpIpArpTableEntryTimeout                                              = 3,
    /// TCP-IP-AUTO-IP-INIT-TIMEOUT
    TcpIpAutoIpInitTimeout                                                 = 2836,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MAX
    TcpIpDhcpV6CnfDelayMax                                                 = 3802,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MIN
    TcpIpDhcpV6CnfDelayMin                                                 = 16,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MAX
    TcpIpDhcpV6InfDelayMax                                                 = 5027,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MIN
    TcpIpDhcpV6InfDelayMin                                                 = 2216,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MAX
    TcpIpDhcpV6SolDelayMax                                                 = 6006,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MIN
    TcpIpDhcpV6SolDelayMin                                                 = 4618,
    /// TCP-IP-ICMP-PROPS-REF
    TcpIpIcmpPropsRef                                                      = 2177,
    /// TCP-IP-ICMP-PROPS-REFS
    TcpIpIcmpPropsRefs                                                     = 6549,
    /// TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED
    TcpIpIcmpV4EchoReplyEnabled                                            = 4520,
    /// TCP-IP-ICMP-V-4-TTL
    TcpIpIcmpV4Ttl                                                         = 6258,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION
    TcpIpIcmpV6EchoReplyAvoidFragmentation                                 = 156,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED
    TcpIpIcmpV6EchoReplyEnabled                                            = 5781,
    /// TCP-IP-ICMP-V-6-HOP-LIMIT
    TcpIpIcmpV6HopLimit                                                    = 6442,
    /// TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED
    TcpIpIcmpV6MsgDestinationUnreachableEnabled                            = 149,
    /// TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED
    TcpIpIcmpV6MsgParameterProblemEnabled                                  = 5088,
    /// TCP-IP-IP-FRAGMENTATION-RX-ENABLED
    TcpIpIpFragmentationRxEnabled                                          = 1200,
    /// TCP-IP-IP-NUM-FRAGMENTS
    TcpIpIpNumFragments                                                    = 789,
    /// TCP-IP-IP-NUM-REASS-DGRAMS
    TcpIpIpNumReassDgrams                                                  = 169,
    /// TCP-IP-IP-REASS-TIMEOUT
    TcpIpIpReassTimeout                                                    = 1901,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-COUNT
    TcpIpIpReassemblyBufferCount                                           = 6252,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-SIZE
    TcpIpIpReassemblyBufferSize                                            = 4373,
    /// TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT
    TcpIpIpReassemblySegmentCount                                          = 5763,
    /// TCP-IP-IP-REASSEMBLY-TIMEOUT
    TcpIpIpReassemblyTimeout                                               = 1795,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT
    TcpIpIpTxFragmentBufferCount                                           = 2622,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE
    TcpIpIpTxFragmentBufferSize                                            = 3763,
    /// TCP-IP-NDP-DEFAULT-REACHABLE-TIME
    TcpIpNdpDefaultReachableTime                                           = 2595,
    /// TCP-IP-NDP-DEFAULT-RETRANS-TIMER
    TcpIpNdpDefaultRetransTimer                                            = 2439,
    /// TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE
    TcpIpNdpDefaultRouterListSize                                          = 6027,
    /// TCP-IP-NDP-DEFENSIVE-PROCESSING
    TcpIpNdpDefensiveProcessing                                            = 251,
    /// TCP-IP-NDP-DELAY-FIRST-PROBE-TIME
    TcpIpNdpDelayFirstProbeTime                                            = 4872,
    /// TCP-IP-NDP-DELAY-FIRST-PROBE-TIME-VALUE
    TcpIpNdpDelayFirstProbeTimeValue                                       = 1784,
    /// TCP-IP-NDP-DESTINATION-CACHE-SIZE
    TcpIpNdpDestinationCacheSize                                           = 6033,
    /// TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED
    TcpIpNdpDynamicHopLimitEnabled                                         = 3998,
    /// TCP-IP-NDP-DYNAMIC-MTU-ENABLED
    TcpIpNdpDynamicMtuEnabled                                              = 3600,
    /// TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED
    TcpIpNdpDynamicReachableTimeEnabled                                    = 1781,
    /// TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED
    TcpIpNdpDynamicRetransTimeEnabled                                      = 3314,
    /// TCP-IP-NDP-MAX-RANDOM-FACTOR
    TcpIpNdpMaxRandomFactor                                                = 2348,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY
    TcpIpNdpMaxRtrSolicitationDelay                                        = 4469,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATIONS
    TcpIpNdpMaxRtrSolicitations                                            = 3791,
    /// TCP-IP-NDP-MIN-RANDOM-FACTOR
    TcpIpNdpMinRandomFactor                                                = 5640,
    /// TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED
    TcpIpNdpNeighborUnreachabilityDetectionEnabled                         = 5959,
    /// TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS
    TcpIpNdpNumMulticastSolicitations                                      = 2093,
    /// TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS
    TcpIpNdpNumUnicastSolicitations                                        = 3850,
    /// TCP-IP-NDP-PACKET-QUEUE-ENABLED
    TcpIpNdpPacketQueueEnabled                                             = 4255,
    /// TCP-IP-NDP-PREFIX-LIST-SIZE
    TcpIpNdpPrefixListSize                                                 = 2371,
    /// TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED
    TcpIpNdpRandomReachableTimeEnabled                                     = 3108,
    /// TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED
    TcpIpNdpRndRtrSolicitationDelayEnabled                                 = 3088,
    /// TCP-IP-NDP-RTR-SOLICITATION-INTERVAL
    TcpIpNdpRtrSolicitationInterval                                        = 416,
    /// TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS
    TcpIpNdpSlaacDadNumberOfTransmissions                                  = 1145,
    /// TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY
    TcpIpNdpSlaacDadRetransmissionDelay                                    = 6491,
    /// TCP-IP-NDP-SLAAC-DELAY-ENABLED
    TcpIpNdpSlaacDelayEnabled                                              = 4700,
    /// TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED
    TcpIpNdpSlaacOptimisticDadEnabled                                      = 6094,
    /// TCP-IP-PROPS-REF
    TcpIpPropsRef                                                          = 3145,
    /// TCP-IP-PROPS-REFS
    TcpIpPropsRefs                                                         = 3190,
    /// TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC
    TcpIpTlsUseSecurityExtensionForceEncryptThenMac                        = 2343,
    /// TCP-KEEP-ALIVE-ENABLED
    TcpKeepAliveEnabled                                                    = 3913,
    /// TCP-KEEP-ALIVE-INTERVAL
    TcpKeepAliveInterval                                                   = 605,
    /// TCP-KEEP-ALIVE-PROBES-MAX
    TcpKeepAliveProbesMax                                                  = 6431,
    /// TCP-KEEP-ALIVE-TIME
    TcpKeepAliveTime                                                       = 339,
    /// TCP-LOCAL-PORT
    TcpLocalPort                                                           = 2232,
    /// TCP-MAX-RTX
    TcpMaxRtx                                                              = 5734,
    /// TCP-MSL
    TcpMsl                                                                 = 4850,
    /// TCP-NAGLE-ENABLED
    TcpNagleEnabled                                                        = 4113,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 1223,
    /// TCP-OPTION-FILTER-LISTS
    TcpOptionFilterLists                                                   = 1608,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 6473,
    /// TCP-PORT
    TcpPort                                                                = 2743,
    /// TCP-PORT-REF
    TcpPortRef                                                             = 6596,
    /// TCP-PROPS
    TcpProps                                                               = 3536,
    /// TCP-RECEIVE-WINDOW-MAX
    TcpReceiveWindowMax                                                    = 5060,
    /// TCP-REMOTE-PORT
    TcpRemotePort                                                          = 3989,
    /// TCP-RETRANSMISSION-TIMEOUT
    TcpRetransmissionTimeout                                               = 4548,
    /// TCP-ROLE
    TcpRole                                                                = 393,
    /// TCP-RULE
    TcpRule                                                                = 4145,
    /// TCP-SLOW-START-ENABLED
    TcpSlowStartEnabled                                                    = 5463,
    /// TCP-SYN-MAX-RTX
    TcpSynMaxRtx                                                           = 1769,
    /// TCP-SYN-RECEIVED-TIMEOUT
    TcpSynReceivedTimeout                                                  = 655,
    /// TCP-TP
    TcpTp                                                                  = 3223,
    /// TCP-TP-CONFIG
    TcpTpConfig                                                            = 5733,
    /// TCP-TP-PORT
    TcpTpPort                                                              = 3373,
    /// TCP-TTL
    TcpTtl                                                                 = 2005,
    /// TCP-UDP-CONFIG
    TcpUdpConfig                                                           = 5289,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 6111,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 712,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 4852,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS
    TdCpSoftwareClusterResourceToTdMappings                                = 2628,
    /// TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS
    TdCpSoftwareClusterToTdMappings                                        = 3837,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 4266,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE
    TdEventBswInternalBehaviorType                                         = 1256,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 4035,
    /// TD-EVENT-BSW-MODE-DECLARATION-TYPE
    TdEventBswModeDeclarationType                                          = 1048,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 531,
    /// TD-EVENT-BSW-MODULE-TYPE
    TdEventBswModuleType                                                   = 139,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 5828,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 6355,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 6082,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 940,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 6602,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 5232,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 2485,
    /// TD-EVENT-MODE-DECLARATION-TYPE
    TdEventModeDeclarationType                                             = 5278,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 2460,
    /// TD-EVENT-OPERATION-TYPE
    TdEventOperationType                                                   = 2868,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 6460,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE
    TdEventServiceInstanceDiscoveryType                                    = 5694,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 1001,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE
    TdEventServiceInstanceEventType                                        = 4730,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 3918,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE
    TdEventServiceInstanceFieldType                                        = 1567,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 4374,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE
    TdEventServiceInstanceMethodType                                       = 4632,
    /// TD-EVENT-SLLET-PORT
    TdEventSlletPort                                                       = 2683,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 1423,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 1895,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE
    TdEventSwcInternalBehaviorType                                         = 2563,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 685,
    /// TD-EVENT-TRIGGER-TYPE
    TdEventTriggerType                                                     = 1542,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 2027,
    /// TD-EVENT-TYPE
    TdEventType                                                            = 1255,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 2575,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE
    TdEventVariableDataPrototypeType                                       = 5406,
    /// TD-EVENT-VFB-PORT-GROUP
    TdEventVfbPortGroup                                                    = 4978,
    /// TD-EVENT-VFB-PORT-REF
    TdEventVfbPortRef                                                      = 5073,
    /// TD-EVENT-VFB-PORT-REFS
    TdEventVfbPortRefs                                                     = 5534,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 2736,
    /// TD-HEADER-ID-FILTERS
    TdHeaderIdFilters                                                      = 1359,
    /// TD-HEADER-ID-RANGE
    TdHeaderIdRange                                                        = 3873,
    /// TD-PDU-TRIGGERING-FILTER-REF
    TdPduTriggeringFilterRef                                               = 3119,
    /// TD-PDU-TRIGGERING-FILTER-REFS
    TdPduTriggeringFilterRefs                                              = 5503,
    /// TDLET-ZONE-CLOCK
    TdletZoneClock                                                         = 4586,
    /// TEAR-DOWN-ACTION-REF
    TearDownActionRef                                                      = 1371,
    /// TEAR-DOWN-ACTION-REFS
    TearDownActionRefs                                                     = 1076,
    /// TEMPERATURE-EXP
    TemperatureExp                                                         = 1813,
    /// TEMPLATE-ARGUMENTS
    TemplateArguments                                                      = 1345,
    /// TEMPLATE-TYPE-REF
    TemplateTypeRef                                                        = 3252,
    /// TERMINATE
    Terminate                                                              = 109,
    /// TERMINATING-CHECKPOINT-REF
    TerminatingCheckpointRef                                               = 1080,
    /// TERMINATING-CHECKPOINT-TIMEOUT-UNTIL-TERMINATION
    TerminatingCheckpointTimeoutUntilTermination                           = 4670,
    /// TERMINATION-BEHAVIOR
    TerminationBehavior                                                    = 2169,
    /// TEST-ID
    TestId                                                                 = 6404,
    /// TEST-ID-REF
    TestIdRef                                                              = 6161,
    /// TEST-IDENTIFIER
    TestIdentifier                                                         = 3722,
    /// TEST-PATTERN
    TestPattern                                                            = 512,
    /// TEST-RESULT-REF
    TestResultRef                                                          = 2331,
    /// TESTED-ITEM-REF
    TestedItemRef                                                          = 520,
    /// TESTED-ITEM-REFS
    TestedItemRefs                                                         = 4083,
    /// TEX-MATH
    TexMath                                                                = 5799,
    /// TEXT
    Text                                                                   = 3100,
    /// TEXT-TABLE-MAPPING
    TextTableMapping                                                       = 323,
    /// TEXT-TABLE-MAPPINGS
    TextTableMappings                                                      = 3930,
    /// TEXT-TABLE-VALUE-PAIR
    TextTableValuePair                                                     = 2592,
    /// TEXT-VALUE-SPECIFICATION
    TextValueSpecification                                                 = 5963,
    /// TEXTUAL-CONDITION
    TextualCondition                                                       = 1317,
    /// TFOOT
    Tfoot                                                                  = 1653,
    /// TGROUP
    Tgroup                                                                 = 396,
    /// THEAD
    Thead                                                                  = 6636,
    /// THRESHOLD
    Threshold                                                              = 2580,
    /// THRESHOLD-NUMBER
    ThresholdNumber                                                        = 1738,
    /// THRESHOLD-SIZE
    ThresholdSize                                                          = 1885,
    /// TIME-BASE
    TimeBase                                                               = 2044,
    /// TIME-BASE-CONSUMER-REF
    TimeBaseConsumerRef                                                    = 6381,
    /// TIME-BASE-JITTER
    TimeBaseJitter                                                         = 5522,
    /// TIME-BASE-KIND
    TimeBaseKind                                                           = 2960,
    /// TIME-BASE-PROVIDER-REF
    TimeBaseProviderRef                                                    = 4809,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 175,
    /// TIME-BASE-RESOURCE-REF
    TimeBaseResourceRef                                                    = 1248,
    /// TIME-BASE-RESOURCE-REF-CONDITIONAL
    TimeBaseResourceRefConditional                                         = 3081,
    /// TIME-BASE-RESOURCE-REFS
    TimeBaseResourceRefs                                                   = 4191,
    /// TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE
    TimeBasedFdcThresholdStorageValue                                      = 275,
    /// TIME-BASES
    TimeBases                                                              = 5189,
    /// TIME-BR
    TimeBr                                                                 = 1347,
    /// TIME-BUFFER
    TimeBuffer                                                             = 6605,
    /// TIME-CS
    TimeCs                                                                 = 2040,
    /// TIME-EXP
    TimeExp                                                                = 2989,
    /// TIME-FAILED-THRESHOLD
    TimeFailedThreshold                                                    = 1440,
    /// TIME-FR-IF
    TimeFrIf                                                               = 6307,
    /// TIME-HARDWARE-CORRECTION-THRESHOLD
    TimeHardwareCorrectionThreshold                                        = 129,
    /// TIME-INTERVAL
    TimeInterval                                                           = 4427,
    /// TIME-LEAP-FUTURE-THRESHOLD
    TimeLeapFutureThreshold                                                = 524,
    /// TIME-LEAP-HEALING-COUNTER
    TimeLeapHealingCounter                                                 = 6037,
    /// TIME-LEAP-PAST-THRESHOLD
    TimeLeapPastThreshold                                                  = 4449,
    /// TIME-MARK
    TimeMark                                                               = 565,
    /// TIME-MASTER-PRIORITY
    TimeMasterPriority                                                     = 1361,
    /// TIME-OFFSET
    TimeOffset                                                             = 1562,
    /// TIME-PASSED-THRESHOLD
    TimePassedThreshold                                                    = 2597,
    /// TIME-PERIOD
    TimePeriod                                                             = 4201,
    /// TIME-SEG-1
    TimeSeg1                                                               = 5538,
    /// TIME-SEG-2
    TimeSeg2                                                               = 510,
    /// TIME-SUB-TLV
    TimeSubTlv                                                             = 176,
    /// TIME-SYNC-CLIENT
    TimeSyncClient                                                         = 4331,
    /// TIME-SYNC-CORRECTION
    TimeSyncCorrection                                                     = 2904,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 5652,
    /// TIME-SYNC-P-PORT-PROTOTYPE-IREF
    TimeSyncPPortPrototypeIref                                             = 166,
    /// TIME-SYNC-PORT-PROTOTYPE-IREF
    TimeSyncPortPrototypeIref                                              = 3520,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 5739,
    /// TIME-SYNC-R-PORT-PROTOTYPE-IREF
    TimeSyncRPortPrototypeIref                                             = 5604,
    /// TIME-SYNC-SERVER
    TimeSyncServer                                                         = 4778,
    /// TIME-SYNC-SERVER-IDENTIFIER
    TimeSyncServerIdentifier                                               = 2419,
    /// TIME-SYNC-SERVER-REF
    TimeSyncServerRef                                                      = 4281,
    /// TIME-SYNC-TECHNOLOGY
    TimeSyncTechnology                                                     = 4536,
    /// TIME-SYNCHRONIZATION
    TimeSynchronization                                                    = 1362,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 5991,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 3361,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 65,
    /// TIME-TO-LIVE
    TimeToLive                                                             = 2718,
    /// TIME-TRIGGERED-CAN-LEVEL
    TimeTriggeredCanLevel                                                  = 1133,
    /// TIME-VALUE
    TimeValue                                                              = 1364,
    /// TIME-VALUE-VALUE-VARIATION-POINT
    TimeValueValueVariationPoint                                           = 4790,
    /// TIMEOUT
    Timeout                                                                = 1419,
    /// TIMEOUT-AR
    TimeoutAr                                                              = 3080,
    /// TIMEOUT-AS
    TimeoutAs                                                              = 6190,
    /// TIMEOUT-BR
    TimeoutBr                                                              = 5178,
    /// TIMEOUT-BS
    TimeoutBs                                                              = 3662,
    /// TIMEOUT-CHECK
    TimeoutCheck                                                           = 3219,
    /// TIMEOUT-CR
    TimeoutCr                                                              = 918,
    /// TIMEOUT-CS
    TimeoutCs                                                              = 4856,
    /// TIMEOUT-SUBSTITUTION-VALUE
    TimeoutSubstitutionValue                                               = 3686,
    /// TIMESTAMP-FORMAT
    TimestampFormat                                                        = 3956,
    /// TIMESTAMP-INTERVAL
    TimestampInterval                                                      = 4340,
    /// TIMESTAMP-RX-ACCEPTANCE-WINDOW
    TimestampRxAcceptanceWindow                                            = 3574,
    /// TIMESTAMP-SUPPORT
    TimestampSupport                                                       = 2805,
    /// TIMING-ARGUMENT-REF
    TimingArgumentRef                                                      = 1906,
    /// TIMING-ARGUMENTS
    TimingArguments                                                        = 729,
    /// TIMING-CLOCK-SYNC-ACCURACY
    TimingClockSyncAccuracy                                                = 6078,
    /// TIMING-CLOCK-SYNC-ACCURACYS
    TimingClockSyncAccuracys                                               = 2550,
    /// TIMING-CLOCKS
    TimingClocks                                                           = 2804,
    /// TIMING-CONDITION
    TimingCondition                                                        = 3755,
    /// TIMING-CONDITION-FORMULA
    TimingConditionFormula                                                 = 2982,
    /// TIMING-CONDITION-REF
    TimingConditionRef                                                     = 583,
    /// TIMING-CONDITIONS
    TimingConditions                                                       = 4907,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 3546,
    /// TIMING-DESCRIPTION-REF
    TimingDescriptionRef                                                   = 3165,
    /// TIMING-DESCRIPTIONS
    TimingDescriptions                                                     = 380,
    /// TIMING-EVENT
    TimingEvent                                                            = 5746,
    /// TIMING-EVENT-REF
    TimingEventRef                                                         = 2889,
    /// TIMING-GUARANTEES
    TimingGuarantees                                                       = 3393,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 4213,
    /// TIMING-MODE-REF
    TimingModeRef                                                          = 1434,
    /// TIMING-MODES
    TimingModes                                                            = 98,
    /// TIMING-PROPS
    TimingProps                                                            = 1209,
    /// TIMING-REQUIREMENTS
    TimingRequirements                                                     = 4245,
    /// TIMING-RESOURCE
    TimingResource                                                         = 2031,
    /// TIMING-VARIABLE-REF
    TimingVariableRef                                                      = 2957,
    /// TIMING-VARIABLES
    TimingVariables                                                        = 4246,
    /// TLS-CIPHER-SUITE
    TlsCipherSuite                                                         = 5732,
    /// TLS-CIPHER-SUITES
    TlsCipherSuites                                                        = 4214,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 3509,
    /// TLS-CONNECTION-REF
    TlsConnectionRef                                                       = 2491,
    /// TLS-CONNECTION-REFS
    TlsConnectionRefs                                                      = 3529,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 4243,
    /// TLS-CRYPTO-MAPPING-REF
    TlsCryptoMappingRef                                                    = 717,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 3325,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 3613,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 5422,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 271,
    /// TLS-JOB-MAPPINGS
    TlsJobMappings                                                         = 3960,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 3382,
    /// TLS-JOB-REQUIREMENT-REF
    TlsJobRequirementRef                                                   = 4873,
    /// TLS-JOB-SEMANTIC
    TlsJobSemantic                                                         = 2414,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 6367,
    /// TLS-SECURE-COM-PROPS-REF
    TlsSecureComPropsRef                                                   = 6340,
    /// TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF
    TlvAppRecordElementInMethodIref                                        = 5938,
    /// TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    TlvAppRecordElementInServiceInterfaceIref                              = 159,
    /// TLV-ARGUMENT-REF
    TlvArgumentRef                                                         = 3365,
    /// TLV-DATA-ID
    TlvDataId                                                              = 2071,
    /// TLV-DATA-ID-0-REF
    TlvDataId0Ref                                                          = 5500,
    /// TLV-DATA-ID-0-REFS
    TlvDataId0Refs                                                         = 13,
    /// TLV-DATA-ID-DEFINITION
    TlvDataIdDefinition                                                    = 2268,
    /// TLV-DATA-ID-DEFINITION-REF
    TlvDataIdDefinitionRef                                                 = 768,
    /// TLV-DATA-ID-DEFINITION-REFS
    TlvDataIdDefinitionRefs                                                = 5429,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 1147,
    /// TLV-DATA-ID-DEFINITIONS
    TlvDataIdDefinitions                                                   = 5924,
    /// TLV-DATA-IDS
    TlvDataIds                                                             = 5349,
    /// TLV-IMPL-RECORD-ELEMENT
    TlvImplRecordElement                                                   = 3293,
    /// TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TlvImplementationDataTypeElementRef                                    = 6163,
    /// TLV-RECORD-ELEMENT-REF
    TlvRecordElementRef                                                    = 3906,
    /// TLV-SUB-ELEMENT-REF
    TlvSubElementRef                                                       = 3115,
    /// TO-REF
    ToRef                                                                  = 3329,
    /// TOL
    Tol                                                                    = 3891,
    /// TOLERANCE
    Tolerance                                                              = 749,
    /// TOLERATED-FAILED-CYCLES
    ToleratedFailedCycles                                                  = 2233,
    /// TOOL
    Tool                                                                   = 5447,
    /// TOOL-VERSION
    ToolVersion                                                            = 1614,
    /// TOPIC-1
    Topic1                                                                 = 269,
    /// TOPIC-DATA
    TopicData                                                              = 5132,
    /// TOPIC-NAME
    TopicName                                                              = 5766,
    /// TP-ADDRESS
    TpAddress                                                              = 6159,
    /// TP-ADDRESS-EXTENSION-VALUE
    TpAddressExtensionValue                                                = 5259,
    /// TP-ADDRESS-REF
    TpAddressRef                                                           = 5011,
    /// TP-ADDRESS-REFS
    TpAddressRefs                                                          = 3141,
    /// TP-ADDRESSS
    TpAddresss                                                             = 5724,
    /// TP-CHANNEL-REF
    TpChannelRef                                                           = 4466,
    /// TP-CHANNELS
    TpChannels                                                             = 1233,
    /// TP-CONCURRENT-PROCESSING-SDUS
    TpConcurrentProcessingSdus                                             = 2112,
    /// TP-CONFIGURATION
    TpConfiguration                                                        = 5256,
    /// TP-CONNECTION-CONTROL-REF
    TpConnectionControlRef                                                 = 3300,
    /// TP-CONNECTION-CONTROLS
    TpConnectionControls                                                   = 596,
    /// TP-CONNECTION-REF
    TpConnectionRef                                                        = 1974,
    /// TP-CONNECTION-REFS
    TpConnectionRefs                                                       = 3619,
    /// TP-CONNECTIONS
    TpConnections                                                          = 860,
    /// TP-ECU
    TpEcu                                                                  = 4634,
    /// TP-ECUS
    TpEcus                                                                 = 5247,
    /// TP-NODES
    TpNodes                                                                = 2384,
    /// TP-PGS
    TpPgs                                                                  = 1128,
    /// TP-PROTOCOL-TYPE
    TpProtocolType                                                         = 6522,
    /// TP-SDU-REF
    TpSduRef                                                               = 1644,
    /// TP-SDU-REFS
    TpSduRefs                                                              = 1268,
    /// TP-TECHNOLOGY
    TpTechnology                                                           = 3629,
    /// TRACE
    Trace                                                                  = 5129,
    /// TRACE-MESSAGE-REF
    TraceMessageRef                                                        = 2503,
    /// TRACE-REF
    TraceRef                                                               = 2951,
    /// TRACE-REFS
    TraceRefs                                                              = 1151,
    /// TRACE-SWITCH
    TraceSwitch                                                            = 2954,
    /// TRACE-SWITCH-CONFIG
    TraceSwitchConfig                                                      = 2379,
    /// TRACE-SWITCH-CONFIGURATION
    TraceSwitchConfiguration                                               = 5565,
    /// TRACE-SWITCH-CONFIGURATIONS
    TraceSwitchConfigurations                                              = 3148,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 3477,
    /// TRACED-FAILURES
    TracedFailures                                                         = 4617,
    /// TRAFFIC-CLASS
    TrafficClass                                                           = 121,
    /// TRAFFIC-CLASS-PREEMPTION-SUPPORT
    TrafficClassPreemptionSupport                                          = 4499,
    /// TRAFFIC-LIMITATION-FILTERS
    TrafficLimitationFilters                                               = 5014,
    /// TRAFFIC-SHAPER-GROUP-REF
    TrafficShaperGroupRef                                                  = 6323,
    /// TRAFFIC-SHAPER-GROUPS
    TrafficShaperGroups                                                    = 3268,
    /// TRANCEIVER-STANDBY-DELAY
    TranceiverStandbyDelay                                                 = 220,
    /// TRANSFER-EXIT-CLASS-REF
    TransferExitClassRef                                                   = 6478,
    /// TRANSFER-PROPERTY
    TransferProperty                                                       = 5727,
    /// TRANSFER-REF
    TransferRef                                                            = 4402,
    /// TRANSFER-REFS
    TransferRefs                                                           = 1376,
    /// TRANSFERS
    Transfers                                                              = 3556,
    /// TRANSFORMATION-COM-SPEC-PROPSS
    TransformationComSpecPropss                                            = 1980,
    /// TRANSFORMATION-DESCRIPTIONS
    TransformationDescriptions                                             = 1403,
    /// TRANSFORMATION-I-SIGNAL-PROPSS
    TransformationISignalPropss                                            = 3831,
    /// TRANSFORMATION-PROPS-MAPPING-SET-REF
    TransformationPropsMappingSetRef                                       = 6269,
    /// TRANSFORMATION-PROPS-REF
    TransformationPropsRef                                                 = 2589,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 1161,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 1372,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 75,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING
    TransformationPropsToServiceInterfaceMapping                           = 2186,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 3948,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF
    TransformationPropsToServiceInterfaceMappingSetRef                     = 4021,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS
    TransformationPropsToServiceInterfaceMappings                          = 123,
    /// TRANSFORMATION-PROPSS
    TransformationPropss                                                   = 2602,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 1864,
    /// TRANSFORMATION-TECHNOLOGYS
    TransformationTechnologys                                              = 1752,
    /// TRANSFORMER-CHAIN-REF
    TransformerChainRef                                                    = 2618,
    /// TRANSFORMER-CHAIN-REFS
    TransformerChainRefs                                                   = 5187,
    /// TRANSFORMER-CLASS
    TransformerClass                                                       = 1090,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 5725,
    /// TRANSFORMER-REF
    TransformerRef                                                         = 6502,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 5105,
    /// TRANSFORMING-SYSTEM-SIGNAL-REF
    TransformingSystemSignalRef                                            = 3530,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 4712,
    /// TRANSIT-TO-INVALID-EXTENDED
    TransitToInvalidExtended                                               = 6604,
    /// TRANSITION-REF
    TransitionRef                                                          = 4358,
    /// TRANSITION-REFS
    TransitionRefs                                                         = 3934,
    /// TRANSITIONS
    Transitions                                                            = 806,
    /// TRANSLATED-VLAN-ID
    TranslatedVlanId                                                       = 2848,
    /// TRANSLATION-TARGET-IREF
    TranslationTargetIref                                                  = 3946,
    /// TRANSMISSION-ACKNOWLEDGE
    TransmissionAcknowledge                                                = 1018,
    /// TRANSMISSION-DEADLINE
    TransmissionDeadline                                                   = 4527,
    /// TRANSMISSION-MODE
    TransmissionMode                                                       = 1047,
    /// TRANSMISSION-MODE-CONDITION
    TransmissionModeCondition                                              = 1055,
    /// TRANSMISSION-MODE-CONDITIONS
    TransmissionModeConditions                                             = 1061,
    /// TRANSMISSION-MODE-DECLARATION
    TransmissionModeDeclaration                                            = 687,
    /// TRANSMISSION-MODE-FALSE-TIMING
    TransmissionModeFalseTiming                                            = 5662,
    /// TRANSMISSION-MODE-TRUE-TIMING
    TransmissionModeTrueTiming                                             = 4844,
    /// TRANSMISSION-PROPS
    TransmissionProps                                                      = 610,
    /// TRANSMISSION-START-SEQUENCE-DURATION
    TransmissionStartSequenceDuration                                      = 2300,
    /// TRANSMISSION-TRIGGER
    TransmissionTrigger                                                    = 2462,
    /// TRANSMIT-CANCELLATION
    TransmitCancellation                                                   = 1904,
    /// TRANSMIT-PDU-REF
    TransmitPduRef                                                         = 2964,
    /// TRANSMIT-PDU-REFS
    TransmitPduRefs                                                        = 619,
    /// TRANSMITTER-REF
    TransmitterRef                                                         = 4710,
    /// TRANSPORT-ERROR-COUNT-ENABLED
    TransportErrorCountEnabled                                             = 1058,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 4860,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 1348,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF
    TransportLayerIndependentInstanceIdRef                                 = 5374,
    /// TRANSPORT-LAYER-RULE
    TransportLayerRule                                                     = 3043,
    /// TRANSPORT-PDU-REF
    TransportPduRef                                                        = 3402,
    /// TRANSPORT-PLUGIN
    TransportPlugin                                                        = 2708,
    /// TRANSPORT-PLUGINS
    TransportPlugins                                                       = 4879,
    /// TRANSPORT-PRIORITY
    TransportPriority                                                      = 2009,
    /// TRANSPORT-PROTOCOL
    TransportProtocol                                                      = 2234,
    /// TRANSPORT-PROTOCOLS
    TransportProtocols                                                     = 4951,
    /// TRCV-DELAY-COMPENSATION-OFFSET
    TrcvDelayCompensationOffset                                            = 1191,
    /// TRCV-PWM-MODE-ENABLED
    TrcvPwmModeEnabled                                                     = 2324,
    /// TRIGGER
    Trigger                                                                = 410,
    /// TRIGGER-DIRECT-IMPLEMENTATIONS
    TriggerDirectImplementations                                           = 5793,
    /// TRIGGER-I-PDU-SEND-CONDITION
    TriggerIPduSendCondition                                               = 5975,
    /// TRIGGER-I-PDU-SEND-CONDITIONS
    TriggerIPduSendConditions                                              = 2855,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 3917,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 4359,
    /// TRIGGER-IREF
    TriggerIref                                                            = 280,
    /// TRIGGER-MAPPING
    TriggerMapping                                                         = 4099,
    /// TRIGGER-MAPPINGS
    TriggerMappings                                                        = 1756,
    /// TRIGGER-MODE
    TriggerMode                                                            = 4039,
    /// TRIGGER-PERIOD
    TriggerPeriod                                                          = 2796,
    /// TRIGGER-PORT-ANNOTATION
    TriggerPortAnnotation                                                  = 1639,
    /// TRIGGER-PORT-ANNOTATIONS
    TriggerPortAnnotations                                                 = 1443,
    /// TRIGGER-REF
    TriggerRef                                                             = 3479,
    /// TRIGGER-REF-CONDITIONAL
    TriggerRefConditional                                                  = 635,
    /// TRIGGER-REFS
    TriggerRefs                                                            = 5985,
    /// TRIGGER-TO-SIGNAL-MAPPING
    TriggerToSignalMapping                                                 = 2318,
    /// TRIGGERING-EVENT-REF
    TriggeringEventRef                                                     = 626,
    /// TRIGGERS
    Triggers                                                               = 3207,
    /// TRIGGERS-RECOVERY-NOTIFICATION
    TriggersRecoveryNotification                                           = 5902,
    /// TROUBLE-CODE-J-1939-REF
    TroubleCodeJ1939Ref                                                    = 3285,
    /// TROUBLE-CODE-OBD-REF
    TroubleCodeObdRef                                                      = 6553,
    /// TROUBLE-CODE-PROPSS
    TroubleCodePropss                                                      = 1486,
    /// TROUBLE-CODE-UDS-REF
    TroubleCodeUdsRef                                                      = 1909,
    /// TRUE-ACTION-LIST-REF
    TrueActionListRef                                                      = 1548,
    /// TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR
    TrustedPlatformExecutableLaunchBehavior                                = 2427,
    /// TT
    Tt                                                                     = 5113,
    /// TT-CAN-CLUSTER-REF
    TtCanClusterRef                                                        = 2125,
    /// TTCAN-ABSOLUTELY-SCHEDULED-TIMING
    TtcanAbsolutelyScheduledTiming                                         = 227,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 3694,
    /// TTCAN-CLUSTER-CONDITIONAL
    TtcanClusterConditional                                                = 2995,
    /// TTCAN-CLUSTER-VARIANTS
    TtcanClusterVariants                                                   = 1690,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 1390,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 5648,
    /// TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    TtcanCommunicationControllerConditional                                = 1487,
    /// TTCAN-COMMUNICATION-CONTROLLER-VARIANTS
    TtcanCommunicationControllerVariants                                   = 5297,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 730,
    /// TTL
    Ttl                                                                    = 671,
    /// TTL-MAX
    TtlMax                                                                 = 5396,
    /// TTL-MIN
    TtlMin                                                                 = 3597,
    /// TWO-KEY-SLOT-MODE
    TwoKeySlotMode                                                         = 119,
    /// TX-BIT-RATE-SWITCH
    TxBitRateSwitch                                                        = 831,
    /// TX-DIRECT-PDU-PRIORITY
    TxDirectPduPriority                                                    = 4567,
    /// TX-ENABLE-WINDOW-LENGTH
    TxEnableWindowLength                                                   = 6046,
    /// TX-MASK
    TxMask                                                                 = 5332,
    /// TX-NM-PDU-REF
    TxNmPduRef                                                             = 3599,
    /// TX-NM-PDU-REFS
    TxNmPduRefs                                                            = 1415,
    /// TX-PDU-POOL-REF
    TxPduPoolRef                                                           = 3758,
    /// TX-PDU-PRIORITY
    TxPduPriority                                                          = 3400,
    /// TX-PDU-TRIGGERING-REF
    TxPduTriggeringRef                                                     = 6519,
    /// TYP
    Typ                                                                    = 1356,
    /// TYPE
    Type                                                                   = 3290,
    /// TYPE-APPROVAL
    TypeApproval                                                           = 6246,
    /// TYPE-BLUEPRINTS
    TypeBlueprints                                                         = 1594,
    /// TYPE-DEFINITION
    TypeDefinition                                                         = 622,
    /// TYPE-EMITTER
    TypeEmitter                                                            = 6567,
    /// TYPE-MAPPING
    TypeMapping                                                            = 1931,
    /// TYPE-OF-DTC-SUPPORTED
    TypeOfDtcSupported                                                     = 5710,
    /// TYPE-OF-EVENT-COMBINATION-SUPPORTED
    TypeOfEventCombinationSupported                                        = 4513,
    /// TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION
    TypeOfFreezeFrameRecordNumeration                                      = 2080,
    /// TYPE-REFERENCE
    TypeReference                                                          = 5673,
    /// TYPE-REFERENCE-REF
    TypeReferenceRef                                                       = 2613,
    /// TYPE-TAILORINGS
    TypeTailorings                                                         = 6278,
    /// TYPE-TREF
    TypeTref                                                               = 3985,
    /// UAS-ID
    UasId                                                                  = 4802,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 2387,
    /// UCM-MASTER-FALLBACK-REF
    UcmMasterFallbackRef                                                   = 6335,
    /// UCM-MASTER-FALLBACK-REFS
    UcmMasterFallbackRefs                                                  = 5863,
    /// UCM-MASTER-MODULE-INSTANTIATION
    UcmMasterModuleInstantiation                                           = 3284,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 2873,
    /// UCM-MODULE-INSTANTIATION-REF
    UcmModuleInstantiationRef                                              = 357,
    /// UCM-NOT-AVAILABLE-ON-THE-NETWORK
    UcmNotAvailableOnTheNetwork                                            = 6476,
    /// UCM-PROCESSINGS
    UcmProcessings                                                         = 3963,
    /// UCM-REF
    UcmRef                                                                 = 930,
    /// UCM-SOFTWARE-VERSION
    UcmSoftwareVersion                                                     = 1349,
    /// UCM-STEP
    UcmStep                                                                = 2034,
    /// UCM-SUBORDINATE-MODULE-INSTANTIATION
    UcmSubordinateModuleInstantiation                                      = 1750,
    /// UCM-TO-TIME-BASE-RESOURCE-MAPPING
    UcmToTimeBaseResourceMapping                                           = 1221,
    /// UCMS
    Ucms                                                                   = 474,
    /// UDP-CHECKSUM-HANDLING
    UdpChecksumHandling                                                    = 3264,
    /// UDP-COLLECTION-BUFFER-SIZE-THRESHOLD
    UdpCollectionBufferSizeThreshold                                       = 4552,
    /// UDP-COLLECTION-BUFFER-TIMEOUT
    UdpCollectionBufferTimeout                                             = 310,
    /// UDP-COLLECTION-TRIGGER
    UdpCollectionTrigger                                                   = 2851,
    /// UDP-LOCAL-PORT
    UdpLocalPort                                                           = 282,
    /// UDP-MIN-TX-BUFFER-SIZE
    UdpMinTxBufferSize                                                     = 5672,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 2669,
    /// UDP-NM-CLUSTER-COUPLING
    UdpNmClusterCoupling                                                   = 4932,
    /// UDP-NM-CLUSTER-REF
    UdpNmClusterRef                                                        = 1284,
    /// UDP-NM-ECU
    UdpNmEcu                                                               = 1978,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 4353,
    /// UDP-PORT
    UdpPort                                                                = 1433,
    /// UDP-PORT-REF
    UdpPortRef                                                             = 3467,
    /// UDP-PROPS
    UdpProps                                                               = 6254,
    /// UDP-REMOTE-PORT
    UdpRemotePort                                                          = 5755,
    /// UDP-RULE
    UdpRule                                                                = 1111,
    /// UDP-TP
    UdpTp                                                                  = 6548,
    /// UDP-TP-PORT
    UdpTpPort                                                              = 581,
    /// UDP-TTL
    UdpTtl                                                                 = 732,
    /// UDS-DTC-NUMBER
    UdsDtcNumber                                                           = 3125,
    /// UDS-DTC-VALUE
    UdsDtcValue                                                            = 6195,
    /// UDS-SERVICE
    UdsService                                                             = 2537,
    /// UNASSIGN-FRAME-ID
    UnassignFrameId                                                        = 15,
    /// UNASSIGNED-FRAME-TRIGGERING-REF
    UnassignedFrameTriggeringRef                                           = 6276,
    /// UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE
    UncompressedSoftwareClusterSize                                        = 1691,
    /// UNICAST-CREDENTIALS
    UnicastCredentials                                                     = 5843,
    /// UNICAST-NETWORK-ENDPOINT-REF
    UnicastNetworkEndpointRef                                              = 4722,
    /// UNICAST-NETWORK-ENDPOINT-REFS
    UnicastNetworkEndpointRefs                                             = 6629,
    /// UNICAST-SECURE-COM-PROPS-REF
    UnicastSecureComPropsRef                                               = 2670,
    /// UNICAST-SECURE-COM-PROPS-REFS
    UnicastSecureComPropsRefs                                              = 719,
    /// UNICAST-UDP-CREDENTIALS
    UnicastUdpCredentials                                                  = 6214,
    /// UNIQUE-STREAM-ID
    UniqueStreamId                                                         = 3537,
    /// UNIT
    Unit                                                                   = 5680,
    /// UNIT-AND-SCALING-ID
    UnitAndScalingId                                                       = 5373,
    /// UNIT-DISPLAY-NAME
    UnitDisplayName                                                        = 4614,
    /// UNIT-GROUP
    UnitGroup                                                              = 1517,
    /// UNIT-GROUP-REF
    UnitGroupRef                                                           = 3063,
    /// UNIT-GROUP-REFS
    UnitGroupRefs                                                          = 1194,
    /// UNIT-REF
    UnitRef                                                                = 2243,
    /// UNIT-REFS
    UnitRefs                                                               = 5700,
    /// UNLIMITED-INTEGER-VALUE-VARIATION-POINT
    UnlimitedIntegerValueVariationPoint                                    = 2753,
    /// UNRESOLVED-REFERENCE-RESTRICTION
    UnresolvedReferenceRestriction                                         = 2297,
    /// UNUSED-BIT-PATTERN
    UnusedBitPattern                                                       = 4782,
    /// UPDATE
    Update                                                                 = 2285,
    /// UPDATE-INDICATION-BIT-POSITION
    UpdateIndicationBitPosition                                            = 97,
    /// UPDATE-KIND
    UpdateKind                                                             = 6139,
    /// UPDATE-SESSION-REJECTED
    UpdateSessionRejected                                                  = 498,
    /// UPDATE-STRATEGY
    UpdateStrategy                                                         = 1099,
    /// UPPER-BOUND
    UpperBound                                                             = 3142,
    /// UPPER-BOUNDARY
    UpperBoundary                                                          = 6053,
    /// UPPER-CAN-ID
    UpperCanId                                                             = 352,
    /// UPPER-HEADER-BITS-TO-SHIFT
    UpperHeaderBitsToShift                                                 = 3706,
    /// UPPER-LIMIT
    UpperLimit                                                             = 4785,
    /// UPPER-MULTIPLICITY
    UpperMultiplicity                                                      = 1388,
    /// UPPER-MULTIPLICITY-INFINITE
    UpperMultiplicityInfinite                                              = 2389,
    /// UPPER-MULTIPLICITY-IS-INFINITE
    UpperMultiplicityIsInfinite                                            = 5825,
    /// UPPER-REF
    UpperRef                                                               = 948,
    /// UPSTREAM-REFERENCE-IREF
    UpstreamReferenceIref                                                  = 4248,
    /// URI
    Uri                                                                    = 2882,
    /// URI-DESCRIPTION
    UriDescription                                                         = 2755,
    /// URL
    Url                                                                    = 960,
    /// USAGE
    Usage                                                                  = 3703,
    /// USAGES
    Usages                                                                 = 993,
    /// USE-AS-CRYPTOGRAPHIC-I-PDU
    UseAsCryptographicIPdu                                                 = 1098,
    /// USE-AUTH-DATA-FRESHNESS
    UseAuthDataFreshness                                                   = 6351,
    /// USE-AUTO-VALIDATION-AT-SHUT-DOWN
    UseAutoValidationAtShutDown                                            = 2973,
    /// USE-CASE
    UseCase                                                                = 1942,
    /// USE-CLIENT-AUTHENTICATION-REQUEST
    UseClientAuthenticationRequest                                         = 2325,
    /// USE-CRC-COMP-MECHANISM
    UseCrcCompMechanism                                                    = 2057,
    /// USE-FRESHNESS-TIMESTAMP
    UseFreshnessTimestamp                                                  = 3940,
    /// USE-INSTEAD-REF
    UseInsteadRef                                                          = 5412,
    /// USE-INSTEAD-REFS
    UseInsteadRefs                                                         = 1801,
    /// USE-MAC-ADDRESS-FOR-IDENTIFICATION
    UseMacAddressForIdentification                                         = 1617,
    /// USE-SECURED-PDU-HEADER
    UseSecuredPduHeader                                                    = 5603,
    /// USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT
    UseSecurityExtensionRecordSizeLimit                                    = 5309,
    /// USE-SMART-SENSOR-API
    UseSmartSensorApi                                                      = 5730,
    /// USE-VARIABLE-DA
    UseVariableDa                                                          = 4648,
    /// USE-VARIABLE-SA
    UseVariableSa                                                          = 5839,
    /// USE-VEHICLE-IDENTIFICATION-SYNC-STATUS
    UseVehicleIdentificationSyncStatus                                     = 1821,
    /// USED-CODE-GENERATOR
    UsedCodeGenerator                                                      = 4196,
    /// USED-DATA-ELEMENT
    UsedDataElement                                                        = 2725,
    /// USED-FID-REF
    UsedFidRef                                                             = 486,
    /// USED-IMPLEMENTATION-DATA-TYPE-REF
    UsedImplementationDataTypeRef                                          = 563,
    /// USED-IMPOSITION-TIME-DEFINITION-GROUP-REF
    UsedImpositionTimeDefinitionGroupRef                                   = 2843,
    /// USED-LANGUAGES
    UsedLanguages                                                          = 2222,
    /// USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF
    UsedLifeCycleStateDefinitionGroupRef                                   = 5112,
    /// USED-PARAMETER-ELEMENT
    UsedParameterElement                                                   = 664,
    /// USED-PIM-REF
    UsedPimRef                                                             = 746,
    /// USED-SECONDARY-FID-REF
    UsedSecondaryFidRef                                                    = 2079,
    /// USED-SECONDARY-FID-REFS
    UsedSecondaryFidRefs                                                   = 4321,
    /// USER-DATA-SUB-TLV
    UserDataSubTlv                                                         = 4633,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 6298,
    /// USER-DEFINED-CLUSTER-CONDITIONAL
    UserDefinedClusterConditional                                          = 3042,
    /// USER-DEFINED-CLUSTER-VARIANTS
    UserDefinedClusterVariants                                             = 4741,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 1062,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 3246,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL
    UserDefinedCommunicationControllerConditional                          = 1352,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS
    UserDefinedCommunicationControllerVariants                             = 1827,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 5426,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 193,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 4198,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 3091,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 4361,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 2946,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 6344,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 849,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 4968,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 1553,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 630,
    /// USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS
    UserDefinedTransformationComSpecProps                                  = 3099,
    /// USER-DEFINED-TRANSFORMATION-DESCRIPTION
    UserDefinedTransformationDescription                                   = 4878,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS
    UserDefinedTransformationISignalProps                                  = 4065,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    UserDefinedTransformationISignalPropsConditional                       = 1172,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    UserDefinedTransformationISignalPropsVariants                          = 3962,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 584,
    /// USER-REF
    UserRef                                                                = 6199,
    /// USER-REFS
    UserRefs                                                               = 4808,
    /// USES-ADDRESS-ARBITRATION
    UsesAddressArbitration                                                 = 4388,
    /// USES-DYNAMIC-ADDRESSING
    UsesDynamicAddressing                                                  = 3069,
    /// USES-END-TO-END-PROTECTION
    UsesEndToEndProtection                                                 = 4784,
    /// USES-MONITOR-DATA
    UsesMonitorData                                                        = 5612,
    /// USES-TIME-BASE-RESOURCE
    UsesTimeBaseResource                                                   = 4317,
    /// V
    V                                                                      = 6452,
    /// V-2-X-DATA-MANAGER-NEEDS
    V2XDataManagerNeeds                                                    = 4262,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 904,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 1689,
    /// V-2-X-SUPPORTED
    V2XSupported                                                           = 5758,
    /// V-LAN-REF
    VLanRef                                                                = 5911,
    /// V-LAN-REFS
    VLanRefs                                                               = 1596,
    /// VALID-BINDING-TIME
    ValidBindingTime                                                       = 2789,
    /// VALID-BINDING-TIMES
    ValidBindingTimes                                                      = 2122,
    /// VALIDATION-CONFIGURATION
    ValidationConfiguration                                                = 4699,
    /// VALIDATION-FORMULA
    ValidationFormula                                                      = 4894,
    /// VALIDATION-REF
    ValidationRef                                                          = 6158,
    /// VALIDATION-ROOT
    ValidationRoot                                                         = 3924,
    /// VALUE
    Value                                                                  = 2631,
    /// VALUE-ACCESS
    ValueAccess                                                            = 3711,
    /// VALUE-AXIS-DATA-TYPE-REF
    ValueAxisDataTypeRef                                                   = 668,
    /// VALUE-CONFIG-CLASSES
    ValueConfigClasses                                                     = 3106,
    /// VALUE-DATA-TYPE-REF
    ValueDataTypeRef                                                       = 430,
    /// VALUE-ERROR-COUNT-ENABLED
    ValueErrorCountEnabled                                                 = 1991,
    /// VALUE-IREF
    ValueIref                                                              = 5098,
    /// VALUE-PAIRS
    ValuePairs                                                             = 5744,
    /// VALUE-REF
    ValueRef                                                               = 3277,
    /// VALUE-RESTRICTION
    ValueRestriction                                                       = 1262,
    /// VALUE-SPEC
    ValueSpec                                                              = 3251,
    /// VALUE-TYPE-REF
    ValueTypeRef                                                           = 4309,
    /// VALUE-TYPE-TREF
    ValueTypeTref                                                          = 3718,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 3473,
    /// VARIABLE-ACCESS-IREF
    VariableAccessIref                                                     = 5949,
    /// VARIABLE-ACCESS-IREFS
    VariableAccessIrefs                                                    = 1072,
    /// VARIABLE-ACCESS-REF
    VariableAccessRef                                                      = 3383,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REF
    VariableAccessedForDebugRef                                            = 1478,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REFS
    VariableAccessedForDebugRefs                                           = 3296,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 2897,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 4600,
    /// VARIABLE-DATA-PROTOTYPE-IREF
    VariableDataPrototypeIref                                              = 4046,
    /// VARIABLE-DATA-PROTOTYPE-IREFS
    VariableDataPrototypeIrefs                                             = 3328,
    /// VARIABLE-INSTANCE
    VariableInstance                                                       = 6099,
    /// VARIABLE-INSTANCE-IREF
    VariableInstanceIref                                                   = 1547,
    /// VARIABLE-LENGTH
    VariableLength                                                         = 1065,
    /// VARIABLE-REF
    VariableRef                                                            = 254,
    /// VARIABLES
    Variables                                                              = 549,
    /// VARIANT-CRITERION-REF
    VariantCriterionRef                                                    = 5806,
    /// VARIANT-ID
    VariantId                                                              = 585,
    /// VARIATION
    Variation                                                              = 771,
    /// VARIATION-POINT
    VariationPoint                                                         = 917,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 3994,
    /// VARIATION-POINT-PROXYS
    VariationPointProxys                                                   = 2627,
    /// VARIATION-RESTRICTION
    VariationRestriction                                                   = 4798,
    /// VCID
    Vcid                                                                   = 2511,
    /// VEHICLE-ANNOUNCEMENT-COUNT
    VehicleAnnouncementCount                                               = 3164,
    /// VEHICLE-ANNOUNCEMENT-INTERVAL
    VehicleAnnouncementInterval                                            = 3006,
    /// VEHICLE-DESCRIPTION-REF
    VehicleDescriptionRef                                                  = 824,
    /// VEHICLE-DRIVER-NOTIFICATION
    VehicleDriverNotification                                              = 3483,
    /// VEHICLE-IDENTIFICATION-SYNC-STATUS
    VehicleIdentificationSyncStatus                                        = 3485,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 6051,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 5030,
    /// VEHICLE-SYSTEM
    VehicleSystem                                                          = 1935,
    /// VEHICLE-SYSTEM-INSTANCE
    VehicleSystemInstance                                                  = 6326,
    /// VENDOR
    Vendor                                                                 = 847,
    /// VENDOR-API-INFIX
    VendorApiInfix                                                         = 4525,
    /// VENDOR-ID
    VendorId                                                               = 5009,
    /// VENDOR-SIGNATURE-REF
    VendorSignatureRef                                                     = 1144,
    /// VENDOR-SPECIFIC-MODULE-DEF-REF
    VendorSpecificModuleDefRef                                             = 6609,
    /// VENDOR-SPECIFIC-MODULE-DEF-REFS
    VendorSpecificModuleDefRefs                                            = 6489,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 1355,
    /// VERBATIM
    Verbatim                                                               = 93,
    /// VERIFICATION-HASH
    VerificationHash                                                       = 2238,
    /// VERIFICATION-METHOD
    VerificationMethod                                                     = 1230,
    /// VERIFICATION-STATUS-INDICATION-MODE
    VerificationStatusIndicationMode                                       = 2846,
    /// VERIFY-IREF
    VerifyIref                                                             = 2797,
    /// VERIFY-IREFS
    VerifyIrefs                                                            = 3707,
    /// VERIFY-REF
    VerifyRef                                                              = 4551,
    /// VERIFY-REFS
    VerifyRefs                                                             = 901,
    /// VERIFY-UPDATE
    VerifyUpdate                                                           = 5915,
    /// VERSION
    Version                                                                = 4447,
    /// VERSION-DRIVEN-FIND-BEHAVIOR
    VersionDrivenFindBehavior                                              = 859,
    /// VF
    Vf                                                                     = 6597,
    /// VFB-TIMING
    VfbTiming                                                              = 405,
    /// VFC-IREF
    VfcIref                                                                = 3017,
    /// VFC-IREFS
    VfcIrefs                                                               = 3748,
    /// VG
    Vg                                                                     = 1360,
    /// VIEW-MAP
    ViewMap                                                                = 955,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 769,
    /// VIEW-MAPS
    ViewMaps                                                               = 2230,
    /// VIN-INVALIDITY-PATTERN
    VinInvalidityPattern                                                   = 544,
    /// VIOLATED-SAFETY-CONDITION-BEHAVIOR
    ViolatedSafetyConditionBehavior                                        = 4119,
    /// VLAN
    Vlan                                                                   = 2375,
    /// VLAN-ID
    VlanId                                                                 = 6463,
    /// VLAN-IDENTIFIER
    VlanIdentifier                                                         = 2818,
    /// VLAN-MEMBERSHIP
    VlanMembership                                                         = 5838,
    /// VLAN-MEMBERSHIPS
    VlanMemberships                                                        = 2153,
    /// VLAN-MODIFIER-REF
    VlanModifierRef                                                        = 5024,
    /// VLAN-PRIORITY
    VlanPriority                                                           = 5035,
    /// VLAN-REF
    VlanRef                                                                = 3576,
    /// VLAN-REFS
    VlanRefs                                                               = 2865,
    /// VLAN-TRANSLATION-TABLES
    VlanTranslationTables                                                  = 1280,
    /// VT
    Vt                                                                     = 6573,
    /// VTF
    Vtf                                                                    = 3433,
    /// WAIT-POINT
    WaitPoint                                                              = 3761,
    /// WAIT-POINTS
    WaitPoints                                                             = 4250,
    /// WAKE-UP-BY-CONTROLLER-SUPPORTED
    WakeUpByControllerSupported                                            = 2808,
    /// WAKE-UP-CHANNEL
    WakeUpChannel                                                          = 5999,
    /// WAKE-UP-OVER-BUS-SUPPORTED
    WakeUpOverBusSupported                                                 = 891,
    /// WAKE-UP-PATTERN
    WakeUpPattern                                                          = 2193,
    /// WAKEUP-FORWARD-LOCAL-ENABLED
    WakeupForwardLocalEnabled                                              = 1123,
    /// WAKEUP-FORWARD-REMOTE-ENABLED
    WakeupForwardRemoteEnabled                                             = 3038,
    /// WAKEUP-FRAME-REF
    WakeupFrameRef                                                         = 2688,
    /// WAKEUP-FRAME-REFS
    WakeupFrameRefs                                                        = 4068,
    /// WAKEUP-LOCAL-DETECTION-TIME
    WakeupLocalDetectionTime                                               = 629,
    /// WAKEUP-LOCAL-DURATION-TIME
    WakeupLocalDurationTime                                                = 6440,
    /// WAKEUP-LOCAL-ENABLED
    WakeupLocalEnabled                                                     = 3054,
    /// WAKEUP-REMOTE-ENABLED
    WakeupRemoteEnabled                                                    = 3446,
    /// WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST
    WakeupRepetitionDelayOfWakeupRequest                                   = 3250,
    /// WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST
    WakeupRepetitionsOfWakeupRequest                                       = 6589,
    /// WAKEUP-RX-IDLE
    WakeupRxIdle                                                           = 6592,
    /// WAKEUP-RX-LOW
    WakeupRxLow                                                            = 6231,
    /// WAKEUP-RX-WINDOW
    WakeupRxWindow                                                         = 647,
    /// WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF
    WakeupSleepOnDatalineConfigRef                                         = 211,
    /// WAKEUP-SYMBOL-RX-WINDOW
    WakeupSymbolRxWindow                                                   = 3028,
    /// WAKEUP-TX-ACTIVE
    WakeupTxActive                                                         = 3636,
    /// WAKEUP-TX-IDLE
    WakeupTxIdle                                                           = 2906,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 3112,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 4155,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 2094,
    /// WINDOW-SIZE
    WindowSize                                                             = 2028,
    /// WINDOW-SIZE-INIT
    WindowSizeInit                                                         = 2994,
    /// WINDOW-SIZE-INVALID
    WindowSizeInvalid                                                      = 5888,
    /// WINDOW-SIZE-VALID
    WindowSizeValid                                                        = 2209,
    /// WITH-AUTO
    WithAuto                                                               = 5068,
    /// WORST-CASE-EXECUTION-TIME
    WorstCaseExecutionTime                                                 = 475,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 2251,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 2901,
    /// WRITE
    Write                                                                  = 4111,
    /// WRITE-ACCESS
    WriteAccess                                                            = 1593,
    /// WRITE-CLASS-REF
    WriteClassRef                                                          = 817,
    /// WRITE-ONLY-ONCE
    WriteOnlyOnce                                                          = 5808,
    /// WRITE-VERIFICATION
    WriteVerification                                                      = 1787,
    /// WRITER-ENTITY-ID
    WriterEntityId                                                         = 2715,
    /// WRITING-FREQUENCY
    WritingFrequency                                                       = 4544,
    /// WRITING-PRIORITY
    WritingPriority                                                        = 2811,
    /// WRITING-STRATEGY-ROLE
    WritingStrategyRole                                                    = 517,
    /// WRITING-STRATEGYS
    WritingStrategys                                                       = 5696,
    /// WRITTEN-LOCAL-VARIABLES
    WrittenLocalVariables                                                  = 6087,
    /// WRITTEN-NV-DATA
    WrittenNvData                                                          = 3215,
    /// WRITTEN-READ-NV-DATA
    WrittenReadNvData                                                      = 3167,
    /// WWH-OBD-DTC-CLASS
    WwhObdDtcClass                                                         = 4264,
    /// X
    X                                                                      = 5556,
    /// XCP-PDU
    XcpPdu                                                                 = 1685,
    /// XDOC
    Xdoc                                                                   = 6238,
    /// XFILE
    Xfile                                                                  = 713,
    /// XREF
    Xref                                                                   = 3541,
    /// XREF-REF
    XrefRef                                                                = 236,
    /// XREF-TARGET
    XrefTarget                                                             = 6318,
}

impl ElementName {
    #[rustfmt::skip]
    const STRING_TABLE: [&'static str; 6652] = ["ROUTING-GROUP-REF", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "MESSAGE-LINE-NUMBER", "TCP-IP-ARP-TABLE-ENTRY-TIMEOUT", "FIELD-REPLY-TOPIC-NAME", "INTER-MESSAGE-TIME", "MEMORY-LOW-ADDRESS-LABEL", "GLOBAL-TIME-ETH-MASTER", "ACTIVATE-AT-START", "PERSISTENCY-FILE-STORAGE-INTERFACE", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "DATA-SEND-COMPLETED-EVENT", "PROVIDED-DDS-SERVICE-INSTANCE-EVENTS", "TLV-DATA-ID-0-REFS", "COMMON-SIGNAL-PATH", "UNASSIGN-FRAME-ID", "TCP-IP-DHCP-V-6-CNF-DELAY-MIN", "REPORTED-SECURITY-EVENT-IREF", "CONNECTIVITY-KEY-NAME", "REQUEST-DATA-SIZE", "REMOTE-ADDRESSS", "BLOCK-IF-STATE-ACTIVE-CP-REFS", "GETTER-REF", "PRIMITIVE-FAMILY", "RX-IDENTIFIER-RANGE", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF", "PORT-BLUEPRINT-REF", "SYSTEM-MAPPING", "BLOCK-INCONSISTENT", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "REST-SERVICE-INTERFACE", "OPERATION-RESPONSE-TRIGGERING-REF", "ML-GRAPHIC", "MAC-MULTICAST-ADDRESS-REF", "SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS", "DATA-IDS", "COUPLING-PORT-SPEED", "HOST-REF", "BSW-MODULE-DESCRIPTION-REF-CONDITIONAL", "LOGGING-BEHAVIOR", "SW-TEST-DESCS", "ALIAS-NAMES", "DIAGNOSTIC-MESSAGE-TYPE", "PARAMETER-INSTANCE", "SW-COMPONENT-PROTOTYPE-ASSIGNMENT", "PDU-REPLICATIONS", "MAXIMUM-DYNAMIC-PAYLOAD-LENGTH", "DATA-PROTOTYPE-REF", "NUMBER-OF-WORKERS", "SENSOR-ACTUATOR-REF", "ECUC-FLOAT-PARAM-DEF", "CUSTOM-SUB-FUNCTION-NUMBER", "E-2-E-PROFILE-COMPATIBILITY-PROPS-REF", "INTERNAL-TRIGGERING-POINT-POLICYS", "INITIAL-STATUS", "CALL-POINTS", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "INTENDED-FILENAME", "ADDRESSING-FORMAT", "MAC-ADDRESS-VLAN-MEMBERSHIP", "SWC-EXCLUSIVE-AREA-POLICY", "I-SIGNAL-TO-I-PDU-MAPPINGS", "IIDC-CHANNEL", "TARGET-VARIABLE-ACCESS-REF", "CRYPTO-JOBS", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "FLEXRAY-TP-NODE", "EVENT-GROUP-CONTROL-TYPE", "PARAMETER-ACCESS", "SW-CLASS-IMPL", "ARBITRATION", "PROCESS-DESIGN", "DTC-STATUS-AVAILABILITY-MASK", "SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF-CONDITIONAL", "NETWORK-ENDPOINT-REF", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "PARAMETER-ACCESSED-FOR-DEBUG-REFS", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "EVENT-IREF", "GROUP-REFS", "SEC-OC-SECURE-COM-PROPS-REF", "FIREWALL-RULE", "CDD-TYPE", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REF", "DATA-ELEMENT-IREF", "PATH-MTU-ENABLED", "INTERPOLATION-ROUTINE-MAPPINGS", "FIRST-MODE-REFS", "IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL", "NETWORK-ENDPOINTS", "FREEZE-CURRENT-STATE-SUPPORTED", "DLT-ECU-ID", "DEADLINE", "VERBATIM", "APPLICATION-ARRAY-ELEMENT-REF", "ACTION-TYPE", "FACTOR-SI-TO-UNIT", "UPDATE-INDICATION-BIT-POSITION", "TIMING-MODES", "KEY-SIZE", "EXTERNAL-TRIGGERING-POINT", "LAST-EGRESS-SCHEDULER-REF", "IEEE-1722-ACF-BUS-PART-RAW-DATA-STREAM-CONSUMER-MAPPING", "MAX-BS", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "POST-BUILD-VALUE-ACCESS-REF", "BURST-SIZE-REQUEST", "MC-FUNCTION", "RESET-TO-DEFAULT", "TERMINATE", "DDS-CP-SERVICE-INSTANCE-EVENT", "COMPU-INVERSE-VALUE", "NETWORK-INTERFACE-REFS", "CONTAINER-TRIGGER", "REQUIRED-SERVICE-PORT-IREF", "BSW-MODULE-DEPENDENCYS", "DIAG-ARG-INTEGRITY", "DDS-CP-SERVICE-INSTANCE-OPERATION", "DO-IP-ENTITY", "TWO-KEY-SLOT-MODE", "P-2-TIMING", "TRAFFIC-CLASS", "MODE-REQUEST-TYPE-MAPS", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS", "GID-INVALIDITY-PATTERN", "SUB-NODE-PHYSICAL-CHANNEL-REF", "KEYWORD-SET", "PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS", "ASYNCHRONOUS-SERVER-CALL-POINT", "TIME-HARDWARE-CORRECTION-THRESHOLD", "SEPARATION-TIME-RESPONSE", "SWC-BSW-SYNCHRONIZED-TRIGGER", "GENERAL-PURPOSE-TIMER-SERVICE-NEEDS", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "ALIVE-SUPERVISIONS", "NM-REPEAT-MSG-IND-ENABLED", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "TAILORING-REFS", "CONTEXT-REF", "TD-EVENT-BSW-MODULE-TYPE", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION", "SW-RECORD-LAYOUT-V-AXIS", "PARAMETER-SIZE", "FRAME-MAPPING", "N", "ECUC-QUERY-REF", "DIAGNOSTIC-SOVD-UPDATE-INTERFACE", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "CONSTR-LEVEL", "TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED", "FIRST-TRIGGER-REF", "PHM-RECOVERY-ACTION-INTERFACE", "SW-SERVICE-IMPL-POLICY", "DIAGNOSTIC-SESSION", "SUPPLIER-ID", "PROCESS-STATE-MACHINE", "TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION", "RETRY-INTERVAL-TIME", "DIAGNOSTIC-ROUTINE-CONTROL", "TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "SIGNAL-SERVICE-TRANSLATION-PROPSS", "ACTIVATION-ACTION", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "PRE-MAPPING", "MSG-ID-MATCH", "TIME-SYNC-P-PORT-PROTOTYPE-IREF", "FIRST-PORT-REF", "FAILED-SUPERVISION-CYCLES-TOLERANCE", "TCP-IP-IP-NUM-REASS-DGRAMS", "IEEE-1722-TP-AAF-CONNECTION", "GATEWAY", "I-PDU-TRIGGERINGS", "DIAGNOSTIC-SOVD-DATA", "CALLED-ENTRY-REF", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "TIME-SUB-TLV", "HW-PORT-MAPPINGS", "EXTERN-RATE-CORRECTION", "SOURCE-I-PDU-REF", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "REGENERATED-PRIORITY", "NM-NODE-DETECTION-ENABLED", "ECU-EXTRACT-REF", "E-2-E-OVERSAMPLING-PERIOD", "STORE-EVENT-ENABLED", "FM-ATTRIBUTE-DEF", "SEC-OC-CRYPTO-SERVICE-MAPPING-REF-CONDITIONAL", "PERSISTENCY-KEY-VALUE-PAIR", "RULES", "REQUIRED-DDS-EVENT-QOS-PROPS", "ASSOCIATED-EVENT-IDENTIFICATION", "MASK", "USER-DEFINED-EVENT-DEPLOYMENT", "CLIENT-SERVER-INTERFACE-MAPPING", "CLIENT-SERVER-TO-SIGNAL-MAPPING", "DIAGNOSTIC-REQUEST-UPLOAD", "FLEXRAY-CLUSTER", "ETHERNET-CLUSTER-CONDITIONAL", "NETWORK-ENDPOINT-REF-CONDITIONAL", "SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS", "MINIMUM-INTENDED-BINDING-TIME", "ICV-SECURED", "CRYPTO-ALGO-CONFIG", "IMMEDIATE-NV-DATA-STORAGE", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "SIZE-OF-ARRAY-LENGTH-FIELDS", "C-CODE", "LENGTH-SEQUENCE-COUNTER", "APMC-IP-V4-ADDRESS-PARAM-DEF", "WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF", "CAN-IDENTIFIER", "APPLICABILITY-INFO", "KEY-EXCHANGE-AUTHENTICATION-REFS", "DESIGN-REFS", "FUNCTION-GROUP-ERROR-MAPPINGS", "COMBINED-NO-DATA-INIT-COUNT", "BSW-MODULE-DESCRIPTION", "SOVD-GROUP-REFS", "TRANCEIVER-STANDBY-DELAY", "ACF-BUS-REF", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "MIN-DEADLINE", "ICV-SECURE-COM-PROPSS", "SEC-OC-COM-PROPS-FOR-MULTICAST-REFS", "PARTITION-REF", "TTCAN-ABSOLUTELY-SCHEDULED-TIMING", "DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING", "BSW-ENTRY-KIND", "MINOR-PROTOCOL-VERSION", "MAX-EXP-BS", "NETWORK-IDLE-TIME", "MODE-DECLARATION-GROUP-REF", "DLT-LOG-CHANNELS", "SUB-SOFTWARE-CLUSTER-REF", "XREF-REF", "BLUEPRINT-CONDITION", "ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING", "APMC-INSTANCE-REFERENCE-VALUE", "MASTER-TO-SLAVE-EVENT-MAPPINGS", "NM-CHANNEL-ID", "FLOW-METERING-REF", "ICMP-V-4-PROPS", "FUP-DATA-ID-LIST", "RESOURCE-IDENTIFIER-TYPE", "LOGICAL-EXPRESSION", "INVALID-VALUE-BITFIELD-ERRORS-ENABLED", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-PORT-MAPPING", "DIAGNOSTIC-TROUBLE-CODE-UDS", "PROFILE-BEHAVIOR", "TCP-IP-NDP-DEFENSIVE-PROCESSING", "FIFO-DEPTH", "ECUC-MULTILINE-STRING-PARAM-DEF", "VARIABLE-REF", "CONSTANT-VALUE-MAPPING-REFS", "EXIT-TIMEOUT-VALUE", "SO-CON-I-PDU-IDENTIFIER-REF", "METHOD-REF", "SEVERITY", "KEYWORD", "DIAGNOSTIC-MEMORY-IDENTIFIER", "CYCLE-AUTOSTART", "MODE-ACCESS-POINT", "REST-SYSTEM-TRIGGERED-EVENT", "PDELAY-LATENCY-THRESHOLD", "DENOMINATOR-CONDITION", "SEC-OC-SECURE-COM-PROPS-REF-CONDITIONAL", "CAN-COMMUNICATION-CONTROLLER-VARIANTS", "TOPIC-1", "CONTROLLED-FID-REF", "TLS-JOB-MAPPING", "I-SIGNAL-GROUP-REF", "CRYPTO-SERVICE-MAPPINGS", "CONNECT", "TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE", "ASSIGN-FRAME-ID", "CAN-BAUDRATE", "FLEXRAY-NM-CLUSTER-COUPLING", "MODE-REQUEST-TYPE-MAP", "TRIGGER-IREF", "DEFAULT-LC-STATE-REF", "UDP-LOCAL-PORT", "SIGNAL-GROUP-REF", "IDS-PLATFORM-INSTANTIATION-REF", "BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING", "BSW-ENTRY-RELATIONSHIP-SET", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "DEVELOPMENT-ERROR", "REQUESTER-REF", "SERVICE-ID", "COUNTER-JUMP-DOWN-VALUE", "SEC-OC-JOB-REQUIREMENT", "DYNAMIC-PART-ALTERNATIVE", "COLOR-MODE", "ARGUMENT", "REASON", "MAXIMUM-MESSAGE-LENGTH", "SYSTEM-MEMORY-USAGE", "STATIC-PARTS", "COUPLING-PORT-SWITCHOFF-DELAY", "NUMBER", "LIVELINESS-PROTECTION-KIND", "PURPOSE-OF-UPDATE-REF", "ALL-CHANNELS-REFS", "APPLICATION-RECORD-ELEMENT-REF", "SIZE-OF-UNION-LENGTH-FIELD", "CONFIG-CLASS", "LIVENESS-KIND", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "UDP-COLLECTION-BUFFER-TIMEOUT", "SEQUENCE-COUNTER-JUMP-WIDTH", "SELECTIONS", "RVF-ACTIVE-PIXELS", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "PER-STATE-TIMEOUTS", "INTEGER-VALUE-VARIATION-POINT", "MAC-ADDRESS-VLAN-ASSIGNMENTS", "FLAT-MAP-REF", "NO-CHECKPOINT-SUPERVISION", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "SWC-TO-SWC-SIGNAL", "DESCRIPTION", "TEXT-TABLE-MAPPING", "POSSIBLE-ERRORS", "SW-CALIBRATION-NOTESS", "TARGET-RUNNABLE-ENTITY-GROUP-REF", "ACL-CONTEXT", "TARGET-P-PORT-REF", "MAX-RETRIES", "SW-FEATURE-DESCS", "RECEIVER-INTENT", "MAX-RUN-CYCLE-LOOP", "APMC-IP-V4-ADDRESS-PARAM-VALUE", "FORMAL-BLUEPRINT-CONDITION", "DESTINATION-URI-REF", "PHYSICAL-DIMENSION-MAPPING-SET", "FM-SYSCOND", "MAX-RESIDENCE-TIME", "TCP-KEEP-ALIVE-TIME", "SIGNAL-MAPPINGS", "CONDITIONS", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "TARGET-TRIGGER-REF", "IEEE-1722-TP-ACF-CAN-PART", "CONTAINED-PACKAGE-ELEMENT-REFS", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPES", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "DIAG-PDU-TYPE", "SW-DATA-DEF-PROPS-CONDITIONAL", "BUNDLED-CONNECTIONS", "APP-ID", "UPPER-CAN-ID", "BUILD-ACTIONS", "PORT-DEFINED-ARGUMENT-VALUE", "ATTRIBUTE-REF", "J-1939-NODES", "UCM-MODULE-INSTANTIATION-REF", "CLUSTERED-COMPONENT-IREF", "ARTIFACT-LOCATOR", "EVALUATED-VARIANT-SET", "CONFIDENCE-INTERVAL", "SERVICE-TABLES", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING", "CHECKPOINTS", "META-DATA-LENGTH", "PREVIOUS-EXECUTABLE-VERSION", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "SUPERVISION-MODE-CONDITION", "CONTAINERS", "GETTER-CALL-REF", "ACF-MAX-TRANSIT-TIME", "PURE-LOCAL-TIME-BASE", "HARDWARE-ELEMENT-REF", "REQUEST-SEED-ID", "DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL", "SOURCE-ID", "ARRAY-ELEMENT-MAPPINGS", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "RECORD-VALUE-SPECIFICATION", "TIMING-DESCRIPTIONS", "CONTEXT-ROOT-COMPONENT-REF", "DO-IP-CONFIG", "DEFAULT-REPORTING-MODE", "RX-SECURITY-VERIFICATION", "CONTAINER-I-PDU-HEADER-BYTE-ORDER", "PROVIDED-SERVICE-PORT-IREF", "BLUEPRINT-POLICY-SINGLE", "STATE-DEP-FIREWALL-REFS", "ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "MODE-DECLARATION-MAPPING", "ETHERNET-CLUSTER", "TCP-ROLE", "CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST", "REQUIRED-SERVICE-INSTANCE-REF", "TGROUP", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "BANDWIDTH-LIMITATION", "ACTIVATION-REASON-REPRESENTATION-REF", "LIN-EVENT-TRIGGERED-FRAME", "DLT-LOG-CHANNEL-REF", "MEMORY-USAGES", "DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE", "VFB-TIMING", "DIAGNOSTIC-SOVD-DATA-CATEGORY", "PHYSICAL-REQUEST-REF", "SERVICE-NEEDS", "BSW-DATA-SEND-POLICY", "TRIGGER", "CLEAR-CONDITION-REFS", "MAX-INSTANCES", "CALIBRATION-PARAMETER-VALUES", "IMPLEMENTS-LEGACY-STRING-SERIALIZATION", "SECTION-INITIALIZATION-POLICY", "TCP-IP-NDP-RTR-SOLICITATION-INTERVAL", "MODE-DECLARATIONS", "EXECUTABLE-ENTITY-REF", "REQUIRED-DDS-SERVICE-INSTANCE", "SUBFUNCTION", "DEFAULT-VLAN-REF", "FIRST-OPERATION-REF", "DATA-PROTECTION-KIND", "INFRASTRUCTURE-SERVICES", "SHAPER", "CAN-NM-ECU", "SW-SERVICE-ARG", "APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION", "MIN-DATA-LENGTH", "VALUE-DATA-TYPE-REF", "MINIMUM-INTER-ARRIVAL-TIME", "PARAMETER-REF", "SPORADIC-EVENT-TRIGGERING", "SW-CLUSTER-REFS", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "SUB-NODE-NUMBER", "NM-STATE-CHANGE-IND-ENABLED", "BUILD-TYPE", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS", "ENABLE-ANYCAST", "NM-REMOTE-SLEEP-INDICATION-TIME", "NV-BLOCK-DESCRIPTOR", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "EXECUTABLE-ENTITY-REFS", "MEMORY-CONSUMPTION", "DURABILITY", "SECOND-PHYSICAL-DIMENSION-REF", "MESSAGE-LINK-POSITION", "EXPLICIT-WAKEUP-PNCS", "CLASSIFICATION", "PORT-CONFIGS", "PHM-RULE", "CRYPTO-SERVICE-PRIMITIVE", "MAC-MULTICAST-ADDRESS-REFS", "DIRECT-PDU-REF", "SOMEIP-SERVICE-INTERFACE", "COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION", "COMPU-PHYS-TO-INTERNAL", "PHM-CONTRIBUTION-REF", "PDUR-I-PDU-GROUP", "MINIMUM-RX-CONTAINER-QUEUE-SIZE", "STATE-MANAGEMENT-REQUEST-ERROR", "MATCHING-INGRESS-RULE-REFS", "R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "CONFLICTS", "SERVER-CALL-POINTS", "IDSM-QUALIFIED-EVENT-RECEIVER-MAPPING", "ADDRESS", "COM-PROPS", "APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-DEF", "REQUESTER-REFS", "TARGET-POSSIBLE-ERROR-REF", "NUMBER-OF-NOTIFIER-SETS", "UCMS", "WORST-CASE-EXECUTION-TIME", "NM-DATA-INFORMATION", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "INITIAL-CHECKPOINT-REFS", "DIAGNOSTIC-DATA-IDENTIFIERS", "COM-MANAGEMENT-PORT-GROUP-IREF", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF", "FORMAT", "IP-V-4-ADDRESS", "PREDECESSOR-REF", "USED-FID-REF", "SWC-RUNNABLE-REF", "REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF", "DTCS", "DISCOVERY-TYPE", "OPERATION-INVOKED-EVENT", "NETWORK-ENDPOINT-REFS", "COMM-CONTROLLER-MAPPINGS", "REFINED-EVENT-IREF", "AUTOSAR-VARIABLE-IN-IMPL-DATATYPE", "E-2-E-TIMEOUT-ERROR-REF", "ACTION-LIST-ITEM-REF", "UPDATE-SESSION-REJECTED", "PRIMITIVE-ATTRIBUTE-CONDITION", "PERSISTENT-STORAGE", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "MAX-DIFF", "PROTOCOL", "ANNOTATION-TEXT", "STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM", "INVALIDATION-POLICY", "REQUIRED-ENTRYS", "R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF", "TIME-SEG-2", "READ-CLASS-REF", "TEST-PATTERN", "ID", "MIN-DESTINATION-PORT-NUMBER", "QUEUED-SENDER-COM-SPEC", "DEFAULT-ENDIANNESS", "WRITING-STRATEGY-ROLE", "I-SIGNAL-PORT", "STATE-MANAGEMENT-STATE-NOTIFICATION", "TESTED-ITEM-REF", "TARGET-FRAME-REF", "RTE-EVENT-SEPARATIONS", "DO-IP-ROUTING-ACTIVATION", "TIME-LEAP-FUTURE-THRESHOLD", "RID-NUMBER", "FRAME-PIDS", "NODE-PORTS", "PERSISTENCY-ACCESS", "PERSISTENCY-FILE-REQUIRED-COM-SPEC", "AFFECTED-APPLICATION", "TD-EVENT-BSW-MODULE", "RATE-POLICYS", "PLATFORM-TIME-BASES", "FIFO-RANGES", "BUFFER-COMPUTATION", "EXECUTION-CONTEXT-REFS", "COMPU-DENOMINATOR", "ACL-CONTEXTS", "FUNCTIONAL-CLUSTER-REFS", "DIAGNOSTIC-PARAMETER", "ROUTING-GROUP-REFS", "SOMEIP-REMOTE-MULTICAST-CONFIG", "REQUIRED-GENERATOR-TOOLS", "VIN-INVALIDITY-PATTERN", "ADDRESS-CONFIGURATION-CAPABILITY", "JUMP-TO-BOOT-LOADER", "SET", "STREAM-IDENTIFICATION-HANDLE-REF", "VARIABLES", "IS-REENTRANT", "N-DATA-SETS", "GOVERNANCE-REF", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "DIAGNOSTIC-ENABLE-CONDITION-REF", "INITIALIZATION-VECTOR-LENGTH", "DIAGNOSTIC-DATA-IDENTIFIER-REF-CONDITIONAL", "COMMUNICATION-BUFFER-LOCKING", "PORT-PROTOTYPE-BLUEPRINT", "PERSISTENCY-FILE-ARRAY", "AGE", "HEALTH-CHANNEL-ARGUMENT-REF", "KEY-SLOT-USAGES", "USED-IMPLEMENTATION-DATA-TYPE-REF", "INITIALIZED-PARAMETER-REF", "TIME-MARK", "SW-ALIGNMENT", "PROCESSOR-MODE", "IS-SYNCHRONOUS", "DIAGNOSTIC-SESSION-REF", "SIGNATURE-SCHEME-ID", "AGING-REF", "PARTITION-NAME", "DIAGNOSTIC-READ-DTC-INFORMATION", "MIN-LENGTH", "REFERENCE-BASES", "HEALTH-CHANNELS", "RAM-BLOCK-INIT-VALUE", "ACCURACY-EXT", "MAC-SEC-PARTICIPANT-SET", "REFERENCE-VALUES", "UDP-TP-PORT", "CONFIDENTIALITY-OFFSET", "TIMING-CONDITION-REF", "USER-DEFINED-TRANSFORMATION-PROPS", "VARIANT-ID", "SW-CALIBRATION-NOTES", "CAN-TP-CONFIG", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "EXECUTION-ORDER-CONSTRAINT", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "INFO-TYPE", "INITIAL-NAD", "GLOBAL-TIME-MASTERS", "DIAGNOSTIC-EXTENDED-DATA-RECORD-CLIENT-PORT-MAPPING", "DATA-TYPE-FOR-SERIALIZATION-REFS", "TP-CONNECTION-CONTROLS", "NETWORK-TIME-CONSUMER-REF", "IP-PROTOCOL", "NETWORK-SEGMENT-ID", "COM-METHOD-GRANT", "CAN-CONTROLLER-CONFIGURATION", "SWC-TIMING", "PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT", "BSW-VARIABLE-ACCESS", "TCP-KEEP-ALIVE-INTERVAL", "ECUC-COND", "IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF", "DOMAIN-IDS", "COUPLING-PORT-ROLE", "TRANSMISSION-PROPS", "APMC-URI-INSTANCE-REFERENCE-VALUE", "COMMUNICATION-CLUSTER-REF", "DATA-PDU-REF", "CRYPTO-SERVICE-QUEUE", "ARTIFACT-DESCRIPTOR", "EVENTS", "FUNCTION-GROUP-TRANSITION-REQUEST-FAILED-ERROR-REF", "I-SIGNAL-TRIGGERINGS", "TRANSMIT-PDU-REFS", "NV-BLOCK-SW-COMPONENT-TYPE", "SYMBOLIC-NAME-PROPS", "TYPE-DEFINITION", "PHYSICAL-DIMENSION", "RETURN-CODE", "STATE-MANAGEMENT-BASED-ON-TCP-FLAGS", "TRIGGERING-EVENT-REF", "ETH-SWITCH-PORT-GROUP-DERIVATION", "NOT-AVAILABLE-VALUE-SPECIFICATION", "WAKEUP-LOCAL-DETECTION-TIME", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "MAX-NUMBER-OF-EVENT-ENTRIES", "FILE-STORAGE-REF", "SUPPORTS-MULTIPLE-INSTANTIATION", "SUCCESSOR-REFS", "TRIGGER-REF-CONDITIONAL", "DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL", "LOCAL-SUPERVISION-REF", "MAX-ACTION-LIST-DURATION", "PDU-COLLECTION-MAX-BUFFER-SIZE", "DIAGNOSTIC-EVENT-NEEDS", "DIAGNOSTIC-EVENT-REF-CONDITIONAL", "READ-DTC-INFORMATION-CLASS-REF", "CONSUMED-EVENTS", "STATE-REFERENCES", "MATCHING-RULE-REF", "MAX-TRCV-DELAY-COMPENSATION-OFFSET", "WAKEUP-RX-WINDOW", "PRECONFIGURED-CONFIGURATION-REFS", "ALGORITHM-MODE", "APPLICATION-PARTITION", "NM-CAR-WAKE-UP-FILTER-ENABLED", "DIAGNOSTIC-TRANSMIT-CERTIFICATE-INTERFACE", "COMPU-INTERNAL-TO-PHYS", "BSW-BEHAVIOR-REF", "TCP-SYN-RECEIVED-TIMEOUT", "INGRESS-VLAN-ID", "GLOBAL-ELEMENT", "BIT-POSITION", "REQUIRED-PACKAGE-ELEMENT-REF", "APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-VALUE", "IP-SEC-CONFIG-PROPS", "INSTANTIATION-RTE-EVENT-PROPSS", "APPL-CONSTANT-REF", "USED-PARAMETER-ELEMENT", "KEY-EXCHANGE-AUTHENTICATION-REF", "DIAGNOSTIC-DE-AUTHENTICATION", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "VALUE-AXIS-DATA-TYPE-REF", "SECOND-APPLICATION-ERROR-REF", "GENERAL-INACTIVITY-TIME", "TTL", "MSR-QUERY-P-1", "CIPHER-SUITE-ID", "INPLACE", "SYSTEM-TIMING", "MINIMUM-SUSTAINED-SIZE", "DEPENDS-ON-REF", "ACCESSED-PARAMETER", "SYNCHRONIZATION-POINT-CONSTRAINT", "CAN-CONTROLLER-ATTRIBUTES", "DATA-PROTOTYPES", "SD-SERVER-EG-TIMING-CONFIGS", "SW-CLASS-IMPLS", "J-1939-TP-CONFIG", "TD-EVENT-TRIGGER", "SW-BASE-TYPE", "TRANSMISSION-MODE-DECLARATION", "COUPLING-ELEMENT-DETAILS", "SIGNIFICANCE", "REPLICA-PDUS-REFS", "MAPPED-SECURITY-EVENTS", "CAN-FD-FRAME-SUPPORT", "AP-APPLICATION-ERROR", "CALIBRATION-PARAMETER-VALUE-SET-REFS", "SIZE-OF-STRUCT-LENGTH-FIELD", "MODE-DEPENDENT-STARTUP-CONFIG", "DLT-LOG-SINK-REFS", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "PLCA-LOCAL-NODE-ID", "NUMERICAL-RULE-BASED-VALUE-SPECIFICATION", "SPECIFICATION-SCOPE", "SYNC-TIME-BASE-MGR-USER-NEEDS", "AUTHENTICATION-PROPSS", "SOMEIP-SERVICE-DISCOVERY-PORT", "RP-IMPL-POLICY", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "INIT-EVENT", "RESOURCE-GROUP-REF", "CRYPTO-JOB-REF", "LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS", "NM-HANDLE-REF", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "XFILE", "TA-TYPE", "IMPLEMENTATION-CONFIG-VARIANT", "SOURCE-CAN-ID-CODE", "TLS-CRYPTO-MAPPING-REF", "SOURCE-FRAME-REF", "UNICAST-SECURE-COM-PROPS-REFS", "STANDARD-REVISION", "FLAT-OBJECT-REF", "NM-INSTANCE-ID", "CONTEXT-LIMITATION-REF", "MAXIMUM-SEGMENT-LENGTH", "MEMORY-SECTION", "CONTRACT-VERSION", "ACTIVE-SUPERVISION-REFS", "FRESHNESS-PROPSS", "TIMING-ARGUMENTS", "TTCAN-PHYSICAL-CHANNEL", "DO-IP-POWER-MODE-STATUS-NEEDS", "UDP-TTL", "REST-ENDPOINT-GET", "FR-CLUSTER-REF", "DATA-TRANSFER-CLASS-REF", "RUNNABLE-ENTITY-ARGUMENT", "DOCUMENT-ELEMENT-SCOPES", "SW-RECORD-LAYOUT-GROUP", "LOCAL-UDP-PORT-REF", "SIGNAL-I-PDU-REPLICATION", "ROOT-AUTOSAR-DATA-PROTOTYPE-REF", "DIRECT-SUCCESSOR-REF", "SIGNATURE-SUPPORT-AP", "RPT-HOOK", "LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING", "USED-PIM-REF", "METHOD-CALL-REFS", "CLUSTERED-COMPONENT-IREFS", "TOLERANCE", "NODE-IDENTIFIER", "ICV-VERIFICATION", "MC-FUNCTION-DATA-REF-SET-VARIANTS", "CRYPTO-KEY-SLOT-REFS", "REMOTE-UNICAST-ADDRESSS", "I-SIGNAL-I-PDU-REF-CONDITIONAL", "BYPASS-VLAN", "REENTRANCY-LEVEL", "DDS-EVENT-QOS-PROFILE-REF", "RAM-LOCATION-REF", "INDEX-DATA-TYPE-REF", "LIN-NM-CLUSTER", "STARTUP-CONFIG-REF", "DIRECT-TP-SDU-REF", "EXTERNAL-REPLACEMENT-REF", "ROM-BLOCK-INIT-VALUE", "SYNCHRONOUS-SERVER-CALL-POINT", "PROCESS-DESIGN-REF", "TLV-DATA-ID-DEFINITION-REF", "VIEW-MAP-SET", "CONSTANT-SPECIFICATION-MAPPING", "VARIATION", "PERIODIC-RESPONSE-UUDT-REF", "I-PDU-REF", "SCOPE-REFS", "HW-ATTRIBUTE-LITERAL-DEF", "TARGET-CHANNEL", "ENABLE-DISCOVERY-PROTECTION", "SNAPSHOT-RECORD-CONTENTS", "P-2-SERVER-MAX", "SUPERVISION-ENTITYS", "INITIAL-INACTIVITY-TIME", "DIAGNOSTIC-DTC-CHANGE-TRIGGER", "TARGET-PHM-SUPERVISED-ENTITY-IREF", "SECOND-ELEMENT-INSTANCE-IREF", "GLOBAL-TIME-TX-PERIOD", "EXECUTABLE-LOGGING-IMPLEMENTATION-PROPS", "HEAP-USAGES", "FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR", "TCP-IP-IP-NUM-FRAGMENTS", "CRYPTO-ALGORITHM-STRING", "EVENT-GROUP-REF", "FREEZE-FRAME-CONTENT-WWH-OBD-REF", "COMMITTED-BURST-SIZE", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF", "BSW-IMPLEMENTATION-REF", "CRYPTO-PRIMITIVE", "SW-VERSION", "FIREWALL-STATE-MODE-DECLARATION-REFS", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS", "OVERRIDE-INITIAL-STATE-IREF", "CONTROLLER-REF", "CALL-TYPE", "LOGICAL-ADDRESS", "DIAGNOSTIC-EDR-DATA-PROVIDER-MAPPING", "SERIALIZATION-TECHNOLOGY", "TRANSITIONS", "ACTION-LIST", "MAX-CONVERSATIONS", "NM-CYCLETIME-MAIN-FUNCTION", "FIRE-AND-FORGET-METHOD-MAPPING", "SERVICE-OFFER-TIME-TO-LIVE", "J-1939-CLUSTER", "DIAGNOSTIC-PROPS", "FUNCTION-INHIBITION-NEEDS", "SECOND-ELEMENT-INSTANCE-IREFS", "CONTEXT-P-PORT-PROTOTYPE-REF", "WRITE-CLASS-REF", "DLT-MESSAGES", "PORT-INSTANTIATION-BEHAVIOR", "METHOD-RETURN-REF", "DO-IP-TESTER-ROUTING-ACTIVATION-REF", "J-1939-CLUSTER-VARIANTS", "ELEMENT-GROUPS", "VEHICLE-DESCRIPTION-REF", "DHCP-ADDRESS-ASSIGNMENT", "ACL-OPERATION", "CHOICES", "EOC-EVENT-REF", "CONNECTION-BUNDLES", "EXTENDED-ADDRESSING", "TX-BIT-RATE-SWITCH", "FUNCTION-GROUP-IREFS", "TARGET-EVENT-REF", "POST-VERIFICATION-REBOOT", "IEEE-1722-TP-IIDC-CONNECTION", "PDUR-TP-CHUNK-SIZE", "ECUC-STRING-PARAM-DEF-VARIANTS", "ENCAPSULATED-ENTRY-REF", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "NETWORK-INTERFACES", "RAW-DATA-STREAM-DEPLOYMENT", "COMMUNICATION-CONNECTOR-REF", "SECURITY-EVENT-CONTEXT-DATA-DEFINITIONS", "NM-CBV-POSITION", "RECEIVER-REF", "BR", "VENDOR", "MSR-QUERY-PROPS", "USER-DEFINED-PDU", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS", "SEC-OC-CRYPTO-SERVICE-MAPPINGS", "REFINED-MODULE-DEF-REF", "ACCESS-CONTROL", "COMPARE-TYPE", "REQUEST-CONFIGURATIONS", "COUNTER-DECREMENT-STEP-SIZE", "SCHEDULING-POLICY", "VERSION-DRIVEN-FIND-BEHAVIOR", "TP-CONNECTIONS", "IDSM-TIMESTAMP-PROVIDER-MAPPING", "CAN-GLOBAL-TIME-DOMAIN-PROPS", "IS-OBD-RELEVANT", "FILTERING-DEBOUNCING", "SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "PROCESS-EXECUTION-ERROR-REF", "DATA-TRANSFORMATION-SET", "EVENT-SYMBOL-NAME", "CRYPTO-CERTIFICATE-GROUP-INTERFACE", "MODE-IREF", "APPLICABILITY-INFO-SET", "LOC-MEASUREMENT-SET", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "TARGET-EEC-REFS", "CURRENT-VALUE-REF", "FOLLOW-UP-ACTION-REF", "TARGET-PDU-TRIGGERINGS", "ASSUMED-CURRENT-STATE-IREF", "ESTIMATED-DURATION-OF-OPERATION", "MAXIMUM-TRANSMIT-BUFFER-LENGTH", "STOP", "PDU-TO-FRAME-MAPPING", "TCP-FAST-RECOVERY-ENABLED", "EXCLUSIVE-AREA-REF", "ENUMS", "CERTIFICATE-GROUP-REF", "AR-PACKAGES", "FILE-TYPE", "MODE-DECLARATION-GROUP", "ECUC-INSTANCE-REFERENCE-VALUE", "WAKE-UP-OVER-BUS-SUPPORTED", "IDSM-INSTANCE-REF", "REQUIRED-MODE-GROUP-REF", "MAPPED-BSW-SERVICE-DEPENDENCY-REF", "DDS-SERVICE-INSTANCE-OPERATION-CP", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "SUSPEND-TO-RAM-SATELLITE-MAPPING", "ALL-PHYSICAL-CHANNELS-REF", "P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF", "SENDER-INTENT", "VERIFY-REFS", "REPLAY-PROTECTION-WINDOW", "I-PV-6-EXT-HEADER-FILTER-LIST", "V-2-X-FAC-USER-NEEDS", "SUSPEND-TO-RAM-AWARENESS", "CLEAR-EVENT-BEHAVIOR", "MAX-EVENTS-IN-INTERVAL", "OBD-MID", "DIAGNOSTIC-SOVD-FAULT-MEMORY-ACCESS", "CONTEXT-PORT-REF", "ENABLE-CONDITION-REF", "APMC-URI-PARAM-DEF", "DIAGNOSTIC-TROUBLE-CODE-OBD", "ACCEPT-VARIABLE-DA", "CONSUMED-EVENT-GROUP-REF", "RECEIVER-IREF", "VARIATION-POINT", "TIMEOUT-CR", "CORE-ID", "POST-BUILD-VARIANT-SUPPORT", "CYCLE-COUNTER", "DATA-TYPE-REF", "FILTER-ACTION-DROP-FRAME", "READ", "SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING", "MAC-LAYER-TYPE", "COUPLING-PORT-SHAPER", "MAX-BUFFER-REQUEST", "NOTIFIER-REF", "UCM-REF", "DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE", "RAW-DATA-STREAM-REF", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "DEADLINE-MIN", "FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL", "DO-IP-LOGICAL-ADDRESS", "SOCKET-ADDRESS-REF-CONDITIONAL", "STREAMS-PER-FRAME", "AUXILIARY-FIELDS", "TD-EVENT-FRAME-ETHERNET", "ALGORITHM-FAMILY", "MINIMUM-DELAY", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "CLIENT-SERVER-ARRAY-ELEMENT-MAPPING", "CONFIG-ELEMENT-DEF-GLOBAL-REF", "RPT-ENABLER-IMPL-TYPE", "DIAGNOSTIC-CONNECTION-REF", "UPPER-REF", "SCHEDULER-NAME-PREFIX-REF", "FILTER-CHAINS", "R-PORT-PROTOTYPE-IREF", "DATA-CONSTR-RULE", "APMC-INSTANCE-REFERENCE-DEF", "CRYPTO-R-PORT-COM-SPEC", "VIEW-MAP", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "LET-INTERVAL-REF", "APPLICATION-MODE-MACHINE", "COUPLING-PORTS", "URL", "MODES", "CAN-CLUSTER-CONDITIONAL", "BSW-MODULE-DOCUMENTATIONS", "PARTIAL-NETWORK-REFS", "METHOD-CALL-REF", "MAC-SEC-KAY-PARTICIPANT", "MAIN-FUNCTION-PERIOD", "CODE-DESCRIPTORS", "DIAGNOSTIC-STORAGE-CONDITION-REF", "INTERNET-HEADER-LENGTH", "ETHER-TYPE", "SHARED-PARAMETERS", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "MORE-FRAGMENTS", "COM-MANAGEMENT-GROUP-REF", "SDG-CONSTRAINT-REFS", "LIN-UNCONDITIONAL-FRAME", "ENTER-TIMEOUT-VALUE", "COMPOUND-PRIMITIVE-ARGUMENTS", "RECORD-ELEMENT-REF", "INTERNAL-TRIGGERING-POINT", "TABLE-CAPTION", "PREDECESSOR-FIFO-REF", "NUMBER-OF-REPETITIONS", "DIRECT-SUCCESSOR-REFS", "MODIFIED-DATAS", "HANDLE-INVALID", "RESOURCE-DEFINITION-REF", "DIAGNOSTIC-EDR-SERVER-PORT-MAPPING", "MC-GROUP-DATA-REF-SET-VARIANTS", "APPLICATION-PARTITION-REFS", "PDELAY-REQUEST-PERIOD", "USAGES", "INTERNAL-TRIGGERING-POINTS", "COMMUNICATION-CONTROLLER-MAPPING", "SWITCH-STREAM-IDENTIFICATION", "MEMORY-RANGE-REFS", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "NM-BUS-SYNCHRONIZATION-ENABLED", "IMPLEMENTATION-RECORD-ELEMENT-REF", "TD-EVENT-SERVICE-INSTANCE-EVENT", "CAN-BIT-RATE-SWITCH", "MODE-DEPENDENT-STARTUP-CONFIGS", "ACF-BUS-PART-REFS", "MAC-MULTICAST-CONFIGURATION", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "CONNECTION-PRIO-PDUS", "GENERIC-TP-CONNECTION", "NM-IMMEDIATE-NM-TRANSMISSIONS", "HEADER-ID-LONG-HEADER", "FUNCTION-GROUP-TYPE-REF", "MAXIMUM-NUMBER-OF-RETRIES", "IMPLEMENTATION-REF", "OWNER-REFS", "DIAGNOSTIC-PARAMETER-REF", "INGRESS-PORT-REFS", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF", "TRANSMISSION-ACKNOWLEDGE", "REFERENCE-VALUE-REF", "PARENT-SHORT-LABEL", "EXPECTED-CALLBACKS", "SHALL-NOT-RUN-ON-REF", "SECOND-ELEMENT-REFS", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "CLIENT-SERVER-RECORD-ELEMENT-MAPPING", "FILTER-PRIORITY", "ABSOLUTELY-SCHEDULED-TIMINGS", "STREAM-FILTER-PORT-RANGE", "ACTION-POINT-OFFSET", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "CATEGORY", "DATA-CONSISTENCY-POLICY", "SEC-OC-DEPLOYMENT", "I-SIGNAL-TYPE", "NM-PDU-REF-CONDITIONAL", "CLIENT-SERVER-OPERATION-COM-PROPS", "SIZE-OF-STRING-LENGTH-FIELD", "STATE-MANAGEMEN-PHM-ERROR-INTERFACE", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "MULTICAST-REF", "RTE-RESOURCE-ESTIMATION", "DDS-PARTITION-REF", "START-STATE-MACHINE-REF", "ECUC-FUNCTION-NAME-DEF", "ENTITYS", "CANCELLATION", "TRANSMISSION-MODE", "TD-EVENT-BSW-MODE-DECLARATION-TYPE", "NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS", "READ-SCALING-DATA-CLASS-REF", "BSW-PER-INSTANCE-MEMORY-POLICYS", "OFFSET-TIME-DOMAIN-REF", "HW-CATEGORY-REFS", "N-ROM-BLOCKS", "TRANSMISSION-MODE-CONDITION", "SAMPLES-PER-MICROTICK", "N-PDU-REF", "TRANSPORT-ERROR-COUNT-ENABLED", "SUPPORTED-CIPHER-SUITES", "PROVIDED-FIELDS", "TRANSMISSION-MODE-CONDITIONS", "USER-DEFINED-COMMUNICATION-CONNECTOR", "PRM-UNIT", "NM-VECTOR-EARLY-UPDATE", "VARIABLE-LENGTH", "ECUC-INSTANCE-REFERENCE-DEF", "NESTED-ELEMENTS", "SDG-CONSTRAINT-REF", "COMM-CONTROLLERS", "SLEEP-MODE-EXECUTION-DELAY", "HW-PIN-CONNECTOR", "VARIABLE-ACCESS-IREFS", "SERVICE-INSTANCE-REF", "DATA-TYPE-MAPPING-REF", "MODE-INTERFACE-MAPPING", "TEAR-DOWN-ACTION-REFS", "DISPLAY-PRESENTATION", "SWC-BSW-RUNNABLE-MAPPING", "DDS-SECURE-COM-PROPS", "TERMINATING-CHECKPOINT-REF", "PADDING-ACTIVATION", "CONSTRAINT-TAILORINGS", "SOCKET-CONNECTION-REF", "LIN-IDENTIFIER", "SWC-SERVICE-DEPENDENCY-IREF", "DIAGNOSTIC-COMPONENT-NEEDS", "RECOVERY-VIA-APPLICATION-ACTION-REF", "DDS-TRANSFORMATION-I-SIGNAL-PROPS", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "TRANSFORMER-CLASS", "COMPOSITE-INTERFACE", "NM-SYNCHRONIZATION-POINT-ENABLED", "GETTER-RETURN-REF", "CONTEXT-DATA-PROTOTYPE-IN-CS-REF", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "TARGET-RUNNABLE-ENTITY-REF", "INTERFACE-BLUEPRINT-REF", "USE-AS-CRYPTOGRAPHIC-I-PDU", "UPDATE-STRATEGY", "COMPU-CONST", "DATA-IDENTIFIER-REF", "NM-CAR-WAKE-UP-RX-ENABLED", "MIN-RPT-EVENT-ID", "ECUC-NUMERICAL-PARAM-VALUE", "SUSPEND-TO-RAM-HUB-INTERFACE", "PROVIDER-REF", "DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF", "COM-CONTROL-CLASS-REF", "FLOW-LABEL", "DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING", "UDP-RULE", "REMOTE-ID", "FMI", "RECEIVER-ANNOTATION", "DATA-EXCHANGE-POINT", "CLIENT-ID", "CAN-XL-CONFIG-REF", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "RATE-CORRECTIONS-PER-MEASUREMENT-DURATION", "ETH-GLOBAL-TIME-DOMAIN-PROPS", "DIAGNOSTIC-STORAGE-CONDITION", "CONTEXT-ID", "WAKEUP-FORWARD-LOCAL-ENABLED", "ENABLE-UPDATE", "COMPU-RATIONAL-COEFFS", "GRANT-REFS", "PDU-ACTIVATION-ROUTING-GROUP", "TP-PGS", "SOMEIP-METHOD-PROPS", "LOCAL-PORT-RANGE-END", "DIAGNOSTIC-COMMON-ELEMENT-REF", "REVERSED-TP-SDU-REF", "TIME-TRIGGERED-CAN-LEVEL", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPINGS", "SYSTEM-SIGNAL-REF", "HISTORY-ORDER-DEPTH", "ABBR-NAME", "LATENCY-TIMING-CONSTRAINT", "CRYPTO-KEY-SLOT-DESIGN-DESCRIPTION", "COUPLED-CLUSTER-REF", "MAX-ACTION-ITEM-DURATION", "AP-APPLICATION-ERROR-DOMAIN", "VENDOR-SIGNATURE-REF", "TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS", "TLV-DATA-ID-DEFINITION-SET", "INDIRECT-API", "INCLUDED-MODE-DECLARATION-GROUP-SETS", "RETURN-VALUE-PROVISION", "TRACE-REFS", "SW-CLASS-PROTOTYPE-REF", "NV-BLOCK-DATA-MAPPING", "ALLOWED-SERVICE-CONSUMERS", "MIN-VALUE", "APPLICATION-COMPOSITE-ELEMENT-IREF", "ALLOW-PROVIDER-RATE-CORRECTION", "CONTENT-REF", "REMOTE-ACCESS-CONTROL-ENABLED", "PROVIDED-SERVICE-INSTANCE-ID", "TRANSFORMATION-PROPS-SET", "RECOVERY-VIA-APPLICATION-ACTION", "STRING-ENCODING", "FILTER-ACTION-BLOCK-SOURCE", "RETURN-TYPE", "DECOMPOSITIONS", "SOFTWARE-CLUSTER-DEPENDENCY", "DATA-READ-ACCESSS", "SEC-OC-IGNORE-VERIFICATION-RESULT", "DIAGNOSTIC-J-1939-SW-MAPPING", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-DIAGNOSTIC-EVENT-MAPPING", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "SW-CLASS-ATTR-INSTANCE-IMPL", "HW-ATTRIBUTE-VALUES", "COMPOSITE-TYPE-MAPPINGS", "CHECKPOINT-INITIAL-REF", "CSE-CODE-FACTOR", "DEBOUNCE-ALGORITHM-PROPSS", "FLOW-CONTROL-REF", "REF-CALPRM-SET", "RANGE-MIN", "ACL-OBJECT-SET", "INPUT", "FRESHNESS-VALUE-TX-LENGTH", "PNC-MAPPING-REF", "S-3-SERVER-TIMEOUT", "CRYPTO-CERTIFICATE-REF", "SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS", "COMMUNICATION-CONNECTOR-REF-CONDITIONAL", "REQUEST-UPLOAD-CLASS-REF", "TRCV-DELAY-COMPENSATION-OFFSET", "ENABLE-READ-ACCESS-CONTROL", "SYMBOL-PROPS", "UNIT-GROUP-REFS", "PERIODIC-RATE-CATEGORY", "IDS-MGR-NEEDS", "REFERENCE-TAILORING", "ETS-AVAILABLE-BANDWIDTH-IN-PERCENT", "FREE-FORMAT", "TCP-IP-IP-FRAGMENTATION-RX-ENABLED", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION", "MSR-QUERY-ARG", "NV-BLOCK-DESCRIPTORS", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "SLEEP-MODE-SUPPORTED", "SOURCE-INSTANCE-IREFS", "LOGICAL-EXPRESSIONS", "TIMING-PROPS", "CRYPTO-KEY-TYPE", "RULE-BASED-VALUES", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "FM-FEATURE", "MAXIMUM", "DLT-ECU-REF", "SEND-INDICATION", "TARGET-PORT-PROTOTYPE-REF", "SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME", "DATA-SEND-POINTS", "ACTION-LIST-ITEM-REFS", "UCM-TO-TIME-BASE-RESOURCE-MAPPING", "LIVELINESS", "TCP-OPTION-FILTER-LIST", "SUPPORTED-TIMER-GRANULARITY", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "SAFETY-CONDITIONS", "MACHINE-REF", "LEAF-ELEMENT-IREF", "BUCKET-SIZE", "VERIFICATION-METHOD", "KEEP-ALIVES", "PROVIDED-DATAS", "TP-CHANNELS", "DEFERRING-FID-REFS", "MODE-DECLARATION-GROUP-REFS", "EXTENDS-META-CLASS", "TCP-FAST-RETRANSMIT-ENABLED", "E", "MSR-QUERY-RESULT-P-2", "CSE-CODE", "SEPARATION-TIME-REQUEST", "FIREWALL-RULE-REFS", "PHM-CHECKPOINT-IREF", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "INITIAL-MODE-REF", "SPECIFIC-CHANNELS", "RTE-PLUGIN-PROPS", "TIME-BASE-RESOURCE-REF", "EVENT-HANDLER", "ECU-RESOURCE-ESTIMATION", "STACK-USAGES", "SECURE-ONBOARD-COM-ERROR-REF", "CONTEXT-PORT-PROTOTYPE-REF", "STATIC-SOCKET-CONNECTION", "TD-EVENT-TYPE", "TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE", "NV-PROVIDE-COM-SPEC", "ASSIGNED-LIN-SLAVE-CONFIG-REF", "METHOD-IREF", "SUB-ATTRIBUTE-TAILORINGS", "SUB-GROUP-REFS", "VALUE-RESTRICTION", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "EVENT-GROUPS", "BSW-TRIGGER-REF", "ABSOLUTE-TOLERANCE", "DOIP-CONNECTION-REFS", "TP-SDU-REFS", "GLOBAL-TIME-FR-SLAVE", "SYNCHRONIZED-TIME-BASE-CONSUMER", "SIGNAL-PATH-CONSTRAINTS", "PAYLOAD-LENGTH", "DISABLED-MODE-IREFS", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "DATA-PROTOTYPE-GROUP-IREF", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "IEEE-1722-TP-CRF-CONNECTION", "APPLICATION-DESCRIPTION", "SW-SYSTEMCONSTANT-VALUES", "VLAN-TRANSLATION-TABLES", "HOST-ID", "FLEXRAY-TP-ECU", "PDU-TRIGGERING-REFS", "UDP-NM-CLUSTER-REF", "SECURE-COM-PROPSS", "RECEIVER-IREFS", "ACTION-ITEM-REF", "NM-COORD-CLUSTER", "SOMEIP-EVENT-GROUP", "EVALUATED-ELEMENT-REF", "COM-FIND-SERVICE-GRANT", "EVENT-WINDOW-TIME", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF", "CRYPTO-KEY-SLOT-CLIENT-INTERFACE", "FALSE-ACTION-LIST-REF", "SOURCE-CHANNEL", "CONSTANT-MAPPING-REF", "DIAGNOSTIC-SOVD-OPERATION-PORT-MAPPING", "MAC-SEC-PROPSS", "IDSM-TIMESTAMP-PROVIDER-INTERFACE", "OBD-PID-SERVICE-NEEDS", "PROCESS-TO-MACHINE-MAPPING", "MONOTONY", "EXEC-IN-USER-MODE", "CRYPTO-CERTIFICATE-GROUPS", "SOURCE-MAX-ADDRESS", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "IIDC-SOURCE-PACKET-HEADER", "ITEM", "DIAGNOSTIC-J-1939-SPN", "SUCCESSOR-REF", "DDS-SERVICE-QOS-PROFILE-REF", "HAS-NOTIFIER", "DIAGNOSTIC-DATA-IDENTIFIER", "SWC-TO-ECU-MAPPING-CONSTRAINT", "SUB-ELEMENT-MAPPINGS", "TEXTUAL-CONDITION", "FIXED-LENGTH", "OBD-DTC-VALUE-3-BYTE", "AGING-REQUIRES-TESTED-CYCLE", "ACCESS-COUNT", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "DPD-DELAY", "RX-ACCEPT-CONTAINED-I-PDU", "PERSISTENCY-DATA-ELEMENT", "SENDER-REC-RECORD-TYPE-MAPPING", "REQUIRED-CLIENT-SERVER-ENTRYS", "HANDLE-TERMINATION-AND-RESTART", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS", "ECUC-ADD-INFO-PARAM-VALUE", "MINIMUM-NUMBER-OF-REQUESTS", "IKE-AUTHENTICATION-METHOD", "SYNCHRONIZATION-CONSTRAINT-TYPE", "SW-COMPONENT-IREF", "DIAGNOSTIC-ECU-RESET-INTERFACE", "LIFE-CYCLE-INFOS", "CONTEXT-P-PORT-REF", "MIRROR-STATUS-CAN-ID", "DATA-UPDATE-PERIOD", "MAPPED-FUNCTION-REF", "FRAME-PID", "EVENT-REFS", "DDS-FIELD-DEPLOYMENT", "LOWER-CAN-ID", "TEMPLATE-ARGUMENTS", "MSR-QUERY-NAME", "TIME-BR", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "UCM-SOFTWARE-VERSION", "QUEUE-SIZE", "CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS", "USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL", "PID-REFS", "BLOCKLISTED-VERSIONS", "VENDOR-SPECIFIC-SERVICE-NEEDS", "TYP", "CONNECTED-INDICATORS", "DLT-APPLICATION-REF", "TD-HEADER-ID-FILTERS", "VG", "TIME-MASTER-PRIORITY", "TIME-SYNCHRONIZATION", "CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF", "TIME-VALUE", "DIAGNOSTIC-SOVD-ARRAY-CONTENT-ELEMENT", "I-SIGNAL-I-PDU-REF", "SWC-INTERNAL-BEHAVIOR", "APPLICATION-CONTEXT-REFS", "BSW-MODE-RECEIVER-POLICY", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NODE-MAPPING", "TEAR-DOWN-ACTION-REF", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "EVENT-DISPLACEMENT-STRATEGY", "MACHINE-ID", "PID-REF", "TRANSFER-REFS", "HW-TYPE", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "AP-SOMEIP-TRANSFORMATION-PROPS", "PAYLOAD-BYTE-PATTERN-RULE-PARTS", "SW-AXIS-GENERIC", "SECURE-COM-PROPS-REF", "DEFAULT-VALUE-HANDLING", "SYSTEM-DOCUMENTATIONS", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "LOWER-REF", "OPERATION-MODE", "UPPER-MULTIPLICITY", "SSP-OFFSET", "TTCAN-COMMUNICATION-CONNECTOR", "RPT-AR-HOOK-IREF", "EXECUTION-TIME-TYPE", "REFERENCE-VALUE-SPECIFICATION", "APMC-CONTAINER-REFERENCE-DEF", "IUMPR-GROUP-IDENTIFIERS", "SW-GENERIC-AXIS-PARAM-TYPE-REF", "SECOND-OPERATION-REF", "MAJOR-VERSION", "SW-SYSTEMCONST-VALUE", "CHECKPOINT-IREF", "SOURCE-MIN-ADDRESS", "MAX-DELTA-COUNTER-INIT", "TRANSFORMATION-DESCRIPTIONS", "SOURCE-SERVICE-INTERFACE-REFS", "RPT-IMPL-POLICY", "ACCEPTED-REMOTE-CERTIFICATE-REF", "SHALL-RUN-ON-REFS", "ELEMENT-CREATION", "MULTIPLE-CONFIGURATION-CONTAINER", "DYNAMIC-PARTS", "CRYPTO-KEY-SLOT-USAGE-DESIGN-MAPPING", "MC-PARAMETER-INSTANCES", "LIN-CLUSTER-CONDITIONAL", "ACCESSED-MODE-GROUPS", "TX-NM-PDU-REFS", "STATE-DEPENDENT-STARTUP-CONFIGS", "MAXIMUM-RESIDENCE-TIME", "BSW-EXCLUSIVE-AREA-POLICY", "TIMEOUT", "ASSIGNED-PORTS", "DECODING-CORRECTION", "PARAMETER-REQUIRE-COM-SPEC", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "LENGTH", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "FLEXRAY-CLUSTER-CONDITIONAL", "AGGREGATION", "EXECUTION-ORDER-CONSTRAINT-TYPE", "SCHEDULING-PRIORITY", "NETWORK-CONFIGURATION", "NM-DATA-DISABLED", "NETWORK-CONFIGURATION-REF", "UDP-PORT", "TIMING-MODE-REF", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "MAXIMUM-ALLOWED-SIZE", "CLASSIFICATIONS", "MAPPED-SWC-SERVICE-DEPENDENCY-IREF", "PROCESSORS", "TIME-FAILED-THRESHOLD", "CLIENT-POLICYS", "EVENT-RECEPTION-DEFAULT-VALUE", "TRIGGER-PORT-ANNOTATIONS", "CAN-ENTER-EXCLUSIVE-AREA-REFS", "SERVICE-INTERFACE-ID", "SOUCE-CAN-ID-REF", "ALLOW-PLAIN-TEXT-EXPORT", "FILTER-ACTION-VLAN-MODIFICATION", "SUBMESSAGE-TYPE", "STATUS-REF", "SOMEIP-TRANSFORMATION-PROPS-REF", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "STATE-MANAGEMENT-LEAVE-SUSPEND-TO-RAM-ACTION-ITEM", "STATE-DEP-FIREWALL-REF", "DIAGNOSTIC-FREEZE-FRAME", "SW-VALUE-CONT", "APPLIES-TO-DEPENDENCIES", "MIXED-BUS-TYPE-COLLECTION", "ECU-COMM-PORT-INSTANCES", "DIAGNOSTIC-ADDRESSES", "PERMISSION-CERTIFICATE-AUTHORITY-REF", "ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF", "DATA-TRANSFORMATIONS", "DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING", "DDS-I-SIGNAL-TO-TOPIC-MAPPINGS", "PROP-SEG", "CONTEXT-SW-COMPONENT-REF", "SW-ADDR-METHOD-REF", "AR-RELEASE-VERSION", "RPT-CONTAINERS", "INTERPOLATION-ROUTINE-REF", "DEFAULT-VALUE-ELEMENTS", "EVENT-HANDLER-REFS", "CUSTOM-SDG-DEF-REF", "SERVICE-TIMING", "PROVIDED-COM-SPECS", "SOVD-GROUP-REF", "VARIABLE-ACCESSED-FOR-DEBUG-REF", "DESTINATION-MAX-ADDRESS", "CAN-FRAME", "CONTAINED-AR-ELEMENT-REFS", "INTERFACE-MAPPINGS", "KEEP-ALIVE-PROBES-MAX", "HW-PIN-GROUP-REF", "CHECKPOINT-INITIAL-REFS", "TROUBLE-CODE-PROPSS", "TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "SW-RECORD-LAYOUT-V-FIX-VALUE", "FRAME-PORT", "STIM-ENABLER", "SECTION-NAME-PREFIXS", "SW-VARIABLE-REFS", "BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL", "ADDITIONAL-INFORMATION", "R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "KEY-SLOT-ONLY-ENABLED", "INTENDED-TARGET-MACHINE-REFS", "IMMEDIATE-RESUME-TIME", "STORE-CYCLIC", "CLOCK-REFERENCE-REF", "PER-INSTANCE-MEMORY-SIZE", "SW-COMPARISON-VARIABLES", "DESTINATION-TYPE", "DESTINATION-MAC-ADDRESS", "ATS-INSTANCE-ENTRY-REF", "PREDEFINED-TEXT", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS", "ABS", "END-2-END-EVENT-PROTECTION-PROPS", "SOURCE-CAN-ID-REF", "IDSM-RATE-LIMITATION-REF-CONDITIONAL", "DATA-PROTOTYPE-IN-METHOD-ARGUMENT-INSTANCE-REF-IREF", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "OWNERSHIP-KIND", "BUS-MIRROR-NETWORK-ID", "SUPPORTS-ASYNCHRONOUS-MODE-SWITCH", "UNIT-GROUP", "IDS-COMMON-ELEMENT-REF", "NO-SUPERVISION", "SECURE-COMMUNICATION-PROPS", "SW-RECORD-LAYOUT-V-INDEX", "NTU", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "CONSTANT-REFERENCE", "SD-CLIENT-EVENT-TIMING-CONFIG", "DURABILITY-SERVICE-HISTORY-DEPTH", "INTERPOLATION-ROUTINES", "GLOBAL-TIME-CAN-SLAVE", "POST-BUILD-VARIANT-VALUE", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "HAS-GETTER", "ENABLE-TAKE-ADDRESS", "PORT-DEFINED-ARGUMENT-BLUEPRINT", "OFFER-CYCLIC-DELAY", "ECUC-QUERY", "SYNC-FRAME-ID-COUNT-MAX", "RESULTING-PROPERTIES", "PERSISTENCY-FILE-ARRAY-REF", "SECURITY-EVENT-FILTER-CHAIN-REF", "PORT-PROTOTYPE-BLUEPRINT-REF", "MODULE-DESCRIPTION-REF", "TD-EVENT-TRIGGER-TYPE", "ETHERNET-PRIORITY-REGENERATIONS", "BSW-TIMING-EVENT", "CONSUMED-SERVICE-INSTANCES", "BSW-MODULE-DESCRIPTION-REF", "VARIABLE-INSTANCE-IREF", "TRUE-ACTION-LIST-REF", "IEEE-1722-TP-ACF-CONNECTION", "DO-IP-PROTOCOL-VERSION", "RPT-WRITE-ACCESS", "DISABLED-MODE-IREF", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "FLEXRAY-COMMUNICATION-CONTROLLER", "LDAP-URL", "CHECKSUM-VERIFICATION", "J-1939-REQUESTABLE", "REST-STRING-PROPERTY-DEF", "REFERENCE-TABLE-REF", "INCLUDED-DATA-TYPE-SET", "SW-CALPRM-AXIS", "TIME-OFFSET", "CLIENT-CAPABILITY", "NM-ECU", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE", "INITIAL-FIND-BEHAVIOR", "DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "RECORD-ELEMENT-MAPPINGS", "SERVICE-BUSY", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "BOR-TIME-L-1", "SOURCE-ADDRESS-CODE", "FRAME-TRIGGERING-REF", "PARAMETER-VALUES", "RPT-EXECUTABLE-ENTITY", "LENGTH-EXP", "OBJECT-REFS", "PARAMETER-DATA-PROTOTYPE", "DIAGNOSTIC-INDICATOR-NEEDS", "REMOTE-MULTICAST-CONFIG-REFS", "NM-WAIT-BUS-SLEEP-TIME", "ASSIGNED-DATAS", "SLOT-CAPACITY", "MEMORY-HIGH-ADDRESS-LABEL", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS", "NON-OS-MODULE-INSTANTIATION-REF", "NO-CHECKPOINT-SUPERVISIONS", "SUBSTITUTED-FRAME-REF", "DIAGNOSTIC-IO-CONTROL", "WRITE-ACCESS", "TYPE-BLUEPRINTS", "PNC-FILTER-DATA-MASK", "V-LAN-REFS", "START-AGENT-ERROR-REF", "STARTUP-CONFIG-SET", "TASK", "IS-FUNC-CLUSTER-ENDPOINT", "CYCLE-STATUS-STORAGE", "CONFLICTS-TO", "DCM-I-PDU", "PWM-S", "SERVICE-DISCOVER-CONFIGS", "SERVICE-INTERFACE-MAPPING", "STREAM-ID", "TCP-OPTION-FILTER-LISTS", "COUNTER-FAILED-THRESHOLD", "COUPLING-ELEMENT-SWITCH-DETAILS", "MODE-SWITCH-SENDER-COM-SPEC-PROPSS", "DURABILITY-SERVICE-HISTORY-KIND", "DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING", "TOOL-VERSION", "EXECUTION-DEPENDENCYS", "DIAG-ROUTINE-TYPE", "USE-MAC-ADDRESS-FOR-IDENTIFICATION", "COMPONENT-IMPLEMENTATION-REF", "SW-POINTER-TARGET-PROPS", "LOCAL-ENDPOINT-CONFIG", "EVENT-KIND", "ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS", "APMC-IP-V6-ADDRESS-PARAM-VALUE", "FLAT-MAP-ENTRY-REF", "BSW-INTERNAL-TRIGGERING-POINT-POLICY", "ARRAY-IMPL-POLICY", "ETHERNET-RAW-DATA-STREAM-GRANT", "REPETITION-PERIOD", "REQUESTABLE", "RPT-PROFILE", "CHECKPOINT-REF", "CRC-SECURED", "PHM-SUPERVISED-ENTITY-INTERFACE", "ENTRY", "CAN-BE-INVOKED-CONCURRENTLY", "DIAGNOSTIC-AGING", "STATE-MANAGEMENT-TRIGGER-COMPARE-RULE", "CONSUMED-EVENT-GROUP-REFS", "TRIGGER-PORT-ANNOTATION", "SECURE-COM-PROPS-FOR-UDP-REFS", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING", "BUILD-ACTION-ENVIRONMENTS", "INTERPOLATION-ROUTINE-MAPPING-SET-REF", "TP-SDU-REF", "JOB-PRIMITIVE-REF", "DESTINATION-URI-DEFS", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS", "PHM-HEALTH-CHANNEL-INTERFACE", "REMOTE-NETWORK-ENDPOINT-REF", "SOCKET-CONNECTION-BUNDLE", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "TFOOT", "PNC-VECTOR-LENGTH", "STATE-DEPENDENT-FIREWALL", "SIGNAL-FAN", "IDLE-SLOPE", "COUPLING-PORT-CREDIT-BASED-SHAPER", "ARTIFACT-CHECKSUMS", "PACKAGING-PIN-NAME", "GLOBAL-TIME-PROPS", "SWC-MODE-GROUP-IREF", "NETWORK-CONFIGURATION-DESIGN-REF", "IS-UNUSED", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "MODE-DECLARATION-REFS", "MAX-TRANSIT-TIME", "I-PDU-IDENTIFIER-UDP-REF", "REPOSITORY", "ROOT-SW-COMPOSITION-PROTOTYPE", "MODIFICATION", "DEFAULT-PATTERN", "ASSOCIATED-PDUR-I-PDU-GROUP-REFS", "SERVICE-CONTROL", "EVALUATED-VARIANT-REFS", "SUPPORT-INFO-BYTE", "SOCKET-CONNECTION-IPDU-IDENTIFIER", "MC-FUNCTION-REF", "MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "INITIAL-REPETITIONS-BASE-DELAY", "RPT-COMPONENTS", "MINISLOT-DURATION", "CIPHER-SUITE-NAME", "IT-REF", "XCP-PDU", "LOGICAL-SUPERVISION", "HEALING-CYCLE-REF", "SHORT-NAME", "V-2-X-M-USER-NEEDS", "TTCAN-CLUSTER-VARIANTS", "UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE", "COUPLING-PORT-RATE-POLICY", "BLOCK-IF-STATE-ACTIVE-AP-IREFS", "PHM-CONTRIBUTION-REFS", "METHOD-MAPPINGS", "RELIABILITY-KIND", "CRYPTO-KEY-MANAGEMENT-NEEDS", "OBJECT-DEFINITION-REF", "LOG-CHANNEL-ID", "EXPLICIT-WAKEUP-CHANNELS", "SOFTWARE-CLUSTER-DEPENDENCY-FORMULA", "INSTANTIATION-DATA-DEF-PROPSS", "ECUC-DESTINATION-URI-DEF", "MEMORY-SECTION-LOCATION", "INTERVAL-LENGTH", "MC-DATA-INSTANCE", "FEATURE-REF", "PROG-CODE", "SEGMENT-RECEPTION-TIMEOUT-TIME-REQUEST", "LIN-SLAVE-ECU-REF", "META-DATA-ITEM-SET", "PNC-WAKEUP-CAN-ID-MASK", "ENABLE-JOIN-ACCESS-CONTROL", "REMOTE-MODE-SENDER-QUEUE-LENGTH-CONTRIBUTION", "START-ON-EVENT-REF", "IS-PRIVATE", "MODE-TRANSITIONS", "SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "FLEXRAY-CLUSTER-VARIANTS", "BASE-TYPE-ENCODING", "DATA-TRANSFORMATION", "LOG-AND-TRACE-INSTANTIATION", "NM-LIGHT-TIMEOUT", "AFFECTED-STATE-IREF", "DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING", "MASTERED-TRIGGER-REF", "FUNCTIONAL-REQUEST-REFS", "ON-BOARD-MONITOR-ID", "MAC-ADDRESS", "ECUC-FUNCTION-NAME-DEF-CONDITIONAL", "PORT-INTERFACE-BLUEPRINT-REF", "CREATED-DATAS", "GETTER-OPERATION-REF", "LOCAL-CERTIFICATE-REF", "EVALUATED-ELEMENT-REFS", "AUTHENTICATION-REF", "THRESHOLD-NUMBER", "MKA-PARTICIPANT-REF", "FIELD-IREF", "DDS-CP-PARTITION", "CRYPTO-SERVICE-KEY-REF", "MAX-BLOCK-SIZE", "SW-AXIS-INDEX", "SOMEIP-FIELD-DEPLOYMENT", "EVENT-ELEMENTS", "SPECIFICATION-DOCUMENT-SCOPES", "ELEMENT-PROPSS", "REQUIRED-RTE-VENDOR", "UCM-SUBORDINATE-MODULE-INSTANTIATION", "BUS-DEPENDENT-NM-ECUS", "TRANSFORMATION-TECHNOLOGYS", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "DIAGNOSTIC-DATA-TRANSFER", "REQUIRED-SERVICE-VERSION", "TRIGGER-MAPPINGS", "DIAGNOSTIC-CONNECTION", "SD-CLIENT-TIMER-CONFIGS", "STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-OS-ACTION-ITEM", "AUTH-ALGORITHM", "RAW-DATA-STREAM-GRANT-DESIGN", "AP-APPLICATION-ENDPOINTS", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "KEY-GENERATION", "SOMEIP-SERVICE-DISCOVERY", "SYSTEM-COM-SPEC-DEFINITION-SET", "SOFTWARE-PACKAGE-STEPS", "MEMORY-DESTINATION-REF", "TCP-SYN-MAX-RTX", "PACKAGE-REF", "ICV-FRESHNESS-VALUE-ID", "STIMULUS-REF", "FLAT-INSTANCE-REF", "SERVICE-INTERFACE-EVENT-MAPPING", "ASSIGNED-FRAME-TRIGGERING-REF", "BYTE-VALUES", "RESPONSE-ON-SECOND-DECLINED-REQUEST", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "ORDERED-MASTER-LIST", "DIAGNOSTIC-OPERATION-CYCLES", "TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED", "RUNTIME-ERROR", "SLEEP-REPETITIONS-OF-SLEEP-REQUEST", "TCP-IP-NDP-DELAY-FIRST-PROBE-TIME-VALUE", "L-5", "STOP-AGENT-ERROR-REF", "WRITE-VERIFICATION", "CONFIGURED-NAD", "EXCLUSIVE-AREA-NESTING-ORDER-REFS", "FRAME-LENGTH", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "SECURE-COM-CONFIG-REF", "SW-IS-VIRTUAL", "REFERENCE-CONDITION", "TCP-IP-IP-REASSEMBLY-TIMEOUT", "ECUC-DESTINATION-URI-DEF-SET", "MODEL-OBJECT-REFERENCE", "SOMEIP-FIELD", "PERSISTENCY-REDUNDANCY-M-OUT-OF-N", "RESOURCE-REF", "USE-INSTEAD-REFS", "SW-CALIBRATION-ACCESS", "PN-RESET-TIMER", "ACCESS-COUNTS", "LC-STATE-REF", "LOWER-BOUND", "CRYPTO-KEY-SLOT-USER-DESIGN", "ECUC-MODULE-CONFIGURATION-VALUES-REF", "IIDC-STREAM-FORMAT", "BSW-DATA-RECEIVED-EVENT", "REQUIRED-EVENT-GROUPS", "DATA-TYPE-REFS", "TEMPERATURE-EXP", "COMPONENT-IREFS", "ALLOCATOR-REF", "RESTRICTIONS", "SHARED-TIMER", "IMPLICIT-INTER-RUNNABLE-VARIABLES", "IDS-TIME-BASE-REF", "SERVICE-INTERFACE-SUB-ELEMENT", "USE-VEHICLE-IDENTIFICATION-SYNC-STATUS", "INVOCATION", "DLT-MESSAGE", "COM-CONFIGURATION-GW-TIME-BASE", "CONTEXT-SWC-PROTOTYPE-REFS", "COM-TRIGGER-GRANT", "USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS", "START-ADDRESS", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "DO-IP-TARGET-ADDRESS-REFS", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF", "ASSOCIATED-PDUR-I-PDU-GROUP-REF", "SW-CALPRM-PROTOTYPE-REF", "INFO-TYPE-REF", "SW-RECORD-LAYOUT-V-PROP", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "HAS-DYNAMIC-LENGTH", "COM-TRIGGER-GRANT-DESIGN", "ROLE-BASED-PORT-ASSIGNMENT", "SELECTOR-FIELD-LENGTH", "PDU-REF", "STATE-MANAGEMENT-PHM-ERROR-INTERFACE", "DPG-REQUIRES-COHERENCYS", "FUNCTION-GROUP-SET", "PROVIDER-IREF", "CIPHER-SUITE", "DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "ACCESS-DATA-RECORD-SIZE", "PROCESS-TO-MACHINE-MAPPINGS", "SDG-TAILORING", "HW-ATTRIBUTE-DEF", "DATA-ID-NIBBLE-OFFSET", "PROVIDED-SERVICE-INSTANCE-REF", "MODE-USER-ERROR-BEHAVIOR", "SOVD-SERVER-INSTANTIATION", "SWC-BSW-MAPPING", "CONTAINED-I-PDU-TRIGGERING-PROPSS", "PAYLOAD-BYTE-PATTERN-RULE-PART", "PNC-IDENTIFIER", "BLUEPRINT-REF", "MODULE-REFS", "SUPPORT-DIRTY-FLAG", "TRANSFORMATION-TECHNOLOGY", "RESOURCE-CONSUMPTION", "IE", "DIAGNOSTIC-SOVD-ACCESS-ARGUMENT", "DISPLAY-NAME", "COUNTER-OFFSET", "PNC-MAPPINGS", "P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "IMPLEMENTATION-ARRAY-ELEMENT-REF", "BUS-MIRROR-CHANNEL-MAPPING-IP", "SOURCE-ADDRESS-RANGE-START", "FLEXRAY-TP-PDU-POOL", "NM-DATA-CYCLE", "TCP-INITIAL-INACTIVITY-TIME", "SW-CLUSTER-REF", "APMC-FOREIGN-REFERENCE-DEF", "NOTATION", "BSW-MGR-NEEDS", "NOMINAL", "DEPENDENCIES", "APPLICATION-PRIMITIVE-DATA-TYPE", "THRESHOLD-SIZE", "CYCLE-TIME-MAIN-FUNCTION", "NETWORK-INTERFACE-ID", "LOAD-BALANCING-WEIGHT", "IS-SERVICE", "RPT-EXECUTABLE-ENTITY-PROPERTIES", "CAN-TP-ECU", "MAX-GRADIENT", "RECOVERY-ACTION-IREF", "DEFAULT-APPLICATION-TIMEOUT", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "FIRE-AND-FORGET", "HW-COMMUNICATION-PORT-REF", "J-1939-NM-NODE-REF", "SOMEIP-SD-RULE", "SOMEIP-PROVIDED-EVENT-GROUP", "TCP-IP-IP-REASS-TIMEOUT", "INTENDED-TARGET-MACHINE-REF", "ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS", "TRANSMIT-CANCELLATION", "FLEXRAY-AR-TP-CHANNEL", "TIMING-ARGUMENT-REF", "SECURITY-EVENT-CONTEXT-DATA", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "TROUBLE-CODE-UDS-REF", "SINGLE-ELEMENT-SERIALIZATION", "SWC-MODE-MANAGER-ERROR-EVENT", "BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE", "ROW", "HW-PIN-GROUP-REFS", "FRESHNESS-VALUE-ID", "OS-ARTI-ADAPTER-LAUNCH-BEHAVIOR", "SUB-ELEMENTS", "ECU-STATE-MGR-USER-NEEDS", "META-DATA-ITEMS", "P-PORT-IN-COMPOSITION-INSTANCE-REF", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "MODE-DECLARATION-MAPPING-SET-REF", "MATCHING-INGRESS-RULE-REF", "APPLICATION-TYPE-REF", "EVENT-OCCURRENCE-KIND", "SYNC-NODE-MAX", "TARGET-P-PORT-PROTOTYPE-REF", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING", "LIN-SCHEDULE-TABLE", "DDS-CP-DOMAIN", "TYPE-MAPPING", "LIN-UNCONDITIONAL-FRAME-REFS", "CAN-COMMUNICATION-CONNECTOR", "CRYPTO-OBJECT-TYPE", "VEHICLE-SYSTEM", "DEFAULT-VALUE", "AGGREGATION-TAILORING", "ASSIGNED-TRAFFIC-CLASSS", "RUNNABLE-ENTITY-GROUP-IREF", "CAN-TP-NODE", "CONDITION-FORMULA", "USE-CASE", "DO-IP-LOGIC-ADDRESSS", "SW-RECORD-LAYOUT-GROUP-INDEX", "SECURE-PROPS-FOR-EXTERNAL-COMM-REF", "INPUT-VARIABLE-TYPE-REF", "SESSION-HANDLING-SR", "RESPONSE-ON-EVENT-REF", "MAX-CYCLE-REPETITIONS", "EVENT-COMBINATION-REPORTING-BEHAVIOR", "STORE-IMMEDIATE", "SUSPEND-TO-RAM-SATELLITE-INTERFACE", "COMMUNICATION-DIRECTION", "PORT-INTERFACE-MAPPINGS", "COMMUNICATION-RESOURCE-PROPS", "CALLBACK-HEADER-REFS", "STORAGE-STATE-EVALUATION", "INTERFACE-BLUEPRINT-MAPPING", "MIN-PWM-O", "CUSTOM-TRIGGER", "DERIVED-FROM-BLUEPRINT-REFS", "CRC-MESSAGE-LENGTH", "DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT", "MAX-NUMBER-OF-TIME-QUANTA-PER-BIT", "MAC-SEC-CIPHER-SUITE-CONFIG", "ERROR-CONTEXT-REF", "SIGNAL-BASED-EVENT-DEPLOYMENT", "SIZE-OF-STRUCT-LENGTH-FIELDS", "BIT", "APPLICATION-ASSOC-MAP-DATA-TYPE", "ASSIGNED-ENTRY-ROLES", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "NM-IF-ECU-REF", "TP-CONNECTION-REF", "MEMORY-SECTION-LOCATIONS", "CONTROL-PNC-REF", "ECUC-VALUE-CONFIGURATION-CLASS", "UDP-NM-ECU", "SW-DATA-DEPENDENCY", "TRANSFORMATION-COM-SPEC-PROPSS", "SERVICE-INTERFACE-TRIGGER-MAPPING", "DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "CERTIFICATE-TO-KEY-SLOT-MAPPINGS", "REFERENCED-BASELINE", "MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING", "ASSIGN-FRAME-ID-RANGE", "SOVD-LOCK-REF", "OPERATION-CYCLE-REF", "SUPERVISION-CONDITION", "VALUE-ERROR-COUNT-ENABLED", "DIAGNOSTIC-ENV-DATA-ELEMENT-CONDITION", "EXECUTE-DESPITE-DATA-UNAVAILABILITY", "COMPONENT-CLUSTERING", "READ-STATUS", "DIAGNOSTIC-SOVD-LOCK", "STEP-SIZE", "MAX-ERROR-STATE-INVALID", "SDG-TAILORINGS", "SW-MAPPINGS", "MULTICAST-SEGMENTATION", "IEEE-1722-STREAM-REF", "PROVIDER-RATE-DEVIATION-MAX", "DATA-ELEMENT-IREFS", "TCP-TTL", "ROUGH-ESTIMATE-STACK-USAGE", "LIN-CONFIGURABLE-FRAME", "SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA", "TRANSPORT-PRIORITY", "ROOT-VARIABLE-DATA-PROTOTYPE-REF", "DISPLAY-IDENTIFIER", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS", "MODE-DRIVEN-TRANSMISSION-MODE-CONDITION", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "SERVER-COM-SPEC-PROPS", "SERVER-NAME-IDENTIFICATION", "RPT-PREPARATION-LEVEL", "ROOT-REF", "NM-MSG-CYCLE-OFFSET", "MC-DATA-INSTANCE-REFS", "SW-COMPONENT-PROTOTYPE", "MULTIPLICITY-CONFIG-CLASSES", "CONTEXT-ROOT-COMPOSITION-REF", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL", "FUNCTION-GROUP-STATE-IREF", "IP-SEC-RULE", "TD-EVENT-TT-CAN-CYCLE-START", "WINDOW-SIZE", "DDS-TOPIC-ACCESS-RULE", "ECU-PARTITION-TO-CORE-MAPPING", "TIMING-RESOURCE", "REQUIRED-DESIGN-ELEMENT-REF", "INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "UCM-STEP", "BSW-QUEUED-DATA-RECEPTION-POLICY", "SW-RECORD-LAYOUT-GROUP-STEP", "MIN-MAJOR-VERSION", "DOIP-CHANNEL-COLLECTION-REF", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE", "TIME-CS", "RESOURCE-GUARD-VALUE", "DLT-CONTEXT", "DIAGNOSTIC-J-1939-SPN-MAPPING", "TIME-BASE", "NETWORK-LAYER-RULE", "CHANNEL-REF", "REMOTE-CONFIGS", "GLOBAL-SUPERVISIONS", "APPLICATION-ENDPOINT-REF", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "SECOND-MODE-REF", "DDS-REQUIRED-SERVICE-INSTANCE", "REPLICA-PDUS-REF", "DYNAMIC-ARRAY-SIZE-PROFILE", "SW-MC-BASE-TYPE", "SERVER-SERVICE-MINOR-VERSION", "USE-CRC-COMP-MECHANISM", "EVENT-HANDLERS", "IS-ACTIVATION-LINE-DEPENDENT", "COMPOSITE-NETWORK-REPRESENTATIONS", "FM-ATTRIBUTE-VALUE", "CAN-XL-NM-PROPS", "MIN-TRCV-DELAY-COMPENSATION-OFFSET", "DIAGNOSTIC-CONDITION-INTERFACE", "PORT-PROTOTYPE-BLUEPRINT-MAPPING", "RESOURCE-DEFINITIONS", "REST-ENUMERATOR-DEF", "MANAGED-MODE-GROUPS", "SW-INTERPOLATION-METHOD", "DO-IP-ENTITY-ROLE", "TLV-DATA-ID", "PSK-IDENTITY-HINT", "SECURE-COMMUNICATION-PROPS-SET", "EMULATION-SUPPORTS", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS", "MAPPING-SCOPE", "LOCAL-COMM-CONNECTOR-REF", "MAXIMUM-INTER-ARRIVAL-TIME", "USED-SECONDARY-FID-REF", "TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION", "PDU-TRIGGERING", "SERVICE-ELEMENT-SECURE-COM-CONFIG-REF", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "FILE-TYPE-PATTERN", "E-2-E-NO-NEW-DATA-EXTENSION-FACTOR", "PAYLOAD-PREAMBLE-INDICATOR", "ABSTRACT-CLASS-TAILORING", "ECUC-DEFINITION-COLLECTION", "CONSUMED-OPERATIONS", "BINARY-MANIFEST-PROVIDE-RESOURCE", "ECUC-VALUES", "SWC-TO-APPLICATION-PARTITION-MAPPING", "TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS", "WATCHDOG-PHM-ACTION-ITEM", "DATA-RECEIVE-ERROR-EVENT", "NETWORK-TIME-MASTER-REF", "HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS", "CLEAR-DTC-LIMITATION", "SOURCE-ELEMENT-REF", "ASSIGNED-ENTRY-REF", "FINAL-CHECKPOINT-REF", "KEY-SLOT-REF", "STD-REF", "PROVIDE-RESOURCES", "MAX-NUM-COMPARISION-OF-VALUE-EVENTS", "KEY-SLOT-ID", "SPECIFICATION-DOCUMENT-SCOPE", "COMPONENT-QUALIFIER", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS", "RECEIVE-ACTIVITY", "DPD-ACTION", "TP-CONCURRENT-PROCESSING-SDUS", "RELEASED-TRIGGER-REF", "IMPLEMENTED-IN-REF", "PATTERN", "MAXIMUM-SUPPORTED-UCM-VERSION", "EXPIRED-SUPERVISION-CYCLES-TOLERANCE", "SD-SERVER-CONFIG", "AUTH-ROLE-REF", "I-SIGNAL-GROUP", "PHM-ACTION-LIST", "VALID-BINDING-TIMES", "DIAGNOSTIC-ROUTINE", "OPT-IMPL-RECORD-ELEMENT", "TT-CAN-CLUSTER-REF", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "MAC-MULTICAST-ADDRESS", "SW-GENERIC-AXIS-PARAM-TYPE", "BASIC-CYCLE-LENGTH", "CERTIFICATE-REF", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "RPT-SERVICE-POINT-PRE-REF", "AUTHENTICATION-ROLE-REF", "SETTER-CALL-SIGNAL-REF", "DATA-PROTOTYPE-IREF", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF", "DDS-METHOD-QOS-PROPS", "DLT-CONTEXT-REF", "IPV-6-PROPS", "COM-METHOD-GRANT-DESIGN", "I-PDU-MAPPING", "RELEASED-TRIGGERS", "FIREWALL-REF", "IDENTITY", "NM-REPEAT-MESSAGE-TIME", "LIN-SPORADIC-FRAME", "REMOTE-UNICAST-CONFIG-REFS", "MEASURED-HEAP-USAGE", "FLEXRAY-TP-CONNECTION", "APMC-STRONG-REVISION-LABEL-PARAM-VALUE", "PORT-PROTOTYPE-IREF", "VLAN-MEMBERSHIPS", "MODE-IREFS", "SW-RECORD-LAYOUT-COMPONENT", "FIREWALL-STATE-IREFS", "RVF-INTERLACED", "POST-BUILD-VARIANT-USED", "PDU-COLLECTION-PDU-TIMEOUT", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF", "SECTION-TYPE", "ACCESS-COUNT-SET", "BSW-SCHEDULABLE-ENTITY", "SW-COMP-TO-ECU-MAPPING-REF", "PREVIOUS-CONTRACT-VERSION", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "MINOR-VERSION", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS", "TERMINATION-BEHAVIOR", "RPT-SERVICE-POINTS", "APMC-CHOICE-CONTAINER-DEF", "DIAGNOSTIC-EVENT-INTERFACE", "IP-SEC-IAM-REMOTE-SUBJECT", "SW-RECORD-LAYOUT-GROUP-AXIS", "REQUEST-CONFIGURATION-DESIGNS", "ACTIVATION", "TCP-IP-ICMP-PROPS-REF", "HISTORY-KIND", "NETWORK-CONFIGURATION-REFS", "BUFFER-OUTPUT", "LIN-UNCONDITIONAL-FRAME-REF", "MINIMUM-FIFO-LENGTH", "ALIVE-TIMEOUT", "MEMORY-SECTIONS", "MINIMUM", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "REQUESTOR-REFS", "IMPLEMENTATION-REFS", "SYNCHRONIZED-TIME-BASE-PROVIDER", "SUB-DOMAIN-REFS", "FIRST-ELEMENT-INSTANCE-IREFS", "WAKE-UP-PATTERN", "EVENT-MULTICAST-UDP-PORT", "DETERMINISTIC-SYNC-MASTER", "MAX-DID-TO-READ", "DIAGNOSTIC-SERVICE-INSTANCE-REF-CONDITIONAL", "PLCA-MAX-BURST-COUNT", "REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF", "ALLOWED-KEYSLOT-USAGE", "SOFTWARE-CLUSTER-DESIGN-DEPENDENCY", "MODE-SWITCHED-ACK", "REST-ARRAY-PROPERTY-DEF", "CAN-TP-ADDRESS-REF", "I-SIGNAL-PROPS", "COLLECTION", "SENSOR-INSTANCE-ID", "SAK-REF", "WINDOW-SIZE-VALID", "LABEL", "EXPECTED-ENTRYS", "ARGUMENT-DATA-PROTOTYPE", "REQUIRED-MINOR-VERSION", "IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER", "CONSUMED-SERVICE-INSTANCE-REF", "TCP-IP-DHCP-V-6-INF-DELAY-MIN", "AUTOSAR-VARIABLE", "SOMEIP-TP-CONFIG", "AUTO-IP-PROPS", "RATE-CORRECTION-OUT", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "USED-LANGUAGES", "DIAGNOSTIC-MONITOR-INTERFACE", "PRIORITY-ID", "RANGE-MAX", "TARGET-EVENT-REFS", "SW-COMPONENT-ASSIGNMENTS", "APPLICATION-RECORD-DATA-TYPE", "LABELED-ITEM", "VIEW-MAPS", "PER-INSTANCE-MEMORY-SIZES", "TCP-LOCAL-PORT", "TOLERATED-FAILED-CYCLES", "TRANSPORT-PROTOCOL", "PROVIDED-DDS-OPERATIONS", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "EXPLICIT-CONGESTION-NOTIFICATION", "VERIFICATION-HASH", "SPEEDUP", "HOP-COUNT", "DERIVED-FROM-BLUEPRINT-REF", "SERVER-ARGUMENT-IMPL-POLICY", "UNIT-REF", "CLASS-CONTENT-CONDITIONAL", "CLIENT-SERVICE-MAJOR-VERSION", "DTC-PROPS-REF", "ARRAY-VALUE-SPECIFICATION", "AUTOSAR-CONNECTOR", "ACCESSED-VARIABLE-REF", "NUMBER-OF-STATIC-SLOTS", "WORST-CASE-HEAP-USAGE", "HW-PIN-GROUP-CONNECTOR", "BEHAVIOR-REF", "DYNAMIC-RUNTIME-LENGTH-HANDLING", "DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING", "APPLIED-STANDARD", "QUEUE-LENGTH", "RECOMMENDED-CONFIGURATION-REFS", "R-PORT-PROTOTYPE-REF", "BYTE-VALUE", "MIN-OK-STATE-INIT", "MANAGED-PHYSICAL-CHANNEL-REF", "SELECTOR-FIELD-START-POSITION", "PDU-REPLICATION-VOTING", "HANDLE-TIMEOUT-TYPE", "EXECUTION-TIME-CONSTRAINT", "REQUIRED-ENVIRONMENT-REF", "TLV-DATA-ID-DEFINITION", "RPT-EXECUTION-CONTROL", "RAW-DATA-STREAM-INTERFACE", "HAS-INTERNAL-STATE", "INTERNAL-BEHAVIOR", "STATE", "CHANNEL-SYNCHRONOUS-WAKEUP", "DELAY-COMPENSATION-B", "SEND-RESP-PEND-ON-TRANS-TO-BOOT", "PDU-TRIGGERINGS", "EXECUTABLE", "PERSISTENCY-FILE-PROXY", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "LISTEN-NOISE", "ESP-CIPHER-SUITE-NAME", "GROUPED-ALIAS-EVENT-REF", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "UPDATE", "EVENT-QOS-PROPSS", "PARENT-CATEGORY", "MATCHING-EGRESS-RULE-REFS", "ELEMENT-REFS", "COMMUNICATION-CONNECTORS", "PUBLISHER", "READER-ENTITY-ID", "TABLE-ENTRYS", "DDS-QOS-PROFILES", "RUNNABLE-ENTITY", "INCLUDED-DATA-TYPE-SETS", "UNRESOLVED-REFERENCE-RESTRICTION", "IDSM-MODULE-INSTANTIATION-REF", "PROCESS", "TRANSMISSION-START-SEQUENCE-DURATION", "CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "DIAGNOSTIC-SOVD-GROUP", "ALIVE-SUPERVISION", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "SOFTWARE-CLUSTER-DESIGN", "PNC-GROUP-REFS", "DLT-APPLICATION-TO-PROCESS-MAPPING", "PROVIDED-SERVICE-INSTANCES", "RVF-FRAME-RATE", "J-1939-TP-NODE", "PDU-COLLECTION-TRIGGER", "DIAGNOSTIC-TROUBLE-CODE-PROPS-REF-CONDITIONAL", "SIGNAL-BASED-EVENT-REF", "COMPILERS", "FLEXRAY-TP-CONFIG", "SDG-PRIMITIVE-ATTRIBUTE", "LIVELINESS-LEASE-DURATION", "TRIGGER-TO-SIGNAL-MAPPING", "SW-GENERIC-AXIS-PARAM", "PROGRAMMING-LANGUAGE", "SERVICE-SW-COMPONENT-TYPE", "LC-STATES", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "TRCV-PWM-MODE-ENABLED", "USE-CLIENT-AUTHENTICATION-REQUEST", "DID-SIZE", "DYNAMIC-ACTION-REF", "PORT-REF", "LANGUAGE", "ESTIMATED-EXECUTION-TIME", "TEST-RESULT-REF", "ATTRIBUTE-TAILORINGS", "SIZE", "SETTER-OPERATION-REF", "ARTIFACT-CHECKSUM", "MAX-AR", "RUNNABLE-MAPPINGS", "ECUC-VALIDATION-CONDS", "RUNNABLE-ENTITY-IREF", "FREEZE-FRAME-REF", "METHOD-RESPONSE-PROPSS", "ADD-INFO-5", "TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC", "MAXIMUM-SELECTED-BINDING-TIME", "SEC-OC-JOB-MAPPINGS", "PACKAGER-ID", "LIST", "TCP-IP-NDP-MAX-RANDOM-FACTOR", "SEGMENT-REF", "SW-VALUE-BLOCK-SIZE-MULTS", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING", "APPLICATION-DATA-TYPE-REF", "CONTENT-ELEMENT-REF", "PLCA-PROPS", "SEGMENT-RECEPTION-TIMEOUT-TIME", "IEEE-1722-RAW-DATA-STREAM-CONSUMER-INTERFACE", "DDS-METHOD-REPLY-TOPIC-REF", "MINIMUM-MEMORY-CONSUMPTION", "J-1939-DTC-VALUE", "CONTEXT-TYPE-LIST", "RTE-EVENT-IREF", "APMC-INTEGER-PARAM-DEF", "SOFTWARE-ACTIVATION-DEPENDENCY-REF", "INTERNAL-PRIORITY-VALUE", "CONSUMED-EVENT-GROUP", "TARGET-I-PDU", "SOCKET-OPTION", "IMPOSITION-TIME", "OUT-MEASUREMENT-SET", "TCP-IP-NDP-PREFIX-LIST-SIZE", "START-AGENT-REF", "DO-IP-SOURCE-ADDRESS-REF", "BASE-IS-THIS-PACKAGE", "VLAN", "SECURITY-DELAY-TIME-ON-BOOT", "FIBEX-ELEMENT-REF-CONDITIONAL", "COLLECTION-REF", "TRACE-SWITCH-CONFIG", "EXPECTED-TX-TRIGGER", "GLOBAL-IN-PACKAGE-REF", "SUPPLIER-VALIDATION-ORDER-REFS", "STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE", "TP-NODES", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "DIAGNOSTIC-AUTH-ROLE", "UCM-DESCRIPTION", "NEXT-HEADER", "UPPER-MULTIPLICITY-INFINITE", "BLUEPRINT-MAPPING-GUIDE", "EID-USE-MAC", "INITIAL-DELAY-MAX-VALUE", "CLIENT-ID-RANGE", "BLUEPRINT-DERIVATION-GUIDE", "SW-ADDR-METHOD", "NV-RAM-BLOCK-ELEMENT", "STARTUP-OPTIONS", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING", "SEND-POLICYS", "TARGET-DATA-PROTOYPE-REF", "SHORT-TERM-ADJUSTMENT-SUPPORTED", "END-TO-END-PROTECTION", "MAX-BUFFER-SIZE", "SW-COMPONENT-PROTOTYPE-IREF", "RESTRICTION", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "REQUEST-ARGUMENT-REF", "FEATURE-IREF", "SECOND-TO-FIRST-DATA-TRANSFORMATION-REF", "REQUEST-FILE-TRANSFER-CLASS-REF", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "GLOBAL-SUPERVISION", "CONTEXT-APPLICATION-RECORD-ELEMENT-REF", "TLS-JOB-SEMANTIC", "ENABLE-WRITE-ACCESS-CONTROL", "TCP-CONNECT-TIMEOUT", "MACRO-INITIAL-OFFSET-B", "COMMANDS", "TIME-SYNC-SERVER-IDENTIFIER", "CLIENT-SERVER-OPERATION", "SEPARATED-COMPONENT-IREFS", "CONTEXT-SW-COMPONENT-PROTOTYPE-REF", "FIGURE-CAPTION", "SW-DIAGNOSTICS-NOTES", "NV-REQUIRE-COM-SPEC", "END-2-END-METHOD-PROTECTION-PROPS", "TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR", "LOG-SINK-REFS", "REMOTE-IP-ADDRESS-REFS", "PNC-GROUP-REF", "SUB-ELEMENT-MAPPING", "LIFESPAN-DURATION", "MODE-SWITCHED-ACK-EVENT", "ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT", "PROVIDED-MEMORY-REF", "DETERMINISTIC-SYNC-MASTER-REF", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL", "PRIMITIVE", "TCP-IP-NDP-DEFAULT-RETRANS-TIMER", "COMMAND", "PATTERN-PERIOD", "PORT-PROTOTYPE-REF-CONDITIONAL", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "SECURITY-EVENT-REPORT-INSTANCE-VALUE", "POSSIBLE-ERROR-REACTION", "LEGISLATED-SOVD-FREEZE-FRAME-CONTENT-REF", "SCHEDULER-MAX-NUMBER", "ROOT-CONTEXT-REF", "AUDIENCES", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF", "DISCOVERY-TECHNOLOGY", "AGING-ALLOWED", "APMC-DEFINITION-COLLECTION", "REQUIRED-TRIGGERS", "ITEM-LABEL", "ESP-CIPHER-SUITE-NAMES", "PERSISTENCY-DATA-PROVIDED-COM-SPEC", "DDS-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE", "TD-EVENT-OPERATION", "MAX-SAMPLES-PER-INSTANCE", "TRANSMISSION-TRIGGER", "MESSAGE-LINK-LENGTH", "IDSM-RATE-LIMITATION", "SECOND-ELEMENTS", "AUTOSAR-ENGINEERING-OBJECT", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF", "SERVICE-INTERFACE", "PHM-LOGICAL-EXPRESSION", "HARDWARE-PLATFORM", "SOURCE-ELEMENT-REFS", "MEM-CLASS-SYMBOL", "INGRESS-PORT-REF", "REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS", "PHYSICAL-PROPS", "LICENSE-REF", "DIAGNOSTIC-CONTROL-DTC-SETTING", "CRYPTO-KEY-SLOT-USAGE-DESIGN-REF", "SENDER-ANNOTATION", "FLEXRAY-NM-NODE", "PNC-WAKEUP-DLC", "SW-DATA-DEPENDENCY-ARGS", "SUPPORT-BUFFER-LOCKING", "FIREWALL-STATE-SWITCH-INTERFACE", "TD-EVENT-MODE-DECLARATION", "SPEED", "MASS-EXP", "MAX-SOURCE-ELEMENT", "DIAGNOSTIC-PROTOCOL", "OS-TASK-EXECUTION-EVENT", "TLS-CONNECTION-REF", "HW-ELEMENT-CONNECTOR", "BLUEPRINT-POLICYS", "RECEIVER-CAPABILITY", "ECU-PARTITION-REF", "SW-DATA-DEF-PROPS-VARIANTS", "REFERENCE-BASE", "RESPONSE-ON-ALL-REQUEST-SIDS", "NM-NODE-REF", "PNC-MAPPING-IDENT-REF", "SW-SYSTEMCONSTANT-VALUE-SET-REFS", "DESTINATION-ORDER", "TRACE-MESSAGE-REF", "REPORTING-BEHAVIOR", "ECUC-ADD-INFO-PARAM-DEF", "REPLY-TOPIC-NAME", "MINIMUM-TIMER-GRANULARITY", "ECU-ID", "I-SIGNAL-TRIGGERING", "ANONYMOUS", "VCID", "CONTEXT-DATA-ELEMENTS", "ENABLE-CONDITION-GROUP-REF", "EVALUATION-ID", "INCLUDED-CERTIFICATE-REFS", "MODE-DECLARATION-MAPPINGS", "SW-MAINTENANCE-NOTES", "DIAGNOSTIC-ECU-RESET", "PROCESS-REF", "ISSUED-BY", "OBJECT-DEFINITION-REFS", "RX-PDU-TRIGGERING-REF", "AUTOSAR-PARAMETER-IREF", "SUB-TLV-CONFIG", "INTERNAL-CONSTRS", "HW-PIN-GROUPS", "IEEE-1722-RAW-DATA-STREAM-PRODUCER-MAPPING", "DTC-STATUS-MASK", "STORE-EMERGENCY", "LET-DATA-EXCHANGE-PARADIGM", "CONTENT-OBJECT", "LIN-TP-N-SDU-REF", "DEBOUNCE-TIME", "IPV-6-DHCP-SERVER-CONFIGURATION", "FLEXRAY-FIFO-CONFIGURATION", "SDG-AGGREGATION-WITH-VARIATION", "UDS-SERVICE", "NM-PDUS", "FALL-BACK-INTERNAL", "MODE-DECLARATION", "MKA-PARTICIPANTS", "NV-DATA-PORT-ANNOTATION", "SESSION-ID-SUPPORT", "MIN-SOURCE-PORT-NUMBER", "FILE-ELEMENTS", "COMPOSITION-SW-COMPONENT-TYPE", "FUNCTIONAL-CLUSTER-REF", "ARGUMENT-MAPPINGS", "COLLECTED-INSTANCE-IREF", "TIMING-CLOCK-SYNC-ACCURACYS", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "MODE-DRIVEN-TRUE-CONDITIONS", "EXECUTABLE-TIMING", "CLIENT-SERVER-INTERFACE-REF", "BEHAVIOR", "IDENT", "IP-SEC-CONFIG-PROPS-REF", "PID-SIZE", "ERROR-DOMAIN-REF", "IP-SEC-CONFIG", "SDU-TYPE", "SECURITY-EVENT-MAPPING", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE", "OPERATION-CYCLE-AUTOSTART", "ECUC-MODULE-DEF", "COMMUNICATION-CONTROLLERS", "DIAGNOSTIC-COM-CONTROL", "MACHINE-MODE-IREF", "PROVIDED-EVENTS", "SYNCHRONIZATION-TIMING-CONSTRAINT", "LENGTH-CLIENT-ID", "CONTAINED-FIBEX-ELEMENT-REF", "COM-CONFIGURATION-RX-TIME-BASE", "REQUEST-RESPONSE-DELAY", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "PARTITIONS", "LOG-TRACE-PORT-ID", "PASS-THROUGH-SW-CONNECTOR", "SSRC", "THRESHOLD", "ENVIRONMENT-VARIABLES", "RTE-EVENT-IN-SYSTEM-SEPARATION", "ACL-ROLE-REF", "CHANNELS-PER-FRAME", "ALLOWED-TCP-OPTION", "EVENT-MAPPING", "TARGET-I-PDU-REF", "DLT-ARGUMENT-PROPS-REF", "TRANSFORMATION-PROPS-REF", "DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT", "INITIAL-OFFER-BEHAVIOR", "TEXT-TABLE-VALUE-PAIR", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "TCP-IP-NDP-DEFAULT-REACHABLE-TIME", "ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT", "TIME-PASSED-THRESHOLD", "I-PDU-TIMING", "PERSISTENCY-PROVIDED-COM-SPEC", "EXPLICIT-INTER-RUNNABLE-VARIABLES", "FIELD-REQUEST-TOPIC-NAME", "TRANSFORMATION-PROPSS", "CP-SOFTWARE-CLUSTER-RESOURCE-REF", "MAC-SEC-GLOBAL-KAY-PROPS", "BSW-MODE-SWITCHED-ACK-EVENT", "LET-INTERVAL-REFS", "IEEE-1722-TP-ETHERNET-FRAME", "FUNCTION-NAMES", "END-TO-END-PROTECTIONS", "RESPONSE-DATA-SIZE", "CAN-XL-PROPS", "HW-ATTRIBUTE-DEFS", "TYPE-REFERENCE-REF", "LIFE-CYCLE-STATE", "PATH-MTU-TIMEOUT", "OUTER-PORT-REF", "SYSTEM-VERSION", "TRANSFORMER-CHAIN-REF", "ACCESSED-DATA-PROTOTYPE-IREF", "APPLICATIONS", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT", "CAN-CONTROLLER-XL-REQUIREMENTS", "NOTIFIER-SIGNAL-TRIGGERING-REF", "MEMORY-LOW-ADDRESS", "SHORT-TERM-ADJUSTMENT", "VARIATION-POINT-PROXYS", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS", "SIMULATED-EXECUTION-TIME", "DOIP-INTERFACES", "VALUE", "BUILD-ACTION-IO-ELEMENT", "DERIVED-CERTIFICATE-ACCEPTED", "BY-PASS-POINT-IREF", "OUT-MEASURMENT-SET", "STORE-AT-SHUTDOWN", "CRYPTO-PROVIDERS", "MATCHING-EGRESS-RULE-REF", "CONDITION-ACCESS", "ROM-BLOCK", "DIAGNOSTIC-COMMON-PROPS-VARIANTS", "DDS-FIELD-REPLY-TOPIC-REF", "DLT-APPLICATION", "NODE-REF", "ACTION-LISTS", "SOURCE-EEC-REFS", "INITIAL-DYNAMIC-PART", "LIN-ERROR-RESPONSE", "PORT-NUMBER", "PID", "PNC-PREPARE-SLEEP-TIMER", "DDS-TRANSFORMATION-DESCRIPTION", "CONTAINER-I-PDU", "MODE-IN-BSW-INSTANCE-REF", "NM-HW-VOTE-ENABLED", "CALIBRATION-PARAMETER-VALUE-SET", "DATE", "ICMP-RULE", "DESTINATION-IPV-6-ADDRESS", "SCALING-INFO-SIZE", "DIAGNOSTIC-SOVD-LOG", "REQUESTED-MODE-IREF", "ARRAY-SIZE-SEMANTICS", "ECUC-QUERYS", "NM-PDU", "REQUEST-RESULT", "PREPARE-UPDATE", "EXCESS-BURST-SIZE", "UDP-NM-CLUSTER", "UNICAST-SECURE-COM-PROPS-REF", "IPV-6-MULTICAST-IP-ADDRESS", "RESTORE-AT-START", "TARGET-R-PORT-PROTOTYPE-REF", "SERVICE-INSTANCE-ID", "IS-MANDATORY", "CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "COM-MANAGEMENT-MAPPINGS", "SYSTEM-REF", "PROCESS-PHM-ACTION-ITEM", "PORT-ARG-VALUES", "PROCESS-MODE-IREF", "TD-EVENT-SLLET-PORT", "PERSISTENCY-KEY-VALUE-STORAGE", "ST-MIN", "FLOW-CONTROL-PDU-REFS", "P-2-MAX", "WAKEUP-FRAME-REF", "CRYPTO-ALGORITHM-ID", "SW-DIAGNOSTICS-NOTESS", "EXTERNAL-TRIGGERING-POINTS", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "CP-SOFTWARE-CLUSTER", "AVERAGE-MEMORY-CONSUMPTION", "APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-VALUE", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF", "MODE-RECEIVER-POLICYS", "PREDECESSOR-REFS", "DIAGNOSTIC-SERVICE-INSTANCE-REF", "LOWER-BOUNDARY", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "SYMBOL", "COMPONENT-REF", "NETWORK-HANDLE-PORT-MAPPING", "ATTRIBUTE-DEFS", "SWITCH-FLOW-METERING-ENTRY", "TRANSPORT-PLUGIN", "NETWORK-MANAGEMENT-PORT-INTERFACE", "SECURITY-EVENT-REPORT-INSTANCE-DEFINITION", "RVF-PIXEL-FORMAT", "FIREWALL-STATE-MODE-DECLARATION-REF", "NAME-TOKEN-VALUE-VARIATION-POINT", "ECU-SCOPE-REFS", "WRITER-ENTITY-ID", "FIRST-TO-SECOND-DATA-TRANSFORMATION-REF", "GLOBAL-TIME-CORRECTION-PROPS", "TIME-TO-LIVE", "HARDWARE-ELEMENT-REFS", "SIGNALS", "FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING", "LOG-TRACE-DEFAULT-LOG-LEVEL", "BUS-OFF-RECOVERY", "SDG", "USED-DATA-ELEMENT", "ENTITY-STATUS-MAX-BYTE-FIELD-USE", "SW-SYSTEMCONSTANT-VALUE-SET", "DETERMINISTIC-CLIENT-REF", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "MAX-REQUEST-BYTES", "ELLIPTIC-CURVE-REF", "POWER-DOWN-TIME", "FUNCTION-POINTER-SIGNATURE-REF", "MC-DATA-ASSIGNMENTS", "TD-EVENT-VFB-REFERENCE", "BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR", "NM-MSG-CYCLE-TIME", "APMC-IP-V6-ADDRESS-PARAM-DEF", "SUPPORT-INFO", "ROLE-BASED-MC-DATA-ASSIGNMENT", "SHORT-LABEL", "TCP-PORT", "J-1939-NM-ECU", "FLEXRAY-FIFOS", "DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL", "RECOVERY-NOTIFICATION-REF", "DELIVERY-ARTIFACTS", "SOVD-VALIDATION-ORDER-REFS", "SA-REKEY-TIME", "MAX-PWM-S", "IDSM-TRAFFIC-LIMITATION-REF", "UNLIMITED-INTEGER-VALUE-VARIATION-POINT", "CRYPTO-KEY-SLOT-USER-DESIGN-REF", "URI-DESCRIPTION", "FORBIDDEN-SIGNAL-PATH", "MAX-RPT-EVENT-ID", "DLT-LOG-CHANNEL", "MODE-DRIVEN-FALSE-CONDITIONS", "DLT-USER-NEEDS", "SAFETY-MARGIN", "BSW-MODULE-ENTITY-REF", "REQUIRED-FIBEX-ELEMENT-REF", "RECOVERY-NOTIFICATION-RETRY", "MODE-REF", "PNC-SYNCHRONOUS-WAKEUP", "KEY-SLOT-USED-FOR-SYNC", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING", "SYSTEM-SIGNAL", "SENDER-REC-ARRAY-ELEMENT-MAPPING", "DERIVATION", "REST-ENDPOINT-POST", "SERVICE-POINT-SYMBOL-POST", "MC-DATA-INSTANCE-VAR-REF", "SLOT-TYPE", "DYNAMIC-LENGTH", "ECU-TASK-PROXY-REFS", "ECUC-LINKER-SYMBOL-DEF-CONDITIONAL", "PERSISTENCY-FILE-ELEMENT", "MAX-DATA-LENGTH", "SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING", "NM-READY-SLEEP-TIME", "FM-FEATURE-SELECTION", "SHORT-NAME-FRAGMENTS", "HW-CATEGORY-REF", "MODE-REPORTER-PORT-IREF", "ALIAS-EVENT-REF", "NM-CHANNEL-ACTIVE", "VALID-BINDING-TIME", "GLOBAL-TIME-DOMAIN-PROPS", "FIELD-REFS", "ERROR-REACTION-POLICY", "NM-DETECTION-LOCK", "CRYPTO-ALG-ID", "I-PDU-TRIGGERING-REF-CONDITIONAL", "TRIGGER-PERIOD", "VERIFY-IREF", "OBJECT-DEFINTION-REFS", "MANUFACTURER-VALIDATION-ORDER-REFS", "POSITION-IN-TABLE", "RESPONSE-ARGUMENT-REF", "BSW-ENTRY-RELATIONSHIP", "GLOBAL-RESOURCE-ID", "TIMING-CLOCKS", "TIMESTAMP-SUPPORT", "LOAD-BALANCING-PRIORITY", "SOURCE-LIN-PID-REF", "WAKE-UP-BY-CONTROLLER-SUPPORTED", "DDS-CP-CONSUMED-SERVICE-INSTANCE", "AAF-NOMINAL-RATE", "WRITING-PRIORITY", "SDXF", "SECURITY-EVENT-CONTEXT-DATA-ELEMENT", "STATE-MANAGEMENT-ACTION-LIST", "CONTAINER-TIMEOUT", "BLUEPRINT-POLICY-LIST", "LOGICAL-EXPRESSION-ARGUMENT-REFS", "VLAN-IDENTIFIER", "BSW-INTERRUPT-EVENT", "IPV-4-ADDRESS-SOURCE", "SENDER-REC-ARRAY-TYPE-MAPPING", "CM-MODULE-INSTANTIATION", "DDS-CP-TOPIC", "DIAG-FAULT-DETECTION-COUNTER-PORT-REF", "KEY-STORAGE-TYPE", "DIAGNOSTIC-SECURITY-LEVEL", "REST-BOOLEAN-PROPERTY-DEF", "FILE-INFO-COMMENT", "BSW-MODULE-INSTANCE-REF", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS", "ECUC-PARAM-CONF-CONTAINER-DEF", "MODE-ELEMENT-REF", "EVENT-MULTICAST-ADDRESSS", "BSW-BACKGROUND-EVENT", "TCP-IP-AUTO-IP-INIT-TIMEOUT", "ROOT-DATA-PROTOTYPE-REF", "ECUC-STRING-PARAM-DEF-CONDITIONAL", "SERVICE-INSTANCE-PORT-CONFIG", "HAS-SETTER", "REQUEST-DOWNLOAD-CLASS-REF", "L-GRAPHIC", "USED-IMPOSITION-TIME-DEFINITION-GROUP-REF", "GLOBAL-TIME-FR-MASTER", "ECUC-CHOICE-REFERENCE-DEF", "VERIFICATION-STATUS-INDICATION-MODE", "DEFAULT-PERIOD-BEGIN", "TRANSLATED-VLAN-ID", "SOURCE-NETWORK-MASK", "SDU-REF", "UDP-COLLECTION-TRIGGER", "COM-HANDLER-TASK-MAPPING-ENABLED", "MAX-SLOTS-PER-CYCLE", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF", "TRIGGER-I-PDU-SEND-CONDITIONS", "REQUEST-GROUP-REF", "GLOBAL-TIME-DOMAINS", "NAMESPACES", "SAVE-CONFIGURATION", "PRM", "HEADER-TYPE", "PERSISTENCY-FILE-PROXY-INTERFACE", "DETERMINISTIC-SYNC-M-OUT-OF-N", "RPT-SERVICE-POINT-POST-REFS", "VLAN-REFS", "ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL", "FUNCTION-NAME", "TD-EVENT-OPERATION-TYPE", "IAM-MODULE-INSTANTIATION", "PROCESSING-STYLE", "BOR-TIME-TX-ENSURED", "I-SIGNAL", "UCM-MODULE-INSTANTIATION", "MODE-SWITCH-POINTS", "DELEGATED-PORT-ANNOTATION", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "MODE-SENDER-POLICYS", "ARRAY-ELEMENT-REF", "POSSIBLE-AP-ERROR-REF", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "SIGNATURE-SCHEME-REFS", "URI", "IMPORTANCE", "SOCKET-PROTOCOL", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS", "COUPLING-PORT-FIFO", "IO-HW-ABSTRACTION-SERVER-ANNOTATIONS", "RAW-DATA-STREAM-CLIENT-INTERFACE", "TIMING-EVENT-REF", "J-1939-NM-NODE", "SW-GENERIC-AXIS-PARAMS", "DATA-LENGTH-REQUEST", "REST-RESOURCE-DEF", "DOCUMENTATION-CONTEXT", "DIAGNOSTIC-FIM-ALIAS-EVENT", "DO-IP-REQUEST-CONFIGURATION-DESIGN", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "J-1939-NM-CLUSTER", "OCCURRENCE-EXPRESSION", "PER-INSTANCE-MEMORY-REF", "WORST-CASE-STACK-USAGE", "DO-IP-TESTER-ROUTING-ACTIVATION-REFS", "BSW-MODE-MANAGER-ERROR-EVENT", "TIME-SYNC-CORRECTION", "SEQUENCE-COUNTER", "WAKEUP-TX-IDLE", "COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE", "RUNTIME-IP-ADDRESS-CONFIGURATION", "LATENCY-CONSTRAINT-TYPE", "FUNCTIONAL-CLUSTER-DESIGNS", "CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING", "FUNCTION-ID", "DISABLE-END-TO-END-STATE-MACHINE", "SERIALIZATION-TECHNOLOGY-REF", "ETHERNET-CLUSTER-REF", "COUPLING-PORT-STRUCTURAL-ELEMENTS", "IEEE-1722-TP-CONFIG", "SW-HOST-VARIABLE", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "LOCAL-SUPERVISION", "SOFTWARE-PACKAGE", "IMPLIED-OPERATION-REFS", "SW-CLUSTERS", "SERVICE-ITEMS", "ASSIGNED-CONTROLLER-REF", "HW-ATTRIBUTE-LITERALS", "ACL-OBJECT-CLASSS", "J-1939-TP-PG", "EXPIRED-SUPERVISION-CYCLE-TOL", "SD-CLIENT-CONFIG", "FILES", "SUBSTITUTED-FRAME-REFS", "COMMON-PROPERTIES", "NETWORK-INTERFACE-REF", "ECU-INSTANCES", "SAVE-CONFIGURATION-ENTRY", "DO-IP-INSTANTIATION", "PRIMARY-DOWNLOAD-URI", "HW-COMMUNICATION-CONTROLLER-REF", "COMPU-SCALES", "CPP-TEMPLATE-ARGUMENT", "FIRST-DATA-PROTOTYPE-REF", "ACCURACY-INT", "JOB-REQUIREMENTS", "DO-IP-NETWORK-CONFIGURATION", "USER-DEFINED-I-PDU", "IPV-4-MULTICAST-IP-ADDRESS", "IO-HW-ABSTRACTION-SERVER-ANNOTATION", "STORING", "DATA-LENGTH-RESPONSE", "TRACE-REF", "NM-NODES", "REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF", "TRACE-SWITCH", "CRYPTO-SERVICE-JOB-NEEDS", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "TIMING-VARIABLE-REF", "DDS-FIELD-QOS-PROPS", "I-PV-6-EXT-HEADER-FILTER-SET", "TIME-BASE-KIND", "CONTAINED-PROCESS-REF", "DIAGNOSTIC-EVENT", "MAX-FR-IF", "TRANSMIT-PDU-REF", "L-1", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "PRESTORAGE-FREEZE-FRAME", "PRE-ACTIVATE-IREFS", "FRAME-PORT-REFS", "TAG-WITH-OPTIONAL-VALUE", "PORT-BLUEPRINT-MAPPING", "SYSC-STRING-REF", "USE-AUTO-VALIDATION-AT-SHUT-DOWN", "APPLICATION-INTERFACE", "BSW-CLIENT-POLICY", "NM-CAR-WAKE-UP-FILTER-NODE-ID", "APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF", "ECU-CONFIGURATION-REF", "REQUEST-TYPE", "DLT-ARGUMENT-ENTRYS", "FM-FEATURE-RESTRICTION", "TIMING-CONDITION-FORMULA", "REMOTE-SUBJECT-REF", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "RESUME-POSITION", "I-PDU-TRIGGERING-REF", "DIAGNOSTIC-PARAMETER-ELEMENT", "AES-3-DATA-TYPE-L", "TIME-EXP", "EXCLUSIVE-AREA", "ATTRIBUTE-VALUES", "E-2-E-PROFILE-CONFIGURATION-SET", "AP-APPLICATION-ERROR-REFS", "WINDOW-SIZE-INIT", "TTCAN-CLUSTER-CONDITIONAL", "L-10", "API-PRINCIPLE", "LABELED-LIST", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "SECURITY-EVENT-REPORT-INTERFACE", "OPTION-ARGUMENT", "ALIAS-NAME-ASSIGNMENT", "OWNER-REF", "SERVICE-INTERFACE-REFS", "PORT-GROUP", "VEHICLE-ANNOUNCEMENT-INTERVAL", "DOC-REVISION", "EXPECTED-ALIVE-INDICATIONS", "METHOD-REPLY-TOPIC-NAME", "NM-CHANNEL-SLEEP-MASTER", "EXCLUSIVE-AREA-REFS", "RECOVERABLE-IN-SAME-OPERATION-CYCLE", "IMPLICIT-DATA-ACCESS-IREF", "START-ACTION-REFS", "BSW-IMPLEMENTATION", "SW-RECORD-LAYOUT", "VFC-IREF", "LIN-MASTER", "FM-FEATURE-MAP-CONDITION", "DATA-PROTOTYPE-GROUP", "MAXIMUM-KEY-LENGTH", "MANUFACTURER-VALIDATION-ORDER-REF", "GLOBAL-KAY-PROPS-REF", "DO-IP-ROUTING-ACTIVATIONS", "AR-PARAMETER", "ENTRY-TYPE", "REQUIREMENT-REFS", "WAKEUP-SYMBOL-RX-WINDOW", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "MIN-SYNC-JUMP-WIDTH", "SERVICE-INTERFACE-ELEMENT-MAPPING-REF", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "PATH-MTU-DISCOVERY-ENABLED", "CONTEXT-COMPONENT-REFS", "CONTEXT-ROOT-SW-COMPOSITION-REF", "NM-MAIN-FUNCTION-ACROSS-FR-CYCLE", "GID", "WAKEUP-FORWARD-REMOTE-ENABLED", "TAILORING-REF", "DDS-EVENT-QOS-PROPS", "NETWORK-HANDLE-REF", "USER-DEFINED-CLUSTER-CONDITIONAL", "TRANSPORT-LAYER-RULE", "DRIVER-PRIMITIVE-REF", "BASE-TYPE-SIZE", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "IPV-4-ADDRESS-MASK", "CYCLE", "FIELD-NOTIFIER-REF", "GLOBAL-TIME-DOMAIN-REF-CONDITIONAL", "APPLIED-STANDARDS", "SERIALIZER", "PRIMITIVE-TYPE-MAPPINGS", "WAKEUP-LOCAL-ENABLED", "MAPPING-DIRECTION", "INTERRUPT-CATEGORY", "IPV-6-CONFIGURATION", "SENDER-IREF", "ERROR-TRACER-NEEDS", "SW-VARIABLE-PROTOTYPE-REF", "SOURCE-METHOD-REF", "DISABLE-END-TO-END-CHECK", "UNIT-GROUP-REF", "FOLLOW-UP-ACTION-REFS", "DLT-MESSAGE-COLLECTION-SET", "INIT-VALUE", "NM-NODE-ID", "IDSM-CONTEXT-PROVIDER-INTERFACE", "USES-DYNAMIC-ADDRESSING", "ALLOW-UNAUTHENTICATED-PARTICIPANTS", "PARAMETER-ACCESSED-FOR-DEBUG-REF", "INTERPOLATION-ROUTINE", "EVENT-GROUP-ID", "DESTINATION-PHYSICAL-ADDRESS", "FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL", "PRECONFIGURED-CONFIGURATION-REF", "EXPECTED-ALIVE-CYCLE", "SYSTEM-COM-SPEC-DEFINITION-REF", "MIN-HEADER-ID", "TIMEOUT-AR", "TIME-BASE-RESOURCE-REF-CONDITIONAL", "CRYPTO-KEY-SLOT-USER-DESIGN-MAPPING", "FUNCTION-CLUSTER-AFFILIATION", "DATA-PROTOTYPE-GROUP-IREFS", "INTERPOLATION-ROUTINE-MAPPING", "ENGINEERING-OBJECT", "RESPONSE-GROUP-REF", "TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED", "DDS-RULE", "RELIABILITY", "USER-DEFINED-GLOBAL-TIME-MASTER", "DURABILITY-SERVICE-MAX-INSTANCES", "NETWORK-TIME-PROVIDER-REF", "MAX-NUMBER-FREEZE-FRAME-RECORDS", "ACL-OBJECT-REFS", "HW-ATTRIBUTE-DEF-REF", "NETWORK-REPRESENTATION", "SCOPE", "USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS", "TEXT", "KEY-SLOT-USED-FOR-START-UP", "RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS", "MAX-PWM-L", "NV-DATA-INTERFACE", "PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "VALUE-CONFIG-CLASSES", "SOMEIP-METHOD-DEPLOYMENT", "TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED", "INTERFACE-VERSION", "DDS-FIELD-REQUEST-TOPIC-REF", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "MINIMUM-SELECTED-BINDING-TIME", "GLOBAL-SUPERVISION-ENTITYS", "TLV-SUB-ELEMENT-REF", "STREAM-IDENTIFIER", "DHCP-PROPS", "TARGET-REFS", "TD-PDU-TRIGGERING-FILTER-REF", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "COMPONENT-SEPARATION", "HEADER-ID-SHORT-HEADER", "DIAGNOSTIC-EDR-SENDER-PORT-MAPPING", "APMC-MAC-ADDRESS-PARAM-DEF", "UDS-DTC-NUMBER", "NM-VOTING-CYCLE", "SYMBOL-WINDOW-ACTION-POINT-OFFSET", "SEED-SIZE", "DIAGNOSTIC-ROUTINE-NEEDS", "RUNNABLE-ENTITY-GROUP-IREFS", "APMC-TEXTUAL-PARAM-VALUE", "MODE-SWITCH-SENDER-COM-SPEC", "BASE-TYPE-REF", "ACK-TYPE", "DPG-DOES-NOT-REQUIRE-COHERENCYS", "AP-APPLICATION-ERROR-REF", "DEBUG-INFOS", "PROVIDER-SOFTWARE-CLUSTER-REF", "MAXIMUM-DURATION-OF-CAMPAIGN", "SWC-FLAT-SERVICE-DEPENDENCY-REF", "TP-ADDRESS-REFS", "UPPER-BOUND", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "NEIGHBOR-CACHE-SIZE", "TCP-IP-PROPS-REF", "SECURED-AREA-LENGTH", "INNER-PORT-IREF", "TRACE-SWITCH-CONFIGURATIONS", "SENDER-CAPABILITY", "COM-FIELD-GRANT-DESIGN", "QUEUED-RECEIVER-COM-SPEC", "DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL", "STATIC-MEMORYS", "HTTP-ACCEPT-ENCODING", "SOVD-DATA-CATEGORY-REF", "IEEE-1722-TP", "SEARCH-BEHAVIOR", "DLT-ARGUMENTS", "DO-NOT-FRAGMENT", "DYNAMICALLY-ASSIGNED", "MSR-QUERY-CHAPTER", "ETHERNET-PRIORITY-REGENERATION", "END-TO-END-CALL-RESPONSE-TIMEOUT", "VEHICLE-ANNOUNCEMENT-COUNT", "TIMING-DESCRIPTION-REF", "CAN-ENTER-EXCLUSIVE-AREA-REF", "WRITTEN-READ-NV-DATA", "DESTINATION-PORTS", "OPERATION-REF", "MAC-SEC-KAY-CONFIG", "INIT-VALUES", "NM-PASSIVE-MODE-ENABLED", "EXECUTABLE-REFS", "SERVICE-INTERFACE-FIELD-MAPPING", "BLOCK-STATE", "RECEIVED-DATA-REF", "SOFTWARE-CONTEXT", "DLT-ARGUMENT-PROPS-SET", "NOTIFICATION-STATE", "APMC-PARAM-CONF-CONTAINER-DEF", "SUBSCRIBE-EVENTGROUP-RETRY-MAX", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "ACCURACY", "OPERATION-PROTOTYPE-IREF", "MACHINE", "CLAIMED-FUNCTION-GROUP-REFS", "MEMORY-HIGH-ADDRESS", "CONTEXT-OPERATION-REF", "MEASURED-EXECUTION-TIME", "TCP-IP-PROPS-REFS", "PDU-MAX-LENGTH", "SOMEIP-EVENT-PROPS", "SEARCH-INTENTION", "PROCESSING-UNIT-REF", "FM-FEATURE-SELECTION-SET", "SYNC-CONFIRMATION-TIMEOUT", "INSTANCE-IDS", "MAXIMUM-EXECUTION-TIME", "ALLOWED-TCP-OPTIONS-REF", "OPERATION-REQUEST-TRIGGERING-REF", "COUPLING-PORT-DETAILS", "REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF", "RPT-EXECUTABLE-ENTITY-EVENT", "HEADER-ID", "I-SIGNAL-PORT-TO-DIAGNOSTIC-EVENT-MAPPING", "FLOW-CONTROL-PDU-REF", "TRIGGERS", "SOFTWARE-CLUSTER-ID", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "PROVIEDE-DATA-REF", "INNER-GROUP-IREFS", "ACTION", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-PORT-MAPPING", "MAX-BYTES-IN-INTERVAL", "WRITTEN-NV-DATA", "ALIGNMENT", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "LOCAL-UNICAST-ADDRESSES", "TIMEOUT-CHECK", "ECUC-MULTIPLICITY-CONFIGURATION-CLASS", "MAX-SIZE-TO-FILL", "POSTS", "TCP-TP", "REMOTE-SUBJECT-REFS", "STD", "SOCKET-CONNECTION-BUNDLE-REF", "EXTERN-OFFSET-CORRECTION", "SAFETY-POLICY", "PARAMETER-POLICYS", "MAX-SOURCE-PORT-NUMBER", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "INNER-GROUP-IREF", "REGULAR-EXPRESSION", "BINARY-MANIFEST-META-DATA-FIELD", "ENTER-SUSPEND-TO-RAM-ERROR-REF", "SWITCH-STREAM-FILTER-ENTRY", "DIAGNOSTIC-TROUBLE-CODE-REF", "COM-TIMEOUT-ERROR-REF", "SERVICE-INSTANCES", "DIAGNOSTIC-ENV-DATA-CONDITION", "DATA-CONSTR-REF", "SHORT-NAME-FRAGMENT", "CALIBRATION-PARAMETER-VALUE-SET-REF", "REST-ENDPOINT-ARGUMENT", "USER-DEFINED-COMMUNICATION-CONTROLLER", "KEYWORDS", "CONTEXT-DATA-SOURCE", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST", "VALUE-SPEC", "TEMPLATE-TYPE-REF", "IN-SCOPE", "GENERAL-PURPOSE-PDU", "INDICATOR-STATUS-NEEDS", "STATUS-ID", "TARGET-MODE-GROUP-REF", "LOGIC-ADDRESS", "ALLOW-DYNAMIC-L-SDU-LENGTH", "SOMEIP-METHOD", "SETTER-RETURN-REF", "LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS", "LOCAL-DEBUG-DATAS", "UDP-CHECKSUM-HANDLING", "LIN-PID-TO-CAN-ID-MAPPINGS", "SECURE-PROPS-FOR-TCP-REF", "CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING", "TRAFFIC-SHAPER-GROUPS", "STATE-MACHINE", "RATE-CORRECTION-MEASUREMENT-DURATION", "DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL", "DIAGNOSTIC-TEST-RESULT-REFS", "ALL-PHYSICAL-CHANNELS", "SUSPEND-TO-RAM-HUB-MAPPING", "AUTHENTICATION-KEY-REF", "PERIOD-BEGIN", "VALUE-REF", "BSW-DEBUG-INFO", "ECUC-STRING-PARAM-DEF", "CLIENT-COM-SPEC", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "DO-IP-LOGIC-ADDRESS", "CONSTANT-SPECIFICATION", "UCM-MASTER-MODULE-INSTANTIATION", "TROUBLE-CODE-J-1939-REF", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "INPUT-DATAS", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "TYPE", "REMOTE-IP-ADDRESS-REF", "PWM-O", "TLV-IMPL-RECORD-ELEMENT", "KEEP-ALIVE-TIME", "IDSM-INSTANCES", "VARIABLE-ACCESSED-FOR-DEBUG-REFS", "DNS-SERVER-ADDRESSES", "ETHERNET-COMMUNICATION-CONTROLLER", "SDG-CLASS", "TP-CONNECTION-CONTROL-REF", "SW-SYSTEMCONST", "RAM-BLOCK-STATUS-CONTROL", "AGE-CONSTRAINT", "MIN-PWM-L", "ECU-PARTITION", "CRYPTO-JOB", "MAX-NUMBER-OF-FILES", "EXCESS-INFORMATION-RATE", "SD-SERVER-TIMER-CONFIGS", "I-SIGNAL-TO-PDU-MAPPINGS", "PLATFORM-ACTION-ITEM", "STATE-DEPENDENT-FIREWALL-REF", "DIAGNOSTIC-DATA-CHANGE-TRIGGER", "TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED", "SECURED-AREA-OFFSET", "CONTEXT-COMPOSITION-REF", "PREDEFINED-VARIANT", "STATE-DEPENDENT-STARTUP-CONFIG", "STARTUP-CONFIG", "PROCESS-ARGUMENT", "DATA-REF", "DDS-TOPICS", "CLIENT-SERVER-RECORD-TYPE-MAPPING", "IDS-COMMON-ELEMENT-REF-CONDITIONAL", "TLS-CRYPTO-SERVICE-MAPPING", "FUNCTION", "NM-PNC-PARTICIPATION", "VARIABLE-DATA-PROTOTYPE-IREFS", "TO-REF", "PDU-ACTIVATION-ROUTING-GROUPS", "MODE-PORT-ANNOTATION", "PARAMETER-PORT-ANNOTATION", "EXTERNAL-AUTHENTICATIONS", "DIAGNOSTIC-IUMPR", "SECTION-NAME-PREFIX", "DROP-NOT-REQUESTED-NAD", "NETWORK-HANDLES", "ACTIONS", "FIRST-ELEMENT-REF", "AUTHENTICATION-CLASS-REF", "BSW-ENTRY-RELATIONSHIP-TYPE", "RUNNABLE-REF", "LIN-FRAME-TRIGGERING", "CLUSTER-DRIFT-DAMPING", "MAX-HEADER-ID", "COUNTER-PASSED-THRESHOLD", "KEY-SLOT-USAGE", "NM-INSTANTIATION", "ARTIFACT-LOCATORS", "RECOVERY-NOTIFICATION-REFS", "DDS-EVENT-TOPIC-REF", "INSTANTIATION-TIMING-EVENT-PROPS", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "FIREWALL-RULE-PROPSS", "BSW-MODULE-ENTRY-REF-CONDITIONAL", "IMPLEMENTATION-DATA-TYPE", "COMMUNICATION-CONNECTOR-REFS", "STATE-REQUEST-PORT-IREF", "FUNCTIONAL-UNIT", "PROVIDED-REQUIRED-INTERFACE-TREF", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "RPT-EVENT-ID", "EXCLUSIVE-AREAS", "SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING", "TLV-ARGUMENT-REF", "STATUS-SUB-TLV", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "J-1939-DCM-I-PDU", "PRIMITIVE-ATTRIBUTE-TAILORING", "DOC-REVISIONS", "PROXIMITY-PROOF-REQUIRED", "CRYPTO-PROVIDER-REF", "TCP-TP-PORT", "DIAGNOSTIC-IO-CONTROL-NEEDS", "STATIC-SLOT-DURATION", "SW-CARB-DOCS", "RECEPTION-POLICYS", "SDG-DEF", "MAC-MULTICAST-GROUP", "SOVD-SNAPSHOT-CONTENT-REF", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "TLS-JOB-REQUIREMENT", "VARIABLE-ACCESS-REF", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "EXPLICIT-RPT-PROFILE-SELECTION-REF", "SETTER-REF", "START-POSITION", "IDS-DESIGN", "EVENT-CLEAR-ALLOWED", "IUMPR-GROUP", "METHOD-ID", "RECEIVE-WINDOW-MIN", "TIMING-GUARANTEES", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF", "IMPLEMENTATION-PROPSS", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "ETHERNET-RAW-DATA-STREAM-MAPPING", "ECU-INSTANCE-REF", "RESOURCE-GROUP", "TX-PDU-PRIORITY", "DEFAULT-GATEWAY", "TRANSPORT-PDU-REF", "EVENT-TRIGGERING-REF", "SECURITY-DELAY-TIME", "CONNECTOR-REFS", "SOURCE-IPV-4-ADDRESS", "ACCESS-PERMISSION-REF", "CAT-2-ISR", "PROTOCOL-NAME", "STATUS-BIT-STORAGE-TEST-FAILED", "PROVIDED-DDS-SERVICE-INSTANCE", "TARGET-PROVIDED-OPERATION-REF", "IMPLEMENTS-SOMEIP-STRING-HANDLING", "APMC-FOREIGN-REFERENCE-VALUE", "TARGET-METHOD-REF", "APPLICATION-SW-COMPONENT-TYPE", "ELEMENT-MAPPINGS", "DIAGNOSTIC-SESSION-CONTROL", "META-DATA-ITEM-TYPE", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "EXTENDED-DATA-RECORDS", "PERIOD-END", "DESTINATION-ORDER-KIND", "PDU-POOLS", "MC-FUNCTION-REFS", "TCP-IP-ARP-PACKET-QUEUE-ENABLED", "SIZE-OF-UNION-TYPE-SELECTOR-FIELD", "CONNECTIONS", "P-PORT-PROTOTYPE", "LOCAL-SUPERVISIONS", "RPT-HOOKS", "LOGICAL-SUPERVISIONS", "VTF", "CONTENT-TYPE", "POST-BUILD-VARIANT-CONDITIONS", "KEY-REF", "DEF-CALPRM-SET", "DDS-RPC-SERVICES", "SCOPE-REF", "DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT", "GENERAL-PURPOSE-I-PDU", "PERSISTENCY-DEPLOYMENT-REF", "FEATURE-REFS", "LC-OBJECT-REF", "DIAGNOSTIC-SOVD-CONFIG-CONTENT-MAPPING", "WAKEUP-REMOTE-ENABLED", "DIAGNOSTIC-AUTHENTICATION-CLASS", "SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY", "RX-TIMEOUT-TIME", "REG-DOES-NOT-REQUIRE-STABILITYS", "EXECUTABLE-REF", "BSW-ENTRY-RELATIONSHIPS", "EXECUTION-TIMES", "CHECKPOINTSS", "AUTHENTICATION-TIMEOUT", "DEPLOYMENT-REF", "RVF-EVENT-DEFAULT", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "SA-OVER-TIME", "SERVICE-IDENTIFIER", "ALLOW-PASSIVE-TO-ACTIVE", "PROPABILITY", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "OPERATIONS", "DEST-META-CLASS", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "UDP-PORT-REF", "SW-COMPONENT-MAPPING-CONSTRAINTS", "LINKER", "MODULE-INSTANTIATIONS", "REQUEST-EMISSION-RELATED-DTC-CLASS-REF", "SW-CLASS-INSTANCE", "VARIABLE-ACCESS", "SW-RECORD-LAYOUT-REF", "I-SIGNAL-I-PDU", "RULE-ARGUMENTS", "TRACEABLE-TABLE", "IKE-CIPHER-SUITE-NAME", "TRIGGER-REF", "EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS", "KEEP-ALIVE-INTERVAL", "FRAGMENTATION-PROPS", "VEHICLE-DRIVER-NOTIFICATION", "HW-ELEMENT", "VEHICLE-IDENTIFICATION-SYNC-STATUS", "REQUIRED-DESIGN-ELEMENT-REFS", "SUBFUNCTIONS", "COM-EVENT-GRANT-DESIGN", "BLUEPRINT-MAPS", "FIELDS", "FIRST-ELEMENT-REFS", "KEY-VALUE-STORAGE-REF", "DERIVED-PORT-PROTOTYPE-REF", "REQUESTER-SOFTWARE-CLUSTER-REF", "DIAGNOSTIC-UDS-SEVERITY-ENUM-VALUE-VARIATION-POINT", "SOFTWARE-CLUSTER", "PROCESSING-KIND", "GENERATOR", "ANNOTATION", "SOFTWARE-CLUSTER-DESIGN-REF", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "PDU-COLLECTION-SEMANTICS", "CORES", "PNC-WAKEUP-CAN-ID", "PAYLOAD-REF", "DIAGNOSTIC-EVENTS", "CRYPTO-KEY-SLOT-INTERFACE", "BUFFER-RATIO", "TLS-CONNECTION-GROUP", "DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT", "CYCLE-COUNT-MAX", "BSW-MODULE-CLIENT-SERVER-ENTRY", "INTERNAL-BEHAVIORS", "CRYPTO-KEY-SLOT-DESIGN", "COMM-CONTROLLER-REF", "ALLOWED-SERVICE-PROVIDERS", "TCP-FIN-WAIT-2-TIMEOUT", "DIAGNOSTIC-SOVD-RECORD-CONTENT-ELEMENT", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "TIME-SYNC-PORT-PROTOTYPE-IREF", "CRYPTO-MODULE-INSTANTIATION", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "SUB-GROUP-REF", "DIAGNOSTIC-CONTRIBUTION-REF", "CONSIDER-PTO-STATUS", "CAN-ENTERS", "REMOTING-TECHNOLOGY", "TLS-CONNECTION-REFS", "TRANSFORMING-SYSTEM-SIGNAL-REF", "CRYPTO-SERVICE-CERTIFICATE-REF", "SYNC-COUNTER-INIT", "PNC-WAKEUP-CAN-ID-EXTENDED", "FIRST-TIMEOUT", "SERVER-COM-SPEC", "TCP-PROPS", "UNIQUE-STREAM-ID", "DDS-RPC-SERVICE-REF", "INVALID-VALUE", "PRODUCT-ID", "XREF", "CUSTOM-SPECIFICATION-REFS", "COM-MGR-USER-NEEDS", "BULK-NV-BLOCK", "SECURITY-EVENT-CONTEXT-PROPS", "TIMING-DESCRIPTION-EVENT-CHAIN", "COUPLING-PORT-SCHEDULER", "COMPARE-VALUE", "REQUIRES-SYMBOLIC-NAME-VALUE", "SUPPORTED-FEATURES", "POSSIBLE-ERROR-REFS", "DIAGNOSTIC-OPERATION-CYCLE", "FILE-PROXYS", "FRESHNESS-VALUE-LENGTH", "LOG-AND-TRACE-INTERFACE", "TRANSFERS", "END-TO-END-TRANSFORMATION-COM-SPEC-PROPS", "DIAGNOSTIC-SECURE-CODING-MAPPING", "CLIENT-INTENT", "SDF", "CHECKPOINT-TRANSITION-REF", "E-2-E-METHOD-PROTECTION-PROPSS", "INTERRUPT-SOURCE", "NOTE", "MAX-AS", "DELAY", "AUTH-INFO-TX-LENGTH", "PROVIDED-OPERATIONS", "DIAGNOSTIC-TEST-RESULT-REF", "BLOCK-IF-STATE-ACTIVE-CP-REF", "AUTH-DATA-FRESHNESS-LENGTH", "GLOBAL-TIME-DOMAIN-REF", "DATA-LENGTH", "TIMESTAMP-RX-ACCEPTANCE-WINDOW", "RUN-RESOURCE", "VLAN-REF", "SW-RECORD-LAYOUT-GROUP-TO", "PAYLOAD-BYTE-PATTERN-RULE", "COLLECTION-TRIGGER", "OPTION", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE", "STATIC-REMOTE-UNICAST-ADDRESSES", "GLOBAL-TIME-ETH-SLAVE", "OFNS-DATA-ID-LISTS", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF", "SPN-REF", "PHYSICAL-DIMENSION-MAPPING", "REQUIRED-MODE-GROUPS", "GETTER-RETURN-SIGNAL-REF", "BSW-DISTINGUISHED-PARTITION", "NM-RANGE-CONFIG", "SIGNAL-I-PDU-COUNTER", "ANNOTATION-ORIGIN", "METHODS", "SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS", "CONTROLLED-DATA-ELEMENT-REFS", "TTL-MIN", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "TX-NM-PDU-REF", "TCP-IP-NDP-DYNAMIC-MTU-ENABLED", "EXPRESSION", "DIAGNOSTIC-ADDRESSS", "SYMBOLIC-NAME-VALUE", "MACROTICK-DURATION", "MINIMUM-START-INTERVAL", "INCLUDE-REFS", "EXT-HEADER-FILTER-LISTS", "SOMEIP-TP-CHANNEL", "SLAVES", "ARTIFACT-CHECKSUM-REFS", "MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF", "IS-EXTERNAL", "TLS-DEPLOYMENT", "DEF-ITEM", "SDG-CLASSES", "DIAGNOSTIC-VALUE-NEEDS", "ETH-TP-CONFIG", "SENDING-NODE-REFS", "TP-CONNECTION-REFS", "MAX-PROGRESSION-MISMATCH-THRESHOLD", "IMPLEMENTED-ENTRYS", "RELIABILITY-MAX-BLOCKING-TIME", "CYCLIC-HANDLING-COM-DATA-TO-OS-TASK-PROXY-MAPPING", "IIDC-DATA-BLOCK-SIZE", "DATA-RECEIVE-POINT-BY-VALUES", "META-DATA-ITEM", "EXTENDED-DATA-RECORD-ELEMENT-REF", "SOURCE-POSSIBLE-ERROR-REF", "TP-TECHNOLOGY", "COM-MANAGEMENT-PORT-GROUP-IREFS", "REPRESENTED-PORT-GROUP-REF", "APPLICATION-DEFERRED-DATA-TYPE", "APPLICATION-ACTION-ITEM", "RESPONSE-ON-EVENT-SCHEDULER-RATE", "DEPENDENT-SOFTWARE-CLUSTER-REF", "WAKEUP-TX-ACTIVE", "ON-TRANSITION-VALUE", "ELEMENT-ROLE", "FRESHNESS-COUNTER-SYNC-ATTEMPTS", "SERVER-COM-SPEC-PROPSS", "RESISTANT-TO-CHANGED-SW", "SW-SYSCOND", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "FAILED-REFERENCE-CYCLES-TOLERANCE", "NAGLES-ALGORITHM", "BITFIELD-TEXT-TABLE-MASK-SECOND", "REMOTE-PORT-RANGE-START", "E-2-E-PROFILE-CONFIGURATION-REF", "PNC-FILTER-ARRAY-MASKS", "ITEMS", "DATA-RECEIVE-POINT-BY-ARGUMENTS", "DEFERRING-FID-REF", "RESOURCE-NEEDS-REFS", "EID-RETRIEVAL", "FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING", "HOST-SYSTEM-REF", "SUPPORTS-MULTIPLE-NOTIFIER-SETS", "STRUCTURED-REQ", "IS-STRUCT-WITH-OPTIONAL-ELEMENT", "LEAVE-SUSPEND-TO-RAM-ERROR-REF", "J-1939-NODE-REF", "TIMEOUT-BS", "FIELD-NOTIFIER-QOS-PROPSS", "OFS-DATA-ID-LISTS", "COM-CONFIGURATION-TX-TIME-BASE", "SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE", "LIN-ORDERED-CONFIGURABLE-FRAME", "IS-DELTA-PACKAGE", "PORT-PROTOTYPE-SLUG-FRAGMENT", "RPT-SW-PROTOTYPING-ACCESS", "E-2-E-PROFILE-CONFIGURATIONS", "HEALING-CYCLE-COUNTER-THRESHOLD", "DEF", "FUNCTION-INSTANCE", "NUMBER-OF-CONNECTED-CLIENTS", "ACF-BUS-REFS", "IEEE-1722-TP-ACF-LIN-PART", "SUB-SDG-REF", "REQUIREMENT-REF", "MODE-DECLARATION-REF", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "APMC-URI-FOREIGN-REFERENCE-VALUE", "ECU-INSTANCE-REFS", "CLAIMED-FUNCTION-GROUP-REF", "SECURE-COMMUNICATION-DEPLOYMENTS", "TIMEOUT-SUBSTITUTION-VALUE", "SW-IMPL-MAPPINGS", "CONFIGURATION-HANDLING", "SUPERVISED-ENTITY-ID", "RECEPTION-PROPS", "CRYPTO-NEED-REF", "SECURITY-EVENT-CONTEXT-DATA-DEFINITION", "SOMEIP-TP-CONNECTION", "TTCAN-CLUSTER", "MODULE-INSTANTIATION-REF", "SUPERVISION-ENTITY", "CRYPTO-ALGORITHM-DESCRIPTION", "MAX-WITHOUT-CLOCK-CORRECTION-FATAL", "NODE-NAME", "CONTAINED-I-PDU-PROPS", "SYNC-JUMP-WIDTH", "ECU-TASK-PROXY-REF", "USAGE", "FIGURE", "IS-OPTIONAL", "UPPER-HEADER-BITS-TO-SHIFT", "VERIFY-IREFS", "PRIORITY", "DLT-ARGUMENT", "DIAGNOSTIC-SOVD-BULK-DATA", "VALUE-ACCESS", "FUNCTIONAL-CLUSTER-TO-DLT-LOG-SINK-MAPPING", "TARGET-EEC-REF", "CLIENT-ID-DEFINITION-SET-REFS", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "EVENT-HANDLER-REF", "CLASS-TAILORINGS", "VALUE-TYPE-TREF", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "OBD-MONITOR-SERVICE-NEEDS", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "TEST-IDENTIFIER", "CRYPTO-CERTIFICATE-INTERFACE", "NM-NETWORK-HANDLE", "CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE", "FIRST-MODE-REF", "METHOD-ACTIVATION-ROUTING-GROUPS", "ACCEPTED-REMOTE-CERTIFICATE-REFS", "ASSIGNED-TRAFFIC-CLASS", "CAPACITY", "CYCLIC-BACKUP-INTERVAL", "MCD-IDENTIFIER", "ETHERNET-NETWORK-CONFIGURATION", "BOOLEAN-VALUE-VARIATION-POINT", "ADDRESS-RANGE-LOWER-BOUND", "SWC-IMPLEMENTATION", "SECOND-ELEMENT-REF", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "STATE-CLIENT-INTERFACE", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "LENGTH-VERIFICATION", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "ECU-INSTANCE", "ELEMENT-REF", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "SAK-REKEY-TIME-SPAN", "SIGNAL-BASED-FIELD-REF", "VFC-IREFS", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF", "INFORMAL-FORMULA", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "DTC-FORMAT-TYPE", "COMMENT", "AR-TYPED-PER-INSTANCE-MEMORY-REF", "TIMING-CONDITION", "CHECKPOINT-FINAL-REFS", "BSW-INTERNAL-TRIGGERING-POINT-REF", "TX-PDU-POOL-REF", "BUFFER-PROPERTIES", "SECURE-COM-PROPS-FOR-DDS-REF", "WAIT-POINT", "SW-NUMBER-OF-AXIS-POINTS", "TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE", "REF", "DEBOUNCE-COUNTER-STORAGE", "TARGET-MODULE-ID", "BSW-MODE-GROUP-REF", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "HOLD-OVER-TIME", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "RESOURCE-GROUPS", "INCLUDED-LIBRARY-REFS", "MAX-AVAILABLE-PERSISTENCY-STORAGE-SPACE", "BSW-COMPOSITION-TIMING", "DEFAULT-LEASE-TIME", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "CONCRETE-PATTERN-EVENT-TRIGGERING", "COMPOSITION-DATA-PROTOTYPE-REF", "SW-COMPONENT-PROTOTYPE-REF", "HARDWARE-TEST-NEEDS", "PROVIDED-SERVICE-INSTANCE", "CALLED-ENTRYS", "NETWORK-CONFIGURATIONS", "CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING", "SWC-MAPPING-CONSTRAINT-REFS", "SAMPLE-CLOCK-PERIOD", "SW-COMPONENT-DOCUMENTATIONS", "ARGUMENTS", "POSSIBLE-AP-ERROR-SET-REFS", "PDU-TRIGGERING-REF-CONDITIONAL", "TCP-IP-NDP-MAX-RTR-SOLICITATIONS", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "DESTINATION-BASE-ID", "CAN-CONFIG", "AREA", "NEEDS-ORIGINAL-DATA", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "DEADLINE-MAX", "SW-TEST-DESC", "IDSM-INSTANCE-REF-CONDITIONAL", "MAX-BASE-TYPE-SIZE", "TCP-IP-DHCP-V-6-CNF-DELAY-MAX", "LUMINOUS-INTENSITY-EXP", "CAN-IDENTIFIER-RANGE", "STATE-MANAGEMENT-SLEEP-ACTION-ITEM", "AP-APPLICATION-ENDPOINT", "EVENT-CONTROLLED-TIMING", "SYNC-LOSS-THRESHOLD", "ROUTINE-INFO", "REF-MEASUREMENT-SET", "BSW-OS-TASK-EXECUTION-EVENT", "R-PORT-IN-COMPOSITION-INSTANCE-REF", "ENABLE-LIVELINESS-PROTECTION", "DEFAULT-PERIOD-END", "ECU-EXTRACT-VERSION", "SOURCE-MAC-ADDRESS-MASK", "PERIOD", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "QUEUED-RECEIVER-COM-SPEC-PROPSS", "LICENSE-REFS", "NM-NODE-REFS", "MODE-GROUP", "ACCEPTANCE-FIELD", "MAX-NUMBER-OF-ALLOWED-UPDATES", "RESOURCES", "SECURE-COM-PROPS-FOR-TCP-REFS", "CUSTOM-SPECIFICATION-REF", "DESTINATION-IPV-4-ADDRESS", "SEC-OC-CRYPTO-SERVICE-MAPPING", "POST-BUILD-VARIANT-CRITERION", "TRANSFORMATION-I-SIGNAL-PROPSS", "SPN", "META-DATA-ITEM-SETS", "ECUC-INTEGER-PARAM-DEF", "SOCKET-ADDRESS-REF", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS", "IMPL-CONSTANT-REF", "DISABLED-IN-MODE-IREF", "FLEXRAY-FRAME", "CLIENT-SERVER-ANNOTATION", "METHOD-REQUEST-PROPSS", "SD-SERVER-CONFIG-REF", "J-1939-CLUSTER-CONDITIONAL", "SOVD-PROPERTIES", "FROM-REF", "CONCRETE-CLASS-TAILORING", "MAX-SAMPLES", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-PORT-MAPPING", "TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS", "DLT-MESSAGE-REF-CONDITIONAL", "ECU-PARTITION-TO-CORE-MAPPINGS", "OPTIONS", "NV-DATAS", "RECEPTION-DEFAULT-VALUES", "CONSUMED-EVENT-GROUPS", "DATA-TYPE-POLICY", "PLCA-LOCAL-NODE-COUNT", "CLIENT-SERVER-ARRAY-TYPE-MAPPING", "APMC-FUNCTIONAL-CLUSTER-DEF", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "SD-SERVER-EVENT-CONFIG", "OPTIONAL", "CHANGE", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "DDS-SERVICE-VERSION", "READ-DATA-CLASS-REF", "DIAGNOSTIC-DATA-ELEMENT-REF", "LITERALS", "PATTERN-LENGTH", "FLEXRAY-FRAME-TRIGGERING", "NOTIFICATIONS", "TD-HEADER-ID-RANGE", "DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF", "SECOND-MODE-GROUP-REF", "CLEAR-EVENT-ALLOWED-BEHAVIOR", "START-INDEX", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "AUTHENTICATION-ROLE-REFS", "REVISION-LABEL-P-2", "METHOD-DEPLOYMENTS", "MAX-NUMBER-OF-OCCURRENCES", "START", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF", "FM-FEATURE-RELATION", "DATA-ELEMENT-REFS", "APPLIES-TO", "GATEWAYS", "DIAGNOSTIC-SOVD-CONTENT", "SOMEIP-TRANSFORMATION-DESCRIPTION", "TOL", "BULK-NV-DATA-DESCRIPTORS", "SECURITY-EVENT-DEFINITION", "REFERENCED-TD-EVENT-VFB-REF", "ASSERTIONS", "DEFINITION-REF", "SECURITY-LEVEL-REF", "ENTERED-MODE-REF", "END-TO-END-PROTECTION-I-SIGNAL-I-PDUS", "CONDITIONAL-CHANGE-NAD", "SYSTEM-MEMORY-USAGES", "ACF-TRANSPORTED-BUSS", "PRESTORED-FREEZEFRAME-STORED-IN-NVM", "FORMULA-CAPTION", "AUTO-REQUIRE", "TLV-RECORD-ELEMENT-REF", "DATA-TYPE-MAPPING-SET-REF", "EXECUTABLE-REQUIRED-PORT-IREF", "DELETES", "DURABILITY-SERVICE", "SEQUENTIAL-INSTRUCTIONS-END", "REQUIRED-CLIENT-SERVER-ENTRY-REF", "TCP-KEEP-ALIVE-ENABLED", "SWC-MODE-SWITCH-EVENT", "EVENT-OBD-READINESS-GROUP", "MEMORY-ENTRY-STORAGE-TRIGGER", "TRIGGER-INTERFACE", "TD-EVENT-SERVICE-INSTANCE-FIELD", "CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF", "DO-IP-INTERFACE", "MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS", "CRC-PRECISE-ORIGIN-TIMESTAMP", "RPT-READS", "VALIDATION-ROOT", "LOCAL-CERTIFICATE-REFS", "I-SIGNAL-IN-I-PDU-REF", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "BUILD-ACTION-MANIFEST", "ASYNCHRONOUS-TRAFFIC-SHAPER-REF", "TEXT-TABLE-MAPPINGS", "DEFAULT-LOG-THRESHOLD", "SUPERVISION-MODE-CONDITIONS", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS", "TRANSITION-REFS", "ROLE-BASED-DATA-ASSIGNMENT", "IS-DEFAULT", "AUDIENCE", "DEFAULT-PRIORITY", "CLIENT-SERVER-OPERATION-REF", "USE-FRESHNESS-TIMESTAMP", "DTC-NUMBER", "I-SIGNAL-TRIGGERING-REF", "DIAGNOSTIC-REQUEST-DOWNLOAD", "BIT-OFFSET", "BSW-RELEASED-TRIGGER-POLICY", "TRANSLATION-TARGET-IREF", "DIAGNOSTIC-OPERATION-CYCLE-REF", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "SHALL-RUN-ON-REF", "COMPONENTS", "IPV-6-ADDRESS-MASK", "IEEE-1722-TP-ACF-LIN", "IMPLEMENTATION-CONFIG-CLASSES", "E-2-E-EVENT-PROTECTION-PROPSS", "DLT-CONFIG", "TIMESTAMP-FORMAT", "DATA-RECEIVE-POINTS", "CHECKPOINT-REFS", "ENVIRONMENTAL-CONDITION-REF", "TLS-JOB-MAPPINGS", "R-PORT-PROTOTYPE-PROPS", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "UCM-PROCESSINGS", "PORT-ADDRESS", "MC-FUNCTION-DATA-REF-SET-CONDITIONAL", "APPLICATION-PARTITION-REF", "LOG-TRACE-FILE-PATH", "REQUIRED-TRIGGER-IREF", "APPLICATION-ERROR", "SHUTDOWN", "DIAGNOSTIC-RELEVANCE", "MIRRORING-PROTOCOL", "DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF", "SOVD-CONTENT-REF", "GLOBAL-TIME-SUB-DOMAINS", "AAF-AES-3-DATA-TYPE", "LIFE-CYCLE-INFO-SET", "MANUFACTURER-CODE", "CONTEXT-DATA-ELEMENT-IDENTIFICATIONS", "REDUNDANCY", "PER-INSTANCE-PARAMETERS", "DIAGNOSTIC-SESSION-REFS", "INCLUDED-VARIANT-REFS", "TARGET-STATUS-REF", "TYPE-TREF", "EXECUTION-CONTEXTS", "MODE-GROUP-IREF", "SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL", "TCP-REMOTE-PORT", "TARGET-PARAMETER-DATA-PROTOTYPE-REF", "NM-VOTE-INFORMATION", "DIAGNOSTIC-INFO-TYPE", "TARGET-REF", "VARIATION-POINT-PROXY", "CONSTANT-VALUE-MAPPING-REF", "DIAGNOSTIC-TROUBLE-CODE-OBD-PROPS", "PROVIDE", "TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED", "MAX-DURATION-EXCEEDED-ERROR-REF", "MAX-SAMPLE-POINT", "PORT-PROTOTYPE-PROPS", "SECURED-RX-VERIFICATION", "HW-PIN-REFS", "FRAME-PREEMPTION-SUPPORT", "SOFTWARE-COMPOSITION-TREF", "CALL-TIMEOUT", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "CONSTRAINT-REF", "SEPARATED-COMPONENT-IREF", "MAC-ADDRESS-MASK", "NM-SYNCHRONIZING-NETWORK", "J-1939-SHARED-ADDRESS-CLUSTER", "FM-FEATURE-MAP-ELEMENT", "ECU-INSTANCE-REF-CONDITIONAL", "PDU-COUNTER-START-POSITION", "FUNCTION-GROUP-REF", "SW-AXIS-INDIVIDUAL", "FEATURE-MODEL-REF", "NOTIFIER-DATA-ELEMENT-REF", "CONSTRAINT-TAILORING", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF", "P", "SW-AXIS-TYPE", "AUTOSAR-DATA-TYPE-REF", "RETRY", "APMC-FUNCTIONAL-CLUSTER-VALUE", "CONNECTION-TYPE", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NODE-MAPPINGS", "IDSM-RATE-LIMITATION-REF", "COUPLING-PORT-STARTUP-ACTIVE-TIME", "PORT-DEFINED-ARGUMENT-BLUEPRINTS", "AGING-CYCLES", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "SDX-REF", "TD-EVENT-BSW-MODE-DECLARATION", "PGN", "NM-IMMEDIATE-NM-CYCLE-TIME", "FIRST-APPLICATION-ERROR-REF", "TRIGGER-MODE", "BSW-INTERNAL-BEHAVIOR", "SOURCE-EEC-REF", "DIAGNOSTIC-EVENT-INFO-NEEDS", "KEY-SERVER-PRIORITY", "TARGET-DATA-PROTOTYPE-GROUP-REF", "DIAGNOSTIC-ECU-RESET-CLASS", "VARIABLE-DATA-PROTOTYPE-IREF", "SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION", "COMPOSITION-SW-COMPONENT-TYPE-REF", "AES-3-DATA-TYPE-H", "INITIAL-REF-OFFSET", "DO-IP-TARGET-ADDRESS-REF", "SIZE-OF-UNION-LENGTH-FIELDS", "RUNS-INSIDE-EXCLUSIVE-AREA-REF", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE", "SLEEP-TIME", "DEADLINE-PERIOD", "FOREIGN-MODEL-REFERENCE", "ALGORITHM-DESCRIPTIONS", "SYNC-DATA-ID-LISTS", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "LOCAL-PARAMETER-REF", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "CAN-TP-CHANNEL-REF", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS", "LISTEN-TIMEOUT", "MAPPING-REF", "WAKEUP-FRAME-REFS", "FIRST-ELEMENT-INSTANCE-IREF", "CALCULATION-FORMULA", "MINIMUM-INTERVAL-LENGTH", "BSW-PER-INSTANCE-MEMORY-POLICY", "REPORT-BEHAVIOR", "SOURCE-SIGNAL-REF", "CALIBRATION-PARAMETER-VALUE", "REG-REQUIRES-STABILITYS", "DESTINATION-CONTEXT", "HW-PIN-REF", "INDEX", "EGRESS-PORT-REFS", "DATA-TYPE-MAPPING", "ENABLE-DEACTIVATION", "TESTED-ITEM-REFS", "BURST-PATTERN-EVENT-TRIGGERING", "STREAM-FILTER-RULE", "REFERENCED-TD-EVENT-SWC-REF", "CONTEXT-BSW-IMPLEMENTATION-REF", "CAN-NM-NODE", "CRYPTO-PROVIDER-DOCUMENTATION-REF", "MC-SUPPORT", "ENCRYPTION-REF", "GLOBAL-TIME-PDU-TRIGGERING-REF", "TARGET-R-PORT-REF", "SW-VARIABLE-IMPLS", "CRYPTO-KEY-SLOT-USAGE-DESIGN", "RTP-TP", "SESSION-ID", "MIN-OK-STATE-INVALID", "TRIGGER-MAPPING", "ECUC-FUNCTION-NAME-DEF-VARIANTS", "CRYPTO-SERVICE-CERTIFICATE", "RUNTIME-PORT-CONFIGURATION", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "APP-TASK-PROXY-REF", "COM-OFFER-SERVICE-GRANT-DESIGN", "CONSISTENCY-NEEDS", "AFFECTED-BY-CONTROL-DTC-SETTING", "COMM-CONNECTORS", "NM-STATE-REQUEST", "WRITE", "CHANNELS", "TCP-NAGLE-ENABLED", "PARAMETER-SW-COMPONENT-TYPE", "PHM-ARBITRATION", "DIAGNOSTIC-COMMON-PROPS-CONDITIONAL", "CLIENT-ID-DEFINITION-SET-REF", "RPT-SERVICE-POINT-POST-REF", "VIOLATED-SAFETY-CONDITION-BEHAVIOR", "FUNCTION-GROUP-IREF", "HISTORY", "DESTINATION-URI-REFS", "PERSISTENCY-PORT-PROTOTYPE-IREF", "MATCHING-CRITERION-REF", "OBJECT-IREF", "GENERAL-PURPOSE-CONNECTION", "DATA-COM-PROPS", "OBD-PROPSS", "METHOD-QOS-PROPSS", "ROOT-SOFTWARE-COMPOSITIONS", "PORT-SCHEDULER", "IS-EVENT", "AUTH-ROLE-REFS", "HTTP-TP", "READ-NV-DATA", "MINIMUM-SUPPORTED-UCM-VERSION", "FUNCTION-GROUP-MODE-IREF", "ENHANCED-MODE-API", "SOFTWARE-PACKAGE-STORING", "DATA-IREF", "STATE-MANAGEMENT-ERROR-COMPARE-RULE", "CONSTANT-SPECIFICATION-MAPPING-SET", "DO-IP-RULE", "MINIMUM-MULTICAST-SEPERATION-TIME", "TCP-RULE", "ECUC-CHOICE-CONTAINER-DEF", "AFFECTED-REF", "METHOD-REFS", "SCOPE-EVENT-REF", "ADMIT-WITHOUT-MESSAGE-ID", "I-PDUS", "CAK-REF", "MESSAGE-TYPE-INFO", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "WATCHDOG-ACTION-ITEM", "MANAGED-PHYSICAL-CHANNEL-REFS", "PHYS-CONSTRS", "DATA-TYPE-MAP", "FUP-DATA-ID-LISTS", "SW-ADDRMETHOD-REF", "ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL", "DESTINATION-MIN-ADDRESS", "SHALL-NOT-RUN-ON-REFS", "SECURE-COM-CONFIGS", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "CYCLIC-HANDLING-COM-DATA-TO-OS-TASK-PROXY-MAPPINGS", "COMPU-NUMERATOR", "PARAMETER", "COUNTER-INCREMENT-STEP-SIZE", "SESSION-CONTROL-CLASS-REF", "COMPU-SCALE", "SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS", "CONTROLLED-HW-ELEMENT-REF", "ECU-INSTANCE-PROPSS", "STREAM-FILTERS", "MIN", "MULTICAST-SD-IP-ADDRESS-REF", "P-2-STAR-SERVER-MAX", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "BINARY-MANIFEST-ITEM", "DIAGNOSTIC-DATA-PORT-MAPPING", "NM-NETWORK-HANDLE-REF", "DIAGNOSTIC-SOVD-CONTENT-PORT-MAPPING", "REMAPPED-CAN-ID", "BUILD-ACTION-MANIFEST-REF", "DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE", "LOCAL-PORT-REF", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "NM-COORDINATOR-ROLE", "MAPPING-CONSTRAINTS", "TIME-BASE-RESOURCE-REFS", "RAW-DATA-STREAM-SERVER-INTERFACE", "SOCKET-ADDRESSS", "P-PORT-PROTOTYPE-IREF", "CHANNEL-NAME", "USED-CODE-GENERATOR", "PORT-INTERFACE-MAPPING-SET", "USER-DEFINED-FIELD-DEPLOYMENT", "M", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "TIME-PERIOD", "SW-ARRAYSIZE", "MIN-NUMBER-OF-OCCURRENCES", "NUMERICAL-VALUE-VARIATION-POINT", "SWC-MODE-SWITCH-EVENT-REF", "SERVICE-FIND-TIME-TO-LIVE", "PROVIDED-OUTER-PORT-REF", "BURST-SIZE", "CAN-FD-CONFIG", "FM-FEATURE-MODEL", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "FUNCTION-GROUPS", "TIMING-MODE-INSTANCE", "TLS-CIPHER-SUITES", "SUB-NODE-CHANNELS", "DO-IP-GID-NEEDS", "NON-VERBOSE-MODE", "CHARGE-MANAGER-NEEDS", "ORDERED-ELEMENTS", "MAXIMUM-DISTANCES", "PER-INSTANCE-MEMORY", "STORE-EVENT-SUPPORT", "CAN-FRAME-TX-BEHAVIOR", "SENDER-REC-RECORD-ELEMENT-MAPPING", "DATA-OFFSET", "MICRO-PER-CYCLE", "REPRESENTED-MODEL-ELEMENT-REF", "BSW-SERVICE-DEPENDENCY", "TARGET-MODE-REF", "DDS-CP-CONFIG", "SYNCHRONIZED-SLAVE-TIME-BASE", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "PNC-GATEWAY-TYPE", "TARGET-DATA-PROTOTYPE-IN-CS-REF", "DO-IP-DESIGN-REF", "DIAGNOSTIC-VALUE-ACCESS", "CAN-TP-ADDRESS", "MAX-DELTA-COUNTER", "MODE-SWITCH-SENDER-COM-SPEC-PROPS", "FT", "EXECUTION-ERROR", "ECUC-BOOLEAN-PARAM-DEF", "TLS-CRYPTO-CIPHER-SUITE", "REQUEST-2-SUPPORT", "TIMING-REQUIREMENTS", "TIMING-VARIABLES", "CRC-CORRECTION-FIELD", "UPSTREAM-REFERENCE-IREF", "MC-DATA-INSTANCE-REF", "WAIT-POINTS", "APMC-BOOLEAN-PARAM-DEF", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "EXCLUSIVE-AREA-NESTING-ORDERS", "MACHINE-MODE-IREFS", "TCP-IP-NDP-PACKET-QUEUE-ENABLED", "POST-BUILD-VARIANT-MULTIPLICITY", "AUTHENTICATION-PROPS-REF", "RATE-BASED-MONITORED-EVENT-REF", "CRYPTO-KEY-SLOT-TO-CLIENT-PORT-PROTOTYPE-MAPPING", "NM-MULTIPLE-CHANNELS-ENABLED", "FLEXRAY-NM-CLUSTER", "V-2-X-DATA-MANAGER-NEEDS", "DLT-ARGUMENT-PROPS", "WWH-OBD-DTC-CLASS", "LOWER-LIMIT", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "READONLY", "DEFAULT-ROUTER", "DATA-TYPE-FOR-SERIALIZATION-REF", "FUNCTIONAL-REQUEST-REF", "RESPONSES", "DETERMINISTIC-CLIENT-RESOURCE-NEEDSS", "OBD-DTC-NUMBER", "GLOBAL-TIME-PORT-ROLE", "COND", "SERVICE-ELEMENT-MAPPING-REF", "AH-CIPHER-SUITE-NAME", "APPLICATION-ARRAY-DATA-TYPE", "OUTGOING-CALLBACKS", "SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "TIME-SYNC-SERVER-REF", "DENOMINATOR-GROUP", "SDG-FOREIGN-REFERENCE", "SERVICE-DEPLOYMENT-REF", "NM-REPEAT-MESSAGE-BIT-ENABLE", "ERROR-CONTEXT-REFS", "SDG-CAPTION", "STOP-STATE-MACHINE-REF", "KEY-SLOT-CONTENT-ALLOWED-USAGES", "FIELD-GET-SET-QOS-PROPSS", "RESOLUTION", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "SOURCE-IPV-6-ADDRESS", "IPV-4-CONFIGURATION", "MACHINE-DESIGN", "DATA-TRANSFORMATION-REF-CONDITIONAL", "CONFIRMATION-THRESHOLD", "OFFSET", "MINIMUM-SUPPORTED-UCM-MASTER-VERSION", "DEBOUNCE-ALGORITHM", "DIAGNOSTIC-DEPLOYMENT-PROPS-REF", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "METHOD-RETURN-REFS", "ERROR-HANDLING", "CONTROLLED-DATA-ELEMENT-REF", "OFS-SUB-TLV", "NM-BUSLOAD-REDUCTION-ACTIVE", "LABEL-1", "VALUE-TYPE-REF", "GROUP-IDENTIFIER", "EXECUTION-CONTEXT-REF", "TARGET-SWC-SERVICE-DEPENDENCY-REF", "SDGS", "SERVICE-INTERFACE-ELEMENT-MAPPING-REFS", "PARAMETER-PROVIDE-COM-SPEC", "AR-TYPED-PER-INSTANCE-MEMORYS", "USES-TIME-BASE-RESOURCE", "OFFSETS", "MEMORY-DESTINATION-REFS", "DIAGNOSTIC-SOVD-OPERATION-INTERFACE", "USED-SECONDARY-FID-REFS", "SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF", "SYNCHRONIZED-TRIGGERS", "J-1939-CONTROLLER-APPLICATION-REF", "SECURITY-EVENTS", "ADDRESS-SEMANTICS", "MEM-ALIGNMENT", "REQUIRED-PORT-PROTOTYPE-IREF", "GLOBAL-IN-PACKAGE-REFS", "BLOCK-IF-STATE-ACTIVE-AP-IREF", "TIME-SYNC-CLIENT", "REMOTE-SERVER-CONFIG", "CYCLIC-WRITING-PERIOD", "ALLOW-HALT-DUE-TO-CLOCK", "END-TO-END-PROFILE", "MODE-TRANSITION", "CONTEXT-DATA-PROTOTYPE-REF", "ELEMENT-BYTE-VALUE", "CLIENT-SERVER-PORTS", "TIMESTAMP-INTERVAL", "BSW-MODULE-ENTRY", "ECU-RESOURCE-MAPPINGS", "FRAME-MAPPINGS", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "DIAGNOSTIC-FIM-EVENT-GROUP", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "CAPTION", "REQUESTS", "NUMBER-OF-BITS", "INCLUDED-LIBRARY-REF", "RVF-PIXEL-DEPTH", "SWITCH-ATS-INSTANCE-ENTRY", "UDP-NM-NODE", "PREDECESSOR-ACTION-REF", "IPV-6-ADDRESS-SOURCE", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "TRANSITION-REF", "TRIGGER-INTERFACE-MAPPING", "SW-CARB-DOC", "USER-DEFINED-GLOBAL-TIME-SLAVE", "STARTUP-CONFIGS", "MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS", "SEND-ACTIVITY", "CAN-CLUSTER-VARIANTS", "PREFIX-REF", "SIGNATURE-SCHEME-REF", "MAXIMUM-SUPPORTED-UCM-MASTER-VERSION", "SERVICE-PROXY-SW-COMPONENT-TYPE", "SOURCE-ADDRESS-MASK", "LIMIT-KIND", "BSW-MODULE-ENTRY-REF", "TCP-IP-IP-REASSEMBLY-BUFFER-SIZE", "TD-EVENT-SERVICE-INSTANCE-METHOD", "R-PORT-IREF", "SERVICE-INTERFACE-METHOD-MAPPING", "FUNCTION-IDENTIFIER-REF", "GRANT-REF", "HW-PIN-GROUP-CONTENT", "PROCESSOR-SPEED", "COMPU-DEFAULT-VALUE", "RECOVERY-NOTIFICATION", "COM-MANAGEMENT-MAPPING", "MSR-QUERY-RESULT-TOPIC-1", "CRYPTO-CERTIFICATE-GROUP", "ACCEPTED-STARTUP-RANGE", "I-SIGNAL-TRIGGERING-REF-CONDITIONAL", "USES-ADDRESS-ARBITRATION", "DIAGNOSTIC-SOVD-CONTENT-INTERFACE", "EXIT-MODE-DECLARATION-REF", "IS-REQUIRED", "SYSTEM-SIGNAL-GROUP-REF", "NDP-PROPS", "IP-ADDRESS-KEEP-BEHAVIOR", "PERMISSIBLE-SIGNAL-PATH", "NM-REMOTE-SLEEP-IND-ENABLED", "CONSIDER-BUILD-NUMBER", "SRVT", "COUPLED-CLUSTER-REFS", "DEVELOPMENT-VALUE", "PDU-COUNTER-SIZE", "TRANSFER-REF", "RECEIVER-REFS", "ETHERNET-FRAME-TRIGGERING", "SOCKET-CONNECTION", "REMOTE-CLIENT-CONFIG", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS", "SW-CALPRM-IMPLS", "IS-FUNC-CLUSTER-ENDPOINT-REF", "ENCODING", "APPL-WATCHDOG-LIMIT", "LOCAL-PORT-RANGE-START", "DATA-LINK-LAYER-RULE", "PROVIDED-EVENT-GROUPS", "CRYPTO-NEED", "ACCESS-MODE", "EXITED-MODE-REF", "DATA-TRANSFORMATION-KIND", "CONTEXT-ELEMENT-REF", "DYNAMIC-PART", "CODE", "PERIODIC-EVENT-TRIGGERING", "OBD-RATIO-SERVICE-NEEDS", "GROUP-NUMBER", "ADDITIONAL-NATIVE-TYPE-QUALIFIER", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "TIME-INTERVAL", "DIAGNOSTIC-SOVD-UPDATE", "MEASURED-STACK-USAGE", "STREAM-GATES", "DEFAULT-TRACE-STATE", "STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-ACTION-ITEM", "REQUIRED-FIBEX-ELEMENT-REFS", "PER-STATE-TIMEOUT", "ALIAS-NAME-SET", "RESPOND-TO-RESET", "APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-DEF", "DLT-LOG-CHANNEL-DESIGN-REF", "SW-FEATURE-DEF", "MULTICAST-CREDENTIALS", "GLOBAL-TIME-DOMAIN", "TARGET-ELEMENT-REF", "PORT-INTERFACE-BLUEPRINT-MAPPING", "CLIENT-PORT-REF", "ARRAY-SIZE-HANDLING", "CAN-CLUSTER", "VERSION", "BASE-REF", "TIME-LEAP-PAST-THRESHOLD", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "DDS-SECURE-GOVERNANCE", "MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE", "STARTUP-OPTION", "ETHERNET-VLAN-TRANSLATION-TABLE", "IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF", "COM-OFFER-SERVICE-GRANT", "I-PDU-SIGNAL-PROCESSING", "MULTICAST-UDP-PORT", "SW-VALUE-BLOCK-SIZE", "SERVICE-INTERFACE-MAPPING-SET", "DEBOUNCE-ALGORITHM-REF", "INVALIDATION-POLICYS", "FRAME-SYNC-ENABLED", "ALLOWED-I-PV-6-EXT-HEADER", "TP-CHANNEL-REF", "SOURCE-PORTS", "MICRO-INITIAL-OFFSET-A", "TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY", "STANDARD", "ACL-SCOPE", "ACTIVATION-REASONS", "SPN-REFS", "SPARSE-TIMESTAMP-ENABLED", "DIAGNOSTIC-SOVD-OPERATION", "MEM-USAGE", "RX-MASK", "DIAGNOSTIC-EVENT-WINDOW", "CONTAINED-PDU-TRIGGERING-REF", "DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA", "ALTERNATIVE-NAME", "COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER", "SERVICE-INTERFACE-TREF", "BULK-DATA-CATEGORY", "ROUTINE-CONTROL-CLASS-REF", "FOLLOW-UP-OFFSET", "TBODY", "PARAMETERS", "FLEXRAY-AR-TP-CONFIG", "INHIBITING-SECONDARY-FID-REF", "CRYPTO-SERVICE-CERTIFICATE-REFS", "DLT-MESSAGE-REFS", "NM-COORD-SYNC-SUPPORT", "PARAMETER-ACCESSS", "DIAGNOSTIC-CLEAR-CONDITION", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING", "ALIVE-REFERENCE-CYCLE", "PWM-L", "TRAFFIC-CLASS-PREEMPTION-SUPPORT", "STREAM-IDENTIFICATION-HANDLE-REFS", "HEALTH-CHANNEL-ID", "COMMUNICATION-CYCLE", "FIRST-ELEMENTS", "ACTION-ITEM-LISTS", "GENERIC-DIAGNOSTIC-TRANSPORT-INSTANTIATION", "ON-CHANGE-DATA-PROTOTYPES", "FREEZE-CURRENT-STATE", "SW-IMPL-POLICY", "EMPTY-SIGNAL", "ETHERNET-COMMUNICATION-CONNECTOR", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "SHORT-NAME-PATTERN", "TYPE-OF-EVENT-COMBINATION-SUPPORTED", "CLIENT-ID-DEFINITION", "PROCESS-ARGUMENTS", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "CAN-NM-CLUSTER", "POSITION", "PROCESS-MODE-MACHINE", "TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED", "IEEE-1722-TP-RULE", "CAN-ID-RANGE-MAPPINGS", "MAX-NUMBER-OF-PARALLEL-TRANSFERS", "PRE-ACTIVATE-REF", "VENDOR-API-INFIX", "HW-CATEGORY", "TRANSMISSION-DEADLINE", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REF", "LOCAL-IP-SEC-RULE-REFS", "SETTER-CALL-REF", "SOURCE-EVENT-REF", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "COMPILER", "DATA-DUMP-ENTRY", "MACRO-INITIAL-OFFSET-A", "TIME-SYNC-TECHNOLOGY", "RECORD-ELEMENTS", "RELEASE-NOTES-REF", "MIN-MARGIN", "OUTER-PORTS", "SOURCE-ADDRESS-RANGE-END", "NM-PDU-REF", "DATA-WRITE-COMPLETED-EVENT", "WRITING-FREQUENCY", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "INTERNAL-TRIGGER-OCCURRED-EVENT", "DDS-SERVICE-INSTANCE-EVENT-CP", "TCP-RETRANSMISSION-TIMEOUT", "CONTAINED-PDU-TRIGGERING-REFS", "BUS-SPECIFIC-NM-ECU", "VERIFY-REF", "UDP-COLLECTION-BUFFER-SIZE-THRESHOLD", "RELATED-TRACE-ITEM-REF", "EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE", "NM-SHUTDOWN-DELAY-TIMER", "RESET-TO-DEFAULT-SUPPORTED", "COMMUNICATION-CONTROLLER-REF", "RVF-COLOR-SPACE", "NUMBER-OF-ELEMENTS", "BASE-CYCLE", "REQUIRED-SERVICE-INSTANCE-ID", "CRYPTO-ELLIPTIC-CURVE-PROPS", "PREEMPTABILITY", "DEADLINE-SUPERVISION", "CAN-TP-ADDRESS-REFS", "SW-VALUES-PHYS", "TX-DIRECT-PDU-PRIORITY", "SYSTEM-TRIGGERED-EVENTS", "COLD-START-ATTEMPTS", "NAME", "A-2-L-DISPLAY-TEXT", "ECUC-URI-REFERENCE-DEF", "SCHEDULE-TABLES", "PROPERTYS", "HEALTH-CHANNEL-ARGUMENT-REFS", "PROVIDED-MODE-GROUP-REF", "LIFE-CYCLE-INFO", "NETWORK-ENDPOINT", "REQUIRED-AR-ELEMENT-REF", "CONTROL-OPTION-RECORD-PRESENT", "PR-PORT-PROTOTYPE", "CONNECTION-NEGOTIATION-BEHAVIOR", "MACHINE-TIMING", "I-SIGNAL-PORT-REFS", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL", "TDLET-ZONE-CLOCK", "NO-SUPERVISIONS", "AP-APPLICATION-ERROR-SET", "SUBNET-NUMBER", "REQUEST-VEHICLE-INFORMATION-CLASS-REF", "SOURCE-MAC-ADDRESS", "IPV-6-ADDRESS", "APMC-ENUMERATION-PARAM-DEF", "PROVIDED-MODE-GROUPS", "FULLY-QUALIFIED-DOMAIN-NAME", "IMPOSITION-TIME-DEFINITION-GROUP", "KEY-SLOT-ALLOWED-MODIFICATION", "SUP", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "VARIABLE-DATA-PROTOTYPE", "DEPENDENCY-ON-ARTIFACT", "COMMUNICATION-MODE", "EXECUTION-ERROR-REF", "MANAGED-COUPLING-PORTS", "SEGMENT-POSITION", "FIRST-VALUE", "DDS-DOMAIN-RANGE", "OPTION-NAME", "MONITORED-IDENTIFIER-REF", "DIAGNOSTIC-EXTRACT-REF", "REKEY-INTERVAL", "ECUC-REFERENCE-DEF", "IN-MEASUREMENT-SET", "UNIT-DISPLAY-NAME", "MAX-NUMBER-OF-ELEMENTS", "SERIALIZER-REF", "TRACED-FAILURES", "TCP-IP-DHCP-V-6-SOL-DELAY-MIN", "NM-NETWORK-TIMEOUT", "SECURE-COM-PROPS-FOR-UDP-REF", "ALLOW-CONTENT-TYPE-CHANGE", "ENUMERATION-MAPPING-TABLE", "SERVICE-MINOR-VERSION", "BSW-PARAMETER-POLICY", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "DDS-EVENT-DEPLOYMENT", "BASE-FREQUENCY", "ECUC-DEF-EDITION", "MAX-PWM-O", "SENDER-RECEIVER-TO-SIGNAL-MAPPING", "TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE", "USER-DATA-SUB-TLV", "TP-ECU", "APPLICABILITY-INFOS", "PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT", "LIMIT", "SW-FEATURE-DEFS", "IDENTICAL-MAPPING", "DEFAULT-ACTION", "J-1939-NODE", "RELEASED-TRIGGER-POLICYS", "NM-DATA-ENABLED", "IPV-4-PROPS", "PORT-BLUEPRINT-MAPPINGS", "SECURE-COM-PROPS-SET", "DIAGNOSTIC-COM-CONTROL-CLASS", "USE-VARIABLE-DA", "SOVD-CONTENT-REFS", "NESTED-ELEMENT-REFS", "DIAGNOSTIC-EXTENDED-DATA-RECORD-ELEMENT", "COUPLING-PORT", "TCP-DELAYED-ACK-TIMEOUT", "CPU-USAGE", "DIAGNOSTIC-TROUBLE-CODE-PROPS-REF", "PERSISTENCY-DEPLOYMENT-ELEMENT-REF", "ATS-INSTANCE-ENTRYS", "HOP-LIMIT", "MAX-NUMBER-OF-CONNECTIONS", "MULTICAST-THRESHOLD", "EXTERNAL-CLOCK-SYNCHRONISATION", "SECURITY-ACCESS-LEVEL", "MAX-SUPPORTED-DID-LENGTH", "ACL-ROLE-REFS", "ERROR-SIGNALING-ENABLED", "SCHEDULER-NAME-PREFIXS", "COM-EVENT-GRANT", "FIREWALL-STATE-IREF", "ACL-PERMISSION", "TERMINATING-CHECKPOINT-TIMEOUT-UNTIL-TERMINATION", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "CRYPTO-PROVIDER-INTERFACE", "STATIC-PART", "REQUEST-METHOD", "PUTS", "TCP-IP-ARP-REQUEST-TIMEOUT", "NOTIFIED-STATUSS", "APMC-URI-INSTANCE-REFERENCE-DEF", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "SIGNAL-SERVICE-TRANSLATION-PROPS", "EXECUTABLE-GROUP", "OPERATOR", "ASYNCHRONOUS-SERVER-CALL-POINT-REF", "DATA-TRANSFORMATION-REF", "DEST-SDG-REF", "EVALUATED-VARIANT-REF", "DDS-PROVIDED-SERVICE-INSTANCE", "CLIENT-SERVICE-MINOR-VERSION", "PDELAY-RESPONSE-ENABLED", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "SEC-OC-SECURE-COM-PROPS", "CRYPTO-KEY-SLOT-DESIGN-REF", "FDC-THRESHOLD-STORAGE-VALUE", "IKE-REKEY-TIME", "SOMEIP-REMOTE-UNICAST-CONFIG", "NETWORK-MANAGEMENT-VECTOR-LENGTH", "SM-INTERACTS-WITH-NM-MAPPING", "VALIDATION-CONFIGURATION", "TCP-IP-NDP-SLAAC-DELAY-ENABLED", "MODE-SWITCH-INTERFACE", "INDUSTRY-GROUP", "PARTS", "NM-NID-POSITION", "RUNS-INSIDES", "SEGMENT-RECEPTION-TIMEOUT-TIME-RESPONSE", "SCALE-CONSTR", "BSW-OPERATION-INVOKED-EVENT", "HANDLE-NEVER-RECEIVED", "TRANSMITTER-REF", "RUNNABLES", "TRANSIENT-FAULT", "DERIVED-OBJECT-REF", "DIAGNOSTIC-EVENT-REF", "MACHINE-MODE-MACHINES", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "MULTICAST-CONNECTOR-REFS", "RESPONSE-ON-EVENT-CLASS-REF", "DEFAULT-MODE-REF", "ROOT-COMPOSITION", "MULTIPLEXED-I-PDU", "UNICAST-NETWORK-ENDPOINT-REF", "HW-PIN-GROUP", "FUNCTION-GROUP-ERROR-MAPPING", "IPV-4-DHCP-SERVER-CONFIGURATION", "NAMED-CURVE-ID", "TARGET-VARIABLE-DATA-PROTOTYPE-REF", "LOG-TRACE-LOG-MODES", "COM-FIELD-GRANT", "TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE", "FIRE-AND-FORGET-MAPPINGS", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "DEPENDS-ON", "IS-GLOBAL", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "INTENDED-PARTIAL-INITIALIZATION-COUNT", "SERVER-PORT-REF", "OPERATION-IREFS", "ACCEPT-VARIABLE-SA", "USER-DEFINED-CLUSTER-VARIANTS", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF", "ADAPTIVE-FIREWALL-MODULE-INSTANTIATION", "PRE-ACTIVATE-REFS", "STREAM-GATE-REF", "SEGMENT-REFS", "FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS", "IP-ADDRESS", "ATTRIBUTE-NAME", "ELEMENTS", "MODE-CONDITION", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "DURABILITY-SERVICE-MAX-SAMPLES", "REQUIRE-RESOURCES", "CONTEXT-DATA-PROTOTYPE-REFS", "SUPERVISION-CHECKPOINT", "PROPS", "DATA-CONSTR", "STATE-MANAGEMENT-COMPARE-FORMULA", "DDS-DOMAINS", "DIAGNOSTIC-FREEZE-FRAME-REF", "CAN-ADDRESSING-MODE", "MIN-PWM-S", "ACL-ROLE", "ROLE", "LIN-CONFIGURABLE-FRAMES", "NOMINAL-EXECUTION-TIME", "POSSIBLE-AP-ERROR-SET-REF", "MAX-ERROR-STATE-VALID", "ROUTING-ACTIVATION-TYPE", "SLOT-ID", "MESSAGE-ID", "APMC-REVISION-LABEL-PARAM-DEF", "EVENT-MAPPINGS", "EVENT-ID", "SUB-FUNCTION-REF", "I-PDU-IDENTIFIER-TCP-REF", "TIME-SYNC-SERVER", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "START-IREF", "AGGREGATION-CONDITION", "UNUSED-BIT-PATTERN", "ACCESS-POINT-REF", "USES-END-TO-END-PROTECTION", "UPPER-LIMIT", "POSSIBLE-ERROR-REF", "PAYLOAD-BYTE-PATTERN-RULES", "STATE-MANAGEMENT-SYNC-ACTION-ITEM", "ROLE-BASED-DATA-TYPE-ASSIGNMENT", "TIME-VALUE-VALUE-VARIATION-POINT", "CONTEXT-R-PORT-PROTOTYPE-REF", "IMPLICIT-DATA-ACCESS-IREFS", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "ABSOLUTE", "INTERFACE-MAPPING-SET", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "SO-AD-ROUTING-GROUP", "VARIATION-RESTRICTION", "SAFETY-CONDITION", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "UAS-ID", "CYCLIC-TIMING", "TARGET-SIGNAL-REF", "COLLECTION-SEMANTICS", "METHOD-MAPPING", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "USER-REFS", "TIME-BASE-PROVIDER-REF", "DIAGNOSTIC-EXTENDED-DATA-RECORD-NEEDS", "COMPU-METHOD-REF", "CRYPTO-KEY-REF", "KEY-IDENTITY-REF", "PADDING-VALUE", "CKN-REF", "MAX-CYCLES", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "EXPLICIT-RPT-PROFILE-SELECTION-REFS", "DIAGNOSTIC-INDICATOR", "ECU-RESET-CLASS-REF", "GENERIC-TP", "MAC-ADDRESS-STREAM-ID", "FLAT-MAP", "ACL-OBJECT-REF", "APMC-STRONG-REVISION-LABEL-PARAM-DEF", "ALLOW-MASTER-RATE-CORRECTION", "ENTRY-MODE-DECLARATION-REF", "SOCKET-OPTIONS", "DO-IP-TP-CONNECTION", "TARGET-OPERATION-REF", "EXCLUSIVE-AREA-REF-CONDITIONAL", "PHM-ACTION", "BINARY-MANIFEST-ITEM-DEFINITION", "NM-NODE-ID-ENABLED", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "MAC-UNICAST-ADDRESS", "OFFSET-CORRECTION-JUMP-THRESHOLD", "ROOT-SW-COMPONENT-PROTOTYPE", "INHIBITING-FID-REF", "FUNCTION-GROUP-STATE-IREFS", "MOLAR-AMOUNT-EXP", "HEALTH-CHANNEL-EXTERNAL-STATUS", "FM-FEATURE-DECOMPOSITION", "TRANSMISSION-MODE-TRUE-TIMING", "LOG-TRACE-PROCESS-DESC", "OFFSET-CORRECTION-ADAPTION-INTERVAL", "BLACKLISTED-VERSIONS", "DO-IP-REQUEST-CONFIGURATION", "SEPARATION-CYCLE-EXPONENT", "TCP-MSL", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "LOCAL-SUPERVISION-ENTITY-REF", "MAX-PERIODIC-DID-TO-READ", "FM-COND", "TIMEOUT-CS", "ACTUAL-EVENT-REF", "PNC-CLUSTER-VECTOR-LENGTH", "DO-IP-NETWORK-CONFIGURATION-DESIGN", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "LITERAL-PREFIX", "DIAGNOSTIC-CONNECTIONS", "REMARK", "DESTINATION-MAC-ADDRESS-MASK", "DDS-METHOD-REQUEST-TOPIC-REF", "AUTOSAR", "SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING", "AUXILIARY-FIELD-DEFINITIONS", "FIREWALL-RULE-PROPS", "CLEAR-FROM-VALID-TO-INVALID", "END-ADDRESS", "TCP-IP-NDP-DELAY-FIRST-PROBE-TIME", "TLS-JOB-REQUIREMENT-REF", "QUEUED-RECEIVER-COM-SPEC-PROPS", "I-PDU-IDENTIFIER-TCP-REFS", "DLT-ARGUMENT-PROPS-REF-CONDITIONAL", "CRYPTO-SERVICE-KEY", "USER-DEFINED-TRANSFORMATION-DESCRIPTION", "TRANSPORT-PLUGINS", "PARAMETER-DATA-PROTOTYPE-IREF", "PROCESS-EXECUTION-ERROR", "COLLECTION-TIMEOUT", "SW-AXIS-CONT", "OCCURRENCE-COUNTER-PROCESSING", "RESOURCE-LIMITS", "OFFSET-CORRECTION-START", "ACL-OBJECT-CLASS", "SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF-CONDITIONAL", "INTERPOLATION-ROUTINE-MAPPING-SET", "DEF-LIST", "DIAGNOSTIC-ENV-CONDITION-FORMULA", "SPECIFIC-PHYSICAL-CHANNEL-REF", "CUSTOM-SERVICE-CLASS-REF", "VALIDATION-FORMULA", "INDENT-SAMPLE", "DESTINATION-URI-POLICY", "LIN-MASTER-CONDITIONAL", "FLOW-METERINGS", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "IS-VARIADIC-TEMPLATE", "CLIENT-ID-DEFINITION-SET", "SW-COMPOSITIONS", "PRODUCT-RELEASE", "CAN-FD-BAUDRATE", "CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL", "COMMUNICATION-RESOURCE-REF", "TIMING-CONDITIONS", "CAN-XL-PROPS-REFS", "INCLUDE-REF", "CONNECTORS", "PRE-ACTIVATION-REBOOT", "PDUS", "RESOURCE-NEEDS-REF", "SWITCH-MAC-ADDRESS-LEARNING-MODE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE", "BINARY-MANIFEST-REQUIRE-RESOURCE", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "DIAGNOSTIC-ROUTINE-INTERFACE", "DDS-METHOD-DEPLOYMENT", "SEC-OC-CRYPTO-MAPPING-REF", "ACTION-ITEMS", "CLIENT-ID-DEFINITIONS", "R-PORT-PROTOTYPE", "SCOPE-EVENT-REFS", "NM-USER-DATA-LENGTH", "IDENTITY-REF", "CRC-DOMAIN-NUMBER", "NM-COM-CONTROL-ENABLED", "CONTEXT-SWC-PROTOTYPE-REF", "BASE-COMPOSITION-REF", "CONTENT-URI", "UDP-NM-CLUSTER-COUPLING", "DIAGNOSTIC-EXTENDED-DATA-RECORD-INTERFACE", "SOURCE-CAN-ID-MASK", "RATIONALE", "OFFSET-CORRECTION-OUT", "OFFSET-SI-TO-UNIT", "MODULE-ID", "ADAPTIVE-AUTOSAR-APPLICATION", "ELLIPTIC-CURVE-REFS", "NRC-VALUE", "NM-BUS-LOAD-REDUCTION-ENABLED", "GET", "OBD-INFO-SERVICE-NEEDS", "CRYPTO-KEY-SLOTS", "STOP-AGENT-REF", "BYPASS-ETHER-TYPE", "SOCKET-CONNECTION-REFS", "ETHERNET-CLUSTER-VARIANTS", "SELECT-BLOCK-FOR-FIRST-INIT-ALL", "TRANSPORT-PROTOCOLS", "APPLICATION-RECORD-ELEMENT", "DIAGNOSTIC-CONTRIBUTION-SET", "CRYPTO-KEY-DESCRIPTION", "REMOTE-CERTIFICATE-REFS", "DERIVED-INTERFACE-REF", "SECONDARY-DOWNLOAD-URIS", "BLUEPRINT-MAPPING", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "ACCESSED-VARIABLE", "PROVIDED-CLIENT-SERVER-ENTRYS", "ARP-PROPS", "CHAPTER", "PARTICIPATING-J-1939-CLUSTER-REFS", "PHYSICAL-DIMENSION-REF", "AUTHENTICATION-RETRIES", "APMC-URI-PARAM-VALUE", "USER-DEFINED-PHYSICAL-CHANNEL", "CHECKPOINT-TRANSITION", "SWC-TRIGGER-IREF", "SOURCE-INSTANCE-IREF", "BIT-NUMBER", "MODE-MAPPING", "CYCLE-REPETITION", "DYNAMIC-SLOT-IDLE-PHASE", "LIN-COMMUNICATION-CONNECTOR", "BUILD-ACTION-MANIFEST-REF-CONDITIONAL", "TD-EVENT-VFB-PORT-GROUP", "APMC-CONTAINER-VALUE", "J-1939-PROTECTED-I-PDU", "SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "OBD-RELEVANCE", "MICROTICK-DURATION", "MSR-QUERY-P-2", "SEPARATION-TIME", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "ECU-INSTANCE-PROPS", "INHIBITING-SECONDARY-FID-REFS", "PNC-MAPPING-IDENT-REF-CONDITIONAL", "SIGNAL-BASED-METHOD-REF", "REVISION-LABEL-P-1", "DIAGNOSTIC-MEMORY-REF", "FIELD-REF", "ROLE-BASED-RESOURCE-DEPENDENCY", "DATA-PROTOTYPE-MAPPING", "APPLICATION-MODE-IREF", "ONE-EVERY-N", "SOURCE-FIELD-REF", "I-PDU-PORT-REF", "DIAGNOSTIC-SOVD-CONTENT-GROUP", "REQUESTOR-REF", "CHAPTERS", "COUNTER-JUMP-DOWN", "EVENT-GROUP-REFS", "FIELD-DEPLOYMENTS", "INCLUDED-CERTIFICATE-REF", "PHYSICAL-CHANNEL-REF", "CRC-SOURCE-PORT-IDENTITY", "VENDOR-ID", "DIAGNOSTIC-SERVICE-SW-MAPPING", "TP-ADDRESS-REF", "ECUC-MODULE-CONFIGURATION-VALUES", "FRESHNESS-PROPS-REF", "TRAFFIC-LIMITATION-FILTERS", "ARGUMENTSS", "IDSM-MODULE-INSTANTIATION", "REPORTABLE-SECURITY-EVENT-REF", "SWC-SERVICE-DEPENDENCY", "MSG-ID-MASK", "RX-PDU-POOL-REF", "MIN-MINOR-VERSION", "IP-V-6-ADDRESS", "SDG-CAPTION-REF", "VLAN-MODIFIER-REF", "INDEXED-ARRAY-ELEMENT", "HARDWARE-CONFIGURATION", "TCP-IP-DHCP-V-6-INF-DELAY-MAX", "APMC-VALUE-COLLECTION", "MC-VARIABLE-INSTANCES", "VEHICLE-ROLLOUT-STEP", "SWC-TO-IMPL-MAPPING", "REST-ELEMENT-DEF", "MSR-QUERY-RESULT-P-1", "DEPENDS-ON-REFS", "VLAN-PRIORITY", "CONTEXTS", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "MAPPED-DATA-ELEMENT-IREF", "APPLICATION-ENDPOINT", "SOURCE-EVENT-REFS", "DURABILITY-SERVICE-MAX-SAMPLES-PER-INSTANCE", "CONNECTOR-REF", "SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL", "DURABILITY-KIND", "DDS-SERVICE-INSTANCE-FIELD-CP", "MAP-ELEMENT-TUPLES", "ADDRESS-RANGE-UPPER-BOUND", "SUSPEND-TO-RAM-MODULE-INSTANTIATION", "SUB-CONTAINERS", "MAC-MULTICAST-GROUP-REF", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "OWNERSHIP-STRENGTH", "DISCOVERY-PROTECTION-KIND", "INITIAL-DELAY-MIN-VALUE", "I-SIGNAL-REFS", "CONTAINED-PROCESS-REFS", "CONSUMED-SERVICE-INSTANCE", "NEXT-STATE-IREF", "CAN-TP-CHANNEL", "TCP-RECEIVE-WINDOW-MAX", "DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF", "STATE-MANAGEMENT-NM-ACTION-ITEM", "MAPPED-OPERATION-IREF", "I-SIGNAL-I-PDU-GROUP", "DOMAIN-ID", "CAN-CONTROLLER-FD-REQUIREMENTS", "PRMS", "WITH-AUTO", "IDSM-REPORTING-MODE-PROVIDER-MAPPING", "CONSISTENCY-NEEDSS", "APPLICATION-ENDPOINT-REF-CONDITIONAL", "APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION", "TD-EVENT-VFB-PORT-REF", "CIPHER-SUITE-SHORT-LABEL", "NUMBER-OF-INSTRUCTIONS", "BURST-SIZE-RESPONSE", "DHCP-SERVER-CONFIGURATION", "PREDECESSOR-ACTION-REFS", "KEY-EXCHANGE-REFS", "CUSTOM-DOCUMENT-ELEMENT-REF", "I-SIGNAL-TO-I-PDU-MAPPING", "CHANNEL-MODE", "NM-REPETITION-CYCLE", "DOIP-CONNECTION-REF", "MAX-MAJOR-VERSION", "ENABLES-LOG-TRACE", "DTC-SETTING-CLASS-REF", "TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED", "GLOBAL-SUPERVISION-ENTITY", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "OS-TASK-PROXY", "HW-ELEMENT-CONNECTIONS", "DIAGNOSTIC-ACCESS-PERMISSION", "REST-OBJECT-REF", "STREAM-IDENTIFICATION-WILDCARD", "EXCLUSIVE-AREA-NESTING-ORDER-REF", "ECUC-TEXTUAL-PARAM-VALUE", "VALUE-IREF", "ETH-IP-PROPS-REFS", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "IKE-REAUTH-TIME", "PATTERN-JITTER", "LATENCY-BUDGET-DURATION", "SW-MAX-AXIS-POINTS", "TRANSFORMER-STATUS-FORWARDING", "ROOT-PARAMETER-DATA-PROTOTYPE-REF", "ETH-IP-PROPS-REF", "INDICATIONS", "PHYSICAL-CHANNELS", "I-PDU-TIMING-SPECIFICATIONS", "REMOTE-CLIENT-QUEUE-LENGTH-CONTRIBUTION", "USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF", "TT", "MAXIMUM-MEMORY-CONSUMPTION", "BSW-TRIGGER-DIRECT-IMPLEMENTATION", "DATA-ELEMENT-REF", "ISSUED-TRIGGERS", "RAW-DATA-STREAM-INTERFACE-REF", "FIBEX-ELEMENT-REF", "IMPLIED-OPERATION-REF", "PROTOCOL-KIND", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "SDG-REFERENCE", "CODE-LABEL", "NETWORK-REPRESENTATION-PROPS", "PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE", "TRACE", "DIAGNOSTIC-ENABLE-CONDITION", "NM-BUSLOAD-REDUCTION-ENABLED", "TOPIC-DATA", "APMC-FLOAT-PARAM-DEF", "PARAMETER-INTERFACE", "LIN-SLAVE-CONDITIONAL", "FILTER-ACTION-DEST-PORT-MODIFICATION", "INTERFACE-BLUEPRINT-MAPPINGS", "APMC-NUMERICAL-PARAM-VALUE", "PHYSICAL-DIMENSION-MAPPING-REF", "GENERIC-MATH", "SOFTWARE-CLUSTER-REF", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "BSW-MODE-SWITCH-EVENT", "EXCLUSIVE-AREA-NESTING-ORDER", "SET-FUNCTION-GROUP-STATE-IREF", "MINIMUM-TX-CONTAINER-QUEUE-SIZE", "CLIENT-SERVER-APPLICATION-ERROR-MAPPING", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "OPTIONAL-ELEMENTS", "MAX-ERROR-STATE-INIT", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "DATA-IDENTIFIER-REFS", "COUPLING-ELEMENT", "POST-BUILD-VARIANT-CRITERION-VALUES", "DEFAULT-VALUE-ELEMENT", "OPT-IMPL-RECORD-ELEMENTS", "NM-CLUSTER-COUPLINGS", "NM-CLUSTERS", "ACK-REQUEST", "COLLISION-RESOLVING-SCHEDULE-REF", "APPLICATION-CONTEXT-REF", "APMC-MAC-ADDRESS-PARAM-VALUE", "ASSOCIATED-COM-I-PDU-GROUP-REFS", "BSW-ENTITY-REF", "RATE-LIMITATION-FILTERS", "SYSTEM-SIGNAL-REFS", "PATH", "RPT-SERVICE-POINT", "COLLECTION-PROPS", "MINIMUM-DISTANCES", "CAN-TP-CONNECTION", "CRYPTO-DRIVER", "BSW-SERVICE-DEPENDENCY-REF", "INITIAL-CHECKPOINT-REF", "MAPPINGS", "COMPUTED", "REFILL-AMOUNT", "TIMEOUT-BR", "FIELD-ELEMENTS", "NM-CONTROL-BIT-VECTOR-ENABLED", "FOLLOW-UP-TIMEOUT-VALUE", "SERVICE-INSTANCE-REFS", "RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS", "PNC-FILTER-ARRAY-MASK", "AUTOSAR-DATA-TYPE-REF-CONDITIONAL", "SLAVE-EVENT-REF", "TRANSFORMER-CHAIN-REFS", "IDSM-PROPERTIES", "TIME-BASES", "REQUIRED-OUTER-PORT-REF", "CONTROL-CONSUMED-EVENT-GROUP-REF", "SECURITY-EVENT-DEFINITION-REF-CONDITIONAL", "BSW-CALLED-ENTITY", "SERVICE-RESOURCE-REF", "EVENT-DEPLOYMENTS", "OBD-RATIO-DENOMINATOR-NEEDS", "CRF-PULL", "ANALYZED-EXECUTION-TIME", "LOCAL-SUPERVISION-ENTITY-REFS", "MODULE-REF", "COUPLING-PORT-CONNECTION", "MAJOR-PROTOCOL-VERSION", "BY-PASS-POINT-IREFS", "I-SIGNAL-PORT-REF", "FLEXRAY-PHYSICAL-CHANNEL", "LIN-CHECKSUM", "MEMORY-ALLOCATION-KEYWORD-POLICY", "DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL", "TARGET-REQUIRED-OPERATION-REF", "EXCLUSIVE-AREA-POLICYS", "SEC-OC-JOB-SEMANTIC", "ICMP-V-6-PROPS", "SW-AXIS-GROUPED", "KEY-VALUE-PAIRS", "INTERPOLATION-ROUTINE-MAPPING-SET-REFS", "DATA-WRITE-ACCESSS", "SECOND-PORT-REF", "PERSISTENCY-FILE-STORAGE", "PREFIX", "DATA-TYPE-MAPS", "ELEMENT-POSITION", "PERSISTENCY-DEPLOYMENT-URI", "IEEE-1722-TP-ACF-CAN", "KIND", "MAX-SERVICE-POINT-ID", "SIZE-OF-STRING-LENGTH-FIELDS", "MATCHING-RULE-REFS", "STATE-REF", "PDU-COLLECTION-TIMEOUT", "I-SIGNAL-MAPPING", "LOCAL-COM-ACCESS-CONTROL-ENABLED", "TD-EVENT-I-SIGNAL", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "SW-SYSTEMCONSTANT-VALUE-SET-REF", "PERSISTENCY-CENTRAL-STORAGE-URI", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "SW-GENERIC-AXIS-DESC", "IEEE-1722-RAW-DATA-STREAM-PRODUCER-INTERFACE", "ECUC-DEFINITION-REF", "ADMIN-DATA", "BINARY-MANIFEST-RESOURCE-DEFINITION", "MODE-DECLARATION-GROUP-PROTOTYPE", "PNC-WAKEUP-ENABLE", "MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "MAXIMUM-SEGMENT-LENGTH-REQUEST", "TP-ECUS", "METADATA-PROTECTION-KIND", "MIN-ALIVE-CYCLE", "APPLICATION-TYPE-TREF", "DATA-PROTOTYPE-TRANSFORMATION-PROPSS", "SENDER-RECEIVER-ANNOTATIONS", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "JOB-REF", "BSW-RESOURCE-ESTIMATION", "TP-CONFIGURATION", "SUB-TYPE", "LIN-TP-CONNECTION", "TP-ADDRESS-EXTENSION-VALUE", "ASSOCIATED-COM-I-PDU-GROUP-REF", "CRC-FLAGS", "BSW-MODE-SENDER-POLICY", "IEEE-1722-TP-CONNECTION-REF-CONDITIONAL", "OP", "SECURITY-LEVEL-REFS", "COUPLING-PORT-REF-CONDITIONAL", "TARGET-TYPE", "CAN-IDENTIFIER-MASK", "REQUIRED-AR-ELEMENT-REFS", "COMPRESSED-SOFTWARE-PACKAGE-SIZE", "SETTER-RETURN-SIGNAL-REF", "RPT-SUPPORT-DATA", "COMMUNICATION-CLUSTER-REFS", "IMPL-INIT-VALUE", "LOCAL-NETWORK-ENDPOINT-REF", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "DO-IP-FUNCTIONAL-CLUSTER-DESIGN", "TD-EVENT-MODE-DECLARATION-TYPE", "COLSPEC", "SUB-NODE-CHANNEL-REF", "ECUC-QUERY-EXPRESSION", "RESOURCE-ESTIMATIONS", "SECONDARY-FRESHNESS-VALUE-ID", "NUMBER-OF-RESTART-ATTEMPTS", "SECURITY-EVENT-PROPS-REF", "IDENTIFIER", "ANNOTATIONS", "ARBITRARY-ADDRESS-CAPABLE", "TCP-UDP-CONFIG", "DEADLINE-SUPERVISIONS", "SW-FILL-CHARACTER", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION", "MAX-ALIVE-CYCLE", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF", "SYSTEM-SIGNAL-TO-COM-RESOURCE-MAPPINGS", "ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "TTCAN-COMMUNICATION-CONTROLLER-VARIANTS", "EVENT-GROUP-IDENTIFIER", "GRAPHIC", "AUTHENTICATION-BUILD-ATTEMPTS", "INGRESS-PRIORITY", "RECOVERY", "SPECIFIC-CHANNEL-REF", "OPT-ARGUMENT-REFS", "STATIC-SOCKET-CONNECTION-REF", "EOC-EXECUTABLE-ENTITY-REF", "ETHERNET-FRAME", "DESTINATION-IP-ADDRESS", "USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT", "ACF-BUS-PART-REF", "PROCESSOR-CORE", "PHYSICAL-CHANNEL-REFS", "DESTINATION-REF", "I-PDU-IDENTIFIERS", "SUB", "API-SERVICE-PREFIX", "DIAGNOSTIC-SOVD-CONTENT-ELEMENT-INTERFACE", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "BINARY-MANIFEST-ITEM-POINTER-VALUE", "STARTS-ON-EVENT-REF", "SUPPORTED-CONFIG-VARIANT", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "SW-INTENDED-RESOLUTION", "CAS-RX-LOW-MAX", "CHECKSUM-VALUE", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "MODE-CONDITION-REF", "DESTINATION-URI-NESTING-CONTRACT", "MSR-QUERY-RESULT-CHAPTER", "COMPONENT-IREF", "OS-MODULE-INSTANTIATION", "TX-MASK", "DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "DELAY-COMPENSATION-A", "SYSTEM-SIGNAL-GROUP-TO-COM-RESOURCE-MAPPINGS", "I-PDU-TRIGGERING", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "SW-MAINTENANCE-NOTESS", "COM-PROCESSING-PERIOD", "DATA-FILTER", "FILE-REF", "LOCAL-UNICAST-ADDRESSS", "APPLICATION-ID", "PRE-ACTIVATE-IREF", "ASSIGNED-DATA-TYPES", "EID", "TLV-DATA-IDS", "SOFTWARE-CLUSTER-COM-RESOURCE-REF", "ENDPOINT-CONFIGURATION-REF", "ETH-IP-PROPS", "SW-FEATURE-DESC", "SYSTEM-COM-SPEC-DEFINITION-REFS", "DATA-PROTOTYPE-TRANSFORMATION-PROPS", "DDS-CP-PROVIDED-SERVICE-INSTANCE", "SEGMENT-LENGTH", "LOG-TRACE-LOG-MODE", "LIN-SLAVE", "CRYPTO-CERTIFICATE", "MODE-ACCESS-POINTS", "SERVICE-INTERFACE-REF", "SW-CALPRM-AXIS-SET", "ENABLE-CONDITIONS", "SW-GENERIC-AXIS-PARAM-TYPES", "SUPPORT-INFO-BIT", "CRYPTO-PROVIDER", "SWC-TO-ECU-MAPPING", "SERVICE-INTERFACE-DEPLOYMENT-REF", "PERSISTENCY-FILE", "SIGNAL-AGE", "DESC", "UNIT-AND-SCALING-ID", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF", "CAN-XL-CONFIG", "COMPOSITE-NETWORK-REPRESENTATION", "FILE-REFS", "DYNAMIC-BS", "FEATURE-MODEL-REFS", "CONTEXT-COMPONENT-PROTOTYPE-REF", "SUB-FUNCTION-REFS", "BASE-REFERENCE-REF", "SOMEIP-RULE", "TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "DO-IP-LOGICAL-ADDRESS-REF", "ROOT-DATA-PROTOTYPE-IN-SR-REF", "BSW-SCHEDULER-NAME-PREFIX", "DESTINATION-REFS", "REFERENCES", "BYPASS-VLANS", "TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP", "PROVIDED-INTERFACE-TREF", "PORT-INTERFACE-REF", "DESTINATION-NETWORK-MASK", "DIAGNOSTIC-SERVICE-TABLE", "TTL-MAX", "DOCUMENT-ELEMENT-SCOPE", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS", "SECURITY-EVENT-REPORT-INSTANCE-DEFINITION-REF", "MAC-SEC-PROPS", "DIAGNOSTIC-SECURITY-ACCESS", "MICRO-INITIAL-OFFSET-B", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "MAX-MARGIN", "TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE", "OBJECT-DEFINTION-REF", "POST-BUILD-CHANGEABLE", "OPERATION-CYCLE", "ECU-MAPPING", "GLOBAL-SUPERVISION-NEEDS", "USE-INSTEAD-REF", "CONDITION", "ETHERNET-RAW-DATA-STREAM-MAPPING-REF", "INHIBITION-MASK", "OFNS-DATA-ID-LIST", "LIN-SLAVE-VARIANTS", "PHYSICAL-DIMENSION-MAPPINGS", "COMMITTED-INFORMATION-RATE", "MAX-DEADLINE", "ACCESS-PERMISSION-VALIDITY", "TLS-IAM-REMOTE-SUBJECT", "PNC-VECTOR-OFFSET", "COLLECTED-INSTANCE-IREFS", "BUILD-ACTION-ENVIRONMENT", "USER-DEFINED-ETHERNET-FRAME", "LOG-TRACE-PROCESS-ID", "POSITIVE-INTEGER-VALUE-VARIATION-POINT", "TLV-DATA-ID-DEFINITION-REFS", "AFFECTED-FUNCTIONAL-CLUSTER", "ROUGH-ESTIMATE-HEAP-USAGE", "CAN-XL-CONFIG-REQS", "IKE-RAND-TIME", "PROFILE-NAME", "HW-ATTRIBUTE-VALUE", "EVENT-SOURCE-REF", "MC-PARAMETER-ELEMENT-GROUP", "PREVIOUS-DATA-TYPE-REF", "SECURE-PROPS-FOR-UDP-REF", "RELATIVE-REPRESENTATION-TIME", "CRYPTO-SERVICE-QUEUE-REF", "ARGUMENT-REF", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL", "CONTEXT-DATA-PROTOTYPE-IN-SR-REF", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "TOOL", "SWC-BSW-MAPPING-REF", "DISABLED-IN-MODE-IREFS", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "RESOURCE-GROUP-REFS", "ROOT-DATA-PROTOTYPE-IN-CS-REF", "MAXIMUM-INTENDED-BINDING-TIME", "ECUC-VALUE-COLLECTION", "MODE-DECLARATION-MAPPING-SET", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE", "MAX-SDU-SIZE", "NM-CAR-WAKE-UP-BIT-POSITION", "CONTROL-ENABLE-MASK-BITS", "REPORTABLE-SECURITY-EVENT-REFS", "TARGET-PHM-CHECKPOINT-REF", "COUNT-PROFILE", "TCP-SLOW-START-ENABLED", "SERVICE-INSTANCE-COLLECTION-SET", "MAX-TESTER-CONNECTIONS", "ENUMERATOR-VALUE", "CONFIGURATION-CLASS-AFFECTION", "PLCA-TRANSMIT-OPPORTUNITY-TIMER", "SECURE-COM-PROPS-FOR-TCP-REF", "KEY-EXCHANGE-REF", "COUNTER-JUMP-UP", "CLASS-CONTENTS", "CAPABILITY", "SECURED-I-PDU", "DIAGNOSTIC-SERVICE-TABLE-REF", "SECURITY-EVENT-REFS", "PLATFORM-PHM-ACTION-ITEM", "MAP", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE", "NOTIFIER", "CONFIG-VARIANT", "MAX-SLOTS", "HEADER-LENGTH", "INHIBITION-SOURCE-REF", "EGRESS-PORT-REF", "SW-RECORD-LAYOUT-GROUP-FROM", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "CONSUMED-DDS-SERVICE-EVENTS", "IS-DYNAMIC-LENGTH-FIELD-SIZE", "BROADCAST", "SW-REFRESH-TIMING", "DID-NUMBER", "MODE-IN-SWC-INSTANCE-REF", "PERSISTENCY-REDUNDANCY-HASH", "SWITCH-STREAM-GATE-ENTRY", "INTERFACE-REF", "E-2-E-CRC-ERROR-REF", "ECUC-VALIDATION-CONDITION", "TARGET-DATA-ELEMENT-REF", "TLV-DATA-ID-0-REF", "DIAGNOSTIC-SOVD-METHOD", "PCM-BIT-DEPTH", "TD-PDU-TRIGGERING-FILTER-REFS", "ARRAY-SIZE", "CAN-NM-TX-CAN-ID", "OBD-CONTROL-SERVICE-NEEDS", "CONTROL-CONSUMED-EVENT-GROUP-REFS", "LOCAL-ID", "CAN-ID-TO-CAN-ID-MAPPINGS", "REDUNDANCY-HANDLINGS", "FUNCTION-GROUP-PORT-MAPPING", "RESET-CONFIRMED-BIT-ON-OVERFLOW", "REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF", "INSTANTIATION-DATA-DEF-PROPS", "SERVICE-REQUEST-CALLBACK-TYPE", "CHECKPOINT-ID", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "NM-IMMEDIATE-RESTART-ENABLED", "TARGET-RTE-EVENT-REF", "EXPRESSION-REF", "ENTRY-REF", "TIME-BASE-JITTER", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "APMC-STRING-PARAM-DEF", "REQUEST-TOPIC-NAME", "NM-SCHEDULE-VARIANT", "NM-CONFIG", "FM-FEATURE-MAP", "PROCESS-STATE-IREF", "I-PDU-PORT-REFS", "RPT-EXECUTABLE-ENTITY-EVENTS", "SEC-OC-CRYPTO-SERVICE-MAPPING-REF", "MAX-MINOR-VERSION", "TD-EVENT-VFB-PORT-REFS", "DERIVED-PORT-INTERFACE-REF", "COUPLING-PORT-REF", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "TIME-SEG-1", "I-PDU-PORT", "FILTER", "FIRE-AND-FORGET-MAPPING", "MODE-INSTANCE", "ACF-PARTS", "SYNCHRONIZED-MASTER-TIME-BASE", "BUS-IDLE-TIMEOUT-PERIOD", "RECOMMENDED-CONFIGURATION-REF", "EVENT-PROPSS", "TARGET-CATEGORY", "OPERATION-IREF", "CRYPTO-KEY-SLOT", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "CAN-XL-BAUDRATE", "SHARED", "MODE-DECLARATION-GROUP-PROTOTYPE-IREF", "DIAGNOSTIC-ADDRESS", "X", "SYNC-PERIOD", "APPLICATION-VALUE-SPECIFICATION", "DRIVER-NOTIFICATIONS", "CP-SOFTWARE-CLUSTER-REF-CONDITIONAL", "DIAGNOSTIC-IO-CONTROL-CLASS", "FLEXRAY-NM-ECU", "NOTIFICATION-PORT-IREF", "COMPLEX-TYPE-MAPPING", "TRACE-SWITCH-CONFIGURATION", "DLT-LOG-SINK-REF", "PERSISTENCY-KEY-VALUE-DATABASE", "RATIO-KIND", "FREEZE-FRAMES", "BSW-INTERRUPT-ENTITY", "ECUC-IMPLEMENTATION-CONFIGURATION-CLASS", "CONTEXT-COMPONENT-REF", "TARGET-APPLICATION-RECORD-ELEMENT-REF", "INITIAL-REPETITIONS-MAX", "HANDLE-OUT-OF-RANGE", "BULK-NV-DATA-DESCRIPTOR", "IUMPR-REF", "STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM", "REQUIRED-INTERFACE-TREF", "CONSUMED-DDS-OPERATIONS", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "RPT-CONTAINER", "IUMPR-REFS", "HANDLE-DATA-STATUS", "PDU-TRIGGERING-REF", "SENDER-RECEIVER-INTERFACE", "LIN-MASTER-VARIANTS", "DIAGNOSTIC-AUTHENTICATION-REF", "DATA-PROVIDER", "SW-VARIABLE-IMPL", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "CONTROL-PNC-REFS", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "INSTANCE-IDENTIFIER", "MACRO-PER-CYCLE", "IIDC-T-CODE", "HEADER-FILE", "LIN-CLUSTER", "NM-PDU-RX-INDICATION-ENABLED", "REMOTE-PORT-RANGE-END", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "BEST-CASE-EXECUTION-TIME", "USE-SECURED-PDU-HEADER", "TIME-SYNC-R-PORT-PROTOTYPE-IREF", "ALLOW-PLAIN-TEXT-IMPORT", "SERVER-SERVICE-MAJOR-VERSION", "DELEGATION-SW-CONNECTOR", "RATE-DEVIATION-MEASUREMENT-DURATION", "SEQUENTIAL-INSTRUCTIONS-BEGIN", "ROLLOUT-QUALIFICATIONS", "AR-PACKAGE", "USES-MONITOR-DATA", "SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING", "CAN-FRAME-TRIGGERING", "SD", "ACCESS-COUNT-SETS", "CRYPTO-CERTIFICATE-GROUP-TO-PORT-PROTOTYPE-MAPPING", "EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD", "IP-V-6-PATH-MTU-TIMEOUT", "ARBITRARY-EVENT-TRIGGERING", "OUT-OF-RANGE-BITFIELD-ERRORS-ENABLED", "JITTER", "BSW-INTERNAL-TRIGGERING-POINT", "DTC-SETTING-PARAMETER", "SOMEIP-EVENT-DEPLOYMENT", "PNC-NM-REQUEST", "DLT-ECU", "CONTEXT-ATOMIC-COMPONENT-REF", "I-PDU-MAPPINGS", "GROUP-REF", "CLEAR-CONDITION-GROUP-REF", "MINIMUM-EXECUTION-TIME", "DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE", "J-1939-CONTROLLER-APPLICATION", "MAX-NUMBER-OF-RESP-PENDING-FRAMES", "CALC-RAM-BLOCK-CRC", "AUTHENTIC-CONNECTION-PROPSS", "MACHINE-DESIGN-REF", "DDS-TOPIC-REF", "TCP-IP-NDP-MIN-RANDOM-FACTOR", "N-PDU-REFS", "AUTOMATIC-END", "BLUEPRINT-MAPPING-SET", "SUPERVISION-CYCLE", "MODE", "IIDC-TAG", "PROVIDED-ENTRYS", "TTCAN-COMMUNICATION-CONTROLLER", "PORT-BLUEPRINT", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING", "ETH-TP-CONNECTION", "TIME-SYNC-MODULE-INSTANTIATION", "GETTER-CALL-SIGNAL-REF", "SW-MAX-TEXT-SIZE", "LOCAL-SUPERVISION-REFS", "FM-FEATURE-MAP-ASSERTION", "SEC-OC-COM-PROPS-FOR-MULTICAST-REF", "LONG-NAME", "PROTOCOL-VERSION", "DYNAMIC-PART-ALTERNATIVES", "REPORTING-PORT-PROTOTYPE-IREF", "TRANSMISSION-MODE-FALSE-TIMING", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "CONTROL-PROVIDED-EVENT-GROUP-REFS", "RPT-HOOK-ACCESS", "REMOTE-SENDER-QUEUE-LENGTH-CONTRIBUTION", "MAX-VALUE", "NM-USER-DATA-ENABLED", "TCP-CONGESTION-AVOIDANCE-ENABLED", "FLOAT", "SOVD-VALIDATION-ORDER-REF", "UDP-MIN-TX-BUFFER-SIZE", "TYPE-REFERENCE", "IPV-4-RULE", "INTERFACE-MAPPING", "BUS-ID", "MAX-FC-WAIT", "RPT-EXECUTION-CONTEXT", "PIN-NUMBER", "UNIT", "DLT-ARGUMENT-PROPSS", "IO-CONTROL-CLASS-REF", "AUTHENTICATION-VERIFY-ATTEMPTS", "DIAG-REQUIREMENT", "AUTO-COLLECT", "KEY-ID", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "STANDARD-REVISIONS", "SW-DATA-DEF-PROPS", "END-TO-END-PROTECTION-SET", "RTE-EVENT-IREFS", "GLOBAL-ELEMENTS", "ACTIVATION-POINTS", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE", "SEGMENT-BYTE-ORDER", "WRITING-STRATEGYS", "SHARED-AXIS-TYPE-REF", "PROVIDED-DDS-EVENT-QOS-PROPS", "RUNNABLE-ENTITY-GROUP", "UNIT-REFS", "MODIFICATIONS", "MASTER-EVENT-REF", "MESSAGE-TYPE", "META-DATA-FIELDS", "FRAGMENT", "AUTOSAR-VARIABLE-INSTANCE", "DIAGNOSTIC-DO-IP-POWER-MODE-PORT-MAPPING", "INHIBIT-SOURCES", "CAN-CONTROLLER-FD-ATTRIBUTES", "TYPE-OF-DTC-SUPPORTED", "NOTIFICATION-TIME", "POST-BUILD-VARIANT-CRITERION-VALUE", "DISTINGUISHED-PARTITIONS", "DIAGNOSTIC-PERIODIC-RATE", "SOVD-OPERATION-REF", "SUPERVISED-ENTITY-NEEDS", "COUNTER-JUMP-UP-VALUE", "RELATIVE", "NEW-NAD", "CERT-COMMON-NAME", "EXECUTABLE-PROVIDED-PORT-IREF", "MINISLOT-ACTION-POINT-OFFSET", "J-1939-SHARED-ADDRESS-CLUSTERS", "TP-ADDRESSS", "TRANSFORMER-HARD-ERROR-EVENT", "PNC-PDUR-GROUP-REFS", "TRANSFER-PROPERTY", "NM-USER-DATA-OFFSET", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "USE-SMART-SENSOR-API", "COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL", "TLS-CIPHER-SUITE", "TCP-TP-CONFIG", "TCP-MAX-RTX", "SCALE-CONSTRS", "DDS-EVENT-REF", "FLEXRAY-FIFO-RANGE", "ETHERNET-PHYSICAL-CHANNEL", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "AUTH-DATA-FRESHNESS-START-POSITION", "CIPHER-SUITE-CONFIGS", "FORMAL-BLUEPRINT-GENERATOR", "DIRECTION", "VALUE-PAIRS", "DDS-CP-I-SIGNAL-TO-DDS-TOPIC-MAPPING", "TIMING-EVENT", "RUN-MODE", "CUSTOM-SDG-DEF-REFS", "NETWORK-MASK", "CHECK-PER-SOURCE-ID", "FIREWALL-RULE-REF", "REQUIRED-PACKAGE-ELEMENT-REFS", "ON-FAIL-PERMISSIVE-MODE", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS", "UDP-REMOTE-PORT", "NUMERICAL-VALUE-SPECIFICATION", "CURRENT-EXP", "V-2-X-SUPPORTED", "SYNC-INTERVAL", "NATIVE-DECLARATION", "AUTOSAR-VARIABLE-IREF", "NM-CONTROL-BIT-VECTOR-ACTIVE", "TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT", "ORIGIN", "RX-NM-PDU-REFS", "TOPIC-NAME", "HW-PIN", "APPLICATION-ENTRY", "DIAGNOSTIC-EVENT-REFS", "CAN-COMMUNICATION-CONTROLLER", "ECUC-ENUMERATION-PARAM-DEF", "MESSAGE-COMPLIANCE", "CAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "NESTED-ELEMENT-REF", "TARGET-COMPONENT-REF", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-PORT-MAPPING", "REPRESENTS-VIN", "REST-ENDPOINT-PUT", "MAX-NO-NEW-OR-REPEATED-DATA", "EVENT-WINDOWS", "TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED", "RUNS-INSIDE-EXCLUSIVE-AREA-REFS", "BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING", "RPT-EXECUTABLE-ENTITYS", "FIRST-MODE-GROUP-REF", "SUPERVISED-ENTITY-IREF", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "CRF-TYPE", "N-PDU", "CONTEXT-R-PORT-REF", "DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL", "HEALTH-CHANNEL-IREF", "TRIGGER-DIRECT-IMPLEMENTATIONS", "SDU-REFS", "DATA-ELEMENTS", "ALLOWED-I-PV-6-EXT-HEADERS-REF", "CUSTOM-SERVICE-ID", "SOMEIP-REQUIRED-EVENT-GROUP", "TEX-MATH", "DEPLOYMENT-URIS", "RUNNABLE-ENTITY-IREFS", "BACKGROUND-EVENT", "FIRE-AND-FORGET-METHOD-MAPPINGS", "SW-CODE-SYNTAX", "PHYSICAL-CHANNEL-REF-CONDITIONAL", "VARIANT-CRITERION-REF", "NV-BLOCK-DATA-MAPPINGS", "WRITE-ONLY-ONCE", "LOGICAL-EXPRESSION-ARGUMENT-REF", "SWC-TO-SWC-OPERATION-ARGUMENTS", "ENVIRONMENT-DATA-CAPTURE", "ASSIGNMENT-PRIORITY", "NM-READY-SLEEP-COUNT", "OWNERSHIP", "BSW-DIRECT-CALL-POINT", "NONQUEUED-RECEIVER-COM-SPEC", "PHM-HEALTH-CHANNEL-STATUS", "CONSTANT-MAPPING-REFS", "DATA-ID", "DEPENDENCY", "COMPU-METHOD", "ETH-TCP-IP-ICMP-PROPS", "AFFECTED-REFS", "SOFTWARE-CLUSTER-REQUIREMENT", "UPPER-MULTIPLICITY-IS-INFINITE", "LATENCY-BUDGET", "ACTION-LIST-EXECUTION", "TD-EVENT-COMPLEX", "EVENT-TOPIC-ACCESS-RULE-REF", "DESIGN-REF", "K-INIT-RESOURCE", "IDSM-TRAFFIC-LIMITATION", "ALLOWED-TCP-OPTIONS", "TARGET-DATA-PROTOTYPE-IN-SR-REF", "CRYPTO-CERTIFICATES", "DURABILITY-SERVICE-CLEANUP-DELAY", "SDG-CLASS-REF", "VLAN-MEMBERSHIP", "USE-VARIABLE-SA", "NM-INTERACTS-WITH-SM-MAPPING", "INSTANCE-ID-PROPS", "FREEZE-FRAME-CONTENT-REF", "UNICAST-CREDENTIALS", "OPTION-KIND", "SWITCH-STREAM-IDENTIFICATIONS", "MASTER-REF", "ALLOCATE-SHADOW-COPY", "PORTS", "MAXIMUM-SEGMENT-LENGTH-RESPONSE", "CLIENT-SERVER-ANNOTATIONS", "LIN-TP-CONFIG", "OFFSET-TIMING-CONSTRAINT", "SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS", "SIZE-OF-ARRAY-LENGTH-FIELD", "DATA-CONSTR-RULES", "SECURITY-EVENT-DEFINITION-REF", "CHECKPOINT-FINAL-REF", "K-RUN-RESOURCE", "SHORT-LABEL-PATTERN", "COMPOSITE-SERVICE-INTERFACE-REF", "MKA-PARTICIPANT-REFS", "FLEXRAY-TP-CONNECTION-CONTROL", "UCM-MASTER-FALLBACK-REFS", "LOGIC-ADDRESSS", "REMOTE-CERTIFICATE-REF", "SIGNAL-BASED-FIELD-DEPLOYMENT", "DDS-QOS-PROFILE-REF", "MAY-BE-UNCONNECTED", "NV-DATA-PORT-ANNOTATIONS", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "SOURCE-TRIGGER-REF", "MASTER", "DLT-MESSAGE-REF", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS", "SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST", "CAN-XL-FRAME-TRIGGERING-PROPS", "PNC-MAPPING-REFS", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "SELECTOR-FIELD-CODE", "RECOVERY-NOTIFICATION-TIMEOUT", "CLEAR-CONDITION-REF", "APMC-REVISION-LABEL-PARAM-VALUE", "ECUC-ENUMERATION-LITERAL-DEF", "SERVICE-POINT-SYMBOL-PRE", "SERVICE-INTERFACE-PEDIGREE", "HEALTH-CHANNEL-SUPERVISION", "MC-SW-EMULATION-METHOD-SUPPORT", "WINDOW-SIZE-INVALID", "SECOND-DATA-PROTOTYPE-REF", "LIN-ORDERED-CONFIGURABLE-FRAMES", "BAUDRATE", "INITIAL-EVENT-STATUS", "ECU-REF", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "PDU-TO-FRAME-MAPPINGS", "SW-TEXT-PROPS", "REQUIRED-COM-SPECS", "SIGNAL-BASED-METHOD-DEPLOYMENT", "RESULTING-RPT-SW-PROTOTYPING-ACCESS", "PARAMETER-PORT-ANNOTATIONS", "CONTEXT-TYPE-LISTS", "TRIGGERS-RECOVERY-NOTIFICATION", "REQUESTER-IREF", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "IDSM-INSTANCE", "CLIENT-SERVER-OPERATION-MAPPING", "CALL-SIGNAL-REF", "STATE-MANAGEMENT-EM-ERROR-INTERFACE", "FLOAT-VALUE-VARIATION-POINT", "REMOTE-PORT-REF", "V-LAN-REF", "INDICATOR-FAILURE-CYCLE-COUNTER-THRESHOLD", "SERVICE-DEPENDENCYS", "DATA-PROTOTYPE-IN-PORT-INTERFACE-REF", "VERIFY-UPDATE", "SOCKET-ADDRESS", "DDS-RPC-SERVICE-DEPLOYMENT", "GETS", "INCLUDED-VARIANT-REF", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "RPT-PROFILES", "IP-TP-RULE", "SYNC-LOSS-TIMEOUT", "TLV-DATA-ID-DEFINITIONS", "SIGNATURE-SUPPORT-CP", "INSTALLATION-BEHAVIOR", "START-ACTION-REF", "ENDPOINTS", "BOR-COUNTER-L-1-TO-L-2", "REVISION-LABELS", "SW-RECORD-LAYOUT-V", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "PORT-API-OPTION", "PROVIDED-PORT-PROTOTYPE-IREF", "OBD-DTC-VALUE", "ELEMENT", "CAN-CONTROLLER-XL-ATTRIBUTES", "TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF", "DOMAIN", "PHM-CHECKPOINT", "SOMEIP-TRANSFORMATION-PROPS", "FIBEX-ELEMENTS", "ROUTINE-SUBFUNCTION-REF", "PORT-API-OPTIONS", "MIN-NUMBER-OF-ELEMENTS", "BINARY-MANIFEST-ITEM-NUMERICAL-VALUE", "DIAGNOSTIC-RESPONSE-ON-EVENT", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "VARIABLE-ACCESS-IREF", "BUILD-ACTION", "LOCAL-VARIABLE-REF", "STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE", "ALLOCATOR", "SUB-DOMAIN-REF", "REPLAY-PROTECTION", "DEPENDENT-RESOURCES", "HW-ELEMENT-REFS", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED", "COLLECTION-THRESHOLD", "NETWORK-REPRESENTATIONS", "MULTIPLICITY-RESTRICTION", "TEXT-VALUE-SPECIFICATION", "DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED", "LIN-SLAVES", "ASSEMBLY-SW-CONNECTOR", "DIAGNOSTIC-CONNECTION-REF-CONDITIONAL", "PRM-CHAR", "PNC-PDUR-GROUP-REF", "DIFFERENTIATED-SERVICE-CODE-POINT", "NM-COORDINATOR", "RETURN-SIGNAL-TRIGGERING-REF", "BYPASS-ETHER-TYPES", "PNC-MAPPING", "TRIGGER-I-PDU-SEND-CONDITION", "REQUIRED-DATAS", "DEBOUNCE-BEHAVIOR", "AGGREGATION-REF", "RESET-PENDING-BIT-ON-OVERFLOW", "E-2-E-PROFILE-CONFIGURATION", "SAFE-TRANSLATION", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "SOURCE-IP-ADDRESS", "DIAGNOSTIC-CONTROL-NEEDS", "TRIGGER-REFS", "TAG-ID", "INTRODUCTION", "STATE-MANAGEMENT-MODULE-INSTANTIATION", "STATE-MANAGEMENT-REQUEST-RULE", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "RPT-READ-ACCESS", "FIREWALL-STATE-MACHINES", "NM-GLOBAL-COORDINATOR-TIME", "ENTRYS", "OPERATION-MAPPINGS", "SOFTWARE-MEMORY-SECTION-REF", "IPV-6-RULE", "WAKE-UP-CHANNEL", "OVERRIDE-ID", "ECU-SCOPE-REF", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "MODE-SWITCH-RECEIVER-COM-SPEC", "DIAGNOSTIC-DATA-ELEMENT", "RELATIONS", "TCP-IP-DHCP-V-6-SOL-DELAY-MAX", "DIAGNOSTIC-CONNECTED-INDICATOR", "FR-GLOBAL-TIME-DOMAIN-PROPS", "RESTRICT-UPDATE", "DIAGNOSTIC-TEST-RESULT", "PNC-WAKEUP-DATA-MASK", "PARTICIPATING-J-1939-CLUSTER-REF", "MINIMUM-SEND-INTERVAL", "ETH-TCP-IP-PROPS", "DIAGNOSTIC-ENV-SOVD-DATA-CONDITION", "INVERSE-PROTOCOL-VERSION", "FLAT-INSTANCE-DESCRIPTOR", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "IEE-E-1722-TP-CONNECTION-REF", "RULE", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "PAYLOAD-TYPE", "CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF", "DATA-TYPE-MAPPING-REFS", "SOURCE-SERVICE-INTERFACE-REF", "ARTIFACT-DESCRIPTORS", "TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE", "GENERIC-MODULE-INSTANTIATION", "CALL-SIGNAL-TRIGGERING-REF", "EVENT-DEFAULT-VALUE", "RECORD-NUMBER", "ECUC-LINKER-SYMBOL-DEF-VARIANTS", "TCP-IP-NDP-DESTINATION-CACHE-SIZE", "PAYLOAD-LENGTH-STATIC", "CONTEXT-DATAS", "NETWORK-ID", "TIME-LEAP-HEALING-COUNTER", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "SUBSCRIBE-EVENTGROUP-RETRY-DELAY", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "CRYPTO-KEY-SLOT-REF", "DIAGNOSTIC-DATA-IDENTIFIER-REF", "IDSM-CONTEXT-PROVIDER-MAPPING", "NM-REPEAT-MSG-INDICATION-ENABLED", "IP-V-6-PATH-MTU-ENABLED", "TX-ENABLE-WINDOW-LENGTH", "ACL-OPERATION-REF", "MULTICAST-CONNECTOR-REF", "ECUC-CONTAINER-VALUE", "PROPAGATION-DELAY", "VEHICLE-PACKAGE", "AFFECTED-BSW-MODULE", "UPPER-BOUNDARY", "CHANNEL-ID", "IDSM-INSTANCE-ID", "MC-DATA-ACCESS-DETAILS", "POSSIBLE-AP-ERROR-REFS", "PRIVACY-LEVEL", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING", "PERSISTENCY-DATA-REQUIRED-COM-SPEC", "LINKERS", "MANDATORY", "ACCEPTS-ENCODINGS", "MODE-MANAGER-ERROR-BEHAVIOR", "STD-REFS", "STATE-MANAGEMENT-REQUEST-TRIGGER", "CUSTOM-DOCUMENTATION-REF", "MIN-SERVICE-POINT-ID", "SENDING-NODE-REF", "RESPONSE-ON-EVENT-ACTION", "MAX", "CRYPTO-SIGNATURE-SCHEME", "SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "HEALTH-CHANNEL-EXTERNAL-MODE", "CLIENT-SERVER-OPERATION-IREF", "SW-MIN-AXIS-POINTS", "TIMING-CLOCK-SYNC-ACCURACY", "DIAGNOSTIC-SERVICE-INSTANCES", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "SYNCHRONIZED-MODE-GROUPS", "TD-EVENT-FRAME", "DIAGNOSTIC-UPLOAD-INTERFACE", "DETECT-NIT-ERROR", "FIELD", "DETERMINISTIC-CLIENT", "WRITTEN-LOCAL-VARIABLES", "CONTEXT-ELEMENT-REFS", "NUMERICAL-VALUE", "SYMBOL-WINDOW", "PERMIT-MULTIPLE-REFERENCES-TO-EE", "PRE-SHARED-KEY-REF", "ROOT-ARGUMENT-DATA-PROTOTYPE-REF", "TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED", "APPL-INIT-VALUE", "APMC-CONTAINER-REFERENCE-VALUE", "OBD-SUPPORT", "OBJECT-REF", "VARIABLE-INSTANCE", "HW-PORT-MAPPING", "CONSTANT-REF", "IS-AUTO-VALUE", "DATA-VALUE-ERROR-REF", "SECOND-VALUE", "PORT-PROTOTYPE-REF", "COM-CONFIGURATION-ID", "FLAT-MAP-ENTRY-REFS", "PULSE-TEST", "CRC-SEQUENCE-ID", "SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "MODE-PORT-ANNOTATIONS", "P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "SOFTWARE-CLUSTER-SOVD-ADDRESS", "ETS-AVAILABLE-BANDWIDTH-IN-WEIGHT-VALUE", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "MAX-DESTINATION-PORT-NUMBER", "BITFIELD-TEXT-TABLE-MASK-FIRST", "HW-TYPE-REF", "RETURN-NO-NEW-DATA-ENABLED", "POST-BUILD-VARIANT-CONDITION", "DIAGNOSTIC-CONDITION-REF", "MEMORY-ID", "CONTROL-PROVIDED-EVENT-GROUP-REF", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "GROUP-ID", "AUTO-START", "IPV-4-ADDRESS", "DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING", "SW-AXIS-CONTS", "SYNC-DATA-ID-LIST", "LOWER-MULTIPLICITY", "IP-ADDRESS-PREFIX-LENGTH", "HW-ELEMENT-REF-CONDITIONAL", "LATEST-TX", "IGNORE-ORDER-ALLOWED", "SUPERVISION-REF", "UPDATE-KIND", "MODULE-INSTANTIATION-REFS", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "REFERENCE-REF", "PARAMETER-ID", "LONG-NAME-1", "PERIODIC-RESPONSE-UUDT-REFS", "INSTANCE-IN-MEMORY", "SEC-OC-JOB-REQUIREMENT-REF", "MAX-COMM-MODE", "FIRST-PHYSICAL-DIMENSION-REF", "I-PDU-IDENTIFIER-UDP-REFS", "BYTE-ORDER", "ALL-CHANNELS-REF", "CS-ERROR-REACTION", "PERSISTENCY-REDUNDANCY-CRC", "IMPOSITION-TIMES", "FINAL-CHECKPOINT-REFS", "IDENTIFIABLE-REF", "VALIDATION-REF", "TP-ADDRESS", "DLT-LOG-SINK", "TEST-ID-REF", "RELATIVE-TOLERANCE", "TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "GLOBAL-TIME-GATEWAY", "CAN-FRAME-RX-BEHAVIOR", "STORE-ON-CHANGE", "ARTIFACT-CHECKSUM-REF", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "CLIENT-PORT-FROM-CONNECTION-REQUEST", "INDICATOR-REF", "FAILURE-MONITORING-REF", "FUNCTION-PROTOTYPE-EMITTER", "DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL", "OPERATION-ARGUMENT-INSTANCE-IREF", "FLEXRAY-COMMUNICATION-CONNECTOR", "SOMEIP-SERVICE-VERSION", "REST-INTEGER-PROPERTY-DEF", "REMOTE-UNICAST-CONFIG-REF", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "QOS-PROFILE", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "ROUTINE-REF", "BSW-MODULE-DEPENDENCY", "CONTAINED-PACKAGE-ELEMENT-REF", "REVISION-LABEL", "SOVD-GATEWAY-INSTANTIATION", "RAM-BLOCK", "EXTERNAL-SYNC", "ECUC-LINKER-SYMBOL-DEF", "TIMEOUT-AS", "SD-CLIENT-CONFIG-REF", "IKE-OVER-TIME", "COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS", "COUPLING-PORT-CONNECTIONS", "UDS-DTC-VALUE", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "REST-NUMBER-PROPERTY-DEF", "IN-PLACE", "USER-REF", "DO-IP-TP-CONFIG", "HW-PIN-GROUP-CONNECTIONS", "PORT-GROUPS", "AAF-FORMAT", "CONFIDENCE-INTERVALS", "ASSOCIATED-RTE-PLUGIN-REF", "OFS-DATA-ID-LIST", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REF", "ASSIGN-NAD", "GLOBAL-TIME-PDU-REF", "NM-VARIANT", "FIELD-MAPPING", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "DATA-TYPE-MAPPING-SET", "UNICAST-UDP-CREDENTIALS", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "DIAGNOSTIC-CONTRIBUTION-REFS", "METHOD-TOPICS-ACCESS-RULE-REF", "DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF", "BGCOLOR", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "CONTAINED-AR-ELEMENT-REF", "EVENT-NEEDS-REF", "ECU-TIMING", "PERIODIC-RATES", "SEPARATE-SIGNAL-PATH", "NONQUEUED-SENDER-COM-SPEC", "MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "PROCESSOR", "FIELD-MAPPINGS", "WAKEUP-RX-LOW", "DIAGNOSTIC-INDICATOR-INTERFACE", "LIN-SLAVE-CONFIG", "RPT-SYSTEM-REF", "TCP-GENERAL-INACTIVITY-TIME", "PARTIAL-NETWORK-REF", "NEXT-HIGHER-CERTIFICATE-REF", "XDOC", "BAM-TIMING", "DO-IP-LOGIC-ADDRESS-PROPS", "ECUC-REFERENCE-VALUE", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "SUPPORTING-MATERIAL", "SERVICE-INTERFACE-VERSION", "CONSUMED-FIELDS", "TYPE-APPROVAL", "SEGMENT-POSITIONS", "REQUIRES-INDEX", "BYTE", "CHECK-STATIC-BLOCK-ID", "ECUC-QUERY-STRING-REF", "TCP-IP-IP-REASSEMBLY-BUFFER-COUNT", "POLICY-ACTION", "UDP-PROPS", "MESSAGE-SOURCE-FILE", "NUMBER-OF-MINISLOTS", "PACKAGER-SIGNATURE-REF", "TCP-IP-ICMP-V-4-TTL", "SUBTITLE", "IEEE-1722-TP-RVF-CONNECTION", "SOFTWARE-CLUSTER-UDS-DIAGNOSTIC-ADDRESS", "NM-MESSAGE-TIMEOUT-TIME", "CIPHER-SUITE-PRIORITY", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "SOURCE-REF", "KEY", "BLOCK-STATES", "TARGET-FIELD-REF", "TRANSFORMATION-PROPS-MAPPING-SET-REF", "MODE-ELEMENTS", "COUPLING-FLAG", "IDS-TIME-BASE-REFS", "SYSC-REF", "SOMEIP-EVENT", "NUMBER-OF-PARALLEL-TCP-SESSIONS", "UNASSIGNED-FRAME-TRIGGERING-REF", "CURRENT-DATA-TYPE-REF", "TYPE-TAILORINGS", "BUILD-ACTION-MANIFESTS", "REACTION-CODE", "DIAGNOSTIC-IUMPR-GROUP", "SERVICE-DISCOVERY-CONFIGS", "INVERT-CONDITION", "STATUSS", "FRAME-PORT-REF", "MSR-QUERY-TOPIC-1", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "SW-DATA-DEPENDENCY-FORMULA", "RULE-BASED-AXIS-CONT", "IP-IAM-REMOTE-SUBJECT", "IEEE-1722-RAW-DATA-STREAM-CONSUMER-MAPPING", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "ALL-NM-MESSAGES-KEEP-AWAKE", "NM-ACTIVE-COORDINATOR", "STATIC-SOCKET-CONNECTIONS", "SECURE-TRANSLATION", "CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING", "USER-DEFINED-CLUSTER", "DLT-SESSION-ID", "NAS-TIMEOUT", "LOG-TRACE-DEFAULT-LOG-THRESHOLD", "DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF", "SECURITY-ACCESS-CLASS-REF", "APMC-CHOICE-CONTAINER-REFERENCE-DEF", "EXPORTABILITY", "FRAME-REF", "TIME-FR-IF", "PORT-BLUEPRINT-INIT-VALUE", "ACTIVE-SUPERVISION-REF", "ROM-LOCATION-REF", "CRC-OFFSET", "METHOD-REQUEST-TOPIC-NAME", "DIAGNOSTIC-J-1939-NODE", "SW-CALPRM-IMPL", "ACL-OPERATION-REFS", "MAXIMUM-RECEIVE-BUFFER-LENGTH", "ECUC-FOREIGN-REFERENCE-DEF", "XREF-TARGET", "FURTHER-ACTION-BYTE-NEEDS", "SW-COMP-TO-ECU-MAPPING-REFS", "MAXIMUM-LENGTH", "LIN-PHYSICAL-CHANNEL", "TRAFFIC-SHAPER-GROUP-REF", "FIELD-SENDER-COM-SPEC", "LOCAL-IP-SEC-RULE-REF", "VEHICLE-SYSTEM-INSTANCE", "LOG-SINK-REF", "EXECUTION-CONTEXT", "IDSM-REPORTING-MODE-PROVIDER-INTERFACE", "INSTANCE-ID", "LOCAL-TCP-PORT-REF", "CAPABILITY-RECORDS", "NETWORK-TIME-SLAVE-REF", "PACKING-BYTE-ORDER", "UCM-MASTER-FALLBACK-REF", "CYCLE-TIME-VALUE", "CONNECTION-IS-MANDATORY", "SUB-SOFTWARE-CLUSTER-REFS", "STORAGE-CONDITIONS", "TLS-SECURE-COM-PROPS-REF", "BSW-RESOLUTION", "ALIVE-CHECK-RESPONSE-TIMEOUT", "IS-PIPELINING-PERMITTED", "USER-DEFINED-METHOD-DEPLOYMENT", "COM-MANAGEMENT-GROUP-REFS", "DDS-PARTITIONS", "IDENTITY-CERTIFICATE-AUTHORITY-REF", "J-1939-TP-CONNECTION", "SW-BIT-REPRESENTATION", "STATE-REQUEST-REF", "USE-AUTH-DATA-FRESHNESS", "NESTED-CONTEXT-DATAS", "IEEE-1722-ACF-BUS-RAW-DATA-STREAM-CONSUMER-MAPPING", "SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "TD-EVENT-FR-CLUSTER-CYCLE-START", "DELTA-PACKAGE-APPLICABLE-VERSION", "MULTICAST-SECURE-COM-PROPS-REF", "ESTIMATED-DURATION-OF-CAMPAIGN", "CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS", "PER-INSTANCE-PARAMETER-REF", "SYSTEM-SIGNAL-GROUP", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "DIAGNOSTIC-SOVD-METHOD-PRIMITIVE", "SUPERVISION-MODE", "CONTEXT-MODE-DECLARATION-GROUP-REF", "PREPARE-ROLLBACK", "TLS-SECURE-COM-PROPS", "HANDLE-OUT-OF-RANGE-STATUS", "PROVIDED-DATA-REF", "RX-NM-PDU-REF", "FULL-DUPLEX-ENABLED", "BSW-MODULE-TIMING", "TARGET-DATA-PROTOTYPE-REF", "ON-FAIL-PERMISSIVE-MODE-TIMEOUT", "DCM-BUS-TYPE", "ACTION-LIST-PROCESSING-FAILED-ERROR-REF", "CAN-XL-PROPS-REF", "HW-ELEMENT-REF", "REFERRABLE-REF", "RETURN-SIGNAL-REF", "TIME-BASE-CONSUMER-REF", "DLT-LOG-CHANNEL-DESIGN", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "ACCEPT-ENCODING", "NV-BLOCK-NEEDS", "IDSM-QUALIFIED-EVENT-RECEIVER-INTERFACE", "SO-CON-I-PDU-IDENTIFIER", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "PERMISSION-TO-CREATE-CHILD-PROCESS", "COM-FIND-SERVICE-GRANT-DESIGN", "SW-SYSTEMCONST-REF", "MINIMUM-SEPARATION-TIME", "PDU-COUNTER-THRESHOLD", "CAN-PHYSICAL-CHANNEL", "I-SIGNAL-REF", "DDS-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "RESPONSE-ERROR-POSITION", "CONTAINED-FIBEX-ELEMENT-REFS", "EXPIRED-SUPERVISION-TOLERANCE", "DISPLAY-FORMAT", "BOR-TIME-L-2", "MEMORY-USAGE", "TEST-ID", "SECOND-KEY-SLOT-ID", "IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "PER-INSTANCE-MEMORYS", "RTPS-PROTECTION-KIND", "INSTANCES", "IMPLEMENTATION-DATA-TYPE-REF", "MODE-SWITCH-POINT", "DNS-SERVER-ADDRESS", "MODE-GROUP-REF", "SECURITY-EVENT-REF", "POSSIBLE-ERROR-REACTIONS", "ECU-EXTRACT-REFERENCE-IREF", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "SYSTEM", "RULE-INIT-STATE", "AUTO-AVAILABLE", "REST-ENDPOINT-DELETE", "REPLACE-WITH", "AH-CIPHER-SUITE-NAMES", "CRYPTO-TRUST-MASTER-INTERFACE", "OPT-ARGUMENT-REF", "ELEMENT-IN-IMPL-DATATYPE", "MAX-SYNC-JUMP-WIDTH", "PRIORITYS", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL", "TCP-KEEP-ALIVE-PROBES-MAX", "J-1939-DCM-DM-19-SUPPORT", "ATTRIBUTES", "SWC-BEHAVIOR-REF", "ERROR-CODE", "NM-MAIN-FUNCTION-PERIOD", "EVENT-READINESS-GROUPS", "MODE-DECLARATION-GROUP-PROTOTYPE-REF", "SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE", "WAKEUP-LOCAL-DURATION-TIME", "MAX-NUMBER-OF-NPDU-PER-CYCLE", "TCP-IP-ICMP-V-6-HOP-LIMIT", "POLICY", "FUNCTION-GROUP-MODE-IREFS", "IDENTITIY-NUMBER", "CONSTANT-MEMORYS", "SERVICE-KIND", "ALLOWED-I-PV-6-EXT-HEADERS", "SOFTWARE-PACKAGE-STEP", "STATUS-IREF", "SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING", "V", "DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER", "SUPERVISION-MODES", "COUPLING-TYPE", "E-2-E-COUNTER-ERROR-REF", "INCLUDED-MODE-DECLARATION-GROUP-SET", "MC-GROUP-DATA-REF-SET-CONDITIONAL", "GENERATED-ARTIFACTS", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "DDS-CP-QOS-PROFILE", "SW-COMPONENT-DOCUMENTATION", "VLAN-ID", "DO-IP-ACTIVATION-LINE-NEEDS", "SWC-MAPPING-CONSTRAINT-REF", "NETWORK-ENDPOINT-ADDRESSES", "CRYPTO-SERVICE-NEEDS", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "GLOBAL-TIME-DOMAIN-PROPERTYS", "IGNORE-AFTER-TX", "NM-MSG-REDUCED-TIME", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "TCP-OPTION-FILTER-SET", "LIN-CLUSTER-VARIANTS", "NM-IF-ECUS", "UCM-NOT-AVAILABLE-ON-THE-NETWORK", "DIAGNOSTIC-EVENT-PORT-MAPPING", "TRANSFER-EXIT-CLASS-REF", "DATA-TYPE-MAPPING-SET-REFS", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "DIAGNOSTIC-TRANSFER-EXIT", "ARBITRATIONS", "AUTHENTICATION-ENABLED", "SO-AD-CONFIG", "BLUEPRINT-POLICY-NOT-MODIFIABLE", "CLIENT-SERVER-INTERFACE", "RESPONSE-REF", "MAC-MULTICAST-GROUPS", "VENDOR-SPECIFIC-MODULE-DEF-REFS", "DATA-FORMAT-TAILORING", "TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY", "LIN-TP-NODE", "FORMULA", "DIAGNOSTIC-ENV-MODE-CONDITION", "IIDC-SY", "OS-TASK-PROXY-REF", "SEQUENCE-OFFSET", "SLAVE-REF", "FUNCTION-GROUP-PHM-STATE-REFERENCE", "CERTIFICATE-EVALUATIONS", "DTC-KIND", "TRANSFORMER-REF", "RX-CYCLE-PROCESS-TIME", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "IP-IAM-AUTHENTIC-CONNECTION-PROPS", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "LOGICAL-OPERATOR", "ADDRESS-CLAIM-ENABLED", "HW-PIN-CONNECTIONS", "PARAMETER-ELEMENT-ACCESS", "SWC-TO-APPLICATION-PARTITION-MAPPINGS", "KEY-SLOTS", "MAPPED-AP-DATA-ELEMENT-IREF", "MEMORY-RANGE-REF", "SEGMENTATION-SUPPORTED", "SCHEDULE-CHANGE-NEXT-TIME-BASE", "DATA-RECEIVED-EVENT", "SW-CLUSTER-MAPPINGS", "TX-PDU-TRIGGERING-REF", "GLOBAL-TIME-CAN-MASTER", "DDS-OPERATION-RESPONSE-TRIGGERING-REF", "TP-PROTOCOL-TYPE", "CONTEXT-DESCRIPTION", "REQUIRED-ARTIFACTS", "PN-RESET-TIME", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "PAYLOADS", "RPT-SERVICE-POINT-PRE-REFS", "DYNAMIC-ACTION-REFS", "L-2", "CALLBACK-HEADER-REF", "MAX-LENGTH", "CPP-IMPLEMENTATION-DATA-TYPE", "FLEXRAY-AR-TP-NODE", "FLEXRAY-AR-TP-CONNECTION", "ECUC-REF", "RAPID-PROTOTYPING-SCENARIO", "CREATE-ECU-WAKEUP-SOURCE", "TARGET-HEALTH-CHANNEL-REF", "TARGET-MODULE-REFS", "IIDC-FRACTION-NUMBER", "FIELD-TOPICS-ACCESS-RULE-REF", "APMC-URI-FOREIGN-REFERENCE-DEF", "FILE-NAME", "IMPLEMENTED-ENTRY-REF", "REMOTE-MULTICAST-CONFIG-REF", "GENERIC-ETHERNET-FRAME", "UDP-TP", "TCP-IP-ICMP-PROPS-REFS", "OPERATION-CYCLE-AUTOMATIC-END", "READ-LOCAL-VARIABLES", "GROUPED-ALIAS-EVENT-REFS", "TROUBLE-CODE-OBD-REF", "STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR", "SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF", "SECOND-TRIGGER-REF", "CONFIG-ELEMENT-DEF-LOCAL-REF", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "RATE-CORRECTION-THRESHOLD", "CAN-NM-CLUSTER-COUPLING", "PHYSICAL-LAYER-TYPE", "INIT-RESOURCE", "ENGINEERING-OBJECTS", "LIFESPAN", "CP-SOFTWARE-CLUSTER-REF", "ACTIVATION-SWITCH", "TYPE-EMITTER", "END-TO-END-TRANSFORMATION-DESCRIPTION", "RESPONSE-ERROR-REF", "ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL", "SUPPLIER-VALIDATION-ORDER-REF", "DATA-FILTER-TYPE", "VT", "INVERT", "AFFECTION-KIND", "PSK-IDENTITY", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "PLCA-MAX-BURST-TIMER", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "SUPPORTED-CONFIG-VARIANTS", "DATA-ID-MODE", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "NM-REPEAT-MESSAGE-BIT-ACTIVE", "ORDERED-MASTER", "EVENT-REF", "MIN-NUMBER-OF-TIME-QUANTA-PER-BIT", "COMPOSITE-RULE-BASED-VALUE-SPECIFICATION", "DOCUMENTATION-CONTENT", "WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST", "TARGET-MODE-DECLARATION-REF", "MAXIMUM-TRANSMISSION-UNIT", "WAKEUP-RX-IDLE", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "MC-GROUP", "EXECUTION-DEPENDENCY", "TCP-PORT-REF", "VF", "RPT-COMPONENT", "TCP-ALIVE-CHECK-RESPONSE-TIMEOUT", "STOP-IREF", "L-4", "TD-EVENT-I-PDU", "PDU-COUNTERS", "TRANSIT-TO-INVALID-EXTENDED", "TIME-BUFFER", "STATIC-REMOTE-MULTICAST-ADDRESSES", "COUPLING-PORT-ENHANCED-TRAFFIC-SHAPER", "IP-SEC-RULES", "VENDOR-SPECIFIC-MODULE-DEF-REF", "SERVICE-ELEMENT-MAPPING-REFS", "I-SIGNAL-I-PDUS", "SA-RAND-TIME", "PROCESS-TO-MACHINE-MAPPING-SET", "MIN-SAMPLE-POINT", "RVF-TOTAL-LINES", "SESSION-HANDLING", "CRC-VALIDATED", "DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER", "NUM-FAILED-SECURITY-ACCESS", "ITEM-DEFINITIONS", "EVENT-ELEMENT-MAPPINGS", "DOCUMENTATION", "SEC-OC-JOB-MAPPING", "APPROVAL-REQUIRED", "APPLICATION-RULE-BASED-VALUE-SPECIFICATION", "DIAGNOSTIC-SOVD-PRIMITIVE-CONTENT-ELEMENT", "APPROVAL-STATUS", "APMC-ENUMERATION-LITERAL-DEF", "UNICAST-NETWORK-ENDPOINT-REFS", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "SELECTOR-FIELD-BYTE-ORDER", "BUS-MIRROR-CAN-ID-RANGE-MAPPING", "SECURITY-EVENT-FILTER-CHAIN", "DIFFERENTIATED-SERVICE-FIELD", "RPT-WRITES", "THEAD", "CONTEXT-LIMITATION-REFS", "DEFAULT-TRAFFIC-CLASS", "STORAGE-CONDITION-GROUP-REF", "ENVIRONMENT-CAPTURE-TO-REPORTING", "REMOTE-ENDPOINT-CONFIGURATION", "DLT-CONTEXT-REF-CONDITIONAL", "MIN-OK-STATE-VALID", "TABLE", "DDS-OPERATION-REQUEST-TRIGGERING-REF", "DEPENDS-ONS", "DATA-MAPPINGS", "FRAME-TRIGGERINGS", "ALGORITHM-SECONDARY-FAMILY", "ERROR-MAPPINGS", "SW-AXIS-TYPE-REF"];

    /// derive an enum entry from an input string using a perfect hash function
    ///
    /// # Errors
    ///
    /// [`ParseElementNameError`]: The input string did not match the name of any enum item
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseElementNameError> {
        #[rustfmt::skip]
        static DISPLACEMENTS: [(u16, u16); 951] = [(3, 3321), (0, 51), (0, 2), (0, 3), (0, 6078), (0, 15), (0, 0), (2, 594), (0, 20), (0, 11), (0, 0), (0, 5), (0, 1), (0, 0), (0, 666), (0, 812), (0, 3108), (0, 20), (0, 71), (0, 117), (0, 1), (0, 3), (0, 874), (0, 891), (0, 24), (0, 117), (0, 2253), (0, 3), (0, 1), (0, 3234), (2, 5614), (0, 2041), (0, 833), (0, 42), (0, 5), (1, 5661), (0, 32), (0, 152), (0, 0), (0, 6), (0, 4), (0, 90), (0, 328), (1, 255), (0, 766), (0, 2064), (1, 4337), (0, 405), (0, 0), (0, 81), (0, 0), (0, 314), (0, 4), (0, 21), (5, 3646), (0, 26), (4, 3668), (24, 3441), (0, 3), (0, 130), (0, 47), (0, 221), (0, 0), (0, 20), (0, 25), (6, 5546), (0, 158), (0, 8), (0, 1052), (0, 2), (0, 63), (0, 2789), (0, 619), (0, 6), (0, 4), (0, 4160), (0, 57), (0, 8), (1, 4986), (0, 54), (0, 0), (0, 0), (0, 5957), (0, 335), (0, 18), (0, 0), (0, 71), (0, 2), (3, 3361), (0, 911), (0, 0), (0, 308), (0, 4070), (0, 21), (4, 5844), (1, 4124), (0, 509), (0, 4967), (0, 298), (0, 1752), (0, 791), (2, 5148), (0, 30), (0, 17), (0, 0), (10, 350), (0, 331), (0, 101), (0, 1), (0, 117), (0, 157), (16, 1429), (0, 34), (0, 0), (0, 19), (0, 1), (0, 1577), (0, 67), (0, 0), (0, 129), (0, 267), (0, 4312), (0, 15), (0, 294), (0, 13), (0, 3547), (0, 338), (1, 2197), (2, 3376), (6, 5784), (0, 0), (0, 868), (24, 4531), (0, 0), (0, 1016), (0, 292), (0, 188), (0, 21), (0, 0), (0, 0), (0, 1), (0, 76), (0, 1742), (0, 981), (1, 4299), (0, 0), (0, 400), (0, 16), (2, 5447), (0, 435), (13, 3036), (0, 1284), (0, 81), (0, 587), (0, 5313), (0, 0), (0, 8), (0, 44), (0, 288), (0, 546), (1, 4617), (4, 44), (0, 498), (0, 705), (6, 6240), (6, 5978), (0, 5898), (0, 1044), (6, 1243), (14, 3066), (3, 418), (2, 5932), (0, 1066), (0, 6), (0, 599), (0, 770), (5, 4754), (4, 293), (0, 1), (0, 1953), (10, 3465), (0, 0), (0, 2879), (0, 28), (0, 65), (23, 1270), (0, 116), (1, 3208), (9, 2026), (0, 17), (0, 21), (0, 4), (0, 45), (2, 4432), (0, 1011), (0, 1684), (5, 2166), (0, 2039), (0, 0), (7, 4505), (0, 61), (15, 2655), (0, 13), (0, 10), (0, 14), (0, 735), (1, 6071), (0, 3), (1, 2955), (0, 116), (0, 11), (0, 454), (0, 0), (0, 1), (0, 2028), (3, 3245), (0, 403), (0, 753), (37, 3446), (0, 316), (0, 9), (0, 14), (0, 1688), (0, 1383), (0, 279), (0, 262), (0, 52), (0, 987), (4, 3883), (0, 184), (0, 7), (0, 171), (0, 92), (0, 4068), (0, 37), (0, 339), (0, 50), (0, 1852), (33, 3034), (0, 3333), (0, 301), (0, 1), (0, 7), (0, 2), (0, 2), (2, 3025), (0, 2067), (16, 3804), (0, 388), (0, 6), (0, 2779), (0, 11), (0, 187), (0, 106), (0, 35), (2, 6560), (1, 757), (0, 3616), (0, 5), (0, 17), (0, 0), (0, 1347), (0, 0), (0, 370), (8, 5965), (13, 1117), (0, 573), (0, 1934), (0, 3), (5, 1315), (0, 1105), (3, 1828), (0, 303), (0, 14), (3, 1440), (0, 7), (22, 4153), (0, 1476), (2, 4042), (0, 101), (0, 25), (0, 280), (0, 1713), (0, 25), (0, 4911), (0, 105), (1, 5343), (0, 1672), (0, 776), (0, 2), (0, 1193), (2, 2157), (0, 25), (0, 18), (0, 1), (0, 2607), (0, 110), (3, 2132), (0, 64), (0, 3), (0, 634), (0, 23), (0, 1480), (0, 124), (0, 0), (22, 807), (0, 2), (0, 2), (0, 106), (3, 3352), (0, 4), (0, 1), (0, 894), (0, 44), (0, 0), (0, 151), (0, 8), (0, 159), (0, 875), (0, 2), (0, 34), (0, 3397), (1, 494), (0, 43), (0, 9), (0, 18), (3, 6264), (0, 665), (0, 426), (0, 1428), (0, 0), (0, 1636), (28, 1560), (0, 28), (0, 8), (0, 2), (0, 246), (0, 3), (26, 2970), (0, 55), (0, 4465), (0, 363), (0, 5), (0, 2363), (0, 4), (0, 72), (0, 22), (0, 1), (0, 3587), (0, 3), (1, 185), (14, 4672), (0, 10), (1, 2222), (0, 6), (0, 230), (0, 281), (25, 3976), (0, 122), (0, 664), (0, 22), (13, 792), (0, 94), (5, 5400), (0, 28), (6, 3386), (0, 25), (1, 143), (0, 77), (0, 875), (0, 1), (0, 248), (0, 229), (0, 165), (0, 807), (0, 3), (16, 6498), (0, 116), (0, 1161), (0, 698), (10, 5121), (22, 3431), (0, 76), (0, 657), (0, 0), (5, 3433), (0, 3188), (0, 149), (0, 4365), (0, 335), (0, 253), (0, 511), (17, 2182), (0, 68), (1, 801), (14, 3259), (0, 95), (5, 2334), (16, 5125), (0, 539), (0, 349), (1, 1659), (0, 381), (0, 837), (0, 38), (0, 3761), (0, 2938), (0, 1), (0, 3491), (0, 2575), (0, 75), (0, 312), (10, 170), (0, 3022), (0, 173), (0, 563), (0, 0), (0, 23), (4, 2313), (0, 1158), (0, 2774), (0, 56), (0, 534), (0, 101), (0, 2509), (0, 16), (0, 1), (0, 182), (0, 4036), (0, 49), (0, 218), (1, 1233), (1, 5475), (0, 4), (37, 4547), (0, 40), (0, 1223), (11, 691), (0, 10), (0, 357), (0, 50), (0, 6335), (2, 6327), (0, 23), (0, 2644), (11, 3105), (0, 38), (0, 71), (0, 1), (2, 360), (0, 3242), (0, 5), (15, 4001), (0, 18), (0, 152), (0, 5555), (0, 4), (0, 206), (0, 0), (0, 4241), (0, 6497), (0, 3669), (0, 317), (0, 413), (0, 78), (6, 4167), (0, 4843), (66, 2365), (0, 2), (0, 14), (4, 3763), (0, 0), (0, 1192), (2, 4406), (8, 4038), (7, 2359), (18, 5400), (39, 2875), (0, 17), (0, 53), (0, 174), (0, 53), (3, 5676), (15, 5480), (14, 2859), (0, 54), (0, 946), (6, 4605), (77, 4712), (0, 73), (0, 23), (0, 269), (0, 67), (43, 4538), (0, 204), (0, 1), (0, 88), (0, 28), (8, 2204), (18, 5979), (0, 91), (0, 1), (1, 744), (0, 643), (0, 4745), (27, 6327), (0, 8), (0, 21), (2, 5301), (36, 1423), (0, 1708), (0, 2), (0, 116), (0, 24), (16, 4943), (0, 41), (0, 1868), (0, 99), (0, 431), (0, 12), (0, 104), (14, 2826), (0, 15), (5, 5317), (2, 6258), (17, 4002), (0, 112), (0, 111), (0, 8), (0, 429), (0, 196), (0, 2474), (0, 5), (0, 39), (15, 1049), (21, 2944), (0, 395), (0, 6180), (0, 464), (0, 6261), (0, 181), (0, 77), (0, 4666), (0, 721), (0, 50), (6, 646), (16, 6371), (0, 5550), (0, 6632), (4, 2805), (21, 6313), (10, 1599), (0, 1), (3, 505), (0, 3941), (0, 20), (97, 100), (51, 1675), (50, 3589), (0, 2153), (0, 22), (0, 803), (0, 94), (1, 4869), (0, 1639), (0, 4), (2, 341), (0, 385), (85, 1011), (11, 2733), (0, 170), (1, 1), (60, 5477), (31, 416), (0, 1206), (0, 30), (0, 0), (0, 37), (0, 1304), (7, 1406), (0, 17), (21, 1906), (0, 19), (133, 2522), (0, 167), (0, 824), (0, 1667), (1, 1375), (2, 1014), (0, 2236), (0, 279), (125, 1768), (0, 13), (0, 1505), (0, 6279), (24, 45), (0, 97), (0, 0), (5, 1654), (1, 5823), (21, 3857), (0, 22), (1, 4435), (0, 1090), (0, 43), (0, 5675), (19, 3007), (0, 1041), (0, 869), (0, 17), (39, 1268), (3, 6233), (0, 29), (0, 183), (0, 150), (0, 530), (0, 819), (0, 182), (0, 605), (26, 3822), (0, 797), (3, 6614), (4, 1196), (10, 4658), (0, 6), (1, 3711), (0, 14), (0, 89), (0, 5716), (2, 4969), (0, 116), (0, 4450), (0, 762), (0, 38), (0, 2), (1, 3965), (0, 1), (0, 2023), (0, 4268), (1, 4438), (0, 242), (5, 1007), (0, 0), (0, 166), (0, 78), (0, 216), (169, 6083), (19, 2542), (0, 2), (0, 17), (28, 3522), (91, 5952), (6, 6066), (2, 1963), (0, 338), (3, 4726), (60, 5771), (0, 6), (64, 1593), (0, 2630), (0, 271), (6, 5747), (0, 5), (0, 635), (0, 0), (0, 2), (12, 4961), (0, 0), (0, 1), (0, 0), (0, 29), (31, 1223), (0, 54), (29, 978), (0, 17), (8, 6253), (0, 539), (3, 1851), (186, 622), (7, 1839), (0, 945), (0, 35), (26, 6584), (1, 4133), (0, 0), (0, 3), (0, 219), (28, 897), (0, 572), (22, 6290), (27, 5331), (0, 258), (0, 2802), (0, 134), (2, 1200), (0, 14), (8, 4109), (0, 0), (0, 384), (0, 162), (3, 4435), (0, 6007), (3, 1080), (0, 384), (0, 75), (0, 49), (0, 4), (63, 2660), (0, 42), (1, 183), (0, 3446), (0, 5), (54, 119), (0, 34), (0, 4029), (27, 3478), (9, 376), (1, 6288), (0, 184), (0, 168), (0, 4588), (0, 574), (0, 183), (0, 94), (0, 1528), (0, 211), (0, 31), (93, 5051), (17, 2061), (0, 752), (31, 3927), (0, 8), (0, 568), (0, 19), (1, 3586), (0, 64), (60, 7), (0, 7), (1, 3), (0, 3), (23, 1507), (0, 523), (51, 1875), (1, 6559), (1, 4645), (0, 1570), (28, 2996), (0, 494), (4, 5467), (0, 1787), (0, 5), (39, 6381), (0, 396), (0, 2255), (4, 2438), (0, 54), (0, 703), (2, 3103), (2, 4627), (20, 2648), (32, 6173), (0, 579), (0, 13), (0, 1464), (373, 3755), (27, 4769), (0, 22), (2, 3363), (237, 218), (2, 1362), (0, 1049), (0, 991), (0, 866), (22, 2291), (0, 16), (0, 18), (12, 4024), (0, 5128), (0, 317), (125, 1562), (75, 5012), (8, 6466), (1, 963), (42, 3326), (0, 47), (0, 14), (1, 4992), (0, 2506), (0, 4907), (0, 5774), (0, 10), (68, 852), (0, 31), (0, 39), (0, 3607), (0, 7), (0, 233), (0, 0), (8, 4874), (0, 0), (0, 689), (0, 194), (11, 295), (0, 44), (3, 3809), (27, 324), (0, 1), (0, 1336), (0, 0), (189, 329), (0, 62), (21, 4155), (0, 13), (0, 169), (49, 6290), (0, 1470), (0, 621), (0, 17), (0, 0), (0, 87), (0, 23), (0, 0), (1, 3369), (0, 3), (0, 30), (0, 178), (0, 694), (23, 2511), (0, 440), (47, 1833), (138, 105), (10, 2410), (0, 6133), (0, 419), (0, 3488), (0, 43), (0, 324), (7, 5321), (2, 1908), (0, 2752), (146, 1466), (25, 3064), (0, 184), (0, 1665), (0, 0), (0, 25), (0, 2124), (0, 52), (51, 2666), (0, 4125), (2, 4373), (576, 4652), (0, 5564), (5, 3982), (0, 1), (128, 6155), (0, 233), (0, 5), (0, 229), (4, 2602), (0, 1011), (0, 20), (1, 3219), (0, 863), (0, 7), (17, 2346), (0, 5683), (2, 2528), (93, 5898), (0, 0), (53, 6171), (191, 3666), (0, 1255), (0, 921), (0, 101), (0, 27), (0, 61), (0, 0), (0, 1330), (0, 3), (0, 154), (1, 4935), (0, 48), (0, 682), (27, 1098), (0, 1), (0, 1302), (0, 563), (0, 28), (3, 1852), (327, 3273), (0, 239), (0, 334), (0, 4835), (0, 0), (0, 11), (0, 690), (449, 3354), (0, 50), (60, 5178), (4, 4920), (10, 1059), (48, 4275), (0, 86), (0, 0), (197, 127), (228, 4999), (0, 284), (0, 2337), (82, 5510), (0, 1929), (0, 0), (705, 45), (5, 473), (0, 479), (0, 33), (49, 4916), (0, 4402), (0, 460), (0, 419), (108, 3621), (31, 4699), (8, 3328), (0, 27), (0, 7), (9, 1454), (2, 5219), (0, 6307), (98, 1872), (0, 5), (0, 6), (69, 3142), (0, 113), (103, 1732), (0, 2386), (146, 57), (0, 12), (0, 2376), (0, 5362), (8, 6372), (6, 2195), (463, 6326), (2, 500), (0, 0), (0, 481), (0, 54), (82, 5675), (5, 1711), (0, 0), (1, 316), (0, 28)];

        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 951) as usize];
        let item_idx = u32::from(d2)
            .wrapping_add(f1.wrapping_mul(u32::from(d1)))
            .wrapping_add(f2) as usize
            % 6652;
        if ElementName::STRING_TABLE[item_idx].as_bytes() != input {
            return Err(ParseElementNameError);
        }
        Ok(unsafe { core::mem::transmute::<u16, Self>(item_idx as u16) })
    }

    /// get the str corresponding to an item
    ///
    /// The returned &str has static lifetime, becasue it is a reference to an entry in a list of constants
    #[must_use]
    pub fn to_str(&self) -> &'static str {
        ElementName::STRING_TABLE[*self as usize]
    }
}

impl core::str::FromStr for ElementName {
    type Err = ParseElementNameError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(input.as_bytes())
    }
}

impl core::fmt::Debug for ElementName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(ElementName::STRING_TABLE[*self as usize])
    }
}

impl core::fmt::Display for ElementName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(ElementName::STRING_TABLE[*self as usize])
    }
}
