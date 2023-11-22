use crate::hashfunc;

#[derive(Debug)]
/// The error type `ParseElementNameError` is returned when `from_str`() / parse() fails for `ElementName`
pub struct ParseElementNameError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
/// Enum of all element names in Autosar
pub enum ElementName {
    /// ABBR-NAME
    AbbrName                                                               = 3526,
    /// ABS
    Abs                                                                    = 5139,
    /// ABSOLUTE
    Absolute                                                               = 5175,
    /// ABSOLUTE-TOLERANCE
    AbsoluteTolerance                                                      = 3552,
    /// ABSOLUTELY-SCHEDULED-TIMINGS
    AbsolutelyScheduledTimings                                             = 2152,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 1405,
    /// ACCEPT-ENCODING
    AcceptEncoding                                                         = 4789,
    /// ACCEPTANCE-FIELD
    AcceptanceField                                                        = 3405,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF
    AcceptedCryptoCipherSuiteWithPskRef                                    = 2389,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS
    AcceptedCryptoCipherSuiteWithPskRefs                                   = 1401,
    /// ACCEPTED-REMOTE-CERTIFICATE-REF
    AcceptedRemoteCertificateRef                                           = 655,
    /// ACCEPTED-REMOTE-CERTIFICATE-REFS
    AcceptedRemoteCertificateRefs                                          = 4817,
    /// ACCEPTED-STARTUP-RANGE
    AcceptedStartupRange                                                   = 2178,
    /// ACCEPTS-ENCODINGS
    AcceptsEncodings                                                       = 1292,
    /// ACCESS-CONTROL
    AccessControl                                                          = 4486,
    /// ACCESS-COUNT
    AccessCount                                                            = 1568,
    /// ACCESS-COUNT-SET
    AccessCountSet                                                         = 2225,
    /// ACCESS-COUNT-SETS
    AccessCountSets                                                        = 4259,
    /// ACCESS-COUNTS
    AccessCounts                                                           = 1294,
    /// ACCESS-DATA-RECORD-SIZE
    AccessDataRecordSize                                                   = 4365,
    /// ACCESS-PERMISSION-REF
    AccessPermissionRef                                                    = 5224,
    /// ACCESS-PERMISSION-VALIDITY
    AccessPermissionValidity                                               = 1029,
    /// ACCESS-POINT-REF
    AccessPointRef                                                         = 1942,
    /// ACCESSED-DATA-PROTOTYPE-IREF
    AccessedDataPrototypeIref                                              = 4325,
    /// ACCESSED-MODE-GROUPS
    AccessedModeGroups                                                     = 4435,
    /// ACCESSED-PARAMETER
    AccessedParameter                                                      = 3582,
    /// ACCESSED-VARIABLE
    AccessedVariable                                                       = 244,
    /// ACCESSED-VARIABLE-REF
    AccessedVariableRef                                                    = 4989,
    /// ACCURACY
    Accuracy                                                               = 3387,
    /// ACCURACY-EXT
    AccuracyExt                                                            = 2033,
    /// ACCURACY-INT
    AccuracyInt                                                            = 788,
    /// ACK-REQUEST
    AckRequest                                                             = 1952,
    /// ACK-TYPE
    AckType                                                                = 921,
    /// ACL-CONTEXT
    AclContext                                                             = 871,
    /// ACL-CONTEXTS
    AclContexts                                                            = 5790,
    /// ACL-OBJECT-CLASS
    AclObjectClass                                                         = 4178,
    /// ACL-OBJECT-CLASSS
    AclObjectClasss                                                        = 2294,
    /// ACL-OBJECT-REF
    AclObjectRef                                                           = 5955,
    /// ACL-OBJECT-REFS
    AclObjectRefs                                                          = 3810,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 3530,
    /// ACL-OPERATION
    AclOperation                                                           = 3892,
    /// ACL-OPERATION-REF
    AclOperationRef                                                        = 873,
    /// ACL-OPERATION-REFS
    AclOperationRefs                                                       = 759,
    /// ACL-PERMISSION
    AclPermission                                                          = 5585,
    /// ACL-ROLE
    AclRole                                                                = 2071,
    /// ACL-ROLE-REF
    AclRoleRef                                                             = 4417,
    /// ACL-ROLE-REFS
    AclRoleRefs                                                            = 333,
    /// ACL-SCOPE
    AclScope                                                               = 2941,
    /// ACTION
    Action                                                                 = 5801,
    /// ACTION-ITEM-LISTS
    ActionItemLists                                                        = 1509,
    /// ACTION-ITEMS
    ActionItems                                                            = 3254,
    /// ACTION-LIST
    ActionList                                                             = 4680,
    /// ACTION-LIST-EXECUTION
    ActionListExecution                                                    = 4410,
    /// ACTION-LIST-ITEM-REF
    ActionListItemRef                                                      = 3522,
    /// ACTION-LIST-ITEM-REFS
    ActionListItemRefs                                                     = 4028,
    /// ACTION-LISTS
    ActionLists                                                            = 732,
    /// ACTION-POINT-OFFSET
    ActionPointOffset                                                      = 5332,
    /// ACTION-TYPE
    ActionType                                                             = 226,
    /// ACTIONS
    Actions                                                                = 5559,
    /// ACTIVATE-AT-START
    ActivateAtStart                                                        = 216,
    /// ACTIVATION
    Activation                                                             = 3304,
    /// ACTIVATION-ACTION
    ActivationAction                                                       = 5406,
    /// ACTIVATION-POINTS
    ActivationPoints                                                       = 1912,
    /// ACTIVATION-REASON-REPRESENTATION-REF
    ActivationReasonRepresentationRef                                      = 3548,
    /// ACTIVATION-REASONS
    ActivationReasons                                                      = 143,
    /// ACTIVATION-SWITCH
    ActivationSwitch                                                       = 2840,
    /// ACTIVE-SUPERVISION-REF
    ActiveSupervisionRef                                                   = 39,
    /// ACTIVE-SUPERVISION-REFS
    ActiveSupervisionRefs                                                  = 4082,
    /// ACTUAL-EVENT-REF
    ActualEventRef                                                         = 2907,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 2593,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 1560,
    /// ADAPTIVE-FIREWALL-MODULE-INSTANTIATION
    AdaptiveFirewallModuleInstantiation                                    = 5208,
    /// ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING
    AdaptiveFirewallToPortPrototypeMapping                                 = 1734,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 513,
    /// ADD-INFO-5
    AddInfo5                                                               = 1602,
    /// ADDITIONAL-INFORMATION
    AdditionalInformation                                                  = 5187,
    /// ADDITIONAL-NATIVE-TYPE-QUALIFIER
    AdditionalNativeTypeQualifier                                          = 1197,
    /// ADDRESS
    Address                                                                = 5153,
    /// ADDRESS-CLAIM-ENABLED
    AddressClaimEnabled                                                    = 2634,
    /// ADDRESS-RANGE-LOWER-BOUND
    AddressRangeLowerBound                                                 = 2403,
    /// ADDRESS-RANGE-UPPER-BOUND
    AddressRangeUpperBound                                                 = 2834,
    /// ADDRESS-SEMANTICS
    AddressSemantics                                                       = 5942,
    /// ADDRESSING-FORMAT
    AddressingFormat                                                       = 509,
    /// ADMIN-DATA
    AdminData                                                              = 2832,
    /// ADMIT-WITHOUT-MESSAGE-ID
    AdmitWithoutMessageId                                                  = 665,
    /// AFFECTED-APPLICATION
    AffectedApplication                                                    = 4110,
    /// AFFECTED-BSW-MODULE
    AffectedBswModule                                                      = 5351,
    /// AFFECTED-FUNCTIONAL-CLUSTER
    AffectedFunctionalCluster                                              = 3609,
    /// AFFECTED-REF
    AffectedRef                                                            = 3233,
    /// AFFECTED-REFS
    AffectedRefs                                                           = 2140,
    /// AFFECTED-STATE-IREF
    AffectedStateIref                                                      = 1683,
    /// AFFECTION-KIND
    AffectionKind                                                          = 2723,
    /// AGE
    Age                                                                    = 552,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 3277,
    /// AGGREGATION
    Aggregation                                                            = 2982,
    /// AGGREGATION-CONDITION
    AggregationCondition                                                   = 5563,
    /// AGGREGATION-REF
    AggregationRef                                                         = 4187,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 4052,
    /// AGING-ALLOWED
    AgingAllowed                                                           = 4205,
    /// AGING-CYCLES
    AgingCycles                                                            = 1453,
    /// AGING-REF
    AgingRef                                                               = 5077,
    /// AGING-REQUIRES-TESTED-CYCLE
    AgingRequiresTestedCycle                                               = 1567,
    /// AH-CIPHER-SUITE-NAME
    AhCipherSuiteName                                                      = 276,
    /// AH-CIPHER-SUITE-NAMES
    AhCipherSuiteNames                                                     = 2888,
    /// ALGORITHM-FAMILY
    AlgorithmFamily                                                        = 403,
    /// ALGORITHM-MODE
    AlgorithmMode                                                          = 4376,
    /// ALGORITHM-SECONDARY-FAMILY
    AlgorithmSecondaryFamily                                               = 4263,
    /// ALIAS-EVENT-REF
    AliasEventRef                                                          = 5013,
    /// ALIAS-NAME-ASSIGNMENT
    AliasNameAssignment                                                    = 1458,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 81,
    /// ALIAS-NAMES
    AliasNames                                                             = 3009,
    /// ALIGNMENT
    Alignment                                                              = 1905,
    /// ALIVE-CHECK-RESPONSE-TIMEOUT
    AliveCheckResponseTimeout                                              = 2399,
    /// ALIVE-REFERENCE-CYCLE
    AliveReferenceCycle                                                    = 2855,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 1341,
    /// ALIVE-SUPERVISIONS
    AliveSupervisions                                                      = 5888,
    /// ALIVE-TIMEOUT
    AliveTimeout                                                           = 1885,
    /// ALL-CHANNELS-REF
    AllChannelsRef                                                         = 2527,
    /// ALL-CHANNELS-REFS
    AllChannelsRefs                                                        = 4880,
    /// ALL-NM-MESSAGES-KEEP-AWAKE
    AllNmMessagesKeepAwake                                                 = 219,
    /// ALLOCATE-SHADOW-COPY
    AllocateShadowCopy                                                     = 4116,
    /// ALLOCATOR
    Allocator                                                              = 969,
    /// ALLOCATOR-REF
    AllocatorRef                                                           = 5776,
    /// ALLOW-CONTENT-TYPE-CHANGE
    AllowContentTypeChange                                                 = 700,
    /// ALLOW-DYNAMIC-L-SDU-LENGTH
    AllowDynamicLSduLength                                                 = 5649,
    /// ALLOW-HALT-DUE-TO-CLOCK
    AllowHaltDueToClock                                                    = 5460,
    /// ALLOW-MASTER-RATE-CORRECTION
    AllowMasterRateCorrection                                              = 3784,
    /// ALLOW-PASSIVE-TO-ACTIVE
    AllowPassiveToActive                                                   = 4122,
    /// ALLOW-PROVIDER-RATE-CORRECTION
    AllowProviderRateCorrection                                            = 743,
    /// ALLOW-UNAUTHENTICATED-PARTICIPANTS
    AllowUnauthenticatedParticipants                                       = 4131,
    /// ALLOWED-I-PV-6-EXT-HEADER
    AllowedIPv6ExtHeader                                                   = 2534,
    /// ALLOWED-I-PV-6-EXT-HEADERS
    AllowedIPv6ExtHeaders                                                  = 1550,
    /// ALLOWED-I-PV-6-EXT-HEADERS-REF
    AllowedIPv6ExtHeadersRef                                               = 1790,
    /// ALLOWED-KEYSLOT-USAGE
    AllowedKeyslotUsage                                                    = 5558,
    /// ALLOWED-TCP-OPTION
    AllowedTcpOption                                                       = 3646,
    /// ALLOWED-TCP-OPTIONS
    AllowedTcpOptions                                                      = 1506,
    /// ALLOWED-TCP-OPTIONS-REF
    AllowedTcpOptionsRef                                                   = 945,
    /// ALTERNATIVE-NAME
    AlternativeName                                                        = 1819,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 5354,
    /// ANNOTATION
    Annotation                                                             = 4967,
    /// ANNOTATION-ORIGIN
    AnnotationOrigin                                                       = 4915,
    /// ANNOTATION-TEXT
    AnnotationText                                                         = 5252,
    /// ANNOTATIONS
    Annotations                                                            = 1008,
    /// ANONYMOUS
    Anonymous                                                              = 3902,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 2603,
    /// AP-APPLICATION-ENDPOINTS
    ApApplicationEndpoints                                                 = 4655,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 3347,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 1595,
    /// AP-APPLICATION-ERROR-REF
    ApApplicationErrorRef                                                  = 4154,
    /// AP-APPLICATION-ERROR-REFS
    ApApplicationErrorRefs                                                 = 1536,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 4674,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 5710,
    /// API-PRINCIPLE
    ApiPrinciple                                                           = 559,
    /// API-SERVICE-PREFIX
    ApiServicePrefix                                                       = 3432,
    /// APP-ID
    AppId                                                                  = 4603,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 4217,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS
    AppOsTaskProxyToEcuTaskProxyMappings                                   = 4401,
    /// APP-TASK-PROXY-REF
    AppTaskProxyRef                                                        = 3796,
    /// APPL-CONSTANT-REF
    ApplConstantRef                                                        = 221,
    /// APPL-INIT-VALUE
    ApplInitValue                                                          = 501,
    /// APPL-WATCHDOG-LIMIT
    ApplWatchdogLimit                                                      = 4430,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 3875,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 5231,
    /// APPLICATION-ARRAY-ELEMENT-REF
    ApplicationArrayElementRef                                             = 3764,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 4360,
    /// APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION
    ApplicationAssocMapElementValueSpecification                           = 4473,
    /// APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION
    ApplicationAssocMapValueSpecification                                  = 5518,
    /// APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF
    ApplicationCompositeDataTypeSubElementRef                              = 3100,
    /// APPLICATION-COMPOSITE-ELEMENT-IREF
    ApplicationCompositeElementIref                                        = 5327,
    /// APPLICATION-CONTEXT-REF
    ApplicationContextRef                                                  = 5545,
    /// APPLICATION-CONTEXT-REFS
    ApplicationContextRefs                                                 = 2517,
    /// APPLICATION-DATA-TYPE-REF
    ApplicationDataTypeRef                                                 = 2233,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 2132,
    /// APPLICATION-DESCRIPTION
    ApplicationDescription                                                 = 3735,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 1736,
    /// APPLICATION-ENDPOINT-REF
    ApplicationEndpointRef                                                 = 1887,
    /// APPLICATION-ENDPOINT-REF-CONDITIONAL
    ApplicationEndpointRefConditional                                      = 2304,
    /// APPLICATION-ENTRY
    ApplicationEntry                                                       = 5221,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 2215,
    /// APPLICATION-ID
    ApplicationId                                                          = 4379,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 835,
    /// APPLICATION-MODE-IREF
    ApplicationModeIref                                                    = 2742,
    /// APPLICATION-MODE-MACHINE
    ApplicationModeMachine                                                 = 5150,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 857,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 688,
    /// APPLICATION-PARTITION-REF
    ApplicationPartitionRef                                                = 1358,
    /// APPLICATION-PARTITION-REFS
    ApplicationPartitionRefs                                               = 2821,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 5506,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS
    ApplicationPartitionToEcuPartitionMappings                             = 1641,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 3266,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 3734,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 783,
    /// APPLICATION-RECORD-ELEMENT-REF
    ApplicationRecordElementRef                                            = 5564,
    /// APPLICATION-RULE-BASED-VALUE-SPECIFICATION
    ApplicationRuleBasedValueSpecification                                 = 3572,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 1744,
    /// APPLICATION-TYPE-REF
    ApplicationTypeRef                                                     = 5302,
    /// APPLICATION-TYPE-TREF
    ApplicationTypeTref                                                    = 3049,
    /// APPLICATION-VALUE-SPECIFICATION
    ApplicationValueSpecification                                          = 3174,
    /// APPLICATIONS
    Applications                                                           = 1563,
    /// APPLIES-TO
    AppliesTo                                                              = 2492,
    /// APPLIES-TO-DEPENDENCIES
    AppliesToDependencies                                                  = 5961,
    /// APPROVAL-REQUIRED
    ApprovalRequired                                                       = 3337,
    /// APPROVAL-STATUS
    ApprovalStatus                                                         = 4225,
    /// AR-PACKAGE
    ArPackage                                                              = 3725,
    /// AR-PACKAGES
    ArPackages                                                             = 2621,
    /// AR-PARAMETER
    ArParameter                                                            = 829,
    /// AR-RELEASE-VERSION
    ArReleaseVersion                                                       = 2926,
    /// AR-TYPED-PER-INSTANCE-MEMORY-REF
    ArTypedPerInstanceMemoryRef                                            = 4355,
    /// AR-TYPED-PER-INSTANCE-MEMORYS
    ArTypedPerInstanceMemorys                                              = 5151,
    /// ARBITRARY-ADDRESS-CAPABLE
    ArbitraryAddressCapable                                                = 2827,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 295,
    /// ARBITRATION
    Arbitration                                                            = 4502,
    /// ARBITRATIONS
    Arbitrations                                                           = 3452,
    /// AREA
    Area                                                                   = 5502,
    /// ARGUMENT
    Argument                                                               = 3956,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 1693,
    /// ARGUMENT-MAPPINGS
    ArgumentMappings                                                       = 5322,
    /// ARGUMENT-REF
    ArgumentRef                                                            = 5319,
    /// ARGUMENTS
    Arguments                                                              = 48,
    /// ARGUMENTSS
    Argumentss                                                             = 5783,
    /// ARP-PROPS
    ArpProps                                                               = 341,
    /// ARRAY-ELEMENT-MAPPINGS
    ArrayElementMappings                                                   = 1528,
    /// ARRAY-ELEMENT-REF
    ArrayElementRef                                                        = 458,
    /// ARRAY-IMPL-POLICY
    ArrayImplPolicy                                                        = 5579,
    /// ARRAY-SIZE
    ArraySize                                                              = 1410,
    /// ARRAY-SIZE-HANDLING
    ArraySizeHandling                                                      = 457,
    /// ARRAY-SIZE-SEMANTICS
    ArraySizeSemantics                                                     = 5081,
    /// ARRAY-VALUE-SPECIFICATION
    ArrayValueSpecification                                                = 5306,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 1389,
    /// ARTIFACT-CHECKSUM-REF
    ArtifactChecksumRef                                                    = 5010,
    /// ARTIFACT-CHECKSUM-REFS
    ArtifactChecksumRefs                                                   = 5906,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 4258,
    /// ARTIFACT-CHECKSUMS
    ArtifactChecksums                                                      = 2612,
    /// ARTIFACT-DESCRIPTOR
    ArtifactDescriptor                                                     = 1879,
    /// ARTIFACT-DESCRIPTORS
    ArtifactDescriptors                                                    = 3669,
    /// ARTIFACT-LOCATOR
    ArtifactLocator                                                        = 1080,
    /// ARTIFACT-LOCATORS
    ArtifactLocators                                                       = 3799,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 2617,
    /// ASSERTIONS
    Assertions                                                             = 4448,
    /// ASSIGN-FRAME-ID
    AssignFrameId                                                          = 4667,
    /// ASSIGN-FRAME-ID-RANGE
    AssignFrameIdRange                                                     = 994,
    /// ASSIGN-NAD
    AssignNad                                                              = 3270,
    /// ASSIGNED-CONTROLLER-REF
    AssignedControllerRef                                                  = 5376,
    /// ASSIGNED-DATA-TYPES
    AssignedDataTypes                                                      = 2064,
    /// ASSIGNED-DATAS
    AssignedDatas                                                          = 5173,
    /// ASSIGNED-ENTRY-REF
    AssignedEntryRef                                                       = 2581,
    /// ASSIGNED-ENTRY-ROLES
    AssignedEntryRoles                                                     = 3964,
    /// ASSIGNED-FRAME-TRIGGERING-REF
    AssignedFrameTriggeringRef                                             = 2639,
    /// ASSIGNED-LIN-SLAVE-CONFIG-REF
    AssignedLinSlaveConfigRef                                              = 4796,
    /// ASSIGNED-PORTS
    AssignedPorts                                                          = 2789,
    /// ASSIGNED-TRAFFIC-CLASS
    AssignedTrafficClass                                                   = 883,
    /// ASSIGNED-TRAFFIC-CLASSS
    AssignedTrafficClasss                                                  = 3319,
    /// ASSIGNMENT-PRIORITY
    AssignmentPriority                                                     = 1001,
    /// ASSOCIATED-COM-I-PDU-GROUP-REF
    AssociatedComIPduGroupRef                                              = 260,
    /// ASSOCIATED-COM-I-PDU-GROUP-REFS
    AssociatedComIPduGroupRefs                                             = 4190,
    /// ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    AssociatedConsumedProvidedServiceInstanceGroups                        = 3002,
    /// ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF
    AssociatedCrossSwClusterComRtePluginRef                                = 5729,
    /// ASSOCIATED-EVENT-IDENTIFICATION
    AssociatedEventIdentification                                          = 572,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REF
    AssociatedPdurIPduGroupRef                                             = 3578,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REFS
    AssociatedPdurIPduGroupRefs                                            = 1247,
    /// ASSOCIATED-RTE-PLUGIN-REF
    AssociatedRtePluginRef                                                 = 5422,
    /// ASSUMED-CURRENT-STATE-IREF
    AssumedCurrentStateIref                                                = 2336,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 382,
    /// ASYNCHRONOUS-SERVER-CALL-POINT-REF
    AsynchronousServerCallPointRef                                         = 4511,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 5666,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS
    AsynchronousServerCallResultPoints                                     = 4180,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 170,
    /// ATTRIBUTE-DEFS
    AttributeDefs                                                          = 2636,
    /// ATTRIBUTE-NAME
    AttributeName                                                          = 1430,
    /// ATTRIBUTE-REF
    AttributeRef                                                           = 5786,
    /// ATTRIBUTE-TAILORINGS
    AttributeTailorings                                                    = 1935,
    /// ATTRIBUTE-VALUES
    AttributeValues                                                        = 3096,
    /// ATTRIBUTES
    Attributes                                                             = 3069,
    /// AUDIENCE
    Audience                                                               = 89,
    /// AUDIENCES
    Audiences                                                              = 3808,
    /// AUTH-ALGORITHM
    AuthAlgorithm                                                          = 719,
    /// AUTH-DATA-FRESHNESS-LENGTH
    AuthDataFreshnessLength                                                = 5679,
    /// AUTH-DATA-FRESHNESS-START-POSITION
    AuthDataFreshnessStartPosition                                         = 2114,
    /// AUTH-INFO-TX-LENGTH
    AuthInfoTxLength                                                       = 5516,
    /// AUTHENTIC-CONNECTION-PROPSS
    AuthenticConnectionPropss                                              = 5747,
    /// AUTHENTICATION-BUILD-ATTEMPTS
    AuthenticationBuildAttempts                                            = 3178,
    /// AUTHENTICATION-CLASS-REF
    AuthenticationClassRef                                                 = 1378,
    /// AUTHENTICATION-PROPS-REF
    AuthenticationPropsRef                                                 = 2740,
    /// AUTHENTICATION-PROPSS
    AuthenticationPropss                                                   = 4691,
    /// AUTHENTICATION-REF
    AuthenticationRef                                                      = 5902,
    /// AUTHENTICATION-RETRIES
    AuthenticationRetries                                                  = 4387,
    /// AUTHENTICATION-ROLE-REF
    AuthenticationRoleRef                                                  = 1093,
    /// AUTHENTICATION-ROLE-REFS
    AuthenticationRoleRefs                                                 = 3070,
    /// AUTHENTICATION-TIMEOUT
    AuthenticationTimeout                                                  = 4000,
    /// AUTHENTICATION-VERIFY-ATTEMPTS
    AuthenticationVerifyAttempts                                           = 1930,
    /// AUTO-AVAILABLE
    AutoAvailable                                                          = 1802,
    /// AUTO-COLLECT
    AutoCollect                                                            = 2924,
    /// AUTO-IP-PROPS
    AutoIpProps                                                            = 3140,
    /// AUTO-REQUIRE
    AutoRequire                                                            = 2641,
    /// AUTO-START
    AutoStart                                                              = 781,
    /// AUTOMATIC-END
    AutomaticEnd                                                           = 4295,
    /// AUTOSAR
    Autosar                                                                = 249,
    /// AUTOSAR-CONNECTOR
    AutosarConnector                                                       = 5860,
    /// AUTOSAR-DATA-TYPE-REF
    AutosarDataTypeRef                                                     = 695,
    /// AUTOSAR-DATA-TYPE-REF-CONDITIONAL
    AutosarDataTypeRefConditional                                          = 4460,
    /// AUTOSAR-ENGINEERING-OBJECT
    AutosarEngineeringObject                                               = 1317,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 3300,
    /// AUTOSAR-PARAMETER-IREF
    AutosarParameterIref                                                   = 4952,
    /// AUTOSAR-VARIABLE
    AutosarVariable                                                        = 2017,
    /// AUTOSAR-VARIABLE-IN-IMPL-DATATYPE
    AutosarVariableInImplDatatype                                          = 4493,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 5265,
    /// AUTOSAR-VARIABLE-IREF
    AutosarVariableIref                                                    = 3142,
    /// AUXILIARY-FIELD-DEFINITIONS
    AuxiliaryFieldDefinitions                                              = 4628,
    /// AUXILIARY-FIELDS
    AuxiliaryFields                                                        = 2506,
    /// AVERAGE-MEMORY-CONSUMPTION
    AverageMemoryConsumption                                               = 1428,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 4179,
    /// BANDWIDTH-LIMITATION
    BandwidthLimitation                                                    = 586,
    /// BASE-COMPOSITION-REF
    BaseCompositionRef                                                     = 3883,
    /// BASE-CYCLE
    BaseCycle                                                              = 2280,
    /// BASE-IS-THIS-PACKAGE
    BaseIsThisPackage                                                      = 4662,
    /// BASE-REF
    BaseRef                                                                = 2198,
    /// BASE-REFERENCE-REF
    BaseReferenceRef                                                       = 2724,
    /// BASE-TYPE-ENCODING
    BaseTypeEncoding                                                       = 5909,
    /// BASE-TYPE-REF
    BaseTypeRef                                                            = 3,
    /// BASE-TYPE-SIZE
    BaseTypeSize                                                           = 5949,
    /// BASIC-CYCLE-LENGTH
    BasicCycleLength                                                       = 5368,
    /// BAUDRATE
    Baudrate                                                               = 978,
    /// BEHAVIOR
    Behavior                                                               = 3260,
    /// BEHAVIOR-REF
    BehaviorRef                                                            = 2804,
    /// BEST-CASE-EXECUTION-TIME
    BestCaseExecutionTime                                                  = 2291,
    /// BGCOLOR
    Bgcolor                                                                = 2768,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 990,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 890,
    /// BINARY-MANIFEST-ITEM-NUMERICAL-VALUE
    BinaryManifestItemNumericalValue                                       = 5549,
    /// BINARY-MANIFEST-ITEM-POINTER-VALUE
    BinaryManifestItemPointerValue                                         = 3553,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 1050,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 5752,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 1062,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 5688,
    /// BIT
    Bit                                                                    = 1373,
    /// BIT-NUMBER
    BitNumber                                                              = 1016,
    /// BIT-OFFSET
    BitOffset                                                              = 2986,
    /// BIT-POSITION
    BitPosition                                                            = 59,
    /// BITFIELD-TEXT-TABLE-MASK-FIRST
    BitfieldTextTableMaskFirst                                             = 3847,
    /// BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR
    BitfieldTextTableMaskNvBlockDescriptor                                 = 3378,
    /// BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE
    BitfieldTextTableMaskPortPrototype                                     = 3898,
    /// BITFIELD-TEXT-TABLE-MASK-SECOND
    BitfieldTextTableMaskSecond                                            = 4294,
    /// BLACKLISTED-VERSIONS
    BlacklistedVersions                                                    = 1675,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREF
    BlockIfStateActiveApIref                                               = 1760,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREFS
    BlockIfStateActiveApIrefs                                              = 5737,
    /// BLOCK-IF-STATE-ACTIVE-CP-REF
    BlockIfStateActiveCpRef                                                = 224,
    /// BLOCK-IF-STATE-ACTIVE-CP-REFS
    BlockIfStateActiveCpRefs                                               = 5665,
    /// BLOCK-INCONSISTENT
    BlockInconsistent                                                      = 4094,
    /// BLOCK-STATE
    BlockState                                                             = 1492,
    /// BLOCK-STATES
    BlockStates                                                            = 4458,
    /// BLOCKLISTED-VERSIONS
    BlocklistedVersions                                                    = 2162,
    /// BLUEPRINT-CONDITION
    BlueprintCondition                                                     = 4982,
    /// BLUEPRINT-DERIVATION-GUIDE
    BlueprintDerivationGuide                                               = 5712,
    /// BLUEPRINT-MAPPING
    BlueprintMapping                                                       = 4337,
    /// BLUEPRINT-MAPPING-GUIDE
    BlueprintMappingGuide                                                  = 1889,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 2087,
    /// BLUEPRINT-MAPS
    BlueprintMaps                                                          = 1897,
    /// BLUEPRINT-POLICY-LIST
    BlueprintPolicyList                                                    = 4328,
    /// BLUEPRINT-POLICY-NOT-MODIFIABLE
    BlueprintPolicyNotModifiable                                           = 1753,
    /// BLUEPRINT-POLICY-SINGLE
    BlueprintPolicySingle                                                  = 5035,
    /// BLUEPRINT-POLICYS
    BlueprintPolicys                                                       = 120,
    /// BLUEPRINT-REF
    BlueprintRef                                                           = 718,
    /// BOOLEAN-VALUE-VARIATION-POINT
    BooleanValueVariationPoint                                             = 4215,
    /// BOR-COUNTER-L-1-TO-L-2
    BorCounterL1ToL2                                                       = 2529,
    /// BOR-TIME-L-1
    BorTimeL1                                                              = 370,
    /// BOR-TIME-L-2
    BorTimeL2                                                              = 2801,
    /// BOR-TIME-TX-ENSURED
    BorTimeTxEnsured                                                       = 3250,
    /// BR
    Br                                                                     = 4338,
    /// BROADCAST
    Broadcast                                                              = 3999,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 162,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 585,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 1861,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 243,
    /// BSW-BEHAVIOR-REF
    BswBehaviorRef                                                         = 1217,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 5369,
    /// BSW-CLIENT-POLICY
    BswClientPolicy                                                        = 2623,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 5943,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 1450,
    /// BSW-DATA-SEND-POLICY
    BswDataSendPolicy                                                      = 2184,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 2254,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 1793,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 4128,
    /// BSW-ENTITY-REF
    BswEntityRef                                                           = 3612,
    /// BSW-ENTRY-KIND
    BswEntryKind                                                           = 5063,
    /// BSW-ENTRY-RELATIONSHIP
    BswEntryRelationship                                                   = 4075,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 5566,
    /// BSW-ENTRY-RELATIONSHIP-TYPE
    BswEntryRelationshipType                                               = 5901,
    /// BSW-ENTRY-RELATIONSHIPS
    BswEntryRelationships                                                  = 807,
    /// BSW-EXCLUSIVE-AREA-POLICY
    BswExclusiveAreaPolicy                                                 = 2853,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 1649,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 5273,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 4427,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 2578,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 1678,
    /// BSW-INTERNAL-TRIGGERING-POINT-POLICY
    BswInternalTriggeringPointPolicy                                       = 3244,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF
    BswInternalTriggeringPointRef                                          = 5428,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL
    BswInternalTriggeringPointRefConditional                               = 5932,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 5258,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 2778,
    /// BSW-MODE-GROUP-REF
    BswModeGroupRef                                                        = 5670,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 5103,
    /// BSW-MODE-RECEIVER-POLICY
    BswModeReceiverPolicy                                                  = 2828,
    /// BSW-MODE-SENDER-POLICY
    BswModeSenderPolicy                                                    = 2767,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 3761,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 4702,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 1054,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 3033,
    /// BSW-MODULE-DEPENDENCYS
    BswModuleDependencys                                                   = 4054,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 5147,
    /// BSW-MODULE-DESCRIPTION-REF
    BswModuleDescriptionRef                                                = 1033,
    /// BSW-MODULE-DESCRIPTION-REF-CONDITIONAL
    BswModuleDescriptionRefConditional                                     = 4814,
    /// BSW-MODULE-DOCUMENTATIONS
    BswModuleDocumentations                                                = 4631,
    /// BSW-MODULE-ENTITY-REF
    BswModuleEntityRef                                                     = 3670,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 4810,
    /// BSW-MODULE-ENTRY-REF
    BswModuleEntryRef                                                      = 5241,
    /// BSW-MODULE-ENTRY-REF-CONDITIONAL
    BswModuleEntryRefConditional                                           = 982,
    /// BSW-MODULE-INSTANCE-REF
    BswModuleInstanceRef                                                   = 2892,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 5379,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 4898,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 3482,
    /// BSW-PARAMETER-POLICY
    BswParameterPolicy                                                     = 1970,
    /// BSW-PER-INSTANCE-MEMORY-POLICY
    BswPerInstanceMemoryPolicy                                             = 3291,
    /// BSW-PER-INSTANCE-MEMORY-POLICYS
    BswPerInstanceMemoryPolicys                                            = 485,
    /// BSW-QUEUED-DATA-RECEPTION-POLICY
    BswQueuedDataReceptionPolicy                                           = 2219,
    /// BSW-RELEASED-TRIGGER-POLICY
    BswReleasedTriggerPolicy                                               = 5454,
    /// BSW-RESOLUTION
    BswResolution                                                          = 2816,
    /// BSW-RESOURCE-ESTIMATION
    BswResourceEstimation                                                  = 3478,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 2185,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 5255,
    /// BSW-SERVICE-DEPENDENCY
    BswServiceDependency                                                   = 522,
    /// BSW-SERVICE-DEPENDENCY-REF
    BswServiceDependencyRef                                                = 2156,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 5025,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 4918,
    /// BSW-TRIGGER-DIRECT-IMPLEMENTATION
    BswTriggerDirectImplementation                                         = 2310,
    /// BSW-TRIGGER-REF
    BswTriggerRef                                                          = 425,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 4692,
    /// BUCKET-SIZE
    BucketSize                                                             = 289,
    /// BUFFER-COMPUTATION
    BufferComputation                                                      = 169,
    /// BUFFER-OUTPUT
    BufferOutput                                                           = 3803,
    /// BUFFER-PROPERTIES
    BufferProperties                                                       = 5721,
    /// BUFFER-RATIO
    BufferRatio                                                            = 4478,
    /// BUILD-ACTION
    BuildAction                                                            = 5531,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 3862,
    /// BUILD-ACTION-ENVIRONMENTS
    BuildActionEnvironments                                                = 574,
    /// BUILD-ACTION-IO-ELEMENT
    BuildActionIoElement                                                   = 4069,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 3419,
    /// BUILD-ACTION-MANIFEST-REF
    BuildActionManifestRef                                                 = 2876,
    /// BUILD-ACTION-MANIFEST-REF-CONDITIONAL
    BuildActionManifestRefConditional                                      = 4639,
    /// BUILD-ACTION-MANIFESTS
    BuildActionManifests                                                   = 3757,
    /// BUILD-ACTIONS
    BuildActions                                                           = 4457,
    /// BUILD-TYPE
    BuildType                                                              = 1434,
    /// BULK-NV-BLOCK
    BulkNvBlock                                                            = 3302,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 752,
    /// BULK-NV-DATA-DESCRIPTORS
    BulkNvDataDescriptors                                                  = 2803,
    /// BUNDLED-CONNECTIONS
    BundledConnections                                                     = 478,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 4868,
    /// BURST-SIZE
    BurstSize                                                              = 4670,
    /// BURST-SIZE-REQUEST
    BurstSizeRequest                                                       = 316,
    /// BURST-SIZE-RESPONSE
    BurstSizeResponse                                                      = 5508,
    /// BUS-DEPENDENT-NM-ECUS
    BusDependentNmEcus                                                     = 5045,
    /// BUS-IDLE-TIMEOUT-PERIOD
    BusIdleTimeoutPeriod                                                   = 5622,
    /// BUS-MIRROR-CAN-ID-RANGE-MAPPING
    BusMirrorCanIdRangeMapping                                             = 3431,
    /// BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING
    BusMirrorCanIdToCanIdMapping                                           = 4728,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 1589,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 4977,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 3962,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 2376,
    /// BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING
    BusMirrorLinPidToCanIdMapping                                          = 575,
    /// BUS-MIRROR-NETWORK-ID
    BusMirrorNetworkId                                                     = 470,
    /// BUS-OFF-RECOVERY
    BusOffRecovery                                                         = 3519,
    /// BUS-SPECIFIC-NM-ECU
    BusSpecificNmEcu                                                       = 3503,
    /// BY-PASS-POINT-IREF
    ByPassPointIref                                                        = 2699,
    /// BY-PASS-POINT-IREFS
    ByPassPointIrefs                                                       = 564,
    /// BYPASS-ETHER-TYPE
    BypassEtherType                                                        = 5399,
    /// BYPASS-ETHER-TYPES
    BypassEtherTypes                                                       = 525,
    /// BYPASS-VLAN
    BypassVlan                                                             = 4589,
    /// BYPASS-VLANS
    BypassVlans                                                            = 5432,
    /// BYTE
    Byte                                                                   = 1728,
    /// BYTE-ORDER
    ByteOrder                                                              = 4847,
    /// BYTE-VALUE
    ByteValue                                                              = 3105,
    /// BYTE-VALUES
    ByteValues                                                             = 510,
    /// C-CODE
    CCode                                                                  = 2161,
    /// CALC-RAM-BLOCK-CRC
    CalcRamBlockCrc                                                        = 3382,
    /// CALCULATION-FORMULA
    CalculationFormula                                                     = 962,
    /// CALIBRATION-PARAMETER-VALUE
    CalibrationParameterValue                                              = 5718,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 3817,
    /// CALIBRATION-PARAMETER-VALUE-SET-REF
    CalibrationParameterValueSetRef                                        = 1751,
    /// CALIBRATION-PARAMETER-VALUE-SET-REFS
    CalibrationParameterValueSetRefs                                       = 2957,
    /// CALIBRATION-PARAMETER-VALUES
    CalibrationParameterValues                                             = 467,
    /// CALL-POINTS
    CallPoints                                                             = 3073,
    /// CALL-SIGNAL-REF
    CallSignalRef                                                          = 2713,
    /// CALL-SIGNAL-TRIGGERING-REF
    CallSignalTriggeringRef                                                = 2387,
    /// CALL-TIMEOUT
    CallTimeout                                                            = 4018,
    /// CALL-TYPE
    CallType                                                               = 1559,
    /// CALLBACK-HEADER-REF
    CallbackHeaderRef                                                      = 471,
    /// CALLBACK-HEADER-REFS
    CallbackHeaderRefs                                                     = 5199,
    /// CALLED-ENTRY-REF
    CalledEntryRef                                                         = 3361,
    /// CALLED-ENTRYS
    CalledEntrys                                                           = 5755,
    /// CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF
    CalledFromWithinExclusiveAreaRef                                       = 3384,
    /// CAN-ADDRESSING-MODE
    CanAddressingMode                                                      = 2942,
    /// CAN-BAUDRATE
    CanBaudrate                                                            = 2738,
    /// CAN-BE-INVOKED-CONCURRENTLY
    CanBeInvokedConcurrently                                               = 1128,
    /// CAN-CLUSTER
    CanCluster                                                             = 336,
    /// CAN-CLUSTER-CONDITIONAL
    CanClusterConditional                                                  = 4999,
    /// CAN-CLUSTER-VARIANTS
    CanClusterVariants                                                     = 1807,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 1412,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 2493,
    /// CAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    CanCommunicationControllerConditional                                  = 2777,
    /// CAN-COMMUNICATION-CONTROLLER-VARIANTS
    CanCommunicationControllerVariants                                     = 5276,
    /// CAN-CONFIG
    CanConfig                                                              = 1495,
    /// CAN-CONTROLLER-ATTRIBUTES
    CanControllerAttributes                                                = 2419,
    /// CAN-CONTROLLER-CONFIGURATION
    CanControllerConfiguration                                             = 699,
    /// CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS
    CanControllerConfigurationRequirements                                 = 1537,
    /// CAN-CONTROLLER-FD-ATTRIBUTES
    CanControllerFdAttributes                                              = 5647,
    /// CAN-CONTROLLER-FD-REQUIREMENTS
    CanControllerFdRequirements                                            = 3082,
    /// CAN-CONTROLLER-XL-ATTRIBUTES
    CanControllerXlAttributes                                              = 2278,
    /// CAN-CONTROLLER-XL-REQUIREMENTS
    CanControllerXlRequirements                                            = 3470,
    /// CAN-ENTER-EXCLUSIVE-AREA-REF
    CanEnterExclusiveAreaRef                                               = 626,
    /// CAN-ENTER-EXCLUSIVE-AREA-REFS
    CanEnterExclusiveAreaRefs                                              = 674,
    /// CAN-ENTERS
    CanEnters                                                              = 4933,
    /// CAN-FD-BAUDRATE
    CanFdBaudrate                                                          = 4652,
    /// CAN-FD-CONFIG
    CanFdConfig                                                            = 4256,
    /// CAN-FD-FRAME-SUPPORT
    CanFdFrameSupport                                                      = 3374,
    /// CAN-FRAME
    CanFrame                                                               = 2584,
    /// CAN-FRAME-RX-BEHAVIOR
    CanFrameRxBehavior                                                     = 3351,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 633,
    /// CAN-FRAME-TX-BEHAVIOR
    CanFrameTxBehavior                                                     = 4908,
    /// CAN-GLOBAL-TIME-DOMAIN-PROPS
    CanGlobalTimeDomainProps                                               = 4444,
    /// CAN-ID-RANGE-MAPPINGS
    CanIdRangeMappings                                                     = 4945,
    /// CAN-ID-TO-CAN-ID-MAPPINGS
    CanIdToCanIdMappings                                                   = 3569,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 3804,
    /// CAN-NM-CLUSTER-COUPLING
    CanNmClusterCoupling                                                   = 100,
    /// CAN-NM-ECU
    CanNmEcu                                                               = 5001,
    /// CAN-NM-NODE
    CanNmNode                                                              = 595,
    /// CAN-NM-TX-CAN-ID
    CanNmTxCanId                                                           = 923,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 5599,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 5938,
    /// CAN-TP-ADDRESS-REF
    CanTpAddressRef                                                        = 3450,
    /// CAN-TP-ADDRESS-REFS
    CanTpAddressRefs                                                       = 5365,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 3801,
    /// CAN-TP-CHANNEL-REF
    CanTpChannelRef                                                        = 4934,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 4008,
    /// CAN-TP-CONNECTION
    CanTpConnection                                                        = 2235,
    /// CAN-TP-ECU
    CanTpEcu                                                               = 2320,
    /// CAN-TP-NODE
    CanTpNode                                                              = 5872,
    /// CAN-XL-BAUDRATE
    CanXlBaudrate                                                          = 250,
    /// CAN-XL-CONFIG
    CanXlConfig                                                            = 4377,
    /// CAN-XL-CONFIG-REF
    CanXlConfigRef                                                         = 4061,
    /// CAN-XL-CONFIG-REQS
    CanXlConfigReqs                                                        = 422,
    /// CAN-XL-FRAME-TRIGGERING-PROPS
    CanXlFrameTriggeringProps                                              = 3815,
    /// CAN-XL-NM-PROPS
    CanXlNmProps                                                           = 3038,
    /// CAN-XL-PROPS
    CanXlProps                                                             = 563,
    /// CAN-XL-PROPS-REF
    CanXlPropsRef                                                          = 4537,
    /// CAN-XL-PROPS-REFS
    CanXlPropsRefs                                                         = 2349,
    /// CANCELLATION
    Cancellation                                                           = 4620,
    /// CAPABILITY
    Capability                                                             = 1643,
    /// CAPABILITY-RECORDS
    CapabilityRecords                                                      = 850,
    /// CAPTION
    Caption                                                                = 2682,
    /// CAS-RX-LOW-MAX
    CasRxLowMax                                                            = 2756,
    /// CAT-2-ISR
    Cat2Isr                                                                = 5887,
    /// CATEGORY
    Category                                                               = 230,
    /// CDD-TYPE
    CddType                                                                = 5899,
    /// CERT-COMMON-NAME
    CertCommonName                                                         = 1153,
    /// CERTIFICATE-REF
    CertificateRef                                                         = 5584,
    /// CERTIFICATE-TO-KEY-SLOT-MAPPINGS
    CertificateToKeySlotMappings                                           = 1798,
    /// CHANGE
    Change                                                                 = 3093,
    /// CHANNEL-ID
    ChannelId                                                              = 1231,
    /// CHANNEL-MODE
    ChannelMode                                                            = 5772,
    /// CHANNEL-NAME
    ChannelName                                                            = 915,
    /// CHANNEL-REF
    ChannelRef                                                             = 1821,
    /// CHANNELS
    Channels                                                               = 3598,
    /// CHAPTER
    Chapter                                                                = 3546,
    /// CHAPTERS
    Chapters                                                               = 2451,
    /// CHECK-PER-SOURCE-ID
    CheckPerSourceId                                                       = 2007,
    /// CHECK-STATIC-BLOCK-ID
    CheckStaticBlockId                                                     = 5429,
    /// CHECKPOINT-FINAL-REF
    CheckpointFinalRef                                                     = 1782,
    /// CHECKPOINT-FINAL-REFS
    CheckpointFinalRefs                                                    = 2658,
    /// CHECKPOINT-ID
    CheckpointId                                                           = 5416,
    /// CHECKPOINT-INITIAL-REF
    CheckpointInitialRef                                                   = 5711,
    /// CHECKPOINT-INITIAL-REFS
    CheckpointInitialRefs                                                  = 664,
    /// CHECKPOINT-IREF
    CheckpointIref                                                         = 5440,
    /// CHECKPOINT-REF
    CheckpointRef                                                          = 5877,
    /// CHECKPOINT-REFS
    CheckpointRefs                                                         = 1846,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 178,
    /// CHECKPOINT-TRANSITION-REF
    CheckpointTransitionRef                                                = 1699,
    /// CHECKPOINTS
    Checkpoints                                                            = 2673,
    /// CHECKPOINTSS
    Checkpointss                                                           = 1021,
    /// CHECKSUM-VALUE
    ChecksumValue                                                          = 3216,
    /// CHECKSUM-VERIFICATION
    ChecksumVerification                                                   = 3326,
    /// CHOICES
    Choices                                                                = 3818,
    /// CIPHER-SUITE
    CipherSuite                                                            = 942,
    /// CIPHER-SUITE-CONFIGS
    CipherSuiteConfigs                                                     = 4356,
    /// CIPHER-SUITE-ID
    CipherSuiteId                                                          = 5578,
    /// CIPHER-SUITE-NAME
    CipherSuiteName                                                        = 2689,
    /// CIPHER-SUITE-PRIORITY
    CipherSuitePriority                                                    = 3596,
    /// CIPHER-SUITE-SHORT-LABEL
    CipherSuiteShortLabel                                                  = 4068,
    /// CKN-REF
    CknRef                                                                 = 1402,
    /// CLAIMED-FUNCTION-GROUP-REF
    ClaimedFunctionGroupRef                                                = 4509,
    /// CLAIMED-FUNCTION-GROUP-REFS
    ClaimedFunctionGroupRefs                                               = 4830,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 2546,
    /// CLASS-CONTENTS
    ClassContents                                                          = 140,
    /// CLASS-TAILORINGS
    ClassTailorings                                                        = 1125,
    /// CLASSIFICATION
    Classification                                                         = 282,
    /// CLASSIFICATIONS
    Classifications                                                        = 3000,
    /// CLEAR-CONDITION-GROUP-REF
    ClearConditionGroupRef                                                 = 294,
    /// CLEAR-CONDITION-REF
    ClearConditionRef                                                      = 72,
    /// CLEAR-CONDITION-REFS
    ClearConditionRefs                                                     = 4787,
    /// CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF
    ClearDiagnosticInformationClassRef                                     = 32,
    /// CLEAR-DTC-LIMITATION
    ClearDtcLimitation                                                     = 992,
    /// CLEAR-EVENT-ALLOWED-BEHAVIOR
    ClearEventAllowedBehavior                                              = 1346,
    /// CLEAR-EVENT-BEHAVIOR
    ClearEventBehavior                                                     = 3783,
    /// CLEAR-FROM-VALID-TO-INVALID
    ClearFromValidToInvalid                                                = 701,
    /// CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF
    ClearResetEmissionRelatedDiagnosticInfoClassRef                        = 1605,
    /// CLIENT-CAPABILITY
    ClientCapability                                                       = 708,
    /// CLIENT-COM-SPEC
    ClientComSpec                                                          = 3133,
    /// CLIENT-ID
    ClientId                                                               = 3508,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 622,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 44,
    /// CLIENT-ID-DEFINITION-SET-REF
    ClientIdDefinitionSetRef                                               = 2257,
    /// CLIENT-ID-DEFINITION-SET-REFS
    ClientIdDefinitionSetRefs                                              = 1865,
    /// CLIENT-ID-DEFINITIONS
    ClientIdDefinitions                                                    = 3715,
    /// CLIENT-ID-RANGE
    ClientIdRange                                                          = 5572,
    /// CLIENT-INTENT
    ClientIntent                                                           = 2151,
    /// CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST
    ClientIpAddrFromConnectionRequest                                      = 2343,
    /// CLIENT-POLICYS
    ClientPolicys                                                          = 4363,
    /// CLIENT-PORT-FROM-CONNECTION-REQUEST
    ClientPortFromConnectionRequest                                        = 1078,
    /// CLIENT-PORT-REF
    ClientPortRef                                                          = 2122,
    /// CLIENT-SERVER-ANNOTATION
    ClientServerAnnotation                                                 = 4802,
    /// CLIENT-SERVER-ANNOTATIONS
    ClientServerAnnotations                                                = 848,
    /// CLIENT-SERVER-APPLICATION-ERROR-MAPPING
    ClientServerApplicationErrorMapping                                    = 436,
    /// CLIENT-SERVER-ARRAY-ELEMENT-MAPPING
    ClientServerArrayElementMapping                                        = 5514,
    /// CLIENT-SERVER-ARRAY-TYPE-MAPPING
    ClientServerArrayTypeMapping                                           = 15,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 2095,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 5305,
    /// CLIENT-SERVER-INTERFACE-REF
    ClientServerInterfaceRef                                               = 791,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 4380,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 3258,
    /// CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING
    ClientServerOperationBlueprintMapping                                  = 154,
    /// CLIENT-SERVER-OPERATION-COM-PROPS
    ClientServerOperationComProps                                          = 1345,
    /// CLIENT-SERVER-OPERATION-IREF
    ClientServerOperationIref                                              = 3819,
    /// CLIENT-SERVER-OPERATION-MAPPING
    ClientServerOperationMapping                                           = 1375,
    /// CLIENT-SERVER-OPERATION-REF
    ClientServerOperationRef                                               = 312,
    /// CLIENT-SERVER-PORTS
    ClientServerPorts                                                      = 4775,
    /// CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING
    ClientServerPrimitiveTypeMapping                                       = 4041,
    /// CLIENT-SERVER-RECORD-ELEMENT-MAPPING
    ClientServerRecordElementMapping                                       = 99,
    /// CLIENT-SERVER-RECORD-TYPE-MAPPING
    ClientServerRecordTypeMapping                                          = 3012,
    /// CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING
    ClientServerToSignalGroupMapping                                       = 1244,
    /// CLIENT-SERVER-TO-SIGNAL-MAPPING
    ClientServerToSignalMapping                                            = 4224,
    /// CLIENT-SERVICE-MAJOR-VERSION
    ClientServiceMajorVersion                                              = 5372,
    /// CLIENT-SERVICE-MINOR-VERSION
    ClientServiceMinorVersion                                              = 3886,
    /// CLOCK-REFERENCE-REF
    ClockReferenceRef                                                      = 1658,
    /// CLUSTER-DRIFT-DAMPING
    ClusterDriftDamping                                                    = 4159,
    /// CLUSTERED-COMPONENT-IREF
    ClusteredComponentIref                                                 = 1340,
    /// CLUSTERED-COMPONENT-IREFS
    ClusteredComponentIrefs                                                = 3246,
    /// CODE
    Code                                                                   = 5065,
    /// CODE-DESCRIPTORS
    CodeDescriptors                                                        = 5301,
    /// CODE-LABEL
    CodeLabel                                                              = 3457,
    /// COLD-START-ATTEMPTS
    ColdStartAttempts                                                      = 917,
    /// COLLECTED-INSTANCE-IREF
    CollectedInstanceIref                                                  = 5841,
    /// COLLECTED-INSTANCE-IREFS
    CollectedInstanceIrefs                                                 = 3624,
    /// COLLECTION
    Collection                                                             = 1922,
    /// COLLECTION-PROPS
    CollectionProps                                                        = 186,
    /// COLLECTION-REF
    CollectionRef                                                          = 4481,
    /// COLLECTION-SEMANTICS
    CollectionSemantics                                                    = 3166,
    /// COLLISION-RESOLVING-SCHEDULE-REF
    CollisionResolvingScheduleRef                                          = 5094,
    /// COLSPEC
    Colspec                                                                = 2022,
    /// COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS
    ComBasedSignalGroupTransformations                                     = 4418,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 2750,
    /// COM-CONFIGURATION-GW-TIME-BASE
    ComConfigurationGwTimeBase                                             = 1662,
    /// COM-CONFIGURATION-ID
    ComConfigurationId                                                     = 3710,
    /// COM-CONFIGURATION-RX-TIME-BASE
    ComConfigurationRxTimeBase                                             = 868,
    /// COM-CONFIGURATION-TX-TIME-BASE
    ComConfigurationTxTimeBase                                             = 3571,
    /// COM-CONTROL-CLASS-REF
    ComControlClassRef                                                     = 598,
    /// COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION
    ComEnableMdtForCyclicTransmission                                      = 4438,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 3631,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 3772,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 1225,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 4508,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 2442,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 555,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 3165,
    /// COM-MANAGEMENT-GROUP-REF
    ComManagementGroupRef                                                  = 4144,
    /// COM-MANAGEMENT-GROUP-REFS
    ComManagementGroupRefs                                                 = 2684,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 2640,
    /// COM-MANAGEMENT-MAPPINGS
    ComManagementMappings                                                  = 4644,
    /// COM-MANAGEMENT-PORT-GROUP-IREF
    ComManagementPortGroupIref                                             = 1291,
    /// COM-MANAGEMENT-PORT-GROUP-IREFS
    ComManagementPortGroupIrefs                                            = 4979,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 3579,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 3549,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 3622,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 4134,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 5409,
    /// COM-PROCESSING-PERIOD
    ComProcessingPeriod                                                    = 103,
    /// COM-PROPS
    ComProps                                                               = 4488,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 3089,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 3541,
    /// COMM-CONNECTORS
    CommConnectors                                                         = 4950,
    /// COMM-CONTROLLER-MAPPINGS
    CommControllerMappings                                                 = 2078,
    /// COMM-CONTROLLER-REF
    CommControllerRef                                                      = 1314,
    /// COMM-CONTROLLERS
    CommControllers                                                        = 1140,
    /// COMMAND
    Command                                                                = 1061,
    /// COMMANDS
    Commands                                                               = 1471,
    /// COMMENT
    Comment                                                                = 4311,
    /// COMMON-PROPERTIES
    CommonProperties                                                       = 1521,
    /// COMMON-SIGNAL-PATH
    CommonSignalPath                                                       = 1311,
    /// COMMUNICATION-BUFFER-LOCKING
    CommunicationBufferLocking                                             = 3570,
    /// COMMUNICATION-CLUSTER-REF
    CommunicationClusterRef                                                = 1645,
    /// COMMUNICATION-CLUSTER-REFS
    CommunicationClusterRefs                                               = 5004,
    /// COMMUNICATION-CONNECTOR-REF
    CommunicationConnectorRef                                              = 4185,
    /// COMMUNICATION-CONNECTOR-REF-CONDITIONAL
    CommunicationConnectorRefConditional                                   = 5371,
    /// COMMUNICATION-CONNECTOR-REFS
    CommunicationConnectorRefs                                             = 1446,
    /// COMMUNICATION-CONNECTORS
    CommunicationConnectors                                                = 2128,
    /// COMMUNICATION-CONTROLLER-MAPPING
    CommunicationControllerMapping                                         = 3489,
    /// COMMUNICATION-CONTROLLER-REF
    CommunicationControllerRef                                             = 5250,
    /// COMMUNICATION-CONTROLLERS
    CommunicationControllers                                               = 2196,
    /// COMMUNICATION-CYCLE
    CommunicationCycle                                                     = 4221,
    /// COMMUNICATION-DIRECTION
    CommunicationDirection                                                 = 677,
    /// COMMUNICATION-MODE
    CommunicationMode                                                      = 5304,
    /// COMMUNICATION-RESOURCE-PROPS
    CommunicationResourceProps                                             = 2481,
    /// COMMUNICATION-RESOURCE-REF
    CommunicationResourceRef                                               = 1283,
    /// COMPARE-TYPE
    CompareType                                                            = 3211,
    /// COMPARE-VALUE
    CompareValue                                                           = 1309,
    /// COMPILER
    Compiler                                                               = 1427,
    /// COMPILERS
    Compilers                                                              = 5285,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 5633,
    /// COMPLEX-TYPE-MAPPING
    ComplexTypeMapping                                                     = 5699,
    /// COMPONENT-CLUSTERING
    ComponentClustering                                                    = 1372,
    /// COMPONENT-IMPLEMENTATION-REF
    ComponentImplementationRef                                             = 5921,
    /// COMPONENT-IREF
    ComponentIref                                                          = 419,
    /// COMPONENT-IREFS
    ComponentIrefs                                                         = 2289,
    /// COMPONENT-QUALIFIER
    ComponentQualifier                                                     = 1955,
    /// COMPONENT-REF
    ComponentRef                                                           = 1833,
    /// COMPONENT-SEPARATION
    ComponentSeparation                                                    = 3844,
    /// COMPONENTS
    Components                                                             = 932,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 787,
    /// COMPOSITE-NETWORK-REPRESENTATION
    CompositeNetworkRepresentation                                         = 1901,
    /// COMPOSITE-NETWORK-REPRESENTATIONS
    CompositeNetworkRepresentations                                        = 5307,
    /// COMPOSITE-RULE-BASED-VALUE-SPECIFICATION
    CompositeRuleBasedValueSpecification                                   = 2246,
    /// COMPOSITE-SERVICE-INTERFACE-REF
    CompositeServiceInterfaceRef                                           = 3130,
    /// COMPOSITE-TYPE-MAPPINGS
    CompositeTypeMappings                                                  = 5005,
    /// COMPOSITION-DATA-PROTOTYPE-REF
    CompositionDataPrototypeRef                                            = 3788,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 2885,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 3512,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 12,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF
    CompositionSwComponentTypeRef                                          = 1216,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL
    CompositionSwComponentTypeRefConditional                               = 2353,
    /// COMPOUND-PRIMITIVE-ARGUMENTS
    CompoundPrimitiveArguments                                             = 5595,
    /// COMPRESSED-SOFTWARE-PACKAGE-SIZE
    CompressedSoftwarePackageSize                                          = 128,
    /// COMPU-CONST
    CompuConst                                                             = 1052,
    /// COMPU-DEFAULT-VALUE
    CompuDefaultValue                                                      = 5731,
    /// COMPU-DENOMINATOR
    CompuDenominator                                                       = 4668,
    /// COMPU-INTERNAL-TO-PHYS
    CompuInternalToPhys                                                    = 5378,
    /// COMPU-INVERSE-VALUE
    CompuInverseValue                                                      = 5797,
    /// COMPU-METHOD
    CompuMethod                                                            = 2282,
    /// COMPU-METHOD-REF
    CompuMethodRef                                                         = 1924,
    /// COMPU-NUMERATOR
    CompuNumerator                                                         = 751,
    /// COMPU-PHYS-TO-INTERNAL
    CompuPhysToInternal                                                    = 2721,
    /// COMPU-RATIONAL-COEFFS
    CompuRationalCoeffs                                                    = 656,
    /// COMPU-SCALE
    CompuScale                                                             = 5598,
    /// COMPU-SCALES
    CompuScales                                                            = 2142,
    /// COMPUTED
    Computed                                                               = 2001,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 1308,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 2083,
    /// COND
    Cond                                                                   = 3370,
    /// CONDITION
    Condition                                                              = 2422,
    /// CONDITION-ACCESS
    ConditionAccess                                                        = 258,
    /// CONDITION-FORMULA
    ConditionFormula                                                       = 4888,
    /// CONDITIONAL-CHANGE-NAD
    ConditionalChangeNad                                                   = 5328,
    /// CONDITIONS
    Conditions                                                             = 3965,
    /// CONFIDENCE-INTERVAL
    ConfidenceInterval                                                     = 5678,
    /// CONFIDENCE-INTERVALS
    ConfidenceIntervals                                                    = 1179,
    /// CONFIDENTIALITY-OFFSET
    ConfidentialityOffset                                                  = 152,
    /// CONFIG-CLASS
    ConfigClass                                                            = 4319,
    /// CONFIG-ELEMENT-DEF-GLOBAL-REF
    ConfigElementDefGlobalRef                                              = 4044,
    /// CONFIG-ELEMENT-DEF-LOCAL-REF
    ConfigElementDefLocalRef                                               = 5178,
    /// CONFIG-VARIANT
    ConfigVariant                                                          = 2959,
    /// CONFIGURATION-CLASS-AFFECTION
    ConfigurationClassAffection                                            = 2224,
    /// CONFIGURATION-HANDLING
    ConfigurationHandling                                                  = 2520,
    /// CONFIGURED-NAD
    ConfiguredNad                                                          = 5882,
    /// CONFIRMATION-THRESHOLD
    ConfirmationThreshold                                                  = 1423,
    /// CONFLICTS
    Conflicts                                                              = 345,
    /// CONFLICTS-TO
    ConflictsTo                                                            = 2516,
    /// CONNECT
    Connect                                                                = 4624,
    /// CONNECTED-INDICATORS
    ConnectedIndicators                                                    = 988,
    /// CONNECTION-BUNDLES
    ConnectionBundles                                                      = 4690,
    /// CONNECTION-IS-MANDATORY
    ConnectionIsMandatory                                                  = 5682,
    /// CONNECTION-NEGOTIATION-BEHAVIOR
    ConnectionNegotiationBehavior                                          = 4254,
    /// CONNECTION-PRIO-PDUS
    ConnectionPrioPdus                                                     = 2771,
    /// CONNECTION-TYPE
    ConnectionType                                                         = 1809,
    /// CONNECTIONS
    Connections                                                            = 3023,
    /// CONNECTOR-REF
    ConnectorRef                                                           = 3703,
    /// CONNECTOR-REFS
    ConnectorRefs                                                          = 3064,
    /// CONNECTORS
    Connectors                                                             = 650,
    /// CONSIDER-BUILD-NUMBER
    ConsiderBuildNumber                                                    = 5768,
    /// CONSIDER-PTO-STATUS
    ConsiderPtoStatus                                                      = 2031,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 1620,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 2784,
    /// CONSISTENCY-NEEDSS
    ConsistencyNeedss                                                      = 815,
    /// CONSTANT-MAPPING-REF
    ConstantMappingRef                                                     = 1057,
    /// CONSTANT-MAPPING-REFS
    ConstantMappingRefs                                                    = 1653,
    /// CONSTANT-MEMORYS
    ConstantMemorys                                                        = 1715,
    /// CONSTANT-REF
    ConstantRef                                                            = 5032,
    /// CONSTANT-REFERENCE
    ConstantReference                                                      = 2866,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 4964,
    /// CONSTANT-SPECIFICATION-MAPPING
    ConstantSpecificationMapping                                           = 3494,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 2716,
    /// CONSTANT-VALUE-MAPPING-REF
    ConstantValueMappingRef                                                = 448,
    /// CONSTANT-VALUE-MAPPING-REFS
    ConstantValueMappingRefs                                               = 4552,
    /// CONSTR-LEVEL
    ConstrLevel                                                            = 5574,
    /// CONSTRAINT-REF
    ConstraintRef                                                          = 5391,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 1019,
    /// CONSTRAINT-TAILORINGS
    ConstraintTailorings                                                   = 2590,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 1870,
    /// CONSUMED-EVENT-GROUP-REF
    ConsumedEventGroupRef                                                  = 4591,
    /// CONSUMED-EVENT-GROUP-REFS
    ConsumedEventGroupRefs                                                 = 5954,
    /// CONSUMED-EVENT-GROUPS
    ConsumedEventGroups                                                    = 1463,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 2262,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF
    ConsumedProvidedServiceInstanceGroupRef                                = 5540,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL
    ConsumedProvidedServiceInstanceGroupRefConditional                     = 5789,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 4632,
    /// CONSUMED-SERVICE-INSTANCE-REF
    ConsumedServiceInstanceRef                                             = 3240,
    /// CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL
    ConsumedServiceInstanceRefConditional                                  = 4721,
    /// CONSUMED-SERVICE-INSTANCES
    ConsumedServiceInstances                                               = 3377,
    /// CONTAINED-AR-ELEMENT-REF
    ContainedArElementRef                                                  = 5079,
    /// CONTAINED-AR-ELEMENT-REFS
    ContainedArElementRefs                                                 = 355,
    /// CONTAINED-FIBEX-ELEMENT-REF
    ContainedFibexElementRef                                               = 1773,
    /// CONTAINED-FIBEX-ELEMENT-REFS
    ContainedFibexElementRefs                                              = 1530,
    /// CONTAINED-I-PDU-PROPS
    ContainedIPduProps                                                     = 3290,
    /// CONTAINED-I-PDU-TRIGGERING-PROPSS
    ContainedIPduTriggeringPropss                                          = 4465,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REF
    ContainedISignalIPduGroupRef                                           = 4756,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS
    ContainedISignalIPduGroupRefs                                          = 1502,
    /// CONTAINED-PACKAGE-ELEMENT-REF
    ContainedPackageElementRef                                             = 1342,
    /// CONTAINED-PACKAGE-ELEMENT-REFS
    ContainedPackageElementRefs                                            = 2397,
    /// CONTAINED-PDU-TRIGGERING-REF
    ContainedPduTriggeringRef                                              = 319,
    /// CONTAINED-PDU-TRIGGERING-REFS
    ContainedPduTriggeringRefs                                             = 1984,
    /// CONTAINED-PROCESS-REF
    ContainedProcessRef                                                    = 4130,
    /// CONTAINED-PROCESS-REFS
    ContainedProcessRefs                                                   = 79,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 385,
    /// CONTAINER-I-PDU-HEADER-BYTE-ORDER
    ContainerIPduHeaderByteOrder                                           = 580,
    /// CONTAINER-TIMEOUT
    ContainerTimeout                                                       = 4186,
    /// CONTAINER-TRIGGER
    ContainerTrigger                                                       = 537,
    /// CONTAINERS
    Containers                                                             = 2191,
    /// CONTENT-TYPE
    ContentType                                                            = 1719,
    /// CONTENT-URI
    ContentUri                                                             = 4782,
    /// CONTEXT-APPLICATION-RECORD-ELEMENT-REF
    ContextApplicationRecordElementRef                                     = 678,
    /// CONTEXT-ATOMIC-COMPONENT-REF
    ContextAtomicComponentRef                                              = 5089,
    /// CONTEXT-BSW-IMPLEMENTATION-REF
    ContextBswImplementationRef                                            = 35,
    /// CONTEXT-COMPONENT-PROTOTYPE-REF
    ContextComponentPrototypeRef                                           = 673,
    /// CONTEXT-COMPONENT-REF
    ContextComponentRef                                                    = 2637,
    /// CONTEXT-COMPONENT-REFS
    ContextComponentRefs                                                   = 4944,
    /// CONTEXT-COMPOSITION-REF
    ContextCompositionRef                                                  = 4630,
    /// CONTEXT-DATA-PROTOTYPE-IN-CS-REF
    ContextDataPrototypeInCsRef                                            = 1771,
    /// CONTEXT-DATA-PROTOTYPE-IN-SR-REF
    ContextDataPrototypeInSrRef                                            = 4085,
    /// CONTEXT-DATA-PROTOTYPE-REF
    ContextDataPrototypeRef                                                = 3231,
    /// CONTEXT-DATA-PROTOTYPE-REFS
    ContextDataPrototypeRefs                                               = 5207,
    /// CONTEXT-DATA-SOURCE
    ContextDataSource                                                      = 3733,
    /// CONTEXT-DATAS
    ContextDatas                                                           = 4797,
    /// CONTEXT-DESCRIPTION
    ContextDescription                                                     = 3863,
    /// CONTEXT-ELEMENT-REF
    ContextElementRef                                                      = 1134,
    /// CONTEXT-ELEMENT-REFS
    ContextElementRefs                                                     = 5650,
    /// CONTEXT-ID
    ContextId                                                              = 5337,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF
    ContextImplementationDataElementRef                                    = 2135,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS
    ContextImplementationDataElementRefs                                   = 3977,
    /// CONTEXT-LIMITATION-REF
    ContextLimitationRef                                                   = 4142,
    /// CONTEXT-LIMITATION-REFS
    ContextLimitationRefs                                                  = 3121,
    /// CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ContextModeDeclarationGroupPrototypeRef                                = 3411,
    /// CONTEXT-MODE-DECLARATION-GROUP-REF
    ContextModeDeclarationGroupRef                                         = 2467,
    /// CONTEXT-OPERATION-REF
    ContextOperationRef                                                    = 5055,
    /// CONTEXT-P-PORT-PROTOTYPE-REF
    ContextPPortPrototypeRef                                               = 452,
    /// CONTEXT-P-PORT-REF
    ContextPPortRef                                                        = 2708,
    /// CONTEXT-PORT-PROTOTYPE-REF
    ContextPortPrototypeRef                                                = 4806,
    /// CONTEXT-PORT-REF
    ContextPortRef                                                         = 2375,
    /// CONTEXT-R-PORT-PROTOTYPE-REF
    ContextRPortPrototypeRef                                               = 4688,
    /// CONTEXT-R-PORT-REF
    ContextRPortRef                                                        = 3495,
    /// CONTEXT-REF
    ContextRef                                                             = 472,
    /// CONTEXT-ROOT-COMPONENT-REF
    ContextRootComponentRef                                                = 1479,
    /// CONTEXT-ROOT-COMPOSITION-REF
    ContextRootCompositionRef                                              = 3628,
    /// CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF
    ContextRootSwClusterDesignComponentPrototypeRef                        = 5528,
    /// CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF
    ContextRootSwComponentPrototypeRef                                     = 4,
    /// CONTEXT-ROOT-SW-COMPOSITION-REF
    ContextRootSwCompositionRef                                            = 2971,
    /// CONTEXT-SW-COMPONENT-PROTOTYPE-REF
    ContextSwComponentPrototypeRef                                         = 884,
    /// CONTEXT-SW-COMPONENT-REF
    ContextSwComponentRef                                                  = 245,
    /// CONTEXT-SWC-PROTOTYPE-REF
    ContextSwcPrototypeRef                                                 = 3176,
    /// CONTEXT-SWC-PROTOTYPE-REFS
    ContextSwcPrototypeRefs                                                = 3926,
    /// CONTEXTS
    Contexts                                                               = 5511,
    /// CONTROL-CONSUMED-EVENT-GROUP-REF
    ControlConsumedEventGroupRef                                           = 1303,
    /// CONTROL-CONSUMED-EVENT-GROUP-REFS
    ControlConsumedEventGroupRefs                                          = 3684,
    /// CONTROL-ENABLE-MASK-BITS
    ControlEnableMaskBits                                                  = 1422,
    /// CONTROL-OPTION-RECORD-PRESENT
    ControlOptionRecordPresent                                             = 2382,
    /// CONTROL-PNC-REF
    ControlPncRef                                                          = 4758,
    /// CONTROL-PNC-REFS
    ControlPncRefs                                                         = 1866,
    /// CONTROL-PROVIDED-EVENT-GROUP-REF
    ControlProvidedEventGroupRef                                           = 1487,
    /// CONTROL-PROVIDED-EVENT-GROUP-REFS
    ControlProvidedEventGroupRefs                                          = 1120,
    /// CONTROLLED-DATA-ELEMENT-REF
    ControlledDataElementRef                                               = 959,
    /// CONTROLLED-DATA-ELEMENT-REFS
    ControlledDataElementRefs                                              = 1069,
    /// CONTROLLED-FID-REF
    ControlledFidRef                                                       = 2952,
    /// CONTROLLED-HW-ELEMENT-REF
    ControlledHwElementRef                                                 = 2541,
    /// CONTROLLER-REF
    ControllerRef                                                          = 3950,
    /// CORE-ID
    CoreId                                                                 = 4920,
    /// CORES
    Cores                                                                  = 2426,
    /// COUNT-PROFILE
    CountProfile                                                           = 2793,
    /// COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE
    CounterBasedFdcThresholdStorageValue                                   = 4462,
    /// COUNTER-DECREMENT-STEP-SIZE
    CounterDecrementStepSize                                               = 4469,
    /// COUNTER-FAILED-THRESHOLD
    CounterFailedThreshold                                                 = 3722,
    /// COUNTER-INCREMENT-STEP-SIZE
    CounterIncrementStepSize                                               = 4118,
    /// COUNTER-JUMP-DOWN
    CounterJumpDown                                                        = 272,
    /// COUNTER-JUMP-DOWN-VALUE
    CounterJumpDownValue                                                   = 3147,
    /// COUNTER-JUMP-UP
    CounterJumpUp                                                          = 1640,
    /// COUNTER-JUMP-UP-VALUE
    CounterJumpUpValue                                                     = 428,
    /// COUNTER-OFFSET
    CounterOffset                                                          = 264,
    /// COUNTER-PASSED-THRESHOLD
    CounterPassedThreshold                                                 = 2456,
    /// COUPLED-CLUSTER-REF
    CoupledClusterRef                                                      = 4916,
    /// COUPLED-CLUSTER-REFS
    CoupledClusterRefs                                                     = 2390,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 2910,
    /// COUPLING-PORT
    CouplingPort                                                           = 5335,
    /// COUPLING-PORT-CONNECTION
    CouplingPortConnection                                                 = 1742,
    /// COUPLING-PORT-CONNECTIONS
    CouplingPortConnections                                                = 814,
    /// COUPLING-PORT-DETAILS
    CouplingPortDetails                                                    = 2544,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 2491,
    /// COUPLING-PORT-RATE-POLICY
    CouplingPortRatePolicy                                                 = 1538,
    /// COUPLING-PORT-REF
    CouplingPortRef                                                        = 961,
    /// COUPLING-PORT-REF-CONDITIONAL
    CouplingPortRefConditional                                             = 63,
    /// COUPLING-PORT-ROLE
    CouplingPortRole                                                       = 5059,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 4132,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 3392,
    /// COUPLING-PORT-SPEED
    CouplingPortSpeed                                                      = 1682,
    /// COUPLING-PORT-STARTUP-ACTIVE-TIME
    CouplingPortStartupActiveTime                                          = 5799,
    /// COUPLING-PORT-STRUCTURAL-ELEMENTS
    CouplingPortStructuralElements                                         = 3091,
    /// COUPLING-PORT-SWITCHOFF-DELAY
    CouplingPortSwitchoffDelay                                             = 5011,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 1700,
    /// COUPLING-PORTS
    CouplingPorts                                                          = 5533,
    /// COUPLING-TYPE
    CouplingType                                                           = 2622,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 2393,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 2631,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 4170,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 2449,
    /// CP-SOFTWARE-CLUSTER-REF
    CpSoftwareClusterRef                                                   = 3794,
    /// CP-SOFTWARE-CLUSTER-REF-CONDITIONAL
    CpSoftwareClusterRefConditional                                        = 3751,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 1975,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-REF
    CpSoftwareClusterResourceRef                                           = 5794,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 4351,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 2068,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 3544,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 5264,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 5557,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 3385,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 5240,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 5586,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 424,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 619,
    /// CPP-TEMPLATE-ARGUMENT
    CppTemplateArgument                                                    = 1832,
    /// CPU-USAGE
    CpuUsage                                                               = 1357,
    /// CRC-CORRECTION-FIELD
    CrcCorrectionField                                                     = 3934,
    /// CRC-DOMAIN-NUMBER
    CrcDomainNumber                                                        = 4846,
    /// CRC-FLAGS
    CrcFlags                                                               = 1269,
    /// CRC-MESSAGE-LENGTH
    CrcMessageLength                                                       = 4901,
    /// CRC-OFFSET
    CrcOffset                                                              = 3891,
    /// CRC-PRECISE-ORIGIN-TIMESTAMP
    CrcPreciseOriginTimestamp                                              = 2458,
    /// CRC-SECURED
    CrcSecured                                                             = 1187,
    /// CRC-SEQUENCE-ID
    CrcSequenceId                                                          = 4158,
    /// CRC-SOURCE-PORT-IDENTITY
    CrcSourcePortIdentity                                                  = 1979,
    /// CRC-VALIDATED
    CrcValidated                                                           = 1548,
    /// CREATE-ECU-WAKEUP-SOURCE
    CreateEcuWakeupSource                                                  = 1323,
    /// CREATED-DATAS
    CreatedDatas                                                           = 4571,
    /// CRYPTO-ALG-ID
    CryptoAlgId                                                            = 5437,
    /// CRYPTO-ALGO-CONFIG
    CryptoAlgoConfig                                                       = 758,
    /// CRYPTO-ALGORITHM-STRING
    CryptoAlgorithmString                                                  = 4750,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 1205,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 666,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 5751,
    /// CRYPTO-CERTIFICATE-REF
    CryptoCertificateRef                                                   = 4340,
    /// CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING
    CryptoCertificateToCryptoKeySlotMapping                                = 5827,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 3194,
    /// CRYPTO-CERTIFICATES
    CryptoCertificates                                                     = 20,
    /// CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS
    CryptoDesignToCryptoDriverMappings                                     = 5749,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 1811,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING
    CryptoDriverToCryptoJobMapping                                         = 3086,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS
    CryptoDriverToCryptoJobMappings                                        = 2938,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 141,
    /// CRYPTO-JOB
    CryptoJob                                                              = 3778,
    /// CRYPTO-JOB-REF
    CryptoJobRef                                                           = 2094,
    /// CRYPTO-JOBS
    CryptoJobs                                                             = 2365,
    /// CRYPTO-KEY-DESCRIPTION
    CryptoKeyDescription                                                   = 1297,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 4625,
    /// CRYPTO-KEY-REF
    CryptoKeyRef                                                           = 2936,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 4298,
    /// CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE
    CryptoKeySlotContentAllowedUsage                                       = 5009,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 627,
    /// CRYPTO-KEY-SLOT-REF
    CryptoKeySlotRef                                                       = 518,
    /// CRYPTO-KEY-SLOT-REFS
    CryptoKeySlotRefs                                                      = 69,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 1388,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 723,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 3889,
    /// CRYPTO-NEED-REF
    CryptoNeedRef                                                          = 524,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 4291,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 2948,
    /// CRYPTO-OBJECT-TYPE
    CryptoObjectType                                                       = 1477,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 4499,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 2690,
    /// CRYPTO-PROVIDER-DOCUMENTATION-REF
    CryptoProviderDocumentationRef                                         = 4780,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 484,
    /// CRYPTO-PROVIDER-REF
    CryptoProviderRef                                                      = 4441,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 1409,
    /// CRYPTO-PROVIDERS
    CryptoProviders                                                        = 5839,
    /// CRYPTO-R-PORT-COM-SPEC
    CryptoRPortComSpec                                                     = 2899,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 5779,
    /// CRYPTO-SERVICE-CERTIFICATE-REF
    CryptoServiceCertificateRef                                            = 177,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 1871,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 3765,
    /// CRYPTO-SERVICE-KEY-REF
    CryptoServiceKeyRef                                                    = 794,
    /// CRYPTO-SERVICE-MAPPINGS
    CryptoServiceMappings                                                  = 1178,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 2843,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 5730,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 1310,
    /// CRYPTO-SERVICE-QUEUE-REF
    CryptoServiceQueueRef                                                  = 3839,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 4206,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 1445,
    /// CS-ERROR-REACTION
    CsErrorReaction                                                        = 4208,
    /// CSE-CODE
    CseCode                                                                = 5539,
    /// CSE-CODE-FACTOR
    CseCodeFactor                                                          = 2238,
    /// CURRENT-DATA-TYPE-REF
    CurrentDataTypeRef                                                     = 927,
    /// CURRENT-EXP
    CurrentExp                                                             = 2392,
    /// CURRENT-VALUE-REF
    CurrentValueRef                                                        = 497,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 4429,
    /// CUSTOM-DOCUMENT-ELEMENT-REF
    CustomDocumentElementRef                                               = 4382,
    /// CUSTOM-DOCUMENTATION-REF
    CustomDocumentationRef                                                 = 180,
    /// CUSTOM-SDG-DEF-REF
    CustomSdgDefRef                                                        = 2822,
    /// CUSTOM-SDG-DEF-REFS
    CustomSdgDefRefs                                                       = 3911,
    /// CUSTOM-SERVICE-CLASS-REF
    CustomServiceClassRef                                                  = 3253,
    /// CUSTOM-SERVICE-ID
    CustomServiceId                                                        = 5026,
    /// CUSTOM-SPECIFICATION-REF
    CustomSpecificationRef                                                 = 5837,
    /// CUSTOM-SPECIFICATION-REFS
    CustomSpecificationRefs                                                = 5914,
    /// CUSTOM-SUB-FUNCTION-NUMBER
    CustomSubFunctionNumber                                                = 3823,
    /// CUSTOM-TRIGGER
    CustomTrigger                                                          = 1175,
    /// CYCLE
    Cycle                                                                  = 4539,
    /// CYCLE-AUTOSTART
    CycleAutostart                                                         = 2844,
    /// CYCLE-COUNT-MAX
    CycleCountMax                                                          = 755,
    /// CYCLE-COUNTER
    CycleCounter                                                           = 3026,
    /// CYCLE-REPETITION
    CycleRepetition                                                        = 2761,
    /// CYCLE-STATUS-STORAGE
    CycleStatusStorage                                                     = 356,
    /// CYCLE-TIME-MAIN-FUNCTION
    CycleTimeMainFunction                                                  = 681,
    /// CYCLE-TIME-VALUE
    CycleTimeValue                                                         = 1407,
    /// CYCLIC-BACKUP-INTERVAL
    CyclicBackupInterval                                                   = 2729,
    /// CYCLIC-TIMING
    CyclicTiming                                                           = 2169,
    /// CYCLIC-WRITING-PERIOD
    CyclicWritingPeriod                                                    = 2514,
    /// DATA-COM-PROPS
    DataComProps                                                           = 3474,
    /// DATA-CONSISTENCY-POLICY
    DataConsistencyPolicy                                                  = 5583,
    /// DATA-CONSTR
    DataConstr                                                             = 2559,
    /// DATA-CONSTR-REF
    DataConstrRef                                                          = 5383,
    /// DATA-CONSTR-RULE
    DataConstrRule                                                         = 4421,
    /// DATA-CONSTR-RULES
    DataConstrRules                                                        = 5061,
    /// DATA-DUMP-ENTRY
    DataDumpEntry                                                          = 412,
    /// DATA-ELEMENT-IREF
    DataElementIref                                                        = 2431,
    /// DATA-ELEMENT-IREFS
    DataElementIrefs                                                       = 5697,
    /// DATA-ELEMENT-REF
    DataElementRef                                                         = 672,
    /// DATA-ELEMENT-REFS
    DataElementRefs                                                        = 2944,
    /// DATA-ELEMENTS
    DataElements                                                           = 2602,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 2881,
    /// DATA-FILTER
    DataFilter                                                             = 3154,
    /// DATA-FILTER-TYPE
    DataFilterType                                                         = 4416,
    /// DATA-FORMAT-TAILORING
    DataFormatTailoring                                                    = 3666,
    /// DATA-ID
    DataId                                                                 = 3545,
    /// DATA-ID-MODE
    DataIdMode                                                             = 5510,
    /// DATA-ID-NIBBLE-OFFSET
    DataIdNibbleOffset                                                     = 2765,
    /// DATA-IDENTIFIER-REF
    DataIdentifierRef                                                      = 31,
    /// DATA-IDENTIFIER-REFS
    DataIdentifierRefs                                                     = 3301,
    /// DATA-IDS
    DataIds                                                                = 536,
    /// DATA-IREF
    DataIref                                                               = 2958,
    /// DATA-LENGTH
    DataLength                                                             = 858,
    /// DATA-LENGTH-REQUEST
    DataLengthRequest                                                      = 1575,
    /// DATA-LENGTH-RESPONSE
    DataLengthResponse                                                     = 3781,
    /// DATA-LINK-LAYER-RULE
    DataLinkLayerRule                                                      = 920,
    /// DATA-MAPPINGS
    DataMappings                                                           = 3214,
    /// DATA-OFFSET
    DataOffset                                                             = 179,
    /// DATA-PDU-REF
    DataPduRef                                                             = 5762,
    /// DATA-PROTECTION-KIND
    DataProtectionKind                                                     = 3562,
    /// DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF
    DataProtototypeInPortInterfaceRef                                      = 1958,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 4172,
    /// DATA-PROTOTYPE-GROUP-IREF
    DataPrototypeGroupIref                                                 = 2852,
    /// DATA-PROTOTYPE-GROUP-IREFS
    DataPrototypeGroupIrefs                                                = 934,
    /// DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF
    DataPrototypeInClientServerInterfaceIref                               = 1417,
    /// DATA-PROTOTYPE-IN-METHOD-ARGUMENT-INSTANCE-REF-IREF
    DataPrototypeInMethodArgumentInstanceRefIref                           = 4025,
    /// DATA-PROTOTYPE-IN-PORT-INTERFACE-REF
    DataPrototypeInPortInterfaceRef                                        = 5334,
    /// DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF
    DataPrototypeInSenderReceiverInterfaceIref                             = 5547,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF
    DataPrototypeInServiceInterfaceIref                                    = 4297,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF
    DataPrototypeInServiceInterfaceRef                                     = 5581,
    /// DATA-PROTOTYPE-IREF
    DataPrototypeIref                                                      = 2930,
    /// DATA-PROTOTYPE-MAPPING
    DataPrototypeMapping                                                   = 5268,
    /// DATA-PROTOTYPE-REF
    DataPrototypeRef                                                       = 875,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPS
    DataPrototypeTransformationProps                                       = 399,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPSS
    DataPrototypeTransformationPropss                                      = 3668,
    /// DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF
    DataPrototypeWithApplicationDataTypeInSystemRef                        = 1212,
    /// DATA-PROTOTYPES
    DataPrototypes                                                         = 5232,
    /// DATA-PROVIDER
    DataProvider                                                           = 1689,
    /// DATA-READ-ACCESSS
    DataReadAccesss                                                        = 2443,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 4627,
    /// DATA-RECEIVE-POINT-BY-ARGUMENTS
    DataReceivePointByArguments                                            = 2427,
    /// DATA-RECEIVE-POINT-BY-VALUES
    DataReceivePointByValues                                               = 3830,
    /// DATA-RECEIVE-POINTS
    DataReceivePoints                                                      = 676,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 2315,
    /// DATA-REF
    DataRef                                                                = 5918,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 4200,
    /// DATA-SEND-POINTS
    DataSendPoints                                                         = 1440,
    /// DATA-TRANSFER-CLASS-REF
    DataTransferClassRef                                                   = 3087,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 3976,
    /// DATA-TRANSFORMATION-KIND
    DataTransformationKind                                                 = 4048,
    /// DATA-TRANSFORMATION-REF
    DataTransformationRef                                                  = 5500,
    /// DATA-TRANSFORMATION-REF-CONDITIONAL
    DataTransformationRefConditional                                       = 3372,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 838,
    /// DATA-TRANSFORMATIONS
    DataTransformations                                                    = 1004,
    /// DATA-TYPE-FOR-SERIALIZATION-REF
    DataTypeForSerializationRef                                            = 4274,
    /// DATA-TYPE-FOR-SERIALIZATION-REFS
    DataTypeForSerializationRefs                                           = 4585,
    /// DATA-TYPE-MAP
    DataTypeMap                                                            = 3780,
    /// DATA-TYPE-MAPPING
    DataTypeMapping                                                        = 92,
    /// DATA-TYPE-MAPPING-REF
    DataTypeMappingRef                                                     = 3128,
    /// DATA-TYPE-MAPPING-REFS
    DataTypeMappingRefs                                                    = 3789,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 5577,
    /// DATA-TYPE-MAPPING-SET-REF
    DataTypeMappingSetRef                                                  = 3676,
    /// DATA-TYPE-MAPPING-SET-REFS
    DataTypeMappingSetRefs                                                 = 3769,
    /// DATA-TYPE-MAPS
    DataTypeMaps                                                           = 1339,
    /// DATA-TYPE-POLICY
    DataTypePolicy                                                         = 2373,
    /// DATA-TYPE-REF
    DataTypeRef                                                            = 2515,
    /// DATA-TYPE-REFS
    DataTypeRefs                                                           = 5056,
    /// DATA-UPDATE-PERIOD
    DataUpdatePeriod                                                       = 5233,
    /// DATA-WRITE-ACCESSS
    DataWriteAccesss                                                       = 1048,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 4330,
    /// DATE
    Date                                                                   = 3681,
    /// DCM-I-PDU
    DcmIPdu                                                                = 579,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 616,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 3840,
    /// DDS-EVENT-QOS-PROPS
    DdsEventQosProps                                                       = 4235,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 3501,
    /// DDS-FIELD-QOS-PROPS
    DdsFieldQosProps                                                       = 2273,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 390,
    /// DDS-METHOD-QOS-PROPS
    DdsMethodQosProps                                                      = 651,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 4550,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 4407,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 4007,
    /// DDS-RPC-SERVICE-REF
    DdsRpcServiceRef                                                       = 4734,
    /// DDS-RPC-SERVICES
    DdsRpcServices                                                         = 1533,
    /// DDS-RULE
    DdsRule                                                                = 4453,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 2180,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 5310,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 1720,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 4392,
    /// DDS-SERVICE-VERSION
    DdsServiceVersion                                                      = 30,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 856,
    /// DEADLINE-MAX
    DeadlineMax                                                            = 1740,
    /// DEADLINE-MIN
    DeadlineMin                                                            = 1384,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 2105,
    /// DEADLINE-SUPERVISIONS
    DeadlineSupervisions                                                   = 2864,
    /// DEBOUNCE-ALGORITHM
    DebounceAlgorithm                                                      = 1053,
    /// DEBOUNCE-ALGORITHM-PROPSS
    DebounceAlgorithmPropss                                                = 5093,
    /// DEBOUNCE-ALGORITHM-REF
    DebounceAlgorithmRef                                                   = 2871,
    /// DEBOUNCE-BEHAVIOR
    DebounceBehavior                                                       = 4031,
    /// DEBOUNCE-COUNTER-STORAGE
    DebounceCounterStorage                                                 = 5588,
    /// DEBOUNCE-TIME
    DebounceTime                                                           = 3215,
    /// DEBUG-INFOS
    DebugInfos                                                             = 227,
    /// DECODING-CORRECTION
    DecodingCorrection                                                     = 2272,
    /// DECOMPOSITIONS
    Decompositions                                                         = 4265,
    /// DEF
    Def                                                                    = 3409,
    /// DEF-CALPRM-SET
    DefCalprmSet                                                           = 1788,
    /// DEF-ITEM
    DefItem                                                                = 3504,
    /// DEF-LIST
    DefList                                                                = 5601,
    /// DEFAULT-ACTION
    DefaultAction                                                          = 1156,
    /// DEFAULT-APPLICATION-TIMEOUT
    DefaultApplicationTimeout                                              = 1587,
    /// DEFAULT-ENDIANNESS
    DefaultEndianness                                                      = 2011,
    /// DEFAULT-GATEWAY
    DefaultGateway                                                         = 4763,
    /// DEFAULT-LC-STATE-REF
    DefaultLcStateRef                                                      = 3816,
    /// DEFAULT-LEASE-TIME
    DefaultLeaseTime                                                       = 1002,
    /// DEFAULT-LOG-THRESHOLD
    DefaultLogThreshold                                                    = 4271,
    /// DEFAULT-MODE-REF
    DefaultModeRef                                                         = 5673,
    /// DEFAULT-PATTERN
    DefaultPattern                                                         = 2477,
    /// DEFAULT-PERIOD-BEGIN
    DefaultPeriodBegin                                                     = 4369,
    /// DEFAULT-PERIOD-END
    DefaultPeriodEnd                                                       = 2705,
    /// DEFAULT-PRIORITY
    DefaultPriority                                                        = 726,
    /// DEFAULT-REPORTING-MODE
    DefaultReportingMode                                                   = 1784,
    /// DEFAULT-ROUTER
    DefaultRouter                                                          = 3435,
    /// DEFAULT-TRACE-STATE
    DefaultTraceState                                                      = 944,
    /// DEFAULT-VALUE
    DefaultValue                                                           = 5530,
    /// DEFAULT-VALUE-ELEMENT
    DefaultValueElement                                                    = 5161,
    /// DEFAULT-VALUE-ELEMENTS
    DefaultValueElements                                                   = 5690,
    /// DEFAULT-VALUE-HANDLING
    DefaultValueHandling                                                   = 1182,
    /// DEFAULT-VLAN-REF
    DefaultVlanRef                                                         = 5000,
    /// DEFERRING-FID-REF
    DeferringFidRef                                                        = 2407,
    /// DEFERRING-FID-REFS
    DeferringFidRefs                                                       = 747,
    /// DEFINITION-REF
    DefinitionRef                                                          = 3580,
    /// DELAY
    Delay                                                                  = 2333,
    /// DELAY-COMPENSATION-A
    DelayCompensationA                                                     = 4358,
    /// DELAY-COMPENSATION-B
    DelayCompensationB                                                     = 4114,
    /// DELEGATED-PORT-ANNOTATION
    DelegatedPortAnnotation                                                = 2489,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 4636,
    /// DELIVERY-ARTIFACTS
    DeliveryArtifacts                                                      = 3098,
    /// DELTA-PACKAGE-APPLICABLE-VERSION
    DeltaPackageApplicableVersion                                          = 2811,
    /// DENOMINATOR-CONDITION
    DenominatorCondition                                                   = 2027,
    /// DENOMINATOR-GROUP
    DenominatorGroup                                                       = 3104,
    /// DEPENDENCIES
    Dependencies                                                           = 5398,
    /// DEPENDENCY
    Dependency                                                             = 4861,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 3036,
    /// DEPENDENT-RESOURCES
    DependentResources                                                     = 384,
    /// DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF
    DependentSoftwareClusterDesignRef                                      = 2653,
    /// DEPENDENT-SOFTWARE-CLUSTER-REF
    DependentSoftwareClusterRef                                            = 2437,
    /// DEPENDS-ON
    DependsOn                                                              = 1722,
    /// DEPENDS-ON-REF
    DependsOnRef                                                           = 3502,
    /// DEPENDS-ON-REFS
    DependsOnRefs                                                          = 5367,
    /// DEPENDS-ONS
    DependsOns                                                             = 2670,
    /// DEPLOYMENT-REF
    DeploymentRef                                                          = 1718,
    /// DEPLOYMENT-URIS
    DeploymentUris                                                         = 5917,
    /// DERIVATION
    Derivation                                                             = 5825,
    /// DERIVED-CERTIFICATE-ACCEPTED
    DerivedCertificateAccepted                                             = 1814,
    /// DERIVED-FROM-BLUEPRINT-REF
    DerivedFromBlueprintRef                                                = 4010,
    /// DERIVED-FROM-BLUEPRINT-REFS
    DerivedFromBlueprintRefs                                               = 684,
    /// DERIVED-INTERFACE-REF
    DerivedInterfaceRef                                                    = 3806,
    /// DERIVED-OBJECT-REF
    DerivedObjectRef                                                       = 3355,
    /// DERIVED-PORT-INTERFACE-REF
    DerivedPortInterfaceRef                                                = 3960,
    /// DERIVED-PORT-PROTOTYPE-REF
    DerivedPortPrototypeRef                                                = 3292,
    /// DESC
    Desc                                                                   = 1194,
    /// DESCRIPTION
    Description                                                            = 662,
    /// DESIGN-REF
    DesignRef                                                              = 2719,
    /// DESIGN-REFS
    DesignRefs                                                             = 2652,
    /// DEST-META-CLASS
    DestMetaClass                                                          = 2430,
    /// DEST-SDG-REF
    DestSdgRef                                                             = 2313,
    /// DESTINATION-BASE-ID
    DestinationBaseId                                                      = 1329,
    /// DESTINATION-CONTEXT
    DestinationContext                                                     = 3558,
    /// DESTINATION-IP-ADDRESS
    DestinationIpAddress                                                   = 3285,
    /// DESTINATION-MAC-ADDRESS
    DestinationMacAddress                                                  = 2189,
    /// DESTINATION-MAC-ADDRESS-MASK
    DestinationMacAddressMask                                              = 922,
    /// DESTINATION-MAX-ADDRESS
    DestinationMaxAddress                                                  = 5694,
    /// DESTINATION-MIN-ADDRESS
    DestinationMinAddress                                                  = 237,
    /// DESTINATION-NETWORK-MASK
    DestinationNetworkMask                                                 = 2625,
    /// DESTINATION-PHYSICAL-ADDRESS
    DestinationPhysicalAddress                                             = 1403,
    /// DESTINATION-REF
    DestinationRef                                                         = 1245,
    /// DESTINATION-REFS
    DestinationRefs                                                        = 201,
    /// DESTINATION-TYPE
    DestinationType                                                        = 1441,
    /// DESTINATION-URI-DEFS
    DestinationUriDefs                                                     = 827,
    /// DESTINATION-URI-NESTING-CONTRACT
    DestinationUriNestingContract                                          = 2665,
    /// DESTINATION-URI-POLICY
    DestinationUriPolicy                                                   = 5479,
    /// DESTINATION-URI-REF
    DestinationUriRef                                                      = 2494,
    /// DESTINATION-URI-REFS
    DestinationUriRefs                                                     = 5804,
    /// DETECT-NIT-ERROR
    DetectNitError                                                         = 1746,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 4480,
    /// DETERMINISTIC-CLIENT-REF
    DeterministicClientRef                                                 = 5347,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 4070,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDSS
    DeterministicClientResourceNeedss                                      = 853,
    /// DETERMINISTIC-SYNC-M-OUT-OF-N
    DeterministicSyncMOutOfN                                               = 5126,
    /// DETERMINISTIC-SYNC-MASTER
    DeterministicSyncMaster                                                = 1899,
    /// DETERMINISTIC-SYNC-MASTER-REF
    DeterministicSyncMasterRef                                             = 494,
    /// DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING
    DeterministicSyncMasterToTimeBaseConsumerMapping                       = 3255,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 2549,
    /// DEVELOPMENT-VALUE
    DevelopmentValue                                                       = 2436,
    /// DHCP-ADDRESS-ASSIGNMENT
    DhcpAddressAssignment                                                  = 5853,
    /// DHCP-PROPS
    DhcpProps                                                              = 3706,
    /// DHCP-SERVER-CONFIGURATION
    DhcpServerConfiguration                                                = 4278,
    /// DIAG-ARG-INTEGRITY
    DiagArgIntegrity                                                       = 1392,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 5275,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 1739,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 4836,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 5162,
    /// DIAG-FAULT-DETECTION-COUNTER-PORT-REF
    DiagFaultDetectionCounterPortRef                                       = 2831,
    /// DIAG-PDU-TYPE
    DiagPduType                                                            = 1271,
    /// DIAG-REQUIREMENT
    DiagRequirement                                                        = 3458,
    /// DIAG-ROUTINE-TYPE
    DiagRoutineType                                                        = 3338,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 4071,
    /// DIAGNOSTIC-ADDRESS
    DiagnosticAddress                                                      = 4484,
    /// DIAGNOSTIC-ADDRESSS
    DiagnosticAddresss                                                     = 2785,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 2505,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 780,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 528,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 4099,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 3252,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 3022,
    /// DIAGNOSTIC-AUTHENTICATION-REF
    DiagnosticAuthenticationRef                                            = 3802,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 2440,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 2501,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 1823,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 5452,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 2468,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 2978,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 2757,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 5680,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 628,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 5445,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 5320,
    /// DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL
    DiagnosticComControlSpecificChannel                                    = 435,
    /// DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL
    DiagnosticComControlSubNodeChannel                                     = 5159,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF
    DiagnosticCommonElementRef                                             = 310,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL
    DiagnosticCommonElementRefConditional                                  = 2222,
    /// DIAGNOSTIC-COMMON-PROPS-CONDITIONAL
    DiagnosticCommonPropsConditional                                       = 2679,
    /// DIAGNOSTIC-COMMON-PROPS-VARIANTS
    DiagnosticCommonPropsVariants                                          = 4812,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 1511,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 3760,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 3876,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 902,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 2069,
    /// DIAGNOSTIC-CONNECTION-REF
    DiagnosticConnectionRef                                                = 1154,
    /// DIAGNOSTIC-CONNECTION-REF-CONDITIONAL
    DiagnosticConnectionRefConditional                                     = 4368,
    /// DIAGNOSTIC-CONNECTIONS
    DiagnosticConnections                                                  = 4657,
    /// DIAGNOSTIC-CONTRIBUTION-REF
    DiagnosticContributionRef                                              = 1371,
    /// DIAGNOSTIC-CONTRIBUTION-REFS
    DiagnosticContributionRefs                                             = 67,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 2148,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 3754,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 3743,
    /// DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT
    DiagnosticControlEnableMaskBit                                         = 2802,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 5280,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 489,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 851,
    /// DIAGNOSTIC-DATA-CHANGE-TRIGGER
    DiagnosticDataChangeTrigger                                            = 2600,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 4935,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 5279,
    /// DIAGNOSTIC-DATA-ELEMENT-REF
    DiagnosticDataElementRef                                               = 4370,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 3534,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 3943,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 3350,
    /// DIAGNOSTIC-DATA-IDENTIFIER-REF
    DiagnosticDataIdentifierRef                                            = 717,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 299,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF
    DiagnosticDataIdentifierSetRef                                         = 2168,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL
    DiagnosticDataIdentifierSetRefConditional                              = 5219,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 2921,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 5024,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 879,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 4534,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 5229,
    /// DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT
    DiagnosticDebounceBehaviorEnumValueVariationPoint                      = 1578,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 1044,
    /// DIAGNOSTIC-DEPLOYMENT-PROPS-REF
    DiagnosticDeploymentPropsRef                                           = 2460,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 3993,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE
    DiagnosticDoIpEntityIdentificationInterface                            = 4308,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 2110,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 5274,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 5470,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 51,
    /// DIAGNOSTIC-DTC-CHANGE-TRIGGER
    DiagnosticDtcChangeTrigger                                             = 1060,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 1482,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 5529,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 3075,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 3045,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 5456,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 5097,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 1758,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 3001,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 3206,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 3587,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 1994,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 4606,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF
    DiagnosticEnableConditionRef                                           = 1756,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL
    DiagnosticEnableConditionRefConditional                                = 3807,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 2334,
    /// DIAGNOSTIC-ENV-CONDITION-FORMULA
    DiagnosticEnvConditionFormula                                          = 5497,
    /// DIAGNOSTIC-ENV-DATA-CONDITION
    DiagnosticEnvDataCondition                                             = 1144,
    /// DIAGNOSTIC-ENV-DATA-ELEMENT-CONDITION
    DiagnosticEnvDataElementCondition                                      = 2645,
    /// DIAGNOSTIC-ENV-MODE-CONDITION
    DiagnosticEnvModeCondition                                             = 2597,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 5741,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 1083,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 3032,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 1398,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 4189,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 2975,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 3397,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 1,
    /// DIAGNOSTIC-EVENT-REF
    DiagnosticEventRef                                                     = 3718,
    /// DIAGNOSTIC-EVENT-REF-CONDITIONAL
    DiagnosticEventRefConditional                                          = 3997,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 5140,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 813,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 4749,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 3616,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 2919,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 2589,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 3746,
    /// DIAGNOSTIC-EVENT-WINDOW
    DiagnosticEventWindow                                                  = 4848,
    /// DIAGNOSTIC-EVENTS
    DiagnosticEvents                                                       = 5015,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 2018,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF
    DiagnosticExtendedDataRecordRef                                        = 4463,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL
    DiagnosticExtendedDataRecordRefConditional                             = 421,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION
    DiagnosticExternalAuthenticationIdentification                         = 446,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 4078,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 49,
    /// DIAGNOSTIC-EXTRACT-REF
    DiagnosticExtractRef                                                   = 1442,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 308,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 3636,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 2138,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 5050,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 2112,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 2807,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 4329,
    /// DIAGNOSTIC-FREEZE-FRAME-REF
    DiagnosticFreezeFrameRef                                               = 5916,
    /// DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL
    DiagnosticFreezeFrameRefConditional                                    = 4035,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 3599,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 4990,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 3472,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 3066,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 5522,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 5884,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 1944,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 1600,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 5012,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 1743,
    /// DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT
    DiagnosticIndicatorTypeEnumValueVariationPoint                         = 5886,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 1963,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 1012,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 4223,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 3747,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 350,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 5099,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 2680,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 3129,
    /// DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER
    DiagnosticIumprGroupIdentifier                                         = 4113,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 4393,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 4939,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 956,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 2380,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 1486,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 2744,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 3203,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 4532,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 4303,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 713,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 5660,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 938,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 2739,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 2139,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 198,
    /// DIAGNOSTIC-MEMORY-REF
    DiagnosticMemoryRef                                                    = 1599,
    /// DIAGNOSTIC-MESSAGE-TYPE
    DiagnosticMessageType                                                  = 4840,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 4884,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 1498,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 3707,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 4859,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 3230,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 4512,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF
    DiagnosticOperationCycleRef                                            = 2979,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL
    DiagnosticOperationCycleRefConditional                                 = 1761,
    /// DIAGNOSTIC-PARAMETER
    DiagnosticParameter                                                    = 5312,
    /// DIAGNOSTIC-PARAMETER-ELEMENT
    DiagnosticParameterElement                                             = 3492,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 1032,
    /// DIAGNOSTIC-PARAMETER-REF
    DiagnosticParameterRef                                                 = 5263,
    /// DIAGNOSTIC-PERIODIC-RATE
    DiagnosticPeriodicRate                                                 = 2973,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 3050,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 253,
    /// DIAGNOSTIC-PROPS
    DiagnosticProps                                                        = 1877,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 5355,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 839,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 2395,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 5695,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 4062,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 1892,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 1705,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 2507,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 1397,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 1400,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 530,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 5720,
    /// DIAGNOSTIC-RELEVANCE
    DiagnosticRelevance                                                    = 1937,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 4286,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 2667,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 3766,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 1584,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 686,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 4049,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 3071,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 5817,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 3143,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 546,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 3212,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 3890,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE
    DiagnosticRequestFileTransferInterface                                 = 4419,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 2165,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 2734,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 2753,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 2752,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 3797,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 4051,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 2177,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 3903,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 167,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 3779,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 4786,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 1065,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 3282,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 415,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 3132,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 894,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 610,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 2312,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 5847,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 1948,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 4854,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 137,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 5039,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 1670,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 4677,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 3565,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 1095,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 2858,
    /// DIAGNOSTIC-SERVICE-INSTANCE-REF
    DiagnosticServiceInstanceRef                                           = 3124,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 3484,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 4036,
    /// DIAGNOSTIC-SERVICE-TABLE-REF
    DiagnosticServiceTableRef                                              = 3731,
    /// DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL
    DiagnosticServiceTableRefConditional                                   = 2199,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 4660,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 93,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 2833,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 571,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 795,
    /// DIAGNOSTIC-SESSION-REF
    DiagnosticSessionRef                                                   = 3247,
    /// DIAGNOSTIC-SESSION-REFS
    DiagnosticSessionRefs                                                  = 916,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 5223,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE
    DiagnosticSovdAuthorizationInterface                                   = 2953,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING
    DiagnosticSovdAuthorizationPortMapping                                 = 2967,
    /// DIAGNOSTIC-SOVD-LOCK
    DiagnosticSovdLock                                                     = 648,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE
    DiagnosticSovdProximityChallengeInterface                              = 753,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING
    DiagnosticSovdProximityChallengePortMapping                            = 2685,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 5186,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 4634,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 1436,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 1694,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF
    DiagnosticStorageConditionRef                                          = 3961,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL
    DiagnosticStorageConditionRefConditional                               = 4724,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 5958,
    /// DIAGNOSTIC-TEST-RESULT-REF
    DiagnosticTestResultRef                                                = 4063,
    /// DIAGNOSTIC-TEST-RESULT-REFS
    DiagnosticTestResultRefs                                               = 4767,
    /// DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT
    DiagnosticTestResultUpdateEnumValueVariationPoint                      = 2532,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 5846,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 1027,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 1869,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 4819,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 3323,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 4505,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 2868,
    /// DIAGNOSTIC-TROUBLE-CODE-REF
    DiagnosticTroubleCodeRef                                               = 5774,
    /// DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL
    DiagnosticTroubleCodeRefConditional                                    = 4491,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 1159,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 2340,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 1286,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 55,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 2415,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 1305,
    /// DIAGNOSTIC-VALUE-ACCESS
    DiagnosticValueAccess                                                  = 2179,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 2810,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 4097,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 86,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 4642,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 4614,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 862,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 76,
    /// DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT
    DiagnosticWwhObdDtcClassEnumValueVariationPoint                        = 1629,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 506,
    /// DID-NUMBER
    DidNumber                                                              = 4826,
    /// DID-SIZE
    DidSize                                                                = 1484,
    /// DIFFERENTIATED-SERVICE-CODE-POINT
    DifferentiatedServiceCodePoint                                         = 566,
    /// DIFFERENTIATED-SERVICE-FIELD
    DifferentiatedServiceField                                             = 426,
    /// DIRECT-PDU-REF
    DirectPduRef                                                           = 3566,
    /// DIRECT-SUCCESSOR-REF
    DirectSuccessorRef                                                     = 5850,
    /// DIRECT-SUCCESSOR-REFS
    DirectSuccessorRefs                                                    = 4138,
    /// DIRECT-TP-SDU-REF
    DirectTpSduRef                                                         = 4850,
    /// DIRECTION
    Direction                                                              = 5189,
    /// DISABLE-END-TO-END-CHECK
    DisableEndToEndCheck                                                   = 2780,
    /// DISABLE-END-TO-END-STATE-MACHINE
    DisableEndToEndStateMachine                                            = 1969,
    /// DISABLED-IN-MODE-IREF
    DisabledInModeIref                                                     = 2355,
    /// DISABLED-IN-MODE-IREFS
    DisabledInModeIrefs                                                    = 3786,
    /// DISABLED-MODE-IREF
    DisabledModeIref                                                       = 1160,
    /// DISABLED-MODE-IREFS
    DisabledModeIrefs                                                      = 1652,
    /// DISCOVERY-PROTECTION-KIND
    DiscoveryProtectionKind                                                = 1520,
    /// DISCOVERY-TECHNOLOGY
    DiscoveryTechnology                                                    = 5277,
    /// DISCOVERY-TYPE
    DiscoveryType                                                          = 1764,
    /// DISPLAY-FORMAT
    DisplayFormat                                                          = 861,
    /// DISPLAY-IDENTIFIER
    DisplayIdentifier                                                      = 5480,
    /// DISPLAY-NAME
    DisplayName                                                            = 935,
    /// DISPLAY-PRESENTATION
    DisplayPresentation                                                    = 1745,
    /// DISTINGUISHED-PARTITIONS
    DistinguishedPartitions                                                = 1296,
    /// DLT-APPLICATION
    DltApplication                                                         = 2618,
    /// DLT-APPLICATION-REF
    DltApplicationRef                                                      = 1289,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 2039,
    /// DLT-ARGUMENT
    DltArgument                                                            = 5038,
    /// DLT-ARGUMENT-ENTRYS
    DltArgumentEntrys                                                      = 2009,
    /// DLT-ARGUMENTS
    DltArguments                                                           = 324,
    /// DLT-CONFIG
    DltConfig                                                              = 1232,
    /// DLT-CONTEXT
    DltContext                                                             = 976,
    /// DLT-CONTEXT-REF
    DltContextRef                                                          = 5226,
    /// DLT-CONTEXT-REF-CONDITIONAL
    DltContextRefConditional                                               = 3811,
    /// DLT-ECU
    DltEcu                                                                 = 1255,
    /// DLT-ECU-ID
    DltEcuId                                                               = 5692,
    /// DLT-ECU-REF
    DltEcuRef                                                              = 1038,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 2841,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 2766,
    /// DLT-LOG-CHANNEL-DESIGN-REF
    DltLogChannelDesignRef                                                 = 3315,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 1385,
    /// DLT-LOG-CHANNEL-REF
    DltLogChannelRef                                                       = 1780,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 4395,
    /// DLT-LOG-CHANNELS
    DltLogChannels                                                         = 5124,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 4019,
    /// DLT-LOG-SINK-REF
    DltLogSinkRef                                                          = 4815,
    /// DLT-LOG-SINK-REFS
    DltLogSinkRefs                                                         = 4117,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 348,
    /// DLT-MESSAGE
    DltMessage                                                             = 671,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 5131,
    /// DLT-MESSAGE-REF
    DltMessageRef                                                          = 2309,
    /// DLT-MESSAGE-REF-CONDITIONAL
    DltMessageRefConditional                                               = 2285,
    /// DLT-MESSAGE-REFS
    DltMessageRefs                                                         = 2781,
    /// DLT-MESSAGES
    DltMessages                                                            = 5343,
    /// DLT-SESSION-ID
    DltSessionId                                                           = 5543,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 4005,
    /// DNS-SERVER-ADDRESS
    DnsServerAddress                                                       = 609,
    /// DNS-SERVER-ADDRESSES
    DnsServerAddresses                                                     = 3858,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 3814,
    /// DO-IP-CONFIG
    DoIpConfig                                                             = 964,
    /// DO-IP-ENTITY
    DoIpEntity                                                             = 1514,
    /// DO-IP-ENTITY-ROLE
    DoIpEntityRole                                                         = 1692,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 3909,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 5740,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 1497,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 3952,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 2284,
    /// DO-IP-LOGIC-ADDRESS-PROPS
    DoIpLogicAddressProps                                                  = 5532,
    /// DO-IP-LOGIC-ADDRESSS
    DoIpLogicAddresss                                                      = 4687,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 2077,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 3182,
    /// DO-IP-NETWORK-CONFIGURATION
    DoIpNetworkConfiguration                                               = 5085,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 5821,
    /// DO-IP-REQUEST-CONFIGURATION
    DoIpRequestConfiguration                                               = 2568,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 3720,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 2605,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 5669,
    /// DO-IP-ROUTING-ACTIVATIONS
    DoIpRoutingActivations                                                 = 4188,
    /// DO-IP-RULE
    DoIpRule                                                               = 951,
    /// DO-IP-SOURCE-ADDRESS-REF
    DoIpSourceAddressRef                                                   = 3531,
    /// DO-IP-TARGET-ADDRESS-REF
    DoIpTargetAddressRef                                                   = 3795,
    /// DO-IP-TARGET-ADDRESS-REFS
    DoIpTargetAddressRefs                                                  = 2949,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REF
    DoIpTesterRoutingActivationRef                                         = 2523,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REFS
    DoIpTesterRoutingActivationRefs                                        = 5552,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 3354,
    /// DO-IP-TP-CONNECTION
    DoIpTpConnection                                                       = 5648,
    /// DO-NOT-FRAGMENT
    DoNotFragment                                                          = 4281,
    /// DOC-REVISION
    DocRevision                                                            = 340,
    /// DOC-REVISIONS
    DocRevisions                                                           = 3748,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 4384,
    /// DOCUMENT-ELEMENT-SCOPES
    DocumentElementScopes                                                  = 5493,
    /// DOCUMENTATION
    Documentation                                                          = 5644,
    /// DOCUMENTATION-CONTENT
    DocumentationContent                                                   = 621,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 5570,
    /// DOIP-CHANNEL-COLLECTION-REF
    DoipChannelCollectionRef                                               = 2327,
    /// DOIP-CONNECTION-REF
    DoipConnectionRef                                                      = 749,
    /// DOIP-CONNECTION-REFS
    DoipConnectionRefs                                                     = 2526,
    /// DOIP-INTERFACES
    DoipInterfaces                                                         = 896,
    /// DOMAIN
    Domain                                                                 = 2200,
    /// DOMAIN-ID
    DomainId                                                               = 4613,
    /// DOMAIN-IDS
    DomainIds                                                              = 5866,
    /// DPD-ACTION
    DpdAction                                                              = 2465,
    /// DPD-DELAY
    DpdDelay                                                               = 5905,
    /// DPG-DOES-NOT-REQUIRE-COHERENCYS
    DpgDoesNotRequireCoherencys                                            = 1702,
    /// DPG-REQUIRES-COHERENCYS
    DpgRequiresCoherencys                                                  = 4024,
    /// DRIVER-NOTIFICATIONS
    DriverNotifications                                                    = 1173,
    /// DRIVER-PRIMITIVE-REF
    DriverPrimitiveRef                                                     = 2774,
    /// DROP-NOT-REQUESTED-NAD
    DropNotRequestedNad                                                    = 193,
    /// DTC-FORMAT-TYPE
    DtcFormatType                                                          = 5753,
    /// DTC-KIND
    DtcKind                                                                = 2096,
    /// DTC-NUMBER
    DtcNumber                                                              = 372,
    /// DTC-PROPS-REF
    DtcPropsRef                                                            = 5611,
    /// DTC-SETTING-CLASS-REF
    DtcSettingClassRef                                                     = 741,
    /// DTC-SETTING-PARAMETER
    DtcSettingParameter                                                    = 3984,
    /// DTC-STATUS-AVAILABILITY-MASK
    DtcStatusAvailabilityMask                                              = 4612,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 3721,
    /// DTC-STATUS-MASK
    DtcStatusMask                                                          = 5555,
    /// DTCS
    Dtcs                                                                   = 4754,
    /// DYNAMIC-ACTION-REF
    DynamicActionRef                                                       = 4966,
    /// DYNAMIC-ACTION-REFS
    DynamicActionRefs                                                      = 2824,
    /// DYNAMIC-ARRAY-SIZE-PROFILE
    DynamicArraySizeProfile                                                = 1778,
    /// DYNAMIC-BS
    DynamicBs                                                              = 207,
    /// DYNAMIC-LENGTH
    DynamicLength                                                          = 2149,
    /// DYNAMIC-PART
    DynamicPart                                                            = 2305,
    /// DYNAMIC-PART-ALTERNATIVE
    DynamicPartAlternative                                                 = 2350,
    /// DYNAMIC-PART-ALTERNATIVES
    DynamicPartAlternatives                                                = 2416,
    /// DYNAMIC-PARTS
    DynamicParts                                                           = 1688,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REF
    DynamicPncMappingPduGroupRef                                           = 1712,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS
    DynamicPncMappingPduGroupRefs                                          = 5802,
    /// DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED
    DynamicPncToChannelMappingEnabled                                      = 3034,
    /// DYNAMIC-RUNTIME-LENGTH-HANDLING
    DynamicRuntimeLengthHandling                                           = 5340,
    /// DYNAMIC-SLOT-IDLE-PHASE
    DynamicSlotIdlePhase                                                   = 2687,
    /// DYNAMICALLY-ASSIGNED
    DynamicallyAssigned                                                    = 301,
    /// DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF
    DynamicallyDefineDataIdentifierClassRef                                = 1875,
    /// E
    E                                                                      = 4248,
    /// E-2-E-EVENT-PROTECTION-PROPSS
    E2EEventProtectionPropss                                               = 459,
    /// E-2-E-METHOD-PROTECTION-PROPSS
    E2EMethodProtectionPropss                                              = 5910,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 5058,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS-REF
    E2EProfileCompatibilityPropsRef                                        = 1195,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 2062,
    /// E-2-E-PROFILE-CONFIGURATION-REF
    E2EProfileConfigurationRef                                             = 4137,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 5655,
    /// E-2-E-PROFILE-CONFIGURATIONS
    E2EProfileConfigurations                                               = 1055,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 734,
    /// ECU-COMM-PORT-INSTANCES
    EcuCommPortInstances                                                   = 327,
    /// ECU-CONFIGURATION-REF
    EcuConfigurationRef                                                    = 2733,
    /// ECU-EXTRACT-REF
    EcuExtractRef                                                          = 3809,
    /// ECU-EXTRACT-REFERENCE-IREF
    EcuExtractReferenceIref                                                = 2123,
    /// ECU-EXTRACT-VERSION
    EcuExtractVersion                                                      = 1037,
    /// ECU-ID
    EcuId                                                                  = 4372,
    /// ECU-INSTANCE
    EcuInstance                                                            = 5814,
    /// ECU-INSTANCE-PROPS
    EcuInstanceProps                                                       = 3849,
    /// ECU-INSTANCE-PROPSS
    EcuInstancePropss                                                      = 603,
    /// ECU-INSTANCE-REF
    EcuInstanceRef                                                         = 2775,
    /// ECU-INSTANCE-REF-CONDITIONAL
    EcuInstanceRefConditional                                              = 2210,
    /// ECU-INSTANCE-REFS
    EcuInstanceRefs                                                        = 766,
    /// ECU-INSTANCES
    EcuInstances                                                           = 3944,
    /// ECU-MAPPING
    EcuMapping                                                             = 1383,
    /// ECU-PARTITION
    EcuPartition                                                           = 1921,
    /// ECU-PARTITION-REF
    EcuPartitionRef                                                        = 3393,
    /// ECU-REF
    EcuRef                                                                 = 2488,
    /// ECU-RESET-CLASS-REF
    EcuResetClassRef                                                       = 187,
    /// ECU-RESOURCE-ESTIMATION
    EcuResourceEstimation                                                  = 5504,
    /// ECU-RESOURCE-MAPPINGS
    EcuResourceMappings                                                    = 3857,
    /// ECU-SCOPE-REF
    EcuScopeRef                                                            = 2244,
    /// ECU-SCOPE-REFS
    EcuScopeRefs                                                           = 265,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 1609,
    /// ECU-TASK-PROXY-REF
    EcuTaskProxyRef                                                        = 746,
    /// ECU-TASK-PROXY-REFS
    EcuTaskProxyRefs                                                       = 4314,
    /// ECU-TIMING
    EcuTiming                                                              = 4794,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 3511,
    /// ECUC-ADD-INFO-PARAM-VALUE
    EcucAddInfoParamValue                                                  = 1698,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 1180,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 5439,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 3873,
    /// ECUC-COND
    EcucCond                                                               = 4904,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 4076,
    /// ECUC-DEF-EDITION
    EcucDefEdition                                                         = 4953,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 4771,
    /// ECUC-DEFINITION-REF
    EcucDefinitionRef                                                      = 1191,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 2236,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 1493,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 5891,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 3678,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 3969,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 5049,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 3307,
    /// ECUC-FUNCTION-NAME-DEF-CONDITIONAL
    EcucFunctionNameDefConditional                                         = 2126,
    /// ECUC-FUNCTION-NAME-DEF-VARIANTS
    EcucFunctionNameDefVariants                                            = 2847,
    /// ECUC-IMPLEMENTATION-CONFIGURATION-CLASS
    EcucImplementationConfigurationClass                                   = 5744,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 218,
    /// ECUC-INSTANCE-REFERENCE-VALUE
    EcucInstanceReferenceValue                                             = 50,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 2677,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 1808,
    /// ECUC-LINKER-SYMBOL-DEF-CONDITIONAL
    EcucLinkerSymbolDefConditional                                         = 3172,
    /// ECUC-LINKER-SYMBOL-DEF-VARIANTS
    EcucLinkerSymbolDefVariants                                            = 4653,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 1280,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF
    EcucModuleConfigurationValuesRef                                       = 704,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL
    EcucModuleConfigurationValuesRefConditional                            = 3928,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 5031,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 1621,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL
    EcucMultilineStringParamDefConditional                                 = 3689,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS
    EcucMultilineStringParamDefVariants                                    = 3506,
    /// ECUC-MULTIPLICITY-CONFIGURATION-CLASS
    EcucMultiplicityConfigurationClass                                     = 4177,
    /// ECUC-NUMERICAL-PARAM-VALUE
    EcucNumericalParamValue                                                = 2086,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 2657,
    /// ECUC-QUERY
    EcucQuery                                                              = 3442,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 3966,
    /// ECUC-QUERY-REF
    EcucQueryRef                                                           = 2413,
    /// ECUC-QUERY-STRING-REF
    EcucQueryStringRef                                                     = 557,
    /// ECUC-QUERYS
    EcucQuerys                                                             = 104,
    /// ECUC-REF
    EcucRef                                                                = 590,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 4141,
    /// ECUC-REFERENCE-VALUE
    EcucReferenceValue                                                     = 416,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 4882,
    /// ECUC-STRING-PARAM-DEF-CONDITIONAL
    EcucStringParamDefConditional                                          = 5393,
    /// ECUC-STRING-PARAM-DEF-VARIANTS
    EcucStringParamDefVariants                                             = 2710,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 796,
    /// ECUC-TEXTUAL-PARAM-VALUE
    EcucTextualParamValue                                                  = 3983,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 1246,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 3625,
    /// ECUC-VALIDATION-CONDS
    EcucValidationConds                                                    = 2678,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 1399,
    /// ECUC-VALUE-CONFIGURATION-CLASS
    EcucValueConfigurationClass                                            = 767,
    /// ECUC-VALUES
    EcucValues                                                             = 690,
    /// EID
    Eid                                                                    = 4582,
    /// EID-RETRIEVAL
    EidRetrieval                                                           = 4475,
    /// EID-USE-MAC
    EidUseMac                                                              = 2459,
    /// ELEMENT
    Element                                                                = 3018,
    /// ELEMENT-BYTE-VALUE
    ElementByteValue                                                       = 5892,
    /// ELEMENT-GROUPS
    ElementGroups                                                          = 5195,
    /// ELEMENT-IN-IMPL-DATATYPE
    ElementInImplDatatype                                                  = 2089,
    /// ELEMENT-MAPPINGS
    ElementMappings                                                        = 2964,
    /// ELEMENT-POSITION
    ElementPosition                                                        = 2862,
    /// ELEMENT-PROPSS
    ElementPropss                                                          = 275,
    /// ELEMENT-REF
    ElementRef                                                             = 3542,
    /// ELEMENT-REFS
    ElementRefs                                                            = 3416,
    /// ELEMENT-ROLE
    ElementRole                                                            = 1570,
    /// ELEMENTS
    Elements                                                               = 1616,
    /// ELLIPTIC-CURVE-REF
    EllipticCurveRef                                                       = 2650,
    /// ELLIPTIC-CURVE-REFS
    EllipticCurveRefs                                                      = 4498,
    /// EMPTY-SIGNAL
    EmptySignal                                                            = 5242,
    /// EMULATION-SUPPORTS
    EmulationSupports                                                      = 1863,
    /// ENABLE-ANYCAST
    EnableAnycast                                                          = 5871,
    /// ENABLE-CONDITION-GROUP-REF
    EnableConditionGroupRef                                                = 1523,
    /// ENABLE-CONDITION-REF
    EnableConditionRef                                                     = 3011,
    /// ENABLE-CONDITIONS
    EnableConditions                                                       = 5200,
    /// ENABLE-DEACTIVATION
    EnableDeactivation                                                     = 5218,
    /// ENABLE-DISCOVERY-PROTECTION
    EnableDiscoveryProtection                                              = 1986,
    /// ENABLE-JOIN-ACCESS-CONTROL
    EnableJoinAccessControl                                                = 5520,
    /// ENABLE-LIVELINESS-PROTECTION
    EnableLivelinessProtection                                             = 5625,
    /// ENABLE-READ-ACCESS-CONTROL
    EnableReadAccessControl                                                = 3957,
    /// ENABLE-TAKE-ADDRESS
    EnableTakeAddress                                                      = 204,
    /// ENABLE-UPDATE
    EnableUpdate                                                           = 5053,
    /// ENABLE-WRITE-ACCESS-CONTROL
    EnableWriteAccessControl                                               = 3327,
    /// ENABLES-LOG-TRACE
    EnablesLogTrace                                                        = 1098,
    /// ENCAPSULATED-ENTRY-REF
    EncapsulatedEntryRef                                                   = 4212,
    /// ENCODING
    Encoding                                                               = 5792,
    /// ENCRYPTION-REF
    EncryptionRef                                                          = 4626,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 5494,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 3904,
    /// END-ADDRESS
    EndAddress                                                             = 1312,
    /// END-TO-END-CALL-RESPONSE-TIMEOUT
    EndToEndCallResponseTimeout                                            = 455,
    /// END-TO-END-PROFILE
    EndToEndProfile                                                        = 1243,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 3591,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 3770,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDUS
    EndToEndProtectionISignalIPdus                                         = 4165,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 2025,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 5826,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPES
    EndToEndProtectionVariablePrototypes                                   = 2893,
    /// END-TO-END-PROTECTIONS
    EndToEndProtections                                                    = 4871,
    /// END-TO-END-TRANSFORMATION-COM-SPEC-PROPS
    EndToEndTransformationComSpecProps                                     = 3268,
    /// END-TO-END-TRANSFORMATION-DESCRIPTION
    EndToEndTransformationDescription                                      = 4885,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS
    EndToEndTransformationISignalProps                                     = 1765,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    EndToEndTransformationISignalPropsConditional                          = 878,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    EndToEndTransformationISignalPropsVariants                             = 2103,
    /// ENDPOINT-CONFIGURATION-REF
    EndpointConfigurationRef                                               = 568,
    /// ENDPOINTS
    Endpoints                                                              = 2319,
    /// ENGINEERING-OBJECT
    EngineeringObject                                                      = 488,
    /// ENGINEERING-OBJECTS
    EngineeringObjects                                                     = 2970,
    /// ENHANCED-MODE-API
    EnhancedModeApi                                                        = 2298,
    /// ENTER-TIMEOUT-VALUE
    EnterTimeoutValue                                                      = 4752,
    /// ENTERED-MODE-REF
    EnteredModeRef                                                         = 5525,
    /// ENTITY-STATUS-MAX-BYTE-FIELD-USE
    EntityStatusMaxByteFieldUse                                            = 2030,
    /// ENTITYS
    Entitys                                                                = 3641,
    /// ENTRY
    Entry                                                                  = 1499,
    /// ENTRY-MODE-DECLARATION-REF
    EntryModeDeclarationRef                                                = 3528,
    /// ENTRY-REF
    EntryRef                                                               = 3425,
    /// ENTRY-TYPE
    EntryType                                                              = 4748,
    /// ENTRYS
    Entrys                                                                 = 1130,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 4333,
    /// ENUMERATOR-VALUE
    EnumeratorValue                                                        = 4558,
    /// ENUMS
    Enums                                                                  = 2111,
    /// ENVIRONMENT-CAPTURE-TO-REPORTING
    EnvironmentCaptureToReporting                                          = 5213,
    /// ENVIRONMENT-DATA-CAPTURE
    EnvironmentDataCapture                                                 = 2797,
    /// ENVIRONMENT-VARIABLES
    EnvironmentVariables                                                   = 5047,
    /// ENVIRONMENTAL-CONDITION-REF
    EnvironmentalConditionRef                                              = 4087,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 3074,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 1982,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 5443,
    /// ERROR-CODE
    ErrorCode                                                              = 4402,
    /// ERROR-CONTEXT-REF
    ErrorContextRef                                                        = 1031,
    /// ERROR-CONTEXT-REFS
    ErrorContextRefs                                                       = 1315,
    /// ERROR-DOMAIN-REF
    ErrorDomainRef                                                         = 634,
    /// ERROR-HANDLING
    ErrorHandling                                                          = 550,
    /// ERROR-MAPPINGS
    ErrorMappings                                                          = 3158,
    /// ERROR-REACTION-POLICY
    ErrorReactionPolicy                                                    = 900,
    /// ERROR-SIGNALING-ENABLED
    ErrorSignalingEnabled                                                  = 3401,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 4753,
    /// ESP-CIPHER-SUITE-NAME
    EspCipherSuiteName                                                     = 3004,
    /// ESP-CIPHER-SUITE-NAMES
    EspCipherSuiteNames                                                    = 3317,
    /// ESTIMATED-DURATION-OF-CAMPAIGN
    EstimatedDurationOfCampaign                                            = 4306,
    /// ESTIMATED-DURATION-OF-OPERATION
    EstimatedDurationOfOperation                                           = 1805,
    /// ESTIMATED-EXECUTION-TIME
    EstimatedExecutionTime                                                 = 82,
    /// ETH-GLOBAL-TIME-DOMAIN-PROPS
    EthGlobalTimeDomainProps                                               = 4173,
    /// ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT
    EthGlobalTimeManagedCouplingPort                                       = 5873,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 61,
    /// ETH-IP-PROPS-REF
    EthIpPropsRef                                                          = 4196,
    /// ETH-IP-PROPS-REFS
    EthIpPropsRefs                                                         = 2772,
    /// ETH-SWITCH-PORT-GROUP-DERIVATION
    EthSwitchPortGroupDerivation                                           = 2872,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 1591,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 3538,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 1759,
    /// ETH-TP-CONNECTION
    EthTpConnection                                                        = 5484,
    /// ETHER-TYPE
    EtherType                                                              = 3945,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 4556,
    /// ETHERNET-CLUSTER-CONDITIONAL
    EthernetClusterConditional                                             = 5907,
    /// ETHERNET-CLUSTER-REF
    EthernetClusterRef                                                     = 240,
    /// ETHERNET-CLUSTER-VARIANTS
    EthernetClusterVariants                                                = 4755,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 4218,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 5707,
    /// ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL
    EthernetCommunicationControllerConditional                             = 2371,
    /// ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS
    EthernetCommunicationControllerVariants                                = 4548,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 2226,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 778,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 5360,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 4353,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 757,
    /// ETHERNET-PRIORITY-REGENERATIONS
    EthernetPriorityRegenerations                                          = 3102,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 4091,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 5130,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 2201,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING-REF
    EthernetRawDataStreamMappingRef                                        = 993,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 5704,
    /// ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS
    EthernetTrafficClassAssignments                                        = 4156,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 1036,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 4004,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS
    EthernetWakeupSleepOnDatalineConfigs                                   = 1124,
    /// EVALUATED-ELEMENT-REF
    EvaluatedElementRef                                                    = 1353,
    /// EVALUATED-ELEMENT-REFS
    EvaluatedElementRefs                                                   = 3972,
    /// EVALUATED-VARIANT-REF
    EvaluatedVariantRef                                                    = 3487,
    /// EVALUATED-VARIANT-REFS
    EvaluatedVariantRefs                                                   = 2046,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 2510,
    /// EVENT-CLEAR-ALLOWED
    EventClearAllowed                                                      = 5257,
    /// EVENT-COMBINATION-REPORTING-BEHAVIOR
    EventCombinationReportingBehavior                                      = 5476,
    /// EVENT-CONTROLLED-TIMING
    EventControlledTiming                                                  = 1601,
    /// EVENT-DEPLOYMENTS
    EventDeployments                                                       = 3563,
    /// EVENT-DISPLACEMENT-STRATEGY
    EventDisplacementStrategy                                              = 5165,
    /// EVENT-ELEMENT-MAPPINGS
    EventElementMappings                                                   = 1516,
    /// EVENT-ELEMENTS
    EventElements                                                          = 2846,
    /// EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD
    EventFailureCycleCounterThreshold                                      = 1473,
    /// EVENT-GROUP-CONTROL-TYPE
    EventGroupControlType                                                  = 2005,
    /// EVENT-GROUP-ID
    EventGroupId                                                           = 3281,
    /// EVENT-GROUP-IDENTIFIER
    EventGroupIdentifier                                                   = 1240,
    /// EVENT-GROUP-REF
    EventGroupRef                                                          = 3507,
    /// EVENT-GROUP-REFS
    EventGroupRefs                                                         = 4739,
    /// EVENT-GROUPS
    EventGroups                                                            = 2519,
    /// EVENT-HANDLER
    EventHandler                                                           = 3488,
    /// EVENT-HANDLER-REF
    EventHandlerRef                                                        = 886,
    /// EVENT-HANDLER-REFS
    EventHandlerRefs                                                       = 1512,
    /// EVENT-HANDLERS
    EventHandlers                                                          = 3386,
    /// EVENT-ID
    EventId                                                                = 2902,
    /// EVENT-IREF
    EventIref                                                              = 1657,
    /// EVENT-KIND
    EventKind                                                              = 213,
    /// EVENT-MAPPING
    EventMapping                                                           = 1474,
    /// EVENT-MAPPINGS
    EventMappings                                                          = 4536,
    /// EVENT-MULTICAST-ADDRESSS
    EventMulticastAddresss                                                 = 3309,
    /// EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS
    EventMulticastSubscriptionAddresss                                     = 2441,
    /// EVENT-MULTICAST-UDP-PORT
    EventMulticastUdpPort                                                  = 4270,
    /// EVENT-NEEDS-REF
    EventNeedsRef                                                          = 4805,
    /// EVENT-OBD-READINESS-GROUP
    EventObdReadinessGroup                                                 = 1322,
    /// EVENT-OCCURRENCE-KIND
    EventOccurrenceKind                                                    = 660,
    /// EVENT-PROPSS
    EventPropss                                                            = 3634,
    /// EVENT-QOS-PROPSS
    EventQosPropss                                                         = 5767,
    /// EVENT-READINESS-GROUPS
    EventReadinessGroups                                                   = 3639,
    /// EVENT-REF
    EventRef                                                               = 3539,
    /// EVENT-REFS
    EventRefs                                                              = 4521,
    /// EVENT-SOURCE-REF
    EventSourceRef                                                         = 4228,
    /// EVENT-SYMBOL-NAME
    EventSymbolName                                                        = 4050,
    /// EVENT-TOPIC-ACCESS-RULE-REF
    EventTopicAccessRuleRef                                                = 3642,
    /// EVENT-WINDOW-TIME
    EventWindowTime                                                        = 2439,
    /// EVENT-WINDOWS
    EventWindows                                                           = 5170,
    /// EVENTS
    Events                                                                 = 3097,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 4422,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 5330,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REF
    ExclusiveAreaNestingOrderRef                                           = 2448,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REFS
    ExclusiveAreaNestingOrderRefs                                          = 1690,
    /// EXCLUSIVE-AREA-NESTING-ORDERS
    ExclusiveAreaNestingOrders                                             = 1454,
    /// EXCLUSIVE-AREA-POLICYS
    ExclusiveAreaPolicys                                                   = 442,
    /// EXCLUSIVE-AREA-REF
    ExclusiveAreaRef                                                       = 5408,
    /// EXCLUSIVE-AREA-REF-CONDITIONAL
    ExclusiveAreaRefConditional                                            = 122,
    /// EXCLUSIVE-AREA-REFS
    ExclusiveAreaRefs                                                      = 3376,
    /// EXCLUSIVE-AREAS
    ExclusiveAreas                                                         = 1108,
    /// EXEC-IN-USER-MODE
    ExecInUserMode                                                         = 3365,
    /// EXECUTABLE
    Executable                                                             = 5775,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 5915,
    /// EXECUTABLE-ENTITY-REF
    ExecutableEntityRef                                                    = 979,
    /// EXECUTABLE-ENTITY-REFS
    ExecutableEntityRefs                                                   = 502,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 3248,
    /// EXECUTABLE-LOGGING-IMPLEMENTATION-PROPS
    ExecutableLoggingImplementationProps                                   = 2192,
    /// EXECUTABLE-PROVIDED-PORT-IREF
    ExecutableProvidedPortIref                                             = 5122,
    /// EXECUTABLE-REF
    ExecutableRef                                                          = 3054,
    /// EXECUTABLE-REFS
    ExecutableRefs                                                         = 3732,
    /// EXECUTABLE-REQUIRED-PORT-IREF
    ExecutableRequiredPortIref                                             = 4531,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 3471,
    /// EXECUTE-DESPITE-DATA-UNAVAILABILITY
    ExecuteDespiteDataUnavailability                                       = 2551,
    /// EXECUTION-CONTEXT
    ExecutionContext                                                       = 1285,
    /// EXECUTION-CONTEXT-REF
    ExecutionContextRef                                                    = 2028,
    /// EXECUTION-CONTEXT-REFS
    ExecutionContextRefs                                                   = 1738,
    /// EXECUTION-CONTEXTS
    ExecutionContexts                                                      = 5553,
    /// EXECUTION-DEPENDENCY
    ExecutionDependency                                                    = 4038,
    /// EXECUTION-DEPENDENCYS
    ExecutionDependencys                                                   = 2718,
    /// EXECUTION-ERROR
    ExecutionError                                                         = 1574,
    /// EXECUTION-ERROR-REF
    ExecutionErrorRef                                                      = 553,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 2383,
    /// EXECUTION-ORDER-CONSTRAINT-TYPE
    ExecutionOrderConstraintType                                           = 5064,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 2693,
    /// EXECUTION-TIME-TYPE
    ExecutionTimeType                                                      = 3483,
    /// EXECUTION-TIMES
    ExecutionTimes                                                         = 2518,
    /// EXIT-MODE-DECLARATION-REF
    ExitModeDeclarationRef                                                 = 4483,
    /// EXIT-TIMEOUT-VALUE
    ExitTimeoutValue                                                       = 5787,
    /// EXITED-MODE-REF
    ExitedModeRef                                                          = 3446,
    /// EXPECTED-ALIVE-CYCLE
    ExpectedAliveCycle                                                     = 4951,
    /// EXPECTED-ALIVE-INDICATIONS
    ExpectedAliveIndications                                               = 3308,
    /// EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE
    ExpectedAliveIndicationsInSupervisionCycle                             = 2288,
    /// EXPECTED-CALLBACKS
    ExpectedCallbacks                                                      = 1848,
    /// EXPECTED-ENTRYS
    ExpectedEntrys                                                         = 2728,
    /// EXPECTED-TX-TRIGGER
    ExpectedTxTrigger                                                      = 846,
    /// EXPIRED-SUPERVISION-CYCLE-TOL
    ExpiredSupervisionCycleTol                                             = 10,
    /// EXPIRED-SUPERVISION-CYCLES-TOLERANCE
    ExpiredSupervisionCyclesTolerance                                      = 1797,
    /// EXPIRED-SUPERVISION-TOLERANCE
    ExpiredSupervisionTolerance                                            = 3762,
    /// EXPLICIT-CONGESTION-NOTIFICATION
    ExplicitCongestionNotification                                         = 4140,
    /// EXPLICIT-INTER-RUNNABLE-VARIABLES
    ExplicitInterRunnableVariables                                         = 1853,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REF
    ExplicitRptProfileSelectionRef                                         = 2698,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REFS
    ExplicitRptProfileSelectionRefs                                        = 5346,
    /// EXPORTABILITY
    Exportability                                                          = 3535,
    /// EXPRESSION
    Expression                                                             = 551,
    /// EXPRESSION-REF
    ExpressionRef                                                          = 2717,
    /// EXT-HEADER-FILTER-LISTS
    ExtHeaderFilterLists                                                   = 826,
    /// EXTENDED-ADDRESSING
    ExtendedAddressing                                                     = 4449,
    /// EXTENDED-DATA-RECORDS
    ExtendedDataRecords                                                    = 3912,
    /// EXTENDS-META-CLASS
    ExtendsMetaClass                                                       = 4240,
    /// EXTERN-OFFSET-CORRECTION
    ExternOffsetCorrection                                                 = 3463,
    /// EXTERN-RATE-CORRECTION
    ExternRateCorrection                                                   = 2266,
    /// EXTERNAL-AUTHENTICATIONS
    ExternalAuthentications                                                = 4518,
    /// EXTERNAL-CLOCK-SYNCHRONISATION
    ExternalClockSynchronisation                                           = 577,
    /// EXTERNAL-REPLACEMENT-REF
    ExternalReplacementRef                                                 = 4383,
    /// EXTERNAL-SYNC
    ExternalSync                                                           = 3115,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 4464,
    /// EXTERNAL-TRIGGERING-POINT
    ExternalTriggeringPoint                                                = 4443,
    /// EXTERNAL-TRIGGERING-POINTS
    ExternalTriggeringPoints                                               = 3536,
    /// FACTOR-SI-TO-UNIT
    FactorSiToUnit                                                         = 2566,
    /// FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL
    FailedAliveSupervisionRefCycleTol                                      = 5113,
    /// FAILED-REFERENCE-CYCLES-TOLERANCE
    FailedReferenceCyclesTolerance                                         = 1270,
    /// FAILED-SUPERVISION-CYCLES-TOLERANCE
    FailedSupervisionCyclesTolerance                                       = 5867,
    /// FAILURE-MONITORING-REF
    FailureMonitoringRef                                                   = 1510,
    /// FALL-BACK-INTERNAL
    FallBackInternal                                                       = 4610,
    /// FALSE-ACTION-LIST-REF
    FalseActionListRef                                                     = 5809,
    /// FDC-THRESHOLD-STORAGE-VALUE
    FdcThresholdStorageValue                                               = 2947,
    /// FEATURE-IREF
    FeatureIref                                                            = 3870,
    /// FEATURE-MODEL-REF
    FeatureModelRef                                                        = 5171,
    /// FEATURE-MODEL-REFS
    FeatureModelRefs                                                       = 1916,
    /// FEATURE-REF
    FeatureRef                                                             = 2995,
    /// FEATURE-REFS
    FeatureRefs                                                            = 4801,
    /// FIBEX-ELEMENT-REF
    FibexElementRef                                                        = 3035,
    /// FIBEX-ELEMENT-REF-CONDITIONAL
    FibexElementRefConditional                                             = 3443,
    /// FIBEX-ELEMENTS
    FibexElements                                                          = 1667,
    /// FIELD
    Field                                                                  = 1288,
    /// FIELD-DEPLOYMENTS
    FieldDeployments                                                       = 1849,
    /// FIELD-ELEMENTS
    FieldElements                                                          = 2820,
    /// FIELD-GET-SET-QOS-PROPSS
    FieldGetSetQosPropss                                                   = 5554,
    /// FIELD-IREF
    FieldIref                                                              = 5761,
    /// FIELD-MAPPING
    FieldMapping                                                           = 5106,
    /// FIELD-MAPPINGS
    FieldMappings                                                          = 74,
    /// FIELD-NOTIFIER-QOS-PROPSS
    FieldNotifierQosPropss                                                 = 3236,
    /// FIELD-NOTIFIER-REF
    FieldNotifierRef                                                       = 2715,
    /// FIELD-REF
    FieldRef                                                               = 1518,
    /// FIELD-REFS
    FieldRefs                                                              = 3364,
    /// FIELD-REPLY-TOPIC-NAME
    FieldReplyTopicName                                                    = 3791,
    /// FIELD-REQUEST-TOPIC-NAME
    FieldRequestTopicName                                                  = 3125,
    /// FIELD-SENDER-COM-SPEC
    FieldSenderComSpec                                                     = 453,
    /// FIELD-TOPICS-ACCESS-RULE-REF
    FieldTopicsAccessRuleRef                                               = 5628,
    /// FIELDS
    Fields                                                                 = 392,
    /// FIFO-DEPTH
    FifoDepth                                                              = 1126,
    /// FIFO-RANGES
    FifoRanges                                                             = 5188,
    /// FIGURE
    Figure                                                                 = 285,
    /// FIGURE-CAPTION
    FigureCaption                                                          = 3083,
    /// FILE-ELEMENTS
    FileElements                                                           = 3046,
    /// FILE-INFO-COMMENT
    FileInfoComment                                                        = 3262,
    /// FILE-NAME
    FileName                                                               = 231,
    /// FILE-PROXYS
    FileProxys                                                             = 116,
    /// FILE-REF
    FileRef                                                                = 3658,
    /// FILE-REFS
    FileRefs                                                               = 4727,
    /// FILE-STORAGE-REF
    FileStorageRef                                                         = 1917,
    /// FILE-TYPE
    FileType                                                               = 106,
    /// FILE-TYPE-PATTERN
    FileTypePattern                                                        = 5948,
    /// FILES
    Files                                                                  = 515,
    /// FILTER
    Filter                                                                 = 731,
    /// FILTER-CHAINS
    FilterChains                                                           = 2158,
    /// FILTERING-DEBOUNCING
    FilteringDebouncing                                                    = 4011,
    /// FINAL-CHECKPOINT-REF
    FinalCheckpointRef                                                     = 3466,
    /// FINAL-CHECKPOINT-REFS
    FinalCheckpointRefs                                                    = 4012,
    /// FIRE-AND-FORGET
    FireAndForget                                                          = 4811,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 58,
    /// FIRE-AND-FORGET-MAPPINGS
    FireAndForgetMappings                                                  = 1732,
    /// FIRE-AND-FORGET-METHOD-MAPPING
    FireAndForgetMethodMapping                                             = 4455,
    /// FIRE-AND-FORGET-METHOD-MAPPINGS
    FireAndForgetMethodMappings                                            = 3989,
    /// FIREWALL-REF
    FirewallRef                                                            = 840,
    /// FIREWALL-RULE
    FirewallRule                                                           = 2732,
    /// FIREWALL-RULE-PROPS
    FirewallRuleProps                                                      = 3550,
    /// FIREWALL-RULE-PROPSS
    FirewallRulePropss                                                     = 3594,
    /// FIREWALL-RULE-REF
    FirewallRuleRef                                                        = 5950,
    /// FIREWALL-RULE-REFS
    FirewallRuleRefs                                                       = 1475,
    /// FIREWALL-STATE-IREF
    FirewallStateIref                                                      = 822,
    /// FIREWALL-STATE-IREFS
    FirewallStateIrefs                                                     = 3039,
    /// FIREWALL-STATE-MACHINES
    FirewallStateMachines                                                  = 5119,
    /// FIREWALL-STATE-SWITCH-INTERFACE
    FirewallStateSwitchInterface                                           = 3500,
    /// FIRST-APPLICATION-ERROR-REF
    FirstApplicationErrorRef                                               = 4581,
    /// FIRST-DATA-PROTOTYPE-REF
    FirstDataPrototypeRef                                                  = 4799,
    /// FIRST-ELEMENT-INSTANCE-IREF
    FirstElementInstanceIref                                               = 975,
    /// FIRST-ELEMENT-INSTANCE-IREFS
    FirstElementInstanceIrefs                                              = 4887,
    /// FIRST-ELEMENT-REF
    FirstElementRef                                                        = 1064,
    /// FIRST-ELEMENT-REFS
    FirstElementRefs                                                       = 2980,
    /// FIRST-ELEMENTS
    FirstElements                                                          = 5845,
    /// FIRST-MODE-GROUP-REF
    FirstModeGroupRef                                                      = 1439,
    /// FIRST-MODE-REF
    FirstModeRef                                                           = 748,
    /// FIRST-MODE-REFS
    FirstModeRefs                                                          = 1895,
    /// FIRST-OPERATION-REF
    FirstOperationRef                                                      = 2607,
    /// FIRST-PHYSICAL-DIMENSION-REF
    FirstPhysicalDimensionRef                                              = 151,
    /// FIRST-PORT-REF
    FirstPortRef                                                           = 602,
    /// FIRST-TIMEOUT
    FirstTimeout                                                           = 5092,
    /// FIRST-TO-SECOND-DATA-TRANSFORMATION-REF
    FirstToSecondDataTransformationRef                                     = 3141,
    /// FIRST-TRIGGER-REF
    FirstTriggerRef                                                        = 904,
    /// FIRST-VALUE
    FirstValue                                                             = 1573,
    /// FIXED-LENGTH
    FixedLength                                                            = 3424,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 2454,
    /// FLAT-INSTANCE-REF
    FlatInstanceRef                                                        = 3740,
    /// FLAT-MAP
    FlatMap                                                                = 3480,
    /// FLAT-MAP-ENTRY-REF
    FlatMapEntryRef                                                        = 1850,
    /// FLAT-MAP-ENTRY-REFS
    FlatMapEntryRefs                                                       = 5542,
    /// FLAT-MAP-REF
    FlatMapRef                                                             = 5663,
    /// FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING
    FlexrayAbsolutelyScheduledTiming                                       = 2421,
    /// FLEXRAY-AR-TP-CHANNEL
    FlexrayArTpChannel                                                     = 2107,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 887,
    /// FLEXRAY-AR-TP-CONNECTION
    FlexrayArTpConnection                                                  = 3047,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 2748,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 2649,
    /// FLEXRAY-CLUSTER-CONDITIONAL
    FlexrayClusterConditional                                              = 3360,
    /// FLEXRAY-CLUSTER-VARIANTS
    FlexrayClusterVariants                                                 = 888,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 273,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 760,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL
    FlexrayCommunicationControllerConditional                              = 1566,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS
    FlexrayCommunicationControllerVariants                                 = 1321,
    /// FLEXRAY-FIFO-CONFIGURATION
    FlexrayFifoConfiguration                                               = 5384,
    /// FLEXRAY-FIFO-RANGE
    FlexrayFifoRange                                                       = 145,
    /// FLEXRAY-FIFOS
    FlexrayFifos                                                           = 5803,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 2940,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 1716,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 5636,
    /// FLEXRAY-NM-CLUSTER-COUPLING
    FlexrayNmClusterCoupling                                               = 5614,
    /// FLEXRAY-NM-ECU
    FlexrayNmEcu                                                           = 2683,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 554,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 3925,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 196,
    /// FLEXRAY-TP-CONNECTION
    FlexrayTpConnection                                                    = 2447,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 3284,
    /// FLEXRAY-TP-ECU
    FlexrayTpEcu                                                           = 5805,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 3600,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 3647,
    /// FLOAT
    Float                                                                  = 2563,
    /// FLOAT-VALUE-VARIATION-POINT
    FloatValueVariationPoint                                               = 675,
    /// FLOW-CONTROL-PDU-REF
    FlowControlPduRef                                                      = 812,
    /// FLOW-CONTROL-PDU-REFS
    FlowControlPduRefs                                                     = 1366,
    /// FLOW-CONTROL-REF
    FlowControlRef                                                         = 2572,
    /// FLOW-LABEL
    FlowLabel                                                              = 4947,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 4081,
    /// FM-ATTRIBUTE-VALUE
    FmAttributeValue                                                       = 926,
    /// FM-COND
    FmCond                                                                 = 977,
    /// FM-FEATURE
    FmFeature                                                              = 4121,
    /// FM-FEATURE-DECOMPOSITION
    FmFeatureDecomposition                                                 = 5278,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 5842,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 2525,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 2418,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 4768,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 5863,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 3444,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 782,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 3917,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 1437,
    /// FM-SYSCOND
    FmSyscond                                                              = 3728,
    /// FMI
    Fmi                                                                    = 4732,
    /// FOLLOW-UP-ACTION-REF
    FollowUpActionRef                                                      = 5303,
    /// FOLLOW-UP-ACTION-REFS
    FollowUpActionRefs                                                     = 5364,
    /// FOLLOW-UP-OFFSET
    FollowUpOffset                                                         = 4451,
    /// FOLLOW-UP-TIMEOUT-VALUE
    FollowUpTimeoutValue                                                   = 1923,
    /// FORBIDDEN-SIGNAL-PATH
    ForbiddenSignalPath                                                    = 613,
    /// FOREIGN-MODEL-REFERENCE
    ForeignModelReference                                                  = 581,
    /// FORMAL-BLUEPRINT-CONDITION
    FormalBlueprintCondition                                               = 22,
    /// FORMAL-BLUEPRINT-GENERATOR
    FormalBlueprintGenerator                                               = 2258,
    /// FORMAT
    Format                                                                 = 2920,
    /// FORMULA
    Formula                                                                = 4596,
    /// FORMULA-CAPTION
    FormulaCaption                                                         = 5890,
    /// FR-CLUSTER-REF
    FrClusterRef                                                           = 4549,
    /// FR-GLOBAL-TIME-DOMAIN-PROPS
    FrGlobalTimeDomainProps                                                = 2500,
    /// FRAGMENT
    Fragment                                                               = 4740,
    /// FRAGMENTATION-PROPS
    FragmentationProps                                                     = 576,
    /// FRAME-LENGTH
    FrameLength                                                            = 657,
    /// FRAME-MAPPING
    FrameMapping                                                           = 5415,
    /// FRAME-MAPPINGS
    FrameMappings                                                          = 3241,
    /// FRAME-PID
    FramePid                                                               = 5342,
    /// FRAME-PIDS
    FramePids                                                              = 5101,
    /// FRAME-PORT
    FramePort                                                              = 4959,
    /// FRAME-PORT-REF
    FramePortRef                                                           = 4307,
    /// FRAME-PORT-REFS
    FramePortRefs                                                          = 793,
    /// FRAME-REF
    FrameRef                                                               = 4315,
    /// FRAME-TRIGGERING-REF
    FrameTriggeringRef                                                     = 5102,
    /// FRAME-TRIGGERINGS
    FrameTriggerings                                                       = 739,
    /// FREE-FORMAT
    FreeFormat                                                             = 4683,
    /// FREEZE-CURRENT-STATE
    FreezeCurrentState                                                     = 2498,
    /// FREEZE-CURRENT-STATE-SUPPORTED
    FreezeCurrentStateSupported                                            = 4839,
    /// FREEZE-FRAME-CONTENT-REF
    FreezeFrameContentRef                                                  = 5114,
    /// FREEZE-FRAME-CONTENT-WWH-OBD-REF
    FreezeFrameContentWwhObdRef                                            = 5313,
    /// FREEZE-FRAME-REF
    FreezeFrameRef                                                         = 1123,
    /// FREEZE-FRAMES
    FreezeFrames                                                           = 3084,
    /// FRESHNESS-COUNTER-SYNC-ATTEMPTS
    FreshnessCounterSyncAttempts                                           = 1904,
    /// FRESHNESS-PROPS-REF
    FreshnessPropsRef                                                      = 2598,
    /// FRESHNESS-PROPSS
    FreshnessPropss                                                        = 4203,
    /// FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR
    FreshnessTimestampTimePeriodFactor                                     = 4616,
    /// FRESHNESS-VALUE-ID
    FreshnessValueId                                                       = 1072,
    /// FRESHNESS-VALUE-LENGTH
    FreshnessValueLength                                                   = 5652,
    /// FRESHNESS-VALUE-TX-LENGTH
    FreshnessValueTxLength                                                 = 1472,
    /// FROM-REF
    FromRef                                                                = 4404,
    /// FT
    Ft                                                                     = 730,
    /// FULL-DUPLEX-ENABLED
    FullDuplexEnabled                                                      = 5703,
    /// FULLY-QUALIFIED-DOMAIN-NAME
    FullyQualifiedDomainName                                               = 1344,
    /// FUNCTION
    Function                                                               = 4597,
    /// FUNCTION-CLUSTER-AFFILIATION
    FunctionClusterAffiliation                                             = 3155,
    /// FUNCTION-GROUP-IREF
    FunctionGroupIref                                                      = 4199,
    /// FUNCTION-GROUP-IREFS
    FunctionGroupIrefs                                                     = 1817,
    /// FUNCTION-GROUP-MODE-IREF
    FunctionGroupModeIref                                                  = 514,
    /// FUNCTION-GROUP-MODE-IREFS
    FunctionGroupModeIrefs                                                 = 5336,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 2051,
    /// FUNCTION-GROUP-PHM-STATE-REFERENCE
    FunctionGroupPhmStateReference                                         = 5490,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 3060,
    /// FUNCTION-GROUP-STATE-IREF
    FunctionGroupStateIref                                                 = 3423,
    /// FUNCTION-GROUP-STATE-IREFS
    FunctionGroupStateIrefs                                                = 2608,
    /// FUNCTION-GROUPS
    FunctionGroups                                                         = 2628,
    /// FUNCTION-ID
    FunctionId                                                             = 4932,
    /// FUNCTION-IDENTIFIER-REF
    FunctionIdentifierRef                                                  = 368,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 4600,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 2172,
    /// FUNCTION-INSTANCE
    FunctionInstance                                                       = 3654,
    /// FUNCTION-POINTER-SIGNATURE-REF
    FunctionPointerSignatureRef                                            = 438,
    /// FUNCTION-PROTOTYPE-EMITTER
    FunctionPrototypeEmitter                                               = 3767,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING
    FunctionalClusterInteractsWithPersistencyDeploymentMapping             = 639,
    /// FUNCTIONAL-CLUSTER-REF
    FunctionalClusterRef                                                   = 582,
    /// FUNCTIONAL-REQUEST-REF
    FunctionalRequestRef                                                   = 1451,
    /// FUNCTIONAL-REQUEST-REFS
    FunctionalRequestRefs                                                  = 5677,
    /// FUNCTIONAL-UNIT
    FunctionalUnit                                                         = 5281,
    /// FUP-DATA-ID-LIST
    FupDataIdList                                                          = 2037,
    /// FUP-DATA-ID-LISTS
    FupDataIdLists                                                         = 1789,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 1149,
    /// GATEWAY
    Gateway                                                                = 3537,
    /// GATEWAYS
    Gateways                                                               = 3090,
    /// GENERAL-INACTIVITY-TIME
    GeneralInactivityTime                                                  = 4490,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 1572,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 271,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 1825,
    /// GENERATED-ARTIFACTS
    GeneratedArtifacts                                                     = 3922,
    /// GENERATOR
    Generator                                                              = 2134,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 709,
    /// GENERIC-MATH
    GenericMath                                                            = 1238,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 3305,
    /// GENERIC-TP
    GenericTp                                                              = 1614,
    /// GET
    Get                                                                    = 5370,
    /// GETTER-CALL-REF
    GetterCallRef                                                          = 4793,
    /// GETTER-CALL-SIGNAL-REF
    GetterCallSignalRef                                                    = 2580,
    /// GETTER-OPERATION-REF
    GetterOperationRef                                                     = 121,
    /// GETTER-REF
    GetterRef                                                              = 4226,
    /// GETTER-RETURN-REF
    GetterReturnRef                                                        = 4399,
    /// GETTER-RETURN-SIGNAL-REF
    GetterReturnSignalRef                                                  = 5732,
    /// GID
    Gid                                                                    = 4129,
    /// GID-INVALIDITY-PATTERN
    GidInvalidityPattern                                                   = 3193,
    /// GLOBAL-ELEMENT
    GlobalElement                                                          = 3674,
    /// GLOBAL-ELEMENTS
    GlobalElements                                                         = 4936,
    /// GLOBAL-IN-PACKAGE-REF
    GlobalInPackageRef                                                     = 2370,
    /// GLOBAL-IN-PACKAGE-REFS
    GlobalInPackageRefs                                                    = 4963,
    /// GLOBAL-KAY-PROPS-REF
    GlobalKayPropsRef                                                      = 2805,
    /// GLOBAL-RESOURCE-ID
    GlobalResourceId                                                       = 1483,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 4574,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 3171,
    /// GLOBAL-SUPERVISION-ENTITYS
    GlobalSupervisionEntitys                                               = 1488,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 5121,
    /// GLOBAL-SUPERVISIONS
    GlobalSupervisions                                                     = 4093,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 2307,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 538,
    /// GLOBAL-TIME-CORRECTION-PROPS
    GlobalTimeCorrectionProps                                              = 5458,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 526,
    /// GLOBAL-TIME-DOMAIN-PROPERTYS
    GlobalTimeDomainPropertys                                              = 5297,
    /// GLOBAL-TIME-DOMAIN-PROPS
    GlobalTimeDomainProps                                                  = 3406,
    /// GLOBAL-TIME-DOMAIN-REF
    GlobalTimeDomainRef                                                    = 387,
    /// GLOBAL-TIME-DOMAIN-REF-CONDITIONAL
    GlobalTimeDomainRefConditional                                         = 5396,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 4390,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 1724,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 2023,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 2890,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 5345,
    /// GLOBAL-TIME-MASTERS
    GlobalTimeMasters                                                      = 1965,
    /// GLOBAL-TIME-PDU-REF
    GlobalTimePduRef                                                       = 4162,
    /// GLOBAL-TIME-PDU-TRIGGERING-REF
    GlobalTimePduTriggeringRef                                             = 3272,
    /// GLOBAL-TIME-PROPS
    GlobalTimeProps                                                        = 4659,
    /// GLOBAL-TIME-SUB-DOMAINS
    GlobalTimeSubDomains                                                   = 1515,
    /// GOVERNANCE-REF
    GovernanceRef                                                          = 2482,
    /// GRANT-REF
    GrantRef                                                               = 381,
    /// GRANT-REFS
    GrantRefs                                                              = 3906,
    /// GRAPHIC
    Graphic                                                                = 5791,
    /// GROUP-ID
    GroupId                                                                = 1320,
    /// GROUP-IDENTIFIER
    GroupIdentifier                                                        = 3485,
    /// GROUP-NUMBER
    GroupNumber                                                            = 2145,
    /// GROUPED-ALIAS-EVENT-REF
    GroupedAliasEventRef                                                   = 1198,
    /// GROUPED-ALIAS-EVENT-REFS
    GroupedAliasEventRefs                                                  = 1992,
    /// HANDLE-DATA-STATUS
    HandleDataStatus                                                       = 1672,
    /// HANDLE-INVALID
    HandleInvalid                                                          = 3312,
    /// HANDLE-NEVER-RECEIVED
    HandleNeverReceived                                                    = 4983,
    /// HANDLE-OUT-OF-RANGE
    HandleOutOfRange                                                       = 3181,
    /// HANDLE-OUT-OF-RANGE-STATUS
    HandleOutOfRangeStatus                                                 = 94,
    /// HANDLE-TERMINATION-AND-RESTART
    HandleTerminationAndRestart                                            = 4277,
    /// HANDLE-TIMEOUT-TYPE
    HandleTimeoutType                                                      = 3138,
    /// HARDWARE-CONFIGURATION
    HardwareConfiguration                                                  = 3505,
    /// HARDWARE-ELEMENT-REF
    HardwareElementRef                                                     = 5177,
    /// HARDWARE-ELEMENT-REFS
    HardwareElementRefs                                                    = 604,
    /// HARDWARE-PLATFORM
    HardwarePlatform                                                       = 3826,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 1161,
    /// HAS-DYNAMIC-LENGTH
    HasDynamicLength                                                       = 1163,
    /// HAS-GETTER
    HasGetter                                                              = 958,
    /// HAS-INTERNAL-STATE
    HasInternalState                                                       = 469,
    /// HAS-NOTIFIER
    HasNotifier                                                            = 4146,
    /// HAS-SETTER
    HasSetter                                                              = 5654,
    /// HEADER-FILE
    HeaderFile                                                             = 3597,
    /// HEADER-ID
    HeaderId                                                               = 3148,
    /// HEADER-ID-LONG-HEADER
    HeaderIdLongHeader                                                     = 4593,
    /// HEADER-ID-SHORT-HEADER
    HeaderIdShortHeader                                                    = 658,
    /// HEADER-LENGTH
    HeaderLength                                                           = 3851,
    /// HEADER-TYPE
    HeaderType                                                             = 1842,
    /// HEALING-CYCLE-COUNTER-THRESHOLD
    HealingCycleCounterThreshold                                           = 5883,
    /// HEALING-CYCLE-REF
    HealingCycleRef                                                        = 1460,
    /// HEALTH-CHANNEL-ARGUMENT-REF
    HealthChannelArgumentRef                                               = 5831,
    /// HEALTH-CHANNEL-ARGUMENT-REFS
    HealthChannelArgumentRefs                                              = 5046,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 449,
    /// HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS
    HealthChannelExternalReportedStatus                                    = 4413,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 774,
    /// HEALTH-CHANNEL-ID
    HealthChannelId                                                        = 1576,
    /// HEALTH-CHANNEL-IREF
    HealthChannelIref                                                      = 2635,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 5455,
    /// HEALTH-CHANNELS
    HealthChannels                                                         = 799,
    /// HEAP-USAGES
    HeapUsages                                                             = 1598,
    /// HOP-COUNT
    HopCount                                                               = 2962,
    /// HOP-LIMIT
    HopLimit                                                               = 2232,
    /// HOST-ID
    HostId                                                                 = 2328,
    /// HOST-REF
    HostRef                                                                = 3112,
    /// HOST-SYSTEM-REF
    HostSystemRef                                                          = 5671,
    /// HTTP-ACCEPT-ENCODING
    HttpAcceptEncoding                                                     = 5315,
    /// HTTP-TP
    HttpTp                                                                 = 3394,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 1164,
    /// HW-ATTRIBUTE-DEF-REF
    HwAttributeDefRef                                                      = 3557,
    /// HW-ATTRIBUTE-DEFS
    HwAttributeDefs                                                        = 1221,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 125,
    /// HW-ATTRIBUTE-LITERALS
    HwAttributeLiterals                                                    = 4664,
    /// HW-ATTRIBUTE-VALUE
    HwAttributeValue                                                       = 3030,
    /// HW-ATTRIBUTE-VALUES
    HwAttributeValues                                                      = 3298,
    /// HW-CATEGORY
    HwCategory                                                             = 5158,
    /// HW-CATEGORY-REF
    HwCategoryRef                                                          = 5447,
    /// HW-CATEGORY-REFS
    HwCategoryRefs                                                         = 2576,
    /// HW-COMMUNICATION-CONTROLLER-REF
    HwCommunicationControllerRef                                           = 5149,
    /// HW-COMMUNICATION-PORT-REF
    HwCommunicationPortRef                                                 = 5167,
    /// HW-ELEMENT
    HwElement                                                              = 3561,
    /// HW-ELEMENT-CONNECTIONS
    HwElementConnections                                                   = 2059,
    /// HW-ELEMENT-CONNECTOR
    HwElementConnector                                                     = 3861,
    /// HW-ELEMENT-REF
    HwElementRef                                                           = 4420,
    /// HW-ELEMENT-REF-CONDITIONAL
    HwElementRefConditional                                                = 3427,
    /// HW-ELEMENT-REFS
    HwElementRefs                                                          = 1590,
    /// HW-PIN
    HwPin                                                                  = 3218,
    /// HW-PIN-CONNECTIONS
    HwPinConnections                                                       = 4040,
    /// HW-PIN-CONNECTOR
    HwPinConnector                                                         = 4599,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 4853,
    /// HW-PIN-GROUP-CONNECTIONS
    HwPinGroupConnections                                                  = 4003,
    /// HW-PIN-GROUP-CONNECTOR
    HwPinGroupConnector                                                    = 4439,
    /// HW-PIN-GROUP-CONTENT
    HwPinGroupContent                                                      = 1007,
    /// HW-PIN-GROUP-REF
    HwPinGroupRef                                                          = 5836,
    /// HW-PIN-GROUP-REFS
    HwPinGroupRefs                                                         = 1856,
    /// HW-PIN-GROUPS
    HwPinGroups                                                            = 989,
    /// HW-PIN-REF
    HwPinRef                                                               = 1081,
    /// HW-PIN-REFS
    HwPinRefs                                                              = 4046,
    /// HW-PORT-MAPPING
    HwPortMapping                                                          = 645,
    /// HW-PORT-MAPPINGS
    HwPortMappings                                                         = 5267,
    /// HW-TYPE
    HwType                                                                 = 728,
    /// HW-TYPE-REF
    HwTypeRef                                                              = 4515,
    /// I-PDU-IDENTIFIER-TCP-REF
    IPduIdentifierTcpRef                                                   = 1803,
    /// I-PDU-IDENTIFIER-TCP-REFS
    IPduIdentifierTcpRefs                                                  = 3933,
    /// I-PDU-IDENTIFIER-UDP-REF
    IPduIdentifierUdpRef                                                   = 1239,
    /// I-PDU-IDENTIFIER-UDP-REFS
    IPduIdentifierUdpRefs                                                  = 5112,
    /// I-PDU-IDENTIFIERS
    IPduIdentifiers                                                        = 1947,
    /// I-PDU-MAPPING
    IPduMapping                                                            = 1703,
    /// I-PDU-MAPPINGS
    IPduMappings                                                           = 4869,
    /// I-PDU-PORT
    IPduPort                                                               = 5777,
    /// I-PDU-PORT-REF
    IPduPortRef                                                            = 5037,
    /// I-PDU-PORT-REFS
    IPduPortRefs                                                           = 3509,
    /// I-PDU-REF
    IPduRef                                                                = 3390,
    /// I-PDU-SIGNAL-PROCESSING
    IPduSignalProcessing                                                   = 2048,
    /// I-PDU-TIMING
    IPduTiming                                                             = 3200,
    /// I-PDU-TIMING-SPECIFICATIONS
    IPduTimingSpecifications                                               = 5575,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 5227,
    /// I-PDU-TRIGGERING-REF
    IPduTriggeringRef                                                      = 5465,
    /// I-PDU-TRIGGERING-REF-CONDITIONAL
    IPduTriggeringRefConditional                                           = 3134,
    /// I-PDU-TRIGGERINGS
    IPduTriggerings                                                        = 3324,
    /// I-PDUS
    IPdus                                                                  = 408,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 3930,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 4348,
    /// I-SIGNAL
    ISignal                                                                = 5283,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 3753,
    /// I-SIGNAL-GROUP-REF
    ISignalGroupRef                                                        = 4230,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 3774,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 2186,
    /// I-SIGNAL-I-PDU-REF
    ISignalIPduRef                                                         = 495,
    /// I-SIGNAL-I-PDU-REF-CONDITIONAL
    ISignalIPduRefConditional                                              = 5284,
    /// I-SIGNAL-I-PDUS
    ISignalIPdus                                                           = 1496,
    /// I-SIGNAL-IN-I-PDU-REF
    ISignalInIPduRef                                                       = 3037,
    /// I-SIGNAL-MAPPING
    ISignalMapping                                                         = 5209,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 5823,
    /// I-SIGNAL-PORT-REF
    ISignalPortRef                                                         = 1733,
    /// I-SIGNAL-PORT-REFS
    ISignalPortRefs                                                        = 5548,
    /// I-SIGNAL-PROPS
    ISignalProps                                                           = 2854,
    /// I-SIGNAL-REF
    ISignalRef                                                             = 239,
    /// I-SIGNAL-REFS
    ISignalRefs                                                            = 4317,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 36,
    /// I-SIGNAL-TO-I-PDU-MAPPINGS
    ISignalToIPduMappings                                                  = 1141,
    /// I-SIGNAL-TO-PDU-MAPPINGS
    ISignalToPduMappings                                                   = 5217,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 3168,
    /// I-SIGNAL-TRIGGERING-REF
    ISignalTriggeringRef                                                   = 5571,
    /// I-SIGNAL-TRIGGERING-REF-CONDITIONAL
    ISignalTriggeringRefConditional                                        = 3067,
    /// I-SIGNAL-TRIGGERINGS
    ISignalTriggerings                                                     = 4579,
    /// I-SIGNAL-TYPE
    ISignalType                                                            = 195,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 3243,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF
    IamRelevantTlsSecureComPropsRef                                        = 4710,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS
    IamRelevantTlsSecureComPropsRefs                                       = 3024,
    /// ICMP-RULE
    IcmpRule                                                               = 142,
    /// ICMP-V-4-PROPS
    IcmpV4Props                                                            = 4991,
    /// ICMP-V-6-PROPS
    IcmpV6Props                                                            = 4184,
    /// ICV-SECURED
    IcvSecured                                                             = 3712,
    /// ICV-VERIFICATION
    IcvVerification                                                        = 23,
    /// ID
    Id                                                                     = 4808,
    /// IDENT
    Ident                                                                  = 1639,
    /// IDENTICAL-MAPPING
    IdenticalMapping                                                       = 5449,
    /// IDENTIFIABLE-REF
    IdentifiableRef                                                        = 4214,
    /// IDENTIFIER
    Identifier                                                             = 2050,
    /// IDENTITIY-NUMBER
    IdentitiyNumber                                                        = 659,
    /// IDENTITY
    Identity                                                               = 5609,
    /// IDENTITY-CERTIFICATE-AUTHORITY-REF
    IdentityCertificateAuthorityRef                                        = 4928,
    /// IDENTITY-REF
    IdentityRef                                                            = 24,
    /// IDLE-SLOPE
    IdleSlope                                                              = 2088,
    /// IDS-COMMON-ELEMENT-REF
    IdsCommonElementRef                                                    = 1209,
    /// IDS-COMMON-ELEMENT-REF-CONDITIONAL
    IdsCommonElementRefConditional                                         = 1646,
    /// IDS-DESIGN
    IdsDesign                                                              = 2323,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 2085,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 5028,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 2509,
    /// IDSM-INSTANCE-ID
    IdsmInstanceId                                                         = 5251,
    /// IDSM-INSTANCE-REF
    IdsmInstanceRef                                                        = 2203,
    /// IDSM-INSTANCE-REF-CONDITIONAL
    IdsmInstanceRefConditional                                             = 2102,
    /// IDSM-INSTANCES
    IdsmInstances                                                          = 4362,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 3422,
    /// IDSM-MODULE-INSTANTIATION-REF
    IdsmModuleInstantiationRef                                             = 5138,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 492,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 818,
    /// IDSM-RATE-LIMITATION-REF
    IdsmRateLimitationRef                                                  = 3183,
    /// IDSM-RATE-LIMITATION-REF-CONDITIONAL
    IdsmRateLimitationRefConditional                                       = 5115,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 930,
    /// IDSM-TRAFFIC-LIMITATION-REF
    IdsmTrafficLimitationRef                                               = 3730,
    /// IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL
    IdsmTrafficLimitationRefConditional                                    = 1066,
    /// IE
    Ie                                                                     = 1752,
    /// IEEE-1722-TP
    Ieee1722Tp                                                             = 3293,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 1077,
    /// IGNORE-AFTER-TX
    IgnoreAfterTx                                                          = 4032,
    /// IGNORE-ORDER-ALLOWED
    IgnoreOrderAllowed                                                     = 2969,
    /// IKE-AUTHENTICATION-METHOD
    IkeAuthenticationMethod                                                = 1950,
    /// IKE-CIPHER-SUITE-NAME
    IkeCipherSuiteName                                                     = 5427,
    /// IKE-OVER-TIME
    IkeOverTime                                                            = 641,
    /// IKE-RAND-TIME
    IkeRandTime                                                            = 359,
    /// IKE-REAUTH-TIME
    IkeReauthTime                                                          = 3454,
    /// IKE-REKEY-TIME
    IkeRekeyTime                                                           = 302,
    /// IMMEDIATE-NV-DATA-STORAGE
    ImmediateNvDataStorage                                                 = 203,
    /// IMMEDIATE-RESUME-TIME
    ImmediateResumeTime                                                    = 5536,
    /// IMPL-CONSTANT-REF
    ImplConstantRef                                                        = 1993,
    /// IMPL-INIT-VALUE
    ImplInitValue                                                          = 3635,
    /// IMPLEMENTATION-ARRAY-ELEMENT-REF
    ImplementationArrayElementRef                                          = 4105,
    /// IMPLEMENTATION-CONFIG-CLASSES
    ImplementationConfigClasses                                            = 2292,
    /// IMPLEMENTATION-CONFIG-VARIANT
    ImplementationConfigVariant                                            = 2898,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 4857,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 4425,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 2121,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF
    ImplementationDataTypeElementInAutosarDataPrototypeRef                 = 2124,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF
    ImplementationDataTypeElementInPortInterfaceRef                        = 617,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF
    ImplementationDataTypeElementInSystemRef                               = 4825,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    ImplementationDataTypeElementRef                                       = 4341,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 2208,
    /// IMPLEMENTATION-DATA-TYPE-REF
    ImplementationDataTypeRef                                              = 2610,
    /// IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF
    ImplementationDataTypeSubElementRef                                    = 1466,
    /// IMPLEMENTATION-PROPSS
    ImplementationPropss                                                   = 5941,
    /// IMPLEMENTATION-RECORD-ELEMENT-REF
    ImplementationRecordElementRef                                         = 4774,
    /// IMPLEMENTATION-REF
    ImplementationRef                                                      = 3605,
    /// IMPLEMENTATION-REFS
    ImplementationRefs                                                     = 4222,
    /// IMPLEMENTED-ENTRY-REF
    ImplementedEntryRef                                                    = 4804,
    /// IMPLEMENTED-ENTRYS
    ImplementedEntrys                                                      = 2583,
    /// IMPLEMENTED-IN-REF
    ImplementedInRef                                                       = 1324,
    /// IMPLEMENTS-LEGACY-STRING-SERIALIZATION
    ImplementsLegacyStringSerialization                                    = 1138,
    /// IMPLEMENTS-SOMEIP-STRING-HANDLING
    ImplementsSomeipStringHandling                                         = 5605,
    /// IMPLICIT-DATA-ACCESS-IREF
    ImplicitDataAccessIref                                                 = 2638,
    /// IMPLICIT-DATA-ACCESS-IREFS
    ImplicitDataAccessIrefs                                                = 1721,
    /// IMPLICIT-INTER-RUNNABLE-VARIABLES
    ImplicitInterRunnableVariables                                         = 3468,
    /// IMPLIED-OPERATION-REF
    ImpliedOperationRef                                                    = 1539,
    /// IMPLIED-OPERATION-REFS
    ImpliedOperationRefs                                                   = 1415,
    /// IMPORTANCE
    Importance                                                             = 1258,
    /// IN-MEASUREMENT-SET
    InMeasurementSet                                                       = 1396,
    /// IN-PLACE
    InPlace                                                                = 2420,
    /// IN-SCOPE
    InScope                                                                = 2745,
    /// INCLUDE-REF
    IncludeRef                                                             = 1359,
    /// INCLUDE-REFS
    IncludeRefs                                                            = 126,
    /// INCLUDED-DATA-TYPE-SET
    IncludedDataTypeSet                                                    = 5191,
    /// INCLUDED-DATA-TYPE-SETS
    IncludedDataTypeSets                                                   = 3992,
    /// INCLUDED-LIBRARY-REF
    IncludedLibraryRef                                                     = 2923,
    /// INCLUDED-LIBRARY-REFS
    IncludedLibraryRefs                                                    = 5900,
    /// INCLUDED-MODE-DECLARATION-GROUP-SET
    IncludedModeDeclarationGroupSet                                        = 3107,
    /// INCLUDED-MODE-DECLARATION-GROUP-SETS
    IncludedModeDeclarationGroupSets                                       = 714,
    /// INCLUDED-VARIANT-REF
    IncludedVariantRef                                                     = 5832,
    /// INCLUDED-VARIANT-REFS
    IncludedVariantRefs                                                    = 217,
    /// INDENT-SAMPLE
    IndentSample                                                           = 597,
    /// INDEX
    Index                                                                  = 1006,
    /// INDEX-DATA-TYPE-REF
    IndexDataTypeRef                                                       = 880,
    /// INDEXED-ARRAY-ELEMENT
    IndexedArrayElement                                                    = 1656,
    /// INDICATIONS
    Indications                                                            = 5526,
    /// INDICATOR-FAILURE-CYCLE-COUNTER-THRESHOLD
    IndicatorFailureCycleCounterThreshold                                  = 2052,
    /// INDICATOR-REF
    IndicatorRef                                                           = 2130,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 542,
    /// INDIRECT-API
    IndirectApi                                                            = 4762,
    /// INDUSTRY-GROUP
    IndustryGroup                                                          = 2109,
    /// INFO-TYPE
    InfoType                                                               = 4495,
    /// INFO-TYPE-REF
    InfoTypeRef                                                            = 821,
    /// INFORMAL-FORMULA
    InformalFormula                                                        = 5546,
    /// INFRASTRUCTURE-SERVICES
    InfrastructureServices                                                 = 5018,
    /// INGRESS-PRIORITY
    IngressPriority                                                        = 5260,
    /// INHIBIT-SOURCES
    InhibitSources                                                         = 2093,
    /// INHIBITING-FID-REF
    InhibitingFidRef                                                       = 1132,
    /// INHIBITING-SECONDARY-FID-REF
    InhibitingSecondaryFidRef                                              = 1679,
    /// INHIBITING-SECONDARY-FID-REFS
    InhibitingSecondaryFidRefs                                             = 589,
    /// INHIBITION-MASK
    InhibitionMask                                                         = 1997,
    /// INHIBITION-SOURCE-REF
    InhibitionSourceRef                                                    = 2513,
    /// INIT-EVENT
    InitEvent                                                              = 4283,
    /// INIT-RESOURCE
    InitResource                                                           = 361,
    /// INIT-VALUE
    InitValue                                                              = 2763,
    /// INIT-VALUES
    InitValues                                                             = 2231,
    /// INITIAL-CHECKPOINT-REF
    InitialCheckpointRef                                                   = 1627,
    /// INITIAL-CHECKPOINT-REFS
    InitialCheckpointRefs                                                  = 3076,
    /// INITIAL-DELAY-MAX-VALUE
    InitialDelayMaxValue                                                   = 560,
    /// INITIAL-DELAY-MIN-VALUE
    InitialDelayMinValue                                                   = 266,
    /// INITIAL-DYNAMIC-PART
    InitialDynamicPart                                                     = 3836,
    /// INITIAL-EVENT-STATUS
    InitialEventStatus                                                     = 3590,
    /// INITIAL-FIND-BEHAVIOR
    InitialFindBehavior                                                    = 4135,
    /// INITIAL-INACTIVITY-TIME
    InitialInactivityTime                                                  = 3188,
    /// INITIAL-MODE-REF
    InitialModeRef                                                         = 4300,
    /// INITIAL-NAD
    InitialNad                                                             = 3322,
    /// INITIAL-OFFER-BEHAVIOR
    InitialOfferBehavior                                                   = 5926,
    /// INITIAL-REF-OFFSET
    InitialRefOffset                                                       = 1684,
    /// INITIAL-REPETITIONS-BASE-DELAY
    InitialRepetitionsBaseDelay                                            = 1307,
    /// INITIAL-REPETITIONS-MAX
    InitialRepetitionsMax                                                  = 591,
    /// INITIAL-STATUS
    InitialStatus                                                          = 4292,
    /// INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    InitialVehicleAnnouncementTime                                         = 4354,
    /// INITIALIZATION-VECTOR-LENGTH
    InitializationVectorLength                                             = 4074,
    /// INITIALIZED-PARAMETER-REF
    InitializedParameterRef                                                = 2281,
    /// INNER-GROUP-IREF
    InnerGroupIref                                                         = 2714,
    /// INNER-GROUP-IREFS
    InnerGroupIrefs                                                        = 2998,
    /// INNER-PORT-IREF
    InnerPortIref                                                          = 1211,
    /// INPLACE
    Inplace                                                                = 1049,
    /// INPUT
    Input                                                                  = 1229,
    /// INPUT-DATAS
    InputDatas                                                             = 2429,
    /// INPUT-VARIABLE-TYPE-REF
    InputVariableTypeRef                                                   = 2601,
    /// INSTALLATION-BEHAVIOR
    InstallationBehavior                                                   = 4736,
    /// INSTANCE-ID
    InstanceId                                                             = 5341,
    /// INSTANCE-ID-PROPS
    InstanceIdProps                                                        = 1890,
    /// INSTANCE-IDENTIFIER
    InstanceIdentifier                                                     = 3737,
    /// INSTANCE-IDS
    InstanceIds                                                            = 2080,
    /// INSTANCE-IN-MEMORY
    InstanceInMemory                                                       = 389,
    /// INSTANCES
    Instances                                                              = 2659,
    /// INSTANTIATION-DATA-DEF-PROPS
    InstantiationDataDefProps                                              = 3736,
    /// INSTANTIATION-DATA-DEF-PROPSS
    InstantiationDataDefPropss                                             = 4080,
    /// INSTANTIATION-RTE-EVENT-PROPSS
    InstantiationRteEventPropss                                            = 819,
    /// INSTANTIATION-TIMING-EVENT-PROPS
    InstantiationTimingEventProps                                          = 5239,
    /// INTEGER-VALUE-VARIATION-POINT
    IntegerValueVariationPoint                                             = 2672,
    /// INTENDED-FILENAME
    IntendedFilename                                                       = 4673,
    /// INTENDED-PARTIAL-INITIALIZATION-COUNT
    IntendedPartialInitializationCount                                     = 5108,
    /// INTENDED-TARGET-MACHINE-REF
    IntendedTargetMachineRef                                               = 3986,
    /// INTENDED-TARGET-MACHINE-REFS
    IntendedTargetMachineRefs                                              = 155,
    /// INTER-MESSAGE-TIME
    InterMessageTime                                                       = 4471,
    /// INTERFACE-BLUEPRINT-MAPPING
    InterfaceBlueprintMapping                                              = 3968,
    /// INTERFACE-BLUEPRINT-MAPPINGS
    InterfaceBlueprintMappings                                             = 4679,
    /// INTERFACE-BLUEPRINT-REF
    InterfaceBlueprintRef                                                  = 2596,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 5294,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 2155,
    /// INTERFACE-MAPPINGS
    InterfaceMappings                                                      = 2720,
    /// INTERFACE-REF
    InterfaceRef                                                           = 380,
    /// INTERFACE-VERSION
    InterfaceVersion                                                       = 1075,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 4735,
    /// INTERNAL-BEHAVIORS
    InternalBehaviors                                                      = 3652,
    /// INTERNAL-CONSTRS
    InternalConstrs                                                        = 4646,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 4017,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 5859,
    /// INTERNAL-TRIGGERING-POINT-POLICYS
    InternalTriggeringPointPolicys                                         = 486,
    /// INTERNAL-TRIGGERING-POINTS
    InternalTriggeringPoints                                               = 4844,
    /// INTERNET-HEADER-LENGTH
    InternetHeaderLength                                                   = 3692,
    /// INTERPOLATION-ROUTINE
    InterpolationRoutine                                                   = 1421,
    /// INTERPOLATION-ROUTINE-MAPPING
    InterpolationRoutineMapping                                            = 1011,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 4476,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REF
    InterpolationRoutineMappingSetRef                                      = 133,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REFS
    InterpolationRoutineMappingSetRefs                                     = 587,
    /// INTERPOLATION-ROUTINE-MAPPINGS
    InterpolationRoutineMappings                                           = 4577,
    /// INTERPOLATION-ROUTINE-REF
    InterpolationRoutineRef                                                = 4583,
    /// INTERPOLATION-ROUTINES
    InterpolationRoutines                                                  = 999,
    /// INTERRUPT-CATEGORY
    InterruptCategory                                                      = 973,
    /// INTERRUPT-SOURCE
    InterruptSource                                                        = 2830,
    /// INTERVAL-LENGTH
    IntervalLength                                                         = 427,
    /// INTRODUCTION
    Introduction                                                           = 4666,
    /// INVALID-VALUE
    InvalidValue                                                           = 3824,
    /// INVALIDATION-POLICY
    InvalidationPolicy                                                     = 2662,
    /// INVALIDATION-POLICYS
    InvalidationPolicys                                                    = 1419,
    /// INVERSE-PROTOCOL-VERSION
    InverseProtocolVersion                                                 = 4921,
    /// INVERT
    Invert                                                                 = 2632,
    /// INVERT-CONDITION
    InvertCondition                                                        = 5090,
    /// INVOCATION
    Invocation                                                             = 5206,
    /// IO-CONTROL-CLASS-REF
    IoControlClassRef                                                      = 5684,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATION
    IoHwAbstractionServerAnnotation                                        = 77,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATIONS
    IoHwAbstractionServerAnnotations                                       = 311,
    /// IP-ADDRESS
    IpAddress                                                              = 2260,
    /// IP-ADDRESS-KEEP-BEHAVIOR
    IpAddressKeepBehavior                                                  = 2195,
    /// IP-ADDRESS-PREFIX-LENGTH
    IpAddressPrefixLength                                                  = 64,
    /// IP-IAM-AUTHENTIC-CONNECTION-PROPS
    IpIamAuthenticConnectionProps                                          = 1328,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 1073,
    /// IP-PROTOCOL
    IpProtocol                                                             = 1213,
    /// IP-SEC-CONFIG
    IpSecConfig                                                            = 3719,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 248,
    /// IP-SEC-CONFIG-PROPS-REF
    IpSecConfigPropsRef                                                    = 1902,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 1951,
    /// IP-SEC-RULE
    IpSecRule                                                              = 985,
    /// IP-SEC-RULES
    IpSecRules                                                             = 2909,
    /// IP-V-4-ADDRESS
    IpV4Address                                                            = 712,
    /// IP-V-6-ADDRESS
    IpV6Address                                                            = 2055,
    /// IP-V-6-PATH-MTU-ENABLED
    IpV6PathMtuEnabled                                                     = 5880,
    /// IP-V-6-PATH-MTU-TIMEOUT
    IpV6PathMtuTimeout                                                     = 33,
    /// IPV-4-ADDRESS
    Ipv4Address                                                            = 3094,
    /// IPV-4-ADDRESS-SOURCE
    Ipv4AddressSource                                                      = 2806,
    /// IPV-4-CONFIGURATION
    Ipv4Configuration                                                      = 5593,
    /// IPV-4-DHCP-SERVER-CONFIGURATION
    Ipv4DhcpServerConfiguration                                            = 3177,
    /// IPV-4-MULTICAST-IP-ADDRESS
    Ipv4MulticastIpAddress                                                 = 2471,
    /// IPV-4-PROPS
    Ipv4Props                                                              = 1710,
    /// IPV-4-RULE
    Ipv4Rule                                                               = 1030,
    /// IPV-6-ADDRESS
    Ipv6Address                                                            = 2067,
    /// IPV-6-ADDRESS-SOURCE
    Ipv6AddressSource                                                      = 1971,
    /// IPV-6-CONFIGURATION
    Ipv6Configuration                                                      = 1350,
    /// IPV-6-DHCP-SERVER-CONFIGURATION
    Ipv6DhcpServerConfiguration                                            = 2457,
    /// IPV-6-MULTICAST-IP-ADDRESS
    Ipv6MulticastIpAddress                                                 = 2808,
    /// IPV-6-PROPS
    Ipv6Props                                                              = 200,
    /// IPV-6-RULE
    Ipv6Rule                                                               = 3313,
    /// IS-ACTIVATION-LINE-DEPENDENT
    IsActivationLineDependent                                              = 3655,
    /// IS-AUTO-VALUE
    IsAutoValue                                                            = 4273,
    /// IS-DEFAULT
    IsDefault                                                              = 3201,
    /// IS-DELTA-PACKAGE
    IsDeltaPackage                                                         = 696,
    /// IS-DYNAMIC-LENGTH-FIELD-SIZE
    IsDynamicLengthFieldSize                                               = 1815,
    /// IS-EVENT
    IsEvent                                                                = 2887,
    /// IS-EXTERNAL
    IsExternal                                                             = 4838,
    /// IS-GLOBAL
    IsGlobal                                                               = 3887,
    /// IS-MANDATORY
    IsMandatory                                                            = 5296,
    /// IS-OBD-RELEVANT
    IsObdRelevant                                                          = 172,
    /// IS-OPTIONAL
    IsOptional                                                             = 3845,
    /// IS-PIPELINING-PERMITTED
    IsPipeliningPermitted                                                  = 1615,
    /// IS-PRIVATE
    IsPrivate                                                              = 903,
    /// IS-REENTRANT
    IsReentrant                                                            = 5778,
    /// IS-REQUIRED
    IsRequired                                                             = 3223,
    /// IS-SERVICE
    IsService                                                              = 4555,
    /// IS-STRUCT-WITH-OPTIONAL-ELEMENT
    IsStructWithOptionalElement                                            = 1644,
    /// IS-SYNCHRONOUS
    IsSynchronous                                                          = 3388,
    /// IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER
    IsSystemWideGlobalTimeMaster                                           = 3491,
    /// IS-UNUSED
    IsUnused                                                               = 1361,
    /// IS-VARIADIC-TEMPLATE
    IsVariadicTemplate                                                     = 2384,
    /// ISSUED-BY
    IssuedBy                                                               = 1196,
    /// ISSUED-TRIGGERS
    IssuedTriggers                                                         = 2929,
    /// ITEM
    Item                                                                   = 3850,
    /// ITEM-DEFINITIONS
    ItemDefinitions                                                        = 5248,
    /// ITEM-LABEL
    ItemLabel                                                              = 4705,
    /// ITEMS
    Items                                                                  = 1150,
    /// IUMPR-GROUP
    IumprGroup                                                             = 4039,
    /// IUMPR-GROUP-IDENTIFIERS
    IumprGroupIdentifiers                                                  = 1792,
    /// IUMPR-REF
    IumprRef                                                               = 238,
    /// IUMPR-REFS
    IumprRefs                                                              = 1331,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 3349,
    /// J-1939-CLUSTER-CONDITIONAL
    J1939ClusterConditional                                                = 2922,
    /// J-1939-CLUSTER-VARIANTS
    J1939ClusterVariants                                                   = 323,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 320,
    /// J-1939-CONTROLLER-APPLICATION-REF
    J1939ControllerApplicationRef                                          = 924,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING
    J1939ControllerApplicationToJ1939NmNodeMapping                         = 437,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS
    J1939ControllerApplicationToJ1939NmNodeMappings                        = 4164,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 2369,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 996,
    /// J-1939-DTC-VALUE
    J1939DtcValue                                                          = 3543,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 3782,
    /// J-1939-NM-ECU
    J1939NmEcu                                                             = 1139,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 5183,
    /// J-1939-NM-NODE-REF
    J1939NmNodeRef                                                         = 4272,
    /// J-1939-REQUESTABLE
    J1939Requestable                                                       = 5309,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 1635,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 2270,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 2099,
    /// J-1939-SHARED-ADDRESS-CLUSTERS
    J1939SharedAddressClusters                                             = 706,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 5562,
    /// J-1939-TP-CONNECTION
    J1939TpConnection                                                      = 4926,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 3698,
    /// J-1939-TP-PG
    J1939TpPg                                                              = 2624,
    /// JITTER
    Jitter                                                                 = 2000,
    /// JOB-PRIMITIVE-REF
    JobPrimitiveRef                                                        = 5818,
    /// JOB-REF
    JobRef                                                                 = 5002,
    /// JOB-REQUIREMENTS
    JobRequirements                                                        = 1527,
    /// JUMP-TO-BOOT-LOADER
    JumpToBootLoader                                                       = 4890,
    /// K-INIT-RESOURCE
    KInitResource                                                          = 1443,
    /// K-RUN-RESOURCE
    KRunResource                                                           = 2414,
    /// KEEP-ALIVE-INTERVAL
    KeepAliveInterval                                                      = 1169,
    /// KEEP-ALIVE-PROBES-MAX
    KeepAliveProbesMax                                                     = 2398,
    /// KEEP-ALIVE-TIME
    KeepAliveTime                                                          = 2024,
    /// KEEP-ALIVES
    KeepAlives                                                             = 3445,
    /// KEY
    Key                                                                    = 3871,
    /// KEY-EXCHANGE-AUTHENTICATION-REF
    KeyExchangeAuthenticationRef                                           = 5298,
    /// KEY-EXCHANGE-AUTHENTICATION-REFS
    KeyExchangeAuthenticationRefs                                          = 1873,
    /// KEY-EXCHANGE-REF
    KeyExchangeRef                                                         = 1109,
    /// KEY-EXCHANGE-REFS
    KeyExchangeRefs                                                        = 832,
    /// KEY-GENERATION
    KeyGeneration                                                          = 503,
    /// KEY-ID
    KeyId                                                                  = 1186,
    /// KEY-IDENTITY-REF
    KeyIdentityRef                                                         = 3613,
    /// KEY-REF
    KeyRef                                                                 = 1432,
    /// KEY-SERVER-PRIORITY
    KeyServerPriority                                                      = 1886,
    /// KEY-SIZE
    KeySize                                                                = 205,
    /// KEY-SLOT-ALLOWED-MODIFICATION
    KeySlotAllowedModification                                             = 1787,
    /// KEY-SLOT-CONTENT-ALLOWED-USAGES
    KeySlotContentAllowedUsages                                            = 4995,
    /// KEY-SLOT-ID
    KeySlotId                                                              = 2851,
    /// KEY-SLOT-ONLY-ENABLED
    KeySlotOnlyEnabled                                                     = 2480,
    /// KEY-SLOT-REF
    KeySlotRef                                                             = 1018,
    /// KEY-SLOT-USAGE
    KeySlotUsage                                                           = 849,
    /// KEY-SLOT-USED-FOR-START-UP
    KeySlotUsedForStartUp                                                  = 1374,
    /// KEY-SLOT-USED-FOR-SYNC
    KeySlotUsedForSync                                                     = 5423,
    /// KEY-SLOTS
    KeySlots                                                               = 14,
    /// KEY-STORAGE-TYPE
    KeyStorageType                                                         = 4837,
    /// KEY-VALUE-PAIRS
    KeyValuePairs                                                          = 4290,
    /// KEY-VALUE-STORAGE-REF
    KeyValueStorageRef                                                     = 303,
    /// KEYWORD
    Keyword                                                                = 1085,
    /// KEYWORD-SET
    KeywordSet                                                             = 5739,
    /// KEYWORDS
    Keywords                                                               = 1172,
    /// KIND
    Kind                                                                   = 309,
    /// L-1
    L1                                                                     = 5087,
    /// L-10
    L10                                                                    = 893,
    /// L-2
    L2                                                                     = 1706,
    /// L-4
    L4                                                                     = 860,
    /// L-5
    L5                                                                     = 3179,
    /// L-GRAPHIC
    LGraphic                                                               = 3296,
    /// LABEL
    Label                                                                  = 1633,
    /// LABEL-1
    Label1                                                                 = 3269,
    /// LABELED-ITEM
    LabeledItem                                                            = 3398,
    /// LABELED-LIST
    LabeledList                                                            = 4432,
    /// LANGUAGE
    Language                                                               = 2402,
    /// LAST-EGRESS-SCHEDULER-REF
    LastEgressSchedulerRef                                                 = 1236,
    /// LATENCY-CONSTRAINT-TYPE
    LatencyConstraintType                                                  = 4119,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 1107,
    /// LATEST-TX
    LatestTx                                                               = 4917,
    /// LC-OBJECT-REF
    LcObjectRef                                                            = 5631,
    /// LC-STATE-REF
    LcStateRef                                                             = 5602,
    /// LC-STATES
    LcStates                                                               = 5796,
    /// LDAP-URL
    LdapUrl                                                                = 5674,
    /// LEAF-ELEMENT-IREF
    LeafElementIref                                                        = 123,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS
    LegislatedFreezeFrameContentUdsObds                                    = 1838,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS
    LegislatedFreezeFrameContentWwhObds                                    = 3407,
    /// LENGTH
    Length                                                                 = 895,
    /// LENGTH-CLIENT-ID
    LengthClientId                                                         = 3559,
    /// LENGTH-EXP
    LengthExp                                                              = 1183,
    /// LENGTH-SEQUENCE-COUNTER
    LengthSequenceCounter                                                  = 505,
    /// LENGTH-VERIFICATION
    LengthVerification                                                     = 4971,
    /// LET-INTERVAL-REF
    LetIntervalRef                                                         = 481,
    /// LET-INTERVAL-REFS
    LetIntervalRefs                                                        = 1857,
    /// LICENSE-REF
    LicenseRef                                                             = 5155,
    /// LICENSE-REFS
    LicenseRefs                                                            = 4930,
    /// LIFE-CYCLE-INFO
    LifeCycleInfo                                                          = 5182,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 3449,
    /// LIFE-CYCLE-INFOS
    LifeCycleInfos                                                         = 2381,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 2917,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 4332,
    /// LIMIT
    Limit                                                                  = 2669,
    /// LIMIT-KIND
    LimitKind                                                              = 583,
    /// LIN-CHECKSUM
    LinChecksum                                                            = 2730,
    /// LIN-CLUSTER
    LinCluster                                                             = 2338,
    /// LIN-CLUSTER-CONDITIONAL
    LinClusterConditional                                                  = 2955,
    /// LIN-CLUSTER-VARIANTS
    LinClusterVariants                                                     = 891,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 1304,
    /// LIN-CONFIGURABLE-FRAME
    LinConfigurableFrame                                                   = 3645,
    /// LIN-CONFIGURABLE-FRAMES
    LinConfigurableFrames                                                  = 5234,
    /// LIN-ERROR-RESPONSE
    LinErrorResponse                                                       = 2255,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 5691,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 4045,
    /// LIN-MASTER
    LinMaster                                                              = 2989,
    /// LIN-MASTER-CONDITIONAL
    LinMasterConditional                                                   = 3577,
    /// LIN-MASTER-VARIANTS
    LinMasterVariants                                                      = 2283,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 1355,
    /// LIN-ORDERED-CONFIGURABLE-FRAME
    LinOrderedConfigurableFrame                                            = 4942,
    /// LIN-ORDERED-CONFIGURABLE-FRAMES
    LinOrderedConfigurableFrames                                           = 5043,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 1251,
    /// LIN-PID-TO-CAN-ID-MAPPINGS
    LinPidToCanIdMappings                                                  = 2848,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 4557,
    /// LIN-SLAVE
    LinSlave                                                               = 3331,
    /// LIN-SLAVE-CONDITIONAL
    LinSlaveConditional                                                    = 325,
    /// LIN-SLAVE-CONFIG
    LinSlaveConfig                                                         = 5615,
    /// LIN-SLAVE-ECU-REF
    LinSlaveEcuRef                                                         = 2455,
    /// LIN-SLAVE-VARIANTS
    LinSlaveVariants                                                       = 2250,
    /// LIN-SLAVES
    LinSlaves                                                              = 3576,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 3320,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 2217,
    /// LIN-TP-CONNECTION
    LinTpConnection                                                        = 4201,
    /// LIN-TP-N-SDU-REF
    LinTpNSduRef                                                           = 5646,
    /// LIN-TP-NODE
    LinTpNode                                                              = 5534,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 5333,
    /// LIN-UNCONDITIONAL-FRAME-REF
    LinUnconditionalFrameRef                                               = 1650,
    /// LIN-UNCONDITIONAL-FRAME-REFS
    LinUnconditionalFrameRefs                                              = 4064,
    /// LINKER
    Linker                                                                 = 4709,
    /// LINKERS
    Linkers                                                                = 199,
    /// LIST
    List                                                                   = 3739,
    /// LISTEN-NOISE
    ListenNoise                                                            = 1723,
    /// LISTEN-TIMEOUT
    ListenTimeout                                                          = 5146,
    /// LITERAL-PREFIX
    LiteralPrefix                                                          = 2927,
    /// LITERALS
    Literals                                                               = 2212,
    /// LIVELINESS-PROTECTION-KIND
    LivelinessProtectionKind                                               = 1293,
    /// LOAD-BALANCING-PRIORITY
    LoadBalancingPriority                                                  = 1281,
    /// LOAD-BALANCING-WEIGHT
    LoadBalancingWeight                                                    = 5348,
    /// LOC-MEASUREMENT-SET
    LocMeasurementSet                                                      = 936,
    /// LOCAL-CERTIFICATE-REF
    LocalCertificateRef                                                    = 4245,
    /// LOCAL-CERTIFICATE-REFS
    LocalCertificateRefs                                                   = 5020,
    /// LOCAL-COM-ACCESS-CONTROL-ENABLED
    LocalComAccessControlEnabled                                           = 4747,
    /// LOCAL-COMM-CONNECTOR-REF
    LocalCommConnectorRef                                                  = 1058,
    /// LOCAL-DEBUG-DATAS
    LocalDebugDatas                                                        = 13,
    /// LOCAL-ENDPOINT-CONFIG
    LocalEndpointConfig                                                    = 5068,
    /// LOCAL-ID
    LocalId                                                                = 2016,
    /// LOCAL-IP-SEC-RULE-REF
    LocalIpSecRuleRef                                                      = 2035,
    /// LOCAL-IP-SEC-RULE-REFS
    LocalIpSecRuleRefs                                                     = 874,
    /// LOCAL-NETWORK-ENDPOINT-REF
    LocalNetworkEndpointRef                                                = 1767,
    /// LOCAL-PARAMETER-REF
    LocalParameterRef                                                      = 756,
    /// LOCAL-PORT-RANGE-END
    LocalPortRangeEnd                                                      = 2795,
    /// LOCAL-PORT-RANGE-START
    LocalPortRangeStart                                                    = 724,
    /// LOCAL-PORT-REF
    LocalPortRef                                                           = 3261,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 1190,
    /// LOCAL-SUPERVISION-ENTITY-REF
    LocalSupervisionEntityRef                                              = 3120,
    /// LOCAL-SUPERVISION-ENTITY-REFS
    LocalSupervisionEntityRefs                                             = 612,
    /// LOCAL-SUPERVISION-REF
    LocalSupervisionRef                                                    = 5913,
    /// LOCAL-SUPERVISION-REFS
    LocalSupervisionRefs                                                   = 3752,
    /// LOCAL-SUPERVISIONS
    LocalSupervisions                                                      = 4788,
    /// LOCAL-TCP-PORT-REF
    LocalTcpPortRef                                                        = 2485,
    /// LOCAL-UDP-PORT-REF
    LocalUdpPortRef                                                        = 733,
    /// LOCAL-UNICAST-ADDRESSS
    LocalUnicastAddresss                                                   = 1836,
    /// LOCAL-VARIABLE-REF
    LocalVariableRef                                                       = 1617,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 705,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 1893,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 5643,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF
    LogAndTraceMessageCollectionSetRef                                     = 1519,
    /// LOG-CHANNEL-ID
    LogChannelId                                                           = 4461,
    /// LOG-SINK-REF
    LogSinkRef                                                             = 395,
    /// LOG-SINK-REFS
    LogSinkRefs                                                            = 1327,
    /// LOG-TRACE-DEFAULT-LOG-LEVEL
    LogTraceDefaultLogLevel                                                = 2054,
    /// LOG-TRACE-DEFAULT-LOG-THRESHOLD
    LogTraceDefaultLogThreshold                                            = 3249,
    /// LOG-TRACE-FILE-PATH
    LogTraceFilePath                                                       = 2293,
    /// LOG-TRACE-LOG-MODE
    LogTraceLogMode                                                        = 3923,
    /// LOG-TRACE-LOG-MODES
    LogTraceLogModes                                                       = 5759,
    /// LOG-TRACE-PORT-ID
    LogTracePortId                                                         = 26,
    /// LOG-TRACE-PROCESS-DESC
    LogTraceProcessDesc                                                    = 1363,
    /// LOG-TRACE-PROCESS-ID
    LogTraceProcessId                                                      = 910,
    /// LOGGING-BEHAVIOR
    LoggingBehavior                                                        = 943,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 729,
    /// LOGIC-ADDRESSS
    LogicAddresss                                                          = 1204,
    /// LOGICAL-ADDRESS
    LogicalAddress                                                         = 3195,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 836,
    /// LOGICAL-EXPRESSION-ARGUMENT-REF
    LogicalExpressionArgumentRef                                           = 4361,
    /// LOGICAL-EXPRESSION-ARGUMENT-REFS
    LogicalExpressionArgumentRefs                                          = 2133,
    /// LOGICAL-EXPRESSIONS
    LogicalExpressions                                                     = 1983,
    /// LOGICAL-OPERATOR
    LogicalOperator                                                        = 5904,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 3822,
    /// LOGICAL-SUPERVISIONS
    LogicalSupervisions                                                    = 1076,
    /// LONG-NAME
    LongName                                                               = 5487,
    /// LONG-NAME-1
    LongName1                                                              = 3448,
    /// LOWER-BOUND
    LowerBound                                                             = 2206,
    /// LOWER-CAN-ID
    LowerCanId                                                             = 3709,
    /// LOWER-LIMIT
    LowerLimit                                                             = 2434,
    /// LOWER-MULTIPLICITY
    LowerMultiplicity                                                      = 2878,
    /// LOWER-REF
    LowerRef                                                               = 2536,
    /// LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING
    LtMessageCollectionToPortPrototypeMapping                              = 139,
    /// LUMINOUS-INTENSITY-EXP
    LuminousIntensityExp                                                   = 2314,
    /// M
    M                                                                      = 1235,
    /// MAC-LAYER-TYPE
    MacLayerType                                                           = 5132,
    /// MAC-MULTICAST-ADDRESS
    MacMulticastAddress                                                    = 5637,
    /// MAC-MULTICAST-ADDRESS-REF
    MacMulticastAddressRef                                                 = 3299,
    /// MAC-MULTICAST-ADDRESS-REFS
    MacMulticastAddressRefs                                                = 4433,
    /// MAC-MULTICAST-CONFIGURATION
    MacMulticastConfiguration                                              = 2173,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 3821,
    /// MAC-MULTICAST-GROUP-REF
    MacMulticastGroupRef                                                   = 1316,
    /// MAC-MULTICAST-GROUPS
    MacMulticastGroups                                                     = 2452,
    /// MAC-SEC-CIPHER-SUITE-CONFIG
    MacSecCipherSuiteConfig                                                = 173,
    /// MAC-SEC-GLOBAL-KAY-PROPS
    MacSecGlobalKayProps                                                   = 4089,
    /// MAC-SEC-KAY-CONFIG
    MacSecKayConfig                                                        = 3913,
    /// MAC-SEC-KAY-PARTICIPANT
    MacSecKayParticipant                                                   = 5412,
    /// MAC-SEC-PARTICIPANT-SET
    MacSecParticipantSet                                                   = 4163,
    /// MAC-SEC-PROPS
    MacSecProps                                                            = 4960,
    /// MAC-SEC-PROPSS
    MacSecPropss                                                           = 2079,
    /// MAC-UNICAST-ADDRESS
    MacUnicastAddress                                                      = 4598,
    /// MACHINE
    Machine                                                                = 3144,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 913,
    /// MACHINE-DESIGN-REF
    MachineDesignRef                                                       = 2900,
    /// MACHINE-ID
    MachineId                                                              = 3568,
    /// MACHINE-MODE-IREF
    MachineModeIref                                                        = 2287,
    /// MACHINE-MODE-IREFS
    MachineModeIrefs                                                       = 3191,
    /// MACHINE-MODE-MACHINES
    MachineModeMachines                                                    = 1237,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 4961,
    /// MACHINE-REF
    MachineRef                                                             = 493,
    /// MACHINE-TIMING
    MachineTiming                                                          = 5627,
    /// MACRO-INITIAL-OFFSET-A
    MacroInitialOffsetA                                                    = 4500,
    /// MACRO-INITIAL-OFFSET-B
    MacroInitialOffsetB                                                    = 1525,
    /// MACRO-PER-CYCLE
    MacroPerCycle                                                          = 2616,
    /// MACROTICK-DURATION
    MacrotickDuration                                                      = 1017,
    /// MAIN-FUNCTION-PERIOD
    MainFunctionPeriod                                                     = 5956,
    /// MAJOR-PROTOCOL-VERSION
    MajorProtocolVersion                                                   = 2131,
    /// MAJOR-VERSION
    MajorVersion                                                           = 1910,
    /// MANAGED-COUPLING-PORTS
    ManagedCouplingPorts                                                   = 3418,
    /// MANAGED-MODE-GROUPS
    ManagedModeGroups                                                      = 5160,
    /// MANAGED-PHYSICAL-CHANNEL-REF
    ManagedPhysicalChannelRef                                              = 2193,
    /// MANAGED-PHYSICAL-CHANNEL-REFS
    ManagedPhysicalChannelRefs                                             = 5374,
    /// MANDATORY
    Mandatory                                                              = 149,
    /// MANUFACTURER-CODE
    ManufacturerCode                                                       = 1918,
    /// MANUFACTURER-VALIDATION-ORDER-REF
    ManufacturerValidationOrderRef                                         = 817,
    /// MANUFACTURER-VALIDATION-ORDER-REFS
    ManufacturerValidationOrderRefs                                        = 644,
    /// MAP
    Map                                                                    = 3825,
    /// MAP-ELEMENT-TUPLES
    MapElementTuples                                                       = 4701,
    /// MAPPED-AP-DATA-ELEMENT-IREF
    MappedApDataElementIref                                                = 4760,
    /// MAPPED-BSW-SERVICE-DEPENDENCY-REF
    MappedBswServiceDependencyRef                                          = 4954,
    /// MAPPED-DATA-ELEMENT-IREF
    MappedDataElementIref                                                  = 210,
    /// MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF
    MappedFlatSwcServiceDependencyRef                                      = 83,
    /// MAPPED-FUNCTION-REF
    MappedFunctionRef                                                      = 131,
    /// MAPPED-OPERATION-IREF
    MappedOperationIref                                                    = 351,
    /// MAPPED-SECURITY-EVENTS
    MappedSecurityEvents                                                   = 214,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    MappedSwcServiceDependencyInExecutableIref                             = 2475,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    MappedSwcServiceDependencyInSystemIref                                 = 4519,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IREF
    MappedSwcServiceDependencyIref                                         = 643,
    /// MAPPING-CONSTRAINTS
    MappingConstraints                                                     = 1634,
    /// MAPPING-DIRECTION
    MappingDirection                                                       = 3379,
    /// MAPPING-REF
    MappingRef                                                             = 5857,
    /// MAPPING-SCOPE
    MappingScope                                                           = 37,
    /// MAPPINGS
    Mappings                                                               = 2176,
    /// MASK
    Mask                                                                   = 278,
    /// MASS-EXP
    MassExp                                                                = 2042,
    /// MASTER
    Master                                                                 = 3497,
    /// MASTER-EVENT-REF
    MasterEventRef                                                         = 473,
    /// MASTER-REF
    MasterRef                                                              = 5222,
    /// MASTER-TO-SLAVE-EVENT-MAPPINGS
    MasterToSlaveEventMappings                                             = 2388,
    /// MASTERED-TRIGGER-REF
    MasteredTriggerRef                                                     = 5698,
    /// MATCHING-CRITERION-REF
    MatchingCriterionRef                                                   = 1367,
    /// MATCHING-RULE-REF
    MatchingRuleRef                                                        = 2013,
    /// MATCHING-RULE-REFS
    MatchingRuleRefs                                                       = 4949,
    /// MAX
    Max                                                                    = 5152,
    /// MAX-ALIVE-CYCLE
    MaxAliveCycle                                                          = 4891,
    /// MAX-AR
    MaxAr                                                                  = 4485,
    /// MAX-AS
    MaxAs                                                                  = 102,
    /// MAX-BASE-TYPE-SIZE
    MaxBaseTypeSize                                                        = 5254,
    /// MAX-BLOCK-SIZE
    MaxBlockSize                                                           = 2438,
    /// MAX-BS
    MaxBs                                                                  = 3994,
    /// MAX-BUFFER-REQUEST
    MaxBufferRequest                                                       = 2043,
    /// MAX-BUFFER-SIZE
    MaxBufferSize                                                          = 2668,
    /// MAX-BYTES-IN-INTERVAL
    MaxBytesInInterval                                                     = 3905,
    /// MAX-COMM-MODE
    MaxCommMode                                                            = 4832,
    /// MAX-CYCLES
    MaxCycles                                                              = 4827,
    /// MAX-DATA-LENGTH
    MaxDataLength                                                          = 2396,
    /// MAX-DEADLINE
    MaxDeadline                                                            = 2707,
    /// MAX-DELTA-COUNTER
    MaxDeltaCounter                                                        = 5288,
    /// MAX-DELTA-COUNTER-INIT
    MaxDeltaCounterInit                                                    = 742,
    /// MAX-DESTINATION-PORT-NUMBER
    MaxDestinationPortNumber                                               = 4533,
    /// MAX-DID-TO-READ
    MaxDidToRead                                                           = 5269,
    /// MAX-DIFF
    MaxDiff                                                                = 3228,
    /// MAX-ERROR-STATE-INIT
    MaxErrorStateInit                                                      = 906,
    /// MAX-ERROR-STATE-INVALID
    MaxErrorStateInvalid                                                   = 4783,
    /// MAX-ERROR-STATE-VALID
    MaxErrorStateValid                                                     = 4922,
    /// MAX-EVENTS-IN-INTERVAL
    MaxEventsInInterval                                                    = 1920,
    /// MAX-EXP-BS
    MaxExpBs                                                               = 4675,
    /// MAX-FC-WAIT
    MaxFcWait                                                              = 5107,
    /// MAX-FR-IF
    MaxFrIf                                                                = 2688,
    /// MAX-GRADIENT
    MaxGradient                                                            = 1490,
    /// MAX-HEADER-ID
    MaxHeaderId                                                            = 409,
    /// MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    MaxInitialVehicleAnnouncementTime                                      = 1549,
    /// MAX-LENGTH
    MaxLength                                                              = 5922,
    /// MAX-MAJOR-VERSION
    MaxMajorVersion                                                        = 4072,
    /// MAX-MARGIN
    MaxMargin                                                              = 2994,
    /// MAX-MINOR-VERSION
    MaxMinorVersion                                                        = 2759,
    /// MAX-NO-NEW-OR-REPEATED-DATA
    MaxNoNewOrRepeatedData                                                 = 5014,
    /// MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS
    MaxNumChangeOfDataIdentfierEvents                                      = 4398,
    /// MAX-NUM-COMPARISION-OF-VALUE-EVENTS
    MaxNumComparisionOfValueEvents                                         = 661,
    /// MAX-NUMBER-FREEZE-FRAME-RECORDS
    MaxNumberFreezeFrameRecords                                            = 2908,
    /// MAX-NUMBER-OF-ALLOWED-UPDATES
    MaxNumberOfAllowedUpdates                                              = 5192,
    /// MAX-NUMBER-OF-CONNECTIONS
    MaxNumberOfConnections                                                 = 2143,
    /// MAX-NUMBER-OF-ELEMENTS
    MaxNumberOfElements                                                    = 1989,
    /// MAX-NUMBER-OF-EVENT-ENTRIES
    MaxNumberOfEventEntries                                                = 4058,
    /// MAX-NUMBER-OF-FILES
    MaxNumberOfFiles                                                       = 146,
    /// MAX-NUMBER-OF-NPDU-PER-CYCLE
    MaxNumberOfNpduPerCycle                                                = 5163,
    /// MAX-NUMBER-OF-OCCURRENCES
    MaxNumberOfOccurrences                                                 = 2472,
    /// MAX-NUMBER-OF-PARALLEL-TRANSFERS
    MaxNumberOfParallelTransfers                                           = 136,
    /// MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING
    MaxNumberOfRequestCorrectlyReceivedResponsePending                     = 5425,
    /// MAX-NUMBER-OF-RESP-PENDING-FRAMES
    MaxNumberOfRespPendingFrames                                           = 1368,
    /// MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS
    MaxNumberOfStoredDtcStatusChangedEvents                                = 4477,
    /// MAX-NUMBER-OF-TIME-QUANTA-PER-BIT
    MaxNumberOfTimeQuantaPerBit                                            = 4322,
    /// MAX-PERIODIC-DID-TO-READ
    MaxPeriodicDidToRead                                                   = 4496,
    /// MAX-PWM-L
    MaxPwmL                                                                = 5331,
    /// MAX-PWM-O
    MaxPwmO                                                                = 5638,
    /// MAX-PWM-S
    MaxPwmS                                                                = 4174,
    /// MAX-REQUEST-BYTES
    MaxRequestBytes                                                        = 2869,
    /// MAX-RETRIES
    MaxRetries                                                             = 1480,
    /// MAX-RPT-EVENT-ID
    MaxRptEventId                                                          = 983,
    /// MAX-RUN-CYCLE-LOOP
    MaxRunCycleLoop                                                        = 5754,
    /// MAX-SAMPLE-POINT
    MaxSamplePoint                                                         = 3015,
    /// MAX-SERVICE-POINT-ID
    MaxServicePointId                                                      = 1352,
    /// MAX-SIZE-TO-FILL
    MaxSizeToFill                                                          = 3362,
    /// MAX-SLOTS
    MaxSlots                                                               = 2838,
    /// MAX-SOURCE-ELEMENT
    MaxSourceElement                                                       = 1816,
    /// MAX-SOURCE-PORT-NUMBER
    MaxSourcePortNumber                                                    = 2604,
    /// MAX-SUPPORTED-DID-LENGTH
    MaxSupportedDidLength                                                  = 4492,
    /// MAX-SYNC-JUMP-WIDTH
    MaxSyncJumpWidth                                                       = 398,
    /// MAX-TESTER-CONNECTIONS
    MaxTesterConnections                                                   = 460,
    /// MAX-TRCV-DELAY-COMPENSATION-OFFSET
    MaxTrcvDelayCompensationOffset                                         = 4615,
    /// MAX-VALUE
    MaxValue                                                               = 3343,
    /// MAX-WITHOUT-CLOCK-CORRECTION-FATAL
    MaxWithoutClockCorrectionFatal                                         = 268,
    /// MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE
    MaxWithoutClockCorrectionPassive                                       = 1092,
    /// MAXIMUM
    Maximum                                                                = 1168,
    /// MAXIMUM-ALLOWED-SIZE
    MaximumAllowedSize                                                     = 3461,
    /// MAXIMUM-DISTANCES
    MaximumDistances                                                       = 4686,
    /// MAXIMUM-DYNAMIC-PAYLOAD-LENGTH
    MaximumDynamicPayloadLength                                            = 4474,
    /// MAXIMUM-EXECUTION-TIME
    MaximumExecutionTime                                                   = 326,
    /// MAXIMUM-INTENDED-BINDING-TIME
    MaximumIntendedBindingTime                                             = 2391,
    /// MAXIMUM-INTER-ARRIVAL-TIME
    MaximumInterArrivalTime                                                = 3941,
    /// MAXIMUM-KEY-LENGTH
    MaximumKeyLength                                                       = 2190,
    /// MAXIMUM-LENGTH
    MaximumLength                                                          = 1193,
    /// MAXIMUM-MEMORY-CONSUMPTION
    MaximumMemoryConsumption                                               = 3412,
    /// MAXIMUM-MESSAGE-LENGTH
    MaximumMessageLength                                                   = 2359,
    /// MAXIMUM-NUMBER-OF-RETRIES
    MaximumNumberOfRetries                                                 = 3980,
    /// MAXIMUM-RECEIVE-BUFFER-LENGTH
    MaximumReceiveBufferLength                                             = 1757,
    /// MAXIMUM-SEGMENT-LENGTH
    MaximumSegmentLength                                                   = 3982,
    /// MAXIMUM-SEGMENT-LENGTH-REQUEST
    MaximumSegmentLengthRequest                                            = 1607,
    /// MAXIMUM-SEGMENT-LENGTH-RESPONSE
    MaximumSegmentLengthResponse                                           = 4733,
    /// MAXIMUM-SELECTED-BINDING-TIME
    MaximumSelectedBindingTime                                             = 1145,
    /// MAXIMUM-SUPPORTED-UCM-MASTER-VERSION
    MaximumSupportedUcmMasterVersion                                       = 5036,
    /// MAXIMUM-SUPPORTED-UCM-VERSION
    MaximumSupportedUcmVersion                                             = 1110,
    /// MAXIMUM-TRANSMISSION-UNIT
    MaximumTransmissionUnit                                                = 3123,
    /// MAXIMUM-TRANSMIT-BUFFER-LENGTH
    MaximumTransmitBufferLength                                            = 2423,
    /// MAY-BE-UNCONNECTED
    MayBeUnconnected                                                       = 4722,
    /// MC-DATA-ACCESS-DETAILS
    McDataAccessDetails                                                    = 434,
    /// MC-DATA-ASSIGNMENTS
    McDataAssignments                                                      = 2223,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 2164,
    /// MC-DATA-INSTANCE-REF
    McDataInstanceRef                                                      = 4023,
    /// MC-DATA-INSTANCE-REFS
    McDataInstanceRefs                                                     = 548,
    /// MC-DATA-INSTANCE-VAR-REF
    McDataInstanceVarRef                                                   = 636,
    /// MC-FUNCTION
    McFunction                                                             = 2295,
    /// MC-FUNCTION-DATA-REF-SET-CONDITIONAL
    McFunctionDataRefSetConditional                                        = 2329,
    /// MC-FUNCTION-DATA-REF-SET-VARIANTS
    McFunctionDataRefSetVariants                                           = 802,
    /// MC-FUNCTION-REF
    McFunctionRef                                                          = 4987,
    /// MC-FUNCTION-REFS
    McFunctionRefs                                                         = 4729,
    /// MC-GROUP
    McGroup                                                                = 3062,
    /// MC-GROUP-DATA-REF-SET-CONDITIONAL
    McGroupDataRefSetConditional                                           = 2324,
    /// MC-GROUP-DATA-REF-SET-VARIANTS
    McGroupDataRefSetVariants                                              = 3340,
    /// MC-PARAMETER-ELEMENT-GROUP
    McParameterElementGroup                                                = 3540,
    /// MC-PARAMETER-INSTANCES
    McParameterInstances                                                   = 1891,
    /// MC-SUPPORT
    McSupport                                                              = 3044,
    /// MC-SW-EMULATION-METHOD-SUPPORT
    McSwEmulationMethodSupport                                             = 4941,
    /// MC-VARIABLE-INSTANCES
    McVariableInstances                                                    = 4883,
    /// MCD-IDENTIFIER
    McdIdentifier                                                          = 5462,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REF
    MeasurableSystemConstantValuesRef                                      = 1349,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS
    MeasurableSystemConstantValuesRefs                                     = 5080,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 5676,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 5381,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 2790,
    /// MEM-ALIGNMENT
    MemAlignment                                                           = 1555,
    /// MEM-CLASS-SYMBOL
    MemClassSymbol                                                         = 5499,
    /// MEM-USAGE
    MemUsage                                                               = 3935,
    /// MEMORY-ALLOCATION-KEYWORD-POLICY
    MemoryAllocationKeywordPolicy                                          = 1406,
    /// MEMORY-CONSUMPTION
    MemoryConsumption                                                      = 95,
    /// MEMORY-DESTINATION-REF
    MemoryDestinationRef                                                   = 2731,
    /// MEMORY-DESTINATION-REFS
    MemoryDestinationRefs                                                  = 4980,
    /// MEMORY-ENTRY-STORAGE-TRIGGER
    MemoryEntryStorageTrigger                                              = 2463,
    /// MEMORY-HIGH-ADDRESS
    MemoryHighAddress                                                      = 3395,
    /// MEMORY-HIGH-ADDRESS-LABEL
    MemoryHighAddressLabel                                                 = 5444,
    /// MEMORY-ID
    MemoryId                                                               = 556,
    /// MEMORY-LOW-ADDRESS
    MemoryLowAddress                                                       = 4238,
    /// MEMORY-LOW-ADDRESS-LABEL
    MemoryLowAddressLabel                                                  = 3103,
    /// MEMORY-RANGE-REF
    MemoryRangeRef                                                         = 5174,
    /// MEMORY-RANGE-REFS
    MemoryRangeRefs                                                        = 4423,
    /// MEMORY-SECTION
    MemorySection                                                          = 2586,
    /// MEMORY-SECTION-LOCATION
    MemorySectionLocation                                                  = 523,
    /// MEMORY-SECTION-LOCATIONS
    MemorySectionLocations                                                 = 2996,
    /// MEMORY-SECTIONS
    MemorySections                                                         = 680,
    /// MEMORY-USAGE
    MemoryUsage                                                            = 5725,
    /// MEMORY-USAGES
    MemoryUsages                                                           = 3834,
    /// MESSAGE-COMPLIANCE
    MessageCompliance                                                      = 3547,
    /// MESSAGE-ID
    MessageId                                                              = 5325,
    /// MESSAGE-LINE-NUMBER
    MessageLineNumber                                                      = 433,
    /// MESSAGE-LINK-LENGTH
    MessageLinkLength                                                      = 745,
    /// MESSAGE-LINK-POSITION
    MessageLinkPosition                                                    = 4034,
    /// MESSAGE-SOURCE-FILE
    MessageSourceFile                                                      = 1697,
    /// MESSAGE-TYPE
    MessageType                                                            = 27,
    /// MESSAGE-TYPE-INFO
    MessageTypeInfo                                                        = 4312,
    /// META-DATA-FIELDS
    MetaDataFields                                                         = 98,
    /// META-DATA-ITEM
    MetaDataItem                                                           = 4160,
    /// META-DATA-ITEM-SET
    MetaDataItemSet                                                        = 4276,
    /// META-DATA-ITEM-SETS
    MetaDataItemSets                                                       = 3827,
    /// META-DATA-ITEM-TYPE
    MetaDataItemType                                                       = 615,
    /// META-DATA-ITEMS
    MetaDataItems                                                          = 1996,
    /// META-DATA-LENGTH
    MetaDataLength                                                         = 2836,
    /// METADATA-PROTECTION-KIND
    MetadataProtectionKind                                                 = 5176,
    /// METHOD-ACTIVATION-ROUTING-GROUPS
    MethodActivationRoutingGroups                                          = 4316,
    /// METHOD-CALL-REF
    MethodCallRef                                                          = 2144,
    /// METHOD-DEPLOYMENTS
    MethodDeployments                                                      = 1457,
    /// METHOD-ID
    MethodId                                                               = 5897,
    /// METHOD-IREF
    MethodIref                                                             = 4803,
    /// METHOD-MAPPING
    MethodMapping                                                          = 2207,
    /// METHOD-MAPPINGS
    MethodMappings                                                         = 3832,
    /// METHOD-QOS-PROPSS
    MethodQosPropss                                                        = 2306,
    /// METHOD-REF
    MethodRef                                                              = 1588,
    /// METHOD-REFS
    MethodRefs                                                             = 1581,
    /// METHOD-REPLY-TOPIC-NAME
    MethodReplyTopicName                                                   = 2555,
    /// METHOD-REQUEST-PROPSS
    MethodRequestPropss                                                    = 3417,
    /// METHOD-REQUEST-TOPIC-NAME
    MethodRequestTopicName                                                 = 2880,
    /// METHOD-RESPONSE-PROPSS
    MethodResponsePropss                                                   = 4568,
    /// METHOD-RETURN-REF
    MethodReturnRef                                                        = 176,
    /// METHOD-TOPICS-ACCESS-RULE-REF
    MethodTopicsAccessRuleRef                                              = 288,
    /// METHODS
    Methods                                                                = 5639,
    /// MICRO-INITIAL-OFFSET-A
    MicroInitialOffsetA                                                    = 2274,
    /// MICRO-INITIAL-OFFSET-B
    MicroInitialOffsetB                                                    = 335,
    /// MICRO-PER-CYCLE
    MicroPerCycle                                                          = 3170,
    /// MICROTICK-DURATION
    MicrotickDuration                                                      = 2188,
    /// MIN
    Min                                                                    = 3894,
    /// MIN-ALIVE-CYCLE
    MinAliveCycle                                                          = 808,
    /// MIN-DATA-LENGTH
    MinDataLength                                                          = 716,
    /// MIN-DEADLINE
    MinDeadline                                                            = 4570,
    /// MIN-DESTINATION-PORT-NUMBER
    MinDestinationPortNumber                                               = 3726,
    /// MIN-HEADER-ID
    MinHeaderId                                                            = 1730,
    /// MIN-LENGTH
    MinLength                                                              = 1234,
    /// MIN-MAJOR-VERSION
    MinMajorVersion                                                        = 2736,
    /// MIN-MARGIN
    MinMargin                                                              = 5066,
    /// MIN-MINOR-VERSION
    MinMinorVersion                                                        = 1334,
    /// MIN-NUMBER-OF-ELEMENTS
    MinNumberOfElements                                                    = 987,
    /// MIN-NUMBER-OF-OCCURRENCES
    MinNumberOfOccurrences                                                 = 1113,
    /// MIN-NUMBER-OF-TIME-QUANTA-PER-BIT
    MinNumberOfTimeQuantaPerBit                                            = 1709,
    /// MIN-OK-STATE-INIT
    MinOkStateInit                                                         = 1279,
    /// MIN-OK-STATE-INVALID
    MinOkStateInvalid                                                      = 1024,
    /// MIN-OK-STATE-VALID
    MinOkStateValid                                                        = 2735,
    /// MIN-PWM-L
    MinPwmL                                                                = 543,
    /// MIN-PWM-O
    MinPwmO                                                                = 5689,
    /// MIN-PWM-S
    MinPwmS                                                                = 4472,
    /// MIN-RPT-EVENT-ID
    MinRptEventId                                                          = 5075,
    /// MIN-SAMPLE-POINT
    MinSamplePoint                                                         = 720,
    /// MIN-SERVICE-POINT-ID
    MinServicePointId                                                      = 4454,
    /// MIN-SOURCE-PORT-NUMBER
    MinSourcePortNumber                                                    = 1088,
    /// MIN-SYNC-JUMP-WIDTH
    MinSyncJumpWidth                                                       = 2058,
    /// MIN-TRCV-DELAY-COMPENSATION-OFFSET
    MinTrcvDelayCompensationOffset                                         = 2026,
    /// MIN-VALUE
    MinValue                                                               = 3335,
    /// MINIMUM
    Minimum                                                                = 4067,
    /// MINIMUM-DELAY
    MinimumDelay                                                           = 2564,
    /// MINIMUM-DISTANCES
    MinimumDistances                                                       = 5411,
    /// MINIMUM-EXECUTION-TIME
    MinimumExecutionTime                                                   = 5517,
    /// MINIMUM-FIFO-LENGTH
    MinimumFifoLength                                                      = 2056,
    /// MINIMUM-INTENDED-BINDING-TIME
    MinimumIntendedBindingTime                                             = 642,
    /// MINIMUM-INTER-ARRIVAL-TIME
    MinimumInterArrivalTime                                                = 3010,
    /// MINIMUM-INTERVAL-LENGTH
    MinimumIntervalLength                                                  = 1259,
    /// MINIMUM-MEMORY-CONSUMPTION
    MinimumMemoryConsumption                                               = 4877,
    /// MINIMUM-MULTICAST-SEPERATION-TIME
    MinimumMulticastSeperationTime                                         = 5701,
    /// MINIMUM-NUMBER-OF-REQUESTS
    MinimumNumberOfRequests                                                = 1227,
    /// MINIMUM-RX-CONTAINER-QUEUE-SIZE
    MinimumRxContainerQueueSize                                            = 1025,
    /// MINIMUM-SELECTED-BINDING-TIME
    MinimumSelectedBindingTime                                             = 991,
    /// MINIMUM-SEND-INTERVAL
    MinimumSendInterval                                                    = 1783,
    /// MINIMUM-SEPARATION-TIME
    MinimumSeparationTime                                                  = 4914,
    /// MINIMUM-START-INTERVAL
    MinimumStartInterval                                                   = 3918,
    /// MINIMUM-SUPPORTED-UCM-MASTER-VERSION
    MinimumSupportedUcmMasterVersion                                       = 5620,
    /// MINIMUM-SUPPORTED-UCM-VERSION
    MinimumSupportedUcmVersion                                             = 4016,
    /// MINIMUM-SUSTAINED-SIZE
    MinimumSustainedSize                                                   = 2409,
    /// MINIMUM-TIMER-GRANULARITY
    MinimumTimerGranularity                                                = 1531,
    /// MINIMUM-TX-CONTAINER-QUEUE-SIZE
    MinimumTxContainerQueueSize                                            = 1677,
    /// MINISLOT-ACTION-POINT-OFFSET
    MinislotActionPointOffset                                              = 2983,
    /// MINISLOT-DURATION
    MinislotDuration                                                       = 2346,
    /// MINOR-PROTOCOL-VERSION
    MinorProtocolVersion                                                   = 1449,
    /// MINOR-VERSION
    MinorVersion                                                           = 2676,
    /// MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID
    MirrorSourceLinToCanRangeBaseId                                        = 4913,
    /// MIRROR-STATUS-CAN-ID
    MirrorStatusCanId                                                      = 483,
    /// MKA-PARTICIPANT-REF
    MkaParticipantRef                                                      = 313,
    /// MKA-PARTICIPANT-REFS
    MkaParticipantRefs                                                     = 2157,
    /// MKA-PARTICIPANTS
    MkaParticipants                                                        = 5210,
    /// ML-GRAPHIC
    MlGraphic                                                              = 3951,
    /// MODE
    Mode                                                                   = 558,
    /// MODE-ACCESS-POINT
    ModeAccessPoint                                                        = 1987,
    /// MODE-ACCESS-POINTS
    ModeAccessPoints                                                       = 1408,
    /// MODE-CONDITION
    ModeCondition                                                          = 4703,
    /// MODE-CONDITION-REF
    ModeConditionRef                                                       = 255,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 3855,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 3955,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 1337,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-IREF
    ModeDeclarationGroupPrototypeIref                                      = 1387,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ModeDeclarationGroupPrototypeRef                                       = 5040,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL
    ModeDeclarationGroupPrototypeRefConditional                            = 5551,
    /// MODE-DECLARATION-GROUP-REF
    ModeDeclarationGroupRef                                                = 4892,
    /// MODE-DECLARATION-GROUP-REFS
    ModeDeclarationGroupRefs                                               = 2937,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 3196,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 4878,
    /// MODE-DECLARATION-MAPPING-SET-REF
    ModeDeclarationMappingSetRef                                           = 2946,
    /// MODE-DECLARATION-MAPPINGS
    ModeDeclarationMappings                                                = 1166,
    /// MODE-DECLARATION-REF
    ModeDeclarationRef                                                     = 1391,
    /// MODE-DECLARATION-REFS
    ModeDeclarationRefs                                                    = 3744,
    /// MODE-DECLARATIONS
    ModeDeclarations                                                       = 406,
    /// MODE-DEPENDENT-STARTUP-CONFIG
    ModeDependentStartupConfig                                             = 1276,
    /// MODE-DEPENDENT-STARTUP-CONFIGS
    ModeDependentStartupConfigs                                            = 4872,
    /// MODE-DRIVEN-FALSE-CONDITIONS
    ModeDrivenFalseConditions                                              = 2798,
    /// MODE-DRIVEN-TRANSMISSION-MODE-CONDITION
    ModeDrivenTransmissionModeCondition                                    = 3777,
    /// MODE-DRIVEN-TRUE-CONDITIONS
    ModeDrivenTrueConditions                                               = 3949,
    /// MODE-ELEMENT-REF
    ModeElementRef                                                         = 3433,
    /// MODE-ELEMENTS
    ModeElements                                                           = 3198,
    /// MODE-GROUP
    ModeGroup                                                              = 1201,
    /// MODE-GROUP-IREF
    ModeGroupIref                                                          = 4543,
    /// MODE-GROUP-REF
    ModeGroupRef                                                           = 4183,
    /// MODE-IN-BSW-INSTANCE-REF
    ModeInBswInstanceRef                                                   = 2267,
    /// MODE-IN-SWC-INSTANCE-REF
    ModeInSwcInstanceRef                                                   = 4400,
    /// MODE-INSTANCE
    ModeInstance                                                           = 2290,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 3946,
    /// MODE-IREF
    ModeIref                                                               = 5503,
    /// MODE-IREFS
    ModeIrefs                                                              = 1462,
    /// MODE-MANAGER-ERROR-BEHAVIOR
    ModeManagerErrorBehavior                                               = 1526,
    /// MODE-MAPPING
    ModeMapping                                                            = 3283,
    /// MODE-PORT-ANNOTATION
    ModePortAnnotation                                                     = 1489,
    /// MODE-PORT-ANNOTATIONS
    ModePortAnnotations                                                    = 2049,
    /// MODE-RECEIVER-POLICYS
    ModeReceiverPolicys                                                    = 2524,
    /// MODE-REF
    ModeRef                                                                = 1896,
    /// MODE-REPORTER-PORT-IREF
    ModeReporterPortIref                                                   = 2522,
    /// MODE-REQUEST-TYPE-MAP
    ModeRequestTypeMap                                                     = 4535,
    /// MODE-REQUEST-TYPE-MAPS
    ModeRequestTypeMaps                                                    = 3937,
    /// MODE-SENDER-POLICYS
    ModeSenderPolicys                                                      = 194,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY
    ModeSwitchEventTriggeredActivity                                       = 234,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS
    ModeSwitchEventTriggeredActivitys                                      = 3888,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 2585,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 5488,
    /// MODE-SWITCH-POINTS
    ModeSwitchPoints                                                       = 5212,
    /// MODE-SWITCH-RECEIVER-COM-SPEC
    ModeSwitchReceiverComSpec                                              = 5672,
    /// MODE-SWITCH-SENDER-COM-SPEC
    ModeSwitchSenderComSpec                                                = 2183,
    /// MODE-SWITCHED-ACK
    ModeSwitchedAck                                                        = 233,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 1553,
    /// MODE-TRANSITION
    ModeTransition                                                         = 1544,
    /// MODE-TRANSITIONS
    ModeTransitions                                                        = 2348,
    /// MODE-USER-ERROR-BEHAVIOR
    ModeUserErrorBehavior                                                  = 1543,
    /// MODEL-OBJECT-REFERENCE
    ModelObjectReference                                                   = 4623,
    /// MODES
    Modes                                                                  = 3016,
    /// MODIFICATION
    Modification                                                           = 4965,
    /// MODIFICATIONS
    Modifications                                                          = 2036,
    /// MODIFIED-DATAS
    ModifiedDatas                                                          = 4831,
    /// MODULE-DESCRIPTION-REF
    ModuleDescriptionRef                                                   = 2626,
    /// MODULE-ID
    ModuleId                                                               = 2090,
    /// MODULE-INSTANTIATION-REF
    ModuleInstantiationRef                                                 = 4343,
    /// MODULE-INSTANTIATION-REFS
    ModuleInstantiationRefs                                                = 2469,
    /// MODULE-INSTANTIATIONS
    ModuleInstantiations                                                   = 3420,
    /// MODULE-REF
    ModuleRef                                                              = 599,
    /// MODULE-REFS
    ModuleRefs                                                             = 3632,
    /// MOLAR-AMOUNT-EXP
    MolarAmountExp                                                         = 4635,
    /// MONITORED-IDENTIFIER-REF
    MonitoredIdentifierRef                                                 = 1364,
    /// MONOTONY
    Monotony                                                               = 1438,
    /// MORE-FRAGMENTS
    MoreFragments                                                          = 1275,
    /// MSG-ID-MASK
    MsgIdMask                                                              = 803,
    /// MSG-ID-MATCH
    MsgIdMatch                                                             = 5431,
    /// MSR-QUERY-ARG
    MsrQueryArg                                                            = 443,
    /// MSR-QUERY-CHAPTER
    MsrQueryChapter                                                        = 3899,
    /// MSR-QUERY-NAME
    MsrQueryName                                                           = 5708,
    /// MSR-QUERY-P-1
    MsrQueryP1                                                             = 2579,
    /// MSR-QUERY-P-2
    MsrQueryP2                                                             = 3729,
    /// MSR-QUERY-PROPS
    MsrQueryProps                                                          = 190,
    /// MSR-QUERY-RESULT-CHAPTER
    MsrQueryResultChapter                                                  = 798,
    /// MSR-QUERY-RESULT-P-1
    MsrQueryResultP1                                                       = 1228,
    /// MSR-QUERY-RESULT-P-2
    MsrQueryResultP2                                                       = 2474,
    /// MSR-QUERY-RESULT-TOPIC-1
    MsrQueryResultTopic1                                                   = 5833,
    /// MSR-QUERY-TOPIC-1
    MsrQueryTopic1                                                         = 3051,
    /// MULTICAST-CONNECTOR-REF
    MulticastConnectorRef                                                  = 447,
    /// MULTICAST-CONNECTOR-REFS
    MulticastConnectorRefs                                                 = 3136,
    /// MULTICAST-CREDENTIALS
    MulticastCredentials                                                   = 2560,
    /// MULTICAST-REF
    MulticastRef                                                           = 4336,
    /// MULTICAST-SD-IP-ADDRESS-REF
    MulticastSdIpAddressRef                                                = 4452,
    /// MULTICAST-SECURE-COM-PROPS-REF
    MulticastSecureComPropsRef                                             = 3927,
    /// MULTICAST-SEGMENTATION
    MulticastSegmentation                                                  = 1988,
    /// MULTICAST-THRESHOLD
    MulticastThreshold                                                     = 3056,
    /// MULTICAST-UDP-PORT
    MulticastUdpPort                                                       = 5618,
    /// MULTIPLE-CONFIGURATION-CONTAINER
    MultipleConfigurationContainer                                         = 2863,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 4043,
    /// MULTIPLICITY-CONFIG-CLASSES
    MultiplicityConfigClasses                                              = 404,
    /// MULTIPLICITY-RESTRICTION
    MultiplicityRestriction                                                = 5044,
    /// N
    N                                                                      = 2674,
    /// N-DATA-SETS
    NDataSets                                                              = 4992,
    /// N-PDU
    NPdu                                                                   = 940,
    /// N-PDU-REF
    NPduRef                                                                = 1381,
    /// N-PDU-REFS
    NPduRefs                                                               = 1086,
    /// N-ROM-BLOCKS
    NRomBlocks                                                             = 1701,
    /// NAGLES-ALGORITHM
    NaglesAlgorithm                                                        = 2906,
    /// NAME
    Name                                                                   = 3629,
    /// NAME-TOKEN-VALUE-VARIATION-POINT
    NameTokenValueVariationPoint                                           = 779,
    /// NAMED-CURVE-ID
    NamedCurveId                                                           = 1332,
    /// NAMESPACES
    Namespaces                                                             = 3063,
    /// NAS-TIMEOUT
    NasTimeout                                                             = 5203,
    /// NATIVE-DECLARATION
    NativeDeclaration                                                      = 4925,
    /// NDP-PROPS
    NdpProps                                                               = 4143,
    /// NEEDS-ORIGINAL-DATA
    NeedsOriginalData                                                      = 2988,
    /// NEIGHBOR-CACHE-SIZE
    NeighborCacheSize                                                      = 4136,
    /// NESTED-ELEMENT-REF
    NestedElementRef                                                       = 2747,
    /// NESTED-ELEMENT-REFS
    NestedElementRefs                                                      = 5700,
    /// NESTED-ELEMENTS
    NestedElements                                                         = 2528,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 3853,
    /// NETWORK-CONFIGURATION-REF
    NetworkConfigurationRef                                                = 4731,
    /// NETWORK-CONFIGURATION-REFS
    NetworkConfigurationRefs                                               = 4923,
    /// NETWORK-CONFIGURATIONS
    NetworkConfigurations                                                  = 4708,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 480,
    /// NETWORK-ENDPOINT-ADDRESSES
    NetworkEndpointAddresses                                               = 2823,
    /// NETWORK-ENDPOINT-REF
    NetworkEndpointRef                                                     = 383,
    /// NETWORK-ENDPOINT-REFS
    NetworkEndpointRefs                                                    = 4083,
    /// NETWORK-ENDPOINTS
    NetworkEndpoints                                                       = 3068,
    /// NETWORK-HANDLES
    NetworkHandles                                                         = 4213,
    /// NETWORK-ID
    NetworkId                                                              = 3588,
    /// NETWORK-IDLE-TIME
    NetworkIdleTime                                                        = 4139,
    /// NETWORK-INTERFACE-ID
    NetworkInterfaceId                                                     = 889,
    /// NETWORK-INTERFACE-REF
    NetworkInterfaceRef                                                    = 5895,
    /// NETWORK-INTERFACE-REFS
    NetworkInterfaceRefs                                                   = 2101,
    /// NETWORK-INTERFACES
    NetworkInterfaces                                                      = 769,
    /// NETWORK-LAYER-RULE
    NetworkLayerRule                                                       = 5957,
    /// NETWORK-MANAGEMENT-VECTOR-LENGTH
    NetworkManagementVectorLength                                          = 4757,
    /// NETWORK-MASK
    NetworkMask                                                            = 4321,
    /// NETWORK-REPRESENTATION
    NetworkRepresentation                                                  = 3672,
    /// NETWORK-REPRESENTATION-PROPS
    NetworkRepresentationProps                                             = 2960,
    /// NETWORK-SEGMENT-ID
    NetworkSegmentId                                                       = 1249,
    /// NETWORK-TIME-CONSUMER-REF
    NetworkTimeConsumerRef                                                 = 4424,
    /// NETWORK-TIME-MASTER-REF
    NetworkTimeMasterRef                                                   = 5215,
    /// NETWORK-TIME-PROVIDER-REF
    NetworkTimeProviderRef                                                 = 1561,
    /// NETWORK-TIME-SLAVE-REF
    NetworkTimeSlaveRef                                                    = 4022,
    /// NEW-NAD
    NewNad                                                                 = 707,
    /// NEXT-HEADER
    NextHeader                                                             = 2118,
    /// NEXT-HIGHER-CERTIFICATE-REF
    NextHigherCertificateRef                                               = 2466,
    /// NEXT-STATE-IREF
    NextStateIref                                                          = 2453,
    /// NM-ACTIVE-COORDINATOR
    NmActiveCoordinator                                                    = 3755,
    /// NM-BUS-LOAD-REDUCTION-ENABLED
    NmBusLoadReductionEnabled                                              = 2540,
    /// NM-BUS-SYNCHRONIZATION-ENABLED
    NmBusSynchronizationEnabled                                            = 4193,
    /// NM-BUSLOAD-REDUCTION-ACTIVE
    NmBusloadReductionActive                                               = 4684,
    /// NM-BUSLOAD-REDUCTION-ENABLED
    NmBusloadReductionEnabled                                              = 4669,
    /// NM-CAR-WAKE-UP-BIT-POSITION
    NmCarWakeUpBitPosition                                                 = 47,
    /// NM-CAR-WAKE-UP-FILTER-ENABLED
    NmCarWakeUpFilterEnabled                                               = 843,
    /// NM-CAR-WAKE-UP-FILTER-NODE-ID
    NmCarWakeUpFilterNodeId                                                = 3620,
    /// NM-CAR-WAKE-UP-RX-ENABLED
    NmCarWakeUpRxEnabled                                                   = 5468,
    /// NM-CBV-POSITION
    NmCbvPosition                                                          = 19,
    /// NM-CHANNEL-ACTIVE
    NmChannelActive                                                        = 4745,
    /// NM-CHANNEL-ID
    NmChannelId                                                            = 3157,
    /// NM-CHANNEL-SLEEP-MASTER
    NmChannelSleepMaster                                                   = 5389,
    /// NM-CLUSTER-COUPLINGS
    NmClusterCouplings                                                     = 4253,
    /// NM-CLUSTERS
    NmClusters                                                             = 3828,
    /// NM-COM-CONTROL-ENABLED
    NmComControlEnabled                                                    = 698,
    /// NM-CONFIG
    NmConfig                                                               = 4541,
    /// NM-CONTROL-BIT-VECTOR-ACTIVE
    NmControlBitVectorActive                                               = 5728,
    /// NM-CONTROL-BIT-VECTOR-ENABLED
    NmControlBitVectorEnabled                                              = 150,
    /// NM-COORD-CLUSTER
    NmCoordCluster                                                         = 4654,
    /// NM-COORD-SYNC-SUPPORT
    NmCoordSyncSupport                                                     = 1585,
    /// NM-COORDINATOR
    NmCoordinator                                                          = 3421,
    /// NM-COORDINATOR-ROLE
    NmCoordinatorRole                                                      = 1586,
    /// NM-CYCLETIME-MAIN-FUNCTION
    NmCycletimeMainFunction                                                = 4352,
    /// NM-DATA-CYCLE
    NmDataCycle                                                            = 3510,
    /// NM-DATA-DISABLED
    NmDataDisabled                                                         = 1762,
    /// NM-DATA-ENABLED
    NmDataEnabled                                                          = 2010,
    /// NM-DATA-INFORMATION
    NmDataInformation                                                      = 2539,
    /// NM-DETECTION-LOCK
    NmDetectionLock                                                        = 620,
    /// NM-ECU
    NmEcu                                                                  = 1015,
    /// NM-GLOBAL-COORDINATOR-TIME
    NmGlobalCoordinatorTime                                                = 5816,
    /// NM-HANDLE-REF
    NmHandleRef                                                            = 2366,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 3601,
    /// NM-HW-VOTE-ENABLED
    NmHwVoteEnabled                                                        = 4147,
    /// NM-IF-ECU-REF
    NmIfEcuRef                                                             = 3229,
    /// NM-IF-ECUS
    NmIfEcus                                                               = 2408,
    /// NM-IMMEDIATE-NM-CYCLE-TIME
    NmImmediateNmCycleTime                                                 = 4095,
    /// NM-IMMEDIATE-NM-TRANSMISSIONS
    NmImmediateNmTransmissions                                             = 1330,
    /// NM-IMMEDIATE-RESTART-ENABLED
    NmImmediateRestartEnabled                                              = 411,
    /// NM-INSTANCE-ID
    NmInstanceId                                                           = 4088,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 263,
    /// NM-MAIN-FUNCTION-ACROSS-FR-CYCLE
    NmMainFunctionAcrossFrCycle                                            = 3910,
    /// NM-MAIN-FUNCTION-PERIOD
    NmMainFunctionPeriod                                                   = 4275,
    /// NM-MESSAGE-TIMEOUT-TIME
    NmMessageTimeoutTime                                                   = 254,
    /// NM-MSG-CYCLE-OFFSET
    NmMsgCycleOffset                                                       = 1507,
    /// NM-MSG-CYCLE-TIME
    NmMsgCycleTime                                                         = 5919,
    /// NM-MSG-REDUCED-TIME
    NmMsgReducedTime                                                       = 2386,
    /// NM-MULTIPLE-CHANNELS-ENABLED
    NmMultipleChannelsEnabled                                              = 5723,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 2220,
    /// NM-NETWORK-TIMEOUT
    NmNetworkTimeout                                                       = 2860,
    /// NM-NID-POSITION
    NmNidPosition                                                          = 1524,
    /// NM-NODE-DETECTION-ENABLED
    NmNodeDetectionEnabled                                                 = 4870,
    /// NM-NODE-ID
    NmNodeId                                                               = 499,
    /// NM-NODE-ID-ENABLED
    NmNodeIdEnabled                                                        = 5238,
    /// NM-NODE-REF
    NmNodeRef                                                              = 5446,
    /// NM-NODE-REFS
    NmNodeRefs                                                             = 5613,
    /// NM-NODES
    NmNodes                                                                = 2547,
    /// NM-PASSIVE-MODE-ENABLED
    NmPassiveModeEnabled                                                   = 1708,
    /// NM-PDU
    NmPdu                                                                  = 605,
    /// NM-PDU-REF
    NmPduRef                                                               = 2700,
    /// NM-PDU-REF-CONDITIONAL
    NmPduRefConditional                                                    = 2542,
    /// NM-PDU-RX-INDICATION-ENABLED
    NmPduRxIndicationEnabled                                               = 533,
    /// NM-PDUS
    NmPdus                                                                 = 2543,
    /// NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS
    NmPnHandleMultipleNetworkRequests                                      = 66,
    /// NM-PNC-PARTICIPATION
    NmPncParticipation                                                     = 5202,
    /// NM-RANGE-CONFIG
    NmRangeConfig                                                          = 4504,
    /// NM-READY-SLEEP-COUNT
    NmReadySleepCount                                                      = 3833,
    /// NM-READY-SLEEP-TIME
    NmReadySleepTime                                                       = 1343,
    /// NM-REMOTE-SLEEP-IND-ENABLED
    NmRemoteSleepIndEnabled                                                = 4242,
    /// NM-REMOTE-SLEEP-INDICATION-TIME
    NmRemoteSleepIndicationTime                                            = 5748,
    /// NM-REPEAT-MESSAGE-BIT-ACTIVE
    NmRepeatMessageBitActive                                               = 5021,
    /// NM-REPEAT-MESSAGE-BIT-ENABLE
    NmRepeatMessageBitEnable                                               = 1318,
    /// NM-REPEAT-MESSAGE-TIME
    NmRepeatMessageTime                                                    = 1687,
    /// NM-REPEAT-MSG-IND-ENABLED
    NmRepeatMsgIndEnabled                                                  = 3459,
    /// NM-REPEAT-MSG-INDICATION-ENABLED
    NmRepeatMsgIndicationEnabled                                           = 801,
    /// NM-REPETITION-CYCLE
    NmRepetitionCycle                                                      = 3383,
    /// NM-SCHEDULE-VARIANT
    NmScheduleVariant                                                      = 540,
    /// NM-SHUTDOWN-DELAY-TIMER
    NmShutdownDelayTimer                                                   = 2552,
    /// NM-STATE-CHANGE-IND-ENABLED
    NmStateChangeIndEnabled                                                = 5201,
    /// NM-SYNCHRONIZATION-POINT-ENABLED
    NmSynchronizationPointEnabled                                          = 1005,
    /// NM-SYNCHRONIZING-NETWORK
    NmSynchronizingNetwork                                                 = 3476,
    /// NM-USER-DATA-ENABLED
    NmUserDataEnabled                                                      = 5793,
    /// NM-USER-DATA-LENGTH
    NmUserDataLength                                                       = 2570,
    /// NM-USER-DATA-OFFSET
    NmUserDataOffset                                                       = 1224,
    /// NM-VECTOR-EARLY-UPDATE
    NmVectorEarlyUpdate                                                    = 3306,
    /// NM-VOTE-INFORMATION
    NmVoteInformation                                                      = 1579,
    /// NM-VOTING-CYCLE
    NmVotingCycle                                                          = 2553,
    /// NM-WAIT-BUS-SLEEP-TIME
    NmWaitBusSleepTime                                                     = 2166,
    /// NO-CHECKPOINT-SUPERVISION
    NoCheckpointSupervision                                                = 5400,
    /// NO-CHECKPOINT-SUPERVISIONS
    NoCheckpointSupervisions                                               = 5438,
    /// NO-SUPERVISION
    NoSupervision                                                          = 1843,
    /// NO-SUPERVISIONS
    NoSupervisions                                                         = 3885,
    /// NODE-IDENTIFIER
    NodeIdentifier                                                         = 811,
    /// NODE-NAME
    NodeName                                                               = 4751,
    /// NODE-PORTS
    NodePorts                                                              = 2354,
    /// NODE-REF
    NodeRef                                                                = 4503,
    /// NOMINAL
    Nominal                                                                = 5338,
    /// NOMINAL-EXECUTION-TIME
    NominalExecutionTime                                                   = 4021,
    /// NON-OS-MODULE-INSTANTIATION-REF
    NonOsModuleInstantiationRef                                            = 2106,
    /// NON-VERBOSE-MODE
    NonVerboseMode                                                         = 4394,
    /// NONQUEUED-RECEIVER-COM-SPEC
    NonqueuedReceiverComSpec                                               = 2464,
    /// NONQUEUED-SENDER-COM-SPEC
    NonqueuedSenderComSpec                                                 = 163,
    /// NOT-AVAILABLE-VALUE-SPECIFICATION
    NotAvailableValueSpecification                                         = 2044,
    /// NOTATION
    Notation                                                               = 2424,
    /// NOTE
    Note                                                                   = 531,
    /// NOTIFICATION-PORT-IREF
    NotificationPortIref                                                   = 1735,
    /// NOTIFICATION-STATE
    NotificationState                                                      = 2015,
    /// NOTIFICATION-TIME
    NotificationTime                                                       = 2249,
    /// NOTIFICATIONS
    Notifications                                                          = 5228,
    /// NOTIFIED-STATUSS
    NotifiedStatuss                                                        = 3711,
    /// NOTIFIER
    Notifier                                                               = 4587,
    /// NOTIFIER-DATA-ELEMENT-REF
    NotifierDataElementRef                                                 = 4020,
    /// NOTIFIER-REF
    NotifierRef                                                            = 5270,
    /// NOTIFIER-SIGNAL-TRIGGERING-REF
    NotifierSignalTriggeringRef                                            = 1608,
    /// NRC-VALUE
    NrcValue                                                               = 618,
    /// NTU
    Ntu                                                                    = 153,
    /// NUM-FAILED-SECURITY-ACCESS
    NumFailedSecurityAccess                                                = 5474,
    /// NUMBER
    Number                                                                 = 5495,
    /// NUMBER-OF-BITS
    NumberOfBits                                                           = 130,
    /// NUMBER-OF-CONNECTED-CLIENTS
    NumberOfConnectedClients                                               = 4822,
    /// NUMBER-OF-INSTRUCTIONS
    NumberOfInstructions                                                   = 4820,
    /// NUMBER-OF-MINISLOTS
    NumberOfMinislots                                                      = 4065,
    /// NUMBER-OF-NOTIFIER-SETS
    NumberOfNotifierSets                                                   = 2620,
    /// NUMBER-OF-PARALLEL-TCP-SESSIONS
    NumberOfParallelTcpSessions                                            = 2691,
    /// NUMBER-OF-REPETITIONS
    NumberOfRepetitions                                                    = 1998,
    /// NUMBER-OF-RESTART-ATTEMPTS
    NumberOfRestartAttempts                                                = 5830,
    /// NUMBER-OF-STATIC-SLOTS
    NumberOfStaticSlots                                                    = 3357,
    /// NUMBER-OF-WORKERS
    NumberOfWorkers                                                        = 908,
    /// NUMERICAL-RULE-BASED-VALUE-SPECIFICATION
    NumericalRuleBasedValueSpecification                                   = 5246,
    /// NUMERICAL-VALUE
    NumericalValue                                                         = 2331,
    /// NUMERICAL-VALUE-SPECIFICATION
    NumericalValueSpecification                                            = 1177,
    /// NUMERICAL-VALUE-VARIATION-POINT
    NumericalValueVariationPoint                                           = 2153,
    /// NV-BLOCK-DATA-MAPPING
    NvBlockDataMapping                                                     = 4101,
    /// NV-BLOCK-DATA-MAPPINGS
    NvBlockDataMappings                                                    = 5661,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 1214,
    /// NV-BLOCK-DESCRIPTORS
    NvBlockDescriptors                                                     = 4112,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 3339,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 534,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 4766,
    /// NV-DATA-PORT-ANNOTATION
    NvDataPortAnnotation                                                   = 4249,
    /// NV-DATA-PORT-ANNOTATIONS
    NvDataPortAnnotations                                                  = 259,
    /// NV-DATAS
    NvDatas                                                                = 2362,
    /// NV-PROVIDE-COM-SPEC
    NvProvideComSpec                                                       = 5632,
    /// NV-RAM-BLOCK-ELEMENT
    NvRamBlockElement                                                      = 1839,
    /// NV-REQUIRE-COM-SPEC
    NvRequireComSpec                                                       = 329,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 3564,
    /// OBD-DTC-NUMBER
    ObdDtcNumber                                                           = 3649,
    /// OBD-DTC-VALUE
    ObdDtcValue                                                            = 3490,
    /// OBD-DTC-VALUE-3-BYTE
    ObdDtcValue3Byte                                                       = 5705,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 7,
    /// OBD-MID
    ObdMid                                                                 = 3837,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 4027,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 5600,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 4777,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 1063,
    /// OBD-SUPPORT
    ObdSupport                                                             = 4956,
    /// OBJECT-DEFINITION-REF
    ObjectDefinitionRef                                                    = 2074,
    /// OBJECT-DEFINITION-REFS
    ObjectDefinitionRefs                                                   = 2573,
    /// OBJECT-DEFINTION-REF
    ObjectDefintionRef                                                     = 584,
    /// OBJECT-DEFINTION-REFS
    ObjectDefintionRefs                                                    = 2977,
    /// OBJECT-REF
    ObjectRef                                                              = 3204,
    /// OBJECT-REFS
    ObjectRefs                                                             = 4546,
    /// OCCURRENCE-COUNTER-PROCESSING
    OccurrenceCounterProcessing                                            = 1661,
    /// OCCURRENCE-EXPRESSION
    OccurrenceExpression                                                   = 837,
    /// OFFER-CYCLIC-DELAY
    OfferCyclicDelay                                                       = 2875,
    /// OFFSET
    Offset                                                                 = 1051,
    /// OFFSET-CORRECTION-ADAPTION-INTERVAL
    OffsetCorrectionAdaptionInterval                                       = 4428,
    /// OFFSET-CORRECTION-JUMP-THRESHOLD
    OffsetCorrectionJumpThreshold                                          = 2230,
    /// OFFSET-CORRECTION-OUT
    OffsetCorrectionOut                                                    = 1911,
    /// OFFSET-CORRECTION-START
    OffsetCorrectionStart                                                  = 2569,
    /// OFFSET-SI-TO-UNIT
    OffsetSiToUnit                                                         = 1737,
    /// OFFSET-TIME-DOMAIN-REF
    OffsetTimeDomainRef                                                    = 2533,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 3683,
    /// OFFSETS
    Offsets                                                                = 3533,
    /// OFNS-DATA-ID-LIST
    OfnsDataIdList                                                         = 624,
    /// OFNS-DATA-ID-LISTS
    OfnsDataIdLists                                                        = 5442,
    /// OFS-DATA-ID-LIST
    OfsDataIdList                                                          = 407,
    /// OFS-DATA-ID-LISTS
    OfsDataIdLists                                                         = 1469,
    /// OFS-SUB-TLV
    OfsSubTlv                                                              = 5363,
    /// ON-BOARD-MONITOR-ID
    OnBoardMonitorId                                                       = 2495,
    /// ON-FAIL-PERMISSIVE-MODE
    OnFailPermissiveMode                                                   = 1659,
    /// ON-FAIL-PERMISSIVE-MODE-TIMEOUT
    OnFailPermissiveModeTimeout                                            = 4255,
    /// ON-TRANSITION-VALUE
    OnTransitionValue                                                      = 5259,
    /// ONE-EVERY-N
    OneEveryN                                                              = 4289,
    /// OP
    Op                                                                     = 4711,
    /// OPERATION-ARGUMENT-INSTANCE-IREF
    OperationArgumentInstanceIref                                          = 5069,
    /// OPERATION-CYCLE
    OperationCycle                                                         = 1953,
    /// OPERATION-CYCLE-AUTOMATIC-END
    OperationCycleAutomaticEnd                                             = 4906,
    /// OPERATION-CYCLE-AUTOSTART
    OperationCycleAutostart                                                = 1827,
    /// OPERATION-CYCLE-REF
    OperationCycleRef                                                      = 3467,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 5681,
    /// OPERATION-IREF
    OperationIref                                                          = 4542,
    /// OPERATION-IREFS
    OperationIrefs                                                         = 3787,
    /// OPERATION-MAPPINGS
    OperationMappings                                                      = 4106,
    /// OPERATION-MODE
    OperationMode                                                          = 939,
    /// OPERATION-PROTOTYPE-IREF
    OperationPrototypeIref                                                 = 5450,
    /// OPERATION-REF
    OperationRef                                                           = 2405,
    /// OPERATIONS
    Operations                                                             = 986,
    /// OPERATOR
    Operator                                                               = 4381,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF
    OptAppRecordElementInMethodIref                                        = 208,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS
    OptAppRecordElementInMethodIrefs                                       = 5271,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    OptAppRecordElementInServiceInterfaceIref                              = 2351,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS
    OptAppRecordElementInServiceInterfaceIrefs                             = 3900,
    /// OPT-ARGUMENT-REF
    OptArgumentRef                                                         = 5808,
    /// OPT-ARGUMENT-REFS
    OptArgumentRefs                                                        = 4171,
    /// OPT-IMPL-RECORD-ELEMENT
    OptImplRecordElement                                                   = 5181,
    /// OPT-IMPL-RECORD-ELEMENTS
    OptImplRecordElements                                                  = 4167,
    /// OPTION
    Option                                                                 = 5377,
    /// OPTION-ARGUMENT
    OptionArgument                                                         = 1084,
    /// OPTION-KIND
    OptionKind                                                             = 5230,
    /// OPTION-NAME
    OptionName                                                             = 5824,
    /// OPTIONAL
    Optional                                                               = 5951,
    /// OPTIONAL-ELEMENTS
    OptionalElements                                                       = 279,
    /// OPTIONS
    Options                                                                = 785,
    /// ORDERED-ELEMENTS
    OrderedElements                                                        = 4927,
    /// ORDERED-MASTER
    OrderedMaster                                                          = 4202,
    /// ORDERED-MASTER-LIST
    OrderedMasterList                                                      = 2651,
    /// ORIGIN
    Origin                                                                 = 3318,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 5683,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 3149,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 5642,
    /// OS-TASK-PROXY-REF
    OsTaskProxyRef                                                         = 5582,
    /// OUT-MEASUREMENT-SET
    OutMeasurementSet                                                      = 630,
    /// OUT-MEASURMENT-SET
    OutMeasurmentSet                                                       = 4622,
    /// OUTER-PORT-REF
    OuterPortRef                                                           = 4148,
    /// OUTER-PORTS
    OuterPorts                                                             = 3798,
    /// OUTGOING-CALLBACKS
    OutgoingCallbacks                                                      = 138,
    /// P
    P                                                                      = 1517,
    /// P-2-MAX
    P2Max                                                                  = 3610,
    /// P-2-SERVER-MAX
    P2ServerMax                                                            = 5764,
    /// P-2-STAR-SERVER-MAX
    P2StarServerMax                                                        = 4718,
    /// P-2-TIMING
    P2Timing                                                               = 711,
    /// P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    PModeGroupInAtomicSwcInstanceRef                                       = 5496,
    /// P-PORT-IN-COMPOSITION-INSTANCE-REF
    PPortInCompositionInstanceRef                                          = 4764,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 3499,
    /// P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    PPortPrototypeInExecutableIref                                         = 1628,
    /// P-PORT-PROTOTYPE-IREF
    PPortPrototypeIref                                                     = 1610,
    /// P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF
    PTriggerInAtomicSwcTypeInstanceRef                                     = 3202,
    /// PACKAGE-REF
    PackageRef                                                             = 2066,
    /// PACKAGER-ID
    PackagerId                                                             = 4897,
    /// PACKAGER-SIGNATURE-REF
    PackagerSignatureRef                                                   = 2410,
    /// PACKING-BYTE-ORDER
    PackingByteOrder                                                       = 2129,
    /// PADDING-ACTIVATION
    PaddingActivation                                                      = 5402,
    /// PADDING-VALUE
    PaddingValue                                                           = 3920,
    /// PARAMETER
    Parameter                                                              = 1333,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 4153,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REF
    ParameterAccessedForDebugRef                                           = 3856,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REFS
    ParameterAccessedForDebugRefs                                          = 129,
    /// PARAMETER-ACCESSS
    ParameterAccesss                                                       = 2496,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 1577,
    /// PARAMETER-DATA-PROTOTYPE-IREF
    ParameterDataPrototypeIref                                             = 2277,
    /// PARAMETER-ELEMENT-ACCESS
    ParameterElementAccess                                                 = 401,
    /// PARAMETER-ID
    ParameterId                                                            = 1845,
    /// PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT
    ParameterImplementationDataTypeElement                                 = 215,
    /// PARAMETER-INSTANCE
    ParameterInstance                                                      = 3065,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 3805,
    /// PARAMETER-POLICYS
    ParameterPolicys                                                       = 2432,
    /// PARAMETER-PORT-ANNOTATION
    ParameterPortAnnotation                                                = 4895,
    /// PARAMETER-PORT-ANNOTATIONS
    ParameterPortAnnotations                                               = 3367,
    /// PARAMETER-PROVIDE-COM-SPEC
    ParameterProvideComSpec                                                = 2630,
    /// PARAMETER-REF
    ParameterRef                                                           = 358,
    /// PARAMETER-REQUIRE-COM-SPEC
    ParameterRequireComSpec                                                = 3785,
    /// PARAMETER-SIZE
    ParameterSize                                                          = 491,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 3931,
    /// PARAMETER-VALUES
    ParameterValues                                                        = 1806,
    /// PARAMETERS
    Parameters                                                             = 4426,
    /// PARENT-CATEGORY
    ParentCategory                                                         = 2170,
    /// PARENT-SHORT-LABEL
    ParentShortLabel                                                       = 3008,
    /// PARTIAL-NETWORK-REF
    PartialNetworkRef                                                      = 632,
    /// PARTIAL-NETWORK-REFS
    PartialNetworkRefs                                                     = 5323,
    /// PARTICIPATING-J-1939-CLUSTER-REF
    ParticipatingJ1939ClusterRef                                           = 2835,
    /// PARTICIPATING-J-1939-CLUSTER-REFS
    ParticipatingJ1939ClusterRefs                                          = 157,
    /// PARTITION-REF
    PartitionRef                                                           = 5849,
    /// PARTITIONS
    Partitions                                                             = 2394,
    /// PARTS
    Parts                                                                  = 715,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 4269,
    /// PATH
    Path                                                                   = 2478,
    /// PATH-MTU-DISCOVERY-ENABLED
    PathMtuDiscoveryEnabled                                                = 1256,
    /// PATH-MTU-ENABLED
    PathMtuEnabled                                                         = 3975,
    /// PATH-MTU-TIMEOUT
    PathMtuTimeout                                                         = 1534,
    /// PATTERN
    Pattern                                                                = 2701,
    /// PATTERN-JITTER
    PatternJitter                                                          = 4769,
    /// PATTERN-LENGTH
    PatternLength                                                          = 3868,
    /// PATTERN-PERIOD
    PatternPeriod                                                          = 3602,
    /// PAYLOAD-BYTE-PATTERN-RULE
    PayloadBytePatternRule                                                 = 4607,
    /// PAYLOAD-BYTE-PATTERN-RULE-PART
    PayloadBytePatternRulePart                                             = 3095,
    /// PAYLOAD-BYTE-PATTERN-RULE-PARTS
    PayloadBytePatternRuleParts                                            = 1203,
    /// PAYLOAD-BYTE-PATTERN-RULES
    PayloadBytePatternRules                                                = 2356,
    /// PAYLOAD-LENGTH
    PayloadLength                                                          = 3650,
    /// PAYLOAD-LENGTH-STATIC
    PayloadLengthStatic                                                    = 2891,
    /// PAYLOAD-PREAMBLE-INDICATOR
    PayloadPreambleIndicator                                               = 3328,
    /// PAYLOAD-REF
    PayloadRef                                                             = 2883,
    /// PAYLOAD-TYPE
    PayloadType                                                            = 4334,
    /// PDELAY-LATENCY-THRESHOLD
    PdelayLatencyThreshold                                                 = 5568,
    /// PDELAY-REQUEST-PERIOD
    PdelayRequestPeriod                                                    = 1552,
    /// PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT
    PdelayRespAndRespFollowUpTimeout                                       = 1978,
    /// PDELAY-RESPONSE-ENABLED
    PdelayResponseEnabled                                                  = 5262,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 1655,
    /// PDU-ACTIVATION-ROUTING-GROUPS
    PduActivationRoutingGroups                                             = 4584,
    /// PDU-COLLECTION-MAX-BUFFER-SIZE
    PduCollectionMaxBufferSize                                             = 3413,
    /// PDU-COLLECTION-PDU-TIMEOUT
    PduCollectionPduTimeout                                                = 914,
    /// PDU-COLLECTION-SEMANTICS
    PduCollectionSemantics                                                 = 1129,
    /// PDU-COLLECTION-TIMEOUT
    PduCollectionTimeout                                                   = 2261,
    /// PDU-COLLECTION-TRIGGER
    PduCollectionTrigger                                                   = 3560,
    /// PDU-COUNTER-SIZE
    PduCounterSize                                                         = 373,
    /// PDU-COUNTER-START-POSITION
    PduCounterStartPosition                                                = 1673,
    /// PDU-COUNTER-THRESHOLD
    PduCounterThreshold                                                    = 866,
    /// PDU-COUNTERS
    PduCounters                                                            = 2352,
    /// PDU-MAX-LENGTH
    PduMaxLength                                                           = 4818,
    /// PDU-POOLS
    PduPools                                                               = 1909,
    /// PDU-REF
    PduRef                                                                 = 1369,
    /// PDU-REPLICATION-VOTING
    PduReplicationVoting                                                   = 1468,
    /// PDU-REPLICATIONS
    PduReplications                                                        = 2911,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 2075,
    /// PDU-TO-FRAME-MAPPINGS
    PduToFrameMappings                                                     = 5435,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 2939,
    /// PDU-TRIGGERING-REF
    PduTriggeringRef                                                       = 4310,
    /// PDU-TRIGGERING-REF-CONDITIONAL
    PduTriggeringRefConditional                                            = 2692,
    /// PDU-TRIGGERING-REFS
    PduTriggeringRefs                                                      = 1097,
    /// PDU-TRIGGERINGS
    PduTriggerings                                                         = 5082,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 4309,
    /// PDUR-TP-CHUNK-SIZE
    PdurTpChunkSize                                                        = 3656,
    /// PDUS
    Pdus                                                                   = 479,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 532,
    /// PER-INSTANCE-MEMORY-REF
    PerInstanceMemoryRef                                                   = 5763,
    /// PER-INSTANCE-MEMORY-SIZE
    PerInstanceMemorySize                                                  = 5621,
    /// PER-INSTANCE-MEMORY-SIZES
    PerInstanceMemorySizes                                                 = 1647,
    /// PER-INSTANCE-MEMORYS
    PerInstanceMemorys                                                     = 2660,
    /// PER-INSTANCE-PARAMETER-REF
    PerInstanceParameterRef                                                = 3441,
    /// PER-INSTANCE-PARAMETERS
    PerInstanceParameters                                                  = 5184,
    /// PER-STATE-TIMEOUT
    PerStateTimeout                                                        = 2842,
    /// PER-STATE-TIMEOUTS
    PerStateTimeouts                                                       = 2446,
    /// PERIOD
    Period                                                                 = 4704,
    /// PERIOD-BEGIN
    PeriodBegin                                                            = 73,
    /// PERIOD-END
    PeriodEnd                                                              = 721,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 3657,
    /// PERIODIC-RATE-CATEGORY
    PeriodicRateCategory                                                   = 4937,
    /// PERIODIC-RATES
    PeriodicRates                                                          = 3031,
    /// PERIODIC-RESPONSE-UUDT-REF
    PeriodicResponseUudtRef                                                = 3630,
    /// PERIODIC-RESPONSE-UUDT-REFS
    PeriodicResponseUudtRefs                                               = 1671,
    /// PERMISSIBLE-SIGNAL-PATH
    PermissibleSignalPath                                                  = 3399,
    /// PERMISSION-CERTIFICATE-AUTHORITY-REF
    PermissionCertificateAuthorityRef                                      = 2174,
    /// PERMIT-MULTIPLE-REFERENCES-TO-EE
    PermitMultipleReferencesToEe                                           = 3415,
    /// PERSISTENCY-ACCESS
    PersistencyAccess                                                      = 5807,
    /// PERSISTENCY-CENTRAL-STORAGE-URI
    PersistencyCentralStorageUri                                           = 2508,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 968,
    /// PERSISTENCY-DATA-PROVIDED-COM-SPEC
    PersistencyDataProvidedComSpec                                         = 1253,
    /// PERSISTENCY-DATA-REQUIRED-COM-SPEC
    PersistencyDataRequiredComSpec                                         = 3451,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-REF
    PersistencyDeploymentElementRef                                        = 2905,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 3219,
    /// PERSISTENCY-DEPLOYMENT-REF
    PersistencyDeploymentRef                                               = 1711,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 4545,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 2762,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 2479,
    /// PERSISTENCY-DEPLOYMENT-URI
    PersistencyDeploymentUri                                               = 1354,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 5928,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 4738,
    /// PERSISTENCY-FILE-ARRAY-REF
    PersistencyFileArrayRef                                                = 625,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 3865,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 364,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 2702,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 5501,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 4466,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 4157,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 2999,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 570,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 725,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 5959,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 4651,
    /// PERSISTENCY-PORT-PROTOTYPE-IREF
    PersistencyPortPrototypeIref                                           = 2567,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 2879,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 4293,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 115,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 941,
    /// PERSISTENCY-PROVIDED-COM-SPEC
    PersistencyProvidedComSpec                                             = 4296,
    /// PERSISTENCY-REDUNDANCY-CRC
    PersistencyRedundancyCrc                                               = 3358,
    /// PERSISTENCY-REDUNDANCY-HASH
    PersistencyRedundancyHash                                              = 1241,
    /// PERSISTENCY-REDUNDANCY-M-OUT-OF-N
    PersistencyRedundancyMOutOfN                                           = 1500,
    /// PERSISTENT-STORAGE
    PersistentStorage                                                      = 1884,
    /// PGN
    Pgn                                                                    = 305,
    /// PHM-ACTION
    PhmAction                                                              = 4371,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 3159,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 1348,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 3593,
    /// PHM-CHECKPOINT-IREF
    PhmCheckpointIref                                                      = 429,
    /// PHM-CONTRIBUTION-REF
    PhmContributionRef                                                     = 5016,
    /// PHM-CONTRIBUTION-REFS
    PhmContributionRefs                                                    = 4527,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 1508,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 3860,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 2057,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 189,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 5829,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 2792,
    /// PHM-RULE
    PhmRule                                                                = 2216,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 5123,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 3077,
    /// PHYS-CONSTRS
    PhysConstrs                                                            = 4396,
    /// PHYSICAL-CHANNEL-REF
    PhysicalChannelRef                                                     = 2857,
    /// PHYSICAL-CHANNEL-REF-CONDITIONAL
    PhysicalChannelRefConditional                                          = 4911,
    /// PHYSICAL-CHANNEL-REFS
    PhysicalChannelRefs                                                    = 3940,
    /// PHYSICAL-CHANNELS
    PhysicalChannels                                                       = 1478,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 3279,
    /// PHYSICAL-DIMENSION-MAPPING
    PhysicalDimensionMapping                                               = 1795,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 1035,
    /// PHYSICAL-DIMENSION-MAPPINGS
    PhysicalDimensionMappings                                              = 2770,
    /// PHYSICAL-DIMENSION-REF
    PhysicalDimensionRef                                                   = 4191,
    /// PHYSICAL-LAYER-TYPE
    PhysicalLayerType                                                      = 3979,
    /// PHYSICAL-PROPS
    PhysicalProps                                                          = 5623,
    /// PHYSICAL-REQUEST-REF
    PhysicalRequestRef                                                     = 3959,
    /// PID
    Pid                                                                    = 3554,
    /// PID-REF
    PidRef                                                                 = 2461,
    /// PID-REFS
    PidRefs                                                                = 307,
    /// PID-SIZE
    PidSize                                                                = 5190,
    /// PIN-NUMBER
    PinNumber                                                              = 5569,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 5537,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 1707,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 2073,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 578,
    /// PLATFORM-TIME-BASES
    PlatformTimeBases                                                      = 4681,
    /// PLCA-LOCAL-NODE-COUNT
    PlcaLocalNodeCount                                                     = 4299,
    /// PLCA-LOCAL-NODE-ID
    PlcaLocalNodeId                                                        = 1068,
    /// PLCA-MAX-BURST-COUNT
    PlcaMaxBurstCount                                                      = 2796,
    /// PLCA-MAX-BURST-TIMER
    PlcaMaxBurstTimer                                                      = 3481,
    /// PLCA-PROPS
    PlcaProps                                                              = 4357,
    /// PLCA-TRANSMIT-OPPORTUNITY-TIMER
    PlcaTransmitOpportunityTimer                                           = 3991,
    /// PN-RESET-TIME
    PnResetTime                                                            = 2704,
    /// PN-RESET-TIMER
    PnResetTimer                                                           = 80,
    /// PNC-CLUSTER-VECTOR-LENGTH
    PncClusterVectorLength                                                 = 2065,
    /// PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    PncConsumedProvidedServiceInstanceGroups                               = 5771,
    /// PNC-FILTER-ARRAY-MASK
    PncFilterArrayMask                                                     = 2966,
    /// PNC-FILTER-ARRAY-MASKS
    PncFilterArrayMasks                                                    = 1882,
    /// PNC-FILTER-DATA-MASK
    PncFilterDataMask                                                      = 223,
    /// PNC-GATEWAY-TYPE
    PncGatewayType                                                         = 2575,
    /// PNC-GROUP-REF
    PncGroupRef                                                            = 5030,
    /// PNC-GROUP-REFS
    PncGroupRefs                                                           = 1020,
    /// PNC-IDENTIFIER
    PncIdentifier                                                          = 1152,
    /// PNC-MAPPING
    PncMapping                                                             = 1651,
    /// PNC-MAPPING-REF
    PncMappingRef                                                          = 4784,
    /// PNC-MAPPING-REFS
    PncMappingRefs                                                         = 5380,
    /// PNC-MAPPINGS
    PncMappings                                                            = 1972,
    /// PNC-NM-REQUEST
    PncNmRequest                                                           = 2163,
    /// PNC-PDUR-GROUP-REF
    PncPdurGroupRef                                                        = 2411,
    /// PNC-PDUR-GROUP-REFS
    PncPdurGroupRefs                                                       = 291,
    /// PNC-PREPARE-SLEEP-TIMER
    PncPrepareSleepTimer                                                   = 3988,
    /// PNC-SYNCHRONOUS-WAKEUP
    PncSynchronousWakeup                                                   = 3727,
    /// PNC-VECTOR-LENGTH
    PncVectorLength                                                        = 4456,
    /// PNC-VECTOR-OFFSET
    PncVectorOffset                                                        = 710,
    /// PNC-WAKEUP-CAN-ID
    PncWakeupCanId                                                         = 2554,
    /// PNC-WAKEUP-CAN-ID-EXTENDED
    PncWakeupCanIdExtended                                                 = 5156,
    /// PNC-WAKEUP-CAN-ID-MASK
    PncWakeupCanIdMask                                                     = 262,
    /// PNC-WAKEUP-DATA-MASK
    PncWakeupDataMask                                                      = 4529,
    /// PNC-WAKEUP-DLC
    PncWakeupDlc                                                           = 5738,
    /// PNC-WAKEUP-ENABLE
    PncWakeupEnable                                                        = 220,
    /// POLICY
    Policy                                                                 = 2337,
    /// POLICY-ACTION
    PolicyAction                                                           = 4975,
    /// PORT-ADDRESS
    PortAddress                                                            = 4744,
    /// PORT-API-OPTION
    PortApiOption                                                          = 1551,
    /// PORT-API-OPTIONS
    PortApiOptions                                                         = 872,
    /// PORT-ARG-VALUES
    PortArgValues                                                          = 232,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 2045,
    /// PORT-BLUEPRINT-INIT-VALUE
    PortBlueprintInitValue                                                 = 4863,
    /// PORT-BLUEPRINT-MAPPING
    PortBlueprintMapping                                                   = 2788,
    /// PORT-BLUEPRINT-MAPPINGS
    PortBlueprintMappings                                                  = 1625,
    /// PORT-BLUEPRINT-REF
    PortBlueprintRef                                                       = 877,
    /// PORT-CONFIGS
    PortConfigs                                                            = 5417,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINT
    PortDefinedArgumentBlueprint                                           = 2082,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINTS
    PortDefinedArgumentBlueprints                                          = 1266,
    /// PORT-DEFINED-ARGUMENT-VALUE
    PortDefinedArgumentValue                                               = 235,
    /// PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS
    PortElementToComResourceMappings                                       = 2837,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 3901,
    /// PORT-GROUP
    PortGroup                                                              = 2325,
    /// PORT-GROUPS
    PortGroups                                                             = 5164,
    /// PORT-INSTANTIATION-BEHAVIOR
    PortInstantiationBehavior                                              = 2002,
    /// PORT-INTERFACE-BLUEPRINT-MAPPING
    PortInterfaceBlueprintMapping                                          = 3812,
    /// PORT-INTERFACE-BLUEPRINT-REF
    PortInterfaceBlueprintRef                                              = 974,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 4305,
    /// PORT-INTERFACE-MAPPINGS
    PortInterfaceMappings                                                  = 820,
    /// PORT-INTERFACE-REF
    PortInterfaceRef                                                       = 3916,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 3838,
    /// PORT-NUMBER
    PortNumber                                                             = 4344,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 247,
    /// PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE
    PortPrototypeBlueprintInitValue                                        = 4778,
    /// PORT-PROTOTYPE-BLUEPRINT-MAPPING
    PortPrototypeBlueprintMapping                                          = 41,
    /// PORT-PROTOTYPE-BLUEPRINT-REF
    PortPrototypeBlueprintRef                                              = 2113,
    /// PORT-PROTOTYPE-IREF
    PortPrototypeIref                                                      = 1796,
    /// PORT-PROTOTYPE-PROPS
    PortPrototypeProps                                                     = 168,
    /// PORT-PROTOTYPE-REF
    PortPrototypeRef                                                       = 5404,
    /// PORT-PROTOTYPE-REF-CONDITIONAL
    PortPrototypeRefConditional                                            = 3041,
    /// PORT-PROTOTYPE-SLUG-FRAGMENT
    PortPrototypeSlugFragment                                              = 1376,
    /// PORT-REF
    PortRef                                                                = 2303,
    /// PORT-SCHEDULER
    PortScheduler                                                          = 500,
    /// PORTS
    Ports                                                                  = 4013,
    /// POSITION
    Position                                                               = 4761,
    /// POSITION-IN-TABLE
    PositionInTable                                                        = 4120,
    /// POSITIVE-INTEGER-VALUE-VARIATION-POINT
    PositiveIntegerValueVariationPoint                                     = 5078,
    /// POSSIBLE-AP-ERROR-REF
    PossibleApErrorRef                                                     = 3573,
    /// POSSIBLE-AP-ERROR-REFS
    PossibleApErrorRefs                                                    = 4056,
    /// POSSIBLE-AP-ERROR-SET-REF
    PossibleApErrorSetRef                                                  = 549,
    /// POSSIBLE-AP-ERROR-SET-REFS
    PossibleApErrorSetRefs                                                 = 113,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 2697,
    /// POSSIBLE-ERROR-REACTIONS
    PossibleErrorReactions                                                 = 3524,
    /// POSSIBLE-ERROR-REF
    PossibleErrorRef                                                       = 640,
    /// POSSIBLE-ERROR-REFS
    PossibleErrorRefs                                                      = 5702,
    /// POSSIBLE-ERRORS
    PossibleErrors                                                         = 4015,
    /// POST-BUILD-CHANGEABLE
    PostBuildChangeable                                                    = 5765,
    /// POST-BUILD-VALUE-ACCESS-REF
    PostBuildValueAccessRef                                                = 5498,
    /// POST-BUILD-VARIANT-CONDITION
    PostBuildVariantCondition                                              = 3515,
    /// POST-BUILD-VARIANT-CONDITIONS
    PostBuildVariantConditions                                             = 703,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 474,
    /// POST-BUILD-VARIANT-CRITERION-VALUE
    PostBuildVariantCriterionValue                                         = 5420,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 1804,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF
    PostBuildVariantCriterionValueSetRef                                   = 3763,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS
    PostBuildVariantCriterionValueSetRefs                                  = 5095,
    /// POST-BUILD-VARIANT-CRITERION-VALUES
    PostBuildVariantCriterionValues                                        = 88,
    /// POST-BUILD-VARIANT-MULTIPLICITY
    PostBuildVariantMultiplicity                                           = 1750,
    /// POST-BUILD-VARIANT-SUPPORT
    PostBuildVariantSupport                                                = 2361,
    /// POST-BUILD-VARIANT-USED
    PostBuildVariantUsed                                                   = 2299,
    /// POST-BUILD-VARIANT-VALUE
    PostBuildVariantValue                                                  = 5243,
    /// POST-VERIFICATION-REBOOT
    PostVerificationReboot                                                 = 1928,
    /// POWER-DOWN-TIME
    PowerDownTime                                                          = 5034,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 3688,
    /// PRE-ACTIVATE-IREF
    PreActivateIref                                                        = 4661,
    /// PRE-ACTIVATE-IREFS
    PreActivateIrefs                                                       = 2364,
    /// PRE-ACTIVATE-REF
    PreActivateRef                                                         = 2175,
    /// PRE-ACTIVATE-REFS
    PreActivateRefs                                                        = 2433,
    /// PRE-ACTIVATION-REBOOT
    PreActivationReboot                                                    = 3523,
    /// PRE-MAPPING
    PreMapping                                                             = 4823,
    /// PRE-SHARED-KEY-REF
    PreSharedKeyRef                                                        = 828,
    /// PRECONFIGURED-CONFIGURATION-REF
    PreconfiguredConfigurationRef                                          = 3586,
    /// PRECONFIGURED-CONFIGURATION-REFS
    PreconfiguredConfigurationRefs                                         = 5350,
    /// PREDECESSOR-ACTION-REF
    PredecessorActionRef                                                   = 1298,
    /// PREDECESSOR-ACTION-REFS
    PredecessorActionRefs                                                  = 2599,
    /// PREDECESSOR-FIFO-REF
    PredecessorFifoRef                                                     = 3516,
    /// PREDECESSOR-REF
    PredecessorRef                                                         = 1071,
    /// PREDECESSOR-REFS
    PredecessorRefs                                                        = 3665,
    /// PREDEFINED-TEXT
    PredefinedText                                                         = 3375,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 3190,
    /// PREEMPTABILITY
    Preemptability                                                         = 2711,
    /// PREFIX
    Prefix                                                                 = 972,
    /// PREFIX-REF
    PrefixRef                                                              = 11,
    /// PREPARE-ROLLBACK
    PrepareRollback                                                        = 4055,
    /// PREPARE-UPDATE
    PrepareUpdate                                                          = 3618,
    /// PRESTORAGE-FREEZE-FRAME
    PrestorageFreezeFrame                                                  = 5430,
    /// PRESTORED-FREEZEFRAME-STORED-IN-NVM
    PrestoredFreezeframeStoredInNvm                                        = 3661,
    /// PREVIOUS-DATA-TYPE-REF
    PreviousDataTypeRef                                                    = 5076,
    /// PREVIOUS-EXECUTABLE-VERSION
    PreviousExecutableVersion                                              = 2205,
    /// PRIMITIVE
    Primitive                                                              = 4649,
    /// PRIMITIVE-ATTRIBUTE-CONDITION
    PrimitiveAttributeCondition                                            = 17,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 4405,
    /// PRIMITIVE-FAMILY
    PrimitiveFamily                                                        = 4561,
    /// PRIMITIVE-TYPE-MAPPINGS
    PrimitiveTypeMappings                                                  = 1938,
    /// PRIORITY
    Priority                                                               = 2904,
    /// PRIORITY-ID
    PriorityId                                                             = 440,
    /// PRIORITYS
    Prioritys                                                              = 2818,
    /// PRIVACY-LEVEL
    PrivacyLevel                                                           = 5471,
    /// PRM
    Prm                                                                    = 57,
    /// PRM-CHAR
    PrmChar                                                                = 5591,
    /// PRM-UNIT
    PrmUnit                                                                = 3589,
    /// PRMS
    Prms                                                                   = 2725,
    /// PROCESS
    Process                                                                = 441,
    /// PROCESS-ARGUMENT
    ProcessArgument                                                        = 5855,
    /// PROCESS-ARGUMENTS
    ProcessArguments                                                       = 363,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 4943,
    /// PROCESS-DESIGN-REF
    ProcessDesignRef                                                       = 3469,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 4638,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 5868,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS
    ProcessDesignToMachineDesignMappings                                   = 1118,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 3651,
    /// PROCESS-MODE-IREF
    ProcessModeIref                                                        = 5937,
    /// PROCESS-MODE-MACHINE
    ProcessModeMachine                                                     = 371,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 4151,
    /// PROCESS-REF
    ProcessRef                                                             = 3741,
    /// PROCESS-STATE-IREF
    ProcessStateIref                                                       = 1957,
    /// PROCESS-STATE-MACHINE
    ProcessStateMachine                                                    = 4257,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 4102,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 2344,
    /// PROCESS-TO-MACHINE-MAPPINGS
    ProcessToMachineMappings                                               = 1991,
    /// PROCESSING-KIND
    ProcessingKind                                                         = 3995,
    /// PROCESSING-STYLE
    ProcessingStyle                                                        = 4096,
    /// PROCESSING-UNIT-REF
    ProcessingUnitRef                                                      = 236,
    /// PROCESSOR
    Processor                                                              = 1045,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 5612,
    /// PROCESSOR-MODE
    ProcessorMode                                                          = 3101,
    /// PROCESSOR-SPEED
    ProcessorSpeed                                                         = 3152,
    /// PROCESSORS
    Processors                                                             = 175,
    /// PRODUCT-ID
    ProductId                                                              = 1810,
    /// PRODUCT-RELEASE
    ProductRelease                                                         = 5143,
    /// PROFILE-BEHAVIOR
    ProfileBehavior                                                        = 4578,
    /// PROFILE-NAME
    ProfileName                                                            = 2675,
    /// PROG-CODE
    ProgCode                                                               = 3693,
    /// PROGRAMMING-LANGUAGE
    ProgrammingLanguage                                                    = 1582,
    /// PROP-SEG
    PropSeg                                                                = 1115,
    /// PROPABILITY
    Propability                                                            = 5592,
    /// PROPAGATION-DELAY
    PropagationDelay                                                       = 2743,
    /// PROPERTYS
    Propertys                                                              = 4912,
    /// PROPS
    Props                                                                  = 1135,
    /// PROTOCOL
    Protocol                                                               = 1185,
    /// PROTOCOL-KIND
    ProtocolKind                                                           = 3529,
    /// PROTOCOL-NAME
    ProtocolName                                                           = 3813,
    /// PROTOCOL-VERSION
    ProtocolVersion                                                        = 4852,
    /// PROVIDE-RESOURCES
    ProvideResources                                                       = 1242,
    /// PROVIDED-CLIENT-SERVER-ENTRYS
    ProvidedClientServerEntrys                                             = 4073,
    /// PROVIDED-COM-SPECS
    ProvidedComSpecs                                                       = 4470,
    /// PROVIDED-DATA-REF
    ProvidedDataRef                                                        = 1476,
    /// PROVIDED-DATAS
    ProvidedDatas                                                          = 1956,
    /// PROVIDED-DDS-EVENT-QOS-PROPS
    ProvidedDdsEventQosProps                                               = 3185,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 5401,
    /// PROVIDED-ENTRYS
    ProvidedEntrys                                                         = 4962,
    /// PROVIDED-EVENT-GROUPS
    ProvidedEventGroups                                                    = 1828,
    /// PROVIDED-INTERFACE-TREF
    ProvidedInterfaceTref                                                  = 5608,
    /// PROVIDED-MEMORY-REF
    ProvidedMemoryRef                                                      = 3702,
    /// PROVIDED-MODE-GROUP-REF
    ProvidedModeGroupRef                                                   = 2548,
    /// PROVIDED-MODE-GROUPS
    ProvidedModeGroups                                                     = 4373,
    /// PROVIDED-OUTER-PORT-REF
    ProvidedOuterPortRef                                                   = 5515,
    /// PROVIDED-PORT-PROTOTYPE-IREF
    ProvidedPortPrototypeIref                                              = 2127,
    /// PROVIDED-REQUIRED-INTERFACE-TREF
    ProvidedRequiredInterfaceTref                                          = 5780,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 3017,
    /// PROVIDED-SERVICE-INSTANCE-REF
    ProvidedServiceInstanceRef                                             = 2038,
    /// PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL
    ProvidedServiceInstanceRefConditional                                  = 5641,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 46,
    /// PROVIDED-SERVICE-INSTANCES
    ProvidedServiceInstances                                               = 4969,
    /// PROVIDED-SERVICE-PORT-IREF
    ProvidedServicePortIref                                                = 4528,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 1668,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 2357,
    /// PROVIDER-IREF
    ProviderIref                                                           = 4685,
    /// PROVIDER-REF
    ProviderRef                                                            = 4695,
    /// PROVIDER-SOFTWARE-CLUSTER-REF
    ProviderSoftwareClusterRef                                             = 5366,
    /// PROVIEDE-DATA-REF
    ProviedeDataRef                                                        = 4530,
    /// PSK-IDENTITY
    PskIdentity                                                            = 2783,
    /// PSK-IDENTITY-HINT
    PskIdentityHint                                                        = 3048,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 456,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS
    PskIdentityToKeySlotMappings                                           = 5662,
    /// PUBLISHER
    Publisher                                                              = 2240,
    /// PULSE-TEST
    PulseTest                                                              = 5587,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 87,
    /// PURPOSE-OF-UPDATE-REF
    PurposeOfUpdateRef                                                     = 2335,
    /// PWM-L
    PwmL                                                                   = 1862,
    /// PWM-O
    PwmO                                                                   = 614,
    /// PWM-S
    PwmS                                                                   = 1219,
    /// QOS-PROFILE
    QosProfile                                                             = 1888,
    /// QUEUE-LENGTH
    QueueLength                                                            = 2558,
    /// QUEUE-SIZE
    QueueSize                                                              = 1103,
    /// QUEUED-RECEIVER-COM-SPEC
    QueuedReceiverComSpec                                                  = 3078,
    /// QUEUED-SENDER-COM-SPEC
    QueuedSenderComSpec                                                    = 164,
    /// R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    RModeGroupInAtomicSwcInstanceRef                                       = 1299,
    /// R-PORT-IN-COMPOSITION-INSTANCE-REF
    RPortInCompositionInstanceRef                                          = 29,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 5237,
    /// R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    RPortPrototypeInExecutableIref                                         = 859,
    /// R-PORT-PROTOTYPE-IREF
    RPortPrototypeIref                                                     = 2047,
    /// R-PORT-PROTOTYPE-PROPS
    RPortPrototypeProps                                                    = 2070,
    /// R-PORT-PROTOTYPE-REF
    RPortPrototypeRef                                                      = 2276,
    /// R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF
    RTriggerInAtomicSwcInstanceRef                                         = 5133,
    /// RAM-BLOCK
    RamBlock                                                               = 4726,
    /// RAM-BLOCK-INIT-VALUE
    RamBlockInitValue                                                      = 1794,
    /// RAM-BLOCK-STATUS-CONTROL
    RamBlockStatusControl                                                  = 702,
    /// RAM-LOCATION-REF
    RamLocationRef                                                         = 2404,
    /// RANGE-MAX
    RangeMax                                                               = 1448,
    /// RANGE-MIN
    RangeMin                                                               = 4807,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 5071,
    /// RATE-BASED-MONITORED-EVENT-REF
    RateBasedMonitoredEventRef                                             = 3259,
    /// RATE-CORRECTION-MEASUREMENT-DURATION
    RateCorrectionMeasurementDuration                                      = 3585,
    /// RATE-CORRECTION-OUT
    RateCorrectionOut                                                      = 2084,
    /// RATE-CORRECTIONS-PER-MEASUREMENT-DURATION
    RateCorrectionsPerMeasurementDuration                                  = 5505,
    /// RATE-DEVIATION-MEASUREMENT-DURATION
    RateDeviationMeasurementDuration                                       = 4233,
    /// RATE-LIMITATION-FILTERS
    RateLimitationFilters                                                  = 376,
    /// RATE-POLICYS
    RatePolicys                                                            = 1010,
    /// RATIO-KIND
    RatioKind                                                              = 3366,
    /// RATIONALE
    Rationale                                                              = 4665,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 4260,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 2483,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 165,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 1218,
    /// RAW-DATA-STREAM-INTERFACE-REF
    RawDataStreamInterfaceRef                                              = 5375,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 25,
    /// RAW-DATA-STREAM-REF
    RawDataStreamRef                                                       = 3329,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 420,
    /// REACTION-CODE
    ReactionCode                                                           = 4280,
    /// READ
    Read                                                                   = 5875,
    /// READ-CLASS-REF
    ReadClassRef                                                           = 1604,
    /// READ-DATA-CLASS-REF
    ReadDataClassRef                                                       = 5945,
    /// READ-DTC-INFORMATION-CLASS-REF
    ReadDtcInformationClassRef                                             = 1844,
    /// READ-LOCAL-VARIABLES
    ReadLocalVariables                                                     = 5142,
    /// READ-NV-DATA
    ReadNvData                                                             = 1501,
    /// READ-SCALING-DATA-CLASS-REF
    ReadScalingDataClassRef                                                = 1420,
    /// READER-ENTITY-ID
    ReaderEntityId                                                         = 1829,
    /// READONLY
    Readonly                                                               = 16,
    /// REASON
    Reason                                                                 = 1208,
    /// RECEIVE-ACTIVITY
    ReceiveActivity                                                        = 293,
    /// RECEIVE-WINDOW-MIN
    ReceiveWindowMin                                                       = 4694,
    /// RECEIVED-DATA-REF
    ReceivedDataRef                                                        = 682,
    /// RECEIVER-ANNOTATION
    ReceiverAnnotation                                                     = 256,
    /// RECEIVER-CAPABILITY
    ReceiverCapability                                                     = 4706,
    /// RECEIVER-INTENT
    ReceiverIntent                                                         = 4166,
    /// RECEIVER-IREF
    ReceiverIref                                                           = 679,
    /// RECEIVER-IREFS
    ReceiverIrefs                                                          = 5645,
    /// RECEIVER-REF
    ReceiverRef                                                            = 1556,
    /// RECEIVER-REFS
    ReceiverRefs                                                           = 3486,
    /// RECEPTION-POLICYS
    ReceptionPolicys                                                       = 4517,
    /// RECEPTION-PROPS
    ReceptionProps                                                         = 2211,
    /// RECOMMENDED-CONFIGURATION-REF
    RecommendedConfigurationRef                                            = 393,
    /// RECOMMENDED-CONFIGURATION-REFS
    RecommendedConfigurationRefs                                           = 5881,
    /// RECORD-ELEMENT-MAPPINGS
    RecordElementMappings                                                  = 5104,
    /// RECORD-ELEMENT-REF
    RecordElementRef                                                       = 84,
    /// RECORD-ELEMENTS
    RecordElements                                                         = 1272,
    /// RECORD-NUMBER
    RecordNumber                                                           = 911,
    /// RECORD-VALUE-SPECIFICATION
    RecordValueSpecification                                               = 184,
    /// RECOVERABLE-IN-SAME-OPERATION-CYCLE
    RecoverableInSameOperationCycle                                        = 4084,
    /// RECOVERY
    Recovery                                                               = 4580,
    /// RECOVERY-ACTION-IREF
    RecoveryActionIref                                                     = 2849,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 2954,
    /// RECOVERY-NOTIFICATION-REF
    RecoveryNotificationRef                                                = 118,
    /// RECOVERY-NOTIFICATION-REFS
    RecoveryNotificationRefs                                               = 3311,
    /// RECOVERY-NOTIFICATION-RETRY
    RecoveryNotificationRetry                                              = 4553,
    /// RECOVERY-NOTIFICATION-TIMEOUT
    RecoveryNotificationTimeout                                            = 2301,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 1188,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 2592,
    /// RECOVERY-VIA-APPLICATION-ACTION-REF
    RecoveryViaApplicationActionRef                                        = 3615,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 2034,
    /// REDUNDANCY
    Redundancy                                                             = 1070,
    /// REDUNDANCY-HANDLINGS
    RedundancyHandlings                                                    = 957,
    /// REENTRANCY-LEVEL
    ReentrancyLevel                                                        = 3139,
    /// REF
    Ref                                                                    = 357,
    /// REF-CALPRM-SET
    RefCalprmSet                                                           = 2754,
    /// REF-MEASUREMENT-SET
    RefMeasurementSet                                                      = 2029,
    /// REFERENCE-BASE
    ReferenceBase                                                          = 5329,
    /// REFERENCE-BASES
    ReferenceBases                                                         = 5405,
    /// REFERENCE-CONDITION
    ReferenceCondition                                                     = 246,
    /// REFERENCE-REF
    ReferenceRef                                                           = 5179,
    /// REFERENCE-TABLE-REF
    ReferenceTableRef                                                      = 4996,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 5299,
    /// REFERENCE-VALUE-REF
    ReferenceValueRef                                                      = 1818,
    /// REFERENCE-VALUE-SPECIFICATION
    ReferenceValueSpecification                                            = 5317,
    /// REFERENCE-VALUES
    ReferenceValues                                                        = 3251,
    /// REFERENCED-BASELINE
    ReferencedBaseline                                                     = 3079,
    /// REFERENCED-TD-EVENT-SWC-REF
    ReferencedTdEventSwcRef                                                = 1898,
    /// REFERENCED-TD-EVENT-VFB-REF
    ReferencedTdEventVfbRef                                                = 1974,
    /// REFERENCES
    References                                                             = 4507,
    /// REFERRABLE-REF
    ReferrableRef                                                          = 4567,
    /// REFILL-AMOUNT
    RefillAmount                                                           = 966,
    /// REFINED-EVENT-IREF
    RefinedEventIref                                                       = 4100,
    /// REFINED-MODULE-DEF-REF
    RefinedModuleDefRef                                                    = 1822,
    /// REG-DOES-NOT-REQUIRE-STABILITYS
    RegDoesNotRequireStabilitys                                            = 2758,
    /// REG-REQUIRES-STABILITYS
    RegRequiresStabilitys                                                  = 1133,
    /// REGENERATED-PRIORITY
    RegeneratedPriority                                                    = 1360,
    /// REGULAR-EXPRESSION
    RegularExpression                                                      = 5129,
    /// REKEY-INTERVAL
    RekeyInterval                                                          = 5051,
    /// RELATED-TRACE-ITEM-REF
    RelatedTraceItemRef                                                    = 1404,
    /// RELATIONS
    Relations                                                              = 5952,
    /// RELATIVE
    Relative                                                               = 3695,
    /// RELATIVE-REPRESENTATION-TIME
    RelativeRepresentationTime                                             = 1504,
    /// RELATIVE-TOLERANCE
    RelativeTolerance                                                      = 1685,
    /// RELEASE-NOTES-REF
    ReleaseNotesRef                                                        = 3970,
    /// RELEASED-TRIGGER-POLICYS
    ReleasedTriggerPolicys                                                 = 2187,
    /// RELEASED-TRIGGER-REF
    ReleasedTriggerRef                                                     = 1455,
    /// RELEASED-TRIGGERS
    ReleasedTriggers                                                       = 1142,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF
    RelevantForDynamicPncMappingRef                                        = 4313,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS
    RelevantForDynamicPncMappingRefs                                       = 3974,
    /// RELIABILITY
    Reliability                                                            = 4972,
    /// REMAPPED-CAN-ID
    RemappedCanId                                                          = 5326,
    /// REMARK
    Remark                                                                 = 2963,
    /// REMOTE-ACCESS-CONTROL-ENABLED
    RemoteAccessControlEnabled                                             = 5461,
    /// REMOTE-ADDRESSS
    RemoteAddresss                                                         = 4957,
    /// REMOTE-CERTIFICATE-REF
    RemoteCertificateRef                                                   = 2935,
    /// REMOTE-CERTIFICATE-REFS
    RemoteCertificateRefs                                                  = 5848,
    /// REMOTE-CLIENT-CONFIG
    RemoteClientConfig                                                     = 3332,
    /// REMOTE-ID
    RemoteId                                                               = 3771,
    /// REMOTE-IP-ADDRESS-REF
    RemoteIpAddressRef                                                     = 3680,
    /// REMOTE-IP-ADDRESS-REFS
    RemoteIpAddressRefs                                                    = 4155,
    /// REMOTE-MULTICAST-CONFIG-REF
    RemoteMulticastConfigRef                                               = 754,
    /// REMOTE-MULTICAST-CONFIG-REFS
    RemoteMulticastConfigRefs                                              = 1925,
    /// REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS
    RemoteMulticastSubscriptionAddresss                                    = 3325,
    /// REMOTE-NETWORK-ENDPOINT-REF
    RemoteNetworkEndpointRef                                               = 2490,
    /// REMOTE-PORT-RANGE-END
    RemotePortRangeEnd                                                     = 3882,
    /// REMOTE-PORT-RANGE-START
    RemotePortRangeStart                                                   = 342,
    /// REMOTE-PORT-REF
    RemotePortRef                                                          = 1547,
    /// REMOTE-SERVER-CONFIG
    RemoteServerConfig                                                     = 4746,
    /// REMOTE-SUBJECT-REF
    RemoteSubjectRef                                                       = 3700,
    /// REMOTE-SUBJECT-REFS
    RemoteSubjectRefs                                                      = 1295,
    /// REMOTE-UNICAST-ADDRESSS
    RemoteUnicastAddresss                                                  = 5109,
    /// REMOTE-UNICAST-CONFIG-REF
    RemoteUnicastConfigRef                                                 = 4210,
    /// REMOTE-UNICAST-CONFIG-REFS
    RemoteUnicastConfigRefs                                                = 3842,
    /// REMOTING-TECHNOLOGY
    RemotingTechnology                                                     = 2286,
    /// REPETITION-PERIOD
    RepetitionPeriod                                                       = 5879,
    /// REPLACE-WITH
    ReplaceWith                                                            = 4940,
    /// REPLAY-PROTECTION
    ReplayProtection                                                       = 1393,
    /// REPLAY-PROTECTION-WINDOW
    ReplayProtectionWindow                                                 = 284,
    /// REPLICA-PDUS-REF
    ReplicaPdusRef                                                         = 3473,
    /// REPLICA-PDUS-REFS
    ReplicaPdusRefs                                                        = 4562,
    /// REPLY-TOPIC-NAME
    ReplyTopicName                                                         = 4389,
    /// REPORT-BEHAVIOR
    ReportBehavior                                                         = 4737,
    /// REPORTABLE-SECURITY-EVENT-REF
    ReportableSecurityEventRef                                             = 925,
    /// REPORTABLE-SECURITY-EVENT-REFS
    ReportableSecurityEventRefs                                            = 3921,
    /// REPORTED-SECURITY-EVENT-IREF
    ReportedSecurityEventIref                                              = 3199,
    /// REPORTING-BEHAVIOR
    ReportingBehavior                                                      = 3173,
    /// REPORTING-PORT-PROTOTYPE-IREF
    ReportingPortPrototypeIref                                             = 4006,
    /// REPOSITORY
    Repository                                                             = 4145,
    /// REPRESENTED-MODEL-ELEMENT-REF
    RepresentedModelElementRef                                             = 3080,
    /// REPRESENTED-PORT-GROUP-REF
    RepresentedPortGroupRef                                                = 3691,
    /// REPRESENTS-VIN
    RepresentsVin                                                          = 3225,
    /// REQUEST-2-SUPPORT
    Request2Support                                                        = 2615,
    /// REQUEST-CONFIGURATIONS
    RequestConfigurations                                                  = 3314,
    /// REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF
    RequestControlOfOnBoardDeviceClassRef                                  = 3088,
    /// REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF
    RequestCurrentPowertrainDiagnosticDataClassRef                         = 2104,
    /// REQUEST-DATA-SIZE
    RequestDataSize                                                        = 2749,
    /// REQUEST-DOWNLOAD-CLASS-REF
    RequestDownloadClassRef                                                = 4865,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF
    RequestEmissionRelatedDtcClassPermanentStatusRef                       = 1382,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-REF
    RequestEmissionRelatedDtcClassRef                                      = 1831,
    /// REQUEST-FILE-TRANSFER-CLASS-REF
    RequestFileTransferClassRef                                            = 5953,
    /// REQUEST-GROUP-REF
    RequestGroupRef                                                        = 1425,
    /// REQUEST-METHOD
    RequestMethod                                                          = 963,
    /// REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF
    RequestOnBoardMonitoringTestResultsClassRef                            = 2308,
    /// REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF
    RequestPowertrainFreezeFrameDataRef                                    = 4367,
    /// REQUEST-RESPONSE-DELAY
    RequestResponseDelay                                                   = 1433,
    /// REQUEST-RESULT
    RequestResult                                                          = 482,
    /// REQUEST-SEED-ID
    RequestSeedId                                                          = 3455,
    /// REQUEST-TOPIC-NAME
    RequestTopicName                                                       = 124,
    /// REQUEST-TYPE
    RequestType                                                            = 3363,
    /// REQUEST-UPLOAD-CLASS-REF
    RequestUploadClassRef                                                  = 3532,
    /// REQUEST-VEHICLE-INFORMATION-CLASS-REF
    RequestVehicleInformationClassRef                                      = 3151,
    /// REQUESTABLE
    Requestable                                                            = 4902,
    /// REQUESTED-MODE-IREF
    RequestedModeIref                                                      = 2764,
    /// REQUESTER-IREF
    RequesterIref                                                          = 1791,
    /// REQUESTER-REF
    RequesterRef                                                           = 2368,
    /// REQUESTER-REFS
    RequesterRefs                                                          = 3713,
    /// REQUESTER-SOFTWARE-CLUSTER-REF
    RequesterSoftwareClusterRef                                            = 918,
    /// REQUESTOR-REF
    RequestorRef                                                           = 375,
    /// REQUESTOR-REFS
    RequestorRefs                                                          = 4608,
    /// REQUESTS
    Requests                                                               = 4590,
    /// REQUIRE-RESOURCES
    RequireResources                                                       = 3430,
    /// REQUIRED-AR-ELEMENT-REF
    RequiredArElementRef                                                   = 4586,
    /// REQUIRED-AR-ELEMENT-REFS
    RequiredArElementRefs                                                  = 2511,
    /// REQUIRED-ARTIFACTS
    RequiredArtifacts                                                      = 5111,
    /// REQUIRED-CLIENT-SERVER-ENTRY-REF
    RequiredClientServerEntryRef                                           = 1835,
    /// REQUIRED-CLIENT-SERVER-ENTRYS
    RequiredClientServerEntrys                                             = 1529,
    /// REQUIRED-COM-SPECS
    RequiredComSpecs                                                       = 5361,
    /// REQUIRED-DATAS
    RequiredDatas                                                          = 2712,
    /// REQUIRED-DDS-EVENT-QOS-PROPS
    RequiredDdsEventQosProps                                               = 5507,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 953,
    /// REQUIRED-ENTRYS
    RequiredEntrys                                                         = 3447,
    /// REQUIRED-ENVIRONMENT-REF
    RequiredEnvironmentRef                                                 = 3163,
    /// REQUIRED-EVENT-GROUPS
    RequiredEventGroups                                                    = 156,
    /// REQUIRED-FIBEX-ELEMENT-REF
    RequiredFibexElementRef                                                = 5936,
    /// REQUIRED-FIBEX-ELEMENT-REFS
    RequiredFibexElementRefs                                               = 3239,
    /// REQUIRED-GENERATOR-TOOLS
    RequiredGeneratorTools                                                 = 4267,
    /// REQUIRED-INTERFACE-TREF
    RequiredInterfaceTref                                                  = 2296,
    /// REQUIRED-MINOR-VERSION
    RequiredMinorVersion                                                   = 1847,
    /// REQUIRED-MODE-GROUP-REF
    RequiredModeGroupRef                                                   = 2746,
    /// REQUIRED-MODE-GROUPS
    RequiredModeGroups                                                     = 768,
    /// REQUIRED-OUTER-PORT-REF
    RequiredOuterPortRef                                                   = 3841,
    /// REQUIRED-PACKAGE-ELEMENT-REF
    RequiredPackageElementRef                                              = 5023,
    /// REQUIRED-PACKAGE-ELEMENT-REFS
    RequiredPackageElementRefs                                             = 5630,
    /// REQUIRED-PORT-PROTOTYPE-IREF
    RequiredPortPrototypeIref                                              = 1939,
    /// REQUIRED-RTE-VENDOR
    RequiredRteVendor                                                      = 3209,
    /// REQUIRED-SERVICE-INSTANCE-ID
    RequiredServiceInstanceId                                              = 4244,
    /// REQUIRED-SERVICE-INSTANCE-REF
    RequiredServiceInstanceRef                                             = 2643,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 2722,
    /// REQUIRED-SERVICE-PORT-IREF
    RequiredServicePortIref                                                = 4327,
    /// REQUIRED-SERVICE-VERSION
    RequiredServiceVersion                                                 = 292,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 1883,
    /// REQUIRED-TRIGGER-IREF
    RequiredTriggerIref                                                    = 2755,
    /// REQUIRED-TRIGGERS
    RequiredTriggers                                                       = 5382,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 4447,
    /// REQUIREMENT-REF
    RequirementRef                                                         = 3701,
    /// REQUIREMENT-REFS
    RequirementRefs                                                        = 5008,
    /// REQUIRES-INDEX
    RequiresIndex                                                          = 1465,
    /// REQUIRES-SYMBOLIC-NAME-VALUE
    RequiresSymbolicNameValue                                              = 771,
    /// RESET-CONFIRMED-BIT-ON-OVERFLOW
    ResetConfirmedBitOnOverflow                                            = 4197,
    /// RESET-TO-DEFAULT
    ResetToDefault                                                         = 4232,
    /// RESET-TO-DEFAULT-SUPPORTED
    ResetToDefaultSupported                                                = 3520,
    /// RESISTANT-TO-CHANGED-SW
    ResistantToChangedSw                                                   = 937,
    /// RESOLUTION
    Resolution                                                             = 5519,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 5394,
    /// RESOURCE-DEFINITION-REF
    ResourceDefinitionRef                                                  = 952,
    /// RESOURCE-DEFINITIONS
    ResourceDefinitions                                                    = 5373,
    /// RESOURCE-ESTIMATIONS
    ResourceEstimations                                                    = 931,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 4459,
    /// RESOURCE-GROUP-REF
    ResourceGroupRef                                                       = 5386,
    /// RESOURCE-GROUP-REFS
    ResourceGroupRefs                                                      = 2769,
    /// RESOURCE-GROUPS
    ResourceGroups                                                         = 5735,
    /// RESOURCE-GUARD-VALUE
    ResourceGuardValue                                                     = 3878,
    /// RESOURCE-IDENTIFIER-TYPE
    ResourceIdentifierType                                                 = 4896,
    /// RESOURCE-NEEDS-REF
    ResourceNeedsRef                                                       = 158,
    /// RESOURCE-NEEDS-REFS
    ResourceNeedsRefs                                                      = 4525,
    /// RESOURCE-REF
    ResourceRef                                                            = 5750,
    /// RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS
    ResourceToApplicationPartitionMappings                                 = 222,
    /// RESOURCES
    Resources                                                              = 5029,
    /// RESPOND-TO-RESET
    RespondToReset                                                         = 5616,
    /// RESPONSE-DATA-SIZE
    ResponseDataSize                                                       = 2976,
    /// RESPONSE-ERROR-POSITION
    ResponseErrorPosition                                                  = 3040,
    /// RESPONSE-ERROR-REF
    ResponseErrorRef                                                       = 4216,
    /// RESPONSE-GROUP-REF
    ResponseGroupRef                                                       = 786,
    /// RESPONSE-ON-ALL-REQUEST-SIDS
    ResponseOnAllRequestSids                                               = 3496,
    /// RESPONSE-ON-EVENT-ACTION
    ResponseOnEventAction                                                  = 2141,
    /// RESPONSE-ON-EVENT-CLASS-REF
    ResponseOnEventClassRef                                                = 5567,
    /// RESPONSE-ON-EVENT-REF
    ResponseOnEventRef                                                     = 520,
    /// RESPONSE-ON-EVENT-SCHEDULER-RATE
    ResponseOnEventSchedulerRate                                           = 4792,
    /// RESPONSE-ON-SECOND-DECLINED-REQUEST
    ResponseOnSecondDeclinedRequest                                        = 4386,
    /// RESPONSE-REF
    ResponseRef                                                            = 3263,
    /// RESPONSES
    Responses                                                              = 5359,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 4086,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 3716,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 2372,
    /// REST-ENDPOINT-ARGUMENT
    RestEndpointArgument                                                   = 4849,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 2504,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 3745,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 1903,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 5214,
    /// REST-ENUMERATOR-DEF
    RestEnumeratorDef                                                      = 5116,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 1603,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 2302,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 4841,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 354,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 1306,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 5048,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 2825,
    /// REST-SYSTEM-TRIGGERED-EVENT
    RestSystemTriggeredEvent                                               = 5687,
    /// RESTORE-AT-START
    RestoreAtStart                                                         = 948,
    /// RESTRICT-UPDATE
    RestrictUpdate                                                         = 541,
    /// RESTRICTION
    Restriction                                                            = 18,
    /// RESTRICTIONS
    Restrictions                                                           = 1545,
    /// RESULTING-PROPERTIES
    ResultingProperties                                                    = 1136,
    /// RESULTING-RPT-SW-PROTOTYPING-ACCESS
    ResultingRptSwPrototypingAccess                                        = 105,
    /// RESUME-POSITION
    ResumePosition                                                         = 2613,
    /// RETRY
    Retry                                                                  = 4033,
    /// RETRY-INTERVAL-TIME
    RetryIntervalTime                                                      = 2259,
    /// RETURN-CODE
    ReturnCode                                                             = 521,
    /// RETURN-SIGNAL-REF
    ReturnSignalRef                                                        = 2004,
    /// RETURN-SIGNAL-TRIGGERING-REF
    ReturnSignalTriggeringRef                                              = 3288,
    /// RETURN-TYPE
    ReturnType                                                             = 668,
    /// RETURN-VALUE-PROVISION
    ReturnValueProvision                                                   = 5073,
    /// REVERSED-TP-SDU-REF
    ReversedTpSduRef                                                       = 2167,
    /// REVISION-LABEL
    RevisionLabel                                                          = 2470,
    /// REVISION-LABEL-P-1
    RevisionLabelP1                                                        = 5781,
    /// REVISION-LABEL-P-2
    RevisionLabelP2                                                        = 5352,
    /// REVISION-LABELS
    RevisionLabels                                                         = 1934,
    /// RID-NUMBER
    RidNumber                                                              = 2120,
    /// ROLE
    Role                                                                   = 1583,
    /// ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT
    RoleBasedBswModuleEntryAssignment                                      = 547,
    /// ROLE-BASED-DATA-ASSIGNMENT
    RoleBasedDataAssignment                                                = 2487,
    /// ROLE-BASED-DATA-TYPE-ASSIGNMENT
    RoleBasedDataTypeAssignment                                            = 5459,
    /// ROLE-BASED-MC-DATA-ASSIGNMENT
    RoleBasedMcDataAssignment                                              = 2020,
    /// ROLE-BASED-PORT-ASSIGNMENT
    RoleBasedPortAssignment                                                = 2445,
    /// ROLE-BASED-RESOURCE-DEPENDENCY
    RoleBasedResourceDependency                                            = 3990,
    /// ROLLOUT-QUALIFICATIONS
    RolloutQualifications                                                  = 34,
    /// ROM-BLOCK
    RomBlock                                                               = 3671,
    /// ROM-BLOCK-INIT-VALUE
    RomBlockInitValue                                                      = 394,
    /// ROM-LOCATION-REF
    RomLocationRef                                                         = 1491,
    /// ROOT-ARGUMENT-DATA-PROTOTYPE-REF
    RootArgumentDataPrototypeRef                                           = 1940,
    /// ROOT-AUTOSAR-DATA-PROTOTYPE-REF
    RootAutosarDataPrototypeRef                                            = 4209,
    /// ROOT-COMPOSITION
    RootComposition                                                        = 792,
    /// ROOT-CONTEXT-REF
    RootContextRef                                                         = 3939,
    /// ROOT-DATA-PROTOTYPE-IN-CS-REF
    RootDataPrototypeInCsRef                                               = 4127,
    /// ROOT-DATA-PROTOTYPE-IN-SR-REF
    RootDataPrototypeInSrRef                                               = 770,
    /// ROOT-DATA-PROTOTYPE-REF
    RootDataPrototypeRef                                                   = 5773,
    /// ROOT-PARAMETER-DATA-PROTOTYPE-REF
    RootParameterDataPrototypeRef                                          = 2913,
    /// ROOT-REF
    RootRef                                                                = 907,
    /// ROOT-SOFTWARE-COMPOSITIONS
    RootSoftwareCompositions                                               = 4723,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 2587,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 4516,
    /// ROOT-VARIABLE-DATA-PROTOTYPE-REF
    RootVariableDataPrototypeRef                                           = 4231,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 4261,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 5696,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 1859,
    /// ROUTINE-CONTROL-CLASS-REF
    RoutineControlClassRef                                                 = 5556,
    /// ROUTINE-INFO
    RoutineInfo                                                            = 3099,
    /// ROUTINE-REF
    RoutineRef                                                             = 2856,
    /// ROUTINE-SUBFUNCTION-REF
    RoutineSubfunctionRef                                                  = 4029,
    /// ROUTING-ACTIVATION-TYPE
    RoutingActivationType                                                  = 2800,
    /// ROUTING-GROUP-REF
    RoutingGroupRef                                                        = 4195,
    /// ROUTING-GROUP-REFS
    RoutingGroupRefs                                                       = 2213,
    /// ROW
    Row                                                                    = 4446,
    /// RP-IMPL-POLICY
    RpImplPolicy                                                           = 2159,
    /// RPT-AR-HOOK-IREF
    RptArHookIref                                                          = 4108,
    /// RPT-COMPONENT
    RptComponent                                                           = 4984,
    /// RPT-COMPONENTS
    RptComponents                                                          = 477,
    /// RPT-CONTAINER
    RptContainer                                                           = 270,
    /// RPT-CONTAINERS
    RptContainers                                                          = 5216,
    /// RPT-ENABLER-IMPL-TYPE
    RptEnablerImplType                                                     = 5864,
    /// RPT-EVENT-ID
    RptEventId                                                             = 5145,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 3697,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 3675,
    /// RPT-EXECUTABLE-ENTITY-EVENTS
    RptExecutableEntityEvents                                              = 4002,
    /// RPT-EXECUTABLE-ENTITY-PROPERTIES
    RptExecutableEntityProperties                                          = 1278,
    /// RPT-EXECUTABLE-ENTITYS
    RptExecutableEntitys                                                   = 5344,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 4779,
    /// RPT-EXECUTION-CONTROL
    RptExecutionControl                                                    = 5392,
    /// RPT-HOOK
    RptHook                                                                = 3518,
    /// RPT-HOOK-ACCESS
    RptHookAccess                                                          = 4412,
    /// RPT-HOOKS
    RptHooks                                                               = 5925,
    /// RPT-IMPL-POLICY
    RptImplPolicy                                                          = 4781,
    /// RPT-PREPARATION-LEVEL
    RptPreparationLevel                                                    = 1695,
    /// RPT-PROFILE
    RptProfile                                                             = 1618,
    /// RPT-PROFILES
    RptProfiles                                                            = 909,
    /// RPT-READ-ACCESS
    RptReadAccess                                                          = 261,
    /// RPT-READS
    RptReads                                                               = 2972,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 4713,
    /// RPT-SERVICE-POINT-POST-REF
    RptServicePointPostRef                                                 = 4520,
    /// RPT-SERVICE-POINT-POST-REFS
    RptServicePointPostRefs                                                = 1841,
    /// RPT-SERVICE-POINT-PRE-REF
    RptServicePointPreRef                                                  = 997,
    /// RPT-SERVICE-POINT-PRE-REFS
    RptServicePointPreRefs                                                 = 111,
    /// RPT-SERVICE-POINTS
    RptServicePoints                                                       = 5220,
    /// RPT-SUPPORT-DATA
    RptSupportData                                                         = 4124,
    /// RPT-SW-PROTOTYPING-ACCESS
    RptSwPrototypingAccess                                                 = 4876,
    /// RPT-SYSTEM-REF
    RptSystemRef                                                           = 1936,
    /// RPT-WRITE-ACCESS
    RptWriteAccess                                                         = 4976,
    /// RPT-WRITES
    RptWrites                                                              = 3303,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 4905,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 3256,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 4347,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 4219,
    /// RTE-EVENT-IREF
    RteEventIref                                                           = 2903,
    /// RTE-EVENT-IREFS
    RteEventIrefs                                                          = 3893,
    /// RTE-EVENT-SEPARATIONS
    RteEventSeparations                                                    = 683,
    /// RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS
    RteEventToOsTaskProxyMappings                                          = 1623,
    /// RTE-PLUGIN-PROPS
    RtePluginProps                                                         = 1874,
    /// RTE-RESOURCE-ESTIMATION
    RteResourceEstimation                                                  = 1801,
    /// RTP-TP
    RtpTp                                                                  = 42,
    /// RTPS-PROTECTION-KIND
    RtpsProtectionKind                                                     = 1961,
    /// RULE
    Rule                                                                   = 3359,
    /// RULE-ARGUMENTS
    RuleArguments                                                          = 3257,
    /// RULE-BASED-AXIS-CONT
    RuleBasedAxisCont                                                      = 5734,
    /// RULE-BASED-VALUES
    RuleBasedValues                                                        = 1613,
    /// RULE-INIT-STATE
    RuleInitState                                                          = 2367,
    /// RULES
    Rules                                                                  = 1260,
    /// RUN-MODE
    RunMode                                                                = 2814,
    /// RUN-RESOURCE
    RunResource                                                            = 646,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 2499,
    /// RUNNABLE-ENTITY-ARGUMENT
    RunnableEntityArgument                                                 = 4696,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 5538,
    /// RUNNABLE-ENTITY-GROUP-IREF
    RunnableEntityGroupIref                                                = 1158,
    /// RUNNABLE-ENTITY-GROUP-IREFS
    RunnableEntityGroupIrefs                                               = 870,
    /// RUNNABLE-ENTITY-IREF
    RunnableEntityIref                                                     = 3479,
    /// RUNNABLE-ENTITY-IREFS
    RunnableEntityIrefs                                                    = 2435,
    /// RUNNABLE-MAPPINGS
    RunnableMappings                                                       = 5017,
    /// RUNNABLE-REF
    RunnableRef                                                            = 4324,
    /// RUNNABLES
    Runnables                                                              = 159,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REF
    RunsInsideExclusiveAreaRef                                             = 2150,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REFS
    RunsInsideExclusiveAreaRefs                                            = 2654,
    /// RUNS-INSIDES
    RunsInsides                                                            = 2812,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 1215,
    /// RUNTIME-IP-ADDRESS-CONFIGURATION
    RuntimeIpAddressConfiguration                                          = 784,
    /// RUNTIME-PORT-CONFIGURATION
    RuntimePortConfiguration                                               = 3224,
    /// RX-ACCEPT-CONTAINED-I-PDU
    RxAcceptContainedIPdu                                                  = 5610,
    /// RX-IDENTIFIER-RANGE
    RxIdentifierRange                                                      = 4266,
    /// RX-MASK
    RxMask                                                                 = 5715,
    /// RX-NM-PDU-REF
    RxNmPduRef                                                             = 4645,
    /// RX-NM-PDU-REFS
    RxNmPduRefs                                                            = 5249,
    /// RX-PDU-POOL-REF
    RxPduPoolRef                                                           = 2275,
    /// RX-PDU-TRIGGERING-REF
    RxPduTriggeringRef                                                     = 4633,
    /// RX-SECURITY-VERIFICATION
    RxSecurityVerification                                                 = 4540,
    /// RX-TIMEOUT-TIME
    RxTimeoutTime                                                          = 2341,
    /// SA-OVER-TIME
    SaOverTime                                                             = 3344,
    /// SA-RAND-TIME
    SaRandTime                                                             = 3042,
    /// SA-REKEY-TIME
    SaRekeyTime                                                            = 4948,
    /// SAFE-TRANSLATION
    SafeTranslation                                                        = 2981,
    /// SAFETY-CONDITION
    SafetyCondition                                                        = 744,
    /// SAFETY-CONDITIONS
    SafetyConditions                                                       = 882,
    /// SAFETY-MARGIN
    SafetyMargin                                                           = 2242,
    /// SAFETY-POLICY
    SafetyPolicy                                                           = 3116,
    /// SAK-REF
    SakRef                                                                 = 1206,
    /// SAK-REKEY-TIME-SPAN
    SakRekeyTimeSpan                                                       = 5869,
    /// SAMPLE-CLOCK-PERIOD
    SampleClockPeriod                                                      = 4564,
    /// SAMPLES-PER-MICROTICK
    SamplesPerMicrotick                                                    = 2932,
    /// SAVE-CONFIGURATION
    SaveConfiguration                                                      = 4149,
    /// SAVE-CONFIGURATION-ENTRY
    SaveConfigurationEntry                                                 = 5385,
    /// SCALE-CONSTR
    ScaleConstr                                                            = 5157,
    /// SCALE-CONSTRS
    ScaleConstrs                                                           = 400,
    /// SCALING-INFO-SIZE
    ScalingInfoSize                                                        = 1273,
    /// SCHEDULE-CHANGE-NEXT-TIME-BASE
    ScheduleChangeNextTimeBase                                             = 3072,
    /// SCHEDULE-TABLES
    ScheduleTables                                                         = 2417,
    /// SCHEDULER-MAX-NUMBER
    SchedulerMaxNumber                                                     = 4816,
    /// SCHEDULER-NAME-PREFIX-REF
    SchedulerNamePrefixRef                                                 = 3919,
    /// SCHEDULER-NAME-PREFIXS
    SchedulerNamePrefixs                                                   = 2247,
    /// SCHEDULING-POLICY
    SchedulingPolicy                                                       = 3221,
    /// SCHEDULING-PRIORITY
    SchedulingPriority                                                     = 5709,
    /// SCOPE
    Scope                                                                  = 3007,
    /// SCOPE-EVENT-REF
    ScopeEventRef                                                          = 4522,
    /// SCOPE-EVENT-REFS
    ScopeEventRefs                                                         = 3414,
    /// SCOPE-REF
    ScopeRef                                                               = 5509,
    /// SCOPE-REFS
    ScopeRefs                                                              = 5413,
    /// SD
    Sd                                                                     = 5225,
    /// SD-CLIENT-CONFIG
    SdClientConfig                                                         = 2609,
    /// SD-CLIENT-CONFIG-REF
    SdClientConfigRef                                                      = 3874,
    /// SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SdClientEventGroupTimingConfigRef                                      = 2901,
    /// SD-CLIENT-EVENT-TIMING-CONFIG
    SdClientEventTimingConfig                                              = 1864,
    /// SD-CLIENT-TIMER-CONFIGS
    SdClientTimerConfigs                                                   = 2845,
    /// SD-SERVER-CONFIG
    SdServerConfig                                                         = 1090,
    /// SD-SERVER-CONFIG-REF
    SdServerConfigRef                                                      = 71,
    /// SD-SERVER-EG-TIMING-CONFIGS
    SdServerEgTimingConfigs                                                = 1985,
    /// SD-SERVER-EVENT-CONFIG
    SdServerEventConfig                                                    = 606,
    /// SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SdServerEventGroupTimingConfigRef                                      = 1105,
    /// SD-SERVER-TIMER-CONFIGS
    SdServerTimerConfigs                                                   = 4663,
    /// SDF
    Sdf                                                                    = 4192,
    /// SDG
    Sdg                                                                    = 1042,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 670,
    /// SDG-CAPTION
    SdgCaption                                                             = 1171,
    /// SDG-CAPTION-REF
    SdgCaptionRef                                                          = 1034,
    /// SDG-CLASS
    SdgClass                                                               = 2234,
    /// SDG-CLASS-REF
    SdgClassRef                                                            = 1351,
    /// SDG-CLASSES
    SdgClasses                                                             = 4009,
    /// SDG-CONSTRAINT-REF
    SdgConstraintRef                                                       = 4715,
    /// SDG-CONSTRAINT-REFS
    SdgConstraintRefs                                                      = 623,
    /// SDG-DEF
    SdgDef                                                                 = 5205,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 3864,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 1851,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 4436,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 4544,
    /// SDG-REFERENCE
    SdgReference                                                           = 1099,
    /// SDG-TAILORING
    SdgTailoring                                                           = 5357,
    /// SDG-TAILORINGS
    SdgTailorings                                                          = 4907,
    /// SDGS
    Sdgs                                                                   = 110,
    /// SDU-REF
    SduRef                                                                 = 4268,
    /// SDU-REFS
    SduRefs                                                                = 5667,
    /// SDU-TYPE
    SduType                                                                = 4856,
    /// SDX-REF
    SdxRef                                                                 = 5746,
    /// SDXF
    Sdxf                                                                   = 1148,
    /// SEARCH-BEHAVIOR
    SearchBehavior                                                         = 5878,
    /// SEARCH-INTENTION
    SearchIntention                                                        = 4489,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REF
    SecOcComPropsForMulticastRef                                           = 5172,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REFS
    SecOcComPropsForMulticastRefs                                          = 5457,
    /// SEC-OC-CRYPTO-MAPPING-REF
    SecOcCryptoMappingRef                                                  = 4194,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 2950,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 905,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 947,
    /// SEC-OC-JOB-MAPPINGS
    SecOcJobMappings                                                       = 5889,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 3617,
    /// SEC-OC-JOB-REQUIREMENT-REF
    SecOcJobRequirementRef                                                 = 70,
    /// SEC-OC-JOB-SEMANTIC
    SecOcJobSemantic                                                       = 4833,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 545,
    /// SECOND-APPLICATION-ERROR-REF
    SecondApplicationErrorRef                                              = 1636,
    /// SECOND-DATA-PROTOTYPE-REF
    SecondDataPrototypeRef                                                 = 3167,
    /// SECOND-ELEMENT-INSTANCE-IREF
    SecondElementInstanceIref                                              = 5852,
    /// SECOND-ELEMENT-INSTANCE-IREFS
    SecondElementInstanceIrefs                                             = 5185,
    /// SECOND-ELEMENT-REF
    SecondElementRef                                                       = 2171,
    /// SECOND-ELEMENT-REFS
    SecondElementRefs                                                      = 929,
    /// SECOND-ELEMENTS
    SecondElements                                                         = 1505,
    /// SECOND-KEY-SLOT-ID
    SecondKeySlotId                                                        = 3146,
    /// SECOND-MODE-GROUP-REF
    SecondModeGroupRef                                                     = 1830,
    /// SECOND-MODE-REF
    SecondModeRef                                                          = 3210,
    /// SECOND-OPERATION-REF
    SecondOperationRef                                                     = 300,
    /// SECOND-PHYSICAL-DIMENSION-REF
    SecondPhysicalDimensionRef                                             = 2561,
    /// SECOND-PORT-REF
    SecondPortRef                                                          = 2412,
    /// SECOND-TO-FIRST-DATA-TRANSFORMATION-REF
    SecondToFirstDataTransformationRef                                     = 1319,
    /// SECOND-TRIGGER-REF
    SecondTriggerRef                                                       = 4251,
    /// SECOND-VALUE
    SecondValue                                                            = 1274,
    /// SECONDARY-FRESHNESS-VALUE-ID
    SecondaryFreshnessValueId                                              = 1131,
    /// SECTION-INITIALIZATION-POLICY
    SectionInitializationPolicy                                            = 3682,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 1532,
    /// SECTION-NAME-PREFIXS
    SectionNamePrefixs                                                     = 4998,
    /// SECTION-TYPE
    SectionType                                                            = 3111,
    /// SECURE-COM-CONFIG-REF
    SecureComConfigRef                                                     = 3525,
    /// SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING
    SecureComConfigToCryptoKeySlotMapping                                  = 3297,
    /// SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS
    SecureComConfigToKeySlotMappings                                       = 4104,
    /// SECURE-COM-CONFIGS
    SecureComConfigs                                                       = 132,
    /// SECURE-COM-PROPS-FOR-DDS-REF
    SecureComPropsForDdsRef                                                = 2779,
    /// SECURE-COM-PROPS-FOR-TCP-REF
    SecureComPropsForTcpRef                                                = 1165,
    /// SECURE-COM-PROPS-FOR-TCP-REFS
    SecureComPropsForTcpRefs                                               = 3464,
    /// SECURE-COM-PROPS-FOR-UDP-REF
    SecureComPropsForUdpRef                                                = 504,
    /// SECURE-COM-PROPS-FOR-UDP-REFS
    SecureComPropsForUdpRefs                                               = 5844,
    /// SECURE-COM-PROPS-REF
    SecureComPropsRef                                                      = 2214,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 1059,
    /// SECURE-COM-PROPSS
    SecureComPropss                                                        = 949,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 3391,
    /// SECURE-COMMUNICATION-DEPLOYMENTS
    SecureCommunicationDeployments                                         = 1867,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 1230,
    /// SECURE-COMMUNICATION-PROPS
    SecureCommunicationProps                                               = 2040,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 1881,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 3426,
    /// SECURE-PROPS-FOR-EXTERNAL-COMM-REF
    SecurePropsForExternalCommRef                                          = 3653,
    /// SECURE-PROPS-FOR-TCP-REF
    SecurePropsForTcpRef                                                   = 4773,
    /// SECURE-PROPS-FOR-UDP-REF
    SecurePropsForUdpRef                                                   = 4152,
    /// SECURE-TRANSLATION
    SecureTranslation                                                      = 3109,
    /// SECURED-AREA-LENGTH
    SecuredAreaLength                                                      = 4611,
    /// SECURED-AREA-OFFSET
    SecuredAreaOffset                                                      = 297,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 4501,
    /// SECURED-RX-VERIFICATION
    SecuredRxVerification                                                  = 539,
    /// SECURITY-ACCESS-CLASS-REF
    SecurityAccessClassRef                                                 = 5261,
    /// SECURITY-ACCESS-LEVEL
    SecurityAccessLevel                                                    = 1356,
    /// SECURITY-DELAY-TIME
    SecurityDelayTime                                                      = 413,
    /// SECURITY-DELAY-TIME-ON-BOOT
    SecurityDelayTimeOnBoot                                                = 806,
    /// SECURITY-EVENT-CONTEXT-DATA
    SecurityEventContextData                                               = 3389,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 1834,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 1654,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 462,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 5726,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 5596,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 2574,
    /// SECURITY-EVENT-DEFINITION-REF
    SecurityEventDefinitionRef                                             = 4326,
    /// SECURITY-EVENT-DEFINITION-REF-CONDITIONAL
    SecurityEventDefinitionRefConditional                                  = 4246,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 1959,
    /// SECURITY-EVENT-FILTER-CHAIN-REF
    SecurityEventFilterChainRef                                            = 833,
    /// SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL
    SecurityEventFilterChainRefConditional                                 = 465,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 1013,
    /// SECURITY-EVENT-PROPS-REF
    SecurityEventPropsRef                                                  = 1571,
    /// SECURITY-EVENT-REF
    SecurityEventRef                                                       = 3137,
    /// SECURITY-EVENT-REFS
    SecurityEventRefs                                                      = 5929,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 1999,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 101,
    /// SECURITY-EVENTS
    SecurityEvents                                                         = 4903,
    /// SECURITY-LEVEL-REF
    SecurityLevelRef                                                       = 5733,
    /// SECURITY-LEVEL-REFS
    SecurityLevelRefs                                                      = 4993,
    /// SEED-SIZE
    SeedSize                                                               = 845,
    /// SEGMENT-BYTE-ORDER
    SegmentByteOrder                                                       = 5624,
    /// SEGMENT-LENGTH
    SegmentLength                                                          = 202,
    /// SEGMENT-POSITION
    SegmentPosition                                                        = 1717,
    /// SEGMENT-POSITIONS
    SegmentPositions                                                       = 405,
    /// SEGMENT-REF
    SegmentRef                                                             = 379,
    /// SEGMENT-REFS
    SegmentRefs                                                            = 5908,
    /// SEGMENTATION-SUPPORTED
    SegmentationSupported                                                  = 5657,
    /// SELECT-BLOCK-FOR-FIRST-INIT-ALL
    SelectBlockForFirstInitAll                                             = 4385,
    /// SELECTIONS
    Selections                                                             = 5403,
    /// SELECTOR-FIELD-BYTE-ORDER
    SelectorFieldByteOrder                                                 = 4001,
    /// SELECTOR-FIELD-CODE
    SelectorFieldCode                                                      = 527,
    /// SELECTOR-FIELD-LENGTH
    SelectorFieldLength                                                    = 3742,
    /// SELECTOR-FIELD-START-POSITION
    SelectorFieldStartPosition                                             = 5521,
    /// SEND-ACTIVITY
    SendActivity                                                           = 4411,
    /// SEND-INDICATION
    SendIndication                                                         = 3985,
    /// SEND-POLICYS
    SendPolicys                                                            = 4161,
    /// SEND-RESP-PEND-ON-TRANS-TO-BOOT
    SendRespPendOnTransToBoot                                              = 5940,
    /// SENDER-ANNOTATION
    SenderAnnotation                                                       = 5560,
    /// SENDER-CAPABILITY
    SenderCapability                                                       = 1665,
    /// SENDER-INTENT
    SenderIntent                                                           = 790,
    /// SENDER-IREF
    SenderIref                                                             = 3626,
    /// SENDER-REC-ARRAY-ELEMENT-MAPPING
    SenderRecArrayElementMapping                                           = 5211,
    /// SENDER-REC-ARRAY-TYPE-MAPPING
    SenderRecArrayTypeMapping                                              = 899,
    /// SENDER-REC-RECORD-ELEMENT-MAPPING
    SenderRecRecordElementMapping                                          = 5491,
    /// SENDER-REC-RECORD-TYPE-MAPPING
    SenderRecRecordTypeMapping                                             = 3113,
    /// SENDER-RECEIVER-ANNOTATIONS
    SenderReceiverAnnotations                                              = 1028,
    /// SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING
    SenderReceiverCompositeElementToSignalMapping                          = 1799,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 4835,
    /// SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING
    SenderReceiverToSignalGroupMapping                                     = 5920,
    /// SENDER-RECEIVER-TO-SIGNAL-MAPPING
    SenderReceiverToSignalMapping                                          = 1079,
    /// SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING
    SenderToSignalTextTableMapping                                         = 5693,
    /// SENDING-NODE-REF
    SendingNodeRef                                                         = 3947,
    /// SENDING-NODE-REFS
    SendingNodeRefs                                                        = 2556,
    /// SENSOR-ACTUATOR-REF
    SensorActuatorRef                                                      = 3567,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 3396,
    /// SENSOR-INSTANCE-ID
    SensorInstanceId                                                       = 1594,
    /// SEPARATE-SIGNAL-PATH
    SeparateSignalPath                                                     = 2136,
    /// SEPARATED-COMPONENT-IREF
    SeparatedComponentIref                                                 = 5524,
    /// SEPARATED-COMPONENT-IREFS
    SeparatedComponentIrefs                                                = 5603,
    /// SEPARATION-CYCLE-EXPONENT
    SeparationCycleExponent                                                = 2484,
    /// SEPARATION-TIME
    SeparationTime                                                         = 3958,
    /// SEPARATION-TIME-REQUEST
    SeparationTimeRequest                                                  = 349,
    /// SEPARATION-TIME-RESPONSE
    SeparationTimeResponse                                                 = 980,
    /// SEQUENCE-COUNTER
    SequenceCounter                                                        = 5843,
    /// SEQUENCE-COUNTER-JUMP-WIDTH
    SequenceCounterJumpWidth                                               = 653,
    /// SEQUENCE-OFFSET
    SequenceOffset                                                         = 3289,
    /// SEQUENTIAL-INSTRUCTIONS-BEGIN
    SequentialInstructionsBegin                                            = 3896,
    /// SEQUENTIAL-INSTRUCTIONS-END
    SequentialInstructionsEnd                                              = 75,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 386,
    /// SERIALIZATION-TECHNOLOGY-REF
    SerializationTechnologyRef                                             = 3724,
    /// SERIALIZER
    Serializer                                                             = 995,
    /// SERIALIZER-REF
    SerializerRef                                                          = 1111,
    /// SERVER-ARGUMENT-IMPL-POLICY
    ServerArgumentImplPolicy                                               = 1541,
    /// SERVER-CALL-POINTS
    ServerCallPoints                                                       = 2197,
    /// SERVER-COM-SPEC
    ServerComSpec                                                          = 314,
    /// SERVER-NAME-IDENTIFICATION
    ServerNameIdentification                                               = 1157,
    /// SERVER-PORT-REF
    ServerPortRef                                                          = 847,
    /// SERVER-SERVICE-MAJOR-VERSION
    ServerServiceMajorVersion                                              = 5885,
    /// SERVER-SERVICE-MINOR-VERSION
    ServerServiceMinorVersion                                              = 1452,
    /// SERVICE-BUSY
    ServiceBusy                                                            = 5770,
    /// SERVICE-CONTROL
    ServiceControl                                                         = 5597,
    /// SERVICE-DEPENDENCYS
    ServiceDependencys                                                     = 1365,
    /// SERVICE-DEPLOYMENT-REF
    ServiceDeploymentRef                                                   = 2915,
    /// SERVICE-DISCOVER-CONFIGS
    ServiceDiscoverConfigs                                                 = 4974,
    /// SERVICE-DISCOVERY-CONFIGS
    ServiceDiscoveryConfigs                                                = 2889,
    /// SERVICE-ELEMENT-MAPPING-REF
    ServiceElementMappingRef                                               = 5433,
    /// SERVICE-ELEMENT-MAPPING-REFS
    ServiceElementMappingRefs                                              = 171,
    /// SERVICE-ELEMENT-SECURE-COM-CONFIG-REF
    ServiceElementSecureComConfigRef                                       = 2664,
    /// SERVICE-FIND-TIME-TO-LIVE
    ServiceFindTimeToLive                                                  = 3014,
    /// SERVICE-ID
    ServiceId                                                              = 2182,
    /// SERVICE-IDENTIFIER
    ServiceIdentifier                                                      = 3465,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 4513,
    /// SERVICE-INSTANCE-ID
    ServiceInstanceId                                                      = 5247,
    /// SERVICE-INSTANCE-PORT-CONFIG
    ServiceInstancePortConfig                                              = 5854,
    /// SERVICE-INSTANCE-REF
    ServiceInstanceRef                                                     = 5007,
    /// SERVICE-INSTANCE-REFS
    ServiceInstanceRefs                                                    = 4860,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 5390,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 5813,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF
    ServiceInstanceToPortPrototypeMappingRef                               = 2339,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 3852,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 4671,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS
    ServiceInstanceToSignalMappings                                        = 4650,
    /// SERVICE-INSTANCES
    ServiceInstances                                                       = 344,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 3978,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 928,
    /// SERVICE-INTERFACE-DEPLOYMENT-REF
    ServiceInterfaceDeploymentRef                                          = 2703,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REF
    ServiceInterfaceElementMappingRef                                      = 825,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REFS
    ServiceInterfaceElementMappingRefs                                     = 1301,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 2817,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 5607,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 3400,
    /// SERVICE-INTERFACE-ID
    ServiceInterfaceId                                                     = 2619,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 2061,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 4776,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 5706,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 4090,
    /// SERVICE-INTERFACE-REF
    ServiceInterfaceRef                                                    = 1596,
    /// SERVICE-INTERFACE-REFS
    ServiceInterfaceRefs                                                   = 5930,
    /// SERVICE-INTERFACE-SUB-ELEMENT
    ServiceInterfaceSubElement                                             = 5311,
    /// SERVICE-INTERFACE-TREF
    ServiceInterfaceTref                                                   = 1592,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 1755,
    /// SERVICE-INTERFACE-VERSION
    ServiceInterfaceVersion                                                = 1167,
    /// SERVICE-ITEMS
    ServiceItems                                                           = 3117,
    /// SERVICE-KIND
    ServiceKind                                                            = 3843,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 4349,
    /// SERVICE-OFFER-TIME-TO-LIVE
    ServiceOfferTimeToLive                                                 = 4791,
    /// SERVICE-POINT-SYMBOL-POST
    ServicePointSymbolPost                                                 = 3614,
    /// SERVICE-POINT-SYMBOL-PRE
    ServicePointSymbolPre                                                  = 4494,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 4342,
    /// SERVICE-REQUEST-CALLBACK-TYPE
    ServiceRequestCallbackType                                             = 1379,
    /// SERVICE-RESOURCE-REF
    ServiceResourceRef                                                     = 2588,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 2916,
    /// SERVICE-TABLES
    ServiceTables                                                          = 4524,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 85,
    /// SESSION-CONTROL-CLASS-REF
    SessionControlClassRef                                                 = 1606,
    /// SESSION-HANDLING
    SessionHandling                                                        = 5362,
    /// SESSION-HANDLING-SR
    SessionHandlingSr                                                      = 5448,
    /// SESSION-ID
    SessionId                                                              = 1282,
    /// SESSION-ID-SUPPORT
    SessionIdSupport                                                       = 3055,
    /// SET
    Set                                                                    = 330,
    /// SET-FUNCTION-GROUP-STATE-IREF
    SetFunctionGroupStateIref                                              = 4526,
    /// SETTER-CALL-REF
    SetterCallRef                                                          = 842,
    /// SETTER-CALL-SIGNAL-REF
    SetterCallSignalRef                                                    = 3380,
    /// SETTER-OPERATION-REF
    SetterOperationRef                                                     = 2252,
    /// SETTER-REF
    SetterRef                                                              = 2041,
    /// SETTER-RETURN-REF
    SetterReturnRef                                                        = 4576,
    /// SETTER-RETURN-SIGNAL-REF
    SetterReturnSignalRef                                                  = 4181,
    /// SEVERITY
    Severity                                                               = 1648,
    /// SHALL-NOT-RUN-ON-REF
    ShallNotRunOnRef                                                       = 3184,
    /// SHALL-NOT-RUN-ON-REFS
    ShallNotRunOnRefs                                                      = 3348,
    /// SHALL-RUN-ON-REF
    ShallRunOnRef                                                          = 3294,
    /// SHALL-RUN-ON-REFS
    ShallRunOnRefs                                                         = 1252,
    /// SHARED-AXIS-TYPE-REF
    SharedAxisTypeRef                                                      = 3685,
    /// SHARED-PARAMETERS
    SharedParameters                                                       = 3790,
    /// SHARED-TIMER
    SharedTimer                                                            = 2562,
    /// SHORT-LABEL
    ShortLabel                                                             = 4098,
    /// SHORT-LABEL-PATTERN
    ShortLabelPattern                                                      = 3334,
    /// SHORT-NAME
    ShortName                                                              = 1908,
    /// SHORT-NAME-FRAGMENT
    ShortNameFragment                                                      = 3052,
    /// SHORT-NAME-FRAGMENTS
    ShortNameFragments                                                     = 4409,
    /// SHORT-NAME-PATTERN
    ShortNamePattern                                                       = 750,
    /// SHORT-TERM-ADJUSTMENT
    ShortTermAdjustment                                                    = 5810,
    /// SHORT-TERM-ADJUSTMENT-SUPPORTED
    ShortTermAdjustmentSupported                                           = 1264,
    /// SHUTDOWN
    Shutdown                                                               = 4900,
    /// SIGNAL-AGE
    SignalAge                                                              = 1094,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 468,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 3555,
    /// SIGNAL-BASED-EVENT-REF
    SignalBasedEventRef                                                    = 2760,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 3085,
    /// SIGNAL-BASED-FIELD-REF
    SignalBasedFieldRef                                                    = 3453,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 3687,
    /// SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFireAndForgetMethodToISignalTriggeringMapping               = 1919,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 1837,
    /// SIGNAL-BASED-METHOD-REF
    SignalBasedMethodRef                                                   = 4066,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 1980,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 211,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 2311,
    /// SIGNAL-FAN
    SignalFan                                                              = 4800,
    /// SIGNAL-GROUP-REF
    SignalGroupRef                                                         = 5041,
    /// SIGNAL-I-PDU-COUNTER
    SignalIPduCounter                                                      = 1326,
    /// SIGNAL-I-PDU-REPLICATION
    SignalIPduReplication                                                  = 2379,
    /// SIGNAL-MAPPINGS
    SignalMappings                                                         = 5091,
    /// SIGNAL-PATH-CONSTRAINTS
    SignalPathConstraints                                                  = 2782,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 1174,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 5898,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS
    SignalServiceTranslationEventPropss                                    = 4910,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 4970,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 3877,
    /// SIGNAL-SERVICE-TRANSLATION-PROPSS
    SignalServiceTranslationPropss                                         = 809,
    /// SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING
    SignalToReceiverTextTableMapping                                       = 3835,
    /// SIGNALS
    Signals                                                                = 3053,
    /// SIGNATURE-SCHEME-ID
    SignatureSchemeId                                                      = 1067,
    /// SIGNATURE-SCHEME-REF
    SignatureSchemeRef                                                     = 2321,
    /// SIGNATURE-SCHEME-REFS
    SignatureSchemeRefs                                                    = 229,
    /// SIGNATURE-SUPPORT-AP
    SignatureSupportAp                                                     = 68,
    /// SIGNATURE-SUPPORT-CP
    SignatureSupportCp                                                     = 4229,
    /// SIGNIFICANCE
    Significance                                                           = 2531,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 4759,
    /// SIZE
    Size                                                                   = 5668,
    /// SIZE-OF-ARRAY-LENGTH-FIELD
    SizeOfArrayLengthField                                                 = 1696,
    /// SIZE-OF-ARRAY-LENGTH-FIELDS
    SizeOfArrayLengthFields                                                = 1748,
    /// SIZE-OF-STRING-LENGTH-FIELD
    SizeOfStringLengthField                                                = 1106,
    /// SIZE-OF-STRING-LENGTH-FIELDS
    SizeOfStringLengthFields                                               = 5634,
    /// SIZE-OF-STRUCT-LENGTH-FIELD
    SizeOfStructLengthField                                                = 654,
    /// SIZE-OF-STRUCT-LENGTH-FIELDS
    SizeOfStructLengthFields                                               = 4929,
    /// SIZE-OF-UNION-LENGTH-FIELD
    SizeOfUnionLengthField                                                 = 3477,
    /// SIZE-OF-UNION-LENGTH-FIELDS
    SizeOfUnionLengthFields                                                = 1546,
    /// SIZE-OF-UNION-TYPE-SELECTOR-FIELD
    SizeOfUnionTypeSelectorField                                           = 5027,
    /// SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE
    SlaveActAsPassiveCommunicationSlave                                    = 212,
    /// SLAVE-EVENT-REF
    SlaveEventRef                                                          = 4772,
    /// SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME
    SlaveQualifiedUnexpectedLinkDownTime                                   = 2644,
    /// SLAVE-REF
    SlaveRef                                                               = 2269,
    /// SLAVES
    Slaves                                                                 = 885,
    /// SLEEP-MODE-EXECUTION-DELAY
    SleepModeExecutionDelay                                                = 5835,
    /// SLEEP-MODE-SUPPORTED
    SleepModeSupported                                                     = 1248,
    /// SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST
    SleepRepetitionDelayOfSleepRequest                                     = 1461,
    /// SLEEP-REPETITIONS-OF-SLEEP-REQUEST
    SleepRepetitionsOfSleepRequest                                         = 166,
    /// SLOT-CAPACITY
    SlotCapacity                                                           = 3242,
    /// SLOT-ID
    SlotId                                                                 = 3131,
    /// SLOT-TYPE
    SlotType                                                               = 5410,
    /// SNAPSHOT-RECORD-CONTENTS
    SnapshotRecordContents                                                 = 1026,
    /// SO-AD-CONFIG
    SoAdConfig                                                             = 663,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 2666,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 1729,
    /// SO-CON-I-PDU-IDENTIFIER-REF
    SoConIPduIdentifierRef                                                 = 2829,
    /// SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL
    SoConIPduIdentifierRefConditional                                      = 2021,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 3664,
    /// SOCKET-ADDRESS-REF
    SocketAddressRef                                                       = 2322,
    /// SOCKET-ADDRESS-REF-CONDITIONAL
    SocketAddressRefConditional                                            = 4899,
    /// SOCKET-ADDRESSS
    SocketAddresss                                                         = 1503,
    /// SOCKET-CONNECTION
    SocketConnection                                                       = 2727,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 3699,
    /// SOCKET-CONNECTION-BUNDLE-REF
    SocketConnectionBundleRef                                              = 2974,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER
    SocketConnectionIpduIdentifier                                         = 1580,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 454,
    /// SOCKET-CONNECTION-REF
    SocketConnectionRef                                                    = 5314,
    /// SOCKET-CONNECTION-REFS
    SocketConnectionRefs                                                   = 62,
    /// SOCKET-OPTION
    SocketOption                                                           = 516,
    /// SOCKET-OPTIONS
    SocketOptions                                                          = 2696,
    /// SOCKET-PROTOCOL
    SocketProtocol                                                         = 4604,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION
    SoftwareActivationDependencyCompareCondition                           = 4867,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA
    SoftwareActivationDependencyFormula                                    = 2076,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-REF
    SoftwareActivationDependencyRef                                        = 5052,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 4133,
    /// SOFTWARE-CLUSTER-DEPENDENCY
    SoftwareClusterDependency                                              = 3723,
    /// SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION
    SoftwareClusterDependencyCompareCondition                              = 5057,
    /// SOFTWARE-CLUSTER-DEPENDENCY-FORMULA
    SoftwareClusterDependencyFormula                                       = 5421,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 912,
    /// SOFTWARE-CLUSTER-DESIGN-DEPENDENCY
    SoftwareClusterDesignDependency                                        = 5861,
    /// SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS
    SoftwareClusterDiagnosticDeploymentProps                               = 3475,
    /// SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS
    SoftwareClusterDoipDiagnosticAddress                                   = 2014,
    /// SOFTWARE-CLUSTER-ID
    SoftwareClusterId                                                      = 2933,
    /// SOFTWARE-CLUSTER-REF
    SoftwareClusterRef                                                     = 3265,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 2895,
    /// SOFTWARE-CLUSTER-SOVD-ADDRESS
    SoftwareClusterSovdAddress                                             = 3437,
    /// SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS
    SoftwareClusterToResourceMappings                                      = 5653,
    /// SOFTWARE-COMPOSITION-TREF
    SoftwareCompositionTref                                                = 601,
    /// SOFTWARE-CONTEXT
    SoftwareContext                                                        = 3273,
    /// SOFTWARE-MEMORY-SECTION-REF
    SoftwareMemorySectionRef                                               = 669,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 1338,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 1250,
    /// SOFTWARE-PACKAGE-STEPS
    SoftwarePackageSteps                                                   = 3708,
    /// SOFTWARE-PACKAGE-STORING
    SoftwarePackageStoring                                                 = 2218,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 3493,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 3908,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 3150,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 2063,
    /// SOMEIP-EVENT-PROPS
    SomeipEventProps                                                       = 5927,
    /// SOMEIP-FIELD
    SomeipField                                                            = 1914,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 2245,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 5127,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 0,
    /// SOMEIP-METHOD-PROPS
    SomeipMethodProps                                                      = 2565,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 569,
    /// SOMEIP-REMOTE-MULTICAST-CONFIG
    SomeipRemoteMulticastConfig                                            = 4712,
    /// SOMEIP-REMOTE-UNICAST-CONFIG
    SomeipRemoteUnicastConfig                                              = 343,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 1542,
    /// SOMEIP-RULE
    SomeipRule                                                             = 4862,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 1763,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdClientEventGroupTimingConfigRef                                = 2614,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdClientEventGroupTimingConfigRefConditional                     = 3517,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 2956,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdClientServiceInstanceConfigRef                                 = 1913,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdClientServiceInstanceConfigRefConditional                      = 3333,
    /// SOMEIP-SD-RULE
    SomeipSdRule                                                           = 3275,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 4809,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdServerEventGroupTimingConfigRef                                = 475,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdServerEventGroupTimingConfigRefConditional                     = 4497,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 5353,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdServerServiceInstanceConfigRef                                 = 1380,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdServerServiceInstanceConfigRefConditional                      = 810,
    /// SOMEIP-SERVICE-DISCOVERY
    SomeipServiceDiscovery                                                 = 2925,
    /// SOMEIP-SERVICE-DISCOVERY-PORT
    SomeipServiceDiscoveryPort                                             = 52,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 4648,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 1009,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 1962,
    /// SOMEIP-SERVICE-VERSION
    SomeipServiceVersion                                                   = 4851,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 1813,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 1774,
    /// SOMEIP-TP-CONNECTION
    SomeipTpConnection                                                     = 805,
    /// SOMEIP-TRANSFORMATION-DESCRIPTION
    SomeipTransformationDescription                                        = 2538,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS
    SomeipTransformationISignalProps                                       = 5193,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    SomeipTransformationISignalPropsConditional                            = 5717,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    SomeipTransformationISignalPropsVariants                               = 2374,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 2686,
    /// SOMEIP-TRANSFORMATION-PROPS-REF
    SomeipTransformationPropsRef                                           = 1429,
    /// SOUCE-CAN-ID-REF
    SouceCanIdRef                                                          = 776,
    /// SOURCE-ADDRESS-CODE
    SourceAddressCode                                                      = 4304,
    /// SOURCE-ADDRESS-MASK
    SourceAddressMask                                                      = 5756,
    /// SOURCE-CAN-ID-CODE
    SourceCanIdCode                                                        = 3081,
    /// SOURCE-CAN-ID-MASK
    SourceCanIdMask                                                        = 3659,
    /// SOURCE-CHANNEL
    SourceChannel                                                          = 946,
    /// SOURCE-EEC-REF
    SourceEecRef                                                           = 3019,
    /// SOURCE-EEC-REFS
    SourceEecRefs                                                          = 1416,
    /// SOURCE-ELEMENT-REF
    SourceElementRef                                                       = 2253,
    /// SOURCE-ELEMENT-REFS
    SourceElementRefs                                                      = 1119,
    /// SOURCE-EVENT-REF
    SourceEventRef                                                         = 464,
    /// SOURCE-EVENT-REFS
    SourceEventRefs                                                        = 4510,
    /// SOURCE-FIELD-REF
    SourceFieldRef                                                         = 1394,
    /// SOURCE-FRAME-REF
    SourceFrameRef                                                         = 5743,
    /// SOURCE-I-PDU-REF
    SourceIPduRef                                                          = 1390,
    /// SOURCE-ID
    SourceId                                                               = 160,
    /// SOURCE-INSTANCE-IREF
    SourceInstanceIref                                                     = 5894,
    /// SOURCE-INSTANCE-IREFS
    SourceInstanceIrefs                                                    = 1170,
    /// SOURCE-IP-ADDRESS
    SourceIpAddress                                                        = 5204,
    /// SOURCE-LIN-PID-REF
    SourceLinPidRef                                                        = 5166,
    /// SOURCE-MAC-ADDRESS
    SourceMacAddress                                                       = 881,
    /// SOURCE-MAC-ADDRESS-MASK
    SourceMacAddressMask                                                   = 2264,
    /// SOURCE-MAX-ADDRESS
    SourceMaxAddress                                                       = 1630,
    /// SOURCE-METHOD-REF
    SourceMethodRef                                                        = 3161,
    /// SOURCE-MIN-ADDRESS
    SourceMinAddress                                                       = 5862,
    /// SOURCE-NETWORK-MASK
    SourceNetworkMask                                                      = 1769,
    /// SOURCE-POSSIBLE-ERROR-REF
    SourcePossibleErrorRef                                                 = 1223,
    /// SOURCE-REF
    SourceRef                                                              = 1435,
    /// SOURCE-SERVICE-INTERFACE-REF
    SourceServiceInterfaceRef                                              = 4234,
    /// SOURCE-SERVICE-INTERFACE-REFS
    SourceServiceInterfaceRefs                                             = 2204,
    /// SOURCE-SIGNAL-REF
    SourceSignalRef                                                        = 2627,
    /// SOURCE-TRIGGER-REF
    SourceTriggerRef                                                       = 3434,
    /// SOVD-GATEWAY-INSTANTIATION
    SovdGatewayInstantiation                                               = 267,
    /// SOVD-LOCK-REF
    SovdLockRef                                                            = 1039,
    /// SOVD-SERVER-INSTANTIATION
    SovdServerInstantiation                                                = 693,
    /// SPECIFIC-CHANNEL-REF
    SpecificChannelRef                                                     = 2535,
    /// SPECIFIC-CHANNELS
    SpecificChannels                                                       = 638,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 1941,
    /// SPECIFICATION-DOCUMENT-SCOPES
    SpecificationDocumentScopes                                            = 4714,
    /// SPECIFICATION-SCOPE
    SpecificationScope                                                     = 4359,
    /// SPEED
    Speed                                                                  = 28,
    /// SPEEDUP
    Speedup                                                                = 3637,
    /// SPN
    Spn                                                                    = 209,
    /// SPN-REF
    SpnRef                                                                 = 119,
    /// SPN-REFS
    SpnRefs                                                                = 4707,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 5098,
    /// SSP-OFFSET
    SspOffset                                                              = 608,
    /// SSRC
    Ssrc                                                                   = 1966,
    /// ST-MIN
    StMin                                                                  = 3245,
    /// STACK-USAGES
    StackUsages                                                            = 2646,
    /// STANDARD
    Standard                                                               = 4592,
    /// STANDARD-REVISION
    StandardRevision                                                       = 445,
    /// STANDARD-REVISIONS
    StandardRevisions                                                      = 5003,
    /// START
    Start                                                                  = 2450,
    /// START-ACTION-REF
    StartActionRef                                                         = 5489,
    /// START-ACTION-REFS
    StartActionRefs                                                        = 2194,
    /// START-ADDRESS
    StartAddress                                                           = 5293,
    /// START-INDEX
    StartIndex                                                             = 148,
    /// START-IREF
    StartIref                                                              = 417,
    /// START-ON-EVENT-REF
    StartOnEventRef                                                        = 4717,
    /// START-POSITION
    StartPosition                                                          = 1096,
    /// STARTS-ON-EVENT-REF
    StartsOnEventRef                                                       = 2874,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 4697,
    /// STARTUP-CONFIG-REF
    StartupConfigRef                                                       = 2473,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 5512,
    /// STARTUP-CONFIGS
    StartupConfigs                                                         = 950,
    /// STARTUP-OPTION
    StartupOption                                                          = 90,
    /// STARTUP-OPTIONS
    StartupOptions                                                         = 4716,
    /// STATE
    State                                                                  = 3403,
    /// STATE-DEPENDENT-FIREWALL
    StateDependentFirewall                                                 = 2221,
    /// STATE-DEPENDENT-FIREWALL-REF
    StateDependentFirewallRef                                              = 3106,
    /// STATE-DEPENDENT-STARTUP-CONFIG
    StateDependentStartupConfig                                            = 4855,
    /// STATE-DEPENDENT-STARTUP-CONFIGS
    StateDependentStartupConfigs                                           = 3440,
    /// STATE-MACHINE
    StateMachine                                                           = 43,
    /// STATE-MANAGEMEN-PHM-ERROR-INTERFACE
    StateManagemenPhmErrorInterface                                        = 5946,
    /// STATE-MANAGEMENT-ACTION-LIST
    StateManagementActionList                                              = 439,
    /// STATE-MANAGEMENT-BASED-ON-TCP-FLAGS
    StateManagementBasedOnTcpFlags                                         = 1003,
    /// STATE-MANAGEMENT-COMPARE-FORMULA
    StateManagementCompareFormula                                          = 1362,
    /// STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE
    StateManagementDiagTriggerInterface                                    = 2363,
    /// STATE-MANAGEMENT-EM-ERROR-INTERFACE
    StateManagementEmErrorInterface                                        = 1414,
    /// STATE-MANAGEMENT-ERROR-COMPARE-RULE
    StateManagementErrorCompareRule                                        = 1074,
    /// STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE
    StateManagementFunctionGroupSwitchNotificationInterface                = 317,
    /// STATE-MANAGEMENT-MODULE-INSTANTIATION
    StateManagementModuleInstantiation                                     = 4168,
    /// STATE-MANAGEMENT-REQUEST-ERROR
    StateManagementRequestError                                            = 1669,
    /// STATE-MANAGEMENT-REQUEST-RULE
    StateManagementRequestRule                                             = 4288,
    /// STATE-MANAGEMENT-REQUEST-TRIGGER
    StateManagementRequestTrigger                                          = 5136,
    /// STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM
    StateManagementSetFunctionGroupStateActionItem                         = 5811,
    /// STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM
    StateManagementStateMachineActionItem                                  = 565,
    /// STATE-MANAGEMENT-STATE-NOTIFICATION
    StateManagementStateNotification                                       = 191,
    /// STATE-MANAGEMENT-SYNC-ACTION-ITEM
    StateManagementSyncActionItem                                          = 367,
    /// STATE-MANAGEMENT-TRIGGER-COMPARE-RULE
    StateManagementTriggerCompareRule                                      = 5651,
    /// STATE-REF
    StateRef                                                               = 192,
    /// STATE-REFERENCES
    StateReferences                                                        = 4077,
    /// STATE-REQUEST-PORT-IREF
    StateRequestPortIref                                                   = 3527,
    /// STATIC-MEMORYS
    StaticMemorys                                                          = 304,
    /// STATIC-PART
    StaticPart                                                             = 225,
    /// STATIC-PARTS
    StaticParts                                                            = 1335,
    /// STATIC-SLOT-DURATION
    StaticSlotDuration                                                     = 2571,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 2271,
    /// STATIC-SOCKET-CONNECTION-REF
    StaticSocketConnectionRef                                              = 1876,
    /// STATIC-SOCKET-CONNECTIONS
    StaticSocketConnections                                                = 4569,
    /// STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR
    StatusBitHandlingTestFailedSinceLastClear                              = 5912,
    /// STATUS-BIT-STORAGE-TEST-FAILED
    StatusBitStorageTestFailed                                             = 3207,
    /// STATUS-ID
    StatusId                                                               = 4247,
    /// STATUS-IREF
    StatusIref                                                             = 1116,
    /// STATUS-REF
    StatusRef                                                              = 1041,
    /// STATUS-SUB-TLV
    StatusSubTlv                                                           = 3948,
    /// STATUSS
    Statuss                                                                = 804,
    /// STD
    Std                                                                    = 2032,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 5339,
    /// STEP-SIZE
    StepSize                                                               = 5795,
    /// STIM-ENABLER
    StimEnabler                                                            = 5944,
    /// STIMULUS-REF
    StimulusRef                                                            = 4924,
    /// STOP
    Stop                                                                   = 1725,
    /// STOP-IREF
    StopIref                                                               = 1176,
    /// STORAGE-CONDITION-GROUP-REF
    StorageConditionGroupRef                                               = 3967,
    /// STORAGE-CONDITIONS
    StorageConditions                                                      = 2181,
    /// STORAGE-STATE-EVALUATION
    StorageStateEvaluation                                                 = 3286,
    /// STORE-AT-SHUTDOWN
    StoreAtShutdown                                                        = 1664,
    /// STORE-CYCLIC
    StoreCyclic                                                            = 2345,
    /// STORE-EMERGENCY
    StoreEmergency                                                         = 5760,
    /// STORE-EVENT-ENABLED
    StoreEventEnabled                                                      = 5544,
    /// STORE-EVENT-SUPPORT
    StoreEventSupport                                                      = 2896,
    /// STORE-IMMEDIATE
    StoreImmediate                                                         = 5523,
    /// STORE-ON-CHANGE
    StoreOnChange                                                          = 765,
    /// STORING
    Storing                                                                = 2006,
    /// STREAM-IDENTIFIER
    StreamIdentifier                                                       = 1481,
    /// STRING-ENCODING
    StringEncoding                                                         = 3187,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 3220,
    /// SUB
    Sub                                                                    = 4828,
    /// SUB-ATTRIBUTE-TAILORINGS
    SubAttributeTailorings                                                 = 3584,
    /// SUB-CONTAINERS
    SubContainers                                                          = 4182,
    /// SUB-DOMAIN-REF
    SubDomainRef                                                           = 4893,
    /// SUB-DOMAIN-REFS
    SubDomainRefs                                                          = 3336,
    /// SUB-ELEMENT-MAPPING
    SubElementMapping                                                      = 56,
    /// SUB-ELEMENT-MAPPINGS
    SubElementMappings                                                     = 2318,
    /// SUB-ELEMENTS
    SubElements                                                            = 5604,
    /// SUB-FUNCTION-REF
    SubFunctionRef                                                         = 629,
    /// SUB-FUNCTION-REFS
    SubFunctionRefs                                                        = 2406,
    /// SUB-GROUP-REF
    SubGroupRef                                                            = 4919,
    /// SUB-GROUP-REFS
    SubGroupRefs                                                           = 3462,
    /// SUB-NODE-CHANNEL-REF
    SubNodeChannelRef                                                      = 5236,
    /// SUB-NODE-CHANNELS
    SubNodeChannels                                                        = 402,
    /// SUB-NODE-NUMBER
    SubNodeNumber                                                          = 1199,
    /// SUB-SDG-REF
    SubSdgRef                                                              = 1749,
    /// SUB-SOFTWARE-CLUSTER-REF
    SubSoftwareClusterRef                                                  = 1184,
    /// SUB-SOFTWARE-CLUSTER-REFS
    SubSoftwareClusterRefs                                                 = 5477,
    /// SUB-TLV-CONFIG
    SubTlvConfig                                                           = 2550,
    /// SUB-TYPE
    SubType                                                                = 3342,
    /// SUBFUNCTION
    Subfunction                                                            = 3186,
    /// SUBFUNCTIONS
    Subfunctions                                                           = 5308,
    /// SUBMESSAGE-TYPE
    SubmessageType                                                         = 3295,
    /// SUBNET-NUMBER
    SubnetNumber                                                           = 4988,
    /// SUBSCRIBE-EVENTGROUP-RETRY-DELAY
    SubscribeEventgroupRetryDelay                                          = 5527,
    /// SUBSCRIBE-EVENTGROUP-RETRY-MAX
    SubscribeEventgroupRetryMax                                            = 2897,
    /// SUBSTITUTED-FRAME-REF
    SubstitutedFrameRef                                                    = 60,
    /// SUBSTITUTED-FRAME-REFS
    SubstitutedFrameRefs                                                   = 2912,
    /// SUBTITLE
    Subtitle                                                               = 2326,
    /// SUCCESSOR-REF
    SuccessorRef                                                           = 5198,
    /// SUCCESSOR-REFS
    SuccessorRefs                                                          = 2787,
    /// SUP
    Sup                                                                    = 287,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 2400,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF
    SupervisedEntityCheckpointNeedsRef                                     = 3859,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL
    SupervisedEntityCheckpointNeedsRefConditional                          = 897,
    /// SUPERVISED-ENTITY-ID
    SupervisedEntityId                                                     = 1485,
    /// SUPERVISED-ENTITY-IREF
    SupervisedEntityIref                                                   = 5154,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 1082,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 1674,
    /// SUPERVISION-CONDITION
    SupervisionCondition                                                   = 5960,
    /// SUPERVISION-CYCLE
    SupervisionCycle                                                       = 2476,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 2377,
    /// SUPERVISION-ENTITYS
    SupervisionEntitys                                                     = 5541,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 4287,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 4241,
    /// SUPERVISION-MODE-CONDITIONS
    SupervisionModeConditions                                              = 800,
    /// SUPERVISION-MODES
    SupervisionModes                                                       = 4514,
    /// SUPERVISION-REF
    SupervisionRef                                                         = 1981,
    /// SUPPLIER-ID
    SupplierId                                                             = 5291,
    /// SUPPLIER-VALIDATION-ORDER-REF
    SupplierValidationOrderRef                                             = 4109,
    /// SUPPLIER-VALIDATION-ORDER-REFS
    SupplierValidationOrderRefs                                            = 5441,
    /// SUPPORT-BUFFER-LOCKING
    SupportBufferLocking                                                   = 4618,
    /// SUPPORT-DIRTY-FLAG
    SupportDirtyFlag                                                       = 1426,
    /// SUPPORT-INFO
    SupportInfo                                                            = 3756,
    /// SUPPORT-INFO-BIT
    SupportInfoBit                                                         = 830,
    /// SUPPORT-INFO-BYTE
    SupportInfoByte                                                        = 2985,
    /// SUPPORTED-CIPHER-SUITES
    SupportedCipherSuites                                                  = 5144,
    /// SUPPORTED-CONFIG-VARIANT
    SupportedConfigVariant                                                 = 1564,
    /// SUPPORTED-CONFIG-VARIANTS
    SupportedConfigVariants                                                = 4560,
    /// SUPPORTED-FEATURES
    SupportedFeatures                                                      = 5042,
    /// SUPPORTED-TIMER-GRANULARITY
    SupportedTimerGranularity                                              = 339,
    /// SUPPORTING-MATERIAL
    SupportingMaterial                                                     = 5316,
    /// SUPPORTS-ASYNCHRONOUS-MODE-SWITCH
    SupportsAsynchronousModeSwitch                                         = 4103,
    /// SUPPORTS-MULTIPLE-INSTANTIATION
    SupportsMultipleInstantiation                                          = 1727,
    /// SUPPORTS-MULTIPLE-NOTIFIER-SETS
    SupportsMultipleNotifierSets                                           = 2108,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 5785,
    /// SW-ADDR-METHOD-REF
    SwAddrMethodRef                                                        = 1494,
    /// SW-ADDRMETHOD-REF
    SwAddrmethodRef                                                        = 2799,
    /// SW-ALIGNMENT
    SwAlignment                                                            = 761,
    /// SW-ARRAYSIZE
    SwArraysize                                                            = 2839,
    /// SW-AXIS-CONT
    SwAxisCont                                                             = 2425,
    /// SW-AXIS-CONTS
    SwAxisConts                                                            = 3643,
    /// SW-AXIS-GENERIC
    SwAxisGeneric                                                          = 529,
    /// SW-AXIS-GROUPED
    SwAxisGrouped                                                          = 954,
    /// SW-AXIS-INDEX
    SwAxisIndex                                                            = 1943,
    /// SW-AXIS-INDIVIDUAL
    SwAxisIndividual                                                       = 4973,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 970,
    /// SW-AXIS-TYPE-REF
    SwAxisTypeRef                                                          = 4364,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 4813,
    /// SW-BIT-REPRESENTATION
    SwBitRepresentation                                                    = 332,
    /// SW-CALIBRATION-ACCESS
    SwCalibrationAccess                                                    = 3881,
    /// SW-CALIBRATION-NOTES
    SwCalibrationNotes                                                     = 3162,
    /// SW-CALIBRATION-NOTESS
    SwCalibrationNotess                                                    = 2594,
    /// SW-CALPRM-AXIS
    SwCalprmAxis                                                           = 2671,
    /// SW-CALPRM-AXIS-SET
    SwCalprmAxisSet                                                        = 5724,
    /// SW-CALPRM-IMPL
    SwCalprmImpl                                                           = 1562,
    /// SW-CALPRM-IMPLS
    SwCalprmImpls                                                          = 5606,
    /// SW-CALPRM-PROTOTYPE-REF
    SwCalprmPrototypeRef                                                   = 2297,
    /// SW-CARB-DOC
    SwCarbDoc                                                              = 1868,
    /// SW-CARB-DOCS
    SwCarbDocs                                                             = 5626,
    /// SW-CLASS-ATTR-INSTANCE-IMPL
    SwClassAttrInstanceImpl                                                = 3673,
    /// SW-CLASS-IMPL
    SwClassImpl                                                            = 1300,
    /// SW-CLASS-IMPLS
    SwClassImpls                                                           = 451,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 3792,
    /// SW-CLASS-PROTOTYPE-REF
    SwClassPrototypeRef                                                    = 4506,
    /// SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF
    SwClusterDesignProvidedPortIref                                        = 2268,
    /// SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF
    SwClusterDesignRequiredPortIref                                        = 1459,
    /// SW-CLUSTER-MAPPINGS
    SwClusterMappings                                                      = 1632,
    /// SW-CLUSTER-REF
    SwClusterRef                                                           = 4220,
    /// SW-CLUSTER-REFS
    SwClusterRefs                                                          = 2961,
    /// SW-CLUSTERS
    SwClusters                                                             = 2330,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 864,
    /// SW-COMP-TO-ECU-MAPPING-REF
    SwCompToEcuMappingRef                                                  = 1779,
    /// SW-COMP-TO-ECU-MAPPING-REFS
    SwCompToEcuMappingRefs                                                 = 414,
    /// SW-COMPARISON-VARIABLES
    SwComparisonVariables                                                  = 5856,
    /// SW-COMPONENT-ASSIGNMENTS
    SwComponentAssignments                                                 = 1263,
    /// SW-COMPONENT-DOCUMENTATION
    SwComponentDocumentation                                               = 764,
    /// SW-COMPONENT-DOCUMENTATIONS
    SwComponentDocumentations                                              = 5870,
    /// SW-COMPONENT-IREF
    SwComponentIref                                                        = 4374,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 4845,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 286,
    /// SW-COMPONENT-PROTOTYPE-ASSIGNMENT
    SwComponentPrototypeAssignment                                         = 4350,
    /// SW-COMPONENT-PROTOTYPE-IREF
    SwComponentPrototypeIref                                               = 4339,
    /// SW-COMPONENT-PROTOTYPE-REF
    SwComponentPrototypeRef                                                = 3705,
    /// SW-COMPOSITIONS
    SwCompositions                                                         = 2503,
    /// SW-DATA-DEF-PROPS
    SwDataDefProps                                                         = 3603,
    /// SW-DATA-DEF-PROPS-CONDITIONAL
    SwDataDefPropsConditional                                              = 5486,
    /// SW-DATA-DEF-PROPS-VARIANTS
    SwDataDefPropsVariants                                                 = 2884,
    /// SW-DATA-DEPENDENCY
    SwDataDependency                                                       = 5397,
    /// SW-DATA-DEPENDENCY-ARGS
    SwDataDependencyArgs                                                   = 2229,
    /// SW-DATA-DEPENDENCY-FORMULA
    SwDataDependencyFormula                                                = 4282,
    /// SW-DIAGNOSTICS-NOTES
    SwDiagnosticsNotes                                                     = 306,
    /// SW-DIAGNOSTICS-NOTESS
    SwDiagnosticsNotess                                                    = 2850,
    /// SW-FEATURE-DEF
    SwFeatureDef                                                           = 971,
    /// SW-FEATURE-DEFS
    SwFeatureDefs                                                          = 4985,
    /// SW-FEATURE-DESC
    SwFeatureDesc                                                          = 2060,
    /// SW-FEATURE-DESCS
    SwFeatureDescs                                                         = 2709,
    /// SW-FILL-CHARACTER
    SwFillCharacter                                                        = 511,
    /// SW-GENERIC-AXIS-DESC
    SwGenericAxisDesc                                                      = 5896,
    /// SW-GENERIC-AXIS-PARAM
    SwGenericAxisParam                                                     = 1569,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 112,
    /// SW-GENERIC-AXIS-PARAM-TYPE-REF
    SwGenericAxisParamTypeRef                                              = 607,
    /// SW-GENERIC-AXIS-PARAM-TYPES
    SwGenericAxisParamTypes                                                = 4014,
    /// SW-GENERIC-AXIS-PARAMS
    SwGenericAxisParams                                                    = 3404,
    /// SW-HOST-VARIABLE
    SwHostVariable                                                         = 4125,
    /// SW-IMPL-MAPPINGS
    SwImplMappings                                                         = 3287,
    /// SW-IMPL-POLICY
    SwImplPolicy                                                           = 1691,
    /// SW-INTENDED-RESOLUTION
    SwIntendedResolution                                                   = 1854,
    /// SW-INTERPOLATION-METHOD
    SwInterpolationMethod                                                  = 1945,
    /// SW-IS-VIRTUAL
    SwIsVirtual                                                            = 183,
    /// SW-MAINTENANCE-NOTES
    SwMaintenanceNotes                                                     = 174,
    /// SW-MAINTENANCE-NOTESS
    SwMaintenanceNotess                                                    = 2317,
    /// SW-MAPPINGS
    SwMappings                                                             = 2991,
    /// SW-MAX-AXIS-POINTS
    SwMaxAxisPoints                                                        = 3092,
    /// SW-MAX-TEXT-SIZE
    SwMaxTextSize                                                          = 3058,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 1977,
    /// SW-MIN-AXIS-POINTS
    SwMinAxisPoints                                                        = 4378,
    /// SW-NUMBER-OF-AXIS-POINTS
    SwNumberOfAxisPoints                                                   = 1091,
    /// SW-POINTER-TARGET-PROPS
    SwPointerTargetProps                                                   = 1277,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 4060,
    /// SW-RECORD-LAYOUT-COMPONENT
    SwRecordLayoutComponent                                                = 4770,
    /// SW-RECORD-LAYOUT-GROUP
    SwRecordLayoutGroup                                                    = 3677,
    /// SW-RECORD-LAYOUT-GROUP-AXIS
    SwRecordLayoutGroupAxis                                                = 1686,
    /// SW-RECORD-LAYOUT-GROUP-FROM
    SwRecordLayoutGroupFrom                                                = 3633,
    /// SW-RECORD-LAYOUT-GROUP-INDEX
    SwRecordLayoutGroupIndex                                               = 1233,
    /// SW-RECORD-LAYOUT-GROUP-STEP
    SwRecordLayoutGroupStep                                                = 5893,
    /// SW-RECORD-LAYOUT-GROUP-TO
    SwRecordLayoutGroupTo                                                  = 5769,
    /// SW-RECORD-LAYOUT-REF
    SwRecordLayoutRef                                                      = 588,
    /// SW-RECORD-LAYOUT-V
    SwRecordLayoutV                                                        = 4798,
    /// SW-RECORD-LAYOUT-V-AXIS
    SwRecordLayoutVAxis                                                    = 54,
    /// SW-RECORD-LAYOUT-V-FIX-VALUE
    SwRecordLayoutVFixValue                                                = 3621,
    /// SW-RECORD-LAYOUT-V-INDEX
    SwRecordLayoutVIndex                                                   = 1162,
    /// SW-RECORD-LAYOUT-V-PROP
    SwRecordLayoutVProp                                                    = 863,
    /// SW-REFRESH-TIMING
    SwRefreshTiming                                                        = 735,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 1731,
    /// SW-SERVICE-IMPL-POLICY
    SwServiceImplPolicy                                                    = 4619,
    /// SW-SYSCOND
    SwSyscond                                                              = 5828,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 5594,
    /// SW-SYSTEMCONST-REF
    SwSystemconstRef                                                       = 3135,
    /// SW-SYSTEMCONST-VALUE
    SwSystemconstValue                                                     = 4026,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 1100,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REF
    SwSystemconstantValueSetRef                                            = 1932,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REFS
    SwSystemconstantValueSetRefs                                           = 998,
    /// SW-SYSTEMCONSTANT-VALUES
    SwSystemconstantValues                                                 = 1915,
    /// SW-TEST-DESC
    SwTestDesc                                                             = 3156,
    /// SW-TEST-DESCS
    SwTestDescs                                                            = 1976,
    /// SW-TEXT-PROPS
    SwTextProps                                                            = 2401,
    /// SW-VALUE-BLOCK-SIZE
    SwValueBlockSize                                                       = 1704,
    /// SW-VALUE-BLOCK-SIZE-MULTS
    SwValueBlockSizeMults                                                  = 567,
    /// SW-VALUE-CONT
    SwValueCont                                                            = 1189,
    /// SW-VALUES-PHYS
    SwValuesPhys                                                           = 1626,
    /// SW-VARIABLE-IMPL
    SwVariableImpl                                                         = 2545,
    /// SW-VARIABLE-IMPLS
    SwVariableImpls                                                        = 3175,
    /// SW-VARIABLE-PROTOTYPE-REF
    SwVariablePrototypeRef                                                 = 867,
    /// SW-VARIABLE-REFS
    SwVariableRefs                                                         = 4479,
    /// SW-VERSION
    SwVersion                                                              = 2992,
    /// SWC-BEHAVIOR-REF
    SwcBehaviorRef                                                         = 1927,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 5576,
    /// SWC-BSW-MAPPING-REF
    SwcBswMappingRef                                                       = 3119,
    /// SWC-BSW-RUNNABLE-MAPPING
    SwcBswRunnableMapping                                                  = 4689,
    /// SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE
    SwcBswSynchronizedModeGroupPrototype                                   = 611,
    /// SWC-BSW-SYNCHRONIZED-TRIGGER
    SwcBswSynchronizedTrigger                                              = 594,
    /// SWC-EXCLUSIVE-AREA-POLICY
    SwcExclusiveAreaPolicy                                                 = 5169,
    /// SWC-FLAT-SERVICE-DEPENDENCY-REF
    SwcFlatServiceDependencyRef                                            = 4388,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 1800,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 4037,
    /// SWC-MAPPING-CONSTRAINT-REF
    SwcMappingConstraintRef                                                = 5120,
    /// SWC-MAPPING-CONSTRAINT-REFS
    SwcMappingConstraintRefs                                               = 3408,
    /// SWC-MODE-GROUP-IREF
    SwcModeGroupIref                                                       = 2097,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 647,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 5105,
    /// SWC-MODE-SWITCH-EVENT-REF
    SwcModeSwitchEventRef                                                  = 4565,
    /// SWC-RUNNABLE-REF
    SwcRunnableRef                                                         = 5939,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 4955,
    /// SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    SwcServiceDependencyInExecutableIref                                   = 697,
    /// SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    SwcServiceDependencyInSystemIref                                       = 2497,
    /// SWC-SERVICE-DEPENDENCY-IREF
    SwcServiceDependencyIref                                               = 3169,
    /// SWC-TIMING
    SwcTiming                                                              = 4236,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 3025,
    /// SWC-TO-APPLICATION-PARTITION-MAPPINGS
    SwcToApplicationPartitionMappings                                      = 252,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 4873,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT
    SwcToEcuMappingConstraint                                              = 1181,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE
    SwcToEcuMappingConstraintType                                          = 347,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 97,
    /// SWC-TO-SWC-OPERATION-ARGUMENTS
    SwcToSwcOperationArguments                                             = 4640,
    /// SWC-TO-SWC-SIGNAL
    SwcToSwcSignal                                                         = 4252,
    /// SWC-TRIGGER-IREF
    SwcTriggerIref                                                         = 3971,
    /// SYMBOL
    Symbol                                                                 = 3208,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 3611,
    /// SYMBOL-WINDOW
    SymbolWindow                                                           = 3640,
    /// SYMBOL-WINDOW-ACTION-POINT-OFFSET
    SymbolWindowActionPointOffset                                          = 5419,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 4742,
    /// SYMBOLIC-NAME-VALUE
    SymbolicNameValue                                                      = 3604,
    /// SYNC-CONFIRMATION-TIMEOUT
    SyncConfirmationTimeout                                                = 331,
    /// SYNC-COUNTER-INIT
    SyncCounterInit                                                        = 3059,
    /// SYNC-DATA-ID-LIST
    SyncDataIdList                                                         = 687,
    /// SYNC-DATA-ID-LISTS
    SyncDataIdLists                                                        = 3278,
    /// SYNC-FRAME-ID-COUNT-MAX
    SyncFrameIdCountMax                                                    = 1336,
    /// SYNC-INTERVAL
    SyncInterval                                                           = 2358,
    /// SYNC-JUMP-WIDTH
    SyncJumpWidth                                                          = 1680,
    /// SYNC-LOSS-THRESHOLD
    SyncLossThreshold                                                      = 3998,
    /// SYNC-LOSS-TIMEOUT
    SyncLossTimeout                                                        = 3345,
    /// SYNC-NODE-MAX
    SyncNodeMax                                                            = 96,
    /// SYNC-PERIOD
    SyncPeriod                                                             = 2791,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 5282,
    /// SYNCHRONIZATION-CONSTRAINT-TYPE
    SynchronizationConstraintType                                          = 4621,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 1611,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 2815,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 5656,
    /// SYNCHRONIZED-MODE-GROUPS
    SynchronizedModeGroups                                                 = 3595,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 1456,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 5719,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 4320,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 834,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 5589,
    /// SYNCHRONIZED-TRIGGERS
    SynchronizedTriggers                                                   = 5903,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 4467,
    /// SYSC-REF
    SyscRef                                                                = 2251,
    /// SYSC-STRING-REF
    SyscStringRef                                                          = 3607,
    /// SYSTEM
    System                                                                 = 5387,
    /// SYSTEM-DOCUMENTATIONS
    SystemDocumentations                                                   = 3043,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 4107,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 1040,
    /// SYSTEM-MEMORY-USAGES
    SystemMemoryUsages                                                     = 3907,
    /// SYSTEM-REF
    SystemRef                                                              = 4331,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 78,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 5,
    /// SYSTEM-SIGNAL-GROUP-REF
    SystemSignalGroupRef                                                   = 737,
    /// SYSTEM-SIGNAL-REF
    SystemSignalRef                                                        = 4285,
    /// SYSTEM-SIGNAL-REFS
    SystemSignalRefs                                                       = 869,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 353,
    /// SYSTEM-TRIGGERED-EVENTS
    SystemTriggeredEvents                                                  = 4647,
    /// SYSTEM-VERSION
    SystemVersion                                                          = 2661,
    /// TA-TYPE
    TaType                                                                 = 4790,
    /// TABLE
    Table                                                                  = 1995,
    /// TABLE-CAPTION
    TableCaption                                                           = 517,
    /// TABLE-ENTRYS
    TableEntrys                                                            = 2629,
    /// TAG-ID
    TagId                                                                  = 4523,
    /// TAG-WITH-OPTIONAL-VALUE
    TagWithOptionalValue                                                   = 5125,
    /// TAILORING-REF
    TailoringRef                                                           = 5714,
    /// TAILORING-REFS
    TailoringRefs                                                          = 5472,
    /// TARGET-APPLICATION-RECORD-ELEMENT-REF
    TargetApplicationRecordElementRef                                      = 3438,
    /// TARGET-CATEGORY
    TargetCategory                                                         = 1200,
    /// TARGET-CHANNEL
    TargetChannel                                                          = 5088,
    /// TARGET-COMPONENT-REF
    TargetComponentRef                                                     = 432,
    /// TARGET-DATA-ELEMENT-REF
    TargetDataElementRef                                                   = 5784,
    /// TARGET-DATA-PROTOTYPE-GROUP-REF
    TargetDataPrototypeGroupRef                                            = 5356,
    /// TARGET-DATA-PROTOTYPE-IN-CS-REF
    TargetDataPrototypeInCsRef                                             = 5758,
    /// TARGET-DATA-PROTOTYPE-IN-SR-REF
    TargetDataPrototypeInSrRef                                             = 841,
    /// TARGET-DATA-PROTOTYPE-REF
    TargetDataPrototypeRef                                                 = 2241,
    /// TARGET-DATA-PROTOYPE-REF
    TargetDataProtoypeRef                                                  = 5318,
    /// TARGET-EEC-REF
    TargetEecRef                                                           = 4881,
    /// TARGET-EEC-REFS
    TargetEecRefs                                                          = 1926,
    /// TARGET-ELEMENT-REF
    TargetElementRef                                                       = 4207,
    /// TARGET-EVENT-REF
    TargetEventRef                                                         = 2990,
    /// TARGET-EVENT-REFS
    TargetEventRefs                                                        = 3028,
    /// TARGET-FIELD-REF
    TargetFieldRef                                                         = 4554,
    /// TARGET-FRAME-REF
    TargetFrameRef                                                         = 865,
    /// TARGET-HEALTH-CHANNEL-REF
    TargetHealthChannelRef                                                 = 1852,
    /// TARGET-I-PDU
    TargetIPdu                                                             = 2502,
    /// TARGET-I-PDU-REF
    TargetIPduRef                                                          = 5473,
    /// TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TargetImplementationDataTypeElementRef                                 = 512,
    /// TARGET-METHOD-REF
    TargetMethodRef                                                        = 1513,
    /// TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    TargetModeDeclarationGroupPrototypeRef                                 = 1147,
    /// TARGET-MODE-DECLARATION-REF
    TargetModeDeclarationRef                                               = 5424,
    /// TARGET-MODE-GROUP-REF
    TargetModeGroupRef                                                     = 5766,
    /// TARGET-MODE-REF
    TargetModeRef                                                          = 2928,
    /// TARGET-MODULE-ID
    TargetModuleId                                                         = 3872,
    /// TARGET-MODULE-REFS
    TargetModuleRefs                                                       = 1776,
    /// TARGET-OPERATION-REF
    TargetOperationRef                                                     = 4829,
    /// TARGET-P-PORT-PROTOTYPE-REF
    TargetPPortPrototypeRef                                                = 2115,
    /// TARGET-P-PORT-REF
    TargetPPortRef                                                         = 3820,
    /// TARGET-PARAMETER-DATA-PROTOTYPE-REF
    TargetParameterDataPrototypeRef                                        = 2786,
    /// TARGET-PDU-TRIGGERINGS
    TargetPduTriggerings                                                   = 5736,
    /// TARGET-PHM-CHECKPOINT-REF
    TargetPhmCheckpointRef                                                 = 418,
    /// TARGET-PHM-SUPERVISED-ENTITY-IREF
    TargetPhmSupervisedEntityIref                                          = 2606,
    /// TARGET-PORT-PROTOTYPE-REF
    TargetPortPrototypeRef                                                 = 4605,
    /// TARGET-POSSIBLE-ERROR-REF
    TargetPossibleErrorRef                                                 = 5485,
    /// TARGET-PROVIDED-OPERATION-REF
    TargetProvidedOperationRef                                             = 1960,
    /// TARGET-R-PORT-PROTOTYPE-REF
    TargetRPortPrototypeRef                                                = 4658,
    /// TARGET-R-PORT-REF
    TargetRPortRef                                                         = 4150,
    /// TARGET-REF
    TargetRef                                                              = 3758,
    /// TARGET-REFS
    TargetRefs                                                             = 3213,
    /// TARGET-REQUIRED-OPERATION-REF
    TargetRequiredOperationRef                                             = 2726,
    /// TARGET-RTE-EVENT-REF
    TargetRteEventRef                                                      = 1826,
    /// TARGET-RUNNABLE-ENTITY-GROUP-REF
    TargetRunnableEntityGroupRef                                           = 5834,
    /// TARGET-RUNNABLE-ENTITY-REF
    TargetRunnableEntityRef                                                = 507,
    /// TARGET-SIGNAL-REF
    TargetSignalRef                                                        = 5135,
    /// TARGET-STATUS-REF
    TargetStatusRef                                                        = 4262,
    /// TARGET-SWC-SERVICE-DEPENDENCY-REF
    TargetSwcServiceDependencyRef                                          = 4725,
    /// TARGET-TRIGGER-REF
    TargetTriggerRef                                                       = 3619,
    /// TARGET-VARIABLE-ACCESS-REF
    TargetVariableAccessRef                                                = 4741,
    /// TARGET-VARIABLE-DATA-PROTOTYPE-REF
    TargetVariableDataPrototypeRef                                         = 5074,
    /// TASK
    Task                                                                   = 127,
    /// TBODY
    Tbody                                                                  = 3267,
    /// TCP-ALIVE-CHECK-RESPONSE-TIMEOUT
    TcpAliveCheckResponseTimeout                                           = 4720,
    /// TCP-CONGESTION-AVOIDANCE-ENABLED
    TcpCongestionAvoidanceEnabled                                          = 5070,
    /// TCP-CONNECT-TIMEOUT
    TcpConnectTimeout                                                      = 3768,
    /// TCP-DELAYED-ACK-TIMEOUT
    TcpDelayedAckTimeout                                                   = 5933,
    /// TCP-FAST-RECOVERY-ENABLED
    TcpFastRecoveryEnabled                                                 = 360,
    /// TCP-FAST-RETRANSMIT-ENABLED
    TcpFastRetransmitEnabled                                               = 3371,
    /// TCP-FIN-WAIT-2-TIMEOUT
    TcpFinWait2Timeout                                                     = 3929,
    /// TCP-GENERAL-INACTIVITY-TIME
    TcpGeneralInactivityTime                                               = 3846,
    /// TCP-INITIAL-INACTIVITY-TIME
    TcpInitialInactivityTime                                               = 3521,
    /// TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP
    TcpIpArpNumGratuitousArpOnStartup                                      = 5197,
    /// TCP-IP-ARP-PACKET-QUEUE-ENABLED
    TcpIpArpPacketQueueEnabled                                             = 4699,
    /// TCP-IP-ARP-REQUEST-TIMEOUT
    TcpIpArpRequestTimeout                                                 = 197,
    /// TCP-IP-ARP-TABLE-ENTRY-TIMEOUT
    TcpIpArpTableEntryTimeout                                              = 5256,
    /// TCP-IP-AUTO-IP-INIT-TIMEOUT
    TcpIpAutoIpInitTimeout                                                 = 2263,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MAX
    TcpIpDhcpV6CnfDelayMax                                                 = 2886,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MIN
    TcpIpDhcpV6CnfDelayMin                                                 = 3189,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MAX
    TcpIpDhcpV6InfDelayMax                                                 = 4931,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MIN
    TcpIpDhcpV6InfDelayMin                                                 = 1786,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MAX
    TcpIpDhcpV6SolDelayMax                                                 = 2428,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MIN
    TcpIpDhcpV6SolDelayMin                                                 = 2951,
    /// TCP-IP-ICMP-PROPS-REF
    TcpIpIcmpPropsRef                                                      = 397,
    /// TCP-IP-ICMP-PROPS-REFS
    TcpIpIcmpPropsRefs                                                     = 2914,
    /// TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED
    TcpIpIcmpV4EchoReplyEnabled                                            = 1220,
    /// TCP-IP-ICMP-V-4-TTL
    TcpIpIcmpV4Ttl                                                         = 1267,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION
    TcpIpIcmpV6EchoReplyAvoidFragmentation                                 = 4169,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED
    TcpIpIcmpV6EchoReplyEnabled                                            = 3583,
    /// TCP-IP-ICMP-V-6-HOP-LIMIT
    TcpIpIcmpV6HopLimit                                                    = 45,
    /// TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED
    TcpIpIcmpV6MsgDestinationUnreachableEnabled                            = 3773,
    /// TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED
    TcpIpIcmpV6MsgParameterProblemEnabled                                  = 2385,
    /// TCP-IP-IP-FRAGMENTATION-RX-ENABLED
    TcpIpIpFragmentationRxEnabled                                          = 3226,
    /// TCP-IP-IP-NUM-FRAGMENTS
    TcpIpIpNumFragments                                                    = 3981,
    /// TCP-IP-IP-NUM-REASS-DGRAMS
    TcpIpIpNumReassDgrams                                                  = 3235,
    /// TCP-IP-IP-REASS-TIMEOUT
    TcpIpIpReassTimeout                                                    = 2147,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-COUNT
    TcpIpIpReassemblyBufferCount                                           = 1146,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-SIZE
    TcpIpIpReassemblyBufferSize                                            = 4641,
    /// TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT
    TcpIpIpReassemblySegmentCount                                          = 1464,
    /// TCP-IP-IP-REASSEMBLY-TIMEOUT
    TcpIpIpReassemblyTimeout                                               = 5300,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT
    TcpIpIpTxFragmentBufferCount                                           = 4894,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE
    TcpIpIpTxFragmentBufferSize                                            = 2237,
    /// TCP-IP-NDP-DEFAULT-REACHABLE-TIME
    TcpIpNdpDefaultReachableTime                                           = 1622,
    /// TCP-IP-NDP-DEFAULT-RETRANS-TIMER
    TcpIpNdpDefaultRetransTimer                                            = 4889,
    /// TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE
    TcpIpNdpDefaultRouterListSize                                          = 274,
    /// TCP-IP-NDP-DEFENSIVE-PROCESSING
    TcpIpNdpDefensiveProcessing                                            = 322,
    /// TCP-IP-NDP-DELAY-FIRST-PROBE-TIME
    TcpIpNdpDelayFirstProbeTime                                            = 476,
    /// TCP-IP-NDP-DESTINATION-CACHE-SIZE
    TcpIpNdpDestinationCacheSize                                           = 4693,
    /// TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED
    TcpIpNdpDynamicHopLimitEnabled                                         = 3114,
    /// TCP-IP-NDP-DYNAMIC-MTU-ENABLED
    TcpIpNdpDynamicMtuEnabled                                              = 4595,
    /// TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED
    TcpIpNdpDynamicReachableTimeEnabled                                    = 5840,
    /// TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED
    TcpIpNdpDynamicRetransTimeEnabled                                      = 1444,
    /// TCP-IP-NDP-MAX-RANDOM-FACTOR
    TcpIpNdpMaxRandomFactor                                                = 2119,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY
    TcpIpNdpMaxRtrSolicitationDelay                                        = 667,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATIONS
    TcpIpNdpMaxRtrSolicitations                                            = 5806,
    /// TCP-IP-NDP-MIN-RANDOM-FACTOR
    TcpIpNdpMinRandomFactor                                                = 8,
    /// TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED
    TcpIpNdpNeighborUnreachabilityDetectionEnabled                         = 892,
    /// TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS
    TcpIpNdpNumMulticastSolicitations                                      = 2521,
    /// TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS
    TcpIpNdpNumUnicastSolicitations                                        = 1726,
    /// TCP-IP-NDP-PACKET-QUEUE-ENABLED
    TcpIpNdpPacketQueueEnabled                                             = 5141,
    /// TCP-IP-NDP-PREFIX-LIST-SIZE
    TcpIpNdpPrefixListSize                                                 = 1929,
    /// TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED
    TcpIpNdpRandomReachableTimeEnabled                                     = 3953,
    /// TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED
    TcpIpNdpRndRtrSolicitationDelayEnabled                                 = 4594,
    /// TCP-IP-NDP-RTR-SOLICITATION-INTERVAL
    TcpIpNdpRtrSolicitationInterval                                        = 3775,
    /// TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS
    TcpIpNdpSlaacDadNumberOfTransmissions                                  = 3205,
    /// TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY
    TcpIpNdpSlaacDadRetransmissionDelay                                    = 3238,
    /// TCP-IP-NDP-SLAAC-DELAY-ENABLED
    TcpIpNdpSlaacDelayEnabled                                              = 2968,
    /// TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED
    TcpIpNdpSlaacOptimisticDadEnabled                                      = 5084,
    /// TCP-IP-PROPS-REF
    TcpIpPropsRef                                                          = 4538,
    /// TCP-IP-PROPS-REFS
    TcpIpPropsRefs                                                         = 3192,
    /// TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC
    TcpIpTlsUseSecurityExtensionForceEncryptThenMac                        = 269,
    /// TCP-KEEP-ALIVE-ENABLED
    TcpKeepAliveEnabled                                                    = 2873,
    /// TCP-KEEP-ALIVE-INTERVAL
    TcpKeepAliveInterval                                                   = 4698,
    /// TCP-KEEP-ALIVE-PROBES-MAX
    TcpKeepAliveProbesMax                                                  = 5019,
    /// TCP-KEEP-ALIVE-TIME
    TcpKeepAliveTime                                                       = 5565,
    /// TCP-LOCAL-PORT
    TcpLocalPort                                                           = 1860,
    /// TCP-MAX-RTX
    TcpMaxRtx                                                              = 4030,
    /// TCP-MSL
    TcpMsl                                                                 = 5060,
    /// TCP-NAGLE-ENABLED
    TcpNagleEnabled                                                        = 3368,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 4994,
    /// TCP-OPTION-FILTER-LISTS
    TcpOptionFilterLists                                                   = 91,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 498,
    /// TCP-PORT
    TcpPort                                                                = 1855,
    /// TCP-PORT-REF
    TcpPortRef                                                             = 4824,
    /// TCP-PROPS
    TcpProps                                                               = 3276,
    /// TCP-RECEIVE-WINDOW-MAX
    TcpReceiveWindowMax                                                    = 5289,
    /// TCP-REMOTE-PORT
    TcpRemotePort                                                          = 5800,
    /// TCP-RETRANSMISSION-TIMEOUT
    TcpRetransmissionTimeout                                               = 2776,
    /// TCP-ROLE
    TcpRole                                                                = 40,
    /// TCP-RULE
    TcpRule                                                                = 5137,
    /// TCP-SLOW-START-ENABLED
    TcpSlowStartEnabled                                                    = 5686,
    /// TCP-SYN-MAX-RTX
    TcpSynMaxRtx                                                           = 2279,
    /// TCP-SYN-RECEIVED-TIMEOUT
    TcpSynReceivedTimeout                                                  = 4968,
    /// TCP-TP
    TcpTp                                                                  = 1558,
    /// TCP-TP-CONFIG
    TcpTpConfig                                                            = 2098,
    /// TCP-TP-PORT
    TcpTpPort                                                              = 5100,
    /// TCP-TTL
    TcpTtl                                                                 = 466,
    /// TCP-UDP-CONFIG
    TcpUdpConfig                                                           = 3575,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 2300,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 4858,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 596,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS
    TdCpSoftwareClusterResourceToTdMappings                                = 965,
    /// TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS
    TdCpSoftwareClusterToTdMappings                                        = 3556,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 1872,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE
    TdEventBswInternalBehaviorType                                         = 5727,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 984,
    /// TD-EVENT-BSW-MODE-DECLARATION-TYPE
    TdEventBswModeDeclarationType                                          = 185,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 2633,
    /// TD-EVENT-BSW-MODULE-TYPE
    TdEventBswModuleType                                                   = 4123,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 3793,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 5573,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 2512,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 3717,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 4198,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 2993,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 5436,
    /// TD-EVENT-MODE-DECLARATION-TYPE
    TdEventModeDeclarationType                                             = 5911,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 362,
    /// TD-EVENT-OPERATION-TYPE
    TdEventOperationType                                                   = 1137,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 592,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE
    TdEventServiceInstanceDiscoveryType                                    = 3513,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 1000,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE
    TdEventServiceInstanceEventType                                        = 1906,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 2681,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE
    TdEventServiceInstanceFieldType                                        = 38,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 107,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE
    TdEventServiceInstanceMethodType                                       = 5580,
    /// TD-EVENT-SLLET-PORT
    TdEventSlletPort                                                       = 981,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 2648,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 2072,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE
    TdEventSwcInternalBehaviorType                                         = 4986,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 3759,
    /// TD-EVENT-TRIGGER-TYPE
    TdEventTriggerType                                                     = 2019,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 3694,
    /// TD-EVENT-TYPE
    TdEventType                                                            = 1155,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 3662,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE
    TdEventVariableDataPrototypeType                                       = 4057,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 346,
    /// TD-HEADER-ID-FILTERS
    TdHeaderIdFilters                                                      = 181,
    /// TD-HEADER-ID-RANGE
    TdHeaderIdRange                                                        = 3180,
    /// TD-PDU-TRIGGERING-FILTER-REF
    TdPduTriggeringFilterRef                                               = 5492,
    /// TD-PDU-TRIGGERING-FILTER-REFS
    TdPduTriggeringFilterRefs                                              = 2582,
    /// TDLET-ZONE-CLOCK
    TdletZoneClock                                                         = 3879,
    /// TEAR-DOWN-ACTION-REF
    TearDownActionRef                                                      = 5617,
    /// TEAR-DOWN-ACTION-REFS
    TearDownActionRefs                                                     = 3353,
    /// TEMPERATURE-EXP
    TemperatureExp                                                         = 631,
    /// TEMPLATE-ARGUMENTS
    TemplateArguments                                                      = 2137,
    /// TEMPLATE-TYPE-REF
    TemplateTypeRef                                                        = 4609,
    /// TERMINATING-CHECKPOINT-REF
    TerminatingCheckpointRef                                               = 1117,
    /// TERMINATING-CHECKPOINT-TIMEOUT-UNTIL-TERMINATION
    TerminatingCheckpointTimeoutUntilTermination                           = 5865,
    /// TERMINATION-BEHAVIOR
    TerminationBehavior                                                    = 6,
    /// TEST-ID
    TestId                                                                 = 4676,
    /// TEST-ID-REF
    TestIdRef                                                              = 328,
    /// TEST-IDENTIFIER
    TestIdentifier                                                         = 4601,
    /// TEST-PATTERN
    TestPattern                                                            = 1192,
    /// TEST-RESULT-REF
    TestResultRef                                                          = 21,
    /// TESTED-ITEM-REF
    TestedItemRef                                                          = 844,
    /// TESTED-ITEM-REFS
    TestedItemRefs                                                         = 3356,
    /// TEX-MATH
    TexMath                                                                = 182,
    /// TEXT
    Text                                                                   = 824,
    /// TEXT-TABLE-MAPPING
    TextTableMapping                                                       = 2209,
    /// TEXT-TABLE-MAPPINGS
    TextTableMappings                                                      = 396,
    /// TEXT-TABLE-VALUE-PAIR
    TextTableValuePair                                                     = 3271,
    /// TEXT-VALUE-SPECIFICATION
    TextValueSpecification                                                 = 5168,
    /// TEXTUAL-CONDITION
    TextualCondition                                                       = 4042,
    /// TFOOT
    Tfoot                                                                  = 2003,
    /// TGROUP
    Tgroup                                                                 = 2706,
    /// THEAD
    Thead                                                                  = 3996,
    /// THRESHOLD
    Threshold                                                              = 490,
    /// THRESHOLD-NUMBER
    ThresholdNumber                                                        = 4997,
    /// THRESHOLD-SIZE
    ThresholdSize                                                          = 257,
    /// TIME-BASE
    TimeBase                                                               = 430,
    /// TIME-BASE-CONSUMER-REF
    TimeBaseConsumerRef                                                    = 1284,
    /// TIME-BASE-JITTER
    TimeBaseJitter                                                         = 117,
    /// TIME-BASE-KIND
    TimeBaseKind                                                           = 4547,
    /// TIME-BASE-PROVIDER-REF
    TimeBaseProviderRef                                                    = 4431,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 2894,
    /// TIME-BASE-RESOURCE-REF
    TimeBaseResourceRef                                                    = 4637,
    /// TIME-BASE-RESOURCE-REF-CONDITIONAL
    TimeBaseResourceRefConditional                                         = 738,
    /// TIME-BASE-RESOURCE-REFS
    TimeBaseResourceRefs                                                   = 5664,
    /// TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE
    TimeBasedFdcThresholdStorageValue                                      = 3686,
    /// TIME-BASES
    TimeBases                                                              = 3498,
    /// TIME-BR
    TimeBr                                                                 = 1954,
    /// TIME-BUFFER
    TimeBuffer                                                             = 2663,
    /// TIME-CS
    TimeCs                                                                 = 1990,
    /// TIME-EXP
    TimeExp                                                                = 5434,
    /// TIME-FAILED-THRESHOLD
    TimeFailedThreshold                                                    = 5117,
    /// TIME-FR-IF
    TimeFrIf                                                               = 9,
    /// TIME-HARDWARE-CORRECTION-THRESHOLD
    TimeHardwareCorrectionThreshold                                        = 4866,
    /// TIME-INTERVAL
    TimeInterval                                                           = 2987,
    /// TIME-LEAP-FUTURE-THRESHOLD
    TimeLeapFutureThreshold                                                = 1202,
    /// TIME-LEAP-HEALING-COUNTER
    TimeLeapHealingCounter                                                 = 4551,
    /// TIME-LEAP-PAST-THRESHOLD
    TimeLeapPastThreshold                                                  = 2997,
    /// TIME-MARK
    TimeMark                                                               = 2965,
    /// TIME-MASTER-PRIORITY
    TimeMasterPriority                                                     = 5716,
    /// TIME-OFFSET
    TimeOffset                                                             = 1470,
    /// TIME-PASSED-THRESHOLD
    TimePassedThreshold                                                    = 1043,
    /// TIME-PERIOD
    TimePeriod                                                             = 3973,
    /// TIME-SEG-1
    TimeSeg1                                                               = 2,
    /// TIME-SEG-2
    TimeSeg2                                                               = 4743,
    /// TIME-SUB-TLV
    TimeSubTlv                                                             = 2227,
    /// TIME-SYNC-CLIENT
    TimeSyncClient                                                         = 5414,
    /// TIME-SYNC-CORRECTION
    TimeSyncCorrection                                                     = 5788,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 4301,
    /// TIME-SYNC-P-PORT-PROTOTYPE-IREF
    TimeSyncPPortPrototypeIref                                             = 1254,
    /// TIME-SYNC-PORT-PROTOTYPE-IREF
    TimeSyncPortPrototypeIref                                              = 3160,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 2918,
    /// TIME-SYNC-R-PORT-PROTOTYPE-IREF
    TimeSyncRPortPrototypeIref                                             = 1101,
    /// TIME-SYNC-SERVER
    TimeSyncServer                                                         = 5253,
    /// TIME-SYNC-SERVER-IDENTIFIER
    TimeSyncServerIdentifier                                               = 1900,
    /// TIME-SYNC-SERVER-REF
    TimeSyncServerRef                                                      = 1046,
    /// TIME-SYNC-TECHNOLOGY
    TimeSyncTechnology                                                     = 3606,
    /// TIME-SYNCHRONIZATION
    TimeSynchronization                                                    = 444,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 1565,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 3127,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 1121,
    /// TIME-TO-LIVE
    TimeToLive                                                             = 2943,
    /// TIME-TRIGGERED-CAN-LEVEL
    TimeTriggeredCanLevel                                                  = 3592,
    /// TIME-VALUE
    TimeValue                                                              = 5245,
    /// TIME-VALUE-VALUE-VARIATION-POINT
    TimeValueValueVariationPoint                                           = 2378,
    /// TIMEOUT
    Timeout                                                                = 5851,
    /// TIMEOUT-AR
    TimeoutAr                                                              = 689,
    /// TIMEOUT-AS
    TimeoutAs                                                              = 3330,
    /// TIMEOUT-BR
    TimeoutBr                                                              = 3197,
    /// TIMEOUT-BS
    TimeoutBs                                                              = 5675,
    /// TIMEOUT-CHECK
    TimeoutCheck                                                           = 685,
    /// TIMEOUT-CR
    TimeoutCr                                                              = 410,
    /// TIMEOUT-CS
    TimeoutCs                                                              = 377,
    /// TIMEOUT-SUBSTITUTION-VALUE
    TimeoutSubstitutionValue                                               = 1858,
    /// TIMESTAMP-FORMAT
    TimestampFormat                                                        = 3608,
    /// TIMESTAMP-RX-ACCEPTANCE-WINDOW
    TimestampRxAcceptanceWindow                                            = 280,
    /// TIMESTAMP-SUPPORT
    TimestampSupport                                                       = 5482,
    /// TIMING-ARGUMENT-REF
    TimingArgumentRef                                                      = 649,
    /// TIMING-ARGUMENTS
    TimingArguments                                                        = 5819,
    /// TIMING-CLOCK-SYNC-ACCURACY
    TimingClockSyncAccuracy                                                = 3866,
    /// TIMING-CLOCK-SYNC-ACCURACYS
    TimingClockSyncAccuracys                                               = 1631,
    /// TIMING-CLOCKS
    TimingClocks                                                           = 5033,
    /// TIMING-CONDITION
    TimingCondition                                                        = 5923,
    /// TIMING-CONDITION-FORMULA
    TimingConditionFormula                                                 = 1056,
    /// TIMING-CONDITION-REF
    TimingConditionRef                                                     = 691,
    /// TIMING-CONDITIONS
    TimingConditions                                                       = 2984,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 4765,
    /// TIMING-DESCRIPTION-REF
    TimingDescriptionRef                                                   = 600,
    /// TIMING-DESCRIPTIONS
    TimingDescriptions                                                     = 1907,
    /// TIMING-EVENT
    TimingEvent                                                            = 1593,
    /// TIMING-EVENT-REF
    TimingEventRef                                                         = 3006,
    /// TIMING-GUARANTEES
    TimingGuarantees                                                       = 5469,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 4302,
    /// TIMING-MODE-REF
    TimingModeRef                                                          = 5022,
    /// TIMING-MODES
    TimingModes                                                            = 1151,
    /// TIMING-PROPS
    TimingProps                                                            = 898,
    /// TIMING-REQUIREMENTS
    TimingRequirements                                                     = 2655,
    /// TIMING-RESOURCE
    TimingResource                                                         = 2081,
    /// TIMING-VARIABLE-REF
    TimingVariableRef                                                      = 967,
    /// TIMING-VARIABLES
    TimingVariables                                                        = 3897,
    /// TLS-CIPHER-SUITE
    TlsCipherSuite                                                         = 4175,
    /// TLS-CIPHER-SUITES
    TlsCipherSuites                                                        = 5194,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 2877,
    /// TLS-CONNECTION-REF
    TlsConnectionRef                                                       = 3122,
    /// TLS-CONNECTION-REFS
    TlsConnectionRefs                                                      = 4842,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 2656,
    /// TLS-CRYPTO-MAPPING-REF
    TlsCryptoMappingRef                                                    = 144,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 388,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 5463,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 4617,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 378,
    /// TLS-JOB-MAPPINGS
    TlsJobMappings                                                         = 4239,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 114,
    /// TLS-JOB-REQUIREMENT-REF
    TlsJobRequirementRef                                                   = 1268,
    /// TLS-JOB-SEMANTIC
    TlsJobSemantic                                                         = 5006,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 692,
    /// TLS-SECURE-COM-PROPS-REF
    TlsSecureComPropsRef                                                   = 1395,
    /// TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF
    TlvAppRecordElementInMethodIref                                        = 3914,
    /// TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    TlvAppRecordElementInServiceInterfaceIref                              = 3932,
    /// TLV-ARGUMENT-REF
    TlvArgumentRef                                                         = 2012,
    /// TLV-DATA-ID
    TlvDataId                                                              = 3829,
    /// TLV-DATA-ID-0-REF
    TlvDataId0Ref                                                          = 5467,
    /// TLV-DATA-ID-0-REFS
    TlvDataId0Refs                                                         = 2146,
    /// TLV-DATA-ID-DEFINITION
    TlvDataIdDefinition                                                    = 4391,
    /// TLV-DATA-ID-DEFINITION-REF
    TlvDataIdDefinitionRef                                                 = 5822,
    /// TLV-DATA-ID-DEFINITION-REFS
    TlvDataIdDefinitionRefs                                                = 5934,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 5659,
    /// TLV-DATA-ID-DEFINITIONS
    TlvDataIdDefinitions                                                   = 4572,
    /// TLV-DATA-IDS
    TlvDataIds                                                             = 573,
    /// TLV-IMPL-RECORD-ELEMENT
    TlvImplRecordElement                                                   = 2865,
    /// TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TlvImplementationDataTypeElementRef                                    = 4602,
    /// TLV-RECORD-ELEMENT-REF
    TlvRecordElementRef                                                    = 919,
    /// TLV-SUB-ELEMENT-REF
    TlvSubElementRef                                                       = 1226,
    /// TO-REF
    ToRef                                                                  = 5535,
    /// TOL
    Tol                                                                    = 722,
    /// TOLERANCE
    Tolerance                                                              = 1820,
    /// TOLERATED-FAILED-CYCLES
    ToleratedFailedCycles                                                  = 4059,
    /// TOOL
    Tool                                                                   = 431,
    /// TOOL-VERSION
    ToolVersion                                                            = 3352,
    /// TOPIC-1
    Topic1                                                                 = 2342,
    /// TOPIC-NAME
    TopicName                                                              = 3749,
    /// TP-ADDRESS
    TpAddress                                                              = 4450,
    /// TP-ADDRESS-EXTENSION-VALUE
    TpAddressExtensionValue                                                = 1467,
    /// TP-ADDRESS-REF
    TpAddressRef                                                           = 2934,
    /// TP-ADDRESS-REFS
    TpAddressRefs                                                          = 4643,
    /// TP-ADDRESSS
    TpAddresss                                                             = 1265,
    /// TP-CHANNEL-REF
    TpChannelRef                                                           = 4875,
    /// TP-CHANNELS
    TpChannels                                                             = 423,
    /// TP-CONFIGURATION
    TpConfiguration                                                        = 2100,
    /// TP-CONNECTION-CONTROL-REF
    TpConnectionControlRef                                                 = 4879,
    /// TP-CONNECTION-CONTROLS
    TpConnectionControls                                                   = 2642,
    /// TP-CONNECTIONS
    TpConnections                                                          = 3381,
    /// TP-ECU
    TpEcu                                                                  = 1946,
    /// TP-ECUS
    TpEcus                                                                 = 2537,
    /// TP-NODES
    TpNodes                                                                = 1411,
    /// TP-PGS
    TpPgs                                                                  = 4434,
    /// TP-SDU-REF
    TpSduRef                                                               = 5451,
    /// TP-SDU-REFS
    TpSduRefs                                                              = 5713,
    /// TP-TECHNOLOGY
    TpTechnology                                                           = 876,
    /// TRACE
    Trace                                                                  = 3227,
    /// TRACE-REF
    TraceRef                                                               = 4678,
    /// TRACE-REFS
    TraceRefs                                                              = 496,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 3005,
    /// TRACED-FAILURES
    TracedFailures                                                         = 4656,
    /// TRAFFIC-CLASS
    TrafficClass                                                           = 2611,
    /// TRAFFIC-LIMITATION-FILTERS
    TrafficLimitationFilters                                               = 4795,
    /// TRANCEIVER-STANDBY-DELAY
    TranceiverStandbyDelay                                                 = 2826,
    /// TRANSFER-EXIT-CLASS-REF
    TransferExitClassRef                                                   = 2813,
    /// TRANSFER-PROPERTY
    TransferProperty                                                       = 3696,
    /// TRANSFER-REF
    TransferRef                                                            = 5513,
    /// TRANSFER-REFS
    TransferRefs                                                           = 2444,
    /// TRANSFERS
    Transfers                                                              = 3848,
    /// TRANSFORMATION-COM-SPEC-PROPSS
    TransformationComSpecPropss                                            = 2117,
    /// TRANSFORMATION-DESCRIPTIONS
    TransformationDescriptions                                             = 3660,
    /// TRANSFORMATION-I-SIGNAL-PROPSS
    TransformationISignalPropss                                            = 4886,
    /// TRANSFORMATION-PROPS-MAPPING-SET-REF
    TransformationPropsMappingSetRef                                       = 3514,
    /// TRANSFORMATION-PROPS-REF
    TransformationPropsRef                                                 = 3704,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 4834,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 5481,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 1287,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING
    TransformationPropsToServiceInterfaceMapping                           = 4938,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 2332,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF
    TransformationPropsToServiceInterfaceMappingSetRef                     = 251,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS
    TransformationPropsToServiceInterfaceMappings                          = 296,
    /// TRANSFORMATION-PROPSS
    TransformationPropss                                                   = 1619,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 5742,
    /// TRANSFORMATION-TECHNOLOGYS
    TransformationTechnologys                                              = 4629,
    /// TRANSFORMER-CHAIN-REF
    TransformerChainRef                                                    = 3627,
    /// TRANSFORMER-CHAIN-REFS
    TransformerChainRefs                                                   = 3776,
    /// TRANSFORMER-CLASS
    TransformerClass                                                       = 762,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 5418,
    /// TRANSFORMER-REF
    TransformerRef                                                         = 5820,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 5054,
    /// TRANSFORMING-SYSTEM-SIGNAL-REF
    TransformingSystemSignalRef                                            = 1535,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 1290,
    /// TRANSIT-TO-INVALID-EXTENDED
    TransitToInvalidExtended                                               = 4211,
    /// TRANSITION-REF
    TransitionRef                                                          = 1112,
    /// TRANSITION-REFS
    TransitionRefs                                                         = 338,
    /// TRANSITIONS
    Transitions                                                            = 290,
    /// TRANSLATION-TARGET-IREF
    TranslationTargetIref                                                  = 2248,
    /// TRANSMISSION-ACKNOWLEDGE
    TransmissionAcknowledge                                                = 3237,
    /// TRANSMISSION-DEADLINE
    TransmissionDeadline                                                   = 487,
    /// TRANSMISSION-MODE
    TransmissionMode                                                       = 5629,
    /// TRANSMISSION-MODE-CONDITION
    TransmissionModeCondition                                              = 1262,
    /// TRANSMISSION-MODE-CONDITIONS
    TransmissionModeConditions                                             = 5815,
    /// TRANSMISSION-MODE-DECLARATION
    TransmissionModeDeclaration                                            = 4323,
    /// TRANSMISSION-MODE-FALSE-TIMING
    TransmissionModeFalseTiming                                            = 1102,
    /// TRANSMISSION-MODE-TRUE-TIMING
    TransmissionModeTrueTiming                                             = 3456,
    /// TRANSMISSION-PROPS
    TransmissionProps                                                      = 5292,
    /// TRANSMISSION-START-SEQUENCE-DURATION
    TransmissionStartSequenceDuration                                      = 2347,
    /// TRANSMISSION-TRIGGER
    TransmissionTrigger                                                    = 2092,
    /// TRANSMIT-CANCELLATION
    TransmitCancellation                                                   = 1554,
    /// TRANSMIT-PDU-REF
    TransmitPduRef                                                         = 2694,
    /// TRANSMIT-PDU-REFS
    TransmitPduRefs                                                        = 5464,
    /// TRANSMITTER-REF
    TransmitterRef                                                         = 3938,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 3954,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 1143,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF
    TransportLayerIndependentInstanceIdRef                                 = 5235,
    /// TRANSPORT-LAYER-RULE
    TransportLayerRule                                                     = 2008,
    /// TRANSPORT-PDU-REF
    TransportPduRef                                                        = 1638,
    /// TRANSPORT-PLUGIN
    TransportPlugin                                                        = 933,
    /// TRANSPORT-PLUGINS
    TransportPlugins                                                       = 4672,
    /// TRANSPORT-PROTOCOL
    TransportProtocol                                                      = 4874,
    /// TRANSPORT-PROTOCOLS
    TransportProtocols                                                     = 3373,
    /// TRCV-DELAY-COMPENSATION-OFFSET
    TrcvDelayCompensationOffset                                            = 1261,
    /// TRCV-PWM-MODE-ENABLED
    TrcvPwmModeEnabled                                                     = 4719,
    /// TRIGGER
    Trigger                                                                = 1878,
    /// TRIGGER-DIRECT-IMPLEMENTATIONS
    TriggerDirectImplementations                                           = 461,
    /// TRIGGER-I-PDU-SEND-CONDITION
    TriggerIPduSendCondition                                               = 2819,
    /// TRIGGER-I-PDU-SEND-CONDITIONS
    TriggerIPduSendConditions                                              = 2882,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 369,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 1104,
    /// TRIGGER-IREF
    TriggerIref                                                            = 3895,
    /// TRIGGER-MAPPING
    TriggerMapping                                                         = 3061,
    /// TRIGGER-MAPPINGS
    TriggerMappings                                                        = 1747,
    /// TRIGGER-MODE
    TriggerMode                                                            = 4487,
    /// TRIGGER-PERIOD
    TriggerPeriod                                                          = 5838,
    /// TRIGGER-PORT-ANNOTATION
    TriggerPortAnnotation                                                  = 1894,
    /// TRIGGER-PORT-ANNOTATIONS
    TriggerPortAnnotations                                                 = 5483,
    /// TRIGGER-REF
    TriggerRef                                                             = 3164,
    /// TRIGGER-REF-CONDITIONAL
    TriggerRefConditional                                                  = 5295,
    /// TRIGGER-REFS
    TriggerRefs                                                            = 3013,
    /// TRIGGER-TO-SIGNAL-MAPPING
    TriggerToSignalMapping                                                 = 1968,
    /// TRIGGERING-EVENT-REF
    TriggeringEventRef                                                     = 3410,
    /// TRIGGERS
    Triggers                                                               = 3987,
    /// TRIGGERS-RECOVERY-NOTIFICATION
    TriggersRecoveryNotification                                           = 1964,
    /// TROUBLE-CODE-J-1939-REF
    TroubleCodeJ1939Ref                                                    = 855,
    /// TROUBLE-CODE-OBD-REF
    TroubleCodeObdRef                                                      = 4946,
    /// TROUBLE-CODE-UDS-REF
    TroubleCodeUdsRef                                                      = 773,
    /// TRUE-ACTION-LIST-REF
    TrueActionListRef                                                      = 2737,
    /// TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR
    TrustedPlatformExecutableLaunchBehavior                                = 1741,
    /// TT
    Tt                                                                     = 775,
    /// TT-CAN-CLUSTER-REF
    TtCanClusterRef                                                        = 2794,
    /// TTCAN-ABSOLUTELY-SCHEDULED-TIMING
    TtcanAbsolutelyScheduledTiming                                         = 4588,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 3321,
    /// TTCAN-CLUSTER-CONDITIONAL
    TtcanClusterConditional                                                = 318,
    /// TTCAN-CLUSTER-VARIANTS
    TtcanClusterVariants                                                   = 3057,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 593,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 135,
    /// TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    TtcanCommunicationControllerConditional                                = 2202,
    /// TTCAN-COMMUNICATION-CONTROLLER-VARIANTS
    TtcanCommunicationControllerVariants                                   = 109,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 652,
    /// TTL
    Ttl                                                                    = 1014,
    /// TTL-MAX
    TtlMax                                                                 = 4785,
    /// TTL-MIN
    TtlMin                                                                 = 134,
    /// TWO-KEY-SLOT-MODE
    TwoKeySlotMode                                                         = 4843,
    /// TX-BIT-RATE-SWITCH
    TxBitRateSwitch                                                        = 519,
    /// TX-ENABLE-WINDOW-LENGTH
    TxEnableWindowLength                                                   = 2577,
    /// TX-MASK
    TxMask                                                                 = 3118,
    /// TX-NM-PDU-REF
    TxNmPduRef                                                             = 1377,
    /// TX-NM-PDU-REFS
    TxNmPduRefs                                                            = 5561,
    /// TX-PDU-POOL-REF
    TxPduPoolRef                                                           = 352,
    /// TX-PDU-TRIGGERING-REF
    TxPduTriggeringRef                                                     = 562,
    /// TYP
    Typ                                                                    = 1210,
    /// TYPE
    Type                                                                   = 366,
    /// TYPE-APPROVAL
    TypeApproval                                                           = 3145,
    /// TYPE-BLUEPRINTS
    TypeBlueprints                                                         = 1781,
    /// TYPE-DEFINITION
    TypeDefinition                                                         = 3341,
    /// TYPE-EMITTER
    TypeEmitter                                                            = 2265,
    /// TYPE-MAPPING
    TypeMapping                                                            = 1302,
    /// TYPE-OF-DTC-SUPPORTED
    TypeOfDtcSupported                                                     = 3663,
    /// TYPE-OF-EVENT-COMBINATION-SUPPORTED
    TypeOfEventCombinationSupported                                        = 3274,
    /// TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION
    TypeOfFreezeFrameRecordNumeration                                      = 4079,
    /// TYPE-REFERENCE
    TypeReference                                                          = 3831,
    /// TYPE-REFERENCE-REF
    TypeReferenceRef                                                       = 2053,
    /// TYPE-TAILORINGS
    TypeTailorings                                                         = 901,
    /// TYPE-TREF
    TypeTref                                                               = 321,
    /// UAS-ID
    UasId                                                                  = 4264,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 4403,
    /// UCM-MASTER-FALLBACK-REF
    UcmMasterFallbackRef                                                   = 161,
    /// UCM-MASTER-FALLBACK-REFS
    UcmMasterFallbackRefs                                                  = 5640,
    /// UCM-MASTER-MODULE-INSTANTIATION
    UcmMasterModuleInstantiation                                           = 2243,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 5196,
    /// UCM-MODULE-INSTANTIATION-REF
    UcmModuleInstantiationRef                                              = 4335,
    /// UCM-PROCESSINGS
    UcmProcessings                                                         = 4366,
    /// UCM-REF
    UcmRef                                                                 = 5072,
    /// UCM-SOFTWARE-VERSION
    UcmSoftwareVersion                                                     = 4981,
    /// UCM-STEP
    UcmStep                                                                = 2486,
    /// UCM-SUBORDINATE-MODULE-INSTANTIATION
    UcmSubordinateModuleInstantiation                                      = 1087,
    /// UCM-TO-TIME-BASE-RESOURCE-MAPPING
    UcmToTimeBaseResourceMapping                                           = 4573,
    /// UCMS
    Ucms                                                                   = 1257,
    /// UDP-CHECKSUM-HANDLING
    UdpChecksumHandling                                                    = 3551,
    /// UDP-COLLECTION-BUFFER-SIZE-THRESHOLD
    UdpCollectionBufferSizeThreshold                                       = 2945,
    /// UDP-COLLECTION-BUFFER-TIMEOUT
    UdpCollectionBufferTimeout                                             = 5272,
    /// UDP-COLLECTION-TRIGGER
    UdpCollectionTrigger                                                   = 5658,
    /// UDP-LOCAL-PORT
    UdpLocalPort                                                           = 3884,
    /// UDP-MIN-TX-BUFFER-SIZE
    UdpMinTxBufferSize                                                     = 1207,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 1812,
    /// UDP-NM-CLUSTER-COUPLING
    UdpNmClusterCoupling                                                   = 1768,
    /// UDP-NM-CLUSTER-REF
    UdpNmClusterRef                                                        = 5812,
    /// UDP-NM-ECU
    UdpNmEcu                                                               = 1418,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 108,
    /// UDP-PORT
    UdpPort                                                                = 5947,
    /// UDP-PORT-REF
    UdpPortRef                                                             = 5244,
    /// UDP-PROPS
    UdpProps                                                               = 3638,
    /// UDP-REMOTE-PORT
    UdpRemotePort                                                          = 5287,
    /// UDP-RULE
    UdpRule                                                                = 228,
    /// UDP-TP
    UdpTp                                                                  = 831,
    /// UDP-TP-PORT
    UdpTpPort                                                              = 4575,
    /// UDP-TTL
    UdpTtl                                                                 = 4864,
    /// UDS-DTC-NUMBER
    UdsDtcNumber                                                           = 4408,
    /// UDS-DTC-VALUE
    UdsDtcValue                                                            = 2751,
    /// UDS-SERVICE
    UdsService                                                             = 727,
    /// UNASSIGN-FRAME-ID
    UnassignFrameId                                                        = 736,
    /// UNASSIGNED-FRAME-TRIGGERING-REF
    UnassignedFrameTriggeringRef                                           = 5395,
    /// UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE
    UncompressedSoftwareClusterSize                                        = 5931,
    /// UNICAST-CREDENTIALS
    UnicastCredentials                                                     = 4111,
    /// UNICAST-NETWORK-ENDPOINT-REF
    UnicastNetworkEndpointRef                                              = 955,
    /// UNICAST-NETWORK-ENDPOINT-REFS
    UnicastNetworkEndpointRefs                                             = 2091,
    /// UNICAST-SECURE-COM-PROPS-REF
    UnicastSecureComPropsRef                                               = 2809,
    /// UNICAST-SECURE-COM-PROPS-REFS
    UnicastSecureComPropsRefs                                              = 960,
    /// UNICAST-UDP-CREDENTIALS
    UnicastUdpCredentials                                                  = 3648,
    /// UNIT
    Unit                                                                   = 3110,
    /// UNIT-AND-SCALING-ID
    UnitAndScalingId                                                       = 635,
    /// UNIT-DISPLAY-NAME
    UnitDisplayName                                                        = 1676,
    /// UNIT-GROUP
    UnitGroup                                                              = 298,
    /// UNIT-GROUP-REF
    UnitGroupRef                                                           = 3880,
    /// UNIT-GROUP-REFS
    UnitGroupRefs                                                          = 1612,
    /// UNIT-REF
    UnitRef                                                                = 1770,
    /// UNIT-REFS
    UnitRefs                                                               = 1777,
    /// UNLIMITED-INTEGER-VALUE-VARIATION-POINT
    UnlimitedIntegerValueVariationPoint                                    = 3439,
    /// UNRESOLVED-REFERENCE-RESTRICTION
    UnresolvedReferenceRestriction                                         = 5453,
    /// UNUSED-BIT-PATTERN
    UnusedBitPattern                                                       = 3750,
    /// UPDATE
    Update                                                                 = 3029,
    /// UPDATE-INDICATION-BIT-POSITION
    UpdateIndicationBitPosition                                            = 4115,
    /// UPDATE-KIND
    UpdateKind                                                             = 5134,
    /// UPDATE-SESSION-REJECTED
    UpdateSessionRejected                                                  = 5062,
    /// UPDATE-STRATEGY
    UpdateStrategy                                                         = 3108,
    /// UPPER-BOUND
    UpperBound                                                             = 4250,
    /// UPPER-CAN-ID
    UpperCanId                                                             = 1122,
    /// UPPER-HEADER-BITS-TO-SHIFT
    UpperHeaderBitsToShift                                                 = 1714,
    /// UPPER-LIMIT
    UpperLimit                                                             = 1424,
    /// UPPER-MULTIPLICITY
    UpperMultiplicity                                                      = 241,
    /// UPPER-MULTIPLICITY-INFINITE
    UpperMultiplicityInfinite                                              = 3264,
    /// UPPER-REF
    UpperRef                                                               = 4047,
    /// UPSTREAM-REFERENCE-IREF
    UpstreamReferenceIref                                                  = 4978,
    /// URI
    Uri                                                                    = 740,
    /// URL
    Url                                                                    = 1931,
    /// USAGE
    Usage                                                                  = 450,
    /// USAGES
    Usages                                                                 = 1624,
    /// USE-AS-CRYPTOGRAPHIC-I-PDU
    UseAsCryptographicIPdu                                                 = 1347,
    /// USE-AUTH-DATA-FRESHNESS
    UseAuthDataFreshness                                                   = 337,
    /// USE-AUTO-VALIDATION-AT-SHUT-DOWN
    UseAutoValidationAtShutDown                                            = 1047,
    /// USE-CASE
    UseCase                                                                = 1089,
    /// USE-CLIENT-AUTHENTICATION-REQUEST
    UseClientAuthenticationRequest                                         = 2870,
    /// USE-CRC-COMP-MECHANISM
    UseCrcCompMechanism                                                    = 5550,
    /// USE-FRESHNESS-TIMESTAMP
    UseFreshnessTimestamp                                                  = 2116,
    /// USE-INSTEAD-REF
    UseInsteadRef                                                          = 4053,
    /// USE-INSTEAD-REFS
    UseInsteadRefs                                                         = 5358,
    /// USE-MAC-ADDRESS-FOR-IDENTIFICATION
    UseMacAddressForIdentification                                         = 1023,
    /// USE-SECURED-PDU-HEADER
    UseSecuredPduHeader                                                    = 5685,
    /// USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT
    UseSecurityExtensionRecordSizeLimit                                    = 1766,
    /// USE-SMART-SENSOR-API
    UseSmartSensorApi                                                      = 2154,
    /// USE-VEHICLE-IDENTIFICATION-SYNC-STATUS
    UseVehicleIdentificationSyncStatus                                     = 763,
    /// USED-CODE-GENERATOR
    UsedCodeGenerator                                                      = 1785,
    /// USED-DATA-ELEMENT
    UsedDataElement                                                        = 5110,
    /// USED-FID-REF
    UsedFidRef                                                             = 1540,
    /// USED-IMPLEMENTATION-DATA-TYPE-REF
    UsedImplementationDataTypeRef                                          = 2859,
    /// USED-LANGUAGES
    UsedLanguages                                                          = 3402,
    /// USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF
    UsedLifeCycleStateDefinitionGroupRef                                   = 5924,
    /// USED-PARAMETER-ELEMENT
    UsedParameterElement                                                   = 3428,
    /// USED-PIM-REF
    UsedPimRef                                                             = 5349,
    /// USED-SECONDARY-FID-REF
    UsedSecondaryFidRef                                                    = 242,
    /// USED-SECONDARY-FID-REFS
    UsedSecondaryFidRefs                                                   = 5324,
    /// USER-DATA-SUB-TLV
    UserDataSubTlv                                                         = 3153,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 1370,
    /// USER-DEFINED-CLUSTER-CONDITIONAL
    UserDefinedClusterConditional                                          = 2591,
    /// USER-DEFINED-CLUSTER-VARIANTS
    UserDefinedClusterVariants                                             = 3460,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 5118,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 2647,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL
    UserDefinedCommunicationControllerConditional                          = 5388,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS
    UserDefinedCommunicationControllerVariants                             = 1022,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 3234,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 1114,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 365,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 1557,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 1597,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 5745,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 5722,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 5128,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 2530,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 4958,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 4227,
    /// USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS
    UserDefinedTransformationComSpecProps                                  = 3738,
    /// USER-DEFINED-TRANSFORMATION-DESCRIPTION
    UserDefinedTransformationDescription                                   = 2360,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS
    UserDefinedTransformationISignalProps                                  = 5757,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    UserDefinedTransformationISignalPropsConditional                       = 1666,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    UserDefinedTransformationISignalPropsVariants                          = 1772,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 777,
    /// USES-ADDRESS-ARBITRATION
    UsesAddressArbitration                                                 = 544,
    /// USES-END-TO-END-PROTECTION
    UsesEndToEndProtection                                                 = 3714,
    /// USES-MONITOR-DATA
    UsesMonitorData                                                        = 561,
    /// USES-TIME-BASE-RESOURCE
    UsesTimeBaseResource                                                   = 4437,
    /// V
    V                                                                      = 4397,
    /// V-2-X-DATA-MANAGER-NEEDS
    V2XDataManagerNeeds                                                    = 315,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 3222,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 1967,
    /// V-2-X-SUPPORTED
    V2XSupported                                                           = 1840,
    /// V-LAN-REF
    VLanRef                                                                = 3690,
    /// V-LAN-REFS
    VLanRefs                                                               = 1754,
    /// VALID-BINDING-TIME
    ValidBindingTime                                                       = 5067,
    /// VALID-BINDING-TIMES
    ValidBindingTimes                                                      = 4468,
    /// VALIDATION-CONFIGURATION
    ValidationConfiguration                                                = 5083,
    /// VALIDATION-FORMULA
    ValidationFormula                                                      = 2695,
    /// VALIDATION-ROOT
    ValidationRoot                                                         = 4445,
    /// VALUE
    Value                                                                  = 283,
    /// VALUE-ACCESS
    ValueAccess                                                            = 1413,
    /// VALUE-AXIS-DATA-TYPE-REF
    ValueAxisDataTypeRef                                                   = 4414,
    /// VALUE-CONFIG-CLASSES
    ValueConfigClasses                                                     = 4092,
    /// VALUE-DATA-TYPE-REF
    ValueDataTypeRef                                                       = 4375,
    /// VALUE-IREF
    ValueIref                                                              = 4566,
    /// VALUE-PAIRS
    ValuePairs                                                             = 463,
    /// VALUE-REF
    ValueRef                                                               = 281,
    /// VALUE-RESTRICTION
    ValueRestriction                                                       = 694,
    /// VALUE-SPEC
    ValueSpec                                                              = 4559,
    /// VALUE-TYPE-REF
    ValueTypeRef                                                           = 334,
    /// VALUE-TYPE-TREF
    ValueTypeTref                                                          = 5798,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 4243,
    /// VARIABLE-ACCESS-IREF
    VariableAccessIref                                                     = 3316,
    /// VARIABLE-ACCESS-IREFS
    VariableAccessIrefs                                                    = 2462,
    /// VARIABLE-ACCESS-REF
    VariableAccessRef                                                      = 3963,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REF
    VariableAccessedForDebugRef                                            = 3854,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REFS
    VariableAccessedForDebugRefs                                           = 2741,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 3217,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 4440,
    /// VARIABLE-DATA-PROTOTYPE-IREF
    VariableDataPrototypeIref                                              = 4482,
    /// VARIABLE-INSTANCE
    VariableInstance                                                       = 5148,
    /// VARIABLE-INSTANCE-IREF
    VariableInstanceIref                                                   = 4126,
    /// VARIABLE-LENGTH
    VariableLength                                                         = 4279,
    /// VARIABLE-REF
    VariableRef                                                            = 53,
    /// VARIABLES
    Variables                                                              = 3623,
    /// VARIANT-CRITERION-REF
    VariantCriterionRef                                                    = 4821,
    /// VARIANT-ID
    VariantId                                                              = 1127,
    /// VARIATION
    Variation                                                              = 3942,
    /// VARIATION-POINT
    VariationPoint                                                         = 5466,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 5407,
    /// VARIATION-POINT-PROXYS
    VariationPointProxys                                                   = 188,
    /// VARIATION-RESTRICTION
    VariationRestriction                                                   = 535,
    /// VCID
    Vcid                                                                   = 3369,
    /// VEHICLE-ANNOUNCEMENT-COUNT
    VehicleAnnouncementCount                                               = 5180,
    /// VEHICLE-ANNOUNCEMENT-INTERVAL
    VehicleAnnouncementInterval                                            = 2160,
    /// VEHICLE-DESCRIPTION-REF
    VehicleDescriptionRef                                                  = 3869,
    /// VEHICLE-DRIVER-NOTIFICATION
    VehicleDriverNotification                                              = 5290,
    /// VEHICLE-IDENTIFICATION-SYNC-STATUS
    VehicleIdentificationSyncStatus                                        = 2861,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 3027,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 3126,
    /// VEHICLE-SYSTEM
    VehicleSystem                                                          = 3867,
    /// VEHICLE-SYSTEM-INSTANCE
    VehicleSystemInstance                                                  = 3800,
    /// VENDOR
    Vendor                                                                 = 3429,
    /// VENDOR-API-INFIX
    VendorApiInfix                                                         = 508,
    /// VENDOR-ID
    VendorId                                                               = 2239,
    /// VENDOR-SIGNATURE-REF
    VendorSignatureRef                                                     = 4415,
    /// VENDOR-SPECIFIC-MODULE-DEF-REF
    VendorSpecificModuleDefRef                                             = 3021,
    /// VENDOR-SPECIFIC-MODULE-DEF-REFS
    VendorSpecificModuleDefRefs                                            = 3346,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 852,
    /// VERBATIM
    Verbatim                                                               = 4237,
    /// VERIFICATION-HASH
    VerificationHash                                                       = 5096,
    /// VERIFICATION-METHOD
    VerificationMethod                                                     = 2125,
    /// VERIFICATION-STATUS-INDICATION-MODE
    VerificationStatusIndicationMode                                       = 3581,
    /// VERIFY-IREF
    VerifyIref                                                             = 3310,
    /// VERIFY-IREFS
    VerifyIrefs                                                            = 2228,
    /// VERIFY-REF
    VerifyRef                                                              = 2316,
    /// VERIFY-REFS
    VerifyRefs                                                             = 3924,
    /// VERIFY-UPDATE
    VerifyUpdate                                                           = 3915,
    /// VERSION
    Version                                                                = 5935,
    /// VERSION-DRIVEN-FIND-BEHAVIOR
    VersionDrivenFindBehavior                                              = 4284,
    /// VF
    Vf                                                                     = 2595,
    /// VFB-TIMING
    VfbTiming                                                              = 4318,
    /// VFC-IREF
    VfcIref                                                                = 5782,
    /// VFC-IREFS
    VfcIrefs                                                               = 5286,
    /// VG
    Vg                                                                     = 4730,
    /// VIEW-MAP
    ViewMap                                                                = 3679,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 1222,
    /// VIEW-MAPS
    ViewMaps                                                               = 5874,
    /// VIN-INVALIDITY-PATTERN
    VinInvalidityPattern                                                   = 206,
    /// VLAN
    Vlan                                                                   = 3574,
    /// VLAN-ID
    VlanId                                                                 = 277,
    /// VLAN-IDENTIFIER
    VlanIdentifier                                                         = 5876,
    /// VLAN-MEMBERSHIP
    VlanMembership                                                         = 1447,
    /// VLAN-MEMBERSHIPS
    VlanMemberships                                                        = 1642,
    /// VLAN-MODIFIER-REF
    VlanModifierRef                                                        = 3280,
    /// VLAN-PRIORITY
    VlanPriority                                                           = 5478,
    /// VLAN-REF
    VlanRef                                                                = 4700,
    /// VLAN-REFS
    VlanRefs                                                               = 5086,
    /// VT
    Vt                                                                     = 2773,
    /// VTF
    Vtf                                                                    = 5619,
    /// WAIT-POINT
    WaitPoint                                                              = 2256,
    /// WAIT-POINTS
    WaitPoints                                                             = 1880,
    /// WAKE-UP-BY-CONTROLLER-SUPPORTED
    WakeUpByControllerSupported                                            = 789,
    /// WAKE-UP-CHANNEL
    WakeUpChannel                                                          = 823,
    /// WAKE-UP-OVER-BUS-SUPPORTED
    WakeUpOverBusSupported                                                 = 1313,
    /// WAKE-UP-PATTERN
    WakeUpPattern                                                          = 4346,
    /// WAKEUP-FORWARD-LOCAL-ENABLED
    WakeupForwardLocalEnabled                                              = 1325,
    /// WAKEUP-FORWARD-REMOTE-ENABLED
    WakeupForwardRemoteEnabled                                             = 5858,
    /// WAKEUP-FRAME-REF
    WakeupFrameRef                                                         = 4909,
    /// WAKEUP-FRAME-REFS
    WakeupFrameRefs                                                        = 4345,
    /// WAKEUP-LOCAL-DETECTION-TIME
    WakeupLocalDetectionTime                                               = 1775,
    /// WAKEUP-LOCAL-DURATION-TIME
    WakeupLocalDurationTime                                                = 3232,
    /// WAKEUP-LOCAL-ENABLED
    WakeupLocalEnabled                                                     = 1637,
    /// WAKEUP-REMOTE-ENABLED
    WakeupRemoteEnabled                                                    = 1824,
    /// WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST
    WakeupRepetitionDelayOfWakeupRequest                                   = 1660,
    /// WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST
    WakeupRepetitionsOfWakeupRequest                                       = 5635,
    /// WAKEUP-RX-IDLE
    WakeupRxIdle                                                           = 1681,
    /// WAKEUP-RX-LOW
    WakeupRxLow                                                            = 4563,
    /// WAKEUP-RX-WINDOW
    WakeupRxWindow                                                         = 1973,
    /// WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF
    WakeupSleepOnDatalineConfigRef                                         = 5590,
    /// WAKEUP-SYMBOL-RX-WINDOW
    WakeupSymbolRxWindow                                                   = 5266,
    /// WAKEUP-TX-ACTIVE
    WakeupTxActive                                                         = 4204,
    /// WAKEUP-TX-IDLE
    WakeupTxIdle                                                           = 3020,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 2931,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 637,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 1713,
    /// WINDOW-SIZE
    WindowSize                                                             = 4442,
    /// WINDOW-SIZE-INIT
    WindowSizeInit                                                         = 5426,
    /// WINDOW-SIZE-INVALID
    WindowSizeInvalid                                                      = 4406,
    /// WINDOW-SIZE-VALID
    WindowSizeValid                                                        = 147,
    /// WITH-AUTO
    WithAuto                                                               = 1431,
    /// WORST-CASE-EXECUTION-TIME
    WorstCaseExecutionTime                                                 = 797,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 2867,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 3667,
    /// WRITE
    Write                                                                  = 1663,
    /// WRITE-ACCESS
    WriteAccess                                                            = 1949,
    /// WRITE-CLASS-REF
    WriteClassRef                                                          = 772,
    /// WRITE-ONLY-ONCE
    WriteOnlyOnce                                                          = 391,
    /// WRITE-VERIFICATION
    WriteVerification                                                      = 816,
    /// WRITER-ENTITY-ID
    WriterEntityId                                                         = 1933,
    /// WRITING-FREQUENCY
    WritingFrequency                                                       = 3936,
    /// WRITING-PRIORITY
    WritingPriority                                                        = 3003,
    /// WRITING-STRATEGY-ROLE
    WritingStrategyRole                                                    = 4682,
    /// WRITING-STRATEGYS
    WritingStrategys                                                       = 1386,
    /// WRITTEN-LOCAL-VARIABLES
    WrittenLocalVariables                                                  = 3644,
    /// WRITTEN-NV-DATA
    WrittenNvData                                                          = 1522,
    /// WRITTEN-READ-NV-DATA
    WrittenReadNvData                                                      = 854,
    /// WWH-OBD-DTC-CLASS
    WwhObdDtcClass                                                         = 5475,
    /// X
    X                                                                      = 2557,
    /// XCP-PDU
    XcpPdu                                                                 = 65,
    /// XDOC
    Xdoc                                                                   = 5321,
    /// XFILE
    Xfile                                                                  = 4176,
    /// XREF
    Xref                                                                   = 3436,
    /// XREF-TARGET
    XrefTarget                                                             = 374,
}

impl ElementName {
    const STRING_TABLE: [&'static str; 5962] = ["SOMEIP-METHOD-DEPLOYMENT", "DIAGNOSTIC-EVENT-PORT-MAPPING", "TIME-SEG-1", "BASE-TYPE-REF", "CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF", "SYSTEM-SIGNAL-GROUP", "TERMINATION-BEHAVIOR", "OBD-INFO-SERVICE-NEEDS", "TCP-IP-NDP-MIN-RANDOM-FACTOR", "TIME-FR-IF", "EXPIRED-SUPERVISION-CYCLE-TOL", "PREFIX-REF", "COMPOSITION-SW-COMPONENT-TYPE", "LOCAL-DEBUG-DATAS", "KEY-SLOTS", "CLIENT-SERVER-ARRAY-TYPE-MAPPING", "READONLY", "PRIMITIVE-ATTRIBUTE-CONDITION", "RESTRICTION", "NM-CBV-POSITION", "CRYPTO-CERTIFICATES", "TEST-RESULT-REF", "FORMAL-BLUEPRINT-CONDITION", "ICV-VERIFICATION", "IDENTITY-REF", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "LOG-TRACE-PORT-ID", "MESSAGE-TYPE", "SPEED", "R-PORT-IN-COMPOSITION-INSTANCE-REF", "DDS-SERVICE-VERSION", "DATA-IDENTIFIER-REF", "CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF", "IP-V-6-PATH-MTU-TIMEOUT", "ROLLOUT-QUALIFICATIONS", "CONTEXT-BSW-IMPLEMENTATION-REF", "I-SIGNAL-TO-I-PDU-MAPPING", "MAPPING-SCOPE", "TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE", "ACTIVE-SUPERVISION-REF", "TCP-ROLE", "PORT-PROTOTYPE-BLUEPRINT-MAPPING", "RTP-TP", "STATE-MACHINE", "CLIENT-ID-DEFINITION-SET", "TCP-IP-ICMP-V-6-HOP-LIMIT", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "NM-CAR-WAKE-UP-BIT-POSITION", "ARGUMENTS", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "ECUC-INSTANCE-REFERENCE-VALUE", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "SOMEIP-SERVICE-DISCOVERY-PORT", "VARIABLE-REF", "SW-RECORD-LAYOUT-V-AXIS", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "SUB-ELEMENT-MAPPING", "PRM", "FIRE-AND-FORGET-MAPPING", "BIT-POSITION", "SUBSTITUTED-FRAME-REF", "ETH-IP-PROPS", "SOCKET-CONNECTION-REFS", "COUPLING-PORT-REF-CONDITIONAL", "IP-ADDRESS-PREFIX-LENGTH", "XCP-PDU", "NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS", "DIAGNOSTIC-CONTRIBUTION-REFS", "SIGNATURE-SUPPORT-AP", "CRYPTO-KEY-SLOT-REFS", "SEC-OC-JOB-REQUIREMENT-REF", "SD-SERVER-CONFIG-REF", "CLEAR-CONDITION-REF", "PERIOD-BEGIN", "FIELD-MAPPINGS", "SEQUENTIAL-INSTRUCTIONS-END", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "IO-HW-ABSTRACTION-SERVER-ANNOTATION", "SYSTEM-SIGNAL", "CONTAINED-PROCESS-REFS", "PN-RESET-TIMER", "ALIAS-NAME-SET", "ESTIMATED-EXECUTION-TIME", "MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF", "RECORD-ELEMENT-REF", "SERVICE-TIMING", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "PURE-LOCAL-TIME-BASE", "POST-BUILD-VARIANT-CRITERION-VALUES", "AUDIENCE", "STARTUP-OPTION", "TCP-OPTION-FILTER-LISTS", "DATA-TYPE-MAPPING", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "HANDLE-OUT-OF-RANGE-STATUS", "MEMORY-CONSUMPTION", "SYNC-NODE-MAX", "SWC-TO-IMPL-MAPPING", "META-DATA-FIELDS", "CLIENT-SERVER-RECORD-ELEMENT-MAPPING", "CAN-NM-CLUSTER-COUPLING", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "MAX-AS", "COM-PROCESSING-PERIOD", "ECUC-QUERYS", "RESULTING-RPT-SW-PROTOTYPING-ACCESS", "FILE-TYPE", "TD-EVENT-SERVICE-INSTANCE-METHOD", "UDP-NM-NODE", "TTCAN-COMMUNICATION-CONTROLLER-VARIANTS", "SDGS", "RPT-SERVICE-POINT-PRE-REFS", "SW-GENERIC-AXIS-PARAM-TYPE", "POSSIBLE-AP-ERROR-SET-REFS", "TLS-JOB-REQUIREMENT", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "FILE-PROXYS", "TIME-BASE-JITTER", "RECOVERY-NOTIFICATION-REF", "SPN-REF", "BLUEPRINT-POLICYS", "GETTER-OPERATION-REF", "EXCLUSIVE-AREA-REF-CONDITIONAL", "LEAF-ELEMENT-IREF", "REQUEST-TOPIC-NAME", "HW-ATTRIBUTE-LITERAL-DEF", "INCLUDE-REFS", "TASK", "COMPRESSED-SOFTWARE-PACKAGE-SIZE", "PARAMETER-ACCESSED-FOR-DEBUG-REFS", "NUMBER-OF-BITS", "MAPPED-FUNCTION-REF", "SECURE-COM-CONFIGS", "INTERPOLATION-ROUTINE-MAPPING-SET-REF", "TTL-MIN", "TTCAN-COMMUNICATION-CONTROLLER", "MAX-NUMBER-OF-PARALLEL-TRANSFERS", "DIAGNOSTIC-SECURITY-LEVEL", "OUTGOING-CALLBACKS", "LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING", "CLASS-CONTENTS", "CRYPTO-ELLIPTIC-CURVE-PROPS", "ICMP-RULE", "ACTIVATION-REASONS", "TLS-CRYPTO-MAPPING-REF", "FLEXRAY-FIFO-RANGE", "MAX-NUMBER-OF-FILES", "WINDOW-SIZE-VALID", "START-INDEX", "MANDATORY", "NM-CONTROL-BIT-VECTOR-ENABLED", "FIRST-PHYSICAL-DIMENSION-REF", "CONFIDENTIALITY-OFFSET", "NTU", "CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING", "INTENDED-TARGET-MACHINE-REFS", "REQUIRED-EVENT-GROUPS", "PARTICIPATING-J-1939-CLUSTER-REFS", "RESOURCE-NEEDS-REF", "RUNNABLES", "SOURCE-ID", "UCM-MASTER-FALLBACK-REF", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "NONQUEUED-SENDER-COM-SPEC", "QUEUED-SENDER-COM-SPEC", "RAW-DATA-STREAM-GRANT-DESIGN", "SLEEP-REPETITIONS-OF-SLEEP-REQUEST", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "PORT-PROTOTYPE-PROPS", "BUFFER-COMPUTATION", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "SERVICE-ELEMENT-MAPPING-REFS", "IS-OBD-RELEVANT", "MAC-SEC-CIPHER-SUITE-CONFIG", "SW-MAINTENANCE-NOTES", "PROCESSORS", "METHOD-RETURN-REF", "CRYPTO-SERVICE-CERTIFICATE-REF", "CHECKPOINT-TRANSITION", "DATA-OFFSET", "CUSTOM-DOCUMENTATION-REF", "TD-HEADER-ID-FILTERS", "TEX-MATH", "SW-IS-VIRTUAL", "RECORD-VALUE-SPECIFICATION", "TD-EVENT-BSW-MODE-DECLARATION-TYPE", "COLLECTION-PROPS", "ECU-RESET-CLASS-REF", "VARIATION-POINT-PROXYS", "PHM-HEALTH-CHANNEL-STATUS", "MSR-QUERY-PROPS", "STATE-MANAGEMENT-STATE-NOTIFICATION", "STATE-REF", "DROP-NOT-REQUESTED-NAD", "MODE-SENDER-POLICYS", "I-SIGNAL-TYPE", "FLEXRAY-TP-CONFIG", "TCP-IP-ARP-REQUEST-TIMEOUT", "DIAGNOSTIC-MEMORY-IDENTIFIER", "LINKERS", "IPV-6-PROPS", "DESTINATION-REFS", "SEGMENT-LENGTH", "IMMEDIATE-NV-DATA-STORAGE", "ENABLE-TAKE-ADDRESS", "KEY-SIZE", "VIN-INVALIDITY-PATTERN", "DYNAMIC-BS", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF", "SPN", "MAPPED-DATA-ELEMENT-IREF", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE", "EVENT-KIND", "MAPPED-SECURITY-EVENTS", "PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT", "ACTIVATE-AT-START", "INCLUDED-VARIANT-REFS", "ECUC-INSTANCE-REFERENCE-DEF", "ALL-NM-MESSAGES-KEEP-AWAKE", "PNC-WAKEUP-ENABLE", "APPL-CONSTANT-REF", "RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS", "PNC-FILTER-DATA-MASK", "BLOCK-IF-STATE-ACTIVE-CP-REF", "STATIC-PART", "ACTION-TYPE", "DEBUG-INFOS", "UDP-RULE", "SIGNATURE-SCHEME-REFS", "CATEGORY", "FILE-NAME", "PORT-ARG-VALUES", "MODE-SWITCHED-ACK", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY", "PORT-DEFINED-ARGUMENT-VALUE", "PROCESSING-UNIT-REF", "DESTINATION-MIN-ADDRESS", "IUMPR-REF", "I-SIGNAL-REF", "ETHERNET-CLUSTER-REF", "UPPER-MULTIPLICITY", "USED-SECONDARY-FID-REF", "BSW-BACKGROUND-EVENT", "ACCESSED-VARIABLE", "CONTEXT-SW-COMPONENT-REF", "REFERENCE-CONDITION", "PORT-PROTOTYPE-BLUEPRINT", "IP-SEC-CONFIG-PROPS", "AUTOSAR", "CAN-XL-BAUDRATE", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF", "SWC-TO-APPLICATION-PARTITION-MAPPINGS", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "NM-MESSAGE-TIMEOUT-TIME", "MODE-CONDITION-REF", "RECEIVER-ANNOTATION", "THRESHOLD-SIZE", "CONDITION-ACCESS", "NV-DATA-PORT-ANNOTATIONS", "ASSOCIATED-COM-I-PDU-GROUP-REF", "RPT-READ-ACCESS", "PNC-WAKEUP-CAN-ID-MASK", "NM-INSTANTIATION", "COUNTER-OFFSET", "ECU-SCOPE-REFS", "INITIAL-DELAY-MIN-VALUE", "SOVD-GATEWAY-INSTANTIATION", "MAX-WITHOUT-CLOCK-CORRECTION-FATAL", "TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC", "RPT-CONTAINER", "GENERAL-PURPOSE-I-PDU", "COUNTER-JUMP-DOWN", "FLEXRAY-COMMUNICATION-CONNECTOR", "TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE", "ELEMENT-PROPSS", "AH-CIPHER-SUITE-NAME", "VLAN-ID", "MASK", "OPTIONAL-ELEMENTS", "TIMESTAMP-RX-ACCEPTANCE-WINDOW", "VALUE-REF", "CLASSIFICATION", "VALUE", "REPLAY-PROTECTION-WINDOW", "FIGURE", "SW-COMPONENT-PROTOTYPE", "SUP", "METHOD-TOPICS-ACCESS-RULE-REF", "BUCKET-SIZE", "TRANSITIONS", "PNC-PDUR-GROUP-REFS", "REQUIRED-SERVICE-VERSION", "RECEIVE-ACTIVITY", "CLEAR-CONDITION-GROUP-REF", "ARBITRARY-EVENT-TRIGGERING", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS", "SECURED-AREA-OFFSET", "UNIT-GROUP", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "SECOND-OPERATION-REF", "DYNAMICALLY-ASSIGNED", "IKE-REKEY-TIME", "KEY-VALUE-STORAGE-REF", "STATIC-MEMORYS", "PGN", "SW-DIAGNOSTICS-NOTES", "PID-REFS", "DIAGNOSTIC-FIM-ALIAS-EVENT", "KIND", "DIAGNOSTIC-COMMON-ELEMENT-REF", "IO-HW-ABSTRACTION-SERVER-ANNOTATIONS", "CLIENT-SERVER-OPERATION-REF", "MKA-PARTICIPANT-REF", "SERVER-COM-SPEC", "V-2-X-DATA-MANAGER-NEEDS", "BURST-SIZE-REQUEST", "STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE", "TTCAN-CLUSTER-CONDITIONAL", "CONTAINED-PDU-TRIGGERING-REF", "J-1939-CONTROLLER-APPLICATION", "TYPE-TREF", "TCP-IP-NDP-DEFENSIVE-PROCESSING", "J-1939-CLUSTER-VARIANTS", "DLT-ARGUMENTS", "LIN-SLAVE-CONDITIONAL", "MAXIMUM-EXECUTION-TIME", "ECU-COMM-PORT-INSTANCES", "TEST-ID-REF", "NV-REQUIRE-COM-SPEC", "SET", "SYNC-CONFIRMATION-TIMEOUT", "SW-BIT-REPRESENTATION", "ACL-ROLE-REFS", "VALUE-TYPE-REF", "MICRO-INITIAL-OFFSET-B", "CAN-CLUSTER", "USE-AUTH-DATA-FRESHNESS", "TRANSITION-REFS", "SUPPORTED-TIMER-GRANULARITY", "DOC-REVISION", "ARP-PROPS", "REMOTE-PORT-RANGE-START", "SOMEIP-REMOTE-UNICAST-CONFIG", "SERVICE-INSTANCES", "CONFLICTS", "TD-EVENT-VFB-REFERENCE", "SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "SEPARATION-TIME-REQUEST", "DIAGNOSTIC-IO-CONTROL-NEEDS", "MAPPED-OPERATION-IREF", "TX-PDU-POOL-REF", "SYSTEM-TIMING", "REST-OBJECT-REF", "CONTAINED-AR-ELEMENT-REFS", "CYCLE-STATUS-STORAGE", "REF", "PARAMETER-REF", "IKE-RAND-TIME", "TCP-FAST-RECOVERY-ENABLED", "INIT-RESOURCE", "TD-EVENT-OPERATION", "PROCESS-ARGUMENTS", "PERSISTENCY-FILE-PROXY", "USER-DEFINED-FIELD-DEPLOYMENT", "TYPE", "STATE-MANAGEMENT-SYNC-ACTION-ITEM", "FUNCTION-IDENTIFIER-REF", "TRIGGER-INTERFACE", "BOR-TIME-L-1", "PROCESS-MODE-MACHINE", "DTC-NUMBER", "PDU-COUNTER-SIZE", "XREF-TARGET", "REQUESTOR-REF", "RATE-LIMITATION-FILTERS", "TIMEOUT-CS", "TLS-JOB-MAPPING", "SEGMENT-REF", "INTERFACE-REF", "GRANT-REF", "ASYNCHRONOUS-SERVER-CALL-POINT", "NETWORK-ENDPOINT-REF", "DEPENDENT-RESOURCES", "CONTAINER-I-PDU", "SERIALIZATION-TECHNOLOGY", "GLOBAL-TIME-DOMAIN-REF", "TLS-CRYPTO-SERVICE-MAPPING", "INSTANCE-IN-MEMORY", "DDS-METHOD-DEPLOYMENT", "WRITE-ONLY-ONCE", "FIELDS", "RECOMMENDED-CONFIGURATION-REF", "ROM-BLOCK-INIT-VALUE", "LOG-SINK-REF", "TEXT-TABLE-MAPPINGS", "TCP-IP-ICMP-PROPS-REF", "MAX-SYNC-JUMP-WIDTH", "DATA-PROTOTYPE-TRANSFORMATION-PROPS", "SCALE-CONSTRS", "PARAMETER-ELEMENT-ACCESS", "SUB-NODE-CHANNELS", "ALGORITHM-FAMILY", "MULTIPLICITY-CONFIG-CLASSES", "SEGMENT-POSITIONS", "MODE-DECLARATIONS", "OFS-DATA-ID-LIST", "I-PDUS", "MAX-HEADER-ID", "TIMEOUT-CR", "NM-IMMEDIATE-RESTART-ENABLED", "DATA-DUMP-ENTRY", "SECURITY-DELAY-TIME", "SW-COMP-TO-ECU-MAPPING-REFS", "DIAGNOSTIC-ROUTINE-CONTROL", "ECUC-REFERENCE-VALUE", "START-IREF", "TARGET-PHM-CHECKPOINT-REF", "COMPONENT-IREF", "RAW-DATA-STREAM-SERVER-INTERFACE", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL", "CAN-XL-CONFIG-REQS", "TP-CHANNELS", "CPP-IMPLEMENTATION-DATA-TYPE", "BSW-TRIGGER-REF", "DIFFERENTIATED-SERVICE-FIELD", "INTERVAL-LENGTH", "COUNTER-JUMP-UP-VALUE", "PHM-CHECKPOINT-IREF", "TIME-BASE", "TOOL", "TARGET-COMPONENT-REF", "MESSAGE-LINE-NUMBER", "MC-DATA-ACCESS-DETAILS", "DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL", "CLIENT-SERVER-APPLICATION-ERROR-MAPPING", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING", "FUNCTION-POINTER-SIGNATURE-REF", "STATE-MANAGEMENT-ACTION-LIST", "PRIORITY-ID", "PROCESS", "EXCLUSIVE-AREA-POLICYS", "MSR-QUERY-ARG", "TIME-SYNCHRONIZATION", "STANDARD-REVISION", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION", "MULTICAST-CONNECTOR-REF", "CONSTANT-VALUE-MAPPING-REF", "HEALTH-CHANNEL-EXTERNAL-MODE", "USAGE", "SW-CLASS-IMPLS", "CONTEXT-P-PORT-PROTOTYPE-REF", "FIELD-SENDER-COM-SPEC", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "END-TO-END-CALL-RESPONSE-TIMEOUT", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "ARRAY-SIZE-HANDLING", "ARRAY-ELEMENT-REF", "E-2-E-EVENT-PROTECTION-PROPSS", "MAX-TESTER-CONNECTIONS", "TRIGGER-DIRECT-IMPLEMENTATIONS", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "VALUE-PAIRS", "SOURCE-EVENT-REF", "SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL", "TCP-TTL", "CALIBRATION-PARAMETER-VALUES", "SIGNAL-BASED-EVENT-DEPLOYMENT", "HAS-INTERNAL-STATE", "BUS-MIRROR-NETWORK-ID", "CALLBACK-HEADER-REF", "CONTEXT-REF", "MASTER-EVENT-REF", "POST-BUILD-VARIANT-CRITERION", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "TCP-IP-NDP-DELAY-FIRST-PROBE-TIME", "RPT-COMPONENTS", "BUNDLED-CONNECTIONS", "PDUS", "NETWORK-ENDPOINT", "LET-INTERVAL-REF", "REQUEST-RESULT", "MIRROR-STATUS-CAN-ID", "CRYPTO-PROVIDER-INTERFACE", "BSW-PER-INSTANCE-MEMORY-POLICYS", "INTERNAL-TRIGGERING-POINT-POLICYS", "TRANSMISSION-DEADLINE", "ENGINEERING-OBJECT", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "THRESHOLD", "PARAMETER-SIZE", "IDSM-PROPERTIES", "MACHINE-REF", "DETERMINISTIC-SYNC-MASTER-REF", "I-SIGNAL-I-PDU-REF", "TRACE-REFS", "CURRENT-VALUE-REF", "TCP-OPTION-FILTER-SET", "NM-NODE-ID", "PORT-SCHEDULER", "APPL-INIT-VALUE", "EXECUTABLE-ENTITY-REFS", "KEY-GENERATION", "SECURE-COM-PROPS-FOR-UDP-REF", "LENGTH-SEQUENCE-COUNTER", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "TARGET-RUNNABLE-ENTITY-REF", "VENDOR-API-INFIX", "ADDRESSING-FORMAT", "BYTE-VALUES", "SW-FILL-CHARACTER", "TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "FUNCTION-GROUP-MODE-IREF", "FILES", "SOCKET-OPTION", "TABLE-CAPTION", "CRYPTO-KEY-SLOT-REF", "TX-BIT-RATE-SWITCH", "RESPONSE-ON-EVENT-REF", "RETURN-CODE", "BSW-SERVICE-DEPENDENCY", "MEMORY-SECTION-LOCATION", "CRYPTO-NEED-REF", "BYPASS-ETHER-TYPES", "GLOBAL-TIME-DOMAIN", "SELECTOR-FIELD-CODE", "DIAGNOSTIC-AUTHENTICATION-CLASS", "SW-AXIS-GENERIC", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "NOTE", "PER-INSTANCE-MEMORY", "NM-PDU-RX-INDICATION-ENABLED", "NV-BLOCK-SW-COMPONENT-TYPE", "VARIATION-RESTRICTION", "DATA-IDS", "CONTAINER-TRIGGER", "GLOBAL-TIME-CAN-SLAVE", "SECURED-RX-VERIFICATION", "NM-SCHEDULE-VARIANT", "RESTRICT-UPDATE", "INDICATOR-STATUS-NEEDS", "MIN-PWM-L", "USES-ADDRESS-ARBITRATION", "SEC-OC-SECURE-COM-PROPS", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT", "MC-DATA-INSTANCE-REFS", "POSSIBLE-AP-ERROR-SET-REF", "ERROR-HANDLING", "EXPRESSION", "AGE", "EXECUTION-ERROR-REF", "FLEXRAY-NM-NODE", "COM-FIND-SERVICE-GRANT-DESIGN", "MEMORY-ID", "ECUC-QUERY-STRING-REF", "MODE", "API-PRINCIPLE", "INITIAL-DELAY-MAX-VALUE", "USES-MONITOR-DATA", "TX-PDU-TRIGGERING-REF", "CAN-XL-PROPS", "BY-PASS-POINT-IREFS", "STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM", "DIFFERENTIATED-SERVICE-CODE-POINT", "SW-VALUE-BLOCK-SIZE-MULTS", "ENDPOINT-CONFIGURATION-REF", "SOMEIP-PROVIDED-EVENT-GROUP", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "DIAGNOSTIC-SESSION-CONTROL", "ASSOCIATED-EVENT-IDENTIFICATION", "TLV-DATA-IDS", "BUILD-ACTION-ENVIRONMENTS", "BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING", "FRAGMENTATION-PROPS", "EXTERNAL-CLOCK-SYNCHRONISATION", "PLATFORM-PHM-ACTION-ITEM", "DCM-I-PDU", "CONTAINER-I-PDU-HEADER-BYTE-ORDER", "FOREIGN-MODEL-REFERENCE", "FUNCTIONAL-CLUSTER-REF", "LIMIT-KIND", "OBJECT-DEFINTION-REF", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "BANDWIDTH-LIMITATION", "INTERPOLATION-ROUTINE-MAPPING-SET-REFS", "SW-RECORD-LAYOUT-REF", "INHIBITING-SECONDARY-FID-REFS", "ECUC-REF", "INITIAL-REPETITIONS-MAX", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "TTCAN-COMMUNICATION-CONNECTOR", "SWC-BSW-SYNCHRONIZED-TRIGGER", "CAN-NM-NODE", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "INDENT-SAMPLE", "COM-CONTROL-CLASS-REF", "MODULE-REF", "TIMING-DESCRIPTION-REF", "SOFTWARE-COMPOSITION-TREF", "FIRST-PORT-REF", "ECU-INSTANCE-PROPSS", "HARDWARE-ELEMENT-REFS", "NM-PDU", "SD-SERVER-EVENT-CONFIG", "SW-GENERIC-AXIS-PARAM-TYPE-REF", "SSP-OFFSET", "DNS-SERVER-ADDRESS", "DIAGNOSTIC-ROUTINE-INTERFACE", "SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE", "LOCAL-SUPERVISION-ENTITY-REFS", "FORBIDDEN-SIGNAL-PATH", "PWM-O", "META-DATA-ITEM-TYPE", "DDS-DOMAIN-RANGE", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF", "NRC-VALUE", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "NM-DETECTION-LOCK", "DOCUMENTATION-CONTENT", "CLIENT-ID-DEFINITION", "SDG-CONSTRAINT-REFS", "OFNS-DATA-ID-LIST", "PERSISTENCY-FILE-ARRAY-REF", "CAN-ENTER-EXCLUSIVE-AREA-REF", "CRYPTO-KEY-SLOT-INTERFACE", "DIAGNOSTIC-COM-CONTROL", "SUB-FUNCTION-REF", "OUT-MEASUREMENT-SET", "TEMPERATURE-EXP", "PARTIAL-NETWORK-REF", "CAN-FRAME-TRIGGERING", "ERROR-DOMAIN-REF", "UNIT-AND-SCALING-ID", "MC-DATA-INSTANCE-VAR-REF", "WATCHDOG-ACTION-ITEM", "SPECIFIC-CHANNELS", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING", "POSSIBLE-ERROR-REF", "IKE-OVER-TIME", "MINIMUM-INTENDED-BINDING-TIME", "MAPPED-SWC-SERVICE-DEPENDENCY-IREF", "MANUFACTURER-VALIDATION-ORDER-REFS", "HW-PORT-MAPPING", "RUN-RESOURCE", "SWC-MODE-MANAGER-ERROR-EVENT", "DIAGNOSTIC-SOVD-LOCK", "TIMING-ARGUMENT-REF", "CONNECTORS", "DDS-METHOD-QOS-PROPS", "TTCAN-PHYSICAL-CHANNEL", "SEQUENCE-COUNTER-JUMP-WIDTH", "SIZE-OF-STRUCT-LENGTH-FIELD", "ACCEPTED-REMOTE-CERTIFICATE-REF", "COMPU-RATIONAL-COEFFS", "FRAME-LENGTH", "HEADER-ID-SHORT-HEADER", "IDENTITIY-NUMBER", "EVENT-OCCURRENCE-KIND", "MAX-NUM-COMPARISION-OF-VALUE-EVENTS", "DESCRIPTION", "SO-AD-CONFIG", "CHECKPOINT-INITIAL-REFS", "ADMIT-WITHOUT-MESSAGE-ID", "CRYPTO-CERTIFICATE-INTERFACE", "TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY", "RETURN-TYPE", "SOFTWARE-MEMORY-SECTION-REF", "SDG-AGGREGATION-WITH-VARIATION", "DLT-MESSAGE", "DATA-ELEMENT-REF", "CONTEXT-COMPONENT-PROTOTYPE-REF", "CAN-ENTER-EXCLUSIVE-AREA-REFS", "FLOAT-VALUE-VARIATION-POINT", "DATA-RECEIVE-POINTS", "COMMUNICATION-DIRECTION", "CONTEXT-APPLICATION-RECORD-ELEMENT-REF", "RECEIVER-IREF", "MEMORY-SECTIONS", "CYCLE-TIME-MAIN-FUNCTION", "RECEIVED-DATA-REF", "RTE-EVENT-SEPARATIONS", "DERIVED-FROM-BLUEPRINT-REFS", "TIMEOUT-CHECK", "DIAGNOSTIC-REQUEST-DOWNLOAD", "SYNC-DATA-ID-LIST", "APPLICATION-PARTITION", "TIMEOUT-AR", "ECUC-VALUES", "TIMING-CONDITION-REF", "TLS-SECURE-COM-PROPS", "SOVD-SERVER-INSTANTIATION", "VALUE-RESTRICTION", "AUTOSAR-DATA-TYPE-REF", "IS-DELTA-PACKAGE", "SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "NM-COM-CONTROL-ENABLED", "CAN-CONTROLLER-CONFIGURATION", "ALLOW-CONTENT-TYPE-CHANGE", "CLEAR-FROM-VALID-TO-INVALID", "RAM-BLOCK-STATUS-CONTROL", "POST-BUILD-VARIANT-CONDITIONS", "ECUC-MODULE-CONFIGURATION-VALUES-REF", "LOG-AND-TRACE-INSTANTIATION", "J-1939-SHARED-ADDRESS-CLUSTERS", "NEW-NAD", "CLIENT-CAPABILITY", "GENERIC-ETHERNET-FRAME", "PNC-VECTOR-OFFSET", "P-2-TIMING", "IP-V-4-ADDRESS", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "INCLUDED-MODE-DECLARATION-GROUP-SETS", "PARTS", "MIN-DATA-LENGTH", "DIAGNOSTIC-DATA-IDENTIFIER-REF", "BLUEPRINT-REF", "AUTH-ALGORITHM", "MIN-SAMPLE-POINT", "PERIOD-END", "TOL", "CRYPTO-MODULE-INSTANTIATION", "LOCAL-PORT-RANGE-START", "PERSISTENCY-KEY-VALUE-PAIR", "DEFAULT-PRIORITY", "UDS-SERVICE", "HW-TYPE", "LOGIC-ADDRESS", "FT", "FILTER", "ACTION-LISTS", "LOCAL-UDP-PORT-REF", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "SW-REFRESH-TIMING", "UNASSIGN-FRAME-ID", "SYSTEM-SIGNAL-GROUP-REF", "TIME-BASE-RESOURCE-REF-CONDITIONAL", "FRAME-TRIGGERINGS", "URI", "DTC-SETTING-CLASS-REF", "MAX-DELTA-COUNTER-INIT", "ALLOW-PROVIDER-RATE-CORRECTION", "SAFETY-CONDITION", "MESSAGE-LINK-LENGTH", "ECU-TASK-PROXY-REF", "DEFERRING-FID-REFS", "FIRST-MODE-REF", "DOIP-CONNECTION-REF", "SHORT-NAME-PATTERN", "COMPU-NUMERATOR", "BULK-NV-DATA-DESCRIPTOR", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE", "REMOTE-MULTICAST-CONFIG-REF", "CYCLE-COUNT-MAX", "LOCAL-PARAMETER-REF", "ETHERNET-PRIORITY-REGENERATION", "CRYPTO-ALGO-CONFIG", "ACL-OPERATION-REFS", "FLEXRAY-COMMUNICATION-CONTROLLER", "SW-ALIGNMENT", "TRANSFORMER-CLASS", "USE-VEHICLE-IDENTIFICATION-SYNC-STATUS", "SW-COMPONENT-DOCUMENTATION", "STORE-ON-CHANGE", "ECU-INSTANCE-REFS", "ECUC-VALUE-CONFIGURATION-CLASS", "REQUIRED-MODE-GROUPS", "NETWORK-INTERFACES", "ROOT-DATA-PROTOTYPE-IN-SR-REF", "REQUIRES-SYMBOLIC-NAME-VALUE", "WRITE-CLASS-REF", "TROUBLE-CODE-UDS-REF", "HEALTH-CHANNEL-EXTERNAL-STATUS", "TT", "SOUCE-CAN-ID-REF", "USER-DEFINED-TRANSFORMATION-PROPS", "ETHERNET-FRAME-TRIGGERING", "NAME-TOKEN-VALUE-VARIATION-POINT", "DIAGNOSTIC-AUTH-ROLE", "AUTO-START", "FM-FEATURE-RESTRICTION", "APPLICATION-RECORD-ELEMENT", "RUNTIME-IP-ADDRESS-CONFIGURATION", "OPTIONS", "RESPONSE-GROUP-REF", "COMPOSITE-INTERFACE", "ACCURACY-INT", "WAKE-UP-BY-CONTROLLER-SUPPORTED", "SENDER-INTENT", "CLIENT-SERVER-INTERFACE-REF", "ROOT-COMPOSITION", "FRAME-PORT-REFS", "CRYPTO-SERVICE-KEY-REF", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "WORST-CASE-EXECUTION-TIME", "MSR-QUERY-RESULT-CHAPTER", "HEALTH-CHANNELS", "SUPERVISION-MODE-CONDITIONS", "NM-REPEAT-MSG-INDICATION-ENABLED", "MC-FUNCTION-DATA-REF-SET-VARIANTS", "MSG-ID-MASK", "STATUSS", "SOMEIP-TP-CONNECTION", "SECURITY-DELAY-TIME-ON-BOOT", "BSW-ENTRY-RELATIONSHIPS", "MIN-ALIVE-CYCLE", "SIGNAL-SERVICE-TRANSLATION-PROPSS", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "NODE-IDENTIFIER", "FLOW-CONTROL-PDU-REF", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "COUPLING-PORT-CONNECTIONS", "CONSISTENCY-NEEDSS", "WRITE-VERIFICATION", "MANUFACTURER-VALIDATION-ORDER-REF", "IDSM-RATE-LIMITATION", "INSTANTIATION-RTE-EVENT-PROPSS", "PORT-INTERFACE-MAPPINGS", "INFO-TYPE-REF", "FIREWALL-STATE-IREF", "WAKE-UP-CHANNEL", "TEXT", "SERVICE-INTERFACE-ELEMENT-MAPPING-REF", "EXT-HEADER-FILTER-LISTS", "DESTINATION-URI-DEFS", "PRE-SHARED-KEY-REF", "AR-PARAMETER", "SUPPORT-INFO-BIT", "UDP-TP", "KEY-EXCHANGE-REFS", "SECURITY-EVENT-FILTER-CHAIN-REF", "SYNCHRONIZED-TIME-BASE-PROVIDER", "APPLICATION-INTERFACE", "LOGICAL-EXPRESSION", "OCCURRENCE-EXPRESSION", "DATA-TRANSFORMATION-SET", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "FIREWALL-REF", "TARGET-DATA-PROTOTYPE-IN-SR-REF", "SETTER-CALL-REF", "NM-CAR-WAKE-UP-FILTER-ENABLED", "TESTED-ITEM-REF", "SEED-SIZE", "EXPECTED-TX-TRIGGER", "SERVER-PORT-REF", "CLIENT-SERVER-ANNOTATIONS", "KEY-SLOT-USAGE", "CAPABILITY-RECORDS", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "VENDOR-SPECIFIC-SERVICE-NEEDS", "DETERMINISTIC-CLIENT-RESOURCE-NEEDSS", "WRITTEN-READ-NV-DATA", "TROUBLE-CODE-J-1939-REF", "DDS-TOPIC-ACCESS-RULE", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "DATA-LENGTH", "R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "L-4", "DISPLAY-FORMAT", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "SW-RECORD-LAYOUT-V-PROP", "SW-CODE-SYNTAX", "TARGET-FRAME-REF", "PDU-COUNTER-THRESHOLD", "SW-VARIABLE-PROTOTYPE-REF", "COM-CONFIGURATION-RX-TIME-BASE", "SYSTEM-SIGNAL-REFS", "RUNNABLE-ENTITY-GROUP-IREFS", "ACL-CONTEXT", "PORT-API-OPTIONS", "ACL-OPERATION-REF", "LOCAL-IP-SEC-RULE-REFS", "DATA-PROTOTYPE-REF", "TP-TECHNOLOGY", "PORT-BLUEPRINT-REF", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "INDEX-DATA-TYPE-REF", "SOURCE-MAC-ADDRESS", "SAFETY-CONDITIONS", "ASSIGNED-TRAFFIC-CLASS", "CONTEXT-SW-COMPONENT-PROTOTYPE-REF", "SLAVES", "EVENT-HANDLER-REF", "FLEXRAY-AR-TP-CONFIG", "FLEXRAY-CLUSTER-VARIANTS", "NETWORK-INTERFACE-ID", "BINARY-MANIFEST-ITEM-DEFINITION", "LIN-CLUSTER-VARIANTS", "TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED", "L-10", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "LENGTH", "DOIP-INTERFACES", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL", "TIMING-PROPS", "SENDER-REC-ARRAY-TYPE-MAPPING", "ERROR-REACTION-POLICY", "TYPE-TAILORINGS", "DIAGNOSTIC-CONNECTED-INDICATOR", "IS-PRIVATE", "FIRST-TRIGGER-REF", "SEC-OC-DEPLOYMENT", "MAX-ERROR-STATE-INIT", "ROOT-REF", "NUMBER-OF-WORKERS", "RPT-PROFILES", "LOG-TRACE-PROCESS-ID", "RECORD-NUMBER", "SOFTWARE-CLUSTER-DESIGN", "MACHINE-DESIGN", "PDU-COLLECTION-PDU-TIMEOUT", "CHANNEL-NAME", "DIAGNOSTIC-SESSION-REFS", "COLD-START-ATTEMPTS", "REQUESTER-SOFTWARE-CLUSTER-REF", "TLV-RECORD-ELEMENT-REF", "DATA-LINK-LAYER-RULE", "ACK-TYPE", "DESTINATION-MAC-ADDRESS-MASK", "CAN-NM-TX-CAN-ID", "J-1939-CONTROLLER-APPLICATION-REF", "REPORTABLE-SECURITY-EVENT-REF", "FM-ATTRIBUTE-VALUE", "CURRENT-DATA-TYPE-REF", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "SECOND-ELEMENT-REFS", "IDSM-TRAFFIC-LIMITATION", "RESOURCE-ESTIMATIONS", "COMPONENTS", "TRANSPORT-PLUGIN", "DATA-PROTOTYPE-GROUP-IREFS", "DISPLAY-NAME", "LOC-MEASUREMENT-SET", "RESISTANT-TO-CHANGED-SW", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "OPERATION-MODE", "N-PDU", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "CIPHER-SUITE", "LOGGING-BEHAVIOR", "DEFAULT-TRACE-STATE", "ALLOWED-TCP-OPTIONS-REF", "SOURCE-CHANNEL", "SEC-OC-JOB-MAPPING", "RESTORE-AT-START", "SECURE-COM-PROPSS", "STARTUP-CONFIGS", "DO-IP-RULE", "RESOURCE-DEFINITION-REF", "REQUIRED-DDS-SERVICE-INSTANCE", "SW-AXIS-GROUPED", "UNICAST-NETWORK-ENDPOINT-REF", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "REDUNDANCY-HANDLINGS", "HAS-GETTER", "CONTROLLED-DATA-ELEMENT-REF", "UNICAST-SECURE-COM-PROPS-REFS", "COUPLING-PORT-REF", "CALCULATION-FORMULA", "REQUEST-METHOD", "DO-IP-CONFIG", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS", "REFILL-AMOUNT", "TIMING-VARIABLE-REF", "PERSISTENCY-DATA-ELEMENT", "ALLOCATOR", "SW-AXIS-TYPE", "SW-FEATURE-DEF", "PREFIX", "INTERRUPT-CATEGORY", "PORT-INTERFACE-BLUEPRINT-REF", "FIRST-ELEMENT-INSTANCE-IREF", "DLT-CONTEXT", "FM-COND", "BAUDRATE", "EXECUTABLE-ENTITY-REF", "SEPARATION-TIME-RESPONSE", "TD-EVENT-SLLET-PORT", "BSW-MODULE-ENTRY-REF-CONDITIONAL", "MAX-RPT-EVENT-ID", "TD-EVENT-BSW-MODE-DECLARATION", "IP-SEC-RULE", "OPERATIONS", "MIN-NUMBER-OF-ELEMENTS", "CONNECTED-INDICATORS", "HW-PIN-GROUPS", "BINARY-MANIFEST-ITEM", "MINIMUM-SELECTED-BINDING-TIME", "CLEAR-DTC-LIMITATION", "ETHERNET-RAW-DATA-STREAM-MAPPING-REF", "ASSIGN-FRAME-ID-RANGE", "SERIALIZER", "J-1939-DCM-I-PDU", "RPT-SERVICE-POINT-PRE-REF", "SW-SYSTEMCONSTANT-VALUE-SET-REFS", "INTERPOLATION-ROUTINES", "TD-EVENT-SERVICE-INSTANCE-EVENT", "ASSIGNMENT-PRIORITY", "DEFAULT-LEASE-TIME", "STATE-MANAGEMENT-BASED-ON-TCP-FLAGS", "DATA-TRANSFORMATIONS", "NM-SYNCHRONIZATION-POINT-ENABLED", "INDEX", "HW-PIN-GROUP-CONTENT", "ANNOTATIONS", "SOMEIP-SERVICE-INTERFACE", "RATE-POLICYS", "INTERPOLATION-ROUTINE-MAPPING", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "SECURITY-EVENT-MAPPING", "TTL", "NM-ECU", "BIT-NUMBER", "MACROTICK-DURATION", "KEY-SLOT-REF", "CONSTRAINT-TAILORING", "PNC-GROUP-REFS", "CHECKPOINTSS", "USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS", "USE-MAC-ADDRESS-FOR-IDENTIFICATION", "MIN-OK-STATE-INVALID", "MINIMUM-RX-CONTAINER-QUEUE-SIZE", "SNAPSHOT-RECORD-CONTENTS", "DIAGNOSTIC-TRANSFER-EXIT", "SENDER-RECEIVER-ANNOTATIONS", "ACCESS-PERMISSION-VALIDITY", "IPV-4-RULE", "ERROR-CONTEXT-REF", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "BSW-MODULE-DESCRIPTION-REF", "SDG-CAPTION-REF", "PHYSICAL-DIMENSION-MAPPING-SET", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "ECU-EXTRACT-VERSION", "DLT-ECU-REF", "SOVD-LOCK-REF", "SYSTEM-MEMORY-USAGE", "STATUS-REF", "SDG", "TIME-PASSED-THRESHOLD", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "PROCESSOR", "TIME-SYNC-SERVER-REF", "USE-AUTO-VALIDATION-AT-SHUT-DOWN", "DATA-WRITE-ACCESSS", "INPLACE", "BINARY-MANIFEST-META-DATA-FIELD", "OFFSET", "COMPU-CONST", "DEBOUNCE-ALGORITHM", "BSW-MODULE-CLIENT-SERVER-ENTRY", "E-2-E-PROFILE-CONFIGURATIONS", "TIMING-CONDITION-FORMULA", "CONSTANT-MAPPING-REF", "LOCAL-COMM-CONNECTOR-REF", "SECURE-COM-PROPS-SET", "DIAGNOSTIC-DTC-CHANGE-TRIGGER", "COMMAND", "BINARY-MANIFEST-REQUIRE-RESOURCE", "OBD-RATIO-SERVICE-NEEDS", "FIRST-ELEMENT-REF", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL", "SIGNATURE-SCHEME-ID", "PLCA-LOCAL-NODE-ID", "CONTROLLED-DATA-ELEMENT-REFS", "REDUNDANCY", "PREDECESSOR-REF", "FRESHNESS-VALUE-ID", "IP-IAM-REMOTE-SUBJECT", "STATE-MANAGEMENT-ERROR-COMPARE-RULE", "INTERFACE-VERSION", "LOGICAL-SUPERVISIONS", "IEEE-1722-TP-ETHERNET-FRAME", "CLIENT-PORT-FROM-CONNECTION-REQUEST", "SENDER-RECEIVER-TO-SIGNAL-MAPPING", "ARTIFACT-LOCATOR", "HW-PIN-REF", "SUPERVISED-ENTITY-NEEDS", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "OPTION-ARGUMENT", "KEYWORD", "N-PDU-REFS", "UCM-SUBORDINATE-MODULE-INSTANTIATION", "MIN-SOURCE-PORT-NUMBER", "USE-CASE", "SD-SERVER-CONFIG", "SW-NUMBER-OF-AXIS-POINTS", "MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE", "AUTHENTICATION-ROLE-REF", "SIGNAL-AGE", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "START-POSITION", "PDU-TRIGGERING-REFS", "ENABLES-LOG-TRACE", "SDG-REFERENCE", "SW-SYSTEMCONSTANT-VALUE-SET", "TIME-SYNC-R-PORT-PROTOTYPE-IREF", "TRANSMISSION-MODE-FALSE-TIMING", "QUEUE-SIZE", "TRIGGER-INTERFACE-MAPPING", "SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "SIZE-OF-STRING-LENGTH-FIELD", "LATENCY-TIMING-CONSTRAINT", "EXCLUSIVE-AREAS", "KEY-EXCHANGE-REF", "MAXIMUM-SUPPORTED-UCM-VERSION", "SERIALIZER-REF", "TRANSITION-REF", "MIN-NUMBER-OF-OCCURRENCES", "USER-DEFINED-EVENT-DEPLOYMENT", "PROP-SEG", "STATUS-IREF", "TERMINATING-CHECKPOINT-REF", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS", "SOURCE-ELEMENT-REFS", "CONTROL-PROVIDED-EVENT-GROUP-REFS", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "UPPER-CAN-ID", "FREEZE-FRAME-REF", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS", "CLASS-TAILORINGS", "FIFO-DEPTH", "VARIANT-ID", "CAN-BE-INVOKED-CONCURRENTLY", "PDU-COLLECTION-SEMANTICS", "ENTRYS", "SECONDARY-FRESHNESS-VALUE-ID", "INHIBITING-FID-REF", "REG-REQUIRES-STABILITYS", "CONTEXT-ELEMENT-REF", "PROPS", "RESULTING-PROPERTIES", "TD-EVENT-OPERATION-TYPE", "IMPLEMENTS-LEGACY-STRING-SERIALIZATION", "J-1939-NM-ECU", "COMM-CONTROLLERS", "I-SIGNAL-TO-I-PDU-MAPPINGS", "RELEASED-TRIGGERS", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "DIAGNOSTIC-ENV-DATA-CONDITION", "MAXIMUM-SELECTED-BINDING-TIME", "TCP-IP-IP-REASSEMBLY-BUFFER-COUNT", "TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "SDXF", "FURTHER-ACTION-BYTE-NEEDS", "ITEMS", "TIMING-MODES", "PNC-IDENTIFIER", "CERT-COMMON-NAME", "DIAGNOSTIC-CONNECTION-REF", "TD-EVENT-TYPE", "DEFAULT-ACTION", "SERVER-NAME-IDENTIFICATION", "RUNNABLE-ENTITY-GROUP-IREF", "DIAGNOSTIC-TROUBLE-CODE-UDS", "DISABLED-MODE-IREF", "HARDWARE-TEST-NEEDS", "SW-RECORD-LAYOUT-V-INDEX", "HAS-DYNAMIC-LENGTH", "HW-ATTRIBUTE-DEF", "SECURE-COM-PROPS-FOR-TCP-REF", "MODE-DECLARATION-MAPPINGS", "SERVICE-INTERFACE-VERSION", "MAXIMUM", "KEEP-ALIVE-INTERVAL", "SOURCE-INSTANCE-IREFS", "SDG-CAPTION", "KEYWORDS", "DRIVER-NOTIFICATIONS", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "CUSTOM-TRIGGER", "STOP-IREF", "NUMERICAL-VALUE-SPECIFICATION", "CRYPTO-SERVICE-MAPPINGS", "CONFIDENCE-INTERVALS", "ECUC-BOOLEAN-PARAM-DEF", "SWC-TO-ECU-MAPPING-CONSTRAINT", "DEFAULT-VALUE-HANDLING", "LENGTH-EXP", "SUB-SOFTWARE-CLUSTER-REF", "PROTOCOL", "KEY-ID", "CRC-SECURED", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "SW-VALUE-CONT", "LOCAL-SUPERVISION", "ECUC-DEFINITION-REF", "TEST-PATTERN", "MAXIMUM-LENGTH", "DESC", "E-2-E-PROFILE-COMPATIBILITY-PROPS-REF", "ISSUED-BY", "ADDITIONAL-NATIVE-TYPE-QUALIFIER", "GROUPED-ALIAS-EVENT-REF", "SUB-NODE-NUMBER", "TARGET-CATEGORY", "MODE-GROUP", "TIME-LEAP-FUTURE-THRESHOLD", "PAYLOAD-BYTE-PATTERN-RULE-PARTS", "LOGIC-ADDRESSS", "CRYPTO-CERTIFICATE", "SAK-REF", "UDP-MIN-TX-BUFFER-SIZE", "REASON", "IDS-COMMON-ELEMENT-REF", "TYP", "INNER-PORT-IREF", "DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF", "IP-PROTOCOL", "NV-BLOCK-DESCRIPTOR", "RUNTIME-ERROR", "COMPOSITION-SW-COMPONENT-TYPE-REF", "BSW-BEHAVIOR-REF", "RAW-DATA-STREAM-INTERFACE", "PWM-S", "TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED", "HW-ATTRIBUTE-DEFS", "VIEW-MAP-SET", "SOURCE-POSSIBLE-ERROR-REF", "NM-USER-DATA-OFFSET", "COM-FIELD-GRANT", "TLV-SUB-ELEMENT-REF", "MINIMUM-NUMBER-OF-REQUESTS", "MSR-QUERY-RESULT-P-1", "INPUT", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "CHANNEL-ID", "DLT-CONFIG", "SW-RECORD-LAYOUT-GROUP-INDEX", "MIN-LENGTH", "M", "LAST-EGRESS-SCHEDULER-REF", "MACHINE-MODE-MACHINES", "GENERIC-MATH", "I-PDU-IDENTIFIER-UDP-REF", "EVENT-GROUP-IDENTIFIER", "PERSISTENCY-REDUNDANCY-HASH", "PROVIDE-RESOURCES", "END-TO-END-PROFILE", "CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING", "DESTINATION-REF", "ECUC-URI-REFERENCE-DEF", "ASSOCIATED-PDUR-I-PDU-GROUP-REFS", "SLEEP-MODE-SUPPORTED", "NETWORK-SEGMENT-ID", "SOFTWARE-PACKAGE-STEP", "LIN-PHYSICAL-CHANNEL", "SHALL-RUN-ON-REFS", "PERSISTENCY-DATA-PROVIDED-COM-SPEC", "TIME-SYNC-P-PORT-PROTOTYPE-IREF", "DLT-ECU", "PATH-MTU-DISCOVERY-ENABLED", "UCMS", "IMPORTANCE", "MINIMUM-INTERVAL-LENGTH", "RULES", "TRCV-DELAY-COMPENSATION-OFFSET", "TRANSMISSION-MODE-CONDITION", "SW-COMPONENT-ASSIGNMENTS", "SHORT-TERM-ADJUSTMENT-SUPPORTED", "TP-ADDRESSS", "PORT-DEFINED-ARGUMENT-BLUEPRINTS", "TCP-IP-ICMP-V-4-TTL", "TLS-JOB-REQUIREMENT-REF", "CRC-FLAGS", "FAILED-REFERENCE-CYCLES-TOLERANCE", "DIAG-PDU-TYPE", "RECORD-ELEMENTS", "SCALING-INFO-SIZE", "SECOND-VALUE", "MORE-FRAGMENTS", "MODE-DEPENDENT-STARTUP-CONFIG", "SW-POINTER-TARGET-PROPS", "RPT-EXECUTABLE-ENTITY-PROPERTIES", "MIN-OK-STATE-INIT", "ECUC-MODULE-CONFIGURATION-VALUES", "LOAD-BALANCING-PRIORITY", "SESSION-ID", "COMMUNICATION-RESOURCE-REF", "TIME-BASE-CONSUMER-REF", "EXECUTION-CONTEXT", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "FIELD", "DLT-APPLICATION-REF", "TRANSIENT-FAULT", "COM-MANAGEMENT-PORT-GROUP-IREF", "ACCEPTS-ENCODINGS", "LIVELINESS-PROTECTION-KIND", "ACCESS-COUNTS", "REMOTE-SUBJECT-REFS", "DISTINGUISHED-PARTITIONS", "CRYPTO-KEY-DESCRIPTION", "PREDECESSOR-ACTION-REF", "R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "SW-CLASS-IMPL", "SERVICE-INTERFACE-ELEMENT-MAPPING-REFS", "TYPE-MAPPING", "CONTROL-CONSUMED-EVENT-GROUP-REF", "LIN-COMMUNICATION-CONNECTOR", "DIAGNOSTIC-UPLOAD-INTERFACE", "REST-RESOURCE-DEF", "INITIAL-REPETITIONS-BASE-DELAY", "CONCRETE-CLASS-TAILORING", "COMPARE-VALUE", "CRYPTO-SERVICE-QUEUE", "COMMON-SIGNAL-PATH", "END-ADDRESS", "WAKE-UP-OVER-BUS-SUPPORTED", "COMM-CONTROLLER-REF", "ERROR-CONTEXT-REFS", "MAC-MULTICAST-GROUP-REF", "AUTOSAR-ENGINEERING-OBJECT", "NM-REPEAT-MESSAGE-BIT-ENABLE", "SECOND-TO-FIRST-DATA-TRANSFORMATION-REF", "GROUP-ID", "FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS", "EVENT-OBD-READINESS-GROUP", "CREATE-ECU-WAKEUP-SOURCE", "IMPLEMENTED-IN-REF", "WAKEUP-FORWARD-LOCAL-ENABLED", "SIGNAL-I-PDU-COUNTER", "LOG-SINK-REFS", "IP-IAM-AUTHENTIC-CONNECTION-PROPS", "DESTINATION-BASE-ID", "NM-IMMEDIATE-NM-TRANSMISSIONS", "IUMPR-REFS", "NAMED-CURVE-ID", "PARAMETER", "MIN-MINOR-VERSION", "STATIC-PARTS", "SYNC-FRAME-ID-COUNT-MAX", "MODE-DECLARATION-GROUP-PROTOTYPE", "SOFTWARE-PACKAGE", "DATA-TYPE-MAPS", "CLUSTERED-COMPONENT-IREF", "ALIVE-SUPERVISION", "CONTAINED-PACKAGE-ELEMENT-REF", "NM-READY-SLEEP-TIME", "FULLY-QUALIFIED-DOMAIN-NAME", "CLIENT-SERVER-OPERATION-COM-PROPS", "CLEAR-EVENT-ALLOWED-BEHAVIOR", "USE-AS-CRYPTOGRAPHIC-I-PDU", "PHM-ARBITRATION", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REF", "IPV-6-CONFIGURATION", "SDG-CLASS-REF", "MAX-SERVICE-POINT-ID", "EVALUATED-ELEMENT-REF", "PERSISTENCY-DEPLOYMENT-URI", "LIN-NM-CLUSTER", "SECURITY-ACCESS-LEVEL", "CPU-USAGE", "APPLICATION-PARTITION-REF", "INCLUDE-REF", "REGENERATED-PRIORITY", "IS-UNUSED", "STATE-MANAGEMENT-COMPARE-FORMULA", "LOG-TRACE-PROCESS-DESC", "MONITORED-IDENTIFIER-REF", "SERVICE-DEPENDENCYS", "FLOW-CONTROL-PDU-REFS", "MATCHING-CRITERION-REF", "MAX-NUMBER-OF-RESP-PENDING-FRAMES", "PDU-REF", "USER-DEFINED-CLUSTER", "DIAGNOSTIC-CONTRIBUTION-REF", "COMPONENT-CLUSTERING", "BIT", "KEY-SLOT-USED-FOR-START-UP", "CLIENT-SERVER-OPERATION-MAPPING", "PORT-PROTOTYPE-SLUG-FRAGMENT", "TX-NM-PDU-REF", "AUTHENTICATION-CLASS-REF", "SERVICE-REQUEST-CALLBACK-TYPE", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF", "N-PDU-REF", "REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF", "ECU-MAPPING", "DEADLINE-MIN", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "WRITING-STRATEGYS", "MODE-DECLARATION-GROUP-PROTOTYPE-IREF", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "ARTIFACT-CHECKSUM", "SOURCE-I-PDU-REF", "MODE-DECLARATION-REF", "DIAG-ARG-INTEGRITY", "REPLAY-PROTECTION", "SOURCE-FIELD-REF", "TLS-SECURE-COM-PROPS-REF", "IN-MEASUREMENT-SET", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "DIAGNOSTIC-EVENT-INFO-NEEDS", "ECUC-VALUE-COLLECTION", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS", "CKN-REF", "DESTINATION-PHYSICAL-ADDRESS", "RELATED-TRACE-ITEM-REF", "ABSTRACT-CLASS-TAILORING", "MEMORY-ALLOCATION-KEYWORD-POLICY", "CYCLE-TIME-VALUE", "MODE-ACCESS-POINTS", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "ARRAY-SIZE", "TP-NODES", "CAN-COMMUNICATION-CONNECTOR", "VALUE-ACCESS", "STATE-MANAGEMENT-EM-ERROR-INTERFACE", "IMPLIED-OPERATION-REFS", "SOURCE-EEC-REFS", "DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF", "UDP-NM-ECU", "INVALIDATION-POLICYS", "READ-SCALING-DATA-CLASS-REF", "INTERPOLATION-ROUTINE", "CONTROL-ENABLE-MASK-BITS", "CONFIRMATION-THRESHOLD", "UPPER-LIMIT", "REQUEST-GROUP-REF", "SUPPORT-DIRTY-FLAG", "COMPILER", "AVERAGE-MEMORY-CONSUMPTION", "SOMEIP-TRANSFORMATION-PROPS-REF", "ATTRIBUTE-NAME", "WITH-AUTO", "KEY-REF", "REQUEST-RESPONSE-DELAY", "BUILD-TYPE", "SOURCE-REF", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "FM-FEATURE-SELECTION-SET", "MONOTONY", "FIRST-MODE-GROUP-REF", "DATA-SEND-POINTS", "DESTINATION-TYPE", "DIAGNOSTIC-EXTRACT-REF", "K-INIT-RESOURCE", "TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED", "CRYPTO-TRUST-MASTER-INTERFACE", "COMMUNICATION-CONNECTOR-REFS", "VLAN-MEMBERSHIP", "RANGE-MAX", "MINOR-PROTOCOL-VERSION", "BSW-DATA-RECEIVED-EVENT", "FUNCTIONAL-REQUEST-REF", "SERVER-SERVICE-MINOR-VERSION", "AGING-CYCLES", "EXCLUSIVE-AREA-NESTING-ORDERS", "RELEASED-TRIGGER-REF", "SYNCHRONIZED-SLAVE-TIME-BASE", "METHOD-DEPLOYMENTS", "ALIAS-NAME-ASSIGNMENT", "SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF", "HEALING-CYCLE-REF", "SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST", "MODE-IREFS", "CONSUMED-EVENT-GROUPS", "TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT", "REQUIRES-INDEX", "IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF", "TP-ADDRESS-EXTENSION-VALUE", "PDU-REPLICATION-VOTING", "OFS-DATA-ID-LISTS", "TIME-OFFSET", "COMMANDS", "FRESHNESS-VALUE-TX-LENGTH", "EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD", "EVENT-MAPPING", "FIREWALL-RULE-REFS", "PROVIDED-DATA-REF", "CRYPTO-OBJECT-TYPE", "PHYSICAL-CHANNELS", "CONTEXT-ROOT-COMPONENT-REF", "MAX-RETRIES", "STREAM-IDENTIFIER", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "GLOBAL-RESOURCE-ID", "DID-SIZE", "SUPERVISED-ENTITY-ID", "DIAGNOSTIC-J-1939-SPN", "CONTROL-PROVIDED-EVENT-GROUP-REF", "GLOBAL-SUPERVISION-ENTITYS", "MODE-PORT-ANNOTATION", "MAX-GRADIENT", "ROM-LOCATION-REF", "BLOCK-STATE", "ECUC-DESTINATION-URI-DEF-SET", "SW-ADDR-METHOD-REF", "CAN-CONFIG", "I-SIGNAL-I-PDUS", "DO-IP-INSTANTIATION", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "ENTRY", "PERSISTENCY-REDUNDANCY-M-OUT-OF-N", "READ-NV-DATA", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS", "SOCKET-ADDRESSS", "RELATIVE-REPRESENTATION-TIME", "SECOND-ELEMENTS", "ALLOWED-TCP-OPTIONS", "NM-MSG-CYCLE-OFFSET", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "ACTION-ITEM-LISTS", "FAILURE-MONITORING-REF", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "EVENT-HANDLER-REFS", "TARGET-METHOD-REF", "DO-IP-ENTITY", "GLOBAL-TIME-SUB-DOMAINS", "EVENT-ELEMENT-MAPPINGS", "P", "FIELD-REF", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF", "DISCOVERY-PROTECTION-KIND", "COMMON-PROPERTIES", "WRITTEN-NV-DATA", "ENABLE-CONDITION-GROUP-REF", "NM-NID-POSITION", "MACRO-INITIAL-OFFSET-B", "MODE-MANAGER-ERROR-BEHAVIOR", "JOB-REQUIREMENTS", "ARRAY-ELEMENT-MAPPINGS", "REQUIRED-CLIENT-SERVER-ENTRYS", "CONTAINED-FIBEX-ELEMENT-REFS", "MINIMUM-TIMER-GRANULARITY", "SECTION-NAME-PREFIX", "DDS-RPC-SERVICES", "PATH-MTU-TIMEOUT", "TRANSFORMING-SYSTEM-SIGNAL-REF", "AP-APPLICATION-ERROR-REFS", "CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS", "COUPLING-PORT-RATE-POLICY", "IMPLIED-OPERATION-REF", "USED-FID-REF", "SERVER-ARGUMENT-IMPL-POLICY", "SOMEIP-REQUIRED-EVENT-GROUP", "MODE-USER-ERROR-BEHAVIOR", "MODE-TRANSITION", "RESTRICTIONS", "SIZE-OF-UNION-LENGTH-FIELDS", "REMOTE-PORT-REF", "CRC-VALIDATED", "MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "ALLOWED-I-PV-6-EXT-HEADERS", "PORT-API-OPTION", "PDELAY-REQUEST-PERIOD", "MODE-SWITCHED-ACK-EVENT", "TRANSMIT-CANCELLATION", "MEM-ALIGNMENT", "RECEIVER-REF", "USER-DEFINED-GLOBAL-TIME-MASTER", "TCP-TP", "CALL-TYPE", "ADAPTIVE-AUTOSAR-APPLICATION", "NETWORK-TIME-PROVIDER-REF", "SW-CALPRM-IMPL", "APPLICATIONS", "SUPPORTED-CONFIG-VARIANT", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL", "AGING-REQUIRES-TESTED-CYCLE", "ACCESS-COUNT", "SW-GENERIC-AXIS-PARAM", "ELEMENT-ROLE", "SECURITY-EVENT-PROPS-REF", "GENERAL-PURPOSE-CONNECTION", "FIRST-VALUE", "EXECUTION-ERROR", "DATA-LENGTH-REQUEST", "HEALTH-CHANNEL-ID", "PARAMETER-DATA-PROTOTYPE", "DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT", "NM-VOTE-INFORMATION", "SOCKET-CONNECTION-IPDU-IDENTIFIER", "METHOD-REFS", "PROGRAMMING-LANGUAGE", "ROLE", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "NM-COORD-SYNC-SUPPORT", "NM-COORDINATOR-ROLE", "DEFAULT-APPLICATION-TIMEOUT", "METHOD-REF", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "HW-ELEMENT-REFS", "ETH-TCP-IP-ICMP-PROPS", "SERVICE-INTERFACE-TREF", "TIMING-EVENT", "SENSOR-INSTANCE-ID", "AP-APPLICATION-ERROR-DOMAIN", "SERVICE-INTERFACE-REF", "USER-DEFINED-GLOBAL-TIME-SLAVE", "HEAP-USAGES", "DIAGNOSTIC-MEMORY-REF", "DIAGNOSTIC-INDICATOR-INTERFACE", "EVENT-CONTROLLED-TIMING", "ADD-INFO-5", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "READ-CLASS-REF", "CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF", "SESSION-CONTROL-CLASS-REF", "MAXIMUM-SEGMENT-LENGTH-REQUEST", "NOTIFIER-SIGNAL-TRIGGERING-REF", "ECU-STATE-MGR-USER-NEEDS", "P-PORT-PROTOTYPE-IREF", "SYNCHRONIZATION-POINT-CONSTRAINT", "UNIT-GROUP-REFS", "RULE-BASED-VALUES", "GENERIC-TP", "IS-PIPELINING-PERMITTED", "ELEMENTS", "LOCAL-VARIABLE-REF", "RPT-PROFILE", "TRANSFORMATION-PROPSS", "CONSISTENCY-NEEDS", "ECUC-MULTILINE-STRING-PARAM-DEF", "TCP-IP-NDP-DEFAULT-REACHABLE-TIME", "RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS", "USAGES", "PORT-BLUEPRINT-MAPPINGS", "SW-VALUES-PHYS", "INITIAL-CHECKPOINT-REF", "P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT", "SOURCE-MAX-ADDRESS", "TIMING-CLOCK-SYNC-ACCURACYS", "SW-CLUSTER-MAPPINGS", "LABEL", "MAPPING-CONSTRAINTS", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "SECOND-APPLICATION-ERROR-REF", "WAKEUP-LOCAL-ENABLED", "TRANSPORT-PDU-REF", "IDENT", "COUNTER-JUMP-UP", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS", "VLAN-MEMBERSHIPS", "CAPABILITY", "IS-STRUCT-WITH-OPTIONAL-ELEMENT", "COMMUNICATION-CLUSTER-REF", "IDS-COMMON-ELEMENT-REF-CONDITIONAL", "PER-INSTANCE-MEMORY-SIZES", "SEVERITY", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "LIN-UNCONDITIONAL-FRAME-REF", "PNC-MAPPING", "DISABLED-MODE-IREFS", "CONSTANT-MAPPING-REFS", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "PDU-ACTIVATION-ROUTING-GROUP", "INDEXED-ARRAY-ELEMENT", "EVENT-IREF", "CLOCK-REFERENCE-REF", "ON-FAIL-PERMISSIVE-MODE", "WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST", "OCCURRENCE-COUNTER-PROCESSING", "COM-CONFIGURATION-GW-TIME-BASE", "WRITE", "STORE-AT-SHUTDOWN", "SENDER-CAPABILITY", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "FIBEX-ELEMENTS", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "STATE-MANAGEMENT-REQUEST-ERROR", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "PERIODIC-RESPONSE-UUDT-REFS", "HANDLE-DATA-STATUS", "PDU-COUNTER-START-POSITION", "SUPERVISION-CHECKPOINT", "BLACKLISTED-VERSIONS", "UNIT-DISPLAY-NAME", "MINIMUM-TX-CONTAINER-QUEUE-SIZE", "BSW-INTERNAL-TRIGGERING-POINT", "INHIBITING-SECONDARY-FID-REF", "SYNC-JUMP-WIDTH", "WAKEUP-RX-IDLE", "COUPLING-PORT-SPEED", "AFFECTED-STATE-IREF", "INITIAL-REF-OFFSET", "RELATIVE-TOLERANCE", "SW-RECORD-LAYOUT-GROUP-AXIS", "NM-REPEAT-MESSAGE-TIME", "DYNAMIC-PARTS", "DATA-PROVIDER", "EXCLUSIVE-AREA-NESTING-ORDER-REFS", "SW-IMPL-POLICY", "DO-IP-ENTITY-ROLE", "ARGUMENT-DATA-PROTOTYPE", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "RPT-PREPARATION-LEVEL", "SIZE-OF-ARRAY-LENGTH-FIELD", "MESSAGE-SOURCE-FILE", "ECUC-ADD-INFO-PARAM-VALUE", "CHECKPOINT-TRANSITION-REF", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "N-ROM-BLOCKS", "DPG-DOES-NOT-REQUIRE-COHERENCYS", "I-PDU-MAPPING", "SW-VALUE-BLOCK-SIZE", "DIAGNOSTIC-READ-DTC-INFORMATION", "L-2", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "NM-PASSIVE-MODE-ENABLED", "MIN-NUMBER-OF-TIME-QUANTA-PER-BIT", "IPV-4-PROPS", "PERSISTENCY-DEPLOYMENT-REF", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REF", "WATCHDOG-PHM-ACTION-ITEM", "UPPER-HEADER-BITS-TO-SHIFT", "CONSTANT-MEMORYS", "FLEXRAY-FRAME-TRIGGERING", "SEGMENT-POSITION", "DEPLOYMENT-REF", "CONTENT-TYPE", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "IMPLICIT-DATA-ACCESS-IREFS", "DEPENDS-ON", "LISTEN-NOISE", "GLOBAL-TIME-ETH-SLAVE", "STOP", "TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS", "SUPPORTS-MULTIPLE-INSTANTIATION", "BYTE", "SO-CON-I-PDU-IDENTIFIER", "MIN-HEADER-ID", "SW-SERVICE-ARG", "FIRE-AND-FORGET-MAPPINGS", "I-SIGNAL-PORT-REF", "ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING", "NOTIFICATION-PORT-IREF", "APPLICATION-ENDPOINT", "OFFSET-SI-TO-UNIT", "EXECUTION-CONTEXT-REFS", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "DEADLINE-MAX", "TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR", "COUPLING-PORT-CONNECTION", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "APPLICATION-SW-COMPONENT-TYPE", "DISPLAY-PRESENTATION", "DETECT-NIT-ERROR", "TRIGGER-MAPPINGS", "SIZE-OF-ARRAY-LENGTH-FIELDS", "SUB-SDG-REF", "POST-BUILD-VARIANT-MULTIPLICITY", "CALIBRATION-PARAMETER-VALUE-SET-REF", "IE", "BLUEPRINT-POLICY-NOT-MODIFIABLE", "V-LAN-REFS", "SERVICE-INTERFACE-TRIGGER-MAPPING", "DIAGNOSTIC-ENABLE-CONDITION-REF", "MAXIMUM-RECEIVE-BUFFER-LENGTH", "DIAGNOSTIC-ECU-RESET-CLASS", "ETH-TP-CONFIG", "BLOCK-IF-STATE-ACTIVE-AP-IREF", "DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL", "NM-DATA-DISABLED", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "DISCOVERY-TYPE", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS", "USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT", "LOCAL-NETWORK-ENDPOINT-REF", "UDP-NM-CLUSTER-COUPLING", "SOURCE-NETWORK-MASK", "UNIT-REF", "CONTEXT-DATA-PROTOTYPE-IN-CS-REF", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "CONTAINED-FIBEX-ELEMENT-REF", "SOMEIP-TP-CONFIG", "WAKEUP-LOCAL-DETECTION-TIME", "TARGET-MODULE-REFS", "UNIT-REFS", "DYNAMIC-ARRAY-SIZE-PROFILE", "SW-COMP-TO-ECU-MAPPING-REF", "DLT-LOG-CHANNEL-REF", "TYPE-BLUEPRINTS", "CHECKPOINT-FINAL-REF", "MINIMUM-SEND-INTERVAL", "DEFAULT-REPORTING-MODE", "USED-CODE-GENERATOR", "TCP-IP-DHCP-V-6-INF-DELAY-MIN", "KEY-SLOT-ALLOWED-MODIFICATION", "DEF-CALPRM-SET", "FUP-DATA-ID-LISTS", "ALLOWED-I-PV-6-EXT-HEADERS-REF", "REQUESTER-IREF", "IUMPR-GROUP-IDENTIFIERS", "BSW-DIRECT-CALL-POINT", "RAM-BLOCK-INIT-VALUE", "PHYSICAL-DIMENSION-MAPPING", "PORT-PROTOTYPE-IREF", "EXPIRED-SUPERVISION-CYCLES-TOLERANCE", "CERTIFICATE-TO-KEY-SLOT-MAPPINGS", "SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING", "SWC-IMPLEMENTATION", "RTE-RESOURCE-ESTIMATION", "AUTO-AVAILABLE", "I-PDU-IDENTIFIER-TCP-REF", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "ESTIMATED-DURATION-OF-OPERATION", "PARAMETER-VALUES", "CAN-CLUSTER-VARIANTS", "ECUC-LINKER-SYMBOL-DEF", "CONNECTION-TYPE", "PRODUCT-ID", "CRYPTO-DRIVER", "UDP-NM-CLUSTER", "SOMEIP-TP-CHANNEL", "DERIVED-CERTIFICATE-ACCEPTED", "IS-DYNAMIC-LENGTH-FIELD-SIZE", "MAX-SOURCE-ELEMENT", "FUNCTION-GROUP-IREFS", "REFERENCE-VALUE-REF", "ALTERNATIVE-NAME", "TOLERANCE", "CHANNEL-REF", "REFINED-MODULE-DEF-REF", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "WAKEUP-REMOTE-ENABLED", "GENERAL-PURPOSE-PDU", "TARGET-RTE-EVENT-REF", "OPERATION-CYCLE-AUTOSTART", "PROVIDED-EVENT-GROUPS", "READER-ENTITY-ID", "SECOND-MODE-GROUP-REF", "REQUEST-EMISSION-RELATED-DTC-CLASS-REF", "CPP-TEMPLATE-ARGUMENT", "COMPONENT-REF", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "REQUIRED-CLIENT-SERVER-ENTRY-REF", "LOCAL-UNICAST-ADDRESSS", "SIGNAL-BASED-METHOD-DEPLOYMENT", "LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS", "NV-RAM-BLOCK-ELEMENT", "V-2-X-SUPPORTED", "RPT-SERVICE-POINT-POST-REFS", "HEADER-TYPE", "NO-SUPERVISION", "READ-DTC-INFORMATION-CLASS-REF", "PARAMETER-ID", "CHECKPOINT-REFS", "REQUIRED-MINOR-VERSION", "EXPECTED-CALLBACKS", "FIELD-DEPLOYMENTS", "FLAT-MAP-ENTRY-REF", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "TARGET-HEALTH-CHANNEL-REF", "EXPLICIT-INTER-RUNNABLE-VARIABLES", "SW-INTENDED-RESOLUTION", "TCP-PORT", "HW-PIN-GROUP-REFS", "LET-INTERVAL-REFS", "TIMEOUT-SUBSTITUTION-VALUE", "ROUGH-ESTIMATE-STACK-USAGE", "TCP-LOCAL-PORT", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "PWM-L", "EMULATION-SUPPORTS", "SD-CLIENT-EVENT-TIMING-CONFIG", "CLIENT-ID-DEFINITION-SET-REFS", "CONTROL-PNC-REFS", "SECURE-COMMUNICATION-DEPLOYMENTS", "SW-CARB-DOC", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "CONSUMED-EVENT-GROUP", "CRYPTO-SERVICE-JOB-NEEDS", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "KEY-EXCHANGE-AUTHENTICATION-REFS", "RTE-PLUGIN-PROPS", "DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF", "STATIC-SOCKET-CONNECTION-REF", "DIAGNOSTIC-PROPS", "TRIGGER", "ARTIFACT-DESCRIPTOR", "WAIT-POINTS", "SECURE-COMMUNICATION-PROPS-SET", "PNC-FILTER-ARRAY-MASKS", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "PERSISTENT-STORAGE", "ALIVE-TIMEOUT", "KEY-SERVER-PRIORITY", "APPLICATION-ENDPOINT-REF", "QOS-PROFILE", "BLUEPRINT-MAPPING-GUIDE", "INSTANCE-ID-PROPS", "MC-PARAMETER-INSTANCES", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "LOG-AND-TRACE-INTERFACE", "TRIGGER-PORT-ANNOTATION", "FIRST-MODE-REFS", "MODE-REF", "BLUEPRINT-MAPS", "REFERENCED-TD-EVENT-SWC-REF", "DETERMINISTIC-SYNC-MASTER", "TIME-SYNC-SERVER-IDENTIFIER", "COMPOSITE-NETWORK-REPRESENTATION", "IP-SEC-CONFIG-PROPS-REF", "REST-ENDPOINT-POST", "FRESHNESS-COUNTER-SYNC-ATTEMPTS", "ALIGNMENT", "TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE", "TIMING-DESCRIPTIONS", "SHORT-NAME", "PDU-POOLS", "MAJOR-VERSION", "OFFSET-CORRECTION-OUT", "ACTIVATION-POINTS", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF", "SOMEIP-FIELD", "SW-SYSTEMCONSTANT-VALUES", "FEATURE-MODEL-REFS", "FILE-STORAGE-REF", "MANUFACTURER-CODE", "SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "MAX-EVENTS-IN-INTERVAL", "ECU-PARTITION", "COLLECTION", "FOLLOW-UP-TIMEOUT-VALUE", "COMPU-METHOD-REF", "REMOTE-MULTICAST-CONFIG-REFS", "TARGET-EEC-REFS", "SWC-BEHAVIOR-REF", "POST-VERIFICATION-REBOOT", "TCP-IP-NDP-PREFIX-LIST-SIZE", "AUTHENTICATION-VERIFY-ATTEMPTS", "URL", "SW-SYSTEMCONSTANT-VALUE-SET-REF", "WRITER-ENTITY-ID", "REVISION-LABELS", "ATTRIBUTE-TAILORINGS", "RPT-SYSTEM-REF", "DIAGNOSTIC-RELEVANCE", "PRIMITIVE-TYPE-MAPPINGS", "REQUIRED-PORT-PROTOTYPE-IREF", "ROOT-ARGUMENT-DATA-PROTOTYPE-REF", "SPECIFICATION-DOCUMENT-SCOPE", "ACCESS-POINT-REF", "SW-AXIS-INDEX", "DIAGNOSTIC-INDICATOR", "SW-INTERPOLATION-METHOD", "TP-ECU", "I-PDU-IDENTIFIERS", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "WRITE-ACCESS", "IKE-AUTHENTICATION-METHOD", "IP-SEC-IAM-REMOTE-SUBJECT", "ACK-REQUEST", "OPERATION-CYCLE", "TIME-BR", "COMPONENT-QUALIFIER", "PROVIDED-DATAS", "PROCESS-STATE-IREF", "DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF", "SECURITY-EVENT-FILTER-CHAIN", "TARGET-PROVIDED-OPERATION-REF", "RTPS-PROTECTION-KIND", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "DIAGNOSTIC-INFO-TYPE", "TRIGGERS-RECOVERY-NOTIFICATION", "GLOBAL-TIME-MASTERS", "SSRC", "V-2-X-M-USER-NEEDS", "TRIGGER-TO-SIGNAL-MAPPING", "DISABLE-END-TO-END-STATE-MACHINE", "BSW-PARAMETER-POLICY", "IPV-6-ADDRESS-SOURCE", "PNC-MAPPINGS", "WAKEUP-RX-WINDOW", "REFERENCED-TD-EVENT-VFB-REF", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "SW-TEST-DESCS", "SW-MC-BASE-TYPE", "PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT", "CRC-SOURCE-PORT-IDENTITY", "SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "SUPERVISION-REF", "EOC-EXECUTABLE-ENTITY-REF", "LOGICAL-EXPRESSIONS", "CONTAINED-PDU-TRIGGERING-REFS", "SD-SERVER-EG-TIMING-CONFIGS", "ENABLE-DISCOVERY-PROTECTION", "MODE-ACCESS-POINT", "MULTICAST-SEGMENTATION", "MAX-NUMBER-OF-ELEMENTS", "TIME-CS", "PROCESS-TO-MACHINE-MAPPINGS", "GROUPED-ALIAS-EVENT-REFS", "IMPL-CONSTANT-REF", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "TABLE", "META-DATA-ITEMS", "INHIBITION-MASK", "NUMBER-OF-REPETITIONS", "SECURITY-EVENT-REPORT-INTERFACE", "JITTER", "COMPUTED", "PORT-INSTANTIATION-BEHAVIOR", "TFOOT", "RETURN-SIGNAL-REF", "EVENT-GROUP-CONTROL-TYPE", "STORING", "CHECK-PER-SOURCE-ID", "TRANSPORT-LAYER-RULE", "DLT-ARGUMENT-ENTRYS", "NM-DATA-ENABLED", "DEFAULT-ENDIANNESS", "TLV-ARGUMENT-REF", "MATCHING-RULE-REF", "SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS", "NOTIFICATION-STATE", "LOCAL-ID", "AUTOSAR-VARIABLE", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "TD-EVENT-TRIGGER-TYPE", "ROLE-BASED-MC-DATA-ASSIGNMENT", "SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL", "COLSPEC", "GLOBAL-TIME-FR-MASTER", "KEEP-ALIVE-TIME", "END-TO-END-PROTECTION-SET", "MIN-TRCV-DELAY-COMPENSATION-OFFSET", "DENOMINATOR-CONDITION", "EXECUTION-CONTEXT-REF", "REF-MEASUREMENT-SET", "ENTITY-STATUS-MAX-BYTE-FIELD-USE", "CONSIDER-PTO-STATUS", "STD", "ACCURACY-EXT", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "LOCAL-IP-SEC-RULE-REF", "MODIFICATIONS", "FUP-DATA-ID-LIST", "PROVIDED-SERVICE-INSTANCE-REF", "DLT-APPLICATION-TO-PROCESS-MAPPING", "SECURE-COMMUNICATION-PROPS", "SETTER-REF", "MASS-EXP", "MAX-BUFFER-REQUEST", "NOT-AVAILABLE-VALUE-SPECIFICATION", "PORT-BLUEPRINT", "EVALUATED-VARIANT-REFS", "R-PORT-PROTOTYPE-IREF", "I-PDU-SIGNAL-PROCESSING", "MODE-PORT-ANNOTATIONS", "IDENTIFIER", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "INDICATOR-FAILURE-CYCLE-COUNTER-THRESHOLD", "TYPE-REFERENCE-REF", "LOG-TRACE-DEFAULT-LOG-LEVEL", "IP-V-6-ADDRESS", "MINIMUM-FIFO-LENGTH", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "MIN-SYNC-JUMP-WIDTH", "HW-ELEMENT-CONNECTIONS", "SW-FEATURE-DESC", "SERVICE-INTERFACE-MAPPING", "E-2-E-PROFILE-CONFIGURATION", "SOMEIP-EVENT-GROUP", "ASSIGNED-DATA-TYPES", "PNC-CLUSTER-VECTOR-LENGTH", "PACKAGE-REF", "IPV-6-ADDRESS", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "DIAGNOSTIC-CONNECTION", "R-PORT-PROTOTYPE-PROPS", "ACL-ROLE", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "OBJECT-DEFINITION-REF", "PDU-TO-FRAME-MAPPING", "SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "COMM-CONTROLLER-MAPPINGS", "MAC-SEC-PROPSS", "INSTANCE-IDS", "TIMING-RESOURCE", "PORT-DEFINED-ARGUMENT-BLUEPRINT", "CONCRETE-PATTERN-EVENT-TRIGGERING", "RATE-CORRECTION-OUT", "IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "ECUC-NUMERICAL-PARAM-VALUE", "BLUEPRINT-MAPPING-SET", "IDLE-SLOPE", "ELEMENT-IN-IMPL-DATATYPE", "MODULE-ID", "UNICAST-NETWORK-ENDPOINT-REFS", "TRANSMISSION-TRIGGER", "INHIBIT-SOURCES", "CRYPTO-JOB-REF", "CLIENT-SERVER-INTERFACE", "DTC-KIND", "SWC-MODE-GROUP-IREF", "TCP-TP-CONFIG", "J-1939-SHARED-ADDRESS-CLUSTER", "TP-CONFIGURATION", "NETWORK-INTERFACE-REFS", "IDSM-INSTANCE-REF-CONDITIONAL", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF", "DEADLINE-SUPERVISION", "NON-OS-MODULE-INSTANTIATION-REF", "FLEXRAY-AR-TP-CHANNEL", "SUPPORTS-MULTIPLE-NOTIFIER-SETS", "INDUSTRY-GROUP", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "ENUMS", "DIAGNOSTIC-FIM-EVENT-GROUP", "PORT-PROTOTYPE-BLUEPRINT-REF", "AUTH-DATA-FRESHNESS-START-POSITION", "TARGET-P-PORT-PROTOTYPE-REF", "USE-FRESHNESS-TIMESTAMP", "TRANSFORMATION-COM-SPEC-PROPSS", "NEXT-HEADER", "TCP-IP-NDP-MAX-RANDOM-FACTOR", "RID-NUMBER", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "CLIENT-PORT-REF", "ECU-EXTRACT-REFERENCE-IREF", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF", "VERIFICATION-METHOD", "ECUC-FUNCTION-NAME-DEF-CONDITIONAL", "PROVIDED-PORT-PROTOTYPE-IREF", "COMMUNICATION-CONNECTORS", "PACKING-BYTE-ORDER", "INDICATOR-REF", "MAJOR-PROTOCOL-VERSION", "APPLICATION-DEFERRED-DATA-TYPE", "LOGICAL-EXPRESSION-ARGUMENT-REFS", "GENERATOR", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF", "SEPARATE-SIGNAL-PATH", "TEMPLATE-ARGUMENTS", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "AFFECTED-REFS", "RESPONSE-ON-EVENT-ACTION", "COMPU-SCALES", "MAX-NUMBER-OF-CONNECTIONS", "METHOD-CALL-REF", "GROUP-NUMBER", "TLV-DATA-ID-0-REFS", "TCP-IP-IP-REASS-TIMEOUT", "DIAGNOSTIC-CONTRIBUTION-SET", "DYNAMIC-LENGTH", "RUNS-INSIDE-EXCLUSIVE-AREA-REF", "CLIENT-INTENT", "ABSOLUTELY-SCHEDULED-TIMINGS", "NUMERICAL-VALUE-VARIATION-POINT", "USE-SMART-SENSOR-API", "INTERFACE-MAPPING-SET", "BSW-SERVICE-DEPENDENCY-REF", "MKA-PARTICIPANT-REFS", "FILTER-CHAINS", "RP-IMPL-POLICY", "VEHICLE-ANNOUNCEMENT-INTERVAL", "C-CODE", "BLOCKLISTED-VERSIONS", "PNC-NM-REQUEST", "MC-DATA-INSTANCE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "NM-WAIT-BUS-SLEEP-TIME", "REVERSED-TP-SDU-REF", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF", "CYCLIC-TIMING", "PARENT-CATEGORY", "SECOND-ELEMENT-REF", "FUNCTION-INHIBITION-NEEDS", "MAC-MULTICAST-CONFIGURATION", "PERMISSION-CERTIFICATE-AUTHORITY-REF", "PRE-ACTIVATE-REF", "MAPPINGS", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "ACCEPTED-STARTUP-RANGE", "DIAGNOSTIC-VALUE-ACCESS", "DDS-SECURE-COM-PROPS", "STORAGE-CONDITIONS", "SERVICE-ID", "MODE-SWITCH-SENDER-COM-SPEC", "BSW-DATA-SEND-POLICY", "BSW-SCHEDULABLE-ENTITY", "I-SIGNAL-I-PDU-GROUP", "RELEASED-TRIGGER-POLICYS", "MICROTICK-DURATION", "DESTINATION-MAC-ADDRESS", "MAXIMUM-KEY-LENGTH", "CONTAINERS", "EXECUTABLE-LOGGING-IMPLEMENTATION-PROPS", "MANAGED-PHYSICAL-CHANNEL-REF", "START-ACTION-REFS", "IP-ADDRESS-KEEP-BEHAVIOR", "COMMUNICATION-CONTROLLERS", "SERVER-CALL-POINTS", "BASE-REF", "DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL", "DOMAIN", "ETHERNET-RAW-DATA-STREAM-MAPPING", "TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "IDSM-INSTANCE-REF", "SOURCE-SERVICE-INTERFACE-REFS", "PREVIOUS-EXECUTABLE-VERSION", "LOWER-BOUND", "METHOD-MAPPING", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "TEXT-TABLE-MAPPING", "ECU-INSTANCE-REF-CONDITIONAL", "RECEPTION-PROPS", "LITERALS", "ROUTING-GROUP-REFS", "SECURE-COM-PROPS-REF", "APPLICATION-ERROR", "PHM-RULE", "LIN-TP-CONFIG", "SOFTWARE-PACKAGE-STORING", "BSW-QUEUED-DATA-RECEPTION-POLICY", "NM-NETWORK-HANDLE", "STATE-DEPENDENT-FIREWALL", "DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL", "MC-DATA-ASSIGNMENTS", "CONFIGURATION-CLASS-AFFECTION", "ACCESS-COUNT-SET", "ETHERNET-FRAME", "TIME-SUB-TLV", "VERIFY-IREFS", "SW-DATA-DEPENDENCY-ARGS", "OFFSET-CORRECTION-JUMP-THRESHOLD", "INIT-VALUES", "HOP-LIMIT", "APPLICATION-DATA-TYPE-REF", "SDG-CLASS", "CAN-TP-CONNECTION", "ECUC-DESTINATION-URI-DEF", "TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE", "CSE-CODE-FACTOR", "VENDOR-ID", "PUBLISHER", "TARGET-DATA-PROTOTYPE-REF", "SAFETY-MARGIN", "UCM-MASTER-MODULE-INSTANTIATION", "ECU-SCOPE-REF", "SOMEIP-FIELD-DEPLOYMENT", "COMPOSITE-RULE-BASED-VALUE-SPECIFICATION", "SCHEDULER-NAME-PREFIXS", "TRANSLATION-TARGET-IREF", "NOTIFICATION-TIME", "LIN-SLAVE-VARIANTS", "SYSC-REF", "SETTER-OPERATION-REF", "SOURCE-ELEMENT-REF", "BSW-DEBUG-INFO", "LIN-ERROR-RESPONSE", "WAIT-POINT", "CLIENT-ID-DEFINITION-SET-REF", "FORMAL-BLUEPRINT-GENERATOR", "RETRY-INTERVAL-TIME", "IP-ADDRESS", "PDU-COLLECTION-TIMEOUT", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "TCP-IP-AUTO-IP-INIT-TIMEOUT", "SOURCE-MAC-ADDRESS-MASK", "TYPE-EMITTER", "EXTERN-RATE-CORRECTION", "MODE-IN-BSW-INSTANCE-REF", "SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF", "SLAVE-REF", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "STATIC-SOCKET-CONNECTION", "DECODING-CORRECTION", "DDS-FIELD-QOS-PROPS", "MICRO-INITIAL-OFFSET-A", "RX-PDU-POOL-REF", "R-PORT-PROTOTYPE-REF", "PARAMETER-DATA-PROTOTYPE-IREF", "CAN-CONTROLLER-XL-ATTRIBUTES", "TCP-SYN-MAX-RTX", "BASE-CYCLE", "INITIALIZED-PARAMETER-REF", "COMPU-METHOD", "LIN-MASTER-VARIANTS", "DO-IP-LOGIC-ADDRESS", "DLT-MESSAGE-REF-CONDITIONAL", "REMOTING-TECHNOLOGY", "MACHINE-MODE-IREF", "EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE", "COMPONENT-IREFS", "MODE-INSTANCE", "BEST-CASE-EXECUTION-TIME", "IMPLEMENTATION-CONFIG-CLASSES", "LOG-TRACE-FILE-PATH", "ACL-OBJECT-CLASSS", "MC-FUNCTION", "REQUIRED-INTERFACE-TREF", "SW-CALPRM-PROTOTYPE-REF", "ENHANCED-MODE-API", "POST-BUILD-VARIANT-USED", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "RECOVERY-NOTIFICATION-TIMEOUT", "REST-INTEGER-PROPERTY-DEF", "PORT-REF", "APPLICATION-ENDPOINT-REF-CONDITIONAL", "DYNAMIC-PART", "METHOD-QOS-PROPSS", "GLOBAL-TIME-CAN-MASTER", "REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF", "DLT-MESSAGE-REF", "BSW-TRIGGER-DIRECT-IMPLEMENTATION", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "DIAGNOSTIC-ROUTINE-NEEDS", "DEST-SDG-REF", "LUMINOUS-INTENSITY-EXP", "DATA-RECEIVED-EVENT", "VERIFY-REF", "SW-MAINTENANCE-NOTESS", "SUB-ELEMENT-MAPPINGS", "ENDPOINTS", "CAN-TP-ECU", "SIGNATURE-SCHEME-REF", "SOCKET-ADDRESS-REF", "IDS-DESIGN", "MC-GROUP-DATA-REF-SET-CONDITIONAL", "PORT-GROUP", "SUBTITLE", "DOIP-CHANNEL-COLLECTION-REF", "HOST-ID", "MC-FUNCTION-DATA-REF-SET-CONDITIONAL", "SW-CLUSTERS", "NUMERICAL-VALUE", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "DELAY", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "PURPOSE-OF-UPDATE-REF", "ASSUMED-CURRENT-STATE-IREF", "POLICY", "LIN-CLUSTER", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "RX-TIMEOUT-TIME", "TOPIC-1", "CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST", "PROCESS-TO-MACHINE-MAPPING-SET", "STORE-CYCLIC", "MINISLOT-DURATION", "TRANSMISSION-START-SEQUENCE-DURATION", "MODE-TRANSITIONS", "CAN-XL-PROPS-REFS", "DYNAMIC-PART-ALTERNATIVE", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "PDU-COUNTERS", "COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL", "NODE-PORTS", "DISABLED-IN-MODE-IREF", "PAYLOAD-BYTE-PATTERN-RULES", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "SYNC-INTERVAL", "MAXIMUM-MESSAGE-LENGTH", "USER-DEFINED-TRANSFORMATION-DESCRIPTION", "POST-BUILD-VARIANT-SUPPORT", "NV-DATAS", "STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE", "PRE-ACTIVATE-IREFS", "CRYPTO-JOBS", "NM-HANDLE-REF", "RULE-INIT-STATE", "REQUESTER-REF", "J-1939-DCM-DM-19-SUPPORT", "GLOBAL-IN-PACKAGE-REF", "ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL", "REST-ELEMENT-DEF", "DATA-TYPE-POLICY", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "CONTEXT-PORT-REF", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "SUPERVISION-ENTITY", "TIME-VALUE-VALUE-VARIATION-POINT", "SIGNAL-I-PDU-REPLICATION", "DIAGNOSTIC-J-1939-NODE", "LIFE-CYCLE-INFOS", "CONTROL-OPTION-RECORD-PRESENT", "EXECUTION-ORDER-CONSTRAINT", "IS-VARIADIC-TEMPLATE", "TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED", "NM-MSG-REDUCED-TIME", "CALL-SIGNAL-TRIGGERING-REF", "MASTER-TO-SLAVE-EVENT-MAPPINGS", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF", "COUPLED-CLUSTER-REFS", "MAXIMUM-INTENDED-BINDING-TIME", "CURRENT-EXP", "CP-SOFTWARE-CLUSTER", "PARTITIONS", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "MAX-DATA-LENGTH", "CONTAINED-PACKAGE-ELEMENT-REFS", "KEEP-ALIVE-PROBES-MAX", "ALIVE-CHECK-RESPONSE-TIMEOUT", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "SW-TEXT-PROPS", "LANGUAGE", "ADDRESS-RANGE-LOWER-BOUND", "RAM-LOCATION-REF", "OPERATION-REF", "SUB-FUNCTION-REFS", "DEFERRING-FID-REF", "NM-IF-ECUS", "MINIMUM-SUSTAINED-SIZE", "PACKAGER-SIGNATURE-REF", "PNC-PDUR-GROUP-REF", "SECOND-PORT-REF", "ECUC-QUERY-REF", "K-RUN-RESOURCE", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "DYNAMIC-PART-ALTERNATIVES", "SCHEDULE-TABLES", "FM-FEATURE-MAP-CONDITION", "CAN-CONTROLLER-ATTRIBUTES", "IN-PLACE", "FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING", "CONDITION", "MAXIMUM-TRANSMIT-BUFFER-LENGTH", "NOTATION", "SW-AXIS-CONT", "CORES", "DATA-RECEIVE-POINT-BY-ARGUMENTS", "TCP-IP-DHCP-V-6-SOL-DELAY-MAX", "INPUT-DATAS", "DEST-META-CLASS", "DATA-ELEMENT-IREF", "PARAMETER-POLICYS", "PRE-ACTIVATE-REFS", "LOWER-LIMIT", "RUNNABLE-ENTITY-IREFS", "DEVELOPMENT-VALUE", "DEPENDENT-SOFTWARE-CLUSTER-REF", "MAX-BLOCK-SIZE", "EVENT-WINDOW-TIME", "DIAGNOSTIC-CLEAR-CONDITION", "EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS", "COM-FIND-SERVICE-GRANT", "DATA-READ-ACCESSS", "TRANSFER-REFS", "ROLE-BASED-PORT-ASSIGNMENT", "PER-STATE-TIMEOUTS", "FLEXRAY-TP-CONNECTION", "EXCLUSIVE-AREA-NESTING-ORDER-REF", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "START", "CHAPTERS", "MAC-MULTICAST-GROUPS", "NEXT-STATE-IREF", "FLAT-INSTANCE-DESCRIPTOR", "LIN-SLAVE-ECU-REF", "COUNTER-PASSED-THRESHOLD", "IPV-6-DHCP-SERVER-CONFIGURATION", "CRC-PRECISE-ORIGIN-TIMESTAMP", "EID-USE-MAC", "DIAGNOSTIC-DEPLOYMENT-PROPS-REF", "PID-REF", "VARIABLE-ACCESS-IREFS", "MEMORY-ENTRY-STORAGE-TRIGGER", "NONQUEUED-RECEIVER-COM-SPEC", "DPD-ACTION", "NEXT-HIGHER-CERTIFICATE-REF", "CONTEXT-MODE-DECLARATION-GROUP-REF", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "MODULE-INSTANTIATION-REFS", "REVISION-LABEL", "IPV-4-MULTICAST-IP-ADDRESS", "MAX-NUMBER-OF-OCCURRENCES", "STARTUP-CONFIG-REF", "MSR-QUERY-RESULT-P-2", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "SUPERVISION-CYCLE", "DEFAULT-PATTERN", "PATH", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "KEY-SLOT-ONLY-ENABLED", "COMMUNICATION-RESOURCE-PROPS", "GOVERNANCE-REF", "RAW-DATA-STREAM-DEPLOYMENT", "SEPARATION-CYCLE-EXPONENT", "LOCAL-TCP-PORT-REF", "UCM-STEP", "ROLE-BASED-DATA-ASSIGNMENT", "ECU-REF", "DELEGATED-PORT-ANNOTATION", "REMOTE-NETWORK-ENDPOINT-REF", "COUPLING-PORT-FIFO", "APPLIES-TO", "CAN-COMMUNICATION-CONTROLLER", "DESTINATION-URI-REF", "ON-BOARD-MONITOR-ID", "PARAMETER-ACCESSS", "SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "FREEZE-CURRENT-STATE", "RUNNABLE-ENTITY", "FR-GLOBAL-TIME-DOMAIN-PROPS", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "TARGET-I-PDU", "SW-COMPOSITIONS", "REST-ENDPOINT-DELETE", "DIAGNOSTIC-AGING", "AUXILIARY-FIELDS", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "PERSISTENCY-CENTRAL-STORAGE-URI", "IDSM-INSTANCE", "EVALUATED-VARIANT-SET", "REQUIRED-AR-ELEMENT-REFS", "TD-EVENT-FRAME", "INHIBITION-SOURCE-REF", "CYCLIC-WRITING-PERIOD", "DATA-TYPE-REF", "CONFLICTS-TO", "APPLICATION-CONTEXT-REFS", "EXECUTION-TIMES", "EVENT-GROUPS", "CONFIGURATION-HANDLING", "TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS", "MODE-REPORTER-PORT-IREF", "DO-IP-TESTER-ROUTING-ACTIVATION-REF", "MODE-RECEIVER-POLICYS", "FM-FEATURE-MAP-ASSERTION", "DOIP-CONNECTION-REFS", "ALL-CHANNELS-REF", "NESTED-ELEMENTS", "BOR-COUNTER-L-1-TO-L-2", "USER-DEFINED-PHYSICAL-CHANNEL", "SIGNIFICANCE", "DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT", "OFFSET-TIME-DOMAIN-REF", "ALLOWED-I-PV-6-EXT-HEADER", "SPECIFIC-CHANNEL-REF", "LOWER-REF", "TP-ECUS", "SOMEIP-TRANSFORMATION-DESCRIPTION", "NM-DATA-INFORMATION", "NM-BUS-LOAD-REDUCTION-ENABLED", "CONTROLLED-HW-ELEMENT-REF", "NM-PDU-REF-CONDITIONAL", "NM-PDUS", "COUPLING-PORT-DETAILS", "SW-VARIABLE-IMPL", "CLASS-CONTENT-CONDITIONAL", "NM-NODES", "PROVIDED-MODE-GROUP-REF", "DEVELOPMENT-ERROR", "SUB-TLV-CONFIG", "EXECUTE-DESPITE-DATA-UNAVAILABILITY", "NM-SHUTDOWN-DELAY-TIMER", "NM-VOTING-CYCLE", "PNC-WAKEUP-CAN-ID", "METHOD-REPLY-TOPIC-NAME", "SENDING-NODE-REFS", "X", "QUEUE-LENGTH", "DATA-CONSTR", "MULTICAST-CREDENTIALS", "SECOND-PHYSICAL-DIMENSION-REF", "SHARED-TIMER", "FLOAT", "MINIMUM-DELAY", "SOMEIP-METHOD-PROPS", "FACTOR-SI-TO-UNIT", "PERSISTENCY-PORT-PROTOTYPE-IREF", "DO-IP-REQUEST-CONFIGURATION", "OFFSET-CORRECTION-START", "NM-USER-DATA-LENGTH", "STATIC-SLOT-DURATION", "FLOW-CONTROL-REF", "OBJECT-DEFINITION-REFS", "SECURITY-EVENT-DEFINITION", "PNC-GATEWAY-TYPE", "HW-CATEGORY-REFS", "TX-ENABLE-WINDOW-LENGTH", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "MSR-QUERY-P-1", "GETTER-CALL-SIGNAL-REF", "ASSIGNED-ENTRY-REF", "TD-PDU-TRIGGERING-FILTER-REFS", "IMPLEMENTED-ENTRYS", "CAN-FRAME", "MODE-SWITCH-INTERFACE", "MEMORY-SECTION", "ROOT-SW-COMPONENT-PROTOTYPE", "SERVICE-RESOURCE-REF", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "CONSTRAINT-TAILORINGS", "USER-DEFINED-CLUSTER-CONDITIONAL", "RECOVERY-VIA-APPLICATION-ACTION", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "SW-CALIBRATION-NOTESS", "VF", "INTERFACE-BLUEPRINT-REF", "DIAGNOSTIC-ENV-MODE-CONDITION", "FRESHNESS-PROPS-REF", "PREDECESSOR-ACTION-REFS", "DIAGNOSTIC-DATA-CHANGE-TRIGGER", "INPUT-VARIABLE-TYPE-REF", "DATA-ELEMENTS", "AP-APPLICATION-ENDPOINT", "MAX-SOURCE-PORT-NUMBER", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "TARGET-PHM-SUPERVISED-ENTITY-IREF", "FIRST-OPERATION-REF", "FUNCTION-GROUP-STATE-IREFS", "SD-CLIENT-CONFIG", "IMPLEMENTATION-DATA-TYPE-REF", "TRAFFIC-CLASS", "ARTIFACT-CHECKSUMS", "RESUME-POSITION", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "REQUEST-2-SUPPORT", "MACRO-PER-CYCLE", "ASSEMBLY-SW-CONNECTOR", "DLT-APPLICATION", "SERVICE-INTERFACE-ID", "NUMBER-OF-NOTIFIER-SETS", "AR-PACKAGES", "COUPLING-TYPE", "BSW-CLIENT-POLICY", "J-1939-TP-PG", "DESTINATION-NETWORK-MASK", "MODULE-DESCRIPTION-REF", "SOURCE-SIGNAL-REF", "FUNCTION-GROUPS", "TABLE-ENTRYS", "PARAMETER-PROVIDE-COM-SPEC", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "INVERT", "TD-EVENT-BSW-MODULE", "ADDRESS-CLAIM-ENABLED", "HEALTH-CHANNEL-IREF", "ATTRIBUTE-DEFS", "CONTEXT-COMPONENT-REF", "IMPLICIT-DATA-ACCESS-IREF", "ASSIGNED-FRAME-TRIGGERING-REF", "COM-MANAGEMENT-MAPPING", "AUTO-REQUIRE", "TP-CONNECTION-CONTROLS", "REQUIRED-SERVICE-INSTANCE-REF", "SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME", "DIAGNOSTIC-ENV-DATA-ELEMENT-CONDITION", "STACK-USAGES", "USER-DEFINED-COMMUNICATION-CONTROLLER", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "FLEXRAY-CLUSTER", "ELLIPTIC-CURVE-REF", "ORDERED-MASTER-LIST", "DESIGN-REFS", "DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF", "RUNS-INSIDE-EXCLUSIVE-AREA-REFS", "TIMING-REQUIREMENTS", "TLS-CRYPTO-CIPHER-SUITE", "ECUC-PARAM-CONF-CONTAINER-DEF", "CHECKPOINT-FINAL-REFS", "INSTANCES", "PER-INSTANCE-MEMORYS", "SYSTEM-VERSION", "INVALIDATION-POLICY", "TIME-BUFFER", "SERVICE-ELEMENT-SECURE-COM-CONFIG-REF", "DESTINATION-URI-NESTING-CONTRACT", "SO-AD-ROUTING-GROUP", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "MAX-BUFFER-SIZE", "LIMIT", "DEPENDS-ONS", "SW-CALPRM-AXIS", "INTEGER-VALUE-VARIATION-POINT", "CHECKPOINTS", "N", "PROFILE-NAME", "MINOR-VERSION", "ECUC-INTEGER-PARAM-DEF", "ECUC-VALIDATION-CONDS", "DIAGNOSTIC-COMMON-PROPS-CONDITIONAL", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "TD-EVENT-SERVICE-INSTANCE-FIELD", "CAPTION", "FLEXRAY-NM-ECU", "COM-MANAGEMENT-GROUP-REFS", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING", "SOMEIP-TRANSFORMATION-PROPS", "DYNAMIC-SLOT-IDLE-PHASE", "MAX-FR-IF", "CIPHER-SUITE-NAME", "CRYPTO-PROVIDER", "NUMBER-OF-PARALLEL-TCP-SESSIONS", "PDU-TRIGGERING-REF-CONDITIONAL", "EXECUTION-TIME-CONSTRAINT", "TRANSMIT-PDU-REF", "VALIDATION-FORMULA", "SOCKET-OPTIONS", "POSSIBLE-ERROR-REACTION", "EXPLICIT-RPT-PROFILE-SELECTION-REF", "BY-PASS-POINT-IREF", "NM-PDU-REF", "PATTERN", "PERSISTENCY-FILE-PROXY-INTERFACE", "SERVICE-INTERFACE-DEPLOYMENT-REF", "PN-RESET-TIME", "DEFAULT-PERIOD-END", "TGROUP", "MAX-DEADLINE", "CONTEXT-P-PORT-REF", "SW-FEATURE-DESCS", "ECUC-STRING-PARAM-DEF-VARIANTS", "PREEMPTABILITY", "REQUIRED-DATAS", "CALL-SIGNAL-REF", "INNER-GROUP-IREF", "FIELD-NOTIFIER-REF", "CONSTANT-SPECIFICATION-MAPPING-SET", "EXPRESSION-REF", "EXECUTION-DEPENDENCYS", "DESIGN-REF", "INTERFACE-MAPPINGS", "COMPU-PHYS-TO-INTERNAL", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "AFFECTION-KIND", "BASE-REFERENCE-REF", "PRMS", "TARGET-REQUIRED-OPERATION-REF", "SOCKET-CONNECTION", "EXPECTED-ENTRYS", "CYCLIC-BACKUP-INTERVAL", "LIN-CHECKSUM", "MEMORY-DESTINATION-REF", "FIREWALL-RULE", "ECU-CONFIGURATION-REF", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "MIN-OK-STATE-VALID", "MIN-MAJOR-VERSION", "TRUE-ACTION-LIST-REF", "CAN-BAUDRATE", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "AUTHENTICATION-PROPS-REF", "VARIABLE-ACCESSED-FOR-DEBUG-REFS", "APPLICATION-MODE-IREF", "PROPAGATION-DELAY", "DIAGNOSTIC-J-1939-SPN-MAPPING", "IN-SCOPE", "REQUIRED-MODE-GROUP-REF", "NESTED-ELEMENT-REF", "FLEXRAY-AR-TP-NODE", "REQUEST-DATA-SIZE", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "UDS-DTC-VALUE", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "REF-CALPRM-SET", "REQUIRED-TRIGGER-IREF", "CAS-RX-LOW-MAX", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "REG-DOES-NOT-REQUIRE-STABILITYS", "MAX-MINOR-VERSION", "SIGNAL-BASED-EVENT-REF", "CYCLE-REPETITION", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "INIT-VALUE", "REQUESTED-MODE-IREF", "DATA-ID-NIBBLE-OFFSET", "DLT-LOG-CHANNEL-DESIGN", "BSW-MODE-SENDER-POLICY", "BGCOLOR", "RESOURCE-GROUP-REFS", "PHYSICAL-DIMENSION-MAPPINGS", "CONNECTION-PRIO-PDUS", "ETH-IP-PROPS-REFS", "VT", "DRIVER-PRIMITIVE-REF", "ECU-INSTANCE-REF", "TCP-RETRANSMISSION-TIMEOUT", "CAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "BSW-MGR-NEEDS", "SECURE-COM-PROPS-FOR-DDS-REF", "DISABLE-END-TO-END-CHECK", "DLT-MESSAGE-REFS", "SIGNAL-PATH-CONSTRAINTS", "PSK-IDENTITY", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "DIAGNOSTIC-ADDRESSS", "TARGET-PARAMETER-DATA-PROTOTYPE-REF", "SUCCESSOR-REFS", "PORT-BLUEPRINT-MAPPING", "ASSIGNED-PORTS", "MEASURED-STACK-USAGE", "SYNC-PERIOD", "PHM-RECOVERY-ACTION-INTERFACE", "COUNT-PROFILE", "TT-CAN-CLUSTER-REF", "LOCAL-PORT-RANGE-END", "PLCA-MAX-BURST-COUNT", "ENVIRONMENT-DATA-CAPTURE", "MODE-DRIVEN-FALSE-CONDITIONS", "SW-ADDRMETHOD-REF", "ROUTING-ACTIVATION-TYPE", "BOR-TIME-L-2", "DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT", "BULK-NV-DATA-DESCRIPTORS", "BEHAVIOR-REF", "GLOBAL-KAY-PROPS-REF", "IPV-4-ADDRESS-SOURCE", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "IPV-6-MULTICAST-IP-ADDRESS", "UNICAST-SECURE-COM-PROPS-REF", "DIAGNOSTIC-VALUE-NEEDS", "DELTA-PACKAGE-APPLICABLE-VERSION", "RUNS-INSIDES", "TRANSFER-EXIT-CLASS-REF", "RUN-MODE", "SYNCHRONIZATION-TIMING-CONSTRAINT", "BSW-RESOLUTION", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "PRIORITYS", "TRIGGER-I-PDU-SEND-CONDITION", "FIELD-ELEMENTS", "APPLICATION-PARTITION-REFS", "CUSTOM-SDG-DEF-REF", "NETWORK-ENDPOINT-ADDRESSES", "DYNAMIC-ACTION-REFS", "REST-STRING-PROPERTY-DEF", "TRANCEIVER-STANDBY-DELAY", "ARBITRARY-ADDRESS-CAPABLE", "BSW-MODE-RECEIVER-POLICY", "SO-CON-I-PDU-IDENTIFIER-REF", "INTERRUPT-SOURCE", "DIAG-FAULT-DETECTION-COUNTER-PORT-REF", "ADMIN-DATA", "DIAGNOSTIC-SESSION", "ADDRESS-RANGE-UPPER-BOUND", "PARTICIPATING-J-1939-CLUSTER-REF", "META-DATA-LENGTH", "PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS", "MAX-SLOTS", "SW-ARRAYSIZE", "ACTIVATION-SWITCH", "DLT-LOG-CHANNEL", "PER-STATE-TIMEOUT", "CRYPTO-SERVICE-NEEDS", "CYCLE-AUTOSTART", "SD-CLIENT-TIMER-CONFIGS", "EVENT-ELEMENTS", "ECUC-FUNCTION-NAME-DEF-VARIANTS", "LIN-PID-TO-CAN-ID-MAPPINGS", "RECOVERY-ACTION-IREF", "SW-DIAGNOSTICS-NOTESS", "KEY-SLOT-ID", "DATA-PROTOTYPE-GROUP-IREF", "BSW-EXCLUSIVE-AREA-POLICY", "I-SIGNAL-PROPS", "ALIVE-REFERENCE-CYCLE", "ROUTINE-REF", "PHYSICAL-CHANNEL-REF", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING", "USED-IMPLEMENTATION-DATA-TYPE-REF", "NM-NETWORK-TIMEOUT", "VEHICLE-IDENTIFICATION-SYNC-STATUS", "ELEMENT-POSITION", "MULTIPLE-CONFIGURATION-CONTAINER", "DEADLINE-SUPERVISIONS", "TLV-IMPL-RECORD-ELEMENT", "CONSTANT-REFERENCE", "WORST-CASE-HEAP-USAGE", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "MAX-REQUEST-BYTES", "USE-CLIENT-AUTHENTICATION-REQUEST", "DEBOUNCE-ALGORITHM-REF", "ETH-SWITCH-PORT-GROUP-DERIVATION", "TCP-KEEP-ALIVE-ENABLED", "STARTS-ON-EVENT-REF", "OFFER-CYCLIC-DELAY", "BUILD-ACTION-MANIFEST-REF", "TLS-CONNECTION-GROUP", "LOWER-MULTIPLICITY", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "METHOD-REQUEST-TOPIC-NAME", "DATA-EXCHANGE-POINT", "TRIGGER-I-PDU-SEND-CONDITIONS", "PAYLOAD-REF", "SW-DATA-DEF-PROPS-VARIANTS", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "TCP-IP-DHCP-V-6-CNF-DELAY-MAX", "IS-EVENT", "AH-CIPHER-SUITE-NAMES", "SERVICE-DISCOVERY-CONFIGS", "GLOBAL-TIME-FR-SLAVE", "PAYLOAD-LENGTH-STATIC", "BSW-MODULE-INSTANCE-REF", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPES", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "SOFTWARE-CLUSTER-REQUIREMENT", "STORE-EVENT-SUPPORT", "SUBSCRIBE-EVENTGROUP-RETRY-MAX", "IMPLEMENTATION-CONFIG-VARIANT", "CRYPTO-R-PORT-COM-SPEC", "MACHINE-DESIGN-REF", "SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "EVENT-ID", "RTE-EVENT-IREF", "PRIORITY", "PERSISTENCY-DEPLOYMENT-ELEMENT-REF", "NAGLES-ALGORITHM", "ACTUAL-EVENT-REF", "MAX-NUMBER-FREEZE-FRAME-RECORDS", "IP-SEC-RULES", "COUPLING-ELEMENT", "PDU-REPLICATIONS", "SUBSTITUTED-FRAME-REFS", "ROOT-PARAMETER-DATA-PROTOTYPE-REF", "TCP-IP-ICMP-PROPS-REFS", "SERVICE-DEPLOYMENT-REF", "SERVICE-SW-COMPONENT-TYPE", "LIFE-CYCLE-STATE", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "FORMAT", "DIAGNOSTIC-DATA-PORT-MAPPING", "J-1939-CLUSTER-CONDITIONAL", "INCLUDED-LIBRARY-REF", "AUTO-COLLECT", "SOMEIP-SERVICE-DISCOVERY", "AR-RELEASE-VERSION", "LITERAL-PREFIX", "TARGET-MODE-REF", "ISSUED-TRIGGERS", "DATA-PROTOTYPE-IREF", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "SAMPLES-PER-MICROTICK", "SOFTWARE-CLUSTER-ID", "TP-ADDRESS-REF", "REMOTE-CERTIFICATE-REF", "CRYPTO-KEY-REF", "MODE-DECLARATION-GROUP-REFS", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS", "PDU-TRIGGERING", "FLEXRAY-FRAME", "ACL-SCOPE", "CAN-ADDRESSING-MODE", "TIME-TO-LIVE", "DATA-ELEMENT-REFS", "UDP-COLLECTION-BUFFER-SIZE-THRESHOLD", "MODE-DECLARATION-MAPPING-SET-REF", "FDC-THRESHOLD-STORAGE-VALUE", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "DO-IP-TARGET-ADDRESS-REFS", "SEC-OC-CRYPTO-SERVICE-MAPPING", "TCP-IP-DHCP-V-6-SOL-DELAY-MIN", "CONTROLLED-FID-REF", "DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE", "RECOVERY-NOTIFICATION", "LIN-CLUSTER-CONDITIONAL", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "CALIBRATION-PARAMETER-VALUE-SET-REFS", "DATA-IREF", "CONFIG-VARIANT", "NETWORK-REPRESENTATION-PROPS", "SW-CLUSTER-REFS", "HOP-COUNT", "REMARK", "ELEMENT-MAPPINGS", "TIME-MARK", "PNC-FILTER-ARRAY-MASK", "DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING", "TCP-IP-NDP-SLAAC-DELAY-ENABLED", "IGNORE-ORDER-ALLOWED", "ENGINEERING-OBJECTS", "CONTEXT-ROOT-SW-COMPOSITION-REF", "RPT-READS", "DIAGNOSTIC-PERIODIC-RATE", "SOCKET-CONNECTION-BUNDLE-REF", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "RESPONSE-DATA-SIZE", "OBJECT-DEFINTION-REFS", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "DIAGNOSTIC-OPERATION-CYCLE-REF", "FIRST-ELEMENT-REFS", "SAFE-TRANSLATION", "AGGREGATION", "MINISLOT-ACTION-POINT-OFFSET", "TIMING-CONDITIONS", "SUPPORT-INFO-BYTE", "BIT-OFFSET", "TIME-INTERVAL", "NEEDS-ORIGINAL-DATA", "LIN-MASTER", "TARGET-EVENT-REF", "SW-MAPPINGS", "SW-VERSION", "TD-EVENT-I-SIGNAL", "MAX-MARGIN", "FEATURE-REF", "MEMORY-SECTION-LOCATIONS", "TIME-LEAP-PAST-THRESHOLD", "INNER-GROUP-IREFS", "PERSISTENCY-KEY-VALUE-DATABASE", "CLASSIFICATIONS", "DIAGNOSTIC-ECU-RESET-INTERFACE", "ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "WRITING-PRIORITY", "ESP-CIPHER-SUITE-NAME", "TRACEABLE-TABLE", "TIMING-EVENT-REF", "SCOPE", "PARENT-SHORT-LABEL", "ALIAS-NAMES", "MINIMUM-INTER-ARRIVAL-TIME", "ENABLE-CONDITION-REF", "CLIENT-SERVER-RECORD-TYPE-MAPPING", "TRIGGER-REFS", "SERVICE-FIND-TIME-TO-LIVE", "MAX-SAMPLE-POINT", "MODES", "PROVIDED-SERVICE-INSTANCE", "ELEMENT", "SOURCE-EEC-REF", "WAKEUP-TX-IDLE", "VENDOR-SPECIFIC-MODULE-DEF-REF", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "CONNECTIONS", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS", "SWC-TO-APPLICATION-PARTITION-MAPPING", "CYCLE-COUNTER", "VEHICLE-PACKAGE", "TARGET-EVENT-REFS", "UPDATE", "HW-ATTRIBUTE-VALUE", "PERIODIC-RATES", "DIAGNOSTIC-EVENT", "BSW-MODULE-DEPENDENCY", "DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED", "FIBEX-ELEMENT-REF", "DEPENDENCY-ON-ARTIFACT", "I-SIGNAL-IN-I-PDU-REF", "CAN-XL-NM-PROPS", "FIREWALL-STATE-IREFS", "RESPONSE-ERROR-POSITION", "PORT-PROTOTYPE-REF-CONDITIONAL", "SA-RAND-TIME", "SYSTEM-DOCUMENTATIONS", "MC-SUPPORT", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "FILE-ELEMENTS", "FLEXRAY-AR-TP-CONNECTION", "PSK-IDENTITY-HINT", "APPLICATION-TYPE-TREF", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "MSR-QUERY-TOPIC-1", "SHORT-NAME-FRAGMENT", "SIGNALS", "EXECUTABLE-REF", "SESSION-ID-SUPPORT", "MULTICAST-THRESHOLD", "TTCAN-CLUSTER-VARIANTS", "SW-MAX-TEXT-SIZE", "SYNC-COUNTER-INIT", "FUNCTION-GROUP-SET", "TRIGGER-MAPPING", "MC-GROUP", "NAMESPACES", "CONNECTOR-REFS", "PARAMETER-INSTANCE", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "I-SIGNAL-TRIGGERING-REF-CONDITIONAL", "NETWORK-ENDPOINTS", "ATTRIBUTES", "AUTHENTICATION-ROLE-REFS", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "SCHEDULE-CHANGE-NEXT-TIME-BASE", "CALL-POINTS", "EOC-EVENT-REF", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "INITIAL-CHECKPOINT-REFS", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "QUEUED-RECEIVER-COM-SPEC", "REFERENCED-BASELINE", "REPRESENTED-MODEL-ELEMENT-REF", "SOURCE-CAN-ID-CODE", "CAN-CONTROLLER-FD-REQUIREMENTS", "FIGURE-CAPTION", "FREEZE-FRAMES", "SIGNAL-BASED-FIELD-DEPLOYMENT", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING", "DATA-TRANSFER-CLASS-REF", "REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "GATEWAYS", "COUPLING-PORT-STRUCTURAL-ELEMENTS", "SW-MAX-AXIS-POINTS", "CHANGE", "IPV-4-ADDRESS", "PAYLOAD-BYTE-PATTERN-RULE-PART", "ATTRIBUTE-VALUES", "EVENTS", "DELIVERY-ARTIFACTS", "ROUTINE-INFO", "APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF", "PROCESSOR-MODE", "ETHERNET-PRIORITY-REGENERATIONS", "MEMORY-LOW-ADDRESS-LABEL", "DENOMINATOR-GROUP", "BYTE-VALUE", "STATE-DEPENDENT-FIREWALL-REF", "INCLUDED-MODE-DECLARATION-GROUP-SET", "UPDATE-STRATEGY", "SECURE-TRANSLATION", "UNIT", "SECTION-TYPE", "HOST-REF", "SENDER-REC-RECORD-TYPE-MAPPING", "TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED", "EXTERNAL-SYNC", "SAFETY-POLICY", "SERVICE-ITEMS", "TX-MASK", "SWC-BSW-MAPPING-REF", "LOCAL-SUPERVISION-ENTITY-REF", "CONTEXT-LIMITATION-REFS", "TLS-CONNECTION-REF", "MAXIMUM-TRANSMISSION-UNIT", "DIAGNOSTIC-SERVICE-INSTANCE-REF", "FIELD-REQUEST-TOPIC-NAME", "VEHICLE-ROLLOUT-STEP", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "DATA-TYPE-MAPPING-REF", "DIAGNOSTIC-IUMPR-GROUP", "COMPOSITE-SERVICE-INTERFACE-REF", "SLOT-ID", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "CLIENT-COM-SPEC", "I-PDU-TRIGGERING-REF-CONDITIONAL", "SW-SYSTEMCONST-REF", "MULTICAST-CONNECTOR-REFS", "SECURITY-EVENT-REF", "HANDLE-TIMEOUT-TYPE", "REENTRANCY-LEVEL", "AUTO-IP-PROPS", "FIRST-TO-SECOND-DATA-TRANSFORMATION-REF", "AUTOSAR-VARIABLE-IREF", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "MACHINE", "TYPE-APPROVAL", "SECOND-KEY-SLOT-ID", "COUNTER-JUMP-DOWN-VALUE", "HEADER-ID", "OS-TASK-EXECUTION-EVENT", "SOMEIP-EVENT-DEPLOYMENT", "REQUEST-VEHICLE-INFORMATION-CLASS-REF", "PROCESSOR-SPEED", "USER-DATA-SUB-TLV", "DATA-FILTER", "FUNCTION-CLUSTER-AFFILIATION", "SW-TEST-DESC", "NM-CHANNEL-ID", "ERROR-MAPPINGS", "PHM-ACTION-LIST", "TIME-SYNC-PORT-PROTOTYPE-IREF", "SOURCE-METHOD-REF", "SW-CALIBRATION-NOTES", "REQUIRED-ENVIRONMENT-REF", "TRIGGER-REF", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "COLLECTION-SEMANTICS", "SECOND-DATA-PROTOTYPE-REF", "I-SIGNAL-TRIGGERING", "SWC-SERVICE-DEPENDENCY-IREF", "MICRO-PER-CYCLE", "GLOBAL-SUPERVISION-ENTITY", "ECUC-LINKER-SYMBOL-DEF-CONDITIONAL", "REPORTING-BEHAVIOR", "APPLICATION-VALUE-SPECIFICATION", "SW-VARIABLE-IMPLS", "CONTEXT-SWC-PROTOTYPE-REF", "IPV-4-DHCP-SERVER-CONFIGURATION", "AUTHENTICATION-BUILD-ATTEMPTS", "L-5", "TD-HEADER-ID-RANGE", "HANDLE-OUT-OF-RANGE", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "IDSM-RATE-LIMITATION-REF", "SHALL-NOT-RUN-ON-REF", "PROVIDED-DDS-EVENT-QOS-PROPS", "SUBFUNCTION", "STRING-ENCODING", "INITIAL-INACTIVITY-TIME", "TCP-IP-DHCP-V-6-CNF-DELAY-MIN", "PREDEFINED-VARIANT", "MACHINE-MODE-IREFS", "TCP-IP-PROPS-REFS", "GID-INVALIDITY-PATTERN", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "LOGICAL-ADDRESS", "MODE-DECLARATION-MAPPING", "TIMEOUT-BR", "MODE-ELEMENTS", "REPORTED-SECURITY-EVENT-IREF", "I-PDU-TIMING", "IS-DEFAULT", "P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF", "DIAGNOSTIC-J-1939-SW-MAPPING", "OBJECT-REF", "TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS", "DIAGNOSTIC-ENABLE-CONDITION", "STATUS-BIT-STORAGE-TEST-FAILED", "SYMBOL", "REQUIRED-RTE-VENDOR", "SECOND-MODE-REF", "COMPARE-TYPE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "TARGET-REFS", "DATA-MAPPINGS", "DEBOUNCE-TIME", "CHECKSUM-VALUE", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "HW-PIN", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "STRUCTURED-REQ", "SCHEDULING-POLICY", "V-2-X-FAC-USER-NEEDS", "IS-REQUIRED", "RUNTIME-PORT-CONFIGURATION", "REPRESENTS-VIN", "TCP-IP-IP-FRAGMENTATION-RX-ENABLED", "TRACE", "MAX-DIFF", "NM-IF-ECU-REF", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "CONTEXT-DATA-PROTOTYPE-REF", "WAKEUP-LOCAL-DURATION-TIME", "AFFECTED-REF", "USER-DEFINED-ETHERNET-FRAME", "TCP-IP-IP-NUM-REASS-DGRAMS", "FIELD-NOTIFIER-QOS-PROPSS", "TRANSMISSION-ACKNOWLEDGE", "TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY", "REQUIRED-FIBEX-ELEMENT-REFS", "CONSUMED-SERVICE-INSTANCE-REF", "FRAME-MAPPINGS", "SLOT-CAPACITY", "IAM-MODULE-INSTANTIATION", "BSW-INTERNAL-TRIGGERING-POINT-POLICY", "ST-MIN", "CLUSTERED-COMPONENT-IREFS", "DIAGNOSTIC-SESSION-REF", "EXECUTABLE-GROUP", "LOG-TRACE-DEFAULT-LOG-THRESHOLD", "BOR-TIME-TX-ENSURED", "REFERENCE-VALUES", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "CUSTOM-SERVICE-CLASS-REF", "ACTION-ITEMS", "DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "RULE-ARGUMENTS", "CLIENT-SERVER-OPERATION", "RATE-BASED-MONITORED-EVENT-REF", "BEHAVIOR", "LOCAL-PORT-REF", "FILE-INFO-COMMENT", "RESPONSE-REF", "UPPER-MULTIPLICITY-INFINITE", "SOFTWARE-CLUSTER-REF", "APPLICATION-PRIMITIVE-DATA-TYPE", "TBODY", "END-TO-END-TRANSFORMATION-COM-SPEC-PROPS", "LABEL-1", "ASSIGN-NAD", "TEXT-TABLE-VALUE-PAIR", "GLOBAL-TIME-PDU-TRIGGERING-REF", "SOFTWARE-CONTEXT", "TYPE-OF-EVENT-COMBINATION-SUPPORTED", "SOMEIP-SD-RULE", "TCP-PROPS", "AGE-CONSTRAINT", "SYNC-DATA-ID-LISTS", "PHYSICAL-DIMENSION", "VLAN-MODIFIER-REF", "EVENT-GROUP-ID", "DIAGNOSTIC-ROUTINE", "MODE-MAPPING", "FLEXRAY-TP-CONNECTION-CONTROL", "DESTINATION-IP-ADDRESS", "STORAGE-STATE-EVALUATION", "SW-IMPL-MAPPINGS", "RETURN-SIGNAL-TRIGGERING-REF", "SEQUENCE-OFFSET", "CONTAINED-I-PDU-PROPS", "BSW-PER-INSTANCE-MEMORY-POLICY", "DERIVED-PORT-PROTOTYPE-REF", "IEEE-1722-TP", "SHALL-RUN-ON-REF", "SUBMESSAGE-TYPE", "L-GRAPHIC", "SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING", "HW-ATTRIBUTE-VALUES", "MAC-MULTICAST-ADDRESS-REF", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "DATA-IDENTIFIER-REFS", "BULK-NV-BLOCK", "RPT-WRITES", "ACTIVATION", "GENERIC-MODULE-INSTANTIATION", "NM-VECTOR-EARLY-UPDATE", "ECUC-FUNCTION-NAME-DEF", "EXPECTED-ALIVE-INDICATIONS", "EVENT-MULTICAST-ADDRESSS", "VERIFY-IREF", "RECOVERY-NOTIFICATION-REFS", "HANDLE-INVALID", "IPV-6-RULE", "REQUEST-CONFIGURATIONS", "DLT-LOG-CHANNEL-DESIGN-REF", "VARIABLE-ACCESS-IREF", "ESP-CIPHER-SUITE-NAMES", "ORIGIN", "ASSIGNED-TRAFFIC-CLASSS", "LIN-SPORADIC-FRAME", "TTCAN-CLUSTER", "INITIAL-NAD", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "I-PDU-TRIGGERINGS", "REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS", "CHECKSUM-VERIFICATION", "ENABLE-WRITE-ACCESS-CONTROL", "PAYLOAD-PREAMBLE-INDICATOR", "RAW-DATA-STREAM-REF", "TIMEOUT-AS", "LIN-SLAVE", "REMOTE-CLIENT-CONFIG", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "SHORT-LABEL-PATTERN", "MIN-VALUE", "SUB-DOMAIN-REFS", "APPROVAL-REQUIRED", "DIAG-ROUTINE-TYPE", "NV-BLOCK-NEEDS", "MC-GROUP-DATA-REF-SET-VARIANTS", "TYPE-DEFINITION", "SUB-TYPE", "MAX-VALUE", "SA-OVER-TIME", "SYNC-LOSS-TIMEOUT", "VENDOR-SPECIFIC-MODULE-DEF-REFS", "AP-APPLICATION-ERROR", "SHALL-NOT-RUN-ON-REFS", "J-1939-CLUSTER", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "CAN-FRAME-RX-BEHAVIOR", "TOOL-VERSION", "TEAR-DOWN-ACTION-REFS", "DO-IP-TP-CONFIG", "DERIVED-OBJECT-REF", "TESTED-ITEM-REFS", "NUMBER-OF-STATIC-SLOTS", "PERSISTENCY-REDUNDANCY-CRC", "RULE", "FLEXRAY-CLUSTER-CONDITIONAL", "CALLED-ENTRY-REF", "MAX-SIZE-TO-FILL", "REQUEST-TYPE", "FIELD-REFS", "EXEC-IN-USER-MODE", "RATIO-KIND", "PARAMETER-PORT-ANNOTATIONS", "TCP-NAGLE-ENABLED", "VCID", "COND", "TCP-FAST-RETRANSMIT-ENABLED", "DATA-TRANSFORMATION-REF-CONDITIONAL", "TRANSPORT-PROTOCOLS", "CAN-FD-FRAME-SUPPORT", "PREDEFINED-TEXT", "EXCLUSIVE-AREA-REFS", "CONSUMED-SERVICE-INSTANCES", "BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR", "MAPPING-DIRECTION", "SETTER-CALL-SIGNAL-REF", "TP-CONNECTIONS", "CALC-RAM-BLOCK-CRC", "NM-REPETITION-CYCLE", "CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "EVENT-HANDLERS", "ACCURACY", "IS-SYNCHRONOUS", "SECURITY-EVENT-CONTEXT-DATA", "I-PDU-REF", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "COUPLING-PORT-SHAPER", "ECU-PARTITION-REF", "HTTP-TP", "MEMORY-HIGH-ADDRESS", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "DIAGNOSTIC-EVENT-NEEDS", "LABELED-ITEM", "PERMISSIBLE-SIGNAL-PATH", "SERVICE-INTERFACE-FIELD-MAPPING", "ERROR-SIGNALING-ENABLED", "USED-LANGUAGES", "STATE", "SW-GENERIC-AXIS-PARAMS", "ACCEPTANCE-FIELD", "GLOBAL-TIME-DOMAIN-PROPS", "LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS", "SWC-MAPPING-CONSTRAINT-REFS", "DEF", "TRIGGERING-EVENT-REF", "CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "MAXIMUM-MEMORY-CONSUMPTION", "PDU-COLLECTION-MAX-BUFFER-SIZE", "SCOPE-EVENT-REFS", "PERMIT-MULTIPLE-REFERENCES-TO-EE", "ELEMENT-REFS", "METHOD-REQUEST-PROPSS", "MANAGED-COUPLING-PORTS", "BUILD-ACTION-MANIFEST", "MODULE-INSTANTIATIONS", "NM-COORDINATOR", "IDSM-MODULE-INSTANTIATION", "FUNCTION-GROUP-STATE-IREF", "FIXED-LENGTH", "ENTRY-REF", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "HW-ELEMENT-REF-CONDITIONAL", "USED-PARAMETER-ELEMENT", "VENDOR", "REQUIRE-RESOURCES", "BUS-MIRROR-CAN-ID-RANGE-MAPPING", "API-SERVICE-PREFIX", "MODE-ELEMENT-REF", "SOURCE-TRIGGER-REF", "DEFAULT-ROUTER", "XREF", "SOFTWARE-CLUSTER-SOVD-ADDRESS", "TARGET-APPLICATION-RECORD-ELEMENT-REF", "UNLIMITED-INTEGER-VALUE-VARIATION-POINT", "STATE-DEPENDENT-STARTUP-CONFIGS", "PER-INSTANCE-PARAMETER-REF", "ECUC-QUERY", "FIBEX-ELEMENT-REF-CONDITIONAL", "FM-FEATURE-RELATION", "KEEP-ALIVES", "EXITED-MODE-REF", "REQUIRED-ENTRYS", "LONG-NAME-1", "LIFE-CYCLE-INFO-SET", "CAN-TP-ADDRESS-REF", "PERSISTENCY-DATA-REQUIRED-COM-SPEC", "ARBITRATIONS", "SIGNAL-BASED-FIELD-REF", "IKE-REAUTH-TIME", "REQUEST-SEED-ID", "TRANSMISSION-MODE-TRUE-TIMING", "CODE-LABEL", "DIAG-REQUIREMENT", "NM-REPEAT-MSG-IND-ENABLED", "USER-DEFINED-CLUSTER-VARIANTS", "MAXIMUM-ALLOWED-SIZE", "SUB-GROUP-REFS", "EXTERN-OFFSET-CORRECTION", "SECURE-COM-PROPS-FOR-TCP-REFS", "SERVICE-IDENTIFIER", "FINAL-CHECKPOINT-REF", "OPERATION-CYCLE-REF", "IMPLICIT-INTER-RUNNABLE-VARIABLES", "PROCESS-DESIGN-REF", "CAN-CONTROLLER-XL-REQUIREMENTS", "EXECUTABLE-TIMING", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "REPLICA-PDUS-REF", "DATA-COM-PROPS", "SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS", "NM-SYNCHRONIZING-NETWORK", "SIZE-OF-UNION-LENGTH-FIELD", "BSW-RESOURCE-ESTIMATION", "RUNNABLE-ENTITY-IREF", "FLAT-MAP", "PLCA-MAX-BURST-TIMER", "BSW-OS-TASK-EXECUTION-EVENT", "EXECUTION-TIME-TYPE", "DIAGNOSTIC-SERVICE-SW-MAPPING", "GROUP-IDENTIFIER", "RECEIVER-REFS", "EVALUATED-VARIANT-REF", "EVENT-HANDLER", "COMMUNICATION-CONTROLLER-MAPPING", "OBD-DTC-VALUE", "IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER", "DIAGNOSTIC-PARAMETER-ELEMENT", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "CONSTANT-SPECIFICATION-MAPPING", "CONTEXT-R-PORT-REF", "RESPONSE-ON-ALL-REQUEST-SIDS", "MASTER", "TIME-BASES", "P-PORT-PROTOTYPE", "FIREWALL-STATE-SWITCH-INTERFACE", "DDS-FIELD-DEPLOYMENT", "DEPENDS-ON-REF", "BUS-SPECIFIC-NM-ECU", "DEF-ITEM", "HARDWARE-CONFIGURATION", "ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS", "EVENT-GROUP-REF", "CLIENT-ID", "I-PDU-PORT-REFS", "NM-DATA-CYCLE", "ECUC-ADD-INFO-PARAM-DEF", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE", "TRANSFORMATION-PROPS-MAPPING-SET-REF", "POST-BUILD-VARIANT-CONDITION", "PREDECESSOR-FIFO-REF", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "RPT-HOOK", "BUS-OFF-RECOVERY", "RESET-TO-DEFAULT-SUPPORTED", "TCP-INITIAL-INACTIVITY-TIME", "ACTION-LIST-ITEM-REF", "PRE-ACTIVATION-REBOOT", "POSSIBLE-ERROR-REACTIONS", "SECURE-COM-CONFIG-REF", "ABBR-NAME", "STATE-REQUEST-PORT-IREF", "ENTRY-MODE-DECLARATION-REF", "PROTOCOL-KIND", "ACL-OBJECT-SET", "DO-IP-SOURCE-ADDRESS-REF", "REQUEST-UPLOAD-CLASS-REF", "OFFSETS", "DIAGNOSTIC-DATA-IDENTIFIER", "EXPORTABILITY", "EXTERNAL-TRIGGERING-POINTS", "GATEWAY", "ETH-TCP-IP-PROPS", "EVENT-REF", "MC-PARAMETER-ELEMENT-GROUP", "COM-TRIGGER-GRANT-DESIGN", "ELEMENT-REF", "J-1939-DTC-VALUE", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "DATA-ID", "CHAPTER", "MESSAGE-COMPLIANCE", "ACTIVATION-REASON-REPRESENTATION-REF", "COM-METHOD-GRANT-DESIGN", "FIREWALL-RULE-PROPS", "UDP-CHECKSUM-HANDLING", "ABSOLUTE-TOLERANCE", "BINARY-MANIFEST-ITEM-POINTER-VALUE", "PID", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS", "HW-ATTRIBUTE-DEF-REF", "DESTINATION-CONTEXT", "LENGTH-CLIENT-ID", "PDU-COLLECTION-TRIGGER", "HW-ELEMENT", "DATA-PROTECTION-KIND", "EVENT-DEPLOYMENTS", "OBD-CONTROL-SERVICE-NEEDS", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "DIRECT-PDU-REF", "SENSOR-ACTUATOR-REF", "MACHINE-ID", "CAN-ID-TO-CAN-ID-MAPPINGS", "COMMUNICATION-BUFFER-LOCKING", "COM-CONFIGURATION-TX-TIME-BASE", "APPLICATION-RULE-BASED-VALUE-SPECIFICATION", "POSSIBLE-AP-ERROR-REF", "VLAN", "TCP-UDP-CONFIG", "LIN-SLAVES", "LIN-MASTER-CONDITIONAL", "ASSOCIATED-PDUR-I-PDU-GROUP-REF", "COM-METHOD-GRANT", "DEFINITION-REF", "VERIFICATION-STATUS-INDICATION-MODE", "ACCESSED-PARAMETER", "TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED", "SUB-ATTRIBUTE-TAILORINGS", "RATE-CORRECTION-MEASUREMENT-DURATION", "PRECONFIGURED-CONFIGURATION-REF", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "NETWORK-ID", "PRM-UNIT", "INITIAL-EVENT-STATUS", "END-TO-END-PROTECTION", "TIME-TRIGGERED-CAN-LEVEL", "PHM-CHECKPOINT", "FIREWALL-RULE-PROPSS", "SYNCHRONIZED-MODE-GROUPS", "CIPHER-SUITE-PRIORITY", "HEADER-FILE", "CHANNELS", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "FLEXRAY-TP-NODE", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "PATTERN-PERIOD", "SW-DATA-DEF-PROPS", "SYMBOLIC-NAME-VALUE", "IMPLEMENTATION-REF", "TIME-SYNC-TECHNOLOGY", "SYSC-STRING-REF", "TIMESTAMP-FORMAT", "AFFECTED-FUNCTIONAL-CLUSTER", "P-2-MAX", "SYMBOL-PROPS", "BSW-ENTITY-REF", "KEY-IDENTITY-REF", "SERVICE-POINT-SYMBOL-POST", "RECOVERY-VIA-APPLICATION-ACTION-REF", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "SEC-OC-JOB-REQUIREMENT", "PREPARE-UPDATE", "TARGET-TRIGGER-REF", "NM-CAR-WAKE-UP-FILTER-NODE-ID", "SW-RECORD-LAYOUT-V-FIX-VALUE", "COM-MGR-USER-NEEDS", "VARIABLES", "COLLECTED-INSTANCE-IREFS", "ECUC-VALIDATION-CONDITION", "SENDER-IREF", "TRANSFORMER-CHAIN-REF", "CONTEXT-ROOT-COMPOSITION-REF", "NAME", "PERIODIC-RESPONSE-UUDT-REF", "COM-EVENT-GRANT", "MODULE-REFS", "SW-RECORD-LAYOUT-GROUP-FROM", "EVENT-PROPSS", "IMPL-INIT-VALUE", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "SPEEDUP", "UDP-PROPS", "EVENT-READINESS-GROUPS", "SYMBOL-WINDOW", "ENTITYS", "EVENT-TOPIC-ACCESS-RULE-REF", "SW-AXIS-CONTS", "WRITTEN-LOCAL-VARIABLES", "LIN-CONFIGURABLE-FRAME", "ALLOWED-TCP-OPTION", "FLEXRAY-TP-PDU-POOL", "UNICAST-UDP-CREDENTIALS", "OBD-DTC-NUMBER", "PAYLOAD-LENGTH", "PROCESS-EXECUTION-ERROR", "INTERNAL-BEHAVIORS", "SECURE-PROPS-FOR-EXTERNAL-COMM-REF", "FUNCTION-INSTANCE", "IS-ACTIVATION-LINE-DEPENDENT", "PDUR-TP-CHUNK-SIZE", "PERIODIC-EVENT-TRIGGERING", "FILE-REF", "SOURCE-CAN-ID-MASK", "TRANSFORMATION-DESCRIPTIONS", "PRESTORED-FREEZEFRAME-STORED-IN-NVM", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "TYPE-OF-DTC-SUPPORTED", "SOCKET-ADDRESS", "PREDECESSOR-REFS", "DATA-FORMAT-TAILORING", "WORST-CASE-STACK-USAGE", "DATA-PROTOTYPE-TRANSFORMATION-PROPSS", "ARTIFACT-DESCRIPTORS", "BSW-MODULE-ENTITY-REF", "ROM-BLOCK", "NETWORK-REPRESENTATION", "SW-CLASS-ATTR-INSTANCE-IMPL", "GLOBAL-ELEMENT", "RPT-EXECUTABLE-ENTITY-EVENT", "DATA-TYPE-MAPPING-SET-REF", "SW-RECORD-LAYOUT-GROUP", "ECUC-ENUMERATION-PARAM-DEF", "VIEW-MAP", "REMOTE-IP-ADDRESS-REF", "DATE", "SECTION-INITIALIZATION-POLICY", "OFFSET-TIMING-CONSTRAINT", "CONTROL-CONSUMED-EVENT-GROUP-REFS", "SHARED-AXIS-TYPE-REF", "TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "PR-PORT-PROTOTYPE", "ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL", "V-LAN-REF", "REPRESENTED-PORT-GROUP-REF", "INTERNET-HEADER-LENGTH", "PROG-CODE", "TD-EVENT-TT-CAN-CYCLE-START", "RELATIVE", "TRANSFER-PROPERTY", "RPT-EXECUTABLE-ENTITY", "J-1939-TP-NODE", "SOCKET-CONNECTION-BUNDLE", "REMOTE-SUBJECT-REF", "REQUIREMENT-REF", "PROVIDED-MEMORY-REF", "CONNECTOR-REF", "TRANSFORMATION-PROPS-REF", "SW-COMPONENT-PROTOTYPE-REF", "DHCP-PROPS", "DIAGNOSTIC-OPERATION-CYCLE", "SOFTWARE-PACKAGE-STEPS", "LOWER-CAN-ID", "COM-CONFIGURATION-ID", "NOTIFIED-STATUSS", "ICV-SECURED", "REQUESTER-REFS", "USES-END-TO-END-PROTECTION", "CLIENT-ID-DEFINITIONS", "REST-BOOLEAN-PROPERTY-DEF", "TD-EVENT-FRAME-ETHERNET", "DIAGNOSTIC-EVENT-REF", "IP-SEC-CONFIG", "DO-IP-ROUTING-ACTIVATION", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "COUNTER-FAILED-THRESHOLD", "SOFTWARE-CLUSTER-DEPENDENCY", "SERIALIZATION-TECHNOLOGY-REF", "AR-PACKAGE", "MIN-DESTINATION-PORT-NUMBER", "PNC-SYNCHRONOUS-WAKEUP", "FM-SYSCOND", "MSR-QUERY-P-2", "IDSM-TRAFFIC-LIMITATION-REF", "DIAGNOSTIC-SERVICE-TABLE-REF", "EXECUTABLE-REFS", "CONTEXT-DATA-SOURCE", "APPLICATION-RECORD-DATA-TYPE", "APPLICATION-DESCRIPTION", "INSTANTIATION-DATA-DEF-PROPS", "INSTANCE-IDENTIFIER", "USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS", "LIST", "FLAT-INSTANCE-REF", "PROCESS-REF", "SELECTOR-FIELD-LENGTH", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "MODE-DECLARATION-REFS", "REST-ENDPOINT-GET", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "DIAGNOSTIC-IO-CONTROL-CLASS", "DOC-REVISIONS", "TOPIC-NAME", "UNUSED-BIT-PATTERN", "CP-SOFTWARE-CLUSTER-REF-CONDITIONAL", "LOCAL-SUPERVISION-REFS", "I-SIGNAL-GROUP", "DIAGNOSTIC-CONTROL-DTC-SETTING", "NM-ACTIVE-COORDINATOR", "SUPPORT-INFO", "BUILD-ACTION-MANIFESTS", "TARGET-REF", "TD-EVENT-TRIGGER", "DIAGNOSTIC-COMPONENT-NEEDS", "BSW-MODE-SWITCH-EVENT", "EXPIRED-SUPERVISION-TOLERANCE", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF", "APPLICATION-ARRAY-ELEMENT-REF", "CRYPTO-SERVICE-KEY", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "FUNCTION-PROTOTYPE-EMITTER", "TCP-CONNECT-TIMEOUT", "DATA-TYPE-MAPPING-SET-REFS", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "REMOTE-ID", "COM-EVENT-GRANT-DESIGN", "TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED", "I-SIGNAL-I-PDU", "TCP-IP-NDP-RTR-SOLICITATION-INTERVAL", "TRANSFORMER-CHAIN-REFS", "MODE-DRIVEN-TRANSMISSION-MODE-CONDITION", "CRYPTO-JOB", "DIAGNOSTIC-RESPONSE-ON-EVENT", "DATA-TYPE-MAP", "DATA-LENGTH-RESPONSE", "J-1939-NM-CLUSTER", "CLEAR-EVENT-BEHAVIOR", "ALLOW-MASTER-RATE-CORRECTION", "PARAMETER-REQUIRE-COM-SPEC", "DISABLED-IN-MODE-IREFS", "OPERATION-IREFS", "COMPOSITION-DATA-PROTOTYPE-REF", "DATA-TYPE-MAPPING-REFS", "SHARED-PARAMETERS", "FIELD-REPLY-TOPIC-NAME", "SW-CLASS-INSTANCE", "TD-EVENT-COMPLEX", "CP-SOFTWARE-CLUSTER-REF", "DO-IP-TARGET-ADDRESS-REF", "APP-TASK-PROXY-REF", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "OUTER-PORTS", "ARTIFACT-LOCATORS", "VEHICLE-SYSTEM-INSTANCE", "CAN-TP-CHANNEL", "DIAGNOSTIC-AUTHENTICATION-REF", "BUFFER-OUTPUT", "CAN-NM-CLUSTER", "PARAMETER-INTERFACE", "DERIVED-INTERFACE-REF", "DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL", "AUDIENCES", "ECU-EXTRACT-REF", "ACL-OBJECT-REFS", "DLT-CONTEXT-REF-CONDITIONAL", "PORT-INTERFACE-BLUEPRINT-MAPPING", "PROTOCOL-NAME", "DO-IP-ACTIVATION-LINE-NEEDS", "CAN-XL-FRAME-TRIGGERING-PROPS", "DEFAULT-LC-STATE-REF", "CALIBRATION-PARAMETER-VALUE-SET", "CHOICES", "CLIENT-SERVER-OPERATION-IREF", "TARGET-P-PORT-REF", "MAC-MULTICAST-GROUP", "LOGICAL-SUPERVISION", "CUSTOM-SUB-FUNCTION-NUMBER", "INVALID-VALUE", "MAP", "HARDWARE-PLATFORM", "META-DATA-ITEM-SETS", "NM-CLUSTERS", "TLV-DATA-ID", "DATA-RECEIVE-POINT-BY-VALUES", "TYPE-REFERENCE", "METHOD-MAPPINGS", "NM-READY-SLEEP-COUNT", "MEMORY-USAGES", "SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING", "INITIAL-DYNAMIC-PART", "OBD-MID", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "CRYPTO-SERVICE-QUEUE-REF", "DDS-EVENT-DEPLOYMENT", "REQUIRED-OUTER-PORT-REF", "REMOTE-UNICAST-CONFIG-REFS", "SERVICE-KIND", "COMPONENT-SEPARATION", "IS-OPTIONAL", "TCP-GENERAL-INACTIVITY-TIME", "BITFIELD-TEXT-TABLE-MASK-FIRST", "TRANSFERS", "ECU-INSTANCE-PROPS", "ITEM", "HEADER-LENGTH", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "NETWORK-CONFIGURATION", "VARIABLE-ACCESSED-FOR-DEBUG-REF", "MODE-DECLARATION", "PARAMETER-ACCESSED-FOR-DEBUG-REF", "ECU-RESOURCE-MAPPINGS", "DNS-SERVER-ADDRESSES", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF", "PHM-HEALTH-CHANNEL-INTERFACE", "HW-ELEMENT-CONNECTOR", "BUILD-ACTION-ENVIRONMENT", "CONTEXT-DESCRIPTION", "SDG-FOREIGN-REFERENCE", "PERSISTENCY-FILE-ELEMENT", "TIMING-CLOCK-SYNC-ACCURACY", "VEHICLE-SYSTEM", "PATTERN-LENGTH", "VEHICLE-DESCRIPTION-REF", "FEATURE-IREF", "KEY", "TARGET-MODULE-ID", "ECUC-CHOICE-REFERENCE-DEF", "SD-CLIENT-CONFIG-REF", "APPLICATION-ACTION-ITEM", "DIAGNOSTIC-CONDITION-INTERFACE", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "RESOURCE-GUARD-VALUE", "TDLET-ZONE-CLOCK", "UNIT-GROUP-REF", "SW-CALIBRATION-ACCESS", "REMOTE-PORT-RANGE-END", "BASE-COMPOSITION-REF", "UDP-LOCAL-PORT", "NO-SUPERVISIONS", "CLIENT-SERVICE-MINOR-VERSION", "IS-GLOBAL", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS", "CRYPTO-NEED", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "CRC-OFFSET", "ACL-OPERATION", "RTE-EVENT-IREFS", "MIN", "TRIGGER-IREF", "SEQUENTIAL-INSTRUCTIONS-BEGIN", "TIMING-VARIABLES", "BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE", "MSR-QUERY-CHAPTER", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "ANONYMOUS", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "END-2-END-METHOD-PROTECTION-PROPS", "MAX-BYTES-IN-INTERVAL", "GRANT-REFS", "SYSTEM-MEMORY-USAGES", "SOMEIP-EVENT", "DO-IP-GID-NEEDS", "NM-MAIN-FUNCTION-ACROSS-FR-CYCLE", "CUSTOM-SDG-DEF-REFS", "EXTENDED-DATA-RECORDS", "MAC-SEC-KAY-CONFIG", "TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF", "VERIFY-UPDATE", "PORT-INTERFACE-REF", "FM-FEATURE-SELECTION", "MINIMUM-START-INTERVAL", "SCHEDULER-NAME-PREFIX-REF", "PADDING-VALUE", "REPORTABLE-SECURITY-EVENT-REFS", "GENERATED-ARTIFACTS", "LOG-TRACE-LOG-MODE", "VERIFY-REFS", "FLEXRAY-PHYSICAL-CHANNEL", "CONTEXT-SWC-PROTOTYPE-REFS", "MULTICAST-SECURE-COM-PROPS-REF", "ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL", "TCP-FIN-WAIT-2-TIMEOUT", "I-PV-6-EXT-HEADER-FILTER-LIST", "PARAMETER-SW-COMPONENT-TYPE", "TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "I-PDU-IDENTIFIER-TCP-REFS", "CRC-CORRECTION-FIELD", "MEM-USAGE", "WRITING-FREQUENCY", "MODE-REQUEST-TYPE-MAPS", "TRANSMITTER-REF", "ROOT-CONTEXT-REF", "PHYSICAL-CHANNEL-REFS", "MAXIMUM-INTER-ARRIVAL-TIME", "VARIATION", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "ECU-INSTANCES", "ETHER-TYPE", "MODE-INTERFACE-MAPPING", "SENDING-NODE-REF", "STATUS-SUB-TLV", "MODE-DRIVEN-TRUE-CONDITIONS", "CONTROLLER-REF", "ML-GRAPHIC", "DO-IP-INTERFACE", "TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "MODE-DECLARATION-GROUP", "ARGUMENT", "ENABLE-READ-ACCESS-CONTROL", "SEPARATION-TIME", "PHYSICAL-REQUEST-REF", "DERIVED-PORT-INTERFACE-REF", "DIAGNOSTIC-STORAGE-CONDITION-REF", "BUS-MIRROR-CHANNEL-MAPPING-IP", "VARIABLE-ACCESS-REF", "ASSIGNED-ENTRY-ROLES", "CONDITIONS", "ECUC-QUERY-EXPRESSION", "STORAGE-CONDITION-GROUP-REF", "INTERFACE-BLUEPRINT-MAPPING", "ECUC-FLOAT-PARAM-DEF", "RELEASE-NOTES-REF", "SWC-TRIGGER-IREF", "EVALUATED-ELEMENT-REFS", "TIME-PERIOD", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS", "PATH-MTU-ENABLED", "DATA-TRANSFORMATION", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS", "SERVICE-INTERFACE", "PHYSICAL-LAYER-TYPE", "MAXIMUM-NUMBER-OF-RETRIES", "TCP-IP-IP-NUM-FRAGMENTS", "MAXIMUM-SEGMENT-LENGTH", "ECUC-TEXTUAL-PARAM-VALUE", "DTC-SETTING-PARAMETER", "SEND-INDICATION", "INTENDED-TARGET-MACHINE-REF", "TRIGGERS", "PNC-PREPARE-SLEEP-TIMER", "FIRE-AND-FORGET-METHOD-MAPPINGS", "ROLE-BASED-RESOURCE-DEPENDENCY", "PLCA-TRANSMIT-OPPORTUNITY-TIMER", "INCLUDED-DATA-TYPE-SETS", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "MAX-BS", "PROCESSING-KIND", "THEAD", "DIAGNOSTIC-EVENT-REF-CONDITIONAL", "SYNC-LOSS-THRESHOLD", "BROADCAST", "AUTHENTICATION-TIMEOUT", "SELECTOR-FIELD-BYTE-ORDER", "RPT-EXECUTABLE-ENTITY-EVENTS", "HW-PIN-GROUP-CONNECTIONS", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "DLT-USER-NEEDS", "REPORTING-PORT-PROTOTYPE-IREF", "DDS-RPC-SERVICE-DEPLOYMENT", "CAN-TP-CONFIG", "SDG-CLASSES", "DERIVED-FROM-BLUEPRINT-REF", "FILTERING-DEBOUNCING", "FINAL-CHECKPOINT-REFS", "PORTS", "SW-GENERIC-AXIS-PARAM-TYPES", "POSSIBLE-ERRORS", "MINIMUM-SUPPORTED-UCM-VERSION", "INTERNAL-TRIGGER-OCCURRED-EVENT", "CALL-TIMEOUT", "DLT-LOG-SINK", "NOTIFIER-DATA-ELEMENT-REF", "NOMINAL-EXECUTION-TIME", "NETWORK-TIME-SLAVE-REF", "MC-DATA-INSTANCE-REF", "DPG-REQUIRES-COHERENCYS", "DATA-PROTOTYPE-IN-METHOD-ARGUMENT-INSTANCE-REF-IREF", "SW-SYSTEMCONST-VALUE", "OBD-MONITOR-SERVICE-NEEDS", "ACTION-LIST-ITEM-REFS", "ROUTINE-SUBFUNCTION-REF", "TCP-MAX-RTX", "DEBOUNCE-BEHAVIOR", "IGNORE-AFTER-TX", "RETRY", "MESSAGE-LINK-POSITION", "DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL", "DIAGNOSTIC-SERVICE-TABLE", "SWC-INTERNAL-BEHAVIOR", "EXECUTION-DEPENDENCY", "IUMPR-GROUP", "HW-PIN-CONNECTIONS", "CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING", "TEXTUAL-CONDITION", "MULTIPLEXED-I-PDU", "CONFIG-ELEMENT-DEF-GLOBAL-REF", "LIN-FRAME-TRIGGERING", "HW-PIN-REFS", "UPPER-REF", "DATA-TRANSFORMATION-KIND", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "EVENT-SYMBOL-NAME", "DIAGNOSTIC-REQUEST-UPLOAD", "AGGREGATION-TAILORING", "USE-INSTEAD-REF", "BSW-MODULE-DEPENDENCYS", "PREPARE-ROLLBACK", "POSSIBLE-AP-ERROR-REFS", "TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE", "MAX-NUMBER-OF-EVENT-ENTRIES", "TOLERATED-FAILED-CYCLES", "SW-RECORD-LAYOUT", "CAN-XL-CONFIG-REF", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "DIAGNOSTIC-TEST-RESULT-REF", "LIN-UNCONDITIONAL-FRAME-REFS", "NUMBER-OF-MINISLOTS", "SIGNAL-BASED-METHOD-REF", "MINIMUM", "CIPHER-SUITE-SHORT-LABEL", "BUILD-ACTION-IO-ELEMENT", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "DIAGNOSTIC-ACCESS-PERMISSION", "MAX-MAJOR-VERSION", "PROVIDED-CLIENT-SERVER-ENTRYS", "INITIALIZATION-VECTOR-LENGTH", "BSW-ENTRY-RELATIONSHIP", "ECUC-CONTAINER-VALUE", "STATE-REFERENCES", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION", "INSTANTIATION-DATA-DEF-PROPSS", "FM-ATTRIBUTE-DEF", "ACTIVE-SUPERVISION-REFS", "NETWORK-ENDPOINT-REFS", "RECOVERABLE-IN-SAME-OPERATION-CYCLE", "CONTEXT-DATA-PROTOTYPE-IN-SR-REF", "REST-ARRAY-PROPERTY-DEF", "ENVIRONMENTAL-CONDITION-REF", "NM-INSTANCE-ID", "MAC-SEC-GLOBAL-KAY-PROPS", "SERVICE-INTERFACE-PEDIGREE", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "VALUE-CONFIG-CLASSES", "GLOBAL-SUPERVISIONS", "BLOCK-INCONSISTENT", "NM-IMMEDIATE-NM-CYCLE-TIME", "PROCESSING-STYLE", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "SHORT-LABEL", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "REFINED-EVENT-IREF", "NV-BLOCK-DATA-MAPPING", "PROCESS-TO-MACHINE-MAPPING", "SUPPORTS-ASYNCHRONOUS-MODE-SWITCH", "SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS", "IMPLEMENTATION-ARRAY-ELEMENT-REF", "OPERATION-MAPPINGS", "SYSTEM-MAPPING", "RPT-AR-HOOK-IREF", "SUPPLIER-VALIDATION-ORDER-REF", "AFFECTED-APPLICATION", "UNICAST-CREDENTIALS", "NV-BLOCK-DESCRIPTORS", "DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER", "DELAY-COMPENSATION-B", "UPDATE-INDICATION-BIT-POSITION", "ALLOCATE-SHADOW-COPY", "DLT-LOG-SINK-REFS", "COUNTER-INCREMENT-STEP-SIZE", "LATENCY-CONSTRAINT-TYPE", "POSITION-IN-TABLE", "FM-FEATURE", "ALLOW-PASSIVE-TO-ACTIVE", "TD-EVENT-BSW-MODULE-TYPE", "RPT-SUPPORT-DATA", "SW-HOST-VARIABLE", "VARIABLE-INSTANCE-IREF", "ROOT-DATA-PROTOTYPE-IN-CS-REF", "BSW-DISTINGUISHED-PARTITION", "GID", "CONTAINED-PROCESS-REF", "ALLOW-UNAUTHENTICATED-PARTICIPANTS", "COUPLING-PORT-SCHEDULER", "SOFTWARE-CLUSTER", "COM-OFFER-SERVICE-GRANT", "INITIAL-FIND-BEHAVIOR", "NEIGHBOR-CACHE-SIZE", "E-2-E-PROFILE-CONFIGURATION-REF", "DIRECT-SUCCESSOR-REFS", "NETWORK-IDLE-TIME", "EXPLICIT-CONGESTION-NOTIFICATION", "ECUC-REFERENCE-DEF", "CONTEXT-LIMITATION-REF", "NDP-PROPS", "COM-MANAGEMENT-GROUP-REF", "REPOSITORY", "HAS-NOTIFIER", "NM-HW-VOTE-ENABLED", "OUTER-PORT-REF", "SAVE-CONFIGURATION", "TARGET-R-PORT-REF", "PROCESS-PHM-ACTION-ITEM", "SECURE-PROPS-FOR-UDP-REF", "PARAMETER-ACCESS", "AP-APPLICATION-ERROR-REF", "REMOTE-IP-ADDRESS-REFS", "ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS", "PERSISTENCY-FILE-STORAGE-INTERFACE", "CRC-SEQUENCE-ID", "CLUSTER-DRIFT-DAMPING", "META-DATA-ITEM", "SEND-POLICYS", "GLOBAL-TIME-PDU-REF", "MAC-SEC-PARTICIPANT-SET", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS", "END-TO-END-PROTECTION-I-SIGNAL-I-PDUS", "RECEIVER-INTENT", "OPT-IMPL-RECORD-ELEMENTS", "STATE-MANAGEMENT-MODULE-INSTANTIATION", "TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "OPT-ARGUMENT-REFS", "DATA-PROTOTYPE-GROUP", "ETH-GLOBAL-TIME-DOMAIN-PROPS", "MAX-PWM-S", "TLS-CIPHER-SUITE", "XFILE", "ECUC-MULTIPLICITY-CONFIGURATION-CLASS", "ACL-OBJECT-CLASS", "BACKGROUND-EVENT", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS", "SETTER-RETURN-SIGNAL-REF", "SUB-CONTAINERS", "MODE-GROUP-REF", "ICMP-V-6-PROPS", "COMMUNICATION-CONNECTOR-REF", "CONTAINER-TIMEOUT", "AGGREGATION-REF", "DO-IP-ROUTING-ACTIVATIONS", "DIAGNOSTIC-EVENT-INTERFACE", "ASSOCIATED-COM-I-PDU-GROUP-REFS", "PHYSICAL-DIMENSION-REF", "SDF", "NM-BUS-SYNCHRONIZATION-ENABLED", "SEC-OC-CRYPTO-MAPPING-REF", "ROUTING-GROUP-REF", "ETH-IP-PROPS-REF", "RESET-CONFIRMED-BIT-ON-OVERFLOW", "TD-EVENT-I-PDU", "FUNCTION-GROUP-IREF", "DATA-SEND-COMPLETED-EVENT", "LIN-TP-CONNECTION", "ORDERED-MASTER", "FRESHNESS-PROPSS", "WAKEUP-TX-ACTIVE", "AGING-ALLOWED", "CRYPTO-SIGNATURE-SCHEME", "TARGET-ELEMENT-REF", "CS-ERROR-REACTION", "ROOT-AUTOSAR-DATA-PROTOTYPE-REF", "REMOTE-UNICAST-CONFIG-REF", "TRANSIT-TO-INVALID-EXTENDED", "ENCAPSULATED-ENTRY-REF", "NETWORK-HANDLES", "IDENTIFIABLE-REF", "BOOLEAN-VALUE-VARIATION-POINT", "RESPONSE-ERROR-REF", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "ETHERNET-COMMUNICATION-CONNECTOR", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "SW-CLUSTER-REF", "COMMUNICATION-CYCLE", "IMPLEMENTATION-REFS", "DIAGNOSTIC-IO-CONTROL", "CLIENT-SERVER-TO-SIGNAL-MAPPING", "APPROVAL-STATUS", "GETTER-REF", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "EVENT-SOURCE-REF", "SIGNATURE-SUPPORT-CP", "I-SIGNAL-GROUP-REF", "ROOT-VARIABLE-DATA-PROTOTYPE-REF", "RESET-TO-DEFAULT", "RATE-DEVIATION-MEASUREMENT-DURATION", "SOURCE-SERVICE-INTERFACE-REF", "DDS-EVENT-QOS-PROPS", "SWC-TIMING", "VERBATIM", "MEMORY-LOW-ADDRESS", "TLS-JOB-MAPPINGS", "EXTENDS-META-CLASS", "SUPERVISION-MODE-CONDITION", "NM-REMOTE-SLEEP-IND-ENABLED", "VARIABLE-ACCESS", "REQUIRED-SERVICE-INSTANCE-ID", "LOCAL-CERTIFICATE-REF", "SECURITY-EVENT-DEFINITION-REF-CONDITIONAL", "STATUS-ID", "E", "NV-DATA-PORT-ANNOTATION", "UPPER-BOUND", "SECOND-TRIGGER-REF", "SWC-TO-SWC-SIGNAL", "NM-CLUSTER-COUPLINGS", "CONNECTION-NEGOTIATION-BEHAVIOR", "ON-FAIL-PERMISSIVE-MODE-TIMEOUT", "CAN-FD-CONFIG", "PROCESS-STATE-MACHINE", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "ACCESS-COUNT-SETS", "RAW-DATA-STREAM-CLIENT-INTERFACE", "ROUGH-ESTIMATE-HEAP-USAGE", "TARGET-STATUS-REF", "ALGORITHM-SECONDARY-FAMILY", "UAS-ID", "DECOMPOSITIONS", "RX-IDENTIFIER-RANGE", "REQUIRED-GENERATOR-TOOLS", "SDU-REF", "PASS-THROUGH-SW-CONNECTOR", "EVENT-MULTICAST-UDP-PORT", "DEFAULT-LOG-THRESHOLD", "J-1939-NM-NODE-REF", "IS-AUTO-VALUE", "DATA-TYPE-FOR-SERIALIZATION-REF", "NM-MAIN-FUNCTION-PERIOD", "META-DATA-ITEM-SET", "HANDLE-TERMINATION-AND-RESTART", "DHCP-SERVER-CONFIGURATION", "VARIABLE-LENGTH", "REACTION-CODE", "DO-NOT-FRAGMENT", "SW-DATA-DEPENDENCY-FORMULA", "INIT-EVENT", "VERSION-DRIVEN-FIND-BEHAVIOR", "SYSTEM-SIGNAL-REF", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "SUPERVISION-MODE", "STATE-MANAGEMENT-REQUEST-RULE", "ONE-EVERY-N", "KEY-VALUE-PAIRS", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "INITIAL-STATUS", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "BITFIELD-TEXT-TABLE-MASK-SECOND", "AUTOMATIC-END", "PERSISTENCY-PROVIDED-COM-SPEC", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF", "CRYPTO-KEY-SLOT", "PLCA-LOCAL-NODE-COUNT", "INITIAL-MODE-REF", "TIME-SYNC-MODULE-INSTANTIATION", "TIMING-MODE-INSTANCE", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "SOURCE-ADDRESS-CODE", "PORT-INTERFACE-MAPPING-SET", "ESTIMATED-DURATION-OF-CAMPAIGN", "FRAME-PORT-REF", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE", "PDUR-I-PDU-GROUP", "PDU-TRIGGERING-REF", "COMMENT", "MESSAGE-TYPE-INFO", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF", "ECU-TASK-PROXY-REFS", "FRAME-REF", "METHOD-ACTIVATION-ROUTING-GROUPS", "I-SIGNAL-REFS", "VFB-TIMING", "CONFIG-CLASS", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "NETWORK-MASK", "MAX-NUMBER-OF-TIME-QUANTA-PER-BIT", "TRANSMISSION-MODE-DECLARATION", "RUNNABLE-REF", "ACCESSED-DATA-PROTOTYPE-IREF", "SECURITY-EVENT-DEFINITION-REF", "REQUIRED-SERVICE-PORT-IREF", "BLUEPRINT-POLICY-LIST", "DIAGNOSTIC-FREEZE-FRAME", "DATA-WRITE-COMPLETED-EVENT", "SYSTEM-REF", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "ENUMERATION-MAPPING-TABLE", "PAYLOAD-TYPE", "UCM-MODULE-INSTANTIATION-REF", "MULTICAST-REF", "BLUEPRINT-MAPPING", "BR", "SW-COMPONENT-PROTOTYPE-IREF", "CRYPTO-CERTIFICATE-REF", "IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "SERVICE-PROXY-SW-COMPONENT-TYPE", "MODULE-INSTANTIATION-REF", "PORT-NUMBER", "WAKEUP-FRAME-REFS", "WAKE-UP-PATTERN", "RTE-EVENT-IN-SYSTEM-SEPARATION", "I-PV-6-EXT-HEADER-FILTER-SET", "SERVICE-NEEDS", "SW-COMPONENT-PROTOTYPE-ASSIGNMENT", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "NM-CYCLETIME-MAIN-FUNCTION", "ETHERNET-PHYSICAL-CHANNEL", "INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "AR-TYPED-PER-INSTANCE-MEMORY-REF", "CIPHER-SUITE-CONFIGS", "PLCA-PROPS", "DELAY-COMPENSATION-A", "SPECIFICATION-SCOPE", "APPLICATION-ASSOC-MAP-DATA-TYPE", "LOGICAL-EXPRESSION-ARGUMENT-REF", "IDSM-INSTANCES", "CLIENT-POLICYS", "SW-AXIS-TYPE-REF", "ACCESS-DATA-RECORD-SIZE", "UCM-PROCESSINGS", "REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF", "DIAGNOSTIC-CONNECTION-REF-CONDITIONAL", "DEFAULT-PERIOD-BEGIN", "DIAGNOSTIC-DATA-ELEMENT-REF", "PHM-ACTION", "ECU-ID", "PROVIDED-MODE-GROUPS", "SW-COMPONENT-IREF", "VALUE-DATA-TYPE-REF", "ALGORITHM-MODE", "CAN-XL-CONFIG", "SW-MIN-AXIS-POINTS", "APPLICATION-ID", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "OPERATOR", "CUSTOM-DOCUMENT-ELEMENT-REF", "EXTERNAL-REPLACEMENT-REF", "DOCUMENT-ELEMENT-SCOPE", "SELECT-BLOCK-FOR-FIRST-INIT-ALL", "RESPONSE-ON-SECOND-DECLINED-REQUEST", "AUTHENTICATION-RETRIES", "SWC-FLAT-SERVICE-DEPENDENCY-REF", "REPLY-TOPIC-NAME", "GLOBAL-TIME-ETH-MASTER", "TLV-DATA-ID-DEFINITION", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "NON-VERBOSE-MODE", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "PHYS-CONSTRS", "V", "MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS", "GETTER-RETURN-REF", "MODE-IN-SWC-INSTANCE-REF", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS", "ERROR-CODE", "UCM-DESCRIPTION", "FROM-REF", "PRIMITIVE-ATTRIBUTE-TAILORING", "WINDOW-SIZE-INVALID", "DDS-REQUIRED-SERVICE-INSTANCE", "UDS-DTC-NUMBER", "SHORT-NAME-FRAGMENTS", "ACTION-LIST-EXECUTION", "SEND-ACTIVITY", "RPT-HOOK-ACCESS", "HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS", "VALUE-AXIS-DATA-TYPE-REF", "VENDOR-SIGNATURE-REF", "DATA-FILTER-TYPE", "ACL-ROLE-REF", "COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE", "HW-ELEMENT-REF", "DATA-CONSTR-RULE", "EXCLUSIVE-AREA", "MEMORY-RANGE-REFS", "NETWORK-TIME-CONSUMER-REF", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "PARAMETERS", "BSW-INTERNAL-BEHAVIOR", "OFFSET-CORRECTION-ADAPTION-INTERVAL", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "APPL-WATCHDOG-LIMIT", "TIME-BASE-PROVIDER-REF", "LABELED-LIST", "MAC-MULTICAST-ADDRESS-REFS", "TP-PGS", "ACCESSED-MODE-GROUPS", "SDG-PRIMITIVE-ATTRIBUTE", "USES-TIME-BASE-RESOURCE", "COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION", "HW-PIN-GROUP-CONNECTOR", "VARIABLE-DATA-PROTOTYPE", "CRYPTO-PROVIDER-REF", "WINDOW-SIZE", "EXTERNAL-TRIGGERING-POINT", "CAN-GLOBAL-TIME-DOMAIN-PROPS", "VALIDATION-ROOT", "ROW", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "ASSERTIONS", "EXTENDED-ADDRESSING", "TP-ADDRESS", "FOLLOW-UP-OFFSET", "MULTICAST-SD-IP-ADDRESS-REF", "DDS-RULE", "MIN-SERVICE-POINT-ID", "FIRE-AND-FORGET-METHOD-MAPPING", "PNC-VECTOR-LENGTH", "BUILD-ACTIONS", "BLOCK-STATES", "RESOURCE-GROUP", "AUTOSAR-DATA-TYPE-REF-CONDITIONAL", "LOG-CHANNEL-ID", "COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "CONTAINED-I-PDU-TRIGGERING-PROPSS", "PERSISTENCY-FILE-STORAGE", "SYNCHRONOUS-SERVER-CALL-POINT", "VALID-BINDING-TIMES", "COUNTER-DECREMENT-STEP-SIZE", "PROVIDED-COM-SPECS", "INTER-MESSAGE-TIME", "MIN-PWM-S", "APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION", "MAXIMUM-DYNAMIC-PAYLOAD-LENGTH", "EID-RETRIEVAL", "INTERPOLATION-ROUTINE-MAPPING-SET", "MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS", "BUFFER-RATIO", "SW-VARIABLE-REFS", "DETERMINISTIC-CLIENT", "COLLECTION-REF", "VARIABLE-DATA-PROTOTYPE-IREF", "EXIT-MODE-DECLARATION-REF", "DIAGNOSTIC-ADDRESS", "MAX-AR", "ACCESS-CONTROL", "TRIGGER-MODE", "COM-PROPS", "SEARCH-INTENTION", "GENERAL-INACTIVITY-TIME", "DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL", "MAX-SUPPORTED-DID-LENGTH", "AUTOSAR-VARIABLE-IN-IMPL-DATATYPE", "SERVICE-POINT-SYMBOL-PRE", "INFO-TYPE", "MAX-PERIODIC-DID-TO-READ", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "ELLIPTIC-CURVE-REFS", "CRYPTO-PRIMITIVE", "MACRO-INITIAL-OFFSET-A", "SECURED-I-PDU", "ARBITRATION", "NODE-REF", "NM-RANGE-CONFIG", "DIAGNOSTIC-TROUBLE-CODE-OBD", "SW-CLASS-PROTOTYPE-REF", "REFERENCES", "COM-FIELD-GRANT-DESIGN", "CLAIMED-FUNCTION-GROUP-REF", "SOURCE-EVENT-REFS", "ASYNCHRONOUS-SERVER-CALL-POINT-REF", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "SERVICE-INSTANCE-COLLECTION-SET", "SUPERVISION-MODES", "HW-TYPE-REF", "ROOT-SW-COMPOSITION-PROTOTYPE", "RECEPTION-POLICYS", "EXTERNAL-AUTHENTICATIONS", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "RPT-SERVICE-POINT-POST-REF", "EVENT-REFS", "SCOPE-EVENT-REF", "TAG-ID", "SERVICE-TABLES", "RESOURCE-NEEDS-REFS", "SET-FUNCTION-GROUP-STATE-IREF", "PHM-CONTRIBUTION-REFS", "PROVIDED-SERVICE-PORT-IREF", "PNC-WAKEUP-DATA-MASK", "PROVIEDE-DATA-REF", "EXECUTABLE-REQUIRED-PORT-IREF", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "MAX-DESTINATION-PORT-NUMBER", "DIAGNOSTIC-DE-AUTHENTICATION", "MODE-REQUEST-TYPE-MAP", "EVENT-MAPPINGS", "CAN-XL-PROPS-REF", "TCP-IP-PROPS-REF", "CYCLE", "RX-SECURITY-VERIFICATION", "NM-CONFIG", "OPERATION-IREF", "MODE-GROUP-IREF", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "OBJECT-REFS", "TIME-BASE-KIND", "ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS", "FR-CLUSTER-REF", "DDS-PROVIDED-SERVICE-INSTANCE", "TIME-LEAP-HEALING-COUNTER", "CONSTANT-VALUE-MAPPING-REFS", "RECOVERY-NOTIFICATION-RETRY", "TARGET-FIELD-REF", "IS-SERVICE", "ETHERNET-CLUSTER", "LIN-SCHEDULE-TABLE", "ENUMERATOR-VALUE", "VALUE-SPEC", "SUPPORTED-CONFIG-VARIANTS", "PRIMITIVE-FAMILY", "REPLICA-PDUS-REFS", "WAKEUP-RX-LOW", "SAMPLE-CLOCK-PERIOD", "SWC-MODE-SWITCH-EVENT-REF", "VALUE-IREF", "REFERRABLE-REF", "METHOD-RESPONSE-PROPSS", "STATIC-SOCKET-CONNECTIONS", "MIN-DEADLINE", "CREATED-DATAS", "TLV-DATA-ID-DEFINITIONS", "UCM-TO-TIME-BASE-RESOURCE-MAPPING", "GLOBAL-SUPERVISION", "UDP-TP-PORT", "SETTER-RETURN-REF", "INTERPOLATION-ROUTINE-MAPPINGS", "PROFILE-BEHAVIOR", "I-SIGNAL-TRIGGERINGS", "RECOVERY", "FIRST-APPLICATION-ERROR-REF", "EID", "INTERPOLATION-ROUTINE-REF", "PDU-ACTIVATION-ROUTING-GROUPS", "DATA-TYPE-FOR-SERIALIZATION-REFS", "REQUIRED-AR-ELEMENT-REF", "NOTIFIER", "TTCAN-ABSOLUTELY-SCHEDULED-TIMING", "BYPASS-VLAN", "REQUESTS", "CONSUMED-EVENT-GROUP-REF", "STANDARD", "HEADER-ID-LONG-HEADER", "TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED", "TCP-IP-NDP-DYNAMIC-MTU-ENABLED", "FORMULA", "FUNCTION", "MAC-UNICAST-ADDRESS", "HW-PIN-CONNECTOR", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "TEST-IDENTIFIER", "TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "APP-ID", "SOCKET-PROTOCOL", "TARGET-PORT-PROTOTYPE-REF", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "PAYLOAD-BYTE-PATTERN-RULE", "REQUESTOR-REFS", "TEMPLATE-TYPE-REF", "FALL-BACK-INTERNAL", "SECURED-AREA-LENGTH", "DTC-STATUS-AVAILABILITY-MASK", "DOMAIN-ID", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "MAX-TRCV-DELAY-COMPENSATION-OFFSET", "FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR", "TLS-IAM-REMOTE-SUBJECT", "SUPPORT-BUFFER-LOCKING", "SW-SERVICE-IMPL-POLICY", "CANCELLATION", "SYNCHRONIZATION-CONSTRAINT-TYPE", "OUT-MEASURMENT-SET", "MODEL-OBJECT-REFERENCE", "CONNECT", "CRYPTO-KEY-MANAGEMENT-NEEDS", "ENCRYPTION-REF", "DATA-RECEIVE-ERROR-EVENT", "AUXILIARY-FIELD-DEFINITIONS", "TRANSFORMATION-TECHNOLOGYS", "CONTEXT-COMPOSITION-REF", "BSW-MODULE-DOCUMENTATIONS", "CONSUMED-SERVICE-INSTANCE", "RX-PDU-TRIGGERING-REF", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "MOLAR-AMOUNT-EXP", "DELEGATION-SW-CONNECTOR", "TIME-BASE-RESOURCE-REF", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "BUILD-ACTION-MANIFEST-REF-CONDITIONAL", "SWC-TO-SWC-OPERATION-ARGUMENTS", "TCP-IP-IP-REASSEMBLY-BUFFER-SIZE", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "TP-ADDRESS-REFS", "COM-MANAGEMENT-MAPPINGS", "RX-NM-PDU-REF", "INTERNAL-CONSTRS", "SYSTEM-TRIGGERED-EVENTS", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "PRIMITIVE", "SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "CAN-FD-BAUDRATE", "ECUC-LINKER-SYMBOL-DEF-VARIANTS", "NM-COORD-CLUSTER", "AP-APPLICATION-ENDPOINTS", "TRACED-FAILURES", "DIAGNOSTIC-CONNECTIONS", "TARGET-R-PORT-PROTOTYPE-REF", "GLOBAL-TIME-PROPS", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "PRE-ACTIVATE-IREF", "BASE-IS-THIS-PACKAGE", "SD-SERVER-TIMER-CONFIGS", "HW-ATTRIBUTE-LITERALS", "RATIONALE", "INTRODUCTION", "ASSIGN-FRAME-ID", "COMPU-DENOMINATOR", "NM-BUSLOAD-REDUCTION-ENABLED", "BURST-SIZE", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "TRANSPORT-PLUGINS", "INTENDED-FILENAME", "AP-APPLICATION-ERROR-SET", "MAX-EXP-BS", "TEST-ID", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "TRACE-REF", "INTERFACE-BLUEPRINT-MAPPINGS", "ACTION-LIST", "PLATFORM-TIME-BASES", "WRITING-STRATEGY-ROLE", "FREE-FORMAT", "NM-BUSLOAD-REDUCTION-ACTIVE", "PROVIDER-IREF", "MAXIMUM-DISTANCES", "DO-IP-LOGIC-ADDRESSS", "CONTEXT-R-PORT-PROTOTYPE-REF", "SWC-BSW-RUNNABLE-MAPPING", "CONNECTION-BUNDLES", "AUTHENTICATION-PROPSS", "BSW-VARIABLE-ACCESS", "TCP-IP-NDP-DESTINATION-CACHE-SIZE", "RECEIVE-WINDOW-MIN", "PROVIDER-REF", "RUNNABLE-ENTITY-ARGUMENT", "STARTUP-CONFIG", "TCP-KEEP-ALIVE-INTERVAL", "TCP-IP-ARP-PACKET-QUEUE-ENABLED", "VLAN-REF", "MAP-ELEMENT-TUPLES", "BSW-MODE-SWITCHED-ACK-EVENT", "MODE-CONDITION", "PERIOD", "ITEM-LABEL", "RECEIVER-CAPABILITY", "SPN-REFS", "NETWORK-CONFIGURATIONS", "LINKER", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF", "OP", "SOMEIP-REMOTE-MULTICAST-CONFIG", "RPT-SERVICE-POINT", "SPECIFICATION-DOCUMENT-SCOPES", "SDG-CONSTRAINT-REF", "STARTUP-OPTIONS", "START-ON-EVENT-REF", "P-2-STAR-SERVER-MAX", "TRCV-PWM-MODE-ENABLED", "TCP-ALIVE-CHECK-RESPONSE-TIMEOUT", "CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL", "MAY-BE-UNCONNECTED", "ROOT-SOFTWARE-COMPOSITIONS", "DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL", "TARGET-SWC-SERVICE-DEPENDENCY-REF", "RAM-BLOCK", "FILE-REFS", "BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING", "MC-FUNCTION-REFS", "VG", "NETWORK-CONFIGURATION-REF", "FMI", "MAXIMUM-SEGMENT-LENGTH-RESPONSE", "DDS-RPC-SERVICE-REF", "INTERNAL-BEHAVIOR", "INSTALLATION-BEHAVIOR", "REPORT-BEHAVIOR", "PERSISTENCY-FILE-ARRAY", "EVENT-GROUP-REFS", "FRAGMENT", "TARGET-VARIABLE-ACCESS-REF", "SYMBOLIC-NAME-PROPS", "TIME-SEG-2", "PORT-ADDRESS", "NM-CHANNEL-ACTIVE", "REMOTE-SERVER-CONFIG", "LOCAL-COM-ACCESS-CONTROL-ENABLED", "ENTRY-TYPE", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "CRYPTO-ALGORITHM-STRING", "NODE-NAME", "ENTER-TIMEOUT-VALUE", "ERROR-TRACER-NEEDS", "DTCS", "ETHERNET-CLUSTER-VARIANTS", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REF", "NETWORK-MANAGEMENT-VECTOR-LENGTH", "CONTROL-PNC-REF", "SIMULATED-EXECUTION-TIME", "MAPPED-AP-DATA-ELEMENT-IREF", "POSITION", "INDIRECT-API", "DEFAULT-GATEWAY", "P-PORT-IN-COMPOSITION-INSTANCE-REF", "TIMING-DESCRIPTION-EVENT-CHAIN", "NV-DATA-INTERFACE", "DIAGNOSTIC-TEST-RESULT-REFS", "FM-FEATURE-MAP-ELEMENT", "PATTERN-JITTER", "SW-RECORD-LAYOUT-COMPONENT", "ECUC-DEFINITION-COLLECTION", "SLAVE-EVENT-REF", "SECURE-PROPS-FOR-TCP-REF", "IMPLEMENTATION-RECORD-ELEMENT-REF", "CLIENT-SERVER-PORTS", "SERVICE-INTERFACE-MAPPING-SET", "OBD-RATIO-DENOMINATOR-NEEDS", "PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE", "RPT-EXECUTION-CONTEXT", "CRYPTO-PROVIDER-DOCUMENTATION-REF", "RPT-IMPL-POLICY", "CONTENT-URI", "MAX-ERROR-STATE-INVALID", "PNC-MAPPING-REF", "TTL-MAX", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "CLEAR-CONDITION-REFS", "LOCAL-SUPERVISIONS", "ACCEPT-ENCODING", "TA-TYPE", "SERVICE-OFFER-TIME-TO-LIVE", "RESPONSE-ON-EVENT-SCHEDULER-RATE", "GETTER-CALL-REF", "ECU-TIMING", "TRAFFIC-LIMITATION-FILTERS", "ASSIGNED-LIN-SLAVE-CONFIG-REF", "CONTEXT-DATAS", "SW-RECORD-LAYOUT-V", "FIRST-DATA-PROTOTYPE-REF", "SIGNAL-FAN", "FEATURE-REFS", "CLIENT-SERVER-ANNOTATION", "METHOD-IREF", "IMPLEMENTED-ENTRY-REF", "EVENT-NEEDS-REF", "CONTEXT-PORT-PROTOTYPE-REF", "RANGE-MIN", "ID", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "BSW-MODULE-ENTRY", "FIRE-AND-FORGET", "DIAGNOSTIC-COMMON-PROPS-VARIANTS", "SW-BASE-TYPE", "BSW-MODULE-DESCRIPTION-REF-CONDITIONAL", "DLT-LOG-SINK-REF", "SCHEDULER-MAX-NUMBER", "ACCEPTED-REMOTE-CERTIFICATE-REFS", "PDU-MAX-LENGTH", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "NUMBER-OF-INSTRUCTIONS", "VARIANT-CRITERION-REF", "NUMBER-OF-CONNECTED-CLIENTS", "PRE-MAPPING", "TCP-PORT-REF", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF", "DID-NUMBER", "MAX-CYCLES", "SUB", "TARGET-OPERATION-REF", "CLAIMED-FUNCTION-GROUP-REFS", "MODIFIED-DATAS", "MAX-COMM-MODE", "SEC-OC-JOB-SEMANTIC", "TRANSFORMATION-PROPS-SET", "SENDER-RECEIVER-INTERFACE", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "KEY-STORAGE-TYPE", "IS-EXTERNAL", "FREEZE-CURRENT-STATE-SUPPORTED", "DIAGNOSTIC-MESSAGE-TYPE", "REST-NUMBER-PROPERTY-DEF", "TLS-CONNECTION-REFS", "TWO-KEY-SLOT-MODE", "INTERNAL-TRIGGERING-POINTS", "SW-COMPONENT-MAPPING-CONSTRAINTS", "CRC-DOMAIN-NUMBER", "BYTE-ORDER", "DIAGNOSTIC-EVENT-WINDOW", "REST-ENDPOINT-ARGUMENT", "DIRECT-TP-SDU-REF", "SOMEIP-SERVICE-VERSION", "PROTOCOL-VERSION", "HW-PIN-GROUP", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "STATE-DEPENDENT-STARTUP-CONFIG", "SDU-TYPE", "IMPLEMENTATION-DATA-TYPE", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "SERVICE-INSTANCE-REFS", "DEPENDENCY", "SOMEIP-RULE", "PORT-BLUEPRINT-INIT-VALUE", "UDP-TTL", "REQUEST-DOWNLOAD-CLASS-REF", "TIME-HARDWARE-CORRECTION-THRESHOLD", "SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION", "BURST-PATTERN-EVENT-TRIGGERING", "I-PDU-MAPPINGS", "NM-NODE-DETECTION-ENABLED", "END-TO-END-PROTECTIONS", "MODE-DEPENDENT-STARTUP-CONFIGS", "SWC-TO-ECU-MAPPING", "TRANSPORT-PROTOCOL", "TP-CHANNEL-REF", "RPT-SW-PROTOTYPING-ACCESS", "MINIMUM-MEMORY-CONSUMPTION", "MODE-DECLARATION-MAPPING-SET", "TP-CONNECTION-CONTROL-REF", "ALL-CHANNELS-REFS", "TARGET-EEC-REF", "ECUC-STRING-PARAM-DEF", "MC-VARIABLE-INSTANCES", "DIAGNOSTIC-MONITOR-INTERFACE", "END-TO-END-TRANSFORMATION-DESCRIPTION", "TRANSFORMATION-I-SIGNAL-PROPSS", "FIRST-ELEMENT-INSTANCE-IREFS", "CONDITION-FORMULA", "TCP-IP-NDP-DEFAULT-RETRANS-TIMER", "JUMP-TO-BOOT-LOADER", "MAX-ALIVE-CYCLE", "MODE-DECLARATION-GROUP-REF", "SUB-DOMAIN-REF", "TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT", "PARAMETER-PORT-ANNOTATION", "RESOURCE-IDENTIFIER-TYPE", "PACKAGER-ID", "BSW-OPERATION-INVOKED-EVENT", "SOCKET-ADDRESS-REF-CONDITIONAL", "SHUTDOWN", "CRC-MESSAGE-LENGTH", "REQUESTABLE", "SECURITY-EVENTS", "ECUC-COND", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "OPERATION-CYCLE-AUTOMATIC-END", "SDG-TAILORINGS", "CAN-FRAME-TX-BEHAVIOR", "WAKEUP-FRAME-REF", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS", "PHYSICAL-CHANNEL-REF-CONDITIONAL", "PROPERTYS", "MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID", "MINIMUM-SEPARATION-TIME", "ANNOTATION-ORIGIN", "COUPLED-CLUSTER-REF", "LATEST-TX", "BSW-TIMING-EVENT", "SUB-GROUP-REF", "CORE-ID", "INVERSE-PROTOCOL-VERSION", "MAX-ERROR-STATE-VALID", "NETWORK-CONFIGURATION-REFS", "STIMULUS-REF", "NATIVE-DECLARATION", "J-1939-TP-CONNECTION", "ORDERED-ELEMENTS", "IDENTITY-CERTIFICATE-AUTHORITY-REF", "SIZE-OF-STRUCT-LENGTH-FIELDS", "LICENSE-REFS", "TCP-IP-DHCP-V-6-INF-DELAY-MAX", "FUNCTION-ID", "CAN-ENTERS", "CAN-TP-CHANNEL-REF", "DIAGNOSTIC-DATA-ELEMENT", "GLOBAL-ELEMENTS", "PERIODIC-RATE-CATEGORY", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "REPLACE-WITH", "MC-SW-EMULATION-METHOD-SUPPORT", "LIN-ORDERED-CONFIGURABLE-FRAME", "PROCESS-DESIGN", "CONTEXT-COMPONENT-REFS", "CAN-ID-RANGE-MAPPINGS", "TROUBLE-CODE-OBD-REF", "FLOW-LABEL", "SA-REKEY-TIME", "MATCHING-RULE-REFS", "COMM-CONNECTORS", "EXPECTED-ALIVE-CYCLE", "AUTOSAR-PARAMETER-IREF", "ECUC-DEF-EDITION", "MAPPED-BSW-SERVICE-DEPENDENCY-REF", "SWC-SERVICE-DEPENDENCY", "OBD-SUPPORT", "REMOTE-ADDRESSS", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "FRAME-PORT", "MAC-SEC-PROPS", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "PROVIDED-ENTRYS", "GLOBAL-IN-PACKAGE-REFS", "CONSTANT-SPECIFICATION", "MODIFICATION", "DYNAMIC-ACTION-REF", "ANNOTATION", "TCP-SYN-RECEIVED-TIMEOUT", "PROVIDED-SERVICE-INSTANCES", "SIGNAL-SERVICE-TRANSLATION-PROPS", "LENGTH-VERIFICATION", "RELIABILITY", "SW-AXIS-INDIVIDUAL", "SERVICE-DISCOVER-CONFIGS", "POLICY-ACTION", "RPT-WRITE-ACCESS", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "UPSTREAM-REFERENCE-IREF", "COM-MANAGEMENT-PORT-GROUP-IREFS", "MEMORY-DESTINATION-REFS", "UCM-SOFTWARE-VERSION", "BLUEPRINT-CONDITION", "HANDLE-NEVER-RECEIVED", "RPT-COMPONENT", "SW-FEATURE-DEFS", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE", "MC-FUNCTION-REF", "SUBNET-NUMBER", "ACCESSED-VARIABLE-REF", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "ICMP-V-4-PROPS", "N-DATA-SETS", "SECURITY-LEVEL-REFS", "TCP-OPTION-FILTER-LIST", "KEY-SLOT-CONTENT-ALLOWED-USAGES", "REFERENCE-TABLE-REF", "THRESHOLD-NUMBER", "SECTION-NAME-PREFIXS", "CAN-CLUSTER-CONDITIONAL", "DEFAULT-VLAN-REF", "CAN-NM-ECU", "JOB-REF", "STANDARD-REVISIONS", "COMMUNICATION-CLUSTER-REFS", "COMPOSITE-TYPE-MAPPINGS", "TLS-JOB-SEMANTIC", "SERVICE-INSTANCE-REF", "REQUIREMENT-REFS", "CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE", "ARTIFACT-CHECKSUM-REF", "COUPLING-PORT-SWITCHOFF-DELAY", "DIAGNOSTIC-INDICATOR-NEEDS", "ALIAS-EVENT-REF", "MAX-NO-NEW-OR-REPEATED-DATA", "DIAGNOSTIC-EVENTS", "PHM-CONTRIBUTION-REF", "RUNNABLE-MAPPINGS", "INFRASTRUCTURE-SERVICES", "TCP-KEEP-ALIVE-PROBES-MAX", "LOCAL-CERTIFICATE-REFS", "NM-REPEAT-MESSAGE-BIT-ACTIVE", "TIMING-MODE-REF", "REQUIRED-PACKAGE-ELEMENT-REF", "DIAGNOSTIC-DATA-TRANSFER", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "CUSTOM-SERVICE-ID", "SIZE-OF-UNION-TYPE-SELECTOR-FIELD", "IDS-MGR-NEEDS", "RESOURCES", "PNC-GROUP-REF", "ECUC-MODULE-DEF", "CONSTANT-REF", "TIMING-CLOCKS", "POWER-DOWN-TIME", "BLUEPRINT-POLICY-SINGLE", "MAXIMUM-SUPPORTED-UCM-MASTER-VERSION", "I-PDU-PORT-REF", "DLT-ARGUMENT", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "MODE-DECLARATION-GROUP-PROTOTYPE-REF", "SIGNAL-GROUP-REF", "SUPPORTED-FEATURES", "LIN-ORDERED-CONFIGURABLE-FRAMES", "MULTIPLICITY-RESTRICTION", "BUS-DEPENDENT-NM-ECUS", "HEALTH-CHANNEL-ARGUMENT-REFS", "ENVIRONMENT-VARIABLES", "REST-SERVICE-INTERFACE", "ECUC-FOREIGN-REFERENCE-DEF", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "REKEY-INTERVAL", "SOFTWARE-ACTIVATION-DEPENDENCY-REF", "ENABLE-UPDATE", "TRANSFORMER-STATUS-FORWARDING", "CONTEXT-OPERATION-REF", "DATA-TYPE-REFS", "SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "COUPLING-PORT-ROLE", "TCP-MSL", "DATA-CONSTR-RULES", "UPDATE-SESSION-REJECTED", "BSW-ENTRY-KIND", "EXECUTION-ORDER-CONSTRAINT-TYPE", "CODE", "MIN-MARGIN", "VALID-BINDING-TIME", "LOCAL-ENDPOINT-CONFIG", "OPERATION-ARGUMENT-INSTANCE-IREF", "TCP-CONGESTION-AVOIDANCE-ENABLED", "RAPID-PROTOTYPING-SCENARIO", "UCM-REF", "RETURN-VALUE-PROVISION", "TARGET-VARIABLE-DATA-PROTOTYPE-REF", "MIN-RPT-EVENT-ID", "PREVIOUS-DATA-TYPE-REF", "AGING-REF", "POSITIVE-INTEGER-VALUE-VARIATION-POINT", "CONTAINED-AR-ELEMENT-REF", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS", "ARRAY-SIZE-SEMANTICS", "PDU-TRIGGERINGS", "VALIDATION-CONFIGURATION", "TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED", "DO-IP-NETWORK-CONFIGURATION", "VLAN-REFS", "L-1", "TARGET-CHANNEL", "CONTEXT-ATOMIC-COMPONENT-REF", "INVERT-CONDITION", "SIGNAL-MAPPINGS", "FIRST-TIMEOUT", "DEBOUNCE-ALGORITHM-PROPSS", "COLLISION-RESOLVING-SCHEDULE-REF", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS", "VERIFICATION-HASH", "DIAGNOSTIC-ECU-RESET", "SPORADIC-EVENT-TRIGGERING", "DIAGNOSTIC-IUMPR", "TCP-TP-PORT", "FRAME-PIDS", "FRAME-TRIGGERING-REF", "BSW-MODE-MANAGER-ERROR-EVENT", "RECORD-ELEMENT-MAPPINGS", "SWC-MODE-SWITCH-EVENT", "FIELD-MAPPING", "MAX-FC-WAIT", "INTENDED-PARTIAL-INITIALIZATION-COUNT", "REMOTE-UNICAST-ADDRESSS", "USED-DATA-ELEMENT", "REQUIRED-ARTIFACTS", "I-PDU-IDENTIFIER-UDP-REFS", "FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL", "FREEZE-FRAME-CONTENT-REF", "IDSM-RATE-LIMITATION-REF-CONDITIONAL", "REST-ENUMERATOR-DEF", "TIME-FAILED-THRESHOLD", "USER-DEFINED-COMMUNICATION-CONNECTOR", "FIREWALL-STATE-MACHINES", "SWC-MAPPING-CONSTRAINT-REF", "GLOBAL-SUPERVISION-NEEDS", "EXECUTABLE-PROVIDED-PORT-IREF", "PHM-SUPERVISED-ENTITY-INTERFACE", "DLT-LOG-CHANNELS", "TAG-WITH-OPTIONAL-VALUE", "DETERMINISTIC-SYNC-M-OUT-OF-N", "SOMEIP-METHOD", "USER-DEFINED-PDU", "REGULAR-EXPRESSION", "ETHERNET-RAW-DATA-STREAM-GRANT", "DLT-MESSAGE-COLLECTION-SET", "MAC-LAYER-TYPE", "R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF", "UPDATE-KIND", "TARGET-SIGNAL-REF", "STATE-MANAGEMENT-REQUEST-TRIGGER", "TCP-RULE", "IDSM-MODULE-INSTANTIATION-REF", "ABS", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "TCP-IP-NDP-PACKET-QUEUE-ENABLED", "READ-LOCAL-VARIABLES", "PRODUCT-RELEASE", "SUPPORTED-CIPHER-SUITES", "RPT-EVENT-ID", "LISTEN-TIMEOUT", "BSW-MODULE-DESCRIPTION", "VARIABLE-INSTANCE", "HW-COMMUNICATION-CONTROLLER-REF", "APPLICATION-MODE-MACHINE", "AR-TYPED-PER-INSTANCE-MEMORYS", "MAX", "ADDRESS", "SUPERVISED-ENTITY-IREF", "LICENSE-REF", "PNC-WAKEUP-CAN-ID-EXTENDED", "SCALE-CONSTR", "HW-CATEGORY", "DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL", "MANAGED-MODE-GROUPS", "DEFAULT-VALUE-ELEMENT", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "MAX-NUMBER-OF-NPDU-PER-CYCLE", "PORT-GROUPS", "EVENT-DISPLACEMENT-STRATEGY", "SOURCE-LIN-PID-REF", "HW-COMMUNICATION-PORT-REF", "TEXT-VALUE-SPECIFICATION", "SWC-EXCLUSIVE-AREA-POLICY", "EVENT-WINDOWS", "FEATURE-MODEL-REF", "SEC-OC-COM-PROPS-FOR-MULTICAST-REF", "ASSIGNED-DATAS", "MEMORY-RANGE-REF", "ABSOLUTE", "METADATA-PROTECTION-KIND", "HARDWARE-ELEMENT-REF", "CONFIG-ELEMENT-DEF-LOCAL-REF", "REFERENCE-REF", "VEHICLE-ANNOUNCEMENT-COUNT", "OPT-IMPL-RECORD-ELEMENT", "LIFE-CYCLE-INFO", "J-1939-NM-NODE", "PER-INSTANCE-PARAMETERS", "SECOND-ELEMENT-INSTANCE-IREFS", "DIAGNOSTIC-STORAGE-CONDITION", "ADDITIONAL-INFORMATION", "FIFO-RANGES", "DIRECTION", "PID-SIZE", "INCLUDED-DATA-TYPE-SET", "MAX-NUMBER-OF-ALLOWED-UPDATES", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS", "TLS-CIPHER-SUITES", "ELEMENT-GROUPS", "UCM-MODULE-INSTANTIATION", "TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP", "SUCCESSOR-REF", "CALLBACK-HEADER-REFS", "ENABLE-CONDITIONS", "NM-STATE-CHANGE-IND-ENABLED", "NM-PNC-PARTICIPATION", "NAS-TIMEOUT", "SOURCE-IP-ADDRESS", "SDG-DEF", "INVOCATION", "CONTEXT-DATA-PROTOTYPE-REFS", "ADAPTIVE-FIREWALL-MODULE-INSTANTIATION", "I-SIGNAL-MAPPING", "MKA-PARTICIPANTS", "SENDER-REC-ARRAY-ELEMENT-MAPPING", "MODE-SWITCH-POINTS", "ENVIRONMENT-CAPTURE-TO-REPORTING", "REST-ENDPOINT-PUT", "NETWORK-TIME-MASTER-REF", "RPT-CONTAINERS", "I-SIGNAL-TO-PDU-MAPPINGS", "ENABLE-DEACTIVATION", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL", "RPT-SERVICE-POINTS", "APPLICATION-ENTRY", "MASTER-REF", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "ACCESS-PERMISSION-REF", "SD", "DLT-CONTEXT-REF", "I-PDU-TRIGGERING", "NOTIFICATIONS", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "OPTION-KIND", "APPLICATION-ARRAY-DATA-TYPE", "DATA-PROTOTYPES", "DATA-UPDATE-PERIOD", "LIN-CONFIGURABLE-FRAMES", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF", "SUB-NODE-CHANNEL-REF", "R-PORT-PROTOTYPE", "NM-NODE-ID-ENABLED", "INSTANTIATION-TIMING-EVENT-PROPS", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "BSW-MODULE-ENTRY-REF", "EMPTY-SIGNAL", "POST-BUILD-VARIANT-VALUE", "UDP-PORT-REF", "TIME-VALUE", "NUMERICAL-RULE-BASED-VALUE-SPECIFICATION", "SERVICE-INSTANCE-ID", "ITEM-DEFINITIONS", "RX-NM-PDU-REFS", "COMMUNICATION-CONTROLLER-REF", "IDSM-INSTANCE-ID", "ANNOTATION-TEXT", "TIME-SYNC-SERVER", "MAX-BASE-TYPE-SIZE", "BSW-SCHEDULER-NAME-PREFIX", "TCP-IP-ARP-TABLE-ENTRY-TIMEOUT", "EVENT-CLEAR-ALLOWED", "BSW-INTERRUPT-ENTITY", "ON-TRANSITION-VALUE", "INGRESS-PRIORITY", "SECURITY-ACCESS-CLASS-REF", "PDELAY-RESPONSE-ENABLED", "DIAGNOSTIC-PARAMETER-REF", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "AUTOSAR-VARIABLE-INSTANCE", "WAKEUP-SYMBOL-RX-WINDOW", "HW-PORT-MAPPINGS", "DATA-PROTOTYPE-MAPPING", "MAX-DID-TO-READ", "NOTIFIER-REF", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS", "UDP-COLLECTION-BUFFER-TIMEOUT", "BSW-IMPLEMENTATION", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "CAN-COMMUNICATION-CONTROLLER-VARIANTS", "DISCOVERY-TECHNOLOGY", "FM-FEATURE-DECOMPOSITION", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "DIAGNOSTIC-CONTROL-NEEDS", "FUNCTIONAL-UNIT", "SYNC-TIME-BASE-MGR-USER-NEEDS", "I-SIGNAL", "I-SIGNAL-I-PDU-REF-CONDITIONAL", "COMPILERS", "VFC-IREFS", "UDP-REMOTE-PORT", "MAX-DELTA-COUNTER", "TCP-RECEIVE-WINDOW-MAX", "VEHICLE-DRIVER-NOTIFICATION", "SUPPLIER-ID", "TRANSMISSION-PROPS", "START-ADDRESS", "INTERFACE-MAPPING", "TRIGGER-REF-CONDITIONAL", "IS-MANDATORY", "GLOBAL-TIME-DOMAIN-PROPERTYS", "KEY-EXCHANGE-AUTHENTICATION-REF", "REFERENCE-TAILORING", "TCP-IP-IP-REASSEMBLY-TIMEOUT", "CODE-DESCRIPTORS", "APPLICATION-TYPE-REF", "FOLLOW-UP-ACTION-REF", "COMMUNICATION-MODE", "CLIENT-SERVER-INTERFACE-MAPPING", "ARRAY-VALUE-SPECIFICATION", "COMPOSITE-NETWORK-REPRESENTATIONS", "SUBFUNCTIONS", "J-1939-REQUESTABLE", "DDS-SECURE-GOVERNANCE", "SERVICE-INTERFACE-SUB-ELEMENT", "DIAGNOSTIC-PARAMETER", "FREEZE-FRAME-CONTENT-WWH-OBD-REF", "SOCKET-CONNECTION-REF", "HTTP-ACCEPT-ENCODING", "SUPPORTING-MATERIAL", "REFERENCE-VALUE-SPECIFICATION", "TARGET-DATA-PROTOYPE-REF", "ARGUMENT-REF", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "XDOC", "ARGUMENT-MAPPINGS", "PARTIAL-NETWORK-REFS", "USED-SECONDARY-FID-REFS", "MESSAGE-ID", "REMAPPED-CAN-ID", "APPLICATION-COMPOSITE-ELEMENT-IREF", "CONDITIONAL-CHANGE-NAD", "REFERENCE-BASE", "EXCLUSIVE-AREA-NESTING-ORDER", "MAX-PWM-L", "ACTION-POINT-OFFSET", "LIN-UNCONDITIONAL-FRAME", "DATA-PROTOTYPE-IN-PORT-INTERFACE-REF", "COUPLING-PORT", "FUNCTION-GROUP-MODE-IREFS", "CONTEXT-ID", "NOMINAL", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "DYNAMIC-RUNTIME-LENGTH-HANDLING", "INSTANCE-ID", "FRAME-PID", "DLT-MESSAGES", "RPT-EXECUTABLE-ENTITYS", "GLOBAL-TIME-GATEWAY", "EXPLICIT-RPT-PROFILE-SELECTION-REFS", "DETERMINISTIC-CLIENT-REF", "LOAD-BALANCING-WEIGHT", "USED-PIM-REF", "PRECONFIGURED-CONFIGURATION-REFS", "AFFECTED-BSW-MODULE", "REVISION-LABEL-P-2", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "ANALYZED-EXECUTION-TIME", "DIAGNOSTIC-PROTOCOL", "TARGET-DATA-PROTOTYPE-GROUP-REF", "SDG-TAILORING", "USE-INSTEAD-REFS", "RESPONSES", "ETHERNET-NETWORK-CONFIGURATION", "REQUIRED-COM-SPECS", "SESSION-HANDLING", "OFS-SUB-TLV", "FOLLOW-UP-ACTION-REFS", "CAN-TP-ADDRESS-REFS", "PROVIDER-SOFTWARE-CLUSTER-REF", "DEPENDS-ON-REFS", "BASIC-CYCLE-LENGTH", "BSW-CALLED-ENTITY", "GET", "COMMUNICATION-CONNECTOR-REF-CONDITIONAL", "CLIENT-SERVICE-MAJOR-VERSION", "RESOURCE-DEFINITIONS", "MANAGED-PHYSICAL-CHANNEL-REFS", "RAW-DATA-STREAM-INTERFACE-REF", "ASSIGNED-CONTROLLER-REF", "OPTION", "COMPU-INTERNAL-TO-PHYS", "BSW-MODULE-TIMING", "PNC-MAPPING-REFS", "MEASURED-HEAP-USAGE", "REQUIRED-TRIGGERS", "DATA-CONSTR-REF", "FLEXRAY-FIFO-CONFIGURATION", "SAVE-CONFIGURATION-ENTRY", "RESOURCE-GROUP-REF", "SYSTEM", "USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL", "NM-CHANNEL-SLEEP-MASTER", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "CONSTRAINT-REF", "RPT-EXECUTION-CONTROL", "ECUC-STRING-PARAM-DEF-CONDITIONAL", "RESOURCE-CONSUMPTION", "UNASSIGNED-FRAME-TRIGGERING-REF", "GLOBAL-TIME-DOMAIN-REF-CONDITIONAL", "SW-DATA-DEPENDENCY", "DEPENDENCIES", "BYPASS-ETHER-TYPE", "NO-CHECKPOINT-SUPERVISION", "PROVIDED-DDS-SERVICE-INSTANCE", "PADDING-ACTIVATION", "SELECTIONS", "PORT-PROTOTYPE-REF", "REFERENCE-BASES", "ACTIVATION-ACTION", "VARIATION-POINT-PROXY", "EXCLUSIVE-AREA-REF", "COM-OFFER-SERVICE-GRANT-DESIGN", "SLOT-TYPE", "MINIMUM-DISTANCES", "MAC-SEC-KAY-PARTICIPANT", "SCOPE-REFS", "TIME-SYNC-CLIENT", "FRAME-MAPPING", "CHECKPOINT-ID", "PORT-CONFIGS", "TRANSFORMER-HARD-ERROR-EVENT", "SYMBOL-WINDOW-ACTION-POINT-OFFSET", "POST-BUILD-VARIANT-CRITERION-VALUE", "SOFTWARE-CLUSTER-DEPENDENCY-FORMULA", "ASSOCIATED-RTE-PLUGIN-REF", "KEY-SLOT-USED-FOR-SYNC", "TARGET-MODE-DECLARATION-REF", "MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING", "WINDOW-SIZE-INIT", "IKE-CIPHER-SUITE-NAME", "BSW-INTERNAL-TRIGGERING-POINT-REF", "CHECK-STATIC-BLOCK-ID", "PRESTORAGE-FREEZE-FRAME", "MSG-ID-MATCH", "BYPASS-VLANS", "SERVICE-ELEMENT-MAPPING-REF", "TIME-EXP", "PDU-TO-FRAME-MAPPINGS", "TD-EVENT-MODE-DECLARATION", "CRYPTO-ALG-ID", "NO-CHECKPOINT-SUPERVISIONS", "ECUC-CHOICE-CONTAINER-DEF", "CHECKPOINT-IREF", "SUPPLIER-VALIDATION-ORDER-REFS", "OFNS-DATA-ID-LISTS", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "MEMORY-HIGH-ADDRESS-LABEL", "DIAGNOSTIC-COM-CONTROL-CLASS", "NM-NODE-REF", "HW-CATEGORY-REF", "SESSION-HANDLING-SR", "IDENTICAL-MAPPING", "OPERATION-PROTOTYPE-IREF", "TP-SDU-REF", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "UNRESOLVED-REFERENCE-RESTRICTION", "BSW-RELEASED-TRIGGER-POLICY", "HEALTH-CHANNEL-SUPERVISION", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "SEC-OC-COM-PROPS-FOR-MULTICAST-REFS", "GLOBAL-TIME-CORRECTION-PROPS", "ROLE-BASED-DATA-TYPE-ASSIGNMENT", "ALLOW-HALT-DUE-TO-CLOCK", "REMOTE-ACCESS-CONTROL-ENABLED", "MCD-IDENTIFIER", "TLS-DEPLOYMENT", "TRANSMIT-PDU-REFS", "I-PDU-TRIGGERING-REF", "VARIATION-POINT", "TLV-DATA-ID-0-REF", "NM-CAR-WAKE-UP-RX-ENABLED", "TIMING-GUARANTEES", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "PRIVACY-LEVEL", "TAILORING-REFS", "TARGET-I-PDU-REF", "NUM-FAILED-SECURITY-ACCESS", "WWH-OBD-DTC-CLASS", "EVENT-COMBINATION-REPORTING-BEHAVIOR", "SUB-SOFTWARE-CLUSTER-REFS", "VLAN-PRIORITY", "DESTINATION-URI-POLICY", "DISPLAY-IDENTIFIER", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "TIMESTAMP-SUPPORT", "TRIGGER-PORT-ANNOTATIONS", "ETH-TP-CONNECTION", "TARGET-POSSIBLE-ERROR-REF", "SW-DATA-DEF-PROPS-CONDITIONAL", "LONG-NAME", "MODE-SWITCH-POINT", "START-ACTION-REF", "FUNCTION-GROUP-PHM-STATE-REFERENCE", "SENDER-REC-RECORD-ELEMENT-MAPPING", "TD-PDU-TRIGGERING-FILTER-REF", "DOCUMENT-ELEMENT-SCOPES", "END-2-END-EVENT-PROTECTION-PROPS", "NUMBER", "P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "DIAGNOSTIC-ENV-CONDITION-FORMULA", "POST-BUILD-VALUE-ACCESS-REF", "MEM-CLASS-SYMBOL", "DATA-TRANSFORMATION-REF", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "AREA", "MODE-IREF", "ECU-RESOURCE-ESTIMATION", "RATE-CORRECTIONS-PER-MEASUREMENT-DURATION", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "REQUIRED-DDS-EVENT-QOS-PROPS", "BURST-SIZE-RESPONSE", "SCOPE-REF", "DATA-ID-MODE", "CONTEXTS", "STARTUP-CONFIG-SET", "TRANSFER-REF", "CLIENT-SERVER-ARRAY-ELEMENT-MAPPING", "PROVIDED-OUTER-PORT-REF", "AUTH-INFO-TX-LENGTH", "MINIMUM-EXECUTION-TIME", "APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION", "RESOLUTION", "ENABLE-JOIN-ACCESS-CONTROL", "SELECTOR-FIELD-START-POSITION", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "STORE-IMMEDIATE", "SEPARATED-COMPONENT-IREF", "ENTERED-MODE-REF", "INDICATIONS", "SUBSCRIBE-EVENTGROUP-RETRY-DELAY", "CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "DEFAULT-VALUE", "BUILD-ACTION", "DO-IP-LOGIC-ADDRESS-PROPS", "COUPLING-PORTS", "LIN-TP-NODE", "TO-REF", "IMMEDIATE-RESUME-TIME", "PLATFORM-ACTION-ITEM", "RUNNABLE-ENTITY-GROUP", "CSE-CODE", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF", "SUPERVISION-ENTITYS", "FLAT-MAP-ENTRY-REFS", "DLT-SESSION-ID", "STORE-EVENT-ENABLED", "APPLICATION-CONTEXT-REF", "INFORMAL-FORMULA", "DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF", "I-SIGNAL-PORT-REFS", "BINARY-MANIFEST-ITEM-NUMERICAL-VALUE", "USE-CRC-COMP-MECHANISM", "MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL", "DO-IP-TESTER-ROUTING-ACTIVATION-REFS", "EXECUTION-CONTEXTS", "FIELD-GET-SET-QOS-PROPSS", "DTC-STATUS-MASK", "ROUTINE-CONTROL-CLASS-REF", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "ALLOWED-KEYSLOT-USAGE", "ACTIONS", "SENDER-ANNOTATION", "TX-NM-PDU-REFS", "J-1939-TP-CONFIG", "AGGREGATION-CONDITION", "APPLICATION-RECORD-ELEMENT-REF", "TCP-KEEP-ALIVE-TIME", "BSW-ENTRY-RELATIONSHIP-SET", "RESPONSE-ON-EVENT-CLASS-REF", "PDELAY-LATENCY-THRESHOLD", "PIN-NUMBER", "DOCUMENTATION-CONTEXT", "I-SIGNAL-TRIGGERING-REF", "CLIENT-ID-RANGE", "TD-EVENT-FR-CLUSTER-CYCLE-START", "CONSTR-LEVEL", "I-PDU-TIMING-SPECIFICATIONS", "SWC-BSW-MAPPING", "DATA-TYPE-MAPPING-SET", "CIPHER-SUITE-ID", "ARRAY-IMPL-POLICY", "TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF", "OS-TASK-PROXY-REF", "DATA-CONSISTENCY-POLICY", "CERTIFICATE-REF", "ACL-PERMISSION", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "PULSE-TEST", "DEBOUNCE-COUNTER-STORAGE", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF", "PRM-CHAR", "PROPABILITY", "IPV-4-CONFIGURATION", "SW-SYSTEMCONST", "COMPOUND-PRIMITIVE-ARGUMENTS", "SECURITY-EVENT-CONTEXT-PROPS", "SERVICE-CONTROL", "COMPU-SCALE", "CAN-PHYSICAL-CHANNEL", "OBD-PID-SERVICE-NEEDS", "DEF-LIST", "LC-STATE-REF", "SEPARATED-COMPONENT-IREFS", "SUB-ELEMENTS", "IMPLEMENTS-SOMEIP-STRING-HANDLING", "SW-CALPRM-IMPLS", "SERVICE-INTERFACE-EVENT-MAPPING", "PROVIDED-INTERFACE-TREF", "IDENTITY", "RX-ACCEPT-CONTAINED-I-PDU", "DTC-PROPS-REF", "PROCESSOR-CORE", "NM-NODE-REFS", "FLEXRAY-NM-CLUSTER-COUPLING", "LIN-SLAVE-CONFIG", "RESPOND-TO-RESET", "TEAR-DOWN-ACTION-REF", "MULTICAST-UDP-PORT", "VTF", "MINIMUM-SUPPORTED-UCM-MASTER-VERSION", "PER-INSTANCE-MEMORY-SIZE", "BUS-IDLE-TIMEOUT-PERIOD", "PHYSICAL-PROPS", "SEGMENT-BYTE-ORDER", "ENABLE-LIVELINESS-PROTECTION", "SW-CARB-DOCS", "MACHINE-TIMING", "FIELD-TOPICS-ACCESS-RULE-REF", "TRANSMISSION-MODE", "REQUIRED-PACKAGE-ELEMENT-REFS", "LC-OBJECT-REF", "NV-PROVIDE-COM-SPEC", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "SIZE-OF-STRING-LENGTH-FIELDS", "WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST", "FLEXRAY-NM-CLUSTER", "MAC-MULTICAST-ADDRESS", "MAX-PWM-O", "METHODS", "UCM-MASTER-FALLBACK-REFS", "PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL", "OS-TASK-PROXY", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "DOCUMENTATION", "RECEIVER-IREFS", "LIN-TP-N-SDU-REF", "CAN-CONTROLLER-FD-ATTRIBUTES", "DO-IP-TP-CONNECTION", "ALLOW-DYNAMIC-L-SDU-LENGTH", "CONTEXT-ELEMENT-REFS", "STATE-MANAGEMENT-TRIGGER-COMPARE-RULE", "FRESHNESS-VALUE-LENGTH", "SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS", "HAS-SETTER", "E-2-E-PROFILE-CONFIGURATION-SET", "SYNCHRONIZED-MASTER-TIME-BASE", "SEGMENTATION-SUPPORTED", "UDP-COLLECTION-TRIGGER", "TLV-DATA-ID-DEFINITION-SET", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "NV-BLOCK-DATA-MAPPINGS", "PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS", "FLAT-MAP-REF", "TIME-BASE-RESOURCE-REFS", "BLOCK-IF-STATE-ACTIVE-CP-REFS", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "SDU-REFS", "SIZE", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "BSW-MODE-GROUP-REF", "HOST-SYSTEM-REF", "MODE-SWITCH-RECEIVER-COM-SPEC", "DEFAULT-MODE-REF", "LDAP-URL", "TIMEOUT-BS", "MEASURED-EXECUTION-TIME", "FUNCTIONAL-REQUEST-REFS", "CONFIDENCE-INTERVAL", "AUTH-DATA-FRESHNESS-LENGTH", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "OPERATION-INVOKED-EVENT", "CONNECTION-IS-MANDATORY", "OS-MODULE-INSTANTIATION", "IO-CONTROL-CLASS-REF", "USE-SECURED-PDU-HEADER", "TCP-SLOW-START-ENABLED", "REST-SYSTEM-TRIGGERED-EVENT", "BINARY-MANIFEST-RESOURCE-DEFINITION", "MIN-PWM-O", "DEFAULT-VALUE-ELEMENTS", "LIN-EVENT-TRIGGERED-FRAME", "DLT-ECU-ID", "SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING", "DESTINATION-MAX-ADDRESS", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "DATA-ELEMENT-IREFS", "MASTERED-TRIGGER-REF", "COMPLEX-TYPE-MAPPING", "NESTED-ELEMENT-REFS", "MINIMUM-MULTICAST-SEPERATION-TIME", "POSSIBLE-ERROR-REFS", "FULL-DUPLEX-ENABLED", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "OBD-DTC-VALUE-3-BYTE", "SERVICE-INTERFACE-METHOD-MAPPING", "ETHERNET-COMMUNICATION-CONTROLLER", "MSR-QUERY-NAME", "SCHEDULING-PRIORITY", "AP-SOMEIP-TRANSFORMATION-PROPS", "CHECKPOINT-INITIAL-REF", "BLUEPRINT-DERIVATION-GUIDE", "TP-SDU-REFS", "TAILORING-REF", "RX-MASK", "TIME-MASTER-PRIORITY", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "CALIBRATION-PARAMETER-VALUE", "SYNCHRONIZED-TIME-BASE-CONSUMER", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "BUFFER-PROPERTIES", "USER-DEFINED-METHOD-DEPLOYMENT", "NM-MULTIPLE-CHANNELS-ENABLED", "SW-CALPRM-AXIS-SET", "MEMORY-USAGE", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE", "NM-CONTROL-BIT-VECTOR-ACTIVE", "ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF", "CRYPTO-SERVICE-PRIMITIVE", "COMPU-DEFAULT-VALUE", "GETTER-RETURN-SIGNAL-REF", "SECURITY-LEVEL-REF", "RULE-BASED-AXIS-CONT", "RESOURCE-GROUPS", "TARGET-PDU-TRIGGERINGS", "BLOCK-IF-STATE-ACTIVE-AP-IREFS", "PNC-WAKEUP-DLC", "KEYWORD-SET", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "TRANSFORMATION-TECHNOLOGY", "SOURCE-FRAME-REF", "ECUC-IMPLEMENTATION-CONFIGURATION-CLASS", "USER-DEFINED-I-PDU", "SDX-REF", "AUTHENTIC-CONNECTION-PROPSS", "NM-REMOTE-SLEEP-INDICATION-TIME", "CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS", "RESOURCE-REF", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "BINARY-MANIFEST-PROVIDE-RESOURCE", "DTC-FORMAT-TYPE", "MAX-RUN-CYCLE-LOOP", "CALLED-ENTRYS", "SOURCE-ADDRESS-MASK", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS", "TARGET-DATA-PROTOTYPE-IN-CS-REF", "LOG-TRACE-LOG-MODES", "STORE-EMERGENCY", "FIELD-IREF", "DATA-PDU-REF", "PER-INSTANCE-MEMORY-REF", "P-2-SERVER-MAX", "POST-BUILD-CHANGEABLE", "TARGET-MODE-GROUP-REF", "EVENT-QOS-PROPSS", "CONSIDER-BUILD-NUMBER", "SW-RECORD-LAYOUT-GROUP-TO", "SERVICE-BUSY", "PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "CHANNEL-MODE", "ROOT-DATA-PROTOTYPE-REF", "DIAGNOSTIC-TROUBLE-CODE-REF", "EXECUTABLE", "ALLOCATOR-REF", "I-PDU-PORT", "IS-REENTRANT", "CRYPTO-SERVICE-CERTIFICATE", "PROVIDED-REQUIRED-INTERFACE-TREF", "REVISION-LABEL-P-1", "VFC-IREF", "ARGUMENTSS", "TARGET-DATA-ELEMENT-REF", "SW-ADDR-METHOD", "ATTRIBUTE-REF", "EXIT-TIMEOUT-VALUE", "TIME-SYNC-CORRECTION", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL", "ACL-CONTEXTS", "GRAPHIC", "ENCODING", "NM-USER-DATA-ENABLED", "CP-SOFTWARE-CLUSTER-RESOURCE-REF", "STEP-SIZE", "LC-STATES", "COMPU-INVERSE-VALUE", "VALUE-TYPE-TREF", "COUPLING-PORT-STARTUP-ACTIVE-TIME", "TCP-REMOTE-PORT", "ACTION", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS", "FLEXRAY-FIFOS", "DESTINATION-URI-REFS", "FLEXRAY-TP-ECU", "TCP-IP-NDP-MAX-RTR-SOLICITATIONS", "PERSISTENCY-ACCESS", "OPT-ARGUMENT-REF", "FALSE-ACTION-LIST-REF", "SHORT-TERM-ADJUSTMENT", "STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM", "UDP-NM-CLUSTER-REF", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "ECU-INSTANCE", "TRANSMISSION-MODE-CONDITIONS", "NM-GLOBAL-COORDINATOR-TIME", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "JOB-PRIMITIVE-REF", "TIMING-ARGUMENTS", "TRANSFORMER-REF", "DO-IP-POWER-MODE-STATUS-NEEDS", "TLV-DATA-ID-DEFINITION-REF", "I-SIGNAL-PORT", "OPTION-NAME", "DERIVATION", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING", "SW-SYSCOND", "PHM-LOGICAL-EXPRESSION", "NUMBER-OF-RESTART-ATTEMPTS", "HEALTH-CHANNEL-ARGUMENT-REF", "INCLUDED-VARIANT-REF", "MSR-QUERY-RESULT-TOPIC-1", "TARGET-RUNNABLE-ENTITY-GROUP-REF", "SLEEP-MODE-EXECUTION-DELAY", "HW-PIN-GROUP-REF", "CUSTOM-SPECIFICATION-REF", "TRIGGER-PERIOD", "CRYPTO-PROVIDERS", "TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED", "COLLECTED-INSTANCE-IREF", "FM-FEATURE-MAP", "SEQUENCE-COUNTER", "SECURE-COM-PROPS-FOR-UDP-REFS", "FIRST-ELEMENTS", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "DIAGNOSTIC-SECURITY-ACCESS", "REMOTE-CERTIFICATE-REFS", "PARTITION-REF", "DIRECT-SUCCESSOR-REF", "TIMEOUT", "SECOND-ELEMENT-INSTANCE-IREF", "DHCP-ADDRESS-ASSIGNMENT", "SERVICE-INSTANCE-PORT-CONFIG", "PROCESS-ARGUMENT", "SW-COMPARISON-VARIABLES", "MAPPING-REF", "WAKEUP-FORWARD-REMOTE-ENABLED", "INTERNAL-TRIGGERING-POINT", "AUTOSAR-CONNECTOR", "SOFTWARE-CLUSTER-DESIGN-DEPENDENCY", "SOURCE-MIN-ADDRESS", "FM-FEATURE-MODEL", "RPT-ENABLER-IMPL-TYPE", "TERMINATING-CHECKPOINT-TIMEOUT-UNTIL-TERMINATION", "DOMAIN-IDS", "FAILED-SUPERVISION-CYCLES-TOLERANCE", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "SAK-REKEY-TIME-SPAN", "SW-COMPONENT-DOCUMENTATIONS", "ENABLE-ANYCAST", "CAN-TP-NODE", "ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT", "VIEW-MAPS", "READ", "VLAN-IDENTIFIER", "CHECKPOINT-REF", "SEARCH-BEHAVIOR", "REPETITION-PERIOD", "IP-V-6-PATH-MTU-ENABLED", "RECOMMENDED-CONFIGURATION-REFS", "CONFIGURED-NAD", "HEALING-CYCLE-COUNTER-THRESHOLD", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "SERVER-SERVICE-MAJOR-VERSION", "DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT", "CAT-2-ISR", "ALIVE-SUPERVISIONS", "SEC-OC-JOB-MAPPINGS", "FORMULA-CAPTION", "ECUC-ENUMERATION-LITERAL-DEF", "ELEMENT-BYTE-VALUE", "SW-RECORD-LAYOUT-GROUP-STEP", "SOURCE-INSTANCE-IREF", "NETWORK-INTERFACE-REF", "SW-GENERIC-AXIS-DESC", "METHOD-ID", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "CDD-TYPE", "INCLUDED-LIBRARY-REFS", "BSW-ENTRY-RELATIONSHIP-TYPE", "AUTHENTICATION-REF", "SYNCHRONIZED-TRIGGERS", "LOGICAL-OPERATOR", "DPD-DELAY", "ARTIFACT-CHECKSUM-REFS", "ETHERNET-CLUSTER-CONDITIONAL", "SEGMENT-REFS", "BASE-TYPE-ENCODING", "E-2-E-METHOD-PROTECTION-PROPSS", "TD-EVENT-MODE-DECLARATION-TYPE", "STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR", "LOCAL-SUPERVISION-REF", "CUSTOM-SPECIFICATION-REFS", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "DIAGNOSTIC-FREEZE-FRAME-REF", "DEPLOYMENT-URIS", "DATA-REF", "NM-MSG-CYCLE-TIME", "SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING", "COMPONENT-IMPLEMENTATION-REF", "MAX-LENGTH", "TIMING-CONDITION", "USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF", "RPT-HOOKS", "INITIAL-OFFER-BEHAVIOR", "SOMEIP-EVENT-PROPS", "PERSISTENCY-FILE", "SECURITY-EVENT-REFS", "SERVICE-INTERFACE-REFS", "UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE", "BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL", "TCP-DELAYED-ACK-TIMEOUT", "TLV-DATA-ID-DEFINITION-REFS", "VERSION", "REQUIRED-FIBEX-ELEMENT-REF", "PROCESS-MODE-IREF", "CAN-TP-ADDRESS", "SWC-RUNNABLE-REF", "SEND-RESP-PEND-ON-TRANS-TO-BOOT", "IMPLEMENTATION-PROPSS", "ADDRESS-SEMANTICS", "BSW-COMPOSITION-TIMING", "STIM-ENABLER", "READ-DATA-CLASS-REF", "STATE-MANAGEMEN-PHM-ERROR-INTERFACE", "UDP-PORT", "FILE-TYPE-PATTERN", "BASE-TYPE-SIZE", "FIREWALL-RULE-REF", "OPTIONAL", "RELATIONS", "REQUEST-FILE-TRANSFER-CLASS-REF", "CONSUMED-EVENT-GROUP-REFS", "ACL-OBJECT-REF", "MAIN-FUNCTION-PERIOD", "NETWORK-LAYER-RULE", "DIAGNOSTIC-TEST-RESULT", "PERSISTENCY-KEY-VALUE-STORAGE", "SUPERVISION-CONDITION", "APPLIES-TO-DEPENDENCIES"];

    /// derive an enum entry from an input string using a perfect hash function
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseElementNameError> {
        static DISPLACEMENTS: [(u16, u16); 852] = [(0, 142), (1, 101), (0, 980), (0, 2), (4, 4668), (0, 342), (0, 153), (0, 462), (0, 113), (0, 0), (0, 0), (0, 13), (0, 41), (1, 2785), (0, 0), (0, 5440), (0, 48), (0, 1), (24, 3104), (0, 1677), (0, 8), (0, 874), (0, 2683), (0, 133), (0, 8), (2, 5007), (6, 981), (11, 398), (0, 9), (0, 4), (0, 20), (0, 12), (0, 122), (1, 5), (0, 3424), (1, 3479), (0, 2), (0, 2), (2, 2165), (0, 777), (0, 1), (0, 345), (0, 5935), (0, 46), (1, 176), (0, 0), (0, 1074), (0, 610), (0, 644), (0, 14), (0, 20), (0, 8), (0, 148), (2, 982), (0, 4240), (0, 10), (0, 11), (0, 55), (0, 121), (3, 2526), (0, 107), (9, 335), (3, 4810), (0, 515), (0, 58), (0, 38), (0, 446), (0, 0), (0, 1), (0, 0), (0, 3010), (0, 65), (0, 4718), (7, 2975), (0, 2), (0, 3841), (0, 2389), (0, 845), (2, 1186), (0, 4), (0, 11), (0, 1838), (0, 1556), (0, 68), (0, 53), (0, 204), (0, 1471), (2, 5685), (0, 46), (0, 27), (0, 906), (2, 1105), (0, 85), (0, 5354), (0, 3091), (5, 930), (0, 15), (1, 4363), (0, 2), (0, 248), (0, 773), (0, 4), (0, 137), (0, 0), (1, 3167), (26, 5717), (2, 5905), (0, 37), (0, 16), (0, 43), (0, 1562), (0, 2103), (0, 15), (0, 48), (0, 0), (0, 1705), (0, 2), (0, 84), (0, 332), (0, 63), (0, 0), (5, 5346), (0, 1), (0, 2078), (0, 8), (0, 60), (0, 2), (0, 0), (0, 98), (0, 885), (0, 4355), (0, 709), (0, 4397), (0, 1960), (0, 0), (0, 44), (0, 384), (0, 0), (13, 3502), (16, 1628), (0, 1424), (0, 62), (0, 1), (0, 2429), (0, 4), (0, 19), (0, 155), (0, 57), (0, 1071), (0, 1313), (0, 4717), (0, 194), (2, 346), (0, 0), (0, 65), (0, 4665), (5, 1671), (18, 1246), (1, 858), (0, 32), (12, 4380), (0, 355), (1, 2199), (0, 289), (0, 1), (0, 343), (0, 236), (0, 5), (0, 3752), (0, 1), (0, 35), (0, 187), (3, 3863), (1, 2165), (0, 491), (0, 1872), (18, 1298), (2, 3024), (0, 4), (0, 346), (0, 1364), (0, 22), (0, 4), (0, 0), (0, 122), (1, 2825), (0, 792), (0, 530), (0, 0), (0, 0), (0, 3317), (1, 612), (11, 4318), (3, 5274), (0, 69), (0, 27), (2, 5403), (0, 2711), (0, 105), (0, 791), (0, 306), (0, 53), (8, 3128), (0, 5299), (1, 3420), (6, 5700), (0, 1), (0, 1), (0, 4980), (0, 8), (1, 1011), (1, 441), (1, 5486), (0, 7), (0, 3), (0, 916), (0, 1), (1, 3429), (0, 57), (0, 35), (0, 54), (3, 17), (0, 45), (0, 909), (1, 1121), (7, 210), (0, 421), (0, 27), (1, 3591), (0, 203), (0, 5228), (1, 5439), (1, 557), (0, 58), (0, 15), (0, 12), (0, 37), (1, 4238), (0, 13), (6, 1292), (0, 5), (2, 4717), (0, 5410), (0, 1174), (0, 106), (1, 617), (0, 617), (0, 4921), (1, 526), (0, 6), (0, 815), (2, 5059), (1, 20), (3, 3608), (0, 87), (1, 1816), (0, 0), (0, 215), (0, 46), (16, 3836), (0, 226), (0, 29), (0, 31), (0, 104), (0, 31), (0, 2309), (1, 2890), (0, 1), (8, 3875), (2, 87), (2, 4800), (0, 30), (0, 1502), (0, 299), (2, 3815), (4, 477), (2, 5345), (0, 0), (0, 1905), (15, 3951), (0, 63), (60, 4338), (24, 5399), (4, 158), (2, 2389), (0, 121), (0, 131), (0, 1), (1, 1388), (19, 4491), (0, 1589), (0, 1), (0, 12), (0, 138), (0, 2), (0, 21), (0, 17), (7, 5276), (0, 4), (0, 222), (0, 12), (1, 990), (2, 3574), (0, 2248), (0, 1), (0, 611), (0, 0), (2, 5615), (0, 291), (0, 1825), (1, 4860), (0, 371), (2, 367), (0, 1023), (15, 5705), (20, 3547), (0, 250), (1, 3191), (9, 3715), (0, 4), (5, 2864), (3, 2704), (0, 7), (0, 1), (21, 2714), (0, 2859), (12, 534), (0, 381), (0, 456), (0, 230), (18, 5301), (0, 99), (0, 1149), (10, 3035), (0, 7), (0, 344), (0, 55), (0, 970), (0, 1), (0, 426), (0, 129), (0, 349), (0, 31), (0, 895), (7, 1859), (22, 3154), (0, 2), (1, 1853), (22, 2447), (0, 1), (0, 537), (0, 0), (3, 404), (0, 562), (0, 86), (0, 383), (0, 26), (0, 1393), (0, 24), (8, 645), (0, 68), (0, 6), (0, 2183), (0, 1), (0, 4488), (0, 29), (0, 174), (1, 1602), (0, 17), (8, 302), (14, 1961), (0, 912), (0, 1), (0, 0), (0, 273), (0, 3), (0, 8), (0, 64), (2, 553), (1, 1433), (0, 960), (4, 979), (1, 4), (0, 0), (0, 26), (0, 1229), (0, 3815), (0, 54), (0, 25), (0, 0), (0, 2), (13, 2995), (0, 344), (0, 0), (4, 4038), (93, 3373), (0, 0), (0, 17), (0, 3), (0, 0), (0, 47), (0, 2), (3, 2601), (46, 4418), (8, 1912), (0, 3436), (24, 3098), (0, 0), (0, 18), (0, 949), (0, 25), (0, 4), (0, 12), (4, 5645), (0, 6), (0, 2), (20, 5248), (0, 6), (10, 2588), (33, 3159), (0, 244), (1, 2538), (0, 46), (0, 5456), (24, 4232), (0, 0), (0, 17), (0, 58), (0, 1787), (0, 161), (0, 19), (6, 3679), (0, 3958), (12, 412), (1, 3680), (0, 65), (0, 8), (8, 4711), (0, 3), (13, 3393), (0, 51), (0, 5571), (0, 73), (0, 5), (0, 631), (0, 11), (5, 2821), (0, 120), (0, 0), (0, 8), (0, 48), (3, 482), (1, 4391), (0, 131), (0, 6), (0, 2821), (2, 1600), (2, 5942), (6, 4537), (0, 0), (93, 3437), (0, 3257), (0, 18), (4, 421), (0, 2495), (31, 2413), (0, 0), (0, 1036), (0, 1656), (1, 5702), (0, 1578), (0, 0), (0, 78), (4, 718), (0, 2), (0, 5627), (0, 175), (0, 47), (14, 4928), (1, 3221), (2, 5213), (0, 3), (6, 4792), (8, 5357), (0, 16), (0, 23), (25, 4469), (2, 4284), (0, 7), (0, 22), (6, 238), (0, 5650), (5, 5701), (0, 2119), (1, 498), (0, 63), (0, 10), (0, 2167), (14, 1426), (49, 3553), (5, 1584), (0, 381), (0, 198), (0, 187), (71, 2277), (0, 186), (5, 2199), (25, 5807), (1, 2890), (0, 2242), (0, 140), (0, 18), (0, 14), (14, 2115), (3, 752), (0, 1188), (17, 653), (0, 98), (0, 2445), (0, 174), (73, 5317), (0, 2730), (0, 362), (0, 589), (0, 22), (38, 151), (0, 1), (0, 50), (0, 43), (96, 1476), (34, 5777), (0, 4), (0, 4478), (29, 3258), (0, 0), (1, 5484), (0, 50), (0, 164), (0, 7), (0, 3), (34, 4513), (0, 44), (90, 1635), (0, 266), (9, 4498), (15, 218), (0, 183), (0, 241), (0, 120), (1, 827), (0, 612), (0, 143), (0, 17), (2, 4295), (29, 4471), (45, 5753), (0, 243), (0, 34), (0, 212), (0, 2), (0, 4007), (0, 1095), (0, 22), (0, 352), (0, 1744), (0, 3138), (31, 3083), (22, 5180), (0, 0), (0, 3109), (0, 685), (4, 946), (0, 43), (6, 3550), (0, 4036), (13, 2260), (0, 2), (0, 5862), (0, 64), (0, 7), (0, 257), (0, 816), (6, 1630), (0, 1051), (0, 132), (6, 2209), (0, 2724), (0, 142), (1, 303), (1, 3197), (0, 76), (0, 1305), (0, 32), (9, 2900), (0, 5), (0, 388), (4, 3308), (0, 32), (0, 551), (0, 0), (0, 91), (1, 562), (0, 7), (0, 5112), (1, 5), (0, 12), (0, 30), (65, 4202), (0, 2), (0, 5300), (30, 16), (0, 7), (0, 444), (0, 40), (133, 1923), (0, 36), (0, 0), (11, 3454), (17, 691), (0, 2), (0, 2), (0, 305), (6, 1244), (26, 4555), (0, 1689), (0, 28), (0, 1448), (0, 0), (0, 5569), (0, 21), (15, 3744), (0, 2218), (2, 2688), (0, 435), (0, 2308), (0, 5851), (3, 4236), (0, 9), (0, 33), (0, 6), (67, 5843), (0, 15), (0, 2367), (0, 199), (0, 293), (3, 4960), (0, 7), (328, 3162), (2, 5070), (0, 93), (0, 3965), (49, 2816), (0, 2403), (1, 3191), (45, 4088), (1, 343), (4, 2373), (0, 47), (0, 0), (0, 2757), (22, 991), (0, 433), (0, 141), (0, 1611), (2, 321), (0, 14), (0, 0), (0, 2966), (0, 1899), (0, 724), (0, 7), (0, 25), (18, 4325), (27, 5774), (95, 1656), (0, 1385), (3, 4749), (0, 2727), (0, 32), (0, 9), (0, 259), (0, 82), (0, 5125), (0, 2086), (2, 1597), (0, 556), (0, 268), (8, 4493), (0, 10), (0, 332), (0, 2990), (0, 0), (0, 1293), (0, 22), (0, 4082), (0, 30), (0, 19), (0, 101), (0, 0), (0, 7), (36, 1440), (0, 359), (0, 0), (7, 4086), (0, 1838), (0, 2510), (23, 3961), (7, 2329), (34, 1493), (0, 98), (12, 5470), (0, 0), (0, 533), (27, 4707), (0, 1), (602, 4865), (83, 2417), (0, 297), (16, 2444), (0, 9), (79, 4453), (0, 299), (0, 22), (0, 1533), (0, 73), (128, 329), (51, 875), (275, 5197), (7, 4581), (0, 2), (0, 3011), (0, 1), (158, 4382), (14, 4515), (0, 9), (0, 681), (0, 2199), (85, 236), (1, 2489), (0, 187), (18, 1839), (206, 3381), (2, 5193), (0, 7), (0, 3574), (181, 4957), (0, 20), (0, 36), (0, 0), (0, 329), (0, 1337), (65, 1666), (0, 0), (1, 4968), (1, 5382), (0, 289), (0, 12), (0, 30), (518, 3896), (6, 2020), (163, 2698), (0, 480), (0, 156), (0, 48), (0, 26), (7, 1984), (0, 0), (51, 768), (0, 1), (0, 0), (0, 2406), (0, 210), (0, 1219), (0, 1), (0, 3411), (4, 904), (0, 49), (1, 2336), (0, 181), (0, 270), (66, 962), (0, 1), (97, 1299), (5, 2645), (13, 5354), (0, 148), (7, 153), (0, 61), (0, 88), (39, 3890), (0, 23), (0, 2), (0, 24), (1229, 4404), (1, 2820), (55, 4008), (1, 627), (0, 49), (0, 355), (1, 5749), (3, 4962), (2, 311), (107, 2716), (0, 0), (0, 4957), (300, 2703), (0, 307), (0, 1225), (0, 4119), (0, 1966), (0, 110), (0, 242), (24, 434), (229, 1046), (3, 121), (0, 5), (0, 5333), (0, 533), (9, 1519), (6, 375), (0, 520), (1, 5820), (22, 1795), (266, 1788), (3, 3231), (0, 2420), (0, 3460), (0, 23), (0, 7), (0, 5490), (0, 186), (0, 550), (0, 166), (1, 5582), (0, 330), (0, 19), (0, 784), (290, 3367), (0, 1213), (0, 0), (0, 1287), (0, 21), (344, 3649), (14, 2912), (2, 1533), (27, 4978), (0, 9), (0, 471), (0, 4), (49, 3823), (79, 1050), (6, 3993)];
        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 852) as usize];
        let item_idx = u32::from(d2).wrapping_add(f1.wrapping_mul(u32::from(d1))).wrapping_add(f2) as usize % 5962;
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
    #[must_use] pub fn to_str(&self) -> &'static str {
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

