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
    A2LDisplayText                                                         = 2123,
    /// AAF-AES-3-DATA-TYPE
    AafAes3DataType                                                        = 5583,
    /// AAF-FORMAT
    AafFormat                                                              = 4901,
    /// AAF-NOMINAL-RATE
    AafNominalRate                                                         = 2271,
    /// ABBR-NAME
    AbbrName                                                               = 1462,
    /// ABS
    Abs                                                                    = 3075,
    /// ABSOLUTE
    Absolute                                                               = 4511,
    /// ABSOLUTE-TOLERANCE
    AbsoluteTolerance                                                      = 2331,
    /// ABSOLUTELY-SCHEDULED-TIMINGS
    AbsolutelyScheduledTimings                                             = 1655,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 4279,
    /// ACCEPT-ENCODING
    AcceptEncoding                                                         = 4264,
    /// ACCEPT-VARIABLE-DA
    AcceptVariableDa                                                       = 1169,
    /// ACCEPT-VARIABLE-SA
    AcceptVariableSa                                                       = 2000,
    /// ACCEPTANCE-FIELD
    AcceptanceField                                                        = 5939,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF
    AcceptedCryptoCipherSuiteWithPskRef                                    = 4028,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS
    AcceptedCryptoCipherSuiteWithPskRefs                                   = 315,
    /// ACCEPTED-REMOTE-CERTIFICATE-REF
    AcceptedRemoteCertificateRef                                           = 4925,
    /// ACCEPTED-REMOTE-CERTIFICATE-REFS
    AcceptedRemoteCertificateRefs                                          = 4866,
    /// ACCEPTED-STARTUP-RANGE
    AcceptedStartupRange                                                   = 4948,
    /// ACCEPTS-ENCODINGS
    AcceptsEncodings                                                       = 829,
    /// ACCESS-CONTROL
    AccessControl                                                          = 2089,
    /// ACCESS-COUNT
    AccessCount                                                            = 2809,
    /// ACCESS-COUNT-SET
    AccessCountSet                                                         = 4830,
    /// ACCESS-COUNT-SETS
    AccessCountSets                                                        = 2248,
    /// ACCESS-COUNTS
    AccessCounts                                                           = 5140,
    /// ACCESS-DATA-RECORD-SIZE
    AccessDataRecordSize                                                   = 5894,
    /// ACCESS-MODE
    AccessMode                                                             = 2759,
    /// ACCESS-PERMISSION-REF
    AccessPermissionRef                                                    = 4523,
    /// ACCESS-PERMISSION-VALIDITY
    AccessPermissionValidity                                               = 5420,
    /// ACCESS-POINT-REF
    AccessPointRef                                                         = 382,
    /// ACCESSED-DATA-PROTOTYPE-IREF
    AccessedDataPrototypeIref                                              = 5713,
    /// ACCESSED-MODE-GROUPS
    AccessedModeGroups                                                     = 3367,
    /// ACCESSED-PARAMETER
    AccessedParameter                                                      = 459,
    /// ACCESSED-VARIABLE
    AccessedVariable                                                       = 2963,
    /// ACCESSED-VARIABLE-REF
    AccessedVariableRef                                                    = 4130,
    /// ACCURACY
    Accuracy                                                               = 2523,
    /// ACCURACY-EXT
    AccuracyExt                                                            = 2029,
    /// ACCURACY-INT
    AccuracyInt                                                            = 3471,
    /// ACF-BUS-PART-REF
    AcfBusPartRef                                                          = 2189,
    /// ACF-BUS-PART-REFS
    AcfBusPartRefs                                                         = 1430,
    /// ACF-BUS-REF
    AcfBusRef                                                              = 4860,
    /// ACF-BUS-REFS
    AcfBusRefs                                                             = 2827,
    /// ACF-MAX-TRANSIT-TIME
    AcfMaxTransitTime                                                      = 4199,
    /// ACF-PARTS
    AcfParts                                                               = 753,
    /// ACF-TRANSPORTED-BUSS
    AcfTransportedBuss                                                     = 6182,
    /// ACK-REQUEST
    AckRequest                                                             = 430,
    /// ACK-TYPE
    AckType                                                                = 5655,
    /// ACL-CONTEXT
    AclContext                                                             = 3051,
    /// ACL-CONTEXTS
    AclContexts                                                            = 5848,
    /// ACL-OBJECT-CLASS
    AclObjectClass                                                         = 200,
    /// ACL-OBJECT-CLASSS
    AclObjectClasss                                                        = 2328,
    /// ACL-OBJECT-REF
    AclObjectRef                                                           = 3926,
    /// ACL-OBJECT-REFS
    AclObjectRefs                                                          = 4071,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 1381,
    /// ACL-OPERATION
    AclOperation                                                           = 1940,
    /// ACL-OPERATION-REF
    AclOperationRef                                                        = 544,
    /// ACL-OPERATION-REFS
    AclOperationRefs                                                       = 4342,
    /// ACL-PERMISSION
    AclPermission                                                          = 229,
    /// ACL-ROLE
    AclRole                                                                = 1416,
    /// ACL-ROLE-REF
    AclRoleRef                                                             = 2835,
    /// ACL-ROLE-REFS
    AclRoleRefs                                                            = 1111,
    /// ACL-SCOPE
    AclScope                                                               = 1545,
    /// ACTION
    Action                                                                 = 345,
    /// ACTION-ITEM-LISTS
    ActionItemLists                                                        = 1375,
    /// ACTION-ITEM-REF
    ActionItemRef                                                          = 5411,
    /// ACTION-ITEMS
    ActionItems                                                            = 5502,
    /// ACTION-LIST
    ActionList                                                             = 25,
    /// ACTION-LIST-EXECUTION
    ActionListExecution                                                    = 4719,
    /// ACTION-LIST-ITEM-REF
    ActionListItemRef                                                      = 3692,
    /// ACTION-LIST-ITEM-REFS
    ActionListItemRefs                                                     = 2178,
    /// ACTION-LISTS
    ActionLists                                                            = 5949,
    /// ACTION-POINT-OFFSET
    ActionPointOffset                                                      = 1279,
    /// ACTION-TYPE
    ActionType                                                             = 4001,
    /// ACTIONS
    Actions                                                                = 2856,
    /// ACTIVATE-AT-START
    ActivateAtStart                                                        = 5497,
    /// ACTIVATION
    Activation                                                             = 2386,
    /// ACTIVATION-ACTION
    ActivationAction                                                       = 1096,
    /// ACTIVATION-POINTS
    ActivationPoints                                                       = 3406,
    /// ACTIVATION-REASON-REPRESENTATION-REF
    ActivationReasonRepresentationRef                                      = 4466,
    /// ACTIVATION-REASONS
    ActivationReasons                                                      = 6162,
    /// ACTIVATION-SWITCH
    ActivationSwitch                                                       = 2540,
    /// ACTIVE-SUPERVISION-REF
    ActiveSupervisionRef                                                   = 1438,
    /// ACTIVE-SUPERVISION-REFS
    ActiveSupervisionRefs                                                  = 6444,
    /// ACTUAL-EVENT-REF
    ActualEventRef                                                         = 480,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 226,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 2851,
    /// ADAPTIVE-FIREWALL-MODULE-INSTANTIATION
    AdaptiveFirewallModuleInstantiation                                    = 6267,
    /// ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING
    AdaptiveFirewallToPortPrototypeMapping                                 = 4217,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 4140,
    /// ADD-INFO-5
    AddInfo5                                                               = 2460,
    /// ADDITIONAL-INFORMATION
    AdditionalInformation                                                  = 6356,
    /// ADDITIONAL-NATIVE-TYPE-QUALIFIER
    AdditionalNativeTypeQualifier                                          = 1265,
    /// ADDRESS
    Address                                                                = 403,
    /// ADDRESS-CLAIM-ENABLED
    AddressClaimEnabled                                                    = 4462,
    /// ADDRESS-CONFIGURATION-CAPABILITY
    AddressConfigurationCapability                                         = 5890,
    /// ADDRESS-RANGE-LOWER-BOUND
    AddressRangeLowerBound                                                 = 2948,
    /// ADDRESS-RANGE-UPPER-BOUND
    AddressRangeUpperBound                                                 = 808,
    /// ADDRESS-SEMANTICS
    AddressSemantics                                                       = 5407,
    /// ADDRESSING-FORMAT
    AddressingFormat                                                       = 335,
    /// ADMIN-DATA
    AdminData                                                              = 2055,
    /// ADMIT-WITHOUT-MESSAGE-ID
    AdmitWithoutMessageId                                                  = 3777,
    /// AES-3-DATA-TYPE-H
    Aes3DataTypeH                                                          = 5450,
    /// AES-3-DATA-TYPE-L
    Aes3DataTypeL                                                          = 1527,
    /// AFFECTED-APPLICATION
    AffectedApplication                                                    = 3142,
    /// AFFECTED-BSW-MODULE
    AffectedBswModule                                                      = 3362,
    /// AFFECTED-BY-CONTROL-DTC-SETTING
    AffectedByControlDtcSetting                                            = 6315,
    /// AFFECTED-FUNCTIONAL-CLUSTER
    AffectedFunctionalCluster                                              = 3203,
    /// AFFECTED-REF
    AffectedRef                                                            = 2929,
    /// AFFECTED-REFS
    AffectedRefs                                                           = 4414,
    /// AFFECTED-STATE-IREF
    AffectedStateIref                                                      = 155,
    /// AFFECTION-KIND
    AffectionKind                                                          = 4971,
    /// AGE
    Age                                                                    = 4776,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 89,
    /// AGGREGATION
    Aggregation                                                            = 3808,
    /// AGGREGATION-CONDITION
    AggregationCondition                                                   = 955,
    /// AGGREGATION-REF
    AggregationRef                                                         = 2494,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 696,
    /// AGING-ALLOWED
    AgingAllowed                                                           = 3475,
    /// AGING-CYCLES
    AgingCycles                                                            = 2536,
    /// AGING-REF
    AgingRef                                                               = 4947,
    /// AGING-REQUIRES-TESTED-CYCLE
    AgingRequiresTestedCycle                                               = 5807,
    /// AH-CIPHER-SUITE-NAME
    AhCipherSuiteName                                                      = 12,
    /// AH-CIPHER-SUITE-NAMES
    AhCipherSuiteNames                                                     = 2440,
    /// ALGORITHM-FAMILY
    AlgorithmFamily                                                        = 6032,
    /// ALGORITHM-MODE
    AlgorithmMode                                                          = 6367,
    /// ALGORITHM-SECONDARY-FAMILY
    AlgorithmSecondaryFamily                                               = 3657,
    /// ALIAS-EVENT-REF
    AliasEventRef                                                          = 4162,
    /// ALIAS-NAME-ASSIGNMENT
    AliasNameAssignment                                                    = 5299,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 576,
    /// ALIAS-NAMES
    AliasNames                                                             = 2129,
    /// ALIGNMENT
    Alignment                                                              = 2402,
    /// ALIVE-CHECK-RESPONSE-TIMEOUT
    AliveCheckResponseTimeout                                              = 6244,
    /// ALIVE-REFERENCE-CYCLE
    AliveReferenceCycle                                                    = 5092,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 4170,
    /// ALIVE-SUPERVISIONS
    AliveSupervisions                                                      = 1729,
    /// ALIVE-TIMEOUT
    AliveTimeout                                                           = 5983,
    /// ALL-CHANNELS-REF
    AllChannelsRef                                                         = 102,
    /// ALL-CHANNELS-REFS
    AllChannelsRefs                                                        = 6046,
    /// ALL-NM-MESSAGES-KEEP-AWAKE
    AllNmMessagesKeepAwake                                                 = 4617,
    /// ALL-PHYSICAL-CHANNELS
    AllPhysicalChannels                                                    = 604,
    /// ALL-PHYSICAL-CHANNELS-REF
    AllPhysicalChannelsRef                                                 = 3945,
    /// ALLOCATE-SHADOW-COPY
    AllocateShadowCopy                                                     = 4656,
    /// ALLOCATOR
    Allocator                                                              = 3306,
    /// ALLOCATOR-REF
    AllocatorRef                                                           = 3653,
    /// ALLOW-CONTENT-TYPE-CHANGE
    AllowContentTypeChange                                                 = 1950,
    /// ALLOW-DYNAMIC-L-SDU-LENGTH
    AllowDynamicLSduLength                                                 = 4642,
    /// ALLOW-HALT-DUE-TO-CLOCK
    AllowHaltDueToClock                                                    = 3967,
    /// ALLOW-MASTER-RATE-CORRECTION
    AllowMasterRateCorrection                                              = 963,
    /// ALLOW-PASSIVE-TO-ACTIVE
    AllowPassiveToActive                                                   = 1980,
    /// ALLOW-PROVIDER-RATE-CORRECTION
    AllowProviderRateCorrection                                            = 5011,
    /// ALLOW-UNAUTHENTICATED-PARTICIPANTS
    AllowUnauthenticatedParticipants                                       = 6109,
    /// ALLOWED-I-PV-6-EXT-HEADER
    AllowedIPv6ExtHeader                                                   = 2027,
    /// ALLOWED-I-PV-6-EXT-HEADERS
    AllowedIPv6ExtHeaders                                                  = 4646,
    /// ALLOWED-I-PV-6-EXT-HEADERS-REF
    AllowedIPv6ExtHeadersRef                                               = 4602,
    /// ALLOWED-KEYSLOT-USAGE
    AllowedKeyslotUsage                                                    = 975,
    /// ALLOWED-SERVICE-CONSUMERS
    AllowedServiceConsumers                                                = 694,
    /// ALLOWED-SERVICE-PROVIDERS
    AllowedServiceProviders                                                = 1523,
    /// ALLOWED-TCP-OPTION
    AllowedTcpOption                                                       = 5643,
    /// ALLOWED-TCP-OPTIONS
    AllowedTcpOptions                                                      = 1387,
    /// ALLOWED-TCP-OPTIONS-REF
    AllowedTcpOptionsRef                                                   = 3948,
    /// ALTERNATIVE-NAME
    AlternativeName                                                        = 3986,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 2192,
    /// ANNOTATION
    Annotation                                                             = 6215,
    /// ANNOTATION-ORIGIN
    AnnotationOrigin                                                       = 2359,
    /// ANNOTATION-TEXT
    AnnotationText                                                         = 5869,
    /// ANNOTATIONS
    Annotations                                                            = 5806,
    /// ANONYMOUS
    Anonymous                                                              = 1868,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 5567,
    /// AP-APPLICATION-ENDPOINTS
    ApApplicationEndpoints                                                 = 1186,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 906,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 3201,
    /// AP-APPLICATION-ERROR-REF
    ApApplicationErrorRef                                                  = 1769,
    /// AP-APPLICATION-ERROR-REFS
    ApApplicationErrorRefs                                                 = 2443,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 3084,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 3763,
    /// API-PRINCIPLE
    ApiPrinciple                                                           = 5506,
    /// API-SERVICE-PREFIX
    ApiServicePrefix                                                       = 3274,
    /// APMC-BOOLEAN-PARAM-DEF
    ApmcBooleanParamDef                                                    = 2317,
    /// APMC-CHOICE-CONTAINER-DEF
    ApmcChoiceContainerDef                                                 = 780,
    /// APMC-CHOICE-CONTAINER-REFERENCE-DEF
    ApmcChoiceContainerReferenceDef                                        = 5744,
    /// APMC-CONTAINER-REFERENCE-DEF
    ApmcContainerReferenceDef                                              = 356,
    /// APMC-CONTAINER-REFERENCE-VALUE
    ApmcContainerReferenceValue                                            = 1715,
    /// APMC-CONTAINER-VALUE
    ApmcContainerValue                                                     = 2842,
    /// APMC-DEFINITION-COLLECTION
    ApmcDefinitionCollection                                               = 1603,
    /// APMC-ENUMERATION-LITERAL-DEF
    ApmcEnumerationLiteralDef                                              = 4364,
    /// APMC-ENUMERATION-PARAM-DEF
    ApmcEnumerationParamDef                                                = 6040,
    /// APMC-FLOAT-PARAM-DEF
    ApmcFloatParamDef                                                      = 2371,
    /// APMC-FOREIGN-REFERENCE-DEF
    ApmcForeignReferenceDef                                                = 5675,
    /// APMC-FOREIGN-REFERENCE-VALUE
    ApmcForeignReferenceValue                                              = 5524,
    /// APMC-FUNCTIONAL-CLUSTER-DEF
    ApmcFunctionalClusterDef                                               = 5287,
    /// APMC-FUNCTIONAL-CLUSTER-VALUE
    ApmcFunctionalClusterValue                                             = 5291,
    /// APMC-INSTANCE-REFERENCE-DEF
    ApmcInstanceReferenceDef                                               = 1,
    /// APMC-INSTANCE-REFERENCE-VALUE
    ApmcInstanceReferenceValue                                             = 1155,
    /// APMC-INTEGER-PARAM-DEF
    ApmcIntegerParamDef                                                    = 3999,
    /// APMC-NUMERICAL-PARAM-VALUE
    ApmcNumericalParamValue                                                = 5690,
    /// APMC-PARAM-CONF-CONTAINER-DEF
    ApmcParamConfContainerDef                                              = 4541,
    /// APMC-STRING-PARAM-DEF
    ApmcStringParamDef                                                     = 3544,
    /// APMC-TEXTUAL-PARAM-VALUE
    ApmcTextualParamValue                                                  = 3420,
    /// APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-DEF
    ApmcUpstreamDocForeignReferenceDef                                     = 3463,
    /// APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-VALUE
    ApmcUpstreamDocForeignReferenceValue                                   = 1534,
    /// APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-DEF
    ApmcUpstreamDocInstanceReferenceDef                                    = 3746,
    /// APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-VALUE
    ApmcUpstreamDocInstanceReferenceValue                                  = 2238,
    /// APMC-URI-FOREIGN-REFERENCE-DEF
    ApmcUriForeignReferenceDef                                             = 1851,
    /// APMC-URI-FOREIGN-REFERENCE-VALUE
    ApmcUriForeignReferenceValue                                           = 2265,
    /// APMC-URI-INSTANCE-REFERENCE-DEF
    ApmcUriInstanceReferenceDef                                            = 4715,
    /// APMC-URI-INSTANCE-REFERENCE-VALUE
    ApmcUriInstanceReferenceValue                                          = 5295,
    /// APMC-VALUE-COLLECTION
    ApmcValueCollection                                                    = 4156,
    /// APP-ID
    AppId                                                                  = 3409,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 4224,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS
    AppOsTaskProxyToEcuTaskProxyMappings                                   = 1994,
    /// APP-TASK-PROXY-REF
    AppTaskProxyRef                                                        = 1199,
    /// APPL-CONSTANT-REF
    ApplConstantRef                                                        = 3373,
    /// APPL-INIT-VALUE
    ApplInitValue                                                          = 5289,
    /// APPL-WATCHDOG-LIMIT
    ApplWatchdogLimit                                                      = 2698,
    /// APPLICABILITY-INFO
    ApplicabilityInfo                                                      = 1061,
    /// APPLICABILITY-INFO-SET
    ApplicabilityInfoSet                                                   = 259,
    /// APPLICABILITY-INFOS
    ApplicabilityInfos                                                     = 4829,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 2163,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 4086,
    /// APPLICATION-ARRAY-ELEMENT-REF
    ApplicationArrayElementRef                                             = 1060,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 2621,
    /// APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION
    ApplicationAssocMapElementValueSpecification                           = 125,
    /// APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION
    ApplicationAssocMapValueSpecification                                  = 3668,
    /// APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF
    ApplicationCompositeDataTypeSubElementRef                              = 4903,
    /// APPLICATION-COMPOSITE-ELEMENT-IREF
    ApplicationCompositeElementIref                                        = 268,
    /// APPLICATION-CONTEXT-REF
    ApplicationContextRef                                                  = 2011,
    /// APPLICATION-CONTEXT-REFS
    ApplicationContextRefs                                                 = 3321,
    /// APPLICATION-DATA-TYPE-REF
    ApplicationDataTypeRef                                                 = 5739,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 4105,
    /// APPLICATION-DESCRIPTION
    ApplicationDescription                                                 = 313,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 5149,
    /// APPLICATION-ENDPOINT-REF
    ApplicationEndpointRef                                                 = 1394,
    /// APPLICATION-ENDPOINT-REF-CONDITIONAL
    ApplicationEndpointRefConditional                                      = 5233,
    /// APPLICATION-ENTRY
    ApplicationEntry                                                       = 1428,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 1714,
    /// APPLICATION-ID
    ApplicationId                                                          = 6280,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 2252,
    /// APPLICATION-MODE-IREF
    ApplicationModeIref                                                    = 3417,
    /// APPLICATION-MODE-MACHINE
    ApplicationModeMachine                                                 = 679,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 1313,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 4410,
    /// APPLICATION-PARTITION-REF
    ApplicationPartitionRef                                                = 3000,
    /// APPLICATION-PARTITION-REFS
    ApplicationPartitionRefs                                               = 2210,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 5017,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS
    ApplicationPartitionToEcuPartitionMappings                             = 2159,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 5246,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 3931,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 510,
    /// APPLICATION-RECORD-ELEMENT-REF
    ApplicationRecordElementRef                                            = 4682,
    /// APPLICATION-RULE-BASED-VALUE-SPECIFICATION
    ApplicationRuleBasedValueSpecification                                 = 2761,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 50,
    /// APPLICATION-TYPE-REF
    ApplicationTypeRef                                                     = 4636,
    /// APPLICATION-TYPE-TREF
    ApplicationTypeTref                                                    = 4365,
    /// APPLICATION-VALUE-SPECIFICATION
    ApplicationValueSpecification                                          = 364,
    /// APPLICATIONS
    Applications                                                           = 316,
    /// APPLIED-STANDARD
    AppliedStandard                                                        = 1998,
    /// APPLIED-STANDARDS
    AppliedStandards                                                       = 2804,
    /// APPLIES-TO
    AppliesTo                                                              = 3202,
    /// APPLIES-TO-DEPENDENCIES
    AppliesToDependencies                                                  = 4795,
    /// APPROVAL-REQUIRED
    ApprovalRequired                                                       = 6171,
    /// APPROVAL-STATUS
    ApprovalStatus                                                         = 4283,
    /// AR-PACKAGE
    ArPackage                                                              = 5250,
    /// AR-PACKAGES
    ArPackages                                                             = 5413,
    /// AR-PARAMETER
    ArParameter                                                            = 5687,
    /// AR-RELEASE-VERSION
    ArReleaseVersion                                                       = 2880,
    /// AR-TYPED-PER-INSTANCE-MEMORY-REF
    ArTypedPerInstanceMemoryRef                                            = 6176,
    /// AR-TYPED-PER-INSTANCE-MEMORYS
    ArTypedPerInstanceMemorys                                              = 6151,
    /// ARBITRARY-ADDRESS-CAPABLE
    ArbitraryAddressCapable                                                = 4923,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 514,
    /// ARBITRATION
    Arbitration                                                            = 762,
    /// ARBITRATIONS
    Arbitrations                                                           = 2350,
    /// AREA
    Area                                                                   = 4064,
    /// ARGUMENT
    Argument                                                               = 1216,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 2325,
    /// ARGUMENT-MAPPINGS
    ArgumentMappings                                                       = 1680,
    /// ARGUMENT-REF
    ArgumentRef                                                            = 2146,
    /// ARGUMENTS
    Arguments                                                              = 3031,
    /// ARGUMENTSS
    Argumentss                                                             = 6146,
    /// ARP-PROPS
    ArpProps                                                               = 2233,
    /// ARRAY-ELEMENT-MAPPINGS
    ArrayElementMappings                                                   = 2025,
    /// ARRAY-ELEMENT-REF
    ArrayElementRef                                                        = 1555,
    /// ARRAY-IMPL-POLICY
    ArrayImplPolicy                                                        = 5357,
    /// ARRAY-SIZE
    ArraySize                                                              = 4037,
    /// ARRAY-SIZE-HANDLING
    ArraySizeHandling                                                      = 3488,
    /// ARRAY-SIZE-SEMANTICS
    ArraySizeSemantics                                                     = 1122,
    /// ARRAY-VALUE-SPECIFICATION
    ArrayValueSpecification                                                = 706,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 1812,
    /// ARTIFACT-CHECKSUM-REF
    ArtifactChecksumRef                                                    = 2834,
    /// ARTIFACT-CHECKSUM-REFS
    ArtifactChecksumRefs                                                   = 3849,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 3238,
    /// ARTIFACT-CHECKSUMS
    ArtifactChecksums                                                      = 5575,
    /// ARTIFACT-DESCRIPTOR
    ArtifactDescriptor                                                     = 1689,
    /// ARTIFACT-DESCRIPTORS
    ArtifactDescriptors                                                    = 2492,
    /// ARTIFACT-LOCATOR
    ArtifactLocator                                                        = 5033,
    /// ARTIFACT-LOCATORS
    ArtifactLocators                                                       = 434,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 4805,
    /// ASSERTIONS
    Assertions                                                             = 1891,
    /// ASSIGN-FRAME-ID
    AssignFrameId                                                          = 3485,
    /// ASSIGN-FRAME-ID-RANGE
    AssignFrameIdRange                                                     = 3665,
    /// ASSIGN-NAD
    AssignNad                                                              = 363,
    /// ASSIGNED-CONTROLLER-REF
    AssignedControllerRef                                                  = 6347,
    /// ASSIGNED-DATA-TYPES
    AssignedDataTypes                                                      = 5989,
    /// ASSIGNED-DATAS
    AssignedDatas                                                          = 3048,
    /// ASSIGNED-ENTRY-REF
    AssignedEntryRef                                                       = 1220,
    /// ASSIGNED-ENTRY-ROLES
    AssignedEntryRoles                                                     = 1621,
    /// ASSIGNED-FRAME-TRIGGERING-REF
    AssignedFrameTriggeringRef                                             = 2675,
    /// ASSIGNED-LIN-SLAVE-CONFIG-REF
    AssignedLinSlaveConfigRef                                              = 4970,
    /// ASSIGNED-PORTS
    AssignedPorts                                                          = 346,
    /// ASSIGNED-TRAFFIC-CLASS
    AssignedTrafficClass                                                   = 4208,
    /// ASSIGNED-TRAFFIC-CLASSS
    AssignedTrafficClasss                                                  = 1033,
    /// ASSIGNMENT-PRIORITY
    AssignmentPriority                                                     = 4775,
    /// ASSOCIATED-COM-I-PDU-GROUP-REF
    AssociatedComIPduGroupRef                                              = 3579,
    /// ASSOCIATED-COM-I-PDU-GROUP-REFS
    AssociatedComIPduGroupRefs                                             = 4501,
    /// ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    AssociatedConsumedProvidedServiceInstanceGroups                        = 5067,
    /// ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF
    AssociatedCrossSwClusterComRtePluginRef                                = 5980,
    /// ASSOCIATED-EVENT-IDENTIFICATION
    AssociatedEventIdentification                                          = 5812,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REF
    AssociatedPdurIPduGroupRef                                             = 4956,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REFS
    AssociatedPdurIPduGroupRefs                                            = 2806,
    /// ASSOCIATED-RTE-PLUGIN-REF
    AssociatedRtePluginRef                                                 = 2489,
    /// ASSUMED-CURRENT-STATE-IREF
    AssumedCurrentStateIref                                                = 5209,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 879,
    /// ASYNCHRONOUS-SERVER-CALL-POINT-REF
    AsynchronousServerCallPointRef                                         = 3418,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 1106,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS
    AsynchronousServerCallResultPoints                                     = 3586,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 5924,
    /// ASYNCHRONOUS-TRAFFIC-SHAPER-REF
    AsynchronousTrafficShaperRef                                           = 3206,
    /// ATTRIBUTE-DEFS
    AttributeDefs                                                          = 1471,
    /// ATTRIBUTE-NAME
    AttributeName                                                          = 2613,
    /// ATTRIBUTE-REF
    AttributeRef                                                           = 5076,
    /// ATTRIBUTE-TAILORINGS
    AttributeTailorings                                                    = 902,
    /// ATTRIBUTE-VALUES
    AttributeValues                                                        = 4941,
    /// ATTRIBUTES
    Attributes                                                             = 3009,
    /// AUDIENCE
    Audience                                                               = 1728,
    /// AUDIENCES
    Audiences                                                              = 4753,
    /// AUTH-ALGORITHM
    AuthAlgorithm                                                          = 239,
    /// AUTH-DATA-FRESHNESS-LENGTH
    AuthDataFreshnessLength                                                = 2358,
    /// AUTH-DATA-FRESHNESS-START-POSITION
    AuthDataFreshnessStartPosition                                         = 1626,
    /// AUTH-INFO-TX-LENGTH
    AuthInfoTxLength                                                       = 1487,
    /// AUTH-ROLE-REF
    AuthRoleRef                                                            = 4415,
    /// AUTH-ROLE-REFS
    AuthRoleRefs                                                           = 311,
    /// AUTHENTIC-CONNECTION-PROPSS
    AuthenticConnectionPropss                                              = 4998,
    /// AUTHENTICATION-BUILD-ATTEMPTS
    AuthenticationBuildAttempts                                            = 6445,
    /// AUTHENTICATION-CLASS-REF
    AuthenticationClassRef                                                 = 5130,
    /// AUTHENTICATION-ENABLED
    AuthenticationEnabled                                                  = 2501,
    /// AUTHENTICATION-PROPS-REF
    AuthenticationPropsRef                                                 = 619,
    /// AUTHENTICATION-PROPSS
    AuthenticationPropss                                                   = 2522,
    /// AUTHENTICATION-REF
    AuthenticationRef                                                      = 2634,
    /// AUTHENTICATION-RETRIES
    AuthenticationRetries                                                  = 3936,
    /// AUTHENTICATION-ROLE-REF
    AuthenticationRoleRef                                                  = 295,
    /// AUTHENTICATION-ROLE-REFS
    AuthenticationRoleRefs                                                 = 3375,
    /// AUTHENTICATION-TIMEOUT
    AuthenticationTimeout                                                  = 1805,
    /// AUTHENTICATION-VERIFY-ATTEMPTS
    AuthenticationVerifyAttempts                                           = 5428,
    /// AUTO-AVAILABLE
    AutoAvailable                                                          = 5958,
    /// AUTO-COLLECT
    AutoCollect                                                            = 211,
    /// AUTO-IP-PROPS
    AutoIpProps                                                            = 2426,
    /// AUTO-REQUIRE
    AutoRequire                                                            = 1232,
    /// AUTO-START
    AutoStart                                                              = 3694,
    /// AUTOMATIC-END
    AutomaticEnd                                                           = 3957,
    /// AUTOSAR
    Autosar                                                                = 4057,
    /// AUTOSAR-CONNECTOR
    AutosarConnector                                                       = 809,
    /// AUTOSAR-DATA-TYPE-REF
    AutosarDataTypeRef                                                     = 3010,
    /// AUTOSAR-DATA-TYPE-REF-CONDITIONAL
    AutosarDataTypeRefConditional                                          = 4938,
    /// AUTOSAR-ENGINEERING-OBJECT
    AutosarEngineeringObject                                               = 6113,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 5021,
    /// AUTOSAR-PARAMETER-IREF
    AutosarParameterIref                                                   = 4999,
    /// AUTOSAR-VARIABLE
    AutosarVariable                                                        = 4311,
    /// AUTOSAR-VARIABLE-IN-IMPL-DATATYPE
    AutosarVariableInImplDatatype                                          = 5171,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 6205,
    /// AUTOSAR-VARIABLE-IREF
    AutosarVariableIref                                                    = 5906,
    /// AUXILIARY-FIELD-DEFINITIONS
    AuxiliaryFieldDefinitions                                              = 2154,
    /// AUXILIARY-FIELDS
    AuxiliaryFields                                                        = 6038,
    /// AVERAGE-MEMORY-CONSUMPTION
    AverageMemoryConsumption                                               = 2787,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 4002,
    /// BANDWIDTH-LIMITATION
    BandwidthLimitation                                                    = 1423,
    /// BASE-COMPOSITION-REF
    BaseCompositionRef                                                     = 301,
    /// BASE-CYCLE
    BaseCycle                                                              = 5888,
    /// BASE-FREQUENCY
    BaseFrequency                                                          = 3877,
    /// BASE-IS-THIS-PACKAGE
    BaseIsThisPackage                                                      = 1425,
    /// BASE-REF
    BaseRef                                                                = 1201,
    /// BASE-REFERENCE-REF
    BaseReferenceRef                                                       = 3730,
    /// BASE-TYPE-ENCODING
    BaseTypeEncoding                                                       = 585,
    /// BASE-TYPE-REF
    BaseTypeRef                                                            = 4484,
    /// BASE-TYPE-SIZE
    BaseTypeSize                                                           = 120,
    /// BASIC-CYCLE-LENGTH
    BasicCycleLength                                                       = 415,
    /// BAUDRATE
    Baudrate                                                               = 1490,
    /// BEHAVIOR
    Behavior                                                               = 40,
    /// BEHAVIOR-REF
    BehaviorRef                                                            = 1709,
    /// BEST-CASE-EXECUTION-TIME
    BestCaseExecutionTime                                                  = 5811,
    /// BGCOLOR
    Bgcolor                                                                = 3314,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 5870,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 645,
    /// BINARY-MANIFEST-ITEM-NUMERICAL-VALUE
    BinaryManifestItemNumericalValue                                       = 1312,
    /// BINARY-MANIFEST-ITEM-POINTER-VALUE
    BinaryManifestItemPointerValue                                         = 3802,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 195,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 1470,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 2227,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 1411,
    /// BIT
    Bit                                                                    = 2654,
    /// BIT-NUMBER
    BitNumber                                                              = 106,
    /// BIT-OFFSET
    BitOffset                                                              = 56,
    /// BIT-POSITION
    BitPosition                                                            = 42,
    /// BITFIELD-TEXT-TABLE-MASK-FIRST
    BitfieldTextTableMaskFirst                                             = 5631,
    /// BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR
    BitfieldTextTableMaskNvBlockDescriptor                                 = 3972,
    /// BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE
    BitfieldTextTableMaskPortPrototype                                     = 869,
    /// BITFIELD-TEXT-TABLE-MASK-SECOND
    BitfieldTextTableMaskSecond                                            = 5198,
    /// BLACKLISTED-VERSIONS
    BlacklistedVersions                                                    = 3815,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREF
    BlockIfStateActiveApIref                                               = 72,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREFS
    BlockIfStateActiveApIrefs                                              = 3577,
    /// BLOCK-IF-STATE-ACTIVE-CP-REF
    BlockIfStateActiveCpRef                                                = 4622,
    /// BLOCK-IF-STATE-ACTIVE-CP-REFS
    BlockIfStateActiveCpRefs                                               = 4050,
    /// BLOCK-INCONSISTENT
    BlockInconsistent                                                      = 1654,
    /// BLOCK-STATE
    BlockState                                                             = 3713,
    /// BLOCK-STATES
    BlockStates                                                            = 453,
    /// BLOCKLISTED-VERSIONS
    BlocklistedVersions                                                    = 4075,
    /// BLUEPRINT-CONDITION
    BlueprintCondition                                                     = 6195,
    /// BLUEPRINT-DERIVATION-GUIDE
    BlueprintDerivationGuide                                               = 5521,
    /// BLUEPRINT-MAPPING
    BlueprintMapping                                                       = 383,
    /// BLUEPRINT-MAPPING-GUIDE
    BlueprintMappingGuide                                                  = 2312,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 2267,
    /// BLUEPRINT-MAPS
    BlueprintMaps                                                          = 4088,
    /// BLUEPRINT-POLICY-LIST
    BlueprintPolicyList                                                    = 618,
    /// BLUEPRINT-POLICY-NOT-MODIFIABLE
    BlueprintPolicyNotModifiable                                           = 3873,
    /// BLUEPRINT-POLICY-SINGLE
    BlueprintPolicySingle                                                  = 1756,
    /// BLUEPRINT-POLICYS
    BlueprintPolicys                                                       = 3735,
    /// BLUEPRINT-REF
    BlueprintRef                                                           = 2599,
    /// BOOLEAN-VALUE-VARIATION-POINT
    BooleanValueVariationPoint                                             = 2891,
    /// BOR-COUNTER-L-1-TO-L-2
    BorCounterL1ToL2                                                       = 1864,
    /// BOR-TIME-L-1
    BorTimeL1                                                              = 896,
    /// BOR-TIME-L-2
    BorTimeL2                                                              = 1013,
    /// BOR-TIME-TX-ENSURED
    BorTimeTxEnsured                                                       = 3441,
    /// BR
    Br                                                                     = 2120,
    /// BROADCAST
    Broadcast                                                              = 387,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 2007,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 2225,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 2488,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 2437,
    /// BSW-BEHAVIOR-REF
    BswBehaviorRef                                                         = 3327,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 3061,
    /// BSW-CLIENT-POLICY
    BswClientPolicy                                                        = 864,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 3786,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 5144,
    /// BSW-DATA-SEND-POLICY
    BswDataSendPolicy                                                      = 1872,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 857,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 5196,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 5386,
    /// BSW-ENTITY-REF
    BswEntityRef                                                           = 1644,
    /// BSW-ENTRY-KIND
    BswEntryKind                                                           = 6254,
    /// BSW-ENTRY-RELATIONSHIP
    BswEntryRelationship                                                   = 2072,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 5034,
    /// BSW-ENTRY-RELATIONSHIP-TYPE
    BswEntryRelationshipType                                               = 3064,
    /// BSW-ENTRY-RELATIONSHIPS
    BswEntryRelationships                                                  = 1206,
    /// BSW-EXCLUSIVE-AREA-POLICY
    BswExclusiveAreaPolicy                                                 = 1038,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 5414,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 5988,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 1724,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 4911,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 5339,
    /// BSW-INTERNAL-TRIGGERING-POINT-POLICY
    BswInternalTriggeringPointPolicy                                       = 4363,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF
    BswInternalTriggeringPointRef                                          = 3289,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL
    BswInternalTriggeringPointRefConditional                               = 3580,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 2015,
    /// BSW-INTERRUPT-EVENT
    BswInterruptEvent                                                      = 1502,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 3635,
    /// BSW-MODE-GROUP-REF
    BswModeGroupRef                                                        = 1849,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 5996,
    /// BSW-MODE-RECEIVER-POLICY
    BswModeReceiverPolicy                                                  = 4783,
    /// BSW-MODE-SENDER-POLICY
    BswModeSenderPolicy                                                    = 3717,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 1903,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 4922,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 854,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 6169,
    /// BSW-MODULE-DEPENDENCYS
    BswModuleDependencys                                                   = 209,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 2449,
    /// BSW-MODULE-DESCRIPTION-REF
    BswModuleDescriptionRef                                                = 6282,
    /// BSW-MODULE-DESCRIPTION-REF-CONDITIONAL
    BswModuleDescriptionRefConditional                                     = 3918,
    /// BSW-MODULE-DOCUMENTATIONS
    BswModuleDocumentations                                                = 1468,
    /// BSW-MODULE-ENTITY-REF
    BswModuleEntityRef                                                     = 3483,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 2505,
    /// BSW-MODULE-ENTRY-REF
    BswModuleEntryRef                                                      = 5790,
    /// BSW-MODULE-ENTRY-REF-CONDITIONAL
    BswModuleEntryRefConditional                                           = 5846,
    /// BSW-MODULE-INSTANCE-REF
    BswModuleInstanceRef                                                   = 1929,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 3890,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 6261,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 1406,
    /// BSW-PARAMETER-POLICY
    BswParameterPolicy                                                     = 950,
    /// BSW-PER-INSTANCE-MEMORY-POLICY
    BswPerInstanceMemoryPolicy                                             = 681,
    /// BSW-PER-INSTANCE-MEMORY-POLICYS
    BswPerInstanceMemoryPolicys                                            = 509,
    /// BSW-QUEUED-DATA-RECEPTION-POLICY
    BswQueuedDataReceptionPolicy                                           = 3026,
    /// BSW-RELEASED-TRIGGER-POLICY
    BswReleasedTriggerPolicy                                               = 1268,
    /// BSW-RESOLUTION
    BswResolution                                                          = 4110,
    /// BSW-RESOURCE-ESTIMATION
    BswResourceEstimation                                                  = 5239,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 4058,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 3985,
    /// BSW-SERVICE-DEPENDENCY
    BswServiceDependency                                                   = 3523,
    /// BSW-SERVICE-DEPENDENCY-REF
    BswServiceDependencyRef                                                = 1686,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 3690,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 4638,
    /// BSW-TRIGGER-DIRECT-IMPLEMENTATION
    BswTriggerDirectImplementation                                         = 6132,
    /// BSW-TRIGGER-REF
    BswTriggerRef                                                          = 483,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 3481,
    /// BUCKET-SIZE
    BucketSize                                                             = 3592,
    /// BUFFER-COMPUTATION
    BufferComputation                                                      = 4738,
    /// BUFFER-OUTPUT
    BufferOutput                                                           = 2816,
    /// BUFFER-PROPERTIES
    BufferProperties                                                       = 918,
    /// BUFFER-RATIO
    BufferRatio                                                            = 2571,
    /// BUILD-ACTION
    BuildAction                                                            = 5723,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 5146,
    /// BUILD-ACTION-ENVIRONMENTS
    BuildActionEnvironments                                                = 1979,
    /// BUILD-ACTION-IO-ELEMENT
    BuildActionIoElement                                                   = 1548,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 1355,
    /// BUILD-ACTION-MANIFEST-REF
    BuildActionManifestRef                                                 = 4327,
    /// BUILD-ACTION-MANIFEST-REF-CONDITIONAL
    BuildActionManifestRefConditional                                      = 5084,
    /// BUILD-ACTION-MANIFESTS
    BuildActionManifests                                                   = 4547,
    /// BUILD-ACTIONS
    BuildActions                                                           = 2193,
    /// BUILD-TYPE
    BuildType                                                              = 1339,
    /// BULK-DATA-CATEGORY
    BulkDataCategory                                                       = 5230,
    /// BULK-NV-BLOCK
    BulkNvBlock                                                            = 2546,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 2464,
    /// BULK-NV-DATA-DESCRIPTORS
    BulkNvDataDescriptors                                                  = 6206,
    /// BUNDLED-CONNECTIONS
    BundledConnections                                                     = 759,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 4385,
    /// BURST-SIZE
    BurstSize                                                              = 1073,
    /// BURST-SIZE-REQUEST
    BurstSizeRequest                                                       = 741,
    /// BURST-SIZE-RESPONSE
    BurstSizeResponse                                                      = 1869,
    /// BUS-DEPENDENT-NM-ECUS
    BusDependentNmEcus                                                     = 5630,
    /// BUS-ID
    BusId                                                                  = 512,
    /// BUS-IDLE-TIMEOUT-PERIOD
    BusIdleTimeoutPeriod                                                   = 3281,
    /// BUS-MIRROR-CAN-ID-RANGE-MAPPING
    BusMirrorCanIdRangeMapping                                             = 3740,
    /// BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING
    BusMirrorCanIdToCanIdMapping                                           = 5542,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 2778,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 2980,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 3396,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 215,
    /// BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING
    BusMirrorLinPidToCanIdMapping                                          = 2332,
    /// BUS-MIRROR-NETWORK-ID
    BusMirrorNetworkId                                                     = 768,
    /// BUS-OFF-RECOVERY
    BusOffRecovery                                                         = 3687,
    /// BUS-SPECIFIC-NM-ECU
    BusSpecificNmEcu                                                       = 1944,
    /// BY-PASS-POINT-IREF
    ByPassPointIref                                                        = 4435,
    /// BY-PASS-POINT-IREFS
    ByPassPointIrefs                                                       = 4955,
    /// BYPASS-ETHER-TYPE
    BypassEtherType                                                        = 1685,
    /// BYPASS-ETHER-TYPES
    BypassEtherTypes                                                       = 1605,
    /// BYPASS-VLAN
    BypassVlan                                                             = 2087,
    /// BYPASS-VLANS
    BypassVlans                                                            = 6274,
    /// BYTE
    Byte                                                                   = 5829,
    /// BYTE-ORDER
    ByteOrder                                                              = 5225,
    /// BYTE-VALUE
    ByteValue                                                              = 2538,
    /// BYTE-VALUES
    ByteValues                                                             = 595,
    /// C-CODE
    CCode                                                                  = 1764,
    /// CALC-RAM-BLOCK-CRC
    CalcRamBlockCrc                                                        = 4611,
    /// CALCULATION-FORMULA
    CalculationFormula                                                     = 6086,
    /// CALIBRATION-PARAMETER-VALUE
    CalibrationParameterValue                                              = 4314,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 6426,
    /// CALIBRATION-PARAMETER-VALUE-SET-REF
    CalibrationParameterValueSetRef                                        = 777,
    /// CALIBRATION-PARAMETER-VALUE-SET-REFS
    CalibrationParameterValueSetRefs                                       = 5343,
    /// CALIBRATION-PARAMETER-VALUES
    CalibrationParameterValues                                             = 4918,
    /// CALL-POINTS
    CallPoints                                                             = 2986,
    /// CALL-SIGNAL-REF
    CallSignalRef                                                          = 3230,
    /// CALL-SIGNAL-TRIGGERING-REF
    CallSignalTriggeringRef                                                = 6015,
    /// CALL-TIMEOUT
    CallTimeout                                                            = 4467,
    /// CALL-TYPE
    CallType                                                               = 5735,
    /// CALLBACK-HEADER-REF
    CallbackHeaderRef                                                      = 2424,
    /// CALLBACK-HEADER-REFS
    CallbackHeaderRefs                                                     = 1590,
    /// CALLED-ENTRY-REF
    CalledEntryRef                                                         = 4133,
    /// CALLED-ENTRYS
    CalledEntrys                                                           = 5535,
    /// CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF
    CalledFromWithinExclusiveAreaRef                                       = 6382,
    /// CAN-ADDRESSING-MODE
    CanAddressingMode                                                      = 5797,
    /// CAN-BAUDRATE
    CanBaudrate                                                            = 5596,
    /// CAN-BE-INVOKED-CONCURRENTLY
    CanBeInvokedConcurrently                                               = 5368,
    /// CAN-BIT-RATE-SWITCH
    CanBitRateSwitch                                                       = 1989,
    /// CAN-CLUSTER
    CanCluster                                                             = 3881,
    /// CAN-CLUSTER-CONDITIONAL
    CanClusterConditional                                                  = 5260,
    /// CAN-CLUSTER-VARIANTS
    CanClusterVariants                                                     = 4752,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 4324,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 2689,
    /// CAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    CanCommunicationControllerConditional                                  = 4851,
    /// CAN-COMMUNICATION-CONTROLLER-VARIANTS
    CanCommunicationControllerVariants                                     = 1532,
    /// CAN-CONFIG
    CanConfig                                                              = 2965,
    /// CAN-CONTROLLER-ATTRIBUTES
    CanControllerAttributes                                                = 4394,
    /// CAN-CONTROLLER-CONFIGURATION
    CanControllerConfiguration                                             = 3491,
    /// CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS
    CanControllerConfigurationRequirements                                 = 3423,
    /// CAN-CONTROLLER-FD-ATTRIBUTES
    CanControllerFdAttributes                                              = 1949,
    /// CAN-CONTROLLER-FD-REQUIREMENTS
    CanControllerFdRequirements                                            = 5286,
    /// CAN-CONTROLLER-XL-ATTRIBUTES
    CanControllerXlAttributes                                              = 3691,
    /// CAN-CONTROLLER-XL-REQUIREMENTS
    CanControllerXlRequirements                                            = 4538,
    /// CAN-ENTER-EXCLUSIVE-AREA-REF
    CanEnterExclusiveAreaRef                                               = 5415,
    /// CAN-ENTER-EXCLUSIVE-AREA-REFS
    CanEnterExclusiveAreaRefs                                              = 4080,
    /// CAN-ENTERS
    CanEnters                                                              = 1924,
    /// CAN-FD-BAUDRATE
    CanFdBaudrate                                                          = 3245,
    /// CAN-FD-CONFIG
    CanFdConfig                                                            = 188,
    /// CAN-FD-FRAME-SUPPORT
    CanFdFrameSupport                                                      = 3974,
    /// CAN-FRAME
    CanFrame                                                               = 1426,
    /// CAN-FRAME-RX-BEHAVIOR
    CanFrameRxBehavior                                                     = 161,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 5199,
    /// CAN-FRAME-TX-BEHAVIOR
    CanFrameTxBehavior                                                     = 6329,
    /// CAN-GLOBAL-TIME-DOMAIN-PROPS
    CanGlobalTimeDomainProps                                               = 4182,
    /// CAN-ID-RANGE-MAPPINGS
    CanIdRangeMappings                                                     = 6022,
    /// CAN-ID-TO-CAN-ID-MAPPINGS
    CanIdToCanIdMappings                                                   = 5338,
    /// CAN-IDENTIFIER
    CanIdentifier                                                          = 1927,
    /// CAN-IDENTIFIER-MASK
    CanIdentifierMask                                                      = 3060,
    /// CAN-IDENTIFIER-RANGE
    CanIdentifierRange                                                     = 4500,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 3419,
    /// CAN-NM-CLUSTER-COUPLING
    CanNmClusterCoupling                                                   = 2284,
    /// CAN-NM-ECU
    CanNmEcu                                                               = 813,
    /// CAN-NM-NODE
    CanNmNode                                                              = 3591,
    /// CAN-NM-TX-CAN-ID
    CanNmTxCanId                                                           = 4751,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 4559,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 2866,
    /// CAN-TP-ADDRESS-REF
    CanTpAddressRef                                                        = 1669,
    /// CAN-TP-ADDRESS-REFS
    CanTpAddressRefs                                                       = 3268,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 5613,
    /// CAN-TP-CHANNEL-REF
    CanTpChannelRef                                                        = 6114,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 298,
    /// CAN-TP-CONNECTION
    CanTpConnection                                                        = 6119,
    /// CAN-TP-ECU
    CanTpEcu                                                               = 3300,
    /// CAN-TP-NODE
    CanTpNode                                                              = 6031,
    /// CAN-XL-BAUDRATE
    CanXlBaudrate                                                          = 6429,
    /// CAN-XL-CONFIG
    CanXlConfig                                                            = 3666,
    /// CAN-XL-CONFIG-REF
    CanXlConfigRef                                                         = 6335,
    /// CAN-XL-CONFIG-REQS
    CanXlConfigReqs                                                        = 5436,
    /// CAN-XL-FRAME-TRIGGERING-PROPS
    CanXlFrameTriggeringProps                                              = 5377,
    /// CAN-XL-NM-PROPS
    CanXlNmProps                                                           = 3461,
    /// CAN-XL-PROPS
    CanXlProps                                                             = 39,
    /// CAN-XL-PROPS-REF
    CanXlPropsRef                                                          = 2309,
    /// CAN-XL-PROPS-REFS
    CanXlPropsRefs                                                         = 6283,
    /// CANCELLATION
    Cancellation                                                           = 153,
    /// CAPABILITY
    Capability                                                             = 2451,
    /// CAPABILITY-RECORDS
    CapabilityRecords                                                      = 4361,
    /// CAPTION
    Caption                                                                = 1604,
    /// CAS-RX-LOW-MAX
    CasRxLowMax                                                            = 1854,
    /// CAT-2-ISR
    Cat2Isr                                                                = 3176,
    /// CATEGORY
    Category                                                               = 5835,
    /// CDD-TYPE
    CddType                                                                = 6014,
    /// CERT-COMMON-NAME
    CertCommonName                                                         = 4880,
    /// CERTIFICATE-EVALUATIONS
    CertificateEvaluations                                                 = 885,
    /// CERTIFICATE-REF
    CertificateRef                                                         = 5393,
    /// CERTIFICATE-TO-KEY-SLOT-MAPPINGS
    CertificateToKeySlotMappings                                           = 5974,
    /// CHANGE
    Change                                                                 = 4535,
    /// CHANNEL-ID
    ChannelId                                                              = 3618,
    /// CHANNEL-MODE
    ChannelMode                                                            = 68,
    /// CHANNEL-NAME
    ChannelName                                                            = 4693,
    /// CHANNEL-REF
    ChannelRef                                                             = 4303,
    /// CHANNEL-SYNCHRONOUS-WAKEUP
    ChannelSynchronousWakeup                                               = 3741,
    /// CHANNELS
    Channels                                                               = 5277,
    /// CHANNELS-PER-FRAME
    ChannelsPerFrame                                                       = 535,
    /// CHAPTER
    Chapter                                                                = 3888,
    /// CHAPTERS
    Chapters                                                               = 1930,
    /// CHARGE-MANAGER-NEEDS
    ChargeManagerNeeds                                                     = 1062,
    /// CHECK-PER-SOURCE-ID
    CheckPerSourceId                                                       = 5925,
    /// CHECK-STATIC-BLOCK-ID
    CheckStaticBlockId                                                     = 5470,
    /// CHECKPOINT-FINAL-REF
    CheckpointFinalRef                                                     = 5197,
    /// CHECKPOINT-FINAL-REFS
    CheckpointFinalRefs                                                    = 109,
    /// CHECKPOINT-ID
    CheckpointId                                                           = 863,
    /// CHECKPOINT-INITIAL-REF
    CheckpointInitialRef                                                   = 4274,
    /// CHECKPOINT-INITIAL-REFS
    CheckpointInitialRefs                                                  = 2631,
    /// CHECKPOINT-IREF
    CheckpointIref                                                         = 6111,
    /// CHECKPOINT-REF
    CheckpointRef                                                          = 98,
    /// CHECKPOINT-REFS
    CheckpointRefs                                                         = 2462,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 457,
    /// CHECKPOINT-TRANSITION-REF
    CheckpointTransitionRef                                                = 2264,
    /// CHECKPOINTS
    Checkpoints                                                            = 4593,
    /// CHECKPOINTSS
    Checkpointss                                                           = 3549,
    /// CHECKSUM-VALUE
    ChecksumValue                                                          = 1822,
    /// CHECKSUM-VERIFICATION
    ChecksumVerification                                                   = 3852,
    /// CHOICES
    Choices                                                                = 4061,
    /// CIPHER-SUITE
    CipherSuite                                                            = 5019,
    /// CIPHER-SUITE-CONFIGS
    CipherSuiteConfigs                                                     = 1973,
    /// CIPHER-SUITE-ID
    CipherSuiteId                                                          = 1168,
    /// CIPHER-SUITE-NAME
    CipherSuiteName                                                        = 586,
    /// CIPHER-SUITE-PRIORITY
    CipherSuitePriority                                                    = 1847,
    /// CIPHER-SUITE-SHORT-LABEL
    CipherSuiteShortLabel                                                  = 1458,
    /// CKN-REF
    CknRef                                                                 = 6049,
    /// CLAIMED-FUNCTION-GROUP-REF
    ClaimedFunctionGroupRef                                                = 1365,
    /// CLAIMED-FUNCTION-GROUP-REFS
    ClaimedFunctionGroupRefs                                               = 4604,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 3753,
    /// CLASS-CONTENTS
    ClassContents                                                          = 5523,
    /// CLASS-TAILORINGS
    ClassTailorings                                                        = 897,
    /// CLASSIFICATION
    Classification                                                         = 3129,
    /// CLASSIFICATIONS
    Classifications                                                        = 2224,
    /// CLEAR-CONDITION-GROUP-REF
    ClearConditionGroupRef                                                 = 877,
    /// CLEAR-CONDITION-REF
    ClearConditionRef                                                      = 3002,
    /// CLEAR-CONDITION-REFS
    ClearConditionRefs                                                     = 2624,
    /// CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF
    ClearDiagnosticInformationClassRef                                     = 3935,
    /// CLEAR-DTC-LIMITATION
    ClearDtcLimitation                                                     = 2792,
    /// CLEAR-EVENT-ALLOWED-BEHAVIOR
    ClearEventAllowedBehavior                                              = 3984,
    /// CLEAR-EVENT-BEHAVIOR
    ClearEventBehavior                                                     = 2141,
    /// CLEAR-FROM-VALID-TO-INVALID
    ClearFromValidToInvalid                                                = 3428,
    /// CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF
    ClearResetEmissionRelatedDiagnosticInfoClassRef                        = 23,
    /// CLIENT-CAPABILITY
    ClientCapability                                                       = 3711,
    /// CLIENT-COM-SPEC
    ClientComSpec                                                          = 3728,
    /// CLIENT-ID
    ClientId                                                               = 1610,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 189,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 4812,
    /// CLIENT-ID-DEFINITION-SET-REF
    ClientIdDefinitionSetRef                                               = 1510,
    /// CLIENT-ID-DEFINITION-SET-REFS
    ClientIdDefinitionSetRefs                                              = 594,
    /// CLIENT-ID-DEFINITIONS
    ClientIdDefinitions                                                    = 3981,
    /// CLIENT-ID-RANGE
    ClientIdRange                                                          = 2048,
    /// CLIENT-INTENT
    ClientIntent                                                           = 1611,
    /// CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST
    ClientIpAddrFromConnectionRequest                                      = 6260,
    /// CLIENT-POLICYS
    ClientPolicys                                                          = 5427,
    /// CLIENT-PORT-FROM-CONNECTION-REQUEST
    ClientPortFromConnectionRequest                                        = 2709,
    /// CLIENT-PORT-REF
    ClientPortRef                                                          = 4534,
    /// CLIENT-SERVER-ANNOTATION
    ClientServerAnnotation                                                 = 892,
    /// CLIENT-SERVER-ANNOTATIONS
    ClientServerAnnotations                                                = 3487,
    /// CLIENT-SERVER-APPLICATION-ERROR-MAPPING
    ClientServerApplicationErrorMapping                                    = 6411,
    /// CLIENT-SERVER-ARRAY-ELEMENT-MAPPING
    ClientServerArrayElementMapping                                        = 2303,
    /// CLIENT-SERVER-ARRAY-TYPE-MAPPING
    ClientServerArrayTypeMapping                                           = 2090,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 5202,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 58,
    /// CLIENT-SERVER-INTERFACE-REF
    ClientServerInterfaceRef                                               = 3210,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 3151,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 4586,
    /// CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING
    ClientServerOperationBlueprintMapping                                  = 2181,
    /// CLIENT-SERVER-OPERATION-COM-PROPS
    ClientServerOperationComProps                                          = 1919,
    /// CLIENT-SERVER-OPERATION-IREF
    ClientServerOperationIref                                              = 5390,
    /// CLIENT-SERVER-OPERATION-MAPPING
    ClientServerOperationMapping                                           = 2979,
    /// CLIENT-SERVER-OPERATION-REF
    ClientServerOperationRef                                               = 2710,
    /// CLIENT-SERVER-PORTS
    ClientServerPorts                                                      = 6334,
    /// CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING
    ClientServerPrimitiveTypeMapping                                       = 763,
    /// CLIENT-SERVER-RECORD-ELEMENT-MAPPING
    ClientServerRecordElementMapping                                       = 1520,
    /// CLIENT-SERVER-RECORD-TYPE-MAPPING
    ClientServerRecordTypeMapping                                          = 588,
    /// CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING
    ClientServerToSignalGroupMapping                                       = 4017,
    /// CLIENT-SERVER-TO-SIGNAL-MAPPING
    ClientServerToSignalMapping                                            = 4704,
    /// CLIENT-SERVICE-MAJOR-VERSION
    ClientServiceMajorVersion                                              = 4689,
    /// CLIENT-SERVICE-MINOR-VERSION
    ClientServiceMinorVersion                                              = 1934,
    /// CLOCK-REFERENCE-REF
    ClockReferenceRef                                                      = 3172,
    /// CLUSTER-DRIFT-DAMPING
    ClusterDriftDamping                                                    = 878,
    /// CLUSTERED-COMPONENT-IREF
    ClusteredComponentIref                                                 = 2857,
    /// CLUSTERED-COMPONENT-IREFS
    ClusteredComponentIrefs                                                = 2967,
    /// CM-MODULE-INSTANTIATION
    CmModuleInstantiation                                                  = 4230,
    /// CODE
    Code                                                                   = 254,
    /// CODE-DESCRIPTORS
    CodeDescriptors                                                        = 4042,
    /// CODE-LABEL
    CodeLabel                                                              = 5500,
    /// COLD-START-ATTEMPTS
    ColdStartAttempts                                                      = 2456,
    /// COLLECTED-INSTANCE-IREF
    CollectedInstanceIref                                                  = 6043,
    /// COLLECTED-INSTANCE-IREFS
    CollectedInstanceIrefs                                                 = 4972,
    /// COLLECTION
    Collection                                                             = 478,
    /// COLLECTION-PROPS
    CollectionProps                                                        = 4244,
    /// COLLECTION-REF
    CollectionRef                                                          = 636,
    /// COLLECTION-SEMANTICS
    CollectionSemantics                                                    = 2558,
    /// COLLECTION-THRESHOLD
    CollectionThreshold                                                    = 2974,
    /// COLLECTION-TIMEOUT
    CollectionTimeout                                                      = 3514,
    /// COLLECTION-TRIGGER
    CollectionTrigger                                                      = 6383,
    /// COLLISION-RESOLVING-SCHEDULE-REF
    CollisionResolvingScheduleRef                                          = 100,
    /// COLOR-MODE
    ColorMode                                                              = 2051,
    /// COLSPEC
    Colspec                                                                = 658,
    /// COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS
    ComBasedSignalGroupTransformations                                     = 3449,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 5459,
    /// COM-CONFIGURATION-GW-TIME-BASE
    ComConfigurationGwTimeBase                                             = 5784,
    /// COM-CONFIGURATION-ID
    ComConfigurationId                                                     = 1389,
    /// COM-CONFIGURATION-RX-TIME-BASE
    ComConfigurationRxTimeBase                                             = 3928,
    /// COM-CONFIGURATION-TX-TIME-BASE
    ComConfigurationTxTimeBase                                             = 2617,
    /// COM-CONTROL-CLASS-REF
    ComControlClassRef                                                     = 5708,
    /// COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION
    ComEnableMdtForCyclicTransmission                                      = 251,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 4275,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 5775,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 351,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 2180,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 439,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 1276,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 132,
    /// COM-MANAGEMENT-GROUP-REF
    ComManagementGroupRef                                                  = 5061,
    /// COM-MANAGEMENT-GROUP-REFS
    ComManagementGroupRefs                                                 = 3550,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 1450,
    /// COM-MANAGEMENT-MAPPINGS
    ComManagementMappings                                                  = 3235,
    /// COM-MANAGEMENT-PORT-GROUP-IREF
    ComManagementPortGroupIref                                             = 2921,
    /// COM-MANAGEMENT-PORT-GROUP-IREFS
    ComManagementPortGroupIrefs                                            = 5632,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 3640,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 4457,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 2978,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 1281,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 5279,
    /// COM-PROCESSING-PERIOD
    ComProcessingPeriod                                                    = 3865,
    /// COM-PROPS
    ComProps                                                               = 5734,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 5488,
    /// COM-TRIGGER-GRANT
    ComTriggerGrant                                                        = 2642,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 664,
    /// COMBINED-NO-DATA-INIT-COUNT
    CombinedNoDataInitCount                                                = 224,
    /// COMM-CONNECTORS
    CommConnectors                                                         = 4352,
    /// COMM-CONTROLLER-MAPPINGS
    CommControllerMappings                                                 = 5293,
    /// COMM-CONTROLLER-REF
    CommControllerRef                                                      = 3430,
    /// COMM-CONTROLLERS
    CommControllers                                                        = 4785,
    /// COMMAND
    Command                                                                = 1774,
    /// COMMANDS
    Commands                                                               = 3698,
    /// COMMENT
    Comment                                                                = 546,
    /// COMMITTED-BURST-SIZE
    CommittedBurstSize                                                     = 3456,
    /// COMMITTED-INFORMATION-RATE
    CommittedInformationRate                                               = 4336,
    /// COMMON-PROPERTIES
    CommonProperties                                                       = 2924,
    /// COMMON-SIGNAL-PATH
    CommonSignalPath                                                       = 5192,
    /// COMMUNICATION-BUFFER-LOCKING
    CommunicationBufferLocking                                             = 2475,
    /// COMMUNICATION-CLUSTER-REF
    CommunicationClusterRef                                                = 4544,
    /// COMMUNICATION-CLUSTER-REFS
    CommunicationClusterRefs                                               = 3117,
    /// COMMUNICATION-CONNECTOR-REF
    CommunicationConnectorRef                                              = 3055,
    /// COMMUNICATION-CONNECTOR-REF-CONDITIONAL
    CommunicationConnectorRefConditional                                   = 5104,
    /// COMMUNICATION-CONNECTOR-REFS
    CommunicationConnectorRefs                                             = 3805,
    /// COMMUNICATION-CONNECTORS
    CommunicationConnectors                                                = 6345,
    /// COMMUNICATION-CONTROLLER-MAPPING
    CommunicationControllerMapping                                         = 2472,
    /// COMMUNICATION-CONTROLLER-REF
    CommunicationControllerRef                                             = 2467,
    /// COMMUNICATION-CONTROLLERS
    CommunicationControllers                                               = 2886,
    /// COMMUNICATION-CYCLE
    CommunicationCycle                                                     = 3876,
    /// COMMUNICATION-DIRECTION
    CommunicationDirection                                                 = 1267,
    /// COMMUNICATION-MODE
    CommunicationMode                                                      = 187,
    /// COMMUNICATION-RESOURCE-PROPS
    CommunicationResourceProps                                             = 2906,
    /// COMMUNICATION-RESOURCE-REF
    CommunicationResourceRef                                               = 3041,
    /// COMPARE-TYPE
    CompareType                                                            = 4212,
    /// COMPARE-VALUE
    CompareValue                                                           = 1508,
    /// COMPILER
    Compiler                                                               = 5614,
    /// COMPILERS
    Compilers                                                              = 6349,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 1093,
    /// COMPLEX-TYPE-MAPPING
    ComplexTypeMapping                                                     = 899,
    /// COMPONENT-CLUSTERING
    ComponentClustering                                                    = 2992,
    /// COMPONENT-IMPLEMENTATION-REF
    ComponentImplementationRef                                             = 6433,
    /// COMPONENT-IREF
    ComponentIref                                                          = 5018,
    /// COMPONENT-IREFS
    ComponentIrefs                                                         = 5584,
    /// COMPONENT-QUALIFIER
    ComponentQualifier                                                     = 1775,
    /// COMPONENT-REF
    ComponentRef                                                           = 3006,
    /// COMPONENT-SEPARATION
    ComponentSeparation                                                    = 4550,
    /// COMPONENTS
    Components                                                             = 3377,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 3667,
    /// COMPOSITE-NETWORK-REPRESENTATION
    CompositeNetworkRepresentation                                         = 4164,
    /// COMPOSITE-NETWORK-REPRESENTATIONS
    CompositeNetworkRepresentations                                        = 625,
    /// COMPOSITE-RULE-BASED-VALUE-SPECIFICATION
    CompositeRuleBasedValueSpecification                                   = 2548,
    /// COMPOSITE-SERVICE-INTERFACE-REF
    CompositeServiceInterfaceRef                                           = 5992,
    /// COMPOSITE-TYPE-MAPPINGS
    CompositeTypeMappings                                                  = 5505,
    /// COMPOSITION-DATA-PROTOTYPE-REF
    CompositionDataPrototypeRef                                            = 5541,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 3137,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 5644,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 3629,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF
    CompositionSwComponentTypeRef                                          = 1360,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL
    CompositionSwComponentTypeRefConditional                               = 5883,
    /// COMPOUND-PRIMITIVE-ARGUMENTS
    CompoundPrimitiveArguments                                             = 183,
    /// COMPRESSED-SOFTWARE-PACKAGE-SIZE
    CompressedSoftwarePackageSize                                          = 1085,
    /// COMPU-CONST
    CompuConst                                                             = 4597,
    /// COMPU-DEFAULT-VALUE
    CompuDefaultValue                                                      = 1203,
    /// COMPU-DENOMINATOR
    CompuDenominator                                                       = 385,
    /// COMPU-INTERNAL-TO-PHYS
    CompuInternalToPhys                                                    = 145,
    /// COMPU-INVERSE-VALUE
    CompuInverseValue                                                      = 4376,
    /// COMPU-METHOD
    CompuMethod                                                            = 185,
    /// COMPU-METHOD-REF
    CompuMethodRef                                                         = 2914,
    /// COMPU-NUMERATOR
    CompuNumerator                                                         = 1409,
    /// COMPU-PHYS-TO-INTERNAL
    CompuPhysToInternal                                                    = 6284,
    /// COMPU-RATIONAL-COEFFS
    CompuRationalCoeffs                                                    = 4951,
    /// COMPU-SCALE
    CompuScale                                                             = 5190,
    /// COMPU-SCALES
    CompuScales                                                            = 317,
    /// COMPUTED
    Computed                                                               = 2459,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 1737,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 3775,
    /// COND
    Cond                                                                   = 2082,
    /// CONDITION
    Condition                                                              = 3594,
    /// CONDITION-ACCESS
    ConditionAccess                                                        = 1379,
    /// CONDITION-FORMULA
    ConditionFormula                                                       = 1163,
    /// CONDITIONAL-CHANGE-NAD
    ConditionalChangeNad                                                   = 1358,
    /// CONDITIONS
    Conditions                                                             = 15,
    /// CONFIDENCE-INTERVAL
    ConfidenceInterval                                                     = 2109,
    /// CONFIDENCE-INTERVALS
    ConfidenceIntervals                                                    = 6428,
    /// CONFIDENTIALITY-OFFSET
    ConfidentialityOffset                                                  = 1256,
    /// CONFIG-CLASS
    ConfigClass                                                            = 4608,
    /// CONFIG-ELEMENT-DEF-GLOBAL-REF
    ConfigElementDefGlobalRef                                              = 959,
    /// CONFIG-ELEMENT-DEF-LOCAL-REF
    ConfigElementDefLocalRef                                               = 6006,
    /// CONFIG-VARIANT
    ConfigVariant                                                          = 578,
    /// CONFIGURATION-CLASS-AFFECTION
    ConfigurationClassAffection                                            = 523,
    /// CONFIGURATION-HANDLING
    ConfigurationHandling                                                  = 815,
    /// CONFIGURED-NAD
    ConfiguredNad                                                          = 2306,
    /// CONFIRMATION-THRESHOLD
    ConfirmationThreshold                                                  = 1246,
    /// CONFLICTS
    Conflicts                                                              = 1897,
    /// CONFLICTS-TO
    ConflictsTo                                                            = 904,
    /// CONNECT
    Connect                                                                = 4705,
    /// CONNECTED-INDICATORS
    ConnectedIndicators                                                    = 4659,
    /// CONNECTION-BUNDLES
    ConnectionBundles                                                      = 4072,
    /// CONNECTION-IS-MANDATORY
    ConnectionIsMandatory                                                  = 2519,
    /// CONNECTION-NEGOTIATION-BEHAVIOR
    ConnectionNegotiationBehavior                                          = 1239,
    /// CONNECTION-PRIO-PDUS
    ConnectionPrioPdus                                                     = 4254,
    /// CONNECTION-TYPE
    ConnectionType                                                         = 4610,
    /// CONNECTIONS
    Connections                                                            = 2026,
    /// CONNECTOR-REF
    ConnectorRef                                                           = 6131,
    /// CONNECTOR-REFS
    ConnectorRefs                                                          = 2997,
    /// CONNECTORS
    Connectors                                                             = 1600,
    /// CONSIDER-BUILD-NUMBER
    ConsiderBuildNumber                                                    = 3859,
    /// CONSIDER-PTO-STATUS
    ConsiderPtoStatus                                                      = 2197,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 3368,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 1855,
    /// CONSISTENCY-NEEDSS
    ConsistencyNeedss                                                      = 3954,
    /// CONSTANT-MAPPING-REF
    ConstantMappingRef                                                     = 2785,
    /// CONSTANT-MAPPING-REFS
    ConstantMappingRefs                                                    = 4299,
    /// CONSTANT-MEMORYS
    ConstantMemorys                                                        = 4794,
    /// CONSTANT-REF
    ConstantRef                                                            = 221,
    /// CONSTANT-REFERENCE
    ConstantReference                                                      = 1235,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 3492,
    /// CONSTANT-SPECIFICATION-MAPPING
    ConstantSpecificationMapping                                           = 1981,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 533,
    /// CONSTANT-VALUE-MAPPING-REF
    ConstantValueMappingRef                                                = 3155,
    /// CONSTANT-VALUE-MAPPING-REFS
    ConstantValueMappingRefs                                               = 2125,
    /// CONSTR-LEVEL
    ConstrLevel                                                            = 5002,
    /// CONSTRAINT-REF
    ConstraintRef                                                          = 2302,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 1227,
    /// CONSTRAINT-TAILORINGS
    ConstraintTailorings                                                   = 6218,
    /// CONSUMED-DDS-OPERATIONS
    ConsumedDdsOperations                                                  = 6397,
    /// CONSUMED-DDS-SERVICE-EVENTS
    ConsumedDdsServiceEvents                                               = 732,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 105,
    /// CONSUMED-EVENT-GROUP-REF
    ConsumedEventGroupRef                                                  = 5110,
    /// CONSUMED-EVENT-GROUP-REFS
    ConsumedEventGroupRefs                                                 = 2290,
    /// CONSUMED-EVENT-GROUPS
    ConsumedEventGroups                                                    = 4407,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 6152,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF
    ConsumedProvidedServiceInstanceGroupRef                                = 1906,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL
    ConsumedProvidedServiceInstanceGroupRefConditional                     = 521,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 4852,
    /// CONSUMED-SERVICE-INSTANCE-REF
    ConsumedServiceInstanceRef                                             = 2900,
    /// CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL
    ConsumedServiceInstanceRefConditional                                  = 2564,
    /// CONSUMED-SERVICE-INSTANCES
    ConsumedServiceInstances                                               = 2334,
    /// CONTAINED-AR-ELEMENT-REF
    ContainedArElementRef                                                  = 4878,
    /// CONTAINED-AR-ELEMENT-REFS
    ContainedArElementRefs                                                 = 4150,
    /// CONTAINED-FIBEX-ELEMENT-REF
    ContainedFibexElementRef                                               = 1277,
    /// CONTAINED-FIBEX-ELEMENT-REFS
    ContainedFibexElementRefs                                              = 5576,
    /// CONTAINED-I-PDU-PROPS
    ContainedIPduProps                                                     = 4489,
    /// CONTAINED-I-PDU-TRIGGERING-PROPSS
    ContainedIPduTriggeringPropss                                          = 1770,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REF
    ContainedISignalIPduGroupRef                                           = 2279,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS
    ContainedISignalIPduGroupRefs                                          = 4531,
    /// CONTAINED-PACKAGE-ELEMENT-REF
    ContainedPackageElementRef                                             = 276,
    /// CONTAINED-PACKAGE-ELEMENT-REFS
    ContainedPackageElementRefs                                            = 2179,
    /// CONTAINED-PDU-TRIGGERING-REF
    ContainedPduTriggeringRef                                              = 4764,
    /// CONTAINED-PDU-TRIGGERING-REFS
    ContainedPduTriggeringRefs                                             = 4767,
    /// CONTAINED-PROCESS-REF
    ContainedProcessRef                                                    = 5895,
    /// CONTAINED-PROCESS-REFS
    ContainedProcessRefs                                                   = 5078,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 4102,
    /// CONTAINER-I-PDU-HEADER-BYTE-ORDER
    ContainerIPduHeaderByteOrder                                           = 2994,
    /// CONTAINER-TIMEOUT
    ContainerTimeout                                                       = 5231,
    /// CONTAINER-TRIGGER
    ContainerTrigger                                                       = 589,
    /// CONTAINERS
    Containers                                                             = 1667,
    /// CONTENT-TYPE
    ContentType                                                            = 65,
    /// CONTENT-URI
    ContentUri                                                             = 1776,
    /// CONTEXT-APPLICATION-RECORD-ELEMENT-REF
    ContextApplicationRecordElementRef                                     = 3145,
    /// CONTEXT-ATOMIC-COMPONENT-REF
    ContextAtomicComponentRef                                              = 6067,
    /// CONTEXT-BSW-IMPLEMENTATION-REF
    ContextBswImplementationRef                                            = 69,
    /// CONTEXT-COMPONENT-PROTOTYPE-REF
    ContextComponentPrototypeRef                                           = 1999,
    /// CONTEXT-COMPONENT-REF
    ContextComponentRef                                                    = 5139,
    /// CONTEXT-COMPONENT-REFS
    ContextComponentRefs                                                   = 4959,
    /// CONTEXT-COMPOSITION-REF
    ContextCompositionRef                                                  = 3861,
    /// CONTEXT-DATA-ELEMENT-IDENTIFICATIONS
    ContextDataElementIdentifications                                      = 3922,
    /// CONTEXT-DATA-ELEMENTS
    ContextDataElements                                                    = 3177,
    /// CONTEXT-DATA-PROTOTYPE-IN-CS-REF
    ContextDataPrototypeInCsRef                                            = 2383,
    /// CONTEXT-DATA-PROTOTYPE-IN-SR-REF
    ContextDataPrototypeInSrRef                                            = 1293,
    /// CONTEXT-DATA-PROTOTYPE-REF
    ContextDataPrototypeRef                                                = 1195,
    /// CONTEXT-DATA-PROTOTYPE-REFS
    ContextDataPrototypeRefs                                               = 6441,
    /// CONTEXT-DATA-SOURCE
    ContextDataSource                                                      = 4326,
    /// CONTEXT-DATAS
    ContextDatas                                                           = 1407,
    /// CONTEXT-DESCRIPTION
    ContextDescription                                                     = 1858,
    /// CONTEXT-ELEMENT-REF
    ContextElementRef                                                      = 3567,
    /// CONTEXT-ELEMENT-REFS
    ContextElementRefs                                                     = 2951,
    /// CONTEXT-ID
    ContextId                                                              = 5855,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF
    ContextImplementationDataElementRef                                    = 876,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS
    ContextImplementationDataElementRefs                                   = 2889,
    /// CONTEXT-LIMITATION-REF
    ContextLimitationRef                                                   = 2136,
    /// CONTEXT-LIMITATION-REFS
    ContextLimitationRefs                                                  = 4549,
    /// CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ContextModeDeclarationGroupPrototypeRef                                = 4723,
    /// CONTEXT-MODE-DECLARATION-GROUP-REF
    ContextModeDeclarationGroupRef                                         = 1392,
    /// CONTEXT-OPERATION-REF
    ContextOperationRef                                                    = 4497,
    /// CONTEXT-P-PORT-PROTOTYPE-REF
    ContextPPortPrototypeRef                                               = 3799,
    /// CONTEXT-P-PORT-REF
    ContextPPortRef                                                        = 423,
    /// CONTEXT-PORT-PROTOTYPE-REF
    ContextPortPrototypeRef                                                = 419,
    /// CONTEXT-PORT-REF
    ContextPortRef                                                         = 1049,
    /// CONTEXT-R-PORT-PROTOTYPE-REF
    ContextRPortPrototypeRef                                               = 3996,
    /// CONTEXT-R-PORT-REF
    ContextRPortRef                                                        = 294,
    /// CONTEXT-REF
    ContextRef                                                             = 2862,
    /// CONTEXT-ROOT-COMPONENT-REF
    ContextRootComponentRef                                                = 2394,
    /// CONTEXT-ROOT-COMPOSITION-REF
    ContextRootCompositionRef                                              = 6028,
    /// CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF
    ContextRootSwClusterDesignComponentPrototypeRef                        = 3370,
    /// CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF
    ContextRootSwComponentPrototypeRef                                     = 2544,
    /// CONTEXT-ROOT-SW-COMPOSITION-REF
    ContextRootSwCompositionRef                                            = 617,
    /// CONTEXT-SW-COMPONENT-PROTOTYPE-REF
    ContextSwComponentPrototypeRef                                         = 660,
    /// CONTEXT-SW-COMPONENT-REF
    ContextSwComponentRef                                                  = 3830,
    /// CONTEXT-SWC-PROTOTYPE-REF
    ContextSwcPrototypeRef                                                 = 2824,
    /// CONTEXT-SWC-PROTOTYPE-REFS
    ContextSwcPrototypeRefs                                                = 3718,
    /// CONTEXT-TYPE-LIST
    ContextTypeList                                                        = 850,
    /// CONTEXT-TYPE-LISTS
    ContextTypeLists                                                       = 6129,
    /// CONTEXTS
    Contexts                                                               = 5637,
    /// CONTRACT-VERSION
    ContractVersion                                                        = 2922,
    /// CONTROL-CONSUMED-EVENT-GROUP-REF
    ControlConsumedEventGroupRef                                           = 5783,
    /// CONTROL-CONSUMED-EVENT-GROUP-REFS
    ControlConsumedEventGroupRefs                                          = 1874,
    /// CONTROL-ENABLE-MASK-BITS
    ControlEnableMaskBits                                                  = 2091,
    /// CONTROL-OPTION-RECORD-PRESENT
    ControlOptionRecordPresent                                             = 2330,
    /// CONTROL-PNC-REF
    ControlPncRef                                                          = 4765,
    /// CONTROL-PNC-REFS
    ControlPncRefs                                                         = 3397,
    /// CONTROL-PROVIDED-EVENT-GROUP-REF
    ControlProvidedEventGroupRef                                           = 5148,
    /// CONTROL-PROVIDED-EVENT-GROUP-REFS
    ControlProvidedEventGroupRefs                                          = 3123,
    /// CONTROLLED-DATA-ELEMENT-REF
    ControlledDataElementRef                                               = 5823,
    /// CONTROLLED-DATA-ELEMENT-REFS
    ControlledDataElementRefs                                              = 3671,
    /// CONTROLLED-FID-REF
    ControlledFidRef                                                       = 3410,
    /// CONTROLLED-HW-ELEMENT-REF
    ControlledHwElementRef                                                 = 5493,
    /// CONTROLLER-REF
    ControllerRef                                                          = 5570,
    /// CORE-ID
    CoreId                                                                 = 4495,
    /// CORES
    Cores                                                                  = 5914,
    /// COUNT-PROFILE
    CountProfile                                                           = 6339,
    /// COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE
    CounterBasedFdcThresholdStorageValue                                   = 2884,
    /// COUNTER-DECREMENT-STEP-SIZE
    CounterDecrementStepSize                                               = 3452,
    /// COUNTER-FAILED-THRESHOLD
    CounterFailedThreshold                                                 = 1986,
    /// COUNTER-INCREMENT-STEP-SIZE
    CounterIncrementStepSize                                               = 2249,
    /// COUNTER-JUMP-DOWN
    CounterJumpDown                                                        = 4584,
    /// COUNTER-JUMP-DOWN-VALUE
    CounterJumpDownValue                                                   = 3716,
    /// COUNTER-JUMP-UP
    CounterJumpUp                                                          = 4118,
    /// COUNTER-JUMP-UP-VALUE
    CounterJumpUpValue                                                     = 5795,
    /// COUNTER-OFFSET
    CounterOffset                                                          = 1638,
    /// COUNTER-PASSED-THRESHOLD
    CounterPassedThreshold                                                 = 2041,
    /// COUPLED-CLUSTER-REF
    CoupledClusterRef                                                      = 1682,
    /// COUPLED-CLUSTER-REFS
    CoupledClusterRefs                                                     = 4408,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 3241,
    /// COUPLING-ELEMENT-DETAILS
    CouplingElementDetails                                                 = 234,
    /// COUPLING-ELEMENT-SWITCH-DETAILS
    CouplingElementSwitchDetails                                           = 441,
    /// COUPLING-FLAG
    CouplingFlag                                                           = 4396,
    /// COUPLING-PORT
    CouplingPort                                                           = 4707,
    /// COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER
    CouplingPortAsynchronousTrafficShaper                                  = 4272,
    /// COUPLING-PORT-CONNECTION
    CouplingPortConnection                                                 = 2511,
    /// COUPLING-PORT-CONNECTIONS
    CouplingPortConnections                                                = 500,
    /// COUPLING-PORT-CREDIT-BASED-SHAPER
    CouplingPortCreditBasedShaper                                          = 5619,
    /// COUPLING-PORT-DETAILS
    CouplingPortDetails                                                    = 1138,
    /// COUPLING-PORT-ENHANCED-TRAFFIC-SHAPER
    CouplingPortEnhancedTrafficShaper                                      = 5127,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 1754,
    /// COUPLING-PORT-RATE-POLICY
    CouplingPortRatePolicy                                                 = 650,
    /// COUPLING-PORT-REF
    CouplingPortRef                                                        = 275,
    /// COUPLING-PORT-REF-CONDITIONAL
    CouplingPortRefConditional                                             = 981,
    /// COUPLING-PORT-ROLE
    CouplingPortRole                                                       = 2527,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 4573,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 5975,
    /// COUPLING-PORT-SPEED
    CouplingPortSpeed                                                      = 319,
    /// COUPLING-PORT-STARTUP-ACTIVE-TIME
    CouplingPortStartupActiveTime                                          = 2784,
    /// COUPLING-PORT-STRUCTURAL-ELEMENTS
    CouplingPortStructuralElements                                         = 5269,
    /// COUPLING-PORT-SWITCHOFF-DELAY
    CouplingPortSwitchoffDelay                                             = 4124,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 498,
    /// COUPLING-PORTS
    CouplingPorts                                                          = 2262,
    /// COUPLING-TYPE
    CouplingType                                                           = 5704,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 1261,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 2702,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 3987,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 1098,
    /// CP-SOFTWARE-CLUSTER-REF
    CpSoftwareClusterRef                                                   = 3167,
    /// CP-SOFTWARE-CLUSTER-REF-CONDITIONAL
    CpSoftwareClusterRefConditional                                        = 5170,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 3316,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-REF
    CpSoftwareClusterResourceRef                                           = 4317,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 744,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 6265,
    /// CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterToApplicationPartitionMapping                         = 6000,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 1243,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 5756,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 5922,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 497,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 5666,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 5439,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 4513,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 6258,
    /// CPP-TEMPLATE-ARGUMENT
    CppTemplateArgument                                                    = 4471,
    /// CPU-USAGE
    CpuUsage                                                               = 151,
    /// CRC-CORRECTION-FIELD
    CrcCorrectionField                                                     = 712,
    /// CRC-DOMAIN-NUMBER
    CrcDomainNumber                                                        = 2214,
    /// CRC-FLAGS
    CrcFlags                                                               = 2795,
    /// CRC-MESSAGE-LENGTH
    CrcMessageLength                                                       = 5461,
    /// CRC-OFFSET
    CrcOffset                                                              = 3334,
    /// CRC-PRECISE-ORIGIN-TIMESTAMP
    CrcPreciseOriginTimestamp                                              = 6435,
    /// CRC-SECURED
    CrcSecured                                                             = 4313,
    /// CRC-SEQUENCE-ID
    CrcSequenceId                                                          = 2597,
    /// CRC-SOURCE-PORT-IDENTITY
    CrcSourcePortIdentity                                                  = 3171,
    /// CRC-VALIDATED
    CrcValidated                                                           = 3662,
    /// CREATE-ECU-WAKEUP-SOURCE
    CreateEcuWakeupSource                                                  = 5314,
    /// CREATED-DATAS
    CreatedDatas                                                           = 3017,
    /// CRF-PULL
    CrfPull                                                                = 3600,
    /// CRF-TYPE
    CrfType                                                                = 671,
    /// CRYPTO-ALG-ID
    CryptoAlgId                                                            = 4872,
    /// CRYPTO-ALGO-CONFIG
    CryptoAlgoConfig                                                       = 3649,
    /// CRYPTO-ALGORITHM-ID
    CryptoAlgorithmId                                                      = 136,
    /// CRYPTO-ALGORITHM-STRING
    CryptoAlgorithmString                                                  = 4111,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 3613,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 1939,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 2481,
    /// CRYPTO-CERTIFICATE-REF
    CryptoCertificateRef                                                   = 6422,
    /// CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING
    CryptoCertificateToCryptoKeySlotMapping                                = 3518,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 2182,
    /// CRYPTO-CERTIFICATES
    CryptoCertificates                                                     = 3994,
    /// CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS
    CryptoDesignToCryptoDriverMappings                                     = 2554,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 5540,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING
    CryptoDriverToCryptoJobMapping                                         = 5483,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS
    CryptoDriverToCryptoJobMappings                                        = 1057,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 2413,
    /// CRYPTO-JOB
    CryptoJob                                                              = 3080,
    /// CRYPTO-JOB-REF
    CryptoJobRef                                                           = 1990,
    /// CRYPTO-JOBS
    CryptoJobs                                                             = 5306,
    /// CRYPTO-KEY-DESCRIPTION
    CryptoKeyDescription                                                   = 418,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 3960,
    /// CRYPTO-KEY-REF
    CryptoKeyRef                                                           = 5938,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 552,
    /// CRYPTO-KEY-SLOT-CLIENT-INTERFACE
    CryptoKeySlotClientInterface                                           = 4242,
    /// CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE
    CryptoKeySlotContentAllowedUsage                                       = 2498,
    /// CRYPTO-KEY-SLOT-DESIGN
    CryptoKeySlotDesign                                                    = 3353,
    /// CRYPTO-KEY-SLOT-DESIGN-REF
    CryptoKeySlotDesignRef                                                 = 2071,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 1309,
    /// CRYPTO-KEY-SLOT-REF
    CryptoKeySlotRef                                                       = 1418,
    /// CRYPTO-KEY-SLOT-REFS
    CryptoKeySlotRefs                                                      = 1647,
    /// CRYPTO-KEY-SLOT-TO-CLIENT-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToClientPortPrototypeMapping                              = 6427,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 4282,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN
    CryptoKeySlotUsageDesign                                               = 3920,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN-MAPPING
    CryptoKeySlotUsageDesignMapping                                        = 3590,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN-REF
    CryptoKeySlotUsageDesignRef                                            = 3919,
    /// CRYPTO-KEY-SLOT-USER-DESIGN
    CryptoKeySlotUserDesign                                                = 3598,
    /// CRYPTO-KEY-SLOT-USER-DESIGN-MAPPING
    CryptoKeySlotUserDesignMapping                                         = 2384,
    /// CRYPTO-KEY-SLOT-USER-DESIGN-REF
    CryptoKeySlotUserDesignRef                                             = 1758,
    /// CRYPTO-KEY-SLOTS
    CryptoKeySlots                                                         = 3542,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 4916,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 354,
    /// CRYPTO-NEED-REF
    CryptoNeedRef                                                          = 4625,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 1968,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 1806,
    /// CRYPTO-OBJECT-TYPE
    CryptoObjectType                                                       = 2560,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 6418,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 1976,
    /// CRYPTO-PROVIDER-DOCUMENTATION-REF
    CryptoProviderDocumentationRef                                         = 1417,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 756,
    /// CRYPTO-PROVIDER-REF
    CryptoProviderRef                                                      = 5786,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 3267,
    /// CRYPTO-PROVIDERS
    CryptoProviders                                                        = 4533,
    /// CRYPTO-R-PORT-COM-SPEC
    CryptoRPortComSpec                                                     = 3136,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 1041,
    /// CRYPTO-SERVICE-CERTIFICATE-REF
    CryptoServiceCertificateRef                                            = 4675,
    /// CRYPTO-SERVICE-CERTIFICATE-REFS
    CryptoServiceCertificateRefs                                           = 3329,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 4232,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 6020,
    /// CRYPTO-SERVICE-KEY-REF
    CryptoServiceKeyRef                                                    = 2537,
    /// CRYPTO-SERVICE-MAPPINGS
    CryptoServiceMappings                                                  = 571,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 5650,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 4431,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 4672,
    /// CRYPTO-SERVICE-QUEUE-REF
    CryptoServiceQueueRef                                                  = 1151,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 2644,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 4252,
    /// CS-ERROR-REACTION
    CsErrorReaction                                                        = 6292,
    /// CSE-CODE
    CseCode                                                                = 3205,
    /// CSE-CODE-FACTOR
    CseCodeFactor                                                          = 4132,
    /// CURRENT-DATA-TYPE-REF
    CurrentDataTypeRef                                                     = 2375,
    /// CURRENT-EXP
    CurrentExp                                                             = 169,
    /// CURRENT-VALUE-REF
    CurrentValueRef                                                        = 5862,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 2826,
    /// CUSTOM-DOCUMENT-ELEMENT-REF
    CustomDocumentElementRef                                               = 2241,
    /// CUSTOM-DOCUMENTATION-REF
    CustomDocumentationRef                                                 = 1024,
    /// CUSTOM-SDG-DEF-REF
    CustomSdgDefRef                                                        = 886,
    /// CUSTOM-SDG-DEF-REFS
    CustomSdgDefRefs                                                       = 3303,
    /// CUSTOM-SERVICE-CLASS-REF
    CustomServiceClassRef                                                  = 4384,
    /// CUSTOM-SERVICE-ID
    CustomServiceId                                                        = 3005,
    /// CUSTOM-SPECIFICATION-REF
    CustomSpecificationRef                                                 = 5161,
    /// CUSTOM-SPECIFICATION-REFS
    CustomSpecificationRefs                                                = 2497,
    /// CUSTOM-SUB-FUNCTION-NUMBER
    CustomSubFunctionNumber                                                = 5710,
    /// CUSTOM-TRIGGER
    CustomTrigger                                                          = 4251,
    /// CYCLE
    Cycle                                                                  = 5944,
    /// CYCLE-AUTOSTART
    CycleAutostart                                                         = 4621,
    /// CYCLE-COUNT-MAX
    CycleCountMax                                                          = 2528,
    /// CYCLE-COUNTER
    CycleCounter                                                           = 3673,
    /// CYCLE-REPETITION
    CycleRepetition                                                        = 3405,
    /// CYCLE-STATUS-STORAGE
    CycleStatusStorage                                                     = 3921,
    /// CYCLE-TIME-MAIN-FUNCTION
    CycleTimeMainFunction                                                  = 3787,
    /// CYCLE-TIME-VALUE
    CycleTimeValue                                                         = 5082,
    /// CYCLIC-BACKUP-INTERVAL
    CyclicBackupInterval                                                   = 3780,
    /// CYCLIC-TIMING
    CyclicTiming                                                           = 4005,
    /// CYCLIC-WRITING-PERIOD
    CyclicWritingPeriod                                                    = 1252,
    /// DATA-COM-PROPS
    DataComProps                                                           = 6337,
    /// DATA-CONSISTENCY-POLICY
    DataConsistencyPolicy                                                  = 4716,
    /// DATA-CONSTR
    DataConstr                                                             = 4194,
    /// DATA-CONSTR-REF
    DataConstrRef                                                          = 3476,
    /// DATA-CONSTR-RULE
    DataConstrRule                                                         = 3950,
    /// DATA-CONSTR-RULES
    DataConstrRules                                                        = 4564,
    /// DATA-DUMP-ENTRY
    DataDumpEntry                                                          = 60,
    /// DATA-ELEMENT-IREF
    DataElementIref                                                        = 6098,
    /// DATA-ELEMENT-IREFS
    DataElementIrefs                                                       = 467,
    /// DATA-ELEMENT-REF
    DataElementRef                                                         = 5972,
    /// DATA-ELEMENT-REFS
    DataElementRefs                                                        = 5652,
    /// DATA-ELEMENTS
    DataElements                                                           = 198,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 2658,
    /// DATA-FILTER
    DataFilter                                                             = 2073,
    /// DATA-FILTER-TYPE
    DataFilterType                                                         = 4582,
    /// DATA-FORMAT-TAILORING
    DataFormatTailoring                                                    = 4085,
    /// DATA-ID
    DataId                                                                 = 5255,
    /// DATA-ID-MODE
    DataIdMode                                                             = 5475,
    /// DATA-ID-NIBBLE-OFFSET
    DataIdNibbleOffset                                                     = 3759,
    /// DATA-IDENTIFIER-REF
    DataIdentifierRef                                                      = 5830,
    /// DATA-IDENTIFIER-REFS
    DataIdentifierRefs                                                     = 2035,
    /// DATA-IDS
    DataIds                                                                = 2626,
    /// DATA-IREF
    DataIref                                                               = 2670,
    /// DATA-LENGTH
    DataLength                                                             = 2925,
    /// DATA-LENGTH-REQUEST
    DataLengthRequest                                                      = 1328,
    /// DATA-LENGTH-RESPONSE
    DataLengthResponse                                                     = 572,
    /// DATA-LINK-LAYER-RULE
    DataLinkLayerRule                                                      = 1139,
    /// DATA-MAPPINGS
    DataMappings                                                           = 4934,
    /// DATA-OFFSET
    DataOffset                                                             = 5813,
    /// DATA-PDU-REF
    DataPduRef                                                             = 5809,
    /// DATA-PROTECTION-KIND
    DataProtectionKind                                                     = 5844,
    /// DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF
    DataProtototypeInPortInterfaceRef                                      = 1439,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 2575,
    /// DATA-PROTOTYPE-GROUP-IREF
    DataPrototypeGroupIref                                                 = 2495,
    /// DATA-PROTOTYPE-GROUP-IREFS
    DataPrototypeGroupIrefs                                                = 5332,
    /// DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF
    DataPrototypeInClientServerInterfaceIref                               = 3228,
    /// DATA-PROTOTYPE-IN-METHOD-ARGUMENT-INSTANCE-REF-IREF
    DataPrototypeInMethodArgumentInstanceRefIref                           = 730,
    /// DATA-PROTOTYPE-IN-PORT-INTERFACE-REF
    DataPrototypeInPortInterfaceRef                                        = 3244,
    /// DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF
    DataPrototypeInSenderReceiverInterfaceIref                             = 5549,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF
    DataPrototypeInServiceInterfaceIref                                    = 2799,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF
    DataPrototypeInServiceInterfaceRef                                     = 6423,
    /// DATA-PROTOTYPE-IREF
    DataPrototypeIref                                                      = 5294,
    /// DATA-PROTOTYPE-MAPPING
    DataPrototypeMapping                                                   = 1670,
    /// DATA-PROTOTYPE-REF
    DataPrototypeRef                                                       = 5498,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPS
    DataPrototypeTransformationProps                                       = 795,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPSS
    DataPrototypeTransformationPropss                                      = 539,
    /// DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF
    DataPrototypeWithApplicationDataTypeInSystemRef                        = 3979,
    /// DATA-PROTOTYPES
    DataPrototypes                                                         = 3170,
    /// DATA-PROVIDER
    DataProvider                                                           = 6437,
    /// DATA-READ-ACCESSS
    DataReadAccesss                                                        = 538,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 1796,
    /// DATA-RECEIVE-POINT-BY-ARGUMENTS
    DataReceivePointByArguments                                            = 3547,
    /// DATA-RECEIVE-POINT-BY-VALUES
    DataReceivePointByValues                                               = 5942,
    /// DATA-RECEIVE-POINTS
    DataReceivePoints                                                      = 5971,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 5978,
    /// DATA-REF
    DataRef                                                                = 1873,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 2305,
    /// DATA-SEND-POINTS
    DataSendPoints                                                         = 1577,
    /// DATA-TRANSFER-CLASS-REF
    DataTransferClassRef                                                   = 2327,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 3251,
    /// DATA-TRANSFORMATION-KIND
    DataTransformationKind                                                 = 1019,
    /// DATA-TRANSFORMATION-REF
    DataTransformationRef                                                  = 3378,
    /// DATA-TRANSFORMATION-REF-CONDITIONAL
    DataTransformationRefConditional                                       = 3912,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 901,
    /// DATA-TRANSFORMATIONS
    DataTransformations                                                    = 5036,
    /// DATA-TYPE-FOR-SERIALIZATION-REF
    DataTypeForSerializationRef                                            = 1712,
    /// DATA-TYPE-FOR-SERIALIZATION-REFS
    DataTypeForSerializationRefs                                           = 4698,
    /// DATA-TYPE-MAP
    DataTypeMap                                                            = 3054,
    /// DATA-TYPE-MAPPING
    DataTypeMapping                                                        = 1587,
    /// DATA-TYPE-MAPPING-REF
    DataTypeMappingRef                                                     = 3451,
    /// DATA-TYPE-MAPPING-REFS
    DataTypeMappingRefs                                                    = 4315,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 1185,
    /// DATA-TYPE-MAPPING-SET-REF
    DataTypeMappingSetRef                                                  = 4319,
    /// DATA-TYPE-MAPPING-SET-REFS
    DataTypeMappingSetRefs                                                 = 5007,
    /// DATA-TYPE-MAPS
    DataTypeMaps                                                           = 4551,
    /// DATA-TYPE-POLICY
    DataTypePolicy                                                         = 6027,
    /// DATA-TYPE-REF
    DataTypeRef                                                            = 1334,
    /// DATA-TYPE-REFS
    DataTypeRefs                                                           = 733,
    /// DATA-UPDATE-PERIOD
    DataUpdatePeriod                                                       = 4372,
    /// DATA-WRITE-ACCESSS
    DataWriteAccesss                                                       = 4526,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 6168,
    /// DATE
    Date                                                                   = 1500,
    /// DCM-I-PDU
    DcmIPdu                                                                = 449,
    /// DDS-CP-CONFIG
    DdsCpConfig                                                            = 5722,
    /// DDS-CP-CONSUMED-SERVICE-INSTANCE
    DdsCpConsumedServiceInstance                                           = 3059,
    /// DDS-CP-DOMAIN
    DdsCpDomain                                                            = 3554,
    /// DDS-CP-I-SIGNAL-TO-DDS-TOPIC-MAPPING
    DdsCpISignalToDdsTopicMapping                                          = 2555,
    /// DDS-CP-PARTITION
    DdsCpPartition                                                         = 865,
    /// DDS-CP-PROVIDED-SERVICE-INSTANCE
    DdsCpProvidedServiceInstance                                           = 3778,
    /// DDS-CP-QOS-PROFILE
    DdsCpQosProfile                                                        = 5203,
    /// DDS-CP-SERVICE-INSTANCE-EVENT
    DdsCpServiceInstanceEvent                                              = 3356,
    /// DDS-CP-SERVICE-INSTANCE-OPERATION
    DdsCpServiceInstanceOperation                                          = 2445,
    /// DDS-CP-TOPIC
    DdsCpTopic                                                             = 4231,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 5612,
    /// DDS-DOMAINS
    DdsDomains                                                             = 6451,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 92,
    /// DDS-EVENT-QOS-PROFILE-REF
    DdsEventQosProfileRef                                                  = 4875,
    /// DDS-EVENT-QOS-PROPS
    DdsEventQosProps                                                       = 1896,
    /// DDS-EVENT-REF
    DdsEventRef                                                            = 4295,
    /// DDS-EVENT-TOPIC-REF
    DdsEventTopicRef                                                       = 4952,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 4260,
    /// DDS-FIELD-QOS-PROPS
    DdsFieldQosProps                                                       = 3182,
    /// DDS-FIELD-REPLY-TOPIC-REF
    DdsFieldReplyTopicRef                                                  = 3941,
    /// DDS-FIELD-REQUEST-TOPIC-REF
    DdsFieldRequestTopicRef                                                = 4980,
    /// DDS-I-SIGNAL-TO-TOPIC-MAPPINGS
    DdsISignalToTopicMappings                                              = 246,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 5560,
    /// DDS-METHOD-QOS-PROPS
    DdsMethodQosProps                                                      = 3774,
    /// DDS-METHOD-REPLY-TOPIC-REF
    DdsMethodReplyTopicRef                                                 = 137,
    /// DDS-METHOD-REQUEST-TOPIC-REF
    DdsMethodRequestTopicRef                                               = 2973,
    /// DDS-OPERATION-REQUEST-TRIGGERING-REF
    DdsOperationRequestTriggeringRef                                       = 3369,
    /// DDS-OPERATION-RESPONSE-TRIGGERING-REF
    DdsOperationResponseTriggeringRef                                      = 227,
    /// DDS-PARTITION-REF
    DdsPartitionRef                                                        = 734,
    /// DDS-PARTITIONS
    DdsPartitions                                                          = 5817,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 4318,
    /// DDS-QOS-PROFILE-REF
    DdsQosProfileRef                                                       = 219,
    /// DDS-QOS-PROFILES
    DdsQosProfiles                                                         = 4706,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 3068,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 1270,
    /// DDS-RPC-SERVICE-REF
    DdsRpcServiceRef                                                       = 844,
    /// DDS-RPC-SERVICES
    DdsRpcServices                                                         = 286,
    /// DDS-RULE
    DdsRule                                                                = 3638,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 4307,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 847,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 5900,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 4381,
    /// DDS-SERVICE-QOS-PROFILE-REF
    DdsServiceQosProfileRef                                                = 819,
    /// DDS-SERVICE-VERSION
    DdsServiceVersion                                                      = 5860,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 5387,
    /// DDS-TOPIC-REF
    DdsTopicRef                                                            = 5621,
    /// DDS-TOPICS
    DdsTopics                                                              = 6004,
    /// DEADLINE
    Deadline                                                               = 5714,
    /// DEADLINE-MAX
    DeadlineMax                                                            = 3605,
    /// DEADLINE-MIN
    DeadlineMin                                                            = 5141,
    /// DEADLINE-PERIOD
    DeadlinePeriod                                                         = 3065,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 6400,
    /// DEADLINE-SUPERVISIONS
    DeadlineSupervisions                                                   = 3372,
    /// DEBOUNCE-ALGORITHM
    DebounceAlgorithm                                                      = 3561,
    /// DEBOUNCE-ALGORITHM-PROPSS
    DebounceAlgorithmPropss                                                = 1946,
    /// DEBOUNCE-ALGORITHM-REF
    DebounceAlgorithmRef                                                   = 4876,
    /// DEBOUNCE-BEHAVIOR
    DebounceBehavior                                                       = 4802,
    /// DEBOUNCE-COUNTER-STORAGE
    DebounceCounterStorage                                                 = 1160,
    /// DEBOUNCE-TIME
    DebounceTime                                                           = 6225,
    /// DEBUG-INFOS
    DebugInfos                                                             = 5804,
    /// DECODING-CORRECTION
    DecodingCorrection                                                     = 5695,
    /// DECOMPOSITIONS
    Decompositions                                                         = 5987,
    /// DEF
    Def                                                                    = 5994,
    /// DEF-CALPRM-SET
    DefCalprmSet                                                           = 128,
    /// DEF-ITEM
    DefItem                                                                = 1369,
    /// DEF-LIST
    DefList                                                                = 4310,
    /// DEFAULT-ACTION
    DefaultAction                                                          = 2321,
    /// DEFAULT-APPLICATION-TIMEOUT
    DefaultApplicationTimeout                                              = 4276,
    /// DEFAULT-ENDIANNESS
    DefaultEndianness                                                      = 1082,
    /// DEFAULT-GATEWAY
    DefaultGateway                                                         = 2033,
    /// DEFAULT-LC-STATE-REF
    DefaultLcStateRef                                                      = 36,
    /// DEFAULT-LEASE-TIME
    DefaultLeaseTime                                                       = 3313,
    /// DEFAULT-LOG-THRESHOLD
    DefaultLogThreshold                                                    = 1422,
    /// DEFAULT-MODE-REF
    DefaultModeRef                                                         = 1374,
    /// DEFAULT-PATTERN
    DefaultPattern                                                         = 2374,
    /// DEFAULT-PERIOD-BEGIN
    DefaultPeriodBegin                                                     = 985,
    /// DEFAULT-PERIOD-END
    DefaultPeriodEnd                                                       = 5321,
    /// DEFAULT-PRIORITY
    DefaultPriority                                                        = 3421,
    /// DEFAULT-REPORTING-MODE
    DefaultReportingMode                                                   = 2850,
    /// DEFAULT-ROUTER
    DefaultRouter                                                          = 6209,
    /// DEFAULT-TRACE-STATE
    DefaultTraceState                                                      = 836,
    /// DEFAULT-TRAFFIC-CLASS
    DefaultTrafficClass                                                    = 6137,
    /// DEFAULT-VALUE
    DefaultValue                                                           = 634,
    /// DEFAULT-VALUE-ELEMENT
    DefaultValueElement                                                    = 2823,
    /// DEFAULT-VALUE-ELEMENTS
    DefaultValueElements                                                   = 898,
    /// DEFAULT-VALUE-HANDLING
    DefaultValueHandling                                                   = 3100,
    /// DEFAULT-VLAN-REF
    DefaultVlanRef                                                         = 925,
    /// DEFERRING-FID-REF
    DeferringFidRef                                                        = 974,
    /// DEFERRING-FID-REFS
    DeferringFidRefs                                                       = 1760,
    /// DEFINITION-REF
    DefinitionRef                                                          = 6311,
    /// DELAY
    Delay                                                                  = 4968,
    /// DELAY-COMPENSATION-A
    DelayCompensationA                                                     = 5667,
    /// DELAY-COMPENSATION-B
    DelayCompensationB                                                     = 1491,
    /// DELEGATED-PORT-ANNOTATION
    DelegatedPortAnnotation                                                = 490,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 3280,
    /// DELETES
    Deletes                                                                = 5833,
    /// DELIVERY-ARTIFACTS
    DeliveryArtifacts                                                      = 5077,
    /// DELTA-PACKAGE-APPLICABLE-VERSION
    DeltaPackageApplicableVersion                                          = 4473,
    /// DENOMINATOR-CONDITION
    DenominatorCondition                                                   = 5265,
    /// DENOMINATOR-GROUP
    DenominatorGroup                                                       = 3609,
    /// DEPENDENCIES
    Dependencies                                                           = 1205,
    /// DEPENDENCY
    Dependency                                                             = 3846,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 4871,
    /// DEPENDENT-RESOURCES
    DependentResources                                                     = 1403,
    /// DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF
    DependentSoftwareClusterDesignRef                                      = 3946,
    /// DEPENDENT-SOFTWARE-CLUSTER-REF
    DependentSoftwareClusterRef                                            = 6126,
    /// DEPENDS-ON
    DependsOn                                                              = 3484,
    /// DEPENDS-ON-REF
    DependsOnRef                                                           = 6309,
    /// DEPENDS-ON-REFS
    DependsOnRefs                                                          = 1348,
    /// DEPENDS-ONS
    DependsOns                                                             = 5096,
    /// DEPLOYMENT-REF
    DeploymentRef                                                          = 5205,
    /// DEPLOYMENT-URIS
    DeploymentUris                                                         = 1842,
    /// DERIVATION
    Derivation                                                             = 1410,
    /// DERIVED-CERTIFICATE-ACCEPTED
    DerivedCertificateAccepted                                             = 5111,
    /// DERIVED-FROM-BLUEPRINT-REF
    DerivedFromBlueprintRef                                                = 5421,
    /// DERIVED-FROM-BLUEPRINT-REFS
    DerivedFromBlueprintRefs                                               = 349,
    /// DERIVED-INTERFACE-REF
    DerivedInterfaceRef                                                    = 6420,
    /// DERIVED-OBJECT-REF
    DerivedObjectRef                                                       = 1537,
    /// DERIVED-PORT-INTERFACE-REF
    DerivedPortInterfaceRef                                                = 174,
    /// DERIVED-PORT-PROTOTYPE-REF
    DerivedPortPrototypeRef                                                = 3710,
    /// DESC
    Desc                                                                   = 5801,
    /// DESCRIPTION
    Description                                                            = 4451,
    /// DESIGN-REF
    DesignRef                                                              = 1054,
    /// DESIGN-REFS
    DesignRefs                                                             = 4655,
    /// DEST-META-CLASS
    DestMetaClass                                                          = 2722,
    /// DEST-SDG-REF
    DestSdgRef                                                             = 2753,
    /// DESTINATION-BASE-ID
    DestinationBaseId                                                      = 4929,
    /// DESTINATION-CONTEXT
    DestinationContext                                                     = 83,
    /// DESTINATION-IP-ADDRESS
    DestinationIpAddress                                                   = 5264,
    /// DESTINATION-IPV-4-ADDRESS
    DestinationIpv4Address                                                 = 3947,
    /// DESTINATION-IPV-6-ADDRESS
    DestinationIpv6Address                                                 = 118,
    /// DESTINATION-MAC-ADDRESS
    DestinationMacAddress                                                  = 3776,
    /// DESTINATION-MAC-ADDRESS-MASK
    DestinationMacAddressMask                                              = 4068,
    /// DESTINATION-MAX-ADDRESS
    DestinationMaxAddress                                                  = 6058,
    /// DESTINATION-MIN-ADDRESS
    DestinationMinAddress                                                  = 3512,
    /// DESTINATION-NETWORK-MASK
    DestinationNetworkMask                                                 = 3729,
    /// DESTINATION-ORDER
    DestinationOrder                                                       = 2364,
    /// DESTINATION-ORDER-KIND
    DestinationOrderKind                                                   = 3570,
    /// DESTINATION-PHYSICAL-ADDRESS
    DestinationPhysicalAddress                                             = 2461,
    /// DESTINATION-PORTS
    DestinationPorts                                                       = 1412,
    /// DESTINATION-REF
    DestinationRef                                                         = 5593,
    /// DESTINATION-REFS
    DestinationRefs                                                        = 409,
    /// DESTINATION-TYPE
    DestinationType                                                        = 4297,
    /// DESTINATION-URI-DEFS
    DestinationUriDefs                                                     = 2260,
    /// DESTINATION-URI-NESTING-CONTRACT
    DestinationUriNestingContract                                          = 1664,
    /// DESTINATION-URI-POLICY
    DestinationUriPolicy                                                   = 5074,
    /// DESTINATION-URI-REF
    DestinationUriRef                                                      = 1539,
    /// DESTINATION-URI-REFS
    DestinationUriRefs                                                     = 653,
    /// DETECT-NIT-ERROR
    DetectNitError                                                         = 272,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 752,
    /// DETERMINISTIC-CLIENT-REF
    DeterministicClientRef                                                 = 3501,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 1112,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDSS
    DeterministicClientResourceNeedss                                      = 4762,
    /// DETERMINISTIC-SYNC-M-OUT-OF-N
    DeterministicSyncMOutOfN                                               = 2404,
    /// DETERMINISTIC-SYNC-MASTER
    DeterministicSyncMaster                                                = 2457,
    /// DETERMINISTIC-SYNC-MASTER-REF
    DeterministicSyncMasterRef                                             = 6366,
    /// DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING
    DeterministicSyncMasterToTimeBaseConsumerMapping                       = 2683,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 5410,
    /// DEVELOPMENT-VALUE
    DevelopmentValue                                                       = 967,
    /// DHCP-ADDRESS-ASSIGNMENT
    DhcpAddressAssignment                                                  = 2628,
    /// DHCP-PROPS
    DhcpProps                                                              = 5680,
    /// DHCP-SERVER-CONFIGURATION
    DhcpServerConfiguration                                                = 3386,
    /// DIAG-ARG-INTEGRITY
    DiagArgIntegrity                                                       = 4166,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 41,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 1681,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 1739,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 1856,
    /// DIAG-FAULT-DETECTION-COUNTER-PORT-REF
    DiagFaultDetectionCounterPortRef                                       = 3158,
    /// DIAG-PDU-TYPE
    DiagPduType                                                            = 2618,
    /// DIAG-REQUIREMENT
    DiagRequirement                                                        = 2817,
    /// DIAG-ROUTINE-TYPE
    DiagRoutineType                                                        = 3923,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 5108,
    /// DIAGNOSTIC-ADDRESS
    DiagnosticAddress                                                      = 821,
    /// DIAGNOSTIC-ADDRESSES
    DiagnosticAddresses                                                    = 3429,
    /// DIAGNOSTIC-ADDRESSS
    DiagnosticAddresss                                                     = 2466,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 2259,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 4438,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE
    DiagnosticAuthTransmitCertificate                                      = 5136,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION
    DiagnosticAuthTransmitCertificateEvaluation                            = 4267,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING
    DiagnosticAuthTransmitCertificateMapping                               = 4087,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 5210,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 3599,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 1663,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 2423,
    /// DIAGNOSTIC-AUTHENTICATION-REF
    DiagnosticAuthenticationRef                                            = 3401,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 252,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 587,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 6101,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 6408,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 1030,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 1302,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 525,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 2043,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 411,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 5969,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 3626,
    /// DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL
    DiagnosticComControlSpecificChannel                                    = 654,
    /// DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL
    DiagnosticComControlSubNodeChannel                                     = 1695,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF
    DiagnosticCommonElementRef                                             = 5097,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL
    DiagnosticCommonElementRefConditional                                  = 840,
    /// DIAGNOSTIC-COMMON-PROPS-CONDITIONAL
    DiagnosticCommonPropsConditional                                       = 6237,
    /// DIAGNOSTIC-COMMON-PROPS-VARIANTS
    DiagnosticCommonPropsVariants                                          = 939,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 4397,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 398,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 2987,
    /// DIAGNOSTIC-CONDITION-REF
    DiagnosticConditionRef                                                 = 2693,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 4426,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 484,
    /// DIAGNOSTIC-CONNECTION-REF
    DiagnosticConnectionRef                                                = 6198,
    /// DIAGNOSTIC-CONNECTION-REF-CONDITIONAL
    DiagnosticConnectionRefConditional                                     = 4259,
    /// DIAGNOSTIC-CONNECTIONS
    DiagnosticConnections                                                  = 5023,
    /// DIAGNOSTIC-CONTRIBUTION-REF
    DiagnosticContributionRef                                              = 3942,
    /// DIAGNOSTIC-CONTRIBUTION-REFS
    DiagnosticContributionRefs                                             = 672,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 4983,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 3583,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 5831,
    /// DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT
    DiagnosticControlEnableMaskBit                                         = 5379,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 3286,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 826,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 5887,
    /// DIAGNOSTIC-DATA-CHANGE-TRIGGER
    DiagnosticDataChangeTrigger                                            = 635,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 6048,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 5997,
    /// DIAGNOSTIC-DATA-ELEMENT-REF
    DiagnosticDataElementRef                                               = 843,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 4548,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 1291,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 5960,
    /// DIAGNOSTIC-DATA-IDENTIFIER-REF
    DiagnosticDataIdentifierRef                                            = 3091,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 1710,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF
    DiagnosticDataIdentifierSetRef                                         = 1218,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL
    DiagnosticDataIdentifierSetRefConditional                              = 4420,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 4309,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 4632,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 4225,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 1084,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 1307,
    /// DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT
    DiagnosticDebounceBehaviorEnumValueVariationPoint                      = 4940,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 663,
    /// DIAGNOSTIC-DEPLOYMENT-PROPS-REF
    DiagnosticDeploymentPropsRef                                           = 1176,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 2579,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-PORT-MAPPING
    DiagnosticDoIpActivationLinePortMapping                                = 1212,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE
    DiagnosticDoIpEntityIdentificationInterface                            = 3905,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-PORT-MAPPING
    DiagnosticDoIpEntityIdentificationPortMapping                          = 2372,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 2593,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-PORT-MAPPING
    DiagnosticDoIpGroupIdentificationPortMapping                           = 5150,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 4881,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-PORT-MAPPING
    DiagnosticDoIpPowerModePortMapping                                     = 4712,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 2138,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-PORT-MAPPING
    DiagnosticDoIpTriggerVehicleAnnouncementPortMapping                    = 1144,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 3659,
    /// DIAGNOSTIC-DTC-CHANGE-TRIGGER
    DiagnosticDtcChangeTrigger                                             = 518,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 6044,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 5662,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 4221,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 117,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 511,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 5545,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 5065,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 4370,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 4445,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 1402,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 4772,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 2057,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF
    DiagnosticEnableConditionRef                                           = 5563,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL
    DiagnosticEnableConditionRefConditional                                = 4677,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 909,
    /// DIAGNOSTIC-ENV-CONDITION-FORMULA
    DiagnosticEnvConditionFormula                                          = 5478,
    /// DIAGNOSTIC-ENV-DATA-CONDITION
    DiagnosticEnvDataCondition                                             = 3192,
    /// DIAGNOSTIC-ENV-DATA-ELEMENT-CONDITION
    DiagnosticEnvDataElementCondition                                      = 5677,
    /// DIAGNOSTIC-ENV-MODE-CONDITION
    DiagnosticEnvModeCondition                                             = 851,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 1459,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 1691,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 556,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 3090,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 3376,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 2587,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 307,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 5931,
    /// DIAGNOSTIC-EVENT-REF
    DiagnosticEventRef                                                     = 1037,
    /// DIAGNOSTIC-EVENT-REF-CONDITIONAL
    DiagnosticEventRefConditional                                          = 4421,
    /// DIAGNOSTIC-EVENT-REFS
    DiagnosticEventRefs                                                    = 4690,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 1444,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 3460,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 5682,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 4234,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 4819,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 5532,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 6240,
    /// DIAGNOSTIC-EVENT-WINDOW
    DiagnosticEventWindow                                                  = 435,
    /// DIAGNOSTIC-EVENTS
    DiagnosticEvents                                                       = 1943,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 3507,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF
    DiagnosticExtendedDataRecordRef                                        = 373,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL
    DiagnosticExtendedDataRecordRefConditional                             = 1936,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION
    DiagnosticExternalAuthenticationIdentification                         = 4333,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 860,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 74,
    /// DIAGNOSTIC-EXTRACT-REF
    DiagnosticExtractRef                                                   = 5138,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 3910,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 673,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 4041,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 4220,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 6029,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 5533,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 1323,
    /// DIAGNOSTIC-FREEZE-FRAME-REF
    DiagnosticFreezeFrameRef                                               = 1880,
    /// DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL
    DiagnosticFreezeFrameRefConditional                                    = 1767,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 4969,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 3317,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 1225,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 5755,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 1707,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 6390,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 119,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 2908,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 5373,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 2337,
    /// DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT
    DiagnosticIndicatorTypeEnumValueVariationPoint                         = 2744,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 5720,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 1993,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 2888,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 1850,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 2322,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 4540,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 473,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 1197,
    /// DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER
    DiagnosticIumprGroupIdentifier                                         = 4553,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 1494,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 343,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 5344,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 4993,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 5857,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 4052,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 2676,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 574,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 1498,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 1646,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 1311,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 3436,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 2324,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 3046,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 772,
    /// DIAGNOSTIC-MEMORY-REF
    DiagnosticMemoryRef                                                    = 5917,
    /// DIAGNOSTIC-MESSAGE-TYPE
    DiagnosticMessageType                                                  = 262,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 3320,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 1319,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE
    DiagnosticMultipleConditionInterface                                   = 1473,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING
    DiagnosticMultipleConditionPortMapping                                 = 3296,
    /// DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE
    DiagnosticMultipleEventInterface                                       = 1064,
    /// DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING
    DiagnosticMultipleEventPortMapping                                     = 3955,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE
    DiagnosticMultipleMonitorInterface                                     = 5509,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING
    DiagnosticMultipleMonitorPortMapping                                   = 6056,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 503,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 2859,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 2757,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 5419,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF
    DiagnosticOperationCycleRef                                            = 846,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL
    DiagnosticOperationCycleRefConditional                                 = 4478,
    /// DIAGNOSTIC-PARAMETER
    DiagnosticParameter                                                    = 462,
    /// DIAGNOSTIC-PARAMETER-ELEMENT
    DiagnosticParameterElement                                             = 2964,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 2767,
    /// DIAGNOSTIC-PARAMETER-REF
    DiagnosticParameterRef                                                 = 4790,
    /// DIAGNOSTIC-PERIODIC-RATE
    DiagnosticPeriodicRate                                                 = 4906,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 978,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 4351,
    /// DIAGNOSTIC-PROPS
    DiagnosticProps                                                        = 6371,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 3704,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 4152,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 3355,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 2256,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 6317,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 5572,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 199,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 2201,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 2292,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 3425,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 5268,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 2447,
    /// DIAGNOSTIC-RELEVANCE
    DiagnosticRelevance                                                    = 6005,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 4461,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 1783,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 2304,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 968,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 4063,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 6331,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 2562,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 3889,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 3892,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 3754,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 1087,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 1870,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE
    DiagnosticRequestFileTransferInterface                                 = 5711,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 1595,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 5134,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 5482,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 530,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 157,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 3332,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 3275,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 4193,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 1665,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 390,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 1161,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 3757,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 5998,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 1733,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 1618,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 1465,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 3706,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 5615,
    /// DIAGNOSTIC-SECURE-CODING-MAPPING
    DiagnosticSecureCodingMapping                                          = 2468,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 2780,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 6407,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 719,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 2107,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 242,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 5155,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 4598,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 549,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 4499,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 4092,
    /// DIAGNOSTIC-SERVICE-INSTANCE-REF
    DiagnosticServiceInstanceRef                                           = 5172,
    /// DIAGNOSTIC-SERVICE-INSTANCE-REF-CONDITIONAL
    DiagnosticServiceInstanceRefConditional                                = 6054,
    /// DIAGNOSTIC-SERVICE-INSTANCES
    DiagnosticServiceInstances                                             = 5966,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 4030,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 5985,
    /// DIAGNOSTIC-SERVICE-TABLE-REF
    DiagnosticServiceTableRef                                              = 5816,
    /// DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL
    DiagnosticServiceTableRefConditional                                   = 4634,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 4271,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 4187,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 4195,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 207,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 676,
    /// DIAGNOSTIC-SESSION-REF
    DiagnosticSessionRef                                                   = 4173,
    /// DIAGNOSTIC-SESSION-REFS
    DiagnosticSessionRefs                                                  = 4329,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 1797,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE
    DiagnosticSovdAuthorizationInterface                                   = 5526,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING
    DiagnosticSovdAuthorizationPortMapping                                 = 261,
    /// DIAGNOSTIC-SOVD-BULK-DATA
    DiagnosticSovdBulkData                                                 = 3970,
    /// DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE
    DiagnosticSovdBulkDataInterface                                        = 2049,
    /// DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING
    DiagnosticSovdBulkDataPortMapping                                      = 6099,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA
    DiagnosticSovdConfigurationBulkData                                    = 1136,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING
    DiagnosticSovdConfigurationDataIdentifierMapping                       = 461,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE
    DiagnosticSovdConfigurationInterface                                   = 5950,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER
    DiagnosticSovdConfigurationParameter                                   = 4798,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING
    DiagnosticSovdConfigurationPortMapping                                 = 5405,
    /// DIAGNOSTIC-SOVD-FAULT-MEMORY-ACCESS
    DiagnosticSovdFaultMemoryAccess                                        = 5867,
    /// DIAGNOSTIC-SOVD-LOCK
    DiagnosticSovdLock                                                     = 3736,
    /// DIAGNOSTIC-SOVD-LOG
    DiagnosticSovdLog                                                      = 1706,
    /// DIAGNOSTIC-SOVD-METHOD
    DiagnosticSovdMethod                                                   = 782,
    /// DIAGNOSTIC-SOVD-METHOD-PRIMITIVE
    DiagnosticSovdMethodPrimitive                                          = 35,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE
    DiagnosticSovdProximityChallengeInterface                              = 3679,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING
    DiagnosticSovdProximityChallengePortMapping                            = 1785,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE
    DiagnosticSovdServiceValidationInterface                               = 369,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING
    DiagnosticSovdServiceValidationPortMapping                             = 5856,
    /// DIAGNOSTIC-SOVD-UPDATE
    DiagnosticSovdUpdate                                                   = 6061,
    /// DIAGNOSTIC-SOVD-UPDATE-INTERFACE
    DiagnosticSovdUpdateInterface                                          = 3596,
    /// DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING
    DiagnosticSovdUpdatePortMapping                                        = 3310,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 4081,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 3678,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 1029,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 6235,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF
    DiagnosticStorageConditionRef                                          = 3546,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL
    DiagnosticStorageConditionRefConditional                               = 1882,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 3940,
    /// DIAGNOSTIC-TEST-RESULT-REF
    DiagnosticTestResultRef                                                = 3474,
    /// DIAGNOSTIC-TEST-RESULT-REFS
    DiagnosticTestResultRefs                                               = 4833,
    /// DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT
    DiagnosticTestResultUpdateEnumValueVariationPoint                      = 4301,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 5779,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 4588,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 3337,
    /// DIAGNOSTIC-TRANSMIT-CERTIFICATE-INTERFACE
    DiagnosticTransmitCertificateInterface                                 = 5574,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 3541,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 2476,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 16,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD-PROPS
    DiagnosticTroubleCodeObdProps                                          = 4094,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 3224,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS-REF
    DiagnosticTroubleCodePropsRef                                          = 5995,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS-REF-CONDITIONAL
    DiagnosticTroubleCodePropsRefConditional                               = 1329,
    /// DIAGNOSTIC-TROUBLE-CODE-REF
    DiagnosticTroubleCodeRef                                               = 3227,
    /// DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL
    DiagnosticTroubleCodeRefConditional                                    = 1342,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 4378,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 4709,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 3434,
    /// DIAGNOSTIC-UDS-SEVERITY-ENUM-VALUE-VARIATION-POINT
    DiagnosticUdsSeverityEnumValueVariationPoint                           = 1650,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 1362,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 474,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 5029,
    /// DIAGNOSTIC-VALUE-ACCESS
    DiagnosticValueAccess                                                  = 320,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 266,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 5384,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 6368,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 1254,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 2728,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 806,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 3335,
    /// DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT
    DiagnosticWwhObdDtcClassEnumValueVariationPoint                        = 2274,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 2116,
    /// DID-NUMBER
    DidNumber                                                              = 53,
    /// DID-SIZE
    DidSize                                                                = 699,
    /// DIFFERENTIATED-SERVICE-CODE-POINT
    DifferentiatedServiceCodePoint                                         = 6387,
    /// DIFFERENTIATED-SERVICE-FIELD
    DifferentiatedServiceField                                             = 4575,
    /// DIRECT-PDU-REF
    DirectPduRef                                                           = 5961,
    /// DIRECT-SUCCESSOR-REF
    DirectSuccessorRef                                                     = 4566,
    /// DIRECT-SUCCESSOR-REFS
    DirectSuccessorRefs                                                    = 5345,
    /// DIRECT-TP-SDU-REF
    DirectTpSduRef                                                         = 984,
    /// DIRECTION
    Direction                                                              = 1102,
    /// DISABLE-END-TO-END-CHECK
    DisableEndToEndCheck                                                   = 2388,
    /// DISABLE-END-TO-END-STATE-MACHINE
    DisableEndToEndStateMachine                                            = 1222,
    /// DISABLED-IN-MODE-IREF
    DisabledInModeIref                                                     = 5206,
    /// DISABLED-IN-MODE-IREFS
    DisabledInModeIrefs                                                    = 1338,
    /// DISABLED-MODE-IREF
    DisabledModeIref                                                       = 1424,
    /// DISABLED-MODE-IREFS
    DisabledModeIrefs                                                      = 3983,
    /// DISCOVERY-PROTECTION-KIND
    DiscoveryProtectionKind                                                = 1269,
    /// DISCOVERY-TECHNOLOGY
    DiscoveryTechnology                                                    = 3973,
    /// DISCOVERY-TYPE
    DiscoveryType                                                          = 1055,
    /// DISPLAY-FORMAT
    DisplayFormat                                                          = 6355,
    /// DISPLAY-IDENTIFIER
    DisplayIdentifier                                                      = 3199,
    /// DISPLAY-NAME
    DisplayName                                                            = 4056,
    /// DISPLAY-PRESENTATION
    DisplayPresentation                                                    = 3271,
    /// DISTINGUISHED-PARTITIONS
    DistinguishedPartitions                                                = 3758,
    /// DLT-APPLICATION
    DltApplication                                                         = 3197,
    /// DLT-APPLICATION-REF
    DltApplicationRef                                                      = 3388,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 2118,
    /// DLT-ARGUMENT
    DltArgument                                                            = 1121,
    /// DLT-ARGUMENT-ENTRYS
    DltArgumentEntrys                                                      = 5724,
    /// DLT-ARGUMENTS
    DltArguments                                                           = 179,
    /// DLT-CONFIG
    DltConfig                                                              = 822,
    /// DLT-CONTEXT
    DltContext                                                             = 4529,
    /// DLT-CONTEXT-REF
    DltContextRef                                                          = 1461,
    /// DLT-CONTEXT-REF-CONDITIONAL
    DltContextRefConditional                                               = 466,
    /// DLT-ECU
    DltEcu                                                                 = 5162,
    /// DLT-ECU-ID
    DltEcuId                                                               = 5249,
    /// DLT-ECU-REF
    DltEcuRef                                                              = 6323,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 6373,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 2338,
    /// DLT-LOG-CHANNEL-DESIGN-REF
    DltLogChannelDesignRef                                                 = 2347,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 4171,
    /// DLT-LOG-CHANNEL-REF
    DltLogChannelRef                                                       = 2532,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 167,
    /// DLT-LOG-CHANNELS
    DltLogChannels                                                         = 3803,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 5762,
    /// DLT-LOG-SINK-REF
    DltLogSinkRef                                                          = 5175,
    /// DLT-LOG-SINK-REFS
    DltLogSinkRefs                                                         = 450,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 1215,
    /// DLT-MESSAGE
    DltMessage                                                             = 4502,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 6110,
    /// DLT-MESSAGE-REF
    DltMessageRef                                                          = 1617,
    /// DLT-MESSAGE-REF-CONDITIONAL
    DltMessageRefConditional                                               = 2376,
    /// DLT-MESSAGE-REFS
    DltMessageRefs                                                         = 841,
    /// DLT-MESSAGES
    DltMessages                                                            = 4184,
    /// DLT-SESSION-ID
    DltSessionId                                                           = 2649,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 5573,
    /// DNS-SERVER-ADDRESS
    DnsServerAddress                                                       = 1058,
    /// DNS-SERVER-ADDRESSES
    DnsServerAddresses                                                     = 4796,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 2623,
    /// DO-IP-CONFIG
    DoIpConfig                                                             = 1668,
    /// DO-IP-DESIGN-REF
    DoIpDesignRef                                                          = 3632,
    /// DO-IP-ENTITY
    DoIpEntity                                                             = 2607,
    /// DO-IP-ENTITY-ROLE
    DoIpEntityRole                                                         = 502,
    /// DO-IP-FUNCTIONAL-CLUSTER-DESIGN
    DoIpFunctionalClusterDesign                                            = 3344,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 437,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 1364,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 2998,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 1692,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 22,
    /// DO-IP-LOGIC-ADDRESS-PROPS
    DoIpLogicAddressProps                                                  = 842,
    /// DO-IP-LOGIC-ADDRESSS
    DoIpLogicAddresss                                                      = 4620,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 5751,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 849,
    /// DO-IP-LOGICAL-ADDRESS
    DoIpLogicalAddress                                                     = 3413,
    /// DO-IP-LOGICAL-ADDRESS-REF
    DoIpLogicalAddressRef                                                  = 5463,
    /// DO-IP-NETWORK-CONFIGURATION
    DoIpNetworkConfiguration                                               = 1344,
    /// DO-IP-NETWORK-CONFIGURATION-DESIGN
    DoIpNetworkConfigurationDesign                                         = 610,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 1368,
    /// DO-IP-PROTOCOL-VERSION
    DoIpProtocolVersion                                                    = 4210,
    /// DO-IP-REQUEST-CONFIGURATION
    DoIpRequestConfiguration                                               = 4946,
    /// DO-IP-REQUEST-CONFIGURATION-DESIGN
    DoIpRequestConfigurationDesign                                         = 4845,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 6073,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 4339,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 5346,
    /// DO-IP-ROUTING-ACTIVATIONS
    DoIpRoutingActivations                                                 = 3312,
    /// DO-IP-RULE
    DoIpRule                                                               = 5485,
    /// DO-IP-SOURCE-ADDRESS-REF
    DoIpSourceAddressRef                                                   = 1928,
    /// DO-IP-TARGET-ADDRESS-REF
    DoIpTargetAddressRef                                                   = 2395,
    /// DO-IP-TARGET-ADDRESS-REFS
    DoIpTargetAddressRefs                                                  = 3082,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REF
    DoIpTesterRoutingActivationRef                                         = 5039,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REFS
    DoIpTesterRoutingActivationRefs                                        = 4237,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 3856,
    /// DO-IP-TP-CONNECTION
    DoIpTpConnection                                                       = 3814,
    /// DO-NOT-FRAGMENT
    DoNotFragment                                                          = 3439,
    /// DOC-REVISION
    DocRevision                                                            = 3331,
    /// DOC-REVISIONS
    DocRevisions                                                           = 6374,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 4207,
    /// DOCUMENT-ELEMENT-SCOPES
    DocumentElementScopes                                                  = 2727,
    /// DOCUMENTATION
    Documentation                                                          = 1814,
    /// DOCUMENTATION-CONTENT
    DocumentationContent                                                   = 4920,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 5484,
    /// DOIP-CHANNEL-COLLECTION-REF
    DoipChannelCollectionRef                                               = 3360,
    /// DOIP-CONNECTION-REF
    DoipConnectionRef                                                      = 6193,
    /// DOIP-CONNECTION-REFS
    DoipConnectionRefs                                                     = 793,
    /// DOIP-INTERFACES
    DoipInterfaces                                                         = 2594,
    /// DOMAIN
    Domain                                                                 = 3029,
    /// DOMAIN-ID
    DomainId                                                               = 2291,
    /// DOMAIN-IDS
    DomainIds                                                              = 2650,
    /// DPD-ACTION
    DpdAction                                                              = 4201,
    /// DPD-DELAY
    DpdDelay                                                               = 5503,
    /// DPG-DOES-NOT-REQUIRE-COHERENCYS
    DpgDoesNotRequireCoherencys                                            = 2119,
    /// DPG-REQUIRES-COHERENCYS
    DpgRequiresCoherencys                                                  = 4799,
    /// DRIVER-NOTIFICATIONS
    DriverNotifications                                                    = 3085,
    /// DRIVER-PRIMITIVE-REF
    DriverPrimitiveRef                                                     = 4975,
    /// DROP-NOT-REQUESTED-NAD
    DropNotRequestedNad                                                    = 4857,
    /// DTC-FORMAT-TYPE
    DtcFormatType                                                          = 1310,
    /// DTC-KIND
    DtcKind                                                                = 5733,
    /// DTC-NUMBER
    DtcNumber                                                              = 1100,
    /// DTC-PROPS-REF
    DtcPropsRef                                                            = 1372,
    /// DTC-SETTING-CLASS-REF
    DtcSettingClassRef                                                     = 2961,
    /// DTC-SETTING-PARAMETER
    DtcSettingParameter                                                    = 1460,
    /// DTC-STATUS-AVAILABILITY-MASK
    DtcStatusAvailabilityMask                                              = 2195,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 5052,
    /// DTC-STATUS-MASK
    DtcStatusMask                                                          = 1167,
    /// DTCS
    Dtcs                                                                   = 683,
    /// DURABILITY
    Durability                                                             = 6084,
    /// DURABILITY-KIND
    DurabilityKind                                                         = 2934,
    /// DURABILITY-SERVICE
    DurabilityService                                                      = 6295,
    /// DURABILITY-SERVICE-CLEANUP-DELAY
    DurabilityServiceCleanupDelay                                          = 1298,
    /// DURABILITY-SERVICE-HISTORY-DEPTH
    DurabilityServiceHistoryDepth                                          = 3953,
    /// DURABILITY-SERVICE-HISTORY-KIND
    DurabilityServiceHistoryKind                                           = 271,
    /// DURABILITY-SERVICE-MAX-INSTANCES
    DurabilityServiceMaxInstances                                          = 3505,
    /// DURABILITY-SERVICE-MAX-SAMPLES
    DurabilityServiceMaxSamples                                            = 1549,
    /// DURABILITY-SERVICE-MAX-SAMPLES-PER-INSTANCE
    DurabilityServiceMaxSamplesPerInstance                                 = 3898,
    /// DYNAMIC-ACTION-REF
    DynamicActionRef                                                       = 5964,
    /// DYNAMIC-ACTION-REFS
    DynamicActionRefs                                                      = 4014,
    /// DYNAMIC-ARRAY-SIZE-PROFILE
    DynamicArraySizeProfile                                                = 5937,
    /// DYNAMIC-BS
    DynamicBs                                                              = 3148,
    /// DYNAMIC-LENGTH
    DynamicLength                                                          = 4755,
    /// DYNAMIC-PART
    DynamicPart                                                            = 3382,
    /// DYNAMIC-PART-ALTERNATIVE
    DynamicPartAlternative                                                 = 779,
    /// DYNAMIC-PART-ALTERNATIVES
    DynamicPartAlternatives                                                = 4312,
    /// DYNAMIC-PARTS
    DynamicParts                                                           = 5788,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REF
    DynamicPncMappingPduGroupRef                                           = 4135,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS
    DynamicPncMappingPduGroupRefs                                          = 4932,
    /// DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED
    DynamicPncToChannelMappingEnabled                                      = 2487,
    /// DYNAMIC-RUNTIME-LENGTH-HANDLING
    DynamicRuntimeLengthHandling                                           = 2588,
    /// DYNAMIC-SLOT-IDLE-PHASE
    DynamicSlotIdlePhase                                                   = 2036,
    /// DYNAMICALLY-ASSIGNED
    DynamicallyAssigned                                                    = 1248,
    /// DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF
    DynamicallyDefineDataIdentifierClassRef                                = 5803,
    /// E
    E                                                                      = 6117,
    /// E-2-E-EVENT-PROTECTION-PROPSS
    E2EEventProtectionPropss                                               = 481,
    /// E-2-E-METHOD-PROTECTION-PROPSS
    E2EMethodProtectionPropss                                              = 2216,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 5948,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS-REF
    E2EProfileCompatibilityPropsRef                                        = 5492,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 4508,
    /// E-2-E-PROFILE-CONFIGURATION-REF
    E2EProfileConfigurationRef                                             = 394,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 2416,
    /// E-2-E-PROFILE-CONFIGURATIONS
    E2EProfileConfigurations                                               = 5558,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 1884,
    /// ECU-COMM-PORT-INSTANCES
    EcuCommPortInstances                                                   = 4400,
    /// ECU-CONFIGURATION-REF
    EcuConfigurationRef                                                    = 97,
    /// ECU-EXTRACT-REF
    EcuExtractRef                                                          = 2052,
    /// ECU-EXTRACT-REFERENCE-IREF
    EcuExtractReferenceIref                                                = 2222,
    /// ECU-EXTRACT-VERSION
    EcuExtractVersion                                                      = 3850,
    /// ECU-ID
    EcuId                                                                  = 3264,
    /// ECU-INSTANCE
    EcuInstance                                                            = 2030,
    /// ECU-INSTANCE-PROPS
    EcuInstanceProps                                                       = 4055,
    /// ECU-INSTANCE-PROPSS
    EcuInstancePropss                                                      = 6259,
    /// ECU-INSTANCE-REF
    EcuInstanceRef                                                         = 3173,
    /// ECU-INSTANCE-REF-CONDITIONAL
    EcuInstanceRefConditional                                              = 2737,
    /// ECU-INSTANCE-REFS
    EcuInstanceRefs                                                        = 4977,
    /// ECU-INSTANCES
    EcuInstances                                                           = 1322,
    /// ECU-MAPPING
    EcuMapping                                                             = 5468,
    /// ECU-PARTITION
    EcuPartition                                                           = 344,
    /// ECU-PARTITION-REF
    EcuPartitionRef                                                        = 5435,
    /// ECU-REF
    EcuRef                                                                 = 751,
    /// ECU-RESET-CLASS-REF
    EcuResetClassRef                                                       = 6289,
    /// ECU-RESOURCE-ESTIMATION
    EcuResourceEstimation                                                  = 623,
    /// ECU-RESOURCE-MAPPINGS
    EcuResourceMappings                                                    = 2550,
    /// ECU-SCOPE-REF
    EcuScopeRef                                                            = 4165,
    /// ECU-SCOPE-REFS
    EcuScopeRefs                                                           = 1772,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 5452,
    /// ECU-TASK-PROXY-REF
    EcuTaskProxyRef                                                        = 116,
    /// ECU-TASK-PROXY-REFS
    EcuTaskProxyRefs                                                       = 6180,
    /// ECU-TIMING
    EcuTiming                                                              = 3480,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 6194,
    /// ECUC-ADD-INFO-PARAM-VALUE
    EcucAddInfoParamValue                                                  = 1738,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 3792,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 1543,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 1931,
    /// ECUC-COND
    EcucCond                                                               = 2794,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 3242,
    /// ECUC-DEF-EDITION
    EcucDefEdition                                                         = 1193,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 4458,
    /// ECUC-DEFINITION-REF
    EcucDefinitionRef                                                      = 3915,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 727,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 4902,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 260,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 6136,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 5083,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 1960,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 5334,
    /// ECUC-FUNCTION-NAME-DEF-CONDITIONAL
    EcucFunctionNameDefConditional                                         = 1337,
    /// ECUC-FUNCTION-NAME-DEF-VARIANTS
    EcucFunctionNameDefVariants                                            = 2414,
    /// ECUC-IMPLEMENTATION-CONFIGURATION-CLASS
    EcucImplementationConfigurationClass                                   = 4528,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 5166,
    /// ECUC-INSTANCE-REFERENCE-VALUE
    EcucInstanceReferenceValue                                             = 2353,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 2971,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 2629,
    /// ECUC-LINKER-SYMBOL-DEF-CONDITIONAL
    EcucLinkerSymbolDefConditional                                         = 3466,
    /// ECUC-LINKER-SYMBOL-DEF-VARIANTS
    EcucLinkerSymbolDefVariants                                            = 2253,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 399,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF
    EcucModuleConfigurationValuesRef                                       = 5015,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL
    EcucModuleConfigurationValuesRefConditional                            = 5868,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 17,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 2949,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL
    EcucMultilineStringParamDefConditional                                 = 5902,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS
    EcucMultilineStringParamDefVariants                                    = 147,
    /// ECUC-MULTIPLICITY-CONFIGURATION-CLASS
    EcucMultiplicityConfigurationClass                                     = 2415,
    /// ECUC-NUMERICAL-PARAM-VALUE
    EcucNumericalParamValue                                                = 5312,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 3416,
    /// ECUC-QUERY
    EcucQuery                                                              = 3092,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 3661,
    /// ECUC-QUERY-REF
    EcucQueryRef                                                           = 5454,
    /// ECUC-QUERY-STRING-REF
    EcucQueryStringRef                                                     = 55,
    /// ECUC-QUERYS
    EcucQuerys                                                             = 5008,
    /// ECUC-REF
    EcucRef                                                                = 5438,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 2173,
    /// ECUC-REFERENCE-VALUE
    EcucReferenceValue                                                     = 2196,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 4810,
    /// ECUC-STRING-PARAM-DEF-CONDITIONAL
    EcucStringParamDefConditional                                          = 4113,
    /// ECUC-STRING-PARAM-DEF-VARIANTS
    EcucStringParamDefVariants                                             = 1579,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 1749,
    /// ECUC-TEXTUAL-PARAM-VALUE
    EcucTextualParamValue                                                  = 126,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 6248,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 5636,
    /// ECUC-VALIDATION-CONDS
    EcucValidationConds                                                    = 5878,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 5131,
    /// ECUC-VALUE-CONFIGURATION-CLASS
    EcucValueConfigurationClass                                            = 6002,
    /// ECUC-VALUES
    EcucValues                                                             = 6360,
    /// EGRESS-PORT-REF
    EgressPortRef                                                          = 2422,
    /// EGRESS-PORT-REFS
    EgressPortRefs                                                         = 1103,
    /// EID
    Eid                                                                    = 5469,
    /// EID-RETRIEVAL
    EidRetrieval                                                           = 378,
    /// EID-USE-MAC
    EidUseMac                                                              = 5824,
    /// ELEMENT
    Element                                                                = 4867,
    /// ELEMENT-BYTE-VALUE
    ElementByteValue                                                       = 5241,
    /// ELEMENT-CREATION
    ElementCreation                                                        = 6271,
    /// ELEMENT-GROUPS
    ElementGroups                                                          = 6393,
    /// ELEMENT-IN-IMPL-DATATYPE
    ElementInImplDatatype                                                  = 1525,
    /// ELEMENT-MAPPINGS
    ElementMappings                                                        = 1592,
    /// ELEMENT-POSITION
    ElementPosition                                                        = 2004,
    /// ELEMENT-PROPSS
    ElementPropss                                                          = 3116,
    /// ELEMENT-REF
    ElementRef                                                             = 2319,
    /// ELEMENT-REFS
    ElementRefs                                                            = 2272,
    /// ELEMENT-ROLE
    ElementRole                                                            = 2378,
    /// ELEMENTS
    Elements                                                               = 3929,
    /// ELLIPTIC-CURVE-REF
    EllipticCurveRef                                                       = 2506,
    /// ELLIPTIC-CURVE-REFS
    EllipticCurveRefs                                                      = 3848,
    /// EMPTY-SIGNAL
    EmptySignal                                                            = 4238,
    /// EMULATION-SUPPORTS
    EmulationSupports                                                      = 1536,
    /// ENABLE-ANYCAST
    EnableAnycast                                                          = 4928,
    /// ENABLE-CONDITION-GROUP-REF
    EnableConditionGroupRef                                                = 2983,
    /// ENABLE-CONDITION-REF
    EnableConditionRef                                                     = 2226,
    /// ENABLE-CONDITIONS
    EnableConditions                                                       = 4515,
    /// ENABLE-DEACTIVATION
    EnableDeactivation                                                     = 1582,
    /// ENABLE-DISCOVERY-PROTECTION
    EnableDiscoveryProtection                                              = 1672,
    /// ENABLE-JOIN-ACCESS-CONTROL
    EnableJoinAccessControl                                                = 5547,
    /// ENABLE-LIVELINESS-PROTECTION
    EnableLivelinessProtection                                             = 455,
    /// ENABLE-READ-ACCESS-CONTROL
    EnableReadAccessControl                                                = 4433,
    /// ENABLE-TAKE-ADDRESS
    EnableTakeAddress                                                      = 6016,
    /// ENABLE-UPDATE
    EnableUpdate                                                           = 2282,
    /// ENABLE-WRITE-ACCESS-CONTROL
    EnableWriteAccessControl                                               = 1607,
    /// ENABLES-LOG-TRACE
    EnablesLogTrace                                                        = 175,
    /// ENCAPSULATED-ENTRY-REF
    EncapsulatedEntryRef                                                   = 2768,
    /// ENCODING
    Encoding                                                               = 6320,
    /// ENCRYPTION-REF
    EncryptionRef                                                          = 2200,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 1616,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 4446,
    /// END-ADDRESS
    EndAddress                                                             = 4839,
    /// END-TO-END-CALL-RESPONSE-TIMEOUT
    EndToEndCallResponseTimeout                                            = 6026,
    /// END-TO-END-PROFILE
    EndToEndProfile                                                        = 4236,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 4198,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 3524,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDUS
    EndToEndProtectionISignalIPdus                                         = 1104,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 945,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 3643,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPES
    EndToEndProtectionVariablePrototypes                                   = 1009,
    /// END-TO-END-PROTECTIONS
    EndToEndProtections                                                    = 3628,
    /// END-TO-END-TRANSFORMATION-COM-SPEC-PROPS
    EndToEndTransformationComSpecProps                                     = 310,
    /// END-TO-END-TRANSFORMATION-DESCRIPTION
    EndToEndTransformationDescription                                      = 2606,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS
    EndToEndTransformationISignalProps                                     = 2496,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    EndToEndTransformationISignalPropsConditional                          = 482,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    EndToEndTransformationISignalPropsVariants                             = 357,
    /// ENDPOINT-CONFIGURATION-REF
    EndpointConfigurationRef                                               = 3895,
    /// ENDPOINTS
    Endpoints                                                              = 3328,
    /// ENGINEERING-OBJECT
    EngineeringObject                                                      = 4334,
    /// ENGINEERING-OBJECTS
    EngineeringObjects                                                     = 5918,
    /// ENHANCED-MODE-API
    EnhancedModeApi                                                        = 2542,
    /// ENTER-TIMEOUT-VALUE
    EnterTimeoutValue                                                      = 5843,
    /// ENTERED-MODE-REF
    EnteredModeRef                                                         = 5073,
    /// ENTITY-STATUS-MAX-BYTE-FIELD-USE
    EntityStatusMaxByteFieldUse                                            = 4011,
    /// ENTITYS
    Entitys                                                                = 4658,
    /// ENTRY
    Entry                                                                  = 296,
    /// ENTRY-MODE-DECLARATION-REF
    EntryModeDeclarationRef                                                = 4263,
    /// ENTRY-REF
    EntryRef                                                               = 4010,
    /// ENTRY-TYPE
    EntryType                                                              = 3611,
    /// ENTRYS
    Entrys                                                                 = 3664,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 2864,
    /// ENUMERATOR-VALUE
    EnumeratorValue                                                        = 2898,
    /// ENUMS
    Enums                                                                  = 6082,
    /// ENVIRONMENT-CAPTURE-TO-REPORTING
    EnvironmentCaptureToReporting                                          = 2739,
    /// ENVIRONMENT-DATA-CAPTURE
    EnvironmentDataCapture                                                 = 3134,
    /// ENVIRONMENT-VARIABLES
    EnvironmentVariables                                                   = 3854,
    /// ENVIRONMENTAL-CONDITION-REF
    EnvironmentalConditionRef                                              = 3163,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 692,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 3677,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 4476,
    /// ERROR-CODE
    ErrorCode                                                              = 2493,
    /// ERROR-CONTEXT-REF
    ErrorContextRef                                                        = 4882,
    /// ERROR-CONTEXT-REFS
    ErrorContextRefs                                                       = 5457,
    /// ERROR-DOMAIN-REF
    ErrorDomainRef                                                         = 5590,
    /// ERROR-HANDLING
    ErrorHandling                                                          = 3249,
    /// ERROR-MAPPINGS
    ErrorMappings                                                          = 5973,
    /// ERROR-REACTION-POLICY
    ErrorReactionPolicy                                                    = 3074,
    /// ERROR-SIGNALING-ENABLED
    ErrorSignalingEnabled                                                  = 4196,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 2155,
    /// ESP-CIPHER-SUITE-NAME
    EspCipherSuiteName                                                     = 5927,
    /// ESP-CIPHER-SUITE-NAMES
    EspCipherSuiteNames                                                    = 4777,
    /// ESTIMATED-DURATION-OF-CAMPAIGN
    EstimatedDurationOfCampaign                                            = 2127,
    /// ESTIMATED-DURATION-OF-OPERATION
    EstimatedDurationOfOperation                                           = 5455,
    /// ESTIMATED-EXECUTION-TIME
    EstimatedExecutionTime                                                 = 824,
    /// ETH-GLOBAL-TIME-DOMAIN-PROPS
    EthGlobalTimeDomainProps                                               = 3601,
    /// ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT
    EthGlobalTimeManagedCouplingPort                                       = 4316,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 3917,
    /// ETH-IP-PROPS-REF
    EthIpPropsRef                                                          = 230,
    /// ETH-IP-PROPS-REFS
    EthIpPropsRefs                                                         = 2622,
    /// ETH-SWITCH-PORT-GROUP-DERIVATION
    EthSwitchPortGroupDerivation                                           = 492,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 1408,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 3681,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 4129,
    /// ETH-TP-CONNECTION
    EthTpConnection                                                        = 1755,
    /// ETHER-TYPE
    EtherType                                                              = 1376,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 2003,
    /// ETHERNET-CLUSTER-CONDITIONAL
    EthernetClusterConditional                                             = 1762,
    /// ETHERNET-CLUSTER-REF
    EthernetClusterRef                                                     = 282,
    /// ETHERNET-CLUSTER-VARIANTS
    EthernetClusterVariants                                                = 5152,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 6399,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 713,
    /// ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL
    EthernetCommunicationControllerConditional                             = 3032,
    /// ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS
    EthernetCommunicationControllerVariants                                = 5472,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 6246,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 194,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 4338,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 3223,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 812,
    /// ETHERNET-PRIORITY-REGENERATIONS
    EthernetPriorityRegenerations                                          = 4565,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 608,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 1899,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 5701,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING-REF
    EthernetRawDataStreamMappingRef                                        = 1885,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 1634,
    /// ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS
    EthernetTrafficClassAssignments                                        = 3752,
    /// ETHERNET-VLAN-TRANSLATION-TABLE
    EthernetVlanTranslationTable                                           = 1445,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 894,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 1481,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS
    EthernetWakeupSleepOnDatalineConfigs                                   = 3118,
    /// ETS-AVAILABLE-BANDWIDTH-IN-PERCENT
    EtsAvailableBandwidthInPercent                                         = 680,
    /// ETS-AVAILABLE-BANDWIDTH-IN-WEIGHT-VALUE
    EtsAvailableBandwidthInWeightValue                                     = 4290,
    /// EVALUATED-ELEMENT-REF
    EvaluatedElementRef                                                    = 442,
    /// EVALUATED-ELEMENT-REFS
    EvaluatedElementRefs                                                   = 3333,
    /// EVALUATED-VARIANT-REF
    EvaluatedVariantRef                                                    = 2772,
    /// EVALUATED-VARIANT-REFS
    EvaluatedVariantRefs                                                   = 5224,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 5737,
    /// EVALUATION-ID
    EvaluationId                                                           = 4039,
    /// EVENT-CLEAR-ALLOWED
    EventClearAllowed                                                      = 2242,
    /// EVENT-COMBINATION-REPORTING-BEHAVIOR
    EventCombinationReportingBehavior                                      = 799,
    /// EVENT-CONTROLLED-TIMING
    EventControlledTiming                                                  = 1923,
    /// EVENT-DEFAULT-VALUE
    EventDefaultValue                                                      = 2852,
    /// EVENT-DEPLOYMENTS
    EventDeployments                                                       = 5611,
    /// EVENT-DISPLACEMENT-STRATEGY
    EventDisplacementStrategy                                              = 1026,
    /// EVENT-ELEMENT-MAPPINGS
    EventElementMappings                                                   = 1623,
    /// EVENT-ELEMENTS
    EventElements                                                          = 4430,
    /// EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD
    EventFailureCycleCounterThreshold                                      = 1442,
    /// EVENT-GROUP-CONTROL-TYPE
    EventGroupControlType                                                  = 828,
    /// EVENT-GROUP-ID
    EventGroupId                                                           = 75,
    /// EVENT-GROUP-IDENTIFIER
    EventGroupIdentifier                                                   = 3508,
    /// EVENT-GROUP-REF
    EventGroupRef                                                          = 2355,
    /// EVENT-GROUP-REFS
    EventGroupRefs                                                         = 1346,
    /// EVENT-GROUPS
    EventGroups                                                            = 2993,
    /// EVENT-HANDLER
    EventHandler                                                           = 2671,
    /// EVENT-HANDLER-REF
    EventHandlerRef                                                        = 3211,
    /// EVENT-HANDLER-REFS
    EventHandlerRefs                                                       = 2131,
    /// EVENT-HANDLERS
    EventHandlers                                                          = 5661,
    /// EVENT-ID
    EventId                                                                = 4561,
    /// EVENT-IREF
    EventIref                                                              = 2448,
    /// EVENT-KIND
    EventKind                                                              = 3126,
    /// EVENT-MAPPING
    EventMapping                                                           = 6296,
    /// EVENT-MAPPINGS
    EventMappings                                                          = 1264,
    /// EVENT-MULTICAST-ADDRESSS
    EventMulticastAddresss                                                 = 5805,
    /// EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS
    EventMulticastSubscriptionAddresss                                     = 2896,
    /// EVENT-MULTICAST-UDP-PORT
    EventMulticastUdpPort                                                  = 4337,
    /// EVENT-NEEDS-REF
    EventNeedsRef                                                          = 5159,
    /// EVENT-OBD-READINESS-GROUP
    EventObdReadinessGroup                                                 = 2844,
    /// EVENT-OCCURRENCE-KIND
    EventOccurrenceKind                                                    = 1051,
    /// EVENT-PROPSS
    EventPropss                                                            = 5849,
    /// EVENT-QOS-PROPSS
    EventQosPropss                                                         = 4913,
    /// EVENT-READINESS-GROUPS
    EventReadinessGroups                                                   = 5689,
    /// EVENT-RECEPTION-DEFAULT-VALUE
    EventReceptionDefaultValue                                             = 1839,
    /// EVENT-REF
    EventRef                                                               = 2677,
    /// EVENT-REFS
    EventRefs                                                              = 1401,
    /// EVENT-SOURCE-REF
    EventSourceRef                                                         = 2147,
    /// EVENT-SYMBOL-NAME
    EventSymbolName                                                        = 1282,
    /// EVENT-TOPIC-ACCESS-RULE-REF
    EventTopicAccessRuleRef                                                = 4694,
    /// EVENT-WINDOW-TIME
    EventWindowTime                                                        = 3056,
    /// EVENT-WINDOWS
    EventWindows                                                           = 6021,
    /// EVENTS
    Events                                                                 = 4954,
    /// EXCESS-BURST-SIZE
    ExcessBurstSize                                                        = 4861,
    /// EXCESS-INFORMATION-RATE
    ExcessInformationRate                                                  = 6301,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 2760,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 1630,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REF
    ExclusiveAreaNestingOrderRef                                           = 6266,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REFS
    ExclusiveAreaNestingOrderRefs                                          = 4215,
    /// EXCLUSIVE-AREA-NESTING-ORDERS
    ExclusiveAreaNestingOrders                                             = 2916,
    /// EXCLUSIVE-AREA-POLICYS
    ExclusiveAreaPolicys                                                   = 1400,
    /// EXCLUSIVE-AREA-REF
    ExclusiveAreaRef                                                       = 5901,
    /// EXCLUSIVE-AREA-REF-CONDITIONAL
    ExclusiveAreaRefConditional                                            = 3545,
    /// EXCLUSIVE-AREA-REFS
    ExclusiveAreaRefs                                                      = 332,
    /// EXCLUSIVE-AREAS
    ExclusiveAreas                                                         = 4341,
    /// EXEC-IN-USER-MODE
    ExecInUserMode                                                         = 5213,
    /// EXECUTABLE
    Executable                                                             = 2945,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 3543,
    /// EXECUTABLE-ENTITY-REF
    ExecutableEntityRef                                                    = 2957,
    /// EXECUTABLE-ENTITY-REFS
    ExecutableEntityRefs                                                   = 1383,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 5122,
    /// EXECUTABLE-LOGGING-IMPLEMENTATION-PROPS
    ExecutableLoggingImplementationProps                                   = 3997,
    /// EXECUTABLE-PROVIDED-PORT-IREF
    ExecutableProvidedPortIref                                             = 4468,
    /// EXECUTABLE-REF
    ExecutableRef                                                          = 5875,
    /// EXECUTABLE-REFS
    ExecutableRefs                                                         = 4401,
    /// EXECUTABLE-REQUIRED-PORT-IREF
    ExecutableRequiredPortIref                                             = 1674,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 1472,
    /// EXECUTE-DESPITE-DATA-UNAVAILABILITY
    ExecuteDespiteDataUnavailability                                       = 5635,
    /// EXECUTION-CONTEXT
    ExecutionContext                                                       = 3656,
    /// EXECUTION-CONTEXT-REF
    ExecutionContextRef                                                    = 4158,
    /// EXECUTION-CONTEXT-REFS
    ExecutionContextRefs                                                   = 940,
    /// EXECUTION-CONTEXTS
    ExecutionContexts                                                      = 146,
    /// EXECUTION-DEPENDENCY
    ExecutionDependency                                                    = 3204,
    /// EXECUTION-DEPENDENCYS
    ExecutionDependencys                                                   = 1283,
    /// EXECUTION-ERROR
    ExecutionError                                                         = 4148,
    /// EXECUTION-ERROR-REF
    ExecutionErrorRef                                                      = 3273,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 6090,
    /// EXECUTION-ORDER-CONSTRAINT-TYPE
    ExecutionOrderConstraintType                                           = 124,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 4856,
    /// EXECUTION-TIME-TYPE
    ExecutionTimeType                                                      = 5088,
    /// EXECUTION-TIMES
    ExecutionTimes                                                         = 6115,
    /// EXIT-MODE-DECLARATION-REF
    ExitModeDeclarationRef                                                 = 1452,
    /// EXIT-TIMEOUT-VALUE
    ExitTimeoutValue                                                       = 3175,
    /// EXITED-MODE-REF
    ExitedModeRef                                                          = 425,
    /// EXPECTED-ALIVE-CYCLE
    ExpectedAliveCycle                                                     = 4099,
    /// EXPECTED-ALIVE-INDICATIONS
    ExpectedAliveIndications                                               = 3911,
    /// EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE
    ExpectedAliveIndicationsInSupervisionCycle                             = 1504,
    /// EXPECTED-CALLBACKS
    ExpectedCallbacks                                                      = 5514,
    /// EXPECTED-ENTRYS
    ExpectedEntrys                                                         = 493,
    /// EXPECTED-TX-TRIGGER
    ExpectedTxTrigger                                                      = 1974,
    /// EXPIRED-SUPERVISION-CYCLE-TOL
    ExpiredSupervisionCycleTol                                             = 6094,
    /// EXPIRED-SUPERVISION-CYCLES-TOLERANCE
    ExpiredSupervisionCyclesTolerance                                      = 3071,
    /// EXPIRED-SUPERVISION-TOLERANCE
    ExpiredSupervisionTolerance                                            = 2446,
    /// EXPLICIT-CONGESTION-NOTIFICATION
    ExplicitCongestionNotification                                         = 3207,
    /// EXPLICIT-INTER-RUNNABLE-VARIABLES
    ExplicitInterRunnableVariables                                         = 6089,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REF
    ExplicitRptProfileSelectionRef                                         = 2206,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REFS
    ExplicitRptProfileSelectionRefs                                        = 1228,
    /// EXPORTABILITY
    Exportability                                                          = 2812,
    /// EXPRESSION
    Expression                                                             = 2903,
    /// EXPRESSION-REF
    ExpressionRef                                                          = 6425,
    /// EXT-HEADER-FILTER-LISTS
    ExtHeaderFilterLists                                                   = 666,
    /// EXTENDED-ADDRESSING
    ExtendedAddressing                                                     = 5026,
    /// EXTENDED-DATA-RECORDS
    ExtendedDataRecords                                                    = 5653,
    /// EXTENDS-META-CLASS
    ExtendsMetaClass                                                       = 3243,
    /// EXTERN-OFFSET-CORRECTION
    ExternOffsetCorrection                                                 = 2567,
    /// EXTERN-RATE-CORRECTION
    ExternRateCorrection                                                   = 2708,
    /// EXTERNAL-AUTHENTICATIONS
    ExternalAuthentications                                                = 4612,
    /// EXTERNAL-CLOCK-SYNCHRONISATION
    ExternalClockSynchronisation                                           = 5310,
    /// EXTERNAL-REPLACEMENT-REF
    ExternalReplacementRef                                                 = 1871,
    /// EXTERNAL-SYNC
    ExternalSync                                                           = 1396,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 652,
    /// EXTERNAL-TRIGGERING-POINT
    ExternalTriggeringPoint                                                = 1226,
    /// EXTERNAL-TRIGGERING-POINTS
    ExternalTriggeringPoints                                               = 6421,
    /// FACTOR-SI-TO-UNIT
    FactorSiToUnit                                                         = 4455,
    /// FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL
    FailedAliveSupervisionRefCycleTol                                      = 3045,
    /// FAILED-REFERENCE-CYCLES-TOLERANCE
    FailedReferenceCyclesTolerance                                         = 6303,
    /// FAILED-SUPERVISION-CYCLES-TOLERANCE
    FailedSupervisionCyclesTolerance                                       = 4516,
    /// FAILURE-MONITORING-REF
    FailureMonitoringRef                                                   = 4138,
    /// FALL-BACK-INTERNAL
    FallBackInternal                                                       = 1088,
    /// FALSE-ACTION-LIST-REF
    FalseActionListRef                                                     = 5641,
    /// FDC-THRESHOLD-STORAGE-VALUE
    FdcThresholdStorageValue                                               = 1528,
    /// FEATURE-IREF
    FeatureIref                                                            = 3350,
    /// FEATURE-MODEL-REF
    FeatureModelRef                                                        = 2510,
    /// FEATURE-MODEL-REFS
    FeatureModelRefs                                                       = 4332,
    /// FEATURE-REF
    FeatureRef                                                             = 1519,
    /// FEATURE-REFS
    FeatureRefs                                                            = 5646,
    /// FIBEX-ELEMENT-REF
    FibexElementRef                                                        = 1145,
    /// FIBEX-ELEMENT-REF-CONDITIONAL
    FibexElementRefConditional                                             = 1113,
    /// FIBEX-ELEMENTS
    FibexElements                                                          = 3745,
    /// FIELD
    Field                                                                  = 5764,
    /// FIELD-DEPLOYMENTS
    FieldDeployments                                                       = 2543,
    /// FIELD-ELEMENTS
    FieldElements                                                          = 5730,
    /// FIELD-GET-SET-QOS-PROPSS
    FieldGetSetQosPropss                                                   = 1003,
    /// FIELD-IREF
    FieldIref                                                              = 4286,
    /// FIELD-MAPPING
    FieldMapping                                                           = 638,
    /// FIELD-MAPPINGS
    FieldMappings                                                          = 436,
    /// FIELD-NOTIFIER-QOS-PROPSS
    FieldNotifierQosPropss                                                 = 5553,
    /// FIELD-NOTIFIER-REF
    FieldNotifierRef                                                       = 739,
    /// FIELD-REF
    FieldRef                                                               = 6308,
    /// FIELD-REFS
    FieldRefs                                                              = 2779,
    /// FIELD-REPLY-TOPIC-NAME
    FieldReplyTopicName                                                    = 872,
    /// FIELD-REQUEST-TOPIC-NAME
    FieldRequestTopicName                                                  = 1147,
    /// FIELD-SENDER-COM-SPEC
    FieldSenderComSpec                                                     = 3513,
    /// FIELD-TOPICS-ACCESS-RULE-REF
    FieldTopicsAccessRuleRef                                               = 6204,
    /// FIELDS
    Fields                                                                 = 3122,
    /// FIFO-DEPTH
    FifoDepth                                                              = 577,
    /// FIFO-RANGES
    FifoRanges                                                             = 5881,
    /// FIGURE
    Figure                                                                 = 3511,
    /// FIGURE-CAPTION
    FigureCaption                                                          = 3560,
    /// FILE-ELEMENTS
    FileElements                                                           = 3338,
    /// FILE-INFO-COMMENT
    FileInfoComment                                                        = 1043,
    /// FILE-NAME
    FileName                                                               = 309,
    /// FILE-PROXYS
    FileProxys                                                             = 472,
    /// FILE-REF
    FileRef                                                                = 4769,
    /// FILE-REFS
    FileRefs                                                               = 2729,
    /// FILE-STORAGE-REF
    FileStorageRef                                                         = 6403,
    /// FILE-TYPE
    FileType                                                               = 3218,
    /// FILE-TYPE-PATTERN
    FileTypePattern                                                        = 2820,
    /// FILES
    Files                                                                  = 977,
    /// FILTER
    Filter                                                                 = 1149,
    /// FILTER-ACTION-BLOCK-SOURCE
    FilterActionBlockSource                                                = 4627,
    /// FILTER-ACTION-DEST-PORT-MODIFICATION
    FilterActionDestPortModification                                       = 3324,
    /// FILTER-ACTION-DROP-FRAME
    FilterActionDropFrame                                                  = 858,
    /// FILTER-ACTION-VLAN-MODIFICATION
    FilterActionVlanModification                                           = 4714,
    /// FILTER-CHAINS
    FilterChains                                                           = 5828,
    /// FILTER-PRIORITY
    FilterPriority                                                         = 2734,
    /// FILTERING-DEBOUNCING
    FilteringDebouncing                                                    = 6102,
    /// FINAL-CHECKPOINT-REF
    FinalCheckpointRef                                                     = 7,
    /// FINAL-CHECKPOINT-REFS
    FinalCheckpointRefs                                                    = 2832,
    /// FIRE-AND-FORGET
    FireAndForget                                                          = 1380,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 1807,
    /// FIRE-AND-FORGET-MAPPINGS
    FireAndForgetMappings                                                  = 5433,
    /// FIRE-AND-FORGET-METHOD-MAPPING
    FireAndForgetMethodMapping                                             = 2533,
    /// FIRE-AND-FORGET-METHOD-MAPPINGS
    FireAndForgetMethodMappings                                            = 4108,
    /// FIREWALL-REF
    FirewallRef                                                            = 1053,
    /// FIREWALL-RULE
    FirewallRule                                                           = 1550,
    /// FIREWALL-RULE-PROPS
    FirewallRuleProps                                                      = 4743,
    /// FIREWALL-RULE-PROPSS
    FirewallRulePropss                                                     = 3260,
    /// FIREWALL-RULE-REF
    FirewallRuleRef                                                        = 1046,
    /// FIREWALL-RULE-REFS
    FirewallRuleRefs                                                       = 2156,
    /// FIREWALL-STATE-IREF
    FirewallStateIref                                                      = 2580,
    /// FIREWALL-STATE-IREFS
    FirewallStateIrefs                                                     = 4181,
    /// FIREWALL-STATE-MACHINES
    FirewallStateMachines                                                  = 5956,
    /// FIREWALL-STATE-MODE-DECLARATION-REF
    FirewallStateModeDeclarationRef                                        = 2047,
    /// FIREWALL-STATE-MODE-DECLARATION-REFS
    FirewallStateModeDeclarationRefs                                       = 5313,
    /// FIREWALL-STATE-SWITCH-INTERFACE
    FirewallStateSwitchInterface                                           = 1173,
    /// FIRST-APPLICATION-ERROR-REF
    FirstApplicationErrorRef                                               = 1074,
    /// FIRST-DATA-PROTOTYPE-REF
    FirstDataPrototypeRef                                                  = 1347,
    /// FIRST-ELEMENT-INSTANCE-IREF
    FirstElementInstanceIref                                               = 4746,
    /// FIRST-ELEMENT-INSTANCE-IREFS
    FirstElementInstanceIrefs                                              = 3809,
    /// FIRST-ELEMENT-REF
    FirstElementRef                                                        = 5839,
    /// FIRST-ELEMENT-REFS
    FirstElementRefs                                                       = 4386,
    /// FIRST-ELEMENTS
    FirstElements                                                          = 6217,
    /// FIRST-MODE-GROUP-REF
    FirstModeGroupRef                                                      = 2142,
    /// FIRST-MODE-REF
    FirstModeRef                                                           = 6276,
    /// FIRST-MODE-REFS
    FirstModeRefs                                                          = 4841,
    /// FIRST-OPERATION-REF
    FirstOperationRef                                                      = 3499,
    /// FIRST-PHYSICAL-DIMENSION-REF
    FirstPhysicalDimensionRef                                              = 371,
    /// FIRST-PORT-REF
    FirstPortRef                                                           = 2400,
    /// FIRST-TIMEOUT
    FirstTimeout                                                           = 1935,
    /// FIRST-TO-SECOND-DATA-TRANSFORMATION-REF
    FirstToSecondDataTransformationRef                                     = 4006,
    /// FIRST-TRIGGER-REF
    FirstTriggerRef                                                        = 3208,
    /// FIRST-VALUE
    FirstValue                                                             = 2301,
    /// FIXED-LENGTH
    FixedLength                                                            = 4320,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 938,
    /// FLAT-INSTANCE-REF
    FlatInstanceRef                                                        = 2360,
    /// FLAT-MAP
    FlatMap                                                                = 6443,
    /// FLAT-MAP-ENTRY-REF
    FlatMapEntryRef                                                        = 2699,
    /// FLAT-MAP-ENTRY-REFS
    FlatMapEntryRefs                                                       = 1166,
    /// FLAT-MAP-REF
    FlatMapRef                                                             = 3733,
    /// FLAT-OBJECT-REF
    FlatObjectRef                                                          = 710,
    /// FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING
    FlexrayAbsolutelyScheduledTiming                                       = 5712,
    /// FLEXRAY-AR-TP-CHANNEL
    FlexrayArTpChannel                                                     = 3120,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 5953,
    /// FLEXRAY-AR-TP-CONNECTION
    FlexrayArTpConnection                                                  = 5031,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 1130,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 6165,
    /// FLEXRAY-CLUSTER-CONDITIONAL
    FlexrayClusterConditional                                              = 3288,
    /// FLEXRAY-CLUSTER-VARIANTS
    FlexrayClusterVariants                                                 = 2660,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 6075,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 2740,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL
    FlexrayCommunicationControllerConditional                              = 2928,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS
    FlexrayCommunicationControllerVariants                                 = 1021,
    /// FLEXRAY-FIFO-CONFIGURATION
    FlexrayFifoConfiguration                                               = 3020,
    /// FLEXRAY-FIFO-RANGE
    FlexrayFifoRange                                                       = 5921,
    /// FLEXRAY-FIFOS
    FlexrayFifos                                                           = 1068,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 2838,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 5217,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 678,
    /// FLEXRAY-NM-CLUSTER-COUPLING
    FlexrayNmClusterCoupling                                               = 5982,
    /// FLEXRAY-NM-ECU
    FlexrayNmEcu                                                           = 4572,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 2919,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 951,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 1495,
    /// FLEXRAY-TP-CONNECTION
    FlexrayTpConnection                                                    = 6448,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 6234,
    /// FLEXRAY-TP-ECU
    FlexrayTpEcu                                                           = 862,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 2738,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 5638,
    /// FLOAT
    Float                                                                  = 4695,
    /// FLOAT-VALUE-VARIATION-POINT
    FloatValueVariationPoint                                               = 629,
    /// FLOW-CONTROL-PDU-REF
    FlowControlPduRef                                                      = 3366,
    /// FLOW-CONTROL-PDU-REFS
    FlowControlPduRefs                                                     = 823,
    /// FLOW-CONTROL-REF
    FlowControlRef                                                         = 5633,
    /// FLOW-LABEL
    FlowLabel                                                              = 5333,
    /// FLOW-METERING-REF
    FlowMeteringRef                                                        = 5004,
    /// FLOW-METERINGS
    FlowMeterings                                                          = 6125,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 71,
    /// FM-ATTRIBUTE-VALUE
    FmAttributeValue                                                       = 4278,
    /// FM-COND
    FmCond                                                                 = 4209,
    /// FM-FEATURE
    FmFeature                                                              = 1800,
    /// FM-FEATURE-DECOMPOSITION
    FmFeatureDecomposition                                                 = 4428,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 1844,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 643,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 1984,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 3233,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 3178,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 4018,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 3473,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 1846,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 3755,
    /// FM-SYSCOND
    FmSyscond                                                              = 2551,
    /// FMI
    Fmi                                                                    = 91,
    /// FOLLOW-UP-ACTION-REF
    FollowUpActionRef                                                      = 1316,
    /// FOLLOW-UP-ACTION-REFS
    FollowUpActionRefs                                                     = 5785,
    /// FOLLOW-UP-OFFSET
    FollowUpOffset                                                         = 3503,
    /// FOLLOW-UP-TIMEOUT-VALUE
    FollowUpTimeoutValue                                                   = 5191,
    /// FORBIDDEN-SIGNAL-PATH
    ForbiddenSignalPath                                                    = 28,
    /// FOREIGN-MODEL-REFERENCE
    ForeignModelReference                                                  = 2679,
    /// FORMAL-BLUEPRINT-CONDITION
    FormalBlueprintCondition                                               = 3128,
    /// FORMAL-BLUEPRINT-GENERATOR
    FormalBlueprintGenerator                                               = 2486,
    /// FORMAT
    Format                                                                 = 3269,
    /// FORMULA
    Formula                                                                = 4663,
    /// FORMULA-CAPTION
    FormulaCaption                                                         = 62,
    /// FR-CLUSTER-REF
    FrClusterRef                                                           = 2773,
    /// FR-GLOBAL-TIME-DOMAIN-PROPS
    FrGlobalTimeDomainProps                                                = 358,
    /// FRAGMENT
    Fragment                                                               = 3322,
    /// FRAGMENTATION-PROPS
    FragmentationProps                                                     = 6164,
    /// FRAME-LENGTH
    FrameLength                                                            = 2691,
    /// FRAME-MAPPING
    FrameMapping                                                           = 2808,
    /// FRAME-MAPPINGS
    FrameMappings                                                          = 3962,
    /// FRAME-PID
    FramePid                                                               = 4729,
    /// FRAME-PIDS
    FramePids                                                              = 4585,
    /// FRAME-PORT
    FramePort                                                              = 5610,
    /// FRAME-PORT-REF
    FramePortRef                                                           = 5364,
    /// FRAME-PORT-REFS
    FramePortRefs                                                          = 944,
    /// FRAME-PREEMPTION-SUPPORT
    FramePreemptionSupport                                                 = 4648,
    /// FRAME-REF
    FrameRef                                                               = 2529,
    /// FRAME-SYNC-ENABLED
    FrameSyncEnabled                                                       = 5905,
    /// FRAME-TRIGGERING-REF
    FrameTriggeringRef                                                     = 3193,
    /// FRAME-TRIGGERINGS
    FrameTriggerings                                                       = 2615,
    /// FREE-FORMAT
    FreeFormat                                                             = 4545,
    /// FREEZE-CURRENT-STATE
    FreezeCurrentState                                                     = 1351,
    /// FREEZE-CURRENT-STATE-SUPPORTED
    FreezeCurrentStateSupported                                            = 4436,
    /// FREEZE-FRAME-CONTENT-REF
    FreezeFrameContentRef                                                  = 2569,
    /// FREEZE-FRAME-CONTENT-WWH-OBD-REF
    FreezeFrameContentWwhObdRef                                            = 73,
    /// FREEZE-FRAME-REF
    FreezeFrameRef                                                         = 1175,
    /// FREEZE-FRAMES
    FreezeFrames                                                           = 593,
    /// FRESHNESS-COUNTER-SYNC-ATTEMPTS
    FreshnessCounterSyncAttempts                                           = 3582,
    /// FRESHNESS-PROPS-REF
    FreshnessPropsRef                                                      = 1620,
    /// FRESHNESS-PROPSS
    FreshnessPropss                                                        = 2984,
    /// FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR
    FreshnessTimestampTimePeriodFactor                                     = 953,
    /// FRESHNESS-VALUE-ID
    FreshnessValueId                                                       = 905,
    /// FRESHNESS-VALUE-LENGTH
    FreshnessValueLength                                                   = 2257,
    /// FRESHNESS-VALUE-TX-LENGTH
    FreshnessValueTxLength                                                 = 4285,
    /// FROM-REF
    FromRef                                                                = 4745,
    /// FT
    Ft                                                                     = 973,
    /// FULL-DUPLEX-ENABLED
    FullDuplexEnabled                                                      = 833,
    /// FULLY-QUALIFIED-DOMAIN-NAME
    FullyQualifiedDomainName                                               = 622,
    /// FUNCTION
    Function                                                               = 6238,
    /// FUNCTION-CLUSTER-AFFILIATION
    FunctionClusterAffiliation                                             = 1913,
    /// FUNCTION-GROUP-IREF
    FunctionGroupIref                                                      = 1635,
    /// FUNCTION-GROUP-IREFS
    FunctionGroupIrefs                                                     = 1388,
    /// FUNCTION-GROUP-MODE-IREF
    FunctionGroupModeIref                                                  = 1533,
    /// FUNCTION-GROUP-MODE-IREFS
    FunctionGroupModeIrefs                                                 = 2482,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 3532,
    /// FUNCTION-GROUP-PHM-STATE-REFERENCE
    FunctionGroupPhmStateReference                                         = 1585,
    /// FUNCTION-GROUP-PORT-MAPPING
    FunctionGroupPortMapping                                               = 914,
    /// FUNCTION-GROUP-REF
    FunctionGroupRef                                                       = 5911,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 4322,
    /// FUNCTION-GROUP-STATE-IREF
    FunctionGroupStateIref                                                 = 524,
    /// FUNCTION-GROUP-STATE-IREFS
    FunctionGroupStateIrefs                                                = 4395,
    /// FUNCTION-GROUP-TYPE-REF
    FunctionGroupTypeRef                                                   = 1398,
    /// FUNCTION-GROUPS
    FunctionGroups                                                         = 2221,
    /// FUNCTION-ID
    FunctionId                                                             = 5552,
    /// FUNCTION-IDENTIFIER-REF
    FunctionIdentifierRef                                                  = 143,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 3509,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 3341,
    /// FUNCTION-INSTANCE
    FunctionInstance                                                       = 2034,
    /// FUNCTION-NAME
    FunctionName                                                           = 4345,
    /// FUNCTION-NAMES
    FunctionNames                                                          = 426,
    /// FUNCTION-POINTER-SIGNATURE-REF
    FunctionPointerSignatureRef                                            = 3363,
    /// FUNCTION-PROTOTYPE-EMITTER
    FunctionPrototypeEmitter                                               = 591,
    /// FUNCTIONAL-CLUSTER-DESIGNS
    FunctionalClusterDesigns                                               = 5417,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-DIAGNOSTIC-EVENT-MAPPING
    FunctionalClusterInteractsWithDiagnosticEventMapping                   = 2231,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING
    FunctionalClusterInteractsWithPersistencyDeploymentMapping             = 5143,
    /// FUNCTIONAL-CLUSTER-REF
    FunctionalClusterRef                                                   = 4847,
    /// FUNCTIONAL-CLUSTER-REFS
    FunctionalClusterRefs                                                  = 4389,
    /// FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING
    FunctionalClusterToSecurityEventDefinitionMapping                      = 1482,
    /// FUNCTIONAL-REQUEST-REF
    FunctionalRequestRef                                                   = 6342,
    /// FUNCTIONAL-REQUEST-REFS
    FunctionalRequestRefs                                                  = 831,
    /// FUNCTIONAL-UNIT
    FunctionalUnit                                                         = 5580,
    /// FUP-DATA-ID-LIST
    FupDataIdList                                                          = 2664,
    /// FUP-DATA-ID-LISTS
    FupDataIdLists                                                         = 1829,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 6192,
    /// GATEWAY
    Gateway                                                                = 5290,
    /// GATEWAYS
    Gateways                                                               = 458,
    /// GENERAL-INACTIVITY-TIME
    GeneralInactivityTime                                                  = 2140,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 1304,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 5282,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 5232,
    /// GENERAL-PURPOSE-TIMER-SERVICE-NEEDS
    GeneralPurposeTimerServiceNeeds                                        = 4412,
    /// GENERATED-ARTIFACTS
    GeneratedArtifacts                                                     = 764,
    /// GENERATOR
    Generator                                                              = 5187,
    /// GENERIC-DIAGNOSTIC-TRANSPORT-INSTANTIATION
    GenericDiagnosticTransportInstantiation                                = 2513,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 4507,
    /// GENERIC-MATH
    GenericMath                                                            = 3490,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 3633,
    /// GENERIC-TP
    GenericTp                                                              = 644,
    /// GENERIC-TP-CONNECTION
    GenericTpConnection                                                    = 5402,
    /// GET
    Get                                                                    = 5709,
    /// GETS
    Gets                                                                   = 2972,
    /// GETTER-CALL-REF
    GetterCallRef                                                          = 3004,
    /// GETTER-CALL-SIGNAL-REF
    GetterCallSignalRef                                                    = 1260,
    /// GETTER-OPERATION-REF
    GetterOperationRef                                                     = 208,
    /// GETTER-REF
    GetterRef                                                              = 2298,
    /// GETTER-RETURN-REF
    GetterReturnRef                                                        = 4661,
    /// GETTER-RETURN-SIGNAL-REF
    GetterReturnSignalRef                                                  = 1200,
    /// GID
    Gid                                                                    = 4335,
    /// GID-INVALIDITY-PATTERN
    GidInvalidityPattern                                                   = 5336,
    /// GLOBAL-ELEMENT
    GlobalElement                                                          = 5984,
    /// GLOBAL-ELEMENTS
    GlobalElements                                                         = 308,
    /// GLOBAL-IN-PACKAGE-REF
    GlobalInPackageRef                                                     = 4374,
    /// GLOBAL-IN-PACKAGE-REFS
    GlobalInPackageRefs                                                    = 4966,
    /// GLOBAL-KAY-PROPS-REF
    GlobalKayPropsRef                                                      = 6025,
    /// GLOBAL-RESOURCE-ID
    GlobalResourceId                                                       = 5106,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 1561,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 5595,
    /// GLOBAL-SUPERVISION-ENTITYS
    GlobalSupervisionEntitys                                               = 3585,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 1290,
    /// GLOBAL-SUPERVISIONS
    GlobalSupervisions                                                     = 4813,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 5451,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 4651,
    /// GLOBAL-TIME-CORRECTION-PROPS
    GlobalTimeCorrectionProps                                              = 1892,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 3132,
    /// GLOBAL-TIME-DOMAIN-PROPERTYS
    GlobalTimeDomainPropertys                                              = 5715,
    /// GLOBAL-TIME-DOMAIN-PROPS
    GlobalTimeDomainProps                                                  = 6249,
    /// GLOBAL-TIME-DOMAIN-REF
    GlobalTimeDomainRef                                                    = 6174,
    /// GLOBAL-TIME-DOMAIN-REF-CONDITIONAL
    GlobalTimeDomainRefConditional                                         = 1466,
    /// GLOBAL-TIME-DOMAINS
    GlobalTimeDomains                                                      = 1962,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 4591,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 3336,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 3670,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 5877,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 1679,
    /// GLOBAL-TIME-MASTERS
    GlobalTimeMasters                                                      = 941,
    /// GLOBAL-TIME-PDU-REF
    GlobalTimePduRef                                                       = 2845,
    /// GLOBAL-TIME-PDU-TRIGGERING-REF
    GlobalTimePduTriggeringRef                                             = 2822,
    /// GLOBAL-TIME-PORT-ROLE
    GlobalTimePortRole                                                     = 4423,
    /// GLOBAL-TIME-PROPS
    GlobalTimeProps                                                        = 6257,
    /// GLOBAL-TIME-SUB-DOMAINS
    GlobalTimeSubDomains                                                   = 1253,
    /// GLOBAL-TIME-TX-PERIOD
    GlobalTimeTxPeriod                                                     = 4176,
    /// GOVERNANCE-REF
    GovernanceRef                                                          = 3794,
    /// GRANT-REF
    GrantRef                                                               = 5212,
    /// GRANT-REFS
    GrantRefs                                                              = 2380,
    /// GRAPHIC
    Graphic                                                                = 1001,
    /// GROUP-ID
    GroupId                                                                = 5932,
    /// GROUP-IDENTIFIER
    GroupIdentifier                                                        = 1184,
    /// GROUP-NUMBER
    GroupNumber                                                            = 731,
    /// GROUPED-ALIAS-EVENT-REF
    GroupedAliasEventRef                                                   = 988,
    /// GROUPED-ALIAS-EVENT-REFS
    GroupedAliasEventRefs                                                  = 2450,
    /// HANDLE-DATA-STATUS
    HandleDataStatus                                                       = 1718,
    /// HANDLE-INVALID
    HandleInvalid                                                          = 182,
    /// HANDLE-NEVER-RECEIVED
    HandleNeverReceived                                                    = 454,
    /// HANDLE-OUT-OF-RANGE
    HandleOutOfRange                                                       = 3790,
    /// HANDLE-OUT-OF-RANGE-STATUS
    HandleOutOfRangeStatus                                                 = 2807,
    /// HANDLE-TERMINATION-AND-RESTART
    HandleTerminationAndRestart                                            = 2802,
    /// HANDLE-TIMEOUT-TYPE
    HandleTimeoutType                                                      = 1811,
    /// HARDWARE-CONFIGURATION
    HardwareConfiguration                                                  = 1123,
    /// HARDWARE-ELEMENT-REF
    HardwareElementRef                                                     = 85,
    /// HARDWARE-ELEMENT-REFS
    HardwareElementRefs                                                    = 2126,
    /// HARDWARE-PLATFORM
    HardwarePlatform                                                       = 1371,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 4701,
    /// HAS-DYNAMIC-LENGTH
    HasDynamicLength                                                       = 791,
    /// HAS-GETTER
    HasGetter                                                              = 3952,
    /// HAS-INTERNAL-STATE
    HasInternalState                                                       = 6398,
    /// HAS-NOTIFIER
    HasNotifier                                                            = 6228,
    /// HAS-SETTER
    HasSetter                                                              = 6455,
    /// HEADER-FILE
    HeaderFile                                                             = 4004,
    /// HEADER-ID
    HeaderId                                                               = 6074,
    /// HEADER-ID-LONG-HEADER
    HeaderIdLongHeader                                                     = 5955,
    /// HEADER-ID-SHORT-HEADER
    HeaderIdShortHeader                                                    = 5705,
    /// HEADER-LENGTH
    HeaderLength                                                           = 3637,
    /// HEADER-TYPE
    HeaderType                                                             = 5480,
    /// HEALING-CYCLE-COUNTER-THRESHOLD
    HealingCycleCounterThreshold                                           = 743,
    /// HEALING-CYCLE-REF
    HealingCycleRef                                                        = 4078,
    /// HEALTH-CHANNEL-ARGUMENT-REF
    HealthChannelArgumentRef                                               = 5185,
    /// HEALTH-CHANNEL-ARGUMENT-REFS
    HealthChannelArgumentRefs                                              = 6306,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 6224,
    /// HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS
    HealthChannelExternalReportedStatus                                    = 5798,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 5827,
    /// HEALTH-CHANNEL-ID
    HealthChannelId                                                        = 4721,
    /// HEALTH-CHANNEL-IREF
    HealthChannelIref                                                      = 867,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 4640,
    /// HEALTH-CHANNELS
    HealthChannels                                                         = 3220,
    /// HEAP-USAGES
    HeapUsages                                                             = 3462,
    /// HISTORY
    History                                                                = 2199,
    /// HISTORY-KIND
    HistoryKind                                                            = 5009,
    /// HISTORY-ORDER-DEPTH
    HistoryOrderDepth                                                      = 6066,
    /// HOLD-OVER-TIME
    HoldOverTime                                                           = 1349,
    /// HOP-COUNT
    HopCount                                                               = 3339,
    /// HOP-LIMIT
    HopLimit                                                               = 4555,
    /// HOST-ID
    HostId                                                                 = 4678,
    /// HOST-REF
    HostRef                                                                = 144,
    /// HOST-SYSTEM-REF
    HostSystemRef                                                          = 6346,
    /// HTTP-ACCEPT-ENCODING
    HttpAcceptEncoding                                                     = 4863,
    /// HTTP-TP
    HttpTp                                                                 = 3389,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 6160,
    /// HW-ATTRIBUTE-DEF-REF
    HwAttributeDefRef                                                      = 3438,
    /// HW-ATTRIBUTE-DEFS
    HwAttributeDefs                                                        = 800,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 740,
    /// HW-ATTRIBUTE-LITERALS
    HwAttributeLiterals                                                    = 1131,
    /// HW-ATTRIBUTE-VALUE
    HwAttributeValue                                                       = 1581,
    /// HW-ATTRIBUTE-VALUES
    HwAttributeValues                                                      = 2836,
    /// HW-CATEGORY
    HwCategory                                                             = 121,
    /// HW-CATEGORY-REF
    HwCategoryRef                                                          = 1171,
    /// HW-CATEGORY-REFS
    HwCategoryRefs                                                         = 1207,
    /// HW-COMMUNICATION-CONTROLLER-REF
    HwCommunicationControllerRef                                           = 4814,
    /// HW-COMMUNICATION-PORT-REF
    HwCommunicationPortRef                                                 = 4984,
    /// HW-ELEMENT
    HwElement                                                              = 48,
    /// HW-ELEMENT-CONNECTIONS
    HwElementConnections                                                   = 1639,
    /// HW-ELEMENT-CONNECTOR
    HwElementConnector                                                     = 499,
    /// HW-ELEMENT-REF
    HwElementRef                                                           = 1596,
    /// HW-ELEMENT-REF-CONDITIONAL
    HwElementRefConditional                                                = 1642,
    /// HW-ELEMENT-REFS
    HwElementRefs                                                          = 3993,
    /// HW-PIN
    HwPin                                                                  = 5426,
    /// HW-PIN-CONNECTIONS
    HwPinConnections                                                       = 2218,
    /// HW-PIN-CONNECTOR
    HwPinConnector                                                         = 569,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 1865,
    /// HW-PIN-GROUP-CONNECTIONS
    HwPinGroupConnections                                                  = 3459,
    /// HW-PIN-GROUP-CONNECTOR
    HwPinGroupConnector                                                    = 504,
    /// HW-PIN-GROUP-CONTENT
    HwPinGroupContent                                                      = 631,
    /// HW-PIN-GROUP-REF
    HwPinGroupRef                                                          = 1094,
    /// HW-PIN-GROUP-REFS
    HwPinGroupRefs                                                         = 3415,
    /// HW-PIN-GROUPS
    HwPinGroups                                                            = 5919,
    /// HW-PIN-REF
    HwPinRef                                                               = 6072,
    /// HW-PIN-REFS
    HwPinRefs                                                              = 3097,
    /// HW-PORT-MAPPING
    HwPortMapping                                                          = 3408,
    /// HW-PORT-MAPPINGS
    HwPortMappings                                                         = 2013,
    /// HW-TYPE
    HwType                                                                 = 689,
    /// HW-TYPE-REF
    HwTypeRef                                                              = 4958,
    /// I-PDU-IDENTIFIER-TCP-REF
    IPduIdentifierTcpRef                                                   = 0,
    /// I-PDU-IDENTIFIER-TCP-REFS
    IPduIdentifierTcpRefs                                                  = 1557,
    /// I-PDU-IDENTIFIER-UDP-REF
    IPduIdentifierUdpRef                                                   = 5358,
    /// I-PDU-IDENTIFIER-UDP-REFS
    IPduIdentifierUdpRefs                                                  = 835,
    /// I-PDU-IDENTIFIERS
    IPduIdentifiers                                                        = 3858,
    /// I-PDU-MAPPING
    IPduMapping                                                            = 2356,
    /// I-PDU-MAPPINGS
    IPduMappings                                                           = 2620,
    /// I-PDU-PORT
    IPduPort                                                               = 3326,
    /// I-PDU-PORT-REF
    IPduPortRef                                                            = 1675,
    /// I-PDU-PORT-REFS
    IPduPortRefs                                                           = 178,
    /// I-PDU-REF
    IPduRef                                                                = 5683,
    /// I-PDU-SIGNAL-PROCESSING
    IPduSignalProcessing                                                   = 2162,
    /// I-PDU-TIMING
    IPduTiming                                                             = 6250,
    /// I-PDU-TIMING-SPECIFICATIONS
    IPduTimingSpecifications                                               = 1065,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 191,
    /// I-PDU-TRIGGERING-REF
    IPduTriggeringRef                                                      = 4732,
    /// I-PDU-TRIGGERING-REF-CONDITIONAL
    IPduTriggeringRefConditional                                           = 1570,
    /// I-PDU-TRIGGERINGS
    IPduTriggerings                                                        = 5627,
    /// I-PDUS
    IPdus                                                                  = 5550,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 2869,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 3138,
    /// I-SIGNAL
    ISignal                                                                = 3782,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 2696,
    /// I-SIGNAL-GROUP-REF
    ISignalGroupRef                                                        = 520,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 5381,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 5337,
    /// I-SIGNAL-I-PDU-REF
    ISignalIPduRef                                                         = 3768,
    /// I-SIGNAL-I-PDU-REF-CONDITIONAL
    ISignalIPduRefConditional                                              = 6157,
    /// I-SIGNAL-I-PDUS
    ISignalIPdus                                                           = 3237,
    /// I-SIGNAL-IN-I-PDU-REF
    ISignalInIPduRef                                                       = 376,
    /// I-SIGNAL-MAPPING
    ISignalMapping                                                         = 3715,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 5416,
    /// I-SIGNAL-PORT-REF
    ISignalPortRef                                                         = 5731,
    /// I-SIGNAL-PORT-REFS
    ISignalPortRefs                                                        = 2781,
    /// I-SIGNAL-PROPS
    ISignalProps                                                           = 2098,
    /// I-SIGNAL-REF
    ISignalRef                                                             = 4379,
    /// I-SIGNAL-REFS
    ISignalRefs                                                            = 3391,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 2442,
    /// I-SIGNAL-TO-I-PDU-MAPPINGS
    ISignalToIPduMappings                                                  = 5063,
    /// I-SIGNAL-TO-PDU-MAPPINGS
    ISignalToPduMappings                                                   = 2926,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 5059,
    /// I-SIGNAL-TRIGGERING-REF
    ISignalTriggeringRef                                                   = 746,
    /// I-SIGNAL-TRIGGERING-REF-CONDITIONAL
    ISignalTriggeringRefConditional                                        = 6439,
    /// I-SIGNAL-TRIGGERINGS
    ISignalTriggerings                                                     = 2865,
    /// I-SIGNAL-TYPE
    ISignalType                                                            = 4033,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 4382,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF
    IamRelevantTlsSecureComPropsRef                                        = 2989,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS
    IamRelevantTlsSecureComPropsRefs                                       = 2872,
    /// ICMP-RULE
    IcmpRule                                                               = 3559,
    /// ICMP-V-4-PROPS
    IcmpV4Props                                                            = 5991,
    /// ICMP-V-6-PROPS
    IcmpV6Props                                                            = 5437,
    /// ICV-FRESHNESS-VALUE-ID
    IcvFreshnessValueId                                                    = 2741,
    /// ICV-SECURE-COM-PROPSS
    IcvSecureComPropss                                                     = 5826,
    /// ICV-SECURED
    IcvSecured                                                             = 1560,
    /// ICV-VERIFICATION
    IcvVerification                                                        = 2117,
    /// ID
    Id                                                                     = 2524,
    /// IDENT
    Ident                                                                  = 2930,
    /// IDENTICAL-MAPPING
    IdenticalMapping                                                       = 81,
    /// IDENTIFIABLE-REF
    IdentifiableRef                                                        = 4978,
    /// IDENTIFIER
    Identifier                                                             = 6009,
    /// IDENTITIY-NUMBER
    IdentitiyNumber                                                        = 5133,
    /// IDENTITY
    Identity                                                               = 1008,
    /// IDENTITY-CERTIFICATE-AUTHORITY-REF
    IdentityCertificateAuthorityRef                                        = 3766,
    /// IDENTITY-REF
    IdentityRef                                                            = 2883,
    /// IDLE-SLOPE
    IdleSlope                                                              = 1900,
    /// IDS-COMMON-ELEMENT-REF
    IdsCommonElementRef                                                    = 2198,
    /// IDS-COMMON-ELEMENT-REF-CONDITIONAL
    IdsCommonElementRefConditional                                         = 2547,
    /// IDS-DESIGN
    IdsDesign                                                              = 2463,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 2084,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 3891,
    /// IDS-PLATFORM-INSTANTIATION-REF
    IdsPlatformInstantiationRef                                            = 609,
    /// IDS-TIME-BASE-REF
    IdsTimeBaseRef                                                         = 2843,
    /// IDS-TIME-BASE-REFS
    IdsTimeBaseRefs                                                        = 70,
    /// IDSM-CONTEXT-PROVIDER-INTERFACE
    IdsmContextProviderInterface                                           = 5753,
    /// IDSM-CONTEXT-PROVIDER-MAPPING
    IdsmContextProviderMapping                                             = 1286,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 957,
    /// IDSM-INSTANCE-ID
    IdsmInstanceId                                                         = 3135,
    /// IDSM-INSTANCE-REF
    IdsmInstanceRef                                                        = 2281,
    /// IDSM-INSTANCE-REF-CONDITIONAL
    IdsmInstanceRefConditional                                             = 926,
    /// IDSM-INSTANCES
    IdsmInstances                                                          = 269,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 3043,
    /// IDSM-MODULE-INSTANTIATION-REF
    IdsmModuleInstantiationRef                                             = 5732,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 4434,
    /// IDSM-QUALIFIED-EVENT-RECEIVER-INTERFACE
    IdsmQualifiedEventReceiverInterface                                    = 1250,
    /// IDSM-QUALIFIED-EVENT-RECEIVER-MAPPING
    IdsmQualifiedEventReceiverMapping                                      = 1367,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 2534,
    /// IDSM-RATE-LIMITATION-REF
    IdsmRateLimitationRef                                                  = 2638,
    /// IDSM-RATE-LIMITATION-REF-CONDITIONAL
    IdsmRateLimitationRefConditional                                       = 6416,
    /// IDSM-REPORTING-MODE-PROVIDER-INTERFACE
    IdsmReportingModeProviderInterface                                     = 1566,
    /// IDSM-REPORTING-MODE-PROVIDER-MAPPING
    IdsmReportingModeProviderMapping                                       = 749,
    /// IDSM-TIMESTAMP-PROVIDER-INTERFACE
    IdsmTimestampProviderInterface                                         = 1591,
    /// IDSM-TIMESTAMP-PROVIDER-MAPPING
    IdsmTimestampProviderMapping                                           = 4096,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 4463,
    /// IDSM-TRAFFIC-LIMITATION-REF
    IdsmTrafficLimitationRef                                               = 4373,
    /// IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL
    IdsmTrafficLimitationRefConditional                                    = 5430,
    /// IE
    Ie                                                                     = 4583,
    /// IEE-E-1722-TP-CONNECTION-REF
    IeeE1722TpConnectionRef                                                = 4344,
    /// IEEE-1722-ACF-BUS-PART-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722AcfBusPartRawDataStreamConsumerMapping                         = 6406,
    /// IEEE-1722-ACF-BUS-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722AcfBusRawDataStreamConsumerMapping                             = 420,
    /// IEEE-1722-RAW-DATA-STREAM-CONSUMER-INTERFACE
    Ieee1722RawDataStreamConsumerInterface                                 = 2377,
    /// IEEE-1722-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722RawDataStreamConsumerMapping                                   = 5873,
    /// IEEE-1722-RAW-DATA-STREAM-PRODUCER-INTERFACE
    Ieee1722RawDataStreamProducerInterface                                 = 1210,
    /// IEEE-1722-RAW-DATA-STREAM-PRODUCER-MAPPING
    Ieee1722RawDataStreamProducerMapping                                   = 4603,
    /// IEEE-1722-STREAM-REF
    Ieee1722StreamRef                                                      = 5624,
    /// IEEE-1722-TP
    Ieee1722Tp                                                             = 3364,
    /// IEEE-1722-TP-AAF-CONNECTION
    Ieee1722TpAafConnection                                                = 3992,
    /// IEEE-1722-TP-ACF-CAN
    Ieee1722TpAcfCan                                                       = 1825,
    /// IEEE-1722-TP-ACF-CAN-PART
    Ieee1722TpAcfCanPart                                                   = 5825,
    /// IEEE-1722-TP-ACF-CONNECTION
    Ieee1722TpAcfConnection                                                = 3393,
    /// IEEE-1722-TP-ACF-LIN
    Ieee1722TpAcfLin                                                       = 1631,
    /// IEEE-1722-TP-ACF-LIN-PART
    Ieee1722TpAcfLinPart                                                   = 2139,
    /// IEEE-1722-TP-CONFIG
    Ieee1722TpConfig                                                       = 5819,
    /// IEEE-1722-TP-CONNECTION-REF-CONDITIONAL
    Ieee1722TpConnectionRefConditional                                     = 2764,
    /// IEEE-1722-TP-CRF-CONNECTION
    Ieee1722TpCrfConnection                                                = 6222,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 695,
    /// IEEE-1722-TP-IIDC-CONNECTION
    Ieee1722TpIidcConnection                                               = 4710,
    /// IEEE-1722-TP-RULE
    Ieee1722TpRule                                                         = 2491,
    /// IEEE-1722-TP-RVF-CONNECTION
    Ieee1722TpRvfConnection                                                = 1894,
    /// IGNORE-AFTER-TX
    IgnoreAfterTx                                                          = 328,
    /// IGNORE-ORDER-ALLOWED
    IgnoreOrderAllowed                                                     = 5774,
    /// IIDC-CHANNEL
    IidcChannel                                                            = 711,
    /// IIDC-DATA-BLOCK-SIZE
    IidcDataBlockSize                                                      = 6211,
    /// IIDC-FRACTION-NUMBER
    IidcFractionNumber                                                     = 1860,
    /// IIDC-SOURCE-PACKET-HEADER
    IidcSourcePacketHeader                                                 = 5551,
    /// IIDC-STREAM-FORMAT
    IidcStreamFormat                                                       = 3738,
    /// IIDC-SY
    IidcSy                                                                 = 2881,
    /// IIDC-T-CODE
    IidcTCode                                                              = 4163,
    /// IIDC-TAG
    IidcTag                                                                = 4174,
    /// IKE-AUTHENTICATION-METHOD
    IkeAuthenticationMethod                                                = 1306,
    /// IKE-CIPHER-SUITE-NAME
    IkeCipherSuiteName                                                     = 5424,
    /// IKE-OVER-TIME
    IkeOverTime                                                            = 5767,
    /// IKE-RAND-TIME
    IkeRandTime                                                            = 4846,
    /// IKE-REAUTH-TIME
    IkeReauthTime                                                          = 1552,
    /// IKE-REKEY-TIME
    IkeRekeyTime                                                           = 2813,
    /// IMMEDIATE-NV-DATA-STORAGE
    ImmediateNvDataStorage                                                 = 657,
    /// IMMEDIATE-RESUME-TIME
    ImmediateResumeTime                                                    = 2848,
    /// IMPL-CONSTANT-REF
    ImplConstantRef                                                        = 3422,
    /// IMPL-INIT-VALUE
    ImplInitValue                                                          = 4360,
    /// IMPLEMENTATION-ARRAY-ELEMENT-REF
    ImplementationArrayElementRef                                          = 3105,
    /// IMPLEMENTATION-CONFIG-CLASSES
    ImplementationConfigClasses                                            = 1224,
    /// IMPLEMENTATION-CONFIG-VARIANT
    ImplementationConfigVariant                                            = 4046,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 4359,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 3816,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 3445,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF
    ImplementationDataTypeElementInAutosarDataPrototypeRef                 = 5516,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF
    ImplementationDataTypeElementInPortInterfaceRef                        = 6149,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF
    ImplementationDataTypeElementInSystemRef                               = 5298,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    ImplementationDataTypeElementRef                                       = 5296,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 3685,
    /// IMPLEMENTATION-DATA-TYPE-REF
    ImplementationDataTypeRef                                              = 3,
    /// IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF
    ImplementationDataTypeSubElementRef                                    = 2695,
    /// IMPLEMENTATION-PROPSS
    ImplementationPropss                                                   = 2746,
    /// IMPLEMENTATION-RECORD-ELEMENT-REF
    ImplementationRecordElementRef                                         = 1356,
    /// IMPLEMENTATION-REF
    ImplementationRef                                                      = 2775,
    /// IMPLEMENTATION-REFS
    ImplementationRefs                                                     = 360,
    /// IMPLEMENTED-ENTRY-REF
    ImplementedEntryRef                                                    = 5946,
    /// IMPLEMENTED-ENTRYS
    ImplementedEntrys                                                      = 5581,
    /// IMPLEMENTED-IN-REF
    ImplementedInRef                                                       = 2549,
    /// IMPLEMENTS-LEGACY-STRING-SERIALIZATION
    ImplementsLegacyStringSerialization                                    = 1271,
    /// IMPLEMENTS-SOMEIP-STRING-HANDLING
    ImplementsSomeipStringHandling                                         = 2855,
    /// IMPLICIT-DATA-ACCESS-IREF
    ImplicitDataAccessIref                                                 = 5304,
    /// IMPLICIT-DATA-ACCESS-IREFS
    ImplicitDataAccessIrefs                                                = 2313,
    /// IMPLICIT-INTER-RUNNABLE-VARIABLES
    ImplicitInterRunnableVariables                                         = 1479,
    /// IMPLIED-OPERATION-REF
    ImpliedOperationRef                                                    = 4032,
    /// IMPLIED-OPERATION-REFS
    ImpliedOperationRefs                                                   = 4479,
    /// IMPORTANCE
    Importance                                                             = 5389,
    /// IMPOSITION-TIME
    ImpositionTime                                                         = 5582,
    /// IMPOSITION-TIME-DEFINITION-GROUP
    ImpositionTimeDefinitionGroup                                          = 336,
    /// IMPOSITION-TIMES
    ImpositionTimes                                                        = 703,
    /// IN-MEASUREMENT-SET
    InMeasurementSet                                                       = 3761,
    /// IN-PLACE
    InPlace                                                                = 3563,
    /// IN-SCOPE
    InScope                                                                = 5928,
    /// INCLUDE-REF
    IncludeRef                                                             = 1146,
    /// INCLUDE-REFS
    IncludeRefs                                                            = 3769,
    /// INCLUDED-DATA-TYPE-SET
    IncludedDataTypeSet                                                    = 4441,
    /// INCLUDED-DATA-TYPE-SETS
    IncludedDataTypeSets                                                   = 1959,
    /// INCLUDED-LIBRARY-REF
    IncludedLibraryRef                                                     = 1496,
    /// INCLUDED-LIBRARY-REFS
    IncludedLibraryRefs                                                    = 6321,
    /// INCLUDED-MODE-DECLARATION-GROUP-SET
    IncludedModeDeclarationGroupSet                                        = 6133,
    /// INCLUDED-MODE-DECLARATION-GROUP-SETS
    IncludedModeDeclarationGroupSets                                       = 2063,
    /// INCLUDED-VARIANT-REF
    IncludedVariantRef                                                     = 2970,
    /// INCLUDED-VARIANT-REFS
    IncludedVariantRefs                                                    = 3841,
    /// INDENT-SAMPLE
    IndentSample                                                           = 964,
    /// INDEX
    Index                                                                  = 3516,
    /// INDEX-DATA-TYPE-REF
    IndexDataTypeRef                                                       = 1717,
    /// INDEXED-ARRAY-ELEMENT
    IndexedArrayElement                                                    = 2897,
    /// INDICATIONS
    Indications                                                            = 6359,
    /// INDICATOR-FAILURE-CYCLE-COUNTER-THRESHOLD
    IndicatorFailureCycleCounterThreshold                                  = 1251,
    /// INDICATOR-REF
    IndicatorRef                                                           = 1683,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 2991,
    /// INDIRECT-API
    IndirectApi                                                            = 603,
    /// INDUSTRY-GROUP
    IndustryGroup                                                          = 2694,
    /// INFO-TYPE
    InfoType                                                               = 5891,
    /// INFO-TYPE-REF
    InfoTypeRef                                                            = 2797,
    /// INFORMAL-FORMULA
    InformalFormula                                                        = 3486,
    /// INFRASTRUCTURE-SERVICES
    InfrastructureServices                                                 = 1140,
    /// INGRESS-PORT-REF
    IngressPortRef                                                         = 2512,
    /// INGRESS-PORT-REFS
    IngressPortRefs                                                        = 5965,
    /// INGRESS-PRIORITY
    IngressPriority                                                        = 2106,
    /// INGRESS-VLAN-ID
    IngressVlanId                                                          = 3576,
    /// INHIBIT-SOURCES
    InhibitSources                                                         = 2632,
    /// INHIBITING-FID-REF
    InhibitingFidRef                                                       = 2611,
    /// INHIBITING-SECONDARY-FID-REF
    InhibitingSecondaryFidRef                                              = 4650,
    /// INHIBITING-SECONDARY-FID-REFS
    InhibitingSecondaryFidRefs                                             = 798,
    /// INHIBITION-MASK
    InhibitionMask                                                         = 142,
    /// INHIBITION-SOURCE-REF
    InhibitionSourceRef                                                    = 5579,
    /// INIT-EVENT
    InitEvent                                                              = 2871,
    /// INIT-RESOURCE
    InitResource                                                           = 5288,
    /// INIT-VALUE
    InitValue                                                              = 1017,
    /// INIT-VALUES
    InitValues                                                             = 6144,
    /// INITIAL-CHECKPOINT-REF
    InitialCheckpointRef                                                   = 205,
    /// INITIAL-CHECKPOINT-REFS
    InitialCheckpointRefs                                                  = 4873,
    /// INITIAL-DELAY-MAX-VALUE
    InitialDelayMaxValue                                                   = 3899,
    /// INITIAL-DELAY-MIN-VALUE
    InitialDelayMinValue                                                   = 4424,
    /// INITIAL-DYNAMIC-PART
    InitialDynamicPart                                                     = 3818,
    /// INITIAL-EVENT-STATUS
    InitialEventStatus                                                     = 3943,
    /// INITIAL-FIND-BEHAVIOR
    InitialFindBehavior                                                    = 3978,
    /// INITIAL-INACTIVITY-TIME
    InitialInactivityTime                                                  = 4470,
    /// INITIAL-MODE-REF
    InitialModeRef                                                         = 1593,
    /// INITIAL-NAD
    InitialNad                                                             = 4808,
    /// INITIAL-OFFER-BEHAVIOR
    InitialOfferBehavior                                                   = 5491,
    /// INITIAL-REF-OFFSET
    InitialRefOffset                                                       = 2062,
    /// INITIAL-REPETITIONS-BASE-DELAY
    InitialRepetitionsBaseDelay                                            = 5617,
    /// INITIAL-REPETITIONS-MAX
    InitialRepetitionsMax                                                  = 1454,
    /// INITIAL-STATUS
    InitialStatus                                                          = 934,
    /// INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    InitialVehicleAnnouncementTime                                         = 1735,
    /// INITIALIZATION-VECTOR-LENGTH
    InitializationVectorLength                                             = 952,
    /// INITIALIZED-PARAMETER-REF
    InitializedParameterRef                                                = 889,
    /// INNER-GROUP-IREF
    InnerGroupIref                                                         = 3533,
    /// INNER-GROUP-IREFS
    InnerGroupIrefs                                                        = 1746,
    /// INNER-PORT-IREF
    InnerPortIref                                                          = 5449,
    /// INPLACE
    Inplace                                                                = 913,
    /// INPUT
    Input                                                                  = 1448,
    /// INPUT-DATAS
    InputDatas                                                             = 3500,
    /// INPUT-VARIABLE-TYPE-REF
    InputVariableTypeRef                                                   = 987,
    /// INSTALLATION-BEHAVIOR
    InstallationBehavior                                                   = 4340,
    /// INSTANCE-ID
    InstanceId                                                             = 916,
    /// INSTANCE-ID-PROPS
    InstanceIdProps                                                        = 5382,
    /// INSTANCE-IDENTIFIER
    InstanceIdentifier                                                     = 2204,
    /// INSTANCE-IDS
    InstanceIds                                                            = 6147,
    /// INSTANCE-IN-MEMORY
    InstanceInMemory                                                       = 1198,
    /// INSTANCES
    Instances                                                              = 3526,
    /// INSTANTIATION-DATA-DEF-PROPS
    InstantiationDataDefProps                                              = 1127,
    /// INSTANTIATION-DATA-DEF-PROPSS
    InstantiationDataDefPropss                                             = 3191,
    /// INSTANTIATION-RTE-EVENT-PROPSS
    InstantiationRteEventPropss                                            = 288,
    /// INSTANTIATION-TIMING-EVENT-PROPS
    InstantiationTimingEventProps                                          = 3001,
    /// INTEGER-VALUE-VARIATION-POINT
    IntegerValueVariationPoint                                             = 5174,
    /// INTENDED-FILENAME
    IntendedFilename                                                       = 5124,
    /// INTENDED-PARTIAL-INITIALIZATION-COUNT
    IntendedPartialInitializationCount                                     = 1404,
    /// INTENDED-TARGET-MACHINE-REF
    IntendedTargetMachineRef                                               = 1568,
    /// INTENDED-TARGET-MACHINE-REFS
    IntendedTargetMachineRefs                                              = 6392,
    /// INTER-MESSAGE-TIME
    InterMessageTime                                                       = 2160,
    /// INTERFACE-BLUEPRINT-MAPPING
    InterfaceBlueprintMapping                                              = 6189,
    /// INTERFACE-BLUEPRINT-MAPPINGS
    InterfaceBlueprintMappings                                             = 2831,
    /// INTERFACE-BLUEPRINT-REF
    InterfaceBlueprintRef                                                  = 5768,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 6436,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 3627,
    /// INTERFACE-MAPPINGS
    InterfaceMappings                                                      = 5164,
    /// INTERFACE-REF
    InterfaceRef                                                           = 1076,
    /// INTERFACE-VERSION
    InterfaceVersion                                                       = 5772,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 5668,
    /// INTERNAL-BEHAVIORS
    InternalBehaviors                                                      = 2097,
    /// INTERNAL-CONSTRS
    InternalConstrs                                                        = 4469,
    /// INTERNAL-PRIORITY-VALUE
    InternalPriorityValue                                                  = 3284,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 1128,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 2211,
    /// INTERNAL-TRIGGERING-POINT-POLICYS
    InternalTriggeringPointPolicys                                         = 4464,
    /// INTERNAL-TRIGGERING-POINTS
    InternalTriggeringPoints                                               = 2283,
    /// INTERNET-HEADER-LENGTH
    InternetHeaderLength                                                   = 2645,
    /// INTERPOLATION-ROUTINE
    InterpolationRoutine                                                   = 747,
    /// INTERPOLATION-ROUTINE-MAPPING
    InterpolationRoutineMapping                                            = 2873,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 942,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REF
    InterpolationRoutineMappingSetRef                                      = 3188,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REFS
    InterpolationRoutineMappingSetRefs                                     = 489,
    /// INTERPOLATION-ROUTINE-MAPPINGS
    InterpolationRoutineMappings                                           = 2518,
    /// INTERPOLATION-ROUTINE-REF
    InterpolationRoutineRef                                                = 6153,
    /// INTERPOLATION-ROUTINES
    InterpolationRoutines                                                  = 647,
    /// INTERRUPT-CATEGORY
    InterruptCategory                                                      = 494,
    /// INTERRUPT-SOURCE
    InterruptSource                                                        = 5604,
    /// INTERVAL-LENGTH
    IntervalLength                                                         = 6279,
    /// INTRODUCTION
    Introduction                                                           = 4392,
    /// INVALID-VALUE
    InvalidValue                                                           = 4982,
    /// INVALIDATION-POLICY
    InvalidationPolicy                                                     = 6457,
    /// INVALIDATION-POLICYS
    InvalidationPolicys                                                    = 3187,
    /// INVERSE-PROTOCOL-VERSION
    InverseProtocolVersion                                                 = 2409,
    /// INVERT
    Invert                                                                 = 5342,
    /// INVERT-CONDITION
    InvertCondition                                                        = 1318,
    /// INVOCATION
    Invocation                                                             = 716,
    /// IO-CONTROL-CLASS-REF
    IoControlClassRef                                                      = 1332,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATION
    IoHwAbstractionServerAnnotation                                        = 33,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATIONS
    IoHwAbstractionServerAnnotations                                       = 1887,
    /// IP-ADDRESS
    IpAddress                                                              = 2316,
    /// IP-ADDRESS-KEEP-BEHAVIOR
    IpAddressKeepBehavior                                                  = 1393,
    /// IP-ADDRESS-PREFIX-LENGTH
    IpAddressPrefixLength                                                  = 4387,
    /// IP-IAM-AUTHENTIC-CONNECTION-PROPS
    IpIamAuthenticConnectionProps                                          = 6438,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 2213,
    /// IP-PROTOCOL
    IpProtocol                                                             = 2046,
    /// IP-SEC-CONFIG
    IpSecConfig                                                            = 5853,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 1632,
    /// IP-SEC-CONFIG-PROPS-REF
    IpSecConfigPropsRef                                                    = 1967,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 5328,
    /// IP-SEC-RULE
    IpSecRule                                                              = 3150,
    /// IP-SEC-RULES
    IpSecRules                                                             = 1063,
    /// IP-TP-RULE
    IpTpRule                                                               = 2308,
    /// IP-V-4-ADDRESS
    IpV4Address                                                            = 1790,
    /// IP-V-6-ADDRESS
    IpV6Address                                                            = 6093,
    /// IP-V-6-PATH-MTU-ENABLED
    IpV6PathMtuEnabled                                                     = 1105,
    /// IP-V-6-PATH-MTU-TIMEOUT
    IpV6PathMtuTimeout                                                     = 3574,
    /// IPV-4-ADDRESS
    Ipv4Address                                                            = 1782,
    /// IPV-4-ADDRESS-MASK
    Ipv4AddressMask                                                        = 3278,
    /// IPV-4-ADDRESS-SOURCE
    Ipv4AddressSource                                                      = 2815,
    /// IPV-4-CONFIGURATION
    Ipv4Configuration                                                      = 5066,
    /// IPV-4-DHCP-SERVER-CONFIGURATION
    Ipv4DhcpServerConfiguration                                            = 5010,
    /// IPV-4-MULTICAST-IP-ADDRESS
    Ipv4MulticastIpAddress                                                 = 115,
    /// IPV-4-PROPS
    Ipv4Props                                                              = 4711,
    /// IPV-4-RULE
    Ipv4Rule                                                               = 2454,
    /// IPV-6-ADDRESS
    Ipv6Address                                                            = 5375,
    /// IPV-6-ADDRESS-MASK
    Ipv6AddressMask                                                        = 1629,
    /// IPV-6-ADDRESS-SOURCE
    Ipv6AddressSource                                                      = 90,
    /// IPV-6-CONFIGURATION
    Ipv6Configuration                                                      = 5898,
    /// IPV-6-DHCP-SERVER-CONFIGURATION
    Ipv6DhcpServerConfiguration                                            = 6287,
    /// IPV-6-MULTICAST-IP-ADDRESS
    Ipv6MulticastIpAddress                                                 = 1793,
    /// IPV-6-PROPS
    Ipv6Props                                                              = 2148,
    /// IPV-6-RULE
    Ipv6Rule                                                               = 5525,
    /// IS-ACTIVATION-LINE-DEPENDENT
    IsActivationLineDependent                                              = 5559,
    /// IS-AUTO-VALUE
    IsAutoValue                                                            = 2418,
    /// IS-DEFAULT
    IsDefault                                                              = 722,
    /// IS-DELTA-PACKAGE
    IsDeltaPackage                                                         = 4590,
    /// IS-DYNAMIC-LENGTH-FIELD-SIZE
    IsDynamicLengthFieldSize                                               = 5859,
    /// IS-EVENT
    IsEvent                                                                = 3897,
    /// IS-EXTERNAL
    IsExternal                                                             = 3902,
    /// IS-GLOBAL
    IsGlobal                                                               = 6207,
    /// IS-MANDATORY
    IsMandatory                                                            = 4606,
    /// IS-OBD-RELEVANT
    IsObdRelevant                                                          = 5800,
    /// IS-OPTIONAL
    IsOptional                                                             = 3606,
    /// IS-PIPELINING-PERMITTED
    IsPipeliningPermitted                                                  = 6179,
    /// IS-PRIVATE
    IsPrivate                                                              = 519,
    /// IS-REENTRANT
    IsReentrant                                                            = 2083,
    /// IS-REQUIRED
    IsRequired                                                             = 832,
    /// IS-SERVICE
    IsService                                                              = 4537,
    /// IS-STRUCT-WITH-OPTIONAL-ELEMENT
    IsStructWithOptionalElement                                            = 5822,
    /// IS-SYNCHRONOUS
    IsSynchronous                                                          = 3727,
    /// IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER
    IsSystemWideGlobalTimeMaster                                           = 4552,
    /// IS-UNUSED
    IsUnused                                                               = 2706,
    /// IS-VARIADIC-TEMPLATE
    IsVariadicTemplate                                                     = 4044,
    /// ISSUED-BY
    IssuedBy                                                               = 5907,
    /// ISSUED-TRIGGERS
    IssuedTriggers                                                         = 5941,
    /// IT-REF
    ItRef                                                                  = 4874,
    /// ITEM
    Item                                                                   = 2581,
    /// ITEM-DEFINITIONS
    ItemDefinitions                                                        = 1509,
    /// ITEM-LABEL
    ItemLabel                                                              = 3008,
    /// ITEMS
    Items                                                                  = 859,
    /// IUMPR-GROUP
    IumprGroup                                                             = 550,
    /// IUMPR-GROUP-IDENTIFIERS
    IumprGroupIdentifiers                                                  = 830,
    /// IUMPR-REF
    IumprRef                                                               = 5193,
    /// IUMPR-REFS
    IumprRefs                                                              = 6214,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 5284,
    /// J-1939-CLUSTER-CONDITIONAL
    J1939ClusterConditional                                                = 3885,
    /// J-1939-CLUSTER-VARIANTS
    J1939ClusterVariants                                                   = 4774,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 4924,
    /// J-1939-CONTROLLER-APPLICATION-REF
    J1939ControllerApplicationRef                                          = 5629,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING
    J1939ControllerApplicationToJ1939NmNodeMapping                         = 5235,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS
    J1939ControllerApplicationToJ1939NmNodeMappings                        = 1124,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 2640,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 4718,
    /// J-1939-DTC-VALUE
    J1939DtcValue                                                          = 1284,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 3290,
    /// J-1939-NM-ECU
    J1939NmEcu                                                             = 2250,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 6264,
    /// J-1939-NM-NODE-REF
    J1939NmNodeRef                                                         = 960,
    /// J-1939-PROTECTED-I-PDU
    J1939ProtectedIPdu                                                     = 3277,
    /// J-1939-REQUESTABLE
    J1939Requestable                                                       = 3552,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 1766,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 2361,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 3675,
    /// J-1939-SHARED-ADDRESS-CLUSTERS
    J1939SharedAddressClusters                                             = 4258,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 3078,
    /// J-1939-TP-CONNECTION
    J1939TpConnection                                                      = 1231,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 2040,
    /// J-1939-TP-PG
    J1939TpPg                                                              = 3219,
    /// JITTER
    Jitter                                                                 = 3493,
    /// JOB-PRIMITIVE-REF
    JobPrimitiveRef                                                        = 5778,
    /// JOB-REF
    JobRef                                                                 = 1964,
    /// JOB-REQUIREMENTS
    JobRequirements                                                        = 4884,
    /// JUMP-TO-BOOT-LOADER
    JumpToBootLoader                                                       = 5071,
    /// K-INIT-RESOURCE
    KInitResource                                                          = 5934,
    /// K-RUN-RESOURCE
    KRunResource                                                           = 2932,
    /// KEEP-ALIVE-INTERVAL
    KeepAliveInterval                                                      = 648,
    /// KEEP-ALIVE-PROBES-MAX
    KeepAliveProbesMax                                                     = 1702,
    /// KEEP-ALIVE-TIME
    KeepAliveTime                                                          = 2977,
    /// KEEP-ALIVES
    KeepAlives                                                             = 1196,
    /// KEY
    Key                                                                    = 1613,
    /// KEY-EXCHANGE-AUTHENTICATION-REF
    KeyExchangeAuthenticationRef                                           = 3190,
    /// KEY-EXCHANGE-AUTHENTICATION-REFS
    KeyExchangeAuthenticationRefs                                          = 3884,
    /// KEY-EXCHANGE-REF
    KeyExchangeRef                                                         = 4912,
    /// KEY-EXCHANGE-REFS
    KeyExchangeRefs                                                        = 566,
    /// KEY-GENERATION
    KeyGeneration                                                          = 3130,
    /// KEY-ID
    KeyId                                                                  = 1652,
    /// KEY-IDENTITY-REF
    KeyIdentityRef                                                         = 687,
    /// KEY-REF
    KeyRef                                                                 = 620,
    /// KEY-SERVER-PRIORITY
    KeyServerPriority                                                      = 330,
    /// KEY-SIZE
    KeySize                                                                = 3011,
    /// KEY-SLOT-ALLOWED-MODIFICATION
    KeySlotAllowedModification                                             = 2008,
    /// KEY-SLOT-CONTENT-ALLOWED-USAGES
    KeySlotContentAllowedUsages                                            = 96,
    /// KEY-SLOT-ID
    KeySlotId                                                              = 6357,
    /// KEY-SLOT-ONLY-ENABLED
    KeySlotOnlyEnabled                                                     = 584,
    /// KEY-SLOT-REF
    KeySlotRef                                                             = 164,
    /// KEY-SLOT-USAGE
    KeySlotUsage                                                           = 6085,
    /// KEY-SLOT-USAGES
    KeySlotUsages                                                          = 1777,
    /// KEY-SLOT-USED-FOR-START-UP
    KeySlotUsedForStartUp                                                  = 2765,
    /// KEY-SLOT-USED-FOR-SYNC
    KeySlotUsedForSync                                                     = 1933,
    /// KEY-SLOTS
    KeySlots                                                               = 428,
    /// KEY-STORAGE-TYPE
    KeyStorageType                                                         = 5167,
    /// KEY-VALUE-PAIRS
    KeyValuePairs                                                          = 5086,
    /// KEY-VALUE-STORAGE-REF
    KeyValueStorageRef                                                     = 3380,
    /// KEYWORD
    Keyword                                                                = 1837,
    /// KEYWORD-SET
    KeywordSet                                                             = 2254,
    /// KEYWORDS
    Keywords                                                               = 4782,
    /// KIND
    Kind                                                                   = 3104,
    /// L-1
    L1                                                                     = 6351,
    /// L-10
    L10                                                                    = 5220,
    /// L-2
    L2                                                                     = 3023,
    /// L-4
    L4                                                                     = 2661,
    /// L-5
    L5                                                                     = 6384,
    /// L-GRAPHIC
    LGraphic                                                               = 1666,
    /// LABEL
    Label                                                                  = 915,
    /// LABEL-1
    Label1                                                                 = 1359,
    /// LABELED-ITEM
    LabeledItem                                                            = 285,
    /// LABELED-LIST
    LabeledList                                                            = 1257,
    /// LANGUAGE
    Language                                                               = 5639,
    /// LAST-EGRESS-SCHEDULER-REF
    LastEgressSchedulerRef                                                 = 3022,
    /// LATENCY-BUDGET
    LatencyBudget                                                          = 3820,
    /// LATENCY-BUDGET-DURATION
    LatencyBudgetDuration                                                  = 6417,
    /// LATENCY-CONSTRAINT-TYPE
    LatencyConstraintType                                                  = 5340,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 6124,
    /// LATEST-TX
    LatestTx                                                               = 3444,
    /// LC-OBJECT-REF
    LcObjectRef                                                            = 2646,
    /// LC-STATE-REF
    LcStateRef                                                             = 4247,
    /// LC-STATES
    LcStates                                                               = 3352,
    /// LDAP-URL
    LdapUrl                                                                = 3965,
    /// LEAF-ELEMENT-IREF
    LeafElementIref                                                        = 1249,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS
    LegislatedFreezeFrameContentUdsObds                                    = 5460,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS
    LegislatedFreezeFrameContentWwhObds                                    = 4823,
    /// LENGTH
    Length                                                                 = 388,
    /// LENGTH-CLIENT-ID
    LengthClientId                                                         = 2243,
    /// LENGTH-EXP
    LengthExp                                                              = 3180,
    /// LENGTH-SEQUENCE-COUNTER
    LengthSequenceCounter                                                  = 1214,
    /// LENGTH-VERIFICATION
    LengthVerification                                                     = 4486,
    /// LET-DATA-EXCHANGE-PARADIGM
    LetDataExchangeParadigm                                                = 1852,
    /// LET-INTERVAL-REF
    LetIntervalRef                                                         = 84,
    /// LET-INTERVAL-REFS
    LetIntervalRefs                                                        = 6197,
    /// LICENSE-REF
    LicenseRef                                                             = 4485,
    /// LICENSE-REFS
    LicenseRefs                                                            = 5243,
    /// LIFE-CYCLE-INFO
    LifeCycleInfo                                                          = 4465,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 567,
    /// LIFE-CYCLE-INFOS
    LifeCycleInfos                                                         = 1983,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 2045,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 3614,
    /// LIFESPAN
    Lifespan                                                               = 4062,
    /// LIFESPAN-DURATION
    LifespanDuration                                                       = 1987,
    /// LIMIT
    Limit                                                                  = 2712,
    /// LIMIT-KIND
    LimitKind                                                              = 1010,
    /// LIN-CHECKSUM
    LinChecksum                                                            = 5423,
    /// LIN-CLUSTER
    LinCluster                                                             = 3639,
    /// LIN-CLUSTER-CONDITIONAL
    LinClusterConditional                                                  = 5305,
    /// LIN-CLUSTER-VARIANTS
    LinClusterVariants                                                     = 3318,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 1463,
    /// LIN-CONFIGURABLE-FRAME
    LinConfigurableFrame                                                   = 6394,
    /// LIN-CONFIGURABLE-FRAMES
    LinConfigurableFrames                                                  = 5534,
    /// LIN-ERROR-RESPONSE
    LinErrorResponse                                                       = 5016,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 170,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 1007,
    /// LIN-IDENTIFIER
    LinIdentifier                                                          = 4556,
    /// LIN-MASTER
    LinMaster                                                              = 565,
    /// LIN-MASTER-CONDITIONAL
    LinMasterConditional                                                   = 547,
    /// LIN-MASTER-VARIANTS
    LinMasterVariants                                                      = 4858,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 3517,
    /// LIN-ORDERED-CONFIGURABLE-FRAME
    LinOrderedConfigurableFrame                                            = 4112,
    /// LIN-ORDERED-CONFIGURABLE-FRAMES
    LinOrderedConfigurableFrames                                           = 1969,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 5226,
    /// LIN-PID-TO-CAN-ID-MAPPINGS
    LinPidToCanIdMappings                                                  = 367,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 4510,
    /// LIN-SLAVE
    LinSlave                                                               = 3980,
    /// LIN-SLAVE-CONDITIONAL
    LinSlaveConditional                                                    = 1997,
    /// LIN-SLAVE-CONFIG
    LinSlaveConfig                                                         = 4204,
    /// LIN-SLAVE-ECU-REF
    LinSlaveEcuRef                                                         = 709,
    /// LIN-SLAVE-VARIANTS
    LinSlaveVariants                                                       = 3453,
    /// LIN-SLAVES
    LinSlaves                                                              = 405,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 1478,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 4305,
    /// LIN-TP-CONNECTION
    LinTpConnection                                                        = 2526,
    /// LIN-TP-N-SDU-REF
    LinTpNSduRef                                                           = 4761,
    /// LIN-TP-NODE
    LinTpNode                                                              = 2,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 1258,
    /// LIN-UNCONDITIONAL-FRAME-REF
    LinUnconditionalFrameRef                                               = 1564,
    /// LIN-UNCONDITIONAL-FRAME-REFS
    LinUnconditionalFrameRefs                                              = 4997,
    /// LINKER
    Linker                                                                 = 1443,
    /// LINKERS
    Linkers                                                                = 3221,
    /// LIST
    List                                                                   = 5163,
    /// LISTEN-NOISE
    ListenNoise                                                            = 5302,
    /// LISTEN-TIMEOUT
    ListenTimeout                                                          = 2604,
    /// LITERAL-PREFIX
    LiteralPrefix                                                          = 3584,
    /// LITERALS
    Literals                                                               = 4496,
    /// LIVELINESS
    Liveliness                                                             = 5317,
    /// LIVELINESS-LEASE-DURATION
    LivelinessLeaseDuration                                                = 887,
    /// LIVELINESS-PROTECTION-KIND
    LivelinessProtectionKind                                               = 264,
    /// LIVENESS-KIND
    LivenessKind                                                           = 402,
    /// LOAD-BALANCING-PRIORITY
    LoadBalancingPriority                                                  = 2966,
    /// LOAD-BALANCING-WEIGHT
    LoadBalancingWeight                                                    = 4449,
    /// LOC-MEASUREMENT-SET
    LocMeasurementSet                                                      = 513,
    /// LOCAL-CERTIFICATE-REF
    LocalCertificateRef                                                    = 2471,
    /// LOCAL-CERTIFICATE-REFS
    LocalCertificateRefs                                                   = 1609,
    /// LOCAL-COM-ACCESS-CONTROL-ENABLED
    LocalComAccessControlEnabled                                           = 1917,
    /// LOCAL-COMM-CONNECTOR-REF
    LocalCommConnectorRef                                                  = 3990,
    /// LOCAL-DEBUG-DATAS
    LocalDebugDatas                                                        = 1336,
    /// LOCAL-ENDPOINT-CONFIG
    LocalEndpointConfig                                                    = 1183,
    /// LOCAL-ID
    LocalId                                                                = 2818,
    /// LOCAL-IP-SEC-RULE-REF
    LocalIpSecRuleRef                                                      = 469,
    /// LOCAL-IP-SEC-RULE-REFS
    LocalIpSecRuleRefs                                                     = 4792,
    /// LOCAL-NETWORK-ENDPOINT-REF
    LocalNetworkEndpointRef                                                = 291,
    /// LOCAL-PARAMETER-REF
    LocalParameterRef                                                      = 3107,
    /// LOCAL-PORT-RANGE-END
    LocalPortRangeEnd                                                      = 4180,
    /// LOCAL-PORT-RANGE-START
    LocalPortRangeStart                                                    = 232,
    /// LOCAL-PORT-REF
    LocalPortRef                                                           = 4488,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 4268,
    /// LOCAL-SUPERVISION-ENTITY-REF
    LocalSupervisionEntityRef                                              = 2455,
    /// LOCAL-SUPERVISION-ENTITY-REFS
    LocalSupervisionEntityRefs                                             = 2565,
    /// LOCAL-SUPERVISION-REF
    LocalSupervisionRef                                                    = 5479,
    /// LOCAL-SUPERVISION-REFS
    LocalSupervisionRefs                                                   = 3589,
    /// LOCAL-SUPERVISIONS
    LocalSupervisions                                                      = 130,
    /// LOCAL-TCP-PORT-REF
    LocalTcpPortRef                                                        = 4568,
    /// LOCAL-UDP-PORT-REF
    LocalUdpPortRef                                                        = 491,
    /// LOCAL-UNICAST-ADDRESSES
    LocalUnicastAddresses                                                  = 992,
    /// LOCAL-UNICAST-ADDRESSS
    LocalUnicastAddresss                                                   = 5025,
    /// LOCAL-VARIABLE-REF
    LocalVariableRef                                                       = 3018,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 4996,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 2878,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 5504,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF
    LogAndTraceMessageCollectionSetRef                                     = 5403,
    /// LOG-CHANNEL-ID
    LogChannelId                                                           = 4683,
    /// LOG-SINK-REF
    LogSinkRef                                                             = 6,
    /// LOG-SINK-REFS
    LogSinkRefs                                                            = 1289,
    /// LOG-TRACE-DEFAULT-LOG-LEVEL
    LogTraceDefaultLogLevel                                                = 5616,
    /// LOG-TRACE-DEFAULT-LOG-THRESHOLD
    LogTraceDefaultLogThreshold                                            = 6150,
    /// LOG-TRACE-FILE-PATH
    LogTraceFilePath                                                       = 2876,
    /// LOG-TRACE-LOG-MODE
    LogTraceLogMode                                                        = 2429,
    /// LOG-TRACE-LOG-MODES
    LogTraceLogModes                                                       = 1042,
    /// LOG-TRACE-PORT-ID
    LogTracePortId                                                         = 2839,
    /// LOG-TRACE-PROCESS-DESC
    LogTraceProcessDesc                                                    = 381,
    /// LOG-TRACE-PROCESS-ID
    LogTraceProcessId                                                      = 4848,
    /// LOGGING-BEHAVIOR
    LoggingBehavior                                                        = 6185,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 5431,
    /// LOGIC-ADDRESSS
    LogicAddresss                                                          = 2789,
    /// LOGICAL-ADDRESS
    LogicalAddress                                                         = 2725,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 517,
    /// LOGICAL-EXPRESSION-ARGUMENT-REF
    LogicalExpressionArgumentRef                                           = 2985,
    /// LOGICAL-EXPRESSION-ARGUMENT-REFS
    LogicalExpressionArgumentRefs                                          = 3878,
    /// LOGICAL-EXPRESSIONS
    LogicalExpressions                                                     = 2910,
    /// LOGICAL-OPERATOR
    LogicalOperator                                                        = 875,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 6065,
    /// LOGICAL-SUPERVISIONS
    LogicalSupervisions                                                    = 1583,
    /// LONG-NAME
    LongName                                                               = 4569,
    /// LONG-NAME-1
    LongName1                                                              = 4074,
    /// LOWER-BOUND
    LowerBound                                                             = 233,
    /// LOWER-BOUNDARY
    LowerBoundary                                                          = 20,
    /// LOWER-CAN-ID
    LowerCanId                                                             = 5673,
    /// LOWER-LIMIT
    LowerLimit                                                             = 6402,
    /// LOWER-MULTIPLICITY
    LowerMultiplicity                                                      = 1108,
    /// LOWER-REF
    LowerRef                                                               = 4197,
    /// LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING
    LtMessageCollectionToPortPrototypeMapping                              = 1958,
    /// LUMINOUS-INTENSITY-EXP
    LuminousIntensityExp                                                   = 3402,
    /// M
    M                                                                      = 274,
    /// MAC-ADDRESS
    MacAddress                                                             = 5936,
    /// MAC-ADDRESS-MASK
    MacAddressMask                                                         = 999,
    /// MAC-ADDRESS-STREAM-ID
    MacAddressStreamId                                                     = 3977,
    /// MAC-ADDRESS-VLAN-ASSIGNMENTS
    MacAddressVlanAssignments                                              = 4082,
    /// MAC-ADDRESS-VLAN-MEMBERSHIP
    MacAddressVlanMembership                                               = 1742,
    /// MAC-LAYER-TYPE
    MacLayerType                                                           = 2515,
    /// MAC-MULTICAST-ADDRESS
    MacMulticastAddress                                                    = 5671,
    /// MAC-MULTICAST-ADDRESS-REF
    MacMulticastAddressRef                                                 = 1178,
    /// MAC-MULTICAST-ADDRESS-REFS
    MacMulticastAddressRefs                                                = 4687,
    /// MAC-MULTICAST-CONFIGURATION
    MacMulticastConfiguration                                              = 5562,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 6096,
    /// MAC-MULTICAST-GROUP-REF
    MacMulticastGroupRef                                                   = 4518,
    /// MAC-MULTICAST-GROUPS
    MacMulticastGroups                                                     = 4754,
    /// MAC-SEC-CIPHER-SUITE-CONFIG
    MacSecCipherSuiteConfig                                                = 2541,
    /// MAC-SEC-GLOBAL-KAY-PROPS
    MacSecGlobalKayProps                                                   = 2245,
    /// MAC-SEC-KAY-CONFIG
    MacSecKayConfig                                                        = 5325,
    /// MAC-SEC-KAY-PARTICIPANT
    MacSecKayParticipant                                                   = 347,
    /// MAC-SEC-PARTICIPANT-SET
    MacSecParticipantSet                                                   = 5432,
    /// MAC-SEC-PROPS
    MacSecProps                                                            = 5048,
    /// MAC-SEC-PROPSS
    MacSecPropss                                                           = 2853,
    /// MAC-UNICAST-ADDRESS
    MacUnicastAddress                                                      = 4296,
    /// MACHINE
    Machine                                                                = 3392,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 5477,
    /// MACHINE-DESIGN-REF
    MachineDesignRef                                                       = 6245,
    /// MACHINE-ID
    MachineId                                                              = 2525,
    /// MACHINE-MODE-IREF
    MachineModeIref                                                        = 2363,
    /// MACHINE-MODE-IREFS
    MachineModeIrefs                                                       = 3247,
    /// MACHINE-MODE-MACHINES
    MachineModeMachines                                                    = 1576,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 3468,
    /// MACHINE-REF
    MachineRef                                                             = 946,
    /// MACHINE-TIMING
    MachineTiming                                                          = 656,
    /// MACRO-INITIAL-OFFSET-A
    MacroInitialOffsetA                                                    = 5882,
    /// MACRO-INITIAL-OFFSET-B
    MacroInitialOffsetB                                                    = 3012,
    /// MACRO-PER-CYCLE
    MacroPerCycle                                                          = 424,
    /// MACROTICK-DURATION
    MacrotickDuration                                                      = 5365,
    /// MAIN-FUNCTION-PERIOD
    MainFunctionPeriod                                                     = 3800,
    /// MAJOR-PROTOCOL-VERSION
    MajorProtocolVersion                                                   = 3257,
    /// MAJOR-VERSION
    MajorVersion                                                           = 3025,
    /// MANAGED-COUPLING-PORTS
    ManagedCouplingPorts                                                   = 3254,
    /// MANAGED-MODE-GROUPS
    ManagedModeGroups                                                      = 4641,
    /// MANAGED-PHYSICAL-CHANNEL-REF
    ManagedPhysicalChannelRef                                              = 4894,
    /// MANAGED-PHYSICAL-CHANNEL-REFS
    ManagedPhysicalChannelRefs                                             = 908,
    /// MANDATORY
    Mandatory                                                              = 4633,
    /// MANUFACTURER-CODE
    ManufacturerCode                                                       = 5622,
    /// MANUFACTURER-VALIDATION-ORDER-REF
    ManufacturerValidationOrderRef                                         = 24,
    /// MANUFACTURER-VALIDATION-ORDER-REFS
    ManufacturerValidationOrderRefs                                        = 2067,
    /// MAP
    Map                                                                    = 947,
    /// MAP-ELEMENT-TUPLES
    MapElementTuples                                                       = 6324,
    /// MAPPED-AP-DATA-ELEMENT-IREF
    MappedApDataElementIref                                                = 2829,
    /// MAPPED-BSW-SERVICE-DEPENDENCY-REF
    MappedBswServiceDependencyRef                                          = 4666,
    /// MAPPED-DATA-ELEMENT-IREF
    MappedDataElementIref                                                  = 5930,
    /// MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF
    MappedFlatSwcServiceDependencyRef                                      = 5320,
    /// MAPPED-FUNCTION-REF
    MappedFunctionRef                                                      = 4843,
    /// MAPPED-OPERATION-IREF
    MappedOperationIref                                                    = 6310,
    /// MAPPED-SECURITY-EVENTS
    MappedSecurityEvents                                                   = 516,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    MappedSwcServiceDependencyInExecutableIref                             = 5285,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    MappedSwcServiceDependencyInSystemIref                                 = 3534,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IREF
    MappedSwcServiceDependencyIref                                         = 5903,
    /// MAPPING-CONSTRAINTS
    MappingConstraints                                                     = 2273,
    /// MAPPING-DIRECTION
    MappingDirection                                                       = 5810,
    /// MAPPING-REF
    MappingRef                                                             = 6177,
    /// MAPPING-SCOPE
    MappingScope                                                           = 4269,
    /// MAPPINGS
    Mappings                                                               = 3194,
    /// MASK
    Mask                                                                   = 4789,
    /// MASS-EXP
    MassExp                                                                = 19,
    /// MASTER
    Master                                                                 = 5440,
    /// MASTER-EVENT-REF
    MasterEventRef                                                         = 954,
    /// MASTER-REF
    MasterRef                                                              = 3343,
    /// MASTER-TO-SLAVE-EVENT-MAPPINGS
    MasterToSlaveEventMappings                                             = 4091,
    /// MASTERED-TRIGGER-REF
    MasteredTriggerRef                                                     = 2183,
    /// MATCHING-CRITERION-REF
    MatchingCriterionRef                                                   = 6377,
    /// MATCHING-EGRESS-RULE-REF
    MatchingEgressRuleRef                                                  = 3411,
    /// MATCHING-EGRESS-RULE-REFS
    MatchingEgressRuleRefs                                                 = 570,
    /// MATCHING-INGRESS-RULE-REF
    MatchingIngressRuleRef                                                 = 45,
    /// MATCHING-INGRESS-RULE-REFS
    MatchingIngressRuleRefs                                                = 34,
    /// MATCHING-RULE-REF
    MatchingRuleRef                                                        = 3608,
    /// MATCHING-RULE-REFS
    MatchingRuleRefs                                                       = 2137,
    /// MAX
    Max                                                                    = 6226,
    /// MAX-ALIVE-CYCLE
    MaxAliveCycle                                                          = 4809,
    /// MAX-AR
    MaxAr                                                                  = 4801,
    /// MAX-AS
    MaxAs                                                                  = 3801,
    /// MAX-AVAILABLE-PERSISTENCY-STORAGE-SPACE
    MaxAvailablePersistencyStorageSpace                                    = 3712,
    /// MAX-BASE-TYPE-SIZE
    MaxBaseTypeSize                                                        = 5347,
    /// MAX-BLOCK-SIZE
    MaxBlockSize                                                           = 2203,
    /// MAX-BS
    MaxBs                                                                  = 4346,
    /// MAX-BUFFER-REQUEST
    MaxBufferRequest                                                       = 2368,
    /// MAX-BUFFER-SIZE
    MaxBufferSize                                                          = 956,
    /// MAX-BYTES-IN-INTERVAL
    MaxBytesInInterval                                                     = 2635,
    /// MAX-COMM-MODE
    MaxCommMode                                                            = 5979,
    /// MAX-CONVERSATIONS
    MaxConversations                                                       = 2724,
    /// MAX-CYCLE-REPETITIONS
    MaxCycleRepetitions                                                    = 5561,
    /// MAX-CYCLES
    MaxCycles                                                              = 1624,
    /// MAX-DATA-LENGTH
    MaxDataLength                                                          = 4898,
    /// MAX-DEADLINE
    MaxDeadline                                                            = 1040,
    /// MAX-DELTA-COUNTER
    MaxDeltaCounter                                                        = 2905,
    /// MAX-DELTA-COUNTER-INIT
    MaxDeltaCounterInit                                                    = 3810,
    /// MAX-DESTINATION-PORT-NUMBER
    MaxDestinationPortNumber                                               = 6186,
    /// MAX-DID-TO-READ
    MaxDidToRead                                                           = 4284,
    /// MAX-DIFF
    MaxDiff                                                                = 5718,
    /// MAX-ERROR-STATE-INIT
    MaxErrorStateInit                                                      = 5893,
    /// MAX-ERROR-STATE-INVALID
    MaxErrorStateInvalid                                                   = 5789,
    /// MAX-ERROR-STATE-VALID
    MaxErrorStateValid                                                     = 805,
    /// MAX-EVENTS-IN-INTERVAL
    MaxEventsInInterval                                                    = 2950,
    /// MAX-EXP-BS
    MaxExpBs                                                               = 2517,
    /// MAX-FC-WAIT
    MaxFcWait                                                              = 558,
    /// MAX-FR-IF
    MaxFrIf                                                                = 3779,
    /// MAX-GRADIENT
    MaxGradient                                                            = 3847,
    /// MAX-HEADER-ID
    MaxHeaderId                                                            = 5323,
    /// MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    MaxInitialVehicleAnnouncementTime                                      = 1275,
    /// MAX-INSTANCES
    MaxInstances                                                           = 2730,
    /// MAX-LENGTH
    MaxLength                                                              = 370,
    /// MAX-MAJOR-VERSION
    MaxMajorVersion                                                        = 3835,
    /// MAX-MARGIN
    MaxMargin                                                              = 2244,
    /// MAX-MINOR-VERSION
    MaxMinorVersion                                                        = 2054,
    /// MAX-NO-NEW-OR-REPEATED-DATA
    MaxNoNewOrRepeatedData                                                 = 6395,
    /// MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS
    MaxNumChangeOfDataIdentfierEvents                                      = 5055,
    /// MAX-NUM-COMPARISION-OF-VALUE-EVENTS
    MaxNumComparisionOfValueEvents                                         = 1419,
    /// MAX-NUMBER-FREEZE-FRAME-RECORDS
    MaxNumberFreezeFrameRecords                                            = 1902,
    /// MAX-NUMBER-OF-ALLOWED-UPDATES
    MaxNumberOfAllowedUpdates                                              = 691,
    /// MAX-NUMBER-OF-CONNECTIONS
    MaxNumberOfConnections                                                 = 140,
    /// MAX-NUMBER-OF-ELEMENTS
    MaxNumberOfElements                                                    = 4629,
    /// MAX-NUMBER-OF-EVENT-ENTRIES
    MaxNumberOfEventEntries                                                = 3904,
    /// MAX-NUMBER-OF-FILES
    MaxNumberOfFiles                                                       = 1015,
    /// MAX-NUMBER-OF-NPDU-PER-CYCLE
    MaxNumberOfNpduPerCycle                                                = 362,
    /// MAX-NUMBER-OF-OCCURRENCES
    MaxNumberOfOccurrences                                                 = 4673,
    /// MAX-NUMBER-OF-PARALLEL-TRANSFERS
    MaxNumberOfParallelTransfers                                           = 5691,
    /// MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING
    MaxNumberOfRequestCorrectlyReceivedResponsePending                     = 1090,
    /// MAX-NUMBER-OF-RESP-PENDING-FRAMES
    MaxNumberOfRespPendingFrames                                           = 640,
    /// MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS
    MaxNumberOfStoredDtcStatusChangedEvents                                = 5303,
    /// MAX-NUMBER-OF-TIME-QUANTA-PER-BIT
    MaxNumberOfTimeQuantaPerBit                                            = 38,
    /// MAX-PERIODIC-DID-TO-READ
    MaxPeriodicDidToRead                                                   = 1584,
    /// MAX-PROGRESSION-MISMATCH-THRESHOLD
    MaxProgressionMismatchThreshold                                        = 248,
    /// MAX-PWM-L
    MaxPwmL                                                                = 937,
    /// MAX-PWM-O
    MaxPwmO                                                                = 1798,
    /// MAX-PWM-S
    MaxPwmS                                                                = 4191,
    /// MAX-REQUEST-BYTES
    MaxRequestBytes                                                        = 206,
    /// MAX-RETRIES
    MaxRetries                                                             = 5126,
    /// MAX-RPT-EVENT-ID
    MaxRptEventId                                                          = 2612,
    /// MAX-RUN-CYCLE-LOOP
    MaxRunCycleLoop                                                        = 49,
    /// MAX-SAMPLE-POINT
    MaxSamplePoint                                                         = 2285,
    /// MAX-SAMPLES
    MaxSamples                                                             = 2121,
    /// MAX-SAMPLES-PER-INSTANCE
    MaxSamplesPerInstance                                                  = 4717,
    /// MAX-SDU-SIZE
    MaxSduSize                                                             = 4509,
    /// MAX-SERVICE-POINT-ID
    MaxServicePointId                                                      = 3886,
    /// MAX-SIZE-TO-FILL
    MaxSizeToFill                                                          = 4963,
    /// MAX-SLOTS
    MaxSlots                                                               = 4601,
    /// MAX-SLOTS-PER-CYCLE
    MaxSlotsPerCycle                                                       = 1708,
    /// MAX-SOURCE-ELEMENT
    MaxSourceElement                                                       = 5726,
    /// MAX-SOURCE-PORT-NUMBER
    MaxSourcePortNumber                                                    = 4119,
    /// MAX-SUPPORTED-DID-LENGTH
    MaxSupportedDidLength                                                  = 2520,
    /// MAX-SYNC-JUMP-WIDTH
    MaxSyncJumpWidth                                                       = 3050,
    /// MAX-TESTER-CONNECTIONS
    MaxTesterConnections                                                   = 4967,
    /// MAX-TRANSIT-TIME
    MaxTransitTime                                                         = 1558,
    /// MAX-TRCV-DELAY-COMPENSATION-OFFSET
    MaxTrcvDelayCompensationOffset                                         = 6299,
    /// MAX-VALUE
    MaxValue                                                               = 2716,
    /// MAX-WITHOUT-CLOCK-CORRECTION-FATAL
    MaxWithoutClockCorrectionFatal                                         = 2573,
    /// MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE
    MaxWithoutClockCorrectionPassive                                       = 3093,
    /// MAXIMUM
    Maximum                                                                = 980,
    /// MAXIMUM-ALLOWED-SIZE
    MaximumAllowedSize                                                     = 761,
    /// MAXIMUM-DISTANCES
    MaximumDistances                                                       = 1540,
    /// MAXIMUM-DURATION-OF-CAMPAIGN
    MaximumDurationOfCampaign                                              = 1881,
    /// MAXIMUM-DYNAMIC-PAYLOAD-LENGTH
    MaximumDynamicPayloadLength                                            = 4869,
    /// MAXIMUM-EXECUTION-TIME
    MaximumExecutionTime                                                   = 1308,
    /// MAXIMUM-INTENDED-BINDING-TIME
    MaximumIntendedBindingTime                                             = 4291,
    /// MAXIMUM-INTER-ARRIVAL-TIME
    MaximumInterArrivalTime                                                = 1562,
    /// MAXIMUM-KEY-LENGTH
    MaximumKeyLength                                                       = 4614,
    /// MAXIMUM-LENGTH
    MaximumLength                                                          = 245,
    /// MAXIMUM-MEMORY-CONSUMPTION
    MaximumMemoryConsumption                                               = 2723,
    /// MAXIMUM-MESSAGE-LENGTH
    MaximumMessageLength                                                   = 786,
    /// MAXIMUM-NUMBER-OF-RETRIES
    MaximumNumberOfRetries                                                 = 6035,
    /// MAXIMUM-RECEIVE-BUFFER-LENGTH
    MaximumReceiveBufferLength                                             = 3822,
    /// MAXIMUM-RESIDENCE-TIME
    MaximumResidenceTime                                                   = 4576,
    /// MAXIMUM-SEGMENT-LENGTH
    MaximumSegmentLength                                                   = 4630,
    /// MAXIMUM-SEGMENT-LENGTH-REQUEST
    MaximumSegmentLengthRequest                                            = 1996,
    /// MAXIMUM-SEGMENT-LENGTH-RESPONSE
    MaximumSegmentLengthResponse                                           = 1080,
    /// MAXIMUM-SELECTED-BINDING-TIME
    MaximumSelectedBindingTime                                             = 1757,
    /// MAXIMUM-SUPPORTED-UCM-MASTER-VERSION
    MaximumSupportedUcmMasterVersion                                       = 4788,
    /// MAXIMUM-SUPPORTED-UCM-VERSION
    MaximumSupportedUcmVersion                                             = 3217,
    /// MAXIMUM-TRANSMISSION-UNIT
    MaximumTransmissionUnit                                                = 5685,
    /// MAXIMUM-TRANSMIT-BUFFER-LENGTH
    MaximumTransmitBufferLength                                            = 6288,
    /// MAY-BE-UNCONNECTED
    MayBeUnconnected                                                       = 4494,
    /// MC-DATA-ACCESS-DETAILS
    McDataAccessDetails                                                    = 1905,
    /// MC-DATA-ASSIGNMENTS
    McDataAssignments                                                      = 2750,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 5267,
    /// MC-DATA-INSTANCE-REF
    McDataInstanceRef                                                      = 1152,
    /// MC-DATA-INSTANCE-REFS
    McDataInstanceRefs                                                     = 1863,
    /// MC-DATA-INSTANCE-VAR-REF
    McDataInstanceVarRef                                                   = 5041,
    /// MC-FUNCTION
    McFunction                                                             = 1781,
    /// MC-FUNCTION-DATA-REF-SET-CONDITIONAL
    McFunctionDataRefSetConditional                                        = 2552,
    /// MC-FUNCTION-DATA-REF-SET-VARIANTS
    McFunctionDataRefSetVariants                                           = 3047,
    /// MC-FUNCTION-REF
    McFunctionRef                                                          = 6141,
    /// MC-FUNCTION-REFS
    McFunctionRefs                                                         = 5458,
    /// MC-GROUP
    McGroup                                                                = 61,
    /// MC-GROUP-DATA-REF-SET-CONDITIONAL
    McGroupDataRefSetConditional                                           = 6155,
    /// MC-GROUP-DATA-REF-SET-VARIANTS
    McGroupDataRefSetVariants                                              = 2478,
    /// MC-PARAMETER-ELEMENT-GROUP
    McParameterElementGroup                                                = 1217,
    /// MC-PARAMETER-INSTANCES
    McParameterInstances                                                   = 4807,
    /// MC-SUPPORT
    McSupport                                                              = 5466,
    /// MC-SW-EMULATION-METHOD-SUPPORT
    McSwEmulationMethodSupport                                             = 6361,
    /// MC-VARIABLE-INSTANCES
    McVariableInstances                                                    = 1391,
    /// MCD-IDENTIFIER
    McdIdentifier                                                          = 983,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REF
    MeasurableSystemConstantValuesRef                                      = 4369,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS
    MeasurableSystemConstantValuesRefs                                     = 2295,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 4594,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 1172,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 674,
    /// MEM-ALIGNMENT
    MemAlignment                                                           = 2166,
    /// MEM-CLASS-SYMBOL
    MemClassSymbol                                                         = 2962,
    /// MEM-USAGE
    MemUsage                                                               = 5311,
    /// MEMORY-ALLOCATION-KEYWORD-POLICY
    MemoryAllocationKeywordPolicy                                          = 2947,
    /// MEMORY-CONSUMPTION
    MemoryConsumption                                                      = 715,
    /// MEMORY-DESTINATION-REF
    MemoryDestinationRef                                                   = 4979,
    /// MEMORY-DESTINATION-REFS
    MemoryDestinationRefs                                                  = 4849,
    /// MEMORY-ENTRY-STORAGE-TRIGGER
    MemoryEntryStorageTrigger                                              = 3398,
    /// MEMORY-HIGH-ADDRESS
    MemoryHighAddress                                                      = 29,
    /// MEMORY-HIGH-ADDRESS-LABEL
    MemoryHighAddressLabel                                                 = 3791,
    /// MEMORY-ID
    MemoryId                                                               = 5094,
    /// MEMORY-LOW-ADDRESS
    MemoryLowAddress                                                       = 5698,
    /// MEMORY-LOW-ADDRESS-LABEL
    MemoryLowAddressLabel                                                  = 4143,
    /// MEMORY-RANGE-REF
    MemoryRangeRef                                                         = 6343,
    /// MEMORY-RANGE-REFS
    MemoryRangeRefs                                                        = 1963,
    /// MEMORY-SECTION
    MemorySection                                                          = 2643,
    /// MEMORY-SECTION-LOCATION
    MemorySectionLocation                                                  = 1516,
    /// MEMORY-SECTION-LOCATIONS
    MemorySectionLocations                                                 = 4065,
    /// MEMORY-SECTIONS
    MemorySections                                                         = 341,
    /// MEMORY-USAGE
    MemoryUsage                                                            = 2800,
    /// MEMORY-USAGES
    MemoryUsages                                                           = 936,
    /// MESSAGE-COMPLIANCE
    MessageCompliance                                                      = 1044,
    /// MESSAGE-ID
    MessageId                                                              = 2130,
    /// MESSAGE-LINE-NUMBER
    MessageLineNumber                                                      = 5586,
    /// MESSAGE-LINK-LENGTH
    MessageLinkLength                                                      = 5598,
    /// MESSAGE-LINK-POSITION
    MessageLinkPosition                                                    = 6045,
    /// MESSAGE-SOURCE-FILE
    MessageSourceFile                                                      = 2879,
    /// MESSAGE-TYPE
    MessageType                                                            = 912,
    /// MESSAGE-TYPE-INFO
    MessageTypeInfo                                                        = 868,
    /// META-DATA-FIELDS
    MetaDataFields                                                         = 300,
    /// META-DATA-ITEM
    MetaDataItem                                                           = 3807,
    /// META-DATA-ITEM-SET
    MetaDataItemSet                                                        = 5566,
    /// META-DATA-ITEM-SETS
    MetaDataItemSets                                                       = 2268,
    /// META-DATA-ITEM-TYPE
    MetaDataItemType                                                       = 4192,
    /// META-DATA-ITEMS
    MetaDataItems                                                          = 46,
    /// META-DATA-LENGTH
    MetaDataLength                                                         = 5258,
    /// METADATA-PROTECTION-KIND
    MetadataProtectionKind                                                 = 5085,
    /// METHOD-ACTIVATION-ROUTING-GROUPS
    MethodActivationRoutingGroups                                          = 3236,
    /// METHOD-CALL-REF
    MethodCallRef                                                          = 861,
    /// METHOD-CALL-REFS
    MethodCallRefs                                                         = 4734,
    /// METHOD-DEPLOYMENTS
    MethodDeployments                                                      = 270,
    /// METHOD-ID
    MethodId                                                               = 4733,
    /// METHOD-IREF
    MethodIref                                                             = 2297,
    /// METHOD-MAPPING
    MethodMapping                                                          = 3152,
    /// METHOD-MAPPINGS
    MethodMappings                                                         = 1327,
    /// METHOD-QOS-PROPSS
    MethodQosPropss                                                        = 2056,
    /// METHOD-REF
    MethodRef                                                              = 4015,
    /// METHOD-REFS
    MethodRefs                                                             = 796,
    /// METHOD-REPLY-TOPIC-NAME
    MethodReplyTopicName                                                   = 5079,
    /// METHOD-REQUEST-PROPSS
    MethodRequestPropss                                                    = 3414,
    /// METHOD-REQUEST-TOPIC-NAME
    MethodRequestTopicName                                                 = 5814,
    /// METHOD-RESPONSE-PROPSS
    MethodResponsePropss                                                   = 701,
    /// METHOD-RETURN-REF
    MethodReturnRef                                                        = 774,
    /// METHOD-RETURN-REFS
    MethodReturnRefs                                                       = 1238,
    /// METHOD-TOPICS-ACCESS-RULE-REF
    MethodTopicsAccessRuleRef                                              = 684,
    /// METHODS
    Methods                                                                = 4886,
    /// MICRO-INITIAL-OFFSET-A
    MicroInitialOffsetA                                                    = 495,
    /// MICRO-INITIAL-OFFSET-B
    MicroInitialOffsetB                                                    = 2609,
    /// MICRO-PER-CYCLE
    MicroPerCycle                                                          = 1809,
    /// MICROTICK-DURATION
    MicrotickDuration                                                      = 4618,
    /// MIN
    Min                                                                    = 721,
    /// MIN-ALIVE-CYCLE
    MinAliveCycle                                                          = 1497,
    /// MIN-DATA-LENGTH
    MinDataLength                                                          = 3539,
    /// MIN-DEADLINE
    MinDeadline                                                            = 3853,
    /// MIN-DESTINATION-PORT-NUMBER
    MinDestinationPortNumber                                               = 583,
    /// MIN-HEADER-ID
    MinHeaderId                                                            = 5696,
    /// MIN-LENGTH
    MinLength                                                              = 6304,
    /// MIN-MAJOR-VERSION
    MinMajorVersion                                                        = 2365,
    /// MIN-MARGIN
    MinMargin                                                              = 2688,
    /// MIN-MINOR-VERSION
    MinMinorVersion                                                        = 5392,
    /// MIN-NUMBER-OF-ELEMENTS
    MinNumberOfElements                                                    = 4302,
    /// MIN-NUMBER-OF-OCCURRENCES
    MinNumberOfOccurrences                                                 = 134,
    /// MIN-NUMBER-OF-TIME-QUANTA-PER-BIT
    MinNumberOfTimeQuantaPerBit                                            = 5845,
    /// MIN-OK-STATE-INIT
    MinOkStateInit                                                         = 669,
    /// MIN-OK-STATE-INVALID
    MinOkStateInvalid                                                      = 714,
    /// MIN-OK-STATE-VALID
    MinOkStateValid                                                        = 1219,
    /// MIN-PWM-L
    MinPwmL                                                                = 6213,
    /// MIN-PWM-O
    MinPwmO                                                                = 3932,
    /// MIN-PWM-S
    MinPwmS                                                                = 3636,
    /// MIN-RPT-EVENT-ID
    MinRptEventId                                                          = 527,
    /// MIN-SAMPLE-POINT
    MinSamplePoint                                                         = 2811,
    /// MIN-SERVICE-POINT-ID
    MinServicePointId                                                      = 611,
    /// MIN-SOURCE-PORT-NUMBER
    MinSourcePortNumber                                                    = 265,
    /// MIN-SYNC-JUMP-WIDTH
    MinSyncJumpWidth                                                       = 1640,
    /// MIN-TRCV-DELAY-COMPENSATION-OFFSET
    MinTrcvDelayCompensationOffset                                         = 2673,
    /// MIN-VALUE
    MinValue                                                               = 380,
    /// MINIMUM
    Minimum                                                                = 3900,
    /// MINIMUM-DELAY
    MinimumDelay                                                           = 4619,
    /// MINIMUM-DISTANCES
    MinimumDistances                                                       = 5929,
    /// MINIMUM-EXECUTION-TIME
    MinimumExecutionTime                                                   = 6062,
    /// MINIMUM-FIFO-LENGTH
    MinimumFifoLength                                                      = 3298,
    /// MINIMUM-INTENDED-BINDING-TIME
    MinimumIntendedBindingTime                                             = 3390,
    /// MINIMUM-INTER-ARRIVAL-TIME
    MinimumInterArrivalTime                                                = 4154,
    /// MINIMUM-INTERVAL-LENGTH
    MinimumIntervalLength                                                  = 2086,
    /// MINIMUM-MEMORY-CONSUMPTION
    MinimumMemoryConsumption                                               = 6440,
    /// MINIMUM-MULTICAST-SEPERATION-TIME
    MinimumMulticastSeperationTime                                         = 4527,
    /// MINIMUM-NUMBER-OF-REQUESTS
    MinimumNumberOfRequests                                                = 602,
    /// MINIMUM-RX-CONTAINER-QUEUE-SIZE
    MinimumRxContainerQueueSize                                            = 1978,
    /// MINIMUM-SELECTED-BINDING-TIME
    MinimumSelectedBindingTime                                             = 5327,
    /// MINIMUM-SEND-INTERVAL
    MinimumSendInterval                                                    = 1211,
    /// MINIMUM-SEPARATION-TIME
    MinimumSeparationTime                                                  = 4048,
    /// MINIMUM-START-INTERVAL
    MinimumStartInterval                                                   = 982,
    /// MINIMUM-SUPPORTED-UCM-MASTER-VERSION
    MinimumSupportedUcmMasterVersion                                       = 4270,
    /// MINIMUM-SUPPORTED-UCM-VERSION
    MinimumSupportedUcmVersion                                             = 621,
    /// MINIMUM-SUSTAINED-SIZE
    MinimumSustainedSize                                                   = 2441,
    /// MINIMUM-TIMER-GRANULARITY
    MinimumTimerGranularity                                                = 866,
    /// MINIMUM-TX-CONTAINER-QUEUE-SIZE
    MinimumTxContainerQueueSize                                            = 2453,
    /// MINISLOT-ACTION-POINT-OFFSET
    MinislotActionPointOffset                                              = 4615,
    /// MINISLOT-DURATION
    MinislotDuration                                                       = 1299,
    /// MINOR-PROTOCOL-VERSION
    MinorProtocolVersion                                                   = 4825,
    /// MINOR-VERSION
    MinorVersion                                                           = 5247,
    /// MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID
    MirrorSourceLinToCanRangeBaseId                                        = 3720,
    /// MIRROR-STATUS-CAN-ID
    MirrorStatusCanId                                                      = 2748,
    /// MIRRORING-PROTOCOL
    MirroringProtocol                                                      = 5186,
    /// MIXED-BUS-TYPE-COLLECTION
    MixedBusTypeCollection                                                 = 333,
    /// MKA-PARTICIPANT-REF
    MkaParticipantRef                                                      = 3781,
    /// MKA-PARTICIPANT-REFS
    MkaParticipantRefs                                                     = 4577,
    /// MKA-PARTICIPANTS
    MkaParticipants                                                        = 5976,
    /// ML-GRAPHIC
    MlGraphic                                                              = 4748,
    /// MODE
    Mode                                                                   = 921,
    /// MODE-ACCESS-POINT
    ModeAccessPoint                                                        = 9,
    /// MODE-ACCESS-POINTS
    ModeAccessPoints                                                       = 2411,
    /// MODE-CONDITION
    ModeCondition                                                          = 5889,
    /// MODE-CONDITION-REF
    ModeConditionRef                                                       = 4206,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 3412,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 4122,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 3234,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-IREF
    ModeDeclarationGroupPrototypeIref                                      = 5309,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ModeDeclarationGroupPrototypeRef                                       = 2351,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL
    ModeDeclarationGroupPrototypeRefConditional                            = 2595,
    /// MODE-DECLARATION-GROUP-REF
    ModeDeclarationGroupRef                                                = 1938,
    /// MODE-DECLARATION-GROUP-REFS
    ModeDeclarationGroupRefs                                               = 4859,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 3603,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 580,
    /// MODE-DECLARATION-MAPPING-SET-REF
    ModeDeclarationMappingSetRef                                           = 4477,
    /// MODE-DECLARATION-MAPPINGS
    ModeDeclarationMappings                                                = 80,
    /// MODE-DECLARATION-REF
    ModeDeclarationRef                                                     = 3901,
    /// MODE-DECLARATION-REFS
    ModeDeclarationRefs                                                    = 5125,
    /// MODE-DECLARATIONS
    ModeDeclarations                                                       = 173,
    /// MODE-DEPENDENT-STARTUP-CONFIG
    ModeDependentStartupConfig                                             = 4854,
    /// MODE-DEPENDENT-STARTUP-CONFIGS
    ModeDependentStartupConfigs                                            = 400,
    /// MODE-DRIVEN-FALSE-CONDITIONS
    ModeDrivenFalseConditions                                              = 6064,
    /// MODE-DRIVEN-TRANSMISSION-MODE-CONDITION
    ModeDrivenTransmissionModeCondition                                    = 2590,
    /// MODE-DRIVEN-TRUE-CONDITIONS
    ModeDrivenTrueConditions                                               = 3287,
    /// MODE-ELEMENT-REF
    ModeElementRef                                                         = 1493,
    /// MODE-ELEMENTS
    ModeElements                                                           = 4306,
    /// MODE-GROUP
    ModeGroup                                                              = 3961,
    /// MODE-GROUP-IREF
    ModeGroupIref                                                          = 5674,
    /// MODE-GROUP-REF
    ModeGroupRef                                                           = 6083,
    /// MODE-IN-BSW-INSTANCE-REF
    ModeInBswInstanceRef                                                   = 5341,
    /// MODE-IN-SWC-INSTANCE-REF
    ModeInSwcInstanceRef                                                   = 236,
    /// MODE-INSTANCE
    ModeInstance                                                           = 3812,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 4991,
    /// MODE-IREF
    ModeIref                                                               = 5425,
    /// MODE-IREFS
    ModeIrefs                                                              = 5647,
    /// MODE-MANAGER-ERROR-BEHAVIOR
    ModeManagerErrorBehavior                                               = 5686,
    /// MODE-MAPPING
    ModeMapping                                                            = 4261,
    /// MODE-PORT-ANNOTATION
    ModePortAnnotation                                                     = 2899,
    /// MODE-PORT-ANNOTATIONS
    ModePortAnnotations                                                    = 6087,
    /// MODE-RECEIVER-POLICYS
    ModeReceiverPolicys                                                    = 848,
    /// MODE-REF
    ModeRef                                                                = 1821,
    /// MODE-REPORTER-PORT-IREF
    ModeReporterPortIref                                                   = 2430,
    /// MODE-REQUEST-TYPE-MAP
    ModeRequestTypeMap                                                     = 5499,
    /// MODE-REQUEST-TYPE-MAPS
    ModeRequestTypeMaps                                                    = 4826,
    /// MODE-SENDER-POLICYS
    ModeSenderPolicys                                                      = 4294,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY
    ModeSwitchEventTriggeredActivity                                       = 1515,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS
    ModeSwitchEventTriggeredActivitys                                      = 3371,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 4482,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 6097,
    /// MODE-SWITCH-POINTS
    ModeSwitchPoints                                                       = 1662,
    /// MODE-SWITCH-RECEIVER-COM-SPEC
    ModeSwitchReceiverComSpec                                              = 1813,
    /// MODE-SWITCH-SENDER-COM-SPEC
    ModeSwitchSenderComSpec                                                = 5051,
    /// MODE-SWITCHED-ACK
    ModeSwitchedAck                                                        = 5399,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 4722,
    /// MODE-TRANSITION
    ModeTransition                                                         = 2076,
    /// MODE-TRANSITIONS
    ModeTransitions                                                        = 4962,
    /// MODE-USER-ERROR-BEHAVIOR
    ModeUserErrorBehavior                                                  = 2387,
    /// MODEL-OBJECT-REFERENCE
    ModelObjectReference                                                   = 1953,
    /// MODES
    Modes                                                                  = 4432,
    /// MODIFICATION
    Modification                                                           = 2796,
    /// MODIFICATIONS
    Modifications                                                          = 3773,
    /// MODIFIED-DATAS
    ModifiedDatas                                                          = 3644,
    /// MODULE-DESCRIPTION-REF
    ModuleDescriptionRef                                                   = 413,
    /// MODULE-ID
    ModuleId                                                               = 6352,
    /// MODULE-INSTANTIATION-REF
    ModuleInstantiationRef                                                 = 1801,
    /// MODULE-INSTANTIATION-REFS
    ModuleInstantiationRefs                                                = 1474,
    /// MODULE-INSTANTIATIONS
    ModuleInstantiations                                                   = 2108,
    /// MODULE-REF
    ModuleRef                                                              = 5408,
    /// MODULE-REFS
    ModuleRefs                                                             = 231,
    /// MOLAR-AMOUNT-EXP
    MolarAmountExp                                                         = 4353,
    /// MONITORED-IDENTIFIER-REF
    MonitoredIdentifierRef                                                 = 871,
    /// MONOTONY
    Monotony                                                               = 4491,
    /// MORE-FRAGMENTS
    MoreFragments                                                          = 2149,
    /// MSG-ID-MASK
    MsgIdMask                                                              = 3562,
    /// MSG-ID-MATCH
    MsgIdMatch                                                             = 856,
    /// MSR-QUERY-ARG
    MsrQueryArg                                                            = 4724,
    /// MSR-QUERY-CHAPTER
    MsrQueryChapter                                                        = 4493,
    /// MSR-QUERY-NAME
    MsrQueryName                                                           = 1114,
    /// MSR-QUERY-P-1
    MsrQueryP1                                                             = 4016,
    /// MSR-QUERY-P-2
    MsrQueryP2                                                             = 210,
    /// MSR-QUERY-PROPS
    MsrQueryProps                                                          = 2132,
    /// MSR-QUERY-RESULT-CHAPTER
    MsrQueryResultChapter                                                  = 5920,
    /// MSR-QUERY-RESULT-P-1
    MsrQueryResultP1                                                       = 4218,
    /// MSR-QUERY-RESULT-P-2
    MsrQueryResultP2                                                       = 1862,
    /// MSR-QUERY-RESULT-TOPIC-1
    MsrQueryResultTopic1                                                   = 4514,
    /// MSR-QUERY-TOPIC-1
    MsrQueryTopic1                                                         = 4168,
    /// MULTICAST-CONNECTOR-REF
    MulticastConnectorRef                                                  = 4115,
    /// MULTICAST-CONNECTOR-REFS
    MulticastConnectorRefs                                                 = 6414,
    /// MULTICAST-CREDENTIALS
    MulticastCredentials                                                   = 5115,
    /// MULTICAST-REF
    MulticastRef                                                           = 5388,
    /// MULTICAST-SD-IP-ADDRESS-REF
    MulticastSdIpAddressRef                                                = 3914,
    /// MULTICAST-SECURE-COM-PROPS-REF
    MulticastSecureComPropsRef                                             = 792,
    /// MULTICAST-SEGMENTATION
    MulticastSegmentation                                                  = 2976,
    /// MULTICAST-THRESHOLD
    MulticastThreshold                                                     = 3153,
    /// MULTICAST-UDP-PORT
    MulticastUdpPort                                                       = 4654,
    /// MULTIPLE-CONFIGURATION-CONTAINER
    MultipleConfigurationContainer                                         = 103,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 5100,
    /// MULTIPLICITY-CONFIG-CLASSES
    MultiplicityConfigClasses                                              = 2134,
    /// MULTIPLICITY-RESTRICTION
    MultiplicityRestriction                                                = 3479,
    /// N
    N                                                                      = 5238,
    /// N-DATA-SETS
    NDataSets                                                              = 3256,
    /// N-PDU
    NPdu                                                                   = 4855,
    /// N-PDU-REF
    NPduRef                                                                = 3161,
    /// N-PDU-REFS
    NPduRefs                                                               = 2342,
    /// N-ROM-BLOCKS
    NRomBlocks                                                             = 592,
    /// NAGLES-ALGORITHM
    NaglesAlgorithm                                                        = 393,
    /// NAME
    Name                                                                   = 334,
    /// NAME-TOKEN-VALUE-VARIATION-POINT
    NameTokenValueVariationPoint                                           = 4076,
    /// NAMED-CURVE-ID
    NamedCurveId                                                           = 5530,
    /// NAMESPACES
    Namespaces                                                             = 6034,
    /// NAS-TIMEOUT
    NasTimeout                                                             = 568,
    /// NATIVE-DECLARATION
    NativeDeclaration                                                      = 4519,
    /// NDP-PROPS
    NdpProps                                                               = 1119,
    /// NEEDS-ORIGINAL-DATA
    NeedsOriginalData                                                      = 1141,
    /// NEIGHBOR-CACHE-SIZE
    NeighborCacheSize                                                      = 4036,
    /// NESTED-CONTEXT-DATAS
    NestedContextDatas                                                     = 4887,
    /// NESTED-ELEMENT-REF
    NestedElementRef                                                       = 6376,
    /// NESTED-ELEMENT-REFS
    NestedElementRefs                                                      = 10,
    /// NESTED-ELEMENTS
    NestedElements                                                         = 37,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 598,
    /// NETWORK-CONFIGURATION-DESIGN-REF
    NetworkConfigurationDesignRef                                          = 4222,
    /// NETWORK-CONFIGURATION-REF
    NetworkConfigurationRef                                                = 4240,
    /// NETWORK-CONFIGURATION-REFS
    NetworkConfigurationRefs                                               = 3014,
    /// NETWORK-CONFIGURATIONS
    NetworkConfigurations                                                  = 5040,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 3013,
    /// NETWORK-ENDPOINT-ADDRESSES
    NetworkEndpointAddresses                                               = 3867,
    /// NETWORK-ENDPOINT-REF
    NetworkEndpointRef                                                     = 2828,
    /// NETWORK-ENDPOINT-REF-CONDITIONAL
    NetworkEndpointRefConditional                                          = 3535,
    /// NETWORK-ENDPOINT-REFS
    NetworkEndpointRefs                                                    = 44,
    /// NETWORK-ENDPOINTS
    NetworkEndpoints                                                       = 5769,
    /// NETWORK-HANDLE-PORT-MAPPING
    NetworkHandlePortMapping                                               = 3222,
    /// NETWORK-HANDLE-REF
    NetworkHandleRef                                                       = 3811,
    /// NETWORK-HANDLES
    NetworkHandles                                                         = 5281,
    /// NETWORK-ID
    NetworkId                                                              = 3308,
    /// NETWORK-IDLE-TIME
    NetworkIdleTime                                                        = 452,
    /// NETWORK-INTERFACE-ID
    NetworkInterfaceId                                                     = 814,
    /// NETWORK-INTERFACE-REF
    NetworkInterfaceRef                                                    = 5028,
    /// NETWORK-INTERFACE-REFS
    NetworkInterfaceRefs                                                   = 2944,
    /// NETWORK-INTERFACES
    NetworkInterfaces                                                      = 5180,
    /// NETWORK-LAYER-RULE
    NetworkLayerRule                                                       = 2102,
    /// NETWORK-MANAGEMENT-PORT-INTERFACE
    NetworkManagementPortInterface                                         = 6380,
    /// NETWORK-MANAGEMENT-VECTOR-LENGTH
    NetworkManagementVectorLength                                          = 6379,
    /// NETWORK-MASK
    NetworkMask                                                            = 4524,
    /// NETWORK-REPRESENTATION
    NetworkRepresentation                                                  = 4784,
    /// NETWORK-REPRESENTATION-PROPS
    NetworkRepresentationProps                                             = 3293,
    /// NETWORK-SEGMENT-ID
    NetworkSegmentId                                                       = 5099,
    /// NETWORK-TIME-CONSUMER-REF
    NetworkTimeConsumerRef                                                 = 4188,
    /// NETWORK-TIME-MASTER-REF
    NetworkTimeMasterRef                                                   = 2128,
    /// NETWORK-TIME-PROVIDER-REF
    NetworkTimeProviderRef                                                 = 5977,
    /// NETWORK-TIME-SLAVE-REF
    NetworkTimeSlaveRef                                                    = 5147,
    /// NEW-NAD
    NewNad                                                                 = 1378,
    /// NEXT-HEADER
    NextHeader                                                             = 1305,
    /// NEXT-HIGHER-CERTIFICATE-REF
    NextHigherCertificateRef                                               = 1441,
    /// NEXT-STATE-IREF
    NextStateIref                                                          = 2369,
    /// NM-ACTIVE-COORDINATOR
    NmActiveCoordinator                                                    = 4177,
    /// NM-BUS-LOAD-REDUCTION-ENABLED
    NmBusLoadReductionEnabled                                              = 3871,
    /// NM-BUS-SYNCHRONIZATION-ENABLED
    NmBusSynchronizationEnabled                                            = 3042,
    /// NM-BUSLOAD-REDUCTION-ACTIVE
    NmBusloadReductionActive                                               = 6328,
    /// NM-BUSLOAD-REDUCTION-ENABLED
    NmBusloadReductionEnabled                                              = 1415,
    /// NM-CAR-WAKE-UP-BIT-POSITION
    NmCarWakeUpBitPosition                                                 = 787,
    /// NM-CAR-WAKE-UP-FILTER-ENABLED
    NmCarWakeUpFilterEnabled                                               = 2050,
    /// NM-CAR-WAKE-UP-FILTER-NODE-ID
    NmCarWakeUpFilterNodeId                                                = 5297,
    /// NM-CAR-WAKE-UP-RX-ENABLED
    NmCarWakeUpRxEnabled                                                   = 5195,
    /// NM-CBV-POSITION
    NmCbvPosition                                                          = 59,
    /// NM-CHANNEL-ACTIVE
    NmChannelActive                                                        = 5727,
    /// NM-CHANNEL-ID
    NmChannelId                                                            = 4512,
    /// NM-CHANNEL-SLEEP-MASTER
    NmChannelSleepMaster                                                   = 4358,
    /// NM-CLUSTER-COUPLINGS
    NmClusterCouplings                                                     = 3437,
    /// NM-CLUSTERS
    NmClusters                                                             = 78,
    /// NM-COM-CONTROL-ENABLED
    NmComControlEnabled                                                    = 1941,
    /// NM-CONFIG
    NmConfig                                                               = 4249,
    /// NM-CONTROL-BIT-VECTOR-ACTIVE
    NmControlBitVectorActive                                               = 5,
    /// NM-CONTROL-BIT-VECTOR-ENABLED
    NmControlBitVectorEnabled                                              = 279,
    /// NM-COORD-CLUSTER
    NmCoordCluster                                                         = 1893,
    /// NM-COORD-SYNC-SUPPORT
    NmCoordSyncSupport                                                     = 3276,
    /// NM-COORDINATOR
    NmCoordinator                                                          = 4649,
    /// NM-COORDINATOR-ROLE
    NmCoordinatorRole                                                      = 5276,
    /// NM-CYCLETIME-MAIN-FUNCTION
    NmCycletimeMainFunction                                                = 4770,
    /// NM-DATA-CYCLE
    NmDataCycle                                                            = 5020,
    /// NM-DATA-DISABLED
    NmDataDisabled                                                         = 573,
    /// NM-DATA-ENABLED
    NmDataEnabled                                                          = 1713,
    /// NM-DATA-INFORMATION
    NmDataInformation                                                      = 3631,
    /// NM-DETECTION-LOCK
    NmDetectionLock                                                        = 5292,
    /// NM-ECU
    NmEcu                                                                  = 2401,
    /// NM-GLOBAL-COORDINATOR-TIME
    NmGlobalCoordinatorTime                                                = 2636,
    /// NM-HANDLE-REF
    NmHandleRef                                                            = 4965,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 2596,
    /// NM-HW-VOTE-ENABLED
    NmHwVoteEnabled                                                        = 1698,
    /// NM-IF-ECU-REF
    NmIfEcuRef                                                             = 6068,
    /// NM-IF-ECUS
    NmIfEcus                                                               = 2059,
    /// NM-IMMEDIATE-NM-CYCLE-TIME
    NmImmediateNmCycleTime                                                 = 5602,
    /// NM-IMMEDIATE-NM-TRANSMISSIONS
    NmImmediateNmTransmissions                                             = 1521,
    /// NM-IMMEDIATE-RESTART-ENABLED
    NmImmediateRestartEnabled                                              = 5042,
    /// NM-INSTANCE-ID
    NmInstanceId                                                           = 5659,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 970,
    /// NM-INTERACTS-WITH-SM-MAPPING
    NmInteractsWithSmMapping                                               = 2172,
    /// NM-LIGHT-TIMEOUT
    NmLightTimeout                                                         = 5601,
    /// NM-MAIN-FUNCTION-ACROSS-FR-CYCLE
    NmMainFunctionAcrossFrCycle                                            = 563,
    /// NM-MAIN-FUNCTION-PERIOD
    NmMainFunctionPeriod                                                   = 2700,
    /// NM-MESSAGE-TIMEOUT-TIME
    NmMessageTimeoutTime                                                   = 2771,
    /// NM-MSG-CYCLE-OFFSET
    NmMsgCycleOffset                                                       = 471,
    /// NM-MSG-CYCLE-TIME
    NmMsgCycleTime                                                         = 536,
    /// NM-MSG-REDUCED-TIME
    NmMsgReducedTime                                                       = 2616,
    /// NM-MULTIPLE-CHANNELS-ENABLED
    NmMultipleChannelsEnabled                                              = 4684,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 359,
    /// NM-NETWORK-HANDLE-REF
    NmNetworkHandleRef                                                     = 1092,
    /// NM-NETWORK-TIMEOUT
    NmNetworkTimeout                                                       = 5940,
    /// NM-NID-POSITION
    NmNidPosition                                                          = 3982,
    /// NM-NODE-DETECTION-ENABLED
    NmNodeDetectionEnabled                                                 = 6050,
    /// NM-NODE-ID
    NmNodeId                                                               = 2999,
    /// NM-NODE-ID-ENABLED
    NmNodeIdEnabled                                                        = 6388,
    /// NM-NODE-REF
    NmNodeRef                                                              = 3531,
    /// NM-NODE-REFS
    NmNodeRefs                                                             = 2918,
    /// NM-NODES
    NmNodes                                                                = 1911,
    /// NM-PASSIVE-MODE-ENABLED
    NmPassiveModeEnabled                                                   = 3650,
    /// NM-PDU
    NmPdu                                                                  = 1645,
    /// NM-PDU-REF
    NmPduRef                                                               = 528,
    /// NM-PDU-REF-CONDITIONAL
    NmPduRefConditional                                                    = 2074,
    /// NM-PDU-RX-INDICATION-ENABLED
    NmPduRxIndicationEnabled                                               = 508,
    /// NM-PDUS
    NmPdus                                                                 = 1693,
    /// NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS
    NmPnHandleMultipleNetworkRequests                                      = 4450,
    /// NM-PNC-PARTICIPATION
    NmPncParticipation                                                     = 6033,
    /// NM-RANGE-CONFIG
    NmRangeConfig                                                          = 2755,
    /// NM-READY-SLEEP-COUNT
    NmReadySleepCount                                                      = 1780,
    /// NM-READY-SLEEP-TIME
    NmReadySleepTime                                                       = 5359,
    /// NM-REMOTE-SLEEP-IND-ENABLED
    NmRemoteSleepIndEnabled                                                = 2258,
    /// NM-REMOTE-SLEEP-INDICATION-TIME
    NmRemoteSleepIndicationTime                                            = 6372,
    /// NM-REPEAT-MESSAGE-BIT-ACTIVE
    NmRepeatMessageBitActive                                               = 52,
    /// NM-REPEAT-MESSAGE-BIT-ENABLE
    NmRepeatMessageBitEnable                                               = 141,
    /// NM-REPEAT-MESSAGE-TIME
    NmRepeatMessageTime                                                    = 1399,
    /// NM-REPEAT-MSG-IND-ENABLED
    NmRepeatMsgIndEnabled                                                  = 6103,
    /// NM-REPEAT-MSG-INDICATION-ENABLED
    NmRepeatMsgIndicationEnabled                                           = 4084,
    /// NM-REPETITION-CYCLE
    NmRepetitionCycle                                                      = 3696,
    /// NM-SCHEDULE-VARIANT
    NmScheduleVariant                                                      = 6216,
    /// NM-SHUTDOWN-DELAY-TIMER
    NmShutdownDelayTimer                                                   = 5098,
    /// NM-STATE-CHANGE-IND-ENABLED
    NmStateChangeIndEnabled                                                = 1449,
    /// NM-STATE-REQUEST
    NmStateRequest                                                         = 1848,
    /// NM-SYNCHRONIZATION-POINT-ENABLED
    NmSynchronizationPointEnabled                                          = 2018,
    /// NM-SYNCHRONIZING-NETWORK
    NmSynchronizingNetwork                                                 = 6229,
    /// NM-USER-DATA-ENABLED
    NmUserDataEnabled                                                      = 910,
    /// NM-USER-DATA-LENGTH
    NmUserDataLength                                                       = 2111,
    /// NM-USER-DATA-OFFSET
    NmUserDataOffset                                                       = 3164,
    /// NM-VARIANT
    NmVariant                                                              = 5363,
    /// NM-VECTOR-EARLY-UPDATE
    NmVectorEarlyUpdate                                                    = 6011,
    /// NM-VOTE-INFORMATION
    NmVoteInformation                                                      = 3052,
    /// NM-VOTING-CYCLE
    NmVotingCycle                                                          = 4131,
    /// NM-WAIT-BUS-SLEEP-TIME
    NmWaitBusSleepTime                                                     = 4355,
    /// NO-CHECKPOINT-SUPERVISION
    NoCheckpointSupervision                                                = 5380,
    /// NO-CHECKPOINT-SUPERVISIONS
    NoCheckpointSupervisions                                               = 111,
    /// NO-SUPERVISION
    NoSupervision                                                          = 5669,
    /// NO-SUPERVISIONS
    NoSupervisions                                                         = 3443,
    /// NODE-IDENTIFIER
    NodeIdentifier                                                         = 4097,
    /// NODE-NAME
    NodeName                                                               = 4842,
    /// NODE-PORTS
    NodePorts                                                              = 757,
    /// NODE-REF
    NodeRef                                                                = 3186,
    /// NOMINAL
    Nominal                                                                = 522,
    /// NOMINAL-EXECUTION-TIME
    NominalExecutionTime                                                   = 1572,
    /// NON-OS-MODULE-INSTANTIATION-REF
    NonOsModuleInstantiationRef                                            = 1834,
    /// NON-VERBOSE-MODE
    NonVerboseMode                                                         = 922,
    /// NONQUEUED-RECEIVER-COM-SPEC
    NonqueuedReceiverComSpec                                               = 6023,
    /// NONQUEUED-SENDER-COM-SPEC
    NonqueuedSenderComSpec                                                 = 2860,
    /// NOT-AVAILABLE-VALUE-SPECIFICATION
    NotAvailableValueSpecification                                         = 2911,
    /// NOTATION
    Notation                                                               = 4120,
    /// NOTE
    Note                                                                   = 1721,
    /// NOTIFICATION-PORT-IREF
    NotificationPortIref                                                   = 5591,
    /// NOTIFICATION-STATE
    NotificationState                                                      = 1992,
    /// NOTIFICATION-TIME
    NotificationTime                                                       = 5441,
    /// NOTIFICATIONS
    Notifications                                                          = 5660,
    /// NOTIFIED-STATUSS
    NotifiedStatuss                                                        = 4090,
    /// NOTIFIER
    Notifier                                                               = 5761,
    /// NOTIFIER-DATA-ELEMENT-REF
    NotifierDataElementRef                                                 = 1535,
    /// NOTIFIER-REF
    NotifierRef                                                            = 5194,
    /// NOTIFIER-SIGNAL-TRIGGERING-REF
    NotifierSignalTriggeringRef                                            = 6052,
    /// NRC-VALUE
    NrcValue                                                               = 243,
    /// NTU
    Ntu                                                                    = 4200,
    /// NUM-FAILED-SECURITY-ACCESS
    NumFailedSecurityAccess                                                = 4865,
    /// NUMBER
    Number                                                                 = 5684,
    /// NUMBER-OF-BITS
    NumberOfBits                                                           = 318,
    /// NUMBER-OF-CONNECTED-CLIENTS
    NumberOfConnectedClients                                               = 6051,
    /// NUMBER-OF-INSTRUCTIONS
    NumberOfInstructions                                                   = 737,
    /// NUMBER-OF-MINISLOTS
    NumberOfMinislots                                                      = 5589,
    /// NUMBER-OF-NOTIFIER-SETS
    NumberOfNotifierSets                                                   = 3465,
    /// NUMBER-OF-PARALLEL-TCP-SESSIONS
    NumberOfParallelTcpSessions                                            = 1162,
    /// NUMBER-OF-REPETITIONS
    NumberOfRepetitions                                                    = 448,
    /// NUMBER-OF-RESTART-ATTEMPTS
    NumberOfRestartAttempts                                                = 5556,
    /// NUMBER-OF-STATIC-SLOTS
    NumberOfStaticSlots                                                    = 4578,
    /// NUMBER-OF-WORKERS
    NumberOfWorkers                                                        = 3089,
    /// NUMERICAL-RULE-BASED-VALUE-SPECIFICATION
    NumericalRuleBasedValueSpecification                                   = 3400,
    /// NUMERICAL-VALUE
    NumericalValue                                                         = 534,
    /// NUMERICAL-VALUE-SPECIFICATION
    NumericalValueSpecification                                            = 4151,
    /// NUMERICAL-VALUE-VARIATION-POINT
    NumericalValueVariationPoint                                           = 3086,
    /// NV-BLOCK-DATA-MAPPING
    NvBlockDataMapping                                                     = 6143,
    /// NV-BLOCK-DATA-MAPPINGS
    NvBlockDataMappings                                                    = 4670,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 1786,
    /// NV-BLOCK-DESCRIPTORS
    NvBlockDescriptors                                                     = 1991,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 290,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 410,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 3764,
    /// NV-DATA-PORT-ANNOTATION
    NvDataPortAnnotation                                                   = 240,
    /// NV-DATA-PORT-ANNOTATIONS
    NvDataPortAnnotations                                                  = 1554,
    /// NV-DATAS
    NvDatas                                                                = 2483,
    /// NV-PROVIDE-COM-SPEC
    NvProvideComSpec                                                       = 1099,
    /// NV-RAM-BLOCK-ELEMENT
    NvRamBlockElement                                                      = 2894,
    /// NV-REQUIRE-COM-SPEC
    NvRequireComSpec                                                       = 2561,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 3798,
    /// OBD-DTC-NUMBER
    ObdDtcNumber                                                           = 6148,
    /// OBD-DTC-VALUE
    ObdDtcValue                                                            = 278,
    /// OBD-DTC-VALUE-3-BYTE
    ObdDtcValue3Byte                                                       = 5422,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 2943,
    /// OBD-MID
    ObdMid                                                                 = 994,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 884,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 1181,
    /// OBD-PROPSS
    ObdPropss                                                              = 5189,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 3347,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 4657,
    /// OBD-RELEVANCE
    ObdRelevance                                                           = 1464,
    /// OBD-SUPPORT
    ObdSupport                                                             = 5319,
    /// OBJECT-DEFINITION-REF
    ObjectDefinitionRef                                                    = 1808,
    /// OBJECT-DEFINITION-REFS
    ObjectDefinitionRefs                                                   = 3963,
    /// OBJECT-DEFINTION-REF
    ObjectDefintionRef                                                     = 2434,
    /// OBJECT-DEFINTION-REFS
    ObjectDefintionRefs                                                    = 4405,
    /// OBJECT-IREF
    ObjectIref                                                             = 2294,
    /// OBJECT-REF
    ObjectRef                                                              = 2576,
    /// OBJECT-REFS
    ObjectRefs                                                             = 4190,
    /// OCCURRENCE-COUNTER-PROCESSING
    OccurrenceCounterProcessing                                            = 6297,
    /// OCCURRENCE-EXPRESSION
    OccurrenceExpression                                                   = 3785,
    /// OFFER-CYCLIC-DELAY
    OfferCyclicDelay                                                       = 2814,
    /// OFFSET
    Offset                                                                 = 5409,
    /// OFFSET-CORRECTION-ADAPTION-INTERVAL
    OffsetCorrectionAdaptionInterval                                       = 5836,
    /// OFFSET-CORRECTION-JUMP-THRESHOLD
    OffsetCorrectionJumpThreshold                                          = 4287,
    /// OFFSET-CORRECTION-OUT
    OffsetCorrectionOut                                                    = 3882,
    /// OFFSET-CORRECTION-START
    OffsetCorrectionStart                                                  = 1787,
    /// OFFSET-SI-TO-UNIT
    OffsetSiToUnit                                                         = 5275,
    /// OFFSET-TIME-DOMAIN-REF
    OffsetTimeDomainRef                                                    = 2145,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 6188,
    /// OFFSETS
    Offsets                                                                = 6201,
    /// OFNS-DATA-ID-LIST
    OfnsDataIdList                                                         = 5765,
    /// OFNS-DATA-ID-LISTS
    OfnsDataIdLists                                                        = 3169,
    /// OFS-DATA-ID-LIST
    OfsDataIdList                                                          = 541,
    /// OFS-DATA-ID-LISTS
    OfsDataIdLists                                                         = 3622,
    /// OFS-SUB-TLV
    OfsSubTlv                                                              = 3959,
    /// ON-BOARD-MONITOR-ID
    OnBoardMonitorId                                                       = 1701,
    /// ON-CHANGE-DATA-PROTOTYPES
    OnChangeDataPrototypes                                                 = 5746,
    /// ON-FAIL-PERMISSIVE-MODE
    OnFailPermissiveMode                                                   = 2263,
    /// ON-FAIL-PERMISSIVE-MODE-TIMEOUT
    OnFailPermissiveModeTimeout                                            = 3739,
    /// ON-TRANSITION-VALUE
    OnTransitionValue                                                      = 2187,
    /// ONE-EVERY-N
    OneEveryN                                                              = 4536,
    /// OP
    Op                                                                     = 5865,
    /// OPERATION-ARGUMENT-INSTANCE-IREF
    OperationArgumentInstanceIref                                          = 4560,
    /// OPERATION-CYCLE
    OperationCycle                                                         = 303,
    /// OPERATION-CYCLE-AUTOMATIC-END
    OperationCycleAutomaticEnd                                             = 5926,
    /// OPERATION-CYCLE-AUTOSTART
    OperationCycleAutostart                                                = 1189,
    /// OPERATION-CYCLE-REF
    OperationCycleRef                                                      = 4343,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 4328,
    /// OPERATION-IREF
    OperationIref                                                          = 2904,
    /// OPERATION-IREFS
    OperationIrefs                                                         = 1763,
    /// OPERATION-MAPPINGS
    OperationMappings                                                      = 5326,
    /// OPERATION-MODE
    OperationMode                                                          = 5706,
    /// OPERATION-PROTOTYPE-IREF
    OperationPrototypeIref                                                 = 3537,
    /// OPERATION-REF
    OperationRef                                                           = 3033,
    /// OPERATIONS
    Operations                                                             = 4128,
    /// OPERATOR
    Operator                                                               = 3660,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF
    OptAppRecordElementInMethodIref                                        = 958,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS
    OptAppRecordElementInMethodIrefs                                       = 4520,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    OptAppRecordElementInServiceInterfaceIref                              = 1158,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS
    OptAppRecordElementInServiceInterfaceIrefs                             = 4280,
    /// OPT-ARGUMENT-REF
    OptArgumentRef                                                         = 122,
    /// OPT-ARGUMENT-REFS
    OptArgumentRefs                                                        = 1831,
    /// OPT-IMPL-RECORD-ELEMENT
    OptImplRecordElement                                                   = 5537,
    /// OPT-IMPL-RECORD-ELEMENTS
    OptImplRecordElements                                                  = 3073,
    /// OPTION
    Option                                                                 = 1925,
    /// OPTION-ARGUMENT
    OptionArgument                                                         = 2927,
    /// OPTION-KIND
    OptionKind                                                             = 4404,
    /// OPTION-NAME
    OptionName                                                             = 3525,
    /// OPTIONAL
    Optional                                                               = 6318,
    /// OPTIONAL-ELEMENTS
    OptionalElements                                                       = 4778,
    /// OPTIONS
    Options                                                                = 943,
    /// ORDERED-ELEMENTS
    OrderedElements                                                        = 3021,
    /// ORDERED-MASTER
    OrderedMaster                                                          = 1340,
    /// ORDERED-MASTER-LIST
    OrderedMasterList                                                      = 1696,
    /// ORIGIN
    Origin                                                                 = 4910,
    /// OS-ARTI-ADAPTER-LAUNCH-BEHAVIOR
    OsArtiAdapterLaunchBehavior                                            = 501,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 4095,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 2310,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 4214,
    /// OS-TASK-PROXY-REF
    OsTaskProxyRef                                                         = 4460,
    /// OUT-MEASUREMENT-SET
    OutMeasurementSet                                                      = 1886,
    /// OUT-MEASURMENT-SET
    OutMeasurmentSet                                                       = 6039,
    /// OUTER-PORT-REF
    OuterPortRef                                                           = 3354,
    /// OUTER-PORTS
    OuterPorts                                                             = 2666,
    /// OUTGOING-CALLBACKS
    OutgoingCallbacks                                                      = 1341,
    /// OVERRIDE-ID
    OverrideId                                                             = 2870,
    /// OVERRIDE-INITIAL-STATE-IREF
    OverrideInitialStateIref                                               = 5555,
    /// OWNER-REF
    OwnerRef                                                               = 3907,
    /// OWNER-REFS
    OwnerRefs                                                              = 599,
    /// OWNERSHIP
    Ownership                                                              = 5858,
    /// OWNERSHIP-KIND
    OwnershipKind                                                          = 1836,
    /// OWNERSHIP-STRENGTH
    OwnershipStrength                                                      = 6135,
    /// P
    P                                                                      = 961,
    /// P-2-MAX
    P2Max                                                                  = 2417,
    /// P-2-SERVER-MAX
    P2ServerMax                                                            = 6370,
    /// P-2-STAR-SERVER-MAX
    P2StarServerMax                                                        = 2223,
    /// P-2-TIMING
    P2Timing                                                               = 1237,
    /// P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    PModeGroupInAtomicSwcInstanceRef                                       = 1048,
    /// P-PORT-IN-COMPOSITION-INSTANCE-REF
    PPortInCompositionInstanceRef                                          = 1697,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 1526,
    /// P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    PPortPrototypeInExecutableIref                                         = 1530,
    /// P-PORT-PROTOTYPE-IREF
    PPortPrototypeIref                                                     = 2810,
    /// P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF
    PTriggerInAtomicSwcTypeInstanceRef                                     = 2473,
    /// PACKAGE-REF
    PackageRef                                                             = 700,
    /// PACKAGER-ID
    PackagerId                                                             = 4223,
    /// PACKAGER-SIGNATURE-REF
    PackagerSignatureRef                                                   = 3229,
    /// PACKAGING-PIN-NAME
    PackagingPinName                                                       = 3146,
    /// PACKING-BYTE-ORDER
    PackingByteOrder                                                       = 417,
    /// PADDING-ACTIVATION
    PaddingActivation                                                      = 1109,
    /// PADDING-VALUE
    PaddingValue                                                           = 4731,
    /// PARAMETER
    Parameter                                                              = 3261,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 4429,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REF
    ParameterAccessedForDebugRef                                           = 2733,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REFS
    ParameterAccessedForDebugRefs                                          = 1259,
    /// PARAMETER-ACCESSS
    ParameterAccesss                                                       = 3703,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 5759,
    /// PARAMETER-DATA-PROTOTYPE-IREF
    ParameterDataPrototypeIref                                             = 1352,
    /// PARAMETER-ELEMENT-ACCESS
    ParameterElementAccess                                                 = 2566,
    /// PARAMETER-ID
    ParameterId                                                            = 526,
    /// PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT
    ParameterImplementationDataTypeElement                                 = 5594,
    /// PARAMETER-INSTANCE
    ParameterInstance                                                      = 4728,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 5512,
    /// PARAMETER-POLICYS
    ParameterPolicys                                                       = 6263,
    /// PARAMETER-PORT-ANNOTATION
    ParameterPortAnnotation                                                = 6003,
    /// PARAMETER-PORT-ANNOTATIONS
    ParameterPortAnnotations                                               = 352,
    /// PARAMETER-PROVIDE-COM-SPEC
    ParameterProvideComSpec                                                = 5620,
    /// PARAMETER-REF
    ParameterRef                                                           = 2094,
    /// PARAMETER-REQUIRE-COM-SPEC
    ParameterRequireComSpec                                                = 5578,
    /// PARAMETER-SIZE
    ParameterSize                                                          = 6281,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 4850,
    /// PARAMETER-VALUES
    ParameterValues                                                        = 3772,
    /// PARAMETERS
    Parameters                                                             = 2577,
    /// PARENT-CATEGORY
    ParentCategory                                                         = 4771,
    /// PARENT-SHORT-LABEL
    ParentShortLabel                                                       = 4183,
    /// PARTIAL-NETWORK-REF
    PartialNetworkRef                                                      = 1385,
    /// PARTIAL-NETWORK-REFS
    PartialNetworkRefs                                                     = 4804,
    /// PARTICIPATING-J-1939-CLUSTER-REF
    ParticipatingJ1939ClusterRef                                           = 2296,
    /// PARTICIPATING-J-1939-CLUSTER-REFS
    ParticipatingJ1939ClusterRefs                                          = 932,
    /// PARTITION-NAME
    PartitionName                                                          = 1794,
    /// PARTITION-REF
    PartitionRef                                                           = 2085,
    /// PARTITIONS
    Partitions                                                             = 3231,
    /// PARTS
    Parts                                                                  = 3304,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 31,
    /// PATH
    Path                                                                   = 293,
    /// PATH-MTU-DISCOVERY-ENABLED
    PathMtuDiscoveryEnabled                                                = 4142,
    /// PATH-MTU-ENABLED
    PathMtuEnabled                                                         = 133,
    /// PATH-MTU-TIMEOUT
    PathMtuTimeout                                                         = 3255,
    /// PATTERN
    Pattern                                                                = 6158,
    /// PATTERN-JITTER
    PatternJitter                                                          = 4256,
    /// PATTERN-LENGTH
    PatternLength                                                          = 1066,
    /// PATTERN-PERIOD
    PatternPeriod                                                          = 4988,
    /// PAYLOAD-BYTE-PATTERN-RULE
    PayloadBytePatternRule                                                 = 2586,
    /// PAYLOAD-BYTE-PATTERN-RULE-PART
    PayloadBytePatternRulePart                                             = 2714,
    /// PAYLOAD-BYTE-PATTERN-RULE-PARTS
    PayloadBytePatternRuleParts                                            = 1602,
    /// PAYLOAD-BYTE-PATTERN-RULES
    PayloadBytePatternRules                                                = 4167,
    /// PAYLOAD-LENGTH
    PayloadLength                                                          = 6404,
    /// PAYLOAD-LENGTH-STATIC
    PayloadLengthStatic                                                    = 4747,
    /// PAYLOAD-PREAMBLE-INDICATOR
    PayloadPreambleIndicator                                               = 3160,
    /// PAYLOAD-REF
    PayloadRef                                                             = 5222,
    /// PAYLOAD-TYPE
    PayloadType                                                            = 4862,
    /// PAYLOADS
    Payloads                                                               = 79,
    /// PCM-BIT-DEPTH
    PcmBitDepth                                                            = 893,
    /// PDELAY-LATENCY-THRESHOLD
    PdelayLatencyThreshold                                                 = 1016,
    /// PDELAY-REQUEST-PERIOD
    PdelayRequestPeriod                                                    = 2592,
    /// PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT
    PdelayRespAndRespFollowUpTimeout                                       = 837,
    /// PDELAY-RESPONSE-ENABLED
    PdelayResponseEnabled                                                  = 2917,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 86,
    /// PDU-ACTIVATION-ROUTING-GROUPS
    PduActivationRoutingGroups                                             = 6170,
    /// PDU-COLLECTION-MAX-BUFFER-SIZE
    PduCollectionMaxBufferSize                                             = 2763,
    /// PDU-COLLECTION-PDU-TIMEOUT
    PduCollectionPduTimeout                                                = 2354,
    /// PDU-COLLECTION-SEMANTICS
    PduCollectionSemantics                                                 = 1641,
    /// PDU-COLLECTION-TIMEOUT
    PduCollectionTimeout                                                   = 5729,
    /// PDU-COLLECTION-TRIGGER
    PduCollectionTrigger                                                   = 2655,
    /// PDU-COUNTER-SIZE
    PduCounterSize                                                         = 4106,
    /// PDU-COUNTER-START-POSITION
    PduCounterStartPosition                                                = 1022,
    /// PDU-COUNTER-THRESHOLD
    PduCounterThreshold                                                    = 1455,
    /// PDU-COUNTERS
    PduCounters                                                            = 1475,
    /// PDU-MAX-LENGTH
    PduMaxLength                                                           = 5676,
    /// PDU-POOLS
    PduPools                                                               = 4160,
    /// PDU-REF
    PduRef                                                                 = 735,
    /// PDU-REPLICATION-VOTING
    PduReplicationVoting                                                   = 1575,
    /// PDU-REPLICATIONS
    PduReplications                                                        = 2651,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 5000,
    /// PDU-TO-FRAME-MAPPINGS
    PduToFrameMappings                                                     = 5539,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 4093,
    /// PDU-TRIGGERING-REF
    PduTriggeringRef                                                       = 5352,
    /// PDU-TRIGGERING-REF-CONDITIONAL
    PduTriggeringRefConditional                                            = 6431,
    /// PDU-TRIGGERING-REFS
    PduTriggeringRefs                                                      = 1937,
    /// PDU-TRIGGERINGS
    PduTriggerings                                                         = 3030,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 1627,
    /// PDUR-TP-CHUNK-SIZE
    PdurTpChunkSize                                                        = 5648,
    /// PDUS
    Pdus                                                                   = 3906,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 1818,
    /// PER-INSTANCE-MEMORY-REF
    PerInstanceMemoryRef                                                   = 5369,
    /// PER-INSTANCE-MEMORY-SIZE
    PerInstanceMemorySize                                                  = 2024,
    /// PER-INSTANCE-MEMORY-SIZES
    PerInstanceMemorySizes                                                 = 6251,
    /// PER-INSTANCE-MEMORYS
    PerInstanceMemorys                                                     = 5351,
    /// PER-INSTANCE-PARAMETER-REF
    PerInstanceParameterRef                                                = 682,
    /// PER-INSTANCE-PARAMETERS
    PerInstanceParameters                                                  = 1867,
    /// PER-STATE-TIMEOUT
    PerStateTimeout                                                        = 6134,
    /// PER-STATE-TIMEOUTS
    PerStateTimeouts                                                       = 3825,
    /// PERIOD
    Period                                                                 = 5607,
    /// PERIOD-BEGIN
    PeriodBegin                                                            = 3464,
    /// PERIOD-END
    PeriodEnd                                                              = 2652,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 4653,
    /// PERIODIC-RATE-CATEGORY
    PeriodicRateCategory                                                   = 1690,
    /// PERIODIC-RATES
    PeriodicRates                                                          = 6353,
    /// PERIODIC-RESPONSE-UUDT-REF
    PeriodicResponseUudtRef                                                = 2684,
    /// PERIODIC-RESPONSE-UUDT-REFS
    PeriodicResponseUudtRefs                                               = 2697,
    /// PERMISSIBLE-SIGNAL-PATH
    PermissibleSignalPath                                                  = 5120,
    /// PERMISSION-CERTIFICATE-AUTHORITY-REF
    PermissionCertificateAuthorityRef                                      = 3564,
    /// PERMISSION-TO-CREATE-CHILD-PROCESS
    PermissionToCreateChildProcess                                         = 6449,
    /// PERMIT-MULTIPLE-REFERENCES-TO-EE
    PermitMultipleReferencesToEe                                           = 4986,
    /// PERSISTENCY-ACCESS
    PersistencyAccess                                                      = 5362,
    /// PERSISTENCY-CENTRAL-STORAGE-URI
    PersistencyCentralStorageUri                                           = 5688,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 4121,
    /// PERSISTENCY-DATA-PROVIDED-COM-SPEC
    PersistencyDataProvidedComSpec                                         = 4900,
    /// PERSISTENCY-DATA-REQUIRED-COM-SPEC
    PersistencyDataRequiredComSpec                                         = 6219,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-REF
    PersistencyDeploymentElementRef                                        = 1601,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 5721,
    /// PERSISTENCY-DEPLOYMENT-REF
    PersistencyDeploymentRef                                               = 3200,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 804,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 3454,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 845,
    /// PERSISTENCY-DEPLOYMENT-URI
    PersistencyDeploymentUri                                               = 5513,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 4739,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 3427,
    /// PERSISTENCY-FILE-ARRAY-REF
    PersistencyFileArrayRef                                                = 3502,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 2333,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 1174,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 2009,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 5970,
    /// PERSISTENCY-FILE-REQUIRED-COM-SPEC
    PersistencyFileRequiredComSpec                                         = 1890,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 5153,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 3213,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 5959,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 3435,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 5743,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 4179,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 4020,
    /// PERSISTENCY-PORT-PROTOTYPE-IREF
    PersistencyPortPrototypeIref                                           = 2975,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 3069,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 3569,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 5129,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 1747,
    /// PERSISTENCY-PROVIDED-COM-SPEC
    PersistencyProvidedComSpec                                             = 5892,
    /// PERSISTENCY-REDUNDANCY-CRC
    PersistencyRedundancyCrc                                               = 1059,
    /// PERSISTENCY-REDUNDANCY-HASH
    PersistencyRedundancyHash                                              = 165,
    /// PERSISTENCY-REDUNDANCY-M-OUT-OF-N
    PersistencyRedundancyMOutOfN                                           = 1413,
    /// PERSISTENT-STORAGE
    PersistentStorage                                                      = 1014,
    /// PGN
    Pgn                                                                    = 4563,
    /// PHM-ACTION
    PhmAction                                                              = 432,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 6161,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 1883,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 4255,
    /// PHM-CHECKPOINT-IREF
    PhmCheckpointIref                                                      = 3184,
    /// PHM-CONTRIBUTION-REF
    PhmContributionRef                                                     = 4885,
    /// PHM-CONTRIBUTION-REFS
    PhmContributionRefs                                                    = 1434,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 4691,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 5256,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 776,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 4668,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 3909,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 4735,
    /// PHM-RULE
    PhmRule                                                                = 564,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 4155,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 2100,
    /// PHYS-CONSTRS
    PhysConstrs                                                            = 2901,
    /// PHYSICAL-CHANNEL-REF
    PhysicalChannelRef                                                     = 4427,
    /// PHYSICAL-CHANNEL-REF-CONDITIONAL
    PhysicalChannelRefConditional                                          = 6108,
    /// PHYSICAL-CHANNEL-REFS
    PhysicalChannelRefs                                                    = 4811,
    /// PHYSICAL-CHANNELS
    PhysicalChannels                                                       = 1330,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 1948,
    /// PHYSICAL-DIMENSION-MAPPING
    PhysicalDimensionMapping                                               = 6285,
    /// PHYSICAL-DIMENSION-MAPPING-REF
    PhysicalDimensionMappingRef                                            = 2465,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 5861,
    /// PHYSICAL-DIMENSION-MAPPINGS
    PhysicalDimensionMappings                                              = 3908,
    /// PHYSICAL-DIMENSION-REF
    PhysicalDimensionRef                                                   = 5378,
    /// PHYSICAL-LAYER-TYPE
    PhysicalLayerType                                                      = 3813,
    /// PHYSICAL-PROPS
    PhysicalProps                                                          = 1926,
    /// PHYSICAL-REQUEST-REF
    PhysicalRequestRef                                                     = 548,
    /// PID
    Pid                                                                    = 4892,
    /// PID-REF
    PidRef                                                                 = 705,
    /// PID-REFS
    PidRefs                                                                = 2545,
    /// PID-SIZE
    PidSize                                                                = 4366,
    /// PIN-NUMBER
    PinNumber                                                              = 5089,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 1741,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 63,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 801,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 1414,
    /// PLATFORM-TIME-BASES
    PlatformTimeBases                                                      = 4791,
    /// PLCA-LOCAL-NODE-COUNT
    PlcaLocalNodeCount                                                     = 6200,
    /// PLCA-LOCAL-NODE-ID
    PlcaLocalNodeId                                                        = 4216,
    /// PLCA-MAX-BURST-COUNT
    PlcaMaxBurstCount                                                      = 2215,
    /// PLCA-MAX-BURST-TIMER
    PlcaMaxBurstTimer                                                      = 2726,
    /// PLCA-PROPS
    PlcaProps                                                              = 2647,
    /// PLCA-TRANSMIT-OPPORTUNITY-TIMER
    PlcaTransmitOpportunityTimer                                           = 2887,
    /// PN-RESET-TIME
    PnResetTime                                                            = 690,
    /// PN-RESET-TIMER
    PnResetTimer                                                           = 2028,
    /// PNC-CLUSTER-VECTOR-LENGTH
    PncClusterVectorLength                                                 = 4417,
    /// PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    PncConsumedProvidedServiceInstanceGroups                               = 5093,
    /// PNC-FILTER-ARRAY-MASK
    PncFilterArrayMask                                                     = 104,
    /// PNC-FILTER-ARRAY-MASKS
    PncFilterArrayMasks                                                    = 3387,
    /// PNC-FILTER-DATA-MASK
    PncFilterDataMask                                                      = 3357,
    /// PNC-GATEWAY-TYPE
    PncGatewayType                                                         = 3697,
    /// PNC-GROUP-REF
    PncGroupRef                                                            = 4203,
    /// PNC-GROUP-REFS
    PncGroupRefs                                                           = 668,
    /// PNC-IDENTIFIER
    PncIdentifier                                                          = 6078,
    /// PNC-MAPPING
    PncMapping                                                             = 2240,
    /// PNC-MAPPING-REF
    PncMappingRef                                                          = 1361,
    /// PNC-MAPPING-REFS
    PncMappingRefs                                                         = 3760,
    /// PNC-MAPPINGS
    PncMappings                                                            = 1565,
    /// PNC-NM-REQUEST
    PncNmRequest                                                           = 2578,
    /// PNC-PDUR-GROUP-REF
    PncPdurGroupRef                                                        = 1357,
    /// PNC-PDUR-GROUP-REFS
    PncPdurGroupRefs                                                       = 1118,
    /// PNC-PREPARE-SLEEP-TIMER
    PncPrepareSleepTimer                                                   = 368,
    /// PNC-SYNCHRONOUS-WAKEUP
    PncSynchronousWakeup                                                   = 4727,
    /// PNC-VECTOR-LENGTH
    PncVectorLength                                                        = 3840,
    /// PNC-VECTOR-OFFSET
    PncVectorOffset                                                        = 2717,
    /// PNC-WAKEUP-CAN-ID
    PncWakeupCanId                                                         = 4248,
    /// PNC-WAKEUP-CAN-ID-EXTENDED
    PncWakeupCanIdExtended                                                 = 3403,
    /// PNC-WAKEUP-CAN-ID-MASK
    PncWakeupCanIdMask                                                     = 5531,
    /// PNC-WAKEUP-DATA-MASK
    PncWakeupDataMask                                                      = 1522,
    /// PNC-WAKEUP-DLC
    PncWakeupDlc                                                           = 5271,
    /// PNC-WAKEUP-ENABLE
    PncWakeupEnable                                                        = 6348,
    /// POLICY
    Policy                                                                 = 4756,
    /// POLICY-ACTION
    PolicyAction                                                           = 3365,
    /// PORT-ADDRESS
    PortAddress                                                            = 1711,
    /// PORT-API-OPTION
    PortApiOption                                                          = 3315,
    /// PORT-API-OPTIONS
    PortApiOptions                                                         = 2419,
    /// PORT-ARG-VALUES
    PortArgValues                                                          = 4413,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 4703,
    /// PORT-BLUEPRINT-INIT-VALUE
    PortBlueprintInitValue                                                 = 4114,
    /// PORT-BLUEPRINT-MAPPING
    PortBlueprintMapping                                                   = 3642,
    /// PORT-BLUEPRINT-MAPPINGS
    PortBlueprintMappings                                                  = 3157,
    /// PORT-BLUEPRINT-REF
    PortBlueprintRef                                                       = 6307,
    /// PORT-CONFIGS
    PortConfigs                                                            = 4974,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINT
    PortDefinedArgumentBlueprint                                           = 2574,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINTS
    PortDefinedArgumentBlueprints                                          = 1744,
    /// PORT-DEFINED-ARGUMENT-VALUE
    PortDefinedArgumentValue                                               = 6255,
    /// PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS
    PortElementToComResourceMappings                                       = 2801,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 3109,
    /// PORT-GROUP
    PortGroup                                                              = 5182,
    /// PORT-GROUPS
    PortGroups                                                             = 3572,
    /// PORT-INSTANTIATION-BEHAVIOR
    PortInstantiationBehavior                                              = 2938,
    /// PORT-INTERFACE-BLUEPRINT-MAPPING
    PortInterfaceBlueprintMapping                                          = 460,
    /// PORT-INTERFACE-BLUEPRINT-REF
    PortInterfaceBlueprintRef                                              = 427,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 1157,
    /// PORT-INTERFACE-MAPPINGS
    PortInterfaceMappings                                                  = 5221,
    /// PORT-INTERFACE-REF
    PortInterfaceRef                                                       = 135,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 6106,
    /// PORT-NUMBER
    PortNumber                                                             = 5841,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 4425,
    /// PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE
    PortPrototypeBlueprintInitValue                                        = 4713,
    /// PORT-PROTOTYPE-BLUEPRINT-MAPPING
    PortPrototypeBlueprintMapping                                          = 1164,
    /// PORT-PROTOTYPE-BLUEPRINT-REF
    PortPrototypeBlueprintRef                                              = 2559,
    /// PORT-PROTOTYPE-IREF
    PortPrototypeIref                                                      = 1456,
    /// PORT-PROTOTYPE-PROPS
    PortPrototypeProps                                                     = 6191,
    /// PORT-PROTOTYPE-REF
    PortPrototypeRef                                                       = 5035,
    /// PORT-PROTOTYPE-REF-CONDITIONAL
    PortPrototypeRefConditional                                            = 3623,
    /// PORT-PROTOTYPE-SLUG-FRAGMENT
    PortPrototypeSlugFragment                                              = 4245,
    /// PORT-REF
    PortRef                                                                = 2756,
    /// PORT-SCHEDULER
    PortScheduler                                                          = 3133,
    /// PORTS
    Ports                                                                  = 6163,
    /// POSITION
    Position                                                               = 4781,
    /// POSITION-IN-TABLE
    PositionInTable                                                        = 6293,
    /// POSITIVE-INTEGER-VALUE-VARIATION-POINT
    PositiveIntegerValueVariationPoint                                     = 1101,
    /// POSSIBLE-AP-ERROR-REF
    PossibleApErrorRef                                                     = 3024,
    /// POSSIBLE-AP-ERROR-REFS
    PossibleApErrorRefs                                                    = 355,
    /// POSSIBLE-AP-ERROR-SET-REF
    PossibleApErrorSetRef                                                  = 4816,
    /// POSSIBLE-AP-ERROR-SET-REFS
    PossibleApErrorSetRefs                                                 = 2169,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 4990,
    /// POSSIBLE-ERROR-REACTIONS
    PossibleErrorReactions                                                 = 172,
    /// POSSIBLE-ERROR-REF
    PossibleErrorRef                                                       = 1427,
    /// POSSIBLE-ERROR-REFS
    PossibleErrorRefs                                                      = 6242,
    /// POSSIBLE-ERRORS
    PossibleErrors                                                         = 2150,
    /// POST-BUILD-CHANGEABLE
    PostBuildChangeable                                                    = 697,
    /// POST-BUILD-VALUE-ACCESS-REF
    PostBuildValueAccessRef                                                = 3558,
    /// POST-BUILD-VARIANT-CONDITION
    PostBuildVariantCondition                                              = 3295,
    /// POST-BUILD-VARIANT-CONDITIONS
    PostBuildVariantConditions                                             = 6018,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 433,
    /// POST-BUILD-VARIANT-CRITERION-VALUE
    PostBuildVariantCriterionValue                                         = 627,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 2068,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF
    PostBuildVariantCriterionValueSetRef                                   = 6230,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS
    PostBuildVariantCriterionValueSetRefs                                  = 1133,
    /// POST-BUILD-VARIANT-CRITERION-VALUES
    PostBuildVariantCriterionValues                                        = 3299,
    /// POST-BUILD-VARIANT-MULTIPLICITY
    PostBuildVariantMultiplicity                                           = 3844,
    /// POST-BUILD-VARIANT-SUPPORT
    PostBuildVariantSupport                                                = 2743,
    /// POST-BUILD-VARIANT-USED
    PostBuildVariantUsed                                                   = 4692,
    /// POST-BUILD-VARIANT-VALUE
    PostBuildVariantValue                                                  = 4023,
    /// POST-VERIFICATION-REBOOT
    PostVerificationReboot                                                 = 5391,
    /// POSTS
    Posts                                                                  = 3149,
    /// POWER-DOWN-TIME
    PowerDownTime                                                          = 3144,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 5538,
    /// PRE-ACTIVATE-IREF
    PreActivateIref                                                        = 3482,
    /// PRE-ACTIVATE-IREFS
    PreActivateIrefs                                                       = 186,
    /// PRE-ACTIVATE-REF
    PreActivateRef                                                         = 3578,
    /// PRE-ACTIVATE-REFS
    PreActivateRefs                                                        = 4178,
    /// PRE-ACTIVATION-REBOOT
    PreActivationReboot                                                    = 2531,
    /// PRE-MAPPING
    PreMapping                                                             = 6159,
    /// PRE-SHARED-KEY-REF
    PreSharedKeyRef                                                        = 1467,
    /// PRECONFIGURED-CONFIGURATION-REF
    PreconfiguredConfigurationRef                                          = 30,
    /// PRECONFIGURED-CONFIGURATION-REFS
    PreconfiguredConfigurationRefs                                         = 4447,
    /// PREDECESSOR-ACTION-REF
    PredecessorActionRef                                                   = 3707,
    /// PREDECESSOR-ACTION-REFS
    PredecessorActionRefs                                                  = 2940,
    /// PREDECESSOR-FIFO-REF
    PredecessorFifoRef                                                     = 2556,
    /// PREDECESSOR-REF
    PredecessorRef                                                         = 1920,
    /// PREDECESSOR-REFS
    PredecessorRefs                                                        = 2584,
    /// PREDEFINED-TEXT
    PredefinedText                                                         = 1488,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 1830,
    /// PREEMPTABILITY
    Preemptability                                                         = 5112,
    /// PREFIX
    Prefix                                                                 = 1571,
    /// PREFIX-REF
    PrefixRef                                                              = 4134,
    /// PREPARE-ROLLBACK
    PrepareRollback                                                        = 4377,
    /// PREPARE-UPDATE
    PrepareUpdate                                                          = 5951,
    /// PRESTORAGE-FREEZE-FRAME
    PrestorageFreezeFrame                                                  = 1779,
    /// PRESTORED-FREEZEFRAME-STORED-IN-NVM
    PrestoredFreezeframeStoredInNvm                                        = 1914,
    /// PREVIOUS-CONTRACT-VERSION
    PreviousContractVersion                                                = 4475,
    /// PREVIOUS-DATA-TYPE-REF
    PreviousDataTypeRef                                                    = 5446,
    /// PREVIOUS-EXECUTABLE-VERSION
    PreviousExecutableVersion                                              = 1321,
    /// PRIMITIVE
    Primitive                                                              = 4393,
    /// PRIMITIVE-ATTRIBUTE-CONDITION
    PrimitiveAttributeCondition                                            = 5013,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 993,
    /// PRIMITIVE-FAMILY
    PrimitiveFamily                                                        = 3510,
    /// PRIMITIVE-TYPE-MAPPINGS
    PrimitiveTypeMappings                                                  = 4686,
    /// PRIORITY
    Priority                                                               = 3346,
    /// PRIORITY-ID
    PriorityId                                                             = 1768,
    /// PRIORITYS
    Prioritys                                                              = 2591,
    /// PRIVACY-LEVEL
    PrivacyLevel                                                           = 6401,
    /// PRM
    Prm                                                                    = 1569,
    /// PRM-CHAR
    PrmChar                                                                = 2019,
    /// PRM-UNIT
    PrmUnit                                                                = 3645,
    /// PRMS
    Prms                                                                   = 1727,
    /// PROCESS
    Process                                                                = 6391,
    /// PROCESS-ARGUMENT
    ProcessArgument                                                        = 2343,
    /// PROCESS-ARGUMENTS
    ProcessArguments                                                       = 5554,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 5915,
    /// PROCESS-DESIGN-REF
    ProcessDesignRef                                                       = 5565,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 5057,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 4402,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS
    ProcessDesignToMachineDesignMappings                                   = 6037,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 5030,
    /// PROCESS-MODE-IREF
    ProcessModeIref                                                        = 1503,
    /// PROCESS-MODE-MACHINE
    ProcessModeMachine                                                     = 4600,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 3111,
    /// PROCESS-REF
    ProcessRef                                                             = 3725,
    /// PROCESS-STATE-IREF
    ProcessStateIref                                                       = 628,
    /// PROCESS-STATE-MACHINE
    ProcessStateMachine                                                    = 4688,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 742,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 1012,
    /// PROCESS-TO-MACHINE-MAPPINGS
    ProcessToMachineMappings                                               = 1077,
    /// PROCESSING-KIND
    ProcessingKind                                                         = 2868,
    /// PROCESSING-STYLE
    ProcessingStyle                                                        = 180,
    /// PROCESSING-UNIT-REF
    ProcessingUnitRef                                                      = 1649,
    /// PROCESSOR
    Processor                                                              = 404,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 2659,
    /// PROCESSOR-MODE
    ProcessorMode                                                          = 5355,
    /// PROCESSOR-SPEED
    ProcessorSpeed                                                         = 4505,
    /// PROCESSORS
    Processors                                                             = 4768,
    /// PRODUCT-ID
    ProductId                                                              = 3737,
    /// PRODUCT-RELEASE
    ProductRelease                                                         = 5116,
    /// PROFILE-BEHAVIOR
    ProfileBehavior                                                        = 5854,
    /// PROFILE-NAME
    ProfileName                                                            = 5957,
    /// PROG-CODE
    ProgCode                                                               = 4976,
    /// PROGRAMMING-LANGUAGE
    ProgrammingLanguage                                                    = 2990,
    /// PROP-SEG
    PropSeg                                                                = 693,
    /// PROPABILITY
    Propability                                                            = 4896,
    /// PROPAGATION-DELAY
    PropagationDelay                                                       = 5837,
    /// PROPERTYS
    Propertys                                                              = 811,
    /// PROPS
    Props                                                                  = 327,
    /// PROTOCOL
    Protocol                                                               = 1295,
    /// PROTOCOL-KIND
    ProtocolKind                                                           = 3548,
    /// PROTOCOL-NAME
    ProtocolName                                                           = 2598,
    /// PROTOCOL-VERSION
    ProtocolVersion                                                        = 5776,
    /// PROVIDE-RESOURCES
    ProvideResources                                                       = 1126,
    /// PROVIDED-CLIENT-SERVER-ENTRYS
    ProvidedClientServerEntrys                                             = 966,
    /// PROVIDED-COM-SPECS
    ProvidedComSpecs                                                       = 2428,
    /// PROVIDED-DATA-REF
    ProvidedDataRef                                                        = 273,
    /// PROVIDED-DATAS
    ProvidedDatas                                                          = 551,
    /// PROVIDED-DDS-EVENT-QOS-PROPS
    ProvidedDdsEventQosProps                                               = 2570,
    /// PROVIDED-DDS-OPERATIONS
    ProvidedDdsOperations                                                  = 2841,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 4298,
    /// PROVIDED-DDS-SERVICE-INSTANCE-EVENTS
    ProvidedDdsServiceInstanceEvents                                       = 5383,
    /// PROVIDED-ENTRYS
    ProvidedEntrys                                                         = 4235,
    /// PROVIDED-EVENT-GROUPS
    ProvidedEventGroups                                                    = 3699,
    /// PROVIDED-INTERFACE-TREF
    ProvidedInterfaceTref                                                  = 2060,
    /// PROVIDED-MEMORY-REF
    ProvidedMemoryRef                                                      = 3964,
    /// PROVIDED-MODE-GROUP-REF
    ProvidedModeGroupRef                                                   = 5793,
    /// PROVIDED-MODE-GROUPS
    ProvidedModeGroups                                                     = 237,
    /// PROVIDED-OUTER-PORT-REF
    ProvidedOuterPortRef                                                   = 3536,
    /// PROVIDED-PORT-PROTOTYPE-IREF
    ProvidedPortPrototypeIref                                              = 159,
    /// PROVIDED-REQUIRED-INTERFACE-TREF
    ProvidedRequiredInterfaceTref                                          = 1843,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 4040,
    /// PROVIDED-SERVICE-INSTANCE-REF
    ProvidedServiceInstanceRef                                             = 3521,
    /// PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL
    ProvidedServiceInstanceRefConditional                                  = 223,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 1876,
    /// PROVIDED-SERVICE-INSTANCES
    ProvidedServiceInstances                                               = 1966,
    /// PROVIDED-SERVICE-PORT-IREF
    ProvidedServicePortIref                                                = 5645,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 5681,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 5678,
    /// PROVIDER-IREF
    ProviderIref                                                           = 3933,
    /// PROVIDER-RATE-DEVIATION-MAX
    ProviderRateDeviationMax                                               = 6178,
    /// PROVIDER-REF
    ProviderRef                                                            = 675,
    /// PROVIDER-SOFTWARE-CLUSTER-REF
    ProviderSoftwareClusterRef                                             = 1382,
    /// PROVIEDE-DATA-REF
    ProviedeDataRef                                                        = 5237,
    /// PSK-IDENTITY
    PskIdentity                                                            = 4995,
    /// PSK-IDENTITY-HINT
    PskIdentityHint                                                        = 6060,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 312,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS
    PskIdentityToKeySlotMappings                                           = 5283,
    /// PUBLISHER
    Publisher                                                              = 4481,
    /// PULSE-TEST
    PulseTest                                                              = 2867,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 2690,
    /// PURPOSE-OF-UPDATE-REF
    PurposeOfUpdateRef                                                     = 2663,
    /// PUTS
    Puts                                                                   = 1000,
    /// PWM-L
    PwmL                                                                   = 263,
    /// PWM-O
    PwmO                                                                   = 5177,
    /// PWM-S
    PwmS                                                                   = 3066,
    /// QOS-PROFILE
    QosProfile                                                             = 3913,
    /// QUEUE-LENGTH
    QueueLength                                                            = 5105,
    /// QUEUE-SIZE
    QueueSize                                                              = 2672,
    /// QUEUED-RECEIVER-COM-SPEC
    QueuedReceiverComSpec                                                  = 3083,
    /// QUEUED-SENDER-COM-SPEC
    QueuedSenderComSpec                                                    = 1877,
    /// R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    RModeGroupInAtomicSwcInstanceRef                                       = 5962,
    /// R-PORT-IN-COMPOSITION-INSTANCE-REF
    RPortInCompositionInstanceRef                                          = 4696,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 2960,
    /// R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    RPortPrototypeInExecutableIref                                         = 2959,
    /// R-PORT-PROTOTYPE-IREF
    RPortPrototypeIref                                                     = 4029,
    /// R-PORT-PROTOTYPE-PROPS
    RPortPrototypeProps                                                    = 5404,
    /// R-PORT-PROTOTYPE-REF
    RPortPrototypeRef                                                      = 1213,
    /// R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF
    RTriggerInAtomicSwcInstanceRef                                         = 3053,
    /// RAM-BLOCK
    RamBlock                                                               = 4067,
    /// RAM-BLOCK-INIT-VALUE
    RamBlockInitValue                                                      = 325,
    /// RAM-BLOCK-STATUS-CONTROL
    RamBlockStatusControl                                                  = 5736,
    /// RAM-LOCATION-REF
    RamLocationRef                                                         = 4818,
    /// RANGE-MAX
    RangeMax                                                               = 4348,
    /// RANGE-MIN
    RangeMin                                                               = 5773,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 1453,
    /// RATE-BASED-MONITORED-EVENT-REF
    RateBasedMonitoredEventRef                                             = 3432,
    /// RATE-CORRECTION-MEASUREMENT-DURATION
    RateCorrectionMeasurementDuration                                      = 2066,
    /// RATE-CORRECTION-OUT
    RateCorrectionOut                                                      = 2300,
    /// RATE-CORRECTIONS-PER-MEASUREMENT-DURATION
    RateCorrectionsPerMeasurementDuration                                  = 4035,
    /// RATE-DEVIATION-MEASUREMENT-DURATION
    RateDeviationMeasurementDuration                                       = 3568,
    /// RATE-LIMITATION-FILTERS
    RateLimitationFilters                                                  = 4262,
    /// RATE-POLICYS
    RatePolicys                                                            = 2234,
    /// RATIO-KIND
    RatioKind                                                              = 882,
    /// RATIONALE
    Rationale                                                              = 1384,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 1699,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 4439,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 2955,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 2995,
    /// RAW-DATA-STREAM-INTERFACE-REF
    RawDataStreamInterfaceRef                                              = 6325,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 2070,
    /// RAW-DATA-STREAM-REF
    RawDataStreamRef                                                       = 3489,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 4935,
    /// REACTION-CODE
    ReactionCode                                                           = 2101,
    /// READ
    Read                                                                   = 6413,
    /// READ-CLASS-REF
    ReadClassRef                                                           = 4281,
    /// READ-DATA-CLASS-REF
    ReadDataClassRef                                                       = 3515,
    /// READ-DTC-INFORMATION-CLASS-REF
    ReadDtcInformationClassRef                                             = 4323,
    /// READ-LOCAL-VARIABLES
    ReadLocalVariables                                                     = 4136,
    /// READ-NV-DATA
    ReadNvData                                                             = 5758,
    /// READ-SCALING-DATA-CLASS-REF
    ReadScalingDataClassRef                                                = 6199,
    /// READER-ENTITY-ID
    ReaderEntityId                                                         = 3472,
    /// READONLY
    Readonly                                                               = 5370,
    /// REASON
    Reason                                                                 = 2516,
    /// RECEIVE-ACTIVITY
    ReceiveActivity                                                        = 5118,
    /// RECEIVE-WINDOW-MIN
    ReceiveWindowMin                                                       = 5787,
    /// RECEIVED-DATA-REF
    ReceivedDataRef                                                        = 1078,
    /// RECEIVER-ANNOTATION
    ReceiverAnnotation                                                     = 2158,
    /// RECEIVER-CAPABILITY
    ReceiverCapability                                                     = 212,
    /// RECEIVER-INTENT
    ReceiverIntent                                                         = 5716,
    /// RECEIVER-IREF
    ReceiverIref                                                           = 1447,
    /// RECEIVER-IREFS
    ReceiverIrefs                                                          = 3571,
    /// RECEIVER-REF
    ReceiverRef                                                            = 624,
    /// RECEIVER-REFS
    ReceiverRefs                                                           = 2608,
    /// RECEPTION-DEFAULT-VALUES
    ReceptionDefaultValues                                                 = 2915,
    /// RECEPTION-POLICYS
    ReceptionPolicys                                                       = 2830,
    /// RECEPTION-PROPS
    ReceptionProps                                                         = 4766,
    /// RECOMMENDED-CONFIGURATION-REF
    RecommendedConfigurationRef                                            = 3702,
    /// RECOMMENDED-CONFIGURATION-REFS
    RecommendedConfigurationRefs                                           = 2044,
    /// RECORD-ELEMENT-MAPPINGS
    RecordElementMappings                                                  = 1730,
    /// RECORD-ELEMENT-REF
    RecordElementRef                                                       = 2194,
    /// RECORD-ELEMENTS
    RecordElements                                                         = 2585,
    /// RECORD-NUMBER
    RecordNumber                                                           = 1188,
    /// RECORD-VALUE-SPECIFICATION
    RecordValueSpecification                                               = 123,
    /// RECOVERABLE-IN-SAME-OPERATION-CYCLE
    RecoverableInSameOperationCycle                                        = 238,
    /// RECOVERY
    Recovery                                                               = 2956,
    /// RECOVERY-ACTION-IREF
    RecoveryActionIref                                                     = 5445,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 6167,
    /// RECOVERY-NOTIFICATION-REF
    RecoveryNotificationRef                                                = 4492,
    /// RECOVERY-NOTIFICATION-REFS
    RecoveryNotificationRefs                                               = 807,
    /// RECOVERY-NOTIFICATION-RETRY
    RecoveryNotificationRetry                                              = 6208,
    /// RECOVERY-NOTIFICATION-TIMEOUT
    RecoveryNotificationTimeout                                            = 6231,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 32,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 4101,
    /// RECOVERY-VIA-APPLICATION-ACTION-REF
    RecoveryViaApplicationActionRef                                        = 6071,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 637,
    /// REDUNDANCY
    Redundancy                                                             = 5802,
    /// REDUNDANCY-HANDLINGS
    RedundancyHandlings                                                    = 3663,
    /// REENTRANCY-LEVEL
    ReentrancyLevel                                                        = 5257,
    /// REF
    Ref                                                                    = 4211,
    /// REF-CALPRM-SET
    RefCalprmSet                                                           = 4398,
    /// REF-MEASUREMENT-SET
    RefMeasurementSet                                                      = 2352,
    /// REFERENCE-BASE
    ReferenceBase                                                          = 1971,
    /// REFERENCE-BASES
    ReferenceBases                                                         = 3607,
    /// REFERENCE-CONDITION
    ReferenceCondition                                                     = 1397,
    /// REFERENCE-REF
    ReferenceRef                                                           = 4213,
    /// REFERENCE-TABLE-REF
    ReferenceTableRef                                                      = 6181,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 440,
    /// REFERENCE-VALUE-REF
    ReferenceValueRef                                                      = 3527,
    /// REFERENCE-VALUE-SPECIFICATION
    ReferenceValueSpecification                                            = 2969,
    /// REFERENCE-VALUES
    ReferenceValues                                                        = 6340,
    /// REFERENCED-BASELINE
    ReferencedBaseline                                                     = 2349,
    /// REFERENCED-TD-EVENT-SWC-REF
    ReferencedTdEventSwcRef                                                = 646,
    /// REFERENCED-TD-EVENT-VFB-REF
    ReferencedTdEventVfbRef                                                = 1898,
    /// REFERENCES
    References                                                             = 5507,
    /// REFERRABLE-REF
    ReferrableRef                                                          = 4506,
    /// REFILL-AMOUNT
    RefillAmount                                                           = 2153,
    /// REFINED-EVENT-IREF
    RefinedEventIref                                                       = 927,
    /// REFINED-MODULE-DEF-REF
    RefinedModuleDefRef                                                    = 3239,
    /// REG-DOES-NOT-REQUIRE-STABILITYS
    RegDoesNotRequireStabilitys                                            = 1324,
    /// REG-REQUIRES-STABILITYS
    RegRequiresStabilitys                                                  = 2770,
    /// REGENERATED-PRIORITY
    RegeneratedPriority                                                    = 4525,
    /// REGULAR-EXPRESSION
    RegularExpression                                                      = 5173,
    /// REKEY-INTERVAL
    RekeyInterval                                                          = 3165,
    /// RELATED-TRACE-ITEM-REF
    RelatedTraceItemRef                                                    = 2639,
    /// RELATIONS
    Relations                                                              = 6233,
    /// RELATIVE
    Relative                                                               = 4760,
    /// RELATIVE-REPRESENTATION-TIME
    RelativeRepresentationTime                                             = 3063,
    /// RELATIVE-TOLERANCE
    RelativeTolerance                                                      = 1240,
    /// RELEASE-NOTES-REF
    ReleaseNotesRef                                                        = 3744,
    /// RELEASED-TRIGGER-POLICYS
    ReleasedTriggerPolicys                                                 = 5847,
    /// RELEASED-TRIGGER-REF
    ReleasedTriggerRef                                                     = 2480,
    /// RELEASED-TRIGGERS
    ReleasedTriggers                                                       = 1274,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF
    RelevantForDynamicPncMappingRef                                        = 5745,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS
    RelevantForDynamicPncMappingRefs                                       = 6247,
    /// RELIABILITY
    Reliability                                                            = 2742,
    /// RELIABILITY-KIND
    ReliabilityKind                                                        = 6030,
    /// RELIABILITY-MAX-BLOCKING-TIME
    ReliabilityMaxBlockingTime                                             = 2502,
    /// REMAPPED-CAN-ID
    RemappedCanId                                                          = 4349,
    /// REMARK
    Remark                                                                 = 331,
    /// REMOTE-ACCESS-CONTROL-ENABLED
    RemoteAccessControlEnabled                                             = 633,
    /// REMOTE-ADDRESSS
    RemoteAddresss                                                         = 5154,
    /// REMOTE-CERTIFICATE-REF
    RemoteCertificateRef                                                   = 1435,
    /// REMOTE-CERTIFICATE-REFS
    RemoteCertificateRefs                                                  = 5072,
    /// REMOTE-CLIENT-CONFIG
    RemoteClientConfig                                                     = 1373,
    /// REMOTE-CONFIGS
    RemoteConfigs                                                          = 2861,
    /// REMOTE-ENDPOINT-CONFIGURATION
    RemoteEndpointConfiguration                                            = 3214,
    /// REMOTE-ID
    RemoteId                                                               = 3855,
    /// REMOTE-IP-ADDRESS-REF
    RemoteIpAddressRef                                                     = 4750,
    /// REMOTE-IP-ADDRESS-REFS
    RemoteIpAddressRefs                                                    = 6203,
    /// REMOTE-MULTICAST-CONFIG-REF
    RemoteMulticastConfigRef                                               = 5165,
    /// REMOTE-MULTICAST-CONFIG-REFS
    RemoteMulticastConfigRefs                                              = 3103,
    /// REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS
    RemoteMulticastSubscriptionAddresss                                    = 5229,
    /// REMOTE-NETWORK-ENDPOINT-REF
    RemoteNetworkEndpointRef                                               = 1589,
    /// REMOTE-PORT-RANGE-END
    RemotePortRangeEnd                                                     = 4744,
    /// REMOTE-PORT-RANGE-START
    RemotePortRangeStart                                                   = 2320,
    /// REMOTE-PORT-REF
    RemotePortRef                                                          = 662,
    /// REMOTE-SERVER-CONFIG
    RemoteServerConfig                                                     = 1350,
    /// REMOTE-SUBJECT-REF
    RemoteSubjectRef                                                       = 3087,
    /// REMOTE-SUBJECT-REFS
    RemoteSubjectRefs                                                      = 2373,
    /// REMOTE-UNICAST-ADDRESSS
    RemoteUnicastAddresss                                                  = 322,
    /// REMOTE-UNICAST-CONFIG-REF
    RemoteUnicastConfigRef                                                 = 5519,
    /// REMOTE-UNICAST-CONFIG-REFS
    RemoteUnicastConfigRefs                                                = 2069,
    /// REMOTING-TECHNOLOGY
    RemotingTechnology                                                     = 4104,
    /// REPETITION-PERIOD
    RepetitionPeriod                                                       = 3185,
    /// REPLACE-WITH
    ReplaceWith                                                            = 6332,
    /// REPLAY-PROTECTION
    ReplayProtection                                                       = 5697,
    /// REPLAY-PROTECTION-WINDOW
    ReplayProtectionWindow                                                 = 1505,
    /// REPLICA-PDUS-REF
    ReplicaPdusRef                                                         = 47,
    /// REPLICA-PDUS-REFS
    ReplicaPdusRefs                                                        = 907,
    /// REPLY-TOPIC-NAME
    ReplyTopicName                                                         = 5663,
    /// REPORT-BEHAVIOR
    ReportBehavior                                                         = 5168,
    /// REPORTABLE-SECURITY-EVENT-REF
    ReportableSecurityEventRef                                             = 1034,
    /// REPORTABLE-SECURITY-EVENT-REFS
    ReportableSecurityEventRefs                                            = 5412,
    /// REPORTED-SECURITY-EVENT-IREF
    ReportedSecurityEventIref                                              = 5880,
    /// REPORTING-BEHAVIOR
    ReportingBehavior                                                      = 1597,
    /// REPORTING-PORT-PROTOTYPE-IREF
    ReportingPortPrototypeIref                                             = 297,
    /// REPOSITORY
    Repository                                                             = 4697,
    /// REPRESENTED-MODEL-ELEMENT-REF
    RepresentedModelElementRef                                             = 4031,
    /// REPRESENTED-PORT-GROUP-REF
    RepresentedPortGroupRef                                                = 5254,
    /// REPRESENTS-VIN
    RepresentsVin                                                          = 1457,
    /// REQUEST-2-SUPPORT
    Request2Support                                                        = 2625,
    /// REQUEST-CONFIGURATION-DESIGNS
    RequestConfigurationDesigns                                            = 3821,
    /// REQUEST-CONFIGURATIONS
    RequestConfigurations                                                  = 2958,
    /// REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF
    RequestControlOfOnBoardDeviceClassRef                                  = 5356,
    /// REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF
    RequestCurrentPowertrainDiagnosticDataClassRef                         = 4175,
    /// REQUEST-DATA-SIZE
    RequestDataSize                                                        = 6278,
    /// REQUEST-DOWNLOAD-CLASS-REF
    RequestDownloadClassRef                                                = 5453,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF
    RequestEmissionRelatedDtcClassPermanentStatusRef                       = 3113,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-REF
    RequestEmissionRelatedDtcClassRef                                      = 582,
    /// REQUEST-FILE-TRANSFER-CLASS-REF
    RequestFileTransferClassRef                                            = 5771,
    /// REQUEST-GROUP-REF
    RequestGroupRef                                                        = 880,
    /// REQUEST-METHOD
    RequestMethod                                                          = 3762,
    /// REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF
    RequestOnBoardMonitoringTestResultsClassRef                            = 431,
    /// REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF
    RequestPowertrainFreezeFrameDataRef                                    = 6338,
    /// REQUEST-RESPONSE-DELAY
    RequestResponseDelay                                                   = 3836,
    /// REQUEST-RESULT
    RequestResult                                                          = 794,
    /// REQUEST-SEED-ID
    RequestSeedId                                                          = 213,
    /// REQUEST-TOPIC-NAME
    RequestTopicName                                                       = 2833,
    /// REQUEST-TYPE
    RequestType                                                            = 2589,
    /// REQUEST-UPLOAD-CLASS-REF
    RequestUploadClassRef                                                  = 4331,
    /// REQUEST-VEHICLE-INFORMATION-CLASS-REF
    RequestVehicleInformationClassRef                                      = 3714,
    /// REQUESTABLE
    Requestable                                                            = 4607,
    /// REQUESTED-MODE-IREF
    RequestedModeIref                                                      = 1036,
    /// REQUESTER-IREF
    RequesterIref                                                          = 1823,
    /// REQUESTER-REF
    RequesterRef                                                           = 6100,
    /// REQUESTER-REFS
    RequesterRefs                                                          = 1303,
    /// REQUESTER-SOFTWARE-CLUSTER-REF
    RequesterSoftwareClusterRef                                            = 1134,
    /// REQUESTOR-REF
    RequestorRef                                                           = 5656,
    /// REQUESTOR-REFS
    RequestorRefs                                                          = 5003,
    /// REQUESTS
    Requests                                                               = 5760,
    /// REQUIRE-RESOURCES
    RequireResources                                                       = 6456,
    /// REQUIRED-AR-ELEMENT-REF
    RequiredArElementRef                                                   = 6232,
    /// REQUIRED-AR-ELEMENT-REFS
    RequiredArElementRefs                                                  = 3154,
    /// REQUIRED-ARTIFACTS
    RequiredArtifacts                                                      = 1580,
    /// REQUIRED-CLIENT-SERVER-ENTRY-REF
    RequiredClientServerEntryRef                                           = 3302,
    /// REQUIRED-CLIENT-SERVER-ENTRYS
    RequiredClientServerEntrys                                             = 649,
    /// REQUIRED-COM-SPECS
    RequiredComSpecs                                                       = 3652,
    /// REQUIRED-DATAS
    RequiredDatas                                                          = 3358,
    /// REQUIRED-DDS-EVENT-QOS-PROPS
    RequiredDdsEventQosProps                                               = 5910,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 1720,
    /// REQUIRED-DESIGN-ELEMENT-REF
    RequiredDesignElementRef                                               = 1761,
    /// REQUIRED-DESIGN-ELEMENT-REFS
    RequiredDesignElementRefs                                              = 2165,
    /// REQUIRED-ENTRYS
    RequiredEntrys                                                         = 2235,
    /// REQUIRED-ENVIRONMENT-REF
    RequiredEnvironmentRef                                                 = 2396,
    /// REQUIRED-EVENT-GROUPS
    RequiredEventGroups                                                    = 655,
    /// REQUIRED-FIBEX-ELEMENT-REF
    RequiredFibexElementRef                                                = 2177,
    /// REQUIRED-FIBEX-ELEMENT-REFS
    RequiredFibexElementRefs                                               = 3125,
    /// REQUIRED-GENERATOR-TOOLS
    RequiredGeneratorTools                                                 = 1547,
    /// REQUIRED-INTERFACE-TREF
    RequiredInterfaceTref                                                  = 4440,
    /// REQUIRED-MINOR-VERSION
    RequiredMinorVersion                                                   = 4141,
    /// REQUIRED-MODE-GROUP-REF
    RequiredModeGroupRef                                                   = 3806,
    /// REQUIRED-MODE-GROUPS
    RequiredModeGroups                                                     = 4308,
    /// REQUIRED-OUTER-PORT-REF
    RequiredOuterPortRef                                                   = 2392,
    /// REQUIRED-PACKAGE-ELEMENT-REF
    RequiredPackageElementRef                                              = 5481,
    /// REQUIRED-PACKAGE-ELEMENT-REFS
    RequiredPackageElementRefs                                             = 553,
    /// REQUIRED-PORT-PROTOTYPE-IREF
    RequiredPortPrototypeIref                                              = 1619,
    /// REQUIRED-RTE-VENDOR
    RequiredRteVendor                                                      = 3381,
    /// REQUIRED-SERVICE-INSTANCE-ID
    RequiredServiceInstanceId                                              = 1440,
    /// REQUIRED-SERVICE-INSTANCE-REF
    RequiredServiceInstanceRef                                             = 1514,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 1895,
    /// REQUIRED-SERVICE-PORT-IREF
    RequiredServicePortIref                                                = 5374,
    /// REQUIRED-SERVICE-VERSION
    RequiredServiceVersion                                                 = 3506,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 688,
    /// REQUIRED-TRIGGER-IREF
    RequiredTriggerIref                                                    = 3330,
    /// REQUIRED-TRIGGERS
    RequiredTriggers                                                       = 5354,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 972,
    /// REQUIREMENT-REF
    RequirementRef                                                         = 929,
    /// REQUIREMENT-REFS
    RequirementRefs                                                        = 3016,
    /// REQUIRES-INDEX
    RequiresIndex                                                          = 919,
    /// REQUIRES-SYMBOLIC-NAME-VALUE
    RequiresSymbolicNameValue                                              = 4043,
    /// RESET-CONFIRMED-BIT-ON-OVERFLOW
    ResetConfirmedBitOnOverflow                                            = 2470,
    /// RESET-PENDING-BIT-ON-OVERFLOW
    ResetPendingBitOnOverflow                                              = 5259,
    /// RESET-TO-DEFAULT
    ResetToDefault                                                         = 6385,
    /// RESET-TO-DEFAULT-SUPPORTED
    ResetToDefaultSupported                                                = 4159,
    /// RESISTANT-TO-CHANGED-SW
    ResistantToChangedSw                                                   = 3819,
    /// RESOLUTION
    Resolution                                                             = 3793,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 465,
    /// RESOURCE-DEFINITION-REF
    ResourceDefinitionRef                                                  = 3971,
    /// RESOURCE-DEFINITIONS
    ResourceDefinitions                                                    = 1803,
    /// RESOURCE-ESTIMATIONS
    ResourceEstimations                                                    = 3072,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 3789,
    /// RESOURCE-GROUP-REF
    ResourceGroupRef                                                       = 1988,
    /// RESOURCE-GROUP-REFS
    ResourceGroupRefs                                                      = 414,
    /// RESOURCE-GROUPS
    ResourceGroups                                                         = 2758,
    /// RESOURCE-GUARD-VALUE
    ResourceGuardValue                                                     = 630,
    /// RESOURCE-IDENTIFIER-TYPE
    ResourceIdentifierType                                                 = 306,
    /// RESOURCE-LIMITS
    ResourceLimits                                                         = 766,
    /// RESOURCE-NEEDS-REF
    ResourceNeedsRef                                                       = 2507,
    /// RESOURCE-NEEDS-REFS
    ResourceNeedsRefs                                                      = 5367,
    /// RESOURCE-REF
    ResourceRef                                                            = 6344,
    /// RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS
    ResourceToApplicationPartitionMappings                                 = 5135,
    /// RESOURCES
    Resources                                                              = 1320,
    /// RESPOND-TO-RESET
    RespondToReset                                                         = 216,
    /// RESPONSE-DATA-SIZE
    ResponseDataSize                                                       = 6434,
    /// RESPONSE-ERROR-POSITION
    ResponseErrorPosition                                                  = 2323,
    /// RESPONSE-ERROR-REF
    ResponseErrorRef                                                       = 1110,
    /// RESPONSE-GROUP-REF
    ResponseGroupRef                                                       = 107,
    /// RESPONSE-ON-ALL-REQUEST-SIDS
    ResponseOnAllRequestSids                                               = 1524,
    /// RESPONSE-ON-EVENT-ACTION
    ResponseOnEventAction                                                  = 1511,
    /// RESPONSE-ON-EVENT-CLASS-REF
    ResponseOnEventClassRef                                                = 3625,
    /// RESPONSE-ON-EVENT-REF
    ResponseOnEventRef                                                     = 903,
    /// RESPONSE-ON-EVENT-SCHEDULER-RATE
    ResponseOnEventSchedulerRate                                           = 2936,
    /// RESPONSE-ON-SECOND-DECLINED-REQUEST
    ResponseOnSecondDeclinedRequest                                        = 5642,
    /// RESPONSE-REF
    ResponseRef                                                            = 1878,
    /// RESPONSES
    Responses                                                              = 4667,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 3872,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 720,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 4981,
    /// REST-ENDPOINT-ARGUMENT
    RestEndpointArgument                                                   = 4927,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 4053,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 803,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 601,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 2687,
    /// REST-ENUMERATOR-DEF
    RestEnumeratorDef                                                      = 4013,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 4676,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 2095,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 470,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 6319,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 4321,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 3927,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 5703,
    /// REST-SYSTEM-TRIGGERED-EVENT
    RestSystemTriggeredEvent                                               = 184,
    /// RESTORE-AT-START
    RestoreAtStart                                                         = 1676,
    /// RESTRICT-UPDATE
    RestrictUpdate                                                         = 5763,
    /// RESTRICTION
    Restriction                                                            = 6412,
    /// RESTRICTIONS
    Restrictions                                                           = 2212,
    /// RESULTING-PROPERTIES
    ResultingProperties                                                    = 4380,
    /// RESULTING-RPT-SW-PROTOTYPING-ACCESS
    ResultingRptSwPrototypingAccess                                        = 1922,
    /// RESUME-POSITION
    ResumePosition                                                         = 112,
    /// RETRY
    Retry                                                                  = 788,
    /// RETRY-INTERVAL-TIME
    RetryIntervalTime                                                      = 6386,
    /// RETURN-CODE
    ReturnCode                                                             = 1486,
    /// RETURN-SIGNAL-REF
    ReturnSignalRef                                                        = 3655,
    /// RETURN-SIGNAL-TRIGGERING-REF
    ReturnSignalTriggeringRef                                              = 3837,
    /// RETURN-TYPE
    ReturnType                                                             = 5496,
    /// RETURN-VALUE-PROVISION
    ReturnValueProvision                                                   = 2236,
    /// REVERSED-TP-SDU-REF
    ReversedTpSduRef                                                       = 4899,
    /// REVISION-LABEL
    RevisionLabel                                                          = 6077,
    /// REVISION-LABEL-P-1
    RevisionLabelP1                                                        = 3620,
    /// REVISION-LABEL-P-2
    RevisionLabelP2                                                        = 5266,
    /// REVISION-LABELS
    RevisionLabels                                                         = 4645,
    /// RID-NUMBER
    RidNumber                                                              = 1956,
    /// ROLE
    Role                                                                   = 2982,
    /// ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT
    RoleBasedBswModuleEntryAssignment                                      = 3399,
    /// ROLE-BASED-DATA-ASSIGNMENT
    RoleBasedDataAssignment                                                = 2485,
    /// ROLE-BASED-DATA-TYPE-ASSIGNMENT
    RoleBasedDataTypeAssignment                                            = 444,
    /// ROLE-BASED-MC-DATA-ASSIGNMENT
    RoleBasedMcDataAssignment                                              = 2508,
    /// ROLE-BASED-PORT-ASSIGNMENT
    RoleBasedPortAssignment                                                = 5884,
    /// ROLE-BASED-RESOURCE-DEPENDENCY
    RoleBasedResourceDependency                                            = 3385,
    /// ROLLOUT-QUALIFICATIONS
    RolloutQualifications                                                  = 1819,
    /// ROM-BLOCK
    RomBlock                                                               = 789,
    /// ROM-BLOCK-INIT-VALUE
    RomBlockInitValue                                                      = 1056,
    /// ROM-LOCATION-REF
    RomLocationRef                                                         = 488,
    /// ROOT-ARGUMENT-DATA-PROTOTYPE-REF
    RootArgumentDataPrototypeRef                                           = 1223,
    /// ROOT-AUTOSAR-DATA-PROTOTYPE-REF
    RootAutosarDataPrototypeRef                                            = 3504,
    /// ROOT-COMPOSITION
    RootComposition                                                        = 6184,
    /// ROOT-CONTEXT-REF
    RootContextRef                                                         = 5261,
    /// ROOT-DATA-PROTOTYPE-IN-CS-REF
    RootDataPrototypeInCsRef                                               = 3749,
    /// ROOT-DATA-PROTOTYPE-IN-SR-REF
    RootDataPrototypeInSrRef                                               = 3522,
    /// ROOT-DATA-PROTOTYPE-REF
    RootDataPrototypeRef                                                   = 4454,
    /// ROOT-PARAMETER-DATA-PROTOTYPE-REF
    RootParameterDataPrototypeRef                                          = 5273,
    /// ROOT-REF
    RootRef                                                                = 4229,
    /// ROOT-SOFTWARE-COMPOSITIONS
    RootSoftwareCompositions                                               = 2701,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 1229,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 3159,
    /// ROOT-VARIABLE-DATA-PROTOTYPE-REF
    RootVariableDataPrototypeRef                                           = 1031,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 4700,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 5717,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 5158,
    /// ROUTINE-CONTROL-CLASS-REF
    RoutineControlClassRef                                                 = 3127,
    /// ROUTINE-INFO
    RoutineInfo                                                            = 3037,
    /// ROUTINE-REF
    RoutineRef                                                             = 1957,
    /// ROUTINE-SUBFUNCTION-REF
    RoutineSubfunctionRef                                                  = 1280,
    /// ROUTING-ACTIVATION-TYPE
    RoutingActivationType                                                  = 1071,
    /// ROUTING-GROUP-REF
    RoutingGroupRef                                                        = 1538,
    /// ROUTING-GROUP-REFS
    RoutingGroupRefs                                                       = 1614,
    /// ROW
    Row                                                                    = 2783,
    /// RP-IMPL-POLICY
    RpImplPolicy                                                           = 2669,
    /// RPT-AR-HOOK-IREF
    RptArHookIref                                                          = 5494,
    /// RPT-COMPONENT
    RptComponent                                                           = 707,
    /// RPT-COMPONENTS
    RptComponents                                                          = 5651,
    /// RPT-CONTAINER
    RptContainer                                                           = 2346,
    /// RPT-CONTAINERS
    RptContainers                                                          = 5145,
    /// RPT-ENABLER-IMPL-TYPE
    RptEnablerImplType                                                     = 244,
    /// RPT-EVENT-ID
    RptEventId                                                             = 1480,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 537,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 1297,
    /// RPT-EXECUTABLE-ENTITY-EVENTS
    RptExecutableEntityEvents                                              = 5215,
    /// RPT-EXECUTABLE-ENTITY-PROPERTIES
    RptExecutableEntityProperties                                          = 4961,
    /// RPT-EXECUTABLE-ENTITYS
    RptExecutableEntitys                                                   = 4671,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 2080,
    /// RPT-EXECUTION-CONTROL
    RptExecutionControl                                                    = 3724,
    /// RPT-HOOK
    RptHook                                                                = 1050,
    /// RPT-HOOK-ACCESS
    RptHookAccess                                                          = 4139,
    /// RPT-HOOKS
    RptHooks                                                               = 4,
    /// RPT-IMPL-POLICY
    RptImplPolicy                                                          = 3834,
    /// RPT-PREPARATION-LEVEL
    RptPreparationLevel                                                    = 661,
    /// RPT-PROFILE
    RptProfile                                                             = 6063,
    /// RPT-PROFILES
    RptProfiles                                                            = 3088,
    /// RPT-READ-ACCESS
    RptReadAccess                                                          = 5815,
    /// RPT-READS
    RptReads                                                               = 2436,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 1285,
    /// RPT-SERVICE-POINT-POST-REF
    RptServicePointPostRef                                                 = 4595,
    /// RPT-SERVICE-POINT-POST-REFS
    RptServicePointPostRefs                                                = 1734,
    /// RPT-SERVICE-POINT-PRE-REF
    RptServicePointPreRef                                                  = 2031,
    /// RPT-SERVICE-POINT-PRE-REFS
    RptServicePointPreRefs                                                 = 2398,
    /// RPT-SERVICE-POINTS
    RptServicePoints                                                       = 1951,
    /// RPT-SUPPORT-DATA
    RptSupportData                                                         = 2293,
    /// RPT-SW-PROTOTYPING-ACCESS
    RptSwPrototypingAccess                                                 = 2731,
    /// RPT-SYSTEM-REF
    RptSystemRef                                                           = 4832,
    /// RPT-WRITE-ACCESS
    RptWriteAccess                                                         = 5064,
    /// RPT-WRITES
    RptWrites                                                              = 3634,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 1773,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 6107,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 2444,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 163,
    /// RTE-EVENT-IREF
    RteEventIref                                                           = 1143,
    /// RTE-EVENT-IREFS
    RteEventIrefs                                                          = 6142,
    /// RTE-EVENT-SEPARATIONS
    RteEventSeparations                                                    = 2885,
    /// RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS
    RteEventToOsTaskProxyMappings                                          = 1326,
    /// RTE-PLUGIN-PROPS
    RtePluginProps                                                         = 5777,
    /// RTE-RESOURCE-ESTIMATION
    RteResourceEstimation                                                  = 114,
    /// RTP-TP
    RtpTp                                                                  = 2122,
    /// RTPS-PROTECTION-KIND
    RtpsProtectionKind                                                     = 4019,
    /// RULE
    Rule                                                                   = 3442,
    /// RULE-ARGUMENTS
    RuleArguments                                                          = 2266,
    /// RULE-BASED-AXIS-CONT
    RuleBasedAxisCont                                                      = 3672,
    /// RULE-BASED-VALUES
    RuleBasedValues                                                        = 895,
    /// RULE-INIT-STATE
    RuleInitState                                                          = 3003,
    /// RULES
    Rules                                                                  = 6069,
    /// RUN-MODE
    RunMode                                                                = 3875,
    /// RUN-RESOURCE
    RunResource                                                            = 5658,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 2161,
    /// RUNNABLE-ENTITY-ARGUMENT
    RunnableEntityArgument                                                 = 5986,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 4943,
    /// RUNNABLE-ENTITY-GROUP-IREF
    RunnableEntityGroupIref                                                = 6298,
    /// RUNNABLE-ENTITY-GROUP-IREFS
    RunnableEntityGroupIrefs                                               = 4639,
    /// RUNNABLE-ENTITY-IREF
    RunnableEntityIref                                                     = 1363,
    /// RUNNABLE-ENTITY-IREFS
    RunnableEntityIrefs                                                    = 3058,
    /// RUNNABLE-MAPPINGS
    RunnableMappings                                                       = 745,
    /// RUNNABLE-REF
    RunnableRef                                                            = 5520,
    /// RUNNABLES
    Runnables                                                              = 4289,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REF
    RunsInsideExclusiveAreaRef                                             = 1704,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REFS
    RunsInsideExclusiveAreaRefs                                            = 4637,
    /// RUNS-INSIDES
    RunsInsides                                                            = 3309,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 18,
    /// RUNTIME-IP-ADDRESS-CONFIGURATION
    RuntimeIpAddressConfiguration                                          = 4914,
    /// RUNTIME-PORT-CONFIGURATION
    RuntimePortConfiguration                                               = 2232,
    /// RVF-ACTIVE-PIXELS
    RvfActivePixels                                                        = 4623,
    /// RVF-COLOR-SPACE
    RvfColorSpace                                                          = 3795,
    /// RVF-EVENT-DEFAULT
    RvfEventDefault                                                        = 5999,
    /// RVF-FRAME-RATE
    RvfFrameRate                                                           = 2432,
    /// RVF-INTERLACED
    RvfInterlaced                                                          = 5038,
    /// RVF-PIXEL-DEPTH
    RvfPixelDepth                                                          = 1191,
    /// RVF-PIXEL-FORMAT
    RvfPixelFormat                                                         = 321,
    /// RVF-TOTAL-LINES
    RvfTotalLines                                                          = 3225,
    /// RX-ACCEPT-CONTAINED-I-PDU
    RxAcceptContainedIPdu                                                  = 4186,
    /// RX-IDENTIFIER-RANGE
    RxIdentifierRange                                                      = 5242,
    /// RX-MASK
    RxMask                                                                 = 1673,
    /// RX-NM-PDU-REF
    RxNmPduRef                                                             = 1982,
    /// RX-NM-PDU-REFS
    RxNmPduRefs                                                            = 5001,
    /// RX-PDU-POOL-REF
    RxPduPoolRef                                                           = 1170,
    /// RX-PDU-TRIGGERING-REF
    RxPduTriggeringRef                                                     = 475,
    /// RX-SECURITY-VERIFICATION
    RxSecurityVerification                                                 = 1563,
    /// RX-TIMEOUT-TIME
    RxTimeoutTime                                                          = 4185,
    /// S-3-SERVER-TIMEOUT
    S3ServerTimeout                                                        = 2682,
    /// SA-OVER-TIME
    SaOverTime                                                             = 532,
    /// SA-RAND-TIME
    SaRandTime                                                             = 4838,
    /// SA-REKEY-TIME
    SaRekeyTime                                                            = 3450,
    /// SAFE-TRANSLATION
    SafeTranslation                                                        = 6220,
    /// SAFETY-CONDITION
    SafetyCondition                                                        = 6396,
    /// SAFETY-CONDITIONS
    SafetyConditions                                                       = 2362,
    /// SAFETY-MARGIN
    SafetyMargin                                                           = 3860,
    /// SAFETY-POLICY
    SafetyPolicy                                                           = 2583,
    /// SAK-REF
    SakRef                                                                 = 5852,
    /// SAK-REKEY-TIME-SPAN
    SakRekeyTimeSpan                                                       = 6409,
    /// SAMPLE-CLOCK-PERIOD
    SampleClockPeriod                                                      = 4904,
    /// SAMPLES-PER-MICROTICK
    SamplesPerMicrotick                                                    = 447,
    /// SAVE-CONFIGURATION
    SaveConfiguration                                                      = 4960,
    /// SAVE-CONFIGURATION-ENTRY
    SaveConfigurationEntry                                                 = 5044,
    /// SCALE-CONSTR
    ScaleConstr                                                            = 3998,
    /// SCALE-CONSTRS
    ScaleConstrs                                                           = 3529,
    /// SCALING-INFO-SIZE
    ScalingInfoSize                                                        = 4367,
    /// SCHEDULE-CHANGE-NEXT-TIME-BASE
    ScheduleChangeNextTimeBase                                             = 555,
    /// SCHEDULE-TABLES
    ScheduleTables                                                         = 1789,
    /// SCHEDULER-MAX-NUMBER
    SchedulerMaxNumber                                                     = 5838,
    /// SCHEDULER-NAME-PREFIX-REF
    SchedulerNamePrefixRef                                                 = 1658,
    /// SCHEDULER-NAME-PREFIXS
    SchedulerNamePrefixs                                                   = 5548,
    /// SCHEDULING-POLICY
    SchedulingPolicy                                                       = 1955,
    /// SCHEDULING-PRIORITY
    SchedulingPriority                                                     = 4453,
    /// SCOPE
    Scope                                                                  = 931,
    /// SCOPE-EVENT-REF
    ScopeEventRef                                                          = 6290,
    /// SCOPE-EVENT-REFS
    ScopeEventRefs                                                         = 5719,
    /// SCOPE-REF
    ScopeRef                                                               = 6415,
    /// SCOPE-REFS
    ScopeRefs                                                              = 2484,
    /// SD
    Sd                                                                     = 1866,
    /// SD-CLIENT-CONFIG
    SdClientConfig                                                         = 4936,
    /// SD-CLIENT-CONFIG-REF
    SdClientConfigRef                                                      = 4277,
    /// SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SdClientEventGroupTimingConfigRef                                      = 2247,
    /// SD-CLIENT-EVENT-TIMING-CONFIG
    SdClientEventTimingConfig                                              = 2752,
    /// SD-CLIENT-TIMER-CONFIGS
    SdClientTimerConfigs                                                   = 2389,
    /// SD-SERVER-CONFIG
    SdServerConfig                                                         = 87,
    /// SD-SERVER-CONFIG-REF
    SdServerConfigRef                                                      = 11,
    /// SD-SERVER-EG-TIMING-CONFIGS
    SdServerEgTimingConfigs                                                = 5315,
    /// SD-SERVER-EVENT-CONFIG
    SdServerEventConfig                                                    = 6053,
    /// SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SdServerEventGroupTimingConfigRef                                      = 5176,
    /// SD-SERVER-TIMER-CONFIGS
    SdServerTimerConfigs                                                   = 4937,
    /// SDF
    Sdf                                                                    = 177,
    /// SDG
    Sdg                                                                    = 4921,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 545,
    /// SDG-CAPTION
    SdgCaption                                                             = 3292,
    /// SDG-CAPTION-REF
    SdgCaptionRef                                                          = 3658,
    /// SDG-CLASS
    SdgClass                                                               = 3305,
    /// SDG-CLASS-REF
    SdgClassRef                                                            = 1725,
    /// SDG-CLASSES
    SdgClasses                                                             = 5782,
    /// SDG-CONSTRAINT-REF
    SdgConstraintRef                                                       = 923,
    /// SDG-CONSTRAINT-REFS
    SdgConstraintRefs                                                      = 4685,
    /// SDG-DEF
    SdgDef                                                                 = 5179,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 1643,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 575,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 852,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 5178,
    /// SDG-REFERENCE
    SdgReference                                                           = 5228,
    /// SDG-TAILORING
    SdgTailoring                                                           = 2662,
    /// SDG-TAILORINGS
    SdgTailorings                                                          = 3124,
    /// SDGS
    Sdgs                                                                   = 4644,
    /// SDU-REF
    SduRef                                                                 = 324,
    /// SDU-REFS
    SduRefs                                                                = 4021,
    /// SDU-TYPE
    SduType                                                                = 456,
    /// SDX-REF
    SdxRef                                                                 = 5234,
    /// SDXF
    Sdxf                                                                   = 6130,
    /// SEARCH-BEHAVIOR
    SearchBehavior                                                         = 4530,
    /// SEARCH-INTENTION
    SearchIntention                                                        = 838,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REF
    SecOcComPropsForMulticastRef                                           = 2269,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REFS
    SecOcComPropsForMulticastRefs                                          = 4908,
    /// SEC-OC-CRYPTO-MAPPING-REF
    SecOcCryptoMappingRef                                                  = 2469,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 891,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 3651,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 1343,
    /// SEC-OC-JOB-MAPPINGS
    SecOcJobMappings                                                       = 962,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 108,
    /// SEC-OC-JOB-REQUIREMENT-REF
    SecOcJobRequirementRef                                                 = 3597,
    /// SEC-OC-JOB-SEMANTIC
    SecOcJobSemantic                                                       = 949,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 6079,
    /// SEC-OC-SECURE-COM-PROPS-REF
    SecOcSecureComPropsRef                                                 = 1759,
    /// SEC-OC-SECURE-COM-PROPS-REF-CONDITIONAL
    SecOcSecureComPropsRefConditional                                      = 3866,
    /// SECOND-APPLICATION-ERROR-REF
    SecondApplicationErrorRef                                              = 395,
    /// SECOND-DATA-PROTOTYPE-REF
    SecondDataPrototypeRef                                                 = 2854,
    /// SECOND-ELEMENT-INSTANCE-IREF
    SecondElementInstanceIref                                              = 4362,
    /// SECOND-ELEMENT-INSTANCE-IREFS
    SecondElementInstanceIrefs                                             = 3272,
    /// SECOND-ELEMENT-REF
    SecondElementRef                                                       = 1245,
    /// SECOND-ELEMENT-REFS
    SecondElementRefs                                                      = 1995,
    /// SECOND-ELEMENTS
    SecondElements                                                         = 5087,
    /// SECOND-KEY-SLOT-ID
    SecondKeySlotId                                                        = 3828,
    /// SECOND-MODE-GROUP-REF
    SecondModeGroupRef                                                     = 3495,
    /// SECOND-MODE-REF
    SecondModeRef                                                          = 3494,
    /// SECOND-OPERATION-REF
    SecondOperationRef                                                     = 1833,
    /// SECOND-PHYSICAL-DIMENSION-REF
    SecondPhysicalDimensionRef                                             = 6223,
    /// SECOND-PORT-REF
    SecondPortRef                                                          = 4592,
    /// SECOND-TO-FIRST-DATA-TRANSFORMATION-REF
    SecondToFirstDataTransformationRef                                     = 5360,
    /// SECOND-TRIGGER-REF
    SecondTriggerRef                                                       = 4939,
    /// SECOND-VALUE
    SecondValue                                                            = 2627,
    /// SECONDARY-FRESHNESS-VALUE-ID
    SecondaryFreshnessValueId                                              = 781,
    /// SECTION-INITIALIZATION-POLICY
    SectionInitializationPolicy                                            = 767,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 4060,
    /// SECTION-NAME-PREFIXS
    SectionNamePrefixs                                                     = 1485,
    /// SECTION-TYPE
    SectionType                                                            = 6447,
    /// SECURE-COM-CONFIG-REF
    SecureComConfigRef                                                     = 3743,
    /// SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING
    SecureComConfigToCryptoKeySlotMapping                                  = 651,
    /// SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS
    SecureComConfigToKeySlotMappings                                       = 2209,
    /// SECURE-COM-CONFIGS
    SecureComConfigs                                                       = 2704,
    /// SECURE-COM-PROPS-FOR-DDS-REF
    SecureComPropsForDdsRef                                                = 5781,
    /// SECURE-COM-PROPS-FOR-TCP-REF
    SecureComPropsForTcpRef                                                = 1553,
    /// SECURE-COM-PROPS-FOR-TCP-REFS
    SecureComPropsForTcpRefs                                               = 969,
    /// SECURE-COM-PROPS-FOR-UDP-REF
    SecureComPropsForUdpRef                                                = 6365,
    /// SECURE-COM-PROPS-FOR-UDP-REFS
    SecureComPropsForUdpRefs                                               = 615,
    /// SECURE-COM-PROPS-REF
    SecureComPropsRef                                                      = 900,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 3612,
    /// SECURE-COM-PROPSS
    SecureComPropss                                                        = 1671,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 3433,
    /// SECURE-COMMUNICATION-DEPLOYMENTS
    SecureCommunicationDeployments                                         = 5278,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 1845,
    /// SECURE-COMMUNICATION-PROPS
    SecureCommunicationProps                                               = 3709,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 1018,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 704,
    /// SECURE-PROPS-FOR-EXTERNAL-COMM-REF
    SecurePropsForExternalCommRef                                          = 3796,
    /// SECURE-PROPS-FOR-TCP-REF
    SecurePropsForTcpRef                                                   = 1204,
    /// SECURE-PROPS-FOR-UDP-REF
    SecurePropsForUdpRef                                                   = 1477,
    /// SECURE-TRANSLATION
    SecureTranslation                                                      = 3140,
    /// SECURED-AREA-LENGTH
    SecuredAreaLength                                                      = 5749,
    /// SECURED-AREA-OFFSET
    SecuredAreaOffset                                                      = 4022,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 3869,
    /// SECURED-RX-VERIFICATION
    SecuredRxVerification                                                  = 2499,
    /// SECURITY-ACCESS-CLASS-REF
    SecurityAccessClassRef                                                 = 2061,
    /// SECURITY-ACCESS-LEVEL
    SecurityAccessLevel                                                    = 5372,
    /// SECURITY-DELAY-TIME
    SecurityDelayTime                                                      = 3845,
    /// SECURITY-DELAY-TIME-ON-BOOT
    SecurityDelayTimeOnBoot                                                = 1546,
    /// SECURITY-EVENT-CONTEXT-DATA
    SecurityEventContextData                                               = 3988,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITION
    SecurityEventContextDataDefinition                                     = 5248,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF
    SecurityEventContextDataDefinitionRef                                  = 5799,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF-CONDITIONAL
    SecurityEventContextDataDefinitionRefConditional                       = 3015,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITIONS
    SecurityEventContextDataDefinitions                                    = 2307,
    /// SECURITY-EVENT-CONTEXT-DATA-ELEMENT
    SecurityEventContextDataElement                                        = 2686,
    /// SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF
    SecurityEventContextDataElementRef                                     = 3040,
    /// SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF-CONDITIONAL
    SecurityEventContextDataElementRefConditional                          = 3340,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 6210,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 667,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 5348,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 2685,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 338,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 3520,
    /// SECURITY-EVENT-DEFINITION-REF
    SecurityEventDefinitionRef                                             = 3108,
    /// SECURITY-EVENT-DEFINITION-REF-CONDITIONAL
    SecurityEventDefinitionRefConditional                                  = 3648,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 5866,
    /// SECURITY-EVENT-FILTER-CHAIN-REF
    SecurityEventFilterChainRef                                            = 665,
    /// SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL
    SecurityEventFilterChainRefConditional                                 = 5442,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 5045,
    /// SECURITY-EVENT-PROPS-REF
    SecurityEventPropsRef                                                  = 464,
    /// SECURITY-EVENT-REF
    SecurityEventRef                                                       = 4883,
    /// SECURITY-EVENT-REFS
    SecurityEventRefs                                                      = 468,
    /// SECURITY-EVENT-REPORT-INSTANCE-DEFINITION
    SecurityEventReportInstanceDefinition                                  = 5090,
    /// SECURITY-EVENT-REPORT-INSTANCE-DEFINITION-REF
    SecurityEventReportInstanceDefinitionRef                               = 3307,
    /// SECURITY-EVENT-REPORT-INSTANCE-VALUE
    SecurityEventReportInstanceValue                                       = 5014,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 5132,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 2601,
    /// SECURITY-EVENTS
    SecurityEvents                                                         = 2605,
    /// SECURITY-LEVEL-REF
    SecurityLevelRef                                                       = 3345,
    /// SECURITY-LEVEL-REFS
    SecurityLevelRefs                                                      = 1148,
    /// SEED-SIZE
    SeedSize                                                               = 397,
    /// SEGMENT-BYTE-ORDER
    SegmentByteOrder                                                       = 6080,
    /// SEGMENT-LENGTH
    SegmentLength                                                          = 203,
    /// SEGMENT-POSITION
    SegmentPosition                                                        = 3141,
    /// SEGMENT-POSITIONS
    SegmentPositions                                                       = 5062,
    /// SEGMENT-RECEPTION-TIMEOUT-TIME
    SegmentReceptionTimeoutTime                                            = 1965,
    /// SEGMENT-RECEPTION-TIMEOUT-TIME-REQUEST
    SegmentReceptionTimeoutTimeRequest                                     = 3610,
    /// SEGMENT-RECEPTION-TIMEOUT-TIME-RESPONSE
    SegmentReceptionTimeoutTimeResponse                                    = 5401,
    /// SEGMENT-REF
    SegmentRef                                                             = 4522,
    /// SEGMENT-REFS
    SegmentRefs                                                            = 162,
    /// SEGMENTATION-SUPPORTED
    SegmentationSupported                                                  = 4562,
    /// SELECT-BLOCK-FOR-FIRST-INIT-ALL
    SelectBlockForFirstInitAll                                             = 77,
    /// SELECTIONS
    Selections                                                             = 3044,
    /// SELECTOR-FIELD-BYTE-ORDER
    SelectorFieldByteOrder                                                 = 3857,
    /// SELECTOR-FIELD-CODE
    SelectorFieldCode                                                      = 64,
    /// SELECTOR-FIELD-LENGTH
    SelectorFieldLength                                                    = 3458,
    /// SELECTOR-FIELD-START-POSITION
    SelectorFieldStartPosition                                             = 287,
    /// SEND-ACTIVITY
    SendActivity                                                           = 870,
    /// SEND-INDICATION
    SendIndication                                                         = 5053,
    /// SEND-POLICYS
    SendPolicys                                                            = 201,
    /// SEND-RESP-PEND-ON-TRANS-TO-BOOT
    SendRespPendOnTransToBoot                                              = 5214,
    /// SENDER-ANNOTATION
    SenderAnnotation                                                       = 6017,
    /// SENDER-CAPABILITY
    SenderCapability                                                       = 4820,
    /// SENDER-INTENT
    SenderIntent                                                           = 4452,
    /// SENDER-IREF
    SenderIref                                                             = 339,
    /// SENDER-REC-ARRAY-ELEMENT-MAPPING
    SenderRecArrayElementMapping                                           = 4868,
    /// SENDER-REC-ARRAY-TYPE-MAPPING
    SenderRecArrayTypeMapping                                              = 4915,
    /// SENDER-REC-RECORD-ELEMENT-MAPPING
    SenderRecRecordElementMapping                                          = 5741,
    /// SENDER-REC-RECORD-TYPE-MAPPING
    SenderRecRecordTypeMapping                                             = 976,
    /// SENDER-RECEIVER-ANNOTATIONS
    SenderReceiverAnnotations                                              = 5874,
    /// SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING
    SenderReceiverCompositeElementToSignalMapping                          = 5280,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 3248,
    /// SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING
    SenderReceiverToSignalGroupMapping                                     = 827,
    /// SENDER-RECEIVER-TO-SIGNAL-MAPPING
    SenderReceiverToSignalMapping                                          = 2637,
    /// SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING
    SenderToSignalTextTableMapping                                         = 6375,
    /// SENDING-NODE-REF
    SendingNodeRef                                                         = 3726,
    /// SENDING-NODE-REFS
    SendingNodeRefs                                                        = 6120,
    /// SENSOR-ACTUATOR-REF
    SensorActuatorRef                                                      = 2335,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 5511,
    /// SENSOR-INSTANCE-ID
    SensorInstanceId                                                       = 5947,
    /// SEPARATE-SIGNAL-PATH
    SeparateSignalPath                                                     = 3049,
    /// SEPARATED-COMPONENT-IREF
    SeparatedComponentIref                                                 = 1011,
    /// SEPARATED-COMPONENT-IREFS
    SeparatedComponentIrefs                                                = 5587,
    /// SEPARATION-CYCLE-EXPONENT
    SeparationCycleExponent                                                = 1745,
    /// SEPARATION-TIME
    SeparationTime                                                         = 5060,
    /// SEPARATION-TIME-REQUEST
    SeparationTimeRequest                                                  = 3067,
    /// SEPARATION-TIME-RESPONSE
    SeparationTimeResponse                                                 = 5102,
    /// SEQUENCE-COUNTER
    SequenceCounter                                                        = 4024,
    /// SEQUENCE-COUNTER-JUMP-WIDTH
    SequenceCounterJumpWidth                                               = 2170,
    /// SEQUENCE-OFFSET
    SequenceOffset                                                         = 3934,
    /// SEQUENTIAL-INSTRUCTIONS-BEGIN
    SequentialInstructionsBegin                                            = 2219,
    /// SEQUENTIAL-INSTRUCTIONS-END
    SequentialInstructionsEnd                                              = 2270,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 4246,
    /// SERIALIZATION-TECHNOLOGY-REF
    SerializationTechnologyRef                                             = 5640,
    /// SERIALIZER
    Serializer                                                             = 642,
    /// SERIALIZER-REF
    SerializerRef                                                          = 5240,
    /// SERVER-ARGUMENT-IMPL-POLICY
    ServerArgumentImplPolicy                                               = 817,
    /// SERVER-CALL-POINTS
    ServerCallPoints                                                       = 3823,
    /// SERVER-COM-SPEC
    ServerComSpec                                                          = 6076,
    /// SERVER-NAME-IDENTIFICATION
    ServerNameIdentification                                               = 6252,
    /// SERVER-PORT-REF
    ServerPortRef                                                          = 5069,
    /// SERVER-SERVICE-MAJOR-VERSION
    ServerServiceMajorVersion                                              = 5207,
    /// SERVER-SERVICE-MINOR-VERSION
    ServerServiceMinorVersion                                              = 1507,
    /// SERVICE-BUSY
    ServiceBusy                                                            = 5107,
    /// SERVICE-CONTROL
    ServiceControl                                                         = 4985,
    /// SERVICE-DEPENDENCYS
    ServiceDependencys                                                     = 5879,
    /// SERVICE-DEPLOYMENT-REF
    ServiceDeploymentRef                                                   = 5181,
    /// SERVICE-DISCOVER-CONFIGS
    ServiceDiscoverConfigs                                                 = 5113,
    /// SERVICE-DISCOVERY-CONFIGS
    ServiceDiscoveryConfigs                                                = 1879,
    /// SERVICE-ELEMENT-MAPPING-REF
    ServiceElementMappingRef                                               = 5444,
    /// SERVICE-ELEMENT-MAPPING-REFS
    ServiceElementMappingRefs                                              = 2144,
    /// SERVICE-ELEMENT-SECURE-COM-CONFIG-REF
    ServiceElementSecureComConfigRef                                       = 2825,
    /// SERVICE-FIND-TIME-TO-LIVE
    ServiceFindTimeToLive                                                  = 1678,
    /// SERVICE-ID
    ServiceId                                                              = 6145,
    /// SERVICE-IDENTIFIER
    ServiceIdentifier                                                      = 2023,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 998,
    /// SERVICE-INSTANCE-ID
    ServiceInstanceId                                                      = 4233,
    /// SERVICE-INSTANCE-PORT-CONFIG
    ServiceInstancePortConfig                                              = 1165,
    /// SERVICE-INSTANCE-REF
    ServiceInstanceRef                                                     = 4357,
    /// SERVICE-INSTANCE-REFS
    ServiceInstanceRefs                                                    = 3283,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 614,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 5557,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF
    ServiceInstanceToPortPrototypeMappingRef                               = 2633,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 6454,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 5876,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS
    ServiceInstanceToSignalMappings                                        = 5577,
    /// SERVICE-INSTANCES
    ServiceInstances                                                       = 784,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 825,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 2092,
    /// SERVICE-INTERFACE-DEPLOYMENT-REF
    ServiceInterfaceDeploymentRef                                          = 5993,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REF
    ServiceInterfaceElementMappingRef                                      = 515,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REFS
    ServiceInterfaceElementMappingRefs                                     = 6092,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 5750,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 4987,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 2996,
    /// SERVICE-INTERFACE-ID
    ServiceInterfaceId                                                     = 1651,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 4749,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 5670,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 2427,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 5945,
    /// SERVICE-INTERFACE-REF
    ServiceInterfaceRef                                                    = 4726,
    /// SERVICE-INTERFACE-REFS
    ServiceInterfaceRefs                                                   = 1296,
    /// SERVICE-INTERFACE-SUB-ELEMENT
    ServiceInterfaceSubElement                                             = 4780,
    /// SERVICE-INTERFACE-TREF
    ServiceInterfaceTref                                                   = 26,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 729,
    /// SERVICE-INTERFACE-VERSION
    ServiceInterfaceVersion                                                = 1859,
    /// SERVICE-ITEMS
    ServiceItems                                                           = 2535,
    /// SERVICE-KIND
    ServiceKind                                                            = 531,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 2237,
    /// SERVICE-OFFER-TIME-TO-LIVE
    ServiceOfferTimeToLive                                                 = 2318,
    /// SERVICE-POINT-SYMBOL-POST
    ServicePointSymbolPost                                                 = 2039,
    /// SERVICE-POINT-SYMBOL-PRE
    ServicePointSymbolPre                                                  = 1476,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 2340,
    /// SERVICE-REQUEST-CALLBACK-TYPE
    ServiceRequestCallbackType                                             = 416,
    /// SERVICE-RESOURCE-REF
    ServiceResourceRef                                                     = 247,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 775,
    /// SERVICE-TABLES
    ServiceTables                                                          = 5840,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 76,
    /// SESSION-CONTROL-CLASS-REF
    SessionControlClassRef                                                 = 778,
    /// SESSION-HANDLING
    SessionHandling                                                        = 2439,
    /// SESSION-HANDLING-SR
    SessionHandlingSr                                                      = 1483,
    /// SESSION-ID
    SessionId                                                              = 5330,
    /// SESSION-ID-SUPPORT
    SessionIdSupport                                                       = 5700,
    /// SET
    Set                                                                    = 4406,
    /// SET-FUNCTION-GROUP-STATE-IREF
    SetFunctionGroupStateIref                                              = 2942,
    /// SETTER-CALL-REF
    SetterCallRef                                                          = 605,
    /// SETTER-CALL-SIGNAL-REF
    SetterCallSignalRef                                                    = 5923,
    /// SETTER-OPERATION-REF
    SetterOperationRef                                                     = 6088,
    /// SETTER-REF
    SetterRef                                                              = 4933,
    /// SETTER-RETURN-REF
    SetterReturnRef                                                        = 222,
    /// SETTER-RETURN-SIGNAL-REF
    SetterReturnSignalRef                                                  = 1506,
    /// SEVERITY
    Severity                                                               = 3470,
    /// SHALL-NOT-RUN-ON-REF
    ShallNotRunOnRef                                                       = 1142,
    /// SHALL-NOT-RUN-ON-REFS
    ShallNotRunOnRefs                                                      = 3646,
    /// SHALL-RUN-ON-REF
    ShallRunOnRef                                                          = 253,
    /// SHALL-RUN-ON-REFS
    ShallRunOnRefs                                                         = 2745,
    /// SHAPER
    Shaper                                                                 = 6008,
    /// SHARED
    Shared                                                                 = 5128,
    /// SHARED-AXIS-TYPE-REF
    SharedAxisTypeRef                                                      = 4205,
    /// SHARED-PARAMETERS
    SharedParameters                                                       = 597,
    /// SHARED-TIMER
    SharedTimer                                                            = 5725,
    /// SHORT-LABEL
    ShortLabel                                                             = 1035,
    /// SHORT-LABEL-PATTERN
    ShortLabelPattern                                                      = 1192,
    /// SHORT-NAME
    ShortName                                                              = 6239,
    /// SHORT-NAME-FRAGMENT
    ShortNameFragment                                                      = 3893,
    /// SHORT-NAME-FRAGMENTS
    ShortNameFragments                                                     = 4725,
    /// SHORT-NAME-PATTERN
    ShortNamePattern                                                       = 5606,
    /// SHORT-TERM-ADJUSTMENT
    ShortTermAdjustment                                                    = 3551,
    /// SHORT-TERM-ADJUSTMENT-SUPPORTED
    ShortTermAdjustmentSupported                                           = 2006,
    /// SHUTDOWN
    Shutdown                                                               = 3719,
    /// SIGNAL-AGE
    SignalAge                                                              = 5208,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 6326,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 2530,
    /// SIGNAL-BASED-EVENT-REF
    SignalBasedEventRef                                                    = 225,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 3252,
    /// SIGNAL-BASED-FIELD-REF
    SignalBasedFieldRef                                                    = 4371,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 5546,
    /// SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFireAndForgetMethodToISignalTriggeringMapping               = 3258,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 4077,
    /// SIGNAL-BASED-METHOD-REF
    SignalBasedMethodRef                                                   = 192,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 43,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 477,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 4628,
    /// SIGNAL-FAN
    SignalFan                                                              = 3079,
    /// SIGNAL-GROUP-REF
    SignalGroupRef                                                         = 1154,
    /// SIGNAL-I-PDU-COUNTER
    SignalIPduCounter                                                      = 3587,
    /// SIGNAL-I-PDU-REPLICATION
    SignalIPduReplication                                                  = 3615,
    /// SIGNAL-MAPPINGS
    SignalMappings                                                         = 3765,
    /// SIGNAL-PATH-CONSTRAINTS
    SignalPathConstraints                                                  = 1753,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 3519,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 1918,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS
    SignalServiceTranslationEventPropss                                    = 2016,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 4126,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 920,
    /// SIGNAL-SERVICE-TRANSLATION-PROPSS
    SignalServiceTranslationPropss                                         = 5738,
    /// SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING
    SignalToReceiverTextTableMapping                                       = 5101,
    /// SIGNALS
    Signals                                                                = 2619,
    /// SIGNATURE-SCHEME-ID
    SignatureSchemeId                                                      = 2988,
    /// SIGNATURE-SCHEME-REF
    SignatureSchemeRef                                                     = 1129,
    /// SIGNATURE-SCHEME-REFS
    SignatureSchemeRefs                                                    = 2458,
    /// SIGNATURE-SUPPORT-AP
    SignatureSupportAp                                                     = 1752,
    /// SIGNATURE-SUPPORT-CP
    SignatureSupportCp                                                     = 5897,
    /// SIGNIFICANCE
    Significance                                                           = 5702,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 1234,
    /// SIZE
    Size                                                                   = 348,
    /// SIZE-OF-ARRAY-LENGTH-FIELD
    SizeOfArrayLengthField                                                 = 4757,
    /// SIZE-OF-ARRAY-LENGTH-FIELDS
    SizeOfArrayLengthFields                                                = 3027,
    /// SIZE-OF-STRING-LENGTH-FIELD
    SizeOfStringLengthField                                                = 1132,
    /// SIZE-OF-STRING-LENGTH-FIELDS
    SizeOfStringLengthFields                                               = 2239,
    /// SIZE-OF-STRUCT-LENGTH-FIELD
    SizeOfStructLengthField                                                = 1262,
    /// SIZE-OF-STRUCT-LENGTH-FIELDS
    SizeOfStructLengthFields                                               = 1648,
    /// SIZE-OF-UNION-LENGTH-FIELD
    SizeOfUnionLengthField                                                 = 4123,
    /// SIZE-OF-UNION-LENGTH-FIELDS
    SizeOfUnionLengthFields                                                = 4045,
    /// SIZE-OF-UNION-TYPE-SELECTOR-FIELD
    SizeOfUnionTypeSelectorField                                           = 738,
    /// SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE
    SlaveActAsPassiveCommunicationSlave                                    = 6241,
    /// SLAVE-EVENT-REF
    SlaveEventRef                                                          = 5252,
    /// SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME
    SlaveQualifiedUnexpectedLinkDownTime                                   = 2207,
    /// SLAVE-REF
    SlaveRef                                                               = 4888,
    /// SLAVES
    Slaves                                                                 = 3099,
    /// SLEEP-MODE-EXECUTION-DELAY
    SleepModeExecutionDelay                                                = 2336,
    /// SLEEP-MODE-SUPPORTED
    SleepModeSupported                                                     = 202,
    /// SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST
    SleepRepetitionDelayOfSleepRequest                                     = 4219,
    /// SLEEP-REPETITIONS-OF-SLEEP-REQUEST
    SleepRepetitionsOfSleepRequest                                         = 2208,
    /// SLEEP-TIME
    SleepTime                                                              = 5095,
    /// SLOT-CAPACITY
    SlotCapacity                                                           = 2680,
    /// SLOT-ID
    SlotId                                                                 = 5307,
    /// SLOT-TYPE
    SlotType                                                               = 6269,
    /// SM-INTERACTS-WITH-NM-MAPPING
    SmInteractsWithNmMapping                                               = 2707,
    /// SNAPSHOT-RECORD-CONTENTS
    SnapshotRecordContents                                                 = 1743,
    /// SO-AD-CONFIG
    SoAdConfig                                                             = 2190,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 4817,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 659,
    /// SO-CON-I-PDU-IDENTIFIER-REF
    SoConIPduIdentifierRef                                                 = 1529,
    /// SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL
    SoConIPduIdentifierRefConditional                                      = 4003,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 2641,
    /// SOCKET-ADDRESS-REF
    SocketAddressRef                                                       = 66,
    /// SOCKET-ADDRESS-REF-CONDITIONAL
    SocketAddressRefConditional                                            = 5832,
    /// SOCKET-ADDRESSS
    SocketAddresss                                                         = 5394,
    /// SOCKET-CONNECTION
    SocketConnection                                                       = 769,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 3156,
    /// SOCKET-CONNECTION-BUNDLE-REF
    SocketConnectionBundleRef                                              = 1660,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER
    SocketConnectionIpduIdentifier                                         = 6166,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 1354,
    /// SOCKET-CONNECTION-REF
    SocketConnectionRef                                                    = 723,
    /// SOCKET-CONNECTION-REFS
    SocketConnectionRefs                                                   = 4059,
    /// SOCKET-OPTION
    SocketOption                                                           = 2176,
    /// SOCKET-OPTIONS
    SocketOptions                                                          = 2718,
    /// SOCKET-PROTOCOL
    SocketProtocol                                                         = 4834,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION
    SoftwareActivationDependencyCompareCondition                           = 5517,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA
    SoftwareActivationDependencyFormula                                    = 57,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-REF
    SoftwareActivationDependencyRef                                        = 5406,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 1236,
    /// SOFTWARE-CLUSTER-COM-RESOURCE-REF
    SoftwareClusterComResourceRef                                          = 3581,
    /// SOFTWARE-CLUSTER-DEPENDENCY
    SoftwareClusterDependency                                              = 790,
    /// SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION
    SoftwareClusterDependencyCompareCondition                              = 1273,
    /// SOFTWARE-CLUSTER-DEPENDENCY-FORMULA
    SoftwareClusterDependencyFormula                                       = 445,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 5308,
    /// SOFTWARE-CLUSTER-DESIGN-DEPENDENCY
    SoftwareClusterDesignDependency                                        = 1446,
    /// SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS
    SoftwareClusterDiagnosticDeploymentProps                               = 2521,
    /// SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS
    SoftwareClusterDoipDiagnosticAddress                                   = 3616,
    /// SOFTWARE-CLUSTER-ID
    SoftwareClusterId                                                      = 2920,
    /// SOFTWARE-CLUSTER-REF
    SoftwareClusterRef                                                     = 5157,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 5820,
    /// SOFTWARE-CLUSTER-SOVD-ADDRESS
    SoftwareClusterSovdAddress                                             = 5536,
    /// SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING
    SoftwareClusterToApplicationPartitionMapping                           = 4442,
    /// SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPINGS
    SoftwareClusterToApplicationPartitionMappings                          = 5396,
    /// SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS
    SoftwareClusterToResourceMappings                                      = 6057,
    /// SOFTWARE-CLUSTER-UDS-DIAGNOSTIC-ADDRESS
    SoftwareClusterUdsDiagnosticAddress                                    = 6312,
    /// SOFTWARE-COMPOSITION-TREF
    SoftwareCompositionTref                                                = 6007,
    /// SOFTWARE-CONTEXT
    SoftwareContext                                                        = 1835,
    /// SOFTWARE-MEMORY-SECTION-REF
    SoftwareMemorySectionRef                                               = 5070,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 3868,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 4992,
    /// SOFTWARE-PACKAGE-STEPS
    SoftwarePackageSteps                                                   = 1079,
    /// SOFTWARE-PACKAGE-STORING
    SoftwarePackageStoring                                                 = 1300,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 5223,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 3604,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 5121,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 5324,
    /// SOMEIP-EVENT-PROPS
    SomeipEventProps                                                       = 881,
    /// SOMEIP-FIELD
    SomeipField                                                            = 5316,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 4890,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 421,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 152,
    /// SOMEIP-METHOD-PROPS
    SomeipMethodProps                                                      = 1726,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 5058,
    /// SOMEIP-REMOTE-MULTICAST-CONFIG
    SomeipRemoteMulticastConfig                                            = 3359,
    /// SOMEIP-REMOTE-UNICAST-CONFIG
    SomeipRemoteUnicastConfig                                              = 3829,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 148,
    /// SOMEIP-RULE
    SomeipRule                                                             = 4647,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 4521,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdClientEventGroupTimingConfigRef                                = 342,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdClientEventGroupTimingConfigRefConditional                     = 4964,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 4472,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdClientServiceInstanceConfigRef                                 = 5474,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdClientServiceInstanceConfigRefConditional                      = 4127,
    /// SOMEIP-SD-RULE
    SomeipSdRule                                                           = 2217,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 13,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdServerEventGroupTimingConfigRef                                = 3250,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdServerEventGroupTimingConfigRefConditional                     = 4253,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 5300,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdServerServiceInstanceConfigRef                                 = 1294,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdServerServiceInstanceConfigRefConditional                      = 3121,
    /// SOMEIP-SERVICE-DISCOVERY
    SomeipServiceDiscovery                                                 = 1107,
    /// SOMEIP-SERVICE-DISCOVERY-PORT
    SomeipServiceDiscoveryPort                                             = 5728,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 1912,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 1657,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 559,
    /// SOMEIP-SERVICE-VERSION
    SomeipServiceVersion                                                   = 2452,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 5418,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 1517,
    /// SOMEIP-TP-CONNECTION
    SomeipTpConnection                                                     = 6010,
    /// SOMEIP-TRANSFORMATION-DESCRIPTION
    SomeipTransformationDescription                                        = 4383,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS
    SomeipTransformationISignalProps                                       = 4474,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    SomeipTransformationISignalPropsConditional                            = 2064,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    SomeipTransformationISignalPropsVariants                               = 4759,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 5329,
    /// SOMEIP-TRANSFORMATION-PROPS-REF
    SomeipTransformationPropsRef                                           = 1748,
    /// SOUCE-CAN-ID-REF
    SouceCanIdRef                                                          = 1083,
    /// SOURCE-ADDRESS-CODE
    SourceAddressCode                                                      = 154,
    /// SOURCE-ADDRESS-MASK
    SourceAddressMask                                                      = 3266,
    /// SOURCE-ADDRESS-RANGE-END
    SourceAddressRangeEnd                                                  = 1177,
    /// SOURCE-ADDRESS-RANGE-START
    SourceAddressRangeStart                                                = 6354,
    /// SOURCE-CAN-ID-CODE
    SourceCanIdCode                                                        = 4957,
    /// SOURCE-CAN-ID-MASK
    SourceCanIdMask                                                        = 2280,
    /// SOURCE-CAN-ID-REF
    SourceCanIdRef                                                         = 4146,
    /// SOURCE-CHANNEL
    SourceChannel                                                          = 5335,
    /// SOURCE-EEC-REF
    SourceEecRef                                                           = 4517,
    /// SOURCE-EEC-REFS
    SourceEecRefs                                                          = 5518,
    /// SOURCE-ELEMENT-REF
    SourceElementRef                                                       = 5963,
    /// SOURCE-ELEMENT-REFS
    SourceElementRefs                                                      = 6013,
    /// SOURCE-EVENT-REF
    SourceEventRef                                                         = 579,
    /// SOURCE-EVENT-REFS
    SourceEventRefs                                                        = 1070,
    /// SOURCE-FIELD-REF
    SourceFieldRef                                                         = 785,
    /// SOURCE-FRAME-REF
    SourceFrameRef                                                         = 3593,
    /// SOURCE-I-PDU-REF
    SourceIPduRef                                                          = 2022,
    /// SOURCE-ID
    SourceId                                                               = 4239,
    /// SOURCE-INSTANCE-IREF
    SourceInstanceIref                                                     = 6322,
    /// SOURCE-INSTANCE-IREFS
    SourceInstanceIrefs                                                    = 708,
    /// SOURCE-IP-ADDRESS
    SourceIpAddress                                                        = 3457,
    /// SOURCE-IPV-4-ADDRESS
    SourceIpv4Address                                                      = 4926,
    /// SOURCE-IPV-6-ADDRESS
    SourceIpv6Address                                                      = 196,
    /// SOURCE-LIN-PID-REF
    SourceLinPidRef                                                        = 412,
    /// SOURCE-MAC-ADDRESS
    SourceMacAddress                                                       = 5462,
    /// SOURCE-MAC-ADDRESS-MASK
    SourceMacAddressMask                                                   = 2568,
    /// SOURCE-MAX-ADDRESS
    SourceMaxAddress                                                       = 797,
    /// SOURCE-METHOD-REF
    SourceMethodRef                                                        = 5244,
    /// SOURCE-MIN-ADDRESS
    SourceMinAddress                                                       = 190,
    /// SOURCE-NETWORK-MASK
    SourceNetworkMask                                                      = 1778,
    /// SOURCE-PORTS
    SourcePorts                                                            = 3958,
    /// SOURCE-POSSIBLE-ERROR-REF
    SourcePossibleErrorRef                                                 = 4325,
    /// SOURCE-REF
    SourceRef                                                              = 3732,
    /// SOURCE-SERVICE-INTERFACE-REF
    SourceServiceInterfaceRef                                              = 2405,
    /// SOURCE-SERVICE-INTERFACE-REFS
    SourceServiceInterfaceRefs                                             = 1159,
    /// SOURCE-SIGNAL-REF
    SourceSignalRef                                                        = 4504,
    /// SOURCE-TRIGGER-REF
    SourceTriggerRef                                                       = 4877,
    /// SOVD-GATEWAY-INSTANTIATION
    SovdGatewayInstantiation                                               = 1032,
    /// SOVD-LOCK-REF
    SovdLockRef                                                            = 2341,
    /// SOVD-SERVER-INSTANTIATION
    SovdServerInstantiation                                                = 6273,
    /// SOVD-VALIDATION-ORDER-REF
    SovdValidationOrderRef                                                 = 3771,
    /// SOVD-VALIDATION-ORDER-REFS
    SovdValidationOrderRefs                                                = 1731,
    /// SPARSE-TIMESTAMP-ENABLED
    SparseTimestampEnabled                                                 = 4189,
    /// SPECIFIC-CHANNEL-REF
    SpecificChannelRef                                                     = 3770,
    /// SPECIFIC-CHANNELS
    SpecificChannels                                                       = 4054,
    /// SPECIFIC-PHYSICAL-CHANNEL-REF
    SpecificPhysicalChannelRef                                             = 2287,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 6450,
    /// SPECIFICATION-DOCUMENT-SCOPES
    SpecificationDocumentScopes                                            = 3112,
    /// SPECIFICATION-SCOPE
    SpecificationScope                                                     = 5081,
    /// SPEED
    Speed                                                                  = 4989,
    /// SPEEDUP
    Speedup                                                                = 4503,
    /// SPN
    Spn                                                                    = 3723,
    /// SPN-REF
    SpnRef                                                                 = 1705,
    /// SPN-REFS
    SpnRefs                                                                = 4543,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 3684,
    /// SRVT
    Srvt                                                                   = 5075,
    /// SSP-OFFSET
    SspOffset                                                              = 250,
    /// SSRC
    Ssrc                                                                   = 304,
    /// ST-MIN
    StMin                                                                  = 3311,
    /// STACK-USAGES
    StackUsages                                                            = 4702,
    /// STANDARD
    Standard                                                               = 5218,
    /// STANDARD-REVISION
    StandardRevision                                                       = 3323,
    /// STANDARD-REVISIONS
    StandardRevisions                                                      = 350,
    /// START
    Start                                                                  = 3756,
    /// START-ACTION-REF
    StartActionRef                                                         = 5508,
    /// START-ACTION-REFS
    StartActionRefs                                                        = 3595,
    /// START-ADDRESS
    StartAddress                                                           = 606,
    /// START-INDEX
    StartIndex                                                             = 3557,
    /// START-IREF
    StartIref                                                              = 3827,
    /// START-ON-EVENT-REF
    StartOnEventRef                                                        = 1828,
    /// START-POSITION
    StartPosition                                                          = 4897,
    /// START-STATE-MACHINE-REF
    StartStateMachineRef                                                   = 995,
    /// STARTS-ON-EVENT-REF
    StartsOnEventRef                                                       = 6410,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 1588,
    /// STARTUP-CONFIG-REF
    StartupConfigRef                                                       = 361,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 5916,
    /// STARTUP-CONFIGS
    StartupConfigs                                                         = 5543,
    /// STARTUP-OPTION
    StartupOption                                                          = 2431,
    /// STARTUP-OPTIONS
    StartupOptions                                                         = 193,
    /// STATE
    State                                                                  = 4000,
    /// STATE-CLIENT-INTERFACE
    StateClientInterface                                                   = 2379,
    /// STATE-DEP-FIREWALL-REF
    StateDepFirewallRef                                                    = 4172,
    /// STATE-DEP-FIREWALL-REFS
    StateDepFirewallRefs                                                   = 5263,
    /// STATE-DEPENDENT-FIREWALL
    StateDependentFirewall                                                 = 1722,
    /// STATE-DEPENDENT-FIREWALL-REF
    StateDependentFirewallRef                                              = 5137,
    /// STATE-DEPENDENT-STARTUP-CONFIG
    StateDependentStartupConfig                                            = 888,
    /// STATE-DEPENDENT-STARTUP-CONFIGS
    StateDependentStartupConfigs                                           = 2786,
    /// STATE-MACHINE
    StateMachine                                                           = 1751,
    /// STATE-MANAGEMEN-PHM-ERROR-INTERFACE
    StateManagemenPhmErrorInterface                                        = 540,
    /// STATE-MANAGEMENT-ACTION-LIST
    StateManagementActionList                                              = 2103,
    /// STATE-MANAGEMENT-BASED-ON-TCP-FLAGS
    StateManagementBasedOnTcpFlags                                         = 6446,
    /// STATE-MANAGEMENT-COMPARE-FORMULA
    StateManagementCompareFormula                                          = 4828,
    /// STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE
    StateManagementDiagTriggerInterface                                    = 1069,
    /// STATE-MANAGEMENT-EM-ERROR-INTERFACE
    StateManagementEmErrorInterface                                        = 1628,
    /// STATE-MANAGEMENT-ERROR-COMPARE-RULE
    StateManagementErrorCompareRule                                        = 1091,
    /// STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE
    StateManagementFunctionGroupSwitchNotificationInterface                = 3833,
    /// STATE-MANAGEMENT-MODULE-INSTANTIATION
    StateManagementModuleInstantiation                                     = 2001,
    /// STATE-MANAGEMENT-NM-ACTION-ITEM
    StateManagementNmActionItem                                            = 2946,
    /// STATE-MANAGEMENT-PHM-ERROR-INTERFACE
    StateManagementPhmErrorInterface                                       = 2715,
    /// STATE-MANAGEMENT-REQUEST-ERROR
    StateManagementRequestError                                            = 5274,
    /// STATE-MANAGEMENT-REQUEST-RULE
    StateManagementRequestRule                                             = 1612,
    /// STATE-MANAGEMENT-REQUEST-TRIGGER
    StateManagementRequestTrigger                                          = 542,
    /// STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM
    StateManagementSetFunctionGroupStateActionItem                         = 3826,
    /// STATE-MANAGEMENT-SLEEP-ACTION-ITEM
    StateManagementSleepActionItem                                         = 5625,
    /// STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM
    StateManagementStateMachineActionItem                                  = 4931,
    /// STATE-MANAGEMENT-STATE-NOTIFICATION
    StateManagementStateNotification                                       = 1518,
    /// STATE-MANAGEMENT-SYNC-ACTION-ITEM
    StateManagementSyncActionItem                                          = 1888,
    /// STATE-MANAGEMENT-TRIGGER-COMPARE-RULE
    StateManagementTriggerCompareRule                                      = 5679,
    /// STATE-REF
    StateRef                                                               = 2261,
    /// STATE-REFERENCES
    StateReferences                                                        = 1804,
    /// STATE-REQUEST-PORT-IREF
    StateRequestPortIref                                                   = 4631,
    /// STATE-REQUEST-REF
    StateRequestRef                                                        = 5056,
    /// STATIC-MEMORYS
    StaticMemorys                                                          = 4149,
    /// STATIC-PART
    StaticPart                                                             = 5886,
    /// STATIC-PARTS
    StaticParts                                                            = 4483,
    /// STATIC-REMOTE-MULTICAST-ADDRESSES
    StaticRemoteMulticastAddresses                                         = 2246,
    /// STATIC-REMOTE-UNICAST-ADDRESSES
    StaticRemoteUnicastAddresses                                           = 5757,
    /// STATIC-SLOT-DURATION
    StaticSlotDuration                                                     = 5623,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 1750,
    /// STATIC-SOCKET-CONNECTION-REF
    StaticSocketConnectionRef                                              = 2514,
    /// STATIC-SOCKET-CONNECTIONS
    StaticSocketConnections                                                = 506,
    /// STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR
    StatusBitHandlingTestFailedSinceLastClear                              = 1006,
    /// STATUS-BIT-STORAGE-TEST-FAILED
    StatusBitStorageTestFailed                                             = 54,
    /// STATUS-ID
    StatusId                                                               = 839,
    /// STATUS-IREF
    StatusIref                                                             = 110,
    /// STATUS-REF
    StatusRef                                                              = 6221,
    /// STATUS-SUB-TLV
    StatusSubTlv                                                           = 986,
    /// STATUSS
    Statuss                                                                = 2205,
    /// STD
    Std                                                                    = 1952,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 3007,
    /// STD-REF
    StdRef                                                                 = 379,
    /// STD-REFS
    StdRefs                                                                = 340,
    /// STEP-SIZE
    StepSize                                                               = 3705,
    /// STIM-ENABLER
    StimEnabler                                                            = 2077,
    /// STIMULUS-REF
    StimulusRef                                                            = 1544,
    /// STOP
    Stop                                                                   = 375,
    /// STOP-IREF
    StopIref                                                               = 1489,
    /// STOP-STATE-MACHINE-REF
    StopStateMachineRef                                                    = 1921,
    /// STORAGE-CONDITION-GROUP-REF
    StorageConditionGroupRef                                               = 2420,
    /// STORAGE-CONDITIONS
    StorageConditions                                                      = 5395,
    /// STORAGE-STATE-EVALUATION
    StorageStateEvaluation                                                 = 1723,
    /// STORE-AT-SHUTDOWN
    StoreAtShutdown                                                        = 3682,
    /// STORE-CYCLIC
    StoreCyclic                                                            = 2890,
    /// STORE-EMERGENCY
    StoreEmergency                                                         = 3094,
    /// STORE-EVENT-ENABLED
    StoreEventEnabled                                                      = 1861,
    /// STORE-EVENT-SUPPORT
    StoreEventSupport                                                      = 2667,
    /// STORE-IMMEDIATE
    StoreImmediate                                                         = 1317,
    /// STORE-ON-CHANGE
    StoreOnChange                                                          = 5796,
    /// STORING
    Storing                                                                = 4893,
    /// STREAM-FILTER-PORT-RANGE
    StreamFilterPortRange                                                  = 4480,
    /// STREAM-FILTER-RULE
    StreamFilterRule                                                       = 3478,
    /// STREAM-FILTERS
    StreamFilters                                                          = 5626,
    /// STREAM-GATE-REF
    StreamGateRef                                                          = 2719,
    /// STREAM-GATES
    StreamGates                                                            = 5211,
    /// STREAM-ID
    StreamId                                                               = 2278,
    /// STREAM-IDENTIFICATION-HANDLE-REF
    StreamIdentificationHandleRef                                          = 639,
    /// STREAM-IDENTIFICATION-HANDLE-REFS
    StreamIdentificationHandleRefs                                         = 1810,
    /// STREAM-IDENTIFICATION-WILDCARD
    StreamIdentificationWildcard                                           = 4265,
    /// STREAM-IDENTIFIER
    StreamIdentifier                                                       = 4273,
    /// STREAMS-PER-FRAME
    StreamsPerFrame                                                        = 1366,
    /// STRING-ENCODING
    StringEncoding                                                         = 3198,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 2391,
    /// SUB
    Sub                                                                    = 3688,
    /// SUB-ATTRIBUTE-TAILORINGS
    SubAttributeTailorings                                                 = 4763,
    /// SUB-CONTAINERS
    SubContainers                                                          = 4137,
    /// SUB-DOMAIN-REF
    SubDomainRef                                                           = 2038,
    /// SUB-DOMAIN-REFS
    SubDomainRefs                                                          = 5585,
    /// SUB-ELEMENT-MAPPING
    SubElementMapping                                                      = 2032,
    /// SUB-ELEMENT-MAPPINGS
    SubElementMappings                                                     = 991,
    /// SUB-ELEMENTS
    SubElements                                                            = 1656,
    /// SUB-FUNCTION-REF
    SubFunctionRef                                                         = 3883,
    /// SUB-FUNCTION-REFS
    SubFunctionRefs                                                        = 4699,
    /// SUB-GROUP-REF
    SubGroupRef                                                            = 4736,
    /// SUB-GROUP-REFS
    SubGroupRefs                                                           = 1687,
    /// SUB-NODE-CHANNEL-REF
    SubNodeChannelRef                                                      = 4679,
    /// SUB-NODE-CHANNELS
    SubNodeChannels                                                        = 3708,
    /// SUB-NODE-NUMBER
    SubNodeNumber                                                          = 5037,
    /// SUB-NODE-PHYSICAL-CHANNEL-REF
    SubNodePhysicalChannelRef                                              = 3034,
    /// SUB-SDG-REF
    SubSdgRef                                                              = 323,
    /// SUB-SOFTWARE-CLUSTER-REF
    SubSoftwareClusterRef                                                  = 1816,
    /// SUB-SOFTWARE-CLUSTER-REFS
    SubSoftwareClusterRefs                                                 = 3788,
    /// SUB-TLV-CONFIG
    SubTlvConfig                                                           = 2656,
    /// SUB-TYPE
    SubType                                                                = 2171,
    /// SUBFUNCTION
    Subfunction                                                            = 4008,
    /// SUBFUNCTIONS
    Subfunctions                                                           = 6123,
    /// SUBMESSAGE-TYPE
    SubmessageType                                                         = 2902,
    /// SUBNET-NUMBER
    SubnetNumber                                                           = 284,
    /// SUBSCRIBE-EVENTGROUP-RETRY-DELAY
    SubscribeEventgroupRetryDelay                                          = 5464,
    /// SUBSCRIBE-EVENTGROUP-RETRY-MAX
    SubscribeEventgroupRetryMax                                            = 2143,
    /// SUBSTITUTED-FRAME-REF
    SubstitutedFrameRef                                                    = 3147,
    /// SUBSTITUTED-FRAME-REFS
    SubstitutedFrameRefs                                                   = 2053,
    /// SUBTITLE
    Subtitle                                                               = 529,
    /// SUCCESSOR-REF
    SuccessorRef                                                           = 816,
    /// SUCCESSOR-REFS
    SuccessorRefs                                                          = 6362,
    /// SUP
    Sup                                                                    = 4288,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 6175,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF
    SupervisedEntityCheckpointNeedsRef                                     = 5850,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL
    SupervisedEntityCheckpointNeedsRefConditional                          = 3951,
    /// SUPERVISED-ENTITY-ID
    SupervisedEntityId                                                     = 2229,
    /// SUPERVISED-ENTITY-IREF
    SupervisedEntityIref                                                   = 3647,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 3556,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 543,
    /// SUPERVISION-CONDITION
    SupervisionCondition                                                   = 3824,
    /// SUPERVISION-CYCLE
    SupervisionCycle                                                       = 1719,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 3742,
    /// SUPERVISION-ENTITYS
    SupervisionEntitys                                                     = 2276,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 1325,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 6350,
    /// SUPERVISION-MODE-CONDITIONS
    SupervisionModeConditions                                              = 1353,
    /// SUPERVISION-MODES
    SupervisionModes                                                       = 4905,
    /// SUPERVISION-REF
    SupervisionRef                                                         = 396,
    /// SUPPLIER-ID
    SupplierId                                                             = 3179,
    /// SUPPLIER-VALIDATION-ORDER-REF
    SupplierValidationOrderRef                                             = 1875,
    /// SUPPLIER-VALIDATION-ORDER-REFS
    SupplierValidationOrderRefs                                            = 6091,
    /// SUPPORT-BUFFER-LOCKING
    SupportBufferLocking                                                   = 560,
    /// SUPPORT-DIRTY-FLAG
    SupportDirtyFlag                                                       = 3530,
    /// SUPPORT-INFO
    SupportInfo                                                            = 3838,
    /// SUPPORT-INFO-BIT
    SupportInfoBit                                                         = 27,
    /// SUPPORT-INFO-BYTE
    SupportInfoByte                                                        = 1004,
    /// SUPPORTED-CIPHER-SUITES
    SupportedCipherSuites                                                  = 228,
    /// SUPPORTED-CONFIG-VARIANT
    SupportedConfigVariant                                                 = 2002,
    /// SUPPORTED-CONFIG-VARIANTS
    SupportedConfigVariants                                                = 3259,
    /// SUPPORTED-FEATURES
    SupportedFeatures                                                      = 2819,
    /// SUPPORTED-TIMER-GRANULARITY
    SupportedTimerGranularity                                              = 3325,
    /// SUPPORTING-MATERIAL
    SupportingMaterial                                                     = 6270,
    /// SUPPORTS-ASYNCHRONOUS-MODE-SWITCH
    SupportsAsynchronousModeSwitch                                         = 3975,
    /// SUPPORTS-MULTIPLE-INSTANTIATION
    SupportsMultipleInstantiation                                          = 2366,
    /// SUPPORTS-MULTIPLE-NOTIFIER-SETS
    SupportsMultipleNotifierSets                                           = 5791,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 3734,
    /// SW-ADDR-METHOD-REF
    SwAddrMethodRef                                                        = 2099,
    /// SW-ADDRMETHOD-REF
    SwAddrmethodRef                                                        = 4266,
    /// SW-ALIGNMENT
    SwAlignment                                                            = 366,
    /// SW-ARRAYSIZE
    SwArraysize                                                            = 249,
    /// SW-AXIS-CONT
    SwAxisCont                                                             = 5219,
    /// SW-AXIS-CONTS
    SwAxisConts                                                            = 487,
    /// SW-AXIS-GENERIC
    SwAxisGeneric                                                          = 5754,
    /// SW-AXIS-GROUPED
    SwAxisGrouped                                                          = 3226,
    /// SW-AXIS-INDEX
    SwAxisIndex                                                            = 834,
    /// SW-AXIS-INDIVIDUAL
    SwAxisIndividual                                                       = 4580,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 5707,
    /// SW-AXIS-TYPE-REF
    SwAxisTypeRef                                                          = 1187,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 5398,
    /// SW-BIT-REPRESENTATION
    SwBitRepresentation                                                    = 214,
    /// SW-CALIBRATION-ACCESS
    SwCalibrationAccess                                                    = 3588,
    /// SW-CALIBRATION-NOTES
    SwCalibrationNotes                                                     = 5904,
    /// SW-CALIBRATION-NOTESS
    SwCalibrationNotess                                                    = 5027,
    /// SW-CALPRM-AXIS
    SwCalprmAxis                                                           = 3077,
    /// SW-CALPRM-AXIS-SET
    SwCalprmAxisSet                                                        = 3843,
    /// SW-CALPRM-IMPL
    SwCalprmImpl                                                           = 2410,
    /// SW-CALPRM-IMPLS
    SwCalprmImpls                                                          = 1945,
    /// SW-CALPRM-PROTOTYPE-REF
    SwCalprmPrototypeRef                                                   = 5253,
    /// SW-CARB-DOC
    SwCarbDoc                                                              = 305,
    /// SW-CARB-DOCS
    SwCarbDocs                                                             = 2648,
    /// SW-CLASS-ATTR-INSTANCE-IMPL
    SwClassAttrInstanceImpl                                                = 2344,
    /// SW-CLASS-IMPL
    SwClassImpl                                                            = 3784,
    /// SW-CLASS-IMPLS
    SwClassImpls                                                           = 3602,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 1551,
    /// SW-CLASS-PROTOTYPE-REF
    SwClassPrototypeRef                                                    = 1659,
    /// SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF
    SwClusterDesignProvidedPortIref                                        = 1005,
    /// SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF
    SwClusterDesignRequiredPortIref                                        = 773,
    /// SW-CLUSTER-MAPPINGS
    SwClusterMappings                                                      = 2582,
    /// SW-CLUSTER-REF
    SwClusterRef                                                           = 1153,
    /// SW-CLUSTER-REFS
    SwClusterRefs                                                          = 4708,
    /// SW-CLUSTERS
    SwClusters                                                             = 2952,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 5968,
    /// SW-COMP-TO-ECU-MAPPING-REF
    SwCompToEcuMappingRef                                                  = 4870,
    /// SW-COMP-TO-ECU-MAPPING-REFS
    SwCompToEcuMappingRefs                                                 = 2277,
    /// SW-COMPARISON-VARIABLES
    SwComparisonVariables                                                  = 2678,
    /// SW-COMPONENT-ASSIGNMENTS
    SwComponentAssignments                                                 = 21,
    /// SW-COMPONENT-DOCUMENTATION
    SwComponentDocumentation                                               = 283,
    /// SW-COMPONENT-DOCUMENTATIONS
    SwComponentDocumentations                                              = 4919,
    /// SW-COMPONENT-IREF
    SwComponentIref                                                        = 4293,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 6196,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 5899,
    /// SW-COMPONENT-PROTOTYPE-ASSIGNMENT
    SwComponentPrototypeAssignment                                         = 748,
    /// SW-COMPONENT-PROTOTYPE-IREF
    SwComponentPrototypeIref                                               = 4953,
    /// SW-COMPONENT-PROTOTYPE-REF
    SwComponentPrototypeRef                                                = 5909,
    /// SW-COMPOSITIONS
    SwCompositions                                                         = 5634,
    /// SW-DATA-DEF-PROPS
    SwDataDefProps                                                         = 113,
    /// SW-DATA-DEF-PROPS-CONDITIONAL
    SwDataDefPropsConditional                                              = 4390,
    /// SW-DATA-DEF-PROPS-VARIANTS
    SwDataDefPropsVariants                                                 = 4098,
    /// SW-DATA-DEPENDENCY
    SwDataDependency                                                       = 1242,
    /// SW-DATA-DEPENDENCY-ARGS
    SwDataDependencyArgs                                                   = 2188,
    /// SW-DATA-DEPENDENCY-FORMULA
    SwDataDependencyFormula                                                = 6001,
    /// SW-DIAGNOSTICS-NOTES
    SwDiagnosticsNotes                                                     = 5628,
    /// SW-DIAGNOSTICS-NOTESS
    SwDiagnosticsNotess                                                    = 2500,
    /// SW-FEATURE-DEF
    SwFeatureDef                                                           = 2058,
    /// SW-FEATURE-DEFS
    SwFeatureDefs                                                          = 3925,
    /// SW-FEATURE-DESC
    SwFeatureDesc                                                          = 979,
    /// SW-FEATURE-DESCS
    SwFeatureDescs                                                         = 1075,
    /// SW-FILL-CHARACTER
    SwFillCharacter                                                        = 3695,
    /// SW-GENERIC-AXIS-DESC
    SwGenericAxisDesc                                                      = 235,
    /// SW-GENERIC-AXIS-PARAM
    SwGenericAxisParam                                                     = 3654,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 2630,
    /// SW-GENERIC-AXIS-PARAM-TYPE-REF
    SwGenericAxisParamTypeRef                                              = 989,
    /// SW-GENERIC-AXIS-PARAM-TYPES
    SwGenericAxisParamTypes                                                = 6314,
    /// SW-GENERIC-AXIS-PARAMS
    SwGenericAxisParams                                                    = 6190,
    /// SW-HOST-VARIABLE
    SwHostVariable                                                         = 1429,
    /// SW-IMPL-MAPPINGS
    SwImplMappings                                                         = 2610,
    /// SW-IMPL-POLICY
    SwImplPolicy                                                           = 4147,
    /// SW-INTENDED-RESOLUTION
    SwIntendedResolution                                                   = 4558,
    /// SW-INTERPOLATION-METHOD
    SwInterpolationMethod                                                  = 3036,
    /// SW-IS-VIRTUAL
    SwIsVirtual                                                            = 1512,
    /// SW-MAINTENANCE-NOTES
    SwMaintenanceNotes                                                     = 4994,
    /// SW-MAINTENANCE-NOTESS
    SwMaintenanceNotess                                                    = 6336,
    /// SW-MAPPINGS
    SwMappings                                                             = 2275,
    /// SW-MAX-AXIS-POINTS
    SwMaxAxisPoints                                                        = 1202,
    /// SW-MAX-TEXT-SIZE
    SwMaxTextSize                                                          = 4609,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 3349,
    /// SW-MIN-AXIS-POINTS
    SwMinAxisPoints                                                        = 5699,
    /// SW-NUMBER-OF-AXIS-POINTS
    SwNumberOfAxisPoints                                                   = 5091,
    /// SW-POINTER-TARGET-PROPS
    SwPointerTargetProps                                                   = 5544,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 6305,
    /// SW-RECORD-LAYOUT-COMPONENT
    SwRecordLayoutComponent                                                = 1484,
    /// SW-RECORD-LAYOUT-GROUP
    SwRecordLayoutGroup                                                    = 2005,
    /// SW-RECORD-LAYOUT-GROUP-AXIS
    SwRecordLayoutGroupAxis                                                = 2798,
    /// SW-RECORD-LAYOUT-GROUP-FROM
    SwRecordLayoutGroupFrom                                                = 5528,
    /// SW-RECORD-LAYOUT-GROUP-INDEX
    SwRecordLayoutGroupIndex                                               = 2777,
    /// SW-RECORD-LAYOUT-GROUP-STEP
    SwRecordLayoutGroupStep                                                = 6187,
    /// SW-RECORD-LAYOUT-GROUP-TO
    SwRecordLayoutGroupTo                                                  = 4443,
    /// SW-RECORD-LAYOUT-REF
    SwRecordLayoutRef                                                      = 1932,
    /// SW-RECORD-LAYOUT-V
    SwRecordLayoutV                                                        = 1531,
    /// SW-RECORD-LAYOUT-V-AXIS
    SwRecordLayoutVAxis                                                    = 1095,
    /// SW-RECORD-LAYOUT-V-FIX-VALUE
    SwRecordLayoutVFixValue                                                = 5692,
    /// SW-RECORD-LAYOUT-V-INDEX
    SwRecordLayoutVIndex                                                   = 255,
    /// SW-RECORD-LAYOUT-V-PROP
    SwRecordLayoutVProp                                                    = 5103,
    /// SW-REFRESH-TIMING
    SwRefreshTiming                                                        = 3619,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 3319,
    /// SW-SERVICE-IMPL-POLICY
    SwServiceImplPolicy                                                    = 2736,
    /// SW-SYSCOND
    SwSyscond                                                              = 4034,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 4487,
    /// SW-SYSTEMCONST-REF
    SwSystemconstRef                                                       = 3174,
    /// SW-SYSTEMCONST-VALUE
    SwSystemconstValue                                                     = 6042,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 4356,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REF
    SwSystemconstantValueSetRef                                            = 4073,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REFS
    SwSystemconstantValueSetRefs                                           = 596,
    /// SW-SYSTEMCONSTANT-VALUES
    SwSystemconstantValues                                                 = 1975,
    /// SW-TEST-DESC
    SwTestDesc                                                             = 3497,
    /// SW-TEST-DESCS
    SwTestDescs                                                            = 168,
    /// SW-TEXT-PROPS
    SwTextProps                                                            = 853,
    /// SW-VALUE-BLOCK-SIZE
    SwValueBlockSize                                                       = 4681,
    /// SW-VALUE-BLOCK-SIZE-MULTS
    SwValueBlockSizeMults                                                  = 1045,
    /// SW-VALUE-CONT
    SwValueCont                                                            = 2042,
    /// SW-VALUES-PHYS
    SwValuesPhys                                                           = 479,
    /// SW-VARIABLE-IMPL
    SwVariableImpl                                                         = 299,
    /// SW-VARIABLE-IMPLS
    SwVariableImpls                                                        = 4730,
    /// SW-VARIABLE-PROTOTYPE-REF
    SwVariablePrototypeRef                                                 = 6341,
    /// SW-VARIABLE-REFS
    SwVariableRefs                                                         = 6275,
    /// SW-VERSION
    SwVersion                                                              = 5665,
    /// SWC-BEHAVIOR-REF
    SwcBehaviorRef                                                         = 3209,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 728,
    /// SWC-BSW-MAPPING-REF
    SwcBswMappingRef                                                       = 2339,
    /// SWC-BSW-RUNNABLE-MAPPING
    SwcBswRunnableMapping                                                  = 101,
    /// SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE
    SwcBswSynchronizedModeGroupPrototype                                   = 2790,
    /// SWC-BSW-SYNCHRONIZED-TRIGGER
    SwcBswSynchronizedTrigger                                              = 5863,
    /// SWC-EXCLUSIVE-AREA-POLICY
    SwcExclusiveAreaPolicy                                                 = 6291,
    /// SWC-FLAT-SERVICE-DEPENDENCY-REF
    SwcFlatServiceDependencyRef                                            = 4498,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 3196,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 93,
    /// SWC-MAPPING-CONSTRAINT-REF
    SwcMappingConstraintRef                                                = 6358,
    /// SWC-MAPPING-CONSTRAINT-REFS
    SwcMappingConstraintRefs                                               = 1117,
    /// SWC-MODE-GROUP-IREF
    SwcModeGroupIref                                                       = 4741,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 911,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 3189,
    /// SWC-MODE-SWITCH-EVENT-REF
    SwcModeSwitchEventRef                                                  = 4831,
    /// SWC-RUNNABLE-REF
    SwcRunnableRef                                                         = 127,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 924,
    /// SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    SwcServiceDependencyInExecutableIref                                   = 4103,
    /// SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    SwcServiceDependencyInSystemIref                                       = 365,
    /// SWC-SERVICE-DEPENDENCY-IREF
    SwcServiceDependencyIref                                               = 1972,
    /// SWC-TIMING
    SwcTiming                                                              = 2665,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 2433,
    /// SWC-TO-APPLICATION-PARTITION-MAPPINGS
    SwcToApplicationPartitionMappings                                      = 5913,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 5216,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT
    SwcToEcuMappingConstraint                                              = 3944,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE
    SwcToEcuMappingConstraintType                                          = 5693,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 1244,
    /// SWC-TO-SWC-OPERATION-ARGUMENTS
    SwcToSwcOperationArguments                                             = 2381,
    /// SWC-TO-SWC-SIGNAL
    SwcToSwcSignal                                                         = 3183,
    /// SWC-TRIGGER-IREF
    SwcTriggerIref                                                         = 3832,
    /// SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY
    SwitchAsynchronousTrafficShaperGroupEntry                              = 2093,
    /// SWITCH-FLOW-METERING-ENTRY
    SwitchFlowMeteringEntry                                                = 6047,
    /// SWITCH-MAC-ADDRESS-LEARNING-MODE
    SwitchMacAddressLearningMode                                           = 1301,
    /// SWITCH-STREAM-FILTER-ENTRY
    SwitchStreamFilterEntry                                                = 1194,
    /// SWITCH-STREAM-GATE-ENTRY
    SwitchStreamGateEntry                                                  = 4070,
    /// SWITCH-STREAM-IDENTIFICATION
    SwitchStreamIdentification                                             = 4605,
    /// SWITCH-STREAM-IDENTIFICATIONS
    SwitchStreamIdentifications                                            = 401,
    /// SYMBOL
    Symbol                                                                 = 3937,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 4806,
    /// SYMBOL-WINDOW
    SymbolWindow                                                           = 607,
    /// SYMBOL-WINDOW-ACTION-POINT-OFFSET
    SymbolWindowActionPointOffset                                          = 1574,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 1331,
    /// SYMBOLIC-NAME-VALUE
    SymbolicNameValue                                                      = 496,
    /// SYNC-CONFIRMATION-TIMEOUT
    SyncConfirmationTimeout                                                = 2412,
    /// SYNC-COUNTER-INIT
    SyncCounterInit                                                        = 1942,
    /// SYNC-DATA-ID-LIST
    SyncDataIdList                                                         = 1039,
    /// SYNC-DATA-ID-LISTS
    SyncDataIdLists                                                        = 166,
    /// SYNC-FRAME-ID-COUNT-MAX
    SyncFrameIdCountMax                                                    = 4626,
    /// SYNC-INTERVAL
    SyncInterval                                                           = 5933,
    /// SYNC-JUMP-WIDTH
    SyncJumpWidth                                                          = 2435,
    /// SYNC-LOSS-THRESHOLD
    SyncLossThreshold                                                      = 4942,
    /// SYNC-LOSS-TIMEOUT
    SyncLossTimeout                                                        = 928,
    /// SYNC-NODE-MAX
    SyncNodeMax                                                            = 6389,
    /// SYNC-PERIOD
    SyncPeriod                                                             = 4840,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 258,
    /// SYNCHRONIZATION-CONSTRAINT-TYPE
    SynchronizationConstraintType                                          = 2931,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 3839,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 139,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 372,
    /// SYNCHRONIZED-MODE-GROUPS
    SynchronizedModeGroups                                                 = 1182,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 3119,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 4973,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 1135,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 2933,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 2479,
    /// SYNCHRONIZED-TRIGGERS
    SynchronizedTriggers                                                   = 3573,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 1209,
    /// SYSC-REF
    SyscRef                                                                = 5568,
    /// SYSC-STRING-REF
    SyscStringRef                                                          = 6458,
    /// SYSTEM
    System                                                                 = 1081,
    /// SYSTEM-DOCUMENTATIONS
    SystemDocumentations                                                   = 1954,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 218,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 3540,
    /// SYSTEM-MEMORY-USAGES
    SystemMemoryUsages                                                     = 616,
    /// SYSTEM-REF
    SystemRef                                                              = 1287,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 754,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 3448,
    /// SYSTEM-SIGNAL-GROUP-REF
    SystemSignalGroupRef                                                   = 429,
    /// SYSTEM-SIGNAL-GROUP-TO-COM-RESOURCE-MAPPINGS
    SystemSignalGroupToComResourceMappings                                 = 3057,
    /// SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalGroupToCommunicationResourceMapping                        = 1765,
    /// SYSTEM-SIGNAL-REF
    SystemSignalRef                                                        = 1501,
    /// SYSTEM-SIGNAL-REFS
    SystemSignalRefs                                                       = 1179,
    /// SYSTEM-SIGNAL-TO-COM-RESOURCE-MAPPINGS
    SystemSignalToComResourceMappings                                      = 3232,
    /// SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalToCommunicationResourceMapping                             = 2367,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 2185,
    /// SYSTEM-TRIGGERED-EVENTS
    SystemTriggeredEvents                                                  = 6059,
    /// SYSTEM-VERSION
    SystemVersion                                                          = 4419,
    /// TA-TYPE
    TaType                                                                 = 3747,
    /// TABLE
    Table                                                                  = 1180,
    /// TABLE-CAPTION
    TableCaption                                                           = 4202,
    /// TABLE-ENTRYS
    TableEntrys                                                            = 820,
    /// TAG-ID
    TagId                                                                  = 4107,
    /// TAG-WITH-OPTIONAL-VALUE
    TagWithOptionalValue                                                   = 5262,
    /// TAILORING-REF
    TailoringRef                                                           = 2251,
    /// TAILORING-REFS
    TailoringRefs                                                          = 2286,
    /// TARGET-APPLICATION-RECORD-ELEMENT-REF
    TargetApplicationRecordElementRef                                      = 5385,
    /// TARGET-CATEGORY
    TargetCategory                                                         = 2953,
    /// TARGET-CHANNEL
    TargetChannel                                                          = 5227,
    /// TARGET-COMPONENT-REF
    TargetComponentRef                                                     = 5649,
    /// TARGET-DATA-ELEMENT-REF
    TargetDataElementRef                                                   = 1116,
    /// TARGET-DATA-PROTOTYPE-GROUP-REF
    TargetDataPrototypeGroupRef                                            = 438,
    /// TARGET-DATA-PROTOTYPE-IN-CS-REF
    TargetDataPrototypeInCsRef                                             = 4827,
    /// TARGET-DATA-PROTOTYPE-IN-SR-REF
    TargetDataPrototypeInSrRef                                             = 5201,
    /// TARGET-DATA-PROTOTYPE-REF
    TargetDataPrototypeRef                                                 = 670,
    /// TARGET-DATA-PROTOYPE-REF
    TargetDataProtoypeRef                                                  = 5123,
    /// TARGET-EEC-REF
    TargetEecRef                                                           = 1255,
    /// TARGET-EEC-REFS
    TargetEecRefs                                                          = 3496,
    /// TARGET-ELEMENT-REF
    TargetElementRef                                                       = 6173,
    /// TARGET-EVENT-REF
    TargetEventRef                                                         = 641,
    /// TARGET-EVENT-REFS
    TargetEventRefs                                                        = 1910,
    /// TARGET-FIELD-REF
    TargetFieldRef                                                         = 5032,
    /// TARGET-FRAME-REF
    TargetFrameRef                                                         = 1137,
    /// TARGET-HEALTH-CHANNEL-REF
    TargetHealthChannelRef                                                 = 2893,
    /// TARGET-I-PDU
    TargetIPdu                                                             = 3098,
    /// TARGET-I-PDU-REF
    TargetIPduRef                                                          = 2114,
    /// TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TargetImplementationDataTypeElementRef                                 = 1677,
    /// TARGET-METHOD-REF
    TargetMethodRef                                                        = 4243,
    /// TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    TargetModeDeclarationGroupPrototypeRef                                 = 5376,
    /// TARGET-MODE-DECLARATION-REF
    TargetModeDeclarationRef                                               = 1908,
    /// TARGET-MODE-GROUP-REF
    TargetModeGroupRef                                                     = 257,
    /// TARGET-MODE-REF
    TargetModeRef                                                          = 4909,
    /// TARGET-MODULE-ID
    TargetModuleId                                                         = 4049,
    /// TARGET-MODULE-REFS
    TargetModuleRefs                                                       = 626,
    /// TARGET-OPERATION-REF
    TargetOperationRef                                                     = 4012,
    /// TARGET-P-PORT-PROTOTYPE-REF
    TargetPPortPrototypeRef                                                = 220,
    /// TARGET-P-PORT-REF
    TargetPPortRef                                                         = 4835,
    /// TARGET-PARAMETER-DATA-PROTOTYPE-REF
    TargetParameterDataPrototypeRef                                        = 760,
    /// TARGET-PDU-TRIGGERINGS
    TargetPduTriggerings                                                   = 3565,
    /// TARGET-PHM-CHECKPOINT-REF
    TargetPhmCheckpointRef                                                 = 717,
    /// TARGET-PHM-SUPERVISED-ENTITY-IREF
    TargetPhmSupervisedEntityIref                                          = 4837,
    /// TARGET-PORT-PROTOTYPE-REF
    TargetPortPrototypeRef                                                 = 2968,
    /// TARGET-POSSIBLE-ERROR-REF
    TargetPossibleErrorRef                                                 = 2774,
    /// TARGET-PROVIDED-OPERATION-REF
    TargetProvidedOperationRef                                             = 3348,
    /// TARGET-R-PORT-PROTOTYPE-REF
    TargetRPortPrototypeRef                                                = 3641,
    /// TARGET-R-PORT-REF
    TargetRPortRef                                                         = 3096,
    /// TARGET-REF
    TargetRef                                                              = 129,
    /// TARGET-REFS
    TargetRefs                                                             = 3407,
    /// TARGET-REQUIRED-OPERATION-REF
    TargetRequiredOperationRef                                             = 4542,
    /// TARGET-RTE-EVENT-REF
    TargetRteEventRef                                                      = 1824,
    /// TARGET-RUNNABLE-ENTITY-GROUP-REF
    TargetRunnableEntityGroupRef                                           = 3467,
    /// TARGET-RUNNABLE-ENTITY-REF
    TargetRunnableEntityRef                                                = 5361,
    /// TARGET-SIGNAL-REF
    TargetSignalRef                                                        = 1947,
    /// TARGET-STATUS-REF
    TargetStatusRef                                                        = 5510,
    /// TARGET-SWC-SERVICE-DEPENDENCY-REF
    TargetSwcServiceDependencyRef                                          = 5495,
    /// TARGET-TRIGGER-REF
    TargetTriggerRef                                                       = 3301,
    /// TARGET-TYPE
    TargetType                                                             = 4786,
    /// TARGET-VARIABLE-ACCESS-REF
    TargetVariableAccessRef                                                = 1977,
    /// TARGET-VARIABLE-DATA-PROTOTYPE-REF
    TargetVariableDataPrototypeRef                                         = 4026,
    /// TASK
    Task                                                                   = 2791,
    /// TBODY
    Tbody                                                                  = 1067,
    /// TCP-ALIVE-CHECK-RESPONSE-TIMEOUT
    TcpAliveCheckResponseTimeout                                           = 2732,
    /// TCP-CONGESTION-AVOIDANCE-ENABLED
    TcpCongestionAvoidanceEnabled                                          = 1072,
    /// TCP-CONNECT-TIMEOUT
    TcpConnectTimeout                                                      = 554,
    /// TCP-DELAYED-ACK-TIMEOUT
    TcpDelayedAckTimeout                                                   = 5654,
    /// TCP-FAST-RECOVERY-ENABLED
    TcpFastRecoveryEnabled                                                 = 2754,
    /// TCP-FAST-RETRANSMIT-ENABLED
    TcpFastRetransmitEnabled                                               = 4740,
    /// TCP-FIN-WAIT-2-TIMEOUT
    TcpFinWait2Timeout                                                     = 3342,
    /// TCP-GENERAL-INACTIVITY-TIME
    TcpGeneralInactivityTime                                               = 6122,
    /// TCP-INITIAL-INACTIVITY-TIME
    TcpInitialInactivityTime                                               = 4773,
    /// TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP
    TcpIpArpNumGratuitousArpOnStartup                                      = 2909,
    /// TCP-IP-ARP-PACKET-QUEUE-ENABLED
    TcpIpArpPacketQueueEnabled                                             = 1089,
    /// TCP-IP-ARP-REQUEST-TIMEOUT
    TcpIpArpRequestTimeout                                                 = 1792,
    /// TCP-IP-ARP-TABLE-ENTRY-TIMEOUT
    TcpIpArpTableEntryTimeout                                              = 4066,
    /// TCP-IP-AUTO-IP-INIT-TIMEOUT
    TcpIpAutoIpInitTimeout                                                 = 4532,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MAX
    TcpIpDhcpV6CnfDelayMax                                                 = 2875,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MIN
    TcpIpDhcpV6CnfDelayMin                                                 = 314,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MAX
    TcpIpDhcpV6InfDelayMax                                                 = 3110,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MIN
    TcpIpDhcpV6InfDelayMin                                                 = 3949,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MAX
    TcpIpDhcpV6SolDelayMax                                                 = 6313,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MIN
    TcpIpDhcpV6SolDelayMin                                                 = 1716,
    /// TCP-IP-ICMP-PROPS-REF
    TcpIpIcmpPropsRef                                                      = 204,
    /// TCP-IP-ICMP-PROPS-REFS
    TcpIpIcmpPropsRefs                                                     = 2614,
    /// TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED
    TcpIpIcmpV4EchoReplyEnabled                                            = 1625,
    /// TCP-IP-ICMP-V-4-TTL
    TcpIpIcmpV4Ttl                                                         = 612,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION
    TcpIpIcmpV6EchoReplyAvoidFragmentation                                 = 5954,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED
    TcpIpIcmpV6EchoReplyEnabled                                            = 1559,
    /// TCP-IP-ICMP-V-6-HOP-LIMIT
    TcpIpIcmpV6HopLimit                                                    = 2079,
    /// TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED
    TcpIpIcmpV6MsgDestinationUnreachableEnabled                            = 1909,
    /// TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED
    TcpIpIcmpV6MsgParameterProblemEnabled                                  = 802,
    /// TCP-IP-IP-FRAGMENTATION-RX-ENABLED
    TcpIpIpFragmentationRxEnabled                                          = 4079,
    /// TCP-IP-IP-NUM-FRAGMENTS
    TcpIpIpNumFragments                                                    = 1208,
    /// TCP-IP-IP-NUM-REASS-DGRAMS
    TcpIpIpNumReassDgrams                                                  = 5047,
    /// TCP-IP-IP-REASS-TIMEOUT
    TcpIpIpReassTimeout                                                    = 6369,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-COUNT
    TcpIpIpReassemblyBufferCount                                           = 1541,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-SIZE
    TcpIpIpReassemblyBufferSize                                            = 5322,
    /// TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT
    TcpIpIpReassemblySegmentCount                                          = 930,
    /// TCP-IP-IP-REASSEMBLY-TIMEOUT
    TcpIpIpReassemblyTimeout                                               = 4574,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT
    TcpIpIpTxFragmentBufferCount                                           = 2703,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE
    TcpIpIpTxFragmentBufferSize                                            = 4257,
    /// TCP-IP-NDP-DEFAULT-REACHABLE-TIME
    TcpIpNdpDefaultReachableTime                                           = 4815,
    /// TCP-IP-NDP-DEFAULT-RETRANS-TIMER
    TcpIpNdpDefaultRetransTimer                                            = 5952,
    /// TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE
    TcpIpNdpDefaultRouterListSize                                          = 1636,
    /// TCP-IP-NDP-DEFENSIVE-PROCESSING
    TcpIpNdpDefensiveProcessing                                            = 4089,
    /// TCP-IP-NDP-DELAY-FIRST-PROBE-TIME
    TcpIpNdpDelayFirstProbeTime                                            = 2288,
    /// TCP-IP-NDP-DELAY-FIRST-PROBE-TIME-VALUE
    TcpIpNdpDelayFirstProbeTimeValue                                       = 3674,
    /// TCP-IP-NDP-DESTINATION-CACHE-SIZE
    TcpIpNdpDestinationCacheSize                                           = 1390,
    /// TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED
    TcpIpNdpDynamicHopLimitEnabled                                         = 181,
    /// TCP-IP-NDP-DYNAMIC-MTU-ENABLED
    TcpIpNdpDynamicMtuEnabled                                              = 1736,
    /// TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED
    TcpIpNdpDynamicReachableTimeEnabled                                    = 890,
    /// TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED
    TcpIpNdpDynamicRetransTimeEnabled                                      = 3168,
    /// TCP-IP-NDP-MAX-RANDOM-FACTOR
    TcpIpNdpMaxRandomFactor                                                = 3212,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY
    TcpIpNdpMaxRtrSolicitationDelay                                        = 4051,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATIONS
    TcpIpNdpMaxRtrSolicitations                                            = 971,
    /// TCP-IP-NDP-MIN-RANDOM-FACTOR
    TcpIpNdpMinRandomFactor                                                = 5908,
    /// TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED
    TcpIpNdpNeighborUnreachabilityDetectionEnabled                         = 2692,
    /// TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS
    TcpIpNdpNumMulticastSolicitations                                      = 2840,
    /// TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS
    TcpIpNdpNumUnicastSolicitations                                        = 507,
    /// TCP-IP-NDP-PACKET-QUEUE-ENABLED
    TcpIpNdpPacketQueueEnabled                                             = 771,
    /// TCP-IP-NDP-PREFIX-LIST-SIZE
    TcpIpNdpPrefixListSize                                                 = 1086,
    /// TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED
    TcpIpNdpRandomReachableTimeEnabled                                     = 2012,
    /// TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED
    TcpIpNdpRndRtrSolicitationDelayEnabled                                 = 873,
    /// TCP-IP-NDP-RTR-SOLICITATION-INTERVAL
    TcpIpNdpRtrSolicitationInterval                                        = 3864,
    /// TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS
    TcpIpNdpSlaacDadNumberOfTransmissions                                  = 935,
    /// TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY
    TcpIpNdpSlaacDadRetransmissionDelay                                    = 4300,
    /// TCP-IP-NDP-SLAAC-DELAY-ENABLED
    TcpIpNdpSlaacDelayEnabled                                              = 4227,
    /// TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED
    TcpIpNdpSlaacOptimisticDadEnabled                                      = 1247,
    /// TCP-IP-PROPS-REF
    TcpIpPropsRef                                                          = 2096,
    /// TCP-IP-PROPS-REFS
    TcpIpPropsRefs                                                         = 5353,
    /// TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC
    TcpIpTlsUseSecurityExtensionForceEncryptThenMac                        = 4950,
    /// TCP-KEEP-ALIVE-ENABLED
    TcpKeepAliveEnabled                                                    = 2081,
    /// TCP-KEEP-ALIVE-INTERVAL
    TcpKeepAliveInterval                                                   = 5473,
    /// TCP-KEEP-ALIVE-PROBES-MAX
    TcpKeepAliveProbesMax                                                  = 2393,
    /// TCP-KEEP-ALIVE-TIME
    TcpKeepAliveTime                                                       = 5501,
    /// TCP-LOCAL-PORT
    TcpLocalPort                                                           = 5981,
    /// TCP-MAX-RTX
    TcpMaxRtx                                                              = 2849,
    /// TCP-MSL
    TcpMsl                                                                 = 4007,
    /// TCP-NAGLE-ENABLED
    TcpNagleEnabled                                                        = 5318,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 1230,
    /// TCP-OPTION-FILTER-LISTS
    TcpOptionFilterLists                                                   = 3469,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 5609,
    /// TCP-PORT
    TcpPort                                                                = 5766,
    /// TCP-PORT-REF
    TcpPortRef                                                             = 3240,
    /// TCP-PROPS
    TcpProps                                                               = 6286,
    /// TCP-RECEIVE-WINDOW-MAX
    TcpReceiveWindowMax                                                    = 6183,
    /// TCP-REMOTE-PORT
    TcpRemotePort                                                          = 2425,
    /// TCP-RETRANSMISSION-TIMEOUT
    TcpRetransmissionTimeout                                               = 5529,
    /// TCP-ROLE
    TcpRole                                                                = 2113,
    /// TCP-RULE
    TcpRule                                                                = 2311,
    /// TCP-SLOW-START-ENABLED
    TcpSlowStartEnabled                                                    = 874,
    /// TCP-SYN-MAX-RTX
    TcpSynMaxRtx                                                           = 5487,
    /// TCP-SYN-RECEIVED-TIMEOUT
    TcpSynReceivedTimeout                                                  = 2509,
    /// TCP-TP
    TcpTp                                                                  = 1970,
    /// TCP-TP-CONFIG
    TcpTpConfig                                                            = 451,
    /// TCP-TP-PORT
    TcpTpPort                                                              = 217,
    /// TCP-TTL
    TcpTtl                                                                 = 2805,
    /// TCP-UDP-CONFIG
    TcpUdpConfig                                                           = 5005,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 6363,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 2939,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 6012,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS
    TdCpSoftwareClusterResourceToTdMappings                                = 5022,
    /// TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS
    TdCpSoftwareClusterToTdMappings                                        = 1020,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 6024,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE
    TdEventBswInternalBehaviorType                                         = 2735,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 5885,
    /// TD-EVENT-BSW-MODE-DECLARATION-TYPE
    TdEventBswModeDeclarationType                                          = 2314,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 5350,
    /// TD-EVENT-BSW-MODULE-TYPE
    TdEventBswModuleType                                                   = 1052,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 2014,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 5571,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 3575,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 6041,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 3038,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 3683,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 3896,
    /// TD-EVENT-MODE-DECLARATION-TYPE
    TdEventModeDeclarationType                                             = 3106,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 1901,
    /// TD-EVENT-OPERATION-TYPE
    TdEventOperationType                                                   = 6453,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 2721,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE
    TdEventServiceInstanceDiscoveryType                                    = 1567,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 2895,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE
    TdEventServiceInstanceEventType                                        = 1985,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 4944,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE
    TdEventServiceInstanceFieldType                                        = 267,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 5522,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE
    TdEventServiceInstanceMethodType                                       = 2385,
    /// TD-EVENT-SLLET-PORT
    TdEventSlletPort                                                       = 6256,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 5200,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 3701,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE
    TdEventSwcInternalBehaviorType                                         = 1335,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 5608,
    /// TD-EVENT-TRIGGER-TYPE
    TdEventTriggerType                                                     = 3617,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 1637,
    /// TD-EVENT-TYPE
    TdEventType                                                            = 5049,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 600,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE
    TdEventVariableDataPrototypeType                                       = 2406,
    /// TD-EVENT-VFB-PORT-GROUP
    TdEventVfbPortGroup                                                    = 1028,
    /// TD-EVENT-VFB-PORT-REF
    TdEventVfbPortRef                                                      = 3297,
    /// TD-EVENT-VFB-PORT-REFS
    TdEventVfbPortRefs                                                     = 1395,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 2088,
    /// TD-HEADER-ID-FILTERS
    TdHeaderIdFilters                                                      = 2408,
    /// TD-HEADER-ID-RANGE
    TdHeaderIdRange                                                        = 6424,
    /// TD-PDU-TRIGGERING-FILTER-REF
    TdPduTriggeringFilterRef                                               = 1097,
    /// TD-PDU-TRIGGERING-FILTER-REFS
    TdPduTriggeringFilterRefs                                              = 2184,
    /// TDLET-ZONE-CLOCK
    TdletZoneClock                                                         = 1451,
    /// TEAR-DOWN-ACTION-REF
    TearDownActionRef                                                      = 4292,
    /// TEAR-DOWN-ACTION-REFS
    TearDownActionRefs                                                     = 3035,
    /// TEMPERATURE-EXP
    TemperatureExp                                                         = 4674,
    /// TEMPLATE-ARGUMENTS
    TemplateArguments                                                      = 3700,
    /// TEMPLATE-TYPE-REF
    TemplateTypeRef                                                        = 6381,
    /// TERMINATING-CHECKPOINT-REF
    TerminatingCheckpointRef                                               = 3831,
    /// TERMINATING-CHECKPOINT-TIMEOUT-UNTIL-TERMINATION
    TerminatingCheckpointTimeoutUntilTermination                           = 5119,
    /// TERMINATION-BEHAVIOR
    TerminationBehavior                                                    = 4448,
    /// TEST-ID
    TestId                                                                 = 4069,
    /// TEST-ID-REF
    TestIdRef                                                              = 1023,
    /// TEST-IDENTIFIER
    TestIdentifier                                                         = 1795,
    /// TEST-PATTERN
    TestPattern                                                            = 1799,
    /// TEST-RESULT-REF
    TestResultRef                                                          = 408,
    /// TESTED-ITEM-REF
    TestedItemRef                                                          = 5592,
    /// TESTED-ITEM-REFS
    TestedItemRefs                                                         = 4144,
    /// TEX-MATH
    TexMath                                                                = 3131,
    /// TEXT
    Text                                                                   = 3624,
    /// TEXT-TABLE-MAPPING
    TextTableMapping                                                       = 6104,
    /// TEXT-TABLE-MAPPINGS
    TextTableMappings                                                      = 1288,
    /// TEXT-TABLE-VALUE-PAIR
    TextTableValuePair                                                     = 2674,
    /// TEXT-VALUE-SPECIFICATION
    TextValueSpecification                                                 = 1653,
    /// TEXTUAL-CONDITION
    TextualCondition                                                       = 4664,
    /// TFOOT
    Tfoot                                                                  = 389,
    /// TGROUP
    Tgroup                                                                 = 2382,
    /// THEAD
    Thead                                                                  = 1314,
    /// THRESHOLD
    Threshold                                                              = 391,
    /// THRESHOLD-NUMBER
    ThresholdNumber                                                        = 2474,
    /// THRESHOLD-SIZE
    ThresholdSize                                                          = 4250,
    /// TIME-BASE
    TimeBase                                                               = 2078,
    /// TIME-BASE-CONSUMER-REF
    TimeBaseConsumerRef                                                    = 171,
    /// TIME-BASE-JITTER
    TimeBaseJitter                                                         = 138,
    /// TIME-BASE-KIND
    TimeBaseKind                                                           = 1694,
    /// TIME-BASE-PROVIDER-REF
    TimeBaseProviderRef                                                    = 3395,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 8,
    /// TIME-BASE-RESOURCE-REF
    TimeBaseResourceRef                                                    = 3181,
    /// TIME-BASE-RESOURCE-REF-CONDITIONAL
    TimeBaseResourceRefConditional                                         = 557,
    /// TIME-BASE-RESOURCE-REFS
    TimeBaseResourceRefs                                                   = 2438,
    /// TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE
    TimeBasedFdcThresholdStorageValue                                      = 3689,
    /// TIME-BASES
    TimeBases                                                              = 1432,
    /// TIME-BR
    TimeBr                                                                 = 4581,
    /// TIME-BUFFER
    TimeBuffer                                                             = 4117,
    /// TIME-CS
    TimeCs                                                                 = 6262,
    /// TIME-EXP
    TimeExp                                                                = 4879,
    /// TIME-FAILED-THRESHOLD
    TimeFailedThreshold                                                    = 6378,
    /// TIME-FR-IF
    TimeFrIf                                                               = 6236,
    /// TIME-HARDWARE-CORRECTION-THRESHOLD
    TimeHardwareCorrectionThreshold                                        = 4800,
    /// TIME-INTERVAL
    TimeInterval                                                           = 6055,
    /// TIME-LEAP-FUTURE-THRESHOLD
    TimeLeapFutureThreshold                                                = 3817,
    /// TIME-LEAP-HEALING-COUNTER
    TimeLeapHealingCounter                                                 = 3969,
    /// TIME-LEAP-PAST-THRESHOLD
    TimeLeapPastThreshold                                                  = 581,
    /// TIME-MARK
    TimeMark                                                               = 561,
    /// TIME-MASTER-PRIORITY
    TimeMasterPriority                                                     = 5429,
    /// TIME-OFFSET
    TimeOffset                                                             = 302,
    /// TIME-PASSED-THRESHOLD
    TimePassedThreshold                                                    = 5742,
    /// TIME-PERIOD
    TimePeriod                                                             = 6430,
    /// TIME-SEG-1
    TimeSeg1                                                               = 3939,
    /// TIME-SEG-2
    TimeSeg2                                                               = 3995,
    /// TIME-SUB-TLV
    TimeSubTlv                                                             = 1732,
    /// TIME-SYNC-CLIENT
    TimeSyncClient                                                         = 3842,
    /// TIME-SYNC-CORRECTION
    TimeSyncCorrection                                                     = 176,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 1661,
    /// TIME-SYNC-P-PORT-PROTOTYPE-IREF
    TimeSyncPPortPrototypeIref                                             = 770,
    /// TIME-SYNC-PORT-PROTOTYPE-IREF
    TimeSyncPortPrototypeIref                                              = 562,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 1815,
    /// TIME-SYNC-R-PORT-PROTOTYPE-IREF
    TimeSyncRPortPrototypeIref                                             = 6432,
    /// TIME-SYNC-SERVER
    TimeSyncServer                                                         = 4737,
    /// TIME-SYNC-SERVER-IDENTIFIER
    TimeSyncServerIdentifier                                               = 4422,
    /// TIME-SYNC-SERVER-REF
    TimeSyncServerRef                                                      = 6139,
    /// TIME-SYNC-TECHNOLOGY
    TimeSyncTechnology                                                     = 2941,
    /// TIME-SYNCHRONIZATION
    TimeSynchronization                                                    = 5476,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 2557,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 3930,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 160,
    /// TIME-TO-LIVE
    TimeToLive                                                             = 51,
    /// TIME-TRIGGERED-CAN-LEVEL
    TimeTriggeredCanLevel                                                  = 3750,
    /// TIME-VALUE
    TimeValue                                                              = 4459,
    /// TIME-VALUE-VALUE-VARIATION-POINT
    TimeValueValueVariationPoint                                           = 3270,
    /// TIMEOUT
    Timeout                                                                = 1156,
    /// TIMEOUT-AR
    TimeoutAr                                                              = 4891,
    /// TIMEOUT-AS
    TimeoutAs                                                              = 613,
    /// TIMEOUT-BR
    TimeoutBr                                                              = 3379,
    /// TIMEOUT-BS
    TimeoutBs                                                              = 5597,
    /// TIMEOUT-CHECK
    TimeoutCheck                                                           = 4403,
    /// TIMEOUT-CR
    TimeoutCr                                                              = 6442,
    /// TIMEOUT-CS
    TimeoutCs                                                              = 2191,
    /// TIMEOUT-SUBSTITUTION-VALUE
    TimeoutSubstitutionValue                                               = 2863,
    /// TIMESTAMP-FORMAT
    TimestampFormat                                                        = 4945,
    /// TIMESTAMP-INTERVAL
    TimestampInterval                                                      = 3426,
    /// TIMESTAMP-RX-ACCEPTANCE-WINDOW
    TimestampRxAcceptanceWindow                                            = 2935,
    /// TIMESTAMP-SUPPORT
    TimestampSupport                                                       = 486,
    /// TIMING-ARGUMENT-REF
    TimingArgumentRef                                                      = 2220,
    /// TIMING-ARGUMENTS
    TimingArguments                                                        = 1832,
    /// TIMING-CLOCK-SYNC-ACCURACY
    TimingClockSyncAccuracy                                                = 1915,
    /// TIMING-CLOCK-SYNC-ACCURACYS
    TimingClockSyncAccuracys                                               = 1802,
    /// TIMING-CLOCKS
    TimingClocks                                                           = 2782,
    /// TIMING-CONDITION
    TimingCondition                                                        = 4616,
    /// TIMING-CONDITION-FORMULA
    TimingConditionFormula                                                 = 4624,
    /// TIMING-CONDITION-REF
    TimingConditionRef                                                     = 5184,
    /// TIMING-CONDITIONS
    TimingConditions                                                       = 2407,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 1499,
    /// TIMING-DESCRIPTION-REF
    TimingDescriptionRef                                                   = 810,
    /// TIMING-DESCRIPTIONS
    TimingDescriptions                                                     = 3102,
    /// TIMING-EVENT
    TimingEvent                                                            = 2907,
    /// TIMING-EVENT-REF
    TimingEventRef                                                         = 6036,
    /// TIMING-GUARANTEES
    TimingGuarantees                                                       = 590,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 1827,
    /// TIMING-MODE-REF
    TimingModeRef                                                          = 2681,
    /// TIMING-MODES
    TimingModes                                                            = 750,
    /// TIMING-PROPS
    TimingProps                                                            = 3874,
    /// TIMING-REQUIREMENTS
    TimingRequirements                                                     = 2793,
    /// TIMING-RESOURCE
    TimingResource                                                         = 326,
    /// TIMING-VARIABLE-REF
    TimingVariableRef                                                      = 3555,
    /// TIMING-VARIABLES
    TimingVariables                                                        = 4836,
    /// TLS-CIPHER-SUITE
    TlsCipherSuite                                                         = 5272,
    /// TLS-CIPHER-SUITES
    TlsCipherSuites                                                        = 2600,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 1233,
    /// TLS-CONNECTION-REF
    TlsConnectionRef                                                       = 3431,
    /// TLS-CONNECTION-REFS
    TlsConnectionRefs                                                      = 2981,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 6452,
    /// TLS-CRYPTO-MAPPING-REF
    TlsCryptoMappingRef                                                    = 292,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 2937,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 5599,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 485,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 4665,
    /// TLS-JOB-MAPPINGS
    TlsJobMappings                                                         = 4554,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 4797,
    /// TLS-JOB-REQUIREMENT-REF
    TlsJobRequirementRef                                                   = 2563,
    /// TLS-JOB-SEMANTIC
    TlsJobSemantic                                                         = 3253,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 2135,
    /// TLS-SECURE-COM-PROPS-REF
    TlsSecureComPropsRef                                                   = 6405,
    /// TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF
    TlvAppRecordElementInMethodIref                                        = 3262,
    /// TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    TlvAppRecordElementInServiceInterfaceIref                              = 783,
    /// TLV-ARGUMENT-REF
    TlvArgumentRef                                                         = 4570,
    /// TLV-DATA-ID
    TlvDataId                                                              = 6127,
    /// TLV-DATA-ID-0-REF
    TlvDataId0Ref                                                          = 5486,
    /// TLV-DATA-ID-0-REFS
    TlvDataId0Refs                                                         = 1684,
    /// TLV-DATA-ID-DEFINITION
    TlvDataIdDefinition                                                    = 149,
    /// TLV-DATA-ID-DEFINITION-REF
    TlvDataIdDefinitionRef                                                 = 3424,
    /// TLV-DATA-ID-DEFINITION-REFS
    TlvDataIdDefinitionRefs                                                = 1241,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 4864,
    /// TLV-DATA-ID-DEFINITIONS
    TlvDataIdDefinitions                                                   = 3894,
    /// TLV-DATA-IDS
    TlvDataIds                                                             = 2705,
    /// TLV-IMPL-RECORD-ELEMENT
    TlvImplRecordElement                                                   = 5864,
    /// TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TlvImplementationDataTypeElementRef                                    = 5054,
    /// TLV-RECORD-ELEMENT-REF
    TlvRecordElementRef                                                    = 3394,
    /// TLV-SUB-ELEMENT-REF
    TlvSubElementRef                                                       = 5236,
    /// TO-REF
    ToRef                                                                  = 736,
    /// TOL
    Tol                                                                    = 67,
    /// TOLERANCE
    Tolerance                                                              = 406,
    /// TOLERATED-FAILED-CYCLES
    ToleratedFailedCycles                                                  = 3938,
    /// TOOL
    Tool                                                                   = 4779,
    /// TOOL-VERSION
    ToolVersion                                                            = 2175,
    /// TOPIC-1
    Topic1                                                                 = 1405,
    /// TOPIC-DATA
    TopicData                                                              = 5489,
    /// TOPIC-NAME
    TopicName                                                              = 150,
    /// TP-ADDRESS
    TpAddress                                                              = 1688,
    /// TP-ADDRESS-EXTENSION-VALUE
    TpAddressExtensionValue                                                = 3498,
    /// TP-ADDRESS-REF
    TpAddressRef                                                           = 1333,
    /// TP-ADDRESS-REFS
    TpAddressRefs                                                          = 3879,
    /// TP-ADDRESSS
    TpAddresss                                                             = 4025,
    /// TP-CHANNEL-REF
    TpChannelRef                                                           = 1578,
    /// TP-CHANNELS
    TpChannels                                                             = 4100,
    /// TP-CONFIGURATION
    TpConfiguration                                                        = 3783,
    /// TP-CONNECTION-CONTROL-REF
    TpConnectionControlRef                                                 = 6019,
    /// TP-CONNECTION-CONTROLS
    TpConnectionControls                                                   = 4009,
    /// TP-CONNECTION-REF
    TpConnectionRef                                                        = 3751,
    /// TP-CONNECTION-REFS
    TpConnectionRefs                                                       = 1791,
    /// TP-CONNECTIONS
    TpConnections                                                          = 5050,
    /// TP-ECU
    TpEcu                                                                  = 2065,
    /// TP-ECUS
    TpEcus                                                                 = 88,
    /// TP-NODES
    TpNodes                                                                = 2289,
    /// TP-PGS
    TpPgs                                                                  = 5569,
    /// TP-PROTOCOL-TYPE
    TpProtocolType                                                         = 3804,
    /// TP-SDU-REF
    TpSduRef                                                               = 4635,
    /// TP-SDU-REFS
    TpSduRefs                                                              = 4853,
    /// TP-TECHNOLOGY
    TpTechnology                                                           = 3070,
    /// TRACE
    Trace                                                                  = 5006,
    /// TRACE-MESSAGE-REF
    TraceMessageRef                                                        = 99,
    /// TRACE-REF
    TraceRef                                                               = 5834,
    /// TRACE-REFS
    TraceRefs                                                              = 4153,
    /// TRACE-SWITCH
    TraceSwitch                                                            = 2299,
    /// TRACE-SWITCH-CONFIGURATION
    TraceSwitchConfiguration                                               = 1513,
    /// TRACE-SWITCH-CONFIGURATIONS
    TraceSwitchConfigurations                                              = 5490,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 2749,
    /// TRACED-FAILURES
    TracedFailures                                                         = 3968,
    /// TRAFFIC-CLASS
    TrafficClass                                                           = 3263,
    /// TRAFFIC-CLASS-PREEMPTION-SUPPORT
    TrafficClassPreemptionSupport                                          = 2167,
    /// TRAFFIC-LIMITATION-FILTERS
    TrafficLimitationFilters                                               = 6140,
    /// TRAFFIC-SHAPER-GROUP-REF
    TrafficShaperGroupRef                                                  = 6172,
    /// TRAFFIC-SHAPER-GROUPS
    TrafficShaperGroups                                                    = 5188,
    /// TRANCEIVER-STANDBY-DELAY
    TranceiverStandbyDelay                                                 = 5270,
    /// TRANSFER-EXIT-CLASS-REF
    TransferExitClassRef                                                   = 2858,
    /// TRANSFER-PROPERTY
    TransferProperty                                                       = 4539,
    /// TRANSFER-REF
    TransferRef                                                            = 2403,
    /// TRANSFER-REFS
    TransferRefs                                                           = 5871,
    /// TRANSFERS
    Transfers                                                              = 4557,
    /// TRANSFORMATION-COM-SPEC-PROPSS
    TransformationComSpecPropss                                            = 2105,
    /// TRANSFORMATION-DESCRIPTIONS
    TransformationDescriptions                                             = 3062,
    /// TRANSFORMATION-I-SIGNAL-PROPSS
    TransformationISignalPropss                                            = 4444,
    /// TRANSFORMATION-PROPS-MAPPING-SET-REF
    TransformationPropsMappingSetRef                                       = 4456,
    /// TRANSFORMATION-PROPS-REF
    TransformationPropsRef                                                 = 1221,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 765,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 4587,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 1784,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING
    TransformationPropsToServiceInterfaceMapping                           = 6070,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 2255,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF
    TransformationPropsToServiceInterfaceMappingSetRef                     = 5349,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS
    TransformationPropsToServiceInterfaceMappings                          = 5943,
    /// TRANSFORMATION-PROPSS
    TransformationPropss                                                   = 1469,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 5400,
    /// TRANSFORMATION-TECHNOLOGYS
    TransformationTechnologys                                              = 5515,
    /// TRANSFORMER-CHAIN-REF
    TransformerChainRef                                                    = 758,
    /// TRANSFORMER-CHAIN-REFS
    TransformerChainRefs                                                   = 3404,
    /// TRANSFORMER-CLASS
    TransformerClass                                                       = 6212,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 280,
    /// TRANSFORMER-REF
    TransformerRef                                                         = 4347,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 5935,
    /// TRANSFORMING-SYSTEM-SIGNAL-REF
    TransformingSystemSignalRef                                            = 4330,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 5600,
    /// TRANSIT-TO-INVALID-EXTENDED
    TransitToInvalidExtended                                               = 1907,
    /// TRANSITION-REF
    TransitionRef                                                          = 2837,
    /// TRANSITION-REFS
    TransitionRefs                                                         = 2345,
    /// TRANSITIONS
    Transitions                                                            = 6419,
    /// TRANSLATED-VLAN-ID
    TranslatedVlanId                                                       = 5371,
    /// TRANSLATION-TARGET-IREF
    TranslationTargetIref                                                  = 6243,
    /// TRANSMISSION-ACKNOWLEDGE
    TransmissionAcknowledge                                                = 2124,
    /// TRANSMISSION-DEADLINE
    TransmissionDeadline                                                   = 3361,
    /// TRANSMISSION-MODE
    TransmissionMode                                                       = 5564,
    /// TRANSMISSION-MODE-CONDITION
    TransmissionModeCondition                                              = 1556,
    /// TRANSMISSION-MODE-CONDITIONS
    TransmissionModeConditions                                             = 3351,
    /// TRANSMISSION-MODE-DECLARATION
    TransmissionModeDeclaration                                            = 3956,
    /// TRANSMISSION-MODE-FALSE-TIMING
    TransmissionModeFalseTiming                                            = 1838,
    /// TRANSMISSION-MODE-TRUE-TIMING
    TransmissionModeTrueTiming                                             = 2603,
    /// TRANSMISSION-PROPS
    TransmissionProps                                                      = 2164,
    /// TRANSMISSION-START-SEQUENCE-DURATION
    TransmissionStartSequenceDuration                                      = 1433,
    /// TRANSMISSION-TRIGGER
    TransmissionTrigger                                                    = 3081,
    /// TRANSMIT-CANCELLATION
    TransmitCancellation                                                   = 5245,
    /// TRANSMIT-PDU-REF
    TransmitPduRef                                                         = 2329,
    /// TRANSMIT-PDU-REFS
    TransmitPduRefs                                                        = 5770,
    /// TRANSMITTER-REF
    TransmitterRef                                                         = 5156,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 1606,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 4613,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF
    TransportLayerIndependentInstanceIdRef                                 = 374,
    /// TRANSPORT-LAYER-RULE
    TransportLayerRule                                                     = 2477,
    /// TRANSPORT-PDU-REF
    TransportPduRef                                                        = 2803,
    /// TRANSPORT-PLUGIN
    TransportPlugin                                                        = 965,
    /// TRANSPORT-PLUGINS
    TransportPlugins                                                       = 4161,
    /// TRANSPORT-PRIORITY
    TransportPriority                                                      = 5443,
    /// TRANSPORT-PROTOCOL
    TransportProtocol                                                      = 3797,
    /// TRANSPORT-PROTOCOLS
    TransportProtocols                                                     = 3966,
    /// TRCV-DELAY-COMPENSATION-OFFSET
    TrcvDelayCompensationOffset                                            = 1115,
    /// TRCV-PWM-MODE-ENABLED
    TrcvPwmModeEnabled                                                     = 702,
    /// TRIGGER
    Trigger                                                                = 1633,
    /// TRIGGER-DIRECT-IMPLEMENTATIONS
    TriggerDirectImplementations                                           = 4660,
    /// TRIGGER-I-PDU-SEND-CONDITION
    TriggerIPduSendCondition                                               = 4375,
    /// TRIGGER-I-PDU-SEND-CONDITIONS
    TriggerIPduSendConditions                                              = 4720,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 1622,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 5024,
    /// TRIGGER-IREF
    TriggerIref                                                            = 4589,
    /// TRIGGER-MAPPING
    TriggerMapping                                                         = 1740,
    /// TRIGGER-MAPPINGS
    TriggerMappings                                                        = 5896,
    /// TRIGGER-MODE
    TriggerMode                                                            = 2504,
    /// TRIGGER-PERIOD
    TriggerPeriod                                                          = 95,
    /// TRIGGER-PORT-ANNOTATION
    TriggerPortAnnotation                                                  = 2877,
    /// TRIGGER-PORT-ANNOTATIONS
    TriggerPortAnnotations                                                 = 3166,
    /// TRIGGER-REF
    TriggerRef                                                             = 5117,
    /// TRIGGER-REF-CONDITIONAL
    TriggerRefConditional                                                  = 3216,
    /// TRIGGER-REFS
    TriggerRefs                                                            = 5527,
    /// TRIGGER-TO-SIGNAL-MAPPING
    TriggerToSignalMapping                                                 = 4596,
    /// TRIGGERING-EVENT-REF
    TriggeringEventRef                                                     = 1703,
    /// TRIGGERS
    Triggers                                                               = 6128,
    /// TRIGGERS-RECOVERY-NOTIFICATION
    TriggersRecoveryNotification                                           = 3115,
    /// TROUBLE-CODE-J-1939-REF
    TroubleCodeJ1939Ref                                                    = 3374,
    /// TROUBLE-CODE-OBD-REF
    TroubleCodeObdRef                                                      = 1853,
    /// TROUBLE-CODE-PROPSS
    TroubleCodePropss                                                      = 4793,
    /// TROUBLE-CODE-UDS-REF
    TroubleCodeUdsRef                                                      = 5842,
    /// TRUE-ACTION-LIST-REF
    TrueActionListRef                                                      = 2874,
    /// TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR
    TrustedPlatformExecutableLaunchBehavior                                = 337,
    /// TT
    Tt                                                                     = 2075,
    /// TT-CAN-CLUSTER-REF
    TtCanClusterRef                                                        = 3285,
    /// TTCAN-ABSOLUTELY-SCHEDULED-TIMING
    TtcanAbsolutelyScheduledTiming                                         = 5808,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 1437,
    /// TTCAN-CLUSTER-CONDITIONAL
    TtcanClusterConditional                                                = 3553,
    /// TTCAN-CLUSTER-VARIANTS
    TtcanClusterVariants                                                   = 2762,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 2399,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 3139,
    /// TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    TtcanCommunicationControllerConditional                                = 4824,
    /// TTCAN-COMMUNICATION-CONTROLLER-VARIANTS
    TtcanCommunicationControllerVariants                                   = 6316,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 3446,
    /// TTL
    Ttl                                                                    = 5331,
    /// TTL-MAX
    TtlMax                                                                 = 2846,
    /// TTL-MIN
    TtlMin                                                                 = 2553,
    /// TWO-KEY-SLOT-MODE
    TwoKeySlotMode                                                         = 6302,
    /// TX-BIT-RATE-SWITCH
    TxBitRateSwitch                                                        = 3767,
    /// TX-ENABLE-WINDOW-LENGTH
    TxEnableWindowLength                                                   = 392,
    /// TX-MASK
    TxMask                                                                 = 3621,
    /// TX-NM-PDU-REF
    TxNmPduRef                                                             = 6202,
    /// TX-NM-PDU-REFS
    TxNmPduRefs                                                            = 933,
    /// TX-PDU-POOL-REF
    TxPduPoolRef                                                           = 6095,
    /// TX-PDU-TRIGGERING-REF
    TxPduTriggeringRef                                                     = 3019,
    /// TYP
    Typ                                                                    = 5251,
    /// TYPE
    Type                                                                   = 2747,
    /// TYPE-APPROVAL
    TypeApproval                                                           = 446,
    /// TYPE-BLUEPRINTS
    TypeBlueprints                                                         = 1820,
    /// TYPE-DEFINITION
    TypeDefinition                                                         = 1421,
    /// TYPE-EMITTER
    TypeEmitter                                                            = 4228,
    /// TYPE-MAPPING
    TypeMapping                                                            = 1377,
    /// TYPE-OF-DTC-SUPPORTED
    TypeOfDtcSupported                                                     = 3721,
    /// TYPE-OF-EVENT-COMBINATION-SUPPORTED
    TypeOfEventCombinationSupported                                        = 2157,
    /// TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION
    TypeOfFreezeFrameRecordNumeration                                      = 6138,
    /// TYPE-REFERENCE
    TypeReference                                                          = 3383,
    /// TYPE-REFERENCE-REF
    TypeReferenceRef                                                       = 3294,
    /// TYPE-TAILORINGS
    TypeTailorings                                                         = 4418,
    /// TYPE-TREF
    TypeTref                                                               = 2421,
    /// UAS-ID
    UasId                                                                  = 3246,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 726,
    /// UCM-MASTER-FALLBACK-REF
    UcmMasterFallbackRef                                                   = 256,
    /// UCM-MASTER-FALLBACK-REFS
    UcmMasterFallbackRefs                                                  = 5043,
    /// UCM-MASTER-MODULE-INSTANTIATION
    UcmMasterModuleInstantiation                                           = 3528,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 4116,
    /// UCM-MODULE-INSTANTIATION-REF
    UcmModuleInstantiationRef                                              = 1025,
    /// UCM-NOT-AVAILABLE-ON-THE-NETWORK
    UcmNotAvailableOnTheNetwork                                            = 1700,
    /// UCM-PROCESSINGS
    UcmProcessings                                                         = 4399,
    /// UCM-REF
    UcmRef                                                                 = 2892,
    /// UCM-SOFTWARE-VERSION
    UcmSoftwareVersion                                                     = 4758,
    /// UCM-STEP
    UcmStep                                                                = 1492,
    /// UCM-SUBORDINATE-MODULE-INSTANTIATION
    UcmSubordinateModuleInstantiation                                      = 686,
    /// UCM-TO-TIME-BASE-RESOURCE-MAPPING
    UcmToTimeBaseResourceMapping                                           = 4567,
    /// UCMS
    Ucms                                                                   = 6156,
    /// UDP-CHECKSUM-HANDLING
    UdpChecksumHandling                                                    = 377,
    /// UDP-COLLECTION-BUFFER-SIZE-THRESHOLD
    UdpCollectionBufferSizeThreshold                                       = 5851,
    /// UDP-COLLECTION-BUFFER-TIMEOUT
    UdpCollectionBufferTimeout                                             = 4917,
    /// UDP-COLLECTION-TRIGGER
    UdpCollectionTrigger                                                   = 2711,
    /// UDP-LOCAL-PORT
    UdpLocalPort                                                           = 1599,
    /// UDP-MIN-TX-BUFFER-SIZE
    UdpMinTxBufferSize                                                     = 1961,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 4437,
    /// UDP-NM-CLUSTER-COUPLING
    UdpNmClusterCoupling                                                   = 4145,
    /// UDP-NM-CLUSTER-REF
    UdpNmClusterRef                                                        = 1840,
    /// UDP-NM-ECU
    UdpNmEcu                                                               = 4083,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 1278,
    /// UDP-PORT
    UdpPort                                                                = 5990,
    /// UDP-PORT-REF
    UdpPortRef                                                             = 1889,
    /// UDP-PROPS
    UdpProps                                                               = 4157,
    /// UDP-REMOTE-PORT
    UdpRemotePort                                                          = 5160,
    /// UDP-RULE
    UdpRule                                                                = 2847,
    /// UDP-TP
    UdpTp                                                                  = 1047,
    /// UDP-TP-PORT
    UdpTpPort                                                              = 1120,
    /// UDP-TTL
    UdpTtl                                                                 = 4304,
    /// UDS-DTC-NUMBER
    UdsDtcNumber                                                           = 6277,
    /// UDS-DTC-VALUE
    UdsDtcValue                                                            = 241,
    /// UDS-SERVICE
    UdsService                                                             = 4169,
    /// UNASSIGN-FRAME-ID
    UnassignFrameId                                                        = 3143,
    /// UNASSIGNED-FRAME-TRIGGERING-REF
    UnassignedFrameTriggeringRef                                           = 386,
    /// UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE
    UncompressedSoftwareClusterSize                                        = 1904,
    /// UNICAST-CREDENTIALS
    UnicastCredentials                                                     = 1826,
    /// UNICAST-NETWORK-ENDPOINT-REF
    UnicastNetworkEndpointRef                                              = 718,
    /// UNICAST-NETWORK-ENDPOINT-REFS
    UnicastNetworkEndpointRefs                                             = 3195,
    /// UNICAST-SECURE-COM-PROPS-REF
    UnicastSecureComPropsRef                                               = 1272,
    /// UNICAST-SECURE-COM-PROPS-REFS
    UnicastSecureComPropsRefs                                              = 4895,
    /// UNICAST-UDP-CREDENTIALS
    UnicastUdpCredentials                                                  = 3630,
    /// UNIQUE-STREAM-ID
    UniqueStreamId                                                         = 4742,
    /// UNIT
    Unit                                                                   = 990,
    /// UNIT-AND-SCALING-ID
    UnitAndScalingId                                                       = 1586,
    /// UNIT-DISPLAY-NAME
    UnitDisplayName                                                        = 2882,
    /// UNIT-GROUP
    UnitGroup                                                              = 4546,
    /// UNIT-GROUP-REF
    UnitGroupRef                                                           = 4822,
    /// UNIT-GROUP-REFS
    UnitGroupRefs                                                          = 407,
    /// UNIT-REF
    UnitRef                                                                = 6081,
    /// UNIT-REFS
    UnitRefs                                                               = 3076,
    /// UNLIMITED-INTEGER-VALUE-VARIATION-POINT
    UnlimitedIntegerValueVariationPoint                                    = 4411,
    /// UNRESOLVED-REFERENCE-RESTRICTION
    UnresolvedReferenceRestriction                                         = 725,
    /// UNUSED-BIT-PATTERN
    UnusedBitPattern                                                       = 855,
    /// UPDATE
    Update                                                                 = 6364,
    /// UPDATE-INDICATION-BIT-POSITION
    UpdateIndicationBitPosition                                            = 6268,
    /// UPDATE-KIND
    UpdateKind                                                             = 3095,
    /// UPDATE-SESSION-REJECTED
    UpdateSessionRejected                                                  = 755,
    /// UPDATE-STRATEGY
    UpdateStrategy                                                         = 2021,
    /// UPPER-BOUND
    UpperBound                                                             = 2720,
    /// UPPER-BOUNDARY
    UpperBoundary                                                          = 5366,
    /// UPPER-CAN-ID
    UpperCanId                                                             = 2821,
    /// UPPER-HEADER-BITS-TO-SHIFT
    UpperHeaderBitsToShift                                                 = 3282,
    /// UPPER-LIMIT
    UpperLimit                                                             = 3991,
    /// UPPER-MULTIPLICITY
    UpperMultiplicity                                                      = 1345,
    /// UPPER-MULTIPLICITY-INFINITE
    UpperMultiplicityInfinite                                              = 3279,
    /// UPPER-MULTIPLICITY-IS-INFINITE
    UpperMultiplicityIsInfinite                                            = 2776,
    /// UPPER-REF
    UpperRef                                                               = 2133,
    /// UPSTREAM-REFERENCE-IREF
    UpstreamReferenceIref                                                  = 2769,
    /// URI
    Uri                                                                    = 329,
    /// URL
    Url                                                                    = 5752,
    /// USAGE
    Usage                                                                  = 2151,
    /// USAGES
    Usages                                                                 = 1594,
    /// USE-AS-CRYPTOGRAPHIC-I-PDU
    UseAsCryptographicIPdu                                                 = 3989,
    /// USE-AUTH-DATA-FRESHNESS
    UseAuthDataFreshness                                                   = 1916,
    /// USE-AUTO-VALIDATION-AT-SHUT-DOWN
    UseAutoValidationAtShutDown                                            = 6253,
    /// USE-CASE
    UseCase                                                                = 4388,
    /// USE-CLIENT-AUTHENTICATION-REQUEST
    UseClientAuthenticationRequest                                         = 3851,
    /// USE-CRC-COMP-MECHANISM
    UseCrcCompMechanism                                                    = 917,
    /// USE-FRESHNESS-TIMESTAMP
    UseFreshnessTimestamp                                                  = 2010,
    /// USE-INSTEAD-REF
    UseInsteadRef                                                          = 5657,
    /// USE-INSTEAD-REFS
    UseInsteadRefs                                                         = 1150,
    /// USE-MAC-ADDRESS-FOR-IDENTIFICATION
    UseMacAddressForIdentification                                         = 1431,
    /// USE-SECURED-PDU-HEADER
    UseSecuredPduHeader                                                    = 698,
    /// USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT
    UseSecurityExtensionRecordSizeLimit                                    = 3447,
    /// USE-SMART-SENSOR-API
    UseSmartSensorApi                                                      = 3114,
    /// USE-VARIABLE-DA
    UseVariableDa                                                          = 2390,
    /// USE-VARIABLE-SA
    UseVariableSa                                                          = 5434,
    /// USE-VEHICLE-IDENTIFICATION-SYNC-STATUS
    UseVehicleIdentificationSyncStatus                                     = 2186,
    /// USED-CODE-GENERATOR
    UsedCodeGenerator                                                      = 3101,
    /// USED-DATA-ELEMENT
    UsedDataElement                                                        = 443,
    /// USED-FID-REF
    UsedFidRef                                                             = 3693,
    /// USED-IMPLEMENTATION-DATA-TYPE-REF
    UsedImplementationDataTypeRef                                          = 2539,
    /// USED-IMPOSITION-TIME-DEFINITION-GROUP-REF
    UsedImpositionTimeDefinitionGroupRef                                   = 5169,
    /// USED-LANGUAGES
    UsedLanguages                                                          = 5447,
    /// USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF
    UsedLifeCycleStateDefinitionGroupRef                                   = 632,
    /// USED-PARAMETER-ELEMENT
    UsedParameterElement                                                   = 1386,
    /// USED-PIM-REF
    UsedPimRef                                                             = 4354,
    /// USED-SECONDARY-FID-REF
    UsedSecondaryFidRef                                                    = 3924,
    /// USED-SECONDARY-FID-REFS
    UsedSecondaryFidRefs                                                   = 4368,
    /// USER-DATA-SUB-TLV
    UserDataSubTlv                                                         = 3455,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 948,
    /// USER-DEFINED-CLUSTER-CONDITIONAL
    UserDefinedClusterConditional                                          = 4599,
    /// USER-DEFINED-CLUSTER-VARIANTS
    UserDefinedClusterVariants                                             = 4662,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 6300,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 2657,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL
    UserDefinedCommunicationControllerConditional                          = 3039,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS
    UserDefinedCommunicationControllerVariants                             = 685,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 1002,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 2020,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 2104,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 4038,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 4579,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 4350,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 6227,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 289,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 5818,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 3215,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 4241,
    /// USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS
    UserDefinedTransformationComSpecProps                                  = 1420,
    /// USER-DEFINED-TRANSFORMATION-DESCRIPTION
    UserDefinedTransformationDescription                                   = 5109,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS
    UserDefinedTransformationISignalProps                                  = 94,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    UserDefinedTransformationISignalPropsConditional                       = 4803,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    UserDefinedTransformationISignalPropsVariants                          = 5694,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 4949,
    /// USER-REF
    UserRef                                                                = 5672,
    /// USER-REFS
    UserRefs                                                               = 4787,
    /// USES-ADDRESS-ARBITRATION
    UsesAddressArbitration                                                 = 5740,
    /// USES-DYNAMIC-ADDRESSING
    UsesDynamicAddressing                                                  = 5821,
    /// USES-END-TO-END-PROTECTION
    UsesEndToEndProtection                                                 = 2788,
    /// USES-MONITOR-DATA
    UsesMonitorData                                                        = 3680,
    /// USES-TIME-BASE-RESOURCE
    UsesTimeBaseResource                                                   = 3887,
    /// V
    V                                                                      = 3538,
    /// V-2-X-DATA-MANAGER-NEEDS
    V2XDataManagerNeeds                                                    = 5114,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 3863,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 6154,
    /// V-2-X-SUPPORTED
    V2XSupported                                                           = 5080,
    /// V-LAN-REF
    VLanRef                                                                = 2713,
    /// V-LAN-REFS
    VLanRefs                                                               = 6118,
    /// VALID-BINDING-TIME
    ValidBindingTime                                                       = 5467,
    /// VALID-BINDING-TIMES
    ValidBindingTimes                                                      = 1027,
    /// VALIDATION-CONFIGURATION
    ValidationConfiguration                                                = 1436,
    /// VALIDATION-FORMULA
    ValidationFormula                                                      = 5603,
    /// VALIDATION-REF
    ValidationRef                                                          = 4391,
    /// VALIDATION-ROOT
    ValidationRoot                                                         = 422,
    /// VALUE
    Value                                                                  = 2602,
    /// VALUE-ACCESS
    ValueAccess                                                            = 5912,
    /// VALUE-AXIS-DATA-TYPE-REF
    ValueAxisDataTypeRef                                                   = 1771,
    /// VALUE-CONFIG-CLASSES
    ValueConfigClasses                                                     = 4027,
    /// VALUE-DATA-TYPE-REF
    ValueDataTypeRef                                                       = 2572,
    /// VALUE-IREF
    ValueIref                                                              = 3870,
    /// VALUE-PAIRS
    ValuePairs                                                             = 197,
    /// VALUE-REF
    ValueRef                                                               = 2115,
    /// VALUE-RESTRICTION
    ValueRestriction                                                       = 5664,
    /// VALUE-SPEC
    ValueSpec                                                              = 5588,
    /// VALUE-TYPE-REF
    ValueTypeRef                                                           = 1292,
    /// VALUE-TYPE-TREF
    ValueTypeTref                                                          = 3976,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 3686,
    /// VARIABLE-ACCESS-IREF
    VariableAccessIref                                                     = 4416,
    /// VARIABLE-ACCESS-IREFS
    VariableAccessIrefs                                                    = 2112,
    /// VARIABLE-ACCESS-REF
    VariableAccessRef                                                      = 4652,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REF
    VariableAccessedForDebugRef                                            = 5046,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REFS
    VariableAccessedForDebugRefs                                           = 158,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 1125,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 3291,
    /// VARIABLE-DATA-PROTOTYPE-IREF
    VariableDataPrototypeIref                                              = 476,
    /// VARIABLE-INSTANCE
    VariableInstance                                                       = 3477,
    /// VARIABLE-INSTANCE-IREF
    VariableInstanceIref                                                   = 281,
    /// VARIABLE-LENGTH
    VariableLength                                                         = 3903,
    /// VARIABLE-REF
    VariableRef                                                            = 3880,
    /// VARIABLES
    Variables                                                              = 2653,
    /// VARIANT-CRITERION-REF
    VariantCriterionRef                                                    = 2037,
    /// VARIANT-ID
    VariantId                                                              = 2017,
    /// VARIATION
    Variation                                                              = 5012,
    /// VARIATION-POINT
    VariationPoint                                                         = 1266,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 6112,
    /// VARIATION-POINT-PROXYS
    VariationPointProxys                                                   = 5183,
    /// VARIATION-RESTRICTION
    VariationRestriction                                                   = 5780,
    /// VCID
    Vcid                                                                   = 4571,
    /// VEHICLE-ANNOUNCEMENT-COUNT
    VehicleAnnouncementCount                                               = 2230,
    /// VEHICLE-ANNOUNCEMENT-INTERVAL
    VehicleAnnouncementInterval                                            = 4907,
    /// VEHICLE-DESCRIPTION-REF
    VehicleDescriptionRef                                                  = 131,
    /// VEHICLE-DRIVER-NOTIFICATION
    VehicleDriverNotification                                              = 2168,
    /// VEHICLE-IDENTIFICATION-SYNC-STATUS
    VehicleIdentificationSyncStatus                                        = 5456,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 2923,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 5068,
    /// VEHICLE-SYSTEM
    VehicleSystem                                                          = 2912,
    /// VEHICLE-SYSTEM-INSTANCE
    VehicleSystemInstance                                                  = 5605,
    /// VENDOR
    Vendor                                                                 = 5794,
    /// VENDOR-API-INFIX
    VendorApiInfix                                                         = 2174,
    /// VENDOR-ID
    VendorId                                                               = 505,
    /// VENDOR-SIGNATURE-REF
    VendorSignatureRef                                                     = 6105,
    /// VENDOR-SPECIFIC-MODULE-DEF-REF
    VendorSpecificModuleDefRef                                             = 1263,
    /// VENDOR-SPECIFIC-MODULE-DEF-REFS
    VendorSpecificModuleDefRefs                                            = 3676,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 3916,
    /// VERBATIM
    Verbatim                                                               = 6272,
    /// VERIFICATION-HASH
    VerificationHash                                                       = 5448,
    /// VERIFICATION-METHOD
    VerificationMethod                                                     = 2397,
    /// VERIFICATION-STATUS-INDICATION-MODE
    VerificationStatusIndicationMode                                       = 4844,
    /// VERIFY-IREF
    VerifyIref                                                             = 6330,
    /// VERIFY-IREFS
    VerifyIrefs                                                            = 4930,
    /// VERIFY-REF
    VerifyRef                                                              = 6327,
    /// VERIFY-REFS
    VerifyRefs                                                             = 2315,
    /// VERIFY-UPDATE
    VerifyUpdate                                                           = 677,
    /// VERSION
    Version                                                                = 1817,
    /// VERSION-DRIVEN-FIND-BEHAVIOR
    VersionDrivenFindBehavior                                              = 3862,
    /// VF
    Vf                                                                     = 14,
    /// VFB-TIMING
    VfbTiming                                                              = 463,
    /// VFC-IREF
    VfcIref                                                                = 2503,
    /// VFC-IREFS
    VfcIrefs                                                               = 1315,
    /// VG
    Vg                                                                     = 6333,
    /// VIEW-MAP
    ViewMap                                                                = 156,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 883,
    /// VIEW-MAPS
    ViewMaps                                                               = 2668,
    /// VIN-INVALIDITY-PATTERN
    VinInvalidityPattern                                                   = 1608,
    /// VIOLATED-SAFETY-CONDITION-BEHAVIOR
    ViolatedSafetyConditionBehavior                                        = 3722,
    /// VLAN
    Vlan                                                                   = 2357,
    /// VLAN-ID
    VlanId                                                                 = 5618,
    /// VLAN-IDENTIFIER
    VlanIdentifier                                                         = 2202,
    /// VLAN-MEMBERSHIP
    VlanMembership                                                         = 2370,
    /// VLAN-MEMBERSHIPS
    VlanMemberships                                                        = 3440,
    /// VLAN-MODIFIER-REF
    VlanModifierRef                                                        = 3566,
    /// VLAN-PRIORITY
    VlanPriority                                                           = 4680,
    /// VLAN-REF
    VlanRef                                                                = 1857,
    /// VLAN-REFS
    VlanRefs                                                               = 2110,
    /// VLAN-TRANSLATION-TABLES
    VlanTranslationTables                                                  = 4643,
    /// VT
    Vt                                                                     = 82,
    /// VTF
    Vtf                                                                    = 2751,
    /// WAIT-POINT
    WaitPoint                                                              = 2326,
    /// WAIT-POINTS
    WaitPoints                                                             = 4409,
    /// WAKE-UP-BY-CONTROLLER-SUPPORTED
    WakeUpByControllerSupported                                            = 3384,
    /// WAKE-UP-CHANNEL
    WakeUpChannel                                                          = 2228,
    /// WAKE-UP-OVER-BUS-SUPPORTED
    WakeUpOverBusSupported                                                 = 277,
    /// WAKE-UP-PATTERN
    WakeUpPattern                                                          = 4109,
    /// WAKEUP-FORWARD-LOCAL-ENABLED
    WakeupForwardLocalEnabled                                              = 2348,
    /// WAKEUP-FORWARD-REMOTE-ENABLED
    WakeupForwardRemoteEnabled                                             = 3028,
    /// WAKEUP-FRAME-REF
    WakeupFrameRef                                                         = 5747,
    /// WAKEUP-FRAME-REFS
    WakeupFrameRefs                                                        = 5967,
    /// WAKEUP-LOCAL-DETECTION-TIME
    WakeupLocalDetectionTime                                               = 818,
    /// WAKEUP-LOCAL-DURATION-TIME
    WakeupLocalDurationTime                                                = 724,
    /// WAKEUP-LOCAL-ENABLED
    WakeupLocalEnabled                                                     = 5471,
    /// WAKEUP-REMOTE-ENABLED
    WakeupRemoteEnabled                                                    = 4490,
    /// WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST
    WakeupRepetitionDelayOfWakeupRequest                                   = 2954,
    /// WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST
    WakeupRepetitionsOfWakeupRequest                                       = 5151,
    /// WAKEUP-RX-IDLE
    WakeupRxIdle                                                           = 4125,
    /// WAKEUP-RX-LOW
    WakeupRxLow                                                            = 5142,
    /// WAKEUP-RX-WINDOW
    WakeupRxWindow                                                         = 1573,
    /// WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF
    WakeupSleepOnDatalineConfigRef                                         = 996,
    /// WAKEUP-SYMBOL-RX-WINDOW
    WakeupSymbolRxWindow                                                   = 3748,
    /// WAKEUP-TX-ACTIVE
    WakeupTxActive                                                         = 5204,
    /// WAKEUP-TX-IDLE
    WakeupTxIdle                                                           = 353,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 5397,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 2152,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 1370,
    /// WINDOW-SIZE
    WindowSize                                                             = 3669,
    /// WINDOW-SIZE-INIT
    WindowSizeInit                                                         = 5301,
    /// WINDOW-SIZE-INVALID
    WindowSizeInvalid                                                      = 1598,
    /// WINDOW-SIZE-VALID
    WindowSizeValid                                                        = 1841,
    /// WITH-AUTO
    WithAuto                                                               = 6294,
    /// WORST-CASE-EXECUTION-TIME
    WorstCaseExecutionTime                                                 = 5792,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 4669,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 2913,
    /// WRITE
    Write                                                                  = 5748,
    /// WRITE-ACCESS
    WriteAccess                                                            = 4226,
    /// WRITE-CLASS-REF
    WriteClassRef                                                          = 2490,
    /// WRITE-ONLY-ONCE
    WriteOnlyOnce                                                          = 4889,
    /// WRITE-VERIFICATION
    WriteVerification                                                      = 1788,
    /// WRITER-ENTITY-ID
    WriterEntityId                                                         = 384,
    /// WRITING-FREQUENCY
    WritingFrequency                                                       = 3162,
    /// WRITING-PRIORITY
    WritingPriority                                                        = 997,
    /// WRITING-STRATEGY-ROLE
    WritingStrategyRole                                                    = 1615,
    /// WRITING-STRATEGYS
    WritingStrategys                                                       = 5465,
    /// WRITTEN-LOCAL-VARIABLES
    WrittenLocalVariables                                                  = 3731,
    /// WRITTEN-NV-DATA
    WrittenNvData                                                          = 2766,
    /// WRITTEN-READ-NV-DATA
    WrittenReadNvData                                                      = 6121,
    /// WWH-OBD-DTC-CLASS
    WwhObdDtcClass                                                         = 6116,
    /// X
    X                                                                      = 5872,
    /// XCP-PDU
    XcpPdu                                                                 = 4047,
    /// XDOC
    Xdoc                                                                   = 1190,
    /// XFILE
    Xfile                                                                  = 3265,
    /// XREF
    Xref                                                                   = 1542,
    /// XREF-TARGET
    XrefTarget                                                             = 4821,
}

impl ElementName {
    const STRING_TABLE: [&'static str; 6459] = ["I-PDU-IDENTIFIER-TCP-REF", "APMC-INSTANCE-REFERENCE-DEF", "LIN-TP-NODE", "IMPLEMENTATION-DATA-TYPE-REF", "RPT-HOOKS", "NM-CONTROL-BIT-VECTOR-ACTIVE", "LOG-SINK-REF", "FINAL-CHECKPOINT-REF", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "MODE-ACCESS-POINT", "NESTED-ELEMENT-REFS", "SD-SERVER-CONFIG-REF", "AH-CIPHER-SUITE-NAME", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "VF", "CONDITIONS", "DIAGNOSTIC-TROUBLE-CODE-OBD", "ECUC-MODULE-DEF", "RUNTIME-ERROR", "MASS-EXP", "LOWER-BOUNDARY", "SW-COMPONENT-ASSIGNMENTS", "DO-IP-LOGIC-ADDRESS", "CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF", "MANUFACTURER-VALIDATION-ORDER-REF", "ACTION-LIST", "SERVICE-INTERFACE-TREF", "SUPPORT-INFO-BIT", "FORBIDDEN-SIGNAL-PATH", "MEMORY-HIGH-ADDRESS", "PRECONFIGURED-CONFIGURATION-REF", "PASS-THROUGH-SW-CONNECTOR", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "IO-HW-ABSTRACTION-SERVER-ANNOTATION", "MATCHING-INGRESS-RULE-REFS", "DIAGNOSTIC-SOVD-METHOD-PRIMITIVE", "DEFAULT-LC-STATE-REF", "NESTED-ELEMENTS", "MAX-NUMBER-OF-TIME-QUANTA-PER-BIT", "CAN-XL-PROPS", "BEHAVIOR", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "BIT-POSITION", "SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "NETWORK-ENDPOINT-REFS", "MATCHING-INGRESS-RULE-REF", "META-DATA-ITEMS", "REPLICA-PDUS-REF", "HW-ELEMENT", "MAX-RUN-CYCLE-LOOP", "APPLICATION-SW-COMPONENT-TYPE", "TIME-TO-LIVE", "NM-REPEAT-MESSAGE-BIT-ACTIVE", "DID-NUMBER", "STATUS-BIT-STORAGE-TEST-FAILED", "ECUC-QUERY-STRING-REF", "BIT-OFFSET", "SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA", "CLIENT-SERVER-INTERFACE-MAPPING", "NM-CBV-POSITION", "DATA-DUMP-ENTRY", "MC-GROUP", "FORMULA-CAPTION", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "SELECTOR-FIELD-CODE", "CONTENT-TYPE", "SOCKET-ADDRESS-REF", "TOL", "CHANNEL-MODE", "CONTEXT-BSW-IMPLEMENTATION-REF", "IDS-TIME-BASE-REFS", "FM-ATTRIBUTE-DEF", "BLOCK-IF-STATE-ACTIVE-AP-IREF", "FREEZE-FRAME-CONTENT-WWH-OBD-REF", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "EVENT-GROUP-ID", "SERVICE-TIMING", "SELECT-BLOCK-FOR-FIRST-INIT-ALL", "NM-CLUSTERS", "PAYLOADS", "MODE-DECLARATION-MAPPINGS", "IDENTICAL-MAPPING", "VT", "DESTINATION-CONTEXT", "LET-INTERVAL-REF", "HARDWARE-ELEMENT-REF", "PDU-ACTIVATION-ROUTING-GROUP", "SD-SERVER-CONFIG", "TP-ECUS", "AGE-CONSTRAINT", "IPV-6-ADDRESS-SOURCE", "FMI", "DDS-EVENT-DEPLOYMENT", "SWC-INTERNAL-BEHAVIOR", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS", "TRIGGER-PERIOD", "KEY-SLOT-CONTENT-ALLOWED-USAGES", "ECU-CONFIGURATION-REF", "CHECKPOINT-REF", "TRACE-MESSAGE-REF", "COLLISION-RESOLVING-SCHEDULE-REF", "SWC-BSW-RUNNABLE-MAPPING", "ALL-CHANNELS-REF", "MULTIPLE-CONFIGURATION-CONTAINER", "PNC-FILTER-ARRAY-MASK", "CONSUMED-EVENT-GROUP", "BIT-NUMBER", "RESPONSE-GROUP-REF", "SEC-OC-JOB-REQUIREMENT", "CHECKPOINT-FINAL-REFS", "STATUS-IREF", "NO-CHECKPOINT-SUPERVISIONS", "RESUME-POSITION", "SW-DATA-DEF-PROPS", "RTE-RESOURCE-ESTIMATION", "IPV-4-MULTICAST-IP-ADDRESS", "ECU-TASK-PROXY-REF", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "DESTINATION-IPV-6-ADDRESS", "DIAGNOSTIC-INDICATOR", "BASE-TYPE-SIZE", "HW-CATEGORY", "OPT-ARGUMENT-REF", "RECORD-VALUE-SPECIFICATION", "EXECUTION-ORDER-CONSTRAINT-TYPE", "APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION", "ECUC-TEXTUAL-PARAM-VALUE", "SWC-RUNNABLE-REF", "DEF-CALPRM-SET", "TARGET-REF", "LOCAL-SUPERVISIONS", "VEHICLE-DESCRIPTION-REF", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "PATH-MTU-ENABLED", "MIN-NUMBER-OF-OCCURRENCES", "PORT-INTERFACE-REF", "CRYPTO-ALGORITHM-ID", "DDS-METHOD-REPLY-TOPIC-REF", "TIME-BASE-JITTER", "SYNCHRONIZATION-TIMING-CONSTRAINT", "MAX-NUMBER-OF-CONNECTIONS", "NM-REPEAT-MESSAGE-BIT-ENABLE", "INHIBITION-MASK", "FUNCTION-IDENTIFIER-REF", "HOST-REF", "COMPU-INTERNAL-TO-PHYS", "EXECUTION-CONTEXTS", "ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS", "SOMEIP-REQUIRED-EVENT-GROUP", "TLV-DATA-ID-DEFINITION", "TOPIC-NAME", "CPU-USAGE", "SOMEIP-METHOD-DEPLOYMENT", "CANCELLATION", "SOURCE-ADDRESS-CODE", "AFFECTED-STATE-IREF", "VIEW-MAP", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "VARIABLE-ACCESSED-FOR-DEBUG-REFS", "PROVIDED-PORT-PROTOTYPE-IREF", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "CAN-FRAME-RX-BEHAVIOR", "SEGMENT-REFS", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "KEY-SLOT-REF", "PERSISTENCY-REDUNDANCY-HASH", "SYNC-DATA-ID-LISTS", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "SW-TEST-DESCS", "CURRENT-EXP", "LIN-EVENT-TRIGGERED-FRAME", "TIME-BASE-CONSUMER-REF", "POSSIBLE-ERROR-REACTIONS", "MODE-DECLARATIONS", "DERIVED-PORT-INTERFACE-REF", "ENABLES-LOG-TRACE", "TIME-SYNC-CORRECTION", "SDF", "I-PDU-PORT-REFS", "DLT-ARGUMENTS", "PROCESSING-STYLE", "TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED", "HANDLE-INVALID", "COMPOUND-PRIMITIVE-ARGUMENTS", "REST-SYSTEM-TRIGGERED-EVENT", "COMPU-METHOD", "PRE-ACTIVATE-IREFS", "COMMUNICATION-MODE", "CAN-FD-CONFIG", "CLIENT-ID-DEFINITION", "SOURCE-MIN-ADDRESS", "I-PDU-TRIGGERING", "SIGNAL-BASED-METHOD-REF", "STARTUP-OPTIONS", "ETHERNET-FRAME-TRIGGERING", "BINARY-MANIFEST-META-DATA-FIELD", "SOURCE-IPV-6-ADDRESS", "VALUE-PAIRS", "DATA-ELEMENTS", "DIAGNOSTIC-READ-DTC-INFORMATION", "ACL-OBJECT-CLASS", "SEND-POLICYS", "SLEEP-MODE-SUPPORTED", "SEGMENT-LENGTH", "TCP-IP-ICMP-PROPS-REF", "INITIAL-CHECKPOINT-REF", "MAX-REQUEST-BYTES", "DIAGNOSTIC-SESSION-CONTROL", "GETTER-OPERATION-REF", "BSW-MODULE-DEPENDENCYS", "MSR-QUERY-P-2", "AUTO-COLLECT", "RECEIVER-CAPABILITY", "REQUEST-SEED-ID", "SW-BIT-REPRESENTATION", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "RESPOND-TO-RESET", "TCP-TP-PORT", "SYSTEM-MAPPING", "DDS-QOS-PROFILE-REF", "TARGET-P-PORT-PROTOTYPE-REF", "CONSTANT-REF", "SETTER-RETURN-REF", "PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL", "COMBINED-NO-DATA-INIT-COUNT", "SIGNAL-BASED-EVENT-REF", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "DDS-OPERATION-RESPONSE-TRIGGERING-REF", "SUPPORTED-CIPHER-SUITES", "ACL-PERMISSION", "ETH-IP-PROPS-REF", "MODULE-REFS", "LOCAL-PORT-RANGE-START", "LOWER-BOUND", "COUPLING-ELEMENT-DETAILS", "SW-GENERIC-AXIS-DESC", "MODE-IN-SWC-INSTANCE-REF", "PROVIDED-MODE-GROUPS", "RECOVERABLE-IN-SAME-OPERATION-CYCLE", "AUTH-ALGORITHM", "NV-DATA-PORT-ANNOTATION", "UDS-DTC-VALUE", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "NRC-VALUE", "RPT-ENABLER-IMPL-TYPE", "MAXIMUM-LENGTH", "DDS-I-SIGNAL-TO-TOPIC-MAPPINGS", "SERVICE-RESOURCE-REF", "MAX-PROGRESSION-MISMATCH-THRESHOLD", "SW-ARRAYSIZE", "SSP-OFFSET", "COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION", "DIAGNOSTIC-CLEAR-CONDITION", "SHALL-RUN-ON-REF", "CODE", "SW-RECORD-LAYOUT-V-INDEX", "UCM-MASTER-FALLBACK-REF", "TARGET-MODE-GROUP-REF", "SYNC-TIME-BASE-MGR-USER-NEEDS", "APPLICABILITY-INFO-SET", "ECUC-ENUMERATION-LITERAL-DEF", "DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING", "DIAGNOSTIC-MESSAGE-TYPE", "PWM-L", "LIVELINESS-PROTECTION-KIND", "MIN-SOURCE-PORT-NUMBER", "DIAGNOSTIC-VALUE-NEEDS", "TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE", "APPLICATION-COMPOSITE-ELEMENT-IREF", "IDSM-INSTANCES", "METHOD-DEPLOYMENTS", "DURABILITY-SERVICE-HISTORY-KIND", "DETECT-NIT-ERROR", "PROVIDED-DATA-REF", "M", "COUPLING-PORT-REF", "CONTAINED-PACKAGE-ELEMENT-REF", "WAKE-UP-OVER-BUS-SUPPORTED", "OBD-DTC-VALUE", "NM-CONTROL-BIT-VECTOR-ENABLED", "TRANSFORMER-HARD-ERROR-EVENT", "VARIABLE-INSTANCE-IREF", "ETHERNET-CLUSTER-REF", "SW-COMPONENT-DOCUMENTATION", "SUBNET-NUMBER", "LABELED-ITEM", "DDS-RPC-SERVICES", "SELECTOR-FIELD-START-POSITION", "INSTANTIATION-RTE-EVENT-PROPSS", "USER-DEFINED-PDU", "NV-BLOCK-NEEDS", "LOCAL-NETWORK-ENDPOINT-REF", "TLS-CRYPTO-MAPPING-REF", "PATH", "CONTEXT-R-PORT-REF", "AUTHENTICATION-ROLE-REF", "ENTRY", "REPORTING-PORT-PROTOTYPE-IREF", "CAN-TP-CONFIG", "SW-VARIABLE-IMPL", "META-DATA-FIELDS", "BASE-COMPOSITION-REF", "TIME-OFFSET", "OPERATION-CYCLE", "SSRC", "SW-CARB-DOC", "RESOURCE-IDENTIFIER-TYPE", "DIAGNOSTIC-EVENT-NEEDS", "GLOBAL-ELEMENTS", "FILE-NAME", "END-TO-END-TRANSFORMATION-COM-SPEC-PROPS", "AUTH-ROLE-REFS", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "APPLICATION-DESCRIPTION", "TCP-IP-DHCP-V-6-CNF-DELAY-MIN", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS", "APPLICATIONS", "COMPU-SCALES", "NUMBER-OF-BITS", "COUPLING-PORT-SPEED", "DIAGNOSTIC-VALUE-ACCESS", "RVF-PIXEL-FORMAT", "REMOTE-UNICAST-ADDRESSS", "SUB-SDG-REF", "SDU-REF", "RAM-BLOCK-INIT-VALUE", "TIMING-RESOURCE", "PROPS", "IGNORE-AFTER-TX", "URI", "KEY-SERVER-PRIORITY", "REMARK", "EXCLUSIVE-AREA-REFS", "MIXED-BUS-TYPE-COLLECTION", "NAME", "ADDRESSING-FORMAT", "IMPOSITION-TIME-DEFINITION-GROUP", "TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR", "SECURITY-EVENT-CONTEXT-PROPS", "SENDER-IREF", "STD-REFS", "MEMORY-SECTIONS", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "ECU-PARTITION", "ACTION", "ASSIGNED-PORTS", "MAC-SEC-KAY-PARTICIPANT", "SIZE", "DERIVED-FROM-BLUEPRINT-REFS", "STANDARD-REVISIONS", "COM-FIELD-GRANT", "PARAMETER-PORT-ANNOTATIONS", "WAKEUP-TX-IDLE", "CRYPTO-NEED", "POSSIBLE-AP-ERROR-REFS", "APMC-CONTAINER-REFERENCE-DEF", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "FR-GLOBAL-TIME-DOMAIN-PROPS", "NM-NETWORK-HANDLE", "IMPLEMENTATION-REFS", "STARTUP-CONFIG-REF", "MAX-NUMBER-OF-NPDU-PER-CYCLE", "ASSIGN-NAD", "APPLICATION-VALUE-SPECIFICATION", "SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "SW-ALIGNMENT", "LIN-PID-TO-CAN-ID-MAPPINGS", "PNC-PREPARE-SLEEP-TIMER", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE", "MAX-LENGTH", "FIRST-PHYSICAL-DIMENSION-REF", "SYNCHRONIZED-MASTER-TIME-BASE", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF", "STOP", "I-SIGNAL-IN-I-PDU-REF", "UDP-CHECKSUM-HANDLING", "EID-RETRIEVAL", "STD-REF", "MIN-VALUE", "LOG-TRACE-PROCESS-DESC", "ACCESS-POINT-REF", "BLUEPRINT-MAPPING", "WRITER-ENTITY-ID", "COMPU-DENOMINATOR", "UNASSIGNED-FRAME-TRIGGERING-REF", "BROADCAST", "LENGTH", "TFOOT", "DIAGNOSTIC-RESPONSE-ON-EVENT", "THRESHOLD", "TX-ENABLE-WINDOW-LENGTH", "NAGLES-ALGORITHM", "E-2-E-PROFILE-CONFIGURATION-REF", "SECOND-APPLICATION-ERROR-REF", "SUPERVISION-REF", "SEED-SIZE", "DIAGNOSTIC-COMPONENT-NEEDS", "ECUC-MODULE-CONFIGURATION-VALUES", "MODE-DEPENDENT-STARTUP-CONFIGS", "SWITCH-STREAM-IDENTIFICATIONS", "LIVENESS-KIND", "ADDRESS", "PROCESSOR", "LIN-SLAVES", "TOLERANCE", "UNIT-GROUP-REFS", "TEST-RESULT-REF", "DESTINATION-REFS", "NV-BLOCK-SW-COMPONENT-TYPE", "DIAGNOSTIC-COM-CONTROL", "SOURCE-LIN-PID-REF", "MODULE-DESCRIPTION-REF", "RESOURCE-GROUP-REFS", "BASIC-CYCLE-LENGTH", "SERVICE-REQUEST-CALLBACK-TYPE", "PACKING-BYTE-ORDER", "CRYPTO-KEY-DESCRIPTION", "CONTEXT-PORT-PROTOTYPE-REF", "IEEE-1722-ACF-BUS-RAW-DATA-STREAM-CONSUMER-MAPPING", "SOMEIP-METHOD", "VALIDATION-ROOT", "CONTEXT-P-PORT-REF", "MACRO-PER-CYCLE", "EXITED-MODE-REF", "FUNCTION-NAMES", "PORT-INTERFACE-BLUEPRINT-REF", "KEY-SLOTS", "SYSTEM-SIGNAL-GROUP-REF", "ACK-REQUEST", "REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF", "PHM-ACTION", "POST-BUILD-VARIANT-CRITERION", "ARTIFACT-LOCATORS", "DIAGNOSTIC-EVENT-WINDOW", "FIELD-MAPPINGS", "DO-IP-GID-NEEDS", "TARGET-DATA-PROTOTYPE-GROUP-REF", "COM-FIND-SERVICE-GRANT", "REFERENCE-TAILORING", "COUPLING-ELEMENT-SWITCH-DETAILS", "EVALUATED-ELEMENT-REF", "USED-DATA-ELEMENT", "ROLE-BASED-DATA-TYPE-ASSIGNMENT", "SOFTWARE-CLUSTER-DEPENDENCY-FORMULA", "TYPE-APPROVAL", "SAMPLES-PER-MICROTICK", "NUMBER-OF-REPETITIONS", "DCM-I-PDU", "DLT-LOG-SINK-REFS", "TCP-TP-CONFIG", "NETWORK-IDLE-TIME", "BLOCK-STATES", "HANDLE-NEVER-RECEIVED", "ENABLE-LIVELINESS-PROTECTION", "SDU-TYPE", "CHECKPOINT-TRANSITION", "GATEWAYS", "ACCESSED-PARAMETER", "PORT-INTERFACE-BLUEPRINT-MAPPING", "DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING", "DIAGNOSTIC-PARAMETER", "VFB-TIMING", "SECURITY-EVENT-PROPS-REF", "RESOURCE-CONSUMPTION", "DLT-CONTEXT-REF-CONDITIONAL", "DATA-ELEMENT-IREFS", "SECURITY-EVENT-REFS", "LOCAL-IP-SEC-RULE-REF", "REST-NUMBER-PROPERTY-DEF", "NM-MSG-CYCLE-OFFSET", "FILE-PROXYS", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "RX-PDU-TRIGGERING-REF", "VARIABLE-DATA-PROTOTYPE-IREF", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "COLLECTION", "SW-VALUES-PHYS", "ACTUAL-EVENT-REF", "E-2-E-EVENT-PROTECTION-PROPSS", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "BSW-TRIGGER-REF", "DIAGNOSTIC-CONNECTION", "TLS-IAM-REMOTE-SUBJECT", "TIMESTAMP-SUPPORT", "SW-AXIS-CONTS", "ROM-LOCATION-REF", "INTERPOLATION-ROUTINE-MAPPING-SET-REFS", "DELEGATED-PORT-ANNOTATION", "LOCAL-UDP-PORT-REF", "ETH-SWITCH-PORT-GROUP-DERIVATION", "EXPECTED-ENTRYS", "INTERRUPT-CATEGORY", "MICRO-INITIAL-OFFSET-A", "SYMBOLIC-NAME-VALUE", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "HW-ELEMENT-CONNECTOR", "COUPLING-PORT-CONNECTIONS", "OS-ARTI-ADAPTER-LAUNCH-BEHAVIOR", "DO-IP-ENTITY-ROLE", "DIAGNOSTIC-OPERATION-CYCLE", "HW-PIN-GROUP-CONNECTOR", "VENDOR-ID", "STATIC-SOCKET-CONNECTIONS", "TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS", "NM-PDU-RX-INDICATION-ENABLED", "BSW-PER-INSTANCE-MEMORY-POLICYS", "APPLICATION-RECORD-ELEMENT", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "BUS-ID", "LOC-MEASUREMENT-SET", "ARBITRARY-EVENT-TRIGGERING", "SERVICE-INTERFACE-ELEMENT-MAPPING-REF", "MAPPED-SECURITY-EVENTS", "LOGICAL-EXPRESSION", "DIAGNOSTIC-DTC-CHANGE-TRIGGER", "IS-PRIVATE", "I-SIGNAL-GROUP-REF", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL", "NOMINAL", "CONFIGURATION-CLASS-AFFECTION", "FUNCTION-GROUP-STATE-IREF", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "PARAMETER-ID", "MIN-RPT-EVENT-ID", "NM-PDU-REF", "SUBTITLE", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "SERVICE-KIND", "SA-OVER-TIME", "CONSTANT-SPECIFICATION-MAPPING-SET", "NUMERICAL-VALUE", "CHANNELS-PER-FRAME", "NM-MSG-CYCLE-TIME", "RPT-EXECUTABLE-ENTITY", "DATA-READ-ACCESSS", "DATA-PROTOTYPE-TRANSFORMATION-PROPSS", "STATE-MANAGEMEN-PHM-ERROR-INTERFACE", "OFS-DATA-ID-LIST", "STATE-MANAGEMENT-REQUEST-TRIGGER", "SUPERVISION-CHECKPOINT", "ACL-OPERATION-REF", "SDG-AGGREGATION-WITH-VARIATION", "COMMENT", "LIN-MASTER-CONDITIONAL", "PHYSICAL-REQUEST-REF", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "IUMPR-GROUP", "PROVIDED-DATAS", "CRYPTO-KEY-SLOT", "REQUIRED-PACKAGE-ELEMENT-REFS", "TCP-CONNECT-TIMEOUT", "SCHEDULE-CHANGE-NEXT-TIME-BASE", "DIAGNOSTIC-EVENT", "TIME-BASE-RESOURCE-REF-CONDITIONAL", "MAX-FC-WAIT", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "SUPPORT-BUFFER-LOCKING", "TIME-MARK", "TIME-SYNC-PORT-PROTOTYPE-IREF", "NM-MAIN-FUNCTION-ACROSS-FR-CYCLE", "PHM-RULE", "LIN-MASTER", "KEY-EXCHANGE-REFS", "LIFE-CYCLE-INFO-SET", "NAS-TIMEOUT", "HW-PIN-CONNECTOR", "MATCHING-EGRESS-RULE-REFS", "CRYPTO-SERVICE-MAPPINGS", "DATA-LENGTH-RESPONSE", "NM-DATA-DISABLED", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "ALIAS-NAME-SET", "FIFO-DEPTH", "CONFIG-VARIANT", "SOURCE-EVENT-REF", "MODE-DECLARATION-MAPPING-SET", "TIME-LEAP-PAST-THRESHOLD", "REQUEST-EMISSION-RELATED-DTC-CLASS-REF", "MIN-DESTINATION-PORT-NUMBER", "KEY-SLOT-ONLY-ENABLED", "BASE-TYPE-ENCODING", "CIPHER-SUITE-NAME", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "CLIENT-SERVER-RECORD-TYPE-MAPPING", "CONTAINER-TRIGGER", "TIMING-GUARANTEES", "FUNCTION-PROTOTYPE-EMITTER", "N-ROM-BLOCKS", "FREEZE-FRAMES", "CLIENT-ID-DEFINITION-SET-REFS", "BYTE-VALUES", "SW-SYSTEMCONSTANT-VALUE-SET-REFS", "SHARED-PARAMETERS", "NETWORK-CONFIGURATION", "OWNER-REFS", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "REST-ENDPOINT-POST", "MINIMUM-NUMBER-OF-REQUESTS", "INDIRECT-API", "ALL-PHYSICAL-CHANNELS", "SETTER-CALL-REF", "START-ADDRESS", "SYMBOL-WINDOW", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "IDS-PLATFORM-INSTANTIATION-REF", "DO-IP-NETWORK-CONFIGURATION-DESIGN", "MIN-SERVICE-POINT-ID", "TCP-IP-ICMP-V-4-TTL", "TIMEOUT-AS", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "SECURE-COM-PROPS-FOR-UDP-REFS", "SYSTEM-MEMORY-USAGES", "CONTEXT-ROOT-SW-COMPOSITION-REF", "BLUEPRINT-POLICY-LIST", "AUTHENTICATION-PROPS-REF", "KEY-REF", "MINIMUM-SUPPORTED-UCM-VERSION", "FULLY-QUALIFIED-DOMAIN-NAME", "ECU-RESOURCE-ESTIMATION", "RECEIVER-REF", "COMPOSITE-NETWORK-REPRESENTATIONS", "TARGET-MODULE-REFS", "POST-BUILD-VARIANT-CRITERION-VALUE", "PROCESS-STATE-IREF", "FLOAT-VALUE-VARIATION-POINT", "RESOURCE-GUARD-VALUE", "HW-PIN-GROUP-CONTENT", "USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF", "REMOTE-ACCESS-CONTROL-ENABLED", "DEFAULT-VALUE", "DIAGNOSTIC-DATA-CHANGE-TRIGGER", "COLLECTION-REF", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "FIELD-MAPPING", "STREAM-IDENTIFICATION-HANDLE-REF", "MAX-NUMBER-OF-RESP-PENDING-FRAMES", "TARGET-EVENT-REF", "SERIALIZER", "FM-FEATURE-MAP-ASSERTION", "GENERIC-TP", "BINARY-MANIFEST-ITEM-DEFINITION", "REFERENCED-TD-EVENT-SWC-REF", "INTERPOLATION-ROUTINES", "KEEP-ALIVE-INTERVAL", "REQUIRED-CLIENT-SERVER-ENTRYS", "COUPLING-PORT-RATE-POLICY", "SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "DESTINATION-URI-REFS", "DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL", "REQUIRED-EVENT-GROUPS", "MACHINE-TIMING", "IMMEDIATE-NV-DATA-STORAGE", "COLSPEC", "SO-CON-I-PDU-IDENTIFIER", "CONTEXT-SW-COMPONENT-PROTOTYPE-REF", "RPT-PREPARATION-LEVEL", "REMOTE-PORT-REF", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "COM-TRIGGER-GRANT-DESIGN", "SECURITY-EVENT-FILTER-CHAIN-REF", "EXT-HEADER-FILTER-LISTS", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "PNC-GROUP-REFS", "MIN-OK-STATE-INIT", "TARGET-DATA-PROTOTYPE-REF", "CRF-TYPE", "DIAGNOSTIC-CONTRIBUTION-REFS", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "MEASURED-STACK-USAGE", "PROVIDER-REF", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "VERIFY-UPDATE", "FLEXRAY-NM-CLUSTER", "APPLICATION-MODE-MACHINE", "ETS-AVAILABLE-BANDWIDTH-IN-PERCENT", "BSW-PER-INSTANCE-MEMORY-POLICY", "PER-INSTANCE-PARAMETER-REF", "DTCS", "METHOD-TOPICS-ACCESS-RULE-REF", "USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS", "UCM-SUBORDINATE-MODULE-INSTANTIATION", "KEY-IDENTITY-REF", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "HW-TYPE", "PN-RESET-TIME", "MAX-NUMBER-OF-ALLOWED-UPDATES", "EOC-EVENT-REF", "PROP-SEG", "ALLOWED-SERVICE-CONSUMERS", "IEEE-1722-TP-ETHERNET-FRAME", "AGGREGATION-TAILORING", "POST-BUILD-CHANGEABLE", "USE-SECURED-PDU-HEADER", "DID-SIZE", "PACKAGE-REF", "METHOD-RESPONSE-PROPSS", "TRCV-PWM-MODE-ENABLED", "IMPOSITION-TIMES", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "PID-REF", "ARRAY-VALUE-SPECIFICATION", "RPT-COMPONENT", "SOURCE-INSTANCE-IREFS", "LIN-SLAVE-ECU-REF", "FLAT-OBJECT-REF", "IIDC-CHANNEL", "CRC-CORRECTION-FIELD", "ETHERNET-COMMUNICATION-CONTROLLER", "MIN-OK-STATE-INVALID", "MEMORY-CONSUMPTION", "INVOCATION", "TARGET-PHM-CHECKPOINT-REF", "UNICAST-NETWORK-ENDPOINT-REF", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "REST-BOOLEAN-PROPERTY-DEF", "MIN", "IS-DEFAULT", "SOCKET-CONNECTION-REF", "WAKEUP-LOCAL-DURATION-TIME", "UNRESOLVED-REFERENCE-RESTRICTION", "UCM-DESCRIPTION", "ECUC-DESTINATION-URI-DEF", "SWC-BSW-MAPPING", "SERVICE-INTERFACE-TRIGGER-MAPPING", "DATA-PROTOTYPE-IN-METHOD-ARGUMENT-INSTANCE-REF-IREF", "GROUP-NUMBER", "CONSUMED-DDS-SERVICE-EVENTS", "DATA-TYPE-REFS", "DDS-PARTITION-REF", "PDU-REF", "TO-REF", "NUMBER-OF-INSTRUCTIONS", "SIZE-OF-UNION-TYPE-SELECTOR-FIELD", "FIELD-NOTIFIER-REF", "HW-ATTRIBUTE-LITERAL-DEF", "BURST-SIZE-REQUEST", "PROCESS-TO-MACHINE-MAPPING", "HEALING-CYCLE-COUNTER-THRESHOLD", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "RUNNABLE-MAPPINGS", "I-SIGNAL-TRIGGERING-REF", "INTERPOLATION-ROUTINE", "SW-COMPONENT-PROTOTYPE-ASSIGNMENT", "IDSM-REPORTING-MODE-PROVIDER-MAPPING", "TIMING-MODES", "ECU-REF", "DETERMINISTIC-CLIENT", "ACF-PARTS", "SYSTEM-SIGNAL", "UPDATE-SESSION-REJECTED", "CRYPTO-PROVIDER-INTERFACE", "NODE-PORTS", "TRANSFORMER-CHAIN-REF", "BUNDLED-CONNECTIONS", "TARGET-PARAMETER-DATA-PROTOTYPE-REF", "MAXIMUM-ALLOWED-SIZE", "ARBITRATION", "CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING", "GENERATED-ARTIFACTS", "TRANSFORMATION-PROPS-SET", "RESOURCE-LIMITS", "SECTION-INITIALIZATION-POLICY", "BUS-MIRROR-NETWORK-ID", "SOCKET-CONNECTION", "TIME-SYNC-P-PORT-PROTOTYPE-IREF", "TCP-IP-NDP-PACKET-QUEUE-ENABLED", "DIAGNOSTIC-MEMORY-IDENTIFIER", "SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF", "METHOD-RETURN-REF", "SERVICE-SW-COMPONENT-TYPE", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "CALIBRATION-PARAMETER-VALUE-SET-REF", "SESSION-CONTROL-CLASS-REF", "DYNAMIC-PART-ALTERNATIVE", "APMC-CHOICE-CONTAINER-DEF", "SECONDARY-FRESHNESS-VALUE-ID", "DIAGNOSTIC-SOVD-METHOD", "TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "SERVICE-INSTANCES", "SOURCE-FIELD-REF", "MAXIMUM-MESSAGE-LENGTH", "NM-CAR-WAKE-UP-BIT-POSITION", "RETRY", "ROM-BLOCK", "SOFTWARE-CLUSTER-DEPENDENCY", "HAS-DYNAMIC-LENGTH", "MULTICAST-SECURE-COM-PROPS-REF", "DOIP-CONNECTION-REFS", "REQUEST-RESULT", "DATA-PROTOTYPE-TRANSFORMATION-PROPS", "METHOD-REFS", "SOURCE-MAX-ADDRESS", "INHIBITING-SECONDARY-FID-REFS", "EVENT-COMBINATION-REPORTING-BEHAVIOR", "HW-ATTRIBUTE-DEFS", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED", "REST-ENDPOINT-GET", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "MAX-ERROR-STATE-VALID", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "RECOVERY-NOTIFICATION-REFS", "ADDRESS-RANGE-UPPER-BOUND", "AUTOSAR-CONNECTOR", "TIMING-DESCRIPTION-REF", "PROPERTYS", "ETHERNET-PRIORITY-REGENERATION", "CAN-NM-ECU", "NETWORK-INTERFACE-ID", "CONFIGURATION-HANDLING", "SUCCESSOR-REF", "SERVER-ARGUMENT-IMPL-POLICY", "WAKEUP-LOCAL-DETECTION-TIME", "DDS-SERVICE-QOS-PROFILE-REF", "TABLE-ENTRYS", "DIAGNOSTIC-ADDRESS", "DLT-CONFIG", "FLOW-CONTROL-PDU-REFS", "ESTIMATED-EXECUTION-TIME", "SERVICE-INTERFACE", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING", "EVENT-GROUP-CONTROL-TYPE", "ACCEPTS-ENCODINGS", "IUMPR-GROUP-IDENTIFIERS", "FUNCTIONAL-REQUEST-REFS", "IS-REQUIRED", "FULL-DUPLEX-ENABLED", "SW-AXIS-INDEX", "I-PDU-IDENTIFIER-UDP-REFS", "DEFAULT-TRACE-STATE", "PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT", "SEARCH-INTENTION", "STATUS-ID", "DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL", "DLT-MESSAGE-REFS", "DO-IP-LOGIC-ADDRESS-PROPS", "DIAGNOSTIC-DATA-ELEMENT-REF", "DDS-RPC-SERVICE-REF", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "DIAGNOSTIC-OPERATION-CYCLE-REF", "DDS-SECURE-GOVERNANCE", "MODE-RECEIVER-POLICYS", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "CONTEXT-TYPE-LIST", "DIAGNOSTIC-ENV-MODE-CONDITION", "SDG-PRIMITIVE-ATTRIBUTE", "SW-TEXT-PROPS", "BSW-MODULE-CLIENT-SERVER-ENTRY", "UNUSED-BIT-PATTERN", "MSG-ID-MATCH", "BSW-DEBUG-INFO", "FILTER-ACTION-DROP-FRAME", "ITEMS", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "METHOD-CALL-REF", "FLEXRAY-TP-ECU", "CHECKPOINT-ID", "BSW-CLIENT-POLICY", "DDS-CP-PARTITION", "MINIMUM-TIMER-GRANULARITY", "HEALTH-CHANNEL-IREF", "MESSAGE-TYPE-INFO", "BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE", "SEND-ACTIVITY", "MONITORED-IDENTIFIER-REF", "FIELD-REPLY-TOPIC-NAME", "TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED", "TCP-SLOW-START-ENABLED", "LOGICAL-OPERATOR", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF", "CLEAR-CONDITION-GROUP-REF", "CLUSTER-DRIFT-DAMPING", "ASYNCHRONOUS-SERVER-CALL-POINT", "REQUEST-GROUP-REF", "SOMEIP-EVENT-PROPS", "RATIO-KIND", "VIEW-MAP-SET", "OBD-MONITOR-SERVICE-NEEDS", "CERTIFICATE-EVALUATIONS", "CUSTOM-SDG-DEF-REF", "LIVELINESS-LEASE-DURATION", "STATE-DEPENDENT-STARTUP-CONFIG", "INITIALIZED-PARAMETER-REF", "TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED", "SEC-OC-CRYPTO-SERVICE-MAPPING", "CLIENT-SERVER-ANNOTATION", "PCM-BIT-DEPTH", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "RULE-BASED-VALUES", "BOR-TIME-L-1", "CLASS-TAILORINGS", "DEFAULT-VALUE-ELEMENTS", "COMPLEX-TYPE-MAPPING", "SECURE-COM-PROPS-REF", "DATA-TRANSFORMATION-SET", "ATTRIBUTE-TAILORINGS", "RESPONSE-ON-EVENT-REF", "CONFLICTS-TO", "FRESHNESS-VALUE-ID", "AP-APPLICATION-ERROR", "REPLICA-PDUS-REFS", "MANAGED-PHYSICAL-CHANNEL-REFS", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "NM-USER-DATA-ENABLED", "SWC-MODE-MANAGER-ERROR-EVENT", "MESSAGE-TYPE", "INPLACE", "FUNCTION-GROUP-PORT-MAPPING", "LABEL", "INSTANCE-ID", "USE-CRC-COMP-MECHANISM", "BUFFER-PROPERTIES", "REQUIRES-INDEX", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "MODE", "NON-VERBOSE-MODE", "SDG-CONSTRAINT-REF", "SWC-SERVICE-DEPENDENCY", "DEFAULT-VLAN-REF", "IDSM-INSTANCE-REF-CONDITIONAL", "REFINED-EVENT-IREF", "SYNC-LOSS-TIMEOUT", "REQUIREMENT-REF", "TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT", "SCOPE", "PARTICIPATING-J-1939-CLUSTER-REFS", "TX-NM-PDU-REFS", "INITIAL-STATUS", "TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS", "MEMORY-USAGES", "MAX-PWM-L", "FLAT-INSTANCE-DESCRIPTOR", "DIAGNOSTIC-COMMON-PROPS-VARIANTS", "EXECUTION-CONTEXT-REFS", "GLOBAL-TIME-MASTERS", "INTERPOLATION-ROUTINE-MAPPING-SET", "OPTIONS", "FRAME-PORT-REFS", "END-TO-END-PROTECTION-SET", "MACHINE-REF", "MAP", "USER-DEFINED-CLUSTER", "SEC-OC-JOB-SEMANTIC", "BSW-PARAMETER-POLICY", "FLEXRAY-PHYSICAL-CHANNEL", "INITIALIZATION-VECTOR-LENGTH", "FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR", "MASTER-EVENT-REF", "AGGREGATION-CONDITION", "MAX-BUFFER-SIZE", "IDSM-INSTANCE", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF", "CONFIG-ELEMENT-DEF-GLOBAL-REF", "J-1939-NM-NODE-REF", "P", "SEC-OC-JOB-MAPPINGS", "ALLOW-MASTER-RATE-CORRECTION", "INDENT-SAMPLE", "TRANSPORT-PLUGIN", "PROVIDED-CLIENT-SERVER-ENTRYS", "DEVELOPMENT-VALUE", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "SECURE-COM-PROPS-FOR-TCP-REFS", "NM-INSTANTIATION", "TCP-IP-NDP-MAX-RTR-SOLICITATIONS", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "FT", "DEFERRING-FID-REF", "ALLOWED-KEYSLOT-USAGE", "SENDER-REC-RECORD-TYPE-MAPPING", "FILES", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "SW-FEATURE-DESC", "MAXIMUM", "COUPLING-PORT-REF-CONDITIONAL", "MINIMUM-START-INTERVAL", "MCD-IDENTIFIER", "DIRECT-TP-SDU-REF", "DEFAULT-PERIOD-BEGIN", "STATUS-SUB-TLV", "INPUT-VARIABLE-TYPE-REF", "GROUPED-ALIAS-EVENT-REF", "SW-GENERIC-AXIS-PARAM-TYPE-REF", "UNIT", "SUB-ELEMENT-MAPPINGS", "LOCAL-UNICAST-ADDRESSES", "PRIMITIVE-ATTRIBUTE-TAILORING", "OBD-MID", "START-STATE-MACHINE-REF", "WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF", "WRITING-PRIORITY", "SERVICE-INSTANCE-COLLECTION-SET", "MAC-ADDRESS-MASK", "PUTS", "GRAPHIC", "USER-DEFINED-ETHERNET-FRAME", "FIELD-GET-SET-QOS-PROPSS", "SUPPORT-INFO-BYTE", "SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF", "STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR", "LIN-FRAME-TRIGGERING", "IDENTITY", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPES", "LIMIT-KIND", "SEPARATED-COMPONENT-IREF", "PROCESS-TO-MACHINE-MAPPING-SET", "BOR-TIME-L-2", "PERSISTENT-STORAGE", "MAX-NUMBER-OF-FILES", "PDELAY-LATENCY-THRESHOLD", "INIT-VALUE", "SECURE-COMMUNICATION-PROPS-SET", "DATA-TRANSFORMATION-KIND", "TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS", "FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS", "PDU-COUNTER-START-POSITION", "TEST-ID-REF", "CUSTOM-DOCUMENTATION-REF", "UCM-MODULE-INSTANTIATION-REF", "EVENT-DISPLACEMENT-STRATEGY", "VALID-BINDING-TIMES", "TD-EVENT-VFB-PORT-GROUP", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "ROOT-VARIABLE-DATA-PROTOTYPE-REF", "SOVD-GATEWAY-INSTANTIATION", "ASSIGNED-TRAFFIC-CLASSS", "REPORTABLE-SECURITY-EVENT-REF", "SHORT-LABEL", "REQUESTED-MODE-IREF", "DIAGNOSTIC-EVENT-REF", "BSW-EXCLUSIVE-AREA-POLICY", "SYNC-DATA-ID-LIST", "MAX-DEADLINE", "CRYPTO-SERVICE-CERTIFICATE", "LOG-TRACE-LOG-MODES", "FILE-INFO-COMMENT", "MESSAGE-COMPLIANCE", "SW-VALUE-BLOCK-SIZE-MULTS", "FIREWALL-RULE-REF", "UDP-TP", "P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "CONTEXT-PORT-REF", "RPT-HOOK", "EVENT-OCCURRENCE-KIND", "TD-EVENT-BSW-MODULE-TYPE", "FIREWALL-REF", "DESIGN-REF", "DISCOVERY-TYPE", "ROM-BLOCK-INIT-VALUE", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS", "DNS-SERVER-ADDRESS", "PERSISTENCY-REDUNDANCY-CRC", "APPLICATION-ARRAY-ELEMENT-REF", "APPLICABILITY-INFO", "CHARGE-MANAGER-NEEDS", "IP-SEC-RULES", "DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE", "I-PDU-TIMING-SPECIFICATIONS", "PATTERN-LENGTH", "TBODY", "FLEXRAY-FIFOS", "STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE", "SOURCE-EVENT-REFS", "ROUTING-ACTIVATION-TYPE", "TCP-CONGESTION-AVOIDANCE-ENABLED", "BURST-SIZE", "FIRST-APPLICATION-ERROR-REF", "SW-FEATURE-DESCS", "INTERFACE-REF", "PROCESS-TO-MACHINE-MAPPINGS", "RECEIVED-DATA-REF", "SOFTWARE-PACKAGE-STEPS", "MAXIMUM-SEGMENT-LENGTH-RESPONSE", "SYSTEM", "DEFAULT-ENDIANNESS", "SOUCE-CAN-ID-REF", "DIAGNOSTIC-DE-AUTHENTICATION", "COMPRESSED-SOFTWARE-PACKAGE-SIZE", "TCP-IP-NDP-PREFIX-LIST-SIZE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "FALL-BACK-INTERNAL", "TCP-IP-ARP-PACKET-QUEUE-ENABLED", "MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING", "STATE-MANAGEMENT-ERROR-COMPARE-RULE", "NM-NETWORK-HANDLE-REF", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "HW-PIN-GROUP-REF", "SW-RECORD-LAYOUT-V-AXIS", "ACTIVATION-ACTION", "TD-PDU-TRIGGERING-FILTER-REF", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "NV-PROVIDE-COM-SPEC", "DTC-NUMBER", "POSITIVE-INTEGER-VALUE-VARIATION-POINT", "DIRECTION", "EGRESS-PORT-REFS", "END-TO-END-PROTECTION-I-SIGNAL-I-PDUS", "IP-V-6-PATH-MTU-ENABLED", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "SOMEIP-SERVICE-DISCOVERY", "LOWER-MULTIPLICITY", "PADDING-ACTIVATION", "RESPONSE-ERROR-REF", "ACL-ROLE-REFS", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "FIBEX-ELEMENT-REF-CONDITIONAL", "MSR-QUERY-NAME", "TRCV-DELAY-COMPENSATION-OFFSET", "TARGET-DATA-ELEMENT-REF", "SWC-MAPPING-CONSTRAINT-REFS", "PNC-PDUR-GROUP-REFS", "NDP-PROPS", "UDP-TP-PORT", "DLT-ARGUMENT", "ARRAY-SIZE-SEMANTICS", "HARDWARE-CONFIGURATION", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "PROVIDE-RESOURCES", "INSTANTIATION-DATA-DEF-PROPS", "INTERNAL-TRIGGER-OCCURRED-EVENT", "SIGNATURE-SCHEME-REF", "FLEXRAY-AR-TP-NODE", "HW-ATTRIBUTE-LITERALS", "SIZE-OF-STRING-LENGTH-FIELD", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS", "REQUESTER-SOFTWARE-CLUSTER-REF", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA", "TARGET-FRAME-REF", "COUPLING-PORT-DETAILS", "DATA-LINK-LAYER-RULE", "INFRASTRUCTURE-SERVICES", "NEEDS-ORIGINAL-DATA", "SHALL-NOT-RUN-ON-REF", "RTE-EVENT-IREF", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-PORT-MAPPING", "FIBEX-ELEMENT-REF", "INCLUDE-REF", "FIELD-REQUEST-TOPIC-NAME", "SECURITY-LEVEL-REFS", "FILTER", "USE-INSTEAD-REFS", "CRYPTO-SERVICE-QUEUE-REF", "MC-DATA-INSTANCE-REF", "SW-CLUSTER-REF", "SIGNAL-GROUP-REF", "APMC-INSTANCE-REFERENCE-VALUE", "TIMEOUT", "PORT-INTERFACE-MAPPING-SET", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "SOURCE-SERVICE-INTERFACE-REFS", "DEBOUNCE-COUNTER-STORAGE", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "NUMBER-OF-PARALLEL-TCP-SESSIONS", "CONDITION-FORMULA", "PORT-PROTOTYPE-BLUEPRINT-MAPPING", "SERVICE-INSTANCE-PORT-CONFIG", "FLAT-MAP-ENTRY-REFS", "DTC-STATUS-MASK", "CIPHER-SUITE-ID", "ACCEPT-VARIABLE-DA", "RX-PDU-POOL-REF", "HW-CATEGORY-REF", "MEASURED-HEAP-USAGE", "FIREWALL-STATE-SWITCH-INTERFACE", "PERSISTENCY-FILE-PROXY", "FREEZE-FRAME-REF", "DIAGNOSTIC-DEPLOYMENT-PROPS-REF", "SOURCE-ADDRESS-RANGE-END", "MAC-MULTICAST-ADDRESS-REF", "SYSTEM-SIGNAL-REFS", "TABLE", "OBD-PID-SERVICE-NEEDS", "SYNCHRONIZED-MODE-GROUPS", "LOCAL-ENDPOINT-CONFIG", "GROUP-IDENTIFIER", "DATA-TYPE-MAPPING-SET", "AP-APPLICATION-ENDPOINTS", "SW-AXIS-TYPE-REF", "RECORD-NUMBER", "OPERATION-CYCLE-AUTOSTART", "XDOC", "RVF-PIXEL-DEPTH", "SHORT-LABEL-PATTERN", "ECUC-DEF-EDITION", "SWITCH-STREAM-FILTER-ENTRY", "CONTEXT-DATA-PROTOTYPE-REF", "KEEP-ALIVES", "DIAGNOSTIC-IUMPR-GROUP", "INSTANCE-IN-MEMORY", "APP-TASK-PROXY-REF", "GETTER-RETURN-SIGNAL-REF", "BASE-REF", "SW-MAX-AXIS-POINTS", "COMPU-DEFAULT-VALUE", "SECURE-PROPS-FOR-TCP-REF", "DEPENDENCIES", "BSW-ENTRY-RELATIONSHIPS", "HW-CATEGORY-REFS", "TCP-IP-IP-NUM-FRAGMENTS", "SYNCHRONOUS-SERVER-CALL-POINT", "IEEE-1722-RAW-DATA-STREAM-PRODUCER-INTERFACE", "MINIMUM-SEND-INTERVAL", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-PORT-MAPPING", "R-PORT-PROTOTYPE-REF", "LENGTH-SEQUENCE-COUNTER", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "ARGUMENT", "MC-PARAMETER-ELEMENT-GROUP", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF", "MIN-OK-STATE-VALID", "ASSIGNED-ENTRY-REF", "TRANSFORMATION-PROPS-REF", "DISABLE-END-TO-END-STATE-MACHINE", "ROOT-ARGUMENT-DATA-PROTOTYPE-REF", "IMPLEMENTATION-CONFIG-CLASSES", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "EXTERNAL-TRIGGERING-POINT", "CONSTRAINT-TAILORING", "EXPLICIT-RPT-PROFILE-SELECTION-REFS", "ROOT-SW-COMPONENT-PROTOTYPE", "TCP-OPTION-FILTER-LIST", "J-1939-TP-CONNECTION", "AUTO-REQUIRE", "TLS-CONNECTION-GROUP", "SIMULATED-EXECUTION-TIME", "CONSTANT-REFERENCE", "SOFTWARE-CLUSTER", "P-2-TIMING", "METHOD-RETURN-REFS", "CONNECTION-NEGOTIATION-BEHAVIOR", "RELATIVE-TOLERANCE", "TLV-DATA-ID-DEFINITION-REFS", "SW-DATA-DEPENDENCY", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "SWC-TO-IMPL-MAPPING", "SECOND-ELEMENT-REF", "CONFIRMATION-THRESHOLD", "TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED", "DYNAMICALLY-ASSIGNED", "LEAF-ELEMENT-IREF", "IDSM-QUALIFIED-EVENT-RECEIVER-INTERFACE", "INDICATOR-FAILURE-CYCLE-COUNTER-THRESHOLD", "CYCLIC-WRITING-PERIOD", "GLOBAL-TIME-SUB-DOMAINS", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "TARGET-EEC-REF", "CONFIDENTIALITY-OFFSET", "LABELED-LIST", "LIN-UNCONDITIONAL-FRAME", "PARAMETER-ACCESSED-FOR-DEBUG-REFS", "GETTER-CALL-SIGNAL-REF", "CP-SOFTWARE-CLUSTER", "SIZE-OF-STRUCT-LENGTH-FIELD", "VENDOR-SPECIFIC-MODULE-DEF-REF", "EVENT-MAPPINGS", "ADDITIONAL-NATIVE-TYPE-QUALIFIER", "VARIATION-POINT", "COMMUNICATION-DIRECTION", "BSW-RELEASED-TRIGGER-POLICY", "DISCOVERY-PROTECTION-KIND", "DDS-RPC-SERVICE-DEPLOYMENT", "IMPLEMENTS-LEGACY-STRING-SERIALIZATION", "UNICAST-SECURE-COM-PROPS-REF", "SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION", "RELEASED-TRIGGERS", "MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "COM-FIND-SERVICE-GRANT-DESIGN", "CONTAINED-FIBEX-ELEMENT-REF", "UDP-NM-NODE", "ACTION-POINT-OFFSET", "ROUTINE-SUBFUNCTION-REF", "COM-OFFER-SERVICE-GRANT", "EVENT-SYMBOL-NAME", "EXECUTION-DEPENDENCYS", "J-1939-DTC-VALUE", "RPT-SERVICE-POINT", "IDSM-CONTEXT-PROVIDER-MAPPING", "SYSTEM-REF", "TEXT-TABLE-MAPPINGS", "LOG-SINK-REFS", "GLOBAL-SUPERVISION-NEEDS", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "VALUE-TYPE-REF", "CONTEXT-DATA-PROTOTYPE-IN-SR-REF", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF", "PROTOCOL", "SERVICE-INTERFACE-REFS", "RPT-EXECUTABLE-ENTITY-EVENT", "DURABILITY-SERVICE-CLEANUP-DELAY", "MINISLOT-DURATION", "SOFTWARE-PACKAGE-STORING", "SWITCH-MAC-ADDRESS-LEARNING-MODE", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "REQUESTER-REFS", "GENERAL-PURPOSE-CONNECTION", "NEXT-HEADER", "IKE-AUTHENTICATION-METHOD", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "MAXIMUM-EXECUTION-TIME", "CRYPTO-KEY-SLOT-INTERFACE", "DTC-FORMAT-TYPE", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "BINARY-MANIFEST-ITEM-NUMERICAL-VALUE", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "THEAD", "VFC-IREFS", "FOLLOW-UP-ACTION-REF", "STORE-IMMEDIATE", "INVERT-CONDITION", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "RESOURCES", "PREVIOUS-EXECUTABLE-VERSION", "ECU-INSTANCES", "DIAGNOSTIC-FREEZE-FRAME", "REG-DOES-NOT-REQUIRE-STABILITYS", "SUPERVISION-MODE", "RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS", "METHOD-MAPPINGS", "DATA-LENGTH-REQUEST", "DIAGNOSTIC-TROUBLE-CODE-PROPS-REF-CONDITIONAL", "PHYSICAL-CHANNELS", "SYMBOLIC-NAME-PROPS", "IO-CONTROL-CLASS-REF", "TP-ADDRESS-REF", "DATA-TYPE-REF", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE", "LOCAL-DEBUG-DATAS", "ECUC-FUNCTION-NAME-DEF-CONDITIONAL", "DISABLED-IN-MODE-IREFS", "BUILD-TYPE", "ORDERED-MASTER", "OUTGOING-CALLBACKS", "DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL", "SEC-OC-JOB-MAPPING", "DO-IP-NETWORK-CONFIGURATION", "UPPER-MULTIPLICITY", "EVENT-GROUP-REFS", "FIRST-DATA-PROTOTYPE-REF", "DEPENDS-ON-REFS", "HOLD-OVER-TIME", "REMOTE-SERVER-CONFIG", "FREEZE-CURRENT-STATE", "PARAMETER-DATA-PROTOTYPE-IREF", "SUPERVISION-MODE-CONDITIONS", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "BUILD-ACTION-MANIFEST", "IMPLEMENTATION-RECORD-ELEMENT-REF", "PNC-PDUR-GROUP-REF", "CONDITIONAL-CHANGE-NAD", "LABEL-1", "COMPOSITION-SW-COMPONENT-TYPE-REF", "PNC-MAPPING-REF", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "RUNNABLE-ENTITY-IREF", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "CLAIMED-FUNCTION-GROUP-REF", "STREAMS-PER-FRAME", "IDSM-QUALIFIED-EVENT-RECEIVER-MAPPING", "DO-IP-POWER-MODE-STATUS-NEEDS", "DEF-ITEM", "WATCHDOG-PHM-ACTION-ITEM", "HARDWARE-PLATFORM", "DTC-PROPS-REF", "REMOTE-CLIENT-CONFIG", "DEFAULT-MODE-REF", "ACTION-ITEM-LISTS", "ETHER-TYPE", "TYPE-MAPPING", "NEW-NAD", "CONDITION-ACCESS", "FIRE-AND-FORGET", "ACL-OBJECT-SET", "PROVIDER-SOFTWARE-CLUSTER-REF", "EXECUTABLE-ENTITY-REFS", "RATIONALE", "PARTIAL-NETWORK-REF", "USED-PARAMETER-ELEMENT", "ALLOWED-TCP-OPTIONS", "FUNCTION-GROUP-IREFS", "COM-CONFIGURATION-ID", "TCP-IP-NDP-DESTINATION-CACHE-SIZE", "MC-VARIABLE-INSTANCES", "CONTEXT-MODE-DECLARATION-GROUP-REF", "IP-ADDRESS-KEEP-BEHAVIOR", "APPLICATION-ENDPOINT-REF", "TD-EVENT-VFB-PORT-REFS", "EXTERNAL-SYNC", "REFERENCE-CONDITION", "FUNCTION-GROUP-TYPE-REF", "NM-REPEAT-MESSAGE-TIME", "EXCLUSIVE-AREA-POLICYS", "EVENT-REFS", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "DEPENDENT-RESOURCES", "INTENDED-PARTIAL-INITIALIZATION-COUNT", "TOPIC-1", "BSW-OS-TASK-EXECUTION-EVENT", "CONTEXT-DATAS", "ETH-TCP-IP-ICMP-PROPS", "COMPU-NUMERATOR", "DERIVATION", "BINARY-MANIFEST-RESOURCE-DEFINITION", "DESTINATION-PORTS", "PERSISTENCY-REDUNDANCY-M-OUT-OF-N", "PLATFORM-PHM-ACTION-ITEM", "NM-BUSLOAD-REDUCTION-ENABLED", "ACL-ROLE", "CRYPTO-PROVIDER-DOCUMENTATION-REF", "CRYPTO-KEY-SLOT-REF", "MAX-NUM-COMPARISION-OF-VALUE-EVENTS", "USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS", "TYPE-DEFINITION", "DEFAULT-LOG-THRESHOLD", "BANDWIDTH-LIMITATION", "DISABLED-MODE-IREF", "BASE-IS-THIS-PACKAGE", "CAN-FRAME", "POSSIBLE-ERROR-REF", "APPLICATION-ENTRY", "SW-HOST-VARIABLE", "ACF-BUS-PART-REFS", "USE-MAC-ADDRESS-FOR-IDENTIFICATION", "TIME-BASES", "TRANSMISSION-START-SEQUENCE-DURATION", "PHM-CONTRIBUTION-REFS", "REMOTE-CERTIFICATE-REF", "VALIDATION-CONFIGURATION", "TTCAN-CLUSTER", "ACTIVE-SUPERVISION-REF", "DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF", "REQUIRED-SERVICE-INSTANCE-ID", "NEXT-HIGHER-CERTIFICATE-REF", "EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD", "LINKER", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "ETHERNET-VLAN-TRANSLATION-TABLE", "SOFTWARE-CLUSTER-DESIGN-DEPENDENCY", "RECEIVER-IREF", "INPUT", "NM-STATE-CHANGE-IND-ENABLED", "COM-MANAGEMENT-MAPPING", "TDLET-ZONE-CLOCK", "EXIT-MODE-DECLARATION-REF", "RAPID-PROTOTYPING-SCENARIO", "INITIAL-REPETITIONS-MAX", "PDU-COUNTER-THRESHOLD", "PORT-PROTOTYPE-IREF", "REPRESENTS-VIN", "CIPHER-SUITE-SHORT-LABEL", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "DTC-SETTING-PARAMETER", "DLT-CONTEXT-REF", "ABBR-NAME", "LIN-COMMUNICATION-CONNECTOR", "OBD-RELEVANCE", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "GLOBAL-TIME-DOMAIN-REF-CONDITIONAL", "PRE-SHARED-KEY-REF", "BSW-MODULE-DOCUMENTATIONS", "TRANSFORMATION-PROPSS", "BINARY-MANIFEST-PROVIDE-RESOURCE", "ATTRIBUTE-DEFS", "EXECUTABLE-TIMING", "DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE", "MODULE-INSTANTIATION-REFS", "PDU-COUNTERS", "SERVICE-POINT-SYMBOL-PRE", "SECURE-PROPS-FOR-UDP-REF", "LIN-SPORADIC-FRAME", "IMPLICIT-INTER-RUNNABLE-VARIABLES", "RPT-EVENT-ID", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING", "SESSION-HANDLING-SR", "SW-RECORD-LAYOUT-COMPONENT", "SECTION-NAME-PREFIXS", "RETURN-CODE", "AUTH-INFO-TX-LENGTH", "PREDEFINED-TEXT", "STOP-IREF", "BAUDRATE", "DELAY-COMPENSATION-B", "UCM-STEP", "MODE-ELEMENT-REF", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "FLEXRAY-TP-CONFIG", "INCLUDED-LIBRARY-REF", "MIN-ALIVE-CYCLE", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "TIMING-DESCRIPTION-EVENT-CHAIN", "DATE", "SYSTEM-SIGNAL-REF", "BSW-INTERRUPT-EVENT", "PROCESS-MODE-IREF", "EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE", "REPLAY-PROTECTION-WINDOW", "SETTER-RETURN-SIGNAL-REF", "SERVER-SERVICE-MINOR-VERSION", "COMPARE-VALUE", "ITEM-DEFINITIONS", "CLIENT-ID-DEFINITION-SET-REF", "RESPONSE-ON-EVENT-ACTION", "SW-IS-VIRTUAL", "TRACE-SWITCH-CONFIGURATION", "REQUIRED-SERVICE-INSTANCE-REF", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY", "MEMORY-SECTION-LOCATION", "SOMEIP-TP-CONFIG", "STATE-MANAGEMENT-STATE-NOTIFICATION", "FEATURE-REF", "CLIENT-SERVER-RECORD-ELEMENT-MAPPING", "NM-IMMEDIATE-NM-TRANSMISSIONS", "PNC-WAKEUP-DATA-MASK", "ALLOWED-SERVICE-PROVIDERS", "RESPONSE-ON-ALL-REQUEST-SIDS", "ELEMENT-IN-IMPL-DATATYPE", "P-PORT-PROTOTYPE", "AES-3-DATA-TYPE-L", "FDC-THRESHOLD-STORAGE-VALUE", "SO-CON-I-PDU-IDENTIFIER-REF", "P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "SW-RECORD-LAYOUT-V", "CAN-COMMUNICATION-CONTROLLER-VARIANTS", "FUNCTION-GROUP-MODE-IREF", "APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-VALUE", "NOTIFIER-DATA-ELEMENT-REF", "EMULATION-SUPPORTS", "DERIVED-OBJECT-REF", "ROUTING-GROUP-REF", "DESTINATION-URI-REF", "MAXIMUM-DISTANCES", "TCP-IP-IP-REASSEMBLY-BUFFER-COUNT", "XREF", "ECUC-CHOICE-CONTAINER-DEF", "STIMULUS-REF", "ACL-SCOPE", "SECURITY-DELAY-TIME-ON-BOOT", "REQUIRED-GENERATOR-TOOLS", "BUILD-ACTION-IO-ELEMENT", "DURABILITY-SERVICE-MAX-SAMPLES", "FIREWALL-RULE", "SW-CLASS-INSTANCE", "IKE-REAUTH-TIME", "SECURE-COM-PROPS-FOR-TCP-REF", "NV-DATA-PORT-ANNOTATIONS", "ARRAY-ELEMENT-REF", "TRANSMISSION-MODE-CONDITION", "I-PDU-IDENTIFIER-TCP-REFS", "MAX-TRANSIT-TIME", "TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED", "ICV-SECURED", "GLOBAL-SUPERVISION", "MAXIMUM-INTER-ARRIVAL-TIME", "RX-SECURITY-VERIFICATION", "LIN-UNCONDITIONAL-FRAME-REF", "PNC-MAPPINGS", "IDSM-REPORTING-MODE-PROVIDER-INTERFACE", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE", "INTENDED-TARGET-MACHINE-REF", "PRM", "I-PDU-TRIGGERING-REF-CONDITIONAL", "PREFIX", "NOMINAL-EXECUTION-TIME", "WAKEUP-RX-WINDOW", "SYMBOL-WINDOW-ACTION-POINT-OFFSET", "PDU-REPLICATION-VOTING", "MACHINE-MODE-MACHINES", "DATA-SEND-POINTS", "TP-CHANNEL-REF", "ECUC-STRING-PARAM-DEF-VARIANTS", "REQUIRED-ARTIFACTS", "HW-ATTRIBUTE-VALUE", "ENABLE-DEACTIVATION", "LOGICAL-SUPERVISIONS", "MAX-PERIODIC-DID-TO-READ", "FUNCTION-GROUP-PHM-STATE-REFERENCE", "UNIT-AND-SCALING-ID", "DATA-TYPE-MAPPING", "STARTUP-CONFIG", "REMOTE-NETWORK-ENDPOINT-REF", "CALLBACK-HEADER-REFS", "IDSM-TIMESTAMP-PROVIDER-INTERFACE", "ELEMENT-MAPPINGS", "INITIAL-MODE-REF", "USAGES", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "HW-ELEMENT-REF", "REPORTING-BEHAVIOR", "WINDOW-SIZE-INVALID", "UDP-LOCAL-PORT", "CONNECTORS", "PERSISTENCY-DEPLOYMENT-ELEMENT-REF", "PAYLOAD-BYTE-PATTERN-RULE-PARTS", "APMC-DEFINITION-COLLECTION", "CAPTION", "BYPASS-ETHER-TYPES", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "ENABLE-WRITE-ACCESS-CONTROL", "VIN-INVALIDITY-PATTERN", "LOCAL-CERTIFICATE-REFS", "CLIENT-ID", "CLIENT-INTENT", "STATE-MANAGEMENT-REQUEST-RULE", "KEY", "ROUTING-GROUP-REFS", "WRITING-STRATEGY-ROLE", "END-2-END-EVENT-PROTECTION-PROPS", "DLT-MESSAGE-REF", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "REQUIRED-PORT-PROTOTYPE-IREF", "FRESHNESS-PROPS-REF", "ASSIGNED-ENTRY-ROLES", "TRIGGER-INTERFACE", "EVENT-ELEMENT-MAPPINGS", "MAX-CYCLES", "TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED", "AUTH-DATA-FRESHNESS-START-POSITION", "PDUR-I-PDU-GROUP", "STATE-MANAGEMENT-EM-ERROR-INTERFACE", "IPV-6-ADDRESS-MASK", "EXCLUSIVE-AREA-NESTING-ORDER", "IEEE-1722-TP-ACF-LIN", "IP-SEC-CONFIG-PROPS", "TRIGGER", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "FUNCTION-GROUP-IREF", "TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE", "TD-EVENT-TT-CAN-CYCLE-START", "COUNTER-OFFSET", "HW-ELEMENT-CONNECTIONS", "MIN-SYNC-JUMP-WIDTH", "PDU-COLLECTION-SEMANTICS", "HW-ELEMENT-REF-CONDITIONAL", "SDG-FOREIGN-REFERENCE", "BSW-ENTITY-REF", "NM-PDU", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "CRYPTO-KEY-SLOT-REFS", "SIZE-OF-STRUCT-LENGTH-FIELDS", "PROCESSING-UNIT-REF", "DIAGNOSTIC-UDS-SEVERITY-ENUM-VALUE-VARIATION-POINT", "SERVICE-INTERFACE-ID", "KEY-ID", "TEXT-VALUE-SPECIFICATION", "BLOCK-INCONSISTENT", "ABSOLUTELY-SCHEDULED-TIMINGS", "SUB-ELEMENTS", "SOMEIP-SERVICE-INTERFACE", "SCHEDULER-NAME-PREFIX-REF", "SW-CLASS-PROTOTYPE-REF", "SOCKET-CONNECTION-BUNDLE-REF", "TIME-SYNC-MODULE-INSTANTIATION", "MODE-SWITCH-POINTS", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "DESTINATION-URI-NESTING-CONTRACT", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "L-GRAPHIC", "CONTAINERS", "DO-IP-CONFIG", "CAN-TP-ADDRESS-REF", "DATA-PROTOTYPE-MAPPING", "SECURE-COM-PROPSS", "ENABLE-DISCOVERY-PROTECTION", "RX-MASK", "EXECUTABLE-REQUIRED-PORT-IREF", "I-PDU-PORT-REF", "RESTORE-AT-START", "TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "SERVICE-FIND-TIME-TO-LIVE", "GLOBAL-TIME-GATEWAY", "ARGUMENT-MAPPINGS", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "COUPLED-CLUSTER-REF", "INDICATOR-REF", "TLV-DATA-ID-0-REFS", "BYPASS-ETHER-TYPE", "BSW-SERVICE-DEPENDENCY-REF", "SUB-GROUP-REFS", "TP-ADDRESS", "ARTIFACT-DESCRIPTOR", "PERIODIC-RATE-CATEGORY", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "DO-IP-INTERFACE", "NM-PDUS", "TIME-BASE-KIND", "DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL", "ORDERED-MASTER-LIST", "P-PORT-IN-COMPOSITION-INSTANCE-REF", "NM-HW-VOTE-ENABLED", "RAW-DATA-STREAM-CLIENT-INTERFACE", "UCM-NOT-AVAILABLE-ON-THE-NETWORK", "ON-BOARD-MONITOR-ID", "KEEP-ALIVE-PROBES-MAX", "TRIGGERING-EVENT-REF", "RUNS-INSIDE-EXCLUSIVE-AREA-REF", "SPN-REF", "DIAGNOSTIC-SOVD-LOG", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "MAX-SLOTS-PER-CYCLE", "BEHAVIOR-REF", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "PORT-ADDRESS", "DATA-TYPE-FOR-SERIALIZATION-REF", "NM-DATA-ENABLED", "APPLICATION-ERROR", "APMC-CONTAINER-REFERENCE-VALUE", "TCP-IP-DHCP-V-6-SOL-DELAY-MIN", "INDEX-DATA-TYPE-REF", "HANDLE-DATA-STATUS", "SUPERVISION-CYCLE", "REQUIRED-DDS-SERVICE-INSTANCE", "NOTE", "STATE-DEPENDENT-FIREWALL", "STORAGE-STATE-EVALUATION", "BSW-INTERNAL-BEHAVIOR", "SDG-CLASS-REF", "SOMEIP-METHOD-PROPS", "PRMS", "AUDIENCE", "ALIVE-SUPERVISIONS", "RECORD-ELEMENT-MAPPINGS", "SOVD-VALIDATION-ORDER-REFS", "TIME-SUB-TLV", "DIAGNOSTIC-ROUTINE-CONTROL", "RPT-SERVICE-POINT-POST-REFS", "INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "TCP-IP-NDP-DYNAMIC-MTU-ENABLED", "CONCRETE-CLASS-TAILORING", "ECUC-ADD-INFO-PARAM-VALUE", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "TRIGGER-MAPPING", "PLATFORM-ACTION-ITEM", "MAC-ADDRESS-VLAN-MEMBERSHIP", "SNAPSHOT-RECORD-CONTENTS", "PORT-DEFINED-ARGUMENT-BLUEPRINTS", "SEPARATION-CYCLE-EXPONENT", "INNER-GROUP-IREFS", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "SOMEIP-TRANSFORMATION-PROPS-REF", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "STATIC-SOCKET-CONNECTION", "STATE-MACHINE", "SIGNATURE-SUPPORT-AP", "SIGNAL-PATH-CONSTRAINTS", "COUPLING-PORT-FIFO", "ETH-TP-CONNECTION", "BLUEPRINT-POLICY-SINGLE", "MAXIMUM-SELECTED-BINDING-TIME", "CRYPTO-KEY-SLOT-USER-DESIGN-REF", "SEC-OC-SECURE-COM-PROPS-REF", "DEFERRING-FID-REFS", "REQUIRED-DESIGN-ELEMENT-REF", "ETHERNET-CLUSTER-CONDITIONAL", "OPERATION-IREFS", "C-CODE", "SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL", "PRIORITY-ID", "AP-APPLICATION-ERROR-REF", "CONTAINED-I-PDU-TRIGGERING-PROPSS", "VALUE-AXIS-DATA-TYPE-REF", "ECU-SCOPE-REFS", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "COMMAND", "COMPONENT-QUALIFIER", "CONTENT-URI", "KEY-SLOT-USAGES", "SOURCE-NETWORK-MASK", "PRESTORAGE-FREEZE-FRAME", "NM-READY-SLEEP-COUNT", "MC-FUNCTION", "IPV-4-ADDRESS", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING", "NV-BLOCK-DESCRIPTOR", "OFFSET-CORRECTION-START", "WRITE-VERIFICATION", "SCHEDULE-TABLES", "IP-V-4-ADDRESS", "TP-CONNECTION-REFS", "TCP-IP-ARP-REQUEST-TIMEOUT", "IPV-6-MULTICAST-IP-ADDRESS", "PARTITION-NAME", "TEST-IDENTIFIER", "DATA-RECEIVE-ERROR-EVENT", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "MAX-PWM-O", "TEST-PATTERN", "FM-FEATURE", "MODULE-INSTANTIATION-REF", "TIMING-CLOCK-SYNC-ACCURACYS", "RESOURCE-DEFINITIONS", "STATE-REFERENCES", "AUTHENTICATION-TIMEOUT", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "FIRE-AND-FORGET-MAPPING", "OBJECT-DEFINITION-REF", "MICRO-PER-CYCLE", "STREAM-IDENTIFICATION-HANDLE-REFS", "HANDLE-TIMEOUT-TYPE", "ARTIFACT-CHECKSUM", "MODE-SWITCH-RECEIVER-COM-SPEC", "DOCUMENTATION", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "SUB-SOFTWARE-CLUSTER-REF", "VERSION", "PER-INSTANCE-MEMORY", "ROLLOUT-QUALIFICATIONS", "TYPE-BLUEPRINTS", "MODE-REF", "CHECKSUM-VALUE", "REQUESTER-IREF", "TARGET-RTE-EVENT-REF", "IEEE-1722-TP-ACF-CAN", "UNICAST-CREDENTIALS", "TIMING-MODE-INSTANCE", "START-ON-EVENT-REF", "FUP-DATA-ID-LISTS", "PREDEFINED-VARIANT", "OPT-ARGUMENT-REFS", "TIMING-ARGUMENTS", "SECOND-OPERATION-REF", "NON-OS-MODULE-INSTANTIATION-REF", "SOFTWARE-CONTEXT", "OWNERSHIP-KIND", "KEYWORD", "TRANSMISSION-MODE-FALSE-TIMING", "EVENT-RECEPTION-DEFAULT-VALUE", "UDP-NM-CLUSTER-REF", "WINDOW-SIZE-VALID", "DEPLOYMENT-URIS", "PROVIDED-REQUIRED-INTERFACE-TREF", "FM-FEATURE-MAP", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "FM-FEATURE-SELECTION", "CIPHER-SUITE-PRIORITY", "NM-STATE-REQUEST", "BSW-MODE-GROUP-REF", "DIAGNOSTIC-IO-CONTROL-CLASS", "APMC-URI-FOREIGN-REFERENCE-DEF", "LET-DATA-EXCHANGE-PARADIGM", "TROUBLE-CODE-OBD-REF", "CAS-RX-LOW-MAX", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "VLAN-REF", "CONTEXT-DESCRIPTION", "SERVICE-INTERFACE-VERSION", "IIDC-FRACTION-NUMBER", "STORE-EVENT-ENABLED", "MSR-QUERY-RESULT-P-2", "MC-DATA-INSTANCE-REFS", "BOR-COUNTER-L-1-TO-L-2", "HW-PIN-GROUP", "SD", "PER-INSTANCE-PARAMETERS", "ANONYMOUS", "BURST-SIZE-RESPONSE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "EXTERNAL-REPLACEMENT-REF", "BSW-DATA-SEND-POLICY", "DATA-REF", "CONTROL-CONSUMED-EVENT-GROUP-REFS", "SUPPLIER-VALIDATION-ORDER-REF", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "QUEUED-SENDER-COM-SPEC", "RESPONSE-REF", "SERVICE-DISCOVERY-CONFIGS", "DIAGNOSTIC-FREEZE-FRAME-REF", "MAXIMUM-DURATION-OF-CAMPAIGN", "DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL", "PHM-ARBITRATION", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "ETHERNET-RAW-DATA-STREAM-MAPPING-REF", "OUT-MEASUREMENT-SET", "IO-HW-ABSTRACTION-SERVER-ANNOTATIONS", "STATE-MANAGEMENT-SYNC-ACTION-ITEM", "UDP-PORT-REF", "PERSISTENCY-FILE-REQUIRED-COM-SPEC", "ASSERTIONS", "GLOBAL-TIME-CORRECTION-PROPS", "NM-COORD-CLUSTER", "IEEE-1722-TP-RVF-CONNECTION", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "DDS-EVENT-QOS-PROPS", "CONFLICTS", "REFERENCED-TD-EVENT-VFB-REF", "ETHERNET-RAW-DATA-STREAM-GRANT", "IDLE-SLOPE", "TD-EVENT-OPERATION", "MAX-NUMBER-FREEZE-FRAME-RECORDS", "BSW-MODE-SWITCH-EVENT", "UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE", "MC-DATA-ACCESS-DETAILS", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF", "TRANSIT-TO-INVALID-EXTENDED", "TARGET-MODE-DECLARATION-REF", "TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED", "TARGET-EVENT-REFS", "NM-NODES", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "FUNCTION-CLUSTER-AFFILIATION", "PRESTORED-FREEZEFRAME-STORED-IN-NVM", "TIMING-CLOCK-SYNC-ACCURACY", "USE-AUTH-DATA-FRESHNESS", "LOCAL-COM-ACCESS-CONTROL-ENABLED", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "CLIENT-SERVER-OPERATION-COM-PROPS", "PREDECESSOR-REF", "STOP-STATE-MACHINE-REF", "RESULTING-RPT-SW-PROTOTYPING-ACCESS", "EVENT-CONTROLLED-TIMING", "CAN-ENTERS", "OPTION", "PHYSICAL-PROPS", "CAN-IDENTIFIER", "DO-IP-SOURCE-ADDRESS-REF", "BSW-MODULE-INSTANCE-REF", "CHAPTERS", "ECUC-CHOICE-REFERENCE-DEF", "SW-RECORD-LAYOUT-REF", "KEY-SLOT-USED-FOR-SYNC", "CLIENT-SERVICE-MINOR-VERSION", "FIRST-TIMEOUT", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL", "PDU-TRIGGERING-REFS", "MODE-DECLARATION-GROUP-REF", "CRYPTO-CERTIFICATE-INTERFACE", "ACL-OPERATION", "NM-COM-CONTROL-ENABLED", "SYNC-COUNTER-INIT", "DIAGNOSTIC-EVENTS", "BUS-SPECIFIC-NM-ECU", "SW-CALPRM-IMPLS", "DEBOUNCE-ALGORITHM-PROPSS", "TARGET-SIGNAL-REF", "PHYSICAL-DIMENSION", "CAN-CONTROLLER-FD-ATTRIBUTES", "ALLOW-CONTENT-TYPE-CHANGE", "RPT-SERVICE-POINTS", "STD", "MODEL-OBJECT-REFERENCE", "SYSTEM-DOCUMENTATIONS", "SCHEDULING-POLICY", "RID-NUMBER", "ROUTINE-REF", "LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING", "INCLUDED-DATA-TYPE-SETS", "ECUC-FOREIGN-REFERENCE-DEF", "UDP-MIN-TX-BUFFER-SIZE", "GLOBAL-TIME-DOMAINS", "MEMORY-RANGE-REFS", "JOB-REF", "SEGMENT-RECEPTION-TIMEOUT-TIME", "PROVIDED-SERVICE-INSTANCES", "IP-SEC-CONFIG-PROPS-REF", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "LIN-ORDERED-CONFIGURABLE-FRAMES", "TCP-TP", "REFERENCE-BASE", "SWC-SERVICE-DEPENDENCY-IREF", "CIPHER-SUITE-CONFIGS", "EXPECTED-TX-TRIGGER", "SW-SYSTEMCONSTANT-VALUES", "CRYPTO-PROVIDER", "TARGET-VARIABLE-ACCESS-REF", "MINIMUM-RX-CONTAINER-QUEUE-SIZE", "BUILD-ACTION-ENVIRONMENTS", "ALLOW-PASSIVE-TO-ACTIVE", "CONSTANT-SPECIFICATION-MAPPING", "RX-NM-PDU-REF", "LIFE-CYCLE-INFOS", "FM-FEATURE-MAP-CONDITION", "TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE", "COUNTER-FAILED-THRESHOLD", "LIFESPAN-DURATION", "RESOURCE-GROUP-REF", "CAN-BIT-RATE-SWITCH", "CRYPTO-JOB-REF", "NV-BLOCK-DESCRIPTORS", "NOTIFICATION-STATE", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS", "SECOND-ELEMENT-REFS", "MAXIMUM-SEGMENT-LENGTH-REQUEST", "LIN-SLAVE-CONDITIONAL", "APPLIED-STANDARD", "CONTEXT-COMPONENT-PROTOTYPE-REF", "ACCEPT-VARIABLE-SA", "STATE-MANAGEMENT-MODULE-INSTANTIATION", "SUPPORTED-CONFIG-VARIANT", "ETHERNET-CLUSTER", "ELEMENT-POSITION", "SW-RECORD-LAYOUT-GROUP", "SHORT-TERM-ADJUSTMENT-SUPPORTED", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "KEY-SLOT-ALLOWED-MODIFICATION", "PERSISTENCY-FILE-PROXY-INTERFACE", "USE-FRESHNESS-TIMESTAMP", "APPLICATION-CONTEXT-REF", "TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED", "HW-PORT-MAPPINGS", "TD-EVENT-COMPLEX", "BSW-INTERRUPT-ENTITY", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS", "VARIANT-ID", "NM-SYNCHRONIZATION-POINT-ENABLED", "PRM-CHAR", "USER-DEFINED-EVENT-DEPLOYMENT", "UPDATE-STRATEGY", "SOURCE-I-PDU-REF", "SERVICE-IDENTIFIER", "PER-INSTANCE-MEMORY-SIZE", "ARRAY-ELEMENT-MAPPINGS", "CONNECTIONS", "ALLOWED-I-PV-6-EXT-HEADER", "PN-RESET-TIMER", "ACCURACY-EXT", "ECU-INSTANCE", "RPT-SERVICE-POINT-PRE-REF", "SUB-ELEMENT-MAPPING", "DEFAULT-GATEWAY", "FUNCTION-INSTANCE", "DATA-IDENTIFIER-REFS", "DYNAMIC-SLOT-IDLE-PHASE", "VARIANT-CRITERION-REF", "SUB-DOMAIN-REF", "SERVICE-POINT-SYMBOL-POST", "J-1939-TP-NODE", "COUNTER-PASSED-THRESHOLD", "SW-VALUE-CONT", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "RECOMMENDED-CONFIGURATION-REFS", "LIFE-CYCLE-STATE", "IP-PROTOCOL", "FIREWALL-STATE-MODE-DECLARATION-REF", "CLIENT-ID-RANGE", "DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE", "NM-CAR-WAKE-UP-FILTER-ENABLED", "COLOR-MODE", "ECU-EXTRACT-REF", "SUBSTITUTED-FRAME-REFS", "MAX-MINOR-VERSION", "ADMIN-DATA", "METHOD-QOS-PROPSS", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "SW-FEATURE-DEF", "NM-IF-ECUS", "PROVIDED-INTERFACE-TREF", "SECURITY-ACCESS-CLASS-REF", "INITIAL-REF-OFFSET", "INCLUDED-MODE-DECLARATION-GROUP-SETS", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "TP-ECU", "RATE-CORRECTION-MEASUREMENT-DURATION", "MANUFACTURER-VALIDATION-ORDER-REFS", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "REMOTE-UNICAST-CONFIG-REFS", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "CRYPTO-KEY-SLOT-DESIGN-REF", "BSW-ENTRY-RELATIONSHIP", "DATA-FILTER", "NM-PDU-REF-CONDITIONAL", "TT", "MODE-TRANSITION", "STIM-ENABLER", "TIME-BASE", "TCP-IP-ICMP-V-6-HOP-LIMIT", "RPT-EXECUTION-CONTEXT", "TCP-KEEP-ALIVE-ENABLED", "COND", "IS-REENTRANT", "IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "PARTITION-REF", "MINIMUM-INTERVAL-LENGTH", "BYPASS-VLAN", "TD-EVENT-VFB-REFERENCE", "ACCESS-CONTROL", "CLIENT-SERVER-ARRAY-TYPE-MAPPING", "CONTROL-ENABLE-MASK-BITS", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY", "PARAMETER-REF", "REST-INTEGER-PROPERTY-DEF", "TCP-IP-PROPS-REF", "INTERNAL-BEHAVIORS", "I-SIGNAL-PROPS", "SW-ADDR-METHOD-REF", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "REACTION-CODE", "NETWORK-LAYER-RULE", "STATE-MANAGEMENT-ACTION-LIST", "USER-DEFINED-FIELD-DEPLOYMENT", "TRANSFORMATION-COM-SPEC-PROPSS", "INGRESS-PRIORITY", "DIAGNOSTIC-SECURITY-LEVEL", "MODULE-INSTANTIATIONS", "CONFIDENCE-INTERVAL", "VLAN-REFS", "NM-USER-DATA-LENGTH", "VARIABLE-ACCESS-IREFS", "TCP-ROLE", "TARGET-I-PDU-REF", "VALUE-REF", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "ICV-VERIFICATION", "DLT-APPLICATION-TO-PROCESS-MAPPING", "DPG-DOES-NOT-REQUIRE-COHERENCYS", "BR", "MAX-SAMPLES", "RTP-TP", "A-2-L-DISPLAY-TEXT", "TRANSMISSION-ACKNOWLEDGE", "CONSTANT-VALUE-MAPPING-REFS", "HARDWARE-ELEMENT-REFS", "ESTIMATED-DURATION-OF-CAMPAIGN", "NETWORK-TIME-MASTER-REF", "ALIAS-NAMES", "MESSAGE-ID", "EVENT-HANDLER-REFS", "MSR-QUERY-PROPS", "UPPER-REF", "MULTIPLICITY-CONFIG-CLASSES", "TLS-SECURE-COM-PROPS", "CONTEXT-LIMITATION-REF", "MATCHING-RULE-REFS", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "IEEE-1722-TP-ACF-LIN-PART", "GENERAL-INACTIVITY-TIME", "CLEAR-EVENT-BEHAVIOR", "FIRST-MODE-GROUP-REF", "SUBSCRIBE-EVENTGROUP-RETRY-MAX", "SERVICE-ELEMENT-MAPPING-REFS", "OFFSET-TIME-DOMAIN-REF", "ARGUMENT-REF", "EVENT-SOURCE-REF", "IPV-6-PROPS", "MORE-FRAGMENTS", "POSSIBLE-ERRORS", "USAGE", "WATCHDOG-ACTION-ITEM", "REFILL-AMOUNT", "AUXILIARY-FIELD-DEFINITIONS", "ERROR-TRACER-NEEDS", "FIREWALL-RULE-REFS", "TYPE-OF-EVENT-COMBINATION-SUPPORTED", "RECEIVER-ANNOTATION", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS", "INTER-MESSAGE-TIME", "RUNNABLE-ENTITY", "I-PDU-SIGNAL-PROCESSING", "APPLICATION-ACTION-ITEM", "TRANSMISSION-PROPS", "REQUIRED-DESIGN-ELEMENT-REFS", "MEM-ALIGNMENT", "TRAFFIC-CLASS-PREEMPTION-SUPPORT", "VEHICLE-DRIVER-NOTIFICATION", "POSSIBLE-AP-ERROR-SET-REFS", "SEQUENCE-COUNTER-JUMP-WIDTH", "SUB-TYPE", "NM-INTERACTS-WITH-SM-MAPPING", "ECUC-REFERENCE-DEF", "VENDOR-API-INFIX", "TOOL-VERSION", "SOCKET-OPTION", "REQUIRED-FIBEX-ELEMENT-REF", "ACTION-LIST-ITEM-REFS", "CONTAINED-PACKAGE-ELEMENT-REFS", "COM-FIELD-GRANT-DESIGN", "CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "MASTERED-TRIGGER-REF", "TD-PDU-TRIGGERING-FILTER-REFS", "SYSTEM-TIMING", "USE-VEHICLE-IDENTIFICATION-SYNC-STATUS", "ON-TRANSITION-VALUE", "SW-DATA-DEPENDENCY-ARGS", "ACF-BUS-PART-REF", "SO-AD-CONFIG", "TIMEOUT-CS", "ANALYZED-EXECUTION-TIME", "BUILD-ACTIONS", "RECORD-ELEMENT-REF", "DTC-STATUS-AVAILABILITY-MASK", "ECUC-REFERENCE-VALUE", "CONSIDER-PTO-STATUS", "IDS-COMMON-ELEMENT-REF", "HISTORY", "ENCRYPTION-REF", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "VLAN-IDENTIFIER", "MAX-BLOCK-SIZE", "INSTANCE-IDENTIFIER", "STATUSS", "EXPLICIT-RPT-PROFILE-SELECTION-REF", "SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME", "SLEEP-REPETITIONS-OF-SLEEP-REQUEST", "SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS", "APPLICATION-PARTITION-REFS", "INTERNAL-TRIGGERING-POINT", "RESTRICTIONS", "IP-IAM-REMOTE-SUBJECT", "CRC-DOMAIN-NUMBER", "PLCA-MAX-BURST-COUNT", "E-2-E-METHOD-PROTECTION-PROPSS", "SOMEIP-SD-RULE", "HW-PIN-CONNECTIONS", "SEQUENTIAL-INSTRUCTIONS-BEGIN", "TIMING-ARGUMENT-REF", "FUNCTION-GROUPS", "ECU-EXTRACT-REFERENCE-IREF", "P-2-STAR-SERVER-MAX", "CLASSIFICATIONS", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "ENABLE-CONDITION-REF", "BINARY-MANIFEST-REQUIRE-RESOURCE", "WAKE-UP-CHANNEL", "SUPERVISED-ENTITY-ID", "VEHICLE-ANNOUNCEMENT-COUNT", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-DIAGNOSTIC-EVENT-MAPPING", "RUNTIME-PORT-CONFIGURATION", "ARP-PROPS", "RATE-POLICYS", "REQUIRED-ENTRYS", "RETURN-VALUE-PROVISION", "SERVICE-NEEDS", "APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-VALUE", "SIZE-OF-STRING-LENGTH-FIELDS", "PNC-MAPPING", "CUSTOM-DOCUMENT-ELEMENT-REF", "EVENT-CLEAR-ALLOWED", "LENGTH-CLIENT-ID", "MAX-MARGIN", "MAC-SEC-GLOBAL-KAY-PROPS", "STATIC-REMOTE-MULTICAST-ADDRESSES", "SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "ACCESS-COUNT-SETS", "COUNTER-INCREMENT-STEP-SIZE", "J-1939-NM-ECU", "TAILORING-REF", "APPLICATION-INTERFACE", "ECUC-LINKER-SYMBOL-DEF-VARIANTS", "KEYWORD-SET", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "FRESHNESS-VALUE-LENGTH", "NM-REMOTE-SLEEP-IND-ENABLED", "DIAGNOSTIC-AGING", "DESTINATION-URI-DEFS", "STATE-REF", "COUPLING-PORTS", "ON-FAIL-PERMISSIVE-MODE", "CHECKPOINT-TRANSITION-REF", "APMC-URI-FOREIGN-REFERENCE-VALUE", "RULE-ARGUMENTS", "BLUEPRINT-MAPPING-SET", "META-DATA-ITEM-SETS", "SEC-OC-COM-PROPS-FOR-MULTICAST-REF", "SEQUENTIAL-INSTRUCTIONS-END", "AAF-NOMINAL-RATE", "ELEMENT-REFS", "MAPPING-CONSTRAINTS", "DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT", "SW-MAPPINGS", "SUPERVISION-ENTITYS", "SW-COMP-TO-ECU-MAPPING-REFS", "STREAM-ID", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REF", "SOURCE-CAN-ID-MASK", "IDSM-INSTANCE-REF", "ENABLE-UPDATE", "INTERNAL-TRIGGERING-POINTS", "CAN-NM-CLUSTER-COUPLING", "MAX-SAMPLE-POINT", "TAILORING-REFS", "SPECIFIC-PHYSICAL-CHANNEL-REF", "TCP-IP-NDP-DELAY-FIRST-PROBE-TIME", "TP-NODES", "CONSUMED-EVENT-GROUP-REFS", "DOMAIN-ID", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "RPT-SUPPORT-DATA", "OBJECT-IREF", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS", "PARTICIPATING-J-1939-CLUSTER-REF", "METHOD-IREF", "GETTER-REF", "TRACE-SWITCH", "RATE-CORRECTION-OUT", "FIRST-VALUE", "CONSTRAINT-REF", "CLIENT-SERVER-ARRAY-ELEMENT-MAPPING", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "DATA-SEND-COMPLETED-EVENT", "CONFIGURED-NAD", "SECURITY-EVENT-CONTEXT-DATA-DEFINITIONS", "IP-TP-RULE", "CAN-XL-PROPS-REF", "OS-TASK-EXECUTION-EVENT", "TCP-RULE", "BLUEPRINT-MAPPING-GUIDE", "IMPLICIT-DATA-ACCESS-IREFS", "TD-EVENT-BSW-MODE-DECLARATION-TYPE", "VERIFY-REFS", "IP-ADDRESS", "APMC-BOOLEAN-PARAM-DEF", "SERVICE-OFFER-TIME-TO-LIVE", "ELEMENT-REF", "REMOTE-PORT-RANGE-START", "DEFAULT-ACTION", "DIAGNOSTIC-IO-CONTROL-NEEDS", "RESPONSE-ERROR-POSITION", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "ARGUMENT-DATA-PROTOTYPE", "WAIT-POINT", "DATA-TRANSFER-CLASS-REF", "ACL-OBJECT-CLASSS", "TRANSMIT-PDU-REF", "CONTROL-OPTION-RECORD-PRESENT", "ABSOLUTE-TOLERANCE", "BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING", "PERSISTENCY-FILE-ELEMENT", "CONSUMED-SERVICE-INSTANCES", "SENSOR-ACTUATOR-REF", "SLEEP-MODE-EXECUTION-DELAY", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "DLT-LOG-CHANNEL-DESIGN", "SWC-BSW-MAPPING-REF", "SERVICE-PROXY-SW-COMPONENT-TYPE", "SOVD-LOCK-REF", "N-PDU-REFS", "PROCESS-ARGUMENT", "SW-CLASS-ATTR-INSTANCE-IMPL", "TRANSITION-REFS", "RPT-CONTAINER", "DLT-LOG-CHANNEL-DESIGN-REF", "WAKEUP-FORWARD-LOCAL-ENABLED", "REFERENCED-BASELINE", "ARBITRATIONS", "MODE-DECLARATION-GROUP-PROTOTYPE-REF", "REF-MEASUREMENT-SET", "ECUC-INSTANCE-REFERENCE-VALUE", "PDU-COLLECTION-PDU-TIMEOUT", "EVENT-GROUP-REF", "I-PDU-MAPPING", "VLAN", "AUTH-DATA-FRESHNESS-LENGTH", "ANNOTATION-ORIGIN", "FLAT-INSTANCE-REF", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "SAFETY-CONDITIONS", "MACHINE-MODE-IREF", "DESTINATION-ORDER", "MIN-MAJOR-VERSION", "SUPPORTS-MULTIPLE-INSTANTIATION", "SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING", "MAX-BUFFER-REQUEST", "NEXT-STATE-IREF", "VLAN-MEMBERSHIP", "APMC-FLOAT-PARAM-DEF", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-PORT-MAPPING", "REMOTE-SUBJECT-REFS", "DEFAULT-PATTERN", "CURRENT-DATA-TYPE-REF", "DLT-MESSAGE-REF-CONDITIONAL", "IEEE-1722-RAW-DATA-STREAM-CONSUMER-INTERFACE", "ELEMENT-ROLE", "STATE-CLIENT-INTERFACE", "GRANT-REFS", "SWC-TO-SWC-OPERATION-ARGUMENTS", "TGROUP", "CONTEXT-DATA-PROTOTYPE-IN-CS-REF", "CRYPTO-KEY-SLOT-USER-DESIGN-MAPPING", "TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE", "ACTIVATION", "MODE-USER-ERROR-BEHAVIOR", "DISABLE-END-TO-END-CHECK", "SD-CLIENT-TIMER-CONFIGS", "USE-VARIABLE-DA", "STRUCTURED-REQ", "REQUIRED-OUTER-PORT-REF", "TCP-KEEP-ALIVE-PROBES-MAX", "CONTEXT-ROOT-COMPONENT-REF", "DO-IP-TARGET-ADDRESS-REF", "REQUIRED-ENVIRONMENT-REF", "VERIFICATION-METHOD", "RPT-SERVICE-POINT-PRE-REFS", "TTCAN-COMMUNICATION-CONNECTOR", "FIRST-PORT-REF", "NM-ECU", "ALIGNMENT", "TRANSFER-REF", "DETERMINISTIC-SYNC-M-OUT-OF-N", "SOURCE-SERVICE-INTERFACE-REF", "TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE", "TIMING-CONDITIONS", "TD-HEADER-ID-FILTERS", "INVERSE-PROTOCOL-VERSION", "SW-CALPRM-IMPL", "MODE-ACCESS-POINTS", "SYNC-CONFIRMATION-TIMEOUT", "CRYPTO-ELLIPTIC-CURVE-PROPS", "ECUC-FUNCTION-NAME-DEF-VARIANTS", "ECUC-MULTIPLICITY-CONFIGURATION-CLASS", "E-2-E-PROFILE-CONFIGURATION-SET", "P-2-MAX", "IS-AUTO-VALUE", "PORT-API-OPTIONS", "STORAGE-CONDITION-GROUP-REF", "TYPE-TREF", "EGRESS-PORT-REF", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "CALLBACK-HEADER-REF", "TCP-REMOTE-PORT", "AUTO-IP-PROPS", "SERVICE-INTERFACE-METHOD-MAPPING", "PROVIDED-COM-SPECS", "LOG-TRACE-LOG-MODE", "MODE-REPORTER-PORT-IREF", "STARTUP-OPTION", "RVF-FRAME-RATE", "SWC-TO-APPLICATION-PARTITION-MAPPING", "OBJECT-DEFINTION-REF", "SYNC-JUMP-WIDTH", "RPT-READS", "BSW-BACKGROUND-EVENT", "TIME-BASE-RESOURCE-REFS", "SESSION-HANDLING", "AH-CIPHER-SUITE-NAMES", "MINIMUM-SUSTAINED-SIZE", "I-SIGNAL-TO-I-PDU-MAPPING", "AP-APPLICATION-ERROR-REFS", "RTE-EVENT-IN-SYSTEM-SEPARATION", "DDS-CP-SERVICE-INSTANCE-OPERATION", "EXPIRED-SUPERVISION-TOLERANCE", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "EVENT-IREF", "BSW-MODULE-DESCRIPTION", "GROUPED-ALIAS-EVENT-REFS", "CAPABILITY", "SOMEIP-SERVICE-VERSION", "MINIMUM-TX-CONTAINER-QUEUE-SIZE", "IPV-4-RULE", "LOCAL-SUPERVISION-ENTITY-REF", "COLD-START-ATTEMPTS", "DETERMINISTIC-SYNC-MASTER", "SIGNATURE-SCHEME-REFS", "COMPUTED", "ADD-INFO-5", "DESTINATION-PHYSICAL-ADDRESS", "CHECKPOINT-REFS", "IDS-DESIGN", "BULK-NV-DATA-DESCRIPTOR", "PHYSICAL-DIMENSION-MAPPING-REF", "DIAGNOSTIC-ADDRESSS", "COMMUNICATION-CONTROLLER-REF", "DIAGNOSTIC-SECURE-CODING-MAPPING", "SEC-OC-CRYPTO-MAPPING-REF", "RESET-CONFIRMED-BIT-ON-OVERFLOW", "LOCAL-CERTIFICATE-REF", "COMMUNICATION-CONTROLLER-MAPPING", "P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF", "THRESHOLD-NUMBER", "COMMUNICATION-BUFFER-LOCKING", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "TRANSPORT-LAYER-RULE", "MC-GROUP-DATA-REF-SET-VARIANTS", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "RELEASED-TRIGGER-REF", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "FUNCTION-GROUP-MODE-IREFS", "NV-DATAS", "SCOPE-REFS", "ROLE-BASED-DATA-ASSIGNMENT", "FORMAL-BLUEPRINT-GENERATOR", "DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "ASSOCIATED-RTE-PLUGIN-REF", "WRITE-CLASS-REF", "IEEE-1722-TP-RULE", "ARTIFACT-DESCRIPTORS", "ERROR-CODE", "AGGREGATION-REF", "DATA-PROTOTYPE-GROUP-IREF", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS", "CUSTOM-SPECIFICATION-REFS", "CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE", "SECURED-RX-VERIFICATION", "SW-DIAGNOSTICS-NOTESS", "AUTHENTICATION-ENABLED", "RELIABILITY-MAX-BLOCKING-TIME", "VFC-IREF", "TRIGGER-MODE", "BSW-MODULE-ENTRY", "ELLIPTIC-CURVE-REF", "RESOURCE-NEEDS-REF", "ROLE-BASED-MC-DATA-ASSIGNMENT", "TCP-SYN-RECEIVED-TIMEOUT", "FEATURE-MODEL-REF", "COUPLING-PORT-CONNECTION", "INGRESS-PORT-REF", "GENERIC-DIAGNOSTIC-TRANSPORT-INSTANTIATION", "STATIC-SOCKET-CONNECTION-REF", "MAC-LAYER-TYPE", "REASON", "MAX-EXP-BS", "INTERPOLATION-ROUTINE-MAPPINGS", "CONNECTION-IS-MANDATORY", "MAX-SUPPORTED-DID-LENGTH", "SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS", "AUTHENTICATION-PROPSS", "ACCURACY", "ID", "MACHINE-ID", "LIN-TP-CONNECTION", "COUPLING-PORT-ROLE", "CYCLE-COUNT-MAX", "FRAME-REF", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "PRE-ACTIVATION-REBOOT", "DLT-LOG-CHANNEL-REF", "FIRE-AND-FORGET-METHOD-MAPPING", "IDSM-RATE-LIMITATION", "SERVICE-ITEMS", "AGING-CYCLES", "CRYPTO-SERVICE-KEY-REF", "BYTE-VALUE", "USED-IMPLEMENTATION-DATA-TYPE-REF", "ACTIVATION-SWITCH", "MAC-SEC-CIPHER-SUITE-CONFIG", "ENHANCED-MODE-API", "FIELD-DEPLOYMENTS", "CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF", "PID-REFS", "BULK-NV-BLOCK", "IDS-COMMON-ELEMENT-REF-CONDITIONAL", "COMPOSITE-RULE-BASED-VALUE-SPECIFICATION", "IMPLEMENTED-IN-REF", "ECU-RESOURCE-MAPPINGS", "FM-SYSCOND", "MC-FUNCTION-DATA-REF-SET-CONDITIONAL", "TTL-MIN", "CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS", "DDS-CP-I-SIGNAL-TO-DDS-TOPIC-MAPPING", "PREDECESSOR-FIFO-REF", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "COLLECTION-SEMANTICS", "PORT-PROTOTYPE-BLUEPRINT-REF", "CRYPTO-OBJECT-TYPE", "NV-REQUIRE-COM-SPEC", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "TLS-JOB-REQUIREMENT-REF", "CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL", "LOCAL-SUPERVISION-ENTITY-REFS", "PARAMETER-ELEMENT-ACCESS", "EXTERN-OFFSET-CORRECTION", "SOURCE-MAC-ADDRESS-MASK", "FREEZE-FRAME-CONTENT-REF", "PROVIDED-DDS-EVENT-QOS-PROPS", "BUFFER-RATIO", "VALUE-DATA-TYPE-REF", "MAX-WITHOUT-CLOCK-CORRECTION-FATAL", "PORT-DEFINED-ARGUMENT-BLUEPRINT", "DATA-PROTOTYPE-GROUP", "OBJECT-REF", "PARAMETERS", "PNC-NM-REQUEST", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "FIREWALL-STATE-IREF", "ITEM", "SW-CLUSTER-MAPPINGS", "SAFETY-POLICY", "PREDECESSOR-REFS", "RECORD-ELEMENTS", "PAYLOAD-BYTE-PATTERN-RULE", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "DYNAMIC-RUNTIME-LENGTH-HANDLING", "REQUEST-TYPE", "MODE-DRIVEN-TRANSMISSION-MODE-CONDITION", "PRIORITYS", "PDELAY-REQUEST-PERIOD", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "DOIP-INTERFACES", "MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "CRC-SEQUENCE-ID", "PROTOCOL-NAME", "BLUEPRINT-REF", "TLS-CIPHER-SUITES", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "VALUE", "TRANSMISSION-MODE-TRUE-TIMING", "LISTEN-TIMEOUT", "SECURITY-EVENTS", "END-TO-END-TRANSFORMATION-DESCRIPTION", "DO-IP-ENTITY", "RECEIVER-REFS", "MICRO-INITIAL-OFFSET-B", "SW-IMPL-MAPPINGS", "INHIBITING-FID-REF", "MAX-RPT-EVENT-ID", "ATTRIBUTE-NAME", "TCP-IP-ICMP-PROPS-REFS", "FRAME-TRIGGERINGS", "NM-MSG-REDUCED-TIME", "COM-CONFIGURATION-TX-TIME-BASE", "DIAG-PDU-TYPE", "SIGNALS", "I-PDU-MAPPINGS", "APPLICATION-ASSOC-MAP-DATA-TYPE", "ETH-IP-PROPS-REFS", "DO-IP-ACTIVATION-LINE-NEEDS", "CLEAR-CONDITION-REFS", "REQUEST-2-SUPPORT", "DATA-IDS", "SECOND-VALUE", "DHCP-ADDRESS-ASSIGNMENT", "ECUC-LINKER-SYMBOL-DEF", "SW-GENERIC-AXIS-PARAM-TYPE", "CHECKPOINT-INITIAL-REFS", "INHIBIT-SOURCES", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF", "AUTHENTICATION-REF", "MAX-BYTES-IN-INTERVAL", "NM-GLOBAL-COORDINATOR-TIME", "SENDER-RECEIVER-TO-SIGNAL-MAPPING", "IDSM-RATE-LIMITATION-REF", "RELATED-TRACE-ITEM-REF", "J-1939-DCM-DM-19-SUPPORT", "SOCKET-ADDRESS", "COM-TRIGGER-GRANT", "MEMORY-SECTION", "CRYPTO-SIGNATURE-SCHEME", "INTERNET-HEADER-LENGTH", "LC-OBJECT-REF", "PLCA-PROPS", "SW-CARB-DOCS", "DLT-SESSION-ID", "DOMAIN-IDS", "PDU-REPLICATIONS", "PERIOD-END", "VARIABLES", "BIT", "PDU-COLLECTION-TRIGGER", "SUB-TLV-CONFIG", "USER-DEFINED-COMMUNICATION-CONTROLLER", "DATA-EXCHANGE-POINT", "PROCESSOR-CORE", "FLEXRAY-CLUSTER-VARIANTS", "L-4", "SDG-TAILORING", "PURPOSE-OF-UPDATE-REF", "FUP-DATA-ID-LIST", "SWC-TIMING", "OUTER-PORTS", "STORE-EVENT-SUPPORT", "VIEW-MAPS", "RP-IMPL-POLICY", "DATA-IREF", "EVENT-HANDLER", "QUEUE-SIZE", "MIN-TRCV-DELAY-COMPENSATION-OFFSET", "TEXT-TABLE-VALUE-PAIR", "ASSIGNED-FRAME-TRIGGERING-REF", "DIAGNOSTIC-J-1939-SW-MAPPING", "EVENT-REF", "SW-COMPARISON-VARIABLES", "FOREIGN-MODEL-REFERENCE", "SLOT-CAPACITY", "TIMING-MODE-REF", "S-3-SERVER-TIMEOUT", "DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING", "PERIODIC-RESPONSE-UUDT-REF", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "SECURITY-EVENT-CONTEXT-DATA-ELEMENT", "REST-ENDPOINT-PUT", "MIN-MARGIN", "CAN-COMMUNICATION-CONTROLLER", "PURE-LOCAL-TIME-BASE", "FRAME-LENGTH", "TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED", "DIAGNOSTIC-CONDITION-REF", "INDUSTRY-GROUP", "IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF", "I-SIGNAL-GROUP", "PERIODIC-RESPONSE-UUDT-REFS", "APPL-WATCHDOG-LIMIT", "FLAT-MAP-ENTRY-REF", "NM-MAIN-FUNCTION-PERIOD", "ROOT-SOFTWARE-COMPOSITIONS", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT", "SECURE-COM-CONFIGS", "TLV-DATA-IDS", "IS-UNUSED", "SM-INTERACTS-WITH-NM-MAPPING", "EXTERN-RATE-CORRECTION", "CLIENT-PORT-FROM-CONNECTION-REQUEST", "CLIENT-SERVER-OPERATION-REF", "UDP-COLLECTION-TRIGGER", "LIMIT", "V-LAN-REF", "PAYLOAD-BYTE-PATTERN-RULE-PART", "STATE-MANAGEMENT-PHM-ERROR-INTERFACE", "MAX-VALUE", "PNC-VECTOR-OFFSET", "SOCKET-OPTIONS", "STREAM-GATE-REF", "UPPER-BOUND", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "DEST-META-CLASS", "MAXIMUM-MEMORY-CONSUMPTION", "MAX-CONVERSATIONS", "LOGICAL-ADDRESS", "PLCA-MAX-BURST-TIMER", "DOCUMENT-ELEMENT-SCOPES", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "FILE-REFS", "MAX-INSTANCES", "RPT-SW-PROTOTYPING-ACCESS", "TCP-ALIVE-CHECK-RESPONSE-TIMEOUT", "PARAMETER-ACCESSED-FOR-DEBUG-REF", "FILTER-PRIORITY", "TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE", "SW-SERVICE-IMPL-POLICY", "ECU-INSTANCE-REF-CONDITIONAL", "FLEXRAY-TP-NODE", "ENVIRONMENT-CAPTURE-TO-REPORTING", "FLEXRAY-COMMUNICATION-CONTROLLER", "ICV-FRESHNESS-VALUE-ID", "RELIABILITY", "POST-BUILD-VARIANT-SUPPORT", "DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT", "SHALL-RUN-ON-REFS", "IMPLEMENTATION-PROPSS", "TYPE", "MIRROR-STATUS-CAN-ID", "TRACEABLE-TABLE", "MC-DATA-ASSIGNMENTS", "VTF", "SD-CLIENT-EVENT-TIMING-CONFIG", "DEST-SDG-REF", "TCP-FAST-RECOVERY-ENABLED", "NM-RANGE-CONFIG", "PORT-REF", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "RESOURCE-GROUPS", "ACCESS-MODE", "EXCLUSIVE-AREA", "APPLICATION-RULE-BASED-VALUE-SPECIFICATION", "TTCAN-CLUSTER-VARIANTS", "PDU-COLLECTION-MAX-BUFFER-SIZE", "IEEE-1722-TP-CONNECTION-REF-CONDITIONAL", "KEY-SLOT-USED-FOR-START-UP", "WRITTEN-NV-DATA", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "ENCAPSULATED-ENTRY-REF", "UPSTREAM-REFERENCE-IREF", "REG-REQUIRES-STABILITYS", "NM-MESSAGE-TIMEOUT-TIME", "EVALUATED-VARIANT-REF", "FR-CLUSTER-REF", "TARGET-POSSIBLE-ERROR-REF", "IMPLEMENTATION-REF", "UPPER-MULTIPLICITY-IS-INFINITE", "SW-RECORD-LAYOUT-GROUP-INDEX", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "FIELD-REFS", "DIAGNOSTIC-SECURITY-ACCESS", "I-SIGNAL-PORT-REFS", "TIMING-CLOCKS", "ROW", "COUPLING-PORT-STARTUP-ACTIVE-TIME", "CONSTANT-MAPPING-REF", "STATE-DEPENDENT-STARTUP-CONFIGS", "AVERAGE-MEMORY-CONSUMPTION", "USES-END-TO-END-PROTECTION", "LOGIC-ADDRESSS", "SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE", "TASK", "CLEAR-DTC-LIMITATION", "TIMING-REQUIREMENTS", "ECUC-COND", "CRC-FLAGS", "MODIFICATION", "INFO-TYPE-REF", "SW-RECORD-LAYOUT-GROUP-AXIS", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF", "MEMORY-USAGE", "PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS", "HANDLE-TERMINATION-AND-RESTART", "TRANSPORT-PDU-REF", "APPLIED-STANDARDS", "TCP-TTL", "ASSOCIATED-PDUR-I-PDU-GROUP-REFS", "HANDLE-OUT-OF-RANGE-STATUS", "FRAME-MAPPING", "ACCESS-COUNT", "P-PORT-PROTOTYPE-IREF", "MIN-SAMPLE-POINT", "EXPORTABILITY", "IKE-REKEY-TIME", "OFFER-CYCLIC-DELAY", "IPV-4-ADDRESS-SOURCE", "BUFFER-OUTPUT", "DIAG-REQUIREMENT", "LOCAL-ID", "SUPPORTED-FEATURES", "FILE-TYPE-PATTERN", "UPPER-CAN-ID", "GLOBAL-TIME-PDU-TRIGGERING-REF", "DEFAULT-VALUE-ELEMENT", "CONTEXT-SWC-PROTOTYPE-REF", "SERVICE-ELEMENT-SECURE-COM-CONFIG-REF", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "ACF-BUS-REFS", "NETWORK-ENDPOINT-REF", "MAPPED-AP-DATA-ELEMENT-IREF", "RECEPTION-POLICYS", "INTERFACE-BLUEPRINT-MAPPINGS", "FINAL-CHECKPOINT-REFS", "REQUEST-TOPIC-NAME", "ARTIFACT-CHECKSUM-REF", "ACL-ROLE-REF", "HW-ATTRIBUTE-VALUES", "TRANSITION-REF", "FLEXRAY-FRAME", "LOG-TRACE-PORT-ID", "TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS", "PROVIDED-DDS-OPERATIONS", "APMC-CONTAINER-VALUE", "IDS-TIME-BASE-REF", "EVENT-OBD-READINESS-GROUP", "GLOBAL-TIME-PDU-REF", "TTL-MAX", "UDP-RULE", "IMMEDIATE-RESUME-TIME", "TCP-MAX-RTX", "DEFAULT-REPORTING-MODE", "ADAPTIVE-AUTOSAR-APPLICATION", "EVENT-DEFAULT-VALUE", "MAC-SEC-PROPSS", "SECOND-DATA-PROTOTYPE-REF", "IMPLEMENTS-SOMEIP-STRING-HANDLING", "ACTIONS", "CLUSTERED-COMPONENT-IREF", "TRANSFER-EXIT-CLASS-REF", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "NONQUEUED-SENDER-COM-SPEC", "REMOTE-CONFIGS", "CONTEXT-REF", "TIMEOUT-SUBSTITUTION-VALUE", "ENUMERATION-MAPPING-TABLE", "I-SIGNAL-TRIGGERINGS", "CAN-TP-ADDRESS", "PULSE-TEST", "PROCESSING-KIND", "I-PV-6-EXT-HEADER-FILTER-LIST", "OVERRIDE-ID", "INIT-EVENT", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS", "INTERPOLATION-ROUTINE-MAPPING", "TRUE-ACTION-LIST-REF", "TCP-IP-DHCP-V-6-CNF-DELAY-MAX", "LOG-TRACE-FILE-PATH", "TRIGGER-PORT-ANNOTATION", "LOG-AND-TRACE-INTERFACE", "MESSAGE-SOURCE-FILE", "AR-RELEASE-VERSION", "IIDC-SY", "UNIT-DISPLAY-NAME", "IDENTITY-REF", "COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE", "RTE-EVENT-SEPARATIONS", "COMMUNICATION-CONTROLLERS", "PLCA-TRANSMIT-OPPORTUNITY-TIMER", "DIAGNOSTIC-IO-CONTROL", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS", "STORE-CYCLIC", "BOOLEAN-VALUE-VARIATION-POINT", "UCM-REF", "TARGET-HEALTH-CHANNEL-REF", "NV-RAM-BLOCK-ELEMENT", "TD-EVENT-SERVICE-INSTANCE-EVENT", "EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS", "INDEXED-ARRAY-ELEMENT", "ENUMERATOR-VALUE", "MODE-PORT-ANNOTATION", "CONSUMED-SERVICE-INSTANCE-REF", "PHYS-CONSTRS", "SUBMESSAGE-TYPE", "EXPRESSION", "OPERATION-IREF", "MAX-DELTA-COUNTER", "COMMUNICATION-RESOURCE-PROPS", "TIMING-EVENT", "DIAGNOSTIC-INDICATOR-INTERFACE", "TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP", "LOGICAL-EXPRESSIONS", "NOT-AVAILABLE-VALUE-SPECIFICATION", "VEHICLE-SYSTEM", "WORST-CASE-STACK-USAGE", "COMPU-METHOD-REF", "RECEPTION-DEFAULT-VALUES", "EXCLUSIVE-AREA-NESTING-ORDERS", "PDELAY-RESPONSE-ENABLED", "NM-NODE-REFS", "FLEXRAY-NM-NODE", "SOFTWARE-CLUSTER-ID", "COM-MANAGEMENT-PORT-GROUP-IREF", "CONTRACT-VERSION", "VEHICLE-PACKAGE", "COMMON-PROPERTIES", "DATA-LENGTH", "I-SIGNAL-TO-PDU-MAPPINGS", "OPTION-ARGUMENT", "FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL", "AFFECTED-REF", "IDENT", "SYNCHRONIZATION-CONSTRAINT-TYPE", "K-RUN-RESOURCE", "SYNCHRONIZED-TIME-BASE-PROVIDER", "DURABILITY-KIND", "TIMESTAMP-RX-ACCEPTANCE-WINDOW", "RESPONSE-ON-EVENT-SCHEDULER-RATE", "TLS-CRYPTO-SERVICE-MAPPING", "PORT-INSTANTIATION-BEHAVIOR", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "PREDECESSOR-ACTION-REFS", "TIME-SYNC-TECHNOLOGY", "SET-FUNCTION-GROUP-STATE-IREF", "OBD-INFO-SERVICE-NEEDS", "NETWORK-INTERFACE-REFS", "EXECUTABLE", "STATE-MANAGEMENT-NM-ACTION-ITEM", "MEMORY-ALLOCATION-KEYWORD-POLICY", "ADDRESS-RANGE-LOWER-BOUND", "ECUC-MULTILINE-STRING-PARAM-DEF", "MAX-EVENTS-IN-INTERVAL", "CONTEXT-ELEMENT-REFS", "SW-CLUSTERS", "TARGET-CATEGORY", "WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST", "RAW-DATA-STREAM-GRANT-DESIGN", "RECOVERY", "EXECUTABLE-ENTITY-REF", "REQUEST-CONFIGURATIONS", "R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "R-PORT-PROTOTYPE", "DTC-SETTING-CLASS-REF", "MEM-CLASS-SYMBOL", "ACCESSED-VARIABLE", "DIAGNOSTIC-PARAMETER-ELEMENT", "CAN-CONFIG", "LOAD-BALANCING-PRIORITY", "CLUSTERED-COMPONENT-IREFS", "TARGET-PORT-PROTOTYPE-REF", "REFERENCE-VALUE-SPECIFICATION", "INCLUDED-VARIANT-REF", "ECUC-INTEGER-PARAM-DEF", "GETS", "DDS-METHOD-REQUEST-TOPIC-REF", "COLLECTION-THRESHOLD", "PERSISTENCY-PORT-PROTOTYPE-IREF", "MULTICAST-SEGMENTATION", "KEEP-ALIVE-TIME", "COM-MGR-USER-NEEDS", "CLIENT-SERVER-OPERATION-MAPPING", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "TLS-CONNECTION-REFS", "ROLE", "ENABLE-CONDITION-GROUP-REF", "FRESHNESS-PROPSS", "LOGICAL-EXPRESSION-ARGUMENT-REF", "CALL-POINTS", "DIAGNOSTIC-CONDITION-INTERFACE", "SIGNATURE-SCHEME-ID", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF", "PROGRAMMING-LANGUAGE", "INDICATOR-STATUS-NEEDS", "COMPONENT-CLUSTERING", "EVENT-GROUPS", "CONTAINER-I-PDU-HEADER-BYTE-ORDER", "RAW-DATA-STREAM-INTERFACE", "SERVICE-INTERFACE-FIELD-MAPPING", "CONNECTOR-REFS", "DO-IP-INSTANTIATION", "NM-NODE-ID", "APPLICATION-PARTITION-REF", "INSTANTIATION-TIMING-EVENT-PROPS", "CLEAR-CONDITION-REF", "RULE-INIT-STATE", "GETTER-CALL-REF", "CUSTOM-SERVICE-ID", "COMPONENT-REF", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "ITEM-LABEL", "ATTRIBUTES", "AUTOSAR-DATA-TYPE-REF", "KEY-SIZE", "MACRO-INITIAL-OFFSET-B", "NETWORK-ENDPOINT", "NETWORK-CONFIGURATION-REFS", "SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF-CONDITIONAL", "REQUIREMENT-REFS", "CREATED-DATAS", "LOCAL-VARIABLE-REF", "TX-PDU-TRIGGERING-REF", "FLEXRAY-FIFO-CONFIGURATION", "ORDERED-ELEMENTS", "LAST-EGRESS-SCHEDULER-REF", "L-2", "POSSIBLE-AP-ERROR-REF", "MAJOR-VERSION", "BSW-QUEUED-DATA-RECEPTION-POLICY", "SIZE-OF-ARRAY-LENGTH-FIELDS", "WAKEUP-FORWARD-REMOTE-ENABLED", "DOMAIN", "PDU-TRIGGERINGS", "ARGUMENTS", "ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL", "OPERATION-REF", "SUB-NODE-PHYSICAL-CHANNEL-REF", "TEAR-DOWN-ACTION-REFS", "SW-INTERPOLATION-METHOD", "ROUTINE-INFO", "TD-EVENT-I-PDU", "USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL", "SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF", "COMMUNICATION-RESOURCE-REF", "NM-BUS-SYNCHRONIZATION-ENABLED", "IDSM-MODULE-INSTANTIATION", "SELECTIONS", "FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "MC-FUNCTION-DATA-REF-SET-VARIANTS", "ASSIGNED-DATAS", "SEPARATE-SIGNAL-PATH", "MAX-SYNC-JUMP-WIDTH", "ACL-CONTEXT", "NM-VOTE-INFORMATION", "R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF", "DATA-TYPE-MAP", "COMMUNICATION-CONNECTOR-REF", "EVENT-WINDOW-TIME", "SYSTEM-SIGNAL-GROUP-TO-COM-RESOURCE-MAPPINGS", "RUNNABLE-ENTITY-IREFS", "DDS-CP-CONSUMED-SERVICE-INSTANCE", "CAN-IDENTIFIER-MASK", "BSW-CALLED-ENTITY", "TRANSFORMATION-DESCRIPTIONS", "RELATIVE-REPRESENTATION-TIME", "BSW-ENTRY-RELATIONSHIP-TYPE", "DEADLINE-PERIOD", "PWM-S", "SEPARATION-TIME-REQUEST", "DDS-REQUIRED-SERVICE-INSTANCE", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "TP-TECHNOLOGY", "EXPIRED-SUPERVISION-CYCLES-TOLERANCE", "RESOURCE-ESTIMATIONS", "OPT-IMPL-RECORD-ELEMENTS", "ERROR-REACTION-POLICY", "ABS", "UNIT-REFS", "SW-CALPRM-AXIS", "J-1939-TP-CONFIG", "SIGNAL-FAN", "CRYPTO-JOB", "TRANSMISSION-TRIGGER", "DO-IP-TARGET-ADDRESS-REFS", "QUEUED-RECEIVER-COM-SPEC", "AP-APPLICATION-ERROR-SET", "DRIVER-NOTIFICATIONS", "NUMERICAL-VALUE-VARIATION-POINT", "REMOTE-SUBJECT-REF", "RPT-PROFILES", "NUMBER-OF-WORKERS", "DIAGNOSTIC-EVENT-INFO-NEEDS", "DIAGNOSTIC-DATA-IDENTIFIER-REF", "ECUC-QUERY", "MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE", "STORE-EMERGENCY", "UPDATE-KIND", "TARGET-R-PORT-REF", "HW-PIN-REFS", "TARGET-I-PDU", "SLAVES", "DEFAULT-VALUE-HANDLING", "USED-CODE-GENERATOR", "TIMING-DESCRIPTIONS", "REMOTE-MULTICAST-CONFIG-REFS", "KIND", "IMPLEMENTATION-ARRAY-ELEMENT-REF", "TD-EVENT-MODE-DECLARATION-TYPE", "LOCAL-PARAMETER-REF", "SECURITY-EVENT-DEFINITION-REF", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "TCP-IP-DHCP-V-6-INF-DELAY-MAX", "PROCESS-PHM-ACTION-ITEM", "SPECIFICATION-DOCUMENT-SCOPES", "REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF", "USE-SMART-SENSOR-API", "TRIGGERS-RECOVERY-NOTIFICATION", "ELEMENT-PROPSS", "COMMUNICATION-CLUSTER-REFS", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS", "SYNCHRONIZED-SLAVE-TIME-BASE", "FLEXRAY-AR-TP-CHANNEL", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "FIELDS", "CONTROL-PROVIDED-EVENT-GROUP-REFS", "SDG-TAILORINGS", "REQUIRED-FIBEX-ELEMENT-REFS", "EVENT-KIND", "ROUTINE-CONTROL-CLASS-REF", "FORMAL-BLUEPRINT-CONDITION", "CLASSIFICATION", "KEY-GENERATION", "TEX-MATH", "GLOBAL-TIME-DOMAIN", "PORT-SCHEDULER", "ENVIRONMENT-DATA-CAPTURE", "IDSM-INSTANCE-ID", "CRYPTO-R-PORT-COM-SPEC", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "I-PV-6-EXT-HEADER-FILTER-SET", "TTCAN-COMMUNICATION-CONTROLLER", "SECURE-TRANSLATION", "SEGMENT-POSITION", "AFFECTED-APPLICATION", "UNASSIGN-FRAME-ID", "POWER-DOWN-TIME", "CONTEXT-APPLICATION-RECORD-ELEMENT-REF", "PACKAGING-PIN-NAME", "SUBSTITUTED-FRAME-REF", "DYNAMIC-BS", "POSTS", "IP-SEC-RULE", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "METHOD-MAPPING", "MULTICAST-THRESHOLD", "REQUIRED-AR-ELEMENT-REFS", "CONSTANT-VALUE-MAPPING-REF", "SOCKET-CONNECTION-BUNDLE", "PORT-BLUEPRINT-MAPPINGS", "DIAG-FAULT-DETECTION-COUNTER-PORT-REF", "ROOT-SW-COMPOSITION-PROTOTYPE", "PAYLOAD-PREAMBLE-INDICATOR", "N-PDU-REF", "WRITING-FREQUENCY", "ENVIRONMENTAL-CONDITION-REF", "NM-USER-DATA-OFFSET", "REKEY-INTERVAL", "TRIGGER-PORT-ANNOTATIONS", "CP-SOFTWARE-CLUSTER-REF", "TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED", "OFNS-DATA-ID-LISTS", "DATA-PROTOTYPES", "CRC-SOURCE-PORT-IDENTITY", "CLOCK-REFERENCE-REF", "ECU-INSTANCE-REF", "SW-SYSTEMCONST-REF", "EXIT-TIMEOUT-VALUE", "CAT-2-ISR", "CONTEXT-DATA-ELEMENTS", "FM-FEATURE-MODEL", "SUPPLIER-ID", "LENGTH-EXP", "TIME-BASE-RESOURCE-REF", "DDS-FIELD-QOS-PROPS", "SWC-TO-SWC-SIGNAL", "PHM-CHECKPOINT-IREF", "REPETITION-PERIOD", "NODE-REF", "INVALIDATION-POLICYS", "INTERPOLATION-ROUTINE-MAPPING-SET-REF", "SWC-MODE-SWITCH-EVENT", "KEY-EXCHANGE-AUTHENTICATION-REF", "INSTANTIATION-DATA-DEF-PROPSS", "DIAGNOSTIC-ENV-DATA-CONDITION", "FRAME-TRIGGERING-REF", "MAPPINGS", "UNICAST-NETWORK-ENDPOINT-REFS", "SWC-IMPLEMENTATION", "DLT-APPLICATION", "STRING-ENCODING", "DISPLAY-IDENTIFIER", "PERSISTENCY-DEPLOYMENT-REF", "AP-APPLICATION-ERROR-DOMAIN", "APPLIES-TO", "AFFECTED-FUNCTIONAL-CLUSTER", "EXECUTION-DEPENDENCY", "CSE-CODE", "ASYNCHRONOUS-TRAFFIC-SHAPER-REF", "EXPLICIT-CONGESTION-NOTIFICATION", "FIRST-TRIGGER-REF", "SWC-BEHAVIOR-REF", "CLIENT-SERVER-INTERFACE-REF", "EVENT-HANDLER-REF", "TCP-IP-NDP-MAX-RANDOM-FACTOR", "PERSISTENCY-FILE-STORAGE-INTERFACE", "REMOTE-ENDPOINT-CONFIGURATION", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "TRIGGER-REF-CONDITIONAL", "MAXIMUM-SUPPORTED-UCM-VERSION", "FILE-TYPE", "J-1939-TP-PG", "HEALTH-CHANNELS", "LINKERS", "NETWORK-HANDLE-PORT-MAPPING", "ETHERNET-PHYSICAL-CHANNEL", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "RVF-TOTAL-LINES", "SW-AXIS-GROUPED", "DIAGNOSTIC-TROUBLE-CODE-REF", "DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF", "PACKAGER-SIGNATURE-REF", "CALL-SIGNAL-REF", "PARTITIONS", "SYSTEM-SIGNAL-TO-COM-RESOURCE-MAPPINGS", "FM-FEATURE-MAP-ELEMENT", "MODE-DECLARATION-GROUP-PROTOTYPE", "COM-MANAGEMENT-MAPPINGS", "METHOD-ACTIVATION-ROUTING-GROUPS", "I-SIGNAL-I-PDUS", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "REFINED-MODULE-DEF-REF", "TCP-PORT-REF", "COUPLING-ELEMENT", "ECUC-CONTAINER-VALUE", "EXTENDS-META-CLASS", "DATA-PROTOTYPE-IN-PORT-INTERFACE-REF", "CAN-FD-BAUDRATE", "UAS-ID", "MACHINE-MODE-IREFS", "SENDER-RECEIVER-INTERFACE", "ERROR-HANDLING", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "DATA-TRANSFORMATION", "SIGNAL-BASED-FIELD-DEPLOYMENT", "TLS-JOB-SEMANTIC", "MANAGED-COUPLING-PORTS", "PATH-MTU-TIMEOUT", "N-DATA-SETS", "MAJOR-PROTOCOL-VERSION", "SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "SUPPORTED-CONFIG-VARIANTS", "FIREWALL-RULE-PROPSS", "PARAMETER", "TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF", "TRAFFIC-CLASS", "ECU-ID", "XFILE", "SOURCE-ADDRESS-MASK", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "CAN-TP-ADDRESS-REFS", "FORMAT", "TIME-VALUE-VALUE-VARIATION-POINT", "DISPLAY-PRESENTATION", "SECOND-ELEMENT-INSTANCE-IREFS", "EXECUTION-ERROR-REF", "API-SERVICE-PREFIX", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "NM-COORD-SYNC-SUPPORT", "J-1939-PROTECTED-I-PDU", "IPV-4-ADDRESS-MASK", "UPPER-MULTIPLICITY-INFINITE", "DELEGATION-SW-CONNECTOR", "BUS-IDLE-TIMEOUT-PERIOD", "UPPER-HEADER-BITS-TO-SHIFT", "SERVICE-INSTANCE-REFS", "INTERNAL-PRIORITY-VALUE", "TT-CAN-CLUSTER-REF", "DIAGNOSTIC-CONTROL-NEEDS", "MODE-DRIVEN-TRUE-CONDITIONS", "FLEXRAY-CLUSTER-CONDITIONAL", "BSW-INTERNAL-TRIGGERING-POINT-REF", "J-1939-NM-CLUSTER", "VARIABLE-DATA-PROTOTYPE", "SDG-CAPTION", "NETWORK-REPRESENTATION-PROPS", "TYPE-REFERENCE-REF", "POST-BUILD-VARIANT-CONDITION", "DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING", "TD-EVENT-VFB-PORT-REF", "MINIMUM-FIFO-LENGTH", "POST-BUILD-VARIANT-CRITERION-VALUES", "CAN-TP-ECU", "TARGET-TRIGGER-REF", "REQUIRED-CLIENT-SERVER-ENTRY-REF", "CUSTOM-SDG-DEF-REFS", "PARTS", "SDG-CLASS", "ALLOCATOR", "SECURITY-EVENT-REPORT-INSTANCE-DEFINITION-REF", "NETWORK-ID", "RUNS-INSIDES", "DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING", "ST-MIN", "DO-IP-ROUTING-ACTIVATIONS", "DEFAULT-LEASE-TIME", "BGCOLOR", "PORT-API-OPTION", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "LIN-CLUSTER-VARIANTS", "SW-SERVICE-ARG", "DIAGNOSTIC-MONITOR-INTERFACE", "APPLICATION-CONTEXT-REFS", "FRAGMENT", "STANDARD-REVISION", "FILTER-ACTION-DEST-PORT-MODIFICATION", "SUPPORTED-TIMER-GRANULARITY", "I-PDU-PORT", "BSW-BEHAVIOR-REF", "ENDPOINTS", "CRYPTO-SERVICE-CERTIFICATE-REFS", "REQUIRED-TRIGGER-IREF", "DOC-REVISION", "DIAGNOSTIC-REQUEST-UPLOAD", "EVALUATED-ELEMENT-REFS", "CRC-OFFSET", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "GLOBAL-TIME-ETH-SLAVE", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "FILE-ELEMENTS", "HOP-COUNT", "SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF-CONDITIONAL", "FUNCTION-INHIBITION-NEEDS", "TCP-FIN-WAIT-2-TIMEOUT", "MASTER-REF", "DO-IP-FUNCTIONAL-CLUSTER-DESIGN", "SECURITY-LEVEL-REF", "PRIORITY", "OBD-RATIO-DENOMINATOR-NEEDS", "TARGET-PROVIDED-OPERATION-REF", "SW-MC-BASE-TYPE", "FEATURE-IREF", "TRANSMISSION-MODE-CONDITIONS", "LC-STATES", "CRYPTO-KEY-SLOT-DESIGN", "OUTER-PORT-REF", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "DDS-CP-SERVICE-INSTANCE-EVENT", "PNC-FILTER-DATA-MASK", "REQUIRED-DATAS", "SOMEIP-REMOTE-MULTICAST-CONFIG", "DOIP-CHANNEL-COLLECTION-REF", "TRANSMISSION-DEADLINE", "AFFECTED-BSW-MODULE", "FUNCTION-POINTER-SIGNATURE-REF", "IEEE-1722-TP", "POLICY-ACTION", "FLOW-CONTROL-PDU-REF", "ACCESSED-MODE-GROUPS", "CONSISTENCY-NEEDS", "DDS-OPERATION-REQUEST-TRIGGERING-REF", "CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS", "DEADLINE-SUPERVISIONS", "APPL-CONSTANT-REF", "TROUBLE-CODE-J-1939-REF", "AUTHENTICATION-ROLE-REFS", "DIAGNOSTIC-EVENT-INTERFACE", "COMPONENTS", "DATA-TRANSFORMATION-REF", "TIMEOUT-BR", "KEY-VALUE-STORAGE-REF", "REQUIRED-RTE-VENDOR", "DYNAMIC-PART", "TYPE-REFERENCE", "WAKE-UP-BY-CONTROLLER-SUPPORTED", "ROLE-BASED-RESOURCE-DEPENDENCY", "DHCP-SERVER-CONFIGURATION", "PNC-FILTER-ARRAY-MASKS", "DLT-APPLICATION-REF", "HTTP-TP", "MINIMUM-INTENDED-BINDING-TIME", "I-SIGNAL-REFS", "MACHINE", "IEEE-1722-TP-ACF-CONNECTION", "TLV-RECORD-ELEMENT-REF", "TIME-BASE-PROVIDER-REF", "BUS-MIRROR-CHANNEL-MAPPING-IP", "CONTROL-PNC-REFS", "MEMORY-ENTRY-STORAGE-TRIGGER", "ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT", "NUMERICAL-RULE-BASED-VALUE-SPECIFICATION", "DIAGNOSTIC-AUTHENTICATION-REF", "LUMINOUS-INTENSITY-EXP", "PNC-WAKEUP-CAN-ID-EXTENDED", "TRANSFORMER-CHAIN-REFS", "CYCLE-REPETITION", "ACTIVATION-POINTS", "TARGET-REFS", "HW-PORT-MAPPING", "APP-ID", "CONTROLLED-FID-REF", "MATCHING-EGRESS-RULE-REF", "MODE-DECLARATION", "DO-IP-LOGICAL-ADDRESS", "METHOD-REQUEST-PROPSS", "HW-PIN-GROUP-REFS", "ECUC-PARAM-CONF-CONTAINER-DEF", "APPLICATION-MODE-IREF", "ASYNCHRONOUS-SERVER-CALL-POINT-REF", "CAN-NM-CLUSTER", "APMC-TEXTUAL-PARAM-VALUE", "DEFAULT-PRIORITY", "IMPL-CONSTANT-REF", "CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS", "TLV-DATA-ID-DEFINITION-REF", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "TIMESTAMP-INTERVAL", "PERSISTENCY-FILE-ARRAY", "CLEAR-FROM-VALID-TO-INVALID", "DIAGNOSTIC-ADDRESSES", "COMM-CONTROLLER-REF", "TLS-CONNECTION-REF", "RATE-BASED-MONITORED-EVENT-REF", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "NM-CLUSTER-COUPLINGS", "HW-ATTRIBUTE-DEF-REF", "DO-NOT-FRAGMENT", "VLAN-MEMBERSHIPS", "BOR-TIME-TX-ENSURED", "RULE", "NO-SUPERVISIONS", "LATEST-TX", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "TTCAN-PHYSICAL-CHANNEL", "USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT", "SYSTEM-SIGNAL-GROUP", "COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS", "SA-REKEY-TIME", "DATA-TYPE-MAPPING-REF", "COUNTER-DECREMENT-STEP-SIZE", "LIN-SLAVE-VARIANTS", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "USER-DATA-SUB-TLV", "COMMITTED-BURST-SIZE", "SOURCE-IP-ADDRESS", "SELECTOR-FIELD-LENGTH", "HW-PIN-GROUP-CONNECTIONS", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "CAN-XL-NM-PROPS", "HEAP-USAGES", "APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-DEF", "PERIOD-BEGIN", "NUMBER-OF-NOTIFIER-SETS", "ECUC-LINKER-SYMBOL-DEF-CONDITIONAL", "TARGET-RUNNABLE-ENTITY-GROUP-REF", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "TCP-OPTION-FILTER-LISTS", "SEVERITY", "ACCURACY-INT", "READER-ENTITY-ID", "FM-FEATURE-RESTRICTION", "DIAGNOSTIC-TEST-RESULT-REF", "AGING-ALLOWED", "DATA-CONSTR-REF", "VARIABLE-INSTANCE", "STREAM-FILTER-RULE", "MULTIPLICITY-RESTRICTION", "ECU-TIMING", "BSW-VARIABLE-ACCESS", "PRE-ACTIVATE-IREF", "BSW-MODULE-ENTITY-REF", "DEPENDS-ON", "ASSIGN-FRAME-ID", "INFORMAL-FORMULA", "CLIENT-SERVER-ANNOTATIONS", "ARRAY-SIZE-HANDLING", "RAW-DATA-STREAM-REF", "GENERIC-MATH", "CAN-CONTROLLER-CONFIGURATION", "CONSTANT-SPECIFICATION", "JITTER", "SECOND-MODE-REF", "SECOND-MODE-GROUP-REF", "TARGET-EEC-REFS", "SW-TEST-DESC", "TP-ADDRESS-EXTENSION-VALUE", "FIRST-OPERATION-REF", "INPUT-DATAS", "DETERMINISTIC-CLIENT-REF", "PERSISTENCY-FILE-ARRAY-REF", "FOLLOW-UP-OFFSET", "ROOT-AUTOSAR-DATA-PROTOTYPE-REF", "DURABILITY-SERVICE-MAX-INSTANCES", "REQUIRED-SERVICE-VERSION", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "EVENT-GROUP-IDENTIFIER", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "PRIMITIVE-FAMILY", "FIGURE", "DESTINATION-MIN-ADDRESS", "FIELD-SENDER-COM-SPEC", "COLLECTION-TIMEOUT", "READ-DATA-CLASS-REF", "INDEX", "LIN-NM-CLUSTER", "CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "SECURITY-EVENT-DEFINITION", "PROVIDED-SERVICE-INSTANCE-REF", "ROOT-DATA-PROTOTYPE-IN-SR-REF", "BSW-SERVICE-DEPENDENCY", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "OPTION-NAME", "INSTANCES", "REFERENCE-VALUE-REF", "UCM-MASTER-MODULE-INSTANTIATION", "SCALE-CONSTRS", "SUPPORT-DIRTY-FLAG", "NM-NODE-REF", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "INNER-GROUP-IREF", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "NETWORK-ENDPOINT-REF-CONDITIONAL", "PROVIDED-OUTER-PORT-REF", "OPERATION-PROTOTYPE-IREF", "V", "MIN-DATA-LENGTH", "SYSTEM-MEMORY-USAGE", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "CRYPTO-KEY-SLOTS", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "APMC-STRING-PARAM-DEF", "EXCLUSIVE-AREA-REF-CONDITIONAL", "DIAGNOSTIC-STORAGE-CONDITION-REF", "DATA-RECEIVE-POINT-BY-ARGUMENTS", "PROTOCOL-KIND", "CHECKPOINTSS", "COM-MANAGEMENT-GROUP-REFS", "SHORT-TERM-ADJUSTMENT", "J-1939-REQUESTABLE", "TTCAN-CLUSTER-CONDITIONAL", "DDS-CP-DOMAIN", "TIMING-VARIABLE-REF", "SUPERVISED-ENTITY-NEEDS", "START-INDEX", "POST-BUILD-VALUE-ACCESS-REF", "ICMP-RULE", "FIGURE-CAPTION", "DEBOUNCE-ALGORITHM", "MSG-ID-MASK", "IN-PLACE", "PERMISSION-CERTIFICATE-AUTHORITY-REF", "TARGET-PDU-TRIGGERINGS", "VLAN-MODIFIER-REF", "CONTEXT-ELEMENT-REF", "RATE-DEVIATION-MEASUREMENT-DURATION", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "DESTINATION-ORDER-KIND", "RECEIVER-IREFS", "PORT-GROUPS", "SYNCHRONIZED-TRIGGERS", "IP-V-6-PATH-MTU-TIMEOUT", "TD-EVENT-FRAME", "INGRESS-VLAN-ID", "BLOCK-IF-STATE-ACTIVE-AP-IREFS", "PRE-ACTIVATE-REF", "ASSOCIATED-COM-I-PDU-GROUP-REF", "BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL", "SOFTWARE-CLUSTER-COM-RESOURCE-REF", "FRESHNESS-COUNTER-SYNC-ATTEMPTS", "DIAGNOSTIC-CONTROL-DTC-SETTING", "LITERAL-PREFIX", "GLOBAL-SUPERVISION-ENTITYS", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS", "SIGNAL-I-PDU-COUNTER", "SW-CALIBRATION-ACCESS", "LOCAL-SUPERVISION-REFS", "CRYPTO-KEY-SLOT-USAGE-DESIGN-MAPPING", "CAN-NM-NODE", "BUCKET-SIZE", "SOURCE-FRAME-REF", "CONDITION", "START-ACTION-REFS", "DIAGNOSTIC-SOVD-UPDATE-INTERFACE", "SEC-OC-JOB-REQUIREMENT-REF", "CRYPTO-KEY-SLOT-USER-DESIGN", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "CRF-PULL", "ETH-GLOBAL-TIME-DOMAIN-PROPS", "SW-CLASS-IMPLS", "MODE-DECLARATION-MAPPING", "SOMEIP-EVENT", "DEADLINE-MAX", "IS-OPTIONAL", "REFERENCE-BASES", "MATCHING-RULE-REF", "DENOMINATOR-GROUP", "SEGMENT-RECEPTION-TIMEOUT-TIME-REQUEST", "ENTRY-TYPE", "SECURE-COM-PROPS-SET", "CRYPTO-CERTIFICATE", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "SIGNAL-I-PDU-REPLICATION", "SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS", "TD-EVENT-TRIGGER-TYPE", "CHANNEL-ID", "SW-REFRESH-TIMING", "REVISION-LABEL-P-1", "TX-MASK", "OFS-DATA-ID-LISTS", "PORT-PROTOTYPE-REF-CONDITIONAL", "TEXT", "RESPONSE-ON-EVENT-CLASS-REF", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "INTERFACE-MAPPING-SET", "END-TO-END-PROTECTIONS", "COMPOSITION-SW-COMPONENT-TYPE", "UNICAST-UDP-CREDENTIALS", "NM-DATA-INFORMATION", "DO-IP-DESIGN-REF", "GENERIC-MODULE-INSTANTIATION", "RPT-WRITES", "BSW-MGR-NEEDS", "MIN-PWM-S", "HEADER-LENGTH", "DDS-RULE", "LIN-CLUSTER", "COM-METHOD-GRANT", "TARGET-R-PORT-PROTOTYPE-REF", "PORT-BLUEPRINT-MAPPING", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "MODIFIED-DATAS", "PRM-UNIT", "SHALL-NOT-RUN-ON-REFS", "SUPERVISED-ENTITY-IREF", "SECURITY-EVENT-DEFINITION-REF-CONDITIONAL", "CRYPTO-ALGO-CONFIG", "NM-PASSIVE-MODE-ENABLED", "SEC-OC-DEPLOYMENT", "REQUIRED-COM-SPECS", "ALLOCATOR-REF", "SW-GENERIC-AXIS-PARAM", "RETURN-SIGNAL-REF", "EXECUTION-CONTEXT", "ALGORITHM-SECONDARY-FAMILY", "SDG-CAPTION-REF", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "OPERATOR", "ECUC-QUERY-EXPRESSION", "CRC-VALIDATED", "REDUNDANCY-HANDLINGS", "ENTRYS", "ASSIGN-FRAME-ID-RANGE", "CAN-XL-CONFIG", "COMPOSITE-INTERFACE", "APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION", "WINDOW-SIZE", "GLOBAL-TIME-FR-MASTER", "CONTROLLED-DATA-ELEMENT-REFS", "RULE-BASED-AXIS-CONT", "CYCLE-COUNTER", "TCP-IP-NDP-DELAY-FIRST-PROBE-TIME-VALUE", "J-1939-SHARED-ADDRESS-CLUSTER", "VENDOR-SPECIFIC-MODULE-DEF-REFS", "EOC-EXECUTABLE-ENTITY-REF", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE", "USES-MONITOR-DATA", "ETH-TCP-IP-PROPS", "STORE-AT-SHUTDOWN", "TD-EVENT-I-SIGNAL", "SPORADIC-EVENT-TRIGGERING", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "VARIABLE-ACCESS", "BUS-OFF-RECOVERY", "SUB", "TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "CAN-CONTROLLER-XL-ATTRIBUTES", "ACTION-LIST-ITEM-REF", "USED-FID-REF", "AUTO-START", "SW-FILL-CHARACTER", "NM-REPETITION-CYCLE", "PNC-GATEWAY-TYPE", "COMMANDS", "PROVIDED-EVENT-GROUPS", "TEMPLATE-ARGUMENTS", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "RECOMMENDED-CONFIGURATION-REF", "PARAMETER-ACCESSS", "DIAGNOSTIC-PROTOCOL", "STEP-SIZE", "DIAGNOSTIC-ROUTINE-INTERFACE", "PREDECESSOR-ACTION-REF", "SUB-NODE-CHANNELS", "SECURE-COMMUNICATION-PROPS", "DERIVED-PORT-PROTOTYPE-REF", "CLIENT-CAPABILITY", "MAX-AVAILABLE-PERSISTENCY-STORAGE-SPACE", "BLOCK-STATE", "REQUEST-VEHICLE-INFORMATION-CLASS-REF", "I-SIGNAL-MAPPING", "COUNTER-JUMP-DOWN-VALUE", "BSW-MODE-SENDER-POLICY", "CONTEXT-SWC-PROTOTYPE-REFS", "SHUTDOWN", "MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID", "TYPE-OF-DTC-SUPPORTED", "VIOLATED-SAFETY-CONDITION-BEHAVIOR", "SPN", "RPT-EXECUTION-CONTROL", "PROCESS-REF", "SENDING-NODE-REF", "IS-SYNCHRONOUS", "CLIENT-COM-SPEC", "DESTINATION-NETWORK-MASK", "BASE-REFERENCE-REF", "WRITTEN-LOCAL-VARIABLES", "SOURCE-REF", "FLAT-MAP-REF", "SW-ADDR-METHOD", "BLUEPRINT-POLICYS", "DIAGNOSTIC-SOVD-LOCK", "PRODUCT-ID", "IIDC-STREAM-FORMAT", "ON-FAIL-PERMISSIVE-MODE-TIMEOUT", "BUS-MIRROR-CAN-ID-RANGE-MAPPING", "CHANNEL-SYNCHRONOUS-WAKEUP", "SUPERVISION-ENTITY", "SECURE-COM-CONFIG-REF", "RELEASE-NOTES-REF", "FIBEX-ELEMENTS", "APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-DEF", "TA-TYPE", "WAKEUP-SYMBOL-RX-WINDOW", "ROOT-DATA-PROTOTYPE-IN-CS-REF", "TIME-TRIGGERED-CAN-LEVEL", "TP-CONNECTION-REF", "ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS", "CLASS-CONTENT-CONDITIONAL", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "FM-FEATURE-SELECTION-SET", "START", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "DISTINGUISHED-PARTITIONS", "DATA-ID-NIBBLE-OFFSET", "PNC-MAPPING-REFS", "IN-MEASUREMENT-SET", "REQUEST-METHOD", "AP-SOMEIP-TRANSFORMATION-PROPS", "NV-DATA-INTERFACE", "SIGNAL-MAPPINGS", "IDENTITY-CERTIFICATE-AUTHORITY-REF", "TX-BIT-RATE-SWITCH", "I-SIGNAL-I-PDU-REF", "INCLUDE-REFS", "SPECIFIC-CHANNEL-REF", "SOVD-VALIDATION-ORDER-REF", "PARAMETER-VALUES", "MODIFICATIONS", "DDS-METHOD-QOS-PROPS", "CONCRETE-PATTERN-EVENT-TRIGGERING", "DESTINATION-MAC-ADDRESS", "ADMIT-WITHOUT-MESSAGE-ID", "DDS-CP-PROVIDED-SERVICE-INSTANCE", "MAX-FR-IF", "CYCLIC-BACKUP-INTERVAL", "MKA-PARTICIPANT-REF", "I-SIGNAL", "TP-CONFIGURATION", "SW-CLASS-IMPL", "OCCURRENCE-EXPRESSION", "BSW-COMPOSITION-TIMING", "CYCLE-TIME-MAIN-FUNCTION", "SUB-SOFTWARE-CLUSTER-REFS", "RESOURCE-GROUP", "HANDLE-OUT-OF-RANGE", "MEMORY-HIGH-ADDRESS-LABEL", "ECUC-BOOLEAN-PARAM-DEF", "RESOLUTION", "GOVERNANCE-REF", "RVF-COLOR-SPACE", "SECURE-PROPS-FOR-EXTERNAL-COMM-REF", "TRANSPORT-PROTOCOL", "OBD-CONTROL-SERVICE-NEEDS", "CONTEXT-P-PORT-PROTOTYPE-REF", "MAIN-FUNCTION-PERIOD", "MAX-AS", "BINARY-MANIFEST-ITEM-POINTER-VALUE", "DLT-LOG-CHANNELS", "TP-PROTOCOL-TYPE", "COMMUNICATION-CONNECTOR-REFS", "REQUIRED-MODE-GROUP-REF", "META-DATA-ITEM", "AGGREGATION", "FIRST-ELEMENT-INSTANCE-IREFS", "MAX-DELTA-COUNTER-INIT", "NETWORK-HANDLE-REF", "MODE-INSTANCE", "PHYSICAL-LAYER-TYPE", "DO-IP-TP-CONNECTION", "BLACKLISTED-VERSIONS", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "TIME-LEAP-FUTURE-THRESHOLD", "INITIAL-DYNAMIC-PART", "RESISTANT-TO-CHANGED-SW", "LATENCY-BUDGET", "REQUEST-CONFIGURATION-DESIGNS", "MAXIMUM-RECEIVE-BUFFER-LENGTH", "SERVER-CALL-POINTS", "SUPERVISION-CONDITION", "PER-STATE-TIMEOUTS", "STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM", "START-IREF", "SECOND-KEY-SLOT-ID", "SOMEIP-REMOTE-UNICAST-CONFIG", "CONTEXT-SW-COMPONENT-REF", "TERMINATING-CHECKPOINT-REF", "SWC-TRIGGER-IREF", "STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE", "RPT-IMPL-POLICY", "MAX-MAJOR-VERSION", "REQUEST-RESPONSE-DELAY", "RETURN-SIGNAL-TRIGGERING-REF", "SUPPORT-INFO", "SYNCHRONIZATION-POINT-CONSTRAINT", "PNC-VECTOR-LENGTH", "INCLUDED-VARIANT-REFS", "TIME-SYNC-CLIENT", "SW-CALPRM-AXIS-SET", "POST-BUILD-VARIANT-MULTIPLICITY", "SECURITY-DELAY-TIME", "DEPENDENCY", "MAX-GRADIENT", "ELLIPTIC-CURVE-REFS", "ARTIFACT-CHECKSUM-REFS", "ECU-EXTRACT-VERSION", "USE-CLIENT-AUTHENTICATION-REQUEST", "CHECKSUM-VERIFICATION", "MIN-DEADLINE", "ENVIRONMENT-VARIABLES", "REMOTE-ID", "DO-IP-TP-CONFIG", "SELECTOR-FIELD-BYTE-ORDER", "I-PDU-IDENTIFIERS", "CONSIDER-BUILD-NUMBER", "SAFETY-MARGIN", "CONTEXT-COMPOSITION-REF", "VERSION-DRIVEN-FIND-BEHAVIOR", "V-2-X-FAC-USER-NEEDS", "TCP-IP-NDP-RTR-SOLICITATION-INTERVAL", "COM-PROCESSING-PERIOD", "SEC-OC-SECURE-COM-PROPS-REF-CONDITIONAL", "NETWORK-ENDPOINT-ADDRESSES", "SOFTWARE-PACKAGE", "SECURED-I-PDU", "VALUE-IREF", "NM-BUS-LOAD-REDUCTION-ENABLED", "REST-ARRAY-PROPERTY-DEF", "BLUEPRINT-POLICY-NOT-MODIFIABLE", "TIMING-PROPS", "RUN-MODE", "COMMUNICATION-CYCLE", "BASE-FREQUENCY", "LOGICAL-EXPRESSION-ARGUMENT-REFS", "TP-ADDRESS-REFS", "VARIABLE-REF", "CAN-CLUSTER", "OFFSET-CORRECTION-OUT", "SUB-FUNCTION-REF", "KEY-EXCHANGE-AUTHENTICATION-REFS", "J-1939-CLUSTER-CONDITIONAL", "MAX-SERVICE-POINT-ID", "USES-TIME-BASE-RESOURCE", "CHAPTER", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "BSW-MODULE-TIMING", "IDS-MGR-NEEDS", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "SHORT-NAME-FRAGMENT", "TLV-DATA-ID-DEFINITIONS", "ENDPOINT-CONFIGURATION-REF", "TD-EVENT-MODE-DECLARATION", "IS-EVENT", "DURABILITY-SERVICE-MAX-SAMPLES-PER-INSTANCE", "INITIAL-DELAY-MAX-VALUE", "MINIMUM", "MODE-DECLARATION-REF", "IS-EXTERNAL", "VARIABLE-LENGTH", "MAX-NUMBER-OF-EVENT-ENTRIES", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE", "PDUS", "OWNER-REF", "PHYSICAL-DIMENSION-MAPPINGS", "PHM-LOGICAL-EXPRESSION", "DIAGNOSTIC-FIM-ALIAS-EVENT", "EXPECTED-ALIVE-INDICATIONS", "DATA-TRANSFORMATION-REF-CONDITIONAL", "QOS-PROFILE", "MULTICAST-SD-IP-ADDRESS-REF", "ECUC-DEFINITION-REF", "VENDOR-SPECIFIC-SERVICE-NEEDS", "ETH-IP-PROPS", "BSW-MODULE-DESCRIPTION-REF-CONDITIONAL", "CRYPTO-KEY-SLOT-USAGE-DESIGN-REF", "CRYPTO-KEY-SLOT-USAGE-DESIGN", "CYCLE-STATUS-STORAGE", "CONTEXT-DATA-ELEMENT-IDENTIFICATIONS", "DIAG-ROUTINE-TYPE", "USED-SECONDARY-FID-REF", "SW-FEATURE-DEFS", "ACL-OBJECT-REF", "REST-SERVICE-INTERFACE", "COM-CONFIGURATION-RX-TIME-BASE", "ELEMENTS", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "APPLICATION-RECORD-DATA-TYPE", "MIN-PWM-O", "PROVIDER-IREF", "SEQUENCE-OFFSET", "CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF", "AUTHENTICATION-RETRIES", "SYMBOL", "TOLERATED-FAILED-CYCLES", "TIME-SEG-1", "DIAGNOSTIC-TEST-RESULT", "DDS-FIELD-REPLY-TOPIC-REF", "DIAGNOSTIC-CONTRIBUTION-REF", "INITIAL-EVENT-STATUS", "SWC-TO-ECU-MAPPING-CONSTRAINT", "ALL-PHYSICAL-CHANNELS-REF", "DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF", "DESTINATION-IPV-4-ADDRESS", "ALLOWED-TCP-OPTIONS-REF", "TCP-IP-DHCP-V-6-INF-DELAY-MIN", "DATA-CONSTR-RULE", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL", "HAS-GETTER", "DURABILITY-SERVICE-HISTORY-DEPTH", "CONSISTENCY-NEEDSS", "DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING", "TRANSMISSION-MODE-DECLARATION", "AUTOMATIC-END", "SOURCE-PORTS", "OFS-SUB-TLV", "CRYPTO-KEY-MANAGEMENT-NEEDS", "MODE-GROUP", "FRAME-MAPPINGS", "OBJECT-DEFINITION-REFS", "PROVIDED-MEMORY-REF", "LDAP-URL", "TRANSPORT-PROTOCOLS", "ALLOW-HALT-DUE-TO-CLOCK", "TRACED-FAILURES", "TIME-LEAP-HEALING-COUNTER", "DIAGNOSTIC-SOVD-BULK-DATA", "RESOURCE-DEFINITION-REF", "BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR", "DISCOVERY-TECHNOLOGY", "CAN-FD-FRAME-SUPPORT", "SUPPORTS-ASYNCHRONOUS-MODE-SWITCH", "VALUE-TYPE-TREF", "MAC-ADDRESS-STREAM-ID", "INITIAL-FIND-BEHAVIOR", "DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF", "LIN-SLAVE", "CLIENT-ID-DEFINITIONS", "NM-NID-POSITION", "DISABLED-MODE-IREFS", "CLEAR-EVENT-ALLOWED-BEHAVIOR", "BSW-SCHEDULER-NAME-PREFIX", "ALTERNATIVE-NAME", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "SECURITY-EVENT-CONTEXT-DATA", "USE-AS-CRYPTOGRAPHIC-I-PDU", "LOCAL-COMM-CONNECTOR-REF", "UPPER-LIMIT", "IEEE-1722-TP-AAF-CONNECTION", "HW-ELEMENT-REFS", "CRYPTO-CERTIFICATES", "TIME-SEG-2", "CONTEXT-R-PORT-PROTOTYPE-REF", "EXECUTABLE-LOGGING-IMPLEMENTATION-PROPS", "SCALE-CONSTR", "APMC-INTEGER-PARAM-DEF", "STATE", "ACTION-TYPE", "BACKGROUND-EVENT", "SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL", "HEADER-FILE", "CYCLIC-TIMING", "FIRST-TO-SECOND-DATA-TRANSFORMATION-REF", "TCP-MSL", "SUBFUNCTION", "TP-CONNECTION-CONTROLS", "ENTRY-REF", "ENTITY-STATUS-MAX-BYTE-FIELD-USE", "TARGET-OPERATION-REF", "REST-ENUMERATOR-DEF", "DYNAMIC-ACTION-REFS", "METHOD-REF", "MSR-QUERY-P-1", "CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING", "FM-FEATURE-RELATION", "RTPS-PROTECTION-KIND", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "SDU-REFS", "SECURED-AREA-OFFSET", "POST-BUILD-VARIANT-VALUE", "SEQUENCE-COUNTER", "TP-ADDRESSS", "TARGET-VARIABLE-DATA-PROTOTYPE-REF", "VALUE-CONFIG-CLASSES", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF", "R-PORT-PROTOTYPE-IREF", "DIAGNOSTIC-SERVICE-SW-MAPPING", "REPRESENTED-MODEL-ELEMENT-REF", "IMPLIED-OPERATION-REF", "I-SIGNAL-TYPE", "SW-SYSCOND", "RATE-CORRECTIONS-PER-MEASUREMENT-DURATION", "NEIGHBOR-CACHE-SIZE", "ARRAY-SIZE", "USER-DEFINED-GLOBAL-TIME-MASTER", "EVALUATION-ID", "PROVIDED-SERVICE-INSTANCE", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "CODE-DESCRIPTORS", "REQUIRES-SYMBOLIC-NAME-VALUE", "IS-VARIADIC-TEMPLATE", "SIZE-OF-UNION-LENGTH-FIELDS", "IMPLEMENTATION-CONFIG-VARIANT", "XCP-PDU", "MINIMUM-SEPARATION-TIME", "TARGET-MODULE-ID", "BLOCK-IF-STATE-ACTIVE-CP-REFS", "TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY", "DIAGNOSTIC-J-1939-SPN-MAPPING", "REST-ENDPOINT-DELETE", "SPECIFIC-CHANNELS", "ECU-INSTANCE-PROPS", "DISPLAY-NAME", "AUTOSAR", "BSW-SCHEDULABLE-ENTITY", "SOCKET-CONNECTION-REFS", "SECTION-NAME-PREFIX", "CHOICES", "LIFESPAN", "DIAGNOSTIC-REQUEST-DOWNLOAD", "AREA", "MEMORY-SECTION-LOCATIONS", "TCP-IP-ARP-TABLE-ENTRY-TIMEOUT", "RAM-BLOCK", "DESTINATION-MAC-ADDRESS-MASK", "TEST-ID", "SWITCH-STREAM-GATE-ENTRY", "ACL-OBJECT-REFS", "CONNECTION-BUNDLES", "SW-SYSTEMCONSTANT-VALUE-SET-REF", "LONG-NAME-1", "BLOCKLISTED-VERSIONS", "NAME-TOKEN-VALUE-VARIATION-POINT", "SIGNAL-BASED-METHOD-DEPLOYMENT", "HEALING-CYCLE-REF", "TCP-IP-IP-FRAGMENTATION-RX-ENABLED", "CAN-ENTER-EXCLUSIVE-AREA-REFS", "DIAGNOSTIC-STORAGE-CONDITION", "MAC-ADDRESS-VLAN-ASSIGNMENTS", "UDP-NM-ECU", "NM-REPEAT-MSG-INDICATION-ENABLED", "DATA-FORMAT-TAILORING", "APPLICATION-ARRAY-DATA-TYPE", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING", "BLUEPRINT-MAPS", "TCP-IP-NDP-DEFENSIVE-PROCESSING", "NOTIFIED-STATUSS", "MASTER-TO-SLAVE-EVENT-MAPPINGS", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING", "PDU-TRIGGERING", "DIAGNOSTIC-TROUBLE-CODE-OBD-PROPS", "OS-MODULE-INSTANTIATION", "IDSM-TIMESTAMP-PROVIDER-MAPPING", "NODE-IDENTIFIER", "SW-DATA-DEF-PROPS-VARIANTS", "EXPECTED-ALIVE-CYCLE", "TP-CHANNELS", "RECOVERY-VIA-APPLICATION-ACTION", "CONTAINER-I-PDU", "SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "REMOTING-TECHNOLOGY", "APPLICATION-DEFERRED-DATA-TYPE", "PDU-COUNTER-SIZE", "TAG-ID", "FIRE-AND-FORGET-METHOD-MAPPINGS", "WAKE-UP-PATTERN", "BSW-RESOLUTION", "CRYPTO-ALGORITHM-STRING", "LIN-ORDERED-CONFIGURABLE-FRAME", "ECUC-STRING-PARAM-DEF-CONDITIONAL", "PORT-BLUEPRINT-INIT-VALUE", "MULTICAST-CONNECTOR-REF", "UCM-MODULE-INSTANTIATION", "TIME-BUFFER", "COUNTER-JUMP-UP", "MAX-SOURCE-PORT-NUMBER", "NOTATION", "PERSISTENCY-DATA-ELEMENT", "MODE-DECLARATION-GROUP", "SIZE-OF-UNION-LENGTH-FIELD", "COUPLING-PORT-SWITCHOFF-DELAY", "WAKEUP-RX-IDLE", "SIGNAL-SERVICE-TRANSLATION-PROPS", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "OPERATIONS", "ETH-TP-CONFIG", "ACCESSED-VARIABLE-REF", "NM-VOTING-CYCLE", "CSE-CODE-FACTOR", "CALLED-ENTRY-REF", "PREFIX-REF", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REF", "READ-LOCAL-VARIABLES", "SUB-CONTAINERS", "FAILURE-MONITORING-REF", "RPT-HOOK-ACCESS", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "REQUIRED-MINOR-VERSION", "PATH-MTU-DISCOVERY-ENABLED", "MEMORY-LOW-ADDRESS-LABEL", "TESTED-ITEM-REFS", "UDP-NM-CLUSTER-COUPLING", "SOURCE-CAN-ID-REF", "SW-IMPL-POLICY", "EXECUTION-ERROR", "STATIC-MEMORYS", "CONTAINED-AR-ELEMENT-REFS", "NUMERICAL-VALUE-SPECIFICATION", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "TRACE-REFS", "MINIMUM-INTER-ARRIVAL-TIME", "PHM-SUPERVISED-ENTITY-INTERFACE", "APMC-VALUE-COLLECTION", "UDP-PROPS", "EXECUTION-CONTEXT-REF", "RESET-TO-DEFAULT-SUPPORTED", "PDU-POOLS", "TRANSPORT-PLUGINS", "ALIAS-EVENT-REF", "IIDC-T-CODE", "COMPOSITE-NETWORK-REPRESENTATION", "ECU-SCOPE-REF", "DIAG-ARG-INTEGRITY", "PAYLOAD-BYTE-PATTERN-RULES", "MSR-QUERY-TOPIC-1", "UDS-SERVICE", "ALIVE-SUPERVISION", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "STATE-DEP-FIREWALL-REF", "DIAGNOSTIC-SESSION-REF", "IIDC-TAG", "REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF", "GLOBAL-TIME-TX-PERIOD", "NM-ACTIVE-COORDINATOR", "PRE-ACTIVATE-REFS", "PERSISTENCY-KEY-VALUE-STORAGE", "LOCAL-PORT-RANGE-END", "FIREWALL-STATE-IREFS", "CAN-GLOBAL-TIME-DOMAIN-PROPS", "PARENT-SHORT-LABEL", "DLT-MESSAGES", "RX-TIMEOUT-TIME", "RX-ACCEPT-CONTAINED-I-PDU", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "NETWORK-TIME-CONSUMER-REF", "SPARSE-TIMESTAMP-ENABLED", "OBJECT-REFS", "MAX-PWM-S", "META-DATA-ITEM-TYPE", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "DATA-CONSTR", "DIAGNOSTIC-SESSION", "ERROR-SIGNALING-ENABLED", "LOWER-REF", "END-TO-END-PROTECTION", "ACF-MAX-TRANSIT-TIME", "NTU", "DPD-ACTION", "TABLE-CAPTION", "PNC-GROUP-REF", "LIN-SLAVE-CONFIG", "SHARED-AXIS-TYPE-REF", "MODE-CONDITION-REF", "DOCUMENT-ELEMENT-SCOPE", "ASSIGNED-TRAFFIC-CLASS", "FM-COND", "DO-IP-PROTOCOL-VERSION", "REF", "COMPARE-TYPE", "REFERENCE-REF", "OS-TASK-PROXY", "EXCLUSIVE-AREA-NESTING-ORDER-REFS", "PLCA-LOCAL-NODE-ID", "ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING", "MSR-QUERY-RESULT-P-1", "SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "NETWORK-CONFIGURATION-DESIGN-REF", "PACKAGER-ID", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "WRITE-ACCESS", "TCP-IP-NDP-SLAAC-DELAY-ENABLED", "TYPE-EMITTER", "ROOT-REF", "CM-MODULE-INSTANTIATION", "DDS-CP-TOPIC", "CRYPTO-SERVICE-JOB-NEEDS", "SERVICE-INSTANCE-ID", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "PROVIDED-ENTRYS", "END-TO-END-PROFILE", "DO-IP-TESTER-ROUTING-ACTIVATION-REFS", "EMPTY-SIGNAL", "SOURCE-ID", "NETWORK-CONFIGURATION-REF", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "CRYPTO-KEY-SLOT-CLIENT-INTERFACE", "TARGET-METHOD-REF", "COLLECTION-PROPS", "PORT-PROTOTYPE-SLUG-FRAGMENT", "SERIALIZATION-TECHNOLOGY", "LC-STATE-REF", "PNC-WAKEUP-CAN-ID", "NM-CONFIG", "THRESHOLD-SIZE", "CUSTOM-TRIGGER", "CRYPTO-TRUST-MASTER-INTERFACE", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "CONNECTION-PRIO-PDUS", "PHM-CHECKPOINT", "PATTERN-JITTER", "TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE", "J-1939-SHARED-ADDRESS-CLUSTERS", "DIAGNOSTIC-CONNECTION-REF-CONDITIONAL", "DDS-FIELD-DEPLOYMENT", "MODE-MAPPING", "RATE-LIMITATION-FILTERS", "ENTRY-MODE-DECLARATION-REF", "ACCEPT-ENCODING", "STREAM-IDENTIFICATION-WILDCARD", "SW-ADDRMETHOD-REF", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION", "LOCAL-SUPERVISION", "MAPPING-SCOPE", "MINIMUM-SUPPORTED-UCM-MASTER-VERSION", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER", "STREAM-IDENTIFIER", "CHECKPOINT-INITIAL-REF", "COM-EVENT-GRANT", "DEFAULT-APPLICATION-TIMEOUT", "SD-CLIENT-CONFIG-REF", "FM-ATTRIBUTE-VALUE", "ABSTRACT-CLASS-TAILORING", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS", "READ-CLASS-REF", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "APPROVAL-STATUS", "MAX-DID-TO-READ", "FRESHNESS-VALUE-TX-LENGTH", "FIELD-IREF", "OFFSET-CORRECTION-JUMP-THRESHOLD", "SUP", "RUNNABLES", "ETS-AVAILABLE-BANDWIDTH-IN-WEIGHT-VALUE", "MAXIMUM-INTENDED-BINDING-TIME", "TEAR-DOWN-ACTION-REF", "SW-COMPONENT-IREF", "MODE-SENDER-POLICYS", "DDS-EVENT-REF", "MAC-UNICAST-ADDRESS", "DESTINATION-TYPE", "PROVIDED-DDS-SERVICE-INSTANCE", "CONSTANT-MAPPING-REFS", "TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY", "DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT", "MIN-NUMBER-OF-ELEMENTS", "CHANNEL-REF", "UDP-TTL", "LIN-TP-CONFIG", "MODE-ELEMENTS", "DDS-SECURE-COM-PROPS", "REQUIRED-MODE-GROUPS", "DIAGNOSTIC-DATA-PORT-MAPPING", "DEF-LIST", "AUTOSAR-VARIABLE", "DYNAMIC-PART-ALTERNATIVES", "CRC-SECURED", "CALIBRATION-PARAMETER-VALUE", "DATA-TYPE-MAPPING-REFS", "ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT", "CP-SOFTWARE-CLUSTER-RESOURCE-REF", "DDS-PROVIDED-SERVICE-INSTANCE", "DATA-TYPE-MAPPING-SET-REF", "FIXED-LENGTH", "REST-RESOURCE-DEF", "FUNCTION-GROUP-SET", "READ-DTC-INFORMATION-CLASS-REF", "CAN-COMMUNICATION-CONNECTOR", "SOURCE-POSSIBLE-ERROR-REF", "CONTEXT-DATA-SOURCE", "BUILD-ACTION-MANIFEST-REF", "OPERATION-INVOKED-EVENT", "DIAGNOSTIC-SESSION-REFS", "TRANSFORMING-SYSTEM-SIGNAL-REF", "REQUEST-UPLOAD-CLASS-REF", "FEATURE-MODEL-REFS", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION", "ENGINEERING-OBJECT", "GID", "COMMITTED-INFORMATION-RATE", "EVENT-MULTICAST-UDP-PORT", "ETHERNET-NETWORK-CONFIGURATION", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "INSTALLATION-BEHAVIOR", "EXCLUSIVE-AREAS", "ACL-OPERATION-REFS", "OPERATION-CYCLE-REF", "IEE-E-1722-TP-CONNECTION-REF", "FUNCTION-NAME", "MAX-BS", "TRANSFORMER-REF", "RANGE-MAX", "REMAPPED-CAN-ID", "USER-DEFINED-I-PDU", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "COMM-CONNECTORS", "MOLAR-AMOUNT-EXP", "USED-PIM-REF", "NM-WAIT-BUS-SLEEP-TIME", "SW-SYSTEMCONSTANT-VALUE-SET", "SERVICE-INSTANCE-REF", "NM-CHANNEL-SLEEP-MASTER", "IMPLEMENTATION-DATA-TYPE", "IMPL-INIT-VALUE", "CAPABILITY-RECORDS", "SECOND-ELEMENT-INSTANCE-IREF", "BSW-INTERNAL-TRIGGERING-POINT-POLICY", "APMC-ENUMERATION-LITERAL-DEF", "APPLICATION-TYPE-TREF", "PID-SIZE", "SCALING-INFO-SIZE", "USED-SECONDARY-FID-REFS", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REF", "DIAGNOSTIC-ECU-RESET-INTERFACE", "SIGNAL-BASED-FIELD-REF", "DATA-UPDATE-PERIOD", "IDSM-TRAFFIC-LIMITATION-REF", "GLOBAL-IN-PACKAGE-REF", "TRIGGER-I-PDU-SEND-CONDITION", "COMPU-INVERSE-VALUE", "PREPARE-ROLLBACK", "DIAGNOSTIC-TROUBLE-CODE-UDS", "I-SIGNAL-REF", "RESULTING-PROPERTIES", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "IAM-MODULE-INSTANTIATION", "SOMEIP-TRANSFORMATION-DESCRIPTION", "CUSTOM-SERVICE-CLASS-REF", "BURST-PATTERN-EVENT-TRIGGERING", "FIRST-ELEMENT-REFS", "IP-ADDRESS-PREFIX-LENGTH", "USE-CASE", "FUNCTIONAL-CLUSTER-REFS", "SW-DATA-DEF-PROPS-CONDITIONAL", "VALIDATION-REF", "INTRODUCTION", "PRIMITIVE", "CAN-CONTROLLER-ATTRIBUTES", "FUNCTION-GROUP-STATE-IREFS", "COUPLING-FLAG", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "REF-CALPRM-SET", "UCM-PROCESSINGS", "ECU-COMM-PORT-INSTANCES", "EXECUTABLE-REFS", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "TIMEOUT-CHECK", "OPTION-KIND", "OBJECT-DEFINTION-REFS", "SET", "CONSUMED-EVENT-GROUPS", "COUPLED-CLUSTER-REFS", "WAIT-POINTS", "APPLICATION-PARTITION", "UNLIMITED-INTEGER-VALUE-VARIATION-POINT", "GENERAL-PURPOSE-TIMER-SERVICE-NEEDS", "PORT-ARG-VALUES", "AFFECTED-REFS", "AUTH-ROLE-REF", "VARIABLE-ACCESS-IREF", "PNC-CLUSTER-VECTOR-LENGTH", "TYPE-TAILORINGS", "SYSTEM-VERSION", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL", "DIAGNOSTIC-EVENT-REF-CONDITIONAL", "TIME-SYNC-SERVER-IDENTIFIER", "GLOBAL-TIME-PORT-ROLE", "INITIAL-DELAY-MIN-VALUE", "PORT-PROTOTYPE-BLUEPRINT", "DIAGNOSTIC-CONNECTED-INDICATOR", "PHYSICAL-CHANNEL-REF", "FM-FEATURE-DECOMPOSITION", "PARAMETER-ACCESS", "EVENT-ELEMENTS", "CRYPTO-SERVICE-PRIMITIVE", "MODES", "ENABLE-READ-ACCESS-CONTROL", "IDSM-PROPERTIES", "BY-PASS-POINT-IREF", "FREEZE-CURRENT-STATE-SUPPORTED", "UDP-NM-CLUSTER", "DIAGNOSTIC-AUTH-ROLE", "RAW-DATA-STREAM-DEPLOYMENT", "REQUIRED-INTERFACE-TREF", "INCLUDED-DATA-TYPE-SET", "SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING", "SW-RECORD-LAYOUT-GROUP-TO", "TRANSFORMATION-I-SIGNAL-PROPSS", "DIAGNOSTIC-ENABLE-CONDITION", "END-2-END-METHOD-PROTECTION-PROPS", "PRECONFIGURED-CONFIGURATION-REFS", "TERMINATION-BEHAVIOR", "LOAD-BALANCING-WEIGHT", "NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS", "DESCRIPTION", "SENDER-INTENT", "SCHEDULING-PRIORITY", "ROOT-DATA-PROTOTYPE-REF", "FACTOR-SI-TO-UNIT", "TRANSFORMATION-PROPS-MAPPING-SET-REF", "COM-METHOD-GRANT-DESIGN", "ECUC-DEFINITION-COLLECTION", "TIME-VALUE", "OS-TASK-PROXY-REF", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "ADDRESS-CLAIM-ENABLED", "IDSM-TRAFFIC-LIMITATION", "INTERNAL-TRIGGERING-POINT-POLICYS", "LIFE-CYCLE-INFO", "ACTIVATION-REASON-REPRESENTATION-REF", "CALL-TIMEOUT", "EXECUTABLE-PROVIDED-PORT-IREF", "INTERNAL-CONSTRS", "INITIAL-INACTIVITY-TIME", "CPP-TEMPLATE-ARGUMENT", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "DELTA-PACKAGE-APPLICABLE-VERSION", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS", "PREVIOUS-CONTRACT-VERSION", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "MODE-DECLARATION-MAPPING-SET-REF", "DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL", "IMPLIED-OPERATION-REFS", "STREAM-FILTER-PORT-RANGE", "PUBLISHER", "MODE-SWITCH-INTERFACE", "STATIC-PARTS", "BASE-TYPE-REF", "LICENSE-REF", "LENGTH-VERIFICATION", "SW-SYSTEMCONST", "LOCAL-PORT-REF", "CONTAINED-I-PDU-PROPS", "WAKEUP-REMOTE-ENABLED", "MONOTONY", "RECOVERY-NOTIFICATION-REF", "MSR-QUERY-CHAPTER", "MAY-BE-UNCONNECTED", "CORE-ID", "LITERALS", "CONTEXT-OPERATION-REF", "SWC-FLAT-SERVICE-DEPENDENCY-REF", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "CAN-IDENTIFIER-RANGE", "ASSOCIATED-COM-I-PDU-GROUP-REFS", "DLT-MESSAGE", "SPEEDUP", "SOURCE-SIGNAL-REF", "PROCESSOR-SPEED", "REFERRABLE-REF", "GENERIC-ETHERNET-FRAME", "E-2-E-PROFILE-CONFIGURATION", "MAX-SDU-SIZE", "LIN-SCHEDULE-TABLE", "ABSOLUTE", "NM-CHANNEL-ID", "CPP-IMPLEMENTATION-DATA-TYPE", "MSR-QUERY-RESULT-TOPIC-1", "ENABLE-CONDITIONS", "FAILED-SUPERVISION-CYCLES-TOLERANCE", "SOURCE-EEC-REF", "MAC-MULTICAST-GROUP-REF", "NATIVE-DECLARATION", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "SEGMENT-REF", "ACCESS-PERMISSION-REF", "NETWORK-MASK", "REGENERATED-PRIORITY", "DATA-WRITE-ACCESSS", "MINIMUM-MULTICAST-SEPERATION-TIME", "ECUC-IMPLEMENTATION-CONFIGURATION-CLASS", "DLT-CONTEXT", "SEARCH-BEHAVIOR", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS", "TCP-IP-AUTO-IP-INIT-TIMEOUT", "CRYPTO-PROVIDERS", "CLIENT-PORT-REF", "CHANGE", "ONE-EVERY-N", "IS-SERVICE", "CAN-CONTROLLER-XL-REQUIREMENTS", "TRANSFER-PROPERTY", "DIAGNOSTIC-IUMPR", "APMC-PARAM-CONF-CONTAINER-DEF", "TARGET-REQUIRED-OPERATION-REF", "SPN-REFS", "COMMUNICATION-CLUSTER-REF", "FREE-FORMAT", "UNIT-GROUP", "BUILD-ACTION-MANIFESTS", "DIAGNOSTIC-DATA-IDENTIFIER", "CONTEXT-LIMITATION-REFS", "COMPONENT-SEPARATION", "DATA-TYPE-MAPS", "IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER", "DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER", "TLS-JOB-MAPPINGS", "HOP-LIMIT", "LIN-IDENTIFIER", "TRANSFERS", "SW-INTENDED-RESOLUTION", "CAN-PHYSICAL-CHANNEL", "OPERATION-ARGUMENT-INSTANCE-IREF", "EVENT-ID", "SEGMENTATION-SUPPORTED", "PGN", "DATA-CONSTR-RULES", "ETHERNET-PRIORITY-REGENERATIONS", "DIRECT-SUCCESSOR-REF", "UCM-TO-TIME-BASE-RESOURCE-MAPPING", "LOCAL-TCP-PORT-REF", "LONG-NAME", "TLV-ARGUMENT-REF", "VCID", "FLEXRAY-NM-ECU", "COUPLING-PORT-SCHEDULER", "TCP-IP-IP-REASSEMBLY-TIMEOUT", "DIFFERENTIATED-SERVICE-FIELD", "MAXIMUM-RESIDENCE-TIME", "MKA-PARTICIPANT-REFS", "NUMBER-OF-STATIC-SLOTS", "USER-DEFINED-GLOBAL-TIME-SLAVE", "SW-AXIS-INDIVIDUAL", "TIME-BR", "DATA-FILTER-TYPE", "IE", "COUNTER-JUMP-DOWN", "FRAME-PIDS", "CLIENT-SERVER-OPERATION", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "DIAGNOSTIC-TRANSFER-EXIT", "TRIGGER-IREF", "IS-DELTA-PACKAGE", "GLOBAL-TIME-ETH-MASTER", "SECOND-PORT-REF", "CHECKPOINTS", "MEASURED-EXECUTION-TIME", "RPT-SERVICE-POINT-POST-REF", "TRIGGER-TO-SIGNAL-MAPPING", "COMPU-CONST", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "USER-DEFINED-CLUSTER-CONDITIONAL", "PROCESS-MODE-MACHINE", "MAX-SLOTS", "ALLOWED-I-PV-6-EXT-HEADERS-REF", "IEEE-1722-RAW-DATA-STREAM-PRODUCER-MAPPING", "CLAIMED-FUNCTION-GROUP-REFS", "SWITCH-STREAM-IDENTIFICATION", "IS-MANDATORY", "REQUESTABLE", "CONFIG-CLASS", "SW-MAX-TEXT-SIZE", "CONNECTION-TYPE", "CALC-RAM-BLOCK-CRC", "EXTERNAL-AUTHENTICATIONS", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "MAXIMUM-KEY-LENGTH", "MINISLOT-ACTION-POINT-OFFSET", "TIMING-CONDITION", "ALL-NM-MESSAGES-KEEP-AWAKE", "MICROTICK-DURATION", "MINIMUM-DELAY", "DO-IP-LOGIC-ADDRESSS", "CYCLE-AUTOSTART", "BLOCK-IF-STATE-ACTIVE-CP-REF", "RVF-ACTIVE-PIXELS", "TIMING-CONDITION-FORMULA", "CRYPTO-NEED-REF", "SYNC-FRAME-ID-COUNT-MAX", "FILTER-ACTION-BLOCK-SOURCE", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "MAX-NUMBER-OF-ELEMENTS", "MAXIMUM-SEGMENT-LENGTH", "STATE-REQUEST-PORT-IREF", "DIAGNOSTIC-DATA-TRANSFER", "MANDATORY", "DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL", "TP-SDU-REF", "APPLICATION-TYPE-REF", "RUNS-INSIDE-EXCLUSIVE-AREA-REFS", "BSW-TIMING-EVENT", "RUNNABLE-ENTITY-GROUP-IREFS", "HEALTH-CHANNEL-SUPERVISION", "MANAGED-MODE-GROUPS", "ALLOW-DYNAMIC-L-SDU-LENGTH", "VLAN-TRANSLATION-TABLES", "SDGS", "REVISION-LABELS", "ALLOWED-I-PV-6-EXT-HEADERS", "SOMEIP-RULE", "FRAME-PREEMPTION-SUPPORT", "NM-COORDINATOR", "INHIBITING-SECONDARY-FID-REF", "GLOBAL-TIME-CAN-SLAVE", "VARIABLE-ACCESS-REF", "PERIODIC-EVENT-TRIGGERING", "MULTICAST-UDP-PORT", "DESIGN-REFS", "ALLOCATE-SHADOW-COPY", "OBD-RATIO-SERVICE-NEEDS", "ENTITYS", "CONNECTED-INDICATORS", "TRIGGER-DIRECT-IMPLEMENTATIONS", "GETTER-RETURN-REF", "USER-DEFINED-CLUSTER-VARIANTS", "FORMULA", "TEXTUAL-CONDITION", "TLS-JOB-MAPPING", "MAPPED-BSW-SERVICE-DEPENDENCY-REF", "RESPONSES", "PHM-HEALTH-CHANNEL-STATUS", "WORST-CASE-HEAP-USAGE", "NV-BLOCK-DATA-MAPPINGS", "RPT-EXECUTABLE-ENTITYS", "CRYPTO-SERVICE-QUEUE", "MAX-NUMBER-OF-OCCURRENCES", "TEMPERATURE-EXP", "CRYPTO-SERVICE-CERTIFICATE-REF", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL", "HOST-ID", "SUB-NODE-CHANNEL-REF", "VLAN-PRIORITY", "SW-VALUE-BLOCK-SIZE", "APPLICATION-RECORD-ELEMENT-REF", "LOG-CHANNEL-ID", "NM-MULTIPLE-CHANNELS-ENABLED", "SDG-CONSTRAINT-REFS", "PRIMITIVE-TYPE-MAPPINGS", "MAC-MULTICAST-ADDRESS-REFS", "PROCESS-STATE-MACHINE", "CLIENT-SERVICE-MAJOR-VERSION", "DIAGNOSTIC-EVENT-REFS", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "POST-BUILD-VARIANT-USED", "CHANNEL-NAME", "EVENT-TOPIC-ACCESS-RULE-REF", "FLOAT", "R-PORT-IN-COMPOSITION-INSTANCE-REF", "REPOSITORY", "DATA-TYPE-FOR-SERIALIZATION-REFS", "SUB-FUNCTION-REFS", "ROUGH-ESTIMATE-HEAP-USAGE", "HARDWARE-TEST-NEEDS", "STACK-USAGES", "PORT-BLUEPRINT", "CLIENT-SERVER-TO-SIGNAL-MAPPING", "CONNECT", "DDS-QOS-PROFILES", "COUPLING-PORT", "SW-CLUSTER-REFS", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "IEEE-1722-TP-IIDC-CONNECTION", "IPV-4-PROPS", "DIAGNOSTIC-DO-IP-POWER-MODE-PORT-MAPPING", "PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE", "FILTER-ACTION-VLAN-MODIFICATION", "APMC-URI-INSTANCE-REFERENCE-DEF", "DATA-CONSISTENCY-POLICY", "MAX-SAMPLES-PER-INSTANCE", "J-1939-DCM-I-PDU", "ACTION-LIST-EXECUTION", "TRIGGER-I-PDU-SEND-CONDITIONS", "HEALTH-CHANNEL-ID", "MODE-SWITCHED-ACK-EVENT", "CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "MSR-QUERY-ARG", "SHORT-NAME-FRAGMENTS", "SERVICE-INTERFACE-REF", "PNC-SYNCHRONOUS-WAKEUP", "PARAMETER-INSTANCE", "FRAME-PID", "SW-VARIABLE-IMPLS", "PADDING-VALUE", "I-PDU-TRIGGERING-REF", "METHOD-ID", "METHOD-CALL-REFS", "PHM-RECOVERY-ACTION-INTERFACE", "SUB-GROUP-REF", "TIME-SYNC-SERVER", "BUFFER-COMPUTATION", "PERSISTENCY-FILE", "TCP-FAST-RETRANSMIT-ENABLED", "SWC-MODE-GROUP-IREF", "UNIQUE-STREAM-ID", "FIREWALL-RULE-PROPS", "REMOTE-PORT-RANGE-END", "FROM-REF", "FIRST-ELEMENT-INSTANCE-IREF", "PAYLOAD-LENGTH-STATIC", "ML-GRAPHIC", "SERVICE-INTERFACE-MAPPING", "REMOTE-IP-ADDRESS-REF", "CAN-NM-TX-CAN-ID", "CAN-CLUSTER-VARIANTS", "AUDIENCES", "MAC-MULTICAST-GROUPS", "DYNAMIC-LENGTH", "POLICY", "SIZE-OF-ARRAY-LENGTH-FIELD", "UCM-SOFTWARE-VERSION", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "RELATIVE", "LIN-TP-N-SDU-REF", "DETERMINISTIC-CLIENT-RESOURCE-NEEDSS", "SUB-ATTRIBUTE-TAILORINGS", "CONTAINED-PDU-TRIGGERING-REF", "CONTROL-PNC-REF", "RECEPTION-PROPS", "CONTAINED-PDU-TRIGGERING-REFS", "PROCESSORS", "FILE-REF", "NM-CYCLETIME-MAIN-FUNCTION", "PARENT-CATEGORY", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "TCP-INITIAL-INACTIVITY-TIME", "J-1939-CLUSTER-VARIANTS", "ASSIGNMENT-PRIORITY", "AGE", "ESP-CIPHER-SUITE-NAMES", "OPTIONAL-ELEMENTS", "TOOL", "SERVICE-INTERFACE-SUB-ELEMENT", "POSITION", "KEYWORDS", "BSW-MODE-RECEIVER-POLICY", "NETWORK-REPRESENTATION", "COMM-CONTROLLERS", "TARGET-TYPE", "USER-REFS", "MAXIMUM-SUPPORTED-UCM-MASTER-VERSION", "MASK", "DIAGNOSTIC-PARAMETER-REF", "PLATFORM-TIME-BASES", "LOCAL-IP-SEC-RULE-REFS", "TROUBLE-CODE-PROPSS", "CONSTANT-MEMORYS", "APPLIES-TO-DEPENDENCIES", "DNS-SERVER-ADDRESSES", "TLS-JOB-REQUIREMENT", "DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER", "DPG-REQUIRES-COHERENCYS", "TIME-HARDWARE-CORRECTION-THRESHOLD", "MAX-AR", "DEBOUNCE-BEHAVIOR", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "PARTIAL-NETWORK-REFS", "ASSEMBLY-SW-CONNECTOR", "SYMBOL-PROPS", "MC-PARAMETER-INSTANCES", "INITIAL-NAD", "MAX-ALIVE-CYCLE", "ECUC-STRING-PARAM-DEF", "PHYSICAL-CHANNEL-REFS", "CLIENT-ID-DEFINITION-SET", "GLOBAL-SUPERVISIONS", "HW-COMMUNICATION-CONTROLLER-REF", "TCP-IP-NDP-DEFAULT-REACHABLE-TIME", "POSSIBLE-AP-ERROR-SET-REF", "SO-AD-ROUTING-GROUP", "RAM-LOCATION-REF", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "SENDER-CAPABILITY", "XREF-TARGET", "UNIT-GROUP-REF", "LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS", "TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "MINOR-PROTOCOL-VERSION", "MODE-REQUEST-TYPE-MAPS", "TARGET-DATA-PROTOTYPE-IN-CS-REF", "STATE-MANAGEMENT-COMPARE-FORMULA", "APPLICABILITY-INFOS", "ACCESS-COUNT-SET", "SWC-MODE-SWITCH-EVENT-REF", "RPT-SYSTEM-REF", "DIAGNOSTIC-TEST-RESULT-REFS", "SOCKET-PROTOCOL", "TARGET-P-PORT-REF", "TIMING-VARIABLES", "TARGET-PHM-SUPERVISED-ENTITY-IREF", "SA-RAND-TIME", "END-ADDRESS", "SYNC-PERIOD", "FIRST-MODE-REFS", "NODE-NAME", "MAPPED-FUNCTION-REF", "VERIFICATION-STATUS-INDICATION-MODE", "DO-IP-REQUEST-CONFIGURATION-DESIGN", "IKE-RAND-TIME", "FUNCTIONAL-CLUSTER-REF", "LOG-TRACE-PROCESS-ID", "MEMORY-DESTINATION-REFS", "PARAMETER-SW-COMPONENT-TYPE", "CAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "CONSUMED-SERVICE-INSTANCE", "TP-SDU-REFS", "MODE-DEPENDENT-STARTUP-CONFIG", "N-PDU", "EXECUTION-TIME-CONSTRAINT", "DROP-NOT-REQUESTED-NAD", "LIN-MASTER-VARIANTS", "MODE-DECLARATION-GROUP-REFS", "ACF-BUS-REF", "EXCESS-BURST-SIZE", "PAYLOAD-TYPE", "HTTP-ACCEPT-ENCODING", "TLV-DATA-ID-DEFINITION-SET", "NUM-FAILED-SECURITY-ACCESS", "ACCEPTED-REMOTE-CERTIFICATE-REFS", "ELEMENT", "SENDER-REC-ARRAY-ELEMENT-MAPPING", "MAXIMUM-DYNAMIC-PAYLOAD-LENGTH", "SW-COMP-TO-ECU-MAPPING-REF", "DEPENDENCY-ON-ARTIFACT", "CRYPTO-ALG-ID", "INITIAL-CHECKPOINT-REFS", "IT-REF", "DDS-EVENT-QOS-PROFILE-REF", "DEBOUNCE-ALGORITHM-REF", "SOURCE-TRIGGER-REF", "CONTAINED-AR-ELEMENT-REF", "TIME-EXP", "CERT-COMMON-NAME", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "ERROR-CONTEXT-REF", "SECURITY-EVENT-REF", "JOB-REQUIREMENTS", "PHM-CONTRIBUTION-REF", "METHODS", "NESTED-CONTEXT-DATAS", "SLAVE-REF", "WRITE-ONLY-ONCE", "SOMEIP-FIELD-DEPLOYMENT", "TIMEOUT-AR", "PID", "STORING", "MANAGED-PHYSICAL-CHANNEL-REF", "UNICAST-SECURE-COM-PROPS-REFS", "PROPABILITY", "START-POSITION", "MAX-DATA-LENGTH", "REVERSED-TP-SDU-REF", "PERSISTENCY-DATA-PROVIDED-COM-SPEC", "AAF-FORMAT", "ECUC-DESTINATION-URI-DEF-SET", "APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF", "SAMPLE-CLOCK-PERIOD", "SUPERVISION-MODES", "DIAGNOSTIC-PERIODIC-RATE", "VEHICLE-ANNOUNCEMENT-INTERVAL", "SEC-OC-COM-PROPS-FOR-MULTICAST-REFS", "TARGET-MODE-REF", "ORIGIN", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "KEY-EXCHANGE-REF", "EVENT-QOS-PROPSS", "RUNTIME-IP-ADDRESS-CONFIGURATION", "SENDER-REC-ARRAY-TYPE-MAPPING", "CRYPTO-MODULE-INSTANTIATION", "UDP-COLLECTION-BUFFER-TIMEOUT", "CALIBRATION-PARAMETER-VALUES", "SW-COMPONENT-DOCUMENTATIONS", "DOCUMENTATION-CONTENT", "SDG", "BSW-MODE-SWITCHED-ACK-EVENT", "ARBITRARY-ADDRESS-CAPABLE", "J-1939-CONTROLLER-APPLICATION", "ACCEPTED-REMOTE-CERTIFICATE-REF", "SOURCE-IPV-4-ADDRESS", "REST-ENDPOINT-ARGUMENT", "ENABLE-ANYCAST", "DESTINATION-BASE-ID", "VERIFY-IREFS", "STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS", "SETTER-REF", "DATA-MAPPINGS", "RAW-DATA-STREAM-SERVER-INTERFACE", "SD-CLIENT-CONFIG", "SD-SERVER-TIMER-CONFIGS", "AUTOSAR-DATA-TYPE-REF-CONDITIONAL", "SECOND-TRIGGER-REF", "DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT", "ATTRIBUTE-VALUES", "SYNC-LOSS-THRESHOLD", "RUNNABLE-ENTITY-GROUP", "TD-EVENT-SERVICE-INSTANCE-FIELD", "TIMESTAMP-FORMAT", "DO-IP-REQUEST-CONFIGURATION", "AGING-REF", "ACCEPTED-STARTUP-RANGE", "USER-DEFINED-TRANSFORMATION-PROPS", "TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC", "COMPU-RATIONAL-COEFFS", "DDS-EVENT-TOPIC-REF", "SW-COMPONENT-PROTOTYPE-IREF", "EVENTS", "BY-PASS-POINT-IREFS", "ASSOCIATED-PDUR-I-PDU-GROUP-REF", "SOURCE-CAN-ID-CODE", "HW-TYPE-REF", "CONTEXT-COMPONENT-REFS", "SAVE-CONFIGURATION", "RPT-EXECUTABLE-ENTITY-PROPERTIES", "MODE-TRANSITIONS", "MAX-SIZE-TO-FILL", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "NM-HANDLE-REF", "GLOBAL-IN-PACKAGE-REFS", "MAX-TESTER-CONNECTIONS", "DELAY", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "ASSIGNED-LIN-SLAVE-CONFIG-REF", "AFFECTION-KIND", "COLLECTED-INSTANCE-IREFS", "SYNCHRONIZED-TIME-BASE-CONSUMER", "PORT-CONFIGS", "DRIVER-PRIMITIVE-REF", "PROG-CODE", "ECU-INSTANCE-REFS", "IDENTIFIABLE-REF", "MEMORY-DESTINATION-REF", "DDS-FIELD-REQUEST-TOPIC-REF", "REST-ELEMENT-DEF", "INVALID-VALUE", "DIAGNOSTIC-CONTRIBUTION-SET", "HW-COMMUNICATION-PORT-REF", "SERVICE-CONTROL", "PERMIT-MULTIPLE-REFERENCES-TO-EE", "SERVICE-INTERFACE-EVENT-MAPPING", "PATTERN-PERIOD", "SPEED", "POSSIBLE-ERROR-REACTION", "MODE-INTERFACE-MAPPING", "SOFTWARE-PACKAGE-STEP", "DIAGNOSTIC-J-1939-NODE", "SW-MAINTENANCE-NOTES", "PSK-IDENTITY", "LOG-AND-TRACE-INSTANTIATION", "LIN-UNCONDITIONAL-FRAME-REFS", "AUTHENTIC-CONNECTION-PROPSS", "AUTOSAR-PARAMETER-IREF", "PDU-TO-FRAME-MAPPING", "RX-NM-PDU-REFS", "CONSTR-LEVEL", "REQUESTOR-REFS", "FLOW-METERING-REF", "TCP-UDP-CONFIG", "TRACE", "DATA-TYPE-MAPPING-SET-REFS", "ECUC-QUERYS", "HISTORY-KIND", "IPV-4-DHCP-SERVER-CONFIGURATION", "ALLOW-PROVIDER-RATE-CORRECTION", "VARIATION", "PRIMITIVE-ATTRIBUTE-CONDITION", "SECURITY-EVENT-REPORT-INSTANCE-VALUE", "ECUC-MODULE-CONFIGURATION-VALUES-REF", "LIN-ERROR-RESPONSE", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "COMPONENT-IREF", "CIPHER-SUITE", "NM-DATA-CYCLE", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS", "DIAGNOSTIC-CONNECTIONS", "TRIGGER-INTERFACE-MAPPING", "LOCAL-UNICAST-ADDRESSS", "EXTENDED-ADDRESSING", "SW-CALIBRATION-NOTESS", "NETWORK-INTERFACE-REF", "DIAGNOSTIC-UPLOAD-INTERFACE", "PROCESS-EXECUTION-ERROR", "FLEXRAY-AR-TP-CONNECTION", "TARGET-FIELD-REF", "ARTIFACT-LOCATOR", "BSW-ENTRY-RELATIONSHIP-SET", "PORT-PROTOTYPE-REF", "DATA-TRANSFORMATIONS", "SUB-NODE-NUMBER", "RVF-INTERLACED", "DO-IP-TESTER-ROUTING-ACTIVATION-REF", "NETWORK-CONFIGURATIONS", "MC-DATA-INSTANCE-VAR-REF", "NM-IMMEDIATE-RESTART-ENABLED", "UCM-MASTER-FALLBACK-REFS", "SAVE-CONFIGURATION-ENTRY", "SECURITY-EVENT-MAPPING", "VARIABLE-ACCESSED-FOR-DEBUG-REF", "TCP-IP-IP-NUM-REASS-DGRAMS", "MAC-SEC-PROPS", "TD-EVENT-TYPE", "TP-CONNECTIONS", "MODE-SWITCH-SENDER-COM-SPEC", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "SEND-INDICATION", "TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS", "STATE-REQUEST-REF", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "SOMEIP-PROVIDED-EVENT-GROUP", "I-SIGNAL-TRIGGERING", "SEPARATION-TIME", "COM-MANAGEMENT-GROUP-REF", "SEGMENT-POSITIONS", "I-SIGNAL-TO-I-PDU-MAPPINGS", "RPT-WRITE-ACCESS", "DIAGNOSTIC-ECU-RESET-CLASS", "IPV-4-CONFIGURATION", "ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "VEHICLE-ROLLOUT-STEP", "SERVER-PORT-REF", "SOFTWARE-MEMORY-SECTION-REF", "JUMP-TO-BOOT-LOADER", "REMOTE-CERTIFICATE-REFS", "ENTERED-MODE-REF", "DESTINATION-URI-POLICY", "SRVT", "ATTRIBUTE-REF", "DELIVERY-ARTIFACTS", "CONTAINED-PROCESS-REFS", "METHOD-REPLY-TOPIC-NAME", "V-2-X-SUPPORTED", "SPECIFICATION-SCOPE", "CYCLE-TIME-VALUE", "ECUC-FLOAT-PARAM-DEF", "BUILD-ACTION-MANIFEST-REF-CONDITIONAL", "METADATA-PROTECTION-KIND", "KEY-VALUE-PAIRS", "SECOND-ELEMENTS", "EXECUTION-TIME-TYPE", "PIN-NUMBER", "SECURITY-EVENT-REPORT-INSTANCE-DEFINITION", "SW-NUMBER-OF-AXIS-POINTS", "ALIVE-REFERENCE-CYCLE", "PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "MEMORY-ID", "SLEEP-TIME", "DEPENDS-ONS", "DIAGNOSTIC-COMMON-ELEMENT-REF", "NM-SHUTDOWN-DELAY-TIMER", "NETWORK-SEGMENT-ID", "MULTIPLEXED-I-PDU", "SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING", "SEPARATION-TIME-RESPONSE", "SW-RECORD-LAYOUT-V-PROP", "COMMUNICATION-CONNECTOR-REF-CONDITIONAL", "QUEUE-LENGTH", "GLOBAL-RESOURCE-ID", "SERVICE-BUSY", "DIAGNOSTIC-ACCESS-PERMISSION", "USER-DEFINED-TRANSFORMATION-DESCRIPTION", "CONSUMED-EVENT-GROUP-REF", "DERIVED-CERTIFICATE-ACCEPTED", "PREEMPTABILITY", "SERVICE-DISCOVER-CONFIGS", "V-2-X-DATA-MANAGER-NEEDS", "MULTICAST-CREDENTIALS", "PRODUCT-RELEASE", "TRIGGER-REF", "RECEIVE-ACTIVITY", "TERMINATING-CHECKPOINT-TIMEOUT-UNTIL-TERMINATION", "PERMISSIBLE-SIGNAL-PATH", "SOMEIP-EVENT-DEPLOYMENT", "EXECUTABLE-GROUP", "TARGET-DATA-PROTOYPE-REF", "INTENDED-FILENAME", "MODE-DECLARATION-REFS", "MAX-RETRIES", "COUPLING-PORT-ENHANCED-TRAFFIC-SHAPER", "SHARED", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "AUTHENTICATION-CLASS-REF", "ECUC-VALUE-COLLECTION", "SECURITY-EVENT-REPORT-INTERFACE", "IDENTITIY-NUMBER", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE", "STATE-DEPENDENT-FIREWALL-REF", "DIAGNOSTIC-EXTRACT-REF", "CONTEXT-COMPONENT-REF", "ACCESS-COUNTS", "DEADLINE-MIN", "WAKEUP-RX-LOW", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING", "BSW-DATA-RECEIVED-EVENT", "RPT-CONTAINERS", "BUILD-ACTION-ENVIRONMENT", "NETWORK-TIME-SLAVE-REF", "CONTROL-PROVIDED-EVENT-GROUP-REF", "APPLICATION-ENDPOINT", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-PORT-MAPPING", "WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST", "ETHERNET-CLUSTER-VARIANTS", "PERSISTENCY-FILE-STORAGE", "REMOTE-ADDRESSS", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "TRANSMITTER-REF", "SOFTWARE-CLUSTER-REF", "ROUGH-ESTIMATE-STACK-USAGE", "EVENT-NEEDS-REF", "UDP-REMOTE-PORT", "CUSTOM-SPECIFICATION-REF", "DLT-ECU", "LIST", "INTERFACE-MAPPINGS", "REMOTE-MULTICAST-CONFIG-REF", "ECUC-INSTANCE-REFERENCE-DEF", "KEY-STORAGE-TYPE", "REPORT-BEHAVIOR", "USED-IMPOSITION-TIME-DEFINITION-GROUP-REF", "CP-SOFTWARE-CLUSTER-REF-CONDITIONAL", "AUTOSAR-VARIABLE-IN-IMPL-DATATYPE", "DIAGNOSTIC-SERVICE-INSTANCE-REF", "REGULAR-EXPRESSION", "INTEGER-VALUE-VARIATION-POINT", "DLT-LOG-SINK-REF", "SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "PWM-O", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "SDG-DEF", "NETWORK-INTERFACES", "SERVICE-DEPLOYMENT-REF", "PORT-GROUP", "VARIATION-POINT-PROXYS", "TIMING-CONDITION-REF", "HEALTH-CHANNEL-ARGUMENT-REF", "MIRRORING-PROTOCOL", "GENERATOR", "TRAFFIC-SHAPER-GROUPS", "OBD-PROPSS", "COMPU-SCALE", "FOLLOW-UP-TIMEOUT-VALUE", "COMMON-SIGNAL-PATH", "IUMPR-REF", "NOTIFIER-REF", "NM-CAR-WAKE-UP-RX-ENABLED", "BSW-DIRECT-CALL-POINT", "CHECKPOINT-FINAL-REF", "BITFIELD-TEXT-TABLE-MASK-SECOND", "CAN-FRAME-TRIGGERING", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "TARGET-DATA-PROTOTYPE-IN-SR-REF", "CLIENT-SERVER-INTERFACE", "DDS-CP-QOS-PROFILE", "WAKEUP-TX-ACTIVE", "DEPLOYMENT-REF", "DISABLED-IN-MODE-IREF", "SERVER-SERVICE-MAJOR-VERSION", "SIGNAL-AGE", "ASSUMED-CURRENT-STATE-IREF", "DIAGNOSTIC-AUTHENTICATION-CLASS", "STREAM-GATES", "GRANT-REF", "EXEC-IN-USER-MODE", "SEND-RESP-PEND-ON-TRANS-TO-BOOT", "RPT-EXECUTABLE-ENTITY-EVENTS", "SWC-TO-ECU-MAPPING", "FLEXRAY-FRAME-TRIGGERING", "STANDARD", "SW-AXIS-CONT", "L-10", "PORT-INTERFACE-MAPPINGS", "PAYLOAD-REF", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "EVALUATED-VARIANT-REFS", "BYTE-ORDER", "LIN-PHYSICAL-CHANNEL", "TARGET-CHANNEL", "SDG-REFERENCE", "REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS", "BULK-DATA-CATEGORY", "CONTAINER-TIMEOUT", "GENERAL-PURPOSE-PDU", "APPLICATION-ENDPOINT-REF-CONDITIONAL", "SDX-REF", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING", "TLV-SUB-ELEMENT-REF", "PROVIEDE-DATA-REF", "N", "BSW-RESOURCE-ESTIMATION", "SERIALIZER-REF", "ELEMENT-BYTE-VALUE", "RX-IDENTIFIER-RANGE", "LICENSE-REFS", "SOURCE-METHOD-REF", "TRANSMIT-CANCELLATION", "APPLICATION-PRIMITIVE-DATA-TYPE", "MINOR-VERSION", "SECURITY-EVENT-CONTEXT-DATA-DEFINITION", "DLT-ECU-ID", "AR-PACKAGE", "TYP", "SLAVE-EVENT-REF", "SW-CALPRM-PROTOTYPE-REF", "REPRESENTED-PORT-GROUP-REF", "DATA-ID", "PHM-HEALTH-CHANNEL-INTERFACE", "REENTRANCY-LEVEL", "META-DATA-LENGTH", "RESET-PENDING-BIT-ON-OVERFLOW", "CAN-CLUSTER-CONDITIONAL", "ROOT-CONTEXT-REF", "TAG-WITH-OPTIONAL-VALUE", "STATE-DEP-FIREWALL-REFS", "DESTINATION-IP-ADDRESS", "DENOMINATOR-CONDITION", "REVISION-LABEL-P-2", "MC-DATA-INSTANCE", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "COUPLING-PORT-STRUCTURAL-ELEMENTS", "TRANCEIVER-STANDBY-DELAY", "PNC-WAKEUP-DLC", "TLS-CIPHER-SUITE", "ROOT-PARAMETER-DATA-PROTOTYPE-REF", "STATE-MANAGEMENT-REQUEST-ERROR", "OFFSET-SI-TO-UNIT", "NM-COORDINATOR-ROLE", "CHANNELS", "SECURE-COMMUNICATION-DEPLOYMENTS", "COM-OFFER-SERVICE-GRANT-DESIGN", "SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING", "NETWORK-HANDLES", "GENERAL-PURPOSE-I-PDU", "PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS", "J-1939-CLUSTER", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "CAN-CONTROLLER-FD-REQUIREMENTS", "APMC-FUNCTIONAL-CLUSTER-DEF", "INIT-RESOURCE", "APPL-INIT-VALUE", "GATEWAY", "APMC-FUNCTIONAL-CLUSTER-VALUE", "NM-DETECTION-LOCK", "COMM-CONTROLLER-MAPPINGS", "DATA-PROTOTYPE-IREF", "APMC-URI-INSTANCE-REFERENCE-VALUE", "IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "NM-CAR-WAKE-UP-FILTER-NODE-ID", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF", "ALIAS-NAME-ASSIGNMENT", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "WINDOW-SIZE-INIT", "LISTEN-NOISE", "MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS", "IMPLICIT-DATA-ACCESS-IREF", "LIN-CLUSTER-CONDITIONAL", "CRYPTO-JOBS", "SLOT-ID", "SOFTWARE-CLUSTER-DESIGN", "MODE-DECLARATION-GROUP-PROTOTYPE-IREF", "EXTERNAL-CLOCK-SYNCHRONISATION", "MEM-USAGE", "ECUC-NUMERICAL-PARAM-VALUE", "FIREWALL-STATE-MODE-DECLARATION-REFS", "CREATE-ECU-WAKEUP-SOURCE", "SD-SERVER-EG-TIMING-CONFIGS", "SOMEIP-FIELD", "LIVELINESS", "TCP-NAGLE-ENABLED", "OBD-SUPPORT", "MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF", "DEFAULT-PERIOD-END", "TCP-IP-IP-REASSEMBLY-BUFFER-SIZE", "MAX-HEADER-ID", "SOMEIP-EVENT-GROUP", "MAC-SEC-KAY-CONFIG", "OPERATION-MAPPINGS", "MINIMUM-SELECTED-BINDING-TIME", "IP-SEC-IAM-REMOTE-SUBJECT", "SOMEIP-TRANSFORMATION-PROPS", "SESSION-ID", "TTL", "DATA-PROTOTYPE-GROUP-IREFS", "FLOW-LABEL", "ECUC-FUNCTION-NAME-DEF", "SOURCE-CHANNEL", "GID-INVALIDITY-PATTERN", "I-SIGNAL-I-PDU-GROUP", "CAN-ID-TO-CAN-ID-MAPPINGS", "BSW-INTERNAL-TRIGGERING-POINT", "LATENCY-CONSTRAINT-TYPE", "MODE-IN-BSW-INSTANCE-REF", "INVERT", "CALIBRATION-PARAMETER-VALUE-SET-REFS", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "DIRECT-SUCCESSOR-REFS", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "MAX-BASE-TYPE-SIZE", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF", "TD-EVENT-BSW-MODULE", "PER-INSTANCE-MEMORYS", "PDU-TRIGGERING-REF", "TCP-IP-PROPS-REFS", "REQUIRED-TRIGGERS", "PROCESSOR-MODE", "REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF", "ARRAY-IMPL-POLICY", "I-PDU-IDENTIFIER-UDP-REF", "NM-READY-SLEEP-TIME", "SECOND-TO-FIRST-DATA-TRANSFORMATION-REF", "TARGET-RUNNABLE-ENTITY-REF", "PERSISTENCY-ACCESS", "NM-VARIANT", "FRAME-PORT-REF", "MACROTICK-DURATION", "UPPER-BOUNDARY", "RESOURCE-NEEDS-REFS", "CAN-BE-INVOKED-CONCURRENTLY", "PER-INSTANCE-MEMORY-REF", "READONLY", "TRANSLATED-VLAN-ID", "SECURITY-ACCESS-LEVEL", "DIAGNOSTIC-INDICATOR-NEEDS", "REQUIRED-SERVICE-PORT-IREF", "IPV-6-ADDRESS", "TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "CAN-XL-FRAME-TRIGGERING-PROPS", "PHYSICAL-DIMENSION-REF", "DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT", "NO-CHECKPOINT-SUPERVISION", "I-SIGNAL-I-PDU", "INSTANCE-ID-PROPS", "PROVIDED-DDS-SERVICE-INSTANCE-EVENTS", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "TARGET-APPLICATION-RECORD-ELEMENT-REF", "BSW-DISTINGUISHED-PARTITION", "DDS-TOPIC-ACCESS-RULE", "MULTICAST-REF", "IMPORTANCE", "CLIENT-SERVER-OPERATION-IREF", "POST-VERIFICATION-REBOOT", "MIN-MINOR-VERSION", "CERTIFICATE-REF", "SOCKET-ADDRESSS", "STORAGE-CONDITIONS", "SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPINGS", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "SW-BASE-TYPE", "MODE-SWITCHED-ACK", "TRANSFORMATION-TECHNOLOGY", "SEGMENT-RECEPTION-TIMEOUT-TIME-RESPONSE", "GENERIC-TP-CONNECTION", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF", "R-PORT-PROTOTYPE-PROPS", "DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING", "SOFTWARE-ACTIVATION-DEPENDENCY-REF", "ADDRESS-SEMANTICS", "MODULE-REF", "OFFSET", "DEVELOPMENT-ERROR", "ACTION-ITEM-REF", "REPORTABLE-SECURITY-EVENT-REFS", "AR-PACKAGES", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "CAN-ENTER-EXCLUSIVE-AREA-REF", "I-SIGNAL-PORT", "FUNCTIONAL-CLUSTER-DESIGNS", "SOMEIP-TP-CHANNEL", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "ACCESS-PERMISSION-VALIDITY", "DERIVED-FROM-BLUEPRINT-REF", "OBD-DTC-VALUE-3-BYTE", "LIN-CHECKSUM", "IKE-CIPHER-SUITE-NAME", "MODE-IREF", "HW-PIN", "CLIENT-POLICYS", "AUTHENTICATION-VERIFY-ATTEMPTS", "TIME-MASTER-PRIORITY", "IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL", "LOGIC-ADDRESS", "MAC-SEC-PARTICIPANT-SET", "FIRE-AND-FORGET-MAPPINGS", "USE-VARIABLE-SA", "ECU-PARTITION-REF", "CAN-XL-CONFIG-REQS", "ICMP-V-6-PROPS", "ECUC-REF", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "MASTER", "NOTIFICATION-TIME", "SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL", "TRANSPORT-PRIORITY", "SERVICE-ELEMENT-MAPPING-REF", "RECOVERY-ACTION-IREF", "PREVIOUS-DATA-TYPE-REF", "USED-LANGUAGES", "VERIFICATION-HASH", "INNER-PORT-IREF", "AES-3-DATA-TYPE-H", "GLOBAL-TIME-CAN-MASTER", "ECU-STATE-MGR-USER-NEEDS", "REQUEST-DOWNLOAD-CLASS-REF", "ECUC-QUERY-REF", "ESTIMATED-DURATION-OF-OPERATION", "VEHICLE-IDENTIFICATION-SYNC-STATUS", "ERROR-CONTEXT-REFS", "MC-FUNCTION-REFS", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS", "CRC-MESSAGE-LENGTH", "SOURCE-MAC-ADDRESS", "DO-IP-LOGICAL-ADDRESS-REF", "SUBSCRIBE-EVENTGROUP-RETRY-DELAY", "WRITING-STRATEGYS", "MC-SUPPORT", "VALID-BINDING-TIME", "ECU-MAPPING", "EID", "CHECK-STATIC-BLOCK-ID", "WAKEUP-LOCAL-ENABLED", "ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS", "TCP-KEEP-ALIVE-INTERVAL", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF", "DATA-ID-MODE", "TIME-SYNCHRONIZATION", "MACHINE-DESIGN", "DIAGNOSTIC-ENV-CONDITION-FORMULA", "LOCAL-SUPERVISION-REF", "HEADER-TYPE", "REQUIRED-PACKAGE-ELEMENT-REF", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING", "DOCUMENTATION-CONTEXT", "DO-IP-RULE", "TLV-DATA-ID-0-REF", "TCP-SYN-MAX-RTX", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "TOPIC-DATA", "TRACE-SWITCH-CONFIGURATIONS", "INITIAL-OFFER-BEHAVIOR", "E-2-E-PROFILE-COMPATIBILITY-PROPS-REF", "CONTROLLED-HW-ELEMENT-REF", "RPT-AR-HOOK-IREF", "TARGET-SWC-SERVICE-DEPENDENCY-REF", "RETURN-TYPE", "ACTIVATE-AT-START", "DATA-PROTOTYPE-REF", "MODE-REQUEST-TYPE-MAP", "CODE-LABEL", "TCP-KEEP-ALIVE-TIME", "ACTION-ITEMS", "DPD-DELAY", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "COMPOSITE-TYPE-MAPPINGS", "API-PRINCIPLE", "REFERENCES", "START-ACTION-REF", "DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE", "TARGET-STATUS-REF", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "PARAMETER-INTERFACE", "PERSISTENCY-DEPLOYMENT-URI", "EXPECTED-CALLBACKS", "TRANSFORMATION-TECHNOLOGYS", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF", "SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION", "SOURCE-EEC-REFS", "REMOTE-UNICAST-CONFIG-REF", "RUNNABLE-REF", "BLUEPRINT-DERIVATION-GUIDE", "TD-EVENT-SERVICE-INSTANCE-METHOD", "CLASS-CONTENTS", "APMC-FOREIGN-REFERENCE-VALUE", "IPV-6-RULE", "DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE", "TRIGGER-REFS", "SW-RECORD-LAYOUT-GROUP-FROM", "TCP-RETRANSMISSION-TIMEOUT", "NAMED-CURVE-ID", "PNC-WAKEUP-CAN-ID-MASK", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "LIN-CONFIGURABLE-FRAMES", "CALLED-ENTRYS", "SOFTWARE-CLUSTER-SOVD-ADDRESS", "OPT-IMPL-RECORD-ELEMENT", "PR-PORT-PROTOTYPE", "PDU-TO-FRAME-MAPPINGS", "CRYPTO-DRIVER", "COMPOSITION-DATA-PROTOTYPE-REF", "BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING", "STARTUP-CONFIGS", "SW-POINTER-TARGET-PROPS", "DIAGNOSTIC-ECU-RESET", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "ENABLE-JOIN-ACCESS-CONTROL", "SCHEDULER-NAME-PREFIXS", "DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF", "I-PDUS", "IIDC-SOURCE-PACKET-HEADER", "FUNCTION-ID", "FIELD-NOTIFIER-QOS-PROPSS", "PROCESS-ARGUMENTS", "OVERRIDE-INITIAL-STATE-IREF", "NUMBER-OF-RESTART-ATTEMPTS", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "E-2-E-PROFILE-CONFIGURATIONS", "IS-ACTIVATION-LINE-DEPENDENT", "DDS-METHOD-DEPLOYMENT", "MAX-CYCLE-REPETITIONS", "MAC-MULTICAST-CONFIGURATION", "DIAGNOSTIC-ENABLE-CONDITION-REF", "TRANSMISSION-MODE", "PROCESS-DESIGN-REF", "META-DATA-ITEM-SET", "AP-APPLICATION-ENDPOINT", "SYSC-REF", "TP-PGS", "CONTROLLER-REF", "TD-EVENT-FR-CLUSTER-CYCLE-START", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "DLT-USER-NEEDS", "DIAGNOSTIC-TRANSMIT-CERTIFICATE-INTERFACE", "ARTIFACT-CHECKSUMS", "CONTAINED-FIBEX-ELEMENT-REFS", "SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS", "PARAMETER-REQUIRE-COM-SPEC", "INHIBITION-SOURCE-REF", "FUNCTIONAL-UNIT", "IMPLEMENTED-ENTRYS", "IMPOSITION-TIME", "AAF-AES-3-DATA-TYPE", "COMPONENT-IREFS", "SUB-DOMAIN-REFS", "MESSAGE-LINE-NUMBER", "SEPARATED-COMPONENT-IREFS", "VALUE-SPEC", "NUMBER-OF-MINISLOTS", "ERROR-DOMAIN-REF", "NOTIFICATION-PORT-IREF", "TESTED-ITEM-REF", "DESTINATION-REF", "PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT", "GLOBAL-SUPERVISION-ENTITY", "CAN-BAUDRATE", "TIMEOUT-BS", "MESSAGE-LINK-LENGTH", "TLS-DEPLOYMENT", "TRANSIENT-FAULT", "NM-LIGHT-TIMEOUT", "NM-IMMEDIATE-NM-CYCLE-TIME", "VALIDATION-FORMULA", "INTERRUPT-SOURCE", "VEHICLE-SYSTEM-INSTANCE", "SHORT-NAME-PATTERN", "PERIOD", "TD-EVENT-TRIGGER", "TCP-OPTION-FILTER-SET", "FRAME-PORT", "EVENT-DEPLOYMENTS", "DDS-DOMAIN-RANGE", "CAN-TP-CHANNEL", "COMPILER", "DIAGNOSTIC-ROUTINE-NEEDS", "LOG-TRACE-DEFAULT-LOG-LEVEL", "INITIAL-REPETITIONS-BASE-DELAY", "VLAN-ID", "COUPLING-PORT-CREDIT-BASED-SHAPER", "PARAMETER-PROVIDE-COM-SPEC", "DDS-TOPIC-REF", "MANUFACTURER-CODE", "STATIC-SLOT-DURATION", "IEEE-1722-STREAM-REF", "STATE-MANAGEMENT-SLEEP-ACTION-ITEM", "STREAM-FILTERS", "I-PDU-TRIGGERINGS", "SW-DIAGNOSTICS-NOTES", "J-1939-CONTROLLER-APPLICATION-REF", "BUS-DEPENDENT-NM-ECUS", "BITFIELD-TEXT-TABLE-MASK-FIRST", "COM-MANAGEMENT-PORT-GROUP-IREFS", "FLOW-CONTROL-REF", "SW-COMPOSITIONS", "EXECUTE-DESPITE-DATA-UNAVAILABILITY", "ECUC-VALIDATION-CONDITION", "CONTEXTS", "FLEXRAY-TP-PDU-POOL", "LANGUAGE", "SERIALIZATION-TECHNOLOGY-REF", "FALSE-ACTION-LIST-REF", "RESPONSE-ON-SECOND-DECLINED-REQUEST", "ALLOWED-TCP-OPTION", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "PROVIDED-SERVICE-PORT-IREF", "FEATURE-REFS", "MODE-IREFS", "PDUR-TP-CHUNK-SIZE", "TARGET-COMPONENT-REF", "CRYPTO-SERVICE-NEEDS", "RPT-COMPONENTS", "DATA-ELEMENT-REFS", "EXTENDED-DATA-RECORDS", "TCP-DELAYED-ACK-TIMEOUT", "ACK-TYPE", "REQUESTOR-REF", "USE-INSTEAD-REF", "RUN-RESOURCE", "NM-INSTANCE-ID", "NOTIFICATIONS", "EVENT-HANDLERS", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "REPLY-TOPIC-NAME", "VALUE-RESTRICTION", "SW-VERSION", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "DELAY-COMPENSATION-A", "INTERNAL-BEHAVIOR", "NO-SUPERVISION", "SERVICE-INTERFACE-MAPPING-SET", "MAC-MULTICAST-ADDRESS", "USER-REF", "LOWER-CAN-ID", "MODE-GROUP-IREF", "APMC-FOREIGN-REFERENCE-DEF", "PDU-MAX-LENGTH", "DIAGNOSTIC-ENV-DATA-ELEMENT-CONDITION", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "STATE-MANAGEMENT-TRIGGER-COMPARE-RULE", "DHCP-PROPS", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "I-PDU-REF", "NUMBER", "MAXIMUM-TRANSMISSION-UNIT", "MODE-MANAGER-ERROR-BEHAVIOR", "AR-PARAMETER", "PERSISTENCY-CENTRAL-STORAGE-URI", "EVENT-READINESS-GROUPS", "APMC-NUMERICAL-PARAM-VALUE", "MAX-NUMBER-OF-PARALLEL-TRANSFERS", "SW-RECORD-LAYOUT-V-FIX-VALUE", "SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "DECODING-CORRECTION", "MIN-HEADER-ID", "REPLAY-PROTECTION", "MEMORY-LOW-ADDRESS", "SW-MIN-AXIS-POINTS", "SESSION-ID-SUPPORT", "ETHERNET-RAW-DATA-STREAM-MAPPING", "SIGNIFICANCE", "REST-STRING-PROPERTY-DEF", "COUPLING-TYPE", "HEADER-ID-SHORT-HEADER", "OPERATION-MODE", "SW-AXIS-TYPE", "COM-CONTROL-CLASS-REF", "GET", "CUSTOM-SUB-FUNCTION-NUMBER", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE", "FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING", "ACCESSED-DATA-PROTOTYPE-IREF", "DEADLINE", "GLOBAL-TIME-DOMAIN-PROPERTYS", "RECEIVER-INTENT", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "MAX-DIFF", "SCOPE-EVENT-REFS", "DIAGNOSTIC-INFO-TYPE", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "DDS-CP-CONFIG", "BUILD-ACTION", "DLT-ARGUMENT-ENTRYS", "SHARED-TIMER", "MAX-SOURCE-ELEMENT", "NM-CHANNEL-ACTIVE", "SOMEIP-SERVICE-DISCOVERY-PORT", "PDU-COLLECTION-TIMEOUT", "FIELD-ELEMENTS", "I-SIGNAL-PORT-REF", "IDSM-MODULE-INSTANTIATION-REF", "DTC-KIND", "COM-PROPS", "CALL-TYPE", "RAM-BLOCK-STATUS-CONTROL", "EVALUATED-VARIANT-SET", "SIGNAL-SERVICE-TRANSLATION-PROPSS", "APPLICATION-DATA-TYPE-REF", "USES-ADDRESS-ARBITRATION", "SENDER-REC-RECORD-ELEMENT-MAPPING", "TIME-PASSED-THRESHOLD", "PERSISTENCY-KEY-VALUE-PAIR", "APMC-CHOICE-CONTAINER-REFERENCE-DEF", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF", "ON-CHANGE-DATA-PROTOTYPES", "WAKEUP-FRAME-REF", "WRITE", "SECURED-AREA-LENGTH", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "URL", "IDSM-CONTEXT-PROVIDER-INTERFACE", "SW-AXIS-GENERIC", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "STATIC-REMOTE-UNICAST-ADDRESSES", "READ-NV-DATA", "PARAMETER-DATA-PROTOTYPE", "REQUESTS", "NOTIFIER", "DLT-LOG-SINK", "RESTRICT-UPDATE", "FIELD", "OFNS-DATA-ID-LIST", "TCP-PORT", "IKE-OVER-TIME", "INTERFACE-BLUEPRINT-REF", "NETWORK-ENDPOINTS", "TRANSMIT-PDU-REFS", "REQUEST-FILE-TRANSFER-CLASS-REF", "INTERFACE-VERSION", "RANGE-MIN", "IGNORE-ORDER-ALLOWED", "COM-EVENT-GRANT-DESIGN", "PROTOCOL-VERSION", "RTE-PLUGIN-PROPS", "JOB-PRIMITIVE-REF", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "VARIATION-RESTRICTION", "SECURE-COM-PROPS-FOR-DDS-REF", "SDG-CLASSES", "CONTROL-CONSUMED-EVENT-GROUP-REF", "COM-CONFIGURATION-GW-TIME-BASE", "FOLLOW-UP-ACTION-REFS", "CRYPTO-PROVIDER-REF", "RECEIVE-WINDOW-MIN", "DYNAMIC-PARTS", "MAX-ERROR-STATE-INVALID", "BSW-MODULE-ENTRY-REF", "SUPPORTS-MULTIPLE-NOTIFIER-SETS", "WORST-CASE-EXECUTION-TIME", "PROVIDED-MODE-GROUP-REF", "VENDOR", "COUNTER-JUMP-UP-VALUE", "STORE-ON-CHANGE", "CAN-ADDRESSING-MODE", "HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS", "SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF", "IS-OBD-RELEVANT", "DESC", "REDUNDANCY", "DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF", "DEBUG-INFOS", "EVENT-MULTICAST-ADDRESSS", "ANNOTATIONS", "AGING-REQUIRES-TESTED-CYCLE", "TTCAN-ABSOLUTELY-SCHEDULED-TIMING", "DATA-PDU-REF", "MAPPING-DIRECTION", "BEST-CASE-EXECUTION-TIME", "ASSOCIATED-EVENT-IDENTIFICATION", "DATA-OFFSET", "METHOD-REQUEST-TOPIC-NAME", "RPT-READ-ACCESS", "DIAGNOSTIC-SERVICE-TABLE-REF", "DDS-PARTITIONS", "USER-DEFINED-PHYSICAL-CHANNEL", "IEEE-1722-TP-CONFIG", "SOFTWARE-CLUSTER-REQUIREMENT", "USES-DYNAMIC-ADDRESSING", "IS-STRUCT-WITH-OPTIONAL-ELEMENT", "CONTROLLED-DATA-ELEMENT-REF", "EID-USE-MAC", "IEEE-1722-TP-ACF-CAN-PART", "ICV-SECURE-COM-PROPSS", "HEALTH-CHANNEL-EXTERNAL-STATUS", "FILTER-CHAINS", "BYTE", "DATA-IDENTIFIER-REF", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "SOCKET-ADDRESS-REF-CONDITIONAL", "DELETES", "TRACE-REF", "CATEGORY", "OFFSET-CORRECTION-ADAPTION-INTERVAL", "PROPAGATION-DELAY", "SCHEDULER-MAX-NUMBER", "FIRST-ELEMENT-REF", "SERVICE-TABLES", "PORT-NUMBER", "TROUBLE-CODE-UDS-REF", "ENTER-TIMEOUT-VALUE", "DATA-PROTECTION-KIND", "MIN-NUMBER-OF-TIME-QUANTA-PER-BIT", "BSW-MODULE-ENTRY-REF-CONDITIONAL", "RELEASED-TRIGGER-POLICYS", "ACL-CONTEXTS", "EVENT-PROPSS", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF", "UDP-COLLECTION-BUFFER-SIZE-THRESHOLD", "SAK-REF", "IP-SEC-CONFIG", "PROFILE-BEHAVIOR", "CONTEXT-ID", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING", "DIAGNOSTIC-J-1939-SPN", "OWNERSHIP", "IS-DYNAMIC-LENGTH-FIELD-SIZE", "DDS-SERVICE-VERSION", "PHYSICAL-DIMENSION-MAPPING-SET", "CURRENT-VALUE-REF", "SWC-BSW-SYNCHRONIZED-TRIGGER", "TLV-IMPL-RECORD-ELEMENT", "OP", "SECURITY-EVENT-FILTER-CHAIN", "DIAGNOSTIC-SOVD-FAULT-MEMORY-ACCESS", "ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL", "ANNOTATION-TEXT", "BINARY-MANIFEST-ITEM", "TRANSFER-REFS", "X", "IEEE-1722-RAW-DATA-STREAM-CONSUMER-MAPPING", "SENDER-RECEIVER-ANNOTATIONS", "EXECUTABLE-REF", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "GLOBAL-TIME-FR-SLAVE", "ECUC-VALIDATION-CONDS", "SERVICE-DEPENDENCYS", "REPORTED-SECURITY-EVENT-IREF", "FIFO-RANGES", "MACRO-INITIAL-OFFSET-A", "COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL", "ROLE-BASED-PORT-ASSIGNMENT", "TD-EVENT-BSW-MODE-DECLARATION", "STATIC-PART", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "BASE-CYCLE", "MODE-CONDITION", "ADDRESS-CONFIGURATION-CAPABILITY", "INFO-TYPE", "PERSISTENCY-PROVIDED-COM-SPEC", "MAX-ERROR-STATE-INIT", "ACCESS-DATA-RECORD-SIZE", "CONTAINED-PROCESS-REF", "TRIGGER-MAPPINGS", "SIGNATURE-SUPPORT-CP", "IPV-6-CONFIGURATION", "SW-COMPONENT-PROTOTYPE", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "EXCLUSIVE-AREA-REF", "ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL", "MAPPED-SWC-SERVICE-DEPENDENCY-IREF", "SW-CALIBRATION-NOTES", "FRAME-SYNC-ENABLED", "AUTOSAR-VARIABLE-IREF", "ISSUED-BY", "TCP-IP-NDP-MIN-RANDOM-FACTOR", "SW-COMPONENT-PROTOTYPE-REF", "REQUIRED-DDS-EVENT-QOS-PROPS", "FUNCTION-GROUP-REF", "VALUE-ACCESS", "SWC-TO-APPLICATION-PARTITION-MAPPINGS", "CORES", "PROCESS-DESIGN", "STARTUP-CONFIG-SET", "DIAGNOSTIC-MEMORY-REF", "ENGINEERING-OBJECTS", "HW-PIN-GROUPS", "MSR-QUERY-RESULT-CHAPTER", "FLEXRAY-FIFO-RANGE", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "SETTER-CALL-SIGNAL-REF", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "CHECK-PER-SOURCE-ID", "OPERATION-CYCLE-AUTOMATIC-END", "ESP-CIPHER-SUITE-NAME", "IN-SCOPE", "MINIMUM-DISTANCES", "MAPPED-DATA-ELEMENT-IREF", "DIAGNOSTIC-EVENT-PORT-MAPPING", "GROUP-ID", "SYNC-INTERVAL", "K-INIT-RESOURCE", "TRANSFORMER-STATUS-FORWARDING", "MAC-ADDRESS", "DYNAMIC-ARRAY-SIZE-PROFILE", "CRYPTO-KEY-REF", "ACCEPTANCE-FIELD", "NM-NETWORK-TIMEOUT", "ISSUED-TRIGGERS", "DATA-RECEIVE-POINT-BY-VALUES", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS", "CYCLE", "SERVICE-INTERFACE-PEDIGREE", "IMPLEMENTED-ENTRY-REF", "SENSOR-INSTANCE-ID", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "ACTION-LISTS", "DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE", "PREPARE-UPDATE", "TCP-IP-NDP-DEFAULT-RETRANS-TIMER", "FLEXRAY-AR-TP-CONFIG", "TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION", "HEADER-ID-LONG-HEADER", "FIREWALL-STATE-MACHINES", "PROFILE-NAME", "AUTO-AVAILABLE", "PERSISTENCY-KEY-VALUE-DATABASE", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "DIRECT-PDU-REF", "R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "SOURCE-ELEMENT-REF", "DYNAMIC-ACTION-REF", "INGRESS-PORT-REFS", "DIAGNOSTIC-SERVICE-INSTANCES", "WAKEUP-FRAME-REFS", "SW-CODE-SYNTAX", "DIAGNOSTIC-COM-CONTROL-CLASS", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "DATA-RECEIVE-POINTS", "DATA-ELEMENT-REF", "ERROR-MAPPINGS", "CERTIFICATE-TO-KEY-SLOT-MAPPINGS", "COUPLING-PORT-SHAPER", "MKA-PARTICIPANTS", "NETWORK-TIME-PROVIDER-REF", "DATA-RECEIVED-EVENT", "MAX-COMM-MODE", "ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF", "TCP-LOCAL-PORT", "FLEXRAY-NM-CLUSTER-COUPLING", "ALIVE-TIMEOUT", "GLOBAL-ELEMENT", "DIAGNOSTIC-SERVICE-TABLE", "RUNNABLE-ENTITY-ARGUMENT", "DECOMPOSITIONS", "BSW-IMPLEMENTATION", "ASSIGNED-DATA-TYPES", "UDP-PORT", "ICMP-V-4-PROPS", "COMPOSITE-SERVICE-INTERFACE-REF", "SERVICE-INTERFACE-DEPLOYMENT-REF", "DEF", "DIAGNOSTIC-TROUBLE-CODE-PROPS-REF", "BSW-MODE-MANAGER-ERROR-EVENT", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "DIAGNOSTIC-ROUTINE", "RVF-EVENT-DEFAULT", "CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING", "SW-DATA-DEPENDENCY-FORMULA", "ECUC-VALUE-CONFIGURATION-CLASS", "PARAMETER-PORT-ANNOTATION", "DDS-TOPICS", "DIAGNOSTIC-RELEVANCE", "CONFIG-ELEMENT-DEF-LOCAL-REF", "SOFTWARE-COMPOSITION-TREF", "SHAPER", "IDENTIFIER", "SOMEIP-TP-CONNECTION", "NM-VECTOR-EARLY-UPDATE", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "SOURCE-ELEMENT-REFS", "CDD-TYPE", "CALL-SIGNAL-TRIGGERING-REF", "ENABLE-TAKE-ADDRESS", "SENDER-ANNOTATION", "POST-BUILD-VARIANT-CONDITIONS", "TP-CONNECTION-CONTROL-REF", "CRYPTO-SERVICE-KEY", "EVENT-WINDOWS", "CAN-ID-RANGE-MAPPINGS", "NONQUEUED-RECEIVER-COM-SPEC", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "GLOBAL-KAY-PROPS-REF", "END-TO-END-CALL-RESPONSE-TIMEOUT", "DATA-TYPE-POLICY", "CONTEXT-ROOT-COMPOSITION-REF", "DIAGNOSTIC-FIM-EVENT-GROUP", "RELIABILITY-KIND", "CAN-TP-NODE", "ALGORITHM-FAMILY", "NM-PNC-PARTICIPATION", "NAMESPACES", "MAXIMUM-NUMBER-OF-RETRIES", "TIMING-EVENT-REF", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS", "AUXILIARY-FIELDS", "OUT-MEASURMENT-SET", "APMC-ENUMERATION-PARAM-DEF", "TD-EVENT-FRAME-ETHERNET", "SW-SYSTEMCONST-VALUE", "COLLECTED-INSTANCE-IREF", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "MESSAGE-LINK-POSITION", "ALL-CHANNELS-REFS", "SWITCH-FLOW-METERING-ENTRY", "DIAGNOSTIC-DATA-ELEMENT", "CKN-REF", "NM-NODE-DETECTION-ENABLED", "NUMBER-OF-CONNECTED-CLIENTS", "NOTIFIER-SIGNAL-TRIGGERING-REF", "SD-SERVER-EVENT-CONFIG", "DIAGNOSTIC-SERVICE-INSTANCE-REF-CONDITIONAL", "TIME-INTERVAL", "DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING", "SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS", "DESTINATION-MAX-ADDRESS", "SYSTEM-TRIGGERED-EVENTS", "PSK-IDENTITY-HINT", "DIAGNOSTIC-SOVD-UPDATE", "MINIMUM-EXECUTION-TIME", "RPT-PROFILE", "MODE-DRIVEN-FALSE-CONDITIONS", "LOGICAL-SUPERVISION", "HISTORY-ORDER-DEPTH", "CONTEXT-ATOMIC-COMPONENT-REF", "NM-IF-ECU-REF", "RULES", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING", "RECOVERY-VIA-APPLICATION-ACTION-REF", "HW-PIN-REF", "DO-IP-ROUTING-ACTIVATION", "HEADER-ID", "FLEXRAY-COMMUNICATION-CONNECTOR", "SERVER-COM-SPEC", "REVISION-LABEL", "PNC-IDENTIFIER", "SEC-OC-SECURE-COM-PROPS", "SEGMENT-BYTE-ORDER", "UNIT-REF", "ENUMS", "MODE-GROUP-REF", "DURABILITY", "KEY-SLOT-USAGE", "CALCULATION-FORMULA", "MODE-PORT-ANNOTATIONS", "SETTER-OPERATION-REF", "EXPLICIT-INTER-RUNNABLE-VARIABLES", "EXECUTION-ORDER-CONSTRAINT", "SUPPLIER-VALIDATION-ORDER-REFS", "SERVICE-INTERFACE-ELEMENT-MAPPING-REFS", "IP-V-6-ADDRESS", "EXPIRED-SUPERVISION-CYCLE-TOL", "TX-PDU-POOL-REF", "MAC-MULTICAST-GROUP", "MODE-SWITCH-POINT", "DATA-ELEMENT-IREF", "DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING", "REQUESTER-REF", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "FILTERING-DEBOUNCING", "NM-REPEAT-MSG-IND-ENABLED", "TEXT-TABLE-MAPPING", "VENDOR-SIGNATURE-REF", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "PHYSICAL-CHANNEL-REF-CONDITIONAL", "ALLOW-UNAUTHENTICATED-PARTICIPANTS", "DLT-MESSAGE-COLLECTION-SET", "CHECKPOINT-IREF", "VARIATION-POINT-PROXY", "AUTOSAR-ENGINEERING-OBJECT", "CAN-TP-CHANNEL-REF", "EXECUTION-TIMES", "WWH-OBD-DTC-CLASS", "E", "V-LAN-REFS", "CAN-TP-CONNECTION", "SENDING-NODE-REFS", "WRITTEN-READ-NV-DATA", "TCP-GENERAL-INACTIVITY-TIME", "SUBFUNCTIONS", "LATENCY-TIMING-CONSTRAINT", "FLOW-METERINGS", "DEPENDENT-SOFTWARE-CLUSTER-REF", "TLV-DATA-ID", "TRIGGERS", "CONTEXT-TYPE-LISTS", "SDXF", "CONNECTOR-REF", "BSW-TRIGGER-DIRECT-IMPLEMENTATION", "INCLUDED-MODE-DECLARATION-GROUP-SET", "PER-STATE-TIMEOUT", "OWNERSHIP-STRENGTH", "ECUC-ENUMERATION-PARAM-DEF", "DEFAULT-TRAFFIC-CLASS", "TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION", "TIME-SYNC-SERVER-REF", "TRAFFIC-LIMITATION-FILTERS", "MC-FUNCTION-REF", "RTE-EVENT-IREFS", "NV-BLOCK-DATA-MAPPING", "INIT-VALUES", "SERVICE-ID", "ARGUMENTSS", "INSTANCE-IDS", "OBD-DTC-NUMBER", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF", "LOG-TRACE-DEFAULT-LOG-THRESHOLD", "AR-TYPED-PER-INSTANCE-MEMORYS", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "INTERPOLATION-ROUTINE-REF", "V-2-X-M-USER-NEEDS", "MC-GROUP-DATA-REF-SET-CONDITIONAL", "UCMS", "I-SIGNAL-I-PDU-REF-CONDITIONAL", "PATTERN", "PRE-MAPPING", "HW-ATTRIBUTE-DEF", "PHM-ACTION-LIST", "ACTIVATION-REASONS", "PORTS", "FRAGMENTATION-PROPS", "FLEXRAY-CLUSTER", "SOCKET-CONNECTION-IPDU-IDENTIFIER", "RECOVERY-NOTIFICATION", "DATA-WRITE-COMPLETED-EVENT", "BSW-MODULE-DEPENDENCY", "PDU-ACTIVATION-ROUTING-GROUPS", "APPROVAL-REQUIRED", "TRAFFIC-SHAPER-GROUP-REF", "TARGET-ELEMENT-REF", "GLOBAL-TIME-DOMAIN-REF", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "AR-TYPED-PER-INSTANCE-MEMORY-REF", "MAPPING-REF", "PROVIDER-RATE-DEVIATION-MAX", "IS-PIPELINING-PERMITTED", "ECU-TASK-PROXY-REFS", "REFERENCE-TABLE-REF", "ACF-TRANSPORTED-BUSS", "TCP-RECEIVE-WINDOW-MAX", "ROOT-COMPOSITION", "LOGGING-BEHAVIOR", "MAX-DESTINATION-PORT-NUMBER", "SW-RECORD-LAYOUT-GROUP-STEP", "OFFSET-TIMING-CONSTRAINT", "INTERFACE-BLUEPRINT-MAPPING", "SW-GENERIC-AXIS-PARAMS", "PORT-PROTOTYPE-PROPS", "FURTHER-ACTION-BYTE-NEEDS", "DOIP-CONNECTION-REF", "ECUC-ADD-INFO-PARAM-DEF", "BLUEPRINT-CONDITION", "SW-COMPONENT-MAPPING-CONSTRAINTS", "LET-INTERVAL-REFS", "DIAGNOSTIC-CONNECTION-REF", "READ-SCALING-DATA-CLASS-REF", "PLCA-LOCAL-NODE-COUNT", "OFFSETS", "TX-NM-PDU-REF", "REMOTE-IP-ADDRESS-REFS", "FIELD-TOPICS-ACCESS-RULE-REF", "AUTOSAR-VARIABLE-INSTANCE", "BULK-NV-DATA-DESCRIPTORS", "IS-GLOBAL", "RECOVERY-NOTIFICATION-RETRY", "DEFAULT-ROUTER", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "IIDC-DATA-BLOCK-SIZE", "TRANSFORMER-CLASS", "MIN-PWM-L", "IUMPR-REFS", "ANNOTATION", "NM-SCHEDULE-VARIANT", "FIRST-ELEMENTS", "CONSTRAINT-TAILORINGS", "PERSISTENCY-DATA-REQUIRED-COM-SPEC", "SAFE-TRANSLATION", "STATUS-REF", "IEEE-1722-TP-CRF-CONNECTION", "SECOND-PHYSICAL-DIMENSION-REF", "HEALTH-CHANNEL-EXTERNAL-MODE", "DEBOUNCE-TIME", "MAX", "USER-DEFINED-METHOD-DEPLOYMENT", "HAS-NOTIFIER", "NM-SYNCHRONIZING-NETWORK", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF", "RECOVERY-NOTIFICATION-TIMEOUT", "REQUIRED-AR-ELEMENT-REF", "RELATIONS", "FLEXRAY-TP-CONNECTION-CONTROL", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "TIME-FR-IF", "DIAGNOSTIC-COMMON-PROPS-CONDITIONAL", "FUNCTION", "SHORT-NAME", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE", "POSSIBLE-ERROR-REFS", "TRANSLATION-TARGET-IREF", "ALIVE-CHECK-RESPONSE-TIMEOUT", "MACHINE-DESIGN-REF", "ETHERNET-FRAME", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS", "ECUC-URI-REFERENCE-DEF", "GLOBAL-TIME-DOMAIN-PROPS", "I-PDU-TIMING", "PER-INSTANCE-MEMORY-SIZES", "SERVER-NAME-IDENTIFICATION", "USE-AUTO-VALIDATION-AT-SHUT-DOWN", "BSW-ENTRY-KIND", "PORT-DEFINED-ARGUMENT-VALUE", "TD-EVENT-SLLET-PORT", "GLOBAL-TIME-PROPS", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "ECU-INSTANCE-PROPSS", "CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST", "BSW-OPERATION-INVOKED-EVENT", "TIME-CS", "PARAMETER-POLICYS", "J-1939-NM-NODE", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "EXCLUSIVE-AREA-NESTING-ORDER-REF", "ADAPTIVE-FIREWALL-MODULE-INSTANTIATION", "UPDATE-INDICATION-BIT-POSITION", "SLOT-TYPE", "SUPPORTING-MATERIAL", "ELEMENT-CREATION", "VERBATIM", "SOVD-SERVER-INSTANTIATION", "BYPASS-VLANS", "SW-VARIABLE-REFS", "FIRST-MODE-REF", "UDS-DTC-NUMBER", "REQUEST-DATA-SIZE", "INTERVAL-LENGTH", "APPLICATION-ID", "PARAMETER-SIZE", "BSW-MODULE-DESCRIPTION-REF", "CAN-XL-PROPS-REFS", "COMPU-PHYS-TO-INTERNAL", "PHYSICAL-DIMENSION-MAPPING", "TCP-PROPS", "IPV-6-DHCP-SERVER-CONFIGURATION", "MAXIMUM-TRANSMIT-BUFFER-LENGTH", "ECU-RESET-CLASS-REF", "SCOPE-EVENT-REF", "SWC-EXCLUSIVE-AREA-POLICY", "CS-ERROR-REACTION", "POSITION-IN-TABLE", "WITH-AUTO", "DURABILITY-SERVICE", "EVENT-MAPPING", "OCCURRENCE-COUNTER-PROCESSING", "RUNNABLE-ENTITY-GROUP-IREF", "MAX-TRCV-DELAY-COMPENSATION-OFFSET", "USER-DEFINED-COMMUNICATION-CONNECTOR", "EXCESS-INFORMATION-RATE", "TWO-KEY-SLOT-MODE", "FAILED-REFERENCE-CYCLES-TOLERANCE", "MIN-LENGTH", "SW-RECORD-LAYOUT", "HEALTH-CHANNEL-ARGUMENT-REFS", "PORT-BLUEPRINT-REF", "FIELD-REF", "DEPENDS-ON-REF", "MAPPED-OPERATION-IREF", "DEFINITION-REF", "SOFTWARE-CLUSTER-UDS-DIAGNOSTIC-ADDRESS", "TCP-IP-DHCP-V-6-SOL-DELAY-MAX", "SW-GENERIC-AXIS-PARAM-TYPES", "AFFECTED-BY-CONTROL-DTC-SETTING", "TTCAN-COMMUNICATION-CONTROLLER-VARIANTS", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "OPTIONAL", "REST-OBJECT-REF", "ENCODING", "INCLUDED-LIBRARY-REFS", "SOURCE-INSTANCE-IREF", "DLT-ECU-REF", "MAP-ELEMENT-TUPLES", "RAW-DATA-STREAM-INTERFACE-REF", "SIGNAL-BASED-EVENT-DEPLOYMENT", "VERIFY-REF", "NM-BUSLOAD-REDUCTION-ACTIVE", "CAN-FRAME-TX-BEHAVIOR", "VERIFY-IREF", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "REPLACE-WITH", "VG", "CLIENT-SERVER-PORTS", "CAN-XL-CONFIG-REF", "SW-MAINTENANCE-NOTESS", "DATA-COM-PROPS", "REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF", "COUNT-PROFILE", "REFERENCE-VALUES", "SW-VARIABLE-PROTOTYPE-REF", "FUNCTIONAL-REQUEST-REF", "MEMORY-RANGE-REF", "RESOURCE-REF", "COMMUNICATION-CONNECTORS", "HOST-SYSTEM-REF", "ASSIGNED-CONTROLLER-REF", "PNC-WAKEUP-ENABLE", "COMPILERS", "SUPERVISION-MODE-CONDITION", "L-1", "MODULE-ID", "PERIODIC-RATES", "SOURCE-ADDRESS-RANGE-START", "DISPLAY-FORMAT", "ADDITIONAL-INFORMATION", "KEY-SLOT-ID", "SWC-MAPPING-CONSTRAINT-REF", "INDICATIONS", "ECUC-VALUES", "MC-SW-EMULATION-METHOD-SUPPORT", "SUCCESSOR-REFS", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "UPDATE", "SECURE-COM-PROPS-FOR-UDP-REF", "DETERMINISTIC-SYNC-MASTER-REF", "ALGORITHM-MODE", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "TCP-IP-IP-REASS-TIMEOUT", "P-2-SERVER-MAX", "DIAGNOSTIC-PROPS", "NM-REMOTE-SLEEP-INDICATION-TIME", "DLT-LOG-CHANNEL", "DOC-REVISIONS", "SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING", "NESTED-ELEMENT-REF", "MATCHING-CRITERION-REF", "TIME-FAILED-THRESHOLD", "NETWORK-MANAGEMENT-VECTOR-LENGTH", "NETWORK-MANAGEMENT-PORT-INTERFACE", "TEMPLATE-TYPE-REF", "CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF", "COLLECTION-TRIGGER", "L-5", "RESET-TO-DEFAULT", "RETRY-INTERVAL-TIME", "DIFFERENTIATED-SERVICE-CODE-POINT", "NM-NODE-ID-ENABLED", "SYNC-NODE-MAX", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "PROCESS", "INTENDED-TARGET-MACHINE-REFS", "ELEMENT-GROUPS", "LIN-CONFIGURABLE-FRAME", "MAX-NO-NEW-OR-REPEATED-DATA", "SAFETY-CONDITION", "CONSUMED-DDS-OPERATIONS", "HAS-INTERNAL-STATE", "ETHERNET-COMMUNICATION-CONNECTOR", "DEADLINE-SUPERVISION", "PRIVACY-LEVEL", "LOWER-LIMIT", "FILE-STORAGE-REF", "PAYLOAD-LENGTH", "TLS-SECURE-COM-PROPS-REF", "IEEE-1722-ACF-BUS-PART-RAW-DATA-STREAM-CONSUMER-MAPPING", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "SAK-REKEY-TIME-SPAN", "STARTS-ON-EVENT-REF", "CLIENT-SERVER-APPLICATION-ERROR-MAPPING", "RESTRICTION", "READ", "MULTICAST-CONNECTOR-REFS", "SCOPE-REF", "IDSM-RATE-LIMITATION-REF-CONDITIONAL", "LATENCY-BUDGET-DURATION", "CRYPTO-PRIMITIVE", "TRANSITIONS", "DERIVED-INTERFACE-REF", "EXTERNAL-TRIGGERING-POINTS", "CRYPTO-CERTIFICATE-REF", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF", "TD-HEADER-ID-RANGE", "EXPRESSION-REF", "CALIBRATION-PARAMETER-VALUE-SET", "CRYPTO-KEY-SLOT-TO-CLIENT-PORT-PROTOTYPE-MAPPING", "CONFIDENCE-INTERVALS", "CAN-XL-BAUDRATE", "TIME-PERIOD", "PDU-TRIGGERING-REF-CONDITIONAL", "TIME-SYNC-R-PORT-PROTOTYPE-IREF", "COMPONENT-IMPLEMENTATION-REF", "RESPONSE-DATA-SIZE", "CRC-PRECISE-ORIGIN-TIMESTAMP", "INTERFACE-MAPPING", "DATA-PROVIDER", "IP-IAM-AUTHENTIC-CONNECTION-PROPS", "I-SIGNAL-TRIGGERING-REF-CONDITIONAL", "MINIMUM-MEMORY-CONSUMPTION", "CONTEXT-DATA-PROTOTYPE-REFS", "TIMEOUT-CR", "FLAT-MAP", "ACTIVE-SUPERVISION-REFS", "AUTHENTICATION-BUILD-ATTEMPTS", "STATE-MANAGEMENT-BASED-ON-TCP-FLAGS", "SECTION-TYPE", "FLEXRAY-TP-CONNECTION", "PERMISSION-TO-CREATE-CHILD-PROCESS", "SPECIFICATION-DOCUMENT-SCOPE", "DDS-DOMAINS", "TLS-CRYPTO-CIPHER-SUITE", "TD-EVENT-OPERATION-TYPE", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "HAS-SETTER", "REQUIRE-RESOURCES", "INVALIDATION-POLICY", "SYSC-STRING-REF"];

    /// derive an enum entry from an input string using a perfect hash function
    ///
    /// # Errors
    ///
    /// [`ParseElementNameError`]: The input string did not match the name of any enum item
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseElementNameError> {
        static DISPLACEMENTS: [(u16, u16); 923] = [(0, 0), (0, 2571), (0, 11), (0, 7), (0, 2778), (0, 2042), (0, 72), (0, 1834), (0, 1282), (0, 456), (0, 381), (0, 50), (1, 1545), (0, 253), (0, 285), (9, 2601), (0, 57), (0, 1), (0, 37), (0, 921), (0, 9), (0, 1495), (0, 33), (3, 1645), (0, 77), (0, 3), (0, 3080), (0, 10), (3, 1017), (0, 206), (5, 1810), (0, 579), (3, 3841), (0, 81), (1, 1307), (0, 615), (0, 380), (0, 4), (0, 850), (0, 28), (0, 8), (0, 22), (0, 28), (0, 12), (0, 24), (0, 25), (10, 5983), (0, 26), (0, 263), (0, 695), (0, 139), (0, 1), (0, 461), (0, 42), (8, 3602), (0, 1012), (0, 46), (9, 3712), (0, 2), (5, 3366), (0, 24), (1, 1669), (0, 97), (0, 0), (0, 4), (4, 6394), (0, 2325), (0, 4), (0, 3), (0, 619), (0, 7), (0, 263), (0, 3899), (0, 1), (0, 517), (0, 46), (0, 21), (0, 78), (0, 22), (0, 6), (1, 207), (0, 92), (3, 3641), (0, 441), (17, 4266), (0, 942), (0, 976), (0, 9), (0, 2), (0, 7), (0, 12), (0, 188), (0, 809), (3, 2963), (4, 2057), (0, 4423), (5, 182), (0, 92), (0, 22), (6, 2579), (5, 2822), (16, 3980), (0, 21), (0, 8), (0, 257), (0, 1597), (0, 574), (0, 2), (0, 1), (0, 2070), (0, 0), (0, 85), (1, 1781), (0, 2), (0, 317), (0, 14), (0, 3), (0, 32), (40, 1403), (4, 5539), (3, 1417), (11, 6067), (0, 56), (0, 2299), (0, 41), (1, 2460), (1, 4729), (0, 3051), (1, 22), (0, 0), (0, 1604), (0, 1), (0, 1859), (0, 63), (2, 6125), (0, 0), (0, 0), (0, 1228), (0, 4), (0, 27), (0, 79), (0, 0), (0, 4), (0, 304), (0, 517), (92, 491), (0, 5907), (0, 378), (0, 3), (2, 3161), (0, 394), (0, 18), (3, 4565), (0, 71), (0, 8), (1, 2480), (0, 433), (0, 67), (0, 3558), (0, 3445), (0, 4186), (0, 378), (0, 1568), (0, 6369), (0, 4), (0, 309), (0, 53), (0, 112), (3, 2205), (0, 6), (0, 19), (0, 18), (0, 1064), (0, 6383), (0, 489), (1, 1), (0, 0), (0, 1597), (0, 2), (0, 64), (0, 0), (0, 47), (1, 1130), (0, 1), (0, 2), (0, 0), (0, 6), (0, 74), (0, 1), (0, 5417), (0, 108), (0, 0), (0, 588), (0, 1421), (0, 8), (0, 3), (0, 295), (5, 4427), (0, 4), (0, 80), (0, 831), (0, 0), (23, 662), (0, 21), (0, 38), (0, 2105), (0, 171), (3, 4125), (0, 5366), (0, 582), (10, 851), (0, 3640), (0, 1983), (7, 3831), (31, 194), (0, 686), (0, 101), (6, 3306), (0, 12), (0, 22), (2, 1177), (0, 23), (0, 168), (0, 513), (0, 6), (0, 44), (0, 0), (60, 3129), (0, 406), (0, 4174), (13, 2549), (0, 103), (15, 151), (0, 2), (0, 3296), (0, 0), (28, 1837), (0, 227), (0, 209), (0, 0), (0, 55), (0, 4996), (0, 4704), (10, 364), (0, 2361), (0, 140), (0, 734), (0, 3882), (1, 3231), (0, 803), (33, 1580), (0, 0), (0, 0), (0, 7), (5, 2366), (10, 186), (1, 1397), (0, 120), (0, 248), (0, 9), (0, 2463), (0, 2), (0, 2074), (4, 540), (0, 1021), (1, 1204), (0, 46), (0, 534), (0, 3639), (0, 2), (31, 3425), (1, 6419), (39, 5413), (0, 168), (0, 5280), (0, 227), (0, 24), (0, 2), (0, 0), (0, 3), (0, 2353), (0, 5712), (0, 689), (0, 502), (0, 0), (0, 303), (3, 2680), (3, 4410), (0, 740), (11, 5104), (0, 23), (1, 5029), (0, 83), (0, 109), (0, 10), (1, 2292), (0, 18), (0, 793), (0, 4692), (0, 314), (0, 84), (20, 1600), (0, 95), (0, 227), (0, 73), (0, 666), (1, 3126), (0, 32), (0, 29), (0, 1342), (0, 1206), (0, 85), (0, 866), (12, 629), (0, 277), (4, 2316), (2, 667), (0, 1136), (0, 1891), (1, 2), (0, 5), (0, 0), (0, 35), (1, 2778), (0, 699), (0, 378), (0, 2), (0, 52), (14, 5970), (0, 1409), (1, 151), (0, 46), (0, 3167), (0, 27), (4, 3316), (1, 838), (0, 114), (18, 4309), (0, 28), (0, 307), (1, 4884), (2, 2833), (1, 6268), (0, 10), (0, 728), (0, 38), (0, 3514), (1, 1304), (0, 5164), (1, 4478), (0, 62), (0, 21), (2, 1677), (24, 3891), (0, 2), (0, 90), (0, 1306), (1, 6324), (0, 155), (9, 1869), (1, 2510), (0, 33), (0, 62), (0, 258), (0, 208), (0, 13), (72, 3462), (0, 5770), (0, 1), (2, 300), (1, 6094), (0, 354), (0, 0), (0, 95), (0, 3899), (4, 4323), (0, 480), (0, 210), (0, 1666), (0, 0), (0, 13), (0, 2), (0, 9), (0, 84), (0, 0), (0, 36), (0, 5044), (0, 428), (0, 5697), (0, 53), (0, 156), (0, 4181), (0, 565), (0, 99), (5, 1296), (0, 0), (0, 1), (0, 6007), (0, 8), (8, 5639), (0, 2), (0, 3), (0, 5009), (19, 4665), (0, 22), (0, 1572), (0, 4), (3, 3918), (0, 3665), (4, 2392), (0, 4), (0, 1), (0, 0), (21, 2690), (0, 532), (0, 782), (0, 506), (0, 854), (0, 1111), (2, 4150), (0, 589), (0, 1813), (0, 72), (0, 1636), (0, 297), (2, 4178), (0, 99), (0, 2), (8, 270), (0, 459), (0, 3992), (3, 5898), (2, 5470), (0, 63), (7, 593), (0, 58), (0, 3), (0, 0), (0, 22), (24, 40), (0, 13), (5, 5192), (0, 11), (1, 1084), (0, 4105), (0, 4), (1, 4884), (0, 6), (0, 130), (0, 34), (25, 5742), (0, 451), (1, 3621), (37, 1141), (62, 5963), (0, 1), (0, 4), (0, 4917), (46, 2675), (0, 0), (0, 6), (14, 1309), (0, 83), (0, 8), (0, 451), (0, 1), (4, 497), (0, 51), (70, 196), (2, 5413), (0, 0), (0, 107), (0, 261), (0, 44), (0, 551), (0, 40), (0, 1287), (0, 4), (22, 1641), (0, 1), (37, 759), (8, 4463), (0, 2), (0, 110), (0, 84), (0, 0), (0, 1065), (0, 4440), (6, 1818), (1, 572), (0, 2553), (0, 21), (38, 2526), (0, 8), (5, 3705), (3, 2150), (5, 4667), (0, 582), (0, 15), (0, 17), (0, 12), (0, 24), (0, 104), (0, 3701), (0, 125), (0, 35), (0, 5051), (2, 5241), (49, 633), (20, 1821), (0, 1), (0, 87), (1, 2918), (0, 1720), (14, 1166), (79, 2446), (74, 5683), (0, 105), (0, 5172), (0, 2), (6, 1767), (17, 3201), (0, 1359), (4, 3253), (0, 330), (0, 13), (0, 0), (0, 0), (0, 111), (3, 3615), (28, 4352), (4, 72), (2, 1673), (0, 133), (5, 3662), (0, 70), (0, 349), (0, 124), (0, 8), (0, 122), (2, 4634), (19, 3274), (0, 2), (69, 5012), (0, 515), (0, 1348), (0, 439), (0, 0), (5, 5176), (0, 52), (45, 2531), (0, 460), (0, 3374), (126, 1712), (0, 380), (5, 2544), (0, 452), (0, 6167), (34, 5159), (0, 19), (0, 4007), (0, 2228), (0, 0), (0, 5595), (0, 9), (24, 4676), (0, 4400), (0, 57), (0, 2088), (0, 32), (0, 213), (10, 500), (0, 8), (12, 1346), (22, 151), (0, 124), (2, 1454), (0, 1315), (0, 80), (0, 32), (1, 5620), (0, 1119), (0, 10), (0, 1284), (4, 5398), (1, 78), (0, 426), (0, 628), (1, 5680), (0, 4), (7, 1607), (0, 5956), (0, 3728), (9, 239), (28, 1470), (0, 1916), (0, 270), (0, 318), (0, 312), (0, 13), (0, 540), (31, 4141), (0, 12), (0, 1), (0, 498), (3, 774), (19, 2878), (0, 14), (27, 4090), (0, 2507), (20, 879), (0, 2240), (0, 5203), (0, 0), (0, 618), (0, 0), (0, 4622), (0, 2), (0, 127), (106, 5495), (0, 234), (0, 11), (0, 3), (0, 9), (0, 4814), (0, 0), (0, 0), (20, 3286), (0, 5), (2, 1822), (0, 4265), (14, 826), (0, 3832), (0, 0), (0, 49), (0, 15), (50, 6109), (0, 22), (0, 1818), (0, 7), (0, 90), (1, 4715), (0, 117), (2, 4695), (72, 5871), (191, 5076), (95, 1072), (0, 435), (0, 1), (1, 5493), (0, 20), (339, 1902), (0, 8), (0, 353), (16, 51), (72, 3539), (0, 3), (0, 479), (0, 0), (0, 3896), (0, 43), (0, 61), (0, 2), (0, 747), (0, 4369), (0, 1987), (0, 0), (0, 3), (293, 5513), (0, 140), (31, 5634), (0, 1), (9, 4091), (123, 2835), (0, 1187), (0, 2), (0, 17), (71, 5948), (0, 6269), (1, 3563), (0, 597), (21, 4278), (63, 5811), (0, 40), (105, 906), (0, 6), (0, 47), (0, 5454), (54, 197), (0, 58), (0, 140), (0, 43), (0, 42), (0, 2), (19, 4306), (0, 18), (43, 802), (0, 118), (0, 1407), (144, 4604), (0, 1176), (0, 2627), (0, 752), (186, 5322), (25, 1665), (2, 3909), (0, 52), (0, 1), (0, 367), (0, 5947), (0, 50), (67, 5799), (1, 3490), (0, 1124), (268, 1725), (2, 3163), (0, 20), (2, 1247), (0, 23), (1, 2685), (0, 2), (2, 4147), (0, 2585), (43, 176), (0, 3), (0, 5179), (0, 1843), (0, 635), (221, 5262), (3, 1218), (0, 21), (9, 4867), (0, 85), (0, 0), (10, 2633), (7, 2910), (0, 113), (1, 2027), (0, 0), (0, 272), (0, 1384), (0, 945), (0, 45), (0, 3679), (57, 3979), (0, 63), (0, 13), (11, 280), (0, 4), (1, 5712), (0, 1143), (0, 7), (0, 2872), (0, 541), (0, 237), (90, 3591), (0, 2242), (0, 88), (0, 26), (1, 2348), (0, 1703), (0, 2173), (2, 2662), (0, 908), (0, 524), (0, 714), (1, 2369), (0, 82), (0, 19), (0, 12), (5, 5951), (9, 3734), (0, 2023), (2, 6136), (48, 932), (0, 1), (0, 706), (0, 749), (3, 2315), (6, 3340), (2, 1603), (0, 2362), (4, 1386), (0, 83), (0, 0), (0, 86), (0, 3687), (12, 5553), (6, 5548), (22, 5066), (0, 5), (0, 2007), (139, 3743), (3, 801), (2, 6153), (7, 1343), (245, 700), (28, 4818), (0, 24), (0, 4739), (30, 5582), (0, 241), (0, 46), (0, 14), (0, 115), (8, 2695), (91, 5005), (1, 553), (0, 28), (0, 106), (117, 3958), (0, 161), (3, 5380), (0, 1333), (44, 3128), (52, 150), (256, 231), (0, 1), (0, 405), (2, 3181), (186, 2783), (0, 35), (2, 2719), (159, 3571), (0, 67), (19, 1455), (0, 1574), (1, 3662), (18, 3181), (0, 328), (4, 3075), (2, 46), (0, 658), (0, 23), (108, 756), (0, 631), (0, 7), (28, 6006), (0, 2258), (384, 194), (0, 1), (0, 4), (0, 1047), (182, 482), (0, 0), (1, 2170), (47, 58), (1261, 5658), (0, 60), (46, 1817), (14, 795), (4, 3684), (12, 5932), (2, 2484), (0, 1075), (1, 2217), (0, 1105), (0, 229), (0, 109), (0, 108), (0, 2113), (0, 0), (0, 11), (0, 0), (0, 45), (7, 885), (0, 0), (0, 49), (328, 5661), (3, 620), (385, 3074), (0, 3), (20, 4014), (0, 57), (0, 1), (0, 6), (33, 2474), (0, 729), (4, 4198), (0, 254), (0, 51), (0, 4), (304, 1975), (2, 3162), (0, 109), (73, 5856), (42, 339), (0, 5343), (0, 0), (1, 3822), (2, 463), (110, 3756), (0, 819), (0, 48), (0, 563), (80, 626), (0, 39), (0, 5484), (2, 2140), (0, 451), (10, 151), (0, 29), (0, 7), (0, 3672), (0, 282), (0, 20), (0, 30), (0, 6), (0, 248), (149, 1354), (0, 6291), (0, 73), (0, 631), (5, 1801), (0, 36), (0, 565), (0, 1967), (7, 3783), (0, 8), (0, 2482), (4, 297), (7, 3976), (0, 3707)];
        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 923) as usize];
        let item_idx = u32::from(d2).wrapping_add(f1.wrapping_mul(u32::from(d1))).wrapping_add(f2) as usize % 6459;
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

