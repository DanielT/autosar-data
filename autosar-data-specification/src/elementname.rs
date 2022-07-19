use crate::hashfunc;

#[derive(Debug)]
/// The error type ParseElementNameError is returned when from_str() / parse() fails for ElementName
pub struct ParseElementNameError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
/// Enum of all element names in Autosar
pub enum ElementName {
    /// ABBR-NAME
    AbbrName                                                               = 948,
    /// ABS
    Abs                                                                    = 1008,
    /// ABSOLUTE
    Absolute                                                               = 3326,
    /// ABSOLUTE-TOLERANCE
    AbsoluteTolerance                                                      = 1961,
    /// ABSOLUTELY-SCHEDULED-TIMINGS
    AbsolutelyScheduledTimings                                             = 268,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 1421,
    /// ACCEPT-ENCODING
    AcceptEncoding                                                         = 2835,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF
    AcceptedCryptoCipherSuiteWithPskRef                                    = 1206,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS
    AcceptedCryptoCipherSuiteWithPskRefs                                   = 5172,
    /// ACCEPTED-REMOTE-CERTIFICATE-REF
    AcceptedRemoteCertificateRef                                           = 3529,
    /// ACCEPTED-REMOTE-CERTIFICATE-REFS
    AcceptedRemoteCertificateRefs                                          = 4024,
    /// ACCEPTED-STARTUP-RANGE
    AcceptedStartupRange                                                   = 2899,
    /// ACCEPTS-ENCODINGS
    AcceptsEncodings                                                       = 1812,
    /// ACCESS-CONTROL
    AccessControl                                                          = 348,
    /// ACCESS-COUNT
    AccessCount                                                            = 91,
    /// ACCESS-COUNT-SET
    AccessCountSet                                                         = 3143,
    /// ACCESS-COUNT-SETS
    AccessCountSets                                                        = 1620,
    /// ACCESS-COUNTS
    AccessCounts                                                           = 1919,
    /// ACCESS-DATA-RECORD-SIZE
    AccessDataRecordSize                                                   = 5379,
    /// ACCESS-PERMISSION-REF
    AccessPermissionRef                                                    = 4824,
    /// ACCESS-PERMISSION-VALIDITY
    AccessPermissionValidity                                               = 5511,
    /// ACCESS-POINT-REF
    AccessPointRef                                                         = 3176,
    /// ACCESSED-MODE-GROUPS
    AccessedModeGroups                                                     = 4050,
    /// ACCESSED-PARAMETER
    AccessedParameter                                                      = 130,
    /// ACCESSED-VARIABLE
    AccessedVariable                                                       = 3917,
    /// ACCESSED-VARIABLE-REF
    AccessedVariableRef                                                    = 1477,
    /// ACK-REQUEST
    AckRequest                                                             = 3869,
    /// ACK-TYPE
    AckType                                                                = 1904,
    /// ACL-CONTEXT
    AclContext                                                             = 2812,
    /// ACL-CONTEXTS
    AclContexts                                                            = 378,
    /// ACL-OBJECT-CLASS
    AclObjectClass                                                         = 497,
    /// ACL-OBJECT-CLASSS
    AclObjectClasss                                                        = 424,
    /// ACL-OBJECT-REF
    AclObjectRef                                                           = 5197,
    /// ACL-OBJECT-REFS
    AclObjectRefs                                                          = 2830,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 809,
    /// ACL-OPERATION
    AclOperation                                                           = 4406,
    /// ACL-OPERATION-REF
    AclOperationRef                                                        = 1093,
    /// ACL-OPERATION-REFS
    AclOperationRefs                                                       = 761,
    /// ACL-PERMISSION
    AclPermission                                                          = 5216,
    /// ACL-ROLE
    AclRole                                                                = 5298,
    /// ACL-ROLE-REF
    AclRoleRef                                                             = 4775,
    /// ACL-ROLE-REFS
    AclRoleRefs                                                            = 1451,
    /// ACL-SCOPE
    AclScope                                                               = 3961,
    /// ACTION
    Action                                                                 = 4160,
    /// ACTION-LIST
    ActionList                                                             = 1377,
    /// ACTION-LIST-EXECUTION
    ActionListExecution                                                    = 3911,
    /// ACTION-LIST-ITEM-REF
    ActionListItemRef                                                      = 2478,
    /// ACTION-LIST-ITEM-REFS
    ActionListItemRefs                                                     = 3685,
    /// ACTION-LISTS
    ActionLists                                                            = 2186,
    /// ACTION-POINT-OFFSET
    ActionPointOffset                                                      = 5349,
    /// ACTION-TYPE
    ActionType                                                             = 2330,
    /// ACTIONS
    Actions                                                                = 2549,
    /// ACTIVATE-AT-START
    ActivateAtStart                                                        = 1705,
    /// ACTIVATION
    Activation                                                             = 5181,
    /// ACTIVATION-ACTION
    ActivationAction                                                       = 69,
    /// ACTIVATION-POINTS
    ActivationPoints                                                       = 1072,
    /// ACTIVATION-REASON-REPRESENTATION-REF
    ActivationReasonRepresentationRef                                      = 681,
    /// ACTIVATION-REASONS
    ActivationReasons                                                      = 3948,
    /// ACTIVATION-SWITCH
    ActivationSwitch                                                       = 4068,
    /// ACTIVE-SUPERVISION-REF
    ActiveSupervisionRef                                                   = 3022,
    /// ACTIVE-SUPERVISION-REFS
    ActiveSupervisionRefs                                                  = 242,
    /// ACTUAL-EVENT-REF
    ActualEventRef                                                         = 1386,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 1810,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 2181,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 5276,
    /// ADD-INFO-5
    AddInfo5                                                               = 3019,
    /// ADDITIONAL-INFORMATION
    AdditionalInformation                                                  = 2356,
    /// ADDITIONAL-NATIVE-TYPE-QUALIFIER
    AdditionalNativeTypeQualifier                                          = 5603,
    /// ADDRESS
    Address                                                                = 986,
    /// ADDRESS-CLAIM-ENABLED
    AddressClaimEnabled                                                    = 971,
    /// ADDRESS-RANGE-LOWER-BOUND
    AddressRangeLowerBound                                                 = 1484,
    /// ADDRESS-RANGE-UPPER-BOUND
    AddressRangeUpperBound                                                 = 3927,
    /// ADDRESS-SEMANTICS
    AddressSemantics                                                       = 2263,
    /// ADDRESSING-FORMAT
    AddressingFormat                                                       = 3106,
    /// ADMIN-DATA
    AdminData                                                              = 5187,
    /// ADMIT-WITHOUT-MESSAGE-ID
    AdmitWithoutMessageId                                                  = 4396,
    /// AFFECTED-APPLICATION
    AffectedApplication                                                    = 3277,
    /// AFFECTED-BSW-MODULE
    AffectedBswModule                                                      = 4777,
    /// AFFECTED-FUNCTIONAL-CLUSTER
    AffectedFunctionalCluster                                              = 3791,
    /// AFFECTED-REF
    AffectedRef                                                            = 81,
    /// AFFECTED-REFS
    AffectedRefs                                                           = 4731,
    /// AFFECTION-KIND
    AffectionKind                                                          = 2116,
    /// AGE
    Age                                                                    = 3638,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 3533,
    /// AGGREGATION
    Aggregation                                                            = 5360,
    /// AGGREGATION-CONDITION
    AggregationCondition                                                   = 649,
    /// AGGREGATION-REF
    AggregationRef                                                         = 2610,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 1829,
    /// AGING-ALLOWED
    AgingAllowed                                                           = 4180,
    /// AGING-CYCLES
    AgingCycles                                                            = 2474,
    /// AGING-REF
    AgingRef                                                               = 1105,
    /// AGING-REQUIRES-TESTED-CYCLE
    AgingRequiresTestedCycle                                               = 2487,
    /// AH-CIPHER-SUITE-NAME
    AhCipherSuiteName                                                      = 2273,
    /// AH-CIPHER-SUITE-NAMES
    AhCipherSuiteNames                                                     = 1136,
    /// ALGORITHM-FAMILY
    AlgorithmFamily                                                        = 817,
    /// ALGORITHM-MODE
    AlgorithmMode                                                          = 107,
    /// ALGORITHM-SECONDARY-FAMILY
    AlgorithmSecondaryFamily                                               = 1452,
    /// ALIAS-EVENT-REF
    AliasEventRef                                                          = 1065,
    /// ALIAS-NAME-ASSIGNMENT
    AliasNameAssignment                                                    = 1467,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 2147,
    /// ALIAS-NAMES
    AliasNames                                                             = 4714,
    /// ALIGNMENT
    Alignment                                                              = 858,
    /// ALIVE-CHECK-RESPONSE-TIMEOUT
    AliveCheckResponseTimeout                                              = 1555,
    /// ALIVE-REFERENCE-CYCLE
    AliveReferenceCycle                                                    = 1261,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 4039,
    /// ALIVE-SUPERVISIONS
    AliveSupervisions                                                      = 697,
    /// ALIVE-TIMEOUT
    AliveTimeout                                                           = 3468,
    /// ALL-CHANNELS-REF
    AllChannelsRef                                                         = 3374,
    /// ALL-CHANNELS-REFS
    AllChannelsRefs                                                        = 1862,
    /// ALL-NM-MESSAGES-KEEP-AWAKE
    AllNmMessagesKeepAwake                                                 = 4821,
    /// ALLOCATE-SHADOW-COPY
    AllocateShadowCopy                                                     = 4920,
    /// ALLOCATOR
    Allocator                                                              = 2578,
    /// ALLOCATOR-REF
    AllocatorRef                                                           = 2742,
    /// ALLOW-CONTENT-TYPE-CHANGE
    AllowContentTypeChange                                                 = 3715,
    /// ALLOW-DYNAMIC-L-SDU-LENGTH
    AllowDynamicLSduLength                                                 = 1166,
    /// ALLOW-HALT-DUE-TO-CLOCK
    AllowHaltDueToClock                                                    = 3910,
    /// ALLOW-MASTER-RATE-CORRECTION
    AllowMasterRateCorrection                                              = 1349,
    /// ALLOW-PASSIVE-TO-ACTIVE
    AllowPassiveToActive                                                   = 1813,
    /// ALLOW-PROVIDER-RATE-CORRECTION
    AllowProviderRateCorrection                                            = 2602,
    /// ALLOW-UNAUTHENTICATED-PARTICIPANTS
    AllowUnauthenticatedParticipants                                       = 4623,
    /// ALLOWED-I-PV-6-EXT-HEADER
    AllowedIPv6ExtHeader                                                   = 3357,
    /// ALLOWED-I-PV-6-EXT-HEADERS
    AllowedIPv6ExtHeaders                                                  = 2051,
    /// ALLOWED-I-PV-6-EXT-HEADERS-REF
    AllowedIPv6ExtHeadersRef                                               = 1591,
    /// ALLOWED-KEYSLOT-USAGE
    AllowedKeyslotUsage                                                    = 5089,
    /// ALLOWED-TCP-OPTION
    AllowedTcpOption                                                       = 5286,
    /// ALLOWED-TCP-OPTIONS
    AllowedTcpOptions                                                      = 2787,
    /// ALLOWED-TCP-OPTIONS-REF
    AllowedTcpOptionsRef                                                   = 4676,
    /// ALTERNATIVE-NAME
    AlternativeName                                                        = 3228,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 618,
    /// ANNOTATION
    Annotation                                                             = 3541,
    /// ANNOTATION-ORIGIN
    AnnotationOrigin                                                       = 4792,
    /// ANNOTATION-TEXT
    AnnotationText                                                         = 5275,
    /// ANNOTATIONS
    Annotations                                                            = 909,
    /// ANONYMOUS
    Anonymous                                                              = 2340,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 4245,
    /// AP-APPLICATION-ENDPOINTS
    ApApplicationEndpoints                                                 = 2439,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 3454,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 664,
    /// AP-APPLICATION-ERROR-REF
    ApApplicationErrorRef                                                  = 4874,
    /// AP-APPLICATION-ERROR-REFS
    ApApplicationErrorRefs                                                 = 2843,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 2311,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 1186,
    /// API-PRINCIPLE
    ApiPrinciple                                                           = 5395,
    /// API-SERVICE-PREFIX
    ApiServicePrefix                                                       = 1858,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 5438,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS
    AppOsTaskProxyToEcuTaskProxyMappings                                   = 5084,
    /// APP-TASK-PROXY-REF
    AppTaskProxyRef                                                        = 5406,
    /// APPL-CONSTANT-REF
    ApplConstantRef                                                        = 576,
    /// APPL-INIT-VALUE
    ApplInitValue                                                          = 2055,
    /// APPL-WATCHDOG-LIMIT
    ApplWatchdogLimit                                                      = 3989,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 2361,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 3004,
    /// APPLICATION-ARRAY-ELEMENT-REF
    ApplicationArrayElementRef                                             = 1445,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 5302,
    /// APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION
    ApplicationAssocMapElementValueSpecification                           = 1688,
    /// APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION
    ApplicationAssocMapValueSpecification                                  = 883,
    /// APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF
    ApplicationCompositeDataTypeSubElementRef                              = 922,
    /// APPLICATION-COMPOSITE-ELEMENT-IREF
    ApplicationCompositeElementIref                                        = 667,
    /// APPLICATION-CONTEXT-REF
    ApplicationContextRef                                                  = 4419,
    /// APPLICATION-CONTEXT-REFS
    ApplicationContextRefs                                                 = 5607,
    /// APPLICATION-DATA-TYPE-REF
    ApplicationDataTypeRef                                                 = 4950,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 1854,
    /// APPLICATION-DESCRIPTION
    ApplicationDescription                                                 = 4437,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 1104,
    /// APPLICATION-ENDPOINT-REF
    ApplicationEndpointRef                                                 = 1473,
    /// APPLICATION-ENDPOINT-REF-CONDITIONAL
    ApplicationEndpointRefConditional                                      = 369,
    /// APPLICATION-ENTRY
    ApplicationEntry                                                       = 4907,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 3450,
    /// APPLICATION-ID
    ApplicationId                                                          = 4823,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 547,
    /// APPLICATION-MODE-IREF
    ApplicationModeIref                                                    = 441,
    /// APPLICATION-MODE-MACHINE
    ApplicationModeMachine                                                 = 3268,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 5640,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 3060,
    /// APPLICATION-PARTITION-REF
    ApplicationPartitionRef                                                = 1365,
    /// APPLICATION-PARTITION-REFS
    ApplicationPartitionRefs                                               = 482,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 420,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS
    ApplicationPartitionToEcuPartitionMappings                             = 3684,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 1158,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 1117,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 4021,
    /// APPLICATION-RECORD-ELEMENT-REF
    ApplicationRecordElementRef                                            = 3005,
    /// APPLICATION-RULE-BASED-VALUE-SPECIFICATION
    ApplicationRuleBasedValueSpecification                                 = 5608,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 1976,
    /// APPLICATION-TYPE-REF
    ApplicationTypeRef                                                     = 1050,
    /// APPLICATION-TYPE-TREF
    ApplicationTypeTref                                                    = 1984,
    /// APPLICATION-VALUE-SPECIFICATION
    ApplicationValueSpecification                                          = 4363,
    /// APPLICATIONS
    Applications                                                           = 5428,
    /// APPLIES-TO
    AppliesTo                                                              = 1899,
    /// APPLIES-TO-DEPENDENCIES
    AppliesToDependencies                                                  = 1005,
    /// APPROVAL-REQUIRED
    ApprovalRequired                                                       = 5486,
    /// APPROVAL-STATUS
    ApprovalStatus                                                         = 367,
    /// AR-PACKAGE
    ArPackage                                                              = 2823,
    /// AR-PACKAGES
    ArPackages                                                             = 5573,
    /// AR-PARAMETER
    ArParameter                                                            = 2437,
    /// AR-RELEASE-VERSION
    ArReleaseVersion                                                       = 315,
    /// AR-TYPED-PER-INSTANCE-MEMORY-REF
    ArTypedPerInstanceMemoryRef                                            = 5008,
    /// AR-TYPED-PER-INSTANCE-MEMORYS
    ArTypedPerInstanceMemorys                                              = 3115,
    /// ARBITRARY-ADDRESS-CAPABLE
    ArbitraryAddressCapable                                                = 4781,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 2177,
    /// ARBITRATION
    Arbitration                                                            = 4062,
    /// ARBITRATIONS
    Arbitrations                                                           = 1286,
    /// AREA
    Area                                                                   = 1865,
    /// ARGUMENT
    Argument                                                               = 3137,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 4683,
    /// ARGUMENT-MAPPINGS
    ArgumentMappings                                                       = 3976,
    /// ARGUMENT-REF
    ArgumentRef                                                            = 665,
    /// ARGUMENTS
    Arguments                                                              = 977,
    /// ARGUMENTSS
    Argumentss                                                             = 3788,
    /// ARP-PROPS
    ArpProps                                                               = 4721,
    /// ARRAY-ELEMENT-MAPPINGS
    ArrayElementMappings                                                   = 625,
    /// ARRAY-ELEMENT-REF
    ArrayElementRef                                                        = 4505,
    /// ARRAY-IMPL-POLICY
    ArrayImplPolicy                                                        = 3064,
    /// ARRAY-SIZE
    ArraySize                                                              = 100,
    /// ARRAY-SIZE-HANDLING
    ArraySizeHandling                                                      = 2077,
    /// ARRAY-SIZE-SEMANTICS
    ArraySizeSemantics                                                     = 5175,
    /// ARRAY-VALUE-SPECIFICATION
    ArrayValueSpecification                                                = 4332,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 3650,
    /// ARTIFACT-CHECKSUM-REF
    ArtifactChecksumRef                                                    = 2813,
    /// ARTIFACT-CHECKSUM-REFS
    ArtifactChecksumRefs                                                   = 1613,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 5462,
    /// ARTIFACT-CHECKSUMS
    ArtifactChecksums                                                      = 2258,
    /// ARTIFACT-DESCRIPTOR
    ArtifactDescriptor                                                     = 5238,
    /// ARTIFACT-DESCRIPTORS
    ArtifactDescriptors                                                    = 2637,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 2172,
    /// ASSERTIONS
    Assertions                                                             = 3896,
    /// ASSIGN-FRAME-ID
    AssignFrameId                                                          = 3177,
    /// ASSIGN-FRAME-ID-RANGE
    AssignFrameIdRange                                                     = 3952,
    /// ASSIGN-NAD
    AssignNad                                                              = 2848,
    /// ASSIGNED-CONTROLLER-REF
    AssignedControllerRef                                                  = 4266,
    /// ASSIGNED-DATA-TYPES
    AssignedDataTypes                                                      = 1622,
    /// ASSIGNED-DATAS
    AssignedDatas                                                          = 492,
    /// ASSIGNED-ENTRY-REF
    AssignedEntryRef                                                       = 3798,
    /// ASSIGNED-ENTRY-ROLES
    AssignedEntryRoles                                                     = 5046,
    /// ASSIGNED-FRAME-TRIGGERING-REF
    AssignedFrameTriggeringRef                                             = 4414,
    /// ASSIGNED-LIN-SLAVE-CONFIG-REF
    AssignedLinSlaveConfigRef                                              = 780,
    /// ASSIGNED-PORTS
    AssignedPorts                                                          = 895,
    /// ASSIGNED-TRAFFIC-CLASS
    AssignedTrafficClass                                                   = 1738,
    /// ASSIGNED-TRAFFIC-CLASSS
    AssignedTrafficClasss                                                  = 1118,
    /// ASSIGNMENT-PRIORITY
    AssignmentPriority                                                     = 4933,
    /// ASSOCIATED-COM-I-PDU-GROUP-REF
    AssociatedComIPduGroupRef                                              = 4658,
    /// ASSOCIATED-COM-I-PDU-GROUP-REFS
    AssociatedComIPduGroupRefs                                             = 429,
    /// ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    AssociatedConsumedProvidedServiceInstanceGroups                        = 475,
    /// ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF
    AssociatedCrossSwClusterComRtePluginRef                                = 5051,
    /// ASSOCIATED-EVENT-IDENTIFICATION
    AssociatedEventIdentification                                          = 129,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REF
    AssociatedPdurIPduGroupRef                                             = 2008,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REFS
    AssociatedPdurIPduGroupRefs                                            = 2201,
    /// ASSOCIATED-RTE-PLUGIN-REF
    AssociatedRtePluginRef                                                 = 2,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 5585,
    /// ASYNCHRONOUS-SERVER-CALL-POINT-REF
    AsynchronousServerCallPointRef                                         = 2040,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 5483,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS
    AsynchronousServerCallResultPoints                                     = 2827,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 1204,
    /// ATTRIBUTE-DEFS
    AttributeDefs                                                          = 1698,
    /// ATTRIBUTE-NAME
    AttributeName                                                          = 2767,
    /// ATTRIBUTE-REF
    AttributeRef                                                           = 3108,
    /// ATTRIBUTE-TAILORINGS
    AttributeTailorings                                                    = 3532,
    /// ATTRIBUTE-VALUES
    AttributeValues                                                        = 3966,
    /// ATTRIBUTES
    Attributes                                                             = 1615,
    /// AUDIENCE
    Audience                                                               = 1342,
    /// AUDIENCES
    Audiences                                                              = 5656,
    /// AUTH-ALGORITHM
    AuthAlgorithm                                                          = 2029,
    /// AUTH-DATA-FRESHNESS-LENGTH
    AuthDataFreshnessLength                                                = 1927,
    /// AUTH-DATA-FRESHNESS-START-POSITION
    AuthDataFreshnessStartPosition                                         = 3593,
    /// AUTH-INFO-TX-LENGTH
    AuthInfoTxLength                                                       = 4921,
    /// AUTHENTIC-CONNECTION-PROPSS
    AuthenticConnectionPropss                                              = 1554,
    /// AUTHENTICATION-BUILD-ATTEMPTS
    AuthenticationBuildAttempts                                            = 2710,
    /// AUTHENTICATION-CLASS-REF
    AuthenticationClassRef                                                 = 5571,
    /// AUTHENTICATION-PROPS-REF
    AuthenticationPropsRef                                                 = 5060,
    /// AUTHENTICATION-PROPSS
    AuthenticationPropss                                                   = 2187,
    /// AUTHENTICATION-REF
    AuthenticationRef                                                      = 51,
    /// AUTHENTICATION-RETRIES
    AuthenticationRetries                                                  = 5545,
    /// AUTHENTICATION-ROLE-REF
    AuthenticationRoleRef                                                  = 4371,
    /// AUTHENTICATION-ROLE-REFS
    AuthenticationRoleRefs                                                 = 2568,
    /// AUTHENTICATION-TIMEOUT
    AuthenticationTimeout                                                  = 5522,
    /// AUTO-AVAILABLE
    AutoAvailable                                                          = 3748,
    /// AUTO-COLLECT
    AutoCollect                                                            = 5157,
    /// AUTO-IP-PROPS
    AutoIpProps                                                            = 3656,
    /// AUTO-REQUIRE
    AutoRequire                                                            = 2589,
    /// AUTOMATIC-END
    AutomaticEnd                                                           = 5381,
    /// AUTOSAR
    Autosar                                                                = 4700,
    /// AUTOSAR-CONNECTOR
    AutosarConnector                                                       = 4949,
    /// AUTOSAR-DATA-TYPE-REF
    AutosarDataTypeRef                                                     = 3561,
    /// AUTOSAR-DATA-TYPE-REF-CONDITIONAL
    AutosarDataTypeRefConditional                                          = 3604,
    /// AUTOSAR-ENGINEERING-OBJECT
    AutosarEngineeringObject                                               = 4299,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 4699,
    /// AUTOSAR-PARAMETER-IREF
    AutosarParameterIref                                                   = 25,
    /// AUTOSAR-VARIABLE
    AutosarVariable                                                        = 1707,
    /// AUTOSAR-VARIABLE-IN-IMPL-DATATYPE
    AutosarVariableInImplDatatype                                          = 3658,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 4596,
    /// AUTOSAR-VARIABLE-IREF
    AutosarVariableIref                                                    = 1739,
    /// AUXILIARY-FIELD-DEFINITIONS
    AuxiliaryFieldDefinitions                                              = 1153,
    /// AUXILIARY-FIELDS
    AuxiliaryFields                                                        = 5263,
    /// AVERAGE-MEMORY-CONSUMPTION
    AverageMemoryConsumption                                               = 2956,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 3498,
    /// BANDWIDTH-LIMITATION
    BandwidthLimitation                                                    = 1307,
    /// BASE-COMPOSITION-REF
    BaseCompositionRef                                                     = 4565,
    /// BASE-CYCLE
    BaseCycle                                                              = 4679,
    /// BASE-IS-THIS-PACKAGE
    BaseIsThisPackage                                                      = 4200,
    /// BASE-REF
    BaseRef                                                                = 674,
    /// BASE-REFERENCE-REF
    BaseReferenceRef                                                       = 1185,
    /// BASE-TYPE-ENCODING
    BaseTypeEncoding                                                       = 4912,
    /// BASE-TYPE-REF
    BaseTypeRef                                                            = 4712,
    /// BASE-TYPE-SIZE
    BaseTypeSize                                                           = 1874,
    /// BASIC-CYCLE-LENGTH
    BasicCycleLength                                                       = 1597,
    /// BAUDRATE
    Baudrate                                                               = 4490,
    /// BEHAVIOR
    Behavior                                                               = 1650,
    /// BEHAVIOR-REF
    BehaviorRef                                                            = 2229,
    /// BEST-CASE-EXECUTION-TIME
    BestCaseExecutionTime                                                  = 241,
    /// BGCOLOR
    Bgcolor                                                                = 3840,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 631,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 758,
    /// BINARY-MANIFEST-ITEM-NUMERICAL-VALUE
    BinaryManifestItemNumericalValue                                       = 1078,
    /// BINARY-MANIFEST-ITEM-POINTER-VALUE
    BinaryManifestItemPointerValue                                         = 1728,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 1281,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 3931,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 5054,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 4430,
    /// BIT
    Bit                                                                    = 1836,
    /// BIT-NUMBER
    BitNumber                                                              = 1223,
    /// BIT-OFFSET
    BitOffset                                                              = 1825,
    /// BIT-POSITION
    BitPosition                                                            = 1080,
    /// BITFIELD-TEXT-TABLE-MASK-FIRST
    BitfieldTextTableMaskFirst                                             = 559,
    /// BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR
    BitfieldTextTableMaskNvBlockDescriptor                                 = 1256,
    /// BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE
    BitfieldTextTableMaskPortPrototype                                     = 1231,
    /// BITFIELD-TEXT-TABLE-MASK-SECOND
    BitfieldTextTableMaskSecond                                            = 5178,
    /// BLACKLISTED-VERSIONS
    BlacklistedVersions                                                    = 2224,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREF
    BlockIfStateActiveApIref                                               = 1965,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREFS
    BlockIfStateActiveApIrefs                                              = 197,
    /// BLOCK-IF-STATE-ACTIVE-CP-REF
    BlockIfStateActiveCpRef                                                = 3885,
    /// BLOCK-IF-STATE-ACTIVE-CP-REFS
    BlockIfStateActiveCpRefs                                               = 5499,
    /// BLOCK-STATE
    BlockState                                                             = 4696,
    /// BLOCK-STATES
    BlockStates                                                            = 4522,
    /// BLUEPRINT-CONDITION
    BlueprintCondition                                                     = 201,
    /// BLUEPRINT-DERIVATION-GUIDE
    BlueprintDerivationGuide                                               = 4269,
    /// BLUEPRINT-MAPPING
    BlueprintMapping                                                       = 5327,
    /// BLUEPRINT-MAPPING-GUIDE
    BlueprintMappingGuide                                                  = 867,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 871,
    /// BLUEPRINT-MAPS
    BlueprintMaps                                                          = 1133,
    /// BLUEPRINT-POLICY-LIST
    BlueprintPolicyList                                                    = 124,
    /// BLUEPRINT-POLICY-NOT-MODIFIABLE
    BlueprintPolicyNotModifiable                                           = 3614,
    /// BLUEPRINT-POLICY-SINGLE
    BlueprintPolicySingle                                                  = 1489,
    /// BLUEPRINT-POLICYS
    BlueprintPolicys                                                       = 712,
    /// BLUEPRINT-REF
    BlueprintRef                                                           = 3276,
    /// BOOLEAN-VALUE-VARIATION-POINT
    BooleanValueVariationPoint                                             = 5066,
    /// BOR-COUNTER-L-1-TO-L-2
    BorCounterL1ToL2                                                       = 5579,
    /// BOR-TIME-L-1
    BorTimeL1                                                              = 3272,
    /// BOR-TIME-L-2
    BorTimeL2                                                              = 2481,
    /// BOR-TIME-TX-ENSURED
    BorTimeTxEnsured                                                       = 908,
    /// BR
    Br                                                                     = 3736,
    /// BROADCAST
    Broadcast                                                              = 5091,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 2711,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 2566,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 5099,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 5525,
    /// BSW-BEHAVIOR-REF
    BswBehaviorRef                                                         = 5146,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 5126,
    /// BSW-CLIENT-POLICY
    BswClientPolicy                                                        = 5359,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 5632,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 483,
    /// BSW-DATA-SEND-POLICY
    BswDataSendPolicy                                                      = 1575,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 5035,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 985,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 4128,
    /// BSW-ENTITY-REF
    BswEntityRef                                                           = 4829,
    /// BSW-ENTRY-KIND
    BswEntryKind                                                           = 1642,
    /// BSW-ENTRY-RELATIONSHIP
    BswEntryRelationship                                                   = 642,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 110,
    /// BSW-ENTRY-RELATIONSHIP-TYPE
    BswEntryRelationshipType                                               = 3389,
    /// BSW-ENTRY-RELATIONSHIPS
    BswEntryRelationships                                                  = 1676,
    /// BSW-EXCLUSIVE-AREA-POLICY
    BswExclusiveAreaPolicy                                                 = 3779,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 2766,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 61,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 533,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 4287,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 3816,
    /// BSW-INTERNAL-TRIGGERING-POINT-POLICY
    BswInternalTriggeringPointPolicy                                       = 2056,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF
    BswInternalTriggeringPointRef                                          = 5120,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL
    BswInternalTriggeringPointRefConditional                               = 3809,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 4019,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 1547,
    /// BSW-MODE-GROUP-REF
    BswModeGroupRef                                                        = 3627,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 3304,
    /// BSW-MODE-RECEIVER-POLICY
    BswModeReceiverPolicy                                                  = 2588,
    /// BSW-MODE-SENDER-POLICY
    BswModeSenderPolicy                                                    = 510,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 65,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 4350,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 968,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 3202,
    /// BSW-MODULE-DEPENDENCYS
    BswModuleDependencys                                                   = 1251,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 3055,
    /// BSW-MODULE-DESCRIPTION-REF
    BswModuleDescriptionRef                                                = 4459,
    /// BSW-MODULE-DESCRIPTION-REF-CONDITIONAL
    BswModuleDescriptionRefConditional                                     = 3058,
    /// BSW-MODULE-DOCUMENTATIONS
    BswModuleDocumentations                                                = 5049,
    /// BSW-MODULE-ENTITY-REF
    BswModuleEntityRef                                                     = 4106,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 518,
    /// BSW-MODULE-ENTRY-REF
    BswModuleEntryRef                                                      = 4483,
    /// BSW-MODULE-ENTRY-REF-CONDITIONAL
    BswModuleEntryRefConditional                                           = 1376,
    /// BSW-MODULE-INSTANCE-REF
    BswModuleInstanceRef                                                   = 3661,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 3459,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 293,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 3233,
    /// BSW-PARAMETER-POLICY
    BswParameterPolicy                                                     = 4100,
    /// BSW-PER-INSTANCE-MEMORY-POLICY
    BswPerInstanceMemoryPolicy                                             = 1024,
    /// BSW-PER-INSTANCE-MEMORY-POLICYS
    BswPerInstanceMemoryPolicys                                            = 767,
    /// BSW-QUEUED-DATA-RECEPTION-POLICY
    BswQueuedDataReceptionPolicy                                           = 2837,
    /// BSW-RELEASED-TRIGGER-POLICY
    BswReleasedTriggerPolicy                                               = 5661,
    /// BSW-RESOLUTION
    BswResolution                                                          = 4040,
    /// BSW-RESOURCE-ESTIMATION
    BswResourceEstimation                                                  = 5059,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 976,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 4342,
    /// BSW-SERVICE-DEPENDENCY
    BswServiceDependency                                                   = 4932,
    /// BSW-SERVICE-DEPENDENCY-REF
    BswServiceDependencyRef                                                = 1356,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 1629,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 718,
    /// BSW-TRIGGER-DIRECT-IMPLEMENTATION
    BswTriggerDirectImplementation                                         = 1834,
    /// BSW-TRIGGER-REF
    BswTriggerRef                                                          = 1138,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 2065,
    /// BUFFER-COMPUTATION
    BufferComputation                                                      = 1679,
    /// BUFFER-OUTPUT
    BufferOutput                                                           = 5243,
    /// BUFFER-PROPERTIES
    BufferProperties                                                       = 5257,
    /// BUFFER-RATIO
    BufferRatio                                                            = 5370,
    /// BUILD-ACTION
    BuildAction                                                            = 257,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 2084,
    /// BUILD-ACTION-ENVIRONMENTS
    BuildActionEnvironments                                                = 3943,
    /// BUILD-ACTION-IO-ELEMENT
    BuildActionIoElement                                                   = 3605,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 1527,
    /// BUILD-ACTION-MANIFEST-REF
    BuildActionManifestRef                                                 = 1041,
    /// BUILD-ACTION-MANIFEST-REF-CONDITIONAL
    BuildActionManifestRefConditional                                      = 2483,
    /// BUILD-ACTION-MANIFESTS
    BuildActionManifests                                                   = 5315,
    /// BUILD-ACTIONS
    BuildActions                                                           = 1014,
    /// BUILD-TYPE
    BuildType                                                              = 2940,
    /// BULK-NV-BLOCK
    BulkNvBlock                                                            = 5183,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 2223,
    /// BULK-NV-DATA-DESCRIPTORS
    BulkNvDataDescriptors                                                  = 4820,
    /// BUNDLED-CONNECTIONS
    BundledConnections                                                     = 3531,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 684,
    /// BURST-SIZE
    BurstSize                                                              = 2587,
    /// BURST-SIZE-REQUEST
    BurstSizeRequest                                                       = 1731,
    /// BURST-SIZE-RESPONSE
    BurstSizeResponse                                                      = 3157,
    /// BUS-DEPENDENT-NM-ECUS
    BusDependentNmEcus                                                     = 3128,
    /// BUS-IDLE-TIMEOUT-PERIOD
    BusIdleTimeoutPeriod                                                   = 5461,
    /// BUS-MIRROR-CAN-ID-RANGE-MAPPING
    BusMirrorCanIdRangeMapping                                             = 893,
    /// BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING
    BusMirrorCanIdToCanIdMapping                                           = 1508,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 2499,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 1425,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 1360,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 5362,
    /// BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING
    BusMirrorLinPidToCanIdMapping                                          = 4782,
    /// BUS-MIRROR-NETWORK-ID
    BusMirrorNetworkId                                                     = 5618,
    /// BUS-OFF-RECOVERY
    BusOffRecovery                                                         = 4360,
    /// BUS-SPECIFIC-NM-ECU
    BusSpecificNmEcu                                                       = 4136,
    /// BY-PASS-POINT-IREF
    ByPassPointIref                                                        = 2370,
    /// BY-PASS-POINT-IREFS
    ByPassPointIrefs                                                       = 1240,
    /// BYTE
    Byte                                                                   = 4108,
    /// BYTE-ORDER
    ByteOrder                                                              = 232,
    /// BYTE-VALUE
    ByteValue                                                              = 3075,
    /// BYTE-VALUES
    ByteValues                                                             = 21,
    /// C-CODE
    CCode                                                                  = 4011,
    /// CALC-RAM-BLOCK-CRC
    CalcRamBlockCrc                                                        = 132,
    /// CALCULATION-FORMULA
    CalculationFormula                                                     = 881,
    /// CALIBRATION-PARAMETER-VALUE
    CalibrationParameterValue                                              = 4828,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 3844,
    /// CALIBRATION-PARAMETER-VALUE-SET-REF
    CalibrationParameterValueSetRef                                        = 5343,
    /// CALIBRATION-PARAMETER-VALUE-SET-REFS
    CalibrationParameterValueSetRefs                                       = 2265,
    /// CALIBRATION-PARAMETER-VALUES
    CalibrationParameterValues                                             = 1174,
    /// CALL-POINTS
    CallPoints                                                             = 1497,
    /// CALL-SIGNAL-REF
    CallSignalRef                                                          = 4135,
    /// CALL-SIGNAL-TRIGGERING-REF
    CallSignalTriggeringRef                                                = 2570,
    /// CALL-TIMEOUT
    CallTimeout                                                            = 5321,
    /// CALL-TYPE
    CallType                                                               = 2527,
    /// CALLBACK-HEADER-REF
    CallbackHeaderRef                                                      = 143,
    /// CALLBACK-HEADER-REFS
    CallbackHeaderRefs                                                     = 3224,
    /// CALLED-ENTRY-REF
    CalledEntryRef                                                         = 5281,
    /// CALLED-ENTRYS
    CalledEntrys                                                           = 5201,
    /// CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF
    CalledFromWithinExclusiveAreaRef                                       = 2854,
    /// CAN-ADDRESSING-MODE
    CanAddressingMode                                                      = 784,
    /// CAN-BE-INVOKED-CONCURRENTLY
    CanBeInvokedConcurrently                                               = 3810,
    /// CAN-CLUSTER
    CanCluster                                                             = 240,
    /// CAN-CLUSTER-CONDITIONAL
    CanClusterConditional                                                  = 602,
    /// CAN-CLUSTER-VARIANTS
    CanClusterVariants                                                     = 3313,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 3795,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 4227,
    /// CAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    CanCommunicationControllerConditional                                  = 127,
    /// CAN-COMMUNICATION-CONTROLLER-VARIANTS
    CanCommunicationControllerVariants                                     = 627,
    /// CAN-CONTROLLER-ATTRIBUTES
    CanControllerAttributes                                                = 43,
    /// CAN-CONTROLLER-CONFIGURATION
    CanControllerConfiguration                                             = 4004,
    /// CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS
    CanControllerConfigurationRequirements                                 = 3028,
    /// CAN-CONTROLLER-FD-ATTRIBUTES
    CanControllerFdAttributes                                              = 3442,
    /// CAN-CONTROLLER-FD-REQUIREMENTS
    CanControllerFdRequirements                                            = 3162,
    /// CAN-ENTER-EXCLUSIVE-AREA-REF
    CanEnterExclusiveAreaRef                                               = 291,
    /// CAN-ENTER-EXCLUSIVE-AREA-REFS
    CanEnterExclusiveAreaRefs                                              = 3416,
    /// CAN-ENTERS
    CanEnters                                                              = 2954,
    /// CAN-FD-BAUDRATE
    CanFdBaudrate                                                          = 3051,
    /// CAN-FD-FRAME-SUPPORT
    CanFdFrameSupport                                                      = 1992,
    /// CAN-FRAME
    CanFrame                                                               = 1912,
    /// CAN-FRAME-RX-BEHAVIOR
    CanFrameRxBehavior                                                     = 4410,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 5028,
    /// CAN-FRAME-TX-BEHAVIOR
    CanFrameTxBehavior                                                     = 4384,
    /// CAN-GLOBAL-TIME-DOMAIN-PROPS
    CanGlobalTimeDomainProps                                               = 3394,
    /// CAN-ID-RANGE-MAPPINGS
    CanIdRangeMappings                                                     = 1892,
    /// CAN-ID-TO-CAN-ID-MAPPINGS
    CanIdToCanIdMappings                                                   = 4158,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 3546,
    /// CAN-NM-CLUSTER-COUPLING
    CanNmClusterCoupling                                                   = 5581,
    /// CAN-NM-ECU
    CanNmEcu                                                               = 4774,
    /// CAN-NM-NODE
    CanNmNode                                                              = 5180,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 5491,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 2105,
    /// CAN-TP-ADDRESS-REF
    CanTpAddressRef                                                        = 890,
    /// CAN-TP-ADDRESS-REFS
    CanTpAddressRefs                                                       = 4861,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 2119,
    /// CAN-TP-CHANNEL-REF
    CanTpChannelRef                                                        = 2718,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 3196,
    /// CAN-TP-CONNECTION
    CanTpConnection                                                        = 30,
    /// CAN-TP-ECU
    CanTpEcu                                                               = 3262,
    /// CAN-TP-NODE
    CanTpNode                                                              = 2461,
    /// CANCELLATION
    Cancellation                                                           = 4871,
    /// CAPABILITY-RECORDS
    CapabilityRecords                                                      = 4987,
    /// CAPTION
    Caption                                                                = 2393,
    /// CAS-RX-LOW-MAX
    CasRxLowMax                                                            = 4611,
    /// CAT-2-ISR
    Cat2Isr                                                                = 1106,
    /// CATEGORY
    Category                                                               = 5280,
    /// CDD-TYPE
    CddType                                                                = 4733,
    /// CERT-COMMON-NAME
    CertCommonName                                                         = 3663,
    /// CERTIFICATE-REF
    CertificateRef                                                         = 3274,
    /// CERTIFICATE-TO-KEY-SLOT-MAPPINGS
    CertificateToKeySlotMappings                                           = 1987,
    /// CHANGE
    Change                                                                 = 155,
    /// CHANNEL-ID
    ChannelId                                                              = 4802,
    /// CHANNEL-MODE
    ChannelMode                                                            = 1112,
    /// CHANNEL-NAME
    ChannelName                                                            = 246,
    /// CHANNEL-REF
    ChannelRef                                                             = 3270,
    /// CHANNELS
    Channels                                                               = 581,
    /// CHAPTER
    Chapter                                                                = 2639,
    /// CHAPTERS
    Chapters                                                               = 3806,
    /// CHECK-PER-SOURCE-ID
    CheckPerSourceId                                                       = 2919,
    /// CHECK-STATIC-BLOCK-ID
    CheckStaticBlockId                                                     = 945,
    /// CHECKPOINT-FINAL-REF
    CheckpointFinalRef                                                     = 4584,
    /// CHECKPOINT-FINAL-REFS
    CheckpointFinalRefs                                                    = 1323,
    /// CHECKPOINT-ID
    CheckpointId                                                           = 4754,
    /// CHECKPOINT-INITIAL-REF
    CheckpointInitialRef                                                   = 848,
    /// CHECKPOINT-INITIAL-REFS
    CheckpointInitialRefs                                                  = 3188,
    /// CHECKPOINT-IREF
    CheckpointIref                                                         = 874,
    /// CHECKPOINT-REF
    CheckpointRef                                                          = 4423,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 2288,
    /// CHECKPOINT-TRANSITION-REF
    CheckpointTransitionRef                                                = 821,
    /// CHECKPOINTS
    Checkpoints                                                            = 5015,
    /// CHECKPOINTSS
    Checkpointss                                                           = 800,
    /// CHECKSUM-VALUE
    ChecksumValue                                                          = 2358,
    /// CHOICES
    Choices                                                                = 3094,
    /// CIPHER-SUITE-ID
    CipherSuiteId                                                          = 48,
    /// CIPHER-SUITE-NAME
    CipherSuiteName                                                        = 2266,
    /// CIPHER-SUITE-SHORT-LABEL
    CipherSuiteShortLabel                                                  = 2852,
    /// CLAIMED-FUNCTION-GROUP-REF
    ClaimedFunctionGroupRef                                                = 4978,
    /// CLAIMED-FUNCTION-GROUP-REFS
    ClaimedFunctionGroupRefs                                               = 4370,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 4288,
    /// CLASS-CONTENTS
    ClassContents                                                          = 754,
    /// CLASS-TAILORINGS
    ClassTailorings                                                        = 5475,
    /// CLASSIFICATION
    Classification                                                         = 2882,
    /// CLASSIFICATIONS
    Classifications                                                        = 2433,
    /// CLEAR-CONDITION-GROUP-REF
    ClearConditionGroupRef                                                 = 4961,
    /// CLEAR-CONDITION-REF
    ClearConditionRef                                                      = 926,
    /// CLEAR-CONDITION-REFS
    ClearConditionRefs                                                     = 3728,
    /// CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF
    ClearDiagnosticInformationClassRef                                     = 2946,
    /// CLEAR-DTC-LIMITATION
    ClearDtcLimitation                                                     = 2913,
    /// CLEAR-EVENT-ALLOWED-BEHAVIOR
    ClearEventAllowedBehavior                                              = 1309,
    /// CLEAR-EVENT-BEHAVIOR
    ClearEventBehavior                                                     = 3997,
    /// CLEAR-FROM-VALID-TO-INVALID
    ClearFromValidToInvalid                                                = 4420,
    /// CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF
    ClearResetEmissionRelatedDiagnosticInfoClassRef                        = 4744,
    /// CLIENT-CAPABILITY
    ClientCapability                                                       = 5174,
    /// CLIENT-COM-SPEC
    ClientComSpec                                                          = 1146,
    /// CLIENT-ID
    ClientId                                                               = 1907,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 18,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 953,
    /// CLIENT-ID-DEFINITION-SET-REF
    ClientIdDefinitionSetRef                                               = 2189,
    /// CLIENT-ID-DEFINITION-SET-REFS
    ClientIdDefinitionSetRefs                                              = 4934,
    /// CLIENT-ID-DEFINITIONS
    ClientIdDefinitions                                                    = 225,
    /// CLIENT-ID-RANGE
    ClientIdRange                                                          = 1022,
    /// CLIENT-INTENT
    ClientIntent                                                           = 4153,
    /// CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST
    ClientIpAddrFromConnectionRequest                                      = 1255,
    /// CLIENT-POLICYS
    ClientPolicys                                                          = 5337,
    /// CLIENT-PORT-FROM-CONNECTION-REQUEST
    ClientPortFromConnectionRequest                                        = 1460,
    /// CLIENT-PORT-REF
    ClientPortRef                                                          = 1414,
    /// CLIENT-SERVER-ANNOTATION
    ClientServerAnnotation                                                 = 3657,
    /// CLIENT-SERVER-ANNOTATIONS
    ClientServerAnnotations                                                = 2847,
    /// CLIENT-SERVER-APPLICATION-ERROR-MAPPING
    ClientServerApplicationErrorMapping                                    = 2698,
    /// CLIENT-SERVER-ARRAY-ELEMENT-MAPPING
    ClientServerArrayElementMapping                                        = 4286,
    /// CLIENT-SERVER-ARRAY-TYPE-MAPPING
    ClientServerArrayTypeMapping                                           = 231,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 3378,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 1303,
    /// CLIENT-SERVER-INTERFACE-REF
    ClientServerInterfaceRef                                               = 5558,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 4383,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 5108,
    /// CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING
    ClientServerOperationBlueprintMapping                                  = 3554,
    /// CLIENT-SERVER-OPERATION-IREF
    ClientServerOperationIref                                              = 938,
    /// CLIENT-SERVER-OPERATION-MAPPING
    ClientServerOperationMapping                                           = 2871,
    /// CLIENT-SERVER-OPERATION-REF
    ClientServerOperationRef                                               = 3900,
    /// CLIENT-SERVER-PORTS
    ClientServerPorts                                                      = 353,
    /// CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING
    ClientServerPrimitiveTypeMapping                                       = 2531,
    /// CLIENT-SERVER-RECORD-ELEMENT-MAPPING
    ClientServerRecordElementMapping                                       = 4397,
    /// CLIENT-SERVER-RECORD-TYPE-MAPPING
    ClientServerRecordTypeMapping                                          = 4834,
    /// CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING
    ClientServerToSignalGroupMapping                                       = 2425,
    /// CLIENT-SERVER-TO-SIGNAL-MAPPING
    ClientServerToSignalMapping                                            = 5417,
    /// CLIENT-SERVICE-MAJOR-VERSION
    ClientServiceMajorVersion                                              = 374,
    /// CLIENT-SERVICE-MINOR-VERSION
    ClientServiceMinorVersion                                              = 4668,
    /// CLUSTER-DRIFT-DAMPING
    ClusterDriftDamping                                                    = 3343,
    /// CLUSTERED-COMPONENT-IREF
    ClusteredComponentIref                                                 = 743,
    /// CLUSTERED-COMPONENT-IREFS
    ClusteredComponentIrefs                                                = 1770,
    /// CODE
    Code                                                                   = 1096,
    /// CODE-DESCRIPTORS
    CodeDescriptors                                                        = 3405,
    /// CODE-LABEL
    CodeLabel                                                              = 3349,
    /// COLD-START-ATTEMPTS
    ColdStartAttempts                                                      = 4072,
    /// COLLECTED-INSTANCE-IREF
    CollectedInstanceIref                                                  = 1690,
    /// COLLECTED-INSTANCE-IREFS
    CollectedInstanceIrefs                                                 = 4457,
    /// COLLECTION
    Collection                                                             = 4757,
    /// COLLECTION-PROPS
    CollectionProps                                                        = 2154,
    /// COLLECTION-REF
    CollectionRef                                                          = 1833,
    /// COLLECTION-SEMANTICS
    CollectionSemantics                                                    = 2585,
    /// COLLISION-RESOLVING-SCHEDULE-REF
    CollisionResolvingScheduleRef                                          = 1841,
    /// COLSPEC
    Colspec                                                                = 4351,
    /// COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS
    ComBasedSignalGroupTransformations                                     = 4818,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 174,
    /// COM-CONFIGURATION-GW-TIME-BASE
    ComConfigurationGwTimeBase                                             = 5191,
    /// COM-CONFIGURATION-ID
    ComConfigurationId                                                     = 3676,
    /// COM-CONFIGURATION-RX-TIME-BASE
    ComConfigurationRxTimeBase                                             = 3424,
    /// COM-CONFIGURATION-TX-TIME-BASE
    ComConfigurationTxTimeBase                                             = 1636,
    /// COM-CONTROL-CLASS-REF
    ComControlClassRef                                                     = 3451,
    /// COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION
    ComEnableMdtForCyclicTransmission                                      = 2396,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 370,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 3463,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 2404,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 2324,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 1793,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 3665,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 4622,
    /// COM-MANAGEMENT-GROUP-REF
    ComManagementGroupRef                                                  = 2976,
    /// COM-MANAGEMENT-GROUP-REFS
    ComManagementGroupRefs                                                 = 4856,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 2941,
    /// COM-MANAGEMENT-MAPPINGS
    ComManagementMappings                                                  = 3562,
    /// COM-MANAGEMENT-PORT-GROUP-IREF
    ComManagementPortGroupIref                                             = 1948,
    /// COM-MANAGEMENT-PORT-GROUP-IREFS
    ComManagementPortGroupIrefs                                            = 4753,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 3285,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 2925,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 3687,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 1820,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 5553,
    /// COM-PROCESSING-PERIOD
    ComProcessingPeriod                                                    = 4922,
    /// COM-PROPS
    ComProps                                                               = 5067,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 4259,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 957,
    /// COMM-CONNECTORS
    CommConnectors                                                         = 1835,
    /// COMM-CONTROLLER-MAPPINGS
    CommControllerMappings                                                 = 2601,
    /// COMM-CONTROLLER-REF
    CommControllerRef                                                      = 4337,
    /// COMM-CONTROLLERS
    CommControllers                                                        = 1293,
    /// COMMAND
    Command                                                                = 5481,
    /// COMMANDS
    Commands                                                               = 3555,
    /// COMMENT
    Comment                                                                = 2126,
    /// COMMON-PROPERTIES
    CommonProperties                                                       = 3165,
    /// COMMON-SIGNAL-PATH
    CommonSignalPath                                                       = 3197,
    /// COMMUNICATION-BUFFER-LOCKING
    CommunicationBufferLocking                                             = 2833,
    /// COMMUNICATION-CLUSTER-REF
    CommunicationClusterRef                                                = 2121,
    /// COMMUNICATION-CLUSTER-REFS
    CommunicationClusterRefs                                               = 2066,
    /// COMMUNICATION-CONNECTOR-REF
    CommunicationConnectorRef                                              = 4764,
    /// COMMUNICATION-CONNECTOR-REF-CONDITIONAL
    CommunicationConnectorRefConditional                                   = 4724,
    /// COMMUNICATION-CONNECTORS
    CommunicationConnectors                                                = 2454,
    /// COMMUNICATION-CONTROLLER-MAPPING
    CommunicationControllerMapping                                         = 3235,
    /// COMMUNICATION-CONTROLLER-REF
    CommunicationControllerRef                                             = 4790,
    /// COMMUNICATION-CYCLE
    CommunicationCycle                                                     = 2775,
    /// COMMUNICATION-DIRECTION
    CommunicationDirection                                                 = 3119,
    /// COMMUNICATION-MODE
    CommunicationMode                                                      = 2444,
    /// COMMUNICATION-RESOURCE-PROPS
    CommunicationResourceProps                                             = 1964,
    /// COMMUNICATION-RESOURCE-REF
    CommunicationResourceRef                                               = 2927,
    /// COMPARE-TYPE
    CompareType                                                            = 2789,
    /// COMPARE-VALUE
    CompareValue                                                           = 4891,
    /// COMPILER
    Compiler                                                               = 3582,
    /// COMPILERS
    Compilers                                                              = 1659,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 3156,
    /// COMPLEX-TYPE-MAPPING
    ComplexTypeMapping                                                     = 2547,
    /// COMPONENT-CLUSTERING
    ComponentClustering                                                    = 1494,
    /// COMPONENT-IMPLEMENTATION-REF
    ComponentImplementationRef                                             = 437,
    /// COMPONENT-IREF
    ComponentIref                                                          = 2252,
    /// COMPONENT-IREFS
    ComponentIrefs                                                         = 4535,
    /// COMPONENT-REF
    ComponentRef                                                           = 4941,
    /// COMPONENT-SEPARATION
    ComponentSeparation                                                    = 2471,
    /// COMPONENTS
    Components                                                             = 486,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 2512,
    /// COMPOSITE-NETWORK-REPRESENTATION
    CompositeNetworkRepresentation                                         = 1518,
    /// COMPOSITE-NETWORK-REPRESENTATIONS
    CompositeNetworkRepresentations                                        = 3301,
    /// COMPOSITE-RULE-BASED-VALUE-SPECIFICATION
    CompositeRuleBasedValueSpecification                                   = 1991,
    /// COMPOSITE-SERVICE-INTERFACE-REF
    CompositeServiceInterfaceRef                                           = 2420,
    /// COMPOSITE-TYPE-MAPPINGS
    CompositeTypeMappings                                                  = 4635,
    /// COMPOSITION-DATA-PROTOTYPE-REF
    CompositionDataPrototypeRef                                            = 2259,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 3348,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 5200,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 4647,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF
    CompositionSwComponentTypeRef                                          = 815,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL
    CompositionSwComponentTypeRefConditional                               = 3629,
    /// COMPOUND-PRIMITIVE-ARGUMENTS
    CompoundPrimitiveArguments                                             = 5660,
    /// COMPRESSED-SOFTWARE-PACKAGE-SIZE
    CompressedSoftwarePackageSize                                          = 2401,
    /// COMPU-CONST
    CompuConst                                                             = 2219,
    /// COMPU-DEFAULT-VALUE
    CompuDefaultValue                                                      = 5587,
    /// COMPU-DENOMINATOR
    CompuDenominator                                                       = 710,
    /// COMPU-INTERNAL-TO-PHYS
    CompuInternalToPhys                                                    = 4184,
    /// COMPU-INVERSE-VALUE
    CompuInverseValue                                                      = 4651,
    /// COMPU-METHOD
    CompuMethod                                                            = 3745,
    /// COMPU-METHOD-REF
    CompuMethodRef                                                         = 5513,
    /// COMPU-NUMERATOR
    CompuNumerator                                                         = 4728,
    /// COMPU-PHYS-TO-INTERNAL
    CompuPhysToInternal                                                    = 2923,
    /// COMPU-RATIONAL-COEFFS
    CompuRationalCoeffs                                                    = 1957,
    /// COMPU-SCALE
    CompuScale                                                             = 4097,
    /// COMPU-SCALES
    CompuScales                                                            = 4621,
    /// COMPUTED
    Computed                                                               = 701,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 4028,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 4810,
    /// COND
    Cond                                                                   = 1167,
    /// CONDITION
    Condition                                                              = 2380,
    /// CONDITION-ACCESS
    ConditionAccess                                                        = 212,
    /// CONDITION-FORMULA
    ConditionFormula                                                       = 2829,
    /// CONDITIONAL-CHANGE-NAD
    ConditionalChangeNad                                                   = 4520,
    /// CONDITIONS
    Conditions                                                             = 286,
    /// CONFIDENCE-INTERVAL
    ConfidenceInterval                                                     = 2532,
    /// CONFIDENCE-INTERVALS
    ConfidenceIntervals                                                    = 4648,
    /// CONFIG-CLASS
    ConfigClass                                                            = 3478,
    /// CONFIG-ELEMENT-DEF-GLOBAL-REF
    ConfigElementDefGlobalRef                                              = 2081,
    /// CONFIG-ELEMENT-DEF-LOCAL-REF
    ConfigElementDefLocalRef                                               = 3871,
    /// CONFIG-VARIANT
    ConfigVariant                                                          = 2206,
    /// CONFIGURATION-CLASS-AFFECTION
    ConfigurationClassAffection                                            = 5004,
    /// CONFIGURATION-HANDLING
    ConfigurationHandling                                                  = 2402,
    /// CONFIGURED-NAD
    ConfiguredNad                                                          = 4497,
    /// CONFIRMATION-THRESHOLD
    ConfirmationThreshold                                                  = 3387,
    /// CONFLICTS
    Conflicts                                                              = 450,
    /// CONFLICTS-TO
    ConflictsTo                                                            = 243,
    /// CONNECT
    Connect                                                                = 2684,
    /// CONNECTED-INDICATORS
    ConnectedIndicators                                                    = 3733,
    /// CONNECTION-BUNDLES
    ConnectionBundles                                                      = 614,
    /// CONNECTION-IS-MANDATORY
    ConnectionIsMandatory                                                  = 2520,
    /// CONNECTION-NEGOTIATION-BEHAVIOR
    ConnectionNegotiationBehavior                                          = 2937,
    /// CONNECTION-PRIO-PDUS
    ConnectionPrioPdus                                                     = 5592,
    /// CONNECTION-TYPE
    ConnectionType                                                         = 4689,
    /// CONNECTIONS
    Connections                                                            = 2254,
    /// CONNECTOR-REF
    ConnectorRef                                                           = 892,
    /// CONNECTOR-REFS
    ConnectorRefs                                                          = 1693,
    /// CONNECTORS
    Connectors                                                             = 4087,
    /// CONSIDER-BUILD-NUMBER
    ConsiderBuildNumber                                                    = 2537,
    /// CONSIDER-PTO-STATUS
    ConsiderPtoStatus                                                      = 1531,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 3290,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 4466,
    /// CONSISTENCY-NEEDSS
    ConsistencyNeedss                                                      = 4455,
    /// CONSTANT-MAPPING-REF
    ConstantMappingRef                                                     = 774,
    /// CONSTANT-MAPPING-REFS
    ConstantMappingRefs                                                    = 1289,
    /// CONSTANT-MEMORYS
    ConstantMemorys                                                        = 2122,
    /// CONSTANT-REF
    ConstantRef                                                            = 310,
    /// CONSTANT-REFERENCE
    ConstantReference                                                      = 3116,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 3218,
    /// CONSTANT-SPECIFICATION-MAPPING
    ConstantSpecificationMapping                                           = 566,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 2891,
    /// CONSTANT-VALUE-MAPPING-REF
    ConstantValueMappingRef                                                = 3287,
    /// CONSTANT-VALUE-MAPPING-REFS
    ConstantValueMappingRefs                                               = 3619,
    /// CONSTR-LEVEL
    ConstrLevel                                                            = 2878,
    /// CONSTRAINT-REF
    ConstraintRef                                                          = 2640,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 3722,
    /// CONSTRAINT-TAILORINGS
    ConstraintTailorings                                                   = 1781,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 1564,
    /// CONSUMED-EVENT-GROUP-REF
    ConsumedEventGroupRef                                                  = 192,
    /// CONSUMED-EVENT-GROUP-REFS
    ConsumedEventGroupRefs                                                 = 194,
    /// CONSUMED-EVENT-GROUPS
    ConsumedEventGroups                                                    = 2907,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 2571,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF
    ConsumedProvidedServiceInstanceGroupRef                                = 5039,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL
    ConsumedProvidedServiceInstanceGroupRefConditional                     = 2914,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 3843,
    /// CONSUMED-SERVICE-INSTANCE-REF
    ConsumedServiceInstanceRef                                             = 509,
    /// CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL
    ConsumedServiceInstanceRefConditional                                  = 260,
    /// CONSUMED-SERVICE-INSTANCES
    ConsumedServiceInstances                                               = 2975,
    /// CONTAINED-AR-ELEMENT-REF
    ContainedArElementRef                                                  = 4601,
    /// CONTAINED-AR-ELEMENT-REFS
    ContainedArElementRefs                                                 = 3395,
    /// CONTAINED-FIBEX-ELEMENT-REF
    ContainedFibexElementRef                                               = 1030,
    /// CONTAINED-FIBEX-ELEMENT-REFS
    ContainedFibexElementRefs                                              = 1603,
    /// CONTAINED-I-PDU-PROPS
    ContainedIPduProps                                                     = 1817,
    /// CONTAINED-I-PDU-TRIGGERING-PROPSS
    ContainedIPduTriggeringPropss                                          = 3383,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REF
    ContainedISignalIPduGroupRef                                           = 611,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS
    ContainedISignalIPduGroupRefs                                          = 3670,
    /// CONTAINED-PACKAGE-ELEMENT-REF
    ContainedPackageElementRef                                             = 2918,
    /// CONTAINED-PACKAGE-ELEMENT-REFS
    ContainedPackageElementRefs                                            = 2023,
    /// CONTAINED-PDU-TRIGGERING-REF
    ContainedPduTriggeringRef                                              = 3152,
    /// CONTAINED-PDU-TRIGGERING-REFS
    ContainedPduTriggeringRefs                                             = 3246,
    /// CONTAINED-PROCESS-REF
    ContainedProcessRef                                                    = 4752,
    /// CONTAINED-PROCESS-REFS
    ContainedProcessRefs                                                   = 2247,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 3534,
    /// CONTAINER-I-PDU-HEADER-BYTE-ORDER
    ContainerIPduHeaderByteOrder                                           = 3710,
    /// CONTAINER-TIMEOUT
    ContainerTimeout                                                       = 3698,
    /// CONTAINER-TRIGGER
    ContainerTrigger                                                       = 5554,
    /// CONTAINERS
    Containers                                                             = 105,
    /// CONTENT-TYPE
    ContentType                                                            = 2607,
    /// CONTENT-URI
    ContentUri                                                             = 1301,
    /// CONTEXT-APPLICATION-RECORD-ELEMENT-REF
    ContextApplicationRecordElementRef                                     = 4578,
    /// CONTEXT-ATOMIC-COMPONENT-REF
    ContextAtomicComponentRef                                              = 22,
    /// CONTEXT-BSW-IMPLEMENTATION-REF
    ContextBswImplementationRef                                            = 4494,
    /// CONTEXT-COMPONENT-PROTOTYPE-REF
    ContextComponentPrototypeRef                                           = 2038,
    /// CONTEXT-COMPONENT-REF
    ContextComponentRef                                                    = 842,
    /// CONTEXT-COMPONENT-REFS
    ContextComponentRefs                                                   = 5572,
    /// CONTEXT-COMPOSITION-REF
    ContextCompositionRef                                                  = 3789,
    /// CONTEXT-DATA-PROTOTYPE-IN-CS-REF
    ContextDataPrototypeInCsRef                                            = 1845,
    /// CONTEXT-DATA-PROTOTYPE-IN-SR-REF
    ContextDataPrototypeInSrRef                                            = 1443,
    /// CONTEXT-DATA-PROTOTYPE-REF
    ContextDataPrototypeRef                                                = 250,
    /// CONTEXT-DATA-PROTOTYPE-REFS
    ContextDataPrototypeRefs                                               = 1716,
    /// CONTEXT-DATA-SOURCE
    ContextDataSource                                                      = 2936,
    /// CONTEXT-DATAS
    ContextDatas                                                           = 1645,
    /// CONTEXT-DESCRIPTION
    ContextDescription                                                     = 3420,
    /// CONTEXT-ELEMENT-REF
    ContextElementRef                                                      = 4935,
    /// CONTEXT-ID
    ContextId                                                              = 3805,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF
    ContextImplementationDataElementRef                                    = 1875,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS
    ContextImplementationDataElementRefs                                   = 1102,
    /// CONTEXT-LIMITATION-REF
    ContextLimitationRef                                                   = 1973,
    /// CONTEXT-LIMITATION-REFS
    ContextLimitationRefs                                                  = 4914,
    /// CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ContextModeDeclarationGroupPrototypeRef                                = 4654,
    /// CONTEXT-MODE-DECLARATION-GROUP-REF
    ContextModeDeclarationGroupRef                                         = 3504,
    /// CONTEXT-OPERATION-REF
    ContextOperationRef                                                    = 4176,
    /// CONTEXT-P-PORT-PROTOTYPE-REF
    ContextPPortPrototypeRef                                               = 3660,
    /// CONTEXT-P-PORT-REF
    ContextPPortRef                                                        = 4217,
    /// CONTEXT-PORT-PROTOTYPE-REF
    ContextPortPrototypeRef                                                = 4121,
    /// CONTEXT-PORT-REF
    ContextPortRef                                                         = 599,
    /// CONTEXT-R-PORT-PROTOTYPE-REF
    ContextRPortPrototypeRef                                               = 37,
    /// CONTEXT-R-PORT-REF
    ContextRPortRef                                                        = 63,
    /// CONTEXT-REF
    ContextRef                                                             = 4963,
    /// CONTEXT-ROOT-COMPONENT-REF
    ContextRootComponentRef                                                = 1771,
    /// CONTEXT-ROOT-COMPOSITION-REF
    ContextRootCompositionRef                                              = 4462,
    /// CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF
    ContextRootSwClusterDesignComponentPrototypeRef                        = 2234,
    /// CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF
    ContextRootSwComponentPrototypeRef                                     = 951,
    /// CONTEXT-ROOT-SW-COMPOSITION-REF
    ContextRootSwCompositionRef                                            = 336,
    /// CONTEXT-SW-COMPONENT-PROTOTYPE-REF
    ContextSwComponentPrototypeRef                                         = 2600,
    /// CONTEXT-SW-COMPONENT-REF
    ContextSwComponentRef                                                  = 573,
    /// CONTEXT-SWC-PROTOTYPE-REF
    ContextSwcPrototypeRef                                                 = 3753,
    /// CONTEXT-SWC-PROTOTYPE-REFS
    ContextSwcPrototypeRefs                                                = 4570,
    /// CONTEXTS
    Contexts                                                               = 4353,
    /// CONTROL-CONSUMED-EVENT-GROUP-REF
    ControlConsumedEventGroupRef                                           = 2662,
    /// CONTROL-CONSUMED-EVENT-GROUP-REFS
    ControlConsumedEventGroupRefs                                          = 4913,
    /// CONTROL-ENABLE-MASK-BITS
    ControlEnableMaskBits                                                  = 2196,
    /// CONTROL-OPTION-RECORD-PRESENT
    ControlOptionRecordPresent                                             = 4311,
    /// CONTROL-PNC-REF
    ControlPncRef                                                          = 4042,
    /// CONTROL-PNC-REFS
    ControlPncRefs                                                         = 781,
    /// CONTROL-PROVIDED-EVENT-GROUP-REF
    ControlProvidedEventGroupRef                                           = 3423,
    /// CONTROL-PROVIDED-EVENT-GROUP-REFS
    ControlProvidedEventGroupRefs                                          = 4762,
    /// CONTROLLED-DATA-ELEMENT-REF
    ControlledDataElementRef                                               = 1517,
    /// CONTROLLED-DATA-ELEMENT-REFS
    ControlledDataElementRefs                                              = 1827,
    /// CONTROLLED-FID-REF
    ControlledFidRef                                                       = 3338,
    /// CONTROLLED-HW-ELEMENT-REF
    ControlledHwElementRef                                                 = 1757,
    /// CONTROLLER-REF
    ControllerRef                                                          = 535,
    /// CORE-ID
    CoreId                                                                 = 1021,
    /// CORES
    Cores                                                                  = 1507,
    /// COUNT-PROFILE
    CountProfile                                                           = 3985,
    /// COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE
    CounterBasedFdcThresholdStorageValue                                   = 1040,
    /// COUNTER-DECREMENT-STEP-SIZE
    CounterDecrementStepSize                                               = 3866,
    /// COUNTER-FAILED-THRESHOLD
    CounterFailedThreshold                                                 = 5516,
    /// COUNTER-INCREMENT-STEP-SIZE
    CounterIncrementStepSize                                               = 1449,
    /// COUNTER-JUMP-DOWN
    CounterJumpDown                                                        = 120,
    /// COUNTER-JUMP-DOWN-VALUE
    CounterJumpDownValue                                                   = 2312,
    /// COUNTER-JUMP-UP
    CounterJumpUp                                                          = 1264,
    /// COUNTER-JUMP-UP-VALUE
    CounterJumpUpValue                                                     = 4800,
    /// COUNTER-OFFSET
    CounterOffset                                                          = 979,
    /// COUNTER-PASSED-THRESHOLD
    CounterPassedThreshold                                                 = 2342,
    /// COUPLED-CLUSTER-REF
    CoupledClusterRef                                                      = 303,
    /// COUPLED-CLUSTER-REFS
    CoupledClusterRefs                                                     = 2100,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 5606,
    /// COUPLING-PORT
    CouplingPort                                                           = 2840,
    /// COUPLING-PORT-CONNECTION
    CouplingPortConnection                                                 = 2285,
    /// COUPLING-PORT-CONNECTIONS
    CouplingPortConnections                                                = 4193,
    /// COUPLING-PORT-DETAILS
    CouplingPortDetails                                                    = 5075,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 3377,
    /// COUPLING-PORT-RATE-POLICY
    CouplingPortRatePolicy                                                 = 5543,
    /// COUPLING-PORT-REF
    CouplingPortRef                                                        = 4755,
    /// COUPLING-PORT-REF-CONDITIONAL
    CouplingPortRefConditional                                             = 2881,
    /// COUPLING-PORT-ROLE
    CouplingPortRole                                                       = 3716,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 3354,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 4680,
    /// COUPLING-PORT-SPEED
    CouplingPortSpeed                                                      = 2879,
    /// COUPLING-PORT-STARTUP-ACTIVE-TIME
    CouplingPortStartupActiveTime                                          = 3986,
    /// COUPLING-PORT-STRUCTURAL-ELEMENTS
    CouplingPortStructuralElements                                         = 5239,
    /// COUPLING-PORT-SWITCHOFF-DELAY
    CouplingPortSwitchoffDelay                                             = 2502,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 923,
    /// COUPLING-PORTS
    CouplingPorts                                                          = 16,
    /// COUPLING-TYPE
    CouplingType                                                           = 2819,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 3160,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 1644,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 996,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 2052,
    /// CP-SOFTWARE-CLUSTER-REF
    CpSoftwareClusterRef                                                   = 822,
    /// CP-SOFTWARE-CLUSTER-REF-CONDITIONAL
    CpSoftwareClusterRefConditional                                        = 5629,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 4460,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-REF
    CpSoftwareClusterResourceRef                                           = 4147,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 252,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 4557,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 2264,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 753,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 2768,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 2586,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 4815,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 4997,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 1004,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 3477,
    /// CPP-TEMPLATE-ARGUMENT
    CppTemplateArgument                                                    = 4036,
    /// CPU-USAGE
    CpuUsage                                                               = 1292,
    /// CRC-CORRECTION-FIELD
    CrcCorrectionField                                                     = 5574,
    /// CRC-DOMAIN-NUMBER
    CrcDomainNumber                                                        = 3757,
    /// CRC-FLAGS
    CrcFlags                                                               = 720,
    /// CRC-MESSAGE-LENGTH
    CrcMessageLength                                                       = 1241,
    /// CRC-OFFSET
    CrcOffset                                                              = 3402,
    /// CRC-PRECISE-ORIGIN-TIMESTAMP
    CrcPreciseOriginTimestamp                                              = 552,
    /// CRC-SECURED
    CrcSecured                                                             = 2289,
    /// CRC-SEQUENCE-ID
    CrcSequenceId                                                          = 3331,
    /// CRC-SOURCE-PORT-IDENTITY
    CrcSourcePortIdentity                                                  = 658,
    /// CRC-VALIDATED
    CrcValidated                                                           = 4723,
    /// CREATE-ECU-WAKEUP-SOURCE
    CreateEcuWakeupSource                                                  = 1848,
    /// CREATED-DATAS
    CreatedDatas                                                           = 5206,
    /// CRYPTO-ALG-ID
    CryptoAlgId                                                            = 1722,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 5465,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 3964,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 3769,
    /// CRYPTO-CERTIFICATE-REF
    CryptoCertificateRef                                                   = 393,
    /// CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING
    CryptoCertificateToCryptoKeySlotMapping                                = 62,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 4053,
    /// CRYPTO-CERTIFICATES
    CryptoCertificates                                                     = 4400,
    /// CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS
    CryptoDesignToCryptoDriverMappings                                     = 5451,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 624,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING
    CryptoDriverToCryptoJobMapping                                         = 121,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS
    CryptoDriverToCryptoJobMappings                                        = 5399,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 5353,
    /// CRYPTO-JOB
    CryptoJob                                                              = 433,
    /// CRYPTO-JOB-REF
    CryptoJobRef                                                           = 5130,
    /// CRYPTO-JOBS
    CryptoJobs                                                             = 2301,
    /// CRYPTO-KEY-DESCRIPTION
    CryptoKeyDescription                                                   = 4422,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 3707,
    /// CRYPTO-KEY-REF
    CryptoKeyRef                                                           = 2033,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 4794,
    /// CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE
    CryptoKeySlotContentAllowedUsage                                       = 259,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 1963,
    /// CRYPTO-KEY-SLOT-REF
    CryptoKeySlotRef                                                       = 5548,
    /// CRYPTO-KEY-SLOT-REFS
    CryptoKeySlotRefs                                                      = 1438,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 670,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 2085,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 3461,
    /// CRYPTO-NEED-REF
    CryptoNeedRef                                                          = 2123,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 3644,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 4253,
    /// CRYPTO-OBJECT-TYPE
    CryptoObjectType                                                       = 3804,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 3111,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 4095,
    /// CRYPTO-PROVIDER-DOCUMENTATION-REF
    CryptoProviderDocumentationRef                                         = 4379,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 1221,
    /// CRYPTO-PROVIDER-REF
    CryptoProviderRef                                                      = 755,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 4225,
    /// CRYPTO-PROVIDERS
    CryptoProviders                                                        = 3203,
    /// CRYPTO-R-PORT-COM-SPEC
    CryptoRPortComSpec                                                     = 2241,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 4746,
    /// CRYPTO-SERVICE-CERTIFICATE-REF
    CryptoServiceCertificateRef                                            = 1601,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 691,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 4482,
    /// CRYPTO-SERVICE-KEY-REF
    CryptoServiceKeyRef                                                    = 5623,
    /// CRYPTO-SERVICE-MAPPINGS
    CryptoServiceMappings                                                  = 2886,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 4766,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 3362,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 1350,
    /// CRYPTO-SERVICE-QUEUE-REF
    CryptoServiceQueueRef                                                  = 4982,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 3511,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 4526,
    /// CS-ERROR-REACTION
    CsErrorReaction                                                        = 3819,
    /// CSE-CODE
    CseCode                                                                = 442,
    /// CSE-CODE-FACTOR
    CseCodeFactor                                                          = 2690,
    /// CURRENT-EXP
    CurrentExp                                                             = 1909,
    /// CURRENT-VALUE-REF
    CurrentValueRef                                                        = 1126,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 903,
    /// CUSTOM-DOCUMENT-ELEMENT-REF
    CustomDocumentElementRef                                               = 2228,
    /// CUSTOM-DOCUMENTATION-REF
    CustomDocumentationRef                                                 = 2113,
    /// CUSTOM-SDG-DEF-REF
    CustomSdgDefRef                                                        = 4571,
    /// CUSTOM-SDG-DEF-REFS
    CustomSdgDefRefs                                                       = 2801,
    /// CUSTOM-SERVICE-CLASS-REF
    CustomServiceClassRef                                                  = 4895,
    /// CUSTOM-SERVICE-ID
    CustomServiceId                                                        = 5333,
    /// CUSTOM-SPECIFICATION-REF
    CustomSpecificationRef                                                 = 1001,
    /// CUSTOM-SPECIFICATION-REFS
    CustomSpecificationRefs                                                = 3970,
    /// CUSTOM-SUB-FUNCTION-NUMBER
    CustomSubFunctionNumber                                                = 3996,
    /// CUSTOM-TRIGGER
    CustomTrigger                                                          = 2646,
    /// CYCLE
    Cycle                                                                  = 4443,
    /// CYCLE-AUTOSTART
    CycleAutostart                                                         = 3310,
    /// CYCLE-COUNT-MAX
    CycleCountMax                                                          = 5394,
    /// CYCLE-COUNTER
    CycleCounter                                                           = 5228,
    /// CYCLE-REPETITION
    CycleRepetition                                                        = 2650,
    /// CYCLE-STATUS-STORAGE
    CycleStatusStorage                                                     = 264,
    /// CYCLE-TIME-MAIN-FUNCTION
    CycleTimeMainFunction                                                  = 6,
    /// CYCLE-TIME-VALUE
    CycleTimeValue                                                         = 3588,
    /// CYCLIC-TIMING
    CyclicTiming                                                           = 3509,
    /// CYCLIC-WRITING-PERIOD
    CyclicWritingPeriod                                                    = 3799,
    /// DATA-COM-PROPS
    DataComProps                                                           = 3159,
    /// DATA-CONSTR
    DataConstr                                                             = 4023,
    /// DATA-CONSTR-REF
    DataConstrRef                                                          = 2536,
    /// DATA-CONSTR-RULE
    DataConstrRule                                                         = 4569,
    /// DATA-CONSTR-RULES
    DataConstrRules                                                        = 3765,
    /// DATA-DUMP-ENTRY
    DataDumpEntry                                                          = 820,
    /// DATA-ELEMENT-IREF
    DataElementIref                                                        = 835,
    /// DATA-ELEMENT-IREFS
    DataElementIrefs                                                       = 4133,
    /// DATA-ELEMENT-REF
    DataElementRef                                                         = 34,
    /// DATA-ELEMENT-REFS
    DataElementRefs                                                        = 173,
    /// DATA-ELEMENTS
    DataElements                                                           = 3899,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 1135,
    /// DATA-FILTER
    DataFilter                                                             = 3011,
    /// DATA-FILTER-TYPE
    DataFilterType                                                         = 5400,
    /// DATA-FORMAT-TAILORING
    DataFormatTailoring                                                    = 4104,
    /// DATA-ID
    DataId                                                                 = 3039,
    /// DATA-ID-MODE
    DataIdMode                                                             = 1224,
    /// DATA-ID-NIBBLE-OFFSET
    DataIdNibbleOffset                                                     = 5429,
    /// DATA-IDENTIFIER-REF
    DataIdentifierRef                                                      = 4819,
    /// DATA-IDENTIFIER-REFS
    DataIdentifierRefs                                                     = 5249,
    /// DATA-IDS
    DataIds                                                                = 2529,
    /// DATA-IREF
    DataIref                                                               = 52,
    /// DATA-LENGTH
    DataLength                                                             = 3818,
    /// DATA-LENGTH-REQUEST
    DataLengthRequest                                                      = 1935,
    /// DATA-LENGTH-RESPONSE
    DataLengthResponse                                                     = 35,
    /// DATA-MAPPINGS
    DataMappings                                                           = 1164,
    /// DATA-OFFSET
    DataOffset                                                             = 1234,
    /// DATA-PDU-REF
    DataPduRef                                                             = 589,
    /// DATA-PROTECTION-KIND
    DataProtectionKind                                                     = 5539,
    /// DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF
    DataProtototypeInPortInterfaceRef                                      = 5371,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 680,
    /// DATA-PROTOTYPE-GROUP-IREF
    DataPrototypeGroupIref                                                 = 2457,
    /// DATA-PROTOTYPE-GROUP-IREFS
    DataPrototypeGroupIrefs                                                = 528,
    /// DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF
    DataPrototypeInClientServerInterfaceIref                               = 1871,
    /// DATA-PROTOTYPE-IN-PORT-INTERFACE-REF
    DataPrototypeInPortInterfaceRef                                        = 1791,
    /// DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF
    DataPrototypeInSenderReceiverInterfaceIref                             = 5026,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF
    DataPrototypeInServiceInterfaceIref                                    = 2799,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF
    DataPrototypeInServiceInterfaceRef                                     = 3012,
    /// DATA-PROTOTYPE-IREF
    DataPrototypeIref                                                      = 3433,
    /// DATA-PROTOTYPE-MAPPING
    DataPrototypeMapping                                                   = 2563,
    /// DATA-PROTOTYPE-REF
    DataPrototypeRef                                                       = 4510,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPS
    DataPrototypeTransformationProps                                       = 2383,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPSS
    DataPrototypeTransformationPropss                                      = 1094,
    /// DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF
    DataPrototypeWithApplicationDataTypeInSystemRef                        = 57,
    /// DATA-PROTOTYPES
    DataPrototypes                                                         = 2212,
    /// DATA-PROVIDER
    DataProvider                                                           = 1809,
    /// DATA-READ-ACCESSS
    DataReadAccesss                                                        = 1762,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 796,
    /// DATA-RECEIVE-POINT-BY-ARGUMENTS
    DataReceivePointByArguments                                            = 1351,
    /// DATA-RECEIVE-POINT-BY-VALUES
    DataReceivePointByValues                                               = 3878,
    /// DATA-RECEIVE-POINTS
    DataReceivePoints                                                      = 1053,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 4880,
    /// DATA-REF
    DataRef                                                                = 2484,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 2323,
    /// DATA-SEND-POINTS
    DataSendPoints                                                         = 1580,
    /// DATA-TRANSFER-CLASS-REF
    DataTransferClassRef                                                   = 4606,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 115,
    /// DATA-TRANSFORMATION-KIND
    DataTransformationKind                                                 = 3336,
    /// DATA-TRANSFORMATION-REF
    DataTransformationRef                                                  = 1895,
    /// DATA-TRANSFORMATION-REF-CONDITIONAL
    DataTransformationRefConditional                                       = 397,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 1161,
    /// DATA-TRANSFORMATIONS
    DataTransformations                                                    = 5369,
    /// DATA-TYPE-FOR-SERIALIZATION-REF
    DataTypeForSerializationRef                                            = 2701,
    /// DATA-TYPE-FOR-SERIALIZATION-REFS
    DataTypeForSerializationRefs                                           = 5179,
    /// DATA-TYPE-MAP
    DataTypeMap                                                            = 319,
    /// DATA-TYPE-MAPPING-REF
    DataTypeMappingRef                                                     = 24,
    /// DATA-TYPE-MAPPING-REFS
    DataTypeMappingRefs                                                    = 3293,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 4664,
    /// DATA-TYPE-MAPPING-SET-REF
    DataTypeMappingSetRef                                                  = 4479,
    /// DATA-TYPE-MAPPING-SET-REFS
    DataTypeMappingSetRefs                                                 = 4969,
    /// DATA-TYPE-MAPS
    DataTypeMaps                                                           = 3124,
    /// DATA-TYPE-POLICY
    DataTypePolicy                                                         = 804,
    /// DATA-TYPE-REF
    DataTypeRef                                                            = 2575,
    /// DATA-TYPE-REFS
    DataTypeRefs                                                           = 4796,
    /// DATA-UPDATE-PERIOD
    DataUpdatePeriod                                                       = 276,
    /// DATA-WRITE-ACCESSS
    DataWriteAccesss                                                       = 1516,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 5220,
    /// DATE
    Date                                                                   = 1071,
    /// DCM-I-PDU
    DcmIPdu                                                                = 3906,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 3712,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 1271,
    /// DDS-EVENT-QOS-PROPS
    DdsEventQosProps                                                       = 2328,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 2687,
    /// DDS-FIELD-QOS-PROPS
    DdsFieldQosProps                                                       = 3455,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 4480,
    /// DDS-METHOD-QOS-PROPS
    DdsMethodQosProps                                                      = 2724,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 3902,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 2489,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 3760,
    /// DDS-RPC-SERVICE-REF
    DdsRpcServiceRef                                                       = 4527,
    /// DDS-RPC-SERVICES
    DdsRpcServices                                                         = 2872,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 391,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 2325,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 626,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 5527,
    /// DDS-SERVICE-VERSION
    DdsServiceVersion                                                      = 3895,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 3506,
    /// DEADLINE-MAX
    DeadlineMax                                                            = 4833,
    /// DEADLINE-MIN
    DeadlineMin                                                            = 4911,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 550,
    /// DEADLINE-SUPERVISIONS
    DeadlineSupervisions                                                   = 594,
    /// DEBOUNCE-ALGORITHM
    DebounceAlgorithm                                                      = 944,
    /// DEBOUNCE-ALGORITHM-PROPSS
    DebounceAlgorithmPropss                                                = 5141,
    /// DEBOUNCE-ALGORITHM-REF
    DebounceAlgorithmRef                                                   = 3133,
    /// DEBOUNCE-BEHAVIOR
    DebounceBehavior                                                       = 1159,
    /// DEBOUNCE-COUNTER-STORAGE
    DebounceCounterStorage                                                 = 4972,
    /// DEBOUNCE-TIME
    DebounceTime                                                           = 1063,
    /// DEBUG-INFOS
    DebugInfos                                                             = 5389,
    /// DECODING-CORRECTION
    DecodingCorrection                                                     = 747,
    /// DECOMPOSITIONS
    Decompositions                                                         = 307,
    /// DEF
    Def                                                                    = 185,
    /// DEF-CALPRM-SET
    DefCalprmSet                                                           = 4187,
    /// DEF-ITEM
    DefItem                                                                = 1300,
    /// DEF-LIST
    DefList                                                                = 468,
    /// DEFAULT-APPLICATION-TIMEOUT
    DefaultApplicationTimeout                                              = 1130,
    /// DEFAULT-ENDIANNESS
    DefaultEndianness                                                      = 4390,
    /// DEFAULT-GATEWAY
    DefaultGateway                                                         = 997,
    /// DEFAULT-LC-STATE-REF
    DefaultLcStateRef                                                      = 1431,
    /// DEFAULT-LEASE-TIME
    DefaultLeaseTime                                                       = 3417,
    /// DEFAULT-LOG-THRESHOLD
    DefaultLogThreshold                                                    = 2207,
    /// DEFAULT-MODE-REF
    DefaultModeRef                                                         = 2062,
    /// DEFAULT-PATTERN
    DefaultPattern                                                         = 228,
    /// DEFAULT-PERIOD-BEGIN
    DefaultPeriodBegin                                                     = 3121,
    /// DEFAULT-PERIOD-END
    DefaultPeriodEnd                                                       = 4436,
    /// DEFAULT-PRIORITY
    DefaultPriority                                                        = 4740,
    /// DEFAULT-REPORTING-MODE
    DefaultReportingMode                                                   = 2518,
    /// DEFAULT-ROUTER
    DefaultRouter                                                          = 592,
    /// DEFAULT-TRACE-STATE
    DefaultTraceState                                                      = 763,
    /// DEFAULT-VALUE
    DefaultValue                                                           = 4653,
    /// DEFAULT-VALUE-ELEMENT
    DefaultValueElement                                                    = 578,
    /// DEFAULT-VALUE-ELEMENTS
    DefaultValueElements                                                   = 3775,
    /// DEFAULT-VALUE-HANDLING
    DefaultValueHandling                                                   = 4765,
    /// DEFAULT-VLAN-REF
    DefaultVlanRef                                                         = 4264,
    /// DEFERRING-FID-REF
    DeferringFidRef                                                        = 961,
    /// DEFERRING-FID-REFS
    DeferringFidRefs                                                       = 994,
    /// DEFINITION-REF
    DefinitionRef                                                          = 1794,
    /// DELAY
    Delay                                                                  = 4759,
    /// DELAY-COMPENSATION-A
    DelayCompensationA                                                     = 1883,
    /// DELAY-COMPENSATION-B
    DelayCompensationB                                                     = 183,
    /// DELEGATED-PORT-ANNOTATION
    DelegatedPortAnnotation                                                = 3207,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 4793,
    /// DELIVERY-ARTIFACTS
    DeliveryArtifacts                                                      = 4625,
    /// DELTA-PACKAGE-APPLICABLE-VERSION
    DeltaPackageApplicableVersion                                          = 3309,
    /// DENOMINATOR-CONDITION
    DenominatorCondition                                                   = 5466,
    /// DENOMINATOR-GROUP
    DenominatorGroup                                                       = 405,
    /// DEPENDENCIES
    Dependencies                                                           = 3250,
    /// DEPENDENCY
    Dependency                                                             = 988,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 112,
    /// DEPENDENT-RESOURCES
    DependentResources                                                     = 211,
    /// DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF
    DependentSoftwareClusterDesignRef                                      = 3072,
    /// DEPENDENT-SOFTWARE-CLUSTER-REF
    DependentSoftwareClusterRef                                            = 4339,
    /// DEPENDS-ON
    DependsOn                                                              = 5056,
    /// DEPENDS-ON-REF
    DependsOnRef                                                           = 1245,
    /// DEPENDS-ON-REFS
    DependsOnRefs                                                          = 1940,
    /// DEPENDS-ONS
    DependsOns                                                             = 4447,
    /// DEPLOYMENT-REF
    DeploymentRef                                                          = 236,
    /// DERIVATION
    Derivation                                                             = 3355,
    /// DERIVED-CERTIFICATE-ACCEPTED
    DerivedCertificateAccepted                                             = 1956,
    /// DERIVED-FROM-BLUEPRINT-REF
    DerivedFromBlueprintRef                                                = 411,
    /// DERIVED-FROM-BLUEPRINT-REFS
    DerivedFromBlueprintRefs                                               = 3221,
    /// DERIVED-INTERFACE-REF
    DerivedInterfaceRef                                                    = 1744,
    /// DERIVED-OBJECT-REF
    DerivedObjectRef                                                       = 1388,
    /// DERIVED-PORT-INTERFACE-REF
    DerivedPortInterfaceRef                                                = 5113,
    /// DERIVED-PORT-PROTOTYPE-REF
    DerivedPortPrototypeRef                                                = 462,
    /// DESC
    Desc                                                                   = 2345,
    /// DESCRIPTION
    Description                                                            = 1560,
    /// DESIGN-REF
    DesignRef                                                              = 2357,
    /// DESIGN-REFS
    DesignRefs                                                             = 4115,
    /// DEST-META-CLASS
    DestMetaClass                                                          = 423,
    /// DEST-SDG-REF
    DestSdgRef                                                             = 3649,
    /// DESTINATION-BASE-ID
    DestinationBaseId                                                      = 561,
    /// DESTINATION-CONTEXT
    DestinationContext                                                     = 4749,
    /// DESTINATION-PHYSICAL-ADDRESS
    DestinationPhysicalAddress                                             = 1945,
    /// DESTINATION-REF
    DestinationRef                                                         = 5364,
    /// DESTINATION-REFS
    DestinationRefs                                                        = 2438,
    /// DESTINATION-TYPE
    DestinationType                                                        = 542,
    /// DESTINATION-URI-DEFS
    DestinationUriDefs                                                     = 5072,
    /// DESTINATION-URI-NESTING-CONTRACT
    DestinationUriNestingContract                                          = 4889,
    /// DESTINATION-URI-POLICY
    DestinationUriPolicy                                                   = 3919,
    /// DESTINATION-URI-REF
    DestinationUriRef                                                      = 5407,
    /// DESTINATION-URI-REFS
    DestinationUriRefs                                                     = 3266,
    /// DETECT-NIT-ERROR
    DetectNitError                                                         = 2970,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 3479,
    /// DETERMINISTIC-CLIENT-REF
    DeterministicClientRef                                                 = 5549,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 2024,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDSS
    DeterministicClientResourceNeedss                                      = 5642,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 4204,
    /// DEVELOPMENT-VALUE
    DevelopmentValue                                                       = 5520,
    /// DHCP-ADDRESS-ASSIGNMENT
    DhcpAddressAssignment                                                  = 5528,
    /// DHCP-PROPS
    DhcpProps                                                              = 3653,
    /// DHCP-SERVER-CONFIGURATION
    DhcpServerConfiguration                                                = 3367,
    /// DIAG-ARG-INTEGRITY
    DiagArgIntegrity                                                       = 274,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 4247,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 5458,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 2009,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 1607,
    /// DIAG-FAULT-DETECTION-COUNTER-PORT-REF
    DiagFaultDetectionCounterPortRef                                       = 1152,
    /// DIAG-PDU-TYPE
    DiagPduType                                                            = 2713,
    /// DIAG-REQUIREMENT
    DiagRequirement                                                        = 1746,
    /// DIAG-ROUTINE-TYPE
    DiagRoutineType                                                        = 1911,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 4113,
    /// DIAGNOSTIC-ADDRESS
    DiagnosticAddress                                                      = 2864,
    /// DIAGNOSTIC-ADDRESSS
    DiagnosticAddresss                                                     = 4644,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 3941,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 2349,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 3589,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 1522,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 1748,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 1905,
    /// DIAGNOSTIC-AUTHENTICATION-REF
    DiagnosticAuthenticationRef                                            = 4409,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 653,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 5323,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 4399,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 801,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 3205,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 1180,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 1412,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 819,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 8,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 2238,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 2709,
    /// DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL
    DiagnosticComControlSpecificChannel                                    = 1324,
    /// DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL
    DiagnosticComControlSubNodeChannel                                     = 383,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF
    DiagnosticCommonElementRef                                             = 3727,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL
    DiagnosticCommonElementRefConditional                                  = 1972,
    /// DIAGNOSTIC-COMMON-PROPS-CONDITIONAL
    DiagnosticCommonPropsConditional                                       = 2075,
    /// DIAGNOSTIC-COMMON-PROPS-VARIANTS
    DiagnosticCommonPropsVariants                                          = 5415,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 4082,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 894,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 4547,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 1302,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 2744,
    /// DIAGNOSTIC-CONNECTION-REF
    DiagnosticConnectionRef                                                = 5305,
    /// DIAGNOSTIC-CONNECTION-REF-CONDITIONAL
    DiagnosticConnectionRefConditional                                     = 2335,
    /// DIAGNOSTIC-CONNECTIONS
    DiagnosticConnections                                                  = 5053,
    /// DIAGNOSTIC-CONTRIBUTION-REF
    DiagnosticContributionRef                                              = 4063,
    /// DIAGNOSTIC-CONTRIBUTION-REFS
    DiagnosticContributionRefs                                             = 4817,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 1561,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 1853,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 643,
    /// DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT
    DiagnosticControlEnableMaskBit                                         = 2622,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 657,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 3114,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 4319,
    /// DIAGNOSTIC-DATA-CHANGE-TRIGGER
    DiagnosticDataChangeTrigger                                            = 4859,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 4294,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 5604,
    /// DIAGNOSTIC-DATA-ELEMENT-REF
    DiagnosticDataElementRef                                               = 4179,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 2018,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 966,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 2310,
    /// DIAGNOSTIC-DATA-IDENTIFIER-REF
    DiagnosticDataIdentifierRef                                            = 147,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 3814,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF
    DiagnosticDataIdentifierSetRef                                         = 36,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL
    DiagnosticDataIdentifierSetRefConditional                              = 574,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 5074,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 1229,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 4162,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 5329,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 1933,
    /// DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT
    DiagnosticDebounceBehaviorEnumValueVariationPoint                      = 142,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 3180,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 5247,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 4832,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 245,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 398,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 38,
    /// DIAGNOSTIC-DTC-CHANGE-TRIGGER
    DiagnosticDtcChangeTrigger                                             = 2651,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 1336,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 2135,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 4694,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 4119,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 4364,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 2476,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 1075,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 5309,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 4763,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 2800,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 878,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 4674,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF
    DiagnosticEnableConditionRef                                           = 386,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL
    DiagnosticEnableConditionRefConditional                                = 5132,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 421,
    /// DIAGNOSTIC-ENV-CONDITION-FORMULA
    DiagnosticEnvConditionFormula                                          = 1510,
    /// DIAGNOSTIC-ENV-DATA-CONDITION
    DiagnosticEnvDataCondition                                             = 5068,
    /// DIAGNOSTIC-ENV-MODE-CONDITION
    DiagnosticEnvModeCondition                                             = 2897,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 1163,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 4000,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 1474,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 3018,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 309,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 5204,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 5273,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 4263,
    /// DIAGNOSTIC-EVENT-REF
    DiagnosticEventRef                                                     = 5426,
    /// DIAGNOSTIC-EVENT-REF-CONDITIONAL
    DiagnosticEventRefConditional                                          = 860,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 1691,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 5177,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 1179,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 2011,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 1046,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 3418,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 1657,
    /// DIAGNOSTIC-EVENT-WINDOW
    DiagnosticEventWindow                                                  = 2257,
    /// DIAGNOSTIC-EVENTS
    DiagnosticEvents                                                       = 4638,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 2783,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF
    DiagnosticExtendedDataRecordRef                                        = 4267,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL
    DiagnosticExtendedDataRecordRefConditional                             = 2026,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION
    DiagnosticExternalAuthenticationIdentification                         = 501,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 1936,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 3043,
    /// DIAGNOSTIC-EXTRACT-REF
    DiagnosticExtractRef                                                   = 4049,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 879,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 2490,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 1885,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 1958,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 705,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 3264,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 3356,
    /// DIAGNOSTIC-FREEZE-FRAME-REF
    DiagnosticFreezeFrameRef                                               = 2873,
    /// DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL
    DiagnosticFreezeFrameRefConditional                                    = 5303,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 1763,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 1220,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 2728,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 707,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 3150,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 3358,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 978,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 1602,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 2104,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 3491,
    /// DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT
    DiagnosticIndicatorTypeEnumValueVariationPoint                         = 3346,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 2515,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 2485,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 2824,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 2190,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 2908,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 5601,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 1192,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 2659,
    /// DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER
    DiagnosticIumprGroupIdentifier                                         = 4901,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 5266,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 4556,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 1801,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 1760,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 489,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 4864,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 222,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 1486,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 2503,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 1584,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 885,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 1475,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 2700,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 2449,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 1176,
    /// DIAGNOSTIC-MEMORY-REF
    DiagnosticMemoryRef                                                    = 3436,
    /// DIAGNOSTIC-MESSAGE-TYPE
    DiagnosticMessageType                                                  = 2779,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 480,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 4077,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 2501,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 261,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 1044,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 2337,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF
    DiagnosticOperationCycleRef                                            = 633,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL
    DiagnosticOperationCycleRefConditional                                 = 1299,
    /// DIAGNOSTIC-PARAMETER
    DiagnosticParameter                                                    = 4550,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 1447,
    /// DIAGNOSTIC-PERIODIC-RATE
    DiagnosticPeriodicRate                                                 = 3854,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 4915,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 92,
    /// DIAGNOSTIC-PROPS
    DiagnosticProps                                                        = 1059,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 5250,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 2178,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 4872,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 5641,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 2204,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 1089,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 2256,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 3492,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 4928,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 1380,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 4027,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 3360,
    /// DIAGNOSTIC-RELEVANCE
    DiagnosticRelevance                                                    = 5272,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 3350,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 1175,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 5041,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 5109,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 3907,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 1761,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 4836,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 4380,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 5231,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 499,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 5069,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 604,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 1989,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 4349,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 2980,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 5345,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 846,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 466,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 3987,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 3535,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 2353,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 3828,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 540,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 5095,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 2934,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 2971,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 2261,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 3758,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 3452,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 1724,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 3599,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 471,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 916,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 4277,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 190,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 1536,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 3148,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 4189,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 1697,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 2304,
    /// DIAGNOSTIC-SERVICE-INSTANCE-REF
    DiagnosticServiceInstanceRef                                           = 3548,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 407,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 2757,
    /// DIAGNOSTIC-SERVICE-TABLE-REF
    DiagnosticServiceTableRef                                              = 799,
    /// DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL
    DiagnosticServiceTableRefConditional                                   = 4239,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 5354,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 4583,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 1717,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 1002,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 4359,
    /// DIAGNOSTIC-SESSION-REF
    DiagnosticSessionRef                                                   = 706,
    /// DIAGNOSTIC-SESSION-REFS
    DiagnosticSessionRefs                                                  = 2276,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 4776,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 2278,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 2863,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 2442,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 870,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF
    DiagnosticStorageConditionRef                                          = 4632,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL
    DiagnosticStorageConditionRefConditional                               = 355,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 1043,
    /// DIAGNOSTIC-TEST-RESULT-REF
    DiagnosticTestResultRef                                                = 4219,
    /// DIAGNOSTIC-TEST-RESULT-REFS
    DiagnosticTestResultRefs                                               = 3763,
    /// DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT
    DiagnosticTestResultUpdateEnumValueVariationPoint                      = 414,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 3801,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 2505,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 1623,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 4936,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 1357,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 863,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 3283,
    /// DIAGNOSTIC-TROUBLE-CODE-REF
    DiagnosticTroubleCodeRef                                               = 1530,
    /// DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL
    DiagnosticTroubleCodeRefConditional                                    = 4120,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 3136,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 5258,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 1283,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 2942,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 5269,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 4236,
    /// DIAGNOSTIC-VALUE-ACCESS
    DiagnosticValueAccess                                                  = 4186,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 1316,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 5593,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 1195,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 3886,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 2184,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 553,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 1532,
    /// DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT
    DiagnosticWwhObdDtcClassEnumValueVariationPoint                        = 5045,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 532,
    /// DID-NUMBER
    DidNumber                                                              = 1682,
    /// DID-SIZE
    DidSize                                                                = 479,
    /// DIFFERENTIATED-SERVICE-FIELD
    DifferentiatedServiceField                                             = 2016,
    /// DIRECT-PDU-REF
    DirectPduRef                                                           = 4916,
    /// DIRECT-SUCCESSOR-REF
    DirectSuccessorRef                                                     = 2395,
    /// DIRECT-SUCCESSOR-REFS
    DirectSuccessorRefs                                                    = 2523,
    /// DIRECT-TP-SDU-REF
    DirectTpSduRef                                                         = 412,
    /// DIRECTION
    Direction                                                              = 1694,
    /// DISABLE-END-TO-END-CHECK
    DisableEndToEndCheck                                                   = 4970,
    /// DISABLE-END-TO-END-STATE-MACHINE
    DisableEndToEndStateMachine                                            = 5240,
    /// DISABLED-IN-MODE-IREF
    DisabledInModeIref                                                     = 2068,
    /// DISABLED-IN-MODE-IREFS
    DisabledInModeIrefs                                                    = 3876,
    /// DISABLED-MODE-IREF
    DisabledModeIref                                                       = 740,
    /// DISABLED-MODE-IREFS
    DisabledModeIrefs                                                      = 675,
    /// DISCOVERY-PROTECTION-KIND
    DiscoveryProtectionKind                                                = 2552,
    /// DISCOVERY-TECHNOLOGY
    DiscoveryTechnology                                                    = 4797,
    /// DISCOVERY-TYPE
    DiscoveryType                                                          = 5203,
    /// DISPLAY-FORMAT
    DisplayFormat                                                          = 116,
    /// DISPLAY-IDENTIFIER
    DisplayIdentifier                                                      = 2413,
    /// DISPLAY-NAME
    DisplayName                                                            = 2583,
    /// DISPLAY-PRESENTATION
    DisplayPresentation                                                    = 511,
    /// DISTINGUISHED-PARTITIONS
    DistinguishedPartitions                                                = 403,
    /// DLT-APPLICATION
    DltApplication                                                         = 4416,
    /// DLT-APPLICATION-REF
    DltApplicationRef                                                      = 862,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 3097,
    /// DLT-ARGUMENT
    DltArgument                                                            = 3735,
    /// DLT-ARGUMENT-ENTRYS
    DltArgumentEntrys                                                      = 1236,
    /// DLT-ARGUMENTS
    DltArguments                                                           = 695,
    /// DLT-CONFIG
    DltConfig                                                              = 3057,
    /// DLT-CONTEXT
    DltContext                                                             = 3781,
    /// DLT-CONTEXT-REF
    DltContextRef                                                          = 244,
    /// DLT-CONTEXT-REF-CONDITIONAL
    DltContextRefConditional                                               = 46,
    /// DLT-ECU
    DltEcu                                                                 = 1985,
    /// DLT-ECU-ID
    DltEcuId                                                               = 5210,
    /// DLT-ECU-REF
    DltEcuRef                                                              = 2915,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 5402,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 3617,
    /// DLT-LOG-CHANNEL-DESIGN-REF
    DltLogChannelDesignRef                                                 = 159,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 5566,
    /// DLT-LOG-CHANNEL-REF
    DltLogChannelRef                                                       = 3482,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 1366,
    /// DLT-LOG-CHANNELS
    DltLogChannels                                                         = 4558,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 2243,
    /// DLT-LOG-SINK-REF
    DltLogSinkRef                                                          = 3404,
    /// DLT-LOG-SINK-REFS
    DltLogSinkRefs                                                         = 4047,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 666,
    /// DLT-MESSAGE
    DltMessage                                                             = 4890,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 2095,
    /// DLT-MESSAGE-REF
    DltMessageRef                                                          = 3521,
    /// DLT-MESSAGE-REF-CONDITIONAL
    DltMessageRefConditional                                               = 3827,
    /// DLT-MESSAGE-REFS
    DltMessageRefs                                                         = 886,
    /// DLT-MESSAGES
    DltMessages                                                            = 427,
    /// DLT-SESSION-ID
    DltSessionId                                                           = 1155,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 3559,
    /// DNS-SERVER-ADDRESS
    DnsServerAddress                                                       = 4177,
    /// DNS-SERVER-ADDRESSES
    DnsServerAddresses                                                     = 4301,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 4446,
    /// DO-IP-CONFIG
    DoIpConfig                                                             = 4851,
    /// DO-IP-ENTITY
    DoIpEntity                                                             = 2546,
    /// DO-IP-ENTITY-ROLE
    DoIpEntityRole                                                         = 1646,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 5111,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 5441,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 2102,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 5509,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 538,
    /// DO-IP-LOGIC-ADDRESS-PROPS
    DoIpLogicAddressProps                                                  = 1901,
    /// DO-IP-LOGIC-ADDRESSS
    DoIpLogicAddresss                                                      = 3576,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 4726,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 983,
    /// DO-IP-NETWORK-CONFIGURATION
    DoIpNetworkConfiguration                                               = 1872,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 5143,
    /// DO-IP-REQUEST-CONFIGURATION
    DoIpRequestConfiguration                                               = 5590,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 4474,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 1052,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 4262,
    /// DO-IP-ROUTING-ACTIVATIONS
    DoIpRoutingActivations                                                 = 3855,
    /// DO-IP-SOURCE-ADDRESS-REF
    DoIpSourceAddressRef                                                   = 4169,
    /// DO-IP-TARGET-ADDRESS-REF
    DoIpTargetAddressRef                                                   = 3232,
    /// DO-IP-TARGET-ADDRESS-REFS
    DoIpTargetAddressRefs                                                  = 4254,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REF
    DoIpTesterRoutingActivationRef                                         = 1487,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REFS
    DoIpTesterRoutingActivationRefs                                        = 4444,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 4983,
    /// DO-IP-TP-CONNECTION
    DoIpTpConnection                                                       = 1375,
    /// DOC-REVISION
    DocRevision                                                            = 4161,
    /// DOC-REVISIONS
    DocRevisions                                                           = 2719,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 1129,
    /// DOCUMENT-ELEMENT-SCOPES
    DocumentElementScopes                                                  = 3332,
    /// DOCUMENTATION
    Documentation                                                          = 3372,
    /// DOCUMENTATION-CONTENT
    DocumentationContent                                                   = 1767,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 4979,
    /// DOIP-CHANNEL-COLLECTION-REF
    DoipChannelCollectionRef                                               = 3771,
    /// DOIP-CONNECTION-REF
    DoipConnectionRef                                                      = 4246,
    /// DOIP-CONNECTION-REFS
    DoipConnectionRefs                                                     = 453,
    /// DOIP-INTERFACES
    DoipInterfaces                                                         = 4424,
    /// DOMAIN
    Domain                                                                 = 2329,
    /// DOMAIN-ID
    DomainId                                                               = 5277,
    /// DOMAIN-IDS
    DomainIds                                                              = 55,
    /// DPD-ACTION
    DpdAction                                                              = 4603,
    /// DPD-DELAY
    DpdDelay                                                               = 4716,
    /// DPG-DOES-NOT-REQUIRE-COHERENCYS
    DpgDoesNotRequireCoherencys                                            = 2598,
    /// DPG-REQUIRES-COHERENCYS
    DpgRequiresCoherencys                                                  = 4866,
    /// DRIVER-NOTIFICATIONS
    DriverNotifications                                                    = 2627,
    /// DRIVER-PRIMITIVE-REF
    DriverPrimitiveRef                                                     = 859,
    /// DROP-NOT-REQUESTED-NAD
    DropNotRequestedNad                                                    = 1218,
    /// DTC-FORMAT-TYPE
    DtcFormatType                                                          = 2814,
    /// DTC-KIND
    DtcKind                                                                = 233,
    /// DTC-NUMBER
    DtcNumber                                                              = 3568,
    /// DTC-PROPS-REF
    DtcPropsRef                                                            = 1009,
    /// DTC-SETTING-CLASS-REF
    DtcSettingClassRef                                                     = 1367,
    /// DTC-SETTING-PARAMETER
    DtcSettingParameter                                                    = 3956,
    /// DTC-STATUS-AVAILABILITY-MASK
    DtcStatusAvailabilityMask                                              = 2160,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 3674,
    /// DTC-STATUS-MASK
    DtcStatusMask                                                          = 2436,
    /// DTCS
    Dtcs                                                                   = 3440,
    /// DYNAMIC-ACTION-REF
    DynamicActionRef                                                       = 4593,
    /// DYNAMIC-ACTION-REFS
    DynamicActionRefs                                                      = 1092,
    /// DYNAMIC-ARRAY-SIZE-PROFILE
    DynamicArraySizeProfile                                                = 1799,
    /// DYNAMIC-BS
    DynamicBs                                                              = 1326,
    /// DYNAMIC-LENGTH
    DynamicLength                                                          = 4888,
    /// DYNAMIC-PART
    DynamicPart                                                            = 4580,
    /// DYNAMIC-PART-ALTERNATIVE
    DynamicPartAlternative                                                 = 676,
    /// DYNAMIC-PART-ALTERNATIVES
    DynamicPartAlternatives                                                = 4139,
    /// DYNAMIC-PARTS
    DynamicParts                                                           = 311,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REF
    DynamicPncMappingPduGroupRef                                           = 456,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS
    DynamicPncMappingPduGroupRefs                                          = 3528,
    /// DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED
    DynamicPncToChannelMappingEnabled                                      = 150,
    /// DYNAMIC-SLOT-IDLE-PHASE
    DynamicSlotIdlePhase                                                   = 3823,
    /// DYNAMICALLY-ASSIGNED
    DynamicallyAssigned                                                    = 3833,
    /// DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF
    DynamicallyDefineDataIdentifierClassRef                                = 1372,
    /// E
    E                                                                      = 2498,
    /// E-2-E-EVENT-PROTECTION-PROPSS
    E2EEventProtectionPropss                                               = 2818,
    /// E-2-E-METHOD-PROTECTION-PROPSS
    E2EMethodProtectionPropss                                              = 3279,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 4065,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS-REF
    E2EProfileCompatibilityPropsRef                                        = 1667,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 1259,
    /// E-2-E-PROFILE-CONFIGURATION-REF
    E2EProfileConfigurationRef                                             = 5169,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 2153,
    /// E-2-E-PROFILE-CONFIGURATIONS
    E2EProfileConfigurations                                               = 3158,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 5655,
    /// ECU-COMM-PORT-INSTANCES
    EcuCommPortInstances                                                   = 2293,
    /// ECU-CONFIGURATION-REF
    EcuConfigurationRef                                                    = 2203,
    /// ECU-EXTRACT-REF
    EcuExtractRef                                                          = 1981,
    /// ECU-EXTRACT-REFERENCE-IREF
    EcuExtractReferenceIref                                                = 1110,
    /// ECU-EXTRACT-VERSION
    EcuExtractVersion                                                      = 1975,
    /// ECU-ID
    EcuId                                                                  = 4233,
    /// ECU-INSTANCE
    EcuInstance                                                            = 512,
    /// ECU-INSTANCE-PROPS
    EcuInstanceProps                                                       = 2947,
    /// ECU-INSTANCE-PROPSS
    EcuInstancePropss                                                      = 4952,
    /// ECU-INSTANCE-REF
    EcuInstanceRef                                                         = 4865,
    /// ECU-INSTANCE-REF-CONDITIONAL
    EcuInstanceRefConditional                                              = 4157,
    /// ECU-INSTANCE-REFS
    EcuInstanceRefs                                                        = 5659,
    /// ECU-INSTANCES
    EcuInstances                                                           = 3768,
    /// ECU-MAPPING
    EcuMapping                                                             = 2743,
    /// ECU-PARTITION
    EcuPartition                                                           = 1544,
    /// ECU-PARTITION-REF
    EcuPartitionRef                                                        = 4199,
    /// ECU-REF
    EcuRef                                                                 = 133,
    /// ECU-RESET-CLASS-REF
    EcuResetClassRef                                                       = 1745,
    /// ECU-RESOURCE-ESTIMATION
    EcuResourceEstimation                                                  = 4600,
    /// ECU-RESOURCE-MAPPINGS
    EcuResourceMappings                                                    = 4975,
    /// ECU-SCOPE-REF
    EcuScopeRef                                                            = 137,
    /// ECU-SCOPE-REFS
    EcuScopeRefs                                                           = 4413,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 5510,
    /// ECU-TASK-PROXY-REF
    EcuTaskProxyRef                                                        = 1423,
    /// ECU-TASK-PROXY-REFS
    EcuTaskProxyRefs                                                       = 5583,
    /// ECU-TIMING
    EcuTiming                                                              = 2939,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 2406,
    /// ECUC-ADD-INFO-PARAM-VALUE
    EcucAddInfoParamValue                                                  = 172,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 3024,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 362,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 1260,
    /// ECUC-COND
    EcucCond                                                               = 5630,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 77,
    /// ECUC-DEF-EDITION
    EcucDefEdition                                                         = 2834,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 5620,
    /// ECUC-DEFINITION-REF
    EcucDefinitionRef                                                      = 5393,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 1216,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 4708,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 1977,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 4943,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 3489,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 5136,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 5229,
    /// ECUC-FUNCTION-NAME-DEF-CONDITIONAL
    EcucFunctionNameDefConditional                                         = 5619,
    /// ECUC-FUNCTION-NAME-DEF-VARIANTS
    EcucFunctionNameDefVariants                                            = 3720,
    /// ECUC-IMPLEMENTATION-CONFIGURATION-CLASS
    EcucImplementationConfigurationClass                                   = 1230,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 3938,
    /// ECUC-INSTANCE-REFERENCE-VALUE
    EcucInstanceReferenceValue                                             = 1404,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 5610,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 3296,
    /// ECUC-LINKER-SYMBOL-DEF-CONDITIONAL
    EcucLinkerSymbolDefConditional                                         = 770,
    /// ECUC-LINKER-SYMBOL-DEF-VARIANTS
    EcucLinkerSymbolDefVariants                                            = 1332,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 4904,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF
    EcucModuleConfigurationValuesRef                                       = 3552,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL
    EcucModuleConfigurationValuesRefConditional                            = 1370,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 5538,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 5198,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL
    EcucMultilineStringParamDefConditional                                 = 1920,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS
    EcucMultilineStringParamDefVariants                                    = 2417,
    /// ECUC-MULTIPLICITY-CONFIGURATION-CLASS
    EcucMultiplicityConfigurationClass                                     = 1683,
    /// ECUC-NUMERICAL-PARAM-VALUE
    EcucNumericalParamValue                                                = 2538,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 5006,
    /// ECUC-QUERY
    EcucQuery                                                              = 2445,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 2681,
    /// ECUC-QUERY-REF
    EcucQueryRef                                                           = 2625,
    /// ECUC-QUERY-STRING-REF
    EcucQueryStringRef                                                     = 1997,
    /// ECUC-QUERYS
    EcucQuerys                                                             = 85,
    /// ECUC-REF
    EcucRef                                                                = 4464,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 930,
    /// ECUC-REFERENCE-VALUE
    EcucReferenceValue                                                     = 5357,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 4327,
    /// ECUC-STRING-PARAM-DEF-CONDITIONAL
    EcucStringParamDefConditional                                          = 2339,
    /// ECUC-STRING-PARAM-DEF-VARIANTS
    EcucStringParamDefVariants                                             = 351,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 5036,
    /// ECUC-TEXTUAL-PARAM-VALUE
    EcucTextualParamValue                                                  = 5162,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 3630,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 4695,
    /// ECUC-VALIDATION-CONDS
    EcucValidationConds                                                    = 160,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 4722,
    /// ECUC-VALUE-CONFIGURATION-CLASS
    EcucValueConfigurationClass                                            = 3870,
    /// ECUC-VALUES
    EcucValues                                                             = 4316,
    /// EID
    Eid                                                                    = 1654,
    /// EID-USE-MAC
    EidUseMac                                                              = 2397,
    /// ELEMENT
    Element                                                                = 2653,
    /// ELEMENT-BYTE-VALUE
    ElementByteValue                                                       = 2419,
    /// ELEMENT-GROUPS
    ElementGroups                                                          = 845,
    /// ELEMENT-IN-IMPL-DATATYPE
    ElementInImplDatatype                                                  = 2148,
    /// ELEMENT-MAPPINGS
    ElementMappings                                                        = 1643,
    /// ELEMENT-POSITION
    ElementPosition                                                        = 2064,
    /// ELEMENT-PROPSS
    ElementPropss                                                          = 3109,
    /// ELEMENT-REF
    ElementRef                                                             = 4492,
    /// ELEMENT-REFS
    ElementRefs                                                            = 5217,
    /// ELEMENT-ROLE
    ElementRole                                                            = 3175,
    /// ELEMENTS
    Elements                                                               = 3898,
    /// ELLIPTIC-CURVE-REF
    EllipticCurveRef                                                       = 2157,
    /// ELLIPTIC-CURVE-REFS
    EllipticCurveRefs                                                      = 4875,
    /// EMPTY-SIGNAL
    EmptySignal                                                            = 3679,
    /// EMULATION-SUPPORTS
    EmulationSupports                                                      = 2605,
    /// ENABLE-ANYCAST
    EnableAnycast                                                          = 668,
    /// ENABLE-CONDITION-GROUP-REF
    EnableConditionGroupRef                                                = 3764,
    /// ENABLE-CONDITION-REF
    EnableConditionRef                                                     = 1838,
    /// ENABLE-CONDITIONS
    EnableConditions                                                       = 813,
    /// ENABLE-DEACTIVATION
    EnableDeactivation                                                     = 4730,
    /// ENABLE-DISCOVERY-PROTECTION
    EnableDiscoveryProtection                                              = 4369,
    /// ENABLE-JOIN-ACCESS-CONTROL
    EnableJoinAccessControl                                                = 385,
    /// ENABLE-LIVELINESS-PROTECTION
    EnableLivelinessProtection                                             = 934,
    /// ENABLE-READ-ACCESS-CONTROL
    EnableReadAccessControl                                                = 2731,
    /// ENABLE-TAKE-ADDRESS
    EnableTakeAddress                                                      = 356,
    /// ENABLE-UPDATE
    EnableUpdate                                                           = 3341,
    /// ENABLE-WRITE-ACCESS-CONTROL
    EnableWriteAccessControl                                               = 2107,
    /// ENABLES-LOG-TRACE
    EnablesLogTrace                                                        = 2533,
    /// ENCAPSULATED-ENTRY-REF
    EncapsulatedEntryRef                                                   = 3856,
    /// ENCODING
    Encoding                                                               = 32,
    /// ENCRYPTION-REF
    EncryptionRef                                                          = 4967,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 1994,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 2306,
    /// END-ADDRESS
    EndAddress                                                             = 361,
    /// END-TO-END-CALL-RESPONSE-TIMEOUT
    EndToEndCallResponseTimeout                                            = 1818,
    /// END-TO-END-PROFILE
    EndToEndProfile                                                        = 2670,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 2764,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 1539,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDUS
    EndToEndProtectionISignalIPdus                                         = 5570,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 1962,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 3098,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPES
    EndToEndProtectionVariablePrototypes                                   = 4043,
    /// END-TO-END-PROTECTIONS
    EndToEndProtections                                                    = 1319,
    /// END-TO-END-TRANSFORMATION-COM-SPEC-PROPS
    EndToEndTransformationComSpecProps                                     = 3252,
    /// END-TO-END-TRANSFORMATION-DESCRIPTION
    EndToEndTransformationDescription                                      = 4481,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS
    EndToEndTransformationISignalProps                                     = 603,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    EndToEndTransformationISignalPropsConditional                          = 2577,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    EndToEndTransformationISignalPropsVariants                             = 3462,
    /// ENDPOINT-CONFIGURATION-REF
    EndpointConfigurationRef                                               = 2042,
    /// ENDPOINTS
    Endpoints                                                              = 196,
    /// ENGINEERING-OBJECT
    EngineeringObject                                                      = 3709,
    /// ENGINEERING-OBJECTS
    EngineeringObjects                                                     = 3105,
    /// ENHANCED-MODE-API
    EnhancedModeApi                                                        = 4534,
    /// ENTER-TIMEOUT-VALUE
    EnterTimeoutValue                                                      = 4835,
    /// ENTERED-MODE-REF
    EnteredModeRef                                                         = 2858,
    /// ENTITY-STATUS-MAX-BYTE-FIELD-USE
    EntityStatusMaxByteFieldUse                                            = 80,
    /// ENTITYS
    Entitys                                                                = 4284,
    /// ENTRY
    Entry                                                                  = 3449,
    /// ENTRY-MODE-DECLARATION-REF
    EntryModeDeclarationRef                                                = 432,
    /// ENTRY-REF
    EntryRef                                                               = 608,
    /// ENTRYS
    Entrys                                                                 = 513,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 2227,
    /// ENUMERATOR-VALUE
    EnumeratorValue                                                        = 2092,
    /// ENUMS
    Enums                                                                  = 5234,
    /// ENVIRONMENT-CAPTURE-TO-REPORTING
    EnvironmentCaptureToReporting                                          = 2776,
    /// ENVIRONMENT-DATA-CAPTURE
    EnvironmentDataCapture                                                 = 3944,
    /// ENVIRONMENT-VARIABLES
    EnvironmentVariables                                                   = 3826,
    /// ENVIRONMENTAL-CONDITION-REF
    EnvironmentalConditionRef                                              = 3323,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 275,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 2118,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 4026,
    /// ERROR-CODE
    ErrorCode                                                              = 5447,
    /// ERROR-CONTEXT-REF
    ErrorContextRef                                                        = 790,
    /// ERROR-CONTEXT-REFS
    ErrorContextRefs                                                       = 5182,
    /// ERROR-DOMAIN-REF
    ErrorDomainRef                                                         = 3231,
    /// ERROR-HANDLING
    ErrorHandling                                                          = 651,
    /// ERROR-MAPPINGS
    ErrorMappings                                                          = 1111,
    /// ERROR-REACTION-POLICY
    ErrorReactionPolicy                                                    = 1988,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 3783,
    /// ESP-CIPHER-SUITE-NAME
    EspCipherSuiteName                                                     = 3217,
    /// ESP-CIPHER-SUITE-NAMES
    EspCipherSuiteNames                                                    = 1842,
    /// ESTIMATED-DURATION-OF-CAMPAIGN
    EstimatedDurationOfCampaign                                            = 2012,
    /// ESTIMATED-DURATION-OF-OPERATION
    EstimatedDurationOfOperation                                           = 5397,
    /// ESTIMATED-EXECUTION-TIME
    EstimatedExecutionTime                                                 = 5427,
    /// ETH-GLOBAL-TIME-DOMAIN-PROPS
    EthGlobalTimeDomainProps                                               = 4426,
    /// ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT
    EthGlobalTimeManagedCouplingPort                                       = 1824,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 3081,
    /// ETH-IP-PROPS-REF
    EthIpPropsRef                                                          = 4067,
    /// ETH-SWITCH-PORT-GROUP-DERIVATION
    EthSwitchPortGroupDerivation                                           = 5387,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 4355,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 4207,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 435,
    /// ETH-TP-CONNECTION
    EthTpConnection                                                        = 2689,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 3766,
    /// ETHERNET-CLUSTER-CONDITIONAL
    EthernetClusterConditional                                             = 4130,
    /// ETHERNET-CLUSTER-VARIANTS
    EthernetClusterVariants                                                = 3460,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 1422,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 2504,
    /// ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL
    EthernetCommunicationControllerConditional                             = 7,
    /// ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS
    EthernetCommunicationControllerVariants                                = 1483,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 3073,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 318,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 3427,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 418,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 1889,
    /// ETHERNET-PRIORITY-REGENERATIONS
    EthernetPriorityRegenerations                                          = 1666,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 128,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 118,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 5492,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING-REF
    EthernetRawDataStreamMappingRef                                        = 388,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 5077,
    /// ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS
    EthernetTrafficClassAssignments                                        = 4203,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 1277,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 1537,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS
    EthernetWakeupSleepOnDatalineConfigs                                   = 1881,
    /// EVALUATED-ELEMENT-REF
    EvaluatedElementRef                                                    = 4879,
    /// EVALUATED-ELEMENT-REFS
    EvaluatedElementRefs                                                   = 3969,
    /// EVALUATED-VARIANT-REF
    EvaluatedVariantRef                                                    = 1461,
    /// EVALUATED-VARIANT-REFS
    EvaluatedVariantRefs                                                   = 3585,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 3335,
    /// EVENT-CLEAR-ALLOWED
    EventClearAllowed                                                      = 4257,
    /// EVENT-COMBINATION-REPORTING-BEHAVIOR
    EventCombinationReportingBehavior                                      = 1267,
    /// EVENT-CONTROLLED-TIMING
    EventControlledTiming                                                  = 5478,
    /// EVENT-DEPLOYMENTS
    EventDeployments                                                       = 98,
    /// EVENT-DISPLACEMENT-STRATEGY
    EventDisplacementStrategy                                              = 5446,
    /// EVENT-ELEMENT-MAPPINGS
    EventElementMappings                                                   = 1946,
    /// EVENT-ELEMENTS
    EventElements                                                          = 2019,
    /// EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD
    EventFailureCycleCounterThreshold                                      = 2674,
    /// EVENT-GROUP-CONTROL-TYPE
    EventGroupControlType                                                  = 2418,
    /// EVENT-GROUP-ID
    EventGroupId                                                           = 2456,
    /// EVENT-GROUP-IDENTIFIER
    EventGroupIdentifier                                                   = 4134,
    /// EVENT-GROUP-REF
    EventGroupRef                                                          = 2649,
    /// EVENT-GROUP-REFS
    EventGroupRefs                                                         = 3146,
    /// EVENT-GROUPS
    EventGroups                                                            = 2308,
    /// EVENT-HANDLER
    EventHandler                                                           = 3686,
    /// EVENT-HANDLER-REF
    EventHandlerRef                                                        = 3538,
    /// EVENT-HANDLER-REFS
    EventHandlerRefs                                                       = 4826,
    /// EVENT-HANDLERS
    EventHandlers                                                          = 1955,
    /// EVENT-ID
    EventId                                                                = 4785,
    /// EVENT-IREF
    EventIref                                                              = 3077,
    /// EVENT-KIND
    EventKind                                                              = 4334,
    /// EVENT-MAPPING
    EventMapping                                                           = 2037,
    /// EVENT-MAPPINGS
    EventMappings                                                          = 199,
    /// EVENT-MULTICAST-ADDRESSS
    EventMulticastAddresss                                                 = 2836,
    /// EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS
    EventMulticastSubscriptionAddresss                                     = 3862,
    /// EVENT-MULTICAST-UDP-PORT
    EventMulticastUdpPort                                                  = 5504,
    /// EVENT-NEEDS-REF
    EventNeedsRef                                                          = 1631,
    /// EVENT-OBD-READINESS-GROUP
    EventObdReadinessGroup                                                 = 1569,
    /// EVENT-OCCURRENCE-KIND
    EventOccurrenceKind                                                    = 4300,
    /// EVENT-PROPSS
    EventPropss                                                            = 570,
    /// EVENT-QOS-PROPSS
    EventQosPropss                                                         = 3275,
    /// EVENT-READINESS-GROUPS
    EventReadinessGroups                                                   = 3056,
    /// EVENT-REF
    EventRef                                                               = 5064,
    /// EVENT-REFS
    EventRefs                                                              = 3381,
    /// EVENT-SOURCE-REF
    EventSourceRef                                                         = 551,
    /// EVENT-SYMBOL-NAME
    EventSymbolName                                                        = 1811,
    /// EVENT-TOPIC-ACCESS-RULE-REF
    EventTopicAccessRuleRef                                                = 585,
    /// EVENT-WINDOW-TIME
    EventWindowTime                                                        = 3324,
    /// EVENT-WINDOWS
    EventWindows                                                           = 814,
    /// EVENTS
    Events                                                                 = 1211,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 4124,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 3040,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REF
    ExclusiveAreaNestingOrderRef                                           = 5332,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REFS
    ExclusiveAreaNestingOrderRefs                                          = 3182,
    /// EXCLUSIVE-AREA-NESTING-ORDERS
    ExclusiveAreaNestingOrders                                             = 5419,
    /// EXCLUSIVE-AREA-POLICYS
    ExclusiveAreaPolicys                                                   = 3946,
    /// EXCLUSIVE-AREA-REF
    ExclusiveAreaRef                                                       = 1563,
    /// EXCLUSIVE-AREA-REF-CONDITIONAL
    ExclusiveAreaRefConditional                                            = 5344,
    /// EXCLUSIVE-AREA-REFS
    ExclusiveAreaRefs                                                      = 2671,
    /// EXCLUSIVE-AREAS
    ExclusiveAreas                                                         = 4143,
    /// EXEC-IN-USER-MODE
    ExecInUserMode                                                         = 5294,
    /// EXECUTABLE
    Executable                                                             = 539,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 5184,
    /// EXECUTABLE-ENTITY-REF
    ExecutableEntityRef                                                    = 2973,
    /// EXECUTABLE-ENTITY-REFS
    ExecutableEntityRefs                                                   = 4618,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 426,
    /// EXECUTABLE-PROVIDED-PORT-IREF
    ExecutableProvidedPortIref                                             = 2535,
    /// EXECUTABLE-REF
    ExecutableRef                                                          = 2654,
    /// EXECUTABLE-REFS
    ExecutableRefs                                                         = 5325,
    /// EXECUTABLE-REQUIRED-PORT-IREF
    ExecutableRequiredPortIref                                             = 5472,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 5537,
    /// EXECUTE-DESPITE-DATA-UNAVAILABILITY
    ExecuteDespiteDataUnavailability                                       = 2432,
    /// EXECUTION-CONTEXT
    ExecutionContext                                                       = 4242,
    /// EXECUTION-CONTEXT-REF
    ExecutionContextRef                                                    = 254,
    /// EXECUTION-CONTEXT-REFS
    ExecutionContextRefs                                                   = 3762,
    /// EXECUTION-CONTEXTS
    ExecutionContexts                                                      = 3120,
    /// EXECUTION-DEPENDENCY
    ExecutionDependency                                                    = 5375,
    /// EXECUTION-DEPENDENCYS
    ExecutionDependencys                                                   = 5463,
    /// EXECUTION-ERROR
    ExecutionError                                                         = 1101,
    /// EXECUTION-ERROR-REF
    ExecutionErrorRef                                                      = 4244,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 1027,
    /// EXECUTION-ORDER-CONSTRAINT-TYPE
    ExecutionOrderConstraintType                                           = 4947,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 4270,
    /// EXECUTION-TIME-TYPE
    ExecutionTimeType                                                      = 2596,
    /// EXECUTION-TIMES
    ExecutionTimes                                                         = 5485,
    /// EXIT-MODE-DECLARATION-REF
    ExitModeDeclarationRef                                                 = 2629,
    /// EXIT-TIMEOUT-VALUE
    ExitTimeoutValue                                                       = 1331,
    /// EXITED-MODE-REF
    ExitedModeRef                                                          = 2541,
    /// EXPECTED-ALIVE-CYCLE
    ExpectedAliveCycle                                                     = 4588,
    /// EXPECTED-ALIVE-INDICATIONS
    ExpectedAliveIndications                                               = 4582,
    /// EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE
    ExpectedAliveIndicationsInSupervisionCycle                             = 126,
    /// EXPECTED-CALLBACKS
    ExpectedCallbacks                                                      = 5144,
    /// EXPECTED-ENTRYS
    ExpectedEntrys                                                         = 4508,
    /// EXPECTED-TX-TRIGGER
    ExpectedTxTrigger                                                      = 2795,
    /// EXPIRED-SUPERVISION-CYCLE-TOL
    ExpiredSupervisionCycleTol                                             = 4295,
    /// EXPIRED-SUPERVISION-CYCLES-TOLERANCE
    ExpiredSupervisionCyclesTolerance                                      = 927,
    /// EXPIRED-SUPERVISION-TOLERANCE
    ExpiredSupervisionTolerance                                            = 1015,
    /// EXPLICIT-INTER-RUNNABLE-VARIABLES
    ExplicitInterRunnableVariables                                         = 3476,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REF
    ExplicitRptProfileSelectionRef                                         = 399,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REFS
    ExplicitRptProfileSelectionRefs                                        = 521,
    /// EXPORTABILITY
    Exportability                                                          = 3248,
    /// EXPRESSION
    Expression                                                             = 3690,
    /// EXPRESSION-REF
    ExpressionRef                                                          = 1785,
    /// EXT-HEADER-FILTER-LISTS
    ExtHeaderFilterLists                                                   = 2374,
    /// EXTENDED-ADDRESSING
    ExtendedAddressing                                                     = 5236,
    /// EXTENDED-DATA-RECORDS
    ExtendedDataRecords                                                    = 227,
    /// EXTENDS-META-CLASS
    ExtendsMetaClass                                                       = 4784,
    /// EXTERN-OFFSET-CORRECTION
    ExternOffsetCorrection                                                 = 828,
    /// EXTERN-RATE-CORRECTION
    ExternRateCorrection                                                   = 3117,
    /// EXTERNAL-AUTHENTICATIONS
    ExternalAuthentications                                                = 5568,
    /// EXTERNAL-CLOCK-SYNCHRONISATION
    ExternalClockSynchronisation                                           = 1578,
    /// EXTERNAL-REPLACEMENT-REF
    ExternalReplacementRef                                                 = 2831,
    /// EXTERNAL-SYNC
    ExternalSync                                                           = 3845,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 1280,
    /// EXTERNAL-TRIGGERING-POINT
    ExternalTriggeringPoint                                                = 3347,
    /// EXTERNAL-TRIGGERING-POINTS
    ExternalTriggeringPoints                                               = 721,
    /// FACTOR-SI-TO-UNIT
    FactorSiToUnit                                                         = 4402,
    /// FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL
    FailedAliveSupervisionRefCycleTol                                      = 1287,
    /// FAILED-REFERENCE-CYCLES-TOLERANCE
    FailedReferenceCyclesTolerance                                         = 4771,
    /// FAILED-SUPERVISION-CYCLES-TOLERANCE
    FailedSupervisionCyclesTolerance                                       = 4405,
    /// FAILURE-MONITORING-REF
    FailureMonitoringRef                                                   = 2283,
    /// FALL-BACK-INTERNAL
    FallBackInternal                                                       = 4340,
    /// FALSE-ACTION-LIST-REF
    FalseActionListRef                                                     = 3815,
    /// FDC-THRESHOLD-STORAGE-VALUE
    FdcThresholdStorageValue                                               = 2755,
    /// FEATURE-IREF
    FeatureIref                                                            = 744,
    /// FEATURE-MODEL-REF
    FeatureModelRef                                                        = 4662,
    /// FEATURE-MODEL-REFS
    FeatureModelRefs                                                       = 568,
    /// FEATURE-REF
    FeatureRef                                                             = 1495,
    /// FEATURE-REFS
    FeatureRefs                                                            = 2429,
    /// FIBEX-ELEMENT-REF
    FibexElementRef                                                        = 3994,
    /// FIBEX-ELEMENT-REF-CONDITIONAL
    FibexElementRefConditional                                             = 1196,
    /// FIBEX-ELEMENTS
    FibexElements                                                          = 94,
    /// FIELD
    Field                                                                  = 330,
    /// FIELD-DEPLOYMENTS
    FieldDeployments                                                       = 3770,
    /// FIELD-ELEMENTS
    FieldElements                                                          = 4908,
    /// FIELD-GET-SET-QOS-PROPSS
    FieldGetSetQosPropss                                                   = 1864,
    /// FIELD-IREF
    FieldIref                                                              = 1847,
    /// FIELD-MAPPING
    FieldMapping                                                           = 1084,
    /// FIELD-MAPPINGS
    FieldMappings                                                          = 2926,
    /// FIELD-NOTIFIER-QOS-PROPSS
    FieldNotifierQosPropss                                                 = 2267,
    /// FIELD-NOTIFIER-REF
    FieldNotifierRef                                                       = 2041,
    /// FIELD-REF
    FieldRef                                                               = 1246,
    /// FIELD-REFS
    FieldRefs                                                              = 5122,
    /// FIELD-REPLY-TOPIC-NAME
    FieldReplyTopicName                                                    = 4265,
    /// FIELD-REQUEST-TOPIC-NAME
    FieldRequestTopicName                                                  = 2134,
    /// FIELD-SENDER-COM-SPEC
    FieldSenderComSpec                                                     = 2844,
    /// FIELD-TOPICS-ACCESS-RULE-REF
    FieldTopicsAccessRuleRef                                               = 4809,
    /// FIELDS
    Fields                                                                 = 973,
    /// FIFO-DEPTH
    FifoDepth                                                              = 3868,
    /// FIFO-RANGES
    FifoRanges                                                             = 3079,
    /// FIGURE
    Figure                                                                 = 4876,
    /// FIGURE-CAPTION
    FigureCaption                                                          = 4516,
    /// FILE-ELEMENTS
    FileElements                                                           = 4966,
    /// FILE-INFO-COMMENT
    FileInfoComment                                                        = 3234,
    /// FILE-NAME
    FileName                                                               = 4331,
    /// FILE-PROXYS
    FileProxys                                                             = 3286,
    /// FILE-REF
    FileRef                                                                = 3215,
    /// FILE-REFS
    FileRefs                                                               = 2951,
    /// FILE-STORAGE-REF
    FileStorageRef                                                         = 5533,
    /// FILE-TYPE
    FileType                                                               = 3426,
    /// FILE-TYPE-PATTERN
    FileTypePattern                                                        = 2905,
    /// FILES
    Files                                                                  = 605,
    /// FILTER
    Filter                                                                 = 5314,
    /// FILTER-CHAINS
    FilterChains                                                           = 2693,
    /// FILTERING-DEBOUNCING
    FilteringDebouncing                                                    = 5633,
    /// FINAL-CHECKPOINT-REF
    FinalCheckpointRef                                                     = 299,
    /// FINAL-CHECKPOINT-REFS
    FinalCheckpointRefs                                                    = 4945,
    /// FIRE-AND-FORGET
    FireAndForget                                                          = 4812,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 5118,
    /// FIRE-AND-FORGET-MAPPINGS
    FireAndForgetMappings                                                  = 3543,
    /// FIRST-APPLICATION-ERROR-REF
    FirstApplicationErrorRef                                               = 1579,
    /// FIRST-DATA-PROTOTYPE-REF
    FirstDataPrototypeRef                                                  = 4540,
    /// FIRST-ELEMENT-INSTANCE-IREF
    FirstElementInstanceIref                                               = 2808,
    /// FIRST-ELEMENT-INSTANCE-IREFS
    FirstElementInstanceIrefs                                              = 2466,
    /// FIRST-ELEMENT-REF
    FirstElementRef                                                        = 2004,
    /// FIRST-ELEMENT-REFS
    FirstElementRefs                                                       = 2477,
    /// FIRST-ELEMENTS
    FirstElements                                                          = 2984,
    /// FIRST-MODE-GROUP-REF
    FirstModeGroupRef                                                      = 1406,
    /// FIRST-MODE-REF
    FirstModeRef                                                           = 1151,
    /// FIRST-MODE-REFS
    FirstModeRefs                                                          = 3851,
    /// FIRST-OPERATION-REF
    FirstOperationRef                                                      = 742,
    /// FIRST-PHYSICAL-DIMENSION-REF
    FirstPhysicalDimensionRef                                              = 3090,
    /// FIRST-PORT-REF
    FirstPortRef                                                           = 5223,
    /// FIRST-TIMEOUT
    FirstTimeout                                                           = 3201,
    /// FIRST-TO-SECOND-DATA-TRANSFORMATION-REF
    FirstToSecondDataTransformationRef                                     = 1033,
    /// FIRST-TRIGGER-REF
    FirstTriggerRef                                                        = 929,
    /// FIRST-VALUE
    FirstValue                                                             = 457,
    /// FIXED-LENGTH
    FixedLength                                                            = 1604,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 764,
    /// FLAT-INSTANCE-REF
    FlatInstanceRef                                                        = 5374,
    /// FLAT-MAP
    FlatMap                                                                = 1859,
    /// FLAT-MAP-ENTRY-REF
    FlatMapEntryRef                                                        = 321,
    /// FLAT-MAP-ENTRY-REFS
    FlatMapEntryRefs                                                       = 5341,
    /// FLAT-MAP-REF
    FlatMapRef                                                             = 2696,
    /// FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING
    FlexrayAbsolutelyScheduledTiming                                       = 1596,
    /// FLEXRAY-AR-TP-CHANNEL
    FlexrayArTpChannel                                                     = 4,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 972,
    /// FLEXRAY-AR-TP-CONNECTION
    FlexrayArTpConnection                                                  = 4476,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 4417,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 3624,
    /// FLEXRAY-CLUSTER-CONDITIONAL
    FlexrayClusterConditional                                              = 4172,
    /// FLEXRAY-CLUSTER-VARIANTS
    FlexrayClusterVariants                                                 = 3591,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 1122,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 2562,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL
    FlexrayCommunicationControllerConditional                              = 4991,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS
    FlexrayCommunicationControllerVariants                                 = 1090,
    /// FLEXRAY-FIFO-CONFIGURATION
    FlexrayFifoConfiguration                                               = 1754,
    /// FLEXRAY-FIFO-RANGE
    FlexrayFifoRange                                                       = 1573,
    /// FLEXRAY-FIFOS
    FlexrayFifos                                                           = 49,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 2786,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 5148,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 4098,
    /// FLEXRAY-NM-CLUSTER-COUPLING
    FlexrayNmClusterCoupling                                               = 1998,
    /// FLEXRAY-NM-ECU
    FlexrayNmEcu                                                           = 3135,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 402,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 3537,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 1007,
    /// FLEXRAY-TP-CONNECTION
    FlexrayTpConnection                                                    = 1254,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 505,
    /// FLEXRAY-TP-ECU
    FlexrayTpEcu                                                           = 4335,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 4929,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 4387,
    /// FLOAT
    Float                                                                  = 5339,
    /// FLOAT-VALUE-VARIATION-POINT
    FloatValueVariationPoint                                               = 5518,
    /// FLOW-CONTROL-PDU-REF
    FlowControlPduRef                                                      = 5594,
    /// FLOW-CONTROL-PDU-REFS
    FlowControlPduRefs                                                     = 2195,
    /// FLOW-CONTROL-REF
    FlowControlRef                                                         = 3520,
    /// FLOW-LABEL
    FlowLabel                                                              = 661,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 1677,
    /// FM-ATTRIBUTE-VALUE
    FmAttributeValue                                                       = 1626,
    /// FM-COND
    FmCond                                                                 = 4471,
    /// FM-FEATURE
    FmFeature                                                              = 3772,
    /// FM-FEATURE-DECOMPOSITION
    FmFeatureDecomposition                                                 = 4395,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 4401,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 1165,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 2200,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 3832,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 825,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 1182,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 4442,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 3692,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 2494,
    /// FM-SYSCOND
    FmSyscond                                                              = 1886,
    /// FMI
    Fmi                                                                    = 1816,
    /// FOLLOW-UP-ACTION-REF
    FollowUpActionRef                                                      = 4711,
    /// FOLLOW-UP-ACTION-REFS
    FollowUpActionRefs                                                     = 1589,
    /// FOLLOW-UP-OFFSET
    FollowUpOffset                                                         = 2047,
    /// FOLLOW-UP-TIMEOUT-VALUE
    FollowUpTimeoutValue                                                   = 5043,
    /// FORBIDDEN-SIGNAL-PATH
    ForbiddenSignalPath                                                    = 3384,
    /// FOREIGN-MODEL-REFERENCE
    ForeignModelReference                                                  = 1328,
    /// FORMAL-BLUEPRINT-CONDITION
    FormalBlueprintCondition                                               = 2170,
    /// FORMAL-BLUEPRINT-GENERATOR
    FormalBlueprintGenerator                                               = 2060,
    /// FORMAT
    Format                                                                 = 749,
    /// FORMULA
    Formula                                                                = 1038,
    /// FORMULA-CAPTION
    FormulaCaption                                                         = 3891,
    /// FR-CLUSTER-REF
    FrClusterRef                                                           = 3761,
    /// FR-GLOBAL-TIME-DOMAIN-PROPS
    FrGlobalTimeDomainProps                                                = 1469,
    /// FRAGMENT
    Fragment                                                               = 144,
    /// FRAGMENTATION-PROPS
    FragmentationProps                                                     = 5376,
    /// FRAME-LENGTH
    FrameLength                                                            = 950,
    /// FRAME-MAPPING
    FrameMapping                                                           = 111,
    /// FRAME-MAPPINGS
    FrameMappings                                                          = 409,
    /// FRAME-PID
    FramePid                                                               = 811,
    /// FRAME-PIDS
    FramePids                                                              = 3318,
    /// FRAME-PORT
    FramePort                                                              = 2232,
    /// FRAME-PORT-REF
    FramePortRef                                                           = 777,
    /// FRAME-PORT-REFS
    FramePortRefs                                                          = 3889,
    /// FRAME-REF
    FrameRef                                                               = 2723,
    /// FRAME-TRIGGERING-REF
    FrameTriggeringRef                                                     = 1310,
    /// FRAME-TRIGGERINGS
    FrameTriggerings                                                       = 3299,
    /// FREE-FORMAT
    FreeFormat                                                             = 4463,
    /// FREEZE-CURRENT-STATE
    FreezeCurrentState                                                     = 5557,
    /// FREEZE-CURRENT-STATE-SUPPORTED
    FreezeCurrentStateSupported                                            = 917,
    /// FREEZE-FRAME-CONTENT-REF
    FreezeFrameContentRef                                                  = 4707,
    /// FREEZE-FRAME-CONTENT-WWH-OBD-REF
    FreezeFrameContentWwhObdRef                                            = 4044,
    /// FREEZE-FRAME-REF
    FreezeFrameRef                                                         = 2220,
    /// FREEZE-FRAMES
    FreezeFrames                                                           = 4624,
    /// FRESHNESS-COUNTER-SYNC-ATTEMPTS
    FreshnessCounterSyncAttempts                                           = 4719,
    /// FRESHNESS-PROPS-REF
    FreshnessPropsRef                                                      = 477,
    /// FRESHNESS-PROPSS
    FreshnessPropss                                                        = 2205,
    /// FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR
    FreshnessTimestampTimePeriodFactor                                     = 4202,
    /// FRESHNESS-VALUE-ID
    FreshnessValueId                                                       = 3243,
    /// FRESHNESS-VALUE-LENGTH
    FreshnessValueLength                                                   = 2059,
    /// FRESHNESS-VALUE-TX-LENGTH
    FreshnessValueTxLength                                                 = 690,
    /// FROM-REF
    FromRef                                                                = 168,
    /// FT
    Ft                                                                     = 219,
    /// FULL-DUPLEX-ENABLED
    FullDuplexEnabled                                                      = 708,
    /// FULLY-QUALIFIED-DOMAIN-NAME
    FullyQualifiedDomainName                                               = 1765,
    /// FUNCTION
    Function                                                               = 3790,
    /// FUNCTION-CLUSTER-AFFILIATION
    FunctionClusterAffiliation                                             = 2225,
    /// FUNCTION-GROUP-IREF
    FunctionGroupIref                                                      = 5050,
    /// FUNCTION-GROUP-IREFS
    FunctionGroupIrefs                                                     = 3155,
    /// FUNCTION-GROUP-MODE-IREF
    FunctionGroupModeIref                                                  = 1939,
    /// FUNCTION-GROUP-MODE-IREFS
    FunctionGroupModeIrefs                                                 = 1996,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 366,
    /// FUNCTION-GROUP-PHM-STATE-REFERENCE
    FunctionGroupPhmStateReference                                         = 4989,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 1772,
    /// FUNCTION-GROUP-STATE-IREF
    FunctionGroupStateIref                                                 = 4703,
    /// FUNCTION-GROUP-STATE-IREFS
    FunctionGroupStateIrefs                                                = 363,
    /// FUNCTION-GROUPS
    FunctionGroups                                                         = 3257,
    /// FUNCTION-ID
    FunctionId                                                             = 3130,
    /// FUNCTION-IDENTIFIER-REF
    FunctionIdentifierRef                                                  = 1617,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 4882,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 3123,
    /// FUNCTION-INSTANCE
    FunctionInstance                                                       = 408,
    /// FUNCTION-POINTER-SIGNATURE-REF
    FunctionPointerSignatureRef                                            = 5173,
    /// FUNCTION-PROTOTYPE-EMITTER
    FunctionPrototypeEmitter                                               = 428,
    /// FUNCTIONAL-REQUEST-REF
    FunctionalRequestRef                                                   = 2286,
    /// FUNCTIONAL-REQUEST-REFS
    FunctionalRequestRefs                                                  = 2606,
    /// FUNCTIONAL-UNIT
    FunctionalUnit                                                         = 1540,
    /// FUP-DATA-ID-LIST
    FupDataIdList                                                          = 3092,
    /// FUP-DATA-ID-LISTS
    FupDataIdLists                                                         = 2213,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 238,
    /// GATEWAY
    Gateway                                                                = 5644,
    /// GATEWAYS
    Gateways                                                               = 2967,
    /// GENERAL-INACTIVITY-TIME
    GeneralInactivityTime                                                  = 2390,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 4398,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 1344,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 3003,
    /// GENERATED-ARTIFACTS
    GeneratedArtifacts                                                     = 3282,
    /// GENERATOR
    Generator                                                              = 485,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 2366,
    /// GENERIC-MATH
    GenericMath                                                            = 1107,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 1077,
    /// GENERIC-TP
    GenericTp                                                              = 3386,
    /// GET
    Get                                                                    = 4081,
    /// GETTER-CALL-REF
    GetterCallRef                                                          = 1100,
    /// GETTER-CALL-SIGNAL-REF
    GetterCallSignalRef                                                    = 3756,
    /// GETTER-OPERATION-REF
    GetterOperationRef                                                     = 4634,
    /// GETTER-REF
    GetterRef                                                              = 5110,
    /// GETTER-RETURN-REF
    GetterReturnRef                                                        = 4564,
    /// GETTER-RETURN-SIGNAL-REF
    GetterReturnSignalRef                                                  = 1850,
    /// GID
    Gid                                                                    = 504,
    /// GID-INVALIDITY-PATTERN
    GidInvalidityPattern                                                   = 226,
    /// GLOBAL-ELEMENT
    GlobalElement                                                          = 3939,
    /// GLOBAL-ELEMENTS
    GlobalElements                                                         = 3082,
    /// GLOBAL-IN-PACKAGE-REF
    GlobalInPackageRef                                                     = 586,
    /// GLOBAL-IN-PACKAGE-REFS
    GlobalInPackageRefs                                                    = 4441,
    /// GLOBAL-RESOURCE-ID
    GlobalResourceId                                                       = 4610,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 2569,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 2226,
    /// GLOBAL-SUPERVISION-ENTITYS
    GlobalSupervisionEntitys                                               = 3794,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 5421,
    /// GLOBAL-SUPERVISIONS
    GlobalSupervisions                                                     = 2354,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 1270,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 4576,
    /// GLOBAL-TIME-CORRECTION-PROPS
    GlobalTimeCorrectionProps                                              = 1819,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 2360,
    /// GLOBAL-TIME-DOMAIN-PROPERTYS
    GlobalTimeDomainPropertys                                              = 5159,
    /// GLOBAL-TIME-DOMAIN-PROPS
    GlobalTimeDomainProps                                                  = 44,
    /// GLOBAL-TIME-DOMAIN-REF
    GlobalTimeDomainRef                                                    = 2139,
    /// GLOBAL-TIME-DOMAIN-REF-CONDITIONAL
    GlobalTimeDomainRefConditional                                         = 3441,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 1462,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 3609,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 4381,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 5038,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 2508,
    /// GLOBAL-TIME-MASTERS
    GlobalTimeMasters                                                      = 4607,
    /// GLOBAL-TIME-PDU-REF
    GlobalTimePduRef                                                       = 4686,
    /// GLOBAL-TIME-PDU-TRIGGERING-REF
    GlobalTimePduTriggeringRef                                             = 4107,
    /// GLOBAL-TIME-PROPS
    GlobalTimeProps                                                        = 638,
    /// GLOBAL-TIME-SUB-DOMAINS
    GlobalTimeSubDomains                                                   = 4489,
    /// GOVERNANCE-REF
    GovernanceRef                                                          = 3694,
    /// GRANT-REF
    GrantRef                                                               = 4195,
    /// GRANT-REFS
    GrantRefs                                                              = 4585,
    /// GRAPHIC
    Graphic                                                                = 5596,
    /// GROUP-ID
    GroupId                                                                = 354,
    /// GROUP-IDENTIFIER
    GroupIdentifier                                                        = 4612,
    /// GROUP-NUMBER
    GroupNumber                                                            = 2114,
    /// GROUPED-ALIAS-EVENT-REF
    GroupedAliasEventRef                                                   = 392,
    /// GROUPED-ALIAS-EVENT-REFS
    GroupedAliasEventRefs                                                  = 2495,
    /// HANDLE-DATA-STATUS
    HandleDataStatus                                                       = 1430,
    /// HANDLE-INVALID
    HandleInvalid                                                          = 2305,
    /// HANDLE-NEVER-RECEIVED
    HandleNeverReceived                                                    = 162,
    /// HANDLE-OUT-OF-RANGE
    HandleOutOfRange                                                       = 724,
    /// HANDLE-OUT-OF-RANGE-STATUS
    HandleOutOfRangeStatus                                                 = 2513,
    /// HANDLE-TERMINATION-AND-RESTART
    HandleTerminationAndRestart                                            = 2013,
    /// HANDLE-TIMEOUT-TYPE
    HandleTimeoutType                                                      = 2796,
    /// HARDWARE-CONFIGURATION
    HardwareConfiguration                                                  = 5523,
    /// HARDWARE-ELEMENT-REF
    HardwareElementRef                                                     = 4642,
    /// HARDWARE-ELEMENT-REFS
    HardwareElementRefs                                                    = 5502,
    /// HARDWARE-PLATFORM
    HardwarePlatform                                                       = 4418,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 1783,
    /// HAS-DYNAMIC-LENGTH
    HasDynamicLength                                                       = 864,
    /// HAS-GETTER
    HasGetter                                                              = 2302,
    /// HAS-INTERNAL-STATE
    HasInternalState                                                       = 3581,
    /// HAS-NOTIFIER
    HasNotifier                                                            = 688,
    /// HAS-SETTER
    HasSetter                                                              = 496,
    /// HEADER-FILE
    HeaderFile                                                             = 2543,
    /// HEADER-ID
    HeaderId                                                               = 2375,
    /// HEADER-ID-LONG-HEADER
    HeaderIdLongHeader                                                     = 677,
    /// HEADER-ID-SHORT-HEADER
    HeaderIdShortHeader                                                    = 1493,
    /// HEADER-LENGTH
    HeaderLength                                                           = 545,
    /// HEADER-TYPE
    HeaderType                                                             = 915,
    /// HEALING-CYCLE-COUNTER-THRESHOLD
    HealingCycleCounterThreshold                                           = 5409,
    /// HEALING-CYCLE-REF
    HealingCycleRef                                                        = 2300,
    /// HEALTH-CHANNEL-ARGUMENT-REF
    HealthChannelArgumentRef                                               = 4235,
    /// HEALTH-CHANNEL-ARGUMENT-REFS
    HealthChannelArgumentRefs                                              = 4772,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 3742,
    /// HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS
    HealthChannelExternalReportedStatus                                    = 3803,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 1942,
    /// HEALTH-CHANNEL-ID
    HealthChannelId                                                        = 1890,
    /// HEALTH-CHANNEL-IREF
    HealthChannelIref                                                      = 4579,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 5643,
    /// HEALTH-CHANNELS
    HealthChannels                                                         = 5330,
    /// HEAP-USAGES
    HeapUsages                                                             = 4238,
    /// HOP-COUNT
    HopCount                                                               = 2781,
    /// HOST-REF
    HostRef                                                                = 891,
    /// HOST-SYSTEM-REF
    HostSystemRef                                                          = 1525,
    /// HTTP-ACCEPT-ENCODING
    HttpAcceptEncoding                                                     = 679,
    /// HTTP-TP
    HttpTp                                                                 = 4421,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 543,
    /// HW-ATTRIBUTE-DEF-REF
    HwAttributeDefRef                                                      = 2729,
    /// HW-ATTRIBUTE-DEFS
    HwAttributeDefs                                                        = 4163,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 465,
    /// HW-ATTRIBUTE-LITERALS
    HwAttributeLiterals                                                    = 2168,
    /// HW-ATTRIBUTE-VALUE
    HwAttributeValue                                                       = 1457,
    /// HW-ATTRIBUTE-VALUES
    HwAttributeValues                                                      = 3096,
    /// HW-CATEGORY
    HwCategory                                                             = 2522,
    /// HW-CATEGORY-REF
    HwCategoryRef                                                          = 3831,
    /// HW-CATEGORY-REFS
    HwCategoryRefs                                                         = 925,
    /// HW-COMMUNICATION-CONTROLLER-REF
    HwCommunicationControllerRef                                           = 3153,
    /// HW-COMMUNICATION-PORT-REF
    HwCommunicationPortRef                                                 = 4451,
    /// HW-ELEMENT
    HwElement                                                              = 4029,
    /// HW-ELEMENT-CONNECTIONS
    HwElementConnections                                                   = 962,
    /// HW-ELEMENT-CONNECTOR
    HwElementConnector                                                     = 1459,
    /// HW-ELEMENT-REF
    HwElementRef                                                           = 4590,
    /// HW-ELEMENT-REF-CONDITIONAL
    HwElementRefConditional                                                = 1974,
    /// HW-ELEMENT-REFS
    HwElementRefs                                                          = 1013,
    /// HW-PIN
    HwPin                                                                  = 3740,
    /// HW-PIN-CONNECTIONS
    HwPinConnections                                                       = 807,
    /// HW-PIN-CONNECTOR
    HwPinConnector                                                         = 1490,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 5114,
    /// HW-PIN-GROUP-CONNECTIONS
    HwPinGroupConnections                                                  = 2677,
    /// HW-PIN-GROUP-CONNECTOR
    HwPinGroupConnector                                                    = 831,
    /// HW-PIN-GROUP-CONTENT
    HwPinGroupContent                                                      = 4804,
    /// HW-PIN-GROUP-REF
    HwPinGroupRef                                                          = 4328,
    /// HW-PIN-GROUP-REFS
    HwPinGroupRefs                                                         = 1202,
    /// HW-PIN-GROUPS
    HwPinGroups                                                            = 278,
    /// HW-PIN-REF
    HwPinRef                                                               = 3481,
    /// HW-PIN-REFS
    HwPinRefs                                                              = 2352,
    /// HW-PORT-MAPPING
    HwPortMapping                                                          = 5222,
    /// HW-PORT-MAPPINGS
    HwPortMappings                                                         = 2714,
    /// HW-TYPE
    HwType                                                                 = 805,
    /// HW-TYPE-REF
    HwTypeRef                                                              = 2688,
    /// I-PDU-IDENTIFIER-TCP-REF
    IPduIdentifierTcpRef                                                   = 23,
    /// I-PDU-IDENTIFIER-TCP-REFS
    IPduIdentifierTcpRefs                                                  = 2363,
    /// I-PDU-IDENTIFIER-UDP-REF
    IPduIdentifierUdpRef                                                   = 3298,
    /// I-PDU-IDENTIFIER-UDP-REFS
    IPduIdentifierUdpRefs                                                  = 4995,
    /// I-PDU-IDENTIFIERS
    IPduIdentifiers                                                        = 2803,
    /// I-PDU-MAPPING
    IPduMapping                                                            = 3247,
    /// I-PDU-MAPPINGS
    IPduMappings                                                           = 1749,
    /// I-PDU-PORT
    IPduPort                                                               = 3344,
    /// I-PDU-PORT-REF
    IPduPortRef                                                            = 2209,
    /// I-PDU-PORT-REFS
    IPduPortRefs                                                           = 3030,
    /// I-PDU-REF
    IPduRef                                                                = 122,
    /// I-PDU-SIGNAL-PROCESSING
    IPduSignalProcessing                                                   = 2001,
    /// I-PDU-TIMING
    IPduTiming                                                             = 2000,
    /// I-PDU-TIMING-SPECIFICATIONS
    IPduTimingSpecifications                                               = 5508,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 451,
    /// I-PDU-TRIGGERING-REF
    IPduTriggeringRef                                                      = 3752,
    /// I-PDU-TRIGGERING-REF-CONDITIONAL
    IPduTriggeringRefConditional                                           = 4090,
    /// I-PDU-TRIGGERINGS
    IPduTriggerings                                                        = 4725,
    /// I-PDUS
    IPdus                                                                  = 3724,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 313,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 4517,
    /// I-SIGNAL
    ISignal                                                                = 1318,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 4282,
    /// I-SIGNAL-GROUP-REF
    ISignalGroupRef                                                        = 72,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 4296,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 2756,
    /// I-SIGNAL-I-PDU-REF
    ISignalIPduRef                                                         = 5163,
    /// I-SIGNAL-I-PDU-REF-CONDITIONAL
    ISignalIPduRefConditional                                              = 1228,
    /// I-SIGNAL-I-PDUS
    ISignalIPdus                                                           = 328,
    /// I-SIGNAL-IN-I-PDU-REF
    ISignalInIPduRef                                                       = 3216,
    /// I-SIGNAL-MAPPING
    ISignalMapping                                                         = 2938,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 4317,
    /// I-SIGNAL-PORT-REF
    ISignalPortRef                                                         = 134,
    /// I-SIGNAL-PORT-REFS
    ISignalPortRefs                                                        = 5488,
    /// I-SIGNAL-PROPS
    ISignalProps                                                           = 3937,
    /// I-SIGNAL-REF
    ISignalRef                                                             = 3191,
    /// I-SIGNAL-REFS
    ISignalRefs                                                            = 4477,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 4507,
    /// I-SIGNAL-TO-I-PDU-MAPPINGS
    ISignalToIPduMappings                                                  = 2943,
    /// I-SIGNAL-TO-PDU-MAPPINGS
    ISignalToPduMappings                                                   = 2161,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 2774,
    /// I-SIGNAL-TRIGGERING-REF
    ISignalTriggeringRef                                                   = 4946,
    /// I-SIGNAL-TRIGGERING-REF-CONDITIONAL
    ISignalTriggeringRefConditional                                        = 698,
    /// I-SIGNAL-TRIGGERINGS
    ISignalTriggerings                                                     = 4056,
    /// I-SIGNAL-TYPE
    ISignalType                                                            = 3140,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 4761,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF
    IamRelevantTlsSecureComPropsRef                                        = 1894,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS
    IamRelevantTlsSecureComPropsRefs                                       = 4016,
    /// ICMP-V-4-PROPS
    IcmpV4Props                                                            = 4166,
    /// ICMP-V-6-PROPS
    IcmpV6Props                                                            = 3664,
    /// ID
    Id                                                                     = 4305,
    /// IDENT
    Ident                                                                  = 1327,
    /// IDENTICAL-MAPPING
    IdenticalMapping                                                       = 345,
    /// IDENTIFIABLE-REF
    IdentifiableRef                                                        = 3435,
    /// IDENTIFIER
    Identifier                                                             = 558,
    /// IDENTITIY-NUMBER
    IdentitiyNumber                                                        = 5161,
    /// IDENTITY
    Identity                                                               = 410,
    /// IDENTITY-CERTIFICATE-AUTHORITY-REF
    IdentityCertificateAuthorityRef                                        = 3480,
    /// IDENTITY-REF
    IdentityRef                                                            = 5652,
    /// IDLE-SLOPE
    IdleSlope                                                              = 5637,
    /// IDS-COMMON-ELEMENT-REF
    IdsCommonElementRef                                                    = 2554,
    /// IDS-COMMON-ELEMENT-REF-CONDITIONAL
    IdsCommonElementRefConditional                                         = 1198,
    /// IDS-DESIGN
    IdsDesign                                                              = 5307,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 4085,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 5188,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 5290,
    /// IDSM-INSTANCE-ID
    IdsmInstanceId                                                         = 897,
    /// IDSM-INSTANCE-REF
    IdsmInstanceRef                                                        = 2210,
    /// IDSM-INSTANCE-REF-CONDITIONAL
    IdsmInstanceRefConditional                                             = 1915,
    /// IDSM-INSTANCES
    IdsmInstances                                                          = 312,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 3704,
    /// IDSM-MODULE-INSTANTIATION-REF
    IdsmModuleInstantiationRef                                             = 165,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 693,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 2509,
    /// IDSM-RATE-LIMITATION-REF
    IdsmRateLimitationRef                                                  = 2043,
    /// IDSM-RATE-LIMITATION-REF-CONDITIONAL
    IdsmRateLimitationRefConditional                                       = 2317,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 502,
    /// IDSM-TRAFFIC-LIMITATION-REF
    IdsmTrafficLimitationRef                                               = 3701,
    /// IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL
    IdsmTrafficLimitationRefConditional                                    = 75,
    /// IE
    Ie                                                                     = 444,
    /// IEEE-1722-TP
    Ieee1722Tp                                                             = 3210,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 5205,
    /// IGNORE-AFTER-TX
    IgnoreAfterTx                                                          = 4706,
    /// IGNORE-ORDER-ALLOWED
    IgnoreOrderAllowed                                                     = 4258,
    /// IKE-AUTHENTICATION-METHOD
    IkeAuthenticationMethod                                                = 4228,
    /// IKE-CIPHER-SUITE-NAME
    IkeCipherSuiteName                                                     = 3749,
    /// IKE-OVER-TIME
    IkeOverTime                                                            = 2959,
    /// IKE-RAND-TIME
    IkeRandTime                                                            = 460,
    /// IKE-REAUTH-TIME
    IkeReauthTime                                                          = 14,
    /// IKE-REKEY-TIME
    IkeRekeyTime                                                           = 3363,
    /// IMMEDIATE-NV-DATA-STORAGE
    ImmediateNvDataStorage                                                 = 3178,
    /// IMMEDIATE-RESUME-TIME
    ImmediateResumeTime                                                    = 562,
    /// IMPL-CONSTANT-REF
    ImplConstantRef                                                        = 5413,
    /// IMPL-INIT-VALUE
    ImplInitValue                                                          = 3421,
    /// IMPLEMENTATION-ARRAY-ELEMENT-REF
    ImplementationArrayElementRef                                          = 4962,
    /// IMPLEMENTATION-CONFIG-CLASSES
    ImplementationConfigClasses                                            = 2838,
    /// IMPLEMENTATION-CONFIG-VARIANT
    ImplementationConfigVariant                                            = 342,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 3499,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 4958,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 5230,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF
    ImplementationDataTypeElementInAutosarDataPrototypeRef                 = 818,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF
    ImplementationDataTypeElementInPortInterfaceRef                        = 45,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF
    ImplementationDataTypeElementInSystemRef                               = 5322,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    ImplementationDataTypeElementRef                                       = 5262,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 1882,
    /// IMPLEMENTATION-DATA-TYPE-REF
    ImplementationDataTypeRef                                              = 425,
    /// IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF
    ImplementationDataTypeSubElementRef                                    = 1213,
    /// IMPLEMENTATION-RECORD-ELEMENT-REF
    ImplementationRecordElementRef                                         = 812,
    /// IMPLEMENTATION-REF
    ImplementationRef                                                      = 2866,
    /// IMPLEMENTATION-REFS
    ImplementationRefs                                                     = 5007,
    /// IMPLEMENTED-ENTRY-REF
    ImplementedEntryRef                                                    = 4461,
    /// IMPLEMENTED-ENTRYS
    ImplementedEntrys                                                      = 798,
    /// IMPLEMENTED-IN-REF
    ImplementedInRef                                                       = 4117,
    /// IMPLEMENTS-LEGACY-STRING-SERIALIZATION
    ImplementsLegacyStringSerialization                                    = 2111,
    /// IMPLEMENTS-SOMEIP-STRING-HANDLING
    ImplementsSomeipStringHandling                                         = 3100,
    /// IMPLICIT-DATA-ACCESS-IREF
    ImplicitDataAccessIref                                                 = 5507,
    /// IMPLICIT-DATA-ACCESS-IREFS
    ImplicitDataAccessIrefs                                                = 2754,
    /// IMPLICIT-INTER-RUNNABLE-VARIABLES
    ImplicitInterRunnableVariables                                         = 3170,
    /// IMPLIED-OPERATION-REF
    ImpliedOperationRef                                                    = 2790,
    /// IMPLIED-OPERATION-REFS
    ImpliedOperationRefs                                                   = 4064,
    /// IMPORTANCE
    Importance                                                             = 2815,
    /// IN-MEASUREMENT-SET
    InMeasurementSet                                                       = 1944,
    /// IN-PLACE
    InPlace                                                                = 824,
    /// IN-SCOPE
    InScope                                                                = 2472,
    /// INCLUDE-REF
    IncludeRef                                                             = 2214,
    /// INCLUDE-REFS
    IncludeRefs                                                            = 3164,
    /// INCLUDED-DATA-TYPE-SET
    IncludedDataTypeSet                                                    = 2717,
    /// INCLUDED-DATA-TYPE-SETS
    IncludedDataTypeSets                                                   = 3213,
    /// INCLUDED-LIBRARY-REF
    IncludedLibraryRef                                                     = 1441,
    /// INCLUDED-LIBRARY-REFS
    IncludedLibraryRefs                                                    = 1018,
    /// INCLUDED-MODE-DECLARATION-GROUP-SET
    IncludedModeDeclarationGroupSet                                        = 449,
    /// INCLUDED-MODE-DECLARATION-GROUP-SETS
    IncludedModeDeclarationGroupSets                                       = 5595,
    /// INCLUDED-VARIANT-REF
    IncludedVariantRef                                                     = 2248,
    /// INCLUDED-VARIANT-REFS
    IncludedVariantRefs                                                    = 3640,
    /// INDENT-SAMPLE
    IndentSample                                                           = 5005,
    /// INDEX
    Index                                                                  = 1476,
    /// INDEX-DATA-TYPE-REF
    IndexDataTypeRef                                                       = 3560,
    /// INDEXED-ARRAY-ELEMENT
    IndexedArrayElement                                                    = 1605,
    /// INDICATIONS
    Indications                                                            = 5555,
    /// INDICATOR-REF
    IndicatorRef                                                           = 3852,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 3570,
    /// INDIRECT-API
    IndirectApi                                                            = 4407,
    /// INDUSTRY-GROUP
    IndustryGroup                                                          = 4096,
    /// INFO-TYPE
    InfoType                                                               = 3668,
    /// INFO-TYPE-REF
    InfoTypeRef                                                            = 5199,
    /// INFORMAL-FORMULA
    InformalFormula                                                        = 1627,
    /// INFRASTRUCTURE-SERVICES
    InfrastructureServices                                                 = 995,
    /// INGRESS-PRIORITY
    IngressPriority                                                        = 5550,
    /// INHIBIT-SOURCES
    InhibitSources                                                         = 3595,
    /// INHIBITING-FID-REF
    InhibitingFidRef                                                       = 2507,
    /// INHIBITING-SECONDARY-FID-REF
    InhibitingSecondaryFidRef                                              = 1362,
    /// INHIBITING-SECONDARY-FID-REFS
    InhibitingSecondaryFidRefs                                             = 13,
    /// INHIBITION-MASK
    InhibitionMask                                                         = 5010,
    /// INHIBITION-SOURCE-REF
    InhibitionSourceRef                                                    = 2367,
    /// INIT-EVENT
    InitEvent                                                              = 596,
    /// INIT-RESOURCE
    InitResource                                                           = 5140,
    /// INIT-VALUE
    InitValue                                                              = 2668,
    /// INIT-VALUES
    InitValues                                                             = 3488,
    /// INITIAL-CHECKPOINT-REF
    InitialCheckpointRef                                                   = 748,
    /// INITIAL-CHECKPOINT-REFS
    InitialCheckpointRefs                                                  = 495,
    /// INITIAL-DELAY-MAX-VALUE
    InitialDelayMaxValue                                                   = 636,
    /// INITIAL-DELAY-MIN-VALUE
    InitialDelayMinValue                                                   = 325,
    /// INITIAL-DYNAMIC-PART
    InitialDynamicPart                                                     = 3125,
    /// INITIAL-EVENT-STATUS
    InitialEventStatus                                                     = 3088,
    /// INITIAL-FIND-BEHAVIOR
    InitialFindBehavior                                                    = 2269,
    /// INITIAL-INACTIVITY-TIME
    InitialInactivityTime                                                  = 967,
    /// INITIAL-MODE-REF
    InitialModeRef                                                         = 1411,
    /// INITIAL-NAD
    InitialNad                                                             = 2968,
    /// INITIAL-OFFER-BEHAVIOR
    InitialOfferBehavior                                                   = 3872,
    /// INITIAL-REF-OFFSET
    InitialRefOffset                                                       = 1222,
    /// INITIAL-REPETITIONS-BASE-DELAY
    InitialRepetitionsBaseDelay                                            = 3288,
    /// INITIAL-REPETITIONS-MAX
    InitialRepetitionsMax                                                  = 621,
    /// INITIAL-STATUS
    InitialStatus                                                          = 2378,
    /// INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    InitialVehicleAnnouncementTime                                         = 4893,
    /// INITIALIZATION-VECTOR-LENGTH
    InitializationVectorLength                                             = 4801,
    /// INITIALIZED-PARAMETER-REF
    InitializedParameterRef                                                = 3017,
    /// INNER-GROUP-IREF
    InnerGroupIref                                                         = 2890,
    /// INNER-GROUP-IREFS
    InnerGroupIrefs                                                        = 4271,
    /// INNER-PORT-IREF
    InnerPortIref                                                          = 5057,
    /// INPLACE
    Inplace                                                                = 717,
    /// INPUT
    Input                                                                  = 875,
    /// INPUT-DATAS
    InputDatas                                                             = 2778,
    /// INPUT-VARIABLE-TYPE-REF
    InputVariableTypeRef                                                   = 852,
    /// INSTANCE-ID
    InstanceId                                                             = 2828,
    /// INSTANCE-ID-PROPS
    InstanceIdProps                                                        = 2957,
    /// INSTANCE-IDENTIFIER
    InstanceIdentifier                                                     = 3437,
    /// INSTANCE-IDS
    InstanceIds                                                            = 2027,
    /// INSTANCE-IN-MEMORY
    InstanceInMemory                                                       = 4886,
    /// INSTANCES
    Instances                                                              = 4938,
    /// INSTANTIATION-DATA-DEF-PROPS
    InstantiationDataDefProps                                              = 682,
    /// INSTANTIATION-DATA-DEF-PROPSS
    InstantiationDataDefPropss                                             = 4857,
    /// INSTANTIATION-RTE-EVENT-PROPSS
    InstantiationRteEventPropss                                            = 4910,
    /// INSTANTIATION-TIMING-EVENT-PROPS
    InstantiationTimingEventProps                                          = 5448,
    /// INTEGER-VALUE-VARIATION-POINT
    IntegerValueVariationPoint                                             = 1124,
    /// INTENDED-FILENAME
    IntendedFilename                                                       = 4013,
    /// INTENDED-PARTIAL-INITIALIZATION-COUNT
    IntendedPartialInitializationCount                                     = 4167,
    /// INTENDED-TARGET-MACHINE-REF
    IntendedTargetMachineRef                                               = 2992,
    /// INTER-MESSAGE-TIME
    InterMessageTime                                                       = 726,
    /// INTERFACE-BLUEPRINT-MAPPING
    InterfaceBlueprintMapping                                              = 1906,
    /// INTERFACE-BLUEPRINT-MAPPINGS
    InterfaceBlueprintMappings                                             = 984,
    /// INTERFACE-BLUEPRINT-REF
    InterfaceBlueprintRef                                                  = 2892,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 904,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 4778,
    /// INTERFACE-MAPPINGS
    InterfaceMappings                                                      = 3068,
    /// INTERFACE-REF
    InterfaceRef                                                           = 3940,
    /// INTERFACE-VERSION
    InterfaceVersion                                                       = 2955,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 5027,
    /// INTERNAL-BEHAVIORS
    InternalBehaviors                                                      = 1171,
    /// INTERNAL-CONSTRS
    InternalConstrs                                                        = 3892,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 593,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 5285,
    /// INTERNAL-TRIGGERING-POINT-POLICYS
    InternalTriggeringPointPolicys                                         = 419,
    /// INTERNAL-TRIGGERING-POINTS
    InternalTriggeringPoints                                               = 4137,
    /// INTERPOLATION-ROUTINE
    InterpolationRoutine                                                   = 4312,
    /// INTERPOLATION-ROUTINE-MAPPING
    InterpolationRoutineMapping                                            = 2006,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 3963,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REF
    InterpolationRoutineMappingSetRef                                      = 1121,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REFS
    InterpolationRoutineMappingSetRefs                                     = 3241,
    /// INTERPOLATION-ROUTINE-MAPPINGS
    InterpolationRoutineMappings                                           = 2389,
    /// INTERPOLATION-ROUTINE-REF
    InterpolationRoutineRef                                                = 2506,
    /// INTERPOLATION-ROUTINES
    InterpolationRoutines                                                  = 2078,
    /// INTERRUPT-CATEGORY
    InterruptCategory                                                      = 239,
    /// INTERRUPT-SOURCE
    InterruptSource                                                        = 3141,
    /// INTERVAL-LENGTH
    IntervalLength                                                         = 851,
    /// INTRODUCTION
    Introduction                                                           = 2842,
    /// INVALID-VALUE
    InvalidValue                                                           = 1704,
    /// INVALIDATION-POLICY
    InvalidationPolicy                                                     = 2550,
    /// INVALIDATION-POLICYS
    InvalidationPolicys                                                    = 5087,
    /// INVERT
    Invert                                                                 = 4509,
    /// INVERT-CONDITION
    InvertCondition                                                        = 3565,
    /// INVOCATION
    Invocation                                                             = 5457,
    /// IO-CONTROL-CLASS-REF
    IoControlClassRef                                                      = 2268,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATION
    IoHwAbstractionServerAnnotation                                        = 5100,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATIONS
    IoHwAbstractionServerAnnotations                                       = 458,
    /// IP-ADDRESS
    IpAddress                                                              = 5270,
    /// IP-ADDRESS-KEEP-BEHAVIOR
    IpAddressKeepBehavior                                                  = 3187,
    /// IP-ADDRESS-PREFIX-LENGTH
    IpAddressPrefixLength                                                  = 910,
    /// IP-IAM-AUTHENTIC-CONNECTION-PROPS
    IpIamAuthenticConnectionProps                                          = 2351,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 3385,
    /// IP-PROTOCOL
    IpProtocol                                                             = 5052,
    /// IP-SEC-CONFIG
    IpSecConfig                                                            = 4551,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 1891,
    /// IP-SEC-CONFIG-PROPS-REF
    IpSecConfigPropsRef                                                    = 102,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 2782,
    /// IP-SEC-RULE
    IpSecRule                                                              = 2149,
    /// IP-SEC-RULES
    IpSecRules                                                             = 4518,
    /// IP-V-4-ADDRESS
    IpV4Address                                                            = 1368,
    /// IP-V-6-ADDRESS
    IpV6Address                                                            = 1203,
    /// IP-V-6-PATH-MTU-ENABLED
    IpV6PathMtuEnabled                                                     = 943,
    /// IP-V-6-PATH-MTU-TIMEOUT
    IpV6PathMtuTimeout                                                     = 3174,
    /// IPV-4-ADDRESS
    Ipv4Address                                                            = 3166,
    /// IPV-4-ADDRESS-SOURCE
    Ipv4AddressSource                                                      = 2455,
    /// IPV-4-CONFIGURATION
    Ipv4Configuration                                                      = 2769,
    /// IPV-4-DHCP-SERVER-CONFIGURATION
    Ipv4DhcpServerConfiguration                                            = 1590,
    /// IPV-4-MULTICAST-IP-ADDRESS
    Ipv4MulticastIpAddress                                                 = 1481,
    /// IPV-4-PROPS
    Ipv4Props                                                              = 2448,
    /// IPV-6-ADDRESS
    Ipv6Address                                                            = 3723,
    /// IPV-6-ADDRESS-SOURCE
    Ipv6AddressSource                                                      = 2216,
    /// IPV-6-CONFIGURATION
    Ipv6Configuration                                                      = 1706,
    /// IPV-6-DHCP-SERVER-CONFIGURATION
    Ipv6DhcpServerConfiguration                                            = 896,
    /// IPV-6-MULTICAST-IP-ADDRESS
    Ipv6MulticastIpAddress                                                 = 5218,
    /// IPV-6-PROPS
    Ipv6Props                                                              = 4320,
    /// IS-ACTIVATION-LINE-DEPENDENT
    IsActivationLineDependent                                              = 5106,
    /// IS-AUTO-VALUE
    IsAutoValue                                                            = 2163,
    /// IS-DEFAULT
    IsDefault                                                              = 5212,
    /// IS-DELTA-PACKAGE
    IsDeltaPackage                                                         = 2949,
    /// IS-DYNAMIC-LENGTH-FIELD-SIZE
    IsDynamicLengthFieldSize                                               = 5576,
    /// IS-EVENT
    IsEvent                                                                = 733,
    /// IS-EXTERNAL
    IsExternal                                                             = 1448,
    /// IS-GLOBAL
    IsGlobal                                                               = 3575,
    /// IS-MANDATORY
    IsMandatory                                                            = 89,
    /// IS-OBD-RELEVANT
    IsObdRelevant                                                          = 5018,
    /// IS-OPTIONAL
    IsOptional                                                             = 1384,
    /// IS-PRIVATE
    IsPrivate                                                              = 1347,
    /// IS-REENTRANT
    IsReentrant                                                            = 1663,
    /// IS-REQUIRED
    IsRequired                                                             = 645,
    /// IS-SERVICE
    IsService                                                              = 2364,
    /// IS-STRUCT-WITH-OPTIONAL-ELEMENT
    IsStructWithOptionalElement                                            = 4787,
    /// IS-SYNCHRONOUS
    IsSynchronous                                                          = 4996,
    /// IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER
    IsSystemWideGlobalTimeMaster                                           = 5246,
    /// IS-UNUSED
    IsUnused                                                               = 2270,
    /// IS-VARIADIC-TEMPLATE
    IsVariadicTemplate                                                     = 2230,
    /// ISSUED-BY
    IssuedBy                                                               = 2452,
    /// ISSUED-TRIGGERS
    IssuedTriggers                                                         = 272,
    /// ITEM
    Item                                                                   = 4345,
    /// ITEM-DEFINITIONS
    ItemDefinitions                                                        = 101,
    /// ITEM-LABEL
    ItemLabel                                                              = 472,
    /// ITEMS
    Items                                                                  = 3015,
    /// IUMPR-GROUP
    IumprGroup                                                             = 1197,
    /// IUMPR-GROUP-IDENTIFIERS
    IumprGroupIdentifiers                                                  = 2341,
    /// IUMPR-REF
    IumprRef                                                               = 2811,
    /// IUMPR-REFS
    IumprRefs                                                              = 4365,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 3192,
    /// J-1939-CLUSTER-CONDITIONAL
    J1939ClusterConditional                                                = 4791,
    /// J-1939-CLUSTER-VARIANTS
    J1939ClusterVariants                                                   = 1797,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 1808,
    /// J-1939-CONTROLLER-APPLICATION-REF
    J1939ControllerApplicationRef                                          = 1115,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING
    J1939ControllerApplicationToJ1939NmNodeMapping                         = 5190,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS
    J1939ControllerApplicationToJ1939NmNodeMappings                        = 2321,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 1099,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 4148,
    /// J-1939-DTC-VALUE
    J1939DtcValue                                                          = 3297,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 2785,
    /// J-1939-NM-ECU
    J1939NmEcu                                                             = 4005,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 203,
    /// J-1939-NM-NODE-REF
    J1939NmNodeRef                                                         = 2850,
    /// J-1939-REQUESTABLE
    J1939Requestable                                                       = 1967,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 1045,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 3223,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 5170,
    /// J-1939-SHARED-ADDRESS-CLUSTERS
    J1939SharedAddressClusters                                             = 359,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 4229,
    /// J-1939-TP-CONNECTION
    J1939TpConnection                                                      = 2896,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 2387,
    /// J-1939-TP-PG
    J1939TpPg                                                              = 2857,
    /// JITTER
    Jitter                                                                 = 2780,
    /// JOB-PRIMITIVE-REF
    JobPrimitiveRef                                                        = 3998,
    /// JOB-REF
    JobRef                                                                 = 3647,
    /// JOB-REQUIREMENTS
    JobRequirements                                                        = 3076,
    /// JUMP-TO-BOOT-LOADER
    JumpToBootLoader                                                       = 519,
    /// K-INIT-RESOURCE
    KInitResource                                                          = 3353,
    /// K-RUN-RESOURCE
    KRunResource                                                           = 3428,
    /// KEEP-ALIVE-INTERVAL
    KeepAliveInterval                                                      = 4604,
    /// KEEP-ALIVE-PROBES-MAX
    KeepAliveProbesMax                                                     = 3774,
    /// KEEP-ALIVE-TIME
    KeepAliveTime                                                          = 470,
    /// KEEP-ALIVES
    KeepAlives                                                             = 4614,
    /// KEY
    Key                                                                    = 5328,
    /// KEY-EXCHANGE-AUTHENTICATION-REF
    KeyExchangeAuthenticationRef                                           = 17,
    /// KEY-EXCHANGE-AUTHENTICATION-REFS
    KeyExchangeAuthenticationRefs                                          = 4691,
    /// KEY-EXCHANGE-REF
    KeyExchangeRef                                                         = 1703,
    /// KEY-EXCHANGE-REFS
    KeyExchangeRefs                                                        = 832,
    /// KEY-GENERATION
    KeyGeneration                                                          = 230,
    /// KEY-ID
    KeyId                                                                  = 1252,
    /// KEY-IDENTITY-REF
    KeyIdentityRef                                                         = 5134,
    /// KEY-REF
    KeyRef                                                                 = 4018,
    /// KEY-SIZE
    KeySize                                                                = 4394,
    /// KEY-SLOT-ALLOWED-MODIFICATION
    KeySlotAllowedModification                                             = 2414,
    /// KEY-SLOT-CONTENT-ALLOWED-USAGES
    KeySlotContentAllowedUsages                                            = 1855,
    /// KEY-SLOT-ID
    KeySlotId                                                              = 5363,
    /// KEY-SLOT-ONLY-ENABLED
    KeySlotOnlyEnabled                                                     = 2948,
    /// KEY-SLOT-REF
    KeySlotRef                                                             = 1334,
    /// KEY-SLOT-USAGE
    KeySlotUsage                                                           = 5405,
    /// KEY-SLOT-USED-FOR-START-UP
    KeySlotUsedForStartUp                                                  = 2889,
    /// KEY-SLOT-USED-FOR-SYNC
    KeySlotUsedForSync                                                     = 5318,
    /// KEY-SLOTS
    KeySlots                                                               = 5501,
    /// KEY-STORAGE-TYPE
    KeyStorageType                                                         = 4906,
    /// KEY-VALUE-PAIRS
    KeyValuePairs                                                          = 2235,
    /// KEY-VALUE-STORAGE-REF
    KeyValueStorageRef                                                     = 267,
    /// KEYWORD
    Keyword                                                                = 2427,
    /// KEYWORD-SET
    KeywordSet                                                             = 4885,
    /// KEYWORDS
    Keywords                                                               = 5577,
    /// KIND
    Kind                                                                   = 3875,
    /// L-1
    L1                                                                     = 3913,
    /// L-10
    L10                                                                    = 937,
    /// L-2
    L2                                                                     = 2721,
    /// L-4
    L4                                                                     = 1055,
    /// L-5
    L5                                                                     = 1867,
    /// L-GRAPHIC
    LGraphic                                                               = 4192,
    /// LABEL
    Label                                                                  = 1458,
    /// LABEL-1
    Label1                                                                 = 4768,
    /// LABELED-ITEM
    LabeledItem                                                            = 1852,
    /// LABELED-LIST
    LabeledList                                                            = 5404,
    /// LANGUAGE
    Language                                                               = 1628,
    /// LAST-EGRESS-SCHEDULER-REF
    LastEgressSchedulerRef                                                 = 5498,
    /// LATENCY-CONSTRAINT-TYPE
    LatencyConstraintType                                                  = 2131,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 4216,
    /// LATEST-TX
    LatestTx                                                               = 2910,
    /// LC-OBJECT-REF
    LcObjectRef                                                            = 3894,
    /// LC-STATE-REF
    LcStateRef                                                             = 294,
    /// LC-STATES
    LcStates                                                               = 490,
    /// LDAP-URL
    LdapUrl                                                                = 5032,
    /// LEAF-ELEMENT-IREF
    LeafElementIref                                                        = 5254,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS
    LegislatedFreezeFrameContentUdsObds                                    = 5138,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS
    LegislatedFreezeFrameContentWwhObds                                    = 3838,
    /// LENGTH
    Length                                                                 = 2805,
    /// LENGTH-CLIENT-ID
    LengthClientId                                                         = 5208,
    /// LENGTH-EXP
    LengthExp                                                              = 5310,
    /// LENGTH-SEQUENCE-COUNTER
    LengthSequenceCounter                                                  = 4806,
    /// LET-INTERVAL-REF
    LetIntervalRef                                                         = 803,
    /// LET-INTERVAL-REFS
    LetIntervalRefs                                                        = 2534,
    /// LICENSE-REF
    LicenseRef                                                             = 3036,
    /// LICENSE-REFS
    LicenseRefs                                                            = 5445,
    /// LIFE-CYCLE-INFO
    LifeCycleInfo                                                          = 1751,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 3945,
    /// LIFE-CYCLE-INFOS
    LifeCycleInfos                                                         = 3311,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 2791,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 3741,
    /// LIMIT
    Limit                                                                  = 1917,
    /// LIMIT-KIND
    LimitKind                                                              = 4756,
    /// LIN-CHECKSUM
    LinChecksum                                                            = 2680,
    /// LIN-CLUSTER
    LinCluster                                                             = 637,
    /// LIN-CLUSTER-CONDITIONAL
    LinClusterConditional                                                  = 4990,
    /// LIN-CLUSTER-VARIANTS
    LinClusterVariants                                                     = 3942,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 869,
    /// LIN-CONFIGURABLE-FRAME
    LinConfigurableFrame                                                   = 3915,
    /// LIN-CONFIGURABLE-FRAMES
    LinConfigurableFrames                                                  = 2514,
    /// LIN-ERROR-RESPONSE
    LinErrorResponse                                                       = 3654,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 2761,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 4905,
    /// LIN-MASTER
    LinMaster                                                              = 262,
    /// LIN-MASTER-CONDITIONAL
    LinMasterConditional                                                   = 548,
    /// LIN-MASTER-VARIANTS
    LinMasterVariants                                                      = 3837,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 4844,
    /// LIN-ORDERED-CONFIGURABLE-FRAME
    LinOrderedConfigurableFrame                                            = 3161,
    /// LIN-ORDERED-CONFIGURABLE-FRAMES
    LinOrderedConfigurableFrames                                           = 193,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 3544,
    /// LIN-PID-TO-CAN-ID-MAPPINGS
    LinPidToCanIdMappings                                                  = 148,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 2869,
    /// LIN-SLAVE
    LinSlave                                                               = 1969,
    /// LIN-SLAVE-CONDITIONAL
    LinSlaveConditional                                                    = 1546,
    /// LIN-SLAVE-CONFIG
    LinSlaveConfig                                                         = 2733,
    /// LIN-SLAVE-ECU-REF
    LinSlaveEcuRef                                                         = 527,
    /// LIN-SLAVE-VARIANTS
    LinSlaveVariants                                                       = 4076,
    /// LIN-SLAVES
    LinSlaves                                                              = 2642,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 3032,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 2032,
    /// LIN-TP-CONNECTION
    LinTpConnection                                                        = 1777,
    /// LIN-TP-N-SDU-REF
    LinTpNSduRef                                                           = 1503,
    /// LIN-TP-NODE
    LinTpNode                                                              = 5011,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 600,
    /// LIN-UNCONDITIONAL-FRAME-REF
    LinUnconditionalFrameRef                                               = 360,
    /// LIN-UNCONDITIONAL-FRAME-REFS
    LinUnconditionalFrameRefs                                              = 3493,
    /// LINKER
    Linker                                                                 = 2528,
    /// LINKERS
    Linkers                                                                = 3474,
    /// LIST
    List                                                                   = 3255,
    /// LISTEN-NOISE
    ListenNoise                                                            = 292,
    /// LISTEN-TIMEOUT
    ListenTimeout                                                          = 447,
    /// LITERAL-PREFIX
    LiteralPrefix                                                          = 2250,
    /// LITERALS
    Literals                                                               = 949,
    /// LIVELINESS-PROTECTION-KIND
    LivelinessProtectionKind                                               = 2222,
    /// LOAD-BALANCING-PRIORITY
    LoadBalancingPriority                                                  = 4325,
    /// LOAD-BALANCING-WEIGHT
    LoadBalancingWeight                                                    = 3328,
    /// LOC-MEASUREMENT-SET
    LocMeasurementSet                                                      = 1648,
    /// LOCAL-CERTIFICATE-REF
    LocalCertificateRef                                                    = 2391,
    /// LOCAL-CERTIFICATE-REFS
    LocalCertificateRefs                                                   = 2752,
    /// LOCAL-COM-ACCESS-CONTROL-ENABLED
    LocalComAccessControlEnabled                                           = 4453,
    /// LOCAL-COMM-CONNECTOR-REF
    LocalCommConnectorRef                                                  = 3643,
    /// LOCAL-DEBUG-DATAS
    LocalDebugDatas                                                        = 1952,
    /// LOCAL-ENDPOINT-CONFIG
    LocalEndpointConfig                                                    = 4491,
    /// LOCAL-ID
    LocalId                                                                = 4102,
    /// LOCAL-IP-SEC-RULE-REF
    LocalIpSecRuleRef                                                      = 998,
    /// LOCAL-IP-SEC-RULE-REFS
    LocalIpSecRuleRefs                                                     = 2558,
    /// LOCAL-NETWORK-ENDPOINT-REF
    LocalNetworkEndpointRef                                                = 2853,
    /// LOCAL-PARAMETER-REF
    LocalParameterRef                                                      = 5194,
    /// LOCAL-PORT-RANGE-END
    LocalPortRangeEnd                                                      = 3193,
    /// LOCAL-PORT-RANGE-START
    LocalPortRangeStart                                                    = 3584,
    /// LOCAL-PORT-REF
    LocalPortRef                                                           = 616,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 2155,
    /// LOCAL-SUPERVISION-ENTITY-REF
    LocalSupervisionEntityRef                                              = 1194,
    /// LOCAL-SUPERVISION-ENTITY-REFS
    LocalSupervisionEntityRefs                                             = 1426,
    /// LOCAL-SUPERVISION-REF
    LocalSupervisionRef                                                    = 4070,
    /// LOCAL-SUPERVISION-REFS
    LocalSupervisionRefs                                                   = 1201,
    /// LOCAL-SUPERVISIONS
    LocalSupervisions                                                      = 1713,
    /// LOCAL-TCP-PORT-REF
    LocalTcpPortRef                                                        = 1087,
    /// LOCAL-UDP-PORT-REF
    LocalUdpPortRef                                                        = 66,
    /// LOCAL-UNICAST-ADDRESSS
    LocalUnicastAddresss                                                   = 3300,
    /// LOCAL-VARIABLE-REF
    LocalVariableRef                                                       = 2969,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 4710,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 615,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 735,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF
    LogAndTraceMessageCollectionSetRef                                     = 1145,
    /// LOG-CHANNEL-ID
    LogChannelId                                                           = 4853,
    /// LOG-SINK-REF
    LogSinkRef                                                             = 2496,
    /// LOG-SINK-REFS
    LogSinkRefs                                                            = 3835,
    /// LOG-TRACE-DEFAULT-LOG-LEVEL
    LogTraceDefaultLogLevel                                                = 4122,
    /// LOG-TRACE-DEFAULT-LOG-THRESHOLD
    LogTraceDefaultLogThreshold                                            = 4198,
    /// LOG-TRACE-FILE-PATH
    LogTraceFilePath                                                       = 5207,
    /// LOG-TRACE-LOG-MODE
    LogTraceLogMode                                                        = 3530,
    /// LOG-TRACE-LOG-MODES
    LogTraceLogModes                                                       = 1802,
    /// LOG-TRACE-PORT-ID
    LogTracePortId                                                         = 2010,
    /// LOG-TRACE-PROCESS-DESC
    LogTraceProcessDesc                                                    = 4094,
    /// LOG-TRACE-PROCESS-ID
    LogTraceProcessId                                                      = 5615,
    /// LOGGING-BEHAVIOR
    LoggingBehavior                                                        = 439,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 4620,
    /// LOGIC-ADDRESSS
    LogicAddresss                                                          = 619,
    /// LOGICAL-ADDRESS
    LogicalAddress                                                         = 373,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 2319,
    /// LOGICAL-EXPRESSION-ARGUMENT-REF
    LogicalExpressionArgumentRef                                           = 4537,
    /// LOGICAL-EXPRESSION-ARGUMENT-REFS
    LogicalExpressionArgumentRefs                                          = 1504,
    /// LOGICAL-EXPRESSIONS
    LogicalExpressions                                                     = 237,
    /// LOGICAL-OPERATOR
    LogicalOperator                                                        = 4205,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 2093,
    /// LOGICAL-SUPERVISIONS
    LogicalSupervisions                                                    = 3726,
    /// LONG-NAME
    LongName                                                               = 2347,
    /// LONG-NAME-1
    LongName1                                                              = 3524,
    /// LOWER-BOUND
    LowerBound                                                             = 4840,
    /// LOWER-CAN-ID
    LowerCanId                                                             = 2475,
    /// LOWER-LIMIT
    LowerLimit                                                             = 27,
    /// LOWER-MULTIPLICITY
    LowerMultiplicity                                                      = 683,
    /// LUMINOUS-INTENSITY-EXP
    LuminousIntensityExp                                                   = 5521,
    /// M
    M                                                                      = 3813,
    /// MAC-LAYER-TYPE
    MacLayerType                                                           = 1037,
    /// MAC-MULTICAST-ADDRESS
    MacMulticastAddress                                                    = 2636,
    /// MAC-MULTICAST-ADDRESS-REF
    MacMulticastAddressRef                                                 = 5164,
    /// MAC-MULTICAST-ADDRESS-REFS
    MacMulticastAddressRefs                                                = 4243,
    /// MAC-MULTICAST-CONFIGURATION
    MacMulticastConfiguration                                              = 3573,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 898,
    /// MAC-MULTICAST-GROUP-REF
    MacMulticastGroupRef                                                   = 2592,
    /// MAC-MULTICAST-GROUPS
    MacMulticastGroups                                                     = 4503,
    /// MAC-UNICAST-ADDRESS
    MacUnicastAddress                                                      = 3361,
    /// MACHINE
    Machine                                                                = 1686,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 4314,
    /// MACHINE-DESIGN-REF
    MachineDesignRef                                                       = 1317,
    /// MACHINE-MODE-IREF
    MachineModeIref                                                        = 1729,
    /// MACHINE-MODE-IREFS
    MachineModeIrefs                                                       = 779,
    /// MACHINE-MODE-MACHINES
    MachineModeMachines                                                    = 5651,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 282,
    /// MACHINE-REF
    MachineRef                                                             = 221,
    /// MACHINE-TIMING
    MachineTiming                                                          = 4164,
    /// MACRO-INITIAL-OFFSET-A
    MacroInitialOffsetA                                                    = 1312,
    /// MACRO-INITIAL-OFFSET-B
    MacroInitialOffsetB                                                    = 5237,
    /// MACRO-PER-CYCLE
    MacroPerCycle                                                          = 2884,
    /// MACROTICK-DURATION
    MacrotickDuration                                                      = 337,
    /// MAIN-FUNCTION-PERIOD
    MainFunctionPeriod                                                     = 5414,
    /// MAJOR-VERSION
    MajorVersion                                                           = 170,
    /// MANAGED-COUPLING-PORTS
    ManagedCouplingPorts                                                   = 4015,
    /// MANAGED-MODE-GROUPS
    ManagedModeGroups                                                      = 685,
    /// MANAGED-PHYSICAL-CHANNEL-REF
    ManagedPhysicalChannelRef                                              = 941,
    /// MANAGED-PHYSICAL-CHANNEL-REFS
    ManagedPhysicalChannelRefs                                             = 4715,
    /// MANDATORY
    Mandatory                                                              = 5296,
    /// MANUFACTURER-CODE
    ManufacturerCode                                                       = 108,
    /// MANUFACTURER-VALIDATION-ORDER-REF
    ManufacturerValidationOrderRef                                         = 3586,
    /// MANUFACTURER-VALIDATION-ORDER-REFS
    ManufacturerValidationOrderRefs                                        = 2469,
    /// MAP
    Map                                                                    = 2322,
    /// MAP-ELEMENT-TUPLES
    MapElementTuples                                                       = 2738,
    /// MAPPED-AP-DATA-ELEMENT-IREF
    MappedApDataElementIref                                                = 1680,
    /// MAPPED-BSW-SERVICE-DEPENDENCY-REF
    MappedBswServiceDependencyRef                                          = 1429,
    /// MAPPED-DATA-ELEMENT-IREF
    MappedDataElementIref                                                  = 919,
    /// MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF
    MappedFlatSwcServiceDependencyRef                                      = 157,
    /// MAPPED-FUNCTION-REF
    MappedFunctionRef                                                      = 5037,
    /// MAPPED-OPERATION-IREF
    MappedOperationIref                                                    = 1417,
    /// MAPPED-SECURITY-EVENTS
    MappedSecurityEvents                                                   = 1699,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    MappedSwcServiceDependencyInExecutableIref                             = 1471,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    MappedSwcServiceDependencyInSystemIref                                 = 5378,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IREF
    MappedSwcServiceDependencyIref                                         = 4034,
    /// MAPPING-CONSTRAINTS
    MappingConstraints                                                     = 601,
    /// MAPPING-DIRECTION
    MappingDirection                                                       = 2929,
    /// MAPPING-REF
    MappingRef                                                             = 3086,
    /// MAPPING-SCOPE
    MappingScope                                                           = 1787,
    /// MAPPINGS
    Mappings                                                               = 5224,
    /// MASK
    Mask                                                                   = 3861,
    /// MASS-EXP
    MassExp                                                                = 2901,
    /// MASTER
    Master                                                                 = 1074,
    /// MASTER-EVENT-REF
    MasterEventRef                                                         = 5079,
    /// MASTER-REF
    MasterRef                                                              = 549,
    /// MASTER-TO-SLAVE-EVENT-MAPPINGS
    MasterToSlaveEventMappings                                             = 1294,
    /// MASTERED-TRIGGER-REF
    MasteredTriggerRef                                                     = 3922,
    /// MATCHING-CRITERION-REF
    MatchingCriterionRef                                                   = 5552,
    /// MAX
    Max                                                                    = 5489,
    /// MAX-ALIVE-CYCLE
    MaxAliveCycle                                                          = 2686,
    /// MAX-AR
    MaxAr                                                                  = 1908,
    /// MAX-AS
    MaxAs                                                                  = 42,
    /// MAX-BASE-TYPE-SIZE
    MaxBaseTypeSize                                                        = 3634,
    /// MAX-BLOCK-SIZE
    MaxBlockSize                                                           = 575,
    /// MAX-BS
    MaxBs                                                                  = 4059,
    /// MAX-BUFFER-REQUEST
    MaxBufferRequest                                                       = 584,
    /// MAX-BUFFER-SIZE
    MaxBufferSize                                                          = 4854,
    /// MAX-BYTES-IN-INTERVAL
    MaxBytesInInterval                                                     = 3925,
    /// MAX-COMM-MODE
    MaxCommMode                                                            = 5408,
    /// MAX-CYCLES
    MaxCycles                                                              = 84,
    /// MAX-DATA-LENGTH
    MaxDataLength                                                          = 1630,
    /// MAX-DEADLINE
    MaxDeadline                                                            = 856,
    /// MAX-DELTA-COUNTER
    MaxDeltaCounter                                                        = 3802,
    /// MAX-DELTA-COUNTER-INIT
    MaxDeltaCounterInit                                                    = 660,
    /// MAX-DID-TO-READ
    MaxDidToRead                                                           = 3721,
    /// MAX-DIFF
    MaxDiff                                                                = 2088,
    /// MAX-ERROR-STATE-INIT
    MaxErrorStateInit                                                      = 5192,
    /// MAX-ERROR-STATE-INVALID
    MaxErrorStateInvalid                                                   = 1585,
    /// MAX-ERROR-STATE-VALID
    MaxErrorStateValid                                                     = 3305,
    /// MAX-EVENTS-IN-INTERVAL
    MaxEventsInInterval                                                    = 4852,
    /// MAX-EXP-BS
    MaxExpBs                                                               = 933,
    /// MAX-FC-WAIT
    MaxFcWait                                                              = 3127,
    /// MAX-FR-IF
    MaxFrIf                                                                = 3857,
    /// MAX-GRADIENT
    MaxGradient                                                            = 5412,
    /// MAX-HEADER-ID
    MaxHeaderId                                                            = 3046,
    /// MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    MaxInitialVehicleAnnouncementTime                                      = 1097,
    /// MAX-LENGTH
    MaxLength                                                              = 2745,
    /// MAX-MARGIN
    MaxMargin                                                              = 4939,
    /// MAX-NO-NEW-OR-REPEATED-DATA
    MaxNoNewOrRepeatedData                                                 = 3549,
    /// MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS
    MaxNumChangeOfDataIdentfierEvents                                      = 256,
    /// MAX-NUM-COMPARISION-OF-VALUE-EVENTS
    MaxNumComparisionOfValueEvents                                         = 3052,
    /// MAX-NUMBER-FREEZE-FRAME-RECORDS
    MaxNumberFreezeFrameRecords                                            = 2194,
    /// MAX-NUMBER-OF-ALLOWED-UPDATES
    MaxNumberOfAllowedUpdates                                              = 4531,
    /// MAX-NUMBER-OF-CONNECTIONS
    MaxNumberOfConnections                                                 = 4215,
    /// MAX-NUMBER-OF-ELEMENTS
    MaxNumberOfElements                                                    = 2962,
    /// MAX-NUMBER-OF-EVENT-ENTRIES
    MaxNumberOfEventEntries                                                = 2350,
    /// MAX-NUMBER-OF-FILES
    MaxNumberOfFiles                                                       = 3037,
    /// MAX-NUMBER-OF-NPDU-PER-CYCLE
    MaxNumberOfNpduPerCycle                                                = 1402,
    /// MAX-NUMBER-OF-OCCURRENCES
    MaxNumberOfOccurrences                                                 = 557,
    /// MAX-NUMBER-OF-PARALLEL-TRANSFERS
    MaxNumberOfParallelTransfers                                           = 1035,
    /// MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING
    MaxNumberOfRequestCorrectlyReceivedResponsePending                     = 714,
    /// MAX-NUMBER-OF-RESP-PENDING-FRAMES
    MaxNumberOfRespPendingFrames                                           = 607,
    /// MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS
    MaxNumberOfStoredDtcStatusChangedEvents                                = 3373,
    /// MAX-NUMBER-OF-TIME-QUANTA-PER-BIT
    MaxNumberOfTimeQuantaPerBit                                            = 1305,
    /// MAX-PERIODIC-DID-TO-READ
    MaxPeriodicDidToRead                                                   = 5093,
    /// MAX-REQUEST-BYTES
    MaxRequestBytes                                                        = 4151,
    /// MAX-RETRIES
    MaxRetries                                                             = 2632,
    /// MAX-RPT-EVENT-ID
    MaxRptEventId                                                          = 4170,
    /// MAX-SAMPLE-POINT
    MaxSamplePoint                                                         = 3112,
    /// MAX-SERVICE-POINT-ID
    MaxServicePointId                                                      = 5342,
    /// MAX-SIZE-TO-FILL
    MaxSizeToFill                                                          = 5411,
    /// MAX-SLOTS
    MaxSlots                                                               = 3681,
    /// MAX-SOURCE-ELEMENT
    MaxSourceElement                                                       = 3379,
    /// MAX-SUPPORTED-DID-LENGTH
    MaxSupportedDidLength                                                  = 4602,
    /// MAX-SYNC-JUMP-WIDTH
    MaxSyncJumpWidth                                                       = 3259,
    /// MAX-TESTER-CONNECTIONS
    MaxTesterConnections                                                   = 564,
    /// MAX-TRCV-DELAY-COMPENSATION-OFFSET
    MaxTrcvDelayCompensationOffset                                         = 696,
    /// MAX-VALUE
    MaxValue                                                               = 224,
    /// MAX-WITHOUT-CLOCK-CORRECTION-FATAL
    MaxWithoutClockCorrectionFatal                                         = 4617,
    /// MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE
    MaxWithoutClockCorrectionPassive                                       = 2807,
    /// MAXIMUM
    Maximum                                                                = 56,
    /// MAXIMUM-ALLOWED-SIZE
    MaximumAllowedSize                                                     = 4968,
    /// MAXIMUM-DISTANCES
    MaximumDistances                                                       = 4182,
    /// MAXIMUM-DYNAMIC-PAYLOAD-LENGTH
    MaximumDynamicPayloadLength                                            = 4132,
    /// MAXIMUM-EXECUTION-TIME
    MaximumExecutionTime                                                   = 1541,
    /// MAXIMUM-INTENDED-BINDING-TIME
    MaximumIntendedBindingTime                                             = 3988,
    /// MAXIMUM-INTER-ARRIVAL-TIME
    MaximumInterArrivalTime                                                = 3621,
    /// MAXIMUM-KEY-LENGTH
    MaximumKeyLength                                                       = 2991,
    /// MAXIMUM-LENGTH
    MaximumLength                                                          = 2579,
    /// MAXIMUM-MEMORY-CONSUMPTION
    MaximumMemoryConsumption                                               = 5565,
    /// MAXIMUM-MESSAGE-LENGTH
    MaximumMessageLength                                                   = 541,
    /// MAXIMUM-RECEIVE-BUFFER-LENGTH
    MaximumReceiveBufferLength                                             = 1295,
    /// MAXIMUM-SEGMENT-LENGTH
    MaximumSegmentLength                                                   = 5288,
    /// MAXIMUM-SEGMENT-LENGTH-REQUEST
    MaximumSegmentLengthRequest                                            = 68,
    /// MAXIMUM-SEGMENT-LENGTH-RESPONSE
    MaximumSegmentLengthResponse                                           = 215,
    /// MAXIMUM-SELECTED-BINDING-TIME
    MaximumSelectedBindingTime                                             = 4484,
    /// MAXIMUM-SUPPORTED-UCM-MASTER-VERSION
    MaximumSupportedUcmMasterVersion                                       = 1932,
    /// MAXIMUM-SUPPORTED-UCM-VERSION
    MaximumSupportedUcmVersion                                             = 3598,
    /// MAXIMUM-TRANSMISSION-UNIT
    MaximumTransmissionUnit                                                = 5611,
    /// MAXIMUM-TRANSMIT-BUFFER-LENGTH
    MaximumTransmitBufferLength                                            = 5155,
    /// MAY-BE-UNCONNECTED
    MayBeUnconnected                                                       = 3411,
    /// MC-DATA-ACCESS-DETAILS
    McDataAccessDetails                                                    = 2108,
    /// MC-DATA-ASSIGNMENTS
    McDataAssignments                                                      = 3330,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 474,
    /// MC-DATA-INSTANCE-REF
    McDataInstanceRef                                                      = 982,
    /// MC-DATA-INSTANCE-REFS
    McDataInstanceRefs                                                     = 3601,
    /// MC-DATA-INSTANCE-VAR-REF
    McDataInstanceVarRef                                                   = 1653,
    /// MC-FUNCTION
    McFunction                                                             = 2326,
    /// MC-FUNCTION-DATA-REF-SET-CONDITIONAL
    McFunctionDataRefSetConditional                                        = 3512,
    /// MC-FUNCTION-DATA-REF-SET-VARIANTS
    McFunctionDataRefSetVariants                                           = 4673,
    /// MC-FUNCTION-REF
    McFunctionRef                                                          = 4942,
    /// MC-FUNCTION-REFS
    McFunctionRefs                                                         = 5559,
    /// MC-GROUP
    McGroup                                                                = 5613,
    /// MC-GROUP-DATA-REF-SET-CONDITIONAL
    McGroupDataRefSetConditional                                           = 3955,
    /// MC-GROUP-DATA-REF-SET-VARIANTS
    McGroupDataRefSetVariants                                              = 3484,
    /// MC-PARAMETER-ELEMENT-GROUP
    McParameterElementGroup                                                = 3219,
    /// MC-PARAMETER-INSTANCES
    McParameterInstances                                                   = 5617,
    /// MC-SUPPORT
    McSupport                                                              = 4780,
    /// MC-SW-EMULATION-METHOD-SUPPORT
    McSwEmulationMethodSupport                                             = 3620,
    /// MC-VARIABLE-INSTANCES
    McVariableInstances                                                    = 3190,
    /// MCD-IDENTIFIER
    McdIdentifier                                                          = 204,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REF
    MeasurableSystemConstantValuesRef                                      = 1719,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS
    MeasurableSystemConstantValuesRefs                                     = 2727,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 565,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 4486,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 4001,
    /// MEM-ALIGNMENT
    MemAlignment                                                           = 1868,
    /// MEM-CLASS-SYMBOL
    MemClassSymbol                                                         = 2128,
    /// MEM-USAGE
    MemUsage                                                               = 855,
    /// MEMORY-ALLOCATION-KEYWORD-POLICY
    MemoryAllocationKeywordPolicy                                          = 3633,
    /// MEMORY-CONSUMPTION
    MemoryConsumption                                                      = 5361,
    /// MEMORY-DESTINATION-REF
    MemoryDestinationRef                                                   = 3841,
    /// MEMORY-DESTINATION-REFS
    MemoryDestinationRefs                                                  = 3860,
    /// MEMORY-ENTRY-STORAGE-TRIGGER
    MemoryEntryStorageTrigger                                              = 4125,
    /// MEMORY-HIGH-ADDRESS
    MemoryHighAddress                                                      = 1733,
    /// MEMORY-HIGH-ADDRESS-LABEL
    MemoryHighAddressLabel                                                 = 2331,
    /// MEMORY-ID
    MemoryId                                                               = 2185,
    /// MEMORY-LOW-ADDRESS
    MemoryLowAddress                                                       = 1769,
    /// MEMORY-LOW-ADDRESS-LABEL
    MemoryLowAddressLabel                                                  = 1709,
    /// MEMORY-RANGE-REF
    MemoryRangeRef                                                         = 93,
    /// MEMORY-RANGE-REFS
    MemoryRangeRefs                                                        = 3583,
    /// MEMORY-SECTION
    MemorySection                                                          = 3067,
    /// MEMORY-SECTION-LOCATION
    MemorySectionLocation                                                  = 1971,
    /// MEMORY-SECTION-LOCATIONS
    MemorySectionLocations                                                 = 3909,
    /// MEMORY-SECTIONS
    MemorySections                                                         = 5012,
    /// MESSAGE-COMPLIANCE
    MessageCompliance                                                      = 4917,
    /// MESSAGE-ID
    MessageId                                                              = 3778,
    /// MESSAGE-LINE-NUMBER
    MessageLineNumber                                                      = 4965,
    /// MESSAGE-LINK-LENGTH
    MessageLinkLength                                                      = 5551,
    /// MESSAGE-LINK-POSITION
    MessageLinkPosition                                                    = 3502,
    /// MESSAGE-SOURCE-FILE
    MessageSourceFile                                                      = 3981,
    /// MESSAGE-TYPE
    MessageType                                                            = 2675,
    /// MESSAGE-TYPE-INFO
    MessageTypeInfo                                                        = 5645,
    /// META-DATA-FIELDS
    MetaDataFields                                                         = 4372,
    /// META-DATA-ITEM
    MetaDataItem                                                           = 186,
    /// META-DATA-ITEM-SET
    MetaDataItemSet                                                        = 3754,
    /// META-DATA-ITEM-SETS
    MetaDataItemSets                                                       = 2747,
    /// META-DATA-ITEM-TYPE
    MetaDataItemType                                                       = 3646,
    /// META-DATA-ITEMS
    MetaDataItems                                                          = 1314,
    /// META-DATA-LENGTH
    MetaDataLength                                                         = 3688,
    /// METADATA-PROTECTION-KIND
    MetadataProtectionKind                                                 = 4069,
    /// METHOD-ACTIVATION-ROUTING-GROUPS
    MethodActivationRoutingGroups                                          = 4773,
    /// METHOD-CALL-REF
    MethodCallRef                                                          = 4918,
    /// METHOD-DEPLOYMENTS
    MethodDeployments                                                      = 3031,
    /// METHOD-ID
    MethodId                                                               = 2221,
    /// METHOD-IREF
    MethodIref                                                             = 3099,
    /// METHOD-MAPPING
    MethodMapping                                                          = 1083,
    /// METHOD-MAPPINGS
    MethodMappings                                                         = 5219,
    /// METHOD-QOS-PROPSS
    MethodQosPropss                                                        = 3513,
    /// METHOD-REF
    MethodRef                                                              = 3628,
    /// METHOD-REFS
    MethodRefs                                                             = 1571,
    /// METHOD-REPLY-TOPIC-NAME
    MethodReplyTopicName                                                   = 104,
    /// METHOD-REQUEST-PROPSS
    MethodRequestPropss                                                    = 2091,
    /// METHOD-REQUEST-TOPIC-NAME
    MethodRequestTopicName                                                 = 2376,
    /// METHOD-RESPONSE-PROPSS
    MethodResponsePropss                                                   = 2458,
    /// METHOD-RETURN-REF
    MethodReturnRef                                                        = 177,
    /// METHOD-TOPICS-ACCESS-RULE-REF
    MethodTopicsAccessRuleRef                                              = 2794,
    /// METHODS
    Methods                                                                = 4008,
    /// MICRO-INITIAL-OFFSET-A
    MicroInitialOffsetA                                                    = 5453,
    /// MICRO-INITIAL-OFFSET-B
    MicroInitialOffsetB                                                    = 3325,
    /// MICRO-PER-CYCLE
    MicroPerCycle                                                          = 1632,
    /// MICROTICK-DURATION
    MicrotickDuration                                                      = 2988,
    /// MIN
    Min                                                                    = 3807,
    /// MIN-ALIVE-CYCLE
    MinAliveCycle                                                          = 3412,
    /// MIN-DATA-LENGTH
    MinDataLength                                                          = 3904,
    /// MIN-DEADLINE
    MinDeadline                                                            = 3536,
    /// MIN-HEADER-ID
    MinHeaderId                                                            = 4863,
    /// MIN-LENGTH
    MinLength                                                              = 2381,
    /// MIN-MARGIN
    MinMargin                                                              = 2058,
    /// MIN-NUMBER-OF-ELEMENTS
    MinNumberOfElements                                                    = 5195,
    /// MIN-NUMBER-OF-OCCURRENCES
    MinNumberOfOccurrences                                                 = 1127,
    /// MIN-NUMBER-OF-TIME-QUANTA-PER-BIT
    MinNumberOfTimeQuantaPerBit                                            = 3439,
    /// MIN-OK-STATE-INIT
    MinOkStateInit                                                         = 1119,
    /// MIN-OK-STATE-INVALID
    MinOkStateInvalid                                                      = 1057,
    /// MIN-OK-STATE-VALID
    MinOkStateValid                                                        = 1288,
    /// MIN-RPT-EVENT-ID
    MinRptEventId                                                          = 96,
    /// MIN-SAMPLE-POINT
    MinSamplePoint                                                         = 2735,
    /// MIN-SERVICE-POINT-ID
    MinServicePointId                                                      = 3406,
    /// MIN-SYNC-JUMP-WIDTH
    MinSyncJumpWidth                                                       = 1715,
    /// MIN-TRCV-DELAY-COMPENSATION-OFFSET
    MinTrcvDelayCompensationOffset                                         = 3171,
    /// MIN-VALUE
    MinValue                                                               = 4742,
    /// MINIMUM
    Minimum                                                                = 2867,
    /// MINIMUM-DELAY
    MinimumDelay                                                           = 4493,
    /// MINIMUM-DISTANCES
    MinimumDistances                                                       = 2993,
    /// MINIMUM-EXECUTION-TIME
    MinimumExecutionTime                                                   = 2750,
    /// MINIMUM-FIFO-LENGTH
    MinimumFifoLength                                                      = 3110,
    /// MINIMUM-INTENDED-BINDING-TIME
    MinimumIntendedBindingTime                                             = 5274,
    /// MINIMUM-INTER-ARRIVAL-TIME
    MinimumInterArrivalTime                                                = 5519,
    /// MINIMUM-INTERVAL-LENGTH
    MinimumIntervalLength                                                  = 1355,
    /// MINIMUM-MEMORY-CONSUMPTION
    MinimumMemoryConsumption                                               = 4495,
    /// MINIMUM-MULTICAST-SEPERATION-TIME
    MinimumMulticastSeperationTime                                         = 866,
    /// MINIMUM-RX-CONTAINER-QUEUE-SIZE
    MinimumRxContainerQueueSize                                            = 1711,
    /// MINIMUM-SELECTED-BINDING-TIME
    MinimumSelectedBindingTime                                             = 2933,
    /// MINIMUM-SEND-INTERVAL
    MinimumSendInterval                                                    = 1373,
    /// MINIMUM-SEPARATION-TIME
    MinimumSeparationTime                                                  = 4669,
    /// MINIMUM-START-INTERVAL
    MinimumStartInterval                                                   = 3993,
    /// MINIMUM-SUPPORTED-UCM-MASTER-VERSION
    MinimumSupportedUcmMasterVersion                                       = 4086,
    /// MINIMUM-SUPPORTED-UCM-VERSION
    MinimumSupportedUcmVersion                                             = 3431,
    /// MINIMUM-SUSTAINED-SIZE
    MinimumSustainedSize                                                   = 3547,
    /// MINIMUM-TIMER-GRANULARITY
    MinimumTimerGranularity                                                = 3787,
    /// MINIMUM-TX-CONTAINER-QUEUE-SIZE
    MinimumTxContainerQueueSize                                            = 5430,
    /// MINISLOT-ACTION-POINT-OFFSET
    MinislotActionPointOffset                                              = 4592,
    /// MINISLOT-DURATION
    MinislotDuration                                                       = 3227,
    /// MINOR-VERSION
    MinorVersion                                                           = 2703,
    /// MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID
    MirrorSourceLinToCanRangeBaseId                                        = 1019,
    /// MIRROR-STATUS-CAN-ID
    MirrorStatusCanId                                                      = 4739,
    /// ML-GRAPHIC
    MlGraphic                                                              = 123,
    /// MODE
    Mode                                                                   = 3999,
    /// MODE-ACCESS-POINT
    ModeAccessPoint                                                        = 181,
    /// MODE-ACCESS-POINTS
    ModeAccessPoints                                                       = 3705,
    /// MODE-CONDITION
    ModeCondition                                                          = 3714,
    /// MODE-CONDITION-REF
    ModeConditionRef                                                       = 5156,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 3195,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 3950,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 1396,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-IREF
    ModeDeclarationGroupPrototypeIref                                      = 5313,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ModeDeclarationGroupPrototypeRef                                       = 4769,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL
    ModeDeclarationGroupPrototypeRefConditional                            = 2999,
    /// MODE-DECLARATION-GROUP-REF
    ModeDeclarationGroupRef                                                = 60,
    /// MODE-DECLARATION-GROUP-REFS
    ModeDeclarationGroupRefs                                               = 517,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 2917,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 41,
    /// MODE-DECLARATION-MAPPING-SET-REF
    ModeDeclarationMappingSetRef                                           = 5647,
    /// MODE-DECLARATION-MAPPINGS
    ModeDeclarationMappings                                                = 4222,
    /// MODE-DECLARATION-REF
    ModeDeclarationRef                                                     = 3926,
    /// MODE-DECLARATION-REFS
    ModeDeclarationRefs                                                    = 1949,
    /// MODE-DECLARATIONS
    ModeDeclarations                                                       = 1742,
    /// MODE-DEPENDENT-STARTUP-CONFIG
    ModeDependentStartupConfig                                             = 1315,
    /// MODE-DEPENDENT-STARTUP-CONFIGS
    ModeDependentStartupConfigs                                            = 5435,
    /// MODE-DRIVEN-FALSE-CONDITIONS
    ModeDrivenFalseConditions                                              = 4051,
    /// MODE-DRIVEN-TRANSMISSION-MODE-CONDITION
    ModeDrivenTransmissionModeCondition                                    = 732,
    /// MODE-DRIVEN-TRUE-CONDITIONS
    ModeDrivenTrueConditions                                               = 4231,
    /// MODE-ELEMENT-REF
    ModeElementRef                                                         = 5215,
    /// MODE-ELEMENTS
    ModeElements                                                           = 4609,
    /// MODE-GROUP
    ModeGroup                                                              = 136,
    /// MODE-GROUP-IREF
    ModeGroupIref                                                          = 1048,
    /// MODE-GROUP-REF
    ModeGroupRef                                                           = 4276,
    /// MODE-IN-BSW-INSTANCE-REF
    ModeInBswInstanceRef                                                   = 2511,
    /// MODE-IN-SWC-INSTANCE-REF
    ModeInSwcInstanceRef                                                   = 3642,
    /// MODE-INSTANCE
    ModeInstance                                                           = 5467,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 516,
    /// MODE-IREF
    ModeIref                                                               = 4066,
    /// MODE-IREFS
    ModeIrefs                                                              = 5107,
    /// MODE-MANAGER-ERROR-BEHAVIOR
    ModeManagerErrorBehavior                                               = 2359,
    /// MODE-MAPPING
    ModeMapping                                                            = 205,
    /// MODE-PORT-ANNOTATION
    ModePortAnnotation                                                     = 3034,
    /// MODE-PORT-ANNOTATIONS
    ModePortAnnotations                                                    = 837,
    /// MODE-RECEIVER-POLICYS
    ModeReceiverPolicys                                                    = 5124,
    /// MODE-REF
    ModeRef                                                                = 2020,
    /// MODE-REPORTER-PORT-IREF
    ModeReporterPortIref                                                   = 2423,
    /// MODE-REQUEST-TYPE-MAP
    ModeRequestTypeMap                                                     = 4358,
    /// MODE-REQUEST-TYPE-MAPS
    ModeRequestTypeMaps                                                    = 4655,
    /// MODE-SENDER-POLICYS
    ModeSenderPolicys                                                      = 4045,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY
    ModeSwitchEventTriggeredActivity                                       = 4250,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS
    ModeSwitchEventTriggeredActivitys                                      = 141,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 641,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 760,
    /// MODE-SWITCH-POINTS
    ModeSwitchPoints                                                       = 5289,
    /// MODE-SWITCH-RECEIVER-COM-SPEC
    ModeSwitchReceiverComSpec                                              = 4985,
    /// MODE-SWITCH-SENDER-COM-SPEC
    ModeSwitchSenderComSpec                                                = 757,
    /// MODE-SWITCHED-ACK
    ModeSwitchedAck                                                        = 1692,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 3398,
    /// MODE-TRANSITION
    ModeTransition                                                         = 980,
    /// MODE-TRANSITIONS
    ModeTransitions                                                        = 3186,
    /// MODE-USER-ERROR-BEHAVIOR
    ModeUserErrorBehavior                                                  = 12,
    /// MODEL-OBJECT-REFERENCE
    ModelObjectReference                                                   = 5628,
    /// MODES
    Modes                                                                  = 4633,
    /// MODIFICATION
    Modification                                                           = 2972,
    /// MODIFICATIONS
    Modifications                                                          = 3680,
    /// MODIFIED-DATAS
    ModifiedDatas                                                          = 4188,
    /// MODULE-DESCRIPTION-REF
    ModuleDescriptionRef                                                   = 4241,
    /// MODULE-ID
    ModuleId                                                               = 2595,
    /// MODULE-INSTANTIATION-REF
    ModuleInstantiationRef                                                 = 1275,
    /// MODULE-INSTANTIATION-REFS
    ModuleInstantiationRefs                                                = 4388,
    /// MODULE-INSTANTIATIONS
    ModuleInstantiations                                                   = 3682,
    /// MODULE-REF
    ModuleRef                                                              = 1725,
    /// MODULE-REFS
    ModuleRefs                                                             = 1581,
    /// MOLAR-AMOUNT-EXP
    MolarAmountExp                                                         = 5416,
    /// MONITORED-IDENTIFIER-REF
    MonitoredIdentifierRef                                                 = 4670,
    /// MONOTONY
    Monotony                                                               = 3651,
    /// MSG-ID-MASK
    MsgIdMask                                                              = 3972,
    /// MSG-ID-MATCH
    MsgIdMatch                                                             = 146,
    /// MSR-QUERY-ARG
    MsrQueryArg                                                            = 2384,
    /// MSR-QUERY-CHAPTER
    MsrQueryChapter                                                        = 3253,
    /// MSR-QUERY-NAME
    MsrQueryName                                                           = 3407,
    /// MSR-QUERY-P-1
    MsrQueryP1                                                             = 4659,
    /// MSR-QUERY-P-2
    MsrQueryP2                                                             = 3780,
    /// MSR-QUERY-PROPS
    MsrQueryProps                                                          = 2564,
    /// MSR-QUERY-RESULT-CHAPTER
    MsrQueryResultChapter                                                  = 1978,
    /// MSR-QUERY-RESULT-P-1
    MsrQueryResultP1                                                       = 4727,
    /// MSR-QUERY-RESULT-P-2
    MsrQueryResultP2                                                       = 2539,
    /// MSR-QUERY-RESULT-TOPIC-1
    MsrQueryResultTopic1                                                   = 606,
    /// MSR-QUERY-TOPIC-1
    MsrQueryTopic1                                                         = 5515,
    /// MULTICAST-CONNECTOR-REF
    MulticastConnectorRef                                                  = 1730,
    /// MULTICAST-CONNECTOR-REFS
    MulticastConnectorRefs                                                 = 2236,
    /// MULTICAST-CREDENTIALS
    MulticastCredentials                                                   = 5166,
    /// MULTICAST-REF
    MulticastRef                                                           = 3342,
    /// MULTICAST-SD-IP-ADDRESS-REF
    MulticastSdIpAddressRef                                                = 1304,
    /// MULTICAST-SECURE-COM-PROPS-REF
    MulticastSecureComPropsRef                                             = 1353,
    /// MULTICAST-SEGMENTATION
    MulticastSegmentation                                                  = 659,
    /// MULTICAST-THRESHOLD
    MulticastThreshold                                                     = 3425,
    /// MULTICAST-UDP-PORT
    MulticastUdpPort                                                       = 776,
    /// MULTIPLE-CONFIGURATION-CONTAINER
    MultipleConfigurationContainer                                         = 2730,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 2174,
    /// MULTIPLICITY-CONFIG-CLASSES
    MultiplicityConfigClasses                                              = 2099,
    /// MULTIPLICITY-RESTRICTION
    MultiplicityRestriction                                                = 2030,
    /// N
    N                                                                      = 1741,
    /// N-DATA-SETS
    NDataSets                                                              = 1338,
    /// N-PDU
    NPdu                                                                   = 1640,
    /// N-PDU-REF
    NPduRef                                                                = 440,
    /// N-PDU-REFS
    NPduRefs                                                               = 169,
    /// N-ROM-BLOCKS
    NRomBlocks                                                             = 5544,
    /// NAGLES-ALGORITHM
    NaglesAlgorithm                                                        = 1950,
    /// NAME
    Name                                                                   = 4450,
    /// NAME-TOKEN-VALUE-VARIATION-POINT
    NameTokenValueVariationPoint                                           = 2860,
    /// NAMED-CURVE-ID
    NamedCurveId                                                           = 3470,
    /// NAMESPACES
    Namespaces                                                             = 727,
    /// NAS-TIMEOUT
    NasTimeout                                                             = 2916,
    /// NATIVE-DECLARATION
    NativeDeclaration                                                      = 2430,
    /// NDP-PROPS
    NdpProps                                                               = 2089,
    /// NEEDS-ORIGINAL-DATA
    NeedsOriginalData                                                      = 1828,
    /// NEIGHBOR-CACHE-SIZE
    NeighborCacheSize                                                      = 1193,
    /// NESTED-ELEMENT-REF
    NestedElementRef                                                       = 3142,
    /// NESTED-ELEMENT-REFS
    NestedElementRefs                                                      = 3995,
    /// NESTED-ELEMENTS
    NestedElements                                                         = 4302,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 1726,
    /// NETWORK-CONFIGURATION-REF
    NetworkConfigurationRef                                                = 3444,
    /// NETWORK-CONFIGURATIONS
    NetworkConfigurations                                                  = 1678,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 2849,
    /// NETWORK-ENDPOINT-ADDRESSES
    NetworkEndpointAddresses                                               = 3971,
    /// NETWORK-ENDPOINT-REF
    NetworkEndpointRef                                                     = 2087,
    /// NETWORK-ENDPOINT-REFS
    NetworkEndpointRefs                                                    = 95,
    /// NETWORK-ENDPOINTS
    NetworkEndpoints                                                       = 322,
    /// NETWORK-HANDLES
    NetworkHandles                                                         = 4873,
    /// NETWORK-ID
    NetworkId                                                              = 1567,
    /// NETWORK-IDLE-TIME
    NetworkIdleTime                                                        = 773,
    /// NETWORK-INTERFACE-ID
    NetworkInterfaceId                                                     = 1346,
    /// NETWORK-INTERFACE-REF
    NetworkInterfaceRef                                                    = 1320,
    /// NETWORK-INTERFACES
    NetworkInterfaces                                                      = 2906,
    /// NETWORK-MANAGEMENT-VECTOR-LENGTH
    NetworkManagementVectorLength                                          = 932,
    /// NETWORK-MASK
    NetworkMask                                                            = 918,
    /// NETWORK-REPRESENTATION
    NetworkRepresentation                                                  = 839,
    /// NETWORK-REPRESENTATION-PROPS
    NetworkRepresentationProps                                             = 4553,
    /// NETWORK-SEGMENT-ID
    NetworkSegmentId                                                       = 959,
    /// NETWORK-TIME-CONSUMER-REF
    NetworkTimeConsumerRef                                                 = 5019,
    /// NETWORK-TIME-MASTER-REF
    NetworkTimeMasterRef                                                   = 1472,
    /// NETWORK-TIME-PROVIDER-REF
    NetworkTimeProviderRef                                                 = 1798,
    /// NETWORK-TIME-SLAVE-REF
    NetworkTimeSlaveRef                                                    = 3242,
    /// NEW-NAD
    NewNad                                                                 = 2348,
    /// NEXT-HIGHER-CERTIFICATE-REF
    NextHigherCertificateRef                                               = 4374,
    /// NM-ACTIVE-COORDINATOR
    NmActiveCoordinator                                                    = 33,
    /// NM-BUS-LOAD-REDUCTION-ENABLED
    NmBusLoadReductionEnabled                                              = 1814,
    /// NM-BUS-SYNCHRONIZATION-ENABLED
    NmBusSynchronizationEnabled                                            = 2143,
    /// NM-BUSLOAD-REDUCTION-ACTIVE
    NmBusloadReductionActive                                               = 1070,
    /// NM-BUSLOAD-REDUCTION-ENABLED
    NmBusloadReductionEnabled                                              = 4249,
    /// NM-CAR-WAKE-UP-BIT-POSITION
    NmCarWakeUpBitPosition                                                 = 940,
    /// NM-CAR-WAKE-UP-FILTER-ENABLED
    NmCarWakeUpFilterEnabled                                               = 4639,
    /// NM-CAR-WAKE-UP-FILTER-NODE-ID
    NmCarWakeUpFilterNodeId                                                = 699,
    /// NM-CAR-WAKE-UP-RX-ENABLED
    NmCarWakeUpRxEnabled                                                   = 4465,
    /// NM-CBV-POSITION
    NmCbvPosition                                                          = 3853,
    /// NM-CHANNEL-ACTIVE
    NmChannelActive                                                        = 2103,
    /// NM-CHANNEL-ID
    NmChannelId                                                            = 563,
    /// NM-CHANNEL-SLEEP-MASTER
    NmChannelSleepMaster                                                   = 305,
    /// NM-CLUSTER-COUPLINGS
    NmClusterCouplings                                                     = 3527,
    /// NM-CLUSTERS
    NmClusters                                                             = 2979,
    /// NM-COM-CONTROL-ENABLED
    NmComControlEnabled                                                    = 3729,
    /// NM-CONFIG
    NmConfig                                                               = 5436,
    /// NM-CONTROL-BIT-VECTOR-ACTIVE
    NmControlBitVectorActive                                               = 5300,
    /// NM-CONTROL-BIT-VECTOR-ENABLED
    NmControlBitVectorEnabled                                              = 2237,
    /// NM-COORD-CLUSTER
    NmCoordCluster                                                         = 2635,
    /// NM-COORD-SYNC-SUPPORT
    NmCoordSyncSupport                                                     = 2421,
    /// NM-COORDINATOR
    NmCoordinator                                                          = 255,
    /// NM-COORDINATOR-ROLE
    NmCoordinatorRole                                                      = 1343,
    /// NM-CYCLETIME-MAIN-FUNCTION
    NmCycletimeMainFunction                                                = 1968,
    /// NM-DATA-CYCLE
    NmDataCycle                                                            = 371,
    /// NM-DATA-DISABLED
    NmDataDisabled                                                         = 1521,
    /// NM-DATA-ENABLED
    NmDataEnabled                                                          = 287,
    /// NM-DATA-INFORMATION
    NmDataInformation                                                      = 663,
    /// NM-DETECTION-LOCK
    NmDetectionLock                                                        = 569,
    /// NM-ECU
    NmEcu                                                                  = 1274,
    /// NM-GLOBAL-COORDINATOR-TIME
    NmGlobalCoordinatorTime                                                = 4144,
    /// NM-HANDLE-REF
    NmHandleRef                                                            = 4529,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 2612,
    /// NM-HW-VOTE-ENABLED
    NmHwVoteEnabled                                                        = 544,
    /// NM-IF-ECU-REF
    NmIfEcuRef                                                             = 3139,
    /// NM-IF-ECUS
    NmIfEcus                                                               = 5103,
    /// NM-IMMEDIATE-NM-CYCLE-TIME
    NmImmediateNmCycleTime                                                 = 3399,
    /// NM-IMMEDIATE-NM-TRANSMISSIONS
    NmImmediateNmTransmissions                                             = 3271,
    /// NM-IMMEDIATE-RESTART-ENABLED
    NmImmediateRestartEnabled                                              = 3256,
    /// NM-INSTANCE-ID
    NmInstanceId                                                           = 4539,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 5621,
    /// NM-MAIN-FUNCTION-ACROSS-FR-CYCLE
    NmMainFunctionAcrossFrCycle                                            = 1369,
    /// NM-MAIN-FUNCTION-PERIOD
    NmMainFunctionPeriod                                                   = 4118,
    /// NM-MESSAGE-TIMEOUT-TIME
    NmMessageTimeoutTime                                                   = 1543,
    /// NM-MSG-CYCLE-OFFSET
    NmMsgCycleOffset                                                       = 3678,
    /// NM-MSG-CYCLE-TIME
    NmMsgCycleTime                                                         = 208,
    /// NM-MSG-REDUCED-TIME
    NmMsgReducedTime                                                       = 438,
    /// NM-MULTIPLE-CHANNELS-ENABLED
    NmMultipleChannelsEnabled                                              = 2140,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 1439,
    /// NM-NETWORK-TIMEOUT
    NmNetworkTimeout                                                       = 4310,
    /// NM-NID-POSITION
    NmNidPosition                                                          = 2346,
    /// NM-NODE-DETECTION-ENABLED
    NmNodeDetectionEnabled                                                 = 2465,
    /// NM-NODE-ID
    NmNodeId                                                               = 3824,
    /// NM-NODE-ID-ENABLED
    NmNodeIdEnabled                                                        = 716,
    /// NM-NODE-REF
    NmNodeRef                                                              = 175,
    /// NM-NODE-REFS
    NmNodeRefs                                                             = 2239,
    /// NM-NODES
    NmNodes                                                                = 2447,
    /// NM-PASSIVE-MODE-ENABLED
    NmPassiveModeEnabled                                                   = 2424,
    /// NM-PDU
    NmPdu                                                                  = 650,
    /// NM-PDU-REF
    NmPduRef                                                               = 4745,
    /// NM-PDU-REF-CONDITIONAL
    NmPduRefConditional                                                    = 4992,
    /// NM-PDU-RX-INDICATION-ENABLED
    NmPduRxIndicationEnabled                                               = 4751,
    /// NM-PDUS
    NmPdus                                                                 = 3545,
    /// NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS
    NmPnHandleMultipleNetworkRequests                                      = 4003,
    /// NM-PNC-PARTICIPATION
    NmPncParticipation                                                     = 314,
    /// NM-RANGE-CONFIG
    NmRangeConfig                                                          = 3500,
    /// NM-READY-SLEEP-COUNT
    NmReadySleepCount                                                      = 4671,
    /// NM-READY-SLEEP-TIME
    NmReadySleepTime                                                       = 3118,
    /// NM-REMOTE-SLEEP-IND-ENABLED
    NmRemoteSleepIndEnabled                                                = 4870,
    /// NM-REMOTE-SLEEP-INDICATION-TIME
    NmRemoteSleepIndicationTime                                            = 1109,
    /// NM-REPEAT-MESSAGE-BIT-ACTIVE
    NmRepeatMessageBitActive                                               = 5588,
    /// NM-REPEAT-MESSAGE-BIT-ENABLE
    NmRepeatMessageBitEnable                                               = 3635,
    /// NM-REPEAT-MESSAGE-TIME
    NmRepeatMessageTime                                                    = 3501,
    /// NM-REPEAT-MSG-IND-ENABLED
    NmRepeatMsgIndEnabled                                                  = 5104,
    /// NM-REPEAT-MSG-INDICATION-ENABLED
    NmRepeatMsgIndicationEnabled                                           = 4552,
    /// NM-REPETITION-CYCLE
    NmRepetitionCycle                                                      = 4308,
    /// NM-SCHEDULE-VARIANT
    NmScheduleVariant                                                      = 3849,
    /// NM-SHUTDOWN-DELAY-TIMER
    NmShutdownDelayTimer                                                   = 5474,
    /// NM-STATE-CHANGE-IND-ENABLED
    NmStateChangeIndEnabled                                                = 2664,
    /// NM-SYNCHRONIZATION-POINT-ENABLED
    NmSynchronizationPointEnabled                                          = 3839,
    /// NM-SYNCHRONIZING-NETWORK
    NmSynchronizingNetwork                                                 = 3327,
    /// NM-USER-DATA-ENABLED
    NmUserDataEnabled                                                      = 4145,
    /// NM-USER-DATA-LENGTH
    NmUserDataLength                                                       = 4382,
    /// NM-USER-DATA-OFFSET
    NmUserDataOffset                                                       = 4149,
    /// NM-VECTOR-EARLY-UPDATE
    NmVectorEarlyUpdate                                                    = 1354,
    /// NM-VOTE-INFORMATION
    NmVoteInformation                                                      = 3490,
    /// NM-VOTING-CYCLE
    NmVotingCycle                                                          = 5030,
    /// NM-WAIT-BUS-SLEEP-TIME
    NmWaitBusSleepTime                                                     = 4439,
    /// NO-SUPERVISION
    NoSupervision                                                          = 4988,
    /// NO-SUPERVISIONS
    NoSupervisions                                                         = 965,
    /// NODE-NAME
    NodeName                                                               = 1755,
    /// NODE-PORTS
    NodePorts                                                              = 1566,
    /// NODE-REF
    NodeRef                                                                = 2115,
    /// NOMINAL
    Nominal                                                                = 4272,
    /// NOMINAL-EXECUTION-TIME
    NominalExecutionTime                                                   = 3132,
    /// NON-OS-MODULE-INSTANTIATION-REF
    NonOsModuleInstantiationRef                                            = 4849,
    /// NON-VERBOSE-MODE
    NonVerboseMode                                                         = 5167,
    /// NONQUEUED-RECEIVER-COM-SPEC
    NonqueuedReceiverComSpec                                               = 1674,
    /// NONQUEUED-SENDER-COM-SPEC
    NonqueuedSenderComSpec                                                 = 4702,
    /// NOT-AVAILABLE-VALUE-SPECIFICATION
    NotAvailableValueSpecification                                         = 2462,
    /// NOTATION
    Notation                                                               = 1393,
    /// NOTE
    Note                                                                   = 2459,
    /// NOTIFICATION-STATE
    NotificationState                                                      = 149,
    /// NOTIFIED-STATUSS
    NotifiedStatuss                                                        = 4575,
    /// NOTIFIER
    Notifier                                                               = 4348,
    /// NOTIFIER-DATA-ELEMENT-REF
    NotifierDataElementRef                                                 = 4499,
    /// NOTIFIER-REF
    NotifierRef                                                            = 4594,
    /// NOTIFIER-SIGNAL-TRIGGERING-REF
    NotifierSignalTriggeringRef                                            = 2736,
    /// NRC-VALUE
    NrcValue                                                               = 3732,
    /// NTU
    Ntu                                                                    = 3101,
    /// NUM-FAILED-SECURITY-ACCESS
    NumFailedSecurityAccess                                                = 5505,
    /// NUMBER
    Number                                                                 = 1189,
    /// NUMBER-OF-BITS
    NumberOfBits                                                           = 2773,
    /// NUMBER-OF-INSTRUCTIONS
    NumberOfInstructions                                                   = 2005,
    /// NUMBER-OF-MINISLOTS
    NumberOfMinislots                                                      = 2591,
    /// NUMBER-OF-NOTIFIER-SETS
    NumberOfNotifierSets                                                   = 1348,
    /// NUMBER-OF-REPETITIONS
    NumberOfRepetitions                                                    = 2165,
    /// NUMBER-OF-RESTART-ATTEMPTS
    NumberOfRestartAttempts                                                = 4475,
    /// NUMBER-OF-STATIC-SLOTS
    NumberOfStaticSlots                                                    = 4105,
    /// NUMBER-OF-WORKERS
    NumberOfWorkers                                                        = 2303,
    /// NUMERICAL-RULE-BASED-VALUE-SPECIFICATION
    NumericalRuleBasedValueSpecification                                   = 1492,
    /// NUMERICAL-VALUE
    NumericalValue                                                         = 1478,
    /// NUMERICAL-VALUE-SPECIFICATION
    NumericalValueSpecification                                            = 2883,
    /// NUMERICAL-VALUE-VARIATION-POINT
    NumericalValueVariationPoint                                           = 3671,
    /// NV-BLOCK-DATA-MAPPING
    NvBlockDataMapping                                                     = 3820,
    /// NV-BLOCK-DATA-MAPPINGS
    NvBlockDataMappings                                                    = 4278,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 2338,
    /// NV-BLOCK-DESCRIPTORS
    NvBlockDescriptors                                                     = 1073,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 2676,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 5119,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 3659,
    /// NV-DATA-PORT-ANNOTATION
    NvDataPortAnnotation                                                   = 2336,
    /// NV-DATA-PORT-ANNOTATIONS
    NvDataPortAnnotations                                                  = 5047,
    /// NV-DATAS
    NvDatas                                                                = 384,
    /// NV-PROVIDE-COM-SPEC
    NvProvideComSpec                                                       = 4538,
    /// NV-RAM-BLOCK-ELEMENT
    NvRamBlockElement                                                      = 3352,
    /// NV-REQUIRE-COM-SPEC
    NvRequireComSpec                                                       = 4830,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 5,
    /// OBD-DTC-NUMBER
    ObdDtcNumber                                                           = 4470,
    /// OBD-DTC-VALUE
    ObdDtcValue                                                            = 187,
    /// OBD-DTC-VALUE-3-BYTE
    ObdDtcValue3Byte                                                       = 838,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 1419,
    /// OBD-MID
    ObdMid                                                                 = 3314,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 5063,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 3928,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 1789,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 491,
    /// OBD-SUPPORT
    ObdSupport                                                             = 5255,
    /// OBJECT-DEFINITION-REF
    ObjectDefinitionRef                                                    = 4816,
    /// OBJECT-DEFINITION-REFS
    ObjectDefinitionRefs                                                   = 29,
    /// OBJECT-DEFINTION-REF
    ObjectDefintionRef                                                     = 4649,
    /// OBJECT-DEFINTION-REFS
    ObjectDefintionRefs                                                    = 1191,
    /// OBJECT-REF
    ObjectRef                                                              = 140,
    /// OBJECT-REFS
    ObjectRefs                                                             = 5460,
    /// OCCURRENCE-COUNTER-PROCESSING
    OccurrenceCounterProcessing                                            = 635,
    /// OCCURRENCE-EXPRESSION
    OccurrenceExpression                                                   = 789,
    /// OFFER-CYCLIC-DELAY
    OfferCyclicDelay                                                       = 1140,
    /// OFFSET
    Offset                                                                 = 3667,
    /// OFFSET-CORRECTION-ADAPTION-INTERVAL
    OffsetCorrectionAdaptionInterval                                       = 4366,
    /// OFFSET-CORRECTION-JUMP-THRESHOLD
    OffsetCorrectionJumpThreshold                                          = 326,
    /// OFFSET-CORRECTION-OUT
    OffsetCorrectionOut                                                    = 4142,
    /// OFFSET-CORRECTION-START
    OffsetCorrectionStart                                                  = 5233,
    /// OFFSET-SI-TO-UNIT
    OffsetSiToUnit                                                         = 4867,
    /// OFFSET-TIME-DOMAIN-REF
    OffsetTimeDomainRef                                                    = 3811,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 1556,
    /// OFFSETS
    Offsets                                                                = 5403,
    /// OFNS-DATA-ID-LIST
    OfnsDataIdList                                                         = 3848,
    /// OFNS-DATA-ID-LISTS
    OfnsDataIdLists                                                        = 2399,
    /// OFS-DATA-ID-LIST
    OfsDataIdList                                                          = 738,
    /// OFS-DATA-ID-LISTS
    OfsDataIdLists                                                         = 3466,
    /// OFS-SUB-TLV
    OfsSubTlv                                                              = 2255,
    /// ON-BOARD-MONITOR-ID
    OnBoardMonitorId                                                       = 1265,
    /// ON-TRANSITION-VALUE
    OnTransitionValue                                                      = 3168,
    /// ONE-EVERY-N
    OneEveryN                                                              = 4629,
    /// OP
    Op                                                                     = 4194,
    /// OPERATION-ARGUMENT-INSTANCE-IREF
    OperationArgumentInstanceIref                                          = 1064,
    /// OPERATION-CYCLE
    OperationCycle                                                         = 4660,
    /// OPERATION-CYCLE-AUTOMATIC-END
    OperationCycleAutomaticEnd                                             = 1921,
    /// OPERATION-CYCLE-AUTOSTART
    OperationCycleAutostart                                                = 1434,
    /// OPERATION-CYCLE-REF
    OperationCycleRef                                                      = 786,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 4980,
    /// OPERATION-IREF
    OperationIref                                                          = 58,
    /// OPERATION-IREFS
    OperationIrefs                                                         = 2097,
    /// OPERATION-MAPPINGS
    OperationMappings                                                      = 1339,
    /// OPERATION-MODE
    OperationMode                                                          = 4306,
    /// OPERATION-PROTOTYPE-IREF
    OperationPrototypeIref                                                 = 3689,
    /// OPERATION-REF
    OperationRef                                                           = 1562,
    /// OPERATIONS
    Operations                                                             = 1413,
    /// OPERATOR
    Operator                                                               = 2996,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF
    OptAppRecordElementInMethodIref                                        = 4598,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS
    OptAppRecordElementInMethodIrefs                                       = 333,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    OptAppRecordElementInServiceInterfaceIref                              = 404,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS
    OptAppRecordElementInServiceInterfaceIrefs                             = 5225,
    /// OPT-ARGUMENT-REF
    OptArgumentRef                                                         = 1398,
    /// OPT-ARGUMENT-REFS
    OptArgumentRefs                                                        = 481,
    /// OPT-IMPL-RECORD-ELEMENT
    OptImplRecordElement                                                   = 329,
    /// OPT-IMPL-RECORD-ELEMENTS
    OptImplRecordElements                                                  = 2298,
    /// OPTION
    Option                                                                 = 2865,
    /// OPTION-ARGUMENT
    OptionArgument                                                         = 2182,
    /// OPTION-KIND
    OptionKind                                                             = 1647,
    /// OPTION-NAME
    OptionName                                                             = 3472,
    /// OPTIONAL
    Optional                                                               = 135,
    /// OPTIONAL-ELEMENTS
    OptionalElements                                                       = 476,
    /// OPTIONS
    Options                                                                = 2199,
    /// ORDERED-ELEMENTS
    OrderedElements                                                        = 2983,
    /// ORDERED-MASTER
    OrderedMaster                                                          = 1614,
    /// ORDERED-MASTER-LIST
    OrderedMasterList                                                      = 4903,
    /// ORIGIN
    Origin                                                                 = 687,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 4704,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 3864,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 1941,
    /// OS-TASK-PROXY-REF
    OsTaskProxyRef                                                         = 854,
    /// OUT-MEASUREMENT-SET
    OutMeasurementSet                                                      = 4560,
    /// OUT-MEASURMENT-SET
    OutMeasurmentSet                                                       = 4071,
    /// OUTER-PORT-REF
    OuterPortRef                                                           = 4268,
    /// OUTER-PORTS
    OuterPorts                                                             = 3184,
    /// OUTGOING-CALLBACKS
    OutgoingCallbacks                                                      = 2524,
    /// P
    P                                                                      = 3921,
    /// P-2-MAX
    P2Max                                                                  = 1036,
    /// P-2-SERVER-MAX
    P2ServerMax                                                            = 610,
    /// P-2-STAR-SERVER-MAX
    P2StarServerMax                                                        = 1296,
    /// P-2-TIMING
    P2Timing                                                               = 1172,
    /// P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    PModeGroupInAtomicSwcInstanceRef                                       = 3858,
    /// P-PORT-IN-COMPOSITION-INSTANCE-REF
    PPortInCompositionInstanceRef                                          = 3087,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 74,
    /// P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    PPortPrototypeInExecutableIref                                         = 484,
    /// P-PORT-PROTOTYPE-IREF
    PPortPrototypeIref                                                     = 3409,
    /// P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF
    PTriggerInAtomicSwcTypeInstanceRef                                     = 2685,
    /// PACKAGE-REF
    PackageRef                                                             = 3563,
    /// PACKAGER-ID
    PackagerId                                                             = 2314,
    /// PACKAGER-SIGNATURE-REF
    PackagerSignatureRef                                                   = 4433,
    /// PACKING-BYTE-ORDER
    PackingByteOrder                                                       = 297,
    /// PADDING-ACTIVATION
    PaddingActivation                                                      = 952,
    /// PADDING-VALUE
    PaddingValue                                                           = 3038,
    /// PARAMETER
    Parameter                                                              = 2486,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 377,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REF
    ParameterAccessedForDebugRef                                           = 1502,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REFS
    ParameterAccessedForDebugRefs                                          = 3084,
    /// PARAMETER-ACCESSS
    ParameterAccesss                                                       = 2441,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 1169,
    /// PARAMETER-DATA-PROTOTYPE-IREF
    ParameterDataPrototypeIref                                             = 4141,
    /// PARAMETER-ID
    ParameterId                                                            = 1879,
    /// PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT
    ParameterImplementationDataTypeElement                                 = 4977,
    /// PARAMETER-INSTANCE
    ParameterInstance                                                      = 3918,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 1401,
    /// PARAMETER-POLICYS
    ParameterPolicys                                                       = 3103,
    /// PARAMETER-PORT-ANNOTATION
    ParameterPortAnnotation                                                = 924,
    /// PARAMETER-PORT-ANNOTATIONS
    ParameterPortAnnotations                                               = 1866,
    /// PARAMETER-PROVIDE-COM-SPEC
    ParameterProvideComSpec                                                = 524,
    /// PARAMETER-REF
    ParameterRef                                                           = 3430,
    /// PARAMETER-REQUIRE-COM-SPEC
    ParameterRequireComSpec                                                = 20,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 3846,
    /// PARAMETER-VALUES
    ParameterValues                                                        = 2617,
    /// PARAMETERS
    Parameters                                                             = 964,
    /// PARENT-CATEGORY
    ParentCategory                                                         = 914,
    /// PARENT-SHORT-LABEL
    ParentShortLabel                                                       = 50,
    /// PARTIAL-NETWORK-REF
    PartialNetworkRef                                                      = 4825,
    /// PARTIAL-NETWORK-REFS
    PartialNetworkRefs                                                     = 2963,
    /// PARTICIPATING-J-1939-CLUSTER-REF
    ParticipatingJ1939ClusterRef                                           = 4562,
    /// PARTICIPATING-J-1939-CLUSTER-REFS
    ParticipatingJ1939ClusterRefs                                          = 253,
    /// PARTITION-REF
    PartitionRef                                                           = 1511,
    /// PARTITIONS
    Partitions                                                             = 4738,
    /// PARTS
    Parts                                                                  = 889,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 5456,
    /// PATH
    Path                                                                   = 3010,
    /// PATH-MTU-DISCOVERY-ENABLED
    PathMtuDiscoveryEnabled                                                = 2385,
    /// PATH-MTU-ENABLED
    PathMtuEnabled                                                         = 3222,
    /// PATH-MTU-TIMEOUT
    PathMtuTimeout                                                         = 1830,
    /// PATTERN
    Pattern                                                                = 4999,
    /// PATTERN-JITTER
    PatternJitter                                                          = 1200,
    /// PATTERN-LENGTH
    PatternLength                                                          = 4645,
    /// PATTERN-PERIOD
    PatternPeriod                                                          = 5654,
    /// PAYLOAD-LENGTH-STATIC
    PayloadLengthStatic                                                    = 3580,
    /// PAYLOAD-PREAMBLE-INDICATOR
    PayloadPreambleIndicator                                               = 158,
    /// PAYLOAD-REF
    PayloadRef                                                             = 1207,
    /// PDELAY-LATENCY-THRESHOLD
    PdelayLatencyThreshold                                                 = 4672,
    /// PDELAY-REQUEST-PERIOD
    PdelayRequestPeriod                                                    = 365,
    /// PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT
    PdelayRespAndRespFollowUpTimeout                                       = 2516,
    /// PDELAY-RESPONSE-ENABLED
    PdelayResponseEnabled                                                  = 5346,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 741,
    /// PDU-ACTIVATION-ROUTING-GROUPS
    PduActivationRoutingGroups                                             = 5473,
    /// PDU-COLLECTION-MAX-BUFFER-SIZE
    PduCollectionMaxBufferSize                                             = 2666,
    /// PDU-COLLECTION-PDU-TIMEOUT
    PduCollectionPduTimeout                                                = 5380,
    /// PDU-COLLECTION-SEMANTICS
    PduCollectionSemantics                                                 = 3258,
    /// PDU-COLLECTION-TIMEOUT
    PduCollectionTimeout                                                   = 1177,
    /// PDU-COLLECTION-TRIGGER
    PduCollectionTrigger                                                   = 5418,
    /// PDU-COUNTER-SIZE
    PduCounterSize                                                         = 704,
    /// PDU-COUNTER-START-POSITION
    PduCounterStartPosition                                                = 3214,
    /// PDU-COUNTER-THRESHOLD
    PduCounterThreshold                                                    = 1750,
    /// PDU-COUNTERS
    PduCounters                                                            = 1162,
    /// PDU-MAX-LENGTH
    PduMaxLength                                                           = 3557,
    /// PDU-POOLS
    PduPools                                                               = 1923,
    /// PDU-REF
    PduRef                                                                 = 3048,
    /// PDU-REPLICATION-VOTING
    PduReplicationVoting                                                   = 1534,
    /// PDU-REPLICATIONS
    PduReplications                                                        = 3673,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 5153,
    /// PDU-TO-FRAME-MAPPINGS
    PduToFrameMappings                                                     = 2233,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 1980,
    /// PDU-TRIGGERING-REF
    PduTriggeringRef                                                       = 2141,
    /// PDU-TRIGGERING-REF-CONDITIONAL
    PduTriggeringRefConditional                                            = 4937,
    /// PDU-TRIGGERING-REFS
    PduTriggeringRefs                                                      = 4677,
    /// PDU-TRIGGERINGS
    PduTriggerings                                                         = 3486,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 3049,
    /// PDUR-TP-CHUNK-SIZE
    PdurTpChunkSize                                                        = 947,
    /// PDUS
    Pdus                                                                   = 3292,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 2661,
    /// PER-INSTANCE-MEMORY-REF
    PerInstanceMemoryRef                                                   = 4923,
    /// PER-INSTANCE-MEMORY-SIZE
    PerInstanceMemorySize                                                  = 5189,
    /// PER-INSTANCE-MEMORY-SIZES
    PerInstanceMemorySizes                                                 = 2316,
    /// PER-INSTANCE-MEMORYS
    PerInstanceMemorys                                                     = 1088,
    /// PER-INSTANCE-PARAMETER-REF
    PerInstanceParameterRef                                                = 2045,
    /// PER-INSTANCE-PARAMETERS
    PerInstanceParameters                                                  = 5044,
    /// PER-STATE-TIMEOUT
    PerStateTimeout                                                        = 3071,
    /// PER-STATE-TIMEOUTS
    PerStateTimeouts                                                       = 644,
    /// PERIOD
    Period                                                                 = 295,
    /// PERIOD-BEGIN
    PeriodBegin                                                            = 5422,
    /// PERIOD-END
    PeriodEnd                                                              = 745,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 2553,
    /// PERIODIC-RATE-CATEGORY
    PeriodicRateCategory                                                   = 3065,
    /// PERIODIC-RATES
    PeriodicRates                                                          = 5616,
    /// PERIODIC-RESPONSE-UUDT-REF
    PeriodicResponseUudtRef                                                = 1662,
    /// PERIODIC-RESPONSE-UUDT-REFS
    PeriodicResponseUudtRefs                                               = 5186,
    /// PERMISSIBLE-SIGNAL-PATH
    PermissibleSignalPath                                                  = 4678,
    /// PERMISSION-CERTIFICATE-AUTHORITY-REF
    PermissionCertificateAuthorityRef                                      = 3225,
    /// PERMIT-MULTIPLE-REFERENCES-TO-EE
    PermitMultipleReferencesToEe                                           = 2405,
    /// PERSISTENCY-CENTRAL-STORAGE-URI
    PersistencyCentralStorageUri                                           = 5514,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 2277,
    /// PERSISTENCY-DATA-PROVIDED-COM-SPEC
    PersistencyDataProvidedComSpec                                         = 1625,
    /// PERSISTENCY-DATA-REQUIRED-COM-SPEC
    PersistencyDataRequiredComSpec                                         = 2297,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-REF
    PersistencyDeploymentElementRef                                        = 220,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 5292,
    /// PERSISTENCY-DEPLOYMENT-REF
    PersistencyDeploymentRef                                               = 138,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 3029,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 5625,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 1619,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 3725,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 4500,
    /// PERSISTENCY-FILE-ARRAY-REF
    PersistencyFileArrayRef                                                = 5127,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 1308,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 4741,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 3014,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 3453,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 139,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 4827,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 3767,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 5627,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 82,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 2521,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 4524,
    /// PERSISTENCY-PORT-PROTOTYPE-IREF
    PersistencyPortPrototypeIref                                           = 151,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 861,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 1592,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 4973,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 1774,
    /// PERSISTENCY-PROVIDED-COM-SPEC
    PersistencyProvidedComSpec                                             = 2482,
    /// PERSISTENCY-REDUNDANCY-CRC
    PersistencyRedundancyCrc                                               = 4429,
    /// PERSISTENCY-REDUNDANCY-HASH
    PersistencyRedundancyHash                                              = 906,
    /// PERSISTENCY-REDUNDANCY-M-OUT-OF-N
    PersistencyRedundancyMOutOfN                                           = 5599,
    /// PERSISTENT-STORAGE
    PersistentStorage                                                      = 1199,
    /// PGN
    Pgn                                                                    = 1937,
    /// PHM-ACTION
    PhmAction                                                              = 1509,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 3924,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 5102,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 751,
    /// PHM-CHECKPOINT-IREF
    PhmCheckpointIref                                                      = 3487,
    /// PHM-CONTRIBUTION-REF
    PhmContributionRef                                                     = 1685,
    /// PHM-CONTRIBUTION-REFS
    PhmContributionRefs                                                    = 3984,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 473,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 673,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 4291,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 3102,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 2098,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 3713,
    /// PHM-RULE
    PhmRule                                                                = 1736,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 73,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 3693,
    /// PHYS-CONSTRS
    PhysConstrs                                                            = 2069,
    /// PHYSICAL-CHANNEL-REF
    PhysicalChannelRef                                                     = 4502,
    /// PHYSICAL-CHANNEL-REF-CONDITIONAL
    PhysicalChannelRefConditional                                          = 2497,
    /// PHYSICAL-CHANNEL-REFS
    PhysicalChannelRefs                                                    = 1128,
    /// PHYSICAL-CHANNELS
    PhysicalChannels                                                       = 1671,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 746,
    /// PHYSICAL-DIMENSION-MAPPING
    PhysicalDimensionMapping                                               = 5267,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 4953,
    /// PHYSICAL-DIMENSION-MAPPINGS
    PhysicalDimensionMappings                                              = 3817,
    /// PHYSICAL-DIMENSION-REF
    PhysicalDimensionRef                                                   = 884,
    /// PHYSICAL-LAYER-TYPE
    PhysicalLayerType                                                      = 4577,
    /// PHYSICAL-PROPS
    PhysicalProps                                                          = 1239,
    /// PHYSICAL-REQUEST-REF
    PhysicalRequestRef                                                     = 3567,
    /// PID
    Pid                                                                    = 1067,
    /// PID-REF
    PidRef                                                                 = 5598,
    /// PID-REFS
    PidRefs                                                                = 5388,
    /// PID-SIZE
    PidSize                                                                = 296,
    /// PIN-NUMBER
    PinNumber                                                              = 1574,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 3683,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 1616,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 3496,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 3008,
    /// PLCA-LOCAL-NODE-COUNT
    PlcaLocalNodeCount                                                     = 3954,
    /// PLCA-LOCAL-NODE-ID
    PlcaLocalNodeId                                                        = 4831,
    /// PLCA-MAX-BURST-COUNT
    PlcaMaxBurstCount                                                      = 4697,
    /// PLCA-MAX-BURST-TIMER
    PlcaMaxBurstTimer                                                      = 2094,
    /// PLCA-PROPS
    PlcaProps                                                              = 3566,
    /// PLCA-TRANSMIT-OPPORTUNITY-TIMER
    PlcaTransmitOpportunityTimer                                           = 316,
    /// PN-RESET-TIME
    PnResetTime                                                            = 1157,
    /// PN-RESET-TIMER
    PnResetTimer                                                           = 1599,
    /// PNC-CLUSTER-VECTOR-LENGTH
    PncClusterVectorLength                                                 = 2708,
    /// PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    PncConsumedProvidedServiceInstanceGroups                               = 3414,
    /// PNC-FILTER-ARRAY-MASK
    PncFilterArrayMask                                                     = 47,
    /// PNC-FILTER-ARRAY-MASKS
    PncFilterArrayMasks                                                    = 1637,
    /// PNC-FILTER-DATA-MASK
    PncFilterDataMask                                                      = 1345,
    /// PNC-GATEWAY-TYPE
    PncGatewayType                                                         = 520,
    /// PNC-GROUP-REF
    PncGroupRef                                                            = 4002,
    /// PNC-GROUP-REFS
    PncGroupRefs                                                           = 2242,
    /// PNC-IDENTIFIER
    PncIdentifier                                                          = 3703,
    /// PNC-MAPPING
    PncMapping                                                             = 4046,
    /// PNC-MAPPING-REF
    PncMappingRef                                                          = 331,
    /// PNC-MAPPING-REFS
    PncMappingRefs                                                         = 5336,
    /// PNC-MAPPINGS
    PncMappings                                                            = 1982,
    /// PNC-PDUR-GROUP-REF
    PncPdurGroupRef                                                        = 3183,
    /// PNC-PDUR-GROUP-REFS
    PncPdurGroupRefs                                                       = 1876,
    /// PNC-PREPARE-SLEEP-TIMER
    PncPrepareSleepTimer                                                   = 5003,
    /// PNC-SYNCHRONOUS-WAKEUP
    PncSynchronousWakeup                                                   = 3773,
    /// PNC-VECTOR-LENGTH
    PncVectorLength                                                        = 3085,
    /// PNC-VECTOR-OFFSET
    PncVectorOffset                                                        = 554,
    /// PNC-WAKEUP-CAN-ID
    PncWakeupCanId                                                         = 4930,
    /// PNC-WAKEUP-CAN-ID-EXTENDED
    PncWakeupCanIdExtended                                                 = 3390,
    /// PNC-WAKEUP-CAN-ID-MASK
    PncWakeupCanIdMask                                                     = 2142,
    /// PNC-WAKEUP-DATA-MASK
    PncWakeupDataMask                                                      = 5293,
    /// PNC-WAKEUP-DLC
    PncWakeupDlc                                                           = 1821,
    /// PNC-WAKEUP-ENABLE
    PncWakeupEnable                                                        = 3095,
    /// POLICY
    Policy                                                                 = 1533,
    /// POLICY-ACTION
    PolicyAction                                                           = 4252,
    /// PORT-ADDRESS
    PortAddress                                                            = 5542,
    /// PORT-API-OPTION
    PortApiOption                                                          = 1247,
    /// PORT-API-OPTIONS
    PortApiOptions                                                         = 454,
    /// PORT-ARG-VALUES
    PortArgValues                                                          = 3392,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 1061,
    /// PORT-BLUEPRINT-INIT-VALUE
    PortBlueprintInitValue                                                 = 3393,
    /// PORT-BLUEPRINT-MAPPING
    PortBlueprintMapping                                                   = 1142,
    /// PORT-BLUEPRINT-MAPPINGS
    PortBlueprintMappings                                                  = 3951,
    /// PORT-BLUEPRINT-REF
    PortBlueprintRef                                                       = 3464,
    /// PORT-CONFIGS
    PortConfigs                                                            = 630,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINT
    PortDefinedArgumentBlueprint                                           = 5424,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINTS
    PortDefinedArgumentBlueprints                                          = 2152,
    /// PORT-DEFINED-ARGUMENT-VALUE
    PortDefinedArgumentValue                                               = 5482,
    /// PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS
    PortElementToComResourceMappings                                       = 2624,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 4281,
    /// PORT-GROUP
    PortGroup                                                              = 1610,
    /// PORT-GROUPS
    PortGroups                                                             = 71,
    /// PORT-INSTANTIATION-BEHAVIOR
    PortInstantiationBehavior                                              = 4221,
    /// PORT-INTERFACE-BLUEPRINT-MAPPING
    PortInterfaceBlueprintMapping                                          = 106,
    /// PORT-INTERFACE-BLUEPRINT-REF
    PortInterfaceBlueprintRef                                              = 5083,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 2573,
    /// PORT-INTERFACE-MAPPINGS
    PortInterfaceMappings                                                  = 4608,
    /// PORT-INTERFACE-REF
    PortInterfaceRef                                                       = 2530,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 2679,
    /// PORT-NUMBER
    PortNumber                                                             = 1178,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 4986,
    /// PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE
    PortPrototypeBlueprintInitValue                                        = 810,
    /// PORT-PROTOTYPE-BLUEPRINT-MAPPING
    PortPrototypeBlueprintMapping                                          = 4373,
    /// PORT-PROTOTYPE-BLUEPRINT-REF
    PortPrototypeBlueprintRef                                              = 4435,
    /// PORT-PROTOTYPE-IREF
    PortPrototypeIref                                                      = 4693,
    /// PORT-PROTOTYPE-PROPS
    PortPrototypeProps                                                     = 3483,
    /// PORT-PROTOTYPE-REF
    PortPrototypeRef                                                       = 4789,
    /// PORT-PROTOTYPE-REF-CONDITIONAL
    PortPrototypeRefConditional                                            = 1408,
    /// PORT-PROTOTYPE-SLUG-FRAGMENT
    PortPrototypeSlugFragment                                              = 4083,
    /// PORT-REF
    PortRef                                                                = 2082,
    /// PORT-SCHEDULER
    PortScheduler                                                          = 4058,
    /// PORTS
    Ports                                                                  = 1779,
    /// POSITION
    Position                                                               = 3280,
    /// POSITION-IN-TABLE
    PositionInTable                                                        = 4392,
    /// POSITIVE-INTEGER-VALUE-VARIATION-POINT
    PositiveIntegerValueVariationPoint                                     = 4183,
    /// POSSIBLE-AP-ERROR-REF
    PossibleApErrorRef                                                     = 2435,
    /// POSSIBLE-AP-ERROR-REFS
    PossibleApErrorRefs                                                    = 4020,
    /// POSSIBLE-AP-ERROR-SET-REF
    PossibleApErrorSetRef                                                  = 3874,
    /// POSSIBLE-AP-ERROR-SET-REFS
    PossibleApErrorSetRefs                                                 = 4709,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 4940,
    /// POSSIBLE-ERROR-REACTIONS
    PossibleErrorReactions                                                 = 3905,
    /// POSSIBLE-ERROR-REF
    PossibleErrorRef                                                       = 3637,
    /// POSSIBLE-ERROR-REFS
    PossibleErrorRefs                                                      = 19,
    /// POSSIBLE-ERRORS
    PossibleErrors                                                         = 5086,
    /// POST-BUILD-CHANGEABLE
    PostBuildChangeable                                                    = 3510,
    /// POST-BUILD-VALUE-ACCESS-REF
    PostBuildValueAccessRef                                                = 265,
    /// POST-BUILD-VARIANT-CONDITION
    PostBuildVariantCondition                                              = 3980,
    /// POST-BUILD-VARIANT-CONDITIONS
    PostBuildVariantConditions                                             = 2525,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 1437,
    /// POST-BUILD-VARIANT-CRITERION-VALUE
    PostBuildVariantCriterionValue                                         = 3884,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 344,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF
    PostBuildVariantCriterionValueSetRef                                   = 1538,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS
    PostBuildVariantCriterionValueSetRefs                                  = 4974,
    /// POST-BUILD-VARIANT-CRITERION-VALUES
    PostBuildVariantCriterionValues                                        = 5301,
    /// POST-BUILD-VARIANT-MULTIPLICITY
    PostBuildVariantMultiplicity                                           = 5536,
    /// POST-BUILD-VARIANT-SUPPORT
    PostBuildVariantSupport                                                = 1399,
    /// POST-BUILD-VARIANT-USED
    PostBuildVariantUsed                                                   = 4701,
    /// POST-BUILD-VARIANT-VALUE
    PostBuildVariantValue                                                  = 1170,
    /// POST-VERIFICATION-REBOOT
    PostVerificationReboot                                                 = 2125,
    /// POWER-DOWN-TIME
    PowerDownTime                                                          = 2039,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 1397,
    /// PRE-ACTIVATE-IREF
    PreActivateIref                                                        = 2841,
    /// PRE-ACTIVATE-IREFS
    PreActivateIrefs                                                       = 4154,
    /// PRE-ACTIVATE-REF
    PreActivateRef                                                         = 2136,
    /// PRE-ACTIVATE-REFS
    PreActivateRefs                                                        = 247,
    /// PRE-ACTIVATION-REBOOT
    PreActivationReboot                                                    = 387,
    /// PRE-MAPPING
    PreMapping                                                             = 273,
    /// PRE-SHARED-KEY-REF
    PreSharedKeyRef                                                        = 768,
    /// PRECONFIGURED-CONFIGURATION-REF
    PreconfiguredConfigurationRef                                          = 26,
    /// PRECONFIGURED-CONFIGURATION-REFS
    PreconfiguredConfigurationRefs                                         = 2846,
    /// PREDECESSOR-ACTION-REF
    PredecessorActionRef                                                   = 2641,
    /// PREDECESSOR-ACTION-REFS
    PredecessorActionRefs                                                  = 5268,
    /// PREDECESSOR-FIFO-REF
    PredecessorFifoRef                                                     = 3949,
    /// PREDECESSOR-REF
    PredecessorRef                                                         = 2450,
    /// PREDECESSOR-REFS
    PredecessorRefs                                                        = 3189,
    /// PREDEFINED-TEXT
    PredefinedText                                                         = 4427,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 2344,
    /// PREEMPTABILITY
    Preemptability                                                         = 2655,
    /// PREFIX
    Prefix                                                                 = 3519,
    /// PREFIX-REF
    PrefixRef                                                              = 2900,
    /// PRESTORAGE-FREEZE-FRAME
    PrestorageFreezeFrame                                                  = 2120,
    /// PRESTORED-FREEZEFRAME-STORED-IN-NVM
    PrestoredFreezeframeStoredInNvm                                        = 5657,
    /// PRIMITIVE
    Primitive                                                              = 3047,
    /// PRIMITIVE-ATTRIBUTE-CONDITION
    PrimitiveAttributeCondition                                            = 3718,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 1999,
    /// PRIMITIVE-FAMILY
    PrimitiveFamily                                                        = 4322,
    /// PRIMITIVE-TYPE-MAPPINGS
    PrimitiveTypeMappings                                                  = 2683,
    /// PRIORITY
    Priority                                                               = 3639,
    /// PRIORITYS
    Prioritys                                                              = 2990,
    /// PRM
    Prm                                                                    = 4713,
    /// PRM-CHAR
    PrmChar                                                                = 5082,
    /// PRM-UNIT
    PrmUnit                                                                = 1766,
    /// PRMS
    Prms                                                                   = 3041,
    /// PROCESS
    Process                                                                = 4767,
    /// PROCESS-ARGUMENT
    ProcessArgument                                                        = 1290,
    /// PROCESS-ARGUMENTS
    ProcessArguments                                                       = 2333,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 2422,
    /// PROCESS-DESIGN-REF
    ProcessDesignRef                                                       = 1082,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 283,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 3825,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS
    ProcessDesignToMachineDesignMappings                                   = 1435,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 1718,
    /// PROCESS-MODE-IREF
    ProcessModeIref                                                        = 5308,
    /// PROCESS-MODE-MACHINE
    ProcessModeMachine                                                     = 1352,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 1634,
    /// PROCESS-REF
    ProcessRef                                                             = 5589,
    /// PROCESS-STATE-IREF
    ProcessStateIref                                                       = 375,
    /// PROCESS-STATE-MACHINE
    ProcessStateMachine                                                    = 4685,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 1031,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 4927,
    /// PROCESS-TO-MACHINE-MAPPINGS
    ProcessToMachineMappings                                               = 5534,
    /// PROCESSING-KIND
    ProcessingKind                                                         = 2291,
    /// PROCESSING-STYLE
    ProcessingStyle                                                        = 778,
    /// PROCESSING-UNIT-REF
    ProcessingUnitRef                                                      = 1843,
    /// PROCESSOR
    Processor                                                              = 1420,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 5227,
    /// PROCESSOR-MODE
    ProcessorMode                                                          = 775,
    /// PROCESSOR-SPEED
    ProcessorSpeed                                                         = 3632,
    /// PROCESSORS
    Processors                                                             = 4006,
    /// PRODUCT-RELEASE
    ProductRelease                                                         = 4868,
    /// PROFILE-BEHAVIOR
    ProfileBehavior                                                        = 5304,
    /// PROFILE-NAME
    ProfileName                                                            = 555,
    /// PROG-CODE
    ProgCode                                                               = 4237,
    /// PROGRAMMING-LANGUAGE
    ProgrammingLanguage                                                    = 2408,
    /// PROP-SEG
    PropSeg                                                                = 2855,
    /// PROPABILITY
    Propability                                                            = 5340,
    /// PROPAGATION-DELAY
    PropagationDelay                                                       = 3401,
    /// PROPERTYS
    Propertys                                                              = 4884,
    /// PROPS
    Props                                                                  = 2493,
    /// PROTOCOL
    Protocol                                                               = 4894,
    /// PROTOCOL-KIND
    ProtocolKind                                                           = 3204,
    /// PROTOCOL-NAME
    ProtocolName                                                           = 2467,
    /// PROTOCOL-VERSION
    ProtocolVersion                                                        = 3759,
    /// PROVIDE-RESOURCES
    ProvideResources                                                       = 3053,
    /// PROVIDED-CLIENT-SERVER-ENTRYS
    ProvidedClientServerEntrys                                             = 4248,
    /// PROVIDED-COM-SPECS
    ProvidedComSpecs                                                       = 4273,
    /// PROVIDED-DATA-REF
    ProvidedDataRef                                                        = 4900,
    /// PROVIDED-DATAS
    ProvidedDatas                                                          = 900,
    /// PROVIDED-DDS-EVENT-QOS-PROPS
    ProvidedDdsEventQosProps                                               = 263,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 648,
    /// PROVIDED-ENTRYS
    ProvidedEntrys                                                         = 1407,
    /// PROVIDED-EVENT-GROUPS
    ProvidedEventGroups                                                    = 955,
    /// PROVIDED-INTERFACE-TREF
    ProvidedInterfaceTref                                                  = 5058,
    /// PROVIDED-MEMORY-REF
    ProvidedMemoryRef                                                      = 963,
    /// PROVIDED-MODE-GROUP-REF
    ProvidedModeGroupRef                                                   = 4175,
    /// PROVIDED-MODE-GROUPS
    ProvidedModeGroups                                                     = 654,
    /// PROVIDED-OUTER-PORT-REF
    ProvidedOuterPortRef                                                   = 5145,
    /// PROVIDED-PORT-PROTOTYPE-IREF
    ProvidedPortPrototypeIref                                              = 1684,
    /// PROVIDED-REQUIRED-INTERFACE-TREF
    ProvidedRequiredInterfaceTref                                          = 4869,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 4220,
    /// PROVIDED-SERVICE-INSTANCE-REF
    ProvidedServiceInstanceRef                                             = 153,
    /// PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL
    ProvidedServiceInstanceRefConditional                                  = 334,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 537,
    /// PROVIDED-SERVICE-INSTANCES
    ProvidedServiceInstances                                               = 1621,
    /// PROVIDED-SERVICE-PORT-IREF
    ProvidedServicePortIref                                                = 4103,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 2106,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 5584,
    /// PROVIDER-IREF
    ProviderIref                                                           = 2998,
    /// PROVIDER-REF
    ProviderRef                                                            = 448,
    /// PROVIDER-SOFTWARE-CLUSTER-REF
    ProviderSoftwareClusterRef                                             = 4732,
    /// PROVIEDE-DATA-REF
    ProviedeDataRef                                                        = 1114,
    /// PSK-IDENTITY
    PskIdentity                                                            = 1512,
    /// PSK-IDENTITY-HINT
    PskIdentityHint                                                        = 1568,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 5196,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS
    PskIdentityToKeySlotMappings                                           = 5065,
    /// PUBLISHER
    Publisher                                                              = 1392,
    /// PULSE-TEST
    PulseTest                                                              = 5193,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 2619,
    /// PURPOSE-OF-UPDATE-REF
    PurposeOfUpdateRef                                                     = 179,
    /// QOS-PROFILE
    QosProfile                                                             = 396,
    /// QUEUE-LENGTH
    QueueLength                                                            = 5477,
    /// QUEUE-SIZE
    QueueSize                                                              = 3967,
    /// QUEUED-RECEIVER-COM-SPEC
    QueuedReceiverComSpec                                                  = 400,
    /// QUEUED-SENDER-COM-SPEC
    QueuedSenderComSpec                                                    = 2083,
    /// R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    RModeGroupInAtomicSwcInstanceRef                                       = 546,
    /// R-PORT-IN-COMPOSITION-INSTANCE-REF
    RPortInCompositionInstanceRef                                          = 4389,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 5372,
    /// R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    RPortPrototypeInExecutableIref                                         = 4568,
    /// R-PORT-PROTOTYPE-IREF
    RPortPrototypeIref                                                     = 4690,
    /// R-PORT-PROTOTYPE-PROPS
    RPortPrototypeProps                                                    = 178,
    /// R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF
    RTriggerInAtomicSwcInstanceRef                                         = 3002,
    /// RAM-BLOCK
    RamBlock                                                               = 2952,
    /// RAM-BLOCK-INIT-VALUE
    RamBlockInitValue                                                      = 3194,
    /// RAM-BLOCK-STATUS-CONTROL
    RamBlockStatusControl                                                  = 1727,
    /// RAM-LOCATION-REF
    RamLocationRef                                                         = 1656,
    /// RANGE-MAX
    RangeMax                                                               = 868,
    /// RANGE-MIN
    RangeMin                                                               = 4944,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 1359,
    /// RATE-BASED-MONITORED-EVENT-REF
    RateBasedMonitoredEventRef                                             = 647,
    /// RATE-CORRECTION-MEASUREMENT-DURATION
    RateCorrectionMeasurementDuration                                      = 4786,
    /// RATE-CORRECTION-OUT
    RateCorrectionOut                                                      = 1217,
    /// RATE-CORRECTIONS-PER-MEASUREMENT-DURATION
    RateCorrectionsPerMeasurementDuration                                  = 3059,
    /// RATE-DEVIATION-MEASUREMENT-DURATION
    RateDeviationMeasurementDuration                                       = 2784,
    /// RATE-LIMITATION-FILTERS
    RateLimitationFilters                                                  = 2798,
    /// RATE-POLICYS
    RatePolicys                                                            = 445,
    /// RATIO-KIND
    RatioKind                                                              = 853,
    /// RATIONALE
    Rationale                                                              = 3289,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 1383,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 3883,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 2176,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 379,
    /// RAW-DATA-STREAM-INTERFACE-REF
    RawDataStreamInterfaceRef                                              = 1947,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 4536,
    /// RAW-DATA-STREAM-REF
    RawDataStreamRef                                                       = 5090,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 1385,
    /// REACTION-CODE
    ReactionCode                                                           = 640,
    /// READ
    Read                                                                   = 4675,
    /// READ-CLASS-REF
    ReadClassRef                                                           = 1160,
    /// READ-DATA-CLASS-REF
    ReadDataClassRef                                                       = 907,
    /// READ-DTC-INFORMATION-CLASS-REF
    ReadDtcInformationClassRef                                             = 2609,
    /// READ-LOCAL-VARIABLES
    ReadLocalVariables                                                     = 4573,
    /// READ-NV-DATA
    ReadNvData                                                             = 1498,
    /// READ-SCALING-DATA-CLASS-REF
    ReadScalingDataClassRef                                                = 4232,
    /// READONLY
    Readonly                                                               = 357,
    /// REASON
    Reason                                                                 = 4926,
    /// RECEIVE-ACTIVITY
    ReceiveActivity                                                        = 3606,
    /// RECEIVE-WINDOW-MIN
    ReceiveWindowMin                                                       = 11,
    /// RECEIVED-DATA-REF
    ReceivedDataRef                                                        = 279,
    /// RECEIVER-ANNOTATION
    ReceiverAnnotation                                                     = 1925,
    /// RECEIVER-CAPABILITY
    ReceiverCapability                                                     = 3206,
    /// RECEIVER-INTENT
    ReceiverIntent                                                         = 4201,
    /// RECEIVER-IREF
    ReceiverIref                                                           = 394,
    /// RECEIVER-IREFS
    ReceiverIrefs                                                          = 4152,
    /// RECEIVER-REF
    ReceiverRef                                                            = 3730,
    /// RECEIVER-REFS
    ReceiverRefs                                                           = 3244,
    /// RECEPTION-POLICYS
    ReceptionPolicys                                                       = 4652,
    /// RECEPTION-PROPS
    ReceptionProps                                                         = 2816,
    /// RECOMMENDED-CONFIGURATION-REF
    RecommendedConfigurationRef                                            = 1276,
    /// RECOMMENDED-CONFIGURATION-REFS
    RecommendedConfigurationRefs                                           = 829,
    /// RECORD-ELEMENT-MAPPINGS
    RecordElementMappings                                                  = 3978,
    /// RECORD-ELEMENT-REF
    RecordElementRef                                                       = 2904,
    /// RECORD-ELEMENTS
    RecordElements                                                         = 2073,
    /// RECORD-NUMBER
    RecordNumber                                                           = 1768,
    /// RECORD-VALUE-SPECIFICATION
    RecordValueSpecification                                               = 623,
    /// RECOVERABLE-IN-SAME-OPERATION-CYCLE
    RecoverableInSameOperationCycle                                        = 109,
    /// RECOVERY
    Recovery                                                               = 4898,
    /// RECOVERY-ACTION-IREF
    RecoveryActionIref                                                     = 3494,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 4498,
    /// RECOVERY-NOTIFICATION-REF
    RecoveryNotificationRef                                                = 613,
    /// RECOVERY-NOTIFICATION-RETRY
    RecoveryNotificationRetry                                              = 5076,
    /// RECOVERY-NOTIFICATION-TIMEOUT
    RecoveryNotificationTimeout                                            = 4173,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 5439,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 459,
    /// RECOVERY-VIA-APPLICATION-ACTION-REF
    RecoveryViaApplicationActionRef                                        = 5297,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 1341,
    /// REDUNDANCY
    Redundancy                                                             = 5658,
    /// REDUNDANCY-HANDLINGS
    RedundancyHandlings                                                    = 632,
    /// REENTRANCY-LEVEL
    ReentrancyLevel                                                        = 1910,
    /// REF
    Ref                                                                    = 4469,
    /// REF-CALPRM-SET
    RefCalprmSet                                                           = 2581,
    /// REF-MEASUREMENT-SET
    RefMeasurementSet                                                      = 2411,
    /// REFERENCE-BASE
    ReferenceBase                                                          = 4909,
    /// REFERENCE-BASES
    ReferenceBases                                                         = 4155,
    /// REFERENCE-CONDITION
    ReferenceCondition                                                     = 5382,
    /// REFERENCE-REF
    ReferenceRef                                                           = 808,
    /// REFERENCE-TABLE-REF
    ReferenceTableRef                                                      = 1954,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 3842,
    /// REFERENCE-VALUE-REF
    ReferenceValueRef                                                      = 531,
    /// REFERENCE-VALUE-SPECIFICATION
    ReferenceValueSpecification                                            = 4799,
    /// REFERENCE-VALUES
    ReferenceValues                                                        = 2909,
    /// REFERENCED-BASELINE
    ReferencedBaseline                                                     = 5001,
    /// REFERENCED-TD-EVENT-SWC-REF
    ReferencedTdEventSwcRef                                                = 1914,
    /// REFERENCED-TD-EVENT-VFB-REF
    ReferencedTdEventVfbRef                                                = 395,
    /// REFERENCES
    References                                                             = 4902,
    /// REFERRABLE-REF
    ReferrableRef                                                          = 4440,
    /// REFINED-EVENT-IREF
    RefinedEventIref                                                       = 1695,
    /// REFINED-MODULE-DEF-REF
    RefinedModuleDefRef                                                    = 2702,
    /// REG-DOES-NOT-REQUIRE-STABILITYS
    RegDoesNotRequireStabilitys                                            = 5524,
    /// REG-REQUIRES-STABILITYS
    RegRequiresStabilitys                                                  = 4208,
    /// REGENERATED-PRIORITY
    RegeneratedPriority                                                    = 1424,
    /// REGULAR-EXPRESSION
    RegularExpression                                                      = 3602,
    /// REKEY-INTERVAL
    RekeyInterval                                                          = 1032,
    /// RELATED-TRACE-ITEM-REF
    RelatedTraceItemRef                                                    = 3962,
    /// RELATIONS
    Relations                                                              = 4657,
    /// RELATIVE
    Relative                                                               = 5135,
    /// RELATIVE-REPRESENTATION-TIME
    RelativeRepresentationTime                                             = 1916,
    /// RELATIVE-TOLERANCE
    RelativeTolerance                                                      = 2678,
    /// RELEASE-NOTES-REF
    ReleaseNotesRef                                                        = 1131,
    /// RELEASED-TRIGGER-POLICYS
    ReleasedTriggerPolicys                                                 = 3933,
    /// RELEASED-TRIGGER-REF
    ReleasedTriggerRef                                                     = 1758,
    /// RELEASED-TRIGGERS
    ReleasedTriggers                                                       = 216,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF
    RelevantForDynamicPncMappingRef                                        = 1190,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS
    RelevantForDynamicPncMappingRefs                                       = 5080,
    /// RELIABILITY
    Reliability                                                            = 4783,
    /// REMAPPED-CAN-ID
    RemappedCanId                                                          = 2623,
    /// REMARK
    Remark                                                                 = 3616,
    /// REMOTE-ACCESS-CONTROL-ENABLED
    RemoteAccessControlEnabled                                             = 5547,
    /// REMOTE-ADDRESSS
    RemoteAddresss                                                         = 2480,
    /// REMOTE-CERTIFICATE-REF
    RemoteCertificateRef                                                   = 324,
    /// REMOTE-CERTIFICATE-REFS
    RemoteCertificateRefs                                                  = 5530,
    /// REMOTE-CLIENT-CONFIG
    RemoteClientConfig                                                     = 4641,
    /// REMOTE-ID
    RemoteId                                                               = 1361,
    /// REMOTE-IP-ADDRESS-REF
    RemoteIpAddressRef                                                     = 3261,
    /// REMOTE-IP-ADDRESS-REFS
    RemoteIpAddressRefs                                                    = 3514,
    /// REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS
    RemoteMulticastSubscriptionAddresss                                    = 2275,
    /// REMOTE-NETWORK-ENDPOINT-REF
    RemoteNetworkEndpointRef                                               = 3734,
    /// REMOTE-PORT-RANGE-END
    RemotePortRangeEnd                                                     = 1926,
    /// REMOTE-PORT-RANGE-START
    RemotePortRangeStart                                                   = 3485,
    /// REMOTE-PORT-REF
    RemotePortRef                                                          = 1095,
    /// REMOTE-SERVER-CONFIG
    RemoteServerConfig                                                     = 4041,
    /// REMOTE-SUBJECT-REF
    RemoteSubjectRef                                                       = 1928,
    /// REMOTE-SUBJECT-REFS
    RemoteSubjectRefs                                                      = 3415,
    /// REMOTE-UNICAST-ADDRESSS
    RemoteUnicastAddresss                                                  = 4587,
    /// REMOTING-TECHNOLOGY
    RemotingTechnology                                                     = 2859,
    /// REPETITION-PERIOD
    RepetitionPeriod                                                       = 5335,
    /// REPLACE-WITH
    ReplaceWith                                                            = 2251,
    /// REPLICA-PDUS-REF
    ReplicaPdusRef                                                         = 1839,
    /// REPLICA-PDUS-REFS
    ReplicaPdusRefs                                                        = 536,
    /// REPLY-TOPIC-NAME
    ReplyTopicName                                                         = 4735,
    /// REPORT-BEHAVIOR
    ReportBehavior                                                         = 646,
    /// REPORTABLE-SECURITY-EVENT-REF
    ReportableSecurityEventRef                                             = 2725,
    /// REPORTABLE-SECURITY-EVENT-REFS
    ReportableSecurityEventRefs                                            = 3172,
    /// REPORTED-SECURITY-EVENT-IREF
    ReportedSecurityEventIref                                              = 2611,
    /// REPORTING-BEHAVIOR
    ReportingBehavior                                                      = 3306,
    /// REPORTING-PORT-PROTOTYPE-IREF
    ReportingPortPrototypeIref                                             = 3229,
    /// REPOSITORY
    Repository                                                             = 4924,
    /// REPRESENTED-PORT-GROUP-REF
    RepresentedPortGroupRef                                                = 2628,
    /// REPRESENTS-VIN
    RepresentsVin                                                          = 2426,
    /// REQUEST-2-SUPPORT
    Request2Support                                                        = 1205,
    /// REQUEST-CONFIGURATIONS
    RequestConfigurations                                                  = 703,
    /// REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF
    RequestControlOfOnBoardDeviceClassRef                                  = 4434,
    /// REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF
    RequestCurrentPowertrainDiagnosticDataClassRef                         = 1708,
    /// REQUEST-DATA-SIZE
    RequestDataSize                                                        = 4297,
    /// REQUEST-DOWNLOAD-CLASS-REF
    RequestDownloadClassRef                                                = 1212,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF
    RequestEmissionRelatedDtcClassPermanentStatusRef                       = 1141,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-REF
    RequestEmissionRelatedDtcClassRef                                      = 1416,
    /// REQUEST-FILE-TRANSFER-CLASS-REF
    RequestFileTransferClassRef                                            = 3062,
    /// REQUEST-GROUP-REF
    RequestGroupRef                                                        = 2633,
    /// REQUEST-METHOD
    RequestMethod                                                          = 709,
    /// REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF
    RequestOnBoardMonitoringTestResultsClassRef                            = 4030,
    /// REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF
    RequestPowertrainFreezeFrameDataRef                                    = 3523,
    /// REQUEST-RESPONSE-DELAY
    RequestResponseDelay                                                   = 3,
    /// REQUEST-RESULT
    RequestResult                                                          = 1559,
    /// REQUEST-SEED-ID
    RequestSeedId                                                          = 1764,
    /// REQUEST-TOPIC-NAME
    RequestTopicName                                                       = 2821,
    /// REQUEST-TYPE
    RequestType                                                            = 2748,
    /// REQUEST-UPLOAD-CLASS-REF
    RequestUploadClassRef                                                  = 4501,
    /// REQUEST-VEHICLE-INFORMATION-CLASS-REF
    RequestVehicleInformationClassRef                                      = 4110,
    /// REQUESTABLE
    Requestable                                                            = 2911,
    /// REQUESTED-MODE-IREF
    RequestedModeIref                                                      = 288,
    /// REQUESTER-IREF
    RequesterIref                                                          = 4280,
    /// REQUESTER-REF
    RequesterRef                                                           = 2822,
    /// REQUESTER-REFS
    RequesterRefs                                                          = 2802,
    /// REQUESTER-SOFTWARE-CLUSTER-REF
    RequesterSoftwareClusterRef                                            = 5151,
    /// REQUESTOR-REF
    RequestorRef                                                           = 3211,
    /// REQUESTOR-REFS
    RequestorRefs                                                          = 1363,
    /// REQUESTS
    Requests                                                               = 1549,
    /// REQUIRE-RESOURCES
    RequireResources                                                       = 4191,
    /// REQUIRED-AR-ELEMENT-REF
    RequiredArElementRef                                                   = 4456,
    /// REQUIRED-AR-ELEMENT-REFS
    RequiredArElementRefs                                                  = 3578,
    /// REQUIRED-ARTIFACTS
    RequiredArtifacts                                                      = 3230,
    /// REQUIRED-CLIENT-SERVER-ENTRY-REF
    RequiredClientServerEntryRef                                           = 556,
    /// REQUIRED-CLIENT-SERVER-ENTRYS
    RequiredClientServerEntrys                                             = 3611,
    /// REQUIRED-COM-SPECS
    RequiredComSpecs                                                       = 1464,
    /// REQUIRED-DATAS
    RequiredDatas                                                          = 2868,
    /// REQUIRED-DDS-EVENT-QOS-PROPS
    RequiredDdsEventQosProps                                               = 372,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 3006,
    /// REQUIRED-ENTRYS
    RequiredEntrys                                                         = 3558,
    /// REQUIRED-ENVIRONMENT-REF
    RequiredEnvironmentRef                                                 = 2665,
    /// REQUIRED-EVENT-GROUPS
    RequiredEventGroups                                                    = 4628,
    /// REQUIRED-FIBEX-ELEMENT-REF
    RequiredFibexElementRef                                                = 992,
    /// REQUIRED-FIBEX-ELEMENT-REFS
    RequiredFibexElementRefs                                               = 3808,
    /// REQUIRED-GENERATOR-TOOLS
    RequiredGeneratorTools                                                 = 1869,
    /// REQUIRED-INTERFACE-TREF
    RequiredInterfaceTref                                                  = 5149,
    /// REQUIRED-MINOR-VERSION
    RequiredMinorVersion                                                   = 3719,
    /// REQUIRED-MODE-GROUP-REF
    RequiredModeGroupRef                                                   = 5213,
    /// REQUIRED-MODE-GROUPS
    RequiredModeGroups                                                     = 3185,
    /// REQUIRED-OUTER-PORT-REF
    RequiredOuterPortRef                                                   = 3340,
    /// REQUIRED-PACKAGE-ELEMENT-REF
    RequiredPackageElementRef                                              = 3859,
    /// REQUIRED-PACKAGE-ELEMENT-REFS
    RequiredPackageElementRefs                                             = 3797,
    /// REQUIRED-PORT-PROTOTYPE-IREF
    RequiredPortPrototypeIref                                              = 5009,
    /// REQUIRED-RTE-VENDOR
    RequiredRteVendor                                                      = 5437,
    /// REQUIRED-SERVICE-INSTANCE-ID
    RequiredServiceInstanceId                                              = 2175,
    /// REQUIRED-SERVICE-INSTANCE-REF
    RequiredServiceInstanceRef                                             = 2017,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 1557,
    /// REQUIRED-SERVICE-PORT-IREF
    RequiredServicePortIref                                                = 76,
    /// REQUIRED-SERVICE-VERSION
    RequiredServiceVersion                                                 = 5158,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 4586,
    /// REQUIRED-TRIGGER-IREF
    RequiredTriggerIref                                                    = 634,
    /// REQUIRED-TRIGGERS
    RequiredTriggers                                                       = 3574,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 2987,
    /// REQUIREMENT-REF
    RequirementRef                                                         = 2193,
    /// REQUIREMENT-REFS
    RequirementRefs                                                        = 2599,
    /// REQUIRES-INDEX
    RequiresIndex                                                          = 5401,
    /// REQUIRES-SYMBOLIC-NAME-VALUE
    RequiresSymbolicNameValue                                              = 5526,
    /// RESET-CONFIRMED-BIT-ON-OVERFLOW
    ResetConfirmedBitOnOverflow                                            = 525,
    /// RESET-TO-DEFAULT
    ResetToDefault                                                         = 672,
    /// RESET-TO-DEFAULT-SUPPORTED
    ResetToDefaultSupported                                                = 4385,
    /// RESISTANT-TO-CHANGED-SW
    ResistantToChangedSw                                                   = 3369,
    /// RESOLUTION
    Resolution                                                             = 0,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 4542,
    /// RESOURCE-DEFINITION-REF
    ResourceDefinitionRef                                                  = 5591,
    /// RESOURCE-DEFINITIONS
    ResourceDefinitions                                                    = 2403,
    /// RESOURCE-ESTIMATIONS
    ResourceEstimations                                                    = 455,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 597,
    /// RESOURCE-GROUP-REF
    ResourceGroupRef                                                       = 5023,
    /// RESOURCE-GROUP-REFS
    ResourceGroupRefs                                                      = 4415,
    /// RESOURCE-GROUPS
    ResourceGroups                                                         = 2597,
    /// RESOURCE-GUARD-VALUE
    ResourceGuardValue                                                     = 2944,
    /// RESOURCE-IDENTIFIER-TYPE
    ResourceIdentifierType                                                 = 463,
    /// RESOURCE-NEEDS-REF
    ResourceNeedsRef                                                       = 3380,
    /// RESOURCE-NEEDS-REFS
    ResourceNeedsRefs                                                      = 4084,
    /// RESOURCE-REF
    ResourceRef                                                            = 3278,
    /// RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS
    ResourceToApplicationPartitionMappings                                 = 3368,
    /// RESOURCES
    Resources                                                              = 4957,
    /// RESPOND-TO-RESET
    RespondToReset                                                         = 3396,
    /// RESPONSE-DATA-SIZE
    ResponseDataSize                                                       = 2593,
    /// RESPONSE-ERROR-POSITION
    ResponseErrorPosition                                                  = 59,
    /// RESPONSE-ERROR-REF
    ResponseErrorRef                                                       = 5612,
    /// RESPONSE-GROUP-REF
    ResponseGroupRef                                                       = 3785,
    /// RESPONSE-ON-ALL-REQUEST-SIDS
    ResponseOnAllRequestSids                                               = 840,
    /// RESPONSE-ON-EVENT-ACTION
    ResponseOnEventAction                                                  = 5423,
    /// RESPONSE-ON-EVENT-CLASS-REF
    ResponseOnEventClassRef                                                = 2792,
    /// RESPONSE-ON-EVENT-REF
    ResponseOnEventRef                                                     = 788,
    /// RESPONSE-ON-EVENT-SCHEDULER-RATE
    ResponseOnEventSchedulerRate                                           = 5129,
    /// RESPONSE-ON-SECOND-DECLINED-REQUEST
    ResponseOnSecondDeclinedRequest                                        = 4313,
    /// RESPONSE-REF
    ResponseRef                                                            = 4330,
    /// RESPONSES
    Responses                                                              = 4168,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 5384,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 2014,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 3836,
    /// REST-ENDPOINT-ARGUMENT
    RestEndpointArgument                                                   = 4951,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 5081,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 970,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 3302,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 417,
    /// REST-ENUMERATOR-DEF
    RestEnumeratorDef                                                      = 5013,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 5088,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 167,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 4506,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 612,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 2986,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 5264,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 5496,
    /// REST-SYSTEM-TRIGGERED-EVENT
    RestSystemTriggeredEvent                                               = 5386,
    /// RESTORE-AT-START
    RestoreAtStart                                                         = 2604,
    /// RESTRICT-UPDATE
    RestrictUpdate                                                         = 5125,
    /// RESTRICTION
    Restriction                                                            = 1491,
    /// RESTRICTIONS
    Restrictions                                                           = 4822,
    /// RESULTING-PROPERTIES
    ResultingProperties                                                    = 114,
    /// RESULTING-RPT-SW-PROTOTYPING-ACCESS
    ResultingRptSwPrototypingAccess                                        = 2746,
    /// RESUME-POSITION
    ResumePosition                                                         = 5211,
    /// RETRY
    Retry                                                                  = 3129,
    /// RETURN-SIGNAL-REF
    ReturnSignalRef                                                        = 2295,
    /// RETURN-SIGNAL-TRIGGERING-REF
    ReturnSignalTriggeringRef                                              = 3920,
    /// RETURN-TYPE
    ReturnType                                                             = 1069,
    /// RETURN-VALUE-PROVISION
    ReturnValueProvision                                                   = 207,
    /// REVERSED-TP-SDU-REF
    ReversedTpSduRef                                                       = 3977,
    /// REVISION-LABEL
    RevisionLabel                                                          = 4323,
    /// REVISION-LABEL-P-1
    RevisionLabelP1                                                        = 1661,
    /// REVISION-LABEL-P-2
    RevisionLabelP2                                                        = 248,
    /// REVISION-LABELS
    RevisionLabels                                                         = 3026,
    /// RID-NUMBER
    RidNumber                                                              = 4599,
    /// ROLE
    Role                                                                   = 1250,
    /// ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT
    RoleBasedBswModuleEntryAssignment                                      = 2697,
    /// ROLE-BASED-DATA-ASSIGNMENT
    RoleBasedDataAssignment                                                = 3212,
    /// ROLE-BASED-DATA-TYPE-ASSIGNMENT
    RoleBasedDataTypeAssignment                                            = 3822,
    /// ROLE-BASED-MC-DATA-ASSIGNMENT
    RoleBasedMcDataAssignment                                              = 3388,
    /// ROLE-BASED-PORT-ASSIGNMENT
    RoleBasedPortAssignment                                                = 3936,
    /// ROLE-BASED-RESOURCE-DEPENDENCY
    RoleBasedResourceDependency                                            = 4101,
    /// ROLLOUT-QUALIFICATIONS
    RolloutQualifications                                                  = 1784,
    /// ROM-BLOCK
    RomBlock                                                               = 2320,
    /// ROM-BLOCK-INIT-VALUE
    RomBlockInitValue                                                      = 3959,
    /// ROM-LOCATION-REF
    RomLocationRef                                                         = 2845,
    /// ROOT-ARGUMENT-DATA-PROTOTYPE-REF
    RootArgumentDataPrototypeRef                                           = 4010,
    /// ROOT-AUTOSAR-DATA-PROTOTYPE-REF
    RootAutosarDataPrototypeRef                                            = 4555,
    /// ROOT-COMPOSITION
    RootComposition                                                        = 4619,
    /// ROOT-CONTEXT-REF
    RootContextRef                                                         = 1066,
    /// ROOT-DATA-PROTOTYPE-IN-CS-REF
    RootDataPrototypeInCsRef                                               = 3776,
    /// ROOT-DATA-PROTOTYPE-IN-SR-REF
    RootDataPrototypeInSrRef                                               = 2722,
    /// ROOT-DATA-PROTOTYPE-REF
    RootDataPrototypeRef                                                   = 3104,
    /// ROOT-PARAMETER-DATA-PROTOTYPE-REF
    RootParameterDataPrototypeRef                                          = 4803,
    /// ROOT-REF
    RootRef                                                                = 4877,
    /// ROOT-SOFTWARE-COMPOSITIONS
    RootSoftwareCompositions                                               = 3009,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 873,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 5639,
    /// ROOT-VARIABLE-DATA-PROTOTYPE-REF
    RootVariableDataPrototypeRef                                           = 1529,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 4795,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 188,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 4150,
    /// ROUTINE-CONTROL-CLASS-REF
    RoutineControlClassRef                                                 = 1513,
    /// ROUTINE-INFO
    RoutineInfo                                                            = 1553,
    /// ROUTINE-REF
    RoutineRef                                                             = 3882,
    /// ROUTINE-SUBFUNCTION-REF
    RoutineSubfunctionRef                                                  = 5649,
    /// ROUTING-ACTIVATION-TYPE
    RoutingActivationType                                                  = 4196,
    /// ROUTING-GROUP-REF
    RoutingGroupRef                                                        = 912,
    /// ROUTING-GROUP-REFS
    RoutingGroupRefs                                                       = 3054,
    /// ROW
    Row                                                                    = 5546,
    /// RP-IMPL-POLICY
    RpImplPolicy                                                           = 4214,
    /// RPT-AR-HOOK-IREF
    RptArHookIref                                                          = 1918,
    /// RPT-COMPONENT
    RptComponent                                                           = 1023,
    /// RPT-COMPONENTS
    RptComponents                                                          = 1638,
    /// RPT-CONTAINER
    RptContainer                                                           = 2817,
    /// RPT-CONTAINERS
    RptContainers                                                          = 4140,
    /// RPT-ENABLER-IMPL-TYPE
    RptEnablerImplType                                                     = 3093,
    /// RPT-EVENT-ID
    RptEventId                                                             = 1132,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 4025,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 588,
    /// RPT-EXECUTABLE-ENTITY-EVENTS
    RptExecutableEntityEvents                                              = 97,
    /// RPT-EXECUTABLE-ENTITY-PROPERTIES
    RptExecutableEntityProperties                                          = 1,
    /// RPT-EXECUTABLE-ENTITYS
    RptExecutableEntitys                                                   = 180,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 2166,
    /// RPT-EXECUTION-CONTROL
    RptExecutionControl                                                    = 4605,
    /// RPT-HOOK
    RptHook                                                                = 308,
    /// RPT-HOOK-ACCESS
    RptHookAccess                                                          = 1903,
    /// RPT-HOOKS
    RptHooks                                                               = 3934,
    /// RPT-IMPL-POLICY
    RptImplPolicy                                                          = 2272,
    /// RPT-PREPARATION-LEVEL
    RptPreparationLevel                                                    = 1548,
    /// RPT-PROFILE
    RptProfile                                                             = 702,
    /// RPT-PROFILES
    RptProfiles                                                            = 1840,
    /// RPT-READ-ACCESS
    RptReadAccess                                                          = 1062,
    /// RPT-READS
    RptReads                                                               = 671,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 4959,
    /// RPT-SERVICE-POINT-POST-REF
    RptServicePointPostRef                                                 = 4925,
    /// RPT-SERVICE-POINT-POST-REFS
    RptServicePointPostRefs                                                = 443,
    /// RPT-SERVICE-POINT-PRE-REF
    RptServicePointPreRef                                                  = 4230,
    /// RPT-SERVICE-POINT-PRE-REFS
    RptServicePointPreRefs                                                 = 2950,
    /// RPT-SERVICE-POINTS
    RptServicePoints                                                       = 1641,
    /// RPT-SUPPORT-DATA
    RptSupportData                                                         = 358,
    /// RPT-SW-PROTOTYPING-ACCESS
    RptSwPrototypingAccess                                                 = 2274,
    /// RPT-SYSTEM-REF
    RptSystemRef                                                           = 235,
    /// RPT-WRITE-ACCESS
    RptWriteAccess                                                         = 436,
    /// RPT-WRITES
    RptWrites                                                              = 3890,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 4218,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 1523,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 2368,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 4344,
    /// RTE-EVENT-IREF
    RteEventIref                                                           = 1446,
    /// RTE-EVENT-IREFS
    RteEventIrefs                                                          = 2173,
    /// RTE-EVENT-SEPARATIONS
    RteEventSeparations                                                    = 5569,
    /// RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS
    RteEventToOsTaskProxyMappings                                          = 2388,
    /// RTE-PLUGIN-PROPS
    RtePluginProps                                                         = 4336,
    /// RTE-RESOURCE-ESTIMATION
    RteResourceEstimation                                                  = 639,
    /// RTP-TP
    RtpTp                                                                  = 5338,
    /// RTPS-PROTECTION-KIND
    RtpsProtectionKind                                                     = 2734,
    /// RULE
    Rule                                                                   = 689,
    /// RULE-ARGUMENTS
    RuleArguments                                                          = 5469,
    /// RULE-BASED-AXIS-CONT
    RuleBasedAxisCont                                                      = 3800,
    /// RULE-BASED-VALUES
    RuleBasedValues                                                        = 3507,
    /// RULE-INIT-STATE
    RuleInitState                                                          = 1485,
    /// RULES
    Rules                                                                  = 2415,
    /// RUN-MODE
    RunMode                                                                = 176,
    /// RUN-RESOURCE
    RunResource                                                            = 1520,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 4393,
    /// RUNNABLE-ENTITY-ARGUMENT
    RunnableEntityArgument                                                 = 3960,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 1570,
    /// RUNNABLE-ENTITY-GROUP-IREF
    RunnableEntityGroupIref                                                = 2930,
    /// RUNNABLE-ENTITY-GROUP-IREFS
    RunnableEntityGroupIrefs                                               = 2025,
    /// RUNNABLE-ENTITY-IREF
    RunnableEntityIref                                                     = 298,
    /// RUNNABLE-ENTITY-IREFS
    RunnableEntityIrefs                                                    = 5048,
    /// RUNNABLE-MAPPINGS
    RunnableMappings                                                       = 1433,
    /// RUNNABLE-REF
    RunnableRef                                                            = 3867,
    /// RUNNABLES
    Runnables                                                              = 4554,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REF
    RunsInsideExclusiveAreaRef                                             = 1242,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REFS
    RunsInsideExclusiveAreaRefs                                            = 3626,
    /// RUNS-INSIDES
    RunsInsides                                                            = 5352,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 1795,
    /// RUNTIME-IP-ADDRESS-CONFIGURATION
    RuntimeIpAddressConfiguration                                          = 1877,
    /// RUNTIME-PORT-CONFIGURATION
    RuntimePortConfiguration                                               = 4626,
    /// RX-ACCEPT-CONTAINED-I-PDU
    RxAcceptContainedIPdu                                                  = 5042,
    /// RX-IDENTIFIER-RANGE
    RxIdentifierRange                                                      = 1120,
    /// RX-MASK
    RxMask                                                                 = 2797,
    /// RX-NM-PDU-REF
    RxNmPduRef                                                             = 3877,
    /// RX-NM-PDU-REFS
    RxNmPduRefs                                                            = 223,
    /// RX-PDU-POOL-REF
    RxPduPoolRef                                                           = 2386,
    /// RX-PDU-TRIGGERING-REF
    RxPduTriggeringRef                                                     = 3618,
    /// RX-SECURITY-VERIFICATION
    RxSecurityVerification                                                 = 2902,
    /// RX-TIMEOUT-TIME
    RxTimeoutTime                                                          = 3983,
    /// SA-OVER-TIME
    SaOverTime                                                             = 1184,
    /// SA-RAND-TIME
    SaRandTime                                                             = 2109,
    /// SA-REKEY-TIME
    SaRekeyTime                                                            = 3782,
    /// SAFE-TRANSLATION
    SafeTranslation                                                        = 1010,
    /// SAFETY-CONDITION
    SafetyCondition                                                        = 5556,
    /// SAFETY-CONDITIONS
    SafetyConditions                                                       = 376,
    /// SAFETY-MARGIN
    SafetyMargin                                                           = 1687,
    /// SAFETY-POLICY
    SafetyPolicy                                                           = 4543,
    /// SAMPLE-CLOCK-PERIOD
    SampleClockPeriod                                                      = 4779,
    /// SAMPLES-PER-MICROTICK
    SamplesPerMicrotick                                                    = 5578,
    /// SAVE-CONFIGURATION
    SaveConfiguration                                                      = 2253,
    /// SAVE-CONFIGURATION-ENTRY
    SaveConfigurationEntry                                                 = 39,
    /// SCALE-CONSTR
    ScaleConstr                                                            = 332,
    /// SCALE-CONSTRS
    ScaleConstrs                                                           = 152,
    /// SCALING-INFO-SIZE
    ScalingInfoSize                                                        = 4533,
    /// SCHEDULE-CHANGE-NEXT-TIME-BASE
    ScheduleChangeNextTimeBase                                             = 2284,
    /// SCHEDULE-TABLES
    ScheduleTables                                                         = 3446,
    /// SCHEDULER-MAX-NUMBER
    SchedulerMaxNumber                                                     = 2434,
    /// SCHEDULER-NAME-PREFIX-REF
    SchedulerNamePrefixRef                                                 = 1470,
    /// SCHEDULER-NAME-PREFIXS
    SchedulerNamePrefixs                                                   = 2468,
    /// SCHEDULING-POLICY
    SchedulingPolicy                                                       = 4285,
    /// SCHEDULING-PRIORITY
    SchedulingPriority                                                     = 5497,
    /// SCOPE
    Scope                                                                  = 1322,
    /// SCOPE-EVENT-REF
    ScopeEventRef                                                          = 234,
    /// SCOPE-EVENT-REFS
    ScopeEventRefs                                                         = 882,
    /// SCOPE-REF
    ScopeRef                                                               = 2880,
    /// SCOPE-REFS
    ScopeRefs                                                              = 1930,
    /// SD
    Sd                                                                     = 4811,
    /// SD-CLIENT-CONFIG
    SdClientConfig                                                         = 5377,
    /// SD-CLIENT-CONFIG-REF
    SdClientConfigRef                                                      = 498,
    /// SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SdClientEventGroupTimingConfigRef                                      = 4574,
    /// SD-CLIENT-EVENT-TIMING-CONFIG
    SdClientEventTimingConfig                                              = 530,
    /// SD-CLIENT-TIMER-CONFIGS
    SdClientTimerConfigs                                                   = 5128,
    /// SD-SERVER-CONFIG
    SdServerConfig                                                         = 269,
    /// SD-SERVER-CONFIG-REF
    SdServerConfigRef                                                      = 1535,
    /// SD-SERVER-EG-TIMING-CONFIGS
    SdServerEgTimingConfigs                                                = 3655,
    /// SD-SERVER-EVENT-CONFIG
    SdServerEventConfig                                                    = 249,
    /// SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SdServerEventGroupTimingConfigRef                                      = 5078,
    /// SD-SERVER-TIMER-CONFIGS
    SdServerTimerConfigs                                                   = 5022,
    /// SDF
    Sdf                                                                    = 2995,
    /// SDG
    Sdg                                                                    = 4848,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 5017,
    /// SDG-CAPTION
    SdgCaption                                                             = 4734,
    /// SDG-CAPTION-REF
    SdgCaptionRef                                                          = 4971,
    /// SDG-CLASS
    SdgClass                                                               = 830,
    /// SDG-CLASS-REF
    SdgClassRef                                                            = 2299,
    /// SDG-CLASSES
    SdgClasses                                                             = 1297,
    /// SDG-CONSTRAINT-REF
    SdgConstraintRef                                                       = 1635,
    /// SDG-CONSTRAINT-REFS
    SdgConstraintRefs                                                      = 734,
    /// SDG-DEF
    SdgDef                                                                 = 2280,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 422,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 2218,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 1479,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 3631,
    /// SDG-REFERENCE
    SdgReference                                                           = 4055,
    /// SDG-TAILORING
    SdgTailoring                                                           = 692,
    /// SDG-TAILORINGS
    SdgTailorings                                                          = 1951,
    /// SDGS
    Sdgs                                                                   = 1805,
    /// SDU-REF
    SduRef                                                                 = 2762,
    /// SDU-REFS
    SduRefs                                                                = 841,
    /// SDX-REF
    SdxRef                                                                 = 5168,
    /// SDXF
    Sdxf                                                                   = 4485,
    /// SEARCH-BEHAVIOR
    SearchBehavior                                                         = 1959,
    /// SEARCH-INTENTION
    SearchIntention                                                        = 3746,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REF
    SecOcComPropsForMulticastRef                                           = 3792,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REFS
    SecOcComPropsForMulticastRefs                                          = 5326,
    /// SEC-OC-CRYPTO-MAPPING-REF
    SecOcCryptoMappingRef                                                  = 2826,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 1056,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 1660,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 5312,
    /// SEC-OC-JOB-MAPPINGS
    SecOcJobMappings                                                       = 3603,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 1330,
    /// SEC-OC-JOB-REQUIREMENT-REF
    SecOcJobRequirementRef                                                 = 1668,
    /// SEC-OC-JOB-SEMANTIC
    SecOcJobSemantic                                                       = 1409,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 4329,
    /// SECOND-APPLICATION-ERROR-REF
    SecondApplicationErrorRef                                              = 304,
    /// SECOND-DATA-PROTOTYPE-REF
    SecondDataPrototypeRef                                                 = 1311,
    /// SECOND-ELEMENT-INSTANCE-IREF
    SecondElementInstanceIref                                              = 1463,
    /// SECOND-ELEMENT-INSTANCE-IREFS
    SecondElementInstanceIrefs                                             = 877,
    /// SECOND-ELEMENT-REF
    SecondElementRef                                                       = 1272,
    /// SECOND-ELEMENT-REFS
    SecondElementRefs                                                      = 2643,
    /// SECOND-ELEMENTS
    SecondElements                                                         = 3932,
    /// SECOND-KEY-SLOT-ID
    SecondKeySlotId                                                        = 154,
    /// SECOND-MODE-GROUP-REF
    SecondModeGroupRef                                                     = 4212,
    /// SECOND-MODE-REF
    SecondModeRef                                                          = 2809,
    /// SECOND-OPERATION-REF
    SecondOperationRef                                                     = 2138,
    /// SECOND-PHYSICAL-DIMENSION-REF
    SecondPhysicalDimensionRef                                             = 1284,
    /// SECOND-PORT-REF
    SecondPortRef                                                          = 1670,
    /// SECOND-TO-FIRST-DATA-TRANSFORMATION-REF
    SecondToFirstDataTransformationRef                                     = 4549,
    /// SECOND-TRIGGER-REF
    SecondTriggerRef                                                       = 4159,
    /// SECOND-VALUE
    SecondValue                                                            = 1085,
    /// SECONDARY-FRESHNESS-VALUE-ID
    SecondaryFreshnessValueId                                              = 2198,
    /// SECTION-INITIALIZATION-POLICY
    SectionInitializationPolicy                                            = 2634,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 2517,
    /// SECTION-NAME-PREFIXS
    SectionNamePrefixs                                                     = 3577,
    /// SECTION-TYPE
    SectionType                                                            = 2519,
    /// SECURE-COM-CONFIG-REF
    SecureComConfigRef                                                     = 3198,
    /// SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING
    SecureComConfigToCryptoKeySlotMapping                                  = 4597,
    /// SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS
    SecureComConfigToKeySlotMappings                                       = 1266,
    /// SECURE-COM-CONFIGS
    SecureComConfigs                                                       = 2382,
    /// SECURE-COM-PROPS-FOR-DDS-REF
    SecureComPropsForDdsRef                                                = 3706,
    /// SECURE-COM-PROPS-FOR-TCP-REF
    SecureComPropsForTcpRef                                                = 2473,
    /// SECURE-COM-PROPS-FOR-TCP-REFS
    SecureComPropsForTcpRefs                                               = 3432,
    /// SECURE-COM-PROPS-FOR-UDP-REF
    SecureComPropsForUdpRef                                                = 1454,
    /// SECURE-COM-PROPS-FOR-UDP-REFS
    SecureComPropsForUdpRefs                                               = 3711,
    /// SECURE-COM-PROPS-REF
    SecureComPropsRef                                                      = 1263,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 3251,
    /// SECURE-COM-PROPSS
    SecureComPropss                                                        = 713,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 4061,
    /// SECURE-COMMUNICATION-DEPLOYMENTS
    SecureCommunicationDeployments                                         = 1125,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 759,
    /// SECURE-COMMUNICATION-PROPS
    SecureCommunicationProps                                               = 3149,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 5351,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 2895,
    /// SECURE-TRANSLATION
    SecureTranslation                                                      = 1086,
    /// SECURED-AREA-LENGTH
    SecuredAreaLength                                                      = 1732,
    /// SECURED-AREA-OFFSET
    SecuredAreaOffset                                                      = 3539,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 4256,
    /// SECURITY-ACCESS-CLASS-REF
    SecurityAccessClassRef                                                 = 5398,
    /// SECURITY-ACCESS-LEVEL
    SecurityAccessLevel                                                    = 715,
    /// SECURITY-DELAY-TIME
    SecurityDelayTime                                                      = 1227,
    /// SECURITY-DELAY-TIME-ON-BOOT
    SecurityDelayTimeOnBoot                                                = 4368,
    /// SECURITY-EVENT-CONTEXT-DATA
    SecurityEventContextData                                               = 2875,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 3208,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 1249,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 5347,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 5034,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 4814,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 1953,
    /// SECURITY-EVENT-DEFINITION-REF
    SecurityEventDefinitionRef                                             = 3515,
    /// SECURITY-EVENT-DEFINITION-REF-CONDITIONAL
    SecurityEventDefinitionRefConditional                                  = 4666,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 1837,
    /// SECURITY-EVENT-FILTER-CHAIN-REF
    SecurityEventFilterChainRef                                            = 184,
    /// SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL
    SecurityEventFilterChainRefConditional                                 = 4338,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 5487,
    /// SECURITY-EVENT-PROPS-REF
    SecurityEventPropsRef                                                  = 3443,
    /// SECURITY-EVENT-REF
    SecurityEventRef                                                       = 90,
    /// SECURITY-EVENT-REFS
    SecurityEventRefs                                                      = 1995,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 3564,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 4131,
    /// SECURITY-EVENTS
    SecurityEvents                                                         = 990,
    /// SECURITY-LEVEL-REF
    SecurityLevelRef                                                       = 2240,
    /// SECURITY-LEVEL-REFS
    SecurityLevelRefs                                                      = 2851,
    /// SEED-SIZE
    SeedSize                                                               = 1156,
    /// SEGMENT-BYTE-ORDER
    SegmentByteOrder                                                       = 1442,
    /// SEGMENT-LENGTH
    SegmentLength                                                          = 1134,
    /// SEGMENT-POSITION
    SegmentPosition                                                        = 526,
    /// SEGMENT-POSITIONS
    SegmentPositions                                                       = 1884,
    /// SEGMENT-REF
    SegmentRef                                                             = 3881,
    /// SEGMENT-REFS
    SegmentRefs                                                            = 5299,
    /// SELECT-BLOCK-FOR-FIRST-INIT-ALL
    SelectBlockForFirstInitAll                                             = 3590,
    /// SELECTIONS
    Selections                                                             = 3696,
    /// SELECTOR-FIELD-BYTE-ORDER
    SelectorFieldByteOrder                                                 = 1898,
    /// SELECTOR-FIELD-CODE
    SelectorFieldCode                                                      = 4931,
    /// SELECTOR-FIELD-LENGTH
    SelectorFieldLength                                                    = 3016,
    /// SELECTOR-FIELD-START-POSITION
    SelectorFieldStartPosition                                             = 5493,
    /// SEND-ACTIVITY
    SendActivity                                                           = 4458,
    /// SEND-INDICATION
    SendIndication                                                         = 1098,
    /// SEND-POLICYS
    SendPolicys                                                            = 2292,
    /// SEND-RESP-PEND-ON-TRANS-TO-BOOT
    SendRespPendOnTransToBoot                                              = 3700,
    /// SENDER-ANNOTATION
    SenderAnnotation                                                       = 1026,
    /// SENDER-CAPABILITY
    SenderCapability                                                       = 3245,
    /// SENDER-INTENT
    SenderIntent                                                           = 827,
    /// SENDER-IREF
    SenderIref                                                             = 2167,
    /// SENDER-REC-ARRAY-ELEMENT-MAPPING
    SenderRecArrayElementMapping                                           = 195,
    /// SENDER-REC-ARRAY-TYPE-MAPPING
    SenderRecArrayTypeMapping                                              = 2618,
    /// SENDER-REC-RECORD-ELEMENT-MAPPING
    SenderRecRecordElementMapping                                          = 4303,
    /// SENDER-REC-RECORD-TYPE-MAPPING
    SenderRecRecordTypeMapping                                             = 1455,
    /// SENDER-RECEIVER-ANNOTATIONS
    SenderReceiverAnnotations                                              = 5020,
    /// SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING
    SenderReceiverCompositeElementToSignalMapping                          = 1611,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 2739,
    /// SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING
    SenderReceiverToSignalGroupMapping                                     = 88,
    /// SENDER-RECEIVER-TO-SIGNAL-MAPPING
    SenderReceiverToSignalMapping                                          = 1658,
    /// SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING
    SenderToSignalTextTableMapping                                         = 4532,
    /// SENDING-NODE-REF
    SendingNodeRef                                                         = 4566,
    /// SENDING-NODE-REFS
    SendingNodeRefs                                                        = 5410,
    /// SENSOR-ACTUATOR-REF
    SensorActuatorRef                                                      = 1815,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 1524,
    /// SENSOR-INSTANCE-ID
    SensorInstanceId                                                       = 3238,
    /// SEPARATE-SIGNAL-PATH
    SeparateSignalPath                                                     = 493,
    /// SEPARATED-COMPONENT-IREF
    SeparatedComponentIref                                                 = 1966,
    /// SEPARATED-COMPONENT-IREFS
    SeparatedComponentIrefs                                                = 2673,
    /// SEPARATION-CYCLE-EXPONENT
    SeparationCycleExponent                                                = 1049,
    /// SEPARATION-TIME
    SeparationTime                                                         = 500,
    /// SEPARATION-TIME-REQUEST
    SeparationTimeRequest                                                  = 954,
    /// SEPARATION-TIME-RESPONSE
    SeparationTimeResponse                                                 = 31,
    /// SEQUENCE-COUNTER
    SequenceCounter                                                        = 595,
    /// SEQUENCE-COUNTER-JUMP-WIDTH
    SequenceCounterJumpWidth                                               = 3912,
    /// SEQUENCE-OFFSET
    SequenceOffset                                                         = 4850,
    /// SEQUENTIAL-INSTRUCTIONS-BEGIN
    SequentialInstructionsBegin                                            = 3908,
    /// SEQUENTIAL-INSTRUCTIONS-END
    SequentialInstructionsEnd                                              = 285,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 3708,
    /// SERIALIZATION-TECHNOLOGY-REF
    SerializationTechnologyRef                                             = 3467,
    /// SERIALIZER
    Serializer                                                             = 3339,
    /// SERIALIZER-REF
    SerializerRef                                                          = 430,
    /// SERVER-ARGUMENT-IMPL-POLICY
    ServerArgumentImplPolicy                                               = 1551,
    /// SERVER-CALL-POINTS
    ServerCallPoints                                                       = 2479,
    /// SERVER-COM-SPEC
    ServerComSpec                                                          = 3069,
    /// SERVER-NAME-IDENTIFICATION
    ServerNameIdentification                                               = 4631,
    /// SERVER-PORT-REF
    ServerPortRef                                                          = 5115,
    /// SERVER-SERVICE-MAJOR-VERSION
    ServerServiceMajorVersion                                              = 4892,
    /// SERVER-SERVICE-MINOR-VERSION
    ServerServiceMinorVersion                                              = 5024,
    /// SERVICE-CONTROL
    ServiceControl                                                         = 3366,
    /// SERVICE-DEPENDENCYS
    ServiceDependencys                                                     = 2262,
    /// SERVICE-DEPLOYMENT-REF
    ServiceDeploymentRef                                                   = 2751,
    /// SERVICE-DISCOVER-CONFIGS
    ServiceDiscoverConfigs                                                 = 3151,
    /// SERVICE-DISCOVERY-CONFIGS
    ServiceDiscoveryConfigs                                                = 1681,
    /// SERVICE-ELEMENT-MAPPING-REF
    ServiceElementMappingRef                                               = 5605,
    /// SERVICE-ELEMENT-MAPPING-REFS
    ServiceElementMappingRefs                                              = 4224,
    /// SERVICE-ELEMENT-SECURE-COM-CONFIG-REF
    ServiceElementSecureComConfigRef                                       = 3879,
    /// SERVICE-FIND-TIME-TO-LIVE
    ServiceFindTimeToLive                                                  = 3408,
    /// SERVICE-ID
    ServiceId                                                              = 5324,
    /// SERVICE-IDENTIFIER
    ServiceIdentifier                                                      = 2551,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 4181,
    /// SERVICE-INSTANCE-ID
    ServiceInstanceId                                                      = 2924,
    /// SERVICE-INSTANCE-PORT-CONFIG
    ServiceInstancePortConfig                                              = 320,
    /// SERVICE-INSTANCE-REF
    ServiceInstanceRef                                                     = 3122,
    /// SERVICE-INSTANCE-REFS
    ServiceInstanceRefs                                                    = 4841,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 4616,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 2615,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF
    ServiceInstanceToPortPrototypeMappingRef                               = 290,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 2825,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 1612,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS
    ServiceInstanceToSignalMappings                                        = 719,
    /// SERVICE-INSTANCES
    ServiceInstances                                                       = 3473,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 464,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 2994,
    /// SERVICE-INTERFACE-DEPLOYMENT-REF
    ServiceInterfaceDeploymentRef                                          = 3675,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REF
    ServiceInterfaceElementMappingRef                                      = 3613,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REFS
    ServiceInterfaceElementMappingRefs                                     = 1371,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 1456,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 797,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 5244,
    /// SERVICE-INTERFACE-ID
    ServiceInterfaceId                                                     = 4736,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 2620,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 4307,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 736,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 2124,
    /// SERVICE-INTERFACE-REF
    ServiceInterfaceRef                                                    = 2657,
    /// SERVICE-INTERFACE-REFS
    ServiceInterfaceRefs                                                   = 3947,
    /// SERVICE-INTERFACE-SUB-ELEMENT
    ServiceInterfaceSubElement                                             = 2151,
    /// SERVICE-INTERFACE-TREF
    ServiceInterfaceTref                                                   = 5071,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 4650,
    /// SERVICE-INTERFACE-VERSION
    ServiceInterfaceVersion                                                = 4954,
    /// SERVICE-ITEMS
    ServiceItems                                                           = 1542,
    /// SERVICE-KIND
    ServiceKind                                                            = 53,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 5433,
    /// SERVICE-OFFER-TIME-TO-LIVE
    ServiceOfferTimeToLive                                                 = 2614,
    /// SERVICE-POINT-SYMBOL-POST
    ServicePointSymbolPost                                                 = 3887,
    /// SERVICE-POINT-SYMBOL-PRE
    ServicePointSymbolPre                                                  = 4035,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 2144,
    /// SERVICE-REQUEST-CALLBACK-TYPE
    ServiceRequestCallbackType                                             = 5367,
    /// SERVICE-RESOURCE-REF
    ServiceResourceRef                                                     = 3169,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 4636,
    /// SERVICE-TABLES
    ServiceTables                                                          = 4561,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 2856,
    /// SESSION-CONTROL-CLASS-REF
    SessionControlClassRef                                                 = 5653,
    /// SESSION-HANDLING
    SessionHandling                                                        = 5033,
    /// SESSION-HANDLING-SR
    SessionHandlingSr                                                      = 1710,
    /// SESSION-ID
    SessionId                                                              = 2022,
    /// SESSION-ID-SUPPORT
    SessionIdSupport                                                       = 1278,
    /// SET
    Set                                                                    = 2365,
    /// SETTER-CALL-REF
    SetterCallRef                                                          = 4075,
    /// SETTER-CALL-SIGNAL-REF
    SetterCallSignalRef                                                    = 2540,
    /// SETTER-OPERATION-REF
    SetterOperationRef                                                     = 4127,
    /// SETTER-REF
    SetterRef                                                              = 4563,
    /// SETTER-RETURN-REF
    SetterReturnRef                                                        = 3167,
    /// SETTER-RETURN-SIGNAL-REF
    SetterReturnSignalRef                                                  = 1804,
    /// SEVERITY
    Severity                                                               = 161,
    /// SHALL-NOT-RUN-ON-REF
    ShallNotRunOnRef                                                       = 802,
    /// SHALL-NOT-RUN-ON-REFS
    ShallNotRunOnRefs                                                      = 1986,
    /// SHALL-RUN-ON-REF
    ShallRunOnRef                                                          = 2965,
    /// SHALL-RUN-ON-REFS
    ShallRunOnRefs                                                         = 4367,
    /// SHARED-AXIS-TYPE-REF
    SharedAxisTypeRef                                                      = 2526,
    /// SHARED-PARAMETERS
    SharedParameters                                                       = 1712,
    /// SHARED-TIMER
    SharedTimer                                                            = 3365,
    /// SHORT-LABEL
    ShortLabel                                                             = 1453,
    /// SHORT-LABEL-PATTERN
    ShortLabelPattern                                                      = 4525,
    /// SHORT-NAME
    ShortName                                                              = 478,
    /// SHORT-NAME-FRAGMENT
    ShortNameFragment                                                      = 1831,
    /// SHORT-NAME-FRAGMENTS
    ShortNameFragments                                                     = 2893,
    /// SHORT-NAME-PATTERN
    ShortNamePattern                                                       = 2974,
    /// SHORT-TERM-ADJUSTMENT
    ShortTermAdjustment                                                    = 2096,
    /// SHORT-TERM-ADJUSTMENT-SUPPORTED
    ShortTermAdjustmentSupported                                           = 2741,
    /// SHUTDOWN
    Shutdown                                                               = 1081,
    /// SIGNAL-AGE
    SignalAge                                                              = 3821,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 1790,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 5631,
    /// SIGNAL-BASED-EVENT-REF
    SignalBasedEventRef                                                    = 5350,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 2460,
    /// SIGNAL-BASED-FIELD-REF
    SignalBasedFieldRef                                                    = 5636,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 5116,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 913,
    /// SIGNAL-BASED-METHOD-REF
    SignalBasedMethodRef                                                   = 3429,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 4665,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 2036,
    /// SIGNAL-FAN
    SignalFan                                                              = 2658,
    /// SIGNAL-GROUP-REF
    SignalGroupRef                                                         = 4411,
    /// SIGNAL-I-PDU-COUNTER
    SignalIPduCounter                                                      = 2090,
    /// SIGNAL-I-PDU-REPLICATION
    SignalIPduReplication                                                  = 1500,
    /// SIGNAL-MAPPINGS
    SignalMappings                                                         = 3254,
    /// SIGNAL-PATH-CONSTRAINTS
    SignalPathConstraints                                                  = 3070,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 2888,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 163,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS
    SignalServiceTranslationEventPropss                                    = 1873,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 5622,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 1306,
    /// SIGNAL-SERVICE-TRANSLATION-PROPSS
    SignalServiceTranslationPropss                                         = 4356,
    /// SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING
    SignalToReceiverTextTableMapping                                       = 3434,
    /// SIGNALS
    Signals                                                                = 823,
    /// SIGNATURE-SCHEME-ID
    SignatureSchemeId                                                      = 4846,
    /// SIGNATURE-SCHEME-REF
    SignatureSchemeRef                                                     = 5147,
    /// SIGNATURE-SCHEME-REFS
    SignatureSchemeRefs                                                    = 577,
    /// SIGNATURE-SUPPORT-AP
    SignatureSupportAp                                                     = 5600,
    /// SIGNATURE-SUPPORT-CP
    SignatureSupportCp                                                     = 620,
    /// SIGNIFICANCE
    Significance                                                           = 1279,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 3080,
    /// SIZE
    Size                                                                   = 791,
    /// SIZE-OF-ARRAY-LENGTH-FIELD
    SizeOfArrayLengthField                                                 = 4994,
    /// SIZE-OF-ARRAY-LENGTH-FIELDS
    SizeOfArrayLengthFields                                                = 1753,
    /// SIZE-OF-STRING-LENGTH-FIELD
    SizeOfStringLengthField                                                = 1788,
    /// SIZE-OF-STRING-LENGTH-FIELDS
    SizeOfStringLengthFields                                               = 1552,
    /// SIZE-OF-STRUCT-LENGTH-FIELD
    SizeOfStructLengthField                                                = 5252,
    /// SIZE-OF-STRUCT-LENGTH-FIELDS
    SizeOfStructLengthFields                                               = 5609,
    /// SIZE-OF-UNION-LENGTH-FIELD
    SizeOfUnionLengthField                                                 = 1390,
    /// SIZE-OF-UNION-LENGTH-FIELDS
    SizeOfUnionLengthFields                                                = 4692,
    /// SIZE-OF-UNION-TYPE-SELECTOR-FIELD
    SizeOfUnionTypeSelectorField                                           = 1759,
    /// SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE
    SlaveActAsPassiveCommunicationSlave                                    = 3000,
    /// SLAVE-EVENT-REF
    SlaveEventRef                                                          = 5165,
    /// SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME
    SlaveQualifiedUnexpectedLinkDownTime                                   = 1700,
    /// SLAVE-REF
    SlaveRef                                                               = 3930,
    /// SLAVES
    Slaves                                                                 = 3391,
    /// SLEEP-MODE-EXECUTION-DELAY
    SleepModeExecutionDelay                                                = 1606,
    /// SLEEP-MODE-SUPPORTED
    SleepModeSupported                                                     = 1258,
    /// SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST
    SleepRepetitionDelayOfSleepRequest                                     = 2500,
    /// SLEEP-REPETITIONS-OF-SLEEP-REQUEST
    SleepRepetitionsOfSleepRequest                                         = 5139,
    /// SLOT-CAPACITY
    SlotCapacity                                                           = 189,
    /// SLOT-ID
    SlotId                                                                 = 2616,
    /// SLOT-TYPE
    SlotType                                                               = 3074,
    /// SNAPSHOT-RECORD-CONTENTS
    SnapshotRecordContents                                                 = 5073,
    /// SO-AD-CONFIG
    SoAdConfig                                                             = 3662,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 3333,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 3370,
    /// SO-CON-I-PDU-IDENTIFIER-REF
    SoConIPduIdentifierRef                                                 = 5484,
    /// SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL
    SoConIPduIdentifierRefConditional                                      = 5495,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 931,
    /// SOCKET-ADDRESS-REF
    SocketAddressRef                                                       = 4514,
    /// SOCKET-ADDRESS-REF-CONDITIONAL
    SocketAddressRefConditional                                            = 327,
    /// SOCKET-ADDRESSS
    SocketAddresss                                                         = 3045,
    /// SOCKET-CONNECTION
    SocketConnection                                                       = 191,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 1391,
    /// SOCKET-CONNECTION-BUNDLE-REF
    SocketConnectionBundleRef                                              = 4758,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER
    SocketConnectionIpduIdentifier                                         = 1696,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 3144,
    /// SOCKET-CONNECTION-REF
    SocketConnectionRef                                                    = 350,
    /// SOCKET-CONNECTION-REFS
    SocketConnectionRefs                                                   = 5287,
    /// SOCKET-OPTION
    SocketOption                                                           = 2180,
    /// SOCKET-OPTIONS
    SocketOptions                                                          = 4126,
    /// SOCKET-PROTOCOL
    SocketProtocol                                                         = 5040,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION
    SoftwareActivationDependencyCompareCondition                           = 54,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA
    SoftwareActivationDependencyFormula                                    = 406,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-REF
    SoftwareActivationDependencyRef                                        = 508,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 1583,
    /// SOFTWARE-CLUSTER-DEPENDENCY
    SoftwareClusterDependency                                              = 416,
    /// SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION
    SoftwareClusterDependencyCompareCondition                              = 1364,
    /// SOFTWARE-CLUSTER-DEPENDENCY-FORMULA
    SoftwareClusterDependencyFormula                                       = 1792,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 4881,
    /// SOFTWARE-CLUSTER-DESIGN-DEPENDENCY
    SoftwareClusterDesignDependency                                        = 364,
    /// SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS
    SoftwareClusterDoipDiagnosticAddress                                   = 3834,
    /// SOFTWARE-CLUSTER-ID
    SoftwareClusterId                                                      = 2648,
    /// SOFTWARE-CLUSTER-REF
    SoftwareClusterRef                                                     = 1440,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 3134,
    /// SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS
    SoftwareClusterToResourceMappings                                      = 2202,
    /// SOFTWARE-COMPOSITION-TREF
    SoftwareCompositionTref                                                = 5479,
    /// SOFTWARE-CONTEXT
    SoftwareContext                                                        = 3542,
    /// SOFTWARE-MEMORY-SECTION-REF
    SoftwareMemorySectionRef                                               = 2431,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 921,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 4283,
    /// SOFTWARE-PACKAGE-STEPS
    SoftwarePackageSteps                                                   = 2715,
    /// SOFTWARE-PACKAGE-STORING
    SoftwarePackageStoring                                                 = 2945,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 1857,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 974,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 3648,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 4468,
    /// SOMEIP-EVENT-PROPS
    SomeipEventProps                                                       = 4178,
    /// SOMEIP-FIELD
    SomeipField                                                            = 3893,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 4887,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 2985,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 2150,
    /// SOMEIP-METHOD-PROPS
    SomeipMethodProps                                                      = 1803,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 3154,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 2765,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 2567,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdClientEventGroupTimingConfigRef                                = 5209,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdClientEventGroupTimingConfigRefConditional                     = 4743,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 1846,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdClientServiceInstanceConfigRef                                 = 2034,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdClientServiceInstanceConfigRefConditional                      = 1273,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 3743,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdServerEventGroupTimingConfigRef                                = 3001,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdServerEventGroupTimingConfigRefConditional                     = 4705,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 3465,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdServerServiceInstanceConfigRef                                 = 1225,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdServerServiceInstanceConfigRefConditional                      = 3078,
    /// SOMEIP-SERVICE-DISCOVERY
    SomeipServiceDiscovery                                                 = 1405,
    /// SOMEIP-SERVICE-DISCOVERY-PORT
    SomeipServiceDiscoveryPort                                             = 1675,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 5560,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 3458,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 783,
    /// SOMEIP-SERVICE-VERSION
    SomeipServiceVersion                                                   = 4376,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 3847,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 5320,
    /// SOMEIP-TP-CONNECTION
    SomeipTpConnection                                                     = 3965,
    /// SOMEIP-TRANSFORMATION-DESCRIPTION
    SomeipTransformationDescription                                        = 946,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS
    SomeipTransformationISignalProps                                       = 1752,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    SomeipTransformationISignalPropsConditional                            = 2820,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    SomeipTransformationISignalPropsVariants                               = 1168,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 2961,
    /// SOMEIP-TRANSFORMATION-PROPS-REF
    SomeipTransformationPropsRef                                           = 3023,
    /// SOUCE-CAN-ID-REF
    SouceCanIdRef                                                          = 5373,
    /// SOURCE-ADDRESS-CODE
    SourceAddressCode                                                      = 4454,
    /// SOURCE-ADDRESS-MASK
    SourceAddressMask                                                      = 3337,
    /// SOURCE-CAN-ID-CODE
    SourceCanIdCode                                                        = 4748,
    /// SOURCE-CAN-ID-MASK
    SourceCanIdMask                                                        = 1173,
    /// SOURCE-CHANNEL
    SourceChannel                                                          = 338,
    /// SOURCE-EEC-REF
    SourceEecRef                                                           = 1139,
    /// SOURCE-EEC-REFS
    SourceEecRefs                                                          = 1466,
    /// SOURCE-ELEMENT-REF
    SourceElementRef                                                       = 4981,
    /// SOURCE-ELEMENT-REFS
    SourceElementRefs                                                      = 2117,
    /// SOURCE-EVENT-REF
    SourceEventRef                                                         = 5319,
    /// SOURCE-EVENT-REFS
    SourceEventRefs                                                        = 4425,
    /// SOURCE-FIELD-REF
    SourceFieldRef                                                         = 2978,
    /// SOURCE-FRAME-REF
    SourceFrameRef                                                         = 4031,
    /// SOURCE-I-PDU-REF
    SourceIPduRef                                                          = 382,
    /// SOURCE-ID
    SourceId                                                               = 5464,
    /// SOURCE-INSTANCE-IREF
    SourceInstanceIref                                                     = 1550,
    /// SOURCE-INSTANCE-IREFS
    SourceInstanceIrefs                                                    = 844,
    /// SOURCE-LIN-PID-REF
    SourceLinPidRef                                                        = 1515,
    /// SOURCE-METHOD-REF
    SourceMethodRef                                                        = 2559,
    /// SOURCE-POSSIBLE-ERROR-REF
    SourcePossibleErrorRef                                                 = 2557,
    /// SOURCE-REF
    SourceRef                                                              = 2451,
    /// SOURCE-SERVICE-INTERFACE-REF
    SourceServiceInterfaceRef                                              = 2217,
    /// SOURCE-SERVICE-INTERFACE-REFS
    SourceServiceInterfaceRefs                                             = 2707,
    /// SOURCE-SIGNAL-REF
    SourceSignalRef                                                        = 5500,
    /// SOURCE-TRIGGER-REF
    SourceTriggerRef                                                       = 3413,
    /// SPECIFIC-CHANNEL-REF
    SpecificChannelRef                                                     = 3329,
    /// SPECIFIC-CHANNELS
    SpecificChannels                                                       = 3422,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 5171,
    /// SPECIFICATION-DOCUMENT-SCOPES
    SpecificationDocumentScopes                                            = 2015,
    /// SPECIFICATION-SCOPE
    SpecificationScope                                                     = 5334,
    /// SPEED
    Speed                                                                  = 3751,
    /// SPEEDUP
    Speedup                                                                = 2542,
    /// SPN
    Spn                                                                    = 928,
    /// SPN-REF
    SpnRef                                                                 = 3033,
    /// SPN-REFS
    SpnRefs                                                                = 1934,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 2810,
    /// SSP-OFFSET
    SspOffset                                                              = 4261,
    /// SSRC
    Ssrc                                                                   = 5392,
    /// ST-MIN
    StMin                                                                  = 5176,
    /// STACK-USAGES
    StackUsages                                                            = 1325,
    /// STANDARD
    Standard                                                               = 2294,
    /// STANDARD-REVISION
    StandardRevision                                                       = 5586,
    /// STANDARD-REVISIONS
    StandardRevisions                                                      = 4805,
    /// START
    Start                                                                  = 3236,
    /// START-ACTION-REF
    StartActionRef                                                         = 2740,
    /// START-ACTION-REFS
    StartActionRefs                                                        = 2699,
    /// START-ADDRESS
    StartAddress                                                           = 2313,
    /// START-INDEX
    StartIndex                                                             = 487,
    /// START-ON-EVENT-REF
    StartOnEventRef                                                        = 3249,
    /// START-POSITION
    StartPosition                                                          = 3990,
    /// STARTS-ON-EVENT-REF
    StartsOnEventRef                                                       = 3273,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 3916,
    /// STARTUP-CONFIG-REF
    StartupConfigRef                                                       = 3291,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 67,
    /// STARTUP-CONFIGS
    StartupConfigs                                                         = 4956,
    /// STARTUP-OPTION
    StartupOption                                                          = 1587,
    /// STARTUP-OPTIONS
    StartupOptions                                                         = 3569,
    /// STATE
    State                                                                  = 4955,
    /// STATE-DEPENDENT-STARTUP-CONFIG
    StateDependentStartupConfig                                            = 15,
    /// STATE-DEPENDENT-STARTUP-CONFIGS
    StateDependentStartupConfigs                                           = 1893,
    /// STATE-REF
    StateRef                                                               = 2548,
    /// STATE-REFERENCES
    StateReferences                                                        = 5476,
    /// STATIC-MEMORYS
    StaticMemorys                                                          = 200,
    /// STATIC-PART
    StaticPart                                                             = 1123,
    /// STATIC-PARTS
    StaticParts                                                            = 4223,
    /// STATIC-SLOT-DURATION
    StaticSlotDuration                                                     = 3958,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 5366,
    /// STATIC-SOCKET-CONNECTION-REF
    StaticSocketConnectionRef                                              = 3526,
    /// STATIC-SOCKET-CONNECTIONS
    StaticSocketConnections                                                = 5271,
    /// STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR
    StatusBitHandlingTestFailedSinceLastClear                              = 3553,
    /// STATUS-BIT-STORAGE-TEST-FAILED
    StatusBitStorageTestFailed                                             = 5391,
    /// STATUS-ID
    StatusId                                                               = 4512,
    /// STATUS-IREF
    StatusIref                                                             = 209,
    /// STATUS-REF
    StatusRef                                                              = 3438,
    /// STATUS-SUB-TLV
    StatusSubTlv                                                           = 958,
    /// STATUSS
    Statuss                                                                = 280,
    /// STD
    Std                                                                    = 2080,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 1701,
    /// STEP-SIZE
    StepSize                                                               = 1137,
    /// STIM-ENABLER
    StimEnabler                                                            = 2935,
    /// STIMULUS-REF
    StimulusRef                                                            = 3448,
    /// STOP
    Stop                                                                   = 4627,
    /// STORAGE-CONDITION-GROUP-REF
    StorageConditionGroupRef                                               = 5390,
    /// STORAGE-CONDITIONS
    StorageConditions                                                      = 3744,
    /// STORAGE-STATE-EVALUATION
    StorageStateEvaluation                                                 = 5626,
    /// STORE-AT-SHUTDOWN
    StoreAtShutdown                                                        = 1800,
    /// STORE-CYCLIC
    StoreCyclic                                                            = 942,
    /// STORE-EMERGENCY
    StoreEmergency                                                         = 5471,
    /// STORE-EVENT-SUPPORT
    StoreEventSupport                                                      = 1896,
    /// STORE-IMMEDIATE
    StoreImmediate                                                         = 1103,
    /// STORE-ON-CHANGE
    StoreOnChange                                                          = 2054,
    /// STORING
    Storing                                                                = 3550,
    /// STREAM-IDENTIFIER
    StreamIdentifier                                                       = 1870,
    /// STRING-ENCODING
    StringEncoding                                                         = 1669,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 1689,
    /// SUB
    Sub                                                                    = 591,
    /// SUB-ATTRIBUTE-TAILORINGS
    SubAttributeTailorings                                                 = 2371,
    /// SUB-CONTAINERS
    SubContainers                                                          = 266,
    /// SUB-DOMAIN-REF
    SubDomainRef                                                           = 1823,
    /// SUB-DOMAIN-REFS
    SubDomainRefs                                                          = 2603,
    /// SUB-ELEMENT-MAPPING
    SubElementMapping                                                      = 5123,
    /// SUB-ELEMENT-MAPPINGS
    SubElementMappings                                                     = 3319,
    /// SUB-ELEMENTS
    SubElements                                                            = 2031,
    /// SUB-FUNCTION-REF
    SubFunctionRef                                                         = 5014,
    /// SUB-FUNCTION-REFS
    SubFunctionRefs                                                        = 2463,
    /// SUB-GROUP-REF
    SubGroupRef                                                            = 1237,
    /// SUB-GROUP-REFS
    SubGroupRefs                                                           = 5454,
    /// SUB-NODE-CHANNEL-REF
    SubNodeChannelRef                                                      = 380,
    /// SUB-NODE-CHANNELS
    SubNodeChannels                                                        = 1938,
    /// SUB-NODE-NUMBER
    SubNodeNumber                                                          = 1262,
    /// SUB-SDG-REF
    SubSdgRef                                                              = 4948,
    /// SUB-SOFTWARE-CLUSTER-REF
    SubSoftwareClusterRef                                                  = 609,
    /// SUB-SOFTWARE-CLUSTER-REFS
    SubSoftwareClusterRefs                                                 = 5265,
    /// SUB-TLV-CONFIG
    SubTlvConfig                                                           = 1020,
    /// SUB-TYPE
    SubType                                                                = 5564,
    /// SUBFUNCTION
    Subfunction                                                            = 5541,
    /// SUBFUNCTIONS
    Subfunctions                                                           = 2398,
    /// SUBNET-NUMBER
    SubnetNumber                                                           = 598,
    /// SUBSCRIBE-EVENTGROUP-RETRY-DELAY
    SubscribeEventgroupRetryDelay                                          = 2652,
    /// SUBSCRIBE-EVENTGROUP-RETRY-MAX
    SubscribeEventgroupRetryMax                                            = 4567,
    /// SUBSTITUTED-FRAME-REF
    SubstitutedFrameRef                                                    = 1238,
    /// SUBSTITUTED-FRAME-REFS
    SubstitutedFrameRefs                                                   = 2922,
    /// SUBTITLE
    Subtitle                                                               = 5226,
    /// SUCCESSOR-REF
    SuccessorRef                                                           = 3793,
    /// SUCCESSOR-REFS
    SuccessorRefs                                                          = 5055,
    /// SUP
    Sup                                                                    = 2074,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 119,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF
    SupervisedEntityCheckpointNeedsRef                                     = 507,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL
    SupervisedEntityCheckpointNeedsRefConditional                          = 4304,
    /// SUPERVISED-ENTITY-ID
    SupervisedEntityId                                                     = 258,
    /// SUPERVISED-ENTITY-IREF
    SupervisedEntityIref                                                   = 4009,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 2669,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 5160,
    /// SUPERVISION-CONDITION
    SupervisionCondition                                                   = 4346,
    /// SUPERVISION-CYCLE
    SupervisionCycle                                                       = 4687,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 4472,
    /// SUPERVISION-ENTITYS
    SupervisionEntitys                                                     = 3021,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 113,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 2720,
    /// SUPERVISION-MODE-CONDITIONS
    SupervisionModeConditions                                              = 529,
    /// SUPERVISION-MODES
    SupervisionModes                                                       = 880,
    /// SUPERVISION-REF
    SupervisionRef                                                         = 2309,
    /// SUPPLIER-ID
    SupplierId                                                             = 4843,
    /// SUPPLIER-VALIDATION-ORDER-REF
    SupplierValidationOrderRef                                             = 302,
    /// SUPPLIER-VALIDATION-ORDER-REFS
    SupplierValidationOrderRefs                                            = 782,
    /// SUPPORT-BUFFER-LOCKING
    SupportBufferLocking                                                   = 335,
    /// SUPPORT-DIRTY-FLAG
    SupportDirtyFlag                                                       = 4361,
    /// SUPPORT-INFO
    SupportInfo                                                            = 920,
    /// SUPPORT-INFO-BIT
    SupportInfoBit                                                         = 4737,
    /// SUPPORT-INFO-BYTE
    SupportInfoByte                                                        = 3702,
    /// SUPPORTED-CIPHER-SUITES
    SupportedCipherSuites                                                  = 1588,
    /// SUPPORTED-CONFIG-VARIANT
    SupportedConfigVariant                                                 = 2245,
    /// SUPPORTED-CONFIG-VARIANTS
    SupportedConfigVariants                                                = 686,
    /// SUPPORTED-FEATURES
    SupportedFeatures                                                      = 3865,
    /// SUPPORTED-TIMER-GRANULARITY
    SupportedTimerGranularity                                              = 4428,
    /// SUPPORTING-MATERIAL
    SupportingMaterial                                                     = 3497,
    /// SUPPORTS-ASYNCHRONOUS-MODE-SWITCH
    SupportsAsynchronousModeSwitch                                         = 2556,
    /// SUPPORTS-MULTIPLE-INSTANTIATION
    SupportsMultipleInstantiation                                          = 4896,
    /// SUPPORTS-MULTIPLE-NOTIFIER-SETS
    SupportsMultipleNotifierSets                                           = 1624,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 2287,
    /// SW-ADDR-METHOD-REF
    SwAddrMethodRef                                                        = 2921,
    /// SW-ADDRMETHOD-REF
    SwAddrmethodRef                                                        = 739,
    /// SW-ALIGNMENT
    SwAlignment                                                            = 628,
    /// SW-ARRAYSIZE
    SwArraysize                                                            = 131,
    /// SW-AXIS-CONT
    SwAxisCont                                                             = 206,
    /// SW-AXIS-CONTS
    SwAxisConts                                                            = 4255,
    /// SW-AXIS-GENERIC
    SwAxisGeneric                                                          = 2079,
    /// SW-AXIS-GROUPED
    SwAxisGrouped                                                          = 171,
    /// SW-AXIS-INDEX
    SwAxisIndex                                                            = 1410,
    /// SW-AXIS-INDIVIDUAL
    SwAxisIndividual                                                       = 2169,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 1593,
    /// SW-AXIS-TYPE-REF
    SwAxisTypeRef                                                          = 1400,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 103,
    /// SW-BIT-REPRESENTATION
    SwBitRepresentation                                                    = 2362,
    /// SW-CALIBRATION-ACCESS
    SwCalibrationAccess                                                    = 1576,
    /// SW-CALIBRATION-NOTES
    SwCalibrationNotes                                                     = 4060,
    /// SW-CALIBRATION-NOTESS
    SwCalibrationNotess                                                    = 833,
    /// SW-CALPRM-AXIS
    SwCalprmAxis                                                           = 1598,
    /// SW-CALPRM-AXIS-SET
    SwCalprmAxisSet                                                        = 2903,
    /// SW-CALPRM-IMPL
    SwCalprmImpl                                                           = 2989,
    /// SW-CALPRM-IMPLS
    SwCalprmImpls                                                          = 2631,
    /// SW-CALPRM-PROTOTYPE-REF
    SwCalprmPrototypeRef                                                   = 2282,
    /// SW-CARB-DOC
    SwCarbDoc                                                              = 3953,
    /// SW-CARB-DOCS
    SwCarbDocs                                                             = 4808,
    /// SW-CLASS-ATTR-INSTANCE-IMPL
    SwClassAttrInstanceImpl                                                = 1743,
    /// SW-CLASS-IMPL
    SwClassImpl                                                            = 5284,
    /// SW-CLASS-IMPLS
    SwClassImpls                                                           = 5624,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 5503,
    /// SW-CLASS-PROTOTYPE-REF
    SwClassPrototypeRef                                                    = 1047,
    /// SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF
    SwClusterDesignProvidedPortIref                                        = 2749,
    /// SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF
    SwClusterDesignRequiredPortIref                                        = 3551,
    /// SW-CLUSTER-MAPPINGS
    SwClusterMappings                                                      = 534,
    /// SW-CLUSTER-REF
    SwClusterRef                                                           = 1147,
    /// SW-CLUSTER-REFS
    SwClusterRefs                                                          = 1427,
    /// SW-CLUSTERS
    SwClusters                                                             = 5142,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 4129,
    /// SW-COMP-TO-ECU-MAPPING-REF
    SwCompToEcuMappingRef                                                  = 2544,
    /// SW-COMP-TO-ECU-MAPPING-REFS
    SwCompToEcuMappingRefs                                                 = 4541,
    /// SW-COMPARISON-VARIABLES
    SwComparisonVariables                                                  = 40,
    /// SW-COMPONENT-ASSIGNMENTS
    SwComponentAssignments                                                 = 3063,
    /// SW-COMPONENT-DOCUMENTATION
    SwComponentDocumentation                                               = 1039,
    /// SW-COMPONENT-DOCUMENTATIONS
    SwComponentDocumentations                                              = 1822,
    /// SW-COMPONENT-IREF
    SwComponentIref                                                        = 2704,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 1900,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 3777,
    /// SW-COMPONENT-PROTOTYPE-ASSIGNMENT
    SwComponentPrototypeAssignment                                         = 5253,
    /// SW-COMPONENT-PROTOTYPE-IREF
    SwComponentPrototypeIref                                               = 2953,
    /// SW-COMPOSITIONS
    SwCompositions                                                         = 1796,
    /// SW-DATA-DEF-PROPS
    SwDataDefProps                                                         = 349,
    /// SW-DATA-DEF-PROPS-CONDITIONAL
    SwDataDefPropsConditional                                              = 4357,
    /// SW-DATA-DEF-PROPS-VARIANTS
    SwDataDefPropsVariants                                                 = 2260,
    /// SW-DATA-DEPENDENCY
    SwDataDependency                                                       = 4591,
    /// SW-DATA-DEPENDENCY-ARGS
    SwDataDependencyArgs                                                   = 4684,
    /// SW-DATA-DEPENDENCY-FORMULA
    SwDataDependencyFormula                                                = 2630,
    /// SW-DIAGNOSTICS-NOTES
    SwDiagnosticsNotes                                                     = 2249,
    /// SW-DIAGNOSTICS-NOTESS
    SwDiagnosticsNotess                                                    = 4074,
    /// SW-FEATURE-DEF
    SwFeatureDef                                                           = 2964,
    /// SW-FEATURE-DEFS
    SwFeatureDefs                                                          = 3612,
    /// SW-FEATURE-DESC
    SwFeatureDesc                                                          = 1011,
    /// SW-FEATURE-DESCS
    SwFeatureDescs                                                         = 3622,
    /// SW-FILL-CHARACTER
    SwFillCharacter                                                        = 4847,
    /// SW-GENERIC-AXIS-DESC
    SwGenericAxisDesc                                                      = 4798,
    /// SW-GENERIC-AXIS-PARAM
    SwGenericAxisParam                                                     = 1116,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 4109,
    /// SW-GENERIC-AXIS-PARAM-TYPE-REF
    SwGenericAxisParamTypeRef                                              = 1651,
    /// SW-GENERIC-AXIS-PARAM-TYPES
    SwGenericAxisParamTypes                                                = 5614,
    /// SW-GENERIC-AXIS-PARAMS
    SwGenericAxisParams                                                    = 787,
    /// SW-HOST-VARIABLE
    SwHostVariable                                                         = 3737,
    /// SW-IMPL-MAPPINGS
    SwImplMappings                                                         = 277,
    /// SW-IMPL-POLICY
    SwImplPolicy                                                           = 2928,
    /// SW-INTENDED-RESOLUTION
    SwIntendedResolution                                                   = 202,
    /// SW-INTERPOLATION-METHOD
    SwInterpolationMethod                                                  = 4511,
    /// SW-IS-VIRTUAL
    SwIsVirtual                                                            = 4007,
    /// SW-MAINTENANCE-NOTES
    SwMaintenanceNotes                                                     = 2164,
    /// SW-MAINTENANCE-NOTESS
    SwMaintenanceNotess                                                    = 1528,
    /// SW-MAPPINGS
    SwMappings                                                             = 2777,
    /// SW-MAX-AXIS-POINTS
    SwMaxAxisPoints                                                        = 4375,
    /// SW-MAX-TEXT-SIZE
    SwMaxTextSize                                                          = 2443,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 3615,
    /// SW-MIN-AXIS-POINTS
    SwMinAxisPoints                                                        = 4640,
    /// SW-NUMBER-OF-AXIS-POINTS
    SwNumberOfAxisPoints                                                   = 2877,
    /// SW-POINTER-TARGET-PROPS
    SwPointerTargetProps                                                   = 3173,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 5455,
    /// SW-RECORD-LAYOUT-COMPONENT
    SwRecordLayoutComponent                                                = 843,
    /// SW-RECORD-LAYOUT-GROUP
    SwRecordLayoutGroup                                                    = 2296,
    /// SW-RECORD-LAYOUT-GROUP-AXIS
    SwRecordLayoutGroupAxis                                                = 3376,
    /// SW-RECORD-LAYOUT-GROUP-FROM
    SwRecordLayoutGroupFrom                                                = 1378,
    /// SW-RECORD-LAYOUT-GROUP-INDEX
    SwRecordLayoutGroupIndex                                               = 3888,
    /// SW-RECORD-LAYOUT-GROUP-STEP
    SwRecordLayoutGroupStep                                                = 2086,
    /// SW-RECORD-LAYOUT-GROUP-TO
    SwRecordLayoutGroupTo                                                  = 737,
    /// SW-RECORD-LAYOUT-REF
    SwRecordLayoutRef                                                      = 5021,
    /// SW-RECORD-LAYOUT-V
    SwRecordLayoutV                                                        = 3312,
    /// SW-RECORD-LAYOUT-V-AXIS
    SwRecordLayoutVAxis                                                    = 145,
    /// SW-RECORD-LAYOUT-V-FIX-VALUE
    SwRecordLayoutVFixValue                                                = 2407,
    /// SW-RECORD-LAYOUT-V-INDEX
    SwRecordLayoutVIndex                                                   = 4488,
    /// SW-RECORD-LAYOUT-V-PROP
    SwRecordLayoutVProp                                                    = 5251,
    /// SW-REFRESH-TIMING
    SwRefreshTiming                                                        = 1545,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 339,
    /// SW-SERVICE-IMPL-POLICY
    SwServiceImplPolicy                                                    = 5355,
    /// SW-SYSCOND
    SwSyscond                                                              = 2130,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 4260,
    /// SW-SYSTEMCONST-REF
    SwSystemconstRef                                                       = 4012,
    /// SW-SYSTEMCONST-VALUE
    SwSystemconstValue                                                     = 3457,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 1226,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REF
    SwSystemconstantValueSetRef                                            = 2644,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REFS
    SwSystemconstantValueSetRefs                                           = 4496,
    /// SW-SYSTEMCONSTANT-VALUES
    SwSystemconstantValues                                                 = 975,
    /// SW-TEST-DESC
    SwTestDesc                                                             = 669,
    /// SW-TEST-DESCS
    SwTestDescs                                                            = 4185,
    /// SW-TEXT-PROPS
    SwTextProps                                                            = 771,
    /// SW-VALUE-BLOCK-SIZE
    SwValueBlockSize                                                       = 5582,
    /// SW-VALUE-BLOCK-SIZE-MULTS
    SwValueBlockSizeMults                                                  = 4341,
    /// SW-VALUE-CONT
    SwValueCont                                                            = 5070,
    /// SW-VALUES-PHYS
    SwValuesPhys                                                           = 2576,
    /// SW-VARIABLE-IMPL
    SwVariableImpl                                                         = 2132,
    /// SW-VARIABLE-IMPLS
    SwVariableImpls                                                        = 2070,
    /// SW-VARIABLE-PROTOTYPE-REF
    SwVariablePrototypeRef                                                 = 3419,
    /// SW-VARIABLE-REFS
    SwVariableRefs                                                         = 1012,
    /// SW-VERSION
    SwVersion                                                              = 5490,
    /// SWC-BEHAVIOR-REF
    SwcBehaviorRef                                                         = 1068,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 3061,
    /// SWC-BSW-MAPPING-REF
    SwcBswMappingRef                                                       = 125,
    /// SWC-BSW-RUNNABLE-MAPPING
    SwcBswRunnableMapping                                                  = 1993,
    /// SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE
    SwcBswSynchronizedModeGroupPrototype                                   = 4089,
    /// SWC-BSW-SYNCHRONIZED-TRIGGER
    SwcBswSynchronizedTrigger                                              = 991,
    /// SWC-EXCLUSIVE-AREA-POLICY
    SwcExclusiveAreaPolicy                                                 = 2049,
    /// SWC-FLAT-SERVICE-DEPENDENCY-REF
    SwcFlatServiceDependencyRef                                            = 3957,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 711,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 1496,
    /// SWC-MAPPING-CONSTRAINT-REF
    SwcMappingConstraintRef                                                = 2410,
    /// SWC-MAPPING-CONSTRAINT-REFS
    SwcMappingConstraintRefs                                               = 1403,
    /// SWC-MODE-GROUP-IREF
    SwcModeGroupIref                                                       = 2488,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 1526,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 4521,
    /// SWC-MODE-SWITCH-EVENT-REF
    SwcModeSwitchEventRef                                                  = 1983,
    /// SWC-RUNNABLE-REF
    SwcRunnableRef                                                         = 2763,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 494,
    /// SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    SwcServiceDependencyInExecutableIref                                   = 1313,
    /// SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    SwcServiceDependencyInSystemIref                                       = 5025,
    /// SWC-SERVICE-DEPENDENCY-IREF
    SwcServiceDependencyIref                                               = 5368,
    /// SWC-TIMING
    SwcTiming                                                              = 5597,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 2656,
    /// SWC-TO-APPLICATION-PARTITION-MAPPINGS
    SwcToApplicationPartitionMappings                                      = 4354,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 1737,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT
    SwcToEcuMappingConstraint                                              = 4504,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE
    SwcToEcuMappingConstraintType                                          = 3636,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 4919,
    /// SWC-TO-SWC-OPERATION-ARGUMENTS
    SwcToSwcOperationArguments                                             = 3308,
    /// SWC-TO-SWC-SIGNAL
    SwcToSwcSignal                                                         = 3375,
    /// SWC-TRIGGER-IREF
    SwcTriggerIref                                                         = 2137,
    /// SYMBOL
    Symbol                                                                 = 1379,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 1034,
    /// SYMBOL-WINDOW
    SymbolWindow                                                           = 766,
    /// SYMBOL-WINDOW-ACTION-POINT-OFFSET
    SymbolWindowActionPointOffset                                          = 5517,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 3126,
    /// SYMBOLIC-NAME-VALUE
    SymbolicNameValue                                                      = 522,
    /// SYNC-CONFIRMATION-TIMEOUT
    SyncConfirmationTimeout                                                = 2290,
    /// SYNC-COUNTER-INIT
    SyncCounterInit                                                        = 5648,
    /// SYNC-DATA-ID-LIST
    SyncDataIdList                                                         = 3672,
    /// SYNC-DATA-ID-LISTS
    SyncDataIdLists                                                        = 4528,
    /// SYNC-FRAME-ID-COUNT-MAX
    SyncFrameIdCountMax                                                    = 2560,
    /// SYNC-INTERVAL
    SyncInterval                                                           = 3717,
    /// SYNC-JUMP-WIDTH
    SyncJumpWidth                                                          = 1029,
    /// SYNC-LOSS-THRESHOLD
    SyncLossThreshold                                                      = 4171,
    /// SYNC-LOSS-TIMEOUT
    SyncLossTimeout                                                        = 2982,
    /// SYNC-NODE-MAX
    SyncNodeMax                                                            = 514,
    /// SYNC-PERIOD
    SyncPeriod                                                             = 3265,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 1856,
    /// SYNCHRONIZATION-CONSTRAINT-TYPE
    SynchronizationConstraintType                                          = 4858,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 4315,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 1054,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 3044,
    /// SYNCHRONIZED-MODE-GROUPS
    SynchronizedModeGroups                                                 = 5098,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 3975,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 117,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 5580,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 5331,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 515,
    /// SYNCHRONIZED-TRIGGERS
    SynchronizedTriggers                                                   = 4545,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 872,
    /// SYSC-REF
    SyscRef                                                                = 3731,
    /// SYSC-STRING-REF
    SyscStringRef                                                          = 381,
    /// SYSTEM
    System                                                                 = 3475,
    /// SYSTEM-DOCUMENTATIONS
    SystemDocumentations                                                   = 2048,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 4688,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 3607,
    /// SYSTEM-MEMORY-USAGES
    SystemMemoryUsages                                                     = 347,
    /// SYSTEM-REF
    SystemRef                                                              = 2691,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 662,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 2334,
    /// SYSTEM-SIGNAL-GROUP-REF
    SystemSignalGroupRef                                                   = 3317,
    /// SYSTEM-SIGNAL-REF
    SystemSignalRef                                                        = 3505,
    /// SYSTEM-SIGNAL-REFS
    SystemSignalRefs                                                       = 2007,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 2003,
    /// SYSTEM-TRIGGERED-EVENTS
    SystemTriggeredEvents                                                  = 2197,
    /// SYSTEM-VERSION
    SystemVersion                                                          = 2626,
    /// TA-TYPE
    TaType                                                                 = 4845,
    /// TABLE
    Table                                                                  = 4111,
    /// TABLE-CAPTION
    TableCaption                                                           = 346,
    /// TABLE-ENTRYS
    TableEntrys                                                            = 2211,
    /// TAG-ID
    TagId                                                                  = 218,
    /// TAG-WITH-OPTIONAL-VALUE
    TagWithOptionalValue                                                   = 446,
    /// TAILORING-REF
    TailoringRef                                                           = 849,
    /// TAILORING-REFS
    TailoringRefs                                                          = 999,
    /// TARGET-APPLICATION-RECORD-ELEMENT-REF
    TargetApplicationRecordElementRef                                      = 4092,
    /// TARGET-CATEGORY
    TargetCategory                                                         = 865,
    /// TARGET-CHANNEL
    TargetChannel                                                          = 4698,
    /// TARGET-COMPONENT-REF
    TargetComponentRef                                                     = 2716,
    /// TARGET-DATA-ELEMENT-REF
    TargetDataElementRef                                                   = 2695,
    /// TARGET-DATA-PROTOTYPE-GROUP-REF
    TargetDataPrototypeGroupRef                                            = 1269,
    /// TARGET-DATA-PROTOTYPE-IN-CS-REF
    TargetDataPrototypeInCsRef                                             = 694,
    /// TARGET-DATA-PROTOTYPE-IN-SR-REF
    TargetDataPrototypeInSrRef                                             = 2129,
    /// TARGET-DATA-PROTOTYPE-REF
    TargetDataPrototypeRef                                                 = 3469,
    /// TARGET-DATA-PROTOYPE-REF
    TargetDataProtoypeRef                                                  = 3083,
    /// TARGET-EEC-REF
    TargetEecRef                                                           = 5635,
    /// TARGET-EEC-REFS
    TargetEecRefs                                                          = 289,
    /// TARGET-EVENT-REF
    TargetEventRef                                                         = 1778,
    /// TARGET-EVENT-REFS
    TargetEventRefs                                                        = 3107,
    /// TARGET-FIELD-REF
    TargetFieldRef                                                         = 1943,
    /// TARGET-FRAME-REF
    TargetFrameRef                                                         = 4878,
    /// TARGET-HEALTH-CHANNEL-REF
    TargetHealthChannelRef                                                 = 4681,
    /// TARGET-I-PDU
    TargetIPdu                                                             = 3850,
    /// TARGET-I-PDU-REF
    TargetIPduRef                                                          = 4637,
    /// TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TargetImplementationDataTypeElementRef                                 = 4234,
    /// TARGET-METHOD-REF
    TargetMethodRef                                                        = 5283,
    /// TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    TargetModeDeclarationGroupPrototypeRef                                 = 1740,
    /// TARGET-MODE-DECLARATION-REF
    TargetModeDeclarationRef                                               = 4211,
    /// TARGET-MODE-GROUP-REF
    TargetModeGroupRef                                                     = 4487,
    /// TARGET-MODE-REF
    TargetModeRef                                                          = 1514,
    /// TARGET-MODULE-ID
    TargetModuleId                                                         = 1395,
    /// TARGET-MODULE-REFS
    TargetModuleRefs                                                       = 793,
    /// TARGET-OPERATION-REF
    TargetOperationRef                                                     = 1233,
    /// TARGET-P-PORT-PROTOTYPE-REF
    TargetPPortPrototypeRef                                                = 1444,
    /// TARGET-P-PORT-REF
    TargetPPortRef                                                         = 4318,
    /// TARGET-PARAMETER-DATA-PROTOTYPE-REF
    TargetParameterDataPrototypeRef                                        = 3623,
    /// TARGET-PDU-TRIGGERINGS
    TargetPduTriggerings                                                   = 2046,
    /// TARGET-PHM-CHECKPOINT-REF
    TargetPhmCheckpointRef                                                 = 1340,
    /// TARGET-PHM-SUPERVISED-ENTITY-IREF
    TargetPhmSupervisedEntityIref                                          = 99,
    /// TARGET-PORT-PROTOTYPE-REF
    TargetPortPrototypeRef                                                 = 2072,
    /// TARGET-POSSIBLE-ERROR-REF
    TargetPossibleErrorRef                                                 = 1832,
    /// TARGET-PROVIDED-OPERATION-REF
    TargetProvidedOperationRef                                             = 4037,
    /// TARGET-R-PORT-PROTOTYPE-REF
    TargetRPortPrototypeRef                                                = 2057,
    /// TARGET-R-PORT-REF
    TargetRPortRef                                                         = 5532,
    /// TARGET-REF
    TargetRef                                                              = 2788,
    /// TARGET-REFS
    TargetRefs                                                             = 4362,
    /// TARGET-REQUIRED-OPERATION-REF
    TargetRequiredOperationRef                                             = 2021,
    /// TARGET-RTE-EVENT-REF
    TargetRteEventRef                                                      = 572,
    /// TARGET-RUNNABLE-ENTITY-GROUP-REF
    TargetRunnableEntityGroupRef                                           = 5282,
    /// TARGET-RUNNABLE-ENTITY-REF
    TargetRunnableEntityRef                                                = 4960,
    /// TARGET-SIGNAL-REF
    TargetSignalRef                                                        = 4112,
    /// TARGET-STATUS-REF
    TargetStatusRef                                                        = 2067,
    /// TARGET-SWC-SERVICE-DEPENDENCY-REF
    TargetSwcServiceDependencyRef                                          = 4747,
    /// TARGET-TRIGGER-REF
    TargetTriggerRef                                                       = 1608,
    /// TARGET-VARIABLE-ACCESS-REF
    TargetVariableAccessRef                                                = 4656,
    /// TARGET-VARIABLE-DATA-PROTOTYPE-REF
    TargetVariableDataPrototypeRef                                         = 4667,
    /// TASK
    Task                                                                   = 4760,
    /// TBODY
    Tbody                                                                  = 5278,
    /// TCP-ALIVE-CHECK-RESPONSE-TIMEOUT
    TcpAliveCheckResponseTimeout                                           = 5221,
    /// TCP-CONGESTION-AVOIDANCE-ENABLED
    TcpCongestionAvoidanceEnabled                                          = 2793,
    /// TCP-CONNECT-TIMEOUT
    TcpConnectTimeout                                                      = 3020,
    /// TCP-DELAYED-ACK-TIMEOUT
    TcpDelayedAckTimeout                                                   = 1639,
    /// TCP-FAST-RECOVERY-ENABLED
    TcpFastRecoveryEnabled                                                 = 1849,
    /// TCP-FAST-RETRANSMIT-ENABLED
    TcpFastRetransmitEnabled                                               = 1076,
    /// TCP-FIN-WAIT-2-TIMEOUT
    TcpFinWait2Timeout                                                     = 4661,
    /// TCP-GENERAL-INACTIVITY-TIME
    TcpGeneralInactivityTime                                               = 3307,
    /// TCP-INITIAL-INACTIVITY-TIME
    TcpInitialInactivityTime                                               = 3220,
    /// TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP
    TcpIpArpNumGratuitousArpOnStartup                                      = 431,
    /// TCP-IP-ARP-PACKET-QUEUE-ENABLED
    TcpIpArpPacketQueueEnabled                                             = 2555,
    /// TCP-IP-ARP-REQUEST-TIMEOUT
    TcpIpArpRequestTimeout                                                 = 5245,
    /// TCP-IP-ARP-TABLE-ENTRY-TIMEOUT
    TcpIpArpTableEntryTimeout                                              = 4515,
    /// TCP-IP-AUTO-IP-INIT-TIMEOUT
    TcpIpAutoIpInitTimeout                                                 = 434,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MAX
    TcpIpDhcpV6CnfDelayMax                                                 = 1017,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MIN
    TcpIpDhcpV6CnfDelayMin                                                 = 1863,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MAX
    TcpIpDhcpV6InfDelayMax                                                 = 5002,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MIN
    TcpIpDhcpV6InfDelayMin                                                 = 678,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MAX
    TcpIpDhcpV6SolDelayMax                                                 = 1282,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MIN
    TcpIpDhcpV6SolDelayMin                                                 = 857,
    /// TCP-IP-ICMP-PROPS-REF
    TcpIpIcmpPropsRef                                                      = 1924,
    /// TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED
    TcpIpIcmpV4EchoReplyEnabled                                            = 1577,
    /// TCP-IP-ICMP-V-4-TTL
    TcpIpIcmpV4Ttl                                                         = 3597,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION
    TcpIpIcmpV6EchoReplyAvoidFragmentation                                 = 3294,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED
    TcpIpIcmpV6EchoReplyEnabled                                            = 4750,
    /// TCP-IP-ICMP-V-6-HOP-LIMIT
    TcpIpIcmpV6HopLimit                                                    = 1209,
    /// TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED
    TcpIpIcmpV6MsgDestinationUnreachableEnabled                            = 2912,
    /// TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED
    TcpIpIcmpV6MsgParameterProblemEnabled                                  = 3371,
    /// TCP-IP-IP-FRAGMENTATION-RX-ENABLED
    TcpIpIpFragmentationRxEnabled                                          = 9,
    /// TCP-IP-IP-NUM-FRAGMENTS
    TcpIpIpNumFragments                                                    = 2672,
    /// TCP-IP-IP-NUM-REASS-DGRAMS
    TcpIpIpNumReassDgrams                                                  = 4352,
    /// TCP-IP-IP-REASS-TIMEOUT
    TcpIpIpReassTimeout                                                    = 389,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-COUNT
    TcpIpIpReassemblyBufferCount                                           = 4038,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-SIZE
    TcpIpIpReassemblyBufferSize                                            = 5468,
    /// TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT
    TcpIpIpReassemblySegmentCount                                          = 2044,
    /// TCP-IP-IP-REASSEMBLY-TIMEOUT
    TcpIpIpReassemblyTimeout                                               = 2394,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT
    TcpIpIpTxFragmentBufferCount                                           = 4445,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE
    TcpIpIpTxFragmentBufferSize                                            = 5311,
    /// TCP-IP-NDP-DEFAULT-REACHABLE-TIME
    TcpIpNdpDefaultReachableTime                                           = 3295,
    /// TCP-IP-NDP-DEFAULT-RETRANS-TIMER
    TcpIpNdpDefaultRetransTimer                                            = 1888,
    /// TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE
    TcpIpNdpDefaultRouterListSize                                          = 5425,
    /// TCP-IP-NDP-DEFENSIVE-PROCESSING
    TcpIpNdpDefensiveProcessing                                            = 229,
    /// TCP-IP-NDP-DELAY-FIRST-PROBE-TIME
    TcpIpNdpDelayFirstProbeTime                                            = 64,
    /// TCP-IP-NDP-DESTINATION-CACHE-SIZE
    TcpIpNdpDestinationCacheSize                                           = 5358,
    /// TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED
    TcpIpNdpDynamicHopLimitEnabled                                         = 2565,
    /// TCP-IP-NDP-DYNAMIC-MTU-ENABLED
    TcpIpNdpDynamicMtuEnabled                                              = 3666,
    /// TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED
    TcpIpNdpDynamicReachableTimeEnabled                                    = 210,
    /// TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED
    TcpIpNdpDynamicRetransTimeEnabled                                      = 5202,
    /// TCP-IP-NDP-MAX-RANDOM-FACTOR
    TcpIpNdpMaxRandomFactor                                                = 4190,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY
    TcpIpNdpMaxRtrSolicitationDelay                                        = 3239,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATIONS
    TcpIpNdpMaxRtrSolicitations                                            = 2372,
    /// TCP-IP-NDP-MIN-RANDOM-FACTOR
    TcpIpNdpMinRandomFactor                                                = 3382,
    /// TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED
    TcpIpNdpNeighborUnreachabilityDetectionEnabled                         = 4478,
    /// TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS
    TcpIpNdpNumMulticastSolicitations                                      = 4682,
    /// TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS
    TcpIpNdpNumUnicastSolicitations                                        = 1257,
    /// TCP-IP-NDP-PACKET-QUEUE-ENABLED
    TcpIpNdpPacketQueueEnabled                                             = 4643,
    /// TCP-IP-NDP-PREFIX-LIST-SIZE
    TcpIpNdpPrefixListSize                                                 = 4289,
    /// TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED
    TcpIpNdpRandomReachableTimeEnabled                                     = 567,
    /// TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED
    TcpIpNdpRndRtrSolicitationDelayEnabled                                 = 3503,
    /// TCP-IP-NDP-RTR-SOLICITATION-INTERVAL
    TcpIpNdpRtrSolicitationInterval                                        = 4788,
    /// TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS
    TcpIpNdpSlaacDadNumberOfTransmissions                                  = 5131,
    /// TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY
    TcpIpNdpSlaacDadRetransmissionDelay                                    = 2355,
    /// TCP-IP-NDP-SLAAC-DELAY-ENABLED
    TcpIpNdpSlaacDelayEnabled                                              = 3359,
    /// TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED
    TcpIpNdpSlaacOptimisticDadEnabled                                      = 5061,
    /// TCP-IP-PROPS-REF
    TcpIpPropsRef                                                          = 2412,
    /// TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC
    TcpIpTlsUseSecurityExtensionForceEncryptThenMac                        = 2977,
    /// TCP-KEEP-ALIVE-ENABLED
    TcpKeepAliveEnabled                                                    = 469,
    /// TCP-KEEP-ALIVE-INTERVAL
    TcpKeepAliveInterval                                                   = 1215,
    /// TCP-KEEP-ALIVE-PROBES-MAX
    TcpKeepAliveProbesMax                                                  = 629,
    /// TCP-KEEP-ALIVE-TIME
    TcpKeepAliveTime                                                       = 503,
    /// TCP-LOCAL-PORT
    TcpLocalPort                                                           = 3316,
    /// TCP-MAX-RTX
    TcpMaxRtx                                                              = 87,
    /// TCP-MSL
    TcpMsl                                                                 = 5000,
    /// TCP-NAGLE-ENABLED
    TcpNagleEnabled                                                        = 1714,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 1382,
    /// TCP-OPTION-FILTER-LISTS
    TcpOptionFilterLists                                                   = 4091,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 4862,
    /// TCP-PORT
    TcpPort                                                                = 5470,
    /// TCP-PORT-REF
    TcpPortRef                                                             = 5092,
    /// TCP-PROPS
    TcpProps                                                               = 2192,
    /// TCP-RECEIVE-WINDOW-MAX
    TcpReceiveWindowMax                                                    = 2561,
    /// TCP-REMOTE-PORT
    TcpRemotePort                                                          = 3652,
    /// TCP-RETRANSMISSION-TIMEOUT
    TcpRetransmissionTimeout                                               = 4404,
    /// TCP-ROLE
    TcpRole                                                                = 1501,
    /// TCP-SLOW-START-ENABLED
    TcpSlowStartEnabled                                                    = 5121,
    /// TCP-SYN-MAX-RTX
    TcpSynMaxRtx                                                           = 5480,
    /// TCP-SYN-RECEIVED-TIMEOUT
    TcpSynReceivedTimeout                                                  = 3320,
    /// TCP-TP
    TcpTp                                                                  = 1673,
    /// TCP-TP-CONFIG
    TcpTpConfig                                                            = 3750,
    /// TCP-TP-PORT
    TcpTpPort                                                              = 1268,
    /// TCP-TTL
    TcpTtl                                                                 = 1929,
    /// TCP-UDP-CONFIG
    TcpUdpConfig                                                           = 4165,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 2638,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 3345,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 1665,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS
    TdCpSoftwareClusterResourceToTdMappings                                = 1079,
    /// TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS
    TdCpSoftwareClusterToTdMappings                                        = 752,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 1183,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE
    TdEventBswInternalBehaviorType                                         = 560,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 317,
    /// TD-EVENT-BSW-MODE-DECLARATION-TYPE
    TdEventBswModeDeclarationType                                          = 993,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 590,
    /// TD-EVENT-BSW-MODULE-TYPE
    TdEventBswModuleType                                                   = 1609,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 5295,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 1418,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 2694,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 3979,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 3923,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 1595,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 4114,
    /// TD-EVENT-MODE-DECLARATION-TYPE
    TdEventModeDeclarationType                                             = 1298,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 729,
    /// TD-EVENT-OPERATION-TYPE
    TdEventOperationType                                                   = 2582,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 2663,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE
    TdEventServiceInstanceDiscoveryType                                    = 3179,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 4452,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE
    TdEventServiceInstanceEventType                                        = 5506,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 1480,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE
    TdEventServiceInstanceFieldType                                        = 3007,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 4209,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE
    TdEventServiceInstanceMethodType                                       = 1720,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 3091,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 1594,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE
    TdEventSwcInternalBehaviorType                                         = 3113,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 2002,
    /// TD-EVENT-TRIGGER-TYPE
    TdEventTriggerType                                                     = 1291,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 1058,
    /// TD-EVENT-TYPE
    TdEventType                                                            = 1649,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 2920,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE
    TdEventVariableDataPrototypeType                                       = 756,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 4123,
    /// TD-HEADER-ID-FILTERS
    TdHeaderIdFilters                                                      = 166,
    /// TD-HEADER-ID-RANGE
    TdHeaderIdRange                                                        = 1970,
    /// TD-PDU-TRIGGERING-FILTER-REF
    TdPduTriggeringFilterRef                                               = 3992,
    /// TD-PDU-TRIGGERING-FILTER-REFS
    TdPduTriggeringFilterRefs                                              = 2307,
    /// TEAR-DOWN-ACTION-REF
    TearDownActionRef                                                      = 3042,
    /// TEAR-DOWN-ACTION-REFS
    TearDownActionRefs                                                     = 5260,
    /// TEMPERATURE-EXP
    TemperatureExp                                                         = 3284,
    /// TEMPLATE-ARGUMENTS
    TemplateArguments                                                      = 3410,
    /// TEMPLATE-TYPE-REF
    TemplateTypeRef                                                        = 5112,
    /// TERMINATION-BEHAVIOR
    TerminationBehavior                                                    = 2101,
    /// TEST-ID
    TestId                                                                 = 4855,
    /// TEST-ID-REF
    TestIdRef                                                              = 4099,
    /// TEST-IDENTIFIER
    TestIdentifier                                                         = 1756,
    /// TEST-PATTERN
    TestPattern                                                            = 5097,
    /// TEST-RESULT-REF
    TestResultRef                                                          = 3518,
    /// TESTED-ITEM-REF
    TestedItemRef                                                          = 506,
    /// TESTED-ITEM-REFS
    TestedItemRefs                                                         = 1931,
    /// TEX-MATH
    TexMath                                                                = 2804,
    /// TEXT
    Text                                                                   = 1880,
    /// TEXT-TABLE-MAPPING
    TextTableMapping                                                       = 3263,
    /// TEXT-TABLE-MAPPINGS
    TextTableMappings                                                      = 2861,
    /// TEXT-TABLE-VALUE-PAIR
    TextTableValuePair                                                     = 2753,
    /// TEXT-VALUE-SPECIFICATION
    TextValueSpecification                                                 = 4544,
    /// TEXTUAL-CONDITION
    TextualCondition                                                       = 2373,
    /// TFOOT
    Tfoot                                                                  = 4048,
    /// TGROUP
    Tgroup                                                                 = 5085,
    /// THEAD
    Thead                                                                  = 2806,
    /// THRESHOLD
    Threshold                                                              = 4548,
    /// THRESHOLD-NUMBER
    ThresholdNumber                                                        = 4615,
    /// THRESHOLD-SIZE
    ThresholdSize                                                          = 4298,
    /// TIME-BASE
    TimeBase                                                               = 2146,
    /// TIME-BASE-JITTER
    TimeBaseJitter                                                         = 4449,
    /// TIME-BASE-KIND
    TimeBaseKind                                                           = 86,
    /// TIME-BASE-PROVIDER-REF
    TimeBaseProviderRef                                                    = 1960,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 2343,
    /// TIME-BASE-RESOURCE-REF
    TimeBaseResourceRef                                                    = 5306,
    /// TIME-BASE-RESOURCE-REF-CONDITIONAL
    TimeBaseResourceRefConditional                                         = 1860,
    /// TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE
    TimeBasedFdcThresholdStorageValue                                      = 2960,
    /// TIME-BASES
    TimeBases                                                              = 4054,
    /// TIME-BR
    TimeBr                                                                 = 2453,
    /// TIME-BUFFER
    TimeBuffer                                                             = 2771,
    /// TIME-CS
    TimeCs                                                                 = 1219,
    /// TIME-EXP
    TimeExp                                                                = 617,
    /// TIME-FAILED-THRESHOLD
    TimeFailedThreshold                                                    = 1735,
    /// TIME-FR-IF
    TimeFrIf                                                               = 652,
    /// TIME-HARDWARE-CORRECTION-THRESHOLD
    TimeHardwareCorrectionThreshold                                        = 3445,
    /// TIME-INTERVAL
    TimeInterval                                                           = 1042,
    /// TIME-LEAP-FUTURE-THRESHOLD
    TimeLeapFutureThreshold                                                = 1782,
    /// TIME-LEAP-HEALING-COUNTER
    TimeLeapHealingCounter                                                 = 3516,
    /// TIME-LEAP-PAST-THRESHOLD
    TimeLeapPastThreshold                                                  = 3089,
    /// TIME-MARK
    TimeMark                                                               = 3456,
    /// TIME-MASTER-PRIORITY
    TimeMasterPriority                                                     = 5101,
    /// TIME-OFFSET
    TimeOffset                                                             = 5650,
    /// TIME-PASSED-THRESHOLD
    TimePassedThreshold                                                    = 4403,
    /// TIME-PERIOD
    TimePeriod                                                             = 4093,
    /// TIME-SEG-1
    TimeSeg1                                                               = 5356,
    /// TIME-SEG-2
    TimeSeg2                                                               = 3968,
    /// TIME-SUB-TLV
    TimeSubTlv                                                             = 4860,
    /// TIME-SYNC-CLIENT
    TimeSyncClient                                                         = 10,
    /// TIME-SYNC-CORRECTION
    TimeSyncCorrection                                                     = 1773,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 4080,
    /// TIME-SYNC-P-PORT-PROTOTYPE-IREF
    TimeSyncPPortPrototypeIref                                             = 2584,
    /// TIME-SYNC-PORT-PROTOTYPE-IREF
    TimeSyncPortPrototypeIref                                              = 5031,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 2862,
    /// TIME-SYNC-R-PORT-PROTOTYPE-IREF
    TimeSyncRPortPrototypeIref                                             = 3181,
    /// TIME-SYNC-SERVER
    TimeSyncServer                                                         = 1144,
    /// TIME-SYNC-SERVER-IDENTIFIER
    TimeSyncServerIdentifier                                               = 5512,
    /// TIME-SYNC-SERVER-REF
    TimeSyncServerRef                                                      = 1747,
    /// TIME-SYNC-TECHNOLOGY
    TimeSyncTechnology                                                     = 2887,
    /// TIME-SYNCHRONIZATION
    TimeSynchronization                                                    = 4984,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 4386,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 4646,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 2028,
    /// TIME-TO-LIVE
    TimeToLive                                                             = 765,
    /// TIME-TRIGGERED-CAN-LEVEL
    TimeTriggeredCanLevel                                                  = 850,
    /// TIME-VALUE
    TimeValue                                                              = 4770,
    /// TIME-VALUE-VALUE-VARIATION-POINT
    TimeValueValueVariationPoint                                           = 4467,
    /// TIMEOUT
    Timeout                                                                = 2594,
    /// TIMEOUT-AR
    TimeoutAr                                                              = 1902,
    /// TIMEOUT-AS
    TimeoutAs                                                              = 352,
    /// TIMEOUT-BR
    TimeoutBr                                                              = 5459,
    /// TIMEOUT-BS
    TimeoutBs                                                              = 1243,
    /// TIMEOUT-CR
    TimeoutCr                                                              = 3571,
    /// TIMEOUT-CS
    TimeoutCs                                                              = 3209,
    /// TIMEOUT-SUBSTITUTION-VALUE
    TimeoutSubstitutionValue                                               = 4883,
    /// TIMESTAMP-FORMAT
    TimestampFormat                                                        = 587,
    /// TIMESTAMP-RX-ACCEPTANCE-WINDOW
    TimestampRxAcceptanceWindow                                            = 723,
    /// TIMESTAMP-SUPPORT
    TimestampSupport                                                       = 3625,
    /// TIMING-ARGUMENT-REF
    TimingArgumentRef                                                      = 1321,
    /// TIMING-ARGUMENTS
    TimingArguments                                                        = 4052,
    /// TIMING-CONDITION
    TimingCondition                                                        = 251,
    /// TIMING-CONDITION-FORMULA
    TimingConditionFormula                                                 = 4033,
    /// TIMING-CONDITION-REF
    TimingConditionRef                                                     = 1600,
    /// TIMING-CONDITIONS
    TimingConditions                                                       = 2510,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 5561,
    /// TIMING-DESCRIPTION-REF
    TimingDescriptionRef                                                   = 4274,
    /// TIMING-DESCRIPTIONS
    TimingDescriptions                                                     = 5634,
    /// TIMING-EVENT
    TimingEvent                                                            = 4174,
    /// TIMING-EVENT-REF
    TimingEventRef                                                         = 1806,
    /// TIMING-GUARANTEES
    TimingGuarantees                                                       = 1060,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 3556,
    /// TIMING-MODE-REF
    TimingModeRef                                                          = 4432,
    /// TIMING-MODES
    TimingModes                                                            = 5248,
    /// TIMING-PROPS
    TimingProps                                                            = 5563,
    /// TIMING-REQUIREMENTS
    TimingRequirements                                                     = 1358,
    /// TIMING-RESOURCE
    TimingResource                                                         = 3131,
    /// TIMING-VARIABLE-REF
    TimingVariableRef                                                      = 1210,
    /// TIMING-VARIABLES
    TimingVariables                                                        = 3397,
    /// TLS-CIPHER-SUITE
    TlsCipherSuite                                                         = 4226,
    /// TLS-CIPHER-SUITES
    TlsCipherSuites                                                        = 2608,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 2379,
    /// TLS-CONNECTION-REF
    TlsConnectionRef                                                       = 3315,
    /// TLS-CONNECTION-REFS
    TlsConnectionRefs                                                      = 2428,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 2981,
    /// TLS-CRYPTO-MAPPING-REF
    TlsCryptoMappingRef                                                    = 1775,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 4837,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 4210,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 5317,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 1664,
    /// TLS-JOB-MAPPINGS
    TlsJobMappings                                                         = 4663,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 156,
    /// TLS-JOB-REQUIREMENT-REF
    TlsJobRequirementRef                                                   = 888,
    /// TLS-JOB-SEMANTIC
    TlsJobSemantic                                                         = 5235,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 2876,
    /// TLS-SECURE-COM-PROPS-REF
    TlsSecureComPropsRef                                                   = 969,
    /// TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF
    TlvAppRecordElementInMethodIref                                        = 3901,
    /// TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    TlvAppRecordElementInServiceInterfaceIref                              = 655,
    /// TLV-ARGUMENT-REF
    TlvArgumentRef                                                         = 3739,
    /// TLV-DATA-ID
    TlvDataId                                                              = 4530,
    /// TLV-DATA-ID-0-REF
    TlvDataId0Ref                                                          = 2377,
    /// TLV-DATA-ID-0-REFS
    TlvDataId0Refs                                                         = 3525,
    /// TLV-DATA-ID-DEFINITION
    TlvDataIdDefinition                                                    = 270,
    /// TLV-DATA-ID-DEFINITION-REF
    TlvDataIdDefinitionRef                                                 = 1672,
    /// TLV-DATA-ID-DEFINITION-REFS
    TlvDataIdDefinitionRefs                                                = 1235,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 1091,
    /// TLV-DATA-ID-DEFINITIONS
    TlvDataIdDefinitions                                                   = 4073,
    /// TLV-DATA-IDS
    TlvDataIds                                                             = 5494,
    /// TLV-IMPL-RECORD-ELEMENT
    TlvImplRecordElement                                                   = 4899,
    /// TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TlvImplementationDataTypeElementRef                                    = 3200,
    /// TLV-RECORD-ELEMENT-REF
    TlvRecordElementRef                                                    = 5094,
    /// TLV-SUB-ELEMENT-REF
    TlvSubElementRef                                                       = 1565,
    /// TO-REF
    ToRef                                                                  = 3199,
    /// TOL
    Tol                                                                    = 284,
    /// TOLERANCE
    Tolerance                                                              = 300,
    /// TOLERATED-FAILED-CYCLES
    ToleratedFailedCycles                                                  = 1214,
    /// TOOL
    Tool                                                                   = 956,
    /// TOOL-VERSION
    ToolVersion                                                            = 4240,
    /// TOPIC-1
    Topic1                                                                 = 1154,
    /// TOPIC-NAME
    TopicName                                                              = 2839,
    /// TP-ADDRESS
    TpAddress                                                              = 2492,
    /// TP-ADDRESS-EXTENSION-VALUE
    TpAddressExtensionValue                                                = 3784,
    /// TP-ADDRESS-REF
    TpAddressRef                                                           = 1232,
    /// TP-ADDRESS-REFS
    TpAddressRefs                                                          = 836,
    /// TP-ADDRESSS
    TpAddresss                                                             = 4519,
    /// TP-CHANNEL-REF
    TpChannelRef                                                           = 1394,
    /// TP-CHANNELS
    TpChannels                                                             = 5432,
    /// TP-CONFIGURATION
    TpConfiguration                                                        = 3447,
    /// TP-CONNECTION-CONTROL-REF
    TpConnectionControlRef                                                 = 3669,
    /// TP-CONNECTION-CONTROLS
    TpConnectionControls                                                   = 2647,
    /// TP-CONNECTIONS
    TpConnections                                                          = 4293,
    /// TP-ECU
    TpEcu                                                                  = 301,
    /// TP-ECUS
    TpEcus                                                                 = 1505,
    /// TP-NODES
    TpNodes                                                                = 656,
    /// TP-PGS
    TpPgs                                                                  = 1003,
    /// TP-SDU-REF
    TpSduRef                                                               = 2271,
    /// TP-SDU-REFS
    TpSduRefs                                                              = 2574,
    /// TP-TECHNOLOGY
    TpTechnology                                                           = 3903,
    /// TRACE
    Trace                                                                  = 785,
    /// TRACE-REF
    TraceRef                                                               = 725,
    /// TRACE-REFS
    TraceRefs                                                              = 939,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 2464,
    /// TRACED-FAILURES
    TracedFailures                                                         = 164,
    /// TRAFFIC-CLASS
    TrafficClass                                                           = 1108,
    /// TRAFFIC-LIMITATION-FILTERS
    TrafficLimitationFilters                                               = 3066,
    /// TRANCEIVER-STANDBY-DELAY
    TranceiverStandbyDelay                                                 = 1208,
    /// TRANSFER-EXIT-CLASS-REF
    TransferExitClassRef                                                   = 2409,
    /// TRANSFER-PROPERTY
    TransferProperty                                                       = 1187,
    /// TRANSFER-REF
    TransferRef                                                            = 2332,
    /// TRANSFER-REFS
    TransferRefs                                                           = 3982,
    /// TRANSFERS
    Transfers                                                              = 876,
    /// TRANSFORMATION-COM-SPEC-PROPSS
    TransformationComSpecPropss                                            = 582,
    /// TRANSFORMATION-DESCRIPTIONS
    TransformationDescriptions                                             = 79,
    /// TRANSFORMATION-I-SIGNAL-PROPSS
    TransformationISignalPropss                                            = 4206,
    /// TRANSFORMATION-PROPS-MAPPING-SET-REF
    TransformationPropsMappingSetRef                                       = 5450,
    /// TRANSFORMATION-PROPS-REF
    TransformationPropsRef                                                 = 989,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 281,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 2613,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 4718,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING
    TransformationPropsToServiceInterfaceMapping                           = 5562,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 488,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF
    TransformationPropsToServiceInterfaceMappingSetRef                     = 2682,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS
    TransformationPropsToServiceInterfaceMappings                          = 2894,
    /// TRANSFORMATION-PROPSS
    TransformationPropss                                                   = 2706,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 2035,
    /// TRANSFORMATION-TECHNOLOGYS
    TransformationTechnologys                                              = 4613,
    /// TRANSFORMER-CHAIN-REF
    TransformerChainRef                                                    = 1143,
    /// TRANSFORMER-CHAIN-REFS
    TransformerChainRefs                                                   = 4412,
    /// TRANSFORMER-CLASS
    TransformerClass                                                       = 2133,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 4717,
    /// TRANSFORMER-REF
    TransformerRef                                                         = 1652,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 5646,
    /// TRANSFORMING-SYSTEM-SIGNAL-REF
    TransformingSystemSignalRef                                            = 571,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 3897,
    /// TRANSIT-TO-INVALID-EXTENDED
    TransitToInvalidExtended                                               = 3594,
    /// TRANSITION-REF
    TransitionRef                                                          = 2050,
    /// TRANSITION-REFS
    TransitionRefs                                                         = 3873,
    /// TRANSITIONS
    Transitions                                                            = 3738,
    /// TRANSLATION-TARGET-IREF
    TranslationTargetIref                                                  = 1006,
    /// TRANSMISSION-ACKNOWLEDGE
    TransmissionAcknowledge                                                = 3600,
    /// TRANSMISSION-DEADLINE
    TransmissionDeadline                                                   = 214,
    /// TRANSMISSION-MODE
    TransmissionMode                                                       = 4321,
    /// TRANSMISSION-MODE-CONDITION
    TransmissionModeCondition                                              = 4057,
    /// TRANSMISSION-MODE-CONDITIONS
    TransmissionModeConditions                                             = 5117,
    /// TRANSMISSION-MODE-DECLARATION
    TransmissionModeDeclaration                                            = 1387,
    /// TRANSMISSION-MODE-FALSE-TIMING
    TransmissionModeFalseTiming                                            = 2621,
    /// TRANSMISSION-MODE-TRUE-TIMING
    TransmissionModeTrueTiming                                             = 2053,
    /// TRANSMISSION-PROPS
    TransmissionProps                                                      = 4581,
    /// TRANSMISSION-START-SEQUENCE-DURATION
    TransmissionStartSequenceDuration                                      = 182,
    /// TRANSMISSION-TRIGGER
    TransmissionTrigger                                                    = 4251,
    /// TRANSMIT-CANCELLATION
    TransmitCancellation                                                   = 4546,
    /// TRANSMIT-PDU-REF
    TransmitPduRef                                                         = 1150,
    /// TRANSMIT-PDU-REFS
    TransmitPduRefs                                                        = 4595,
    /// TRANSMITTER-REF
    TransmitterRef                                                         = 762,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 1016,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 583,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF
    TransportLayerIndependentInstanceIdRef                                 = 5365,
    /// TRANSPORT-PDU-REF
    TransportPduRef                                                        = 5449,
    /// TRANSPORT-PLUGIN
    TransportPlugin                                                        = 4729,
    /// TRANSPORT-PLUGINS
    TransportPlugins                                                       = 5096,
    /// TRANSPORT-PROTOCOL
    TransportProtocol                                                      = 1844,
    /// TRANSPORT-PROTOCOLS
    TransportProtocols                                                     = 3540,
    /// TRCV-DELAY-COMPENSATION-OFFSET
    TrcvDelayCompensationOffset                                            = 1432,
    /// TRIGGER
    Trigger                                                                = 3508,
    /// TRIGGER-DIRECT-IMPLEMENTATIONS
    TriggerDirectImplementations                                           = 3973,
    /// TRIGGER-I-PDU-SEND-CONDITION
    TriggerIPduSendCondition                                               = 2590,
    /// TRIGGER-I-PDU-SEND-CONDITIONS
    TriggerIPduSendConditions                                              = 2183,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 3579,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 2159,
    /// TRIGGER-IREF
    TriggerIref                                                            = 5291,
    /// TRIGGER-MAPPING
    TriggerMapping                                                         = 3027,
    /// TRIGGER-MAPPINGS
    TriggerMappings                                                        = 2491,
    /// TRIGGER-MODE
    TriggerMode                                                            = 2758,
    /// TRIGGER-PERIOD
    TriggerPeriod                                                          = 2246,
    /// TRIGGER-PORT-ANNOTATION
    TriggerPortAnnotation                                                  = 5535,
    /// TRIGGER-PORT-ANNOTATIONS
    TriggerPortAnnotations                                                 = 826,
    /// TRIGGER-REF
    TriggerRef                                                             = 3610,
    /// TRIGGER-REF-CONDITIONAL
    TriggerRefConditional                                                  = 1861,
    /// TRIGGER-REFS
    TriggerRefs                                                            = 1465,
    /// TRIGGER-TO-SIGNAL-MAPPING
    TriggerToSignalMapping                                                 = 2392,
    /// TRIGGERING-EVENT-REF
    TriggeringEventRef                                                     = 2870,
    /// TRIGGERS
    Triggers                                                               = 3572,
    /// TRIGGERS-RECOVERY-NOTIFICATION
    TriggersRecoveryNotification                                           = 2932,
    /// TROUBLE-CODE-J-1939-REF
    TroubleCodeJ1939Ref                                                    = 78,
    /// TROUBLE-CODE-OBD-REF
    TroubleCodeObdRef                                                      = 3880,
    /// TROUBLE-CODE-UDS-REF
    TroubleCodeUdsRef                                                      = 341,
    /// TRUE-ACTION-LIST-REF
    TrueActionListRef                                                      = 3260,
    /// TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR
    TrustedPlatformExecutableLaunchBehavior                                = 4438,
    /// TT
    Tt                                                                     = 1329,
    /// TT-CAN-CLUSTER-REF
    TtCanClusterRef                                                        = 2732,
    /// TTCAN-ABSOLUTELY-SCHEDULED-TIMING
    TtcanAbsolutelyScheduledTiming                                         = 5434,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 901,
    /// TTCAN-CLUSTER-CONDITIONAL
    TtcanClusterConditional                                                = 806,
    /// TTCAN-CLUSTER-VARIANTS
    TtcanClusterVariants                                                   = 3517,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 2760,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 3796,
    /// TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    TtcanCommunicationControllerConditional                                = 452,
    /// TTCAN-COMMUNICATION-CONTROLLER-VARIANTS
    TtcanCommunicationControllerVariants                                   = 579,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 4408,
    /// TTL
    Ttl                                                                    = 4116,
    /// TWO-KEY-SLOT-MODE
    TwoKeySlotMode                                                         = 1028,
    /// TX-BIT-RATE-SWITCH
    TxBitRateSwitch                                                        = 271,
    /// TX-ENABLE-WINDOW-LENGTH
    TxEnableWindowLength                                                   = 936,
    /// TX-MASK
    TxMask                                                                 = 1468,
    /// TX-NM-PDU-REF
    TxNmPduRef                                                             = 769,
    /// TX-NM-PDU-REFS
    TxNmPduRefs                                                            = 4275,
    /// TX-PDU-POOL-REF
    TxPduPoolRef                                                           = 4017,
    /// TX-PDU-TRIGGERING-REF
    TxPduTriggeringRef                                                     = 2885,
    /// TYP
    Typ                                                                    = 4523,
    /// TYPE
    Type                                                                   = 3281,
    /// TYPE-APPROVAL
    TypeApproval                                                           = 4279,
    /// TYPE-BLUEPRINTS
    TypeBlueprints                                                         = 5316,
    /// TYPE-DEFINITION
    TypeDefinition                                                         = 3829,
    /// TYPE-EMITTER
    TypeEmitter                                                            = 905,
    /// TYPE-MAPPING
    TypeMapping                                                            = 622,
    /// TYPE-OF-DTC-SUPPORTED
    TypeOfDtcSupported                                                     = 3522,
    /// TYPE-OF-EVENT-COMBINATION-SUPPORTED
    TypeOfEventCombinationSupported                                        = 4146,
    /// TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION
    TypeOfFreezeFrameRecordNumeration                                      = 3322,
    /// TYPE-REFERENCE
    TypeReference                                                          = 700,
    /// TYPE-REFERENCE-REF
    TypeReferenceRef                                                       = 1851,
    /// TYPE-TAILORINGS
    TypeTailorings                                                         = 1181,
    /// TYPE-TREF
    TypeTref                                                               = 1051,
    /// UAS-ID
    UasId                                                                  = 2470,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 2231,
    /// UCM-MASTER-FALLBACK-REF
    UcmMasterFallbackRef                                                   = 5261,
    /// UCM-MASTER-FALLBACK-REFS
    UcmMasterFallbackRefs                                                  = 1488,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 1721,
    /// UCM-MODULE-INSTANTIATION-REF
    UcmModuleInstantiationRef                                              = 987,
    /// UCM-PROCESSINGS
    UcmProcessings                                                         = 1374,
    /// UCM-REF
    UcmRef                                                                 = 4022,
    /// UCM-SOFTWARE-VERSION
    UcmSoftwareVersion                                                     = 3812,
    /// UCM-STEP
    UcmStep                                                                = 1786,
    /// UCMS
    Ucms                                                                   = 5443,
    /// UDP-CHECKSUM-HANDLING
    UdpChecksumHandling                                                    = 1586,
    /// UDP-COLLECTION-BUFFER-SIZE-THRESHOLD
    UdpCollectionBufferSizeThreshold                                       = 3695,
    /// UDP-COLLECTION-BUFFER-TIMEOUT
    UdpCollectionBufferTimeout                                             = 1702,
    /// UDP-COLLECTION-TRIGGER
    UdpCollectionTrigger                                                   = 1826,
    /// UDP-LOCAL-PORT
    UdpLocalPort                                                           = 4324,
    /// UDP-MIN-TX-BUFFER-SIZE
    UdpMinTxBufferSize                                                     = 1558,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 3495,
    /// UDP-NM-CLUSTER-COUPLING
    UdpNmClusterCoupling                                                   = 1148,
    /// UDP-NM-CLUSTER-REF
    UdpNmClusterRef                                                        = 3351,
    /// UDP-NM-ECU
    UdpNmEcu                                                               = 4807,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 2931,
    /// UDP-PORT
    UdpPort                                                                = 5575,
    /// UDP-PORT-REF
    UdpPortRef                                                             = 198,
    /// UDP-PROPS
    UdpProps                                                               = 2545,
    /// UDP-REMOTE-PORT
    UdpRemotePort                                                          = 902,
    /// UDP-TP
    UdpTp                                                                  = 911,
    /// UDP-TP-PORT
    UdpTpPort                                                              = 1878,
    /// UDP-TTL
    UdpTtl                                                                 = 2061,
    /// UDS-DTC-NUMBER
    UdsDtcNumber                                                           = 4290,
    /// UDS-DTC-VALUE
    UdsDtcValue                                                            = 1025,
    /// UNASSIGN-FRAME-ID
    UnassignFrameId                                                        = 2772,
    /// UNASSIGNED-FRAME-TRIGGERING-REF
    UnassignedFrameTriggeringRef                                           = 3267,
    /// UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE
    UncompressedSoftwareClusterSize                                        = 4448,
    /// UNICAST-CREDENTIALS
    UnicastCredentials                                                     = 3050,
    /// UNICAST-NETWORK-ENDPOINT-REF
    UnicastNetworkEndpointRef                                              = 1482,
    /// UNICAST-SECURE-COM-PROPS-REF
    UnicastSecureComPropsRef                                               = 5133,
    /// UNICAST-SECURE-COM-PROPS-REFS
    UnicastSecureComPropsRefs                                              = 795,
    /// UNICAST-UDP-CREDENTIALS
    UnicastUdpCredentials                                                  = 4842,
    /// UNIT
    Unit                                                                   = 899,
    /// UNIT-AND-SCALING-ID
    UnitAndScalingId                                                       = 1582,
    /// UNIT-DISPLAY-NAME
    UnitDisplayName                                                        = 2063,
    /// UNIT-GROUP
    UnitGroup                                                              = 523,
    /// UNIT-GROUP-REF
    UnitGroupRef                                                           = 847,
    /// UNIT-GROUP-REFS
    UnitGroupRefs                                                          = 2171,
    /// UNIT-REF
    UnitRef                                                                = 2832,
    /// UNIT-REFS
    UnitRefs                                                               = 750,
    /// UNLIMITED-INTEGER-VALUE-VARIATION-POINT
    UnlimitedIntegerValueVariationPoint                                    = 5420,
    /// UNRESOLVED-REFERENCE-RESTRICTION
    UnresolvedReferenceRestriction                                         = 5105,
    /// UNUSED-BIT-PATTERN
    UnusedBitPattern                                                       = 5540,
    /// UPDATE
    Update                                                                 = 5602,
    /// UPDATE-INDICATION-BIT-POSITION
    UpdateIndicationBitPosition                                            = 2440,
    /// UPDATE-KIND
    UpdateKind                                                             = 3914,
    /// UPDATE-STRATEGY
    UpdateStrategy                                                         = 401,
    /// UPPER-BOUND
    UpperBound                                                             = 3269,
    /// UPPER-CAN-ID
    UpperCanId                                                             = 5444,
    /// UPPER-HEADER-BITS-TO-SHIFT
    UpperHeaderBitsToShift                                                 = 3677,
    /// UPPER-LIMIT
    UpperLimit                                                             = 1519,
    /// UPPER-MULTIPLICITY
    UpperMultiplicity                                                      = 3641,
    /// UPPER-MULTIPLICITY-INFINITE
    UpperMultiplicityInfinite                                              = 2127,
    /// UPSTREAM-REFERENCE-IREF
    UpstreamReferenceIref                                                  = 3163,
    /// URI
    Uri                                                                    = 3747,
    /// URL
    Url                                                                    = 2318,
    /// USAGE
    Usage                                                                  = 1415,
    /// USAGES
    Usages                                                                 = 792,
    /// USE-AS-CRYPTOGRAPHIC-I-PDU
    UseAsCryptographicIPdu                                                 = 2660,
    /// USE-AUTH-DATA-FRESHNESS
    UseAuthDataFreshness                                                   = 3035,
    /// USE-AUTO-VALIDATION-AT-SHUT-DOWN
    UseAutoValidationAtShutDown                                            = 4138,
    /// USE-CASE
    UseCase                                                                = 4998,
    /// USE-CLIENT-AUTHENTICATION-REQUEST
    UseClientAuthenticationRequest                                         = 1990,
    /// USE-CRC-COMP-MECHANISM
    UseCrcCompMechanism                                                    = 5185,
    /// USE-FRESHNESS-TIMESTAMP
    UseFreshnessTimestamp                                                  = 1655,
    /// USE-INSTEAD-REF
    UseInsteadRef                                                          = 3025,
    /// USE-INSTEAD-REFS
    UseInsteadRefs                                                         = 2898,
    /// USE-MAC-ADDRESS-FOR-IDENTIFICATION
    UseMacAddressForIdentification                                         = 960,
    /// USE-SECURED-PDU-HEADER
    UseSecuredPduHeader                                                    = 1499,
    /// USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT
    UseSecurityExtensionRecordSizeLimit                                    = 4589,
    /// USE-SMART-SENSOR-API
    UseSmartSensorApi                                                      = 3321,
    /// USE-VEHICLE-IDENTIFICATION-SYNC-STATUS
    UseVehicleIdentificationSyncStatus                                     = 3935,
    /// USED-CODE-GENERATOR
    UsedCodeGenerator                                                      = 1506,
    /// USED-DATA-ELEMENT
    UsedDataElement                                                        = 3145,
    /// USED-FID-REF
    UsedFidRef                                                             = 217,
    /// USED-IMPLEMENTATION-DATA-TYPE-REF
    UsedImplementationDataTypeRef                                          = 2279,
    /// USED-LANGUAGES
    UsedLanguages                                                          = 4326,
    /// USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF
    UsedLifeCycleStateDefinitionGroupRef                                   = 2369,
    /// USED-PARAMETER-ELEMENT
    UsedParameterElement                                                   = 1887,
    /// USED-PIM-REF
    UsedPimRef                                                             = 3138,
    /// USED-SECONDARY-FID-REF
    UsedSecondaryFidRef                                                    = 4431,
    /// USED-SECONDARY-FID-REFS
    UsedSecondaryFidRefs                                                   = 3403,
    /// USER-DATA-SUB-TLV
    UserDataSubTlv                                                         = 3240,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 2874,
    /// USER-DEFINED-CLUSTER-CONDITIONAL
    UserDefinedClusterConditional                                          = 4088,
    /// USER-DEFINED-CLUSTER-VARIANTS
    UserDefinedClusterVariants                                             = 1149,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 1723,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 5348,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL
    UserDefinedCommunicationControllerConditional                          = 722,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS
    UserDefinedCommunicationControllerVariants                             = 3863,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 1113,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 1913,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 3929,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 4309,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 1450,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 3226,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 4720,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 1922,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 4032,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 4559,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 1381,
    /// USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS
    UserDefinedTransformationComSpecProps                                  = 1633,
    /// USER-DEFINED-TRANSFORMATION-DESCRIPTION
    UserDefinedTransformationDescription                                   = 3786,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS
    UserDefinedTransformationISignalProps                                  = 4839,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    UserDefinedTransformationISignalPropsConditional                       = 1734,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    UserDefinedTransformationISignalPropsVariants                          = 461,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 1897,
    /// USES-ADDRESS-ARBITRATION
    UsesAddressArbitration                                                 = 2759,
    /// USES-END-TO-END-PROTECTION
    UsesEndToEndProtection                                                 = 2315,
    /// USES-MONITOR-DATA
    UsesMonitorData                                                        = 1389,
    /// V
    V                                                                      = 5383,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 2112,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 3471,
    /// V-2-X-SUPPORTED
    V2XSupported                                                           = 2770,
    /// V-LAN-REF
    VLanRef                                                                = 730,
    /// V-LAN-REFS
    VLanRefs                                                               = 4014,
    /// VALID-BINDING-TIME
    ValidBindingTime                                                       = 816,
    /// VALID-BINDING-TIMES
    ValidBindingTimes                                                      = 981,
    /// VALIDATION-CONFIGURATION
    ValidationConfiguration                                                = 4572,
    /// VALIDATION-FORMULA
    ValidationFormula                                                      = 2179,
    /// VALIDATION-ROOT
    ValidationRoot                                                         = 2327,
    /// VALUE
    Value                                                                  = 323,
    /// VALUE-ACCESS
    ValueAccess                                                            = 4197,
    /// VALUE-AXIS-DATA-TYPE-REF
    ValueAxisDataTypeRef                                                   = 4333,
    /// VALUE-CONFIG-CLASSES
    ValueConfigClasses                                                     = 5154,
    /// VALUE-DATA-TYPE-REF
    ValueDataTypeRef                                                       = 1780,
    /// VALUE-IREF
    ValueIref                                                              = 1244,
    /// VALUE-PAIRS
    ValuePairs                                                             = 2580,
    /// VALUE-REF
    ValueRef                                                               = 2145,
    /// VALUE-RESTRICTION
    ValueRestriction                                                       = 1253,
    /// VALUE-SPEC
    ValueSpec                                                              = 4079,
    /// VALUE-TYPE-REF
    ValueTypeRef                                                           = 3334,
    /// VALUE-TYPE-TREF
    ValueTypeTref                                                          = 4292,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 4213,
    /// VARIABLE-ACCESS-IREF
    VariableAccessIref                                                     = 794,
    /// VARIABLE-ACCESS-IREFS
    VariableAccessIrefs                                                    = 4964,
    /// VARIABLE-ACCESS-REF
    VariableAccessRef                                                      = 4378,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REF
    VariableAccessedForDebugRef                                            = 3645,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REFS
    VariableAccessedForDebugRefs                                           = 5016,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 5279,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 2244,
    /// VARIABLE-DATA-PROTOTYPE-IREF
    VariableDataPrototypeIref                                              = 3400,
    /// VARIABLE-INSTANCE
    VariableInstance                                                       = 3364,
    /// VARIABLE-INSTANCE-IREF
    VariableInstanceIref                                                   = 5440,
    /// VARIABLE-LENGTH
    VariableLength                                                         = 1807,
    /// VARIABLE-REF
    VariableRef                                                            = 4156,
    /// VARIABLES
    Variables                                                              = 5567,
    /// VARIANT-CRITERION-REF
    VariantCriterionRef                                                    = 3013,
    /// VARIANT-ID
    VariantId                                                              = 728,
    /// VARIATION
    Variation                                                              = 5259,
    /// VARIATION-POINT
    VariationPoint                                                         = 343,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 3147,
    /// VARIATION-POINT-PROXYS
    VariationPointProxys                                                   = 1333,
    /// VARIATION-RESTRICTION
    VariationRestriction                                                   = 5431,
    /// VEHICLE-ANNOUNCEMENT-COUNT
    VehicleAnnouncementCount                                               = 2191,
    /// VEHICLE-ANNOUNCEMENT-INTERVAL
    VehicleAnnouncementInterval                                            = 5241,
    /// VEHICLE-DESCRIPTION-REF
    VehicleDescriptionRef                                                  = 5232,
    /// VEHICLE-DRIVER-NOTIFICATION
    VehicleDriverNotification                                              = 2071,
    /// VEHICLE-IDENTIFICATION-SYNC-STATUS
    VehicleIdentificationSyncStatus                                        = 2705,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 390,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 467,
    /// VEHICLE-SYSTEM
    VehicleSystem                                                          = 1618,
    /// VEHICLE-SYSTEM-INSTANCE
    VehicleSystemInstance                                                  = 5214,
    /// VENDOR
    Vendor                                                                 = 3691,
    /// VENDOR-API-INFIX
    VendorApiInfix                                                         = 5062,
    /// VENDOR-ID
    VendorId                                                               = 5442,
    /// VENDOR-SIGNATURE-REF
    VendorSignatureRef                                                     = 5242,
    /// VENDOR-SPECIFIC-MODULE-DEF-REF
    VendorSpecificModuleDefRef                                             = 3608,
    /// VENDOR-SPECIFIC-MODULE-DEF-REFS
    VendorSpecificModuleDefRefs                                            = 4391,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 1285,
    /// VERBATIM
    Verbatim                                                               = 2446,
    /// VERIFICATION-HASH
    VerificationHash                                                       = 2110,
    /// VERIFICATION-STATUS-INDICATION-MODE
    VerificationStatusIndicationMode                                       = 4993,
    /// VERIFY-IREF
    VerifyIref                                                             = 83,
    /// VERIFY-IREFS
    VerifyIrefs                                                            = 772,
    /// VERIFY-REF
    VerifyRef                                                              = 4343,
    /// VERIFY-REFS
    VerifyRefs                                                             = 5029,
    /// VERSION
    Version                                                                = 2966,
    /// VERSION-DRIVEN-FIND-BEHAVIOR
    VersionDrivenFindBehavior                                              = 2667,
    /// VF
    Vf                                                                     = 3592,
    /// VFB-TIMING
    VfbTiming                                                              = 2156,
    /// VFC-IREF
    VfcIref                                                                = 413,
    /// VFC-IREFS
    VfcIrefs                                                               = 3587,
    /// VG
    Vg                                                                     = 5150,
    /// VIEW-MAP
    ViewMap                                                                = 731,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 4377,
    /// VIEW-MAPS
    ViewMaps                                                               = 2997,
    /// VIN-INVALIDITY-PATTERN
    VinInvalidityPattern                                                   = 1335,
    /// VLAN
    Vlan                                                                   = 3697,
    /// VLAN-IDENTIFIER
    VlanIdentifier                                                         = 5529,
    /// VLAN-MEMBERSHIP
    VlanMembership                                                         = 368,
    /// VLAN-MEMBERSHIPS
    VlanMemberships                                                        = 4078,
    /// VLAN-MODIFIER-REF
    VlanModifierRef                                                        = 1776,
    /// VLAN-PRIORITY
    VlanPriority                                                           = 2215,
    /// VLAN-REF
    VlanRef                                                                = 3303,
    /// VLAN-REFS
    VlanRefs                                                               = 4813,
    /// VT
    Vt                                                                     = 28,
    /// VTF
    Vtf                                                                    = 1337,
    /// WAIT-POINT
    WaitPoint                                                              = 2400,
    /// WAIT-POINTS
    WaitPoints                                                             = 2208,
    /// WAKE-UP-BY-CONTROLLER-SUPPORTED
    WakeUpByControllerSupported                                            = 4838,
    /// WAKE-UP-CHANNEL
    WakeUpChannel                                                          = 3974,
    /// WAKE-UP-OVER-BUS-SUPPORTED
    WakeUpOverBusSupported                                                 = 935,
    /// WAKE-UP-PATTERN
    WakeUpPattern                                                          = 2416,
    /// WAKEUP-FORWARD-LOCAL-ENABLED
    WakeupForwardLocalEnabled                                              = 2188,
    /// WAKEUP-FORWARD-REMOTE-ENABLED
    WakeupForwardRemoteEnabled                                             = 5531,
    /// WAKEUP-FRAME-REF
    WakeupFrameRef                                                         = 2158,
    /// WAKEUP-FRAME-REFS
    WakeupFrameRefs                                                        = 4473,
    /// WAKEUP-LOCAL-DETECTION-TIME
    WakeupLocalDetectionTime                                               = 1979,
    /// WAKEUP-LOCAL-DURATION-TIME
    WakeupLocalDurationTime                                                = 1572,
    /// WAKEUP-LOCAL-ENABLED
    WakeupLocalEnabled                                                     = 5152,
    /// WAKEUP-REMOTE-ENABLED
    WakeupRemoteEnabled                                                    = 2076,
    /// WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST
    WakeupRepetitionDelayOfWakeupRequest                                   = 887,
    /// WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST
    WakeupRepetitionsOfWakeupRequest                                       = 4513,
    /// WAKEUP-RX-IDLE
    WakeupRxIdle                                                           = 3991,
    /// WAKEUP-RX-LOW
    WakeupRxLow                                                            = 3830,
    /// WAKEUP-RX-WINDOW
    WakeupRxWindow                                                         = 340,
    /// WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF
    WakeupSleepOnDatalineConfigRef                                         = 3755,
    /// WAKEUP-SYMBOL-RX-WINDOW
    WakeupSymbolRxWindow                                                   = 2726,
    /// WAKEUP-TX-ACTIVE
    WakeupTxActive                                                         = 415,
    /// WAKEUP-TX-IDLE
    WakeupTxIdle                                                           = 213,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 5137,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 1188,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 1428,
    /// WINDOW-SIZE
    WindowSize                                                             = 2645,
    /// WINDOW-SIZE-INIT
    WindowSizeInit                                                         = 3596,
    /// WINDOW-SIZE-INVALID
    WindowSizeInvalid                                                      = 5452,
    /// WINDOW-SIZE-VALID
    WindowSizeValid                                                        = 3699,
    /// WITH-AUTO
    WithAuto                                                               = 1436,
    /// WORST-CASE-EXECUTION-TIME
    WorstCaseExecutionTime                                                 = 2737,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 2162,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 5385,
    /// WRITE
    Write                                                                  = 580,
    /// WRITE-ACCESS
    WriteAccess                                                            = 1000,
    /// WRITE-CLASS-REF
    WriteClassRef                                                          = 1248,
    /// WRITE-ONLY-ONCE
    WriteOnlyOnce                                                          = 5256,
    /// WRITE-VERIFICATION
    WriteVerification                                                      = 4976,
    /// WRITING-FREQUENCY
    WritingFrequency                                                       = 2572,
    /// WRITING-PRIORITY
    WritingPriority                                                        = 70,
    /// WRITING-STRATEGY-ROLE
    WritingStrategyRole                                                    = 2958,
    /// WRITING-STRATEGYS
    WritingStrategys                                                       = 3237,
    /// WRITTEN-LOCAL-VARIABLES
    WrittenLocalVariables                                                  = 2712,
    /// WRITTEN-NV-DATA
    WrittenNvData                                                          = 4347,
    /// WRITTEN-READ-NV-DATA
    WrittenReadNvData                                                      = 834,
    /// WWH-OBD-DTC-CLASS
    WwhObdDtcClass                                                         = 4630,
    /// X
    X                                                                      = 4897,
    /// XCP-PDU
    XcpPdu                                                                 = 5396,
    /// XDOC
    Xdoc                                                                   = 2692,
    /// XFILE
    Xfile                                                                  = 5638,
    /// XREF
    Xref                                                                   = 2281,
    /// XREF-TARGET
    XrefTarget                                                             = 306,
}

impl ElementName {
    const STRING_TABLE: [&'static str; 5662] = ["RESOLUTION", "RPT-EXECUTABLE-ENTITY-PROPERTIES", "ASSOCIATED-RTE-PLUGIN-REF", "REQUEST-RESPONSE-DELAY", "FLEXRAY-AR-TP-CHANNEL", "OBD-CONTROL-SERVICE-NEEDS", "CYCLE-TIME-MAIN-FUNCTION", "ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL", "DIAGNOSTIC-COM-CONTROL", "TCP-IP-IP-FRAGMENTATION-RX-ENABLED", "TIME-SYNC-CLIENT", "RECEIVE-WINDOW-MIN", "MODE-USER-ERROR-BEHAVIOR", "INHIBITING-SECONDARY-FID-REFS", "IKE-REAUTH-TIME", "STATE-DEPENDENT-STARTUP-CONFIG", "COUPLING-PORTS", "KEY-EXCHANGE-AUTHENTICATION-REF", "CLIENT-ID-DEFINITION", "POSSIBLE-ERROR-REFS", "PARAMETER-REQUIRE-COM-SPEC", "BYTE-VALUES", "CONTEXT-ATOMIC-COMPONENT-REF", "I-PDU-IDENTIFIER-TCP-REF", "DATA-TYPE-MAPPING-REF", "AUTOSAR-PARAMETER-IREF", "PRECONFIGURED-CONFIGURATION-REF", "LOWER-LIMIT", "VT", "OBJECT-DEFINITION-REFS", "CAN-TP-CONNECTION", "SEPARATION-TIME-RESPONSE", "ENCODING", "NM-ACTIVE-COORDINATOR", "DATA-ELEMENT-REF", "DATA-LENGTH-RESPONSE", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF", "CONTEXT-R-PORT-PROTOTYPE-REF", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "SAVE-CONFIGURATION-ENTRY", "SW-COMPARISON-VARIABLES", "MODE-DECLARATION-MAPPING-SET", "MAX-AS", "CAN-CONTROLLER-ATTRIBUTES", "GLOBAL-TIME-DOMAIN-PROPS", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF", "DLT-CONTEXT-REF-CONDITIONAL", "PNC-FILTER-ARRAY-MASK", "CIPHER-SUITE-ID", "FLEXRAY-FIFOS", "PARENT-SHORT-LABEL", "AUTHENTICATION-REF", "DATA-IREF", "SERVICE-KIND", "SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION", "DOMAIN-IDS", "MAXIMUM", "DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF", "OPERATION-IREF", "RESPONSE-ERROR-POSITION", "MODE-DECLARATION-GROUP-REF", "BSW-IMPLEMENTATION", "CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING", "CONTEXT-R-PORT-REF", "TCP-IP-NDP-DELAY-FIRST-PROBE-TIME", "BSW-MODE-SWITCH-EVENT", "LOCAL-UDP-PORT-REF", "STARTUP-CONFIG-SET", "MAXIMUM-SEGMENT-LENGTH-REQUEST", "ACTIVATION-ACTION", "WRITING-PRIORITY", "PORT-GROUPS", "I-SIGNAL-GROUP-REF", "PHM-SUPERVISED-ENTITY-INTERFACE", "P-PORT-PROTOTYPE", "IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL", "REQUIRED-SERVICE-PORT-IREF", "ECUC-CONTAINER-VALUE", "TROUBLE-CODE-J-1939-REF", "TRANSFORMATION-DESCRIPTIONS", "ENTITY-STATUS-MAX-BYTE-FIELD-USE", "AFFECTED-REF", "PERSISTENCY-KEY-VALUE-PAIR", "VERIFY-IREF", "MAX-CYCLES", "ECUC-QUERYS", "TIME-BASE-KIND", "TCP-MAX-RTX", "SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING", "IS-MANDATORY", "SECURITY-EVENT-REF", "ACCESS-COUNT", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "MEMORY-RANGE-REF", "FIBEX-ELEMENTS", "NETWORK-ENDPOINT-REFS", "MIN-RPT-EVENT-ID", "RPT-EXECUTABLE-ENTITY-EVENTS", "EVENT-DEPLOYMENTS", "TARGET-PHM-SUPERVISED-ENTITY-IREF", "ARRAY-SIZE", "ITEM-DEFINITIONS", "IP-SEC-CONFIG-PROPS-REF", "SW-BASE-TYPE", "METHOD-REPLY-TOPIC-NAME", "CONTAINERS", "PORT-INTERFACE-BLUEPRINT-MAPPING", "ALGORITHM-MODE", "MANUFACTURER-CODE", "RECOVERABLE-IN-SAME-OPERATION-CYCLE", "BSW-ENTRY-RELATIONSHIP-SET", "FRAME-MAPPING", "DEPENDENCY-ON-ARTIFACT", "SUPERVISION-MODE", "RESULTING-PROPERTIES", "DATA-TRANSFORMATION", "DISPLAY-FORMAT", "SYNCHRONIZED-TIME-BASE-CONSUMER", "ETHERNET-RAW-DATA-STREAM-GRANT", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "COUNTER-JUMP-DOWN", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING", "I-PDU-REF", "ML-GRAPHIC", "BLUEPRINT-POLICY-LIST", "SWC-BSW-MAPPING-REF", "EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE", "CAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "ASSOCIATED-EVENT-IDENTIFICATION", "ACCESSED-PARAMETER", "SW-ARRAYSIZE", "CALC-RAM-BLOCK-CRC", "ECU-REF", "I-SIGNAL-PORT-REF", "OPTIONAL", "MODE-GROUP", "ECU-SCOPE-REF", "PERSISTENCY-DEPLOYMENT-REF", "PERSISTENCY-FILE-STORAGE", "OBJECT-REF", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS", "DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT", "CALLBACK-HEADER-REF", "FRAGMENT", "SW-RECORD-LAYOUT-V-AXIS", "MSG-ID-MATCH", "DIAGNOSTIC-DATA-IDENTIFIER-REF", "LIN-PID-TO-CAN-ID-MAPPINGS", "NOTIFICATION-STATE", "DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED", "PERSISTENCY-PORT-PROTOTYPE-IREF", "SCALE-CONSTRS", "PROVIDED-SERVICE-INSTANCE-REF", "SECOND-KEY-SLOT-ID", "CHANGE", "TLS-JOB-REQUIREMENT", "MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF", "PAYLOAD-PREAMBLE-INDICATOR", "DLT-LOG-CHANNEL-DESIGN-REF", "ECUC-VALIDATION-CONDS", "SEVERITY", "HANDLE-NEVER-RECEIVED", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "TRACED-FAILURES", "IDSM-MODULE-INSTANTIATION-REF", "TD-HEADER-ID-FILTERS", "REST-INTEGER-PROPERTY-DEF", "FROM-REF", "N-PDU-REFS", "MAJOR-VERSION", "SW-AXIS-GROUPED", "ECUC-ADD-INFO-PARAM-VALUE", "DATA-ELEMENT-REFS", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "NM-NODE-REF", "RUN-MODE", "METHOD-RETURN-REF", "R-PORT-PROTOTYPE-PROPS", "PURPOSE-OF-UPDATE-REF", "RPT-EXECUTABLE-ENTITYS", "MODE-ACCESS-POINT", "TRANSMISSION-START-SEQUENCE-DURATION", "DELAY-COMPENSATION-B", "SECURITY-EVENT-FILTER-CHAIN-REF", "DEF", "META-DATA-ITEM", "OBD-DTC-VALUE", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "SLOT-CAPACITY", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "SOCKET-CONNECTION", "CONSUMED-EVENT-GROUP-REF", "LIN-ORDERED-CONFIGURABLE-FRAMES", "CONSUMED-EVENT-GROUP-REFS", "SENDER-REC-ARRAY-ELEMENT-MAPPING", "ENDPOINTS", "BLOCK-IF-STATE-ACTIVE-AP-IREFS", "UDP-PORT-REF", "EVENT-MAPPINGS", "STATIC-MEMORYS", "BLUEPRINT-CONDITION", "SW-INTENDED-RESOLUTION", "J-1939-NM-NODE", "MCD-IDENTIFIER", "MODE-MAPPING", "SW-AXIS-CONT", "RETURN-VALUE-PROVISION", "NM-MSG-CYCLE-TIME", "STATUS-IREF", "TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED", "DEPENDENT-RESOURCES", "CONDITION-ACCESS", "WAKEUP-TX-IDLE", "TRANSMISSION-DEADLINE", "MAXIMUM-SEGMENT-LENGTH-RESPONSE", "RELEASED-TRIGGERS", "USED-FID-REF", "TAG-ID", "FT", "PERSISTENCY-DEPLOYMENT-ELEMENT-REF", "MACHINE-REF", "DIAGNOSTIC-J-1939-SW-MAPPING", "RX-NM-PDU-REFS", "MAX-VALUE", "CLIENT-ID-DEFINITIONS", "GID-INVALIDITY-PATTERN", "EXTENDED-DATA-RECORDS", "DEFAULT-PATTERN", "TCP-IP-NDP-DEFENSIVE-PROCESSING", "KEY-GENERATION", "CLIENT-SERVER-ARRAY-TYPE-MAPPING", "BYTE-ORDER", "DTC-KIND", "SCOPE-EVENT-REF", "RPT-SYSTEM-REF", "DEPLOYMENT-REF", "LOGICAL-EXPRESSIONS", "FURTHER-ACTION-BYTE-NEEDS", "INTERRUPT-CATEGORY", "CAN-CLUSTER", "BEST-CASE-EXECUTION-TIME", "ACTIVE-SUPERVISION-REFS", "CONFLICTS-TO", "DLT-CONTEXT-REF", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "CHANNEL-NAME", "PRE-ACTIVATE-REFS", "REVISION-LABEL-P-2", "SD-SERVER-EVENT-CONFIG", "CONTEXT-DATA-PROTOTYPE-REF", "TIMING-CONDITION", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "PARTICIPATING-J-1939-CLUSTER-REFS", "EXECUTION-CONTEXT-REF", "NM-COORDINATOR", "MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS", "BUILD-ACTION", "SUPERVISED-ENTITY-ID", "CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE", "CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "LIN-MASTER", "PROVIDED-DDS-EVENT-QOS-PROPS", "CYCLE-STATUS-STORAGE", "POST-BUILD-VALUE-ACCESS-REF", "SUB-CONTAINERS", "KEY-VALUE-STORAGE-REF", "ABSOLUTELY-SCHEDULED-TIMINGS", "SD-SERVER-CONFIG", "TLV-DATA-ID-DEFINITION", "TX-BIT-RATE-SWITCH", "ISSUED-TRIGGERS", "PRE-MAPPING", "DIAG-ARG-INTEGRITY", "EOC-EVENT-REF", "DATA-UPDATE-PERIOD", "SW-IMPL-MAPPINGS", "HW-PIN-GROUPS", "RECEIVED-DATA-REF", "STATUSS", "TRANSFORMATION-PROPS-SET", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "TOL", "SEQUENTIAL-INSTRUCTIONS-END", "CONDITIONS", "NM-DATA-ENABLED", "REQUESTED-MODE-IREF", "TARGET-EEC-REFS", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF", "CAN-ENTER-EXCLUSIVE-AREA-REF", "LISTEN-NOISE", "BSW-OPERATION-INVOKED-EVENT", "LC-STATE-REF", "PERIOD", "PID-SIZE", "PACKING-BYTE-ORDER", "RUNNABLE-ENTITY-IREF", "FINAL-CHECKPOINT-REF", "TOLERANCE", "TP-ECU", "SUPPLIER-VALIDATION-ORDER-REF", "COUPLED-CLUSTER-REF", "SECOND-APPLICATION-ERROR-REF", "NM-CHANNEL-SLEEP-MASTER", "XREF-TARGET", "DECOMPOSITIONS", "RPT-HOOK", "DIAGNOSTIC-EVENT-INTERFACE", "CONSTANT-REF", "DYNAMIC-PARTS", "IDSM-INSTANCES", "I-PV-6-EXT-HEADER-FILTER-LIST", "NM-PNC-PARTICIPATION", "AR-RELEASE-VERSION", "PLCA-TRANSMIT-OPPORTUNITY-TIMER", "TD-EVENT-BSW-MODE-DECLARATION", "ETHERNET-FRAME-TRIGGERING", "DATA-TYPE-MAP", "SERVICE-INSTANCE-PORT-CONFIG", "FLAT-MAP-ENTRY-REF", "NETWORK-ENDPOINTS", "VALUE", "REMOTE-CERTIFICATE-REF", "INITIAL-DELAY-MIN-VALUE", "OFFSET-CORRECTION-JUMP-THRESHOLD", "SOCKET-ADDRESS-REF-CONDITIONAL", "I-SIGNAL-I-PDUS", "OPT-IMPL-RECORD-ELEMENT", "FIELD", "PNC-MAPPING-REF", "SCALE-CONSTR", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS", "PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL", "SUPPORT-BUFFER-LOCKING", "CONTEXT-ROOT-SW-COMPOSITION-REF", "MACROTICK-DURATION", "SOURCE-CHANNEL", "SW-SERVICE-ARG", "WAKEUP-RX-WINDOW", "TROUBLE-CODE-UDS-REF", "IMPLEMENTATION-CONFIG-VARIANT", "VARIATION-POINT", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "IDENTICAL-MAPPING", "TABLE-CAPTION", "SYSTEM-MEMORY-USAGES", "ACCESS-CONTROL", "SW-DATA-DEF-PROPS", "SOCKET-CONNECTION-REF", "ECUC-STRING-PARAM-DEF-VARIANTS", "TIMEOUT-AS", "CLIENT-SERVER-PORTS", "GROUP-ID", "DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL", "ENABLE-TAKE-ADDRESS", "READONLY", "RPT-SUPPORT-DATA", "J-1939-SHARED-ADDRESS-CLUSTERS", "LIN-UNCONDITIONAL-FRAME-REF", "END-ADDRESS", "ECUC-CHOICE-CONTAINER-DEF", "FUNCTION-GROUP-STATE-IREFS", "SOFTWARE-CLUSTER-DESIGN-DEPENDENCY", "PDELAY-REQUEST-PERIOD", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "APPROVAL-STATUS", "VLAN-MEMBERSHIP", "APPLICATION-ENDPOINT-REF-CONDITIONAL", "COM-EVENT-GRANT", "NM-DATA-CYCLE", "REQUIRED-DDS-EVENT-QOS-PROPS", "LOGICAL-ADDRESS", "CLIENT-SERVICE-MAJOR-VERSION", "PROCESS-STATE-IREF", "SAFETY-CONDITIONS", "PARAMETER-ACCESS", "ACL-CONTEXTS", "RAW-DATA-STREAM-INTERFACE", "SUB-NODE-CHANNEL-REF", "SYSC-STRING-REF", "SOURCE-I-PDU-REF", "DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL", "NV-DATAS", "ENABLE-JOIN-ACCESS-CONTROL", "DIAGNOSTIC-ENABLE-CONDITION-REF", "PRE-ACTIVATION-REBOOT", "ETHERNET-RAW-DATA-STREAM-MAPPING-REF", "TCP-IP-IP-REASS-TIMEOUT", "VEHICLE-PACKAGE", "DDS-SECURE-COM-PROPS", "GROUPED-ALIAS-EVENT-REF", "CRYPTO-CERTIFICATE-REF", "RECEIVER-IREF", "REFERENCED-TD-EVENT-VFB-REF", "QOS-PROFILE", "DATA-TRANSFORMATION-REF-CONDITIONAL", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "EXPLICIT-RPT-PROFILE-SELECTION-REF", "QUEUED-RECEIVER-COM-SPEC", "UPDATE-STRATEGY", "FLEXRAY-NM-NODE", "DISTINGUISHED-PARTITIONS", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "DENOMINATOR-GROUP", "SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA", "DIAGNOSTIC-SERVICE-SW-MAPPING", "FUNCTION-INSTANCE", "FRAME-MAPPINGS", "IDENTITY", "DERIVED-FROM-BLUEPRINT-REF", "DIRECT-TP-SDU-REF", "VFC-IREF", "DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT", "WAKEUP-TX-ACTIVE", "SOFTWARE-CLUSTER-DEPENDENCY", "REST-ENDPOINT-PUT", "ETHERNET-PHYSICAL-CHANNEL", "INTERNAL-TRIGGERING-POINT-POLICYS", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "SDG-FOREIGN-REFERENCE", "DEST-META-CLASS", "ACL-OBJECT-CLASSS", "IMPLEMENTATION-DATA-TYPE-REF", "EXECUTABLE-GROUP", "DLT-MESSAGES", "FUNCTION-PROTOTYPE-EMITTER", "ASSOCIATED-COM-I-PDU-GROUP-REFS", "SERIALIZER-REF", "TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP", "ENTRY-MODE-DECLARATION-REF", "CRYPTO-JOB", "TCP-IP-AUTO-IP-INIT-TIMEOUT", "ETH-TP-CONFIG", "RPT-WRITE-ACCESS", "COMPONENT-IMPLEMENTATION-REF", "NM-MSG-REDUCED-TIME", "LOGGING-BEHAVIOR", "N-PDU-REF", "APPLICATION-MODE-IREF", "CSE-CODE", "RPT-SERVICE-POINT-POST-REFS", "IE", "RATE-POLICYS", "TAG-WITH-OPTIONAL-VALUE", "LISTEN-TIMEOUT", "PROVIDER-REF", "INCLUDED-MODE-DECLARATION-GROUP-SET", "CONFLICTS", "I-PDU-TRIGGERING", "TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "DOIP-CONNECTION-REFS", "PORT-API-OPTIONS", "RESOURCE-ESTIMATIONS", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REF", "FIRST-VALUE", "IO-HW-ABSTRACTION-SERVER-ANNOTATIONS", "RECOVERY-VIA-APPLICATION-ACTION", "IKE-RAND-TIME", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "DERIVED-PORT-PROTOTYPE-REF", "RESOURCE-IDENTIFIER-TYPE", "SERVICE-INTERFACE", "HW-ATTRIBUTE-LITERAL-DEF", "DIAGNOSTIC-REQUEST-UPLOAD", "VEHICLE-ROLLOUT-STEP", "DEF-LIST", "TCP-KEEP-ALIVE-ENABLED", "KEEP-ALIVE-TIME", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "ITEM-LABEL", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "MC-DATA-INSTANCE", "ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "OPTIONAL-ELEMENTS", "FRESHNESS-PROPS-REF", "SHORT-NAME", "DID-SIZE", "DIAGNOSTIC-MONITOR-INTERFACE", "OPT-ARGUMENT-REFS", "APPLICATION-PARTITION-REFS", "BSW-DATA-RECEIVED-EVENT", "P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "GENERATOR", "COMPONENTS", "START-INDEX", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "DIAGNOSTIC-J-1939-SPN", "LC-STATES", "OBD-RATIO-SERVICE-NEEDS", "ASSIGNED-DATAS", "SEPARATE-SIGNAL-PATH", "SWC-SERVICE-DEPENDENCY", "INITIAL-CHECKPOINT-REFS", "HAS-SETTER", "ACL-OBJECT-CLASS", "SD-CLIENT-CONFIG-REF", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "SEPARATION-TIME", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION", "IDSM-TRAFFIC-LIMITATION", "TCP-KEEP-ALIVE-TIME", "GID", "FLEXRAY-TP-CONNECTION-CONTROL", "TESTED-ITEM-REF", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF", "SOFTWARE-ACTIVATION-DEPENDENCY-REF", "CONSUMED-SERVICE-INSTANCE-REF", "BSW-MODE-SENDER-POLICY", "DISPLAY-PRESENTATION", "ECU-INSTANCE", "ENTRYS", "SYNC-NODE-MAX", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "MODE-INTERFACE-MAPPING", "MODE-DECLARATION-GROUP-REFS", "BSW-MODULE-ENTRY", "JUMP-TO-BOOT-LOADER", "PNC-GATEWAY-TYPE", "EXPLICIT-RPT-PROFILE-SELECTION-REFS", "SYMBOLIC-NAME-VALUE", "UNIT-GROUP", "PARAMETER-PROVIDE-COM-SPEC", "RESET-CONFIRMED-BIT-ON-OVERFLOW", "SEGMENT-POSITION", "LIN-SLAVE-ECU-REF", "DATA-PROTOTYPE-GROUP-IREFS", "SUPERVISION-MODE-CONDITIONS", "SD-CLIENT-EVENT-TIMING-CONFIG", "REFERENCE-VALUE-REF", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "BSW-INTERNAL-BEHAVIOR", "SW-CLUSTER-MAPPINGS", "CONTROLLER-REF", "REPLICA-PDUS-REFS", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "DO-IP-LOGIC-ADDRESS", "EXECUTABLE", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "MAXIMUM-MESSAGE-LENGTH", "DESTINATION-TYPE", "HW-ATTRIBUTE-DEF", "NM-HW-VOTE-ENABLED", "HEADER-LENGTH", "R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "APPLICATION-INTERFACE", "LIN-MASTER-CONDITIONAL", "MASTER-REF", "DEADLINE-SUPERVISION", "EVENT-SOURCE-REF", "CRC-PRECISE-ORIGIN-TIMESTAMP", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "PNC-VECTOR-OFFSET", "PROFILE-NAME", "REQUIRED-CLIENT-SERVER-ENTRY-REF", "MAX-NUMBER-OF-OCCURRENCES", "IDENTIFIER", "BITFIELD-TEXT-TABLE-MASK-FIRST", "TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE", "DESTINATION-BASE-ID", "IMMEDIATE-RESUME-TIME", "NM-CHANNEL-ID", "MAX-TESTER-CONNECTIONS", "MEASURED-EXECUTION-TIME", "CONSTANT-SPECIFICATION-MAPPING", "TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED", "FEATURE-MODEL-REFS", "NM-DETECTION-LOCK", "EVENT-PROPSS", "TRANSFORMING-SYSTEM-SIGNAL-REF", "TARGET-RTE-EVENT-REF", "CONTEXT-SW-COMPONENT-REF", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL", "MAX-BLOCK-SIZE", "APPL-CONSTANT-REF", "SIGNATURE-SCHEME-REFS", "DEFAULT-VALUE-ELEMENT", "TTCAN-COMMUNICATION-CONTROLLER-VARIANTS", "WRITE", "CHANNELS", "TRANSFORMATION-COM-SPEC-PROPSS", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "MAX-BUFFER-REQUEST", "EVENT-TOPIC-ACCESS-RULE-REF", "GLOBAL-IN-PACKAGE-REF", "TIMESTAMP-FORMAT", "RPT-EXECUTABLE-ENTITY-EVENT", "DATA-PDU-REF", "TD-EVENT-BSW-MODULE", "SUB", "DEFAULT-ROUTER", "INTERNAL-TRIGGER-OCCURRED-EVENT", "DEADLINE-SUPERVISIONS", "SEQUENCE-COUNTER", "INIT-EVENT", "RESOURCE-GROUP", "SUBNET-NUMBER", "CONTEXT-PORT-REF", "LIN-UNCONDITIONAL-FRAME", "MAPPING-CONSTRAINTS", "CAN-CLUSTER-CONDITIONAL", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "FILES", "MSR-QUERY-RESULT-TOPIC-1", "MAX-NUMBER-OF-RESP-PENDING-FRAMES", "ENTRY-REF", "SUB-SOFTWARE-CLUSTER-REF", "P-2-SERVER-MAX", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REF", "REST-OBJECT-REF", "RECOVERY-NOTIFICATION-REF", "CONNECTION-BUNDLES", "LOG-AND-TRACE-INTERFACE", "LOCAL-PORT-REF", "TIME-EXP", "ANALYZED-EXECUTION-TIME", "LOGIC-ADDRESSS", "SIGNATURE-SUPPORT-CP", "INITIAL-REPETITIONS-MAX", "TYPE-MAPPING", "RECORD-VALUE-SPECIFICATION", "CRYPTO-DRIVER", "ARRAY-ELEMENT-MAPPINGS", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "CAN-COMMUNICATION-CONTROLLER-VARIANTS", "SW-ALIGNMENT", "TCP-KEEP-ALIVE-PROBES-MAX", "PORT-CONFIGS", "BINARY-MANIFEST-ITEM", "REDUNDANCY-HANDLINGS", "DIAGNOSTIC-OPERATION-CYCLE-REF", "REQUIRED-TRIGGER-IREF", "OCCURRENCE-COUNTER-PROCESSING", "INITIAL-DELAY-MAX-VALUE", "LIN-CLUSTER", "GLOBAL-TIME-PROPS", "RTE-RESOURCE-ESTIMATION", "REACTION-CODE", "MODE-SWITCH-INTERFACE", "BSW-ENTRY-RELATIONSHIP", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "PER-STATE-TIMEOUTS", "IS-REQUIRED", "REPORT-BEHAVIOR", "RATE-BASED-MONITORED-EVENT-REF", "PROVIDED-DDS-SERVICE-INSTANCE", "AGGREGATION-CONDITION", "NM-PDU", "ERROR-HANDLING", "TIME-FR-IF", "DIAGNOSTIC-CLEAR-CONDITION", "PROVIDED-MODE-GROUPS", "TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "TP-NODES", "DIAGNOSTIC-CONTROL-NEEDS", "CRC-SOURCE-PORT-IDENTITY", "MULTICAST-SEGMENTATION", "MAX-DELTA-COUNTER-INIT", "FLOW-LABEL", "SYSTEM-SIGNAL", "NM-DATA-INFORMATION", "AP-APPLICATION-ERROR-DOMAIN", "ARGUMENT-REF", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "APPLICATION-COMPOSITE-ELEMENT-IREF", "ENABLE-ANYCAST", "SW-TEST-DESC", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "RPT-READS", "RESET-TO-DEFAULT", "PHM-HEALTH-CHANNEL-INTERFACE", "BASE-REF", "DISABLED-MODE-IREFS", "DYNAMIC-PART-ALTERNATIVE", "HEADER-ID-LONG-HEADER", "TCP-IP-DHCP-V-6-INF-DELAY-MIN", "HTTP-ACCEPT-ENCODING", "DATA-PROTOTYPE-GROUP", "ACTIVATION-REASON-REPRESENTATION-REF", "INSTANTIATION-DATA-DEF-PROPS", "LOWER-MULTIPLICITY", "BURST-PATTERN-EVENT-TRIGGERING", "MANAGED-MODE-GROUPS", "SUPPORTED-CONFIG-VARIANTS", "ORIGIN", "HAS-NOTIFIER", "RULE", "FRESHNESS-VALUE-TX-LENGTH", "CRYPTO-SERVICE-JOB-NEEDS", "SDG-TAILORING", "IDSM-PROPERTIES", "TARGET-DATA-PROTOTYPE-IN-CS-REF", "DLT-ARGUMENTS", "MAX-TRCV-DELAY-COMPENSATION-OFFSET", "ALIVE-SUPERVISIONS", "I-SIGNAL-TRIGGERING-REF-CONDITIONAL", "NM-CAR-WAKE-UP-FILTER-NODE-ID", "TYPE-REFERENCE", "COMPUTED", "RPT-PROFILE", "REQUEST-CONFIGURATIONS", "PDU-COUNTER-SIZE", "DIAGNOSTIC-FIM-EVENT-GROUP", "DIAGNOSTIC-SESSION-REF", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "FULL-DUPLEX-ENABLED", "REQUEST-METHOD", "COMPU-DENOMINATOR", "SWC-IMPLEMENTATION", "BLUEPRINT-POLICYS", "SECURE-COM-PROPSS", "MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING", "SECURITY-ACCESS-LEVEL", "NM-NODE-ID-ENABLED", "INPLACE", "BSW-TIMING-EVENT", "SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS", "CRC-FLAGS", "EXTERNAL-TRIGGERING-POINTS", "USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL", "TIMESTAMP-RX-ACCEPTANCE-WINDOW", "HANDLE-OUT-OF-RANGE", "TRACE-REF", "INTER-MESSAGE-TIME", "NAMESPACES", "VARIANT-ID", "TD-EVENT-OPERATION", "V-LAN-REF", "VIEW-MAP", "MODE-DRIVEN-TRANSMISSION-MODE-CONDITION", "IS-EVENT", "SDG-CONSTRAINT-REFS", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "SERVICE-INTERFACE-METHOD-MAPPING", "SW-RECORD-LAYOUT-GROUP-TO", "OFS-DATA-ID-LIST", "SW-ADDRMETHOD-REF", "DISABLED-MODE-IREF", "PDU-ACTIVATION-ROUTING-GROUP", "FIRST-OPERATION-REF", "CLUSTERED-COMPONENT-IREF", "FEATURE-IREF", "PERIOD-END", "PHYSICAL-DIMENSION", "DECODING-CORRECTION", "INITIAL-CHECKPOINT-REF", "FORMAT", "UNIT-REFS", "PHM-CHECKPOINT", "TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "CLASS-CONTENTS", "CRYPTO-PROVIDER-REF", "TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE", "MODE-SWITCH-SENDER-COM-SPEC", "BINARY-MANIFEST-ITEM-DEFINITION", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "MODE-SWITCH-POINT", "ACL-OPERATION-REFS", "TRANSMITTER-REF", "DEFAULT-TRACE-STATE", "FLAT-INSTANCE-DESCRIPTOR", "TIME-TO-LIVE", "SYMBOL-WINDOW", "BSW-PER-INSTANCE-MEMORY-POLICYS", "PRE-SHARED-KEY-REF", "TX-NM-PDU-REF", "ECUC-LINKER-SYMBOL-DEF-CONDITIONAL", "SW-TEXT-PROPS", "VERIFY-IREFS", "NETWORK-IDLE-TIME", "CONSTANT-MAPPING-REF", "PROCESSOR-MODE", "MULTICAST-UDP-PORT", "FRAME-PORT-REF", "PROCESSING-STYLE", "MACHINE-MODE-IREFS", "ASSIGNED-LIN-SLAVE-CONFIG-REF", "CONTROL-PNC-REFS", "SUPPLIER-VALIDATION-ORDER-REFS", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "CAN-ADDRESSING-MODE", "TRACE", "OPERATION-CYCLE-REF", "SW-GENERIC-AXIS-PARAMS", "RESPONSE-ON-EVENT-REF", "OCCURRENCE-EXPRESSION", "ERROR-CONTEXT-REF", "SIZE", "USAGES", "TARGET-MODULE-REFS", "VARIABLE-ACCESS-IREF", "UNICAST-SECURE-COM-PROPS-REFS", "DATA-RECEIVE-ERROR-EVENT", "SERVICE-INTERFACE-EVENT-MAPPING", "IMPLEMENTED-ENTRYS", "DIAGNOSTIC-SERVICE-TABLE-REF", "CHECKPOINTSS", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "SHALL-NOT-RUN-ON-REF", "LET-INTERVAL-REF", "DATA-TYPE-POLICY", "HW-TYPE", "TTCAN-CLUSTER-CONDITIONAL", "HW-PIN-CONNECTIONS", "REFERENCE-REF", "ACL-OBJECT-SET", "PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE", "FRAME-PID", "IMPLEMENTATION-RECORD-ELEMENT-REF", "ENABLE-CONDITIONS", "EVENT-WINDOWS", "COMPOSITION-SW-COMPONENT-TYPE-REF", "VALID-BINDING-TIME", "ALGORITHM-FAMILY", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "DATA-DUMP-ENTRY", "CHECKPOINT-TRANSITION-REF", "CP-SOFTWARE-CLUSTER-REF", "SIGNALS", "IN-PLACE", "FM-FEATURE-MODEL", "TRIGGER-PORT-ANNOTATIONS", "SENDER-INTENT", "EXTERN-OFFSET-CORRECTION", "RECOMMENDED-CONFIGURATION-REFS", "SDG-CLASS", "HW-PIN-GROUP-CONNECTOR", "KEY-EXCHANGE-REFS", "SW-CALIBRATION-NOTESS", "WRITTEN-READ-NV-DATA", "DATA-ELEMENT-IREF", "TP-ADDRESS-REFS", "MODE-PORT-ANNOTATIONS", "OBD-DTC-VALUE-3-BYTE", "NETWORK-REPRESENTATION", "RESPONSE-ON-ALL-REQUEST-SIDS", "SDU-REFS", "CONTEXT-COMPONENT-REF", "SW-RECORD-LAYOUT-COMPONENT", "SOURCE-INSTANCE-IREFS", "ELEMENT-GROUPS", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "UNIT-GROUP-REF", "CHECKPOINT-INITIAL-REF", "TAILORING-REF", "TIME-TRIGGERED-CAN-LEVEL", "INTERVAL-LENGTH", "INPUT-VARIABLE-TYPE-REF", "RATIO-KIND", "OS-TASK-PROXY-REF", "MEM-USAGE", "MAX-DEADLINE", "TCP-IP-DHCP-V-6-SOL-DELAY-MIN", "ALIGNMENT", "DRIVER-PRIMITIVE-REF", "DIAGNOSTIC-EVENT-REF-CONDITIONAL", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "DLT-APPLICATION-REF", "DIAGNOSTIC-TROUBLE-CODE-OBD", "HAS-DYNAMIC-LENGTH", "TARGET-CATEGORY", "MINIMUM-MULTICAST-SEPERATION-TIME", "BLUEPRINT-MAPPING-GUIDE", "RANGE-MAX", "LIN-COMMUNICATION-CONNECTOR", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "BLUEPRINT-MAPPING-SET", "SYNCHRONOUS-SERVER-CALL-POINT", "ROOT-SW-COMPONENT-PROTOTYPE", "CHECKPOINT-IREF", "INPUT", "TRANSFERS", "SECOND-ELEMENT-INSTANCE-IREFS", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "DIAGNOSTIC-FIM-ALIAS-EVENT", "SUPERVISION-MODES", "CALCULATION-FORMULA", "SCOPE-EVENT-REFS", "APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION", "PHYSICAL-DIMENSION-REF", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "DLT-MESSAGE-REFS", "WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST", "TLS-JOB-REQUIREMENT-REF", "PARTS", "CAN-TP-ADDRESS-REF", "HOST-REF", "CONNECTOR-REF", "BUS-MIRROR-CAN-ID-RANGE-MAPPING", "DIAGNOSTIC-COMPONENT-NEEDS", "ASSIGNED-PORTS", "IPV-6-DHCP-SERVER-CONFIGURATION", "IDSM-INSTANCE-ID", "MAC-MULTICAST-GROUP", "UNIT", "PROVIDED-DATAS", "TTCAN-CLUSTER", "UDP-REMOTE-PORT", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "INTERFACE-MAPPING", "TYPE-EMITTER", "PERSISTENCY-REDUNDANCY-HASH", "READ-DATA-CLASS-REF", "BOR-TIME-TX-ENSURED", "ANNOTATIONS", "IP-ADDRESS-PREFIX-LENGTH", "UDP-TP", "ROUTING-GROUP-REF", "SIGNAL-BASED-METHOD-DEPLOYMENT", "PARENT-CATEGORY", "HEADER-TYPE", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "FREEZE-CURRENT-STATE-SUPPORTED", "NETWORK-MASK", "MAPPED-DATA-ELEMENT-IREF", "SUPPORT-INFO", "SOFTWARE-PACKAGE", "APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "PARAMETER-PORT-ANNOTATION", "HW-CATEGORY-REFS", "CLEAR-CONDITION-REF", "EXPIRED-SUPERVISION-CYCLES-TOLERANCE", "SPN", "FIRST-TRIGGER-REF", "ECUC-REFERENCE-DEF", "SOCKET-ADDRESS", "NETWORK-MANAGEMENT-VECTOR-LENGTH", "MAX-EXP-BS", "ENABLE-LIVELINESS-PROTECTION", "WAKE-UP-OVER-BUS-SUPPORTED", "TX-ENABLE-WINDOW-LENGTH", "L-10", "CLIENT-SERVER-OPERATION-IREF", "TRACE-REFS", "NM-CAR-WAKE-UP-BIT-POSITION", "MANAGED-PHYSICAL-CHANNEL-REF", "STORE-CYCLIC", "IP-V-6-PATH-MTU-ENABLED", "DEBOUNCE-ALGORITHM", "CHECK-STATIC-BLOCK-ID", "SOMEIP-TRANSFORMATION-DESCRIPTION", "PDUR-TP-CHUNK-SIZE", "ABBR-NAME", "LITERALS", "FRAME-LENGTH", "CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF", "PADDING-ACTIVATION", "CLIENT-ID-DEFINITION-SET", "SEPARATION-TIME-REQUEST", "PROVIDED-EVENT-GROUPS", "TOOL", "COM-TRIGGER-GRANT-DESIGN", "STATUS-SUB-TLV", "NETWORK-SEGMENT-ID", "USE-MAC-ADDRESS-FOR-IDENTIFICATION", "DEFERRING-FID-REF", "HW-ELEMENT-CONNECTIONS", "PROVIDED-MEMORY-REF", "PARAMETERS", "NO-SUPERVISIONS", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "INITIAL-INACTIVITY-TIME", "BSW-MODULE-CLIENT-SERVER-ENTRY", "TLS-SECURE-COM-PROPS-REF", "REST-ENDPOINT-GET", "ADDRESS-CLAIM-ENABLED", "FLEXRAY-AR-TP-CONFIG", "FIELDS", "SOMEIP-EVENT", "SW-SYSTEMCONSTANT-VALUES", "BSW-SCHEDULABLE-ENTITY", "ARGUMENTS", "DIAGNOSTIC-INDICATOR", "COUNTER-OFFSET", "MODE-TRANSITION", "VALID-BINDING-TIMES", "MC-DATA-INSTANCE-REF", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "INTERFACE-BLUEPRINT-MAPPINGS", "BSW-DIRECT-CALL-POINT", "ADDRESS", "UCM-MODULE-INSTANTIATION-REF", "DEPENDENCY", "TRANSFORMATION-PROPS-REF", "SECURITY-EVENTS", "SWC-BSW-SYNCHRONIZED-TRIGGER", "REQUIRED-FIBEX-ELEMENT-REF", "TD-EVENT-BSW-MODE-DECLARATION-TYPE", "DEFERRING-FID-REFS", "INFRASTRUCTURE-SERVICES", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "DEFAULT-GATEWAY", "LOCAL-IP-SEC-RULE-REF", "TAILORING-REFS", "WRITE-ACCESS", "CUSTOM-SPECIFICATION-REF", "DIAGNOSTIC-SESSION-CONTROL", "TP-PGS", "CPP-IMPLEMENTATION-DATA-TYPE", "APPLIES-TO-DEPENDENCIES", "TRANSLATION-TARGET-IREF", "FLEXRAY-TP-CONFIG", "ABS", "DTC-PROPS-REF", "SAFE-TRANSLATION", "SW-FEATURE-DESC", "SW-VARIABLE-REFS", "HW-ELEMENT-REFS", "BUILD-ACTIONS", "EXPIRED-SUPERVISION-TOLERANCE", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "TCP-IP-DHCP-V-6-CNF-DELAY-MAX", "INCLUDED-LIBRARY-REFS", "MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID", "SUB-TLV-CONFIG", "CORE-ID", "CLIENT-ID-RANGE", "RPT-COMPONENT", "BSW-PER-INSTANCE-MEMORY-POLICY", "UDS-DTC-VALUE", "SENDER-ANNOTATION", "EXECUTION-ORDER-CONSTRAINT", "TWO-KEY-SLOT-MODE", "SYNC-JUMP-WIDTH", "CONTAINED-FIBEX-ELEMENT-REF", "PROCESS-TO-MACHINE-MAPPING", "REKEY-INTERVAL", "FIRST-TO-SECOND-DATA-TRANSFORMATION-REF", "SYMBOL-PROPS", "MAX-NUMBER-OF-PARALLEL-TRANSFERS", "P-2-MAX", "MAC-LAYER-TYPE", "FORMULA", "SW-COMPONENT-DOCUMENTATION", "COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE", "BUILD-ACTION-MANIFEST-REF", "TIME-INTERVAL", "DIAGNOSTIC-TEST-RESULT", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "SW-CLASS-PROTOTYPE-REF", "MODE-GROUP-IREF", "SEPARATION-CYCLE-EXPONENT", "APPLICATION-TYPE-REF", "TYPE-TREF", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "DATA-RECEIVE-POINTS", "SYNCHRONIZATION-TIMING-CONSTRAINT", "L-4", "SEC-OC-CRYPTO-SERVICE-MAPPING", "MIN-OK-STATE-INVALID", "TD-EVENT-TT-CAN-CYCLE-START", "DIAGNOSTIC-PROPS", "TIMING-GUARANTEES", "PORT-BLUEPRINT", "RPT-READ-ACCESS", "DEBOUNCE-TIME", "OPERATION-ARGUMENT-INSTANCE-IREF", "ALIAS-EVENT-REF", "ROOT-CONTEXT-REF", "PID", "SWC-BEHAVIOR-REF", "RETURN-TYPE", "NM-BUSLOAD-REDUCTION-ACTIVE", "DATE", "ACTIVATION-POINTS", "NV-BLOCK-DESCRIPTORS", "MASTER", "DIAGNOSTIC-ECU-RESET-CLASS", "TCP-FAST-RETRANSMIT-ENABLED", "GENERIC-MODULE-INSTANTIATION", "BINARY-MANIFEST-ITEM-NUMERICAL-VALUE", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS", "BIT-POSITION", "SHUTDOWN", "PROCESS-DESIGN-REF", "METHOD-MAPPING", "FIELD-MAPPING", "SECOND-VALUE", "SECURE-TRANSLATION", "LOCAL-TCP-PORT-REF", "PER-INSTANCE-MEMORYS", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS", "TLV-DATA-ID-DEFINITION-SET", "DYNAMIC-ACTION-REFS", "ACL-OPERATION-REF", "DATA-PROTOTYPE-TRANSFORMATION-PROPSS", "REMOTE-PORT-REF", "CODE", "MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "SEND-INDICATION", "J-1939-DCM-DM-19-SUPPORT", "GETTER-CALL-REF", "EXECUTION-ERROR", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS", "STORE-IMMEDIATE", "APPLICATION-ENDPOINT", "AGING-REF", "CAT-2-ISR", "GENERIC-MATH", "TRAFFIC-CLASS", "NM-REMOTE-SLEEP-INDICATION-TIME", "ECU-EXTRACT-REFERENCE-IREF", "ERROR-MAPPINGS", "CHANNEL-MODE", "USER-DEFINED-ETHERNET-FRAME", "PROVIEDE-DATA-REF", "J-1939-CONTROLLER-APPLICATION-REF", "SW-GENERIC-AXIS-PARAM", "APPLICATION-RECORD-DATA-TYPE", "ASSIGNED-TRAFFIC-CLASSS", "MIN-OK-STATE-INIT", "RX-IDENTIFIER-RANGE", "INTERPOLATION-ROUTINE-MAPPING-SET-REF", "FLEXRAY-COMMUNICATION-CONNECTOR", "STATIC-PART", "INTEGER-VALUE-VARIATION-POINT", "SECURE-COMMUNICATION-DEPLOYMENTS", "CURRENT-VALUE-REF", "MIN-NUMBER-OF-OCCURRENCES", "PHYSICAL-CHANNEL-REFS", "DOCUMENT-ELEMENT-SCOPE", "DEFAULT-APPLICATION-TIMEOUT", "RELEASE-NOTES-REF", "RPT-EVENT-ID", "BLUEPRINT-MAPS", "SEGMENT-LENGTH", "DATA-EXCHANGE-POINT", "AH-CIPHER-SUITE-NAMES", "STEP-SIZE", "BSW-TRIGGER-REF", "SOURCE-EEC-REF", "OFFER-CYCLIC-DELAY", "REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF", "PORT-BLUEPRINT-MAPPING", "TRANSFORMER-CHAIN-REF", "TIME-SYNC-SERVER", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF", "CLIENT-COM-SPEC", "SW-CLUSTER-REF", "UDP-NM-CLUSTER-COUPLING", "USER-DEFINED-CLUSTER-VARIANTS", "TRANSMIT-PDU-REF", "FIRST-MODE-REF", "DIAG-FAULT-DETECTION-COUNTER-PORT-REF", "AUXILIARY-FIELD-DEFINITIONS", "TOPIC-1", "DLT-SESSION-ID", "SEED-SIZE", "PN-RESET-TIME", "APPLICATION-PRIMITIVE-DATA-TYPE", "DEBOUNCE-BEHAVIOR", "READ-CLASS-REF", "DATA-TRANSFORMATION-SET", "PDU-COUNTERS", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "DATA-MAPPINGS", "FM-FEATURE-MAP-ASSERTION", "ALLOW-DYNAMIC-L-SDU-LENGTH", "COND", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "PARAMETER-DATA-PROTOTYPE", "POST-BUILD-VARIANT-VALUE", "INTERNAL-BEHAVIORS", "P-2-TIMING", "SOURCE-CAN-ID-MASK", "CALIBRATION-PARAMETER-VALUES", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "DIAGNOSTIC-MEMORY-IDENTIFIER", "PDU-COLLECTION-TIMEOUT", "PORT-NUMBER", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "TYPE-TAILORINGS", "FM-FEATURE-RELATION", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "SA-OVER-TIME", "BASE-REFERENCE-REF", "AP-SOMEIP-TRANSFORMATION-PROPS", "TRANSFER-PROPERTY", "WATCHDOG-ACTION-ITEM", "NUMBER", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF", "OBJECT-DEFINTION-REFS", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "NEIGHBOR-CACHE-SIZE", "LOCAL-SUPERVISION-ENTITY-REF", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "FIBEX-ELEMENT-REF-CONDITIONAL", "IUMPR-GROUP", "IDS-COMMON-ELEMENT-REF-CONDITIONAL", "PERSISTENT-STORAGE", "PATTERN-JITTER", "LOCAL-SUPERVISION-REFS", "HW-PIN-GROUP-REFS", "IP-V-6-ADDRESS", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "REQUEST-2-SUPPORT", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF", "PAYLOAD-REF", "TRANCEIVER-STANDBY-DELAY", "TCP-IP-ICMP-V-6-HOP-LIMIT", "TIMING-VARIABLE-REF", "EVENTS", "REQUEST-DOWNLOAD-CLASS-REF", "IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF", "TOLERATED-FAILED-CYCLES", "TCP-KEEP-ALIVE-INTERVAL", "ECUC-DESTINATION-URI-DEF", "RATE-CORRECTION-OUT", "DROP-NOT-REQUESTED-NAD", "TIME-CS", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "CRYPTO-PROVIDER-INTERFACE", "INITIAL-REF-OFFSET", "BIT-NUMBER", "DATA-ID-MODE", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF", "SW-SYSTEMCONSTANT-VALUE-SET", "SECURITY-DELAY-TIME", "I-SIGNAL-I-PDU-REF-CONDITIONAL", "DIAGNOSTIC-DATA-TRANSFER", "ECUC-IMPLEMENTATION-CONFIGURATION-CLASS", "BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE", "TP-ADDRESS-REF", "TARGET-OPERATION-REF", "DATA-OFFSET", "TLV-DATA-ID-DEFINITION-REFS", "DLT-ARGUMENT-ENTRYS", "SUB-GROUP-REF", "SUBSTITUTED-FRAME-REF", "PHYSICAL-PROPS", "BY-PASS-POINT-IREFS", "CRC-MESSAGE-LENGTH", "RUNS-INSIDE-EXCLUSIVE-AREA-REF", "TIMEOUT-BS", "VALUE-IREF", "DEPENDS-ON-REF", "FIELD-REF", "PORT-API-OPTION", "WRITE-CLASS-REF", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "ROLE", "BSW-MODULE-DEPENDENCYS", "KEY-ID", "VALUE-RESTRICTION", "FLEXRAY-TP-CONNECTION", "CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST", "BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR", "TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS", "SLEEP-MODE-SUPPORTED", "E-2-E-PROFILE-CONFIGURATION", "ECUC-CHOICE-REFERENCE-DEF", "ALIVE-REFERENCE-CYCLE", "SUB-NODE-NUMBER", "SECURE-COM-PROPS-REF", "COUNTER-JUMP-UP", "ON-BOARD-MONITOR-ID", "SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS", "EVENT-COMBINATION-REPORTING-BEHAVIOR", "TCP-TP-PORT", "TARGET-DATA-PROTOTYPE-GROUP-REF", "GLOBAL-TIME-CAN-MASTER", "DDS-EVENT-DEPLOYMENT", "SECOND-ELEMENT-REF", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "NM-ECU", "MODULE-INSTANTIATION-REF", "RECOMMENDED-CONFIGURATION-REF", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "SESSION-ID-SUPPORT", "SIGNIFICANCE", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "BINARY-MANIFEST-META-DATA-FIELD", "TCP-IP-DHCP-V-6-SOL-DELAY-MAX", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "SECOND-PHYSICAL-DIMENSION-REF", "VENDOR-SPECIFIC-SERVICE-NEEDS", "ARBITRATIONS", "FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL", "MIN-OK-STATE-VALID", "CONSTANT-MAPPING-REFS", "PROCESS-ARGUMENT", "TD-EVENT-TRIGGER-TYPE", "CPU-USAGE", "COMM-CONTROLLERS", "MASTER-TO-SLAVE-EVENT-MAPPINGS", "MAXIMUM-RECEIVE-BUFFER-LENGTH", "P-2-STAR-SERVER-MAX", "SDG-CLASSES", "TD-EVENT-MODE-DECLARATION-TYPE", "DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL", "DEF-ITEM", "CONTENT-URI", "DIAGNOSTIC-CONNECTED-INDICATOR", "CLIENT-SERVER-INTERFACE-MAPPING", "MULTICAST-SD-IP-ADDRESS-REF", "MAX-NUMBER-OF-TIME-QUANTA-PER-BIT", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "BANDWIDTH-LIMITATION", "PERSISTENCY-FILE-ELEMENT", "CLEAR-EVENT-ALLOWED-BEHAVIOR", "FRAME-TRIGGERING-REF", "SECOND-DATA-PROTOTYPE-REF", "MACRO-INITIAL-OFFSET-A", "SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "META-DATA-ITEMS", "MODE-DEPENDENT-STARTUP-CONFIG", "DIAGNOSTIC-VALUE-NEEDS", "MACHINE-DESIGN-REF", "I-SIGNAL", "END-TO-END-PROTECTIONS", "NETWORK-INTERFACE-REF", "TIMING-ARGUMENT-REF", "SCOPE", "CHECKPOINT-FINAL-REFS", "DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL", "STACK-USAGES", "DYNAMIC-BS", "IDENT", "FOREIGN-MODEL-REFERENCE", "TT", "SEC-OC-JOB-REQUIREMENT", "EXIT-TIMEOUT-VALUE", "ECUC-LINKER-SYMBOL-DEF-VARIANTS", "VARIATION-POINT-PROXYS", "KEY-SLOT-REF", "VIN-INVALIDITY-PATTERN", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "VTF", "N-DATA-SETS", "OPERATION-MAPPINGS", "TARGET-PHM-CHECKPOINT-REF", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "AUDIENCE", "NM-COORDINATOR-ROLE", "GENERAL-PURPOSE-I-PDU", "PNC-FILTER-DATA-MASK", "NETWORK-INTERFACE-ID", "IS-PRIVATE", "NUMBER-OF-NOTIFIER-SETS", "ALLOW-MASTER-RATE-CORRECTION", "CRYPTO-SERVICE-QUEUE", "DATA-RECEIVE-POINT-BY-ARGUMENTS", "PROCESS-MODE-MACHINE", "MULTICAST-SECURE-COM-PROPS-REF", "NM-VECTOR-EARLY-UPDATE", "MINIMUM-INTERVAL-LENGTH", "BSW-SERVICE-DEPENDENCY-REF", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "TIMING-REQUIREMENTS", "RAPID-PROTOTYPING-SCENARIO", "BUS-MIRROR-CHANNEL-MAPPING-IP", "REMOTE-ID", "INHIBITING-SECONDARY-FID-REF", "REQUESTOR-REFS", "SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION", "APPLICATION-PARTITION-REF", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "DTC-SETTING-CLASS-REF", "IP-V-4-ADDRESS", "NM-MAIN-FUNCTION-ACROSS-FR-CYCLE", "ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL", "SERVICE-INTERFACE-ELEMENT-MAPPING-REFS", "DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF", "MINIMUM-SEND-INTERVAL", "UCM-PROCESSINGS", "DO-IP-TP-CONNECTION", "BSW-MODULE-ENTRY-REF-CONDITIONAL", "ACTION-LIST", "SW-RECORD-LAYOUT-GROUP-FROM", "SYMBOL", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "TCP-OPTION-FILTER-LIST", "RAW-DATA-STREAM-CLIENT-INTERFACE", "IS-OPTIONAL", "RAW-DATA-STREAM-SERVER-INTERFACE", "ACTUAL-EVENT-REF", "TRANSMISSION-MODE-DECLARATION", "DERIVED-OBJECT-REF", "USES-MONITOR-DATA", "SIZE-OF-UNION-LENGTH-FIELD", "SOCKET-CONNECTION-BUNDLE", "PUBLISHER", "NOTATION", "TP-CHANNEL-REF", "TARGET-MODULE-ID", "MODE-DECLARATION-GROUP-PROTOTYPE", "PR-PORT-PROTOTYPE", "OPT-ARGUMENT-REF", "POST-BUILD-VARIANT-SUPPORT", "SW-AXIS-TYPE-REF", "PARAMETER-INTERFACE", "MAX-NUMBER-OF-NPDU-PER-CYCLE", "SWC-MAPPING-CONSTRAINT-REFS", "ECUC-INSTANCE-REFERENCE-VALUE", "SOMEIP-SERVICE-DISCOVERY", "FIRST-MODE-GROUP-REF", "PROVIDED-ENTRYS", "PORT-PROTOTYPE-REF-CONDITIONAL", "SEC-OC-JOB-SEMANTIC", "SW-AXIS-INDEX", "INITIAL-MODE-REF", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "OPERATIONS", "CLIENT-PORT-REF", "USAGE", "REQUEST-EMISSION-RELATED-DTC-CLASS-REF", "MAPPED-OPERATION-IREF", "TD-EVENT-FR-CLUSTER-CYCLE-START", "OBD-INFO-SERVICE-NEEDS", "PROCESSOR", "ABSTRACT-CLASS-TAILORING", "ETHERNET-COMMUNICATION-CONNECTOR", "ECU-TASK-PROXY-REF", "REGENERATED-PRIORITY", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "LOCAL-SUPERVISION-ENTITY-REFS", "SW-CLUSTER-REFS", "WATCHDOG-PHM-ACTION-ITEM", "MAPPED-BSW-SERVICE-DEPENDENCY-REF", "HANDLE-DATA-STATUS", "DEFAULT-LC-STATE-REF", "TRCV-DELAY-COMPENSATION-OFFSET", "RUNNABLE-MAPPINGS", "OPERATION-CYCLE-AUTOSTART", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS", "WITH-AUTO", "POST-BUILD-VARIANT-CRITERION", "CRYPTO-KEY-SLOT-REFS", "NM-NETWORK-HANDLE", "SOFTWARE-CLUSTER-REF", "INCLUDED-LIBRARY-REF", "SEGMENT-BYTE-ORDER", "CONTEXT-DATA-PROTOTYPE-IN-SR-REF", "TARGET-P-PORT-PROTOTYPE-REF", "APPLICATION-ARRAY-ELEMENT-REF", "RTE-EVENT-IREF", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "IS-EXTERNAL", "COUNTER-INCREMENT-STEP-SIZE", "USER-DEFINED-GLOBAL-TIME-SLAVE", "ACL-ROLE-REFS", "ALGORITHM-SECONDARY-FAMILY", "SHORT-LABEL", "SECURE-COM-PROPS-FOR-UDP-REF", "SENDER-REC-RECORD-TYPE-MAPPING", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "HW-ATTRIBUTE-VALUE", "LABEL", "HW-ELEMENT-CONNECTOR", "CLIENT-PORT-FROM-CONNECTION-REQUEST", "EVALUATED-VARIANT-REF", "GLOBAL-TIME-ETH-MASTER", "SECOND-ELEMENT-INSTANCE-IREF", "REQUIRED-COM-SPECS", "TRIGGER-REFS", "SOURCE-EEC-REFS", "ALIAS-NAME-ASSIGNMENT", "TX-MASK", "FR-GLOBAL-TIME-DOMAIN-PROPS", "SCHEDULER-NAME-PREFIX-REF", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "NETWORK-TIME-MASTER-REF", "APPLICATION-ENDPOINT-REF", "DIAGNOSTIC-EVENT", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "INDEX", "ACCESSED-VARIABLE-REF", "NUMERICAL-VALUE", "SDG-PRIMITIVE-ATTRIBUTE", "TD-EVENT-SERVICE-INSTANCE-FIELD", "IPV-4-MULTICAST-IP-ADDRESS", "UNICAST-NETWORK-ENDPOINT-REF", "ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS", "ADDRESS-RANGE-LOWER-BOUND", "RULE-INIT-STATE", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "DO-IP-TESTER-ROUTING-ACTIVATION-REF", "UCM-MASTER-FALLBACK-REFS", "BLUEPRINT-POLICY-SINGLE", "HW-PIN-CONNECTOR", "RESTRICTION", "NUMERICAL-RULE-BASED-VALUE-SPECIFICATION", "HEADER-ID-SHORT-HEADER", "COMPONENT-CLUSTERING", "FEATURE-REF", "SWC-INTERNAL-BEHAVIOR", "CALL-POINTS", "READ-NV-DATA", "USE-SECURED-PDU-HEADER", "SIGNAL-I-PDU-REPLICATION", "TCP-ROLE", "PARAMETER-ACCESSED-FOR-DEBUG-REF", "LIN-TP-N-SDU-REF", "LOGICAL-EXPRESSION-ARGUMENT-REFS", "TP-ECUS", "USED-CODE-GENERATOR", "CORES", "BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING", "PHM-ACTION", "DIAGNOSTIC-ENV-CONDITION-FORMULA", "PARTITION-REF", "PSK-IDENTITY", "ROUTINE-CONTROL-CLASS-REF", "TARGET-MODE-REF", "SOURCE-LIN-PID-REF", "DATA-WRITE-ACCESSS", "CONTROLLED-DATA-ELEMENT-REF", "COMPOSITE-NETWORK-REPRESENTATION", "UPPER-LIMIT", "RUN-RESOURCE", "NM-DATA-DISABLED", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "HOST-SYSTEM-REF", "SWC-MODE-MANAGER-ERROR-EVENT", "BUILD-ACTION-MANIFEST", "SW-MAINTENANCE-NOTESS", "ROOT-VARIABLE-DATA-PROTOTYPE-REF", "DIAGNOSTIC-TROUBLE-CODE-REF", "CONSIDER-PTO-STATUS", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "POLICY", "PDU-REPLICATION-VOTING", "SD-SERVER-CONFIG-REF", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "FUNCTIONAL-UNIT", "MAXIMUM-EXECUTION-TIME", "SERVICE-ITEMS", "NM-MESSAGE-TIMEOUT-TIME", "ECU-PARTITION", "SW-REFRESH-TIMING", "LIN-SLAVE-CONDITIONAL", "BSW-MGR-NEEDS", "RPT-PREPARATION-LEVEL", "REQUESTS", "SOURCE-INSTANCE-IREF", "SERVER-ARGUMENT-IMPL-POLICY", "SIZE-OF-STRING-LENGTH-FIELDS", "ROUTINE-INFO", "AUTHENTIC-CONNECTION-PROPSS", "ALIVE-CHECK-RESPONSE-TIMEOUT", "OFFSET-TIMING-CONSTRAINT", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "UDP-MIN-TX-BUFFER-SIZE", "REQUEST-RESULT", "DESCRIPTION", "DIAGNOSTIC-CONTRIBUTION-SET", "OPERATION-REF", "EXCLUSIVE-AREA-REF", "CONSUMED-EVENT-GROUP", "TLV-SUB-ELEMENT-REF", "NODE-PORTS", "NETWORK-ID", "PSK-IDENTITY-HINT", "EVENT-OBD-READINESS-GROUP", "RUNNABLE-ENTITY-GROUP", "METHOD-REFS", "WAKEUP-LOCAL-DURATION-TIME", "FLEXRAY-FIFO-RANGE", "PIN-NUMBER", "BSW-DATA-SEND-POLICY", "SW-CALIBRATION-ACCESS", "TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED", "EXTERNAL-CLOCK-SYNCHRONISATION", "FIRST-APPLICATION-ERROR-REF", "DATA-SEND-POINTS", "MODULE-REFS", "UNIT-AND-SCALING-ID", "SOFTWARE-CLUSTER", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "MAX-ERROR-STATE-INVALID", "UDP-CHECKSUM-HANDLING", "STARTUP-OPTION", "SUPPORTED-CIPHER-SUITES", "FOLLOW-UP-ACTION-REFS", "IPV-4-DHCP-SERVER-CONFIGURATION", "ALLOWED-I-PV-6-EXT-HEADERS-REF", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "SW-AXIS-TYPE", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "TD-EVENT-I-SIGNAL", "FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING", "BASIC-CYCLE-LENGTH", "SW-CALPRM-AXIS", "PN-RESET-TIMER", "TIMING-CONDITION-REF", "CRYPTO-SERVICE-CERTIFICATE-REF", "DIAGNOSTIC-INDICATOR-INTERFACE", "CONTAINED-FIBEX-ELEMENT-REFS", "FIXED-LENGTH", "INDEXED-ARRAY-ELEMENT", "SLEEP-MODE-EXECUTION-DELAY", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "TARGET-TRIGGER-REF", "TD-EVENT-BSW-MODULE-TYPE", "PORT-GROUP", "SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "ARTIFACT-CHECKSUM-REFS", "ORDERED-MASTER", "ATTRIBUTES", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "FUNCTION-IDENTIFIER-REF", "VEHICLE-SYSTEM", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "ACCESS-COUNT-SETS", "PROVIDED-SERVICE-INSTANCES", "ASSIGNED-DATA-TYPES", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "SUPPORTS-MULTIPLE-NOTIFIER-SETS", "PERSISTENCY-DATA-PROVIDED-COM-SPEC", "FM-ATTRIBUTE-VALUE", "INFORMAL-FORMULA", "LANGUAGE", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "MAX-DATA-LENGTH", "EVENT-NEEDS-REF", "MICRO-PER-CYCLE", "USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS", "PROCESS-PHM-ACTION-ITEM", "SDG-CONSTRAINT-REF", "COM-CONFIGURATION-TX-TIME-BASE", "PNC-FILTER-ARRAY-MASKS", "RPT-COMPONENTS", "TCP-DELAYED-ACK-TIMEOUT", "N-PDU", "RPT-SERVICE-POINTS", "BSW-ENTRY-KIND", "ELEMENT-MAPPINGS", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "CONTEXT-DATAS", "DO-IP-ENTITY-ROLE", "OPTION-KIND", "LOC-MEASUREMENT-SET", "TD-EVENT-TYPE", "BEHAVIOR", "SW-GENERIC-AXIS-PARAM-TYPE-REF", "TRANSFORMER-REF", "MC-DATA-INSTANCE-VAR-REF", "EID", "USE-FRESHNESS-TIMESTAMP", "RAM-LOCATION-REF", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "SENDER-RECEIVER-TO-SIGNAL-MAPPING", "COMPILERS", "SEC-OC-DEPLOYMENT", "REVISION-LABEL-P-1", "PERIODIC-RESPONSE-UUDT-REF", "IS-REENTRANT", "TLS-JOB-MAPPING", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "ETHERNET-PRIORITY-REGENERATIONS", "E-2-E-PROFILE-COMPATIBILITY-PROPS-REF", "SEC-OC-JOB-REQUIREMENT-REF", "STRING-ENCODING", "SECOND-PORT-REF", "PHYSICAL-CHANNELS", "TLV-DATA-ID-DEFINITION-REF", "TCP-TP", "NONQUEUED-RECEIVER-COM-SPEC", "SOMEIP-SERVICE-DISCOVERY-PORT", "BSW-ENTRY-RELATIONSHIPS", "FM-ATTRIBUTE-DEF", "NETWORK-CONFIGURATIONS", "BUFFER-COMPUTATION", "MAPPED-AP-DATA-ELEMENT-IREF", "SERVICE-DISCOVERY-CONFIGS", "DID-NUMBER", "ECUC-MULTIPLICITY-CONFIGURATION-CLASS", "PROVIDED-PORT-PROTOTYPE-IREF", "PHM-CONTRIBUTION-REF", "MACHINE", "SAFETY-MARGIN", "APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION", "STRUCTURED-REQ", "COLLECTED-INSTANCE-IREF", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "MODE-SWITCHED-ACK", "CONNECTOR-REFS", "DIRECTION", "REFINED-EVENT-IREF", "SOCKET-CONNECTION-IPDU-IDENTIFIER", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "ATTRIBUTE-DEFS", "MAPPED-SECURITY-EVENTS", "SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "UDP-COLLECTION-BUFFER-TIMEOUT", "KEY-EXCHANGE-REF", "INVALID-VALUE", "ACTIVATE-AT-START", "IPV-6-CONFIGURATION", "AUTOSAR-VARIABLE", "REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF", "MEMORY-LOW-ADDRESS-LABEL", "SESSION-HANDLING-SR", "MINIMUM-RX-CONTAINER-QUEUE-SIZE", "SHARED-PARAMETERS", "LOCAL-SUPERVISIONS", "TCP-NAGLE-ENABLED", "MIN-SYNC-JUMP-WIDTH", "CONTEXT-DATA-PROTOTYPE-REFS", "DIAGNOSTIC-SESSION", "PROCESS-EXECUTION-ERROR", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REF", "TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE", "UCM-MODULE-INSTANTIATION", "CRYPTO-ALG-ID", "USER-DEFINED-COMMUNICATION-CONNECTOR", "DIAGNOSTIC-ROUTINE-NEEDS", "MODULE-REF", "NETWORK-CONFIGURATION", "RAM-BLOCK-STATUS-CONTROL", "BINARY-MANIFEST-ITEM-POINTER-VALUE", "MACHINE-MODE-IREF", "MULTICAST-CONNECTOR-REF", "BURST-SIZE-REQUEST", "SECURED-AREA-LENGTH", "MEMORY-HIGH-ADDRESS", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "TIME-FAILED-THRESHOLD", "PHM-RULE", "SWC-TO-ECU-MAPPING", "ASSIGNED-TRAFFIC-CLASS", "AUTOSAR-VARIABLE-IREF", "TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "N", "MODE-DECLARATIONS", "SW-CLASS-ATTR-INSTANCE-IMPL", "DERIVED-INTERFACE-REF", "ECU-RESET-CLASS-REF", "DIAG-REQUIREMENT", "TIME-SYNC-SERVER-REF", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "I-PDU-MAPPINGS", "PDU-COUNTER-THRESHOLD", "LIFE-CYCLE-INFO", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS", "SIZE-OF-ARRAY-LENGTH-FIELDS", "FLEXRAY-FIFO-CONFIGURATION", "NODE-NAME", "TEST-IDENTIFIER", "CONTROLLED-HW-ELEMENT-REF", "RELEASED-TRIGGER-REF", "SIZE-OF-UNION-TYPE-SELECTOR-FIELD", "DIAGNOSTIC-J-1939-NODE", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "DATA-READ-ACCESSS", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "REQUEST-SEED-ID", "FULLY-QUALIFIED-DOMAIN-NAME", "PRM-UNIT", "DOCUMENTATION-CONTENT", "RECORD-NUMBER", "MEMORY-LOW-ADDRESS", "CLUSTERED-COMPONENT-IREFS", "CONTEXT-ROOT-COMPONENT-REF", "FUNCTION-GROUP-SET", "TIME-SYNC-CORRECTION", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "TLS-CRYPTO-MAPPING-REF", "VLAN-MODIFIER-REF", "LIN-TP-CONNECTION", "TARGET-EVENT-REF", "PORTS", "VALUE-DATA-TYPE-REF", "CONSTRAINT-TAILORINGS", "TIME-LEAP-FUTURE-THRESHOLD", "HARDWARE-TEST-NEEDS", "ROLLOUT-QUALIFICATIONS", "EXPRESSION-REF", "UCM-STEP", "MAPPING-SCOPE", "SIZE-OF-STRING-LENGTH-FIELD", "OBD-RATIO-DENOMINATOR-NEEDS", "SIGNAL-BASED-EVENT-DEPLOYMENT", "DATA-PROTOTYPE-IN-PORT-INTERFACE-REF", "SOFTWARE-CLUSTER-DEPENDENCY-FORMULA", "COM-FIND-SERVICE-GRANT", "DEFINITION-REF", "RUNTIME-ERROR", "SW-COMPOSITIONS", "J-1939-CLUSTER-VARIANTS", "NETWORK-TIME-PROVIDER-REF", "DYNAMIC-ARRAY-SIZE-PROFILE", "STORE-AT-SHUTDOWN", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "LOG-TRACE-LOG-MODES", "SOMEIP-METHOD-PROPS", "SETTER-RETURN-SIGNAL-REF", "SDGS", "TIMING-EVENT-REF", "VARIABLE-LENGTH", "J-1939-CONTROLLER-APPLICATION", "DATA-PROVIDER", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "EVENT-SYMBOL-NAME", "ACCEPTS-ENCODINGS", "ALLOW-PASSIVE-TO-ACTIVE", "NM-BUS-LOAD-REDUCTION-ENABLED", "SENSOR-ACTUATOR-REF", "FMI", "CONTAINED-I-PDU-PROPS", "END-TO-END-CALL-RESPONSE-TIMEOUT", "GLOBAL-TIME-CORRECTION-PROPS", "COM-OFFER-SERVICE-GRANT", "PNC-WAKEUP-DLC", "SW-COMPONENT-DOCUMENTATIONS", "SUB-DOMAIN-REF", "ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT", "BIT-OFFSET", "UDP-COLLECTION-TRIGGER", "CONTROLLED-DATA-ELEMENT-REFS", "NEEDS-ORIGINAL-DATA", "AGGREGATION-TAILORING", "PATH-MTU-TIMEOUT", "SHORT-NAME-FRAGMENT", "TARGET-POSSIBLE-ERROR-REF", "COLLECTION-REF", "BSW-TRIGGER-DIRECT-IMPLEMENTATION", "COMM-CONNECTORS", "BIT", "SECURITY-EVENT-FILTER-CHAIN", "ENABLE-CONDITION-REF", "REPLICA-PDUS-REF", "RPT-PROFILES", "COLLISION-RESOLVING-SCHEDULE-REF", "ESP-CIPHER-SUITE-NAMES", "PROCESSING-UNIT-REF", "TRANSPORT-PROTOCOL", "CONTEXT-DATA-PROTOTYPE-IN-CS-REF", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "FIELD-IREF", "CREATE-ECU-WAKEUP-SOURCE", "TCP-FAST-RECOVERY-ENABLED", "GETTER-RETURN-SIGNAL-REF", "TYPE-REFERENCE-REF", "LABELED-ITEM", "DIAGNOSTIC-CONTROL-DTC-SETTING", "APPLICATION-DEFERRED-DATA-TYPE", "KEY-SLOT-CONTENT-ALLOWED-USAGES", "SYNC-TIME-BASE-MGR-USER-NEEDS", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "API-SERVICE-PREFIX", "FLAT-MAP", "TIME-BASE-RESOURCE-REF-CONDITIONAL", "TRIGGER-REF-CONDITIONAL", "ALL-CHANNELS-REFS", "TCP-IP-DHCP-V-6-CNF-DELAY-MIN", "FIELD-GET-SET-QOS-PROPSS", "AREA", "PARAMETER-PORT-ANNOTATIONS", "L-5", "MEM-ALIGNMENT", "REQUIRED-GENERATOR-TOOLS", "STREAM-IDENTIFIER", "DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF", "DO-IP-NETWORK-CONFIGURATION", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS", "BASE-TYPE-SIZE", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF", "PNC-PDUR-GROUP-REFS", "RUNTIME-IP-ADDRESS-CONFIGURATION", "UDP-TP-PORT", "PARAMETER-ID", "TEXT", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "DELAY-COMPENSATION-A", "SEGMENT-POSITIONS", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "FM-SYSCOND", "USED-PARAMETER-ELEMENT", "TCP-IP-NDP-DEFAULT-RETRANS-TIMER", "ETHERNET-PRIORITY-REGENERATION", "HEALTH-CHANNEL-ID", "IP-SEC-CONFIG-PROPS", "CAN-ID-RANGE-MAPPINGS", "STATE-DEPENDENT-STARTUP-CONFIGS", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF", "DATA-TRANSFORMATION-REF", "STORE-EVENT-SUPPORT", "USER-DEFINED-TRANSFORMATION-PROPS", "SELECTOR-FIELD-BYTE-ORDER", "APPLIES-TO", "SW-COMPONENT-MAPPING-CONSTRAINTS", "DO-IP-LOGIC-ADDRESS-PROPS", "TIMEOUT-AR", "RPT-HOOK-ACCESS", "ACK-TYPE", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "INTERFACE-BLUEPRINT-MAPPING", "CLIENT-ID", "MAX-AR", "CURRENT-EXP", "REENTRANCY-LEVEL", "DIAG-ROUTINE-TYPE", "CAN-FRAME", "USER-DEFINED-EVENT-DEPLOYMENT", "REFERENCED-TD-EVENT-SWC-REF", "IDSM-INSTANCE-REF-CONDITIONAL", "RELATIVE-REPRESENTATION-TIME", "LIMIT", "RPT-AR-HOOK-IREF", "ACCESS-COUNTS", "ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL", "OPERATION-CYCLE-AUTOMATIC-END", "USER-DEFINED-PDU", "PDU-POOLS", "TCP-IP-ICMP-PROPS-REF", "RECEIVER-ANNOTATION", "REMOTE-PORT-RANGE-END", "AUTH-DATA-FRESHNESS-LENGTH", "REMOTE-SUBJECT-REF", "TCP-TTL", "SCOPE-REFS", "TESTED-ITEM-REFS", "MAXIMUM-SUPPORTED-UCM-MASTER-VERSION", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "SPN-REFS", "DATA-LENGTH-REQUEST", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "PGN", "SUB-NODE-CHANNELS", "FUNCTION-GROUP-MODE-IREF", "DEPENDS-ON-REFS", "OS-TASK-PROXY", "HEALTH-CHANNEL-EXTERNAL-STATUS", "TARGET-FIELD-REF", "IN-MEASUREMENT-SET", "DESTINATION-PHYSICAL-ADDRESS", "EVENT-ELEMENT-MAPPINGS", "RAW-DATA-STREAM-INTERFACE-REF", "COM-MANAGEMENT-PORT-GROUP-IREF", "MODE-DECLARATION-REFS", "NAGLES-ALGORITHM", "SDG-TAILORINGS", "LOCAL-DEBUG-DATAS", "SECURITY-EVENT-DEFINITION", "REFERENCE-TABLE-REF", "EVENT-HANDLERS", "DERIVED-CERTIFICATE-ACCEPTED", "COMPU-RATIONAL-COEFFS", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "SEARCH-BEHAVIOR", "TIME-BASE-PROVIDER-REF", "ABSOLUTE-TOLERANCE", "END-TO-END-PROTECTION-SET", "CRYPTO-KEY-SLOT-INTERFACE", "COMMUNICATION-RESOURCE-PROPS", "BLOCK-IF-STATE-ACTIVE-AP-IREF", "SEPARATED-COMPONENT-IREF", "J-1939-REQUESTABLE", "NM-CYCLETIME-MAIN-FUNCTION", "LIN-SLAVE", "TD-HEADER-ID-RANGE", "MEMORY-SECTION-LOCATION", "DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL", "CONTEXT-LIMITATION-REF", "HW-ELEMENT-REF-CONDITIONAL", "ECU-EXTRACT-VERSION", "APPLICATION-SW-COMPONENT-TYPE", "ECUC-ENUMERATION-LITERAL-DEF", "MSR-QUERY-RESULT-CHAPTER", "WAKEUP-LOCAL-DETECTION-TIME", "PDU-TRIGGERING", "ECU-EXTRACT-REF", "PNC-MAPPINGS", "SWC-MODE-SWITCH-EVENT-REF", "APPLICATION-TYPE-TREF", "DLT-ECU", "SHALL-NOT-RUN-ON-REFS", "CERTIFICATE-TO-KEY-SLOT-MAPPINGS", "ERROR-REACTION-POLICY", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "USE-CLIENT-AUTHENTICATION-REQUEST", "COMPOSITE-RULE-BASED-VALUE-SPECIFICATION", "CAN-FD-FRAME-SUPPORT", "SWC-BSW-RUNNABLE-MAPPING", "END-2-END-EVENT-PROTECTION-PROPS", "SECURITY-EVENT-REFS", "FUNCTION-GROUP-MODE-IREFS", "ECUC-QUERY-STRING-REF", "FLEXRAY-NM-CLUSTER-COUPLING", "PRIMITIVE-ATTRIBUTE-TAILORING", "I-PDU-TIMING", "I-PDU-SIGNAL-PROCESSING", "TD-EVENT-TRIGGER", "SYSTEM-TIMING", "FIRST-ELEMENT-REF", "NUMBER-OF-INSTRUCTIONS", "INTERPOLATION-ROUTINE-MAPPING", "SYSTEM-SIGNAL-REFS", "ASSOCIATED-PDUR-I-PDU-GROUP-REF", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "LOG-TRACE-PORT-ID", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "ESTIMATED-DURATION-OF-CAMPAIGN", "HANDLE-TERMINATION-AND-RESTART", "REST-BOOLEAN-PROPERTY-DEF", "SPECIFICATION-DOCUMENT-SCOPES", "DIFFERENTIATED-SERVICE-FIELD", "REQUIRED-SERVICE-INSTANCE-REF", "DIAGNOSTIC-DATA-IDENTIFIER", "EVENT-ELEMENTS", "MODE-REF", "TARGET-REQUIRED-OPERATION-REF", "SESSION-ID", "CONTAINED-PACKAGE-ELEMENT-REFS", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "RUNNABLE-ENTITY-GROUP-IREFS", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL", "INSTANCE-IDS", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "AUTH-ALGORITHM", "MULTIPLICITY-RESTRICTION", "SUB-ELEMENTS", "LIN-TP-CONFIG", "CRYPTO-KEY-REF", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF", "TRANSFORMATION-TECHNOLOGY", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "EVENT-MAPPING", "CONTEXT-COMPONENT-PROTOTYPE-REF", "POWER-DOWN-TIME", "ASYNCHRONOUS-SERVER-CALL-POINT-REF", "FIELD-NOTIFIER-REF", "ENDPOINT-CONFIGURATION-REF", "IDSM-RATE-LIMITATION-REF", "TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT", "PER-INSTANCE-PARAMETER-REF", "TARGET-PDU-TRIGGERINGS", "FOLLOW-UP-OFFSET", "SYSTEM-DOCUMENTATIONS", "SWC-EXCLUSIVE-AREA-POLICY", "TRANSITION-REF", "ALLOWED-I-PV-6-EXT-HEADERS", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "TRANSMISSION-MODE-TRUE-TIMING", "STORE-ON-CHANGE", "APPL-INIT-VALUE", "BSW-INTERNAL-TRIGGERING-POINT-POLICY", "TARGET-R-PORT-PROTOTYPE-REF", "MIN-MARGIN", "FRESHNESS-VALUE-LENGTH", "FORMAL-BLUEPRINT-GENERATOR", "UDP-TTL", "DEFAULT-MODE-REF", "UNIT-DISPLAY-NAME", "ELEMENT-POSITION", "BSW-VARIABLE-ACCESS", "COMMUNICATION-CLUSTER-REFS", "TARGET-STATUS-REF", "DISABLED-IN-MODE-IREF", "PHYS-CONSTRS", "SW-VARIABLE-IMPLS", "VEHICLE-DRIVER-NOTIFICATION", "TARGET-PORT-PROTOTYPE-REF", "RECORD-ELEMENTS", "SUP", "DIAGNOSTIC-COMMON-PROPS-CONDITIONAL", "WAKEUP-REMOTE-ENABLED", "ARRAY-SIZE-HANDLING", "INTERPOLATION-ROUTINES", "SW-AXIS-GENERIC", "STD", "CONFIG-ELEMENT-DEF-GLOBAL-REF", "PORT-REF", "QUEUED-SENDER-COM-SPEC", "BUILD-ACTION-ENVIRONMENT", "CRYPTO-MODULE-INSTANTIATION", "SW-RECORD-LAYOUT-GROUP-STEP", "NETWORK-ENDPOINT-REF", "MAX-DIFF", "NDP-PROPS", "SIGNAL-I-PDU-COUNTER", "METHOD-REQUEST-PROPSS", "ENUMERATOR-VALUE", "LOGICAL-SUPERVISION", "PLCA-MAX-BURST-TIMER", "DLT-MESSAGE-COLLECTION-SET", "SHORT-TERM-ADJUSTMENT", "OPERATION-IREFS", "PHM-LOGICAL-EXPRESSION", "MULTIPLICITY-CONFIG-CLASSES", "COUPLED-CLUSTER-REFS", "TERMINATION-BEHAVIOR", "DO-IP-INSTANTIATION", "NM-CHANNEL-ACTIVE", "DIAGNOSTIC-INDICATOR-NEEDS", "CAN-TP-ADDRESS", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "ENABLE-WRITE-ACCESS-CONTROL", "MC-DATA-ACCESS-DETAILS", "SA-RAND-TIME", "VERIFICATION-HASH", "IMPLEMENTS-LEGACY-STRING-SERIALIZATION", "V-2-X-FAC-USER-NEEDS", "CUSTOM-DOCUMENTATION-REF", "GROUP-NUMBER", "NODE-REF", "AFFECTION-KIND", "SOURCE-ELEMENT-REFS", "EOC-EXECUTABLE-ENTITY-REF", "CAN-TP-CHANNEL", "PRESTORAGE-FREEZE-FRAME", "COMMUNICATION-CLUSTER-REF", "CONSTANT-MEMORYS", "CRYPTO-NEED-REF", "SERVICE-INTERFACE-PEDIGREE", "POST-VERIFICATION-REBOOT", "COMMENT", "UPPER-MULTIPLICITY-INFINITE", "MEM-CLASS-SYMBOL", "TARGET-DATA-PROTOTYPE-IN-SR-REF", "SW-SYSCOND", "LATENCY-CONSTRAINT-TYPE", "SW-VARIABLE-IMPL", "TRANSFORMER-CLASS", "FIELD-REQUEST-TOPIC-NAME", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "PRE-ACTIVATE-REF", "SWC-TRIGGER-IREF", "SECOND-OPERATION-REF", "GLOBAL-TIME-DOMAIN-REF", "NM-MULTIPLE-CHANNELS-ENABLED", "PDU-TRIGGERING-REF", "PNC-WAKEUP-CAN-ID-MASK", "NM-BUS-SYNCHRONIZATION-ENABLED", "SERVICE-PROXY-SW-COMPONENT-TYPE", "VALUE-REF", "TIME-BASE", "ALIAS-NAME-SET", "ELEMENT-IN-IMPL-DATATYPE", "IP-SEC-RULE", "SOMEIP-METHOD-DEPLOYMENT", "SERVICE-INTERFACE-SUB-ELEMENT", "PORT-DEFINED-ARGUMENT-BLUEPRINTS", "E-2-E-PROFILE-CONFIGURATION-SET", "COLLECTION-PROPS", "LOCAL-SUPERVISION", "VFB-TIMING", "ELLIPTIC-CURVE-REF", "WAKEUP-FRAME-REF", "TRIGGER-INTERFACE-MAPPING", "DTC-STATUS-AVAILABILITY-MASK", "I-SIGNAL-TO-PDU-MAPPINGS", "WORST-CASE-HEAP-USAGE", "IS-AUTO-VALUE", "SW-MAINTENANCE-NOTES", "NUMBER-OF-REPETITIONS", "RPT-EXECUTION-CONTEXT", "SENDER-IREF", "HW-ATTRIBUTE-LITERALS", "SW-AXIS-INDIVIDUAL", "FORMAL-BLUEPRINT-CONDITION", "UNIT-GROUP-REFS", "ASSEMBLY-SW-CONNECTOR", "RTE-EVENT-IREFS", "MULTIPLEXED-I-PDU", "REQUIRED-SERVICE-INSTANCE-ID", "RAW-DATA-STREAM-GRANT-DESIGN", "ARBITRARY-EVENT-TRIGGERING", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "VALIDATION-FORMULA", "SOCKET-OPTION", "ADAPTIVE-AUTOSAR-APPLICATION", "OPTION-ARGUMENT", "TRIGGER-I-PDU-SEND-CONDITIONS", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "MEMORY-ID", "ACTION-LISTS", "AUTHENTICATION-PROPSS", "WAKEUP-FORWARD-LOCAL-ENABLED", "CLIENT-ID-DEFINITION-SET-REF", "DIAGNOSTIC-IO-CONTROL-CLASS", "VEHICLE-ANNOUNCEMENT-COUNT", "TCP-PROPS", "REQUIREMENT-REF", "MAX-NUMBER-FREEZE-FRAME-RECORDS", "FLOW-CONTROL-PDU-REFS", "CONTROL-ENABLE-MASK-BITS", "SYSTEM-TRIGGERED-EVENTS", "SECONDARY-FRESHNESS-VALUE-ID", "OPTIONS", "FM-FEATURE-MAP-CONDITION", "ASSOCIATED-PDUR-I-PDU-GROUP-REFS", "SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS", "ECU-CONFIGURATION-REF", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "FRESHNESS-PROPSS", "CONFIG-VARIANT", "DEFAULT-LOG-THRESHOLD", "WAIT-POINTS", "I-PDU-PORT-REF", "IDSM-INSTANCE-REF", "TABLE-ENTRYS", "DATA-PROTOTYPES", "FUP-DATA-ID-LISTS", "INCLUDE-REF", "VLAN-PRIORITY", "IPV-6-ADDRESS-SOURCE", "SOURCE-SERVICE-INTERFACE-REF", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "COMPU-CONST", "FREEZE-FRAME-REF", "METHOD-ID", "LIVELINESS-PROTECTION-KIND", "BULK-NV-DATA-DESCRIPTOR", "BLACKLISTED-VERSIONS", "FUNCTION-CLUSTER-AFFILIATION", "GLOBAL-SUPERVISION-ENTITY", "ENUMERATION-MAPPING-TABLE", "CUSTOM-DOCUMENT-ELEMENT-REF", "BEHAVIOR-REF", "IS-VARIADIC-TEMPLATE", "UCM-DESCRIPTION", "FRAME-PORT", "PDU-TO-FRAME-MAPPINGS", "CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF", "KEY-VALUE-PAIRS", "MULTICAST-CONNECTOR-REFS", "NM-CONTROL-BIT-VECTOR-ENABLED", "DIAGNOSTIC-COM-CONTROL-CLASS", "NM-NODE-REFS", "SECURITY-LEVEL-REF", "CRYPTO-R-PORT-COM-SPEC", "PNC-GROUP-REFS", "DLT-LOG-SINK", "VARIABLE-DATA-PROTOTYPE", "SUPPORTED-CONFIG-VARIANT", "TRIGGER-PERIOD", "CONTAINED-PROCESS-REFS", "INCLUDED-VARIANT-REF", "SW-DIAGNOSTICS-NOTES", "LITERAL-PREFIX", "REPLACE-WITH", "COMPONENT-IREF", "SAVE-CONFIGURATION", "CONNECTIONS", "OFS-SUB-TLV", "DIAGNOSTIC-READ-DTC-INFORMATION", "DIAGNOSTIC-EVENT-WINDOW", "ARTIFACT-CHECKSUMS", "COMPOSITION-DATA-PROTOTYPE-REF", "SW-DATA-DEF-PROPS-VARIANTS", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "SERVICE-DEPENDENCYS", "ADDRESS-SEMANTICS", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "CALIBRATION-PARAMETER-VALUE-SET-REFS", "CIPHER-SUITE-NAME", "FIELD-NOTIFIER-QOS-PROPSS", "IO-CONTROL-CLASS-REF", "INITIAL-FIND-BEHAVIOR", "IS-UNUSED", "TP-SDU-REF", "RPT-IMPL-POLICY", "AH-CIPHER-SUITE-NAME", "RPT-SW-PROTOTYPING-ACCESS", "REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS", "DIAGNOSTIC-SESSION-REFS", "PERSISTENCY-DATA-ELEMENT", "DIAGNOSTIC-STORAGE-CONDITION", "USED-IMPLEMENTATION-DATA-TYPE-REF", "SDG-DEF", "XREF", "SW-CALPRM-PROTOTYPE-REF", "FAILURE-MONITORING-REF", "SCHEDULE-CHANGE-NEXT-TIME-BASE", "COUPLING-PORT-CONNECTION", "FUNCTIONAL-REQUEST-REF", "SW-ADDR-METHOD", "CHECKPOINT-TRANSITION", "CRC-SECURED", "SYNC-CONFIRMATION-TIMEOUT", "PROCESSING-KIND", "SEND-POLICYS", "ECU-COMM-PORT-INSTANCES", "STANDARD", "RETURN-SIGNAL-REF", "SW-RECORD-LAYOUT-GROUP", "PERSISTENCY-DATA-REQUIRED-COM-SPEC", "OPT-IMPL-RECORD-ELEMENTS", "SDG-CLASS-REF", "HEALING-CYCLE-REF", "CRYPTO-JOBS", "HAS-GETTER", "NUMBER-OF-WORKERS", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING", "HANDLE-INVALID", "END-2-END-METHOD-PROTECTION-PROPS", "TD-PDU-TRIGGERING-FILTER-REFS", "EVENT-GROUPS", "SUPERVISION-REF", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "AP-APPLICATION-ERROR-SET", "COUNTER-JUMP-DOWN-VALUE", "START-ADDRESS", "PACKAGER-ID", "USES-END-TO-END-PROTECTION", "PER-INSTANCE-MEMORY-SIZES", "IDSM-RATE-LIMITATION-REF-CONDITIONAL", "URL", "LOGICAL-EXPRESSION", "ROM-BLOCK", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS", "MAP", "DATA-SEND-COMPLETED-EVENT", "COM-FIELD-GRANT-DESIGN", "DDS-SECURE-GOVERNANCE", "MC-FUNCTION", "VALIDATION-ROOT", "DDS-EVENT-QOS-PROPS", "DOMAIN", "ACTION-TYPE", "MEMORY-HIGH-ADDRESS-LABEL", "TRANSFER-REF", "PROCESS-ARGUMENTS", "SYSTEM-SIGNAL-GROUP", "DIAGNOSTIC-CONNECTION-REF-CONDITIONAL", "NV-DATA-PORT-ANNOTATION", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "NV-BLOCK-DESCRIPTOR", "ECUC-STRING-PARAM-DEF-CONDITIONAL", "ANONYMOUS", "IUMPR-GROUP-IDENTIFIERS", "COUNTER-PASSED-THRESHOLD", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "PREDEFINED-VARIANT", "DESC", "NM-NID-POSITION", "LONG-NAME", "NEW-NAD", "DIAGNOSTIC-AUTH-ROLE", "MAX-NUMBER-OF-EVENT-ENTRIES", "IP-IAM-AUTHENTIC-CONNECTION-PROPS", "HW-PIN-REFS", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "GLOBAL-SUPERVISIONS", "TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY", "ADDITIONAL-INFORMATION", "DESIGN-REF", "CHECKSUM-VALUE", "MODE-MANAGER-ERROR-BEHAVIOR", "GLOBAL-TIME-DOMAIN", "APPLICATION-ACTION-ITEM", "SW-BIT-REPRESENTATION", "I-PDU-IDENTIFIER-TCP-REFS", "IS-SERVICE", "SET", "GENERIC-ETHERNET-FRAME", "INHIBITION-SOURCE-REF", "RTE-EVENT-IN-SYSTEM-SEPARATION", "USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF", "BY-PASS-POINT-IREF", "SUB-ATTRIBUTE-TAILORINGS", "TCP-IP-NDP-MAX-RTR-SOLICITATIONS", "TEXTUAL-CONDITION", "EXT-HEADER-FILTER-LISTS", "HEADER-ID", "METHOD-REQUEST-TOPIC-NAME", "TLV-DATA-ID-0-REF", "INITIAL-STATUS", "TLS-CONNECTION-GROUP", "CONDITION", "MIN-LENGTH", "SECURE-COM-CONFIGS", "DATA-PROTOTYPE-TRANSFORMATION-PROPS", "MSR-QUERY-ARG", "PATH-MTU-DISCOVERY-ENABLED", "RX-PDU-POOL-REF", "J-1939-TP-NODE", "RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS", "INTERPOLATION-ROUTINE-MAPPINGS", "GENERAL-INACTIVITY-TIME", "LOCAL-CERTIFICATE-REF", "TRIGGER-TO-SIGNAL-MAPPING", "CAPTION", "TCP-IP-IP-REASSEMBLY-TIMEOUT", "DIRECT-SUCCESSOR-REF", "COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION", "EID-USE-MAC", "SUBFUNCTIONS", "OFNS-DATA-ID-LISTS", "WAIT-POINT", "COMPRESSED-SOFTWARE-PACKAGE-SIZE", "CONFIGURATION-HANDLING", "RESOURCE-DEFINITIONS", "COM-FIELD-GRANT", "PERMIT-MULTIPLE-REFERENCES-TO-EE", "ECUC-ADD-INFO-PARAM-DEF", "SW-RECORD-LAYOUT-V-FIX-VALUE", "PROGRAMMING-LANGUAGE", "TRANSFER-EXIT-CLASS-REF", "SWC-MAPPING-CONSTRAINT-REF", "REF-MEASUREMENT-SET", "TCP-IP-PROPS-REF", "DISPLAY-IDENTIFIER", "KEY-SLOT-ALLOWED-MODIFICATION", "RULES", "WAKE-UP-PATTERN", "ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS", "EVENT-GROUP-CONTROL-TYPE", "ELEMENT-BYTE-VALUE", "COMPOSITE-SERVICE-INTERFACE-REF", "NM-COORD-SYNC-SUPPORT", "PROCESS-DESIGN", "MODE-REPORTER-PORT-IREF", "NM-PASSIVE-MODE-ENABLED", "CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING", "REPRESENTS-VIN", "KEYWORD", "TLS-CONNECTION-REFS", "FEATURE-REFS", "NATIVE-DECLARATION", "SOFTWARE-MEMORY-SECTION-REF", "EXECUTE-DESPITE-DATA-UNAVAILABILITY", "CLASSIFICATIONS", "SCHEDULER-MAX-NUMBER", "POSSIBLE-AP-ERROR-REF", "DTC-STATUS-MASK", "AR-PARAMETER", "DESTINATION-REFS", "AP-APPLICATION-ENDPOINTS", "UPDATE-INDICATION-BIT-POSITION", "PARAMETER-ACCESSS", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "SW-MAX-TEXT-SIZE", "COMMUNICATION-MODE", "ECUC-QUERY", "VERBATIM", "NM-NODES", "IPV-4-PROPS", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "PREDECESSOR-REF", "SOURCE-REF", "ISSUED-BY", "TIME-BR", "COMMUNICATION-CONNECTORS", "IPV-4-ADDRESS-SOURCE", "EVENT-GROUP-ID", "DATA-PROTOTYPE-GROUP-IREF", "METHOD-RESPONSE-PROPSS", "NOTE", "SIGNAL-BASED-FIELD-DEPLOYMENT", "CAN-TP-NODE", "NOT-AVAILABLE-VALUE-SPECIFICATION", "SUB-FUNCTION-REFS", "TRACEABLE-TABLE", "NM-NODE-DETECTION-ENABLED", "FIRST-ELEMENT-INSTANCE-IREFS", "PROTOCOL-NAME", "SCHEDULER-NAME-PREFIXS", "MANUFACTURER-VALIDATION-ORDER-REFS", "UAS-ID", "COMPONENT-SEPARATION", "IN-SCOPE", "SECURE-COM-PROPS-FOR-TCP-REF", "AGING-CYCLES", "LOWER-CAN-ID", "DIAGNOSTIC-ECU-RESET", "FIRST-ELEMENT-REFS", "ACTION-LIST-ITEM-REF", "SERVER-CALL-POINTS", "REMOTE-ADDRESSS", "BOR-TIME-L-2", "PERSISTENCY-PROVIDED-COM-SPEC", "BUILD-ACTION-MANIFEST-REF-CONDITIONAL", "DATA-REF", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "PARAMETER", "AGING-REQUIRES-TESTED-CYCLE", "SWC-MODE-GROUP-IREF", "DDS-REQUIRED-SERVICE-INSTANCE", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "TRIGGER-MAPPINGS", "TP-ADDRESS", "PROPS", "FM-FEATURE-SELECTION-SET", "GROUPED-ALIAS-EVENT-REFS", "LOG-SINK-REF", "PHYSICAL-CHANNEL-REF-CONDITIONAL", "E", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST", "DIAGNOSTIC-OPERATION-CYCLE", "COUPLING-PORT-SWITCHOFF-DELAY", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "ETHERNET-COMMUNICATION-CONTROLLER", "DIAGNOSTIC-TRANSFER-EXIT", "INTERPOLATION-ROUTINE-REF", "INHIBITING-FID-REF", "GLOBAL-TIME-GATEWAY", "IDSM-RATE-LIMITATION", "TIMING-CONDITIONS", "MODE-IN-BSW-INSTANCE-REF", "COMPOSITE-INTERFACE", "HANDLE-OUT-OF-RANGE-STATUS", "LIN-CONFIGURABLE-FRAMES", "DIAGNOSTIC-INFO-TYPE", "PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT", "SECTION-NAME-PREFIX", "DEFAULT-REPORTING-MODE", "SECTION-TYPE", "CONNECTION-IS-MANDATORY", "PERSISTENCY-KEY-VALUE-STORAGE", "HW-CATEGORY", "DIRECT-SUCCESSOR-REFS", "OUTGOING-CALLBACKS", "POST-BUILD-VARIANT-CONDITIONS", "SHARED-AXIS-TYPE-REF", "CALL-TYPE", "LINKER", "DATA-IDS", "PORT-INTERFACE-REF", "CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING", "CONFIDENCE-INTERVAL", "ENABLES-LOG-TRACE", "LET-INTERVAL-REFS", "EXECUTABLE-PROVIDED-PORT-IREF", "DATA-CONSTR-REF", "CONSIDER-BUILD-NUMBER", "ECUC-NUMERICAL-PARAM-VALUE", "MSR-QUERY-RESULT-P-2", "SETTER-CALL-SIGNAL-REF", "EXITED-MODE-REF", "SPEEDUP", "HEADER-FILE", "SW-COMP-TO-ECU-MAPPING-REF", "UDP-PROPS", "DO-IP-ENTITY", "COMPLEX-TYPE-MAPPING", "STATE-REF", "ACTIONS", "INVALIDATION-POLICY", "SERVICE-IDENTIFIER", "DISCOVERY-PROTECTION-KIND", "PERIODIC-EVENT-TRIGGERING", "IDS-COMMON-ELEMENT-REF", "TCP-IP-ARP-PACKET-QUEUE-ENABLED", "SUPPORTS-ASYNCHRONOUS-MODE-SWITCH", "SOURCE-POSSIBLE-ERROR-REF", "LOCAL-IP-SEC-RULE-REFS", "SOURCE-METHOD-REF", "SYNC-FRAME-ID-COUNT-MAX", "TCP-RECEIVE-WINDOW-MAX", "FLEXRAY-COMMUNICATION-CONTROLLER", "DATA-PROTOTYPE-MAPPING", "MSR-QUERY-PROPS", "TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "AUTHENTICATION-ROLE-REFS", "GLOBAL-SUPERVISION", "CALL-SIGNAL-TRIGGERING-REF", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "WRITING-FREQUENCY", "PORT-INTERFACE-MAPPING-SET", "TP-SDU-REFS", "DATA-TYPE-REF", "SW-VALUES-PHYS", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "ALLOCATOR", "MAXIMUM-LENGTH", "VALUE-PAIRS", "REF-CALPRM-SET", "TD-EVENT-OPERATION-TYPE", "DISPLAY-NAME", "TIME-SYNC-P-PORT-PROTOTYPE-IREF", "COLLECTION-SEMANTICS", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "BURST-SIZE", "BSW-MODE-RECEIVER-POLICY", "AUTO-REQUIRE", "TRIGGER-I-PDU-SEND-CONDITION", "NUMBER-OF-MINISLOTS", "MAC-MULTICAST-GROUP-REF", "RESPONSE-DATA-SIZE", "TIMEOUT", "MODULE-ID", "EXECUTION-TIME-TYPE", "RESOURCE-GROUPS", "DPG-DOES-NOT-REQUIRE-COHERENCYS", "REQUIREMENT-REFS", "CONTEXT-SW-COMPONENT-PROTOTYPE-REF", "COMM-CONTROLLER-MAPPINGS", "ALLOW-PROVIDER-RATE-CORRECTION", "SUB-DOMAIN-REFS", "RESTORE-AT-START", "EMULATION-SUPPORTS", "FUNCTIONAL-REQUEST-REFS", "CONTENT-TYPE", "TLS-CIPHER-SUITES", "READ-DTC-INFORMATION-CLASS-REF", "AGGREGATION-REF", "REPORTED-SECURITY-EVENT-IREF", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "SERVICE-OFFER-TIME-TO-LIVE", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "SLOT-ID", "PARAMETER-VALUES", "SENDER-REC-ARRAY-TYPE-MAPPING", "PURE-LOCAL-TIME-BASE", "SERVICE-INTERFACE-MAPPING", "TRANSMISSION-MODE-FALSE-TIMING", "DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT", "REMAPPED-CAN-ID", "PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS", "ECUC-QUERY-REF", "SYSTEM-VERSION", "DRIVER-NOTIFICATIONS", "REPRESENTED-PORT-GROUP-REF", "EXIT-MODE-DECLARATION-REF", "SW-DATA-DEPENDENCY-FORMULA", "SW-CALPRM-IMPLS", "MAX-RETRIES", "REQUEST-GROUP-REF", "SECTION-INITIALIZATION-POLICY", "NM-COORD-CLUSTER", "MAC-MULTICAST-ADDRESS", "ARTIFACT-DESCRIPTORS", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "CHAPTER", "CONSTRAINT-REF", "PREDECESSOR-ACTION-REF", "LIN-SLAVES", "SECOND-ELEMENT-REFS", "SW-SYSTEMCONSTANT-VALUE-SET-REF", "WINDOW-SIZE", "CUSTOM-TRIGGER", "TP-CONNECTION-CONTROLS", "SOFTWARE-CLUSTER-ID", "EVENT-GROUP-REF", "CYCLE-REPETITION", "DIAGNOSTIC-DTC-CHANGE-TRIGGER", "SUBSCRIBE-EVENTGROUP-RETRY-DELAY", "ELEMENT", "EXECUTABLE-REF", "PREEMPTABILITY", "SWC-TO-APPLICATION-PARTITION-MAPPING", "SERVICE-INTERFACE-REF", "SIGNAL-FAN", "DIAGNOSTIC-IUMPR-GROUP", "USE-AS-CRYPTOGRAPHIC-I-PDU", "PER-INSTANCE-MEMORY", "CONTROL-CONSUMED-EVENT-GROUP-REF", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "NM-STATE-CHANGE-IND-ENABLED", "REQUIRED-ENVIRONMENT-REF", "PDU-COLLECTION-MAX-BUFFER-SIZE", "VERSION-DRIVEN-FIND-BEHAVIOR", "INIT-VALUE", "SUPERVISED-ENTITY-NEEDS", "END-TO-END-PROFILE", "EXCLUSIVE-AREA-REFS", "TCP-IP-IP-NUM-FRAGMENTS", "SEPARATED-COMPONENT-IREFS", "EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD", "MESSAGE-TYPE", "NV-BLOCK-NEEDS", "HW-PIN-GROUP-CONNECTIONS", "RELATIVE-TOLERANCE", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "LIN-CHECKSUM", "ECUC-QUERY-EXPRESSION", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF", "PRIMITIVE-TYPE-MAPPINGS", "CONNECT", "P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF", "MAX-ALIVE-CYCLE", "DDS-FIELD-DEPLOYMENT", "HW-TYPE-REF", "ETH-TP-CONNECTION", "CSE-CODE-FACTOR", "SYSTEM-REF", "XDOC", "FILTER-CHAINS", "TD-EVENT-FRAME", "TARGET-DATA-ELEMENT-REF", "FLAT-MAP-REF", "ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT", "CLIENT-SERVER-APPLICATION-ERROR-MAPPING", "START-ACTION-REFS", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "DATA-TYPE-FOR-SERIALIZATION-REF", "REFINED-MODULE-DEF-REF", "MINOR-VERSION", "SW-COMPONENT-IREF", "VEHICLE-IDENTIFICATION-SYNC-STATUS", "TRANSFORMATION-PROPSS", "SOURCE-SERVICE-INTERFACE-REFS", "PNC-CLUSTER-VECTOR-LENGTH", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "AUTHENTICATION-BUILD-ATTEMPTS", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "WRITTEN-LOCAL-VARIABLES", "DIAG-PDU-TYPE", "HW-PORT-MAPPINGS", "SOFTWARE-PACKAGE-STEPS", "TARGET-COMPONENT-REF", "INCLUDED-DATA-TYPE-SET", "CAN-TP-CHANNEL-REF", "DOC-REVISIONS", "SUPERVISION-MODE-CONDITION", "L-2", "ROOT-DATA-PROTOTYPE-IN-SR-REF", "FRAME-REF", "DDS-METHOD-QOS-PROPS", "REPORTABLE-SECURITY-EVENT-REF", "WAKEUP-SYMBOL-RX-WINDOW", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "HW-ATTRIBUTE-DEF-REF", "MULTIPLE-CONFIGURATION-CONTAINER", "ENABLE-READ-ACCESS-CONTROL", "TT-CAN-CLUSTER-REF", "LIN-SLAVE-CONFIG", "RTPS-PROTECTION-KIND", "MIN-SAMPLE-POINT", "NOTIFIER-SIGNAL-TRIGGERING-REF", "WORST-CASE-EXECUTION-TIME", "MAP-ELEMENT-TUPLES", "SENDER-RECEIVER-INTERFACE", "START-ACTION-REF", "SHORT-TERM-ADJUSTMENT-SUPPORTED", "ALLOCATOR-REF", "ECU-MAPPING", "DIAGNOSTIC-CONNECTION", "MAX-LENGTH", "RESULTING-RPT-SW-PROTOTYPING-ACCESS", "META-DATA-ITEM-SETS", "REQUEST-TYPE", "SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF", "MINIMUM-EXECUTION-TIME", "SERVICE-DEPLOYMENT-REF", "LOCAL-CERTIFICATE-REFS", "TEXT-TABLE-VALUE-PAIR", "IMPLICIT-DATA-ACCESS-IREFS", "FDC-THRESHOLD-STORAGE-VALUE", "I-SIGNAL-I-PDU-GROUP", "DIAGNOSTIC-SERVICE-TABLE", "TRIGGER-MODE", "USES-ADDRESS-ARBITRATION", "TTCAN-COMMUNICATION-CONNECTOR", "LIN-EVENT-TRIGGERED-FRAME", "SDU-REF", "SWC-RUNNABLE-REF", "END-TO-END-PROTECTION", "SOMEIP-REQUIRED-EVENT-GROUP", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "ATTRIBUTE-NAME", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "IPV-4-CONFIGURATION", "V-2-X-SUPPORTED", "TIME-BUFFER", "UNASSIGN-FRAME-ID", "NUMBER-OF-BITS", "I-SIGNAL-TRIGGERING", "COMMUNICATION-CYCLE", "ENVIRONMENT-CAPTURE-TO-REPORTING", "SW-MAPPINGS", "INPUT-DATAS", "DIAGNOSTIC-MESSAGE-TYPE", "JITTER", "HOP-COUNT", "IP-SEC-IAM-REMOTE-SUBJECT", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "RATE-DEVIATION-MEASUREMENT-DURATION", "J-1939-NM-CLUSTER", "FLEXRAY-FRAME", "ALLOWED-TCP-OPTIONS", "TARGET-REF", "COMPARE-TYPE", "IMPLIED-OPERATION-REF", "LIFE-CYCLE-STATE", "RESPONSE-ON-EVENT-CLASS-REF", "TCP-CONGESTION-AVOIDANCE-ENABLED", "METHOD-TOPICS-ACCESS-RULE-REF", "EXPECTED-TX-TRIGGER", "HANDLE-TIMEOUT-TYPE", "RX-MASK", "RATE-LIMITATION-FILTERS", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "CUSTOM-SDG-DEF-REFS", "REQUESTER-REFS", "I-PDU-IDENTIFIERS", "TEX-MATH", "LENGTH", "THEAD", "MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE", "FIRST-ELEMENT-INSTANCE-IREF", "SECOND-MODE-REF", "SPORADIC-EVENT-TRIGGERING", "IUMPR-REF", "ACL-CONTEXT", "ARTIFACT-CHECKSUM-REF", "DTC-FORMAT-TYPE", "IMPORTANCE", "RECEPTION-PROPS", "RPT-CONTAINER", "E-2-E-EVENT-PROTECTION-PROPSS", "COUPLING-TYPE", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "REQUEST-TOPIC-NAME", "REQUESTER-REF", "AR-PACKAGE", "DIAGNOSTIC-IO-CONTROL", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "SEC-OC-CRYPTO-MAPPING-REF", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS", "INSTANCE-ID", "CONDITION-FORMULA", "ACL-OBJECT-REFS", "EXTERNAL-REPLACEMENT-REF", "UNIT-REF", "COMMUNICATION-BUFFER-LOCKING", "ECUC-DEF-EDITION", "ACCEPT-ENCODING", "EVENT-MULTICAST-ADDRESSS", "BSW-QUEUED-DATA-RECEPTION-POLICY", "IMPLEMENTATION-CONFIG-CLASSES", "TOPIC-NAME", "COUPLING-PORT", "PRE-ACTIVATE-IREF", "INTRODUCTION", "AP-APPLICATION-ERROR-REFS", "FIELD-SENDER-COM-SPEC", "ROM-LOCATION-REF", "PRECONFIGURED-CONFIGURATION-REFS", "CLIENT-SERVER-ANNOTATIONS", "ASSIGN-NAD", "NETWORK-ENDPOINT", "J-1939-NM-NODE-REF", "SECURITY-LEVEL-REFS", "CIPHER-SUITE-SHORT-LABEL", "LOCAL-NETWORK-ENDPOINT-REF", "CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF", "PROP-SEG", "SERVICE-TIMING", "J-1939-TP-PG", "ENTERED-MODE-REF", "REMOTING-TECHNOLOGY", "NAME-TOKEN-VALUE-VARIATION-POINT", "TEXT-TABLE-MAPPINGS", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "DIAGNOSTIC-ADDRESS", "OPTION", "IMPLEMENTATION-REF", "MINIMUM", "REQUIRED-DATAS", "LIN-SCHEDULE-TABLE", "TRIGGERING-EVENT-REF", "CLIENT-SERVER-OPERATION-MAPPING", "DDS-RPC-SERVICES", "DIAGNOSTIC-FREEZE-FRAME-REF", "USER-DEFINED-CLUSTER", "SECURITY-EVENT-CONTEXT-DATA", "TLS-SECURE-COM-PROPS", "SW-NUMBER-OF-AXIS-POINTS", "CONSTR-LEVEL", "COUPLING-PORT-SPEED", "SCOPE-REF", "COUPLING-PORT-REF-CONDITIONAL", "CLASSIFICATION", "NUMERICAL-VALUE-SPECIFICATION", "MACRO-PER-CYCLE", "TX-PDU-TRIGGERING-REF", "CRYPTO-SERVICE-MAPPINGS", "TIME-SYNC-TECHNOLOGY", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "KEY-SLOT-USED-FOR-START-UP", "INNER-GROUP-IREF", "CONSTANT-SPECIFICATION-MAPPING-SET", "INTERFACE-BLUEPRINT-REF", "SHORT-NAME-FRAGMENTS", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "J-1939-TP-CONNECTION", "DIAGNOSTIC-ENV-MODE-CONDITION", "USE-INSTEAD-REFS", "ACCEPTED-STARTUP-RANGE", "PREFIX-REF", "MASS-EXP", "RX-SECURITY-VERIFICATION", "SW-CALPRM-AXIS-SET", "RECORD-ELEMENT-REF", "FILE-TYPE-PATTERN", "NETWORK-INTERFACES", "CONSUMED-EVENT-GROUPS", "DIAGNOSTIC-IO-CONTROL-NEEDS", "REFERENCE-VALUES", "LATEST-TX", "REQUESTABLE", "TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED", "CLEAR-DTC-LIMITATION", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL", "DLT-ECU-REF", "NAS-TIMEOUT", "MODE-DECLARATION-MAPPING", "CONTAINED-PACKAGE-ELEMENT-REF", "CHECK-PER-SOURCE-ID", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "SW-ADDR-METHOD-REF", "SUBSTITUTED-FRAME-REFS", "COMPU-PHYS-TO-INTERNAL", "SERVICE-INSTANCE-ID", "COM-METHOD-GRANT-DESIGN", "FIELD-MAPPINGS", "COMMUNICATION-RESOURCE-REF", "SW-IMPL-POLICY", "MAPPING-DIRECTION", "RUNNABLE-ENTITY-GROUP-IREF", "UDP-NM-NODE", "TRIGGERS-RECOVERY-NOTIFICATION", "MINIMUM-SELECTED-BINDING-TIME", "DIAGNOSTIC-ROUTINE", "STIM-ENABLER", "CONTEXT-DATA-SOURCE", "CONNECTION-NEGOTIATION-BEHAVIOR", "I-SIGNAL-MAPPING", "ECU-TIMING", "BUILD-TYPE", "COM-MANAGEMENT-MAPPING", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "I-SIGNAL-TO-I-PDU-MAPPINGS", "RESOURCE-GUARD-VALUE", "SOFTWARE-PACKAGE-STORING", "CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF", "ECU-INSTANCE-PROPS", "KEY-SLOT-ONLY-ENABLED", "IS-DELTA-PACKAGE", "RPT-SERVICE-POINT-PRE-REFS", "FILE-REFS", "RAM-BLOCK", "SW-COMPONENT-PROTOTYPE-IREF", "CAN-ENTERS", "INTERFACE-VERSION", "AVERAGE-MEMORY-CONSUMPTION", "INSTANCE-ID-PROPS", "WRITING-STRATEGY-ROLE", "IKE-OVER-TIME", "TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE", "SOMEIP-TRANSFORMATION-PROPS", "MAX-NUMBER-OF-ELEMENTS", "PARTIAL-NETWORK-REFS", "SW-FEATURE-DEF", "SHALL-RUN-ON-REF", "VERSION", "GATEWAYS", "INITIAL-NAD", "LOCAL-VARIABLE-REF", "DETECT-NIT-ERROR", "DIAGNOSTIC-ROUTINE-CONTROL", "MODIFICATION", "EXECUTABLE-ENTITY-REF", "SHORT-NAME-PATTERN", "CONSUMED-SERVICE-INSTANCES", "COM-MANAGEMENT-GROUP-REF", "TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC", "SOURCE-FIELD-REF", "NM-CLUSTERS", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "TLS-CRYPTO-CIPHER-SUITE", "SYNC-LOSS-TIMEOUT", "ORDERED-ELEMENTS", "FIRST-ELEMENTS", "SOMEIP-METHOD", "REST-RESOURCE-DEF", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "MICROTICK-DURATION", "SW-CALPRM-IMPL", "PRIORITYS", "MAXIMUM-KEY-LENGTH", "INTENDED-TARGET-MACHINE-REF", "MINIMUM-DISTANCES", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "SDF", "OPERATOR", "VIEW-MAPS", "PROVIDER-IREF", "MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL", "SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF", "GENERAL-PURPOSE-PDU", "APPLICATION-ARRAY-DATA-TYPE", "APPLICATION-RECORD-ELEMENT-REF", "REQUIRED-DDS-SERVICE-INSTANCE", "TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE", "PLATFORM-PHM-ACTION-ITEM", "ROOT-SOFTWARE-COMPOSITIONS", "PATH", "DATA-FILTER", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF", "VARIANT-CRITERION-REF", "PERSISTENCY-FILE-PROXY-INTERFACE", "ITEMS", "SELECTOR-FIELD-LENGTH", "INITIALIZED-PARAMETER-REF", "DIAGNOSTIC-EVENT-INFO-NEEDS", "ADD-INFO-5", "TCP-CONNECT-TIMEOUT", "SUPERVISION-ENTITYS", "ACTIVE-SUPERVISION-REF", "SOMEIP-TRANSFORMATION-PROPS-REF", "ECUC-BOOLEAN-PARAM-DEF", "USE-INSTEAD-REF", "REVISION-LABELS", "TRIGGER-MAPPING", "CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "I-PDU-PORT-REFS", "METHOD-DEPLOYMENTS", "LIN-SPORADIC-FRAME", "SPN-REF", "MODE-PORT-ANNOTATION", "USE-AUTH-DATA-FRESHNESS", "LICENSE-REF", "MAX-NUMBER-OF-FILES", "PADDING-VALUE", "DATA-ID", "EXCLUSIVE-AREA-NESTING-ORDER", "PRMS", "TEAR-DOWN-ACTION-REF", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "SYNCHRONIZED-MASTER-TIME-BASE", "SOCKET-ADDRESSS", "MAX-HEADER-ID", "PRIMITIVE", "PDU-REF", "PDUR-I-PDU-GROUP", "UNICAST-CREDENTIALS", "CAN-FD-BAUDRATE", "MAX-NUM-COMPARISION-OF-VALUE-EVENTS", "PROVIDE-RESOURCES", "ROUTING-GROUP-REFS", "BSW-MODULE-DESCRIPTION", "EVENT-READINESS-GROUPS", "DLT-CONFIG", "BSW-MODULE-DESCRIPTION-REF-CONDITIONAL", "RATE-CORRECTIONS-PER-MEASUREMENT-DURATION", "APPLICATION-PARTITION", "SWC-BSW-MAPPING", "REQUEST-FILE-TRANSFER-CLASS-REF", "SW-COMPONENT-ASSIGNMENTS", "ARRAY-IMPL-POLICY", "PERIODIC-RATE-CATEGORY", "TRAFFIC-LIMITATION-FILTERS", "MEMORY-SECTION", "INTERFACE-MAPPINGS", "SERVER-COM-SPEC", "SIGNAL-PATH-CONSTRAINTS", "PER-STATE-TIMEOUT", "DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF", "ETHERNET-FRAME", "SLOT-TYPE", "BYTE-VALUE", "JOB-REQUIREMENTS", "EVENT-IREF", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "FIFO-RANGES", "SIMULATED-EXECUTION-TIME", "ETH-IP-PROPS", "GLOBAL-ELEMENTS", "TARGET-DATA-PROTOYPE-REF", "PARAMETER-ACCESSED-FOR-DEBUG-REFS", "PNC-VECTOR-LENGTH", "MAPPING-REF", "P-PORT-IN-COMPOSITION-INSTANCE-REF", "INITIAL-EVENT-STATUS", "TIME-LEAP-PAST-THRESHOLD", "FIRST-PHYSICAL-DIMENSION-REF", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "FUP-DATA-ID-LIST", "RPT-ENABLER-IMPL-TYPE", "CHOICES", "PNC-WAKEUP-ENABLE", "HW-ATTRIBUTE-VALUES", "DLT-APPLICATION-TO-PROCESS-MAPPING", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "METHOD-IREF", "IMPLEMENTS-SOMEIP-STRING-HANDLING", "NTU", "PHM-HEALTH-CHANNEL-STATUS", "PARAMETER-POLICYS", "ROOT-DATA-PROTOTYPE-REF", "ENGINEERING-OBJECTS", "ADDRESSING-FORMAT", "TARGET-EVENT-REFS", "ATTRIBUTE-REF", "ELEMENT-PROPSS", "MINIMUM-FIFO-LENGTH", "CRYPTO-PRIMITIVE", "MAX-SAMPLE-POINT", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "AR-TYPED-PER-INSTANCE-MEMORYS", "CONSTANT-REFERENCE", "EXTERN-RATE-CORRECTION", "NM-READY-SLEEP-TIME", "COMMUNICATION-DIRECTION", "EXECUTION-CONTEXTS", "DEFAULT-PERIOD-BEGIN", "SERVICE-INSTANCE-REF", "FUNCTION-INHIBITION-NEEDS", "DATA-TYPE-MAPS", "INITIAL-DYNAMIC-PART", "SYMBOLIC-NAME-PROPS", "MAX-FC-WAIT", "BUS-DEPENDENT-NM-ECUS", "RETRY", "FUNCTION-ID", "TIMING-RESOURCE", "NOMINAL-EXECUTION-TIME", "DEBOUNCE-ALGORITHM-REF", "SOFTWARE-CLUSTER-REQUIREMENT", "FLEXRAY-NM-ECU", "DIAGNOSTIC-TROUBLE-CODE-UDS", "ARGUMENT", "USED-PIM-REF", "NM-IF-ECU-REF", "I-SIGNAL-TYPE", "INTERRUPT-SOURCE", "NESTED-ELEMENT-REF", "ACCESS-COUNT-SET", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "USED-DATA-ELEMENT", "EVENT-GROUP-REFS", "VARIATION-POINT-PROXY", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "SECURE-COMMUNICATION-PROPS", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "SERVICE-DISCOVER-CONFIGS", "CONTAINED-PDU-TRIGGERING-REF", "HW-COMMUNICATION-CONTROLLER-REF", "SOMEIP-PROVIDED-EVENT-GROUP", "FUNCTION-GROUP-IREFS", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "BURST-SIZE-RESPONSE", "E-2-E-PROFILE-CONFIGURATIONS", "DATA-COM-PROPS", "CP-SOFTWARE-CLUSTER", "LIN-ORDERED-CONFIGURABLE-FRAME", "CAN-CONTROLLER-FD-REQUIREMENTS", "UPSTREAM-REFERENCE-IREF", "INCLUDE-REFS", "COMMON-PROPERTIES", "IPV-4-ADDRESS", "SETTER-RETURN-REF", "ON-TRANSITION-VALUE", "SERVICE-RESOURCE-REF", "IMPLICIT-INTER-RUNNABLE-VARIABLES", "MIN-TRCV-DELAY-COMPENSATION-OFFSET", "REPORTABLE-SECURITY-EVENT-REFS", "SW-POINTER-TARGET-PROPS", "IP-V-6-PATH-MTU-TIMEOUT", "ELEMENT-ROLE", "ACCESS-POINT-REF", "ASSIGN-FRAME-ID", "IMMEDIATE-NV-DATA-STORAGE", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "TIME-SYNC-R-PORT-PROTOTYPE-IREF", "EXCLUSIVE-AREA-NESTING-ORDER-REFS", "PNC-PDUR-GROUP-REF", "OUTER-PORTS", "REQUIRED-MODE-GROUPS", "MODE-TRANSITIONS", "IP-ADDRESS-KEEP-BEHAVIOR", "CHECKPOINT-INITIAL-REFS", "PREDECESSOR-REFS", "MC-VARIABLE-INSTANCES", "I-SIGNAL-REF", "J-1939-CLUSTER", "LOCAL-PORT-RANGE-END", "RAM-BLOCK-INIT-VALUE", "MODE-DECLARATION", "CAN-TP-CONFIG", "COMMON-SIGNAL-PATH", "SECURE-COM-CONFIG-REF", "TO-REF", "TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "FIRST-TIMEOUT", "BSW-MODULE-DEPENDENCY", "CRYPTO-PROVIDERS", "PROTOCOL-KIND", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "RECEIVER-CAPABILITY", "DELEGATED-PORT-ANNOTATION", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "TIMEOUT-CS", "IEEE-1722-TP", "REQUESTOR-REF", "ROLE-BASED-DATA-ASSIGNMENT", "INCLUDED-DATA-TYPE-SETS", "PDU-COUNTER-START-POSITION", "FILE-REF", "I-SIGNAL-IN-I-PDU-REF", "ESP-CIPHER-SUITE-NAME", "CONSTANT-SPECIFICATION", "MC-PARAMETER-ELEMENT-GROUP", "TCP-INITIAL-INACTIVITY-TIME", "DERIVED-FROM-BLUEPRINT-REFS", "PATH-MTU-ENABLED", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "CALLBACK-HEADER-REFS", "PERMISSION-CERTIFICATE-AUTHORITY-REF", "USER-DEFINED-I-PDU", "MINISLOT-DURATION", "ALTERNATIVE-NAME", "REPORTING-PORT-PROTOTYPE-IREF", "REQUIRED-ARTIFACTS", "ERROR-DOMAIN-REF", "DO-IP-TARGET-ADDRESS-REF", "BSW-OS-TASK-EXECUTION-EVENT", "FILE-INFO-COMMENT", "COMMUNICATION-CONTROLLER-MAPPING", "START", "WRITING-STRATEGYS", "SENSOR-INSTANCE-ID", "TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY", "USER-DATA-SUB-TLV", "INTERPOLATION-ROUTINE-MAPPING-SET-REFS", "NETWORK-TIME-SLAVE-REF", "FRESHNESS-VALUE-ID", "RECEIVER-REFS", "SENDER-CAPABILITY", "CONTAINED-PDU-TRIGGERING-REFS", "I-PDU-MAPPING", "EXPORTABILITY", "START-ON-EVENT-REF", "DEPENDENCIES", "SECURE-COM-PROPS-SET", "END-TO-END-TRANSFORMATION-COM-SPEC-PROPS", "MSR-QUERY-CHAPTER", "SIGNAL-MAPPINGS", "LIST", "NM-IMMEDIATE-RESTART-ENABLED", "FUNCTION-GROUPS", "PDU-COLLECTION-SEMANTICS", "MAX-SYNC-JUMP-WIDTH", "TRUE-ACTION-LIST-REF", "REMOTE-IP-ADDRESS-REF", "CAN-TP-ECU", "TEXT-TABLE-MAPPING", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "SYNC-PERIOD", "DESTINATION-URI-REFS", "UNASSIGNED-FRAME-TRIGGERING-REF", "APPLICATION-MODE-MACHINE", "UPPER-BOUND", "CHANNEL-REF", "NM-IMMEDIATE-NM-TRANSMISSIONS", "BOR-TIME-L-1", "STARTS-ON-EVENT-REF", "CERTIFICATE-REF", "EVENT-QOS-PROPSS", "BLUEPRINT-REF", "AFFECTED-APPLICATION", "RESOURCE-REF", "E-2-E-METHOD-PROTECTION-PROPSS", "POSITION", "TYPE", "GENERATED-ARTIFACTS", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "TEMPERATURE-EXP", "COM-METHOD-GRANT", "FILE-PROXYS", "CONSTANT-VALUE-MAPPING-REF", "INITIAL-REPETITIONS-BASE-DELAY", "RATIONALE", "CONSISTENCY-NEEDS", "STARTUP-CONFIG-REF", "PDUS", "DATA-TYPE-MAPPING-REFS", "TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION", "TCP-IP-NDP-DEFAULT-REACHABLE-TIME", "ECUC-LINKER-SYMBOL-DEF", "J-1939-DTC-VALUE", "I-PDU-IDENTIFIER-UDP-REF", "FRAME-TRIGGERINGS", "LOCAL-UNICAST-ADDRESSS", "COMPOSITE-NETWORK-REPRESENTATIONS", "REST-ENDPOINT-POST", "VLAN-REF", "BSW-MODE-MANAGER-ERROR-EVENT", "MAX-ERROR-STATE-VALID", "REPORTING-BEHAVIOR", "TCP-GENERAL-INACTIVITY-TIME", "SWC-TO-SWC-OPERATION-ARGUMENTS", "DELTA-PACKAGE-APPLICABLE-VERSION", "CYCLE-AUTOSTART", "LIFE-CYCLE-INFOS", "SW-RECORD-LAYOUT-V", "CAN-CLUSTER-VARIANTS", "OBD-MID", "TLS-CONNECTION-REF", "TCP-LOCAL-PORT", "SYSTEM-SIGNAL-GROUP-REF", "FRAME-PIDS", "SUB-ELEMENT-MAPPINGS", "TCP-SYN-RECEIVED-TIMEOUT", "USE-SMART-SENSOR-API", "TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION", "ENVIRONMENTAL-CONDITION-REF", "EVENT-WINDOW-TIME", "MICRO-INITIAL-OFFSET-B", "ABSOLUTE", "NM-SYNCHRONIZING-NETWORK", "LOAD-BALANCING-WEIGHT", "SPECIFIC-CHANNEL-REF", "MC-DATA-ASSIGNMENTS", "CRC-SEQUENCE-ID", "DOCUMENT-ELEMENT-SCOPES", "SO-AD-ROUTING-GROUP", "VALUE-TYPE-REF", "EVALUATED-VARIANT-SET", "DATA-TRANSFORMATION-KIND", "SOURCE-ADDRESS-MASK", "CONTROLLED-FID-REF", "SERIALIZER", "REQUIRED-OUTER-PORT-REF", "ENABLE-UPDATE", "MULTICAST-REF", "CLUSTER-DRIFT-DAMPING", "I-PDU-PORT", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT", "EXTERNAL-TRIGGERING-POINT", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "CODE-LABEL", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "UDP-NM-CLUSTER-REF", "NV-RAM-BLOCK-ELEMENT", "K-INIT-RESOURCE", "COUPLING-PORT-SCHEDULER", "DERIVATION", "DIAGNOSTIC-FREEZE-FRAME", "ALLOWED-I-PV-6-EXT-HEADER", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "TCP-IP-NDP-SLAAC-DELAY-ENABLED", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "MAC-UNICAST-ADDRESS", "CRYPTO-SERVICE-PRIMITIVE", "IKE-REKEY-TIME", "VARIABLE-INSTANCE", "SHARED-TIMER", "SERVICE-CONTROL", "DHCP-SERVER-CONFIGURATION", "RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS", "RESISTANT-TO-CHANGED-SW", "SO-CON-I-PDU-IDENTIFIER", "TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED", "DOCUMENTATION", "MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS", "ALL-CHANNELS-REF", "SWC-TO-SWC-SIGNAL", "SW-RECORD-LAYOUT-GROUP-AXIS", "COUPLING-PORT-FIFO", "CLIENT-SERVER-INTERFACE", "MAX-SOURCE-ELEMENT", "RESOURCE-NEEDS-REF", "EVENT-REFS", "TCP-IP-NDP-MIN-RANDOM-FACTOR", "CONTAINED-I-PDU-TRIGGERING-PROPSS", "FORBIDDEN-SIGNAL-PATH", "IP-IAM-REMOTE-SUBJECT", "GENERIC-TP", "CONFIRMATION-THRESHOLD", "ROLE-BASED-MC-DATA-ASSIGNMENT", "BSW-ENTRY-RELATIONSHIP-TYPE", "PNC-WAKEUP-CAN-ID-EXTENDED", "SLAVES", "PORT-ARG-VALUES", "PORT-BLUEPRINT-INIT-VALUE", "CAN-GLOBAL-TIME-DOMAIN-PROPS", "CONTAINED-AR-ELEMENT-REFS", "RESPOND-TO-RESET", "TIMING-VARIABLES", "MODE-SWITCHED-ACK-EVENT", "NM-IMMEDIATE-NM-CYCLE-TIME", "VARIABLE-DATA-PROTOTYPE-IREF", "PROPAGATION-DELAY", "CRC-OFFSET", "USED-SECONDARY-FID-REFS", "DLT-LOG-SINK-REF", "CODE-DESCRIPTORS", "MIN-SERVICE-POINT-ID", "MSR-QUERY-NAME", "SERVICE-FIND-TIME-TO-LIVE", "P-PORT-PROTOTYPE-IREF", "TEMPLATE-ARGUMENTS", "MAY-BE-UNCONNECTED", "MIN-ALIVE-CYCLE", "SOURCE-TRIGGER-REF", "PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "REMOTE-SUBJECT-REFS", "CAN-ENTER-EXCLUSIVE-AREA-REFS", "DEFAULT-LEASE-TIME", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "SW-VARIABLE-PROTOTYPE-REF", "CONTEXT-DESCRIPTION", "IMPL-INIT-VALUE", "SPECIFIC-CHANNELS", "CONTROL-PROVIDED-EVENT-GROUP-REF", "COM-CONFIGURATION-RX-TIME-BASE", "MULTICAST-THRESHOLD", "FILE-TYPE", "ETHERNET-NETWORK-CONFIGURATION", "K-RUN-RESOURCE", "SIGNAL-BASED-METHOD-REF", "PARAMETER-REF", "MINIMUM-SUPPORTED-UCM-VERSION", "SECURE-COM-PROPS-FOR-TCP-REFS", "DATA-PROTOTYPE-IREF", "SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING", "IDENTIFIABLE-REF", "DIAGNOSTIC-MEMORY-REF", "INSTANCE-IDENTIFIER", "STATUS-REF", "MIN-NUMBER-OF-TIME-QUANTA-PER-BIT", "DTCS", "GLOBAL-TIME-DOMAIN-REF-CONDITIONAL", "CAN-CONTROLLER-FD-ATTRIBUTES", "SECURITY-EVENT-PROPS-REF", "NETWORK-CONFIGURATION-REF", "TIME-HARDWARE-CORRECTION-THRESHOLD", "SCHEDULE-TABLES", "TP-CONFIGURATION", "STIMULUS-REF", "ENTRY", "APPLICATION-ERROR", "COM-CONTROL-CLASS-REF", "DIAGNOSTIC-ROUTINE-INTERFACE", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "AP-APPLICATION-ERROR", "DDS-FIELD-QOS-PROPS", "TIME-MARK", "SW-SYSTEMCONST-VALUE", "SOMEIP-SERVICE-INTERFACE", "BSW-MODULE-TIMING", "ETHERNET-CLUSTER-VARIANTS", "CRYPTO-NEED", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "COM-EVENT-GRANT-DESIGN", "PORT-BLUEPRINT-REF", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "OFS-DATA-ID-LISTS", "SERIALIZATION-TECHNOLOGY-REF", "ALIVE-TIMEOUT", "TARGET-DATA-PROTOTYPE-REF", "NAMED-CURVE-ID", "V-2-X-M-USER-NEEDS", "OPTION-NAME", "SERVICE-INSTANCES", "LINKERS", "SYSTEM", "EXPLICIT-INTER-RUNNABLE-VARIABLES", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "CONFIG-CLASS", "DETERMINISTIC-CLIENT", "IDENTITY-CERTIFICATE-AUTHORITY-REF", "HW-PIN-REF", "DLT-LOG-CHANNEL-REF", "PORT-PROTOTYPE-PROPS", "MC-GROUP-DATA-REF-SET-VARIANTS", "REMOTE-PORT-RANGE-START", "PDU-TRIGGERINGS", "PHM-CHECKPOINT-IREF", "INIT-VALUES", "ECUC-FLOAT-PARAM-DEF", "NM-VOTE-INFORMATION", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "LIN-UNCONDITIONAL-FRAME-REFS", "RECOVERY-ACTION-IREF", "UDP-NM-CLUSTER", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "SUPPORTING-MATERIAL", "BACKGROUND-EVENT", "IMPLEMENTATION-DATA-TYPE", "NM-RANGE-CONFIG", "NM-REPEAT-MESSAGE-TIME", "MESSAGE-LINK-POSITION", "TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED", "CONTEXT-MODE-DECLARATION-GROUP-REF", "SYSTEM-SIGNAL-REF", "DDS-TOPIC-ACCESS-RULE", "RULE-BASED-VALUES", "TRIGGER", "CYCLIC-TIMING", "POST-BUILD-CHANGEABLE", "CRYPTO-SIGNATURE-SCHEME", "MC-FUNCTION-DATA-REF-SET-CONDITIONAL", "METHOD-QOS-PROPSS", "REMOTE-IP-ADDRESS-REFS", "SECURITY-EVENT-DEFINITION-REF", "TIME-LEAP-HEALING-COUNTER", "TTCAN-CLUSTER-VARIANTS", "TEST-RESULT-REF", "PREFIX", "FLOW-CONTROL-REF", "DLT-MESSAGE-REF", "TYPE-OF-DTC-SUPPORTED", "REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF", "LONG-NAME-1", "TLV-DATA-ID-0-REFS", "STATIC-SOCKET-CONNECTION-REF", "NM-CLUSTER-COUPLINGS", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS", "ACCEPTED-REMOTE-CERTIFICATE-REF", "LOG-TRACE-LOG-MODE", "BUNDLED-CONNECTIONS", "ATTRIBUTE-TAILORINGS", "AGE-CONSTRAINT", "CONTAINER-I-PDU", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "MIN-DEADLINE", "FLEXRAY-PHYSICAL-CHANNEL", "EVENT-HANDLER-REF", "SECURED-AREA-OFFSET", "TRANSPORT-PROTOCOLS", "ANNOTATION", "SOFTWARE-CONTEXT", "FIRE-AND-FORGET-MAPPINGS", "LIN-PHYSICAL-CHANNEL", "NM-PDUS", "CAN-NM-CLUSTER", "MINIMUM-SUSTAINED-SIZE", "DIAGNOSTIC-SERVICE-INSTANCE-REF", "MAX-NO-NEW-OR-REPEATED-DATA", "STORING", "SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF", "ECUC-MODULE-CONFIGURATION-VALUES-REF", "STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR", "CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING", "COMMANDS", "TIMING-MODE-INSTANCE", "PDU-MAX-LENGTH", "REQUIRED-ENTRYS", "DLT-USER-NEEDS", "INDEX-DATA-TYPE-REF", "AUTOSAR-DATA-TYPE-REF", "COM-MANAGEMENT-MAPPINGS", "PACKAGE-REF", "SECURITY-EVENT-REPORT-INTERFACE", "INVERT-CONDITION", "PLCA-PROPS", "PHYSICAL-REQUEST-REF", "DTC-NUMBER", "STARTUP-OPTIONS", "INDICATOR-STATUS-NEEDS", "TIMEOUT-CR", "TRIGGERS", "MAC-MULTICAST-CONFIGURATION", "REQUIRED-TRIGGERS", "IS-GLOBAL", "DO-IP-LOGIC-ADDRESSS", "SECTION-NAME-PREFIXS", "REQUIRED-AR-ELEMENT-REFS", "TRIGGER-INTERFACE", "PAYLOAD-LENGTH-STATIC", "HAS-INTERNAL-STATE", "COMPILER", "MEMORY-RANGE-REFS", "LOCAL-PORT-RANGE-START", "EVALUATED-VARIANT-REFS", "MANUFACTURER-VALIDATION-ORDER-REF", "VFC-IREFS", "CYCLE-TIME-VALUE", "DIAGNOSTIC-AUTHENTICATION-CLASS", "SELECT-BLOCK-FOR-FIRST-INIT-ALL", "FLEXRAY-CLUSTER-VARIANTS", "VF", "AUTH-DATA-FRESHNESS-START-POSITION", "TRANSIT-TO-INVALID-EXTENDED", "INHIBIT-SOURCES", "WINDOW-SIZE-INIT", "TCP-IP-ICMP-V-4-TTL", "MAXIMUM-SUPPORTED-UCM-VERSION", "DIAGNOSTIC-SECURITY-ACCESS", "TRANSMISSION-ACKNOWLEDGE", "MC-DATA-INSTANCE-REFS", "REGULAR-EXPRESSION", "SEC-OC-JOB-MAPPINGS", "AUTOSAR-DATA-TYPE-REF-CONDITIONAL", "BUILD-ACTION-IO-ELEMENT", "RECEIVE-ACTIVITY", "SYSTEM-MEMORY-USAGE", "VENDOR-SPECIFIC-MODULE-DEF-REF", "GLOBAL-TIME-ETH-SLAVE", "TRIGGER-REF", "REQUIRED-CLIENT-SERVER-ENTRYS", "SW-FEATURE-DEFS", "SERVICE-INTERFACE-ELEMENT-MAPPING-REF", "BLUEPRINT-POLICY-NOT-MODIFIABLE", "SW-MC-BASE-TYPE", "REMARK", "DLT-LOG-CHANNEL-DESIGN", "RX-PDU-TRIGGERING-REF", "CONSTANT-VALUE-MAPPING-REFS", "MC-SW-EMULATION-METHOD-SUPPORT", "MAXIMUM-INTER-ARRIVAL-TIME", "SW-FEATURE-DESCS", "TARGET-PARAMETER-DATA-PROTOTYPE-REF", "FLEXRAY-CLUSTER", "TIMESTAMP-SUPPORT", "RUNS-INSIDE-EXCLUSIVE-AREA-REFS", "BSW-MODE-GROUP-REF", "METHOD-REF", "COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL", "ECUC-URI-REFERENCE-DEF", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "PROCESSOR-SPEED", "MEMORY-ALLOCATION-KEYWORD-POLICY", "MAX-BASE-TYPE-SIZE", "NM-REPEAT-MESSAGE-BIT-ENABLE", "SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE", "POSSIBLE-ERROR-REF", "AGE", "PRIORITY", "INCLUDED-VARIANT-REFS", "UPPER-MULTIPLICITY", "MODE-IN-SWC-INSTANCE-REF", "LOCAL-COMM-CONNECTOR-REF", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "VARIABLE-ACCESSED-FOR-DEBUG-REF", "META-DATA-ITEM-TYPE", "JOB-REF", "SOMEIP-EVENT-DEPLOYMENT", "DEST-SDG-REF", "ARTIFACT-CHECKSUM", "MONOTONY", "TCP-REMOTE-PORT", "DHCP-PROPS", "LIN-ERROR-RESPONSE", "SD-SERVER-EG-TIMING-CONFIGS", "AUTO-IP-PROPS", "CLIENT-SERVER-ANNOTATION", "AUTOSAR-VARIABLE-IN-IMPL-DATATYPE", "NV-DATA-INTERFACE", "CONTEXT-P-PORT-PROTOTYPE-REF", "BSW-MODULE-INSTANCE-REF", "SO-AD-CONFIG", "CERT-COMMON-NAME", "ICMP-V-6-PROPS", "COM-FIND-SERVICE-GRANT-DESIGN", "TCP-IP-NDP-DYNAMIC-MTU-ENABLED", "OFFSET", "INFO-TYPE", "TP-CONNECTION-CONTROL-REF", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS", "NUMERICAL-VALUE-VARIATION-POINT", "SYNC-DATA-ID-LIST", "PDU-REPLICATIONS", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "SERVICE-INTERFACE-DEPLOYMENT-REF", "COM-CONFIGURATION-ID", "UPPER-HEADER-BITS-TO-SHIFT", "NM-MSG-CYCLE-OFFSET", "EMPTY-SIGNAL", "MODIFICATIONS", "MAX-SLOTS", "MODULE-INSTANTIATIONS", "PLATFORM-ACTION-ITEM", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS", "ACTION-LIST-ITEM-REFS", "EVENT-HANDLER", "COM-MGR-USER-NEEDS", "META-DATA-LENGTH", "OPERATION-PROTOTYPE-IREF", "EXPRESSION", "VENDOR", "FM-FEATURE-SELECTION", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "GOVERNANCE-REF", "UDP-COLLECTION-BUFFER-SIZE-THRESHOLD", "SELECTIONS", "VLAN", "CONTAINER-TIMEOUT", "WINDOW-SIZE-VALID", "SEND-RESP-PEND-ON-TRANS-TO-BOOT", "IDSM-TRAFFIC-LIMITATION-REF", "SUPPORT-INFO-BYTE", "PNC-IDENTIFIER", "IDSM-MODULE-INSTANTIATION", "MODE-ACCESS-POINTS", "SECURE-COM-PROPS-FOR-DDS-REF", "CRYPTO-KEY-MANAGEMENT-NEEDS", "SERIALIZATION-TECHNOLOGY", "ENGINEERING-OBJECT", "CONTAINER-I-PDU-HEADER-BYTE-ORDER", "SECURE-COM-PROPS-FOR-UDP-REFS", "DDS-DOMAIN-RANGE", "PHM-RECOVERY-ACTION-INTERFACE", "MODE-CONDITION", "ALLOW-CONTENT-TYPE-CHANGE", "COUPLING-PORT-ROLE", "SYNC-INTERVAL", "PRIMITIVE-ATTRIBUTE-CONDITION", "REQUIRED-MINOR-VERSION", "ECUC-FUNCTION-NAME-DEF-VARIANTS", "MAX-DID-TO-READ", "CONSTRAINT-TAILORING", "IPV-6-ADDRESS", "I-PDUS", "PERSISTENCY-FILE", "LOGICAL-SUPERVISIONS", "DIAGNOSTIC-COMMON-ELEMENT-REF", "CLEAR-CONDITION-REFS", "NM-COM-CONTROL-ENABLED", "RECEIVER-REF", "SYSC-REF", "NRC-VALUE", "CONNECTED-INDICATORS", "REMOTE-NETWORK-ENDPOINT-REF", "DLT-ARGUMENT", "BR", "SW-HOST-VARIABLE", "TRANSITIONS", "TLV-ARGUMENT-REF", "HW-PIN", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "HEALTH-CHANNEL-EXTERNAL-MODE", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "STORAGE-CONDITIONS", "COMPU-METHOD", "SEARCH-INTENTION", "URI", "AUTO-AVAILABLE", "IKE-CIPHER-SUITE-NAME", "TCP-TP-CONFIG", "SPEED", "I-PDU-TRIGGERING-REF", "CONTEXT-SWC-PROTOTYPE-REF", "META-DATA-ITEM-SET", "WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF", "GETTER-CALL-SIGNAL-REF", "CRC-DOMAIN-NUMBER", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "PROTOCOL-VERSION", "DDS-RPC-SERVICE-DEPLOYMENT", "FR-CLUSTER-REF", "EXECUTION-CONTEXT-REFS", "DIAGNOSTIC-TEST-RESULT-REFS", "ENABLE-CONDITION-GROUP-REF", "DATA-CONSTR-RULES", "ETHERNET-CLUSTER", "PERSISTENCY-KEY-VALUE-DATABASE", "ECU-INSTANCES", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "FIELD-DEPLOYMENTS", "DOIP-CHANNEL-COLLECTION-REF", "FM-FEATURE", "PNC-SYNCHRONOUS-WAKEUP", "KEEP-ALIVE-PROBES-MAX", "DEFAULT-VALUE-ELEMENTS", "ROOT-DATA-PROTOTYPE-IN-CS-REF", "SW-COMPONENT-PROTOTYPE", "MESSAGE-ID", "BSW-EXCLUSIVE-AREA-POLICY", "MSR-QUERY-P-2", "DLT-CONTEXT", "SA-REKEY-TIME", "ERROR-TRACER-NEEDS", "TP-ADDRESS-EXTENSION-VALUE", "RESPONSE-GROUP-REF", "USER-DEFINED-TRANSFORMATION-DESCRIPTION", "MINIMUM-TIMER-GRANULARITY", "ARGUMENTSS", "CONTEXT-COMPOSITION-REF", "FUNCTION", "AFFECTED-FUNCTIONAL-CLUSTER", "SEC-OC-COM-PROPS-FOR-MULTICAST-REF", "SUCCESSOR-REF", "GLOBAL-SUPERVISION-ENTITYS", "CAN-COMMUNICATION-CONNECTOR", "TTCAN-COMMUNICATION-CONTROLLER", "REQUIRED-PACKAGE-ELEMENT-REFS", "ASSIGNED-ENTRY-REF", "CYCLIC-WRITING-PERIOD", "RULE-BASED-AXIS-CONT", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "MAX-DELTA-COUNTER", "HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS", "CRYPTO-OBJECT-TYPE", "CONTEXT-ID", "CHAPTERS", "MIN", "REQUIRED-FIBEX-ELEMENT-REFS", "BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL", "CAN-BE-INVOKED-CONCURRENTLY", "OFFSET-TIME-DOMAIN-REF", "UCM-SOFTWARE-VERSION", "M", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "FALSE-ACTION-LIST-REF", "BSW-INTERNAL-TRIGGERING-POINT", "PHYSICAL-DIMENSION-MAPPINGS", "DATA-LENGTH", "CS-ERROR-REACTION", "NV-BLOCK-DATA-MAPPING", "SIGNAL-AGE", "ROLE-BASED-DATA-TYPE-ASSIGNMENT", "DYNAMIC-SLOT-IDLE-PHASE", "NM-NODE-ID", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "ENVIRONMENT-VARIABLES", "DLT-MESSAGE-REF-CONDITIONAL", "DIAGNOSTIC-RESPONSE-ON-EVENT", "TYPE-DEFINITION", "WAKEUP-RX-LOW", "HW-CATEGORY-REF", "FM-FEATURE-MAP-ELEMENT", "DYNAMICALLY-ASSIGNED", "SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS", "LOG-SINK-REFS", "REST-ELEMENT-DEF", "LIN-MASTER-VARIANTS", "LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS", "NM-SYNCHRONIZATION-POINT-ENABLED", "BGCOLOR", "MEMORY-DESTINATION-REF", "REFERENCE-TAILORING", "CONSUMED-SERVICE-INSTANCE", "CALIBRATION-PARAMETER-VALUE-SET", "EXTERNAL-SYNC", "PARAMETER-SW-COMPONENT-TYPE", "SOMEIP-TP-CHANNEL", "OFNS-DATA-ID-LIST", "NM-SCHEDULE-VARIANT", "TARGET-I-PDU", "FIRST-MODE-REFS", "INDICATOR-REF", "NM-CBV-POSITION", "DIAGNOSTIC-PERIODIC-RATE", "DO-IP-ROUTING-ACTIVATIONS", "ENCAPSULATED-ENTRY-REF", "MAX-FR-IF", "P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "REQUIRED-PACKAGE-ELEMENT-REF", "MEMORY-DESTINATION-REFS", "MASK", "EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS", "USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS", "OS-TASK-EXECUTION-EVENT", "SUPPORTED-FEATURES", "COUNTER-DECREMENT-STEP-SIZE", "RUNNABLE-REF", "FIFO-DEPTH", "ACK-REQUEST", "ECUC-VALUE-CONFIGURATION-CLASS", "CONFIG-ELEMENT-DEF-LOCAL-REF", "INITIAL-OFFER-BEHAVIOR", "TRANSITION-REFS", "POSSIBLE-AP-ERROR-SET-REF", "KIND", "DISABLED-IN-MODE-IREFS", "RX-NM-PDU-REF", "DATA-RECEIVE-POINT-BY-VALUES", "SERVICE-ELEMENT-SECURE-COM-CONFIG-REF", "TROUBLE-CODE-OBD-REF", "SEGMENT-REF", "ROUTINE-REF", "RAW-DATA-STREAM-DEPLOYMENT", "POST-BUILD-VARIANT-CRITERION-VALUE", "BLOCK-IF-STATE-ACTIVE-CP-REF", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "SERVICE-POINT-SYMBOL-POST", "SW-RECORD-LAYOUT-GROUP-INDEX", "FRAME-PORT-REFS", "RPT-WRITES", "FORMULA-CAPTION", "INTERNAL-CONSTRS", "SOMEIP-FIELD", "LC-OBJECT-REF", "DDS-SERVICE-VERSION", "ASSERTIONS", "TRANSIENT-FAULT", "ELEMENTS", "DATA-ELEMENTS", "CLIENT-SERVER-OPERATION-REF", "TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF", "DDS-PROVIDED-SERVICE-INSTANCE", "TP-TECHNOLOGY", "MIN-DATA-LENGTH", "POSSIBLE-ERROR-REACTIONS", "DCM-I-PDU", "DIAGNOSTIC-REQUEST-DOWNLOAD", "SEQUENTIAL-INSTRUCTIONS-BEGIN", "MEMORY-SECTION-LOCATIONS", "ALLOW-HALT-DUE-TO-CLOCK", "ACTION-LIST-EXECUTION", "SEQUENCE-COUNTER-JUMP-WIDTH", "L-1", "UPDATE-KIND", "LIN-CONFIGURABLE-FRAME", "STARTUP-CONFIG", "ACCESSED-VARIABLE", "PARAMETER-INSTANCE", "DESTINATION-URI-POLICY", "RETURN-SIGNAL-TRIGGERING-REF", "P", "MASTERED-TRIGGER-REF", "TD-EVENT-I-PDU", "PHM-ACTION-LIST", "MAX-BYTES-IN-INTERVAL", "MODE-DECLARATION-REF", "ADDRESS-RANGE-UPPER-BOUND", "OBD-PID-SERVICE-NEEDS", "USER-DEFINED-FIELD-DEPLOYMENT", "SLAVE-REF", "BINARY-MANIFEST-PROVIDE-RESOURCE", "SECOND-ELEMENTS", "RELEASED-TRIGGER-POLICYS", "RPT-HOOKS", "USE-VEHICLE-IDENTIFICATION-SYNC-STATUS", "ROLE-BASED-PORT-ASSIGNMENT", "I-SIGNAL-PROPS", "ECUC-INSTANCE-REFERENCE-DEF", "GLOBAL-ELEMENT", "INTERFACE-REF", "DIAGNOSTIC-AGING", "LIN-CLUSTER-VARIANTS", "BUILD-ACTION-ENVIRONMENTS", "ENVIRONMENT-DATA-CAPTURE", "LIFE-CYCLE-INFO-SET", "EXCLUSIVE-AREA-POLICYS", "SERVICE-INTERFACE-REFS", "ACTIVATION-REASONS", "PREDECESSOR-FIFO-REF", "MODE-DECLARATION-GROUP", "PORT-BLUEPRINT-MAPPINGS", "ASSIGN-FRAME-ID-RANGE", "SW-CARB-DOC", "PLCA-LOCAL-NODE-COUNT", "MC-GROUP-DATA-REF-SET-CONDITIONAL", "DTC-SETTING-PARAMETER", "SWC-FLAT-SERVICE-DEPENDENCY-REF", "STATIC-SLOT-DURATION", "ROM-BLOCK-INIT-VALUE", "RUNNABLE-ENTITY-ARGUMENT", "ACL-SCOPE", "RELATED-TRACE-ITEM-REF", "INTERPOLATION-ROUTINE-MAPPING-SET", "CRYPTO-CERTIFICATE-INTERFACE", "SOMEIP-TP-CONNECTION", "ATTRIBUTE-VALUES", "QUEUE-SIZE", "TIME-SEG-2", "EVALUATED-ELEMENT-REFS", "CUSTOM-SPECIFICATION-REFS", "NETWORK-ENDPOINT-ADDRESSES", "MSG-ID-MASK", "TRIGGER-DIRECT-IMPLEMENTATIONS", "WAKE-UP-CHANNEL", "SYNCHRONIZED-SLAVE-TIME-BASE", "ARGUMENT-MAPPINGS", "REVERSED-TP-SDU-REF", "RECORD-ELEMENT-MAPPINGS", "TD-EVENT-FRAME-ETHERNET", "POST-BUILD-VARIANT-CONDITION", "MESSAGE-SOURCE-FILE", "TRANSFER-REFS", "RX-TIMEOUT-TIME", "PHM-CONTRIBUTION-REFS", "COUNT-PROFILE", "COUPLING-PORT-STARTUP-ACTIVE-TIME", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "MAXIMUM-INTENDED-BINDING-TIME", "APPL-WATCHDOG-LIMIT", "START-POSITION", "WAKEUP-RX-IDLE", "TD-PDU-TRIGGERING-FILTER-REF", "MINIMUM-START-INTERVAL", "FIBEX-ELEMENT-REF", "NESTED-ELEMENT-REFS", "CUSTOM-SUB-FUNCTION-NUMBER", "CLEAR-EVENT-BEHAVIOR", "JOB-PRIMITIVE-REF", "MODE", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "MEASURED-STACK-USAGE", "PNC-GROUP-REF", "NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS", "CAN-CONTROLLER-CONFIGURATION", "J-1939-NM-ECU", "PROCESSORS", "SW-IS-VIRTUAL", "METHODS", "SUPERVISED-ENTITY-IREF", "ROOT-ARGUMENT-DATA-PROTOTYPE-REF", "C-CODE", "SW-SYSTEMCONST-REF", "INTENDED-FILENAME", "V-LAN-REFS", "MANAGED-COUPLING-PORTS", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS", "TX-PDU-POOL-REF", "KEY-REF", "BSW-INTERRUPT-ENTITY", "POSSIBLE-AP-ERROR-REFS", "APPLICATION-RECORD-ELEMENT", "UCM-REF", "DATA-CONSTR", "ACCEPTED-REMOTE-CERTIFICATE-REFS", "RPT-EXECUTABLE-ENTITY", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "CONCRETE-CLASS-TAILORING", "HW-ELEMENT", "REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF", "SOURCE-FRAME-REF", "USER-DEFINED-PHYSICAL-CHANNEL", "TIMING-CONDITION-FORMULA", "MAPPED-SWC-SERVICE-DEPENDENCY-IREF", "SERVICE-POINT-SYMBOL-PRE", "CPP-TEMPLATE-ARGUMENT", "TARGET-PROVIDED-OPERATION-REF", "TCP-IP-IP-REASSEMBLY-BUFFER-COUNT", "ALIVE-SUPERVISION", "BSW-RESOLUTION", "REMOTE-SERVER-CONFIG", "CONTROL-PNC-REF", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPES", "FREEZE-FRAME-CONTENT-WWH-OBD-REF", "MODE-SENDER-POLICYS", "PNC-MAPPING", "DLT-LOG-SINK-REFS", "TFOOT", "DIAGNOSTIC-EXTRACT-REF", "ACCESSED-MODE-GROUPS", "MODE-DRIVEN-FALSE-CONDITIONS", "TIMING-ARGUMENTS", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "TIME-BASES", "SDG-REFERENCE", "I-SIGNAL-TRIGGERINGS", "TRANSMISSION-MODE-CONDITION", "PORT-SCHEDULER", "MAX-BS", "SW-CALIBRATION-NOTES", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "ARBITRATION", "DIAGNOSTIC-CONTRIBUTION-REF", "IMPLIED-OPERATION-REFS", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "MODE-IREF", "ETH-IP-PROPS-REF", "ACTIVATION-SWITCH", "METADATA-PROTECTION-KIND", "LOCAL-SUPERVISION-REF", "OUT-MEASURMENT-SET", "COLD-START-ATTEMPTS", "TLV-DATA-ID-DEFINITIONS", "SW-DIAGNOSTICS-NOTESS", "SETTER-CALL-REF", "LIN-SLAVE-VARIANTS", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "VLAN-MEMBERSHIPS", "VALUE-SPEC", "TIME-SYNC-MODULE-INSTANTIATION", "GET", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "PORT-PROTOTYPE-SLUG-FRAGMENT", "RESOURCE-NEEDS-REFS", "IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "MINIMUM-SUPPORTED-UCM-MASTER-VERSION", "CONNECTORS", "USER-DEFINED-CLUSTER-CONDITIONAL", "SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE", "I-PDU-TRIGGERING-REF-CONDITIONAL", "TCP-OPTION-FILTER-LISTS", "TARGET-APPLICATION-RECORD-ELEMENT-REF", "TIME-PERIOD", "LOG-TRACE-PROCESS-DESC", "CRYPTO-PROVIDER", "INDUSTRY-GROUP", "COMPU-SCALE", "FLEXRAY-NM-CLUSTER", "TEST-ID-REF", "BSW-PARAMETER-POLICY", "ROLE-BASED-RESOURCE-DEPENDENCY", "LOCAL-ID", "PROVIDED-SERVICE-PORT-IREF", "DATA-FORMAT-TAILORING", "NUMBER-OF-STATIC-SLOTS", "BSW-MODULE-ENTITY-REF", "GLOBAL-TIME-PDU-TRIGGERING-REF", "BYTE", "SW-GENERIC-AXIS-PARAM-TYPE", "REQUEST-VEHICLE-INFORMATION-CLASS-REF", "TABLE", "TARGET-SIGNAL-REF", "DIAGNOSTIC-ACCESS-PERMISSION", "TD-EVENT-MODE-DECLARATION", "DESIGN-REFS", "TTL", "IMPLEMENTED-IN-REF", "NM-MAIN-FUNCTION-PERIOD", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL", "CONTEXT-PORT-PROTOTYPE-REF", "LOG-TRACE-DEFAULT-LOG-LEVEL", "TD-EVENT-VFB-REFERENCE", "EXCLUSIVE-AREA", "MEMORY-ENTRY-STORAGE-TRIGGER", "SOCKET-OPTIONS", "SETTER-OPERATION-REF", "BSW-DISTINGUISHED-PARTITION", "SW-CODE-SYNTAX", "ETHERNET-CLUSTER-CONDITIONAL", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "MAXIMUM-DYNAMIC-PAYLOAD-LENGTH", "DATA-ELEMENT-IREFS", "EVENT-GROUP-IDENTIFIER", "CALL-SIGNAL-REF", "BUS-SPECIFIC-NM-ECU", "INTERNAL-TRIGGERING-POINTS", "USE-AUTO-VALIDATION-AT-SHUT-DOWN", "DYNAMIC-PART-ALTERNATIVES", "RPT-CONTAINERS", "PARAMETER-DATA-PROTOTYPE-IREF", "OFFSET-CORRECTION-OUT", "EXCLUSIVE-AREAS", "NM-GLOBAL-COORDINATOR-TIME", "NM-USER-DATA-ENABLED", "TYPE-OF-EVENT-COMBINATION-SUPPORTED", "CP-SOFTWARE-CLUSTER-RESOURCE-REF", "J-1939-DCM-I-PDU", "NM-USER-DATA-OFFSET", "ROUGH-ESTIMATE-STACK-USAGE", "MAX-REQUEST-BYTES", "RECEIVER-IREFS", "CLIENT-INTENT", "PRE-ACTIVATE-IREFS", "REFERENCE-BASES", "VARIABLE-REF", "ECU-INSTANCE-REF-CONDITIONAL", "CAN-ID-TO-CAN-ID-MAPPINGS", "SECOND-TRIGGER-REF", "ACTION", "DOC-REVISION", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "HW-ATTRIBUTE-DEFS", "MACHINE-TIMING", "TCP-UDP-CONFIG", "ICMP-V-4-PROPS", "INTENDED-PARTIAL-INITIALIZATION-COUNT", "RESPONSES", "DO-IP-SOURCE-ADDRESS-REF", "MAX-RPT-EVENT-ID", "SYNC-LOSS-THRESHOLD", "FLEXRAY-CLUSTER-CONDITIONAL", "RECOVERY-NOTIFICATION-TIMEOUT", "TIMING-EVENT", "PROVIDED-MODE-GROUP-REF", "CONTEXT-OPERATION-REF", "DNS-SERVER-ADDRESS", "SOMEIP-EVENT-PROPS", "DIAGNOSTIC-DATA-ELEMENT-REF", "AGING-ALLOWED", "SERVICE-INSTANCE-COLLECTION-SET", "MAXIMUM-DISTANCES", "POSITIVE-INTEGER-VALUE-VARIATION-POINT", "COMPU-INTERNAL-TO-PHYS", "SW-TEST-DESCS", "DIAGNOSTIC-VALUE-ACCESS", "DEF-CALPRM-SET", "MODIFIED-DATAS", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "TCP-IP-NDP-MAX-RANDOM-FACTOR", "REQUIRE-RESOURCES", "L-GRAPHIC", "COUPLING-PORT-CONNECTIONS", "OP", "GRANT-REF", "ROUTING-ACTIVATION-TYPE", "VALUE-ACCESS", "LOG-TRACE-DEFAULT-LOG-THRESHOLD", "ECU-PARTITION-REF", "BASE-IS-THIS-PACKAGE", "RECEIVER-INTENT", "FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR", "ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS", "DEVELOPMENT-ERROR", "LOGICAL-OPERATOR", "TRANSFORMATION-I-SIGNAL-PROPSS", "ETH-TCP-IP-PROPS", "REG-REQUIRES-STABILITYS", "TD-EVENT-SERVICE-INSTANCE-METHOD", "TLS-DEPLOYMENT", "TARGET-MODE-DECLARATION-REF", "SECOND-MODE-GROUP-REF", "VARIABLE-ACCESS", "RP-IMPL-POLICY", "MAX-NUMBER-OF-CONNECTIONS", "LATENCY-TIMING-CONSTRAINT", "CONTEXT-P-PORT-REF", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "DIAGNOSTIC-TEST-RESULT-REF", "PROVIDED-SERVICE-INSTANCE", "PORT-INSTANTIATION-BEHAVIOR", "MODE-DECLARATION-MAPPINGS", "STATIC-PARTS", "SERVICE-ELEMENT-MAPPING-REFS", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "TLS-CIPHER-SUITE", "CAN-COMMUNICATION-CONTROLLER", "IKE-AUTHENTICATION-METHOD", "J-1939-TP-CONFIG", "RPT-SERVICE-POINT-PRE-REF", "MODE-DRIVEN-TRUE-CONDITIONS", "READ-SCALING-DATA-CLASS-REF", "ECU-ID", "TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "HEALTH-CHANNEL-ARGUMENT-REF", "DIAGNOSTIC-UPLOAD-INTERFACE", "PROG-CODE", "HEAP-USAGES", "DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL", "TOOL-VERSION", "MODULE-DESCRIPTION-REF", "EXECUTION-CONTEXT", "MAC-MULTICAST-ADDRESS-REFS", "EXECUTION-ERROR-REF", "AP-APPLICATION-ENDPOINT", "DOIP-CONNECTION-REF", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "PROVIDED-CLIENT-SERVER-ENTRYS", "NM-BUSLOAD-REDUCTION-ENABLED", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY", "TRANSMISSION-TRIGGER", "POLICY-ACTION", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "DO-IP-TARGET-ADDRESS-REFS", "SW-AXIS-CONTS", "SECURED-I-PDU", "EVENT-CLEAR-ALLOWED", "IGNORE-ORDER-ALLOWED", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "SW-SYSTEMCONST", "SSP-OFFSET", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "DIAGNOSTIC-EVENT-PORT-MAPPING", "DEFAULT-VLAN-REF", "FIELD-REPLY-TOPIC-NAME", "ASSIGNED-CONTROLLER-REF", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF", "OUTER-PORT-REF", "BLUEPRINT-DERIVATION-GUIDE", "EXECUTION-TIME-CONSTRAINT", "INNER-GROUP-IREFS", "NOMINAL", "PROVIDED-COM-SPECS", "TIMING-DESCRIPTION-REF", "TX-NM-PDU-REFS", "MODE-GROUP-REF", "DIAGNOSTIC-SECURITY-LEVEL", "NV-BLOCK-DATA-MAPPINGS", "TYPE-APPROVAL", "REQUESTER-IREF", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "I-SIGNAL-GROUP", "SOFTWARE-PACKAGE-STEP", "ENTITYS", "SCHEDULING-POLICY", "CLIENT-SERVER-ARRAY-ELEMENT-MAPPING", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "CLASS-CONTENT-CONDITIONAL", "TCP-IP-NDP-PREFIX-LIST-SIZE", "UDS-DTC-NUMBER", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "VALUE-TYPE-TREF", "TP-CONNECTIONS", "DIAGNOSTIC-DATA-ELEMENT", "EXPIRED-SUPERVISION-CYCLE-TOL", "I-SIGNAL-I-PDU", "REQUEST-DATA-SIZE", "THRESHOLD-SIZE", "AUTOSAR-ENGINEERING-OBJECT", "EVENT-OCCURRENCE-KIND", "DNS-SERVER-ADDRESSES", "NESTED-ELEMENTS", "SENDER-REC-RECORD-ELEMENT-MAPPING", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL", "ID", "OPERATION-MODE", "SERVICE-INTERFACE-MAPPING-SET", "NM-REPETITION-CYCLE", "USER-DEFINED-GLOBAL-TIME-MASTER", "NM-NETWORK-TIMEOUT", "CONTROL-OPTION-RECORD-PRESENT", "INTERPOLATION-ROUTINE", "RESPONSE-ON-SECOND-DECLINED-REQUEST", "MACHINE-DESIGN", "SYNCHRONIZATION-POINT-CONSTRAINT", "ECUC-VALUES", "I-SIGNAL-PORT", "TARGET-P-PORT-REF", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "IPV-6-PROPS", "TRANSMISSION-MODE", "PRIMITIVE-FAMILY", "REVISION-LABEL", "UDP-LOCAL-PORT", "LOAD-BALANCING-PRIORITY", "USED-LANGUAGES", "ECUC-STRING-PARAM-DEF", "HW-PIN-GROUP-REF", "SEC-OC-SECURE-COM-PROPS", "RESPONSE-REF", "FILE-NAME", "ARRAY-VALUE-SPECIFICATION", "VALUE-AXIS-DATA-TYPE-REF", "EVENT-KIND", "FLEXRAY-TP-ECU", "RTE-PLUGIN-PROPS", "COMM-CONTROLLER-REF", "SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL", "DEPENDENT-SOFTWARE-CLUSTER-REF", "FALL-BACK-INTERNAL", "SW-VALUE-BLOCK-SIZE-MULTS", "BSW-SCHEDULER-NAME-PREFIX", "VERIFY-REF", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "ITEM", "SUPERVISION-CONDITION", "WRITTEN-NV-DATA", "NOTIFIER", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "BSW-MODE-SWITCHED-ACK-EVENT", "COLSPEC", "TCP-IP-IP-NUM-REASS-DGRAMS", "CONTEXTS", "SWC-TO-APPLICATION-PARTITION-MAPPINGS", "ETH-TCP-IP-ICMP-PROPS", "SIGNAL-SERVICE-TRANSLATION-PROPSS", "SW-DATA-DEF-PROPS-CONDITIONAL", "MODE-REQUEST-TYPE-MAP", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "BUS-OFF-RECOVERY", "SUPPORT-DIRTY-FLAG", "TARGET-REFS", "APPLICATION-VALUE-SPECIFICATION", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "IUMPR-REFS", "OFFSET-CORRECTION-ADAPTION-INTERVAL", "SHALL-RUN-ON-REFS", "SECURITY-DELAY-TIME-ON-BOOT", "ENABLE-DISCOVERY-PROTECTION", "CLAIMED-FUNCTION-GROUP-REFS", "AUTHENTICATION-ROLE-REF", "META-DATA-FIELDS", "PORT-PROTOTYPE-BLUEPRINT-MAPPING", "NEXT-HIGHER-CERTIFICATE-REF", "SW-MAX-AXIS-POINTS", "SOMEIP-SERVICE-VERSION", "VIEW-MAP-SET", "VARIABLE-ACCESS-REF", "CRYPTO-PROVIDER-DOCUMENTATION-REF", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "GLOBAL-TIME-FR-MASTER", "NM-USER-DATA-LENGTH", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "CAN-FRAME-TX-BEHAVIOR", "RESET-TO-DEFAULT-SUPPORTED", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "FLEXRAY-TP-PDU-POOL", "MODULE-INSTANTIATION-REFS", "R-PORT-IN-COMPOSITION-INSTANCE-REF", "DEFAULT-ENDIANNESS", "VENDOR-SPECIFIC-MODULE-DEF-REFS", "POSITION-IN-TABLE", "RUNNABLE-ENTITY", "KEY-SIZE", "FM-FEATURE-DECOMPOSITION", "ADMIT-WITHOUT-MESSAGE-ID", "CLIENT-SERVER-RECORD-ELEMENT-MAPPING", "GENERAL-PURPOSE-CONNECTION", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "CRYPTO-CERTIFICATES", "FM-FEATURE-MAP", "FACTOR-SI-TO-UNIT", "TIME-PASSED-THRESHOLD", "TCP-RETRANSMISSION-TIMEOUT", "FAILED-SUPERVISION-CYCLES-TOLERANCE", "ACL-OPERATION", "INDIRECT-API", "TTCAN-PHYSICAL-CHANNEL", "DIAGNOSTIC-AUTHENTICATION-REF", "CAN-FRAME-RX-BEHAVIOR", "SIGNAL-GROUP-REF", "TRANSFORMER-CHAIN-REFS", "ECU-SCOPE-REFS", "ASSIGNED-FRAME-TRIGGERING-REF", "RESOURCE-GROUP-REFS", "DLT-APPLICATION", "FLEXRAY-AR-TP-NODE", "HARDWARE-PLATFORM", "APPLICATION-CONTEXT-REF", "CLEAR-FROM-VALID-TO-INVALID", "HTTP-TP", "CRYPTO-KEY-DESCRIPTION", "CHECKPOINT-REF", "DOIP-INTERFACES", "SOURCE-EVENT-REFS", "ETH-GLOBAL-TIME-DOMAIN-PROPS", "PREDEFINED-TEXT", "SUPPORTED-TIMER-GRANULARITY", "PERSISTENCY-REDUNDANCY-CRC", "BINARY-MANIFEST-RESOURCE-DEFINITION", "USED-SECONDARY-FID-REF", "TIMING-MODE-REF", "PACKAGER-SIGNATURE-REF", "REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF", "PORT-PROTOTYPE-BLUEPRINT-REF", "DEFAULT-PERIOD-END", "APPLICATION-DESCRIPTION", "TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR", "NM-WAIT-BUS-SLEEP-TIME", "REFERRABLE-REF", "GLOBAL-IN-PACKAGE-REFS", "FM-FEATURE-RESTRICTION", "CYCLE", "DO-IP-TESTER-ROUTING-ACTIVATION-REFS", "TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT", "DO-IP-ACTIVATION-LINE-NEEDS", "DEPENDS-ONS", "UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE", "TIME-BASE-JITTER", "NAME", "HW-COMMUNICATION-PORT-REF", "TD-EVENT-SERVICE-INSTANCE-EVENT", "LOCAL-COM-ACCESS-CONTROL-ENABLED", "SOURCE-ADDRESS-CODE", "CONSISTENCY-NEEDSS", "REQUIRED-AR-ELEMENT-REF", "COLLECTED-INSTANCE-IREFS", "SEND-ACTIVITY", "BSW-MODULE-DESCRIPTION-REF", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "IMPLEMENTED-ENTRY-REF", "CONTEXT-ROOT-COMPOSITION-REF", "FREE-FORMAT", "ECUC-REF", "NM-CAR-WAKE-UP-RX-ENABLED", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "TIME-VALUE-VALUE-VARIATION-POINT", "SOMEIP-EVENT-GROUP", "REF", "OBD-DTC-NUMBER", "FM-COND", "SUPERVISION-ENTITY", "WAKEUP-FRAME-REFS", "DO-IP-ROUTING-ACTIVATION", "NUMBER-OF-RESTART-ATTEMPTS", "FLEXRAY-AR-TP-CONNECTION", "I-SIGNAL-REFS", "TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED", "DATA-TYPE-MAPPING-SET-REF", "DDS-METHOD-DEPLOYMENT", "END-TO-END-TRANSFORMATION-DESCRIPTION", "CRYPTO-SERVICE-KEY", "BSW-MODULE-ENTRY-REF", "MAXIMUM-SELECTED-BINDING-TIME", "SDXF", "MEASURED-HEAP-USAGE", "TARGET-MODE-GROUP-REF", "SW-RECORD-LAYOUT-V-INDEX", "GLOBAL-TIME-SUB-DOMAINS", "BAUDRATE", "LOCAL-ENDPOINT-CONFIG", "ELEMENT-REF", "MINIMUM-DELAY", "CONTEXT-BSW-IMPLEMENTATION-REF", "MINIMUM-MEMORY-CONSUMPTION", "SW-SYSTEMCONSTANT-VALUE-SET-REFS", "CONFIGURED-NAD", "RECOVERY-NOTIFICATION", "NOTIFIER-DATA-ELEMENT-REF", "PERSISTENCY-FILE-ARRAY", "REQUEST-UPLOAD-CLASS-REF", "PHYSICAL-CHANNEL-REF", "MAC-MULTICAST-GROUPS", "SWC-TO-ECU-MAPPING-CONSTRAINT", "ARRAY-ELEMENT-REF", "REST-NUMBER-PROPERTY-DEF", "I-SIGNAL-TO-I-PDU-MAPPING", "EXPECTED-ENTRYS", "INVERT", "DATA-PROTOTYPE-REF", "SW-INTERPOLATION-METHOD", "STATUS-ID", "WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST", "SOCKET-ADDRESS-REF", "TCP-IP-ARP-TABLE-ENTRY-TIMEOUT", "FIGURE-CAPTION", "I-PV-6-EXT-HEADER-FILTER-SET", "IP-SEC-RULES", "TP-ADDRESSS", "CONDITIONAL-CHANGE-NAD", "SWC-MODE-SWITCH-EVENT", "BLOCK-STATES", "TYP", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "SHORT-LABEL-PATTERN", "CRYPTO-TRUST-MASTER-INTERFACE", "DDS-RPC-SERVICE-REF", "SYNC-DATA-ID-LISTS", "NM-HANDLE-REF", "TLV-DATA-ID", "MAX-NUMBER-OF-ALLOWED-UPDATES", "SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING", "SCALING-INFO-SIZE", "ENHANCED-MODE-API", "COMPONENT-IREFS", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "LOGICAL-EXPRESSION-ARGUMENT-REF", "NV-PROVIDE-COM-SPEC", "NM-INSTANCE-ID", "FIRST-DATA-PROTOTYPE-REF", "SW-COMP-TO-ECU-MAPPING-REFS", "RESOURCE-CONSUMPTION", "SAFETY-POLICY", "TEXT-VALUE-SPECIFICATION", "SYNCHRONIZED-TRIGGERS", "TRANSMIT-CANCELLATION", "DIAGNOSTIC-CONDITION-INTERFACE", "THRESHOLD", "SECOND-TO-FIRST-DATA-TRANSFORMATION-REF", "DIAGNOSTIC-PARAMETER", "IP-SEC-CONFIG", "NM-REPEAT-MSG-INDICATION-ENABLED", "NETWORK-REPRESENTATION-PROPS", "RUNNABLES", "ROOT-AUTOSAR-DATA-PROTOTYPE-REF", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "DLT-LOG-CHANNELS", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "OUT-MEASUREMENT-SET", "SERVICE-TABLES", "PARTICIPATING-J-1939-CLUSTER-REF", "SETTER-REF", "GETTER-RETURN-REF", "BASE-COMPOSITION-REF", "SENDING-NODE-REF", "SUBSCRIBE-EVENTGROUP-RETRY-MAX", "R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "DATA-CONSTR-RULE", "CONTEXT-SWC-PROTOTYPE-REFS", "CUSTOM-SDG-DEF-REF", "VALIDATION-CONFIGURATION", "READ-LOCAL-VARIABLES", "SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "NOTIFIED-STATUSS", "GLOBAL-TIME-CAN-SLAVE", "PHYSICAL-LAYER-TYPE", "CONTEXT-APPLICATION-RECORD-ELEMENT-REF", "HEALTH-CHANNEL-IREF", "DYNAMIC-PART", "TRANSMISSION-PROPS", "EXPECTED-ALIVE-INDICATIONS", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "CHECKPOINT-FINAL-REF", "GRANT-REFS", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "REMOTE-UNICAST-ADDRESSS", "EXPECTED-ALIVE-CYCLE", "USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT", "HW-ELEMENT-REF", "SW-DATA-DEPENDENCY", "MINISLOT-ACTION-POINT-OFFSET", "DYNAMIC-ACTION-REF", "NOTIFIER-REF", "TRANSMIT-PDU-REFS", "AUTOSAR-VARIABLE-INSTANCE", "SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF", "RID-NUMBER", "ECU-RESOURCE-ESTIMATION", "CONTAINED-AR-ELEMENT-REF", "MAX-SUPPORTED-DID-LENGTH", "DPD-ACTION", "KEEP-ALIVE-INTERVAL", "RPT-EXECUTION-CONTROL", "DATA-TRANSFER-CLASS-REF", "GLOBAL-TIME-MASTERS", "PORT-INTERFACE-MAPPINGS", "MODE-ELEMENTS", "GLOBAL-RESOURCE-ID", "CAS-RX-LOW-MAX", "GROUP-IDENTIFIER", "TRANSFORMATION-TECHNOLOGYS", "KEEP-ALIVES", "THRESHOLD-NUMBER", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "MAX-WITHOUT-CLOCK-CORRECTION-FATAL", "EXECUTABLE-ENTITY-REFS", "ROOT-COMPOSITION", "LOGIC-ADDRESS", "COMPU-SCALES", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "ALLOW-UNAUTHENTICATED-PARTICIPANTS", "FREEZE-FRAMES", "DELIVERY-ARTIFACTS", "RUNTIME-PORT-CONFIGURATION", "STOP", "REQUIRED-EVENT-GROUPS", "ONE-EVERY-N", "WWH-OBD-DTC-CLASS", "SERVER-NAME-IDENTIFICATION", "DIAGNOSTIC-STORAGE-CONDITION-REF", "MODES", "GETTER-OPERATION-REF", "COMPOSITE-TYPE-MAPPINGS", "SERVICE-SW-COMPONENT-TYPE", "TARGET-I-PDU-REF", "DIAGNOSTIC-EVENTS", "NM-CAR-WAKE-UP-FILTER-ENABLED", "SW-MIN-AXIS-POINTS", "REMOTE-CLIENT-CONFIG", "HARDWARE-ELEMENT-REF", "TCP-IP-NDP-PACKET-QUEUE-ENABLED", "DIAGNOSTIC-ADDRESSS", "PATTERN-LENGTH", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "COMPOSITION-SW-COMPONENT-TYPE", "CONFIDENCE-INTERVALS", "OBJECT-DEFINTION-REF", "SERVICE-INTERFACE-TRIGGER-MAPPING", "COMPU-INVERSE-VALUE", "RECEPTION-POLICYS", "DEFAULT-VALUE", "CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "MODE-REQUEST-TYPE-MAPS", "TARGET-VARIABLE-ACCESS-REF", "RELATIONS", "ASSOCIATED-COM-I-PDU-GROUP-REF", "MSR-QUERY-P-1", "OPERATION-CYCLE", "TCP-FIN-WAIT-2-TIMEOUT", "FEATURE-MODEL-REF", "TLS-JOB-MAPPINGS", "DATA-TYPE-MAPPING-SET", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "SECURITY-EVENT-DEFINITION-REF-CONDITIONAL", "TARGET-VARIABLE-DATA-PROTOTYPE-REF", "CLIENT-SERVICE-MINOR-VERSION", "MINIMUM-SEPARATION-TIME", "MONITORED-IDENTIFIER-REF", "NM-READY-SLEEP-COUNT", "PDELAY-LATENCY-THRESHOLD", "MC-FUNCTION-DATA-REF-SET-VARIANTS", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "READ", "ALLOWED-TCP-OPTIONS-REF", "PDU-TRIGGERING-REFS", "PERMISSIBLE-SIGNAL-PATH", "BASE-CYCLE", "COUPLING-PORT-SHAPER", "TARGET-HEALTH-CHANNEL-REF", "TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS", "ARGUMENT-DATA-PROTOTYPE", "SW-DATA-DEPENDENCY-ARGS", "PROCESS-STATE-MACHINE", "GLOBAL-TIME-PDU-REF", "SUPERVISION-CYCLE", "SYSTEM-MAPPING", "CONNECTION-TYPE", "R-PORT-PROTOTYPE-IREF", "KEY-EXCHANGE-AUTHENTICATION-REFS", "SIZE-OF-UNION-LENGTH-FIELDS", "PORT-PROTOTYPE-IREF", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "ECUC-VALIDATION-CONDITION", "BLOCK-STATE", "PLCA-MAX-BURST-COUNT", "TARGET-CHANNEL", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "AUTOSAR", "POST-BUILD-VARIANT-USED", "NONQUEUED-SENDER-COM-SPEC", "FUNCTION-GROUP-STATE-IREF", "OS-MODULE-INSTANTIATION", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "IGNORE-AFTER-TX", "FREEZE-FRAME-CONTENT-REF", "ECUC-DESTINATION-URI-DEF-SET", "POSSIBLE-AP-ERROR-SET-REFS", "LOG-AND-TRACE-INSTANTIATION", "FOLLOW-UP-ACTION-REF", "BASE-TYPE-REF", "PRM", "ALIAS-NAMES", "MANAGED-PHYSICAL-CHANNEL-REFS", "DPD-DELAY", "TRANSFORMER-HARD-ERROR-EVENT", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "FRESHNESS-COUNTER-SYNC-ATTEMPTS", "USER-DEFINED-METHOD-DEPLOYMENT", "ARP-PROPS", "ECUC-VALUE-COLLECTION", "CRC-VALIDATED", "COMMUNICATION-CONNECTOR-REF-CONDITIONAL", "I-PDU-TRIGGERINGS", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "MSR-QUERY-RESULT-P-1", "COMPU-NUMERATOR", "TRANSPORT-PLUGIN", "ENABLE-DEACTIVATION", "AFFECTED-REFS", "PROVIDER-SOFTWARE-CLUSTER-REF", "CDD-TYPE", "SDG-CAPTION", "REPLY-TOPIC-NAME", "SERVICE-INTERFACE-ID", "SUPPORT-INFO-BIT", "PARTITIONS", "MIRROR-STATUS-CAN-ID", "DEFAULT-PRIORITY", "PERSISTENCY-FILE-PROXY", "MIN-VALUE", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF", "NM-PDU-REF", "CRYPTO-SERVICE-CERTIFICATE", "TARGET-SWC-SERVICE-DEPENDENCY-REF", "SOURCE-CAN-ID-CODE", "DESTINATION-CONTEXT", "TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED", "NM-PDU-RX-INDICATION-ENABLED", "CONTAINED-PROCESS-REF", "COM-MANAGEMENT-PORT-GROUP-IREFS", "CHECKPOINT-ID", "COUPLING-PORT-REF", "LIMIT-KIND", "COLLECTION", "SOCKET-CONNECTION-BUNDLE-REF", "DELAY", "TASK", "IAM-MODULE-INSTANTIATION", "CONTROL-PROVIDED-EVENT-GROUP-REFS", "DIAGNOSTIC-ENABLE-CONDITION", "COMMUNICATION-CONNECTOR-REF", "DEFAULT-VALUE-HANDLING", "CRYPTO-SERVICE-NEEDS", "PROCESS", "LABEL-1", "MODE-DECLARATION-GROUP-PROTOTYPE-REF", "TIME-VALUE", "FAILED-REFERENCE-CYCLES-TOLERANCE", "HEALTH-CHANNEL-ARGUMENT-REFS", "METHOD-ACTIVATION-ROUTING-GROUPS", "CAN-NM-ECU", "ACL-ROLE-REF", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "AFFECTED-BSW-MODULE", "INTERFACE-MAPPING-SET", "SAMPLE-CLOCK-PERIOD", "MC-SUPPORT", "ARBITRARY-ADDRESS-CAPABLE", "BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING", "RELIABILITY", "EXTENDS-META-CLASS", "EVENT-ID", "RATE-CORRECTION-MEASUREMENT-DURATION", "IS-STRUCT-WITH-OPTIONAL-ELEMENT", "TCP-IP-NDP-RTR-SOLICITATION-INTERVAL", "PORT-PROTOTYPE-REF", "COMMUNICATION-CONTROLLER-REF", "J-1939-CLUSTER-CONDITIONAL", "ANNOTATION-ORIGIN", "DELEGATION-SW-CONNECTOR", "CRYPTO-KEY-SLOT", "ROUGH-ESTIMATE-HEAP-USAGE", "DATA-TYPE-REFS", "DISCOVERY-TECHNOLOGY", "SW-GENERIC-AXIS-DESC", "REFERENCE-VALUE-SPECIFICATION", "COUNTER-JUMP-UP-VALUE", "INITIALIZATION-VECTOR-LENGTH", "CHANNEL-ID", "ROOT-PARAMETER-DATA-PROTOTYPE-REF", "HW-PIN-GROUP-CONTENT", "STANDARD-REVISIONS", "LENGTH-SEQUENCE-COUNTER", "UDP-NM-ECU", "SW-CARB-DOCS", "FIELD-TOPICS-ACCESS-RULE-REF", "CONCRETE-PATTERN-EVENT-TRIGGERING", "SD", "FIRE-AND-FORGET", "VLAN-REFS", "SECURITY-EVENT-CONTEXT-PROPS", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "OBJECT-DEFINITION-REF", "DIAGNOSTIC-CONTRIBUTION-REFS", "COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS", "DATA-IDENTIFIER-REF", "BULK-NV-DATA-DESCRIPTORS", "ALL-NM-MESSAGES-KEEP-AWAKE", "RESTRICTIONS", "APPLICATION-ID", "ACCESS-PERMISSION-REF", "PARTIAL-NETWORK-REF", "EVENT-HANDLER-REFS", "PERSISTENCY-FILE-STORAGE-INTERFACE", "CALIBRATION-PARAMETER-VALUE", "BSW-ENTITY-REF", "NV-REQUIRE-COM-SPEC", "PLCA-LOCAL-NODE-ID", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "DEADLINE-MAX", "CLIENT-SERVER-RECORD-TYPE-MAPPING", "ENTER-TIMEOUT-VALUE", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "TLS-CRYPTO-SERVICE-MAPPING", "WAKE-UP-BY-CONTROLLER-SUPPORTED", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS", "LOWER-BOUND", "SERVICE-INSTANCE-REFS", "UNICAST-UDP-CREDENTIALS", "SUPPLIER-ID", "LIN-NM-CLUSTER", "TA-TYPE", "SIGNATURE-SCHEME-ID", "SW-FILL-CHARACTER", "SDG", "NON-OS-MODULE-INSTANTIATION-REF", "SEQUENCE-OFFSET", "DO-IP-CONFIG", "MAX-EVENTS-IN-INTERVAL", "LOG-CHANNEL-ID", "MAX-BUFFER-SIZE", "TEST-ID", "COM-MANAGEMENT-GROUP-REFS", "INSTANTIATION-DATA-DEF-PROPSS", "SYNCHRONIZATION-CONSTRAINT-TYPE", "DIAGNOSTIC-DATA-CHANGE-TRIGGER", "TIME-SUB-TLV", "CAN-TP-ADDRESS-REFS", "TCP-OPTION-FILTER-SET", "MIN-HEADER-ID", "DIAGNOSTIC-J-1939-SPN-MAPPING", "ECU-INSTANCE-REF", "DPG-REQUIRES-COHERENCYS", "OFFSET-SI-TO-UNIT", "PRODUCT-RELEASE", "PROVIDED-REQUIRED-INTERFACE-TREF", "NM-REMOTE-SLEEP-IND-ENABLED", "CANCELLATION", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "NETWORK-HANDLES", "AP-APPLICATION-ERROR-REF", "ELLIPTIC-CURVE-REFS", "FIGURE", "ROOT-REF", "TARGET-FRAME-REF", "EVALUATED-ELEMENT-REF", "DATA-RECEIVED-EVENT", "SOFTWARE-CLUSTER-DESIGN", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "TIMEOUT-SUBSTITUTION-VALUE", "PROPERTYS", "KEYWORD-SET", "INSTANCE-IN-MEMORY", "SOMEIP-FIELD-DEPLOYMENT", "DYNAMIC-LENGTH", "DESTINATION-URI-NESTING-CONTRACT", "DLT-MESSAGE", "COMPARE-VALUE", "SERVER-SERVICE-MAJOR-VERSION", "INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "PROTOCOL", "CUSTOM-SERVICE-CLASS-REF", "SUPPORTS-MULTIPLE-INSTANTIATION", "X", "RECOVERY", "TLV-IMPL-RECORD-ELEMENT", "PROVIDED-DATA-REF", "DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER", "REFERENCES", "ORDERED-MASTER-LIST", "ECUC-MODULE-CONFIGURATION-VALUES", "LIN-FRAME-TRIGGERING", "KEY-STORAGE-TYPE", "APPLICATION-ENTRY", "FIELD-ELEMENTS", "REFERENCE-BASE", "INSTANTIATION-RTE-EVENT-PROPSS", "DEADLINE-MIN", "BASE-TYPE-ENCODING", "CONTROL-CONSUMED-EVENT-GROUP-REFS", "CONTEXT-LIMITATION-REFS", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "DIRECT-PDU-REF", "MESSAGE-COMPLIANCE", "METHOD-CALL-REF", "SWC-TO-IMPL-MAPPING", "ALLOCATE-SHADOW-COPY", "AUTH-INFO-TX-LENGTH", "COM-PROCESSING-PERIOD", "PER-INSTANCE-MEMORY-REF", "REPOSITORY", "RPT-SERVICE-POINT-POST-REF", "REASON", "PROCESS-TO-MACHINE-MAPPING-SET", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "FLEXRAY-TP-NODE", "PNC-WAKEUP-CAN-ID", "SELECTOR-FIELD-CODE", "BSW-SERVICE-DEPENDENCY", "ASSIGNMENT-PRIORITY", "CLIENT-ID-DEFINITION-SET-REFS", "CONTEXT-ELEMENT-REF", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "PDU-TRIGGERING-REF-CONDITIONAL", "INSTANCES", "MAX-MARGIN", "POSSIBLE-ERROR-REACTION", "COMPONENT-REF", "MC-FUNCTION-REF", "ECUC-ENUMERATION-PARAM-DEF", "RANGE-MIN", "FINAL-CHECKPOINT-REFS", "I-SIGNAL-TRIGGERING-REF", "EXECUTION-ORDER-CONSTRAINT-TYPE", "SUB-SDG-REF", "AUTOSAR-CONNECTOR", "APPLICATION-DATA-TYPE-REF", "REST-ENDPOINT-ARGUMENT", "ECU-INSTANCE-PROPSS", "PHYSICAL-DIMENSION-MAPPING-SET", "SERVICE-INTERFACE-VERSION", "STATE", "STARTUP-CONFIGS", "RESOURCES", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "RPT-SERVICE-POINT", "TARGET-RUNNABLE-ENTITY-REF", "CLEAR-CONDITION-GROUP-REF", "IMPLEMENTATION-ARRAY-ELEMENT-REF", "CONTEXT-REF", "VARIABLE-ACCESS-IREFS", "MESSAGE-LINE-NUMBER", "FILE-ELEMENTS", "ENCRYPTION-REF", "MAXIMUM-ALLOWED-SIZE", "DATA-TYPE-MAPPING-SET-REFS", "DISABLE-END-TO-END-CHECK", "SDG-CAPTION-REF", "DEBOUNCE-COUNTER-STORAGE", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS", "ECU-RESOURCE-MAPPINGS", "WRITE-VERIFICATION", "PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT", "CLAIMED-FUNCTION-GROUP-REF", "DOCUMENTATION-CONTEXT", "OPERATION-INVOKED-EVENT", "SOURCE-ELEMENT-REF", "CRYPTO-SERVICE-QUEUE-REF", "DO-IP-TP-CONFIG", "TIME-SYNCHRONIZATION", "MODE-SWITCH-RECEIVER-COM-SPEC", "PORT-PROTOTYPE-BLUEPRINT", "CAPABILITY-RECORDS", "NO-SUPERVISION", "FUNCTION-GROUP-PHM-STATE-REFERENCE", "LIN-CLUSTER-CONDITIONAL", "FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL", "NM-PDU-REF-CONDITIONAL", "VERIFICATION-STATUS-INDICATION-MODE", "SIZE-OF-ARRAY-LENGTH-FIELD", "I-PDU-IDENTIFIER-UDP-REFS", "IS-SYNCHRONOUS", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "USE-CASE", "PATTERN", "TCP-MSL", "REFERENCED-BASELINE", "TCP-IP-DHCP-V-6-INF-DELAY-MAX", "PNC-PREPARE-SLEEP-TIMER", "CONFIGURATION-CLASS-AFFECTION", "INDENT-SAMPLE", "ECUC-PARAM-CONF-CONTAINER-DEF", "IMPLEMENTATION-REFS", "AR-TYPED-PER-INSTANCE-MEMORY-REF", "REQUIRED-PORT-PROTOTYPE-IREF", "INHIBITION-MASK", "LIN-TP-NODE", "MEMORY-SECTIONS", "REST-ENUMERATOR-DEF", "SUB-FUNCTION-REF", "CHECKPOINTS", "VARIABLE-ACCESSED-FOR-DEBUG-REFS", "SDG-AGGREGATION-WITH-VARIATION", "IS-OBD-RELEVANT", "NETWORK-TIME-CONSUMER-REF", "SENDER-RECEIVER-ANNOTATIONS", "SW-RECORD-LAYOUT-REF", "SD-SERVER-TIMER-CONFIGS", "RESOURCE-GROUP-REF", "SERVER-SERVICE-MINOR-VERSION", "SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF", "INTERNAL-BEHAVIOR", "CAN-FRAME-TRIGGERING", "VERIFY-REFS", "NM-VOTING-CYCLE", "TIME-SYNC-PORT-PROTOTYPE-IREF", "LDAP-URL", "SESSION-HANDLING", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "BSW-DEBUG-INFO", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "MAPPED-FUNCTION-REF", "GLOBAL-TIME-FR-SLAVE", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF", "SOCKET-PROTOCOL", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "RX-ACCEPT-CONTAINED-I-PDU", "FOLLOW-UP-TIMEOUT-VALUE", "PER-INSTANCE-PARAMETERS", "DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT", "ASSIGNED-ENTRY-ROLES", "NV-DATA-PORT-ANNOTATIONS", "RUNNABLE-ENTITY-IREFS", "BSW-MODULE-DOCUMENTATIONS", "FUNCTION-GROUP-IREF", "ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF", "IP-PROTOCOL", "DIAGNOSTIC-CONNECTIONS", "BINARY-MANIFEST-REQUIRE-RESOURCE", "SUCCESSOR-REFS", "DEPENDS-ON", "INNER-PORT-IREF", "PROVIDED-INTERFACE-TREF", "BSW-RESOURCE-ESTIMATION", "AUTHENTICATION-PROPS-REF", "TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED", "VENDOR-API-INFIX", "OBD-MONITOR-SERVICE-NEEDS", "EVENT-REF", "PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS", "BOOLEAN-VALUE-VARIATION-POINT", "COM-PROPS", "DIAGNOSTIC-ENV-DATA-CONDITION", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "SW-VALUE-CONT", "SERVICE-INTERFACE-TREF", "DESTINATION-URI-DEFS", "SNAPSHOT-RECORD-CONTENTS", "DIAGNOSTIC-DATA-PORT-MAPPING", "COUPLING-PORT-DETAILS", "RECOVERY-NOTIFICATION-RETRY", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "MASTER-EVENT-REF", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS", "REST-ENDPOINT-DELETE", "PRM-CHAR", "PORT-INTERFACE-BLUEPRINT-REF", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS", "TGROUP", "POSSIBLE-ERRORS", "INVALIDATION-POLICYS", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "ALLOWED-KEYSLOT-USAGE", "RAW-DATA-STREAM-REF", "BROADCAST", "TCP-PORT-REF", "MAX-PERIODIC-DID-TO-READ", "TLV-RECORD-ELEMENT-REF", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "TRANSPORT-PLUGINS", "TEST-PATTERN", "SYNCHRONIZED-MODE-GROUPS", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "IO-HW-ABSTRACTION-SERVER-ANNOTATION", "TIME-MASTER-PRIORITY", "PHM-ARBITRATION", "NM-IF-ECUS", "NM-REPEAT-MSG-IND-ENABLED", "UNRESOLVED-REFERENCE-RESTRICTION", "IS-ACTIVATION-LINE-DEPENDENT", "MODE-IREFS", "CLIENT-SERVER-OPERATION", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "GETTER-REF", "DO-IP-GID-NEEDS", "TEMPLATE-TYPE-REF", "DERIVED-PORT-INTERFACE-REF", "HW-PIN-GROUP", "SERVER-PORT-REF", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "TRANSMISSION-MODE-CONDITIONS", "FIRE-AND-FORGET-MAPPING", "NV-BLOCK-SW-COMPONENT-TYPE", "BSW-INTERNAL-TRIGGERING-POINT-REF", "TCP-SLOW-START-ENABLED", "FIELD-REFS", "SUB-ELEMENT-MAPPING", "MODE-RECEIVER-POLICYS", "RESTRICT-UPDATE", "BSW-CALLED-ENTITY", "PERSISTENCY-FILE-ARRAY-REF", "SD-CLIENT-TIMER-CONFIGS", "RESPONSE-ON-EVENT-SCHEDULER-RATE", "CRYPTO-JOB-REF", "TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS", "DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL", "UNICAST-SECURE-COM-PROPS-REF", "KEY-IDENTITY-REF", "RELATIVE", "ECUC-FOREIGN-REFERENCE-DEF", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS", "SLEEP-REPETITIONS-OF-SLEEP-REQUEST", "INIT-RESOURCE", "DEBOUNCE-ALGORITHM-PROPSS", "SW-CLUSTERS", "DO-IP-POWER-MODE-STATUS-NEEDS", "EXPECTED-CALLBACKS", "PROVIDED-OUTER-PORT-REF", "BSW-BEHAVIOR-REF", "SIGNATURE-SCHEME-REF", "FLEXRAY-FRAME-TRIGGERING", "REQUIRED-INTERFACE-TREF", "VG", "REQUESTER-SOFTWARE-CLUSTER-REF", "WAKEUP-LOCAL-ENABLED", "PDU-TO-FRAME-MAPPING", "VALUE-CONFIG-CLASSES", "MAXIMUM-TRANSMIT-BUFFER-LENGTH", "MODE-CONDITION-REF", "AUTO-COLLECT", "REQUIRED-SERVICE-VERSION", "GLOBAL-TIME-DOMAIN-PROPERTYS", "SUPERVISION-CHECKPOINT", "IDENTITIY-NUMBER", "ECUC-TEXTUAL-PARAM-VALUE", "I-SIGNAL-I-PDU-REF", "MAC-MULTICAST-ADDRESS-REF", "SLAVE-EVENT-REF", "MULTICAST-CREDENTIALS", "NON-VERBOSE-MODE", "SDX-REF", "E-2-E-PROFILE-CONFIGURATION-REF", "J-1939-SHARED-ADDRESS-CLUSTER", "SPECIFICATION-DOCUMENT-SCOPE", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS", "FUNCTION-POINTER-SIGNATURE-REF", "CLIENT-CAPABILITY", "ARRAY-SIZE-SEMANTICS", "ST-MIN", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "BITFIELD-TEXT-TABLE-MASK-SECOND", "DATA-TYPE-FOR-SERIALIZATION-REFS", "CAN-NM-NODE", "ACTIVATION", "ERROR-CONTEXT-REFS", "BULK-NV-BLOCK", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "USE-CRC-COMP-MECHANISM", "PERIODIC-RESPONSE-UUDT-REFS", "ADMIN-DATA", "IDS-MGR-NEEDS", "PER-INSTANCE-MEMORY-SIZE", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING", "COM-CONFIGURATION-GW-TIME-BASE", "MAX-ERROR-STATE-INIT", "PULSE-TEST", "LOCAL-PARAMETER-REF", "MIN-NUMBER-OF-ELEMENTS", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "ACL-OBJECT-REF", "ECUC-MULTILINE-STRING-PARAM-DEF", "INFO-TYPE-REF", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "CALLED-ENTRYS", "TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED", "DISCOVERY-TYPE", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "IEEE-1722-TP-ETHERNET-FRAME", "CREATED-DATAS", "LOG-TRACE-FILE-PATH", "LENGTH-CLIENT-ID", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "DLT-ECU-ID", "RESUME-POSITION", "IS-DEFAULT", "REQUIRED-MODE-GROUP-REF", "VEHICLE-SYSTEM-INSTANCE", "MODE-ELEMENT-REF", "ACL-PERMISSION", "ELEMENT-REFS", "IPV-6-MULTICAST-IP-ADDRESS", "METHOD-MAPPINGS", "DATA-WRITE-COMPLETED-EVENT", "TCP-ALIVE-CHECK-RESPONSE-TIMEOUT", "HW-PORT-MAPPING", "FIRST-PORT-REF", "MAPPINGS", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS", "SUBTITLE", "PROCESSOR-CORE", "CYCLE-COUNTER", "ECUC-FUNCTION-NAME-DEF", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "VEHICLE-DESCRIPTION-REF", "OFFSET-CORRECTION-START", "ENUMS", "TLS-JOB-SEMANTIC", "EXTENDED-ADDRESSING", "MACRO-INITIAL-OFFSET-B", "ARTIFACT-DESCRIPTOR", "COUPLING-PORT-STRUCTURAL-ELEMENTS", "DISABLE-END-TO-END-STATE-MACHINE", "VEHICLE-ANNOUNCEMENT-INTERVAL", "VENDOR-SIGNATURE-REF", "BUFFER-OUTPUT", "SERVICE-INTERFACE-FIELD-MAPPING", "TCP-IP-ARP-REQUEST-TIMEOUT", "IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "TIMING-MODES", "DATA-IDENTIFIER-REFS", "DIAGNOSTIC-PROTOCOL", "SW-RECORD-LAYOUT-V-PROP", "SIZE-OF-STRUCT-LENGTH-FIELD", "SW-COMPONENT-PROTOTYPE-ASSIGNMENT", "LEAF-ELEMENT-IREF", "OBD-SUPPORT", "WRITE-ONLY-ONCE", "BUFFER-PROPERTIES", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "VARIATION", "TEAR-DOWN-ACTION-REFS", "UCM-MASTER-FALLBACK-REF", "IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "AUXILIARY-FIELDS", "REST-SERVICE-INTERFACE", "SUB-SOFTWARE-CLUSTER-REFS", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "PHYSICAL-DIMENSION-MAPPING", "PREDECESSOR-ACTION-REFS", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "IP-ADDRESS", "STATIC-SOCKET-CONNECTIONS", "DIAGNOSTIC-RELEVANCE", "DIAGNOSTIC-EVENT-NEEDS", "MINIMUM-INTENDED-BINDING-TIME", "ANNOTATION-TEXT", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "DOMAIN-ID", "TBODY", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "CATEGORY", "CALLED-ENTRY-REF", "TARGET-RUNNABLE-ENTITY-GROUP-REF", "TARGET-METHOD-REF", "SW-CLASS-IMPL", "INTERNAL-TRIGGERING-POINT", "ALLOWED-TCP-OPTION", "SOCKET-CONNECTION-REFS", "MAXIMUM-SEGMENT-LENGTH", "MODE-SWITCH-POINTS", "IDSM-INSTANCE", "TRIGGER-IREF", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "PNC-WAKEUP-DATA-MASK", "EXEC-IN-USER-MODE", "TD-EVENT-COMPLEX", "MANDATORY", "RECOVERY-VIA-APPLICATION-ACTION-REF", "ACL-ROLE", "SEGMENT-REFS", "NM-CONTROL-BIT-VECTOR-ACTIVE", "POST-BUILD-VARIANT-CRITERION-VALUES", "APPLICATION-ASSOC-MAP-DATA-TYPE", "DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL", "PROFILE-BEHAVIOR", "DIAGNOSTIC-CONNECTION-REF", "TIME-BASE-RESOURCE-REF", "IDS-DESIGN", "PROCESS-MODE-IREF", "DIAGNOSTIC-ECU-RESET-INTERFACE", "LENGTH-EXP", "TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE", "SEC-OC-JOB-MAPPING", "MODE-DECLARATION-GROUP-PROTOTYPE-IREF", "FILTER", "BUILD-ACTION-MANIFESTS", "TYPE-BLUEPRINTS", "TLS-IAM-REMOTE-SUBJECT", "KEY-SLOT-USED-FOR-SYNC", "SOURCE-EVENT-REF", "SOMEIP-TP-CONFIG", "CALL-TIMEOUT", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "SERVICE-ID", "EXECUTABLE-REFS", "SEC-OC-COM-PROPS-FOR-MULTICAST-REFS", "BLUEPRINT-MAPPING", "KEY", "DIAGNOSTIC-DE-AUTHENTICATION", "HEALTH-CHANNELS", "SYNCHRONIZED-TIME-BASE-PROVIDER", "EXCLUSIVE-AREA-NESTING-ORDER-REF", "CUSTOM-SERVICE-ID", "SPECIFICATION-SCOPE", "REPETITION-PERIOD", "PNC-MAPPING-REFS", "CLIENT-POLICYS", "RTP-TP", "FLOAT", "PROPABILITY", "FLAT-MAP-ENTRY-REFS", "MAX-SERVICE-POINT-ID", "CALIBRATION-PARAMETER-VALUE-SET-REF", "EXCLUSIVE-AREA-REF-CONDITIONAL", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "PDELAY-RESPONSE-ENABLED", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "USER-DEFINED-COMMUNICATION-CONTROLLER", "ACTION-POINT-OFFSET", "SIGNAL-BASED-EVENT-REF", "SECURE-COMMUNICATION-PROPS-SET", "RUNS-INSIDES", "CRYPTO-ELLIPTIC-CURVE-PROPS", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "SW-SERVICE-IMPL-POLICY", "TIME-SEG-1", "ECUC-REFERENCE-VALUE", "TCP-IP-NDP-DESTINATION-CACHE-SIZE", "BSW-CLIENT-POLICY", "AGGREGATION", "MEMORY-CONSUMPTION", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "KEY-SLOT-ID", "DESTINATION-REF", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF", "STATIC-SOCKET-CONNECTION", "SERVICE-REQUEST-CALLBACK-TYPE", "SWC-SERVICE-DEPENDENCY-IREF", "DATA-TRANSFORMATIONS", "BUFFER-RATIO", "DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF", "R-PORT-PROTOTYPE", "SOUCE-CAN-ID-REF", "FLAT-INSTANCE-REF", "EXECUTION-DEPENDENCY", "FRAGMENTATION-PROPS", "SD-CLIENT-CONFIG", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "ACCESS-DATA-RECORD-SIZE", "PDU-COLLECTION-PDU-TIMEOUT", "AUTOMATIC-END", "REFERENCE-CONDITION", "V", "REST-ARRAY-PROPERTY-DEF", "WORST-CASE-STACK-USAGE", "REST-SYSTEM-TRIGGERED-EVENT", "ETH-SWITCH-PORT-GROUP-DERIVATION", "PID-REFS", "DEBUG-INFOS", "STORAGE-CONDITION-GROUP-REF", "STATUS-BIT-STORAGE-TEST-FAILED", "SSRC", "ECUC-DEFINITION-REF", "CYCLE-COUNT-MAX", "API-PRINCIPLE", "XCP-PDU", "ESTIMATED-DURATION-OF-OPERATION", "SECURITY-ACCESS-CLASS-REF", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS", "DATA-FILTER-TYPE", "REQUIRES-INDEX", "DLT-LOG-CHANNEL", "OFFSETS", "LABELED-LIST", "KEY-SLOT-USAGE", "APP-TASK-PROXY-REF", "DESTINATION-URI-REF", "MAX-COMM-MODE", "HEALING-CYCLE-COUNTER-THRESHOLD", "SENDING-NODE-REFS", "MAX-SIZE-TO-FILL", "MAX-GRADIENT", "IMPL-CONSTANT-REF", "MAIN-FUNCTION-PERIOD", "DIAGNOSTIC-COMMON-PROPS-VARIANTS", "MOLAR-AMOUNT-EXP", "CLIENT-SERVER-TO-SIGNAL-MAPPING", "PDU-COLLECTION-TRIGGER", "EXCLUSIVE-AREA-NESTING-ORDERS", "UNLIMITED-INTEGER-VALUE-VARIATION-POINT", "GLOBAL-SUPERVISION-NEEDS", "PERIOD-BEGIN", "RESPONSE-ON-EVENT-ACTION", "PORT-DEFINED-ARGUMENT-BLUEPRINT", "TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE", "DIAGNOSTIC-EVENT-REF", "ESTIMATED-EXECUTION-TIME", "APPLICATIONS", "DATA-ID-NIBBLE-OFFSET", "MINIMUM-TX-CONTAINER-QUEUE-SIZE", "VARIATION-RESTRICTION", "TP-CHANNELS", "SERVICE-NEEDS", "TTCAN-ABSOLUTELY-SCHEDULED-TIMING", "MODE-DEPENDENT-STARTUP-CONFIGS", "NM-CONFIG", "REQUIRED-RTE-VENDOR", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "VARIABLE-INSTANCE-IREF", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "VENDOR-ID", "UCMS", "UPPER-CAN-ID", "LICENSE-REFS", "EVENT-DISPLACEMENT-STRATEGY", "ERROR-CODE", "INSTANTIATION-TIMING-EVENT-PROPS", "TRANSPORT-PDU-REF", "TRANSFORMATION-PROPS-MAPPING-SET-REF", "CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS", "WINDOW-SIZE-INVALID", "MICRO-INITIAL-OFFSET-A", "SUB-GROUP-REFS", "SW-RECORD-LAYOUT", "PASS-THROUGH-SW-CONNECTOR", "INVOCATION", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "TIMEOUT-BR", "OBJECT-REFS", "BUS-IDLE-TIMEOUT-PERIOD", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "EXECUTION-DEPENDENCYS", "SOURCE-ID", "CRYPTO-CERTIFICATE", "DENOMINATOR-CONDITION", "MODE-INSTANCE", "TCP-IP-IP-REASSEMBLY-BUFFER-SIZE", "RULE-ARGUMENTS", "TCP-PORT", "STORE-EMERGENCY", "EXECUTABLE-REQUIRED-PORT-IREF", "PDU-ACTIVATION-ROUTING-GROUPS", "NM-SHUTDOWN-DELAY-TIMER", "CLASS-TAILORINGS", "STATE-REFERENCES", "QUEUE-LENGTH", "EVENT-CONTROLLED-TIMING", "SOFTWARE-COMPOSITION-TREF", "TCP-SYN-MAX-RTX", "COMMAND", "PORT-DEFINED-ARGUMENT-VALUE", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "SO-CON-I-PDU-IDENTIFIER-REF", "EXECUTION-TIMES", "APPROVAL-REQUIRED", "SECURITY-EVENT-MAPPING", "I-SIGNAL-PORT-REFS", "MAX", "SW-VERSION", "CAN-PHYSICAL-CHANNEL", "ETHERNET-RAW-DATA-STREAM-MAPPING", "SELECTOR-FIELD-START-POSITION", "TLV-DATA-IDS", "SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL", "REST-STRING-PROPERTY-DEF", "SCHEDULING-PRIORITY", "LAST-EGRESS-SCHEDULER-REF", "BLOCK-IF-STATE-ACTIVE-CP-REFS", "SOURCE-SIGNAL-REF", "KEY-SLOTS", "HARDWARE-ELEMENT-REFS", "SW-CLASS-INSTANCE", "EVENT-MULTICAST-UDP-PORT", "NUM-FAILED-SECURITY-ACCESS", "TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE", "IMPLICIT-DATA-ACCESS-IREF", "I-PDU-TIMING-SPECIFICATIONS", "DO-IP-INTERFACE", "ECU-STATE-MGR-USER-NEEDS", "ACCESS-PERMISSION-VALIDITY", "TIME-SYNC-SERVER-IDENTIFIER", "COMPU-METHOD-REF", "PERSISTENCY-CENTRAL-STORAGE-URI", "MSR-QUERY-TOPIC-1", "COUNTER-FAILED-THRESHOLD", "SYMBOL-WINDOW-ACTION-POINT-OFFSET", "FLOAT-VALUE-VARIATION-POINT", "MINIMUM-INTER-ARRIVAL-TIME", "DEVELOPMENT-VALUE", "LUMINOUS-INTENSITY-EXP", "AUTHENTICATION-TIMEOUT", "HARDWARE-CONFIGURATION", "REG-DOES-NOT-REQUIRE-STABILITYS", "BSW-BACKGROUND-EVENT", "REQUIRES-SYMBOLIC-NAME-VALUE", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "DHCP-ADDRESS-ASSIGNMENT", "VLAN-IDENTIFIER", "REMOTE-CERTIFICATE-REFS", "WAKEUP-FORWARD-REMOTE-ENABLED", "TARGET-R-PORT-REF", "FILE-STORAGE-REF", "PROCESS-TO-MACHINE-MAPPINGS", "TRIGGER-PORT-ANNOTATION", "POST-BUILD-VARIANT-MULTIPLICITY", "EXECUTABLE-TIMING", "ECUC-MODULE-DEF", "DATA-PROTECTION-KIND", "UNUSED-BIT-PATTERN", "SUBFUNCTION", "PORT-ADDRESS", "COUPLING-PORT-RATE-POLICY", "N-ROM-BLOCKS", "AUTHENTICATION-RETRIES", "ROW", "REMOTE-ACCESS-CONTROL-ENABLED", "CRYPTO-KEY-SLOT-REF", "DETERMINISTIC-CLIENT-REF", "INGRESS-PRIORITY", "MESSAGE-LINK-LENGTH", "MATCHING-CRITERION-REF", "COM-OFFER-SERVICE-GRANT-DESIGN", "CONTAINER-TRIGGER", "INDICATIONS", "SAFETY-CONDITION", "FREEZE-CURRENT-STATE", "CLIENT-SERVER-INTERFACE-REF", "MC-FUNCTION-REFS", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "TIMING-DESCRIPTION-EVENT-CHAIN", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING", "TIMING-PROPS", "SUB-TYPE", "MAXIMUM-MEMORY-CONSUMPTION", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "VARIABLES", "EXTERNAL-AUTHENTICATIONS", "RTE-EVENT-SEPARATIONS", "END-TO-END-PROTECTION-I-SIGNAL-I-PDUS", "AUTHENTICATION-CLASS-REF", "CONTEXT-COMPONENT-REFS", "AR-PACKAGES", "CRC-CORRECTION-FIELD", "UDP-PORT", "IS-DYNAMIC-LENGTH-FIELD-SIZE", "KEYWORDS", "SAMPLES-PER-MICROTICK", "BOR-COUNTER-L-1-TO-L-2", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "CAN-NM-CLUSTER-COUPLING", "SW-VALUE-BLOCK-SIZE", "ECU-TASK-PROXY-REFS", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "ASYNCHRONOUS-SERVER-CALL-POINT", "STANDARD-REVISION", "COMPU-DEFAULT-VALUE", "NM-REPEAT-MESSAGE-BIT-ACTIVE", "PROCESS-REF", "DO-IP-REQUEST-CONFIGURATION", "RESOURCE-DEFINITION-REF", "CONNECTION-PRIO-PDUS", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "FLOW-CONTROL-PDU-REF", "INCLUDED-MODE-DECLARATION-GROUP-SETS", "GRAPHIC", "SWC-TIMING", "PID-REF", "PERSISTENCY-REDUNDANCY-M-OUT-OF-N", "SIGNATURE-SUPPORT-AP", "DIAGNOSTIC-IUMPR", "UPDATE", "ADDITIONAL-NATIVE-TYPE-QUALIFIER", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "SERVICE-ELEMENT-MAPPING-REF", "COUPLING-ELEMENT", "APPLICATION-CONTEXT-REFS", "APPLICATION-RULE-BASED-VALUE-SPECIFICATION", "SIZE-OF-STRUCT-LENGTH-FIELDS", "ECUC-INTEGER-PARAM-DEF", "MAXIMUM-TRANSMISSION-UNIT", "RESPONSE-ERROR-REF", "MC-GROUP", "SW-GENERIC-AXIS-PARAM-TYPES", "LOG-TRACE-PROCESS-ID", "PERIODIC-RATES", "MC-PARAMETER-INSTANCES", "BUS-MIRROR-NETWORK-ID", "ECUC-FUNCTION-NAME-DEF-CONDITIONAL", "ECUC-DEFINITION-COLLECTION", "NM-INSTANTIATION", "SIGNAL-SERVICE-TRANSLATION-PROPS", "CRYPTO-SERVICE-KEY-REF", "SW-CLASS-IMPLS", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "STORAGE-STATE-EVALUATION", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "MODEL-OBJECT-REFERENCE", "CP-SOFTWARE-CLUSTER-REF-CONDITIONAL", "ECUC-COND", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "BSW-COMPOSITION-TIMING", "FILTERING-DEBOUNCING", "TIMING-DESCRIPTIONS", "TARGET-EEC-REF", "SIGNAL-BASED-FIELD-REF", "IDLE-SLOPE", "XFILE", "ROOT-SW-COMPOSITION-PROTOTYPE", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "DETERMINISTIC-CLIENT-RESOURCE-NEEDSS", "HEALTH-CHANNEL-SUPERVISION", "GATEWAY", "MESSAGE-TYPE-INFO", "TRANSFORMER-STATUS-FORWARDING", "MODE-DECLARATION-MAPPING-SET-REF", "SYNC-COUNTER-INIT", "ROUTINE-SUBFUNCTION-REF", "TIME-OFFSET", "MACHINE-MODE-MACHINES", "IDENTITY-REF", "SESSION-CONTROL-CLASS-REF", "PATTERN-PERIOD", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "AUDIENCES", "PRESTORED-FREEZEFRAME-STORED-IN-NVM", "REDUNDANCY", "ECU-INSTANCE-REFS", "COMPOUND-PRIMITIVE-ARGUMENTS", "BSW-RELEASED-TRIGGER-POLICY"];

    /// derive an enum entry from an input string using a perfect hash function
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseElementNameError> {
        static DISPLACEMENTS: [(u16, u16); 809] = [(0, 1), (0, 1618), (2, 4682), (1, 120), (0, 18), (0, 285), (0, 26), (0, 1), (0, 20), (0, 20), (0, 8), (0, 0), (1, 4648), (0, 24), (0, 0), (2, 3249), (0, 1208), (0, 14), (0, 10), (3, 1330), (0, 1983), (0, 214), (0, 37), (0, 513), (0, 4), (0, 241), (0, 56), (5, 4025), (0, 1312), (0, 515), (0, 2791), (0, 1), (0, 2043), (0, 1), (0, 0), (0, 6), (0, 124), (0, 386), (0, 69), (0, 0), (0, 472), (0, 30), (0, 3557), (1, 4472), (0, 5434), (0, 0), (0, 367), (6, 1758), (0, 2710), (0, 6), (3, 1409), (0, 3560), (0, 57), (0, 700), (0, 149), (0, 195), (0, 21), (0, 712), (0, 3056), (0, 13), (0, 47), (4, 5190), (0, 3), (0, 1838), (0, 12), (5, 2052), (0, 12), (0, 12), (1, 2787), (4, 4316), (0, 859), (0, 0), (0, 16), (0, 69), (0, 74), (0, 27), (1, 855), (0, 19), (1, 1829), (1, 2656), (0, 2), (0, 2240), (0, 5), (0, 123), (0, 1), (0, 11), (0, 63), (0, 982), (0, 31), (0, 228), (0, 435), (0, 89), (0, 20), (0, 2), (0, 885), (0, 206), (0, 1696), (0, 17), (0, 4), (0, 91), (6, 47), (0, 128), (0, 682), (0, 131), (0, 9), (2, 3481), (1, 3605), (0, 2), (0, 20), (0, 89), (0, 141), (0, 495), (2, 2991), (0, 394), (0, 230), (0, 3), (1, 1323), (0, 24), (0, 1), (0, 23), (3, 604), (0, 646), (0, 42), (17, 4519), (0, 0), (0, 1326), (0, 5), (1, 5243), (0, 2294), (2, 5043), (1, 5271), (9, 746), (0, 2785), (0, 58), (0, 1842), (0, 898), (0, 57), (0, 29), (0, 5647), (9, 2659), (0, 0), (0, 435), (0, 3886), (1, 5317), (0, 7), (0, 5383), (14, 3979), (0, 4), (0, 972), (0, 0), (0, 17), (0, 9), (8, 642), (0, 12), (13, 3106), (0, 4007), (1, 3561), (0, 1253), (0, 3), (0, 409), (0, 33), (0, 129), (0, 3), (0, 85), (5, 783), (0, 67), (0, 475), (0, 1), (0, 20), (0, 199), (0, 13), (0, 1), (0, 695), (0, 225), (0, 49), (1, 256), (5, 3285), (0, 1), (0, 102), (0, 197), (0, 0), (0, 2), (0, 474), (15, 4647), (0, 170), (0, 975), (0, 341), (0, 996), (1, 4062), (1, 1890), (0, 27), (0, 1), (0, 26), (0, 711), (0, 0), (0, 6), (0, 1088), (0, 8), (0, 98), (0, 4969), (2, 4079), (0, 2503), (0, 1), (0, 431), (0, 0), (0, 52), (0, 1971), (0, 3343), (0, 5), (0, 2), (0, 9), (0, 0), (0, 187), (2, 4413), (0, 1), (0, 102), (3, 4022), (4, 2860), (0, 423), (3, 4470), (0, 72), (24, 4779), (1, 4701), (0, 408), (0, 286), (1, 5130), (0, 1), (3, 5386), (0, 521), (0, 8), (0, 719), (0, 58), (6, 1781), (0, 3408), (0, 2048), (0, 4646), (0, 0), (0, 7), (0, 10), (0, 61), (0, 4), (2, 3216), (0, 5), (16, 3384), (0, 151), (0, 124), (3, 1713), (0, 7), (0, 122), (10, 2710), (0, 0), (0, 1072), (23, 4627), (0, 22), (0, 1899), (0, 24), (34, 2977), (47, 5462), (15, 107), (0, 3333), (2, 5392), (0, 17), (0, 2862), (0, 15), (0, 13), (0, 7), (0, 4), (0, 10), (0, 38), (0, 1500), (0, 2542), (0, 7), (0, 0), (0, 1372), (0, 65), (0, 447), (0, 179), (0, 69), (5, 4966), (0, 8), (0, 2276), (5, 3370), (0, 143), (0, 834), (1, 4738), (0, 567), (2, 4808), (0, 939), (0, 0), (5, 2564), (0, 1881), (0, 1999), (0, 251), (1, 3629), (0, 20), (0, 3127), (0, 2962), (6, 1056), (0, 483), (0, 0), (4, 1668), (3, 3554), (0, 1933), (3, 2287), (0, 30), (9, 1488), (0, 24), (16, 3605), (0, 224), (0, 9), (0, 4), (4, 1389), (0, 8), (0, 2), (0, 10), (4, 5075), (0, 3), (8, 330), (0, 2), (0, 804), (0, 510), (14, 4), (0, 4784), (0, 168), (0, 6), (0, 4), (1, 3837), (0, 3), (0, 414), (0, 23), (32, 2873), (2, 1823), (0, 716), (0, 75), (33, 3192), (5, 3500), (0, 3761), (0, 245), (11, 2878), (2, 4416), (0, 8), (0, 144), (0, 337), (7, 891), (0, 184), (0, 194), (1, 1088), (0, 1547), (0, 0), (0, 30), (0, 4708), (0, 1), (0, 9), (2, 3169), (6, 1533), (0, 1), (13, 917), (0, 27), (0, 702), (0, 68), (0, 205), (0, 8), (0, 3), (0, 125), (0, 677), (0, 3738), (0, 1445), (0, 8), (0, 53), (1, 214), (0, 506), (0, 478), (0, 313), (8, 1775), (0, 77), (0, 6), (0, 29), (0, 2293), (0, 19), (4, 890), (10, 1074), (0, 25), (0, 3209), (31, 5441), (1, 3068), (1, 4148), (0, 4634), (0, 167), (0, 81), (0, 1679), (0, 39), (0, 6), (0, 3), (0, 156), (0, 1), (0, 122), (0, 428), (0, 2), (0, 145), (0, 24), (0, 4173), (0, 2683), (0, 144), (0, 612), (26, 3100), (0, 2), (0, 10), (0, 345), (6, 843), (0, 468), (0, 2047), (0, 8), (4, 5293), (18, 3349), (0, 35), (0, 5), (9, 944), (0, 10), (38, 3586), (0, 5076), (1, 1501), (0, 914), (0, 2327), (0, 1), (0, 177), (12, 112), (0, 0), (1, 2890), (0, 16), (1, 5323), (0, 8), (0, 2345), (1, 2492), (0, 26), (0, 10), (0, 4928), (0, 5), (40, 5013), (0, 406), (0, 12), (0, 12), (34, 1433), (0, 65), (0, 2336), (0, 2448), (0, 1529), (0, 252), (0, 5), (3, 2495), (2, 2217), (0, 2587), (0, 0), (0, 198), (8, 2614), (1, 1467), (0, 21), (0, 7), (0, 148), (64, 1012), (0, 4), (1, 3916), (0, 4170), (17, 2235), (13, 1569), (0, 331), (0, 12), (1, 4296), (0, 3669), (0, 1768), (2, 156), (25, 5442), (0, 11), (42, 407), (0, 40), (0, 190), (0, 0), (0, 3521), (0, 0), (0, 1602), (0, 2), (1, 4755), (2, 4674), (0, 881), (59, 4461), (0, 1480), (0, 1210), (7, 3966), (44, 140), (0, 628), (0, 2), (0, 3635), (0, 52), (0, 183), (0, 3300), (16, 3909), (0, 2384), (0, 1044), (0, 1971), (0, 58), (83, 4079), (40, 1318), (2, 4724), (21, 3011), (11, 4779), (0, 158), (0, 5), (1, 3044), (0, 2), (0, 438), (0, 8), (0, 3), (7, 4853), (0, 61), (0, 1), (0, 13), (0, 293), (0, 26), (1, 1523), (0, 3954), (0, 56), (0, 235), (31, 2173), (0, 231), (0, 9), (1, 5554), (0, 59), (1, 5067), (0, 0), (8, 1477), (6, 3325), (1, 84), (13, 4115), (1, 3207), (0, 0), (0, 866), (12, 3745), (21, 4415), (0, 44), (0, 361), (0, 0), (1, 834), (0, 276), (0, 845), (0, 0), (0, 0), (0, 844), (10, 1422), (2, 1155), (32, 4782), (0, 42), (0, 686), (27, 5629), (8, 1326), (20, 1931), (0, 1617), (0, 107), (0, 262), (0, 2452), (21, 3299), (40, 1236), (4, 2544), (2, 3320), (24, 5336), (5, 2183), (0, 80), (1, 379), (0, 0), (0, 953), (1, 132), (0, 1696), (55, 4165), (12, 3366), (106, 1853), (2, 4123), (0, 1722), (0, 2887), (0, 12), (3, 4898), (6, 2150), (0, 2440), (0, 580), (2, 3900), (0, 3242), (10, 3020), (0, 10), (3, 1876), (0, 3), (1, 5071), (1, 302), (0, 1327), (3, 3528), (1, 1643), (0, 1758), (0, 22), (0, 99), (0, 4581), (0, 271), (3, 2797), (0, 245), (0, 453), (0, 3426), (0, 1204), (0, 0), (0, 0), (0, 271), (28, 3531), (0, 1147), (20, 3568), (0, 809), (7, 1205), (0, 1), (0, 0), (0, 281), (0, 70), (0, 454), (0, 5222), (71, 1554), (0, 1215), (0, 20), (14, 355), (0, 6), (0, 2), (1, 3595), (0, 18), (2, 5613), (0, 0), (0, 2530), (1, 1165), (0, 309), (11, 801), (112, 3700), (0, 386), (0, 17), (6, 4195), (12, 4292), (120, 1479), (12, 4268), (19, 5381), (0, 270), (0, 263), (0, 90), (0, 80), (11, 615), (0, 6), (0, 280), (0, 0), (0, 80), (0, 3), (110, 4871), (0, 258), (0, 7), (0, 2241), (0, 1147), (0, 1262), (0, 5337), (0, 270), (2, 1165), (1, 1692), (37, 1565), (0, 0), (0, 47), (0, 229), (2, 1698), (0, 93), (0, 8), (8, 457), (149, 2722), (14, 4805), (0, 444), (42, 950), (0, 2160), (0, 807), (12, 3338), (0, 0), (0, 2894), (0, 15), (0, 1958), (0, 10), (0, 4308), (0, 37), (90, 4379), (28, 4904), (0, 0), (0, 11), (0, 645), (0, 2985), (0, 30), (0, 419), (0, 262), (1, 3110), (38, 1318), (0, 6), (4, 4388), (6, 3063), (0, 266), (0, 4), (0, 5), (1, 404), (0, 18), (0, 9), (1, 543), (0, 4605), (85, 4007), (0, 642), (0, 225), (0, 2917), (0, 171), (48, 4469), (0, 191), (0, 118), (59, 3139), (0, 3030), (2, 3801), (0, 4), (0, 657), (189, 746), (5, 4568), (20, 1901), (25, 2123), (0, 666), (0, 4685), (0, 120), (11, 5439), (0, 1025), (0, 1527), (53, 641), (0, 1040), (0, 723), (0, 18), (183, 5015), (84, 1742), (0, 1), (0, 27), (7, 2009), (4, 1187), (155, 4866), (0, 607), (0, 376), (0, 16), (0, 1), (21, 4074), (0, 2626), (0, 1385), (0, 4), (0, 11), (288, 2665), (78, 3232), (0, 550), (0, 0), (32, 4784), (0, 95), (0, 0), (0, 2), (4, 4609), (0, 36), (0, 109), (10, 2517), (0, 4669), (357, 5118), (0, 311), (40, 4647), (0, 2913), (0, 5054), (46, 2427), (0, 4582), (0, 1212), (97, 936), (51, 433), (90, 547), (15, 775), (0, 67), (258, 4163), (0, 290), (1044, 741), (1, 3182), (0, 500), (0, 64), (0, 9), (283, 256), (5, 3617), (32, 4069), (559, 2499), (0, 109), (0, 5), (0, 1461), (0, 1409), (1, 4386), (2, 1482), (0, 4), (31, 97), (2, 1252), (0, 60), (84, 1983), (1, 554), (138, 2904), (0, 865), (28, 197), (249, 5176), (0, 3009), (24, 3329), (0, 2000), (12, 1227), (1, 46), (278, 3251), (0, 3882), (375, 2002), (243, 1596), (255, 2194), (0, 628)];
        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 809) as usize];
        let item_idx = (d2 as u32).wrapping_add(f1.wrapping_mul(d1 as u32)).wrapping_add(f2) as usize % 5662;
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

