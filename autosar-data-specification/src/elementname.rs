use crate::hashfunc;

#[derive(Debug)]
/// The error type `ParseElementNameError` is returned when `from_str()` / `parse()` fails for `ElementName`
pub struct ParseElementNameError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
#[non_exhaustive]
/// Enum of all element names in Autosar
pub enum ElementName {
    /// A-2-L-DISPLAY-TEXT
    A2LDisplayText                                                         = 2019,
    /// AAF-AES-3-DATA-TYPE
    AafAes3DataType                                                        = 5534,
    /// AAF-FORMAT
    AafFormat                                                              = 5633,
    /// AAF-NOMINAL-RATE
    AafNominalRate                                                         = 4734,
    /// ABBR-NAME
    AbbrName                                                               = 2476,
    /// ABS
    Abs                                                                    = 2048,
    /// ABSOLUTE
    Absolute                                                               = 6039,
    /// ABSOLUTE-TOLERANCE
    AbsoluteTolerance                                                      = 739,
    /// ABSOLUTELY-SCHEDULED-TIMINGS
    AbsolutelyScheduledTimings                                             = 5056,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 5817,
    /// ACCEPT-ENCODING
    AcceptEncoding                                                         = 6137,
    /// ACCEPTANCE-FIELD
    AcceptanceField                                                        = 2609,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF
    AcceptedCryptoCipherSuiteWithPskRef                                    = 2857,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS
    AcceptedCryptoCipherSuiteWithPskRefs                                   = 4281,
    /// ACCEPTED-REMOTE-CERTIFICATE-REF
    AcceptedRemoteCertificateRef                                           = 1713,
    /// ACCEPTED-REMOTE-CERTIFICATE-REFS
    AcceptedRemoteCertificateRefs                                          = 3109,
    /// ACCEPTED-STARTUP-RANGE
    AcceptedStartupRange                                                   = 4144,
    /// ACCEPTS-ENCODINGS
    AcceptsEncodings                                                       = 815,
    /// ACCESS-CONTROL
    AccessControl                                                          = 2992,
    /// ACCESS-COUNT
    AccessCount                                                            = 1239,
    /// ACCESS-COUNT-SET
    AccessCountSet                                                         = 2701,
    /// ACCESS-COUNT-SETS
    AccessCountSets                                                        = 4037,
    /// ACCESS-COUNTS
    AccessCounts                                                           = 3514,
    /// ACCESS-DATA-RECORD-SIZE
    AccessDataRecordSize                                                   = 1354,
    /// ACCESS-PERMISSION-REF
    AccessPermissionRef                                                    = 5367,
    /// ACCESS-PERMISSION-VALIDITY
    AccessPermissionValidity                                               = 5596,
    /// ACCESS-POINT-REF
    AccessPointRef                                                         = 1475,
    /// ACCESSED-DATA-PROTOTYPE-IREF
    AccessedDataPrototypeIref                                              = 2126,
    /// ACCESSED-MODE-GROUPS
    AccessedModeGroups                                                     = 1933,
    /// ACCESSED-PARAMETER
    AccessedParameter                                                      = 6025,
    /// ACCESSED-VARIABLE
    AccessedVariable                                                       = 5691,
    /// ACCESSED-VARIABLE-REF
    AccessedVariableRef                                                    = 110,
    /// ACCURACY
    Accuracy                                                               = 3102,
    /// ACCURACY-EXT
    AccuracyExt                                                            = 2188,
    /// ACCURACY-INT
    AccuracyInt                                                            = 3410,
    /// ACF-PARTS
    AcfParts                                                               = 5937,
    /// ACF-TRANSPORTED-BUSS
    AcfTransportedBuss                                                     = 5504,
    /// ACK-REQUEST
    AckRequest                                                             = 3876,
    /// ACK-TYPE
    AckType                                                                = 1013,
    /// ACL-CONTEXT
    AclContext                                                             = 3586,
    /// ACL-CONTEXTS
    AclContexts                                                            = 1432,
    /// ACL-OBJECT-CLASS
    AclObjectClass                                                         = 5846,
    /// ACL-OBJECT-CLASSS
    AclObjectClasss                                                        = 3634,
    /// ACL-OBJECT-REF
    AclObjectRef                                                           = 3523,
    /// ACL-OBJECT-REFS
    AclObjectRefs                                                          = 3827,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 3398,
    /// ACL-OPERATION
    AclOperation                                                           = 630,
    /// ACL-OPERATION-REF
    AclOperationRef                                                        = 2286,
    /// ACL-OPERATION-REFS
    AclOperationRefs                                                       = 6045,
    /// ACL-PERMISSION
    AclPermission                                                          = 2883,
    /// ACL-ROLE
    AclRole                                                                = 769,
    /// ACL-ROLE-REF
    AclRoleRef                                                             = 2908,
    /// ACL-ROLE-REFS
    AclRoleRefs                                                            = 3773,
    /// ACL-SCOPE
    AclScope                                                               = 5698,
    /// ACTION
    Action                                                                 = 6268,
    /// ACTION-ITEM-LISTS
    ActionItemLists                                                        = 5369,
    /// ACTION-ITEM-REF
    ActionItemRef                                                          = 4640,
    /// ACTION-ITEMS
    ActionItems                                                            = 2740,
    /// ACTION-LIST
    ActionList                                                             = 4002,
    /// ACTION-LIST-EXECUTION
    ActionListExecution                                                    = 370,
    /// ACTION-LIST-ITEM-REF
    ActionListItemRef                                                      = 6166,
    /// ACTION-LIST-ITEM-REFS
    ActionListItemRefs                                                     = 792,
    /// ACTION-LISTS
    ActionLists                                                            = 3082,
    /// ACTION-POINT-OFFSET
    ActionPointOffset                                                      = 4450,
    /// ACTION-TYPE
    ActionType                                                             = 1737,
    /// ACTIONS
    Actions                                                                = 5038,
    /// ACTIVATE-AT-START
    ActivateAtStart                                                        = 5863,
    /// ACTIVATION
    Activation                                                             = 3916,
    /// ACTIVATION-ACTION
    ActivationAction                                                       = 97,
    /// ACTIVATION-POINTS
    ActivationPoints                                                       = 2430,
    /// ACTIVATION-REASON-REPRESENTATION-REF
    ActivationReasonRepresentationRef                                      = 3065,
    /// ACTIVATION-REASONS
    ActivationReasons                                                      = 5100,
    /// ACTIVATION-SWITCH
    ActivationSwitch                                                       = 4270,
    /// ACTIVE-SUPERVISION-REF
    ActiveSupervisionRef                                                   = 4558,
    /// ACTIVE-SUPERVISION-REFS
    ActiveSupervisionRefs                                                  = 4344,
    /// ACTUAL-EVENT-REF
    ActualEventRef                                                         = 2349,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 3891,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 2702,
    /// ADAPTIVE-FIREWALL-MODULE-INSTANTIATION
    AdaptiveFirewallModuleInstantiation                                    = 1489,
    /// ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING
    AdaptiveFirewallToPortPrototypeMapping                                 = 4213,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 2761,
    /// ADD-INFO-5
    AddInfo5                                                               = 899,
    /// ADDITIONAL-INFORMATION
    AdditionalInformation                                                  = 4556,
    /// ADDITIONAL-NATIVE-TYPE-QUALIFIER
    AdditionalNativeTypeQualifier                                          = 3190,
    /// ADDRESS
    Address                                                                = 2216,
    /// ADDRESS-CLAIM-ENABLED
    AddressClaimEnabled                                                    = 4419,
    /// ADDRESS-CONFIGURATION-CAPABILITY
    AddressConfigurationCapability                                         = 1019,
    /// ADDRESS-RANGE-LOWER-BOUND
    AddressRangeLowerBound                                                 = 1408,
    /// ADDRESS-RANGE-UPPER-BOUND
    AddressRangeUpperBound                                                 = 5187,
    /// ADDRESS-SEMANTICS
    AddressSemantics                                                       = 2062,
    /// ADDRESSING-FORMAT
    AddressingFormat                                                       = 5709,
    /// ADMIN-DATA
    AdminData                                                              = 2012,
    /// ADMIT-WITHOUT-MESSAGE-ID
    AdmitWithoutMessageId                                                  = 5072,
    /// AES-3-DATA-TYPE-H
    Aes3DataTypeH                                                          = 2420,
    /// AES-3-DATA-TYPE-L
    Aes3DataTypeL                                                          = 2413,
    /// AFFECTED-APPLICATION
    AffectedApplication                                                    = 2643,
    /// AFFECTED-BSW-MODULE
    AffectedBswModule                                                      = 1594,
    /// AFFECTED-FUNCTIONAL-CLUSTER
    AffectedFunctionalCluster                                              = 3530,
    /// AFFECTED-REF
    AffectedRef                                                            = 5310,
    /// AFFECTED-REFS
    AffectedRefs                                                           = 2495,
    /// AFFECTED-STATE-IREF
    AffectedStateIref                                                      = 4457,
    /// AFFECTION-KIND
    AffectionKind                                                          = 3926,
    /// AGE
    Age                                                                    = 4787,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 945,
    /// AGGREGATION
    Aggregation                                                            = 2660,
    /// AGGREGATION-CONDITION
    AggregationCondition                                                   = 2595,
    /// AGGREGATION-REF
    AggregationRef                                                         = 1496,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 3218,
    /// AGING-ALLOWED
    AgingAllowed                                                           = 755,
    /// AGING-CYCLES
    AgingCycles                                                            = 1016,
    /// AGING-REF
    AgingRef                                                               = 3256,
    /// AGING-REQUIRES-TESTED-CYCLE
    AgingRequiresTestedCycle                                               = 3118,
    /// AH-CIPHER-SUITE-NAME
    AhCipherSuiteName                                                      = 5053,
    /// AH-CIPHER-SUITE-NAMES
    AhCipherSuiteNames                                                     = 7,
    /// ALGORITHM-FAMILY
    AlgorithmFamily                                                        = 1528,
    /// ALGORITHM-MODE
    AlgorithmMode                                                          = 514,
    /// ALGORITHM-SECONDARY-FAMILY
    AlgorithmSecondaryFamily                                               = 3245,
    /// ALIAS-EVENT-REF
    AliasEventRef                                                          = 931,
    /// ALIAS-NAME-ASSIGNMENT
    AliasNameAssignment                                                    = 1623,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 5551,
    /// ALIAS-NAMES
    AliasNames                                                             = 3387,
    /// ALIGNMENT
    Alignment                                                              = 3248,
    /// ALIVE-CHECK-RESPONSE-TIMEOUT
    AliveCheckResponseTimeout                                              = 1374,
    /// ALIVE-REFERENCE-CYCLE
    AliveReferenceCycle                                                    = 4434,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 5807,
    /// ALIVE-SUPERVISIONS
    AliveSupervisions                                                      = 6110,
    /// ALIVE-TIMEOUT
    AliveTimeout                                                           = 1493,
    /// ALL-CHANNELS-REF
    AllChannelsRef                                                         = 1007,
    /// ALL-CHANNELS-REFS
    AllChannelsRefs                                                        = 4185,
    /// ALL-NM-MESSAGES-KEEP-AWAKE
    AllNmMessagesKeepAwake                                                 = 1033,
    /// ALL-PHYSICAL-CHANNELS
    AllPhysicalChannels                                                    = 3568,
    /// ALL-PHYSICAL-CHANNELS-REF
    AllPhysicalChannelsRef                                                 = 4642,
    /// ALLOCATE-SHADOW-COPY
    AllocateShadowCopy                                                     = 4230,
    /// ALLOCATOR
    Allocator                                                              = 3932,
    /// ALLOCATOR-REF
    AllocatorRef                                                           = 3229,
    /// ALLOW-CONTENT-TYPE-CHANGE
    AllowContentTypeChange                                                 = 5756,
    /// ALLOW-DYNAMIC-L-SDU-LENGTH
    AllowDynamicLSduLength                                                 = 1240,
    /// ALLOW-HALT-DUE-TO-CLOCK
    AllowHaltDueToClock                                                    = 2656,
    /// ALLOW-MASTER-RATE-CORRECTION
    AllowMasterRateCorrection                                              = 4174,
    /// ALLOW-PASSIVE-TO-ACTIVE
    AllowPassiveToActive                                                   = 2924,
    /// ALLOW-PROVIDER-RATE-CORRECTION
    AllowProviderRateCorrection                                            = 5370,
    /// ALLOW-UNAUTHENTICATED-PARTICIPANTS
    AllowUnauthenticatedParticipants                                       = 4166,
    /// ALLOWED-I-PV-6-EXT-HEADER
    AllowedIPv6ExtHeader                                                   = 6281,
    /// ALLOWED-I-PV-6-EXT-HEADERS
    AllowedIPv6ExtHeaders                                                  = 3220,
    /// ALLOWED-I-PV-6-EXT-HEADERS-REF
    AllowedIPv6ExtHeadersRef                                               = 1553,
    /// ALLOWED-KEYSLOT-USAGE
    AllowedKeyslotUsage                                                    = 3511,
    /// ALLOWED-SERVICE-CONSUMERS
    AllowedServiceConsumers                                                = 3668,
    /// ALLOWED-SERVICE-PROVIDERS
    AllowedServiceProviders                                                = 4820,
    /// ALLOWED-TCP-OPTION
    AllowedTcpOption                                                       = 6238,
    /// ALLOWED-TCP-OPTIONS
    AllowedTcpOptions                                                      = 321,
    /// ALLOWED-TCP-OPTIONS-REF
    AllowedTcpOptionsRef                                                   = 4665,
    /// ALTERNATIVE-NAME
    AlternativeName                                                        = 1771,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 569,
    /// ANNOTATION
    Annotation                                                             = 898,
    /// ANNOTATION-ORIGIN
    AnnotationOrigin                                                       = 108,
    /// ANNOTATION-TEXT
    AnnotationText                                                         = 3179,
    /// ANNOTATIONS
    Annotations                                                            = 3474,
    /// ANONYMOUS
    Anonymous                                                              = 954,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 4947,
    /// AP-APPLICATION-ENDPOINTS
    ApApplicationEndpoints                                                 = 873,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 1171,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 168,
    /// AP-APPLICATION-ERROR-REF
    ApApplicationErrorRef                                                  = 2144,
    /// AP-APPLICATION-ERROR-REFS
    ApApplicationErrorRefs                                                 = 4993,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 5297,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 85,
    /// API-PRINCIPLE
    ApiPrinciple                                                           = 915,
    /// API-SERVICE-PREFIX
    ApiServicePrefix                                                       = 3115,
    /// APP-ID
    AppId                                                                  = 3483,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 3653,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS
    AppOsTaskProxyToEcuTaskProxyMappings                                   = 2040,
    /// APP-TASK-PROXY-REF
    AppTaskProxyRef                                                        = 878,
    /// APPL-CONSTANT-REF
    ApplConstantRef                                                        = 6233,
    /// APPL-INIT-VALUE
    ApplInitValue                                                          = 2366,
    /// APPL-WATCHDOG-LIMIT
    ApplWatchdogLimit                                                      = 1228,
    /// APPLICABILITY-INFO
    ApplicabilityInfo                                                      = 4261,
    /// APPLICABILITY-INFO-SET
    ApplicabilityInfoSet                                                   = 4572,
    /// APPLICABILITY-INFOS
    ApplicabilityInfos                                                     = 5180,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 5509,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 617,
    /// APPLICATION-ARRAY-ELEMENT-REF
    ApplicationArrayElementRef                                             = 2109,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 3361,
    /// APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION
    ApplicationAssocMapElementValueSpecification                           = 1541,
    /// APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION
    ApplicationAssocMapValueSpecification                                  = 3025,
    /// APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF
    ApplicationCompositeDataTypeSubElementRef                              = 5600,
    /// APPLICATION-COMPOSITE-ELEMENT-IREF
    ApplicationCompositeElementIref                                        = 1088,
    /// APPLICATION-CONTEXT-REF
    ApplicationContextRef                                                  = 3707,
    /// APPLICATION-CONTEXT-REFS
    ApplicationContextRefs                                                 = 5193,
    /// APPLICATION-DATA-TYPE-REF
    ApplicationDataTypeRef                                                 = 2713,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 5415,
    /// APPLICATION-DESCRIPTION
    ApplicationDescription                                                 = 1531,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 6189,
    /// APPLICATION-ENDPOINT-REF
    ApplicationEndpointRef                                                 = 1548,
    /// APPLICATION-ENDPOINT-REF-CONDITIONAL
    ApplicationEndpointRefConditional                                      = 5119,
    /// APPLICATION-ENTRY
    ApplicationEntry                                                       = 1556,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 3243,
    /// APPLICATION-ID
    ApplicationId                                                          = 1674,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 1462,
    /// APPLICATION-MODE-IREF
    ApplicationModeIref                                                    = 4928,
    /// APPLICATION-MODE-MACHINE
    ApplicationModeMachine                                                 = 4072,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 5973,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 1373,
    /// APPLICATION-PARTITION-REF
    ApplicationPartitionRef                                                = 4100,
    /// APPLICATION-PARTITION-REFS
    ApplicationPartitionRefs                                               = 1050,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 387,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS
    ApplicationPartitionToEcuPartitionMappings                             = 964,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 1867,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 180,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 5127,
    /// APPLICATION-RECORD-ELEMENT-REF
    ApplicationRecordElementRef                                            = 2637,
    /// APPLICATION-RULE-BASED-VALUE-SPECIFICATION
    ApplicationRuleBasedValueSpecification                                 = 4700,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 4762,
    /// APPLICATION-TYPE-REF
    ApplicationTypeRef                                                     = 1667,
    /// APPLICATION-TYPE-TREF
    ApplicationTypeTref                                                    = 842,
    /// APPLICATION-VALUE-SPECIFICATION
    ApplicationValueSpecification                                          = 5269,
    /// APPLICATIONS
    Applications                                                           = 619,
    /// APPLIED-STANDARD
    AppliedStandard                                                        = 485,
    /// APPLIED-STANDARDS
    AppliedStandards                                                       = 3305,
    /// APPLIES-TO
    AppliesTo                                                              = 1546,
    /// APPLIES-TO-DEPENDENCIES
    AppliesToDependencies                                                  = 838,
    /// APPROVAL-REQUIRED
    ApprovalRequired                                                       = 239,
    /// APPROVAL-STATUS
    ApprovalStatus                                                         = 6075,
    /// AR-PACKAGE
    ArPackage                                                              = 2244,
    /// AR-PACKAGES
    ArPackages                                                             = 2886,
    /// AR-PARAMETER
    ArParameter                                                            = 3476,
    /// AR-RELEASE-VERSION
    ArReleaseVersion                                                       = 4176,
    /// AR-TYPED-PER-INSTANCE-MEMORY-REF
    ArTypedPerInstanceMemoryRef                                            = 699,
    /// AR-TYPED-PER-INSTANCE-MEMORYS
    ArTypedPerInstanceMemorys                                              = 5491,
    /// ARBITRARY-ADDRESS-CAPABLE
    ArbitraryAddressCapable                                                = 1036,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 588,
    /// ARBITRATION
    Arbitration                                                            = 46,
    /// ARBITRATIONS
    Arbitrations                                                           = 5963,
    /// AREA
    Area                                                                   = 93,
    /// ARGUMENT
    Argument                                                               = 5124,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 4657,
    /// ARGUMENT-MAPPINGS
    ArgumentMappings                                                       = 4292,
    /// ARGUMENT-REF
    ArgumentRef                                                            = 3316,
    /// ARGUMENTS
    Arguments                                                              = 1886,
    /// ARGUMENTSS
    Argumentss                                                             = 1942,
    /// ARP-PROPS
    ArpProps                                                               = 1824,
    /// ARRAY-ELEMENT-MAPPINGS
    ArrayElementMappings                                                   = 5209,
    /// ARRAY-ELEMENT-REF
    ArrayElementRef                                                        = 3405,
    /// ARRAY-IMPL-POLICY
    ArrayImplPolicy                                                        = 5453,
    /// ARRAY-SIZE
    ArraySize                                                              = 1137,
    /// ARRAY-SIZE-HANDLING
    ArraySizeHandling                                                      = 3475,
    /// ARRAY-SIZE-SEMANTICS
    ArraySizeSemantics                                                     = 5795,
    /// ARRAY-VALUE-SPECIFICATION
    ArrayValueSpecification                                                = 6124,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 948,
    /// ARTIFACT-CHECKSUM-REF
    ArtifactChecksumRef                                                    = 2751,
    /// ARTIFACT-CHECKSUM-REFS
    ArtifactChecksumRefs                                                   = 2782,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 5536,
    /// ARTIFACT-CHECKSUMS
    ArtifactChecksums                                                      = 1043,
    /// ARTIFACT-DESCRIPTOR
    ArtifactDescriptor                                                     = 3306,
    /// ARTIFACT-DESCRIPTORS
    ArtifactDescriptors                                                    = 2572,
    /// ARTIFACT-LOCATOR
    ArtifactLocator                                                        = 2479,
    /// ARTIFACT-LOCATORS
    ArtifactLocators                                                       = 3874,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 2087,
    /// ASSERTIONS
    Assertions                                                             = 3214,
    /// ASSIGN-FRAME-ID
    AssignFrameId                                                          = 6138,
    /// ASSIGN-FRAME-ID-RANGE
    AssignFrameIdRange                                                     = 711,
    /// ASSIGN-NAD
    AssignNad                                                              = 2781,
    /// ASSIGNED-CONTROLLER-REF
    AssignedControllerRef                                                  = 4573,
    /// ASSIGNED-DATA-TYPES
    AssignedDataTypes                                                      = 3970,
    /// ASSIGNED-DATAS
    AssignedDatas                                                          = 3095,
    /// ASSIGNED-ENTRY-REF
    AssignedEntryRef                                                       = 377,
    /// ASSIGNED-ENTRY-ROLES
    AssignedEntryRoles                                                     = 4882,
    /// ASSIGNED-FRAME-TRIGGERING-REF
    AssignedFrameTriggeringRef                                             = 4369,
    /// ASSIGNED-LIN-SLAVE-CONFIG-REF
    AssignedLinSlaveConfigRef                                              = 6217,
    /// ASSIGNED-PORTS
    AssignedPorts                                                          = 4155,
    /// ASSIGNED-TRAFFIC-CLASS
    AssignedTrafficClass                                                   = 3973,
    /// ASSIGNED-TRAFFIC-CLASSS
    AssignedTrafficClasss                                                  = 1871,
    /// ASSIGNMENT-PRIORITY
    AssignmentPriority                                                     = 428,
    /// ASSOCIATED-COM-I-PDU-GROUP-REF
    AssociatedComIPduGroupRef                                              = 1397,
    /// ASSOCIATED-COM-I-PDU-GROUP-REFS
    AssociatedComIPduGroupRefs                                             = 2985,
    /// ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    AssociatedConsumedProvidedServiceInstanceGroups                        = 2215,
    /// ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF
    AssociatedCrossSwClusterComRtePluginRef                                = 2787,
    /// ASSOCIATED-EVENT-IDENTIFICATION
    AssociatedEventIdentification                                          = 1173,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REF
    AssociatedPdurIPduGroupRef                                             = 5769,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REFS
    AssociatedPdurIPduGroupRefs                                            = 3331,
    /// ASSOCIATED-RTE-PLUGIN-REF
    AssociatedRtePluginRef                                                 = 2128,
    /// ASSUMED-CURRENT-STATE-IREF
    AssumedCurrentStateIref                                                = 5537,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 1822,
    /// ASYNCHRONOUS-SERVER-CALL-POINT-REF
    AsynchronousServerCallPointRef                                         = 501,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 5055,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS
    AsynchronousServerCallResultPoints                                     = 990,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 1989,
    /// ASYNCHRONOUS-TRAFFIC-SHAPER-REF
    AsynchronousTrafficShaperRef                                           = 2316,
    /// ATTRIBUTE-DEFS
    AttributeDefs                                                          = 241,
    /// ATTRIBUTE-NAME
    AttributeName                                                          = 4525,
    /// ATTRIBUTE-REF
    AttributeRef                                                           = 844,
    /// ATTRIBUTE-TAILORINGS
    AttributeTailorings                                                    = 848,
    /// ATTRIBUTE-VALUES
    AttributeValues                                                        = 2959,
    /// ATTRIBUTES
    Attributes                                                             = 850,
    /// AUDIENCE
    Audience                                                               = 4293,
    /// AUDIENCES
    Audiences                                                              = 3629,
    /// AUTH-ALGORITHM
    AuthAlgorithm                                                          = 5577,
    /// AUTH-DATA-FRESHNESS-LENGTH
    AuthDataFreshnessLength                                                = 3221,
    /// AUTH-DATA-FRESHNESS-START-POSITION
    AuthDataFreshnessStartPosition                                         = 43,
    /// AUTH-INFO-TX-LENGTH
    AuthInfoTxLength                                                       = 4907,
    /// AUTH-ROLE-REF
    AuthRoleRef                                                            = 2730,
    /// AUTH-ROLE-REFS
    AuthRoleRefs                                                           = 350,
    /// AUTHENTIC-CONNECTION-PROPSS
    AuthenticConnectionPropss                                              = 1647,
    /// AUTHENTICATION-BUILD-ATTEMPTS
    AuthenticationBuildAttempts                                            = 4595,
    /// AUTHENTICATION-CLASS-REF
    AuthenticationClassRef                                                 = 1635,
    /// AUTHENTICATION-ENABLED
    AuthenticationEnabled                                                  = 2037,
    /// AUTHENTICATION-PROPS-REF
    AuthenticationPropsRef                                                 = 4684,
    /// AUTHENTICATION-PROPSS
    AuthenticationPropss                                                   = 1136,
    /// AUTHENTICATION-REF
    AuthenticationRef                                                      = 782,
    /// AUTHENTICATION-RETRIES
    AuthenticationRetries                                                  = 466,
    /// AUTHENTICATION-ROLE-REF
    AuthenticationRoleRef                                                  = 1951,
    /// AUTHENTICATION-ROLE-REFS
    AuthenticationRoleRefs                                                 = 4298,
    /// AUTHENTICATION-TIMEOUT
    AuthenticationTimeout                                                  = 2629,
    /// AUTHENTICATION-VERIFY-ATTEMPTS
    AuthenticationVerifyAttempts                                           = 537,
    /// AUTO-AVAILABLE
    AutoAvailable                                                          = 1934,
    /// AUTO-COLLECT
    AutoCollect                                                            = 4489,
    /// AUTO-IP-PROPS
    AutoIpProps                                                            = 2922,
    /// AUTO-REQUIRE
    AutoRequire                                                            = 971,
    /// AUTO-START
    AutoStart                                                              = 423,
    /// AUTOMATIC-END
    AutomaticEnd                                                           = 104,
    /// AUTOSAR
    Autosar                                                                = 3311,
    /// AUTOSAR-CONNECTOR
    AutosarConnector                                                       = 607,
    /// AUTOSAR-DATA-TYPE-REF
    AutosarDataTypeRef                                                     = 1468,
    /// AUTOSAR-DATA-TYPE-REF-CONDITIONAL
    AutosarDataTypeRefConditional                                          = 407,
    /// AUTOSAR-ENGINEERING-OBJECT
    AutosarEngineeringObject                                               = 2055,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 3058,
    /// AUTOSAR-PARAMETER-IREF
    AutosarParameterIref                                                   = 2708,
    /// AUTOSAR-VARIABLE
    AutosarVariable                                                        = 3910,
    /// AUTOSAR-VARIABLE-IN-IMPL-DATATYPE
    AutosarVariableInImplDatatype                                          = 6122,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 2972,
    /// AUTOSAR-VARIABLE-IREF
    AutosarVariableIref                                                    = 1929,
    /// AUXILIARY-FIELD-DEFINITIONS
    AuxiliaryFieldDefinitions                                              = 5631,
    /// AUXILIARY-FIELDS
    AuxiliaryFields                                                        = 1526,
    /// AVERAGE-MEMORY-CONSUMPTION
    AverageMemoryConsumption                                               = 3569,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 4015,
    /// BANDWIDTH-LIMITATION
    BandwidthLimitation                                                    = 2094,
    /// BASE-COMPOSITION-REF
    BaseCompositionRef                                                     = 1919,
    /// BASE-CYCLE
    BaseCycle                                                              = 4074,
    /// BASE-FREQUENCY
    BaseFrequency                                                          = 17,
    /// BASE-IS-THIS-PACKAGE
    BaseIsThisPackage                                                      = 2550,
    /// BASE-REF
    BaseRef                                                                = 5417,
    /// BASE-REFERENCE-REF
    BaseReferenceRef                                                       = 4191,
    /// BASE-TYPE-ENCODING
    BaseTypeEncoding                                                       = 2336,
    /// BASE-TYPE-REF
    BaseTypeRef                                                            = 5412,
    /// BASE-TYPE-SIZE
    BaseTypeSize                                                           = 722,
    /// BASIC-CYCLE-LENGTH
    BasicCycleLength                                                       = 5267,
    /// BAUDRATE
    Baudrate                                                               = 6028,
    /// BEHAVIOR
    Behavior                                                               = 5905,
    /// BEHAVIOR-REF
    BehaviorRef                                                            = 707,
    /// BEST-CASE-EXECUTION-TIME
    BestCaseExecutionTime                                                  = 4875,
    /// BGCOLOR
    Bgcolor                                                                = 6003,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 3088,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 1990,
    /// BINARY-MANIFEST-ITEM-NUMERICAL-VALUE
    BinaryManifestItemNumericalValue                                       = 1968,
    /// BINARY-MANIFEST-ITEM-POINTER-VALUE
    BinaryManifestItemPointerValue                                         = 2465,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 2823,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 1152,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 1642,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 5266,
    /// BIT
    Bit                                                                    = 4884,
    /// BIT-NUMBER
    BitNumber                                                              = 5986,
    /// BIT-OFFSET
    BitOffset                                                              = 4959,
    /// BIT-POSITION
    BitPosition                                                            = 2620,
    /// BITFIELD-TEXT-TABLE-MASK-FIRST
    BitfieldTextTableMaskFirst                                             = 2918,
    /// BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR
    BitfieldTextTableMaskNvBlockDescriptor                                 = 2133,
    /// BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE
    BitfieldTextTableMaskPortPrototype                                     = 1292,
    /// BITFIELD-TEXT-TABLE-MASK-SECOND
    BitfieldTextTableMaskSecond                                            = 5834,
    /// BLACKLISTED-VERSIONS
    BlacklistedVersions                                                    = 156,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREF
    BlockIfStateActiveApIref                                               = 4788,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREFS
    BlockIfStateActiveApIrefs                                              = 1520,
    /// BLOCK-IF-STATE-ACTIVE-CP-REF
    BlockIfStateActiveCpRef                                                = 717,
    /// BLOCK-IF-STATE-ACTIVE-CP-REFS
    BlockIfStateActiveCpRefs                                               = 1509,
    /// BLOCK-INCONSISTENT
    BlockInconsistent                                                      = 713,
    /// BLOCK-STATE
    BlockState                                                             = 2299,
    /// BLOCK-STATES
    BlockStates                                                            = 123,
    /// BLOCKLISTED-VERSIONS
    BlocklistedVersions                                                    = 1271,
    /// BLUEPRINT-CONDITION
    BlueprintCondition                                                     = 3954,
    /// BLUEPRINT-DERIVATION-GUIDE
    BlueprintDerivationGuide                                               = 2810,
    /// BLUEPRINT-MAPPING
    BlueprintMapping                                                       = 5866,
    /// BLUEPRINT-MAPPING-GUIDE
    BlueprintMappingGuide                                                  = 3557,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 2561,
    /// BLUEPRINT-MAPS
    BlueprintMaps                                                          = 1206,
    /// BLUEPRINT-POLICY-LIST
    BlueprintPolicyList                                                    = 3880,
    /// BLUEPRINT-POLICY-NOT-MODIFIABLE
    BlueprintPolicyNotModifiable                                           = 5686,
    /// BLUEPRINT-POLICY-SINGLE
    BlueprintPolicySingle                                                  = 2213,
    /// BLUEPRINT-POLICYS
    BlueprintPolicys                                                       = 2261,
    /// BLUEPRINT-REF
    BlueprintRef                                                           = 581,
    /// BOOLEAN-VALUE-VARIATION-POINT
    BooleanValueVariationPoint                                             = 2030,
    /// BOR-COUNTER-L-1-TO-L-2
    BorCounterL1ToL2                                                       = 2786,
    /// BOR-TIME-L-1
    BorTimeL1                                                              = 683,
    /// BOR-TIME-L-2
    BorTimeL2                                                              = 441,
    /// BOR-TIME-TX-ENSURED
    BorTimeTxEnsured                                                       = 2147,
    /// BR
    Br                                                                     = 902,
    /// BROADCAST
    Broadcast                                                              = 4636,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 3048,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 2462,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 1561,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 1315,
    /// BSW-BEHAVIOR-REF
    BswBehaviorRef                                                         = 1977,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 5023,
    /// BSW-CLIENT-POLICY
    BswClientPolicy                                                        = 6207,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 5661,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 2021,
    /// BSW-DATA-SEND-POLICY
    BswDataSendPolicy                                                      = 5705,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 906,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 5784,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 1913,
    /// BSW-ENTITY-REF
    BswEntityRef                                                           = 974,
    /// BSW-ENTRY-KIND
    BswEntryKind                                                           = 3716,
    /// BSW-ENTRY-RELATIONSHIP
    BswEntryRelationship                                                   = 2811,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 2640,
    /// BSW-ENTRY-RELATIONSHIP-TYPE
    BswEntryRelationshipType                                               = 4764,
    /// BSW-ENTRY-RELATIONSHIPS
    BswEntryRelationships                                                  = 4590,
    /// BSW-EXCLUSIVE-AREA-POLICY
    BswExclusiveAreaPolicy                                                 = 656,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 2222,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 195,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 5938,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 2477,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 3528,
    /// BSW-INTERNAL-TRIGGERING-POINT-POLICY
    BswInternalTriggeringPointPolicy                                       = 4079,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF
    BswInternalTriggeringPointRef                                          = 1684,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL
    BswInternalTriggeringPointRefConditional                               = 2091,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 3959,
    /// BSW-INTERRUPT-EVENT
    BswInterruptEvent                                                      = 2446,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 875,
    /// BSW-MODE-GROUP-REF
    BswModeGroupRef                                                        = 1362,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 287,
    /// BSW-MODE-RECEIVER-POLICY
    BswModeReceiverPolicy                                                  = 1113,
    /// BSW-MODE-SENDER-POLICY
    BswModeSenderPolicy                                                    = 2317,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 1853,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 469,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 5851,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 4619,
    /// BSW-MODULE-DEPENDENCYS
    BswModuleDependencys                                                   = 5904,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 2297,
    /// BSW-MODULE-DESCRIPTION-REF
    BswModuleDescriptionRef                                                = 3038,
    /// BSW-MODULE-DESCRIPTION-REF-CONDITIONAL
    BswModuleDescriptionRefConditional                                     = 3288,
    /// BSW-MODULE-DOCUMENTATIONS
    BswModuleDocumentations                                                = 822,
    /// BSW-MODULE-ENTITY-REF
    BswModuleEntityRef                                                     = 5244,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 4081,
    /// BSW-MODULE-ENTRY-REF
    BswModuleEntryRef                                                      = 4099,
    /// BSW-MODULE-ENTRY-REF-CONDITIONAL
    BswModuleEntryRefConditional                                           = 3298,
    /// BSW-MODULE-INSTANCE-REF
    BswModuleInstanceRef                                                   = 5427,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 3952,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 507,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 4163,
    /// BSW-PARAMETER-POLICY
    BswParameterPolicy                                                     = 2672,
    /// BSW-PER-INSTANCE-MEMORY-POLICY
    BswPerInstanceMemoryPolicy                                             = 2063,
    /// BSW-PER-INSTANCE-MEMORY-POLICYS
    BswPerInstanceMemoryPolicys                                            = 2884,
    /// BSW-QUEUED-DATA-RECEPTION-POLICY
    BswQueuedDataReceptionPolicy                                           = 1227,
    /// BSW-RELEASED-TRIGGER-POLICY
    BswReleasedTriggerPolicy                                               = 6060,
    /// BSW-RESOLUTION
    BswResolution                                                          = 1403,
    /// BSW-RESOURCE-ESTIMATION
    BswResourceEstimation                                                  = 6260,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 1231,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 3681,
    /// BSW-SERVICE-DEPENDENCY
    BswServiceDependency                                                   = 3998,
    /// BSW-SERVICE-DEPENDENCY-REF
    BswServiceDependencyRef                                                = 1084,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 6174,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 2053,
    /// BSW-TRIGGER-DIRECT-IMPLEMENTATION
    BswTriggerDirectImplementation                                         = 4432,
    /// BSW-TRIGGER-REF
    BswTriggerRef                                                          = 2995,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 5113,
    /// BUCKET-SIZE
    BucketSize                                                             = 2385,
    /// BUFFER-COMPUTATION
    BufferComputation                                                      = 4546,
    /// BUFFER-OUTPUT
    BufferOutput                                                           = 821,
    /// BUFFER-PROPERTIES
    BufferProperties                                                       = 353,
    /// BUFFER-RATIO
    BufferRatio                                                            = 4083,
    /// BUILD-ACTION
    BuildAction                                                            = 1433,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 910,
    /// BUILD-ACTION-ENVIRONMENTS
    BuildActionEnvironments                                                = 1655,
    /// BUILD-ACTION-IO-ELEMENT
    BuildActionIoElement                                                   = 4413,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 5496,
    /// BUILD-ACTION-MANIFEST-REF
    BuildActionManifestRef                                                 = 5282,
    /// BUILD-ACTION-MANIFEST-REF-CONDITIONAL
    BuildActionManifestRefConditional                                      = 5152,
    /// BUILD-ACTION-MANIFESTS
    BuildActionManifests                                                   = 3461,
    /// BUILD-ACTIONS
    BuildActions                                                           = 1203,
    /// BUILD-TYPE
    BuildType                                                              = 1955,
    /// BULK-DATA-CATEGORY
    BulkDataCategory                                                       = 1750,
    /// BULK-NV-BLOCK
    BulkNvBlock                                                            = 2182,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 4638,
    /// BULK-NV-DATA-DESCRIPTORS
    BulkNvDataDescriptors                                                  = 1116,
    /// BUNDLED-CONNECTIONS
    BundledConnections                                                     = 3198,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 2073,
    /// BURST-SIZE
    BurstSize                                                              = 2492,
    /// BURST-SIZE-REQUEST
    BurstSizeRequest                                                       = 1319,
    /// BURST-SIZE-RESPONSE
    BurstSizeResponse                                                      = 4045,
    /// BUS-DEPENDENT-NM-ECUS
    BusDependentNmEcus                                                     = 301,
    /// BUS-ID
    BusId                                                                  = 473,
    /// BUS-IDLE-TIMEOUT-PERIOD
    BusIdleTimeoutPeriod                                                   = 2256,
    /// BUS-MIRROR-CAN-ID-RANGE-MAPPING
    BusMirrorCanIdRangeMapping                                             = 4652,
    /// BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING
    BusMirrorCanIdToCanIdMapping                                           = 3141,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 2583,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 5186,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 4544,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 3173,
    /// BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING
    BusMirrorLinPidToCanIdMapping                                          = 6193,
    /// BUS-MIRROR-NETWORK-ID
    BusMirrorNetworkId                                                     = 2480,
    /// BUS-OFF-RECOVERY
    BusOffRecovery                                                         = 4931,
    /// BUS-SPECIFIC-NM-ECU
    BusSpecificNmEcu                                                       = 1165,
    /// BY-PASS-POINT-IREF
    ByPassPointIref                                                        = 6008,
    /// BY-PASS-POINT-IREFS
    ByPassPointIrefs                                                       = 6279,
    /// BYPASS-ETHER-TYPE
    BypassEtherType                                                        = 3621,
    /// BYPASS-ETHER-TYPES
    BypassEtherTypes                                                       = 1482,
    /// BYPASS-VLAN
    BypassVlan                                                             = 3147,
    /// BYPASS-VLANS
    BypassVlans                                                            = 1799,
    /// BYTE
    Byte                                                                   = 3268,
    /// BYTE-ORDER
    ByteOrder                                                              = 100,
    /// BYTE-VALUE
    ByteValue                                                              = 4207,
    /// BYTE-VALUES
    ByteValues                                                             = 2510,
    /// C-CODE
    CCode                                                                  = 5845,
    /// CALC-RAM-BLOCK-CRC
    CalcRamBlockCrc                                                        = 3918,
    /// CALCULATION-FORMULA
    CalculationFormula                                                     = 2304,
    /// CALIBRATION-PARAMETER-VALUE
    CalibrationParameterValue                                              = 4752,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 575,
    /// CALIBRATION-PARAMETER-VALUE-SET-REF
    CalibrationParameterValueSetRef                                        = 3637,
    /// CALIBRATION-PARAMETER-VALUE-SET-REFS
    CalibrationParameterValueSetRefs                                       = 4341,
    /// CALIBRATION-PARAMETER-VALUES
    CalibrationParameterValues                                             = 1980,
    /// CALL-POINTS
    CallPoints                                                             = 3255,
    /// CALL-SIGNAL-REF
    CallSignalRef                                                          = 663,
    /// CALL-SIGNAL-TRIGGERING-REF
    CallSignalTriggeringRef                                                = 1578,
    /// CALL-TIMEOUT
    CallTimeout                                                            = 55,
    /// CALL-TYPE
    CallType                                                               = 1065,
    /// CALLBACK-HEADER-REF
    CallbackHeaderRef                                                      = 2268,
    /// CALLBACK-HEADER-REFS
    CallbackHeaderRefs                                                     = 2006,
    /// CALLED-ENTRY-REF
    CalledEntryRef                                                         = 2248,
    /// CALLED-ENTRYS
    CalledEntrys                                                           = 3686,
    /// CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF
    CalledFromWithinExclusiveAreaRef                                       = 2442,
    /// CAN-ADDRESSING-MODE
    CanAddressingMode                                                      = 4817,
    /// CAN-BAUDRATE
    CanBaudrate                                                            = 5865,
    /// CAN-BE-INVOKED-CONCURRENTLY
    CanBeInvokedConcurrently                                               = 2898,
    /// CAN-BIT-RATE-SWITCH
    CanBitRateSwitch                                                       = 3682,
    /// CAN-CLUSTER
    CanCluster                                                             = 4960,
    /// CAN-CLUSTER-CONDITIONAL
    CanClusterConditional                                                  = 4236,
    /// CAN-CLUSTER-VARIANTS
    CanClusterVariants                                                     = 68,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 4006,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 4169,
    /// CAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    CanCommunicationControllerConditional                                  = 1552,
    /// CAN-COMMUNICATION-CONTROLLER-VARIANTS
    CanCommunicationControllerVariants                                     = 5927,
    /// CAN-CONFIG
    CanConfig                                                              = 179,
    /// CAN-CONTROLLER-ATTRIBUTES
    CanControllerAttributes                                                = 4709,
    /// CAN-CONTROLLER-CONFIGURATION
    CanControllerConfiguration                                             = 912,
    /// CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS
    CanControllerConfigurationRequirements                                 = 5039,
    /// CAN-CONTROLLER-FD-ATTRIBUTES
    CanControllerFdAttributes                                              = 293,
    /// CAN-CONTROLLER-FD-REQUIREMENTS
    CanControllerFdRequirements                                            = 3437,
    /// CAN-CONTROLLER-XL-ATTRIBUTES
    CanControllerXlAttributes                                              = 1870,
    /// CAN-CONTROLLER-XL-REQUIREMENTS
    CanControllerXlRequirements                                            = 3116,
    /// CAN-ENTER-EXCLUSIVE-AREA-REF
    CanEnterExclusiveAreaRef                                               = 5083,
    /// CAN-ENTER-EXCLUSIVE-AREA-REFS
    CanEnterExclusiveAreaRefs                                              = 385,
    /// CAN-ENTERS
    CanEnters                                                              = 3153,
    /// CAN-FD-BAUDRATE
    CanFdBaudrate                                                          = 88,
    /// CAN-FD-CONFIG
    CanFdConfig                                                            = 119,
    /// CAN-FD-FRAME-SUPPORT
    CanFdFrameSupport                                                      = 2223,
    /// CAN-FRAME
    CanFrame                                                               = 666,
    /// CAN-FRAME-RX-BEHAVIOR
    CanFrameRxBehavior                                                     = 1676,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 3793,
    /// CAN-FRAME-TX-BEHAVIOR
    CanFrameTxBehavior                                                     = 1861,
    /// CAN-GLOBAL-TIME-DOMAIN-PROPS
    CanGlobalTimeDomainProps                                               = 1237,
    /// CAN-ID-RANGE-MAPPINGS
    CanIdRangeMappings                                                     = 1214,
    /// CAN-ID-TO-CAN-ID-MAPPINGS
    CanIdToCanIdMappings                                                   = 5880,
    /// CAN-IDENTIFIER
    CanIdentifier                                                          = 5000,
    /// CAN-IDENTIFIER-MASK
    CanIdentifierMask                                                      = 6108,
    /// CAN-IDENTIFIER-RANGE
    CanIdentifierRange                                                     = 227,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 1739,
    /// CAN-NM-CLUSTER-COUPLING
    CanNmClusterCoupling                                                   = 903,
    /// CAN-NM-ECU
    CanNmEcu                                                               = 436,
    /// CAN-NM-NODE
    CanNmNode                                                              = 4582,
    /// CAN-NM-TX-CAN-ID
    CanNmTxCanId                                                           = 5779,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 3490,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 4526,
    /// CAN-TP-ADDRESS-REF
    CanTpAddressRef                                                        = 4869,
    /// CAN-TP-ADDRESS-REFS
    CanTpAddressRefs                                                       = 5685,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 1082,
    /// CAN-TP-CHANNEL-REF
    CanTpChannelRef                                                        = 508,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 4755,
    /// CAN-TP-CONNECTION
    CanTpConnection                                                        = 1515,
    /// CAN-TP-ECU
    CanTpEcu                                                               = 1222,
    /// CAN-TP-NODE
    CanTpNode                                                              = 1042,
    /// CAN-XL-BAUDRATE
    CanXlBaudrate                                                          = 2610,
    /// CAN-XL-CONFIG
    CanXlConfig                                                            = 5533,
    /// CAN-XL-CONFIG-REF
    CanXlConfigRef                                                         = 4028,
    /// CAN-XL-CONFIG-REQS
    CanXlConfigReqs                                                        = 785,
    /// CAN-XL-FRAME-TRIGGERING-PROPS
    CanXlFrameTriggeringProps                                              = 6192,
    /// CAN-XL-NM-PROPS
    CanXlNmProps                                                           = 1281,
    /// CAN-XL-PROPS
    CanXlProps                                                             = 2836,
    /// CAN-XL-PROPS-REF
    CanXlPropsRef                                                          = 4313,
    /// CAN-XL-PROPS-REFS
    CanXlPropsRefs                                                         = 3908,
    /// CANCELLATION
    Cancellation                                                           = 4493,
    /// CAPABILITY
    Capability                                                             = 1341,
    /// CAPABILITY-RECORDS
    CapabilityRecords                                                      = 634,
    /// CAPTION
    Caption                                                                = 4104,
    /// CAS-RX-LOW-MAX
    CasRxLowMax                                                            = 2404,
    /// CAT-2-ISR
    Cat2Isr                                                                = 109,
    /// CATEGORY
    Category                                                               = 1673,
    /// CDD-TYPE
    CddType                                                                = 5103,
    /// CERT-COMMON-NAME
    CertCommonName                                                         = 845,
    /// CERTIFICATE-EVALUATIONS
    CertificateEvaluations                                                 = 5422,
    /// CERTIFICATE-REF
    CertificateRef                                                         = 2432,
    /// CERTIFICATE-TO-KEY-SLOT-MAPPINGS
    CertificateToKeySlotMappings                                           = 3199,
    /// CHANGE
    Change                                                                 = 1564,
    /// CHANNEL-ID
    ChannelId                                                              = 946,
    /// CHANNEL-MODE
    ChannelMode                                                            = 450,
    /// CHANNEL-NAME
    ChannelName                                                            = 1979,
    /// CHANNEL-REF
    ChannelRef                                                             = 1021,
    /// CHANNEL-SYNCHRONOUS-WAKEUP
    ChannelSynchronousWakeup                                               = 37,
    /// CHANNELS
    Channels                                                               = 4901,
    /// CHANNELS-PER-FRAME
    ChannelsPerFrame                                                       = 3037,
    /// CHAPTER
    Chapter                                                                = 5843,
    /// CHAPTERS
    Chapters                                                               = 4935,
    /// CHECK-PER-SOURCE-ID
    CheckPerSourceId                                                       = 295,
    /// CHECK-STATIC-BLOCK-ID
    CheckStaticBlockId                                                     = 4612,
    /// CHECKPOINT-FINAL-REF
    CheckpointFinalRef                                                     = 5941,
    /// CHECKPOINT-FINAL-REFS
    CheckpointFinalRefs                                                    = 3608,
    /// CHECKPOINT-ID
    CheckpointId                                                           = 1611,
    /// CHECKPOINT-INITIAL-REF
    CheckpointInitialRef                                                   = 3241,
    /// CHECKPOINT-INITIAL-REFS
    CheckpointInitialRefs                                                  = 1379,
    /// CHECKPOINT-IREF
    CheckpointIref                                                         = 3580,
    /// CHECKPOINT-REF
    CheckpointRef                                                          = 6198,
    /// CHECKPOINT-REFS
    CheckpointRefs                                                         = 981,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 3519,
    /// CHECKPOINT-TRANSITION-REF
    CheckpointTransitionRef                                                = 706,
    /// CHECKPOINTS
    Checkpoints                                                            = 5478,
    /// CHECKPOINTSS
    Checkpointss                                                           = 4205,
    /// CHECKSUM-VALUE
    ChecksumValue                                                          = 1297,
    /// CHECKSUM-VERIFICATION
    ChecksumVerification                                                   = 3050,
    /// CHOICES
    Choices                                                                = 2406,
    /// CIPHER-SUITE
    CipherSuite                                                            = 3427,
    /// CIPHER-SUITE-CONFIGS
    CipherSuiteConfigs                                                     = 2373,
    /// CIPHER-SUITE-ID
    CipherSuiteId                                                          = 235,
    /// CIPHER-SUITE-NAME
    CipherSuiteName                                                        = 4404,
    /// CIPHER-SUITE-PRIORITY
    CipherSuitePriority                                                    = 4531,
    /// CIPHER-SUITE-SHORT-LABEL
    CipherSuiteShortLabel                                                  = 4835,
    /// CKN-REF
    CknRef                                                                 = 2031,
    /// CLAIMED-FUNCTION-GROUP-REF
    ClaimedFunctionGroupRef                                                = 1104,
    /// CLAIMED-FUNCTION-GROUP-REFS
    ClaimedFunctionGroupRefs                                               = 4088,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 526,
    /// CLASS-CONTENTS
    ClassContents                                                          = 5751,
    /// CLASS-TAILORINGS
    ClassTailorings                                                        = 826,
    /// CLASSIFICATION
    Classification                                                         = 4929,
    /// CLASSIFICATIONS
    Classifications                                                        = 3662,
    /// CLEAR-CONDITION-GROUP-REF
    ClearConditionGroupRef                                                 = 3584,
    /// CLEAR-CONDITION-REF
    ClearConditionRef                                                      = 3904,
    /// CLEAR-CONDITION-REFS
    ClearConditionRefs                                                     = 1485,
    /// CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF
    ClearDiagnosticInformationClassRef                                     = 5701,
    /// CLEAR-DTC-LIMITATION
    ClearDtcLimitation                                                     = 5329,
    /// CLEAR-EVENT-ALLOWED-BEHAVIOR
    ClearEventAllowedBehavior                                              = 3894,
    /// CLEAR-EVENT-BEHAVIOR
    ClearEventBehavior                                                     = 5763,
    /// CLEAR-FROM-VALID-TO-INVALID
    ClearFromValidToInvalid                                                = 210,
    /// CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF
    ClearResetEmissionRelatedDiagnosticInfoClassRef                        = 3659,
    /// CLIENT-CAPABILITY
    ClientCapability                                                       = 6148,
    /// CLIENT-COM-SPEC
    ClientComSpec                                                          = 1539,
    /// CLIENT-ID
    ClientId                                                               = 631,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 3676,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 819,
    /// CLIENT-ID-DEFINITION-SET-REF
    ClientIdDefinitionSetRef                                               = 3731,
    /// CLIENT-ID-DEFINITION-SET-REFS
    ClientIdDefinitionSetRefs                                              = 4714,
    /// CLIENT-ID-DEFINITIONS
    ClientIdDefinitions                                                    = 643,
    /// CLIENT-ID-RANGE
    ClientIdRange                                                          = 3862,
    /// CLIENT-INTENT
    ClientIntent                                                           = 3292,
    /// CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST
    ClientIpAddrFromConnectionRequest                                      = 3983,
    /// CLIENT-POLICYS
    ClientPolicys                                                          = 816,
    /// CLIENT-PORT-FROM-CONNECTION-REQUEST
    ClientPortFromConnectionRequest                                        = 6030,
    /// CLIENT-PORT-REF
    ClientPortRef                                                          = 4986,
    /// CLIENT-SERVER-ANNOTATION
    ClientServerAnnotation                                                 = 5001,
    /// CLIENT-SERVER-ANNOTATIONS
    ClientServerAnnotations                                                = 978,
    /// CLIENT-SERVER-APPLICATION-ERROR-MAPPING
    ClientServerApplicationErrorMapping                                    = 3657,
    /// CLIENT-SERVER-ARRAY-ELEMENT-MAPPING
    ClientServerArrayElementMapping                                        = 497,
    /// CLIENT-SERVER-ARRAY-TYPE-MAPPING
    ClientServerArrayTypeMapping                                           = 2220,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 44,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 495,
    /// CLIENT-SERVER-INTERFACE-REF
    ClientServerInterfaceRef                                               = 303,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 3994,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 505,
    /// CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING
    ClientServerOperationBlueprintMapping                                  = 4484,
    /// CLIENT-SERVER-OPERATION-COM-PROPS
    ClientServerOperationComProps                                          = 4065,
    /// CLIENT-SERVER-OPERATION-IREF
    ClientServerOperationIref                                              = 3609,
    /// CLIENT-SERVER-OPERATION-MAPPING
    ClientServerOperationMapping                                           = 5539,
    /// CLIENT-SERVER-OPERATION-REF
    ClientServerOperationRef                                               = 3056,
    /// CLIENT-SERVER-PORTS
    ClientServerPorts                                                      = 1337,
    /// CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING
    ClientServerPrimitiveTypeMapping                                       = 1455,
    /// CLIENT-SERVER-RECORD-ELEMENT-MAPPING
    ClientServerRecordElementMapping                                       = 5347,
    /// CLIENT-SERVER-RECORD-TYPE-MAPPING
    ClientServerRecordTypeMapping                                          = 3560,
    /// CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING
    ClientServerToSignalGroupMapping                                       = 669,
    /// CLIENT-SERVER-TO-SIGNAL-MAPPING
    ClientServerToSignalMapping                                            = 4887,
    /// CLIENT-SERVICE-MAJOR-VERSION
    ClientServiceMajorVersion                                              = 5775,
    /// CLIENT-SERVICE-MINOR-VERSION
    ClientServiceMinorVersion                                              = 5836,
    /// CLOCK-REFERENCE-REF
    ClockReferenceRef                                                      = 5212,
    /// CLUSTER-DRIFT-DAMPING
    ClusterDriftDamping                                                    = 1170,
    /// CLUSTERED-COMPONENT-IREF
    ClusteredComponentIref                                                 = 6154,
    /// CLUSTERED-COMPONENT-IREFS
    ClusteredComponentIrefs                                                = 5966,
    /// CODE
    Code                                                                   = 5185,
    /// CODE-DESCRIPTORS
    CodeDescriptors                                                        = 5174,
    /// CODE-LABEL
    CodeLabel                                                              = 2923,
    /// COLD-START-ATTEMPTS
    ColdStartAttempts                                                      = 2,
    /// COLLECTED-INSTANCE-IREF
    CollectedInstanceIref                                                  = 3502,
    /// COLLECTED-INSTANCE-IREFS
    CollectedInstanceIrefs                                                 = 3620,
    /// COLLECTION
    Collection                                                             = 3460,
    /// COLLECTION-PROPS
    CollectionProps                                                        = 2504,
    /// COLLECTION-REF
    CollectionRef                                                          = 3866,
    /// COLLECTION-SEMANTICS
    CollectionSemantics                                                    = 3046,
    /// COLLECTION-THRESHOLD
    CollectionThreshold                                                    = 1590,
    /// COLLECTION-TIMEOUT
    CollectionTimeout                                                      = 565,
    /// COLLECTION-TRIGGER
    CollectionTrigger                                                      = 1601,
    /// COLLISION-RESOLVING-SCHEDULE-REF
    CollisionResolvingScheduleRef                                          = 3562,
    /// COLOR-MODE
    ColorMode                                                              = 2564,
    /// COLSPEC
    Colspec                                                                = 1194,
    /// COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS
    ComBasedSignalGroupTransformations                                     = 1987,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 6136,
    /// COM-CONFIGURATION-GW-TIME-BASE
    ComConfigurationGwTimeBase                                             = 5720,
    /// COM-CONFIGURATION-ID
    ComConfigurationId                                                     = 4449,
    /// COM-CONFIGURATION-RX-TIME-BASE
    ComConfigurationRxTimeBase                                             = 1896,
    /// COM-CONFIGURATION-TX-TIME-BASE
    ComConfigurationTxTimeBase                                             = 3142,
    /// COM-CONTROL-CLASS-REF
    ComControlClassRef                                                     = 3669,
    /// COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION
    ComEnableMdtForCyclicTransmission                                      = 4906,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 4930,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 2994,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 4056,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 5914,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 6036,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 2284,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 6255,
    /// COM-MANAGEMENT-GROUP-REF
    ComManagementGroupRef                                                  = 3319,
    /// COM-MANAGEMENT-GROUP-REFS
    ComManagementGroupRefs                                                 = 5402,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 5070,
    /// COM-MANAGEMENT-MAPPINGS
    ComManagementMappings                                                  = 824,
    /// COM-MANAGEMENT-PORT-GROUP-IREF
    ComManagementPortGroupIref                                             = 4431,
    /// COM-MANAGEMENT-PORT-GROUP-IREFS
    ComManagementPortGroupIrefs                                            = 2235,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 4172,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 2437,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 2600,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 3804,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 1525,
    /// COM-PROCESSING-PERIOD
    ComProcessingPeriod                                                    = 5541,
    /// COM-PROPS
    ComProps                                                               = 5956,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 2132,
    /// COM-TRIGGER-GRANT
    ComTriggerGrant                                                        = 814,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 580,
    /// COMM-CONNECTORS
    CommConnectors                                                         = 4038,
    /// COMM-CONTROLLER-MAPPINGS
    CommControllerMappings                                                 = 6169,
    /// COMM-CONTROLLER-REF
    CommControllerRef                                                      = 5512,
    /// COMM-CONTROLLERS
    CommControllers                                                        = 98,
    /// COMMAND
    Command                                                                = 1204,
    /// COMMANDS
    Commands                                                               = 3139,
    /// COMMENT
    Comment                                                                = 3430,
    /// COMMITTED-BURST-SIZE
    CommittedBurstSize                                                     = 5910,
    /// COMMITTED-INFORMATION-RATE
    CommittedInformationRate                                               = 1588,
    /// COMMON-PROPERTIES
    CommonProperties                                                       = 2946,
    /// COMMON-SIGNAL-PATH
    CommonSignalPath                                                       = 1039,
    /// COMMUNICATION-BUFFER-LOCKING
    CommunicationBufferLocking                                             = 1865,
    /// COMMUNICATION-CLUSTER-REF
    CommunicationClusterRef                                                = 5503,
    /// COMMUNICATION-CLUSTER-REFS
    CommunicationClusterRefs                                               = 3784,
    /// COMMUNICATION-CONNECTOR-REF
    CommunicationConnectorRef                                              = 5700,
    /// COMMUNICATION-CONNECTOR-REF-CONDITIONAL
    CommunicationConnectorRefConditional                                   = 2085,
    /// COMMUNICATION-CONNECTOR-REFS
    CommunicationConnectorRefs                                             = 2247,
    /// COMMUNICATION-CONNECTORS
    CommunicationConnectors                                                = 757,
    /// COMMUNICATION-CONTROLLER-MAPPING
    CommunicationControllerMapping                                         = 5261,
    /// COMMUNICATION-CONTROLLER-REF
    CommunicationControllerRef                                             = 3766,
    /// COMMUNICATION-CONTROLLERS
    CommunicationControllers                                               = 6112,
    /// COMMUNICATION-CYCLE
    CommunicationCycle                                                     = 592,
    /// COMMUNICATION-DIRECTION
    CommunicationDirection                                                 = 4465,
    /// COMMUNICATION-MODE
    CommunicationMode                                                      = 328,
    /// COMMUNICATION-RESOURCE-PROPS
    CommunicationResourceProps                                             = 4984,
    /// COMMUNICATION-RESOURCE-REF
    CommunicationResourceRef                                               = 3253,
    /// COMPARE-TYPE
    CompareType                                                            = 2298,
    /// COMPARE-VALUE
    CompareValue                                                           = 3829,
    /// COMPILER
    Compiler                                                               = 1314,
    /// COMPILERS
    Compilers                                                              = 2892,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 6097,
    /// COMPLEX-TYPE-MAPPING
    ComplexTypeMapping                                                     = 4408,
    /// COMPONENT-CLUSTERING
    ComponentClustering                                                    = 2275,
    /// COMPONENT-IMPLEMENTATION-REF
    ComponentImplementationRef                                             = 2553,
    /// COMPONENT-IREF
    ComponentIref                                                          = 5304,
    /// COMPONENT-IREFS
    ComponentIrefs                                                         = 64,
    /// COMPONENT-QUALIFIER
    ComponentQualifier                                                     = 3356,
    /// COMPONENT-REF
    ComponentRef                                                           = 2169,
    /// COMPONENT-SEPARATION
    ComponentSeparation                                                    = 5177,
    /// COMPONENTS
    Components                                                             = 2955,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 590,
    /// COMPOSITE-NETWORK-REPRESENTATION
    CompositeNetworkRepresentation                                         = 745,
    /// COMPOSITE-NETWORK-REPRESENTATIONS
    CompositeNetworkRepresentations                                        = 6120,
    /// COMPOSITE-RULE-BASED-VALUE-SPECIFICATION
    CompositeRuleBasedValueSpecification                                   = 177,
    /// COMPOSITE-SERVICE-INTERFACE-REF
    CompositeServiceInterfaceRef                                           = 934,
    /// COMPOSITE-TYPE-MAPPINGS
    CompositeTypeMappings                                                  = 4880,
    /// COMPOSITION-DATA-PROTOTYPE-REF
    CompositionDataPrototypeRef                                            = 3611,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 5066,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 470,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 6115,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF
    CompositionSwComponentTypeRef                                          = 2608,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL
    CompositionSwComponentTypeRefConditional                               = 3011,
    /// COMPOUND-PRIMITIVE-ARGUMENTS
    CompoundPrimitiveArguments                                             = 1825,
    /// COMPRESSED-SOFTWARE-PACKAGE-SIZE
    CompressedSoftwarePackageSize                                          = 4171,
    /// COMPU-CONST
    CompuConst                                                             = 5357,
    /// COMPU-DEFAULT-VALUE
    CompuDefaultValue                                                      = 5666,
    /// COMPU-DENOMINATOR
    CompuDenominator                                                       = 4557,
    /// COMPU-INTERNAL-TO-PHYS
    CompuInternalToPhys                                                    = 1369,
    /// COMPU-INVERSE-VALUE
    CompuInverseValue                                                      = 5377,
    /// COMPU-METHOD
    CompuMethod                                                            = 1703,
    /// COMPU-METHOD-REF
    CompuMethodRef                                                         = 5594,
    /// COMPU-NUMERATOR
    CompuNumerator                                                         = 444,
    /// COMPU-PHYS-TO-INTERNAL
    CompuPhysToInternal                                                    = 3362,
    /// COMPU-RATIONAL-COEFFS
    CompuRationalCoeffs                                                    = 5201,
    /// COMPU-SCALE
    CompuScale                                                             = 2657,
    /// COMPU-SCALES
    CompuScales                                                            = 2905,
    /// COMPUTED
    Computed                                                               = 2912,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 291,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 2455,
    /// COND
    Cond                                                                   = 4976,
    /// CONDITION
    Condition                                                              = 1858,
    /// CONDITION-ACCESS
    ConditionAccess                                                        = 3111,
    /// CONDITION-FORMULA
    ConditionFormula                                                       = 1470,
    /// CONDITIONAL-CHANGE-NAD
    ConditionalChangeNad                                                   = 1688,
    /// CONDITIONS
    Conditions                                                             = 5714,
    /// CONFIDENCE-INTERVAL
    ConfidenceInterval                                                     = 2679,
    /// CONFIDENCE-INTERVALS
    ConfidenceIntervals                                                    = 2454,
    /// CONFIDENTIALITY-OFFSET
    ConfidentialityOffset                                                  = 883,
    /// CONFIG-CLASS
    ConfigClass                                                            = 1752,
    /// CONFIG-ELEMENT-DEF-GLOBAL-REF
    ConfigElementDefGlobalRef                                              = 1950,
    /// CONFIG-ELEMENT-DEF-LOCAL-REF
    ConfigElementDefLocalRef                                               = 1651,
    /// CONFIG-VARIANT
    ConfigVariant                                                          = 5229,
    /// CONFIGURATION-CLASS-AFFECTION
    ConfigurationClassAffection                                            = 4921,
    /// CONFIGURATION-HANDLING
    ConfigurationHandling                                                  = 395,
    /// CONFIGURED-NAD
    ConfiguredNad                                                          = 4659,
    /// CONFIRMATION-THRESHOLD
    ConfirmationThreshold                                                  = 3054,
    /// CONFLICTS
    Conflicts                                                              = 3473,
    /// CONFLICTS-TO
    ConflictsTo                                                            = 2567,
    /// CONNECT
    Connect                                                                = 973,
    /// CONNECTED-INDICATORS
    ConnectedIndicators                                                    = 1880,
    /// CONNECTION-BUNDLES
    ConnectionBundles                                                      = 21,
    /// CONNECTION-IS-MANDATORY
    ConnectionIsMandatory                                                  = 597,
    /// CONNECTION-NEGOTIATION-BEHAVIOR
    ConnectionNegotiationBehavior                                          = 1852,
    /// CONNECTION-PRIO-PDUS
    ConnectionPrioPdus                                                     = 1510,
    /// CONNECTION-TYPE
    ConnectionType                                                         = 3279,
    /// CONNECTIONS
    Connections                                                            = 2944,
    /// CONNECTOR-REF
    ConnectorRef                                                           = 4927,
    /// CONNECTOR-REFS
    ConnectorRefs                                                          = 3352,
    /// CONNECTORS
    Connectors                                                             = 5354,
    /// CONSIDER-BUILD-NUMBER
    ConsiderBuildNumber                                                    = 4029,
    /// CONSIDER-PTO-STATUS
    ConsiderPtoStatus                                                      = 5684,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 5731,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 4790,
    /// CONSISTENCY-NEEDSS
    ConsistencyNeedss                                                      = 2982,
    /// CONSTANT-MAPPING-REF
    ConstantMappingRef                                                     = 254,
    /// CONSTANT-MAPPING-REFS
    ConstantMappingRefs                                                    = 951,
    /// CONSTANT-MEMORYS
    ConstantMemorys                                                        = 1146,
    /// CONSTANT-REF
    ConstantRef                                                            = 3771,
    /// CONSTANT-REFERENCE
    ConstantReference                                                      = 4020,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 1777,
    /// CONSTANT-SPECIFICATION-MAPPING
    ConstantSpecificationMapping                                           = 3479,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 2581,
    /// CONSTANT-VALUE-MAPPING-REF
    ConstantValueMappingRef                                                = 357,
    /// CONSTANT-VALUE-MAPPING-REFS
    ConstantValueMappingRefs                                               = 1803,
    /// CONSTR-LEVEL
    ConstrLevel                                                            = 3185,
    /// CONSTRAINT-REF
    ConstraintRef                                                          = 3690,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 5897,
    /// CONSTRAINT-TAILORINGS
    ConstraintTailorings                                                   = 336,
    /// CONSUMED-DDS-OPERATIONS
    ConsumedDdsOperations                                                  = 1639,
    /// CONSUMED-DDS-SERVICE-EVENTS
    ConsumedDdsServiceEvents                                               = 4607,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 1832,
    /// CONSUMED-EVENT-GROUP-REF
    ConsumedEventGroupRef                                                  = 5628,
    /// CONSUMED-EVENT-GROUP-REFS
    ConsumedEventGroupRefs                                                 = 5398,
    /// CONSUMED-EVENT-GROUPS
    ConsumedEventGroups                                                    = 2051,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 6,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF
    ConsumedProvidedServiceInstanceGroupRef                                = 3645,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL
    ConsumedProvidedServiceInstanceGroupRefConditional                     = 5447,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 4692,
    /// CONSUMED-SERVICE-INSTANCE-REF
    ConsumedServiceInstanceRef                                             = 1511,
    /// CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL
    ConsumedServiceInstanceRefConditional                                  = 4306,
    /// CONSUMED-SERVICE-INSTANCES
    ConsumedServiceInstances                                               = 3941,
    /// CONTAINED-AR-ELEMENT-REF
    ContainedArElementRef                                                  = 1181,
    /// CONTAINED-AR-ELEMENT-REFS
    ContainedArElementRefs                                                 = 3465,
    /// CONTAINED-FIBEX-ELEMENT-REF
    ContainedFibexElementRef                                               = 5526,
    /// CONTAINED-FIBEX-ELEMENT-REFS
    ContainedFibexElementRefs                                              = 1562,
    /// CONTAINED-I-PDU-PROPS
    ContainedIPduProps                                                     = 3803,
    /// CONTAINED-I-PDU-TRIGGERING-PROPSS
    ContainedIPduTriggeringPropss                                          = 3166,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REF
    ContainedISignalIPduGroupRef                                           = 5256,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS
    ContainedISignalIPduGroupRefs                                          = 1031,
    /// CONTAINED-PACKAGE-ELEMENT-REF
    ContainedPackageElementRef                                             = 312,
    /// CONTAINED-PACKAGE-ELEMENT-REFS
    ContainedPackageElementRefs                                            = 2762,
    /// CONTAINED-PDU-TRIGGERING-REF
    ContainedPduTriggeringRef                                              = 506,
    /// CONTAINED-PDU-TRIGGERING-REFS
    ContainedPduTriggeringRefs                                             = 5915,
    /// CONTAINED-PROCESS-REF
    ContainedProcessRef                                                    = 3063,
    /// CONTAINED-PROCESS-REFS
    ContainedProcessRefs                                                   = 3809,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 1352,
    /// CONTAINER-I-PDU-HEADER-BYTE-ORDER
    ContainerIPduHeaderByteOrder                                           = 2198,
    /// CONTAINER-TIMEOUT
    ContainerTimeout                                                       = 4965,
    /// CONTAINER-TRIGGER
    ContainerTrigger                                                       = 4957,
    /// CONTAINERS
    Containers                                                             = 5333,
    /// CONTENT-TYPE
    ContentType                                                            = 3260,
    /// CONTENT-URI
    ContentUri                                                             = 1476,
    /// CONTEXT-APPLICATION-RECORD-ELEMENT-REF
    ContextApplicationRecordElementRef                                     = 522,
    /// CONTEXT-ATOMIC-COMPONENT-REF
    ContextAtomicComponentRef                                              = 6177,
    /// CONTEXT-BSW-IMPLEMENTATION-REF
    ContextBswImplementationRef                                            = 1346,
    /// CONTEXT-COMPONENT-PROTOTYPE-REF
    ContextComponentPrototypeRef                                           = 4991,
    /// CONTEXT-COMPONENT-REF
    ContextComponentRef                                                    = 1670,
    /// CONTEXT-COMPONENT-REFS
    ContextComponentRefs                                                   = 4077,
    /// CONTEXT-COMPOSITION-REF
    ContextCompositionRef                                                  = 3807,
    /// CONTEXT-DATA-PROTOTYPE-IN-CS-REF
    ContextDataPrototypeInCsRef                                            = 5295,
    /// CONTEXT-DATA-PROTOTYPE-IN-SR-REF
    ContextDataPrototypeInSrRef                                            = 1412,
    /// CONTEXT-DATA-PROTOTYPE-REF
    ContextDataPrototypeRef                                                = 1738,
    /// CONTEXT-DATA-PROTOTYPE-REFS
    ContextDataPrototypeRefs                                               = 3531,
    /// CONTEXT-DATA-SOURCE
    ContextDataSource                                                      = 4682,
    /// CONTEXT-DATAS
    ContextDatas                                                           = 5488,
    /// CONTEXT-DESCRIPTION
    ContextDescription                                                     = 2888,
    /// CONTEXT-ELEMENT-REF
    ContextElementRef                                                      = 765,
    /// CONTEXT-ELEMENT-REFS
    ContextElementRefs                                                     = 2913,
    /// CONTEXT-ID
    ContextId                                                              = 3130,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF
    ContextImplementationDataElementRef                                    = 5442,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS
    ContextImplementationDataElementRefs                                   = 758,
    /// CONTEXT-LIMITATION-REF
    ContextLimitationRef                                                   = 4224,
    /// CONTEXT-LIMITATION-REFS
    ContextLimitationRefs                                                  = 405,
    /// CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ContextModeDeclarationGroupPrototypeRef                                = 3770,
    /// CONTEXT-MODE-DECLARATION-GROUP-REF
    ContextModeDeclarationGroupRef                                         = 3914,
    /// CONTEXT-OPERATION-REF
    ContextOperationRef                                                    = 3767,
    /// CONTEXT-P-PORT-PROTOTYPE-REF
    ContextPPortPrototypeRef                                               = 4917,
    /// CONTEXT-P-PORT-REF
    ContextPPortRef                                                        = 266,
    /// CONTEXT-PORT-PROTOTYPE-REF
    ContextPortPrototypeRef                                                = 1976,
    /// CONTEXT-PORT-REF
    ContextPortRef                                                         = 1970,
    /// CONTEXT-R-PORT-PROTOTYPE-REF
    ContextRPortPrototypeRef                                               = 1636,
    /// CONTEXT-R-PORT-REF
    ContextRPortRef                                                        = 159,
    /// CONTEXT-REF
    ContextRef                                                             = 5033,
    /// CONTEXT-ROOT-COMPONENT-REF
    ContextRootComponentRef                                                = 2784,
    /// CONTEXT-ROOT-COMPOSITION-REF
    ContextRootCompositionRef                                              = 362,
    /// CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF
    ContextRootSwClusterDesignComponentPrototypeRef                        = 5169,
    /// CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF
    ContextRootSwComponentPrototypeRef                                     = 6023,
    /// CONTEXT-ROOT-SW-COMPOSITION-REF
    ContextRootSwCompositionRef                                            = 791,
    /// CONTEXT-SW-COMPONENT-PROTOTYPE-REF
    ContextSwComponentPrototypeRef                                         = 1349,
    /// CONTEXT-SW-COMPONENT-REF
    ContextSwComponentRef                                                  = 548,
    /// CONTEXT-SWC-PROTOTYPE-REF
    ContextSwcPrototypeRef                                                 = 5528,
    /// CONTEXT-SWC-PROTOTYPE-REFS
    ContextSwcPrototypeRefs                                                = 2664,
    /// CONTEXTS
    Contexts                                                               = 5944,
    /// CONTRACT-VERSION
    ContractVersion                                                        = 4936,
    /// CONTROL-CONSUMED-EVENT-GROUP-REF
    ControlConsumedEventGroupRef                                           = 2412,
    /// CONTROL-CONSUMED-EVENT-GROUP-REFS
    ControlConsumedEventGroupRefs                                          = 3227,
    /// CONTROL-ENABLE-MASK-BITS
    ControlEnableMaskBits                                                  = 2231,
    /// CONTROL-OPTION-RECORD-PRESENT
    ControlOptionRecordPresent                                             = 3664,
    /// CONTROL-PNC-REF
    ControlPncRef                                                          = 4516,
    /// CONTROL-PNC-REFS
    ControlPncRefs                                                         = 4551,
    /// CONTROL-PROVIDED-EVENT-GROUP-REF
    ControlProvidedEventGroupRef                                           = 1109,
    /// CONTROL-PROVIDED-EVENT-GROUP-REFS
    ControlProvidedEventGroupRefs                                          = 3962,
    /// CONTROLLED-DATA-ELEMENT-REF
    ControlledDataElementRef                                               = 1872,
    /// CONTROLLED-DATA-ELEMENT-REFS
    ControlledDataElementRefs                                              = 2515,
    /// CONTROLLED-FID-REF
    ControlledFidRef                                                       = 1358,
    /// CONTROLLED-HW-ELEMENT-REF
    ControlledHwElementRef                                                 = 3138,
    /// CONTROLLER-REF
    ControllerRef                                                          = 4987,
    /// CORE-ID
    CoreId                                                                 = 5703,
    /// CORES
    Cores                                                                  = 3132,
    /// COUNT-PROFILE
    CountProfile                                                           = 2353,
    /// COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE
    CounterBasedFdcThresholdStorageValue                                   = 3789,
    /// COUNTER-DECREMENT-STEP-SIZE
    CounterDecrementStepSize                                               = 3296,
    /// COUNTER-FAILED-THRESHOLD
    CounterFailedThreshold                                                 = 578,
    /// COUNTER-INCREMENT-STEP-SIZE
    CounterIncrementStepSize                                               = 6145,
    /// COUNTER-JUMP-DOWN
    CounterJumpDown                                                        = 5286,
    /// COUNTER-JUMP-DOWN-VALUE
    CounterJumpDownValue                                                   = 3407,
    /// COUNTER-JUMP-UP
    CounterJumpUp                                                          = 1794,
    /// COUNTER-JUMP-UP-VALUE
    CounterJumpUpValue                                                     = 1972,
    /// COUNTER-OFFSET
    CounterOffset                                                          = 5332,
    /// COUNTER-PASSED-THRESHOLD
    CounterPassedThreshold                                                 = 1984,
    /// COUPLED-CLUSTER-REF
    CoupledClusterRef                                                      = 2484,
    /// COUPLED-CLUSTER-REFS
    CoupledClusterRefs                                                     = 5690,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 2559,
    /// COUPLING-ELEMENT-DETAILS
    CouplingElementDetails                                                 = 710,
    /// COUPLING-ELEMENT-SWITCH-DETAILS
    CouplingElementSwitchDetails                                           = 2253,
    /// COUPLING-FLAG
    CouplingFlag                                                           = 3135,
    /// COUPLING-PORT
    CouplingPort                                                           = 5429,
    /// COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER
    CouplingPortAsynchronousTrafficShaper                                  = 4198,
    /// COUPLING-PORT-CONNECTION
    CouplingPortConnection                                                 = 3312,
    /// COUPLING-PORT-CONNECTIONS
    CouplingPortConnections                                                = 1195,
    /// COUPLING-PORT-CREDIT-BASED-SHAPER
    CouplingPortCreditBasedShaper                                          = 2511,
    /// COUPLING-PORT-DETAILS
    CouplingPortDetails                                                    = 188,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 6230,
    /// COUPLING-PORT-RATE-POLICY
    CouplingPortRatePolicy                                                 = 866,
    /// COUPLING-PORT-REF
    CouplingPortRef                                                        = 4750,
    /// COUPLING-PORT-REF-CONDITIONAL
    CouplingPortRefConditional                                             = 1965,
    /// COUPLING-PORT-ROLE
    CouplingPortRole                                                       = 4359,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 563,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 5290,
    /// COUPLING-PORT-SPEED
    CouplingPortSpeed                                                      = 3456,
    /// COUPLING-PORT-STARTUP-ACTIVE-TIME
    CouplingPortStartupActiveTime                                          = 3814,
    /// COUPLING-PORT-STRUCTURAL-ELEMENTS
    CouplingPortStructuralElements                                         = 6249,
    /// COUPLING-PORT-SWITCHOFF-DELAY
    CouplingPortSwitchoffDelay                                             = 452,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 775,
    /// COUPLING-PORTS
    CouplingPorts                                                          = 5912,
    /// COUPLING-TYPE
    CouplingType                                                           = 1554,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 3705,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 285,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 1100,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 3845,
    /// CP-SOFTWARE-CLUSTER-REF
    CpSoftwareClusterRef                                                   = 4550,
    /// CP-SOFTWARE-CLUSTER-REF-CONDITIONAL
    CpSoftwareClusterRefConditional                                        = 3343,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 5189,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-REF
    CpSoftwareClusterResourceRef                                           = 2391,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 3237,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 4485,
    /// CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterToApplicationPartitionMapping                         = 6117,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 71,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 6130,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 2719,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 1377,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 32,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 4080,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 2957,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 4,
    /// CPP-TEMPLATE-ARGUMENT
    CppTemplateArgument                                                    = 3901,
    /// CPU-USAGE
    CpuUsage                                                               = 2606,
    /// CRC-CORRECTION-FIELD
    CrcCorrectionField                                                     = 4165,
    /// CRC-DOMAIN-NUMBER
    CrcDomainNumber                                                        = 1490,
    /// CRC-FLAGS
    CrcFlags                                                               = 5626,
    /// CRC-MESSAGE-LENGTH
    CrcMessageLength                                                       = 5278,
    /// CRC-OFFSET
    CrcOffset                                                              = 3169,
    /// CRC-PRECISE-ORIGIN-TIMESTAMP
    CrcPreciseOriginTimestamp                                              = 686,
    /// CRC-SECURED
    CrcSecured                                                             = 1936,
    /// CRC-SEQUENCE-ID
    CrcSequenceId                                                          = 2755,
    /// CRC-SOURCE-PORT-IDENTITY
    CrcSourcePortIdentity                                                  = 1514,
    /// CRC-VALIDATED
    CrcValidated                                                           = 1064,
    /// CREATE-ECU-WAKEUP-SOURCE
    CreateEcuWakeupSource                                                  = 3752,
    /// CREATED-DATAS
    CreatedDatas                                                           = 2500,
    /// CRF-PULL
    CrfPull                                                                = 1029,
    /// CRF-TYPE
    CrfType                                                                = 5327,
    /// CRYPTO-ALG-ID
    CryptoAlgId                                                            = 1439,
    /// CRYPTO-ALGO-CONFIG
    CryptoAlgoConfig                                                       = 3573,
    /// CRYPTO-ALGORITHM-STRING
    CryptoAlgorithmString                                                  = 1723,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 2163,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 3546,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 605,
    /// CRYPTO-CERTIFICATE-REF
    CryptoCertificateRef                                                   = 3833,
    /// CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING
    CryptoCertificateToCryptoKeySlotMapping                                = 5268,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 5999,
    /// CRYPTO-CERTIFICATES
    CryptoCertificates                                                     = 1518,
    /// CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS
    CryptoDesignToCryptoDriverMappings                                     = 3018,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 2138,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING
    CryptoDriverToCryptoJobMapping                                         = 4448,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS
    CryptoDriverToCryptoJobMappings                                        = 460,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 4899,
    /// CRYPTO-JOB
    CryptoJob                                                              = 378,
    /// CRYPTO-JOB-REF
    CryptoJobRef                                                           = 2798,
    /// CRYPTO-JOBS
    CryptoJobs                                                             = 5326,
    /// CRYPTO-KEY-DESCRIPTION
    CryptoKeyDescription                                                   = 3203,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 6128,
    /// CRYPTO-KEY-REF
    CryptoKeyRef                                                           = 91,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 1649,
    /// CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE
    CryptoKeySlotContentAllowedUsage                                       = 812,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 5815,
    /// CRYPTO-KEY-SLOT-REF
    CryptoKeySlotRef                                                       = 877,
    /// CRYPTO-KEY-SLOT-REFS
    CryptoKeySlotRefs                                                      = 6280,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 2750,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 4221,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 4645,
    /// CRYPTO-NEED-REF
    CryptoNeedRef                                                          = 5220,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 4361,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 3117,
    /// CRYPTO-OBJECT-TYPE
    CryptoObjectType                                                       = 949,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 5,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 1790,
    /// CRYPTO-PROVIDER-DOCUMENTATION-REF
    CryptoProviderDocumentationRef                                         = 4983,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 1625,
    /// CRYPTO-PROVIDER-REF
    CryptoProviderRef                                                      = 1677,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 940,
    /// CRYPTO-PROVIDERS
    CryptoProviders                                                        = 4239,
    /// CRYPTO-R-PORT-COM-SPEC
    CryptoRPortComSpec                                                     = 4459,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 371,
    /// CRYPTO-SERVICE-CERTIFICATE-REF
    CryptoServiceCertificateRef                                            = 322,
    /// CRYPTO-SERVICE-CERTIFICATE-REFS
    CryptoServiceCertificateRefs                                           = 2653,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 1310,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 4532,
    /// CRYPTO-SERVICE-KEY-REF
    CryptoServiceKeyRef                                                    = 1027,
    /// CRYPTO-SERVICE-MAPPINGS
    CryptoServiceMappings                                                  = 2415,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 3353,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 1044,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 3522,
    /// CRYPTO-SERVICE-QUEUE-REF
    CryptoServiceQueueRef                                                  = 2494,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 905,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 2383,
    /// CS-ERROR-REACTION
    CsErrorReaction                                                        = 556,
    /// CSE-CODE
    CseCode                                                                = 4580,
    /// CSE-CODE-FACTOR
    CseCodeFactor                                                          = 6020,
    /// CURRENT-DATA-TYPE-REF
    CurrentDataTypeRef                                                     = 402,
    /// CURRENT-EXP
    CurrentExp                                                             = 2919,
    /// CURRENT-VALUE-REF
    CurrentValueRef                                                        = 3467,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 2401,
    /// CUSTOM-DOCUMENT-ELEMENT-REF
    CustomDocumentElementRef                                               = 4865,
    /// CUSTOM-DOCUMENTATION-REF
    CustomDocumentationRef                                                 = 2744,
    /// CUSTOM-SDG-DEF-REF
    CustomSdgDefRef                                                        = 4874,
    /// CUSTOM-SDG-DEF-REFS
    CustomSdgDefRefs                                                       = 4542,
    /// CUSTOM-SERVICE-CLASS-REF
    CustomServiceClassRef                                                  = 5463,
    /// CUSTOM-SERVICE-ID
    CustomServiceId                                                        = 3796,
    /// CUSTOM-SPECIFICATION-REF
    CustomSpecificationRef                                                 = 4049,
    /// CUSTOM-SPECIFICATION-REFS
    CustomSpecificationRefs                                                = 1154,
    /// CUSTOM-SUB-FUNCTION-NUMBER
    CustomSubFunctionNumber                                                = 364,
    /// CUSTOM-TRIGGER
    CustomTrigger                                                          = 2009,
    /// CYCLE
    Cycle                                                                  = 3286,
    /// CYCLE-AUTOSTART
    CycleAutostart                                                         = 5578,
    /// CYCLE-COUNT-MAX
    CycleCountMax                                                          = 1300,
    /// CYCLE-COUNTER
    CycleCounter                                                           = 1718,
    /// CYCLE-REPETITION
    CycleRepetition                                                        = 3942,
    /// CYCLE-STATUS-STORAGE
    CycleStatusStorage                                                     = 2089,
    /// CYCLE-TIME-MAIN-FUNCTION
    CycleTimeMainFunction                                                  = 4685,
    /// CYCLE-TIME-VALUE
    CycleTimeValue                                                         = 463,
    /// CYCLIC-BACKUP-INTERVAL
    CyclicBackupInterval                                                   = 2358,
    /// CYCLIC-TIMING
    CyclicTiming                                                           = 2443,
    /// CYCLIC-WRITING-PERIOD
    CyclicWritingPeriod                                                    = 4861,
    /// DATA-COM-PROPS
    DataComProps                                                           = 1298,
    /// DATA-CONSISTENCY-POLICY
    DataConsistencyPolicy                                                  = 4633,
    /// DATA-CONSTR
    DataConstr                                                             = 1085,
    /// DATA-CONSTR-REF
    DataConstrRef                                                          = 1495,
    /// DATA-CONSTR-RULE
    DataConstrRule                                                         = 4273,
    /// DATA-CONSTR-RULES
    DataConstrRules                                                        = 3769,
    /// DATA-DUMP-ENTRY
    DataDumpEntry                                                          = 559,
    /// DATA-ELEMENT-IREF
    DataElementIref                                                        = 3900,
    /// DATA-ELEMENT-IREFS
    DataElementIrefs                                                       = 5205,
    /// DATA-ELEMENT-REF
    DataElementRef                                                         = 542,
    /// DATA-ELEMENT-REFS
    DataElementRefs                                                        = 3853,
    /// DATA-ELEMENTS
    DataElements                                                           = 5050,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 1193,
    /// DATA-FILTER
    DataFilter                                                             = 2301,
    /// DATA-FILTER-TYPE
    DataFilterType                                                         = 5009,
    /// DATA-FORMAT-TAILORING
    DataFormatTailoring                                                    = 3812,
    /// DATA-ID
    DataId                                                                 = 1229,
    /// DATA-ID-MODE
    DataIdMode                                                             = 3338,
    /// DATA-ID-NIBBLE-OFFSET
    DataIdNibbleOffset                                                     = 4934,
    /// DATA-IDENTIFIER-REF
    DataIdentifierRef                                                      = 175,
    /// DATA-IDENTIFIER-REFS
    DataIdentifierRefs                                                     = 5215,
    /// DATA-IDS
    DataIds                                                                = 1957,
    /// DATA-IREF
    DataIref                                                               = 2478,
    /// DATA-LENGTH
    DataLength                                                             = 3365,
    /// DATA-LENGTH-REQUEST
    DataLengthRequest                                                      = 4942,
    /// DATA-LENGTH-RESPONSE
    DataLengthResponse                                                     = 1299,
    /// DATA-LINK-LAYER-RULE
    DataLinkLayerRule                                                      = 1981,
    /// DATA-MAPPINGS
    DataMappings                                                           = 1259,
    /// DATA-OFFSET
    DataOffset                                                             = 2582,
    /// DATA-PDU-REF
    DataPduRef                                                             = 719,
    /// DATA-PROTECTION-KIND
    DataProtectionKind                                                     = 4646,
    /// DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF
    DataProtototypeInPortInterfaceRef                                      = 692,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 5204,
    /// DATA-PROTOTYPE-GROUP-IREF
    DataPrototypeGroupIref                                                 = 1081,
    /// DATA-PROTOTYPE-GROUP-IREFS
    DataPrototypeGroupIrefs                                                = 1009,
    /// DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF
    DataPrototypeInClientServerInterfaceIref                               = 4251,
    /// DATA-PROTOTYPE-IN-METHOD-ARGUMENT-INSTANCE-REF-IREF
    DataPrototypeInMethodArgumentInstanceRefIref                           = 5237,
    /// DATA-PROTOTYPE-IN-PORT-INTERFACE-REF
    DataPrototypeInPortInterfaceRef                                        = 2289,
    /// DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF
    DataPrototypeInSenderReceiverInterfaceIref                             = 3743,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF
    DataPrototypeInServiceInterfaceIref                                    = 2838,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF
    DataPrototypeInServiceInterfaceRef                                     = 1168,
    /// DATA-PROTOTYPE-IREF
    DataPrototypeIref                                                      = 5305,
    /// DATA-PROTOTYPE-MAPPING
    DataPrototypeMapping                                                   = 3004,
    /// DATA-PROTOTYPE-REF
    DataPrototypeRef                                                       = 5356,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPS
    DataPrototypeTransformationProps                                       = 4179,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPSS
    DataPrototypeTransformationPropss                                      = 196,
    /// DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF
    DataPrototypeWithApplicationDataTypeInSystemRef                        = 1760,
    /// DATA-PROTOTYPES
    DataPrototypes                                                         = 2441,
    /// DATA-PROVIDER
    DataProvider                                                           = 2112,
    /// DATA-READ-ACCESSS
    DataReadAccesss                                                        = 2594,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 289,
    /// DATA-RECEIVE-POINT-BY-ARGUMENTS
    DataReceivePointByArguments                                            = 5887,
    /// DATA-RECEIVE-POINT-BY-VALUES
    DataReceivePointByValues                                               = 5616,
    /// DATA-RECEIVE-POINTS
    DataReceivePoints                                                      = 4958,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 3350,
    /// DATA-REF
    DataRef                                                                = 5388,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 342,
    /// DATA-SEND-POINTS
    DataSendPoints                                                         = 2423,
    /// DATA-TRANSFER-CLASS-REF
    DataTransferClassRef                                                   = 3435,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 2165,
    /// DATA-TRANSFORMATION-KIND
    DataTransformationKind                                                 = 6160,
    /// DATA-TRANSFORMATION-REF
    DataTransformationRef                                                  = 4438,
    /// DATA-TRANSFORMATION-REF-CONDITIONAL
    DataTransformationRefConditional                                       = 2615,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 5752,
    /// DATA-TRANSFORMATIONS
    DataTransformations                                                    = 2915,
    /// DATA-TYPE-FOR-SERIALIZATION-REF
    DataTypeForSerializationRef                                            = 4487,
    /// DATA-TYPE-FOR-SERIALIZATION-REFS
    DataTypeForSerializationRefs                                           = 422,
    /// DATA-TYPE-MAP
    DataTypeMap                                                            = 4102,
    /// DATA-TYPE-MAPPING
    DataTypeMapping                                                        = 5810,
    /// DATA-TYPE-MAPPING-REF
    DataTypeMappingRef                                                     = 2150,
    /// DATA-TYPE-MAPPING-REFS
    DataTypeMappingRefs                                                    = 4146,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 471,
    /// DATA-TYPE-MAPPING-SET-REF
    DataTypeMappingSetRef                                                  = 3469,
    /// DATA-TYPE-MAPPING-SET-REFS
    DataTypeMappingSetRefs                                                 = 157,
    /// DATA-TYPE-MAPS
    DataTypeMaps                                                           = 374,
    /// DATA-TYPE-POLICY
    DataTypePolicy                                                         = 4795,
    /// DATA-TYPE-REF
    DataTypeRef                                                            = 1215,
    /// DATA-TYPE-REFS
    DataTypeRefs                                                           = 2754,
    /// DATA-UPDATE-PERIOD
    DataUpdatePeriod                                                       = 5068,
    /// DATA-WRITE-ACCESSS
    DataWriteAccesss                                                       = 4152,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 2221,
    /// DATE
    Date                                                                   = 2933,
    /// DCM-I-PDU
    DcmIPdu                                                                = 4226,
    /// DDS-CP-CONFIG
    DdsCpConfig                                                            = 4063,
    /// DDS-CP-CONSUMED-SERVICE-INSTANCE
    DdsCpConsumedServiceInstance                                           = 837,
    /// DDS-CP-DOMAIN
    DdsCpDomain                                                            = 335,
    /// DDS-CP-I-SIGNAL-TO-DDS-TOPIC-MAPPING
    DdsCpISignalToDdsTopicMapping                                          = 6084,
    /// DDS-CP-PARTITION
    DdsCpPartition                                                         = 1517,
    /// DDS-CP-PROVIDED-SERVICE-INSTANCE
    DdsCpProvidedServiceInstance                                           = 416,
    /// DDS-CP-QOS-PROFILE
    DdsCpQosProfile                                                        = 205,
    /// DDS-CP-SERVICE-INSTANCE-EVENT
    DdsCpServiceInstanceEvent                                              = 174,
    /// DDS-CP-SERVICE-INSTANCE-OPERATION
    DdsCpServiceInstanceOperation                                          = 715,
    /// DDS-CP-TOPIC
    DdsCpTopic                                                             = 1008,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 262,
    /// DDS-DOMAINS
    DdsDomains                                                             = 4766,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 4768,
    /// DDS-EVENT-QOS-PROFILE-REF
    DdsEventQosProfileRef                                                  = 3643,
    /// DDS-EVENT-QOS-PROPS
    DdsEventQosProps                                                       = 1322,
    /// DDS-EVENT-REF
    DdsEventRef                                                            = 677,
    /// DDS-EVENT-TOPIC-REF
    DdsEventTopicRef                                                       = 4858,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 456,
    /// DDS-FIELD-QOS-PROPS
    DdsFieldQosProps                                                       = 2421,
    /// DDS-FIELD-REPLY-TOPIC-REF
    DdsFieldReplyTopicRef                                                  = 626,
    /// DDS-FIELD-REQUEST-TOPIC-REF
    DdsFieldRequestTopicRef                                                = 2917,
    /// DDS-I-SIGNAL-TO-TOPIC-MAPPINGS
    DdsISignalToTopicMappings                                              = 1285,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 3425,
    /// DDS-METHOD-QOS-PROPS
    DdsMethodQosProps                                                      = 4975,
    /// DDS-METHOD-REPLY-TOPIC-REF
    DdsMethodReplyTopicRef                                                 = 5981,
    /// DDS-METHOD-REQUEST-TOPIC-REF
    DdsMethodRequestTopicRef                                               = 4678,
    /// DDS-OPERATION-REQUEST-TRIGGERING-REF
    DdsOperationRequestTriggeringRef                                       = 3016,
    /// DDS-OPERATION-RESPONSE-TRIGGERING-REF
    DdsOperationResponseTriggeringRef                                      = 652,
    /// DDS-PARTITION-REF
    DdsPartitionRef                                                        = 639,
    /// DDS-PARTITIONS
    DdsPartitions                                                          = 252,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 6096,
    /// DDS-QOS-PROFILE-REF
    DdsQosProfileRef                                                       = 5214,
    /// DDS-QOS-PROFILES
    DdsQosProfiles                                                         = 3269,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 2306,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 4816,
    /// DDS-RPC-SERVICE-REF
    DdsRpcServiceRef                                                       = 3930,
    /// DDS-RPC-SERVICES
    DdsRpcServices                                                         = 4791,
    /// DDS-RULE
    DdsRule                                                                = 3041,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 3824,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 2998,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 233,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 2509,
    /// DDS-SERVICE-QOS-PROFILE-REF
    DdsServiceQosProfileRef                                                = 3956,
    /// DDS-SERVICE-VERSION
    DdsServiceVersion                                                      = 5452,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 4151,
    /// DDS-TOPIC-REF
    DdsTopicRef                                                            = 5289,
    /// DDS-TOPICS
    DdsTopics                                                              = 5336,
    /// DEADLINE
    Deadline                                                               = 3029,
    /// DEADLINE-MAX
    DeadlineMax                                                            = 1714,
    /// DEADLINE-MIN
    DeadlineMin                                                            = 5717,
    /// DEADLINE-PERIOD
    DeadlinePeriod                                                         = 3085,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 5874,
    /// DEADLINE-SUPERVISIONS
    DeadlineSupervisions                                                   = 761,
    /// DEBOUNCE-ALGORITHM
    DebounceAlgorithm                                                      = 4998,
    /// DEBOUNCE-ALGORITHM-PROPSS
    DebounceAlgorithmPropss                                                = 4105,
    /// DEBOUNCE-ALGORITHM-REF
    DebounceAlgorithmRef                                                   = 2419,
    /// DEBOUNCE-BEHAVIOR
    DebounceBehavior                                                       = 1761,
    /// DEBOUNCE-COUNTER-STORAGE
    DebounceCounterStorage                                                 = 3648,
    /// DEBOUNCE-TIME
    DebounceTime                                                           = 1139,
    /// DEBUG-INFOS
    DebugInfos                                                             = 3377,
    /// DECODING-CORRECTION
    DecodingCorrection                                                     = 1697,
    /// DECOMPOSITIONS
    Decompositions                                                         = 1764,
    /// DEF
    Def                                                                    = 4644,
    /// DEF-CALPRM-SET
    DefCalprmSet                                                           = 3175,
    /// DEF-ITEM
    DefItem                                                                = 5451,
    /// DEF-LIST
    DefList                                                                = 2411,
    /// DEFAULT-ACTION
    DefaultAction                                                          = 4897,
    /// DEFAULT-APPLICATION-TIMEOUT
    DefaultApplicationTimeout                                              = 4614,
    /// DEFAULT-ENDIANNESS
    DefaultEndianness                                                      = 6087,
    /// DEFAULT-GATEWAY
    DefaultGateway                                                         = 1945,
    /// DEFAULT-LC-STATE-REF
    DefaultLcStateRef                                                      = 5619,
    /// DEFAULT-LEASE-TIME
    DefaultLeaseTime                                                       = 72,
    /// DEFAULT-LOG-THRESHOLD
    DefaultLogThreshold                                                    = 1619,
    /// DEFAULT-MODE-REF
    DefaultModeRef                                                         = 3193,
    /// DEFAULT-PATTERN
    DefaultPattern                                                         = 4428,
    /// DEFAULT-PERIOD-BEGIN
    DefaultPeriodBegin                                                     = 3996,
    /// DEFAULT-PERIOD-END
    DefaultPeriodEnd                                                       = 3624,
    /// DEFAULT-PRIORITY
    DefaultPriority                                                        = 5255,
    /// DEFAULT-REPORTING-MODE
    DefaultReportingMode                                                   = 4727,
    /// DEFAULT-ROUTER
    DefaultRouter                                                          = 1884,
    /// DEFAULT-TRACE-STATE
    DefaultTraceState                                                      = 5566,
    /// DEFAULT-VALUE
    DefaultValue                                                           = 4439,
    /// DEFAULT-VALUE-ELEMENT
    DefaultValueElement                                                    = 6266,
    /// DEFAULT-VALUE-ELEMENTS
    DefaultValueElements                                                   = 4440,
    /// DEFAULT-VALUE-HANDLING
    DefaultValueHandling                                                   = 1529,
    /// DEFAULT-VLAN-REF
    DefaultVlanRef                                                         = 380,
    /// DEFERRING-FID-REF
    DeferringFidRef                                                        = 3441,
    /// DEFERRING-FID-REFS
    DeferringFidRefs                                                       = 3064,
    /// DEFINITION-REF
    DefinitionRef                                                          = 4499,
    /// DELAY
    Delay                                                                  = 5298,
    /// DELAY-COMPENSATION-A
    DelayCompensationA                                                     = 4630,
    /// DELAY-COMPENSATION-B
    DelayCompensationB                                                     = 2450,
    /// DELEGATED-PORT-ANNOTATION
    DelegatedPortAnnotation                                                = 5588,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 5514,
    /// DELETES
    Deletes                                                                = 3521,
    /// DELIVERY-ARTIFACTS
    DeliveryArtifacts                                                      = 613,
    /// DELTA-PACKAGE-APPLICABLE-VERSION
    DeltaPackageApplicableVersion                                          = 3382,
    /// DENOMINATOR-CONDITION
    DenominatorCondition                                                   = 2177,
    /// DENOMINATOR-GROUP
    DenominatorGroup                                                       = 5340,
    /// DEPENDENCIES
    Dependencies                                                           = 2269,
    /// DEPENDENCY
    Dependency                                                             = 4774,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 2183,
    /// DEPENDENT-RESOURCES
    DependentResources                                                     = 5847,
    /// DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF
    DependentSoftwareClusterDesignRef                                      = 3759,
    /// DEPENDENT-SOFTWARE-CLUSTER-REF
    DependentSoftwareClusterRef                                            = 4923,
    /// DEPENDS-ON
    DependsOn                                                              = 533,
    /// DEPENDS-ON-REF
    DependsOnRef                                                           = 3630,
    /// DEPENDS-ON-REFS
    DependsOnRefs                                                          = 5233,
    /// DEPENDS-ONS
    DependsOns                                                             = 797,
    /// DEPLOYMENT-REF
    DeploymentRef                                                          = 2535,
    /// DEPLOYMENT-URIS
    DeploymentUris                                                         = 5675,
    /// DERIVATION
    Derivation                                                             = 4141,
    /// DERIVED-CERTIFICATE-ACCEPTED
    DerivedCertificateAccepted                                             = 856,
    /// DERIVED-FROM-BLUEPRINT-REF
    DerivedFromBlueprintRef                                                = 419,
    /// DERIVED-FROM-BLUEPRINT-REFS
    DerivedFromBlueprintRefs                                               = 446,
    /// DERIVED-INTERFACE-REF
    DerivedInterfaceRef                                                    = 4103,
    /// DERIVED-OBJECT-REF
    DerivedObjectRef                                                       = 2545,
    /// DERIVED-PORT-INTERFACE-REF
    DerivedPortInterfaceRef                                                = 950,
    /// DERIVED-PORT-PROTOTYPE-REF
    DerivedPortPrototypeRef                                                = 3751,
    /// DESC
    Desc                                                                   = 4799,
    /// DESCRIPTION
    Description                                                            = 4938,
    /// DESIGN-REF
    DesignRef                                                              = 2290,
    /// DESIGN-REFS
    DesignRefs                                                             = 893,
    /// DEST-META-CLASS
    DestMetaClass                                                          = 1125,
    /// DEST-SDG-REF
    DestSdgRef                                                             = 1644,
    /// DESTINATION-BASE-ID
    DestinationBaseId                                                      = 3351,
    /// DESTINATION-CONTEXT
    DestinationContext                                                     = 1607,
    /// DESTINATION-IP-ADDRESS
    DestinationIpAddress                                                   = 5183,
    /// DESTINATION-IPV-4-ADDRESS
    DestinationIpv4Address                                                 = 804,
    /// DESTINATION-IPV-6-ADDRESS
    DestinationIpv6Address                                                 = 3503,
    /// DESTINATION-MAC-ADDRESS
    DestinationMacAddress                                                  = 4447,
    /// DESTINATION-MAC-ADDRESS-MASK
    DestinationMacAddressMask                                              = 1417,
    /// DESTINATION-MAX-ADDRESS
    DestinationMaxAddress                                                  = 3419,
    /// DESTINATION-MIN-ADDRESS
    DestinationMinAddress                                                  = 4206,
    /// DESTINATION-NETWORK-MASK
    DestinationNetworkMask                                                 = 702,
    /// DESTINATION-ORDER
    DestinationOrder                                                       = 1058,
    /// DESTINATION-ORDER-KIND
    DestinationOrderKind                                                   = 2226,
    /// DESTINATION-PHYSICAL-ADDRESS
    DestinationPhysicalAddress                                             = 2345,
    /// DESTINATION-PORTS
    DestinationPorts                                                       = 3412,
    /// DESTINATION-REF
    DestinationRef                                                         = 4323,
    /// DESTINATION-REFS
    DestinationRefs                                                        = 5935,
    /// DESTINATION-TYPE
    DestinationType                                                        = 2793,
    /// DESTINATION-URI-DEFS
    DestinationUriDefs                                                     = 1070,
    /// DESTINATION-URI-NESTING-CONTRACT
    DestinationUriNestingContract                                          = 5840,
    /// DESTINATION-URI-POLICY
    DestinationUriPolicy                                                   = 5191,
    /// DESTINATION-URI-REF
    DestinationUriRef                                                      = 5970,
    /// DESTINATION-URI-REFS
    DestinationUriRefs                                                     = 2965,
    /// DETECT-NIT-ERROR
    DetectNitError                                                         = 282,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 1702,
    /// DETERMINISTIC-CLIENT-REF
    DeterministicClientRef                                                 = 3360,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 60,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDSS
    DeterministicClientResourceNeedss                                      = 925,
    /// DETERMINISTIC-SYNC-M-OUT-OF-N
    DeterministicSyncMOutOfN                                               = 3785,
    /// DETERMINISTIC-SYNC-MASTER
    DeterministicSyncMaster                                                = 2071,
    /// DETERMINISTIC-SYNC-MASTER-REF
    DeterministicSyncMasterRef                                             = 2853,
    /// DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING
    DeterministicSyncMasterToTimeBaseConsumerMapping                       = 5226,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 6151,
    /// DEVELOPMENT-VALUE
    DevelopmentValue                                                       = 1011,
    /// DHCP-ADDRESS-ASSIGNMENT
    DhcpAddressAssignment                                                  = 4671,
    /// DHCP-PROPS
    DhcpProps                                                              = 431,
    /// DHCP-SERVER-CONFIGURATION
    DhcpServerConfiguration                                                = 860,
    /// DIAG-ARG-INTEGRITY
    DiagArgIntegrity                                                       = 3145,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 2800,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 4472,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 5067,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 5365,
    /// DIAG-FAULT-DETECTION-COUNTER-PORT-REF
    DiagFaultDetectionCounterPortRef                                       = 3615,
    /// DIAG-PDU-TYPE
    DiagPduType                                                            = 5012,
    /// DIAG-REQUIREMENT
    DiagRequirement                                                        = 882,
    /// DIAG-ROUTINE-TYPE
    DiagRoutineType                                                        = 48,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 2951,
    /// DIAGNOSTIC-ADDRESS
    DiagnosticAddress                                                      = 5489,
    /// DIAGNOSTIC-ADDRESSES
    DiagnosticAddresses                                                    = 2016,
    /// DIAGNOSTIC-ADDRESSS
    DiagnosticAddresss                                                     = 2654,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 5746,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 2288,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE
    DiagnosticAuthTransmitCertificate                                      = 42,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION
    DiagnosticAuthTransmitCertificateEvaluation                            = 6099,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING
    DiagnosticAuthTransmitCertificateMapping                               = 2185,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 1821,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 2003,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 2292,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 2470,
    /// DIAGNOSTIC-AUTHENTICATION-REF
    DiagnosticAuthenticationRef                                            = 2970,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 3258,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 1843,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 4872,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 3183,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 1103,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 2534,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 4519,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 1423,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 870,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 360,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 3987,
    /// DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL
    DiagnosticComControlSpecificChannel                                    = 4792,
    /// DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL
    DiagnosticComControlSubNodeChannel                                     = 679,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF
    DiagnosticCommonElementRef                                             = 5958,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL
    DiagnosticCommonElementRefConditional                                  = 2830,
    /// DIAGNOSTIC-COMMON-PROPS-CONDITIONAL
    DiagnosticCommonPropsConditional                                       = 3817,
    /// DIAGNOSTIC-COMMON-PROPS-VARIANTS
    DiagnosticCommonPropsVariants                                          = 5291,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 2313,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 1691,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 3951,
    /// DIAGNOSTIC-CONDITION-REF
    DiagnosticConditionRef                                                 = 438,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 326,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 1823,
    /// DIAGNOSTIC-CONNECTION-REF
    DiagnosticConnectionRef                                                = 2076,
    /// DIAGNOSTIC-CONNECTION-REF-CONDITIONAL
    DiagnosticConnectionRefConditional                                     = 3718,
    /// DIAGNOSTIC-CONNECTIONS
    DiagnosticConnections                                                  = 965,
    /// DIAGNOSTIC-CONTRIBUTION-REF
    DiagnosticContributionRef                                              = 2255,
    /// DIAGNOSTIC-CONTRIBUTION-REFS
    DiagnosticContributionRefs                                             = 4549,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 3212,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 4364,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 5087,
    /// DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT
    DiagnosticControlEnableMaskBit                                         = 1060,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 4423,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 4160,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 2164,
    /// DIAGNOSTIC-DATA-CHANGE-TRIGGER
    DiagnosticDataChangeTrigger                                            = 3024,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 4220,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 756,
    /// DIAGNOSTIC-DATA-ELEMENT-REF
    DiagnosticDataElementRef                                               = 1904,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 3133,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 2364,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 6071,
    /// DIAGNOSTIC-DATA-IDENTIFIER-REF
    DiagnosticDataIdentifierRef                                            = 5930,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 4433,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF
    DiagnosticDataIdentifierSetRef                                         = 5837,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL
    DiagnosticDataIdentifierSetRefConditional                              = 126,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 3813,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 5355,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 363,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 4758,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 412,
    /// DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT
    DiagnosticDebounceBehaviorEnumValueVariationPoint                      = 6194,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 933,
    /// DIAGNOSTIC-DEPLOYMENT-PROPS-REF
    DiagnosticDeploymentPropsRef                                           = 3433,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 1453,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE
    DiagnosticDoIpEntityIdentificationInterface                            = 5572,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 535,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 4970,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 5646,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 3786,
    /// DIAGNOSTIC-DTC-CHANGE-TRIGGER
    DiagnosticDtcChangeTrigger                                             = 2124,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 35,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 4479,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 982,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 1826,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 4744,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 1492,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 5413,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 5522,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 5515,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 6093,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 4318,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 3317,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF
    DiagnosticEnableConditionRef                                           = 2941,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL
    DiagnosticEnableConditionRefConditional                                = 1053,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 418,
    /// DIAGNOSTIC-ENV-CONDITION-FORMULA
    DiagnosticEnvConditionFormula                                          = 4654,
    /// DIAGNOSTIC-ENV-DATA-CONDITION
    DiagnosticEnvDataCondition                                             = 5239,
    /// DIAGNOSTIC-ENV-DATA-ELEMENT-CONDITION
    DiagnosticEnvDataElementCondition                                      = 1755,
    /// DIAGNOSTIC-ENV-MODE-CONDITION
    DiagnosticEnvModeCondition                                             = 4278,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 650,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 5345,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 3137,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 6088,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 2357,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 1328,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 4316,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 2029,
    /// DIAGNOSTIC-EVENT-REF
    DiagnosticEventRef                                                     = 3264,
    /// DIAGNOSTIC-EVENT-REF-CONDITIONAL
    DiagnosticEventRefConditional                                          = 5287,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 3369,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 3013,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 4896,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 5094,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 1875,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 2849,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 4214,
    /// DIAGNOSTIC-EVENT-WINDOW
    DiagnosticEventWindow                                                  = 292,
    /// DIAGNOSTIC-EVENTS
    DiagnosticEvents                                                       = 2544,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 5672,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF
    DiagnosticExtendedDataRecordRef                                        = 2728,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL
    DiagnosticExtendedDataRecordRefConditional                             = 4071,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION
    DiagnosticExternalAuthenticationIdentification                         = 4951,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 2647,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 1451,
    /// DIAGNOSTIC-EXTRACT-REF
    DiagnosticExtractRef                                                   = 2212,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 4347,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 219,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 4272,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 6271,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 1140,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 3915,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 4356,
    /// DIAGNOSTIC-FREEZE-FRAME-REF
    DiagnosticFreezeFrameRef                                               = 4856,
    /// DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL
    DiagnosticFreezeFrameRefConditional                                    = 6155,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 3709,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 2665,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 4332,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 6049,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 5019,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 5140,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 3960,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 1786,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 6219,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 6131,
    /// DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT
    DiagnosticIndicatorTypeEnumValueVariationPoint                         = 5998,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 2399,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 3712,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 3106,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 868,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 1573,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 4145,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 6031,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 255,
    /// DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER
    DiagnosticIumprGroupIdentifier                                         = 1077,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 5919,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 4718,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 1210,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 2783,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 504,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 140,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 5711,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 4132,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 4031,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 5249,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 900,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 5473,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 519,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 6027,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 696,
    /// DIAGNOSTIC-MEMORY-REF
    DiagnosticMemoryRef                                                    = 4007,
    /// DIAGNOSTIC-MESSAGE-TYPE
    DiagnosticMessageType                                                  = 4597,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 4396,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 1863,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE
    DiagnosticMultipleConditionInterface                                   = 5706,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING
    DiagnosticMultipleConditionPortMapping                                 = 424,
    /// DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE
    DiagnosticMultipleEventInterface                                       = 4909,
    /// DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING
    DiagnosticMultipleEventPortMapping                                     = 1089,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE
    DiagnosticMultipleMonitorInterface                                     = 1921,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING
    DiagnosticMultipleMonitorPortMapping                                   = 5387,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 2346,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 5002,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 595,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 1368,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF
    DiagnosticOperationCycleRef                                            = 4994,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL
    DiagnosticOperationCycleRefConditional                                 = 4199,
    /// DIAGNOSTIC-PARAMETER
    DiagnosticParameter                                                    = 2095,
    /// DIAGNOSTIC-PARAMETER-ELEMENT
    DiagnosticParameterElement                                             = 3125,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 4267,
    /// DIAGNOSTIC-PARAMETER-REF
    DiagnosticParameterRef                                                 = 4309,
    /// DIAGNOSTIC-PERIODIC-RATE
    DiagnosticPeriodicRate                                                 = 5194,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 5760,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 101,
    /// DIAGNOSTIC-PROPS
    DiagnosticProps                                                        = 1722,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 3367,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 4977,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 3329,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 4523,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 5824,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 1327,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 4961,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 5121,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 2265,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 2505,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 1004,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 4338,
    /// DIAGNOSTIC-RELEVANCE
    DiagnosticRelevance                                                    = 5553,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 2410,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 1115,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 763,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 4704,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 5548,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 6015,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 6119,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 3651,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 2315,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 1473,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 3439,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 3151,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE
    DiagnosticRequestFileTransferInterface                                 = 80,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 4937,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 2386,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 1059,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 2780,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 5144,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 659,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 1023,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 4498,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 4265,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 5497,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 4039,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 1491,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 1542,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 6010,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 2142,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 5510,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 5084,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 3336,
    /// DIAGNOSTIC-SECURE-CODING-MAPPING
    DiagnosticSecureCodingMapping                                          = 4280,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 4062,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 399,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 4060,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 2427,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 2020,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 2962,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 5135,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 2436,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 6201,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 1595,
    /// DIAGNOSTIC-SERVICE-INSTANCE-REF
    DiagnosticServiceInstanceRef                                           = 776,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 4966,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 1902,
    /// DIAGNOSTIC-SERVICE-TABLE-REF
    DiagnosticServiceTableRef                                              = 449,
    /// DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL
    DiagnosticServiceTableRefConditional                                   = 118,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 2797,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 2120,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 4329,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 3612,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 5609,
    /// DIAGNOSTIC-SESSION-REF
    DiagnosticSessionRef                                                   = 1992,
    /// DIAGNOSTIC-SESSION-REFS
    DiagnosticSessionRefs                                                  = 926,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 5037,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE
    DiagnosticSovdAuthorizationInterface                                   = 4358,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING
    DiagnosticSovdAuthorizationPortMapping                                 = 1301,
    /// DIAGNOSTIC-SOVD-BULK-DATA
    DiagnosticSovdBulkData                                                 = 2318,
    /// DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE
    DiagnosticSovdBulkDataInterface                                        = 2625,
    /// DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING
    DiagnosticSovdBulkDataPortMapping                                      = 3928,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA
    DiagnosticSovdConfigurationBulkData                                    = 1530,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING
    DiagnosticSovdConfigurationDataIdentifierMapping                       = 2548,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE
    DiagnosticSovdConfigurationInterface                                   = 5924,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER
    DiagnosticSovdConfigurationParameter                                   = 2980,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING
    DiagnosticSovdConfigurationPortMapping                                 = 1484,
    /// DIAGNOSTIC-SOVD-LOCK
    DiagnosticSovdLock                                                     = 5052,
    /// DIAGNOSTIC-SOVD-LOG
    DiagnosticSovdLog                                                      = 4024,
    /// DIAGNOSTIC-SOVD-METHOD
    DiagnosticSovdMethod                                                   = 929,
    /// DIAGNOSTIC-SOVD-METHOD-PRIMITIVE
    DiagnosticSovdMethodPrimitive                                          = 6240,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE
    DiagnosticSovdProximityChallengeInterface                              = 2314,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING
    DiagnosticSovdProximityChallengePortMapping                            = 6047,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE
    DiagnosticSovdServiceValidationInterface                               = 3940,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING
    DiagnosticSovdServiceValidationPortMapping                             = 243,
    /// DIAGNOSTIC-SOVD-UPDATE
    DiagnosticSovdUpdate                                                   = 1068,
    /// DIAGNOSTIC-SOVD-UPDATE-INTERFACE
    DiagnosticSovdUpdateInterface                                          = 3745,
    /// DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING
    DiagnosticSovdUpdatePortMapping                                        = 5272,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 437,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 6197,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 3725,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 849,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF
    DiagnosticStorageConditionRef                                          = 4568,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL
    DiagnosticStorageConditionRefConditional                               = 2302,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 3391,
    /// DIAGNOSTIC-TEST-RESULT-REF
    DiagnosticTestResultRef                                                = 1798,
    /// DIAGNOSTIC-TEST-RESULT-REFS
    DiagnosticTestResultRefs                                               = 458,
    /// DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT
    DiagnosticTestResultUpdateEnumValueVariationPoint                      = 1612,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 1419,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 4410,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 5302,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 3390,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 5643,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 737,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 1615,
    /// DIAGNOSTIC-TROUBLE-CODE-REF
    DiagnosticTroubleCodeRef                                               = 146,
    /// DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL
    DiagnosticTroubleCodeRefConditional                                    = 2580,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 3607,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 2092,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 4818,
    /// DIAGNOSTIC-UDS-SEVERITY-ENUM-VALUE-VARIATION-POINT
    DiagnosticUdsSeverityEnumValueVariationPoint                           = 346,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 5216,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 6021,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 3225,
    /// DIAGNOSTIC-VALUE-ACCESS
    DiagnosticValueAccess                                                  = 5225,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 4883,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 5595,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 6204,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 1431,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 4561,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 4381,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 4246,
    /// DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT
    DiagnosticWwhObdDtcClassEnumValueVariationPoint                        = 3578,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 5679,
    /// DID-NUMBER
    DidNumber                                                              = 4111,
    /// DID-SIZE
    DidSize                                                                = 3606,
    /// DIFFERENTIATED-SERVICE-CODE-POINT
    DifferentiatedServiceCodePoint                                         = 2458,
    /// DIFFERENTIATED-SERVICE-FIELD
    DifferentiatedServiceField                                             = 5397,
    /// DIRECT-PDU-REF
    DirectPduRef                                                           = 8,
    /// DIRECT-SUCCESSOR-REF
    DirectSuccessorRef                                                     = 920,
    /// DIRECT-SUCCESSOR-REFS
    DirectSuccessorRefs                                                    = 3428,
    /// DIRECT-TP-SDU-REF
    DirectTpSduRef                                                         = 5704,
    /// DIRECTION
    Direction                                                              = 817,
    /// DISABLE-END-TO-END-CHECK
    DisableEndToEndCheck                                                   = 3272,
    /// DISABLE-END-TO-END-STATE-MACHINE
    DisableEndToEndStateMachine                                            = 2230,
    /// DISABLED-IN-MODE-IREF
    DisabledInModeIref                                                     = 4468,
    /// DISABLED-IN-MODE-IREFS
    DisabledInModeIrefs                                                    = 2189,
    /// DISABLED-MODE-IREF
    DisabledModeIref                                                       = 5161,
    /// DISABLED-MODE-IREFS
    DisabledModeIrefs                                                      = 5780,
    /// DISCOVERY-PROTECTION-KIND
    DiscoveryProtectionKind                                                = 1856,
    /// DISCOVERY-TECHNOLOGY
    DiscoveryTechnology                                                    = 5351,
    /// DISCOVERY-TYPE
    DiscoveryType                                                          = 2612,
    /// DISPLAY-FORMAT
    DisplayFormat                                                          = 3280,
    /// DISPLAY-IDENTIFIER
    DisplayIdentifier                                                      = 2646,
    /// DISPLAY-NAME
    DisplayName                                                            = 3047,
    /// DISPLAY-PRESENTATION
    DisplayPresentation                                                    = 6267,
    /// DISTINGUISHED-PARTITIONS
    DistinguishedPartitions                                                = 198,
    /// DLT-APPLICATION
    DltApplication                                                         = 6190,
    /// DLT-APPLICATION-REF
    DltApplicationRef                                                      = 1695,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 1294,
    /// DLT-ARGUMENT
    DltArgument                                                            = 5740,
    /// DLT-ARGUMENT-ENTRYS
    DltArgumentEntrys                                                      = 203,
    /// DLT-ARGUMENTS
    DltArguments                                                           = 4314,
    /// DLT-CONFIG
    DltConfig                                                              = 2522,
    /// DLT-CONTEXT
    DltContext                                                             = 2902,
    /// DLT-CONTEXT-REF
    DltContextRef                                                          = 2696,
    /// DLT-CONTEXT-REF-CONDITIONAL
    DltContextRefConditional                                               = 642,
    /// DLT-ECU
    DltEcu                                                                 = 5523,
    /// DLT-ECU-ID
    DltEcuId                                                               = 1746,
    /// DLT-ECU-REF
    DltEcuRef                                                              = 194,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 3685,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 214,
    /// DLT-LOG-CHANNEL-DESIGN-REF
    DltLogChannelDesignRef                                                 = 803,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 4094,
    /// DLT-LOG-CHANNEL-REF
    DltLogChannelRef                                                       = 3795,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 3848,
    /// DLT-LOG-CHANNELS
    DltLogChannels                                                         = 684,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 5884,
    /// DLT-LOG-SINK-REF
    DltLogSinkRef                                                          = 2342,
    /// DLT-LOG-SINK-REFS
    DltLogSinkRefs                                                         = 3224,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 5344,
    /// DLT-MESSAGE
    DltMessage                                                             = 1466,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 1464,
    /// DLT-MESSAGE-REF
    DltMessageRef                                                          = 3414,
    /// DLT-MESSAGE-REF-CONDITIONAL
    DltMessageRefConditional                                               = 3297,
    /// DLT-MESSAGE-REFS
    DltMessageRefs                                                         = 2772,
    /// DLT-MESSAGES
    DltMessages                                                            = 1830,
    /// DLT-SESSION-ID
    DltSessionId                                                           = 2861,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 2709,
    /// DNS-SERVER-ADDRESS
    DnsServerAddress                                                       = 3924,
    /// DNS-SERVER-ADDRESSES
    DnsServerAddresses                                                     = 127,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 746,
    /// DO-IP-CONFIG
    DoIpConfig                                                             = 614,
    /// DO-IP-ENTITY
    DoIpEntity                                                             = 5154,
    /// DO-IP-ENTITY-ROLE
    DoIpEntityRole                                                         = 4235,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 1429,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 2642,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 1091,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 922,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 3553,
    /// DO-IP-LOGIC-ADDRESS-PROPS
    DoIpLogicAddressProps                                                  = 2928,
    /// DO-IP-LOGIC-ADDRESSS
    DoIpLogicAddresss                                                      = 1250,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 347,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 4168,
    /// DO-IP-NETWORK-CONFIGURATION
    DoIpNetworkConfiguration                                               = 4594,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 1719,
    /// DO-IP-REQUEST-CONFIGURATION
    DoIpRequestConfiguration                                               = 2627,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 3836,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 1709,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 417,
    /// DO-IP-ROUTING-ACTIVATIONS
    DoIpRoutingActivations                                                 = 3541,
    /// DO-IP-RULE
    DoIpRule                                                               = 445,
    /// DO-IP-SOURCE-ADDRESS-REF
    DoIpSourceAddressRef                                                   = 4748,
    /// DO-IP-TARGET-ADDRESS-REF
    DoIpTargetAddressRef                                                   = 5759,
    /// DO-IP-TARGET-ADDRESS-REFS
    DoIpTargetAddressRefs                                                  = 6216,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REF
    DoIpTesterRoutingActivationRef                                         = 1258,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REFS
    DoIpTesterRoutingActivationRefs                                        = 888,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 1669,
    /// DO-IP-TP-CONNECTION
    DoIpTpConnection                                                       = 4661,
    /// DO-NOT-FRAGMENT
    DoNotFragment                                                          = 4262,
    /// DOC-REVISION
    DocRevision                                                            = 1791,
    /// DOC-REVISIONS
    DocRevisions                                                           = 6092,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 3330,
    /// DOCUMENT-ELEMENT-SCOPES
    DocumentElementScopes                                                  = 530,
    /// DOCUMENTATION
    Documentation                                                          = 4670,
    /// DOCUMENTATION-CONTENT
    DocumentationContent                                                   = 2533,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 1997,
    /// DOIP-CHANNEL-COLLECTION-REF
    DoipChannelCollectionRef                                               = 2472,
    /// DOIP-CONNECTION-REF
    DoipConnectionRef                                                      = 1617,
    /// DOIP-CONNECTION-REFS
    DoipConnectionRefs                                                     = 325,
    /// DOIP-INTERFACES
    DoipInterfaces                                                         = 6199,
    /// DOMAIN
    Domain                                                                 = 2978,
    /// DOMAIN-ID
    DomainId                                                               = 1130,
    /// DOMAIN-IDS
    DomainIds                                                              = 2601,
    /// DPD-ACTION
    DpdAction                                                              = 3604,
    /// DPD-DELAY
    DpdDelay                                                               = 2463,
    /// DPG-DOES-NOT-REQUIRE-COHERENCYS
    DpgDoesNotRequireCoherencys                                            = 1457,
    /// DPG-REQUIRES-COHERENCYS
    DpgRequiresCoherencys                                                  = 144,
    /// DRIVER-NOTIFICATIONS
    DriverNotifications                                                    = 2891,
    /// DRIVER-PRIMITIVE-REF
    DriverPrimitiveRef                                                     = 5158,
    /// DROP-NOT-REQUESTED-NAD
    DropNotRequestedNad                                                    = 3947,
    /// DTC-FORMAT-TYPE
    DtcFormatType                                                          = 3794,
    /// DTC-KIND
    DtcKind                                                                = 5725,
    /// DTC-NUMBER
    DtcNumber                                                              = 1261,
    /// DTC-PROPS-REF
    DtcPropsRef                                                            = 5446,
    /// DTC-SETTING-CLASS-REF
    DtcSettingClassRef                                                     = 6022,
    /// DTC-SETTING-PARAMETER
    DtcSettingParameter                                                    = 984,
    /// DTC-STATUS-AVAILABILITY-MASK
    DtcStatusAvailabilityMask                                              = 2262,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 3985,
    /// DTC-STATUS-MASK
    DtcStatusMask                                                          = 2199,
    /// DTCS
    Dtcs                                                                   = 3113,
    /// DURABILITY
    Durability                                                             = 1729,
    /// DURABILITY-KIND
    DurabilityKind                                                         = 189,
    /// DURABILITY-SERVICE
    DurabilityService                                                      = 6181,
    /// DURABILITY-SERVICE-CLEANUP-DELAY
    DurabilityServiceCleanupDelay                                          = 310,
    /// DURABILITY-SERVICE-HISTORY-DEPTH
    DurabilityServiceHistoryDepth                                          = 1344,
    /// DURABILITY-SERVICE-HISTORY-KIND
    DurabilityServiceHistoryKind                                           = 2619,
    /// DURABILITY-SERVICE-MAX-INSTANCES
    DurabilityServiceMaxInstances                                          = 907,
    /// DURABILITY-SERVICE-MAX-SAMPLES
    DurabilityServiceMaxSamples                                            = 901,
    /// DURABILITY-SERVICE-MAX-SAMPLES-PER-INSTANCE
    DurabilityServiceMaxSamplesPerInstance                                 = 2874,
    /// DYNAMIC-ACTION-REF
    DynamicActionRef                                                       = 5629,
    /// DYNAMIC-ACTION-REFS
    DynamicActionRefs                                                      = 5812,
    /// DYNAMIC-ARRAY-SIZE-PROFILE
    DynamicArraySizeProfile                                                = 3231,
    /// DYNAMIC-BS
    DynamicBs                                                              = 2435,
    /// DYNAMIC-LENGTH
    DynamicLength                                                          = 1648,
    /// DYNAMIC-PART
    DynamicPart                                                            = 3277,
    /// DYNAMIC-PART-ALTERNATIVE
    DynamicPartAlternative                                                 = 2460,
    /// DYNAMIC-PART-ALTERNATIVES
    DynamicPartAlternatives                                                = 359,
    /// DYNAMIC-PARTS
    DynamicParts                                                           = 264,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REF
    DynamicPncMappingPduGroupRef                                           = 750,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS
    DynamicPncMappingPduGroupRefs                                          = 1255,
    /// DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED
    DynamicPncToChannelMappingEnabled                                      = 5793,
    /// DYNAMIC-RUNTIME-LENGTH-HANDLING
    DynamicRuntimeLengthHandling                                           = 2211,
    /// DYNAMIC-SLOT-IDLE-PHASE
    DynamicSlotIdlePhase                                                   = 6080,
    /// DYNAMICALLY-ASSIGNED
    DynamicallyAssigned                                                    = 3755,
    /// DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF
    DynamicallyDefineDataIdentifierClassRef                                = 2359,
    /// E
    E                                                                      = 3909,
    /// E-2-E-EVENT-PROTECTION-PROPSS
    E2EEventProtectionPropss                                               = 3832,
    /// E-2-E-METHOD-PROTECTION-PROPSS
    E2EMethodProtectionPropss                                              = 426,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 13,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS-REF
    E2EProfileCompatibilityPropsRef                                        = 2878,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 4903,
    /// E-2-E-PROFILE-CONFIGURATION-REF
    E2EProfileConfigurationRef                                             = 1142,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 4295,
    /// E-2-E-PROFILE-CONFIGURATIONS
    E2EProfileConfigurations                                               = 3988,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 430,
    /// ECU-COMM-PORT-INSTANCES
    EcuCommPortInstances                                                   = 1359,
    /// ECU-CONFIGURATION-REF
    EcuConfigurationRef                                                    = 1138,
    /// ECU-EXTRACT-REF
    EcuExtractRef                                                          = 3121,
    /// ECU-EXTRACT-REFERENCE-IREF
    EcuExtractReferenceIref                                                = 5366,
    /// ECU-EXTRACT-VERSION
    EcuExtractVersion                                                      = 2770,
    /// ECU-ID
    EcuId                                                                  = 3400,
    /// ECU-INSTANCE
    EcuInstance                                                            = 3081,
    /// ECU-INSTANCE-PROPS
    EcuInstanceProps                                                       = 5918,
    /// ECU-INSTANCE-PROPSS
    EcuInstancePropss                                                      = 5487,
    /// ECU-INSTANCE-REF
    EcuInstanceRef                                                         = 767,
    /// ECU-INSTANCE-REF-CONDITIONAL
    EcuInstanceRefConditional                                              = 244,
    /// ECU-INSTANCE-REFS
    EcuInstanceRefs                                                        = 1274,
    /// ECU-INSTANCES
    EcuInstances                                                           = 5527,
    /// ECU-MAPPING
    EcuMapping                                                             = 3885,
    /// ECU-PARTITION
    EcuPartition                                                           = 2527,
    /// ECU-PARTITION-REF
    EcuPartitionRef                                                        = 4150,
    /// ECU-REF
    EcuRef                                                                 = 799,
    /// ECU-RESET-CLASS-REF
    EcuResetClassRef                                                       = 729,
    /// ECU-RESOURCE-ESTIMATION
    EcuResourceEstimation                                                  = 560,
    /// ECU-RESOURCE-MAPPINGS
    EcuResourceMappings                                                    = 4873,
    /// ECU-SCOPE-REF
    EcuScopeRef                                                            = 1268,
    /// ECU-SCOPE-REFS
    EcuScopeRefs                                                           = 5358,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 4392,
    /// ECU-TASK-PROXY-REF
    EcuTaskProxyRef                                                        = 5207,
    /// ECU-TASK-PROXY-REFS
    EcuTaskProxyRefs                                                       = 2101,
    /// ECU-TIMING
    EcuTiming                                                              = 3068,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 2307,
    /// ECUC-ADD-INFO-PARAM-VALUE
    EcucAddInfoParamValue                                                  = 2378,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 5625,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 4030,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 6123,
    /// ECUC-COND
    EcucCond                                                               = 1080,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 5112,
    /// ECUC-DEF-EDITION
    EcucDefEdition                                                         = 1946,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 2542,
    /// ECUC-DEFINITION-REF
    EcucDefinitionRef                                                      = 3854,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 4689,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 5265,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 4978,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 2309,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 790,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 3776,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 5034,
    /// ECUC-FUNCTION-NAME-DEF-CONDITIONAL
    EcucFunctionNameDefConditional                                         = 3060,
    /// ECUC-FUNCTION-NAME-DEF-VARIANTS
    EcucFunctionNameDefVariants                                            = 2655,
    /// ECUC-IMPLEMENTATION-CONFIGURATION-CLASS
    EcucImplementationConfigurationClass                                   = 5362,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 5257,
    /// ECUC-INSTANCE-REFERENCE-VALUE
    EcucInstanceReferenceValue                                             = 3971,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 3861,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 2529,
    /// ECUC-LINKER-SYMBOL-DEF-CONDITIONAL
    EcucLinkerSymbolDefConditional                                         = 4621,
    /// ECUC-LINKER-SYMBOL-DEF-VARIANTS
    EcucLinkerSymbolDefVariants                                            = 1157,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 1119,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF
    EcucModuleConfigurationValuesRef                                       = 5745,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL
    EcucModuleConfigurationValuesRefConditional                            = 3978,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 2119,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 89,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL
    EcucMultilineStringParamDefConditional                                 = 1420,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS
    EcucMultilineStringParamDefVariants                                    = 3961,
    /// ECUC-MULTIPLICITY-CONFIGURATION-CLASS
    EcucMultiplicityConfigurationClass                                     = 5471,
    /// ECUC-NUMERICAL-PARAM-VALUE
    EcucNumericalParamValue                                                = 361,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 2590,
    /// ECUC-QUERY
    EcucQuery                                                              = 413,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 1974,
    /// ECUC-QUERY-REF
    EcucQueryRef                                                           = 3152,
    /// ECUC-QUERY-STRING-REF
    EcucQueryStringRef                                                     = 5787,
    /// ECUC-QUERYS
    EcucQuerys                                                             = 5730,
    /// ECUC-REF
    EcucRef                                                                = 480,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 4508,
    /// ECUC-REFERENCE-VALUE
    EcucReferenceValue                                                     = 1681,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 6065,
    /// ECUC-STRING-PARAM-DEF-CONDITIONAL
    EcucStringParamDefConditional                                          = 4956,
    /// ECUC-STRING-PARAM-DEF-VARIANTS
    EcucStringParamDefVariants                                             = 1749,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 1395,
    /// ECUC-TEXTUAL-PARAM-VALUE
    EcucTextualParamValue                                                  = 1087,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 2038,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 5156,
    /// ECUC-VALIDATION-CONDS
    EcucValidationConds                                                    = 1999,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 2854,
    /// ECUC-VALUE-CONFIGURATION-CLASS
    EcucValueConfigurationClass                                            = 2375,
    /// ECUC-VALUES
    EcucValues                                                             = 2502,
    /// EGRESS-PORT-REF
    EgressPortRef                                                          = 2417,
    /// EGRESS-PORT-REFS
    EgressPortRefs                                                         = 3366,
    /// EID
    Eid                                                                    = 4203,
    /// EID-RETRIEVAL
    EidRetrieval                                                           = 3760,
    /// EID-USE-MAC
    EidUseMac                                                              = 5159,
    /// ELEMENT
    Element                                                                = 4845,
    /// ELEMENT-BYTE-VALUE
    ElementByteValue                                                       = 1246,
    /// ELEMENT-GROUPS
    ElementGroups                                                          = 3888,
    /// ELEMENT-IN-IMPL-DATATYPE
    ElementInImplDatatype                                                  = 3902,
    /// ELEMENT-MAPPINGS
    ElementMappings                                                        = 615,
    /// ELEMENT-POSITION
    ElementPosition                                                        = 1099,
    /// ELEMENT-PROPSS
    ElementPropss                                                          = 4307,
    /// ELEMENT-REF
    ElementRef                                                             = 5044,
    /// ELEMENT-REFS
    ElementRefs                                                            = 2809,
    /// ELEMENT-ROLE
    ElementRole                                                            = 2954,
    /// ELEMENTS
    Elements                                                               = 1205,
    /// ELLIPTIC-CURVE-REF
    EllipticCurveRef                                                       = 1876,
    /// ELLIPTIC-CURVE-REFS
    EllipticCurveRefs                                                      = 5894,
    /// EMPTY-SIGNAL
    EmptySignal                                                            = 4772,
    /// EMULATION-SUPPORTS
    EmulationSupports                                                      = 5614,
    /// ENABLE-ANYCAST
    EnableAnycast                                                          = 351,
    /// ENABLE-CONDITION-GROUP-REF
    EnableConditionGroupRef                                                = 2616,
    /// ENABLE-CONDITION-REF
    EnableConditionRef                                                     = 5766,
    /// ENABLE-CONDITIONS
    EnableConditions                                                       = 6059,
    /// ENABLE-DEACTIVATION
    EnableDeactivation                                                     = 5197,
    /// ENABLE-DISCOVERY-PROTECTION
    EnableDiscoveryProtection                                              = 5957,
    /// ENABLE-JOIN-ACCESS-CONTROL
    EnableJoinAccessControl                                                = 2557,
    /// ENABLE-LIVELINESS-PROTECTION
    EnableLivelinessProtection                                             = 5296,
    /// ENABLE-READ-ACCESS-CONTROL
    EnableReadAccessControl                                                = 5940,
    /// ENABLE-TAKE-ADDRESS
    EnableTakeAddress                                                      = 5420,
    /// ENABLE-UPDATE
    EnableUpdate                                                           = 3753,
    /// ENABLE-WRITE-ACCESS-CONTROL
    EnableWriteAccessControl                                               = 3937,
    /// ENABLES-LOG-TRACE
    EnablesLogTrace                                                        = 2526,
    /// ENCAPSULATED-ENTRY-REF
    EncapsulatedEntryRef                                                   = 2537,
    /// ENCODING
    Encoding                                                               = 1174,
    /// ENCRYPTION-REF
    EncryptionRef                                                          = 2863,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 5172,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 6106,
    /// END-ADDRESS
    EndAddress                                                             = 5057,
    /// END-TO-END-CALL-RESPONSE-TIMEOUT
    EndToEndCallResponseTimeout                                            = 3865,
    /// END-TO-END-PROFILE
    EndToEndProfile                                                        = 5170,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 4135,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 5320,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDUS
    EndToEndProtectionISignalIPdus                                         = 4596,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 5832,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 4941,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPES
    EndToEndProtectionVariablePrototypes                                   = 5223,
    /// END-TO-END-PROTECTIONS
    EndToEndProtections                                                    = 392,
    /// END-TO-END-TRANSFORMATION-COM-SPEC-PROPS
    EndToEndTransformationComSpecProps                                     = 1927,
    /// END-TO-END-TRANSFORMATION-DESCRIPTION
    EndToEndTransformationDescription                                      = 1643,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS
    EndToEndTransformationISignalProps                                     = 4842,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    EndToEndTransformationISignalPropsConditional                          = 1067,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    EndToEndTransformationISignalPropsVariants                             = 2732,
    /// ENDPOINT-CONFIGURATION-REF
    EndpointConfigurationRef                                               = 5798,
    /// ENDPOINTS
    Endpoints                                                              = 606,
    /// ENGINEERING-OBJECT
    EngineeringObject                                                      = 3001,
    /// ENGINEERING-OBJECTS
    EngineeringObjects                                                     = 539,
    /// ENHANCED-MODE-API
    EnhancedModeApi                                                        = 6274,
    /// ENTER-TIMEOUT-VALUE
    EnterTimeoutValue                                                      = 367,
    /// ENTERED-MODE-REF
    EnteredModeRef                                                         = 2979,
    /// ENTITY-STATUS-MAX-BYTE-FIELD-USE
    EntityStatusMaxByteFieldUse                                            = 1248,
    /// ENTITYS
    Entitys                                                                = 1995,
    /// ENTRY
    Entry                                                                  = 3687,
    /// ENTRY-MODE-DECLARATION-REF
    EntryModeDeclarationRef                                                = 913,
    /// ENTRY-REF
    EntryRef                                                               = 1124,
    /// ENTRY-TYPE
    EntryType                                                              = 747,
    /// ENTRYS
    Entrys                                                                 = 616,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 5218,
    /// ENUMERATOR-VALUE
    EnumeratorValue                                                        = 5889,
    /// ENUMS
    Enums                                                                  = 3463,
    /// ENVIRONMENT-CAPTURE-TO-REPORTING
    EnvironmentCaptureToReporting                                          = 564,
    /// ENVIRONMENT-DATA-CAPTURE
    EnvironmentDataCapture                                                 = 3239,
    /// ENVIRONMENT-VARIABLES
    EnvironmentVariables                                                   = 3027,
    /// ENVIRONMENTAL-CONDITION-REF
    EnvironmentalConditionRef                                              = 1627,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 4615,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 2969,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 3192,
    /// ERROR-CODE
    ErrorCode                                                              = 4870,
    /// ERROR-CONTEXT-REF
    ErrorContextRef                                                        = 5241,
    /// ERROR-CONTEXT-REFS
    ErrorContextRefs                                                       = 4545,
    /// ERROR-DOMAIN-REF
    ErrorDomainRef                                                         = 2496,
    /// ERROR-HANDLING
    ErrorHandling                                                          = 2233,
    /// ERROR-MAPPINGS
    ErrorMappings                                                          = 1010,
    /// ERROR-REACTION-POLICY
    ErrorReactionPolicy                                                    = 4972,
    /// ERROR-SIGNALING-ENABLED
    ErrorSignalingEnabled                                                  = 2795,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 4380,
    /// ESP-CIPHER-SUITE-NAME
    EspCipherSuiteName                                                     = 5873,
    /// ESP-CIPHER-SUITE-NAMES
    EspCipherSuiteNames                                                    = 2645,
    /// ESTIMATED-DURATION-OF-CAMPAIGN
    EstimatedDurationOfCampaign                                            = 2950,
    /// ESTIMATED-DURATION-OF-OPERATION
    EstimatedDurationOfOperation                                           = 1523,
    /// ESTIMATED-EXECUTION-TIME
    EstimatedExecutionTime                                                 = 2310,
    /// ETH-GLOBAL-TIME-DOMAIN-PROPS
    EthGlobalTimeDomainProps                                               = 4173,
    /// ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT
    EthGlobalTimeManagedCouplingPort                                       = 3572,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 4415,
    /// ETH-IP-PROPS-REF
    EthIpPropsRef                                                          = 4849,
    /// ETH-IP-PROPS-REFS
    EthIpPropsRefs                                                         = 892,
    /// ETH-SWITCH-PORT-GROUP-DERIVATION
    EthSwitchPortGroupDerivation                                           = 2168,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 5378,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 5234,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 474,
    /// ETH-TP-CONNECTION
    EthTpConnection                                                        = 5819,
    /// ETHER-TYPE
    EtherType                                                              = 3974,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 3524,
    /// ETHERNET-CLUSTER-CONDITIONAL
    EthernetClusterConditional                                             = 1112,
    /// ETHERNET-CLUSTER-REF
    EthernetClusterRef                                                     = 5222,
    /// ETHERNET-CLUSTER-VARIANTS
    EthernetClusterVariants                                                = 3721,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 2369,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 4044,
    /// ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL
    EthernetCommunicationControllerConditional                             = 6243,
    /// ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS
    EthernetCommunicationControllerVariants                                = 3396,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 1129,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 323,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 166,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 5724,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 3346,
    /// ETHERNET-PRIORITY-REGENERATIONS
    EthernetPriorityRegenerations                                          = 3242,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 4932,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 4425,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 3500,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING-REF
    EthernetRawDataStreamMappingRef                                        = 3518,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 2815,
    /// ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS
    EthernetTrafficClassAssignments                                        = 4161,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 5293,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 793,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS
    EthernetWakeupSleepOnDatalineConfigs                                   = 3499,
    /// EVALUATED-ELEMENT-REF
    EvaluatedElementRef                                                    = 609,
    /// EVALUATED-ELEMENT-REFS
    EvaluatedElementRefs                                                   = 3466,
    /// EVALUATED-VARIANT-REF
    EvaluatedVariantRef                                                    = 3877,
    /// EVALUATED-VARIANT-REFS
    EvaluatedVariantRefs                                                   = 1478,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 39,
    /// EVALUATION-ID
    EvaluationId                                                           = 4260,
    /// EVENT-CLEAR-ALLOWED
    EventClearAllowed                                                      = 5802,
    /// EVENT-COMBINATION-REPORTING-BEHAVIOR
    EventCombinationReportingBehavior                                      = 4587,
    /// EVENT-CONTROLLED-TIMING
    EventControlledTiming                                                  = 4222,
    /// EVENT-DEFAULT-VALUE
    EventDefaultValue                                                      = 3368,
    /// EVENT-DEPLOYMENTS
    EventDeployments                                                       = 5035,
    /// EVENT-DISPLACEMENT-STRATEGY
    EventDisplacementStrategy                                              = 5797,
    /// EVENT-ELEMENT-MAPPINGS
    EventElementMappings                                                   = 5015,
    /// EVENT-ELEMENTS
    EventElements                                                          = 2464,
    /// EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD
    EventFailureCycleCounterThreshold                                      = 3544,
    /// EVENT-GROUP-CONTROL-TYPE
    EventGroupControlType                                                  = 2474,
    /// EVENT-GROUP-ID
    EventGroupId                                                           = 1094,
    /// EVENT-GROUP-IDENTIFIER
    EventGroupIdentifier                                                   = 11,
    /// EVENT-GROUP-REF
    EventGroupRef                                                          = 3820,
    /// EVENT-GROUP-REFS
    EventGroupRefs                                                         = 1860,
    /// EVENT-GROUPS
    EventGroups                                                            = 6091,
    /// EVENT-HANDLER
    EventHandler                                                           = 3579,
    /// EVENT-HANDLER-REF
    EventHandlerRef                                                        = 1162,
    /// EVENT-HANDLER-REFS
    EventHandlerRefs                                                       = 5980,
    /// EVENT-HANDLERS
    EventHandlers                                                          = 2776,
    /// EVENT-ID
    EventId                                                                = 531,
    /// EVENT-IREF
    EventIref                                                              = 5967,
    /// EVENT-KIND
    EventKind                                                              = 5558,
    /// EVENT-MAPPING
    EventMapping                                                           = 5755,
    /// EVENT-MAPPINGS
    EventMappings                                                          = 3993,
    /// EVENT-MULTICAST-ADDRESSS
    EventMulticastAddresss                                                 = 5036,
    /// EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS
    EventMulticastSubscriptionAddresss                                     = 5076,
    /// EVENT-MULTICAST-UDP-PORT
    EventMulticastUdpPort                                                  = 5777,
    /// EVENT-NEEDS-REF
    EventNeedsRef                                                          = 3870,
    /// EVENT-OBD-READINESS-GROUP
    EventObdReadinessGroup                                                 = 4769,
    /// EVENT-OCCURRENCE-KIND
    EventOccurrenceKind                                                    = 3762,
    /// EVENT-PROPSS
    EventPropss                                                            = 482,
    /// EVENT-QOS-PROPSS
    EventQosPropss                                                         = 846,
    /// EVENT-READINESS-GROUPS
    EventReadinessGroups                                                   = 4348,
    /// EVENT-RECEPTION-DEFAULT-VALUE
    EventReceptionDefaultValue                                             = 3787,
    /// EVENT-REF
    EventRef                                                               = 5879,
    /// EVENT-REFS
    EventRefs                                                              = 3590,
    /// EVENT-SOURCE-REF
    EventSourceRef                                                         = 5907,
    /// EVENT-SYMBOL-NAME
    EventSymbolName                                                        = 4676,
    /// EVENT-TOPIC-ACCESS-RULE-REF
    EventTopicAccessRuleRef                                                = 258,
    /// EVENT-WINDOW-TIME
    EventWindowTime                                                        = 798,
    /// EVENT-WINDOWS
    EventWindows                                                           = 4803,
    /// EVENTS
    Events                                                                 = 2252,
    /// EXCESS-BURST-SIZE
    ExcessBurstSize                                                        = 1288,
    /// EXCESS-INFORMATION-RATE
    ExcessInformationRate                                                  = 6011,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 513,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 3667,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REF
    ExclusiveAreaNestingOrderRef                                           = 5348,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REFS
    ExclusiveAreaNestingOrderRefs                                          = 248,
    /// EXCLUSIVE-AREA-NESTING-ORDERS
    ExclusiveAreaNestingOrders                                             = 6101,
    /// EXCLUSIVE-AREA-POLICYS
    ExclusiveAreaPolicys                                                   = 334,
    /// EXCLUSIVE-AREA-REF
    ExclusiveAreaRef                                                       = 5208,
    /// EXCLUSIVE-AREA-REF-CONDITIONAL
    ExclusiveAreaRefConditional                                            = 5568,
    /// EXCLUSIVE-AREA-REFS
    ExclusiveAreaRefs                                                      = 4804,
    /// EXCLUSIVE-AREAS
    ExclusiveAreas                                                         = 1158,
    /// EXEC-IN-USER-MODE
    ExecInUserMode                                                         = 3984,
    /// EXECUTABLE
    Executable                                                             = 4451,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 1891,
    /// EXECUTABLE-ENTITY-REF
    ExecutableEntityRef                                                    = 5375,
    /// EXECUTABLE-ENTITY-REFS
    ExecutableEntityRefs                                                   = 2758,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 5206,
    /// EXECUTABLE-LOGGING-IMPLEMENTATION-PROPS
    ExecutableLoggingImplementationProps                                   = 5334,
    /// EXECUTABLE-PROVIDED-PORT-IREF
    ExecutableProvidedPortIref                                             = 5713,
    /// EXECUTABLE-REF
    ExecutableRef                                                          = 3792,
    /// EXECUTABLE-REFS
    ExecutableRefs                                                         = 2986,
    /// EXECUTABLE-REQUIRED-PORT-IREF
    ExecutableRequiredPortIref                                             = 1461,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 1078,
    /// EXECUTE-DESPITE-DATA-UNAVAILABILITY
    ExecuteDespiteDataUnavailability                                       = 1014,
    /// EXECUTION-CONTEXT
    ExecutionContext                                                       = 3148,
    /// EXECUTION-CONTEXT-REF
    ExecutionContextRef                                                    = 2050,
    /// EXECUTION-CONTEXT-REFS
    ExecutionContextRefs                                                   = 1734,
    /// EXECUTION-CONTEXTS
    ExecutionContexts                                                      = 3348,
    /// EXECUTION-DEPENDENCY
    ExecutionDependency                                                    = 1311,
    /// EXECUTION-DEPENDENCYS
    ExecutionDependencys                                                   = 3538,
    /// EXECUTION-ERROR
    ExecutionError                                                         = 4618,
    /// EXECUTION-ERROR-REF
    ExecutionErrorRef                                                      = 2767,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 5434,
    /// EXECUTION-ORDER-CONSTRAINT-TYPE
    ExecutionOrderConstraintType                                           = 1454,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 2789,
    /// EXECUTION-TIME-TYPE
    ExecutionTimeType                                                      = 2530,
    /// EXECUTION-TIMES
    ExecutionTimes                                                         = 3917,
    /// EXIT-MODE-DECLARATION-REF
    ExitModeDeclarationRef                                                 = 1613,
    /// EXIT-TIMEOUT-VALUE
    ExitTimeoutValue                                                       = 40,
    /// EXITED-MODE-REF
    ExitedModeRef                                                          = 1574,
    /// EXPECTED-ALIVE-CYCLE
    ExpectedAliveCycle                                                     = 4245,
    /// EXPECTED-ALIVE-INDICATIONS
    ExpectedAliveIndications                                               = 4264,
    /// EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE
    ExpectedAliveIndicationsInSupervisionCycle                             = 1766,
    /// EXPECTED-CALLBACKS
    ExpectedCallbacks                                                      = 3688,
    /// EXPECTED-ENTRYS
    ExpectedEntrys                                                         = 1646,
    /// EXPECTED-TX-TRIGGER
    ExpectedTxTrigger                                                      = 4527,
    /// EXPIRED-SUPERVISION-CYCLE-TOL
    ExpiredSupervisionCycleTol                                             = 3997,
    /// EXPIRED-SUPERVISION-CYCLES-TOLERANCE
    ExpiredSupervisionCyclesTolerance                                      = 723,
    /// EXPIRED-SUPERVISION-TOLERANCE
    ExpiredSupervisionTolerance                                            = 5192,
    /// EXPLICIT-CONGESTION-NOTIFICATION
    ExplicitCongestionNotification                                         = 481,
    /// EXPLICIT-INTER-RUNNABLE-VARIABLES
    ExplicitInterRunnableVariables                                         = 1582,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REF
    ExplicitRptProfileSelectionRef                                         = 1097,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REFS
    ExplicitRptProfileSelectionRefs                                        = 4327,
    /// EXPORTABILITY
    Exportability                                                          = 3341,
    /// EXPRESSION
    Expression                                                             = 4854,
    /// EXPRESSION-REF
    ExpressionRef                                                          = 5858,
    /// EXT-HEADER-FILTER-LISTS
    ExtHeaderFilterLists                                                   = 4724,
    /// EXTENDED-ADDRESSING
    ExtendedAddressing                                                     = 4086,
    /// EXTENDED-DATA-RECORDS
    ExtendedDataRecords                                                    = 2817,
    /// EXTENDS-META-CLASS
    ExtendsMetaClass                                                       = 2842,
    /// EXTERN-OFFSET-CORRECTION
    ExternOffsetCorrection                                                 = 2240,
    /// EXTERN-RATE-CORRECTION
    ExternRateCorrection                                                   = 5942,
    /// EXTERNAL-AUTHENTICATIONS
    ExternalAuthentications                                                = 4600,
    /// EXTERNAL-CLOCK-SYNCHRONISATION
    ExternalClockSynchronisation                                           = 3443,
    /// EXTERNAL-REPLACEMENT-REF
    ExternalReplacementRef                                                 = 4122,
    /// EXTERNAL-SYNC
    ExternalSync                                                           = 975,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 635,
    /// EXTERNAL-TRIGGERING-POINT
    ExternalTriggeringPoint                                                = 1503,
    /// EXTERNAL-TRIGGERING-POINTS
    ExternalTriggeringPoints                                               = 5545,
    /// FACTOR-SI-TO-UNIT
    FactorSiToUnit                                                         = 136,
    /// FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL
    FailedAliveSupervisionRefCycleTol                                      = 4461,
    /// FAILED-REFERENCE-CYCLES-TOLERANCE
    FailedReferenceCyclesTolerance                                         = 92,
    /// FAILED-SUPERVISION-CYCLES-TOLERANCE
    FailedSupervisionCyclesTolerance                                       = 5867,
    /// FAILURE-MONITORING-REF
    FailureMonitoringRef                                                   = 3386,
    /// FALL-BACK-INTERNAL
    FallBackInternal                                                       = 5638,
    /// FALSE-ACTION-LIST-REF
    FalseActionListRef                                                     = 3289,
    /// FDC-THRESHOLD-STORAGE-VALUE
    FdcThresholdStorageValue                                               = 1440,
    /// FEATURE-IREF
    FeatureIref                                                            = 3600,
    /// FEATURE-MODEL-REF
    FeatureModelRef                                                        = 2414,
    /// FEATURE-MODEL-REFS
    FeatureModelRefs                                                       = 2392,
    /// FEATURE-REF
    FeatureRef                                                             = 1273,
    /// FEATURE-REFS
    FeatureRefs                                                            = 383,
    /// FIBEX-ELEMENT-REF
    FibexElementRef                                                        = 324,
    /// FIBEX-ELEMENT-REF-CONDITIONAL
    FibexElementRefConditional                                             = 3375,
    /// FIBEX-ELEMENTS
    FibexElements                                                          = 4847,
    /// FIELD
    Field                                                                  = 5069,
    /// FIELD-DEPLOYMENTS
    FieldDeployments                                                       = 84,
    /// FIELD-ELEMENTS
    FieldElements                                                          = 304,
    /// FIELD-GET-SET-QOS-PROPSS
    FieldGetSetQosPropss                                                   = 2322,
    /// FIELD-IREF
    FieldIref                                                              = 5025,
    /// FIELD-MAPPING
    FieldMapping                                                           = 345,
    /// FIELD-MAPPINGS
    FieldMappings                                                          = 442,
    /// FIELD-NOTIFIER-QOS-PROPSS
    FieldNotifierQosPropss                                                 = 5022,
    /// FIELD-NOTIFIER-REF
    FieldNotifierRef                                                       = 2820,
    /// FIELD-REF
    FieldRef                                                               = 6116,
    /// FIELD-REFS
    FieldRefs                                                              = 4739,
    /// FIELD-REPLY-TOPIC-NAME
    FieldReplyTopicName                                                    = 3310,
    /// FIELD-REQUEST-TOPIC-NAME
    FieldRequestTopicName                                                  = 1452,
    /// FIELD-SENDER-COM-SPEC
    FieldSenderComSpec                                                     = 4018,
    /// FIELD-TOPICS-ACCESS-RULE-REF
    FieldTopicsAccessRuleRef                                               = 491,
    /// FIELDS
    Fields                                                                 = 4123,
    /// FIFO-DEPTH
    FifoDepth                                                              = 2174,
    /// FIFO-RANGES
    FifoRanges                                                             = 340,
    /// FIGURE
    Figure                                                                 = 3567,
    /// FIGURE-CAPTION
    FigureCaption                                                          = 1736,
    /// FILE-ELEMENTS
    FileElements                                                           = 3839,
    /// FILE-INFO-COMMENT
    FileInfoComment                                                        = 808,
    /// FILE-NAME
    FileName                                                               = 1618,
    /// FILE-PROXYS
    FileProxys                                                             = 5712,
    /// FILE-REF
    FileRef                                                                = 3332,
    /// FILE-REFS
    FileRefs                                                               = 2890,
    /// FILE-STORAGE-REF
    FileStorageRef                                                         = 6253,
    /// FILE-TYPE
    FileType                                                               = 869,
    /// FILE-TYPE-PATTERN
    FileTypePattern                                                        = 2650,
    /// FILES
    Files                                                                  = 5753,
    /// FILTER
    Filter                                                                 = 5529,
    /// FILTER-ACTION-BLOCK-SOURCE
    FilterActionBlockSource                                                = 318,
    /// FILTER-ACTION-DEST-PORT-MODIFICATION
    FilterActionDestPortModification                                       = 1848,
    /// FILTER-ACTION-DROP-FRAME
    FilterActionDropFrame                                                  = 2250,
    /// FILTER-ACTION-VLAN-MODIFICATION
    FilterActionVlanModification                                           = 4781,
    /// FILTER-CHAINS
    FilterChains                                                           = 1460,
    /// FILTER-PRIORITY
    FilterPriority                                                         = 4536,
    /// FILTERING-DEBOUNCING
    FilteringDebouncing                                                    = 932,
    /// FINAL-CHECKPOINT-REF
    FinalCheckpointRef                                                     = 4366,
    /// FINAL-CHECKPOINT-REFS
    FinalCheckpointRefs                                                    = 4233,
    /// FIRE-AND-FORGET
    FireAndForget                                                          = 1994,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 30,
    /// FIRE-AND-FORGET-MAPPINGS
    FireAndForgetMappings                                                  = 461,
    /// FIRE-AND-FORGET-METHOD-MAPPING
    FireAndForgetMethodMapping                                             = 5459,
    /// FIRE-AND-FORGET-METHOD-MAPPINGS
    FireAndForgetMethodMappings                                            = 5540,
    /// FIREWALL-REF
    FirewallRef                                                            = 1415,
    /// FIREWALL-RULE
    FirewallRule                                                           = 4530,
    /// FIREWALL-RULE-PROPS
    FirewallRuleProps                                                      = 3440,
    /// FIREWALL-RULE-PROPSS
    FirewallRulePropss                                                     = 6252,
    /// FIREWALL-RULE-REF
    FirewallRuleRef                                                        = 1624,
    /// FIREWALL-RULE-REFS
    FirewallRuleRefs                                                       = 2593,
    /// FIREWALL-STATE-IREF
    FirewallStateIref                                                      = 209,
    /// FIREWALL-STATE-IREFS
    FirewallStateIrefs                                                     = 4674,
    /// FIREWALL-STATE-MACHINES
    FirewallStateMachines                                                  = 2870,
    /// FIREWALL-STATE-MODE-DECLARATION-REF
    FirewallStateModeDeclarationRef                                        = 754,
    /// FIREWALL-STATE-MODE-DECLARATION-REFS
    FirewallStateModeDeclarationRefs                                       = 2348,
    /// FIREWALL-STATE-SWITCH-INTERFACE
    FirewallStateSwitchInterface                                           = 4979,
    /// FIRST-APPLICATION-ERROR-REF
    FirstApplicationErrorRef                                               = 2093,
    /// FIRST-DATA-PROTOTYPE-REF
    FirstDataPrototypeRef                                                  = 24,
    /// FIRST-ELEMENT-INSTANCE-IREF
    FirstElementInstanceIref                                               = 1207,
    /// FIRST-ELEMENT-INSTANCE-IREFS
    FirstElementInstanceIrefs                                              = 2685,
    /// FIRST-ELEMENT-REF
    FirstElementRef                                                        = 4971,
    /// FIRST-ELEMENT-REFS
    FirstElementRefs                                                       = 4658,
    /// FIRST-ELEMENTS
    FirstElements                                                          = 5163,
    /// FIRST-MODE-GROUP-REF
    FirstModeGroupRef                                                      = 629,
    /// FIRST-MODE-REF
    FirstModeRef                                                           = 186,
    /// FIRST-MODE-REFS
    FirstModeRefs                                                          = 5246,
    /// FIRST-OPERATION-REF
    FirstOperationRef                                                      = 5893,
    /// FIRST-PHYSICAL-DIMENSION-REF
    FirstPhysicalDimensionRef                                              = 3641,
    /// FIRST-PORT-REF
    FirstPortRef                                                           = 5175,
    /// FIRST-TIMEOUT
    FirstTimeout                                                           = 2387,
    /// FIRST-TO-SECOND-DATA-TRANSFORMATION-REF
    FirstToSecondDataTransformationRef                                     = 1793,
    /// FIRST-TRIGGER-REF
    FirstTriggerRef                                                        = 3772,
    /// FIRST-VALUE
    FirstValue                                                             = 404,
    /// FIXED-LENGTH
    FixedLength                                                            = 4043,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 1047,
    /// FLAT-INSTANCE-REF
    FlatInstanceRef                                                        = 5352,
    /// FLAT-MAP
    FlatMap                                                                = 1587,
    /// FLAT-MAP-ENTRY-REF
    FlatMapEntryRef                                                        = 6102,
    /// FLAT-MAP-ENTRY-REFS
    FlatMapEntryRefs                                                       = 4113,
    /// FLAT-MAP-REF
    FlatMapRef                                                             = 2605,
    /// FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING
    FlexrayAbsolutelyScheduledTiming                                       = 4406,
    /// FLEXRAY-AR-TP-CHANNEL
    FlexrayArTpChannel                                                     = 421,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 1527,
    /// FLEXRAY-AR-TP-CONNECTION
    FlexrayArTpConnection                                                  = 1605,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 5530,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 3459,
    /// FLEXRAY-CLUSTER-CONDITIONAL
    FlexrayClusterConditional                                              = 2576,
    /// FLEXRAY-CLUSTER-VARIANTS
    FlexrayClusterVariants                                                 = 3156,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 2281,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 3777,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL
    FlexrayCommunicationControllerConditional                              = 6159,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS
    FlexrayCommunicationControllerVariants                                 = 5788,
    /// FLEXRAY-FIFO-CONFIGURATION
    FlexrayFifoConfiguration                                               = 190,
    /// FLEXRAY-FIFO-RANGE
    FlexrayFifoRange                                                       = 5900,
    /// FLEXRAY-FIFOS
    FlexrayFifos                                                           = 2266,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 4324,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 5005,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 2840,
    /// FLEXRAY-NM-CLUSTER-COUPLING
    FlexrayNmClusterCoupling                                               = 2102,
    /// FLEXRAY-NM-ECU
    FlexrayNmEcu                                                           = 5584,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 1073,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 4429,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 4785,
    /// FLEXRAY-TP-CONNECTION
    FlexrayTpConnection                                                    = 3822,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 571,
    /// FLEXRAY-TP-ECU
    FlexrayTpEcu                                                           = 577,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 2192,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 5325,
    /// FLOAT
    Float                                                                  = 5104,
    /// FLOAT-VALUE-VARIATION-POINT
    FloatValueVariationPoint                                               = 5859,
    /// FLOW-CONTROL-PDU-REF
    FlowControlPduRef                                                      = 2098,
    /// FLOW-CONTROL-PDU-REFS
    FlowControlPduRefs                                                     = 5939,
    /// FLOW-CONTROL-REF
    FlowControlRef                                                         = 1035,
    /// FLOW-LABEL
    FlowLabel                                                              = 6231,
    /// FLOW-METERING-REF
    FlowMeteringRef                                                        = 5313,
    /// FLOW-METERINGS
    FlowMeterings                                                          = 5960,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 5835,
    /// FM-ATTRIBUTE-VALUE
    FmAttributeValue                                                       = 1442,
    /// FM-COND
    FmCond                                                                 = 4277,
    /// FM-FEATURE
    FmFeature                                                              = 2785,
    /// FM-FEATURE-DECOMPOSITION
    FmFeatureDecomposition                                                 = 670,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 4679,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 4895,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 4290,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 2499,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 3696,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 3263,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 6206,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 5747,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 4593,
    /// FM-SYSCOND
    FmSyscond                                                              = 4616,
    /// FMI
    Fmi                                                                    = 4339,
    /// FOLLOW-UP-ACTION-REF
    FollowUpActionRef                                                      = 5270,
    /// FOLLOW-UP-ACTION-REFS
    FollowUpActionRefs                                                     = 4687,
    /// FOLLOW-UP-OFFSET
    FollowUpOffset                                                         = 2416,
    /// FOLLOW-UP-TIMEOUT-VALUE
    FollowUpTimeoutValue                                                   = 212,
    /// FORBIDDEN-SIGNAL-PATH
    ForbiddenSignalPath                                                    = 5839,
    /// FOREIGN-MODEL-REFERENCE
    ForeignModelReference                                                  = 5060,
    /// FORMAL-BLUEPRINT-CONDITION
    FormalBlueprintCondition                                               = 5133,
    /// FORMAL-BLUEPRINT-GENERATOR
    FormalBlueprintGenerator                                               = 6224,
    /// FORMAT
    Format                                                                 = 2845,
    /// FORMULA
    Formula                                                                = 4650,
    /// FORMULA-CAPTION
    FormulaCaption                                                         = 3927,
    /// FR-CLUSTER-REF
    FrClusterRef                                                           = 4663,
    /// FR-GLOBAL-TIME-DOMAIN-PROPS
    FrGlobalTimeDomainProps                                                = 2461,
    /// FRAGMENT
    Fragment                                                               = 5273,
    /// FRAGMENTATION-PROPS
    FragmentationProps                                                     = 766,
    /// FRAME-LENGTH
    FrameLength                                                            = 2596,
    /// FRAME-MAPPING
    FrameMapping                                                           = 865,
    /// FRAME-MAPPINGS
    FrameMappings                                                          = 5524,
    /// FRAME-PID
    FramePid                                                               = 1543,
    /// FRAME-PIDS
    FramePids                                                              = 3946,
    /// FRAME-PORT
    FramePort                                                              = 5964,
    /// FRAME-PORT-REF
    FramePortRef                                                           = 3555,
    /// FRAME-PORT-REFS
    FramePortRefs                                                          = 5250,
    /// FRAME-REF
    FrameRef                                                               = 2603,
    /// FRAME-SYNC-ENABLED
    FrameSyncEnabled                                                       = 6273,
    /// FRAME-TRIGGERING-REF
    FrameTriggeringRef                                                     = 4398,
    /// FRAME-TRIGGERINGS
    FrameTriggerings                                                       = 2856,
    /// FREE-FORMAT
    FreeFormat                                                             = 141,
    /// FREEZE-CURRENT-STATE
    FreezeCurrentState                                                     = 87,
    /// FREEZE-CURRENT-STATE-SUPPORTED
    FreezeCurrentStateSupported                                            = 3397,
    /// FREEZE-FRAME-CONTENT-REF
    FreezeFrameContentRef                                                  = 2766,
    /// FREEZE-FRAME-CONTENT-WWH-OBD-REF
    FreezeFrameContentWwhObdRef                                            = 1230,
    /// FREEZE-FRAME-REF
    FreezeFrameRef                                                         = 4626,
    /// FREEZE-FRAMES
    FreezeFrames                                                           = 4801,
    /// FRESHNESS-COUNTER-SYNC-ATTEMPTS
    FreshnessCounterSyncAttempts                                           = 1656,
    /// FRESHNESS-PROPS-REF
    FreshnessPropsRef                                                      = 3080,
    /// FRESHNESS-PROPSS
    FreshnessPropss                                                        = 735,
    /// FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR
    FreshnessTimestampTimePeriodFactor                                     = 4153,
    /// FRESHNESS-VALUE-ID
    FreshnessValueId                                                       = 5061,
    /// FRESHNESS-VALUE-LENGTH
    FreshnessValueLength                                                   = 2054,
    /// FRESHNESS-VALUE-TX-LENGTH
    FreshnessValueTxLength                                                 = 4223,
    /// FROM-REF
    FromRef                                                                = 2519,
    /// FT
    Ft                                                                     = 4521,
    /// FULL-DUPLEX-ENABLED
    FullDuplexEnabled                                                      = 4698,
    /// FULLY-QUALIFIED-DOMAIN-NAME
    FullyQualifiedDomainName                                               = 529,
    /// FUNCTION
    Function                                                               = 6034,
    /// FUNCTION-CLUSTER-AFFILIATION
    FunctionClusterAffiliation                                             = 5546,
    /// FUNCTION-GROUP-IREF
    FunctionGroupIref                                                      = 1480,
    /// FUNCTION-GROUP-IREFS
    FunctionGroupIrefs                                                     = 1967,
    /// FUNCTION-GROUP-MODE-IREF
    FunctionGroupModeIref                                                  = 2225,
    /// FUNCTION-GROUP-MODE-IREFS
    FunctionGroupModeIrefs                                                 = 4617,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 2068,
    /// FUNCTION-GROUP-PHM-STATE-REFERENCE
    FunctionGroupPhmStateReference                                         = 3101,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 2984,
    /// FUNCTION-GROUP-STATE-IREF
    FunctionGroupStateIref                                                 = 5517,
    /// FUNCTION-GROUP-STATE-IREFS
    FunctionGroupStateIrefs                                                = 1888,
    /// FUNCTION-GROUPS
    FunctionGroups                                                         = 4110,
    /// FUNCTION-ID
    FunctionId                                                             = 3540,
    /// FUNCTION-IDENTIFIER-REF
    FunctionIdentifierRef                                                  = 1686,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 582,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 4900,
    /// FUNCTION-INSTANCE
    FunctionInstance                                                       = 880,
    /// FUNCTION-NAME
    FunctionName                                                           = 976,
    /// FUNCTION-NAMES
    FunctionNames                                                          = 337,
    /// FUNCTION-POINTER-SIGNATURE-REF
    FunctionPointerSignatureRef                                            = 5428,
    /// FUNCTION-PROTOTYPE-EMITTER
    FunctionPrototypeEmitter                                               = 5990,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING
    FunctionalClusterInteractsWithPersistencyDeploymentMapping             = 5852,
    /// FUNCTIONAL-CLUSTER-REF
    FunctionalClusterRef                                                   = 1855,
    /// FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING
    FunctionalClusterToSecurityEventDefinitionMapping                      = 5694,
    /// FUNCTIONAL-REQUEST-REF
    FunctionalRequestRef                                                   = 3943,
    /// FUNCTIONAL-REQUEST-REFS
    FunctionalRequestRefs                                                  = 3244,
    /// FUNCTIONAL-UNIT
    FunctionalUnit                                                         = 4394,
    /// FUP-DATA-ID-LIST
    FupDataIdList                                                          = 1245,
    /// FUP-DATA-ID-LISTS
    FupDataIdLists                                                         = 1443,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 886,
    /// GATEWAY
    Gateway                                                                = 2267,
    /// GATEWAYS
    Gateways                                                               = 911,
    /// GENERAL-INACTIVITY-TIME
    GeneralInactivityTime                                                  = 3061,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 5953,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 4372,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 4187,
    /// GENERATED-ARTIFACTS
    GeneratedArtifacts                                                     = 5470,
    /// GENERATOR
    Generator                                                              = 4218,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 3213,
    /// GENERIC-MATH
    GenericMath                                                            = 4981,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 5581,
    /// GENERIC-TP
    GenericTp                                                              = 5032,
    /// GET
    Get                                                                    = 3261,
    /// GETS
    Gets                                                                   = 916,
    /// GETTER-CALL-REF
    GetterCallRef                                                          = 5436,
    /// GETTER-CALL-SIGNAL-REF
    GetterCallSignalRef                                                    = 4032,
    /// GETTER-OPERATION-REF
    GetterOperationRef                                                     = 4319,
    /// GETTER-REF
    GetterRef                                                              = 4701,
    /// GETTER-RETURN-REF
    GetterReturnRef                                                        = 4302,
    /// GETTER-RETURN-SIGNAL-REF
    GetterReturnSignalRef                                                  = 1253,
    /// GID
    Gid                                                                    = 6227,
    /// GID-INVALIDITY-PATTERN
    GidInvalidityPattern                                                   = 223,
    /// GLOBAL-ELEMENT
    GlobalElement                                                          = 4656,
    /// GLOBAL-ELEMENTS
    GlobalElements                                                         = 700,
    /// GLOBAL-IN-PACKAGE-REF
    GlobalInPackageRef                                                     = 2739,
    /// GLOBAL-IN-PACKAGE-REFS
    GlobalInPackageRefs                                                    = 4867,
    /// GLOBAL-KAY-PROPS-REF
    GlobalKayPropsRef                                                      = 5831,
    /// GLOBAL-RESOURCE-ID
    GlobalResourceId                                                       = 2916,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 4982,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 5396,
    /// GLOBAL-SUPERVISION-ENTITYS
    GlobalSupervisionEntitys                                               = 1474,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 5593,
    /// GLOBAL-SUPERVISIONS
    GlobalSupervisions                                                     = 3364,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 660,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 5227,
    /// GLOBAL-TIME-CORRECTION-PROPS
    GlobalTimeCorrectionProps                                              = 2260,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 2363,
    /// GLOBAL-TIME-DOMAIN-PROPERTYS
    GlobalTimeDomainPropertys                                              = 1522,
    /// GLOBAL-TIME-DOMAIN-PROPS
    GlobalTimeDomainProps                                                  = 307,
    /// GLOBAL-TIME-DOMAIN-REF
    GlobalTimeDomainRef                                                    = 5862,
    /// GLOBAL-TIME-DOMAIN-REF-CONDITIONAL
    GlobalTimeDomainRefConditional                                         = 1797,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 2909,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 149,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 4721,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 3906,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 5649,
    /// GLOBAL-TIME-MASTERS
    GlobalTimeMasters                                                      = 5102,
    /// GLOBAL-TIME-PDU-REF
    GlobalTimePduRef                                                       = 207,
    /// GLOBAL-TIME-PDU-TRIGGERING-REF
    GlobalTimePduTriggeringRef                                             = 3494,
    /// GLOBAL-TIME-PORT-ROLE
    GlobalTimePortRole                                                     = 5128,
    /// GLOBAL-TIME-PROPS
    GlobalTimeProps                                                        = 4311,
    /// GLOBAL-TIME-SUB-DOMAINS
    GlobalTimeSubDomains                                                   = 1521,
    /// GLOBAL-TIME-TX-PERIOD
    GlobalTimeTxPeriod                                                     = 2871,
    /// GOVERNANCE-REF
    GovernanceRef                                                          = 22,
    /// GRANT-REF
    GrantRef                                                               = 1885,
    /// GRANT-REFS
    GrantRefs                                                              = 2993,
    /// GRAPHIC
    Graphic                                                                = 6212,
    /// GROUP-ID
    GroupId                                                                = 376,
    /// GROUP-IDENTIFIER
    GroupIdentifier                                                        = 105,
    /// GROUP-NUMBER
    GroupNumber                                                            = 3358,
    /// GROUPED-ALIAS-EVENT-REF
    GroupedAliasEventRef                                                   = 5618,
    /// GROUPED-ALIAS-EVENT-REFS
    GroupedAliasEventRefs                                                  = 2764,
    /// HANDLE-DATA-STATUS
    HandleDataStatus                                                       = 1519,
    /// HANDLE-INVALID
    HandleInvalid                                                          = 420,
    /// HANDLE-NEVER-RECEIVED
    HandleNeverReceived                                                    = 4765,
    /// HANDLE-OUT-OF-RANGE
    HandleOutOfRange                                                       = 4025,
    /// HANDLE-OUT-OF-RANGE-STATUS
    HandleOutOfRangeStatus                                                 = 1591,
    /// HANDLE-TERMINATION-AND-RESTART
    HandleTerminationAndRestart                                            = 4564,
    /// HANDLE-TIMEOUT-TYPE
    HandleTimeoutType                                                      = 5683,
    /// HARDWARE-CONFIGURATION
    HardwareConfiguration                                                  = 4209,
    /// HARDWARE-ELEMENT-REF
    HardwareElementRef                                                     = 829,
    /// HARDWARE-ELEMENT-REFS
    HardwareElementRefs                                                    = 5933,
    /// HARDWARE-PLATFORM
    HardwarePlatform                                                       = 2176,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 3613,
    /// HAS-DYNAMIC-LENGTH
    HasDynamicLength                                                       = 265,
    /// HAS-GETTER
    HasGetter                                                              = 2900,
    /// HAS-INTERNAL-STATE
    HasInternalState                                                       = 532,
    /// HAS-NOTIFIER
    HasNotifier                                                            = 3554,
    /// HAS-SETTER
    HasSetter                                                              = 5864,
    /// HEADER-FILE
    HeaderFile                                                             = 2005,
    /// HEADER-ID
    HeaderId                                                               = 4416,
    /// HEADER-ID-LONG-HEADER
    HeaderIdLongHeader                                                     = 4212,
    /// HEADER-ID-SHORT-HEADER
    HeaderIdShortHeader                                                    = 1334,
    /// HEADER-LENGTH
    HeaderLength                                                           = 2914,
    /// HEADER-TYPE
    HeaderType                                                             = 2710,
    /// HEALING-CYCLE-COUNTER-THRESHOLD
    HealingCycleCounterThreshold                                           = 3189,
    /// HEALING-CYCLE-REF
    HealingCycleRef                                                        = 2136,
    /// HEALTH-CHANNEL-ARGUMENT-REF
    HealthChannelArgumentRef                                               = 1335,
    /// HEALTH-CHANNEL-ARGUMENT-REFS
    HealthChannelArgumentRefs                                              = 5968,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 1092,
    /// HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS
    HealthChannelExternalReportedStatus                                    = 1413,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 807,
    /// HEALTH-CHANNEL-ID
    HealthChannelId                                                        = 1407,
    /// HEALTH-CHANNEL-IREF
    HealthChannelIref                                                      = 4964,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 3733,
    /// HEALTH-CHANNELS
    HealthChannels                                                         = 4106,
    /// HEAP-USAGES
    HeapUsages                                                             = 718,
    /// HISTORY
    History                                                                = 4426,
    /// HISTORY-KIND
    HistoryKind                                                            = 4694,
    /// HISTORY-ORDER-DEPTH
    HistoryOrderDepth                                                      = 6005,
    /// HOLD-OVER-TIME
    HoldOverTime                                                           = 2034,
    /// HOP-COUNT
    HopCount                                                               = 1242,
    /// HOP-LIMIT
    HopLimit                                                               = 3325,
    /// HOST-ID
    HostId                                                                 = 4894,
    /// HOST-REF
    HostRef                                                                = 4813,
    /// HOST-SYSTEM-REF
    HostSystemRef                                                          = 3380,
    /// HTTP-ACCEPT-ENCODING
    HttpAcceptEncoding                                                     = 3413,
    /// HTTP-TP
    HttpTp                                                                 = 3982,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 206,
    /// HW-ATTRIBUTE-DEF-REF
    HwAttributeDefRef                                                      = 2749,
    /// HW-ATTRIBUTE-DEFS
    HwAttributeDefs                                                        = 4602,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 4814,
    /// HW-ATTRIBUTE-LITERALS
    HwAttributeLiterals                                                    = 4444,
    /// HW-ATTRIBUTE-VALUE
    HwAttributeValue                                                       = 5323,
    /// HW-ATTRIBUTE-VALUES
    HwAttributeValues                                                      = 1874,
    /// HW-CATEGORY
    HwCategory                                                             = 2850,
    /// HW-CATEGORY-REF
    HwCategoryRef                                                          = 53,
    /// HW-CATEGORY-REFS
    HwCategoryRefs                                                         = 3086,
    /// HW-COMMUNICATION-CONTROLLER-REF
    HwCommunicationControllerRef                                           = 5651,
    /// HW-COMMUNICATION-PORT-REF
    HwCommunicationPortRef                                                 = 1890,
    /// HW-ELEMENT
    HwElement                                                              = 4418,
    /// HW-ELEMENT-CONNECTIONS
    HwElementConnections                                                   = 2716,
    /// HW-ELEMENT-CONNECTOR
    HwElementConnector                                                     = 5782,
    /// HW-ELEMENT-REF
    HwElementRef                                                           = 5157,
    /// HW-ELEMENT-REF-CONDITIONAL
    HwElementRefConditional                                                = 1802,
    /// HW-ELEMENT-REFS
    HwElementRefs                                                          = 1599,
    /// HW-PIN
    HwPin                                                                  = 953,
    /// HW-PIN-CONNECTIONS
    HwPinConnections                                                       = 5827,
    /// HW-PIN-CONNECTOR
    HwPinConnector                                                         = 3291,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 1357,
    /// HW-PIN-GROUP-CONNECTIONS
    HwPinGroupConnections                                                  = 1892,
    /// HW-PIN-GROUP-CONNECTOR
    HwPinGroupConnector                                                    = 2123,
    /// HW-PIN-GROUP-CONTENT
    HwPinGroupContent                                                      = 1759,
    /// HW-PIN-GROUP-REF
    HwPinGroupRef                                                          = 3739,
    /// HW-PIN-GROUP-REFS
    HwPinGroupRefs                                                         = 2806,
    /// HW-PIN-GROUPS
    HwPinGroups                                                            = 1726,
    /// HW-PIN-REF
    HwPinRef                                                               = 3240,
    /// HW-PIN-REFS
    HwPinRefs                                                              = 881,
    /// HW-PORT-MAPPING
    HwPortMapping                                                          = 2308,
    /// HW-PORT-MAPPINGS
    HwPortMappings                                                         = 3421,
    /// HW-TYPE
    HwType                                                                 = 5971,
    /// HW-TYPE-REF
    HwTypeRef                                                              = 5871,
    /// I-PDU-IDENTIFIER-TCP-REF
    IPduIdentifierTcpRef                                                   = 6012,
    /// I-PDU-IDENTIFIER-TCP-REFS
    IPduIdentifierTcpRefs                                                  = 5363,
    /// I-PDU-IDENTIFIER-UDP-REF
    IPduIdentifierUdpRef                                                   = 4475,
    /// I-PDU-IDENTIFIER-UDP-REFS
    IPduIdentifierUdpRefs                                                  = 947,
    /// I-PDU-IDENTIFIERS
    IPduIdentifiers                                                        = 3744,
    /// I-PDU-MAPPING
    IPduMapping                                                            = 4547,
    /// I-PDU-MAPPINGS
    IPduMappings                                                           = 391,
    /// I-PDU-PORT
    IPduPort                                                               = 6185,
    /// I-PDU-PORT-REF
    IPduPortRef                                                            = 4036,
    /// I-PDU-PORT-REFS
    IPduPortRefs                                                           = 1266,
    /// I-PDU-REF
    IPduRef                                                                = 4786,
    /// I-PDU-SIGNAL-PROCESSING
    IPduSignalProcessing                                                   = 2259,
    /// I-PDU-TIMING
    IPduTiming                                                             = 3732,
    /// I-PDU-TIMING-SPECIFICATIONS
    IPduTimingSpecifications                                               = 1663,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 4925,
    /// I-PDU-TRIGGERING-REF
    IPduTriggeringRef                                                      = 863,
    /// I-PDU-TRIGGERING-REF-CONDITIONAL
    IPduTriggeringRefConditional                                           = 5016,
    /// I-PDU-TRIGGERINGS
    IPduTriggerings                                                        = 4789,
    /// I-PDUS
    IPdus                                                                  = 3438,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 3337,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 4383,
    /// I-SIGNAL
    ISignal                                                                = 4524,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 4322,
    /// I-SIGNAL-GROUP-REF
    ISignalGroupRef                                                        = 3402,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 664,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 5928,
    /// I-SIGNAL-I-PDU-REF
    ISignalIPduRef                                                         = 6244,
    /// I-SIGNAL-I-PDU-REF-CONDITIONAL
    ISignalIPduRefConditional                                              = 2674,
    /// I-SIGNAL-I-PDUS
    ISignalIPdus                                                           = 3592,
    /// I-SIGNAL-IN-I-PDU-REF
    ISignalInIPduRef                                                       = 827,
    /// I-SIGNAL-MAPPING
    ISignalMapping                                                         = 1063,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 3039,
    /// I-SIGNAL-PORT-REF
    ISignalPortRef                                                         = 3957,
    /// I-SIGNAL-PORT-REFS
    ISignalPortRefs                                                        = 1817,
    /// I-SIGNAL-PROPS
    ISignalProps                                                           = 2921,
    /// I-SIGNAL-REF
    ISignalRef                                                             = 5343,
    /// I-SIGNAL-REFS
    ISignalRefs                                                            = 5601,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 2007,
    /// I-SIGNAL-TO-I-PDU-MAPPINGS
    ISignalToIPduMappings                                                  = 1384,
    /// I-SIGNAL-TO-PDU-MAPPINGS
    ISignalToPduMappings                                                   = 5932,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 2602,
    /// I-SIGNAL-TRIGGERING-REF
    ISignalTriggeringRef                                                   = 5450,
    /// I-SIGNAL-TRIGGERING-REF-CONDITIONAL
    ISignalTriggeringRefConditional                                        = 4486,
    /// I-SIGNAL-TRIGGERINGS
    ISignalTriggerings                                                     = 393,
    /// I-SIGNAL-TYPE
    ISignalType                                                            = 3913,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 3112,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF
    IamRelevantTlsSecureComPropsRef                                        = 6026,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS
    IamRelevantTlsSecureComPropsRefs                                       = 3247,
    /// ICMP-RULE
    IcmpRule                                                               = 5979,
    /// ICMP-V-4-PROPS
    IcmpV4Props                                                            = 1143,
    /// ICMP-V-6-PROPS
    IcmpV6Props                                                            = 3720,
    /// ICV-SECURED
    IcvSecured                                                             = 3446,
    /// ICV-VERIFICATION
    IcvVerification                                                        = 5108,
    /// ID
    Id                                                                     = 5105,
    /// IDENT
    Ident                                                                  = 2598,
    /// IDENTICAL-MAPPING
    IdenticalMapping                                                       = 4342,
    /// IDENTIFIABLE-REF
    IdentifiableRef                                                        = 3481,
    /// IDENTIFIER
    Identifier                                                             = 3355,
    /// IDENTITIY-NUMBER
    IdentitiyNumber                                                        = 1325,
    /// IDENTITY
    Identity                                                               = 1682,
    /// IDENTITY-CERTIFICATE-AUTHORITY-REF
    IdentityCertificateAuthorityRef                                        = 622,
    /// IDENTITY-REF
    IdentityRef                                                            = 3340,
    /// IDLE-SLOPE
    IdleSlope                                                              = 4683,
    /// IDS-COMMON-ELEMENT-REF
    IdsCommonElementRef                                                    = 1743,
    /// IDS-COMMON-ELEMENT-REF-CONDITIONAL
    IdsCommonElementRefConditional                                         = 1671,
    /// IDS-DESIGN
    IdsDesign                                                              = 3905,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 5200,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 638,
    /// IDS-PLATFORM-INSTANTIATION-REF
    IdsPlatformInstantiationRef                                            = 509,
    /// IDSM-CONTEXT-PROVIDER-INTERFACE
    IdsmContextProviderInterface                                           = 6147,
    /// IDSM-CONTEXT-PROVIDER-MAPPING
    IdsmContextProviderMapping                                             = 5139,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 3652,
    /// IDSM-INSTANCE-ID
    IdsmInstanceId                                                         = 4282,
    /// IDSM-INSTANCE-REF
    IdsmInstanceRef                                                        = 1324,
    /// IDSM-INSTANCE-REF-CONDITIONAL
    IdsmInstanceRefConditional                                             = 5493,
    /// IDSM-INSTANCES
    IdsmInstances                                                          = 1445,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 2604,
    /// IDSM-MODULE-INSTANTIATION-REF
    IdsmModuleInstantiationRef                                             = 5014,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 432,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 4892,
    /// IDSM-RATE-LIMITATION-REF
    IdsmRateLimitationRef                                                  = 2271,
    /// IDSM-RATE-LIMITATION-REF-CONDITIONAL
    IdsmRateLimitationRefConditional                                       = 3955,
    /// IDSM-TIMESTAMP-PROVIDER-INTERFACE
    IdsmTimestampProviderInterface                                         = 3320,
    /// IDSM-TIMESTAMP-PROVIDER-MAPPING
    IdsmTimestampProviderMapping                                           = 3393,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 5555,
    /// IDSM-TRAFFIC-LIMITATION-REF
    IdsmTrafficLimitationRef                                               = 3223,
    /// IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL
    IdsmTrafficLimitationRefConditional                                    = 368,
    /// IE
    Ie                                                                     = 4409,
    /// IEE-E-1722-TP-CONNECTION-REF
    IeeE1722TpConnectionRef                                                = 5499,
    /// IEEE-1722-TP
    Ieee1722Tp                                                             = 4371,
    /// IEEE-1722-TP-AAF-CONNECTION
    Ieee1722TpAafConnection                                                = 4275,
    /// IEEE-1722-TP-ACF-CAN
    Ieee1722TpAcfCan                                                       = 5395,
    /// IEEE-1722-TP-ACF-CAN-PART
    Ieee1722TpAcfCanPart                                                   = 5335,
    /// IEEE-1722-TP-ACF-CONNECTION
    Ieee1722TpAcfConnection                                                = 5620,
    /// IEEE-1722-TP-ACF-LIN
    Ieee1722TpAcfLin                                                       = 2717,
    /// IEEE-1722-TP-ACF-LIN-PART
    Ieee1722TpAcfLinPart                                                   = 16,
    /// IEEE-1722-TP-CONFIG
    Ieee1722TpConfig                                                       = 988,
    /// IEEE-1722-TP-CONNECTION-REF-CONDITIONAL
    Ieee1722TpConnectionRefConditional                                     = 3919,
    /// IEEE-1722-TP-CRF-CONNECTION
    Ieee1722TpCrfConnection                                                = 3610,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 1502,
    /// IEEE-1722-TP-IIDC-CONNECTION
    Ieee1722TpIidcConnection                                               = 1364,
    /// IEEE-1722-TP-RULE
    Ieee1722TpRule                                                         = 286,
    /// IEEE-1722-TP-RVF-CONNECTION
    Ieee1722TpRvfConnection                                                = 1706,
    /// IGNORE-AFTER-TX
    IgnoreAfterTx                                                          = 4707,
    /// IGNORE-ORDER-ALLOWED
    IgnoreOrderAllowed                                                     = 6004,
    /// IIDC-CHANNEL
    IidcChannel                                                            = 1371,
    /// IIDC-DATA-BLOCK-SIZE
    IidcDataBlockSize                                                      = 2989,
    /// IIDC-FRACTION-NUMBER
    IidcFractionNumber                                                     = 4474,
    /// IIDC-SOURCE-PACKET-HEADER
    IidcSourcePacketHeader                                                 = 3497,
    /// IIDC-STREAM-FORMAT
    IidcStreamFormat                                                       = 1632,
    /// IIDC-SY
    IidcSy                                                                 = 6161,
    /// IIDC-T-CODE
    IidcTCode                                                              = 3014,
    /// IIDC-TAG
    IidcTag                                                                = 4576,
    /// IKE-AUTHENTICATION-METHOD
    IkeAuthenticationMethod                                                = 1569,
    /// IKE-CIPHER-SUITE-NAME
    IkeCipherSuiteName                                                     = 2394,
    /// IKE-OVER-TIME
    IkeOverTime                                                            = 2677,
    /// IKE-RAND-TIME
    IkeRandTime                                                            = 4247,
    /// IKE-REAUTH-TIME
    IkeReauthTime                                                          = 1098,
    /// IKE-REKEY-TIME
    IkeRekeyTime                                                           = 1424,
    /// IMMEDIATE-NV-DATA-STORAGE
    ImmediateNvDataStorage                                                 = 1903,
    /// IMMEDIATE-RESUME-TIME
    ImmediateResumeTime                                                    = 2285,
    /// IMPL-CONSTANT-REF
    ImplConstantRef                                                        = 4513,
    /// IMPL-INIT-VALUE
    ImplInitValue                                                          = 1920,
    /// IMPLEMENTATION-ARRAY-ELEMENT-REF
    ImplementationArrayElementRef                                          = 4567,
    /// IMPLEMENTATION-CONFIG-CLASSES
    ImplementationConfigClasses                                            = 3661,
    /// IMPLEMENTATION-CONFIG-VARIANT
    ImplementationConfigVariant                                            = 528,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 1829,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 490,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 5557,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF
    ImplementationDataTypeElementInAutosarDataPrototypeRef                 = 5374,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF
    ImplementationDataTypeElementInPortInterfaceRef                        = 6150,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF
    ImplementationDataTypeElementInSystemRef                               = 4374,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    ImplementationDataTypeElementRef                                       = 4672,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 2065,
    /// IMPLEMENTATION-DATA-TYPE-REF
    ImplementationDataTypeRef                                              = 4905,
    /// IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF
    ImplementationDataTypeSubElementRef                                    = 1148,
    /// IMPLEMENTATION-PROPSS
    ImplementationPropss                                                   = 2418,
    /// IMPLEMENTATION-RECORD-ELEMENT-REF
    ImplementationRecordElementRef                                         = 5188,
    /// IMPLEMENTATION-REF
    ImplementationRef                                                      = 2153,
    /// IMPLEMENTATION-REFS
    ImplementationRefs                                                     = 6007,
    /// IMPLEMENTED-ENTRY-REF
    ImplementedEntryRef                                                    = 2384,
    /// IMPLEMENTED-ENTRYS
    ImplementedEntrys                                                      = 1366,
    /// IMPLEMENTED-IN-REF
    ImplementedInRef                                                       = 694,
    /// IMPLEMENTS-LEGACY-STRING-SERIALIZATION
    ImplementsLegacyStringSerialization                                    = 4034,
    /// IMPLEMENTS-SOMEIP-STRING-HANDLING
    ImplementsSomeipStringHandling                                         = 3841,
    /// IMPLICIT-DATA-ACCESS-IREF
    ImplicitDataAccessIref                                                 = 2906,
    /// IMPLICIT-DATA-ACCESS-IREFS
    ImplicitDataAccessIrefs                                                = 6187,
    /// IMPLICIT-INTER-RUNNABLE-VARIABLES
    ImplicitInterRunnableVariables                                         = 4412,
    /// IMPLIED-OPERATION-REF
    ImpliedOperationRef                                                    = 5190,
    /// IMPLIED-OPERATION-REFS
    ImpliedOperationRefs                                                   = 2246,
    /// IMPORTANCE
    Importance                                                             = 1284,
    /// IMPOSITION-TIME
    ImpositionTime                                                         = 1212,
    /// IMPOSITION-TIME-DEFINITION-GROUP
    ImpositionTimeDefinitionGroup                                          = 4363,
    /// IMPOSITION-TIMES
    ImpositionTimes                                                        = 2753,
    /// IN-MEASUREMENT-SET
    InMeasurementSet                                                       = 4452,
    /// IN-PLACE
    InPlace                                                                = 1751,
    /// IN-SCOPE
    InScope                                                                = 6052,
    /// INCLUDE-REF
    IncludeRef                                                             = 2517,
    /// INCLUDE-REFS
    IncludeRefs                                                            = 5165,
    /// INCLUDED-DATA-TYPE-SET
    IncludedDataTypeSet                                                    = 740,
    /// INCLUDED-DATA-TYPE-SETS
    IncludedDataTypeSets                                                   = 3044,
    /// INCLUDED-LIBRARY-REF
    IncludedLibraryRef                                                     = 4349,
    /// INCLUDED-LIBRARY-REFS
    IncludedLibraryRefs                                                    = 4193,
    /// INCLUDED-MODE-DECLARATION-GROUP-SET
    IncludedModeDeclarationGroupSet                                        = 1079,
    /// INCLUDED-MODE-DECLARATION-GROUP-SETS
    IncludedModeDeclarationGroupSets                                       = 4093,
    /// INCLUDED-VARIANT-REF
    IncludedVariantRef                                                     = 4294,
    /// INCLUDED-VARIANT-REFS
    IncludedVariantRefs                                                    = 1540,
    /// INDENT-SAMPLE
    IndentSample                                                           = 896,
    /// INDEX
    Index                                                                  = 2630,
    /// INDEX-DATA-TYPE-REF
    IndexDataTypeRef                                                       = 1835,
    /// INDEXED-ARRAY-ELEMENT
    IndexedArrayElement                                                    = 3287,
    /// INDICATIONS
    Indications                                                            = 2300,
    /// INDICATOR-FAILURE-CYCLE-COUNTER-THRESHOLD
    IndicatorFailureCycleCounterThreshold                                  = 1796,
    /// INDICATOR-REF
    IndicatorRef                                                           = 1114,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 4119,
    /// INDIRECT-API
    IndirectApi                                                            = 1680,
    /// INDUSTRY-GROUP
    IndustryGroup                                                          = 3282,
    /// INFO-TYPE
    InfoType                                                               = 4837,
    /// INFO-TYPE-REF
    InfoTypeRef                                                            = 4177,
    /// INFORMAL-FORMULA
    InformalFormula                                                        = 5203,
    /// INFRASTRUCTURE-SERVICES
    InfrastructureServices                                                 = 4821,
    /// INGRESS-PORT-REF
    IngressPortRef                                                         = 4407,
    /// INGRESS-PORT-REFS
    IngressPortRefs                                                        = 5639,
    /// INGRESS-PRIORITY
    IngressPriority                                                        = 1177,
    /// INHIBIT-SOURCES
    InhibitSources                                                         = 4574,
    /// INHIBITING-FID-REF
    InhibitingFidRef                                                       = 5425,
    /// INHIBITING-SECONDARY-FID-REF
    InhibitingSecondaryFidRef                                              = 4476,
    /// INHIBITING-SECONDARY-FID-REFS
    InhibitingSecondaryFidRefs                                             = 4609,
    /// INHIBITION-MASK
    InhibitionMask                                                         = 996,
    /// INHIBITION-SOURCE-REF
    InhibitionSourceRef                                                    = 6098,
    /// INIT-EVENT
    InitEvent                                                              = 3740,
    /// INIT-RESOURCE
    InitResource                                                           = 3582,
    /// INIT-VALUE
    InitValue                                                              = 3404,
    /// INIT-VALUES
    InitValues                                                             = 6037,
    /// INITIAL-CHECKPOINT-REF
    InitialCheckpointRef                                                   = 6237,
    /// INITIAL-CHECKPOINT-REFS
    InitialCheckpointRefs                                                  = 5107,
    /// INITIAL-DELAY-MAX-VALUE
    InitialDelayMaxValue                                                   = 1389,
    /// INITIAL-DELAY-MIN-VALUE
    InitialDelayMinValue                                                   = 1780,
    /// INITIAL-DYNAMIC-PART
    InitialDynamicPart                                                     = 3958,
    /// INITIAL-EVENT-STATUS
    InitialEventStatus                                                     = 6277,
    /// INITIAL-FIND-BEHAVIOR
    InitialFindBehavior                                                    = 3309,
    /// INITIAL-INACTIVITY-TIME
    InitialInactivityTime                                                  = 955,
    /// INITIAL-MODE-REF
    InitialModeRef                                                         = 6038,
    /// INITIAL-NAD
    InitialNad                                                             = 5606,
    /// INITIAL-OFFER-BEHAVIOR
    InitialOfferBehavior                                                   = 834,
    /// INITIAL-REF-OFFSET
    InitialRefOffset                                                       = 3559,
    /// INITIAL-REPETITIONS-BASE-DELAY
    InitialRepetitionsBaseDelay                                            = 1290,
    /// INITIAL-REPETITIONS-MAX
    InitialRepetitionsMax                                                  = 5040,
    /// INITIAL-STATUS
    InitialStatus                                                          = 3077,
    /// INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    InitialVehicleAnnouncementTime                                         = 9,
    /// INITIALIZATION-VECTOR-LENGTH
    InitializationVectorLength                                             = 2056,
    /// INITIALIZED-PARAMETER-REF
    InitializedParameterRef                                                = 3799,
    /// INNER-GROUP-IREF
    InnerGroupIref                                                         = 2137,
    /// INNER-GROUP-IREFS
    InnerGroupIrefs                                                        = 3134,
    /// INNER-PORT-IREF
    InnerPortIref                                                          = 820,
    /// INPLACE
    Inplace                                                                = 5026,
    /// INPUT
    Input                                                                  = 107,
    /// INPUT-DATAS
    InputDatas                                                             = 2122,
    /// INPUT-VARIABLE-TYPE-REF
    InputVariableTypeRef                                                   = 4708,
    /// INSTALLATION-BEHAVIOR
    InstallationBehavior                                                   = 1959,
    /// INSTANCE-ID
    InstanceId                                                             = 4216,
    /// INSTANCE-ID-PROPS
    InstanceIdProps                                                        = 5064,
    /// INSTANCE-IDENTIFIER
    InstanceIdentifier                                                     = 2339,
    /// INSTANCE-IDS
    InstanceIds                                                            = 3328,
    /// INSTANCE-IN-MEMORY
    InstanceInMemory                                                       = 4005,
    /// INSTANCES
    Instances                                                              = 2934,
    /// INSTANTIATION-DATA-DEF-PROPS
    InstantiationDataDefProps                                              = 2940,
    /// INSTANTIATION-DATA-DEF-PROPSS
    InstantiationDataDefPropss                                             = 2727,
    /// INSTANTIATION-RTE-EVENT-PROPSS
    InstantiationRteEventPropss                                            = 4699,
    /// INSTANTIATION-TIMING-EVENT-PROPS
    InstantiationTimingEventProps                                          = 992,
    /// INTEGER-VALUE-VARIATION-POINT
    IntegerValueVariationPoint                                             = 553,
    /// INTENDED-FILENAME
    IntendedFilename                                                       = 2456,
    /// INTENDED-PARTIAL-INITIALIZATION-COUNT
    IntendedPartialInitializationCount                                     = 306,
    /// INTENDED-TARGET-MACHINE-REF
    IntendedTargetMachineRef                                               = 4033,
    /// INTENDED-TARGET-MACHINE-REFS
    IntendedTargetMachineRefs                                              = 2899,
    /// INTER-MESSAGE-TIME
    InterMessageTime                                                       = 5441,
    /// INTERFACE-BLUEPRINT-MAPPING
    InterfaceBlueprintMapping                                              = 4490,
    /// INTERFACE-BLUEPRINT-MAPPINGS
    InterfaceBlueprintMappings                                             = 2380,
    /// INTERFACE-BLUEPRINT-REF
    InterfaceBlueprintRef                                                  = 2107,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 2241,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 3986,
    /// INTERFACE-MAPPINGS
    InterfaceMappings                                                      = 2431,
    /// INTERFACE-REF
    InterfaceRef                                                           = 1800,
    /// INTERFACE-VERSION
    InterfaceVersion                                                       = 2395,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 83,
    /// INTERNAL-BEHAVIORS
    InternalBehaviors                                                      = 2070,
    /// INTERNAL-CONSTRS
    InternalConstrs                                                        = 2329,
    /// INTERNAL-PRIORITY-VALUE
    InternalPriorityValue                                                  = 5123,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 583,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 2283,
    /// INTERNAL-TRIGGERING-POINT-POLICYS
    InternalTriggeringPointPolicys                                         = 1715,
    /// INTERNAL-TRIGGERING-POINTS
    InternalTriggeringPoints                                               = 4368,
    /// INTERNET-HEADER-LENGTH
    InternetHeaderLength                                                   = 691,
    /// INTERPOLATION-ROUTINE
    InterpolationRoutine                                                   = 246,
    /// INTERPOLATION-ROUTINE-MAPPING
    InterpolationRoutineMapping                                            = 1308,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 2083,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REF
    InterpolationRoutineMappingSetRef                                      = 5891,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REFS
    InterpolationRoutineMappingSetRefs                                     = 1024,
    /// INTERPOLATION-ROUTINE-MAPPINGS
    InterpolationRoutineMappings                                           = 3498,
    /// INTERPOLATION-ROUTINE-REF
    InterpolationRoutineRef                                                = 317,
    /// INTERPOLATION-ROUTINES
    InterpolationRoutines                                                  = 4283,
    /// INTERRUPT-CATEGORY
    InterruptCategory                                                      = 3995,
    /// INTERRUPT-SOURCE
    InterruptSource                                                        = 4127,
    /// INTERVAL-LENGTH
    IntervalLength                                                         = 305,
    /// INTRODUCTION
    Introduction                                                           = 6086,
    /// INVALID-VALUE
    InvalidValue                                                           = 2832,
    /// INVALIDATION-POLICY
    InvalidationPolicy                                                     = 697,
    /// INVALIDATION-POLICYS
    InvalidationPolicys                                                    = 1593,
    /// INVERSE-PROTOCOL-VERSION
    InverseProtocolVersion                                                 = 3486,
    /// INVERT
    Invert                                                                 = 961,
    /// INVERT-CONDITION
    InvertCondition                                                        = 2877,
    /// INVOCATION
    Invocation                                                             = 308,
    /// IO-CONTROL-CLASS-REF
    IoControlClassRef                                                      = 987,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATION
    IoHwAbstractionServerAnnotation                                        = 4944,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATIONS
    IoHwAbstractionServerAnnotations                                       = 4068,
    /// IP-ADDRESS
    IpAddress                                                              = 3464,
    /// IP-ADDRESS-KEEP-BEHAVIOR
    IpAddressKeepBehavior                                                  = 2828,
    /// IP-ADDRESS-PREFIX-LENGTH
    IpAddressPrefixLength                                                  = 2819,
    /// IP-IAM-AUTHENTIC-CONNECTION-PROPS
    IpIamAuthenticConnectionProps                                          = 5337,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 2942,
    /// IP-PROTOCOL
    IpProtocol                                                             = 5376,
    /// IP-SEC-CONFIG
    IpSecConfig                                                            = 2439,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 4581,
    /// IP-SEC-CONFIG-PROPS-REF
    IpSecConfigPropsRef                                                    = 1425,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 1291,
    /// IP-SEC-RULE
    IpSecRule                                                              = 5235,
    /// IP-SEC-RULES
    IpSecRules                                                             = 1265,
    /// IP-TP-RULE
    IpTpRule                                                               = 1740,
    /// IP-V-4-ADDRESS
    IpV4Address                                                            = 4331,
    /// IP-V-6-ADDRESS
    IpV6Address                                                            = 6077,
    /// IP-V-6-PATH-MTU-ENABLED
    IpV6PathMtuEnabled                                                     = 5881,
    /// IP-V-6-PATH-MTU-TIMEOUT
    IpV6PathMtuTimeout                                                     = 6107,
    /// IPV-4-ADDRESS
    Ipv4Address                                                            = 4390,
    /// IPV-4-ADDRESS-MASK
    Ipv4AddressMask                                                        = 5380,
    /// IPV-4-ADDRESS-SOURCE
    Ipv4AddressSource                                                      = 4055,
    /// IPV-4-CONFIGURATION
    Ipv4Configuration                                                      = 4604,
    /// IPV-4-DHCP-SERVER-CONFIGURATION
    Ipv4DhcpServerConfiguration                                            = 5630,
    /// IPV-4-MULTICAST-IP-ADDRESS
    Ipv4MulticastIpAddress                                                 = 2493,
    /// IPV-4-PROPS
    Ipv4Props                                                              = 406,
    /// IPV-4-RULE
    Ipv4Rule                                                               = 1501,
    /// IPV-6-ADDRESS
    Ipv6Address                                                            = 2396,
    /// IPV-6-ADDRESS-MASK
    Ipv6AddressMask                                                        = 5676,
    /// IPV-6-ADDRESS-SOURCE
    Ipv6AddressSource                                                      = 1376,
    /// IPV-6-CONFIGURATION
    Ipv6Configuration                                                      = 789,
    /// IPV-6-DHCP-SERVER-CONFIGURATION
    Ipv6DhcpServerConfiguration                                            = 113,
    /// IPV-6-MULTICAST-IP-ADDRESS
    Ipv6MulticastIpAddress                                                 = 5886,
    /// IPV-6-PROPS
    Ipv6Props                                                              = 2562,
    /// IPV-6-RULE
    Ipv6Rule                                                               = 2826,
    /// IS-ACTIVATION-LINE-DEPENDENT
    IsActivationLineDependent                                              = 5949,
    /// IS-AUTO-VALUE
    IsAutoValue                                                            = 4539,
    /// IS-DEFAULT
    IsDefault                                                              = 5829,
    /// IS-DELTA-PACKAGE
    IsDeltaPackage                                                         = 3896,
    /// IS-DYNAMIC-LENGTH-FIELD-SIZE
    IsDynamicLengthFieldSize                                               = 1916,
    /// IS-EVENT
    IsEvent                                                                = 2741,
    /// IS-EXTERNAL
    IsExternal                                                             = 3506,
    /// IS-GLOBAL
    IsGlobal                                                               = 114,
    /// IS-MANDATORY
    IsMandatory                                                            = 3150,
    /// IS-OBD-RELEVANT
    IsObdRelevant                                                          = 970,
    /// IS-OPTIONAL
    IsOptional                                                             = 1037,
    /// IS-PIPELINING-PERMITTED
    IsPipeliningPermitted                                                  = 257,
    /// IS-PRIVATE
    IsPrivate                                                              = 5359,
    /// IS-REENTRANT
    IsReentrant                                                            = 2293,
    /// IS-REQUIRED
    IsRequired                                                             = 2370,
    /// IS-SERVICE
    IsService                                                              = 1506,
    /// IS-STRUCT-WITH-OPTIONAL-ELEMENT
    IsStructWithOptionalElement                                            = 1805,
    /// IS-SYNCHRONOUS
    IsSynchronous                                                          = 3205,
    /// IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER
    IsSystemWideGlobalTimeMaster                                           = 2848,
    /// IS-UNUSED
    IsUnused                                                               = 4510,
    /// IS-VARIADIC-TEMPLATE
    IsVariadicTemplate                                                     = 5361,
    /// ISSUED-BY
    IssuedBy                                                               = 2779,
    /// ISSUED-TRIGGERS
    IssuedTriggers                                                         = 3293,
    /// IT-REF
    ItRef                                                                  = 2720,
    /// ITEM
    Item                                                                   = 2280,
    /// ITEM-DEFINITIONS
    ItemDefinitions                                                        = 5934,
    /// ITEM-LABEL
    ItemLabel                                                              = 4188,
    /// ITEMS
    Items                                                                  = 5088,
    /// IUMPR-GROUP
    IumprGroup                                                             = 3345,
    /// IUMPR-GROUP-IDENTIFIERS
    IumprGroupIdentifiers                                                  = 6275,
    /// IUMPR-REF
    IumprRef                                                               = 5196,
    /// IUMPR-REFS
    IumprRefs                                                              = 1536,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 3349,
    /// J-1939-CLUSTER-CONDITIONAL
    J1939ClusterConditional                                                = 3072,
    /// J-1939-CLUSTER-VARIANTS
    J1939ClusterVariants                                                   = 1577,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 5416,
    /// J-1939-CONTROLLER-APPLICATION-REF
    J1939ControllerApplicationRef                                          = 1513,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING
    J1939ControllerApplicationToJ1939NmNodeMapping                         = 5224,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS
    J1939ControllerApplicationToJ1939NmNodeMappings                        = 864,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 2552,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 230,
    /// J-1939-DTC-VALUE
    J1939DtcValue                                                          = 1827,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 6063,
    /// J-1939-NM-ECU
    J1939NmEcu                                                             = 4023,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 566,
    /// J-1939-NM-NODE-REF
    J1939NmNodeRef                                                         = 4243,
    /// J-1939-REQUESTABLE
    J1939Requestable                                                       = 1580,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 2880,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 4848,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 5283,
    /// J-1939-SHARED-ADDRESS-CLUSTERS
    J1939SharedAddressClusters                                             = 5148,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 2513,
    /// J-1939-TP-CONNECTION
    J1939TpConnection                                                      = 25,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 593,
    /// J-1939-TP-PG
    J1939TpPg                                                              = 5167,
    /// JITTER
    Jitter                                                                 = 2673,
    /// JOB-PRIMITIVE-REF
    JobPrimitiveRef                                                        = 333,
    /// JOB-REF
    JobRef                                                                 = 2249,
    /// JOB-REQUIREMENTS
    JobRequirements                                                        = 4467,
    /// JUMP-TO-BOOT-LOADER
    JumpToBootLoader                                                       = 1003,
    /// K-INIT-RESOURCE
    KInitResource                                                          = 6068,
    /// K-RUN-RESOURCE
    KRunResource                                                           = 1105,
    /// KEEP-ALIVE-INTERVAL
    KeepAliveInterval                                                      = 5464,
    /// KEEP-ALIVE-PROBES-MAX
    KeepAliveProbesMax                                                     = 2425,
    /// KEEP-ALIVE-TIME
    KeepAliveTime                                                          = 4115,
    /// KEEP-ALIVES
    KeepAlives                                                             = 5465,
    /// KEY
    Key                                                                    = 6035,
    /// KEY-EXCHANGE-AUTHENTICATION-REF
    KeyExchangeAuthenticationRef                                           = 4898,
    /// KEY-EXCHANGE-AUTHENTICATION-REFS
    KeyExchangeAuthenticationRefs                                          = 4130,
    /// KEY-EXCHANGE-REF
    KeyExchangeRef                                                         = 753,
    /// KEY-EXCHANGE-REFS
    KeyExchangeRefs                                                        = 5043,
    /// KEY-GENERATION
    KeyGeneration                                                          = 5521,
    /// KEY-ID
    KeyId                                                                  = 2735,
    /// KEY-IDENTITY-REF
    KeyIdentityRef                                                         = 6113,
    /// KEY-REF
    KeyRef                                                                 = 410,
    /// KEY-SERVER-PRIORITY
    KeyServerPriority                                                      = 4092,
    /// KEY-SIZE
    KeySize                                                                = 2825,
    /// KEY-SLOT-ALLOWED-MODIFICATION
    KeySlotAllowedModification                                             = 5692,
    /// KEY-SLOT-CONTENT-ALLOWED-USAGES
    KeySlotContentAllowedUsages                                            = 2541,
    /// KEY-SLOT-ID
    KeySlotId                                                              = 5476,
    /// KEY-SLOT-ONLY-ENABLED
    KeySlotOnlyEnabled                                                     = 1893,
    /// KEY-SLOT-REF
    KeySlotRef                                                             = 3408,
    /// KEY-SLOT-USAGE
    KeySlotUsage                                                           = 1046,
    /// KEY-SLOT-USED-FOR-START-UP
    KeySlotUsedForStartUp                                                  = 4350,
    /// KEY-SLOT-USED-FOR-SYNC
    KeySlotUsedForSync                                                     = 408,
    /// KEY-SLOTS
    KeySlots                                                               = 2340,
    /// KEY-STORAGE-TYPE
    KeyStorageType                                                         = 1178,
    /// KEY-VALUE-PAIRS
    KeyValuePairs                                                          = 861,
    /// KEY-VALUE-STORAGE-REF
    KeyValueStorageRef                                                     = 831,
    /// KEYWORD
    Keyword                                                                = 3457,
    /// KEYWORD-SET
    KeywordSet                                                             = 2352,
    /// KEYWORDS
    Keywords                                                               = 3934,
    /// KIND
    Kind                                                                   = 1422,
    /// L-1
    L1                                                                     = 1537,
    /// L-10
    L10                                                                    = 3666,
    /// L-2
    L2                                                                     = 1469,
    /// L-4
    L4                                                                     = 5623,
    /// L-5
    L5                                                                     = 4933,
    /// L-GRAPHIC
    LGraphic                                                               = 3886,
    /// LABEL
    Label                                                                  = 2305,
    /// LABEL-1
    Label1                                                                 = 640,
    /// LABELED-ITEM
    LabeledItem                                                            = 4035,
    /// LABELED-LIST
    LabeledList                                                            = 1393,
    /// LANGUAGE
    Language                                                               = 5081,
    /// LAST-EGRESS-SCHEDULER-REF
    LastEgressSchedulerRef                                                 = 3860,
    /// LATENCY-BUDGET
    LatencyBudget                                                          = 2402,
    /// LATENCY-BUDGET-DURATION
    LatencyBudgetDuration                                                  = 637,
    /// LATENCY-CONSTRAINT-TYPE
    LatencyConstraintType                                                  = 5058,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 4385,
    /// LATEST-TX
    LatestTx                                                               = 2491,
    /// LC-OBJECT-REF
    LcObjectRef                                                            = 4822,
    /// LC-STATE-REF
    LcStateRef                                                             = 3543,
    /// LC-STATES
    LcStates                                                               = 1907,
    /// LDAP-URL
    LdapUrl                                                                = 139,
    /// LEAF-ELEMENT-IREF
    LeafElementIref                                                        = 3694,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS
    LegislatedFreezeFrameContentUdsObds                                    = 3844,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS
    LegislatedFreezeFrameContentWwhObds                                    = 3392,
    /// LENGTH
    Length                                                                 = 852,
    /// LENGTH-CLIENT-ID
    LengthClientId                                                         = 516,
    /// LENGTH-EXP
    LengthExp                                                              = 4443,
    /// LENGTH-SEQUENCE-COUNTER
    LengthSequenceCounter                                                  = 632,
    /// LENGTH-VERIFICATION
    LengthVerification                                                     = 3570,
    /// LET-DATA-EXCHANGE-PARADIGM
    LetDataExchangeParadigm                                                = 2104,
    /// LET-INTERVAL-REF
    LetIntervalRef                                                         = 409,
    /// LET-INTERVAL-REFS
    LetIntervalRefs                                                        = 5394,
    /// LICENSE-REF
    LicenseRef                                                             = 1701,
    /// LICENSE-REFS
    LicenseRefs                                                            = 1776,
    /// LIFE-CYCLE-INFO
    LifeCycleInfo                                                          = 6261,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 2004,
    /// LIFE-CYCLE-INFOS
    LifeCycleInfos                                                         = 164,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 1757,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 6032,
    /// LIFESPAN
    Lifespan                                                               = 5162,
    /// LIFESPAN-DURATION
    LifespanDuration                                                       = 3196,
    /// LIMIT
    Limit                                                                  = 5762,
    /// LIMIT-KIND
    LimitKind                                                              = 4340,
    /// LIN-CHECKSUM
    LinChecksum                                                            = 2632,
    /// LIN-CLUSTER
    LinCluster                                                             = 2204,
    /// LIN-CLUSTER-CONDITIONAL
    LinClusterConditional                                                  = 5727,
    /// LIN-CLUSTER-VARIANTS
    LinClusterVariants                                                     = 2118,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 2991,
    /// LIN-CONFIGURABLE-FRAME
    LinConfigurableFrame                                                   = 5339,
    /// LIN-CONFIGURABLE-FRAMES
    LinConfigurableFrames                                                  = 5735,
    /// LIN-ERROR-RESPONSE
    LinErrorResponse                                                       = 2822,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 3665,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 1286,
    /// LIN-IDENTIFIER
    LinIdentifier                                                          = 2835,
    /// LIN-MASTER
    LinMaster                                                              = 5093,
    /// LIN-MASTER-CONDITIONAL
    LinMasterConditional                                                   = 3764,
    /// LIN-MASTER-VARIANTS
    LinMasterVariants                                                      = 267,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 779,
    /// LIN-ORDERED-CONFIGURABLE-FRAME
    LinOrderedConfigurableFrame                                            = 5854,
    /// LIN-ORDERED-CONFIGURABLE-FRAMES
    LinOrderedConfigurableFrames                                           = 4259,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 2977,
    /// LIN-PID-TO-CAN-ID-MAPPINGS
    LinPidToCanIdMappings                                                  = 4481,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 162,
    /// LIN-SLAVE
    LinSlave                                                               = 2060,
    /// LIN-SLAVE-CONDITIONAL
    LinSlaveConditional                                                    = 4483,
    /// LIN-SLAVE-CONFIG
    LinSlaveConfig                                                         = 3176,
    /// LIN-SLAVE-ECU-REF
    LinSlaveEcuRef                                                         = 2802,
    /// LIN-SLAVE-VARIANTS
    LinSlaveVariants                                                       = 5406,
    /// LIN-SLAVES
    LinSlaves                                                              = 1198,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 2469,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 1658,
    /// LIN-TP-CONNECTION
    LinTpConnection                                                        = 465,
    /// LIN-TP-N-SDU-REF
    LinTpNSduRef                                                           = 1570,
    /// LIN-TP-NODE
    LinTpNode                                                              = 1585,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 773,
    /// LIN-UNCONDITIONAL-FRAME-REF
    LinUnconditionalFrameRef                                               = 2448,
    /// LIN-UNCONDITIONAL-FRAME-REFS
    LinUnconditionalFrameRefs                                              = 5184,
    /// LINKER
    Linker                                                                 = 1304,
    /// LINKERS
    Linkers                                                                = 1179,
    /// LIST
    List                                                                   = 2237,
    /// LISTEN-NOISE
    ListenNoise                                                            = 2617,
    /// LISTEN-TIMEOUT
    ListenTimeout                                                          = 1912,
    /// LITERAL-PREFIX
    LiteralPrefix                                                          = 3526,
    /// LITERALS
    Literals                                                               = 4189,
    /// LIVELINESS
    Liveliness                                                             = 4201,
    /// LIVELINESS-LEASE-DURATION
    LivelinessLeaseDuration                                                = 4560,
    /// LIVELINESS-PROTECTION-KIND
    LivelinessProtectionKind                                               = 4728,
    /// LIVENESS-KIND
    LivenessKind                                                           = 2867,
    /// LOAD-BALANCING-PRIORITY
    LoadBalancingPriority                                                  = 5535,
    /// LOAD-BALANCING-WEIGHT
    LoadBalancingWeight                                                    = 4506,
    /// LOC-MEASUREMENT-SET
    LocMeasurementSet                                                      = 1897,
    /// LOCAL-CERTIFICATE-REF
    LocalCertificateRef                                                    = 4592,
    /// LOCAL-CERTIFICATE-REFS
    LocalCertificateRefs                                                   = 1579,
    /// LOCAL-COM-ACCESS-CONTROL-ENABLED
    LocalComAccessControlEnabled                                           = 5051,
    /// LOCAL-COMM-CONNECTOR-REF
    LocalCommConnectorRef                                                  = 5808,
    /// LOCAL-DEBUG-DATAS
    LocalDebugDatas                                                        = 356,
    /// LOCAL-ENDPOINT-CONFIG
    LocalEndpointConfig                                                    = 185,
    /// LOCAL-ID
    LocalId                                                                = 2851,
    /// LOCAL-IP-SEC-RULE-REF
    LocalIpSecRuleRef                                                      = 3079,
    /// LOCAL-IP-SEC-RULE-REFS
    LocalIpSecRuleRefs                                                     = 5386,
    /// LOCAL-NETWORK-ENDPOINT-REF
    LocalNetworkEndpointRef                                                = 1844,
    /// LOCAL-PARAMETER-REF
    LocalParameterRef                                                      = 6081,
    /// LOCAL-PORT-RANGE-END
    LocalPortRangeEnd                                                      = 2279,
    /// LOCAL-PORT-RANGE-START
    LocalPortRangeStart                                                    = 2158,
    /// LOCAL-PORT-REF
    LocalPortRef                                                           = 4705,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 2903,
    /// LOCAL-SUPERVISION-ENTITY-REF
    LocalSupervisionEntityRef                                              = 1704,
    /// LOCAL-SUPERVISION-ENTITY-REFS
    LocalSupervisionEntityRefs                                             = 5350,
    /// LOCAL-SUPERVISION-REF
    LocalSupervisionRef                                                    = 1316,
    /// LOCAL-SUPERVISION-REFS
    LocalSupervisionRefs                                                   = 644,
    /// LOCAL-SUPERVISIONS
    LocalSupervisions                                                      = 2178,
    /// LOCAL-TCP-PORT-REF
    LocalTcpPortRef                                                        = 3098,
    /// LOCAL-UDP-PORT-REF
    LocalUdpPortRef                                                        = 5674,
    /// LOCAL-UNICAST-ADDRESSES
    LocalUnicastAddresses                                                  = 2563,
    /// LOCAL-UNICAST-ADDRESSS
    LocalUnicastAddresss                                                   = 695,
    /// LOCAL-VARIABLE-REF
    LocalVariableRef                                                       = 784,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 2796,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 3647,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 3702,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF
    LogAndTraceMessageCollectionSetRef                                     = 3863,
    /// LOG-CHANNEL-ID
    LogChannelId                                                           = 4833,
    /// LOG-SINK-REF
    LogSinkRef                                                             = 2059,
    /// LOG-SINK-REFS
    LogSinkRefs                                                            = 923,
    /// LOG-TRACE-DEFAULT-LOG-LEVEL
    LogTraceDefaultLogLevel                                                = 5923,
    /// LOG-TRACE-DEFAULT-LOG-THRESHOLD
    LogTraceDefaultLogThreshold                                            = 2475,
    /// LOG-TRACE-FILE-PATH
    LogTraceFilePath                                                       = 172,
    /// LOG-TRACE-LOG-MODE
    LogTraceLogMode                                                        = 2468,
    /// LOG-TRACE-LOG-MODES
    LogTraceLogModes                                                       = 4360,
    /// LOG-TRACE-PORT-ID
    LogTracePortId                                                         = 4808,
    /// LOG-TRACE-PROCESS-DESC
    LogTraceProcessDesc                                                    = 1550,
    /// LOG-TRACE-PROCESS-ID
    LogTraceProcessId                                                      = 5925,
    /// LOGGING-BEHAVIOR
    LoggingBehavior                                                        = 4913,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 4603,
    /// LOGIC-ADDRESSS
    LogicAddresss                                                          = 554,
    /// LOGICAL-ADDRESS
    LogicalAddress                                                         = 3487,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 2524,
    /// LOGICAL-EXPRESSION-ARGUMENT-REF
    LogicalExpressionArgumentRef                                           = 4192,
    /// LOGICAL-EXPRESSION-ARGUMENT-REFS
    LogicalExpressionArgumentRefs                                          = 3589,
    /// LOGICAL-EXPRESSIONS
    LogicalExpressions                                                     = 2920,
    /// LOGICAL-OPERATOR
    LogicalOperator                                                        = 65,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 3638,
    /// LOGICAL-SUPERVISIONS
    LogicalSupervisions                                                    = 4124,
    /// LONG-NAME
    LongName                                                               = 3545,
    /// LONG-NAME-1
    LongName1                                                              = 5696,
    /// LOWER-BOUND
    LowerBound                                                             = 5125,
    /// LOWER-BOUNDARY
    LowerBoundary                                                          = 4167,
    /// LOWER-CAN-ID
    LowerCanId                                                             = 3182,
    /// LOWER-LIMIT
    LowerLimit                                                             = 6282,
    /// LOWER-MULTIPLICITY
    LowerMultiplicity                                                      = 5031,
    /// LOWER-REF
    LowerRef                                                               = 54,
    /// LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING
    LtMessageCollectionToPortPrototypeMapping                              = 2251,
    /// LUMINOUS-INTENSITY-EXP
    LuminousIntensityExp                                                   = 2077,
    /// M
    M                                                                      = 4463,
    /// MAC-ADDRESS
    MacAddress                                                             = 3321,
    /// MAC-ADDRESS-MASK
    MacAddressMask                                                         = 4069,
    /// MAC-ADDRESS-STREAM-ID
    MacAddressStreamId                                                     = 5474,
    /// MAC-LAYER-TYPE
    MacLayerType                                                           = 4779,
    /// MAC-MULTICAST-ADDRESS
    MacMulticastAddress                                                    = 498,
    /// MAC-MULTICAST-ADDRESS-REF
    MacMulticastAddressRef                                                 = 1626,
    /// MAC-MULTICAST-ADDRESS-REFS
    MacMulticastAddressRefs                                                = 5632,
    /// MAC-MULTICAST-CONFIGURATION
    MacMulticastConfiguration                                              = 3539,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 3830,
    /// MAC-MULTICAST-GROUP-REF
    MacMulticastGroupRef                                                   = 1150,
    /// MAC-MULTICAST-GROUPS
    MacMulticastGroups                                                     = 3295,
    /// MAC-SEC-CIPHER-SUITE-CONFIG
    MacSecCipherSuiteConfig                                                = 1696,
    /// MAC-SEC-GLOBAL-KAY-PROPS
    MacSecGlobalKayProps                                                   = 1128,
    /// MAC-SEC-KAY-CONFIG
    MacSecKayConfig                                                        = 1076,
    /// MAC-SEC-KAY-PARTICIPANT
    MacSecKayParticipant                                                   = 966,
    /// MAC-SEC-PARTICIPANT-SET
    MacSecParticipantSet                                                   = 908,
    /// MAC-SEC-PROPS
    MacSecProps                                                            = 1716,
    /// MAC-SEC-PROPSS
    MacSecPropss                                                           = 3628,
    /// MAC-UNICAST-ADDRESS
    MacUnicastAddress                                                      = 1283,
    /// MACHINE
    Machine                                                                = 1293,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 1372,
    /// MACHINE-DESIGN-REF
    MachineDesignRef                                                       = 6061,
    /// MACHINE-ID
    MachineId                                                              = 4830,
    /// MACHINE-MODE-IREF
    MachineModeIref                                                        = 2175,
    /// MACHINE-MODE-IREFS
    MachineModeIrefs                                                       = 979,
    /// MACHINE-MODE-MACHINES
    MachineModeMachines                                                    = 269,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 3689,
    /// MACHINE-REF
    MachineRef                                                             = 3713,
    /// MACHINE-TIMING
    MachineTiming                                                          = 2873,
    /// MACRO-INITIAL-OFFSET-A
    MacroInitialOffsetA                                                    = 3089,
    /// MACRO-INITIAL-OFFSET-B
    MacroInitialOffsetB                                                    = 2692,
    /// MACRO-PER-CYCLE
    MacroPerCycle                                                          = 4078,
    /// MACROTICK-DURATION
    MacrotickDuration                                                      = 4756,
    /// MAIN-FUNCTION-PERIOD
    MainFunctionPeriod                                                     = 5142,
    /// MAJOR-PROTOCOL-VERSION
    MajorProtocolVersion                                                   = 4287,
    /// MAJOR-VERSION
    MajorVersion                                                           = 5554,
    /// MANAGED-COUPLING-PORTS
    ManagedCouplingPorts                                                   = 300,
    /// MANAGED-MODE-GROUPS
    ManagedModeGroups                                                      = 5583,
    /// MANAGED-PHYSICAL-CHANNEL-REF
    ManagedPhysicalChannelRef                                              = 655,
    /// MANAGED-PHYSICAL-CHANNEL-REFS
    ManagedPhysicalChannelRefs                                             = 475,
    /// MANDATORY
    Mandatory                                                              = 1350,
    /// MANUFACTURER-CODE
    ManufacturerCode                                                       = 1090,
    /// MANUFACTURER-VALIDATION-ORDER-REF
    ManufacturerValidationOrderRef                                         = 2876,
    /// MANUFACTURER-VALIDATION-ORDER-REFS
    ManufacturerValidationOrderRefs                                        = 2025,
    /// MAP
    Map                                                                    = 56,
    /// MAP-ELEMENT-TUPLES
    MapElementTuples                                                       = 81,
    /// MAPPED-AP-DATA-ELEMENT-IREF
    MappedApDataElementIref                                                = 843,
    /// MAPPED-BSW-SERVICE-DEPENDENCY-REF
    MappedBswServiceDependencyRef                                          = 3869,
    /// MAPPED-DATA-ELEMENT-IREF
    MappedDataElementIref                                                  = 5454,
    /// MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF
    MappedFlatSwcServiceDependencyRef                                      = 4379,
    /// MAPPED-FUNCTION-REF
    MappedFunctionRef                                                      = 2678,
    /// MAPPED-OPERATION-IREF
    MappedOperationIref                                                    = 1576,
    /// MAPPED-SECURITY-EVENTS
    MappedSecurityEvents                                                   = 253,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    MappedSwcServiceDependencyInExecutableIref                             = 3230,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    MappedSwcServiceDependencyInSystemIref                                 = 3977,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IREF
    MappedSwcServiceDependencyIref                                         = 3684,
    /// MAPPING-CONSTRAINTS
    MappingConstraints                                                     = 1966,
    /// MAPPING-DIRECTION
    MappingDirection                                                       = 2860,
    /// MAPPING-REF
    MappingRef                                                             = 3765,
    /// MAPPING-SCOPE
    MappingScope                                                           = 427,
    /// MAPPINGS
    Mappings                                                               = 5901,
    /// MASK
    Mask                                                                   = 1629,
    /// MASS-EXP
    MassExp                                                                = 5977,
    /// MASTER
    Master                                                                 = 1107,
    /// MASTER-EVENT-REF
    MasterEventRef                                                         = 4823,
    /// MASTER-REF
    MasterRef                                                              = 1975,
    /// MASTER-TO-SLAVE-EVENT-MAPPINGS
    MasterToSlaveEventMappings                                             = 2724,
    /// MASTERED-TRIGGER-REF
    MasteredTriggerRef                                                     = 1586,
    /// MATCHING-CRITERION-REF
    MatchingCriterionRef                                                   = 228,
    /// MATCHING-EGRESS-RULE-REF
    MatchingEgressRuleRef                                                  = 2659,
    /// MATCHING-EGRESS-RULE-REFS
    MatchingEgressRuleRefs                                                 = 2497,
    /// MATCHING-INGRESS-RULE-REF
    MatchingIngressRuleRef                                                 = 5399,
    /// MATCHING-INGRESS-RULE-REFS
    MatchingIngressRuleRefs                                                = 122,
    /// MATCHING-RULE-REF
    MatchingRuleRef                                                        = 2214,
    /// MATCHING-RULE-REFS
    MatchingRuleRefs                                                       = 2638,
    /// MAX
    Max                                                                    = 602,
    /// MAX-ALIVE-CYCLE
    MaxAliveCycle                                                          = 5791,
    /// MAX-AR
    MaxAr                                                                  = 4589,
    /// MAX-AS
    MaxAs                                                                  = 584,
    /// MAX-AVAILABLE-PERSISTENCY-STORAGE-SPACE
    MaxAvailablePersistencyStorageSpace                                    = 956,
    /// MAX-BASE-TYPE-SIZE
    MaxBaseTypeSize                                                        = 3160,
    /// MAX-BLOCK-SIZE
    MaxBlockSize                                                           = 4918,
    /// MAX-BS
    MaxBs                                                                  = 2193,
    /// MAX-BUFFER-REQUEST
    MaxBufferRequest                                                       = 486,
    /// MAX-BUFFER-SIZE
    MaxBufferSize                                                          = 6170,
    /// MAX-BYTES-IN-INTERVAL
    MaxBytesInInterval                                                     = 3577,
    /// MAX-COMM-MODE
    MaxCommMode                                                            = 2688,
    /// MAX-CYCLE-REPETITIONS
    MaxCycleRepetitions                                                    = 6062,
    /// MAX-CYCLES
    MaxCycles                                                              = 3195,
    /// MAX-DATA-LENGTH
    MaxDataLength                                                          = 2700,
    /// MAX-DEADLINE
    MaxDeadline                                                            = 50,
    /// MAX-DELTA-COUNTER
    MaxDeltaCounter                                                        = 4610,
    /// MAX-DELTA-COUNTER-INIT
    MaxDeltaCounterInit                                                    = 4238,
    /// MAX-DESTINATION-PORT-NUMBER
    MaxDestinationPortNumber                                               = 3327,
    /// MAX-DID-TO-READ
    MaxDidToRead                                                           = 2287,
    /// MAX-DIFF
    MaxDiff                                                                = 2893,
    /// MAX-ERROR-STATE-INIT
    MaxErrorStateInit                                                      = 4606,
    /// MAX-ERROR-STATE-INVALID
    MaxErrorStateInvalid                                                   = 1938,
    /// MAX-ERROR-STATE-VALID
    MaxErrorStateValid                                                     = 3650,
    /// MAX-EVENTS-IN-INTERVAL
    MaxEventsInInterval                                                    = 2236,
    /// MAX-EXP-BS
    MaxExpBs                                                               = 1233,
    /// MAX-FC-WAIT
    MaxFcWait                                                              = 5627,
    /// MAX-FR-IF
    MaxFrIf                                                                = 2027,
    /// MAX-GRADIENT
    MaxGradient                                                            = 2227,
    /// MAX-HEADER-ID
    MaxHeaderId                                                            = 716,
    /// MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    MaxInitialVehicleAnnouncementTime                                      = 2707,
    /// MAX-INSTANCES
    MaxInstances                                                           = 5408,
    /// MAX-LENGTH
    MaxLength                                                              = 5145,
    /// MAX-MAJOR-VERSION
    MaxMajorVersion                                                        = 1914,
    /// MAX-MARGIN
    MaxMargin                                                              = 1020,
    /// MAX-MINOR-VERSION
    MaxMinorVersion                                                        = 2205,
    /// MAX-NO-NEW-OR-REPEATED-DATA
    MaxNoNewOrRepeatedData                                                 = 5443,
    /// MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS
    MaxNumChangeOfDataIdentfierEvents                                      = 238,
    /// MAX-NUM-COMPARISION-OF-VALUE-EVENTS
    MaxNumComparisionOfValueEvents                                         = 1190,
    /// MAX-NUMBER-FREEZE-FRAME-RECORDS
    MaxNumberFreezeFrameRecords                                            = 3548,
    /// MAX-NUMBER-OF-ALLOWED-UPDATES
    MaxNumberOfAllowedUpdates                                              = 454,
    /// MAX-NUMBER-OF-CONNECTIONS
    MaxNumberOfConnections                                                 = 1498,
    /// MAX-NUMBER-OF-ELEMENTS
    MaxNumberOfElements                                                    = 5574,
    /// MAX-NUMBER-OF-EVENT-ENTRIES
    MaxNumberOfEventEntries                                                = 4194,
    /// MAX-NUMBER-OF-FILES
    MaxNumberOfFiles                                                       = 2945,
    /// MAX-NUMBER-OF-NPDU-PER-CYCLE
    MaxNumberOfNpduPerCycle                                                = 2397,
    /// MAX-NUMBER-OF-OCCURRENCES
    MaxNumberOfOccurrences                                                 = 2531,
    /// MAX-NUMBER-OF-PARALLEL-TRANSFERS
    MaxNumberOfParallelTransfers                                           = 5822,
    /// MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING
    MaxNumberOfRequestCorrectlyReceivedResponsePending                     = 1488,
    /// MAX-NUMBER-OF-RESP-PENDING-FRAMES
    MaxNumberOfRespPendingFrames                                           = 833,
    /// MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS
    MaxNumberOfStoredDtcStatusChangedEvents                                = 1622,
    /// MAX-NUMBER-OF-TIME-QUANTA-PER-BIT
    MaxNumberOfTimeQuantaPerBit                                            = 1568,
    /// MAX-PERIODIC-DID-TO-READ
    MaxPeriodicDidToRead                                                   = 1151,
    /// MAX-PWM-L
    MaxPwmL                                                                = 4717,
    /// MAX-PWM-O
    MaxPwmO                                                                = 3216,
    /// MAX-PWM-S
    MaxPwmS                                                                = 2508,
    /// MAX-REQUEST-BYTES
    MaxRequestBytes                                                        = 3033,
    /// MAX-RETRIES
    MaxRetries                                                             = 1923,
    /// MAX-RPT-EVENT-ID
    MaxRptEventId                                                          = 6072,
    /// MAX-RUN-CYCLE-LOOP
    MaxRunCycleLoop                                                        = 4819,
    /// MAX-SAMPLE-POINT
    MaxSamplePoint                                                         = 2127,
    /// MAX-SAMPLES
    MaxSamples                                                             = 1544,
    /// MAX-SAMPLES-PER-INSTANCE
    MaxSamplesPerInstance                                                  = 3034,
    /// MAX-SDU-SIZE
    MaxSduSize                                                             = 6094,
    /// MAX-SERVICE-POINT-ID
    MaxServicePointId                                                      = 3673,
    /// MAX-SIZE-TO-FILL
    MaxSizeToFill                                                          = 2209,
    /// MAX-SLOTS
    MaxSlots                                                               = 2682,
    /// MAX-SLOTS-PER-CYCLE
    MaxSlotsPerCycle                                                       = 2135,
    /// MAX-SOURCE-ELEMENT
    MaxSourceElement                                                       = 3482,
    /// MAX-SOURCE-PORT-NUMBER
    MaxSourcePortNumber                                                    = 5147,
    /// MAX-SUPPORTED-DID-LENGTH
    MaxSupportedDidLength                                                  = 5913,
    /// MAX-SYNC-JUMP-WIDTH
    MaxSyncJumpWidth                                                       = 3201,
    /// MAX-TESTER-CONNECTIONS
    MaxTesterConnections                                                   = 800,
    /// MAX-TRANSIT-TIME
    MaxTransitTime                                                         = 3663,
    /// MAX-TRCV-DELAY-COMPENSATION-OFFSET
    MaxTrcvDelayCompensationOffset                                         = 5143,
    /// MAX-VALUE
    MaxValue                                                               = 6114,
    /// MAX-WITHOUT-CLOCK-CORRECTION-FATAL
    MaxWithoutClockCorrectionFatal                                         = 5688,
    /// MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE
    MaxWithoutClockCorrectionPassive                                       = 4866,
    /// MAXIMUM
    Maximum                                                                = 1500,
    /// MAXIMUM-ALLOWED-SIZE
    MaximumAllowedSize                                                     = 2904,
    /// MAXIMUM-DISTANCES
    MaximumDistances                                                       = 3626,
    /// MAXIMUM-DURATION-OF-CAMPAIGN
    MaximumDurationOfCampaign                                              = 5182,
    /// MAXIMUM-DYNAMIC-PAYLOAD-LENGTH
    MaximumDynamicPayloadLength                                            = 1331,
    /// MAXIMUM-EXECUTION-TIME
    MaximumExecutionTime                                                   = 1380,
    /// MAXIMUM-INTENDED-BINDING-TIME
    MaximumIntendedBindingTime                                             = 2130,
    /// MAXIMUM-INTER-ARRIVAL-TIME
    MaximumInterArrivalTime                                                = 51,
    /// MAXIMUM-KEY-LENGTH
    MaximumKeyLength                                                       = 4807,
    /// MAXIMUM-LENGTH
    MaximumLength                                                          = 5896,
    /// MAXIMUM-MEMORY-CONSUMPTION
    MaximumMemoryConsumption                                               = 5077,
    /// MAXIMUM-MESSAGE-LENGTH
    MaximumMessageLength                                                   = 5838,
    /// MAXIMUM-NUMBER-OF-RETRIES
    MaximumNumberOfRetries                                                 = 1535,
    /// MAXIMUM-RECEIVE-BUFFER-LENGTH
    MaximumReceiveBufferLength                                             = 4868,
    /// MAXIMUM-RESIDENCE-TIME
    MaximumResidenceTime                                                   = 788,
    /// MAXIMUM-SEGMENT-LENGTH
    MaximumSegmentLength                                                   = 240,
    /// MAXIMUM-SEGMENT-LENGTH-REQUEST
    MaximumSegmentLengthRequest                                            = 1690,
    /// MAXIMUM-SEGMENT-LENGTH-RESPONSE
    MaximumSegmentLengthResponse                                           = 5974,
    /// MAXIMUM-SELECTED-BINDING-TIME
    MaximumSelectedBindingTime                                             = 5466,
    /// MAXIMUM-SUPPORTED-UCM-MASTER-VERSION
    MaximumSupportedUcmMasterVersion                                       = 3585,
    /// MAXIMUM-SUPPORTED-UCM-VERSION
    MaximumSupportedUcmVersion                                             = 1810,
    /// MAXIMUM-TRANSMISSION-UNIT
    MaximumTransmissionUnit                                                = 5801,
    /// MAXIMUM-TRANSMIT-BUFFER-LENGTH
    MaximumTransmitBufferLength                                            = 1634,
    /// MAY-BE-UNCONNECTED
    MayBeUnconnected                                                       = 4058,
    /// MC-DATA-ACCESS-DETAILS
    McDataAccessDetails                                                    = 2015,
    /// MC-DATA-ASSIGNMENTS
    McDataAssignments                                                      = 5461,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 1721,
    /// MC-DATA-INSTANCE-REF
    McDataInstanceRef                                                      = 4566,
    /// MC-DATA-INSTANCE-REFS
    McDataInstanceRefs                                                     = 623,
    /// MC-DATA-INSTANCE-VAR-REF
    McDataInstanceVarRef                                                   = 5657,
    /// MC-FUNCTION
    McFunction                                                             = 6184,
    /// MC-FUNCTION-DATA-REF-SET-CONDITIONAL
    McFunctionDataRefSetConditional                                        = 4317,
    /// MC-FUNCTION-DATA-REF-SET-VARIANTS
    McFunctionDataRefSetVariants                                           = 5247,
    /// MC-FUNCTION-REF
    McFunctionRef                                                          = 2694,
    /// MC-FUNCTION-REFS
    McFunctionRefs                                                         = 2532,
    /// MC-GROUP
    McGroup                                                                = 3561,
    /// MC-GROUP-DATA-REF-SET-CONDITIONAL
    McGroupDataRefSetConditional                                           = 2752,
    /// MC-GROUP-DATA-REF-SET-VARIANTS
    McGroupDataRefSetVariants                                              = 5508,
    /// MC-PARAMETER-ELEMENT-GROUP
    McParameterElementGroup                                                = 1638,
    /// MC-PARAMETER-INSTANCES
    McParameterInstances                                                   = 2613,
    /// MC-SUPPORT
    McSupport                                                              = 1251,
    /// MC-SW-EMULATION-METHOD-SUPPORT
    McSwEmulationMethodSupport                                             = 1040,
    /// MC-VARIABLE-INSTANCES
    McVariableInstances                                                    = 4326,
    /// MCD-IDENTIFIER
    McdIdentifier                                                          = 1901,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REF
    MeasurableSystemConstantValuesRef                                      = 457,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS
    MeasurableSystemConstantValuesRefs                                     = 3912,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 415,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 6163,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 5432,
    /// MEM-ALIGNMENT
    MemAlignment                                                           = 4620,
    /// MEM-CLASS-SYMBOL
    MemClassSymbol                                                         = 5640,
    /// MEM-USAGE
    MemUsage                                                               = 4180,
    /// MEMORY-ALLOCATION-KEYWORD-POLICY
    MemoryAllocationKeywordPolicy                                          = 2139,
    /// MEMORY-CONSUMPTION
    MemoryConsumption                                                      = 2968,
    /// MEMORY-DESTINATION-REF
    MemoryDestinationRef                                                   = 4761,
    /// MEMORY-DESTINATION-REFS
    MemoryDestinationRefs                                                  = 1808,
    /// MEMORY-ENTRY-STORAGE-TRIGGER
    MemoryEntryStorageTrigger                                              = 3210,
    /// MEMORY-HIGH-ADDRESS
    MemoryHighAddress                                                      = 1006,
    /// MEMORY-HIGH-ADDRESS-LABEL
    MemoryHighAddressLabel                                                 = 6082,
    /// MEMORY-ID
    MemoryId                                                               = 1915,
    /// MEMORY-LOW-ADDRESS
    MemoryLowAddress                                                       = 6221,
    /// MEMORY-LOW-ADDRESS-LABEL
    MemoryLowAddressLabel                                                  = 523,
    /// MEMORY-RANGE-REF
    MemoryRangeRef                                                         = 794,
    /// MEMORY-RANGE-REFS
    MemoryRangeRefs                                                        = 1815,
    /// MEMORY-SECTION
    MemorySection                                                          = 4570,
    /// MEMORY-SECTION-LOCATION
    MemorySectionLocation                                                  = 4780,
    /// MEMORY-SECTION-LOCATIONS
    MemorySectionLocations                                                 = 6246,
    /// MEMORY-SECTIONS
    MemorySections                                                         = 5954,
    /// MEMORY-USAGE
    MemoryUsage                                                            = 777,
    /// MEMORY-USAGES
    MemoryUsages                                                           = 4310,
    /// MESSAGE-COMPLIANCE
    MessageCompliance                                                      = 208,
    /// MESSAGE-ID
    MessageId                                                              = 4082,
    /// MESSAGE-LINE-NUMBER
    MessageLineNumber                                                      = 270,
    /// MESSAGE-LINK-LENGTH
    MessageLinkLength                                                      = 5516,
    /// MESSAGE-LINK-POSITION
    MessageLinkPosition                                                    = 5384,
    /// MESSAGE-SOURCE-FILE
    MessageSourceFile                                                      = 5411,
    /// MESSAGE-TYPE
    MessageType                                                            = 163,
    /// MESSAGE-TYPE-INFO
    MessageTypeInfo                                                        = 31,
    /// META-DATA-FIELDS
    MetaDataFields                                                         = 4288,
    /// META-DATA-ITEM
    MetaDataItem                                                           = 5809,
    /// META-DATA-ITEM-SET
    MetaDataItemSet                                                        = 5962,
    /// META-DATA-ITEM-SETS
    MetaDataItemSets                                                       = 3867,
    /// META-DATA-ITEM-TYPE
    MetaDataItemType                                                       = 3504,
    /// META-DATA-ITEMS
    MetaDataItems                                                          = 3708,
    /// META-DATA-LENGTH
    MetaDataLength                                                         = 764,
    /// METADATA-PROTECTION-KIND
    MetadataProtectionKind                                                 = 884,
    /// METHOD-ACTIVATION-ROUTING-GROUPS
    MethodActivationRoutingGroups                                          = 546,
    /// METHOD-CALL-REF
    MethodCallRef                                                          = 2618,
    /// METHOD-CALL-REFS
    MethodCallRefs                                                         = 1932,
    /// METHOD-DEPLOYMENTS
    MethodDeployments                                                      = 2931,
    /// METHOD-ID
    MethodId                                                               = 1851,
    /// METHOD-IREF
    MethodIref                                                             = 3136,
    /// METHOD-MAPPING
    MethodMapping                                                          = 5275,
    /// METHOD-MAPPINGS
    MethodMappings                                                         = 3385,
    /// METHOD-QOS-PROPSS
    MethodQosPropss                                                        = 4805,
    /// METHOD-REF
    MethodRef                                                              = 959,
    /// METHOD-REFS
    MethodRefs                                                             = 1813,
    /// METHOD-REPLY-TOPIC-NAME
    MethodReplyTopicName                                                   = 1302,
    /// METHOD-REQUEST-PROPSS
    MethodRequestPropss                                                    = 1866,
    /// METHOD-REQUEST-TOPIC-NAME
    MethodRequestTopicName                                                 = 3660,
    /// METHOD-RESPONSE-PROPSS
    MethodResponsePropss                                                   = 2354,
    /// METHOD-RETURN-REF
    MethodReturnRef                                                        = 1305,
    /// METHOD-RETURN-REFS
    MethodReturnRefs                                                       = 5955,
    /// METHOD-TOPICS-ACCESS-RULE-REF
    MethodTopicsAccessRuleRef                                              = 1775,
    /// METHODS
    Methods                                                                = 5737,
    /// MICRO-INITIAL-OFFSET-A
    MicroInitialOffsetA                                                    = 2075,
    /// MICRO-INITIAL-OFFSET-B
    MicroInitialOffsetB                                                    = 543,
    /// MICRO-PER-CYCLE
    MicroPerCycle                                                          = 4569,
    /// MICROTICK-DURATION
    MicrotickDuration                                                      = 1758,
    /// MIN
    Min                                                                    = 2543,
    /// MIN-ALIVE-CYCLE
    MinAliveCycle                                                          = 4967,
    /// MIN-DATA-LENGTH
    MinDataLength                                                          = 1122,
    /// MIN-DEADLINE
    MinDeadline                                                            = 1603,
    /// MIN-DESTINATION-PORT-NUMBER
    MinDestinationPortNumber                                               = 2245,
    /// MIN-HEADER-ID
    MinHeaderId                                                            = 6139,
    /// MIN-LENGTH
    MinLength                                                              = 1881,
    /// MIN-MAJOR-VERSION
    MinMajorVersion                                                        = 5544,
    /// MIN-MARGIN
    MinMargin                                                              = 2190,
    /// MIN-MINOR-VERSION
    MinMinorVersion                                                        = 3552,
    /// MIN-NUMBER-OF-ELEMENTS
    MinNumberOfElements                                                    = 3587,
    /// MIN-NUMBER-OF-OCCURRENCES
    MinNumberOfOccurrences                                                 = 2113,
    /// MIN-NUMBER-OF-TIME-QUANTA-PER-BIT
    MinNumberOfTimeQuantaPerBit                                            = 5294,
    /// MIN-OK-STATE-INIT
    MinOkStateInit                                                         = 2584,
    /// MIN-OK-STATE-INVALID
    MinOkStateInvalid                                                      = 169,
    /// MIN-OK-STATE-VALID
    MinOkStateValid                                                        = 4362,
    /// MIN-PWM-L
    MinPwmL                                                                = 5071,
    /// MIN-PWM-O
    MinPwmO                                                                = 2507,
    /// MIN-PWM-S
    MinPwmS                                                                = 2194,
    /// MIN-RPT-EVENT-ID
    MinRptEventId                                                          = 5857,
    /// MIN-SAMPLE-POINT
    MinSamplePoint                                                         = 1747,
    /// MIN-SERVICE-POINT-ID
    MinServicePointId                                                      = 2343,
    /// MIN-SOURCE-PORT-NUMBER
    MinSourcePortNumber                                                    = 5240,
    /// MIN-SYNC-JUMP-WIDTH
    MinSyncJumpWidth                                                       = 1948,
    /// MIN-TRCV-DELAY-COMPENSATION-OFFSET
    MinTrcvDelayCompensationOffset                                         = 4871,
    /// MIN-VALUE
    MinValue                                                               = 1592,
    /// MINIMUM
    Minimum                                                                = 1351,
    /// MINIMUM-DELAY
    MinimumDelay                                                           = 1512,
    /// MINIMUM-DISTANCES
    MinimumDistances                                                       = 2927,
    /// MINIMUM-EXECUTION-TIME
    MinimumExecutionTime                                                   = 5853,
    /// MINIMUM-FIFO-LENGTH
    MinimumFifoLength                                                      = 570,
    /// MINIMUM-INTENDED-BINDING-TIME
    MinimumIntendedBindingTime                                             = 3691,
    /// MINIMUM-INTER-ARRIVAL-TIME
    MinimumInterArrivalTime                                                = 2855,
    /// MINIMUM-INTERVAL-LENGTH
    MinimumIntervalLength                                                  = 4974,
    /// MINIMUM-MEMORY-CONSUMPTION
    MinimumMemoryConsumption                                               = 4501,
    /// MINIMUM-MULTICAST-SEPERATION-TIME
    MinimumMulticastSeperationTime                                         = 1338,
    /// MINIMUM-NUMBER-OF-REQUESTS
    MinimumNumberOfRequests                                                = 6239,
    /// MINIMUM-RX-CONTAINER-QUEUE-SIZE
    MinimumRxContainerQueueSize                                            = 5993,
    /// MINIMUM-SELECTED-BINDING-TIME
    MinimumSelectedBindingTime                                             = 4973,
    /// MINIMUM-SEND-INTERVAL
    MinimumSendInterval                                                    = 4301,
    /// MINIMUM-SEPARATION-TIME
    MinimumSeparationTime                                                  = 3619,
    /// MINIMUM-START-INTERVAL
    MinimumStartInterval                                                   = 4731,
    /// MINIMUM-SUPPORTED-UCM-MASTER-VERSION
    MinimumSupportedUcmMasterVersion                                       = 4893,
    /// MINIMUM-SUPPORTED-UCM-VERSION
    MinimumSupportedUcmVersion                                             = 5622,
    /// MINIMUM-SUSTAINED-SIZE
    MinimumSustainedSize                                                   = 1323,
    /// MINIMUM-TIMER-GRANULARITY
    MinimumTimerGranularity                                                = 152,
    /// MINIMUM-TX-CONTAINER-QUEUE-SIZE
    MinimumTxContainerQueueSize                                            = 4495,
    /// MINISLOT-ACTION-POINT-OFFSET
    MinislotActionPointOffset                                              = 2829,
    /// MINISLOT-DURATION
    MinislotDuration                                                       = 15,
    /// MINOR-PROTOCOL-VERSION
    MinorProtocolVersion                                                   = 1132,
    /// MINOR-VERSION
    MinorVersion                                                           = 5738,
    /// MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID
    MirrorSourceLinToCanRangeBaseId                                        = 1187,
    /// MIRROR-STATUS-CAN-ID
    MirrorStatusCanId                                                      = 4662,
    /// MIRRORING-PROTOCOL
    MirroringProtocol                                                      = 5726,
    /// MIXED-BUS-TYPE-COLLECTION
    MixedBusTypeCollection                                                 = 3104,
    /// MKA-PARTICIPANT-REF
    MkaParticipantRef                                                      = 199,
    /// MKA-PARTICIPANT-REFS
    MkaParticipantRefs                                                     = 5258,
    /// MKA-PARTICIPANTS
    MkaParticipants                                                        = 5878,
    /// ML-GRAPHIC
    MlGraphic                                                              = 4702,
    /// MODE
    Mode                                                                   = 1834,
    /// MODE-ACCESS-POINT
    ModeAccessPoint                                                        = 3706,
    /// MODE-ACCESS-POINTS
    ModeAccessPoints                                                       = 3452,
    /// MODE-CONDITION
    ModeCondition                                                          = 5804,
    /// MODE-CONDITION-REF
    ModeConditionRef                                                       = 1390,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 708,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 5284,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 5525,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-IREF
    ModeDeclarationGroupPrototypeIref                                      = 4197,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ModeDeclarationGroupPrototypeRef                                       = 1900,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL
    ModeDeclarationGroupPrototypeRefConditional                            = 4170,
    /// MODE-DECLARATION-GROUP-REF
    ModeDeclarationGroupRef                                                = 259,
    /// MODE-DECLARATION-GROUP-REFS
    ModeDeclarationGroupRefs                                               = 181,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 3631,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 6024,
    /// MODE-DECLARATION-MAPPING-SET-REF
    ModeDeclarationMappingSetRef                                           = 2108,
    /// MODE-DECLARATION-MAPPINGS
    ModeDeclarationMappings                                                = 3758,
    /// MODE-DECLARATION-REF
    ModeDeclarationRef                                                     = 5921,
    /// MODE-DECLARATION-REFS
    ModeDeclarationRefs                                                    = 4456,
    /// MODE-DECLARATIONS
    ModeDeclarations                                                       = 4469,
    /// MODE-DEPENDENT-STARTUP-CONFIG
    ModeDependentStartupConfig                                             = 874,
    /// MODE-DEPENDENT-STARTUP-CONFIGS
    ModeDependentStartupConfigs                                            = 1219,
    /// MODE-DRIVEN-FALSE-CONDITIONS
    ModeDrivenFalseConditions                                              = 1428,
    /// MODE-DRIVEN-TRANSMISSION-MODE-CONDITION
    ModeDrivenTransmissionModeCondition                                    = 1159,
    /// MODE-DRIVEN-TRUE-CONDITIONS
    ModeDrivenTrueConditions                                               = 3597,
    /// MODE-ELEMENT-REF
    ModeElementRef                                                         = 1559,
    /// MODE-ELEMENTS
    ModeElements                                                           = 5004,
    /// MODE-GROUP
    ModeGroup                                                              = 4885,
    /// MODE-GROUP-IREF
    ModeGroupIref                                                          = 3301,
    /// MODE-GROUP-REF
    ModeGroupRef                                                           = 515,
    /// MODE-IN-BSW-INSTANCE-REF
    ModeInBswInstanceRef                                                   = 3683,
    /// MODE-IN-SWC-INSTANCE-REF
    ModeInSwcInstanceRef                                                   = 5228,
    /// MODE-INSTANCE
    ModeInstance                                                           = 4453,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 645,
    /// MODE-IREF
    ModeIref                                                               = 2775,
    /// MODE-IREFS
    ModeIrefs                                                              = 1280,
    /// MODE-MANAGER-ERROR-BEHAVIOR
    ModeManagerErrorBehavior                                               = 939,
    /// MODE-MAPPING
    ModeMapping                                                            = 143,
    /// MODE-PORT-ANNOTATION
    ModePortAnnotation                                                     = 594,
    /// MODE-PORT-ANNOTATIONS
    ModePortAnnotations                                                    = 3754,
    /// MODE-RECEIVER-POLICYS
    ModeReceiverPolicys                                                    = 4719,
    /// MODE-REF
    ModeRef                                                                = 5448,
    /// MODE-REPORTER-PORT-IREF
    ModeReporterPortIref                                                   = 1637,
    /// MODE-REQUEST-TYPE-MAP
    ModeRequestTypeMap                                                     = 4289,
    /// MODE-REQUEST-TYPE-MAPS
    ModeRequestTypeMaps                                                    = 2156,
    /// MODE-SENDER-POLICYS
    ModeSenderPolicys                                                      = 3290,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY
    ModeSwitchEventTriggeredActivity                                       = 4154,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS
    ModeSwitchEventTriggeredActivitys                                      = 567,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 1226,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 854,
    /// MODE-SWITCH-POINTS
    ModeSwitchPoints                                                       = 914,
    /// MODE-SWITCH-RECEIVER-COM-SPEC
    ModeSwitchReceiverComSpec                                              = 4196,
    /// MODE-SWITCH-SENDER-COM-SPEC
    ModeSwitchSenderComSpec                                                = 1456,
    /// MODE-SWITCHED-ACK
    ModeSwitchedAck                                                        = 4522,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 3026,
    /// MODE-TRANSITION
    ModeTransition                                                         = 4422,
    /// MODE-TRANSITIONS
    ModeTransitions                                                        = 6215,
    /// MODE-USER-ERROR-BEHAVIOR
    ModeUserErrorBehavior                                                  = 171,
    /// MODEL-OBJECT-REFERENCE
    ModelObjectReference                                                   = 4754,
    /// MODES
    Modes                                                                  = 3422,
    /// MODIFICATION
    Modification                                                           = 2723,
    /// MODIFICATIONS
    Modifications                                                          = 4802,
    /// MODIFIED-DATAS
    ModifiedDatas                                                          = 1698,
    /// MODULE-DESCRIPTION-REF
    ModuleDescriptionRef                                                   = 4912,
    /// MODULE-ID
    ModuleId                                                               = 5883,
    /// MODULE-INSTANTIATION-REF
    ModuleInstantiationRef                                                 = 3270,
    /// MODULE-INSTANTIATION-REFS
    ModuleInstantiationRefs                                                = 3736,
    /// MODULE-INSTANTIATIONS
    ModuleInstantiations                                                   = 705,
    /// MODULE-REF
    ModuleRef                                                              = 4943,
    /// MODULE-REFS
    ModuleRefs                                                             = 2669,
    /// MOLAR-AMOUNT-EXP
    MolarAmountExp                                                         = 904,
    /// MONITORED-IDENTIFIER-REF
    MonitoredIdentifierRef                                                 = 4231,
    /// MONOTONY
    Monotony                                                               = 5771,
    /// MORE-FRAGMENTS
    MoreFragments                                                          = 2254,
    /// MSG-ID-MASK
    MsgIdMask                                                              = 3015,
    /// MSG-ID-MATCH
    MsgIdMatch                                                             = 4681,
    /// MSR-QUERY-ARG
    MsrQueryArg                                                            = 6140,
    /// MSR-QUERY-CHAPTER
    MsrQueryChapter                                                        = 3992,
    /// MSR-QUERY-NAME
    MsrQueryName                                                           = 662,
    /// MSR-QUERY-P-1
    MsrQueryP1                                                             = 2996,
    /// MSR-QUERY-P-2
    MsrQueryP2                                                             = 2337,
    /// MSR-QUERY-PROPS
    MsrQueryProps                                                          = 3436,
    /// MSR-QUERY-RESULT-CHAPTER
    MsrQueryResultChapter                                                  = 3704,
    /// MSR-QUERY-RESULT-P-1
    MsrQueryResultP1                                                       = 5079,
    /// MSR-QUERY-RESULT-P-2
    MsrQueryResultP2                                                       = 5707,
    /// MSR-QUERY-RESULT-TOPIC-1
    MsrQueryResultTopic1                                                   = 4304,
    /// MSR-QUERY-TOPIC-1
    MsrQueryTopic1                                                         = 2680,
    /// MULTICAST-CONNECTOR-REF
    MulticastConnectorRef                                                  = 1247,
    /// MULTICAST-CONNECTOR-REFS
    MulticastConnectorRefs                                                 = 4824,
    /// MULTICAST-CREDENTIALS
    MulticastCredentials                                                   = 3558,
    /// MULTICAST-REF
    MulticastRef                                                           = 1733,
    /// MULTICAST-SD-IP-ADDRESS-REF
    MulticastSdIpAddressRef                                                = 1847,
    /// MULTICAST-SECURE-COM-PROPS-REF
    MulticastSecureComPropsRef                                             = 2591,
    /// MULTICAST-SEGMENTATION
    MulticastSegmentation                                                  = 3168,
    /// MULTICAST-THRESHOLD
    MulticastThreshold                                                     = 624,
    /// MULTICAST-UDP-PORT
    MulticastUdpPort                                                       = 3847,
    /// MULTIPLE-CONFIGURATION-CONTAINER
    MultipleConfigurationContainer                                         = 1685,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 3872,
    /// MULTIPLICITY-CONFIG-CLASSES
    MultiplicityConfigClasses                                              = 4952,
    /// MULTIPLICITY-RESTRICTION
    MultiplicityRestriction                                                = 3649,
    /// N
    N                                                                      = 1278,
    /// N-DATA-SETS
    NDataSets                                                              = 1434,
    /// N-PDU
    NPdu                                                                   = 284,
    /// N-PDU-REF
    NPduRef                                                                = 4299,
    /// N-PDU-REFS
    NPduRefs                                                               = 130,
    /// N-ROM-BLOCKS
    NRomBlocks                                                             = 671,
    /// NAGLES-ALGORITHM
    NaglesAlgorithm                                                        = 1410,
    /// NAME
    Name                                                                   = 6033,
    /// NAME-TOKEN-VALUE-VARIATION-POINT
    NameTokenValueVariationPoint                                           = 3010,
    /// NAMED-CURVE-ID
    NamedCurveId                                                           = 2106,
    /// NAMESPACES
    Namespaces                                                             = 5075,
    /// NAS-TIMEOUT
    NasTimeout                                                             = 1985,
    /// NATIVE-DECLARATION
    NativeDeclaration                                                      = 1041,
    /// NDP-PROPS
    NdpProps                                                               = 1267,
    /// NEEDS-ORIGINAL-DATA
    NeedsOriginalData                                                      = 1792,
    /// NEIGHBOR-CACHE-SIZE
    NeighborCacheSize                                                      = 3701,
    /// NESTED-ELEMENT-REF
    NestedElementRef                                                       = 4148,
    /// NESTED-ELEMENT-REFS
    NestedElementRefs                                                      = 4770,
    /// NESTED-ELEMENTS
    NestedElements                                                         = 277,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 213,
    /// NETWORK-CONFIGURATION-REF
    NetworkConfigurationRef                                                = 4414,
    /// NETWORK-CONFIGURATION-REFS
    NetworkConfigurationRefs                                               = 5106,
    /// NETWORK-CONFIGURATIONS
    NetworkConfigurations                                                  = 524,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 3929,
    /// NETWORK-ENDPOINT-ADDRESSES
    NetworkEndpointAddresses                                               = 5260,
    /// NETWORK-ENDPOINT-REF
    NetworkEndpointRef                                                     = 2774,
    /// NETWORK-ENDPOINT-REF-CONDITIONAL
    NetworkEndpointRefConditional                                          = 3811,
    /// NETWORK-ENDPOINT-REFS
    NetworkEndpointRefs                                                    = 5404,
    /// NETWORK-ENDPOINTS
    NetworkEndpoints                                                       = 1134,
    /// NETWORK-HANDLE-PORT-MAPPING
    NetworkHandlePortMapping                                               = 4437,
    /// NETWORK-HANDLE-REF
    NetworkHandleRef                                                       = 1249,
    /// NETWORK-HANDLES
    NetworkHandles                                                         = 5636,
    /// NETWORK-ID
    NetworkId                                                              = 2987,
    /// NETWORK-IDLE-TIME
    NetworkIdleTime                                                        = 4886,
    /// NETWORK-INTERFACE-ID
    NetworkInterfaceId                                                     = 2964,
    /// NETWORK-INTERFACE-REF
    NetworkInterfaceRef                                                    = 1111,
    /// NETWORK-INTERFACE-REFS
    NetworkInterfaceRefs                                                   = 1241,
    /// NETWORK-INTERFACES
    NetworkInterfaces                                                      = 1072,
    /// NETWORK-LAYER-RULE
    NetworkLayerRule                                                       = 4520,
    /// NETWORK-MANAGEMENT-PORT-INTERFACE
    NetworkManagementPortInterface                                         = 3781,
    /// NETWORK-MANAGEMENT-VECTOR-LENGTH
    NetworkManagementVectorLength                                          = 725,
    /// NETWORK-MASK
    NetworkMask                                                            = 4995,
    /// NETWORK-REPRESENTATION
    NetworkRepresentation                                                  = 6069,
    /// NETWORK-REPRESENTATION-PROPS
    NetworkRepresentationProps                                             = 5129,
    /// NETWORK-SEGMENT-ID
    NetworkSegmentId                                                       = 2483,
    /// NETWORK-TIME-CONSUMER-REF
    NetworkTimeConsumerRef                                                 = 75,
    /// NETWORK-TIME-MASTER-REF
    NetworkTimeMasterRef                                                   = 5330,
    /// NETWORK-TIME-PROVIDER-REF
    NetworkTimeProviderRef                                                 = 3232,
    /// NETWORK-TIME-SLAVE-REF
    NetworkTimeSlaveRef                                                    = 5403,
    /// NEW-NAD
    NewNad                                                                 = 4815,
    /// NEXT-HEADER
    NextHeader                                                             = 2864,
    /// NEXT-HIGHER-CERTIFICATE-REF
    NextHigherCertificateRef                                               = 2658,
    /// NEXT-STATE-IREF
    NextStateIref                                                          = 5952,
    /// NM-ACTIVE-COORDINATOR
    NmActiveCoordinator                                                    = 1340,
    /// NM-BUS-LOAD-REDUCTION-ENABLED
    NmBusLoadReductionEnabled                                              = 5010,
    /// NM-BUS-SYNCHRONIZATION-ENABLED
    NmBusSynchronizationEnabled                                            = 384,
    /// NM-BUSLOAD-REDUCTION-ACTIVE
    NmBusloadReductionActive                                               = 701,
    /// NM-BUSLOAD-REDUCTION-ENABLED
    NmBusloadReductionEnabled                                              = 5468,
    /// NM-CAR-WAKE-UP-BIT-POSITION
    NmCarWakeUpBitPosition                                                 = 4939,
    /// NM-CAR-WAKE-UP-FILTER-ENABLED
    NmCarWakeUpFilterEnabled                                               = 4387,
    /// NM-CAR-WAKE-UP-FILTER-NODE-ID
    NmCarWakeUpFilterNodeId                                                = 503,
    /// NM-CAR-WAKE-UP-RX-ENABLED
    NmCarWakeUpRxEnabled                                                   = 5673,
    /// NM-CBV-POSITION
    NmCbvPosition                                                          = 5137,
    /// NM-CHANNEL-ACTIVE
    NmChannelActive                                                        = 3711,
    /// NM-CHANNEL-ID
    NmChannelId                                                            = 5748,
    /// NM-CHANNEL-SLEEP-MASTER
    NmChannelSleepMaster                                                   = 5662,
    /// NM-CLUSTER-COUPLINGS
    NmClusterCouplings                                                     = 5431,
    /// NM-CLUSTERS
    NmClusters                                                             = 3278,
    /// NM-COM-CONTROL-ENABLED
    NmComControlEnabled                                                    = 1313,
    /// NM-CONFIG
    NmConfig                                                               = 294,
    /// NM-CONTROL-BIT-VECTOR-ACTIVE
    NmControlBitVectorActive                                               = 796,
    /// NM-CONTROL-BIT-VECTOR-ENABLED
    NmControlBitVectorEnabled                                              = 5936,
    /// NM-COORD-CLUSTER
    NmCoordCluster                                                         = 494,
    /// NM-COORD-SYNC-SUPPORT
    NmCoordSyncSupport                                                     = 5911,
    /// NM-COORDINATOR
    NmCoordinator                                                          = 5498,
    /// NM-COORDINATOR-ROLE
    NmCoordinatorRole                                                      = 1303,
    /// NM-CYCLETIME-MAIN-FUNCTION
    NmCycletimeMainFunction                                                = 3547,
    /// NM-DATA-CYCLE
    NmDataCycle                                                            = 3601,
    /// NM-DATA-DISABLED
    NmDataDisabled                                                         = 2718,
    /// NM-DATA-ENABLED
    NmDataEnabled                                                          = 4713,
    /// NM-DATA-INFORMATION
    NmDataInformation                                                      = 1753,
    /// NM-DETECTION-LOCK
    NmDetectionLock                                                        = 3501,
    /// NM-ECU
    NmEcu                                                                  = 1657,
    /// NM-GLOBAL-COORDINATOR-TIME
    NmGlobalCoordinatorTime                                                = 809,
    /// NM-HANDLE-REF
    NmHandleRef                                                            = 2103,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 142,
    /// NM-HW-VOTE-ENABLED
    NmHwVoteEnabled                                                        = 5131,
    /// NM-IF-ECU-REF
    NmIfEcuRef                                                             = 1221,
    /// NM-IF-ECUS
    NmIfEcus                                                               = 3966,
    /// NM-IMMEDIATE-NM-CYCLE-TIME
    NmImmediateNmCycleTime                                                 = 5615,
    /// NM-IMMEDIATE-NM-TRANSMISSIONS
    NmImmediateNmTransmissions                                             = 6264,
    /// NM-IMMEDIATE-RESTART-ENABLED
    NmImmediateRestartEnabled                                              = 1666,
    /// NM-INSTANCE-ID
    NmInstanceId                                                           = 3324,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 1906,
    /// NM-INTERACTS-WITH-SM-MAPPING
    NmInteractsWithSmMapping                                               = 3158,
    /// NM-MAIN-FUNCTION-ACROSS-FR-CYCLE
    NmMainFunctionAcrossFrCycle                                            = 3944,
    /// NM-MAIN-FUNCTION-PERIOD
    NmMainFunctionPeriod                                                   = 1610,
    /// NM-MESSAGE-TIMEOUT-TIME
    NmMessageTimeoutTime                                                   = 5531,
    /// NM-MSG-CYCLE-OFFSET
    NmMsgCycleOffset                                                       = 579,
    /// NM-MSG-CYCLE-TIME
    NmMsgCycleTime                                                         = 6002,
    /// NM-MSG-REDUCED-TIME
    NmMsgReducedTime                                                       = 462,
    /// NM-MULTIPLE-CHANNELS-ENABLED
    NmMultipleChannelsEnabled                                              = 4747,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 5903,
    /// NM-NETWORK-HANDLE-REF
    NmNetworkHandleRef                                                     = 4736,
    /// NM-NETWORK-TIMEOUT
    NmNetworkTimeout                                                       = 4511,
    /// NM-NID-POSITION
    NmNidPosition                                                          = 604,
    /// NM-NODE-DETECTION-ENABLED
    NmNodeDetectionEnabled                                                 = 4740,
    /// NM-NODE-ID
    NmNodeId                                                               = 3100,
    /// NM-NODE-ID-ENABLED
    NmNodeIdEnabled                                                        = 5349,
    /// NM-NODE-REF
    NmNodeRef                                                              = 4274,
    /// NM-NODE-REFS
    NmNodeRefs                                                             = 4315,
    /// NM-NODES
    NmNodes                                                                = 3644,
    /// NM-PASSIVE-MODE-ENABLED
    NmPassiveModeEnabled                                                   = 3922,
    /// NM-PDU
    NmPdu                                                                  = 6203,
    /// NM-PDU-REF
    NmPduRef                                                               = 512,
    /// NM-PDU-REF-CONDITIONAL
    NmPduRefConditional                                                    = 372,
    /// NM-PDU-RX-INDICATION-ENABLED
    NmPduRxIndicationEnabled                                               = 5987,
    /// NM-PDUS
    NmPdus                                                                 = 4373,
    /// NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS
    NmPnHandleMultipleNetworkRequests                                      = 102,
    /// NM-PNC-PARTICIPATION
    NmPncParticipation                                                     = 2711,
    /// NM-RANGE-CONFIG
    NmRangeConfig                                                          = 6213,
    /// NM-READY-SLEEP-COUNT
    NmReadySleepCount                                                      = 6105,
    /// NM-READY-SLEEP-TIME
    NmReadySleepTime                                                       = 3892,
    /// NM-REMOTE-SLEEP-IND-ENABLED
    NmRemoteSleepIndEnabled                                                = 2389,
    /// NM-REMOTE-SLEEP-INDICATION-TIME
    NmRemoteSleepIndicationTime                                            = 500,
    /// NM-REPEAT-MESSAGE-BIT-ACTIVE
    NmRepeatMessageBitActive                                               = 3491,
    /// NM-REPEAT-MESSAGE-BIT-ENABLE
    NmRepeatMessageBitEnable                                               = 1565,
    /// NM-REPEAT-MESSAGE-TIME
    NmRepeatMessageTime                                                    = 2520,
    /// NM-REPEAT-MSG-IND-ENABLED
    NmRepeatMsgIndEnabled                                                  = 4757,
    /// NM-REPEAT-MSG-INDICATION-ENABLED
    NmRepeatMsgIndicationEnabled                                           = 5624,
    /// NM-REPETITION-CYCLE
    NmRepetitionCycle                                                      = 3596,
    /// NM-SCHEDULE-VARIANT
    NmScheduleVariant                                                      = 2452,
    /// NM-SHUTDOWN-DELAY-TIMER
    NmShutdownDelayTimer                                                   = 3931,
    /// NM-STATE-CHANGE-IND-ENABLED
    NmStateChangeIndEnabled                                                = 1306,
    /// NM-STATE-REQUEST
    NmStateRequest                                                         = 1857,
    /// NM-SYNCHRONIZATION-POINT-ENABLED
    NmSynchronizationPointEnabled                                          = 1864,
    /// NM-SYNCHRONIZING-NETWORK
    NmSynchronizingNetwork                                                 = 813,
    /// NM-USER-DATA-ENABLED
    NmUserDataEnabled                                                      = 4175,
    /// NM-USER-DATA-LENGTH
    NmUserDataLength                                                       = 2035,
    /// NM-USER-DATA-OFFSET
    NmUserDataOffset                                                       = 5511,
    /// NM-VECTOR-EARLY-UPDATE
    NmVectorEarlyUpdate                                                    = 731,
    /// NM-VOTE-INFORMATION
    NmVoteInformation                                                      = 5943,
    /// NM-VOTING-CYCLE
    NmVotingCycle                                                          = 394,
    /// NM-WAIT-BUS-SLEEP-TIME
    NmWaitBusSleepTime                                                     = 6257,
    /// NO-CHECKPOINT-SUPERVISION
    NoCheckpointSupervision                                                = 2966,
    /// NO-CHECKPOINT-SUPERVISIONS
    NoCheckpointSupervisions                                               = 2599,
    /// NO-SUPERVISION
    NoSupervision                                                          = 5437,
    /// NO-SUPERVISIONS
    NoSupervisions                                                         = 5767,
    /// NODE-IDENTIFIER
    NodeIdentifier                                                         = 963,
    /// NODE-NAME
    NodeName                                                               = 5018,
    /// NODE-PORTS
    NodePorts                                                              = 3259,
    /// NODE-REF
    NodeRef                                                                = 3535,
    /// NOMINAL
    Nominal                                                                = 1270,
    /// NOMINAL-EXECUTION-TIME
    NominalExecutionTime                                                   = 5961,
    /// NON-OS-MODULE-INSTANTIATION-REF
    NonOsModuleInstantiationRef                                            = 4503,
    /// NON-VERBOSE-MODE
    NonVerboseMode                                                         = 2570,
    /// NONQUEUED-RECEIVER-COM-SPEC
    NonqueuedReceiverComSpec                                               = 5708,
    /// NONQUEUED-SENDER-COM-SPEC
    NonqueuedSenderComSpec                                                 = 5644,
    /// NOT-AVAILABLE-VALUE-SPECIFICATION
    NotAvailableValueSpecification                                         = 3672,
    /// NOTATION
    Notation                                                               = 4237,
    /// NOTE
    Note                                                                   = 4382,
    /// NOTIFICATION-PORT-IREF
    NotificationPortIref                                                   = 280,
    /// NOTIFICATION-STATE
    NotificationState                                                      = 3730,
    /// NOTIFICATION-TIME
    NotificationTime                                                       = 5652,
    /// NOTIFICATIONS
    Notifications                                                          = 2105,
    /// NOTIFIED-STATUSS
    NotifiedStatuss                                                        = 271,
    /// NOTIFIER
    Notifier                                                               = 2666,
    /// NOTIFIER-DATA-ELEMENT-REF
    NotifierDataElementRef                                                 = 4405,
    /// NOTIFIER-REF
    NotifierRef                                                            = 2589,
    /// NOTIFIER-SIGNAL-TRIGGERING-REF
    NotifierSignalTriggeringRef                                            = 4776,
    /// NRC-VALUE
    NrcValue                                                               = 3722,
    /// NTU
    Ntu                                                                    = 3852,
    /// NUM-FAILED-SECURITY-ACCESS
    NumFailedSecurityAccess                                                = 1700,
    /// NUMBER
    Number                                                                 = 5003,
    /// NUMBER-OF-BITS
    NumberOfBits                                                           = 3209,
    /// NUMBER-OF-CONNECTED-CLIENTS
    NumberOfConnectedClients                                               = 4365,
    /// NUMBER-OF-INSTRUCTIONS
    NumberOfInstructions                                                   = 69,
    /// NUMBER-OF-MINISLOTS
    NumberOfMinislots                                                      = 147,
    /// NUMBER-OF-NOTIFIER-SETS
    NumberOfNotifierSets                                                   = 4328,
    /// NUMBER-OF-PARALLEL-TCP-SESSIONS
    NumberOfParallelTcpSessions                                            = 2335,
    /// NUMBER-OF-REPETITIONS
    NumberOfRepetitions                                                    = 4846,
    /// NUMBER-OF-RESTART-ATTEMPTS
    NumberOfRestartAttempts                                                = 4588,
    /// NUMBER-OF-STATIC-SLOTS
    NumberOfStaticSlots                                                    = 2988,
    /// NUMBER-OF-WORKERS
    NumberOfWorkers                                                        = 2814,
    /// NUMERICAL-RULE-BASED-VALUE-SPECIFICATION
    NumericalRuleBasedValueSpecification                                   = 4697,
    /// NUMERICAL-VALUE
    NumericalValue                                                         = 3453,
    /// NUMERICAL-VALUE-SPECIFICATION
    NumericalValueSpecification                                            = 3333,
    /// NUMERICAL-VALUE-VARIATION-POINT
    NumericalValueVariationPoint                                           = 5983,
    /// NV-BLOCK-DATA-MAPPING
    NvBlockDataMapping                                                     = 4352,
    /// NV-BLOCK-DATA-MAPPINGS
    NvBlockDataMappings                                                    = 2947,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 3969,
    /// NV-BLOCK-DESCRIPTORS
    NvBlockDescriptors                                                     = 1329,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 6209,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 4552,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 29,
    /// NV-DATA-PORT-ANNOTATION
    NvDataPortAnnotation                                                   = 2765,
    /// NV-DATA-PORT-ANNOTATIONS
    NvDataPortAnnotations                                                  = 6183,
    /// NV-DATAS
    NvDatas                                                                = 5560,
    /// NV-PROVIDE-COM-SPEC
    NvProvideComSpec                                                       = 5656,
    /// NV-RAM-BLOCK-ELEMENT
    NvRamBlockElement                                                      = 3128,
    /// NV-REQUIRE-COM-SPEC
    NvRequireComSpec                                                       = 551,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 4563,
    /// OBD-DTC-NUMBER
    ObdDtcNumber                                                           = 4442,
    /// OBD-DTC-VALUE
    ObdDtcValue                                                            = 4202,
    /// OBD-DTC-VALUE-3-BYTE
    ObdDtcValue3Byte                                                       = 3381,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 3851,
    /// OBD-MID
    ObdMid                                                                 = 3059,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 225,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 935,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 3571,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 1728,
    /// OBD-SUPPORT
    ObdSupport                                                             = 6142,
    /// OBJECT-DEFINITION-REF
    ObjectDefinitionRef                                                    = 1708,
    /// OBJECT-DEFINITION-REFS
    ObjectDefinitionRefs                                                   = 4181,
    /// OBJECT-DEFINTION-REF
    ObjectDefintionRef                                                     = 5047,
    /// OBJECT-DEFINTION-REFS
    ObjectDefintionRefs                                                    = 1312,
    /// OBJECT-REF
    ObjectRef                                                              = 352,
    /// OBJECT-REFS
    ObjectRefs                                                             = 2224,
    /// OCCURRENCE-COUNTER-PROCESSING
    OccurrenceCounterProcessing                                            = 4504,
    /// OCCURRENCE-EXPRESSION
    OccurrenceExpression                                                   = 1781,
    /// OFFER-CYCLIC-DELAY
    OfferCyclicDelay                                                       = 2080,
    /// OFFSET
    Offset                                                                 = 5667,
    /// OFFSET-CORRECTION-ADAPTION-INTERVAL
    OffsetCorrectionAdaptionInterval                                       = 2501,
    /// OFFSET-CORRECTION-JUMP-THRESHOLD
    OffsetCorrectionJumpThreshold                                          = 6262,
    /// OFFSET-CORRECTION-OUT
    OffsetCorrectionOut                                                    = 4591,
    /// OFFSET-CORRECTION-START
    OffsetCorrectionStart                                                  = 1448,
    /// OFFSET-SI-TO-UNIT
    OffsetSiToUnit                                                         = 2715,
    /// OFFSET-TIME-DOMAIN-REF
    OffsetTimeDomainRef                                                    = 1640,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 5855,
    /// OFFSETS
    Offsets                                                                = 4248,
    /// OFNS-DATA-ID-LIST
    OfnsDataIdList                                                         = 2733,
    /// OFNS-DATA-ID-LISTS
    OfnsDataIdLists                                                        = 4629,
    /// OFS-DATA-ID-LIST
    OfsDataIdList                                                          = 3856,
    /// OFS-DATA-ID-LISTS
    OfsDataIdLists                                                         = 1795,
    /// OFS-SUB-TLV
    OfsSubTlv                                                              = 3529,
    /// ON-BOARD-MONITOR-ID
    OnBoardMonitorId                                                       = 2540,
    /// ON-FAIL-PERMISSIVE-MODE
    OnFailPermissiveMode                                                   = 2161,
    /// ON-FAIL-PERMISSIVE-MODE-TIMEOUT
    OnFailPermissiveModeTimeout                                            = 349,
    /// ON-TRANSITION-VALUE
    OnTransitionValue                                                      = 5435,
    /// ONE-EVERY-N
    OneEveryN                                                              = 67,
    /// OP
    Op                                                                     = 5926,
    /// OPERATION-ARGUMENT-INSTANCE-IREF
    OperationArgumentInstanceIref                                          = 434,
    /// OPERATION-CYCLE
    OperationCycle                                                         = 2347,
    /// OPERATION-CYCLE-AUTOMATIC-END
    OperationCycleAutomaticEnd                                             = 358,
    /// OPERATION-CYCLE-AUTOSTART
    OperationCycleAutostart                                                = 2907,
    /// OPERATION-CYCLE-REF
    OperationCycleRef                                                      = 1056,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 4142,
    /// OPERATION-IREF
    OperationIref                                                          = 2911,
    /// OPERATION-IREFS
    OperationIrefs                                                         = 855,
    /// OPERATION-MAPPINGS
    OperationMappings                                                      = 1811,
    /// OPERATION-MODE
    OperationMode                                                          = 4777,
    /// OPERATION-PROTOTYPE-IREF
    OperationPrototypeIref                                                 = 5892,
    /// OPERATION-REF
    OperationRef                                                           = 1182,
    /// OPERATIONS
    Operations                                                             = 2294,
    /// OPERATOR
    Operator                                                               = 4908,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF
    OptAppRecordElementInMethodIref                                        = 5210,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS
    OptAppRecordElementInMethodIrefs                                       = 3399,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    OptAppRecordElementInServiceInterfaceIref                              = 5658,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS
    OptAppRecordElementInServiceInterfaceIrefs                             = 3342,
    /// OPT-ARGUMENT-REF
    OptArgumentRef                                                         = 1982,
    /// OPT-ARGUMENT-REFS
    OptArgumentRefs                                                        = 5492,
    /// OPT-IMPL-RECORD-ELEMENT
    OptImplRecordElement                                                   = 5006,
    /// OPT-IMPL-RECORD-ELEMENTS
    OptImplRecordElements                                                  = 3228,
    /// OPTION
    Option                                                                 = 5931,
    /// OPTION-ARGUMENT
    OptionArgument                                                         = 6208,
    /// OPTION-KIND
    OptionKind                                                             = 1877,
    /// OPTION-NAME
    OptionName                                                             = 4133,
    /// OPTIONAL
    Optional                                                               = 4395,
    /// OPTIONAL-ELEMENTS
    OptionalElements                                                       = 4195,
    /// OPTIONS
    Options                                                                = 3222,
    /// ORDERED-ELEMENTS
    OrderedElements                                                        = 1223,
    /// ORDERED-MASTER
    OrderedMaster                                                          = 4016,
    /// ORDERED-MASTER-LIST
    OrderedMasterList                                                      = 3825,
    /// ORIGIN
    Origin                                                                 = 5565,
    /// OS-ARTI-ADAPTER-LAUNCH-BEHAVIOR
    OsArtiAdapterLaunchBehavior                                            = 5741,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 1842,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 1254,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 5789,
    /// OS-TASK-PROXY-REF
    OsTaskProxyRef                                                         = 5285,
    /// OUT-MEASUREMENT-SET
    OutMeasurementSet                                                      = 464,
    /// OUT-MEASURMENT-SET
    OutMeasurmentSet                                                       = 5082,
    /// OUTER-PORT-REF
    OuterPortRef                                                           = 3274,
    /// OUTER-PORTS
    OuterPorts                                                             = 4949,
    /// OUTGOING-CALLBACKS
    OutgoingCallbacks                                                      = 2440,
    /// OVERRIDE-ID
    OverrideId                                                             = 653,
    /// OVERRIDE-INITIAL-STATE-IREF
    OverrideInitialStateIref                                               = 4061,
    /// OWNERSHIP
    Ownership                                                              = 4745,
    /// OWNERSHIP-KIND
    OwnershipKind                                                          = 330,
    /// OWNERSHIP-STRENGTH
    OwnershipStrength                                                      = 538,
    /// P
    P                                                                      = 1459,
    /// P-2-MAX
    P2Max                                                                  = 944,
    /// P-2-SERVER-MAX
    P2ServerMax                                                            = 3180,
    /// P-2-STAR-SERVER-MAX
    P2StarServerMax                                                        = 1849,
    /// P-2-TIMING
    P2Timing                                                               = 4730,
    /// P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    PModeGroupInAtomicSwcInstanceRef                                       = 2896,
    /// P-PORT-IN-COMPOSITION-INSTANCE-REF
    PPortInCompositionInstanceRef                                          = 1838,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 3052,
    /// P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    PPortPrototypeInExecutableIref                                         = 1196,
    /// P-PORT-PROTOTYPE-IREF
    PPortPrototypeIref                                                     = 1944,
    /// P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF
    PTriggerInAtomicSwcTypeInstanceRef                                     = 5280,
    /// PACKAGE-REF
    PackageRef                                                             = 2895,
    /// PACKAGER-ID
    PackagerId                                                             = 369,
    /// PACKAGER-SIGNATURE-REF
    PackagerSignatureRef                                                   = 5095,
    /// PACKAGING-PIN-NAME
    PackagingPinName                                                       = 2636,
    /// PACKING-BYTE-ORDER
    PackingByteOrder                                                       = 5669,
    /// PADDING-ACTIVATION
    PaddingActivation                                                      = 1252,
    /// PADDING-VALUE
    PaddingValue                                                           = 5021,
    /// PARAMETER
    Parameter                                                              = 1234,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 4389,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REF
    ParameterAccessedForDebugRef                                           = 319,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REFS
    ParameterAccessedForDebugRefs                                          = 4446,
    /// PARAMETER-ACCESSS
    ParameterAccesss                                                       = 4732,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 3073,
    /// PARAMETER-DATA-PROTOTYPE-IREF
    ParameterDataPrototypeIref                                             = 1263,
    /// PARAMETER-ELEMENT-ACCESS
    ParameterElementAccess                                                 = 167,
    /// PARAMETER-ID
    ParameterId                                                            = 1472,
    /// PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT
    ParameterImplementationDataTypeElement                                 = 268,
    /// PARAMETER-INSTANCE
    ParameterInstance                                                      = 5959,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 1608,
    /// PARAMETER-POLICYS
    ParameterPolicys                                                       = 1051,
    /// PARAMETER-PORT-ANNOTATION
    ParameterPortAnnotation                                                = 1156,
    /// PARAMETER-PORT-ANNOTATIONS
    ParameterPortAnnotations                                               = 6129,
    /// PARAMETER-PROVIDE-COM-SPEC
    ParameterProvideComSpec                                                = 5114,
    /// PARAMETER-REF
    ParameterRef                                                           = 1444,
    /// PARAMETER-REQUIRE-COM-SPEC
    ParameterRequireComSpec                                                = 2897,
    /// PARAMETER-SIZE
    ParameterSize                                                          = 3890,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 5776,
    /// PARAMETER-VALUES
    ParameterValues                                                        = 1754,
    /// PARAMETERS
    Parameters                                                             = 1614,
    /// PARENT-CATEGORY
    ParentCategory                                                         = 6195,
    /// PARENT-SHORT-LABEL
    ParentShortLabel                                                       = 2846,
    /// PARTIAL-NETWORK-REF
    PartialNetworkRef                                                      = 1180,
    /// PARTIAL-NETWORK-REFS
    PartialNetworkRefs                                                     = 3677,
    /// PARTICIPATING-J-1939-CLUSTER-REF
    ParticipatingJ1939ClusterRef                                           = 3157,
    /// PARTICIPATING-J-1939-CLUSTER-REFS
    ParticipatingJ1939ClusterRefs                                          = 2579,
    /// PARTITION-NAME
    PartitionName                                                          = 5477,
    /// PARTITION-REF
    PartitionRef                                                           = 4578,
    /// PARTITIONS
    Partitions                                                             = 1075,
    /// PARTS
    Parts                                                                  = 3670,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 4399,
    /// PATH
    Path                                                                   = 1820,
    /// PATH-MTU-DISCOVERY-ENABLED
    PathMtuDiscoveryEnabled                                                = 1276,
    /// PATH-MTU-ENABLED
    PathMtuEnabled                                                         = 4178,
    /// PATH-MTU-TIMEOUT
    PathMtuTimeout                                                         = 1388,
    /// PATTERN
    Pattern                                                                = 1772,
    /// PATTERN-JITTER
    PatternJitter                                                          = 825,
    /// PATTERN-LENGTH
    PatternLength                                                          = 2067,
    /// PATTERN-PERIOD
    PatternPeriod                                                          = 3378,
    /// PAYLOAD-BYTE-PATTERN-RULE
    PayloadBytePatternRule                                                 = 3842,
    /// PAYLOAD-BYTE-PATTERN-RULE-PART
    PayloadBytePatternRulePart                                             = 5132,
    /// PAYLOAD-BYTE-PATTERN-RULE-PARTS
    PayloadBytePatternRuleParts                                            = 4421,
    /// PAYLOAD-BYTE-PATTERN-RULES
    PayloadBytePatternRules                                                = 3508,
    /// PAYLOAD-LENGTH
    PayloadLength                                                          = 1725,
    /// PAYLOAD-LENGTH-STATIC
    PayloadLengthStatic                                                    = 6225,
    /// PAYLOAD-PREAMBLE-INDICATOR
    PayloadPreambleIndicator                                               = 612,
    /// PAYLOAD-REF
    PayloadRef                                                             = 1086,
    /// PAYLOAD-TYPE
    PayloadType                                                            = 1833,
    /// PCM-BIT-DEPTH
    PcmBitDepth                                                            = 2238,
    /// PDELAY-LATENCY-THRESHOLD
    PdelayLatencyThreshold                                                 = 2453,
    /// PDELAY-REQUEST-PERIOD
    PdelayRequestPeriod                                                    = 33,
    /// PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT
    PdelayRespAndRespFollowUpTimeout                                       = 2621,
    /// PDELAY-RESPONSE-ENABLED
    PdelayResponseEnabled                                                  = 135,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 4184,
    /// PDU-ACTIVATION-ROUTING-GROUPS
    PduActivationRoutingGroups                                             = 1183,
    /// PDU-COLLECTION-MAX-BUFFER-SIZE
    PduCollectionMaxBufferSize                                             = 3423,
    /// PDU-COLLECTION-PDU-TIMEOUT
    PduCollectionPduTimeout                                                = 891,
    /// PDU-COLLECTION-SEMANTICS
    PduCollectionSemantics                                                 = 1396,
    /// PDU-COLLECTION-TIMEOUT
    PduCollectionTimeout                                                   = 3963,
    /// PDU-COLLECTION-TRIGGER
    PduCollectionTrigger                                                   = 2379,
    /// PDU-COUNTER-SIZE
    PduCounterSize                                                         = 3493,
    /// PDU-COUNTER-START-POSITION
    PduCounterStartPosition                                                = 1123,
    /// PDU-COUNTER-THRESHOLD
    PduCounterThreshold                                                    = 1773,
    /// PDU-COUNTERS
    PduCounters                                                            = 2210,
    /// PDU-MAX-LENGTH
    PduMaxLength                                                           = 112,
    /// PDU-POOLS
    PduPools                                                               = 1061,
    /// PDU-REF
    PduRef                                                                 = 4723,
    /// PDU-REPLICATION-VOTING
    PduReplicationVoting                                                   = 3895,
    /// PDU-REPLICATIONS
    PduReplications                                                        = 3533,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 2834,
    /// PDU-TO-FRAME-MAPPINGS
    PduToFrameMappings                                                     = 4143,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 621,
    /// PDU-TRIGGERING-REF
    PduTriggeringRef                                                       = 5049,
    /// PDU-TRIGGERING-REF-CONDITIONAL
    PduTriggeringRefConditional                                            = 1345,
    /// PDU-TRIGGERING-REFS
    PduTriggeringRefs                                                      = 4517,
    /// PDU-TRIGGERINGS
    PduTriggerings                                                         = 1941,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 1532,
    /// PDUR-TP-CHUNK-SIZE
    PdurTpChunkSize                                                        = 2971,
    /// PDUS
    Pdus                                                                   = 5059,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 4454,
    /// PER-INSTANCE-MEMORY-REF
    PerInstanceMemoryRef                                                   = 5020,
    /// PER-INSTANCE-MEMORY-SIZE
    PerInstanceMemorySize                                                  = 373,
    /// PER-INSTANCE-MEMORY-SIZES
    PerInstanceMemorySizes                                                 = 823,
    /// PER-INSTANCE-MEMORYS
    PerInstanceMemorys                                                     = 5042,
    /// PER-INSTANCE-PARAMETER-REF
    PerInstanceParameterRef                                                = 242,
    /// PER-INSTANCE-PARAMETERS
    PerInstanceParameters                                                  = 917,
    /// PER-STATE-TIMEOUT
    PerStateTimeout                                                        = 26,
    /// PER-STATE-TIMEOUTS
    PerStateTimeouts                                                       = 3838,
    /// PERIOD
    Period                                                                 = 4346,
    /// PERIOD-BEGIN
    PeriodBegin                                                            = 5089,
    /// PERIOD-END
    PeriodEnd                                                              = 5312,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 858,
    /// PERIODIC-RATE-CATEGORY
    PeriodicRateCategory                                                   = 2769,
    /// PERIODIC-RATES
    PeriodicRates                                                          = 6258,
    /// PERIODIC-RESPONSE-UUDT-REF
    PeriodicResponseUudtRef                                                = 4850,
    /// PERIODIC-RESPONSE-UUDT-REFS
    PeriodicResponseUudtRefs                                               = 3737,
    /// PERMISSIBLE-SIGNAL-PATH
    PermissibleSignalPath                                                  = 3724,
    /// PERMISSION-CERTIFICATE-AUTHORITY-REF
    PermissionCertificateAuthorityRef                                      = 5243,
    /// PERMISSION-TO-CREATE-CHILD-PROCESS
    PermissionToCreateChildProcess                                         = 155,
    /// PERMIT-MULTIPLE-REFERENCES-TO-EE
    PermitMultipleReferencesToEe                                           = 4812,
    /// PERSISTENCY-ACCESS
    PersistencyAccess                                                      = 4878,
    /// PERSISTENCY-CENTRAL-STORAGE-URI
    PersistencyCentralStorageUri                                           = 1083,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 4076,
    /// PERSISTENCY-DATA-PROVIDED-COM-SPEC
    PersistencyDataProvidedComSpec                                         = 5048,
    /// PERSISTENCY-DATA-REQUIRED-COM-SPEC
    PersistencyDataRequiredComSpec                                         = 94,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-REF
    PersistencyDeploymentElementRef                                        = 2344,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 876,
    /// PERSISTENCY-DEPLOYMENT-REF
    PersistencyDeploymentRef                                               = 1164,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 1149,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 3911,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 2131,
    /// PERSISTENCY-DEPLOYMENT-URI
    PersistencyDeploymentUri                                               = 3714,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 5899,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 2525,
    /// PERSISTENCY-FILE-ARRAY-REF
    PersistencyFileArrayRef                                                = 2816,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 4403,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 1185,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 61,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 6172,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 811,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 3563,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 4334,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 4540,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 3818,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 6051,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 4888,
    /// PERSISTENCY-PORT-PROTOTYPE-IREF
    PersistencyPortPrototypeIref                                           = 4000,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 3114,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 5969,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 4388,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 4829,
    /// PERSISTENCY-PROVIDED-COM-SPEC
    PersistencyProvidedComSpec                                             = 5259,
    /// PERSISTENCY-REDUNDANCY-CRC
    PersistencyRedundancyCrc                                               = 3536,
    /// PERSISTENCY-REDUNDANCY-HASH
    PersistencyRedundancyHash                                              = 3188,
    /// PERSISTENCY-REDUNDANCY-M-OUT-OF-N
    PersistencyRedundancyMOutOfN                                           = 2573,
    /// PERSISTENT-STORAGE
    PersistentStorage                                                      = 5916,
    /// PGN
    Pgn                                                                    = 4881,
    /// PHM-ACTION
    PhmAction                                                              = 2565,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 5670,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 3042,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 5946,
    /// PHM-CHECKPOINT-IREF
    PhmCheckpointIref                                                      = 5549,
    /// PHM-CONTRIBUTION-REF
    PhmContributionRef                                                     = 1516,
    /// PHM-CONTRIBUTION-REFS
    PhmContributionRefs                                                    = 3728,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 5353,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 62,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 6019,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 5426,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 6222,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 3868,
    /// PHM-RULE
    PhmRule                                                                = 4458,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 2528,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 27,
    /// PHYS-CONSTRS
    PhysConstrs                                                            = 2734,
    /// PHYSICAL-CHANNEL-REF
    PhysicalChannelRef                                                     = 4397,
    /// PHYSICAL-CHANNEL-REF-CONDITIONAL
    PhysicalChannelRefConditional                                          = 3388,
    /// PHYSICAL-CHANNEL-REFS
    PhysicalChannelRefs                                                    = 4643,
    /// PHYSICAL-CHANNELS
    PhysicalChannels                                                       = 3679,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 5734,
    /// PHYSICAL-DIMENSION-MAPPING
    PhysicalDimensionMapping                                               = 1917,
    /// PHYSICAL-DIMENSION-MAPPING-REF
    PhysicalDimensionMappingRef                                            = 3053,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 3370,
    /// PHYSICAL-DIMENSION-MAPPINGS
    PhysicalDimensionMappings                                              = 2157,
    /// PHYSICAL-DIMENSION-REF
    PhysicalDimensionRef                                                   = 1062,
    /// PHYSICAL-LAYER-TYPE
    PhysicalLayerType                                                      = 549,
    /// PHYSICAL-PROPS
    PhysicalProps                                                          = 3972,
    /// PHYSICAL-REQUEST-REF
    PhysicalRequestRef                                                     = 6144,
    /// PID
    Pid                                                                    = 2558,
    /// PID-REF
    PidRef                                                                 = 2588,
    /// PID-REFS
    PidRefs                                                                = 2046,
    /// PID-SIZE
    PidSize                                                                = 3,
    /// PIN-NUMBER
    PinNumber                                                              = 375,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 921,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 2368,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 2812,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 3525,
    /// PLATFORM-TIME-BASES
    PlatformTimeBases                                                      = 6176,
    /// PLCA-LOCAL-NODE-COUNT
    PlcaLocalNodeCount                                                     = 3788,
    /// PLCA-LOCAL-NODE-ID
    PlcaLocalNodeId                                                        = 121,
    /// PLCA-MAX-BURST-COUNT
    PlcaMaxBurstCount                                                      = 2372,
    /// PLCA-MAX-BURST-TIMER
    PlcaMaxBurstTimer                                                      = 1744,
    /// PLCA-PROPS
    PlcaProps                                                              = 879,
    /// PLCA-TRANSMIT-OPPORTUNITY-TIMER
    PlcaTransmitOpportunityTimer                                           = 647,
    /// PN-RESET-TIME
    PnResetTime                                                            = 5994,
    /// PN-RESET-TIMER
    PnResetTimer                                                           = 4286,
    /// PNC-CLUSTER-VECTOR-LENGTH
    PncClusterVectorLength                                                 = 4999,
    /// PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    PncConsumedProvidedServiceInstanceGroups                               = 2482,
    /// PNC-FILTER-ARRAY-MASK
    PncFilterArrayMask                                                     = 3235,
    /// PNC-FILTER-ARRAY-MASKS
    PncFilterArrayMasks                                                    = 5317,
    /// PNC-FILTER-DATA-MASK
    PncFilterDataMask                                                      = 980,
    /// PNC-GATEWAY-TYPE
    PncGatewayType                                                         = 5086,
    /// PNC-GROUP-REF
    PncGroupRef                                                            = 3618,
    /// PNC-GROUP-REFS
    PncGroupRefs                                                           = 2146,
    /// PNC-IDENTIFIER
    PncIdentifier                                                          = 689,
    /// PNC-MAPPING
    PncMapping                                                             = 520,
    /// PNC-MAPPING-REF
    PncMappingRef                                                          = 1785,
    /// PNC-MAPPING-REFS
    PncMappingRefs                                                         = 2623,
    /// PNC-MAPPINGS
    PncMappings                                                            = 1145,
    /// PNC-NM-REQUEST
    PncNmRequest                                                           = 1012,
    /// PNC-PDUR-GROUP-REF
    PncPdurGroupRef                                                        = 4054,
    /// PNC-PDUR-GROUP-REFS
    PncPdurGroupRefs                                                       = 3019,
    /// PNC-PREPARE-SLEEP-TIMER
    PncPrepareSleepTimer                                                   = 5872,
    /// PNC-SYNCHRONOUS-WAKEUP
    PncSynchronousWakeup                                                   = 5729,
    /// PNC-VECTOR-LENGTH
    PncVectorLength                                                        = 4257,
    /// PNC-VECTOR-OFFSET
    PncVectorOffset                                                        = 2044,
    /// PNC-WAKEUP-CAN-ID
    PncWakeupCanId                                                         = 5328,
    /// PNC-WAKEUP-CAN-ID-EXTENDED
    PncWakeupCanIdExtended                                                 = 6095,
    /// PNC-WAKEUP-CAN-ID-MASK
    PncWakeupCanIdMask                                                     = 780,
    /// PNC-WAKEUP-DATA-MASK
    PncWakeupDataMask                                                      = 4806,
    /// PNC-WAKEUP-DLC
    PncWakeupDlc                                                           = 1887,
    /// PNC-WAKEUP-ENABLE
    PncWakeupEnable                                                        = 4300,
    /// POLICY
    Policy                                                                 = 5065,
    /// POLICY-ACTION
    PolicyAction                                                           = 5579,
    /// PORT-ADDRESS
    PortAddress                                                            = 4954,
    /// PORT-API-OPTION
    PortApiOption                                                          = 2134,
    /// PORT-API-OPTIONS
    PortApiOptions                                                         = 1724,
    /// PORT-ARG-VALUES
    PortArgValues                                                          = 3186,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 751,
    /// PORT-BLUEPRINT-INIT-VALUE
    PortBlueprintInitValue                                                 = 316,
    /// PORT-BLUEPRINT-MAPPING
    PortBlueprintMapping                                                   = 1928,
    /// PORT-BLUEPRINT-MAPPINGS
    PortBlueprintMappings                                                  = 2721,
    /// PORT-BLUEPRINT-REF
    PortBlueprintRef                                                       = 2668,
    /// PORT-CONFIGS
    PortConfigs                                                            = 5462,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINT
    PortDefinedArgumentBlueprint                                           = 4509,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINTS
    PortDefinedArgumentBlueprints                                          = 2684,
    /// PORT-DEFINED-ARGUMENT-VALUE
    PortDefinedArgumentValue                                               = 5382,
    /// PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS
    PortElementToComResourceMappings                                       = 283,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 6223,
    /// PORT-GROUP
    PortGroup                                                              = 4052,
    /// PORT-GROUPS
    PortGroups                                                             = 2338,
    /// PORT-INSTANTIATION-BEHAVIOR
    PortInstantiationBehavior                                              = 4876,
    /// PORT-INTERFACE-BLUEPRINT-MAPPING
    PortInterfaceBlueprintMapping                                          = 3516,
    /// PORT-INTERFACE-BLUEPRINT-REF
    PortInterfaceBlueprintRef                                              = 1471,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 3177,
    /// PORT-INTERFACE-MAPPINGS
    PortInterfaceMappings                                                  = 4946,
    /// PORT-INTERFACE-REF
    PortInterfaceRef                                                       = 5392,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 379,
    /// PORT-NUMBER
    PortNumber                                                             = 176,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 3898,
    /// PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE
    PortPrototypeBlueprintInitValue                                        = 4660,
    /// PORT-PROTOTYPE-BLUEPRINT-MAPPING
    PortPrototypeBlueprintMapping                                          = 5732,
    /// PORT-PROTOTYPE-BLUEPRINT-REF
    PortPrototypeBlueprintRef                                              = 3005,
    /// PORT-PROTOTYPE-IREF
    PortPrototypeIref                                                      = 2149,
    /// PORT-PROTOTYPE-PROPS
    PortPrototypeProps                                                     = 472,
    /// PORT-PROTOTYPE-REF
    PortPrototypeRef                                                       = 2111,
    /// PORT-PROTOTYPE-REF-CONDITIONAL
    PortPrototypeRefConditional                                            = 4673,
    /// PORT-PROTOTYPE-SLUG-FRAGMENT
    PortPrototypeSlugFragment                                              = 4862,
    /// PORT-REF
    PortRef                                                                = 5221,
    /// PORT-SCHEDULER
    PortScheduler                                                          = 487,
    /// PORTS
    Ports                                                                  = 1998,
    /// POSITION
    Position                                                               = 1244,
    /// POSITION-IN-TABLE
    PositionInTable                                                        = 3828,
    /// POSITIVE-INTEGER-VALUE-VARIATION-POINT
    PositiveIntegerValueVariationPoint                                     = 6250,
    /// POSSIBLE-AP-ERROR-REF
    PossibleApErrorRef                                                     = 3779,
    /// POSSIBLE-AP-ERROR-REFS
    PossibleApErrorRefs                                                    = 4256,
    /// POSSIBLE-AP-ERROR-SET-REF
    PossibleApErrorSetRef                                                  = 2409,
    /// POSSIBLE-AP-ERROR-SET-REFS
    PossibleApErrorSetRefs                                                 = 278,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 4136,
    /// POSSIBLE-ERROR-REACTIONS
    PossibleErrorReactions                                                 = 749,
    /// POSSIBLE-ERROR-REF
    PossibleErrorRef                                                       = 712,
    /// POSSIBLE-ERROR-REFS
    PossibleErrorRefs                                                      = 5736,
    /// POSSIBLE-ERRORS
    PossibleErrors                                                         = 2036,
    /// POST-BUILD-CHANGEABLE
    PostBuildChangeable                                                    = 5567,
    /// POST-BUILD-VALUE-ACCESS-REF
    PostBuildValueAccessRef                                                = 1289,
    /// POST-BUILD-VARIANT-CONDITION
    PostBuildVariantCondition                                              = 5303,
    /// POST-BUILD-VARIANT-CONDITIONS
    PostBuildVariantConditions                                             = 3028,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 488,
    /// POST-BUILD-VARIANT-CRITERION-VALUE
    PostBuildVariantCriterionValue                                         = 1653,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 5160,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF
    PostBuildVariantCriterionValueSetRef                                   = 3167,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS
    PostBuildVariantCriterionValueSetRefs                                  = 5681,
    /// POST-BUILD-VARIANT-CRITERION-VALUES
    PostBuildVariantCriterionValues                                        = 5176,
    /// POST-BUILD-VARIANT-MULTIPLICITY
    PostBuildVariantMultiplicity                                           = 1538,
    /// POST-BUILD-VARIANT-SUPPORT
    PostBuildVariantSupport                                                = 3655,
    /// POST-BUILD-VARIANT-USED
    PostBuildVariantUsed                                                   = 5677,
    /// POST-BUILD-VARIANT-VALUE
    PostBuildVariantValue                                                  = 5381,
    /// POST-VERIFICATION-REBOOT
    PostVerificationReboot                                                 = 1026,
    /// POSTS
    Posts                                                                  = 3187,
    /// POWER-DOWN-TIME
    PowerDownTime                                                          = 5457,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 4953,
    /// PRE-ACTIVATE-IREF
    PreActivateIref                                                        = 1481,
    /// PRE-ACTIVATE-IREFS
    PreActivateIrefs                                                       = 3593,
    /// PRE-ACTIVATE-REF
    PreActivateRef                                                         = 3057,
    /// PRE-ACTIVATE-REFS
    PreActivateRefs                                                        = 1717,
    /// PRE-ACTIVATION-REBOOT
    PreActivationReboot                                                    = 4969,
    /// PRE-MAPPING
    PreMapping                                                             = 138,
    /// PRE-SHARED-KEY-REF
    PreSharedKeyRef                                                        = 5098,
    /// PRECONFIGURED-CONFIGURATION-REF
    PreconfiguredConfigurationRef                                          = 3165,
    /// PRECONFIGURED-CONFIGURATION-REFS
    PreconfiguredConfigurationRefs                                         = 2141,
    /// PREDECESSOR-ACTION-REF
    PredecessorActionRef                                                   = 256,
    /// PREDECESSOR-ACTION-REFS
    PredecessorActionRefs                                                  = 5612,
    /// PREDECESSOR-FIFO-REF
    PredecessorFifoRef                                                     = 1633,
    /// PREDECESSOR-REF
    PredecessorRef                                                         = 5485,
    /// PREDECESSOR-REFS
    PredecessorRefs                                                        = 5419,
    /// PREDEFINED-TEXT
    PredefinedText                                                         = 3509,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 1883,
    /// PREEMPTABILITY
    Preemptability                                                         = 2334,
    /// PREFIX
    Prefix                                                                 = 2869,
    /// PREFIX-REF
    PrefixRef                                                              = 3161,
    /// PREPARE-ROLLBACK
    PrepareRollback                                                        = 4351,
    /// PREPARE-UPDATE
    PrepareUpdate                                                          = 5115,
    /// PRESTORAGE-FREEZE-FRAME
    PrestorageFreezeFrame                                                  = 2162,
    /// PRESTORED-FREEZEFRAME-STORED-IN-NVM
    PrestoredFreezeframeStoredInNvm                                        = 4014,
    /// PREVIOUS-CONTRACT-VERSION
    PreviousContractVersion                                                = 3819,
    /// PREVIOUS-DATA-TYPE-REF
    PreviousDataTypeRef                                                    = 651,
    /// PREVIOUS-EXECUTABLE-VERSION
    PreviousExecutableVersion                                              = 2148,
    /// PRIMITIVE
    Primitive                                                              = 801,
    /// PRIMITIVE-ATTRIBUTE-CONDITION
    PrimitiveAttributeCondition                                            = 851,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 6156,
    /// PRIMITIVE-FAMILY
    PrimitiveFamily                                                        = 5608,
    /// PRIMITIVE-TYPE-MAPPINGS
    PrimitiveTypeMappings                                                  = 3750,
    /// PRIORITY
    Priority                                                               = 2129,
    /// PRIORITY-ID
    PriorityId                                                             = 1071,
    /// PRIORITYS
    Prioritys                                                              = 6218,
    /// PRIVACY-LEVEL
    PrivacyLevel                                                           = 502,
    /// PRM
    Prm                                                                    = 1973,
    /// PRM-CHAR
    PrmChar                                                                = 3062,
    /// PRM-UNIT
    PrmUnit                                                                = 3703,
    /// PRMS
    Prms                                                                   = 2866,
    /// PROCESS
    Process                                                                = 4712,
    /// PROCESS-ARGUMENT
    ProcessArgument                                                        = 2429,
    /// PROCESS-ARGUMENTS
    ProcessArguments                                                       = 161,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 5828,
    /// PROCESS-DESIGN-REF
    ProcessDesignRef                                                       = 1361,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 2948,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 1363,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS
    ProcessDesignToMachineDesignMappings                                   = 3680,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 10,
    /// PROCESS-MODE-IREF
    ProcessModeIref                                                        = 182,
    /// PROCESS-MODE-MACHINE
    ProcessModeMachine                                                     = 2022,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 1197,
    /// PROCESS-REF
    ProcessRef                                                             = 608,
    /// PROCESS-STATE-IREF
    ProcessStateIref                                                       = 2312,
    /// PROCESS-STATE-MACHINE
    ProcessStateMachine                                                    = 6079,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 398,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 6056,
    /// PROCESS-TO-MACHINE-MAPPINGS
    ProcessToMachineMappings                                               = 1710,
    /// PROCESSING-KIND
    ProcessingKind                                                         = 3534,
    /// PROCESSING-STYLE
    ProcessingStyle                                                        = 2444,
    /// PROCESSING-UNIT-REF
    ProcessingUnitRef                                                      = 5825,
    /// PROCESSOR
    Processor                                                              = 6067,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 5271,
    /// PROCESSOR-MODE
    ProcessorMode                                                          = 433,
    /// PROCESSOR-SPEED
    ProcessorSpeed                                                         = 1034,
    /// PROCESSORS
    Processors                                                             = 1859,
    /// PRODUCT-ID
    ProductId                                                              = 4624,
    /// PRODUCT-RELEASE
    ProductRelease                                                         = 6179,
    /// PROFILE-BEHAVIOR
    ProfileBehavior                                                        = 5861,
    /// PROFILE-NAME
    ProfileName                                                            = 714,
    /// PROG-CODE
    ProgCode                                                               = 5571,
    /// PROGRAMMING-LANGUAGE
    ProgrammingLanguage                                                    = 2143,
    /// PROP-SEG
    PropSeg                                                                = 3923,
    /// PROPABILITY
    Propability                                                            = 2400,
    /// PROPAGATION-DELAY
    PropagationDelay                                                       = 2626,
    /// PROPERTYS
    Propertys                                                              = 3363,
    /// PROPS
    Props                                                                  = 5597,
    /// PROTOCOL
    Protocol                                                               = 1898,
    /// PROTOCOL-KIND
    ProtocolKind                                                           = 2633,
    /// PROTOCOL-NAME
    ProtocolName                                                           = 34,
    /// PROTOCOL-VERSION
    ProtocolVersion                                                        = 1996,
    /// PROVIDE-RESOURCES
    ProvideResources                                                       = 5248,
    /// PROVIDED-CLIENT-SERVER-ENTRYS
    ProvidedClientServerEntrys                                             = 1616,
    /// PROVIDED-COM-SPECS
    ProvidedComSpecs                                                       = 5481,
    /// PROVIDED-DATA-REF
    ProvidedDataRef                                                        = 5543,
    /// PROVIDED-DATAS
    ProvidedDatas                                                          = 134,
    /// PROVIDED-DDS-EVENT-QOS-PROPS
    ProvidedDdsEventQosProps                                               = 3093,
    /// PROVIDED-DDS-OPERATIONS
    ProvidedDdsOperations                                                  = 1118,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 1209,
    /// PROVIDED-DDS-SERVICE-INSTANCE-EVENTS
    ProvidedDdsServiceInstanceEvents                                       = 574,
    /// PROVIDED-ENTRYS
    ProvidedEntrys                                                         = 3373,
    /// PROVIDED-EVENT-GROUPS
    ProvidedEventGroups                                                    = 2675,
    /// PROVIDED-INTERFACE-TREF
    ProvidedInterfaceTref                                                  = 4622,
    /// PROVIDED-MEMORY-REF
    ProvidedMemoryRef                                                      = 2597,
    /// PROVIDED-MODE-GROUP-REF
    ProvidedModeGroupRef                                                   = 4285,
    /// PROVIDED-MODE-GROUPS
    ProvidedModeGroups                                                     = 3837,
    /// PROVIDED-OUTER-PORT-REF
    ProvidedOuterPortRef                                                   = 1720,
    /// PROVIDED-PORT-PROTOTYPE-IREF
    ProvidedPortPrototypeIref                                              = 1225,
    /// PROVIDED-REQUIRED-INTERFACE-TREF
    ProvidedRequiredInterfaceTref                                          = 4455,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 1467,
    /// PROVIDED-SERVICE-INSTANCE-REF
    ProvidedServiceInstanceRef                                             = 4095,
    /// PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL
    ProvidedServiceInstanceRefConditional                                  = 1699,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 2170,
    /// PROVIDED-SERVICE-INSTANCES
    ProvidedServiceInstances                                               = 3846,
    /// PROVIDED-SERVICE-PORT-IREF
    ProvidedServicePortIref                                                = 857,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 1,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 3897,
    /// PROVIDER-IREF
    ProviderIref                                                           = 3939,
    /// PROVIDER-REF
    ProviderRef                                                            = 2291,
    /// PROVIDER-SOFTWARE-CLUSTER-REF
    ProviderSoftwareClusterRef                                             = 2506,
    /// PROVIEDE-DATA-REF
    ProviedeDataRef                                                        = 5978,
    /// PSK-IDENTITY
    PskIdentity                                                            = 5149,
    /// PSK-IDENTITY-HINT
    PskIdentityHint                                                        = 1850,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 1672,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS
    PskIdentityToKeySlotMappings                                           = 599,
    /// PUBLISHER
    Publisher                                                              = 3949,
    /// PULSE-TEST
    PulseTest                                                              = 1375,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 3699,
    /// PURPOSE-OF-UPDATE-REF
    PurposeOfUpdateRef                                                     = 1106,
    /// PUTS
    Puts                                                                   = 2788,
    /// PWM-L
    PwmL                                                                   = 1200,
    /// PWM-O
    PwmO                                                                   = 2624,
    /// PWM-S
    PwmS                                                                   = 6143,
    /// QOS-PROFILE
    QosProfile                                                             = 4204,
    /// QUEUE-LENGTH
    QueueLength                                                            = 909,
    /// QUEUE-SIZE
    QueueSize                                                              = 1398,
    /// QUEUED-RECEIVER-COM-SPEC
    QueuedReceiverComSpec                                                  = 1991,
    /// QUEUED-SENDER-COM-SPEC
    QueuedSenderComSpec                                                    = 867,
    /// R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    RModeGroupInAtomicSwcInstanceRef                                       = 3002,
    /// R-PORT-IN-COMPOSITION-INSTANCE-REF
    RPortInCompositionInstanceRef                                          = 1583,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 6126,
    /// R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    RPortPrototypeInExecutableIref                                         = 47,
    /// R-PORT-PROTOTYPE-IREF
    RPortPrototypeIref                                                     = 3022,
    /// R-PORT-PROTOTYPE-PROPS
    RPortPrototypeProps                                                    = 1133,
    /// R-PORT-PROTOTYPE-REF
    RPortPrototypeRef                                                      = 1382,
    /// R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF
    RTriggerInAtomicSwcInstanceRef                                         = 1447,
    /// RAM-BLOCK
    RamBlock                                                               = 2449,
    /// RAM-BLOCK-INIT-VALUE
    RamBlockInitValue                                                      = 5764,
    /// RAM-BLOCK-STATUS-CONTROL
    RamBlockStatusControl                                                  = 2578,
    /// RAM-LOCATION-REF
    RamLocationRef                                                         = 2490,
    /// RANGE-MAX
    RangeMax                                                               = 5092,
    /// RANGE-MIN
    RangeMin                                                               = 1400,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 1295,
    /// RATE-BASED-MONITORED-EVENT-REF
    RateBasedMonitoredEventRef                                             = 435,
    /// RATE-CORRECTION-MEASUREMENT-DURATION
    RateCorrectionMeasurementDuration                                      = 220,
    /// RATE-CORRECTION-OUT
    RateCorrectionOut                                                      = 2117,
    /// RATE-CORRECTIONS-PER-MEASUREMENT-DURATION
    RateCorrectionsPerMeasurementDuration                                  = 476,
    /// RATE-DEVIATION-MEASUREMENT-DURATION
    RateDeviationMeasurementDuration                                       = 1774,
    /// RATE-LIMITATION-FILTERS
    RateLimitationFilters                                                  = 4051,
    /// RATE-POLICYS
    RatePolicys                                                            = 6158,
    /// RATIO-KIND
    RatioKind                                                              = 4441,
    /// RATIONALE
    Rationale                                                              = 1621,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 1661,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 4980,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 3654,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 6167,
    /// RAW-DATA-STREAM-INTERFACE-REF
    RawDataStreamInterfaceRef                                              = 2351,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 5888,
    /// RAW-DATA-STREAM-REF
    RawDataStreamRef                                                       = 1679,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 73,
    /// REACTION-CODE
    ReactionCode                                                           = 998,
    /// READ
    Read                                                                   = 1074,
    /// READ-CLASS-REF
    ReadClassRef                                                           = 90,
    /// READ-DATA-CLASS-REF
    ReadDataClassRef                                                       = 1427,
    /// READ-DTC-INFORMATION-CLASS-REF
    ReadDtcInformationClassRef                                             = 5242,
    /// READ-LOCAL-VARIABLES
    ReadLocalVariables                                                     = 4627,
    /// READ-NV-DATA
    ReadNvData                                                             = 478,
    /// READ-SCALING-DATA-CLASS-REF
    ReadScalingDataClassRef                                                = 4227,
    /// READER-ENTITY-ID
    ReaderEntityId                                                         = 4139,
    /// READONLY
    Readonly                                                               = 3790,
    /// REASON
    Reason                                                                 = 5538,
    /// RECEIVE-ACTIVITY
    ReceiveActivity                                                        = 4742,
    /// RECEIVE-WINDOW-MIN
    ReceiveWindowMin                                                       = 329,
    /// RECEIVED-DATA-REF
    ReceivedDataRef                                                        = 6234,
    /// RECEIVER-ANNOTATION
    ReceiverAnnotation                                                     = 2278,
    /// RECEIVER-CAPABILITY
    ReceiverCapability                                                     = 3551,
    /// RECEIVER-INTENT
    ReceiverIntent                                                         = 5856,
    /// RECEIVER-IREF
    ReceiverIref                                                           = 5917,
    /// RECEIVER-IREFS
    ReceiverIrefs                                                          = 6205,
    /// RECEIVER-REF
    ReceiverRef                                                            = 5792,
    /// RECEIVER-REFS
    ReceiverRefs                                                           = 1282,
    /// RECEPTION-POLICYS
    ReceptionPolicys                                                       = 601,
    /// RECEPTION-PROPS
    ReceptionProps                                                         = 5424,
    /// RECOMMENDED-CONFIGURATION-REF
    RecommendedConfigurationRef                                            = 4027,
    /// RECOMMENDED-CONFIGURATION-REFS
    RecommendedConfigurationRefs                                           = 5564,
    /// RECORD-ELEMENT-MAPPINGS
    RecordElementMappings                                                  = 1069,
    /// RECORD-ELEMENT-REF
    RecordElementRef                                                       = 4001,
    /// RECORD-ELEMENTS
    RecordElements                                                         = 936,
    /// RECORD-NUMBER
    RecordNumber                                                           = 5110,
    /// RECORD-VALUE-SPECIFICATION
    RecordValueSpecification                                               = 2018,
    /// RECOVERABLE-IN-SAME-OPERATION-CYCLE
    RecoverableInSameOperationCycle                                        = 63,
    /// RECOVERY
    Recovery                                                               = 943,
    /// RECOVERY-ACTION-IREF
    RecoveryActionIref                                                     = 727,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 5041,
    /// RECOVERY-NOTIFICATION-REF
    RecoveryNotificationRef                                                = 4420,
    /// RECOVERY-NOTIFICATION-REFS
    RecoveryNotificationRefs                                               = 106,
    /// RECOVERY-NOTIFICATION-RETRY
    RecoveryNotificationRetry                                              = 1756,
    /// RECOVERY-NOTIFICATION-TIMEOUT
    RecoveryNotificationTimeout                                            = 3783,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 1135,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 3614,
    /// RECOVERY-VIA-APPLICATION-ACTION-REF
    RecoveryViaApplicationActionRef                                        = 4330,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 6228,
    /// REDUNDANCY
    Redundancy                                                             = 1899,
    /// REDUNDANCY-HANDLINGS
    RedundancyHandlings                                                    = 5598,
    /// REENTRANCY-LEVEL
    ReentrancyLevel                                                        = 5276,
    /// REF
    Ref                                                                    = 2949,
    /// REF-CALPRM-SET
    RefCalprmSet                                                           = 3021,
    /// REF-MEASUREMENT-SET
    RefMeasurementSet                                                      = 5211,
    /// REFERENCE-BASE
    ReferenceBase                                                          = 4711,
    /// REFERENCE-BASES
    ReferenceBases                                                         = 5972,
    /// REFERENCE-CONDITION
    ReferenceCondition                                                     = 6186,
    /// REFERENCE-REF
    ReferenceRef                                                           = 1066,
    /// REFERENCE-TABLE-REF
    ReferenceTableRef                                                      = 759,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 641,
    /// REFERENCE-VALUE-REF
    ReferenceValueRef                                                      = 2229,
    /// REFERENCE-VALUE-SPECIFICATION
    ReferenceValueSpecification                                            = 2801,
    /// REFERENCE-VALUES
    ReferenceValues                                                        = 2277,
    /// REFERENCED-BASELINE
    ReferencedBaseline                                                     = 4810,
    /// REFERENCED-TD-EVENT-SWC-REF
    ReferencedTdEventSwcRef                                                = 216,
    /// REFERENCED-TD-EVENT-VFB-REF
    ReferencedTdEventVfbRef                                                = 5909,
    /// REFERENCES
    References                                                             = 3045,
    /// REFERRABLE-REF
    ReferrableRef                                                          = 3715,
    /// REFILL-AMOUNT
    RefillAmount                                                           = 36,
    /// REFINED-EVENT-IREF
    RefinedEventIref                                                       = 3078,
    /// REFINED-MODULE-DEF-REF
    RefinedModuleDefRef                                                    = 3808,
    /// REG-DOES-NOT-REQUIRE-STABILITYS
    RegDoesNotRequireStabilitys                                            = 3537,
    /// REG-REQUIRES-STABILITYS
    RegRequiresStabilitys                                                  = 2239,
    /// REGENERATED-PRIORITY
    RegeneratedPriority                                                    = 2350,
    /// REGULAR-EXPRESSION
    RegularExpression                                                      = 1620,
    /// REKEY-INTERVAL
    RekeyInterval                                                          = 1730,
    /// RELATED-TRACE-ITEM-REF
    RelatedTraceItemRef                                                    = 2024,
    /// RELATIONS
    Relations                                                              = 1879,
    /// RELATIVE
    Relative                                                               = 1584,
    /// RELATIVE-REPRESENTATION-TIME
    RelativeRepresentationTime                                             = 772,
    /// RELATIVE-TOLERANCE
    RelativeTolerance                                                      = 2201,
    /// RELEASE-NOTES-REF
    ReleaseNotesRef                                                        = 5319,
    /// RELEASED-TRIGGER-POLICYS
    ReleasedTriggerPolicys                                                 = 2686,
    /// RELEASED-TRIGGER-REF
    ReleasedTriggerRef                                                     = 5929,
    /// RELEASED-TRIGGERS
    ReleasedTriggers                                                       = 518,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF
    RelevantForDynamicPncMappingRef                                        = 4149,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS
    RelevantForDynamicPncMappingRefs                                       = 3495,
    /// RELIABILITY
    Reliability                                                            = 41,
    /// RELIABILITY-KIND
    ReliabilityKind                                                        = 1873,
    /// RELIABILITY-MAX-BLOCKING-TIME
    ReliabilityMaxBlockingTime                                             = 3383,
    /// REMAPPED-CAN-ID
    RemappedCanId                                                          = 5030,
    /// REMARK
    Remark                                                                 = 6146,
    /// REMOTE-ACCESS-CONTROL-ENABLED
    RemoteAccessControlEnabled                                             = 4677,
    /// REMOTE-ADDRESSS
    RemoteAddresss                                                         = 4647,
    /// REMOTE-CERTIFICATE-REF
    RemoteCertificateRef                                                   = 2746,
    /// REMOTE-CERTIFICATE-REFS
    RemoteCertificateRefs                                                  = 2777,
    /// REMOTE-CLIENT-CONFIG
    RemoteClientConfig                                                     = 1839,
    /// REMOTE-ID
    RemoteId                                                               = 2703,
    /// REMOTE-IP-ADDRESS-REF
    RemoteIpAddressRef                                                     = 3154,
    /// REMOTE-IP-ADDRESS-REFS
    RemoteIpAddressRefs                                                    = 3012,
    /// REMOTE-MULTICAST-CONFIG-REF
    RemoteMulticastConfigRef                                               = 5150,
    /// REMOTE-MULTICAST-CONFIG-REFS
    RemoteMulticastConfigRefs                                              = 1660,
    /// REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS
    RemoteMulticastSubscriptionAddresss                                    = 2090,
    /// REMOTE-NETWORK-ENDPOINT-REF
    RemoteNetworkEndpointRef                                               = 1356,
    /// REMOTE-PORT-RANGE-END
    RemotePortRangeEnd                                                     = 5130,
    /// REMOTE-PORT-RANGE-START
    RemotePortRangeStart                                                   = 5483,
    /// REMOTE-PORT-REF
    RemotePortRef                                                          = 1383,
    /// REMOTE-SERVER-CONFIG
    RemoteServerConfig                                                     = 3871,
    /// REMOTE-SUBJECT-REF
    RemoteSubjectRef                                                       = 690,
    /// REMOTE-SUBJECT-REFS
    RemoteSubjectRefs                                                      = 4460,
    /// REMOTE-UNICAST-ADDRESSS
    RemoteUnicastAddresss                                                  = 6232,
    /// REMOTE-UNICAST-CONFIG-REF
    RemoteUnicastConfigRef                                                 = 1360,
    /// REMOTE-UNICAST-CONFIG-REFS
    RemoteUnicastConfigRefs                                                = 3304,
    /// REMOTING-TECHNOLOGY
    RemotingTechnology                                                     = 4827,
    /// REPETITION-PERIOD
    RepetitionPeriod                                                       = 2671,
    /// REPLACE-WITH
    ReplaceWith                                                            = 191,
    /// REPLAY-PROTECTION
    ReplayProtection                                                       = 3007,
    /// REPLAY-PROTECTION-WINDOW
    ReplayProtectionWindow                                                 = 4583,
    /// REPLICA-PDUS-REF
    ReplicaPdusRef                                                         = 2539,
    /// REPLICA-PDUS-REFS
    ReplicaPdusRefs                                                        = 400,
    /// REPLY-TOPIC-NAME
    ReplyTopicName                                                         = 2200,
    /// REPORT-BEHAVIOR
    ReportBehavior                                                         = 2388,
    /// REPORTABLE-SECURITY-EVENT-REF
    ReportableSecurityEventRef                                             = 2447,
    /// REPORTABLE-SECURITY-EVENT-REFS
    ReportableSecurityEventRefs                                            = 5945,
    /// REPORTED-SECURITY-EVENT-IREF
    ReportedSecurityEventIref                                              = 3889,
    /// REPORTING-BEHAVIOR
    ReportingBehavior                                                      = 4466,
    /// REPORTING-PORT-PROTOTYPE-IREF
    ReportingPortPrototypeIref                                             = 6251,
    /// REPOSITORY
    Repository                                                             = 4666,
    /// REPRESENTED-MODEL-ELEMENT-REF
    RepresentedModelElementRef                                             = 2641,
    /// REPRESENTED-PORT-GROUP-REF
    RepresentedPortGroupRef                                                = 3990,
    /// REPRESENTS-VIN
    RepresentsVin                                                          = 1188,
    /// REQUEST-2-SUPPORT
    Request2Support                                                        = 4402,
    /// REQUEST-CONFIGURATIONS
    RequestConfigurations                                                  = 2058,
    /// REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF
    RequestControlOfOnBoardDeviceClassRef                                  = 3480,
    /// REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF
    RequestCurrentPowertrainDiagnosticDataClassRef                         = 4548,
    /// REQUEST-DATA-SIZE
    RequestDataSize                                                        = 4008,
    /// REQUEST-DOWNLOAD-CLASS-REF
    RequestDownloadClassRef                                                = 4436,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF
    RequestEmissionRelatedDtcClassPermanentStatusRef                       = 150,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-REF
    RequestEmissionRelatedDtcClassRef                                      = 3265,
    /// REQUEST-FILE-TRANSFER-CLASS-REF
    RequestFileTransferClassRef                                            = 1609,
    /// REQUEST-GROUP-REF
    RequestGroupRef                                                        = 3055,
    /// REQUEST-METHOD
    RequestMethod                                                          = 2403,
    /// REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF
    RequestOnBoardMonitoringTestResultsClassRef                            = 1479,
    /// REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF
    RequestPowertrainFreezeFrameDataRef                                    = 5569,
    /// REQUEST-RESPONSE-DELAY
    RequestResponseDelay                                                   = 1986,
    /// REQUEST-RESULT
    RequestResult                                                          = 3124,
    /// REQUEST-SEED-ID
    RequestSeedId                                                          = 3084,
    /// REQUEST-TOPIC-NAME
    RequestTopicName                                                       = 3635,
    /// REQUEST-TYPE
    RequestType                                                            = 2184,
    /// REQUEST-UPLOAD-CLASS-REF
    RequestUploadClassRef                                                  = 3411,
    /// REQUEST-VEHICLE-INFORMATION-CLASS-REF
    RequestVehicleInformationClassRef                                      = 5520,
    /// REQUESTABLE
    Requestable                                                            = 3729,
    /// REQUESTED-MODE-IREF
    RequestedModeIref                                                      = 4507,
    /// REQUESTER-IREF
    RequesterIref                                                          = 4778,
    /// REQUESTER-REF
    RequesterRef                                                           = 3747,
    /// REQUESTER-REFS
    RequesterRefs                                                          = 633,
    /// REQUESTER-SOFTWARE-CLUSTER-REF
    RequesterSoftwareClusterRef                                            = 5689,
    /// REQUESTOR-REF
    RequestorRef                                                           = 510,
    /// REQUESTOR-REFS
    RequestorRefs                                                          = 4831,
    /// REQUESTS
    Requests                                                               = 2804,
    /// REQUIRE-RESOURCES
    RequireResources                                                       = 3945,
    /// REQUIRED-AR-ELEMENT-REF
    RequiredArElementRef                                                   = 4252,
    /// REQUIRED-AR-ELEMENT-REFS
    RequiredArElementRefs                                                  = 3426,
    /// REQUIRED-ARTIFACTS
    RequiredArtifacts                                                      = 1436,
    /// REQUIRED-CLIENT-SERVER-ENTRY-REF
    RequiredClientServerEntryRef                                           = 2670,
    /// REQUIRED-CLIENT-SERVER-ENTRYS
    RequiredClientServerEntrys                                             = 1508,
    /// REQUIRED-COM-SPECS
    RequiredComSpecs                                                       = 4097,
    /// REQUIRED-DATAS
    RequiredDatas                                                          = 2066,
    /// REQUIRED-DDS-EVENT-QOS-PROPS
    RequiredDdsEventQosProps                                               = 4208,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 3207,
    /// REQUIRED-DESIGN-ELEMENT-REF
    RequiredDesignElementRef                                               = 2195,
    /// REQUIRED-DESIGN-ELEMENT-REFS
    RequiredDesignElementRefs                                              = 2792,
    /// REQUIRED-ENTRYS
    RequiredEntrys                                                         = 2467,
    /// REQUIRED-ENVIRONMENT-REF
    RequiredEnvironmentRef                                                 = 1558,
    /// REQUIRED-EVENT-GROUPS
    RequiredEventGroups                                                    = 5438,
    /// REQUIRED-FIBEX-ELEMENT-REF
    RequiredFibexElementRef                                                = 4675,
    /// REQUIRED-FIBEX-ELEMENT-REFS
    RequiredFibexElementRefs                                               = 1963,
    /// REQUIRED-GENERATOR-TOOLS
    RequiredGeneratorTools                                                 = 4057,
    /// REQUIRED-INTERFACE-TREF
    RequiredInterfaceTref                                                  = 4085,
    /// REQUIRED-MINOR-VERSION
    RequiredMinorVersion                                                   = 302,
    /// REQUIRED-MODE-GROUP-REF
    RequiredModeGroupRef                                                   = 70,
    /// REQUIRED-MODE-GROUPS
    RequiredModeGroups                                                     = 561,
    /// REQUIRED-OUTER-PORT-REF
    RequiredOuterPortRef                                                   = 298,
    /// REQUIRED-PACKAGE-ELEMENT-REF
    RequiredPackageElementRef                                              = 5409,
    /// REQUIRED-PACKAGE-ELEMENT-REFS
    RequiredPackageElementRefs                                             = 1692,
    /// REQUIRED-PORT-PROTOTYPE-IREF
    RequiredPortPrototypeIref                                              = 568,
    /// REQUIRED-RTE-VENDOR
    RequiredRteVendor                                                      = 1477,
    /// REQUIRED-SERVICE-INSTANCE-ID
    RequiredServiceInstanceId                                              = 3075,
    /// REQUIRED-SERVICE-INSTANCE-REF
    RequiredServiceInstanceRef                                             = 5171,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 4680,
    /// REQUIRED-SERVICE-PORT-IREF
    RequiredServicePortIref                                                = 468,
    /// REQUIRED-SERVICE-VERSION
    RequiredServiceVersion                                                 = 1836,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 810,
    /// REQUIRED-TRIGGER-IREF
    RequiredTriggerIref                                                    = 4400,
    /// REQUIRED-TRIGGERS
    RequiredTriggers                                                       = 960,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 2445,
    /// REQUIREMENT-REF
    RequirementRef                                                         = 534,
    /// REQUIREMENT-REFS
    RequirementRefs                                                        = 1732,
    /// REQUIRES-INDEX
    RequiresIndex                                                          = 1232,
    /// REQUIRES-SYMBOLIC-NAME-VALUE
    RequiresSymbolicNameValue                                              = 2768,
    /// RESET-CONFIRMED-BIT-ON-OVERFLOW
    ResetConfirmedBitOnOverflow                                            = 2272,
    /// RESET-PENDING-BIT-ON-OVERFLOW
    ResetPendingBitOnOverflow                                              = 2151,
    /// RESET-TO-DEFAULT
    ResetToDefault                                                         = 78,
    /// RESET-TO-DEFAULT-SUPPORTED
    ResetToDefaultSupported                                                = 125,
    /// RESISTANT-TO-CHANGED-SW
    ResistantToChangedSw                                                   = 4470,
    /// RESOLUTION
    Resolution                                                             = 5116,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 3674,
    /// RESOURCE-DEFINITION-REF
    ResourceDefinitionRef                                                  = 5605,
    /// RESOURCE-DEFINITIONS
    ResourceDefinitions                                                    = 3105,
    /// RESOURCE-ESTIMATIONS
    ResourceEstimations                                                    = 2862,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 5573,
    /// RESOURCE-GROUP-REF
    ResourceGroupRef                                                       = 5783,
    /// RESOURCE-GROUP-REFS
    ResourceGroupRefs                                                      = 5641,
    /// RESOURCE-GROUPS
    ResourceGroups                                                         = 2110,
    /// RESOURCE-GUARD-VALUE
    ResourceGuardValue                                                     = 1547,
    /// RESOURCE-IDENTIFIER-TYPE
    ResourceIdentifierType                                                 = 3642,
    /// RESOURCE-LIMITS
    ResourceLimits                                                         = 5254,
    /// RESOURCE-NEEDS-REF
    ResourceNeedsRef                                                       = 1678,
    /// RESOURCE-NEEDS-REFS
    ResourceNeedsRefs                                                      = 672,
    /// RESOURCE-REF
    ResourceRef                                                            = 3603,
    /// RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS
    ResourceToApplicationPartitionMappings                                 = 762,
    /// RESOURCES
    Resources                                                              = 1572,
    /// RESPOND-TO-RESET
    RespondToReset                                                         = 2028,
    /// RESPONSE-DATA-SIZE
    ResponseDataSize                                                       = 1668,
    /// RESPONSE-ERROR-POSITION
    ResponseErrorPosition                                                  = 484,
    /// RESPONSE-ERROR-REF
    ResponseErrorRef                                                       = 2187,
    /// RESPONSE-GROUP-REF
    ResponseGroupRef                                                       = 4473,
    /// RESPONSE-ON-ALL-REQUEST-SIDS
    ResponseOnAllRequestSids                                               = 4249,
    /// RESPONSE-ON-EVENT-ACTION
    ResponseOnEventAction                                                  = 6235,
    /// RESPONSE-ON-EVENT-CLASS-REF
    ResponseOnEventClassRef                                                = 5743,
    /// RESPONSE-ON-EVENT-REF
    ResponseOnEventRef                                                     = 5582,
    /// RESPONSE-ON-EVENT-SCHEDULER-RATE
    ResponseOnEventSchedulerRate                                           = 3512,
    /// RESPONSE-ON-SECOND-DECLINED-REQUEST
    ResponseOnSecondDeclinedRequest                                        = 1385,
    /// RESPONSE-REF
    ResponseRef                                                            = 4255,
    /// RESPONSES
    Responses                                                              = 2930,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 2078,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 5997,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 2319,
    /// REST-ENDPOINT-ARGUMENT
    RestEndpointArgument                                                   = 6214,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 994,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 5680,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 678,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 4565,
    /// REST-ENUMERATOR-DEF
    RestEnumeratorDef                                                      = 573,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 170,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 6196,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 6048,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 5794,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 4492,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 4703,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 2328,
    /// REST-SYSTEM-TRIGGERED-EVENT
    RestSystemTriggeredEvent                                               = 5678,
    /// RESTORE-AT-START
    RestoreAtStart                                                         = 5610,
    /// RESTRICT-UPDATE
    RestrictUpdate                                                         = 1402,
    /// RESTRICTION
    Restriction                                                            = 5870,
    /// RESTRICTIONS
    Restrictions                                                           = 4186,
    /// RESULTING-PROPERTIES
    ResultingProperties                                                    = 4611,
    /// RESULTING-RPT-SW-PROTOTYPING-ACCESS
    ResultingRptSwPrototypingAccess                                        = 4515,
    /// RESUME-POSITION
    ResumePosition                                                         = 4096,
    /// RETRY
    Retry                                                                  = 5120,
    /// RETRY-INTERVAL-TIME
    RetryIntervalTime                                                      = 5860,
    /// RETURN-CODE
    ReturnCode                                                             = 1878,
    /// RETURN-SIGNAL-REF
    ReturnSignalRef                                                        = 459,
    /// RETURN-SIGNAL-TRIGGERING-REF
    ReturnSignalTriggeringRef                                              = 4760,
    /// RETURN-TYPE
    ReturnType                                                             = 145,
    /// RETURN-VALUE-PROVISION
    ReturnValueProvision                                                   = 6055,
    /// REVERSED-TP-SDU-REF
    ReversedTpSduRef                                                       = 1533,
    /// REVISION-LABEL
    RevisionLabel                                                          = 997,
    /// REVISION-LABEL-P-1
    RevisionLabelP1                                                        = 919,
    /// REVISION-LABEL-P-2
    RevisionLabelP2                                                        = 983,
    /// REVISION-LABELS
    RevisionLabels                                                         = 4003,
    /// RID-NUMBER
    RidNumber                                                              = 4688,
    /// ROLE
    Role                                                                   = 5455,
    /// ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT
    RoleBasedBswModuleEntryAssignment                                      = 5277,
    /// ROLE-BASED-DATA-ASSIGNMENT
    RoleBasedDataAssignment                                                = 343,
    /// ROLE-BASED-DATA-TYPE-ASSIGNMENT
    RoleBasedDataTypeAssignment                                            = 403,
    /// ROLE-BASED-MC-DATA-ASSIGNMENT
    RoleBasedMcDataAssignment                                              = 859,
    /// ROLE-BASED-PORT-ASSIGNMENT
    RoleBasedPortAssignment                                                = 3099,
    /// ROLE-BASED-RESOURCE-DEPENDENCY
    RoleBasedResourceDependency                                            = 1693,
    /// ROLLOUT-QUALIFICATIONS
    RolloutQualifications                                                  = 730,
    /// ROM-BLOCK
    RomBlock                                                               = 12,
    /// ROM-BLOCK-INIT-VALUE
    RomBlockInitValue                                                      = 4129,
    /// ROM-LOCATION-REF
    RomLocationRef                                                         = 628,
    /// ROOT-ARGUMENT-DATA-PROTOTYPE-REF
    RootArgumentDataPrototypeRef                                           = 2549,
    /// ROOT-AUTOSAR-DATA-PROTOTYPE-REF
    RootAutosarDataPrototypeRef                                            = 2683,
    /// ROOT-COMPOSITION
    RootComposition                                                        = 890,
    /// ROOT-CONTEXT-REF
    RootContextRef                                                         = 3300,
    /// ROOT-DATA-PROTOTYPE-IN-CS-REF
    RootDataPrototypeInCsRef                                               = 4601,
    /// ROOT-DATA-PROTOTYPE-IN-SR-REF
    RootDataPrototypeInSrRef                                               = 1905,
    /// ROOT-DATA-PROTOTYPE-REF
    RootDataPrototypeRef                                                   = 1807,
    /// ROOT-PARAMETER-DATA-PROTOTYPE-REF
    RootParameterDataPrototypeRef                                          = 4325,
    /// ROOT-REF
    RootRef                                                                = 1144,
    /// ROOT-SOFTWARE-COMPOSITIONS
    RootSoftwareCompositions                                               = 2956,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 993,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 3122,
    /// ROOT-VARIABLE-DATA-PROTOTYPE-REF
    RootVariableDataPrototypeRef                                           = 5080,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 1779,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 440,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 275,
    /// ROUTINE-CONTROL-CLASS-REF
    RoutineControlClassRef                                                 = 5599,
    /// ROUTINE-INFO
    RoutineInfo                                                            = 2487,
    /// ROUTINE-REF
    RoutineRef                                                             = 6104,
    /// ROUTINE-SUBFUNCTION-REF
    RoutineSubfunctionRef                                                  = 3671,
    /// ROUTING-ACTIVATION-TYPE
    RoutingActivationType                                                  = 1275,
    /// ROUTING-GROUP-REF
    RoutingGroupRef                                                        = 2196,
    /// ROUTING-GROUP-REFS
    RoutingGroupRefs                                                       = 5816,
    /// ROW
    Row                                                                    = 3032,
    /// RP-IMPL-POLICY
    RpImplPolicy                                                           = 5833,
    /// RPT-AR-HOOK-IREF
    RptArHookIref                                                          = 3834,
    /// RPT-COMPONENT
    RptComponent                                                           = 5181,
    /// RPT-COMPONENTS
    RptComponents                                                          = 894,
    /// RPT-CONTAINER
    RptContainer                                                           = 2489,
    /// RPT-CONTAINERS
    RptContainers                                                          = 2868,
    /// RPT-ENABLER-IMPL-TYPE
    RptEnablerImplType                                                     = 2611,
    /// RPT-EVENT-ID
    RptEventId                                                             = 610,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 2121,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 4690,
    /// RPT-EXECUTABLE-ENTITY-EVENTS
    RptExecutableEntityEvents                                              = 448,
    /// RPT-EXECUTABLE-ENTITY-PROPERTIES
    RptExecutableEntityProperties                                          = 2451,
    /// RPT-EXECUTABLE-ENTITYS
    RptExecutableEntitys                                                   = 5813,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 840,
    /// RPT-EXECUTION-CONTROL
    RptExecutionControl                                                    = 4017,
    /// RPT-HOOK
    RptHook                                                                = 3741,
    /// RPT-HOOK-ACCESS
    RptHookAccess                                                          = 6276,
    /// RPT-HOOKS
    RptHooks                                                               = 6259,
    /// RPT-IMPL-POLICY
    RptImplPolicy                                                          = 4048,
    /// RPT-PREPARATION-LEVEL
    RptPreparationLevel                                                    = 2555,
    /// RPT-PROFILE
    RptProfile                                                             = 3810,
    /// RPT-PROFILES
    RptProfiles                                                            = 3591,
    /// RPT-READ-ACCESS
    RptReadAccess                                                          = 4464,
    /// RPT-READS
    RptReads                                                               = 2592,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 600,
    /// RPT-SERVICE-POINT-POST-REF
    RptServicePointPostRef                                                 = 1645,
    /// RPT-SERVICE-POINT-POST-REFS
    RptServicePointPostRefs                                                = 6175,
    /// RPT-SERVICE-POINT-PRE-REF
    RptServicePointPreRef                                                  = 4863,
    /// RPT-SERVICE-POINT-PRE-REFS
    RptServicePointPreRefs                                                 = 2422,
    /// RPT-SERVICE-POINTS
    RptServicePoints                                                       = 6050,
    /// RPT-SUPPORT-DATA
    RptSupportData                                                         = 1549,
    /// RPT-SW-PROTOTYPING-ACCESS
    RptSwPrototypingAccess                                                 = 4244,
    /// RPT-SYSTEM-REF
    RptSystemRef                                                           = 366,
    /// RPT-WRITE-ACCESS
    RptWriteAccess                                                         = 2152,
    /// RPT-WRITES
    RptWrites                                                              = 2074,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 4117,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 5796,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 4839,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 5231,
    /// RTE-EVENT-IREF
    RteEventIref                                                           = 786,
    /// RTE-EVENT-IREFS
    RteEventIrefs                                                          = 783,
    /// RTE-EVENT-SEPARATIONS
    RteEventSeparations                                                    = 5109,
    /// RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS
    RteEventToOsTaskProxyMappings                                          = 1947,
    /// RTE-PLUGIN-PROPS
    RtePluginProps                                                         = 5826,
    /// RTE-RESOURCE-ESTIMATION
    RteResourceEstimation                                                  = 4691,
    /// RTP-TP
    RtpTp                                                                  = 2166,
    /// RTPS-PROTECTION-KIND
    RtpsProtectionKind                                                     = 111,
    /// RULE
    Rule                                                                   = 4634,
    /// RULE-ARGUMENTS
    RuleArguments                                                          = 3266,
    /// RULE-BASED-AXIS-CONT
    RuleBasedAxisCont                                                      = 1399,
    /// RULE-BASED-VALUES
    RuleBasedValues                                                        = 3717,
    /// RULE-INIT-STATE
    RuleInitState                                                          = 3627,
    /// RULES
    Rules                                                                  = 3127,
    /// RUN-MODE
    RunMode                                                                = 4393,
    /// RUN-RESOURCE
    RunResource                                                            = 5547,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 2023,
    /// RUNNABLE-ENTITY-ARGUMENT
    RunnableEntityArgument                                                 = 2813,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 2901,
    /// RUNNABLE-ENTITY-GROUP-IREF
    RunnableEntityGroupIref                                                = 3371,
    /// RUNNABLE-ENTITY-GROUP-IREFS
    RunnableEntityGroupIrefs                                               = 3967,
    /// RUNNABLE-ENTITY-IREF
    RunnableEntityIref                                                     = 4121,
    /// RUNNABLE-ENTITY-IREFS
    RunnableEntityIrefs                                                    = 5842,
    /// RUNNABLE-MAPPINGS
    RunnableMappings                                                       = 5029,
    /// RUNNABLE-REF
    RunnableRef                                                            = 1262,
    /// RUNNABLES
    Runnables                                                              = 3510,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REF
    RunsInsideExclusiveAreaRef                                             = 1387,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REFS
    RunsInsideExclusiveAreaRefs                                            = 2498,
    /// RUNS-INSIDES
    RunsInsides                                                            = 354,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 1507,
    /// RUNTIME-IP-ADDRESS-CONFIGURATION
    RuntimeIpAddressConfiguration                                          = 3215,
    /// RUNTIME-PORT-CONFIGURATION
    RuntimePortConfiguration                                               = 1155,
    /// RVF-ACTIVE-PIXELS
    RvfActivePixels                                                        = 1630,
    /// RVF-COLOR-SPACE
    RvfColorSpace                                                          = 5118,
    /// RVF-EVENT-DEFAULT
    RvfEventDefault                                                        = 3816,
    /// RVF-FRAME-RATE
    RvfFrameRate                                                           = 5532,
    /// RVF-INTERLACED
    RvfInterlaced                                                          = 1257,
    /// RVF-PIXEL-DEPTH
    RvfPixelDepth                                                          = 3217,
    /// RVF-PIXEL-FORMAT
    RvfPixelFormat                                                         = 3275,
    /// RVF-TOTAL-LINES
    RvfTotalLines                                                          = 2002,
    /// RX-ACCEPT-CONTAINED-I-PDU
    RxAcceptContainedIPdu                                                  = 3276,
    /// RX-IDENTIFIER-RANGE
    RxIdentifierRange                                                      = 5585,
    /// RX-MASK
    RxMask                                                                 = 4004,
    /// RX-NM-PDU-REF
    RxNmPduRef                                                             = 3442,
    /// RX-NM-PDU-REFS
    RxNmPduRefs                                                            = 6200,
    /// RX-PDU-POOL-REF
    RxPduPoolRef                                                           = 332,
    /// RX-PDU-TRIGGERING-REF
    RxPduTriggeringRef                                                     = 1762,
    /// RX-SECURITY-VERIFICATION
    RxSecurityVerification                                                 = 4215,
    /// RX-TIMEOUT-TIME
    RxTimeoutTime                                                          = 2167,
    /// S-3-SERVER-TIMEOUT
    S3ServerTimeout                                                        = 3864,
    /// SA-OVER-TIME
    SaOverTime                                                             = 1497,
    /// SA-RAND-TIME
    SaRandTime                                                             = 6074,
    /// SA-REKEY-TIME
    SaRekeyTime                                                            = 5301,
    /// SAFE-TRANSLATION
    SafeTranslation                                                        = 3823,
    /// SAFETY-CONDITION
    SafetyCondition                                                        = 137,
    /// SAFETY-CONDITIONS
    SafetyConditions                                                       = 1093,
    /// SAFETY-MARGIN
    SafetyMargin                                                           = 5876,
    /// SAFETY-POLICY
    SafetyPolicy                                                           = 2875,
    /// SAK-REF
    SakRef                                                                 = 5719,
    /// SAK-REKEY-TIME-SPAN
    SakRekeyTimeSpan                                                       = 3899,
    /// SAMPLE-CLOCK-PERIOD
    SampleClockPeriod                                                      = 184,
    /// SAMPLES-PER-MICROTICK
    SamplesPerMicrotick                                                    = 1438,
    /// SAVE-CONFIGURATION
    SaveConfiguration                                                      = 2791,
    /// SAVE-CONFIGURATION-ENTRY
    SaveConfigurationEntry                                                 = 3164,
    /// SCALE-CONSTR
    ScaleConstr                                                            = 4538,
    /// SCALE-CONSTRS
    ScaleConstrs                                                           = 5486,
    /// SCALING-INFO-SIZE
    ScalingInfoSize                                                        = 4047,
    /// SCHEDULE-CHANGE-NEXT-TIME-BASE
    ScheduleChangeNextTimeBase                                             = 1317,
    /// SCHEDULE-TABLES
    ScheduleTables                                                         = 5653,
    /// SCHEDULER-MAX-NUMBER
    SchedulerMaxNumber                                                     = 4354,
    /// SCHEDULER-NAME-PREFIX-REF
    SchedulerNamePrefixRef                                                 = 1869,
    /// SCHEDULER-NAME-PREFIXS
    SchedulerNamePrefixs                                                   = 6018,
    /// SCHEDULING-POLICY
    SchedulingPolicy                                                       = 4853,
    /// SCHEDULING-PRIORITY
    SchedulingPriority                                                     = 4430,
    /// SCOPE
    Scope                                                                  = 5262,
    /// SCOPE-EVENT-REF
    ScopeEventRef                                                          = 120,
    /// SCOPE-EVENT-REFS
    ScopeEventRefs                                                         = 3581,
    /// SCOPE-REF
    ScopeRef                                                               = 4637,
    /// SCOPE-REFS
    ScopeRefs                                                              = 2808,
    /// SD
    Sd                                                                     = 5307,
    /// SD-CLIENT-CONFIG
    SdClientConfig                                                         = 6042,
    /// SD-CLIENT-CONFIG-REF
    SdClientConfigRef                                                      = 6054,
    /// SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SdClientEventGroupTimingConfigRef                                      = 2821,
    /// SD-CLIENT-EVENT-TIMING-CONFIG
    SdClientEventTimingConfig                                              = 3517,
    /// SD-CLIENT-TIMER-CONFIGS
    SdClientTimerConfigs                                                   = 2228,
    /// SD-SERVER-CONFIG
    SdServerConfig                                                         = 1426,
    /// SD-SERVER-CONFIG-REF
    SdServerConfigRef                                                      = 288,
    /// SD-SERVER-EG-TIMING-CONFIGS
    SdServerEgTimingConfigs                                                = 1353,
    /// SD-SERVER-EVENT-CONFIG
    SdServerEventConfig                                                    = 2242,
    /// SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SdServerEventGroupTimingConfigRef                                      = 4156,
    /// SD-SERVER-TIMER-CONFIGS
    SdServerTimerConfigs                                                   = 5648,
    /// SDF
    Sdf                                                                    = 5772,
    /// SDG
    Sdg                                                                    = 1184,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 6029,
    /// SDG-CAPTION
    SdgCaption                                                             = 887,
    /// SDG-CAPTION-REF
    SdgCaptionRef                                                          = 3857,
    /// SDG-CLASS
    SdgClass                                                               = 2434,
    /// SDG-CLASS-REF
    SdgClassRef                                                            = 52,
    /// SDG-CLASSES
    SdgClasses                                                             = 1801,
    /// SDG-CONSTRAINT-REF
    SdgConstraintRef                                                       = 2273,
    /// SDG-CONSTRAINT-REFS
    SdgConstraintRefs                                                      = 1628,
    /// SDG-DEF
    SdgDef                                                                 = 1256,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 3448,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 6109,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 281,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 6152,
    /// SDG-REFERENCE
    SdgReference                                                           = 1828,
    /// SDG-TAILORING
    SdgTailoring                                                           = 2889,
    /// SDG-TAILORINGS
    SdgTailorings                                                          = 1163,
    /// SDGS
    Sdgs                                                                   = 1659,
    /// SDU-REF
    SduRef                                                                 = 1763,
    /// SDU-REFS
    SduRefs                                                                = 1243,
    /// SDU-TYPE
    SduType                                                                = 309,
    /// SDX-REF
    SdxRef                                                                 = 4376,
    /// SDXF
    Sdxf                                                                   = 173,
    /// SEARCH-BEHAVIOR
    SearchBehavior                                                         = 4258,
    /// SEARCH-INTENTION
    SearchIntention                                                        = 4462,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REF
    SecOcComPropsForMulticastRef                                           = 3821,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REFS
    SecOcComPropsForMulticastRefs                                          = 2858,
    /// SEC-OC-CRYPTO-MAPPING-REF
    SecOcCryptoMappingRef                                                  = 1487,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 2485,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 4924,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 2575,
    /// SEC-OC-JOB-MAPPINGS
    SecOcJobMappings                                                       = 1711,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 4137,
    /// SEC-OC-JOB-REQUIREMENT-REF
    SecOcJobRequirementRef                                                 = 2206,
    /// SEC-OC-JOB-SEMANTIC
    SecOcJobSemantic                                                       = 4649,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 4988,
    /// SECOND-APPLICATION-ERROR-REF
    SecondApplicationErrorRef                                              = 5264,
    /// SECOND-DATA-PROTOTYPE-REF
    SecondDataPrototypeRef                                                 = 985,
    /// SECOND-ELEMENT-INSTANCE-IREF
    SecondElementInstanceIref                                              = 4164,
    /// SECOND-ELEMENT-INSTANCE-IREFS
    SecondElementInstanceIrefs                                             = 527,
    /// SECOND-ELEMENT-REF
    SecondElementRef                                                       = 5505,
    /// SECOND-ELEMENT-REFS
    SecondElementRefs                                                      = 3389,
    /// SECOND-ELEMENTS
    SecondElements                                                         = 4628,
    /// SECOND-KEY-SLOT-ID
    SecondKeySlotId                                                        = 3254,
    /// SECOND-MODE-GROUP-REF
    SecondModeGroupRef                                                     = 924,
    /// SECOND-MODE-REF
    SecondModeRef                                                          = 704,
    /// SECOND-OPERATION-REF
    SecondOperationRef                                                     = 5421,
    /// SECOND-PHYSICAL-DIMENSION-REF
    SecondPhysicalDimensionRef                                             = 1909,
    /// SECOND-PORT-REF
    SecondPortRef                                                          = 1465,
    /// SECOND-TO-FIRST-DATA-TRANSFORMATION-REF
    SecondToFirstDataTransformationRef                                     = 2887,
    /// SECOND-TRIGGER-REF
    SecondTriggerRef                                                       = 1483,
    /// SECOND-VALUE
    SecondValue                                                            = 1694,
    /// SECONDARY-FRESHNESS-VALUE-ID
    SecondaryFreshnessValueId                                              = 249,
    /// SECTION-INITIALIZATION-POLICY
    SectionInitializationPolicy                                            = 3454,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 2726,
    /// SECTION-NAME-PREFIXS
    SectionNamePrefixs                                                     = 598,
    /// SECTION-TYPE
    SectionType                                                            = 6064,
    /// SECURE-COM-CONFIG-REF
    SecureComConfigRef                                                     = 5494,
    /// SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING
    SecureComConfigToCryptoKeySlotMapping                                  = 2546,
    /// SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS
    SecureComConfigToKeySlotMappings                                       = 338,
    /// SECURE-COM-CONFIGS
    SecureComConfigs                                                       = 5754,
    /// SECURE-COM-PROPS-FOR-DDS-REF
    SecureComPropsForDdsRef                                                = 5288,
    /// SECURE-COM-PROPS-FOR-TCP-REF
    SecureComPropsForTcpRef                                                = 5823,
    /// SECURE-COM-PROPS-FOR-TCP-REFS
    SecureComPropsForTcpRefs                                               = 439,
    /// SECURE-COM-PROPS-FOR-UDP-REF
    SecureComPropsForUdpRef                                                = 6058,
    /// SECURE-COM-PROPS-FOR-UDP-REFS
    SecureComPropsForUdpRefs                                               = 5314,
    /// SECURE-COM-PROPS-REF
    SecureComPropsRef                                                      = 4963,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 5456,
    /// SECURE-COM-PROPSS
    SecureComPropss                                                        = 5230,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 1809,
    /// SECURE-COMMUNICATION-DEPLOYMENTS
    SecureCommunicationDeployments                                         = 986,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 5460,
    /// SECURE-COMMUNICATION-PROPS
    SecureCommunicationProps                                               = 1277,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 3204,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 3981,
    /// SECURE-PROPS-FOR-EXTERNAL-COMM-REF
    SecurePropsForExternalCommRef                                          = 4210,
    /// SECURE-PROPS-FOR-TCP-REF
    SecurePropsForTcpRef                                                   = 5407,
    /// SECURE-PROPS-FOR-UDP-REF
    SecurePropsForUdpRef                                                   = 778,
    /// SECURE-TRANSLATION
    SecureTranslation                                                      = 1048,
    /// SECURED-AREA-LENGTH
    SecuredAreaLength                                                      = 5576,
    /// SECURED-AREA-OFFSET
    SecuredAreaOffset                                                      = 4809,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 2331,
    /// SECURED-RX-VERIFICATION
    SecuredRxVerification                                                  = 511,
    /// SECURITY-ACCESS-CLASS-REF
    SecurityAccessClassRef                                                 = 3778,
    /// SECURITY-ACCESS-LEVEL
    SecurityAccessLevel                                                    = 1005,
    /// SECURITY-DELAY-TIME
    SecurityDelayTime                                                      = 688,
    /// SECURITY-DELAY-TIME-ON-BOOT
    SecurityDelayTimeOnBoot                                                = 1264,
    /// SECURITY-EVENT-CONTEXT-DATA
    SecurityEventContextData                                               = 3308,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 995,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 3334,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 6272,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 401,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 6269,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 4695,
    /// SECURITY-EVENT-DEFINITION-REF
    SecurityEventDefinitionRef                                             = 3149,
    /// SECURITY-EVENT-DEFINITION-REF-CONDITIONAL
    SecurityEventDefinitionRefConditional                                  = 3468,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 6053,
    /// SECURITY-EVENT-FILTER-CHAIN-REF
    SecurityEventFilterChainRef                                            = 2243,
    /// SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL
    SecurityEventFilterChainRefConditional                                 = 5965,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 455,
    /// SECURITY-EVENT-PROPS-REF
    SecurityEventPropsRef                                                  = 952,
    /// SECURITY-EVENT-REF
    SecurityEventRef                                                       = 5749,
    /// SECURITY-EVENT-REFS
    SecurityEventRefs                                                      = 5995,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 2667,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 1930,
    /// SECURITY-EVENTS
    SecurityEvents                                                         = 388,
    /// SECURITY-LEVEL-REF
    SecurityLevelRef                                                       = 5770,
    /// SECURITY-LEVEL-REFS
    SecurityLevelRefs                                                      = 2704,
    /// SEED-SIZE
    SeedSize                                                               = 5401,
    /// SEGMENT-BYTE-ORDER
    SegmentByteOrder                                                       = 4514,
    /// SEGMENT-LENGTH
    SegmentLength                                                          = 1172,
    /// SEGMENT-POSITION
    SegmentPosition                                                        = 4990,
    /// SEGMENT-POSITIONS
    SegmentPositions                                                       = 4041,
    /// SEGMENT-REF
    SegmentRef                                                             = 3262,
    /// SEGMENT-REFS
    SegmentRefs                                                            = 4098,
    /// SEGMENTATION-SUPPORTED
    SegmentationSupported                                                  = 99,
    /// SELECT-BLOCK-FOR-FIRST-INIT-ALL
    SelectBlockForFirstInitAll                                             = 649,
    /// SELECTIONS
    Selections                                                             = 3067,
    /// SELECTOR-FIELD-BYTE-ORDER
    SelectorFieldByteOrder                                                 = 5750,
    /// SELECTOR-FIELD-CODE
    SelectorFieldCode                                                      = 355,
    /// SELECTOR-FIELD-LENGTH
    SelectorFieldLength                                                    = 3968,
    /// SELECTOR-FIELD-START-POSITION
    SelectorFieldStartPosition                                             = 3881,
    /// SEND-ACTIVITY
    SendActivity                                                           = 131,
    /// SEND-INDICATION
    SendIndication                                                         = 5563,
    /// SEND-POLICYS
    SendPolicys                                                            = 2725,
    /// SEND-RESP-PEND-ON-TRANS-TO-BOOT
    SendRespPendOnTransToBoot                                              = 3748,
    /// SENDER-ANNOTATION
    SenderAnnotation                                                       = 5613,
    /// SENDER-CAPABILITY
    SenderCapability                                                       = 4608,
    /// SENDER-INTENT
    SenderIntent                                                           = 5253,
    /// SENDER-IREF
    SenderIref                                                             = 3965,
    /// SENDER-REC-ARRAY-ELEMENT-MAPPING
    SenderRecArrayElementMapping                                           = 2586,
    /// SENDER-REC-ARRAY-TYPE-MAPPING
    SenderRecArrayTypeMapping                                              = 4321,
    /// SENDER-REC-RECORD-ELEMENT-MAPPING
    SenderRecRecordElementMapping                                          = 3384,
    /// SENDER-REC-RECORD-TYPE-MAPPING
    SenderRecRecordTypeMapping                                             = 2518,
    /// SENDER-RECEIVER-ANNOTATIONS
    SenderReceiverAnnotations                                              = 1534,
    /// SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING
    SenderReceiverCompositeElementToSignalMapping                          = 6265,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 5603,
    /// SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING
    SenderReceiverToSignalGroupMapping                                     = 86,
    /// SENDER-RECEIVER-TO-SIGNAL-MAPPING
    SenderReceiverToSignalMapping                                          = 3070,
    /// SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING
    SenderToSignalTextTableMapping                                         = 5875,
    /// SENDING-NODE-REF
    SendingNodeRef                                                         = 2644,
    /// SENDING-NODE-REFS
    SendingNodeRefs                                                        = 2651,
    /// SENSOR-ACTUATOR-REF
    SensorActuatorRef                                                      = 2010,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 3858,
    /// SENSOR-INSTANCE-ID
    SensorInstanceId                                                       = 585,
    /// SEPARATE-SIGNAL-PATH
    SeparateSignalPath                                                     = 3574,
    /// SEPARATED-COMPONENT-IREF
    SeparatedComponentIref                                                 = 5702,
    /// SEPARATED-COMPONENT-IREFS
    SeparatedComponentIrefs                                                = 3640,
    /// SEPARATION-CYCLE-EXPONENT
    SeparationCycleExponent                                                = 4391,
    /// SEPARATION-TIME
    SeparationTime                                                         = 154,
    /// SEPARATION-TIME-REQUEST
    SeparationTimeRequest                                                  = 4375,
    /// SEPARATION-TIME-RESPONSE
    SeparationTimeResponse                                                 = 2585,
    /// SEQUENCE-COUNTER
    SequenceCounter                                                        = 4955,
    /// SEQUENCE-COUNTER-JUMP-WIDTH
    SequenceCounterJumpWidth                                               = 3831,
    /// SEQUENCE-OFFSET
    SequenceOffset                                                         = 667,
    /// SEQUENTIAL-INSTRUCTIONS-BEGIN
    SequentialInstructionsBegin                                            = 4843,
    /// SEQUENTIAL-INSTRUCTIONS-END
    SequentialInstructionsEnd                                              = 1705,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 5580,
    /// SERIALIZATION-TECHNOLOGY-REF
    SerializationTechnologyRef                                             = 4297,
    /// SERIALIZER
    Serializer                                                             = 2361,
    /// SERIALIZER-REF
    SerializerRef                                                          = 4800,
    /// SERVER-ARGUMENT-IMPL-POLICY
    ServerArgumentImplPolicy                                               = 2326,
    /// SERVER-CALL-POINTS
    ServerCallPoints                                                       = 5439,
    /// SERVER-COM-SPEC
    ServerComSpec                                                          = 675,
    /// SERVER-NAME-IDENTIFICATION
    ServerNameIdentification                                               = 5768,
    /// SERVER-PORT-REF
    ServerPortRef                                                          = 1769,
    /// SERVER-SERVICE-MAJOR-VERSION
    ServerServiceMajorVersion                                              = 2881,
    /// SERVER-SERVICE-MINOR-VERSION
    ServerServiceMinorVersion                                              = 5390,
    /// SERVICE-BUSY
    ServiceBusy                                                            = 96,
    /// SERVICE-CONTROL
    ServiceControl                                                         = 1783,
    /// SERVICE-DEPENDENCYS
    ServiceDependencys                                                     = 4116,
    /// SERVICE-DEPLOYMENT-REF
    ServiceDeploymentRef                                                   = 1220,
    /// SERVICE-DISCOVER-CONFIGS
    ServiceDiscoverConfigs                                                 = 5844,
    /// SERVICE-DISCOVERY-CONFIGS
    ServiceDiscoveryConfigs                                                = 654,
    /// SERVICE-ELEMENT-MAPPING-REF
    ServiceElementMappingRef                                               = 4477,
    /// SERVICE-ELEMENT-MAPPING-REFS
    ServiceElementMappingRefs                                              = 2859,
    /// SERVICE-ELEMENT-SECURE-COM-CONFIG-REF
    ServiceElementSecureComConfigRef                                       = 2099,
    /// SERVICE-FIND-TIME-TO-LIVE
    ServiceFindTimeToLive                                                  = 736,
    /// SERVICE-ID
    ServiceId                                                              = 1131,
    /// SERVICE-IDENTIFIER
    ServiceIdentifier                                                      = 5469,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 4120,
    /// SERVICE-INSTANCE-ID
    ServiceInstanceId                                                      = 1450,
    /// SERVICE-INSTANCE-PORT-CONFIG
    ServiceInstancePortConfig                                              = 2512,
    /// SERVICE-INSTANCE-REF
    ServiceInstanceRef                                                     = 4162,
    /// SERVICE-INSTANCE-REFS
    ServiceInstanceRefs                                                    = 2270,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 2323,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 2722,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF
    ServiceInstanceToPortPrototypeMappingRef                               = 160,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 3975,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 4759,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS
    ServiceInstanceToSignalMappings                                        = 5841,
    /// SERVICE-INSTANCES
    ServiceInstances                                                       = 5368,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 1505,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 1348,
    /// SERVICE-INTERFACE-DEPLOYMENT-REF
    ServiceInterfaceDeploymentRef                                          = 3893,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REF
    ServiceInterfaceElementMappingRef                                      = 1108,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REFS
    ServiceInterfaceElementMappingRefs                                     = 5479,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 895,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 5342,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 2424,
    /// SERVICE-INTERFACE-ID
    ServiceInterfaceId                                                     = 1421,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 5405,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 3322,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 4478,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 1956,
    /// SERVICE-INTERFACE-REF
    ServiceInterfaceRef                                                    = 1414,
    /// SERVICE-INTERFACE-REFS
    ServiceInterfaceRefs                                                   = 2973,
    /// SERVICE-INTERFACE-SUB-ELEMENT
    ServiceInterfaceSubElement                                             = 2566,
    /// SERVICE-INTERFACE-TREF
    ServiceInterfaceTref                                                   = 6245,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 3999,
    /// SERVICE-INTERFACE-VERSION
    ServiceInterfaceVersion                                                = 2390,
    /// SERVICE-ITEMS
    ServiceItems                                                           = 3394,
    /// SERVICE-KIND
    ServiceKind                                                            = 2556,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 2125,
    /// SERVICE-OFFER-TIME-TO-LIVE
    ServiceOfferTimeToLive                                                 = 2045,
    /// SERVICE-POINT-SYMBOL-POST
    ServicePointSymbolPost                                                 = 3451,
    /// SERVICE-POINT-SYMBOL-PRE
    ServicePointSymbolPre                                                  = 6248,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 1816,
    /// SERVICE-REQUEST-CALLBACK-TYPE
    ServiceRequestCallbackType                                             = 3873,
    /// SERVICE-RESOURCE-REF
    ServiceResourceRef                                                     = 6236,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 3656,
    /// SERVICE-TABLES
    ServiceTables                                                          = 5765,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 5869,
    /// SESSION-CONTROL-CLASS-REF
    SessionControlClassRef                                                 = 3887,
    /// SESSION-HANDLING
    SessionHandling                                                        = 3485,
    /// SESSION-HANDLING-SR
    SessionHandlingSr                                                      = 4114,
    /// SESSION-ID
    SessionId                                                              = 1032,
    /// SESSION-ID-SUPPORT
    SessionIdSupport                                                       = 1889,
    /// SET
    Set                                                                    = 4353,
    /// SET-FUNCTION-GROUP-STATE-IREF
    SetFunctionGroupStateIref                                              = 5136,
    /// SETTER-CALL-REF
    SetterCallRef                                                          = 3488,
    /// SETTER-CALL-SIGNAL-REF
    SetterCallSignalRef                                                    = 2171,
    /// SETTER-OPERATION-REF
    SetterOperationRef                                                     = 5122,
    /// SETTER-REF
    SetterRef                                                              = 59,
    /// SETTER-RETURN-REF
    SetterReturnRef                                                        = 4797,
    /// SETTER-RETURN-SIGNAL-REF
    SetterReturnSignalRef                                                  = 2032,
    /// SEVERITY
    Severity                                                               = 1895,
    /// SHALL-NOT-RUN-ON-REF
    ShallNotRunOnRef                                                       = 3376,
    /// SHALL-NOT-RUN-ON-REFS
    ShallNotRunOnRefs                                                      = 4631,
    /// SHALL-RUN-ON-REF
    ShallRunOnRef                                                          = 5480,
    /// SHALL-RUN-ON-REFS
    ShallRunOnRefs                                                         = 2799,
    /// SHAPER
    Shaper                                                                 = 721,
    /// SHARED-AXIS-TYPE-REF
    SharedAxisTypeRef                                                      = 4844,
    /// SHARED-PARAMETERS
    SharedParameters                                                       = 4225,
    /// SHARED-TIMER
    SharedTimer                                                            = 3273,
    /// SHORT-LABEL
    ShortLabel                                                             = 5693,
    /// SHORT-LABEL-PATTERN
    ShortLabelPattern                                                      = 20,
    /// SHORT-NAME
    ShortName                                                              = 5611,
    /// SHORT-NAME-FRAGMENT
    ShortNameFragment                                                      = 2729,
    /// SHORT-NAME-FRAGMENTS
    ShortNameFragments                                                     = 5890,
    /// SHORT-NAME-PATTERN
    ShortNamePattern                                                       = 4254,
    /// SHORT-TERM-ADJUSTMENT
    ShortTermAdjustment                                                    = 5274,
    /// SHORT-TERM-ADJUSTMENT-SUPPORTED
    ShortTermAdjustmentSupported                                           = 3710,
    /// SHUTDOWN
    Shutdown                                                               = 1120,
    /// SIGNAL-AGE
    SignalAge                                                              = 1504,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 2356,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 2574,
    /// SIGNAL-BASED-EVENT-REF
    SignalBasedEventRef                                                    = 4101,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 4775,
    /// SIGNAL-BASED-FIELD-REF
    SignalBasedFieldRef                                                    = 3003,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 545,
    /// SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFireAndForgetMethodToISignalTriggeringMapping               = 3826,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 1030,
    /// SIGNAL-BASED-METHOD-REF
    SignalBasedMethodRef                                                   = 2041,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 3071,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 3903,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 5695,
    /// SIGNAL-FAN
    SignalFan                                                              = 1176,
    /// SIGNAL-GROUP-REF
    SignalGroupRef                                                         = 1186,
    /// SIGNAL-I-PDU-COUNTER
    SignalIPduCounter                                                      = 2033,
    /// SIGNAL-I-PDU-REPLICATION
    SignalIPduReplication                                                  = 1707,
    /// SIGNAL-MAPPINGS
    SignalMappings                                                         = 5742,
    /// SIGNAL-PATH-CONSTRAINTS
    SignalPathConstraints                                                  = 1789,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 1269,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 5433,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS
    SignalServiceTranslationEventPropss                                    = 3267,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 5440,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 2154,
    /// SIGNAL-SERVICE-TRANSLATION-PROPSS
    SignalServiceTranslationPropss                                         = 5391,
    /// SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING
    SignalToReceiverTextTableMapping                                       = 5141,
    /// SIGNALS
    Signals                                                                = 2551,
    /// SIGNATURE-SCHEME-ID
    SignatureSchemeId                                                      = 2082,
    /// SIGNATURE-SCHEME-REF
    SignatureSchemeRef                                                     = 5950,
    /// SIGNATURE-SCHEME-REFS
    SignatureSchemeRefs                                                    = 1486,
    /// SIGNATURE-SUPPORT-AP
    SignatureSupportAp                                                     = 3875,
    /// SIGNATURE-SUPPORT-CP
    SignatureSupportCp                                                     = 2155,
    /// SIGNIFICANCE
    Significance                                                           = 4217,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 2662,
    /// SIZE
    Size                                                                   = 2631,
    /// SIZE-OF-ARRAY-LENGTH-FIELD
    SizeOfArrayLengthField                                                 = 23,
    /// SIZE-OF-ARRAY-LENGTH-FIELDS
    SizeOfArrayLengthFields                                                = 685,
    /// SIZE-OF-STRING-LENGTH-FIELD
    SizeOfStringLengthField                                                = 3588,
    /// SIZE-OF-STRING-LENGTH-FIELDS
    SizeOfStringLengthFields                                               = 4345,
    /// SIZE-OF-STRUCT-LENGTH-FIELD
    SizeOfStructLengthField                                                = 5733,
    /// SIZE-OF-STRUCT-LENGTH-FIELDS
    SizeOfStructLengthFields                                               = 4019,
    /// SIZE-OF-UNION-LENGTH-FIELD
    SizeOfUnionLengthField                                                 = 805,
    /// SIZE-OF-UNION-LENGTH-FIELDS
    SizeOfUnionLengthFields                                                = 5781,
    /// SIZE-OF-UNION-TYPE-SELECTOR-FIELD
    SizeOfUnionTypeSelectorField                                           = 2377,
    /// SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE
    SlaveActAsPassiveCommunicationSlave                                    = 3418,
    /// SLAVE-EVENT-REF
    SlaveEventRef                                                          = 5490,
    /// SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME
    SlaveQualifiedUnexpectedLinkDownTime                                   = 4296,
    /// SLAVE-REF
    SlaveRef                                                               = 5645,
    /// SLAVES
    Slaves                                                                 = 3471,
    /// SLEEP-MODE-EXECUTION-DELAY
    SleepModeExecutionDelay                                                = 4308,
    /// SLEEP-MODE-SUPPORTED
    SleepModeSupported                                                     = 272,
    /// SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST
    SleepRepetitionDelayOfSleepRequest                                     = 6173,
    /// SLEEP-REPETITIONS-OF-SLEEP-REQUEST
    SleepRepetitionsOfSleepRequest                                         = 365,
    /// SLEEP-TIME
    SleepTime                                                              = 3294,
    /// SLOT-CAPACITY
    SlotCapacity                                                           = 674,
    /// SLOT-ID
    SlotId                                                                 = 2114,
    /// SLOT-TYPE
    SlotType                                                               = 4911,
    /// SM-INTERACTS-WITH-NM-MAPPING
    SmInteractsWithNmMapping                                               = 1260,
    /// SNAPSHOT-RECORD-CONTENTS
    SnapshotRecordContents                                                 = 4512,
    /// SO-AD-CONFIG
    SoAdConfig                                                             = 4050,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 5908,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 1463,
    /// SO-CON-I-PDU-IDENTIFIER-REF
    SoConIPduIdentifierRef                                                 = 1000,
    /// SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL
    SoConIPduIdentifierRefConditional                                      = 5318,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 3171,
    /// SOCKET-ADDRESS-REF
    SocketAddressRef                                                       = 544,
    /// SOCKET-ADDRESS-REF-CONDITIONAL
    SocketAddressRefConditional                                            = 2745,
    /// SOCKET-ADDRESSS
    SocketAddresss                                                         = 4891,
    /// SOCKET-CONNECTION
    SocketConnection                                                       = 4067,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 1166,
    /// SOCKET-CONNECTION-BUNDLE-REF
    SocketConnectionBundleRef                                              = 5099,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER
    SocketConnectionIpduIdentifier                                         = 2824,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 1416,
    /// SOCKET-CONNECTION-REF
    SocketConnectionRef                                                    = 4242,
    /// SOCKET-CONNECTION-REFS
    SocketConnectionRefs                                                   = 1320,
    /// SOCKET-OPTION
    SocketOption                                                           = 1560,
    /// SOCKET-OPTIONS
    SocketOptions                                                          = 1748,
    /// SOCKET-PROTOCOL
    SocketProtocol                                                         = 5848,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION
    SoftwareActivationDependencyCompareCondition                           = 5360,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA
    SoftwareActivationDependencyFormula                                    = 2634,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-REF
    SoftwareActivationDependencyRef                                        = 918,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 76,
    /// SOFTWARE-CLUSTER-COM-RESOURCE-REF
    SoftwareClusterComResourceRef                                          = 2648,
    /// SOFTWARE-CLUSTER-DEPENDENCY
    SoftwareClusterDependency                                              = 2983,
    /// SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION
    SoftwareClusterDependencyCompareCondition                              = 5400,
    /// SOFTWARE-CLUSTER-DEPENDENCY-FORMULA
    SoftwareClusterDependencyFormula                                       = 1566,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 1342,
    /// SOFTWARE-CLUSTER-DESIGN-DEPENDENCY
    SoftwareClusterDesignDependency                                        = 5991,
    /// SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS
    SoftwareClusterDiagnosticDeploymentProps                               = 5621,
    /// SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS
    SoftwareClusterDoipDiagnosticAddress                                   = 6001,
    /// SOFTWARE-CLUSTER-ID
    SoftwareClusterId                                                      = 2473,
    /// SOFTWARE-CLUSTER-REF
    SoftwareClusterRef                                                     = 1409,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 2043,
    /// SOFTWARE-CLUSTER-SOVD-ADDRESS
    SoftwareClusterSovdAddress                                             = 4046,
    /// SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING
    SoftwareClusterToApplicationPartitionMapping                           = 5063,
    /// SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPINGS
    SoftwareClusterToApplicationPartitionMappings                          = 5198,
    /// SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS
    SoftwareClusterToResourceMappings                                      = 558,
    /// SOFTWARE-CLUSTER-UDS-DIAGNOSTIC-ADDRESS
    SoftwareClusterUdsDiagnosticAddress                                    = 2568,
    /// SOFTWARE-COMPOSITION-TREF
    SoftwareCompositionTref                                                = 4140,
    /// SOFTWARE-CONTEXT
    SoftwareContext                                                        = 2207,
    /// SOFTWARE-MEMORY-SECTION-REF
    SoftwareMemorySectionRef                                               = 830,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 2321,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 3006,
    /// SOFTWARE-PACKAGE-STEPS
    SoftwarePackageSteps                                                   = 5300,
    /// SOFTWARE-PACKAGE-STORING
    SoftwarePackageStoring                                                 = 5138,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 1343,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 2172,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 3884,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 4091,
    /// SOMEIP-EVENT-PROPS
    SomeipEventProps                                                       = 5155,
    /// SOMEIP-FIELD
    SomeipField                                                            = 339,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 3815,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 4535,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 5982,
    /// SOMEIP-METHOD-PROPS
    SomeipMethodProps                                                      = 1961,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 3354,
    /// SOMEIP-REMOTE-MULTICAST-CONFIG
    SomeipRemoteMulticastConfig                                            = 158,
    /// SOMEIP-REMOTE-UNICAST-CONFIG
    SomeipRemoteUnicastConfig                                              = 937,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 1430,
    /// SOMEIP-RULE
    SomeipRule                                                             = 2771,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 589,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdClientEventGroupTimingConfigRef                                = 1167,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdClientEventGroupTimingConfigRefConditional                     = 1057,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 183,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdClientServiceInstanceConfigRef                                 = 3174,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdClientServiceInstanceConfigRefConditional                      = 3119,
    /// SOMEIP-SD-RULE
    SomeipSdRule                                                           = 5178,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 3208,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdServerEventGroupTimingConfigRef                                = 4253,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdServerEventGroupTimingConfigRefConditional                     = 5027,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 4668,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdServerServiceInstanceConfigRef                                 = 4962,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdServerServiceInstanceConfigRefConditional                      = 4128,
    /// SOMEIP-SERVICE-DISCOVERY
    SomeipServiceDiscovery                                                 = 3162,
    /// SOMEIP-SERVICE-DISCOVERY-PORT
    SomeipServiceDiscoveryPort                                             = 4386,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 2693,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 153,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 3416,
    /// SOMEIP-SERVICE-VERSION
    SomeipServiceVersion                                                   = 1378,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 1437,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 2879,
    /// SOMEIP-TP-CONNECTION
    SomeipTpConnection                                                     = 3066,
    /// SOMEIP-TRANSFORMATION-DESCRIPTION
    SomeipTransformationDescription                                        = 4696,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS
    SomeipTransformationISignalProps                                       = 4234,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    SomeipTransformationISignalPropsConditional                            = 2017,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    SomeipTransformationISignalPropsVariants                               = 5423,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 5985,
    /// SOMEIP-TRANSFORMATION-PROPS-REF
    SomeipTransformationPropsRef                                           = 279,
    /// SOUCE-CAN-ID-REF
    SouceCanIdRef                                                          = 4855,
    /// SOURCE-ADDRESS-CODE
    SourceAddressCode                                                      = 839,
    /// SOURCE-ADDRESS-MASK
    SourceAddressMask                                                      = 341,
    /// SOURCE-CAN-ID-CODE
    SourceCanIdCode                                                        = 2742,
    /// SOURCE-CAN-ID-MASK
    SourceCanIdMask                                                        = 972,
    /// SOURCE-CHANNEL
    SourceChannel                                                          = 2217,
    /// SOURCE-EEC-REF
    SourceEecRef                                                           = 3314,
    /// SOURCE-EEC-REFS
    SourceEecRefs                                                          = 2839,
    /// SOURCE-ELEMENT-REF
    SourceElementRef                                                       = 496,
    /// SOURCE-ELEMENT-REFS
    SourceElementRefs                                                      = 4838,
    /// SOURCE-EVENT-REF
    SourceEventRef                                                         = 2407,
    /// SOURCE-EVENT-REFS
    SourceEventRefs                                                        = 45,
    /// SOURCE-FIELD-REF
    SourceFieldRef                                                         = 5778,
    /// SOURCE-FRAME-REF
    SourceFrameRef                                                         = 386,
    /// SOURCE-I-PDU-REF
    SourceIPduRef                                                          = 2852,
    /// SOURCE-ID
    SourceId                                                               = 2569,
    /// SOURCE-INSTANCE-IREF
    SourceInstanceIref                                                     = 1524,
    /// SOURCE-INSTANCE-IREFS
    SourceInstanceIrefs                                                    = 2681,
    /// SOURCE-IP-ADDRESS
    SourceIpAddress                                                        = 1494,
    /// SOURCE-IPV-4-ADDRESS
    SourceIpv4Address                                                      = 1110,
    /// SOURCE-IPV-6-ADDRESS
    SourceIpv6Address                                                      = 1309,
    /// SOURCE-LIN-PID-REF
    SourceLinPidRef                                                        = 2882,
    /// SOURCE-MAC-ADDRESS
    SourceMacAddress                                                       = 331,
    /// SOURCE-MAC-ADDRESS-MASK
    SourceMacAddressMask                                                   = 2296,
    /// SOURCE-MAX-ADDRESS
    SourceMaxAddress                                                       = 3000,
    /// SOURCE-METHOD-REF
    SourceMethodRef                                                        = 2676,
    /// SOURCE-MIN-ADDRESS
    SourceMinAddress                                                       = 1958,
    /// SOURCE-NETWORK-MASK
    SourceNetworkMask                                                      = 2958,
    /// SOURCE-PORTS
    SourcePorts                                                            = 4605,
    /// SOURCE-POSSIBLE-ERROR-REF
    SourcePossibleErrorRef                                                 = 5126,
    /// SOURCE-REF
    SourceRef                                                              = 1868,
    /// SOURCE-SERVICE-INTERFACE-REF
    SourceServiceInterfaceRef                                              = 1665,
    /// SOURCE-SERVICE-INTERFACE-REFS
    SourceServiceInterfaceRefs                                             = 1953,
    /// SOURCE-SIGNAL-REF
    SourceSignalRef                                                        = 2773,
    /// SOURCE-TRIGGER-REF
    SourceTriggerRef                                                       = 4089,
    /// SOVD-GATEWAY-INSTANTIATION
    SovdGatewayInstantiation                                               = 4011,
    /// SOVD-LOCK-REF
    SovdLockRef                                                            = 4537,
    /// SOVD-SERVER-INSTANTIATION
    SovdServerInstantiation                                                = 3434,
    /// SOVD-VALIDATION-ORDER-REF
    SovdValidationOrderRef                                                 = 4904,
    /// SOVD-VALIDATION-ORDER-REFS
    SovdValidationOrderRefs                                                = 5252,
    /// SPARSE-TIMESTAMP-ENABLED
    SparseTimestampEnabled                                                 = 4159,
    /// SPECIFIC-CHANNEL-REF
    SpecificChannelRef                                                     = 443,
    /// SPECIFIC-CHANNELS
    SpecificChannels                                                       = 5811,
    /// SPECIFIC-PHYSICAL-CHANNEL-REF
    SpecificPhysicalChannelRef                                             = 1213,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 4134,
    /// SPECIFICATION-DOCUMENT-SCOPES
    SpecificationDocumentScopes                                            = 5989,
    /// SPECIFICATION-SCOPE
    SpecificationScope                                                     = 3335,
    /// SPEED
    Speed                                                                  = 6078,
    /// SPEEDUP
    Speedup                                                                = 4070,
    /// SPN
    Spn                                                                    = 3172,
    /// SPN-REF
    SpnRef                                                                 = 5715,
    /// SPN-REFS
    SpnRefs                                                                = 1055,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 4107,
    /// SSP-OFFSET
    SspOffset                                                              = 229,
    /// SSRC
    Ssrc                                                                   = 2661,
    /// ST-MIN
    StMin                                                                  = 5984,
    /// STACK-USAGES
    StackUsages                                                            = 991,
    /// STANDARD
    Standard                                                               = 1208,
    /// STANDARD-REVISION
    StandardRevision                                                       = 201,
    /// STANDARD-REVISIONS
    StandardRevisions                                                      = 2536,
    /// START
    Start                                                                  = 687,
    /// START-ACTION-REF
    StartActionRef                                                         = 4480,
    /// START-ACTION-REFS
    StartActionRefs                                                        = 5085,
    /// START-ADDRESS
    StartAddress                                                           = 2026,
    /// START-INDEX
    StartIndex                                                             = 6165,
    /// START-IREF
    StartIref                                                              = 3090,
    /// START-ON-EVENT-REF
    StartOnEventRef                                                        = 5251,
    /// START-POSITION
    StartPosition                                                          = 5219,
    /// START-STATE-MACHINE-REF
    StartStateMachineRef                                                   = 5668,
    /// STARTS-ON-EVENT-REF
    StartsOnEventRef                                                       = 3953,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 5654,
    /// STARTUP-CONFIG-REF
    StartupConfigRef                                                       = 1969,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 1392,
    /// STARTUP-CONFIGS
    StartupConfigs                                                         = 5045,
    /// STARTUP-OPTION
    StartupOption                                                          = 1770,
    /// STARTUP-OPTIONS
    StartupOptions                                                         = 5806,
    /// STATE
    State                                                                  = 381,
    /// STATE-DEP-FIREWALL-REF
    StateDepFirewallRef                                                    = 748,
    /// STATE-DEP-FIREWALL-REFS
    StateDepFirewallRefs                                                   = 3126,
    /// STATE-DEPENDENT-FIREWALL
    StateDependentFirewall                                                 = 3935,
    /// STATE-DEPENDENT-FIREWALL-REF
    StateDependentFirewallRef                                              = 3678,
    /// STATE-DEPENDENT-STARTUP-CONFIG
    StateDependentStartupConfig                                            = 5134,
    /// STATE-DEPENDENT-STARTUP-CONFIGS
    StateDependentStartupConfigs                                           = 4118,
    /// STATE-MACHINE
    StateMachine                                                           = 2047,
    /// STATE-MANAGEMEN-PHM-ERROR-INTERFACE
    StateManagemenPhmErrorInterface                                        = 5213,
    /// STATE-MANAGEMENT-ACTION-LIST
    StateManagementActionList                                              = 3989,
    /// STATE-MANAGEMENT-BASED-ON-TCP-FLAGS
    StateManagementBasedOnTcpFlags                                         = 4190,
    /// STATE-MANAGEMENT-COMPARE-FORMULA
    StateManagementCompareFormula                                          = 2639,
    /// STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE
    StateManagementDiagTriggerInterface                                    = 4502,
    /// STATE-MANAGEMENT-EM-ERROR-INTERFACE
    StateManagementEmErrorInterface                                        = 6180,
    /// STATE-MANAGEMENT-ERROR-COMPARE-RULE
    StateManagementErrorCompareRule                                        = 3948,
    /// STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE
    StateManagementFunctionGroupSwitchNotificationInterface                = 941,
    /// STATE-MANAGEMENT-MODULE-INSTANTIATION
    StateManagementModuleInstantiation                                     = 5238,
    /// STATE-MANAGEMENT-NM-ACTION-ITEM
    StateManagementNmActionItem                                            = 1931,
    /// STATE-MANAGEMENT-REQUEST-ERROR
    StateManagementRequestError                                            = 4505,
    /// STATE-MANAGEMENT-REQUEST-RULE
    StateManagementRequestRule                                             = 6162,
    /// STATE-MANAGEMENT-REQUEST-TRIGGER
    StateManagementRequestTrigger                                          = 82,
    /// STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM
    StateManagementSetFunctionGroupStateActionItem                         = 4575,
    /// STATE-MANAGEMENT-SLEEP-ACTION-ITEM
    StateManagementSleepActionItem                                         = 5786,
    /// STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM
    StateManagementStateMachineActionItem                                  = 5101,
    /// STATE-MANAGEMENT-STATE-NOTIFICATION
    StateManagementStateNotification                                       = 4291,
    /// STATE-MANAGEMENT-SYNC-ACTION-ITEM
    StateManagementSyncActionItem                                          = 4496,
    /// STATE-MANAGEMENT-TRIGGER-COMPARE-RULE
    StateManagementTriggerCompareRule                                      = 4940,
    /// STATE-REF
    StateRef                                                               = 3096,
    /// STATE-REFERENCES
    StateReferences                                                        = 2790,
    /// STATE-REQUEST-PORT-IREF
    StateRequestPortIref                                                   = 3432,
    /// STATE-REQUEST-REF
    StateRequestRef                                                        = 1589,
    /// STATIC-MEMORYS
    StaticMemorys                                                          = 5976,
    /// STATIC-PART
    StaticPart                                                             = 781,
    /// STATIC-PARTS
    StaticParts                                                            = 5542,
    /// STATIC-REMOTE-MULTICAST-ADDRESSES
    StaticRemoteMulticastAddresses                                         = 447,
    /// STATIC-REMOTE-UNICAST-ADDRESSES
    StaticRemoteUnicastAddresses                                           = 3040,
    /// STATIC-SLOT-DURATION
    StaticSlotDuration                                                     = 4782,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 4743,
    /// STATIC-SOCKET-CONNECTION-REF
    StaticSocketConnectionRef                                              = 3882,
    /// STATIC-SOCKET-CONNECTIONS
    StaticSocketConnections                                                = 4131,
    /// STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR
    StatusBitHandlingTestFailedSinceLastClear                              = 1837,
    /// STATUS-BIT-STORAGE-TEST-FAILED
    StatusBitStorageTestFailed                                             = 313,
    /// STATUS-ID
    StatusId                                                               = 2747,
    /// STATUS-IREF
    StatusIref                                                             = 3303,
    /// STATUS-REF
    StatusRef                                                              = 4864,
    /// STATUS-SUB-TLV
    StatusSubTlv                                                           = 3233,
    /// STATUSS
    Statuss                                                                = 5761,
    /// STD
    Std                                                                    = 989,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 6247,
    /// STD-REF
    StdRef                                                                 = 5299,
    /// STD-REFS
    StdRefs                                                                = 3636,
    /// STEP-SIZE
    StepSize                                                               = 3131,
    /// STIM-ENABLER
    StimEnabler                                                            = 4915,
    /// STIMULUS-REF
    StimulusRef                                                            = 4533,
    /// STOP
    Stop                                                                   = 5091,
    /// STOP-IREF
    StopIref                                                               = 1441,
    /// STOP-STATE-MACHINE-REF
    StopStateMachineRef                                                    = 6220,
    /// STORAGE-CONDITION-GROUP-REF
    StorageConditionGroupRef                                               = 1332,
    /// STORAGE-CONDITIONS
    StorageConditions                                                      = 5008,
    /// STORAGE-STATE-EVALUATION
    StorageStateEvaluation                                                 = 2282,
    /// STORE-AT-SHUTDOWN
    StoreAtShutdown                                                        = 1768,
    /// STORE-CYCLIC
    StoreCyclic                                                            = 5506,
    /// STORE-EMERGENCY
    StoreEmergency                                                         = 1279,
    /// STORE-EVENT-ENABLED
    StoreEventEnabled                                                      = 5518,
    /// STORE-EVENT-SUPPORT
    StoreEventSupport                                                      = 6133,
    /// STORE-IMMEDIATE
    StoreImmediate                                                         = 290,
    /// STORE-ON-CHANGE
    StoreOnChange                                                          = 4735,
    /// STORING
    Storing                                                                = 3801,
    /// STREAM-FILTER-PORT-RANGE
    StreamFilterPortRange                                                  = 1767,
    /// STREAM-FILTER-RULE
    StreamFilterRule                                                       = 4686,
    /// STREAM-FILTERS
    StreamFilters                                                          = 3103,
    /// STREAM-GATE-REF
    StreamGateRef                                                          = 6016,
    /// STREAM-GATES
    StreamGates                                                            = 4022,
    /// STREAM-ID
    StreamId                                                               = 968,
    /// STREAM-IDENTIFICATION-HANDLE-REF
    StreamIdentificationHandleRef                                          = 2937,
    /// STREAM-IDENTIFICATION-HANDLE-REFS
    StreamIdentificationHandleRefs                                         = 2079,
    /// STREAM-IDENTIFICATION-WILDCARD
    StreamIdentificationWildcard                                           = 250,
    /// STREAM-IDENTIFIER
    StreamIdentifier                                                       = 1814,
    /// STREAMS-PER-FRAME
    StreamsPerFrame                                                        = 382,
    /// STRING-ENCODING
    StringEncoding                                                         = 3742,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 414,
    /// SUB
    Sub                                                                    = 453,
    /// SUB-ATTRIBUTE-TAILORINGS
    SubAttributeTailorings                                                 = 4268,
    /// SUB-CONTAINERS
    SubContainers                                                          = 3675,
    /// SUB-DOMAIN-REF
    SubDomainRef                                                           = 517,
    /// SUB-DOMAIN-REFS
    SubDomainRefs                                                          = 4585,
    /// SUB-ELEMENT-MAPPING
    SubElementMapping                                                      = 3583,
    /// SUB-ELEMENT-MAPPINGS
    SubElementMappings                                                     = 5449,
    /// SUB-ELEMENTS
    SubElements                                                            = 211,
    /// SUB-FUNCTION-REF
    SubFunctionRef                                                         = 4357,
    /// SUB-FUNCTION-REFS
    SubFunctionRefs                                                        = 4087,
    /// SUB-GROUP-REF
    SubGroupRef                                                            = 5046,
    /// SUB-GROUP-REFS
    SubGroupRefs                                                           = 237,
    /// SUB-NODE-CHANNEL-REF
    SubNodeChannelRef                                                      = 4751,
    /// SUB-NODE-CHANNELS
    SubNodeChannels                                                        = 5586,
    /// SUB-NODE-NUMBER
    SubNodeNumber                                                          = 771,
    /// SUB-NODE-PHYSICAL-CHANNEL-REF
    SubNodePhysicalChannelRef                                              = 5513,
    /// SUB-SDG-REF
    SubSdgRef                                                              = 6168,
    /// SUB-SOFTWARE-CLUSTER-REF
    SubSoftwareClusterRef                                                  = 103,
    /// SUB-SOFTWARE-CLUSTER-REFS
    SubSoftwareClusterRefs                                                 = 5587,
    /// SUB-TLV-CONFIG
    SubTlvConfig                                                           = 3695,
    /// SUB-TYPE
    SubType                                                                = 4625,
    /// SUBFUNCTION
    Subfunction                                                            = 2052,
    /// SUBFUNCTIONS
    Subfunctions                                                           = 3513,
    /// SUBMESSAGE-TYPE
    SubmessageType                                                         = 4639,
    /// SUBNET-NUMBER
    SubnetNumber                                                           = 6171,
    /// SUBSCRIBE-EVENTGROUP-RETRY-DELAY
    SubscribeEventgroupRetryDelay                                          = 550,
    /// SUBSCRIBE-EVENTGROUP-RETRY-MAX
    SubscribeEventgroupRetryMax                                            = 5728,
    /// SUBSTITUTED-FRAME-REF
    SubstitutedFrameRef                                                    = 1960,
    /// SUBSTITUTED-FRAME-REFS
    SubstitutedFrameRefs                                                   = 1943,
    /// SUBTITLE
    Subtitle                                                               = 5078,
    /// SUCCESSOR-REF
    SuccessorRef                                                           = 3429,
    /// SUCCESSOR-REFS
    SuccessorRefs                                                          = 4926,
    /// SUP
    Sup                                                                    = 5947,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 1049,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF
    SupervisedEntityCheckpointNeedsRef                                     = 128,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL
    SupervisedEntityCheckpointNeedsRefConditional                          = 2160,
    /// SUPERVISED-ENTITY-ID
    SupervisedEntityId                                                     = 557,
    /// SUPERVISED-ENTITY-IREF
    SupervisedEntityIref                                                   = 4066,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 2100,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 5311,
    /// SUPERVISION-CONDITION
    SupervisionCondition                                                   = 2257,
    /// SUPERVISION-CYCLE
    SupervisionCycle                                                       = 596,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 3616,
    /// SUPERVISION-ENTITYS
    SupervisionEntitys                                                     = 57,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 3092,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 5663,
    /// SUPERVISION-MODE-CONDITIONS
    SupervisionModeConditions                                              = 5372,
    /// SUPERVISION-MODES
    SupervisionModes                                                       = 3094,
    /// SUPERVISION-REF
    SupervisionRef                                                         = 1141,
    /// SUPPLIER-ID
    SupplierId                                                             = 1854,
    /// SUPPLIER-VALIDATION-ORDER-REF
    SupplierValidationOrderRef                                             = 3043,
    /// SUPPLIER-VALIDATION-ORDER-REFS
    SupplierValidationOrderRefs                                            = 4783,
    /// SUPPORT-BUFFER-LOCKING
    SupportBufferLocking                                                   = 3271,
    /// SUPPORT-DIRTY-FLAG
    SupportDirtyFlag                                                       = 1404,
    /// SUPPORT-INFO
    SupportInfo                                                            = 3347,
    /// SUPPORT-INFO-BIT
    SupportInfoBit                                                         = 3146,
    /// SUPPORT-INFO-BYTE
    SupportInfoByte                                                        = 3359,
    /// SUPPORTED-CIPHER-SUITES
    SupportedCipherSuites                                                  = 5500,
    /// SUPPORTED-CONFIG-VARIANT
    SupportedConfigVariant                                                 = 4336,
    /// SUPPORTED-CONFIG-VARIANTS
    SupportedConfigVariants                                                = 3246,
    /// SUPPORTED-FEATURES
    SupportedFeatures                                                      = 4445,
    /// SUPPORTED-TIMER-GRANULARITY
    SupportedTimerGranularity                                              = 2705,
    /// SUPPORTING-MATERIAL
    SupportingMaterial                                                     = 314,
    /// SUPPORTS-ASYNCHRONOUS-MODE-SWITCH
    SupportsAsynchronousModeSwitch                                         = 3049,
    /// SUPPORTS-MULTIPLE-INSTANTIATION
    SupportsMultipleInstantiation                                          = 1846,
    /// SUPPORTS-MULTIPLE-NOTIFIER-SETS
    SupportsMultipleNotifierSets                                           = 5697,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 552,
    /// SW-ADDR-METHOD-REF
    SwAddrMethodRef                                                        = 3964,
    /// SW-ADDRMETHOD-REF
    SwAddrmethodRef                                                        = 3462,
    /// SW-ALIGNMENT
    SwAlignment                                                            = 3202,
    /// SW-ARRAYSIZE
    SwArraysize                                                            = 5321,
    /// SW-AXIS-CONT
    SwAxisCont                                                             = 4182,
    /// SW-AXIS-CONTS
    SwAxisConts                                                            = 2872,
    /// SW-AXIS-GENERIC
    SwAxisGeneric                                                          = 2173,
    /// SW-AXIS-GROUPED
    SwAxisGrouped                                                          = 3023,
    /// SW-AXIS-INDEX
    SwAxisIndex                                                            = 6103,
    /// SW-AXIS-INDIVIDUAL
    SwAxisIndividual                                                       = 4482,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 5379,
    /// SW-AXIS-TYPE-REF
    SwAxisTypeRef                                                          = 3549,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 1841,
    /// SW-BIT-REPRESENTATION
    SwBitRepresentation                                                    = 3595,
    /// SW-CALIBRATION-ACCESS
    SwCalibrationAccess                                                    = 3110,
    /// SW-CALIBRATION-NOTES
    SwCalibrationNotes                                                     = 1217,
    /// SW-CALIBRATION-NOTESS
    SwCalibrationNotess                                                    = 3449,
    /// SW-CALPRM-AXIS
    SwCalprmAxis                                                           = 703,
    /// SW-CALPRM-AXIS-SET
    SwCalprmAxisSet                                                        = 3791,
    /// SW-CALPRM-IMPL
    SwCalprmImpl                                                           = 2844,
    /// SW-CALPRM-IMPLS
    SwCalprmImpls                                                          = 5800,
    /// SW-CALPRM-PROTOTYPE-REF
    SwCalprmPrototypeRef                                                   = 967,
    /// SW-CARB-DOC
    SwCarbDoc                                                              = 4641,
    /// SW-CARB-DOCS
    SwCarbDocs                                                             = 1983,
    /// SW-CLASS-ATTR-INSTANCE-IMPL
    SwClassAttrInstanceImpl                                                = 4989,
    /// SW-CLASS-IMPL
    SwClassImpl                                                            = 2925,
    /// SW-CLASS-IMPLS
    SwClassImpls                                                           = 247,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 4859,
    /// SW-CLASS-PROTOTYPE-REF
    SwClassPrototypeRef                                                    = 1598,
    /// SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF
    SwClusterDesignProvidedPortIref                                        = 2689,
    /// SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF
    SwClusterDesignRequiredPortIref                                        = 3878,
    /// SW-CLUSTER-MAPPINGS
    SwClusterMappings                                                      = 2488,
    /// SW-CLUSTER-REF
    SwClusterRef                                                           = 5785,
    /// SW-CLUSTER-REFS
    SwClusterRefs                                                          = 2697,
    /// SW-CLUSTERS
    SwClusters                                                             = 3083,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 1788,
    /// SW-COMP-TO-ECU-MAPPING-REF
    SwCompToEcuMappingRef                                                  = 1731,
    /// SW-COMP-TO-ECU-MAPPING-REFS
    SwCompToEcuMappingRefs                                                 = 2088,
    /// SW-COMPARISON-VARIABLES
    SwComparisonVariables                                                  = 6229,
    /// SW-COMPONENT-ASSIGNMENTS
    SwComponentAssignments                                                 = 2990,
    /// SW-COMPONENT-DOCUMENTATION
    SwComponentDocumentation                                               = 3401,
    /// SW-COMPONENT-DOCUMENTATIONS
    SwComponentDocumentations                                              = 1689,
    /// SW-COMPONENT-IREF
    SwComponentIref                                                        = 3163,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 3409,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 4158,
    /// SW-COMPONENT-PROTOTYPE-ASSIGNMENT
    SwComponentPrototypeAssignment                                         = 3849,
    /// SW-COMPONENT-PROTOTYPE-IREF
    SwComponentPrototypeIref                                               = 4377,
    /// SW-COMPONENT-PROTOTYPE-REF
    SwComponentPrototypeRef                                                = 3496,
    /// SW-COMPOSITIONS
    SwCompositions                                                         = 4794,
    /// SW-DATA-DEF-PROPS
    SwDataDefProps                                                         = 1449,
    /// SW-DATA-DEF-PROPS-CONDITIONAL
    SwDataDefPropsConditional                                              = 1925,
    /// SW-DATA-DEF-PROPS-VARIANTS
    SwDataDefPropsVariants                                                 = 6066,
    /// SW-DATA-DEPENDENCY
    SwDataDependency                                                       = 636,
    /// SW-DATA-DEPENDENCY-ARGS
    SwDataDependencyArgs                                                   = 5482,
    /// SW-DATA-DEPENDENCY-FORMULA
    SwDataDependencyFormula                                                = 3774,
    /// SW-DIAGNOSTICS-NOTES
    SwDiagnosticsNotes                                                     = 1017,
    /// SW-DIAGNOSTICS-NOTESS
    SwDiagnosticsNotess                                                    = 787,
    /// SW-FEATURE-DEF
    SwFeatureDef                                                           = 6125,
    /// SW-FEATURE-DEFS
    SwFeatureDefs                                                          = 5920,
    /// SW-FEATURE-DESC
    SwFeatureDesc                                                          = 2687,
    /// SW-FEATURE-DESCS
    SwFeatureDescs                                                         = 1126,
    /// SW-FILL-CHARACTER
    SwFillCharacter                                                        = 4784,
    /// SW-GENERIC-AXIS-DESC
    SwGenericAxisDesc                                                      = 3417,
    /// SW-GENERIC-AXIS-PARAM
    SwGenericAxisParam                                                     = 4902,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 2963,
    /// SW-GENERIC-AXIS-PARAM-TYPE-REF
    SwGenericAxisParamTypeRef                                              = 2049,
    /// SW-GENERIC-AXIS-PARAM-TYPES
    SwGenericAxisParamTypes                                                = 1102,
    /// SW-GENERIC-AXIS-PARAMS
    SwGenericAxisParams                                                    = 4922,
    /// SW-HOST-VARIABLE
    SwHostVariable                                                         = 3756,
    /// SW-IMPL-MAPPINGS
    SwImplMappings                                                         = 2738,
    /// SW-IMPL-POLICY
    SwImplPolicy                                                           = 4312,
    /// SW-INTENDED-RESOLUTION
    SwIntendedResolution                                                   = 5849,
    /// SW-INTERPOLATION-METHOD
    SwInterpolationMethod                                                  = 2115,
    /// SW-IS-VIRTUAL
    SwIsVirtual                                                            = 5821,
    /// SW-MAINTENANCE-NOTES
    SwMaintenanceNotes                                                     = 1296,
    /// SW-MAINTENANCE-NOTESS
    SwMaintenanceNotess                                                    = 115,
    /// SW-MAPPINGS
    SwMappings                                                             = 4920,
    /// SW-MAX-AXIS-POINTS
    SwMaxAxisPoints                                                        = 2459,
    /// SW-MAX-TEXT-SIZE
    SwMaxTextSize                                                          = 2367,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 4767,
    /// SW-MIN-AXIS-POINTS
    SwMinAxisPoints                                                        = 576,
    /// SW-NUMBER-OF-AXIS-POINTS
    SwNumberOfAxisPoints                                                   = 3035,
    /// SW-POINTER-TARGET-PROPS
    SwPointerTargetProps                                                   = 4584,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 6040,
    /// SW-RECORD-LAYOUT-COMPONENT
    SwRecordLayoutComponent                                                = 3403,
    /// SW-RECORD-LAYOUT-GROUP
    SwRecordLayoutGroup                                                    = 3883,
    /// SW-RECORD-LAYOUT-GROUP-AXIS
    SwRecordLayoutGroupAxis                                                = 5757,
    /// SW-RECORD-LAYOUT-GROUP-FROM
    SwRecordLayoutGroupFrom                                                = 3008,
    /// SW-RECORD-LAYOUT-GROUP-INDEX
    SwRecordLayoutGroupIndex                                               = 3532,
    /// SW-RECORD-LAYOUT-GROUP-STEP
    SwRecordLayoutGroupStep                                                = 4811,
    /// SW-RECORD-LAYOUT-GROUP-TO
    SwRecordLayoutGroupTo                                                  = 5591,
    /// SW-RECORD-LAYOUT-REF
    SwRecordLayoutRef                                                      = 1664,
    /// SW-RECORD-LAYOUT-V
    SwRecordLayoutV                                                        = 1937,
    /// SW-RECORD-LAYOUT-V-AXIS
    SwRecordLayoutVAxis                                                    = 3921,
    /// SW-RECORD-LAYOUT-V-FIX-VALUE
    SwRecordLayoutVFixValue                                                = 6009,
    /// SW-RECORD-LAYOUT-V-INDEX
    SwRecordLayoutVIndex                                                   = 3859,
    /// SW-RECORD-LAYOUT-V-PROP
    SwRecordLayoutVProp                                                    = 2936,
    /// SW-REFRESH-TIMING
    SwRefreshTiming                                                        = 3726,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 2652,
    /// SW-SERVICE-IMPL-POLICY
    SwServiceImplPolicy                                                    = 3780,
    /// SW-SYSCOND
    SwSyscond                                                              = 6041,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 489,
    /// SW-SYSTEMCONST-REF
    SwSystemconstRef                                                       = 1935,
    /// SW-SYSTEMCONST-VALUE
    SwSystemconstValue                                                     = 2939,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 6149,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REF
    SwSystemconstantValueSetRef                                            = 3017,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REFS
    SwSystemconstantValueSetRefs                                           = 5799,
    /// SW-SYSTEMCONSTANT-VALUES
    SwSystemconstantValues                                                 = 5607,
    /// SW-TEST-DESC
    SwTestDesc                                                             = 3768,
    /// SW-TEST-DESCS
    SwTestDescs                                                            = 4108,
    /// SW-TEXT-PROPS
    SwTextProps                                                            = 5519,
    /// SW-VALUE-BLOCK-SIZE
    SwValueBlockSize                                                       = 4599,
    /// SW-VALUE-BLOCK-SIZE-MULTS
    SwValueBlockSizeMults                                                  = 2975,
    /// SW-VALUE-CONT
    SwValueCont                                                            = 4084,
    /// SW-VALUES-PHYS
    SwValuesPhys                                                           = 6043,
    /// SW-VARIABLE-IMPL
    SwVariableImpl                                                         = 2064,
    /// SW-VARIABLE-IMPLS
    SwVariableImpls                                                        = 2974,
    /// SW-VARIABLE-PROTOTYPE-REF
    SwVariablePrototypeRef                                                 = 202,
    /// SW-VARIABLE-REFS
    SwVariableRefs                                                         = 3802,
    /// SW-VERSION
    SwVersion                                                              = 4553,
    /// SWC-BEHAVIOR-REF
    SwcBehaviorRef                                                         = 4706,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 2202,
    /// SWC-BSW-MAPPING-REF
    SwcBswMappingRef                                                       = 562,
    /// SWC-BSW-RUNNABLE-MAPPING
    SwcBswRunnableMapping                                                  = 2756,
    /// SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE
    SwcBswSynchronizedModeGroupPrototype                                   = 742,
    /// SWC-BSW-SYNCHRONIZED-TRIGGER
    SwcBswSynchronizedTrigger                                              = 525,
    /// SWC-EXCLUSIVE-AREA-POLICY
    SwcExclusiveAreaPolicy                                                 = 871,
    /// SWC-FLAT-SERVICE-DEPENDENCY-REF
    SwcFlatServiceDependencyRef                                            = 6100,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 14,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 3723,
    /// SWC-MAPPING-CONSTRAINT-REF
    SwcMappingConstraintRef                                                = 222,
    /// SWC-MAPPING-CONSTRAINT-REFS
    SwcMappingConstraintRefs                                               = 231,
    /// SWC-MODE-GROUP-IREF
    SwcModeGroupIref                                                       = 396,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 1025,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 4996,
    /// SWC-MODE-SWITCH-EVENT-REF
    SwcModeSwitchEventRef                                                  = 4229,
    /// SWC-RUNNABLE-REF
    SwcRunnableRef                                                         = 4343,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 2577,
    /// SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    SwcServiceDependencyInExecutableIref                                   = 5716,
    /// SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    SwcServiceDependencyInSystemIref                                       = 3879,
    /// SWC-SERVICE-DEPENDENCY-IREF
    SwcServiceDependencyIref                                               = 5445,
    /// SWC-TIMING
    SwcTiming                                                              = 2997,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 5647,
    /// SWC-TO-APPLICATION-PARTITION-MAPPINGS
    SwcToApplicationPartitionMappings                                      = 1908,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 6070,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT
    SwcToEcuMappingConstraint                                              = 2197,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE
    SwcToEcuMappingConstraintType                                          = 4271,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 5739,
    /// SWC-TO-SWC-OPERATION-ARGUMENTS
    SwcToSwcOperationArguments                                             = 744,
    /// SWC-TO-SWC-SIGNAL
    SwcToSwcSignal                                                         = 4276,
    /// SWC-TRIGGER-IREF
    SwcTriggerIref                                                         = 5617,
    /// SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY
    SwitchAsynchronousTrafficShaperGroupEntry                              = 6044,
    /// SWITCH-FLOW-METERING-ENTRY
    SwitchFlowMeteringEntry                                                = 5562,
    /// SWITCH-STREAM-FILTER-ENTRY
    SwitchStreamFilterEntry                                                = 2013,
    /// SWITCH-STREAM-GATE-ENTRY
    SwitchStreamGateEntry                                                  = 5153,
    /// SWITCH-STREAM-IDENTIFICATION
    SwitchStreamIdentification                                             = 3797,
    /// SWITCH-STREAM-IDENTIFICATIONS
    SwitchStreamIdentifications                                            = 5263,
    /// SYMBOL
    Symbol                                                                 = 5232,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 5655,
    /// SYMBOL-WINDOW
    SymbolWindow                                                           = 3379,
    /// SYMBOL-WINDOW-ACTION-POINT-OFFSET
    SymbolWindowActionPointOffset                                          = 4263,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 4384,
    /// SYMBOLIC-NAME-VALUE
    SymbolicNameValue                                                      = 5472,
    /// SYNC-CONFIRMATION-TIMEOUT
    SyncConfirmationTimeout                                                = 2706,
    /// SYNC-COUNTER-INIT
    SyncCounterInit                                                        = 4427,
    /// SYNC-DATA-ID-LIST
    SyncDataIdList                                                         = 1370,
    /// SYNC-DATA-ID-LISTS
    SyncDataIdLists                                                        = 1458,
    /// SYNC-FRAME-ID-COUNT-MAX
    SyncFrameIdCountMax                                                    = 5383,
    /// SYNC-INTERVAL
    SyncInterval                                                           = 4009,
    /// SYNC-JUMP-WIDTH
    SyncJumpWidth                                                          = 2932,
    /// SYNC-LOSS-THRESHOLD
    SyncLossThreshold                                                      = 3184,
    /// SYNC-LOSS-TIMEOUT
    SyncLossTimeout                                                        = 5718,
    /// SYNC-NODE-MAX
    SyncNodeMax                                                            = 774,
    /// SYNC-PERIOD
    SyncPeriod                                                             = 3617,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 1153,
    /// SYNCHRONIZATION-CONSTRAINT-TYPE
    SynchronizationConstraintType                                          = 2374,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 411,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 889,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 6134,
    /// SYNCHRONIZED-MODE-GROUPS
    SynchronizedModeGroups                                                 = 738,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 116,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 4200,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 625,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 2695,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 4825,
    /// SYNCHRONIZED-TRIGGERS
    SynchronizedTriggers                                                   = 587,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 4667,
    /// SYSC-REF
    SyscRef                                                                = 4250,
    /// SYSC-STRING-REF
    SyscStringRef                                                          = 3489,
    /// SYSTEM
    System                                                                 = 3097,
    /// SYSTEM-DOCUMENTATIONS
    SystemDocumentations                                                   = 1575,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 4577,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 741,
    /// SYSTEM-MEMORY-USAGES
    SystemMemoryUsages                                                     = 1988,
    /// SYSTEM-REF
    SystemRef                                                              = 802,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 1597,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 5592,
    /// SYSTEM-SIGNAL-GROUP-REF
    SystemSignalGroupRef                                                   = 6241,
    /// SYSTEM-SIGNAL-GROUP-TO-COM-RESOURCE-MAPPINGS
    SystemSignalGroupToComResourceMappings                                 = 1405,
    /// SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalGroupToCommunicationResourceMapping                        = 4834,
    /// SYSTEM-SIGNAL-REF
    SystemSignalRef                                                        = 3030,
    /// SYSTEM-SIGNAL-REFS
    SystemSignalRefs                                                       = 547,
    /// SYSTEM-SIGNAL-TO-COM-RESOURCE-MAPPINGS
    SystemSignalToComResourceMappings                                      = 1926,
    /// SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalToCommunicationResourceMapping                             = 1812,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 4910,
    /// SYSTEM-TRIGGERED-EVENTS
    SystemTriggeredEvents                                                  = 4586,
    /// SYSTEM-VERSION
    SystemVersion                                                          = 6121,
    /// TA-TYPE
    TaType                                                                 = 3565,
    /// TABLE
    Table                                                                  = 752,
    /// TABLE-CAPTION
    TableCaption                                                           = 3120,
    /// TABLE-ENTRYS
    TableEntrys                                                            = 3323,
    /// TAG-ID
    TagId                                                                  = 1993,
    /// TAG-WITH-OPTIONAL-VALUE
    TagWithOptionalValue                                                   = 2376,
    /// TAILORING-REF
    TailoringRef                                                           = 178,
    /// TAILORING-REFS
    TailoringRefs                                                          = 4090,
    /// TARGET-APPLICATION-RECORD-ELEMENT-REF
    TargetApplicationRecordElementRef                                      = 218,
    /// TARGET-CATEGORY
    TargetCategory                                                         = 4997,
    /// TARGET-CHANNEL
    TargetChannel                                                          = 1662,
    /// TARGET-COMPONENT-REF
    TargetComponentRef                                                     = 977,
    /// TARGET-DATA-ELEMENT-REF
    TargetDataElementRef                                                   = 6278,
    /// TARGET-DATA-PROTOTYPE-GROUP-REF
    TargetDataPrototypeGroupRef                                            = 3036,
    /// TARGET-DATA-PROTOTYPE-IN-CS-REF
    TargetDataPrototypeInCsRef                                             = 3907,
    /// TARGET-DATA-PROTOTYPE-IN-SR-REF
    TargetDataPrototypeInSrRef                                             = 927,
    /// TARGET-DATA-PROTOTYPE-REF
    TargetDataPrototypeRef                                                 = 1002,
    /// TARGET-DATA-PROTOYPE-REF
    TargetDataProtoypeRef                                                  = 1199,
    /// TARGET-EEC-REF
    TargetEecRef                                                           = 1954,
    /// TARGET-EEC-REFS
    TargetEecRefs                                                          = 1918,
    /// TARGET-ELEMENT-REF
    TargetElementRef                                                       = 273,
    /// TARGET-EVENT-REF
    TargetEventRef                                                         = 1192,
    /// TARGET-EVENT-REFS
    TargetEventRefs                                                        = 297,
    /// TARGET-FIELD-REF
    TargetFieldRef                                                         = 2805,
    /// TARGET-FRAME-REF
    TargetFrameRef                                                         = 18,
    /// TARGET-HEALTH-CHANNEL-REF
    TargetHealthChannelRef                                                 = 2763,
    /// TARGET-I-PDU
    TargetIPdu                                                             = 215,
    /// TARGET-I-PDU-REF
    TargetIPduRef                                                          = 148,
    /// TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TargetImplementationDataTypeElementRef                                 = 4010,
    /// TARGET-METHOD-REF
    TargetMethodRef                                                        = 1175,
    /// TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    TargetModeDeclarationGroupPrototypeRef                                 = 5475,
    /// TARGET-MODE-DECLARATION-REF
    TargetModeDeclarationRef                                               = 3805,
    /// TARGET-MODE-GROUP-REF
    TargetModeGroupRef                                                     = 3211,
    /// TARGET-MODE-REF
    TargetModeRef                                                          = 129,
    /// TARGET-MODULE-ID
    TargetModuleId                                                         = 3197,
    /// TARGET-MODULE-REFS
    TargetModuleRefs                                                       = 5758,
    /// TARGET-OPERATION-REF
    TargetOperationRef                                                     = 3976,
    /// TARGET-P-PORT-PROTOTYPE-REF
    TargetPPortPrototypeRef                                                = 3445,
    /// TARGET-P-PORT-REF
    TargetPPortRef                                                         = 6270,
    /// TARGET-PARAMETER-DATA-PROTOTYPE-REF
    TargetParameterDataPrototypeRef                                        = 1600,
    /// TARGET-PDU-TRIGGERINGS
    TargetPduTriggerings                                                   = 1784,
    /// TARGET-PHM-CHECKPOINT-REF
    TargetPhmCheckpointRef                                                 = 3775,
    /// TARGET-PHM-SUPERVISED-ENTITY-IREF
    TargetPhmSupervisedEntityIref                                          = 4435,
    /// TARGET-PORT-PROTOTYPE-REF
    TargetPortPrototypeRef                                                 = 397,
    /// TARGET-POSSIBLE-ERROR-REF
    TargetPossibleErrorRef                                                 = 4555,
    /// TARGET-PROVIDED-OPERATION-REF
    TargetProvidedOperationRef                                             = 2203,
    /// TARGET-R-PORT-PROTOTYPE-REF
    TargetRPortPrototypeRef                                                = 3979,
    /// TARGET-R-PORT-REF
    TargetRPortRef                                                         = 3527,
    /// TARGET-REF
    TargetRef                                                              = 1652,
    /// TARGET-REFS
    TargetRefs                                                             = 2938,
    /// TARGET-REQUIRED-OPERATION-REF
    TargetRequiredOperationRef                                             = 586,
    /// TARGET-RTE-EVENT-REF
    TargetRteEventRef                                                      = 3761,
    /// TARGET-RUNNABLE-ENTITY-GROUP-REF
    TargetRunnableEntityGroupRef                                           = 1745,
    /// TARGET-RUNNABLE-ENTITY-REF
    TargetRunnableEntityRef                                                = 5790,
    /// TARGET-SIGNAL-REF
    TargetSignalRef                                                        = 4598,
    /// TARGET-STATUS-REF
    TargetStatusRef                                                        = 2961,
    /// TARGET-SWC-SERVICE-DEPENDENCY-REF
    TargetSwcServiceDependencyRef                                          = 1235,
    /// TARGET-TRIGGER-REF
    TargetTriggerRef                                                       = 1557,
    /// TARGET-VARIABLE-ACCESS-REF
    TargetVariableAccessRef                                                = 3855,
    /// TARGET-VARIABLE-DATA-PROTOTYPE-REF
    TargetVariableDataPrototypeRef                                         = 4738,
    /// TASK
    Task                                                                   = 2560,
    /// TBODY
    Tbody                                                                  = 4026,
    /// TCP-ALIVE-CHECK-RESPONSE-TIMEOUT
    TcpAliveCheckResponseTimeout                                           = 3455,
    /// TCP-CONGESTION-AVOIDANCE-ENABLED
    TcpCongestionAvoidanceEnabled                                          = 4655,
    /// TCP-CONNECT-TIMEOUT
    TcpConnectTimeout                                                      = 836,
    /// TCP-DELAYED-ACK-TIMEOUT
    TcpDelayedAckTimeout                                                   = 3123,
    /// TCP-FAST-RECOVERY-ENABLED
    TcpFastRecoveryEnabled                                                 = 5820,
    /// TCP-FAST-RETRANSMIT-ENABLED
    TcpFastRetransmitEnabled                                               = 1962,
    /// TCP-FIN-WAIT-2-TIMEOUT
    TcpFinWait2Timeout                                                     = 1978,
    /// TCP-GENERAL-INACTIVITY-TIME
    TcpGeneralInactivityTime                                               = 1355,
    /// TCP-INITIAL-INACTIVITY-TIME
    TcpInitialInactivityTime                                               = 1641,
    /// TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP
    TcpIpArpNumGratuitousArpOnStartup                                      = 6153,
    /// TCP-IP-ARP-PACKET-QUEUE-ENABLED
    TcpIpArpPacketQueueEnabled                                             = 236,
    /// TCP-IP-ARP-REQUEST-TIMEOUT
    TcpIpArpRequestTimeout                                                 = 5604,
    /// TCP-IP-ARP-TABLE-ENTRY-TIMEOUT
    TcpIpArpTableEntryTimeout                                              = 5013,
    /// TCP-IP-AUTO-IP-INIT-TIMEOUT
    TcpIpAutoIpInitTimeout                                                 = 3507,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MAX
    TcpIpDhcpV6CnfDelayMax                                                 = 1910,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MIN
    TcpIpDhcpV6CnfDelayMin                                                 = 95,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MAX
    TcpIpDhcpV6InfDelayMax                                                 = 1038,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MIN
    TcpIpDhcpV6InfDelayMin                                                 = 451,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MAX
    TcpIpDhcpV6SolDelayMax                                                 = 3424,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MIN
    TcpIpDhcpV6SolDelayMin                                                 = 467,
    /// TCP-IP-ICMP-PROPS-REF
    TcpIpIcmpPropsRef                                                      = 872,
    /// TCP-IP-ICMP-PROPS-REFS
    TcpIpIcmpPropsRefs                                                     = 3140,
    /// TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED
    TcpIpIcmpV4EchoReplyEnabled                                            = 5635,
    /// TCP-IP-ICMP-V-4-TTL
    TcpIpIcmpV4Ttl                                                         = 4075,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION
    TcpIpIcmpV6EchoReplyAvoidFragmentation                                 = 4211,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED
    TcpIpIcmpV6EchoReplyEnabled                                            = 6076,
    /// TCP-IP-ICMP-V-6-HOP-LIMIT
    TcpIpIcmpV6HopLimit                                                    = 4042,
    /// TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED
    TcpIpIcmpV6MsgDestinationUnreachableEnabled                            = 4335,
    /// TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED
    TcpIpIcmpV6MsgParameterProblemEnabled                                  = 2981,
    /// TCP-IP-IP-FRAGMENTATION-RX-ENABLED
    TcpIpIpFragmentationRxEnabled                                          = 1727,
    /// TCP-IP-IP-NUM-FRAGMENTS
    TcpIpIpNumFragments                                                    = 1101,
    /// TCP-IP-IP-NUM-REASS-DGRAMS
    TcpIpIpNumReassDgrams                                                  = 4013,
    /// TCP-IP-IP-REASS-TIMEOUT
    TcpIpIpReassTimeout                                                    = 3520,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-COUNT
    TcpIpIpReassemblyBufferCount                                           = 6242,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-SIZE
    TcpIpIpReassemblyBufferSize                                            = 1333,
    /// TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT
    TcpIpIpReassemblySegmentCount                                          = 2847,
    /// TCP-IP-IP-REASSEMBLY-TIMEOUT
    TcpIpIpReassemblyTimeout                                               = 4693,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT
    TcpIpIpTxFragmentBufferCount                                           = 5309,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE
    TcpIpIpTxFragmentBufferSize                                            = 4491,
    /// TCP-IP-NDP-DEFAULT-REACHABLE-TIME
    TcpIpNdpDefaultReachableTime                                           = 2547,
    /// TCP-IP-NDP-DEFAULT-RETRANS-TIMER
    TcpIpNdpDefaultRetransTimer                                            = 3250,
    /// TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE
    TcpIpNdpDefaultRouterListSize                                          = 2264,
    /// TCP-IP-NDP-DEFENSIVE-PROCESSING
    TcpIpNdpDefensiveProcessing                                            = 1238,
    /// TCP-IP-NDP-DELAY-FIRST-PROBE-TIME
    TcpIpNdpDelayFirstProbeTime                                            = 2818,
    /// TCP-IP-NDP-DELAY-FIRST-PROBE-TIME-VALUE
    TcpIpNdpDelayFirstProbeTimeValue                                       = 251,
    /// TCP-IP-NDP-DESTINATION-CACHE-SIZE
    TcpIpNdpDestinationCacheSize                                           = 2362,
    /// TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED
    TcpIpNdpDynamicHopLimitEnabled                                         = 1782,
    /// TCP-IP-NDP-DYNAMIC-MTU-ENABLED
    TcpIpNdpDynamicMtuEnabled                                              = 5664,
    /// TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED
    TcpIpNdpDynamicReachableTimeEnabled                                    = 296,
    /// TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED
    TcpIpNdpDynamicRetransTimeEnabled                                      = 3051,
    /// TCP-IP-NDP-MAX-RANDOM-FACTOR
    TcpIpNdpMaxRandomFactor                                                = 4053,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY
    TcpIpNdpMaxRtrSolicitationDelay                                        = 3920,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATIONS
    TcpIpNdpMaxRtrSolicitations                                            = 4320,
    /// TCP-IP-NDP-MIN-RANDOM-FACTOR
    TcpIpNdpMinRandomFactor                                                = 2116,
    /// TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED
    TcpIpNdpNeighborUnreachabilityDetectionEnabled                         = 682,
    /// TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS
    TcpIpNdpNumMulticastSolicitations                                      = 5895,
    /// TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS
    TcpIpNdpNumUnicastSolicitations                                        = 2607,
    /// TCP-IP-NDP-PACKET-QUEUE-ENABLED
    TcpIpNdpPacketQueueEnabled                                             = 743,
    /// TCP-IP-NDP-PREFIX-LIST-SIZE
    TcpIpNdpPrefixListSize                                                 = 6046,
    /// TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED
    TcpIpNdpRandomReachableTimeEnabled                                     = 3550,
    /// TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED
    TcpIpNdpRndRtrSolicitationDelayEnabled                                 = 4073,
    /// TCP-IP-NDP-RTR-SOLICITATION-INTERVAL
    TcpIpNdpRtrSolicitationInterval                                        = 3219,
    /// TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS
    TcpIpNdpSlaacDadNumberOfTransmissions                                  = 540,
    /// TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY
    TcpIpNdpSlaacDadRetransmissionDelay                                    = 4669,
    /// TCP-IP-NDP-SLAAC-DELAY-ENABLED
    TcpIpNdpSlaacDelayEnabled                                              = 3458,
    /// TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED
    TcpIpNdpSlaacOptimisticDadEnabled                                      = 5602,
    /// TCP-IP-PROPS-REF
    TcpIpPropsRef                                                          = 2714,
    /// TCP-IP-PROPS-REFS
    TcpIpPropsRefs                                                         = 3515,
    /// TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC
    TcpIpTlsUseSecurityExtensionForceEncryptThenMac                        = 2159,
    /// TCP-KEEP-ALIVE-ENABLED
    TcpKeepAliveEnabled                                                    = 1604,
    /// TCP-KEEP-ALIVE-INTERVAL
    TcpKeepAliveInterval                                                   = 2180,
    /// TCP-KEEP-ALIVE-PROBES-MAX
    TcpKeepAliveProbesMax                                                  = 4851,
    /// TCP-KEEP-ALIVE-TIME
    TcpKeepAliveTime                                                       = 2234,
    /// TCP-LOCAL-PORT
    TcpLocalPort                                                           = 2952,
    /// TCP-MAX-RTX
    TcpMaxRtx                                                              = 5774,
    /// TCP-MSL
    TcpMsl                                                                 = 3178,
    /// TCP-NAGLE-ENABLED
    TcpNagleEnabled                                                        = 1862,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 832,
    /// TCP-OPTION-FILTER-LISTS
    TcpOptionFilterLists                                                   = 726,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 3129,
    /// TCP-PORT
    TcpPort                                                                = 2807,
    /// TCP-PORT-REF
    TcpPortRef                                                             = 5458,
    /// TCP-PROPS
    TcpProps                                                               = 3746,
    /// TCP-RECEIVE-WINDOW-MAX
    TcpReceiveWindowMax                                                    = 2457,
    /// TCP-REMOTE-PORT
    TcpRemotePort                                                          = 3284,
    /// TCP-RETRANSMISSION-TIMEOUT
    TcpRetransmissionTimeout                                               = 2827,
    /// TCP-ROLE
    TcpRole                                                                = 1406,
    /// TCP-RULE
    TcpRule                                                                = 5902,
    /// TCP-SLOW-START-ENABLED
    TcpSlowStartEnabled                                                    = 276,
    /// TCP-SYN-MAX-RTX
    TcpSynMaxRtx                                                           = 897,
    /// TCP-SYN-RECEIVED-TIMEOUT
    TcpSynReceivedTimeout                                                  = 2865,
    /// TCP-TP
    TcpTp                                                                  = 2833,
    /// TCP-TP-CONFIG
    TcpTpConfig                                                            = 5948,
    /// TCP-TP-PORT
    TcpTpPort                                                              = 4471,
    /// TCP-TTL
    TcpTtl                                                                 = 1675,
    /// TCP-UDP-CONFIG
    TcpUdpConfig                                                           = 2355,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 853,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 4840,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 2011,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS
    TdCpSoftwareClusterResourceToTdMappings                                = 4500,
    /// TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS
    TdCpSoftwareClusterToTdMappings                                        = 3735,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 1922,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE
    TdEventBswInternalBehaviorType                                         = 5721,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 4653,
    /// TD-EVENT-BSW-MODE-DECLARATION-TYPE
    TdEventBswModeDeclarationType                                          = 1127,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 2649,
    /// TD-EVENT-BSW-MODULE-TYPE
    TdEventBswModuleType                                                   = 4733,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 1001,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 5898,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 1330,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 2069,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 5660,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 5316,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 2757,
    /// TD-EVENT-MODE-DECLARATION-TYPE
    TdEventModeDeclarationType                                             = 5414,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 1741,
    /// TD-EVENT-OPERATION-TYPE
    TdEventOperationType                                                   = 1418,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 5507,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE
    TdEventServiceInstanceDiscoveryType                                    = 3599,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 2145,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE
    TdEventServiceInstanceEventType                                        = 3444,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 4749,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE
    TdEventServiceInstanceFieldType                                        = 5502,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 3087,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE
    TdEventServiceInstanceMethodType                                       = 5341,
    /// TD-EVENT-SLLET-PORT
    TdEventSlletPort                                                       = 2743,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 4021,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 724,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE
    TdEventSwcInternalBehaviorType                                         = 680,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 5744,
    /// TD-EVENT-TRIGGER-TYPE
    TdEventTriggerType                                                     = 1307,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 3159,
    /// TD-EVENT-TYPE
    TdEventType                                                            = 315,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 6256,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE
    TdEventVariableDataPrototypeType                                       = 5868,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 5096,
    /// TD-HEADER-ID-FILTERS
    TdHeaderIdFilters                                                      = 3542,
    /// TD-HEADER-ID-RANGE
    TdHeaderIdRange                                                        = 5642,
    /// TD-PDU-TRIGGERING-FILTER-REF
    TdPduTriggeringFilterRef                                               = 2072,
    /// TD-PDU-TRIGGERING-FILTER-REFS
    TdPduTriggeringFilterRefs                                              = 4266,
    /// TDLET-ZONE-CLOCK
    TdletZoneClock                                                         = 1318,
    /// TEAR-DOWN-ACTION-REF
    TearDownActionRef                                                      = 3108,
    /// TEAR-DOWN-ACTION-REFS
    TearDownActionRefs                                                     = 5996,
    /// TEMPERATURE-EXP
    TemperatureExp                                                         = 5173,
    /// TEMPLATE-ARGUMENTS
    TemplateArguments                                                      = 5028,
    /// TEMPLATE-TYPE-REF
    TemplateTypeRef                                                        = 806,
    /// TERMINATING-CHECKPOINT-REF
    TerminatingCheckpointRef                                               = 1804,
    /// TERMINATING-CHECKPOINT-TIMEOUT-UNTIL-TERMINATION
    TerminatingCheckpointTimeoutUntilTermination                           = 320,
    /// TERMINATION-BEHAVIOR
    TerminationBehavior                                                    = 133,
    /// TEST-ID
    TestId                                                                 = 3719,
    /// TEST-ID-REF
    TestIdRef                                                              = 3727,
    /// TEST-IDENTIFIER
    TestIdentifier                                                         = 5393,
    /// TEST-PATTERN
    TestPattern                                                            = 5097,
    /// TEST-RESULT-REF
    TestResultRef                                                          = 6254,
    /// TESTED-ITEM-REF
    TestedItemRef                                                          = 2303,
    /// TESTED-ITEM-REFS
    TestedItemRefs                                                         = 1161,
    /// TEX-MATH
    TexMath                                                                = 728,
    /// TEXT
    Text                                                                   = 4064,
    /// TEXT-TABLE-MAPPING
    TextTableMapping                                                       = 1654,
    /// TEXT-TABLE-MAPPINGS
    TextTableMappings                                                      = 1216,
    /// TEXT-TABLE-VALUE-PAIR
    TextTableValuePair                                                     = 3234,
    /// TEXT-VALUE-SPECIFICATION
    TextValueSpecification                                                 = 3658,
    /// TEXTUAL-CONDITION
    TextualCondition                                                       = 4948,
    /// TFOOT
    Tfoot                                                                  = 389,
    /// TGROUP
    Tgroup                                                                 = 1381,
    /// THEAD
    Thead                                                                  = 1121,
    /// THRESHOLD
    Threshold                                                              = 5687,
    /// THRESHOLD-NUMBER
    ThresholdNumber                                                        = 4836,
    /// THRESHOLD-SIZE
    ThresholdSize                                                          = 1224,
    /// TIME-BASE
    TimeBase                                                               = 2831,
    /// TIME-BASE-CONSUMER-REF
    TimeBaseConsumerRef                                                    = 2841,
    /// TIME-BASE-JITTER
    TimeBaseJitter                                                         = 3763,
    /// TIME-BASE-KIND
    TimeBaseKind                                                           = 4417,
    /// TIME-BASE-PROVIDER-REF
    TimeBaseProviderRef                                                    = 3991,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 5467,
    /// TIME-BASE-RESOURCE-REF
    TimeBaseResourceRef                                                    = 4562,
    /// TIME-BASE-RESOURCE-REF-CONDITIONAL
    TimeBaseResourceRefConditional                                         = 1551,
    /// TIME-BASE-RESOURCE-REFS
    TimeBaseResourceRefs                                                   = 620,
    /// TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE
    TimeBasedFdcThresholdStorageValue                                      = 2274,
    /// TIME-BASES
    TimeBases                                                              = 4773,
    /// TIME-BR
    TimeBr                                                                 = 2614,
    /// TIME-BUFFER
    TimeBuffer                                                             = 5373,
    /// TIME-CS
    TimeCs                                                                 = 3238,
    /// TIME-EXP
    TimeExp                                                                = 3200,
    /// TIME-FAILED-THRESHOLD
    TimeFailedThreshold                                                    = 1272,
    /// TIME-FR-IF
    TimeFrIf                                                               = 6014,
    /// TIME-HARDWARE-CORRECTION-THRESHOLD
    TimeHardwareCorrectionThreshold                                        = 720,
    /// TIME-INTERVAL
    TimeInterval                                                           = 348,
    /// TIME-LEAP-FUTURE-THRESHOLD
    TimeLeapFutureThreshold                                                = 2663,
    /// TIME-LEAP-HEALING-COUNTER
    TimeLeapHealingCounter                                                 = 3285,
    /// TIME-LEAP-PAST-THRESHOLD
    TimeLeapPastThreshold                                                  = 3431,
    /// TIME-MARK
    TimeMark                                                               = 3315,
    /// TIME-MASTER-PRIORITY
    TimeMasterPriority                                                     = 2481,
    /// TIME-OFFSET
    TimeOffset                                                             = 2333,
    /// TIME-PASSED-THRESHOLD
    TimePassedThreshold                                                    = 2341,
    /// TIME-PERIOD
    TimePeriod                                                             = 2523,
    /// TIME-SEG-1
    TimeSeg1                                                               = 3843,
    /// TIME-SEG-2
    TimeSeg2                                                               = 2926,
    /// TIME-SUB-TLV
    TimeSubTlv                                                             = 3181,
    /// TIME-SYNC-CLIENT
    TimeSyncClient                                                         = 1831,
    /// TIME-SYNC-CORRECTION
    TimeSyncCorrection                                                     = 1840,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 1147,
    /// TIME-SYNC-P-PORT-PROTOTYPE-IREF
    TimeSyncPPortPrototypeIref                                             = 3933,
    /// TIME-SYNC-PORT-PROTOTYPE-IREF
    TimeSyncPortPrototypeIref                                              = 5495,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 770,
    /// TIME-SYNC-R-PORT-PROTOTYPE-IREF
    TimeSyncRPortPrototypeIref                                             = 4370,
    /// TIME-SYNC-SERVER
    TimeSyncServer                                                         = 2999,
    /// TIME-SYNC-SERVER-IDENTIFIER
    TimeSyncServerIdentifier                                               = 3798,
    /// TIME-SYNC-SERVER-REF
    TimeSyncServerRef                                                      = 4613,
    /// TIME-SYNC-TECHNOLOGY
    TimeSyncTechnology                                                     = 835,
    /// TIME-SYNCHRONIZATION
    TimeSynchronization                                                    = 2008,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 3757,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 2778,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 661,
    /// TIME-TO-LIVE
    TimeToLive                                                             = 3950,
    /// TIME-TRIGGERED-CAN-LEVEL
    TimeTriggeredCanLevel                                                  = 3470,
    /// TIME-VALUE
    TimeValue                                                              = 2736,
    /// TIME-VALUE-VALUE-VARIATION-POINT
    TimeValueValueVariationPoint                                           = 2371,
    /// TIMEOUT
    Timeout                                                                = 2731,
    /// TIMEOUT-AR
    TimeoutAr                                                              = 1964,
    /// TIMEOUT-AS
    TimeoutAs                                                              = 4664,
    /// TIMEOUT-BR
    TimeoutBr                                                              = 618,
    /// TIMEOUT-BS
    TimeoutBs                                                              = 2967,
    /// TIMEOUT-CHECK
    TimeoutCheck                                                           = 5322,
    /// TIMEOUT-CR
    TimeoutCr                                                              = 5117,
    /// TIMEOUT-CS
    TimeoutCs                                                              = 2428,
    /// TIMEOUT-SUBSTITUTION-VALUE
    TimeoutSubstitutionValue                                               = 4916,
    /// TIMESTAMP-FORMAT
    TimestampFormat                                                        = 5501,
    /// TIMESTAMP-INTERVAL
    TimestampInterval                                                      = 74,
    /// TIMESTAMP-RX-ACCEPTANCE-WINDOW
    TimestampRxAcceptanceWindow                                            = 5992,
    /// TIMESTAMP-SUPPORT
    TimestampSupport                                                       = 2885,
    /// TIMING-ARGUMENT-REF
    TimingArgumentRef                                                      = 6283,
    /// TIMING-ARGUMENTS
    TimingArguments                                                        = 2712,
    /// TIMING-CLOCK-SYNC-ACCURACY
    TimingClockSyncAccuracy                                                = 3450,
    /// TIMING-CLOCK-SYNC-ACCURACYS
    TimingClockSyncAccuracys                                               = 3194,
    /// TIMING-CLOCKS
    TimingClocks                                                           = 3143,
    /// TIMING-CONDITION
    TimingCondition                                                        = 6090,
    /// TIMING-CONDITION-FORMULA
    TimingConditionFormula                                                 = 4729,
    /// TIMING-CONDITION-REF
    TimingConditionRef                                                     = 493,
    /// TIMING-CONDITIONS
    TimingConditions                                                       = 2320,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 1581,
    /// TIMING-DESCRIPTION-REF
    TimingDescriptionRef                                                   = 2084,
    /// TIMING-DESCRIPTIONS
    TimingDescriptions                                                     = 2263,
    /// TIMING-EVENT
    TimingEvent                                                            = 999,
    /// TIMING-EVENT-REF
    TimingEventRef                                                         = 3556,
    /// TIMING-GUARANTEES
    TimingGuarantees                                                       = 5007,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 6141,
    /// TIMING-MODE-REF
    TimingModeRef                                                          = 1499,
    /// TIMING-MODES
    TimingModes                                                            = 3249,
    /// TIMING-PROPS
    TimingProps                                                            = 1365,
    /// TIMING-REQUIREMENTS
    TimingRequirements                                                     = 5090,
    /// TIMING-RESOURCE
    TimingResource                                                         = 5699,
    /// TIMING-VARIABLE-REF
    TimingVariableRef                                                      = 673,
    /// TIMING-VARIABLES
    TimingVariables                                                        = 4635,
    /// TLS-CIPHER-SUITE
    TlsCipherSuite                                                         = 5561,
    /// TLS-CIPHER-SUITES
    TlsCipherSuites                                                        = 1735,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 2186,
    /// TLS-CONNECTION-REF
    TlsConnectionRef                                                       = 3009,
    /// TLS-CONNECTION-REFS
    TlsConnectionRefs                                                      = 5552,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 1845,
    /// TLS-CRYPTO-MAPPING-REF
    TlsCryptoMappingRef                                                    = 2759,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 2521,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 4367,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 5364,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 1555,
    /// TLS-JOB-MAPPINGS
    TlsJobMappings                                                         = 3938,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 3069,
    /// TLS-JOB-REQUIREMENT-REF
    TlsJobRequirementRef                                                   = 2000,
    /// TLS-JOB-SEMANTIC
    TlsJobSemantic                                                         = 4763,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 4112,
    /// TLS-SECURE-COM-PROPS-REF
    TlsSecureComPropsRef                                                   = 4059,
    /// TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF
    TlvAppRecordElementInMethodIref                                        = 3800,
    /// TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    TlvAppRecordElementInServiceInterfaceIref                              = 1018,
    /// TLV-ARGUMENT-REF
    TlvArgumentRef                                                         = 5236,
    /// TLV-DATA-ID
    TlvDataId                                                              = 4378,
    /// TLV-DATA-ID-0-REF
    TlvDataId0Ref                                                          = 958,
    /// TLV-DATA-ID-0-REFS
    TlvDataId0Refs                                                         = 4554,
    /// TLV-DATA-ID-DEFINITION
    TlvDataIdDefinition                                                    = 6182,
    /// TLV-DATA-ID-DEFINITION-REF
    TlvDataIdDefinitionRef                                                 = 1287,
    /// TLV-DATA-ID-DEFINITION-REFS
    TlvDataIdDefinitionRefs                                                = 3318,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 4919,
    /// TLV-DATA-ID-DEFINITIONS
    TlvDataIdDefinitions                                                   = 4832,
    /// TLV-DATA-IDS
    TlvDataIds                                                             = 3206,
    /// TLV-IMPL-RECORD-ELEMENT
    TlvImplRecordElement                                                   = 4279,
    /// TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TlvImplementationDataTypeElementRef                                    = 5906,
    /// TLV-RECORD-ELEMENT-REF
    TlvRecordElementRef                                                    = 3850,
    /// TLV-SUB-ELEMENT-REF
    TlvSubElementRef                                                       = 3020,
    /// TO-REF
    ToRef                                                                  = 4852,
    /// TOL
    Tol                                                                    = 1218,
    /// TOLERANCE
    Tolerance                                                              = 3639,
    /// TOLERATED-FAILED-CYCLES
    ToleratedFailedCycles                                                  = 499,
    /// TOOL
    Tool                                                                   = 3693,
    /// TOOL-VERSION
    ToolVersion                                                            = 1778,
    /// TOPIC-1
    Topic1                                                                 = 1606,
    /// TOPIC-DATA
    TopicData                                                              = 6013,
    /// TOPIC-NAME
    TopicName                                                              = 1189,
    /// TP-ADDRESS
    TpAddress                                                              = 734,
    /// TP-ADDRESS-EXTENSION-VALUE
    TpAddressExtensionValue                                                = 969,
    /// TP-ADDRESS-REF
    TpAddressRef                                                           = 117,
    /// TP-ADDRESS-REFS
    TpAddressRefs                                                          = 591,
    /// TP-ADDRESSS
    TpAddresss                                                             = 1939,
    /// TP-CHANNEL-REF
    TpChannelRef                                                           = 709,
    /// TP-CHANNELS
    TpChannels                                                             = 4355,
    /// TP-CONFIGURATION
    TpConfiguration                                                        = 4793,
    /// TP-CONNECTION-CONTROL-REF
    TpConnectionControlRef                                                 = 5570,
    /// TP-CONNECTION-CONTROLS
    TpConnectionControls                                                   = 5830,
    /// TP-CONNECTIONS
    TpConnections                                                          = 4877,
    /// TP-ECU
    TpEcu                                                                  = 234,
    /// TP-ECUS
    TpEcus                                                                 = 5589,
    /// TP-NODES
    TpNodes                                                                = 79,
    /// TP-PGS
    TpPgs                                                                  = 627,
    /// TP-SDU-REF
    TpSduRef                                                               = 4746,
    /// TP-SDU-REFS
    TpSduRefs                                                              = 3602,
    /// TP-TECHNOLOGY
    TpTechnology                                                           = 957,
    /// TRACE
    Trace                                                                  = 665,
    /// TRACE-MESSAGE-REF
    TraceMessageRef                                                        = 204,
    /// TRACE-REF
    TraceRef                                                               = 760,
    /// TRACE-REFS
    TraceRefs                                                              = 4879,
    /// TRACE-SWITCH
    TraceSwitch                                                            = 479,
    /// TRACE-SWITCH-CONFIGURATION
    TraceSwitchConfiguration                                               = 2324,
    /// TRACE-SWITCH-CONFIGURATIONS
    TraceSwitchConfigurations                                              = 197,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 2218,
    /// TRACED-FAILURES
    TracedFailures                                                         = 6263,
    /// TRAFFIC-CLASS
    TrafficClass                                                           = 3252,
    /// TRAFFIC-LIMITATION-FILTERS
    TrafficLimitationFilters                                               = 3505,
    /// TRAFFIC-SHAPER-GROUP-REF
    TrafficShaperGroupRef                                                  = 4857,
    /// TRAFFIC-SHAPER-GROUPS
    TrafficShaperGroups                                                    = 4968,
    /// TRANCEIVER-STANDBY-DELAY
    TranceiverStandbyDelay                                                 = 1819,
    /// TRANSFER-EXIT-CLASS-REF
    TransferExitClassRef                                                   = 3170,
    /// TRANSFER-PROPERTY
    TransferProperty                                                       = 2219,
    /// TRANSFER-REF
    TransferRef                                                            = 4890,
    /// TRANSFER-REFS
    TransferRefs                                                           = 2953,
    /// TRANSFERS
    Transfers                                                              = 6083,
    /// TRANSFORMATION-COM-SPEC-PROPSS
    TransformationComSpecPropss                                            = 1096,
    /// TRANSFORMATION-DESCRIPTIONS
    TransformationDescriptions                                             = 274,
    /// TRANSFORMATION-I-SIGNAL-PROPSS
    TransformationISignalPropss                                            = 4228,
    /// TRANSFORMATION-PROPS-MAPPING-SET-REF
    TransformationPropsMappingSetRef                                       = 3484,
    /// TRANSFORMATION-PROPS-REF
    TransformationPropsRef                                                 = 3395,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 4889,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 1052,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 4109,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING
    TransformationPropsToServiceInterfaceMapping                           = 1446,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 1211,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF
    TransformationPropsToServiceInterfaceMappingSetRef                     = 4771,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS
    TransformationPropsToServiceInterfaceMappings                          = 6188,
    /// TRANSFORMATION-PROPSS
    TransformationPropss                                                   = 1045,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 200,
    /// TRANSFORMATION-TECHNOLOGYS
    TransformationTechnologys                                              = 5850,
    /// TRANSFORMER-CHAIN-REF
    TransformerChainRef                                                    = 1391,
    /// TRANSFORMER-CHAIN-REFS
    TransformerChainRefs                                                   = 5634,
    /// TRANSFORMER-CLASS
    TransformerClass                                                       = 3594,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 5308,
    /// TRANSFORMER-REF
    TransformerRef                                                         = 3406,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 2486,
    /// TRANSFORMING-SYSTEM-SIGNAL-REF
    TransformingSystemSignalRef                                            = 2760,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 1882,
    /// TRANSIT-TO-INVALID-EXTENDED
    TransitToInvalidExtended                                               = 6210,
    /// TRANSITION-REF
    TransitionRef                                                          = 5281,
    /// TRANSITION-REFS
    TransitionRefs                                                         = 5166,
    /// TRANSITIONS
    Transitions                                                            = 5245,
    /// TRANSLATION-TARGET-IREF
    TranslationTargetIref                                                  = 668,
    /// TRANSMISSION-ACKNOWLEDGE
    TransmissionAcknowledge                                                = 5146,
    /// TRANSMISSION-DEADLINE
    TransmissionDeadline                                                   = 1339,
    /// TRANSMISSION-MODE
    TransmissionMode                                                       = 4138,
    /// TRANSMISSION-MODE-CONDITION
    TransmissionModeCondition                                              = 1952,
    /// TRANSMISSION-MODE-CONDITIONS
    TransmissionModeConditions                                             = 3357,
    /// TRANSMISSION-MODE-DECLARATION
    TransmissionModeDeclaration                                            = 5885,
    /// TRANSMISSION-MODE-FALSE-TIMING
    TransmissionModeFalseTiming                                            = 4528,
    /// TRANSMISSION-MODE-TRUE-TIMING
    TransmissionModeTrueTiming                                             = 5556,
    /// TRANSMISSION-PROPS
    TransmissionProps                                                      = 5306,
    /// TRANSMISSION-START-SEQUENCE-DURATION
    TransmissionStartSequenceDuration                                      = 4497,
    /// TRANSMISSION-TRIGGER
    TransmissionTrigger                                                    = 2181,
    /// TRANSMIT-CANCELLATION
    TransmitCancellation                                                   = 658,
    /// TRANSMIT-PDU-REF
    TransmitPduRef                                                         = 4826,
    /// TRANSMIT-PDU-REFS
    TransmitPduRefs                                                        = 4534,
    /// TRANSMITTER-REF
    TransmitterRef                                                         = 3031,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 2471,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 221,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF
    TransportLayerIndependentInstanceIdRef                                 = 1336,
    /// TRANSPORT-LAYER-RULE
    TransportLayerRule                                                     = 2622,
    /// TRANSPORT-PDU-REF
    TransportPduRef                                                        = 4183,
    /// TRANSPORT-PLUGIN
    TransportPlugin                                                        = 3374,
    /// TRANSPORT-PLUGINS
    TransportPlugins                                                       = 4753,
    /// TRANSPORT-PRIORITY
    TransportPriority                                                      = 5682,
    /// TRANSPORT-PROTOCOL
    TransportProtocol                                                      = 3605,
    /// TRANSPORT-PROTOCOLS
    TransportProtocols                                                     = 5637,
    /// TRCV-DELAY-COMPENSATION-OFFSET
    TrcvDelayCompensationOffset                                            = 1683,
    /// TRCV-PWM-MODE-ENABLED
    TrcvPwmModeEnabled                                                     = 6157,
    /// TRIGGER
    Trigger                                                                = 4725,
    /// TRIGGER-DIRECT-IMPLEMENTATIONS
    TriggerDirectImplementations                                           = 2587,
    /// TRIGGER-I-PDU-SEND-CONDITION
    TriggerIPduSendCondition                                               = 3074,
    /// TRIGGER-I-PDU-SEND-CONDITIONS
    TriggerIPduSendConditions                                              = 3633,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 6178,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 66,
    /// TRIGGER-IREF
    TriggerIref                                                            = 1602,
    /// TRIGGER-MAPPING
    TriggerMapping                                                         = 483,
    /// TRIGGER-MAPPINGS
    TriggerMappings                                                        = 3575,
    /// TRIGGER-MODE
    TriggerMode                                                            = 4950,
    /// TRIGGER-PERIOD
    TriggerPeriod                                                          = 4518,
    /// TRIGGER-PORT-ANNOTATION
    TriggerPortAnnotation                                                  = 2381,
    /// TRIGGER-PORT-ANNOTATIONS
    TriggerPortAnnotations                                                 = 5988,
    /// TRIGGER-REF
    TriggerRef                                                             = 77,
    /// TRIGGER-REF-CONDITIONAL
    TriggerRefConditional                                                  = 6111,
    /// TRIGGER-REFS
    TriggerRefs                                                            = 1411,
    /// TRIGGER-TO-SIGNAL-MAPPING
    TriggerToSignalMapping                                                 = 425,
    /// TRIGGERING-EVENT-REF
    TriggeringEventRef                                                     = 841,
    /// TRIGGERS
    Triggers                                                               = 5814,
    /// TRIGGERS-RECOVERY-NOTIFICATION
    TriggersRecoveryNotification                                           = 5418,
    /// TROUBLE-CODE-J-1939-REF
    TroubleCodeJ1939Ref                                                    = 2426,
    /// TROUBLE-CODE-OBD-REF
    TroubleCodeObdRef                                                      = 2258,
    /// TROUBLE-CODE-UDS-REF
    TroubleCodeUdsRef                                                      = 4543,
    /// TRUE-ACTION-LIST-REF
    TrueActionListRef                                                      = 1367,
    /// TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR
    TrustedPlatformExecutableLaunchBehavior                                = 28,
    /// TT
    Tt                                                                     = 818,
    /// TT-CAN-CLUSTER-REF
    TtCanClusterRef                                                        = 5410,
    /// TTCAN-ABSOLUTELY-SCHEDULED-TIMING
    TtcanAbsolutelyScheduledTiming                                         = 5590,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 2405,
    /// TTCAN-CLUSTER-CONDITIONAL
    TtcanClusterConditional                                                = 3477,
    /// TTCAN-CLUSTER-VARIANTS
    TtcanClusterVariants                                                   = 4651,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 4241,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 4232,
    /// TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    TtcanCommunicationControllerConditional                                = 5430,
    /// TTCAN-COMMUNICATION-CONTROLLER-VARIANTS
    TtcanCommunicationControllerVariants                                   = 3782,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 2365,
    /// TTL
    Ttl                                                                    = 4737,
    /// TTL-MAX
    TtlMax                                                                 = 245,
    /// TTL-MIN
    TtlMin                                                                 = 2929,
    /// TWO-KEY-SLOT-MODE
    TwoKeySlotMode                                                         = 942,
    /// TX-BIT-RATE-SWITCH
    TxBitRateSwitch                                                        = 1567,
    /// TX-ENABLE-WINDOW-LENGTH
    TxEnableWindowLength                                                   = 4157,
    /// TX-MASK
    TxMask                                                                 = 4240,
    /// TX-NM-PDU-REF
    TxNmPduRef                                                             = 6191,
    /// TX-NM-PDU-REFS
    TxNmPduRefs                                                            = 311,
    /// TX-PDU-POOL-REF
    TxPduPoolRef                                                           = 2208,
    /// TX-PDU-TRIGGERING-REF
    TxPduTriggeringRef                                                     = 1028,
    /// TYP
    Typ                                                                    = 5665,
    /// TYPE
    Type                                                                   = 2311,
    /// TYPE-APPROVAL
    TypeApproval                                                           = 4147,
    /// TYPE-BLUEPRINTS
    TypeBlueprints                                                         = 5195,
    /// TYPE-DEFINITION
    TypeDefinition                                                         = 3307,
    /// TYPE-EMITTER
    TypeEmitter                                                            = 603,
    /// TYPE-MAPPING
    TypeMapping                                                            = 3107,
    /// TYPE-OF-DTC-SUPPORTED
    TypeOfDtcSupported                                                     = 4796,
    /// TYPE-OF-EVENT-COMBINATION-SUPPORTED
    TypeOfEventCombinationSupported                                        = 4559,
    /// TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION
    TypeOfFreezeFrameRecordNumeration                                      = 3632,
    /// TYPE-REFERENCE
    TypeReference                                                          = 2516,
    /// TYPE-REFERENCE-REF
    TypeReferenceRef                                                       = 4269,
    /// TYPE-TAILORINGS
    TypeTailorings                                                         = 2191,
    /// TYPE-TREF
    TypeTref                                                               = 1787,
    /// UAS-ID
    UasId                                                                  = 299,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 2393,
    /// UCM-MASTER-FALLBACK-REF
    UcmMasterFallbackRef                                                   = 2690,
    /// UCM-MASTER-FALLBACK-REFS
    UcmMasterFallbackRefs                                                  = 1054,
    /// UCM-MASTER-MODULE-INSTANTIATION
    UcmMasterModuleInstantiation                                           = 698,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 2014,
    /// UCM-MODULE-INSTANTIATION-REF
    UcmModuleInstantiationRef                                              = 217,
    /// UCM-PROCESSINGS
    UcmProcessings                                                         = 2179,
    /// UCM-REF
    UcmRef                                                                 = 4529,
    /// UCM-SOFTWARE-VERSION
    UcmSoftwareVersion                                                     = 4126,
    /// UCM-STEP
    UcmStep                                                                = 4798,
    /// UCM-SUBORDINATE-MODULE-INSTANTIATION
    UcmSubordinateModuleInstantiation                                      = 5346,
    /// UCM-TO-TIME-BASE-RESOURCE-MAPPING
    UcmToTimeBaseResourceMapping                                           = 3734,
    /// UCMS
    Ucms                                                                   = 2843,
    /// UDP-CHECKSUM-HANDLING
    UdpChecksumHandling                                                    = 4494,
    /// UDP-COLLECTION-BUFFER-SIZE-THRESHOLD
    UdpCollectionBufferSizeThreshold                                       = 5803,
    /// UDP-COLLECTION-BUFFER-TIMEOUT
    UdpCollectionBufferTimeout                                             = 536,
    /// UDP-COLLECTION-TRIGGER
    UdpCollectionTrigger                                                   = 49,
    /// UDP-LOCAL-PORT
    UdpLocalPort                                                           = 3302,
    /// UDP-MIN-TX-BUFFER-SIZE
    UdpMinTxBufferSize                                                     = 5773,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 1765,
    /// UDP-NM-CLUSTER-COUPLING
    UdpNmClusterCoupling                                                   = 3478,
    /// UDP-NM-CLUSTER-REF
    UdpNmClusterRef                                                        = 2398,
    /// UDP-NM-ECU
    UdpNmEcu                                                               = 3076,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 232,
    /// UDP-PORT
    UdpPort                                                                = 5550,
    /// UDP-PORT-REF
    UdpPortRef                                                             = 260,
    /// UDP-PROPS
    UdpProps                                                               = 187,
    /// UDP-REMOTE-PORT
    UdpRemotePort                                                          = 4579,
    /// UDP-RULE
    UdpRule                                                                = 3566,
    /// UDP-TP
    UdpTp                                                                  = 3236,
    /// UDP-TP-PORT
    UdpTpPort                                                              = 3415,
    /// UDP-TTL
    UdpTtl                                                                 = 38,
    /// UDS-DTC-NUMBER
    UdsDtcNumber                                                           = 6085,
    /// UDS-DTC-VALUE
    UdsDtcValue                                                            = 2433,
    /// UDS-SERVICE
    UdsService                                                             = 6073,
    /// UNASSIGN-FRAME-ID
    UnassignFrameId                                                        = 611,
    /// UNASSIGNED-FRAME-TRIGGERING-REF
    UnassignedFrameTriggeringRef                                           = 492,
    /// UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE
    UncompressedSoftwareClusterSize                                        = 2943,
    /// UNICAST-CREDENTIALS
    UnicastCredentials                                                     = 2332,
    /// UNICAST-NETWORK-ENDPOINT-REF
    UnicastNetworkEndpointRef                                              = 4040,
    /// UNICAST-NETWORK-ENDPOINT-REFS
    UnicastNetworkEndpointRefs                                             = 327,
    /// UNICAST-SECURE-COM-PROPS-REF
    UnicastSecureComPropsRef                                               = 3980,
    /// UNICAST-SECURE-COM-PROPS-REFS
    UnicastSecureComPropsRefs                                              = 768,
    /// UNICAST-UDP-CREDENTIALS
    UnicastUdpCredentials                                                  = 5371,
    /// UNIQUE-STREAM-ID
    UniqueStreamId                                                         = 5024,
    /// UNIT
    Unit                                                                   = 261,
    /// UNIT-AND-SCALING-ID
    UnitAndScalingId                                                       = 4914,
    /// UNIT-DISPLAY-NAME
    UnitDisplayName                                                        = 2295,
    /// UNIT-GROUP
    UnitGroup                                                              = 5338,
    /// UNIT-GROUP-REF
    UnitGroupRef                                                           = 151,
    /// UNIT-GROUP-REFS
    UnitGroupRefs                                                          = 5331,
    /// UNIT-REF
    UnitRef                                                                = 19,
    /// UNIT-REFS
    UnitRefs                                                               = 6164,
    /// UNLIMITED-INTEGER-VALUE-VARIATION-POINT
    UnlimitedIntegerValueVariationPoint                                    = 3840,
    /// UNRESOLVED-REFERENCE-RESTRICTION
    UnresolvedReferenceRestriction                                         = 1806,
    /// UNUSED-BIT-PATTERN
    UnusedBitPattern                                                       = 6135,
    /// UPDATE
    Update                                                                 = 5659,
    /// UPDATE-INDICATION-BIT-POSITION
    UpdateIndicationBitPosition                                            = 6132,
    /// UPDATE-KIND
    UpdateKind                                                             = 3283,
    /// UPDATE-SESSION-REJECTED
    UpdateSessionRejected                                                  = 3251,
    /// UPDATE-STRATEGY
    UpdateStrategy                                                         = 1015,
    /// UPPER-BOUND
    UpperBound                                                             = 3646,
    /// UPPER-BOUNDARY
    UpperBoundary                                                          = 4424,
    /// UPPER-CAN-ID
    UpperCanId                                                             = 4571,
    /// UPPER-HEADER-BITS-TO-SHIFT
    UpperHeaderBitsToShift                                                 = 3372,
    /// UPPER-LIMIT
    UpperLimit                                                             = 1596,
    /// UPPER-MULTIPLICITY
    UpperMultiplicity                                                      = 4648,
    /// UPPER-MULTIPLICITY-INFINITE
    UpperMultiplicityInfinite                                              = 3698,
    /// UPPER-REF
    UpperRef                                                               = 6226,
    /// UPSTREAM-REFERENCE-IREF
    UpstreamReferenceIref                                                  = 2330,
    /// URI
    Uri                                                                    = 1236,
    /// URL
    Url                                                                    = 5877,
    /// USAGE
    Usage                                                                  = 4710,
    /// USAGES
    Usages                                                                 = 4715,
    /// USE-AS-CRYPTOGRAPHIC-I-PDU
    UseAsCryptographicIPdu                                                 = 5559,
    /// USE-AUTH-DATA-FRESHNESS
    UseAuthDataFreshness                                                   = 5324,
    /// USE-AUTO-VALIDATION-AT-SHUT-DOWN
    UseAutoValidationAtShutDown                                            = 5385,
    /// USE-CASE
    UseCase                                                                = 1326,
    /// USE-CLIENT-AUTHENTICATION-REQUEST
    UseClientAuthenticationRequest                                         = 2325,
    /// USE-CRC-COMP-MECHANISM
    UseCrcCompMechanism                                                    = 2894,
    /// USE-FRESHNESS-TIMESTAMP
    UseFreshnessTimestamp                                                  = 5722,
    /// USE-INSTEAD-REF
    UseInsteadRef                                                          = 132,
    /// USE-INSTEAD-REFS
    UseInsteadRefs                                                         = 4411,
    /// USE-MAC-ADDRESS-FOR-IDENTIFICATION
    UseMacAddressForIdentification                                         = 6017,
    /// USE-SECURED-PDU-HEADER
    UseSecuredPduHeader                                                    = 5151,
    /// USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT
    UseSecurityExtensionRecordSizeLimit                                    = 3226,
    /// USE-SMART-SENSOR-API
    UseSmartSensorApi                                                      = 1401,
    /// USE-VEHICLE-IDENTIFICATION-SYNC-STATUS
    UseVehicleIdentificationSyncStatus                                     = 930,
    /// USED-CODE-GENERATOR
    UsedCodeGenerator                                                      = 885,
    /// USED-DATA-ELEMENT
    UsedDataElement                                                        = 1117,
    /// USED-FID-REF
    UsedFidRef                                                             = 4012,
    /// USED-IMPLEMENTATION-DATA-TYPE-REF
    UsedImplementationDataTypeRef                                          = 6127,
    /// USED-IMPOSITION-TIME-DEFINITION-GROUP-REF
    UsedImpositionTimeDefinitionGroupRef                                   = 2698,
    /// USED-LANGUAGES
    UsedLanguages                                                          = 2438,
    /// USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF
    UsedLifeCycleStateDefinitionGroupRef                                   = 3091,
    /// USED-PARAMETER-ELEMENT
    UsedParameterElement                                                   = 5164,
    /// USED-PIM-REF
    UsedPimRef                                                             = 6118,
    /// USED-SECONDARY-FID-REF
    UsedSecondaryFidRef                                                    = 5073,
    /// USED-SECONDARY-FID-REFS
    UsedSecondaryFidRefs                                                   = 3598,
    /// USER-DATA-SUB-TLV
    UserDataSubTlv                                                         = 5951,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 2571,
    /// USER-DEFINED-CLUSTER-CONDITIONAL
    UserDefinedClusterConditional                                          = 192,
    /// USER-DEFINED-CLUSTER-VARIANTS
    UserDefinedClusterVariants                                             = 3339,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 2276,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 4303,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL
    UserDefinedCommunicationControllerConditional                          = 165,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS
    UserDefinedCommunicationControllerVariants                             = 3313,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 1022,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 1742,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 828,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 4985,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 2382,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 1394,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 5168,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 1924,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 224,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 521,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 1712,
    /// USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS
    UserDefinedTransformationComSpecProps                                  = 541,
    /// USER-DEFINED-TRANSFORMATION-DESCRIPTION
    UserDefinedTransformationDescription                                   = 5444,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS
    UserDefinedTransformationISignalProps                                  = 3326,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    UserDefinedTransformationISignalPropsConditional                       = 2699,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    UserDefinedTransformationISignalPropsVariants                          = 58,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 5199,
    /// USES-ADDRESS-ARBITRATION
    UsesAddressArbitration                                                 = 3835,
    /// USES-DYNAMIC-ADDRESSING
    UsesDynamicAddressing                                                  = 344,
    /// USES-END-TO-END-PROTECTION
    UsesEndToEndProtection                                                 = 3806,
    /// USES-MONITOR-DATA
    UsesMonitorData                                                        = 962,
    /// USES-TIME-BASE-RESOURCE
    UsesTimeBaseResource                                                   = 4333,
    /// V
    V                                                                      = 4992,
    /// V-2-X-DATA-MANAGER-NEEDS
    V2XDataManagerNeeds                                                    = 1169,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 3623,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 4488,
    /// V-2-X-SUPPORTED
    V2XSupported                                                           = 4841,
    /// V-LAN-REF
    VLanRef                                                                = 4284,
    /// V-LAN-REFS
    VLanRefs                                                               = 193,
    /// VALID-BINDING-TIME
    ValidBindingTime                                                       = 6057,
    /// VALID-BINDING-TIMES
    ValidBindingTimes                                                      = 572,
    /// VALIDATION-CONFIGURATION
    ValidationConfiguration                                                = 2408,
    /// VALIDATION-FORMULA
    ValidationFormula                                                      = 5818,
    /// VALIDATION-REF
    ValidationRef                                                          = 4828,
    /// VALIDATION-ROOT
    ValidationRoot                                                         = 4716,
    /// VALUE
    Value                                                                  = 3420,
    /// VALUE-ACCESS
    ValueAccess                                                            = 2232,
    /// VALUE-AXIS-DATA-TYPE-REF
    ValueAxisDataTypeRef                                                   = 3700,
    /// VALUE-CONFIG-CLASSES
    ValueConfigClasses                                                     = 5389,
    /// VALUE-DATA-TYPE-REF
    ValueDataTypeRef                                                       = 3692,
    /// VALUE-IREF
    ValueIref                                                              = 5111,
    /// VALUE-PAIRS
    ValuePairs                                                             = 1095,
    /// VALUE-REF
    ValueRef                                                               = 555,
    /// VALUE-RESTRICTION
    ValueRestriction                                                       = 4726,
    /// VALUE-SPEC
    ValueSpec                                                              = 2976,
    /// VALUE-TYPE-REF
    ValueTypeRef                                                           = 3472,
    /// VALUE-TYPE-TREF
    ValueTypeTref                                                          = 2554,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 4945,
    /// VARIABLE-ACCESS-IREF
    VariableAccessIref                                                     = 1563,
    /// VARIABLE-ACCESS-IREFS
    VariableAccessIrefs                                                    = 2061,
    /// VARIABLE-ACCESS-REF
    VariableAccessRef                                                      = 4741,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REF
    VariableAccessedForDebugRef                                            = 847,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REFS
    VariableAccessedForDebugRefs                                           = 3257,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 2140,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 3492,
    /// VARIABLE-DATA-PROTOTYPE-IREF
    VariableDataPrototypeIref                                              = 2086,
    /// VARIABLE-INSTANCE
    VariableInstance                                                       = 5179,
    /// VARIABLE-INSTANCE-IREF
    VariableInstanceIref                                                   = 2803,
    /// VARIABLE-LENGTH
    VariableLength                                                         = 1202,
    /// VARIABLE-REF
    VariableRef                                                            = 733,
    /// VARIABLES
    Variables                                                              = 795,
    /// VARIANT-CRITERION-REF
    VariantCriterionRef                                                    = 5723,
    /// VARIANT-ID
    VariantId                                                              = 477,
    /// VARIATION
    Variation                                                              = 5011,
    /// VARIATION-POINT
    VariationPoint                                                         = 1894,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 2794,
    /// VARIATION-POINT-PROXYS
    VariationPointProxys                                                   = 2097,
    /// VARIATION-RESTRICTION
    VariationRestriction                                                   = 5882,
    /// VCID
    Vcid                                                                   = 5074,
    /// VEHICLE-ANNOUNCEMENT-COUNT
    VehicleAnnouncementCount                                               = 3925,
    /// VEHICLE-ANNOUNCEMENT-INTERVAL
    VehicleAnnouncementInterval                                            = 2001,
    /// VEHICLE-DESCRIPTION-REF
    VehicleDescriptionRef                                                  = 2503,
    /// VEHICLE-DRIVER-NOTIFICATION
    VehicleDriverNotification                                              = 2327,
    /// VEHICLE-IDENTIFICATION-SYNC-STATUS
    VehicleIdentificationSyncStatus                                        = 4337,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 5484,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 5975,
    /// VEHICLE-SYSTEM
    VehicleSystem                                                          = 4623,
    /// VEHICLE-SYSTEM-INSTANCE
    VehicleSystemInstance                                                  = 5292,
    /// VENDOR
    Vendor                                                                 = 732,
    /// VENDOR-API-INFIX
    VendorApiInfix                                                         = 263,
    /// VENDOR-ID
    VendorId                                                               = 5315,
    /// VENDOR-SIGNATURE-REF
    VendorSignatureRef                                                     = 3155,
    /// VENDOR-SPECIFIC-MODULE-DEF-REF
    VendorSpecificModuleDefRef                                             = 4125,
    /// VENDOR-SPECIFIC-MODULE-DEF-REFS
    VendorSpecificModuleDefRefs                                            = 2910,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 2081,
    /// VERBATIM
    Verbatim                                                               = 2737,
    /// VERIFICATION-HASH
    VerificationHash                                                       = 3936,
    /// VERIFICATION-METHOD
    VerificationMethod                                                     = 2635,
    /// VERIFICATION-STATUS-INDICATION-MODE
    VerificationStatusIndicationMode                                       = 648,
    /// VERIFY-IREF
    VerifyIref                                                             = 1435,
    /// VERIFY-IREFS
    VerifyIrefs                                                            = 3697,
    /// VERIFY-REF
    VerifyRef                                                              = 3299,
    /// VERIFY-REFS
    VerifyRefs                                                             = 938,
    /// VERIFY-UPDATE
    VerifyUpdate                                                           = 3144,
    /// VERSION
    Version                                                                = 4632,
    /// VERSION-DRIVEN-FIND-BEHAVIOR
    VersionDrivenFindBehavior                                              = 1971,
    /// VF
    Vf                                                                     = 1386,
    /// VFB-TIMING
    VfbTiming                                                              = 5054,
    /// VFC-IREF
    VfcIref                                                                = 3749,
    /// VFC-IREFS
    VfcIrefs                                                               = 1818,
    /// VG
    Vg                                                                     = 3564,
    /// VIEW-MAP
    ViewMap                                                                = 6211,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 4305,
    /// VIEW-MAPS
    ViewMaps                                                               = 5279,
    /// VIN-INVALIDITY-PATTERN
    VinInvalidityPattern                                                   = 2935,
    /// VIOLATED-SAFETY-CONDITION-BEHAVIOR
    ViolatedSafetyConditionBehavior                                        = 4401,
    /// VLAN
    Vlan                                                                   = 4860,
    /// VLAN-ID
    VlanId                                                                 = 2042,
    /// VLAN-IDENTIFIER
    VlanIdentifier                                                         = 226,
    /// VLAN-MEMBERSHIP
    VlanMembership                                                         = 1321,
    /// VLAN-MEMBERSHIPS
    VlanMemberships                                                        = 3281,
    /// VLAN-MODIFIER-REF
    VlanModifierRef                                                        = 1191,
    /// VLAN-PRIORITY
    VlanPriority                                                           = 2960,
    /// VLAN-REF
    VlanRef                                                                = 4722,
    /// VLAN-REFS
    VlanRefs                                                               = 2691,
    /// VT
    Vt                                                                     = 3625,
    /// VTF
    Vtf                                                                    = 3622,
    /// WAIT-POINT
    WaitPoint                                                              = 5017,
    /// WAIT-POINTS
    WaitPoints                                                             = 1687,
    /// WAKE-UP-BY-CONTROLLER-SUPPORTED
    WakeUpByControllerSupported                                            = 6006,
    /// WAKE-UP-CHANNEL
    WakeUpChannel                                                          = 3191,
    /// WAKE-UP-OVER-BUS-SUPPORTED
    WakeUpOverBusSupported                                                 = 4720,
    /// WAKE-UP-PATTERN
    WakeUpPattern                                                          = 0,
    /// WAKEUP-FORWARD-LOCAL-ENABLED
    WakeupForwardLocalEnabled                                              = 1201,
    /// WAKEUP-FORWARD-REMOTE-ENABLED
    WakeupForwardRemoteEnabled                                             = 1571,
    /// WAKEUP-FRAME-REF
    WakeupFrameRef                                                         = 1545,
    /// WAKEUP-FRAME-REFS
    WakeupFrameRefs                                                        = 2538,
    /// WAKEUP-LOCAL-DETECTION-TIME
    WakeupLocalDetectionTime                                               = 3447,
    /// WAKEUP-LOCAL-DURATION-TIME
    WakeupLocalDurationTime                                                = 646,
    /// WAKEUP-LOCAL-ENABLED
    WakeupLocalEnabled                                                     = 6202,
    /// WAKEUP-REMOTE-ENABLED
    WakeupRemoteEnabled                                                    = 1160,
    /// WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST
    WakeupRepetitionDelayOfWakeupRequest                                   = 2039,
    /// WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST
    WakeupRepetitionsOfWakeupRequest                                       = 5062,
    /// WAKEUP-RX-IDLE
    WakeupRxIdle                                                           = 2514,
    /// WAKEUP-RX-LOW
    WakeupRxLow                                                            = 2748,
    /// WAKEUP-RX-WINDOW
    WakeupRxWindow                                                         = 6089,
    /// WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF
    WakeupSleepOnDatalineConfigRef                                         = 928,
    /// WAKEUP-SYMBOL-RX-WINDOW
    WakeupSymbolRxWindow                                                   = 1911,
    /// WAKEUP-TX-ACTIVE
    WakeupTxActive                                                         = 5575,
    /// WAKEUP-TX-IDLE
    WakeupTxIdle                                                           = 4541,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 2466,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 4219,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 2360,
    /// WINDOW-SIZE
    WindowSize                                                             = 1631,
    /// WINDOW-SIZE-INIT
    WindowSizeInit                                                         = 676,
    /// WINDOW-SIZE-INVALID
    WindowSizeInvalid                                                      = 390,
    /// WINDOW-SIZE-VALID
    WindowSizeValid                                                        = 429,
    /// WITH-AUTO
    WithAuto                                                               = 6000,
    /// WORST-CASE-EXECUTION-TIME
    WorstCaseExecutionTime                                                 = 1650,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 693,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 2837,
    /// WRITE
    Write                                                                  = 1347,
    /// WRITE-ACCESS
    WriteAccess                                                            = 3344,
    /// WRITE-CLASS-REF
    WriteClassRef                                                          = 5202,
    /// WRITE-ONLY-ONCE
    WriteOnlyOnce                                                          = 2057,
    /// WRITE-VERIFICATION
    WriteVerification                                                      = 5671,
    /// WRITER-ENTITY-ID
    WriterEntityId                                                         = 3576,
    /// WRITING-FREQUENCY
    WritingFrequency                                                       = 1940,
    /// WRITING-PRIORITY
    WritingPriority                                                        = 681,
    /// WRITING-STRATEGY-ROLE
    WritingStrategyRole                                                    = 5710,
    /// WRITING-STRATEGYS
    WritingStrategys                                                       = 124,
    /// WRITTEN-LOCAL-VARIABLES
    WrittenLocalVariables                                                  = 5217,
    /// WRITTEN-NV-DATA
    WrittenNvData                                                          = 5805,
    /// WRITTEN-READ-NV-DATA
    WrittenReadNvData                                                      = 2096,
    /// WWH-OBD-DTC-CLASS
    WwhObdDtcClass                                                         = 2628,
    /// X
    X                                                                      = 862,
    /// XCP-PDU
    XcpPdu                                                                 = 1949,
    /// XDOC
    Xdoc                                                                   = 3738,
    /// XFILE
    Xfile                                                                  = 657,
    /// XREF
    Xref                                                                   = 5650,
    /// XREF-TARGET
    XrefTarget                                                             = 5922,
}

impl ElementName {
    const STRING_TABLE: [&'static str; 6284] = ["WAKE-UP-PATTERN", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "COLD-START-ATTEMPTS", "PID-SIZE", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "CRYPTO-PRIMITIVE", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "AH-CIPHER-SUITE-NAMES", "DIRECT-PDU-REF", "INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "PROCESS-EXECUTION-ERROR", "EVENT-GROUP-IDENTIFIER", "ROM-BLOCK", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "SWC-IMPLEMENTATION", "MINISLOT-DURATION", "IEEE-1722-TP-ACF-LIN-PART", "BASE-FREQUENCY", "TARGET-FRAME-REF", "UNIT-REF", "SHORT-LABEL-PATTERN", "CONNECTION-BUNDLES", "GOVERNANCE-REF", "SIZE-OF-ARRAY-LENGTH-FIELD", "FIRST-DATA-PROTOTYPE-REF", "J-1939-TP-CONNECTION", "PER-STATE-TIMEOUT", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR", "NV-DATA-INTERFACE", "FIRE-AND-FORGET-MAPPING", "MESSAGE-TYPE-INFO", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "PDELAY-REQUEST-PERIOD", "PROTOCOL-NAME", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "REFILL-AMOUNT", "CHANNEL-SYNCHRONOUS-WAKEUP", "UDP-TTL", "EVALUATED-VARIANT-SET", "EXIT-TIMEOUT-VALUE", "RELIABILITY", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE", "AUTH-DATA-FRESHNESS-START-POSITION", "CLIENT-SERVER-INTERFACE", "SOURCE-EVENT-REFS", "ARBITRATION", "R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "DIAG-ROUTINE-TYPE", "UDP-COLLECTION-TRIGGER", "MAX-DEADLINE", "MAXIMUM-INTER-ARRIVAL-TIME", "SDG-CLASS-REF", "HW-CATEGORY-REF", "LOWER-REF", "CALL-TIMEOUT", "MAP", "SUPERVISION-ENTITYS", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "SETTER-REF", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "PERSISTENCY-FILE-PROXY-INTERFACE", "PHM-HEALTH-CHANNEL-INTERFACE", "RECOVERABLE-IN-SAME-OPERATION-CYCLE", "COMPONENT-IREFS", "LOGICAL-OPERATOR", "TRIGGER-INTERFACE-MAPPING", "ONE-EVERY-N", "CAN-CLUSTER-VARIANTS", "NUMBER-OF-INSTRUCTIONS", "REQUIRED-MODE-GROUP-REF", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "DEFAULT-LEASE-TIME", "RAW-DATA-STREAM-SERVER-INTERFACE", "TIMESTAMP-INTERVAL", "NETWORK-TIME-CONSUMER-REF", "SOFTWARE-CLUSTER", "TRIGGER-REF", "RESET-TO-DEFAULT", "TP-NODES", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE", "MAP-ELEMENT-TUPLES", "STATE-MANAGEMENT-REQUEST-TRIGGER", "INTERNAL-BEHAVIOR", "FIELD-DEPLOYMENTS", "AP-SOMEIP-TRANSFORMATION-PROPS", "SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING", "FREEZE-CURRENT-STATE", "CAN-FD-BAUDRATE", "ECUC-MULTILINE-STRING-PARAM-DEF", "READ-CLASS-REF", "CRYPTO-KEY-REF", "FAILED-REFERENCE-CYCLES-TOLERANCE", "AREA", "PERSISTENCY-DATA-REQUIRED-COM-SPEC", "TCP-IP-DHCP-V-6-CNF-DELAY-MIN", "SERVICE-BUSY", "ACTIVATION-ACTION", "COMM-CONTROLLERS", "SEGMENTATION-SUPPORTED", "BYTE-ORDER", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS", "SUB-SOFTWARE-CLUSTER-REF", "AUTOMATIC-END", "GROUP-IDENTIFIER", "RECOVERY-NOTIFICATION-REFS", "INPUT", "ANNOTATION-ORIGIN", "CAT-2-ISR", "ACCESSED-VARIABLE-REF", "RTPS-PROTECTION-KIND", "PDU-MAX-LENGTH", "IPV-6-DHCP-SERVER-CONFIGURATION", "IS-GLOBAL", "SW-MAINTENANCE-NOTESS", "SYNCHRONIZED-SLAVE-TIME-BASE", "TP-ADDRESS-REF", "DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL", "CAN-FD-CONFIG", "SCOPE-EVENT-REF", "PLCA-LOCAL-NODE-ID", "MATCHING-INGRESS-RULE-REFS", "BLOCK-STATES", "WRITING-STRATEGYS", "RESET-TO-DEFAULT-SUPPORTED", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL", "DNS-SERVER-ADDRESSES", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF", "TARGET-MODE-REF", "N-PDU-REFS", "SEND-ACTIVITY", "USE-INSTEAD-REF", "TERMINATION-BEHAVIOR", "PROVIDED-DATAS", "PDELAY-RESPONSE-ENABLED", "FACTOR-SI-TO-UNIT", "SAFETY-CONDITION", "PRE-MAPPING", "LDAP-URL", "DIAGNOSTIC-J-1939-SPN-MAPPING", "FREE-FORMAT", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "MODE-MAPPING", "DPG-REQUIRES-COHERENCYS", "RETURN-TYPE", "DIAGNOSTIC-TROUBLE-CODE-REF", "NUMBER-OF-MINISLOTS", "TARGET-I-PDU-REF", "GLOBAL-TIME-ETH-SLAVE", "REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF", "UNIT-GROUP-REF", "MINIMUM-TIMER-GRANULARITY", "SOMEIP-SERVICE-INTERFACE", "SEPARATION-TIME", "PERMISSION-TO-CREATE-CHILD-PROCESS", "BLACKLISTED-VERSIONS", "DATA-TYPE-MAPPING-SET-REFS", "SOMEIP-REMOTE-MULTICAST-CONFIG", "CONTEXT-R-PORT-REF", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF", "PROCESS-ARGUMENTS", "LIN-SCHEDULE-TABLE", "MESSAGE-TYPE", "LIFE-CYCLE-INFOS", "USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL", "ETHERNET-NETWORK-CONFIGURATION", "PARAMETER-ELEMENT-ACCESS", "AP-APPLICATION-ERROR-DOMAIN", "MIN-OK-STATE-INVALID", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "MODE-USER-ERROR-BEHAVIOR", "LOG-TRACE-FILE-PATH", "SDXF", "DDS-CP-SERVICE-INSTANCE-EVENT", "DATA-IDENTIFIER-REF", "PORT-NUMBER", "COMPOSITE-RULE-BASED-VALUE-SPECIFICATION", "TAILORING-REF", "CAN-CONFIG", "APPLICATION-RECORD-DATA-TYPE", "MODE-DECLARATION-GROUP-REFS", "PROCESS-MODE-IREF", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "SAMPLE-CLOCK-PERIOD", "LOCAL-ENDPOINT-CONFIG", "FIRST-MODE-REF", "UDP-PROPS", "COUPLING-PORT-DETAILS", "DURABILITY-KIND", "FLEXRAY-FIFO-CONFIGURATION", "REPLACE-WITH", "USER-DEFINED-CLUSTER-CONDITIONAL", "V-LAN-REFS", "DLT-ECU-REF", "BSW-IMPLEMENTATION", "DATA-PROTOTYPE-TRANSFORMATION-PROPSS", "TRACE-SWITCH-CONFIGURATIONS", "DISTINGUISHED-PARTITIONS", "MKA-PARTICIPANT-REF", "TRANSFORMATION-TECHNOLOGY", "STANDARD-REVISION", "SW-VARIABLE-PROTOTYPE-REF", "DLT-ARGUMENT-ENTRYS", "TRACE-MESSAGE-REF", "DDS-CP-QOS-PROFILE", "HW-ATTRIBUTE-DEF", "GLOBAL-TIME-PDU-REF", "MESSAGE-COMPLIANCE", "FIREWALL-STATE-IREF", "CLEAR-FROM-VALID-TO-INVALID", "SUB-ELEMENTS", "FOLLOW-UP-TIMEOUT-VALUE", "NETWORK-CONFIGURATION", "DLT-LOG-CHANNEL-DESIGN", "TARGET-I-PDU", "REFERENCED-TD-EVENT-SWC-REF", "UCM-MODULE-INSTANTIATION-REF", "TARGET-APPLICATION-RECORD-ELEMENT-REF", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "RATE-CORRECTION-MEASUREMENT-DURATION", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "SWC-MAPPING-CONSTRAINT-REF", "GID-INVALIDITY-PATTERN", "USER-DEFINED-PHYSICAL-CHANNEL", "OBD-MONITOR-SERVICE-NEEDS", "VLAN-IDENTIFIER", "CAN-IDENTIFIER-RANGE", "MATCHING-CRITERION-REF", "SSP-OFFSET", "J-1939-DCM-I-PDU", "SWC-MAPPING-CONSTRAINT-REFS", "UDP-NM-NODE", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "TP-ECU", "CIPHER-SUITE-ID", "TCP-IP-ARP-PACKET-QUEUE-ENABLED", "SUB-GROUP-REFS", "MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS", "APPROVAL-REQUIRED", "MAXIMUM-SEGMENT-LENGTH", "ATTRIBUTE-DEFS", "PER-INSTANCE-PARAMETER-REF", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING", "ECU-INSTANCE-REF-CONDITIONAL", "TTL-MAX", "INTERPOLATION-ROUTINE", "SW-CLASS-IMPLS", "EXCLUSIVE-AREA-NESTING-ORDER-REFS", "SECONDARY-FRESHNESS-VALUE-ID", "STREAM-IDENTIFICATION-WILDCARD", "TCP-IP-NDP-DELAY-FIRST-PROBE-TIME-VALUE", "DDS-PARTITIONS", "MAPPED-SECURITY-EVENTS", "CONSTANT-MAPPING-REF", "DIAGNOSTIC-IUMPR-GROUP", "PREDECESSOR-ACTION-REF", "IS-PIPELINING-PERMITTED", "EVENT-TOPIC-ACCESS-RULE-REF", "MODE-DECLARATION-GROUP-REF", "UDP-PORT-REF", "UNIT", "DDS-DOMAIN-RANGE", "VENDOR-API-INFIX", "DYNAMIC-PARTS", "HAS-DYNAMIC-LENGTH", "CONTEXT-P-PORT-REF", "LIN-MASTER-VARIANTS", "PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT", "MACHINE-MODE-MACHINES", "MESSAGE-LINE-NUMBER", "NOTIFIED-STATUSS", "SLEEP-MODE-SUPPORTED", "TARGET-ELEMENT-REF", "TRANSFORMATION-DESCRIPTIONS", "ROUGH-ESTIMATE-STACK-USAGE", "TCP-SLOW-START-ENABLED", "NESTED-ELEMENTS", "POSSIBLE-AP-ERROR-SET-REFS", "SOMEIP-TRANSFORMATION-PROPS-REF", "NOTIFICATION-PORT-IREF", "SDG-PRIMITIVE-ATTRIBUTE", "DETECT-NIT-ERROR", "PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS", "N-PDU", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "IEEE-1722-TP-RULE", "BSW-MODE-MANAGER-ERROR-EVENT", "SD-SERVER-CONFIG-REF", "DATA-RECEIVE-ERROR-EVENT", "STORE-IMMEDIATE", "CONCRETE-CLASS-TAILORING", "DIAGNOSTIC-EVENT-WINDOW", "CAN-CONTROLLER-FD-ATTRIBUTES", "NM-CONFIG", "CHECK-PER-SOURCE-ID", "TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED", "TARGET-EVENT-REFS", "REQUIRED-OUTER-PORT-REF", "UAS-ID", "MANAGED-COUPLING-PORTS", "BUS-DEPENDENT-NM-ECUS", "REQUIRED-MINOR-VERSION", "CLIENT-SERVER-INTERFACE-REF", "FIELD-ELEMENTS", "INTERVAL-LENGTH", "INTENDED-PARTIAL-INITIALIZATION-COUNT", "GLOBAL-TIME-DOMAIN-PROPS", "INVOCATION", "SDU-TYPE", "DURABILITY-SERVICE-CLEANUP-DELAY", "TX-NM-PDU-REFS", "CONTAINED-PACKAGE-ELEMENT-REF", "STATUS-BIT-STORAGE-TEST-FAILED", "SUPPORTING-MATERIAL", "TD-EVENT-TYPE", "PORT-BLUEPRINT-INIT-VALUE", "INTERPOLATION-ROUTINE-REF", "FILTER-ACTION-BLOCK-SOURCE", "PARAMETER-ACCESSED-FOR-DEBUG-REF", "TERMINATING-CHECKPOINT-TIMEOUT-UNTIL-TERMINATION", "ALLOWED-TCP-OPTIONS", "CRYPTO-SERVICE-CERTIFICATE-REF", "ETHERNET-FRAME-TRIGGERING", "FIBEX-ELEMENT-REF", "DOIP-CONNECTION-REFS", "DIAGNOSTIC-CONNECTED-INDICATOR", "UNICAST-NETWORK-ENDPOINT-REFS", "COMMUNICATION-MODE", "RECEIVE-WINDOW-MIN", "OWNERSHIP-KIND", "SOURCE-MAC-ADDRESS", "RX-PDU-POOL-REF", "JOB-PRIMITIVE-REF", "EXCLUSIVE-AREA-POLICYS", "DDS-CP-DOMAIN", "CONSTRAINT-TAILORINGS", "FUNCTION-NAMES", "SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS", "SOMEIP-FIELD", "FIFO-RANGES", "SOURCE-ADDRESS-MASK", "DATA-SEND-COMPLETED-EVENT", "ROLE-BASED-DATA-ASSIGNMENT", "USES-DYNAMIC-ADDRESSING", "FIELD-MAPPING", "DIAGNOSTIC-UDS-SEVERITY-ENUM-VALUE-VARIATION-POINT", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "TIME-INTERVAL", "ON-FAIL-PERMISSIVE-MODE-TIMEOUT", "AUTH-ROLE-REFS", "ENABLE-ANYCAST", "OBJECT-REF", "BUFFER-PROPERTIES", "RUNS-INSIDES", "SELECTOR-FIELD-CODE", "LOCAL-DEBUG-DATAS", "CONSTANT-VALUE-MAPPING-REF", "OPERATION-CYCLE-AUTOMATIC-END", "DYNAMIC-PART-ALTERNATIVES", "DIAGNOSTIC-COM-CONTROL-CLASS", "ECUC-NUMERICAL-PARAM-VALUE", "CONTEXT-ROOT-COMPOSITION-REF", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "CUSTOM-SUB-FUNCTION-NUMBER", "SLEEP-REPETITIONS-OF-SLEEP-REQUEST", "RPT-SYSTEM-REF", "ENTER-TIMEOUT-VALUE", "IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL", "PACKAGER-ID", "ACTION-LIST-EXECUTION", "CRYPTO-SERVICE-CERTIFICATE", "NM-PDU-REF-CONDITIONAL", "PER-INSTANCE-MEMORY-SIZE", "DATA-TYPE-MAPS", "PIN-NUMBER", "GROUP-ID", "ASSIGNED-ENTRY-REF", "CRYPTO-JOB", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "DEFAULT-VLAN-REF", "STATE", "STREAMS-PER-FRAME", "FEATURE-REFS", "NM-BUS-SYNCHRONIZATION-ENABLED", "CAN-ENTER-EXCLUSIVE-AREA-REFS", "SOURCE-FRAME-REF", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "SECURITY-EVENTS", "TFOOT", "WINDOW-SIZE-INVALID", "I-PDU-MAPPINGS", "END-TO-END-PROTECTIONS", "I-SIGNAL-TRIGGERINGS", "NM-VOTING-CYCLE", "CONFIGURATION-HANDLING", "SWC-MODE-GROUP-IREF", "TARGET-PORT-PROTOTYPE-REF", "PROCESS-TO-MACHINE-MAPPING", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "REPLICA-PDUS-REFS", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "CURRENT-DATA-TYPE-REF", "ROLE-BASED-DATA-TYPE-ASSIGNMENT", "FIRST-VALUE", "CONTEXT-LIMITATION-REFS", "IPV-4-PROPS", "AUTOSAR-DATA-TYPE-REF-CONDITIONAL", "KEY-SLOT-USED-FOR-SYNC", "LET-INTERVAL-REF", "KEY-REF", "SYNCHRONIZATION-POINT-CONSTRAINT", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "ECUC-QUERY", "STRUCTURED-REQ", "MEASURED-EXECUTION-TIME", "DDS-CP-PROVIDED-SERVICE-INSTANCE", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "DERIVED-FROM-BLUEPRINT-REF", "HANDLE-INVALID", "FLEXRAY-AR-TP-CHANNEL", "DATA-TYPE-FOR-SERIALIZATION-REFS", "AUTO-START", "DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING", "TRIGGER-TO-SIGNAL-MAPPING", "E-2-E-METHOD-PROTECTION-PROPSS", "MAPPING-SCOPE", "ASSIGNMENT-PRIORITY", "WINDOW-SIZE-VALID", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "DHCP-PROPS", "IDSM-PROPERTIES", "PROCESSOR-MODE", "OPERATION-ARGUMENT-INSTANCE-IREF", "RATE-BASED-MONITORED-EVENT-REF", "CAN-NM-ECU", "DIAGNOSTIC-STORAGE-CONDITION", "DIAGNOSTIC-CONDITION-REF", "SECURE-COM-PROPS-FOR-TCP-REFS", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "BOR-TIME-L-2", "FIELD-MAPPINGS", "SPECIFIC-CHANNEL-REF", "COMPU-NUMERATOR", "DO-IP-RULE", "DERIVED-FROM-BLUEPRINT-REFS", "STATIC-REMOTE-MULTICAST-ADDRESSES", "RPT-EXECUTABLE-ENTITY-EVENTS", "DIAGNOSTIC-SERVICE-TABLE-REF", "CHANNEL-MODE", "TCP-IP-DHCP-V-6-INF-DELAY-MIN", "COUPLING-PORT-SWITCHOFF-DELAY", "SUB", "MAX-NUMBER-OF-ALLOWED-UPDATES", "SECURITY-EVENT-MAPPING", "DDS-FIELD-DEPLOYMENT", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REF", "DIAGNOSTIC-TEST-RESULT-REFS", "RETURN-SIGNAL-REF", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS", "FIRE-AND-FORGET-MAPPINGS", "NM-MSG-REDUCED-TIME", "CYCLE-TIME-VALUE", "OUT-MEASUREMENT-SET", "LIN-TP-CONNECTION", "AUTHENTICATION-RETRIES", "TCP-IP-DHCP-V-6-SOL-DELAY-MIN", "REQUIRED-SERVICE-PORT-IREF", "BSW-MODE-SWITCHED-ACK-EVENT", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "DATA-TYPE-MAPPING-SET", "PORT-PROTOTYPE-PROPS", "BUS-ID", "ETH-TP-CONFIG", "MANAGED-PHYSICAL-CHANNEL-REFS", "RATE-CORRECTIONS-PER-MEASUREMENT-DURATION", "VARIANT-ID", "READ-NV-DATA", "TRACE-SWITCH", "ECUC-REF", "EXPLICIT-CONGESTION-NOTIFICATION", "EVENT-PROPSS", "TRIGGER-MAPPING", "RESPONSE-ERROR-POSITION", "APPLIED-STANDARD", "MAX-BUFFER-REQUEST", "PORT-SCHEDULER", "POST-BUILD-VARIANT-CRITERION", "SW-SYSTEMCONST", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "FIELD-TOPICS-ACCESS-RULE-REF", "UNASSIGNED-FRAME-TRIGGERING-REF", "TIMING-CONDITION-REF", "NM-COORD-CLUSTER", "CLIENT-SERVER-INTERFACE-MAPPING", "SOURCE-ELEMENT-REF", "CLIENT-SERVER-ARRAY-ELEMENT-MAPPING", "MAC-MULTICAST-ADDRESS", "TOLERATED-FAILED-CYCLES", "NM-REMOTE-SLEEP-INDICATION-TIME", "ASYNCHRONOUS-SERVER-CALL-POINT-REF", "PRIVACY-LEVEL", "NM-CAR-WAKE-UP-FILTER-NODE-ID", "DIAGNOSTIC-J-1939-SPN", "CLIENT-SERVER-OPERATION", "CONTAINED-PDU-TRIGGERING-REF", "BSW-OPERATION-INVOKED-EVENT", "CAN-TP-CHANNEL-REF", "IDS-PLATFORM-INSTANTIATION-REF", "REQUESTOR-REF", "SECURED-RX-VERIFICATION", "NM-PDU-REF", "EXCLUSIVE-AREA", "ALGORITHM-MODE", "MODE-GROUP-REF", "LENGTH-CLIENT-ID", "SUB-DOMAIN-REF", "RELEASED-TRIGGERS", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "PNC-MAPPING", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "CONTEXT-APPLICATION-RECORD-ELEMENT-REF", "MEMORY-LOW-ADDRESS-LABEL", "NETWORK-CONFIGURATIONS", "SWC-BSW-SYNCHRONIZED-TRIGGER", "CLASS-CONTENT-CONDITIONAL", "SECOND-ELEMENT-INSTANCE-IREFS", "IMPLEMENTATION-CONFIG-VARIANT", "FULLY-QUALIFIED-DOMAIN-NAME", "DOCUMENT-ELEMENT-SCOPES", "EVENT-ID", "HAS-INTERNAL-STATE", "DEPENDS-ON", "REQUIREMENT-REF", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "UDP-COLLECTION-BUFFER-TIMEOUT", "AUTHENTICATION-VERIFY-ATTEMPTS", "OWNERSHIP-STRENGTH", "ENGINEERING-OBJECTS", "TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS", "USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS", "DATA-ELEMENT-REF", "MICRO-INITIAL-OFFSET-B", "SOCKET-ADDRESS-REF", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "METHOD-ACTIVATION-ROUTING-GROUPS", "SYSTEM-SIGNAL-REFS", "CONTEXT-SW-COMPONENT-REF", "PHYSICAL-LAYER-TYPE", "SUBSCRIBE-EVENTGROUP-RETRY-DELAY", "NV-REQUIRE-COM-SPEC", "SW-ADDR-METHOD", "INTEGER-VALUE-VARIATION-POINT", "LOGIC-ADDRESSS", "VALUE-REF", "CS-ERROR-REACTION", "SUPERVISED-ENTITY-ID", "SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS", "DATA-DUMP-ENTRY", "ECU-RESOURCE-ESTIMATION", "REQUIRED-MODE-GROUPS", "SWC-BSW-MAPPING-REF", "COUPLING-PORT-SCHEDULER", "ENVIRONMENT-CAPTURE-TO-REPORTING", "COLLECTION-TIMEOUT", "J-1939-NM-NODE", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS", "REQUIRED-PORT-PROTOTYPE-IREF", "ANALYZED-EXECUTION-TIME", "MINIMUM-FIFO-LENGTH", "FLEXRAY-TP-CONNECTION-CONTROL", "VALID-BINDING-TIMES", "REST-ENUMERATOR-DEF", "PROVIDED-DDS-SERVICE-INSTANCE-EVENTS", "CALIBRATION-PARAMETER-VALUE-SET", "SW-MIN-AXIS-POINTS", "FLEXRAY-TP-ECU", "COUNTER-FAILED-THRESHOLD", "NM-MSG-CYCLE-OFFSET", "COM-TRIGGER-GRANT-DESIGN", "BLUEPRINT-REF", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "INTERNAL-TRIGGER-OCCURRED-EVENT", "MAX-AS", "SENSOR-INSTANCE-ID", "TARGET-REQUIRED-OPERATION-REF", "SYNCHRONIZED-TRIGGERS", "ARBITRARY-EVENT-TRIGGERING", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "COMPOSITE-INTERFACE", "TP-ADDRESS-REFS", "COMMUNICATION-CYCLE", "J-1939-TP-NODE", "MODE-PORT-ANNOTATION", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "SUPERVISION-CYCLE", "CONNECTION-IS-MANDATORY", "SECTION-NAME-PREFIXS", "PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS", "RPT-SERVICE-POINT", "RECEPTION-POLICYS", "MAX", "TYPE-EMITTER", "NM-NID-POSITION", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "ENDPOINTS", "AUTOSAR-CONNECTOR", "PROCESS-REF", "EVALUATED-ELEMENT-REF", "RPT-EVENT-ID", "UNASSIGN-FRAME-ID", "PAYLOAD-PREAMBLE-INDICATOR", "DELIVERY-ARTIFACTS", "DO-IP-CONFIG", "ELEMENT-MAPPINGS", "ENTRYS", "APPLICATION-ARRAY-DATA-TYPE", "TIMEOUT-BR", "APPLICATIONS", "TIME-BASE-RESOURCE-REFS", "PDU-TRIGGERING", "IDENTITY-CERTIFICATE-AUTHORITY-REF", "MC-DATA-INSTANCE-REFS", "MULTICAST-THRESHOLD", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "DDS-FIELD-REPLY-TOPIC-REF", "TP-PGS", "ROM-LOCATION-REF", "FIRST-MODE-GROUP-REF", "ACL-OPERATION", "CLIENT-ID", "LENGTH-SEQUENCE-COUNTER", "REQUESTER-REFS", "CAPABILITY-RECORDS", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "SW-DATA-DEPENDENCY", "LATENCY-BUDGET-DURATION", "IDS-MGR-NEEDS", "DDS-PARTITION-REF", "LABEL-1", "REFERENCE-TAILORING", "DLT-CONTEXT-REF-CONDITIONAL", "CLIENT-ID-DEFINITIONS", "LOCAL-SUPERVISION-REFS", "MODE-INTERFACE-MAPPING", "WAKEUP-LOCAL-DURATION-TIME", "PLCA-TRANSMIT-OPPORTUNITY-TIMER", "VERIFICATION-STATUS-INDICATION-MODE", "SELECT-BLOCK-FOR-FIRST-INIT-ALL", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "PREVIOUS-DATA-TYPE-REF", "DDS-OPERATION-RESPONSE-TRIGGERING-REF", "OVERRIDE-ID", "SERVICE-DISCOVERY-CONFIGS", "MANAGED-PHYSICAL-CHANNEL-REF", "BSW-EXCLUSIVE-AREA-POLICY", "XFILE", "TRANSMIT-CANCELLATION", "DIAGNOSTIC-REQUEST-UPLOAD", "GLOBAL-TIME-CAN-MASTER", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "MSR-QUERY-NAME", "CALL-SIGNAL-REF", "I-SIGNAL-I-PDU", "TRACE", "CAN-FRAME", "SEQUENCE-OFFSET", "TRANSLATION-TARGET-IREF", "CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING", "FM-FEATURE-DECOMPOSITION", "N-ROM-BLOCKS", "RESOURCE-NEEDS-REFS", "TIMING-VARIABLE-REF", "SLOT-CAPACITY", "SERVER-COM-SPEC", "WINDOW-SIZE-INIT", "DDS-EVENT-REF", "REST-ENDPOINT-POST", "DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE", "WRITING-PRIORITY", "TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED", "BOR-TIME-L-1", "DLT-LOG-CHANNELS", "SIZE-OF-ARRAY-LENGTH-FIELDS", "CRC-PRECISE-ORIGIN-TIMESTAMP", "START", "SECURITY-DELAY-TIME", "PNC-IDENTIFIER", "REMOTE-SUBJECT-REF", "INTERNET-HEADER-LENGTH", "DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF", "WORST-CASE-HEAP-USAGE", "IMPLEMENTED-IN-REF", "LOCAL-UNICAST-ADDRESSS", "DIAGNOSTIC-MEMORY-IDENTIFIER", "INVALIDATION-POLICY", "UCM-MASTER-MODULE-INSTANTIATION", "AR-TYPED-PER-INSTANCE-MEMORY-REF", "GLOBAL-ELEMENTS", "NM-BUSLOAD-REDUCTION-ACTIVE", "DESTINATION-NETWORK-MASK", "SW-CALPRM-AXIS", "SECOND-MODE-REF", "MODULE-INSTANTIATIONS", "CHECKPOINT-TRANSITION-REF", "BEHAVIOR-REF", "MODE-DECLARATION", "TP-CHANNEL-REF", "COUPLING-ELEMENT-DETAILS", "ASSIGN-FRAME-ID-RANGE", "POSSIBLE-ERROR-REF", "BLOCK-INCONSISTENT", "PROFILE-NAME", "DDS-CP-SERVICE-INSTANCE-OPERATION", "MAX-HEADER-ID", "BLOCK-IF-STATE-ACTIVE-CP-REF", "HEAP-USAGES", "DATA-PDU-REF", "TIME-HARDWARE-CORRECTION-THRESHOLD", "SHAPER", "BASE-TYPE-SIZE", "EXPIRED-SUPERVISION-CYCLES-TOLERANCE", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "NETWORK-MANAGEMENT-VECTOR-LENGTH", "TCP-OPTION-FILTER-LISTS", "RECOVERY-ACTION-IREF", "TEX-MATH", "ECU-RESET-CLASS-REF", "ROLLOUT-QUALIFICATIONS", "NM-VECTOR-EARLY-UPDATE", "VENDOR", "VARIABLE-REF", "TP-ADDRESS", "FRESHNESS-PROPSS", "SERVICE-FIND-TIME-TO-LIVE", "DIAGNOSTIC-TROUBLE-CODE-OBD", "SYNCHRONIZED-MODE-GROUPS", "ABSOLUTE-TOLERANCE", "INCLUDED-DATA-TYPE-SET", "SYSTEM-MEMORY-USAGE", "SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE", "TCP-IP-NDP-PACKET-QUEUE-ENABLED", "SWC-TO-SWC-OPERATION-ARGUMENTS", "COMPOSITE-NETWORK-REPRESENTATION", "DO-IP-ACTIVATION-LINE-NEEDS", "ENTRY-TYPE", "STATE-DEP-FIREWALL-REF", "POSSIBLE-ERROR-REACTIONS", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REF", "PORT-BLUEPRINT", "TABLE", "KEY-EXCHANGE-REF", "FIREWALL-STATE-MODE-DECLARATION-REF", "AGING-ALLOWED", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "COMMUNICATION-CONNECTORS", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS", "REFERENCE-TABLE-REF", "TRACE-REF", "DEADLINE-SUPERVISIONS", "RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "META-DATA-LENGTH", "CONTEXT-ELEMENT-REF", "FRAGMENTATION-PROPS", "ECU-INSTANCE-REF", "UNICAST-SECURE-COM-PROPS-REFS", "ACL-ROLE", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "SUB-NODE-NUMBER", "RELATIVE-REPRESENTATION-TIME", "LIN-UNCONDITIONAL-FRAME", "SYNC-NODE-MAX", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "DIAGNOSTIC-SERVICE-INSTANCE-REF", "MEMORY-USAGE", "SECURE-PROPS-FOR-UDP-REF", "LIN-NM-CLUSTER", "PNC-WAKEUP-CAN-ID-MASK", "STATIC-PART", "AUTHENTICATION-REF", "RTE-EVENT-IREFS", "LOCAL-VARIABLE-REF", "CAN-XL-CONFIG-REQS", "RTE-EVENT-IREF", "SW-DIAGNOSTICS-NOTESS", "MAXIMUM-RESIDENCE-TIME", "IPV-6-CONFIGURATION", "ECUC-FLOAT-PARAM-DEF", "CONTEXT-ROOT-SW-COMPOSITION-REF", "ACTION-LIST-ITEM-REFS", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "MEMORY-RANGE-REF", "VARIABLES", "NM-CONTROL-BIT-VECTOR-ACTIVE", "DEPENDS-ONS", "EVENT-WINDOW-TIME", "ECU-REF", "MAX-TESTER-CONNECTIONS", "PRIMITIVE", "SYSTEM-REF", "DLT-LOG-CHANNEL-DESIGN-REF", "DESTINATION-IPV-4-ADDRESS", "SIZE-OF-UNION-LENGTH-FIELD", "TEMPLATE-TYPE-REF", "HEALTH-CHANNEL-EXTERNAL-STATUS", "FILE-INFO-COMMENT", "NM-GLOBAL-COORDINATOR-TIME", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "PERSISTENCY-FILE-STORAGE", "CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE", "NM-SYNCHRONIZING-NETWORK", "COM-TRIGGER-GRANT", "ACCEPTS-ENCODINGS", "CLIENT-POLICYS", "DIRECTION", "TT", "CLIENT-ID-DEFINITION-SET", "INNER-PORT-IREF", "BUFFER-OUTPUT", "BSW-MODULE-DOCUMENTATIONS", "PER-INSTANCE-MEMORY-SIZES", "COM-MANAGEMENT-MAPPINGS", "PATTERN-JITTER", "CLASS-TAILORINGS", "I-SIGNAL-IN-I-PDU-REF", "USER-DEFINED-FIELD-DEPLOYMENT", "HARDWARE-ELEMENT-REF", "SOFTWARE-MEMORY-SECTION-REF", "KEY-VALUE-STORAGE-REF", "TCP-OPTION-FILTER-LIST", "MAX-NUMBER-OF-RESP-PENDING-FRAMES", "INITIAL-OFFER-BEHAVIOR", "TIME-SYNC-TECHNOLOGY", "TCP-CONNECT-TIMEOUT", "DDS-CP-CONSUMED-SERVICE-INSTANCE", "APPLIES-TO-DEPENDENCIES", "SOURCE-ADDRESS-CODE", "RPT-EXECUTION-CONTEXT", "TRIGGERING-EVENT-REF", "APPLICATION-TYPE-TREF", "MAPPED-AP-DATA-ELEMENT-IREF", "ATTRIBUTE-REF", "CERT-COMMON-NAME", "EVENT-QOS-PROPSS", "VARIABLE-ACCESSED-FOR-DEBUG-REF", "ATTRIBUTE-TAILORINGS", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "ATTRIBUTES", "PRIMITIVE-ATTRIBUTE-CONDITION", "LENGTH", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "MODE-SWITCH-POINT", "OPERATION-IREFS", "DERIVED-CERTIFICATE-ACCEPTED", "PROVIDED-SERVICE-PORT-IREF", "PERIODIC-EVENT-TRIGGERING", "ROLE-BASED-MC-DATA-ASSIGNMENT", "DHCP-SERVER-CONFIGURATION", "KEY-VALUE-PAIRS", "X", "I-PDU-TRIGGERING-REF", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS", "FRAME-MAPPING", "COUPLING-PORT-RATE-POLICY", "QUEUED-SENDER-COM-SPEC", "DIAGNOSTIC-IO-CONTROL-CLASS", "FILE-TYPE", "DIAGNOSTIC-COM-CONTROL", "SWC-EXCLUSIVE-AREA-POLICY", "TCP-IP-ICMP-PROPS-REF", "AP-APPLICATION-ENDPOINTS", "MODE-DEPENDENT-STARTUP-CONFIG", "BSW-MGR-NEEDS", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "CRYPTO-KEY-SLOT-REF", "APP-TASK-PROXY-REF", "PLCA-PROPS", "FUNCTION-INSTANCE", "HW-PIN-REFS", "DIAG-REQUIREMENT", "CONFIDENTIALITY-OFFSET", "METADATA-PROTECTION-KIND", "USED-CODE-GENERATOR", "FURTHER-ACTION-BYTE-NEEDS", "SDG-CAPTION", "DO-IP-TESTER-ROUTING-ACTIVATION-REFS", "SYNCHRONIZATION-TIMING-CONSTRAINT", "ROOT-COMPOSITION", "PDU-COLLECTION-PDU-TIMEOUT", "ETH-IP-PROPS-REFS", "DESIGN-REFS", "RPT-COMPONENTS", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "INDENT-SAMPLE", "TCP-SYN-MAX-RTX", "ANNOTATION", "ADD-INFO-5", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "DURABILITY-SERVICE-MAX-SAMPLES", "BR", "CAN-NM-CLUSTER-COUPLING", "MOLAR-AMOUNT-EXP", "CRYPTO-SIGNATURE-SCHEME", "BSW-DEBUG-INFO", "DURABILITY-SERVICE-MAX-INSTANCES", "MAC-SEC-PARTICIPANT-SET", "QUEUE-LENGTH", "BUILD-ACTION-ENVIRONMENT", "GATEWAYS", "CAN-CONTROLLER-CONFIGURATION", "ENTRY-MODE-DECLARATION-REF", "MODE-SWITCH-POINTS", "API-PRINCIPLE", "GETS", "PER-INSTANCE-PARAMETERS", "SOFTWARE-ACTIVATION-DEPENDENCY-REF", "REVISION-LABEL-P-1", "DIRECT-SUCCESSOR-REF", "PLATFORM-ACTION-ITEM", "DO-IP-INTERFACE", "LOG-SINK-REFS", "SECOND-MODE-GROUP-REF", "DETERMINISTIC-CLIENT-RESOURCE-NEEDSS", "DIAGNOSTIC-SESSION-REFS", "TARGET-DATA-PROTOTYPE-IN-SR-REF", "WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF", "DIAGNOSTIC-SOVD-METHOD", "USE-VEHICLE-IDENTIFICATION-SYNC-STATUS", "ALIAS-EVENT-REF", "FILTERING-DEBOUNCING", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "COMPOSITE-SERVICE-INTERFACE-REF", "OBD-PID-SERVICE-NEEDS", "RECORD-ELEMENTS", "SOMEIP-REMOTE-UNICAST-CONFIG", "VERIFY-REFS", "MODE-MANAGER-ERROR-BEHAVIOR", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE", "TWO-KEY-SLOT-MODE", "RECOVERY", "P-2-MAX", "AGE-CONSTRAINT", "CHANNEL-ID", "I-PDU-IDENTIFIER-UDP-REFS", "ARTIFACT-CHECKSUM", "CRYPTO-OBJECT-TYPE", "DERIVED-PORT-INTERFACE-REF", "CONSTANT-MAPPING-REFS", "SECURITY-EVENT-PROPS-REF", "HW-PIN", "ANONYMOUS", "INITIAL-INACTIVITY-TIME", "MAX-AVAILABLE-PERSISTENCY-STORAGE-SPACE", "TP-TECHNOLOGY", "TLV-DATA-ID-0-REF", "METHOD-REF", "REQUIRED-TRIGGERS", "INVERT", "USES-MONITOR-DATA", "NODE-IDENTIFIER", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS", "DIAGNOSTIC-CONNECTIONS", "MAC-SEC-KAY-PARTICIPANT", "SW-CALPRM-PROTOTYPE-REF", "STREAM-ID", "TP-ADDRESS-EXTENSION-VALUE", "IS-OBD-RELEVANT", "AUTO-REQUIRE", "SOURCE-CAN-ID-MASK", "CONNECT", "BSW-ENTITY-REF", "EXTERNAL-SYNC", "FUNCTION-NAME", "TARGET-COMPONENT-REF", "CLIENT-SERVER-ANNOTATIONS", "MACHINE-MODE-IREFS", "PNC-FILTER-DATA-MASK", "CHECKPOINT-REFS", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "REVISION-LABEL-P-2", "DTC-SETTING-PARAMETER", "SECOND-DATA-PROTOTYPE-REF", "SECURE-COMMUNICATION-DEPLOYMENTS", "IO-CONTROL-CLASS-REF", "IEEE-1722-TP-CONFIG", "STD", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS", "STACK-USAGES", "INSTANTIATION-TIMING-EVENT-PROPS", "ROOT-SW-COMPONENT-PROTOTYPE", "REST-ENDPOINT-DELETE", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "INHIBITION-MASK", "REVISION-LABEL", "REACTION-CODE", "TIMING-EVENT", "SO-CON-I-PDU-IDENTIFIER-REF", "TD-EVENT-COMPLEX", "TARGET-DATA-PROTOTYPE-REF", "JUMP-TO-BOOT-LOADER", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "SECURITY-ACCESS-LEVEL", "MEMORY-HIGH-ADDRESS", "ALL-CHANNELS-REF", "DDS-CP-TOPIC", "DATA-PROTOTYPE-GROUP-IREFS", "ERROR-MAPPINGS", "DEVELOPMENT-VALUE", "PNC-NM-REQUEST", "ACK-TYPE", "EXECUTE-DESPITE-DATA-UNAVAILABILITY", "UPDATE-STRATEGY", "AGING-CYCLES", "SW-DIAGNOSTICS-NOTES", "TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "ADDRESS-CONFIGURATION-CAPABILITY", "MAX-MARGIN", "CHANNEL-REF", "USER-DEFINED-ETHERNET-FRAME", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "INTERPOLATION-ROUTINE-MAPPING-SET-REFS", "SWC-MODE-MANAGER-ERROR-EVENT", "POST-VERIFICATION-REBOOT", "CRYPTO-SERVICE-KEY-REF", "TX-PDU-TRIGGERING-REF", "CRF-PULL", "SIGNAL-BASED-METHOD-DEPLOYMENT", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS", "SESSION-ID", "ALL-NM-MESSAGES-KEEP-AWAKE", "PROCESSOR-SPEED", "FLOW-CONTROL-REF", "ARBITRARY-ADDRESS-CAPABLE", "IS-OPTIONAL", "TCP-IP-DHCP-V-6-INF-DELAY-MAX", "COMMON-SIGNAL-PATH", "MC-SW-EMULATION-METHOD-SUPPORT", "NATIVE-DECLARATION", "CAN-TP-NODE", "ARTIFACT-CHECKSUMS", "CRYPTO-SERVICE-PRIMITIVE", "TRANSFORMATION-PROPSS", "KEY-SLOT-USAGE", "FLAT-INSTANCE-DESCRIPTOR", "SECURE-TRANSLATION", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "APPLICATION-PARTITION-REFS", "PARAMETER-POLICYS", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL", "UCM-MASTER-FALLBACK-REFS", "SPN-REFS", "OPERATION-CYCLE-REF", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "DESTINATION-ORDER", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT", "PDU-POOLS", "PHYSICAL-DIMENSION-REF", "I-SIGNAL-MAPPING", "CRC-VALIDATED", "CALL-TYPE", "REFERENCE-REF", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "DIAGNOSTIC-SOVD-UPDATE", "RECORD-ELEMENT-MAPPINGS", "DESTINATION-URI-DEFS", "PRIORITY-ID", "NETWORK-INTERFACES", "FLEXRAY-NM-NODE", "READ", "PARTITIONS", "MAC-SEC-KAY-CONFIG", "DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER", "EXECUTABLE-TIMING", "INCLUDED-MODE-DECLARATION-GROUP-SET", "ECUC-COND", "DATA-PROTOTYPE-GROUP-IREF", "CAN-TP-CHANNEL", "PERSISTENCY-CENTRAL-STORAGE-URI", "BSW-SERVICE-DEPENDENCY-REF", "DATA-CONSTR", "PAYLOAD-REF", "ECUC-TEXTUAL-PARAM-VALUE", "APPLICATION-COMPOSITE-ELEMENT-IREF", "DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING", "MANUFACTURER-CODE", "DO-IP-INSTANTIATION", "HEALTH-CHANNEL-EXTERNAL-MODE", "SAFETY-CONDITIONS", "EVENT-GROUP-ID", "VALUE-PAIRS", "TRANSFORMATION-COM-SPEC-PROPSS", "EXPLICIT-RPT-PROFILE-SELECTION-REF", "IKE-REAUTH-TIME", "ELEMENT-POSITION", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "TCP-IP-IP-NUM-FRAGMENTS", "SW-GENERIC-AXIS-PARAM-TYPES", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "CLAIMED-FUNCTION-GROUP-REF", "K-RUN-RESOURCE", "PURPOSE-OF-UPDATE-REF", "MASTER", "SERVICE-INTERFACE-ELEMENT-MAPPING-REF", "CONTROL-PROVIDED-EVENT-GROUP-REF", "SOURCE-IPV-4-ADDRESS", "NETWORK-INTERFACE-REF", "ETHERNET-CLUSTER-CONDITIONAL", "BSW-MODE-RECEIVER-POLICY", "INDICATOR-REF", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "BULK-NV-DATA-DESCRIPTORS", "USED-DATA-ELEMENT", "PROVIDED-DDS-OPERATIONS", "ECUC-MODULE-CONFIGURATION-VALUES", "SHUTDOWN", "THEAD", "MIN-DATA-LENGTH", "PDU-COUNTER-START-POSITION", "ENTRY-REF", "DEST-META-CLASS", "SW-FEATURE-DESCS", "TD-EVENT-BSW-MODE-DECLARATION-TYPE", "MAC-SEC-GLOBAL-KAY-PROPS", "ETHERNET-FRAME", "DOMAIN-ID", "SERVICE-ID", "MINOR-PROTOCOL-VERSION", "R-PORT-PROTOTYPE-PROPS", "NETWORK-ENDPOINTS", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "AUTHENTICATION-PROPSS", "ARRAY-SIZE", "ECU-CONFIGURATION-REF", "DEBOUNCE-TIME", "DIAGNOSTIC-FIM-EVENT-GROUP", "SUPERVISION-REF", "E-2-E-PROFILE-CONFIGURATION-REF", "ICMP-V-4-PROPS", "ROOT-REF", "PNC-MAPPINGS", "CONSTANT-MEMORYS", "TIME-SYNC-MODULE-INSTANTIATION", "IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "MAC-MULTICAST-GROUP-REF", "MAX-PERIODIC-DID-TO-READ", "BINARY-MANIFEST-PROVIDE-RESOURCE", "SYNC-TIME-BASE-MGR-USER-NEEDS", "CUSTOM-SPECIFICATION-REFS", "RUNTIME-PORT-CONFIGURATION", "PARAMETER-PORT-ANNOTATION", "ECUC-LINKER-SYMBOL-DEF-VARIANTS", "EXCLUSIVE-AREAS", "MODE-DRIVEN-TRANSMISSION-MODE-CONDITION", "WAKEUP-REMOTE-ENABLED", "TESTED-ITEM-REFS", "EVENT-HANDLER-REF", "SDG-TAILORINGS", "PERSISTENCY-DEPLOYMENT-REF", "BUS-SPECIFIC-NM-ECU", "SOCKET-CONNECTION-BUNDLE", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF", "V-2-X-DATA-MANAGER-NEEDS", "CLUSTER-DRIFT-DAMPING", "AP-APPLICATION-ERROR", "SEGMENT-LENGTH", "ASSOCIATED-EVENT-IDENTIFICATION", "ENCODING", "TARGET-METHOD-REF", "SIGNAL-FAN", "INGRESS-PRIORITY", "KEY-STORAGE-TYPE", "LINKERS", "PARTIAL-NETWORK-REF", "CONTAINED-AR-ELEMENT-REF", "OPERATION-REF", "PDU-ACTIVATION-ROUTING-GROUPS", "SDG", "PERSISTENCY-FILE-PROXY", "SIGNAL-GROUP-REF", "MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID", "REPRESENTS-VIN", "TOPIC-NAME", "MAX-NUM-COMPARISION-OF-VALUE-EVENTS", "VLAN-MODIFIER-REF", "TARGET-EVENT-REF", "DATA-EXCHANGE-POINT", "COLSPEC", "COUPLING-PORT-CONNECTIONS", "P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "PROCESS-PHM-ACTION-ITEM", "LIN-SLAVES", "TARGET-DATA-PROTOYPE-REF", "PWM-L", "WAKEUP-FORWARD-LOCAL-ENABLED", "VARIABLE-LENGTH", "BUILD-ACTIONS", "COMMAND", "ELEMENTS", "BLUEPRINT-MAPS", "FIRST-ELEMENT-INSTANCE-IREF", "STANDARD", "PROVIDED-DDS-SERVICE-INSTANCE", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "IMPOSITION-TIME", "SPECIFIC-PHYSICAL-CHANNEL-REF", "CAN-ID-RANGE-MAPPINGS", "DATA-TYPE-REF", "TEXT-TABLE-MAPPINGS", "SW-CALIBRATION-NOTES", "TOL", "MODE-DEPENDENT-STARTUP-CONFIGS", "SERVICE-DEPLOYMENT-REF", "NM-IF-ECU-REF", "CAN-TP-ECU", "ORDERED-ELEMENTS", "THRESHOLD-SIZE", "PROVIDED-PORT-PROTOTYPE-IREF", "MODE-SWITCH-INTERFACE", "BSW-QUEUED-DATA-RECEPTION-POLICY", "APPL-WATCHDOG-LIMIT", "DATA-ID", "FREEZE-FRAME-CONTENT-WWH-OBD-REF", "BSW-SCHEDULABLE-ENTITY", "REQUIRES-INDEX", "MAX-EXP-BS", "PARAMETER", "TARGET-SWC-SERVICE-DEPENDENCY-REF", "URI", "CAN-GLOBAL-TIME-DOMAIN-PROPS", "TCP-IP-NDP-DEFENSIVE-PROCESSING", "ACCESS-COUNT", "ALLOW-DYNAMIC-L-SDU-LENGTH", "NETWORK-INTERFACE-REFS", "HOP-COUNT", "SDU-REFS", "POSITION", "FUP-DATA-ID-LIST", "ELEMENT-BYTE-VALUE", "MULTICAST-CONNECTOR-REF", "ENTITY-STATUS-MAX-BYTE-FIELD-USE", "NETWORK-HANDLE-REF", "DO-IP-LOGIC-ADDRESSS", "MC-SUPPORT", "PADDING-ACTIVATION", "GETTER-RETURN-SIGNAL-REF", "OS-TASK-EXECUTION-EVENT", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS", "SDG-DEF", "RVF-INTERLACED", "DO-IP-TESTER-ROUTING-ACTIVATION-REF", "DATA-MAPPINGS", "SM-INTERACTS-WITH-NM-MAPPING", "DTC-NUMBER", "RUNNABLE-REF", "PARAMETER-DATA-PROTOTYPE-IREF", "SECURITY-DELAY-TIME-ON-BOOT", "IP-SEC-RULES", "I-PDU-PORT-REFS", "NDP-PROPS", "ECU-SCOPE-REF", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "NOMINAL", "BLOCKLISTED-VERSIONS", "TIME-FAILED-THRESHOLD", "FEATURE-REF", "ECU-INSTANCE-REFS", "ROUTING-ACTIVATION-TYPE", "PATH-MTU-DISCOVERY-ENABLED", "SECURE-COMMUNICATION-PROPS", "N", "STORE-EMERGENCY", "MODE-IREFS", "CAN-XL-NM-PROPS", "RECEIVER-REFS", "MAC-UNICAST-ADDRESS", "IMPORTANCE", "DDS-I-SIGNAL-TO-TOPIC-MAPPINGS", "LIN-FRAME-TRIGGERING", "TLV-DATA-ID-DEFINITION-REF", "EXCESS-BURST-SIZE", "POST-BUILD-VALUE-ACCESS-REF", "INITIAL-REPETITIONS-BASE-DELAY", "IP-SEC-IAM-REMOTE-SUBJECT", "BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE", "MACHINE", "DLT-APPLICATION-TO-PROCESS-MAPPING", "RAPID-PROTOTYPING-SCENARIO", "SW-MAINTENANCE-NOTES", "CHECKSUM-VALUE", "DATA-COM-PROPS", "DATA-LENGTH-RESPONSE", "CYCLE-COUNT-MAX", "DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING", "METHOD-REPLY-TOPIC-NAME", "NM-COORDINATOR-ROLE", "LINKER", "METHOD-RETURN-REF", "NM-STATE-CHANGE-IND-ENABLED", "TD-EVENT-TRIGGER-TYPE", "INTERPOLATION-ROUTINE-MAPPING", "SOURCE-IPV-6-ADDRESS", "CRYPTO-SERVICE-JOB-NEEDS", "EXECUTION-DEPENDENCY", "OBJECT-DEFINTION-REFS", "NM-COM-CONTROL-ENABLED", "COMPILER", "BSW-BACKGROUND-EVENT", "LOCAL-SUPERVISION-REF", "SCHEDULE-CHANGE-NEXT-TIME-BASE", "TDLET-ZONE-CLOCK", "BURST-SIZE-REQUEST", "SOCKET-CONNECTION-REFS", "VLAN-MEMBERSHIP", "DDS-EVENT-QOS-PROPS", "MINIMUM-SUSTAINED-SIZE", "IDSM-INSTANCE-REF", "IDENTITIY-NUMBER", "USE-CASE", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "NV-BLOCK-DESCRIPTORS", "TD-EVENT-FRAME", "MAXIMUM-DYNAMIC-PAYLOAD-LENGTH", "STORAGE-CONDITION-GROUP-REF", "TCP-IP-IP-REASSEMBLY-BUFFER-SIZE", "HEADER-ID-SHORT-HEADER", "HEALTH-CHANNEL-ARGUMENT-REF", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF", "CLIENT-SERVER-PORTS", "MINIMUM-MULTICAST-SEPERATION-TIME", "TRANSMISSION-DEADLINE", "NM-ACTIVE-COORDINATOR", "CAPABILITY", "SOFTWARE-CLUSTER-DESIGN", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "DURABILITY-SERVICE-HISTORY-DEPTH", "PDU-TRIGGERING-REF-CONDITIONAL", "CONTEXT-BSW-IMPLEMENTATION-REF", "WRITE", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "CONTEXT-SW-COMPONENT-PROTOTYPE-REF", "MANDATORY", "MINIMUM", "CONTAINER-I-PDU", "SD-SERVER-EG-TIMING-CONFIGS", "ACCESS-DATA-RECORD-SIZE", "TCP-GENERAL-INACTIVITY-TIME", "REMOTE-NETWORK-ENDPOINT-REF", "HW-PIN-GROUP", "CONTROLLED-FID-REF", "ECU-COMM-PORT-INSTANCES", "REMOTE-UNICAST-CONFIG-REF", "PROCESS-DESIGN-REF", "BSW-MODE-GROUP-REF", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "IEEE-1722-TP-IIDC-CONNECTION", "TIMING-PROPS", "IMPLEMENTED-ENTRYS", "TRUE-ACTION-LIST-REF", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "COMPU-INTERNAL-TO-PHYS", "SYNC-DATA-ID-LIST", "IIDC-CHANNEL", "MACHINE-DESIGN", "APPLICATION-PARTITION", "ALIVE-CHECK-RESPONSE-TIMEOUT", "PULSE-TEST", "IPV-6-ADDRESS-SOURCE", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "SOMEIP-SERVICE-VERSION", "CHECKPOINT-INITIAL-REFS", "MAXIMUM-EXECUTION-TIME", "TGROUP", "R-PORT-PROTOTYPE-REF", "REMOTE-PORT-REF", "I-SIGNAL-TO-I-PDU-MAPPINGS", "RESPONSE-ON-SECOND-DECLINED-REQUEST", "VF", "RUNS-INSIDE-EXCLUSIVE-AREA-REF", "PATH-MTU-TIMEOUT", "INITIAL-DELAY-MAX-VALUE", "MODE-CONDITION-REF", "TRANSFORMER-CHAIN-REF", "STARTUP-CONFIG-SET", "LABELED-LIST", "USER-DEFINED-I-PDU", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "PDU-COLLECTION-SEMANTICS", "ASSOCIATED-COM-I-PDU-GROUP-REF", "QUEUE-SIZE", "RULE-BASED-AXIS-CONT", "RANGE-MIN", "USE-SMART-SENSOR-API", "RESTRICT-UPDATE", "BSW-RESOLUTION", "SUPPORT-DIRTY-FLAG", "SYSTEM-SIGNAL-GROUP-TO-COM-RESOURCE-MAPPINGS", "TCP-ROLE", "HEALTH-CHANNEL-ID", "ADDRESS-RANGE-LOWER-BOUND", "SOFTWARE-CLUSTER-REF", "NAGLES-ALGORITHM", "TRIGGER-REFS", "CONTEXT-DATA-PROTOTYPE-IN-SR-REF", "HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS", "SERVICE-INTERFACE-REF", "FIREWALL-REF", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "DESTINATION-MAC-ADDRESS-MASK", "TD-EVENT-OPERATION-TYPE", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL", "SERVICE-INTERFACE-ID", "KIND", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "IKE-REKEY-TIME", "IP-SEC-CONFIG-PROPS-REF", "SD-SERVER-CONFIG", "READ-DATA-CLASS-REF", "MODE-DRIVEN-FALSE-CONDITIONS", "DO-IP-GID-NEEDS", "SOMEIP-REQUIRED-EVENT-GROUP", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "ACL-CONTEXTS", "BUILD-ACTION", "N-DATA-SETS", "VERIFY-IREF", "REQUIRED-ARTIFACTS", "SOMEIP-TP-CHANNEL", "SAMPLES-PER-MICROTICK", "CRYPTO-ALG-ID", "FDC-THRESHOLD-STORAGE-VALUE", "STOP-IREF", "FM-ATTRIBUTE-VALUE", "FUP-DATA-ID-LISTS", "PARAMETER-REF", "IDSM-INSTANCES", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING", "R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF", "OFFSET-CORRECTION-START", "SW-DATA-DEF-PROPS", "SERVICE-INSTANCE-ID", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "FIELD-REQUEST-TOPIC-NAME", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "EXECUTION-ORDER-CONSTRAINT-TYPE", "CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING", "MODE-SWITCH-SENDER-COM-SPEC", "DPG-DOES-NOT-REQUIRE-COHERENCYS", "SYNC-DATA-ID-LISTS", "P", "FILTER-CHAINS", "EXECUTABLE-REQUIRED-PORT-IREF", "APPLICATION-INTERFACE", "SO-CON-I-PDU-IDENTIFIER", "DLT-MESSAGE-COLLECTION-SET", "SECOND-PORT-REF", "DLT-MESSAGE", "PROVIDED-SERVICE-INSTANCE", "AUTOSAR-DATA-TYPE-REF", "L-2", "CONDITION-FORMULA", "PORT-INTERFACE-BLUEPRINT-REF", "PARAMETER-ID", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "GLOBAL-SUPERVISION-ENTITYS", "ACCESS-POINT-REF", "CONTENT-URI", "REQUIRED-RTE-VENDOR", "EVALUATED-VARIANT-REFS", "REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF", "FUNCTION-GROUP-IREF", "PRE-ACTIVATE-IREF", "BYPASS-ETHER-TYPES", "SECOND-TRIGGER-REF", "DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING", "CLEAR-CONDITION-REFS", "SIGNATURE-SCHEME-REFS", "SEC-OC-CRYPTO-MAPPING-REF", "MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING", "ADAPTIVE-FIREWALL-MODULE-INSTANTIATION", "CRC-DOMAIN-NUMBER", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "DIAGNOSTIC-ECU-RESET", "ALIVE-TIMEOUT", "SOURCE-IP-ADDRESS", "DATA-CONSTR-REF", "AGGREGATION-REF", "SA-OVER-TIME", "MAX-NUMBER-OF-CONNECTIONS", "TIMING-MODE-REF", "MAXIMUM", "IPV-4-RULE", "IEEE-1722-TP-ETHERNET-FRAME", "EXTERNAL-TRIGGERING-POINT", "SIGNAL-AGE", "SERVICE-INTERFACE", "IS-SERVICE", "RUNTIME-ERROR", "REQUIRED-CLIENT-SERVER-ENTRYS", "BLOCK-IF-STATE-ACTIVE-CP-REFS", "CONNECTION-PRIO-PDUS", "CONSUMED-SERVICE-INSTANCE-REF", "MINIMUM-DELAY", "J-1939-CONTROLLER-APPLICATION-REF", "CRC-SOURCE-PORT-IDENTITY", "CAN-TP-CONNECTION", "PHM-CONTRIBUTION-REF", "DDS-CP-PARTITION", "CRYPTO-CERTIFICATES", "HANDLE-DATA-STATUS", "BLOCK-IF-STATE-ACTIVE-AP-IREFS", "GLOBAL-TIME-SUB-DOMAINS", "GLOBAL-TIME-DOMAIN-PROPERTYS", "ESTIMATED-DURATION-OF-OPERATION", "SOURCE-INSTANCE-IREF", "COM-OFFER-SERVICE-GRANT-DESIGN", "AUXILIARY-FIELDS", "FLEXRAY-AR-TP-CONFIG", "ALGORITHM-FAMILY", "DEFAULT-VALUE-HANDLING", "DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA", "APPLICATION-DESCRIPTION", "PDUR-I-PDU-GROUP", "REVERSED-TP-SDU-REF", "SENDER-RECEIVER-ANNOTATIONS", "MAXIMUM-NUMBER-OF-RETRIES", "IUMPR-REFS", "L-1", "POST-BUILD-VARIANT-MULTIPLICITY", "CLIENT-COM-SPEC", "INCLUDED-VARIANT-REFS", "APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION", "DIAGNOSTIC-ROUTINE", "FRAME-PID", "MAX-SAMPLES", "WAKEUP-FRAME-REF", "APPLIES-TO", "RESOURCE-GUARD-VALUE", "APPLICATION-ENDPOINT-REF", "RPT-SUPPORT-DATA", "LOG-TRACE-PROCESS-DESC", "TIME-BASE-RESOURCE-REF-CONDITIONAL", "CAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "ALLOWED-I-PV-6-EXT-HEADERS-REF", "COUPLING-TYPE", "TLS-JOB-MAPPING", "APPLICATION-ENTRY", "TARGET-TRIGGER-REF", "REQUIRED-ENVIRONMENT-REF", "MODE-ELEMENT-REF", "SOCKET-OPTION", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "CONTAINED-FIBEX-ELEMENT-REFS", "VARIABLE-ACCESS-IREF", "CHANGE", "NM-REPEAT-MESSAGE-BIT-ENABLE", "SOFTWARE-CLUSTER-DEPENDENCY-FORMULA", "TX-BIT-RATE-SWITCH", "MAX-NUMBER-OF-TIME-QUANTA-PER-BIT", "IKE-AUTHENTICATION-METHOD", "LIN-TP-N-SDU-REF", "WAKEUP-FORWARD-REMOTE-ENABLED", "RESOURCES", "DIAGNOSTIC-IO-CONTROL-NEEDS", "EXITED-MODE-REF", "SYSTEM-DOCUMENTATIONS", "MAPPED-OPERATION-IREF", "J-1939-CLUSTER-VARIANTS", "CALL-SIGNAL-TRIGGERING-REF", "LOCAL-CERTIFICATE-REFS", "J-1939-REQUESTABLE", "TIMING-DESCRIPTION-EVENT-CHAIN", "EXPLICIT-INTER-RUNNABLE-VARIABLES", "R-PORT-IN-COMPOSITION-INSTANCE-REF", "RELATIVE", "LIN-TP-NODE", "MASTERED-TRIGGER-REF", "FLAT-MAP", "COMMITTED-INFORMATION-RATE", "STATE-REQUEST-REF", "COLLECTION-THRESHOLD", "HANDLE-OUT-OF-RANGE-STATUS", "MIN-VALUE", "INVALIDATION-POLICYS", "AFFECTED-BSW-MODULE", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING", "UPPER-LIMIT", "SYSTEM-SIGNAL", "SW-CLASS-PROTOTYPE-REF", "HW-ELEMENT-REFS", "TARGET-PARAMETER-DATA-PROTOTYPE-REF", "COLLECTION-TRIGGER", "TRIGGER-IREF", "MIN-DEADLINE", "TCP-KEEP-ALIVE-ENABLED", "FLEXRAY-AR-TP-CONNECTION", "TOPIC-1", "DESTINATION-CONTEXT", "PARAMETER-INTERFACE", "REQUEST-FILE-TRANSFER-CLASS-REF", "NM-MAIN-FUNCTION-PERIOD", "CHECKPOINT-ID", "DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT", "EXIT-MODE-DECLARATION-REF", "PARAMETERS", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "PROVIDED-CLIENT-SERVER-ENTRYS", "DOIP-CONNECTION-REF", "FILE-NAME", "DEFAULT-LOG-THRESHOLD", "REGULAR-EXPRESSION", "RATIONALE", "MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS", "ALIAS-NAME-ASSIGNMENT", "FIREWALL-RULE-REF", "CRYPTO-PROVIDER-INTERFACE", "MAC-MULTICAST-ADDRESS-REF", "ENVIRONMENTAL-CONDITION-REF", "SDG-CONSTRAINT-REFS", "MASK", "RVF-ACTIVE-PIXELS", "WINDOW-SIZE", "IIDC-STREAM-FORMAT", "PREDECESSOR-FIFO-REF", "MAXIMUM-TRANSMIT-BUFFER-LENGTH", "AUTHENTICATION-CLASS-REF", "CONTEXT-R-PORT-PROTOTYPE-REF", "MODE-REPORTER-PORT-IREF", "MC-PARAMETER-ELEMENT-GROUP", "CONSUMED-DDS-OPERATIONS", "OFFSET-TIME-DOMAIN-REF", "TCP-INITIAL-INACTIVITY-TIME", "BINARY-MANIFEST-REQUIRE-RESOURCE", "END-TO-END-TRANSFORMATION-DESCRIPTION", "DEST-SDG-REF", "RPT-SERVICE-POINT-POST-REF", "EXPECTED-ENTRYS", "AUTHENTIC-CONNECTION-PROPSS", "DYNAMIC-LENGTH", "CRYPTO-KEY-SLOT", "WORST-CASE-EXECUTION-TIME", "CONFIG-ELEMENT-DEF-LOCAL-REF", "TARGET-REF", "POST-BUILD-VARIANT-CRITERION-VALUE", "TEXT-TABLE-MAPPING", "BUILD-ACTION-ENVIRONMENTS", "FRESHNESS-COUNTER-SYNC-ATTEMPTS", "NM-ECU", "LIN-TP-CONFIG", "SDGS", "REMOTE-MULTICAST-CONFIG-REFS", "RAW-DATA-STREAM-CLIENT-INTERFACE", "TARGET-CHANNEL", "I-PDU-TIMING-SPECIFICATIONS", "SW-RECORD-LAYOUT-REF", "SOURCE-SERVICE-INTERFACE-REF", "NM-IMMEDIATE-RESTART-ENABLED", "APPLICATION-TYPE-REF", "RESPONSE-DATA-SIZE", "DO-IP-TP-CONFIG", "CONTEXT-COMPONENT-REF", "IDS-COMMON-ELEMENT-REF-CONDITIONAL", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "CATEGORY", "APPLICATION-ID", "TCP-TTL", "CAN-FRAME-RX-BEHAVIOR", "CRYPTO-PROVIDER-REF", "RESOURCE-NEEDS-REF", "RAW-DATA-STREAM-REF", "INDIRECT-API", "ECUC-REFERENCE-VALUE", "IDENTITY", "TRCV-DELAY-COMPENSATION-OFFSET", "BSW-INTERNAL-TRIGGERING-POINT-REF", "MULTIPLE-CONFIGURATION-CONTAINER", "FUNCTION-IDENTIFIER-REF", "WAIT-POINTS", "CONDITIONAL-CHANGE-NAD", "SW-COMPONENT-DOCUMENTATIONS", "MAXIMUM-SEGMENT-LENGTH-REQUEST", "DIAGNOSTIC-COMPONENT-NEEDS", "REQUIRED-PACKAGE-ELEMENT-REFS", "ROLE-BASED-RESOURCE-DEPENDENCY", "SECOND-VALUE", "DLT-APPLICATION-REF", "MAC-SEC-CIPHER-SUITE-CONFIG", "DECODING-CORRECTION", "MODIFIED-DATAS", "PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL", "NUM-FAILED-SECURITY-ACCESS", "LICENSE-REF", "DETERMINISTIC-CLIENT", "COMPU-METHOD", "LOCAL-SUPERVISION-ENTITY-REF", "SEQUENTIAL-INSTRUCTIONS-END", "IEEE-1722-TP-RVF-CONNECTION", "SIGNAL-I-PDU-REPLICATION", "OBJECT-DEFINITION-REF", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "PROCESS-TO-MACHINE-MAPPINGS", "SEC-OC-JOB-MAPPINGS", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "ACCEPTED-REMOTE-CERTIFICATE-REF", "DEADLINE-MAX", "INTERNAL-TRIGGERING-POINT-POLICYS", "MAC-SEC-PROPS", "PRE-ACTIVATE-REFS", "CYCLE-COUNTER", "DO-IP-POWER-MODE-STATUS-NEEDS", "PROVIDED-OUTER-PORT-REF", "MC-DATA-INSTANCE", "DIAGNOSTIC-PROPS", "CRYPTO-ALGORITHM-STRING", "PORT-API-OPTIONS", "PAYLOAD-LENGTH", "HW-PIN-GROUPS", "TCP-IP-IP-FRAGMENTATION-RX-ENABLED", "OBD-RATIO-SERVICE-NEEDS", "DURABILITY", "REKEY-INTERVAL", "SW-COMP-TO-ECU-MAPPING-REF", "REQUIREMENT-REFS", "MULTICAST-REF", "EXECUTION-CONTEXT-REFS", "TLS-CIPHER-SUITES", "FIGURE-CAPTION", "ACTION-TYPE", "CONTEXT-DATA-PROTOTYPE-REF", "CAN-NM-CLUSTER", "IP-TP-RULE", "TD-EVENT-OPERATION", "USER-DEFINED-EVENT-DEPLOYMENT", "IDS-COMMON-ELEMENT-REF", "PLCA-MAX-BURST-TIMER", "TARGET-RUNNABLE-ENTITY-GROUP-REF", "DLT-ECU-ID", "MIN-SAMPLE-POINT", "SOCKET-OPTIONS", "ECUC-STRING-PARAM-DEF-VARIANTS", "BULK-DATA-CATEGORY", "IN-PLACE", "CONFIG-CLASS", "NM-DATA-INFORMATION", "PARAMETER-VALUES", "DIAGNOSTIC-ENV-DATA-ELEMENT-CONDITION", "RECOVERY-NOTIFICATION-RETRY", "LIFE-CYCLE-STATE", "MICROTICK-DURATION", "HW-PIN-GROUP-CONTENT", "DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF", "DEBOUNCE-BEHAVIOR", "RX-PDU-TRIGGERING-REF", "SDU-REF", "DECOMPOSITIONS", "UDP-NM-CLUSTER", "EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE", "STREAM-FILTER-PORT-RANGE", "STORE-AT-SHUTDOWN", "SERVER-PORT-REF", "STARTUP-OPTION", "ALTERNATIVE-NAME", "PATTERN", "PDU-COUNTER-THRESHOLD", "RATE-DEVIATION-MEASUREMENT-DURATION", "METHOD-TOPICS-ACCESS-RULE-REF", "LICENSE-REFS", "CONSTANT-SPECIFICATION", "TOOL-VERSION", "ROUGH-ESTIMATE-HEAP-USAGE", "INITIAL-DELAY-MIN-VALUE", "OCCURRENCE-EXPRESSION", "TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED", "SERVICE-CONTROL", "TARGET-PDU-TRIGGERINGS", "PNC-MAPPING-REF", "DIAGNOSTIC-INDICATOR-INTERFACE", "TYPE-TREF", "SW-CODE-SYNTAX", "SIGNAL-PATH-CONSTRAINTS", "CRYPTO-PROVIDER", "DOC-REVISION", "NEEDS-ORIGINAL-DATA", "FIRST-TO-SECOND-DATA-TRANSFORMATION-REF", "COUNTER-JUMP-UP", "OFS-DATA-ID-LISTS", "INDICATOR-FAILURE-CYCLE-COUNTER-THRESHOLD", "GLOBAL-TIME-DOMAIN-REF-CONDITIONAL", "DIAGNOSTIC-TEST-RESULT-REF", "BYPASS-VLANS", "INTERFACE-REF", "SDG-CLASSES", "HW-ELEMENT-REF-CONDITIONAL", "CONSTANT-VALUE-MAPPING-REFS", "TERMINATING-CHECKPOINT-REF", "IS-STRUCT-WITH-OPTIONAL-ELEMENT", "UNRESOLVED-REFERENCE-RESTRICTION", "ROOT-DATA-PROTOTYPE-REF", "MEMORY-DESTINATION-REFS", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "MAXIMUM-SUPPORTED-UCM-VERSION", "OPERATION-MAPPINGS", "SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING", "METHOD-REFS", "STREAM-IDENTIFIER", "MEMORY-RANGE-REFS", "SERVICE-PROXY-SW-COMPONENT-TYPE", "I-SIGNAL-PORT-REFS", "VFC-IREFS", "TRANCEIVER-STANDBY-DELAY", "PATH", "DIAGNOSTIC-AUTHENTICATION-CLASS", "ASYNCHRONOUS-SERVER-CALL-POINT", "DIAGNOSTIC-CONNECTION", "ARP-PROPS", "COMPOUND-PRIMITIVE-ARGUMENTS", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "J-1939-DTC-VALUE", "SDG-REFERENCE", "IMPLEMENTATION-DATA-TYPE", "DLT-MESSAGES", "TIME-SYNC-CLIENT", "CONSUMED-EVENT-GROUP", "PAYLOAD-TYPE", "MODE", "INDEX-DATA-TYPE-REF", "REQUIRED-SERVICE-VERSION", "STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR", "P-PORT-IN-COMPOSITION-INSTANCE-REF", "REMOTE-CLIENT-CONFIG", "TIME-SYNC-CORRECTION", "SW-BASE-TYPE", "OS-MODULE-INSTANTIATION", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "LOCAL-NETWORK-ENDPOINT-REF", "TLS-CRYPTO-CIPHER-SUITE", "SUPPORTS-MULTIPLE-INSTANTIATION", "MULTICAST-SD-IP-ADDRESS-REF", "FILTER-ACTION-DEST-PORT-MODIFICATION", "P-2-STAR-SERVER-MAX", "PSK-IDENTITY-HINT", "METHOD-ID", "CONNECTION-NEGOTIATION-BEHAVIOR", "BSW-MODE-SWITCH-EVENT", "SUPPLIER-ID", "FUNCTIONAL-CLUSTER-REF", "DISCOVERY-PROTECTION-KIND", "NM-STATE-REQUEST", "CONDITION", "PROCESSORS", "EVENT-GROUP-REFS", "CAN-FRAME-TX-BEHAVIOR", "TCP-NAGLE-ENABLED", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "NM-SYNCHRONIZATION-POINT-ENABLED", "COMMUNICATION-BUFFER-LOCKING", "METHOD-REQUEST-PROPSS", "APPLICATION-PRIMITIVE-DATA-TYPE", "SOURCE-REF", "SCHEDULER-NAME-PREFIX-REF", "CAN-CONTROLLER-XL-ATTRIBUTES", "ASSIGNED-TRAFFIC-CLASSS", "CONTROLLED-DATA-ELEMENT-REF", "RELIABILITY-KIND", "HW-ATTRIBUTE-VALUES", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "ELLIPTIC-CURVE-REF", "OPTION-KIND", "RETURN-CODE", "RELATIONS", "CONNECTED-INDICATORS", "MIN-LENGTH", "TRANSIENT-FAULT", "PREDEFINED-VARIANT", "DEFAULT-ROUTER", "GRANT-REF", "ARGUMENTS", "PNC-WAKEUP-DLC", "FUNCTION-GROUP-STATE-IREFS", "SESSION-ID-SUPPORT", "HW-COMMUNICATION-PORT-REF", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "HW-PIN-GROUP-CONNECTIONS", "KEY-SLOT-ONLY-ENABLED", "VARIATION-POINT", "SEVERITY", "COM-CONFIGURATION-RX-TIME-BASE", "LOC-MEASUREMENT-SET", "PROTOCOL", "REDUNDANCY", "MODE-DECLARATION-GROUP-PROTOTYPE-REF", "MCD-IDENTIFIER", "DIAGNOSTIC-SERVICE-TABLE", "IMMEDIATE-NV-DATA-STORAGE", "DIAGNOSTIC-DATA-ELEMENT-REF", "ROOT-DATA-PROTOTYPE-IN-SR-REF", "NM-INSTANTIATION", "LC-STATES", "SWC-TO-APPLICATION-PARTITION-MAPPINGS", "SECOND-PHYSICAL-DIMENSION-REF", "TCP-IP-DHCP-V-6-CNF-DELAY-MAX", "WAKEUP-SYMBOL-RX-WINDOW", "LISTEN-TIMEOUT", "BSW-DISTINGUISHED-PARTITION", "MAX-MAJOR-VERSION", "MEMORY-ID", "IS-DYNAMIC-LENGTH-FIELD-SIZE", "PHYSICAL-DIMENSION-MAPPING", "TARGET-EEC-REFS", "BASE-COMPOSITION-REF", "IMPL-INIT-VALUE", "DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "MAX-RETRIES", "USER-DEFINED-PDU", "SW-DATA-DEF-PROPS-CONDITIONAL", "SYSTEM-SIGNAL-TO-COM-RESOURCE-MAPPINGS", "END-TO-END-TRANSFORMATION-COM-SPEC-PROPS", "PORT-BLUEPRINT-MAPPING", "AUTOSAR-VARIABLE-IREF", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "STATE-MANAGEMENT-NM-ACTION-ITEM", "METHOD-CALL-REFS", "ACCESSED-MODE-GROUPS", "AUTO-AVAILABLE", "SW-SYSTEMCONST-REF", "CRC-SECURED", "SW-RECORD-LAYOUT-V", "MAX-ERROR-STATE-INVALID", "TP-ADDRESSS", "WRITING-FREQUENCY", "PDU-TRIGGERINGS", "ARGUMENTSS", "SUBSTITUTED-FRAME-REFS", "P-PORT-PROTOTYPE-IREF", "DEFAULT-GATEWAY", "ECUC-DEF-EDITION", "RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS", "MIN-SYNC-JUMP-WIDTH", "XCP-PDU", "CONFIG-ELEMENT-DEF-GLOBAL-REF", "AUTHENTICATION-ROLE-REF", "TRANSMISSION-MODE-CONDITION", "SOURCE-SERVICE-INTERFACE-REFS", "TARGET-EEC-REF", "BUILD-TYPE", "SERVICE-INTERFACE-PEDIGREE", "DATA-IDS", "SOURCE-MIN-ADDRESS", "INSTALLATION-BEHAVIOR", "SUBSTITUTED-FRAME-REF", "SOMEIP-METHOD-PROPS", "TCP-FAST-RETRANSMIT-ENABLED", "REQUIRED-FIBEX-ELEMENT-REFS", "TIMEOUT-AR", "COUPLING-PORT-REF-CONDITIONAL", "MAPPING-CONSTRAINTS", "FUNCTION-GROUP-IREFS", "BINARY-MANIFEST-ITEM-NUMERICAL-VALUE", "STARTUP-CONFIG-REF", "CONTEXT-PORT-REF", "VERSION-DRIVEN-FIND-BEHAVIOR", "COUNTER-JUMP-UP-VALUE", "PRM", "ECUC-QUERY-EXPRESSION", "MASTER-REF", "CONTEXT-PORT-PROTOTYPE-REF", "BSW-BEHAVIOR-REF", "TCP-FIN-WAIT-2-TIMEOUT", "CHANNEL-NAME", "CALIBRATION-PARAMETER-VALUES", "DATA-LINK-LAYER-RULE", "OPT-ARGUMENT-REF", "SW-CARB-DOCS", "COUNTER-PASSED-THRESHOLD", "NAS-TIMEOUT", "REQUEST-RESPONSE-DELAY", "COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS", "SYSTEM-MEMORY-USAGES", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "BINARY-MANIFEST-ITEM-DEFINITION", "QUEUED-RECEIVER-COM-SPEC", "DIAGNOSTIC-SESSION-REF", "TAG-ID", "FIRE-AND-FORGET", "ENTITYS", "PROTOCOL-VERSION", "DOCUMENTATION-CONTEXT", "PORTS", "ECUC-VALIDATION-CONDS", "TLS-JOB-REQUIREMENT-REF", "VEHICLE-ANNOUNCEMENT-INTERVAL", "RVF-TOTAL-LINES", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "LIFE-CYCLE-INFO-SET", "HEADER-FILE", "CALLBACK-HEADER-REFS", "I-SIGNAL-TO-I-PDU-MAPPING", "TIME-SYNCHRONIZATION", "CUSTOM-TRIGGER", "SENSOR-ACTUATOR-REF", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "ADMIN-DATA", "SWITCH-STREAM-FILTER-ENTRY", "UCM-MODULE-INSTANTIATION", "MC-DATA-ACCESS-DETAILS", "DIAGNOSTIC-ADDRESSES", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "RECORD-VALUE-SPECIFICATION", "A-2-L-DISPLAY-TEXT", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "BSW-DATA-RECEIVED-EVENT", "PROCESS-MODE-MACHINE", "RUNNABLE-ENTITY", "RELATED-TRACE-ITEM-REF", "MANUFACTURER-VALIDATION-ORDER-REFS", "START-ADDRESS", "MAX-FR-IF", "RESPOND-TO-RESET", "DIAGNOSTIC-EVENT-PORT-MAPPING", "BOOLEAN-VALUE-VARIATION-POINT", "CKN-REF", "SETTER-RETURN-SIGNAL-REF", "SIGNAL-I-PDU-COUNTER", "HOLD-OVER-TIME", "NM-USER-DATA-LENGTH", "POSSIBLE-ERRORS", "AUTHENTICATION-ENABLED", "ECUC-URI-REFERENCE-DEF", "WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS", "SIGNAL-BASED-METHOD-REF", "VLAN-ID", "SOFTWARE-CLUSTER-REQUIREMENT", "PNC-VECTOR-OFFSET", "SERVICE-OFFER-TIME-TO-LIVE", "PID-REFS", "STATE-MACHINE", "ABS", "SW-GENERIC-AXIS-PARAM-TYPE-REF", "EXECUTION-CONTEXT-REF", "CONSUMED-EVENT-GROUPS", "SUBFUNCTION", "BSW-TIMING-EVENT", "FRESHNESS-VALUE-LENGTH", "AUTOSAR-ENGINEERING-OBJECT", "INITIALIZATION-VECTOR-LENGTH", "WRITE-ONLY-ONCE", "REQUEST-CONFIGURATIONS", "LOG-SINK-REF", "LIN-SLAVE", "VARIABLE-ACCESS-IREFS", "ADDRESS-SEMANTICS", "BSW-PER-INSTANCE-MEMORY-POLICY", "SW-VARIABLE-IMPL", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "REQUIRED-DATAS", "PATTERN-LENGTH", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "TD-EVENT-FRAME-ETHERNET", "INTERNAL-BEHAVIORS", "DETERMINISTIC-SYNC-MASTER", "TD-PDU-TRIGGERING-FILTER-REF", "BURST-PATTERN-EVENT-TRIGGERING", "RPT-WRITES", "MICRO-INITIAL-OFFSET-A", "DIAGNOSTIC-CONNECTION-REF", "LUMINOUS-INTENSITY-EXP", "REST-ARRAY-PROPERTY-DEF", "STREAM-IDENTIFICATION-HANDLE-REFS", "OFFER-CYCLIC-DELAY", "VENDOR-SPECIFIC-SERVICE-NEEDS", "SIGNATURE-SCHEME-ID", "INTERPOLATION-ROUTINE-MAPPING-SET", "TIMING-DESCRIPTION-REF", "COMMUNICATION-CONNECTOR-REF-CONDITIONAL", "VARIABLE-DATA-PROTOTYPE-IREF", "ASSEMBLY-SW-CONNECTOR", "SW-COMP-TO-ECU-MAPPING-REFS", "CYCLE-STATUS-STORAGE", "REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS", "BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "FIRST-APPLICATION-ERROR-REF", "BANDWIDTH-LIMITATION", "DIAGNOSTIC-PARAMETER", "WRITTEN-READ-NV-DATA", "VARIATION-POINT-PROXYS", "FLOW-CONTROL-PDU-REF", "SERVICE-ELEMENT-SECURE-COM-CONFIG-REF", "SUPERVISED-ENTITY-NEEDS", "ECU-TASK-PROXY-REFS", "FLEXRAY-NM-CLUSTER-COUPLING", "NM-HANDLE-REF", "LET-DATA-EXCHANGE-PARADIGM", "NOTIFICATIONS", "NAMED-CURVE-ID", "INTERFACE-BLUEPRINT-REF", "MODE-DECLARATION-MAPPING-SET-REF", "APPLICATION-ARRAY-ELEMENT-REF", "RESOURCE-GROUPS", "PORT-PROTOTYPE-REF", "DATA-PROVIDER", "MIN-NUMBER-OF-OCCURRENCES", "SLOT-ID", "SW-INTERPOLATION-METHOD", "TCP-IP-NDP-MIN-RANDOM-FACTOR", "RATE-CORRECTION-OUT", "LIN-CLUSTER-VARIANTS", "ECUC-MODULE-DEF", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "RPT-EXECUTABLE-ENTITY", "INPUT-DATAS", "HW-PIN-GROUP-CONNECTOR", "DIAGNOSTIC-DTC-CHANGE-TRIGGER", "SERVICE-NEEDS", "ACCESSED-DATA-PROTOTYPE-IREF", "MAX-SAMPLE-POINT", "ASSOCIATED-RTE-PLUGIN-REF", "PRIORITY", "MAXIMUM-INTENDED-BINDING-TIME", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR", "PORT-API-OPTION", "MAX-SLOTS-PER-CYCLE", "HEALING-CYCLE-REF", "INNER-GROUP-IREF", "CRYPTO-DRIVER", "MEMORY-ALLOCATION-KEYWORD-POLICY", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "PRECONFIGURED-CONFIGURATION-REFS", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "PROGRAMMING-LANGUAGE", "AP-APPLICATION-ERROR-REF", "TD-EVENT-SERVICE-INSTANCE-EVENT", "PNC-GROUP-REFS", "BOR-TIME-TX-ENSURED", "PREVIOUS-EXECUTABLE-VERSION", "PORT-PROTOTYPE-IREF", "DATA-TYPE-MAPPING-REF", "RESET-PENDING-BIT-ON-OVERFLOW", "RPT-WRITE-ACCESS", "IMPLEMENTATION-REF", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "SIGNATURE-SUPPORT-CP", "MODE-REQUEST-TYPE-MAPS", "PHYSICAL-DIMENSION-MAPPINGS", "LOCAL-PORT-RANGE-START", "TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL", "ON-FAIL-PERMISSIVE-MODE", "PRESTORAGE-FREEZE-FRAME", "CRYPTO-CERTIFICATE", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "DATA-TRANSFORMATION", "RTP-TP", "RX-TIMEOUT-TIME", "ETH-SWITCH-PORT-GROUP-DERIVATION", "COMPONENT-REF", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "SETTER-CALL-SIGNAL-REF", "SOMEIP-EVENT", "SW-AXIS-GENERIC", "FIFO-DEPTH", "MACHINE-MODE-IREF", "HARDWARE-PLATFORM", "DENOMINATOR-CONDITION", "LOCAL-SUPERVISIONS", "UCM-PROCESSINGS", "TCP-KEEP-ALIVE-INTERVAL", "TRANSMISSION-TRIGGER", "BULK-NV-BLOCK", "DEPENDENCY-ON-ARTIFACT", "REQUEST-TYPE", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING", "TLS-CONNECTION-GROUP", "RESPONSE-ERROR-REF", "ACCURACY-EXT", "DISABLED-IN-MODE-IREFS", "MIN-MARGIN", "TYPE-TAILORINGS", "FLEXRAY-TP-NODE", "MAX-BS", "MIN-PWM-S", "REQUIRED-DESIGN-ELEMENT-REF", "ROUTING-GROUP-REF", "SWC-TO-ECU-MAPPING-CONSTRAINT", "CONTAINER-I-PDU-HEADER-BYTE-ORDER", "DTC-STATUS-MASK", "REPLY-TOPIC-NAME", "RELATIVE-TOLERANCE", "SWC-BSW-MAPPING", "TARGET-PROVIDED-OPERATION-REF", "LIN-CLUSTER", "MAX-MINOR-VERSION", "SEC-OC-JOB-REQUIREMENT-REF", "SOFTWARE-CONTEXT", "TX-PDU-POOL-REF", "MAX-SIZE-TO-FILL", "PDU-COUNTERS", "DYNAMIC-RUNTIME-LENGTH-HANDLING", "DIAGNOSTIC-EXTRACT-REF", "BLUEPRINT-POLICY-SINGLE", "MATCHING-RULE-REF", "ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "ADDRESS", "SOURCE-CHANNEL", "TRACEABLE-TABLE", "TRANSFER-PROPERTY", "CLIENT-SERVER-ARRAY-TYPE-MAPPING", "DATA-WRITE-COMPLETED-EVENT", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "CAN-FD-FRAME-SUPPORT", "OBJECT-REFS", "FUNCTION-GROUP-MODE-IREF", "DESTINATION-ORDER-KIND", "MAX-GRADIENT", "SD-CLIENT-TIMER-CONFIGS", "REFERENCE-VALUE-REF", "DISABLE-END-TO-END-STATE-MACHINE", "CONTROL-ENABLE-MASK-BITS", "VALUE-ACCESS", "ERROR-HANDLING", "TCP-KEEP-ALIVE-TIME", "COM-MANAGEMENT-PORT-GROUP-IREFS", "MAX-EVENTS-IN-INTERVAL", "LIST", "PCM-BIT-DEPTH", "REG-REQUIRES-STABILITYS", "EXTERN-OFFSET-CORRECTION", "INTERFACE-MAPPING", "SD-SERVER-EVENT-CONFIG", "SECURITY-EVENT-FILTER-CHAIN-REF", "AR-PACKAGE", "MIN-DESTINATION-PORT-NUMBER", "IMPLIED-OPERATION-REFS", "COMMUNICATION-CONNECTOR-REFS", "CALLED-ENTRY-REF", "JOB-REF", "FILTER-ACTION-DROP-FRAME", "LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING", "EVENTS", "COUPLING-ELEMENT-SWITCH-DETAILS", "MORE-FRAGMENTS", "DIAGNOSTIC-CONTRIBUTION-REF", "BUS-IDLE-TIMEOUT-PERIOD", "SUPERVISION-CONDITION", "TROUBLE-CODE-OBD-REF", "I-PDU-SIGNAL-PROCESSING", "GLOBAL-TIME-CORRECTION-PROPS", "BLUEPRINT-POLICYS", "DTC-STATUS-AVAILABILITY-MASK", "TIMING-DESCRIPTIONS", "TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "FLEXRAY-FIFOS", "GATEWAY", "CALLBACK-HEADER-REF", "DEPENDENCIES", "SERVICE-INSTANCE-REFS", "IDSM-RATE-LIMITATION-REF", "RESET-CONFIRMED-BIT-ON-OVERFLOW", "SDG-CONSTRAINT-REF", "TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE", "COMPONENT-CLUSTERING", "USER-DEFINED-COMMUNICATION-CONNECTOR", "REFERENCE-VALUES", "RECEIVER-ANNOTATION", "LOCAL-PORT-RANGE-END", "ITEM", "FLEXRAY-COMMUNICATION-CONNECTOR", "STORAGE-STATE-EVALUATION", "INTERNAL-TRIGGERING-POINT", "COM-FIND-SERVICE-GRANT-DESIGN", "IMMEDIATE-RESUME-TIME", "ACL-OPERATION-REF", "MAX-DID-TO-READ", "DIAGNOSTIC-AUTH-ROLE", "DATA-PROTOTYPE-IN-PORT-INTERFACE-REF", "DESIGN-REF", "PROVIDER-REF", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "IS-REENTRANT", "OPERATIONS", "UNIT-DISPLAY-NAME", "SOURCE-MAC-ADDRESS-MASK", "BSW-MODULE-DESCRIPTION", "COMPARE-TYPE", "BLOCK-STATE", "INDICATIONS", "DATA-FILTER", "DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL", "TESTED-ITEM-REF", "CALCULATION-FORMULA", "LABEL", "DDS-REQUIRED-SERVICE-INSTANCE", "ECUC-ADD-INFO-PARAM-DEF", "HW-PORT-MAPPING", "ECUC-ENUMERATION-PARAM-DEF", "ESTIMATED-EXECUTION-TIME", "TYPE", "PROCESS-STATE-IREF", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "ASYNCHRONOUS-TRAFFIC-SHAPER-REF", "BSW-MODE-SENDER-POLICY", "DIAGNOSTIC-SOVD-BULK-DATA", "REST-ELEMENT-DEF", "TIMING-CONDITIONS", "SOFTWARE-PACKAGE", "FIELD-GET-SET-QOS-PROPSS", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "TRACE-SWITCH-CONFIGURATION", "USE-CLIENT-AUTHENTICATION-REQUEST", "SERVER-ARGUMENT-IMPL-POLICY", "VEHICLE-DRIVER-NOTIFICATION", "REST-STRING-PROPERTY-DEF", "INTERNAL-CONSTRS", "UPSTREAM-REFERENCE-IREF", "SECURED-I-PDU", "UNICAST-CREDENTIALS", "TIME-OFFSET", "PREEMPTABILITY", "NUMBER-OF-PARALLEL-TCP-SESSIONS", "BASE-TYPE-ENCODING", "MSR-QUERY-P-2", "PORT-GROUPS", "INSTANCE-IDENTIFIER", "KEY-SLOTS", "TIME-PASSED-THRESHOLD", "DLT-LOG-SINK-REF", "MIN-SERVICE-POINT-ID", "PERSISTENCY-DEPLOYMENT-ELEMENT-REF", "DESTINATION-PHYSICAL-ADDRESS", "DIAGNOSTIC-OPERATION-CYCLE", "OPERATION-CYCLE", "FIREWALL-STATE-MODE-DECLARATION-REFS", "ACTUAL-EVENT-REF", "REGENERATED-PRIORITY", "RAW-DATA-STREAM-INTERFACE-REF", "KEYWORD-SET", "COUNT-PROFILE", "METHOD-RESPONSE-PROPSS", "TCP-UDP-CONFIG", "SIGNAL-BASED-EVENT-DEPLOYMENT", "DIAGNOSTIC-EVENT-INTERFACE", "CYCLIC-BACKUP-INTERVAL", "DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF", "WATCHDOG-PHM-ACTION-ITEM", "SERIALIZER", "TCP-IP-NDP-DESTINATION-CACHE-SIZE", "GLOBAL-TIME-DOMAIN", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "TTCAN-PHYSICAL-CHANNEL", "APPL-INIT-VALUE", "SW-MAX-TEXT-SIZE", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "ETHERNET-COMMUNICATION-CONNECTOR", "IS-REQUIRED", "TIME-VALUE-VALUE-VARIATION-POINT", "PLCA-MAX-BURST-COUNT", "CIPHER-SUITE-CONFIGS", "SYNCHRONIZATION-CONSTRAINT-TYPE", "ECUC-VALUE-CONFIGURATION-CLASS", "TAG-WITH-OPTIONAL-VALUE", "SIZE-OF-UNION-TYPE-SELECTOR-FIELD", "ECUC-ADD-INFO-PARAM-VALUE", "PDU-COLLECTION-TRIGGER", "INTERFACE-BLUEPRINT-MAPPINGS", "TRIGGER-PORT-ANNOTATION", "USER-DEFINED-GLOBAL-TIME-SLAVE", "CRYPTO-TRUST-MASTER-INTERFACE", "IMPLEMENTED-ENTRY-REF", "BUCKET-SIZE", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "FIRST-TIMEOUT", "REPORT-BEHAVIOR", "NM-REMOTE-SLEEP-IND-ENABLED", "SERVICE-INTERFACE-VERSION", "CP-SOFTWARE-CLUSTER-RESOURCE-REF", "FEATURE-MODEL-REFS", "UCM-DESCRIPTION", "IKE-CIPHER-SUITE-NAME", "INTERFACE-VERSION", "IPV-6-ADDRESS", "MAX-NUMBER-OF-NPDU-PER-CYCLE", "UDP-NM-CLUSTER-REF", "DIAGNOSTIC-INFO-TYPE", "PROPABILITY", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "LATENCY-BUDGET", "REQUEST-METHOD", "CAS-RX-LOW-MAX", "TTCAN-CLUSTER", "CHOICES", "SOURCE-EVENT-REF", "VALIDATION-CONFIGURATION", "POSSIBLE-AP-ERROR-SET-REF", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "DEF-LIST", "CONTROL-CONSUMED-EVENT-GROUP-REF", "AES-3-DATA-TYPE-L", "FEATURE-MODEL-REF", "CRYPTO-SERVICE-MAPPINGS", "FOLLOW-UP-OFFSET", "EGRESS-PORT-REF", "IMPLEMENTATION-PROPSS", "DEBOUNCE-ALGORITHM-REF", "AES-3-DATA-TYPE-H", "DDS-FIELD-QOS-PROPS", "RPT-SERVICE-POINT-PRE-REFS", "DATA-SEND-POINTS", "SERVICE-INTERFACE-FIELD-MAPPING", "KEEP-ALIVE-PROBES-MAX", "TROUBLE-CODE-J-1939-REF", "DIAGNOSTIC-SECURITY-LEVEL", "TIMEOUT-CS", "PROCESS-ARGUMENT", "ACTIVATION-POINTS", "INTERFACE-MAPPINGS", "CERTIFICATE-REF", "UDS-DTC-VALUE", "SDG-CLASS", "DYNAMIC-BS", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "COM-METHOD-GRANT-DESIGN", "USED-LANGUAGES", "IP-SEC-CONFIG", "OUTGOING-CALLBACKS", "DATA-PROTOTYPES", "CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF", "CYCLIC-TIMING", "PROCESSING-STYLE", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "BSW-INTERRUPT-EVENT", "REPORTABLE-SECURITY-EVENT-REF", "LIN-UNCONDITIONAL-FRAME-REF", "RAM-BLOCK", "DELAY-COMPENSATION-B", "RPT-EXECUTABLE-ENTITY-PROPERTIES", "NM-SCHEDULE-VARIANT", "PDELAY-LATENCY-THRESHOLD", "CONFIDENCE-INTERVALS", "CONCRETE-PATTERN-EVENT-TRIGGERING", "INTENDED-FILENAME", "TCP-RECEIVE-WINDOW-MAX", "DIFFERENTIATED-SERVICE-CODE-POINT", "SW-MAX-AXIS-POINTS", "DYNAMIC-PART-ALTERNATIVE", "FR-GLOBAL-TIME-DOMAIN-PROPS", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "DPD-DELAY", "EVENT-ELEMENTS", "BINARY-MANIFEST-ITEM-POINTER-VALUE", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "REQUIRED-ENTRYS", "LOG-TRACE-LOG-MODE", "LIN-SPORADIC-FRAME", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "DOIP-CHANNEL-COLLECTION-REF", "SOFTWARE-CLUSTER-ID", "EVENT-GROUP-CONTROL-TYPE", "LOG-TRACE-DEFAULT-LOG-THRESHOLD", "ABBR-NAME", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "DATA-IREF", "ARTIFACT-LOCATOR", "BUS-MIRROR-NETWORK-ID", "TIME-MASTER-PRIORITY", "PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "NETWORK-SEGMENT-ID", "COUPLED-CLUSTER-REF", "SEC-OC-CRYPTO-SERVICE-MAPPING", "TRANSFORMER-STATUS-FORWARDING", "ROUTINE-INFO", "SW-CLUSTER-MAPPINGS", "RPT-CONTAINER", "RAM-LOCATION-REF", "LATEST-TX", "BURST-SIZE", "IPV-4-MULTICAST-IP-ADDRESS", "CRYPTO-SERVICE-QUEUE-REF", "AFFECTED-REFS", "ERROR-DOMAIN-REF", "MATCHING-EGRESS-RULE-REFS", "RUNS-INSIDE-EXCLUSIVE-AREA-REFS", "FM-FEATURE-MAP-ELEMENT", "CREATED-DATAS", "OFFSET-CORRECTION-ADAPTION-INTERVAL", "ECUC-VALUES", "VEHICLE-DESCRIPTION-REF", "COLLECTION-PROPS", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "PROVIDER-SOFTWARE-CLUSTER-REF", "MIN-PWM-O", "MAX-PWM-S", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "BYTE-VALUES", "COUPLING-PORT-CREDIT-BASED-SHAPER", "SERVICE-INSTANCE-PORT-CONFIG", "J-1939-TP-CONFIG", "WAKEUP-RX-IDLE", "CONTROLLED-DATA-ELEMENT-REFS", "TYPE-REFERENCE", "INCLUDE-REF", "SENDER-REC-RECORD-TYPE-MAPPING", "FROM-REF", "NM-REPEAT-MESSAGE-TIME", "TLS-CRYPTO-SERVICE-MAPPING", "DLT-CONFIG", "TIME-PERIOD", "LOGICAL-EXPRESSION", "PERSISTENCY-FILE-ARRAY", "ENABLES-LOG-TRACE", "ECU-PARTITION", "PHM-SUPERVISED-ENTITY-INTERFACE", "ECUC-LINKER-SYMBOL-DEF", "EXECUTION-TIME-TYPE", "MAX-NUMBER-OF-OCCURRENCES", "MC-FUNCTION-REFS", "DOCUMENTATION-CONTENT", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "DEPLOYMENT-REF", "STANDARD-REVISIONS", "ENCAPSULATED-ENTRY-REF", "WAKEUP-FRAME-REFS", "REPLICA-PDUS-REF", "ON-BOARD-MONITOR-ID", "KEY-SLOT-CONTENT-ALLOWED-USAGES", "ECUC-DEFINITION-COLLECTION", "MIN", "DIAGNOSTIC-EVENTS", "DERIVED-OBJECT-REF", "SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING", "TCP-IP-NDP-DEFAULT-REACHABLE-TIME", "DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING", "ROOT-ARGUMENT-DATA-PROTOTYPE-REF", "BASE-IS-THIS-PACKAGE", "SIGNALS", "J-1939-DCM-DM-19-SUPPORT", "COMPONENT-IMPLEMENTATION-REF", "VALUE-TYPE-TREF", "RPT-PREPARATION-LEVEL", "SERVICE-KIND", "ENABLE-JOIN-ACCESS-CONTROL", "PID", "COUPLING-ELEMENT", "TASK", "BLUEPRINT-MAPPING-SET", "IPV-6-PROPS", "LOCAL-UNICAST-ADDRESSES", "COLOR-MODE", "PHM-ACTION", "SERVICE-INTERFACE-SUB-ELEMENT", "CONFLICTS-TO", "SOFTWARE-CLUSTER-UDS-DIAGNOSTIC-ADDRESS", "SOURCE-ID", "NON-VERBOSE-MODE", "USER-DEFINED-CLUSTER", "ARTIFACT-DESCRIPTORS", "PERSISTENCY-REDUNDANCY-M-OUT-OF-N", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "SEC-OC-JOB-MAPPING", "FLEXRAY-CLUSTER-CONDITIONAL", "SWC-SERVICE-DEPENDENCY", "RAM-BLOCK-STATUS-CONTROL", "PARTICIPATING-J-1939-CLUSTER-REFS", "DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL", "CONSTANT-SPECIFICATION-MAPPING-SET", "DATA-OFFSET", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "MIN-OK-STATE-INIT", "SEPARATION-TIME-RESPONSE", "SENDER-REC-ARRAY-ELEMENT-MAPPING", "TRIGGER-DIRECT-IMPLEMENTATIONS", "PID-REF", "NOTIFIER-REF", "ECUC-PARAM-CONF-CONTAINER-DEF", "MULTICAST-SECURE-COM-PROPS-REF", "RPT-READS", "FIREWALL-RULE-REFS", "DATA-READ-ACCESSS", "AGGREGATION-CONDITION", "FRAME-LENGTH", "PROVIDED-MEMORY-REF", "IDENT", "NO-CHECKPOINT-SUPERVISIONS", "COM-MGR-USER-NEEDS", "DOMAIN-IDS", "I-SIGNAL-TRIGGERING", "FRAME-REF", "IDSM-MODULE-INSTANTIATION", "FLAT-MAP-REF", "CPU-USAGE", "TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS", "COMPOSITION-SW-COMPONENT-TYPE-REF", "ACCEPTANCE-FIELD", "CAN-XL-BAUDRATE", "RPT-ENABLER-IMPL-TYPE", "DISCOVERY-TYPE", "MC-PARAMETER-INSTANCES", "TIME-BR", "DATA-TRANSFORMATION-REF-CONDITIONAL", "ENABLE-CONDITION-GROUP-REF", "LISTEN-NOISE", "METHOD-CALL-REF", "DURABILITY-SERVICE-HISTORY-KIND", "BIT-POSITION", "PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT", "TRANSPORT-LAYER-RULE", "PNC-MAPPING-REFS", "PWM-O", "DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE", "PROPAGATION-DELAY", "DO-IP-REQUEST-CONFIGURATION", "WWH-OBD-DTC-CLASS", "AUTHENTICATION-TIMEOUT", "INDEX", "SIZE", "LIN-CHECKSUM", "PROTOCOL-KIND", "SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA", "VERIFICATION-METHOD", "PACKAGING-PIN-NAME", "APPLICATION-RECORD-ELEMENT-REF", "MATCHING-RULE-REFS", "STATE-MANAGEMENT-COMPARE-FORMULA", "BSW-ENTRY-RELATIONSHIP-SET", "REPRESENTED-MODEL-ELEMENT-REF", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "AFFECTED-APPLICATION", "SENDING-NODE-REF", "ESP-CIPHER-SUITE-NAMES", "DISPLAY-IDENTIFIER", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "SOFTWARE-CLUSTER-COM-RESOURCE-REF", "TD-EVENT-BSW-MODULE", "FILE-TYPE-PATTERN", "SENDING-NODE-REFS", "SW-SERVICE-ARG", "CRYPTO-SERVICE-CERTIFICATE-REFS", "DIAGNOSTIC-ADDRESSS", "ECUC-FUNCTION-NAME-DEF-VARIANTS", "ALLOW-HALT-DUE-TO-CLOCK", "COMPU-SCALE", "NEXT-HIGHER-CERTIFICATE-REF", "MATCHING-EGRESS-RULE-REF", "AGGREGATION", "SSRC", "SIMULATED-EXECUTION-TIME", "TIME-LEAP-FUTURE-THRESHOLD", "CONTEXT-SWC-PROTOTYPE-REFS", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "NOTIFIER", "SECURITY-EVENT-REPORT-INTERFACE", "PORT-BLUEPRINT-REF", "MODULE-REFS", "REQUIRED-CLIENT-SERVER-ENTRY-REF", "REPETITION-PERIOD", "BSW-PARAMETER-POLICY", "JITTER", "I-SIGNAL-I-PDU-REF-CONDITIONAL", "PROVIDED-EVENT-GROUPS", "SOURCE-METHOD-REF", "IKE-OVER-TIME", "MAPPED-FUNCTION-REF", "CONFIDENCE-INTERVAL", "MSR-QUERY-TOPIC-1", "SOURCE-INSTANCE-IREFS", "MAX-SLOTS", "ROOT-AUTOSAR-DATA-PROTOTYPE-REF", "PORT-DEFINED-ARGUMENT-BLUEPRINTS", "FIRST-ELEMENT-INSTANCE-IREFS", "RELEASED-TRIGGER-POLICYS", "SW-FEATURE-DESC", "MAX-COMM-MODE", "SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF", "UCM-MASTER-FALLBACK-REF", "VLAN-REFS", "MACRO-INITIAL-OFFSET-B", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "MC-FUNCTION-REF", "SYNCHRONIZED-TIME-BASE-PROVIDER", "DLT-CONTEXT-REF", "SW-CLUSTER-REFS", "USED-IMPOSITION-TIME-DEFINITION-GROUP-REF", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "MAX-DATA-LENGTH", "ACCESS-COUNT-SET", "ADAPTIVE-AUTOSAR-APPLICATION", "REMOTE-ID", "SECURITY-LEVEL-REFS", "SUPPORTED-TIMER-GRANULARITY", "SYNC-CONFIRMATION-TIMEOUT", "MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "AUTOSAR-PARAMETER-IREF", "DLT-USER-NEEDS", "HEADER-TYPE", "NM-PNC-PARTICIPATION", "TIMING-ARGUMENTS", "APPLICATION-DATA-TYPE-REF", "TCP-IP-PROPS-REF", "OFFSET-SI-TO-UNIT", "HW-ELEMENT-CONNECTIONS", "IEEE-1722-TP-ACF-LIN", "NM-DATA-DISABLED", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "IT-REF", "PORT-BLUEPRINT-MAPPINGS", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "MODIFICATION", "MASTER-TO-SLAVE-EVENT-MAPPINGS", "SEND-POLICYS", "SECTION-NAME-PREFIX", "INSTANTIATION-DATA-DEF-PROPSS", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF", "SHORT-NAME-FRAGMENT", "AUTH-ROLE-REF", "TIMEOUT", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "OFNS-DATA-ID-LIST", "PHYS-CONSTRS", "KEY-ID", "TIME-VALUE", "VERBATIM", "SW-IMPL-MAPPINGS", "GLOBAL-IN-PACKAGE-REF", "ACTION-ITEMS", "IS-EVENT", "SOURCE-CAN-ID-CODE", "TD-EVENT-SLLET-PORT", "CUSTOM-DOCUMENTATION-REF", "SOCKET-ADDRESS-REF-CONDITIONAL", "REMOTE-CERTIFICATE-REF", "STATUS-ID", "WAKEUP-RX-LOW", "HW-ATTRIBUTE-DEF-REF", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "ARTIFACT-CHECKSUM-REF", "MC-GROUP-DATA-REF-SET-CONDITIONAL", "IMPOSITION-TIMES", "DATA-TYPE-REFS", "CRC-SEQUENCE-ID", "SWC-BSW-RUNNABLE-MAPPING", "TD-EVENT-MODE-DECLARATION", "EXECUTABLE-ENTITY-REFS", "TLS-CRYPTO-MAPPING-REF", "TRANSFORMING-SYSTEM-SIGNAL-REF", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "CONTAINED-PACKAGE-ELEMENT-REFS", "TARGET-HEALTH-CHANNEL-REF", "GROUPED-ALIAS-EVENT-REFS", "NV-DATA-PORT-ANNOTATION", "FREEZE-FRAME-CONTENT-REF", "EXECUTION-ERROR-REF", "REQUIRES-SYMBOLIC-NAME-VALUE", "PERIODIC-RATE-CATEGORY", "ECU-EXTRACT-VERSION", "SOMEIP-RULE", "DLT-MESSAGE-REFS", "SOURCE-SIGNAL-REF", "NETWORK-ENDPOINT-REF", "MODE-IREF", "EVENT-HANDLERS", "REMOTE-CERTIFICATE-REFS", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "ISSUED-BY", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "ASSIGN-NAD", "ARTIFACT-CHECKSUM-REFS", "DIAGNOSTIC-J-1939-NODE", "CONTEXT-ROOT-COMPONENT-REF", "FM-FEATURE", "BOR-COUNTER-L-1-TO-L-2", "ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF", "PUTS", "EXECUTION-TIME-CONSTRAINT", "STATE-REFERENCES", "SAVE-CONFIGURATION", "REQUIRED-DESIGN-ELEMENT-REFS", "DESTINATION-TYPE", "VARIATION-POINT-PROXY", "ERROR-SIGNALING-ENABLED", "LOG-AND-TRACE-INSTANTIATION", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "CRYPTO-JOB-REF", "SHALL-RUN-ON-REFS", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "REFERENCE-VALUE-SPECIFICATION", "LIN-SLAVE-ECU-REF", "VARIABLE-INSTANCE-IREF", "REQUESTS", "TARGET-FIELD-REF", "HW-PIN-GROUP-REFS", "TCP-PORT", "SCOPE-REFS", "ELEMENT-REFS", "BLUEPRINT-DERIVATION-GUIDE", "BSW-ENTRY-RELATIONSHIP", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "RUNNABLE-ENTITY-ARGUMENT", "NUMBER-OF-WORKERS", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "PERSISTENCY-FILE-ARRAY-REF", "EXTENDED-DATA-RECORDS", "TCP-IP-NDP-DELAY-FIRST-PROBE-TIME", "IP-ADDRESS-PREFIX-LENGTH", "FIELD-NOTIFIER-REF", "SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "LIN-ERROR-RESPONSE", "BINARY-MANIFEST-META-DATA-FIELD", "SOCKET-CONNECTION-IPDU-IDENTIFIER", "KEY-SIZE", "IPV-6-RULE", "TCP-RETRANSMISSION-TIMEOUT", "IP-ADDRESS-KEEP-BEHAVIOR", "MINISLOT-ACTION-POINT-OFFSET", "DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL", "TIME-BASE", "INVALID-VALUE", "TCP-TP", "PDU-TO-FRAME-MAPPING", "LIN-IDENTIFIER", "CAN-XL-PROPS", "WORST-CASE-STACK-USAGE", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF", "SOURCE-EEC-REFS", "FLEXRAY-NM-CLUSTER", "TIME-BASE-CONSUMER-REF", "EXTENDS-META-CLASS", "UCMS", "SW-CALPRM-IMPL", "FORMAT", "PARENT-SHORT-LABEL", "TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT", "IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "HW-CATEGORY", "LOCAL-ID", "SOURCE-I-PDU-REF", "DETERMINISTIC-SYNC-MASTER-REF", "ECUC-VALUE-COLLECTION", "MINIMUM-INTER-ARRIVAL-TIME", "FRAME-TRIGGERINGS", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF", "SEC-OC-COM-PROPS-FOR-MULTICAST-REFS", "SERVICE-ELEMENT-MAPPING-REFS", "MAPPING-DIRECTION", "DLT-SESSION-ID", "RESOURCE-ESTIMATIONS", "ENCRYPTION-REF", "NEXT-HEADER", "TCP-SYN-RECEIVED-TIMEOUT", "PRMS", "LIVENESS-KIND", "RPT-CONTAINERS", "PREFIX", "FIREWALL-STATE-MACHINES", "GLOBAL-TIME-TX-PERIOD", "SW-AXIS-CONTS", "MACHINE-TIMING", "DURABILITY-SERVICE-MAX-SAMPLES-PER-INSTANCE", "SAFETY-POLICY", "MANUFACTURER-VALIDATION-ORDER-REF", "INVERT-CONDITION", "E-2-E-PROFILE-COMPATIBILITY-PROPS-REF", "SOMEIP-TP-CONFIG", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "SERVER-SERVICE-MAJOR-VERSION", "SOURCE-LIN-PID-REF", "ACL-PERMISSION", "BSW-PER-INSTANCE-MEMORY-POLICYS", "TIMESTAMP-SUPPORT", "AR-PACKAGES", "SECOND-TO-FIRST-DATA-TRANSFORMATION-REF", "CONTEXT-DESCRIPTION", "SDG-TAILORING", "FILE-REFS", "DRIVER-NOTIFICATIONS", "COMPILERS", "MAX-DIFF", "USE-CRC-COMP-MECHANISM", "PACKAGE-REF", "P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "PARAMETER-REQUIRE-COM-SPEC", "CAN-BE-INVOKED-CONCURRENTLY", "INTENDED-TARGET-MACHINE-REFS", "HAS-GETTER", "RUNNABLE-ENTITY-GROUP", "DLT-CONTEXT", "LOCAL-SUPERVISION", "MAXIMUM-ALLOWED-SIZE", "COMPU-SCALES", "IMPLICIT-DATA-ACCESS-IREF", "OPERATION-CYCLE-AUTOSTART", "ACL-ROLE-REF", "GLOBAL-TIME-ETH-MASTER", "VENDOR-SPECIFIC-MODULE-DEF-REFS", "OPERATION-IREF", "COMPUTED", "CONTEXT-ELEMENT-REFS", "HEADER-LENGTH", "DATA-TRANSFORMATIONS", "GLOBAL-RESOURCE-ID", "DDS-FIELD-REQUEST-TOPIC-REF", "BITFIELD-TEXT-TABLE-MASK-FIRST", "CURRENT-EXP", "LOGICAL-EXPRESSIONS", "I-SIGNAL-PROPS", "AUTO-IP-PROPS", "CODE-LABEL", "ALLOW-PASSIVE-TO-ACTIVE", "SW-CLASS-IMPL", "TIME-SEG-2", "MINIMUM-DISTANCES", "DO-IP-LOGIC-ADDRESS-PROPS", "TTL-MIN", "RESPONSES", "METHOD-DEPLOYMENTS", "SYNC-JUMP-WIDTH", "DATE", "INSTANCES", "VIN-INVALIDITY-PATTERN", "SW-RECORD-LAYOUT-V-PROP", "STREAM-IDENTIFICATION-HANDLE-REF", "TARGET-REFS", "SW-SYSTEMCONST-VALUE", "INSTANTIATION-DATA-DEF-PROPS", "DIAGNOSTIC-ENABLE-CONDITION-REF", "IP-IAM-REMOTE-SUBJECT", "UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE", "CONNECTIONS", "MAX-NUMBER-OF-FILES", "COMMON-PROPERTIES", "NV-BLOCK-DATA-MAPPINGS", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "REF", "ESTIMATED-DURATION-OF-CAMPAIGN", "DIAGNOSTIC-ACCESS-PERMISSION", "TCP-LOCAL-PORT", "TRANSFER-REFS", "ELEMENT-ROLE", "COMPONENTS", "ROOT-SOFTWARE-COMPOSITIONS", "CPP-IMPLEMENTATION-DATA-TYPE", "SOURCE-NETWORK-MASK", "ATTRIBUTE-VALUES", "VLAN-PRIORITY", "TARGET-STATUS-REF", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "SW-GENERIC-AXIS-PARAM-TYPE", "NETWORK-INTERFACE-ID", "DESTINATION-URI-REFS", "NO-CHECKPOINT-SUPERVISION", "TIMEOUT-BS", "MEMORY-CONSUMPTION", "EOC-EXECUTABLE-ENTITY-REF", "DIAGNOSTIC-AUTHENTICATION-REF", "PDUR-TP-CHUNK-SIZE", "AUTOSAR-VARIABLE-INSTANCE", "SERVICE-INTERFACE-REFS", "SW-VARIABLE-IMPLS", "SW-VALUE-BLOCK-SIZE-MULTS", "VALUE-SPEC", "LIN-PHYSICAL-CHANNEL", "DOMAIN", "ENTERED-MODE-REF", "DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER", "TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED", "CONSISTENCY-NEEDSS", "SOFTWARE-CLUSTER-DEPENDENCY", "FUNCTION-GROUP-SET", "ASSOCIATED-COM-I-PDU-GROUP-REFS", "EXECUTABLE-REFS", "NETWORK-ID", "NUMBER-OF-STATIC-SLOTS", "IIDC-DATA-BLOCK-SIZE", "SW-COMPONENT-ASSIGNMENTS", "LIN-COMMUNICATION-CONNECTOR", "ACCESS-CONTROL", "GRANT-REFS", "COM-EVENT-GRANT-DESIGN", "BSW-TRIGGER-REF", "MSR-QUERY-P-1", "SWC-TIMING", "DDS-SECURE-GOVERNANCE", "TIME-SYNC-SERVER", "SOURCE-MAX-ADDRESS", "ENGINEERING-OBJECT", "R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "SIGNAL-BASED-FIELD-REF", "DATA-PROTOTYPE-MAPPING", "PORT-PROTOTYPE-BLUEPRINT-REF", "SOFTWARE-PACKAGE-STEP", "REPLAY-PROTECTION", "SW-RECORD-LAYOUT-GROUP-FROM", "TLS-CONNECTION-REF", "NAME-TOKEN-VALUE-VARIATION-POINT", "COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL", "REMOTE-IP-ADDRESS-REFS", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "IIDC-T-CODE", "MSG-ID-MASK", "DDS-OPERATION-REQUEST-TRIGGERING-REF", "SW-SYSTEMCONSTANT-VALUE-SET-REF", "CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS", "PNC-PDUR-GROUP-REFS", "TLV-SUB-ELEMENT-REF", "REF-CALPRM-SET", "R-PORT-PROTOTYPE-IREF", "SW-AXIS-GROUPED", "DIAGNOSTIC-DATA-CHANGE-TRIGGER", "APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION", "MODE-SWITCHED-ACK-EVENT", "ENVIRONMENT-VARIABLES", "POST-BUILD-VARIANT-CONDITIONS", "DEADLINE", "SYSTEM-SIGNAL-REF", "TRANSMITTER-REF", "ROW", "MAX-REQUEST-BYTES", "MAX-SAMPLES-PER-INSTANCE", "SW-NUMBER-OF-AXIS-POINTS", "TARGET-DATA-PROTOTYPE-GROUP-REF", "CHANNELS-PER-FRAME", "BSW-MODULE-DESCRIPTION-REF", "I-SIGNAL-PORT", "STATIC-REMOTE-UNICAST-ADDRESSES", "DDS-RULE", "PHM-ARBITRATION", "SUPPLIER-VALIDATION-ORDER-REF", "INCLUDED-DATA-TYPE-SETS", "REFERENCES", "COLLECTION-SEMANTICS", "DISPLAY-NAME", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "SUPPORTS-ASYNCHRONOUS-MODE-SWITCH", "CHECKSUM-VERIFICATION", "TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED", "P-PORT-PROTOTYPE", "PHYSICAL-DIMENSION-MAPPING-REF", "CONFIRMATION-THRESHOLD", "REQUEST-GROUP-REF", "CLIENT-SERVER-OPERATION-REF", "PRE-ACTIVATE-REF", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "OBD-MID", "ECUC-FUNCTION-NAME-DEF-CONDITIONAL", "GENERAL-INACTIVITY-TIME", "PRM-CHAR", "CONTAINED-PROCESS-REF", "DEFERRING-FID-REFS", "ACTIVATION-REASON-REPRESENTATION-REF", "SOMEIP-TP-CONNECTION", "SELECTIONS", "ECU-TIMING", "TLS-JOB-REQUIREMENT", "SENDER-RECEIVER-TO-SIGNAL-MAPPING", "SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "J-1939-CLUSTER-CONDITIONAL", "PARAMETER-DATA-PROTOTYPE", "TRIGGER-I-PDU-SEND-CONDITION", "REQUIRED-SERVICE-INSTANCE-ID", "UDP-NM-ECU", "INITIAL-STATUS", "REFINED-EVENT-IREF", "LOCAL-IP-SEC-RULE-REF", "FRESHNESS-PROPS-REF", "ECU-INSTANCE", "ACTION-LISTS", "SW-CLUSTERS", "REQUEST-SEED-ID", "DEADLINE-PERIOD", "HW-CATEGORY-REFS", "TD-EVENT-SERVICE-INSTANCE-METHOD", "BINARY-MANIFEST-ITEM", "MACRO-INITIAL-OFFSET-A", "START-IREF", "USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF", "SUPERVISION-MODE", "PROVIDED-DDS-EVENT-QOS-PROPS", "SUPERVISION-MODES", "ASSIGNED-DATAS", "STATE-REF", "SYSTEM", "LOCAL-TCP-PORT-REF", "ROLE-BASED-PORT-ASSIGNMENT", "NM-NODE-ID", "FUNCTION-GROUP-PHM-STATE-REFERENCE", "ACCURACY", "STREAM-FILTERS", "MIXED-BUS-TYPE-COLLECTION", "RESOURCE-DEFINITIONS", "DIAGNOSTIC-IO-CONTROL", "TYPE-MAPPING", "TEAR-DOWN-ACTION-REF", "ACCEPTED-REMOTE-CERTIFICATE-REFS", "SW-CALIBRATION-ACCESS", "CONDITION-ACCESS", "IAM-MODULE-INSTANTIATION", "DTCS", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "API-SERVICE-PREFIX", "CAN-CONTROLLER-XL-REQUIREMENTS", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "AGING-REQUIRES-TESTED-CYCLE", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "TABLE-CAPTION", "ECU-EXTRACT-REF", "ROOT-SW-COMPOSITION-PROTOTYPE", "TCP-DELAYED-ACK-TIMEOUT", "REQUEST-RESULT", "DIAGNOSTIC-PARAMETER-ELEMENT", "STATE-DEP-FIREWALL-REFS", "RULES", "NV-RAM-BLOCK-ELEMENT", "TCP-OPTION-FILTER-SET", "CONTEXT-ID", "STEP-SIZE", "CORES", "DIAGNOSTIC-DATA-IDENTIFIER", "INNER-GROUP-IREFS", "COUPLING-FLAG", "METHOD-IREF", "DIAGNOSTIC-EVENT", "CONTROLLED-HW-ELEMENT-REF", "COMMANDS", "TCP-IP-ICMP-PROPS-REFS", "BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING", "COM-CONFIGURATION-TX-TIME-BASE", "TIMING-CLOCKS", "VERIFY-UPDATE", "DIAG-ARG-INTEGRITY", "SUPPORT-INFO-BIT", "BYPASS-VLAN", "EXECUTION-CONTEXT", "SECURITY-EVENT-DEFINITION-REF", "IS-MANDATORY", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "ECUC-QUERY-REF", "CAN-ENTERS", "REMOTE-IP-ADDRESS-REF", "VENDOR-SIGNATURE-REF", "FLEXRAY-CLUSTER-VARIANTS", "PARTICIPATING-J-1939-CLUSTER-REF", "NM-INTERACTS-WITH-SM-MAPPING", "TD-EVENT-TT-CAN-CYCLE-START", "MAX-BASE-TYPE-SIZE", "PREFIX-REF", "SOMEIP-SERVICE-DISCOVERY", "SW-COMPONENT-IREF", "SAVE-CONFIGURATION-ENTRY", "PRECONFIGURED-CONFIGURATION-REF", "CONTAINED-I-PDU-TRIGGERING-PROPSS", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF", "MULTICAST-SEGMENTATION", "CRC-OFFSET", "TRANSFER-EXIT-CLASS-REF", "SOCKET-ADDRESS", "SPN", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF", "DEF-CALPRM-SET", "LIN-SLAVE-CONFIG", "PORT-INTERFACE-MAPPING-SET", "TCP-MSL", "ANNOTATION-TEXT", "P-2-SERVER-MAX", "TIME-SUB-TLV", "LOWER-CAN-ID", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "SYNC-LOSS-THRESHOLD", "CONSTR-LEVEL", "PORT-ARG-VALUES", "POSTS", "PERSISTENCY-REDUNDANCY-HASH", "HEALING-CYCLE-COUNTER-THRESHOLD", "ADDITIONAL-NATIVE-TYPE-QUALIFIER", "WAKE-UP-CHANNEL", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "DEFAULT-MODE-REF", "TIMING-CLOCK-SYNC-ACCURACYS", "MAX-CYCLES", "LIFESPAN-DURATION", "TARGET-MODULE-ID", "BUNDLED-CONNECTIONS", "CERTIFICATE-TO-KEY-SLOT-MAPPINGS", "TIME-EXP", "MAX-SYNC-JUMP-WIDTH", "SW-ALIGNMENT", "CRYPTO-KEY-DESCRIPTION", "SECURE-COMMUNICATION-PROPS-SET", "IS-SYNCHRONOUS", "TLV-DATA-IDS", "REQUIRED-DDS-SERVICE-INSTANCE", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "NUMBER-OF-BITS", "MEMORY-ENTRY-STORAGE-TRIGGER", "TARGET-MODE-GROUP-REF", "DIAGNOSTIC-CONTRIBUTION-SET", "GENERIC-ETHERNET-FRAME", "ASSERTIONS", "RUNTIME-IP-ADDRESS-CONFIGURATION", "MAX-PWM-O", "RVF-PIXEL-DEPTH", "AGGREGATION-TAILORING", "TCP-IP-NDP-RTR-SOLICITATION-INTERVAL", "ALLOWED-I-PV-6-EXT-HEADERS", "AUTH-DATA-FRESHNESS-LENGTH", "OPTIONS", "IDSM-TRAFFIC-LIMITATION-REF", "DLT-LOG-SINK-REFS", "DIAGNOSTIC-UPLOAD-INTERFACE", "USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT", "CONTROL-CONSUMED-EVENT-GROUP-REFS", "OPT-IMPL-RECORD-ELEMENTS", "ALLOCATOR-REF", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "DYNAMIC-ARRAY-SIZE-PROFILE", "NETWORK-TIME-PROVIDER-REF", "STATUS-SUB-TLV", "TEXT-TABLE-VALUE-PAIR", "PNC-FILTER-ARRAY-MASK", "UDP-TP", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "TIME-CS", "ENVIRONMENT-DATA-CAPTURE", "HW-PIN-REF", "CHECKPOINT-INITIAL-REF", "ETHERNET-PRIORITY-REGENERATIONS", "APPLICATION-ERROR", "FUNCTIONAL-REQUEST-REFS", "ALGORITHM-SECONDARY-FAMILY", "SUPPORTED-CONFIG-VARIANTS", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS", "ALIGNMENT", "TIMING-MODES", "TCP-IP-NDP-DEFAULT-RETRANS-TIMER", "UPDATE-SESSION-REJECTED", "TRAFFIC-CLASS", "COMMUNICATION-RESOURCE-REF", "SECOND-KEY-SLOT-ID", "CALL-POINTS", "AGING-REF", "VARIABLE-ACCESSED-FOR-DEBUG-REFS", "DIAGNOSTIC-CLEAR-CONDITION", "NODE-PORTS", "CONTENT-TYPE", "GET", "SEGMENT-REF", "FM-FEATURE-RELATION", "DIAGNOSTIC-EVENT-REF", "REQUEST-EMISSION-RELATED-DTC-CLASS-REF", "RULE-ARGUMENTS", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS", "BYTE", "DDS-QOS-PROFILES", "MODULE-INSTANTIATION-REF", "SUPPORT-BUFFER-LOCKING", "DISABLE-END-TO-END-CHECK", "SHARED-TIMER", "OUTER-PORT-REF", "RVF-PIXEL-FORMAT", "RX-ACCEPT-CONTAINED-I-PDU", "DYNAMIC-PART", "NM-CLUSTERS", "CONNECTION-TYPE", "DISPLAY-FORMAT", "VLAN-MEMBERSHIPS", "INDUSTRY-GROUP", "UPDATE-KIND", "TCP-REMOTE-PORT", "TIME-LEAP-HEALING-COUNTER", "CYCLE", "INDEXED-ARRAY-ELEMENT", "BSW-MODULE-DESCRIPTION-REF-CONDITIONAL", "FALSE-ACTION-LIST-REF", "MODE-SENDER-POLICYS", "HW-PIN-CONNECTOR", "CLIENT-INTENT", "ISSUED-TRIGGERS", "SLEEP-TIME", "MAC-MULTICAST-GROUPS", "COUNTER-DECREMENT-STEP-SIZE", "DLT-MESSAGE-REF-CONDITIONAL", "BSW-MODULE-ENTRY-REF-CONDITIONAL", "VERIFY-REF", "ROOT-CONTEXT-REF", "MODE-GROUP-IREF", "UDP-LOCAL-PORT", "STATUS-IREF", "REMOTE-UNICAST-CONFIG-REFS", "APPLIED-STANDARDS", "ARTIFACT-DESCRIPTOR", "TYPE-DEFINITION", "SECURITY-EVENT-CONTEXT-DATA", "INITIAL-FIND-BEHAVIOR", "FIELD-REPLY-TOPIC-NAME", "AUTOSAR", "COUPLING-PORT-CONNECTION", "USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS", "SOURCE-EEC-REF", "TIME-MARK", "ARGUMENT-REF", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "TLV-DATA-ID-DEFINITION-REFS", "COM-MANAGEMENT-GROUP-REF", "IDSM-TIMESTAMP-PROVIDER-INTERFACE", "MAC-ADDRESS", "SERVICE-INTERFACE-MAPPING-SET", "TABLE-ENTRYS", "NM-INSTANCE-ID", "HOP-LIMIT", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS", "MAX-DESTINATION-PORT-NUMBER", "INSTANCE-IDS", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "DOCUMENT-ELEMENT-SCOPE", "ASSOCIATED-PDUR-I-PDU-GROUP-REFS", "FILE-REF", "NUMERICAL-VALUE-SPECIFICATION", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "SPECIFICATION-SCOPE", "DIAGNOSTIC-ROUTINE-NEEDS", "I-PV-6-EXT-HEADER-FILTER-LIST", "DATA-ID-MODE", "USER-DEFINED-CLUSTER-VARIANTS", "IDENTITY-REF", "EXPORTABILITY", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS", "CP-SOFTWARE-CLUSTER-REF-CONDITIONAL", "WRITE-ACCESS", "IUMPR-GROUP", "ETHERNET-PRIORITY-REGENERATION", "SUPPORT-INFO", "EXECUTION-CONTEXTS", "J-1939-CLUSTER", "DATA-RECEIVED-EVENT", "DESTINATION-BASE-ID", "CONNECTOR-REFS", "CRYPTO-SERVICE-NEEDS", "SOMEIP-PROVIDED-EVENT-GROUP", "IDENTIFIER", "COMPONENT-QUALIFIER", "TRANSMISSION-MODE-CONDITIONS", "GROUP-NUMBER", "SUPPORT-INFO-BYTE", "DETERMINISTIC-CLIENT-REF", "APPLICATION-ASSOC-MAP-DATA-TYPE", "COMPU-PHYS-TO-INTERNAL", "PROPERTYS", "GLOBAL-SUPERVISIONS", "DATA-LENGTH", "EGRESS-PORT-REFS", "DIAGNOSTIC-PROTOCOL", "EVENT-DEFAULT-VALUE", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "PHYSICAL-DIMENSION-MAPPING-SET", "RUNNABLE-ENTITY-GROUP-IREF", "UPPER-HEADER-BITS-TO-SHIFT", "PROVIDED-ENTRYS", "TRANSPORT-PLUGIN", "FIBEX-ELEMENT-REF-CONDITIONAL", "SHALL-NOT-RUN-ON-REF", "DEBUG-INFOS", "PATTERN-PERIOD", "SYMBOL-WINDOW", "HOST-SYSTEM-REF", "OBD-DTC-VALUE-3-BYTE", "DELTA-PACKAGE-APPLICABLE-VERSION", "RELIABILITY-MAX-BLOCKING-TIME", "SENDER-REC-RECORD-ELEMENT-MAPPING", "METHOD-MAPPINGS", "FAILURE-MONITORING-REF", "ALIAS-NAMES", "PHYSICAL-CHANNEL-REF-CONDITIONAL", "SECOND-ELEMENT-REFS", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "DIAGNOSTIC-TEST-RESULT", "LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS", "IDSM-TIMESTAMP-PROVIDER-MAPPING", "SERVICE-ITEMS", "TRANSFORMATION-PROPS-REF", "ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS", "FREEZE-CURRENT-STATE-SUPPORTED", "ACL-OBJECT-SET", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS", "ECU-ID", "SW-COMPONENT-DOCUMENTATION", "I-SIGNAL-GROUP-REF", "SW-RECORD-LAYOUT-COMPONENT", "INIT-VALUE", "ARRAY-ELEMENT-REF", "TRANSFORMER-REF", "COUNTER-JUMP-DOWN-VALUE", "KEY-SLOT-REF", "SW-COMPONENT-MAPPING-CONSTRAINTS", "ACCURACY-INT", "REQUEST-UPLOAD-CLASS-REF", "DESTINATION-PORTS", "HTTP-ACCEPT-ENCODING", "DLT-MESSAGE-REF", "UDP-TP-PORT", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "SW-GENERIC-AXIS-DESC", "SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE", "DESTINATION-MAX-ADDRESS", "VALUE", "HW-PORT-MAPPINGS", "MODES", "PDU-COLLECTION-MAX-BUFFER-SIZE", "TCP-IP-DHCP-V-6-SOL-DELAY-MAX", "DDS-METHOD-DEPLOYMENT", "REQUIRED-AR-ELEMENT-REFS", "CIPHER-SUITE", "DIRECT-SUCCESSOR-REFS", "SUCCESSOR-REF", "COMMENT", "TIME-LEAP-PAST-THRESHOLD", "STATE-REQUEST-PORT-IREF", "DIAGNOSTIC-DEPLOYMENT-PROPS-REF", "SOVD-SERVER-INSTANTIATION", "DATA-TRANSFER-CLASS-REF", "MSR-QUERY-PROPS", "CAN-CONTROLLER-FD-REQUIREMENTS", "I-PDUS", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "FIREWALL-RULE-PROPS", "DEFERRING-FID-REF", "RX-NM-PDU-REF", "EXTERNAL-CLOCK-SYNCHRONISATION", "TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE", "TARGET-P-PORT-PROTOTYPE-REF", "ICV-SECURED", "WAKEUP-LOCAL-DETECTION-TIME", "SDG-FOREIGN-REFERENCE", "SW-CALIBRATION-NOTESS", "TIMING-CLOCK-SYNC-ACCURACY", "SERVICE-POINT-SYMBOL-POST", "MODE-ACCESS-POINTS", "NUMERICAL-VALUE", "SECTION-INITIALIZATION-POLICY", "TCP-ALIVE-CHECK-RESPONSE-TIMEOUT", "COUPLING-PORT-SPEED", "KEYWORD", "TCP-IP-NDP-SLAAC-DELAY-ENABLED", "FLEXRAY-CLUSTER", "COLLECTION", "BUILD-ACTION-MANIFESTS", "SW-ADDRMETHOD-REF", "ENUMS", "IP-ADDRESS", "CONTAINED-AR-ELEMENT-REFS", "EVALUATED-ELEMENT-REFS", "CURRENT-VALUE-REF", "SECURITY-EVENT-DEFINITION-REF-CONDITIONAL", "DATA-TYPE-MAPPING-SET-REF", "TIME-TRIGGERED-CAN-LEVEL", "SLAVES", "VALUE-TYPE-REF", "CONFLICTS", "ANNOTATIONS", "ARRAY-SIZE-HANDLING", "AR-PARAMETER", "TTCAN-CLUSTER-CONDITIONAL", "UDP-NM-CLUSTER-COUPLING", "CONSTANT-SPECIFICATION-MAPPING", "REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF", "IDENTIFIABLE-REF", "MAX-SOURCE-ELEMENT", "APP-ID", "TRANSFORMATION-PROPS-MAPPING-SET-REF", "SESSION-HANDLING", "INVERSE-PROTOCOL-VERSION", "LOGICAL-ADDRESS", "SETTER-CALL-REF", "SYSC-STRING-REF", "CAN-PHYSICAL-CHANNEL", "NM-REPEAT-MESSAGE-BIT-ACTIVE", "VARIABLE-DATA-PROTOTYPE", "PDU-COUNTER-SIZE", "GLOBAL-TIME-PDU-TRIGGERING-REF", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS", "SW-COMPONENT-PROTOTYPE-REF", "IIDC-SOURCE-PACKET-HEADER", "INTERPOLATION-ROUTINE-MAPPINGS", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS", "ETHERNET-RAW-DATA-STREAM-MAPPING", "NM-DETECTION-LOCK", "COLLECTED-INSTANCE-IREF", "DESTINATION-IPV-6-ADDRESS", "META-DATA-ITEM-TYPE", "TRAFFIC-LIMITATION-FILTERS", "IS-EXTERNAL", "TCP-IP-AUTO-IP-INIT-TIMEOUT", "PAYLOAD-BYTE-PATTERN-RULES", "PREDEFINED-TEXT", "RUNNABLES", "ALLOWED-KEYSLOT-USAGE", "RESPONSE-ON-EVENT-SCHEDULER-RATE", "SUBFUNCTIONS", "ACCESS-COUNTS", "TCP-IP-PROPS-REFS", "PORT-INTERFACE-BLUEPRINT-MAPPING", "SD-CLIENT-EVENT-TIMING-CONFIG", "ETHERNET-RAW-DATA-STREAM-MAPPING-REF", "CHECKPOINT-TRANSITION", "TCP-IP-IP-REASS-TIMEOUT", "DELETES", "CRYPTO-SERVICE-QUEUE", "ACL-OBJECT-REF", "ETHERNET-CLUSTER", "PLATFORM-PHM-ACTION-ITEM", "LITERAL-PREFIX", "TARGET-R-PORT-REF", "BSW-INTERNAL-TRIGGERING-POINT", "OFS-SUB-TLV", "AFFECTED-FUNCTIONAL-CLUSTER", "CONTEXT-DATA-PROTOTYPE-REFS", "SW-RECORD-LAYOUT-GROUP-INDEX", "PDU-REPLICATIONS", "PROCESSING-KIND", "NODE-REF", "PERSISTENCY-REDUNDANCY-CRC", "REG-DOES-NOT-REQUIRE-STABILITYS", "EXECUTION-DEPENDENCYS", "MAC-MULTICAST-CONFIGURATION", "FUNCTION-ID", "DO-IP-ROUTING-ACTIVATIONS", "TD-HEADER-ID-FILTERS", "LC-STATE-REF", "EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD", "LONG-NAME", "CRYPTO-CERTIFICATE-INTERFACE", "NM-CYCLETIME-MAIN-FUNCTION", "MAX-NUMBER-FREEZE-FRAME-RECORDS", "SW-AXIS-TYPE-REF", "TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED", "RECEIVER-CAPABILITY", "MIN-MINOR-VERSION", "DO-IP-LOGIC-ADDRESS", "HAS-NOTIFIER", "FRAME-PORT-REF", "TIMING-EVENT-REF", "BLUEPRINT-MAPPING-GUIDE", "MULTICAST-CREDENTIALS", "INITIAL-REF-OFFSET", "CLIENT-SERVER-RECORD-TYPE-MAPPING", "MC-GROUP", "COLLISION-RESOLVING-SCHEDULE-REF", "PERSISTENCY-FILE-STORAGE-INTERFACE", "VG", "TA-TYPE", "UDP-RULE", "FIGURE", "ALL-PHYSICAL-CHANNELS", "AVERAGE-MEMORY-CONSUMPTION", "LENGTH-VERIFICATION", "OBD-RATIO-DENOMINATOR-NEEDS", "ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT", "CRYPTO-ALGO-CONFIG", "SEPARATE-SIGNAL-PATH", "TRIGGER-MAPPINGS", "WRITER-ENTITY-ID", "MAX-BYTES-IN-INTERVAL", "DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT", "EVENT-HANDLER", "CHECKPOINT-IREF", "SCOPE-EVENT-REFS", "INIT-RESOURCE", "SUB-ELEMENT-MAPPING", "CLEAR-CONDITION-GROUP-REF", "MAXIMUM-SUPPORTED-UCM-MASTER-VERSION", "ACL-CONTEXT", "MIN-NUMBER-OF-ELEMENTS", "SIZE-OF-STRING-LENGTH-FIELD", "LOGICAL-EXPRESSION-ARGUMENT-REFS", "EVENT-REFS", "RPT-PROFILES", "I-SIGNAL-I-PDUS", "PRE-ACTIVATE-IREFS", "TRANSFORMER-CLASS", "SW-BIT-REPRESENTATION", "NM-REPETITION-CYCLE", "MODE-DRIVEN-TRUE-CONDITIONS", "USED-SECONDARY-FID-REFS", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE", "FEATURE-IREF", "NM-DATA-CYCLE", "TP-SDU-REFS", "RESOURCE-REF", "DPD-ACTION", "TRANSPORT-PROTOCOL", "DID-SIZE", "DIAGNOSTIC-TROUBLE-CODE-UDS", "CHECKPOINT-FINAL-REFS", "CLIENT-SERVER-OPERATION-IREF", "IEEE-1722-TP-CRF-CONNECTION", "COMPOSITION-DATA-PROTOTYPE-REF", "DIAGNOSTIC-SESSION-CONTROL", "HARDWARE-TEST-NEEDS", "RECOVERY-VIA-APPLICATION-ACTION", "DIAG-FAULT-DETECTION-COUNTER-PORT-REF", "SUPERVISION-ENTITY", "SYNC-PERIOD", "PNC-GROUP-REF", "MINIMUM-SEPARATION-TIME", "COLLECTED-INSTANCE-IREFS", "BYPASS-ETHER-TYPE", "VTF", "V-2-X-FAC-USER-NEEDS", "DEFAULT-PERIOD-END", "VT", "MAXIMUM-DISTANCES", "RULE-INIT-STATE", "MAC-SEC-PROPSS", "AUDIENCES", "DEPENDS-ON-REF", "MODE-DECLARATION-MAPPING", "TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION", "TRIGGER-I-PDU-SEND-CONDITIONS", "ACL-OBJECT-CLASSS", "REQUEST-TOPIC-NAME", "STD-REFS", "CALIBRATION-PARAMETER-VALUE-SET-REF", "LOGICAL-SUPERVISION", "TOLERANCE", "SEPARATED-COMPONENT-IREFS", "FIRST-PHYSICAL-DIMENSION-REF", "RESOURCE-IDENTIFIER-TYPE", "DDS-EVENT-QOS-PROFILE-REF", "NM-NODES", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF", "UPPER-BOUND", "LOG-AND-TRACE-INTERFACE", "DEBOUNCE-COUNTER-STORAGE", "MULTIPLICITY-RESTRICTION", "MAX-ERROR-STATE-VALID", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "IDSM-INSTANCE", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "RAW-DATA-STREAM-GRANT-DESIGN", "POST-BUILD-VARIANT-SUPPORT", "SERVICE-SW-COMPONENT-TYPE", "CLIENT-SERVER-APPLICATION-ERROR-MAPPING", "TEXT-VALUE-SPECIFICATION", "CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF", "METHOD-REQUEST-TOPIC-NAME", "IMPLEMENTATION-CONFIG-CLASSES", "CLASSIFICATIONS", "MAX-TRANSIT-TIME", "CONTROL-OPTION-RECORD-PRESENT", "LIN-EVENT-TRIGGERED-FRAME", "L-10", "EXCLUSIVE-AREA-NESTING-ORDER", "ALLOWED-SERVICE-CONSUMERS", "COM-CONTROL-CLASS-REF", "PARTS", "ROUTINE-SUBFUNCTION-REF", "NOT-AVAILABLE-VALUE-SPECIFICATION", "MAX-SERVICE-POINT-ID", "RESOURCE-CONSUMPTION", "SUB-CONTAINERS", "CLIENT-ID-DEFINITION", "PARTIAL-NETWORK-REFS", "STATE-DEPENDENT-FIREWALL-REF", "PHYSICAL-CHANNELS", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS", "BSW-SCHEDULER-NAME-PREFIX", "CAN-BIT-RATE-SWITCH", "MODE-IN-BSW-INSTANCE-REF", "MAPPED-SWC-SERVICE-DEPENDENCY-IREF", "DLT-LOG-CHANNEL", "CALLED-ENTRYS", "ENTRY", "EXPECTED-CALLBACKS", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "CONSTRAINT-REF", "MINIMUM-INTENDED-BINDING-TIME", "VALUE-DATA-TYPE-REF", "TOOL", "LEAF-ELEMENT-IREF", "SUB-TLV-CONFIG", "FM-FEATURE-MODEL", "VERIFY-IREFS", "UPPER-MULTIPLICITY-INFINITE", "PURE-LOCAL-TIME-BASE", "VALUE-AXIS-DATA-TYPE-REF", "NEIGHBOR-CACHE-SIZE", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "PRM-UNIT", "MSR-QUERY-RESULT-CHAPTER", "CP-SOFTWARE-CLUSTER", "MODE-ACCESS-POINT", "APPLICATION-CONTEXT-REF", "META-DATA-ITEMS", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "SHORT-TERM-ADJUSTMENT-SUPPORTED", "NM-CHANNEL-ACTIVE", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "MACHINE-REF", "PERSISTENCY-DEPLOYMENT-URI", "REFERRABLE-REF", "BSW-ENTRY-KIND", "RULE-BASED-VALUES", "DIAGNOSTIC-CONNECTION-REF-CONDITIONAL", "TEST-ID", "ICMP-V-6-PROPS", "ETHERNET-CLUSTER-VARIANTS", "NRC-VALUE", "SWC-INTERNAL-BEHAVIOR", "PERMISSIBLE-SIGNAL-PATH", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "SW-REFRESH-TIMING", "TEST-ID-REF", "PHM-CONTRIBUTION-REFS", "REQUESTABLE", "NOTIFICATION-STATE", "CLIENT-ID-DEFINITION-SET-REF", "I-PDU-TIMING", "HEALTH-CHANNEL-SUPERVISION", "UCM-TO-TIME-BASE-RESOURCE-MAPPING", "TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS", "MODULE-INSTANTIATION-REFS", "PERIODIC-RESPONSE-UUDT-REFS", "XDOC", "HW-PIN-GROUP-REF", "INIT-EVENT", "RPT-HOOK", "STRING-ENCODING", "DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF", "I-PDU-IDENTIFIERS", "DIAGNOSTIC-SOVD-UPDATE-INTERFACE", "TCP-PROPS", "REQUESTER-REF", "SEND-RESP-PEND-ON-TRANS-TO-BOOT", "VFC-IREF", "PRIMITIVE-TYPE-MAPPINGS", "DERIVED-PORT-PROTOTYPE-REF", "CREATE-ECU-WAKEUP-SOURCE", "ENABLE-UPDATE", "MODE-PORT-ANNOTATIONS", "DYNAMICALLY-ASSIGNED", "SW-HOST-VARIABLE", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "MODE-DECLARATION-MAPPINGS", "DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF", "EID-RETRIEVAL", "TARGET-RTE-EVENT-REF", "EVENT-OCCURRENCE-KIND", "TIME-BASE-JITTER", "LIN-MASTER-CONDITIONAL", "MAPPING-REF", "COMMUNICATION-CONTROLLER-REF", "CONTEXT-OPERATION-REF", "SW-TEST-DESC", "DATA-CONSTR-RULES", "CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "CONSTANT-REF", "FIRST-TRIGGER-REF", "ACL-ROLE-REFS", "SW-DATA-DEPENDENCY-FORMULA", "TARGET-PHM-CHECKPOINT-REF", "ECUC-FOREIGN-REFERENCE-DEF", "FLEXRAY-COMMUNICATION-CONTROLLER", "SECURITY-ACCESS-CLASS-REF", "POSSIBLE-AP-ERROR-REF", "SW-SERVICE-IMPL-POLICY", "NETWORK-MANAGEMENT-PORT-INTERFACE", "TTCAN-COMMUNICATION-CONTROLLER-VARIANTS", "RECOVERY-NOTIFICATION-TIMEOUT", "COMMUNICATION-CLUSTER-REFS", "DETERMINISTIC-SYNC-M-OUT-OF-N", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "EVENT-RECEPTION-DEFAULT-VALUE", "PLCA-LOCAL-NODE-COUNT", "COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE", "READONLY", "SW-CALPRM-AXIS-SET", "EXECUTABLE-REF", "CAN-FRAME-TRIGGERING", "DTC-FORMAT-TYPE", "DLT-LOG-CHANNEL-REF", "CUSTOM-SERVICE-ID", "SWITCH-STREAM-IDENTIFICATION", "TIME-SYNC-SERVER-IDENTIFIER", "INITIALIZED-PARAMETER-REF", "TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF", "STORING", "SW-VARIABLE-REFS", "CONTAINED-I-PDU-PROPS", "COM-OFFER-SERVICE-GRANT", "TARGET-MODE-DECLARATION-REF", "USES-END-TO-END-PROTECTION", "CONTEXT-COMPOSITION-REF", "REFINED-MODULE-DEF-REF", "CONTAINED-PROCESS-REFS", "RPT-PROFILE", "NETWORK-ENDPOINT-REF-CONDITIONAL", "DATA-FORMAT-TAILORING", "DIAGNOSTIC-DATA-PORT-MAPPING", "COUPLING-PORT-STARTUP-ACTIVE-TIME", "SOMEIP-FIELD-DEPLOYMENT", "RVF-EVENT-DEFAULT", "DIAGNOSTIC-COMMON-PROPS-CONDITIONAL", "PERSISTENCY-KEY-VALUE-PAIR", "PREVIOUS-CONTRACT-VERSION", "EVENT-GROUP-REF", "SEC-OC-COM-PROPS-FOR-MULTICAST-REF", "FLEXRAY-TP-CONNECTION", "SAFE-TRANSLATION", "DDS-SECURE-COM-PROPS", "ORDERED-MASTER-LIST", "SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "ACL-OBJECT-REFS", "POSITION-IN-TABLE", "COMPARE-VALUE", "MAC-MULTICAST-GROUP", "SEQUENCE-COUNTER-JUMP-WIDTH", "E-2-E-EVENT-PROTECTION-PROPSS", "CRYPTO-CERTIFICATE-REF", "RPT-AR-HOOK-IREF", "USES-ADDRESS-ARBITRATION", "DO-IP-ROUTING-ACTIVATION", "PROVIDED-MODE-GROUPS", "PER-STATE-TIMEOUTS", "FILE-ELEMENTS", "UNLIMITED-INTEGER-VALUE-VARIATION-POINT", "IMPLEMENTS-SOMEIP-STRING-HANDLING", "PAYLOAD-BYTE-PATTERN-RULE", "TIME-SEG-1", "LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "PROVIDED-SERVICE-INSTANCES", "MULTICAST-UDP-PORT", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "SW-COMPONENT-PROTOTYPE-ASSIGNMENT", "TLV-RECORD-ELEMENT-REF", "OBD-INFO-SERVICE-NEEDS", "NTU", "DATA-ELEMENT-REFS", "ECUC-DEFINITION-REF", "TARGET-VARIABLE-ACCESS-REF", "OFS-DATA-ID-LIST", "SDG-CAPTION-REF", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "SW-RECORD-LAYOUT-V-INDEX", "LAST-EGRESS-SCHEDULER-REF", "ECUC-INTEGER-PARAM-DEF", "CLIENT-ID-RANGE", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF", "S-3-SERVER-TIMEOUT", "END-TO-END-CALL-RESPONSE-TIMEOUT", "COLLECTION-REF", "META-DATA-ITEM-SETS", "PHM-RECOVERY-ACTION-INTERFACE", "MAPPED-BSW-SERVICE-DEPENDENCY-REF", "EVENT-NEEDS-REF", "REMOTE-SERVER-CONFIG", "MULTIPLEXED-I-PDU", "SERVICE-REQUEST-CALLBACK-TYPE", "ARTIFACT-LOCATORS", "SIGNATURE-SUPPORT-AP", "ACK-REQUEST", "EVALUATED-VARIANT-REF", "SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF", "SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "BLUEPRINT-POLICY-LIST", "SELECTOR-FIELD-START-POSITION", "STATIC-SOCKET-CONNECTION-REF", "SW-RECORD-LAYOUT-GROUP", "SOMEIP-EVENT-DEPLOYMENT", "ECU-MAPPING", "L-GRAPHIC", "SESSION-CONTROL-CLASS-REF", "ELEMENT-GROUPS", "REPORTED-SECURITY-EVENT-IREF", "PARAMETER-SIZE", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "NM-READY-SLEEP-TIME", "SERVICE-INTERFACE-DEPLOYMENT-REF", "CLEAR-EVENT-ALLOWED-BEHAVIOR", "PDU-REPLICATION-VOTING", "IS-DELTA-PACKAGE", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "PORT-PROTOTYPE-BLUEPRINT", "SAK-REKEY-TIME-SPAN", "DATA-ELEMENT-IREF", "CPP-TEMPLATE-ARGUMENT", "ELEMENT-IN-IMPL-DATATYPE", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "CLEAR-CONDITION-REF", "IDS-DESIGN", "GLOBAL-TIME-FR-SLAVE", "TARGET-DATA-PROTOTYPE-IN-CS-REF", "CAN-XL-PROPS-REFS", "E", "AUTOSAR-VARIABLE", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS", "I-SIGNAL-TYPE", "CONTEXT-MODE-DECLARATION-GROUP-REF", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "ACTIVATION", "EXECUTION-TIMES", "CALC-RAM-BLOCK-CRC", "IEEE-1722-TP-CONNECTION-REF-CONDITIONAL", "TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY", "SW-RECORD-LAYOUT-V-AXIS", "NM-PASSIVE-MODE-ENABLED", "PROP-SEG", "DNS-SERVER-ADDRESS", "VEHICLE-ANNOUNCEMENT-COUNT", "AFFECTION-KIND", "FORMULA-CAPTION", "DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING", "NETWORK-ENDPOINT", "DDS-RPC-SERVICE-REF", "NM-SHUTDOWN-DELAY-TIMER", "ALLOCATOR", "TIME-SYNC-P-PORT-PROTOTYPE-IREF", "KEYWORDS", "STATE-DEPENDENT-FIREWALL", "VERIFICATION-HASH", "ENABLE-WRITE-ACCESS-CONTROL", "TLS-JOB-MAPPINGS", "PROVIDER-IREF", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE", "CONSUMED-SERVICE-INSTANCES", "CYCLE-REPETITION", "FUNCTIONAL-REQUEST-REF", "NM-MAIN-FUNCTION-ACROSS-FR-CYCLE", "REQUIRE-RESOURCES", "FRAME-PIDS", "DROP-NOT-REQUESTED-NAD", "STATE-MANAGEMENT-ERROR-COMPARE-RULE", "PUBLISHER", "TIME-TO-LIVE", "DIAGNOSTIC-CONDITION-INTERFACE", "BSW-MODULE-TIMING", "STARTS-ON-EVENT-REF", "BLUEPRINT-CONDITION", "IDSM-RATE-LIMITATION-REF-CONDITIONAL", "DDS-SERVICE-QOS-PROFILE-REF", "I-SIGNAL-PORT-REF", "INITIAL-DYNAMIC-PART", "BSW-INTERRUPT-ENTITY", "DIAGNOSTIC-INDICATOR", "ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS", "CONTROL-PROVIDED-EVENT-GROUP-REFS", "PDU-COLLECTION-TIMEOUT", "SW-ADDR-METHOD-REF", "SENDER-IREF", "NM-IF-ECUS", "RUNNABLE-ENTITY-GROUP-IREFS", "SELECTOR-FIELD-LENGTH", "NV-BLOCK-DESCRIPTOR", "ASSIGNED-DATA-TYPES", "ECUC-INSTANCE-REFERENCE-VALUE", "PHYSICAL-PROPS", "ASSIGNED-TRAFFIC-CLASS", "ETHER-TYPE", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "TARGET-OPERATION-REF", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL", "TARGET-R-PORT-PROTOTYPE-REF", "UNICAST-SECURE-COM-PROPS-REF", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "HTTP-TP", "CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST", "EXEC-IN-USER-MODE", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "INTERFACE-MAPPING-SET", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "E-2-E-PROFILE-CONFIGURATIONS", "STATE-MANAGEMENT-ACTION-LIST", "REPRESENTED-PORT-GROUP-REF", "TIME-BASE-PROVIDER-REF", "MSR-QUERY-CHAPTER", "EVENT-MAPPINGS", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "INTERRUPT-CATEGORY", "DEFAULT-PERIOD-BEGIN", "EXPIRED-SUPERVISION-CYCLE-TOL", "BSW-SERVICE-DEPENDENCY", "SERVICE-INTERFACE-TRIGGER-MAPPING", "PERSISTENCY-PORT-PROTOTYPE-IREF", "RECORD-ELEMENT-REF", "ACTION-LIST", "REVISION-LABELS", "RX-MASK", "INSTANCE-IN-MEMORY", "CAN-COMMUNICATION-CONNECTOR", "DIAGNOSTIC-MEMORY-REF", "REQUEST-DATA-SIZE", "SYNC-INTERVAL", "TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "SOVD-GATEWAY-INSTANTIATION", "USED-FID-REF", "TCP-IP-IP-NUM-REASS-DGRAMS", "PRESTORED-FREEZEFRAME-STORED-IN-NVM", "BACKGROUND-EVENT", "ORDERED-MASTER", "RPT-EXECUTION-CONTROL", "FIELD-SENDER-COM-SPEC", "SIZE-OF-STRUCT-LENGTH-FIELDS", "CONSTANT-REFERENCE", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "STREAM-GATES", "J-1939-NM-ECU", "DIAGNOSTIC-SOVD-LOG", "HANDLE-OUT-OF-RANGE", "TBODY", "RECOMMENDED-CONFIGURATION-REF", "CAN-XL-CONFIG-REF", "CONSIDER-BUILD-NUMBER", "ECUC-CHOICE-CONTAINER-DEF", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "GETTER-CALL-SIGNAL-REF", "INTENDED-TARGET-MACHINE-REF", "IMPLEMENTS-LEGACY-STRING-SERIALIZATION", "LABELED-ITEM", "I-PDU-PORT-REF", "ACCESS-COUNT-SETS", "COMM-CONNECTORS", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "UNICAST-NETWORK-ENDPOINT-REF", "SEGMENT-POSITIONS", "TCP-IP-ICMP-V-6-HOP-LIMIT", "FIXED-LENGTH", "ETHERNET-COMMUNICATION-CONTROLLER", "BURST-SIZE-RESPONSE", "SOFTWARE-CLUSTER-SOVD-ADDRESS", "SCALING-INFO-SIZE", "RPT-IMPL-POLICY", "CUSTOM-SPECIFICATION-REF", "SO-AD-CONFIG", "RATE-LIMITATION-FILTERS", "PORT-GROUP", "TCP-IP-NDP-MAX-RANDOM-FACTOR", "PNC-PDUR-GROUP-REF", "IPV-4-ADDRESS-SOURCE", "COM-FIELD-GRANT", "REQUIRED-GENERATOR-TOOLS", "MAY-BE-UNCONNECTED", "TLS-SECURE-COM-PROPS-REF", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "OVERRIDE-INITIAL-STATE-IREF", "DIAGNOSTIC-SECURITY-ACCESS", "DDS-CP-CONFIG", "TEXT", "CLIENT-SERVER-OPERATION-COM-PROPS", "SUPERVISED-ENTITY-IREF", "SOCKET-CONNECTION", "IO-HW-ABSTRACTION-SERVER-ANNOTATIONS", "MAC-ADDRESS-MASK", "SPEEDUP", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL", "APPLICATION-MODE-MACHINE", "TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED", "BASE-CYCLE", "TCP-IP-ICMP-V-4-TTL", "PERSISTENCY-DATA-ELEMENT", "CONTEXT-COMPONENT-REFS", "MACRO-PER-CYCLE", "BSW-INTERNAL-TRIGGERING-POINT-POLICY", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "BSW-MODULE-ENTRY", "MESSAGE-ID", "BUFFER-RATIO", "SW-VALUE-CONT", "REQUIRED-INTERFACE-TREF", "EXTENDED-ADDRESSING", "SUB-FUNCTION-REFS", "CLAIMED-FUNCTION-GROUP-REFS", "SOURCE-TRIGGER-REF", "TAILORING-REFS", "SOMEIP-EVENT-GROUP", "KEY-SERVER-PRIORITY", "INCLUDED-MODE-DECLARATION-GROUP-SETS", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "PROVIDED-SERVICE-INSTANCE-REF", "RESUME-POSITION", "REQUIRED-COM-SPECS", "SEGMENT-REFS", "BSW-MODULE-ENTRY-REF", "APPLICATION-PARTITION-REF", "SIGNAL-BASED-EVENT-REF", "DATA-TYPE-MAP", "DERIVED-INTERFACE-REF", "CAPTION", "DEBOUNCE-ALGORITHM-PROPSS", "HEALTH-CHANNELS", "SPORADIC-EVENT-TRIGGERING", "SW-TEST-DESCS", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "FUNCTION-GROUPS", "DID-NUMBER", "TLS-SECURE-COM-PROPS", "FLAT-MAP-ENTRY-REFS", "SESSION-HANDLING-SR", "KEEP-ALIVE-TIME", "SERVICE-DEPENDENCYS", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "STATE-DEPENDENT-STARTUP-CONFIGS", "INDICATOR-STATUS-NEEDS", "SERVICE-INSTANCE-COLLECTION-SET", "RUNNABLE-ENTITY-IREF", "EXTERNAL-REPLACEMENT-REF", "FIELDS", "LOGICAL-SUPERVISIONS", "VENDOR-SPECIFIC-MODULE-DEF-REF", "UCM-SOFTWARE-VERSION", "INTERRUPT-SOURCE", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "ROM-BLOCK-INIT-VALUE", "KEY-EXCHANGE-AUTHENTICATION-REFS", "STATIC-SOCKET-CONNECTIONS", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "OPTION-NAME", "SPECIFICATION-DOCUMENT-SCOPE", "END-TO-END-PROTECTION", "POSSIBLE-ERROR-REACTION", "SEC-OC-JOB-REQUIREMENT", "TRANSMISSION-MODE", "READER-ENTITY-ID", "SOFTWARE-COMPOSITION-TREF", "DERIVATION", "OPERATION-INVOKED-EVENT", "PDU-TO-FRAME-MAPPINGS", "ACCEPTED-STARTUP-RANGE", "DIAGNOSTIC-IUMPR", "DATA-TYPE-MAPPING-REFS", "TYPE-APPROVAL", "NESTED-ELEMENT-REF", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF", "ECU-PARTITION-REF", "DDS-TOPIC-ACCESS-RULE", "DATA-WRITE-ACCESSS", "FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY", "ASSIGNED-PORTS", "SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "TX-ENABLE-WINDOW-LENGTH", "SW-COMPONENT-PROTOTYPE", "SPARSE-TIMESTAMP-ENABLED", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS", "SERVICE-INSTANCE-REF", "BSW-OS-TASK-EXECUTION-EVENT", "SECOND-ELEMENT-INSTANCE-IREF", "CRC-CORRECTION-FIELD", "ALLOW-UNAUTHENTICATED-PARTICIPANTS", "LOWER-BOUNDARY", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "CAN-COMMUNICATION-CONTROLLER", "MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL", "COMPRESSED-SOFTWARE-PACKAGE-SIZE", "COM-METHOD-GRANT", "ETH-GLOBAL-TIME-DOMAIN-PROPS", "ALLOW-MASTER-RATE-CORRECTION", "NM-USER-DATA-ENABLED", "AR-RELEASE-VERSION", "INFO-TYPE-REF", "PATH-MTU-ENABLED", "DATA-PROTOTYPE-TRANSFORMATION-PROPS", "MEM-USAGE", "OBJECT-DEFINITION-REFS", "SW-AXIS-CONT", "TRANSPORT-PDU-REF", "PDU-ACTIVATION-ROUTING-GROUP", "ALL-CHANNELS-REFS", "RESTRICTIONS", "GENERAL-PURPOSE-PDU", "ITEM-LABEL", "LITERALS", "STATE-MANAGEMENT-BASED-ON-TCP-FLAGS", "BASE-REFERENCE-REF", "LOGICAL-EXPRESSION-ARGUMENT-REF", "INCLUDED-LIBRARY-REFS", "MAX-NUMBER-OF-EVENT-ENTRIES", "OPTIONAL-ELEMENTS", "MODE-SWITCH-RECEIVER-COM-SPEC", "MODE-DECLARATION-GROUP-PROTOTYPE-IREF", "COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER", "DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL", "SYNCHRONIZED-TIME-BASE-CONSUMER", "LIVELINESS", "OBD-DTC-VALUE", "EID", "QOS-PROFILE", "CHECKPOINTSS", "DESTINATION-MIN-ADDRESS", "BYTE-VALUE", "REQUIRED-DDS-EVENT-QOS-PROPS", "HARDWARE-CONFIGURATION", "SECURE-PROPS-FOR-EXTERNAL-COMM-REF", "TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION", "HEADER-ID-LONG-HEADER", "ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "RX-SECURITY-VERIFICATION", "INSTANCE-ID", "SIGNIFICANCE", "GENERATOR", "WATCHDOG-ACTION-ITEM", "DIAGNOSTIC-DATA-ELEMENT", "CRYPTO-MODULE-INSTANTIATION", "EVENT-CONTROLLED-TIMING", "FRESHNESS-VALUE-TX-LENGTH", "CONTEXT-LIMITATION-REF", "SHARED-PARAMETERS", "DCM-I-PDU", "READ-SCALING-DATA-CLASS-REF", "TRANSFORMATION-I-SIGNAL-PROPSS", "SWC-MODE-SWITCH-EVENT-REF", "ALLOCATE-SHADOW-COPY", "MONITORED-IDENTIFIER-REF", "TTCAN-COMMUNICATION-CONTROLLER", "FINAL-CHECKPOINT-REFS", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS", "DO-IP-ENTITY-ROLE", "CAN-CLUSTER-CONDITIONAL", "NOTATION", "MAX-DELTA-COUNTER-INIT", "CRYPTO-PROVIDERS", "TX-MASK", "TTCAN-COMMUNICATION-CONNECTOR", "SOCKET-CONNECTION-REF", "J-1939-NM-NODE-REF", "RPT-SW-PROTOTYPING-ACCESS", "EXPECTED-ALIVE-CYCLE", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "IKE-RAND-TIME", "OFFSETS", "RESPONSE-ON-ALL-REQUEST-SIDS", "SYSC-REF", "DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF", "REQUIRED-AR-ELEMENT-REF", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "SHORT-NAME-PATTERN", "RESPONSE-REF", "POSSIBLE-AP-ERROR-REFS", "PNC-VECTOR-LENGTH", "SEARCH-BEHAVIOR", "LIN-ORDERED-CONFIGURABLE-FRAMES", "EVALUATION-ID", "APPLICABILITY-INFO", "DO-NOT-FRAGMENT", "SYMBOL-WINDOW-ACTION-POINT-OFFSET", "EXPECTED-ALIVE-INDICATIONS", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "TD-PDU-TRIGGERING-FILTER-REFS", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "SUB-ATTRIBUTE-TAILORINGS", "TYPE-REFERENCE-REF", "ACTIVATION-SWITCH", "SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "DATA-CONSTR-RULE", "NM-NODE-REF", "IEEE-1722-TP-AAF-CONNECTION", "SWC-TO-SWC-SIGNAL", "FM-COND", "DIAGNOSTIC-ENV-MODE-CONDITION", "TLV-IMPL-RECORD-ELEMENT", "DIAGNOSTIC-SECURE-CODING-MAPPING", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS", "IDSM-INSTANCE-ID", "INTERPOLATION-ROUTINES", "V-LAN-REF", "PROVIDED-MODE-GROUP-REF", "PN-RESET-TIMER", "MAJOR-PROTOCOL-VERSION", "META-DATA-FIELDS", "MODE-REQUEST-TYPE-MAP", "FM-FEATURE-MAP-CONDITION", "STATE-MANAGEMENT-STATE-NOTIFICATION", "ARGUMENT-MAPPINGS", "AUDIENCE", "INCLUDED-VARIANT-REF", "E-2-E-PROFILE-CONFIGURATION-SET", "SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME", "SERIALIZATION-TECHNOLOGY-REF", "AUTHENTICATION-ROLE-REFS", "N-PDU-REF", "PNC-WAKEUP-ENABLE", "MINIMUM-SEND-INTERVAL", "GETTER-RETURN-REF", "USER-DEFINED-COMMUNICATION-CONTROLLER", "MSR-QUERY-RESULT-TOPIC-1", "VIEW-MAP-SET", "CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL", "ELEMENT-PROPSS", "SLEEP-MODE-EXECUTION-DELAY", "DIAGNOSTIC-PARAMETER-REF", "MEMORY-USAGES", "GLOBAL-TIME-PROPS", "SW-IMPL-POLICY", "CAN-XL-PROPS-REF", "DLT-ARGUMENTS", "NM-NODE-REFS", "DIAGNOSTIC-EVENT-NEEDS", "MC-FUNCTION-DATA-REF-SET-CONDITIONAL", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "GETTER-OPERATION-REF", "TCP-IP-NDP-MAX-RTR-SOLICITATIONS", "SENDER-REC-ARRAY-TYPE-MAPPING", "I-SIGNAL-GROUP", "DESTINATION-REF", "FLEXRAY-FRAME", "ROOT-PARAMETER-DATA-PROTOTYPE-REF", "MC-VARIABLE-INSTANCES", "EXPLICIT-RPT-PROFILE-SELECTION-REFS", "NUMBER-OF-NOTIFIER-SETS", "DIAGNOSTIC-SESSION", "RECOVERY-VIA-APPLICATION-ACTION-REF", "IP-V-4-ADDRESS", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "USES-TIME-BASE-RESOURCE", "PERSISTENCY-KEY-VALUE-DATABASE", "TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED", "SUPPORTED-CONFIG-VARIANT", "VEHICLE-IDENTIFICATION-SYNC-STATUS", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "FMI", "LIMIT-KIND", "CALIBRATION-PARAMETER-VALUE-SET-REFS", "IDENTICAL-MAPPING", "SWC-RUNNABLE-REF", "ACTIVE-SUPERVISION-REFS", "SIZE-OF-STRING-LENGTH-FIELDS", "PERIOD", "DIAGNOSTIC-FIM-ALIAS-EVENT", "EVENT-READINESS-GROUPS", "INCLUDED-LIBRARY-REF", "KEY-SLOT-USED-FOR-START-UP", "PREPARE-ROLLBACK", "NV-BLOCK-DATA-MAPPING", "SET", "SCHEDULER-MAX-NUMBER", "TP-CHANNELS", "DIAGNOSTIC-FREEZE-FRAME", "SUB-FUNCTION-REF", "DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE", "COUPLING-PORT-ROLE", "LOG-TRACE-LOG-MODES", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "MIN-OK-STATE-VALID", "IMPOSITION-TIME-DEFINITION-GROUP", "DIAGNOSTIC-CONTROL-DTC-SETTING", "NUMBER-OF-CONNECTED-CLIENTS", "FINAL-CHECKPOINT-REF", "TLS-DEPLOYMENT", "INTERNAL-TRIGGERING-POINTS", "ASSIGNED-FRAME-TRIGGERING-REF", "TIME-SYNC-R-PORT-PROTOTYPE-IREF", "IEEE-1722-TP", "GENERAL-PURPOSE-I-PDU", "NM-PDUS", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF", "SEPARATION-TIME-REQUEST", "SDX-REF", "SW-COMPONENT-PROTOTYPE-IREF", "TLV-DATA-ID", "MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF", "ERROR-TRACER-NEEDS", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "NOTE", "I-PV-6-EXT-HEADER-FILTER-SET", "SYMBOLIC-NAME-PROPS", "LATENCY-TIMING-CONSTRAINT", "SOMEIP-SERVICE-DISCOVERY-PORT", "NM-CAR-WAKE-UP-FILTER-ENABLED", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "PARAMETER-ACCESS", "IPV-4-ADDRESS", "SEPARATION-CYCLE-EXPONENT", "ECU-STATE-MGR-USER-NEEDS", "RUN-MODE", "FUNCTIONAL-UNIT", "OPTIONAL", "DIAGNOSTIC-MONITOR-INTERFACE", "PHYSICAL-CHANNEL-REF", "FRAME-TRIGGERING-REF", "PASS-THROUGH-SW-CONNECTOR", "REQUIRED-TRIGGER-IREF", "VIOLATED-SAFETY-CONDITION-BEHAVIOR", "REQUEST-2-SUPPORT", "PERSISTENCY-FILE-ELEMENT", "CIPHER-SUITE-NAME", "NOTIFIER-DATA-ELEMENT-REF", "FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING", "INGRESS-PORT-REF", "COMPLEX-TYPE-MAPPING", "IE", "DIAGNOSTIC-TRANSFER-EXIT", "USE-INSTEAD-REFS", "IMPLICIT-INTER-RUNNABLE-VARIABLES", "BUILD-ACTION-IO-ELEMENT", "NETWORK-CONFIGURATION-REF", "ETH-IP-PROPS", "HEADER-ID", "TIME-BASE-KIND", "HW-ELEMENT", "ADDRESS-CLAIM-ENABLED", "RECOVERY-NOTIFICATION-REF", "PAYLOAD-BYTE-PATTERN-RULE-PARTS", "MODE-TRANSITION", "DIAGNOSTIC-CONTROL-NEEDS", "UPPER-BOUNDARY", "ETHERNET-RAW-DATA-STREAM-GRANT", "HISTORY", "SYNC-COUNTER-INIT", "DEFAULT-PATTERN", "FLEXRAY-PHYSICAL-CHANNEL", "SCHEDULING-PRIORITY", "COM-MANAGEMENT-PORT-GROUP-IREF", "BSW-TRIGGER-DIRECT-IMPLEMENTATION", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "ALIVE-REFERENCE-CYCLE", "TARGET-PHM-SUPERVISED-ENTITY-IREF", "REQUEST-DOWNLOAD-CLASS-REF", "NETWORK-HANDLE-PORT-MAPPING", "DATA-TRANSFORMATION-REF", "DEFAULT-VALUE", "DEFAULT-VALUE-ELEMENTS", "RATIO-KIND", "OBD-DTC-NUMBER", "LENGTH-EXP", "HW-ATTRIBUTE-LITERALS", "SUPPORTED-FEATURES", "PARAMETER-ACCESSED-FOR-DEBUG-REFS", "DESTINATION-MAC-ADDRESS", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING", "COM-CONFIGURATION-ID", "ACTION-POINT-OFFSET", "EXECUTABLE", "IN-MEASUREMENT-SET", "MODE-INSTANCE", "PER-INSTANCE-MEMORY", "PROVIDED-REQUIRED-INTERFACE-TREF", "MODE-DECLARATION-REFS", "AFFECTED-STATE-IREF", "PHM-RULE", "CRYPTO-R-PORT-COM-SPEC", "REMOTE-SUBJECT-REFS", "FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL", "SEARCH-INTENTION", "M", "RPT-READ-ACCESS", "COMMUNICATION-DIRECTION", "REPORTING-BEHAVIOR", "JOB-REQUIREMENTS", "DISABLED-IN-MODE-IREF", "MODE-DECLARATIONS", "RESISTANT-TO-CHANGED-SW", "TCP-TP-PORT", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "RESPONSE-GROUP-REF", "IIDC-FRACTION-NUMBER", "I-PDU-IDENTIFIER-UDP-REF", "INHIBITING-SECONDARY-FID-REF", "SERVICE-ELEMENT-MAPPING-REF", "SERVICE-INTERFACE-METHOD-MAPPING", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "START-ACTION-REF", "LIN-PID-TO-CAN-ID-MAPPINGS", "SW-AXIS-INDIVIDUAL", "LIN-SLAVE-CONDITIONAL", "CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "I-SIGNAL-TRIGGERING-REF-CONDITIONAL", "DATA-TYPE-FOR-SERIALIZATION-REF", "V-2-X-M-USER-NEEDS", "AUTO-COLLECT", "INTERFACE-BLUEPRINT-MAPPING", "TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE", "REST-RESOURCE-DEF", "CANCELLATION", "UDP-CHECKSUM-HANDLING", "MINIMUM-TX-CONTAINER-QUEUE-SIZE", "STATE-MANAGEMENT-SYNC-ACTION-ITEM", "TRANSMISSION-START-SEQUENCE-DURATION", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "DEFINITION-REF", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS", "MINIMUM-MEMORY-CONSUMPTION", "STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE", "NON-OS-MODULE-INSTANTIATION-REF", "OCCURRENCE-COUNTER-PROCESSING", "STATE-MANAGEMENT-REQUEST-ERROR", "LOAD-BALANCING-WEIGHT", "REQUESTED-MODE-IREF", "ECUC-REFERENCE-DEF", "PORT-DEFINED-ARGUMENT-BLUEPRINT", "IS-UNUSED", "NM-NETWORK-TIMEOUT", "SNAPSHOT-RECORD-CONTENTS", "IMPL-CONSTANT-REF", "SEGMENT-BYTE-ORDER", "RESULTING-RPT-SW-PROTOTYPING-ACCESS", "CONTROL-PNC-REF", "PDU-TRIGGERING-REFS", "TRIGGER-PERIOD", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "NETWORK-LAYER-RULE", "FT", "MODE-SWITCHED-ACK", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "I-SIGNAL", "ATTRIBUTE-NAME", "CAN-TP-ADDRESS", "EXPECTED-TX-TRIGGER", "TRANSMISSION-MODE-FALSE-TIMING", "UCM-REF", "FIREWALL-RULE", "CIPHER-SUITE-PRIORITY", "CRYPTO-SERVICE-KEY", "STIMULUS-REF", "TRANSMIT-PDU-REFS", "SOMEIP-METHOD", "FILTER-PRIORITY", "SOVD-LOCK-REF", "SCALE-CONSTR", "IS-AUTO-VALUE", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "WAKEUP-TX-IDLE", "CUSTOM-SDG-DEF-REFS", "TROUBLE-CODE-UDS-REF", "BUS-MIRROR-CHANNEL-MAPPING-IP", "ERROR-CONTEXT-REFS", "BUFFER-COMPUTATION", "I-PDU-MAPPING", "REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF", "DIAGNOSTIC-CONTRIBUTION-REFS", "CP-SOFTWARE-CLUSTER-REF", "CONTROL-PNC-REFS", "NV-BLOCK-SW-COMPONENT-TYPE", "SW-VERSION", "TLV-DATA-ID-0-REFS", "TARGET-POSSIBLE-ERROR-REF", "ADDITIONAL-INFORMATION", "COMPU-DENOMINATOR", "ACTIVE-SUPERVISION-REF", "TYPE-OF-EVENT-COMBINATION-SUPPORTED", "LIVELINESS-LEASE-DURATION", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "TIME-BASE-RESOURCE-REF", "OBD-CONTROL-SERVICE-NEEDS", "HANDLE-TERMINATION-AND-RESTART", "REST-ENDPOINT-PUT", "MC-DATA-INSTANCE-REF", "IMPLEMENTATION-ARRAY-ELEMENT-REF", "DIAGNOSTIC-STORAGE-CONDITION-REF", "MICRO-PER-CYCLE", "MEMORY-SECTION", "UPPER-CAN-ID", "APPLICABILITY-INFO-SET", "ASSIGNED-CONTROLLER-REF", "INHIBIT-SOURCES", "STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM", "IIDC-TAG", "SYSTEM-MAPPING", "PARTITION-REF", "UDP-REMOTE-PORT", "CSE-CODE", "IP-SEC-CONFIG-PROPS", "CAN-NM-NODE", "REPLAY-PROTECTION-WINDOW", "SW-POINTER-TARGET-PROPS", "SUB-DOMAIN-REFS", "SYSTEM-TRIGGERED-EVENTS", "EVENT-COMBINATION-REPORTING-BEHAVIOR", "NUMBER-OF-RESTART-ATTEMPTS", "MAX-AR", "BSW-ENTRY-RELATIONSHIPS", "OFFSET-CORRECTION-OUT", "LOCAL-CERTIFICATE-REF", "FM-FEATURE-SELECTION-SET", "DO-IP-NETWORK-CONFIGURATION", "AUTHENTICATION-BUILD-ATTEMPTS", "END-TO-END-PROTECTION-I-SIGNAL-I-PDUS", "DIAGNOSTIC-MESSAGE-TYPE", "TARGET-SIGNAL-REF", "SW-VALUE-BLOCK-SIZE", "EXTERNAL-AUTHENTICATIONS", "ROOT-DATA-PROTOTYPE-IN-CS-REF", "HW-ATTRIBUTE-DEFS", "LOGIC-ADDRESS", "IPV-4-CONFIGURATION", "SOURCE-PORTS", "MAX-ERROR-STATE-INIT", "CONSUMED-DDS-SERVICE-EVENTS", "SENDER-CAPABILITY", "INHIBITING-SECONDARY-FID-REFS", "MAX-DELTA-COUNTER", "RESULTING-PROPERTIES", "CHECK-STATIC-BLOCK-ID", "TIME-SYNC-SERVER-REF", "DEFAULT-APPLICATION-TIMEOUT", "EOC-EVENT-REF", "FM-SYSCOND", "FUNCTION-GROUP-MODE-IREFS", "EXECUTION-ERROR", "BSW-MODULE-DEPENDENCY", "MEM-ALIGNMENT", "ECUC-LINKER-SYMBOL-DEF-CONDITIONAL", "PROVIDED-INTERFACE-TREF", "VEHICLE-SYSTEM", "PRODUCT-ID", "SUB-TYPE", "FREEZE-FRAME-REF", "READ-LOCAL-VARIABLES", "SECOND-ELEMENTS", "OFNS-DATA-ID-LISTS", "DELAY-COMPENSATION-A", "SHALL-NOT-RUN-ON-REFS", "VERSION", "DATA-CONSISTENCY-POLICY", "RULE", "TIMING-VARIABLES", "BROADCAST", "SCOPE-REF", "BULK-NV-DATA-DESCRIPTOR", "SUBMESSAGE-TYPE", "ACTION-ITEM-REF", "SW-CARB-DOC", "ALL-PHYSICAL-CHANNELS-REF", "PHYSICAL-CHANNEL-REFS", "DEF", "CRYPTO-NEED", "DATA-PROTECTION-KIND", "REMOTE-ADDRESSS", "UPPER-MULTIPLICITY", "SEC-OC-JOB-SEMANTIC", "FORMULA", "TTCAN-CLUSTER-VARIANTS", "BUS-MIRROR-CAN-ID-RANGE-MAPPING", "TD-EVENT-BSW-MODE-DECLARATION", "DIAGNOSTIC-ENV-CONDITION-FORMULA", "TCP-CONGESTION-AVOIDANCE-ENABLED", "GLOBAL-ELEMENT", "ARGUMENT-DATA-PROTOTYPE", "FIRST-ELEMENT-REFS", "CONFIGURED-NAD", "PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE", "DO-IP-TP-CONNECTION", "MIRROR-STATUS-CAN-ID", "FR-CLUSTER-REF", "TIMEOUT-AS", "ALLOWED-TCP-OPTIONS-REF", "REPOSITORY", "SYNCHRONOUS-SERVER-CALL-POINT", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY", "DOCUMENTATION", "DHCP-ADDRESS-ASSIGNMENT", "IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "PORT-PROTOTYPE-REF-CONDITIONAL", "FIREWALL-STATE-IREFS", "REQUIRED-FIBEX-ELEMENT-REF", "EVENT-SYMBOL-NAME", "REMOTE-ACCESS-CONTROL-ENABLED", "DDS-METHOD-REQUEST-TOPIC-REF", "FM-FEATURE-MAP", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "MSG-ID-MATCH", "CONTEXT-DATA-SOURCE", "IDLE-SLOPE", "AUTHENTICATION-PROPS-REF", "CYCLE-TIME-MAIN-FUNCTION", "STREAM-FILTER-RULE", "FOLLOW-UP-ACTION-REFS", "RID-NUMBER", "ECUC-DESTINATION-URI-DEF", "RPT-EXECUTABLE-ENTITY-EVENT", "RTE-RESOURCE-ESTIMATION", "CONSUMED-SERVICE-INSTANCE", "TCP-IP-IP-REASSEMBLY-TIMEOUT", "HISTORY-KIND", "SECURITY-EVENT-DEFINITION", "SOMEIP-TRANSFORMATION-DESCRIPTION", "NUMERICAL-RULE-BASED-VALUE-SPECIFICATION", "FULL-DUPLEX-ENABLED", "INSTANTIATION-RTE-EVENT-PROPSS", "APPLICATION-RULE-BASED-VALUE-SPECIFICATION", "GETTER-REF", "ML-GRAPHIC", "REST-SERVICE-INTERFACE", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "LOCAL-PORT-REF", "SWC-BEHAVIOR-REF", "IGNORE-AFTER-TX", "INPUT-VARIABLE-TYPE-REF", "CAN-CONTROLLER-ATTRIBUTES", "USAGE", "REFERENCE-BASE", "PROCESS", "NM-DATA-ENABLED", "CLIENT-ID-DEFINITION-SET-REFS", "USAGES", "VALIDATION-ROOT", "MAX-PWM-L", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "MODE-RECEIVER-POLICYS", "WAKE-UP-OVER-BUS-SUPPORTED", "GLOBAL-TIME-FR-MASTER", "VLAN-REF", "PDU-REF", "EXT-HEADER-FILTER-LISTS", "TRIGGER", "VALUE-RESTRICTION", "DEFAULT-REPORTING-MODE", "LIVELINESS-PROTECTION-KIND", "TIMING-CONDITION-FORMULA", "P-2-TIMING", "MINIMUM-START-INTERVAL", "PARAMETER-ACCESSS", "TD-EVENT-BSW-MODULE-TYPE", "AAF-NOMINAL-RATE", "STORE-ON-CHANGE", "NM-NETWORK-HANDLE-REF", "TTL", "TARGET-VARIABLE-DATA-PROTOTYPE-REF", "FIELD-REFS", "NM-NODE-DETECTION-ENABLED", "VARIABLE-ACCESS-REF", "RECEIVE-ACTIVITY", "STATIC-SOCKET-CONNECTION", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "OWNERSHIP", "TP-SDU-REF", "NM-MULTIPLE-CHANNELS-ENABLED", "DO-IP-SOURCE-ADDRESS-REF", "TD-EVENT-SERVICE-INSTANCE-FIELD", "COUPLING-PORT-REF", "SUB-NODE-CHANNEL-REF", "CALIBRATION-PARAMETER-VALUE", "TRANSPORT-PLUGINS", "MODEL-OBJECT-REFERENCE", "CAN-TP-CONFIG", "MACROTICK-DURATION", "NM-REPEAT-MSG-IND-ENABLED", "DIAGNOSTIC-DE-AUTHENTICATION", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "RETURN-SIGNAL-TRIGGERING-REF", "MEMORY-DESTINATION-REF", "APPLICATION-SW-COMPONENT-TYPE", "TLS-JOB-SEMANTIC", "BSW-ENTRY-RELATIONSHIP-TYPE", "HANDLE-NEVER-RECEIVED", "DDS-DOMAINS", "SW-MC-BASE-TYPE", "DDS-EVENT-DEPLOYMENT", "EVENT-OBD-READINESS-GROUP", "NESTED-ELEMENT-REFS", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF", "EMPTY-SIGNAL", "TIME-BASES", "DEPENDENCY", "SIGNAL-BASED-FIELD-DEPLOYMENT", "NOTIFIER-SIGNAL-TRIGGERING-REF", "OPERATION-MODE", "REQUESTER-IREF", "MAC-LAYER-TYPE", "MEMORY-SECTION-LOCATION", "FILTER-ACTION-VLAN-MODIFICATION", "STATIC-SLOT-DURATION", "SUPPLIER-VALIDATION-ORDER-REFS", "SW-FILL-CHARACTER", "FLEXRAY-TP-CONFIG", "I-PDU-REF", "AGE", "BLOCK-IF-STATE-ACTIVE-AP-IREF", "I-PDU-TRIGGERINGS", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "DDS-RPC-SERVICES", "DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL", "TP-CONFIGURATION", "SW-COMPOSITIONS", "DATA-TYPE-POLICY", "TYPE-OF-DTC-SUPPORTED", "SETTER-RETURN-REF", "UCM-STEP", "DESC", "SERIALIZER-REF", "FREEZE-FRAMES", "MODIFICATIONS", "EVENT-WINDOWS", "EXCLUSIVE-AREA-REFS", "METHOD-QOS-PROPSS", "PNC-WAKEUP-DATA-MASK", "MAXIMUM-KEY-LENGTH", "LOG-TRACE-PORT-ID", "SECURED-AREA-OFFSET", "REFERENCED-BASELINE", "SW-RECORD-LAYOUT-GROUP-STEP", "PERMIT-MULTIPLE-REFERENCES-TO-EE", "HOST-REF", "HW-ATTRIBUTE-LITERAL-DEF", "NEW-NAD", "DDS-RPC-SERVICE-DEPLOYMENT", "CAN-ADDRESSING-MODE", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "MAX-RUN-CYCLE-LOOP", "ALLOWED-SERVICE-PROVIDERS", "INFRASTRUCTURE-SERVICES", "LC-OBJECT-REF", "MASTER-EVENT-REF", "MULTICAST-CONNECTOR-REFS", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "TRANSMIT-PDU-REF", "REMOTING-TECHNOLOGY", "VALIDATION-REF", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "MACHINE-ID", "REQUESTOR-REFS", "TLV-DATA-ID-DEFINITIONS", "LOG-CHANNEL-ID", "SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING", "CIPHER-SUITE-SHORT-LABEL", "THRESHOLD-NUMBER", "INFO-TYPE", "SOURCE-ELEMENT-REFS", "RTE-EVENT-IN-SYSTEM-SEPARATION", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "V-2-X-SUPPORTED", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS", "SEQUENTIAL-INSTRUCTIONS-BEGIN", "SHARED-AXIS-TYPE-REF", "ELEMENT", "NUMBER-OF-REPETITIONS", "FIBEX-ELEMENTS", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "ETH-IP-PROPS-REF", "PERIODIC-RESPONSE-UUDT-REF", "TCP-KEEP-ALIVE-PROBES-MAX", "TO-REF", "SCHEDULING-POLICY", "EXPRESSION", "SOUCE-CAN-ID-REF", "DIAGNOSTIC-FREEZE-FRAME-REF", "TRAFFIC-SHAPER-GROUP-REF", "DDS-EVENT-TOPIC-REF", "SW-CLASS-INSTANCE", "VLAN", "CYCLIC-WRITING-PERIOD", "PORT-PROTOTYPE-SLUG-FRAGMENT", "RPT-SERVICE-POINT-PRE-REF", "STATUS-REF", "CUSTOM-DOCUMENT-ELEMENT-REF", "MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE", "GLOBAL-IN-PACKAGE-REFS", "MAXIMUM-RECEIVE-BUFFER-LENGTH", "CAN-TP-ADDRESS-REF", "ERROR-CODE", "MIN-TRCV-DELAY-COMPENSATION-OFFSET", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "ECU-RESOURCE-MAPPINGS", "CUSTOM-SDG-DEF-REF", "BEST-CASE-EXECUTION-TIME", "PORT-INSTANTIATION-BEHAVIOR", "TP-CONNECTIONS", "PERSISTENCY-ACCESS", "TRACE-REFS", "COMPOSITE-TYPE-MAPPINGS", "PGN", "ASSIGNED-ENTRY-ROLES", "DIAGNOSTIC-VALUE-NEEDS", "BIT", "MODE-GROUP", "NETWORK-IDLE-TIME", "CLIENT-SERVER-TO-SIGNAL-MAPPING", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "TRANSFORMATION-PROPS-SET", "TRANSFER-REF", "SOCKET-ADDRESSS", "IDSM-RATE-LIMITATION", "MINIMUM-SUPPORTED-UCM-MASTER-VERSION", "HOST-ID", "FM-FEATURE-MAP-ASSERTION", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "DEFAULT-ACTION", "KEY-EXCHANGE-AUTHENTICATION-REF", "CRYPTO-ELLIPTIC-CURVE-PROPS", "FUNCTION-INHIBITION-NEEDS", "CHANNELS", "SW-GENERIC-AXIS-PARAM", "E-2-E-PROFILE-CONFIGURATION", "SOVD-VALIDATION-ORDER-REF", "IMPLEMENTATION-DATA-TYPE-REF", "COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION", "AUTH-INFO-TX-LENGTH", "OPERATOR", "DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE", "SYSTEM-TIMING", "SLOT-TYPE", "MODULE-DESCRIPTION-REF", "LOGGING-BEHAVIOR", "UNIT-AND-SCALING-ID", "STIM-ENABLER", "TIMEOUT-SUBSTITUTION-VALUE", "CONTEXT-P-PORT-PROTOTYPE-REF", "MAX-BLOCK-SIZE", "TLV-DATA-ID-DEFINITION-SET", "SW-MAPPINGS", "CONFIGURATION-CLASS-AFFECTION", "SW-GENERIC-AXIS-PARAMS", "DEPENDENT-SOFTWARE-CLUSTER-REF", "SEC-OC-DEPLOYMENT", "I-PDU-TRIGGERING", "SUCCESSOR-REFS", "CONNECTOR-REF", "APPLICATION-MODE-IREF", "CLASSIFICATION", "COM-EVENT-GRANT", "BUS-OFF-RECOVERY", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "L-5", "DATA-ID-NIBBLE-OFFSET", "CHAPTERS", "CONTRACT-VERSION", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "DESCRIPTION", "NM-CAR-WAKE-UP-BIT-POSITION", "STATE-MANAGEMENT-TRIGGER-COMPARE-RULE", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "DATA-LENGTH-REQUEST", "MODULE-REF", "IO-HW-ABSTRACTION-SERVER-ANNOTATION", "VARIABLE-ACCESS", "PORT-INTERFACE-MAPPINGS", "AP-APPLICATION-ENDPOINT", "TEXTUAL-CONDITION", "OUTER-PORTS", "TRIGGER-MODE", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION", "MULTIPLICITY-CONFIG-CLASSES", "PR-PORT-PROTOTYPE", "PORT-ADDRESS", "SEQUENCE-COUNTER", "ECUC-STRING-PARAM-DEF-CONDITIONAL", "CONTAINER-TRIGGER", "DATA-RECEIVE-POINTS", "BIT-OFFSET", "CAN-CLUSTER", "DIAGNOSTIC-READ-DTC-INFORMATION", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF", "SECURE-COM-PROPS-REF", "HEALTH-CHANNEL-IREF", "CONTAINER-TIMEOUT", "DIAGNOSTIC-SERVICE-SW-MAPPING", "MIN-ALIVE-CYCLE", "TRAFFIC-SHAPER-GROUPS", "PRE-ACTIVATION-REBOOT", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "FIRST-ELEMENT-REF", "ERROR-REACTION-POLICY", "MINIMUM-SELECTED-BINDING-TIME", "MINIMUM-INTERVAL-LENGTH", "DDS-METHOD-QOS-PROPS", "COND", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "ECUC-ENUMERATION-LITERAL-DEF", "FIREWALL-STATE-SWITCH-INTERFACE", "RAW-DATA-STREAM-DEPLOYMENT", "GENERIC-MATH", "GLOBAL-SUPERVISION", "CRYPTO-PROVIDER-DOCUMENTATION-REF", "COMMUNICATION-RESOURCE-PROPS", "USER-DEFINED-GLOBAL-TIME-MASTER", "CLIENT-PORT-REF", "CONTROLLER-REF", "SEC-OC-SECURE-COM-PROPS", "SW-CLASS-ATTR-INSTANCE-IMPL", "SEGMENT-POSITION", "CONTEXT-COMPONENT-PROTOTYPE-REF", "V", "AP-APPLICATION-ERROR-REFS", "DIAGNOSTIC-OPERATION-CYCLE-REF", "NETWORK-MASK", "SWC-MODE-SWITCH-EVENT", "TARGET-CATEGORY", "DEBOUNCE-ALGORITHM", "PNC-CLUSTER-VECTOR-LENGTH", "CAN-IDENTIFIER", "CLIENT-SERVER-ANNOTATION", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "NUMBER", "MODE-ELEMENTS", "FLEXRAY-FRAME-TRIGGERING", "OPT-IMPL-RECORD-ELEMENT", "TIMING-GUARANTEES", "STORAGE-CONDITIONS", "DATA-FILTER-TYPE", "NM-BUS-LOAD-REDUCTION-ENABLED", "VARIATION", "DIAG-PDU-TYPE", "TCP-IP-ARP-TABLE-ENTRY-TIMEOUT", "IDSM-MODULE-INSTANTIATION-REF", "EVENT-ELEMENT-MAPPINGS", "I-PDU-TRIGGERING-REF-CONDITIONAL", "WAIT-POINT", "NODE-NAME", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "PER-INSTANCE-MEMORY-REF", "PADDING-VALUE", "FIELD-NOTIFIER-QOS-PROPSS", "BSW-CALLED-ENTITY", "UNIQUE-STREAM-ID", "FIELD-IREF", "INPLACE", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "TEMPLATE-ARGUMENTS", "RUNNABLE-MAPPINGS", "REMAPPED-CAN-ID", "LOWER-MULTIPLICITY", "GENERIC-TP", "CONTEXT-REF", "ECUC-FUNCTION-NAME-DEF", "EVENT-DEPLOYMENTS", "EVENT-MULTICAST-ADDRESSS", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "ACTIONS", "CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS", "INITIAL-REPETITIONS-MAX", "RECOVERY-NOTIFICATION", "PER-INSTANCE-MEMORYS", "KEY-EXCHANGE-REFS", "ELEMENT-REF", "STARTUP-CONFIGS", "SUB-GROUP-REF", "OBJECT-DEFINTION-REF", "PERSISTENCY-DATA-PROVIDED-COM-SPEC", "PDU-TRIGGERING-REF", "DATA-ELEMENTS", "LOCAL-COM-ACCESS-CONTROL-ENABLED", "DIAGNOSTIC-SOVD-LOCK", "AH-CIPHER-SUITE-NAME", "VFB-TIMING", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "ABSOLUTELY-SCHEDULED-TIMINGS", "END-ADDRESS", "LATENCY-CONSTRAINT-TYPE", "PDUS", "FOREIGN-MODEL-REFERENCE", "FRESHNESS-VALUE-ID", "WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST", "SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING", "INSTANCE-ID-PROPS", "POLICY", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "DATA-UPDATE-PERIOD", "FIELD", "COM-MANAGEMENT-MAPPING", "MIN-PWM-L", "ADMIT-WITHOUT-MESSAGE-ID", "USED-SECONDARY-FID-REF", "VCID", "NAMESPACES", "EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS", "MAXIMUM-MEMORY-CONSUMPTION", "SUBTITLE", "MSR-QUERY-RESULT-P-1", "ROOT-VARIABLE-DATA-PROTOTYPE-REF", "LANGUAGE", "OUT-MEASURMENT-SET", "CAN-ENTER-EXCLUSIVE-AREA-REF", "DIAGNOSTIC-ROUTINE-INTERFACE", "START-ACTION-REFS", "PNC-GATEWAY-TYPE", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "ITEMS", "PERIOD-BEGIN", "TIMING-REQUIREMENTS", "STOP", "RANGE-MAX", "LIN-MASTER", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "PACKAGER-SIGNATURE-REF", "TD-EVENT-VFB-REFERENCE", "TEST-PATTERN", "PRE-SHARED-KEY-REF", "SOCKET-CONNECTION-BUNDLE-REF", "ACTIVATION-REASONS", "STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM", "GLOBAL-TIME-MASTERS", "CDD-TYPE", "FLOAT", "ID", "NETWORK-CONFIGURATION-REFS", "INITIAL-CHECKPOINT-REFS", "ICV-VERIFICATION", "RTE-EVENT-SEPARATIONS", "RECORD-NUMBER", "VALUE-IREF", "ECUC-CONTAINER-VALUE", "BSW-VARIABLE-ACCESS", "PARAMETER-PROVIDE-COM-SPEC", "PREPARE-UPDATE", "RESOLUTION", "TIMEOUT-CR", "RVF-COLOR-SPACE", "APPLICATION-ENDPOINT-REF-CONDITIONAL", "RETRY", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "SETTER-OPERATION-REF", "INTERNAL-PRIORITY-VALUE", "ARGUMENT", "LOWER-BOUND", "SOURCE-POSSIBLE-ERROR-REF", "APPLICATION-RECORD-ELEMENT", "GLOBAL-TIME-PORT-ROLE", "NETWORK-REPRESENTATION-PROPS", "REMOTE-PORT-RANGE-END", "NM-HW-VOTE-ENABLED", "PAYLOAD-BYTE-PATTERN-RULE-PART", "FORMAL-BLUEPRINT-CONDITION", "STATE-DEPENDENT-STARTUP-CONFIG", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "SET-FUNCTION-GROUP-STATE-IREF", "NM-CBV-POSITION", "SOFTWARE-PACKAGE-STORING", "IDSM-CONTEXT-PROVIDER-MAPPING", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING", "MAIN-FUNCTION-PERIOD", "MAX-TRCV-DELAY-COMPENSATION-OFFSET", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "MAX-LENGTH", "TRANSMISSION-ACKNOWLEDGE", "MAX-SOURCE-PORT-NUMBER", "J-1939-SHARED-ADDRESS-CLUSTERS", "PSK-IDENTITY", "REMOTE-MULTICAST-CONFIG-REF", "USE-SECURED-PDU-HEADER", "BUILD-ACTION-MANIFEST-REF-CONDITIONAL", "SWITCH-STREAM-GATE-ENTRY", "DO-IP-ENTITY", "SOMEIP-EVENT-PROPS", "ECUC-VALIDATION-CONDITION", "HW-ELEMENT-REF", "DRIVER-PRIMITIVE-REF", "EID-USE-MAC", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "DISABLED-MODE-IREF", "LIFESPAN", "FIRST-ELEMENTS", "USED-PARAMETER-ELEMENT", "INCLUDE-REFS", "TRANSITION-REFS", "J-1939-TP-PG", "USER-DEFINED-METHOD-DEPLOYMENT", "CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF", "END-TO-END-PROFILE", "REQUIRED-SERVICE-INSTANCE-REF", "END-2-END-EVENT-PROTECTION-PROPS", "TEMPERATURE-EXP", "CODE-DESCRIPTORS", "FIRST-PORT-REF", "POST-BUILD-VARIANT-CRITERION-VALUES", "COMPONENT-SEPARATION", "SOMEIP-SD-RULE", "VARIABLE-INSTANCE", "APPLICABILITY-INFOS", "RPT-COMPONENT", "MAXIMUM-DURATION-OF-CAMPAIGN", "DESTINATION-IP-ADDRESS", "LIN-UNCONDITIONAL-FRAME-REFS", "CODE", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "ADDRESS-RANGE-UPPER-BOUND", "IMPLEMENTATION-RECORD-ELEMENT-REF", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "IMPLIED-OPERATION-REF", "DESTINATION-URI-POLICY", "EXPIRED-SUPERVISION-TOLERANCE", "APPLICATION-CONTEXT-REFS", "DIAGNOSTIC-PERIODIC-RATE", "TYPE-BLUEPRINTS", "IUMPR-REF", "ENABLE-DEACTIVATION", "SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPINGS", "USER-DEFINED-TRANSFORMATION-PROPS", "IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "COMPU-RATIONAL-COEFFS", "WRITE-CLASS-REF", "INFORMAL-FORMULA", "DATA-PROTOTYPE-GROUP", "DATA-ELEMENT-IREFS", "EXECUTABLE-GROUP", "ECU-TASK-PROXY-REF", "EXCLUSIVE-AREA-REF", "ARRAY-ELEMENT-MAPPINGS", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF", "REF-MEASUREMENT-SET", "CLOCK-REFERENCE-REF", "STATE-MANAGEMEN-PHM-ERROR-INTERFACE", "DDS-QOS-PROFILE-REF", "DATA-IDENTIFIER-REFS", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "WRITTEN-LOCAL-VARIABLES", "ENUMERATION-MAPPING-TABLE", "START-POSITION", "CRYPTO-NEED-REF", "PORT-REF", "ETHERNET-CLUSTER-REF", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPES", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING", "DIAGNOSTIC-VALUE-ACCESS", "DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING", "GLOBAL-TIME-CAN-SLAVE", "MODE-IN-SWC-INSTANCE-REF", "CONFIG-VARIANT", "SECURE-COM-PROPSS", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "SYMBOL", "DEPENDS-ON-REFS", "ETH-TCP-IP-PROPS", "IP-SEC-RULE", "TLV-ARGUMENT-REF", "DATA-PROTOTYPE-IN-METHOD-ARGUMENT-INSTANCE-REF-IREF", "STATE-MANAGEMENT-MODULE-INSTANTIATION", "DIAGNOSTIC-ENV-DATA-CONDITION", "MIN-SOURCE-PORT-NUMBER", "ERROR-CONTEXT-REF", "READ-DTC-INFORMATION-CLASS-REF", "PERMISSION-CERTIFICATE-AUTHORITY-REF", "BSW-MODULE-ENTITY-REF", "TRANSITIONS", "FIRST-MODE-REFS", "MC-FUNCTION-DATA-REF-SET-VARIANTS", "PROVIDE-RESOURCES", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "FRAME-PORT-REFS", "START-ON-EVENT-REF", "SOVD-VALIDATION-ORDER-REFS", "SENDER-INTENT", "RESOURCE-LIMITS", "DEFAULT-PRIORITY", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REF", "ECUC-INSTANCE-REFERENCE-DEF", "MKA-PARTICIPANT-REFS", "PERSISTENCY-PROVIDED-COM-SPEC", "NETWORK-ENDPOINT-ADDRESSES", "COMMUNICATION-CONTROLLER-MAPPING", "SCOPE", "SWITCH-STREAM-IDENTIFICATIONS", "SECOND-APPLICATION-ERROR-REF", "ECUC-DESTINATION-URI-DEF-SET", "BINARY-MANIFEST-RESOURCE-DEFINITION", "BASIC-CYCLE-LENGTH", "CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING", "APPLICATION-VALUE-SPECIFICATION", "FOLLOW-UP-ACTION-REF", "PROCESSOR-CORE", "DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING", "FRAGMENT", "SHORT-TERM-ADJUSTMENT", "METHOD-MAPPING", "REENTRANCY-LEVEL", "ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT", "CRC-MESSAGE-LENGTH", "VIEW-MAPS", "P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF", "TRANSITION-REF", "BUILD-ACTION-MANIFEST-REF", "J-1939-SHARED-ADDRESS-CLUSTER", "MODE-DECLARATION-GROUP", "OS-TASK-PROXY-REF", "COUNTER-JUMP-DOWN", "DIAGNOSTIC-EVENT-REF-CONDITIONAL", "SECURE-COM-PROPS-FOR-DDS-REF", "DDS-TOPIC-REF", "COUPLING-PORT-SHAPER", "DIAGNOSTIC-COMMON-PROPS-VARIANTS", "VEHICLE-SYSTEM-INSTANCE", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "MIN-NUMBER-OF-TIME-QUANTA-PER-BIT", "CONTEXT-DATA-PROTOTYPE-IN-CS-REF", "ENABLE-LIVELINESS-PROTECTION", "AP-APPLICATION-ERROR-SET", "DELAY", "STD-REF", "SOFTWARE-PACKAGE-STEPS", "SA-REKEY-TIME", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "POST-BUILD-VARIANT-CONDITION", "COMPONENT-IREF", "DATA-PROTOTYPE-IREF", "TRANSMISSION-PROPS", "SD", "TRANSFORMER-HARD-ERROR-EVENT", "TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT", "AFFECTED-REF", "SUPERVISION-CHECKPOINT", "PERIOD-END", "FLOW-METERING-REF", "SECURE-COM-PROPS-FOR-UDP-REFS", "VENDOR-ID", "TD-EVENT-I-SIGNAL", "PNC-FILTER-ARRAY-MASKS", "SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL", "RELEASE-NOTES-REF", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "SW-ARRAYSIZE", "TIMEOUT-CHECK", "HW-ATTRIBUTE-VALUE", "USE-AUTH-DATA-FRESHNESS", "FLEXRAY-TP-PDU-POOL", "CRYPTO-JOBS", "CRF-TYPE", "PNC-WAKEUP-CAN-ID", "CLEAR-DTC-LIMITATION", "NETWORK-TIME-MASTER-REF", "UNIT-GROUP-REFS", "COUNTER-OFFSET", "CONTAINERS", "EXECUTABLE-LOGGING-IMPLEMENTATION-PROPS", "IEEE-1722-TP-ACF-CAN-PART", "DDS-TOPICS", "IP-IAM-AUTHENTIC-CONNECTION-PROPS", "UNIT-GROUP", "LIN-CONFIGURABLE-FRAME", "DENOMINATOR-GROUP", "TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE", "SERVICE-INTERFACE-EVENT-MAPPING", "I-SIGNAL-REF", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "UCM-SUBORDINATE-MODULE-INSTANTIATION", "CLIENT-SERVER-RECORD-ELEMENT-MAPPING", "EXCLUSIVE-AREA-NESTING-ORDER-REF", "NM-NODE-ID-ENABLED", "LOCAL-SUPERVISION-ENTITY-REFS", "DISCOVERY-TECHNOLOGY", "FLAT-INSTANCE-REF", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "CONNECTORS", "DIAGNOSTIC-DATA-TRANSFER", "DATA-PROTOTYPE-REF", "COMPU-CONST", "ECU-SCOPE-REFS", "IS-PRIVATE", "SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION", "IS-VARIADIC-TEMPLATE", "ECUC-IMPLEMENTATION-CONFIGURATION-CLASS", "I-PDU-IDENTIFIER-TCP-REFS", "TLS-IAM-REMOTE-SUBJECT", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "ECU-EXTRACT-REFERENCE-IREF", "ACCESS-PERMISSION-REF", "SERVICE-INSTANCES", "ACTION-ITEM-LISTS", "ALLOW-PROVIDER-RATE-CORRECTION", "UNICAST-UDP-CREDENTIALS", "SUPERVISION-MODE-CONDITIONS", "TIME-BUFFER", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF", "EXECUTABLE-ENTITY-REF", "IP-PROTOCOL", "COMPU-INVERSE-VALUE", "ETH-TCP-IP-ICMP-PROPS", "SW-AXIS-TYPE", "IPV-4-ADDRESS-MASK", "POST-BUILD-VARIANT-VALUE", "PORT-DEFINED-ARGUMENT-VALUE", "SYNC-FRAME-ID-COUNT-MAX", "MESSAGE-LINK-POSITION", "USE-AUTO-VALIDATION-AT-SHUT-DOWN", "LOCAL-IP-SEC-RULE-REFS", "DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING", "DATA-REF", "VALUE-CONFIG-CLASSES", "SERVER-SERVICE-MINOR-VERSION", "SIGNAL-SERVICE-TRANSLATION-PROPSS", "PORT-INTERFACE-REF", "TEST-IDENTIFIER", "LET-INTERVAL-REFS", "IEEE-1722-TP-ACF-CAN", "GLOBAL-SUPERVISION-ENTITY", "DIFFERENTIATED-SERVICE-FIELD", "CONSUMED-EVENT-GROUP-REFS", "MATCHING-INGRESS-RULE-REF", "SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION", "SEED-SIZE", "COM-MANAGEMENT-GROUP-REFS", "NETWORK-TIME-SLAVE-REF", "NETWORK-ENDPOINT-REFS", "SERVICE-INTERFACE-MAPPING", "LIN-SLAVE-VARIANTS", "SECURE-PROPS-FOR-TCP-REF", "MAX-INSTANCES", "REQUIRED-PACKAGE-ELEMENT-REF", "TT-CAN-CLUSTER-REF", "MESSAGE-SOURCE-FILE", "BASE-TYPE-REF", "DIAGNOSTIC-ECU-RESET-CLASS", "TD-EVENT-MODE-DECLARATION-TYPE", "APPLICATION-DEFERRED-DATA-TYPE", "J-1939-CONTROLLER-APPLICATION", "BASE-REF", "TRIGGERS-RECOVERY-NOTIFICATION", "PREDECESSOR-REFS", "ENABLE-TAKE-ADDRESS", "SECOND-OPERATION-REF", "CERTIFICATE-EVALUATIONS", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "RECEPTION-PROPS", "INHIBITING-FID-REF", "PHM-HEALTH-CHANNEL-STATUS", "BSW-MODULE-INSTANCE-REF", "FUNCTION-POINTER-SIGNATURE-REF", "COUPLING-PORT", "TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "NM-CLUSTER-COUPLINGS", "MEASURED-STACK-USAGE", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "EXECUTION-ORDER-CONSTRAINT", "ON-TRANSITION-VALUE", "GETTER-CALL-REF", "NO-SUPERVISION", "REQUIRED-EVENT-GROUPS", "SERVER-CALL-POINTS", "SIGNAL-SERVICE-TRANSLATION-PROPS", "INTER-MESSAGE-TIME", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF", "MAX-NO-NEW-OR-REPEATED-DATA", "USER-DEFINED-TRANSFORMATION-DESCRIPTION", "SWC-SERVICE-DEPENDENCY-IREF", "DTC-PROPS-REF", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL", "MODE-REF", "SUB-ELEMENT-MAPPINGS", "I-SIGNAL-TRIGGERING-REF", "DEF-ITEM", "DDS-SERVICE-VERSION", "ARRAY-IMPL-POLICY", "MAPPED-DATA-ELEMENT-IREF", "ROLE", "SECURE-COM-PROPS-SET", "POWER-DOWN-TIME", "TCP-PORT-REF", "FIRE-AND-FORGET-METHOD-MAPPING", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "MC-DATA-ASSIGNMENTS", "PORT-CONFIGS", "CUSTOM-SERVICE-CLASS-REF", "KEEP-ALIVE-INTERVAL", "KEEP-ALIVES", "MAXIMUM-SELECTED-BINDING-TIME", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "NM-BUSLOAD-REDUCTION-ENABLED", "SERVICE-IDENTIFIER", "GENERATED-ARTIFACTS", "ECUC-MULTIPLICITY-CONFIGURATION-CLASS", "SYMBOLIC-NAME-VALUE", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "MAC-ADDRESS-STREAM-ID", "TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "KEY-SLOT-ID", "PARTITION-NAME", "CHECKPOINTS", "SERVICE-INTERFACE-ELEMENT-MAPPING-REFS", "SHALL-RUN-ON-REF", "PROVIDED-COM-SPECS", "SW-DATA-DEPENDENCY-ARGS", "REMOTE-PORT-RANGE-START", "VEHICLE-PACKAGE", "PREDECESSOR-REF", "SCALE-CONSTRS", "ECU-INSTANCE-PROPSS", "CONTEXT-DATAS", "DIAGNOSTIC-ADDRESS", "SLAVE-EVENT-REF", "AR-TYPED-PER-INSTANCE-MEMORYS", "OPT-ARGUMENT-REFS", "IDSM-INSTANCE-REF-CONDITIONAL", "SECURE-COM-CONFIG-REF", "TIME-SYNC-PORT-PROTOTYPE-IREF", "BUILD-ACTION-MANIFEST", "DIAGNOSTIC-RESPONSE-ON-EVENT", "NM-COORDINATOR", "IEE-E-1722-TP-CONNECTION-REF", "SUPPORTED-CIPHER-SUITES", "TIMESTAMP-FORMAT", "TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE", "COMMUNICATION-CLUSTER-REF", "ACF-TRANSPORTED-BUSS", "SECOND-ELEMENT-REF", "STORE-CYCLIC", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "MC-GROUP-DATA-REF-SET-VARIANTS", "APPLICATION-ACTION-ITEM", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "NM-USER-DATA-OFFSET", "COMM-CONTROLLER-REF", "SUB-NODE-PHYSICAL-CHANNEL-REF", "DELEGATION-SW-CONNECTOR", "DIAGNOSTIC-ENABLE-CONDITION", "MESSAGE-LINK-LENGTH", "FUNCTION-GROUP-STATE-IREF", "STORE-EVENT-ENABLED", "SW-TEXT-PROPS", "REQUEST-VEHICLE-INFORMATION-CLASS-REF", "KEY-GENERATION", "DIAGNOSTIC-ECU-RESET-INTERFACE", "DLT-ECU", "FRAME-MAPPINGS", "MODE-DECLARATION-GROUP-PROTOTYPE", "CONTAINED-FIBEX-ELEMENT-REF", "ECU-INSTANCES", "CONTEXT-SWC-PROTOTYPE-REF", "FILTER", "FLEXRAY-AR-TP-NODE", "NM-MESSAGE-TIMEOUT-TIME", "RVF-FRAME-RATE", "CAN-XL-CONFIG", "AAF-AES-3-DATA-TYPE", "LOAD-BALANCING-PRIORITY", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "ASSUMED-CURRENT-STATE-IREF", "REASON", "CLIENT-SERVER-OPERATION-MAPPING", "FIRE-AND-FORGET-METHOD-MAPPINGS", "COM-PROCESSING-PERIOD", "STATIC-PARTS", "PROVIDED-DATA-REF", "MIN-MAJOR-VERSION", "EXTERNAL-TRIGGERING-POINTS", "FUNCTION-CLUSTER-AFFILIATION", "RUN-RESOURCE", "DIAGNOSTIC-REQUEST-DOWNLOAD", "PHM-CHECKPOINT-IREF", "UDP-PORT", "ALIAS-NAME-SET", "TLS-CONNECTION-REFS", "DIAGNOSTIC-RELEVANCE", "MAJOR-VERSION", "IDSM-TRAFFIC-LIMITATION", "TRANSMISSION-MODE-TRUE-TIMING", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "EVENT-KIND", "USE-AS-CRYPTOGRAPHIC-I-PDU", "NV-DATAS", "TLS-CIPHER-SUITE", "SWITCH-FLOW-METERING-ENTRY", "SEND-INDICATION", "RECOMMENDED-CONFIGURATION-REFS", "ORIGIN", "DEFAULT-TRACE-STATE", "POST-BUILD-CHANGEABLE", "EXCLUSIVE-AREA-REF-CONDITIONAL", "REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF", "TP-CONNECTION-CONTROL-REF", "PROG-CODE", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE", "RESOURCE-GROUP", "MAX-NUMBER-OF-ELEMENTS", "WAKEUP-TX-ACTIVE", "SECURED-AREA-LENGTH", "AUTH-ALGORITHM", "CYCLE-AUTOSTART", "POLICY-ACTION", "SERIALIZATION-TECHNOLOGY", "GENERIC-MODULE-INSTANTIATION", "RESPONSE-ON-EVENT-REF", "MANAGED-MODE-GROUPS", "FLEXRAY-NM-ECU", "RX-IDENTIFIER-RANGE", "SUB-NODE-CHANNELS", "SUB-SOFTWARE-CLUSTER-REFS", "DELEGATED-PORT-ANNOTATION", "TP-ECUS", "TTCAN-ABSOLUTELY-SCHEDULED-TIMING", "SW-RECORD-LAYOUT-GROUP-TO", "SYSTEM-SIGNAL-GROUP", "GLOBAL-SUPERVISION-NEEDS", "COMPU-METHOD-REF", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "ACCESS-PERMISSION-VALIDITY", "PROPS", "REDUNDANCY-HANDLINGS", "ROUTINE-CONTROL-CLASS-REF", "APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF", "I-SIGNAL-REFS", "TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED", "SENDER-RECEIVER-INTERFACE", "TCP-IP-ARP-REQUEST-TIMEOUT", "RESOURCE-DEFINITION-REF", "INITIAL-NAD", "SW-SYSTEMCONSTANT-VALUES", "PRIMITIVE-FAMILY", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "RESTORE-AT-START", "SHORT-NAME", "PREDECESSOR-ACTION-REFS", "SENDER-ANNOTATION", "EMULATION-SUPPORTS", "NM-IMMEDIATE-NM-CYCLE-TIME", "DATA-RECEIVE-POINT-BY-VALUES", "SWC-TRIGGER-IREF", "GROUPED-ALIAS-EVENT-REF", "DEFAULT-LC-STATE-REF", "IEEE-1722-TP-ACF-CONNECTION", "SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS", "MINIMUM-SUPPORTED-UCM-VERSION", "L-4", "NM-REPEAT-MSG-INDICATION-ENABLED", "ECUC-BOOLEAN-PARAM-DEF", "CRC-FLAGS", "MAX-FC-WAIT", "CONSUMED-EVENT-GROUP-REF", "DYNAMIC-ACTION-REF", "IPV-4-DHCP-SERVER-CONFIGURATION", "AUXILIARY-FIELD-DEFINITIONS", "MAC-MULTICAST-ADDRESS-REFS", "AAF-FORMAT", "TRANSFORMER-CHAIN-REFS", "TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED", "NETWORK-HANDLES", "TRANSPORT-PROTOCOLS", "FALL-BACK-INTERNAL", "INGRESS-PORT-REFS", "MEM-CLASS-SYMBOL", "RESOURCE-GROUP-REFS", "TD-HEADER-ID-RANGE", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "NONQUEUED-SENDER-COM-SPEC", "SLAVE-REF", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "SWC-TO-APPLICATION-PARTITION-MAPPING", "SD-SERVER-TIMER-CONFIGS", "GLOBAL-TIME-GATEWAY", "XREF", "HW-COMMUNICATION-CONTROLLER-REF", "NOTIFICATION-TIME", "SCHEDULE-TABLES", "STARTUP-CONFIG", "SYMBOL-PROPS", "NV-PROVIDE-COM-SPEC", "MC-DATA-INSTANCE-VAR-REF", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "UPDATE", "TD-EVENT-I-PDU", "BSW-COMPOSITION-TIMING", "NM-CHANNEL-SLEEP-MASTER", "SUPERVISION-MODE-CONDITION", "TCP-IP-NDP-DYNAMIC-MTU-ENABLED", "TYP", "COMPU-DEFAULT-VALUE", "OFFSET", "START-STATE-MACHINE-REF", "PACKING-BYTE-ORDER", "PHM-ACTION-LIST", "WRITE-VERIFICATION", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "NM-CAR-WAKE-UP-RX-ENABLED", "LOCAL-UDP-PORT-REF", "DEPLOYMENT-URIS", "IPV-6-ADDRESS-MASK", "POST-BUILD-VARIANT-USED", "REST-SYSTEM-TRIGGERED-EVENT", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "REST-ENDPOINT-GET", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS", "TRANSPORT-PRIORITY", "HANDLE-TIMEOUT-TYPE", "CONSIDER-PTO-STATUS", "CAN-TP-ADDRESS-REFS", "BLUEPRINT-POLICY-NOT-MODIFIABLE", "THRESHOLD", "MAX-WITHOUT-CLOCK-CORRECTION-FATAL", "REQUESTER-SOFTWARE-CLUSTER-REF", "COUPLED-CLUSTER-REFS", "ACCESSED-VARIABLE", "KEY-SLOT-ALLOWED-MODIFICATION", "SHORT-LABEL", "FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "LONG-NAME-1", "SUPPORTS-MULTIPLE-NOTIFIER-SETS", "ACL-SCOPE", "TIMING-RESOURCE", "COMMUNICATION-CONNECTOR-REF", "CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF", "SEPARATED-COMPONENT-IREF", "CORE-ID", "DIRECT-TP-SDU-REF", "BSW-DATA-SEND-POLICY", "DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE", "MSR-QUERY-RESULT-P-2", "NONQUEUED-RECEIVER-COM-SPEC", "ADDRESSING-FORMAT", "WRITING-STRATEGY-ROLE", "DIAGNOSTIC-J-1939-SW-MAPPING", "FILE-PROXYS", "EXECUTABLE-PROVIDED-PORT-IREF", "CONDITIONS", "SPN-REF", "SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "DEADLINE-MIN", "SYNC-LOSS-TIMEOUT", "SAK-REF", "COM-CONFIGURATION-GW-TIME-BASE", "TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE", "USE-FRESHNESS-TIMESTAMP", "VARIANT-CRITERION-REF", "ETHERNET-PHYSICAL-CHANNEL", "DTC-KIND", "MIRRORING-PROTOCOL", "LIN-CLUSTER-CONDITIONAL", "SUBSCRIBE-EVENTGROUP-RETRY-MAX", "PNC-SYNCHRONOUS-WAKEUP", "ECUC-QUERYS", "CONSISTENCY-NEEDS", "PORT-PROTOTYPE-BLUEPRINT-MAPPING", "SIZE-OF-STRUCT-LENGTH-FIELD", "PHYSICAL-DIMENSION", "LIN-CONFIGURABLE-FRAMES", "POSSIBLE-ERROR-REFS", "METHODS", "MINOR-VERSION", "SWC-TO-IMPL-MAPPING", "DLT-ARGUMENT", "OS-ARTI-ADAPTER-LAUNCH-BEHAVIOR", "SIGNAL-MAPPINGS", "RESPONSE-ON-EVENT-CLASS-REF", "TD-EVENT-TRIGGER", "ECUC-MODULE-CONFIGURATION-VALUES-REF", "DIAGNOSTIC-AGING", "FM-FEATURE-SELECTION", "NM-CHANNEL-ID", "SECURITY-EVENT-REF", "SELECTOR-FIELD-BYTE-ORDER", "CLASS-CONTENTS", "DATA-TRANSFORMATION-SET", "FILES", "SECURE-COM-CONFIGS", "EVENT-MAPPING", "ALLOW-CONTENT-TYPE-CHANGE", "SW-RECORD-LAYOUT-GROUP-AXIS", "TARGET-MODULE-REFS", "DO-IP-TARGET-ADDRESS-REF", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "STATUSS", "LIMIT", "CLEAR-EVENT-BEHAVIOR", "RAM-BLOCK-INIT-VALUE", "SERVICE-TABLES", "ENABLE-CONDITION-REF", "NO-SUPERVISIONS", "SERVER-NAME-IDENTIFICATION", "ASSOCIATED-PDUR-I-PDU-GROUP-REF", "SECURITY-LEVEL-REF", "MONOTONY", "SDF", "UDP-MIN-TX-BUFFER-SIZE", "TCP-MAX-RTX", "CLIENT-SERVICE-MAJOR-VERSION", "PARAMETER-SW-COMPONENT-TYPE", "EVENT-MULTICAST-UDP-PORT", "SOURCE-FIELD-REF", "CAN-NM-TX-CAN-ID", "DISABLED-MODE-IREFS", "SIZE-OF-UNION-LENGTH-FIELDS", "HW-ELEMENT-CONNECTOR", "RESOURCE-GROUP-REF", "BSW-DIRECT-CALL-POINT", "SW-CLUSTER-REF", "STATE-MANAGEMENT-SLEEP-ACTION-ITEM", "ECUC-QUERY-STRING-REF", "FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS", "OS-TASK-PROXY", "TARGET-RUNNABLE-ENTITY-REF", "MAX-ALIVE-CYCLE", "RECEIVER-REF", "DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED", "REST-OBJECT-REF", "ARRAY-SIZE-SEMANTICS", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "EVENT-DISPLACEMENT-STRATEGY", "ENDPOINT-CONFIGURATION-REF", "SW-SYSTEMCONSTANT-VALUE-SET-REFS", "SW-CALPRM-IMPLS", "MAXIMUM-TRANSMISSION-UNIT", "EVENT-CLEAR-ALLOWED", "UDP-COLLECTION-BUFFER-SIZE-THRESHOLD", "MODE-CONDITION", "WRITTEN-NV-DATA", "STARTUP-OPTIONS", "ALIVE-SUPERVISION", "LOCAL-COMM-CONNECTOR-REF", "META-DATA-ITEM", "DATA-TYPE-MAPPING", "SPECIFIC-CHANNELS", "DYNAMIC-ACTION-REFS", "RPT-EXECUTABLE-ENTITYS", "TRIGGERS", "CRYPTO-KEY-SLOT-INTERFACE", "ROUTING-GROUP-REFS", "ABSTRACT-CLASS-TAILORING", "VALIDATION-FORMULA", "ETH-TP-CONNECTION", "TCP-FAST-RECOVERY-ENABLED", "SW-IS-VIRTUAL", "MAX-NUMBER-OF-PARALLEL-TRANSFERS", "SECURE-COM-PROPS-FOR-TCP-REF", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "PROCESSING-UNIT-REF", "RTE-PLUGIN-PROPS", "HW-PIN-CONNECTIONS", "PROCESS-DESIGN", "IS-DEFAULT", "TP-CONNECTION-CONTROLS", "GLOBAL-KAY-PROPS-REF", "END-TO-END-PROTECTION-SET", "RP-IMPL-POLICY", "BITFIELD-TEXT-TABLE-MASK-SECOND", "FM-ATTRIBUTE-DEF", "CLIENT-SERVICE-MINOR-VERSION", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF", "MAXIMUM-MESSAGE-LENGTH", "FORBIDDEN-SIGNAL-PATH", "DESTINATION-URI-NESTING-CONTRACT", "SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS", "RUNNABLE-ENTITY-IREFS", "CHAPTER", "SERVICE-DISCOVER-CONFIGS", "C-CODE", "ACL-OBJECT-CLASS", "DEPENDENT-RESOURCES", "SOCKET-PROTOCOL", "SW-INTENDED-RESOLUTION", "TRANSFORMATION-TECHNOLOGYS", "BSW-MODULE-CLIENT-SERVER-ENTRY", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING", "MINIMUM-EXECUTION-TIME", "LIN-ORDERED-CONFIGURABLE-FRAME", "OFFSET-TIMING-CONSTRAINT", "RECEIVER-INTENT", "MIN-RPT-EVENT-ID", "EXPRESSION-REF", "FLOAT-VALUE-VARIATION-POINT", "RETRY-INTERVAL-TIME", "PROFILE-BEHAVIOR", "GLOBAL-TIME-DOMAIN-REF", "ACTIVATE-AT-START", "HAS-SETTER", "CAN-BAUDRATE", "BLUEPRINT-MAPPING", "FAILED-SUPERVISION-CYCLES-TOLERANCE", "TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE", "SERVICE-TIMING", "RESTRICTION", "HW-TYPE-REF", "PNC-PREPARE-SLEEP-TIMER", "ESP-CIPHER-SUITE-NAME", "DEADLINE-SUPERVISION", "SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING", "SAFETY-MARGIN", "URL", "MKA-PARTICIPANTS", "EVENT-REF", "CAN-ID-TO-CAN-ID-MAPPINGS", "IP-V-6-PATH-MTU-ENABLED", "VARIATION-RESTRICTION", "MODULE-ID", "DLT-LOG-SINK", "TRANSMISSION-MODE-DECLARATION", "IPV-6-MULTICAST-IP-ADDRESS", "DATA-RECEIVE-POINT-BY-ARGUMENTS", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "ENUMERATOR-VALUE", "SHORT-NAME-FRAGMENTS", "INTERPOLATION-ROUTINE-MAPPING-SET-REF", "OPERATION-PROTOTYPE-IREF", "FIRST-OPERATION-REF", "ELLIPTIC-CURVE-REFS", "TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS", "MAXIMUM-LENGTH", "CONSTRAINT-TAILORING", "TD-EVENT-FR-CLUSTER-CYCLE-START", "PERSISTENCY-FILE", "FLEXRAY-FIFO-RANGE", "MAPPINGS", "TCP-RULE", "NM-NETWORK-HANDLE", "BSW-MODULE-DEPENDENCYS", "BEHAVIOR", "TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "EVENT-SOURCE-REF", "SO-AD-ROUTING-GROUP", "REFERENCED-TD-EVENT-VFB-REF", "COMMITTED-BURST-SIZE", "NM-COORD-SYNC-SUPPORT", "COUPLING-PORTS", "MAX-SUPPORTED-DID-LENGTH", "COM-FIELD-GRANT-DESIGN", "CONTAINED-PDU-TRIGGERING-REFS", "PERSISTENT-STORAGE", "RECEIVER-IREF", "ECU-INSTANCE-PROPS", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "SW-FEATURE-DEFS", "MODE-DECLARATION-REF", "XREF-TARGET", "LOG-TRACE-DEFAULT-LOG-LEVEL", "DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE", "LOG-TRACE-PROCESS-ID", "OP", "CAN-COMMUNICATION-CONTROLLER-VARIANTS", "I-SIGNAL-I-PDU-GROUP", "RELEASED-TRIGGER-REF", "DIAGNOSTIC-DATA-IDENTIFIER-REF", "OPTION", "I-SIGNAL-TO-PDU-MAPPINGS", "HARDWARE-ELEMENT-REFS", "ITEM-DEFINITIONS", "DESTINATION-REFS", "NM-CONTROL-BIT-VECTOR-ENABLED", "ACF-PARTS", "BSW-INTERNAL-BEHAVIOR", "FLOW-CONTROL-PDU-REFS", "ENABLE-READ-ACCESS-CONTROL", "CHECKPOINT-FINAL-REF", "EXTERN-RATE-CORRECTION", "NM-VOTE-INFORMATION", "CONTEXTS", "REPORTABLE-SECURITY-EVENT-REFS", "PHM-CHECKPOINT", "SUP", "TCP-TP-CONFIG", "IS-ACTIVATION-LINE-DEPENDENT", "SIGNATURE-SCHEME-REF", "USER-DATA-SUB-TLV", "NEXT-STATE-IREF", "GENERAL-PURPOSE-CONNECTION", "MEMORY-SECTIONS", "METHOD-RETURN-REFS", "COM-PROPS", "ENABLE-DISCOVERY-PROTECTION", "DIAGNOSTIC-COMMON-ELEMENT-REF", "PARAMETER-INSTANCE", "FLOW-METERINGS", "NOMINAL-EXECUTION-TIME", "META-DATA-ITEM-SET", "ARBITRATIONS", "FRAME-PORT", "SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL", "CLUSTERED-COMPONENT-IREFS", "EVENT-IREF", "HEALTH-CHANNEL-ARGUMENT-REFS", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "DESTINATION-URI-REF", "HW-TYPE", "REFERENCE-BASES", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "MAXIMUM-SEGMENT-LENGTH-RESPONSE", "VEHICLE-ROLLOUT-STEP", "STATIC-MEMORYS", "MASS-EXP", "PROVIEDE-DATA-REF", "ICMP-RULE", "EVENT-HANDLER-REFS", "DDS-METHOD-REPLY-TOPIC-REF", "SOMEIP-METHOD-DEPLOYMENT", "NUMERICAL-VALUE-VARIATION-POINT", "ST-MIN", "SOMEIP-TRANSFORMATION-PROPS", "BIT-NUMBER", "NM-PDU-RX-INDICATION-ENABLED", "TRIGGER-PORT-ANNOTATIONS", "SPECIFICATION-DOCUMENT-SCOPES", "FUNCTION-PROTOTYPE-EMITTER", "SOFTWARE-CLUSTER-DESIGN-DEPENDENCY", "TIMESTAMP-RX-ACCEPTANCE-WINDOW", "MINIMUM-RX-CONTAINER-QUEUE-SIZE", "PN-RESET-TIME", "SECURITY-EVENT-REFS", "TEAR-DOWN-ACTION-REFS", "REST-BOOLEAN-PROPERTY-DEF", "DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "WITH-AUTO", "SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS", "NM-MSG-CYCLE-TIME", "BGCOLOR", "IGNORE-ORDER-ALLOWED", "HISTORY-ORDER-DEPTH", "WAKE-UP-BY-CONTROLLER-SUPPORTED", "IMPLEMENTATION-REFS", "BY-PASS-POINT-IREF", "SW-RECORD-LAYOUT-V-FIX-VALUE", "DIAGNOSTIC-ROUTINE-CONTROL", "EXCESS-INFORMATION-RATE", "I-PDU-IDENTIFIER-TCP-REF", "TOPIC-DATA", "TIME-FR-IF", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "STREAM-GATE-REF", "USE-MAC-ADDRESS-FOR-IDENTIFICATION", "SCHEDULER-NAME-PREFIXS", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "CSE-CODE-FACTOR", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "DTC-SETTING-CLASS-REF", "CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF", "MODE-DECLARATION-MAPPING-SET", "ACCESSED-PARAMETER", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "BAUDRATE", "SDG-AGGREGATION-WITH-VARIATION", "CLIENT-PORT-FROM-CONNECTION-REQUEST", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "NAME", "FUNCTION", "KEY", "COM-FIND-SERVICE-GRANT", "INIT-VALUES", "INITIAL-MODE-REF", "ABSOLUTE", "SW-RECORD-LAYOUT", "SW-SYSCOND", "SD-CLIENT-CONFIG", "SW-VALUES-PHYS", "SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY", "ACL-OPERATION-REFS", "TCP-IP-NDP-PREFIX-LIST-SIZE", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING", "REST-NUMBER-PROPERTY-DEF", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "RPT-SERVICE-POINTS", "PERSISTENCY-KEY-VALUE-STORAGE", "IN-SCOPE", "SECURITY-EVENT-FILTER-CHAIN", "SD-CLIENT-CONFIG-REF", "RETURN-VALUE-PROVISION", "PROCESS-TO-MACHINE-MAPPING-SET", "VALID-BINDING-TIME", "SECURE-COM-PROPS-FOR-UDP-REF", "ENABLE-CONDITIONS", "BSW-RELEASED-TRIGGER-POLICY", "MACHINE-DESIGN-REF", "MAX-CYCLE-REPETITIONS", "J-1939-NM-CLUSTER", "SECTION-TYPE", "ECUC-STRING-PARAM-DEF", "SW-DATA-DEF-PROPS-VARIANTS", "PROCESSOR", "K-INIT-RESOURCE", "NETWORK-REPRESENTATION", "SWC-TO-ECU-MAPPING", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "MAX-RPT-EVENT-ID", "UDS-SERVICE", "SA-RAND-TIME", "APPROVAL-STATUS", "TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED", "IP-V-6-ADDRESS", "SPEED", "PROCESS-STATE-MACHINE", "DYNAMIC-SLOT-IDLE-PHASE", "LOCAL-PARAMETER-REF", "MEMORY-HIGH-ADDRESS-LABEL", "TRANSFERS", "DDS-CP-I-SIGNAL-TO-DDS-TOPIC-MAPPING", "UDS-DTC-NUMBER", "INTRODUCTION", "DEFAULT-ENDIANNESS", "DIAGNOSTIC-EVENT-INFO-NEEDS", "WAKEUP-RX-WINDOW", "TIMING-CONDITION", "EVENT-GROUPS", "DOC-REVISIONS", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "MAX-SDU-SIZE", "PNC-WAKEUP-CAN-ID-EXTENDED", "DDS-PROVIDED-SERVICE-INSTANCE", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "INHIBITION-SOURCE-REF", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION", "SWC-FLAT-SERVICE-DEPENDENCY-REF", "EXCLUSIVE-AREA-NESTING-ORDERS", "FLAT-MAP-ENTRY-REF", "SW-AXIS-INDEX", "ROUTINE-REF", "NM-READY-SLEEP-COUNT", "END-2-END-METHOD-PROTECTION-PROPS", "IP-V-6-PATH-MTU-TIMEOUT", "CAN-IDENTIFIER-MASK", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "ALIVE-SUPERVISIONS", "TRIGGER-REF-CONDITIONAL", "COMMUNICATION-CONTROLLERS", "KEY-IDENTITY-REF", "MAX-VALUE", "COMPOSITION-SW-COMPONENT-TYPE", "FIELD-REF", "CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING", "USED-PIM-REF", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "COMPOSITE-NETWORK-REPRESENTATIONS", "SYSTEM-VERSION", "AUTOSAR-VARIABLE-IN-IMPL-DATATYPE", "ECUC-CHOICE-REFERENCE-DEF", "ARRAY-VALUE-SPECIFICATION", "SW-FEATURE-DEF", "R-PORT-PROTOTYPE", "USED-IMPLEMENTATION-DATA-TYPE-REF", "CRYPTO-KEY-MANAGEMENT-NEEDS", "PARAMETER-PORT-ANNOTATIONS", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "UPDATE-INDICATION-BIT-POSITION", "STORE-EVENT-SUPPORT", "SYNCHRONIZED-MASTER-TIME-BASE", "UNUSED-BIT-PATTERN", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "ACCEPT-ENCODING", "ASSIGN-FRAME-ID", "MIN-HEADER-ID", "MSR-QUERY-ARG", "TIMING-MODE-INSTANCE", "OBD-SUPPORT", "PWM-S", "PHYSICAL-REQUEST-REF", "COUNTER-INCREMENT-STEP-SIZE", "REMARK", "IDSM-CONTEXT-PROVIDER-INTERFACE", "CLIENT-CAPABILITY", "SW-SYSTEMCONSTANT-VALUE-SET", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF", "DEVELOPMENT-ERROR", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP", "CLUSTERED-COMPONENT-IREF", "DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL", "PRIMITIVE-ATTRIBUTE-TAILORING", "TRCV-PWM-MODE-ENABLED", "RATE-POLICYS", "FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL", "DATA-TRANSFORMATION-KIND", "IIDC-SY", "STATE-MANAGEMENT-REQUEST-RULE", "MEASURED-HEAP-USAGE", "UNIT-REFS", "START-INDEX", "ACTION-LIST-ITEM-REF", "RAW-DATA-STREAM-INTERFACE", "SUB-SDG-REF", "COMM-CONTROLLER-MAPPINGS", "MAX-BUFFER-SIZE", "SUBNET-NUMBER", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "RPT-SERVICE-POINT-POST-REFS", "PLATFORM-TIME-BASES", "CONTEXT-ATOMIC-COMPONENT-REF", "TRIGGER-INTERFACE", "PRODUCT-RELEASE", "STATE-MANAGEMENT-EM-ERROR-INTERFACE", "DURABILITY-SERVICE", "TLV-DATA-ID-DEFINITION", "NV-DATA-PORT-ANNOTATIONS", "MC-FUNCTION", "I-PDU-PORT", "REFERENCE-CONDITION", "IMPLICIT-DATA-ACCESS-IREFS", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS", "APPLICATION-ENDPOINT", "DLT-APPLICATION", "TX-NM-PDU-REF", "CAN-XL-FRAME-TRIGGERING-PROPS", "BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING", "DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT", "PARENT-CATEGORY", "REST-INTEGER-PROPERTY-DEF", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "CHECKPOINT-REF", "DOIP-INTERFACES", "RX-NM-PDU-REFS", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "WAKEUP-LOCAL-ENABLED", "NM-PDU", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "RECEIVER-IREFS", "FM-FEATURE-RESTRICTION", "BSW-CLIENT-POLICY", "OPTION-ARGUMENT", "NV-BLOCK-NEEDS", "TRANSIT-TO-INVALID-EXTENDED", "VIEW-MAP", "GRAPHIC", "NM-RANGE-CONFIG", "REST-ENDPOINT-ARGUMENT", "MODE-TRANSITIONS", "DO-IP-TARGET-ADDRESS-REFS", "ASSIGNED-LIN-SLAVE-CONFIG-REF", "PRIORITYS", "DIAGNOSTIC-INDICATOR-NEEDS", "STOP-STATE-MACHINE-REF", "MEMORY-LOW-ADDRESS", "PHM-LOGICAL-EXPRESSION", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "FORMAL-BLUEPRINT-GENERATOR", "PAYLOAD-LENGTH-STATIC", "UPPER-REF", "GID", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "SW-COMPARISON-VARIABLES", "COUPLING-PORT-FIFO", "FLOW-LABEL", "REMOTE-UNICAST-ADDRESSS", "APPL-CONSTANT-REF", "RECEIVED-DATA-REF", "RESPONSE-ON-EVENT-ACTION", "SERVICE-RESOURCE-REF", "INITIAL-CHECKPOINT-REF", "ALLOWED-TCP-OPTION", "MINIMUM-NUMBER-OF-REQUESTS", "DIAGNOSTIC-SOVD-METHOD-PRIMITIVE", "SYSTEM-SIGNAL-GROUP-REF", "TCP-IP-IP-REASSEMBLY-BUFFER-COUNT", "ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL", "I-SIGNAL-I-PDU-REF", "SERVICE-INTERFACE-TREF", "MEMORY-SECTION-LOCATIONS", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "SERVICE-POINT-SYMBOL-PRE", "COUPLING-PORT-STRUCTURAL-ELEMENTS", "POSITIVE-INTEGER-VALUE-VARIATION-POINT", "REPORTING-PORT-PROTOTYPE-IREF", "FIREWALL-RULE-PROPSS", "FILE-STORAGE-REF", "TEST-RESULT-REF", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "NM-WAIT-BUS-SLEEP-TIME", "PERIODIC-RATES", "RPT-HOOKS", "BSW-RESOURCE-ESTIMATION", "LIFE-CYCLE-INFO", "OFFSET-CORRECTION-JUMP-THRESHOLD", "TRACED-FAILURES", "NM-IMMEDIATE-NM-TRANSMISSIONS", "SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING", "DEFAULT-VALUE-ELEMENT", "DISPLAY-PRESENTATION", "ACTION", "SECURITY-EVENT-CONTEXT-PROPS", "TARGET-P-PORT-REF", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "FRAME-SYNC-ENABLED", "ENHANCED-MODE-API", "IUMPR-GROUP-IDENTIFIERS", "RPT-HOOK-ACCESS", "INITIAL-EVENT-STATUS", "TARGET-DATA-ELEMENT-REF", "BY-PASS-POINT-IREFS", "CRYPTO-KEY-SLOT-REFS", "ALLOWED-I-PV-6-EXT-HEADER", "LOWER-LIMIT", "TIMING-ARGUMENT-REF"];

    /// derive an enum entry from an input string using a perfect hash function
    ///
    /// # Errors
    ///
    /// [`ParseElementNameError`]: The input string did not match the name of any enum item
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseElementNameError> {
        static DISPLACEMENTS: [(u16, u16); 898] = [(1, 2474), (0, 269), (0, 1006), (0, 6), (0, 1257), (0, 31), (0, 50), (0, 17), (0, 1111), (0, 127), (1, 1310), (0, 87), (0, 4407), (0, 3), (0, 49), (0, 3), (9, 2931), (0, 177), (0, 245), (3, 810), (0, 247), (1, 3711), (0, 3), (8, 2545), (0, 2465), (0, 10), (0, 5188), (0, 13), (0, 21), (0, 5), (6, 3965), (0, 1531), (0, 4603), (2, 1669), (2, 3020), (0, 8), (0, 3), (3, 884), (0, 2), (0, 2927), (1, 3515), (0, 1), (0, 1338), (0, 1064), (0, 360), (0, 127), (0, 3297), (3, 5592), (0, 0), (0, 98), (4, 2935), (0, 1492), (0, 0), (0, 861), (0, 185), (4, 2663), (0, 1291), (0, 29), (0, 32), (0, 71), (0, 10), (0, 21), (0, 204), (0, 97), (1, 4538), (0, 97), (0, 1), (0, 486), (0, 867), (0, 2098), (0, 0), (0, 2932), (5, 2726), (0, 0), (3, 1655), (6, 2940), (17, 5672), (0, 1162), (0, 879), (0, 55), (0, 81), (0, 57), (0, 3700), (1, 4719), (2, 64), (0, 1), (0, 8), (0, 0), (0, 2), (0, 43), (0, 15), (0, 3245), (4, 1391), (0, 3170), (0, 160), (0, 7), (0, 2372), (4, 1747), (0, 249), (0, 194), (0, 203), (0, 764), (14, 2653), (0, 569), (0, 12), (0, 48), (0, 3), (0, 1818), (0, 181), (0, 1), (0, 6), (0, 3608), (0, 11), (0, 168), (0, 9), (0, 51), (0, 26), (0, 2178), (0, 45), (0, 69), (0, 3698), (1, 1941), (0, 104), (0, 3745), (0, 71), (0, 0), (0, 2), (0, 103), (0, 995), (0, 149), (0, 2530), (0, 10), (0, 365), (0, 265), (0, 1), (0, 3458), (0, 0), (5, 4161), (0, 4881), (1, 1995), (0, 455), (2, 3925), (0, 3), (0, 488), (0, 9), (2, 2352), (0, 1630), (0, 18), (0, 17), (0, 30), (0, 12), (4, 5565), (0, 131), (0, 51), (0, 132), (0, 4597), (0, 16), (0, 3125), (0, 56), (0, 1), (1, 2848), (1, 3403), (0, 12), (0, 103), (0, 47), (0, 114), (0, 3), (0, 215), (0, 439), (8, 2339), (0, 146), (0, 973), (0, 36), (0, 6125), (0, 253), (12, 4364), (0, 524), (1, 501), (0, 1949), (1, 727), (2, 3905), (3, 5813), (27, 4265), (0, 159), (0, 412), (0, 590), (0, 21), (0, 40), (0, 1), (1, 1468), (0, 4733), (3, 2276), (0, 361), (1, 3538), (0, 28), (0, 34), (1, 3171), (0, 6119), (1, 898), (0, 1615), (0, 2854), (0, 4528), (0, 2726), (7, 5413), (0, 0), (2, 2783), (0, 76), (0, 1677), (0, 539), (3, 1601), (2, 2065), (0, 13), (15, 5817), (6, 1648), (0, 3), (9, 2272), (0, 5), (0, 236), (0, 3), (0, 0), (0, 0), (0, 180), (0, 2206), (0, 423), (0, 11), (0, 32), (0, 194), (11, 3049), (0, 60), (0, 110), (0, 11), (0, 13), (0, 58), (5, 414), (0, 679), (0, 142), (1, 319), (0, 2), (1, 2), (0, 3452), (0, 15), (0, 238), (0, 1543), (0, 54), (0, 4711), (0, 404), (0, 1), (0, 0), (1, 480), (0, 403), (7, 2449), (0, 164), (0, 3), (0, 5), (3, 5386), (0, 0), (4, 3771), (19, 187), (0, 5498), (3, 507), (14, 3199), (0, 29), (0, 422), (0, 6038), (0, 444), (0, 553), (0, 3192), (0, 16), (0, 41), (0, 4), (0, 15), (0, 29), (0, 219), (0, 500), (0, 39), (0, 6), (3, 3041), (0, 130), (0, 960), (0, 491), (0, 0), (1, 16), (0, 9), (0, 20), (0, 1144), (0, 2), (0, 0), (3, 2538), (0, 4), (0, 35), (1, 4719), (1, 6113), (0, 5988), (0, 193), (18, 1939), (0, 0), (0, 0), (0, 4), (0, 3), (0, 11), (0, 66), (0, 8), (0, 8), (6, 3394), (0, 1100), (3, 47), (0, 13), (0, 6), (0, 2), (0, 356), (22, 2663), (1, 1435), (11, 4216), (0, 0), (0, 61), (0, 1029), (0, 156), (0, 1413), (0, 93), (0, 1042), (0, 230), (0, 12), (0, 177), (0, 0), (3, 6184), (0, 5), (1, 1725), (0, 15), (0, 682), (0, 63), (1, 5358), (1, 34), (0, 0), (1, 69), (0, 73), (0, 2724), (0, 203), (0, 2529), (3, 5756), (0, 1), (0, 25), (1, 1375), (0, 290), (1, 5717), (0, 2744), (0, 5219), (0, 3), (1, 4932), (0, 1632), (0, 177), (0, 354), (6, 1044), (0, 292), (0, 6), (0, 19), (0, 1530), (0, 567), (0, 0), (0, 2), (0, 0), (19, 2916), (0, 1110), (0, 42), (0, 52), (0, 0), (1, 148), (0, 41), (0, 61), (1, 2319), (0, 10), (0, 1), (0, 0), (0, 77), (0, 1583), (2, 2172), (0, 1603), (1, 2070), (0, 3151), (0, 9), (0, 51), (0, 310), (0, 3), (0, 366), (18, 5169), (0, 101), (0, 4469), (0, 0), (0, 3), (0, 3), (0, 44), (0, 115), (8, 3352), (0, 470), (1, 12), (0, 1037), (11, 3489), (0, 124), (0, 48), (0, 381), (0, 76), (0, 3553), (1, 1818), (0, 5), (3, 5542), (0, 6205), (0, 33), (0, 64), (0, 58), (0, 5113), (0, 641), (0, 1208), (0, 87), (0, 135), (0, 0), (0, 76), (0, 544), (0, 9), (0, 20), (4, 5651), (3, 2249), (0, 6), (0, 154), (0, 0), (0, 2034), (46, 2420), (0, 135), (0, 113), (0, 257), (3, 5441), (1, 62), (0, 100), (0, 120), (1, 110), (0, 3), (0, 2), (0, 2774), (0, 106), (5, 708), (1, 2731), (0, 1033), (1, 779), (4, 5042), (0, 133), (3, 581), (0, 85), (0, 5), (0, 346), (3, 6135), (0, 0), (0, 1889), (0, 131), (0, 3879), (0, 56), (4, 3960), (0, 1365), (16, 3604), (6, 4529), (0, 20), (0, 288), (0, 1088), (0, 144), (24, 3267), (0, 236), (3, 537), (11, 1775), (0, 492), (0, 0), (0, 243), (2, 3108), (5, 2382), (0, 594), (0, 332), (3, 4138), (2, 3033), (1, 3843), (6, 533), (0, 4098), (0, 2365), (0, 3209), (0, 1896), (1, 74), (4, 2747), (0, 1466), (3, 2104), (0, 375), (20, 319), (7, 1403), (0, 26), (5, 1109), (0, 19), (0, 2), (0, 446), (0, 519), (0, 1), (0, 49), (3, 124), (0, 52), (66, 3716), (0, 646), (0, 35), (0, 210), (17, 2471), (7, 4036), (0, 320), (19, 1447), (1, 373), (0, 5070), (0, 3284), (0, 0), (9, 794), (2, 4840), (0, 156), (3, 2517), (0, 744), (3, 6077), (1, 5791), (0, 2505), (0, 3), (0, 0), (8, 833), (0, 90), (0, 1082), (0, 535), (0, 9), (1, 1381), (0, 11), (0, 161), (18, 477), (16, 2353), (0, 69), (0, 19), (8, 624), (0, 14), (0, 1352), (2, 1787), (3, 417), (0, 8), (24, 4704), (0, 298), (97, 2547), (0, 1559), (0, 96), (0, 156), (0, 1007), (0, 3483), (0, 0), (0, 109), (0, 393), (0, 79), (0, 3775), (0, 291), (0, 1), (0, 2), (0, 447), (23, 3458), (0, 12), (1, 5999), (0, 38), (14, 2144), (0, 1), (3, 5186), (0, 2445), (0, 11), (53, 1317), (13, 2732), (0, 248), (0, 1567), (1, 4892), (0, 319), (0, 15), (6, 6108), (0, 265), (0, 7), (0, 2), (0, 334), (0, 3), (11, 92), (0, 621), (1, 1), (0, 5025), (3, 371), (0, 2764), (0, 10), (3, 2262), (0, 3), (12, 4770), (23, 6182), (0, 3656), (0, 10), (0, 792), (0, 476), (122, 4690), (14, 709), (0, 3604), (0, 13), (29, 4328), (15, 6077), (8, 5617), (37, 4737), (0, 49), (0, 562), (8, 3811), (22, 387), (102, 5111), (0, 0), (3, 338), (0, 648), (0, 2), (0, 90), (9, 1744), (1, 626), (0, 6), (0, 1488), (14, 1219), (0, 1296), (1, 782), (3, 3979), (15, 2318), (0, 3), (48, 134), (0, 961), (0, 656), (0, 40), (98, 5162), (24, 3669), (0, 35), (16, 122), (11, 3975), (0, 139), (0, 1407), (0, 3639), (0, 4101), (0, 62), (24, 122), (7, 2937), (0, 426), (0, 1), (0, 1404), (40, 3367), (0, 20), (0, 807), (0, 125), (4, 2412), (0, 5), (0, 1004), (12, 5223), (0, 4), (0, 1), (4, 1066), (1, 0), (13, 2636), (0, 54), (2, 995), (0, 1691), (0, 1), (0, 9), (4, 1509), (0, 85), (0, 3099), (9, 1448), (0, 786), (0, 0), (0, 2), (44, 813), (0, 23), (0, 5), (0, 4964), (10, 5544), (13, 4044), (9, 4724), (0, 43), (0, 10), (0, 22), (1, 1124), (1, 71), (0, 0), (0, 13), (0, 5), (0, 70), (1, 450), (0, 861), (39, 1461), (11, 2803), (0, 36), (0, 5341), (0, 20), (4, 5002), (0, 659), (5, 2947), (2, 889), (109, 1804), (26, 2979), (23, 3607), (162, 5749), (0, 0), (0, 0), (0, 2842), (0, 50), (0, 11), (0, 1250), (9, 3295), (0, 3487), (34, 5081), (0, 111), (0, 27), (0, 37), (46, 312), (0, 384), (54, 2025), (0, 22), (0, 0), (0, 407), (1, 4933), (5, 1685), (2, 3221), (0, 21), (2, 1541), (0, 1122), (0, 0), (0, 1795), (17, 995), (7, 4737), (0, 60), (1, 2022), (0, 78), (2, 3653), (0, 27), (1, 3511), (0, 120), (0, 5), (6, 2635), (0, 833), (0, 48), (0, 162), (3, 4845), (0, 0), (0, 138), (163, 1356), (0, 2649), (0, 2665), (0, 5), (0, 598), (0, 4160), (6, 5844), (354, 4776), (0, 119), (132, 5947), (1, 2338), (0, 102), (22, 5863), (27, 3605), (3, 5832), (3, 3041), (0, 2070), (4, 715), (0, 1122), (0, 396), (0, 67), (0, 5), (0, 5), (0, 1273), (13, 393), (0, 208), (9, 2142), (0, 23), (0, 0), (1, 720), (0, 50), (0, 3), (0, 5947), (88, 131), (0, 56), (64, 1272), (1, 1122), (37, 3995), (0, 217), (0, 1), (0, 3533), (0, 1468), (5, 4363), (1, 2030), (0, 1318), (0, 58), (0, 4), (0, 3194), (0, 1858), (60, 3082), (0, 2466), (0, 5771), (7, 1406), (1, 5), (3, 812), (22, 2327), (0, 160), (0, 307), (0, 2), (0, 2), (1, 2126), (1, 2974), (0, 248), (3, 2933), (310, 3809), (1, 2518), (1, 1128), (1, 0), (16, 1603), (0, 6123), (0, 11), (9, 5194), (0, 3), (0, 4434), (80, 4687), (0, 76), (0, 12), (11, 2050), (26, 4250), (0, 60), (45, 1747), (0, 44), (0, 1735), (0, 36), (0, 19), (0, 47), (0, 0), (0, 206), (1, 177), (25, 3636), (0, 2238), (123, 2288), (1, 3066), (0, 0), (0, 7), (0, 497), (0, 97), (86, 6074), (0, 1389), (105, 1856), (0, 249), (12, 1720), (0, 7), (0, 0), (0, 39), (106, 1972), (0, 13), (67, 666), (70, 2880), (15, 4299), (0, 118), (13, 2059), (642, 1022), (0, 5), (0, 101), (18, 2416), (0, 890), (0, 375), (232, 341), (11, 6214), (0, 25), (35, 3874), (0, 5), (32, 5744), (8, 2345), (2, 3003), (0, 3), (0, 221), (1, 730), (21, 2920), (0, 216), (0, 8), (0, 0), (0, 4), (0, 825), (0, 779), (0, 1075), (1, 4951), (2, 934), (0, 45), (0, 5635), (6, 366), (0, 5), (0, 33), (0, 3906), (16, 5654), (6, 1922), (35, 1261), (0, 2637), (7, 1544), (24, 1042), (0, 3617), (180, 3566), (0, 1), (22, 4074), (0, 89), (378, 4027), (43, 902), (0, 193), (138, 1217), (6, 6071)];
        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 898) as usize];
        let item_idx = u32::from(d2).wrapping_add(f1.wrapping_mul(u32::from(d1))).wrapping_add(f2) as usize % 6284;
        if ElementName::STRING_TABLE[item_idx].as_bytes() != input {
            return Err(ParseElementNameError);
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
        ElementName::STRING_TABLE[*self as usize]
    }
}

impl std::str::FromStr for ElementName {
    type Err = ParseElementNameError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(input.as_bytes())
    }
}

impl std::fmt::Debug for ElementName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(ElementName::STRING_TABLE[*self as usize])
    }
}

impl std::fmt::Display for ElementName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(ElementName::STRING_TABLE[*self as usize])
    }
}

